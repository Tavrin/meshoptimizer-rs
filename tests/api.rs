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
