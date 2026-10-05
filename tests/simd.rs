#![cfg(all(feature = "simd", feature = "parity-internals"))]
use meshoptimizer_rs::{codec::*, Error, Workspace};

fn levels() -> Vec<Level> {
    let detected = detected_level();
    [
        Level::Scalar,
        Level::Sse2,
        Level::Ssse3,
        Level::Sse41,
        Level::Neon,
        Level::Wasm,
    ]
    .into_iter()
    .filter(|&l| with_level(l, || ()).is_ok())
    .inspect(|l| assert!(*l <= detected))
    .collect()
}
#[test]
fn lowering_is_checked_nested_and_unwind_safe() {
    for l in [
        Level::Scalar,
        Level::Sse2,
        Level::Ssse3,
        Level::Sse41,
        Level::Neon,
        Level::Wasm,
    ] {
        if !levels().contains(&l) {
            assert_eq!(with_level(l, || ()), Err(Error::InvalidParameter));
        }
    }
    with_level(Level::Scalar, || {
        with_level(Level::Scalar, || ()).unwrap();
    })
    .unwrap();
}
fn random(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}
#[test]
fn filters_match_scalar_with_extreme_words_and_tails() {
    let mut seed = 20261005u32;
    for kind in 1..=4 {
        for stride in if kind == 2 {
            vec![8]
        } else if kind == 3 {
            vec![4, 12, 32]
        } else {
            vec![4, 8]
        } {
            for count in [0, 1, 3, 4, 5, 15, 16, 17, 65] {
                let mut source = vec![0; count * stride + 19];
                for b in &mut source {
                    *b = random(&mut seed) as u8;
                }
                for edge in 0..8 {
                    if edge < 5 {
                        for (i, b) in source.iter_mut().enumerate().take(count * stride) {
                            *b =
                                [0u8, 1, 127, 128, 255][edge].wrapping_add((i % stride == 3) as u8);
                        }
                    }
                    let run = |l| {
                        with_level(l, || {
                            let mut output = source.clone();
                            let mut w = Workspace::default();
                            let result = match kind {
                                1 => decode_filter_oct(&mut output, count, stride, &mut w),
                                2 => decode_filter_quat(&mut output, count, stride, &mut w),
                                3 => decode_filter_exp(&mut output, count, stride, &mut w),
                                _ => decode_filter_color(&mut output, count, stride, &mut w),
                            };
                            assert_eq!(&output[count * stride..], &source[count * stride..]);
                            (result, output)
                        })
                        .unwrap()
                    };
                    let expected = run(Level::Scalar);
                    for l in levels() {
                        let actual = run(l);
                        assert_eq!(actual.0, expected.0, "{kind}/{stride}/{count}/{edge}/{l:?}");
                        if actual.0.is_ok() {
                            assert_eq!(
                                actual.1, expected.1,
                                "{kind}/{stride}/{count}/{edge}/{l:?}"
                            );
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn vertex_versions_channels_boundaries_and_malformed_results() {
    let mut seed = 321u32;
    // Miri executes both group boundaries and the 256-vertex block boundary.
    // Native tests and differential native/wasm corpora keep the larger matrix.
    let counts: &[usize] = if cfg!(miri) {
        &[0, 1, 15, 16, 17, 63, 64, 65, 257]
    } else {
        &[0, 1, 15, 16, 17, 63, 64, 65, 255, 256, 257, 513]
    };
    let strides: &[usize] = if cfg!(miri) {
        &[4, 12]
    } else {
        &[4, 12, 32, 256]
    };
    for &count in counts {
        for &stride in strides {
            let vertices: Vec<u8> = (0..count * stride)
                .map(|_| random(&mut seed) as u8)
                .collect();
            for version in 0..=1 {
                let mut w = Workspace::default();
                let encoded = encode_vertex_buffer(
                    &vertices,
                    count,
                    stride,
                    VertexEncoding::new(version, 2).unwrap(),
                    &mut w,
                )
                .unwrap();
                for length in [
                    0,
                    1,
                    encoded.len() / 2,
                    encoded.len().saturating_sub(1),
                    encoded.len(),
                ] {
                    let run = |l| {
                        with_level(l, || {
                            let mut w = Workspace::default();
                            let mut out = vec![0xcc; count * stride + 7];
                            let r = decode_vertex_buffer_into(
                                &mut out,
                                count,
                                stride,
                                &encoded[..length],
                                &mut w,
                            );
                            assert_eq!(&out[count * stride..], &[0xcc; 7]);
                            (r, out)
                        })
                        .unwrap()
                    };
                    let expected = run(Level::Scalar);
                    for l in levels() {
                        let actual = run(l);
                        assert_eq!(actual.0, expected.0);
                        if actual.0.is_ok() {
                            assert_eq!(actual.1, expected.1);
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn meshlet_controls_widths_partial_groups_and_tail_preservation() {
    let mut w = Workspace::default();
    for count in [0, 1, 3, 4, 5, 63, 64, 65, 255, 256] {
        let vertices: Vec<u32> = (0..count)
            .map(|i| (i as u32).wrapping_mul(0x100001))
            .collect();
        let triangles = vec![0u8; count * 3];
        let encoded = encode_meshlet(&vertices, &triangles, &mut w).unwrap();
        for vs in [2, 4] {
            for ts in [3, 4] {
                let run = |l| {
                    with_level(l, || {
                        let mut w = Workspace::default();
                        let mut v = vec![0xcc; count * vs + 7];
                        let mut t = vec![0xcc; count * ts + 7];
                        let r = decode_meshlet_into(
                            &mut v, count, vs, &mut t, count, ts, &encoded, &mut w,
                        );
                        assert_eq!(&v[count * vs..], &[0xcc; 7]);
                        assert_eq!(&t[count * ts..], &[0xcc; 7]);
                        (r, v, t)
                    })
                    .unwrap()
                };
                let expected = run(Level::Scalar);
                for l in levels() {
                    assert_eq!(run(l), expected);
                }
            }
        }
    }
}

#[test]
fn meshlet_all_pair_codes_odd_tails_and_counter_wrap() {
    let run = |source: &[u8], vc: usize, tc: usize, l| {
        with_level(l, || {
            let mut w = Workspace::default();
            let mut rows = Vec::new();
            for vs in [2, 4] {
                for ts in [3, 4] {
                    let mut v = vec![0xcc; vc * vs + 7];
                    let mut t = vec![0xcc; tc * ts + 7];
                    let r = decode_meshlet_into(&mut v, vc, vs, &mut t, tc, ts, source, &mut w);
                    assert_eq!(&v[vc * vs..], &[0xcc; 7]);
                    assert_eq!(&t[tc * ts..], &[0xcc; 7]);
                    rows.push((
                        r,
                        if r.is_ok() { v } else { Vec::new() },
                        if r.is_ok() { t } else { Vec::new() },
                    ));
                }
            }
            let mut v = vec![0xcccccccc; vc + 7];
            let mut t = vec![0xcccccccc; tc + 7];
            let r = decode_meshlet_raw_into(&mut v, vc, &mut t, tc, source, &mut w);
            assert_eq!(&v[vc..], &[0xcccccccc; 7]);
            assert_eq!(&t[tc..], &[0xcccccccc; 7]);
            (
                rows,
                r,
                if r.is_ok() { v } else { Vec::new() },
                if r.is_ok() { t } else { Vec::new() },
            )
        })
        .unwrap()
    };
    let extra = |n: u8| {
        if n < 12 {
            usize::from(n & 1)
        } else {
            usize::from(n - 12)
        }
    };
    // Miri covers every individual nibble and mixed reuse/restart ordering;
    // the ordinary test covers all 256 paired codes, both odd/even tails.
    let codes: Vec<u8> = if cfg!(miri) {
        (0..16)
            .map(|i| i * 17)
            .chain([0x0f, 0xf0, 0xce, 0xec, 0xbd, 0xdb])
            .collect()
    } else {
        (0..=255).collect()
    };
    for code in codes {
        for tc in [1, 2] {
            let mut source = vec![7; 16];
            let n = extra(code & 15) + if tc == 2 { extra(code >> 4) } else { 0 };
            source.extend((0..n).map(|i| (i * 71 + 19) as u8));
            source.extend([0; 14]);
            source.extend([255, code]);
            for len in [source.len() - 1, source.len()] {
                let expected = run(&source[..len], 4, tc, Level::Scalar);
                for l in levels() {
                    assert_eq!(
                        run(&source[..len], 4, tc, l),
                        expected,
                        "{code}/{tc}/{len}/{l:?}"
                    );
                }
            }
        }
    }
    for tc in [85usize, 86, 255, 256] {
        let codes = tc.div_ceil(2);
        let mut source = vec![0; 16usize.saturating_sub(1 + codes)];
        source.push(0);
        source.extend(vec![0xcc; codes]);
        let expected = run(&source, 1, tc, Level::Scalar);
        for l in levels() {
            assert_eq!(run(&source, 1, tc, l), expected, "counter/{tc}/{l:?}");
        }
    }
}
