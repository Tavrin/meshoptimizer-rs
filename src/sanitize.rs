//! Triangle sanitation. This module is an extension: it is not in C++
//! meshoptimizer, and no existing function's behaviour depends on it.

use crate::budget::Budget;
use crate::remap::{weld_position, weld_table_len};
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

const NONE: u32 = u32::MAX;

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
    /// Also drop triangles with two corners at bit-identical positions
    /// (signed zeros compare equal, so `0.0` and `-0.0` count as identical).
    /// Nothing else is tested: triangles whose three corners are collinear
    /// but distinct, or merely tiny, are kept because no area is computed.
    pub const COINCIDENT_CORNERS: Self = Self(2);
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
/// coincident corners, duplicate, reversed duplicate. So `input_triangles` equals
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
    /// Removed with [`SanitizeOptions::COINCIDENT_CORNERS`]: two corners have
    /// bit-identical positions (signed zeros compare equal).
    pub coincident_corners: usize,
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
            + self.coincident_corners
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
pub struct SanitizedMeshGeneric<I: crate::Index = u32> {
    /// Kept triangles in input order, with their original corners and winding.
    pub indices: Vec<I>,
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
/// `NumericalFailure` for the first invalid triangle. `SizeOverflow` for size
/// overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for
/// failed reservation. An error returns no partial mesh.
///
/// Work is `O(n log n)` in the number of triangles that pass the per-triangle
/// checks, whatever the input: duplicates are found by sorting, not hashing.
/// Scratch storage is proportional to those triangles, never to the position
/// count.
#[inline]
pub fn sanitize_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizedMeshGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        sanitize_u32(indices, positions, options, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn sanitize_u32(
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizedMeshGeneric, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (keep, report) = classify(indices, positions, options, &mut budget, &mut work)?;
        let mut out = budget.reserved::<u32>(report.index_count())?;
        emit(indices, &keep, &mut work, |triangle| {
            out.extend_from_slice(triangle);
        })?;
        Ok(SanitizedMeshGeneric {
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
#[inline]
pub fn sanitize_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizeReport, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            sanitize_into_u32(destination, indices, positions, options, workspace)
        })
    })
}

fn sanitize_into_u32(
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

/// Lexicographically smallest cyclic rotation, so rotated copies share a key.
#[inline]
pub(crate) fn canonical(t: [u32; 3]) -> [u32; 3] {
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

/// Comparison steps charged per element of a sort or binary search over `n`
/// elements: `ceil(log2(n)) + 1`, an upper bound on both.
pub(crate) fn steps(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()) as usize + 1
}

/// Work charged for sorting `n` elements, or for `n` binary searches in `n`
/// elements.
pub(crate) fn sort_work(n: usize) -> Result<usize, Error> {
    n.checked_mul(steps(n)).ok_or(Error::SizeOverflow)
}

/// One candidate triangle in the duplicate sort. Faces that are equal, or
/// mirror images when `IGNORE_ORIENTATION` is set, share a `class`.
#[derive(Clone, Copy)]
struct Face {
    /// Canonical corners of the face, or of its mirror image if that sorts lower.
    class: [u32; 3],
    /// Input triangle index; ties inside a class resolve to the lowest index.
    index: u32,
    /// Whether this triangle is the mirror image of `class`.
    mirrored: bool,
}

#[inline]
fn finite(p: [f32; 3]) -> bool {
    p.iter().all(|v| v.is_finite())
}

/// Classify every triangle without writing output. Returns the keep mask and
/// the report. Scratch is charged to the budget and released before returning
/// so the output can reuse its bytes.
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
    budget.ws.charge_retained()?;
    let mut keep = budget.filled(triangles.len(), false)?;
    let mut report = SanitizeReport {
        input_triangles: triangles.len(),
        ..SanitizeReport::default()
    };
    let candidates = if let Some(packed) = positions.packed_values() {
        screen(options, triangles, &mut keep, &mut report, work, |i| {
            packed.get(i as usize).copied()
        })?
    } else {
        screen(options, triangles, &mut keep, &mut report, work, |i| {
            positions.get(i as usize)
        })?
    };
    // `keep` now marks the candidates. Scratch below is sized by them alone.
    if candidates > 0 {
        let read = |i: u32| positions.get(i as usize);
        dedup(
            options,
            triangles,
            &mut keep,
            &mut report,
            budget,
            work,
            read,
        )?;
    }
    Ok((keep, report))
}

/// Per-triangle checks in input order. Marks every triangle that survives them
/// in `keep` and returns how many did.
pub(crate) fn screen(
    options: SanitizeOptions,
    triangles: &[[u32; 3]],
    keep: &mut [bool],
    report: &mut SanitizeReport,
    work: &mut Work,
    read: impl Fn(u32) -> Option<[f32; 3]>,
) -> Result<usize, Error> {
    let reject = options.contains(SanitizeOptions::REJECT_INVALID);
    let coincident = options.contains(SanitizeOptions::COINCIDENT_CORNERS);
    let mut candidates = 0;
    for (&[a, b, c], kept) in triangles.iter().zip(keep.iter_mut()) {
        work.add(1)?;
        let (Some(pa), Some(pb), Some(pc)) = (read(a), read(b), read(c)) else {
            if reject {
                return Err(Error::IndexOutOfBounds);
            }
            report.out_of_range += 1;
            continue;
        };
        if !(finite(pa) && finite(pb) && finite(pc)) {
            if reject {
                return Err(Error::NumericalFailure);
            }
            report.non_finite += 1;
            continue;
        }
        if a == b || a == c || b == c {
            report.repeated_index += 1;
            continue;
        }
        if coincident && (pa == pb || pa == pc || pb == pc) {
            report.coincident_corners += 1;
            continue;
        }
        *kept = true;
        candidates += 1;
    }
    Ok(candidates)
}

/// Clear `keep` for every candidate that repeats an earlier candidate, counting
/// it as a duplicate or reversed duplicate, and set `output_triangles`.
///
/// Candidates are sorted by `(class, input index)`; the first of each class is
/// kept. A later member with the first member's orientation is a duplicate,
/// the other orientation a reversed duplicate. Worst case `O(n log n)` with no
/// dependence on the values, unlike hashing.
fn dedup(
    options: SanitizeOptions,
    triangles: &[[u32; 3]],
    keep: &mut [bool],
    report: &mut SanitizeReport,
    budget: &mut Budget<'_>,
    work: &mut Work,
    read: impl Fn(u32) -> Option<[f32; 3]>,
) -> Result<(), Error> {
    let candidates = keep.iter().filter(|&&k| k).count();
    u32::try_from(triangles.len()).map_err(|_| Error::SizeOverflow)?;
    let welded = if options.contains(SanitizeOptions::WELD_POSITIONS) {
        Some(weld_candidates(
            triangles, keep, candidates, budget, work, &read,
        )?)
    } else {
        None
    };
    let reversed = options.contains(SanitizeOptions::IGNORE_ORIENTATION);
    work.add(candidates)?;
    work.add(sort_work(candidates)?)?;
    let mut faces = budget.reserved::<Face>(candidates)?;
    let mut nth = 0;
    for (index, (triangle, &kept)) in triangles.iter().zip(keep.iter()).enumerate() {
        if !kept {
            continue;
        }
        let corners = welded.as_ref().map_or(*triangle, |w| w[nth]);
        nth += 1;
        let key = canonical(corners);
        let mirror = canonical([corners[0], corners[2], corners[1]]);
        let (class, mirrored) = if reversed && mirror < key {
            (mirror, true)
        } else {
            (key, false)
        };
        faces.push(Face {
            class,
            index: index as u32,
            mirrored,
        });
    }
    if let Some(welded) = welded {
        budget.release(welded);
    }
    faces.sort_unstable_by(|a, b| a.class.cmp(&b.class).then(a.index.cmp(&b.index)));
    let mut first: Option<([u32; 3], bool)> = None;
    for face in &faces {
        match first {
            Some((class, mirrored)) if class == face.class => {
                keep[face.index as usize] = false;
                if mirrored == face.mirrored {
                    report.duplicate += 1;
                } else {
                    report.reversed_duplicate += 1;
                }
            }
            _ => {
                first = Some((face.class, face.mirrored));
                report.output_triangles += 1;
            }
        }
    }
    budget.release(faces);
    Ok(())
}

/// Weld the corners of every candidate triangle, in input order. Only vertices
/// that candidates reference are welded: they are gathered by sorting, copied
/// into a compact position array, and welded there with the same hash and
/// equality as `generate_position_remap`. Tables are sized by those vertices,
/// never by the position count.
fn weld_candidates(
    triangles: &[[u32; 3]],
    keep: &[bool],
    candidates: usize,
    budget: &mut Budget<'_>,
    work: &mut Work,
    read: &impl Fn(u32) -> Option<[f32; 3]>,
) -> Result<Vec<[u32; 3]>, Error> {
    let corner_count = candidates.checked_mul(3).ok_or(Error::SizeOverflow)?;
    work.add(corner_count)?;
    work.add(sort_work(corner_count)?)?;
    let mut referenced = budget.reserved::<u32>(corner_count)?;
    for (triangle, _) in triangles.iter().zip(keep).filter(|(_, &k)| k) {
        referenced.extend_from_slice(triangle);
    }
    referenced.sort_unstable();
    referenced.dedup();
    let unique = referenced.len();
    if unique >= NONE as usize {
        return Err(Error::SizeOverflow);
    }
    work.add(unique)?;
    let mut compact = budget.reserved::<[f32; 3]>(unique)?;
    for &vertex in &referenced {
        compact.push(read(vertex).ok_or(Error::IndexOutOfBounds)?);
    }
    let table_len = weld_table_len(unique)?;
    work.add(unique)?;
    work.add(table_len)?;
    let mut remap = budget.filled(unique, NONE)?;
    let mut table = budget.filled(table_len, NONE)?;
    work.add(sort_work(corner_count)?)?;
    let mut welded = budget.reserved::<[u32; 3]>(candidates)?;
    let compact_positions = Positions::from_packed(&compact);
    let result = (|| {
        for (triangle, _) in triangles.iter().zip(keep).filter(|(_, &k)| k) {
            let mut corners = [0; 3];
            for (corner, &vertex) in corners.iter_mut().zip(triangle) {
                let id = referenced
                    .binary_search(&vertex)
                    .map_err(|_| Error::IndexOutOfBounds)?;
                *corner =
                    weld_position(id as u32, &mut remap, &mut table, compact_positions, work)?;
            }
            welded.push(corners);
        }
        Ok(())
    })();
    budget.release(referenced);
    budget.release(compact);
    budget.release(remap);
    budget.release(table);
    result?;
    Ok(welded)
}

/// The source-compatible u32 entry point; see [`sanitize_generic`] for selectable index widths.
#[inline]
pub fn sanitize(
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizedMesh, Error> {
    sanitize_generic::<u32>(indices, positions, options, workspace).map(Into::into)
}

/// The source-compatible u32 entry point; see [`sanitize_into_generic`] for selectable index widths.
#[inline]
pub fn sanitize_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizeReport, Error> {
    sanitize_into_generic::<u32>(destination, indices, positions, options, workspace)
}

/// The source-compatible u32 form of [`SanitizedMeshGeneric`].
/// Cleaned triangle list from [`sanitize`]. Not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SanitizedMesh {
    /// Kept triangles in input order, with their original corners and winding.
    pub indices: Vec<u32>,
    /// Counts by removal reason.
    pub report: SanitizeReport,
}
impl From<SanitizedMeshGeneric<u32>> for SanitizedMesh {
    #[inline]
    fn from(value: SanitizedMeshGeneric<u32>) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}

impl From<SanitizedMesh> for SanitizedMeshGeneric<u32> {
    #[inline]
    fn from(value: SanitizedMesh) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}
