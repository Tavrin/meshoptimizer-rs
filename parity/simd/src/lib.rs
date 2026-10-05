//! Arithmetic release gates shared by native and executed wasm qualification.
use meshoptimizer_rs::{codec::*, Workspace};
fn random(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}
/// Compare a resumable range. Zero means pass; otherwise the first mismatching
/// input/record index plus one. Family 0 sqrt, 1 Exp, 2 Oct8, 3 Oct16, 4 Quat.
#[no_mangle]
pub extern "C" fn check_range(family: u32, start: u32, count: u32) -> u64 {
    let start = u64::from(start);
    let end = start + u64::from(count);
    if family > 4 || end > 1u64 << 32 {
        return u64::MAX;
    }
    let mut w = Workspace::default();
    let mut seed = 20261005u32;
    if family >= 3 {
        for _ in 0..start * 4 {
            random(&mut seed);
        }
    }
    for base in (start..end).step_by(1024) {
        let n = (end - base).min(1024) as usize;
        if family == 0 {
            for b in (base..base + n as u64).step_by(4) {
                let input = core::array::from_fn(|i| f32::from_bits((b + i as u64) as u32));
                let Some(actual) = simd_sqrt4(input) else {
                    return u64::MAX;
                };
                for i in 0..4 {
                    if b + (i as u64) < end
                        && actual[i].to_bits() != libm::sqrtf(input[i]).to_bits()
                    {
                        return b + i as u64 + 1;
                    }
                }
            }
            continue;
        }
        let stride = if family >= 3 { 8 } else { 4 };
        let mut original = vec![0u8; n * stride];
        for i in 0..n {
            let bits = (base + i as u64) as u32;
            if family <= 2 {
                original[i * 4..i * 4 + 4].copy_from_slice(&bits.to_le_bytes());
                if family == 2 {
                    original[i * 4 + 3] = 0;
                }
            } else {
                for c in 0..4 {
                    original[i * 8 + c * 2..i * 8 + c * 2 + 2]
                        .copy_from_slice(&(random(&mut seed) as u16).to_le_bytes());
                }
            }
        }
        let run = |data: &mut [u8], records, w: &mut Workspace| match family {
            1 => decode_filter_exp(data, records, stride, w),
            2 | 3 => decode_filter_oct(data, records, stride, w),
            _ => decode_filter_quat(data, records, stride, w),
        };
        let mut scalar = original.clone();
        let mut vector = original.clone();
        let expected = with_level(Level::Scalar, || run(&mut scalar, n, &mut w)).unwrap();
        let actual = run(&mut vector, n, &mut w);
        if actual != expected {
            return base + 1;
        }
        if actual.is_ok() {
            if vector != scalar {
                return base + 1;
            }
        } else {
            // Do not let a zero Oct vector hide its batch's valid records.
            // Distinct valid sentinels also prevent the repeated-record cache.
            for i in 0..n {
                let mut input = vec![0u8; stride * 4];
                input[..stride].copy_from_slice(&original[i * stride..(i + 1) * stride]);
                for k in 1..4 {
                    if stride == 4 {
                        input[k * 4 + 2] = k as u8;
                    } else {
                        input[k * 8 + 4..k * 8 + 6].copy_from_slice(&(k as i16).to_le_bytes());
                    }
                }
                let mut a = input.clone();
                let mut b = input;
                let s = with_level(Level::Scalar, || run(&mut a, 4, &mut w)).unwrap();
                let v = run(&mut b, 4, &mut w);
                if s != v || (s.is_ok() && a != b) {
                    return base + i as u64 + 1;
                }
            }
        }
    }
    0
}

/// Every Quat scale word and component/sign extremes; Oct edge combinations.
#[no_mangle]
pub extern "C" fn check_edges() -> u64 {
    let mut w = Workspace::default();
    for family in [3u32, 4] {
        let values = [i16::MIN, -32767, -16384, -1, 0, 1, 16383, 32766, i16::MAX];
        let mut source = Vec::new();
        if family == 4 {
            for d in i16::MIN..=i16::MAX {
                for &[a, b, c] in &[
                    [0, 0, 0],
                    [i16::MIN, i16::MAX, -1],
                    [i16::MAX, i16::MAX, i16::MAX],
                    [1, -1, 1],
                    [-1, 1, -1],
                ] {
                    for v in [a, b, c, d] {
                        source.extend_from_slice(&v.to_le_bytes());
                    }
                }
            }
        } else {
            for a in values {
                for b in values {
                    for c in values {
                        for v in [a, b, c, 0] {
                            source.extend_from_slice(&v.to_le_bytes());
                        }
                    }
                }
            }
        }
        let n = source.len() / 8;
        let mut a = source.clone();
        let mut b = source;
        let run = |data: &mut [u8], w: &mut Workspace| {
            if family == 4 {
                decode_filter_quat(data, n, 8, w)
            } else {
                decode_filter_oct(data, n, 8, w)
            }
        };
        let s = with_level(Level::Scalar, || run(&mut a, &mut w)).unwrap();
        let v = run(&mut b, &mut w);
        if s != v || (s.is_ok() && a != b) {
            return u64::from(family);
        }
        if s.is_err() && family == 3 {
            // Each nonzero edge record runs in a vector batch with distinct sentinels.
            for x in values {
                for y in values {
                    for z in values {
                        let mut input = vec![0; 32];
                        for (i, word) in [x, y, z, 0].into_iter().enumerate() {
                            input[i * 2..i * 2 + 2].copy_from_slice(&word.to_le_bytes());
                        }
                        for k in 1..4 {
                            input[k * 8 + 4..k * 8 + 6].copy_from_slice(&(k as i16).to_le_bytes());
                        }
                        let mut a = input.clone();
                        let mut b = input;
                        let s =
                            with_level(Level::Scalar, || decode_filter_oct(&mut a, 4, 8, &mut w))
                                .unwrap();
                        let v = decode_filter_oct(&mut b, 4, 8, &mut w);
                        if s != v || (s.is_ok() && a != b) {
                            return 3;
                        }
                    }
                }
            }
        }
    }
    0
}

// Purpose-built WASM timing adapter. Source/output copies happen in the same
// Node loop as upstream's shipped decoder. No transport serialization is timed.
use std::sync::Mutex;
struct Bench {
    source: Vec<u8>,
    output: Vec<u8>,
    workspace: Workspace,
    op: u32,
    count: usize,
    stride: usize,
    mode: Mode,
    filter: Filter,
}
static BENCH: Mutex<Option<Bench>> = Mutex::new(None);
#[no_mangle]
pub extern "C" fn bench_prepare(
    length: u32,
    op: u32,
    count: u32,
    stride: u32,
    mode: u32,
    filter: u32,
) -> u32 {
    let mode = match mode {
        0 => Mode::Attributes,
        1 => Mode::Triangles,
        2 => Mode::Indices,
        _ => return 1,
    };
    let filter = match filter {
        0 => Filter::None,
        1 => Filter::Octahedral,
        2 => Filter::Quaternion,
        3 => Filter::Exponential,
        _ => return 1,
    };
    let Some(bytes) = (count as usize).checked_mul(stride as usize) else {
        return 1;
    };
    if length > 128 * 1024 * 1024 || bytes > 128 * 1024 * 1024 || !matches!(op, 1 | 2 | 3 | 7) {
        return 1;
    }
    *BENCH.lock().unwrap() = Some(Bench {
        source: vec![0; length as usize],
        output: vec![0; bytes],
        workspace: Workspace::default(),
        op,
        count: count as usize,
        stride: stride as usize,
        mode,
        filter,
    });
    0
}
#[no_mangle]
pub extern "C" fn bench_source_ptr() -> usize {
    BENCH
        .lock()
        .unwrap()
        .as_ref()
        .map_or(0, |b| b.source.as_ptr() as usize)
}
#[no_mangle]
pub extern "C" fn bench_output_ptr() -> usize {
    BENCH
        .lock()
        .unwrap()
        .as_ref()
        .map_or(0, |b| b.output.as_ptr() as usize)
}
#[no_mangle]
pub extern "C" fn bench_run(into: u32) -> u32 {
    #[cfg(feature = "force-scalar")]
    {
        with_level(Level::Scalar, || bench_run_inner(into)).unwrap()
    }
    #[cfg(not(feature = "force-scalar"))]
    bench_run_inner(into)
}
fn bench_run_inner(into: u32) -> u32 {
    let mut lock = BENCH.lock().unwrap();
    let Some(b) = lock.as_mut() else {
        return 1;
    };
    let Bench {
        source,
        output,
        workspace,
        op,
        count,
        stride,
        mode,
        filter,
    } = b;
    let result = if into != 0 {
        match *op {
            1 => decode_vertex_buffer_into(output, *count, *stride, source, workspace),
            2 => decode_index_buffer_into(output, *count, *stride, source, workspace),
            3 => decode_index_sequence_into(output, *count, *stride, source, workspace),
            _ => {
                decode_buffer_view_into(output, *mode, *filter, *count, *stride, source, workspace)
            }
        }
    } else {
        let result = match *op {
            1 => decode_vertex_buffer(*count, *stride, source, workspace),
            2 => decode_index_buffer(*count, *stride, source, workspace),
            3 => decode_index_sequence(*count, *stride, source, workspace),
            _ => decode_buffer_view(*mode, *filter, *count, *stride, source, workspace),
        };
        result.map(|out| *output = out)
    };
    std::hint::black_box(result).is_err() as u32
}
