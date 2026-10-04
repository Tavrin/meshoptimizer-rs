// One pinned backend for std and no_std geometric operations.
pub(crate) fn sqrt(value: f32) -> f32 {
    libm::sqrtf(value)
}

// Per-call memoization of the pinned backend, keyed by the complete operand.
pub(crate) struct SqrtCache {
    entries: [(u32, f32); 512],
}
impl SqrtCache {
    pub(crate) const fn new() -> Self {
        // Every initial entry is already the correct result for positive zero.
        Self {
            entries: [(0, 0.0); 512],
        }
    }
    #[inline(always)]
    pub(crate) fn sqrt(&mut self, value: f32) -> f32 {
        let bits = value.to_bits();
        let bucket = (bits.wrapping_mul(0x9e3779b9) >> 23) as usize;
        let entry = &mut self.entries[bucket];
        if entry.0 != bits {
            *entry = (bits, sqrt(value));
        }
        entry.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_preserves_full_bits_through_collisions_and_special_values() {
        let mut cache = SqrtCache::new();
        let mut compare = |bits| {
            let value = f32::from_bits(bits);
            let expected = sqrt(value).to_bits();
            assert_eq!(cache.sqrt(value).to_bits(), expected);
            assert_eq!(cache.sqrt(value).to_bits(), expected);
        };
        for bits in [
            0, 0x80000000, 1, 0x7fffff, 0x800000, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc00123,
            0x3f800000,
        ] {
            compare(bits);
        }
        let mut previous = [None; 512];
        let mut collisions = 0;
        for bits in 0x3f000000u32..0x3f001000 {
            let bucket = (bits.wrapping_mul(0x9e3779b9) >> 23) as usize;
            if let Some(old) = previous[bucket] {
                compare(old);
                compare(bits);
                collisions += 1;
            }
            previous[bucket] = Some(bits);
        }
        assert!(collisions > 3000);
        let mut bits = 0x12345678u32;
        for _ in 0..65536 {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            compare(bits);
        }
    }
}
