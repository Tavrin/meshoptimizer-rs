use meshoptimizer_rs::{
    analyze_coverage, analyze_overdraw, analyze_vertex_cache, analyze_vertex_fetch,
    opacity_map_compact, opacity_map_entry_size, opacity_map_measure, opacity_map_rasterize,
    stripify, unstripify, Positions, Workspace,
};

#[test]
fn pinned_strip_and_cache_vectors() {
    let source = [0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7];
    let mut workspace = Workspace::default();
    let normal = stripify(&source, 8, 0, &mut workspace).unwrap();
    assert_eq!(normal, [0, 1, 2, 3, 3, 4, 4, 5, 6, 7]);
    assert_eq!(unstripify(&normal, 0, &mut workspace).unwrap(), source);
    let restarted = stripify(&source, 8, 65535, &mut workspace).unwrap();
    assert_eq!(restarted, [0, 1, 2, 3, 65535, 4, 5, 6, 7]);
    assert_eq!(
        unstripify(&restarted, 65535, &mut workspace).unwrap(),
        source
    );
    for cache in [3, 16] {
        let stats = analyze_vertex_cache(&source, 8, cache, 32, 2, &mut workspace).unwrap();
        assert_eq!((stats.vertices_transformed, stats.warps_executed), (8, 2));
        assert_eq!(
            (stats.acmr.to_bits(), stats.atvr.to_bits()),
            (0x40000000, 0x3f800000)
        );
    }
    let fetch12 = analyze_vertex_fetch(&source, 8, 12, &mut workspace).unwrap();
    assert_eq!(
        (fetch12.bytes_fetched, fetch12.overfetch.to_bits()),
        (128, 0x3faaaaab)
    );
    let fetch64 = analyze_vertex_fetch(&source, 8, 64, &mut workspace).unwrap();
    assert_eq!(
        (fetch64.bytes_fetched, fetch64.overfetch.to_bits()),
        (512, 0x3f800000)
    );
}

#[test]
fn pinned_raster_vector() {
    let mut workspace = Workspace::default();
    let positions = Positions::from_packed(&[
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ]);
    let indices = [0, 1, 2, 2, 1, 3, 0, 1, 2];
    let overdraw = analyze_overdraw(&indices, positions, &mut workspace).unwrap();
    assert_eq!(
        (
            overdraw.pixels_covered,
            overdraw.pixels_shaded,
            overdraw.overdraw.to_bits()
        ),
        (65536, 98176, 0x3fbfc000)
    );
    let coverage = analyze_coverage(&indices, positions, &mut workspace).unwrap();
    assert_eq!(coverage.coverage.map(f32::to_bits), [0, 0, 0x3f800000]);
    assert_eq!(coverage.extent.to_bits(), 0x3f800000);
}

#[test]
fn pinned_opacity_vectors() {
    let mut workspace = Workspace::default();
    let uvs = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let measured = opacity_map_measure(
        &[0, 1, 2, 0, 1, 2],
        &uvs,
        3,
        2,
        32,
        32,
        2,
        0.0,
        &mut workspace,
    )
    .unwrap();
    assert_eq!(measured.levels, [2]);
    assert_eq!(measured.sources, [0]);
    assert_eq!(measured.omm_indices, [0, 0]);
    for level in 0..=2 {
        for (states, value, special) in [
            (2, 0, -1),
            (2, 255, -2),
            (4, 0, -1),
            (4, 0x55, -2),
            (4, 0xaa, -3),
            (4, 0xff, -4),
        ] {
            let mut data = vec![value; opacity_map_entry_size(level, states).unwrap()];
            let mut levels = [level];
            let mut offsets = [0];
            let mut indices = [0];
            assert_eq!(
                opacity_map_compact(
                    &mut data,
                    &mut levels,
                    &mut offsets,
                    &mut indices,
                    states,
                    &mut workspace
                )
                .unwrap()
                .0,
                0
            );
            assert_eq!(indices, [special]);
        }
    }
    let mut overlapping = [0x11u8; 1];
    let mut levels = [1u8; 2];
    let mut offsets = [0u32; 2];
    let mut indices = [0i32, 1];
    assert_eq!(
        opacity_map_compact(
            &mut overlapping,
            &mut levels,
            &mut offsets,
            &mut indices,
            4,
            &mut workspace
        ),
        Err(meshoptimizer_rs::Error::BufferTooSmall)
    );
    let mut texture = [0u8; 32 * 32];
    for y in 0..32 {
        for x in 0..32 {
            let dx = x as f32 + 0.5 - 16.0;
            let dy = y as f32 + 0.5 - 16.0;
            let value = 10.0 - libm::sqrtf(dx * dx + dy * dy) + 0.5;
            texture[y * 32 + x] = (value.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        }
    }
    for (corners, expected) in [
        ([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], 2),
        ([[0.0, 0.0], [0.5, 0.0], [0.0, 0.5]], 0),
        ([[0.5, 0.5], [0.5, 0.0], [0.0, 0.5]], 3),
    ] {
        assert_eq!(
            opacity_map_rasterize(0, 4, corners, &texture, 1, 32, 32, 32, &mut workspace).unwrap(),
            [expected]
        );
    }
}

#[cfg(feature = "experimental")]
#[test]
fn upstream_normals_basic() {
    use meshoptimizer_rs::generate_normals;
    let vertices = [
        [-1.0, -0.57735, 0.0],
        [1.0, -0.57735, 0.0],
        [0.0, 1.15470, 0.0],
        [0.0, 0.0, 0.38],
    ];
    let indices = [0, 2, 1, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let mut workspace = Workspace::default();
    let result = generate_normals(
        Some(&indices),
        indices.len(),
        Positions::from_packed(&vertices),
        core::f32::consts::PI / 3.0,
        0.0,
        &mut workspace,
    )
    .unwrap();
    assert_eq!(
        result[3].map(f32::to_bits),
        [0xbe8a9b79, 0xbe200cc9, 0x3f732bb5]
    );
    assert_eq!(result[7].map(f32::to_bits), [0, 0x3ea00ccd, 0x3f732bb4]);
    let expected = [
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [-0.2707, -0.1563, 0.9499],
        [0.2707, -0.1563, 0.9499],
        [0.0, 0.0, 1.0],
        [0.2707, -0.1563, 0.9499],
        [0.0, 0.3126, 0.9499],
        [0.0, 0.0, 1.0],
        [0.0, 0.3126, 0.9499],
        [-0.2707, -0.1563, 0.9499],
        [0.0, 0.0, 1.0],
    ];
    for (a, b) in result.iter().zip(expected) {
        for i in 0..3 {
            assert!((a[i] - b[i]).abs() < 1e-3, "{a:?} != {b:?}");
        }
    }
}

#[test]
fn upstream_tangents_basic() {
    use meshoptimizer_rs::generate_tangents;
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    let normals = [
        [-0.28, 0.0, 0.96],
        [0.28, 0.0, 0.96],
        [0.28, 0.0, 0.96],
        [-0.28, 0.0, 0.96],
        [0.28, 0.0, 0.96],
        [-0.28, 0.0, 0.96],
    ];
    let uvs = [
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ];
    let mut workspace = Workspace::default();
    let result = generate_tangents(
        None,
        6,
        Positions::from_packed(&positions),
        &normals,
        &uvs,
        0,
        &mut workspace,
    )
    .unwrap();
    assert_eq!(
        result[0].map(f32::to_bits),
        [0x3f75c28f, 0, 0x3e8f5c29, 0x3f800000]
    );
    assert_eq!(
        result[1].map(f32::to_bits),
        [0x3f75c28f, 0, 0xbe8f5c29, 0x3f800000]
    );
    let expected = [
        [0.96, 0.0, 0.28, 1.0],
        [0.96, 0.0, -0.28, 1.0],
        [0.96, 0.0, -0.28, 1.0],
        [0.96, 0.0, 0.28, 1.0],
        [0.96, 0.0, -0.28, 1.0],
        [0.96, 0.0, 0.28, 1.0],
    ];
    for (a, b) in result.iter().zip(expected) {
        for i in 0..4 {
            assert!((a[i] - b[i]).abs() < 1e-3, "{a:?} != {b:?}");
        }
    }
    assert_eq!(result[0], result[3]);
    assert_eq!(result[2], result[4]);
}

#[cfg(feature = "experimental")]
#[test]
fn pinned_remesh_tetrahedron() {
    use meshoptimizer_rs::{remesh, remesh_bound};
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let indices = [0, 2, 1, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let mut workspace = Workspace::default();
    let cases = [
        (
            4,
            [
                (4, 0xfdbc596d3f544551),
                (4, 0xfdbc596d3f544551),
                (4, 0x929fab9a5531c4b0),
                (4, 0x929fab9a5531c4b0),
            ],
        ),
        (
            8,
            [
                (106, 0x2f1c71d29a37d766),
                (114, 0xf32d6fa172b468de),
                (106, 0x75bdb49bfc3faf3f),
                (114, 0xbef9b033595b159f),
            ],
        ),
        (
            16,
            [
                (682, 0x172d94a5e284490c),
                (1058, 0x4e1ea23631ca4bf4),
                (682, 0xe31f2a1e45a8bb70),
                (1058, 0x26d5b15f388e1c88),
            ],
        ),
    ];
    for (resolution, expected) in cases {
        for (options, (expected_count, expected_hash)) in expected.into_iter().enumerate() {
            let bound = remesh_bound(
                &indices,
                Positions::from_packed(&positions),
                resolution,
                options as u32,
                &mut workspace,
            )
            .unwrap();
            let output = remesh(
                &indices,
                Positions::from_packed(&positions),
                resolution,
                options as u32,
                &mut workspace,
            )
            .unwrap();
            let mut hash = 14695981039346656037u64;
            for component in output.iter().flatten() {
                for byte in component.to_bits().to_le_bytes() {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(1099511628211);
                }
            }
            assert_eq!(bound, expected_count);
            assert_eq!(output.len() / 3, expected_count);
            assert_eq!(hash, expected_hash);
        }
    }
}
