// Translation of meshoptimizer 1.3 src/overdrawoptimizer.cpp; see LICENSE.
use crate::workspace::{checked_bytes, output, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

/// Reorder cache-optimized triangles using upstream `meshopt_optimizeOverdraw`.
///
/// `threshold` is the permitted cache degradation factor (Moss uses 1.05).
/// All finite thresholds retain upstream behavior, including values below one.
/// Geometry and intermediates must be finite. All supplied vertices contribute
/// to the mesh centroid, including vertices not referenced by any triangle.
/// Empty and degenerate meshes are supported. No cache optimization is implicit.
pub fn optimize_overdraw(
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
pub fn optimize_overdraw_into(
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
pub fn optimize_overdraw_in_place(
    indices: &mut [u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = optimize_overdraw(indices, positions, threshold, workspace)?;
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

    #[inline(always)]
    fn score(
        self,
        mesh_centroid: [f32; 3],
        sqrt: &mut impl FnMut(f32) -> f32,
    ) -> Result<f32, Error> {
        let Self {
            area_sum,
            mut centroid,
            mut normal,
        } = self;
        finite(area_sum)?;
        let inv_area = if area_sum == 0. {
            0.
        } else {
            finite(1. / area_sum)?
        };
        for x in &mut centroid {
            *x = finite(*x * inv_area)?;
        }
        let length = sqrt(finite(
            normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2],
        )?);
        let inv_length = if length == 0. {
            0.
        } else {
            finite(1. / length)?
        };
        for x in &mut normal {
            *x = finite(*x * inv_length)?;
        }
        let cv = [
            finite(centroid[0] - mesh_centroid[0])?,
            finite(centroid[1] - mesh_centroid[1])?,
            finite(centroid[2] - mesh_centroid[2])?,
        ];
        finite(cv[0] * normal[0] + cv[1] * normal[1] + cv[2] * normal[2])
    }
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
    time = 0;
    let mut soft_count = 0;
    for h in 0..hard_count {
        work.add(1)?;
        let start = hard[h] as usize;
        let end = if h + 1 < hard_count {
            hard[h + 1] as usize
        } else {
            faces
        };
        time = time.wrapping_add(17);
        let mut misses = 0u32;
        work.scan(triangles[start..end].iter(), |tri| {
            misses += update(tri, stamps, &mut time);
            Ok(())
        })?;
        let target = finite(threshold * (misses as f32 / (end - start) as f32))?;
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
    // Hard boundaries are dead after soft clustering; reuse their storage for
    // sort order. Only live clusters need keys and floating-point sort data.
    let ints = ws.integers.len();
    ws.prepare([ints, soft_count, 0, soft_count], output_bytes)?;
    let (_, rest) = ws.integers.split_at_mut(v);
    let (order, soft) = rest.split_at_mut(faces);
    // Validation already accumulated these coordinates in the same order.
    // Retain the original centroid scan's charge here, including its exact
    // exhaustion prefix, and defer numerical checks until this original point.
    work.scan(0..v, |_| Ok(()))?;
    for x in &mut mesh_centroid {
        *x = finite(*x)? / v as f32;
    }
    let data = &mut ws.floats;
    // Scalar IEEE sqrt is correctly rounded, like the pinned software backend.
    // A hardware root avoids hashing and software-root misses on varied faces;
    // retain the original cached backend when std is unavailable.
    #[cfg(feature = "std")]
    let mut sqrt = f32::sqrt;
    #[cfg(not(feature = "std"))]
    let mut sqrt_cache = crate::math::SqrtCache::new();
    #[cfg(not(feature = "std"))]
    let mut sqrt = |value| sqrt_cache.sqrt(value);
    // For strided storage, dispatch once if the entire geometry phase fits.
    // Packed reads are already cheap; retain their per-cluster scan so the
    // triangle loop needs no extra cluster-end branch.
    // Cluster visits can be charged after the scan: all charges fit, and the
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
                data[cluster] = core::mem::take(&mut geometry).score(mesh_centroid, &mut sqrt)?;
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
            data[cluster] = geometry.score(mesh_centroid, &mut sqrt)?;
        }
    }

    let mut max = 1e-3f32;
    work.scan(data[..soft_count].iter().copied(), |d| {
        if max < d.abs() {
            max = d.abs();
        }
        Ok(())
    })?;
    let mut histogram = [0u32; 2048];
    work.scan(
        data[..soft_count].iter().zip(&mut ws.keys[..soft_count]),
        |(&d, sort_key)| {
            let key = 0.5f32 - 0.5f32 * (d / max);
            let clamped = key.clamp(0., 1.);
            *sort_key = ((clamped * 2047. + 0.5) as u16) & 2047;
            histogram[*sort_key as usize] += 1;
            Ok(())
        },
    )?;
    let mut sum = 0;
    work.add(histogram.len())?;
    for h in &mut histogram {
        let count = *h;
        *h = sum;
        sum += count;
    }
    work.scan(ws.keys[..soft_count].iter().enumerate(), |(i, &key)| {
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
        dest[offset..offset + end - start].copy_from_slice(&indices[start..end]);
        offset += end - start;
    }
    Ok(())
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
