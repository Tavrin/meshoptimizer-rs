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
        validate(indices, positions, threshold, &mut work)?;
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
            positions,
            threshold,
            workspace,
            &mut work,
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
        validate(indices, positions, threshold, &mut work)?;
        if destination.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        prepare(indices.len(), positions.len(), 0, workspace)?;
        kernel(
            destination,
            indices,
            positions,
            threshold,
            workspace,
            &mut work,
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
) -> Result<(), Error> {
    topology(indices, positions.len(), work)?;
    if !threshold.is_finite() {
        return Err(Error::InvalidParameter);
    }
    for i in 0..positions.len() {
        work.add(1)?;
        if !positions.at(i)?.iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
    }
    Ok(())
}

fn prepare(n: usize, v: usize, out: usize, ws: &mut Workspace) -> Result<(), Error> {
    if n == 0 {
        return ws.prepare([0; 4], out);
    }
    let f = n / 3;
    let ints = f
        .checked_mul(3)
        .and_then(|x| x.checked_add(v))
        .and_then(|x| x.checked_add(1))
        .ok_or(Error::SizeOverflow)?;
    ws.prepare([ints, f, 0, f], out)
}

fn update(triangle: &[u32], stamps: &mut [u32], time: &mut u32) -> u32 {
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

fn kernel(
    dest: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    ws: &mut Workspace,
    work: &mut Work,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let faces = indices.len() / 3;
    let v = positions.len();
    let (stamps, rest) = ws.integers.split_at_mut(v);
    let (hard, rest) = rest.split_at_mut(faces);
    let (soft, order) = rest.split_at_mut(faces + 1);
    work.add(v)?;
    stamps.fill(0);
    let mut time = 17;
    let mut hard_count = 0;
    for (i, tri) in indices.as_chunks::<3>().0.iter().enumerate() {
        work.add(1)?;
        let misses = update(tri, stamps, &mut time);
        if i == 0 || misses == 3 {
            hard[hard_count] = i as u32;
            hard_count += 1;
        }
    }
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
        for i in start..end {
            work.add(1)?;
            misses += update(&indices[i * 3..i * 3 + 3], stamps, &mut time);
        }
        let target = finite(threshold * (misses as f32 / (end - start) as f32))?;
        soft[soft_count] = start as u32;
        soft_count += 1;
        time = time.wrapping_add(17);
        let mut running_misses = 0;
        let mut running_faces = 0;
        for i in start..end {
            work.add(1)?;
            running_misses += update(&indices[i * 3..i * 3 + 3], stamps, &mut time);
            running_faces += 1;
            if running_misses as f32 / running_faces as f32 <= target {
                soft[soft_count] = (i + 1) as u32;
                soft_count += 1;
                time = time.wrapping_add(17);
                running_misses = 0;
                running_faces = 0;
            }
        }
        if soft[soft_count - 1] != start as u32 {
            soft_count -= 1;
        }
    }
    let mut mesh_centroid = [0f32; 3];
    for i in 0..v {
        work.add(1)?;
        let p = positions.at(i)?;
        for j in 0..3 {
            mesh_centroid[j] = finite(mesh_centroid[j] + p[j])?;
        }
    }
    for x in &mut mesh_centroid {
        *x /= v as f32;
    }
    let data = &mut ws.floats;
    for cluster in 0..soft_count {
        work.add(1)?;
        let start = soft[cluster] as usize * 3;
        let end = if cluster + 1 < soft_count {
            soft[cluster + 1] as usize * 3
        } else {
            indices.len()
        };
        let mut area_sum = 0f32;
        let mut centroid = [0f32; 3];
        let mut normal = [0f32; 3];
        for tri in indices[start..end].as_chunks::<3>().0.iter() {
            work.add(1)?;
            let p0 = positions.at(tri[0] as usize)?;
            let p1 = positions.at(tri[1] as usize)?;
            let p2 = positions.at(tri[2] as usize)?;
            let mut p10 = [0f32; 3];
            let mut p20 = [0f32; 3];
            for j in 0..3 {
                p10[j] = finite(p1[j] - p0[j])?;
                p20[j] = finite(p2[j] - p0[j])?;
            }
            let n = [
                finite(p10[1] * p20[2] - p10[2] * p20[1])?,
                finite(p10[2] * p20[0] - p10[0] * p20[2])?,
                finite(p10[0] * p20[1] - p10[1] * p20[0])?,
            ];
            let area = crate::math::sqrt(finite(n[0] * n[0] + n[1] * n[1] + n[2] * n[2])?);
            for j in 0..3 {
                centroid[j] = finite(centroid[j] + (p0[j] + p1[j] + p2[j]) * (area / 3.))?;
                normal[j] = finite(normal[j] + n[j])?;
            }
            area_sum = finite(area_sum + area)?;
        }
        let inv_area = if area_sum == 0. {
            0.
        } else {
            finite(1. / area_sum)?
        };
        for x in &mut centroid {
            *x = finite(*x * inv_area)?;
        }
        let length = crate::math::sqrt(finite(
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
        data[cluster] = finite(cv[0] * normal[0] + cv[1] * normal[1] + cv[2] * normal[2])?;
    }
    let mut max = 1e-3f32;
    for &d in &data[..soft_count] {
        work.add(1)?;
        if max < d.abs() {
            max = d.abs();
        }
    }
    let mut histogram = [0u32; 2048];
    for i in 0..soft_count {
        work.add(1)?;
        let key = 0.5f32 - 0.5f32 * (data[i] / max);
        let clamped = key.clamp(0., 1.);
        ws.keys[i] = ((clamped * 2047. + 0.5) as u16) & 2047;
        histogram[ws.keys[i] as usize] += 1;
    }
    let mut sum = 0;
    for h in &mut histogram {
        work.add(1)?;
        let count = *h;
        *h = sum;
        sum += count;
    }
    for i in 0..soft_count {
        work.add(1)?;
        let h = &mut histogram[ws.keys[i] as usize];
        order[*h as usize] = i as u32;
        *h += 1;
    }
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
