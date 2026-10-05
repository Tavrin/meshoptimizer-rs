// Checked storage and record-visit accounting shared by the 0.3 translations.
use crate::workspace::{checked_bytes, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;
use core::mem::size_of;

pub(crate) struct Context<'a> {
    pub(crate) workspace: &'a mut Workspace,
    pub(crate) work: Work,
    bytes: usize,
    retained: usize,
    pub(crate) moderate: bool,
    roots: Option<[(u32, f32); 64]>,
    large_roots: Option<[(u32, f32); 512]>,
}
impl<'a> Context<'a> {
    #[inline(always)]
    pub(crate) fn new(workspace: &'a mut Workspace) -> Result<Self, Error> {
        let work = workspace.begin();
        let retained = workspace.retained_storage()?;
        workspace.processing_storage(retained)?;
        Ok(Self {
            workspace,
            work,
            bytes: 0,
            retained,
            moderate: true,
            roots: None,
            large_roots: None,
        })
    }
    #[allow(clippy::unnecessary_lazy_evaluations)]
    #[inline(always)]
    pub(crate) fn sqrt(&mut self, value: f32) -> f32 {
        let bits = value.to_bits();
        let slot = if let Some(roots) = &mut self.large_roots {
            &mut roots[(bits.wrapping_mul(0x9e3779b9) >> 23) as usize]
        } else {
            &mut self.roots.get_or_insert_with(|| [(0, 0.0); 64])
                [(bits.wrapping_mul(0x9e3779b9) >> 26) as usize]
        };
        if slot.0 != bits {
            *slot = (bits, crate::math::sqrt(value));
        }
        slot.1
    }
    pub(crate) fn large_memo(&mut self) {
        self.large_roots = Some([(0, 0.0); 512]);
    }
    #[inline(always)]
    pub(crate) fn tick(&mut self, n: usize) -> Result<(), Error> {
        self.work.add(n)
    }
    fn reserve<T>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(n, size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        self.workspace.processing_storage(
            self.bytes
                .checked_add(self.retained)
                .ok_or(Error::SizeOverflow)?,
        )?;
        let mut result = Vec::new();
        result
            .try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        let extra = checked_bytes(result.capacity() - n, size_of::<T>())?;
        if extra != 0 {
            self.bytes = self.bytes.checked_add(extra).ok_or(Error::SizeOverflow)?;
            self.workspace.processing_storage(
                self.bytes
                    .checked_add(self.retained)
                    .ok_or(Error::SizeOverflow)?,
            )?;
        }
        Ok(result)
    }
    pub(crate) fn alloc<T: Default>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        let mut v = self.reserve(n)?;
        v.resize_with(n, T::default);
        Ok(v)
    }
    #[cfg(feature = "clusterlod")]
    pub(crate) fn filled<T: Clone>(&mut self, n: usize, value: T) -> Result<Vec<T>, Error> {
        let mut v = self.reserve(n)?;
        v.resize(n, value);
        Ok(v)
    }
    pub(crate) fn copy<T: Copy>(&mut self, values: &[T]) -> Result<Vec<T>, Error> {
        let mut v = self.reserve(values.len())?;
        v.extend_from_slice(values);
        Ok(v)
    }
    // Visitors may use math/storage, but do not charge nested work. Preserve
    // both the failing visit and the original short-budget exhaustion point.
    #[inline(always)]
    pub(crate) fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(&mut Self, T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = values.len();
        if self.work.covers(count)? {
            for (i, value) in values.enumerate() {
                if let Err(error) = visit(self, value) {
                    self.tick(i + 1)?;
                    return Err(error);
                }
            }
            self.tick(count)
        } else {
            for value in values {
                self.tick(1)?;
                visit(self, value)?;
            }
            Ok(())
        }
    }
    // A search can end early. Charge only the visited prefix; on a short
    // budget retain the scalar loop's exhaustion point and side effects.
    #[inline(always)]
    pub(crate) fn search<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(T) -> bool,
    ) -> Result<(), Error> {
        if self.work.covers(values.len())? {
            let mut visited = 0;
            for value in values {
                visited += 1;
                if visit(value) {
                    break;
                }
            }
            self.tick(visited)
        } else {
            for value in values {
                self.tick(1)?;
                if visit(value) {
                    break;
                }
            }
            Ok(())
        }
    }
    pub(crate) fn release<T>(&mut self, values: &Vec<T>) -> Result<(), Error> {
        self.bytes = self
            .bytes
            .checked_sub(checked_bytes(values.capacity(), size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        Ok(())
    }
    pub(crate) fn free<T>(&mut self, values: Vec<T>) -> Result<(), Error> {
        self.release(&values)?;
        drop(values);
        Ok(())
    }
    #[cfg(feature = "clusterlod")]
    pub(crate) fn child<T: OwnedBytes>(
        &mut self,
        run: impl FnOnce(&mut Workspace) -> Result<T, Error>,
    ) -> Result<T, Error> {
        self.workspace.finish(&self.work);
        let limits = self.workspace.limits();
        let mut child = Workspace::new(crate::Limits {
            max_bytes: limits
                .max_bytes
                .checked_sub(
                    self.bytes
                        .checked_add(self.retained)
                        .ok_or(Error::SizeOverflow)?,
                )
                .ok_or(Error::LimitExceeded)?,
            max_work: limits.max_work - self.workspace.usage().work,
        });
        let result = run(&mut child);
        self.work.add_processing(child.usage().work)?;
        self.workspace.processing_storage(
            self.bytes
                .checked_add(self.retained)
                .and_then(|n| n.checked_add(child.usage().bytes))
                .ok_or(Error::SizeOverflow)?,
        )?;
        if let Ok(value) = &result {
            self.bytes = self
                .bytes
                .checked_add(value.owned_bytes()?)
                .ok_or(Error::SizeOverflow)?;
        }
        result
    }
    #[cfg(feature = "clusterlod")]
    pub(crate) fn remaining_limits(&mut self) -> Result<crate::Limits, Error> {
        self.workspace.finish(&self.work);
        let limits = self.workspace.limits();
        Ok(crate::Limits {
            max_bytes: limits
                .max_bytes
                .checked_sub(
                    self.bytes
                        .checked_add(self.retained)
                        .ok_or(Error::SizeOverflow)?,
                )
                .ok_or(Error::LimitExceeded)?,
            max_work: limits.max_work - self.workspace.usage().work,
        })
    }
    #[cfg(feature = "clusterlod")]
    pub(crate) fn absorb(&mut self, bytes: usize, work: u64, output: usize) -> Result<(), Error> {
        self.work.add_processing(work)?;
        self.workspace.processing_storage(
            self.bytes
                .checked_add(self.retained)
                .and_then(|n| n.checked_add(bytes))
                .ok_or(Error::SizeOverflow)?,
        )?;
        self.bytes = self.bytes.checked_add(output).ok_or(Error::SizeOverflow)?;
        Ok(())
    }
    #[cfg(feature = "clusterlod")]
    pub(crate) fn grow<T>(&mut self, v: &mut Vec<T>, extra: usize) -> Result<(), Error> {
        let needed = v.len().checked_add(extra).ok_or(Error::SizeOverflow)?;
        if needed > v.capacity() {
            let old = v.capacity();
            let added = checked_bytes(needed - old, size_of::<T>())?;
            self.workspace.processing_storage(
                self.bytes
                    .checked_add(self.retained)
                    .and_then(|n| n.checked_add(added))
                    .ok_or(Error::SizeOverflow)?,
            )?;
            v.try_reserve_exact(extra)
                .map_err(|_| Error::AllocationFailed)?;
            self.bytes = self
                .bytes
                .checked_add(checked_bytes(v.capacity() - old, size_of::<T>())?)
                .ok_or(Error::SizeOverflow)?;
            self.workspace.processing_storage(
                self.bytes
                    .checked_add(self.retained)
                    .ok_or(Error::SizeOverflow)?,
            )?;
        }
        Ok(())
    }
    pub(crate) fn positions(&mut self, positions: Positions<'_>) -> Result<(), Error> {
        if positions.len() > u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        self.tick(positions.len())?;
        let mut largest = 0u32;
        positions.for_each(|p| {
            largest = largest
                .max(p[0].to_bits() & 0x7fffffff)
                .max(p[1].to_bits() & 0x7fffffff)
                .max(p[2].to_bits() & 0x7fffffff);
            Ok(())
        })?;
        self.moderate &= largest <= 1.0e8f32.to_bits();
        if largest > f32::MAX.to_bits() {
            Err(Error::NumericalFailure)
        } else {
            Ok(())
        }
    }
    pub(crate) fn topology(&mut self, indices: &[u32], n: usize) -> Result<(), Error> {
        topology(indices, n, &mut self.work)
    }
}
impl Drop for Context<'_> {
    fn drop(&mut self) {
        self.workspace.finish(&self.work);
    }
}

#[inline(always)]
pub(crate) fn finite(x: f32) -> Result<f32, Error> {
    if x.is_finite() {
        Ok(x)
    } else {
        Err(Error::NumericalFailure)
    }
}
#[inline(always)]
pub(crate) fn point(p: Positions<'_>, i: u32) -> [f32; 3] {
    p.get(i as usize).expect("validated vertex")
}
#[inline(always)]
pub(crate) fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline(always)]
pub(crate) fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline(always)]
pub(crate) fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
// Stable radix passes preserve source tie order, including all-zero keys.
// All P histograms come from one sweep over the keys, and their prefix sums
// run as P independent chains in one loop (upstream computeHistogram).
pub(crate) fn radix<const P: usize, K: Copy + Into<u64>>(
    order: &mut [u32],
    scratch: &mut [u32],
    keys: &[K],
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let mut fragments = ctx.alloc::<u16>(keys.len())?;
    let mut hist = [[0u32; P]; 1024];
    for &key in keys {
        let key: u64 = key.into();
        for k in 0..P {
            hist[((key >> (k * 10)) & 1023) as usize][k] += 1;
        }
    }
    let mut sums = [0u32; P];
    for row in hist.iter_mut() {
        for (slot, sum) in row.iter_mut().zip(sums.iter_mut()) {
            let n = *slot;
            *slot = *sum;
            *sum += n;
        }
    }
    let mut source = order;
    let mut destination = scratch;
    #[allow(clippy::needless_range_loop)]
    for k in 0..P {
        ctx.tick(source.len().checked_mul(2).ok_or(Error::SizeOverflow)?)?;
        for (f, &key) in fragments.iter_mut().zip(keys) {
            *f = ((key.into() >> (k * 10)) & 1023) as u16;
        }
        for &v in source.iter() {
            let h = &mut hist[fragments[v as usize] as usize][k];
            destination[*h as usize] = v;
            *h += 1;
        }
        core::mem::swap(&mut source, &mut destination);
    }
    if !P.is_multiple_of(2) {
        destination.copy_from_slice(source);
    }
    ctx.release(&fragments)?;
    Ok(())
}

#[cfg(feature = "clusterlod")]
pub(crate) trait OwnedBytes {
    fn owned_bytes(&self) -> Result<usize, Error>;
}
#[cfg(feature = "clusterlod")]
impl<T> OwnedBytes for Vec<T> {
    fn owned_bytes(&self) -> Result<usize, Error> {
        checked_bytes(self.capacity(), size_of::<T>())
    }
}
#[cfg(feature = "clusterlod")]
impl OwnedBytes for () {
    fn owned_bytes(&self) -> Result<usize, Error> {
        Ok(0)
    }
}
#[cfg(feature = "clusterlod")]
impl OwnedBytes for crate::Bounds {
    fn owned_bytes(&self) -> Result<usize, Error> {
        Ok(0)
    }
}
#[cfg(feature = "clusterlod")]
impl OwnedBytes for crate::Meshlets {
    fn owned_bytes(&self) -> Result<usize, Error> {
        self.meshlets
            .owned_bytes()?
            .checked_add(self.vertices.owned_bytes()?)
            .and_then(|n| n.checked_add(self.triangles.capacity()))
            .ok_or(Error::SizeOverflow)
    }
}
#[cfg(feature = "clusterlod")]
impl OwnedBytes for crate::Partitions {
    fn owned_bytes(&self) -> Result<usize, Error> {
        self.assignments.owned_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn processing_memos_preserve_backend_bits_and_collisions() {
        for large in [false, true] {
            let mut workspace = Workspace::default();
            let mut context = Context::new(&mut workspace).unwrap();
            if large {
                context.large_memo();
            }
            let mut check = |bits| {
                let value = f32::from_bits(bits);
                let expected = crate::math::sqrt(value).to_bits();
                assert_eq!(context.sqrt(value).to_bits(), expected);
                assert_eq!(context.sqrt(value).to_bits(), expected);
            };
            for bits in [
                0, 0x80000000, 1, 0x7fffff, 0x800000, 0x7f7fffff, 0x7f800000, 0xff800000,
                0x7fc00123, 0x3f800000,
            ] {
                check(bits);
            }
            let mut bits = 0x12345678u32;
            for _ in 0..65536 {
                bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
                check(bits);
            }
        }
    }
    #[test]
    fn processing_scan_preserves_budget_failure_visit_and_side_effects() {
        for limit in 0..=8 {
            for failure in 0..=5 {
                let mut ws = Workspace::new(crate::Limits {
                    max_bytes: 0,
                    max_work: limit,
                });
                let mut context = Context::new(&mut ws).unwrap();
                let mut scanned = 0;
                let result = context.scan(0..5, |_, i| {
                    scanned += i + 1;
                    if i == failure {
                        Err(Error::NumericalFailure)
                    } else {
                        Ok(())
                    }
                });
                drop(context);
                let mut consumed = 0;
                let mut expected_sum = 0;
                let expected = (|| {
                    for i in 0..5 {
                        if consumed == limit {
                            return Err(Error::LimitExceeded);
                        }
                        consumed += 1;
                        expected_sum += i + 1;
                        if i == failure {
                            return Err(Error::NumericalFailure);
                        }
                    }
                    Ok(())
                })();
                assert_eq!(result, expected);
                assert_eq!(scanned, expected_sum);
                assert_eq!(ws.usage().work, consumed);
            }
        }
    }
    #[test]
    fn processing_search_charges_only_visited_prefix() {
        for limit in 0..=8 {
            for stop in 0..=5 {
                let mut ws = Workspace::new(crate::Limits {
                    max_bytes: 0,
                    max_work: limit,
                });
                let mut context = Context::new(&mut ws).unwrap();
                let mut visited = 0;
                let result = context.search(0..5, |i| {
                    visited += 1;
                    i == stop
                });
                drop(context);
                let needed = (stop + 1).min(5);
                assert_eq!(visited, (limit as usize).min(needed));
                assert_eq!(ws.usage().work as usize, visited);
                assert_eq!(result.is_ok(), limit as usize >= needed);
            }
        }
    }
    #[cfg(feature = "clusterlod")]
    #[test]
    fn child_and_external_work_keep_u64_counts_on_32_bit_targets() {
        let visits = (1u64 << 32) + 17;
        let mut workspace = Workspace::new(crate::Limits {
            max_bytes: 0,
            max_work: 2 * visits + 1,
        });
        {
            let mut context = Context::new(&mut workspace).unwrap();
            context
                .child(|child| {
                    let mut work = child.begin();
                    work.add(u32::MAX as usize)?;
                    work.add(18)?;
                    child.finish(&work);
                    Ok(())
                })
                .unwrap();
            context.absorb(0, visits, 0).unwrap();
        }
        assert_eq!(workspace.usage().work, 2 * visits);
        workspace.set_limits(crate::Limits {
            max_bytes: 0,
            max_work: visits - 1,
        });
        {
            let mut context = Context::new(&mut workspace).unwrap();
            assert_eq!(context.absorb(0, visits, 0), Err(Error::LimitExceeded));
        }
        assert_eq!(workspace.usage().work, 0);
    }
}
