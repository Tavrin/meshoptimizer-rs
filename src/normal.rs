//! Experimental normal generation from pinned `tangentspace.cpp`.
#![allow(clippy::approx_constant)] // Upstream uses these exact rounded literals.
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

#[derive(Clone, Copy, Default)]
struct Face {
    normal: [f32; 3],
    id: bool,
}

fn reserve<T: Default + Clone>(count: usize) -> Result<Vec<T>, Error> {
    checked_bytes(count, core::mem::size_of::<T>())?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    result.resize(count, T::default());
    Ok(result)
}

fn vertex_hash(position: [f32; 3]) -> u32 {
    let [mut x, mut y, mut z] = position.map(|v| {
        if v.to_bits() == 0x80000000 {
            0
        } else {
            v.to_bits()
        }
    });
    x ^= x >> 17;
    y ^= y >> 17;
    z ^= z >> 17;
    x.wrapping_mul(73856093) ^ y.wrapping_mul(19349663) ^ z.wrapping_mul(83492791)
}

fn find(parents: &mut [u32], mut index: usize) -> usize {
    while index != parents[index] as usize {
        let parent = parents[index] as usize;
        parents[index] = parents[parent];
        index = parent;
    }
    index
}

fn optacos(value: f32) -> f32 {
    let mut ax = value.abs();
    ax = ax.min(1.0);
    let mut r = 1.570337 + ax * (-0.2053972 + ax * 0.05147786);
    r *= libm::sqrtf(1.0 - ax);
    if value < 0.0 {
        3.1415926 - r
    } else {
        r
    }
}

fn index_at(indices: Option<&[u32]>, corner: usize) -> usize {
    indices.map_or(corner, |v| v[corner] as usize)
}

fn position(positions: Positions<'_>, index: usize) -> [f32; 3] {
    positions.get(index).expect("validated index")
}

fn build_remap(positions: Positions<'_>, work: &mut Work) -> Result<Vec<u32>, Error> {
    let count = positions.len();
    let buckets = count
        .checked_add(count / 4)
        .and_then(usize::checked_next_power_of_two)
        .ok_or(Error::SizeOverflow)?;
    let mut table = reserve::<u32>(buckets)?;
    table.fill(u32::MAX);
    let mut remap = reserve::<u32>(count)?;
    for (i, remap_entry) in remap.iter_mut().enumerate() {
        let p = position(positions, i);
        let mut bucket = vertex_hash(p) as usize & (buckets - 1);
        for probe in 0..buckets {
            work.add(1)?;
            let entry = table[bucket];
            if entry == u32::MAX {
                table[bucket] = i as u32;
                *remap_entry = i as u32;
                break;
            }
            if p == position(positions, entry as usize) {
                *remap_entry = entry;
                break;
            }
            bucket = (bucket + probe + 1) & (buckets - 1);
        }
    }
    Ok(remap)
}

fn adjacency(
    indices: Option<&[u32]>,
    count: usize,
    remap: &[u32],
    vertex_count: usize,
    work: &mut Work,
) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut offsets = reserve::<u32>(vertex_count + 1)?;
    let mut data = reserve::<u32>(count)?;
    for i in 0..count {
        work.add(1)?;
        offsets[remap[index_at(indices, i)] as usize + 1] += 1;
    }
    let mut offset = 0u32;
    for value in offsets.iter_mut().skip(1) {
        let n = *value;
        *value = offset;
        offset += n;
    }
    for face in 0..count / 3 {
        for corner in 0..3 {
            work.add(1)?;
            let vertex = remap[index_at(indices, face * 3 + corner)] as usize;
            let slot = offsets[vertex + 1] as usize;
            data[slot] = (face as u32 * 4) | corner as u32;
            offsets[vertex + 1] += 1;
        }
    }
    Ok((offsets, data))
}

fn face_normals(
    indices: Option<&[u32]>,
    count: usize,
    positions: Positions<'_>,
    work: &mut Work,
) -> Result<Vec<Face>, Error> {
    let mut faces = reserve::<Face>(count / 3)?;
    for (i, face) in faces.iter_mut().enumerate() {
        work.add(1)?;
        let a = position(positions, index_at(indices, i * 3));
        let b = position(positions, index_at(indices, i * 3 + 1));
        let c = position(positions, index_at(indices, i * 3 + 2));
        let (x1, y1, z1) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
        let (x2, y2, z2) = (c[0] - a[0], c[1] - a[1], c[2] - a[2]);
        let (nx, ny, nz) = (y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2);
        let length = libm::sqrtf(nx * nx + ny * ny + nz * nz);
        let scale = if length != 0.0 { 1.0 / length } else { 0.0 };
        *face = Face {
            normal: [nx * scale, ny * scale, nz * scale],
            id: length != 0.0,
        };
    }
    Ok(faces)
}

fn merge_groups(
    groups: &mut [u32],
    data: &[u32],
    indices: Option<&[u32]>,
    remap: &[u32],
    faces: &[Face],
    cutoff: f32,
    work: &mut Work,
) -> Result<(), Error> {
    const NEXT: [usize; 4] = [1, 2, 0, 1];
    for (i, &corner_i) in data.iter().enumerate() {
        let fi = (corner_i >> 2) as usize;
        let ci = (corner_i & 3) as usize;
        let ib = remap[index_at(indices, fi * 3 + NEXT[ci])];
        let ic = remap[index_at(indices, fi * 3 + NEXT[ci + 1])];
        for &corner_j in &data[i + 1..] {
            work.add(1)?;
            let fj = (corner_j >> 2) as usize;
            let cj = (corner_j & 3) as usize;
            let jb = remap[index_at(indices, fj * 3 + NEXT[cj])];
            let jc = remap[index_at(indices, fj * 3 + NEXT[cj + 1])];
            let ni = faces[fi];
            let nj = faces[fj];
            let dp = ni.normal[0] * nj.normal[0]
                + ni.normal[1] * nj.normal[1]
                + ni.normal[2] * nj.normal[2];
            if (jb == ic || jc == ib) && dp > cutoff && ni.id == nj.id {
                let gi = find(groups, fi * 3 + ci);
                let gj = find(groups, fj * 3 + cj);
                if gi != gj {
                    groups[gj] = gi as u32;
                }
            }
        }
    }
    Ok(())
}

fn accumulate(
    result: &mut [[f32; 3]],
    groups: &[u32],
    indices: Option<&[u32]>,
    positions: Positions<'_>,
    faces: &[Face],
    work: &mut Work,
) -> Result<(), Error> {
    const NEXT: [usize; 4] = [1, 2, 0, 1];
    for (i, face) in faces.iter().enumerate() {
        for corner in 0..3 {
            work.add(1)?;
            let a = position(positions, index_at(indices, i * 3 + corner));
            let b = position(positions, index_at(indices, i * 3 + NEXT[corner]));
            let c = position(positions, index_at(indices, i * 3 + NEXT[corner + 1]));
            let (x1, y1, z1) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
            let (x2, y2, z2) = (c[0] - a[0], c[1] - a[1], c[2] - a[2]);
            let l1 = x1 * x1 + y1 * y1 + z1 * z1;
            let l2 = x2 * x2 + y2 * y2 + z2 * z2;
            let length = libm::sqrtf(l1 * l2);
            let cosine =
                (x1 * x2 + y1 * y2 + z1 * z2) * if length == 0.0 { 0.0 } else { 1.0 / length };
            let weight = optacos(cosine) * length;
            let target = &mut result[groups[i * 3 + corner] as usize];
            target[0] += face.normal[0] * weight;
            target[1] += face.normal[1] * weight;
            target[2] += face.normal[2] * weight;
        }
    }
    Ok(())
}

fn normalize(value: &mut [f32; 3]) {
    let length = libm::sqrtf(value[0] * value[0] + value[1] * value[1] + value[2] * value[2]);
    let scale = if length == 0.0 { 0.0 } else { 1.0 / length };
    for component in value {
        *component *= scale;
    }
}

fn smooth(result: &mut [[f32; 3]], scratch: &mut [[f32; 4]], groups: &[u32], alpha: f32) {
    const NEXT: [usize; 3] = [1, 2, 0];
    scratch.fill([0.0; 4]);
    for face in 0..groups.len() / 3 {
        for corner in 0..3 {
            let a = groups[face * 3 + corner] as usize;
            let b = groups[face * 3 + NEXT[corner]] as usize;
            let na = result[a];
            let nb = result[b];
            let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
            let weight = if dot > 0.0 { dot * dot } else { 0.0 };
            let delta = [
                (nb[0] - na[0]) * weight,
                (nb[1] - na[1]) * weight,
                (nb[2] - na[2]) * weight,
            ];
            for j in 0..3 {
                scratch[a][j] += delta[j];
                scratch[b][j] -= delta[j];
            }
            scratch[a][3] += 1.0;
            scratch[b][3] += 1.0;
        }
    }
    for i in 0..groups.len() {
        if groups[i] as usize == i && scratch[i][3] > 0.0 {
            let scale = alpha / scratch[i][3];
            for j in 0..3 {
                result[i][j] += scratch[i][j] * scale;
            }
            normalize(&mut result[i]);
        }
    }
}

/// Experimental upstream `meshopt_generateNormals`, exposed only with `experimental`.
/// `None` indices denote an unindexed triangle list.
#[allow(clippy::too_many_arguments)]
pub fn generate_normals(
    indices: Option<&[u32]>,
    index_count: usize,
    positions: Positions<'_>,
    crease_angle: f32,
    smoothing: f32,
    workspace: &mut Workspace,
) -> Result<Vec<[f32; 3]>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !index_count.is_multiple_of(3) {
            return Err(Error::InvalidTopology);
        }
        if indices.is_none() && index_count != positions.len()
            || indices.is_some_and(|v| v.len() != index_count)
        {
            return Err(Error::InvalidLayout);
        }
        if !crease_angle.is_finite()
            || !(0.0..=3.1415927).contains(&crease_angle)
            || !smoothing.is_finite()
            || smoothing < 0.0
        {
            return Err(Error::InvalidParameter);
        }
        if positions.len() > u32::MAX as usize || index_count > u32::MAX as usize / 4 {
            return Err(Error::SizeOverflow);
        }
        for i in 0..positions.len() {
            work.add(1)?;
            if position(positions, i).iter().any(|v| !v.is_finite()) {
                return Err(Error::NumericalFailure);
            }
        }
        for &index in indices.unwrap_or(&[]) {
            work.add(1)?;
            if index as usize >= positions.len() {
                return Err(Error::IndexOutOfBounds);
            }
        }
        let count = positions.len();
        let faces_count = index_count / 3;
        let buckets = count
            .checked_add(count / 4)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(Error::SizeOverflow)?;
        let bytes = checked_bytes(buckets, 4)?
            .checked_add(checked_bytes(count, 8)?)
            .and_then(|v| v.checked_add(index_count * 4 * 2))
            .and_then(|v| v.checked_add(faces_count * 16))
            .and_then(|v| v.checked_add(index_count * 12))
            .and_then(|v| v.checked_add(if smoothing > 0.0 { index_count * 16 } else { 0 }))
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(bytes)?;
        let remap = build_remap(positions, &mut work)?;
        let (offsets, data) = adjacency(indices, index_count, &remap, count, &mut work)?;
        let faces = face_normals(indices, index_count, positions, &mut work)?;
        let cutoff = libm::cosf(crease_angle);
        let mut groups = reserve::<u32>(index_count)?;
        for (i, group) in groups.iter_mut().enumerate() {
            *group = i as u32;
        }
        for i in 0..count {
            let start = offsets[i] as usize;
            let end = offsets[i + 1] as usize;
            if end > start {
                merge_groups(
                    &mut groups,
                    &data[start..end],
                    indices,
                    &remap,
                    &faces,
                    cutoff,
                    &mut work,
                )?;
            }
        }
        for i in 0..index_count {
            groups[i] = find(&mut groups, i) as u32;
        }
        let mut output = reserve::<[f32; 3]>(index_count)?;
        accumulate(&mut output, &groups, indices, positions, &faces, &mut work)?;
        for i in 0..index_count {
            if groups[i] as usize == i {
                normalize(&mut output[i]);
            }
        }
        if smoothing > 0.0 {
            let mut scratch = reserve::<[f32; 4]>(index_count)?;
            let passes = (libm::ceilf(smoothing) as usize).min(10);
            for pass in 0..passes {
                let remaining = smoothing - pass as f32;
                let alpha = 0.5 * remaining.min(1.0);
                smooth(&mut output, &mut scratch, &groups, alpha);
            }
        }
        for i in 0..index_count {
            if groups[i] as usize != i {
                output[i] = output[groups[i] as usize];
            }
        }
        Ok(output)
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer form of the experimental normal generator.
/// The destination is unchanged if validation or allocation fails.
pub fn generate_normals_into(
    destination: &mut [[f32; 3]],
    indices: Option<&[u32]>,
    index_count: usize,
    positions: Positions<'_>,
    crease_angle: f32,
    smoothing: f32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    if destination.len() < index_count {
        return Err(Error::BufferTooSmall);
    }
    let result = generate_normals(
        indices,
        index_count,
        positions,
        crease_angle,
        smoothing,
        workspace,
    )?;
    destination[..index_count].copy_from_slice(&result);
    Ok(())
}
