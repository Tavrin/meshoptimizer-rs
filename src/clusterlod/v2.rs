//! Build policy for MDAG 2. Legacy demo entry points retain upstream semantics.
use super::*;
use crate::{LodQualityMesh, LodQualityOptions};

/// Inputs for a static virtual-geometry build.
#[non_exhaustive]
pub struct BuildMesh<'a> {
    /// Source triangle indices.
    pub indices: &'a [u32],
    /// Source positions, snapped before simplification.
    pub positions: &'a [[f32; 3]],
    /// Optional unit normals, weighted by 0.5 in simplification.
    pub normals: Option<&'a [[f32; 3]]>,
    /// Optional checked attributes in source vertex order.
    pub attributes: Option<Attributes<'a>>,
    /// Weights for the supplied attribute components.
    pub attribute_weights: &'a [f32],
    /// Protected attribute component mask.
    pub attribute_protect_mask: u32,
    /// Optional material slot per source triangle.
    pub materials: Option<&'a [u16]>,
    /// Opt-in alpha coverage policy; UV components refer to `attributes`.
    pub alpha_coverage: Option<AlphaCoverage<'a>>,
    /// Optional source vertex locks.
    pub vertex_lock: Option<&'a [VertexFlags]>,
}
impl<'a> BuildMesh<'a> {
    /// Position-only input. Set optional channels before building.
    pub const fn new(indices: &'a [u32], positions: &'a [[f32; 3]]) -> Self {
        Self {
            indices,
            positions,
            normals: None,
            attributes: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
            materials: None,
            vertex_lock: None,
            alpha_coverage: None,
        }
    }
}
/// Owned flat DAG ready for the MDAG 2 writer.
#[derive(Default, Debug)]
pub struct Built {
    /// Snapped positions, duplicated at material boundaries.
    pub positions: Vec<[f32; 3]>,
    /// Original source vertex for each built vertex.
    pub vertex_source: Vec<u32>,
    /// Slot per built vertex (zero when absent).
    pub vertex_material: Vec<u16>,
    /// Packed source attributes, then normals when supplied, then the material component.
    pub attributes: Vec<f32>,
    /// Number of components per built vertex.
    pub attribute_stride: usize,
    /// Flat triangle indices.
    pub indices: Vec<u32>,
    /// Clusters in fine-to-coarse emission order.
    pub clusters: Vec<FlatDagCluster>,
    /// Groups in fine-to-coarse emission order.
    pub groups: Vec<FlatDagGroup>,
    /// Groups rejected by retained-ratio, empty-output, mixed-slot or alpha coverage rules.
    pub stuck_groups: usize,
    /// Steps rejected by the alpha coverage rule and retained as pinned stuck groups.
    pub alpha_coverage_stuck_groups: usize,
    /// Steps with raised error (zero: the coverage policy rejects failing steps).
    pub alpha_coverage_raised_groups: usize,
    /// Groups rejected because simplification, including fallback, mixed material slots.
    pub material_stuck_groups: usize,
}
/// Build a snapped DAG with normals weighted by 0.5, protected material seams,
/// and a sampled symmetric original-source error floor (4000 area-scaled
/// interior samples, plus all corners). Each region is the union of its leaf
/// triangles before snapping. Errors also bound inherited child errors.
/// Sampling is not a continuous Hausdorff certificate.
/// Dilation is rejected: this path retains exact source attribute references.
///
/// # Errors
/// Legacy build errors, invalid channel lengths, mixed source slots, unsupported
/// dilation, or exhausted workspace limits. No partial result is returned.
pub fn build(
    config: Config,
    mesh: BuildMesh<'_>,
    grid: page::QuantGrid,
    workspace: &mut Workspace,
) -> Result<Built, Error> {
    let mut ctx = Context::new(workspace)?;
    if config.simplify_dilate_borders || config.simplify_error_merge_previous < 1.0 {
        return Err(Error::InvalidParameter);
    }
    let n = mesh.positions.len();
    u32::try_from(n).map_err(|_| Error::SizeOverflow)?;
    ctx.positions(Positions::from_packed(mesh.positions))?;
    ctx.topology(mesh.indices, n)?;
    crate::validate_vertex_flags(mesh.vertex_lock, n)?;
    let ac = mesh.attributes.map_or(0, |a| a.components());
    if ac > 32
        || mesh.attribute_weights.len() != ac
        || mesh.attributes.is_some_and(|a| a.len() != n)
        || mesh.normals.is_some_and(|a| a.len() != n)
        || mesh
            .materials
            .is_some_and(|a| a.len() != mesh.indices.len() / 3)
        || (ac < 32 && mesh.attribute_protect_mask >> ac != 0)
    {
        return Err(Error::InvalidLayout);
    }
    if let Some(alpha) = mesh.alpha_coverage {
        alpha.validate(ac)?;
    }
    let stride =
        ac + usize::from(mesh.normals.is_some()) * 3 + usize::from(mesh.materials.is_some());
    if stride > 32 {
        return Err(Error::InvalidLayout);
    }
    let mut out = Built {
        attribute_stride: stride,
        ..Built::default()
    };
    // Sort unique (source, slot) pairs; no allocation proportional to slot IDs.
    let mut pairs = ctx.alloc::<(u32, u16)>(if mesh.materials.is_some() {
        mesh.indices.len()
    } else {
        0
    })?;
    if let Some(materials) = mesh.materials {
        for (k, (&i, p)) in mesh.indices.iter().zip(&mut pairs).enumerate() {
            *p = (i, materials[k / 3]);
        }
        ctx.tick(pairs.len().saturating_mul(usize::BITS as usize * 2))?;
        pairs.sort_unstable();
        pairs.dedup();
    }
    let count = if mesh.materials.is_some() {
        pairs.len()
    } else {
        n
    };
    out.positions = ctx.alloc(count)?;
    out.vertex_source = ctx.alloc(count)?;
    out.vertex_material = ctx.alloc(count)?;
    out.attributes = ctx.alloc(count.checked_mul(stride).ok_or(Error::SizeOverflow)?)?;
    let mut locks = ctx.alloc::<VertexFlags>(count)?;
    for v in 0..count {
        let (src, slot) = if mesh.materials.is_some() {
            pairs[v]
        } else {
            (v as u32, 0)
        };
        out.positions[v] = grid.position(
            grid.snap(mesh.positions[src as usize])
                .map_err(|_| Error::InvalidParameter)?,
        );
        out.vertex_source[v] = src;
        out.vertex_material[v] = slot;
        if let Some(l) = mesh.vertex_lock {
            locks[v] = l[src as usize];
        }
        for c in 0..ac {
            out.attributes[v * stride + c] = mesh
                .attributes
                .and_then(|a| a.get(src as usize, c))
                .ok_or(Error::InvalidLayout)?;
        }
        if let Some(normals) = mesh.normals {
            let normal = normals[src as usize];
            if normal.iter().any(|x| !x.is_finite()) || normal.iter().all(|&x| x == 0.0) {
                return Err(Error::InvalidParameter);
            }
            out.attributes[v * stride + ac..v * stride + ac + 3].copy_from_slice(&normal);
        }
        if mesh.materials.is_some() {
            out.attributes[(v + 1) * stride - 1] = f32::from(slot);
        }
    }
    let mut topology = ctx.alloc(if mesh.materials.is_some() {
        mesh.indices.len()
    } else {
        0
    })?;
    if let Some(materials) = mesh.materials {
        for (k, (&i, t)) in mesh.indices.iter().zip(&mut topology).enumerate() {
            *t = u32::try_from(
                pairs
                    .binary_search(&(i, materials[k / 3]))
                    .map_err(|_| Error::InvalidTopology)?,
            )
            .map_err(|_| Error::SizeOverflow)?;
        }
    }
    ctx.free(pairs)?;
    let mut weights = ctx.copy(mesh.attribute_weights)?;
    ctx.grow(&mut weights, stride - ac)?;
    if mesh.normals.is_some() {
        weights.extend_from_slice(&[0.5; 3]);
    }
    let slot_component = mesh.materials.map(|_| stride - 1);
    if slot_component.is_some() {
        weights.push(0.0);
    }
    let protect = mesh.attribute_protect_mask | slot_component.map_or(0, |s| 1 << s);
    let attrs = if stride == 0 {
        None
    } else {
        Some(Attributes::from_interleaved(
            &out.attributes,
            count,
            stride,
            stride,
            0,
        )?)
    };
    let (stuck_groups, material_stuck_groups, alpha_stuck_groups) = super::build_in_context_mode(
        config,
        MeshGeneric {
            indices: if mesh.materials.is_some() {
                &topology
            } else {
                mesh.indices
            },
            positions: &mut out.positions,
            attributes: attrs,
            vertex_lock: Some(&locks),
            attribute_weights: &weights,
            attribute_protect_mask: protect,
        },
        &mut |g, mut cs, _p, ctx| {
            if g.depth < 0 || g.depth as u32 >= super::dag_format::v2::MAX_LEVELS {
                return Err(Error::LimitExceeded);
            }
            let gid = u32::try_from(out.groups.len()).map_err(|_| Error::SizeOverflow)?;
            let first = u32::try_from(out.clusters.len()).map_err(|_| Error::SizeOverflow)?;
            u32::try_from(
                out.clusters
                    .len()
                    .checked_add(cs.len())
                    .ok_or(Error::SizeOverflow)?,
            )
            .map_err(|_| Error::SizeOverflow)?;
            grow_output(ctx, &mut out.groups, 1)?;
            out.groups.push(FlatDagGroup {
                center: g.simplified.center,
                radius: g.simplified.radius,
                error: g.simplified.error,
                depth: g.depth,
                first_cluster: first,
                cluster_count: u32::try_from(cs.len()).map_err(|_| Error::SizeOverflow)?,
            });
            for c in &mut cs {
                grow_output(ctx, &mut out.clusters, 1)?;
                let start = u32::try_from(out.indices.len()).map_err(|_| Error::SizeOverflow)?;
                u32::try_from(
                    out.indices
                        .len()
                        .checked_add(c.indices.len())
                        .ok_or(Error::SizeOverflow)?,
                )
                .map_err(|_| Error::SizeOverflow)?;
                grow_output(ctx, &mut out.indices, c.indices.len())?;
                out.indices.extend_from_slice(&c.indices);
                // Stable sort by slot; topology within each range keeps its existing order.
                if mesh.materials.is_some() {
                    let triangles = out.indices[start as usize..].as_chunks_mut::<3>().0;
                    for i in 1..triangles.len() {
                        let t = triangles[i];
                        let slot = out.vertex_material[t[0] as usize];
                        let mut at = i;
                        while at > 0 && out.vertex_material[triangles[at - 1][0] as usize] > slot {
                            ctx.tick(1)?;
                            triangles[at] = triangles[at - 1];
                            at -= 1;
                        }
                        triangles[at] = t;
                    }
                    if config.optimize_clusters {
                        optimize_material_ranges(
                            &mut out.indices[start as usize..],
                            &out.vertex_material,
                            config.optimize_clusters_level,
                            ctx,
                        )?;
                    }
                }
                out.clusters.push(FlatDagCluster {
                    first_index: start,
                    index_count: u32::try_from(c.indices.len()).map_err(|_| Error::SizeOverflow)?,
                    group: gid,
                    refined: c.refined,
                    center: c.bounds.center,
                    radius: c.bounds.radius,
                });
                ctx.free(core::mem::take(&mut c.indices))?;
            }
            ctx.free(cs)?;
            i32::try_from(gid).map_err(|_| Error::SizeOverflow)
        },
        &mut ctx,
        cfg!(feature = "parallel"),
        Some(BuildPolicy {
            material_component: slot_component,
            alpha: mesh.alpha_coverage,
        }),
    )?;
    out.alpha_coverage_stuck_groups = alpha_stuck_groups;
    out.stuck_groups = stuck_groups;
    out.material_stuck_groups = material_stuck_groups;
    ctx.free(topology)?;
    ctx.free(weights)?;
    ctx.free(locks)?;
    source_error_floor(&mut out, mesh.positions, &mut ctx)?;
    Ok(out)
}

// A cluster split can send siblings to different parents. Follow refined groups
// once per region, not once per cluster: each replacement represents the union
// of all source triangles under the group, including siblings in other parents.
fn source_error_floor(
    out: &mut Built,
    original_positions: &[[f32; 3]],
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let mut source_positions = ctx.alloc(out.positions.len())?;
    ctx.tick(source_positions.len())?;
    for (p, &id) in source_positions.iter_mut().zip(&out.vertex_source) {
        *p = original_positions[id as usize];
    }
    let mut heads = ctx.alloc::<Option<usize>>(out.groups.len())?;
    let mut next = ctx.alloc::<Option<usize>>(out.clusters.len())?;
    ctx.tick(out.clusters.len())?;
    for (i, c) in out.clusters.iter().enumerate().rev() {
        if c.refined >= 0 {
            let g = c.refined as usize;
            next[i] = heads[g];
            heads[g] = Some(i);
        }
    }
    let mut measured = ctx.alloc::<f32>(out.groups.len())?;
    #[cfg(feature = "parallel")]
    let parallel = {
        let limits = ctx.workspace.limits();
        limits.max_bytes == usize::MAX && limits.max_work == u64::MAX
    };
    #[cfg(not(feature = "parallel"))]
    let parallel = false;
    if parallel {
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            // Limit live scratch to one job per worker. Account for the sum of
            // simultaneous peaks; bounded workspaces use the serial path below.
            let width = rayon::current_num_threads();
            for (batch, values) in measured.chunks_mut(width).enumerate() {
                let mut usages = ctx.alloc::<crate::Usage>(values.len())?;
                let mut results = ctx.alloc::<Option<Result<f32, Error>>>(values.len())?;
                results
                    .par_iter_mut()
                    .zip(usages.par_iter_mut())
                    .enumerate()
                    .for_each(|(i, (result, usage))| {
                        let mut ws = Workspace::new(crate::Limits {
                            max_bytes: usize::MAX,
                            max_work: u64::MAX,
                        });
                        *result = Some(Context::new(&mut ws).and_then(|mut local| {
                            measure_region(
                                out,
                                batch * width + i,
                                &heads,
                                &next,
                                &source_positions,
                                &mut local,
                            )
                        }));
                        *usage = ws.usage();
                    });
                let peak = usages.iter().try_fold(0usize, |sum, u| {
                    sum.checked_add(u.bytes).ok_or(Error::SizeOverflow)
                })?;
                ctx.absorb(peak, 0, 0)?;
                for ((value, result), usage) in
                    values.iter_mut().zip(results.iter_mut()).zip(&usages)
                {
                    ctx.absorb(0, usage.work, 0)?;
                    *value = result.take().ok_or(Error::InvalidTopology)??;
                }
                ctx.free(usages)?;
                ctx.free(results)?;
            }
        }
    } else {
        for (g, value) in measured.iter_mut().enumerate() {
            *value = measure_region(out, g, &heads, &next, &source_positions, ctx)?;
        }
    }
    // Measurement is independent of inherited errors. Commit in emission order.
    ctx.release(&measured)?;
    for (g, value) in measured.into_iter().enumerate() {
        let group = &out.groups[g];
        let mut inherited = 0f32;
        for c in &out.clusters
            [group.first_cluster as usize..(group.first_cluster + group.cluster_count) as usize]
        {
            ctx.tick(1)?;
            if c.refined >= 0 {
                inherited = inherited.max(out.groups[c.refined as usize].error);
            }
        }
        out.groups[g].error = out.groups[g].error.max(inherited).max(value);
    }
    ctx.free(source_positions)?;
    ctx.free(heads)?;
    ctx.free(next)?;
    Ok(())
}

fn measure_region(
    out: &Built,
    g: usize,
    heads: &[Option<usize>],
    next: &[Option<usize>],
    source_positions: &[[f32; 3]],
    ctx: &mut Context<'_>,
) -> Result<f32, Error> {
    if out.groups[g].error == f32::MAX {
        return Ok(0.0);
    }
    let mut visited = ctx.alloc::<bool>(out.groups.len())?;
    let mut stack = ctx.alloc::<usize>(0)?;
    let mut source = ctx.alloc::<u32>(0)?;
    let mut simplified = ctx.alloc::<u32>(0)?;
    grow_output(ctx, &mut stack, 1)?;
    stack.push(g);
    visited[g] = true;
    while let Some(region) = stack.pop() {
        let group = &out.groups[region];
        let start = group.first_cluster as usize;
        for c in &out.clusters[start..start + group.cluster_count as usize] {
            ctx.tick(1)?;
            if c.refined >= 0 {
                let child = c.refined as usize;
                if !visited[child] {
                    visited[child] = true;
                    grow_output(ctx, &mut stack, 1)?;
                    stack.push(child);
                }
            } else {
                let start = c.first_index as usize;
                let triangles = &out.indices[start..start + c.index_count as usize];
                ctx.tick(triangles.len())?;
                grow_output(ctx, &mut source, triangles.len())?;
                source.extend_from_slice(triangles);
            }
        }
    }
    let mut cursor = heads[g];
    while let Some(i) = cursor {
        let c = &out.clusters[i];
        let start = c.first_index as usize;
        let triangles = &out.indices[start..start + c.index_count as usize];
        ctx.tick(triangles.len())?;
        grow_output(ctx, &mut simplified, triangles.len())?;
        simplified.extend_from_slice(triangles);
        cursor = next[i];
    }
    let measured = region_error(&source, &simplified, source_positions, &out.positions, ctx)?;
    ctx.free(visited)?;
    ctx.free(stack)?;
    ctx.free(source)?;
    ctx.free(simplified)?;
    Ok(measured)
}

fn grow_output<T>(ctx: &mut Context<'_>, values: &mut Vec<T>, extra: usize) -> Result<(), Error> {
    let needed = values.len().checked_add(extra).ok_or(Error::SizeOverflow)?;
    if needed > values.capacity() {
        let capacity = needed.max(values.capacity().saturating_mul(2)).max(4);
        ctx.grow(values, capacity - values.len())?;
    }
    Ok(())
}

fn optimize_material_ranges(
    indices: &mut [u32],
    materials: &[u16],
    level: u8,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    if indices.len() > 768 || !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let mut first = 0;
    while first < indices.len() {
        let slot = materials[indices[first] as usize];
        let mut end = first + 3;
        while end < indices.len() && materials[indices[end] as usize] == slot {
            end += 3;
        }
        let mut vertices = [0u32; 256];
        let mut count = 0;
        let mut triangles = [0u8; 768];
        for (t, &v) in triangles.iter_mut().zip(&indices[first..end]) {
            ctx.tick(count + 1)?;
            let local = match vertices[..count].iter().position(|&old| old == v) {
                Some(i) => i,
                None => {
                    if count == vertices.len() {
                        return Err(Error::InvalidTopology);
                    }
                    vertices[count] = v;
                    count += 1;
                    count - 1
                }
            };
            *t = local as u8;
        }
        ctx.child(|ws| {
            crate::optimize_meshlet_level_in_place_generic(
                &mut vertices[..count],
                &mut triangles[..end - first],
                level,
                ws,
            )
        })?;
        for (dst, &local) in indices[first..end]
            .iter_mut()
            .zip(&triangles[..end - first])
        {
            *dst = vertices[local as usize];
        }
        first = end;
    }
    Ok(())
}

pub(super) fn step_error(
    input: &[u32],
    output: &[u32],
    positions: &[[f32; 3]],
    ctx: &mut Context<'_>,
) -> Result<f32, Error> {
    region_error(input, output, positions, positions, ctx)
}

fn region_error(
    input: &[u32],
    output: &[u32],
    positions: &[[f32; 3]],
    output_positions: &[[f32; 3]],
    ctx: &mut Context<'_>,
) -> Result<f32, Error> {
    let mut area = 0f64;
    ctx.tick(input.len())?;
    for t in input.as_chunks::<3>().0 {
        let [a, b, c] = t.map(|i| positions[i as usize].map(f64::from));
        let u = [0, 1, 2].map(|k| b[k] - a[k]);
        let v = [0, 1, 2].map(|k| c[k] - a[k]);
        let n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        area += libm::sqrt(n.iter().map(|x| x * x).sum::<f64>()) * 0.5;
    }
    let options = LodQualityOptions {
        samples_per_unit_area: 4000.0 / area.max(1e-12),
    };
    let mut measured = 0.0;
    ctx.child(|ws| {
        let q = crate::measure_lod_quality(
            LodQualityMesh::new(input, Positions::from_packed(positions)),
            LodQualityMesh::new(output, Positions::from_packed(output_positions)),
            &options,
            ws,
        )?;
        measured = super::bounds_math::f32_up(q.symmetric.max);
        Ok(())
    })?;
    Ok(measured)
}

/// Opt-in alpha silhouette guard, using nine fixed sphere-grid views (including
/// repeated seam directions), nearest repeated UV sampling, cutoff 0.5, and
/// two samples per axis. A failing step is stuck, never assigned a guessed error.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct AlphaCoverage<'a> {
    /// Mask per material slot; missing slots and `None` are opaque.
    pub masks: &'a [Option<crate::AlphaMask<'a>>],
    /// First of two consecutive UV components in the source attributes.
    pub uv_component: usize,
    /// Inclusive output/input covered-area band (default 0.93 to 1.07).
    pub ratio_band: [f64; 2],
    /// Maximum mean absolute 3x3 box-blurred alpha difference (default 0.12).
    pub blurred_difference: f64,
    /// Pixels per view side, 8 to 128 (default 32).
    pub resolution: u32,
}
impl<'a> AlphaCoverage<'a> {
    /// Default guard for two consecutive UV channels and caller material masks.
    pub const fn new(masks: &'a [Option<crate::AlphaMask<'a>>], uv_component: usize) -> Self {
        Self {
            masks,
            uv_component,
            ratio_band: [0.93, 1.07],
            blurred_difference: 0.12,
            resolution: 32,
        }
    }
    fn validate(self, components: usize) -> Result<(), Error> {
        if self
            .uv_component
            .checked_add(2)
            .is_none_or(|end| end > components)
        {
            return Err(Error::InvalidLayout);
        }
        if !self.ratio_band.iter().all(|x| x.is_finite())
            || self.ratio_band[0] < 0.0
            || self.ratio_band[0] > 1.0
            || self.ratio_band[1] < 1.0
            || !self.blurred_difference.is_finite()
            || !(0.0..=1.0).contains(&self.blurred_difference)
            || !(8..=128).contains(&self.resolution)
        {
            return Err(Error::InvalidParameter);
        }
        Ok(())
    }
}

pub(super) fn alpha_failed(
    input: &[u32],
    output: &[u32],
    mesh: &MeshGeneric<'_>,
    slot: Option<usize>,
    alpha: AlphaCoverage<'_>,
    ctx: &mut Context<'_>,
) -> Result<bool, Error> {
    let attrs = mesh.attributes.ok_or(Error::InvalidLayout)?;
    ctx.tick(input.len())?;
    let has_mask = input.iter().any(|&v| {
        let m = slot
            .and_then(|s| attrs.get(v as usize, s))
            .map_or(0, |x| x as usize);
        alpha.masks.get(m).is_some_and(Option::is_some)
    });
    if !has_mask {
        return Ok(false);
    }
    let mut uvs = ctx.alloc::<[f32; 2]>(mesh.positions.len())?;
    ctx.tick(uvs.len())?;
    for (v, uv) in uvs.iter_mut().enumerate() {
        *uv = [
            attrs
                .get(v, alpha.uv_component)
                .ok_or(Error::InvalidLayout)?,
            attrs
                .get(v, alpha.uv_component + 1)
                .ok_or(Error::InvalidLayout)?,
        ];
    }
    // Use the input framing for both renders; independently fitting the output
    // would conceal shrinkage. Source-referenced vertices stay inside this sphere.
    let mut options = crate::ImpostorOptions {
        grid: crate::ImpostorGrid::new(crate::ImpostorLayout::Sphere, 3)?,
        view_resolution: alpha.resolution,
        dilation: 0,
        coverage_threshold: 1.0,
        ..crate::ImpostorOptions::default()
    };
    let mut bounds = None;
    let mut render = |indices: &[u32], ctx: &mut Context<'_>| {
        options.bounds = bounds;
        ctx.child(|ws| {
            let sampler = |t: usize, uv: [f32; 2]| {
                let m = slot
                    .and_then(|s| attrs.get(indices[t * 3] as usize, s))
                    .map_or(0, |x| x as usize);
                let a = alpha.masks.get(m).copied().flatten().map_or(1.0, |mask| {
                    let x = (f64::from(uv[0]) - libm::floor(f64::from(uv[0])))
                        * f64::from(mask.width());
                    let y = (f64::from(uv[1]) - libm::floor(f64::from(uv[1])))
                        * f64::from(mask.height());
                    mask.alpha(
                        (x as u32).min(mask.width() - 1),
                        (y as u32).min(mask.height() - 1),
                    )
                    .map_or(0.0, f32::from)
                        / 255.0
                });
                [1.0, 1.0, 1.0, a]
            };
            let atlas = crate::bake_impostor(
                indices,
                Positions::from_packed(mesh.positions),
                None,
                crate::ImpostorAlbedo::Texture {
                    uvs: &uvs,
                    sampler: &sampler,
                },
                options,
                ws,
            )?;
            bounds = Some(atlas.bounds);
            Ok(atlas.albedo_alpha)
        })
    };
    let before = render(input, ctx)?;
    let after = render(output, ctx)?;
    let r = alpha.resolution as usize;
    let size = r * 3;
    ctx.tick(9 * r * r * 20)?;
    let mut failed = false;
    for view in 0..9 {
        let (ox, oy) = ((view % 3) * r, (view / 3) * r);
        let mut sums = [0u64; 2];
        let mut diff = 0.0;
        for y in 0..r {
            for x in 0..r {
                let i = (oy + y) * size + ox + x;
                sums[0] += u64::from(before[i][3]);
                sums[1] += u64::from(after[i][3]);
                let mut blur = [0i32; 2];
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        let xx = x.saturating_add_signed(dx).min(r - 1);
                        let yy = y.saturating_add_signed(dy).min(r - 1);
                        let i = (oy + yy) * size + ox + xx;
                        blur[0] += i32::from(before[i][3]);
                        blur[1] += i32::from(after[i][3]);
                    }
                }
                diff += f64::from((blur[0] - blur[1]).abs()) / (9.0 * 255.0);
            }
        }
        // An empty reference view must stay empty; do not divide by zero.
        let ratio = if sums[0] == 0 {
            if sums[1] == 0 {
                1.0
            } else {
                f64::INFINITY
            }
        } else {
            sums[1] as f64 / sums[0] as f64
        };
        failed |= ratio < alpha.ratio_band[0]
            || ratio > alpha.ratio_band[1]
            || diff / (r * r) as f64 > alpha.blurred_difference;
    }
    ctx.free(before)?;
    ctx.free(after)?;
    ctx.free(uvs)?;
    Ok(failed)
}

/// Coarsest fixed-error extraction and sampled fallback quality. This is a
/// measured lower estimate of surface deviation, never a ray-error certificate.
#[derive(Debug)]
#[non_exhaustive]
pub struct FallbackReport {
    /// Indices of every parentless group's clusters, including stuck regions.
    pub indices: Vec<u32>,
    /// Symmetric sampled source/fallback deviation.
    pub quality: crate::LodQuality,
}
/// Extract the coarsest complete mesh and measure it against source geometry.
///
/// # Errors
/// Invalid DAG topology, mesh inputs, quality settings, or workspace exhaustion.
pub fn measure_fallback(
    built: &Built,
    options: &crate::LodQualityOptions,
    workspace: &mut Workspace,
) -> Result<FallbackReport, Error> {
    let mut ctx = Context::new(workspace)?;
    let mut parents = ctx.alloc::<bool>(built.groups.len())?;
    ctx.tick(built.clusters.len())?;
    for c in &built.clusters {
        if c.refined >= 0 {
            *parents
                .get_mut(c.refined as usize)
                .ok_or(Error::InvalidTopology)? = true;
        }
    }
    let mut source = ctx.alloc::<u32>(0)?;
    let mut indices = ctx.alloc::<u32>(0)?;
    for c in &built.clusters {
        ctx.tick(1)?;
        let end = c
            .first_index
            .checked_add(c.index_count)
            .ok_or(Error::SizeOverflow)?;
        let t = built
            .indices
            .get(c.first_index as usize..end as usize)
            .ok_or(Error::InvalidTopology)?;
        if !*parents
            .get(c.group as usize)
            .ok_or(Error::InvalidTopology)?
        {
            ctx.tick(t.len())?;
            grow_output(&mut ctx, &mut indices, t.len())?;
            indices.extend_from_slice(t);
        }
        if c.refined < 0 {
            ctx.tick(t.len())?;
            grow_output(&mut ctx, &mut source, t.len())?;
            source.extend_from_slice(t);
        }
    }
    let mut quality = None;
    ctx.child(|ws| {
        quality = Some(crate::measure_lod_quality(
            LodQualityMesh::new(&source, Positions::from_packed(&built.positions)),
            LodQualityMesh::new(&indices, Positions::from_packed(&built.positions)),
            options,
            ws,
        )?);
        Ok(())
    })?;
    ctx.free(source)?;
    ctx.free(parents)?;
    Ok(FallbackReport {
        indices,
        quality: quality.ok_or(Error::InvalidTopology)?,
    })
}

#[cfg(test)]
mod source_error_tests {
    use super::*;

    #[test]
    fn successive_unit_steps_bound_original_region() {
        let positions: Vec<_> = (0..3)
            .flat_map(|z| [[0., 0., z as f32], [1., 0., z as f32], [0., 1., z as f32]])
            .collect();
        let a: Vec<u32> = (0..9).collect();
        let b: Vec<u32> = (3..9).collect();
        let c: Vec<u32> = (6..9).collect();
        let mut workspace = Workspace::default();
        let mut ctx = Context::new(&mut workspace).expect("workspace");
        assert_eq!(step_error(&a, &b, &positions, &mut ctx), Ok(1.));
        assert_eq!(step_error(&b, &c, &positions, &mut ctx), Ok(1.));
        assert_eq!(step_error(&a, &c, &positions, &mut ctx), Ok(2.));
        let mut built = Built {
            positions: positions.clone(),
            vertex_source: (0..9).collect(),
            indices: [a, b, c].concat(),
            ..Built::default()
        };
        for (g, (first, count)) in [(0, 9), (9, 6), (15, 3)].into_iter().enumerate() {
            built.groups.push(FlatDagGroup {
                first_cluster: g as u32,
                cluster_count: 1,
                error: if g == 2 { f32::MAX } else { 1. },
                ..FlatDagGroup::default()
            });
            built.clusters.push(FlatDagCluster {
                first_index: first,
                index_count: count,
                group: g as u32,
                refined: g as i32 - 1,
                ..FlatDagCluster::default()
            });
        }
        source_error_floor(&mut built, &positions, &mut ctx).expect("source measurement");
        assert_eq!(built.groups[0].error, 1.);
        assert_eq!(built.groups[1].error, 2.);
        assert_eq!(built.groups[2].error, f32::MAX);
    }
}
