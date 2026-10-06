// meshoptimizer 1.3 filter encoders (vertexfilter.cpp) and the inline
// quantization helpers from meshoptimizer.h, MIT, Arseny Kapoulkine.
// Arithmetic follows scalar-strict f32 evaluation order without contraction.
use super::{filter::trunc_small, ExpMode};
use crate::{math::sqrt, Error};

/// meshopt_quantizeSnorm (n <= 16). The value is clamped to [-1, 1] (NaN
/// becomes -1), so |v * scale + round| < 2^22 and the exact small-range
/// truncation equals the reference's conversion.
#[inline(always)]
fn snorm(v: f32, n: u32) -> i32 {
    let scale = ((1i32 << (n - 1)) - 1) as f32;
    let round = if v >= 0.0 { 0.5 } else { -0.5 };
    let v = if v >= -1.0 { v } else { -1.0 };
    let v = if v <= 1.0 { v } else { 1.0 };
    trunc_small(v * scale + round)
}

/// meshopt_quantizeUnorm (n <= 16): clamped to [0, 1], so in the exact range.
#[inline(always)]
fn unorm(v: f32, n: u32) -> i32 {
    let scale = ((1i32 << n) - 1) as f32;
    let v = if v >= 0.0 { v } else { 0.0 };
    let v = if v <= 1.0 { v } else { 1.0 };
    trunc_small(v * scale + 0.5)
}

#[inline]
fn store4(out: &mut [u8], stride: usize, values: [i32; 4]) {
    for (i, v) in values.into_iter().enumerate() {
        if stride == 4 {
            out[i] = v as u8;
        } else {
            out[i * 2..i * 2 + 2].copy_from_slice(&(v as u16).to_le_bytes());
        }
    }
}

/// Apply a four-float to four-component record encoder. Four records per
/// step are computed together (identical per-record code, so the compiler
/// can vectorize across records) and stored as whole words; the remainder
/// uses the same record function.
#[inline(always)]
fn records(out: &mut [u8], stride: usize, data: &[f32], f: impl Fn([f32; 4]) -> [i32; 4]) {
    let (inputs, rest_in) = data.as_chunks::<16>();
    let mut blocks = out.chunks_exact_mut(stride * 4);
    for (block, input) in (&mut blocks).zip(inputs) {
        let v: [[i32; 4]; 4] = core::array::from_fn(|r| {
            f([
                input[r * 4],
                input[r * 4 + 1],
                input[r * 4 + 2],
                input[r * 4 + 3],
            ])
        });
        if stride == 4 {
            let words: [u32; 4] = core::array::from_fn(|r| {
                (v[r][0] as u32 & 0xff)
                    | (v[r][1] as u32 & 0xff) << 8
                    | (v[r][2] as u32 & 0xff) << 16
                    | (v[r][3] as u32) << 24
            });
            for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(words) {
                *dst = w.to_le_bytes();
            }
        } else {
            let words: [u32; 8] = core::array::from_fn(|i| {
                let (r, k) = (i / 2, (i % 2) * 2);
                (v[r][k] as u32 & 0xffff) | (v[r][k + 1] as u32) << 16
            });
            for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(words) {
                *dst = w.to_le_bytes();
            }
        }
    }
    let rest_out = blocks.into_remainder();
    for (record, n) in rest_out
        .chunks_exact_mut(stride)
        .zip(rest_in.as_chunks::<4>().0)
    {
        store4(record, stride, f(*n));
    }
}

/// Bounded quantization for small calls. Clamping makes the input finite and
/// bounded by 32767.5, including NaN (mapped to -1 like the reference).
/// Repair the integer nearest result directly, avoiding a second magic sum.
#[inline(always)]
fn snorm_tiny(v: f32, n: u32) -> i32 {
    let scale = ((1i32 << (n - 1)) - 1) as f32;
    let round = if v >= 0.0 { 0.5 } else { -0.5 };
    let v = if v >= -1.0 { v } else { -1.0 };
    let v = if v <= 1.0 { v } else { 1.0 };
    let f = v * scale + round;
    const MAGIC: f32 = 12582912.0;
    let sum = f + MAGIC;
    let nearest = sum - MAGIC;
    let integer = (sum.to_bits() as i32).wrapping_sub(MAGIC.to_bits() as i32);
    let sign = 1 | ((f.to_bits() as i32) >> 31);
    integer - if nearest.abs() > f.abs() { sign } else { 0 }
}

#[inline(always)]
fn snorm_mode<const TINY: bool>(v: f32, n: u32) -> i32 {
    if TINY {
        snorm_tiny(v, n)
    } else {
        snorm(v, n)
    }
}

pub(super) fn oct(out: &mut [u8], stride: usize, bits: u32, data: &[f32]) {
    // Small calls keep the lane arithmetic but use a shorter bounded
    // quantizer. Separate kernels keep their setup out of generic dispatch.
    if data.len() <= 32 * 4 {
        if stride == 4 {
            oct_fixed::<4, true>(out, bits, data);
        } else {
            oct_fixed::<8, true>(out, bits, data);
        }
    } else if stride == 4 {
        oct_fixed::<4, false>(out, bits, data);
    } else {
        oct_fixed::<8, false>(out, bits, data);
    }
}

#[inline(never)]
fn oct_fixed<const STRIDE: usize, const TINY: bool>(out: &mut [u8], bits: u32, data: &[f32]) {
    let stride = STRIDE;
    let byte_bits = stride as u32 * 2;
    // Component-major lanes over four records (vectorizes); the remainder
    // uses the per-record form, with identical arithmetic per value.
    let (inputs, rest_in) = data.as_chunks::<16>();
    let mut blocks = out.chunks_exact_mut(stride * 4);
    let one = snorm_mode::<TINY>(1.0, bits);
    for (block, input) in (&mut blocks).zip(inputs) {
        let c = |k: usize| -> [f32; 4] { core::array::from_fn(|r| input[r * 4 + k]) };
        let (mut nx, mut ny, nz, nw) = (c(0), c(1), c(2), c(3));
        let mut u = [0f32; 4];
        let mut v = [0f32; 4];
        for l in 0..4 {
            let nl = nx[l].abs() + ny[l].abs() + nz[l].abs();
            // Divide unconditionally and select, so the lanes vectorize.
            let inverse = 1.0 / nl;
            let ns = if nl == 0.0 { 0.0 } else { inverse };
            nx[l] *= ns;
            ny[l] *= ns;
            let su = if nx[l] >= 0.0 { 1.0 } else { -1.0 };
            let sv = if ny[l] >= 0.0 { 1.0 } else { -1.0 };
            u[l] = if nz[l] >= 0.0 {
                nx[l]
            } else {
                (1.0 - ny[l].abs()) * su
            };
            v[l] = if nz[l] >= 0.0 {
                ny[l]
            } else {
                (1.0 - nx[l].abs()) * sv
            };
        }
        let fu = u.map(|x| snorm_mode::<TINY>(x, bits));
        let fv = v.map(|x| snorm_mode::<TINY>(x, bits));
        let fw = nw.map(|x| snorm_mode::<TINY>(x, byte_bits));
        if stride == 4 {
            let words: [u32; 4] = core::array::from_fn(|r| {
                (fu[r] as u32 & 0xff)
                    | (fv[r] as u32 & 0xff) << 8
                    | (one as u32 & 0xff) << 16
                    | (fw[r] as u32) << 24
            });
            for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(words) {
                *dst = w.to_le_bytes();
            }
        } else {
            let words: [u32; 8] = core::array::from_fn(|i| {
                let r = i / 2;
                if i % 2 == 0 {
                    (fu[r] as u32 & 0xffff) | (fv[r] as u32) << 16
                } else {
                    (one as u32 & 0xffff) | (fw[r] as u32) << 16
                }
            });
            for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(words) {
                *dst = w.to_le_bytes();
            }
        }
    }
    let rest_out = blocks.into_remainder();
    oct_records::<TINY>(
        rest_out,
        stride,
        bits,
        byte_bits,
        &rest_in[..rest_in.len() / 4 * 4],
    );
}

fn oct_records<const TINY: bool>(
    out: &mut [u8],
    stride: usize,
    bits: u32,
    byte_bits: u32,
    data: &[f32],
) {
    records(out, stride, data, |[mut nx, mut ny, nz, nw]| {
        let nl = nx.abs() + ny.abs() + nz.abs();
        let ns = if nl == 0.0 { 0.0 } else { 1.0 / nl };
        nx *= ns;
        ny *= ns;
        let u = if nz >= 0.0 {
            nx
        } else {
            (1.0 - ny.abs()) * if nx >= 0.0 { 1.0 } else { -1.0 }
        };
        let v = if nz >= 0.0 {
            ny
        } else {
            (1.0 - nx.abs()) * if ny >= 0.0 { 1.0 } else { -1.0 }
        };
        [
            snorm_mode::<TINY>(u, bits),
            snorm_mode::<TINY>(v, bits),
            snorm_mode::<TINY>(1.0, bits),
            snorm_mode::<TINY>(nw, byte_bits),
        ]
    });
}

pub(super) fn quat(out: &mut [u8], bits: u32, data: &[f32]) {
    if data.len() <= 32 * 4 {
        quat_tiny(out, bits, data);
    } else {
        quat_bulk(out, bits, data);
    }
}

#[inline(never)]
fn quat_tiny(out: &mut [u8], bits: u32, data: &[f32]) {
    let scale = ((1i32 << (bits - 1)) - 1) as f32;
    let one = ((1i32 << (bits - 1)) - 1) & !3;
    for (dst, q) in out
        .as_chunks_mut::<8>()
        .0
        .iter_mut()
        .zip(data.as_chunks::<4>().0)
    {
        *dst = quat_record(q, scale, one).to_le_bytes();
    }
}

// Keep vectorization within one quaternion. Cross-record SLP leaves packed
// selectors, gathered swizzles and quantization intermediates live together.
#[inline(never)]
fn quat_record(q: &[f32; 4], scale: f32, one: i32) -> u64 {
    let [qx, qy, qz, qw] = *q;
    let mut largest = qx;
    let mut qc = 0;
    if qy.abs() > largest.abs() {
        largest = qy;
        qc = 1;
    }
    if qz.abs() > largest.abs() {
        largest = qz;
        qc = 2;
    }
    if qw.abs() > largest.abs() {
        largest = qw;
        qc = 3;
    }
    let rotated = match qc {
        0 => [qy, qz, qw],
        1 => [qz, qw, qx],
        2 => [qw, qx, qy],
        _ => [qx, qy, qz],
    };
    let sign = if largest < 0.0 { -1.0 } else { 1.0 };
    let [x, y, z] = rotated.map(|v| {
        let v = v * sqrt(2.0) * sign;
        let round = if v >= 0.0 { 0.5 } else { -0.5 };
        let v = if v >= -1.0 { v } else { -1.0 };
        let v = if v <= 1.0 { v } else { 1.0 };
        (v * scale + round) as i16 as u16
    });
    u64::from(x) | u64::from(y) << 16 | u64::from(z) << 32 | u64::from((one | qc) as u16) << 48
}

#[inline(never)]
fn quat_bulk(out: &mut [u8], bits: u32, data: &[f32]) {
    let scaler = sqrt(2.0);
    let (inputs, rest) = data.as_chunks::<16>();
    let (blocks, rest_out) = out.as_chunks_mut::<32>();
    let one = snorm(1.0, bits) & !3;
    for (block, input) in blocks.iter_mut().zip(inputs) {
        let q: [[f32; 4]; 4] = core::array::from_fn(|c| core::array::from_fn(|r| input[r * 4 + c]));
        let mut largest = q[0];
        let mut selectors = [0u32; 4];
        for (c, component) in q.iter().enumerate().skip(1) {
            for r in 0..4 {
                // Bit selects preserve the first strict maximum, including NaN
                // and signed-zero behavior, without gathering q[selector].
                let mask = 0u32.wrapping_sub(u32::from(component[r].abs() > largest[r].abs()));
                selectors[r] = (selectors[r] & !mask) | (c as u32 & mask);
                largest[r] = f32::from_bits(
                    (largest[r].to_bits() & !mask) | (component[r].to_bits() & mask),
                );
            }
        }
        let signs = largest.map(|x| if x < 0.0 { -1.0 } else { 1.0 });
        let low = selectors.map(|s| 0u32.wrapping_sub(s & 1));
        let high = selectors.map(|s| 0u32.wrapping_sub(s >> 1));
        let select = |a: [f32; 4], b: [f32; 4], masks: [u32; 4]| -> [f32; 4] {
            core::array::from_fn(|r| {
                f32::from_bits(a[r].to_bits() ^ ((a[r].to_bits() ^ b[r].to_bits()) & masks[r]))
            })
        };
        let q01 = select(q[0], q[1], low);
        let q12 = select(q[1], q[2], low);
        let q23 = select(q[2], q[3], low);
        let q30 = select(q[3], q[0], low);
        let (sx, sy, sz) = (
            select(q12, q30, high),
            select(q23, q01, high),
            select(q30, q12, high),
        );
        let quantize = |v: [f32; 4]| -> [i32; 4] {
            core::array::from_fn(|r| snorm(v[r] * scaler * signs[r], bits))
        };
        let (x, y, z) = (quantize(sx), quantize(sy), quantize(sz));
        for (r, dst) in block.as_chunks_mut::<8>().0.iter_mut().enumerate() {
            let packed = u64::from(x[r] as u16)
                | u64::from(y[r] as u16) << 16
                | u64::from(z[r] as u16) << 32
                | u64::from((one | selectors[r] as i32) as u16) << 48;
            *dst = packed.to_le_bytes();
        }
    }
    for (record, q) in rest_out
        .as_chunks_mut::<8>()
        .0
        .iter_mut()
        .zip(rest.as_chunks::<4>().0)
    {
        let mut qc = 0;
        for i in 1..4 {
            if q[i].abs() > q[qc].abs() {
                qc = i;
            }
        }
        let sign = if q[qc] < 0.0 { -1.0 } else { 1.0 };
        store4(
            record,
            8,
            [
                snorm(q[(qc + 1) & 3] * scaler * sign, bits),
                snorm(q[(qc + 2) & 3] * scaler * sign, bits),
                snorm(q[(qc + 3) & 3] * scaler * sign, bits),
                one | qc as i32,
            ],
        );
    }
}

#[inline]
fn log2(v: f32) -> i32 {
    // Zero and subnormal values are clamped by the callers.
    ((v.to_bits() >> 23) & 0xff) as i32 - 127 + 1
}

#[inline]
fn exp2(e: i32) -> f32 {
    f32::from_bits((e.wrapping_add(127) as u32) << 23)
}

const SEPARATE: u8 = 0;
const SHARED_VECTOR: u8 = 1;
const SHARED_COMPONENT: u8 = 2;
const CLAMPED: u8 = 3;

/// Returns NumericalFailure exactly where the reference's float-to-int
/// conversion is undefined: non-finite input, or bits = 1 with a shared or
/// own exponent of 128. The destination may then be partially written.
pub(super) fn exp(
    out: &mut [u8],
    stride: usize,
    bits: u32,
    data: &[f32],
    mode: ExpMode,
) -> Result<(), Error> {
    let valid = match mode {
        ExpMode::Separate => exp_mode::<SEPARATE>(out, stride, bits, data),
        ExpMode::SharedVector => exp_mode::<SHARED_VECTOR>(out, stride, bits, data),
        ExpMode::SharedComponent => exp_mode::<SHARED_COMPONENT>(out, stride, bits, data),
        ExpMode::Clamped => exp_mode::<CLAMPED>(out, stride, bits, data),
    };
    if valid {
        Ok(())
    } else {
        Err(Error::NumericalFailure)
    }
}

/// Common layouts retain a constant record width through the component loops.
/// The dynamic fallback handles every other supported stride identically.
fn exp_mode<const MODE: u8>(out: &mut [u8], stride: usize, bits: u32, data: &[f32]) -> bool {
    match stride {
        4 => exp_fixed::<MODE, 1>(out, stride, bits, data),
        8 => exp_fixed::<MODE, 2>(out, stride, bits, data),
        12 => exp_fixed::<MODE, 3>(out, stride, bits, data),
        16 => exp_fixed::<MODE, 4>(out, stride, bits, data),
        _ => exp_fixed::<MODE, 0>(out, stride, bits, data),
    }
}

/// One mode/layout, monomorphized. False marks an undefined conversion.
fn exp_fixed<const MODE: u8, const FLOATS: usize>(
    out: &mut [u8],
    stride: usize,
    bits: u32,
    data: &[f32],
) -> bool {
    const MIN_EXP: i32 = -100;
    const MASK: i32 = (1 << 24) - 1;
    const MAX: i32 = MASK >> 1;
    let floats = if FLOATS == 0 { stride / 4 } else { FLOATS };
    let stride = floats * 4;
    let mut component = [if MODE == SEPARATE { 0 } else { MIN_EXP }; 64];
    let component = &mut component[..floats];
    if MODE == SHARED_COMPONENT {
        for v in data.chunks_exact(floats) {
            for (c, &x) in component.iter_mut().zip(v) {
                *c = (*c).max(log2(x));
            }
        }
    }
    // |x * 2^-e| <= 2^(bits - 1) whenever the exponent is defined, so only a
    // non-finite or i32-overflowing value marks an undefined conversion.
    let mut valid = true;
    for (record, v) in out.chunks_exact_mut(stride).zip(data.chunks_exact(floats)) {
        let mut vector = MIN_EXP;
        match MODE {
            SHARED_VECTOR => {
                for &x in v {
                    vector = vector.max(log2(x));
                }
            }
            SEPARATE => {
                for (c, &x) in component.iter_mut().zip(v) {
                    // Zero values inherit the last exponent for compressibility.
                    if x != 0.0 {
                        *c = MIN_EXP.max(log2(x));
                    }
                }
            }
            CLAMPED => {
                for (c, &x) in component.iter_mut().zip(v) {
                    *c = 0.max(log2(x));
                }
            }
            _ => {}
        }
        for ((word, &x), &c) in record
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(v)
            .zip(component.iter())
        {
            let e = if MODE == SHARED_VECTOR { vector } else { c } - (bits as i32 - 1);
            let f = x * exp2(-e) + if x >= 0.0 { 0.5 } else { -0.5 };
            valid &= (-2147483648.0..2147483648.0).contains(&f);
            // Clamp so that rounding away from zero cannot overflow.
            let m = (f as i32).min(MAX);
            *word = ((m & MASK) as u32 | (e as u32) << 24).to_le_bytes();
        }
    }
    valid
}

pub(super) fn color(out: &mut [u8], stride: usize, bits: u32, data: &[f32]) {
    records(out, stride, data, |c| {
        let r = unorm(c[0], bits);
        let g = unorm(c[1], bits);
        let b = unorm(c[2], bits);
        // YCoCg-R with truncated Co/Cg allows exact integer reconstruction.
        let co = (r - b) / 2;
        let tmp = b + co;
        let cg = (g - tmp) / 2;
        let y = tmp + cg;
        // Alpha uses K-1 bits with the high bit set.
        let a = (unorm(c[3], bits) >> 1) | (1 << (bits - 1));
        [y, co, cg, a]
    });
}

#[cfg(test)]
mod tiny_tests {
    use alloc::{vec, vec::Vec};

    use super::{oct, oct_fixed, quat, quat_bulk};

    #[test]
    fn tiny_matches_bulk_at_precision_and_dispatch_boundaries() {
        // Ties, signed zero, folds, clamps and non-finite snorm inputs.
        let values = [
            [0.0, -0.0, 0.0, -0.0],
            [0.5, -0.5, -0.5, 1.0],
            [-0.5, 0.5, 0.5, -1.0],
            [1.0, 1.0, 1.0, 1.0],
            [f32::NAN, 0.0, -1.0, f32::NAN],
            [f32::INFINITY, f32::NEG_INFINITY, 0.0, 2.0],
        ];
        for count in [0, 1, 3, 4, 17, 31, 32, 33] {
            let data: Vec<f32> = (0..count).flat_map(|i| values[i % values.len()]).collect();
            for stride in [4, 8] {
                for bits in 2..=if stride == 4 { 8 } else { 16 } {
                    let mut actual = vec![0; count * stride];
                    let mut expected = actual.clone();
                    oct(&mut actual, stride, bits, &data);
                    if stride == 4 {
                        oct_fixed::<4, false>(&mut expected, bits, &data);
                    } else {
                        oct_fixed::<8, false>(&mut expected, bits, &data);
                    }
                    assert_eq!(
                        actual, expected,
                        "Oct count={count} stride={stride} bits={bits}"
                    );
                }
            }
            for bits in 4..=16 {
                let mut actual = vec![0; count * 8];
                let mut expected = actual.clone();
                quat(&mut actual, bits, &data);
                quat_bulk(&mut expected, bits, &data);
                assert_eq!(actual, expected, "Quat count={count} bits={bits}");
            }
        }
    }
}
