use meshoptimizer_rs::{
    analyze_coverage, analyze_overdraw, analyze_vertex_cache, analyze_vertex_fetch,
    opacity_map_compact, opacity_map_entry_size, opacity_map_measure, opacity_map_rasterize,
    opacity_map_rasterize_into, stripify, unstripify, Error, Limits, Positions, Workspace,
};

fn exact_work_boundary(mut call: impl FnMut(&mut Workspace) -> Result<(), Error>) {
    let mut workspace = Workspace::default();
    call(&mut workspace).unwrap();
    let used = workspace.usage();
    assert!(used.work > 0);
    workspace.set_limits(Limits {
        max_bytes: used.bytes,
        max_work: used.work,
    });
    call(&mut workspace).unwrap();
    workspace.set_limits(Limits {
        max_bytes: used.bytes,
        max_work: used.work - 1,
    });
    assert_eq!(call(&mut workspace), Err(Error::LimitExceeded));
}

#[test]
fn batched_strip_and_fetch_work_respects_exact_boundary() {
    let indices = [0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7];
    exact_work_boundary(|workspace| stripify(&indices, 8, 0, workspace).map(|_| ()));
    let strip = [0, 1, 2, 3, 3, 4, 4, 5, 6, 7];
    exact_work_boundary(|workspace| unstripify(&strip, 0, workspace).map(|_| ()));
    for vertex_size in [1, 12, 64, 127, 128, 255, 256] {
        exact_work_boundary(|workspace| {
            analyze_vertex_fetch(&indices, 8, vertex_size, workspace).map(|_| ())
        });
    }
    for warp in [0, 16] {
        for group in [0, 4] {
            exact_work_boundary(|workspace| {
                analyze_vertex_cache(&indices, 8, 16, warp, group, workspace).map(|_| ())
            });
        }
    }
}

#[cfg(feature = "experimental")]
#[test]
fn batched_normal_tangent_and_remesh_work_respects_exact_boundary() {
    use meshoptimizer_rs::{generate_normals, generate_tangents, remesh};

    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let indices = [0, 2, 1, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let normals = [[0.0, 0.0, 1.0]; 4];
    let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    exact_work_boundary(|workspace| {
        generate_tangents(
            Some(&indices),
            indices.len(),
            Positions::from_packed(&positions),
            &normals,
            &uvs,
            0,
            workspace,
        )
        .map(|_| ())
    });
    exact_work_boundary(|workspace| {
        generate_normals(
            Some(&indices),
            indices.len(),
            Positions::from_packed(&positions),
            1.0,
            0.5,
            workspace,
        )
        .map(|_| ())
    });
    exact_work_boundary(|workspace| {
        remesh(
            &indices,
            Positions::from_packed(&positions),
            4,
            0,
            workspace,
        )
        .map(|_| ())
    });
}

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

    // Layout dispatch must preserve exact raster values, ignored padding and
    // charge/error prefixes, including the newly direct packed transform.
    use meshoptimizer_rs::ByteOrder;
    let mut strided = vec![f32::NAN];
    let mut little = vec![0xcd];
    let mut big = vec![0xcd];
    for i in 0..positions.len() {
        let p = positions.get(i).unwrap();
        strided.extend(p);
        strided.push(f32::NAN);
        for value in p.into_iter().chain([f32::NAN]) {
            little.extend(value.to_bits().to_le_bytes());
            big.extend(value.to_bits().to_be_bytes());
        }
    }
    let views = [
        positions,
        Positions::from_interleaved(&strided, 4, 4, 1).unwrap(),
        Positions::from_bytes(&little, 4, 16, 1, ByteOrder::LittleEndian).unwrap(),
        Positions::from_bytes(&big, 4, 16, 1, ByteOrder::BigEndian).unwrap(),
    ];
    for view in views {
        let mut ws = Workspace::default();
        let actual = analyze_overdraw(&indices, view, &mut ws).unwrap();
        let usage = ws.usage();
        assert_eq!(
            (
                actual.pixels_covered,
                actual.pixels_shaded,
                actual.overdraw.to_bits()
            ),
            (
                overdraw.pixels_covered,
                overdraw.pixels_shaded,
                overdraw.overdraw.to_bits()
            )
        );
        let actual = analyze_coverage(&indices, view, &mut ws).unwrap();
        assert_eq!(
            actual.coverage.map(f32::to_bits),
            coverage.coverage.map(f32::to_bits)
        );
        assert_eq!(actual.extent.to_bits(), coverage.extent.to_bits());
        for fuel in [0, 1, 8, 9, 12, 13, usage.work - 1, usage.work] {
            let limits = Limits {
                max_bytes: usage.bytes,
                max_work: fuel,
            };
            let mut reference = Workspace::new(limits);
            let mut actual = Workspace::new(limits);
            assert_eq!(
                analyze_overdraw(&indices, view, &mut actual),
                analyze_overdraw(&indices, positions, &mut reference)
            );
            assert_eq!(actual.usage(), reference.usage());
            assert_eq!(
                analyze_coverage(&indices, view, &mut actual),
                analyze_coverage(&indices, positions, &mut reference)
            );
            assert_eq!(actual.usage(), reference.usage());
        }
    }
    let invalid = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [f32::NAN; 3],
    ];
    let packed = Positions::from_packed(&invalid);
    let flat: Vec<_> = invalid.iter().flatten().copied().collect();
    let view = Positions::from_interleaved(&flat, 5, 3, 0).unwrap();
    for positions in [packed, view] {
        for fuel in [13, 14] {
            let mut ws = Workspace::new(Limits {
                max_bytes: 1 << 21,
                max_work: fuel,
            });
            let expected = if fuel == 13 {
                Error::LimitExceeded
            } else {
                Error::NumericalFailure
            };
            assert_eq!(
                analyze_overdraw(&indices, positions, &mut ws),
                Err(expected)
            );
            assert_eq!(ws.usage().work, fuel);
        }
    }
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

#[test]
fn raster_into_uses_caller_storage_and_preserves_atomic_failure() {
    let uvs = [[0.1, 0.1], [0.8, 0.2], [0.2, 0.9]];
    let texture = core::array::from_fn::<_, 64, _>(|i| ((i * 37) & 255) as u8);
    let mut workspace = Workspace::default();
    for level in 0..=3 {
        for states in [2, 4] {
            workspace.set_limits(Limits::default());
            let expected =
                opacity_map_rasterize(level, states, uvs, &texture, 1, 8, 8, 8, &mut workspace)
                    .unwrap();
            let mut destination = vec![0xa5; expected.len() + 2];
            workspace.set_limits(Limits {
                max_bytes: 0,
                max_work: u64::MAX,
            });
            assert_eq!(
                opacity_map_rasterize_into(
                    &mut destination,
                    level,
                    states,
                    uvs,
                    &texture,
                    1,
                    8,
                    8,
                    8,
                    &mut workspace,
                ),
                Ok(expected.len())
            );
            assert_eq!(&destination[..expected.len()], expected);
            assert_eq!(&destination[expected.len()..], &[0xa5, 0xa5]);
            assert_eq!(workspace.usage().bytes, 0);
            let used = workspace.usage().work;
            destination.fill(0xa5);
            workspace.set_limits(Limits {
                max_bytes: expected.len(),
                max_work: used - 1,
            });
            assert_eq!(
                opacity_map_rasterize_into(
                    &mut destination,
                    level,
                    states,
                    uvs,
                    &texture,
                    1,
                    8,
                    8,
                    8,
                    &mut workspace,
                ),
                Err(Error::LimitExceeded)
            );
            assert!(destination.iter().all(|&value| value == 0xa5));
            assert_eq!(workspace.usage().work, used - 1);
        }
    }
    let mut destination = [0xa5; 4];
    workspace.set_limits(Limits {
        max_bytes: 0,
        max_work: u64::MAX,
    });
    assert_eq!(
        opacity_map_rasterize_into(
            &mut destination,
            0,
            4,
            [[f32::NAN, 0.0], uvs[1], uvs[2]],
            &texture,
            1,
            8,
            8,
            8,
            &mut workspace,
        ),
        Err(Error::InvalidParameter)
    );
    assert_eq!(destination, [0xa5; 4]);
    assert_eq!(workspace.usage().work, 0);
}

#[cfg(feature = "experimental")]
#[test]
fn normal_tangent_into_uses_caller_storage_and_preserves_failures() {
    use meshoptimizer_rs::{
        generate_normals, generate_normals_into, generate_tangents, generate_tangents_into,
        ByteOrder,
    };
    let positions: [[f32; 3]; 4] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let indices = [0, 2, 1, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let normals = [[0.0, 0.0, 1.0]; 4];
    let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let interleaved: Vec<f32> = positions
        .iter()
        .flat_map(|p| [f32::NAN, p[0], p[1], p[2]])
        .collect();
    let little: Vec<u8> = positions
        .iter()
        .flatten()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let big: Vec<u8> = positions
        .iter()
        .flatten()
        .flat_map(|v| v.to_be_bytes())
        .collect();
    for view in [
        Positions::from_packed(&positions),
        Positions::from_interleaved(&interleaved, 4, 4, 1).unwrap(),
        Positions::from_bytes(&little, 4, 12, 0, ByteOrder::LittleEndian).unwrap(),
        Positions::from_bytes(&big, 4, 12, 0, ByteOrder::BigEndian).unwrap(),
    ] {
        for smoothing in [0.0, 1.5, 10.0] {
            let mut allocating = Workspace::default();
            let expected = generate_normals(
                Some(&indices),
                12,
                Positions::from_packed(&positions),
                1.0,
                smoothing,
                &mut allocating,
            )
            .unwrap();
            let usage = allocating.usage();
            let mut caller = Workspace::new(Limits {
                max_bytes: usage.bytes - 12 * 12,
                max_work: u64::MAX,
            });
            let mut destination = [[42.0; 3]; 14];
            generate_normals_into(
                &mut destination,
                Some(&indices),
                12,
                view,
                1.0,
                smoothing,
                &mut caller,
            )
            .unwrap();
            for (actual, expected) in destination[..12].iter().zip(&expected) {
                assert_eq!(actual.map(f32::to_bits), expected.map(f32::to_bits));
            }
            assert_eq!(destination[12..], [[42.0; 3]; 2]);
            assert_eq!(caller.usage().bytes, usage.bytes - 12 * 12);
            assert_eq!(caller.usage().work, usage.work);
            for fuel in 0..=usage.work {
                allocating.set_limits(Limits {
                    max_bytes: usize::MAX,
                    max_work: fuel,
                });
                caller.set_limits(Limits {
                    max_bytes: usize::MAX,
                    max_work: fuel,
                });
                destination.fill([42.0; 3]);
                let result = generate_normals(
                    Some(&indices),
                    12,
                    Positions::from_packed(&positions),
                    1.0,
                    smoothing,
                    &mut allocating,
                );
                let actual = generate_normals_into(
                    &mut destination,
                    Some(&indices),
                    12,
                    view,
                    1.0,
                    smoothing,
                    &mut caller,
                );
                assert_eq!(actual, result.as_ref().map(|_| ()).map_err(|e| *e));
                assert_eq!(caller.usage().work, allocating.usage().work);
                if result.is_err() {
                    assert_eq!(destination, [[42.0; 3]; 14]);
                }
            }
            caller.set_limits(Limits {
                max_bytes: 0,
                max_work: u64::MAX,
            });
            destination.fill([42.0; 3]);
            assert_eq!(
                generate_normals_into(
                    &mut destination,
                    Some(&indices),
                    12,
                    view,
                    1.0,
                    smoothing,
                    &mut caller
                ),
                Err(Error::LimitExceeded)
            );
            assert_eq!(destination, [[42.0; 3]; 14]);
        }
        for options in 0..=3 {
            let mut allocating = Workspace::default();
            let expected = generate_tangents(
                Some(&indices),
                12,
                view,
                &normals,
                &uvs,
                options,
                &mut allocating,
            )
            .unwrap();
            let usage = allocating.usage();
            let mut caller = Workspace::new(Limits {
                max_bytes: usage.bytes - 12 * 16,
                max_work: u64::MAX,
            });
            let mut destination = [[42.0; 4]; 14];
            generate_tangents_into(
                &mut destination,
                Some(&indices),
                12,
                view,
                &normals,
                &uvs,
                options,
                &mut caller,
            )
            .unwrap();
            for (actual, expected) in destination[..12].iter().zip(&expected) {
                assert_eq!(actual.map(f32::to_bits), expected.map(f32::to_bits));
            }
            assert_eq!(destination[12..], [[42.0; 4]; 2]);
            assert_eq!(caller.usage().bytes, usage.bytes - 12 * 16);
            assert_eq!(caller.usage().work, usage.work);
            for fuel in 0..=usage.work {
                allocating.set_limits(Limits {
                    max_bytes: usize::MAX,
                    max_work: fuel,
                });
                caller.set_limits(Limits {
                    max_bytes: usize::MAX,
                    max_work: fuel,
                });
                destination.fill([42.0; 4]);
                let result = generate_tangents(
                    Some(&indices),
                    12,
                    view,
                    &normals,
                    &uvs,
                    options,
                    &mut allocating,
                );
                let actual = generate_tangents_into(
                    &mut destination,
                    Some(&indices),
                    12,
                    view,
                    &normals,
                    &uvs,
                    options,
                    &mut caller,
                );
                assert_eq!(actual, result.as_ref().map(|_| ()).map_err(|e| *e));
                assert_eq!(caller.usage().work, allocating.usage().work);
                if result.is_err() {
                    assert_eq!(destination, [[42.0; 4]; 14]);
                }
            }
            caller.set_limits(Limits {
                max_bytes: 0,
                max_work: u64::MAX,
            });
            destination.fill([42.0; 4]);
            assert_eq!(
                generate_tangents_into(
                    &mut destination,
                    Some(&indices),
                    12,
                    view,
                    &normals,
                    &uvs,
                    options,
                    &mut caller
                ),
                Err(Error::LimitExceeded)
            );
            assert_eq!(destination, [[42.0; 4]; 14]);
        }
    }
}

#[cfg(feature = "experimental")]
#[test]
fn normal_tangent_peak_storage_excludes_retired_remap_table() {
    use meshoptimizer_rs::{generate_normals_into, generate_tangents_into};
    let positions = [[0.0; 3]; 8];
    let indices = [0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7];
    let view = Positions::from_packed(&positions);
    // Pinned C++ requested peaks for eight vertices and four faces: 228/248.
    // The 96-byte remap phase ends before the larger adjacency/group phase.
    let mut workspace = Workspace::new(Limits {
        max_bytes: 228,
        max_work: u64::MAX,
    });
    let mut normals = [[42.0; 3]; 14];
    generate_normals_into(
        &mut normals,
        Some(&indices),
        12,
        view,
        1.0,
        0.0,
        &mut workspace,
    )
    .unwrap();
    assert_eq!(workspace.usage().bytes, 228);
    assert_eq!(normals[12..], [[42.0; 3]; 2]);
    workspace.set_limits(Limits {
        max_bytes: 248,
        max_work: u64::MAX,
    });
    let mut tangents = [[42.0; 4]; 14];
    generate_tangents_into(
        &mut tangents,
        Some(&indices),
        12,
        view,
        &[[0.0, 0.0, 1.0]; 8],
        &[[0.0; 2]; 8],
        0,
        &mut workspace,
    )
    .unwrap();
    assert_eq!(workspace.usage().bytes, 248);
    assert_eq!(tangents[12..], [[42.0; 4]; 2]);
    workspace.set_limits(Limits {
        max_bytes: 247,
        max_work: u64::MAX,
    });
    tangents.fill([42.0; 4]);
    assert_eq!(
        generate_tangents_into(
            &mut tangents,
            Some(&indices),
            12,
            view,
            &[[0.0, 0.0, 1.0]; 8],
            &[[0.0; 2]; 8],
            0,
            &mut workspace
        ),
        Err(Error::LimitExceeded)
    );
    assert_eq!(tangents, [[42.0; 4]; 14]);
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

#[test]
fn opacity_bulk_probe_work_preserves_numerical_failure_prefix() {
    let indices = [0, 1, 2, 0, 1, 2, 0, 1, 3];
    let uvs = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, f32::NAN, 1.0];
    let mut workspace = Workspace::new(Limits {
        max_bytes: 1024,
        max_work: 24,
    });
    let call = |workspace: &mut Workspace| {
        opacity_map_measure(&indices, &uvs, 4, 2, 8, 8, 2, 0.0, workspace)
    };
    assert_eq!(call(&mut workspace), Err(Error::NumericalFailure));
    assert_eq!(workspace.usage().work, 14);
    workspace.set_limits(Limits {
        max_bytes: 1024,
        max_work: 14,
    });
    assert_eq!(call(&mut workspace), Err(Error::NumericalFailure));
    assert_eq!(workspace.usage().work, 14);
    workspace.set_limits(Limits {
        max_bytes: 1024,
        max_work: 13,
    });
    assert_eq!(call(&mut workspace), Err(Error::LimitExceeded));
    assert_eq!(workspace.usage().work, 13);
}

#[test]
fn strip_append_output_preserves_caller_prefixes_and_tails() {
    use meshoptimizer_rs::{stripify_bound, stripify_into, unstripify_bound, unstripify_into};
    let indices = [0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7];
    for restart in [0, 65535] {
        let mut workspace = Workspace::default();
        let expected = stripify(&indices, 8, restart, &mut workspace).unwrap();
        let work = workspace.usage().work;
        let bound = stripify_bound(indices.len()).unwrap();
        for fuel in 0..=work {
            let limits = Limits {
                max_bytes: usize::MAX,
                max_work: fuel,
            };
            let mut allocating = Workspace::new(limits);
            let result = stripify(&indices, 8, restart, &mut allocating);
            let mut caller = Workspace::new(limits);
            let mut destination = vec![u32::MAX; bound + 2];
            let actual = stripify_into(&mut destination, &indices, 8, restart, &mut caller);
            assert_eq!(actual, result.as_ref().map(Vec::len).map_err(|e| *e));
            assert_eq!(caller.usage().work, allocating.usage().work);
            let written = destination.iter().position(|&v| v == u32::MAX).unwrap();
            assert!(written <= expected.len());
            assert_eq!(destination[..written], expected[..written]);
            assert!(destination[written..].iter().all(|&v| v == u32::MAX));
        }
    }
    let strip = [0, 1, 2, 3, 3, 4, 4, 5, 6, 7];
    let expected = unstripify(&strip, 0, &mut Workspace::default()).unwrap();
    let bound = unstripify_bound(strip.len()).unwrap();
    for fuel in 0..=strip.len() as u64 {
        let limits = Limits {
            max_bytes: usize::MAX,
            max_work: fuel,
        };
        let mut allocating = Workspace::new(limits);
        let result = unstripify(&strip, 0, &mut allocating);
        let mut caller = Workspace::new(limits);
        let mut destination = vec![u32::MAX; bound + 2];
        let actual = unstripify_into(&mut destination, &strip, 0, &mut caller);
        assert_eq!(actual, result.as_ref().map(Vec::len).map_err(|e| *e));
        assert_eq!(caller.usage().work, allocating.usage().work);
        let written = destination.iter().position(|&v| v == u32::MAX).unwrap();
        assert!(written <= expected.len());
        assert_eq!(destination[..written], expected[..written]);
        assert!(destination[written..].iter().all(|&v| v == u32::MAX));
    }
}

#[test]
fn opacity_measure_ranges_keep_global_sources_and_work_prefixes() {
    let mut indices = [0, 1, 2].repeat(128);
    indices.extend_from_slice(&[0, 1, 3]);
    let packed = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let padded = [
        0.0,
        0.0,
        f32::NAN,
        1.0,
        0.0,
        f32::NAN,
        0.0,
        1.0,
        f32::NAN,
        1.0,
        1.0,
        f32::NAN,
    ];
    for (uvs, stride) in [(&packed[..], 2), (&padded[..], 3)] {
        let mut workspace = Workspace::new(Limits {
            max_work: 33_500,
            ..Limits::default()
        });
        let result =
            opacity_map_measure(&indices, uvs, 4, stride, 8, 8, 1, 0.0, &mut workspace).unwrap();
        assert_eq!(result.sources, [0, 128]);
        assert_eq!(result.levels, [1, 1]);
        assert_eq!(result.omm_indices[..128], [0; 128]);
        assert_eq!(result.omm_indices[128], 1);
        let usage = workspace.usage();
        workspace.set_limits(Limits {
            max_work: usage.work,
            max_bytes: usage.bytes,
        });
        assert_eq!(
            opacity_map_measure(&indices, uvs, 4, stride, 8, 8, 1, 0.0, &mut workspace).unwrap(),
            result
        );
        assert_eq!(workspace.usage().work, usage.work);
        let mut invalid = uvs.to_vec();
        invalid[3 * stride] = f32::NAN;
        for fuel in [33_500, 644, 643] {
            workspace.set_limits(Limits {
                max_work: fuel,
                ..Limits::default()
            });
            let result =
                opacity_map_measure(&indices, &invalid, 4, stride, 8, 8, 1, 0.0, &mut workspace);
            assert_eq!(
                result,
                Err(if fuel == 643 {
                    Error::LimitExceeded
                } else {
                    Error::NumericalFailure
                })
            );
            assert_eq!(workspace.usage().work, if fuel == 643 { 643 } else { 644 });
        }
    }
}

#[test]
fn compact_scratch_boundaries_keep_duplicates_and_limits() {
    // Cross the byte, remap and hash-table stack capacities independently.
    for count in [8usize, 9, 13, 17] {
        for length in [count, 64, 65] {
            let buckets = (count + count / 4).next_power_of_two();
            let bytes = length + buckets * 4 + count * 4;
            let total_work = (4 * count + 1) as u64;
            for fuel in 0..=total_work {
                let mut data = vec![0x11; length];
                let mut levels = vec![1; count];
                let original_offsets: Vec<_> = (0..count as u32).rev().collect();
                let mut offsets = original_offsets.clone();
                let mut indices: Vec<_> = (0..count as i32).chain([-3]).collect();
                let mut ws = Workspace::new(Limits {
                    max_bytes: bytes,
                    max_work: fuel,
                });
                let result = opacity_map_compact(
                    &mut data,
                    &mut levels,
                    &mut offsets,
                    &mut indices,
                    4,
                    &mut ws,
                );
                assert_eq!(ws.usage().work, fuel);
                if fuel == total_work {
                    assert_eq!(result, Ok((1, 1)));
                    assert_eq!(data, vec![0x11; length]);
                    assert_eq!(levels, vec![1; count]);
                    let mut expected_offsets = original_offsets;
                    expected_offsets[0] = 0;
                    expected_offsets[1] = 1;
                    assert_eq!(offsets, expected_offsets);
                    assert_eq!(&indices[..count], vec![0; count]);
                    assert_eq!(indices[count], -3);
                    assert_eq!(ws.usage().bytes, bytes);
                } else {
                    assert_eq!(result, Err(Error::LimitExceeded));
                    if fuel < (2 * count + 1) as u64 {
                        assert_eq!(offsets, original_offsets);
                        assert_eq!(indices, (0..count as i32).chain([-3]).collect::<Vec<_>>());
                    }
                }
            }
            let mut data = vec![0x11; length];
            let mut levels = vec![1; count];
            let mut offsets: Vec<_> = (0..count as u32).collect();
            let mut indices: Vec<_> = (0..count as i32).chain([-3]).collect();
            let mut ws = Workspace::new(Limits {
                max_bytes: bytes - 1,
                max_work: total_work,
            });
            assert_eq!(
                opacity_map_compact(
                    &mut data,
                    &mut levels,
                    &mut offsets,
                    &mut indices,
                    4,
                    &mut ws,
                ),
                Err(Error::LimitExceeded)
            );
            assert_eq!(ws.usage().work, (2 * count + 1) as u64);
            assert_eq!(offsets, (0..count as u32).collect::<Vec<_>>());
            assert_eq!(indices, (0..count as i32).chain([-3]).collect::<Vec<_>>());
        }
    }
}
