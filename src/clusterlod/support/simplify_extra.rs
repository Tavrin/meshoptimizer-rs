//! Grid simplification and connected-component pruning from simplifier.cpp.
use super::*;
use crate::clusterlod::support::budget::Budget as Heap;

fn scaled(
    positions: Positions<'_>,
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<Vec<V>, Error> {
    work.add(positions.len())?;
    let (lo, extent) = bounds(positions)?;
    let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
    let mut p = budget.filled(positions.len(), V::default())?;
    for (i, v) in p.iter_mut().enumerate() {
        work.add(1)?;
        let a = positions.at(i)?;
        *v = V {
            x: (a[0] - lo[0]) * scale,
            y: (a[1] - lo[1]) * scale,
            z: (a[2] - lo[2]) * scale,
        };
        if ![v.x, v.y, v.z].iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
    }
    Ok(p)
}
pub(super) fn components(
    p: &[V],
    remap: &[u32],
    indices: &[u32],
    budget: &mut Heap<'_>,
    work: &mut Work,
) -> Result<(Vec<u32>, Vec<f32>), Error> {
    let n = p.len();
    let mut c = budget.filled(n, 0u32)?;
    for (i, r) in c.iter_mut().enumerate() {
        work.add(1)?;
        *r = i as u32;
    }
    fn follow(c: &mut [u32], mut i: usize, work: &mut Work) -> Result<usize, Error> {
        while i != c[i] as usize {
            work.add(1)?;
            let parent = c[i] as usize;
            c[i] = c[parent];
            i = parent;
        }
        Ok(i)
    }
    for tri in indices.as_chunks::<3>().0 {
        for e in 0..3 {
            work.add(1)?;
            let a = follow(&mut c, remap[tri[e] as usize] as usize, work)?;
            let b = follow(&mut c, remap[tri[(e + 1) % 3] as usize] as usize, work)?;
            if a != b {
                c[a.max(b)] = a.min(b) as u32;
            }
        }
    }
    for (i, &r) in remap.iter().enumerate() {
        work.add(1)?;
        if r as usize == i {
            c[i] = follow(&mut c, i, work)? as u32;
        }
    }
    let mut nc = 0;
    for (i, &r) in remap.iter().enumerate() {
        work.add(1)?;
        if r as usize == i {
            let root = c[i] as usize;
            c[i] = if root == i {
                let v = nc;
                nc += 1;
                v
            } else {
                c[root]
            };
        } else {
            c[i] = c[r as usize];
        }
    }
    let mut errors = budget.filled(checked_bytes(nc as usize, 4)?, 0.0f32)?;
    for (i, v) in p.iter().enumerate() {
        work.add(1)?;
        let j = c[i] as usize * 4;
        errors[j] += v.x;
        errors[j + 1] += v.y;
        errors[j + 2] += v.z;
        errors[j + 3] += 1.0;
    }
    for j in (0..errors.len()).step_by(4) {
        work.add(1)?;
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
    for (i, v) in p.iter().enumerate() {
        work.add(1)?;
        let j = c[i] as usize * 4;
        let dx = v.x - errors[j];
        let dy = v.y - errors[j + 1];
        let dz = v.z - errors[j + 2];
        let r = dx * dx + dy * dy + dz * dz;
        if errors[j + 3] < r {
            errors[j + 3] = r;
        }
    }
    for i in 0..nc as usize {
        work.add(1)?;
        errors[i] = errors[i * 4 + 3];
    }
    errors.truncate(nc as usize);
    Ok((c, errors))
}
pub(super) fn prune(
    out: &mut [u32],
    count: usize,
    components: &[u32],
    errors: &[f32],
    cutoff: f32,
    work: &mut Work,
) -> Result<(usize, f32), Error> {
    let mut write = 0;
    let mut next = f32::MAX;
    for i in (0..count).step_by(3) {
        work.add(1)?;
        let error = errors[components[out[i] as usize] as usize];
        if error > cutoff {
            if next > error {
                next = error;
            }
            out.copy_within(i..i + 3, write);
            write += 3;
        }
    }
    Ok((write, next))
}
/// Remove disconnected components whose bounding-sphere radius is at most the
/// relative error. Position welding is numerical, including signed zeros; unused positions affect bounds.
pub fn simplify_prune(
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
        for (i, r) in remap.iter_mut().enumerate() {
            work.add(1)?;
            let raw = positions.at(i)?;
            let h = raw.map(|v| {
                let x = if v == 0.0 { 0 } else { v.to_bits() };
                x ^ (x >> 17)
            });
            let hash = h[0].wrapping_mul(73856093)
                ^ h[1].wrapping_mul(19349663)
                ^ h[2].wrapping_mul(83492791);
            let mut slot = hash as usize & (table.len() - 1);
            for probe in 0..table.len() {
                work.add(1)?;
                let old = table[slot];
                if old == u32::MAX || positions.at(old as usize)? == raw {
                    if old == u32::MAX {
                        table[slot] = i as u32;
                    }
                    *r = table[slot];
                    break;
                }
                slot = (slot + probe + 1) & (table.len() - 1);
            }
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
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    mut settings: SimplifySettings,
    ws: &mut Workspace,
    work: &mut Work,
    output: usize,
    solve: bool,
) -> Result<(SimplifyResult, State), Error> {
    let mut b = Budget { ws, bytes: output };
    let mut filter = b.filled(positions.len().div_ceil(8), 0u8)?;
    let mut unique = 0;
    for &i in indices {
        work.add(1)?;
        let flag = 1u8 << (i % 8);
        if filter[i as usize / 8] & flag == 0 {
            unique += 1;
            filter[i as usize / 8] |= flag;
        }
    }
    b.release(filter);
    let mut sparse = b.reserve::<u32>(unique)?;
    let mut reverse = b.filled(hash_size(unique)?, u32::MAX)?;
    let mut topology = b.vec::<u32>(indices.len())?;
    for (&i, r) in indices.iter().zip(&mut topology) {
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
    let (r, mut state) = run_state(
        out,
        &topology,
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
    Ok((r, state))
}
pub(super) fn fold_quadrics(s: &mut State, work: &mut Work) -> Result<(), Error> {
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
    for &v in indices {
        work.add(1)?;
        s.locked[v as usize] = true;
        s.locked[s.remap.get(v as usize)] = true;
    }
    s.adjacency(indices, true, work)?;
    for i in 0..s.p.len() {
        work.add(1)?;
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
                work.add(s.ac)?;
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
            work.add(1)?;
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
            work.add(1)?;
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
        work.add(1)?;
        if !s.locked[i] || s.remap.get(i) != i {
            continue;
        }
        for k in 0..s.ac {
            work.add(1)?;
            let mut shared = NONE;
            if s.kind[i] == COMPLEX || s.kind[i] == FRINGE {
                shared = i;
                let mut v = s.wedge.get(i);
                while v != i {
                    work.add(1)?;
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
                work.add(1)?;
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
}
/// Destructively simplify indices and solve surviving positions and weighted
/// attributes. Late work or numerical failures may modify the index prefix;
/// position and attribute writes start only after solving completes. Unused
/// vertices, externally locked vertices and zero-weight attributes are preserved.
#[allow(clippy::too_many_arguments)]
pub fn simplify_with_update(
    indices: &mut [u32],
    positions: &mut crate::clusterlod::support::PositionsMut<'_>,
    mut attributes: Option<&mut crate::clusterlod::support::AttributesMut<'_>>,
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
        let mut b = Budget {
            ws: workspace,
            bytes: 0,
        };
        let mut original = b.vec::<u32>(indices.len())?;
        work.add(indices.len())?;
        original.copy_from_slice(indices);
        let bytes = b.bytes;
        let (r, s) = run_state(
            indices,
            &original,
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
        let mut lo = [f32::MAX; 3];
        let mut hi = [-f32::MAX; 3];
        for i in 0..s.p.len() {
            work.add(1)?;
            let source = if s.sparse.is_empty() {
                i
            } else {
                s.sparse[i] as usize
            };
            let p = positions.as_view().at(source)?;
            for k in 0..3 {
                if lo[k] > p[k] {
                    lo[k] = p[k];
                }
                if hi[k] < p[k] {
                    hi[k] = p[k];
                }
            }
        }
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
#[inline(always)]
fn cell_slot(
    table: &[u32],
    ids: Option<&[u32]>,
    key: u32,
    work: &mut Work,
) -> Result<usize, Error> {
    let id = ids.map_or(key, |v| v[key as usize]);
    let mask = table.len() - 1;
    let mut bucket = cell_hash(id) as usize & mask;
    for probe in 0..=mask {
        work.add(1)?;
        let old = table[bucket];
        if old == u32::MAX || ids.map_or(old, |v| v[old as usize]) == id {
            return Ok(bucket);
        }
        bucket = (bucket + probe + 1) & mask;
    }
    Err(Error::NumericalFailure)
}
fn vertex_ids(
    ids: &mut [u32],
    p: &[V],
    locks: Option<&[VertexFlags]>,
    grid: i32,
    work: &mut Work,
) -> Result<(), Error> {
    if locks.is_some() {
        vertex_ids_kernel::<true>(ids, p, locks, grid, work)
    } else {
        vertex_ids_kernel::<false>(ids, p, None, grid, work)
    }
}
fn vertex_ids_kernel<const LOCKS: bool>(
    ids: &mut [u32],
    p: &[V],
    locks: Option<&[VertexFlags]>,
    grid: i32,
    work: &mut Work,
) -> Result<(), Error> {
    let scale = (grid - 1) as f32;
    work.scan(ids.iter_mut().zip(p).enumerate(), |(i, (id, v))| {
        let x = (v.x * scale + 0.5) as i32 as u32;
        let y = (v.y * scale + 0.5) as i32 as u32;
        let z = (v.z * scale + 0.5) as i32 as u32;
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
    for t in indices.as_chunks::<3>().0 {
        work.add(1)?;
        let [a, b, c] = t.map(|i| ids[i as usize]);
        n += usize::from(a != b && a != c && b != c);
    }
    Ok(n)
}
#[inline(always)]
fn cells(
    table: &mut [u32],
    ids: &[u32],
    mut output: Option<&mut [u32]>,
    work: &mut Work,
) -> Result<usize, Error> {
    work.add(table.len())?;
    table.fill(u32::MAX);
    let mut count = 0;
    for (i, &id) in ids.iter().enumerate() {
        work.add(1)?;
        let key = if output.is_some() { i as u32 } else { id };
        let slot = cell_slot(
            table,
            if output.is_some() { Some(ids) } else { None },
            key,
            work,
        )?;
        if let Some(ref mut out) = output {
            if table[slot] == u32::MAX {
                table[slot] = i as u32;
                out[i] = count as u32;
                count += 1;
            } else {
                out[i] = out[table[slot] as usize];
            }
        } else {
            count += usize::from(table[slot] == u32::MAX);
            table[slot] = id;
        }
    }
    Ok(count)
}
#[inline(always)]
fn interpolate(y: f32, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let num = (y1 - y) * (x1 - x2) * (x1 - x0) * (y2 - y0);
    let den = (y2 - y) * (x1 - x2) * (y0 - y1) + (y0 - y) * (x1 - x0) * (y1 - y2);
    x1 + if den == 0.0 { 0.0 } else { num / den }
}

/// Grid-based sloppy triangle simplification. Only `LOCK` flags are accepted;
/// target error and returned error are relative to the full position extent.
pub fn simplify_sloppy(
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    Ok(simplify_sloppy_kernel(
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
fn simplify_sloppy_kernel(
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<(SimplifiedMesh, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, positions.len(), &mut work)?;
        validate_vertex_flags(locks, positions.len())?;
        if locks.is_some_and(|v| v.iter().any(|f| f.bits() & !1 != 0)) {
            return Err(Error::UnknownFlags);
        }
        if target_index_count > indices.len() || !target_error.is_finite() || target_error < 0.0 {
            return Err(Error::InvalidParameter);
        }
        if locks.is_some() && positions.len() > 1 << 30 {
            return Err(Error::SizeOverflow);
        }
        let mut budget = Heap::new(workspace);
        let p = scaled(positions, &mut budget, &mut work)?;
        let mut ids = budget.filled(p.len(), 0u32)?;
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
            return Ok((
                SimplifiedMesh {
                    indices: Vec::new(),
                    error: 1.0,
                },
                0,
            ));
        }
        let mut table = budget.filled(hash_size(p.len())?, u32::MAX)?;
        let mut vertex_cells = budget.filled(p.len(), 0u32)?;
        vertex_ids(&mut ids, &p, locks, min_grid, &mut work)?;
        let nc = cells(&mut table, &ids, Some(&mut vertex_cells), &mut work)?;
        let mut q = budget.filled(nc, Q::default())?;
        let mut cache = crate::clusterlod::support::math::SqrtCache::new();
        for t in indices.as_chunks::<3>().0 {
            work.add(1)?;
            let [a, b, c] = t.map(|i| vertex_cells[i as usize] as usize);
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
        for q in &mut q {
            q.cache_inverse_weight();
            if !q.finite() {
                return Err(Error::NumericalFailure);
            }
        }
        let mut remap = budget.filled(nc, u32::MAX)?;
        let mut errors = budget.filled(nc, 0.0f32)?;
        for (i, &v) in p.iter().enumerate() {
            work.add(1)?;
            let c = vertex_cells[i] as usize;
            let e = q[c].error(v);
            if !e.is_finite() {
                return Err(Error::NumericalFailure);
            }
            if remap[c] == u32::MAX || errors[c] > e {
                remap[c] = i as u32;
                errors[c] = e;
            }
        }
        let mut error = 0.0;
        for &e in &errors {
            work.add(1)?;
            if error < e {
                error = e;
            }
        }
        let mut triangles = budget.filled(hash_size(min_triangles)?, u32::MAX)?;
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
        let mut count = 0;
        for t in indices.as_chunks::<3>().0 {
            work.add(1)?;
            let c = t.map(|i| vertex_cells[i as usize]);
            if c[0] == c[1] || c[0] == c[2] || c[1] == c[2] {
                continue;
            }
            let mut tri = c.map(|i| remap[i as usize]);
            if tri[1] < tri[0] && tri[1] < tri[2] {
                tri.rotate_left(1);
            } else if tri[2] < tri[0] && tri[2] < tri[1] {
                tri.rotate_right(1);
            }
            out[count * 3..count * 3 + 3].copy_from_slice(&tri);
            let hash = tri[0].wrapping_mul(73856093)
                ^ tri[1].wrapping_mul(19349663)
                ^ tri[2].wrapping_mul(83492791);
            let mask = triangles.len() - 1;
            let mut slot = hash as usize & mask;
            for probe in 0..=mask {
                work.add(1)?;
                let old = triangles[slot];
                if old == u32::MAX {
                    triangles[slot] = count as u32;
                    count += 1;
                    break;
                }
                if out[old as usize * 3..old as usize * 3 + 3] == tri {
                    break;
                }
                slot = (slot + probe + 1) & mask;
            }
        }
        owned.truncate(count * 3);
        Ok((
            SimplifiedMesh {
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
pub fn simplify_points(
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
    if colors.is_some() {
        simplify_points_kernel_impl::<true>(
            positions,
            colors,
            color_weight,
            target_vertex_count,
            destination,
            workspace,
        )
    } else {
        simplify_points_kernel_impl::<false>(
            positions,
            None,
            color_weight,
            target_vertex_count,
            destination,
            workspace,
        )
    }
}
fn simplify_points_kernel_impl<const COLORS: bool>(
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
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
            work.add(c.len())?;
            c.for_each(|v| {
                if v.iter().all(|x| x.is_finite()) {
                    Ok(())
                } else {
                    Err(Error::InvalidParameter)
                }
            })?;
        }
        let mut budget = Heap::new(workspace);
        let p = scaled(positions, &mut budget, &mut work)?;
        if target_vertex_count == 0 {
            return Ok((Vec::new(), 0));
        }
        let mut ids = budget.filled(p.len(), 0u32)?;
        let mut table = budget.filled(hash_size(p.len())?, u32::MAX)?;
        let mut min_grid = 0;
        let mut max_grid = 1025;
        let mut min_vertices = 0;
        let mut max_vertices = p.len();
        let mut next = (sqrt(target_vertex_count as f32) + 0.5) as i32;
        for pass in 0..15 {
            let grid = next.clamp(min_grid + 1, max_grid - 1);
            vertex_ids(&mut ids, &p, None, grid, &mut work)?;
            let count = cells(&mut table, &ids, None, &mut work)?;
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
        let mut vc = budget.filled(p.len(), 0u32)?;
        vertex_ids(&mut ids, &p, None, min_grid, &mut work)?;
        let nc = cells(&mut table, &ids, Some(&mut vc), &mut work)?;
        let mut reservoirs = budget.filled(nc, [0.0f32; 7])?;
        work.scan(p.iter().enumerate(), |(i, v)| {
            let r = &mut reservoirs[vc[i] as usize];
            let color = if COLORS {
                color_at(&colors, i)?
            } else {
                [0.0; 3]
            };
            r[0] += v.x;
            r[1] += v.y;
            r[2] += v.z;
            r[3] += color[0];
            r[4] += color[1];
            r[5] += color[2];
            r[6] += 1.0;
            Ok(())
        })?;
        for r in &mut reservoirs {
            work.add(1)?;
            let iw = if r[6] == 0.0 { 0.0 } else { 1.0 / r[6] };
            for value in &mut r[..6] {
                *value *= iw;
            }
        }
        let mut owned = if destination.is_none() {
            budget.filled(nc, u32::MAX)?
        } else {
            Vec::new()
        };
        let remap = destination.unwrap_or(&mut owned);
        if remap.len() < nc {
            return Err(Error::BufferTooSmall);
        }
        let remap = &mut remap[..nc];
        remap.fill(u32::MAX);
        let mut errors = budget.filled(nc, 0.0f32)?;
        let weight = color_weight
            * if min_grid == 1 {
                1.0
            } else {
                1.0 / (min_grid - 1) as f32
            };
        let weight = weight * weight;
        work.scan(p.iter().enumerate(), |(i, v)| {
            let c = vc[i] as usize;
            let r = reservoirs[c];
            let color = if COLORS {
                color_at(&colors, i)?
            } else {
                [0.0; 3]
            };
            let pe = (v.x - r[0]) * (v.x - r[0])
                + (v.y - r[1]) * (v.y - r[1])
                + (v.z - r[2]) * (v.z - r[2]);
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
        Ok((owned, nc))
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer sloppy simplification. Requires input-length capacity; work exhaustion may partially modify output.
pub fn simplify_sloppy_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    locks: Option<&[VertexFlags]>,
    target_index_count: usize,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    if destination.len() < indices.len() {
        return Err(Error::BufferTooSmall);
    }
    let (out, count) = simplify_sloppy_kernel(
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
pub fn simplify_prune_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    target_error: f32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if destination.len() < indices.len() {
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
pub fn simplify_points_into(
    destination: &mut [u32],
    positions: Positions<'_>,
    colors: Option<Positions<'_>>,
    color_weight: f32,
    target_vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if destination.len() < target_vertex_count {
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
