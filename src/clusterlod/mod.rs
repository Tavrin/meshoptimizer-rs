//! Pure-Rust port of the pinned upstream `demo/clusterlod.h` (MIT, Arseny Kapoulkine).
//!
//! This optional module follows the demo output, not a stable meshoptimizer library
//! contract. Positions are explicit mutable packed XYZ because border dilation
//! changes them. Callback identifiers retain the demo's refinement semantics.
#![allow(clippy::too_many_arguments)]
mod support;
use crate::processing::{cross, dot, finite, point, sub, Context};
use crate::{Attributes, Error, MeshletSettings, Positions, VertexFlags, Workspace};
use alloc::vec::Vec;

/// Complete configuration of the upstream cluster-LOD demo.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Maximum vertices per cluster.
    pub max_vertices: usize,
    /// Minimum triangles per cluster.
    pub min_triangles: usize,
    /// Maximum triangles per cluster.
    pub max_triangles: usize,
    /// Include positions when partitioning.
    pub partition_spatial: bool,
    /// Sort partitions spatially.
    pub partition_sort: bool,
    /// Target clusters per partition.
    pub partition_size: usize,
    /// Use the spatial meshlet builder.
    pub cluster_spatial: bool,
    /// Spatial builder fill weight.
    pub cluster_fill_weight: f32,
    /// Flexible builder split factor.
    pub cluster_split_factor: f32,
    /// Target triangle ratio at each level.
    pub simplify_ratio: f32,
    /// Maximum retained ratio before declaring a group stuck.
    pub simplify_threshold: f32,
    /// Multiplier applied to inherited error.
    pub simplify_error_merge_previous: f32,
    /// Additive fraction of this level's error.
    pub simplify_error_merge_additive: f32,
    /// Error multiplier after sloppy fallback.
    pub simplify_error_factor_sloppy: f32,
    /// Optional edge-length error cap; zero disables it.
    pub simplify_error_edge_limit: f32,
    /// Clamp attribute error to positional scale.
    pub simplify_error_clamped: bool,
    /// Enable permissive simplification.
    pub simplify_permissive: bool,
    /// Retry stuck regular simplification in permissive mode.
    pub simplify_fallback_permissive: bool,
    /// Retry stuck simplification with grid-based sloppy simplification.
    pub simplify_fallback_sloppy: bool,
    /// Apply stronger regularization.
    pub simplify_regularize: bool,
    /// Preserve opposing geometric folds.
    pub simplify_preserve_folds: bool,
    /// Dilate simplified open boundaries, mutating positions.
    pub simplify_dilate_borders: bool,
    /// Compute precise culling bounds for simplified output clusters.
    pub optimize_bounds: bool,
    /// Optimize cluster topology.
    pub optimize_clusters: bool,
    /// Meshlet optimization level, zero through nine.
    pub optimize_clusters_level: u8,
}
/// `clodDefaultConfig`: rasterization defaults, for 4 through 256 triangles.
pub fn default_config(max_triangles: usize) -> Result<Config, Error> {
    if !(4..=256).contains(&max_triangles) {
        return Err(Error::InvalidParameter);
    }
    Ok(Config {
        max_vertices: max_triangles,
        min_triangles: max_triangles / 3,
        max_triangles,
        partition_spatial: true,
        partition_sort: false,
        partition_size: 16,
        cluster_spatial: false,
        cluster_fill_weight: 0.,
        cluster_split_factor: 2.,
        optimize_clusters: true,
        optimize_clusters_level: 1,
        simplify_ratio: 0.5,
        simplify_threshold: 0.85,
        simplify_error_merge_previous: 1.,
        simplify_error_merge_additive: 0.,
        simplify_error_factor_sloppy: 2.,
        simplify_error_edge_limit: 0.,
        simplify_error_clamped: true,
        simplify_permissive: true,
        simplify_fallback_permissive: false,
        simplify_fallback_sloppy: true,
        simplify_regularize: false,
        simplify_preserve_folds: false,
        simplify_dilate_borders: false,
        optimize_bounds: false,
    })
}
/// `clodDefaultConfigRT`: ray-tracing defaults.
pub fn default_config_rt(max_triangles: usize) -> Result<Config, Error> {
    let mut c = default_config(max_triangles)?;
    c.min_triangles = max_triangles / 4;
    c.max_vertices = 256.min(max_triangles * 2);
    c.cluster_spatial = true;
    c.cluster_fill_weight = 0.5;
    Ok(c)
}
/// Explicit borrowed mesh inputs. Weights and attribute components must match.
pub struct Mesh<'a> {
    /// Triangle topology referencing the full vertex array.
    pub indices: &'a [u32],
    /// Mutable positions; only dilation changes these.
    pub positions: &'a mut [[f32; 3]],
    /// Optional checked attribute view.
    pub attributes: Option<Attributes<'a>>,
    /// Optional per-vertex flags.
    pub vertex_lock: Option<&'a [VertexFlags]>,
    /// Attribute weights for simplification.
    pub attribute_weights: &'a [f32],
    /// Bit K protects discontinuities in component K.
    pub attribute_protect_mask: u32,
}
/// Conservative sphere and accumulated error.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LodBounds {
    /// Sphere center.
    pub center: [f32; 3],
    /// Sphere radius.
    pub radius: f32,
    /// Absolute simplification error; terminal groups use `f32::MAX`.
    pub error: f32,
}
/// Cluster emitted by the demo builder.
#[derive(Debug, Default, PartialEq)]
pub struct Cluster {
    /// More refined group identifier, or -1 for original geometry.
    pub refined: i32,
    /// Culling bounds; error is not necessarily monotonic across the DAG.
    pub bounds: LodBounds,
    /// Topology referencing the original vertex array.
    pub indices: Vec<u32>,
    /// Number of unique vertex references.
    pub vertex_count: usize,
}
/// Group metadata passed to the output callback.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Group {
    /// DAG level.
    pub depth: i32,
    /// Simplified-group bounds used for monotonic LOD selection.
    pub simplified: LodBounds,
}
/// Owned group record and its emitted clusters.
#[derive(Debug, Default, PartialEq)]
pub struct GroupOutput {
    /// Group metadata.
    pub group: Group,
    /// Clusters in callback order.
    pub clusters: Vec<Cluster>,
}
/// Node in the per-level spatial forest.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Node {
    /// Conservative subtree bounds.
    pub bounds: LodBounds,
    /// Group identifier, or -1 for internal nodes.
    pub group: i32,
    /// Offset of consecutive children.
    pub child_offset: u32,
    /// Number of children.
    pub child_count: u32,
}
#[derive(Clone, Copy, Default)]
struct Pending {
    vertices: usize,
    offset: usize,
    count: usize,
    refined: i32,
    bounds: LodBounds,
}
type CompactGroup = (Vec<u32>, Vec<[f32; 3]>, Vec<u32>);
fn merged(bounds: &[LodBounds], ctx: &mut Context<'_>) -> Result<LodBounds, Error> {
    let mut p = ctx.alloc(bounds.len())?;
    let mut radii = ctx.alloc(bounds.len())?;
    let mut error = 0.;
    for (i, b) in bounds.iter().enumerate() {
        p[i] = b.center;
        radii[i] = b.radius;
        if b.error > error {
            error = b.error;
        }
    }
    let r = Attributes::from_interleaved(&radii, bounds.len(), 1, 1, 0)?;
    let b =
        ctx.child(|ws| crate::compute_sphere_bounds(Positions::from_packed(&p), Some(r), ws))?;
    ctx.free(p)?;
    ctx.free(radii)?;
    Ok(LodBounds {
        center: b.center,
        radius: b.radius,
        error,
    })
}
fn clusterize(
    indices: &[u32],
    p: Positions<'_>,
    c: Config,
    refined: i32,
    moderate: bool,
    ctx: &mut Context<'_>,
) -> Result<(Vec<Pending>, Vec<u32>), Error> {
    let s = MeshletSettings {
        max_vertices: c.max_vertices,
        max_triangles: c.max_triangles,
    };
    let mut m = if !c.cluster_spatial && p.len() > indices.len().saturating_mul(4) {
        // Upstream switches to sparse adjacency for small groups of a large
        // source mesh. Compacting only the temporary builder input lets the
        // safe Rust builder use group-sized adjacency and live-vertex arrays.
        let (local_indices, local_positions, originals) = compact_group(indices, p, ctx)?;
        let mut result = ctx.child(|ws| {
            crate::meshlet::build_meshlets_flex_validated(
                &local_indices,
                Positions::from_packed(&local_positions),
                s,
                c.min_triangles,
                0.,
                c.cluster_split_factor,
                moderate,
                ws,
            )
        })?;
        for v in &mut result.vertices {
            *v = originals[*v as usize];
        }
        ctx.free(local_indices)?;
        ctx.free(local_positions)?;
        ctx.free(originals)?;
        result
    } else {
        ctx.child(|ws| {
            if c.cluster_spatial {
                crate::meshlet::build_meshlets_spatial_validated(
                    indices,
                    p,
                    s,
                    c.min_triangles,
                    c.cluster_fill_weight,
                    moderate,
                    ws,
                )
            } else {
                crate::meshlet::build_meshlets_flex_validated(
                    indices,
                    p,
                    s,
                    c.min_triangles,
                    0.,
                    c.cluster_split_factor,
                    moderate,
                    ws,
                )
            }
        })?
    };
    let mut clusters = ctx.alloc::<Pending>(m.meshlets.len())?;
    let mut idx = ctx.alloc(indices.len())?;
    let mut offset = 0;
    for (i, d) in m.meshlets.iter().enumerate() {
        let v = &mut m.vertices[d.vertex_offset as usize..][..d.vertex_count as usize];
        let t = &mut m.triangles[d.triangle_offset as usize..][..d.triangle_count as usize * 3];
        if c.optimize_clusters {
            ctx.child(|ws| {
                crate::optimize_meshlet_level_in_place(v, t, c.optimize_clusters_level, ws)
            })?;
        }
        clusters[i] = Pending {
            vertices: v.len(),
            offset,
            count: t.len(),
            refined,
            bounds: LodBounds::default(),
        };
        for &t in t.iter() {
            idx[offset] = v[t as usize];
            offset += 1;
        }
    }
    ctx.free(m.meshlets)?;
    ctx.free(m.vertices)?;
    ctx.free(m.triangles)?;
    Ok((clusters, idx))
}
fn compact_group(
    indices: &[u32],
    p: Positions<'_>,
    ctx: &mut Context<'_>,
) -> Result<CompactGroup, Error> {
    let size = indices
        .len()
        .checked_mul(2)
        .and_then(usize::checked_next_power_of_two)
        .ok_or(Error::SizeOverflow)?;
    let mut table = ctx.filled(size, u64::MAX)?;
    let mut local_indices = ctx.copy::<u32>(&[])?;
    let mut local_positions = ctx.copy::<[f32; 3]>(&[])?;
    let mut originals = ctx.copy::<u32>(&[])?;
    ctx.grow(&mut local_indices, indices.len())?;
    ctx.grow(&mut local_positions, indices.len())?;
    ctx.grow(&mut originals, indices.len())?;
    let mut count = 0usize;
    for &source in indices {
        let mut slot = source.wrapping_mul(0x9e3779b9) as usize & (size - 1);
        loop {
            ctx.tick(1)?;
            let entry = table[slot];
            if entry == u64::MAX {
                table[slot] = ((source as u64) << 32) | count as u64;
                local_positions.push(p.get(source as usize).ok_or(Error::IndexOutOfBounds)?);
                originals.push(source);
                local_indices.push(count as u32);
                count += 1;
                break;
            }
            if entry >> 32 == source as u64 {
                local_indices.push(entry as u32);
                break;
            }
            slot = (slot + 1) & (size - 1);
        }
    }
    ctx.free(table)?;
    Ok((local_indices, local_positions, originals))
}
fn position_remap(p: Positions<'_>, ctx: &mut Context<'_>) -> Result<Vec<u32>, Error> {
    let mut size = 1usize;
    let target = p
        .len()
        .checked_add(p.len() / 4)
        .ok_or(Error::SizeOverflow)?;
    while size < target {
        size = size.checked_mul(2).ok_or(Error::SizeOverflow)?;
    }
    let mut table = ctx.alloc::<u32>(size)?;
    table.fill(u32::MAX);
    let mut out = ctx.alloc(p.len())?;
    let mask = size - 1;
    for (i, v) in out.iter_mut().enumerate() {
        let p0 = point(p, i as u32);
        let xyz = p0.map(|x| {
            let b = if x == 0. { 0 } else { x.to_bits() };
            b ^ (b >> 17)
        });
        let hash = xyz[0].wrapping_mul(73856093)
            ^ xyz[1].wrapping_mul(19349663)
            ^ xyz[2].wrapping_mul(83492791);
        let mut slot = hash as usize & mask;
        for probe in 0..size {
            ctx.tick(1)?;
            if table[slot] == u32::MAX {
                table[slot] = i as u32;
                break;
            }
            if point(p, table[slot]) == p0 {
                break;
            }
            slot = (slot + probe + 1) & mask;
        }
        *v = table[slot];
    }
    ctx.free(table)?;
    Ok(out)
}
fn partition(
    clusters: &mut [Pending],
    indices: &[u32],
    remap: &[u32],
    p: Positions<'_>,
    c: Config,
    ctx: &mut Context<'_>,
) -> Result<Vec<u32>, Error> {
    if clusters.len() <= c.partition_size {
        let mut o = ctx.alloc(2)?;
        o[1] = clusters.len() as u32;
        return Ok(o);
    }
    let mut counts = ctx.alloc(clusters.len())?;
    let mut idx = ctx.alloc(indices.len())?;
    let mut off = 0;
    for (i, cl) in clusters.iter().enumerate() {
        counts[i] = cl.count as u32;
        for &v in &indices[cl.offset..cl.offset + cl.count] {
            idx[off] = remap[v as usize];
            off += 1;
        }
    }
    let mut parts = ctx.child(|ws| {
        crate::partition_clusters(
            &idx[..off],
            &counts,
            remap.len(),
            if c.partition_spatial { Some(p) } else { None },
            c.partition_size,
            ws,
        )
    })?;
    if c.partition_sort {
        let mut centers = ctx.alloc(parts.count)?;
        for (i, cl) in clusters.iter().enumerate() {
            centers[parts.assignments[i] as usize] = cl.bounds.center;
        }
        let order =
            ctx.child(|ws| crate::spatial_sort_remap(Positions::from_packed(&centers), ws))?;
        for v in &mut parts.assignments {
            *v = order[*v as usize];
        }
        ctx.free(centers)?;
        ctx.free(order)?;
    }
    let mut offsets = ctx.alloc::<u32>(parts.count + 1)?;
    for &v in &parts.assignments {
        offsets[v as usize + 1] += 1;
    }
    for i in 1..offsets.len() {
        offsets[i] += offsets[i - 1];
    }
    let mut write = ctx.alloc::<u32>(offsets.len())?;
    write.copy_from_slice(&offsets);
    let mut sorted = ctx.alloc(clusters.len())?;
    for (i, &cl) in clusters.iter().enumerate() {
        let w = &mut write[parts.assignments[i] as usize];
        sorted[*w as usize] = cl;
        *w += 1;
    }
    clusters.copy_from_slice(&sorted);
    ctx.free(counts)?;
    ctx.free(idx)?;
    ctx.free(parts.assignments)?;
    ctx.free(write)?;
    ctx.free(sorted)?;
    Ok(offsets)
}
// Reuse the existing byte-sized boundary storage as simplifier flags. Bit 7
// is private discovery state and is removed from every entry before return.
fn lock_boundary(
    locks: &mut [support::VertexFlags],
    clusters: &[Pending],
    offsets: &[u32],
    indices: &[u32],
    remap: &[u32],
    flags: Option<&[VertexFlags]>,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    ctx.tick(locks.len())?;
    for l in locks.iter_mut() {
        *l.boundary_bits_mut() &= !(1 | 128);
    }
    for pair in offsets.windows(2) {
        for cl in &clusters[pair[0] as usize..pair[1] as usize] {
            for &v in &indices[cl.offset..cl.offset + cl.count] {
                ctx.tick(1)?;
                let r = remap[v as usize] as usize;
                let bits = locks[r].bits();
                *locks[r].boundary_bits_mut() |= bits >> 7;
            }
        }
        for cl in &clusters[pair[0] as usize..pair[1] as usize] {
            for &v in &indices[cl.offset..cl.offset + cl.count] {
                ctx.tick(1)?;
                *locks[remap[v as usize] as usize].boundary_bits_mut() |= 128;
            }
        }
    }
    for (i, &r) in remap.iter().enumerate() {
        let bits = (locks[r as usize].bits() & 1) | (locks[i].bits() & 2);
        *locks[i].boundary_bits_mut() = bits;
        if let Some(f) = flags {
            *locks[i].boundary_bits_mut() |= f[i].bits();
        }
    }
    Ok(())
}
fn simplify(
    indices: &[u32],
    mesh: &Mesh<'_>,
    locks: &[support::VertexFlags],
    c: Config,
    target: usize,
    ctx: &mut Context<'_>,
) -> Result<(Vec<u32>, f32), Error> {
    let p = support::Positions::from_packed(mesh.positions);
    debug_assert!(locks.iter().all(|f| f.bits() & !7 == 0));
    let a = match mesh.attributes {
        Some(source) => support::Attributes::from_source(source),
        None => support::Attributes::from_interleaved(&[], mesh.positions.len(), 0, 0, 0)?,
    };
    let mut bits = 2 | 4;
    if c.simplify_error_clamped {
        bits |= 256;
    }
    if c.simplify_permissive {
        bits |= 32;
    }
    if c.simplify_regularize {
        bits |= 16;
    }
    if c.simplify_preserve_folds {
        bits |= 128;
    }
    let mut run = |bits: u32| -> Result<support::SimplifiedMesh, Error> {
        let limits = ctx.remaining_limits()?;
        let mut ws = support::Workspace::new(support::Limits {
            max_bytes: limits.max_bytes,
            max_work: limits.max_work,
        });
        let options = support::SimplifyOptions::from_bits(bits)?;
        let result = support::simplify_validated(
            indices,
            p,
            a,
            mesh.attribute_weights,
            locks,
            support::SimplifySettings {
                target_index_count: target,
                target_error: f32::MAX,
                options,
            },
            &mut ws,
        );
        ctx.absorb(
            ws.usage().bytes,
            ws.usage().work,
            result.as_ref().map_or(0, |r| r.indices.capacity() * 4),
        )?;
        result
    };
    let mut result = run(bits)?;
    if result.indices.len() > target && c.simplify_fallback_permissive && !c.simplify_permissive {
        let next = run(bits | 32)?;
        ctx.free(result.indices)?;
        result = next;
    }
    if result.indices.len() > target && c.simplify_fallback_sloppy {
        let mut subset = ctx.alloc::<[f32; 3]>(indices.len())?;
        let mut slocks = ctx.alloc(indices.len())?;
        let mut seq = ctx.alloc(indices.len())?;
        for (i, &v) in indices.iter().enumerate() {
            subset[i] = mesh.positions[v as usize];
            slocks[i] = support::VertexFlags::from_bits(locks[v as usize].bits() & 1)?;
            seq[i] = i as u32;
        }
        let p = support::Positions::from_packed(&subset);
        let limits = ctx.remaining_limits()?;
        let mut ws = support::Workspace::new(support::Limits {
            max_bytes: limits.max_bytes,
            max_work: limits.max_work,
        });
        let next = support::simplify_sloppy(&seq, p, Some(&slocks), target, f32::MAX, &mut ws);
        ctx.absorb(
            ws.usage().bytes,
            ws.usage().work,
            next.as_ref().map_or(0, |r| r.indices.capacity() * 4),
        )?;
        ctx.free(result.indices)?;
        result = next?;
        result.error *= support::simplify_scale(p)?;
        result.error *= c.simplify_error_factor_sloppy;
        for v in &mut result.indices {
            *v = indices[*v as usize];
        }
        ctx.free(subset)?;
        ctx.free(slocks)?;
        ctx.free(seq)?;
    }
    if result.indices.is_empty() && !indices.is_empty() {
        let mut fallback = ctx.alloc(3)?;
        fallback.copy_from_slice(&indices[..3]);
        ctx.free(result.indices)?;
        result.indices = fallback;
    }
    if c.simplify_error_edge_limit > 0. {
        let mut maxsq = 0f32;
        for t in indices.as_chunks::<3>().0 {
            ctx.tick(1)?;
            let a = mesh.positions[t[0] as usize];
            let b = mesh.positions[t[1] as usize];
            let cc = mesh.positions[t[2] as usize];
            let (ab, ac, bc) = (sub(a, b), sub(a, cc), sub(b, cc));
            let (ab, ac, bc) = (
                finite(dot(ab, ab))?,
                finite(dot(ac, ac))?,
                finite(dot(bc, bc))?,
            );
            let max = ab.max(ac).max(bc);
            let min = ab.min(ac).min(bc);
            maxsq = maxsq.max(min.max(max / 4.));
        }
        result.error = result
            .error
            .min(crate::math::sqrt(maxsq) * c.simplify_error_edge_limit);
    }
    finite(result.error)?;
    Ok((result.indices, result.error))
}
fn edge_slot(table: &[u64], key: u64, ctx: &mut Context<'_>) -> Result<usize, Error> {
    let mask = table.len() - 1;
    let mut h = (key.wrapping_mul(0x9e3779b97f4a7c15) >> 32) as usize & mask;
    while table[h] != u64::MAX && table[h] != key {
        ctx.tick(1)?;
        h = (h + 1) & mask;
    }
    Ok(h)
}
fn boundary_area(
    p: Positions<'_>,
    indices: &[u32],
    locks: &[support::VertexFlags],
    remap: &[u32],
    table: &mut [u64],
    ctx: &mut Context<'_>,
) -> Result<f32, Error> {
    table.fill(u64::MAX);
    for t in indices.as_chunks::<3>().0 {
        for e in 0..3 {
            ctx.tick(1)?;
            let (a, b) = (remap[t[e] as usize], remap[t[(e + 1) % 3] as usize]);
            let key = (a as u64) << 32 | b as u64;
            let slot = edge_slot(table, key, ctx)?;
            table[slot] = key;
        }
    }
    let mut area = 0.;
    for t in indices.as_chunks::<3>().0 {
        let mut border = false;
        for e in 0..3 {
            ctx.tick(1)?;
            let (a, b) = (remap[t[e] as usize], remap[t[(e + 1) % 3] as usize]);
            let key = (b as u64) << 32 | a as u64;
            border |= (locks[a as usize].bits() & locks[b as usize].bits() & 1) == 0
                && table[edge_slot(table, key, ctx)?] == u64::MAX;
        }
        if border {
            let a = point(p, t[0]);
            let nt = cross(sub(point(p, t[1]), a), sub(point(p, t[2]), a));
            area = finite(area + crate::math::sqrt(finite(dot(nt, nt))?) * 0.5)?;
        }
    }
    Ok(area)
}
fn dilate(
    mesh: &mut Mesh<'_>,
    old: &[u32],
    new: &[u32],
    locks: &[support::VertexFlags],
    remap: &[u32],
    bounds: &mut LodBounds,
    offsets: &mut [[f32; 4]],
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let mut size = 16usize;
    let capacity = old
        .len()
        .checked_add(old.len() / 2)
        .ok_or(Error::SizeOverflow)?;
    while size < capacity {
        size = size.checked_mul(2).ok_or(Error::SizeOverflow)?;
    }
    let mut table = ctx.alloc::<u64>(size)?;
    let p = Positions::from_packed(mesh.positions);
    let oa = boundary_area(p, old, locks, remap, &mut table, ctx)?;
    if oa == 0. {
        ctx.free(table)?;
        return Ok(());
    }
    let na = boundary_area(p, new, locks, remap, &mut table, ctx)?;
    if na >= oa || na < oa / 4. {
        ctx.free(table)?;
        return Ok(());
    }
    let mut perimeter = 0.;
    for t in new.as_chunks::<3>().0 {
        for e in 0..3 {
            ctx.tick(1)?;
            let (a, b) = (
                remap[t[e] as usize] as usize,
                remap[t[(e + 1) % 3] as usize] as usize,
            );
            let key = (b as u64) << 32 | a as u64;
            if locks[a].bits() & locks[b].bits() & 1 != 0
                || table[edge_slot(&table, key, ctx)?] != u64::MAX
            {
                continue;
            }
            let va = point(p, t[e]);
            let vb = point(p, t[(e + 1) % 3]);
            let vc = point(p, t[(e + 2) % 3]);
            let ev = sub(vb, va);
            let el = crate::math::sqrt(finite(dot(ev, ev))?);
            let cv = sub(va, vc);
            let cp = dot(cv, ev) / if el > 0. { el * el } else { 1. };
            let nv = [cv[0] - ev[0] * cp, cv[1] - ev[1] * cp, cv[2] - ev[2] * cp];
            let nl = crate::math::sqrt(finite(dot(nv, nv))?);
            let ns = if nl > 0. { el / nl } else { 0. };
            for v in [a, b] {
                if locks[v].bits() & 1 == 0 {
                    for (k, &n) in nv.iter().enumerate() {
                        offsets[v][k] += n * ns;
                    }
                    offsets[v][3] += el;
                }
            }
            perimeter += el;
        }
    }
    let distance = if perimeter > 0. {
        (oa - na) / perimeter
    } else {
        0.
    };
    for &v in new {
        ctx.tick(1)?;
        let r = remap[v as usize] as usize;
        if locks[r].bits() & 1 != 0 {
            continue;
        }
        let n = &mut offsets[r];
        if n[3] > 0. {
            let nn = [n[0] / n[3], n[1] / n[3], n[2] / n[3]];
            let l = dot(nn, nn);
            let ns = distance / if l > 0.15 { l } else { 0.15 };
            for (k, &x) in nn.iter().enumerate() {
                mesh.positions[r][k] = finite(mesh.positions[r][k] + x * ns)?;
            }
            *n = [0.; 4];
            let d = sub(mesh.positions[r], bounds.center);
            bounds.radius = bounds.radius.max(crate::math::sqrt(finite(dot(d, d))?));
        }
        mesh.positions[v as usize] = mesh.positions[r];
    }
    ctx.free(table)?;
    Ok(())
}
fn output(
    group: &[Pending],
    idx: &[u32],
    mesh: &Mesh<'_>,
    c: Config,
    bounds: LodBounds,
    depth: i32,
    callback: &mut impl FnMut(Group, Vec<Cluster>, &mut Context<'_>) -> Result<i32, Error>,
    ctx: &mut Context<'_>,
) -> Result<i32, Error> {
    let mut out = ctx.alloc::<Cluster>(group.len())?;
    for (i, cl) in group.iter().enumerate() {
        let indices = &idx[cl.offset..cl.offset + cl.count];
        let b = if c.optimize_bounds && cl.refined != -1 {
            let b = ctx.child(|ws| {
                crate::compute_cluster_bounds(indices, Positions::from_packed(mesh.positions), ws)
            })?;
            LodBounds {
                center: b.center,
                radius: b.radius,
                error: cl.bounds.error,
            }
        } else {
            cl.bounds
        };
        let copy = ctx.copy(indices)?;
        out[i] = Cluster {
            refined: cl.refined,
            bounds: b,
            indices: copy,
            vertex_count: cl.vertices,
        };
    }
    callback(
        Group {
            depth,
            simplified: bounds,
        },
        out,
        ctx,
    )
}
fn validate(mesh: &Mesh<'_>, c: Config, ctx: &mut Context<'_>) -> Result<(), Error> {
    crate::build_meshlets_bound(mesh.indices.len(), c.max_vertices, c.max_triangles)?;
    if c.min_triangles == 0
        || c.min_triangles > c.max_triangles
        || c.partition_size == 0
        || c.optimize_clusters_level > 9
    {
        return Err(Error::InvalidParameter);
    }
    for v in [
        c.cluster_fill_weight,
        c.cluster_split_factor,
        c.simplify_ratio,
        c.simplify_threshold,
        c.simplify_error_merge_previous,
        c.simplify_error_merge_additive,
        c.simplify_error_factor_sloppy,
        c.simplify_error_edge_limit,
    ] {
        if !v.is_finite() || v < 0. {
            return Err(Error::InvalidParameter);
        }
    }
    if c.simplify_ratio >= 1. || c.simplify_threshold >= 1. {
        return Err(Error::InvalidParameter);
    }
    let n = mesh.positions.len();
    ctx.positions(Positions::from_packed(mesh.positions))?;
    ctx.topology(mesh.indices, n)?;
    crate::validate_vertex_flags(mesh.vertex_lock, n)?;
    if mesh.attribute_weights.len() > 32 {
        return Err(Error::InvalidLayout);
    }
    if let Some(a) = mesh.attributes {
        if a.len() != n || a.components() != mesh.attribute_weights.len() {
            return Err(Error::InvalidLayout);
        }
        ctx.tick(n.checked_mul(a.components()).ok_or(Error::SizeOverflow)?)?;
        for i in 0..n {
            for j in 0..a.components() {
                finite(a.get(i, j).ok_or(Error::InvalidLayout)?)?;
            }
        }
    } else if !mesh.attribute_weights.is_empty() {
        return Err(Error::InvalidLayout);
    }
    for &w in mesh.attribute_weights {
        if !w.is_finite() || w < 0. {
            return Err(Error::InvalidParameter);
        }
    }
    if mesh.attribute_weights.len() < 32
        && mesh.attribute_protect_mask >> mesh.attribute_weights.len() != 0
    {
        return Err(Error::InvalidParameter);
    }
    Ok(())
}
fn build_internal(
    c: Config,
    mut mesh: Mesh<'_>,
    callback: &mut impl FnMut(Group, Vec<Cluster>, &mut Context<'_>) -> Result<i32, Error>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    validate(&mesh, c, &mut ctx)?;
    let mut moderate = ctx.moderate;
    let p = Positions::from_packed(mesh.positions);
    let remap = position_remap(p, &mut ctx)?;
    let mut locks = ctx.alloc::<support::VertexFlags>(p.len())?;
    if let Some(a) = mesh.attributes {
        for (i, l) in locks.iter_mut().enumerate() {
            let r = remap[i] as usize;
            for j in 0..a.components() {
                if r != i
                    && mesh.attribute_protect_mask & (1u32 << j) != 0
                    && a.get(i, j) != a.get(r, j)
                {
                    *l.boundary_bits_mut() |= 2;
                }
            }
        }
    }
    let mut offsets = ctx.alloc::<[f32; 4]>(if c.simplify_dilate_borders {
        p.len()
    } else {
        0
    })?;
    let (mut clusters, mut idx) = clusterize(mesh.indices, p, c, -1, moderate, &mut ctx)?;
    for cl in &mut clusters {
        let b = ctx.child(|ws| {
            crate::meshlet_util::compute_cluster_bounds_validated(
                &idx[cl.offset..cl.offset + cl.count],
                p,
                moderate,
                ws,
            )
        })?;
        cl.bounds = LodBounds {
            center: b.center,
            radius: b.radius,
            error: 0.,
        };
    }
    let mut depth = 0;
    while clusters.len() > 1 {
        let group_offsets = partition(
            &mut clusters,
            &idx,
            &remap,
            Positions::from_packed(mesh.positions),
            c,
            &mut ctx,
        )?;
        lock_boundary(
            &mut locks,
            &clusters,
            &group_offsets,
            &idx,
            &remap,
            mesh.vertex_lock,
            &mut ctx,
        )?;
        let mut pending = ctx.alloc::<Pending>(idx.len() / 3)?;
        let mut pi = ctx.alloc::<u32>(idx.len())?;
        let (mut pc, mut ic) = (0, 0);
        for pair in group_offsets.windows(2) {
            let group = &clusters[pair[0] as usize..pair[1] as usize];
            let n = group.iter().map(|cl| cl.count).sum();
            let mut merged_idx = ctx.alloc::<u32>(n)?;
            let mut write = 0;
            let mut gb = ctx.alloc::<LodBounds>(group.len())?;
            for (i, cl) in group.iter().enumerate() {
                merged_idx[write..write + cl.count]
                    .copy_from_slice(&idx[cl.offset..cl.offset + cl.count]);
                write += cl.count;
                gb[i] = cl.bounds;
            }
            let target = ((merged_idx.len() / 3) as f32 * c.simplify_ratio) as usize * 3;
            let mut bounds = merged(&gb, &mut ctx)?;
            let (simplified, error) = simplify(&merged_idx, &mesh, &locks, c, target, &mut ctx)?;
            if simplified.len() as f32 > merged_idx.len() as f32 * c.simplify_threshold {
                bounds.error = f32::MAX;
                output(group, &idx, &mesh, c, bounds, depth, callback, &mut ctx)?;
                ctx.free(merged_idx)?;
                ctx.free(gb)?;
                ctx.free(simplified)?;
                continue;
            }
            bounds.error = finite(
                (bounds.error * c.simplify_error_merge_previous).max(error)
                    + error * c.simplify_error_merge_additive,
            )?;
            let refined = output(group, &idx, &mesh, c, bounds, depth, callback, &mut ctx)?;
            if c.simplify_dilate_borders {
                dilate(
                    &mut mesh,
                    &merged_idx,
                    &simplified,
                    &locks,
                    &remap,
                    &mut bounds,
                    &mut offsets,
                    &mut ctx,
                )?;
                // Dilation can move a coordinate beyond the validated range.
                // Use the checked flex/bounds path for this and later levels.
                moderate = false;
            }
            let (new, ni) = clusterize(
                &simplified,
                Positions::from_packed(mesh.positions),
                c,
                refined,
                moderate,
                &mut ctx,
            )?;
            for &original in &new {
                let mut cl = original;
                cl.offset += ic;
                cl.bounds = bounds;
                pending[pc] = cl;
                pc += 1;
            }
            pi[ic..ic + ni.len()].copy_from_slice(&ni);
            ic += ni.len();
            ctx.free(new)?;
            ctx.free(ni)?;
            ctx.free(merged_idx)?;
            ctx.free(gb)?;
            ctx.free(simplified)?;
        }
        ctx.free(group_offsets)?;
        pending.truncate(pc);
        pi.truncate(ic);
        ctx.free(clusters)?;
        ctx.free(idx)?;
        clusters = pending;
        idx = pi;
        depth += 1;
    }
    if !clusters.is_empty() {
        let mut bounds = clusters[0].bounds;
        bounds.error = f32::MAX;
        output(&clusters, &idx, &mesh, c, bounds, depth, callback, &mut ctx)?;
    }
    Ok(())
}
/// `clodBuild`, with explicit callback return identifiers. A late error may leave
/// dilated positions changed and callbacks already emitted.
pub fn build_with_output(
    c: Config,
    mesh: Mesh<'_>,
    mut output: impl FnMut(Group, &[Cluster]) -> Result<i32, Error>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    build_internal(
        c,
        mesh,
        &mut |g, clusters, ctx| {
            let result = output(g, &clusters);
            for cl in &clusters {
                ctx.release(&cl.indices)?;
            }
            ctx.free(clusters)?;
            result
        },
        workspace,
    )
}
/// Allocating demo hierarchy. Refined identifiers index the returned group list.
/// Dilation may change positions even if a later operation fails.
pub fn build(
    c: Config,
    mesh: Mesh<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<GroupOutput>, Error> {
    let mut result = Vec::new();
    build_internal(
        c,
        mesh,
        &mut |g, clusters, ctx| {
            if result.len() >= i32::MAX as usize {
                return Err(Error::SizeOverflow);
            }
            let id = result.len() as i32;
            ctx.tick(1)?;
            ctx.grow(&mut result, 1)?;
            result.push(GroupOutput { group: g, clusters });
            Ok(id)
        },
        workspace,
    )?;
    Ok(result)
}
/// `clodLocalIndices`: first-visit local extraction.
pub fn local_indices(
    indices: &[u32],
    workspace: &mut Workspace,
) -> Result<crate::LocalMeshlet, Error> {
    crate::extract_meshlet_indices(indices, workspace)
}
/// `clodBuildHierarchyBound`: checked bound for a per-level spatial forest.
pub fn build_hierarchy_bound(
    group_count: usize,
    node_width: usize,
    level_count: usize,
) -> Result<usize, Error> {
    if node_width < 2 {
        return Err(Error::InvalidParameter);
    }
    let mut total = level_count;
    let mut frontier = group_count;
    while frontier > 1 {
        total = total
            .checked_add(frontier)
            .and_then(|n| n.checked_add(level_count))
            .ok_or(Error::SizeOverflow)?;
        frontier = frontier
            .checked_add(node_width - 1)
            .ok_or(Error::SizeOverflow)?
            / node_width;
    }
    Ok(total)
}
/// `clodBuildHierarchy`: every supplied level must have at least one group.
pub fn build_hierarchy(
    groups: &[Group],
    node_width: usize,
    level_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<Node>, Error> {
    let mut ctx = Context::new(workspace)?;
    let bound = build_hierarchy_bound(groups.len(), node_width, level_count)?;
    if groups.len() > i32::MAX as usize || bound > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    ctx.tick(groups.len())?;
    for g in groups {
        if g.depth < 0
            || g.depth as usize >= level_count
            || !g.simplified.radius.is_finite()
            || g.simplified.radius < 0.
            || !g.simplified.error.is_finite()
            || g.simplified.error < 0.
            || g.simplified.center.iter().any(|v| !v.is_finite())
        {
            return Err(Error::InvalidParameter);
        }
    }
    let mut nodes = ctx.alloc::<Node>(bound)?;
    let mut row = ctx.alloc::<Node>(groups.len())?;
    let mut offset = level_count;
    for level in 0..level_count {
        ctx.tick(groups.len())?;
        let mut n = 0;
        for (i, g) in groups.iter().enumerate() {
            if g.depth as usize == level {
                row[n] = Node {
                    bounds: g.simplified,
                    group: i as i32,
                    child_offset: 0,
                    child_count: 0,
                };
                n += 1;
            }
        }
        if n == 0 {
            return Err(Error::InvalidParameter);
        }
        while n > 1 {
            let mut p = ctx.alloc(n)?;
            for i in 0..n {
                p[i] = row[i].bounds.center;
            }
            let order = ctx.child(|ws| {
                crate::spatial_cluster_points(Positions::from_packed(&p), node_width, ws)
            })?;
            ctx.tick(n.checked_mul(2).ok_or(Error::SizeOverflow)?)?;
            for i in 0..n {
                nodes[offset + i] = row[order[i] as usize];
            }
            let mut next = 0;
            for i in (0..n).step_by(node_width) {
                let count = node_width.min(n - i);
                let mut bs = ctx.alloc(count)?;
                for j in 0..count {
                    bs[j] = nodes[offset + i + j].bounds;
                }
                row[next] = Node {
                    bounds: merged(&bs, &mut ctx)?,
                    group: -1,
                    child_offset: (offset + i) as u32,
                    child_count: count as u32,
                };
                next += 1;
                ctx.free(bs)?;
            }
            ctx.free(p)?;
            ctx.free(order)?;
            offset += n;
            n = next;
        }
        nodes[level] = row[0];
    }
    nodes.truncate(offset);
    Ok(nodes)
}
