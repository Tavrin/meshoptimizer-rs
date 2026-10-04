// One pinned backend for std and no_std geometric operations.
pub(crate) fn sqrt(value: f32) -> f32 {
    libm::sqrtf(value)
}

/// Lanes of [`sqrt8`].
pub(crate) const LANES: usize = 8;

/// The pinned, correctly rounded roots of eight independent operands, equal to
/// `sqrt` element by element (D78).
///
/// When every operand is a positive normal finite value, each lane uses only
/// exact IEEE f64 operations, which the compiler can evaluate in vector
/// registers: a halved-exponent estimate within [s, 1.0614 s], three Newton
/// steps from above (relative error below 1e-11), rounding to f32, then a
/// correction by one ulp decided by exact f64 squares of the two adjacent
/// rounding midpoints (25-bit midpoints have exact 50-bit squares; a midpoint
/// square never equals an f32 operand). Any other operand sends the chunk to
/// the scalar backend. All 2^32 operand bit patterns were checked against
/// `libm::sqrtf` with zero mismatches; `sqrt8_matches_backend` samples them.
#[inline(always)]
pub(crate) fn sqrt8(values: [f32; LANES]) -> [f32; LANES] {
    if !values
        .iter()
        .all(|v| v.to_bits().wrapping_sub(0x0080_0000) < 0x7f00_0000)
    {
        return values.map(sqrt);
    }
    let mut operand = [0f64; LANES];
    let mut root = [0f64; LANES];
    for i in 0..LANES {
        operand[i] = values[i] as f64;
        root[i] = f64::from_bits((operand[i].to_bits() >> 1) + 0x1ff8_0000_0000_0000);
    }
    for _ in 0..3 {
        for i in 0..LANES {
            root[i] = 0.5 * (root[i] + operand[i] / root[i]);
        }
    }
    let mut result = [0f32; LANES];
    for i in 0..LANES {
        let rounded = root[i] as f32;
        let bits = rounded.to_bits();
        let r = rounded as f64;
        let up = (f32::from_bits(bits + 1) as f64 + r) * 0.5;
        let down = (f32::from_bits(bits - 1) as f64 + r) * 0.5;
        let bits = bits + u32::from(operand[i] > up * up) - u32::from(operand[i] < down * down);
        result[i] = f32::from_bits(bits);
    }
    result
}

/// Cheap bounds on the pinned, correctly rounded square root, used to decide
/// comparisons without computing it.
///
/// Write a positive normal operand in [2^-100, f32::MAX) as 4^k * f with f in
/// [1, 4). `table` reads precomputed bounds of sqrt(f) over the operand's
/// 1/128-wide mantissa bucket (relative width below 0.4%) and scales them by
/// 2^k exactly. For tighter bounds, halving the exponent bits gives an
/// estimate `e` in [s, 1.0614 s] of the real root s; one Newton step from
/// above stays above s, within relative error 0.0614^2 / 2.1228 < 0.00178 in
/// real arithmetic, plus at most 4e-7 for its three roundings, so
/// `n * NEWTON_LOW <= s <= n * NEWTON_HIGH`. The final products round by at
/// most half an ulp, inside the margins. A correctly rounded root cannot
/// cross a representable bound enclosing s. `bounds_hold_for_every_operand_class`
/// samples the whole operand range; the D78 record retains an exhaustive
/// check of all in-range operands.
#[derive(Clone, Copy)]
pub(crate) struct RootEstimate {
    pub(crate) value: f32,
    pub(crate) estimate: f32,
}
pub(crate) const NEWTON_LOW: f32 = 0.998;
pub(crate) const NEWTON_HIGH: f32 = 1.000_001;
// sqrt(x) in f64 for x in [1, 4], by Newton from above (const evaluation).
const fn table_root(x: f64) -> f64 {
    let mut r = 2.0;
    let mut i = 0;
    while i < 64 {
        r = 0.5 * (r + x / r);
        i += 1;
    }
    r
}
// Entry p * 128 + j bounds sqrt(f) for f in [2^p (1 + j/128), 2^p (1 + (j+1)/128)].
// The f64 roots are exact to about 1e-16; the 1e-6 margins exceed that and
// the f32 rounding of each entry.
const fn root_table(high: bool) -> [f32; 256] {
    let mut table = [0f32; 256];
    let mut i = 0;
    while i < 256 {
        let scale = if i >= 128 { 2.0 } else { 1.0 };
        let j = (i % 128) as f64;
        table[i] = if high {
            (table_root(scale * (1.0 + (j + 1.0) / 128.0)) * (1.0 + 1e-6)) as f32
        } else {
            (table_root(scale * (1.0 + j / 128.0)) * (1.0 - 1e-6)) as f32
        };
        i += 1;
    }
    table
}
static ROOT_LOW: [f32; 256] = root_table(false);
static ROOT_HIGH: [f32; 256] = root_table(true);
// Bounds of 1 / sqrt(f) over the same buckets. The 1e-6 margins also cover
// the two roundings of fl(1 / fl(sqrt(x))) (each at most 2^-24 relative).
const fn inverse_root_table(high: bool) -> [f32; 256] {
    let mut table = [0f32; 256];
    let mut i = 0;
    while i < 256 {
        let scale = if i >= 128 { 2.0 } else { 1.0 };
        let j = (i % 128) as f64;
        table[i] = if high {
            (1.0 / table_root(scale * (1.0 + j / 128.0)) * (1.0 + 1e-6)) as f32
        } else {
            (1.0 / table_root(scale * (1.0 + (j + 1.0) / 128.0)) * (1.0 - 1e-6)) as f32
        };
        i += 1;
    }
    table
}
static INVERSE_LOW: [f32; 256] = inverse_root_table(false);
static INVERSE_HIGH: [f32; 256] = inverse_root_table(true);
impl RootEstimate {
    #[inline(always)]
    pub(crate) fn new(value: f32) -> Option<Self> {
        let bits = value.to_bits();
        if !(0x0d80_0000..0x7f80_0000).contains(&bits) {
            return None;
        }
        Some(Self {
            value,
            estimate: f32::from_bits((bits >> 1) + 0x1fc0_0000),
        })
    }
    /// Table bounds `(low, high)`, without a division.
    #[inline(always)]
    pub(crate) fn table(self) -> (f32, f32) {
        let bits = self.value.to_bits();
        let biased = bits >> 23;
        // value = 2^(biased - 127) (1 + m) = 4^k * 2^p (1 + m)
        let p = (biased & 1) ^ 1;
        let k = (biased as i32 - 127 - p as i32) >> 1;
        let index = (p << 7 | (bits >> 16) & 127) as usize;
        let scale = f32::from_bits(((k + 127) as u32) << 23);
        (ROOT_LOW[index] * scale, ROOT_HIGH[index] * scale)
    }
    /// Table bounds `(low, high)` of the rounded reciprocal of the pinned
    /// root, `fl(1 / sqrt(value))`, without a division.
    #[inline(always)]
    pub(crate) fn inverse_table(self) -> (f32, f32) {
        let bits = self.value.to_bits();
        let biased = bits >> 23;
        let p = (biased & 1) ^ 1;
        let k = (biased as i32 - 127 - p as i32) >> 1;
        let index = (p << 7 | (bits >> 16) & 127) as usize;
        let scale = f32::from_bits(((127 - k) as u32) << 23);
        (INVERSE_LOW[index] * scale, INVERSE_HIGH[index] * scale)
    }
    /// One Newton step from the bit estimate (see `bounds`).
    #[inline(always)]
    pub(crate) fn newton(self) -> f32 {
        0.5 * (self.estimate + self.value / self.estimate)
    }
    /// Tight `(low, high)` bounds after one Newton step.
    #[inline(always)]
    pub(crate) fn bounds(self) -> (f32, f32) {
        let newton = self.newton();
        (newton * NEWTON_LOW, newton * NEWTON_HIGH)
    }
}

/// True only if the pinned root of `square` is at least `limit`; false means
/// undecided. Used to skip a candidate whose exact distance cannot win.
#[inline(always)]
pub(crate) fn root_at_least(square: f32, limit: f32) -> bool {
    match RootEstimate::new(square) {
        Some(root) => root.table().0 >= limit || root.bounds().0 >= limit,
        None => false,
    }
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
    fn sqrt8_matches_backend() {
        // Every exponent with a mantissa stride, special values, and mixed
        // chunks that take the scalar path.
        let mut values = alloc::vec::Vec::new();
        for exponent in 0u32..512 {
            for mantissa in (0u32..1 << 23).step_by(8191).chain([1, 0x7fffff]) {
                values.push(f32::from_bits(exponent << 23 | mantissa));
            }
        }
        values.extend([0.0, -0.0, f32::INFINITY, f32::NAN, f32::MIN_POSITIVE, 1.0]);
        for chunk in values.chunks(LANES) {
            let mut lanes = [1.0f32; LANES];
            lanes[..chunk.len()].copy_from_slice(chunk);
            for (root, value) in sqrt8(lanes).iter().zip(lanes) {
                assert_eq!(
                    root.to_bits(),
                    sqrt(value).to_bits(),
                    "{:08x}",
                    value.to_bits()
                );
            }
        }
    }
    #[test]
    fn bounds_hold_for_every_operand_class() {
        // Every exponent, both mantissa ends and a stride through the middle.
        let mut checked = 0;
        for exponent in 0u32..256 {
            for mantissa in (0u32..1 << 23).step_by(4093).chain([0x7fffff]) {
                let bits = exponent << 23 | mantissa;
                let value = f32::from_bits(bits);
                let Some(root) = RootEstimate::new(value) else {
                    assert!(!(0x0d80_0000..0x7f80_0000).contains(&bits));
                    continue;
                };
                let s = sqrt(value);
                let (low, high) = root.bounds();
                let (table_low, table_high) = root.table();
                assert!(table_low <= s && s <= table_high, "{bits:08x}");
                let (inverse_low, inverse_high) = root.inverse_table();
                let inverse = 1. / s;
                assert!(
                    inverse_low <= inverse && inverse <= inverse_high,
                    "{bits:08x}"
                );
                assert!(low <= s && s <= high, "{bits:08x}");
                checked += 1;
            }
        }
        assert!(checked > 400_000);
        assert!(RootEstimate::new(0.0).is_none() && RootEstimate::new(-1.0).is_none());
        assert!(RootEstimate::new(f32::INFINITY).is_none());
    }
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
