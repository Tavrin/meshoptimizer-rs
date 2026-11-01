//! Grid simplification and connected-component pruning from simplifier.cpp.
use super::*;
use crate::budget::Budget as Heap;

fn counted_bounds(positions: Positions<'_>, work: &mut Work) -> Result<([f32; 3], f32), Error> {
    let mut lo = [f32::MAX; 3];
    let mut hi = [-f32::MAX; 3];
    // Packed positions that are all finite and fit the budget take one pass
    // without per-vertex error exits; it yields the bounds of the ordered
    // selects below and the scan's charge is one visit per vertex. Otherwise
    // the checked scan reports the first non-finite value at the original charge.
    let packed = positions
        .packed_values()
        .filter(|values| work.covers(values.len()).unwrap_or(false));
    if let Some(values) = packed {
        if let Some((lo, hi)) = packed_bounds(values) {
            work.add(values.len())?;
            return bounds_extent(positions, lo, hi);
        }
    }
    positions.for_each_counted(work, |v| {
        for j in 0..3 {
            if !v[j].is_finite() {
                return Err(Error::InvalidParameter);
            }
            if lo[j] > v[j] {
                lo[j] = v[j];
            }
            if hi[j] < v[j] {
                hi[j] = v[j];
            }
        }
        Ok(())
    })?;
    bounds_extent(positions, lo, hi)
}
#[inline(always)]
fn bounds_extent(
    positions: Positions<'_>,
    lo: [f32; 3],
    hi: [f32; 3],
) -> Result<([f32; 3], f32), Error> {
    if positions.is_empty() {
        return Ok((lo, 0.0));
    }
    let mut extent = 0.0;
    for j in 0..3 {
        let d = hi[j] - lo[j];
        if d >= extent {
            extent = d;
        }
    }
    if !extent.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok((lo, extent))
}
/// Bounds that the ordered selects `if lo > v { lo = v }` and
/// `if hi < v { hi = v }` give over finite packed positions, or `None` if a
/// value is not finite. The flattened coordinates are read in blocks of four
/// vertices: lane `l` of accumulator `k` holds component `(4 * k + l) % 3`.
/// For finite values the selects compute the numerical minimum and maximum,
/// so lane order changes only the sign of a zero result; the ordered selects
/// keep the first zero of that component (every earlier value lies strictly on
/// the other side of it), which the final pass restores.
#[inline(always)]
#[allow(clippy::eq_op)] // `v - v` is the finiteness probe, not a zero.
fn packed_bounds(values: &[[f32; 3]]) -> Option<([f32; 3], [f32; 3])> {
    let (blocks, tail) = values.as_flattened().as_chunks::<12>();
    let [lo0, lo1, lo2, hi0, hi1, hi2, nan0, nan1, nan2] = bounds_lanes(blocks);
    let mut lo = [f32::MAX; 3];
    let mut hi = [-f32::MAX; 3];
    let mut nan = 0.0f32;
    for (k, (lanes_lo, lanes_hi, lanes_nan)) in
        [(lo0, hi0, nan0), (lo1, hi1, nan1), (lo2, hi2, nan2)]
            .into_iter()
            .enumerate()
    {
        for l in 0..4 {
            let j = (4 * k + l) % 3;
            if lo[j] > lanes_lo[l] {
                lo[j] = lanes_lo[l];
            }
            if hi[j] < lanes_hi[l] {
                hi[j] = lanes_hi[l];
            }
            nan += lanes_nan[l];
        }
    }
    for (i, &v) in tail.iter().enumerate() {
        let j = i % 3;
        if lo[j] > v {
            lo[j] = v;
        }
        if hi[j] < v {
            hi[j] = v;
        }
        nan += v - v;
    }
    if nan != 0.0 {
        return None;
    }
    for j in 0..3 {
        for bound in [&mut lo[j], &mut hi[j]] {
            if *bound == 0.0 {
                if let Some(v) = values.iter().find(|v| v[j] == 0.0) {
                    *bound = v[j];
                }
            }
        }
    }
    Some((lo, hi))
}
/// Lane accumulators of [`packed_bounds`]: minima, maxima and finiteness
/// sums for the three quads of each twelve-value block. Kept out of line so
/// the lane-to-component combination is not folded into the loop as shuffles.
#[inline(never)]
#[allow(clippy::eq_op)] // `v - v` is the finiteness probe, not a zero.
fn bounds_lanes(blocks: &[[f32; 12]]) -> [[f32; 4]; 9] {
    // `v - v` is +0 for finite `v` and NaN otherwise; NaN persists in a sum.
    #[inline(always)]
    fn quad(lo: &mut [f32; 4], hi: &mut [f32; 4], nan: &mut [f32; 4], v: &[f32; 4]) {
        for l in 0..4 {
            lo[l] = if lo[l] > v[l] { v[l] } else { lo[l] };
            hi[l] = if hi[l] < v[l] { v[l] } else { hi[l] };
            nan[l] += v[l] - v[l];
        }
    }
    let (mut lo0, mut lo1, mut lo2) = ([f32::MAX; 4], [f32::MAX; 4], [f32::MAX; 4]);
    let (mut hi0, mut hi1, mut hi2) = ([-f32::MAX; 4], [-f32::MAX; 4], [-f32::MAX; 4]);
    let (mut nan0, mut nan1, mut nan2) = ([0.0f32; 4], [0.0f32; 4], [0.0f32; 4]);
    for block in blocks {
        let q = block.as_chunks::<4>().0;
        quad(&mut lo0, &mut hi0, &mut nan0, &q[0]);
        quad(&mut lo1, &mut hi1, &mut nan1, &q[1]);
        quad(&mut lo2, &mut hi2, &mut nan2, &q[2]);
    }
    [lo0, lo1, lo2, hi0, hi1, hi2, nan0, nan1, nan2]
}
fn scaled(
    positions: Positions<'_>,
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<Vec<V>, Error> {
    let (lo, extent) = counted_bounds(positions, work)?;
    let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
    if !scale.is_finite() {
        return Err(Error::NumericalFailure);
    }
    let mut p = budget.filled(positions.len(), V::default())?;
    scale_values(positions, &mut p, lo, scale, work)?;
    Ok(p)
}
fn scale_values(
    positions: Positions<'_>,
    p: &mut [V],
    lo: [f32; 3],
    scale: f32,
    work: &mut Work,
) -> Result<(), Error> {
    if let Some(source) = positions.packed_values() {
        work.scan(source.iter().zip(p.iter_mut()), |(a, v)| {
            *v = V {
                x: (a[0] - lo[0]) * scale,
                y: (a[1] - lo[1]) * scale,
                z: (a[2] - lo[2]) * scale,
            };
            Ok(())
        })?;
    } else {
        let mut i = 0;
        positions.for_each_counted(work, |a| {
            let v = &mut p[i];
            i += 1;
            *v = V {
                x: (a[0] - lo[0]) * scale,
                y: (a[1] - lo[1]) * scale,
                z: (a[2] - lo[2]) * scale,
            };
            Ok(())
        })?;
    }
    Ok(())
}
pub(super) fn components(
    p: &[V],
    remap: &[u32],
    indices: &[u32],
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<(Vec<u32>, Vec<f32>), Error> {
    let (c, nc) = component_ids(remap, indices, budget, work)?;
    let errors = component_errors(p, &c, nc, budget, work)?;
    Ok((c, errors))
}
/// Charge `count` unit visits and run them without per-visit limit checks
/// when all fit; otherwise run the checked visits. Visits cannot fail except
/// by exhaustion, so the charged prefix and error are unchanged.
macro_rules! units {
    ($work:expr, $count:expr, |$m:ident| $body:expr) => {{
        let work: &mut Work = $work;
        if work.precharge($count, 1)? {
            let mut tally = super::Prepaid(0);
            let $m = &mut tally;
            $body
        } else {
            let $m = &mut *work;
            $body
        }
    }};
}
pub(super) fn component_ids(
    remap: &[u32],
    indices: &[u32],
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<(Vec<u32>, usize), Error> {
    let mut c = budget.filled(remap.len(), 0u32)?;
    let triangles = indices.as_chunks::<3>().0;
    // Each batch proves its complete traversal bound before using a local
    // tally. Tight budgets retain the original charge-before-visit path.
    units!(work, remap.len(), |m| {
        for (i, r) in c.iter_mut().enumerate() {
            m.add(1)?;
            *r = i as u32;
        }
    });
    let mut start = 0;
    // Each triangle has three edge charges and at most six n-1 walks.
    let edge_bound = remap.len().checked_mul(6).and_then(|n| n.checked_add(3));
    while start < triangles.len() {
        let batch = match edge_bound {
            Some(bound) => work.batch(triangles.len() - start, bound)?,
            None => 0,
        };
        if batch == 0 {
            component_edges::<true>(&mut c, remap, &triangles[start..], work)?;
            break;
        }
        component_edges::<false>(&mut c, remap, &triangles[start..start + batch], work)?;
        start += batch;
    }
    start = 0;
    while start < remap.len() {
        let batch = work.batch(remap.len() - start, remap.len())?;
        if batch == 0 {
            component_roots::<true>(&mut c, remap, start, remap.len(), work)?;
            break;
        }
        component_roots::<false>(&mut c, remap, start, start + batch, work)?;
        start += batch;
    }
    let mut nc = 0;
    units!(work, remap.len(), |m| {
        for (i, &r) in remap.iter().enumerate() {
            m.add(1)?;
            component_number(&mut c, &mut nc, i, r);
        }
    });
    Ok((c, nc as usize))
}
/// Union-find over welded triangle edges, then dense component ids. With
/// `CHECKED`, every visit is charged to `work` before it runs; otherwise the
/// visits are added to the batch's local tally.
#[inline(always)]
fn component_follow<const CHECKED: bool>(
    c: &mut [u32],
    mut i: usize,
    work: &mut Work,
    steps: &mut usize,
) -> Result<usize, Error> {
    // Parents are smaller than their children (unions link the larger root to
    // the smaller, compression copies a smaller parent), so one walk visits at
    // most `n - 1` parents and a record's tally below stays under 2^36.
    while i != c[i] as usize {
        if CHECKED {
            work.add(1)?;
        } else {
            *steps += 1;
        }
        let parent = c[i] as usize;
        c[i] = c[parent];
        i = parent;
    }
    Ok(i)
}
#[inline(always)]
fn component_edges<const CHECKED: bool>(
    c: &mut [u32],
    remap: &[u32],
    triangles: &[[u32; 3]],
    work: &mut Work,
) -> Result<(), Error> {
    let mut steps = 0;
    for tri in triangles {
        for e in 0..3 {
            if CHECKED {
                work.add(1)?;
            } else {
                steps += 1;
            }
            let a =
                component_follow::<CHECKED>(c, remap[tri[e] as usize] as usize, work, &mut steps)?;
            let b = component_follow::<CHECKED>(
                c,
                remap[tri[(e + 1) % 3] as usize] as usize,
                work,
                &mut steps,
            )?;
            if a != b {
                c[a.max(b)] = a.min(b) as u32;
            }
        }
    }
    if !CHECKED {
        work.add(steps)?;
    }
    Ok(())
}
#[inline(always)]
fn component_roots<const CHECKED: bool>(
    c: &mut [u32],
    remap: &[u32],
    start: usize,
    end: usize,
    work: &mut Work,
) -> Result<(), Error> {
    let mut steps = 0;
    for (offset, &r) in remap[start..end].iter().enumerate() {
        let i = start + offset;
        if CHECKED {
            work.add(1)?;
        } else {
            steps += 1;
        }
        if r as usize == i {
            c[i] = component_follow::<CHECKED>(c, i, work, &mut steps)? as u32;
        }
    }
    if !CHECKED {
        work.add(steps)?;
    }
    Ok(())
}
#[inline(always)]
fn component_number(c: &mut [u32], nc: &mut u32, i: usize, r: u32) {
    if r as usize == i {
        let root = c[i] as usize;
        c[i] = if root == i {
            let v = *nc;
            *nc += 1;
            v
        } else {
            c[root]
        };
    } else {
        c[i] = c[r as usize];
    }
}
pub(super) fn component_errors(
    p: &[V],
    c: &[u32],
    nc: usize,
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<Vec<f32>, Error> {
    let mut errors = budget.filled(checked_bytes(nc, 4)?, 0.0f32)?;
    // `errors` holds one [x, y, z, w] record per component.
    units!(work, p.len(), |m| {
        let records = errors.as_chunks_mut::<4>().0;
        for (i, v) in p.iter().enumerate() {
            m.add(1)?;
            let e = &mut records[c[i] as usize];
            e[0] += v.x;
            e[1] += v.y;
            e[2] += v.z;
            e[3] += 1.0;
        }
    });
    units!(work, nc, |m| {
        for j in (0..errors.len()).step_by(4) {
            m.add(1)?;
            let iw = if errors[j + 3] == 0.0 {
                0.0
            } else {
                1.0 / errors[j + 3]
            };
            errors[j] *= iw;
            errors[j + 1] *= iw;
            errors[j + 2] *= iw;
            errors[j + 3] = 0.0;
        }
    });
    units!(work, p.len(), |m| {
        let records = errors.as_chunks_mut::<4>().0;
        for (i, v) in p.iter().enumerate() {
            m.add(1)?;
            let e = &mut records[c[i] as usize];
            let dx = v.x - e[0];
            let dy = v.y - e[1];
            let dz = v.z - e[2];
            let r = dx * dx + dy * dy + dz * dz;
            if e[3] < r {
                e[3] = r;
            }
        }
    });
    units!(work, nc, |m| {
        for i in 0..nc {
            m.add(1)?;
            errors[i] = errors[i * 4 + 3];
        }
    });
    errors.truncate(nc);
    Ok(errors)
}
pub(super) fn prune(
    out: &mut [u32],
    count: usize,
    components: &[u32],
    errors: &[f32],
    cutoff: f32,
    work: &mut Work,
) -> Result<(usize, f32), Error> {
    let mut write_generic = 0;
    let mut next = f32::MAX;
    units!(work, (0..count).step_by(3).len(), |m| prune_triangles(
        out,
        count,
        components,
        errors,
        cutoff,
        (&mut write_generic, &mut next),
        m
    ))?;
    Ok((write_generic, next))
}
#[inline(always)]
fn prune_triangles<M: super::Meter>(
    out: &mut [u32],
    count: usize,
    components: &[u32],
    errors: &[f32],
    cutoff: f32,
    (write_generic, next): (&mut usize, &mut f32),
    m: &mut M,
) -> Result<(), Error> {
    for i in (0..count).step_by(3) {
        m.add(1)?;
        let error = errors[components[out[i] as usize] as usize];
        if error > cutoff {
            if *next > error {
                *next = error;
            }
            out.copy_within(i..i + 3, *write_generic);
            *write_generic += 3;
        }
    }
    Ok(())
}
/// Remove disconnected components whose bounding-sphere radius is at most the
/// relative error. Position welding is numerical, including signed zeros; unused positions affect bounds.
///
/// # Errors
/// `InvalidTopology` for incomplete triangles or `IndexOutOfBounds` for invalid vertex references.
/// `InvalidParameter` for invalid error/positions; `NumericalFailure` for non-finite geometry or calculations.
/// `SizeOverflow` for size overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed reservation.
#[inline]
pub fn simplify_prune_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<Vec<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        simplify_prune_u32(indices, positions, target_error, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn simplify_prune_u32(
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    Ok(simplify_prune_kernel(indices, positions, target_error, None, workspace)?.0)
}
fn simplify_prune_kernel(
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<(Vec<u32>, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, positions.len(), &mut work)?;
        if !target_error.is_finite() || target_error < 0.0 {
            return Err(Error::InvalidParameter);
        }
        let mut budget = Heap::new(workspace);
        let p = scaled(positions, &mut budget, &mut work)?;
        let mut remap = budget.filled(p.len(), 0u32)?;
        let mut table = budget.filled(hash_size(p.len())?, u32::MAX)?;
        if let Some(source) = positions.packed_values() {
            let mut start = 0;
            while start < remap.len() {
                let batch = work.batch(remap.len() - start, table.len() + 1)?;
                if batch == 0 {
                    let mut visits = 0;
                    weld_pass::<true>(
                        &mut remap[start..],
                        &mut table,
                        &|i| Ok(source[i]),
                        &mut work,
                        &mut visits,
                        start,
                    )?;
                    break;
                }
                let visits =
                    weld_packed(&mut remap[start..start + batch], &mut table, source, start);
                let settled = work.settle(visits);
                debug_assert!(settled);
                start += batch;
            }
        } else {
            weld(&mut remap, &mut table, |i| positions.at(i), &mut work)?;
        }
        budget.release(table);
        let (c, e) = components(&p, &remap, indices, &mut budget, &mut work)?;
        let mut owned = if destination.is_none() {
            budget.filled(indices.len(), 0u32)?
        } else {
            Vec::new()
        };
        let out = destination.unwrap_or(&mut owned);
        if out.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        let out = &mut out[..indices.len()];
        out.copy_from_slice(indices);
        let (count, _) = prune(
            out,
            indices.len(),
            &c,
            &e,
            target_error * target_error,
            &mut work,
        )?;
        owned.truncate(count);
        Ok((owned, count))
    })();
    workspace.finish(&work);
    result
}
/// Map each vertex to the first vertex with numerically equal coordinates.
/// One record charges one visit plus one per probe, at most `1 + table.len()`.
#[inline(always)]
fn weld(
    remap: &mut [u32],
    table: &mut [u32],
    position: impl Fn(usize) -> Result<[f32; 3], Error>,
    work: &mut Work,
) -> Result<(), Error> {
    // A tallied pass that fits would have charged the same visits, including
    // those before a failed read, without exhausting the budget. Otherwise the
    // probe table is reset and the checked pass replays the 0.2.14 charges;
    // `remap` is scratch that the caller discards on error.
    let mut visits = 0u128;
    let bound = remap.len() as u128 * (table.len() as u128 + 1);
    if work.covers_wide(bound) {
        let result = weld_pass::<false>(remap, table, &position, work, &mut visits, 0);
        let settled = work.settle(visits);
        debug_assert!(settled);
        return result;
    }
    table.fill(u32::MAX);
    weld_pass::<true>(remap, table, &position, work, &mut visits, 0)
}
/// Tallied weld of packed positions; returns the visits the checked pass
/// charges (one per vertex and one per probe). Table entries are earlier
/// vertices or `u32::MAX`, which is never a valid index because the vertex
/// count is at most `u32::MAX`; so the empty test and the position read share
/// one bounds test. `table.len() > n` holds and the triangular probe sequence
/// visits every slot of a power-of-two table, so each lookup ends at its match
/// or an empty slot within `table.len()` probes, as in the checked pass.
#[inline(always)]
fn weld_packed(remap: &mut [u32], table: &mut [u32], source: &[[f32; 3]], start: usize) -> u128 {
    let mask = table.len() - 1;
    // Same slice; its length makes `slot & mask` visibly in bounds.
    let table = &mut table[..=mask];
    let mut visits = 0u128;
    for (offset, (r, raw)) in remap.iter_mut().zip(&source[start..]).enumerate() {
        let i = start + offset;
        let h = raw.map(|v| {
            let x = if v == 0.0 { 0 } else { v.to_bits() };
            x ^ (x >> 17)
        });
        let hash =
            h[0].wrapping_mul(73856093) ^ h[1].wrapping_mul(19349663) ^ h[2].wrapping_mul(83492791);
        let mut slot = hash as usize & mask;
        let mut probes = table.len();
        for probe in 0..table.len() {
            let old = table[slot];
            let found = match source.get(old as usize) {
                None => {
                    table[slot] = i as u32;
                    true
                }
                Some(p) => p == raw,
            };
            if found {
                *r = table[slot];
                probes = probe + 1;
                break;
            }
            slot = (slot + probe + 1) & mask;
        }
        visits += 1 + probes as u128;
    }
    visits
}
#[inline(always)]
fn weld_pass<const CHECKED: bool>(
    remap: &mut [u32],
    table: &mut [u32],
    position: &impl Fn(usize) -> Result<[f32; 3], Error>,
    work: &mut Work,
    visits: &mut u128,
    start: usize,
) -> Result<(), Error> {
    let mask = table.len() - 1;
    for (offset, r) in remap.iter_mut().enumerate() {
        let i = start + offset;
        // One record visits at most 1 + table.len() < 2^35 times.
        let mut steps = 0u64;
        let result = (|| {
            if CHECKED {
                work.add(1)?;
            } else {
                steps += 1;
            }
            let raw = position(i)?;
            let h = raw.map(|v| {
                let x = if v == 0.0 { 0 } else { v.to_bits() };
                x ^ (x >> 17)
            });
            let hash = h[0].wrapping_mul(73856093)
                ^ h[1].wrapping_mul(19349663)
                ^ h[2].wrapping_mul(83492791);
            let mut slot = hash as usize & mask;
            for probe in 0..table.len() {
                if CHECKED {
                    work.add(1)?;
                } else {
                    steps += 1;
                }
                let old = table[slot];
                if old == u32::MAX || position(old as usize)? == raw {
                    if old == u32::MAX {
                        table[slot] = i as u32;
                    }
                    *r = table[slot];
                    break;
                }
                slot = (slot + probe + 1) & mask;
            }
            Ok(())
        })();
        *visits += u128::from(steps);
        result?;
    }
    Ok(())
}
fn hash_size(n: usize) -> Result<usize, Error> {
    n.checked_add(n / 4)
        .ok_or(Error::SizeOverflow)?
        .max(1)
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)
}
#[allow(clippy::too_many_arguments)]
pub(super) fn sparse_run(
    out: &mut [u32],
    mut input: Input<'_>,
    positions: Positions<'_>,
    attributes: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    mut settings: SimplifySettings,
    ws: &mut Workspace,
    work: &mut Work,
    output: usize,
    solve: bool,
) -> Result<(SimplifyResult, State, PositionBounds), Error> {
    let mut b = Budget::new(ws, output);
    b.reclaim_input(
        &mut input.original,
        checked_bytes(positions.len().div_ceil(8), 1),
    )?;
    let mut filter = b.filled(positions.len().div_ceil(8), 0u8)?;
    let mut unique = 0;
    for &i in input.as_slice() {
        work.add(1)?;
        let flag = 1u8 << (i % 8);
        if filter[i as usize / 8] & flag == 0 {
            unique += 1;
            filter[i as usize / 8] |= flag;
        }
    }
    b.release(filter);
    b.reclaim_input(&mut input.original, checked_bytes(unique, 4))?;
    let mut sparse = b.reserve::<u32>(unique)?;
    b.reclaim_input(
        &mut input.original,
        hash_size(unique).and_then(|n| checked_bytes(n, 4)),
    )?;
    let mut reverse = b.filled(hash_size(unique)?, u32::MAX)?;
    let topology_bytes = checked_bytes(input.as_slice().len(), 4);
    b.reclaim_input(&mut input.original, topology_bytes)?;
    let mut topology = b.vec::<u32>(input.as_slice().len())?;
    for (&i, r) in input.as_slice().iter().zip(&mut topology) {
        work.add(1)?;
        let mask = reverse.len() - 1;
        let mut slot = i.wrapping_mul(0x5bd1e995) as usize & mask;
        for probe in 0..=mask {
            work.add(1)?;
            let old = reverse[slot];
            if old == u32::MAX {
                reverse[slot] = sparse.len() as u32;
                sparse.push(i);
                *r = reverse[slot];
                break;
            }
            if sparse[old as usize] == i {
                *r = old;
                break;
            }
            slot = (slot + probe + 1) & mask;
        }
    }
    b.release(reverse);
    b.reclaim_input(
        &mut input.original,
        checked_bytes(
            if flags.is_some() { sparse.len() } else { 0 },
            core::mem::size_of::<VertexFlags>(),
        ),
    )?;
    let mut f = b.filled(
        if flags.is_some() { sparse.len() } else { 0 },
        VertexFlags::EMPTY,
    )?;
    if let Some(flags) = flags {
        for (k, &i) in sparse.iter().enumerate() {
            work.add(1)?;
            f[k] = flags[i as usize];
        }
    }
    let bytes = b.bytes;
    settings.options = SimplifyOptions(settings.options.bits() & !2);
    let (r, mut state, bounds) = run_state(
        out,
        Input {
            indices: &topology,
            original: input.original,
            update: false,
        },
        positions.mapped(&sparse),
        attributes.map(|a| a.mapped(&sparse)),
        weights,
        if flags.is_some() { Some(&f) } else { None },
        settings,
        b.ws,
        work,
        bytes,
        solve,
    )?;
    // Retain original-space indices for update finalization. The packed copies
    // and reverse table are dropped after this call; their peak stays accounted.
    for i in &mut out[..r.index_count] {
        work.add(1)?;
        *i = sparse[*i as usize];
    }
    state.sparse = sparse;
    Ok((r, state, bounds))
}
pub(super) fn fold_quadrics<M: super::Meter>(s: &mut State, work: &mut M) -> Result<(), Error> {
    for a in 0..s.p.len() {
        for edge in s.offsets.get(a)..s.offsets.get(a + 1) {
            work.add(1)?;
            let b = s.edges[edge].next as usize;
            let c = s.edges[edge].prev as usize;
            if s.remap.get(b) > s.remap.get(a) {
                continue;
            }
            let mut d = NONE;
            let mut v = b;
            loop {
                for e in s.offsets.get(v)..s.offsets.get(v + 1) {
                    work.add(1)?;
                    if s.remap.get(s.edges[e].next as usize) == s.remap.get(a) {
                        d = if d == NONE {
                            s.edges[e].prev as usize
                        } else {
                            b
                        };
                    }
                }
                v = s.wedge.get(v);
                if v == b {
                    break;
                }
            }
            if d == b || d == NONE {
                continue;
            }
            let p10 = s.p[b].sub(s.p[a]);
            let normal = p10.cross(s.p[c].sub(s.p[a]));
            let opposite = s.p[d].sub(s.p[a]).cross(p10);
            let nl = s.sqrt_cache.sqrt(normal.dot(normal));
            let ol = s.sqrt_cache.sqrt(opposite.dot(opposite));
            if normal.dot(opposite) >= -0.85 * nl * ol {
                continue;
            }
            let mut q = Q::edge(s.p[a], s.p[b], s.p[c], 1.0, &mut s.sqrt_cache);
            let qo = Q::edge(s.p[a], s.p[b], s.p[d], 1.0, &mut s.sqrt_cache);
            q.add(qo);
            s.q[s.remap.get(a)].add(q);
            s.q[s.remap.get(b)].add(q);
        }
    }
    Ok(())
}
fn quadric_solve(q: Q, g: G) -> Option<V> {
    let eps = 1e-6 * q.w;
    let d0 = q.a00;
    let l10 = q.a10 / d0;
    let l20 = q.a20 / d0;
    let d1 = q.a11 - q.a10 * l10;
    let dl21 = q.a21 - q.a20 * l10;
    let l21 = dl21 / d1;
    let d2 = q.a22 - q.a20 * l20 - dl21 * l21;
    let y0 = -q.b0;
    let y1 = -q.b1 - l10 * y0;
    let y2 = -q.b2 - l20 * y0 - l21 * y1;
    let z0 = y0 / d0;
    let z1 = y1 / d1;
    let z2 = y2 / d2;
    let l30 = g.x / d0;
    let dl31 = g.y - g.x * l10;
    let l31 = dl31 / d1;
    let dl32 = g.z - g.x * l20 - dl31 * l21;
    let l32 = dl32 / d2;
    let d3 = 0.0 - g.x * l30 - dl31 * l31 - dl32 * l32;
    let y3 = -g.w - l30 * y0 - l31 * y1 - l32 * y2;
    let z3 = if d3.abs() > eps { y3 / d3 } else { 0.0 };
    let z = z2 - l32 * z3;
    let y = z1 - l21 * z - l31 * z3;
    let x = z0 - l10 * y - l20 * z - l30 * z3;
    if d0.abs() > eps && d1.abs() > eps && d2.abs() > eps {
        Some(V { x, y, z })
    } else {
        None
    }
}
fn reduce(q: &mut Q, a: Q, g: &[G]) {
    q.a00 += a.a00 * q.w;
    q.a11 += a.a11 * q.w;
    q.a22 += a.a22 * q.w;
    q.a10 += a.a10 * q.w;
    q.a20 += a.a20 * q.w;
    q.a21 += a.a21 * q.w;
    q.b0 += a.b0 * q.w;
    q.b1 += a.b1 * q.w;
    q.b2 += a.b2 * q.w;
    let iw = if a.w == 0.0 { 0.0 } else { q.w / a.w };
    for g in g {
        q.a00 -= (g.x * g.x) * iw;
        q.a11 -= (g.y * g.y) * iw;
        q.a22 -= (g.z * g.z) * iw;
        q.a10 -= (g.x * g.y) * iw;
        q.a20 -= (g.x * g.z) * iw;
        q.a21 -= (g.y * g.z) * iw;
        q.b0 -= (g.x * g.w) * iw;
        q.b1 -= (g.y * g.w) * iw;
        q.b2 -= (g.z * g.w) * iw;
    }
}
pub(super) fn solve_state(s: &mut State, indices: &[u32], work: &mut Work) -> Result<(), Error> {
    work.add(s.locked.len())?;
    s.locked.fill(false);
    let charged = work.precharge(indices.len(), 1)?;
    for &v in indices {
        if !charged {
            work.add(1)?;
        }
        s.locked[v as usize] = true;
        s.locked[s.remap.get(v as usize)] = true;
    }
    s.adjacency::<true>(indices, work)?;
    // Canonical roots partition the wedge cycles. Each vertex contributes at
    // most four attribute visits per component; two edge scans visit at most
    // the full adjacency twice. Overflow keeps the original limited path.
    let bound =
        s.p.len()
            .checked_mul(2)
            .and_then(|n| s.g.len().checked_mul(4).and_then(|g| n.checked_add(g)))
            .and_then(|n| s.edges.len().checked_mul(2).and_then(|e| n.checked_add(e)));
    if bound.map_or(Ok(false), |n| work.covers(n))? {
        solve_records::<false>(s, work)
    } else {
        solve_records::<true>(s, work)
    }
}
#[inline(always)]
fn solve_visit<const LIMITED: bool>(
    work: &mut Work,
    visits: &mut usize,
    count: usize,
) -> Result<(), Error> {
    if LIMITED {
        work.add(count)
    } else {
        *visits += count;
        Ok(())
    }
}
fn solve_records<const LIMITED: bool>(s: &mut State, work: &mut Work) -> Result<(), Error> {
    let mut visits = 0;
    let result = (|| {
        for i in 0..s.p.len() {
            solve_visit::<LIMITED>(work, &mut visits, 1)?;
            if !s.locked[i] || !(s.kind[i] == MAN || s.kind[i] == COMPLEX) {
                continue;
            }
            if s.remap.get(i) != i {
                s.p[i] = s.p[s.remap.get(i)];
                continue;
            }
            let vp = s.p[i];
            let mut q = s.q[i];
            q.add(Q::point(vp, q.w * 1e-4));
            let mut gv = G::default();
            if s.ac > 0 {
                let mut v = i;
                loop {
                    solve_visit::<LIMITED>(work, &mut visits, s.ac)?;
                    reduce(&mut q, s.aq[v], &s.g[v * s.ac..(v + 1) * s.ac]);
                    v = s.wedge.get(v);
                    if v == i {
                        break;
                    }
                }
                if !s.vg.is_empty() {
                    gv = s.vg[i];
                }
            }
            let Some(p) = quadric_solve(q, gv) else {
                continue;
            };
            if ![p.x, p.y, p.z].iter().all(|x| x.is_finite()) {
                return Err(Error::NumericalFailure);
            }
            let mut nr = 0.0;
            for e in s.offsets.get(i)..s.offsets.get(i + 1) {
                solve_visit::<LIMITED>(work, &mut visits, 1)?;
                let edge = s.edges[e];
                let a = s.p[edge.next as usize].sub(vp);
                let b = s.p[edge.prev as usize].sub(vp);
                let da = a.dot(a);
                let db = b.dot(b);
                if nr < da {
                    nr = da;
                }
                if nr < db {
                    nr = db;
                }
            }
            let nr = s.sqrt_cache.sqrt(nr);
            let d = p.sub(vp);
            if d.dot(d) > nr * nr {
                continue;
            }
            let mut flip = false;
            for e in s.offsets.get(i)..s.offsets.get(i + 1) {
                solve_visit::<LIMITED>(work, &mut visits, 1)?;
                let edge = s.edges[e];
                let a = s.p[edge.next as usize];
                let b = s.p[edge.prev as usize];
                let ab = b.sub(a);
                let n0 = ab.cross(vp.sub(a));
                let n1 = ab.cross(p.sub(a));
                if n0.dot(n1) <= 0.25 * s.sqrt_cache.sqrt(n0.dot(n0) * n1.dot(n1)) {
                    flip = true;
                    break;
                }
            }
            if flip {
                continue;
            }
            if s.q[i].error(p) > s.q[i].error(vp) * 1.5 + 1e-6 {
                continue;
            }
            s.p[i] = p;
        }
        for i in 0..s.p.len() {
            solve_visit::<LIMITED>(work, &mut visits, 1)?;
            if !s.locked[i] || s.remap.get(i) != i {
                continue;
            }
            for k in 0..s.ac {
                solve_visit::<LIMITED>(work, &mut visits, 1)?;
                let mut shared = NONE;
                if s.kind[i] == COMPLEX || s.kind[i] == FRINGE {
                    shared = i;
                    let mut v = s.wedge.get(i);
                    while v != i {
                        solve_visit::<LIMITED>(work, &mut visits, 1)?;
                        if s.a[v * s.ac + k] != s.a[i * s.ac + k] {
                            shared = NONE;
                        } else if shared != NONE && s.aq[v].w > s.aq[shared].w {
                            shared = v;
                        }
                        v = s.wedge.get(v);
                    }
                }
                let mut v = i;
                loop {
                    solve_visit::<LIMITED>(work, &mut visits, 1)?;
                    let r = if shared == NONE { v } else { shared };
                    let p = s.p[i];
                    let a = s.aq[r];
                    let g = s.g[r * s.ac + k];
                    let iw = if a.w == 0.0 { 0.0 } else { 1.0 / a.w };
                    let value = (g.x * p.x + g.y * p.y + g.z * p.z + g.w) * iw;
                    if !value.is_finite() {
                        return Err(Error::NumericalFailure);
                    }
                    s.a[v * s.ac + k] = value;
                    v = s.wedge.get(v);
                    if v == i {
                        break;
                    }
                }
            }
        }
        Ok(())
    })();
    if !LIMITED {
        work.add(visits)?;
    }
    result
}
/// Destructively simplify indices and solve surviving positions and weighted
/// attributes. Late work or numerical failures may modify the index prefix;
/// position and attribute writes start only after solving completes. Unused
/// vertices, externally locked vertices and zero-weight attributes are preserved.
///
/// # Errors
/// Same errors as [`crate::simplify_with_attributes`]. `InvalidLayout` for incompatible mutable views.
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn simplify_with_update_generic<I: crate::Index>(
    indices: &mut [I],
    positions: &mut crate::PositionsMut<'_>,
    attributes: Option<&mut crate::AttributesMut<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    I::with_mut(indices, workspace, |indices, workspace| {
        simplify_with_update_u32(
            indices, positions, attributes, weights, flags, settings, workspace,
        )
    })
}

fn simplify_with_update_u32(
    indices: &mut [u32],
    positions: &mut crate::PositionsMut<'_>,
    mut attributes: Option<&mut crate::AttributesMut<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate(
            indices,
            positions.as_view(),
            attributes.as_ref().map(|a| a.as_view()),
            weights,
            flags,
            settings,
            &mut work,
        )?;
        workspace.acquire(crate::workspace::ScratchFamily::Simplify);
        let mut original = workspace
            .simplify
            .as_mut()
            .map_or_else(Vec::new, |s| core::mem::take(&mut s.original));
        let mut b = Budget::new(workspace, checked_bytes(original.capacity(), 4)?);
        b.reuse(&mut original, indices.len())?;
        work.add(indices.len())?;
        original.copy_from_slice(indices);
        let bytes = b.bytes;
        let (r, s, bounds) = run_state(
            indices,
            Input {
                indices: &[],
                original,
                update: true,
            },
            positions.as_view(),
            attributes.as_ref().map(|a| a.as_view()),
            weights,
            flags,
            settings,
            b.ws,
            &mut work,
            bytes,
            true,
        )?;
        // The normalization scan visited the same original-space positions
        // in this order (including the sparse map). Caller positions are still
        // untouched, and all those reads are in bounds. Keep every work visit,
        // including the limited failure prefix, without repeating the reads.
        work.scan(s.p.iter(), |_| Ok(()))?;
        let PositionBounds { lo, hi } = bounds;
        let mut extent = 0.0;
        for k in 0..3 {
            if hi[k] - lo[k] >= extent {
                extent = hi[k] - lo[k];
            }
        }
        for i in 0..s.p.len() {
            work.add(1)?;
            if !s.locked[i] {
                continue;
            }
            let source = if s.sparse.is_empty() {
                i
            } else {
                s.sparse[i] as usize
            };
            if flag(flags, source, VertexFlags::LOCK) {
                continue;
            }
            if s.kind[i] != LOCKED {
                let p = s.p[i];
                let value = [
                    p.x * extent + lo[0],
                    p.y * extent + lo[1],
                    p.z * extent + lo[2],
                ];
                if !value.iter().all(|x| x.is_finite()) {
                    return Err(Error::NumericalFailure);
                }
                positions.set(source, value);
            }
            if let Some(ref mut a) = attributes {
                let mut k = 0;
                for (j, &weight) in weights.iter().enumerate() {
                    if weight > 0.0 {
                        work.add(1)?;
                        a.set(source, j, s.a[i * s.ac + k] / weight);
                        k += 1;
                    }
                }
            }
        }
        s.retain(b.ws);
        Ok(r)
    })();
    workspace.finish(&work);
    result
}
#[inline(always)]
fn cell_hash(mut h: u32) -> u32 {
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1e995);
    h ^= h >> 15;
    h
}
/// Find the slot holding cell `id` or the first empty slot on its probe
/// sequence, with the number of probes (at most `table.len()`); `None` if the
/// sequence ends first. A table entry is a cell id, or with `MAPPED` a vertex
/// whose cell is `ids[entry]`.
#[inline(always)]
fn cell_slot<const MAPPED: bool>(table: &[u32], ids: &[u32], id: u32) -> Option<(usize, usize)> {
    let mask = table.len() - 1;
    let mut bucket = cell_hash(id) as usize & mask;
    for probe in 0..table.len() {
        let old = table[bucket];
        if old == u32::MAX || (if MAPPED { ids[old as usize] } else { old }) == id {
            return Some((bucket, probe + 1));
        }
        bucket = (bucket + probe + 1) & mask;
    }
    None
}
/// 0.2.14 accounting for one lookup: a covered lookup charges its probes once
/// found (or the whole table before failing); otherwise each probe is charged
/// before it is read.
#[inline(always)]
fn cell_slot_checked<const MAPPED: bool>(
    table: &[u32],
    ids: &[u32],
    id: u32,
    work: &mut Work,
) -> Result<usize, Error> {
    if work.covers(table.len())? {
        return match cell_slot::<MAPPED>(table, ids, id) {
            Some((slot, probes)) => {
                work.add(probes)?;
                Ok(slot)
            }
            None => {
                work.add(table.len())?;
                Err(Error::NumericalFailure)
            }
        };
    }
    let mask = table.len() - 1;
    let mut bucket = cell_hash(id) as usize & mask;
    for probe in 0..table.len() {
        work.add(1)?;
        let old = table[bucket];
        if old == u32::MAX || (if MAPPED { ids[old as usize] } else { old }) == id {
            return Ok(bucket);
        }
        bucket = (bucket + probe + 1) & mask;
    }
    Err(Error::NumericalFailure)
}
fn vertex_ids<P: Copy + Into<V>>(
    ids: &mut [u32],
    p: &[P],
    locks: Option<&[VertexFlags]>,
    grid: i32,
    work: &mut Work,
) -> Result<(), Error> {
    if locks.is_some() {
        vertex_ids_kernel::<true, P>(ids, p, locks, grid, work)
    } else {
        vertex_ids_kernel::<false, P>(ids, p, None, grid, work)
    }
}
#[inline(always)]
fn grid_integer(value: f32) -> u32 {
    // Adding 2^23 rounds to an integer in this bounded positive interval.
    // The mantissa gives that integer; correct a rounded-up result to floor.
    debug_assert!((0.5..1024.0).contains(&value));
    let rounded = value + 8388608.0;
    let integer = rounded.to_bits() & 0x7fffff;
    integer - u32::from(rounded - 8388608.0 > value)
}

fn vertex_ids_kernel<const LOCKS: bool, P: Copy + Into<V>>(
    ids: &mut [u32],
    p: &[P],
    locks: Option<&[VertexFlags]>,
    grid: i32,
    work: &mut Work,
) -> Result<(), Error> {
    let scale = (grid - 1) as f32;
    work.scan(ids.iter_mut().zip(p).enumerate(), |(i, (id, v))| {
        let v = (*v).into();
        let x = grid_integer(v.x * scale + 0.5);
        let y = grid_integer(v.y * scale + 0.5);
        let z = grid_integer(v.z * scale + 0.5);
        *id = if LOCKS && flag(locks, i, VertexFlags::LOCK) {
            (1 << 30) | i as u32
        } else {
            (x << 20) | (y << 10) | z
        };
        Ok(())
    })
}
fn triangle_count(ids: &[u32], indices: &[u32], work: &mut Work) -> Result<usize, Error> {
    let mut n = 0;
    let triangles = indices.as_chunks::<3>().0;
    let charged = work.precharge(triangles.len(), 1)?;
    for t in triangles {
        if !charged {
            work.add(1)?;
        }
        let a = ids[t[0] as usize];
        let b = ids[t[1] as usize];
        let c = ids[t[2] as usize];
        n += usize::from(a != b && a != c && b != c);
    }
    Ok(n)
}
#[inline(always)]
fn cells(
    table: &mut [u32],
    ids: &[u32],
    output: Option<&mut [u32]>,
    work: &mut Work,
) -> Result<usize, Error> {
    match output {
        Some(out) => cells_kernel::<true>(table, ids, out, work),
        None => cells_kernel::<false>(table, ids, &mut [], work),
    }
}
/// Count distinct cells; with `MAPPED`, also write each vertex's dense cell
/// index to `out`. Each vertex visits at most one full table plus its entry
/// charge. A batch runs with a local tally only after that complete bound fits;
/// otherwise the checked path preserves the original charge-before-visit order.
#[inline(always)]
fn cells_kernel<const MAPPED: bool>(
    table: &mut [u32],
    ids: &[u32],
    out: &mut [u32],
    work: &mut Work,
) -> Result<usize, Error> {
    work.add(table.len())?;
    table.fill(u32::MAX);
    let mut count = 0;
    let mut start = 0;
    while start < ids.len() {
        let batch = work.batch(ids.len() - start, table.len() + 1)?;
        if batch == 0 {
            for (offset, &id) in ids[start..].iter().enumerate() {
                work.add(1)?;
                let slot = cell_slot_checked::<MAPPED>(table, ids, id, work)?;
                cell_insert::<MAPPED>(table, out, &mut count, start + offset, id, slot);
            }
            break;
        }
        let mut visits = 0;
        for i in start..start + batch {
            visits += 1;
            let Some((slot, probes)) = cell_slot::<MAPPED>(table, ids, ids[i]) else {
                work.add(visits + table.len())?;
                return Err(Error::NumericalFailure);
            };
            visits += probes;
            cell_insert::<MAPPED>(table, out, &mut count, i, ids[i], slot);
        }
        work.add(visits)?;
        start += batch;
    }
    Ok(count)
}
#[inline(always)]
fn cell_insert<const MAPPED: bool>(
    table: &mut [u32],
    out: &mut [u32],
    count: &mut usize,
    i: usize,
    id: u32,
    slot: usize,
) {
    if MAPPED {
        if table[slot] == u32::MAX {
            table[slot] = i as u32;
            out[i] = *count as u32;
            *count += 1;
        } else {
            out[i] = out[table[slot] as usize];
        }
    } else {
        *count += usize::from(table[slot] == u32::MAX);
        table[slot] = id;
    }
}
#[inline(always)]
fn interpolate(y: f32, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let num = (y1 - y) * (x1 - x2) * (x1 - x0) * (y2 - y0);
    let den = (y2 - y) * (x1 - x2) * (y0 - y1) + (y0 - y) * (x1 - x0) * (y1 - y2);
    x1 + if den == 0.0 { 0.0 } else { num / den }
}

/// Grid-based sloppy triangle simplification. Only `LOCK` flags are accepted;
/// target error and returned error are relative to the full position extent.
///
/// # Errors
/// `InvalidTopology` for incomplete triangles or `IndexOutOfBounds` for invalid vertex references.
/// `InvalidLayout` for mismatched locks; `InvalidParameter` for invalid targets/error/positions; `UnknownFlags` for unsupported locks; `NumericalFailure` for non-finite geometry or calculations.
/// `SizeOverflow` for size overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed reservation.
#[inline]
pub fn simplify_sloppy_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifiedMeshGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        simplify_sloppy_u32(
            indices,
            positions,
            locks,
            target_index_count,
            target_error,
            workspace,
        )
        .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn simplify_sloppy_u32(
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifiedMeshGeneric, Error> {
    Ok(simplify_sloppy_kernel::<false>(
        indices,
        positions,
        locks,
        target_index_count,
        target_error,
        None,
        workspace,
    )?
    .0)
}
fn sloppy_triangles<const LIMITED: bool>(
    out: &mut [u32],
    triangles: &mut [u32],
    indices: &[[u32; 3]],
    vertex_cells: &[u32],
    remap: &[u32],
    count: &mut usize,
    work: &mut Work,
) -> Result<(), Error> {
    let mut visits = 0;
    for t in indices {
        if LIMITED {
            work.add(1)?;
        } else {
            visits += 1;
        }
        let c = [
            vertex_cells[t[0] as usize],
            vertex_cells[t[1] as usize],
            vertex_cells[t[2] as usize],
        ];
        if c[0] == c[1] || c[0] == c[2] || c[1] == c[2] {
            continue;
        }
        let mut tri = [
            remap[c[0] as usize],
            remap[c[1] as usize],
            remap[c[2] as usize],
        ];
        if tri[1] < tri[0] && tri[1] < tri[2] {
            tri = [tri[1], tri[2], tri[0]];
        } else if tri[2] < tri[0] && tri[2] < tri[1] {
            tri = [tri[2], tri[0], tri[1]];
        }
        out[*count * 3..*count * 3 + 3].copy_from_slice(&tri);
        let hash = tri[0].wrapping_mul(73856093)
            ^ tri[1].wrapping_mul(19349663)
            ^ tri[2].wrapping_mul(83492791);
        let mask = triangles.len() - 1;
        let mut slot = hash as usize & mask;
        for probe in 0..=mask {
            if LIMITED {
                work.add(1)?;
            } else {
                visits += 1;
            }
            let old = triangles[slot];
            if old == u32::MAX {
                triangles[slot] = *count as u32;
                *count += 1;
                break;
            }
            if out[old as usize * 3..old as usize * 3 + 3] == tri {
                break;
            }
            slot = (slot + probe + 1) & mask;
        }
    }
    if !LIMITED {
        work.add(visits)?;
    }
    Ok(())
}
// The sloppy accumulation phase needs eleven floats. After accumulation,
// reuse the weight slot for its exact reciprocal; no later addition occurs.
#[derive(Clone, Copy, Default)]
struct GridQuadric {
    values: [f32; 11],
}
impl GridQuadric {
    #[inline(always)]
    fn add(&mut self, q: Q) {
        let values = [
            q.a00, q.a11, q.a22, q.a10, q.a20, q.a21, q.b0, q.b1, q.b2, q.c, q.w,
        ];
        for (a, b) in self.values.iter_mut().zip(values) {
            *a += b;
        }
    }
    fn cache_inverse_weight(&mut self) {
        let w = &mut self.values[10];
        *w = if *w == 0.0 { 0.0 } else { 1.0 / *w };
    }
    #[inline(always)]
    fn error(self, p: V) -> f32 {
        let v = self.values;
        let q = Q {
            a00: v[0],
            a11: v[1],
            a22: v[2],
            a10: v[3],
            a20: v[4],
            a21: v[5],
            b0: v[6],
            b1: v[7],
            b2: v[8],
            c: v[9],
            w: 0.0,
            inverse_weight: v[10],
        };
        q.error(p)
    }
}
pub(crate) struct SloppyScratch {
    p: Vec<V>,
    ids: Vec<u32>,
    table: Vec<u32>,
    vertex_cells: Vec<u32>,
    q: Vec<GridQuadric>,
    remap: Vec<u32>,
    errors: Vec<f32>,
    triangles: Vec<u32>,
    cache: SqrtCache,
}
impl Default for SloppyScratch {
    fn default() -> Self {
        Self {
            p: Vec::new(),
            ids: Vec::new(),
            table: Vec::new(),
            vertex_cells: Vec::new(),
            q: Vec::new(),
            remap: Vec::new(),
            errors: Vec::new(),
            triangles: Vec::new(),
            cache: SqrtCache::new(),
        }
    }
}
impl SloppyScratch {
    pub(crate) fn capacity_bytes(&self) -> Result<usize, Error> {
        let fields = [
            (self.p.capacity(), core::mem::size_of::<V>()),
            (self.ids.capacity(), 4),
            (self.table.capacity(), 4),
            (self.vertex_cells.capacity(), 4),
            (self.q.capacity(), core::mem::size_of::<GridQuadric>()),
            (self.remap.capacity(), 4),
            (self.errors.capacity(), 4),
            (self.triangles.capacity(), 4),
        ];
        fields.into_iter().try_fold(0usize, |sum, (n, size)| {
            sum.checked_add(checked_bytes(n, size)?)
                .ok_or(Error::SizeOverflow)
        })
    }
}
impl core::fmt::Debug for SloppyScratch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SloppyScratch")
            .field("heap_bytes", &self.capacity_bytes())
            .finish()
    }
}
fn simplify_sloppy_kernel<const CALLER: bool>(
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<(SimplifiedMeshGeneric, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, positions.len(), &mut work)?;
        validate_vertex_flags(locks, positions.len())?;
        if let Some(flags) = locks {
            work.scan(flags.iter(), |f| {
                if f.bits() & !1 != 0 {
                    Err(Error::UnknownFlags)
                } else {
                    Ok(())
                }
            })?;
        }
        if target_index_count > indices.len() || !target_error.is_finite() || target_error < 0.0 {
            return Err(Error::InvalidParameter);
        }
        if locks.is_some() && positions.len() > 1 << 30 {
            return Err(Error::SizeOverflow);
        }
        let (lo, extent) = counted_bounds(positions, &mut work)?;
        let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
        if !scale.is_finite() {
            return Err(Error::NumericalFailure);
        }
        let mut scratch = workspace.sloppy.take().unwrap_or_default();
        let projected = (|| {
            let added_positions = checked_bytes(
                positions.len().saturating_sub(scratch.p.capacity()),
                core::mem::size_of::<V>(),
            )?;
            let added_ids =
                checked_bytes(positions.len().saturating_sub(scratch.ids.capacity()), 4)?;
            let projected = scratch
                .capacity_bytes()?
                .checked_add(checked_bytes(if CALLER { 0 } else { indices.len() }, 4)?)
                .and_then(|n| n.checked_add(added_positions))
                .and_then(|n| n.checked_add(added_ids))
                .ok_or(Error::SizeOverflow)?;
            Ok::<_, Error>(projected)
        })();
        if projected.is_ok_and(|bytes| bytes > workspace.limits().max_bytes) {
            scratch = SloppyScratch::default();
        }
        let mut budget = Heap::with_bytes(workspace, scratch.capacity_bytes()?);
        let SloppyScratch {
            mut p,
            mut ids,
            mut table,
            mut vertex_cells,
            mut q,
            mut remap,
            mut errors,
            mut triangles,
            mut cache,
        } = scratch;
        // No previous-call contents are live. Later pressure may reclaim
        // either dormant allocations or excess capacity of initialized arrays.
        p.clear();
        ids.clear();
        table.clear();
        vertex_cells.clear();
        q.clear();
        remap.clear();
        errors.clear();
        triangles.clear();
        macro_rules! reclaim {
            ($added:expr) => {{
                let requested = $added
                    .and_then(|added| budget.bytes().checked_add(added).ok_or(Error::SizeOverflow));
                if requested.is_ok_and(|bytes| bytes > budget.ws.limits().max_bytes) {
                    let mut released = 0;
                    macro_rules! shrink {
                        ($v:ident, $size:expr) => {{
                            let before = checked_bytes($v.capacity(), $size)?;
                            crate::workspace::shrink(&mut $v, true)?;
                            released += before - checked_bytes($v.capacity(), $size)?;
                        }};
                    }
                    shrink!(p, core::mem::size_of::<V>());
                    shrink!(ids, 4);
                    shrink!(table, 4);
                    shrink!(vertex_cells, 4);
                    shrink!(q, core::mem::size_of::<GridQuadric>());
                    shrink!(remap, 4);
                    shrink!(errors, 4);
                    shrink!(triangles, 4);
                    let bytes = budget.bytes() - released;
                    budget = Heap::with_bytes(budget.ws, bytes);
                }
            }};
        }
        reclaim!(checked_bytes(
            positions.len().saturating_sub(p.capacity()),
            core::mem::size_of::<V>()
        ));
        budget.reuse(&mut p, positions.len(), V::default())?;
        scale_values(positions, &mut p, lo, scale, &mut work)?;
        reclaim!(checked_bytes((p.len()).saturating_sub(ids.capacity()), 4));
        budget.reuse(&mut ids, p.len(), 0u32)?;
        let mut min_grid = (1.0 / target_error.clamp(1e-3, 1.0)) as i32;
        let mut max_grid = 1025;
        let mut min_triangles = 0;
        let mut max_triangles = indices.len() / 3;
        if min_grid > 1 || locks.is_some() {
            vertex_ids(&mut ids, &p, locks, min_grid, &mut work)?;
            min_triangles = triangle_count(&ids, indices, &mut work)?;
        }
        let mut next = (sqrt((target_index_count / 6) as f32) + 0.5) as i32;
        for pass in 0..15 {
            if min_triangles >= target_index_count / 3 || max_grid - min_grid <= 1 {
                break;
            }
            let grid = next.clamp(min_grid + 1, max_grid - 1);
            vertex_ids(&mut ids, &p, locks, grid, &mut work)?;
            let triangles = triangle_count(&ids, indices, &mut work)?;
            let tip = interpolate(
                (target_index_count / 3) as f32,
                min_grid as f32,
                min_triangles as f32,
                grid as f32,
                triangles as f32,
                max_grid as f32,
                max_triangles as f32,
            );
            if triangles <= target_index_count / 3 {
                min_grid = grid;
                min_triangles = triangles;
            } else {
                max_grid = grid;
                max_triangles = triangles;
            }
            next = if pass < 5 {
                (tip + 0.5) as i32
            } else {
                (min_grid + max_grid) / 2
            };
        }
        if min_triangles == 0 {
            budget.ws.has_retained = true;
            budget.ws.sloppy = Some(SloppyScratch {
                p,
                ids,
                table,
                vertex_cells,
                q,
                remap,
                errors,
                triangles,
                cache,
            });
            return Ok((
                SimplifiedMeshGeneric {
                    indices: Vec::new(),
                    error: 1.0,
                },
                0,
            ));
        }
        reclaim!(checked_bytes(
            (hash_size(p.len())?).saturating_sub(table.capacity()),
            4
        ));
        budget.reuse(&mut table, hash_size(p.len())?, u32::MAX)?;
        reclaim!(checked_bytes(
            (p.len()).saturating_sub(vertex_cells.capacity()),
            4
        ));
        budget.reuse(&mut vertex_cells, p.len(), 0u32)?;
        vertex_ids(&mut ids, &p, locks, min_grid, &mut work)?;
        let nc = cells(&mut table, &ids, Some(&mut vertex_cells), &mut work)?;
        reclaim!(checked_bytes(
            (nc).saturating_sub(q.capacity()),
            core::mem::size_of::<GridQuadric>()
        ));
        budget.reuse(&mut q, nc, GridQuadric::default())?;
        q.fill(GridQuadric::default());
        let source_triangles = indices.as_chunks::<3>().0;
        let charged = work.precharge(source_triangles.len(), 1)?;
        for t in source_triangles {
            if !charged {
                work.add(1)?;
            }
            let [a, b, c] = [
                vertex_cells[t[0] as usize] as usize,
                vertex_cells[t[1] as usize] as usize,
                vertex_cells[t[2] as usize] as usize,
            ];
            let single = a == b && a == c;
            let qt = Q::triangle(
                p[t[0] as usize],
                p[t[1] as usize],
                p[t[2] as usize],
                if single { 3.0 } else { 1.0 },
                &mut cache,
            );
            q[a].add(qt);
            if !single {
                q[b].add(qt);
                q[c].add(qt);
            }
        }
        work.scan(q.iter_mut(), |q| {
            // Normalized coordinates are in [0, 2]; triangle coefficients
            // have magnitude at most 2048. f32 accumulation of such terms
            // stays below 2^36, and nonzero weight is at least 2^-38.
            // Positional error is therefore below 2^82, including rounding.
            q.cache_inverse_weight();
            Ok(())
        })?;
        reclaim!(checked_bytes((nc).saturating_sub(remap.capacity()), 4));
        budget.reuse(&mut remap, nc, u32::MAX)?;
        remap.fill(u32::MAX);
        reclaim!(checked_bytes((nc).saturating_sub(errors.capacity()), 4));
        budget.reuse(&mut errors, nc, 0.0f32)?;
        let charged = work.precharge(p.len(), 1)?;
        for (i, &v) in p.iter().enumerate() {
            if !charged {
                work.add(1)?;
            }
            let c = vertex_cells[i] as usize;
            let e = q[c].error(v);
            if remap[c] == u32::MAX || errors[c] > e {
                remap[c] = i as u32;
                errors[c] = e;
            }
        }
        let mut error = 0.0;
        work.scan(errors.iter().copied(), |e| {
            if error < e {
                error = e;
            }
            Ok(())
        })?;
        reclaim!(checked_bytes(
            (hash_size(min_triangles)?).saturating_sub(triangles.capacity()),
            4
        ));
        budget.reuse(&mut triangles, hash_size(min_triangles)?, u32::MAX)?;
        triangles.fill(u32::MAX);
        if !CALLER {
            reclaim!(checked_bytes(indices.len(), 4));
        }
        let mut owned = if !CALLER {
            budget.filled(indices.len(), 0u32)?
        } else {
            Vec::new()
        };
        let out = destination.unwrap_or(&mut owned);
        if out.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        let out = &mut out[..indices.len()];
        let mut count = 0;
        let mut source = indices.as_chunks::<3>().0;
        while !source.is_empty() {
            let batch = work.batch(source.len(), triangles.len() + 1)?;
            if batch == 0 {
                sloppy_triangles::<true>(
                    out,
                    &mut triangles,
                    source,
                    &vertex_cells,
                    &remap,
                    &mut count,
                    &mut work,
                )?;
                break;
            }
            sloppy_triangles::<false>(
                out,
                &mut triangles,
                &source[..batch],
                &vertex_cells,
                &remap,
                &mut count,
                &mut work,
            )?;
            source = &source[batch..];
        }
        owned.truncate(count * 3);
        budget.ws.has_retained = true;
        budget.ws.sloppy = Some(SloppyScratch {
            p,
            ids,
            table,
            vertex_cells,
            q,
            remap,
            errors,
            triangles,
            cache,
        });
        Ok((
            SimplifiedMeshGeneric {
                indices: owned,
                error: sqrt(error),
            },
            count * 3,
        ))
    })();
    workspace.finish(&work);
    result
}
#[inline(always)]
fn color_at(colors: &Option<Positions<'_>>, i: usize) -> Result<[f32; 3], Error> {
    if let Some(c) = colors {
        c.at(i)
    } else {
        Ok([0.0; 3])
    }
}
/// Simplify points to original vertex indices using grid reservoirs. Optional
/// colors are checked three-component views; all supplied values must be finite.
///
/// # Errors
/// `InvalidLayout` for mismatched colors; `InvalidParameter` for invalid target, weight or positions;
/// `NumericalFailure` for non-finite geometry or calculations.
/// `SizeOverflow` for size overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed reservation.
pub fn simplify_points_generic<I: crate::Index>(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<I>, Error> {
    simplify_points_u32(
        positions,
        colors,
        color_weight,
        target_vertex_count,
        workspace,
    )
    .and_then(|value| I::from_vec(value, workspace))
}

fn simplify_points_u32(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    Ok(simplify_points_kernel(
        positions,
        colors,
        color_weight,
        target_vertex_count,
        None,
        workspace,
    )?
    .0)
}
fn simplify_points_kernel(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<(Vec<u32>, usize), Error> {
    if let Some(p) = colors.and_then(Positions::packed_values) {
        simplify_points_kernel_impl::<true, 7>(
            positions,
            colors,
            color_weight,
            target_vertex_count,
            destination,
            workspace,
            |i| Ok(p[i]),
        )
    } else if colors.is_some() {
        simplify_points_kernel_impl::<true, 7>(
            positions,
            colors,
            color_weight,
            target_vertex_count,
            destination,
            workspace,
            |i| color_at(&colors, i),
        )
    } else {
        simplify_points_kernel_impl::<false, 4>(
            positions,
            colors,
            color_weight,
            target_vertex_count,
            destination,
            workspace,
            |_| Ok([0.0; 3]),
        )
    }
}
fn simplify_points_kernel_impl<const COLORS: bool, const RESERVOIR: usize>(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
    color: impl Fn(usize) -> Result<[f32; 3], Error>,
) -> Result<(Vec<u32>, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if positions.len() > u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        if target_vertex_count > positions.len() || !color_weight.is_finite() {
            return Err(Error::InvalidParameter);
        }
        if colors.is_some_and(|c| c.len() != positions.len()) {
            return Err(Error::InvalidLayout);
        }
        if let Some(c) = colors {
            c.for_each_counted(&mut work, |v| {
                if v.iter().all(|x| x.is_finite()) {
                    Ok(())
                } else {
                    Err(Error::InvalidParameter)
                }
            })?;
        }
        if target_vertex_count == 0 {
            positions.for_each_counted(&mut work, |v| {
                if v.iter().all(|x| x.is_finite()) {
                    Ok(())
                } else {
                    Err(Error::InvalidParameter)
                }
            })?;
            workspace.prepare([0; 4], 0)?;
            return Ok((Vec::new(), 0));
        }
        let n = positions.len();
        let table_len = hash_size(n)?;
        let integer_len = n
            .checked_mul(2)
            .and_then(|v| v.checked_add(table_len))
            .ok_or(Error::SizeOverflow)?;
        let position_len = n.checked_mul(3).ok_or(Error::SizeOverflow)?;
        let (lo, extent) = counted_bounds(positions, &mut work)?;
        let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
        if !scale.is_finite() {
            return Err(Error::NumericalFailure);
        }
        workspace.prepare([integer_len, position_len, 0, 0], 0)?;
        let p = workspace.floats[..position_len].as_chunks_mut::<3>().0;
        if let Some(source) = positions.packed_values() {
            work.scan(source.iter().zip(&mut *p), |(a, v)| {
                *v = [
                    (a[0] - lo[0]) * scale,
                    (a[1] - lo[1]) * scale,
                    (a[2] - lo[2]) * scale,
                ];
                Ok(())
            })?;
        } else {
            let mut at = 0;
            positions.for_each_counted(&mut work, |a| {
                let value = [
                    (a[0] - lo[0]) * scale,
                    (a[1] - lo[1]) * scale,
                    (a[2] - lo[2]) * scale,
                ];
                p[at] = value;
                at += 1;
                Ok(())
            })?;
        }
        let (ids, rest) = workspace.integers[..integer_len].split_at_mut(n);
        let (table, _) = rest.split_at_mut(table_len);
        let mut min_grid = 0;
        let mut max_grid = 1025;
        let mut min_vertices = 0;
        let mut max_vertices = n;
        let mut next = (sqrt(target_vertex_count as f32) + 0.5) as i32;
        for pass in 0..15 {
            let grid = next.clamp(min_grid + 1, max_grid - 1);
            vertex_ids(ids, p, None, grid, &mut work)?;
            let count = cells(table, ids, None, &mut work)?;
            let tip = interpolate(
                target_vertex_count as f32,
                min_grid as f32,
                min_vertices as f32,
                grid as f32,
                count as f32,
                max_grid as f32,
                max_vertices as f32,
            );
            if count <= target_vertex_count {
                min_grid = grid;
                min_vertices = count;
            } else {
                max_grid = grid;
                max_vertices = count;
            }
            if count == target_vertex_count || max_grid - min_grid <= 1 {
                break;
            }
            next = if pass < 5 {
                (tip + 0.5) as i32
            } else {
                (min_grid + max_grid) / 2
            };
        }
        if min_vertices == 0 {
            return Ok((Vec::new(), 0));
        }
        let weight = color_weight
            * if min_grid == 1 {
                1.0
            } else {
                1.0 / (min_grid - 1) as f32
            };
        let weight = weight * weight;
        if min_vertices == n {
            if !weight.is_finite() {
                return Err(Error::NumericalFailure);
            }
            let output_bytes = if destination.is_none() {
                checked_bytes(n, 4)?
            } else {
                0
            };
            workspace.prepare([integer_len, position_len, 0, 0], output_bytes)?;
            let mut owned = if destination.is_none() {
                crate::workspace::output(n)?
            } else {
                Vec::new()
            };
            if destination.is_none() && owned.capacity() != n {
                workspace.prepare(
                    [integer_len, position_len, 0, 0],
                    checked_bytes(owned.capacity(), 4)?,
                )?;
            }
            let out = destination.unwrap_or(&mut owned);
            if out.len() < n {
                return Err(Error::BufferTooSmall);
            }
            work.scan(out[..n].iter_mut().enumerate(), |(i, value)| {
                *value = i as u32;
                Ok(())
            })?;
            return Ok((owned, n));
        }
        // Grow only after the grid search; the normalized prefix is retained.
        let float_len = min_vertices
            .checked_mul(RESERVOIR + 1)
            .and_then(|v| v.checked_add(position_len))
            .ok_or(Error::SizeOverflow)?;
        let out_bytes = if destination.is_none() {
            checked_bytes(min_vertices, 4)?
        } else {
            0
        };
        workspace.prepare([integer_len, float_len, 0, 0], out_bytes)?;
        let mut owned = if destination.is_none() {
            crate::workspace::output(min_vertices)?
        } else {
            Vec::new()
        };
        if owned.capacity() != min_vertices && destination.is_none() {
            workspace.prepare(
                [integer_len, float_len, 0, 0],
                checked_bytes(owned.capacity(), 4)?,
            )?;
        }
        let p = workspace.floats[..position_len].as_chunks::<3>().0;
        let (ids, rest) = workspace.integers[..integer_len].split_at_mut(n);
        let (table, vc) = rest.split_at_mut(table_len);
        vertex_ids(ids, p, None, min_grid, &mut work)?;
        let nc = cells(table, ids, Some(vc), &mut work)?;
        debug_assert_eq!(nc, min_vertices);
        let (positions_data, rest) = workspace.floats[..float_len].split_at_mut(position_len);
        let p = positions_data.as_chunks::<3>().0;
        let (reservoir_data, errors) = rest.split_at_mut(nc * RESERVOIR);
        let reservoirs = reservoir_data.as_chunks_mut::<RESERVOIR>().0;
        reservoirs.fill([0.0; RESERVOIR]);
        if COLORS {
            work.scan(p.iter().enumerate(), |(i, v)| {
                let r = &mut reservoirs[vc[i] as usize];
                let color = color(i)?;
                r[0] += v[0];
                r[1] += v[1];
                r[2] += v[2];
                r[3] += color[0];
                r[4] += color[1];
                r[5] += color[2];
                r[RESERVOIR - 1] += 1.0;
                Ok(())
            })?;
        } else {
            // Without colors no visit can fail, so a fitting scan is prepaid.
            units!(&mut work, p.len(), |m| {
                for (v, &c) in p.iter().zip(&vc[..p.len()]) {
                    m.add(1)?;
                    let r = &mut reservoirs[c as usize];
                    r[0] += v[0];
                    r[1] += v[1];
                    r[2] += v[2];
                    r[RESERVOIR - 1] += 1.0;
                }
            });
        }
        work.scan(reservoirs.iter_mut(), |r| {
            let iw = if r[RESERVOIR - 1] == 0.0 {
                0.0
            } else {
                1.0 / r[RESERVOIR - 1]
            };
            for value in &mut r[..if COLORS { 6 } else { 3 }] {
                *value *= iw;
            }
            Ok(())
        })?;
        let remap = destination.unwrap_or(&mut owned);
        if remap.len() < nc {
            return Err(Error::BufferTooSmall);
        }
        let remap = &mut remap[..nc];
        remap.fill(u32::MAX);
        if !COLORS && !weight.is_finite() {
            return Err(Error::NumericalFailure);
        }
        if COLORS {
            work.scan(p.iter().enumerate(), |(i, v)| {
                let c = vc[i] as usize;
                let r = reservoirs[c];
                let color = color(i)?;
                let pe = (v[0] - r[0]) * (v[0] - r[0])
                    + (v[1] - r[1]) * (v[1] - r[1])
                    + (v[2] - r[2]) * (v[2] - r[2]);
                let ce = (color[0] - r[3]) * (color[0] - r[3])
                    + (color[1] - r[4]) * (color[1] - r[4])
                    + (color[2] - r[5]) * (color[2] - r[5]);
                let e = pe + weight * ce;
                if !e.is_finite() {
                    return Err(Error::NumericalFailure);
                }
                if remap[c] == u32::MAX || errors[c] > e {
                    remap[c] = i as u32;
                    errors[c] = e;
                }
                Ok(())
            })?;
        } else {
            // Normalized coordinates and their cell means are finite and
            // bounded (PROOFS.md, "Uncolored point errors"), so `e` is finite
            // and no visit can fail: a fitting scan is prepaid.
            units!(&mut work, p.len(), |m| {
                for (i, (v, &c)) in p.iter().zip(&vc[..p.len()]).enumerate() {
                    m.add(1)?;
                    let c = c as usize;
                    let r = reservoirs[c];
                    let e = (v[0] - r[0]) * (v[0] - r[0])
                        + (v[1] - r[1]) * (v[1] - r[1])
                        + (v[2] - r[2]) * (v[2] - r[2]);
                    if remap[c] == u32::MAX || errors[c] > e {
                        remap[c] = i as u32;
                        errors[c] = e;
                    }
                }
            });
        }
        Ok((owned, nc))
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer sloppy simplification. Requires input-length capacity; work exhaustion may partially modify output.
///
/// # Errors
/// Same errors as [`simplify_sloppy`]. `BufferTooSmall` if the destination cannot hold the required prefix.
#[inline]
pub fn simplify_sloppy_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            simplify_sloppy_into_u32(
                destination,
                indices,
                positions,
                locks,
                target_index_count,
                target_error,
                workspace,
            )
        })
    })
}

fn simplify_sloppy_into_u32(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    if destination.len() < indices.len() {
        workspace.begin();
        return Err(Error::BufferTooSmall);
    }
    let (out, count) = simplify_sloppy_kernel::<true>(
        indices,
        positions,
        locks,
        target_index_count,
        target_error,
        Some(destination),
        workspace,
    )?;
    Ok(SimplifyResult {
        index_count: count,
        error: out.error,
    })
}
/// Caller-buffer component pruning. Requires input-length capacity; work exhaustion may partially modify output.
///
/// # Errors
/// Same errors as [`simplify_prune`]. `BufferTooSmall` if the destination cannot hold the required prefix.
#[inline]
pub fn simplify_prune_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            simplify_prune_into_u32(destination, indices, positions, target_error, workspace)
        })
    })
}

fn simplify_prune_into_u32(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if destination.len() < indices.len() {
        workspace.begin();
        return Err(Error::BufferTooSmall);
    }
    Ok(simplify_prune_kernel(
        indices,
        positions,
        target_error,
        Some(destination),
        workspace,
    )?
    .1)
}
/// Caller-buffer point reduction. Requires target-count capacity; work exhaustion may partially modify output.
///
/// # Errors
/// Same errors as [`simplify_points`]. `BufferTooSmall` if the destination cannot hold the required prefix.
#[inline]
pub fn simplify_points_into_generic<I: crate::Index>(
    destination: &mut [I],
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    I::with_mut(destination, workspace, |destination, workspace| {
        simplify_points_into_u32(
            destination,
            positions,
            colors,
            color_weight,
            target_vertex_count,
            workspace,
        )
    })
}

fn simplify_points_into_u32(
    destination: &mut [u32],
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if destination.len() < target_vertex_count {
        workspace.begin();
        return Err(Error::BufferTooSmall);
    }
    Ok(simplify_points_kernel(
        positions,
        colors,
        color_weight,
        target_vertex_count,
        Some(destination),
        workspace,
    )?
    .1)
}

/// The source-compatible u32 entry point; see [`simplify_prune_generic`] for selectable index widths.
#[inline]
pub fn simplify_prune(
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    simplify_prune_generic::<u32>(indices, positions, target_error, workspace)
}

/// The source-compatible u32 entry point; see [`simplify_with_update_generic`] for selectable index widths.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn simplify_with_update(
    indices: &mut [u32],
    positions: &mut crate::PositionsMut<'_>,
    attributes: Option<&mut crate::AttributesMut<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    simplify_with_update_generic::<u32>(
        indices, positions, attributes, weights, flags, settings, workspace,
    )
}

/// The source-compatible u32 entry point; see [`simplify_sloppy_generic`] for selectable index widths.
#[inline]
pub fn simplify_sloppy(
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    simplify_sloppy_generic::<u32>(
        indices,
        positions,
        locks,
        target_index_count,
        target_error,
        workspace,
    )
    .map(Into::into)
}

/// The source-compatible u32 entry point; see [`simplify_points_generic`] for selectable index widths.
#[inline]
pub fn simplify_points(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    simplify_points_generic::<u32>(
        positions,
        colors,
        color_weight,
        target_vertex_count,
        workspace,
    )
}

/// The source-compatible u32 entry point; see [`simplify_sloppy_into_generic`] for selectable index widths.
#[inline]
pub fn simplify_sloppy_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    simplify_sloppy_into_generic::<u32>(
        destination,
        indices,
        positions,
        locks,
        target_index_count,
        target_error,
        workspace,
    )
}

/// The source-compatible u32 entry point; see [`simplify_prune_into_generic`] for selectable index widths.
#[inline]
pub fn simplify_prune_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    simplify_prune_into_generic::<u32>(destination, indices, positions, target_error, workspace)
}

/// The source-compatible u32 entry point; see [`simplify_points_into_generic`] for selectable index widths.
#[inline]
pub fn simplify_points_into(
    destination: &mut [u32],
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    simplify_points_into_generic::<u32>(
        destination,
        positions,
        colors,
        color_weight,
        target_vertex_count,
        workspace,
    )
}

#[cfg(test)]
mod solve_tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn accumulated_solve_preserves_numerical_failure_work_and_position_prefix() {
        fn run<const LIMITED: bool>(fail: usize) -> (Result<(), Error>, Vec<u32>, u64) {
            let mut ws = Workspace::new(Limits::default());
            let mut s = State::new(3, 0, 0, &mut ws, 0, false, &mut Vec::new()).unwrap();
            s.p.extend([V::default(); 3]);
            s.kind.fill(MAN);
            s.locked.fill(true);
            s.remap.0.extend([0, 1, 2]);
            s.wedge.0.extend([0, 1, 2]);
            for i in 0..3 {
                s.q[i].a00 = 1.;
                s.q[i].a11 = 1.;
                s.q[i].a22 = 1.;
            }
            s.q[fail].b0 = f32::INFINITY;
            let mut work = ws.begin();
            let result = solve_records::<LIMITED>(&mut s, &mut work);
            ws.finish(&work);
            (
                result,
                s.p.iter()
                    .flat_map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
                    .collect(),
                ws.usage().work,
            )
        }
        for fail in 0..3 {
            let expected = run::<true>(fail);
            assert_eq!(expected.0, Err(Error::NumericalFailure));
            assert_eq!(expected.2, (fail + 1) as u64);
            assert_eq!(run::<false>(fail), expected);
        }
    }
}

#[cfg(test)]
mod bounds_tests {
    use super::*;

    #[test]
    fn packed_bounds_match_ordered_selects_including_zero_signs() {
        fn ordered(values: &[[f32; 3]]) -> Option<([u32; 3], [u32; 3])> {
            let mut lo = [f32::MAX; 3];
            let mut hi = [-f32::MAX; 3];
            for v in values {
                for j in 0..3 {
                    if !v[j].is_finite() {
                        return None;
                    }
                    if lo[j] > v[j] {
                        lo[j] = v[j];
                    }
                    if hi[j] < v[j] {
                        hi[j] = v[j];
                    }
                }
            }
            Some((lo.map(f32::to_bits), hi.map(f32::to_bits)))
        }
        // Single-sign pools make a signed zero the extreme; the last pool
        // reaches the non-finite exit.
        let pools: [&[f32]; 5] = [
            &[0.0, -0.0, 0.5, 1.0],
            &[0.0, -0.0, -0.5, -1.0],
            &[0.0, -0.0],
            &[0.0, -0.0, 0.5, -0.5, f32::MAX, -f32::MAX, f32::MIN_POSITIVE],
            &[0.0, -0.0, 1.0, f32::INFINITY, f32::NEG_INFINITY, f32::NAN],
        ];
        let mut state = 0x9e37_79b9u32;
        for n in 0..30 {
            for round in 0..250 {
                let pool = pools[round % pools.len()];
                let values: Vec<[f32; 3]> = (0..n)
                    .map(|_| {
                        core::array::from_fn(|_| {
                            state ^= state << 13;
                            state ^= state >> 17;
                            state ^= state << 5;
                            pool[state as usize % pool.len()]
                        })
                    })
                    .collect();
                let got = packed_bounds(&values)
                    .map(|(lo, hi)| (lo.map(f32::to_bits), hi.map(f32::to_bits)));
                assert_eq!(got, ordered(&values), "{values:?}");
            }
        }
    }
}
