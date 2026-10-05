use meshoptimizer_rs::{codec::*, Error, Limits, Workspace};

#[test]
fn versions_are_header_inspection_and_ext_remains_narrow() {
    for (header, version) in [(0xa0, 0), (0xa1, 1)] {
        assert_eq!(decode_vertex_version(&[header]), Ok(version));
    }
    for (header, version) in [(0xd0, 0), (0xd1, 1), (0xe0, 0), (0xe1, 1)] {
        assert_eq!(decode_index_version(&[header]), Ok(version));
    }
    assert_eq!(decode_vertex_version(&[]), Err(Error::InvalidStream));
    assert_eq!(
        decode_vertex_version(&[0xa2]),
        Err(Error::UnsupportedVersion)
    );
    assert_eq!(decode_index_version(&[0xa0]), Err(Error::InvalidStream));
    let mut w = Workspace::default();
    assert_eq!(
        decode_buffer_view(Mode::Attributes, Filter::None, 1, 4, &[0xa1], &mut w),
        Err(Error::UnsupportedVersion)
    );
}

#[test]
fn ext_rules_and_parent_layout() {
    assert_eq!(Mode::parse("ATTRIBUTES"), Ok(Mode::Attributes));
    assert_eq!(Filter::parse("COLOR"), Err(Error::InvalidParameter));
    for stride in 0..=260 {
        assert_eq!(
            BufferView::new(Mode::Attributes, Filter::None, 1, stride).is_ok(),
            (4..=256).contains(&stride) && stride % 4 == 0
        );
        assert_eq!(
            BufferView::new(Mode::Indices, Filter::None, 1, stride).is_ok(),
            stride == 2 || stride == 4
        );
    }
    assert_eq!(
        BufferView::new(Mode::Triangles, Filter::None, 1, 4),
        Err(Error::InvalidTopology)
    );
    assert_eq!(
        BufferView::new(Mode::Indices, Filter::Octahedral, 1, 4),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        BufferView::new(Mode::Attributes, Filter::Quaternion, 1, 4),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        BufferView::new(Mode::Attributes, Filter::None, 0, 4),
        Err(Error::InvalidParameter)
    );
    let view = BufferView::new(Mode::Attributes, Filter::Exponential, 2, 12).unwrap();
    assert_eq!(view.byte_length(), 24);
    assert_eq!(view.validate_parent(24, Some(12)), Ok(()));
    assert_eq!(view.validate_parent(24, None), Ok(()));
    assert_eq!(view.validate_parent(23, None), Err(Error::InvalidLayout));
    assert_eq!(view.validate_parent(24, Some(4)), Err(Error::InvalidLayout));
}

#[test]
fn sequence_fixture_tail_truncation_and_raw_empty() {
    let source = [
        0xd1, 0x00, 0x04, 0xcd, 0x01, 0x04, 0x07, 0x98, 0x1f, 0, 0, 0, 0,
    ];
    let mut w = Workspace::default();
    let expected = [0u32, 1, 51, 2, 49, 1000]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect::<Vec<_>>();
    assert_eq!(
        decode_index_sequence(6, 4, &source, &mut w).unwrap(),
        expected
    );
    let mut out = [0x55; 32];
    decode_index_sequence_into(&mut out, 6, 4, &source, &mut w).unwrap();
    assert_eq!(&out[..24], expected);
    assert_eq!(&out[24..], &[0x55; 8]);
    for n in 0..source.len() {
        assert!(decode_index_sequence(6, 4, &source[..n], &mut w).is_err());
    }
    let mut extra = source.to_vec();
    extra.push(0);
    assert_eq!(
        decode_index_sequence(6, 4, &extra, &mut w),
        Err(Error::InvalidStream)
    );
    assert_eq!(
        decode_index_sequence(0, 4, &[0xd0, 0, 0, 0, 0], &mut w),
        Ok(vec![])
    );
    assert_eq!(
        decode_index_buffer(
            0,
            2,
            &[0xe1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &mut w
        ),
        Ok(vec![])
    );
    let mut empty = [0u8; 33];
    empty[0] = 0xa0;
    assert_eq!(decode_vertex_buffer(0, 4, &empty, &mut w), Ok(vec![]));
}

#[test]
fn scalar_filter_goldens_and_undefined_oct() {
    let mut w = Workspace::default();
    let mut oct = [
        0, 1, 127, 0, 0, 187, 127, 1, 255, 1, 127, 0, 14, 130, 127, 1,
    ];
    decode_filter_oct(&mut oct, 4, 4, &mut w).unwrap();
    assert_eq!(
        oct,
        [0, 1, 127, 0, 0, 159, 82, 1, 255, 1, 127, 0, 1, 130, 241, 1]
    );
    let mut quat = [
        0i16, 1, 0, 0x7fc, 0, 1870, 0, 0x7fd, 2017, 1, 0, 0x7fe, 14, 1300, 0, 0x7ff,
    ]
    .into_iter()
    .flat_map(i16::to_le_bytes)
    .collect::<Vec<_>>();
    decode_filter_quat(&mut quat, 4, 8, &mut w).unwrap();
    let expected = [
        32767i16, 0, 11, 0, 0, 25013, 0, 21166, 11, 0, 23504, 22830, 158, 14715, 0, 29277,
    ]
    .into_iter()
    .flat_map(i16::to_le_bytes)
    .collect::<Vec<_>>();
    assert_eq!(quat, expected);
    let mut exp = [0u32, 0xff000003, 0x02fffff7, 0xfe7fffff]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect::<Vec<_>>();
    decode_filter_exp(&mut exp, 4, 4, &mut w).unwrap();
    assert_eq!(
        exp,
        [0u32, 0x3fc00000, 0xc2100000, 0x49fffffe]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        decode_filter_oct(&mut [0; 4], 1, 4, &mut w),
        Err(Error::NumericalFailure)
    );
}

#[test]
fn resources_destination_and_narrowing() {
    let mut w = Workspace::new(Limits {
        max_bytes: 0,
        max_work: 100,
    });
    assert_eq!(
        decode_index_sequence(1, 4, &[0xd1, 0, 0, 0, 0, 0], &mut w),
        Err(Error::LimitExceeded)
    );
    let mut out = [0xcc; 8];
    decode_index_sequence_into(&mut out, 1, 4, &[0xd1, 0, 0, 0, 0, 0], &mut w).unwrap();
    assert_eq!(w.usage().bytes, 0);
    assert_eq!(w.usage().work, 10);
    assert_eq!(&out[4..], &[0xcc; 4]);
    w.set_limits(Limits {
        max_bytes: 100,
        max_work: 9,
    });
    out.fill(0xcc);
    assert_eq!(
        decode_index_sequence_into(&mut out, 1, 4, &[0xd1, 0, 0, 0, 0, 0], &mut w),
        Err(Error::LimitExceeded)
    );
    assert_eq!(out, [0xcc; 8]);
    assert_eq!(
        decode_index_sequence_into(&mut [], 1, 4, &[], &mut w),
        Err(Error::BufferTooSmall)
    );
    assert_eq!(
        decode_vertex_buffer(usize::MAX, 4, &[], &mut w),
        Err(Error::SizeOverflow)
    );
    // Upstream sequence narrowing wraps; it does not reject indices above u16.
    w.set_limits(Limits::default());
    let encoded = [0xd1, 0x80, 0x80, 0x10, 0, 0, 0, 0];
    assert_eq!(
        decode_index_sequence(1, 2, &encoded, &mut w).unwrap(),
        [0, 0]
    );
}

#[test]
fn arbitrary_malformed_bytes_never_panic() {
    let mut seed = 0x20261004u32;
    let mut w = Workspace::new(Limits {
        max_bytes: 65536,
        max_work: 65536,
    });
    for length in 0..256 {
        let mut data = vec![0; length];
        for b in &mut data {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            *b = (seed >> 24) as u8;
        }
        for count in [0, 1, 3, 17, 256, usize::MAX] {
            let mut destination = [0u8; 1024];
            let _ = decode_vertex_buffer_into(&mut destination, count, 4, &data, &mut w);
            let _ = decode_index_buffer_into(&mut destination, count, 2, &data, &mut w);
            let _ = decode_index_sequence_into(&mut destination, count, 4, &data, &mut w);
            let _ = decode_buffer_view_into(
                &mut destination,
                Mode::Attributes,
                Filter::Octahedral,
                count,
                4,
                &data,
                &mut w,
            );
        }
    }
}

#[test]
fn quaternion_rounding_uses_the_original_component_sign() {
    let mut data = [1i16, 1, 1, -1]
        .into_iter()
        .flat_map(i16::to_le_bytes)
        .collect::<Vec<_>>();
    decode_filter_quat(&mut data, 1, 8, &mut Workspace::default()).unwrap();
    let expected = [-23169i16, -23169, -23169, 0]
        .into_iter()
        .flat_map(i16::to_le_bytes)
        .collect::<Vec<_>>();
    assert_eq!(data, expected);
}

#[test]
fn decoder_accounts_retained_geometry_workspace_without_allocating_scratch() {
    let mut w = Workspace::default();
    let _ = meshoptimizer_rs::optimize_vertex_cache(&[0, 1, 2], 3, &mut w).unwrap();
    let mut output = [0u8; 4];
    let source = [0xd1, 0, 0, 0, 0, 0];
    decode_index_sequence_into(&mut output, 1, 4, &source, &mut w).unwrap();
    let retained = w.usage().bytes;
    assert!(retained > 0);
    decode_index_sequence(1, 4, &source, &mut w).unwrap();
    assert_eq!(w.usage().bytes, retained + 4);
    let _ = meshoptimizer_rs::optimize_vertex_cache(&[0, 1, 2], 3, &mut w).unwrap();
    w.clear();
    decode_index_sequence_into(&mut output, 1, 4, &source, &mut w).unwrap();
    assert_eq!(w.usage().bytes, 0);
}

#[test]
fn allocating_sequence_blocks_preserve_limits_headers_and_tails() {
    for count in [63, 64, 65, 127, 128, 129] {
        let indices: Vec<u32> = (0..count)
            .map(|i| match i % 3 {
                0 => u32::MAX.wrapping_sub(i as u32),
                1 => i as u32,
                _ => 70000 + i as u32,
            })
            .collect();
        for version in [0, 1] {
            let source = encode_index_sequence(
                &indices,
                IndexEncoding::new(version).unwrap(),
                &mut Workspace::default(),
            )
            .unwrap();
            for stride in [2, 4] {
                let bytes = count * stride;
                let mut w = Workspace::new(Limits {
                    max_bytes: bytes,
                    max_work: 1 << 34,
                });
                let owned = decode_index_sequence(count, stride, &source, &mut w).unwrap();
                assert_eq!(owned.len(), bytes);
                assert_eq!(owned.capacity(), bytes);
                assert_eq!(w.usage().bytes, bytes);
                let mut into = vec![0xa5; bytes + 7];
                decode_index_sequence_into(&mut into, count, stride, &source, &mut w).unwrap();
                assert_eq!(&into[..bytes], owned);
                assert_eq!(&into[bytes..], &[0xa5; 7]);
                let mut limited = Workspace::new(Limits {
                    max_bytes: bytes - 1,
                    max_work: 1 << 34,
                });
                assert_eq!(
                    decode_index_sequence(count, stride, &source, &mut limited),
                    Err(Error::LimitExceeded)
                );
                for header in [0, 0xe0, 0xd2] {
                    let mut bad = source.clone();
                    bad[0] = header;
                    assert_eq!(
                        decode_index_sequence(count, stride, &bad, &mut w).unwrap_err(),
                        decode_index_sequence_into(&mut into, count, stride, &bad, &mut w)
                            .unwrap_err()
                    );
                }
                let mut extra = source.clone();
                extra.push(0);
                assert_eq!(
                    decode_index_sequence(count, stride, &extra, &mut w),
                    Err(Error::InvalidStream)
                );
            }
        }
    }
}
