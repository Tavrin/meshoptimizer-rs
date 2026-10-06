//! Tangent generation following pinned `tangentspace.cpp`.
#![allow(clippy::approx_constant)] // Exact reference polynomial coefficients.
use crate::input::PositionReader;
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

/// Preserve MikkTSpace weighting instead of upstream's area-weighted default.
pub const TANGENT_COMPATIBLE: u32 = 1;
/// Emit zero tangents for isolated degenerate faces.
pub const TANGENT_ZERO_FALLBACK: u32 = 2;

fn reserve<T: Default + Clone>(count: usize) -> Result<Vec<T>, Error> {
    reserve_initialized(count, T::default())
}

fn reserve_initialized<T: Clone>(count: usize, value: T) -> Result<Vec<T>, Error> {
    checked_bytes(count, core::mem::size_of::<T>())?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    result.resize(count, value);
    Ok(result)
}
fn vertex(indices: Option<&[u32]>, corner: usize) -> usize {
    indices.map_or(corner, |v| v[corner] as usize)
}
#[inline(always)]
fn position(positions: impl PositionReader, index: usize) -> [f32; 3] {
    positions.read(index)
}
fn find(parents: &mut [u32], mut index: usize) -> usize {
    while index != parents[index] as usize {
        let parent = parents[index] as usize;
        parents[index] = parents[parent];
        index = parent;
    }
    index
}
#[inline(always)]
fn canonical_bits(value: f32) -> u32 {
    let bits = value.to_bits();
    if bits == 0x80000000 {
        0
    } else {
        bits
    }
}
fn hash(p: [f32; 3], n: [f32; 3], uv: [f32; 2]) -> u32 {
    let (mut x, mut y, mut z) = (
        canonical_bits(p[0]),
        canonical_bits(p[1]),
        canonical_bits(p[2]),
    );
    x ^= x >> 17;
    y ^= y >> 17;
    z ^= z >> 17;
    x ^= canonical_bits(n[0]) >> 15;
    y ^= canonical_bits(n[1]) >> 15;
    z ^= canonical_bits(n[2]) >> 15;
    let mut w = (uv[0].to_bits() ^ uv[1].to_bits()) & 0x7fffffff;
    w ^= w >> 13;
    x.wrapping_mul(73856093)
        ^ y.wrapping_mul(19349663)
        ^ z.wrapping_mul(83492791)
        ^ w.wrapping_mul(50331653)
}
fn build_remap(
    positions: impl PositionReader,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    work: &mut Work,
) -> Result<Vec<u32>, Error> {
    let count = positions.len();
    let buckets = count
        .checked_add(count / 4)
        .and_then(usize::checked_next_power_of_two)
        .ok_or(Error::SizeOverflow)?;
    let mut table = reserve_initialized(buckets, u32::MAX)?;
    let mut remap = reserve::<u32>(count)?;
    for (range, chunk) in remap.chunks_mut(128).enumerate() {
        let covered = if let Some(bound) = buckets.checked_mul(chunk.len()) {
            work.covers(bound)?
        } else {
            false
        };
        if covered {
            remap_range::<false>(
                positions,
                normals,
                uvs,
                &mut table,
                chunk,
                buckets,
                range * 128,
                work,
            )?;
        } else {
            remap_range::<true>(
                positions,
                normals,
                uvs,
                &mut table,
                chunk,
                buckets,
                range * 128,
                work,
            )?;
        }
    }
    Ok(remap)
}
#[allow(clippy::too_many_arguments)]
fn remap_range<const CHARGE: bool>(
    positions: impl PositionReader,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    table: &mut [u32],
    remap: &mut [u32],
    buckets: usize,
    start: usize,
    work: &mut Work,
) -> Result<(), Error> {
    let mut visits = 0;
    for (offset, entry) in remap.iter_mut().enumerate() {
        let i = start + offset;
        let p = position(positions, i);
        let n = normals[i];
        let uv = uvs[i];
        let mut bucket = hash(p, n, uv) as usize & (buckets - 1);
        for probe in 0..buckets {
            if CHARGE {
                work.add(1)?;
            } else {
                visits += 1;
            }
            let old = table[bucket];
            if old == u32::MAX {
                table[bucket] = i as u32;
                *entry = i as u32;
                break;
            }
            let j = old as usize;
            if p == position(positions, j) && n == normals[j] && uv == uvs[j] {
                *entry = old;
                break;
            }
            bucket = (bucket + probe + 1) & (buckets - 1);
        }
    }
    if !CHARGE {
        work.add(visits)?;
    }
    Ok(())
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
    let bulk_counts = work.covers(count)?;
    if bulk_counts {
        work.add(count)?;
    }
    for i in 0..count {
        if !bulk_counts {
            work.add(1)?;
        }
        offsets[remap[vertex(indices, i)] as usize + 1] += 1;
    }
    let mut offset = 0u32;
    for value in offsets.iter_mut().skip(1) {
        let n = *value;
        *value = offset;
        offset += n;
    }
    let bulk_fill = work.covers(count)?;
    if bulk_fill {
        work.add(count)?;
    }
    for face in 0..count / 3 {
        for corner in 0..3 {
            if !bulk_fill {
                work.add(1)?;
            }
            let v = remap[vertex(indices, face * 3 + corner)] as usize;
            let slot = offsets[v + 1] as usize;
            data[slot] = (face as u32 * 4) | corner as u32;
            offsets[v + 1] += 1;
        }
    }
    Ok((offsets, data))
}
fn face_tangents(
    indices: Option<&[u32]>,
    count: usize,
    positions: impl PositionReader,
    uvs: &[[f32; 2]],
    work: &mut Work,
) -> Result<Vec<[f32; 4]>, Error> {
    let mut result = reserve::<[f32; 4]>(count / 3)?;
    let bulk_work = work.covers(result.len())?;
    if bulk_work {
        work.add(result.len())?;
    }
    for (i, tangent) in result.iter_mut().enumerate() {
        if !bulk_work {
            work.add(1)?;
        }
        let (a, b, c) = (
            vertex(indices, i * 3),
            vertex(indices, i * 3 + 1),
            vertex(indices, i * 3 + 2),
        );
        let (pa, pb, pc) = (
            position(positions, a),
            position(positions, b),
            position(positions, c),
        );
        let (ta, tb, tc) = (uvs[a], uvs[b], uvs[c]);
        let (dp1x, dp1y, dp1z) = (pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]);
        let (dp2x, dp2y, dp2z) = (pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]);
        let (dt1x, dt1y) = (tb[0] - ta[0], tb[1] - ta[1]);
        let (dt2x, dt2y) = (tc[0] - ta[0], tc[1] - ta[1]);
        let (rx, ry, rz) = (
            dt2y * dp1x - dt1y * dp2x,
            dt2y * dp1y - dt1y * dp2y,
            dt2y * dp1z - dt1y * dp2z,
        );
        let det = dt1x * dt2y - dt1y * dt2x;
        let mut sign = if det == 0.0 {
            0.0
        } else if det > 0.0 {
            1.0
        } else {
            -1.0
        };
        if pa == pb || pa == pc || pb == pc {
            sign = 0.0;
        }
        let length = square_root(rx * rx + ry * ry + rz * rz);
        let scale = if length != 0.0 { sign / length } else { 0.0 };
        *tangent = [rx * scale, ry * scale, rz * scale, sign];
    }
    Ok(result)
}
fn merge(
    groups: &mut [u32],
    facegroups: &mut [u32],
    signs: &mut [u8],
    data: &[u32],
    indices: Option<&[u32]>,
    remap: &[u32],
    work: &mut Work,
) -> Result<(), Error> {
    if let Some(twice) = data.len().checked_mul(data.len().saturating_sub(1)) {
        let pairs = twice / 2;
        if work.covers(pairs)? {
            work.add(pairs)?;
            if data.len() <= 32 {
                const NEXT: [usize; 4] = [1, 2, 0, 1];
                let mut points = [[0u32; 2]; 32];
                for (&corner, point) in data.iter().zip(&mut points) {
                    let (f, c) = ((corner >> 2) as usize, (corner & 3) as usize);
                    *point = [
                        remap[vertex(indices, f * 3 + NEXT[c])],
                        remap[vertex(indices, f * 3 + NEXT[c + 1])],
                    ];
                }
                merge_cached(groups, facegroups, signs, data, &points[..data.len()]);
                return Ok(());
            }
            return merge_loop::<false>(groups, facegroups, signs, data, indices, remap, work);
        }
    }
    merge_loop::<true>(groups, facegroups, signs, data, indices, remap, work)
}
fn merge_loop<const CHARGE: bool>(
    groups: &mut [u32],
    facegroups: &mut [u32],
    signs: &mut [u8],
    data: &[u32],
    indices: Option<&[u32]>,
    remap: &[u32],
    work: &mut Work,
) -> Result<(), Error> {
    const NEXT: [usize; 4] = [1, 2, 0, 1];
    for (i, &corner_i) in data.iter().enumerate() {
        let (fi, ci) = ((corner_i >> 2) as usize, (corner_i & 3) as usize);
        let ib = remap[vertex(indices, fi * 3 + NEXT[ci])];
        let ic = remap[vertex(indices, fi * 3 + NEXT[ci + 1])];
        for &corner_j in &data[i + 1..] {
            if CHARGE {
                work.add(1)?;
            }
            let (fj, cj) = ((corner_j >> 2) as usize, (corner_j & 3) as usize);
            let jb = remap[vertex(indices, fj * 3 + NEXT[cj])];
            let jc = remap[vertex(indices, fj * 3 + NEXT[cj + 1])];
            if (jb == ic || jc == ib) && (signs[fi] | signs[fj]) != 3 {
                if (signs[fi] & signs[fj]) == 0 {
                    let gi = find(facegroups, fi);
                    let gj = find(facegroups, fj);
                    if gi != gj {
                        if (signs[gi] | signs[gj]) == 3 {
                            continue;
                        }
                        facegroups[gj] = gi as u32;
                        signs[gi] |= signs[gj];
                    }
                }
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
fn merge_cached(
    groups: &mut [u32],
    facegroups: &mut [u32],
    signs: &mut [u8],
    data: &[u32],
    points: &[[u32; 2]],
) {
    for (i, (&corner_i, &[ib, ic])) in data.iter().zip(points).enumerate() {
        let (fi, ci) = ((corner_i >> 2) as usize, (corner_i & 3) as usize);
        for (&corner_j, &[jb, jc]) in data[i + 1..].iter().zip(&points[i + 1..]) {
            let (fj, cj) = ((corner_j >> 2) as usize, (corner_j & 3) as usize);
            if (jb == ic || jc == ib) && (signs[fi] | signs[fj]) != 3 {
                if (signs[fi] & signs[fj]) == 0 {
                    let gi = find(facegroups, fi);
                    let gj = find(facegroups, fj);
                    if gi != gj {
                        if (signs[gi] | signs[gj]) == 3 {
                            continue;
                        }
                        facegroups[gj] = gi as u32;
                        signs[gi] |= signs[gj];
                    }
                }
                let gi = find(groups, fi * 3 + ci);
                let gj = find(groups, fj * 3 + cj);
                if gi != gj {
                    groups[gj] = gi as u32;
                }
            }
        }
    }
}
#[inline]
fn square_root(value: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        value.sqrt()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::sqrtf(value)
    }
}

fn optacos(value: f32) -> f32 {
    let ax = value.abs().min(1.0);
    let mut r = 1.570337 + ax * (-0.2053972 + ax * 0.05147786);
    r *= square_root(1.0 - ax);
    if value < 0.0 {
        3.1415926 - r
    } else {
        r
    }
}
#[allow(clippy::too_many_arguments)]
fn accumulate(
    output: &mut [[f32; 4]],
    groups: &[u32],
    indices: Option<&[u32]>,
    positions: impl PositionReader,
    normals: &[[f32; 3]],
    faces: &[[f32; 4]],
    options: u32,
    work: &mut Work,
) -> Result<(), Error> {
    if let Some(bound) = faces.len().checked_mul(3) {
        if work.covers(bound)? {
            return accumulate_loop::<false>(
                output, groups, indices, positions, normals, faces, options, work,
            );
        }
    }
    accumulate_loop::<true>(
        output, groups, indices, positions, normals, faces, options, work,
    )
}

#[allow(clippy::too_many_arguments)]
fn accumulate_loop<const CHARGE: bool>(
    output: &mut [[f32; 4]],
    groups: &[u32],
    indices: Option<&[u32]>,
    positions: impl PositionReader,
    normals: &[[f32; 3]],
    faces: &[[f32; 4]],
    options: u32,
    work: &mut Work,
) -> Result<(), Error> {
    const NEXT: [usize; 4] = [1, 2, 0, 1];
    let mut visits = 0;
    for (i, &tangent) in faces.iter().enumerate() {
        if tangent[3] == 0.0 {
            continue;
        }
        let vertices = [
            vertex(indices, i * 3),
            vertex(indices, i * 3 + 1),
            vertex(indices, i * 3 + 2),
        ];
        let corners = [
            position(positions, vertices[0]),
            position(positions, vertices[1]),
            position(positions, vertices[2]),
        ];
        let bulk_corners = !CHARGE || work.covers(3)?;
        if CHARGE && bulk_corners {
            work.add(3)?;
        }
        visits += 3;
        for corner in 0..3 {
            if CHARGE && !bulk_corners {
                work.add(1)?;
            }
            let a = vertices[corner];
            let (pa, pb, pc) = (
                corners[corner],
                corners[NEXT[corner]],
                corners[NEXT[corner + 1]],
            );
            let n = normals[a];
            let dot = tangent[0] * n[0] + tangent[1] * n[1] + tangent[2] * n[2];
            let (sx, sy, sz) = (
                tangent[0] - n[0] * dot,
                tangent[1] - n[1] * dot,
                tangent[2] - n[2] * dot,
            );
            let sl = square_root(sx * sx + sy * sy + sz * sz);
            let (mut x1, mut y1, mut z1) = (pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]);
            let (mut x2, mut y2, mut z2) = (pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]);
            let d1 = x1 * n[0] + y1 * n[1] + z1 * n[2];
            let d2 = x2 * n[0] + y2 * n[1] + z2 * n[2];
            x1 -= n[0] * d1;
            y1 -= n[1] * d1;
            z1 -= n[2] * d1;
            x2 -= n[0] * d2;
            y2 -= n[1] * d2;
            z2 -= n[2] * d2;
            let l1 = x1 * x1 + y1 * y1 + z1 * z1;
            let l2 = x2 * x2 + y2 * y2 + z2 * z2;
            let length = square_root(l1 * l2);
            let cosine =
                (x1 * x2 + y1 * y2 + z1 * z2) * if length == 0.0 { 0.0 } else { 1.0 / length };
            let angle = optacos(cosine);
            let mut weight = angle * if sl == 0.0 { 0.0 } else { 1.0 / sl };
            if options & TANGENT_COMPATIBLE == 0 {
                weight *= length;
            }
            let target = &mut output[groups[i * 3 + corner] as usize];
            target[0] += sx * weight;
            target[1] += sy * weight;
            target[2] += sz * weight;
        }
    }
    if !CHARGE {
        work.add(visits)?;
    }
    Ok(())
}

/// Generate per-corner XYZ plus handedness, matching `meshopt_generateTangents`.
/// `None` indices denote an unindexed triangle list. Normals and UVs are packed.
#[allow(clippy::too_many_arguments)]
pub fn generate_tangents(
    indices: Option<&[u32]>,
    index_count: usize,
    positions: Positions<'_>,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    options: u32,
    workspace: &mut Workspace,
) -> Result<Vec<[f32; 4]>, Error> {
    generate_tangents_impl(
        indices,
        index_count,
        positions,
        normals,
        uvs,
        options,
        workspace,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn generate_tangents_impl(
    indices: Option<&[u32]>,
    index_count: usize,
    positions: Positions<'_>,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    options: u32,
    workspace: &mut Workspace,
    destination: Option<&mut [[f32; 4]]>,
) -> Result<Vec<[f32; 4]>, Error> {
    if let Some(packed) = positions.packed() {
        generate_tangents_core(
            indices,
            index_count,
            packed,
            normals,
            uvs,
            options,
            workspace,
            destination,
        )
    } else {
        generate_tangents_core(
            indices,
            index_count,
            positions,
            normals,
            uvs,
            options,
            workspace,
            destination,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_tangents_core(
    indices: Option<&[u32]>,
    index_count: usize,
    positions: impl PositionReader,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    options: u32,
    workspace: &mut Workspace,
    mut destination: Option<&mut [[f32; 4]]>,
) -> Result<Vec<[f32; 4]>, Error> {
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
        if normals.len() != positions.len() || uvs.len() != positions.len() {
            return Err(Error::InvalidLayout);
        }
        if options & !(TANGENT_COMPATIBLE | TANGENT_ZERO_FALLBACK) != 0 {
            return Err(Error::UnknownFlags);
        }
        if positions.len() > u32::MAX as usize || index_count > u32::MAX as usize / 4 {
            return Err(Error::SizeOverflow);
        }
        work.scan(0..positions.len(), |i| {
            if position(positions, i)
                .iter()
                .chain(normals[i].iter())
                .chain(uvs[i].iter())
                .any(|v| !v.is_finite())
            {
                return Err(Error::NumericalFailure);
            }
            Ok(())
        })?;
        work.indices(indices.unwrap_or(&[]), positions.len())?;
        let count = positions.len();
        let faces_count = index_count / 3;
        let buckets = count
            .checked_add(count / 4)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(Error::SizeOverflow)?;
        let budget_direct =
            destination.is_some() && work.covers_corner_generation(count, buckets, index_count);
        // The remap hash table is dropped before adjacency is allocated.
        // Report the maximum live requested storage, not the sum of phases.
        let remap_peak = checked_bytes(buckets, 4)?
            .checked_add(checked_bytes(count, 4)?)
            .ok_or(Error::SizeOverflow)?;
        let scratch = checked_bytes(count, 8)?
            .checked_add(4) // offsets has vertex_count + 1 entries
            .and_then(|v| v.checked_add(index_count * 8))
            .and_then(|v| v.checked_add(faces_count * 21))
            .ok_or(Error::SizeOverflow)?;
        let staged_peak = scratch
            .checked_add(checked_bytes(index_count, 16)?)
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(remap_peak.max(if budget_direct {
            scratch
        } else {
            staged_peak
        }))?;
        let remap = build_remap(positions, normals, uvs, &mut work)?;
        let (offsets, data) = adjacency(indices, index_count, &remap, count, &mut work)?;
        let faces = face_tangents(indices, index_count, positions, uvs, &mut work)?;
        let mut groups = reserve::<u32>(index_count)?;
        for (i, group) in groups.iter_mut().enumerate() {
            *group = i as u32;
        }
        let mut facegroups = reserve::<u32>(faces_count)?;
        let mut signs = reserve::<u8>(faces_count)?;
        for i in 0..faces_count {
            facegroups[i] = i as u32;
            signs[i] = u8::from(faces[i][3] > 0.0) | (u8::from(faces[i][3] < 0.0) << 1);
        }
        for i in 0..count {
            let start = offsets[i] as usize;
            let end = offsets[i + 1] as usize;
            if end > start {
                merge(
                    &mut groups,
                    &mut facegroups,
                    &mut signs,
                    &data[start..end],
                    indices,
                    &remap,
                    &mut work,
                )?;
            }
        }
        for i in 0..index_count {
            groups[i] = find(&mut groups, i) as u32;
        }
        // All remaining visits are accumulation corners; no later validation
        // or allocation may fail once caller writes begin.
        let direct = destination.is_some() && work.covers(index_count)?;
        if direct && !budget_direct {
            workspace.account_codec(remap_peak.max(scratch))?;
        }
        let mut owned = if direct {
            Vec::new()
        } else {
            reserve::<[f32; 4]>(index_count)?
        };
        let output = if direct {
            let output = &mut destination.as_deref_mut().expect("caller storage")[..index_count];
            output.fill([0.0; 4]);
            output
        } else {
            owned.as_mut_slice()
        };
        accumulate(
            output, &groups, indices, positions, normals, &faces, options, &mut work,
        )?;
        for i in 0..faces_count {
            let fg = find(&mut facegroups, i);
            let sign = if signs[fg] & 1 != 0 { 1.0 } else { -1.0 };
            for corner in 0..3 {
                output[i * 3 + corner][3] = sign;
            }
        }
        for i in 0..index_count {
            if groups[i] as usize == i {
                let result = &mut output[i];
                let length = square_root(
                    result[0] * result[0] + result[1] * result[1] + result[2] * result[2],
                );
                let scale = if length == 0.0 { 0.0 } else { 1.0 / length };
                result[0] *= scale;
                result[1] *= scale;
                result[2] *= scale;
                if length == 0.0 && options & TANGENT_ZERO_FALLBACK == 0 {
                    result[0] = 1.0;
                }
            }
        }
        for i in 0..index_count {
            if groups[i] as usize != i {
                let root = output[groups[i] as usize];
                output[i][..3].copy_from_slice(&root[..3]);
            }
        }
        if !direct {
            if let Some(destination) = destination {
                destination[..index_count].copy_from_slice(&owned);
            }
        }
        Ok(owned)
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer form of `meshopt_generateTangents`; writes one vector per corner.
/// The destination is unchanged if validation or allocation fails.
#[allow(clippy::too_many_arguments)]
pub fn generate_tangents_into(
    destination: &mut [[f32; 4]],
    indices: Option<&[u32]>,
    index_count: usize,
    positions: Positions<'_>,
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    options: u32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    if destination.len() < index_count {
        return Err(Error::BufferTooSmall);
    }
    generate_tangents_impl(
        indices,
        index_count,
        positions,
        normals,
        uvs,
        options,
        workspace,
        Some(destination),
    )
    .map(|_| ())
}
