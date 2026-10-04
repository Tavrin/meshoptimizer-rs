use meshoptimizer_rs::*;

#[test]
fn fifo_reused_mixed_scratch_matches_fresh_results_and_work() {
    let bytes = [0u8; 16];
    let vertices = VertexStream::packed(&bytes, 1).unwrap();
    let mut ws = Workspace::default();
    let mut indices = (0..16).rev().collect::<Vec<_>>();
    optimize_vertex_fetch_into(&mut [0; 16], &mut indices, vertices, &mut ws).unwrap();
    for cache_size in [3, 16, u32::MAX] {
        for (count, input) in [
            (3, vec![0, 1, 2, 2, 1, 0, 1, 2, 0]),
            (7, vec![6, 5, 4, 3, 2, 1, 0, 0, 0, 6, 5, 0]),
            (1, vec![0, 0, 0]),
        ] {
            let mut fresh = Workspace::default();
            let expected =
                optimize_vertex_cache_fifo(&input, count, cache_size, &mut fresh).unwrap();
            let actual = optimize_vertex_cache_fifo(&input, count, cache_size, &mut ws).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(ws.usage().work, fresh.usage().work);
        }
    }
}

#[test]
fn compact_float_views_exclude_padding_and_preserve_validation_prefixes() {
    let positions = [f32::NAN, 0., 0., 0., 1., 2., 3., f32::NAN];
    let p = Positions::from_interleaved(&positions, 2, 3, 1).unwrap();
    assert_eq!(p.len(), 2);
    assert_eq!(p.get(0), Some([0.; 3]));
    assert_eq!(p.get(1), Some([1., 2., 3.]));
    assert_eq!(p.get(2), None);
    assert!(
        Positions::from_interleaved(&positions, 0, 3, positions.len())
            .unwrap()
            .is_empty()
    );
    let mut values = [f32::NAN, 1., 2., f32::NAN, f32::NAN, 3., f32::NAN, f32::NAN];
    let settings = SimplifySettings {
        target_index_count: 0,
        target_error: 1.,
        options: SimplifyOptions::EMPTY,
    };
    let mut ws = Workspace::default();
    let a = Attributes::from_interleaved(&values, 2, 2, 4, 1).unwrap();
    assert!(matches!(
        simplify_with_attributes(&[], p, a, &[1., 1.], None, settings, &mut ws),
        Err(Error::InvalidParameter)
    ));
    assert_eq!(ws.usage(), Usage { work: 8, bytes: 0 });
    values[6] = 4.;
    let a = Attributes::from_interleaved(&values, 2, 2, 4, 1).unwrap();
    assert!(simplify_with_attributes(&[], p, a, &[1., 1.], None, settings, &mut ws).is_ok());
}

#[test]
fn normalization_counts_only_the_visited_validation_prefix() {
    let p = [[f32::NAN, 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let mut ws = Workspace::default();
    assert!(matches!(
        simplify_points(Positions::from_packed(&p), None, 0., 1, &mut ws),
        Err(Error::InvalidParameter)
    ));
    assert_eq!(ws.usage().work, 1);
    assert_eq!(ws.usage().bytes, 0);
    let mut out = [99; 4];
    let finite = [[0.; 3]; 4];
    let mut limited = Workspace::new(Limits {
        max_bytes: 0,
        max_work: 2,
    });
    assert_eq!(
        simplify_points_into(
            &mut out,
            Positions::from_packed(&finite),
            None,
            0.,
            1,
            &mut limited
        ),
        Err(Error::LimitExceeded)
    );
    assert_eq!(limited.usage().work, 2);
    assert_eq!(out, [99; 4]);
    let colors = [[f32::NAN, 0., 0.], [0.; 3], [0.; 3], [0.; 3]];
    let mut ws = Workspace::default();
    assert!(matches!(
        simplify_points(
            Positions::from_packed(&finite),
            Some(Positions::from_packed(&colors)),
            0.5,
            1,
            &mut ws
        ),
        Err(Error::InvalidParameter)
    ));
    assert_eq!(ws.usage(), Usage { bytes: 0, work: 1 });
}

#[test]
fn early_validation_resets_measurements_and_preserves_retained_scratch() {
    let bytes = [0, 1, 2];
    let vertices = VertexStream::packed(&bytes, 1).unwrap();
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    for case in 0..9 {
        let mut ws = Workspace::default();
        generate_vertex_remap(None, vertices, &mut ws).unwrap();
        assert!(ws.usage().work > 0 && ws.usage().bytes > 0);
        let result = match case {
            0 => generate_vertex_remap_multi(None, &[], &mut ws).map(|_| ()),
            1 => generate_vertex_remap_multi_into(&mut [0; 3], None, &[], &mut ws).map(|_| ()),
            2 => generate_shadow_index_buffer_multi(&[], &[], &mut ws).map(|_| ()),
            3 => filter_index_buffer_multi(&[], &[], &mut ws).map(|_| ()),
            4 => remap_vertex_buffer(vertices, &[], &mut ws).map(|_| ()),
            5 => remap_vertex_buffer(vertices, &[3, 0, 1], &mut ws).map(|_| ()),
            6 => simplify_sloppy_into(
                &mut [],
                &[0, 1, 2],
                Positions::from_packed(&p),
                None,
                0,
                1.,
                &mut ws,
            )
            .map(|_| ()),
            7 => simplify_prune_into(&mut [], &[0, 1, 2], Positions::from_packed(&p), 1., &mut ws)
                .map(|_| ()),
            _ => simplify_points_into(&mut [], Positions::from_packed(&p), None, 1., 1, &mut ws)
                .map(|_| ()),
        };
        assert!(result.is_err());
        if case == 5 {
            assert_eq!(ws.usage().work, 1);
        } else {
            assert_eq!(ws.usage(), Usage::default());
        }
        optimize_vertex_fetch_remap_into(&mut [], &[], 0, &mut ws).unwrap();
        assert!(ws.usage().bytes > 0, "case {case} dropped retained scratch");
    }
}

#[test]
fn singleton_point_cells_preserve_numerics_tails_and_exhaustion_prefix() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let colors = [[f32::MAX; 3]; 4];
    let mut out = [99; 6];
    let mut ws = Workspace::default();
    assert_eq!(
        simplify_points_into(
            &mut out,
            Positions::from_packed(&p),
            Some(Positions::from_packed(&colors)),
            0.,
            4,
            &mut ws,
        )
        .unwrap(),
        4
    );
    assert_eq!(out, [0, 1, 2, 3, 99, 99]);
    let usage = ws.usage();
    let mut limited = Workspace::new(Limits {
        max_bytes: usage.bytes,
        max_work: usage.work - 1,
    });
    out.fill(99);
    assert_eq!(
        simplify_points_into(
            &mut out,
            Positions::from_packed(&p),
            Some(Positions::from_packed(&colors)),
            0.,
            4,
            &mut limited,
        ),
        Err(Error::LimitExceeded)
    );
    assert_eq!(out, [0, 1, 2, 99, 99, 99]);
    out.fill(99);
    assert_eq!(
        simplify_points_into(
            &mut out,
            Positions::from_packed(&p),
            None,
            f32::MAX,
            4,
            &mut Workspace::default(),
        ),
        Err(Error::NumericalFailure)
    );
    assert_eq!(out, [99; 6]);
    let tiny = [[0.; 3], [f32::MIN_POSITIVE / 256., 0., 0.]];
    assert!(matches!(
        simplify_points(
            Positions::from_packed(&tiny),
            None,
            0.,
            2,
            &mut Workspace::default(),
        ),
        Err(Error::NumericalFailure)
    ));
    assert_eq!(
        simplify_points(
            Positions::from_packed(&tiny),
            None,
            0.,
            0,
            &mut Workspace::default(),
        )
        .unwrap(),
        []
    );
}

#[test]
fn first_use_fetch_and_binary_welding_are_distinct() {
    let vertices = VertexStream::packed(&[10, 20, 10, 30], 1).unwrap();
    let indices = [3, 2, 1, 0];
    let mut ws = Workspace::default();
    let weld = generate_vertex_remap(Some(&indices), vertices, &mut ws).unwrap();
    assert_eq!(weld.remap, [1, 2, 1, 0]);
    assert_eq!(weld.vertex_count, 3);
    let fetch = optimize_vertex_fetch(&indices, vertices, &mut ws).unwrap();
    assert_eq!(fetch.indices, [0, 1, 2, 3]);
    assert_eq!(fetch.vertices, [30, 10, 20, 10]);
    let unindexed = generate_vertex_remap(None, vertices, &mut ws).unwrap();
    assert_eq!(unindexed.remap, [0, 1, 0, 2]);
}
#[test]
fn remap_callbacks_and_signed_zero_follow_upstream_rules() {
    let p = [[-0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    let mut calls = Vec::new();
    let mut ws = Workspace::default();
    let r = generate_vertex_remap_custom(
        None,
        Positions::from_packed(&p),
        |a, b| {
            calls.push((a, b));
            true
        },
        &mut ws,
    )
    .unwrap();
    assert_eq!(r.remap, [0, 0, 0]);
    assert_eq!(calls, [(0, 1), (0, 2)]);
    assert_eq!(
        generate_position_remap(Positions::from_packed(&p), &mut ws).unwrap(),
        [0, 0, 0]
    );
    let raw: Vec<_> = p
        .iter()
        .flatten()
        .flat_map(|x| f32::to_bits(*x).to_le_bytes())
        .collect();
    let s = VertexStream::packed(&raw, 12).unwrap();
    assert_eq!(
        generate_vertex_remap(None, s, &mut ws).unwrap().remap,
        [0, 1, 1]
    );
}
#[test]
fn filtering_preserves_winding_and_representative_source_indices() {
    let s = VertexStream::packed(&[0, 1, 2, 0], 1).unwrap();
    let mut ws = Workspace::default();
    let ib = [0, 1, 2, 1, 2, 3, 0, 2, 1, 0, 0, 2];
    assert_eq!(
        filter_index_buffer(&ib, s, &mut ws).unwrap(),
        [0, 1, 2, 0, 2, 1]
    );
    assert_eq!(
        generate_shadow_index_buffer(&ib, s, &mut ws).unwrap(),
        [0, 1, 2, 1, 2, 0, 0, 2, 1, 0, 0, 2]
    );
}
#[test]
fn checked_layout_topology_and_unused_remaps() {
    assert!(matches!(
        VertexStream::new(&[0], usize::MAX, 1, usize::MAX, 1),
        Err(Error::SizeOverflow)
    ));
    assert!(matches!(
        VertexStream::new(&[], 0, 0, 0, 0),
        Err(Error::InvalidLayout)
    ));
    let mut ws = Workspace::default();
    let s = VertexStream::packed(&[0, 1, 2], 1).unwrap();
    assert!(matches!(
        filter_index_buffer(&[0], s, &mut ws),
        Err(Error::InvalidTopology)
    ));
    assert!(matches!(
        optimize_vertex_fetch_remap(&[3], 3, &mut ws),
        Err(Error::IndexOutOfBounds)
    ));
    let mut out = [123; 4];
    assert_eq!(
        remap_index_buffer_into(&mut out, Some(&[1]), &[0, u32::MAX], &mut ws),
        Err(Error::InvalidParameter)
    );
    assert_eq!(out, [123; 4]);
    assert_eq!(
        optimize_vertex_fetch_remap_into(&mut out, &[2, 0, 2], 3, &mut ws).unwrap(),
        2
    );
    assert_eq!(out, [1, u32::MAX, 0, 123]);
}
#[test]
fn preprocessing_work_and_memory_limits_are_enforced() {
    let vertices = VertexStream::packed(&[0, 1, 2], 1).unwrap();
    let mut ws = Workspace::default();
    generate_vertex_remap(None, vertices, &mut ws).unwrap();
    let used = ws.usage();
    let exact = Limits {
        max_bytes: used.bytes,
        max_work: used.work,
    };
    assert!(generate_vertex_remap(None, vertices, &mut Workspace::new(exact)).is_ok());
    assert!(matches!(
        generate_vertex_remap(
            None,
            vertices,
            &mut Workspace::new(Limits {
                max_work: used.work - 1,
                ..exact
            })
        ),
        Err(Error::LimitExceeded)
    ));
    assert!(matches!(
        generate_vertex_remap(
            None,
            vertices,
            &mut Workspace::new(Limits {
                max_bytes: used.bytes - 1,
                ..exact
            })
        ),
        Err(Error::LimitExceeded)
    ));
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let flags = [VertexFlags::PROTECT; 3];
    assert!(matches!(
        simplify_sloppy(
            &[0, 1, 2],
            Positions::from_packed(&p),
            Some(&flags),
            0,
            1.,
            &mut ws
        ),
        Err(Error::UnknownFlags)
    ));
}

#[test]
fn pruning_welds_signed_zero_across_disconnected_triangle_lists() {
    let p = [
        [-0.0, 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [0.0, 0., 0.],
        [-1., 0., 0.],
        [0., -1., 0.],
    ];
    assert_eq!(
        simplify_prune(
            &[0, 1, 2, 3, 4, 5],
            Positions::from_packed(&p),
            0.4,
            &mut Workspace::default()
        )
        .unwrap(),
        [0, 1, 2, 3, 4, 5]
    );
}
#[test]
fn mutable_byte_views_preserve_padding_and_external_locks() {
    let mut positions = [0u8; 32];
    positions[12..16].copy_from_slice(&[7, 8, 9, 10]);
    positions[28..32].copy_from_slice(&[11, 12, 13, 14]);
    let mut p = PositionsMut::from_bytes(&mut positions, 2, 16, 0, ByteOrder::BigEndian).unwrap();
    let mut indices = [];
    let flags = [VertexFlags::LOCK; 2];
    let mut ws = Workspace::default();
    let r = simplify_with_update(
        &mut indices,
        &mut p,
        None,
        &[],
        Some(&flags),
        SimplifySettings {
            target_index_count: 0,
            target_error: 1.,
            options: SimplifyOptions::SPARSE,
        },
        &mut ws,
    )
    .unwrap();
    assert_eq!(r.index_count, 0);
    assert_eq!(&positions[12..16], &[7, 8, 9, 10]);
    assert_eq!(&positions[28..32], &[11, 12, 13, 14]);
}
#[test]
fn half_rules_preserve_payloads_and_flush_subnormals() {
    assert_eq!(quantize_unorm(1., 23).unwrap(), 8_388_607);
    assert_eq!(quantize_unorm(1., 24).unwrap(), 16_777_216);
    assert_eq!(
        quantize_unorm(f32::from_bits(0x3f7fffff), 24).unwrap(),
        16_777_214
    );
    assert_eq!(quantize_unorm(f32::INFINITY, 30).unwrap(), 1 << 30);
    assert_eq!(quantize_half(-0.0), 0x8000);
    assert_eq!(quantize_half(f32::from_bits(0xff800001)), 0xfe00);
    assert_eq!(dequantize_half(0x8001).to_bits(), 0x80000000);
    assert_eq!(dequantize_half(0x7e01).to_bits(), 0x7fc02000);
    assert_eq!(quantize_float(-0.0, 23).unwrap().to_bits(), 0);
    assert_eq!(
        quantize_float(f32::from_bits(0x7f800001), 1)
            .unwrap()
            .to_bits(),
        0x7f800001
    );
    assert_eq!(quantize_unorm(f32::NAN, 8).unwrap(), 0);
    assert_eq!(quantize_snorm(f32::NAN, 8).unwrap(), -127);
    assert!(quantize_float(1.0, 24).is_err());
    assert_eq!(quantize_unorm(1.0, 0).unwrap(), 0);
    assert_eq!(quantize_unorm(1.0, 30).unwrap(), 1 << 30);
    assert_eq!(quantize_snorm(-1.0, 1).unwrap(), 0);
    assert_eq!(quantize_snorm(1.0, 31).unwrap(), 1 << 30);
    assert_eq!(
        compute_position_exponent([0.; 3], [0.; 3], i32::MAX, 24).unwrap(),
        i32::MAX
    );
}

#[test]
fn every_preprocessing_kernel_enforces_its_measured_work_and_heap_boundary() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 0.]];
    let ib = [0, 1, 2, 3, 1, 2];
    let raw = [0u8, 1, 2, 0];
    let stream = VertexStream::packed(&raw, 1).unwrap();
    let remap = [0, 1, 2, 0];
    let run = |family: usize, ws: &mut Workspace| -> Result<(), Error> {
        let positions = Positions::from_packed(&p);
        let mut destination = [0; 24];
        match family {
            0 => {
                optimize_vertex_cache_strip(&ib, p.len(), ws)?;
            }
            1 => {
                optimize_vertex_cache_fifo(&ib, p.len(), 16, ws)?;
            }
            2 => {
                generate_vertex_remap(Some(&ib), stream, ws)?;
            }
            3 => {
                generate_vertex_remap_multi(Some(&ib), &[stream, stream], ws)?;
            }
            4 => {
                generate_vertex_remap_custom(Some(&ib), positions, |_, _| true, ws)?;
            }
            5 => {
                remap_vertex_buffer(stream, &remap, ws)?;
            }
            6 => {
                remap_index_buffer(Some(&ib), &remap, ws)?;
            }
            7 => {
                filter_index_buffer(&ib, stream, ws)?;
            }
            8 => {
                filter_index_buffer_multi(&ib, &[stream, stream], ws)?;
            }
            9 => {
                generate_shadow_index_buffer(&ib, stream, ws)?;
            }
            10 => {
                generate_shadow_index_buffer_multi(&ib, &[stream, stream], ws)?;
            }
            11 => {
                generate_position_remap(positions, ws)?;
            }
            12 => {
                generate_adjacency_index_buffer(&ib, positions, ws)?;
            }
            13 => {
                generate_tessellation_index_buffer(&ib, positions, ws)?;
            }
            14 => {
                generate_provoking_index_buffer(&ib, p.len(), ws)?;
            }
            15 => {
                optimize_vertex_fetch(&ib, stream, ws)?;
            }
            16 => {
                optimize_vertex_fetch_remap(&ib, p.len(), ws)?;
            }
            17 => {
                simplify_sloppy(&ib, positions, None, 3, 1., ws)?;
            }
            18 => {
                simplify_prune(&ib, positions, 0.1, ws)?;
            }
            19 => {
                simplify_points(positions, None, 0., 2, ws)?;
            }
            20 => {
                let mut positions = p;
                let mut indices = ib;
                simplify_with_update(
                    &mut indices,
                    &mut PositionsMut::from_packed(&mut positions),
                    None,
                    &[],
                    None,
                    SimplifySettings {
                        target_index_count: 3,
                        target_error: 1.,
                        options: SimplifyOptions::default(),
                    },
                    ws,
                )?;
            }
            21 => {
                optimize_vertex_cache_fifo_into(&mut destination, &ib, p.len(), 16, ws)?;
            }
            22 => {
                filter_index_buffer_into(&mut destination, &ib, stream, ws)?;
            }
            23 => {
                simplify_points_into(&mut destination, positions, None, 0., 2, ws)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    };
    for family in 0..24 {
        let mut ws = Workspace::default();
        run(family, &mut ws).unwrap();
        let usage = ws.usage();
        assert!(usage.bytes > 0 && usage.work > 0, "family {family}");
        let limits = Limits {
            max_bytes: usage.bytes,
            max_work: usage.work,
        };
        run(family, &mut Workspace::new(limits)).unwrap();
        assert_eq!(
            run(
                family,
                &mut Workspace::new(Limits {
                    max_work: usage.work - 1,
                    ..limits
                })
            ),
            Err(Error::LimitExceeded),
            "work family {family}"
        );
        assert_eq!(
            run(
                family,
                &mut Workspace::new(Limits {
                    max_bytes: usage.bytes - 1,
                    ..limits
                })
            ),
            Err(Error::LimitExceeded),
            "heap family {family}"
        );
    }
}

#[test]
fn zero_point_target_validates_without_rescaling_or_allocating() {
    let p = [[-f32::MAX, 0.0, 0.0], [f32::MAX, 0.0, 0.0]];
    let mut ws = Workspace::new(Limits {
        max_bytes: 0,
        max_work: 2,
    });
    assert!(
        simplify_points(Positions::from_packed(&p), None, 0.0, 0, &mut ws)
            .unwrap()
            .is_empty()
    );
    assert_eq!(ws.usage(), Usage { bytes: 0, work: 2 });
    let mut out = [123; 3];
    assert_eq!(
        simplify_points_into(&mut out, Positions::from_packed(&p), None, 0.0, 0, &mut ws).unwrap(),
        0
    );
    assert_eq!(out, [123; 3]);
    let nan = [[f32::NAN, 0.0, 0.0]];
    assert_eq!(
        simplify_points(Positions::from_packed(&nan), None, 0.0, 0, &mut ws),
        Err(Error::InvalidParameter)
    );
}

#[test]
fn packed_and_strided_preprocessing_views_have_identical_results() {
    let p = [
        [-0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0],
    ];
    let bytes: Vec<_> = p
        .iter()
        .flat_map(|v| {
            v.iter()
                .flat_map(|x| f32::to_bits(*x).to_be_bytes())
                .chain([123; 4])
        })
        .collect();
    let packed = Positions::from_packed(&p);
    let strided = Positions::from_bytes(&bytes, 4, 16, 0, ByteOrder::BigEndian).unwrap();
    let mut ws = Workspace::default();
    assert_eq!(
        generate_position_remap(packed, &mut ws).unwrap(),
        generate_position_remap(strided, &mut ws).unwrap()
    );
    assert_eq!(
        generate_vertex_remap_custom(None, packed, |_, _| true, &mut ws)
            .unwrap()
            .remap,
        generate_vertex_remap_custom(None, strided, |_, _| true, &mut ws)
            .unwrap()
            .remap
    );
    for target in 1..=4 {
        assert_eq!(
            simplify_points(packed, Some(packed), 1.0, target, &mut ws).unwrap(),
            simplify_points(strided, Some(strided), 1.0, target, &mut ws).unwrap()
        );
    }
    assert_eq!(
        generate_adjacency_index_buffer(&[0, 1, 2, 3, 2, 1], packed, &mut ws).unwrap(),
        generate_adjacency_index_buffer(&[0, 1, 2, 3, 2, 1], strided, &mut ws).unwrap()
    );
}

#[test]
fn typed_simplifier_scratch_reuse_preserves_results_and_retained_budget() {
    let mut reused = Workspace::default();
    for (side, width) in [(5, 8), (2, 0), (4, 32), (1, 1), (3, 3)] {
        let mut p = Vec::new();
        for y in 0..=side {
            for x in 0..=side {
                p.push([x as f32, y as f32, (x * x + y * y) as f32 * 0.001]);
            }
        }
        p.extend([[0.; 3], [99., 99., 99.]]);
        let mut indices = Vec::new();
        for y in 0..side {
            for x in 0..side {
                let a = (y * (side + 1) + x) as u32;
                let b = a + 1;
                let c = a + side as u32 + 1;
                let d = c + 1;
                indices.extend([a, b, c, c, b, d]);
            }
        }
        let stride = width.max(1);
        let attributes: Vec<_> = (0..p.len() * stride)
            .map(|i| (i % 17) as f32 / 8.)
            .collect();
        let weights: Vec<_> = (0..width)
            .map(|i| if i % 4 == 0 { 0. } else { 1. })
            .collect();
        let flags: Vec<_> = (0..p.len())
            .map(|i| {
                if i % 7 == 0 {
                    VertexFlags::LOCK
                } else {
                    VertexFlags::EMPTY
                }
            })
            .collect();
        for bits in [0, 2, 8, 64, 128, 256] {
            let Ok(options) = SimplifyOptions::from_bits(bits) else {
                continue;
            };
            let settings = SimplifySettings {
                target_index_count: indices.len() / 2,
                target_error: 0.5,
                options,
            };
            let a = Attributes::from_interleaved(&attributes, p.len(), width, stride, 0).unwrap();
            let expected = simplify_with_attributes(
                &indices,
                Positions::from_packed(&p),
                a,
                &weights,
                Some(&flags),
                settings,
                &mut Workspace::default(),
            )
            .unwrap();
            let actual = simplify_with_attributes(
                &indices,
                Positions::from_packed(&p),
                a,
                &weights,
                Some(&flags),
                settings,
                &mut reused,
            )
            .unwrap();
            assert_eq!(actual.indices, expected.indices);
            assert_eq!(actual.error.to_bits(), expected.error.to_bits());
            let mut pa = p.clone();
            let mut pe = p.clone();
            let mut aa = attributes.clone();
            let mut ae = attributes.clone();
            let mut ia = indices.clone();
            let mut ie = indices.clone();
            let expected = simplify_with_update(
                &mut ie,
                &mut PositionsMut::from_packed(&mut pe),
                Some(
                    &mut AttributesMut::from_interleaved(&mut ae, p.len(), width, stride, 0)
                        .unwrap(),
                ),
                &weights,
                Some(&flags),
                settings,
                &mut Workspace::default(),
            )
            .unwrap();
            let actual = simplify_with_update(
                &mut ia,
                &mut PositionsMut::from_packed(&mut pa),
                Some(
                    &mut AttributesMut::from_interleaved(&mut aa, p.len(), width, stride, 0)
                        .unwrap(),
                ),
                &weights,
                Some(&flags),
                settings,
                &mut reused,
            )
            .unwrap();
            assert_eq!(actual.index_count, expected.index_count);
            assert_eq!(actual.error.to_bits(), expected.error.to_bits());
            assert_eq!(ia, ie);
            assert_eq!(
                pa.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>(),
                pe.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>()
            );
            assert_eq!(
                aa.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                ae.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
            );
        }
    }
    optimize_vertex_fetch_remap_into(&mut [], &[], 0, &mut reused).unwrap();
    assert!(reused.usage().bytes > 0);
    reused.set_limits(Limits {
        max_bytes: 0,
        max_work: 0,
    });
    optimize_vertex_fetch_remap_into(&mut [], &[], 0, &mut reused).unwrap();
    assert_eq!(reused.usage(), Usage::default());
}

#[test]
fn sloppy_scratch_reuse_clears_cell_state_and_combines_typed_budgets() {
    let mut reused = Workspace::default();
    for (side, flat) in [(5, false), (1, true), (3, false), (2, false)] {
        let mut p = Vec::new();
        for y in 0..=side {
            for x in 0..=side {
                p.push(if flat {
                    [0.; 3]
                } else {
                    [x as f32, y as f32, (x * x + y * y) as f32 * 0.001]
                });
            }
        }
        let mut indices = Vec::new();
        for y in 0..side {
            for x in 0..side {
                let a = (y * (side + 1) + x) as u32;
                let b = a + 1;
                let c = a + side as u32 + 1;
                indices.extend([a, b, c, c, b, c + 1]);
            }
        }
        let locks: Vec<_> = (0..p.len())
            .map(|i| {
                if i % 7 == 0 {
                    VertexFlags::LOCK
                } else {
                    VertexFlags::EMPTY
                }
            })
            .collect();
        for locked in [None, Some(locks.as_slice())] {
            for ratio in [0, 2, 1] {
                let target = indices.len() * ratio / 2;
                let mut fresh = Workspace::default();
                let expected = simplify_sloppy(
                    &indices,
                    Positions::from_packed(&p),
                    locked,
                    target,
                    0.1,
                    &mut fresh,
                )
                .unwrap();
                let actual = simplify_sloppy(
                    &indices,
                    Positions::from_packed(&p),
                    locked,
                    target,
                    0.1,
                    &mut reused,
                )
                .unwrap();
                assert_eq!(actual.indices, expected.indices);
                assert_eq!(actual.error.to_bits(), expected.error.to_bits());
                assert_eq!(reused.usage().work, fresh.usage().work);
                let mut out = vec![99; indices.len() + 3];
                let actual = simplify_sloppy_into(
                    &mut out,
                    &indices,
                    Positions::from_packed(&p),
                    locked,
                    target,
                    0.1,
                    &mut reused,
                )
                .unwrap();
                assert_eq!(&out[..actual.index_count], expected.indices);
                assert_eq!(actual.error.to_bits(), expected.error.to_bits());
                assert_eq!(&out[indices.len()..], &[99; 3]);
            }
        }
        let settings = SimplifySettings {
            target_index_count: indices.len() / 2,
            target_error: 0.5,
            options: SimplifyOptions::EMPTY,
        };
        let expected = simplify(
            &indices,
            Positions::from_packed(&p),
            settings,
            &mut Workspace::default(),
        )
        .unwrap();
        let actual = simplify(&indices, Positions::from_packed(&p), settings, &mut reused).unwrap();
        assert_eq!(actual.indices, expected.indices);
        assert_eq!(actual.error.to_bits(), expected.error.to_bits());
        let mut baseline = Workspace::default();
        simplify(
            &indices,
            Positions::from_packed(&p),
            settings,
            &mut baseline,
        )
        .unwrap();
        let main_bytes = baseline.usage().bytes;
        let mut baseline = Workspace::default();
        simplify_sloppy(
            &indices,
            Positions::from_packed(&p),
            None,
            indices.len() / 2,
            0.1,
            &mut baseline,
        )
        .unwrap();
        let sloppy_bytes = baseline.usage().bytes;
        let mut bounded = Workspace::new(Limits {
            max_bytes: main_bytes.max(sloppy_bytes),
            ..Limits::default()
        });
        simplify_sloppy(
            &indices,
            Positions::from_packed(&p),
            None,
            indices.len() / 2,
            0.1,
            &mut bounded,
        )
        .unwrap();
        assert!(matches!(
            simplify(&indices, Positions::from_packed(&p), settings, &mut bounded),
            Err(Error::LimitExceeded)
        ));
        bounded.clear();
        simplify(&indices, Positions::from_packed(&p), settings, &mut bounded).unwrap();
    }
}

#[test]
fn remap_validation_precedence_and_limited_write_prefix() {
    let mut ws = Workspace::default();
    let mut out = [99; 3];
    assert_eq!(
        remap_index_buffer_into(&mut out, Some(&[0, 3]), &[u32::MAX, 7], &mut ws),
        Err(Error::IndexOutOfBounds)
    );
    assert_eq!(out, [99; 3]);
    assert_eq!(ws.usage().work, 2);
    assert_eq!(
        remap_index_buffer_into(&mut out[..1], Some(&[0, 1]), &[u32::MAX, 7], &mut ws),
        Err(Error::BufferTooSmall)
    );
    assert_eq!(out, [99; 3]);
    assert_eq!(ws.usage().work, 2);
    assert_eq!(
        remap_index_buffer_into(&mut out, Some(&[1, 0, 1]), &[u32::MAX, 7], &mut ws),
        Err(Error::InvalidParameter)
    );
    assert_eq!(out, [99; 3]);
    assert_eq!(ws.usage().work, 5);
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: 4,
    });
    assert_eq!(
        remap_index_buffer_into(&mut out, Some(&[1, 0, 1]), &[u32::MAX, 7], &mut ws),
        Err(Error::LimitExceeded)
    );
    assert_eq!(out, [99; 3]);
    assert_eq!(ws.usage().work, 4);
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: 8,
    });
    assert_eq!(
        remap_index_buffer_into(&mut out, Some(&[1, 0, 1]), &[5, 7], &mut ws),
        Err(Error::LimitExceeded)
    );
    assert_eq!(out, [7, 5, 99]);
    assert_eq!(ws.usage().work, 8);
}

#[test]
fn bounded_hash_batches_preserve_every_callback_and_write_prefix() {
    let positions = [[0.0; 3]; 4];
    let indices = [0, 1, 2, 3, 1, 2];
    let callbacks = [
        (22, (0, 1)),
        (25, (0, 2)),
        (26, (1, 2)),
        (29, (0, 3)),
        (30, (1, 3)),
        (31, (2, 3)),
    ];
    for limit in 0..=64 {
        let mut ws = Workspace::new(Limits {
            max_bytes: 1024,
            max_work: limit,
        });
        let mut destination = [99; 7];
        let mut seen = Vec::new();
        let result = generate_vertex_remap_custom_into(
            &mut destination,
            Some(&indices),
            Positions::from_packed(&positions),
            |a, b| {
                seen.push((a, b));
                false
            },
            &mut ws,
        );
        assert_eq!(
            result,
            if limit >= 34 {
                Ok(4)
            } else {
                Err(Error::LimitExceeded)
            },
            "limit {limit}"
        );
        assert_eq!(
            seen,
            callbacks
                .iter()
                .filter(|(at, _)| *at <= limit)
                .map(|(_, pair)| *pair)
                .collect::<Vec<_>>(),
            "limit {limit}"
        );
        let mut expected = [99; 7];
        if limit >= 10 {
            expected[..4].fill(u32::MAX);
        }
        for (index, at) in [20, 23, 27, 32].into_iter().enumerate() {
            if limit >= at {
                expected[index] = index as u32;
            }
        }
        assert_eq!(destination, expected, "limit {limit}");
        let used = if limit < 6 {
            limit
        } else if limit < 10 {
            6
        } else if limit < 18 {
            10
        } else {
            limit.min(34)
        };
        assert_eq!(ws.usage().work, used, "limit {limit}");
    }
}

#[test]
fn fused_shadow_generation_preserves_counted_write_prefixes() {
    let data = [0u8; 16];
    let vertices = VertexStream::packed(&data, 4).unwrap();
    let indices = [0, 1, 2, 2, 1, 3];
    for limit in 0..=40 {
        let mut ws = Workspace::new(Limits {
            max_bytes: 1024,
            max_work: limit,
        });
        let mut output = [99; 7];
        let result = generate_shadow_index_buffer_into(&mut output, &indices, vertices, &mut ws);
        assert_eq!(
            result,
            if limit >= 34 {
                Ok(6)
            } else {
                Err(Error::LimitExceeded)
            },
            "limit {limit}"
        );
        let mut expected = [99; 7];
        for (i, at) in [21, 24, 27, 29, 31, 34].into_iter().enumerate() {
            if limit >= at {
                expected[i] = 0;
            }
        }
        assert_eq!(output, expected, "limit {limit}");
        assert!(ws.usage().work <= limit);
        if limit >= 34 {
            assert_eq!(ws.usage().work, 34);
        }
    }
}

#[test]
fn bounded_triangle_filter_preserves_winding_and_work_prefixes() {
    let vertices = VertexStream::packed(&[0, 1, 2], 1).unwrap();
    let indices = [0, 1, 2, 1, 2, 0, 2, 1, 0, 0, 0, 1];
    for limit in 0..=60 {
        let mut ws = Workspace::new(Limits {
            max_bytes: 1024,
            max_work: limit,
        });
        let mut output = [99; 13];
        let result = filter_index_buffer_into(&mut output, &indices, vertices, &mut ws);
        assert_eq!(
            result,
            if limit >= 51 {
                Ok(6)
            } else {
                Err(Error::LimitExceeded)
            },
            "limit {limit}"
        );
        let mut expected = [99; 13];
        if limit >= 45 {
            expected[..3].copy_from_slice(&[0, 1, 2]);
        }
        if limit >= 50 {
            expected[3..6].copy_from_slice(&[2, 1, 0]);
        }
        assert_eq!(output, expected, "limit {limit}");
        assert!(ws.usage().work <= limit);
        if limit >= 51 {
            assert_eq!(ws.usage().work, 51);
        }
    }
}

#[test]
fn fetch_remap_initialization_preserves_mixed_workspace_reuse() {
    let mut reused = Workspace::default();
    for count in [3usize, 7, 1, 5, 0, 9, 1024, 2] {
        let bytes: Vec<u8> = (0..count * 4).map(|i| i as u8).collect();
        let vertices = VertexStream::packed(&bytes, 4).unwrap();
        let indices: Vec<u32> = (0..count as u32).rev().chain(0..count as u32).collect();
        let mut fresh = Workspace::default();
        let expected = optimize_vertex_fetch(&indices, vertices, &mut fresh).unwrap();
        let work = fresh.usage().work;
        let actual = optimize_vertex_fetch(&indices, vertices, &mut reused).unwrap();
        assert_eq!(actual.indices, expected.indices);
        assert_eq!(actual.vertices, expected.vertices);
        assert_eq!(actual.vertex_count, expected.vertex_count);
        assert_eq!(reused.usage().work, work);

        let mut actual_indices = indices.clone();
        let mut expected_indices = indices.clone();
        let mut actual_bytes = vec![0xa5; bytes.len() + 4];
        let mut expected_bytes = actual_bytes.clone();
        let expected_count = optimize_vertex_fetch_into(
            &mut expected_bytes,
            &mut expected_indices,
            vertices,
            &mut fresh,
        )
        .unwrap();
        let actual_count = optimize_vertex_fetch_into(
            &mut actual_bytes,
            &mut actual_indices,
            vertices,
            &mut reused,
        )
        .unwrap();
        assert_eq!(actual_count, expected_count);
        assert_eq!(actual_indices, expected_indices);
        assert_eq!(actual_bytes, expected_bytes);
        assert_eq!(reused.usage().work, fresh.usage().work);
        optimize_vertex_cache_fifo(&[0, 1, 2], 3, 16, &mut reused).unwrap();
    }
}

#[test]
fn fixed_width_fetch_copies_only_used_records_and_charges_full_capacity() {
    for count in [6usize, 1024] {
        for size in [4, 8, 12, 16, 33] {
            let stride = size + 5;
            let data: Vec<u8> = (0..3 + count * stride).map(|i| (i * 7) as u8).collect();
            let vertices = VertexStream::new(&data, count, size, stride, 3).unwrap();
            let last = count as u32 - 1;
            let indices = [last, 1, last, 0, 1];
            let expected_bytes: Vec<u8> = [count - 1, 1, 0]
                .into_iter()
                .flat_map(|i| data[3 + i * stride..3 + i * stride + size].iter().copied())
                .collect();
            let mut baseline = Workspace::default();
            let result = optimize_vertex_fetch(&indices, vertices, &mut baseline).unwrap();
            assert_eq!(result.indices, [0, 1, 0, 2, 1]);
            assert_eq!(result.vertices, expected_bytes);
            assert_eq!(result.vertex_count, 3);
            let usage = baseline.usage();
            assert_eq!(usage.work, 15);
            assert_eq!(usage.bytes, indices.len() * 4 + count * (size + 4));
            let exact = Limits {
                max_bytes: usage.bytes,
                max_work: usage.work,
            };
            optimize_vertex_fetch(&indices, vertices, &mut Workspace::new(exact)).unwrap();
            for limited in [
                Limits {
                    max_bytes: usage.bytes - 1,
                    ..exact
                },
                Limits {
                    max_work: usage.work - 1,
                    ..exact
                },
            ] {
                assert!(matches!(
                    optimize_vertex_fetch(&indices, vertices, &mut Workspace::new(limited)),
                    Err(Error::LimitExceeded)
                ));
            }
        }
    }
}

#[test]
fn sentinel_shadow_scratch_resets_after_fifo_and_across_stream_widths() {
    let mut reused = Workspace::default();
    for count in [3usize, 7, 1, 0, 128, 2] {
        for size in [4, 8, 12, 16, 33] {
            optimize_vertex_cache_fifo(&[0, 1, 2], 3, 16, &mut reused).unwrap();
            let stride = size + 3;
            let bytes: Vec<u8> = (0..2 + count * stride).map(|i| (i * 13) as u8).collect();
            let stream = VertexStream::new(&bytes, count, size, stride, 2).unwrap();
            let indices: Vec<u32> = (0..count as u32)
                .flat_map(|i| [i, (i + 1) % count as u32, i])
                .collect();
            for streams in [vec![stream], vec![stream, stream]] {
                let mut fresh = Workspace::default();
                let expected =
                    generate_shadow_index_buffer_multi(&indices, &streams, &mut fresh).unwrap();
                let expected_work = fresh.usage().work;
                let actual =
                    generate_shadow_index_buffer_multi(&indices, &streams, &mut reused).unwrap();
                assert_eq!(actual, expected);
                assert_eq!(reused.usage().work, expected_work);
                let mut output = vec![99; indices.len() + 4];
                let count = generate_shadow_index_buffer_multi_into(
                    &mut output,
                    &indices,
                    &streams,
                    &mut reused,
                )
                .unwrap();
                assert_eq!(count, indices.len());
                assert_eq!(&output[..count], expected);
                assert_eq!(&output[count..], [99; 4]);
                assert_eq!(reused.usage().work, expected_work);
            }
        }
    }
}

#[test]
fn update_solving_matches_tight_work_limits_for_wedges_and_attributes() {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for patch in 0..2 {
        for y in 0..5 {
            for x in 0..5 {
                positions.push([x as f32, y as f32, (x * x + y * y) as f32 * 0.00001]);
            }
        }
        for y in 0..4 {
            for x in 0..4 {
                let a = patch * 25 + y * 5 + x;
                indices.extend([a, a + 1, a + 5, a + 5, a + 1, a + 6]);
            }
        }
    }
    positions.extend([[0., 0., 0.], [8., 8., 8.]]);
    for width in [0, 1, 8, 32] {
        let stride = width + 1;
        let weights: Vec<_> = (0..width).map(|k| [0., 0.25, 1., 2.][k % 4]).collect();
        let attributes: Vec<_> = (0..positions.len())
            .flat_map(|i| {
                (0..stride).map(move |k| {
                    if k == width {
                        f32::NAN
                    } else {
                        ((i * 3 + k) % 17) as f32 / 8.
                    }
                })
            })
            .collect();
        let flags: Vec<_> = (0..positions.len())
            .map(|i| {
                if i % 13 == 0 {
                    VertexFlags::LOCK
                } else {
                    VertexFlags::EMPTY
                }
            })
            .collect();
        for options in [
            SimplifyOptions::EMPTY,
            SimplifyOptions::SPARSE,
            SimplifyOptions::PERMISSIVE,
        ] {
            let run = |limits| {
                let mut p = positions.clone();
                let mut a = attributes.clone();
                let mut ib = indices.clone();
                let mut ws = Workspace::new(limits);
                let mut av = if width == 0 {
                    None
                } else {
                    Some(
                        AttributesMut::from_interleaved(&mut a, p.len(), width, stride, 0).unwrap(),
                    )
                };
                let result = simplify_with_update(
                    &mut ib,
                    &mut PositionsMut::from_packed(&mut p),
                    av.as_mut(),
                    &weights,
                    Some(&flags),
                    SimplifySettings {
                        target_index_count: indices.len() / 2,
                        target_error: 0.5,
                        options,
                    },
                    &mut ws,
                )
                .map(|r| (r.index_count, r.error.to_bits()));
                (
                    result,
                    ib,
                    p.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    ws.usage(),
                )
            };
            let ample = run(Limits::default());
            assert!(ample.0.is_ok());
            let tight = Limits {
                max_bytes: ample.4.bytes,
                max_work: ample.4.work,
            };
            assert_eq!(run(tight), ample, "width {width}, options {options:?}");
            assert_eq!(
                run(Limits {
                    max_work: tight.max_work - 1,
                    ..tight
                })
                .0,
                Err(Error::LimitExceeded)
            );
        }
    }
}
