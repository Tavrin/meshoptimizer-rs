//! Exact invariant division for the BVH's validated small positive divisors.
#[derive(Clone, Copy)]
pub(crate) struct Divisor {
    value: usize,
    reciprocal: u64,
}
impl Divisor {
    #[inline(always)]
    pub(crate) fn new(value: usize) -> Self {
        debug_assert!((1..=512).contains(&value));
        Self {
            value,
            reciprocal: (u64::MAX / value as u64).wrapping_add(1),
        }
    }
    #[inline(always)]
    pub(crate) fn quotient(self, n: usize) -> usize {
        if self.value == 1 {
            return n;
        }
        // ceil(2^64 / d) exceeds the real reciprocal by less than one. For
        // 0 <= n < 2^64 its high product is floor(n/d) or one greater.
        let q = ((n as u128 * self.reciprocal as u128) >> 64) as usize;
        // An overestimate has a negative remainder in [-(d-1), -1]. Its
        // wrapping representation exceeds d (d <= 512), even if q*d wraps.
        let remainder = n.wrapping_sub(q.wrapping_mul(self.value));
        q - usize::from(remainder >= self.value)
    }
}
