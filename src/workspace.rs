use crate::Error;
use alloc::vec::Vec;
use core::mem::size_of;

/// Per-call limits for crate-owned heap storage and counted work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Maximum output plus retained scratch capacity, excluding caller buffers
    /// and allocator overhead. Defaults to 1 GiB.
    pub max_bytes: usize,
    /// Maximum counted record visits. Defaults to 2^34.
    pub max_work: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_bytes: 1 << 30,
            max_work: 1 << 34,
        }
    }
}

/// Resource measurements for the most recent algorithm call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// Crate-owned output bytes plus retained heap scratch capacity.
    pub bytes: usize,
    /// Counted record visits actually performed, including validation.
    pub work: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CacheVertex {
    pub(crate) live: u32,
    pub(crate) offset: u32,
    pub(crate) score: f32,
}

/// Reusable fallibly allocated scratch storage and per-call resource limits.
///
/// Cache work counts topology/index validation, each adjacency initialization,
/// insertion, search and score update, and each cache/emission visit. Overdraw
/// work counts validation, cache simulation, geometry and counting-sort record
/// visits, and copied indices. Fixed scalar arithmetic inside a visit is not
/// counted separately. Simplification counts validation, position/hash and
/// adjacency visits, edge searches, classification, quadric and attribute
/// accumulation, collapse ranking/sorting/flip checks, and remapping visits.
/// Its typed scratch is allocated fallibly and retained after successful calls;
/// usage reports peak storage including all retained workspace buffers. Scale
/// computation is allocation-free and has no workspace argument.
/// Limits never disable checked size arithmetic.
#[derive(Debug, Default)]
pub struct Workspace {
    limits: Limits,
    usage: Usage,
    pub(crate) has_retained: bool,
    pub(crate) simplify: Option<crate::simplify::State>,
    pub(crate) sloppy: Option<crate::simplify::SloppyScratch>,
    pub(crate) vertices: Vec<CacheVertex>,
    pub(crate) integers: Vec<u32>,
    pub(crate) floats: Vec<f32>,
    pub(crate) flags: Vec<u8>,
    pub(crate) keys: Vec<u16>,
}

impl Workspace {
    /// Create an empty workspace with explicit limits.
    pub fn new(limits: Limits) -> Self {
        Self {
            limits,
            ..Self::default()
        }
    }
    /// Current per-call limits.
    pub const fn limits(&self) -> Limits {
        self.limits
    }
    /// Set limits and release scratch so a lower budget is effective immediately.
    pub fn set_limits(&mut self, limits: Limits) {
        self.clear();
        self.limits = limits;
    }
    /// Release all retained scratch allocations and reset measurements.
    pub fn clear(&mut self) {
        self.simplify = None;
        self.sloppy = None;
        self.vertices = Vec::new();
        self.integers = Vec::new();
        self.floats = Vec::new();
        self.flags = Vec::new();
        self.keys = Vec::new();
        self.usage = Usage::default();
        self.has_retained = false;
    }
    /// Measurements for the most recent call, including a failed call's work.
    pub const fn usage(&self) -> Usage {
        self.usage
    }
    #[inline]
    pub(crate) fn begin(&mut self) -> Work {
        self.usage = Usage::default();
        Work {
            remaining: self.limits.max_work,
            limit: self.limits.max_work,
        }
    }
    #[inline]
    pub(crate) fn finish(&mut self, work: &Work) {
        self.usage.work = work.limit - work.remaining;
    }
    // Scratch capacities are stable while a local Budget borrows the workspace.
    #[inline]
    pub(crate) fn retained_bytes(&self) -> Result<usize, Error> {
        if !self.has_retained {
            debug_assert!(self.simplify.is_none() && self.sloppy.is_none());
            debug_assert_eq!(
                self.integers.capacity()
                    | self.floats.capacity()
                    | self.flags.capacity()
                    | self.keys.capacity()
                    | self.vertices.capacity(),
                0
            );
            return Ok(0);
        }
        total(
            [
                self.integers.capacity(),
                self.floats.capacity(),
                self.flags.capacity(),
                self.keys.capacity(),
            ],
            checked_bytes(self.vertices.capacity(), size_of::<CacheVertex>())?
                .checked_add(
                    self.simplify
                        .as_ref()
                        .map_or(Ok(0), |s| s.capacity_bytes())?,
                )
                .ok_or(Error::SizeOverflow)?
                .checked_add(self.sloppy.as_ref().map_or(Ok(0), |s| s.capacity_bytes())?)
                .ok_or(Error::SizeOverflow)?,
        )
    }
    #[inline]
    pub(crate) fn charge_owned(&mut self, retained: usize, owned: usize) -> Result<(), Error> {
        let bytes = retained.checked_add(owned).ok_or(Error::SizeOverflow)?;
        if bytes > self.limits.max_bytes {
            return Err(Error::LimitExceeded);
        }
        self.usage.bytes = self.usage.bytes.max(bytes);
        Ok(())
    }
    #[inline]
    pub(crate) fn charge_retained(&mut self) -> Result<(), Error> {
        self.charge_owned(self.retained_bytes()?, 0)
    }
    pub(crate) fn prepare(&mut self, lengths: [usize; 4], output: usize) -> Result<(), Error> {
        self.prepare_cache(lengths, 0, output)
    }
    pub(crate) fn prepare_remap(&mut self, vertices: usize, output: usize) -> Result<(), Error> {
        let previous = self.integers.len();
        self.prepare_cache_value([vertices, 0, 0, 0], 0, output, u32::MAX)?;
        self.integers[..previous.min(vertices)].fill(u32::MAX);
        Ok(())
    }
    pub(crate) fn prepare_cache(
        &mut self,
        lengths: [usize; 4],
        vertices: usize,
        output: usize,
    ) -> Result<(), Error> {
        self.prepare_cache_value(lengths, vertices, output, 0)
    }
    fn prepare_cache_value(
        &mut self,
        lengths: [usize; 4],
        vertices: usize,
        output: usize,
        integer_value: u32,
    ) -> Result<(), Error> {
        let [i, f, b, k] = lengths;
        let simplify_bytes = self
            .simplify
            .as_ref()
            .map_or(Ok(0), |s| s.capacity_bytes())?
            .checked_add(self.sloppy.as_ref().map_or(Ok(0), |s| s.capacity_bytes())?)
            .ok_or(Error::SizeOverflow)?;
        let vertex_bytes = checked_bytes(
            vertices.max(self.vertices.capacity()),
            size_of::<CacheVertex>(),
        )?;
        let owned = output
            .checked_add(vertex_bytes)
            .and_then(|n| n.checked_add(simplify_bytes))
            .ok_or(Error::SizeOverflow)?;
        let bytes = total(
            [
                i.max(self.integers.capacity()),
                f.max(self.floats.capacity()),
                b.max(self.flags.capacity()),
                k.max(self.keys.capacity()),
            ],
            owned,
        )?;
        if bytes > self.limits.max_bytes {
            return Err(Error::LimitExceeded);
        }
        self.has_retained |= i != 0 || f != 0 || b != 0 || k != 0 || vertices != 0;
        reserve(&mut self.vertices, vertices)?;
        reserve_value(&mut self.integers, i, integer_value)?;
        reserve(&mut self.floats, f)?;
        reserve(&mut self.flags, b)?;
        reserve(&mut self.keys, k)?;
        let actual = total(
            [
                self.integers.capacity(),
                self.floats.capacity(),
                self.flags.capacity(),
                self.keys.capacity(),
            ],
            output
                .checked_add(checked_bytes(
                    self.vertices.capacity(),
                    size_of::<CacheVertex>(),
                )?)
                .and_then(|n| n.checked_add(simplify_bytes))
                .ok_or(Error::SizeOverflow)?,
        )?;
        if actual > self.limits.max_bytes {
            self.clear();
            return Err(Error::LimitExceeded);
        }
        self.usage.bytes = self.usage.bytes.max(actual);
        Ok(())
    }
}

fn total(lengths: [usize; 4], output: usize) -> Result<usize, Error> {
    let mut sum = output;
    for (len, size) in lengths.into_iter().zip([4, 4, 1, 2]) {
        let bytes = checked_bytes(len, size)?;
        sum = sum.checked_add(bytes).ok_or(Error::SizeOverflow)?;
    }
    Ok(sum)
}

// Codec-only wiring: account retained geometry scratch without resizing any
// vector. Raw decoding requires no heap scratch and preserves the workspace.
impl Workspace {
    #[inline]
    pub(crate) fn account_codec(&mut self, output: usize) -> Result<(), Error> {
        if self.vertices.capacity() == 0
            && self.integers.capacity() == 0
            && self.floats.capacity() == 0
            && self.flags.capacity() == 0
            && self.keys.capacity() == 0
        {
            if output > self.limits.max_bytes {
                return Err(Error::LimitExceeded);
            }
            self.usage.bytes = output;
            return Ok(());
        }
        let owned = output
            .checked_add(checked_bytes(
                self.vertices.capacity(),
                size_of::<CacheVertex>(),
            )?)
            .ok_or(Error::SizeOverflow)?;
        let bytes = total(
            [
                self.integers.capacity(),
                self.floats.capacity(),
                self.flags.capacity(),
                self.keys.capacity(),
            ],
            owned,
        )?;
        if bytes > self.limits.max_bytes {
            return Err(Error::LimitExceeded);
        }
        self.usage.bytes = bytes;
        Ok(())
    }
}

pub(crate) fn checked_bytes(len: usize, size: usize) -> Result<usize, Error> {
    let bytes = len.checked_mul(size).ok_or(Error::SizeOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    Ok(bytes)
}

fn reserve<T: Default + Clone>(v: &mut Vec<T>, len: usize) -> Result<(), Error> {
    reserve_value(v, len, T::default())
}
fn reserve_value<T: Clone>(v: &mut Vec<T>, len: usize, value: T) -> Result<(), Error> {
    checked_bytes(len, size_of::<T>())?;
    if len > v.capacity() {
        v.try_reserve_exact(len.saturating_sub(v.len()))
            .map_err(|_| Error::AllocationFailed)?;
    }
    v.resize(len, value);
    Ok(())
}

pub(crate) fn output(len: usize) -> Result<Vec<u32>, Error> {
    checked_bytes(len, 4)?;
    let mut v = Vec::new();
    reserve(&mut v, len)?;
    Ok(v)
}
pub(crate) fn output_sentinel(len: usize) -> Result<Vec<u32>, Error> {
    checked_bytes(len, 4)?;
    let mut v = Vec::new();
    reserve_value(&mut v, len, u32::MAX)?;
    Ok(v)
}

pub(crate) struct Work {
    remaining: u64,
    limit: u64,
}
impl Work {
    #[inline]
    pub(crate) fn covers(&self, count: usize) -> Result<bool, Error> {
        Ok(u64::try_from(count).map_err(|_| Error::SizeOverflow)? <= self.remaining)
    }

    #[inline(always)]
    pub(crate) fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.scan_units(values, 1, visit)
    }
    /// Each callback has the same pre-visit charge. Keep that granularity on
    /// exhaustion, including callbacks that fail before completing their work.
    #[inline(always)]
    pub(crate) fn scan_units<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        units: usize,
        mut visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = u64::try_from(values.len()).map_err(|_| Error::SizeOverflow)?;
        let unit_count = u64::try_from(units).map_err(|_| Error::SizeOverflow)?;
        if let Some(visits) = count
            .checked_mul(unit_count)
            .filter(|&v| v <= self.remaining)
        {
            for (i, value) in values.enumerate() {
                if let Err(error) = visit(value) {
                    self.remaining -= (i as u64 + 1) * unit_count;
                    return Err(error);
                }
            }
            self.remaining -= visits;
            Ok(())
        } else {
            for value in values {
                self.add(units)?;
                visit(value)?;
            }
            Ok(())
        }
    }
    /// Precharge a fixed scan only when every visit fits. The caller must
    /// execute every visit without a fallible callback on this path.
    #[inline]
    pub(crate) fn precharge(&mut self, count: usize, units: usize) -> Result<bool, Error> {
        let count = u64::try_from(count).map_err(|_| Error::SizeOverflow)?;
        let units = u64::try_from(units).map_err(|_| Error::SizeOverflow)?;
        if let Some(visits) = count.checked_mul(units).filter(|&v| v <= self.remaining) {
            self.remaining -= visits;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// Number of records whose full per-record search bound fits.
    #[inline]
    pub(crate) fn batch(&self, count: usize, units: usize) -> Result<usize, Error> {
        let units = u64::try_from(units).map_err(|_| Error::SizeOverflow)?;
        let count = u64::try_from(count).map_err(|_| Error::SizeOverflow)?;
        let allowed = self
            .remaining
            .min(usize::MAX as u64)
            .checked_div(units)
            .map_or(count, |n| count.min(n));
        usize::try_from(allowed).map_err(|_| Error::SizeOverflow)
    }
    /// Count a bounded search once when every possible visit fits. The limited
    /// path checks before each callback, preserving its observable prefix.
    #[inline(always)]
    pub(crate) fn search(
        &mut self,
        count: usize,
        mut found: impl FnMut(usize) -> bool,
    ) -> Result<Option<usize>, Error> {
        if self.covers(count)? {
            for i in 0..count {
                if found(i) {
                    self.add(i + 1)?;
                    return Ok(Some(i));
                }
            }
            self.add(count)?;
        } else {
            for i in 0..count {
                self.add(1)?;
                if found(i) {
                    return Ok(Some(i));
                }
            }
        }
        Ok(None)
    }
    #[inline]
    pub(crate) fn add(&mut self, count: usize) -> Result<(), Error> {
        let count = u64::try_from(count).map_err(|_| Error::SizeOverflow)?;
        if count > self.remaining {
            return self.exhausted(count);
        }
        self.remaining -= count;
        Ok(())
    }
    #[cold]
    #[inline(never)]
    fn exhausted(&self, count: u64) -> Result<(), Error> {
        // Preserve both the failed visit's count and the overflow error.
        (self.limit - self.remaining)
            .checked_add(count)
            .ok_or(Error::SizeOverflow)?;
        Err(Error::LimitExceeded)
    }
}

pub(crate) fn topology(indices: &[u32], vertices: usize, work: &mut Work) -> Result<(), Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    // Upstream adjacency offsets and triangle identifiers are u32.
    if vertices > u32::MAX as usize || indices.len() > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    checked_bytes(vertices, 4)?;
    checked_bytes(indices.len(), 4)?;
    work.scan(indices.iter().copied(), |index| {
        if index as usize >= vertices {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(())
    })
}

// Additive 0.3 accounting: processing modules use local typed scratch and do not
// resize the workspace's cache/overdraw buffers.
impl Workspace {
    pub(crate) fn retained_storage(&self) -> Result<usize, Error> {
        let mut bytes = 0usize;
        for (n, size) in [
            (self.vertices.capacity(), size_of::<CacheVertex>()),
            (self.integers.capacity(), 4),
            (self.floats.capacity(), 4),
            (self.flags.capacity(), 1),
            (self.keys.capacity(), 2),
        ] {
            bytes = bytes
                .checked_add(checked_bytes(n, size)?)
                .ok_or(Error::SizeOverflow)?;
        }
        Ok(bytes)
    }
    #[inline]
    pub(crate) fn processing_storage(&mut self, bytes: usize) -> Result<(), Error> {
        if bytes > self.limits.max_bytes {
            return Err(Error::LimitExceeded);
        }
        self.usage.bytes = self.usage.bytes.max(bytes);
        Ok(())
    }
}

// Child operation totals are u64 even on 32-bit targets.
#[cfg(any(feature = "clusterlod", feature = "parallel"))]
impl Work {
    #[inline]
    pub(crate) fn add_processing(&mut self, count: u64) -> Result<(), Error> {
        if count > self.remaining {
            return self.exhausted(count);
        }
        self.remaining -= count;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precharged_scans_and_batches_fit_their_counter_width() {
        for units in [0, 1, 3, 99] {
            for limit in 0..=units * 5 + 1 {
                let mut scalar = Work {
                    remaining: limit as u64,
                    limit: limit as u64,
                };
                let mut batched = Work {
                    remaining: limit as u64,
                    limit: limit as u64,
                };
                let mut expected_prefix = Vec::new();
                let mut actual_prefix = Vec::new();
                let expected: Result<(), Error> = (|| {
                    for i in 0..5 {
                        scalar.add(units)?;
                        expected_prefix.push(i);
                    }
                    Ok(())
                })();
                let actual: Result<(), Error> = (|| {
                    let charged = batched.precharge(5, units)?;
                    for i in 0..5 {
                        if !charged {
                            batched.add(units)?;
                        }
                        actual_prefix.push(i);
                    }
                    Ok(())
                })();
                assert_eq!(actual, expected);
                assert_eq!(actual_prefix, expected_prefix);
                assert_eq!(batched.remaining, scalar.remaining);
            }
        }
        let huge = Work {
            remaining: u64::MAX,
            limit: u64::MAX,
        };
        for units in [1, 2, 99, usize::MAX] {
            let count = huge.batch(usize::MAX, units).unwrap();
            assert!(count.checked_mul(units).is_some());
            assert!(count as u64 * units as u64 <= huge.remaining);
        }
    }

    #[test]
    fn fixed_charge_scans_preserve_failure_prefixes() {
        for units in [0, 1, 3, 99] {
            for limit in 0..=units * 5 + 1 {
                for failure in 0..=5 {
                    let mut scalar = Work {
                        remaining: limit as u64,
                        limit: limit as u64,
                    };
                    let mut batched = Work {
                        remaining: limit as u64,
                        limit: limit as u64,
                    };
                    let mut expected_prefix = Vec::new();
                    let mut actual_prefix = Vec::new();
                    let expected = (|| {
                        for i in 0..5 {
                            scalar.add(units)?;
                            expected_prefix.push(i);
                            if i == failure {
                                return Err(Error::InvalidParameter);
                            }
                        }
                        Ok(())
                    })();
                    let actual = batched.scan_units(0..5, units, |i| {
                        actual_prefix.push(i);
                        if i == failure {
                            Err(Error::InvalidParameter)
                        } else {
                            Ok(())
                        }
                    });
                    assert_eq!(actual, expected);
                    assert_eq!(batched.remaining, scalar.remaining);
                    assert_eq!(actual_prefix, expected_prefix);
                }
            }
        }
    }

    #[test]
    fn bounded_search_preserves_the_checked_callback_prefix() {
        for limit in 0..8 {
            for match_at in 0..8 {
                let mut scalar = Work {
                    remaining: limit,
                    limit,
                };
                let mut batched = Work {
                    remaining: limit,
                    limit,
                };
                let mut scalar_visits = Vec::new();
                let mut batched_visits = Vec::new();
                let expected = (|| {
                    for i in 0..5 {
                        scalar.add(1)?;
                        scalar_visits.push(i);
                        if i == match_at {
                            return Ok(Some(i));
                        }
                    }
                    Ok(None)
                })();
                let actual = batched.search(5, |i| {
                    batched_visits.push(i);
                    i == match_at
                });
                assert_eq!(actual, expected);
                assert_eq!(batched.remaining, scalar.remaining);
                assert_eq!(batched_visits, scalar_visits);
            }
        }
    }
    #[test]
    fn scan_preserves_failures_work_and_side_effects() {
        for limit in 0..=8 {
            for fail_at in 0usize..=5 {
                let mut scalar = Work {
                    remaining: limit,
                    limit,
                };
                let mut scanned = Work {
                    remaining: limit,
                    limit,
                };
                let mut scalar_sum = 0;
                let mut scanned_sum = 0;
                let visit = |i, sum: &mut usize| {
                    *sum += i + 1;
                    if i == fail_at {
                        Err(Error::NumericalFailure)
                    } else {
                        Ok(())
                    }
                };
                let expected = (|| {
                    for i in 0..5 {
                        scalar.add(1)?;
                        visit(i, &mut scalar_sum)?;
                    }
                    Ok(())
                })();
                let actual = scanned.scan(0..5, |i| visit(i, &mut scanned_sum));
                assert_eq!(actual, expected);
                assert_eq!(scanned.remaining, scalar.remaining);
                assert_eq!(scanned_sum, scalar_sum);
            }
        }
    }
}
