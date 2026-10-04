use crate::clusterlod::support::Error;
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
/// Its typed scratch is allocated fallibly per call and released on return;
/// usage reports peak storage including retained workspace buffers. Scale
/// computation is allocation-free and has no workspace argument.
/// Limits never disable checked size arithmetic.
#[derive(Debug, Default)]
pub struct Workspace {
    limits: Limits,
    usage: Usage,
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
        self.vertices = Vec::new();
        self.integers = Vec::new();
        self.floats = Vec::new();
        self.flags = Vec::new();
        self.keys = Vec::new();
        self.usage = Usage::default();
    }
    /// Measurements for the most recent call, including a failed call's work.
    pub const fn usage(&self) -> Usage {
        self.usage
    }
    pub(crate) fn begin(&mut self) -> Work {
        self.usage = Usage::default();
        Work {
            remaining: self.limits.max_work,
            limit: self.limits.max_work,
        }
    }
    pub(crate) fn finish(&mut self, work: &Work) {
        self.usage.work = work.limit - work.remaining;
    }
    pub(crate) fn prepare(&mut self, lengths: [usize; 4], output: usize) -> Result<(), Error> {
        self.prepare_cache(lengths, 0, output)
    }
    pub(crate) fn prepare_cache(
        &mut self,
        lengths: [usize; 4],
        vertices: usize,
        output: usize,
    ) -> Result<(), Error> {
        let [i, f, b, k] = lengths;
        let vertex_bytes = checked_bytes(
            vertices.max(self.vertices.capacity()),
            size_of::<CacheVertex>(),
        )?;
        let owned = output
            .checked_add(vertex_bytes)
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
        reserve(&mut self.vertices, vertices)?;
        reserve(&mut self.integers, i)?;
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

pub(crate) fn checked_bytes(len: usize, size: usize) -> Result<usize, Error> {
    let bytes = len.checked_mul(size).ok_or(Error::SizeOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    Ok(bytes)
}

fn reserve<T: Default + Clone>(v: &mut Vec<T>, len: usize) -> Result<(), Error> {
    checked_bytes(len, size_of::<T>())?;
    if len > v.capacity() {
        v.try_reserve_exact(len.saturating_sub(v.len()))
            .map_err(|_| Error::AllocationFailed)?;
    }
    v.resize(len, T::default());
    Ok(())
}

pub(crate) fn output(len: usize) -> Result<Vec<u32>, Error> {
    checked_bytes(len, 4)?;
    let mut v = Vec::new();
    reserve(&mut v, len)?;
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

    #[inline]
    pub(crate) fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = values.len();
        let visits = u64::try_from(count).map_err(|_| Error::SizeOverflow)?;
        if visits <= self.remaining {
            // Charge the complete scan once. On a callback failure, charge only
            // its visited prefix, including the failing record as add(1) did.
            for (i, value) in values.enumerate() {
                if let Err(error) = visit(value) {
                    self.add(i + 1)?;
                    return Err(error);
                }
            }
            self.add(count)
        } else {
            // Preserve the original exhaustion point and partial side effects.
            for value in values {
                self.add(1)?;
                visit(value)?;
            }
            Ok(())
        }
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

#[cfg(test)]
mod tests {
    use super::*;

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
