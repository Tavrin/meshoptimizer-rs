use super::dispatch::Baseline;
use core::arch::wasm32::*;
#[inline]
#[target_feature(enable = "simd128")]
fn fload(v: [f32; 4]) -> v128 {
    f32x4(v[0], v[1], v[2], v[3])
}
#[inline]
#[target_feature(enable = "simd128")]
fn splat(v: f32) -> v128 {
    f32x4_splat(v)
}
#[inline]
#[target_feature(enable = "simd128")]
fn add(a: v128, b: v128) -> v128 {
    f32x4_add(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn sub(a: v128, b: v128) -> v128 {
    f32x4_sub(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn mul(a: v128, b: v128) -> v128 {
    f32x4_mul(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn div(a: v128, b: v128) -> v128 {
    f32x4_div(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn sqrt(a: v128) -> v128 {
    select(ge(a, splat(0.0)), f32x4_sqrt(a), splat(f32::NAN))
}
#[inline]
#[target_feature(enable = "simd128")]
fn abs(a: v128) -> v128 {
    f32x4_abs(a)
}
#[inline]
#[target_feature(enable = "simd128")]
fn neg(a: v128) -> v128 {
    f32x4_neg(a)
}
#[inline]
#[target_feature(enable = "simd128")]
fn ge(a: v128, b: v128) -> v128 {
    f32x4_ge(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn select(mask: v128, a: v128, b: v128) -> v128 {
    v128_bitselect(a, b, mask)
}
#[inline]
#[target_feature(enable = "simd128")]
fn load(bytes: &[u8; 16]) -> v128 {
    // SAFETY: the initialized array supplies 16 readable bytes; v128_load
    // permits unaligned addresses and retains no pointer.
    unsafe { v128_load(bytes.as_ptr().cast()) }
}
#[inline]
#[target_feature(enable = "simd128")]
fn store(bytes: &mut [u8; 16], v: v128) {
    // SAFETY: the exclusive array supplies 16 writable bytes; v128_store
    // permits unaligned addresses and retains no pointer.
    unsafe { v128_store(bytes.as_mut_ptr().cast(), v) }
}
#[target_feature(enable = "simd128")]
fn fout(v: v128) -> [f32; 4] {
    let mut b = [0; 16];
    store(&mut b, v);
    core::array::from_fn(|i| {
        f32::from_bits(u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
    })
}
#[target_feature(enable = "simd128")]
fn filter_kernel(kind: u8, data: &mut [u8], stride: usize) -> Result<(), crate::Error> {
    match kind {
        1 if stride == 4 => oct_kernel::<4, 16>(data),
        1 => oct_kernel::<8, 32>(data),
        2 => quat_kernel(data),
        3 => {
            exp_kernel(data);
            Ok(())
        }
        _ if stride == 4 => color_kernel::<4, 16>(data),
        _ => color_kernel::<8, 32>(data),
    }
}

// Four packed halfword records become four independent component vectors.
#[inline]
#[target_feature(enable = "simd128")]
fn fields16(block: &[u8; 32]) -> [v128; 4] {
    let a = load(block[..16].first_chunk().unwrap());
    let b = load(block[16..].first_chunk().unwrap());
    let lo = i32x4_shuffle::<0, 2, 4, 6>(a, b);
    let hi = i32x4_shuffle::<1, 3, 5, 7>(a, b);
    [
        i32x4_shr(i32x4_shl(lo, 16), 16),
        i32x4_shr(lo, 16),
        i32x4_shr(i32x4_shl(hi, 16), 16),
        i32x4_shr(hi, 16),
    ]
}
#[inline]
#[target_feature(enable = "simd128")]
fn round_vector(value: v128, sign: v128) -> v128 {
    i32x4_trunc_sat_f32x4(add(
        value,
        select(ge(sign, splat(0.0)), splat(0.5), splat(-0.5)),
    ))
}
#[inline]
#[target_feature(enable = "simd128")]
fn pair16(a: v128, b: v128) -> v128 {
    v128_or(v128_and(a, i32x4_splat(65535)), i32x4_shl(b, 16))
}
#[target_feature(enable = "simd128")]
fn oct_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let raw = if W == 4 {
            let v = load(block.first_chunk::<16>().unwrap());
            [
                i32x4_shr(i32x4_shl(v, 24), 24),
                i32x4_shr(i32x4_shl(v, 16), 24),
                i32x4_shr(i32x4_shl(v, 8), 24),
                u32x4_shr(v, 24),
            ]
        } else {
            fields16(block.first_chunk::<32>().unwrap())
        };
        let mut x = f32x4_convert_i32x4(raw[0]);
        let mut y = f32x4_convert_i32x4(raw[1]);
        let z = sub(sub(f32x4_convert_i32x4(raw[2]), abs(x)), abs(y));
        let t = select(ge(z, splat(0.0)), splat(0.0), z);
        x = add(x, select(ge(x, splat(0.0)), t, neg(t)));
        y = add(y, select(ge(y, splat(0.0)), t, neg(t)));
        let length = f32x4_sqrt(add(add(mul(x, x), mul(y, y)), mul(z, z)));
        if i32x4_bitmask(f32x4_eq(length, splat(0.0))) != 0 {
            return Err(crate::Error::NumericalFailure);
        }
        let ss = div(splat(if W == 4 { 127.0 } else { 32767.0 }), length);
        let r = round_vector(mul(x, ss), x);
        let g = round_vector(mul(y, ss), y);
        let b = round_vector(mul(z, ss), z);
        if W == 4 {
            let packed = v128_or(
                v128_or(
                    v128_and(r, i32x4_splat(255)),
                    i32x4_shl(v128_and(g, i32x4_splat(255)), 8),
                ),
                v128_or(
                    i32x4_shl(v128_and(b, i32x4_splat(255)), 16),
                    i32x4_shl(raw[3], 24),
                ),
            );
            store(block.first_chunk_mut::<16>().unwrap(), packed);
        } else {
            let rg = pair16(r, g);
            let ba = pair16(b, raw[3]);
            store(
                block[..16].first_chunk_mut().unwrap(),
                i32x4_shuffle::<0, 4, 1, 5>(rg, ba),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                i32x4_shuffle::<2, 6, 3, 7>(rg, ba),
            );
        }
    }
    crate::codec::filter::scalar_oct(tail, W)
}
#[target_feature(enable = "simd128")]
fn quat_kernel(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<32>();
    let scale = splat(32767.0 / crate::math::sqrt(2.0));
    for block in blocks {
        // Keep rotation selectors before the packed output overwrites them.
        let rotations = [block[6], block[14], block[22], block[30]];
        let [xi, yi, zi, ci] = fields16(block);
        let x = f32x4_convert_i32x4(xi);
        let y = f32x4_convert_i32x4(yi);
        let z = f32x4_convert_i32x4(zi);
        let s = f32x4_convert_i32x4(v128_or(ci, i32x4_splat(3)));
        let ww = sub(
            sub(sub(mul(mul(s, s), splat(2.0)), mul(x, x)), mul(y, y)),
            mul(z, z),
        );
        let w = f32x4_sqrt(select(ge(ww, splat(0.0)), ww, splat(0.0)));
        let ss = div(scale, s);
        let xr = round_vector(mul(x, ss), x);
        let yr = round_vector(mul(y, ss), y);
        let zr = round_vector(mul(z, ss), z);
        let wr = i32x4_trunc_sat_f32x4(add(mul(w, ss), splat(0.5)));
        let xy = pair16(xr, yr);
        let zw = pair16(zr, wr);
        let mut packed = [0u8; 32];
        store(
            packed[..16].first_chunk_mut().unwrap(),
            i32x4_shuffle::<0, 4, 1, 5>(xy, zw),
        );
        store(
            packed[16..].first_chunk_mut().unwrap(),
            i32x4_shuffle::<2, 6, 3, 7>(xy, zw),
        );
        for ((out, value), rotation) in block
            .as_chunks_mut::<8>()
            .0
            .iter_mut()
            .zip(packed.as_chunks::<8>().0)
            .zip(rotations)
        {
            *out = u64::from_le_bytes(*value)
                .rotate_left(u32::from((rotation.wrapping_add(1)) & 3) * 16)
                .to_le_bytes();
        }
    }
    crate::codec::filter::scalar_quat(tail)
}

#[inline]
#[target_feature(enable = "simd128")]
fn unpack_bytes(a: v128, b: v128) -> v128 {
    i8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(a, b)
}
#[inline]
#[target_feature(enable = "simd128")]
fn group_kernel(data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    if bits == 0 {
        out.fill(0);
        return 0;
    }
    if bits == 8 {
        out.copy_from_slice(&data[..16]);
        return 16;
    }
    let input = load(data.first_chunk::<16>().unwrap());
    let (fields, mask) = if bits == 1 {
        (
            i8x16_splat(0),
            usize::from(data[0]) | (usize::from(data[1]) << 8),
        )
    } else {
        let fields = if bits == 2 {
            let pairs = unpack_bytes(u16x8_shr(input, 4), input);
            v128_and(unpack_bytes(u16x8_shr(pairs, 2), pairs), i8x16_splat(3))
        } else {
            v128_and(unpack_bytes(u16x8_shr(input, 4), input), i8x16_splat(15))
        };
        (
            fields,
            i8x16_bitmask(i8x16_eq(fields, i8x16_splat(((1 << bits) - 1) as i8))) as usize,
        )
    };
    let lo = mask & 255;
    let hi = mask >> 8;
    let upper = i8x16_add(load(&super::MASKS[hi]), i8x16_splat(lo.count_ones() as i8));
    let shuf = i64x2_shuffle::<0, 2>(load(&super::MASKS[lo]), upper);
    let skip = (bits * 2) as usize;
    let rest = i8x16_swizzle(load(data[skip..].first_chunk::<16>().unwrap()), shuf);
    let result = if bits == 1 {
        rest
    } else {
        v128_bitselect(
            rest,
            fields,
            i8x16_eq(fields, i8x16_splat(((1 << bits) - 1) as i8)),
        )
    };
    store(out, result);
    skip + mask.count_ones() as usize
}
pub(super) fn filter(
    _token: Baseline,
    kind: u8,
    data: &mut [u8],
    stride: usize,
) -> Result<(), crate::Error> {
    // SAFETY: Baseline is issued only on this architecture with simd128
    // enabled; the safe kernel checks all slice bounds.
    filter_kernel(kind, data, stride)
}
pub(super) fn group(_token: Baseline, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Baseline proves simd128 availability; complete initialized
    // input and exclusive output arrays supply all memory used by the kernel.
    group_kernel(data, out, bits)
}

#[cfg(feature = "parity-internals")]
pub(super) fn sqrt4(_token: Baseline, values: [f32; 4]) -> [f32; 4] {
    sqrt4_kernel(values)
}
#[cfg(feature = "parity-internals")]
#[target_feature(enable = "simd128")]
fn sqrt4_kernel(values: [f32; 4]) -> [f32; 4] {
    fout(sqrt(fload(values)))
}

#[target_feature(enable = "simd128")]
fn exp_kernel(data: &mut [u8]) {
    let (chunks, tail) = data.as_chunks_mut::<16>();
    for chunk in chunks {
        let v = load(chunk);
        let m = f32x4_convert_i32x4(i32x4_shr(i32x4_shl(v, 8), 8));
        let e = i32x4_shl(i32x4_add(i32x4_shr(v, 24), i32x4_splat(127)), 23);
        store(chunk, f32x4_mul(e, m));
    }
    crate::codec::filter::scalar_exp(tail);
}

#[target_feature(enable = "simd128")]
fn color_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let (y, co, cg, alpha) = if W == 4 {
            let v = load(block.first_chunk::<16>().unwrap());
            (
                v128_and(v, i32x4_splat(255)),
                i32x4_shr(i32x4_shl(v, 16), 24),
                i32x4_shr(i32x4_shl(v, 8), 24),
                u32x4_shr(v, 24),
            )
        } else {
            let (lo, hi) = {
                let a = load(block[..16].first_chunk().unwrap());
                let b = load(block[16..].first_chunk().unwrap());
                (
                    i32x4_shuffle::<0, 2, 4, 6>(a, b),
                    i32x4_shuffle::<1, 3, 5, 7>(a, b),
                )
            };
            (
                v128_and(lo, i32x4_splat(65535)),
                i32x4_shr(lo, 16),
                i32x4_shr(i32x4_shl(hi, 16), 16),
                u32x4_shr(hi, 16),
            )
        };
        let mut scale = alpha;
        scale = v128_or(scale, i32x4_shr(scale, 1));
        scale = v128_or(scale, i32x4_shr(scale, 2));
        scale = v128_or(scale, i32x4_shr(scale, 4));
        scale = v128_or(scale, i32x4_shr(scale, 8));
        let a = v128_or(
            v128_and(i32x4_shl(alpha, 1), scale),
            v128_and(alpha, i32x4_splat(1)),
        );
        let ss = div(
            splat(if W == 4 { 255.0 } else { 65535.0 }),
            f32x4_convert_i32x4(scale),
        );
        let channels = [
            i32x4_sub(i32x4_add(y, co), cg),
            i32x4_add(y, cg),
            i32x4_sub(i32x4_sub(y, co), cg),
            a,
        ];
        let f = channels.map(|v| add(mul(f32x4_convert_i32x4(v), ss), splat(0.5)));
        let mut mask = i32x4_splat(-1);
        for v in f {
            mask = v128_and(
                mask,
                v128_and(
                    f32x4_ge(v, splat(-2147483648.0)),
                    f32x4_lt(v, splat(2147483648.0)),
                ),
            );
        }
        let valid = i32x4_bitmask(mask) == 15;
        if !valid {
            crate::codec::filter::scalar_color(block, W)?;
            continue;
        }
        let [r, g, b, a] = f.map(|v| i32x4_trunc_sat_f32x4(v));
        if W == 4 {
            let value = v128_or(
                v128_or(
                    v128_and(r, i32x4_splat(255)),
                    i32x4_shl(v128_and(g, i32x4_splat(255)), 8),
                ),
                v128_or(
                    i32x4_shl(v128_and(b, i32x4_splat(255)), 16),
                    i32x4_shl(a, 24),
                ),
            );
            store(block.first_chunk_mut::<16>().unwrap(), value);
        } else {
            let lo = v128_or(v128_and(r, i32x4_splat(65535)), i32x4_shl(g, 16));
            let hi = v128_or(v128_and(b, i32x4_splat(65535)), i32x4_shl(a, 16));
            store(
                block[..16].first_chunk_mut().unwrap(),
                i32x4_shuffle::<0, 4, 1, 5>(lo, hi),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                i32x4_shuffle::<2, 6, 3, 7>(lo, hi),
            );
        }
    }
    crate::codec::filter::scalar_color(tail, W)
}

pub(super) fn deltas8(
    _token: Baseline,
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
) {
    deltas8_kernel(buffer, target, count, stride, last)
}
#[target_feature(enable = "simd128")]
fn deltas8_kernel(buffer: &[u8], target: &mut [u8], count: usize, stride: usize, last: &[u8]) {
    let mut previous = [last[0], last[1], last[2], last[3]];
    for start in (0..count).step_by(16) {
        let n = (count - start).min(16);
        let mut planes = [i8x16_splat(0); 4];
        for c in 0..4 {
            let plane = &buffer[c * count + start..c * count + start + n];
            let mut input = [0; 16];
            let v = if let Some(full) = plane.first_chunk::<16>() {
                load(full)
            } else {
                input[..n].copy_from_slice(plane);
                load(&input)
            };
            let mut r = v128_xor(
                v128_and(u16x8_shr(v, 1), i8x16_splat(127)),
                i8x16_sub(i8x16_splat(0), v128_and(v, i8x16_splat(1))),
            );
            r = i8x16_add(
                r,
                i8x16_shuffle::<16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14>(
                    r,
                    i8x16_splat(0),
                ),
            );
            r = i8x16_add(
                r,
                i8x16_shuffle::<16, 16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13>(
                    r,
                    i8x16_splat(0),
                ),
            );
            r = i8x16_add(
                r,
                i8x16_shuffle::<16, 16, 16, 16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11>(
                    r,
                    i8x16_splat(0),
                ),
            );
            r = i8x16_add(
                r,
                i8x16_shuffle::<16, 16, 16, 16, 16, 16, 16, 16, 0, 1, 2, 3, 4, 5, 6, 7>(
                    r,
                    i8x16_splat(0),
                ),
            );
            r = i8x16_add(r, i8x16_splat(previous[c] as i8));
            previous[c] = if n == 16 {
                i8x16_extract_lane::<15>(r) as u8
            } else {
                let mut bytes = [0; 16];
                store(&mut bytes, r);
                bytes[n - 1]
            };
            planes[c] = r;
        }
        let ab0 = i8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(
            planes[0], planes[1],
        );
        let ab1 = i8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(
            planes[0], planes[1],
        );
        let cd0 = i8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(
            planes[2], planes[3],
        );
        let cd1 = i8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(
            planes[2], planes[3],
        );
        let packed = [
            i8x16_shuffle::<0, 1, 16, 17, 2, 3, 18, 19, 4, 5, 20, 21, 6, 7, 22, 23>(ab0, cd0),
            i8x16_shuffle::<8, 9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31>(ab0, cd0),
            i8x16_shuffle::<0, 1, 16, 17, 2, 3, 18, 19, 4, 5, 20, 21, 6, 7, 22, 23>(ab1, cd1),
            i8x16_shuffle::<8, 9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31>(ab1, cd1),
        ];
        if stride == 4 && n == 16 {
            for (chunk, v) in target[start * 4..(start + 16) * 4]
                .as_chunks_mut::<16>()
                .0
                .iter_mut()
                .zip(packed)
            {
                store(chunk, v);
            }
            continue;
        }
        let mut bytes = [0; 64];
        for (chunk, v) in bytes.as_chunks_mut::<16>().0.iter_mut().zip(packed) {
            store(chunk, v);
        }
        for i in 0..n {
            target[(start + i) * stride..(start + i) * stride + 4]
                .copy_from_slice(&bytes[i * 4..i * 4 + 4]);
        }
    }
}
