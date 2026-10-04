#![allow(clippy::too_many_arguments)]
// Port of meshoptimizer 1.3 spatialorder.cpp (MIT, Arseny Kapoulkine).
use crate::processing::{finite, point, radix, Context};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

fn part(mut x: u64) -> u64 {
    x &= 0xfffff;
    x = (x ^ (x << 32)) & 0x000f00000000ffff;
    x = (x ^ (x << 16)) & 0x000f0000ff0000ff;
    x = (x ^ (x << 8)) & 0x000f00f00f00f00f;
    x = (x ^ (x << 4)) & 0x00c30c30c30c30c3;
    (x ^ (x << 2)) & 0x0249249249249249
}
fn keys(p: Positions<'_>, morton: bool, ctx: &mut Context<'_>) -> Result<Vec<u64>, Error> {
    let mut keys = ctx.alloc(p.len())?;
    if p.is_empty() {
        return Ok(keys);
    }
    let mut lo = [f32::MAX; 3];
    let mut hi = [-f32::MAX; 3];
    ctx.tick(p.len() * 2)?;
    p.for_each(|v| {
        for j in 0..3 {
            if lo[j] > v[j] {
                lo[j] = v[j];
            }
            if hi[j] < v[j] {
                hi[j] = v[j];
            }
        }
        Ok(())
    })?;
    let mut extent = 0.0;
    for j in 0..3 {
        let e = finite(hi[j] - lo[j])?;
        if e >= extent {
            extent = e;
        }
    }
    let scale = if extent == 0.0 {
        0.0
    } else {
        finite(65535.0 / extent)?
    };
    let mut destination = keys.iter_mut();
    p.for_each(|v| {
        // All positions are finite and within [lo, hi]. Extent and scale
        // were checked above; these products are bounded by 65535 plus
        // rounding, including the zero-extent case.
        let x = ((v[0] - lo[0]) * scale + 0.5) as u32 as u64;
        let y = ((v[1] - lo[1]) * scale + 0.5) as u32 as u64;
        let z = ((v[2] - lo[2]) * scale + 0.5) as u32 as u64;
        let key = destination.next().expect("validated position count");
        *key = if morton {
            part(x) | (part(y) << 1) | (part(z) << 2)
        } else {
            x | (y << 20) | (z << 40)
        };
        Ok(())
    })?;
    Ok(keys)
}
fn remap_into(out: &mut [u32], p: Positions<'_>, ctx: &mut Context<'_>) -> Result<(), Error> {
    let keys = keys(p, true, ctx)?;
    let mut order = ctx.alloc::<u32>(p.len())?;
    let mut temp = ctx.alloc::<u32>(p.len())?;
    for (i, v) in order.iter_mut().enumerate() {
        *v = i as u32;
    }
    radix::<5, u64>(&mut order, &mut temp, &keys, ctx)?;
    ctx.tick(order.len())?;
    for (i, &v) in order.iter().enumerate() {
        out[v as usize] = i as u32;
    }
    Ok(())
}
/// `meshopt_spatialSortRemap`: old-to-new Morton remap.
pub fn spatial_sort_remap(p: Positions<'_>, workspace: &mut Workspace) -> Result<Vec<u32>, Error> {
    let mut ctx = Context::new(workspace)?;
    ctx.positions(p)?;
    let mut out = ctx.alloc(p.len())?;
    remap_into(&mut out, p, &mut ctx)?;
    Ok(out)
}
/// Caller-buffer Morton remap; the unused tail is preserved.
pub fn spatial_sort_remap_into(
    out: &mut [u32],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    if out.len() < p.len() {
        return Err(Error::BufferTooSmall);
    }
    ctx.positions(p)?;
    remap_into(&mut out[..p.len()], p, &mut ctx)
}
fn triangles_into(
    out: &mut [u32],
    indices: &[u32],
    p: Positions<'_>,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let mut centers = ctx.alloc::<[f32; 3]>(indices.len() / 3)?;
    ctx.tick(centers.len())?;
    for (i, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        let a = point(p, t[0]);
        let b = point(p, t[1]);
        let c = point(p, t[2]);
        for j in 0..3 {
            centers[i][j] = finite((a[j] + b[j] + c[j]) / 3.0)?;
        }
    }
    let mut remap = ctx.alloc(centers.len())?;
    remap_into(&mut remap, Positions::from_packed(&centers), ctx)?;
    ctx.tick(centers.len())?;
    for (i, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        out[remap[i] as usize * 3..][..3].copy_from_slice(t);
    }
    Ok(())
}
/// `meshopt_spatialSortTriangles`: reorder complete triangles by centroid Morton order.
pub fn spatial_sort_triangles(
    indices: &[u32],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut ctx = Context::new(workspace)?;
    ctx.topology(indices, p.len())?;
    ctx.positions(p)?;
    let mut out = ctx.alloc(indices.len())?;
    triangles_into(&mut out, indices, p, &mut ctx)?;
    Ok(out)
}
/// Caller-buffer spatial triangle sorting; the unused tail is preserved.
pub fn spatial_sort_triangles_into(
    out: &mut [u32],
    indices: &[u32],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    if out.len() < indices.len() {
        return Err(Error::BufferTooSmall);
    }
    ctx.topology(indices, p.len())?;
    ctx.positions(p)?;
    triangles_into(&mut out[..indices.len()], indices, p, &mut ctx)
}
/// Atomic in-place spatial triangle sorting.
pub fn spatial_sort_triangles_in_place(
    indices: &mut [u32],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = spatial_sort_triangles(indices, p, workspace)?;
    indices.copy_from_slice(&result);
    Ok(())
}
fn split(
    out: &mut [u32],
    axes: &mut [Vec<u32>; 3],
    start: usize,
    n: usize,
    keys: &[u64],
    temp: &mut [u32],
    sides: &mut [u8],
    size: usize,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    if n <= size {
        out.copy_from_slice(&axes[0][start..start + n]);
        return Ok(());
    }
    ctx.tick(n * 3)?;
    let mut best = 0;
    let mut dim = 0;
    for (k, axis) in axes.iter().enumerate() {
        let d = ((keys[axis[start + n - 1] as usize] >> (k * 20)) & 0xfffff)
            - ((keys[axis[start] as usize] >> (k * 20)) & 0xfffff);
        if d >= dim {
            best = k;
            dim = d;
        }
    }
    let s = (n / 2).div_ceil(size) * size;
    for i in 0..n {
        sides[axes[best][start + i] as usize] = u8::from(i >= s);
    }
    for (k, axis) in axes.iter_mut().enumerate() {
        if k == best {
            continue;
        }
        let (mut l, mut r) = (0, s);
        for &v in &axis[start..start + n] {
            let side = sides[v as usize] as usize;
            temp[if side != 0 { r } else { l }] = v;
            l += 1 - side;
            r += side;
        }
        axis[start..start + n].copy_from_slice(&temp[..n]);
    }
    let (left, right) = out.split_at_mut(s);
    split(left, axes, start, s, keys, temp, sides, size, ctx)?;
    split(right, axes, start + s, n - s, keys, temp, sides, size, ctx)
}
fn cluster_into(
    out: &mut [u32],
    p: Positions<'_>,
    size: usize,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let keys = keys(p, false, ctx)?;
    let n = p.len();
    let mut axes = [ctx.alloc(n)?, ctx.alloc(n)?, ctx.alloc(n)?];
    let mut temp = ctx.alloc(n)?;
    let mut sides = ctx.alloc(n)?;
    let mut key = ctx.alloc::<u16>(n)?;
    for (k, axis) in axes.iter_mut().enumerate() {
        for i in 0..n {
            axis[i] = i as u32;
            key[i] = (keys[i] >> (k * 20)) as u16;
        }
        ctx.tick(n * 4)?;
        let mut hist = [[0u32; 2]; 256];
        for &v in &key {
            hist[(v & 255) as usize][0] += 1;
            hist[(v >> 8) as usize][1] += 1;
        }
        let mut sums = [0; 2];
        for h in &mut hist {
            let counts = *h;
            *h = sums;
            sums[0] += counts[0];
            sums[1] += counts[1];
        }
        for &v in axis.iter() {
            let h = &mut hist[(key[v as usize] & 255) as usize][0];
            temp[*h as usize] = v;
            *h += 1;
        }
        for &v in &temp {
            let h = &mut hist[(key[v as usize] >> 8) as usize][1];
            axis[*h as usize] = v;
            *h += 1;
        }
    }
    split(
        out, &mut axes, 0, n, &keys, &mut temp, &mut sides, size, ctx,
    )
}
/// `meshopt_spatialClusterPoints`: new-to-old permutation in fixed-size clusters.
pub fn spatial_cluster_points(
    p: Positions<'_>,
    size: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut ctx = Context::new(workspace)?;
    if size == 0 {
        return Err(Error::InvalidParameter);
    }
    ctx.positions(p)?;
    let mut out = ctx.alloc(p.len())?;
    cluster_into(&mut out, p, size, &mut ctx)?;
    Ok(out)
}
/// Caller-buffer point clustering; the unused tail is preserved.
pub fn spatial_cluster_points_into(
    out: &mut [u32],
    p: Positions<'_>,
    size: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    if size == 0 {
        return Err(Error::InvalidParameter);
    }
    if out.len() < p.len() {
        return Err(Error::BufferTooSmall);
    }
    ctx.positions(p)?;
    cluster_into(&mut out[..p.len()], p, size, &mut ctx)
}
