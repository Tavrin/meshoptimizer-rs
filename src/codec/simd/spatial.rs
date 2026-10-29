//! Exact BVH prefix/suffix box sweeps. Only initialized four-float arrays cross
//! the ISA boundary. Lane 3 is padding and never contributes to an area.
use crate::meshlet_spatial::Box3;

pub(crate) fn areas(boxes: &[Box3], order: &[u32], left: &mut [f32], right: &mut [f32]) -> bool {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = super::dispatch::baseline() {
        arch::areas(token, boxes, order, left, right);
        return true;
    }
    let _ = (boxes, order, left, right);
    false
}

#[cfg(target_arch = "x86_64")]
use x86_64 as arch;
#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::super::dispatch::Baseline;
    use super::Box3;
    use core::arch::x86_64::*;

    #[target_feature(enable = "sse2")]
    fn load(a: &[f32; 4]) -> __m128 {
        // SAFETY: a contains four initialized floats; unaligned load reads exactly 16 bytes.
        unsafe { _mm_loadu_ps(a.as_ptr()) }
    }
    #[target_feature(enable = "sse2")]
    fn min(a: __m128, b: __m128) -> __m128 {
        _mm_min_ps(a, b)
    }
    #[target_feature(enable = "sse2")]
    fn max(a: __m128, b: __m128) -> __m128 {
        _mm_max_ps(a, b)
    }
    #[target_feature(enable = "sse2")]
    fn area(lo: __m128, hi: __m128) -> f32 {
        let d = _mm_sub_ps(hi, lo);
        let p = _mm_mul_ps(d, _mm_shuffle_ps::<0xc9>(d, d));
        _mm_cvtss_f32(_mm_add_ss(
            _mm_add_ss(p, _mm_shuffle_ps::<0x55>(p, p)),
            _mm_shuffle_ps::<0xaa>(p, p),
        ))
    }
    pub(super) fn areas(
        _token: Baseline,
        boxes: &[Box3],
        order: &[u32],
        left: &mut [f32],
        right: &mut [f32],
    ) {
        // SAFETY: Baseline proves sse2; the kernel uses checked indexing and array-backed loads.
        unsafe { sweep(boxes, order, left, right) }
    }
    #[target_feature(enable = "sse2")]
    fn sweep(boxes: &[Box3], order: &[u32], left: &mut [f32], right: &mut [f32]) {
        let mut ll = _mm_set1_ps(f32::MAX);
        let mut lh = _mm_set1_ps(-f32::MAX);
        let mut rl = ll;
        let mut rh = lh;
        for i in 0..order.len() {
            let l = &boxes[order[i] as usize];
            let r = &boxes[order[order.len() - 1 - i] as usize];
            ll = min(ll, load(&l.lo));
            lh = max(lh, load(&l.hi));
            rl = min(rl, load(&r.lo));
            rh = max(rh, load(&r.hi));
            left[i] = area(ll, lh);
            right[i] = area(rl, rh);
        }
    }
}

#[cfg(target_arch = "aarch64")]
use aarch64 as arch;
#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::super::dispatch::Baseline;
    use super::Box3;
    use core::arch::aarch64::*;

    #[target_feature(enable = "neon")]
    fn load(a: &[f32; 4]) -> float32x4_t {
        // SAFETY: a contains four initialized floats; unaligned load reads exactly 16 bytes.
        unsafe { vld1q_f32(a.as_ptr()) }
    }
    #[target_feature(enable = "neon")]
    fn min(a: float32x4_t, b: float32x4_t) -> float32x4_t {
        vbslq_f32(vcltq_f32(a, b), a, b)
    }
    #[target_feature(enable = "neon")]
    fn max(a: float32x4_t, b: float32x4_t) -> float32x4_t {
        vbslq_f32(vcgtq_f32(a, b), a, b)
    }
    #[target_feature(enable = "neon")]
    fn area(lo: float32x4_t, hi: float32x4_t) -> f32 {
        let d = vsubq_f32(hi, lo);
        let x = vgetq_lane_f32::<0>(d);
        let y = vgetq_lane_f32::<1>(d);
        let z = vgetq_lane_f32::<2>(d);
        (x * y + y * z) + z * x
    }
    pub(super) fn areas(
        _token: Baseline,
        boxes: &[Box3],
        order: &[u32],
        left: &mut [f32],
        right: &mut [f32],
    ) {
        // SAFETY: Baseline proves neon; the kernel uses checked indexing and array-backed loads.
        unsafe { sweep(boxes, order, left, right) }
    }
    #[target_feature(enable = "neon")]
    fn sweep(boxes: &[Box3], order: &[u32], left: &mut [f32], right: &mut [f32]) {
        let mut ll = vdupq_n_f32(f32::MAX);
        let mut lh = vdupq_n_f32(-f32::MAX);
        let mut rl = ll;
        let mut rh = lh;
        for i in 0..order.len() {
            let l = &boxes[order[i] as usize];
            let r = &boxes[order[order.len() - 1 - i] as usize];
            ll = min(ll, load(&l.lo));
            lh = max(lh, load(&l.hi));
            rl = min(rl, load(&r.lo));
            rh = max(rh, load(&r.hi));
            left[i] = area(ll, lh);
            right[i] = area(rl, rh);
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use wasm32 as arch;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm32 {
    use super::super::dispatch::Baseline;
    use super::Box3;
    use core::arch::wasm32::*;

    #[target_feature(enable = "simd128")]
    fn load(a: &[f32; 4]) -> v128 {
        // SAFETY: a contains four initialized floats; unaligned load reads exactly 16 bytes.
        unsafe { v128_load(a.as_ptr().cast()) }
    }
    #[target_feature(enable = "simd128")]
    fn min(a: v128, b: v128) -> v128 {
        v128_bitselect(a, b, f32x4_lt(a, b))
    }
    #[target_feature(enable = "simd128")]
    fn max(a: v128, b: v128) -> v128 {
        v128_bitselect(a, b, f32x4_gt(a, b))
    }
    #[target_feature(enable = "simd128")]
    fn area(lo: v128, hi: v128) -> f32 {
        let d = f32x4_sub(hi, lo);
        let p = f32x4_mul(d, i32x4_shuffle::<1, 2, 0, 3>(d, d));
        (f32x4_extract_lane::<0>(p) + f32x4_extract_lane::<1>(p)) + f32x4_extract_lane::<2>(p)
    }
    pub(super) fn areas(
        _token: Baseline,
        boxes: &[Box3],
        order: &[u32],
        left: &mut [f32],
        right: &mut [f32],
    ) {
        // simd128 is enabled for the whole crate here, so this call is safe Rust;
        // Baseline still gates dispatch and the kernel uses checked indexing.
        sweep(boxes, order, left, right)
    }
    #[target_feature(enable = "simd128")]
    fn sweep(boxes: &[Box3], order: &[u32], left: &mut [f32], right: &mut [f32]) {
        let mut ll = f32x4_splat(f32::MAX);
        let mut lh = f32x4_splat(-f32::MAX);
        let mut rl = ll;
        let mut rh = lh;
        for i in 0..order.len() {
            let l = &boxes[order[i] as usize];
            let r = &boxes[order[order.len() - 1 - i] as usize];
            ll = min(ll, load(&l.lo));
            lh = max(lh, load(&l.hi));
            rl = min(rl, load(&r.lo));
            rh = max(rh, load(&r.hi));
            left[i] = area(ll, lh);
            right[i] = area(rl, rh);
        }
    }
}
