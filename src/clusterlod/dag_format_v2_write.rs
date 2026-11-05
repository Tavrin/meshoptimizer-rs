//! MDAG 2 writer and build key (requires the `clusterlod` feature).
#[allow(unused_imports)] // MIN/MAX_PAGE_CAPACITY are named by rustdoc links.
use super::{
    parent_lists, prefix_hash, xxh64, DependencyLists, FarField, PageLimits, Xxh64,
    ATTRIBUTE_HEADER_BYTES, CHUNK_ALIGN, CHUNK_ENTRY_BYTES, CHUNK_REQUIRED, CLUSTER_REFINED_BYTES,
    CLUSTER_SPHERE_BYTES, DEFAULT_MAX_PAGE_CLUSTERS, DEFAULT_PAGE_CAPACITY, FAR_FIELD_BYTES,
    GROUP_BYTES, HEADER_BYTES, MAJOR, MAX_BUFFER_VIEW_BYTES, MAX_LEVELS, MAX_PAGE_CAPACITY,
    MESH_BYTES, MINOR, MIN_PAGE_CAPACITY, NODE_BYTES, PAGE_BYTES, PAGE_PINNED, RESIDENT_HASH_FIELD,
    TAG_ATTRIBUTES, TAG_CLUSTER_REFINED, TAG_CLUSTER_SPHERES, TAG_DEPENDENCIES, TAG_FAR_FIELD,
    TAG_GROUPS, TAG_MATERIALS, TAG_MESH, TAG_NODES, TAG_PAGES,
};
use crate::clusterlod::bounds_math::{add_up, containing_radius, f32_up};
use crate::clusterlod::cut::{self, CutError, MonotonicReport};
use crate::clusterlod::page::{
    encode_disk_page, encode_page, DiskCodec, PageCluster, QuantGrid, MAX_CLUSTER_TRIANGLES,
    MAX_CLUSTER_VERTICES, PAGE_HEADER_WORDS,
};
use crate::clusterlod::{
    build_hierarchy, dag_format::Error, Config, FlatDagCluster, FlatDagGroup, Group, LodBounds,
    Node,
};
use crate::{Attributes, VertexFlags, Workspace};
use alloc::vec::Vec;

/// Bumped whenever equal inputs produce different bytes; part of [`build_key`].
/// Revision 2 added the `CREF` chunk and extended the resident hash over the
/// header. Revision 3 adds the measured build policy, attributes and materials.
/// Revision 4 measures replacements against their original source regions.
/// Revision 5 keeps replacement siblings in one parent source region.
pub const BUILDER_REVISION: u32 = 5;
/// Default culling-hierarchy node width.
pub const DEFAULT_NODE_WIDTH: usize = 4;

/// Flat DAG tables to serialize: the successful prefixes of
/// [`build_dag_into`](crate::clusterlod::build_dag_into), groups in emission
/// order (fine to coarse). Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Source<'a, I: crate::Index = u32> {
    /// Groups in emission order.
    pub groups: &'a [FlatDagGroup],
    /// Clusters in emission order.
    pub clusters: &'a [FlatDagCluster],
    /// Vertex snapshot positions.
    pub positions: &'a [[f32; 3]],
    /// Flat topology referencing `positions`.
    pub indices: &'a [I],
    /// Attribute layout, referencing `attributes` component offsets.
    pub channels: &'a [crate::clusterlod::attributes::Channel],
    /// Attribute values in the same vertex order as `positions`.
    pub attributes: Option<Attributes<'a>>,
    /// Material slot per vertex; every triangle must have one uniform slot.
    pub materials: Option<&'a [u16]>,
}

impl<'a, I: crate::Index> Source<'a, I> {
    /// Tables for [`write`].
    pub const fn new(
        groups: &'a [FlatDagGroup],
        clusters: &'a [FlatDagCluster],
        positions: &'a [[f32; 3]],
        indices: &'a [I],
    ) -> Self {
        Self {
            groups,
            clusters,
            positions,
            indices,
            channels: &[],
            attributes: None,
            materials: None,
        }
    }
}

/// Writer settings. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct WriteOptions {
    /// Position grid; snap the mesh to it before building (see
    /// [`QuantGrid::snap_positions`]).
    pub grid: QuantGrid,
    /// Page capacity in bytes of the memory form: a power of two from
    /// [`MIN_PAGE_CAPACITY`] to [`MAX_PAGE_CAPACITY`].
    pub page_capacity: usize,
    /// Most clusters per page, at least 1; values above
    /// [`PageLimits::cluster_bound`] of the capacity are clamped to it (the
    /// header records the clamped value).
    pub max_page_clusters: usize,
    /// Most vertices per cluster, 1 to 256; larger clusters are rejected.
    pub max_cluster_vertices: usize,
    /// Most triangles per cluster, 1 to 256; larger clusters are rejected.
    pub max_cluster_triangles: usize,
    /// Pin coarse pages in order while their memory form fits this many bytes
    /// (parentless groups are pinned regardless). `None` is one page.
    pub root_budget: Option<usize>,
    /// Culling-hierarchy node width, at least two.
    pub node_width: usize,
    /// Disk form of every page.
    pub disk_codec: DiskCodec,
    /// Optional caller-owned impostor handoff. Omitted by default.
    pub far_field: Option<FarField>,
    /// Key stored in the header, normally [`build_key`] of the build inputs.
    pub build_key: u64,
}

impl WriteOptions {
    /// Defaults: [`DEFAULT_PAGE_CAPACITY`], [`DEFAULT_MAX_PAGE_CLUSTERS`], 256
    /// vertices and triangles per cluster, a root budget of one page,
    /// [`DEFAULT_NODE_WIDTH`], [`DiskCodec::Compressed`] and build key zero.
    pub const fn new(grid: QuantGrid) -> Self {
        Self {
            grid,
            page_capacity: DEFAULT_PAGE_CAPACITY,
            max_page_clusters: DEFAULT_MAX_PAGE_CLUSTERS,
            max_cluster_vertices: MAX_CLUSTER_VERTICES,
            max_cluster_triangles: MAX_CLUSTER_TRIANGLES,
            root_budget: None,
            node_width: DEFAULT_NODE_WIDTH,
            disk_codec: DiskCodec::Compressed,
            build_key: 0,
            far_field: None,
        }
    }
    fn root_budget_bytes(&self) -> usize {
        self.root_budget.unwrap_or(self.page_capacity)
    }
    fn limits(&self) -> PageLimits {
        PageLimits {
            page_capacity: self.page_capacity,
            max_clusters: self
                .max_page_clusters
                .min(PageLimits::cluster_bound(self.page_capacity)),
            max_vertices: self.max_cluster_vertices,
            max_triangles: self.max_cluster_triangles,
        }
    }
}

/// What [`write`] did. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct WriteReport {
    /// Pages written.
    pub pages: usize,
    /// Pinned pages, a prefix of the page table.
    pub pinned_pages: usize,
    /// Memory-form bytes of the pinned pages; a large value is a quality signal.
    pub pinned_bytes: usize,
    /// Length of the resident prefix.
    pub resident_bytes: usize,
    /// Total disk-form bytes of all pages.
    pub disk_bytes: usize,
    /// Groups no other group was simplified from (always pinned).
    pub parentless_groups: usize,
    /// Parentless groups below the coarsest level: simplification got stuck there.
    pub stuck_groups: usize,
    /// Groups whose LOD sphere was grown to contain their decoded vertices.
    pub spheres_grown: usize,
    /// The repair of [`cut::enforce_monotonic`], always applied.
    pub monotonic: MonotonicReport,
}

/// Inputs of [`build_key`]. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct KeyInputs<'a, I: crate::Index = u32> {
    /// Builder configuration.
    pub config: &'a Config,
    /// Writer settings (their `build_key` field is ignored).
    pub options: &'a WriteOptions,
    /// Mesh positions as passed to the builder.
    pub positions: &'a [[f32; 3]],
    /// Mesh indices as passed to the builder.
    pub indices: &'a [I],
    /// Vertex flags as passed to the builder (`Mesh::vertex_lock`).
    pub vertex_lock: Option<&'a [VertexFlags]>,
    /// Attributes as passed to the builder (`Mesh::attributes`).
    pub attributes: Option<Attributes<'a>>,
    /// Attribute weights as passed to the builder.
    pub attribute_weights: &'a [f32],
    /// Attribute protect mask as passed to the builder.
    pub attribute_protect_mask: u32,
    /// Wire layout; every field participates in identity.
    pub channels: &'a [crate::clusterlod::attributes::Channel],
    /// Source per-triangle material slots.
    pub materials: Option<&'a [u16]>,
    /// Supplied source normals (the v2 builder uses weight 0.5).
    pub normals: Option<&'a [[f32; 3]]>,
    /// Opt-in alpha policy; masks and every setting participate in the key.
    pub alpha_coverage: Option<crate::clusterlod::v2::AlphaCoverage<'a>>,
    /// Whether the v2 sampled step and original-source error policy is used.
    pub measured_steps: bool,
}

impl<'a, I: crate::Index> KeyInputs<'a, I> {
    /// Inputs for [`build_key`] of the v2 measured policy without optional channels.
    /// Set the public fields for supplied channels, normals, materials or flags;
    /// set `measured_steps` false when keying a legacy builder result.
    pub const fn new(
        config: &'a Config,
        options: &'a WriteOptions,
        positions: &'a [[f32; 3]],
        indices: &'a [I],
    ) -> Self {
        Self {
            config,
            options,
            positions,
            indices,
            vertex_lock: None,
            attributes: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
            channels: &[],
            materials: None,
            normals: None,
            measured_steps: true,
            alpha_coverage: None,
        }
    }
}

/// XXH64 over [`BUILDER_REVISION`], the canonical bytes of the configuration
/// and writer settings, the bit patterns of the mesh positions and indices
/// (little-endian, indices widened to `u32`), and every other builder input
/// that changes simplification: vertex flags, attribute values, attribute
/// weights and the protect mask. Equal keys mean equal files, so a cooker can
/// test its cache before building. Not in C++ meshoptimizer.
pub fn build_key<I: crate::Index>(inputs: &KeyInputs<'_, I>) -> u64 {
    let mut h = Xxh64::new(0);
    let u32s = |h: &mut Xxh64, v: &[u32]| {
        for x in v {
            h.update(&x.to_le_bytes());
        }
    };
    h.update(b"MDAG2KEY");
    u32s(&mut h, &[BUILDER_REVISION]);
    let Config {
        max_vertices,
        min_triangles,
        max_triangles,
        partition_spatial,
        partition_sort,
        partition_size,
        cluster_spatial,
        cluster_fill_weight,
        cluster_split_factor,
        simplify_ratio,
        simplify_threshold,
        simplify_error_merge_previous,
        simplify_error_merge_additive,
        simplify_error_factor_sloppy,
        simplify_error_edge_limit,
        simplify_error_clamped,
        simplify_permissive,
        simplify_fallback_permissive,
        simplify_fallback_sloppy,
        simplify_regularize,
        simplify_preserve_folds,
        simplify_dilate_borders,
        optimize_bounds,
        optimize_clusters,
        optimize_clusters_level,
    } = *inputs.config;
    let z = |v: usize| v as u64;
    for v in [z(max_vertices), z(min_triangles), z(max_triangles)] {
        h.update(&v.to_le_bytes());
    }
    let b = u32::from;
    u32s(&mut h, &[b(partition_spatial), b(partition_sort)]);
    h.update(&z(partition_size).to_le_bytes());
    u32s(
        &mut h,
        &[
            b(cluster_spatial),
            cluster_fill_weight.to_bits(),
            cluster_split_factor.to_bits(),
            simplify_ratio.to_bits(),
            simplify_threshold.to_bits(),
            simplify_error_merge_previous.to_bits(),
            simplify_error_merge_additive.to_bits(),
            simplify_error_factor_sloppy.to_bits(),
            simplify_error_edge_limit.to_bits(),
            b(simplify_error_clamped),
            b(simplify_permissive),
            b(simplify_fallback_permissive),
            b(simplify_fallback_sloppy),
            b(simplify_regularize),
            b(simplify_preserve_folds),
            b(simplify_dilate_borders),
            b(optimize_bounds),
            b(optimize_clusters),
            u32::from(optimize_clusters_level),
        ],
    );
    if let Some(alpha) = inputs.alpha_coverage {
        h.update(b"ALPHA-L5-REJECT-1");
        h.update(&(alpha.uv_component as u64).to_le_bytes());
        h.update(&alpha.resolution.to_le_bytes());
        for v in [
            alpha.ratio_band[0],
            alpha.ratio_band[1],
            alpha.blurred_difference,
        ] {
            h.update(&v.to_le_bytes());
        }
        h.update(&(alpha.masks.len() as u64).to_le_bytes());
        for mask in alpha.masks {
            h.update(&[u8::from(mask.is_some())]);
            if let Some(mask) = mask {
                h.update(&mask.width().to_le_bytes());
                h.update(&mask.height().to_le_bytes());
                for y in 0..mask.height() {
                    for x in 0..mask.width() {
                        // Coordinates are inside the validated mask dimensions.
                        if let Some(alpha) = mask.alpha(x, y) {
                            h.update(&[alpha]);
                        }
                    }
                }
            }
        }
    }
    let o = inputs.options;
    if let Some(far) = o.far_field {
        h.update(b"FARF-L5-1");
        // Invalid settings cannot be written; still key every input bit.
        h.update(&far.caller_tag.to_le_bytes());
        h.update(&far.payload_hash.to_le_bytes());
        h.update(&[u8::from(far.grid.layout == crate::ImpostorLayout::Sphere)]);
        h.update(&far.grid.views_per_axis.to_le_bytes());
        h.update(&far.view_resolution.to_le_bytes());
        h.update(&far.texel_error.to_le_bytes());
        h.update(&far.view_error.to_le_bytes());
        h.update(&far.world_error.to_le_bytes());
    }
    let mut layout_hash = 0;
    if let Ok(value) = crate::clusterlod::attributes::layout_hash(inputs.channels) {
        layout_hash = value;
    }
    u32s(
        &mut h,
        &[
            o.grid.exponent() as u32,
            layout_hash,
            disk_codec_id(o.disk_codec),
        ],
    );
    for v in [
        z(o.page_capacity),
        z(o.max_page_clusters),
        z(o.max_cluster_vertices),
        z(o.max_cluster_triangles),
        z(o.root_budget_bytes()),
        z(o.node_width),
        z(inputs.positions.len()),
    ] {
        h.update(&v.to_le_bytes());
    }
    for p in inputs.positions {
        u32s(&mut h, &p.map(f32::to_bits));
    }
    h.update(&z(inputs.indices.len()).to_le_bytes());
    for i in inputs.indices {
        u32s(&mut h, &[i.to_u32()]);
    }
    // Optional inputs: a presence word, then a length and the values.
    match inputs.vertex_lock {
        Some(flags) => {
            u32s(&mut h, &[1]);
            h.update(&z(flags.len()).to_le_bytes());
            for f in flags {
                h.update(&[f.bits()]);
            }
        }
        None => u32s(&mut h, &[0]),
    }
    match inputs.attributes {
        Some(a) => {
            u32s(&mut h, &[1]);
            h.update(&z(a.len()).to_le_bytes());
            h.update(&z(a.components()).to_le_bytes());
            for v in 0..a.len() {
                for k in 0..a.components() {
                    u32s(&mut h, &[a.get(v, k).map_or(0, f32::to_bits)]);
                }
            }
        }
        None => u32s(&mut h, &[0]),
    }
    h.update(&z(inputs.attribute_weights.len()).to_le_bytes());
    for w in inputs.attribute_weights {
        u32s(&mut h, &[w.to_bits()]);
    }
    u32s(&mut h, &[inputs.attribute_protect_mask]);
    u32s(&mut h, &[u32::from(inputs.measured_steps)]);
    h.update(&z(inputs.channels.len()).to_le_bytes());
    for &ch in inputs.channels {
        // The writer rejects unsupported encodings represented by the sentinel.
        let mut encoding = u32::MAX;
        if let Ok(code) = ch.encoding.code() {
            encoding = code;
        }
        u32s(&mut h, &[ch.tag, encoding, ch.source_offset]);
    }
    u32s(&mut h, &[u32::from(inputs.materials.is_some())]);
    if let Some(values) = inputs.materials {
        h.update(&z(values.len()).to_le_bytes());
        for &v in values {
            h.update(&v.to_le_bytes());
        }
    }
    u32s(&mut h, &[u32::from(inputs.normals.is_some())]);
    if let Some(normals) = inputs.normals {
        h.update(&z(normals.len()).to_le_bytes());
        for &n in normals {
            u32s(&mut h, &n.map(f32::to_bits));
        }
    }
    h.finish()
}

fn disk_codec_id(c: DiskCodec) -> u32 {
    match c {
        DiskCodec::None => 0,
        DiskCodec::Compressed => 1,
    }
}

fn cut_error(e: CutError) -> Error {
    match e {
        CutError::Allocation => Error::Allocation,
        _ => Error::Invalid,
    }
}
fn crate_error(e: crate::Error) -> Error {
    match e {
        crate::Error::AllocationFailed => Error::Allocation,
        crate::Error::SizeOverflow | crate::Error::LimitExceeded => Error::Limit,
        _ => Error::Invalid,
    }
}
fn uint(v: usize) -> Result<u32, Error> {
    u32::try_from(v).map_err(|_| Error::Limit)
}
fn reserve<T>(v: &mut Vec<T>, n: usize) -> Result<(), Error> {
    v.try_reserve(n).map_err(|_| Error::Allocation)
}
fn filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| Error::Allocation)?;
    v.resize(n, value);
    Ok(v)
}
fn put(out: &mut Vec<u8>, words: &[u32]) {
    for w in words {
        out.extend_from_slice(&w.to_le_bytes());
    }
}
fn align(out: &mut Vec<u8>) {
    out.resize(out.len().next_multiple_of(CHUNK_ALIGN), 0);
}

/// Emission-order tables: groups partition clusters in order, clusters name
/// their group, refinements point to earlier groups, depths are below
/// [`MAX_LEVELS`] and non-decreasing, and index ranges lie in `indices`.
fn validate_source<I: crate::Index>(s: &Source<'_, I>) -> Result<(), Error> {
    if i32::try_from(s.groups.len()).is_err() || u32::try_from(s.clusters.len()).is_err() {
        return Err(Error::Limit);
    }
    let (mut next, mut depth) = (0usize, 0i32);
    for (g, group) in s.groups.iter().enumerate() {
        let first = group.first_cluster as usize;
        let end = first
            .checked_add(group.cluster_count as usize)
            .ok_or(Error::Invalid)?;
        if first != next
            || group.cluster_count == 0
            || end > s.clusters.len()
            || group.depth < depth
            || group.depth as u32 >= MAX_LEVELS
        {
            return Err(Error::Invalid);
        }
        depth = group.depth;
        for c in &s.clusters[first..end] {
            let refined_ok = c.refined == -1 || (c.refined >= 0 && (c.refined as usize) < g);
            let index_end = (c.first_index as usize).checked_add(c.index_count as usize);
            if c.group as usize != g
                || !refined_ok
                || c.index_count == 0
                || !c.index_count.is_multiple_of(3)
                || index_end.is_none_or(|e| e > s.indices.len())
            {
                return Err(Error::Invalid);
            }
        }
        next = end;
    }
    if next != s.clusters.len() {
        return Err(Error::Invalid);
    }
    Ok(())
}

/// Cluster-local decoded positions and local indices, in emission order.
struct Locals {
    vertex_start: Vec<u32>,
    index_start: Vec<u32>,
    positions: Vec<[f32; 3]>,
    sources: Vec<u32>,
    indices: Vec<u32>,
}

impl Locals {
    fn build<I: crate::Index>(s: &Source<'_, I>, grid: QuantGrid) -> Result<Self, Error> {
        let mut l = Self {
            vertex_start: Vec::new(),
            index_start: Vec::new(),
            positions: Vec::new(),
            sources: Vec::new(),
            indices: Vec::new(),
        };
        reserve(&mut l.vertex_start, s.clusters.len() + 1)?;
        reserve(&mut l.index_start, s.clusters.len() + 1)?;
        let mut remap = filled(s.positions.len(), u32::MAX)?;
        let mut touched = Vec::new();
        for c in s.clusters {
            l.vertex_start.push(uint(l.positions.len())?);
            l.index_start.push(uint(l.indices.len())?);
            let first = c.first_index as usize;
            let range = &s.indices[first..first + c.index_count as usize];
            reserve(&mut l.indices, range.len())?;
            let base = l.positions.len();
            for i in range {
                let i = i.to_u32() as usize;
                let slot = remap.get_mut(i).ok_or(Error::Invalid)?;
                if *slot == u32::MAX {
                    *slot = uint(l.positions.len() - base)?;
                    reserve(&mut l.positions, 1)?;
                    reserve(&mut touched, 1)?;
                    l.positions.push(grid.position(grid.snap(s.positions[i])?));
                    reserve(&mut l.sources, 1)?;
                    l.sources.push(uint(i)?);
                    touched.push(i);
                }
                l.indices.push(*slot);
            }
            for &i in &touched {
                remap[i] = u32::MAX;
            }
            touched.clear();
        }
        l.vertex_start.push(uint(l.positions.len())?);
        l.index_start.push(uint(l.indices.len())?);
        Ok(l)
    }
    fn positions(&self, c: usize) -> &[[f32; 3]] {
        &self.positions[self.vertex_start[c] as usize..self.vertex_start[c + 1] as usize]
    }
    fn indices(&self, c: usize) -> &[u32] {
        &self.indices[self.index_start[c] as usize..self.index_start[c + 1] as usize]
    }
}

#[derive(Clone, Copy)]
struct PageSpan {
    first: usize,
    count: usize,
    words: usize,
}

/// Serialize flat DAG tables as MDAG 2. Not in C++ meshoptimizer.
///
/// The writer snaps positions to `options.grid`, grows each group's LOD sphere
/// to contain its decoded vertices, applies [`cut::enforce_monotonic`], adds
/// [`QuantGrid::max_displacement`] to every group error below `f32::MAX`,
/// reorders groups coarse-first, packs clusters into pages (a group that fits
/// one page is never split), pins every parentless group plus coarse pages
/// within the root budget, records page dependencies and builds the culling
/// hierarchy. Output is deterministic and passes [`super::Reader::read_all`].
///
/// # Examples
/// ```
/// use meshoptimizer_rs::clusterlod::{dag_format::v2, page::QuantGrid};
/// let grid = QuantGrid::new(-10)?;
/// let source = v2::Source::<u32>::new(&[], &[], &[], &[]);
/// let mut workspace = meshoptimizer_rs::Workspace::default();
/// let (bytes, report) = v2::write(&source, &v2::WriteOptions::new(grid), &mut workspace)?;
/// assert_eq!(report.pages, 0);
/// let reader = v2::Reader::open(&bytes[..v2::resident_len(&bytes)?])?;
/// assert_eq!(reader.mesh().group_count, 0);
/// # Ok::<(), meshoptimizer_rs::clusterlod::dag_format::Error>(())
/// ```
///
/// # Errors
/// `Invalid` for inconsistent tables, settings outside their domain, clusters
/// above the vertex or triangle limit, or a missing DAG level; `Limit` for
/// coordinates beyond the grid, overflowing counts, or a resident prefix above
/// [`MAX_BUFFER_VIEW_BYTES`]; `Allocation` and `Codec` for allocation and codec
/// failures.
pub fn write<I: crate::Index>(
    source: &Source<'_, I>,
    options: &WriteOptions,
    workspace: &mut Workspace,
) -> Result<(Vec<u8>, WriteReport), Error> {
    let cap = options.page_capacity;
    let limits = options.limits();
    if !limits.valid() || options.node_width < 2 || u32::try_from(options.node_width).is_err() {
        return Err(Error::Invalid);
    }
    validate_source(source)?;
    let grid = options.grid;
    let q = grid.max_displacement();
    crate::clusterlod::attributes::validate_layout(source.channels)?;
    if source.channels.is_empty() != source.attributes.is_none()
        || source
            .attributes
            .is_some_and(|a| a.len() != source.positions.len())
        || source
            .materials
            .is_some_and(|a| a.len() != source.positions.len())
    {
        return Err(Error::Invalid);
    }
    let layout_hash = crate::clusterlod::attributes::layout_hash(source.channels)?;
    let locals = Locals::build(source, grid)?;
    let mut attribute_blocks = Vec::new();
    let mut material_slots = Vec::new();
    reserve(&mut attribute_blocks, source.clusters.len())?;
    reserve(&mut material_slots, source.clusters.len())?;
    for (c, cl) in source.clusters.iter().enumerate() {
        let vertices =
            &locals.sources[locals.vertex_start[c] as usize..locals.vertex_start[c + 1] as usize];
        let block = if let Some(a) = source.attributes {
            crate::clusterlod::attributes::encode(source.channels, a, vertices, workspace)?
        } else {
            Vec::new()
        };
        attribute_blocks.push(block);
        let mut slots = Vec::new();
        if let Some(m) = source.materials {
            let first = cl.first_index as usize;
            reserve(&mut slots, cl.index_count as usize / 3)?;
            for t in source.indices[first..first + cl.index_count as usize]
                .as_chunks::<3>()
                .0
            {
                let ids = t.map(|i| i.to_u32() as usize);
                let slot = *m.get(ids[0]).ok_or(Error::Invalid)?;
                if ids.iter().any(|&i| m.get(i) != Some(&slot)) {
                    return Err(Error::Invalid);
                }
                slots.push(slot);
            }
            if slots.windows(2).any(|s| s[0] > s[1]) {
                return Err(Error::Invalid);
            }
        }
        material_slots.push(slots);
    }
    let count = source.groups.len();
    let mut report = WriteReport::default();

    // Bounds in emission order: contain decoded vertices, then repair monotonicity.
    let mut fine = Vec::new();
    reserve(&mut fine, count)?;
    fine.extend_from_slice(source.groups);
    for g in &mut fine {
        let mut grown = false;
        for c in g.first_cluster as usize..(g.first_cluster + g.cluster_count) as usize {
            for &v in locals.positions(c) {
                let needed = containing_radius(g.center, v, 0.0);
                if needed > f64::from(g.radius) {
                    g.radius = f32_up(needed);
                    grown = true;
                }
            }
        }
        if !g.radius.is_finite() {
            return Err(Error::Limit);
        }
        report.spheres_grown += usize::from(grown);
    }
    report.monotonic = cut::enforce_monotonic(&mut fine, source.clusters).map_err(cut_error)?;
    for g in &mut fine {
        if g.error != f32::MAX {
            g.error = f32_up(add_up(f64::from(g.error), f64::from(q)));
            if !g.error.is_finite() {
                return Err(Error::Limit);
            }
        }
    }

    // Coarse-first order: group g becomes count - 1 - g.
    let flip = |g: usize| count - 1 - g;
    let mut groups = Vec::new();
    let mut order = Vec::new();
    let mut links = Vec::new();
    reserve(&mut groups, count)?;
    reserve(&mut order, source.clusters.len())?;
    reserve(&mut links, source.clusters.len())?;
    for g in 0..count {
        let mut group = fine[flip(g)];
        let first = group.first_cluster as usize;
        group.first_cluster = uint(order.len())?;
        for c in first..first + group.cluster_count as usize {
            let refined = source.clusters[c].refined;
            let refined = if refined < 0 {
                -1
            } else {
                flip(refined as usize) as i32
            };
            order.push(c);
            links.push((g as u32, refined));
        }
        groups.push(group);
    }

    // Size every cluster's memory form, then pack in order.
    let page_cluster = |k: usize| {
        let c = order[k];
        let mut cluster = PageCluster::new(
            links[k].0,
            links[k].1,
            locals.positions(c),
            locals.indices(c),
        );
        cluster.attributes = if attribute_blocks[c].is_empty() {
            None
        } else {
            Some(&attribute_blocks[c])
        };
        cluster.materials = if material_slots[c].is_empty() {
            None
        } else {
            Some(&material_slots[c])
        };
        cluster
    };
    let mut words = Vec::new();
    let mut spheres = Vec::new();
    reserve(&mut words, order.len())?;
    reserve(&mut spheres, order.len() * 4)?;
    for (k, &c) in order.iter().enumerate() {
        if locals.positions(c).len() > limits.max_vertices
            || locals.indices(c).len() > 3 * limits.max_triangles
        {
            return Err(Error::Invalid);
        }
        let page = encode_page(grid, &[page_cluster(k)], layout_hash, workspace)?;
        words.push(page.len() - PAGE_HEADER_WORDS);
        // Descriptor words 10-13: the tight sphere, repeated in `CSPH`.
        let at = PAGE_HEADER_WORDS + 10;
        spheres.extend_from_slice(page.get(at..at + 4).ok_or(Error::Invalid)?);
    }
    let capacity_words = cap / 4;
    let max_clusters = limits.max_clusters;
    let mut pages: Vec<PageSpan> = Vec::new();
    let mut spans = Vec::new();
    reserve(&mut spans, count)?;
    for group in &groups {
        let range =
            group.first_cluster as usize..(group.first_cluster + group.cluster_count) as usize;
        let total: usize = words[range.clone()].iter().sum();
        let n = range.len();
        let fits_open = pages
            .last()
            .is_some_and(|p| p.words + total <= capacity_words && p.count + n <= max_clusters);
        if !fits_open && PAGE_HEADER_WORDS + total <= capacity_words && n <= max_clusters {
            reserve(&mut pages, 1)?;
            pages.push(PageSpan {
                first: range.start,
                count: 0,
                words: PAGE_HEADER_WORDS,
            });
        }
        let mut first_page = None;
        for k in range {
            if pages
                .last()
                .is_none_or(|p| p.words + words[k] > capacity_words || p.count == max_clusters)
            {
                if PAGE_HEADER_WORDS + words[k] > capacity_words {
                    return Err(Error::Limit);
                }
                reserve(&mut pages, 1)?;
                pages.push(PageSpan {
                    first: k,
                    count: 0,
                    words: PAGE_HEADER_WORDS,
                });
            }
            let last = pages.len() - 1;
            pages[last].count += 1;
            pages[last].words += words[k];
            first_page.get_or_insert(last);
        }
        let first_page = first_page.ok_or(Error::Invalid)?;
        spans.push((uint(first_page)?, uint(pages.len() - first_page)?));
    }
    uint(pages.len())?;

    // Parents, dependencies and the pinned prefix.
    let (offsets, parents) = parent_lists(count, &links)?;
    let mut deps = Vec::new();
    let mut dep_ranges = Vec::new();
    let mut lists = DependencyLists::new(&offsets, &parents)?;
    reserve(&mut dep_ranges, pages.len())?;
    for (p, span) in pages.iter().enumerate() {
        let first = links[span.first].0;
        let last = links[span.first + span.count - 1].0;
        let list = lists.next(
            p as u32,
            first..last + 1,
            |g| {
                let (f, n) = spans[g as usize];
                Ok(f..f + n)
            },
            usize::MAX,
        )?;
        dep_ranges.push((uint(deps.len())?, uint(list.len())?));
        reserve(&mut deps, list.len())?;
        deps.extend_from_slice(list);
    }
    let mut pinned = 0usize;
    for (g, &(f, n)) in spans.iter().enumerate() {
        if offsets[g] == offsets[g + 1] {
            report.parentless_groups += 1;
            if groups[g].depth < groups[0].depth {
                report.stuck_groups += 1;
            }
            pinned = pinned.max((f + n) as usize);
        }
    }
    let (mut budget_pages, mut budget_bytes) = (0usize, 0usize);
    for p in &pages {
        budget_bytes += p.words * 4;
        if budget_bytes > options.root_budget_bytes() {
            break;
        }
        budget_pages += 1;
    }
    pinned = pinned.max(budget_pages);

    // Page bytes.
    let mut disk = Vec::new();
    let mut page_meta = Vec::new();
    reserve(&mut page_meta, pages.len())?;
    let mut clusters_buf = Vec::new();
    for span in &pages {
        clusters_buf.clear();
        reserve(&mut clusters_buf, span.count)?;
        clusters_buf.extend((span.first..span.first + span.count).map(page_cluster));
        let memory = encode_page(grid, &clusters_buf, layout_hash, workspace)?;
        if memory.len() != span.words {
            return Err(Error::Invalid);
        }
        let bytes = encode_disk_page(&memory, grid, options.disk_codec, workspace)?;
        if bytes.len() > MAX_BUFFER_VIEW_BYTES {
            return Err(Error::Limit);
        }
        page_meta.push((disk.len(), bytes.len(), xxh64(&bytes, 0)));
        reserve(&mut disk, bytes.len())?;
        disk.extend_from_slice(&bytes);
    }

    // Culling hierarchy over coarse-first groups, laid out breadth-first.
    let levels = groups.first().map_or(0, |g| g.depth as usize + 1);
    let lod = |g: &FlatDagGroup| LodBounds {
        center: g.center,
        radius: g.radius,
        error: g.error,
    };
    let (nodes, bvh_depth) = if count == 0 {
        (Vec::new(), 0)
    } else {
        let mut input = Vec::new();
        reserve(&mut input, count)?;
        input.extend(groups.iter().map(|g| Group {
            depth: g.depth,
            simplified: lod(g),
        }));
        let built =
            build_hierarchy(&input, options.node_width, levels, workspace).map_err(crate_error)?;
        breadth_first(&built, levels, &groups)?
    };

    // Mesh bounds over every decoded vertex.
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for v in &locals.positions {
        for i in 0..3 {
            lo[i] = lo[i].min(v[i]);
            hi[i] = hi[i].max(v[i]);
        }
    }
    if locals.positions.is_empty() {
        (lo, hi) = ([0.0; 3], [0.0; 3]);
    }
    let center = [0, 1, 2].map(|i| ((f64::from(lo[i]) + f64::from(hi[i])) * 0.5) as f32);
    let radius = locals
        .positions
        .iter()
        .map(|&v| containing_radius(center, v, 0.0))
        .fold(0.0, f64::max);
    let radius = f32_up(radius);
    if !radius.is_finite() {
        return Err(Error::Limit);
    }
    let (mut dag_triangles, mut leaf_triangles) = (0u64, 0u64);
    for c in source.clusters {
        let t = u64::from(c.index_count / 3);
        dag_triangles += t;
        if c.refined < 0 {
            leaf_triangles += t;
        }
    }

    // Resident layout.
    let material_count = source.materials.map_or(u32::from(!order.is_empty()), |m| {
        m.iter().copied().max().map_or(0, |x| u32::from(x) + 1)
    });
    let far_record = options.far_field.map(FarField::encode).transpose()?;
    let base_count = if source.materials.is_some() { 9 } else { 8 };
    let chunk_count = base_count + usize::from(far_record.is_some());
    let mut lengths = [
        MESH_BYTES,
        GROUP_BYTES * count,
        NODE_BYTES * nodes.len(),
        PAGE_BYTES * pages.len(),
        4 * deps.len(),
        ATTRIBUTE_HEADER_BYTES + source.channels.len() * 16,
        CLUSTER_SPHERE_BYTES * order.len(),
        CLUSTER_REFINED_BYTES * order.len(),
        16,
        0,
    ];
    if far_record.is_some() {
        lengths[base_count] = FAR_FIELD_BYTES;
    }
    let lengths = &lengths[..chunk_count];
    let mut tags = [
        TAG_MESH,
        TAG_GROUPS,
        TAG_NODES,
        TAG_PAGES,
        TAG_DEPENDENCIES,
        TAG_ATTRIBUTES,
        TAG_CLUSTER_SPHERES,
        TAG_CLUSTER_REFINED,
        TAG_MATERIALS,
        TAG_FAR_FIELD,
    ];
    if far_record.is_some() {
        tags[base_count] = TAG_FAR_FIELD;
    }
    let tags = &tags[..chunk_count];
    let mut chunk_at = [0usize; 10];
    let mut cursor = HEADER_BYTES + tags.len() * CHUNK_ENTRY_BYTES;
    for (at, len) in chunk_at.iter_mut().zip(lengths.iter().copied()) {
        *at = cursor;
        cursor = (cursor + len).next_multiple_of(CHUNK_ALIGN);
    }
    let resident = cursor;
    if resident > MAX_BUFFER_VIEW_BYTES {
        return Err(Error::Limit);
    }
    let mut out = Vec::new();
    reserve(&mut out, resident + disk.len())?;
    out.extend_from_slice(b"MDAG");
    put(
        &mut out,
        &[
            MAJOR,
            MINOR,
            tags.len() as u32,
            pages.len() as u32,
            cap as u32,
            resident as u32,
            limits.max_clusters as u32,
            0,
            0,
            0,
            0,
            limits.max_vertices as u32,
            limits.max_triangles as u32,
        ],
    );
    out.resize(HEADER_BYTES, 0);
    for ((tag, at), len) in tags.iter().zip(chunk_at).zip(lengths.iter().copied()) {
        out.extend_from_slice(tag);
        put(
            &mut out,
            &[
                if *tag == TAG_FAR_FIELD {
                    0
                } else {
                    CHUNK_REQUIRED
                },
                uint(at)?,
                uint(len)?,
            ],
        );
    }
    let f = f32::to_bits;
    put(
        &mut out,
        &[
            levels as u32,
            bvh_depth,
            options.node_width as u32,
            count as u32,
            order.len() as u32,
            uint(nodes.len())?,
            pages.len() as u32,
            pinned as u32,
            dag_triangles as u32,
            (dag_triangles >> 32) as u32,
            leaf_triangles as u32,
            (leaf_triangles >> 32) as u32,
            f(lo[0]),
            f(lo[1]),
            f(lo[2]),
            f(hi[0]),
            f(hi[1]),
            f(hi[2]),
            f(center[0]),
            f(center[1]),
            f(center[2]),
            f(radius),
            grid.exponent() as u32,
            f(q),
            layout_hash,
            material_count,
            0,
            0,
        ],
    );
    for (k, (g, &(first_page, page_count))) in groups.iter().zip(&spans).enumerate() {
        put(
            &mut out,
            &[
                f(g.center[0]),
                f(g.center[1]),
                f(g.center[2]),
                f(g.radius),
                f(g.error),
                g.depth as u32,
                g.first_cluster,
                g.cluster_count,
                first_page,
                page_count,
                offsets[k + 1] - offsets[k],
                0,
            ],
        );
    }
    align(&mut out);
    for n in &nodes {
        put(
            &mut out,
            &[
                f(n.bounds.center[0]),
                f(n.bounds.center[1]),
                f(n.bounds.center[2]),
                f(n.bounds.radius),
                f(n.bounds.error),
                n.group as u32,
                n.child_offset,
                n.child_count,
            ],
        );
    }
    align(&mut out);
    for (p, ((at, len, hash), (first_dep, dep_count))) in
        page_meta.iter().zip(&dep_ranges).enumerate()
    {
        let offset = (resident + at) as u64;
        let span = pages[p];
        put(
            &mut out,
            &[
                offset as u32,
                (offset >> 32) as u32,
                uint(*len)?,
                span.words as u32,
                span.first as u32,
                span.count as u32,
                *first_dep,
                *dep_count,
                *hash as u32,
                (*hash >> 32) as u32,
                if p < pinned { PAGE_PINNED } else { 0 },
                0,
            ],
        );
    }
    align(&mut out);
    put(&mut out, &deps);
    align(&mut out);
    put(&mut out, &[uint(source.channels.len())?, layout_hash, 0, 0]);
    for &ch in source.channels {
        put(&mut out, &ch.record()?);
    }
    align(&mut out);
    put(&mut out, &spheres);
    align(&mut out);
    for &(_, refined) in &links {
        out.extend_from_slice(&refined.to_le_bytes());
    }
    align(&mut out);
    if source.materials.is_some() {
        put(&mut out, &[material_count, 0, 0, 0]);
        align(&mut out);
    }
    if let Some(record) = far_record {
        out.extend_from_slice(&record);
        align(&mut out);
    }
    if out.len() != resident {
        return Err(Error::Invalid);
    }
    out[32..40].copy_from_slice(&options.build_key.to_le_bytes());
    let resident_hash = prefix_hash(&out).ok_or(Error::Invalid)?;
    out[RESIDENT_HASH_FIELD].copy_from_slice(&resident_hash.to_le_bytes());
    out.extend_from_slice(&disk);
    report.pages = pages.len();
    report.pinned_pages = pinned;
    report.pinned_bytes = pages[..pinned].iter().map(|p| p.words * 4).sum();
    report.resident_bytes = resident;
    report.disk_bytes = disk.len();
    Ok((out, report))
}

/// Lay a builder hierarchy out breadth-first from its per-level roots (so
/// children follow their parent), point leaves at the final group bounds, and
/// grow every internal node to contain its children in upward-rounded `f64`.
/// Returns the nodes and the longest root-to-leaf node count.
fn breadth_first(
    built: &[Node],
    levels: usize,
    groups: &[FlatDagGroup],
) -> Result<(Vec<Node>, u32), Error> {
    let mut source = Vec::new();
    reserve(&mut source, built.len())?;
    source.extend(0..levels.min(built.len()));
    let mut nodes = Vec::new();
    reserve(&mut nodes, built.len())?;
    let mut i = 0;
    while i < source.len() {
        let mut n = *built.get(source[i]).ok_or(Error::Invalid)?;
        if n.group >= 0 {
            let g = groups.get(n.group as usize).ok_or(Error::Invalid)?;
            n.bounds = LodBounds {
                center: g.center,
                radius: g.radius,
                error: g.error,
            };
        } else {
            let first = n.child_offset as usize;
            if source.len() + n.child_count as usize > built.len() {
                return Err(Error::Invalid);
            }
            n.child_offset = uint(source.len())?;
            source.extend(first..first + n.child_count as usize);
        }
        nodes.push(n);
        i += 1;
    }
    for i in (0..nodes.len()).rev() {
        let n = nodes[i];
        if n.group >= 0 {
            continue;
        }
        let mut bounds = n.bounds;
        let children = n.child_offset as usize..(n.child_offset + n.child_count) as usize;
        for child in &nodes[children] {
            let child = child.bounds;
            bounds.error = bounds.error.max(child.error);
            let needed = containing_radius(bounds.center, child.center, child.radius);
            if needed > f64::from(bounds.radius) {
                bounds.radius = f32_up(needed);
            }
        }
        if !bounds.radius.is_finite() {
            return Err(Error::Limit);
        }
        nodes[i].bounds = bounds;
    }
    let mut depth = filled(nodes.len(), 1u32)?;
    let mut longest = 0;
    for i in 0..nodes.len() {
        let n = nodes[i];
        longest = longest.max(depth[i]);
        for k in n.child_offset as usize..(n.child_offset + n.child_count) as usize {
            depth[k] = depth[i] + 1;
        }
    }
    Ok((nodes, longest))
}
