#![forbid(unsafe_code)]
use meshoptimizer_rs::{codec::*, Limits, Workspace};
pub fn exercise(entry: u8, data: &[u8]) {
    differential(entry, data);
    exercise_reference(entry, data);
}
fn exercise_reference(entry: u8, data: &[u8]) {
    if data.len() < 9 {
        return;
    }
    let count = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
    let stride = u16::from_le_bytes(data[4..6].try_into().unwrap()) as usize;
    let mode = match data[6] % 3 {
        0 => Mode::Attributes,
        1 => Mode::Triangles,
        _ => Mode::Indices,
    };
    let filter = match data[7] % 4 {
        0 => Filter::None,
        1 => Filter::Octahedral,
        2 => Filter::Quaternion,
        _ => Filter::Exponential,
    };
    let source = &data[9..];
    let mut ws = Workspace::new(Limits {
        max_bytes: 65536,
        max_work: 131072,
    });
    // Small limits retain oversized headers as errors instead of masking them.
    let len = usize::from(data[8]) * 256;
    let mut out = vec![0xcc; len];
    match entry {
        0 => {
            let _ = decode_vertex_buffer(count, stride, source, &mut ws);
        }
        1 => {
            let _ = decode_vertex_buffer_into(&mut out, count, stride, source, &mut ws);
        }
        2 => {
            let _ = decode_index_buffer(count, stride, source, &mut ws);
        }
        3 => {
            let _ = decode_index_buffer_into(&mut out, count, stride, source, &mut ws);
        }
        4 => {
            let _ = decode_index_sequence(count, stride, source, &mut ws);
        }
        5 => {
            let _ = decode_index_sequence_into(&mut out, count, stride, source, &mut ws);
        }
        6..=8 => {
            let n = source.len().min(out.len());
            out[..n].copy_from_slice(&source[..n]);
            match entry {
                6 => {
                    let _ = decode_filter_oct(&mut out, count, stride, &mut ws);
                }
                7 => {
                    let _ = decode_filter_quat(&mut out, count, stride, &mut ws);
                }
                _ => {
                    let _ = decode_filter_exp(&mut out, count, stride, &mut ws);
                }
            }
        }
        9 => {
            let _ = decode_vertex_version(source);
        }
        10 => {
            let _ = decode_index_version(source);
        }
        11 => {
            let _ = decode_buffer_view(mode, filter, count, stride, source, &mut ws);
        }
        12 => {
            let _ = decode_buffer_view_into(&mut out, mode, filter, count, stride, source, &mut ws);
        }
        _ => exercise04(
            entry, data[6], data[7], count, stride, source, &mut out, &mut ws,
        ),
    }
}

fn words(b: &[u8]) -> Vec<u32> {
    b.as_chunks::<4>()
        .0
        .iter()
        .map(|c| u32::from_le_bytes(*c))
        .collect()
}

fn rotations<T: PartialEq + Copy>(a: &[T], b: &[T]) -> bool {
    let (a, b) = (a.as_chunks::<3>().0, b.as_chunks::<3>().0);
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(&[x, y, z], d)| [[x, y, z], [y, z, x], [z, x, y]].contains(d))
}

/// 0.4 entry points. Encoders also assert their round trips, so a panic is a
/// parity defect as well as a robustness one.
#[allow(clippy::too_many_arguments)]
fn exercise04(
    entry: u8,
    a: u8,
    b: u8,
    count: usize,
    stride: usize,
    source: &[u8],
    out: &mut [u8],
    ws: &mut Workspace,
) {
    let version = a & 1;
    let level = (a >> 1) % 11;
    let mut w = Workspace::default();
    match entry {
        13 | 14 => {
            let Ok(e) = VertexEncoding::new(version, level) else {
                return;
            };
            let stride = stride % 260;
            let count = source.len().checked_div(stride).unwrap_or(0);
            let vertices = &source[..count * stride];
            let encoded = if entry == 13 {
                encode_vertex_buffer(vertices, count, stride, e, ws)
            } else {
                encode_vertex_buffer_into(out, vertices, count, stride, e, ws)
                    .map(|n| out[..n].to_vec())
            };
            if let Ok(encoded) = encoded {
                let decoded = decode_vertex_buffer(count, stride, &encoded, &mut w);
                assert_eq!(decoded.as_deref(), Ok(vertices));
            }
        }
        15..=18 => {
            let e = IndexEncoding::new(version).unwrap_or_default();
            let mut indices = words(source);
            let triangles = entry <= 16;
            if triangles {
                indices.truncate(indices.len() / 3 * 3);
            }
            let encoded = match entry {
                15 => encode_index_buffer(&indices, e, ws),
                16 => encode_index_buffer_into(out, &indices, e, ws).map(|n| out[..n].to_vec()),
                17 => encode_index_sequence(&indices, e, ws),
                _ => encode_index_sequence_into(out, &indices, e, ws).map(|n| out[..n].to_vec()),
            };
            if let Ok(encoded) = encoded {
                let n = indices.len();
                let decoded = if triangles {
                    decode_index_buffer(n, 4, &encoded, &mut w)
                } else {
                    decode_index_sequence(n, 4, &encoded, &mut w)
                };
                let decoded = words(&decoded.expect("encoded indices decode"));
                if triangles {
                    // u32::MAX equals upstream's empty-FIFO sentinel, so such
                    // lists are not lossless upstream either (D61).
                    if !indices.contains(&u32::MAX) {
                        assert!(rotations(&indices, &decoded));
                    }
                } else if indices.iter().all(|&v| v < 1 << 30) {
                    // Sequence codes drop bit 31 of large zigzag deltas, as upstream.
                    assert_eq!(decoded, indices);
                }
            }
        }
        19..=22 => {
            let floats: Vec<f32> = words(source).into_iter().map(f32::from_bits).collect();
            let stride = stride % 260;
            let bits = u32::from(b % 26);
            let per = if entry == 21 { stride / 4 } else { 4 };
            let count = floats.len().checked_div(per).unwrap_or(0);
            let data = &floats[..count * per];
            let encoded = match entry {
                19 => encode_filter_oct(count, stride, bits, data, ws),
                20 => encode_filter_quat(count, stride, bits, data, ws),
                21 => {
                    let mode = [
                        ExpMode::Separate,
                        ExpMode::SharedVector,
                        ExpMode::SharedComponent,
                        ExpMode::Clamped,
                    ][usize::from(a % 4)];
                    encode_filter_exp(count, stride, bits, data, mode, ws)
                }
                _ => encode_filter_color(count, stride, bits, data, ws),
            };
            if let Ok(mut encoded) = encoded {
                let decoded = match entry {
                    19 => decode_filter_oct(&mut encoded, count, stride, &mut w),
                    20 => decode_filter_quat(&mut encoded, count, stride, &mut w),
                    21 => decode_filter_exp(&mut encoded, count, stride, &mut w),
                    _ => decode_filter_color(&mut encoded, count, stride, &mut w),
                };
                // Encoded Exp and Color always decode; Oct may meet a zero vector.
                assert!(decoded.is_ok() || entry == 19);
            }
        }
        23 => {
            let n = source.len().min(out.len());
            out[..n].copy_from_slice(&source[..n]);
            let _ = decode_filter_color(out, count, stride, ws);
        }
        24 | 25 => {
            let split = (usize::from(a) * 4).min(source.len() / 4 * 4);
            let vertices = words(&source[..split]);
            let triangles = &source[split..];
            let triangles = &triangles[..triangles.len() / 3 * 3];
            let encoded = if entry == 24 {
                encode_meshlet(&vertices, triangles, ws)
            } else {
                encode_meshlet_into(out, &vertices, triangles, ws).map(|n| out[..n].to_vec())
            };
            if let Ok(encoded) = encoded {
                let tc = triangles.len() / 3;
                let d = decode_meshlet(vertices.len(), 4, tc, 3, &encoded, &mut w)
                    .expect("encoded meshlet decodes");
                assert_eq!(words(&d.vertices), vertices);
                assert!(rotations(triangles, &d.triangles));
            }
        }
        26 | 27 => {
            let vc = usize::from(a);
            let tc = (usize::from(b) + (count & 1) * 256).min(256);
            let vs = [2, 4][(count >> 1) & 1];
            let ts = [3, 4][(count >> 2) & 1];
            if entry == 26 {
                let _ = decode_meshlet(vc, vs, tc, ts, source, ws);
            } else {
                let (v, t) = out.split_at_mut(out.len() / 2);
                let _ = decode_meshlet_into(v, vc, vs, t, tc, ts, source, ws);
            }
        }
        28 | 29 => {
            let (vc, tc) = (usize::from(a), usize::from(b));
            if entry == 28 {
                let _ = decode_meshlet_raw(vc, tc, source, ws);
            } else {
                let mut v = [0u32; 256];
                let mut t = [0u32; 256];
                let _ = decode_meshlet_raw_into(&mut v, vc, &mut t, tc, source, ws);
            }
        }
        31 => {
            let floats: Vec<f32> = words(source).into_iter().map(f32::from_bits).collect();
            let (stride, bits) = (stride % 260, u32::from(b % 26));
            let per = if a % 4 == 2 { stride / 4 } else { 4 };
            let count = floats.len().checked_div(per).unwrap_or(0);
            let data = &floats[..count * per];
            let _ = match a % 4 {
                0 => encode_filter_oct_into(out, count, stride, bits, data, ws),
                1 => encode_filter_quat_into(out, count, stride, bits, data, ws),
                2 => encode_filter_exp_into(out, count, stride, bits, data, ExpMode::Clamped, ws),
                _ => encode_filter_color_into(out, count, stride, bits, data, ws),
            };
        }
        _ => {
            let _ = encode_vertex_buffer_bound(count, stride);
            let _ = encode_index_buffer_bound(count, stride.wrapping_mul(count));
            let _ = encode_index_sequence_bound(count, stride.wrapping_mul(count));
            let _ = encode_meshlet_bound(count, stride);
        }
    }
}

// All decoder targets compare the exact typed result and successful meaningful
// bytes under every level this process can execute. Each process covers all ISAs.
static LEVEL_COUNTS: [std::sync::atomic::AtomicU64; 6] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 6];
fn differential(entry: u8, data: &[u8]) {
    if data.len() < 9 || !matches!(entry, 0..=12 | 23 | 26..=29) {
        return;
    }
    let count = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
    let stride = u16::from_le_bytes(data[4..6].try_into().unwrap()) as usize;
    let source = &data[9..];
    let run = || {
        let mut ws = Workspace::new(Limits {
            max_bytes: 65536,
            max_work: 131072,
        });
        let mut out = vec![0xcc; usize::from(data[8]) * 256];
        let r = match entry {
            0 => decode_vertex_buffer(count, stride, source, &mut ws),
            1 => decode_vertex_buffer_into(&mut out, count, stride, source, &mut ws).map(|()| out),
            2 => decode_index_buffer(count, stride, source, &mut ws),
            3 => decode_index_buffer_into(&mut out, count, stride, source, &mut ws).map(|()| out),
            4 => decode_index_sequence(count, stride, source, &mut ws),
            5 => decode_index_sequence_into(&mut out, count, stride, source, &mut ws).map(|()| out),
            6..=8 | 23 => {
                let n = source.len().min(out.len());
                out[..n].copy_from_slice(&source[..n]);
                match entry {
                    6 => decode_filter_oct(&mut out, count, stride, &mut ws),
                    7 => decode_filter_quat(&mut out, count, stride, &mut ws),
                    8 => decode_filter_exp(&mut out, count, stride, &mut ws),
                    _ => decode_filter_color(&mut out, count, stride, &mut ws),
                }
                .map(|()| out)
            }
            9 => decode_vertex_version(source).map(|version| vec![version]),
            10 => decode_index_version(source).map(|version| vec![version]),
            11 | 12 => {
                let mode =
                    [Mode::Attributes, Mode::Triangles, Mode::Indices][usize::from(data[6] % 3)];
                let filter = [
                    Filter::None,
                    Filter::Octahedral,
                    Filter::Quaternion,
                    Filter::Exponential,
                ][usize::from(data[7] % 4)];
                if entry == 11 {
                    decode_buffer_view(mode, filter, count, stride, source, &mut ws)
                } else {
                    decode_buffer_view_into(&mut out, mode, filter, count, stride, source, &mut ws)
                        .map(|()| out)
                }
            }
            26 | 27 => {
                let vc = usize::from(data[6]);
                let tc = (usize::from(data[7]) + (count & 1) * 256).min(256);
                let vs = [2, 4][(count >> 1) & 1];
                let ts = [3, 4][(count >> 2) & 1];
                if entry == 26 {
                    decode_meshlet(vc, vs, tc, ts, source, &mut ws)
                        .map(|d| [d.vertices, d.triangles].concat())
                } else {
                    let half = out.len() / 2;
                    let (v, t) = out.split_at_mut(half);
                    decode_meshlet_into(v, vc, vs, t, tc, ts, source, &mut ws).map(|()| out)
                }
            }
            _ => {
                let (vc, tc) = (usize::from(data[6]), usize::from(data[7]));
                if entry == 28 {
                    decode_meshlet_raw(vc, tc, source, &mut ws).map(|d| {
                        d.vertices
                            .into_iter()
                            .chain(d.triangles)
                            .flat_map(u32::to_le_bytes)
                            .collect()
                    })
                } else {
                    let mut v = [0xccu32; 256];
                    let mut t = [0xccu32; 256];
                    decode_meshlet_raw_into(&mut v, vc, &mut t, tc, source, &mut ws)
                        .map(|()| v.into_iter().chain(t).flat_map(u32::to_le_bytes).collect())
                }
            }
        };
        r
    };
    let expected = with_level(Level::Scalar, run).unwrap();
    let n = LEVEL_COUNTS[0].fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    for l in [
        Level::Sse2,
        Level::Ssse3,
        Level::Sse41,
        Level::Neon,
        Level::Wasm,
    ] {
        if let Ok(actual) = with_level(l, run) {
            LEVEL_COUNTS[l as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(actual, expected, "entry {entry}, level {l:?}");
        }
    }
    if n.is_multiple_of(1024) {
        if let Ok(path) = std::env::var("MESHOPT_FUZZ_COUNTS") {
            let counts = LEVEL_COUNTS
                .each_ref()
                .map(|c| c.load(std::sync::atomic::Ordering::Relaxed));
            std::fs::write(path, format!("{counts:?}\n")).expect("write per-level checkpoint");
        }
    }
}
