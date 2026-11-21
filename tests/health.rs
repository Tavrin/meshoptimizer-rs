#![cfg(feature = "clusterlod")]
//! The structural health report agrees between a built DAG and its written
//! MDAG 2 container.
use meshoptimizer_rs::clusterlod::{
    self as cl,
    dag_format::v2 as wire,
    page::QuantGrid,
    v2::{self, BuildMesh, GroupStatus},
};
use meshoptimizer_rs::Workspace;

fn sphere(rows: u32, cols: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = Vec::new();
    for r in 0..=rows {
        let t = r as f32 * core::f32::consts::PI / rows as f32;
        for c in 0..cols {
            let a = c as f32 * core::f32::consts::TAU / cols as f32;
            p.push([
                10. * t.sin() * a.cos(),
                10. * t.cos(),
                10. * t.sin() * a.sin(),
            ]);
        }
    }
    let mut idx = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let (a, b) = (r * cols + c, r * cols + (c + 1) % cols);
            idx.extend([a, a + cols, b, b, a + cols, b + cols]);
        }
    }
    (p, idx)
}

fn planar(side: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let positions = (0..side)
        .flat_map(|y| (0..side).map(move |x| [x as f32, y as f32, 0.]))
        .collect();
    let indices = (0..side - 1)
        .flat_map(|y| {
            (0..side - 1).flat_map(move |x| {
                let a = y * side + x;
                [a, a + 1, a + side, a + 1, a + side + 1, a + side]
            })
        })
        .collect();
    (positions, indices)
}

fn check_reader(built: &v2::Built, grid: QuantGrid, check: impl FnOnce(&wire::Reader<'_>, &[u32])) {
    let source = wire::Source::new(
        &built.groups,
        &built.clusters,
        &built.positions,
        &built.indices,
    );
    let (file, _) = wire::write(
        &source,
        &wire::WriteOptions::new(grid),
        &mut Workspace::default(),
    )
    .unwrap();
    let reader = wire::Reader::read_all(&file, &mut Workspace::default()).unwrap();
    let mesh = reader.mesh();
    let mut triangles = vec![0u32; mesh.cluster_count as usize];
    let mut words = vec![0u32; reader.page_capacity() / 4];
    for p in 0..mesh.page_count as usize {
        let page = reader.page(p).unwrap();
        let range = page.disk_range();
        let view = reader
            .transcode_page(
                p,
                &file[range.start as usize..range.end as usize],
                &mut words,
                &mut Workspace::default(),
            )
            .unwrap();
        for k in 0..view.cluster_count() {
            triangles[page.first_cluster as usize + k] = view.cluster(k).unwrap().triangle_count;
        }
    }
    check(&reader, &triangles);
}

#[test]
fn built_and_container_reports_agree() {
    let grid = QuantGrid::new(-10).unwrap();
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 4;
    for (positions, indices) in [sphere(24, 40), planar(24)] {
        let built = v2::build(
            config,
            BuildMesh::new(&indices, &positions),
            grid,
            &mut Workspace::default(),
        )
        .unwrap();
        let from_built = v2::health(&built, 3, &mut Workspace::default()).unwrap();
        assert!(from_built.passed, "{from_built:?}");
        assert!(from_built.depths.len() >= 3);
        assert_eq!(from_built.depths[0].triangles as usize, indices.len() / 3);
        assert_eq!(from_built.pinned_groups[0].reason, GroupStatus::Root);
        check_reader(&built, grid, |reader, triangles| {
            let from_reader =
                v2::health_from_reader(reader, triangles, 3, &mut Workspace::default()).unwrap();
            let shape = |r: &v2::HealthReport| {
                r.depths
                    .iter()
                    .map(|d| (d.depth, d.groups, d.triangles, d.pinned))
                    .collect::<Vec<_>>()
            };
            assert_eq!(shape(&from_reader), shape(&from_built));
            assert!(from_reader.passed, "{from_reader:?}");
            assert_eq!(from_reader.pinned_groups[0].reason, GroupStatus::Root);
            assert!(matches!(
                v2::health_from_reader(reader, &triangles[1..], 3, &mut Workspace::default()),
                Err(meshoptimizer_rs::Error::InvalidParameter)
            ));
        });
    }
}

#[test]
fn measured_flags_follow_sampled_terms() {
    let grid = QuantGrid::new(-10).unwrap();
    let (positions, indices) = sphere(24, 40);
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 4;
    let build = |config| {
        v2::build_with_diagnostics(
            config,
            BuildMesh::new(&indices, &positions),
            grid,
            &mut Workspace::default(),
        )
        .unwrap()
    };
    let (built, diagnostics) = build(config);
    let finite = || {
        built
            .groups
            .iter()
            .zip(&diagnostics)
            .filter(|(g, _)| g.error != f32::MAX)
    };
    assert!(finite().any(|(_, d)| d.error_measured));
    for (g, d) in finite().filter(|(_, d)| d.error_measured) {
        assert!([d.step_error, d.source_error, d.inherited_error].contains(&g.error));
    }
    // A configured additive margin raises leaf errors above every sample.
    config.simplify_error_merge_additive = 1.0;
    let (built, diagnostics) = build(config);
    assert!(built
        .groups
        .iter()
        .zip(&diagnostics)
        .all(|(g, d)| g.depth != 0 || g.error == f32::MAX || !d.error_measured));
}

#[test]
fn all_stuck_at_the_final_depth_has_no_root() {
    use meshoptimizer_rs::VertexFlags;

    let (positions, indices) = planar(12);
    let grid = QuantGrid::new(-10).unwrap();
    let mut mesh = BuildMesh::new(&indices, &positions);
    let locks = vec![VertexFlags::LOCK; positions.len()];
    mesh.vertex_lock = Some(&locks);
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 512; // One terminal group, but more than one cluster.
    config.simplify_fallback_sloppy = false;
    let (built, diagnostics) =
        v2::build_with_diagnostics(config, mesh, grid, &mut Workspace::default()).unwrap();
    let from_built =
        v2::health_with_diagnostics(&built, &diagnostics, 3, &mut Workspace::default()).unwrap();
    assert!(!from_built.passed);
    assert_eq!(from_built.pinned_groups.len(), 1);
    assert!(from_built
        .pinned_groups
        .iter()
        .all(|g| g.reason == GroupStatus::RetainedRatio));
    assert_eq!(from_built.depths.len(), 1);
    assert!(
        !v2::health(&built, 3, &mut Workspace::default())
            .unwrap()
            .passed
    );
    check_reader(&built, grid, |reader, triangles| {
        let report =
            v2::health_from_reader(reader, triangles, 3, &mut Workspace::default()).unwrap();
        assert!(!report.passed);
        assert_eq!(report.pinned_groups.len(), from_built.pinned_groups.len());
        assert!(report
            .pinned_groups
            .iter()
            .all(|g| g.reason == GroupStatus::UnknownPinned));
    });
}

#[test]
fn health_rejects_chain_lengths_outside_format_limits() {
    use meshoptimizer_rs::Error;

    let grid = QuantGrid::new(-10).unwrap();
    let (positions, indices) = planar(2);
    let built = v2::build(
        cl::default_config(32).unwrap(),
        BuildMesh::new(&indices, &positions),
        grid,
        &mut Workspace::default(),
    )
    .unwrap();
    check_reader(&built, grid, |reader, triangles| {
        for levels in [0, 1, wire::MAX_LEVELS + 1, u32::MAX] {
            assert!(matches!(
                v2::health(&built, levels, &mut Workspace::default()),
                Err(Error::InvalidParameter)
            ));
            assert!(matches!(
                v2::health_from_reader(reader, triangles, levels, &mut Workspace::default()),
                Err(Error::InvalidParameter)
            ));
        }
        for levels in [2, wire::MAX_LEVELS] {
            assert!(
                v2::health(&built, levels, &mut Workspace::default())
                    .unwrap()
                    .passed
            );
            assert!(
                v2::health_from_reader(reader, triangles, levels, &mut Workspace::default())
                    .unwrap()
                    .passed
            );
        }
    });
}

#[test]
#[allow(clippy::field_reassign_with_default)] // the flat records are non-exhaustive
fn reducing_chains_use_all_distinct_child_groups() {
    let grid = QuantGrid::new(-10).unwrap();
    let mut built = v2::Built {
        positions: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vertex_source: Vec::new(),
        vertex_material: Vec::new(),
        attributes: Vec::new(),
        attribute_stride: 0,
        indices: Vec::new(),
        clusters: Vec::new(),
        groups: Vec::new(),
        stuck_groups: 0,
        alpha_coverage_stuck_groups: 0,
        alpha_coverage_raised_groups: 0,
        material_stuck_groups: 0,
    };
    let mut group = |depth, error, clusters: &[(i32, u32)]| {
        let id = built.groups.len() as u32;
        let mut record = cl::FlatDagGroup::default();
        record.depth = depth;
        record.error = error;
        record.radius = 1.0;
        record.first_cluster = built.clusters.len() as u32;
        record.cluster_count = clusters.len() as u32;
        built.groups.push(record);
        for &(refined, triangles) in clusters {
            let mut cluster = cl::FlatDagCluster::default();
            cluster.group = id;
            cluster.refined = refined;
            cluster.radius = 1.0;
            cluster.first_index = built.indices.len() as u32;
            cluster.index_count = triangles * 3;
            built.clusters.push(cluster);
            for _ in 0..triangles {
                built.indices.extend([0, 1, 2]);
            }
        }
    };
    // Groups 0, 5, 9, 12 trace the 32/32/32/4 branch. Each
    // 32-triangle parent also has a sibling input, so the links shrink
    // even though no individual input is larger.
    // Groups 7 and 8 enter a two-wide plateau mid-DAG. Repeated links
    // to a single child in groups 11 and 14 only count once.
    group(0, 1.0, &[(-1, 32)]);
    group(0, 0.5, &[(-1, 32)]);
    group(0, 0.5, &[(-1, 32)]);
    group(0, 0.25, &[(-1, 32)]);
    group(0, 0.25, &[(-1, 32)]);
    group(1, 1.0, &[(0, 16), (1, 16)]);
    group(1, 0.5, &[(2, 16)]);
    group(1, 2.0, &[(3, 16)]);
    group(1, 2.0, &[(4, 16)]);
    group(2, 1.0, &[(5, 24), (6, 8)]);
    group(2, 2.0, &[(7, 8), (8, 8)]);
    group(2, 2.0, &[(7, 8), (7, 8)]);
    group(3, f32::MAX, &[(9, 4)]);
    group(3, 2.0, &[(10, 8), (11, 8)]);
    group(3, 2.0, &[(11, 8), (11, 8)]);
    group(4, 3.0, &[(13, 4), (14, 4)]);
    group(5, f32::MAX, &[(15, 4)]);
    let report = v2::health(&built, 3, &mut Workspace::default()).unwrap();
    assert_eq!(
        report
            .equal_error_chains
            .iter()
            .map(|c| c.group)
            .collect::<Vec<_>>(),
        [9, 13]
    );
    check_reader(&built, grid, |reader, triangles| {
        let report =
            v2::health_from_reader(reader, triangles, 3, &mut Workspace::default()).unwrap();
        assert!(!report.passed);
        let mut depths: Vec<_> = report
            .equal_error_chains
            .iter()
            .map(|chain| {
                assert_eq!(chain.levels, 3);
                reader.group(chain.group as usize).unwrap().depth
            })
            .collect();
        depths.sort_unstable();
        assert_eq!(depths, [2, 3]);
    });
}

#[test]
fn progressing_partial_steps_pin_at_the_format_depth_limit() {
    use meshoptimizer_rs::VertexFlags;

    // Every isolated quad has one collapsible boundary corner. The target
    // permits one collapse per level, and the other three corners are locked.
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut locks = Vec::new();
    for i in 0..70 {
        let x = (i * 3) as f32;
        positions.extend([[x, 0., 0.], [x + 1., 0., 0.], [x, 1., 0.], [x + 1., 1., 0.]]);
        let a = i * 4;
        indices.extend([a, a + 1, a + 2, a + 1, a + 3, a + 2]);
        locks.extend([
            VertexFlags::LOCK,
            VertexFlags::LOCK,
            VertexFlags::LOCK,
            VertexFlags::EMPTY,
        ]);
    }
    let mut config = cl::default_config(4).unwrap();
    config.partition_size = 512;
    config.simplify_ratio = 0.999;
    config.simplify_threshold = 0.5;
    let mut mesh = BuildMesh::new(&indices, &positions);
    mesh.vertex_lock = Some(&locks);
    let grid = QuantGrid::new(-10).unwrap();
    let (built, diagnostics) =
        v2::build_with_diagnostics(config, mesh, grid, &mut Workspace::default()).unwrap();
    let report =
        v2::health_with_diagnostics(&built, &diagnostics, 3, &mut Workspace::default()).unwrap();
    assert_eq!(report.depths.len(), wire::MAX_LEVELS as usize);
    for pair in report.depths.windows(2) {
        assert_eq!(pair[0].partial, 1);
        assert_eq!(pair[0].triangles, pair[1].triangles + 1);
    }
    assert_eq!(built.stuck_groups, 1);
    assert_eq!(report.pinned_groups[0].reason, GroupStatus::DepthLimit);
    check_reader(&built, grid, |reader, triangles| {
        assert_eq!(reader.mesh().level_count, wire::MAX_LEVELS);
        let report =
            v2::health_from_reader(reader, triangles, 3, &mut Workspace::default()).unwrap();
        assert!(!report.passed);
        assert_eq!(report.pinned_groups[0].reason, GroupStatus::UnknownPinned);
    });
}

#[test]
fn repeated_partial_reductions_do_not_pin_progressing_geometry() {
    let (positions, indices) = sphere(24, 40);
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 4;
    // Every half-sized replacement is partial under this stricter trigger.
    config.simplify_threshold = 0.1;
    let (built, diagnostics) = v2::build_with_diagnostics(
        config,
        BuildMesh::new(&indices, &positions),
        QuantGrid::new(-10).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(built.stuck_groups, 0);
    let partial_depths: std::collections::BTreeSet<_> = built
        .groups
        .iter()
        .zip(&diagnostics)
        .filter(|(_, d)| d.status == GroupStatus::PartialReduction)
        .map(|(g, _)| g.depth)
        .collect();
    assert!(partial_depths.len() >= 3);
    assert!(
        v2::health(&built, 3, &mut Workspace::default())
            .unwrap()
            .passed
    );
}

#[test]
fn measured_errors_are_independent_of_the_heuristic_edge_cap() {
    let (positions, indices) = sphere(24, 40);
    let normals: Vec<_> = positions.iter().map(|p| p.map(|v| v / 10.)).collect();
    let build = |cap| {
        let mut config = cl::default_config(32).unwrap();
        config.partition_size = 4;
        config.simplify_error_edge_limit = cap;
        let mut mesh = BuildMesh::new(&indices, &positions);
        mesh.normals = Some(&normals);
        v2::build(
            config,
            mesh,
            QuantGrid::new(-10).unwrap(),
            &mut Workspace::default(),
        )
        .unwrap()
    };
    let regular = build(0.);
    let capped = build(1e-6);
    assert_eq!(regular.indices, capped.indices);
    assert_eq!(regular.groups.len(), capped.groups.len());
    for (a, b) in regular.groups.iter().zip(&capped.groups) {
        assert_eq!(a.error.to_bits(), b.error.to_bits());
    }
}
