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
        &[0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 257]
    } else {
        &[0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 513]
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
        for tc in [1, 2, 3, 4, 5] {
            let pair_codes = [code, code.rotate_left(4), code ^ 0x55];
            let mut source = vec![7; 16];
            let n: usize = (0..tc)
                .map(|i| {
                    let pair = pair_codes[i / 2];
                    extra((pair >> ((i % 2) * 4)) & 15)
                })
                .sum();
            source.extend((0..n).map(|i| (i * 71 + 19) as u8));
            source.extend([0; 14]);
            source.push(255);
            source.extend_from_slice(&pair_codes[..tc.div_ceil(2)]);
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

#[test]
fn color_alpha_conversion_bounds_and_wrap() {
    for stride in [4, 8] {
        for alphas in [
            0, 1, 2, 3, 4, 7, 8, 127, 128, 129, 255, 2047, 2048, 2049, 4094, 4095, 4096, 32767,
            32768, 65535,
        ]
        .into_iter()
        .map(|alpha| [alpha; 5])
        .chain([
            [127, 128, 129, 255, 0],
            [2047, 2048, 2049, 4095, 4096],
            [32767, 32768, 65535, 3, 4],
        ]) {
            let mut source = Vec::new();
            for ([y, co, cg], alpha) in [
                [0, 0, 0],
                [65535, 32767, -32768],
                [65535, -32768, -32768],
                [0, -32768, 32767],
                [128, 127, -128],
            ]
            .into_iter()
            .zip(alphas)
            {
                for c in [y, co, cg, alpha] {
                    if stride == 4 {
                        source.push(c as u8);
                    } else {
                        source.extend_from_slice(&(c as u16).to_le_bytes());
                    }
                }
            }
            let run = |level| {
                with_level(level, || {
                    let mut out = source.clone();
                    let status =
                        decode_filter_color(&mut out, 5, stride, &mut Workspace::default());
                    (status, out)
                })
                .unwrap()
            };
            let scalar = run(Level::Scalar);
            for level in levels() {
                let actual = run(level);
                assert_eq!(actual.0, scalar.0, "{stride}/{alphas:?}/{level:?}");
                if scalar.0.is_ok() {
                    assert_eq!(actual.1, scalar.1);
                }
            }
        }
    }
}

#[test]
#[cfg(not(miri))]
fn exp_dispatch_boundary_preserves_words_and_tail() {
    for stride in [4, 12, 32] {
        let boundary = 4 * 1024 * 1024 / stride;
        for count in [17, 4097, boundary - 1, boundary, boundary + 1] {
            let words = [
                0, 0xffffffff, 0x80000000, 0x7fffffff, 0x01800000, 0x7f000001, 0xff123456,
                0x00ffffff,
            ];
            let mut source = vec![0xa5; count * stride + 11];
            for (i, w) in source[..count * stride].chunks_exact_mut(4).enumerate() {
                w.copy_from_slice(&u32::to_le_bytes(words[i % words.len()]));
            }
            let run = |level| {
                with_level(level, || {
                    let mut out = source.clone();
                    let mut work = Workspace::default();
                    decode_filter_exp(&mut out, count, stride, &mut work).unwrap();
                    assert_eq!(&out[count * stride..], &source[count * stride..]);
                    (out, work.usage())
                })
                .unwrap()
            };
            let expected = run(Level::Scalar);
            for level in levels() {
                assert_eq!(run(level), expected);
            }
        }
    }
}

#[test]
fn repeated_filter_runs_preserve_alpha_and_changed_suffix() {
    for (kind, stride) in [(1, 4), (1, 8), (2, 8)] {
        for count in [4, 5, 17, 129] {
            let mut source = vec![0xa5; count * stride + 13];
            for i in 0..count {
                let e = &mut source[i * stride..(i + 1) * stride];
                let changed = (i % 19 == 7) as i16;
                if stride == 4 {
                    e.copy_from_slice(&[changed as u8, 0, 127, i as u8]);
                } else {
                    for (out, word) in e.chunks_exact_mut(2).zip([
                        changed,
                        0,
                        if kind == 1 { 32767 } else { 0 },
                        if kind == 1 { i as i16 } else { 16383 },
                    ]) {
                        out.copy_from_slice(&word.to_le_bytes());
                    }
                }
            }
            let run = |level| {
                with_level(level, || {
                    let mut out = source.clone();
                    let mut w = Workspace::default();
                    let status = if kind == 1 {
                        decode_filter_oct(&mut out, count, stride, &mut w)
                    } else {
                        decode_filter_quat(&mut out, count, stride, &mut w)
                    };
                    assert_eq!(&out[count * stride..], &source[count * stride..]);
                    (status, out, w.usage())
                })
                .unwrap()
            };
            let expected = run(Level::Scalar);
            for level in levels() {
                assert_eq!(run(level), expected);
            }
        }
    }
}
