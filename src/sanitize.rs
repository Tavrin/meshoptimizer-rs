//! Triangle sanitation. This module is an extension: it is not in C++
//! meshoptimizer, and no existing function's behaviour depends on it.

use crate::budget::Budget;
use crate::remap::{weld_position, weld_table_len};
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

const NONE: u32 = u32::MAX;
const EMPTY_KEY: [u32; 3] = [NONE; 3];

/// Options for [`sanitize`]. Not in C++ meshoptimizer.
///
/// The default, [`Self::EMPTY`], drops triangles with out-of-range indices or
/// non-finite referenced positions, drops triangles that repeat an index, and
/// drops later copies of a face with the same winding. Combine options with `|`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SanitizeOptions(u32);
impl SanitizeOptions {
    /// Default policy: drop and count invalid triangles; keep reverse-wound faces.
    pub const EMPTY: Self = Self(0);
    /// Return an error instead of dropping a triangle with an out-of-range
    /// index (`IndexOutOfBounds`) or a non-finite referenced position
    /// (`NumericalFailure`). The first such triangle in input order decides
    /// the error.
    pub const REJECT_INVALID: Self = Self(1);
    /// Also drop triangles with two corners at exactly equal positions
    /// (signed zeros compare equal). Collinear triangles with three distinct
    /// positions are kept: no area is computed.
    pub const ZERO_AREA: Self = Self(2);
    /// Compare faces for duplicates by exact position instead of by index:
    /// vertices with equal positions count as the same corner, using the same
    /// equality as [`generate_position_remap`](crate::generate_position_remap).
    /// Output indices are never rewritten.
    pub const WELD_POSITIONS: Self = Self(4);
    /// Also drop a face that matches an earlier kept face with the opposite
    /// winding. By default such coincident reverse-wound faces are kept,
    /// because two-sided geometry relies on them.
    pub const IGNORE_ORIENTATION: Self = Self(8);
    /// Validate a raw option mask.
    ///
    /// # Errors
    /// `UnknownFlags` if any option bit is unknown.
    pub const fn from_bits(bits: u32) -> Result<Self, Error> {
        if bits & !15 != 0 {
            Err(Error::UnknownFlags)
        } else {
            Ok(Self(bits))
        }
    }
    /// Return the option mask.
    pub const fn bits(self) -> u32 {
        self.0
    }
    /// Whether all bits in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SanitizeOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for SanitizeOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Triangle counts from [`sanitize`] or [`sanitize_into`]. Not in C++
/// meshoptimizer.
///
/// Each removed triangle is counted once, under the first reason that applies
/// in this order: out-of-range index, non-finite position, repeated index,
/// zero area, duplicate, reversed duplicate. So `input_triangles` equals
/// `output_triangles` plus [`Self::removed`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SanitizeReport {
    /// Triangles in the input.
    pub input_triangles: usize,
    /// Triangles kept in the output.
    pub output_triangles: usize,
    /// Removed: an index is at or beyond the vertex count.
    pub out_of_range: usize,
    /// Removed: a referenced position has a NaN or infinite component.
    pub non_finite: usize,
    /// Removed: two corners use the same index.
    pub repeated_index: usize,
    /// Removed with [`SanitizeOptions::ZERO_AREA`]: two corners have equal positions.
    pub zero_area: usize,
    /// Removed: an earlier kept face has the same corners in the same cyclic order.
    pub duplicate: usize,
    /// Removed with [`SanitizeOptions::IGNORE_ORIENTATION`]: an earlier kept
    /// face has the same corners with the opposite winding.
    pub reversed_duplicate: usize,
}
impl SanitizeReport {
    /// Total removed triangles over all reasons.
    pub const fn removed(&self) -> usize {
        self.out_of_range
            + self.non_finite
            + self.repeated_index
            + self.zero_area
            + self.duplicate
            + self.reversed_duplicate
    }
    /// Output index count, three per kept triangle.
    pub const fn index_count(&self) -> usize {
        self.output_triangles * 3
    }
}

/// Cleaned triangle list from [`sanitize`]. Not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SanitizedMesh {
    /// Kept triangles in input order, with their original corners and winding.
    pub indices: Vec<u32>,
    /// Counts by removal reason.
    pub report: SanitizeReport,
}

/// Remove invalid, degenerate and duplicate triangles from a triangle list.
/// Not in C++ meshoptimizer.
///
/// Kept triangles stay in input order with their original indices, corners and
/// winding; vertices are never moved, renumbered or removed. Positions are
/// read only for referenced vertices. The first copy of a duplicate face is
/// kept. Faces are equal when their corners match after a cyclic rotation, so
/// `[0, 1, 2]` and `[1, 2, 0]` are duplicates; `[0, 2, 1]` is the same face
/// with the opposite winding and is kept unless
/// [`SanitizeOptions::IGNORE_ORIENTATION`] is set. A successful result
/// guarantees in-range indices and finite positions for kept triangles; it
/// does not guarantee a manifold or closed mesh.
///
/// ```
/// use meshoptimizer_rs::{sanitize, Positions, SanitizeOptions, Workspace};
///
/// let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [f32::NAN, 0.0, 0.0]];
/// let indices = [
///     0, 1, 2, // kept
///     1, 2, 0, // duplicate of the first face
///     0, 0, 1, // repeated index
///     0, 1, 3, // non-finite position
///     0, 1, 9, // out-of-range index
/// ];
/// let mut workspace = Workspace::default();
/// let mesh = sanitize(&indices, Positions::from_packed(&positions), SanitizeOptions::EMPTY, &mut workspace)?;
/// assert_eq!(mesh.indices, [0, 1, 2]);
/// assert_eq!(mesh.report.duplicate, 1);
/// assert_eq!(mesh.report.removed(), 4);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// `InvalidTopology` if the index count is not a multiple of three.
/// With [`SanitizeOptions::REJECT_INVALID`], `IndexOutOfBounds` or
/// `NumericalFailure` for the first invalid triangle. `NumericalFailure` if
/// bounded hash probing finds no slot. `SizeOverflow` for size
/// overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for
/// failed reservation. An error returns no partial mesh.
pub fn sanitize(
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizedMesh, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (keep, report) = classify(indices, positions, options, &mut budget, &mut work)?;
        let mut out = budget.reserved::<u32>(report.index_count())?;
        emit(indices, &keep, &mut work, |triangle| {
            out.extend_from_slice(triangle);
        })?;
        Ok(SanitizedMesh {
            indices: out,
            report,
        })
    })();
    workspace.finish(&work);
    result
}

/// Upper bound on the indices [`sanitize_into`] writes: the input index count.
/// Not in C++ meshoptimizer. The exact count is
/// [`SanitizeReport::index_count`] of a previous call with the same input.
pub const fn sanitize_bound(index_count: usize) -> usize {
    index_count
}

/// [`sanitize`] into caller storage. Not in C++ meshoptimizer.
///
/// All triangles are classified before anything is written. Then, if
/// `destination` holds fewer than the kept indices, the call returns
/// `BufferTooSmall` and leaves `destination` unchanged; a destination of
/// [`sanitize_bound`] indices is always large enough. On success the first
/// [`SanitizeReport::index_count`] entries hold the kept triangles and the
/// tail is untouched. Every error leaves `destination` unchanged.
///
/// # Errors
/// Same errors as [`sanitize`]. `BufferTooSmall` if `destination` is shorter
/// than the kept index count.
pub fn sanitize_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizeReport, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (keep, report) = classify(indices, positions, options, &mut budget, &mut work)?;
        let Some(out) = destination.get_mut(..report.index_count()) else {
            return Err(Error::BufferTooSmall);
        };
        let mut write = 0;
        emit(indices, &keep, &mut work, |triangle| {
            out[write..write + 3].copy_from_slice(triangle);
            write += 3;
        })?;
        Ok(report)
    })();
    workspace.finish(&work);
    result
}

/// Copy kept triangles in input order. Charges one visit per input triangle
/// before the first write, so a work failure leaves the output untouched.
fn emit(
    indices: &[u32],
    keep: &[bool],
    work: &mut Work,
    mut write: impl FnMut(&[u32; 3]),
) -> Result<(), Error> {
    let triangles = indices.as_chunks::<3>().0;
    work.add(triangles.len())?;
    for (triangle, &kept) in triangles.iter().zip(keep) {
        if kept {
            write(triangle);
        }
    }
    Ok(())
}

fn buckets(n: usize) -> Result<usize, Error> {
    n.checked_add(n / 4)
        .ok_or(Error::SizeOverflow)?
        .max(1)
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)
}

/// Lexicographically smallest cyclic rotation, so rotated copies share a key.
#[inline]
fn canonical(t: [u32; 3]) -> [u32; 3] {
    let a = t;
    let b = [t[1], t[2], t[0]];
    let c = [t[2], t[0], t[1]];
    let first = if b < a { b } else { a };
    if c < first {
        c
    } else {
        first
    }
}

#[inline]
fn key_hash(key: [u32; 3]) -> u32 {
    key[0].wrapping_mul(73856093) ^ key[1].wrapping_mul(19349663) ^ key[2].wrapping_mul(83492791)
}

/// Find `key` or the empty slot where it would go. Each probe is one visit.
fn probe(table: &[[u32; 3]], key: [u32; 3], work: &mut Work) -> Result<(bool, usize), Error> {
    let mask = table.len() - 1;
    let mut bucket = key_hash(key) as usize & mask;
    for step in 0..table.len() {
        work.add(1)?;
        if table[bucket] == key {
            return Ok((true, bucket));
        }
        if table[bucket] == EMPTY_KEY {
            return Ok((false, bucket));
        }
        bucket = (bucket + step + 1) & mask;
    }
    Err(Error::NumericalFailure)
}

#[inline]
fn finite(p: [f32; 3]) -> bool {
    p.iter().all(|v| v.is_finite())
}

/// Classify every triangle without writing output. Returns the keep mask and
/// the report. Scratch is charged to the budget; the hash tables are released
/// before returning so the output can reuse their bytes.
fn classify(
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(Vec<bool>, SanitizeReport), Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    checked_bytes(indices.len(), 4)?;
    let triangles = indices.as_chunks::<3>().0;
    let vertex_count = positions.len();
    let weld = options.contains(SanitizeOptions::WELD_POSITIONS);
    if weld && vertex_count > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    budget.ws.charge_retained()?;
    let mut keep = budget.filled(triangles.len(), false)?;
    let table_len = buckets(triangles.len())?;
    work.add(table_len)?;
    let mut table = budget.filled(table_len, EMPTY_KEY)?;
    let (mut remap, mut weld_table) = if weld {
        let weld_len = weld_table_len(vertex_count)?;
        work.add(vertex_count)?;
        work.add(weld_len)?;
        (
            budget.filled(vertex_count, NONE)?,
            budget.filled(weld_len, NONE)?,
        )
    } else {
        (Vec::new(), Vec::new())
    };
    let mut state = Classify {
        options,
        positions,
        table: &mut table,
        remap: &mut remap,
        weld_table: &mut weld_table,
        work,
        report: SanitizeReport {
            input_triangles: triangles.len(),
            ..SanitizeReport::default()
        },
    };
    let result = if let Some(packed) = positions.packed_values() {
        state.run(triangles, &mut keep, |i| packed.get(i as usize).copied())
    } else {
        state.run(triangles, &mut keep, |i| positions.get(i as usize))
    };
    let report = state.report;
    budget.release(table);
    budget.release(remap);
    budget.release(weld_table);
    result?;
    Ok((keep, report))
}

struct Classify<'a, 'w> {
    options: SanitizeOptions,
    positions: Positions<'a>,
    table: &'a mut [[u32; 3]],
    remap: &'a mut [u32],
    weld_table: &'a mut [u32],
    work: &'w mut Work,
    report: SanitizeReport,
}

impl Classify<'_, '_> {
    fn run(
        &mut self,
        triangles: &[[u32; 3]],
        keep: &mut [bool],
        read: impl Fn(u32) -> Option<[f32; 3]>,
    ) -> Result<(), Error> {
        let reject = self.options.contains(SanitizeOptions::REJECT_INVALID);
        let zero_area = self.options.contains(SanitizeOptions::ZERO_AREA);
        let weld = self.options.contains(SanitizeOptions::WELD_POSITIONS);
        let reversed = self.options.contains(SanitizeOptions::IGNORE_ORIENTATION);
        for (&[a, b, c], kept) in triangles.iter().zip(keep.iter_mut()) {
            self.work.add(1)?;
            let (Some(pa), Some(pb), Some(pc)) = (read(a), read(b), read(c)) else {
                if reject {
                    return Err(Error::IndexOutOfBounds);
                }
                self.report.out_of_range += 1;
                continue;
            };
            if !(finite(pa) && finite(pb) && finite(pc)) {
                if reject {
                    return Err(Error::NumericalFailure);
                }
                self.report.non_finite += 1;
                continue;
            }
            if a == b || a == c || b == c {
                self.report.repeated_index += 1;
                continue;
            }
            if zero_area && (pa == pb || pa == pc || pb == pc) {
                self.report.zero_area += 1;
                continue;
            }
            let corners = if weld {
                [self.weld(a)?, self.weld(b)?, self.weld(c)?]
            } else {
                [a, b, c]
            };
            let key = canonical(corners);
            let (found, slot) = probe(self.table, key, self.work)?;
            if found {
                self.report.duplicate += 1;
                continue;
            }
            if reversed {
                let mirror = canonical([corners[0], corners[2], corners[1]]);
                if mirror != key && probe(self.table, mirror, self.work)?.0 {
                    self.report.reversed_duplicate += 1;
                    continue;
                }
            }
            self.table[slot] = key;
            *kept = true;
            self.report.output_triangles += 1;
        }
        Ok(())
    }

    fn weld(&mut self, index: u32) -> Result<u32, Error> {
        weld_position(
            index,
            self.remap,
            self.weld_table,
            self.positions,
            self.work,
        )
    }
}
