use meshoptimizer_rs::*;

#[test]
fn empty_meshes_succeed() {
    let mut ws = Workspace::default();
    assert_eq!(optimize_vertex_cache(&[], 0, &mut ws), Ok(vec![]));
    assert_eq!(
        optimize_overdraw(&[], Positions::from_packed(&[]), 1.05, &mut ws),
        Ok(vec![])
    );
}

#[test]
fn cache_preserves_triangle_corners() {
    let indices = [0, 1, 2, 4, 5, 6, 2, 1, 3];
    let result = optimize_vertex_cache(&indices, 7, &mut Workspace::default()).unwrap();
    let mut before: Vec<_> = indices
        .as_chunks::<3>()
        .0
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let mut after: Vec<_> = result
        .as_chunks::<3>()
        .0
        .iter()
        .map(|x| x.to_vec())
        .collect();
    before.sort();
    after.sort();
    assert_eq!(before, after);
}

#[test]
fn degenerate_triangles_remain_valid() {
    let input = [0, 0, 0, 0, 1, 1, 1, 0, 1];
    let p = [[0.; 3]; 2];
    let mut ws = Workspace::default();
    let cached = optimize_vertex_cache(&input, 2, &mut ws).unwrap();
    assert_eq!(
        optimize_overdraw(&cached, Positions::from_packed(&p), 1.05, &mut ws)
            .unwrap()
            .len(),
        input.len()
    );
}

#[test]
fn topology_is_checked() {
    assert_eq!(
        optimize_vertex_cache(&[0, 1], 2, &mut Workspace::default()),
        Err(Error::InvalidTopology)
    );
}

#[test]
fn indices_are_checked() {
    assert_eq!(
        optimize_vertex_cache(&[0, 1, 2], 2, &mut Workspace::default()),
        Err(Error::IndexOutOfBounds)
    );
}

#[test]
fn finite_thresholds_below_one_are_accepted() {
    let p = [[0.; 3]; 1];
    assert!(optimize_overdraw(
        &[0, 0, 0],
        Positions::from_packed(&p),
        -1.,
        &mut Workspace::default()
    )
    .is_ok());
}

#[test]
fn nonfinite_threshold_is_rejected() {
    assert_eq!(
        optimize_overdraw(
            &[],
            Positions::from_packed(&[]),
            f32::NAN,
            &mut Workspace::default()
        ),
        Err(Error::InvalidParameter)
    );
}

#[test]
fn unused_nonfinite_geometry_is_rejected() {
    let p = [[0.; 3], [f32::INFINITY; 3]];
    assert_eq!(
        optimize_overdraw(
            &[0, 0, 0],
            Positions::from_packed(&p),
            1.,
            &mut Workspace::default()
        ),
        Err(Error::NumericalFailure)
    );
}

#[test]
fn intermediate_overflow_is_rejected() {
    let p = [[f32::MAX; 3]; 2];
    assert_eq!(
        optimize_overdraw(
            &[0, 0, 1],
            Positions::from_packed(&p),
            1.,
            &mut Workspace::default()
        ),
        Err(Error::NumericalFailure)
    );
}

#[test]
fn destination_tail_is_untouched() {
    let mut dest = [99; 4];
    optimize_vertex_cache_into(&mut dest, &[0, 1, 2], 3, &mut Workspace::default()).unwrap();
    assert_eq!(dest, [0, 1, 2, 99]);
}

#[test]
fn short_destination_is_unchanged() {
    let mut dest = [99; 2];
    assert_eq!(
        optimize_vertex_cache_into(&mut dest, &[0, 1, 2], 3, &mut Workspace::default()),
        Err(Error::BufferTooSmall)
    );
    assert_eq!(dest, [99; 2]);
}

#[test]
fn in_place_errors_are_atomic() {
    let mut indices = [0, 1, 2];
    let mut ws = Workspace::new(Limits {
        max_bytes: 1024,
        max_work: 10,
    });
    assert_eq!(
        optimize_vertex_cache_in_place(&mut indices, 3, &mut ws),
        Err(Error::LimitExceeded)
    );
    assert_eq!(indices, [0, 1, 2]);
}

#[test]
fn storage_limit_includes_output() {
    // Cache heap scratch: 12*3 + 4*3 + 5*1 = 53 bytes.
    let mut ws = Workspace::new(Limits {
        max_bytes: 53,
        max_work: 1000,
    });
    assert_eq!(
        optimize_vertex_cache(&[0, 1, 2], 3, &mut ws),
        Err(Error::LimitExceeded)
    );
    let mut dest = [0; 3];
    optimize_vertex_cache_into(&mut dest, &[0, 1, 2], 3, &mut ws).unwrap();
    assert_eq!(ws.usage().bytes, 53);
}

#[test]
fn counted_cache_work_has_an_exact_boundary() {
    let mut ws = Workspace::default();
    optimize_vertex_cache(&[0, 1, 2], 3, &mut ws).unwrap();
    let required = ws.usage().work;
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: required - 1,
    });
    assert_eq!(
        optimize_vertex_cache(&[0, 1, 2], 3, &mut ws),
        Err(Error::LimitExceeded)
    );
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: required,
    });
    assert!(optimize_vertex_cache(&[0, 1, 2], 3, &mut ws).is_ok());
}

#[test]
fn counted_overdraw_work_has_an_exact_boundary() {
    let p = [[0.; 3]; 3];
    let mut ws = Workspace::default();
    optimize_overdraw(&[0, 1, 2], Positions::from_packed(&p), 1.05, &mut ws).unwrap();
    let required = ws.usage().work;
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: required - 1,
    });
    assert_eq!(
        optimize_overdraw(&[0, 1, 2], Positions::from_packed(&p), 1.05, &mut ws),
        Err(Error::LimitExceeded)
    );
    ws.set_limits(Limits {
        max_bytes: 1024,
        max_work: required,
    });
    assert!(optimize_overdraw(&[0, 1, 2], Positions::from_packed(&p), 1.05, &mut ws).is_ok());
}

#[test]
fn platform_size_overflow_is_checked_without_allocation() {
    let data = [0.; 3];
    assert!(matches!(
        Positions::from_interleaved(&data, usize::MAX, 64, 0),
        Err(Error::SizeOverflow)
    ));
    assert_eq!(
        optimize_vertex_cache(&[], usize::MAX, &mut Workspace::default()),
        Err(Error::SizeOverflow)
    );
}

#[test]
fn lowering_limits_releases_retained_scratch() {
    let mut ws = Workspace::default();
    optimize_vertex_cache(&[0, 1, 2], 3, &mut ws).unwrap();
    ws.set_limits(Limits {
        max_bytes: 0,
        max_work: 0,
    });
    assert_eq!(optimize_vertex_cache(&[], 0, &mut ws), Ok(vec![]));
    assert_eq!(ws.usage(), Usage::default());
}

#[test]
fn flags_preserve_all_combinations_and_reject_unknown_bits() {
    assert_eq!(
        (VertexFlags::LOCK | VertexFlags::PROTECT | VertexFlags::PRIORITY).bits(),
        7
    );
    for bits in 0..8 {
        assert_eq!(VertexFlags::from_bits(bits).unwrap().bits(), bits);
    }
    assert_eq!(VertexFlags::from_bits(8), Err(Error::UnknownFlags));
}

#[test]
fn optional_flags_require_one_entry_per_vertex() {
    assert_eq!(
        validate_vertex_flags(Some(&[VertexFlags::EMPTY]), 2),
        Err(Error::InvalidLayout)
    );
    assert_eq!(validate_vertex_flags(None, 2), Ok(()));
}

#[test]
fn strided_positions_honor_offset_and_last_record_extent() {
    let p = Positions::from_interleaved(&[99., 1., 2., 3., 99., 4., 5., 6.], 2, 4, 1).unwrap();
    assert_eq!(p.get(1), Some([4., 5., 6.]));
    assert_eq!(p.get(2), None);
}

#[test]
fn byte_positions_honor_unaligned_offsets_and_endianness() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut bytes = vec![99];
        for x in [1f32, -0., 3.] {
            bytes.extend_from_slice(&match order {
                ByteOrder::LittleEndian => x.to_le_bytes(),
                ByteOrder::BigEndian => x.to_be_bytes(),
            });
        }
        let p = Positions::from_bytes(&bytes, 1, 12, 1, order)
            .unwrap()
            .get(0)
            .unwrap();
        assert_eq!(p.map(f32::to_bits), [1f32, -0., 3.].map(f32::to_bits));
    }
}

#[test]
fn layouts_enforce_upstream_stride_bounds() {
    assert!(matches!(
        Positions::from_interleaved(&[0.; 3], 1, 2, 0),
        Err(Error::InvalidLayout)
    ));
    assert!(matches!(
        Positions::from_bytes(&[0; 12], 1, 13, 0, ByteOrder::LittleEndian),
        Err(Error::InvalidLayout)
    ));
    assert!(matches!(
        Positions::from_interleaved(&[0.; 3], 1, 65, 0),
        Err(Error::InvalidLayout)
    ));
}

#[test]
fn attributes_enforce_width_and_support_zero_components() {
    assert!(matches!(
        Attributes::from_interleaved(&[0.; 33], 1, 33, 33, 0),
        Err(Error::InvalidLayout)
    ));
    let a = Attributes::from_interleaved(&[], 2, 0, 0, 0).unwrap();
    assert_eq!(a.components(), 0);
    assert_eq!(a.len(), 2);
    assert_eq!(a.get(0, 0), None);
}

#[test]
fn overdraw_destination_tail_is_untouched() {
    let mut destination = [99; 4];
    optimize_overdraw_into(
        &mut destination,
        &[0, 1, 2],
        Positions::from_packed(&[[0.; 3]; 3]),
        1.05,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(destination, [0, 1, 2, 99]);
}

#[test]
fn overdraw_in_place_numerical_error_is_atomic() {
    let mut indices = [0, 1, 2];
    assert_eq!(
        optimize_overdraw_in_place(
            &mut indices,
            Positions::from_packed(&[[f32::MAX; 3]; 3]),
            1.05,
            &mut Workspace::default(),
        ),
        Err(Error::NumericalFailure)
    );
    assert_eq!(indices, [0, 1, 2]);
}

#[test]
fn strided_and_byte_positions_match_packed_overdraw() {
    let packed = [[0., 0., 1.], [1., 0., 0.], [0., 1., 0.], [0., 0., -1.]];
    let indices = [0, 1, 2, 3, 2, 1];
    let mut interleaved = Vec::new();
    let mut bytes = vec![99];
    for point in packed {
        interleaved.push(99.);
        interleaved.extend_from_slice(&point);
        for x in point {
            bytes.extend_from_slice(&f32::to_be_bytes(x));
        }
    }
    let mut ws = Workspace::default();
    let expected =
        optimize_overdraw(&indices, Positions::from_packed(&packed), 1.05, &mut ws).unwrap();
    for positions in [
        Positions::from_interleaved(&interleaved, 4, 4, 1).unwrap(),
        Positions::from_bytes(&bytes, 4, 12, 1, ByteOrder::BigEndian).unwrap(),
    ] {
        assert_eq!(
            optimize_overdraw(&indices, positions, 1.05, &mut ws).unwrap(),
            expected
        );
    }
}

fn simplify_settings(target: usize) -> SimplifySettings {
    SimplifySettings {
        target_index_count: target,
        target_error: 1.0,
        options: SimplifyOptions::EMPTY,
    }
}
fn simplifier_fixture() -> ([[f32; 3]; 6], [u32; 12]) {
    (
        [
            [0., 4., 0.],
            [0., 1., 0.],
            [2., 2., 0.],
            [0., 0., 0.],
            [1., 0., 0.],
            [4., 0., 0.],
        ],
        [0, 2, 1, 1, 2, 3, 3, 2, 4, 2, 5, 4],
    )
}
#[test]
fn simplifier_matches_upstream_strip() {
    let (p, ib) = simplifier_fixture();
    let mut settings = simplify_settings(3);
    settings.target_error = 0.01;
    let out = simplify(
        &ib,
        Positions::from_packed(&p),
        settings,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(out.indices, [0, 5, 3]);
    assert!(out.error < 1e-4);
}
#[test]
fn simplifier_empty_has_zero_error() {
    let out = simplify(
        &[],
        Positions::from_packed(&[]),
        simplify_settings(0),
        &mut Workspace::default(),
    )
    .unwrap();
    assert!(out.indices.is_empty());
    assert_eq!(out.error.to_bits(), 0);
}
#[test]
fn scale_has_no_epsilon_clamp() {
    assert_eq!(simplify_scale(Positions::from_packed(&[])), Ok(0.));
    assert_eq!(
        simplify_scale(Positions::from_packed(&[[1., 2., 3.]; 2])),
        Ok(0.)
    );
    assert_eq!(
        simplify_scale(Positions::from_packed(&[[0., 0., 0.], [1., 2., 3.]])),
        Ok(3.)
    );
}
#[test]
fn simplifier_accepts_partial_triangle_target() {
    let (p, ib) = simplifier_fixture();
    assert!(simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(5),
        &mut Workspace::default()
    )
    .is_ok());
}
#[test]
fn simplifier_checks_target_and_error() {
    let (p, ib) = simplifier_fixture();
    let mut s = simplify_settings(13);
    assert!(matches!(
        simplify(
            &ib,
            Positions::from_packed(&p),
            s,
            &mut Workspace::default()
        ),
        Err(Error::InvalidParameter)
    ));
    for e in [-1., f32::INFINITY, f32::NAN] {
        s.target_index_count = 3;
        s.target_error = e;
        assert!(matches!(
            simplify(
                &ib,
                Positions::from_packed(&p),
                s,
                &mut Workspace::default()
            ),
            Err(Error::InvalidParameter)
        ));
    }
}
#[test]
fn simplifier_rejects_nonfinite_unused_geometry() {
    assert!(simplify(
        &[],
        Positions::from_packed(&[[f32::NAN, 0., 0.]]),
        simplify_settings(0),
        &mut Workspace::default()
    )
    .is_err());
}
#[test]
fn simplifier_reports_overflowing_extent() {
    assert_eq!(
        simplify_scale(Positions::from_packed(&[
            [-f32::MAX, 0., 0.],
            [f32::MAX, 0., 0.]
        ])),
        Err(Error::NumericalFailure)
    );
}
#[test]
fn simplifier_locks_preserve_vertices() {
    let (p, ib) = simplifier_fixture();
    let a = Attributes::from_interleaved(&[], 6, 0, 0, 0).unwrap();
    let out = simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[],
        Some(&[VertexFlags::LOCK; 6]),
        simplify_settings(0),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(out.indices, ib);
}
#[test]
fn simplifier_zero_weights_match_position_only() {
    let (p, ib) = simplifier_fixture();
    let a = Attributes::from_interleaved(&[100.; 18], 6, 3, 3, 0).unwrap();
    let mut ws = Workspace::default();
    let x = simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    let y = simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[0.; 3],
        None,
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    assert_eq!(x.indices, y.indices);
    assert_eq!(x.error.to_bits(), y.error.to_bits());
}
#[test]
fn simplifier_validates_attribute_dimensions_and_values() {
    let (p, ib) = simplifier_fixture();
    let a = Attributes::from_interleaved(&[1.; 18], 6, 3, 3, 0).unwrap();
    for weights in [&[1., 1.][..], &[-1., 1., 1.], &[f32::NAN, 1., 1.]] {
        assert!(simplify_with_attributes(
            &ib,
            Positions::from_packed(&p),
            a,
            weights,
            None,
            simplify_settings(3),
            &mut Workspace::default()
        )
        .is_err());
    }
    let a = Attributes::from_interleaved(&[f32::NAN; 6], 6, 1, 1, 0).unwrap();
    assert!(simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[0.],
        None,
        simplify_settings(3),
        &mut Workspace::default()
    )
    .is_err());
}
#[test]
fn simplifier_validates_flag_length() {
    let (p, ib) = simplifier_fixture();
    let a = Attributes::from_interleaved(&[], 6, 0, 0, 0).unwrap();
    assert!(simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[],
        Some(&[VertexFlags::LOCK; 5]),
        simplify_settings(3),
        &mut Workspace::default()
    )
    .is_err());
}
#[test]
fn simplifier_into_matches_allocating_and_preserves_tail() {
    let (p, ib) = simplifier_fixture();
    let mut out = [u32::MAX; 15];
    let mut ws = Workspace::default();
    let a = simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    let b = simplify_into(
        &mut out,
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    assert_eq!(&out[..b.index_count], a.indices);
    assert_eq!(b.error.to_bits(), a.error.to_bits());
    assert_eq!(&out[12..], &[u32::MAX; 3]);
}
#[test]
fn simplifier_exact_work_budget_is_enforced() {
    let (p, ib) = simplifier_fixture();
    let mut ws = Workspace::default();
    simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    let used = ws.usage();
    ws.set_limits(Limits {
        max_bytes: used.bytes,
        max_work: used.work,
    });
    assert!(simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws
    )
    .is_ok());
    ws.set_limits(Limits {
        max_bytes: used.bytes,
        max_work: used.work - 1,
    });
    assert!(matches!(
        simplify(
            &ib,
            Positions::from_packed(&p),
            simplify_settings(3),
            &mut ws
        ),
        Err(Error::LimitExceeded)
    ));
}
#[test]
fn simplifier_memory_budget_includes_scratch_and_output() {
    let (p, ib) = simplifier_fixture();
    let mut ws = Workspace::default();
    simplify(
        &ib,
        Positions::from_packed(&p),
        simplify_settings(3),
        &mut ws,
    )
    .unwrap();
    let used = ws.usage();
    ws.set_limits(Limits {
        max_bytes: used.bytes - 1,
        max_work: u64::MAX,
    });
    assert!(matches!(
        simplify(
            &ib,
            Positions::from_packed(&p),
            simplify_settings(3),
            &mut ws
        ),
        Err(Error::LimitExceeded)
    ));
}
#[test]
fn simplifier_rejects_unimplemented_option_bits() {
    assert!(SimplifyOptions::from_bits(2).is_err());
    assert_eq!(
        (SimplifyOptions::PERMISSIVE | SimplifyOptions::LOCK_BORDER).bits(),
        33
    );
}

#[test]
fn priority_increases_positional_cost() {
    let (p, ib) = simplifier_fixture();
    let a = Attributes::from_interleaved(&[], 6, 0, 0, 0).unwrap();
    let mut settings = simplify_settings(3);
    settings.target_error = 0.01;
    let out = simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[],
        Some(&[VertexFlags::PRIORITY; 6]),
        settings,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(out.indices, ib);
    assert_eq!(out.error.to_bits(), 0);
}
#[test]
fn protect_prevents_permissive_discontinuity_collapses() {
    let base = [
        [1., 0., 0.],
        [-1., 0., 0.],
        [0., 1., 0.],
        [0., -1., 0.],
        [0., 0., 1.],
        [0., 0., -1.],
    ];
    let faces = [
        0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 4, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3, 5,
    ];
    let p = faces.map(|i| base[i]);
    let ib: Vec<u32> = (0..24).collect();
    let a = Attributes::from_interleaved(&[], 24, 0, 0, 0).unwrap();
    let mut settings = simplify_settings(6);
    settings.options = SimplifyOptions::PERMISSIVE;
    let unprotected = simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[],
        None,
        settings,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(unprotected.indices, [9, 1, 11, 5, 16, 18]);
    assert_eq!(unprotected.error.to_bits(), 1053885933);
    let protected = simplify_with_attributes(
        &ib,
        Positions::from_packed(&p),
        a,
        &[],
        Some(&[VertexFlags::PROTECT; 24]),
        settings,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(protected.indices, ib);
    assert_eq!(protected.error.to_bits(), 0);
}
