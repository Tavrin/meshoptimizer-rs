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
