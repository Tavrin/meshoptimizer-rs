//! Experimental voxel remesher, using the pinned 1.3 case table.
#![allow(clippy::too_many_arguments)] // Translation keeps the upstream argument sets visible.
use crate::workspace::{checked_bytes, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;
#[path = "remesh_table.rs"]
mod table;

/// Produce a two-sided surface shell.
pub const REMESH_SHELL: u32 = 1;
/// Solve the output positions using accumulated quadrics.
pub const REMESH_SOLVE: u32 = 2;

#[derive(Clone, Copy, Default)]
struct Voxel {
    coord: u32,
    octants: u8,
    p: [f32; 3],
    w: f32,
    a00: f32,
    a11: f32,
    a22: f32,
    a10: f32,
    a20: f32,
    a21: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    c: f32,
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
fn pos(positions: Positions<'_>, index: usize) -> [f32; 3] {
    positions.get(index).expect("validated")
}
fn measure(
    positions: Positions<'_>,
    resolution: usize,
    work: &mut Work,
) -> Result<(f32, [f32; 3]), Error> {
    let mut minv = [f32::MAX; 3];
    let mut maxv = [-f32::MAX; 3];
    for i in 0..positions.len() {
        work.add(1)?;
        let p = pos(positions, i);
        for j in 0..3 {
            minv[j] = if minv[j] > p[j] { p[j] } else { minv[j] };
            maxv[j] = if maxv[j] < p[j] { p[j] } else { maxv[j] };
        }
    }
    let mut extent = 0.0f32;
    for j in 0..3 {
        let d = maxv[j] - minv[j];
        extent = if d < extent { extent } else { d };
    }
    extent *= (resolution as f32 + 0.01) / resolution as f32;
    if !extent.is_finite() {
        return Err(Error::NumericalFailure);
    }
    let scale = if extent == 0.0 {
        0.0
    } else {
        (resolution - 2) as f32 / extent
    };
    let offset = [0, 1, 2].map(|j| minv[j] - (extent - (maxv[j] - minv[j])) * 0.5);
    Ok((scale, offset))
}
fn accumulate_voxel(
    voxel: &mut Voxel,
    point: [f32; 3],
    normal: [f32; 3],
    weight: f32,
    solve: bool,
) {
    for (target, &source) in voxel.p.iter_mut().zip(point.iter()) {
        *target += source * weight;
    }
    voxel.w += weight;
    if !solve {
        return;
    }
    let [a, b, c] = normal;
    let d = -(a * point[0] + b * point[1] + c * point[2]);
    let (aw, bw, cw, dw) = (a * weight, b * weight, c * weight, d * weight);
    voxel.a00 += a * aw;
    voxel.a11 += b * bw;
    voxel.a22 += c * cw;
    voxel.a10 += a * bw;
    voxel.a20 += a * cw;
    voxel.a21 += b * cw;
    voxel.b0 += a * dw;
    voxel.b1 += b * dw;
    voxel.b2 += c * dw;
    voxel.c += d * dw;
}
#[allow(clippy::too_many_arguments)]
fn voxelize(
    grid: &mut [u8],
    rowmap: Option<&[u32]>,
    mut voxels: Option<&mut [Voxel]>,
    indices: &[u32],
    positions: Positions<'_>,
    resolution: usize,
    scale: f32,
    offset: [f32; 3],
    options: u32,
    work: &mut Work,
) -> Result<(), Error> {
    for triangle in indices.as_chunks::<3>().0 {
        work.add(1)?;
        let (a, b, c) = (
            pos(positions, triangle[0] as usize),
            pos(positions, triangle[1] as usize),
            pos(positions, triangle[2] as usize),
        );
        let (ex, ey, ez) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
        let (fx, fy, fz) = (c[0] - a[0], c[1] - a[1], c[2] - a[2]);
        let (gx, gy, gz) = (c[0] - b[0], c[1] - b[1], c[2] - b[2]);
        let el2 = ex * ex + ey * ey + ez * ez;
        let fl2 = fx * fx + fy * fy + fz * fz;
        let gl2 = gx * gx + gy * gy + gz * gz;
        let mut maximum = if el2 > fl2 { el2 } else { fl2 };
        maximum = if maximum > gl2 { maximum } else { gl2 };
        maximum = libm::sqrtf(maximum);
        let samples = ((maximum * scale * 2.0) as i32).clamp(1, (resolution * 2) as i32);
        let (mut nx, mut ny, mut nz) = (ey * fz - ez * fy, ez * fx - ex * fz, ex * fy - ey * fx);
        let area = libm::sqrtf(nx * nx + ny * ny + nz * nz);
        if area == 0.0 {
            continue;
        }
        let ns = 1.0 / area;
        nx *= ns;
        ny *= ns;
        nz *= ns;
        let (sx, sy, sz) = (a[0] - offset[0], a[1] - offset[1], a[2] - offset[2]);
        let weight = area / ((samples + 1) * (samples + 2)) as f32;
        let sr = if samples > 1 {
            1.0 / samples as f32
        } else {
            1.0
        };
        for u in 0..=samples {
            for v in 0..=samples - u {
                work.add(1)?;
                let (su, sv) = (u as f32 * sr, v as f32 * sr);
                let point = [
                    sx + su * ex + sv * fx,
                    sy + su * ey + sv * fy,
                    sz + su * ez + sv * fz,
                ];
                let [hx, hy, hz] = point.map(|p| (p * (scale * 2.0)) as i32);
                let cutoff = (resolution - 3) as i32;
                let [x, y, z] = [hx >> 1, hy >> 1, hz >> 1].map(|p| {
                    if (p as u32) < cutoff as u32 {
                        p as usize
                    } else {
                        cutoff as usize
                    }
                });
                let row = y + 1 + resolution * (z + 1);
                let idx = x + 1 + resolution * row;
                if let Some(ref mut voxels) = voxels {
                    let rows = rowmap.expect("second pass rowmap");
                    let voxel = &mut voxels[rows[row] as usize + grid[idx] as usize - 1];
                    voxel.coord = ((x as u32) << 20) | ((y as u32) << 10) | z as u32;
                    voxel.octants |= 1 << ((hx & 1) | ((hy & 1) << 1) | ((hz & 1) << 2));
                    accumulate_voxel(
                        voxel,
                        point,
                        [nx, ny, nz],
                        weight,
                        options & REMESH_SOLVE != 0,
                    );
                } else {
                    grid[idx] = 1;
                }
            }
        }
    }
    Ok(())
}
fn rowpack(
    grid: &mut [u8],
    resolution: usize,
    work: &mut Work,
) -> Result<(Vec<u32>, usize), Error> {
    let mut rows = reserve::<u32>(resolution * resolution)?;
    let mut result = 0usize;
    for (i, row) in grid.chunks_exact_mut(resolution).enumerate() {
        work.add(resolution)?;
        if row.iter().all(|&v| v == 0) {
            rows[i] = u32::MAX;
            continue;
        }
        let mut count = 0u8;
        for value in row {
            if *value != 0 {
                count += 1;
                *value = count;
            }
        }
        rows[i] = result as u32;
        result += count as usize;
    }
    Ok((rows, result))
}
fn queue(row: usize, worklist: &mut [u32], queued: &mut [u8], pending: &mut usize) {
    if queued[row] == 0 {
        queued[row] = 1;
        worklist[*pending] = row as u32;
        *pending += 1;
    }
}
fn solidify(
    grid: &mut [u8],
    rows: &[u32],
    resolution: usize,
    work: &mut Work,
) -> Result<(), Error> {
    let row_count = resolution * resolution;
    let mut worklist = reserve::<u32>(row_count)?;
    let mut queued = reserve::<u8>(row_count)?;
    for z in 1..resolution - 1 {
        for y in 1..resolution - 1 {
            let row = y + resolution * z;
            if rows[row] != u32::MAX {
                for x in 1..resolution - 1 {
                    let cell = &mut grid[x + resolution * row];
                    if *cell == 0 {
                        *cell = 0xff;
                    }
                }
            }
        }
    }
    let mut pending = 0;
    for row in 0..row_count {
        queue(row, &mut worklist, &mut queued, &mut pending);
    }
    while pending > 0 {
        work.add(1)?;
        pending -= 1;
        let row = worklist[pending] as usize;
        queued[row] = 0;
        let base = resolution * row;
        if rows[row] != u32::MAX {
            for x in 1..resolution - 1 {
                if grid[base + x] == 0xff && grid[base + x - 1] == 0 {
                    grid[base + x] = 0;
                }
            }
            for x in (1..resolution - 1).rev() {
                if grid[base + x] == 0xff && grid[base + x + 1] == 0 {
                    grid[base + x] = 0;
                }
            }
        }
        let (y, z) = (row % resolution, row / resolution);
        for k in 0..4 {
            let yn = y as i32
                + if k == 0 {
                    -1
                } else if k == 1 {
                    1
                } else {
                    0
                };
            let zn = z as i32
                + if k == 2 {
                    -1
                } else if k == 3 {
                    1
                } else {
                    0
                };
            if yn < 1 || yn >= resolution as i32 - 1 || zn < 1 || zn >= resolution as i32 - 1 {
                continue;
            }
            let neighbor = yn as usize + resolution * zn as usize;
            if rows[neighbor] == u32::MAX {
                continue;
            }
            let nb = resolution * neighbor;
            let mut changed = false;
            for x in 1..resolution - 1 {
                if grid[base + x] == 0 && grid[nb + x] == 0xff {
                    grid[nb + x] = 0;
                    changed = true;
                }
            }
            if changed {
                queue(neighbor, &mut worklist, &mut queued, &mut pending);
            }
        }
    }
    Ok(())
}

fn solve_point(voxel: &Voxel, lambda: f32) -> ([f32; 3], bool) {
    let rw = lambda * voxel.w;
    let (a00, a11, a22) = (rw + voxel.a00, rw + voxel.a11, rw + voxel.a22);
    let (a10, a20, a21) = (voxel.a10, voxel.a20, voxel.a21);
    let (x0, x1, x2) = (
        lambda * voxel.p[0] - voxel.b0,
        lambda * voxel.p[1] - voxel.b1,
        lambda * voxel.p[2] - voxel.b2,
    );
    let eps = 1e-6 * voxel.w;
    let d0 = a00;
    let l10 = a10 / d0;
    let l20 = a20 / d0;
    let d1 = a11 - a10 * l10;
    let dl21 = a21 - a20 * l10;
    let l21 = dl21 / d1;
    let d2 = a22 - a20 * l20 - dl21 * l21;
    let y0 = x0;
    let y1 = x1 - l10 * y0;
    let y2 = x2 - l20 * y0 - l21 * y1;
    let z0 = y0 / d0;
    let z1 = y1 / d1;
    let z2 = y2 / d2;
    let rz = z2;
    let ry = z1 - l21 * rz;
    let rx = z0 - l10 * ry - l20 * rz;
    (
        [rx, ry, rz],
        d0.abs() > eps && d1.abs() > eps && d2.abs() > eps,
    )
}
fn solve(voxels: &mut [Voxel], scale: f32, options: u32, work: &mut Work) -> Result<(), Error> {
    let cutoff = 3.0 / (scale * scale);
    let rscale = 1.0 / scale;
    for voxel in voxels {
        work.add(1)?;
        let mut point = voxel.p.map(|p| p / voxel.w);
        let corner = [
            ((voxel.coord >> 20) & 1023) as f32 * rscale,
            ((voxel.coord >> 10) & 1023) as f32 * rscale,
            (voxel.coord & 1023) as f32 * rscale,
        ];
        if options & REMESH_SOLVE != 0 {
            let (candidate, success) = solve_point(voxel, 3e-2);
            if success {
                let delta = [
                    (candidate[0] - point[0]),
                    (candidate[1] - point[1]),
                    (candidate[2] - point[2]),
                ];
                let distance = delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2];
                if distance < cutoff {
                    for j in 0..3 {
                        point[j] = candidate[j].max(corner[j]).min(corner[j] + rscale);
                    }
                }
            }
        }
        voxel.p = point;
    }
    Ok(())
}
fn corner_voxel<'a>(
    x: usize,
    y: usize,
    z: usize,
    corner: usize,
    grid: &[u8],
    rows: &[u32],
    voxels: &'a [Voxel],
    resolution: usize,
) -> Option<&'a Voxel> {
    let row = y + ((corner >> 1) & 1) + resolution * (z + ((corner >> 2) & 1));
    let idx = x + (corner & 1) + resolution * row;
    let value = grid[idx];
    if value == 0 || value == 0xff {
        return None;
    }
    voxels.get(rows[row] as usize + value as usize - 1)
}
fn octant_decider(
    x: usize,
    y: usize,
    z: usize,
    cube: usize,
    grid: &[u8],
    rows: &[u32],
    voxels: &[Voxel],
    resolution: usize,
) -> bool {
    for corner in 0..8 {
        if cube & (1 << corner) == 0 {
            continue;
        }
        if let Some(voxel) = corner_voxel(x, y, z, corner, grid, rows, voxels, resolution) {
            if voxel.octants & (1 << (7 - corner)) != 0 {
                return false;
            }
        }
    }
    true
}
fn voxel_error(voxel: &Voxel, point: [f32; 3]) -> f32 {
    let [x, y, z] = point;
    let rx = (voxel.b0 + voxel.a10 * y) * 2.0 + voxel.a00 * x;
    let ry = (voxel.b1 + voxel.a21 * z) * 2.0 + voxel.a11 * y;
    let rz = (voxel.b2 + voxel.a20 * x) * 2.0 + voxel.a22 * z;
    (voxel.c + rx * x + ry * y + rz * z).abs()
}
fn quadric_decider(
    x: usize,
    y: usize,
    z: usize,
    cube: usize,
    grid: &[u8],
    rows: &[u32],
    voxels: &[Voxel],
    resolution: usize,
    rscale: f32,
) -> bool {
    let primary = table::TRIANGLE_TABLE[cube][0];
    let quad = ((primary[0] as usize) << 4) | ((primary[1] as usize) & 0xf);
    let corners = [0, 1, 2, 3].map(|i| {
        corner_voxel(
            x,
            y,
            z,
            (quad >> (12 - i * 4)) & 0xf,
            grid,
            rows,
            voxels,
            resolution,
        )
        .expect("occupied quad corner")
    });
    let [a, b, c, d] = corners;
    let mut sum = Voxel::default();
    macro_rules! sum {
        ($field:ident) => {
            sum.$field = (a.$field + d.$field) + (b.$field + c.$field);
        };
    }
    sum!(a00);
    sum!(a11);
    sum!(a22);
    sum!(a10);
    sum!(a20);
    sum!(a21);
    sum!(b0);
    sum!(b1);
    sum!(b2);
    sum!(c);
    sum!(w);
    let error0 = voxel_error(
        &sum,
        [
            (b.p[0] + c.p[0]) * 0.5,
            (b.p[1] + c.p[1]) * 0.5,
            (b.p[2] + c.p[2]) * 0.5,
        ],
    );
    let error1 = voxel_error(
        &sum,
        [
            (a.p[0] + d.p[0]) * 0.5,
            (a.p[1] + d.p[1]) * 0.5,
            (a.p[2] + d.p[2]) * 0.5,
        ],
    );
    error0 > sum.w * (0.1 * 0.1 * rscale * rscale) && error1 < error0 * (0.85 * 0.85)
}
#[allow(clippy::too_many_arguments)]
fn polygonize(
    destination: Option<&mut [[f32; 3]]>,
    grid: &[u8],
    rows: &[u32],
    voxels: &[Voxel],
    resolution: usize,
    scale: f32,
    offset: [f32; 3],
    options: u32,
    work: &mut Work,
) -> Result<usize, Error> {
    let mut destination = destination;
    let mut count = 0usize;
    let slice = resolution * resolution;
    let rscale = 1.0 / scale;
    for z in 0..resolution - 1 {
        for y in 0..resolution - 1 {
            let row = y + resolution * z;
            if rows[row] & rows[row + 1] & rows[row + resolution] & rows[row + resolution + 1]
                == u32::MAX
            {
                continue;
            }
            let base = resolution * row;
            let mut last = usize::from(grid[base] != 0)
                | (usize::from(grid[base + resolution] != 0) << 2)
                | (usize::from(grid[base + slice] != 0) << 4)
                | (usize::from(grid[base + slice + resolution] != 0) << 6);
            for x in 0..resolution - 1 {
                work.add(1)?;
                let next = usize::from(grid[base + x + 1] != 0)
                    | (usize::from(grid[base + x + 1 + resolution] != 0) << 2)
                    | (usize::from(grid[base + x + 1 + slice] != 0) << 4)
                    | (usize::from(grid[base + x + 1 + slice + resolution] != 0) << 6);
                let cube = last | (next << 1);
                last = next;
                if cube == 0 || cube == 255 {
                    continue;
                }
                if destination.is_none() {
                    count += table::TRIANGLE_COUNT[cube] as usize;
                    continue;
                }
                let alternate = if table::TRIANGLE_ALT[cube] == 1 {
                    octant_decider(x, y, z, cube, grid, rows, voxels, resolution)
                } else {
                    table::TRIANGLE_ALT[cube] == 2
                        && options & REMESH_SOLVE != 0
                        && quadric_decider(x, y, z, cube, grid, rows, voxels, resolution, rscale)
                };
                for &triangle in &table::TRIANGLE_TABLE[cube][usize::from(alternate)] {
                    if triangle == 0 {
                        break;
                    }
                    if let Some(output) = &mut destination {
                        if count < output.len() / 3 {
                            for corner in 0..3 {
                                let code = ((triangle as usize) >> (8 - corner * 4)) & 0xf;
                                let voxel =
                                    corner_voxel(x, y, z, code, grid, rows, voxels, resolution)
                                        .expect("occupied output corner");
                                output[count * 3 + corner] = [
                                    voxel.p[0] + offset[0],
                                    voxel.p[1] + offset[1],
                                    voxel.p[2] + offset[2],
                                ];
                            }
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

#[allow(clippy::too_many_arguments)]
fn run(
    destination: Option<&mut [[f32; 3]]>,
    owned_output_bytes: usize,
    indices: &[u32],
    positions: Positions<'_>,
    resolution: usize,
    options: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !(4..=256).contains(&resolution) {
            return Err(Error::InvalidParameter);
        }
        if options & !(REMESH_SHELL | REMESH_SOLVE) != 0 {
            return Err(Error::UnknownFlags);
        }
        topology(indices, positions.len(), &mut work)?;
        for i in 0..positions.len() {
            work.add(1)?;
            if pos(positions, i).iter().any(|v| !v.is_finite()) {
                return Err(Error::NumericalFailure);
            }
        }
        let (scale, offset) = measure(positions, resolution, &mut work)?;
        let cells = resolution * resolution * resolution;
        let row_count = resolution * resolution;
        let bytes = cells
            .checked_add(row_count * 4)
            .and_then(|v| {
                v.checked_add(if options & REMESH_SHELL == 0 {
                    row_count * 5
                } else {
                    0
                })
            })
            .and_then(|v| v.checked_add(owned_output_bytes))
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(bytes)?;
        let mut grid = reserve::<u8>(cells)?;
        voxelize(
            &mut grid, None, None, indices, positions, resolution, scale, offset, options,
            &mut work,
        )?;
        let (rows, voxel_count) = rowpack(&mut grid, resolution, &mut work)?;
        let voxel_bytes = checked_bytes(voxel_count, core::mem::size_of::<Voxel>())?;
        workspace.account_codec(
            bytes
                .checked_add(if destination.is_some() {
                    voxel_bytes
                } else {
                    0
                })
                .ok_or(Error::SizeOverflow)?,
        )?;
        let mut voxels = if destination.is_some() {
            reserve::<Voxel>(voxel_count)?
        } else {
            Vec::new()
        };
        if options & REMESH_SHELL == 0 {
            solidify(&mut grid, &rows, resolution, &mut work)?;
        }
        if destination.is_some() {
            voxelize(
                &mut grid,
                Some(&rows),
                Some(&mut voxels),
                indices,
                positions,
                resolution,
                scale,
                offset,
                options,
                &mut work,
            )?;
            solve(&mut voxels, scale, options, &mut work)?;
        }
        polygonize(
            destination,
            &grid,
            &rows,
            &voxels,
            resolution,
            scale,
            offset,
            options,
            &mut work,
        )
    })();
    workspace.finish(&work);
    result
}

/// Return the upstream triangle-count upper bound with no output allocation.
pub fn remesh_bound(
    indices: &[u32],
    positions: Positions<'_>,
    resolution: usize,
    options: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    run(None, 0, indices, positions, resolution, options, workspace)
}

/// Write up to `destination.len()/3` triangles and return the full triangle count.
/// A short destination receives a prefix, matching upstream's partial-write rule.
pub fn remesh_into(
    destination: &mut [[f32; 3]],
    indices: &[u32],
    positions: Positions<'_>,
    resolution: usize,
    options: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if !destination.len().is_multiple_of(3) {
        return Err(Error::InvalidLayout);
    }
    run(
        Some(destination),
        0,
        indices,
        positions,
        resolution,
        options,
        workspace,
    )
}

/// Allocate the upper bound, remesh, and return three vertices per output triangle.
pub fn remesh(
    indices: &[u32],
    positions: Positions<'_>,
    resolution: usize,
    options: u32,
    workspace: &mut Workspace,
) -> Result<Vec<[f32; 3]>, Error> {
    let bound = remesh_bound(indices, positions, resolution, options, workspace)?;
    let capacity = bound.checked_mul(3).ok_or(Error::SizeOverflow)?;
    let output_bytes = checked_bytes(capacity, core::mem::size_of::<[f32; 3]>())?;
    if output_bytes > workspace.limits().max_bytes {
        return Err(Error::LimitExceeded);
    }
    let mut output = reserve::<[f32; 3]>(capacity)?;
    let count = run(
        Some(&mut output),
        output_bytes,
        indices,
        positions,
        resolution,
        options,
        workspace,
    )?;
    output.truncate(count * 3);
    Ok(output)
}
