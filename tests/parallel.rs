#![cfg(feature = "parallel")]
use meshoptimizer_rs::codec::*;
use meshoptimizer_rs::parallel::*;
use meshoptimizer_rs::*;

fn pools() -> Vec<rayon::ThreadPool> {
    let n = std::thread::available_parallelism().unwrap().get();
    [1, 2, 8, n]
        .into_iter()
        .map(|threads| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
        })
        .collect()
}
fn mesh(n: usize, seed: usize) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..n * n)
        .map(|i| {
            let x = (i % n) as f32;
            let y = (i / n) as f32;
            [x, y, ((i * 37 + seed * 13) % 19) as f32 / 32.]
        })
        .collect();
    let mut indices = Vec::new();
    for y in 0..n - 1 {
        for x in 0..n - 1 {
            let a = (y * n + x) as u32;
            indices.extend([
                a,
                a + 1,
                a + n as u32,
                a + n as u32,
                a + 1,
                a + n as u32 + 1,
            ]);
        }
    }
    (p, indices)
}
fn words(values: impl IntoIterator<Item = u32>, bytes: &mut Vec<u8>) {
    for value in values {
        bytes.extend(value.to_le_bytes());
    }
}
fn lod_bytes(levels: &[SimplifiedMesh]) -> Vec<u8> {
    let mut bytes = Vec::new();
    words([levels.len() as u32], &mut bytes);
    for level in levels {
        words(
            [level.error.to_bits(), level.indices.len() as u32],
            &mut bytes,
        );
        words(level.indices.iter().copied(), &mut bytes);
    }
    bytes
}
fn meshlet_bytes(value: &Meshlets) -> Vec<u8> {
    let mut bytes = Vec::new();
    words(
        [
            value.meshlets.len() as u32,
            value.vertices.len() as u32,
            value.triangles.len() as u32,
        ],
        &mut bytes,
    );
    for m in &value.meshlets {
        words(
            [
                m.vertex_offset,
                m.triangle_offset,
                m.vertex_count,
                m.triangle_count,
            ],
            &mut bytes,
        );
    }
    words(value.vertices.iter().copied(), &mut bytes);
    bytes.extend(&value.triangles);
    bytes
}

#[test]
fn lod_chains_match_direct_sequential_calls_at_1_2_8_and_n_threads() {
    let meshes: Vec<_> = (0..40).map(|i| mesh(2 + i % 8, i)).collect();
    let attrs: Vec<Vec<f32>> = meshes
        .iter()
        .map(|(p, _)| p.iter().flat_map(|v| [v[0] / 8., v[1] / 8.]).collect())
        .collect();
    let flags: Vec<Vec<_>> = meshes
        .iter()
        .map(|(p, _)| {
            (0..p.len())
                .map(|i| {
                    if i % 11 == 0 {
                        VertexFlags::LOCK
                    } else {
                        VertexFlags::EMPTY
                    }
                })
                .collect()
        })
        .collect();
    let settings: Vec<Vec<_>> = meshes
        .iter()
        .map(|(_, indices)| {
            [1., 0.5, 0.25]
                .map(|ratio| SimplifySettings {
                    target_index_count: (indices.len() as f32 * ratio) as usize,
                    target_error: 1.,
                    options: SimplifyOptions::PERMISSIVE,
                })
                .to_vec()
        })
        .collect();
    let mut inputs: Vec<_> = meshes
        .iter()
        .enumerate()
        .map(|(i, (p, indices))| LodChainInput {
            indices,
            positions: Positions::from_packed(p),
            levels: &settings[i],
            attributes: (i.is_multiple_of(2)).then(|| LodAttributes {
                attributes: Attributes::from_interleaved(&attrs[i], p.len(), 2, 2, 0).unwrap(),
                weights: &[1., 2.],
                vertex_flags: Some(&flags[i]),
            }),
        })
        .collect();
    let invalid = [0, 1];
    inputs.insert(
        3,
        LodChainInput {
            indices: &invalid,
            ..inputs[0]
        },
    );
    inputs.push(LodChainInput {
        levels: &[],
        ..inputs[0]
    });
    let expected: Vec<_> = inputs
        .iter()
        .map(|input| {
            let mut previous = input.indices;
            let mut levels = Vec::new();
            for &settings in input.levels {
                let mut ws = Workspace::default();
                let result = match input.attributes {
                    None => simplify(previous, input.positions, settings, &mut ws),
                    Some(a) => simplify_with_attributes(
                        previous,
                        input.positions,
                        a.attributes,
                        a.weights,
                        a.vertex_flags,
                        settings,
                        &mut ws,
                    ),
                };
                match result {
                    Ok(level) => levels.push(level),
                    Err(e) => return Err(e),
                }
                previous = &levels.last().unwrap().indices;
            }
            Ok(lod_bytes(&levels))
        })
        .collect();
    assert_eq!(expected[3], Err(Error::InvalidTopology));
    for pool in pools() {
        for _ in 0..3 {
            let result = pool
                .install(|| simplify_lod_chains_batch(&inputs, Limits::default()))
                .unwrap();
            let actual: Vec<_> = result
                .into_iter()
                .map(|r| r.map(|v| lod_bytes(&v)))
                .collect();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn chain_budget_includes_all_levels_and_cumulative_work() {
    let (p, indices) = mesh(9, 12);
    let levels = [SimplifySettings {
        target_index_count: indices.len(),
        target_error: 0.01,
        options: SimplifyOptions::EMPTY,
    }; 3];
    let input = LodChainInput {
        indices: &indices,
        positions: Positions::from_packed(&p),
        levels: &levels,
        attributes: None,
    };
    let mut ws = Workspace::default();
    let expected = simplify_lod_chain(&input, &mut ws).unwrap();
    let exact = Limits {
        max_bytes: ws.usage().bytes,
        max_work: ws.usage().work,
    };
    assert!(exact.max_bytes >= indices.len() * 4 * 3 + std::mem::size_of::<SimplifiedMesh>() * 3);
    for limits in [
        exact,
        Limits {
            max_work: exact.max_work - 1,
            ..exact
        },
        Limits {
            max_bytes: exact.max_bytes - 1,
            ..exact
        },
    ] {
        let sequential =
            simplify_lod_chain(&input, &mut Workspace::new(limits)).map(|v| lod_bytes(&v));
        if limits == exact {
            assert_eq!(sequential, Ok(lod_bytes(&expected)));
        } else {
            assert_eq!(sequential, Err(Error::LimitExceeded));
        }
        for pool in pools() {
            let result = pool
                .install(|| simplify_lod_chains_batch(&[input; 7], limits))
                .unwrap();
            assert!(result
                .into_iter()
                .all(|v| v.map(|v| lod_bytes(&v)) == sequential));
        }
    }
}

#[test]
fn mixed_encodings_and_limits_are_independent_of_worker_history() {
    let buffers: Vec<Vec<u8>> = (0..64)
        .map(|i| {
            (0..(i % 9 + 1) * 12)
                .map(|j| (i * 13 + j * 7) as u8)
                .collect()
        })
        .collect();
    let indices = [0, 1, 2, 2, 1, 3];
    let sequence = [1, 42, 7, 9, 2, 123];
    let invalid = [1, 2];
    let mut inputs = Vec::new();
    for (i, bytes) in buffers.iter().enumerate() {
        inputs.extend([
            EncodeInput::Vertices {
                vertices: bytes,
                count: bytes.len() / 12,
                stride: 12,
                encoding: VertexEncoding::new((i % 2) as u8, (i % 10) as u8).unwrap(),
            },
            EncodeInput::Triangles {
                indices: &indices,
                encoding: IndexEncoding::new((i % 2) as u8).unwrap(),
            },
            EncodeInput::Sequence {
                indices: &sequence,
                encoding: IndexEncoding::new((i % 2) as u8).unwrap(),
            },
        ]);
    }
    inputs.insert(
        4,
        EncodeInput::Triangles {
            indices: &invalid,
            encoding: IndexEncoding::DEFAULT,
        },
    );
    inputs.push(EncodeInput::Vertices {
        vertices: &[],
        count: usize::MAX,
        stride: 12,
        encoding: VertexEncoding::DEFAULT,
    });
    for limits in [
        Limits::default(),
        Limits {
            max_bytes: 128,
            max_work: 512,
        },
        Limits {
            max_bytes: 0,
            max_work: 0,
        },
    ] {
        let expected: Vec<_> = inputs
            .iter()
            .map(|input| input.encode(&mut Workspace::new(limits)))
            .collect();
        for pool in pools() {
            for _ in 0..3 {
                assert_eq!(
                    pool.install(|| encode_buffers_batch(&inputs, limits))
                        .unwrap(),
                    expected
                );
            }
            assert!(pool
                .install(|| encode_buffers_batch(&[], limits))
                .unwrap()
                .is_empty());
        }
    }
}

#[test]
fn complete_views_match_sequential_decoding_including_filter_failures() {
    let mut encoded = Vec::new();
    let mut views = Vec::new();
    for i in 0..24 {
        let mut ws = Workspace::default();
        let count = i + 1;
        let vectors: Vec<_> = (0..count)
            .flat_map(|j| [j as f32 / 4., 1., 0.25, 1.])
            .collect();
        let (filter, stride, raw) = match i % 4 {
            0 => (
                Filter::None,
                16,
                vectors.iter().flat_map(|v| v.to_le_bytes()).collect(),
            ),
            1 => (
                Filter::Octahedral,
                8,
                encode_filter_oct(count, 8, 12, &vectors, &mut ws).unwrap(),
            ),
            2 => (
                Filter::Quaternion,
                8,
                encode_filter_quat(count, 8, 12, &vectors, &mut ws).unwrap(),
            ),
            _ => (
                Filter::Exponential,
                16,
                encode_filter_exp(count, 16, 16, &vectors, ExpMode::SharedVector, &mut ws).unwrap(),
            ),
        };
        encoded.push(
            encode_vertex_buffer(
                &raw,
                count,
                stride,
                VertexEncoding::new(0, 2).unwrap(),
                &mut ws,
            )
            .unwrap(),
        );
        views.push(BufferView::new(Mode::Attributes, filter, count, stride).unwrap());
    }
    let mut inputs: Vec<_> = encoded
        .iter()
        .zip(views)
        .map(|(source, view)| DecodeInput { source, view })
        .collect();
    inputs.insert(
        3,
        DecodeInput {
            source: &[0xff],
            ..inputs[0]
        },
    );
    let zero = encode_vertex_buffer(
        &[0; 8],
        1,
        8,
        VertexEncoding::new(0, 2).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    inputs.push(DecodeInput {
        source: &zero,
        view: BufferView::new(Mode::Attributes, Filter::Octahedral, 1, 8).unwrap(),
    });
    for limits in [
        Limits::default(),
        Limits {
            max_bytes: 64,
            max_work: 128,
        },
    ] {
        let expected: Vec<_> = inputs
            .iter()
            .map(|i| i.view.decode(i.source, &mut Workspace::new(limits)))
            .collect();
        assert_eq!(expected[3], Err(Error::InvalidStream));
        assert_eq!(expected.last().unwrap(), &Err(Error::NumericalFailure));
        for pool in pools() {
            assert_eq!(
                pool.install(|| decode_buffer_views_batch(&inputs, limits))
                    .unwrap(),
                expected
            );
        }
    }
}

#[test]
fn every_meshlet_builder_matches_sequential_packed_output_and_errors() {
    let meshes: Vec<_> = (0..32).map(|i| mesh(2 + i % 7, i)).collect();
    let mut inputs: Vec<_> = meshes
        .iter()
        .enumerate()
        .map(|(i, (p, indices))| MeshletInput {
            indices,
            positions: Positions::from_packed(p),
            settings: MeshletSettings {
                max_vertices: 16,
                max_triangles: 12,
            },
            builder: match i % 4 {
                0 => MeshletBuilder::Scan,
                1 => MeshletBuilder::Standard { cone_weight: 0.5 },
                2 => MeshletBuilder::Flex {
                    min_triangles: 4,
                    cone_weight: 0.25,
                    split_factor: 2.,
                },
                _ => MeshletBuilder::Spatial {
                    min_triangles: 4,
                    fill_weight: 0.5,
                },
            },
        })
        .collect();
    inputs.insert(
        5,
        MeshletInput {
            indices: &[0, 9999, 2],
            ..inputs[0]
        },
    );
    for limits in [
        Limits::default(),
        Limits {
            max_bytes: 2048,
            max_work: 500,
        },
    ] {
        let expected: Vec<_> = inputs
            .iter()
            .map(|i| {
                i.build(&mut Workspace::new(limits))
                    .map(|v| meshlet_bytes(&v))
            })
            .collect();
        assert_eq!(expected[5], Err(Error::IndexOutOfBounds));
        for pool in pools() {
            for _ in 0..3 {
                let actual = pool
                    .install(|| build_meshlets_batch(&inputs, limits))
                    .unwrap();
                assert_eq!(
                    actual
                        .into_iter()
                        .map(|v| v.map(|v| meshlet_bytes(&v)))
                        .collect::<Vec<_>>(),
                    expected
                );
            }
        }
    }
}

#[cfg(feature = "clusterlod")]
fn bounds_bytes(b: clusterlod::LodBounds, bytes: &mut Vec<u8>) {
    words(
        b.center
            .into_iter()
            .chain([b.radius, b.error])
            .map(f32::to_bits),
        bytes,
    );
}
#[cfg(feature = "clusterlod")]
fn group_bytes(groups: &[clusterlod::GroupOutput]) -> Vec<u8> {
    let mut bytes = Vec::new();
    words([groups.len() as u32], &mut bytes);
    for group in groups {
        words(
            [group.group.depth as u32, group.clusters.len() as u32],
            &mut bytes,
        );
        bounds_bytes(group.group.simplified, &mut bytes);
        for cluster in &group.clusters {
            words(
                [
                    cluster.refined as u32,
                    cluster.vertex_count as u32,
                    cluster.indices.len() as u32,
                ],
                &mut bytes,
            );
            bounds_bytes(cluster.bounds, &mut bytes);
            words(cluster.indices.iter().copied(), &mut bytes);
        }
    }
    bytes
}
#[cfg(feature = "clusterlod")]
fn node_bytes(nodes: &[clusterlod::Node]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for node in nodes {
        bounds_bytes(node.bounds, &mut bytes);
        words(
            [node.group as u32, node.child_offset, node.child_count],
            &mut bytes,
        );
    }
    bytes
}

#[cfg(feature = "clusterlod")]
#[test]
fn cluster_lod_dags_dilation_and_hierarchies_match_at_each_thread_count() {
    let meshes: Vec<_> = (0..12).map(|i| mesh(3 + i % 5, i)).collect();
    let config = |i: usize| {
        let mut c = clusterlod::default_config(8).unwrap();
        c.simplify_dilate_borders = i.is_multiple_of(2);
        if i == 4 {
            c.simplify_ratio = 1.;
        }
        c
    };
    for limits in [
        Limits::default(),
        Limits {
            max_bytes: 4096,
            max_work: 2000,
        },
    ] {
        let mut sequential_positions: Vec<_> = meshes.iter().map(|(p, _)| p.clone()).collect();
        let expected: Vec<_> = sequential_positions
            .iter_mut()
            .enumerate()
            .map(|(i, p)| {
                clusterlod::build(
                    config(i),
                    clusterlod::Mesh {
                        indices: &meshes[i].1,
                        positions: p,
                        attributes: None,
                        vertex_lock: None,
                        attribute_weights: &[],
                        attribute_protect_mask: 0,
                    },
                    &mut Workspace::new(limits),
                )
            })
            .collect();
        let expected_bytes: Vec<_> = expected
            .iter()
            .map(|r| r.as_ref().map(|v| group_bytes(v)).map_err(|e| *e))
            .collect();
        assert_eq!(expected_bytes[4], Err(Error::InvalidParameter));
        let groups: Vec<Vec<_>> = expected
            .iter()
            .map(|r| {
                r.as_ref()
                    .map(|v| v.iter().map(|g| g.group).collect())
                    .unwrap_or_default()
            })
            .collect();
        let hierarchy_inputs: Vec<_> = groups
            .iter()
            .map(|groups| ClusterHierarchyInput {
                groups,
                node_width: 4,
                level_count: groups
                    .iter()
                    .map(|g| g.depth as usize + 1)
                    .max()
                    .unwrap_or(0),
            })
            .collect();
        let expected_nodes: Vec<_> = hierarchy_inputs
            .iter()
            .map(|i| {
                clusterlod::build_hierarchy(
                    i.groups,
                    i.node_width,
                    i.level_count,
                    &mut Workspace::new(limits),
                )
                .map(|v| node_bytes(&v))
            })
            .collect();
        for pool in pools() {
            let mut p: Vec<_> = meshes.iter().map(|(p, _)| p.clone()).collect();
            let mut inputs: Vec<_> = p
                .iter_mut()
                .enumerate()
                .map(|(i, p)| ClusterLodInput {
                    config: config(i),
                    mesh: clusterlod::Mesh {
                        indices: &meshes[i].1,
                        positions: p,
                        attributes: None,
                        vertex_lock: None,
                        attribute_weights: &[],
                        attribute_protect_mask: 0,
                    },
                })
                .collect();
            let actual = pool
                .install(|| build_cluster_lod_batch(&mut inputs, limits))
                .unwrap();
            assert_eq!(
                actual
                    .into_iter()
                    .map(|v| v.map(|v| group_bytes(&v)))
                    .collect::<Vec<_>>(),
                expected_bytes
            );
            assert_eq!(
                p.iter()
                    .map(|p| p.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                sequential_positions
                    .iter()
                    .map(|p| p.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            );
            let actual = pool
                .install(|| build_cluster_hierarchies_batch(&hierarchy_inputs, limits))
                .unwrap();
            assert_eq!(
                actual
                    .into_iter()
                    .map(|v| v.map(|v| node_bytes(&v)))
                    .collect::<Vec<_>>(),
                expected_nodes
            );
        }
    }
}

#[test]
fn global_pool_and_concurrent_calls_keep_order() {
    let input = [EncodeInput::Sequence {
        indices: &[3, 1, 2, 0],
        encoding: IndexEncoding::DEFAULT,
    }; 13];
    let expected = encode_buffers_batch(&input, Limits::default()).unwrap();
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_buffers_batch(&input, Limits::default()).unwrap()))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
}
