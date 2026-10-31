use super::*;

/// Caller storage. Only successful returned prefixes are results; tails are preserved.
///
/// Caller capacity is excluded from [`Workspace`] limits. A late error may
/// write prefixes and dilation may mutate mesh positions. Discard failed output
/// and retain fresh source positions for retries; no allocating fallback occurs.
pub struct DagBuffers<'a> {
    /// Source vertex identifiers, in first-occurrence order across the mesh.
    pub vertex_source: &'a mut [u32],
    /// Bit-exact position snapshots at emission, before later dilation.
    pub vertex_positions: &'a mut [[f32; 3]],
    /// Flat topology referencing the snapshot vertex prefix.
    pub indices: &'a mut [u32],
    /// Clusters in emission order.
    pub clusters: &'a mut [FlatDagCluster],
    /// Groups in callback order.
    pub groups: &'a mut [FlatDagGroup],
    /// Per-level spatial forest, with roots first.
    pub nodes: &'a mut [Node],
}
/// Initialized prefix lengths and spatial forest dimensions.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct DagCounts {
    /// Length of both vertex prefixes.
    pub vertices: usize,
    /// Length of the index prefix.
    pub indices: usize,
    /// Length of the cluster prefix.
    pub clusters: usize,
    /// Length of the group prefix.
    pub groups: usize,
    /// Length of the node prefix.
    pub nodes: usize,
    /// Number of DAG levels and forest roots; zero for empty input.
    pub level_count: u32,
    /// Maximum root-to-leaf node count; zero for empty input.
    pub bvh_depth: u32,
}

fn hash(key: [u32; 4]) -> usize {
    let mut h = 0u64;
    for word in key {
        h = (h ^ u64::from(word)).wrapping_mul(0x9e3779b97f4a7c15);
        h ^= h >> 32;
    }
    h as usize
}
fn key(dst: &DagBuffers<'_>, i: usize) -> [u32; 4] {
    let p = dst.vertex_positions[i];
    [
        dst.vertex_source[i],
        p[0].to_bits(),
        p[1].to_bits(),
        p[2].to_bits(),
    ]
}
fn slot(
    table: &[u32],
    k: [u32; 4],
    dst: &DagBuffers<'_>,
    ctx: &mut Context<'_>,
) -> Result<usize, Error> {
    let mut at = hash(k) & (table.len() - 1);
    loop {
        ctx.tick(1)?;
        if table[at] == u32::MAX || key(dst, table[at] as usize) == k {
            return Ok(at);
        }
        at = (at + 1) & (table.len() - 1);
    }
}
fn depth(nodes: &[Node], i: usize, d: u32, ctx: &mut Context<'_>) -> Result<u32, Error> {
    ctx.tick(1)?;
    if d > 32 {
        return Err(Error::LimitExceeded);
    }
    let n = nodes[i];
    let mut result = d;
    if n.group < 0 {
        for c in 0..n.child_count {
            result = result.max(depth(nodes, (n.child_offset + c) as usize, d + 1, ctx)?);
        }
    }
    Ok(result)
}

/// Build the canonical DAG and per-level forest into caller buffers.
///
/// Uses [`default_config`] settings as supplied, with `node_width >= 2`.
/// Vertices share one full-key table across all levels: source index and XYZ
/// float bits. Group, cluster and local first-occurrence order determine IDs.
/// Positions are snapshotted before later dilation. Only returned prefixes are
/// results; tails are preserved. `BufferTooSmall` never allocates replacement
/// output. Late errors may write prefixes and mutate mesh positions; discard
/// failed output and restore fresh source positions before retrying.
///
/// Scratch and cumulative work for construction, flattening and hierarchy share
/// one workspace budget; caller buffers are excluded. Levels above 64 and
/// forest depth above 32 return `LimitExceeded`; u32 identifier overflow returns
/// `SizeOverflow`. Empty input returns empty prefixes and zero dimensions.
///
/// # Errors
/// Same errors as [`super::build`]. `InvalidParameter` for node width below two; `BufferTooSmall` if a caller buffer is too short.
pub fn build_dag_into(
    config: Config,
    mesh: Mesh<'_>,
    node_width: usize,
    destination: DagBuffers<'_>,
    workspace: &mut Workspace,
) -> Result<DagCounts, Error> {
    let mut ctx = Context::new(workspace)?;
    if node_width < 2 {
        return Err(Error::InvalidParameter);
    }
    let mut counts = DagCounts::default();
    let mut table = Vec::<u32>::new();
    let mut groups = Vec::<Group>::new();
    let dst = destination;
    build_in_context(
        config,
        mesh,
        &mut |g, clusters, positions, ctx| {
            if g.depth < 0 || g.depth >= 64 {
                return Err(Error::LimitExceeded);
            }
            if counts.groups >= i32::MAX as usize {
                return Err(Error::SizeOverflow);
            }
            if counts.groups >= dst.groups.len() {
                return Err(Error::BufferTooSmall);
            }
            let end = counts
                .clusters
                .checked_add(clusters.len())
                .ok_or(Error::SizeOverflow)?;
            if end > u32::MAX as usize {
                return Err(Error::SizeOverflow);
            }
            if end > dst.clusters.len() {
                return Err(Error::BufferTooSmall);
            }
            let gid = counts.groups as u32;
            dst.groups[counts.groups] = FlatDagGroup {
                center: g.simplified.center,
                radius: g.simplified.radius,
                error: g.simplified.error,
                depth: g.depth,
                first_cluster: counts.clusters as u32,
                cluster_count: clusters.len() as u32,
            };
            ctx.grow(&mut groups, 1)?;
            groups.push(g);
            counts.groups += 1;
            counts.level_count = counts.level_count.max(g.depth as u32 + 1);
            for cl in &clusters {
                let end = counts
                    .indices
                    .checked_add(cl.indices.len())
                    .ok_or(Error::SizeOverflow)?;
                if end > u32::MAX as usize {
                    return Err(Error::SizeOverflow);
                }
                if end > dst.indices.len() {
                    return Err(Error::BufferTooSmall);
                }
                dst.clusters[counts.clusters] = FlatDagCluster {
                    first_index: counts.indices as u32,
                    index_count: cl.indices.len() as u32,
                    group: gid,
                    refined: cl.refined,
                    center: cl.bounds.center,
                    radius: cl.bounds.radius,
                };
                // Scanning indices directly has exactly clodLocalIndices' local
                // first-occurrence order, including repeated visits to shared keys.
                for &src in &cl.indices {
                    let p = positions[src as usize];
                    let k = [src, p[0].to_bits(), p[1].to_bits(), p[2].to_bits()];
                    if counts.vertices >= table.len() / 2 {
                        let size = table
                            .len()
                            .max(8)
                            .checked_mul(2)
                            .ok_or(Error::SizeOverflow)?;
                        let mut new = ctx.filled(size, u32::MAX)?;
                        for i in 0..counts.vertices {
                            let at = slot(&new, key(&dst, i), &dst, ctx)?;
                            new[at] = i as u32;
                        }
                        ctx.free(core::mem::replace(&mut table, new))?;
                    }
                    let at = slot(&table, k, &dst, ctx)?;
                    let id = if table[at] != u32::MAX {
                        table[at]
                    } else {
                        if counts.vertices >= u32::MAX as usize {
                            return Err(Error::SizeOverflow);
                        }
                        if counts.vertices >= dst.vertex_source.len()
                            || counts.vertices >= dst.vertex_positions.len()
                        {
                            return Err(Error::BufferTooSmall);
                        }
                        let id = counts.vertices as u32;
                        dst.vertex_source[counts.vertices] = src;
                        dst.vertex_positions[counts.vertices] = p;
                        counts.vertices += 1;
                        table[at] = id;
                        id
                    };
                    dst.indices[counts.indices] = id;
                    counts.indices += 1;
                }
                counts.clusters += 1;
                ctx.release(&cl.indices)?;
            }
            ctx.free(clusters)?;
            Ok(gid as i32)
        },
        &mut ctx,
    )?;
    ctx.free(table)?;
    let nodes = hierarchy_in_context(&groups, node_width, counts.level_count as usize, &mut ctx)?;
    if nodes.len() > dst.nodes.len() {
        return Err(Error::BufferTooSmall);
    }
    for i in 0..counts.level_count as usize {
        counts.bvh_depth = counts.bvh_depth.max(depth(&nodes, i, 1, &mut ctx)?);
    }
    counts.nodes = nodes.len();
    ctx.tick(nodes.len())?;
    dst.nodes[..nodes.len()].copy_from_slice(&nodes);
    Ok(counts)
}
