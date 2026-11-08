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

#[test]
fn built_and_container_reports_agree() {
    let (positions, indices) = sphere(24, 40);
    let grid = QuantGrid::new(-10).unwrap();
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 4;
    let built = v2::build(
        config,
        BuildMesh::new(&indices, &positions),
        grid,
        &mut Workspace::default(),
    )
    .unwrap();
    let from_built = v2::health(&built, 3, &mut Workspace::default()).unwrap();
    assert!(from_built.depths.len() >= 3);
    assert_eq!(from_built.depths[0].triangles as usize, indices.len() / 3);
    assert_eq!(built.diagnostics.len(), built.groups.len());
    assert!(from_built
        .pinned_groups
        .iter()
        .any(|p| p.reason == GroupStatus::Root));

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
    let from_reader =
        v2::health_from_reader(&reader, &triangles, 3, &mut Workspace::default()).unwrap();
    let shape = |r: &v2::HealthReport| {
        r.depths
            .iter()
            .map(|d| (d.depth, d.groups, d.triangles, d.pinned))
            .collect::<Vec<_>>()
    };
    assert_eq!(shape(&from_reader), shape(&from_built));
    assert_eq!(
        from_reader.equal_error_chains.len(),
        from_built.equal_error_chains.len()
    );
    assert_eq!(
        from_reader.pinned_groups.len(),
        from_built.pinned_groups.len()
    );
    assert_eq!(from_reader.passed, from_built.passed);
    assert!(matches!(
        v2::health_from_reader(&reader, &triangles[1..], 3, &mut Workspace::default()),
        Err(meshoptimizer_rs::Error::InvalidParameter)
    ));
}

#[test]
fn repeated_partial_reductions_do_not_pin_progressing_geometry() {
    let (positions, indices) = sphere(24, 40);
    let mut config = cl::default_config(32).unwrap();
    config.partition_size = 4;
    // Every half-sized replacement is partial under this stricter trigger.
    config.simplify_threshold = 0.1;
    let built = v2::build(
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
        .zip(&built.diagnostics)
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
