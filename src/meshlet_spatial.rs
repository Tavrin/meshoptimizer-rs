// Scalar-strict SAH builder from meshoptimizer 1.3 clusterizer.cpp.
use crate::meshlet::{MeshletSettings, Output};
use crate::processing::{finite, point, radix, Context};
use crate::{Error, Positions};
use alloc::vec::Vec;
// Boxes padded to four lanes (lane 3 is ignored), so each merge loads
// whole aligned records. Lanes 0-2 use upstream's ordered selects and area
// expression, so results are identical.
#[derive(Clone, Copy, Default)]
struct Box3 {
    lo: [f32; 4],
    hi: [f32; 4],
}
impl Box3 {
    fn empty() -> Self {
        Self {
            lo: [f32::MAX; 4],
            hi: [-f32::MAX; 4],
        }
    }
    #[inline(always)]
    fn merge(&mut self, b: Self) -> f32 {
        for k in 0..4 {
            self.lo[k] = if b.lo[k] < self.lo[k] {
                b.lo[k]
            } else {
                self.lo[k]
            };
            self.hi[k] = if b.hi[k] > self.hi[k] {
                b.hi[k]
            } else {
                self.hi[k]
            };
        }
        let [x, y, z] = [
            self.hi[0] - self.lo[0],
            self.hi[1] - self.lo[1],
            self.hi[2] - self.lo[2],
        ];
        x * y + y * z + z * x
    }
}
fn count(
    order: &[u32],
    used: &mut [i16],
    indices: &[u32],
    mut out: Option<&mut [u32]>,
    ctx: &mut Context<'_>,
) -> Result<usize, Error> {
    ctx.tick(order.len() * 2)?;
    let mut n = 0;
    for (i, &v) in order.iter().enumerate() {
        let t = &indices[v as usize * 3..][..3];
        n += t.iter().filter(|&&v| used[v as usize] < 0).count();
        for &v in t {
            used[v as usize] = 1;
        }
        if let Some(o) = out.as_deref_mut() {
            o[i] = n as u32;
        }
    }
    for &v in order {
        for &v in &indices[v as usize * 3..][..3] {
            used[v as usize] = -1;
        }
    }
    Ok(n)
}
fn divisible(n: usize, min: usize, max: usize) -> bool {
    if min * 2 <= max {
        n >= min
    } else {
        n % min <= (n / min) * (max - min)
    }
}
// `bvhPivot`: the best SAH split along one axis.
struct Pivot<'a> {
    areas: &'a [f32],
    counts: &'a [u32],
    n: usize,
    step: usize,
    mint: usize,
    max: usize,
    aligned: bool,
    end: usize,
    maxfill: usize,
    fill: f32,
}
impl Pivot<'_> {
    // COUNTS: vertex-bound fill counts (n <= max triangles). EASY: mint * 2 <=
    // max, where divisible(x) is x >= mint; every visited left size is at
    // least mint, and with `aligned` every right size is too (end = n - mint),
    // so the divisibility tests are always true and are skipped.
    #[inline(always)]
    fn run<const COUNTS: bool, const EASY: bool>(&self) -> (f32, usize) {
        let n = self.n;
        let mut best = (f32::MAX, 0);
        let mut i = self.mint - 1;
        if EASY && self.step == 1 {
            // Compute eight base costs at a time (vectorizable, same
            // per-element expression), then visit them in order.
            while i + 8 <= self.end {
                let left = &self.areas[i..i + 8];
                let right = &self.areas[2 * n - 9 - i..2 * n - 1 - i];
                let mut costs = [0f32; 8];
                for j in 0..8 {
                    let l = i + j + 1;
                    costs[j] = left[j] * l as f32 + right[7 - j] * (n - l) as f32;
                }
                for (j, &cost) in costs.iter().enumerate() {
                    if cost <= best.0 {
                        self.fill_cost::<COUNTS>(i + j, cost, &mut best);
                    }
                }
                i += 8;
            }
        }
        while i < self.end {
            let l = i + 1;
            let r = n - l;
            if !EASY
                && (!divisible(l, self.mint, self.max)
                    || (self.aligned && !divisible(r, self.mint, self.max)))
            {
                i += self.step;
                continue;
            }
            let cost = self.areas[i] * l as f32 + self.areas[n - 1 - l + n] * r as f32;
            if cost <= best.0 {
                self.fill_cost::<COUNTS>(i, cost, &mut best);
            }
            i += self.step;
        }
        best
    }
    // Add the vertex or triangle fill cost to a split's base SAH cost.
    #[inline(always)]
    fn fill_cost<const COUNTS: bool>(&self, i: usize, cost: f32, best: &mut (f32, usize)) {
        let n = self.n;
        let maxfill = self.maxfill;
        let rmaxfill = 1. / maxfill as f32;
        let l = i + 1;
        let r = n - l;
        let la = self.areas[i];
        let ra = self.areas[n - 1 - l + n];
        let (lf, rf) = if COUNTS {
            let c = self.counts[i] as usize;
            (c, c)
        } else {
            (l, r)
        };
        let lr = ((lf + maxfill - 1) as f32 * rmaxfill) as i32 * maxfill as i32 - lf as i32;
        let rr = ((rf + maxfill - 1) as f32 * rmaxfill) as i32 * maxfill as i32 - rf as i32;
        let cost = cost + self.fill * (lr as f32 * la + rr as f32 * ra);
        if cost < best.0 {
            *best = (cost, l);
        }
    }
}
struct Bvh<'a> {
    boxes: &'a [Box3],
    axes: [Vec<u32>; 3],
    boundary: Vec<u8>,
    used: Vec<i16>,
    areas: Vec<f32>,
    counts: Vec<u32>,
    temp: Vec<u32>,
    sides: Vec<u8>,
    indices: &'a [u32],
    settings: MeshletSettings,
    min: usize,
    fill: f32,
}
impl Bvh<'_> {
    fn mark(&mut self, start: usize, n: usize) {
        self.boundary[start] = 1;
        self.boundary[start + 1..start + n].fill(0);
    }
    fn tail(&mut self, start: usize, n: usize, ctx: &mut Context<'_>) -> Result<(), Error> {
        let mut i = 0;
        while i < n {
            let chunk = self.settings.max_triangles.min(n - i);
            let cnt = count(
                &self.axes[0][start + i..start + i + chunk],
                &mut self.used,
                self.indices,
                None,
                ctx,
            )?;
            let size = if cnt <= self.settings.max_vertices {
                chunk
            } else {
                self.settings.max_vertices / 3
            };
            self.mark(start + i, size);
            i += size;
        }
        Ok(())
    }
    fn split(
        &mut self,
        start: usize,
        n: usize,
        depth: usize,
        ctx: &mut Context<'_>,
    ) -> Result<(), Error> {
        let s = self.settings;
        if n <= s.max_triangles
            && count(
                &self.axes[0][start..start + n],
                &mut self.used,
                self.indices,
                None,
                ctx,
            )? <= s.max_vertices
        {
            self.mark(start, n);
            return Ok(());
        }
        let step = if self.min == s.max_triangles && n > s.max_triangles {
            s.max_triangles
        } else {
            1
        };
        let mint = if n <= s.max_triangles && s.max_vertices / 3 < self.min {
            s.max_vertices / 3
        } else {
            self.min
        };
        let maxfill = if n <= s.max_triangles {
            s.max_vertices
        } else {
            s.max_triangles
        };
        let mut best = None;
        let mut bestcost = f32::MAX;
        for k in 0..3 {
            ctx.tick(n)?;
            let mut left = Box3::empty();
            let mut right = Box3::empty();
            // Local slices: no per-iteration reloads or index checks besides boxes.
            let boxes = self.boxes;
            let order = &self.axes[k][start..start + n];
            let (left_areas, right_areas) = self.areas[..2 * n].split_at_mut(n);
            for ((la, ra), (&l, &r)) in left_areas
                .iter_mut()
                .zip(right_areas.iter_mut())
                .zip(order.iter().zip(order.iter().rev()))
            {
                *la = left.merge(boxes[l as usize]);
                *ra = right.merge(boxes[r as usize]);
            }
            if n <= s.max_triangles {
                count(
                    &self.axes[k][start..start + n],
                    &mut self.used,
                    self.indices,
                    Some(&mut self.counts[..n]),
                    ctx,
                )?;
            }
            let aligned = n >= mint * 2 && divisible(n, mint, s.max_triangles);
            let end = if aligned { n - mint } else { n - 1 };
            ctx.tick((mint - 1..end).step_by(step).len())?;
            let pivot = Pivot {
                areas: &self.areas[..2 * n],
                counts: &self.counts[..n.min(s.max_triangles)],
                n,
                step,
                mint,
                max: s.max_triangles,
                aligned,
                end,
                maxfill,
                fill: self.fill,
            };
            // Specialize the loop-invariant choices; same visits and costs.
            let (axiscost, axissplit) = match (n <= s.max_triangles, mint * 2 <= s.max_triangles) {
                (true, true) => pivot.run::<true, true>(),
                (true, false) => pivot.run::<true, false>(),
                (false, true) => pivot.run::<false, true>(),
                (false, false) => pivot.run::<false, false>(),
            };
            if axissplit != 0 && axiscost < bestcost {
                best = Some((k, axissplit));
                bestcost = axiscost;
            }
        }
        let Some((axis, split)) = best else {
            return self.tail(start, n, ctx);
        };
        if depth >= 50 {
            return self.tail(start, n, ctx);
        }
        ctx.tick(n * 3)?;
        for i in 0..n {
            self.sides[self.axes[axis][start + i] as usize] = u8::from(i >= split);
        }
        for k in 0..3 {
            if k == axis {
                continue;
            }
            let (mut l, mut r) = (0, split);
            for &v in &self.axes[k][start..start + n] {
                if self.sides[v as usize] == 0 {
                    self.temp[l] = v;
                    l += 1;
                } else {
                    self.temp[r] = v;
                    r += 1;
                }
            }
            self.axes[k][start..start + n].copy_from_slice(&self.temp[..n]);
        }
        self.split(start, split, depth + 1, ctx)?;
        self.split(start + split, n - split, depth + 1, ctx)
    }
}
pub(crate) fn spatial(
    out: &mut Output<'_>,
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min: usize,
    fill: f32,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let n = indices.len() / 3;
    if n == 0 {
        return Ok(());
    }
    let mut boxes = ctx.alloc::<Box3>(n)?;
    ctx.tick(n)?;
    for (i, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        let a = point(p, t[0]);
        let b = point(p, t[1]);
        let c = point(p, t[2]);
        for k in 0..3 {
            let mut lo = if a[k] < b[k] { a[k] } else { b[k] };
            if c[k] < lo {
                lo = c[k];
            }
            let mut hi = if a[k] > b[k] { a[k] } else { b[k] };
            if c[k] > hi {
                hi = c[k];
            }
            boxes[i].lo[k] = lo;
            boxes[i].hi[k] = hi;
        }
    }
    let mut axes = [ctx.alloc::<u32>(n)?, ctx.alloc(n)?, ctx.alloc(n)?];
    let mut temp = ctx.alloc(n)?;
    // Upstream radixFloat keys without the two low bits fit in u32.
    let mut keys = ctx.alloc::<u32>(n)?;
    for (k, axis) in axes.iter_mut().enumerate() {
        for i in 0..n {
            axis[i] = i as u32;
            let c = finite((boxes[i].lo[k] + boxes[i].hi[k]) / 2.)?.to_bits();
            keys[i] = (c ^ if c & 0x80000000 != 0 {
                u32::MAX
            } else {
                0x80000000
            }) >> 2;
        }
        radix::<3, u32>(axis, &mut temp, &keys, ctx)?;
    }
    // The keys are not needed past sorting; release them before the builder's
    // scratch so peak storage stays comparable with upstream's shared scratch.
    ctx.free(keys)?;
    let mut used = ctx.alloc::<i16>(p.len())?;
    used.fill(-1);
    let mut bvh = Bvh {
        boxes: &boxes,
        axes,
        boundary: ctx.alloc(n)?,
        used,
        areas: ctx.alloc(n * 2)?,
        counts: ctx.alloc(n)?,
        temp,
        sides: ctx.alloc(n)?,
        indices,
        settings: s,
        min,
        fill,
    };
    bvh.split(0, n, 0, ctx)?;
    let count = bvh.boundary.iter().map(|&v| v as usize).sum::<usize>();
    let bound = crate::build_meshlets_bound(indices.len(), s.max_vertices, min)?;
    let mut pending = count;
    for i in 0..n {
        let mut split = i > 0 && bvh.boundary[i] == 1;
        if split && count > bound && out.count + pending >= bound {
            split = false;
        }
        let t = &indices.as_chunks::<3>().0[bvh.axes[0][i] as usize];
        out.append(t, &mut bvh.used, s, split, ctx)?;
        pending -= bvh.boundary[i] as usize;
    }
    Ok(())
}
