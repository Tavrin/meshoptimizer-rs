//! Exact independent geometry lanes. No reduction across candidates.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128"),
    all(test, feature = "std")
))]
use super::dispatch;

/// Only nonnegative finite operands use the hardware root. Other bit patterns
/// retain the caller's pinned backend, including its NaN and signed-zero rules.
#[inline]
pub(crate) fn roots(values: [f32; 8]) -> Option<[f32; 8]> {
    if !values.iter().all(|v| v.to_bits() <= f32::MAX.to_bits()) {
        return None;
    }
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = dispatch::baseline() {
        // SAFETY: Baseline proves the selected backend's ISA. Both complete
        // initialized arrays are accessed only through fixed four-float windows.
        return Some(unsafe { arch::roots(token, values) });
    }
    None
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
macro_rules! geometry {
    ($feature:literal) => {
        #[inline]
        #[target_feature(enable = $feature)]
        pub(super) fn roots(_: super::dispatch::Baseline, values: [f32; 8]) -> [f32; 8] {
            let mut result = [0.; 8];
            for (input, output) in values
                .as_chunks::<4>()
                .0
                .iter()
                .zip(result.as_chunks_mut::<4>().0)
            {
                store(output, sqrt(load(input)));
            }
            result
        }
    };
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use core::arch::x86_64::*;
    #[inline]
    #[target_feature(enable = "sse2")]
    fn load(v: &[f32; 4]) -> __m128 {
        // SAFETY: v supplies exactly 16 initialized readable bytes. loadu
        // permits unaligned input and retains no pointer.
        unsafe { _mm_loadu_ps(v.as_ptr()) }
    }
    #[inline]
    #[target_feature(enable = "sse2")]
    fn store(v: &mut [f32; 4], x: __m128) {
        // SAFETY: v supplies exactly 16 exclusive writable bytes. storeu
        // permits unaligned output and retains no pointer.
        unsafe { _mm_storeu_ps(v.as_mut_ptr(), x) }
    }
    #[inline]
    #[target_feature(enable = "sse2")]
    fn sqrt(v: __m128) -> __m128 {
        _mm_sqrt_ps(v)
    }
    geometry!("sse2");
}
#[cfg(target_arch = "aarch64")]
mod arch {
    use core::arch::aarch64::*;
    #[inline]
    #[target_feature(enable = "neon")]
    fn load(v: &[f32; 4]) -> float32x4_t {
        // SAFETY: v supplies 16 initialized readable bytes, with the alignment
        // permitted by vld1q_f32. The pointer is not retained.
        unsafe { vld1q_f32(v.as_ptr()) }
    }
    #[inline]
    #[target_feature(enable = "neon")]
    fn store(v: &mut [f32; 4], x: float32x4_t) {
        // SAFETY: v supplies 16 exclusive writable bytes, with the alignment
        // permitted by vst1q_f32. The pointer is not retained.
        unsafe { vst1q_f32(v.as_mut_ptr(), x) }
    }
    #[inline]
    #[target_feature(enable = "neon")]
    fn sqrt(v: float32x4_t) -> float32x4_t {
        vsqrtq_f32(v)
    }
    geometry!("neon");
}
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod arch {
    use core::arch::wasm32::*;
    #[inline]
    #[target_feature(enable = "simd128")]
    fn load(v: &[f32; 4]) -> v128 {
        // SAFETY: v supplies 16 initialized readable bytes. v128_load permits
        // unaligned input and retains no pointer.
        unsafe { v128_load(v.as_ptr().cast()) }
    }
    #[inline]
    #[target_feature(enable = "simd128")]
    fn store(v: &mut [f32; 4], x: v128) {
        // SAFETY: v supplies 16 exclusive writable bytes. v128_store permits
        // unaligned output and retains no pointer.
        unsafe { v128_store(v.as_mut_ptr().cast(), x) }
    }
    #[inline]
    #[target_feature(enable = "simd128")]
    fn sqrt(v: v128) -> v128 {
        f32x4_sqrt(v)
    }
    geometry!("simd128");
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use dispatch::{with_level, Level};

    #[test]
    fn roots_match_pinned_bits_and_preserve_scalar_fallback() {
        for level in [
            Level::Sse2,
            Level::Ssse3,
            Level::Sse41,
            Level::Neon,
            Level::Wasm,
        ] {
            let _ = with_level(level, || {
                let special = [
                    0,
                    1,
                    0x007f_ffff,
                    0x0080_0000,
                    0x3f80_0000,
                    0x4000_0000,
                    0x7f7f_fffe,
                    0x7f7f_ffff,
                ];
                let input = special.map(f32::from_bits);
                assert_eq!(
                    roots(input).unwrap().map(f32::to_bits),
                    input.map(crate::math::sqrt).map(f32::to_bits)
                );
                let mut seed = 0x0216_0003u32;
                for _ in 0..4096 {
                    let input = core::array::from_fn(|_| {
                        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                        f32::from_bits(seed % 0x7f80_0000)
                    });
                    assert_eq!(
                        roots(input).unwrap().map(f32::to_bits),
                        input.map(crate::math::sqrt).map(f32::to_bits)
                    );
                }
                for bits in [
                    0x8000_0000,
                    0xbf80_0000,
                    0x7f80_0000,
                    0xff80_0000,
                    0x7fc0_0123,
                ] {
                    let mut input = [1.; 8];
                    input[3] = f32::from_bits(bits);
                    assert!(roots(input).is_none());
                }
            });
        }
        with_level(Level::Scalar, || {
            assert!(roots([1.; 8]).is_none());
        })
        .unwrap();
    }
}
