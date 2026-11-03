// Translation of meshoptimizer 1.3 src/overdrawoptimizer.cpp; see LICENSE.
use crate::workspace::{checked_bytes, output, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

/// Reorder cache-optimized triangles using upstream `meshopt_optimizeOverdraw`.
///
/// `threshold` is the permitted cache degradation factor (for example 1.05).
/// All finite thresholds retain upstream behavior, including values below one.
/// Geometry and intermediates must be finite. All supplied vertices contribute
/// to the mesh centroid, including vertices not referenced by any triangle.
/// Empty and degenerate meshes are supported. No cache optimization is implicit.
///
/// # Errors
/// `InvalidTopology` for incomplete triangles or `IndexOutOfBounds` for invalid vertex references.
/// `InvalidParameter` for a non-finite threshold; `NumericalFailure` for non-finite geometry or calculations.
/// `SizeOverflow` for size overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed reservation.
#[inline]
pub fn optimize_overdraw_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<Vec<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        optimize_overdraw_u32(indices, positions, threshold, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn optimize_overdraw_u32(
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mesh_centroid = validate(indices, positions, threshold, &mut work)?;
        prepare(
            indices.len(),
            positions.len(),
            checked_bytes(indices.len(), 4)?,
            workspace,
        )?;
        let mut destination = output(indices.len())?;
        kernel(
            &mut destination,
            indices,
            (positions, mesh_centroid),
            threshold,
            workspace,
            &mut work,
            checked_bytes(indices.len(), 4)?,
        )?;
        Ok(destination)
    })();
    workspace.finish(&work);
    result
}

/// Write upstream overdraw ordering to the used prefix of a caller buffer.
///
/// The unused tail is untouched. Validation and reservation precede writes;
/// a later numerical or work-limit failure may leave output partially modified.
/// Caller-owned destination storage does not count toward the byte limit.
///
/// # Errors
/// Same errors as [`optimize_overdraw`]. `BufferTooSmall` if the destination is shorter than the input.
#[inline]
pub fn optimize_overdraw_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            optimize_overdraw_into_u32(destination, indices, positions, threshold, workspace)
        })
    })
}

fn optimize_overdraw_into_u32(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mesh_centroid = validate(indices, positions, threshold, &mut work)?;
        if destination.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        prepare(indices.len(), positions.len(), 0, workspace)?;
        kernel(
            destination,
            indices,
            (positions, mesh_centroid),
            threshold,
            workspace,
            &mut work,
            0,
        )
    })();
    workspace.finish(&work);
    result
}

/// Destructively apply upstream overdraw ordering, leaving input unchanged on error.
/// A fallibly allocated temporary output counts toward the byte limit.
///
/// # Errors
/// Same errors as [`optimize_overdraw`].
#[inline]
pub fn optimize_overdraw_in_place_generic<I: crate::Index>(
    indices: &mut [I],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    I::with_mut(indices, workspace, |indices, workspace| {
        optimize_overdraw_in_place_u32(indices, positions, threshold, workspace)
    })
}

fn optimize_overdraw_in_place_u32(
    indices: &mut [u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = optimize_overdraw_generic(indices, positions, threshold, workspace)?;
    indices.copy_from_slice(&result);
    Ok(())
}

fn validate(
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    work: &mut Work,
) -> Result<[f32; 3], Error> {
    topology(indices, positions.len(), work)?;
    if !threshold.is_finite() {
        return Err(Error::InvalidParameter);
    }
    // A nonfinite coordinate makes its running sum nonfinite for good: inf
    // and NaN never return to a finite value under addition. Finite sums
    // therefore prove every coordinate finite, and are the same sums the
    // checked scan computes. Otherwise replay that scan for its error prefix.
    if let Some(values) = positions.packed_values() {
        if work.covers(values.len())? {
            let mut centroid = [0f32; 3];
            for p in values {
                for j in 0..3 {
                    centroid[j] += p[j];
                }
            }
            if centroid.iter().all(|x| x.is_finite()) {
                work.add_covered(values.len());
                return Ok(centroid);
            }
        }
    }
    let mut centroid = [0f32; 3];
    positions.for_each_counted(work, |p| {
        if !p.iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
        for j in 0..3 {
            centroid[j] += p[j];
        }
        Ok(())
    })?;
    Ok(centroid)
}

fn prepare(n: usize, v: usize, out: usize, ws: &mut Workspace) -> Result<(), Error> {
    ws.acquire(crate::workspace::ScratchFamily::Overdraw);
    if n == 0 {
        return ws.prepare([0; 4], out);
    }
    let f = n / 3;
    let ints = f
        .checked_mul(2)
        .and_then(|x| x.checked_add(v))
        .and_then(|x| x.checked_add(1))
        .ok_or(Error::SizeOverflow)?;
    ws.prepare([ints, 0, 0, 0], out)
}

#[inline(always)]
fn update(triangle: &[u32; 3], stamps: &mut [u32], time: &mut u32) -> u32 {
    // One proof covers all three stamp accesses in the unrolled triangle.
    let max = triangle[0].max(triangle[1]).max(triangle[2]) as usize;
    assert!(max < stamps.len());
    let mut misses = 0;
    for &index in triangle {
        let stamp = &mut stamps[index as usize];
        if time.wrapping_sub(*stamp) > 16 {
            *stamp = *time;
            *time = time.wrapping_add(1);
            misses += 1;
        }
    }
    misses
}

fn finite(x: f32) -> Result<f32, Error> {
    if x.is_finite() {
        Ok(x)
    } else {
        Err(Error::NumericalFailure)
    }
}

#[derive(Default)]
struct ClusterGeometry {
    area_sum: f32,
    centroid: [f32; 3],
    normal: [f32; 3],
}
impl ClusterGeometry {
    #[inline(always)]
    fn triangle(&mut self, [p0, p1, p2]: [[f32; 3]; 3], sqrt: &mut impl FnMut(f32) -> f32) {
        let mut p10 = [0f32; 3];
        let mut p20 = [0f32; 3];
        for j in 0..3 {
            p10[j] = p1[j] - p0[j];
            p20[j] = p2[j] - p0[j];
        }
        let n = [
            p10[1] * p20[2] - p10[2] * p20[1],
            p10[2] * p20[0] - p10[0] * p20[2],
            p10[0] * p20[1] - p10[1] * p20[0],
        ];
        // A nonfinite squared norm produces a nonfinite area. Areas are
        // nonnegative, so the checked cluster sum below cannot hide it.
        let area = sqrt(n[0] * n[0] + n[1] * n[1] + n[2] * n[2]);
        for j in 0..3 {
            self.centroid[j] += (p0[j] + p1[j] + p2[j]) * (area / 3.);
            self.normal[j] += n[j];
        }
        self.area_sum += area;
    }

    /// The cluster's sort value, or `None` exactly when any intermediate the
    /// 0.2.14 sequence checked one by one is nonfinite (PROOFS.md, score).
    /// Every intermediate except a reciprocal propagates a nonfinite value
    /// into the dot product; the reciprocals' inputs are checked directly.
    #[inline(always)]
    fn score(self, mesh_centroid: [f32; 3], sqrt: &mut impl FnMut(f32) -> f32) -> Option<f32> {
        let Self {
            area_sum,
            mut centroid,
            mut normal,
        } = self;
        let inv_area = if area_sum == 0. { 0. } else { 1. / area_sum };
        for x in &mut centroid {
            *x *= inv_area;
        }
        let norm = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
        let length = sqrt(norm);
        let inv_length = if length == 0. { 0. } else { 1. / length };
        for x in &mut normal {
            *x *= inv_length;
        }
        let cv = [
            centroid[0] - mesh_centroid[0],
            centroid[1] - mesh_centroid[1],
            centroid[2] - mesh_centroid[2],
        ];
        let dot = cv[0] * normal[0] + cv[1] * normal[1] + cv[2] * normal[2];
        (area_sum.is_finite() & norm.is_finite() & dot.is_finite()).then_some(dot)
    }
}

// Prepaid is used only when every visit of the phase fits and has been or will
// be charged in total; for soft clustering also only when the target
// multiplication cannot overflow. All other inputs retain Work's original
// per-visit path.
trait ClusterMeter {
    /// Soft-cluster targets are proved finite on this path (PROOFS.md, target).
    const PREPAID: bool;
    fn add(&mut self, n: usize) -> Result<(), Error>;
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error>;
}
impl ClusterMeter for Work {
    const PREPAID: bool = false;
    #[inline(always)]
    fn add(&mut self, n: usize) -> Result<(), Error> {
        Work::add(self, n)
    }
    #[inline(always)]
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        Work::scan(self, values, visit)
    }
}
struct ClusterPrepaid;
impl ClusterMeter for ClusterPrepaid {
    const PREPAID: bool = true;
    #[inline(always)]
    fn add(&mut self, _n: usize) -> Result<(), Error> {
        Ok(())
    }
    #[inline(always)]
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        for value in values {
            visit(value)?;
        }
        Ok(())
    }
}

fn soft_clusters<M: ClusterMeter>(
    triangles: &[[u32; 3]],
    hard: &[u32],
    soft: &mut [u32],
    stamps: &mut [u32],
    threshold: f32,
    work: &mut M,
) -> Result<usize, Error> {
    let faces = triangles.len();
    let mut time = 0u32;
    let mut soft_count = 0;
    for (&start, end) in hard
        .iter()
        .zip(hard[1..].iter().copied().map(|x| x as usize).chain([faces]))
    {
        work.add(1)?;
        let start = start as usize;
        time = time.wrapping_add(17);
        let mut misses = 0u32;
        work.scan(triangles[start..end].iter(), |tri| {
            misses += update(tri, stamps, &mut time);
            Ok(())
        })?;
        let target = threshold * (misses as f32 / (end - start) as f32);
        if !M::PREPAID {
            finite(target)?;
        }
        soft[soft_count] = start as u32;
        soft_count += 1;
        time = time.wrapping_add(17);
        let mut running_misses = 0;
        let mut running_faces = 0;
        work.scan(triangles[start..end].iter().enumerate(), |(offset, tri)| {
            running_misses += update(tri, stamps, &mut time);
            running_faces += 1;
            if running_misses as f32 / running_faces as f32 <= target {
                soft[soft_count] = (start + offset + 1) as u32;
                soft_count += 1;
                time = time.wrapping_add(17);
                running_misses = 0;
                running_faces = 0;
            }
            Ok(())
        })?;
        if soft[soft_count - 1] != start as u32 {
            soft_count -= 1;
        }
    }
    Ok(soft_count)
}

fn kernel(
    dest: &mut [u32],
    indices: &[u32],
    geometry: (Positions<'_>, [f32; 3]),
    threshold: f32,
    ws: &mut Workspace,
    work: &mut Work,
    output_bytes: usize,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let (positions, mut mesh_centroid) = geometry;
    let faces = indices.len() / 3;
    let triangles = indices.as_chunks::<3>().0;
    let v = positions.len();
    let (stamps, rest) = ws.integers.split_at_mut(v);
    let (hard, rest) = rest.split_at_mut(faces);
    let soft = rest;
    work.add(v)?;
    stamps.fill(0);
    let mut time = 17;
    let mut hard_count = 0;
    work.scan(triangles.iter().enumerate(), |(i, tri)| {
        let misses = update(tri, stamps, &mut time);
        if i == 0 || misses == 3 {
            hard[hard_count] = i as u32;
            hard_count += 1;
        }
        Ok(())
    })?;
    work.add(v)?;
    stamps.fill(0);
    let bound = faces.checked_mul(2).and_then(|n| n.checked_add(hard_count));
    // Every nonempty hard cluster has at most three misses per triangle.
    // At this threshold magnitude its target is finite, so the whole phase
    // is infallible after precharging its exact hard_count + 2*faces visits.
    let prepaid = match bound {
        Some(n) if threshold.abs() <= f32::MAX / 4.0 => work.precharge(n, 1)?,
        _ => false,
    };
    let soft_count = if prepaid {
        soft_clusters(
            triangles,
            &hard[..hard_count],
            soft,
            stamps,
            threshold,
            &mut ClusterPrepaid,
        )?
    } else {
        soft_clusters(
            triangles,
            &hard[..hard_count],
            soft,
            stamps,
            threshold,
            work,
        )?
    };
    // Hard boundaries are dead after soft clustering; reuse their storage for
    // sort order. Only live clusters need keys and floating-point sort data.
    let ints = ws.integers.len();
    ws.prepare([ints, soft_count, 0, soft_count], output_bytes)?;
    let (_, rest) = ws.integers.split_at_mut(v);
    let (order, soft) = rest.split_at_mut(faces);
    let soft = &soft[..soft_count];
    let data = &mut ws.floats[..soft_count];
    let keys = &mut ws.keys[..soft_count];
    // Scalar IEEE sqrt is correctly rounded, like the pinned software backend.
    // A hardware root avoids hashing and software-root misses on varied faces;
    // retain the original cached backend when std is unavailable.
    #[cfg(feature = "std")]
    let mut sqrt = f32::sqrt;
    #[cfg(not(feature = "std"))]
    let mut sqrt_cache = crate::math::SqrtCache::new();
    #[cfg(not(feature = "std"))]
    let mut sqrt = |value| sqrt_cache.sqrt(value);
    // When every remaining charge fits, none of them can fail: packed geometry
    // and emission then run without per-step accounting and charge the same
    // totals, or the same prefix before a numerical failure (PROOFS.md, W).
    if let (Some(values), Some(charges)) = (
        positions.packed_values(),
        remaining_charges(v, faces, soft_count, indices.len()),
    ) {
        if work.covers(charges)? {
            work.add_covered(v);
            for x in &mut mesh_centroid {
                *x = finite(*x)? / v as f32;
            }
            let mut visits = 0;
            for (cluster, (d, &start)) in data.iter_mut().zip(soft).enumerate() {
                let start = start as usize;
                let end = soft.get(cluster + 1).map_or(faces, |&end| end as usize);
                let mut geometry = ClusterGeometry::default();
                for tri in &triangles[start..end] {
                    let triangle = [
                        values[tri[0] as usize],
                        values[tri[1] as usize],
                        values[tri[2] as usize],
                    ];
                    geometry.triangle(triangle, &mut sqrt);
                }
                visits += 1 + (end - start);
                let Some(score) = geometry.score(mesh_centroid, &mut sqrt) else {
                    work.add_covered(visits);
                    return Err(Error::NumericalFailure);
                };
                *d = score;
            }
            work.add_covered(visits);
            emit(dest, indices, soft, order, data, keys, &mut ClusterPrepaid)?;
            // Emission visits each cluster once in four passes, the histogram,
            // and each index of every cluster once.
            work.add_covered(4 * soft_count + 2048 + 3 * (visits - soft_count));
            return Ok(());
        }
    }
    // Validation already accumulated these coordinates in the same order.
    // Retain the original centroid scan's charge here, including its exact
    // exhaustion prefix, and defer numerical checks until this original point.
    work.scan(0..v, |_| Ok(()))?;
    for x in &mut mesh_centroid {
        *x = finite(*x)? / v as f32;
    }
    // For strided storage, dispatch once if the entire geometry phase fits.
    // Packed reads are already cheap; retain their per-cluster scan so the
    // triangle loop needs no extra cluster-end branch.
    // ClusterGeneric visits can be charged after the scan: all charges fit, and the
    // number of started clusters preserves usage on the first numerical error.
    if positions.packed_values().is_none() && work.covers(faces + soft_count)? {
        let mut cluster = 0;
        let mut started_clusters = 1;
        let mut face = 0;
        let mut geometry = ClusterGeometry::default();
        let result = positions.for_each_triangle_counted(triangles, work, |triangle| {
            geometry.triangle(triangle, &mut sqrt);
            face += 1;
            let end = if cluster + 1 < soft_count {
                soft[cluster + 1] as usize
            } else {
                faces
            };
            if face == end {
                data[cluster] = core::mem::take(&mut geometry)
                    .score(mesh_centroid, &mut sqrt)
                    .ok_or(Error::NumericalFailure)?;
                cluster += 1;
                started_clusters += usize::from(cluster < soft_count);
            }
            Ok(())
        });
        work.add(started_clusters)?;
        result?;
    } else {
        for cluster in 0..soft_count {
            work.add(1)?;
            let start = soft[cluster] as usize * 3;
            let end = if cluster + 1 < soft_count {
                soft[cluster + 1] as usize * 3
            } else {
                indices.len()
            };
            let mut geometry = ClusterGeometry::default();
            positions.for_each_triangle_counted(
                indices[start..end].as_chunks::<3>().0,
                work,
                |triangle| {
                    geometry.triangle(triangle, &mut sqrt);
                    Ok(())
                },
            )?;
            data[cluster] = geometry
                .score(mesh_centroid, &mut sqrt)
                .ok_or(Error::NumericalFailure)?;
        }
    }
    emit(dest, indices, soft, order, data, keys, work)
}

/// Every charge after soft clustering on the per-step path: the retained
/// centroid scan, one visit per cluster and per face, the maximum, key and
/// order scans, the histogram, and one visit per emitted cluster and index.
fn remaining_charges(v: usize, faces: usize, clusters: usize, n: usize) -> Option<usize> {
    v.checked_add(faces)?
        .checked_add(clusters.checked_mul(5)?)?
        .checked_add(2048)?
        .checked_add(n)
}

/// Sort clusters by their scores and copy them to the destination.
fn emit<M: ClusterMeter>(
    dest: &mut [u32],
    indices: &[u32],
    soft: &[u32],
    order: &mut [u32],
    data: &[f32],
    keys: &mut [u16],
    work: &mut M,
) -> Result<(), Error> {
    let soft_count = soft.len();
    let mut max = 1e-3f32;
    work.scan(data.iter().copied(), |d| {
        if max < d.abs() {
            max = d.abs();
        }
        Ok(())
    })?;
    let mut histogram = [0u32; 2048];
    work.scan(data.iter().zip(keys.iter_mut()), |(&d, sort_key)| {
        let key = 0.5f32 - 0.5f32 * (d / max);
        let clamped = key.clamp(0., 1.);
        *sort_key = ((clamped * 2047. + 0.5) as u16) & 2047;
        histogram[*sort_key as usize] += 1;
        Ok(())
    })?;
    let mut sum = 0;
    work.add(histogram.len())?;
    for h in &mut histogram {
        let count = *h;
        *h = sum;
        sum += count;
    }
    work.scan(keys.iter().enumerate(), |(i, &key)| {
        let h = &mut histogram[key as usize];
        order[*h as usize] = i as u32;
        *h += 1;
        Ok(())
    })?;
    let mut offset = 0;
    for &cluster in &order[..soft_count] {
        work.add(1)?;
        let cluster = cluster as usize;
        let start = soft[cluster] as usize * 3;
        let end = if cluster + 1 < soft_count {
            soft[cluster + 1] as usize * 3
        } else {
            indices.len()
        };
        work.add(end - start)?;
        if end - start == 3 {
            // A constant-length copy for the common one-triangle cluster.
            dest[offset..offset + 3].copy_from_slice(&indices[start..start + 3]);
        } else {
            dest[offset..offset + end - start].copy_from_slice(&indices[start..end]);
        }
        offset += end - start;
    }
    Ok(())
}

/// The source-compatible u32 entry point; see [`optimize_overdraw_generic`] for selectable index widths.
#[inline]
pub fn optimize_overdraw(
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    optimize_overdraw_generic::<u32>(indices, positions, threshold, workspace)
}

/// The source-compatible u32 entry point; see [`optimize_overdraw_into_generic`] for selectable index widths.
#[inline]
pub fn optimize_overdraw_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    optimize_overdraw_into_generic::<u32>(destination, indices, positions, threshold, workspace)
}

/// The source-compatible u32 entry point; see [`optimize_overdraw_in_place_generic`] for selectable index widths.
#[inline]
pub fn optimize_overdraw_in_place(
    indices: &mut [u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    optimize_overdraw_in_place_generic::<u32>(indices, positions, threshold, workspace)
}

#[cfg(all(test, feature = "std"))]
mod tests {
    #[test]
    fn scalar_root_matches_pinned_backend() {
        // Every finite nonnegative exponent, both mantissa ends, and a stride
        // through each binade, including subnormals and overflow-adjacent norms.
        for exponent in 0u32..255 {
            for mantissa in (0..1 << 23).step_by(4093).chain([1, 0x7fffff]) {
                let bits = exponent << 23 | mantissa;
                let value = f32::from_bits(bits);
                assert_eq!(
                    value.sqrt().to_bits(),
                    crate::math::sqrt(value).to_bits(),
                    "{bits:08x}"
                );
            }
        }
        assert_eq!(
            (-0.0f32).sqrt().to_bits(),
            crate::math::sqrt(-0.0).to_bits()
        );
    }
}
