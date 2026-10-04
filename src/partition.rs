// Port of meshoptimizer 1.3 partition.cpp (MIT, Arseny Kapoulkine).
use crate::processing::{dot, finite, point, sub, Context};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;
/// Cluster-to-partition assignments in upstream order.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Partitions {
    /// Partition id for each input cluster.
    pub assignments: Vec<u32>,
    /// Number of partitions.
    pub count: usize,
}
#[derive(Clone, Copy, Default)]
struct Group {
    group: i32,
    next: i32,
    size: u32,
    vertices: u32,
    center: [f32; 3],
    radius: f32,
}
#[derive(Clone, Copy, Default)]
struct Order {
    id: u32,
    order: i32,
}
fn push(heap: &mut [Order], n: &mut usize, item: Order) {
    let mut i = *n;
    heap[i] = item;
    *n += 1;
    while i > 0 && heap[i].order < heap[(i - 1) / 2].order {
        let p = (i - 1) / 2;
        heap.swap(i, p);
        i = p;
    }
}
fn pop(heap: &mut [Order], n: &mut usize) -> Order {
    let top = heap[0];
    *n -= 1;
    heap[0] = heap[*n];
    let mut i = 0;
    while i * 2 + 1 < *n {
        let mut j = i * 2 + 1;
        if j + 1 < *n && heap[j + 1].order < heap[j].order {
            j += 1;
        }
        if heap[j].order >= heap[i].order {
            break;
        }
        heap.swap(i, j);
        i = j;
    }
    top
}
fn score(a: Group, b: Group, ctx: &mut Context<'_>) -> Result<f32, Error> {
    let dp = sub(b.center, a.center);
    let d = ctx.sqrt(finite(dot(dp, dp))?);
    let mr = if d + a.radius < b.radius {
        b.radius
    } else if d + b.radius < a.radius {
        a.radius
    } else {
        (d + b.radius + a.radius) / 2.
    };
    finite(if mr > 0. { a.radius / mr } else { 0. })
}
fn merge(a: &mut Group, b: Group, ctx: &mut Context<'_>) -> Result<(), Error> {
    let dp = sub(b.center, a.center);
    let d = ctx.sqrt(finite(dot(dp, dp))?);
    if d + a.radius < b.radius {
        a.center = b.center;
        a.radius = b.radius;
    } else if d + b.radius > a.radius {
        let k = if d > 0. {
            (d + b.radius - a.radius) / (2. * d)
        } else {
            0.
        };
        for (j, &v) in dp.iter().enumerate() {
            a.center[j] = finite(a.center[j] + v * k)?;
        }
        a.radius = finite((d + b.radius + a.radius) / 2.)?;
    }
    Ok(())
}
fn leaf(
    groups: &mut [Group],
    order: &[u32],
    target: usize,
    max: usize,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    for &id in order {
        let id = id as usize;
        if groups[id].size == 0 || groups[id].size as usize >= target {
            continue;
        }
        let mut best = None;
        let mut bestscore = -1.;
        for &other in order {
            ctx.tick(1)?;
            let other = other as usize;
            if id == other
                || groups[other].size == 0
                || (groups[id].size as usize + groups[other].size as usize) > max
            {
                continue;
            }
            let sc = score(groups[id], groups[other], ctx)?;
            if sc > bestscore {
                best = Some(other);
                bestscore = sc;
            }
        }
        if let Some(other) = best {
            let mut tail = other;
            while groups[tail].next >= 0 {
                ctx.tick(1)?;
                tail = groups[tail].next as usize;
            }
            groups[tail].next = id as i32;
            groups[other].size += groups[id].size;
            groups[id].size = 0;
            let g = groups[id];
            merge(&mut groups[other], g, ctx)?;
            groups[id].radius = 0.;
        }
    }
    Ok(())
}
fn spatial(
    groups: &mut [Group],
    order: &mut [u32],
    target: usize,
    max: usize,
    depth: usize,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    ctx.tick(order.len())?;
    let total = order
        .iter()
        .map(|&v| groups[v as usize].size as usize)
        .sum::<usize>();
    if total <= max || order.len() <= 8 {
        return leaf(groups, order, target, max, ctx);
    }
    let mut mean = [0.; 3];
    let mut vars = [0.; 3];
    let mut runc = 1.;
    let mut runs = 1.;
    ctx.tick(order.len() * 2)?;
    for &v in order.iter() {
        let p = groups[v as usize].center;
        for k in 0..3 {
            let delta = p[k] - mean[k];
            mean[k] += delta * runs;
            vars[k] += delta * (p[k] - mean[k]);
        }
        runc += 1.;
        runs = 1. / runc;
    }
    let axis = if vars[0] >= vars[1] && vars[0] >= vars[2] {
        0
    } else if vars[1] >= vars[2] {
        1
    } else {
        2
    };
    let mut middle = 0;
    for i in 0..order.len() {
        let v = groups[order[i] as usize].center[axis];
        order.swap(middle, i);
        middle += usize::from(v < mean[axis]);
    }
    if middle <= 4 || order.len() - middle <= 4 || depth >= 40 {
        middle = order.len() / 2;
    }
    let (left, right) = order.split_at_mut(middle);
    spatial(groups, left, target, max, depth + 1, ctx)?;
    spatial(groups, right, target, max, depth + 1, ctx)
}
struct Adj {
    offsets: Vec<u32>,
    clusters: Vec<u32>,
    shared: Vec<u32>,
}
fn adjacent(
    indices: &[u32],
    offsets: &[u32],
    n: usize,
    vertices: usize,
    ctx: &mut Context<'_>,
) -> Result<Adj, Error> {
    let mut refs = ctx.alloc::<u32>(vertices + 1)?;
    ctx.tick(indices.len())?;
    for &v in indices {
        refs[v as usize] += 1;
    }
    let mut total = 0usize;
    for i in 0..n {
        let mut count = 0usize;
        // Batched visit charges (ctx.scan) keep each per-visit exhaustion point.
        ctx.scan(
            indices[offsets[i] as usize..offsets[i + 1] as usize].iter(),
            |_, &v| {
                count = count
                    .checked_add(refs[v as usize] as usize - 1)
                    .ok_or(Error::SizeOverflow)?;
                Ok(())
            },
        )?;
        total = total
            .checked_add(count.min(n - 1))
            .ok_or(Error::SizeOverflow)?;
    }
    if total > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    let mut adj = Adj {
        offsets: ctx.alloc(n + 1)?,
        clusters: ctx.alloc(total)?,
        shared: ctx.alloc(total)?,
    };
    let mut nr = 0;
    for r in &mut refs[..vertices] {
        let n = *r;
        *r = nr;
        nr += n;
    }
    let mut data = ctx.alloc::<u32>(nr as usize)?;
    for i in 0..n {
        ctx.scan(
            indices[offsets[i] as usize..offsets[i + 1] as usize].iter(),
            |_, &v| {
                let r = &mut refs[v as usize];
                data[*r as usize] = i as u32;
                *r += 1;
                Ok(())
            },
        )?;
    }
    refs.copy_within(0..vertices, 1);
    refs[0] = 0;
    let mut slots = ctx.alloc::<u32>(n)?;
    slots.fill(u32::MAX);
    for i in 0..n {
        let start = adj.offsets[i] as usize;
        let mut count = 0;
        for &v in &indices[offsets[i] as usize..offsets[i + 1] as usize] {
            let others = &data[refs[v as usize] as usize..refs[v as usize + 1] as usize];
            // These visits cannot fail: charge a covered list once, otherwise
            // keep the per-visit exhaustion point.
            let batched = ctx.work.covers(others.len())?;
            for &c in others {
                if !batched {
                    ctx.tick(1)?;
                }
                if c as usize == i {
                    continue;
                }
                let slot = slots[c as usize] as usize;
                if slot < count && adj.clusters[start + slot] == c {
                    adj.shared[start + slot] += 1;
                } else {
                    slots[c as usize] = count as u32;
                    adj.clusters[start + count] = c;
                    adj.shared[start + count] = 1;
                    count += 1;
                }
            }
            if batched {
                ctx.tick(others.len())?;
            }
        }
        adj.offsets[i + 1] = (start + count) as u32;
    }
    ctx.release(&refs)?;
    ctx.release(&data)?;
    ctx.release(&slots)?;
    Ok(adj)
}
fn pick(
    groups: &[Group],
    id: usize,
    adj: &Adj,
    max: usize,
    bounds: bool,
    acc: &mut [u32],
    ctx: &mut Context<'_>,
) -> Result<Option<(usize, u32)>, Error> {
    let r = 1. / ctx.sqrt(groups[id].vertices as i32 as f32);
    let mut ci = id as i32;
    while ci >= 0 {
        let c = ci as usize;
        ctx.tick((adj.offsets[c + 1] - adj.offsets[c]) as usize)?;
        for i in adj.offsets[c] as usize..adj.offsets[c + 1] as usize {
            let other = groups[adj.clusters[i] as usize].group;
            if other >= 0 {
                acc[other as usize] = acc[other as usize]
                    .checked_add(adj.shared[i])
                    .ok_or(Error::SizeOverflow)?;
            }
        }
        ci = groups[c].next;
    }
    let mut best = None;
    let mut bestscore = 0.;
    ci = id as i32;
    while ci >= 0 {
        let c = ci as usize;
        ctx.tick((adj.offsets[c + 1] - adj.offsets[c]) as usize)?;
        for i in adj.offsets[c] as usize..adj.offsets[c + 1] as usize {
            let other = groups[adj.clusters[i] as usize].group;
            if other < 0 || acc[other as usize] == 0 {
                continue;
            }
            let other = other as usize;
            let shared = acc[other];
            acc[other] = 0;
            if groups[id].size as usize + groups[other].size as usize > max {
                continue;
            }
            let mut sc =
                shared as i32 as f32 * (r + 1. / ctx.sqrt(groups[other].vertices as i32 as f32));
            if bounds {
                sc *= 1. + 0.4 * score(groups[id], groups[other], ctx)?;
            }
            finite(sc)?;
            if sc > bestscore {
                best = Some((other, shared));
                bestscore = sc;
            }
        }
        ci = groups[c].next;
    }
    Ok(best)
}
#[inline(always)]
fn checked<const CHECK: bool>(x: f32) -> Result<f32, Error> {
    if CHECK {
        finite(x)
    } else {
        Ok(x)
    }
}
// Cluster centroid and squared radius, in upstream operation order.
#[inline(always)]
fn group_sphere<const CHECK: bool>(
    p: Positions<'_>,
    verts: &[u32],
) -> Result<([f32; 3], f32), Error> {
    let mut center = [0f32; 3];
    for &v in verts {
        let pp = point(p, v);
        for (k, &x) in pp.iter().enumerate() {
            center[k] = checked::<CHECK>(center[k] + x)?;
        }
    }
    for x in &mut center {
        *x /= verts.len() as f32;
    }
    let mut r = 0.;
    for &v in verts {
        let dp = sub(point(p, v), center);
        let d = checked::<CHECK>(dot(dp, dp))?;
        if r < d {
            r = d;
        }
    }
    Ok((center, r))
}
fn partition(
    out: &mut [u32],
    indices: &[u32],
    counts: &[u32],
    vertices: usize,
    p: Option<Positions<'_>>,
    target: usize,
    ctx: &mut Context<'_>,
) -> Result<usize, Error> {
    if target == 0 {
        return Err(Error::InvalidParameter);
    }
    let max = target.checked_add(target / 3).ok_or(Error::SizeOverflow)?;
    let n = counts.len();
    ctx.large_memo();
    if n > i32::MAX as usize || vertices > u32::MAX as usize || indices.len() > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    ctx.tick(counts.len() + indices.len())?;
    let mut sum = 0usize;
    for &n in counts {
        if n == 0 {
            return Err(Error::InvalidParameter);
        }
        sum = sum.checked_add(n as usize).ok_or(Error::SizeOverflow)?;
    }
    if sum != indices.len() {
        return Err(Error::InvalidLayout);
    }
    if indices.iter().any(|&v| v as usize >= vertices) {
        return Err(Error::IndexOutOfBounds);
    }
    if let Some(p) = p {
        if p.len() != vertices {
            return Err(Error::InvalidLayout);
        }
        ctx.positions(p)?;
    }
    let mut used = ctx.alloc::<u8>(vertices)?;
    let mut filtered = ctx.alloc::<u32>(indices.len())?;
    let mut offsets = ctx.alloc::<u32>(n + 1)?;
    let (mut start, mut write) = (0, 0);
    for (i, &count) in counts.iter().enumerate() {
        offsets[i] = write as u32;
        ctx.tick(count as usize)?;
        for &v in &indices[start..start + count as usize] {
            if used[v as usize] == 0 {
                filtered[write] = v;
                write += 1;
                used[v as usize] = 1;
            }
        }
        for &v in &filtered[offsets[i] as usize..write] {
            used[v as usize] = 0;
        }
        start += count as usize;
    }
    offsets[n] = write as u32;
    filtered.truncate(write);
    let adj = adjacent(&filtered, &offsets, n, vertices, ctx)?;
    let mut groups = ctx.alloc::<Group>(n)?;
    let mut heap = ctx.alloc::<Order>(n)?;
    let mut acc = ctx.alloc(n)?;
    let mut pending = 0;
    for i in 0..n {
        let verts = &filtered[offsets[i] as usize..offsets[i + 1] as usize];
        let mut g = Group {
            group: i as i32,
            next: -1,
            size: 1,
            vertices: verts.len() as u32,
            ..Group::default()
        };
        if let Some(p) = p {
            ctx.tick(verts.len() * 2)?;
            // Moderate coordinates (|x| <= 1e8) bound every partial sum by
            // 1e8 * 2^32 and every squared offset by 3 * (2e8)^2, so these
            // finiteness checks cannot fail there (as in the meshlet modules).
            let (center, r) = if ctx.moderate {
                group_sphere::<false>(p, verts)?
            } else {
                group_sphere::<true>(p, verts)?
            };
            g.center = center;
            g.radius = ctx.sqrt(r);
        }
        groups[i] = g;
        push(
            &mut heap,
            &mut pending,
            Order {
                id: i as u32,
                order: g.vertices as i32,
            },
        );
    }
    while pending != 0 {
        ctx.tick(1)?;
        let mut top = pop(&mut heap, &mut pending);
        let id = top.id as usize;
        if groups[id].size == 0 {
            continue;
        }
        let mut ci = id as i32;
        while ci >= 0 {
            ctx.tick(1)?;
            groups[ci as usize].group = -1;
            ci = groups[ci as usize].next;
        }
        if groups[id].size as usize >= target {
            continue;
        }
        let Some((best, shared)) = pick(&groups, id, &adj, max, p.is_some(), &mut acc, ctx)? else {
            continue;
        };
        let mut tail = id;
        while groups[tail].next >= 0 {
            ctx.tick(1)?;
            tail = groups[tail].next as usize;
        }
        groups[tail].next = best as i32;
        groups[id].size += groups[best].size;
        let vertices = groups[id]
            .vertices
            .checked_add(groups[best].vertices)
            .ok_or(Error::SizeOverflow)?;
        groups[id].vertices = if vertices > shared {
            vertices - shared
        } else {
            1
        };
        groups[best].size = 0;
        groups[best].vertices = 0;
        if p.is_some() {
            let b = groups[best];
            merge(&mut groups[id], b, ctx)?;
            groups[best].radius = 0.;
        }
        ci = id as i32;
        while ci >= 0 {
            ctx.tick(1)?;
            groups[ci as usize].group = id as i32;
            ci = groups[ci as usize].next;
        }
        top.order = groups[id].vertices as i32;
        push(&mut heap, &mut pending, top);
    }
    if p.is_some() {
        let mut order = ctx.alloc::<u32>(n)?;
        let mut count = 0;
        for (i, g) in groups.iter().enumerate() {
            if g.size != 0 {
                order[count] = i as u32;
                count += 1;
            }
        }
        spatial(&mut groups, &mut order[..count], target, max, 0, ctx)?;
    }
    let mut next = 0;
    for i in 0..n {
        if groups[i].size == 0 {
            continue;
        }
        let mut j = i as i32;
        while j >= 0 {
            ctx.tick(1)?;
            out[j as usize] = next as u32;
            j = groups[j as usize].next;
        }
        next += 1;
    }
    Ok(next)
}
/// `meshopt_partitionClusters`. Clusters may contain arbitrary index lists; empty clusters are invalid.
/// Positions are optional; without them only connected clusters merge.
pub fn partition_clusters(
    indices: &[u32],
    counts: &[u32],
    vertex_count: usize,
    p: Option<Positions<'_>>,
    target: usize,
    workspace: &mut Workspace,
) -> Result<Partitions, Error> {
    let mut ctx = Context::new(workspace)?;
    let mut assignments = ctx.alloc(counts.len())?;
    let count = partition(
        &mut assignments,
        indices,
        counts,
        vertex_count,
        p,
        target,
        &mut ctx,
    )?;
    Ok(Partitions { assignments, count })
}
/// Caller-buffer cluster partitioning; a late failure may modify the used prefix.
pub fn partition_clusters_into(
    out: &mut [u32],
    indices: &[u32],
    counts: &[u32],
    vertex_count: usize,
    p: Option<Positions<'_>>,
    target: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut ctx = Context::new(workspace)?;
    if out.len() < counts.len() {
        return Err(Error::BufferTooSmall);
    }
    partition(
        &mut out[..counts.len()],
        indices,
        counts,
        vertex_count,
        p,
        target,
        &mut ctx,
    )
}
