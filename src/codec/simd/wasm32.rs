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
        // Rotate each packed record in the vector. The mask repeats its
        // record's two-byte selector, wraps within eight bytes, then adds
        // the second record's eight-byte offset. No scalar u64 rotations.
        let rotations = i32x4_shl(v128_and(i32x4_add(ci, i32x4_splat(1)), i32x4_splat(3)), 1);
        let base = i8x16(0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7);
        let offset = i8x16(0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8);
        let rotate = |packed, shifts| {
            i8x16_swizzle(
                packed,
                v128_or(v128_and(i8x16_sub(base, shifts), i8x16_splat(7)), offset),
            )
        };
        store(
            block[..16].first_chunk_mut().unwrap(),
            rotate(
                i32x4_shuffle::<0, 4, 1, 5>(xy, zw),
                i8x16_shuffle::<0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4>(
                    rotations, rotations,
                ),
            ),
        );
        store(
            block[16..].first_chunk_mut().unwrap(),
            rotate(
                i32x4_shuffle::<2, 6, 3, 7>(xy, zw),
                i8x16_shuffle::<8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 12, 12, 12>(
                    rotations, rotations,
                ),
            ),
        );
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
        // For 8-bit words |channel| <= 511, hence nonzero scale proves
        // finite i32 conversion. For 16-bit words |channel| <= 131071;
        // alpha >= 4 gives scale >= 7 and magnitude < 2^31 even after
        // rounding. Smaller alpha uses the scalar conversion/error contract.
        let valid = i32x4_bitmask(i32x4_gt(alpha, i32x4_splat(if W == 4 { 0 } else { 3 }))) == 15;
        if !valid {
            crate::codec::filter::scalar_color(block, W)?;
            continue;
        }
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
#[inline]
#[target_feature(enable = "simd128")]
fn deltas8_kernel(buffer: &[u8], target: &mut [u8], count: usize, stride: usize, last: &[u8]) {
    deltas_kernel::<0>(buffer, target, count, stride, last, 0);
}
#[inline]
#[target_feature(enable = "simd128")]
fn combine<const CHANNEL: u8>(a: v128, b: v128) -> v128 {
    if CHANNEL == 0 {
        i8x16_add(a, b)
    } else if CHANNEL == 1 {
        i16x8_add(a, b)
    } else {
        v128_xor(a, b)
    }
}
#[inline]
#[target_feature(enable = "simd128")]
fn delta_prefix<const CHANNEL: u8>(mut r: v128, previous: v128, rot: u32) -> v128 {
    r = if CHANNEL == 0 {
        v128_xor(
            v128_and(u16x8_shr(r, 1), i8x16_splat(127)),
            i8x16_sub(i8x16_splat(0), v128_and(r, i8x16_splat(1))),
        )
    } else if CHANNEL == 1 {
        v128_xor(
            u16x8_shr(r, 1),
            i16x8_sub(i16x8_splat(0), v128_and(r, i16x8_splat(1))),
        )
    } else {
        v128_or(i32x4_shl(r, rot), u32x4_shr(r, 32 - rot))
    };
    let zero = i32x4_splat(0);
    r = combine::<CHANNEL>(r, i32x4_shuffle::<4, 0, 1, 2>(r, zero));
    r = combine::<CHANNEL>(r, i32x4_shuffle::<4, 4, 0, 1>(r, zero));
    combine::<CHANNEL>(r, previous)
}
#[inline]
#[target_feature(enable = "simd128")]
fn scatter4(dst: &mut [u8], stride: usize, r: v128) {
    if stride == 4 {
        store(dst.first_chunk_mut().unwrap(), r);
    } else {
        dst[..4].copy_from_slice(&i32x4_extract_lane::<0>(r).to_le_bytes());
        dst[stride..stride + 4].copy_from_slice(&i32x4_extract_lane::<1>(r).to_le_bytes());
        dst[stride * 2..stride * 2 + 4].copy_from_slice(&i32x4_extract_lane::<2>(r).to_le_bytes());
        dst[stride * 3..stride * 3 + 4].copy_from_slice(&i32x4_extract_lane::<3>(r).to_le_bytes());
    }
}
#[inline]
#[target_feature(enable = "simd128")]
fn deltas_kernel<const CHANNEL: u8>(
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
    rot: u32,
) {
    let mut previous = i32x4_splat(i32::from_le_bytes(last[..4].try_into().unwrap()));
    let full = count & !15;
    for start in (0..full).step_by(16) {
        let planes = core::array::from_fn::<_, 4, _>(|c| {
            load(buffer[c * count + start..].first_chunk::<16>().unwrap())
        });
        let ab0 = unpack_bytes(planes[0], planes[1]);
        let cd0 = unpack_bytes(planes[2], planes[3]);
        let ab1 = i8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(
            planes[0], planes[1],
        );
        let cd1 = i8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(
            planes[2], planes[3],
        );
        let records = [
            i16x8_shuffle::<0, 8, 1, 9, 2, 10, 3, 11>(ab0, cd0),
            i16x8_shuffle::<4, 12, 5, 13, 6, 14, 7, 15>(ab0, cd0),
            i16x8_shuffle::<0, 8, 1, 9, 2, 10, 3, 11>(ab1, cd1),
            i16x8_shuffle::<4, 12, 5, 13, 6, 14, 7, 15>(ab1, cd1),
        ];
        let dst = &mut target[start * stride..(start + 15) * stride + 4];
        macro_rules! emit {
            ($g:literal) => {{
                let r = delta_prefix::<CHANNEL>(records[$g], previous, rot);
                previous = i32x4_splat(i32x4_extract_lane::<3>(r));
                scatter4(&mut dst[$g * 4 * stride..], stride, r);
            }};
        }
        emit!(0);
        emit!(1);
        emit!(2);
        emit!(3);
    }
    if full < count {
        let mut last = [0; 16];
        store(&mut last, previous);
        let dst = &mut target[full * stride..];
        match CHANNEL {
            0 => crate::codec::vertex::scalar_deltas::<1, false>(
                buffer, dst, count, stride, &last, rot, full,
            ),
            1 => crate::codec::vertex::scalar_deltas::<2, false>(
                buffer, dst, count, stride, &last, rot, full,
            ),
            _ => crate::codec::vertex::scalar_deltas::<4, true>(
                buffer, dst, count, stride, &last, rot, full,
            ),
        }
    }
}

#[inline]
#[target_feature(enable = "simd128")]
fn header_group(data: &[u8; 24], out: &mut [u8; 16], h: usize) -> usize {
    const BITS: [u32; 9] = [0, 2, 4, 8, 0, 1, 2, 4, 8];
    group_kernel(data, out, BITS[h])
}
#[inline]
#[target_feature(enable = "simd128")]
fn bytes_kernel(
    data: &[u8],
    pos: usize,
    out: &mut [u8],
    hshift: usize,
) -> Result<usize, crate::Error> {
    match hshift {
        0 => bytes_header_kernel::<0>(data, pos, out),
        4 => bytes_header_kernel::<4>(data, pos, out),
        5 => bytes_header_kernel::<5>(data, pos, out),
        _ => unreachable!(),
    }
}
#[inline]
#[target_feature(enable = "simd128")]
fn bytes_header_kernel<const H: usize>(
    data: &[u8],
    mut pos: usize,
    out: &mut [u8],
) -> Result<usize, crate::Error> {
    let headers = (out.len() / 16).div_ceil(4);
    let header = data
        .get(pos..pos + headers)
        .ok_or(crate::Error::InvalidStream)?;
    pos += headers;
    let (blocks, tail) = out.as_chunks_mut::<64>();
    for (i, block) in blocks.iter_mut().enumerate() {
        let control = header[i];
        if let Some(window) = data.get(pos..).and_then(|v| v.first_chunk::<96>()) {
            if control == 0 && H != 5 {
                block.fill(0);
                continue;
            }
            if control == 255 && H != 4 {
                block.copy_from_slice(&window[..64]);
                pos += 64;
                continue;
            }
            // Each group consumes at most 24 bytes. A single checked 96-byte
            // array therefore proves all four lookaheads, even full escapes.
            let chunks = block.as_chunks_mut::<16>().0;
            let mut used = header_group(
                window.first_chunk().unwrap(),
                &mut chunks[0],
                H + usize::from(control & 3),
            );
            used += header_group(
                window[used..].first_chunk().unwrap(),
                &mut chunks[1],
                H + usize::from((control >> 2) & 3),
            );
            used += header_group(
                window[used..].first_chunk().unwrap(),
                &mut chunks[2],
                H + usize::from((control >> 4) & 3),
            );
            used += header_group(
                window[used..].first_chunk().unwrap(),
                &mut chunks[3],
                H + usize::from(control >> 6),
            );
            pos += used;
        } else {
            for (j, chunk) in block.as_chunks_mut::<16>().0.iter_mut().enumerate() {
                let window = data
                    .get(pos..)
                    .and_then(|v| v.first_chunk::<24>())
                    .ok_or(crate::Error::InvalidStream)?;
                pos += header_group(window, chunk, H + usize::from((control >> (j * 2)) & 3));
            }
        }
    }
    for (j, chunk) in tail.as_chunks_mut::<16>().0.iter_mut().enumerate() {
        let window = data
            .get(pos..)
            .and_then(|v| v.first_chunk::<24>())
            .ok_or(crate::Error::InvalidStream)?;
        pos += header_group(
            window,
            chunk,
            H + usize::from((header[blocks.len()] >> (j * 2)) & 3),
        );
    }
    Ok(pos)
}

pub(super) fn vertex(
    _token: Baseline,
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    if count <= 32 {
        vertex_kernel::<128>(output, count, stride, data)
    } else {
        vertex_kernel::<1024>(output, count, stride, data)
    }
}
#[target_feature(enable = "simd128")]
fn vertex_kernel<const SCRATCH: usize>(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    let version = crate::codec::decode_vertex_version(data)?;
    let tail = stride + if version == 0 { 0 } else { stride / 4 };
    let padded = tail.max(if version == 0 { 32 } else { 24 });
    if data.len() < 1 + padded {
        return Err(crate::Error::InvalidStream);
    }
    let start = data.len() - tail;
    let channels = &data[start + stride..];
    let mut last = [0u8; 256];
    last[..stride].copy_from_slice(&data[start..start + stride]);
    let block_size = ((8192 / stride) & !15).min(256);
    let mut pos = 1;
    // Four planes, each starting j * block and rounded up to 16 lanes.
    // Up to two 16-record groups fit 128 bytes; full blocks fit 1024.
    let mut deltas = [0u8; SCRATCH];
    let mut offset = 0;
    while offset < count {
        let block = block_size.min(count - offset);
        let aligned = block.div_ceil(16) * 16;
        let controls = if version == 0 {
            &[][..]
        } else {
            let c = data
                .get(pos..pos + stride / 4)
                .ok_or(crate::Error::InvalidStream)?;
            pos += stride / 4;
            c
        };
        // Each bounded block fits 8 KiB. Write directly into its checked
        // destination slice, avoiding zeroing/copying an extra stack block.
        let block_output = &mut output[offset * stride..(offset + block) * stride];
        for k in (0..stride).step_by(4) {
            let control = if version == 0 { 0 } else { controls[k / 4] };
            for j in 0..4 {
                let ctrl = (control >> (j * 2)) & 3;
                let out = &mut deltas[j * block..j * block + aligned];
                match ctrl {
                    3 => {
                        out[..block].copy_from_slice(
                            data.get(pos..pos + block)
                                .ok_or(crate::Error::InvalidStream)?,
                        );
                        pos += block;
                    }
                    2 => out[..block].fill(0),
                    _ => {
                        pos = bytes_kernel(
                            data,
                            pos,
                            out,
                            if version == 0 {
                                0
                            } else {
                                4 + usize::from(ctrl)
                            },
                        )?
                    }
                }
            }
            let channel = if version == 0 { 0 } else { channels[k / 4] };
            let target = &mut block_output[k..];
            // k is a four-byte-aligned channel group; only complete records
            // belong to this group. Extend the target to keep chunk strides
            // complete without touching bytes beyond this group's four bytes.
            let target_len = (block - 1) * stride + 4;
            let target = &mut target[..target_len];
            match channel & 3 {
                0 => deltas8_kernel(&deltas[..block * 4], target, block, stride, &last[k..k + 4]),
                1 => deltas_kernel::<1>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                2 => deltas_kernel::<2>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    (32 - u32::from(channel >> 4)) & 31,
                ),
                _ => return Err(crate::Error::InvalidStream),
            }
        }
        last[..stride].copy_from_slice(&block_output[(block - 1) * stride..]);
        offset += block;
    }
    if data.len().checked_sub(pos) != Some(padded) {
        return Err(crate::Error::InvalidStream);
    }
    Ok(())
}
