use super::dispatch::Baseline;
use core::arch::aarch64::*;
#[inline]
#[target_feature(enable = "neon")]
fn fload(v: [f32; 4]) -> float32x4_t {
    vsetq_lane_f32::<3>(
        v[3],
        vsetq_lane_f32::<2>(v[2], vsetq_lane_f32::<1>(v[1], vdupq_n_f32(v[0]))),
    )
}
#[inline]
#[target_feature(enable = "neon")]
fn splat(v: f32) -> float32x4_t {
    vdupq_n_f32(v)
}
#[inline]
#[target_feature(enable = "neon")]
fn add(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vaddq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn sub(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vsubq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn mul(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vmulq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn div(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vdivq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn sqrt(a: float32x4_t) -> float32x4_t {
    select(ge(a, splat(0.0)), vsqrtq_f32(a), splat(f32::NAN))
}
#[inline]
#[target_feature(enable = "neon")]
fn abs(a: float32x4_t) -> float32x4_t {
    vabsq_f32(a)
}
#[inline]
#[target_feature(enable = "neon")]
fn neg(a: float32x4_t) -> float32x4_t {
    vnegq_f32(a)
}
#[inline]
#[target_feature(enable = "neon")]
fn ge(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vreinterpretq_f32_u32(vcgeq_f32(a, b))
}
#[inline]
#[target_feature(enable = "neon")]
fn select(mask: float32x4_t, a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vbslq_f32(vreinterpretq_u32_f32(mask), a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn load(bytes: &[u8; 16]) -> uint8x16_t {
    // SAFETY: the initialized array supplies 16 readable bytes; vld1q_u8
    // permits byte alignment and the pointer is not retained.
    unsafe { vld1q_u8(bytes.as_ptr()) }
}
#[inline]
#[target_feature(enable = "neon")]
fn store(bytes: &mut [u8; 16], v: uint8x16_t) {
    // SAFETY: the exclusive array supplies 16 writable bytes; vst1q_u8
    // permits byte alignment and the pointer is not retained.
    unsafe { vst1q_u8(bytes.as_mut_ptr(), v) }
}
#[target_feature(enable = "neon")]
fn fout(v: float32x4_t) -> [f32; 4] {
    let mut b = [0; 16];
    store(&mut b, vreinterpretq_u8_f32(v));
    core::array::from_fn(|i| {
        f32::from_bits(u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
    })
}
#[target_feature(enable = "neon")]
fn trunc(v: float32x4_t) -> [i32; 4] {
    let mut b = [0; 16];
    store(&mut b, vreinterpretq_u8_s32(vcvtq_s32_f32(v)));
    core::array::from_fn(|i| i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}
#[target_feature(enable = "neon")]
fn expand(data: &[u8; 16], mask: &[u8; 16], fields: &[u8; 16], out: &mut [u8; 16]) {
    store(
        out,
        vorrq_u8(vqtbl1q_u8(load(data), load(mask)), load(fields)),
    );
}
filter_kernel!("neon");
portable_group!("neon");
pub(super) fn filter(
    _token: Baseline,
    kind: u8,
    data: &mut [u8],
    stride: usize,
) -> Result<(), crate::Error> {
    // SAFETY: Baseline is issued only on this architecture with neon
    // enabled; the safe kernel checks all slice bounds.
    unsafe { filter_kernel(kind, data, stride) }
}
pub(super) fn group(_token: Baseline, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Baseline proves neon availability; complete initialized
    // input and exclusive output arrays supply all memory used by the kernel.
    unsafe { group_kernel(data, out, bits) }
}
pub(super) fn meshlet(_token: Baseline, data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    // SAFETY: Baseline proves AArch64 NEON, and the complete input is typed.
    unsafe { meshlet_kernel(data, code, last) }
}
#[target_feature(enable = "neon")]
fn meshlet_kernel(data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    let (mask, used) = super::meshlet_mask(code);
    let v = vreinterpretq_u32_u8(vqtbl1q_u8(load(data), load(&mask)));
    let d = veorq_u32(
        vshrq_n_u32::<1>(v),
        vsubq_u32(vdupq_n_u32(0), vandq_u32(v, vdupq_n_u32(1))),
    );
    let mut r = vaddq_u32(d, vdupq_n_u32(1));
    r = vaddq_u32(r, vextq_u32::<3>(vdupq_n_u32(0), r));
    r = vaddq_u32(r, vextq_u32::<2>(vdupq_n_u32(0), r));
    r = vaddq_u32(r, vdupq_n_u32(last));
    let mut out = [0; 16];
    store(&mut out, vreinterpretq_u8_u32(r));
    (
        core::array::from_fn(|i| u32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().unwrap())),
        used,
    )
}

#[cfg(feature = "parity-internals")]
pub(super) fn sqrt4(_token: Baseline, values: [f32; 4]) -> [f32; 4] {
    // SAFETY: Baseline proves this backend's ISA; the kernel uses only values
    // and the already audited fixed-array store helper.
    unsafe { sqrt4_kernel(values) }
}
#[cfg(feature = "parity-internals")]
#[target_feature(enable = "neon")]
fn sqrt4_kernel(values: [f32; 4]) -> [f32; 4] {
    fout(sqrt(fload(values)))
}

#[target_feature(enable = "neon")]
fn exp_kernel(data: &mut [u8]) {
    let (chunks, tail) = data.as_chunks_mut::<16>();
    for chunk in chunks {
        let v = vreinterpretq_s32_u8(load(chunk));
        let m = vcvtq_f32_s32(vshrq_n_s32::<8>(vshlq_n_s32::<8>(v)));
        let e = vshlq_n_s32::<23>(vaddq_s32(vshrq_n_s32::<24>(v), vdupq_n_s32(127)));
        store(
            chunk,
            vreinterpretq_u8_f32(vmulq_f32(vreinterpretq_f32_s32(e), m)),
        );
    }
    crate::codec::filter::scalar_exp(tail);
}

#[target_feature(enable = "neon")]
fn color_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let (y, co, cg, alpha) = if W == 4 {
            let v = vreinterpretq_s32_u8(load(block.first_chunk::<16>().unwrap()));
            (
                vandq_s32(v, vdupq_n_s32(255)),
                vshrq_n_s32::<24>(vshlq_n_s32::<16>(v)),
                vshrq_n_s32::<24>(vshlq_n_s32::<8>(v)),
                vreinterpretq_s32_u32(vshrq_n_u32::<24>(vreinterpretq_u32_s32(v))),
            )
        } else {
            let (lo, hi) = {
                let a = vreinterpretq_s32_u8(load(block[..16].first_chunk().unwrap()));
                let b = vreinterpretq_s32_u8(load(block[16..].first_chunk().unwrap()));
                (vuzp1q_s32(a, b), vuzp2q_s32(a, b))
            };
            (
                vandq_s32(lo, vdupq_n_s32(65535)),
                vshrq_n_s32::<16>(lo),
                vshrq_n_s32::<16>(vshlq_n_s32::<16>(hi)),
                vreinterpretq_s32_u32(vshrq_n_u32::<16>(vreinterpretq_u32_s32(hi))),
            )
        };
        let mut scale = alpha;
        scale = vorrq_s32(scale, vshrq_n_s32::<1>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<2>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<4>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<8>(scale));
        let a = vorrq_s32(
            vandq_s32(vshlq_n_s32::<1>(alpha), scale),
            vandq_s32(alpha, vdupq_n_s32(1)),
        );
        let ss = div(
            splat(if W == 4 { 255.0 } else { 65535.0 }),
            vcvtq_f32_s32(scale),
        );
        let channels = [
            vsubq_s32(vaddq_s32(y, co), cg),
            vaddq_s32(y, cg),
            vsubq_s32(vsubq_s32(y, co), cg),
            a,
        ];
        let f = channels.map(|v| add(mul(vcvtq_f32_s32(v), ss), splat(0.5)));
        let mut mask = vdupq_n_u32(u32::MAX);
        for v in f {
            mask = vandq_u32(
                mask,
                vandq_u32(
                    vcgeq_f32(v, splat(-2147483648.0)),
                    vcltq_f32(v, splat(2147483648.0)),
                ),
            );
        }
        let valid = vminvq_u32(mask) == u32::MAX;
        if !valid {
            crate::codec::filter::scalar_color(block, W)?;
            continue;
        }
        let [r, g, b, a] = f.map(|v| vcvtq_s32_f32(v));
        if W == 4 {
            let value = vorrq_s32(
                vorrq_s32(
                    vandq_s32(r, vdupq_n_s32(255)),
                    vshlq_n_s32::<8>(vandq_s32(g, vdupq_n_s32(255))),
                ),
                vorrq_s32(
                    vshlq_n_s32::<16>(vandq_s32(b, vdupq_n_s32(255))),
                    vshlq_n_s32::<24>(a),
                ),
            );
            store(
                block.first_chunk_mut::<16>().unwrap(),
                vreinterpretq_u8_s32(value),
            );
        } else {
            let lo = vorrq_s32(vandq_s32(r, vdupq_n_s32(65535)), vshlq_n_s32::<16>(g));
            let hi = vorrq_s32(vandq_s32(b, vdupq_n_s32(65535)), vshlq_n_s32::<16>(a));
            store(
                block[..16].first_chunk_mut().unwrap(),
                vreinterpretq_u8_s32(vzip1q_s32(lo, hi)),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                vreinterpretq_u8_s32(vzip2q_s32(lo, hi)),
            );
        }
    }
    crate::codec::filter::scalar_color(tail, W)
}

triangle_kernel!("neon");
#[target_feature(enable = "neon")]
fn triangle_step(state: &mut [u8; 16], extra: &[u8; 16], shuf: &[u8; 16], next: &[u8; 16]) {
    let mask = [255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let s = vbslq_u8(load(&mask), load(extra), load(state));
    store(state, vaddq_u8(vqtbl1q_u8(s, load(shuf)), load(next)));
}
pub(super) fn triangles(
    _token: Baseline,
    source: &[u8],
    bound: usize,
    codes: &[u8],
    data: usize,
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Option<Result<usize, crate::Error>> {
    // SAFETY: the dispatch token proves this kernel's ISA, and the kernel
    // checks each complete input window and scalar-equivalent bound before use.
    unsafe { triangles_kernel(source, bound, codes, data, count, out) }
}
