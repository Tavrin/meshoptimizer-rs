use meshoptimizer_rs::*;
fn mesh() -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..81)
        .map(|i| [(i % 9) as f32, (i / 9) as f32, ((i % 9) as f32 * 0.3).sin()])
        .collect();
    let mut idx = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            let a = y * 9 + x;
            idx.extend([a, a + 1, a + 9, a + 9, a + 1, a + 10]);
        }
    }
    (p, idx)
}
#[test]
fn early_errors_replace_previous_workspace_measurements() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let p = Positions::from_packed(&p);
    let indices = [0, 1, 2];
    let mut workspace = Workspace::default();
    for variant in 0..14 {
        spatial_sort_remap(p, &mut workspace).unwrap();
        assert!(workspace.usage().work > 0 && workspace.usage().bytes > 0);
        let s = MeshletSettings::default();
        let result = match variant {
            0 => build_meshlets_scan(
                &indices,
                3,
                MeshletSettings {
                    max_vertices: 2,
                    ..s
                },
                &mut workspace,
            )
            .map(|_| ()),
            1 => {
                build_meshlets_scan_into(&mut [], &mut [], &mut [], &indices, 3, s, &mut workspace)
                    .map(|_| ())
            }
            2 => compute_cluster_bounds(&[0; 1539], p, &mut workspace).map(|_| ()),
            3 => compute_meshlet_bounds(&indices, &[0], p, &mut workspace).map(|_| ()),
            4 => {
                extract_meshlet_indices_into(&mut [], &mut [], &indices, &mut workspace).map(|_| ())
            }
            5 => optimize_meshlet_in_place(&mut [0, 1, 2], &mut [3, 0, 0], &mut workspace),
            6 => optimize_meshlet_into(&mut [], &mut [], &indices, &[0, 1, 2], &mut workspace),
            7 => optimize_meshlet(&indices, &[0], &mut workspace).map(|_| ()),
            8 => partition_clusters_into(&mut [], &indices, &[3], 3, Some(p), 1, &mut workspace)
                .map(|_| ()),
            9 => spatial_sort_remap_into(&mut [], p, &mut workspace),
            10 => spatial_sort_triangles_into(&mut [], &indices, p, &mut workspace),
            11 => spatial_cluster_points(p, 0, &mut workspace).map(|_| ()),
            12 => spatial_cluster_points_into(&mut [], p, 1, &mut workspace),
            _ => spatial_cluster_points_into(&mut [0; 3], p, 0, &mut workspace),
        };
        assert!(result.is_err(), "variant {variant}");
        assert_eq!(workspace.usage(), Usage::default(), "variant {variant}");
    }
    #[cfg(feature = "clusterlod")]
    {
        spatial_sort_remap(p, &mut workspace).unwrap();
        assert_eq!(
            clusterlod::build_hierarchy(&[], 0, 0, &mut workspace),
            Err(Error::InvalidParameter)
        );
        assert_eq!(workspace.usage(), Usage::default());
    }
}
#[test]
fn packed_meshlets_and_caller_buffers_preserve_topology_and_tails() {
    let (p, idx) = mesh();
    let p = Positions::from_packed(&p);
    let s = MeshletSettings {
        max_vertices: 16,
        max_triangles: 12,
    };
    let mut ws = Workspace::default();
    let bound = build_meshlets_bound(idx.len(), 16, 4).unwrap();
    for variant in 0..4 {
        let expected = match variant {
            0 => build_meshlets(&idx, p, s, 0.5, &mut ws),
            1 => build_meshlets_scan(&idx, p.len(), s, &mut ws),
            2 => build_meshlets_flex(&idx, p, s, 4, 0.5, 2., &mut ws),
            _ => build_meshlets_spatial(&idx, p, s, 4, 0.5, &mut ws),
        }
        .unwrap();
        let marker = Meshlet {
            vertex_offset: 123,
            triangle_offset: 123,
            vertex_count: 123,
            triangle_count: 123,
        };
        let mut m = vec![marker; bound + 2];
        let mut v = vec![u32::MAX; idx.len() + 2];
        let mut t = vec![255; idx.len() + 2];
        let sizes = match variant {
            0 => build_meshlets_into(&mut m, &mut v, &mut t, &idx, p, s, 0.5, &mut ws),
            1 => build_meshlets_scan_into(&mut m, &mut v, &mut t, &idx, p.len(), s, &mut ws),
            2 => build_meshlets_flex_into(&mut m, &mut v, &mut t, &idx, p, s, 4, 0.5, 2., &mut ws),
            _ => build_meshlets_spatial_into(&mut m, &mut v, &mut t, &idx, p, s, 4, 0.5, &mut ws),
        }
        .unwrap();
        assert_eq!(&m[..sizes.meshlets], expected.meshlets);
        assert_eq!(&v[..sizes.vertices], expected.vertices);
        assert_eq!(&t[..sizes.triangles], expected.triangles);
        assert!(m[sizes.meshlets..].iter().all(|&v| v == marker));
        assert!(v[sizes.vertices..].iter().all(|&v| v == u32::MAX));
        assert!(t[sizes.triangles..].iter().all(|&v| v == 255));
        let mut actual = Vec::new();
        let (mut vo, mut to) = (0, 0);
        for m in &expected.meshlets {
            assert_eq!(m.vertex_offset, vo);
            assert_eq!(m.triangle_offset, to);
            for tri in expected.triangles[to as usize..][..m.triangle_count as usize * 3]
                .as_chunks::<3>()
                .0
                .iter()
            {
                actual.push(
                    tri.iter()
                        .map(|&i| expected.vertices[vo as usize + i as usize])
                        .collect::<Vec<_>>(),
                );
            }
            vo += m.vertex_count;
            to += m.triangle_count * 3;
        }
        let mut original = idx
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| t.to_vec())
            .collect::<Vec<_>>();
        actual.sort();
        original.sort();
        assert_eq!(actual, original);
    }
}
#[test]
fn utilities_handle_collision_unused_vertices_wrapping_valence_and_rotation() {
    let idx = [0, 7, 15, 1039, 7, 15];
    let mut ws = Workspace::default();
    let local = extract_meshlet_indices(&idx, &mut ws).unwrap();
    assert_eq!(local.vertices, [0, 7, 15, 1039]);
    assert_eq!(local.triangles, [0, 1, 2, 3, 1, 2]);
    for level in 0..=9 {
        let mut vertices = local.vertices.clone();
        vertices.push(42);
        let out = optimize_meshlet_level(&vertices, &local.triangles, level, &mut ws).unwrap();
        assert_eq!(out.vertices.last(), Some(&42));
        let mut v = vec![123; vertices.len() + 1];
        let mut t = vec![234; local.triangles.len() + 1];
        optimize_meshlet_level_into(&mut v, &mut t, &vertices, &local.triangles, level, &mut ws)
            .unwrap();
        assert_eq!(&v[..vertices.len()], out.vertices);
        assert_eq!(&t[..local.triangles.len()], out.triangles);
        assert_eq!(v[vertices.len()], 123);
        assert_eq!(t[local.triangles.len()], 234);
        let original = vertices.clone();
        let mut tp = vec![0; 1536];
        optimize_meshlet_level_in_place(&mut vertices, &mut tp, level, &mut ws).unwrap();
        assert_eq!(vertices.len(), original.len());
    }
}
#[test]
fn bounds_keep_zero_degenerate_cones_and_enclose_spheres() {
    let p = [[0., 0., 0.], [0., 1., 0.], [0., 0., 1.], [1., 0., 1.]];
    let p = Positions::from_packed(&p);
    let radii = [0., 1., 2., 3.];
    let r = Attributes::from_interleaved(&radii, 4, 1, 1, 0).unwrap();
    let mut ws = Workspace::default();
    let sphere = compute_sphere_bounds(p, Some(r), &mut ws).unwrap();
    assert_eq!(sphere.center, [1., 0., 1.]);
    assert_eq!(sphere.radius, 3.);
    assert_eq!(
        compute_cluster_bounds(&[0, 0, 0], p, &mut ws).unwrap(),
        Bounds::default()
    );
    let b = compute_cluster_bounds(&[0, 1, 2, 0, 2, 1], p, &mut ws).unwrap();
    assert_eq!(b.cone_cutoff, 1.);
    assert_eq!(b.cone_cutoff_s8, 127);
    assert_eq!(b.cone_axis, [0.; 3]);
}
#[test]
fn spatial_permutation_directions_and_strided_identity() {
    let (p, idx) = mesh();
    let mut interleaved = Vec::new();
    for xyz in &p {
        interleaved.extend([123., xyz[0], xyz[1], xyz[2], 456.]);
    }
    let packed = Positions::from_packed(&p);
    let strided = Positions::from_interleaved(&interleaved, p.len(), 5, 1).unwrap();
    let mut ws = Workspace::default();
    let remap = spatial_sort_remap(packed, &mut ws).unwrap();
    assert_eq!(spatial_sort_remap(strided, &mut ws).unwrap(), remap);
    let mut sorted = remap.clone();
    sorted.sort();
    assert_eq!(sorted, (0..p.len() as u32).collect::<Vec<_>>());
    let tri = spatial_sort_triangles(&idx, packed, &mut ws).unwrap();
    assert_eq!(spatial_sort_triangles(&idx, strided, &mut ws).unwrap(), tri);
    let mut inplace = idx.clone();
    spatial_sort_triangles_in_place(&mut inplace, packed, &mut ws).unwrap();
    assert_eq!(inplace, tri);
    let mut out = vec![u32::MAX; idx.len() + 3];
    spatial_sort_triangles_into(&mut out, &idx, strided, &mut ws).unwrap();
    assert_eq!(&out[..idx.len()], tri);
    assert_eq!(&out[idx.len()..], &[u32::MAX; 3]);
    for size in [1, 2, 8, 81, usize::MAX] {
        let v = spatial_cluster_points(strided, size, &mut ws).unwrap();
        let mut sorted = v.clone();
        sorted.sort();
        assert_eq!(sorted, (0..p.len() as u32).collect::<Vec<_>>());
    }
}
#[test]
fn partition_arbitrary_lists_empty_and_disconnected() {
    let mut ws = Workspace::default();
    let idx = [0, 1, 2, 3, 4, 5];
    assert_eq!(
        partition_clusters(&idx, &[3, 3], 6, None, 2, &mut ws)
            .unwrap()
            .count,
        2
    );
    let p = [[0.; 3]; 6];
    assert_eq!(
        partition_clusters(
            &idx,
            &[3, 3],
            6,
            Some(Positions::from_packed(&p)),
            2,
            &mut ws
        )
        .unwrap()
        .count,
        1
    );
    assert_eq!(
        partition_clusters(&[], &[], 0, None, 1, &mut ws)
            .unwrap()
            .count,
        0
    );
    assert_eq!(
        partition_clusters(&[0, 1], &[1, 1], 2, None, 1, &mut ws)
            .unwrap()
            .count,
        2
    );
    assert_eq!(
        partition_clusters(&idx, &[0, 6], 6, None, 2, &mut ws),
        Err(Error::InvalidParameter)
    );
}
#[test]
fn validation_and_exact_resource_boundaries() {
    let (p, idx) = mesh();
    let p = Positions::from_packed(&p);
    let mut ws = Workspace::default();
    assert_eq!(build_meshlets_bound(4, 64, 64), Err(Error::InvalidTopology));
    assert_eq!(build_meshlets_bound(0, 2, 64), Err(Error::InvalidParameter));
    assert_eq!(
        spatial_cluster_points(p, 0, &mut ws),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        compute_cluster_bounds(&[999, 1, 2], p, &mut ws),
        Err(Error::IndexOutOfBounds)
    );
    assert_eq!(
        optimize_meshlet(&[0], &[0, 1, 0], &mut ws),
        Err(Error::IndexOutOfBounds)
    );
    assert_eq!(
        extract_meshlet_indices(&(0..258).collect::<Vec<_>>(), &mut ws),
        Err(Error::InvalidParameter)
    );
    for op in 0..4 {
        ws.clear();
        let expected = match op {
            0 => spatial_sort_remap(p, &mut ws),
            1 => spatial_sort_triangles(&idx, p, &mut ws),
            2 => spatial_cluster_points(p, 8, &mut ws),
            _ => {
                let out = build_meshlets_scan(&idx, p.len(), MeshletSettings::default(), &mut ws)
                    .unwrap();
                Ok(out.vertices)
            }
        }
        .unwrap();
        let usage = ws.usage();
        let run = |ws: &mut Workspace| match op {
            0 => spatial_sort_remap(p, ws),
            1 => spatial_sort_triangles(&idx, p, ws),
            2 => spatial_cluster_points(p, 8, ws),
            _ => build_meshlets_scan(&idx, p.len(), MeshletSettings::default(), ws)
                .map(|x| x.vertices),
        };
        let mut limited = Workspace::new(Limits {
            max_bytes: usage.bytes,
            max_work: usage.work,
        });
        assert_eq!(run(&mut limited).unwrap(), expected);
        limited.set_limits(Limits {
            max_bytes: usage.bytes - 1,
            max_work: usage.work,
        });
        assert_eq!(run(&mut limited), Err(Error::LimitExceeded));
        limited.set_limits(Limits {
            max_bytes: usage.bytes,
            max_work: usage.work - 1,
        });
        assert_eq!(run(&mut limited), Err(Error::LimitExceeded));
    }
}
#[cfg(feature = "clusterlod")]
#[test]
fn demo_hierarchy_references_progress_and_explicit_errors() {
    let (mut p, idx) = mesh();
    let mut ws = Workspace::default();
    let config = clusterlod::default_config(16).unwrap();
    let groups = clusterlod::build(
        config,
        clusterlod::Mesh {
            indices: &idx,
            positions: &mut p,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        &mut ws,
    )
    .unwrap();
    assert!(!groups.is_empty());
    for (i, g) in groups.iter().enumerate() {
        for c in &g.clusters {
            assert!(c.refined < 0 || c.refined < i as i32);
            assert!(!c.indices.is_empty());
        }
    }
    let meta = groups.iter().map(|g| g.group).collect::<Vec<_>>();
    let levels = meta.iter().map(|g| g.depth as usize + 1).max().unwrap();
    let nodes = clusterlod::build_hierarchy(&meta, 4, levels, &mut ws).unwrap();
    assert!(nodes.len() <= clusterlod::build_hierarchy_bound(meta.len(), 4, levels).unwrap());
    assert_eq!(
        clusterlod::build_hierarchy_bound(1, 1, 1),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        clusterlod::build_hierarchy(&[], 4, 1, &mut ws),
        Err(Error::InvalidParameter)
    );
}

#[test]
fn all_processing_families_enforce_work_and_retained_storage() {
    let (packed, indices) = mesh();
    let p = Positions::from_packed(&packed);
    let mut base = Workspace::default();
    let local = extract_meshlet_indices(&indices, &mut base).unwrap();
    let counts = [indices.len() as u32];
    for family in 1..=15 {
        if family == 5 {
            continue;
        } // A scalar checked size helper owns no storage.
        let run = |ws: &mut Workspace| -> Result<(), Error> {
            match family {
                1 => build_meshlets(&indices, p, MeshletSettings::default(), 0.5, ws).map(|_| ()),
                2 => build_meshlets_scan(&indices, p.len(), MeshletSettings::default(), ws)
                    .map(|_| ()),
                3 => build_meshlets_flex(&indices, p, MeshletSettings::default(), 16, 0.5, 2., ws)
                    .map(|_| ()),
                4 => build_meshlets_spatial(&indices, p, MeshletSettings::default(), 16, 0.5, ws)
                    .map(|_| ()),
                6 => compute_cluster_bounds(&indices, p, ws).map(|_| ()),
                7 => compute_meshlet_bounds(&local.vertices, &local.triangles, p, ws).map(|_| ()),
                8 => compute_sphere_bounds(p, None, ws).map(|_| ()),
                9 => optimize_meshlet(&local.vertices, &local.triangles, ws).map(|_| ()),
                10 => optimize_meshlet_level(&local.vertices, &local.triangles, 3, ws).map(|_| ()),
                11 => extract_meshlet_indices(&indices, ws).map(|_| ()),
                12 => partition_clusters(&indices, &counts, p.len(), Some(p), 4, ws).map(|_| ()),
                13 => spatial_sort_remap(p, ws).map(|_| ()),
                14 => spatial_sort_triangles(&indices, p, ws).map(|_| ()),
                _ => spatial_cluster_points(p, 8, ws).map(|_| ()),
            }
        };
        let mut ws = Workspace::default();
        run(&mut ws).unwrap();
        let usage = ws.usage();
        let mut exact = Workspace::new(Limits {
            max_bytes: usage.bytes,
            max_work: usage.work,
        });
        run(&mut exact).unwrap();
        exact.set_limits(Limits {
            max_bytes: usage.bytes,
            max_work: usage.work - 1,
        });
        assert_eq!(
            run(&mut exact),
            Err(Error::LimitExceeded),
            "family {family}"
        );
        if usage.bytes > 0 {
            exact.set_limits(Limits {
                max_bytes: usage.bytes - 1,
                max_work: usage.work,
            });
            assert_eq!(
                run(&mut exact),
                Err(Error::LimitExceeded),
                "family {family}"
            );
        }
        let cached = optimize_vertex_cache(&indices, p.len(), &mut ws).unwrap();
        let retained = ws.usage().bytes - cached.capacity() * 4;
        drop(cached);
        run(&mut ws).unwrap();
        assert!(ws.usage().bytes >= retained);
    }
}
#[cfg(feature = "clusterlod")]
#[test]
fn demo_limits_charge_outputs_scratch_and_callback_lifetimes() {
    let (p, idx) = mesh();
    let c = clusterlod::default_config(8).unwrap();
    let run = |ws: &mut Workspace| {
        let mut positions = p.clone();
        clusterlod::build(
            c,
            clusterlod::Mesh {
                indices: &idx,
                positions: &mut positions,
                attributes: None,
                vertex_lock: None,
                attribute_weights: &[],
                attribute_protect_mask: 0,
            },
            ws,
        )
    };
    let mut ws = Workspace::default();
    let expected = run(&mut ws).unwrap();
    let usage = ws.usage();
    let mut exact = Workspace::new(Limits {
        max_bytes: usage.bytes,
        max_work: usage.work,
    });
    assert_eq!(run(&mut exact).unwrap(), expected);
    exact.set_limits(Limits {
        max_bytes: usage.bytes - 1,
        max_work: usage.work,
    });
    assert_eq!(run(&mut exact), Err(Error::LimitExceeded));
    exact.set_limits(Limits {
        max_bytes: usage.bytes,
        max_work: usage.work - 1,
    });
    assert_eq!(run(&mut exact), Err(Error::LimitExceeded));
    let mut positions = p.clone();
    let mut ids = Vec::new();
    clusterlod::build_with_output(
        c,
        clusterlod::Mesh {
            indices: &idx,
            positions: &mut positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        |_, clusters| {
            for cl in clusters {
                if cl.refined >= 0 {
                    assert!(ids.contains(&cl.refined));
                }
            }
            let id = ids.len() as i32 * 7 + 2;
            ids.push(id);
            Ok(id)
        },
        &mut ws,
    )
    .unwrap();
    assert!(ws.usage().bytes <= usage.bytes);
}

#[cfg(feature = "clusterlod")]
#[test]
fn demo_single_cluster_unused_attributes_do_not_consume_workspace() {
    let mut positions = vec![[0.0, 0.0, 0.0]; 2_048];
    positions[1] = [1.0, 0.0, 0.0];
    positions[2] = [0.0, 1.0, 0.0];
    let attributes = vec![0.25_f32; positions.len() * 32];
    let weights = [0.5_f32; 32];
    let config = clusterlod::default_config(128).unwrap();
    let run = |bytes: usize, with_attributes: bool| {
        let mut p = positions.clone();
        let mut ws = Workspace::new(Limits {
            max_bytes: bytes,
            max_work: Limits::default().max_work,
        });
        let view = with_attributes
            .then(|| Attributes::from_interleaved(&attributes, p.len(), 32, 32, 0).unwrap());
        let result = clusterlod::build_with_output(
            config,
            clusterlod::Mesh {
                indices: &[0, 1, 2],
                positions: &mut p,
                attributes: view,
                vertex_lock: None,
                attribute_weights: if with_attributes { &weights } else { &[] },
                attribute_protect_mask: 0,
            },
            |_, _| Ok(0),
            &mut ws,
        );
        (result, ws.usage().bytes)
    };
    let (baseline, peak) = run(usize::MAX, false);
    assert!(baseline.is_ok());
    println!("single_cluster_peak_bytes={peak}");
    assert!(run(usize::MAX, true).0.is_ok());
    for limit in (peak.saturating_sub(128)..=peak + 128).step_by(8) {
        assert_eq!(
            run(limit, true).0.is_ok(),
            run(limit, false).0.is_ok(),
            "{limit}"
        );
    }
}

#[cfg(feature = "clusterlod")]
#[test]
fn demo_dilation_near_position_range_threshold() {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for y in 0..12 {
        for x in 0..12 {
            let wave = ((x as f32 * 0.4 + 2.0).sin() * (y as f32 * 0.3).cos()).abs();
            positions.push([99_999_992.0 - wave * 32.0, x as f32 * 8.0, y as f32 * 8.0]);
        }
    }
    for y in 0..11 {
        for x in 0..11 {
            let a = (y * 12 + x) as u32;
            indices.extend([a, a + 1, a + 12, a + 12, a + 1, a + 13]);
        }
    }
    let before = positions.clone();
    assert!(before.iter().all(|p| p[0].abs() <= 1e8));
    let mut config = clusterlod::default_config(32).unwrap();
    config.simplify_dilate_borders = true;
    clusterlod::build_with_output(
        config,
        clusterlod::Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        |_, _| Ok(0),
        &mut Workspace::default(),
    )
    .unwrap();
    assert!(positions.iter().zip(&before).any(|(a, b)| a != b));
    assert!(positions.iter().flatten().all(|v| v.is_finite()));
}

#[cfg(feature = "clusterlod")]
#[test]
fn demo_attribute_layouts_preserve_output_and_workspace_peak() {
    let (positions, indices) = mesh();
    let n = positions.len();
    let packed: Vec<f32> = (0..n)
        .flat_map(|i| [i as f32 * 0.01, 0.25, (i % 7) as f32 * 0.1])
        .collect();
    let mut interleaved = vec![f32::NAN; n * 5];
    let mut little = vec![0xff; n * 20 + 1];
    let mut big = little.clone();
    for i in 0..n {
        for j in 0..3 {
            let value = packed[i * 3 + j];
            interleaved[i * 5 + 1 + j] = value;
            let at = i * 20 + 1 + j * 4;
            little[at..at + 4].copy_from_slice(&value.to_le_bytes());
            big[at..at + 4].copy_from_slice(&value.to_be_bytes());
        }
    }
    let flags: Vec<_> = (0..n)
        .map(|i| match i % 17 {
            0 => VertexFlags::LOCK,
            1 => VertexFlags::PROTECT,
            2 => VertexFlags::PRIORITY,
            _ => VertexFlags::EMPTY,
        })
        .collect();
    let views = [
        Attributes::from_interleaved(&packed, n, 3, 3, 0).unwrap(),
        Attributes::from_interleaved(&interleaved, n, 3, 5, 1).unwrap(),
        Attributes::from_bytes(&little, n, 3, 20, 1, ByteOrder::LittleEndian).unwrap(),
        Attributes::from_bytes(&big, n, 3, 20, 1, ByteOrder::BigEndian).unwrap(),
    ];
    for dilate in [false, true] {
        let mut config = clusterlod::default_config(8).unwrap();
        config.simplify_dilate_borders = dilate;
        let run = |view, limit| {
            let mut p = positions.clone();
            let mut ws = Workspace::new(Limits {
                max_bytes: limit,
                ..Limits::default()
            });
            let out = clusterlod::build(
                config,
                clusterlod::Mesh {
                    positions: &mut p,
                    indices: &indices,
                    attributes: Some(view),
                    vertex_lock: Some(&flags),
                    attribute_weights: &[0.5, 0., 0.25],
                    attribute_protect_mask: 1,
                },
                &mut ws,
            );
            (out, p, ws.usage())
        };
        let expected = run(views[0], usize::MAX);
        assert!(expected
            .0
            .as_ref()
            .unwrap()
            .iter()
            .any(|g| g.group.depth > 0));
        for view in views {
            assert_eq!(run(view, usize::MAX), expected);
            assert_eq!(run(view, expected.2.bytes), expected);
            assert_eq!(run(view, expected.2.bytes - 1).0, Err(Error::LimitExceeded));
        }
    }
}

#[cfg(feature = "clusterlod")]
mod caller_dag {
    use super::*;
    use clusterlod::{DagBuffers, DagCounts, FlatDagCluster, FlatDagGroup, Mesh, Node};

    struct Storage {
        source: Vec<u32>,
        positions: Vec<[f32; 3]>,
        indices: Vec<u32>,
        clusters: Vec<FlatDagCluster>,
        groups: Vec<FlatDagGroup>,
        nodes: Vec<Node>,
    }
    impl Storage {
        fn new() -> Self {
            let mut cluster = FlatDagCluster::default();
            cluster.refined = -123;
            let mut group = FlatDagGroup::default();
            group.depth = -123;
            Self {
                source: vec![u32::MAX; 4096],
                positions: vec![[-123.; 3]; 4096],
                indices: vec![u32::MAX; 16384],
                clusters: vec![cluster; 4096],
                groups: vec![group; 4096],
                nodes: vec![
                    Node {
                        group: -123,
                        ..Node::default()
                    };
                    4096
                ],
            }
        }
        fn buffers(&mut self) -> DagBuffers<'_> {
            DagBuffers {
                vertex_source: &mut self.source,
                vertex_positions: &mut self.positions,
                indices: &mut self.indices,
                clusters: &mut self.clusters,
                groups: &mut self.groups,
                nodes: &mut self.nodes,
            }
        }
        fn words(&self, c: DagCounts) -> Vec<u32> {
            let mut out = vec![
                c.vertices as u32,
                c.indices as u32,
                c.clusters as u32,
                c.groups as u32,
                c.nodes as u32,
                c.level_count,
                c.bvh_depth,
            ];
            out.extend(&self.source[..c.vertices]);
            out.extend(
                self.positions[..c.vertices]
                    .iter()
                    .flatten()
                    .map(|v| v.to_bits()),
            );
            out.extend(&self.indices[..c.indices]);
            for cl in &self.clusters[..c.clusters] {
                out.extend([cl.first_index, cl.index_count, cl.group, cl.refined as u32]);
                out.extend(cl.center.map(f32::to_bits));
                out.push(cl.radius.to_bits());
            }
            for g in &self.groups[..c.groups] {
                out.extend(g.center.map(f32::to_bits));
                out.extend([
                    g.radius.to_bits(),
                    g.error.to_bits(),
                    g.depth as u32,
                    g.first_cluster,
                    g.cluster_count,
                ]);
            }
            for n in &self.nodes[..c.nodes] {
                out.extend(n.bounds.center.map(f32::to_bits));
                out.extend([
                    n.bounds.radius.to_bits(),
                    n.bounds.error.to_bits(),
                    n.group as u32,
                    n.child_offset,
                    n.child_count,
                ]);
            }
            out
        }
    }
    fn input<'a>(p: &'a mut [[f32; 3]], indices: &'a [u32]) -> Mesh<'a> {
        Mesh {
            indices,
            positions: p,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        }
    }
    fn disk() -> (Vec<[f32; 3]>, Vec<u32>) {
        let n = 16;
        let mut p = vec![[0.; 3]];
        p.extend((0..n).map(|i| {
            let a = i as f32 * core::f32::consts::TAU / n as f32;
            [a.cos(), a.sin(), 0.]
        }));
        (
            p,
            (0..n).flat_map(|i| [0, i + 1, (i + 1) % n + 1]).collect(),
        )
    }
    #[test]
    fn flat_dag_matches_callback_order_hierarchy_and_preserves_all_tails() {
        let (original, indices) = super::mesh();
        let config = clusterlod::default_config(16).unwrap();
        let mut p = original.clone();
        let mut ws = Workspace::default();
        let owned = clusterlod::build(config, input(&mut p, &indices), &mut ws).unwrap();
        let meta: Vec<_> = owned.iter().map(|g| g.group).collect();
        let levels = meta.iter().map(|g| g.depth as usize + 1).max().unwrap();
        let nodes = clusterlod::build_hierarchy(&meta, 4, levels, &mut ws).unwrap();
        let mut storage = Storage::new();
        let c = clusterlod::build_dag_into(
            config,
            input(&mut p, &indices),
            4,
            storage.buffers(),
            &mut ws,
        )
        .unwrap();
        let mut keys = std::collections::BTreeMap::new();
        let mut expected_source = Vec::new();
        let mut expected_indices = Vec::new();
        let mut ci = 0;
        for (gid, g) in owned.iter().enumerate() {
            let flat = storage.groups[gid];
            assert_eq!(flat.first_cluster as usize, ci);
            assert_eq!(flat.cluster_count as usize, g.clusters.len());
            assert_eq!(flat.depth, g.group.depth);
            assert_eq!(flat.error.to_bits(), g.group.simplified.error.to_bits());
            assert_eq!(
                flat.center.map(f32::to_bits),
                g.group.simplified.center.map(f32::to_bits)
            );
            assert_eq!(flat.radius.to_bits(), g.group.simplified.radius.to_bits());
            for cl in &g.clusters {
                let flat = storage.clusters[ci];
                ci += 1;
                assert_eq!(flat.first_index as usize, expected_indices.len());
                assert_eq!(flat.index_count as usize, cl.indices.len());
                assert_eq!(flat.group as usize, gid);
                assert_eq!(flat.refined, cl.refined);
                assert_eq!(
                    flat.center.map(f32::to_bits),
                    cl.bounds.center.map(f32::to_bits)
                );
                assert_eq!(flat.radius.to_bits(), cl.bounds.radius.to_bits());
                for &src in &cl.indices {
                    let k = (src, original[src as usize].map(f32::to_bits));
                    let id = *keys.entry(k).or_insert_with(|| {
                        let id = expected_source.len() as u32;
                        expected_source.push(src);
                        id
                    });
                    expected_indices.push(id);
                }
            }
        }
        assert_eq!(&storage.source[..c.vertices], expected_source);
        assert_eq!(&storage.indices[..c.indices], expected_indices);
        assert_eq!(&storage.nodes[..c.nodes], nodes);
        assert_eq!(c.groups, owned.len());
        assert_eq!(c.clusters, ci);
        assert_eq!(c.level_count as usize, levels);
        assert!(storage.source[c.vertices..].iter().all(|&v| v == u32::MAX));
        assert!(storage.positions[c.vertices..]
            .iter()
            .all(|&v| v == [-123.; 3]));
        assert!(storage.indices[c.indices..].iter().all(|&v| v == u32::MAX));
        assert!(storage.clusters[c.clusters..]
            .iter()
            .all(|v| v.refined == -123));
        assert!(storage.groups[c.groups..].iter().all(|v| v.depth == -123));
        assert!(storage.nodes[c.nodes..].iter().all(|v| v.group == -123));
    }
    #[test]
    fn each_caller_buffer_can_fail_without_replacement_output() {
        for short in 0..6 {
            let (mut p, indices) = disk();
            let mut storage = Storage::new();
            let mut dst = storage.buffers();
            match short {
                0 => dst.vertex_source = &mut [],
                1 => dst.vertex_positions = &mut [],
                2 => dst.indices = &mut [],
                3 => dst.clusters = &mut [],
                4 => dst.groups = &mut [],
                _ => dst.nodes = &mut [],
            }
            let result = clusterlod::build_dag_into(
                clusterlod::default_config(8).unwrap(),
                input(&mut p, &indices),
                4,
                dst,
                &mut Workspace::default(),
            );
            assert_eq!(result.unwrap_err(), Error::BufferTooSmall);
        }
    }
    #[test]
    fn dilation_keeps_earlier_snapshots_and_full_dag_limits_are_cumulative() {
        let (original, indices) = disk();
        let mut p = original.clone();
        let mut storage = Storage::new();
        let mut config = clusterlod::default_config(8).unwrap();
        config.simplify_dilate_borders = true;
        let mut ws = Workspace::default();
        let c = clusterlod::build_dag_into(
            config,
            input(&mut p, &indices),
            4,
            storage.buffers(),
            &mut ws,
        )
        .unwrap();
        let usage = ws.usage();
        assert_ne!(p, original);
        let mut distinct = std::collections::BTreeSet::new();
        let mut repeated_source = false;
        for i in 0..c.vertices {
            assert!(distinct.insert((storage.source[i], storage.positions[i].map(f32::to_bits))));
            repeated_source |= storage.source[..i].contains(&storage.source[i]);
        }
        assert!(
            repeated_source,
            "moved source must have multiple emission snapshots"
        );
        let leaf = storage
            .groups
            .iter()
            .take(c.groups)
            .find(|g| g.depth == 0)
            .unwrap();
        for cl in &storage.clusters
            [leaf.first_cluster as usize..(leaf.first_cluster + leaf.cluster_count) as usize]
        {
            for &v in &storage.indices
                [cl.first_index as usize..(cl.first_index + cl.index_count) as usize]
            {
                assert_eq!(
                    storage.positions[v as usize].map(f32::to_bits),
                    original[storage.source[v as usize] as usize].map(f32::to_bits)
                );
            }
        }
        for limits in [
            Limits {
                max_bytes: usage.bytes - 1,
                max_work: usage.work,
            },
            Limits {
                max_bytes: usage.bytes,
                max_work: usage.work - 1,
            },
        ] {
            let mut p = original.clone();
            let result = clusterlod::build_dag_into(
                config,
                input(&mut p, &indices),
                4,
                storage.buffers(),
                &mut Workspace::new(limits),
            );
            assert_eq!(result.unwrap_err(), Error::LimitExceeded);
        }
    }
    #[test]
    fn empty_and_invalid_width_have_explicit_results() {
        let mut storage = Storage::new();
        let mut p = [];
        let c = clusterlod::build_dag_into(
            clusterlod::default_config(8).unwrap(),
            input(&mut p, &[]),
            4,
            storage.buffers(),
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(
            (
                c.vertices,
                c.indices,
                c.clusters,
                c.groups,
                c.nodes,
                c.level_count,
                c.bvh_depth
            ),
            (0, 0, 0, 0, 0, 0, 0)
        );
        let mut workspace = Workspace::default();
        spatial_sort_remap(
            Positions::from_packed(&[[0., 0., 0.], [1., 0., 0.]]),
            &mut workspace,
        )
        .unwrap();
        assert!(workspace.usage().work > 0);
        assert_eq!(
            clusterlod::build_dag_into(
                clusterlod::default_config(8).unwrap(),
                input(&mut p, &[]),
                1,
                storage.buffers(),
                &mut workspace
            )
            .unwrap_err(),
            Error::InvalidParameter
        );
        assert_eq!(workspace.usage(), Usage::default());
    }
    #[cfg(feature = "parallel")]
    #[test]
    fn caller_dag_batches_match_at_1_2_8_threads_and_isolate_ordered_errors() {
        use meshoptimizer_rs::parallel::{build_dags_batch_into, DagBuildInput};
        let (original, indices) = disk();
        let mut config = clusterlod::default_config(8).unwrap();
        config.simplify_dilate_borders = true;
        let configs = [config; 4];
        let mut expected = Vec::new();
        let mut expected_positions = Vec::new();
        for i in 0..4 {
            let mut p = original.clone();
            let mut storage = Storage::new();
            let mut dst = storage.buffers();
            if i == 1 {
                dst.indices = &mut [];
            }
            let result = clusterlod::build_dag_into(
                config,
                input(&mut p, &indices),
                if i == 2 { 1 } else { 4 },
                dst,
                &mut Workspace::default(),
            );
            expected.push(result.map(|c| storage.words(c)));
            expected_positions.push(p);
        }
        assert!(expected[0].is_ok());
        assert_eq!(expected[1], Err(Error::BufferTooSmall));
        assert_eq!(expected[2], Err(Error::InvalidParameter));
        assert!(expected[3].is_ok());
        for threads in [1, 2, 8] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            let mut positions = vec![original.clone(); 4];
            let mut storage: Vec<_> = (0..4).map(|_| Storage::new()).collect();
            let mut inputs: Vec<_> = positions
                .iter_mut()
                .zip(&mut storage)
                .enumerate()
                .map(|(i, (p, s))| {
                    let mut dst = s.buffers();
                    if i == 1 {
                        dst.indices = &mut [];
                    }
                    DagBuildInput {
                        config: configs[i],
                        mesh: input(p, &indices),
                        node_width: if i == 2 { 1 } else { 4 },
                        destination: dst,
                    }
                })
                .collect();
            let results = pool
                .install(|| build_dags_batch_into(&mut inputs, Limits::default()))
                .unwrap();
            drop(inputs);
            for (i, result) in results.into_iter().enumerate() {
                assert_eq!(result.map(|c| storage[i].words(c)), expected[i]);
            }
            assert_eq!(positions, expected_positions);
        }
    }
}

fn bounds_bits(result: Result<Bounds, Error>) -> Result<([u32; 11], [i8; 4]), Error> {
    result.map(|b| {
        let mut floats = [0.; 11];
        floats[..3].copy_from_slice(&b.center);
        floats[3] = b.radius;
        floats[4..7].copy_from_slice(&b.cone_apex);
        floats[7..10].copy_from_slice(&b.cone_axis);
        floats[10] = b.cone_cutoff;
        (
            floats.map(f32::to_bits),
            [
                b.cone_axis_s8[0],
                b.cone_axis_s8[1],
                b.cone_axis_s8[2],
                b.cone_cutoff_s8,
            ],
        )
    })
}

#[test]
fn cached_cluster_bounds_preserve_bits_errors_and_usage() {
    let (base, indices) = mesh();
    let large = indices.repeat(4);
    for scale in [1., 1.0e9, f32::MAX, f32::NAN, f32::INFINITY] {
        let mut points = base.clone();
        // Include an unused invalid/extreme position in the cached scan.
        points.push([scale, -0., 0.]);
        let positions = Positions::from_packed(&points);
        let cached = BoundsPositions::new(positions);
        for topology in [
            &indices[..3],
            &indices[..96],
            &indices[..384],
            &large[..],
            &[][..],
            &[0][..],
            &[u32::MAX, 0, 0][..],
            &[0; 1539][..],
        ] {
            for max_work in (0..=32).chain([96, 384, 1000, u64::MAX]) {
                let limits = Limits {
                    max_bytes: 0,
                    max_work,
                };
                let mut old = Workspace::new(limits);
                let mut new = Workspace::new(limits);
                assert_eq!(
                    bounds_bits(compute_cluster_bounds(topology, positions, &mut old)),
                    bounds_bits(compute_cluster_bounds_with_positions(
                        topology, cached, &mut new
                    ))
                );
                assert_eq!(old.usage(), new.usage());
            }
        }
    }
}

#[test]
fn cached_meshlet_bounds_preserve_bits_errors_and_usage() {
    let (base, _) = mesh();
    let triangles = [0, 1, 2].repeat(512);
    for scale in [1., 1.0e9, f32::MAX, f32::NAN, f32::INFINITY] {
        let points: Vec<_> = base.iter().map(|p| p.map(|v| v * scale)).collect();
        let positions = Positions::from_packed(&points);
        let cached = BoundsPositions::new(positions);
        for vertices in [
            &[0, 1, 9][..],
            &[0, 1, u32::MAX][..],
            &[][..],
            &[0; 257][..],
        ] {
            for triangles in [
                &[0, 1, 2][..],
                &[0][..],
                &[3, 0, 0][..],
                &[][..],
                &triangles[..96],
                &triangles[..384],
                &triangles[..],
                &[0; 1539][..],
            ] {
                for max_work in (0..=32).chain([96, 384, 1000, u64::MAX]) {
                    let limits = Limits {
                        max_bytes: 0,
                        max_work,
                    };
                    let mut old = Workspace::new(limits);
                    let mut new = Workspace::new(limits);
                    assert_eq!(
                        bounds_bits(compute_meshlet_bounds(
                            vertices, triangles, positions, &mut old
                        )),
                        bounds_bits(compute_meshlet_bounds_with_positions(
                            vertices, triangles, cached, &mut new
                        ))
                    );
                    assert_eq!(old.usage(), new.usage());
                }
            }
        }
    }
}

#[test]
fn bounds_positions_constructor_defers_failed_scans_for_error_precedence() {
    let data = [
        17.,
        0.,
        0.,
        0.,
        17.,
        1.,
        0.,
        0.,
        17.,
        0.,
        1.,
        0.,
        17.,
        f32::NAN,
        0.,
        0.,
    ];
    let p = Positions::from_interleaved(&data, 4, 4, 1).unwrap();
    let cached = BoundsPositions::new(p);
    let mut w = Workspace::default();
    assert_eq!(
        compute_cluster_bounds_with_positions(&[0], cached, &mut w),
        Err(Error::InvalidTopology)
    );
    assert_eq!(
        compute_cluster_bounds_with_positions(&[0, 1, 2], cached, &mut w),
        Err(Error::NumericalFailure)
    );
    assert_eq!(
        compute_meshlet_bounds_with_positions(&[0, 1, 2], &[3, 0, 0], cached, &mut w),
        Err(Error::IndexOutOfBounds)
    );
    assert_eq!(
        compute_meshlet_bounds_with_positions(&[0, 1, 2], &[0, 1, 2], cached, &mut w),
        Err(Error::NumericalFailure)
    );
}

#[test]
fn cached_meshlet_bounds_layouts_and_stack_buckets_match_checked_path() {
    let points: Vec<[f32; 3]> = (0..65)
        .map(|i| [(i % 7) as f32, (i / 7) as f32, (i % 3) as f32 * 0.25])
        .collect();
    let interleaved: Vec<_> = points
        .iter()
        .flat_map(|p| [99., p[0], p[1], p[2]])
        .collect();
    let bytes: Vec<_> = points
        .iter()
        .flatten()
        .flat_map(|v| v.to_be_bytes())
        .collect();
    let views = [
        Positions::from_packed(&points),
        Positions::from_interleaved(&interleaved, points.len(), 4, 1).unwrap(),
        Positions::from_bytes(&bytes, points.len(), 12, 0, ByteOrder::BigEndian).unwrap(),
    ];
    let mut vertices: Vec<u32> = (0..64).collect();
    for invalid_unused_vertex in [false, true] {
        vertices[63] = if invalid_unused_vertex { u32::MAX } else { 63 };
        for count in [0, 3, 6, 96, 99, 384, 387, 1536] {
            let triangles: Vec<u8> = (0..count).map(|i| (i % 63) as u8).collect();
            for max_work in [0, 65, 65 + 64 + count as u64, u64::MAX] {
                let limits = Limits {
                    max_bytes: 0,
                    max_work,
                };
                let mut reference = Workspace::new(limits);
                let expected = bounds_bits(compute_meshlet_bounds(
                    &vertices,
                    &triangles,
                    views[0],
                    &mut reference,
                ));
                for p in views {
                    let mut w = Workspace::new(limits);
                    assert_eq!(
                        bounds_bits(compute_meshlet_bounds_with_positions(
                            &vertices,
                            &triangles,
                            BoundsPositions::new(p),
                            &mut w,
                        )),
                        expected
                    );
                    assert_eq!(w.usage(), reference.usage());
                }
            }
        }
    }
}

#[test]
fn packed_and_interleaved_uncached_bounds_preserve_bits_and_usage() {
    let indices = [0, 1, 2, 2, 1, 3].repeat(17);
    let vertices = [0, 1, 2, 3];
    let triangles: Vec<u8> = indices.iter().map(|&v| v as u8).collect();
    for scale in [1., 1e-20, 1e9, f32::MAX, f32::NAN] {
        let points = [[-0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.25]]
            .map(|p| p.map(|v| v * scale));
        let data: Vec<_> = points
            .iter()
            .flat_map(|p| [99., p[0], p[1], p[2]])
            .collect();
        let packed = Positions::from_packed(&points);
        let interleaved = Positions::from_interleaved(&data, points.len(), 4, 1).unwrap();
        for local in [false, true] {
            let call = |p, w: &mut Workspace| {
                if local {
                    compute_meshlet_bounds(&vertices, &triangles, p, w)
                } else {
                    compute_cluster_bounds(&indices, p, w)
                }
            };
            let mut unlimited = Workspace::default();
            let _ = call(packed, &mut unlimited);
            for max_work in 0..=unlimited.usage().work + 1 {
                let limits = Limits {
                    max_bytes: 0,
                    max_work,
                };
                let mut a = Workspace::new(limits);
                let mut b = Workspace::new(limits);
                assert_eq!(
                    bounds_bits(call(packed, &mut a)),
                    bounds_bits(call(interleaved, &mut b))
                );
                assert_eq!(a.usage(), b.usage());
            }
        }
    }
}
