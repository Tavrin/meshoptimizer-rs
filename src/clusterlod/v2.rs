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
    /// Groups rejected by the retained-ratio, empty-output or mixed-slot rule.
    pub stuck_groups: usize,
    /// Groups rejected because simplification, including fallback, mixed material slots.
    pub material_stuck_groups: usize,
}
/// Build a snapped DAG with normals weighted by 0.5, protected material seams,
/// and a sampled symmetric simplification-step error floor (4000 area-scaled
/// interior samples, plus all corners). The floor is a measurement, not a
/// continuous Hausdorff certificate. Errors are accumulated monotonically.
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
    let (stuck_groups, material_stuck_groups) = super::build_in_context_mode(
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
        }),
    )?;
    out.stuck_groups = stuck_groups;
    out.material_stuck_groups = material_stuck_groups;
    ctx.free(topology)?;
    ctx.free(weights)?;
    ctx.free(locks)?;
    Ok(out)
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
            LodQualityMesh::new(output, Positions::from_packed(positions)),
            &options,
            ws,
        )?;
        measured = super::bounds_math::f32_up(q.symmetric.max);
        Ok(())
    })?;
    Ok(measured)
}
