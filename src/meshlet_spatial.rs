// Scalar-strict SAH builder from meshoptimizer 1.3 clusterizer.cpp.
use crate::meshlet::{MeshletSettings, Output};
use crate::processing::{finite, point, radix, Context};
use crate::{Error, Positions};
use alloc::vec::Vec;
#[path = "meshlet_division.rs"]
mod division;
use division::Divisor;
// Boxes padded to four lanes (component 3 is ignored), so each record is one
// 32-byte stride. Lanes 0-2 use upstream's area expression.
#[derive(Clone, Copy, Default)]
pub(crate) struct Box3 {
    pub(crate) lo: [f32; 4],
    pub(crate) hi: [f32; 4],
}
impl Box3 {
    fn empty() -> Self {
        Self {
            lo: [f32::MAX; 4],
            hi: [-f32::MAX; 4],
        }
    }
    // Ordered selects choose the incoming bound on ties, including signed zero.
    #[inline(always)]
    fn merge(&mut self, b: &Self) -> f32 {
        for k in 0..3 {
            self.lo[k] = if self.lo[k] < b.lo[k] {
                self.lo[k]
            } else {
                b.lo[k]
            };
            self.hi[k] = if self.hi[k] > b.hi[k] {
                self.hi[k]
            } else {
                b.hi[k]
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
    // Whole-triangle records: one index check per triangle instead of two
    // slice range checks.
    let triangles = indices.as_chunks::<3>().0;
    let mut n = 0;
    for (i, &v) in order.iter().enumerate() {
        let [a, b, c] = triangles[v as usize];
        // All three tests precede the marks, as upstream counts them.
        n += usize::from(used[a as usize] < 0)
            + usize::from(used[b as usize] < 0)
            + usize::from(used[c as usize] < 0);
        used[a as usize] = 1;
        used[b as usize] = 1;
        used[c as usize] = 1;
        if let Some(o) = out.as_deref_mut() {
            o[i] = n as u32;
        }
    }
    for &v in order {
        for v in triangles[v as usize] {
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
        // Reuse one exact reciprocal instead of two dependent integer
        // divisions per candidate split. EASY loops do not need it.
        let divisor = Divisor::new(if EASY { 1 } else { self.mint });
        let possible = |count| {
            let q = divisor.quotient(count);
            count - q * self.mint <= q * (self.max - self.mint)
        };
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
            if !EASY && (!possible(l) || (self.aligned && !possible(r))) {
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
            #[cfg(feature = "simd")]
            let vectorized =
                crate::codec::simd::spatial::areas(boxes, order, left_areas, right_areas);
            #[cfg(not(feature = "simd"))]
            let vectorized = false;
            if !vectorized {
                for i in 0..n {
                    left_areas[i] = left.merge(&boxes[order[i] as usize]);
                    right_areas[i] = right.merge(&boxes[order[n - 1 - i] as usize]);
                }
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
        // Local slices: the stores below cannot alias the builder's fields,
        // so their pointers and lengths stay in registers.
        let sides = &mut self.sides[..];
        let temp = &mut self.temp[..n];
        let (lower, upper) = self.axes[axis][start..start + n].split_at(split);
        for &v in lower {
            sides[v as usize] = 0;
        }
        for &v in upper {
            sides[v as usize] = 1;
        }
        for k in 0..3 {
            if k == axis {
                continue;
            }
            let order = &mut self.axes[k][start..start + n];
            let (mut l, mut r) = (0, split);
            for &v in order.iter() {
                // Sides are exactly 0/1. Select a destination and update both
                // cursors, avoiding an unpredictable branch for interleaved axes.
                let side = sides[v as usize] as usize;
                let destination = if side == 0 { l } else { r };
                temp[destination] = v;
                l += 1 - side;
                r += side;
            }
            order.copy_from_slice(temp);
        }
        self.split(start, split, depth + 1, ctx)?;
        self.split(start + split, n - split, depth + 1, ctx)
    }
}
fn fill_boxes(boxes: &mut [Box3], triangles: &[[u32; 3]], read: impl Fn(u32) -> [f32; 3]) {
    for (i, t) in triangles.iter().enumerate() {
        let a = read(t[0]);
        let b = read(t[1]);
        let c = read(t[2]);
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
    let triangles = indices.as_chunks::<3>().0;
    if let Some(values) = p.packed_values() {
        fill_boxes(&mut boxes, triangles, |i| values[i as usize]);
    } else {
        fill_boxes(&mut boxes, triangles, |i| point(p, i));
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

#[cfg(all(test, feature = "simd"))]
mod simd_tests {
    use super::Box3;
    use crate::codec::simd::{dispatch, spatial};

    fn compare(boxes: &[Box3], order: &[u32]) {
        for level in [dispatch::Level::Scalar, dispatch::detected_level()] {
            dispatch::with_level(level, || {
                let mut left = alloc::vec![123.; order.len() + 1];
                let mut right = left.clone();
                let used = spatial::areas(boxes, order, &mut left, &mut right);
                if level == dispatch::Level::Scalar {
                    assert!(!used);
                    return;
                }
                assert!(used);
                let (mut l, mut r) = (Box3::empty(), Box3::empty());
                for i in 0..order.len() {
                    for (actual, expected) in [
                        (left[i], l.merge(&boxes[order[i] as usize])),
                        (
                            right[i],
                            r.merge(&boxes[order[order.len() - 1 - i] as usize]),
                        ),
                    ] {
                        assert!(
                            actual.to_bits() == expected.to_bits()
                                || (actual.is_nan() && expected.is_nan()),
                            "{actual:?} != {expected:?}"
                        );
                    }
                }
                assert_eq!(left[order.len()], 123.);
                assert_eq!(right[order.len()], 123.);
            })
            .unwrap();
        }
    }

    #[test]
    fn signed_zero_ties() {
        for bits in 0..64 {
            let boxes: alloc::vec::Vec<_> = (0..2)
                .map(|i| Box3 {
                    lo: core::array::from_fn(|k| {
                        if bits & (1 << (i * 3 + k)) == 0 {
                            0.
                        } else {
                            -0.
                        }
                    }),
                    hi: core::array::from_fn(|k| {
                        if bits & (1 << (i * 3 + k)) == 0 {
                            -0.
                        } else {
                            0.
                        }
                    }),
                })
                .collect();
            compare(&boxes, &[0, 1, 0]);
        }
    }

    #[test]
    fn ordered_area_extremes() {
        let mut seed = 216u32;
        let boxes: alloc::vec::Vec<_> = (0..1024)
            .map(|_| {
                let mut next = || {
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    // Finite signed values: includes subnormals, overflow extents and areas.
                    f32::from_bits(seed & 0xff7f_ffff)
                };
                let a: [f32; 4] = core::array::from_fn(|_| next());
                let b: [f32; 4] = core::array::from_fn(|_| next());
                Box3 {
                    lo: core::array::from_fn(|k| if a[k] < b[k] { a[k] } else { b[k] }),
                    hi: core::array::from_fn(|k| if a[k] > b[k] { a[k] } else { b[k] }),
                }
            })
            .collect();
        compare(&boxes, &(0..1024).rev().collect::<alloc::vec::Vec<_>>());
    }

    #[test]
    fn explicit_boundary_values() {
        // Signed zeros, subnormals, MIN_POSITIVE and values at/next to the largest finite f32.
        let v = [
            0.,
            -0.,
            f32::from_bits(1),
            -f32::from_bits(1),
            f32::from_bits(0x007f_ffff),
            f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE,
            1.,
            f32::MAX,
            -f32::MAX,
            f32::from_bits(0x7f7f_fffe),
            -f32::from_bits(0x7f7f_fffe),
        ];
        let n = v.len();
        let mut boxes = alloc::vec::Vec::new();
        for i in 0..n {
            for j in 0..n {
                let (a, b) = (v[i], v[j]);
                let pair = |k: usize| if k & 1 == 0 { (a, b) } else { (b, a) };
                let lo = core::array::from_fn(|k| {
                    let (p, q) = pair(k + i);
                    if p < q {
                        p
                    } else {
                        q
                    }
                });
                let hi = core::array::from_fn(|k| {
                    let (p, q) = pair(k + j);
                    if p > q {
                        p
                    } else {
                        q
                    }
                });
                // Each box twice: exact ties against the accumulator.
                boxes.push(Box3 { lo, hi });
                boxes.push(Box3 { lo, hi });
            }
        }
        let len = boxes.len() as u32;
        compare(&boxes, &(0..len).collect::<alloc::vec::Vec<_>>());
        compare(
            &boxes,
            &(0..len)
                .map(|i| (i * 37) % len)
                .collect::<alloc::vec::Vec<_>>(),
        );
    }

    #[test]
    fn short_sweeps_preserve_output_tails() {
        let boxes = [Box3 {
            lo: [-1., -2., -3., 0.],
            hi: [2., 3., 4., 0.],
        }; 3];
        for n in 0..=3 {
            compare(&boxes[..n], &(0..n as u32).collect::<alloc::vec::Vec<_>>());
        }
    }
}
