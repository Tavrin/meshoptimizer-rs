//! Scalar software rasterizer used by the two upstream raster analyzers.
use crate::workspace::{checked_bytes, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

/// Result of `meshopt_analyzeOverdraw`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OverdrawStatistics {
    /// Rasterized pixels with at least one depth pass.
    pub pixels_covered: u32,
    /// Total depth passes.
    pub pixels_shaded: u32,
    /// Shaded to covered pixel ratio.
    pub overdraw: f32,
}

/// Result of `meshopt_analyzeCoverage`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CoverageStatistics {
    /// Covered viewport fraction along X, Y and Z.
    pub coverage: [f32; 3],
    /// Largest axis extent in mesh coordinates.
    pub extent: f32,
}

#[derive(Clone, Copy, Default)]
struct Pixel {
    depth: [f32; 2],
    overdraw: [u32; 2],
}
const VIEWPORT: usize = 256;

fn reserve<T: Default + Clone>(count: usize) -> Result<Vec<T>, Error> {
    checked_bytes(count, core::mem::size_of::<T>())?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    value.resize(count, T::default());
    Ok(value)
}

fn transform(
    indices: &[u32],
    positions: Positions<'_>,
    work: &mut Work,
) -> Result<(Vec<[f32; 3]>, f32), Error> {
    let mut minv = [f32::MAX; 3];
    let mut maxv = [-f32::MAX; 3];
    work.scan(0..positions.len(), |i| {
        let value = positions.get(i).ok_or(Error::InvalidLayout)?;
        for j in 0..3 {
            if !value[j].is_finite() {
                return Err(Error::NumericalFailure);
            }
            minv[j] = if minv[j] > value[j] {
                value[j]
            } else {
                minv[j]
            };
            maxv[j] = if maxv[j] < value[j] {
                value[j]
            } else {
                maxv[j]
            };
        }
        Ok(())
    })?;
    let mut extent = 0.0f32;
    for j in 0..3 {
        let d = maxv[j] - minv[j];
        extent = if d < extent { extent } else { d };
    }
    if !extent.is_finite() {
        return Err(Error::NumericalFailure);
    }
    let scale = if extent == 0.0 {
        0.0
    } else {
        VIEWPORT as f32 / extent
    };
    let mut transformed = reserve::<[f32; 3]>(indices.len())?;
    let bulk_transform = work.covers(indices.len())?;
    if bulk_transform {
        work.add(indices.len())?;
    }
    for (i, &index) in indices.iter().enumerate() {
        if !bulk_transform {
            work.add(1)?;
        }
        let value = positions
            .get(index as usize)
            .ok_or(Error::IndexOutOfBounds)?;
        transformed[i] = [
            (value[0] - minv[0]) * scale,
            (value[1] - minv[1]) * scale,
            (value[2] - minv[2]) * scale,
        ];
    }
    Ok((transformed, extent))
}

fn rasterize(buffer: &mut [Pixel], mut p: [[f32; 3]; 3], work: &mut Work) -> Result<(), Error> {
    let det = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
    let invdet = if det == 0.0 { 0.0 } else { 1.0 / det };
    let mut dzx = ((p[1][2] - p[0][2]) * (p[2][1] - p[0][1])
        - (p[1][1] - p[0][1]) * (p[2][2] - p[0][2]))
        * invdet;
    let mut dzy = ((p[1][0] - p[0][0]) * (p[2][2] - p[0][2])
        - (p[1][2] - p[0][2]) * (p[2][0] - p[0][0]))
        * invdet;
    let side = usize::from(det > 0.0);
    if side != 0 {
        let (a, b) = p.split_at_mut(2);
        core::mem::swap(&mut a[1][0], &mut b[0][0]);
        core::mem::swap(&mut a[1][1], &mut b[0][1]);
        p[0][2] = VIEWPORT as f32 - p[0][2];
        dzx = -dzx;
        dzy = -dzy;
    }
    let x = p.map(|v| (16.0 * v[0] + 0.5) as i32);
    let y = p.map(|v| (16.0 * v[1] + 0.5) as i32);
    let minx = ((x[0].min(x[1]).min(x[2]) + 7) >> 4).max(0);
    let miny = ((y[0].min(y[1]).min(y[2]) + 7) >> 4).max(0);
    let maxx = ((x[0].max(x[1]).max(x[2]) + 7) >> 4).min(VIEWPORT as i32);
    let maxy = ((y[0].max(y[1]).max(y[2]) + 7) >> 4).min(VIEWPORT as i32);
    let (dx12, dx23, dx31) = (x[0] - x[1], x[1] - x[2], x[2] - x[0]);
    let (dy12, dy23, dy31) = (y[0] - y[1], y[1] - y[2], y[2] - y[0]);
    let tl1 = i32::from(dy12 < 0 || (dy12 == 0 && dx12 > 0));
    let tl2 = i32::from(dy23 < 0 || (dy23 == 0 && dx23 > 0));
    let tl3 = i32::from(dy31 < 0 || (dy31 == 0 && dx31 > 0));
    let (fx, fy) = ((minx << 4) + 8, (miny << 4) + 8);
    let mut cy1 = dx12 * (fy - y[0]) - dy12 * (fx - x[0]) + tl1 - 1;
    let mut cy2 = dx23 * (fy - y[1]) - dy23 * (fx - x[1]) + tl2 - 1;
    let mut cy3 = dx31 * (fy - y[2]) - dy31 * (fx - x[2]) + tl3 - 1;
    let mut zy = p[0][2] + (dzx * (fx - x[0]) as f32 + dzy * (fy - y[0]) as f32) * (1.0 / 16.0);
    let visits = (maxx - minx).max(0) as usize * (maxy - miny).max(0) as usize;
    let bulk_pixels = work.covers(visits)?;
    if bulk_pixels {
        work.add(visits)?;
    }
    if minx >= maxx || miny >= maxy {
        return Ok(());
    }
    for yy in miny..maxy {
        let (mut cx1, mut cx2, mut cx3) = (cy1, cy2, cy3);
        let mut zx = zy;
        let row_start = yy as usize * VIEWPORT + minx as usize;
        let row_end = yy as usize * VIEWPORT + maxx as usize;
        for pixel in &mut buffer[row_start..row_end] {
            if !bulk_pixels {
                work.add(1)?;
            }
            if (cx1 | cx2 | cx3) >= 0 && zx >= pixel.depth[side] {
                pixel.depth[side] = zx;
                pixel.overdraw[side] += 1;
            }
            cx1 -= dy12 << 4;
            cx2 -= dy23 << 4;
            cx3 -= dy31 << 4;
            zx += dzx;
        }
        cy1 += dx12 << 4;
        cy2 += dx23 << 4;
        cy3 += dx31 << 4;
        zy += dzy;
    }
    Ok(())
}

fn analyze(
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
    coverage: bool,
) -> Result<(OverdrawStatistics, CoverageStatistics), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, positions.len(), &mut work)?;
        let bytes = checked_bytes(indices.len(), 12)?
            .checked_add(checked_bytes(
                VIEWPORT * VIEWPORT,
                core::mem::size_of::<Pixel>(),
            )?)
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(bytes)?;
        let (triangles, extent) = transform(indices, positions, &mut work)?;
        let mut buffer = reserve::<Pixel>(VIEWPORT * VIEWPORT)?;
        let mut overdraw = OverdrawStatistics::default();
        let mut result_coverage = CoverageStatistics {
            extent,
            ..CoverageStatistics::default()
        };
        for axis in 0..3 {
            buffer.fill(Pixel::default());
            for tri in triangles.as_chunks::<3>().0 {
                work.add(1)?;
                let [a, b, c] = *tri;
                let reordered = match axis {
                    0 => [[a[2], a[1], a[0]], [b[2], b[1], b[0]], [c[2], c[1], c[0]]],
                    1 => [[a[0], a[2], a[1]], [b[0], b[2], b[1]], [c[0], c[2], c[1]]],
                    _ => [[a[1], a[0], a[2]], [b[1], b[0], b[2]], [c[1], c[0], c[2]]],
                };
                rasterize(&mut buffer, reordered, &mut work)?;
            }
            let mut covered = 0;
            let bulk_scan = work.covers(buffer.len())?;
            if bulk_scan {
                work.add(buffer.len())?;
            }
            for pixel in &buffer {
                if !bulk_scan {
                    work.add(1)?;
                }
                if coverage {
                    covered += u32::from((pixel.overdraw[0] | pixel.overdraw[1]) > 0);
                } else {
                    for &value in &pixel.overdraw {
                        overdraw.pixels_covered += u32::from(value > 0);
                        overdraw.pixels_shaded += value;
                    }
                }
            }
            if coverage {
                result_coverage.coverage[axis] = covered as f32 / (VIEWPORT * VIEWPORT) as f32;
            }
        }
        overdraw.overdraw = if overdraw.pixels_covered == 0 {
            0.0
        } else {
            overdraw.pixels_shaded as f32 / overdraw.pixels_covered as f32
        };
        Ok((overdraw, result_coverage))
    })();
    workspace.finish(&work);
    result
}

/// Analyze raster overdraw with upstream's 256-pixel viewport model.
pub fn analyze_overdraw(
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<OverdrawStatistics, Error> {
    Ok(analyze(indices, positions, workspace, false)?.0)
}

/// Analyze projected coverage on three axes with the upstream rasterizer.
pub fn analyze_coverage(
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<CoverageStatistics, Error> {
    Ok(analyze(indices, positions, workspace, true)?.1)
}
