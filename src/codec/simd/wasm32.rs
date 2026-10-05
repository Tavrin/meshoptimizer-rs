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
fn trunc(v: v128) -> [i32; 4] {
    let mut b = [0; 16];
    store(&mut b, i32x4_trunc_sat_f32x4(v));
    core::array::from_fn(|i| i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}
#[target_feature(enable = "simd128")]
fn expand(data: &[u8; 16], mask: &[u8; 16], fields: &[u8; 16], out: &mut [u8; 16]) {
    store(
        out,
        v128_or(i8x16_swizzle(load(data), load(mask)), load(fields)),
    );
}
filter_kernel!("simd128");
portable_group!("simd128");
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
