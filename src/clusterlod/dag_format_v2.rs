//! MDAG 2: a chunked cluster-LOD container whose resident prefix opens without
//! reading any page. Not in C++ meshoptimizer.
//!
//! A file is `header | chunk table | resident chunks | pages`. The resident
//! prefix holds the group, node, page and dependency tables; pages hold the
//! clusters in the disk form of [`page`](super::super::page) and are fetched
//! and transcoded one at a time. Groups are stored coarse-first, so a page's
//! dependencies always have smaller page indices and every prefix of the page
//! table that contains the pinned pages is a valid residency set.
//!
//! [`Reader::open`] validates the resident prefix only; [`Reader::transcode_page`]
//! validates one page against it; [`Reader::read_all`] does both for a whole
//! file. Every byte is untrusted: failures are typed [`Error`]s, nothing
//! recurses, and allocations are bounded by the input length. Opening is linear
//! in the prefix; checking a page's dependency links costs at most its cluster
//! count times the largest page span of a refined group, times a logarithm.
//!
//! Wire specification: `docs/DAG_FORMAT.md`, section "MDAG 2".
use super::Error;
use crate::clusterlod::bounds_math::containing_radius;
use crate::clusterlod::page::{self, PageView, QuantGrid};
use crate::Workspace;
use alloc::vec::Vec;
use core::ops::Range;

#[cfg(feature = "clusterlod")]
#[path = "dag_format_v2_write.rs"]
mod writer;
#[cfg(feature = "clusterlod")]
pub use writer::{
    build_key, write, KeyInputs, Source, WriteOptions, WriteReport, BUILDER_REVISION,
    DEFAULT_NODE_WIDTH,
};

/// Major version written and accepted.
pub const MAJOR: u32 = 2;
/// Minor version written: the newest chunk set this crate knows. Readers
/// accept any minor; it is informational.
pub const MINOR: u32 = 0;
/// Header length in bytes.
pub const HEADER_BYTES: usize = 64;
/// Chunk table entry length in bytes.
pub const CHUNK_ENTRY_BYTES: usize = 16;
/// Alignment of every chunk offset and of the resident length.
pub const CHUNK_ALIGN: usize = 16;
/// Largest accepted chunk count.
pub const MAX_CHUNKS: usize = 64;
/// Largest accepted DAG level count.
pub const MAX_LEVELS: u32 = 64;
/// Smallest page capacity in bytes of the memory form.
pub const MIN_PAGE_CAPACITY: usize = 4096;
/// Largest page capacity in bytes of the memory form.
pub const MAX_PAGE_CAPACITY: usize = 1 << 20;
/// Default page capacity, 128 KiB of memory form.
pub const DEFAULT_PAGE_CAPACITY: usize = 128 * 1024;
/// Largest resident prefix and largest page disk form in bytes: each must fit
/// one storage-buffer binding of the smallest common limit (128 MiB).
pub const MAX_BUFFER_VIEW_BYTES: usize = 128 << 20;
/// Default limit on clusters per page. About 200 clusters of the measured
/// average size (`docs/PAGE_FORMAT.md`) fill a default page, so it rarely binds.
pub const DEFAULT_MAX_PAGE_CLUSTERS: usize = 256;
/// Chunk flag: a reader that does not know the tag must reject the file.
pub const CHUNK_REQUIRED: u32 = 1;
/// Page flag: the page is pinned (always resident).
pub const PAGE_PINNED: u32 = 1;

/// `MESH`: one 112-byte record of counts, bounds and the position grid.
pub const TAG_MESH: [u8; 4] = *b"MESH";
/// `GRPS`: one 48-byte record per group, coarse-first.
pub const TAG_GROUPS: [u8; 4] = *b"GRPS";
/// `NODE`: one 32-byte record per culling-hierarchy node.
pub const TAG_NODES: [u8; 4] = *b"NODE";
/// `PAGE`: one 48-byte record per page.
pub const TAG_PAGES: [u8; 4] = *b"PAGE";
/// `PDEP`: page dependency lists, one `u32` page index per entry.
pub const TAG_DEPENDENCIES: [u8; 4] = *b"PDEP";
/// `ATTR`: the vertex attribute layout (16 bytes plus 16 per channel).
pub const TAG_ATTRIBUTES: [u8; 4] = *b"ATTR";
/// `CSPH`: one 16-byte culling sphere per cluster (the tight sphere of the
/// cluster's page descriptor), so culling works while pages are absent.
pub const TAG_CLUSTER_SPHERES: [u8; 4] = *b"CSPH";
/// Reserved optional tag: per-node axis-aligned boxes.
pub const TAG_NODE_BOXES: [u8; 4] = *b"NAAB";
/// Reserved optional tag: far-field (impostor) parameters.
pub const TAG_FAR_FIELD: [u8; 4] = *b"FARF";
/// Reserved optional tag: source vertex identifiers kept in pages.
pub const TAG_SOURCE_IDS: [u8; 4] = *b"SRCI";

/// `MESH` record length.
pub const MESH_BYTES: usize = 112;
/// `GRPS` record length.
pub const GROUP_BYTES: usize = 48;
/// `NODE` record length.
pub const NODE_BYTES: usize = 32;
/// `PAGE` record length.
pub const PAGE_BYTES: usize = 48;
/// `PDEP` entry length.
pub const DEPENDENCY_BYTES: usize = 4;
/// `ATTR` header length.
pub const ATTRIBUTE_HEADER_BYTES: usize = 16;
/// `ATTR` channel record length.
pub const ATTRIBUTE_CHANNEL_BYTES: usize = 16;
/// `CSPH` record length.
pub const CLUSTER_SPHERE_BYTES: usize = 16;

const REQUIRED: [[u8; 4]; 7] = [
    TAG_MESH,
    TAG_GROUPS,
    TAG_NODES,
    TAG_PAGES,
    TAG_DEPENDENCIES,
    TAG_ATTRIBUTES,
    TAG_CLUSTER_SPHERES,
];
const UNSET: u8 = u8::MAX;

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

fn xxh_round(acc: u64, lane: u64) -> u64 {
    acc.wrapping_add(lane.wrapping_mul(P2))
        .rotate_left(31)
        .wrapping_mul(P1)
}
fn xxh_merge(acc: u64, v: u64) -> u64 {
    (acc ^ xxh_round(0, v)).wrapping_mul(P1).wrapping_add(P4)
}

/// Streaming XXH64 (the 64-bit xxHash of Yann Collet), implemented in the crate.
/// Used for page, resident and build-key hashes; it detects corruption, not
/// tampering. Not in C++ meshoptimizer.
#[derive(Clone, Debug)]
pub struct Xxh64 {
    seed: u64,
    lanes: [u64; 4],
    buffer: [u8; 32],
    buffered: usize,
    total: u64,
}

impl Xxh64 {
    /// A hasher with `seed`.
    pub const fn new(seed: u64) -> Self {
        Self {
            seed,
            lanes: [
                seed.wrapping_add(P1).wrapping_add(P2),
                seed.wrapping_add(P2),
                seed,
                seed.wrapping_sub(P1),
            ],
            buffer: [0; 32],
            buffered: 0,
            total: 0,
        }
    }

    fn stripe(&mut self, stripe: &[u8; 32]) {
        for (lane, bytes) in self.lanes.iter_mut().zip(stripe.as_chunks::<8>().0) {
            *lane = xxh_round(*lane, u64::from_le_bytes(*bytes));
        }
    }

    /// Append `data`.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total = self.total.wrapping_add(data.len() as u64);
        if self.buffered > 0 {
            let take = (32 - self.buffered).min(data.len());
            let (head, rest) = data.split_at(take);
            self.buffer[self.buffered..self.buffered + take].copy_from_slice(head);
            self.buffered += take;
            data = rest;
            if self.buffered < 32 {
                return;
            }
            let stripe = self.buffer;
            self.stripe(&stripe);
            self.buffered = 0;
        }
        let (stripes, rest) = data.as_chunks::<32>();
        for stripe in stripes {
            self.stripe(stripe);
        }
        self.buffer[..rest.len()].copy_from_slice(rest);
        self.buffered = rest.len();
    }

    /// The hash of everything appended so far.
    pub fn finish(&self) -> u64 {
        let [v1, v2, v3, v4] = self.lanes;
        let mut h = if self.total >= 32 {
            let h = v1
                .rotate_left(1)
                .wrapping_add(v2.rotate_left(7))
                .wrapping_add(v3.rotate_left(12))
                .wrapping_add(v4.rotate_left(18));
            xxh_merge(xxh_merge(xxh_merge(xxh_merge(h, v1), v2), v3), v4)
        } else {
            self.seed.wrapping_add(P5)
        };
        h = h.wrapping_add(self.total);
        let (eights, mut tail) = self.buffer[..self.buffered].as_chunks::<8>();
        for e in eights {
            h ^= xxh_round(0, u64::from_le_bytes(*e));
            h = h.rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
        }
        if let Some((four, rest)) = tail.split_first_chunk::<4>() {
            h ^= u64::from(u32::from_le_bytes(*four)).wrapping_mul(P1);
            h = h.rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
            tail = rest;
        }
        for &b in tail {
            h ^= u64::from(b).wrapping_mul(P5);
            h = h.rotate_left(11).wrapping_mul(P1);
        }
        h ^= h >> 33;
        h = h.wrapping_mul(P2);
        h ^= h >> 29;
        h = h.wrapping_mul(P3);
        h ^ (h >> 32)
    }
}

/// XXH64 of `data` with `seed`; see [`Xxh64`]. Not in C++ meshoptimizer.
pub fn xxh64(data: &[u8], seed: u64) -> u64 {
    let mut h = Xxh64::new(seed);
    h.update(data);
    h.finish()
}

/// Attribute layout hash: the low 32 bits of XXH64 (seed 0) over the `ATTR`
/// channel records. Pages and `MESH` repeat it.
pub(crate) fn attribute_layout_hash(channels: &[u8]) -> u32 {
    xxh64(channels, 0) as u32
}

fn size(v: u32) -> Result<usize, Error> {
    usize::try_from(v).map_err(|_| Error::Limit)
}
fn add(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_add(b).ok_or(Error::Limit)
}
fn mul(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_mul(b).ok_or(Error::Limit)
}
fn le32(b: &[u8], at: usize) -> Result<u32, Error> {
    let w = b
        .get(at..add(at, 4)?)
        .and_then(|w| w.first_chunk::<4>())
        .ok_or(Error::Invalid)?;
    Ok(u32::from_le_bytes(*w))
}
fn le64(b: &[u8], at: usize) -> Result<u64, Error> {
    Ok(u64::from(le32(b, at)?) | u64::from(le32(b, add(at, 4)?)?) << 32)
}
fn fl(b: &[u8], at: usize) -> Result<f32, Error> {
    Ok(f32::from_bits(le32(b, at)?))
}
fn vec3(b: &[u8], at: usize) -> Result<[f32; 3], Error> {
    Ok([fl(b, at)?, fl(b, at + 4)?, fl(b, at + 8)?])
}
fn finite3(v: [f32; 3]) -> bool {
    v.iter().all(|x| x.is_finite())
}
fn filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| Error::Allocation)?;
    v.resize(n, value);
    Ok(v)
}
fn sphere_ok(center: [f32; 3], radius: f32, error: f32) -> bool {
    finite3(center) && radius.is_finite() && radius >= 0.0 && error.is_finite() && error >= 0.0
}

/// Length of the resident prefix, read from the first [`HEADER_BYTES`] bytes
/// alone, so a runtime can fetch exactly the bytes [`Reader::open`] needs.
/// Not in C++ meshoptimizer.
///
/// # Errors
/// `Invalid` for a short or malformed header; `Unsupported` for another major
/// version; `Limit` for more than [`MAX_CHUNKS`] chunks or a prefix above
/// [`MAX_BUFFER_VIEW_BYTES`].
pub fn resident_len(header: &[u8]) -> Result<usize, Error> {
    let h = header.get(..HEADER_BYTES).ok_or(Error::Invalid)?;
    if h.get(..4) != Some(b"MDAG".as_slice()) {
        return Err(Error::Invalid);
    }
    if le32(h, 4)? != MAJOR {
        return Err(Error::Unsupported);
    }
    let chunks = size(le32(h, 12)?)?;
    if chunks > MAX_CHUNKS || size(le32(h, 24)?)? > MAX_BUFFER_VIEW_BYTES {
        return Err(Error::Limit);
    }
    let len = size(le32(h, 24)?)?;
    if !len.is_multiple_of(CHUNK_ALIGN) || len < HEADER_BYTES + chunks * CHUNK_ENTRY_BYTES {
        return Err(Error::Invalid);
    }
    Ok(len)
}

/// Page format parameters from the header, chosen by the writer, so a runtime
/// can size page slots and per-page tables before reading any page.
/// Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PageLimits {
    /// Page capacity in bytes of the memory form: a power of two from
    /// [`MIN_PAGE_CAPACITY`] to [`MAX_PAGE_CAPACITY`].
    pub page_capacity: usize,
    /// Most clusters in one page.
    pub max_clusters: usize,
    /// Most vertices in one cluster, at most 256.
    pub max_vertices: usize,
    /// Most triangles in one cluster, at most 256.
    pub max_triangles: usize,
}

impl PageLimits {
    /// Most clusters a page of `page_capacity` bytes can physically hold.
    pub const fn cluster_bound(page_capacity: usize) -> usize {
        (page_capacity / 4).saturating_sub(page::PAGE_HEADER_WORDS) / page::CLUSTER_WORDS
    }
    fn valid(&self) -> bool {
        self.page_capacity.is_power_of_two()
            && (MIN_PAGE_CAPACITY..=MAX_PAGE_CAPACITY).contains(&self.page_capacity)
            && (1..=Self::cluster_bound(self.page_capacity)).contains(&self.max_clusters)
            && (1..=page::MAX_CLUSTER_VERTICES).contains(&self.max_vertices)
            && (1..=page::MAX_CLUSTER_TRIANGLES).contains(&self.max_triangles)
    }
}

/// Whole-mesh record (`MESH`). Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct MeshInfo {
    /// Number of DAG levels; group depths are below it.
    pub level_count: u32,
    /// Node count of the longest root-to-leaf path of the culling hierarchy.
    pub bvh_depth: u32,
    /// Largest child count of a hierarchy node.
    pub node_width: u32,
    /// Group count.
    pub group_count: u32,
    /// Cluster count.
    pub cluster_count: u32,
    /// Hierarchy node count.
    pub node_count: u32,
    /// Page count.
    pub page_count: u32,
    /// Number of pinned pages, a prefix of the page table.
    pub pinned_page_count: u32,
    /// Triangles over all clusters of all levels.
    pub dag_triangles: u64,
    /// Triangles of the source-geometry clusters.
    pub leaf_triangles: u64,
    /// Minimum corner of the box containing every decoded vertex of every level.
    pub aabb_min: [f32; 3],
    /// Maximum corner of that box.
    pub aabb_max: [f32; 3],
    /// Center of a sphere containing every decoded vertex.
    pub sphere_center: [f32; 3],
    /// Radius of that sphere.
    pub sphere_radius: f32,
    /// Position grid of every page.
    pub grid: QuantGrid,
    /// Largest distance between a source position and its decoded position,
    /// [`QuantGrid::max_displacement`]; already included in every finite group error.
    pub max_displacement: f32,
    /// Attribute layout hash, repeated in every page header.
    pub attribute_layout_hash: u32,
    /// Number of material slots; every cluster's slot is below it.
    pub material_count: u32,
}

/// Group record (`GRPS`). Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct GroupRecord {
    /// LOD sphere center.
    pub center: [f32; 3],
    /// LOD sphere radius; the sphere contains every decoded vertex of the group
    /// and the LOD sphere of every group its clusters were simplified from.
    pub radius: f32,
    /// Group error, at least that of every refined group; parentless groups
    /// usually carry `f32::MAX`.
    pub error: f32,
    /// DAG level; non-increasing over the table.
    pub depth: u32,
    /// First cluster.
    pub first_cluster: u32,
    /// Cluster count, at least one.
    pub cluster_count: u32,
    /// First page holding one of its clusters.
    pub first_page: u32,
    /// Number of consecutive pages holding its clusters.
    pub page_count: u32,
    /// Number of distinct groups whose clusters were simplified from this one;
    /// zero for parentless groups, which are pinned.
    pub parent_count: u32,
}

impl GroupRecord {
    /// Pages holding this group's clusters.
    pub fn pages(&self) -> Range<u32> {
        self.first_page..self.first_page.saturating_add(self.page_count)
    }
    /// This group's clusters.
    pub fn clusters(&self) -> Range<u32> {
        self.first_cluster..self.first_cluster.saturating_add(self.cluster_count)
    }
}

/// Culling-hierarchy node record (`NODE`). Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct NodeRecord {
    /// Sphere center.
    pub center: [f32; 3],
    /// Sphere radius, containing every child's sphere (or the group's).
    pub radius: f32,
    /// Error, at least every child's (or the group's).
    pub error: f32,
    /// Group of a leaf, or -1 for an internal node.
    pub group: i32,
    /// First child; children follow their parent in the table.
    pub child_offset: u32,
    /// Child count; zero for leaves.
    pub child_count: u32,
}

/// Page record (`PAGE`). Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PageRecord {
    /// Byte offset of the disk form from the start of the file.
    pub disk_offset: u64,
    /// Byte length of the disk form.
    pub disk_length: u32,
    /// Words of the memory form, at most the page capacity.
    pub memory_words: u32,
    /// First cluster.
    pub first_cluster: u32,
    /// Cluster count, at least one.
    pub cluster_count: u32,
    /// First `PDEP` entry.
    pub first_dependency: u32,
    /// Number of `PDEP` entries.
    pub dependency_count: u32,
    /// XXH64 (seed 0) of the disk form.
    pub hash: u64,
    /// The page is pinned.
    pub pinned: bool,
}

impl PageRecord {
    /// Byte range of the disk form in the file.
    pub fn disk_range(&self) -> Range<u64> {
        self.disk_offset..self.disk_offset.saturating_add(u64::from(self.disk_length))
    }
}

/// Dependencies of one page, ascending. Not in C++ meshoptimizer.
#[derive(Clone, Debug)]
pub struct Dependencies<'a>(core::slice::Iter<'a, [u8; 4]>);

impl Iterator for Dependencies<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.0.next().map(|b| u32::from_le_bytes(*b))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl ExactSizeIterator for Dependencies<'_> {}

fn parse_group(b: &[u8]) -> Result<GroupRecord, Error> {
    Ok(GroupRecord {
        center: vec3(b, 0)?,
        radius: fl(b, 12)?,
        error: fl(b, 16)?,
        depth: le32(b, 20)?,
        first_cluster: le32(b, 24)?,
        cluster_count: le32(b, 28)?,
        first_page: le32(b, 32)?,
        page_count: le32(b, 36)?,
        parent_count: le32(b, 40)?,
    })
}
fn parse_node(b: &[u8]) -> Result<NodeRecord, Error> {
    Ok(NodeRecord {
        center: vec3(b, 0)?,
        radius: fl(b, 12)?,
        error: fl(b, 16)?,
        group: le32(b, 20)? as i32,
        child_offset: le32(b, 24)?,
        child_count: le32(b, 28)?,
    })
}
fn parse_page(b: &[u8]) -> Result<PageRecord, Error> {
    Ok(PageRecord {
        disk_offset: le64(b, 0)?,
        disk_length: le32(b, 8)?,
        memory_words: le32(b, 12)?,
        first_cluster: le32(b, 16)?,
        cluster_count: le32(b, 20)?,
        first_dependency: le32(b, 24)?,
        dependency_count: le32(b, 28)?,
        hash: le64(b, 32)?,
        pinned: le32(b, 40)? & PAGE_PINNED != 0,
    })
}

/// Distinct parents of every group in compressed rows (`offsets` has
/// `group_count + 1` entries), from `(owner, refined)` per cluster in cluster
/// order. Owners must be non-decreasing; parents come out ascending.
pub(crate) fn parent_lists(
    group_count: usize,
    links: &[(u32, i32)],
) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut offsets = filled(add(group_count, 1)?, 0u32)?;
    let mut last = filled(group_count, u32::MAX)?;
    for &(owner, refined) in links {
        if refined < 0 {
            continue;
        }
        let q = refined as usize;
        let slot = last.get_mut(q).ok_or(Error::Invalid)?;
        if *slot != owner {
            *slot = owner;
            offsets[q + 1] += 1;
        }
    }
    for q in 0..group_count {
        offsets[q + 1] += offsets[q];
    }
    let mut parents = filled(size(offsets[group_count])?, 0u32)?;
    let mut fill = filled(group_count, 0u32)?;
    fill.copy_from_slice(&offsets[..group_count]);
    last.fill(u32::MAX);
    for &(owner, refined) in links {
        if refined < 0 {
            continue;
        }
        let q = refined as usize;
        if last[q] != owner {
            last[q] = owner;
            parents[fill[q] as usize] = owner;
            fill[q] += 1;
        }
    }
    Ok((offsets, parents))
}

/// Dependencies of page `page`: every page of every parent of every group in
/// `groups`, excluding `page`; written sorted and unique into `out`.
pub(crate) fn page_dependencies(
    page: u32,
    groups: Range<u32>,
    offsets: &[u32],
    parents: &[u32],
    pages_of: impl Fn(u32) -> Result<Range<u32>, Error>,
    out: &mut Vec<u32>,
) -> Result<(), Error> {
    out.clear();
    for q in groups {
        let q = q as usize;
        let row = offsets.get(q..q + 2).ok_or(Error::Invalid)?;
        let list = parents
            .get(row[0] as usize..row[1] as usize)
            .ok_or(Error::Invalid)?;
        for &parent in list {
            for d in pages_of(parent)? {
                if d != page {
                    out.try_reserve(1).map_err(|_| Error::Allocation)?;
                    out.push(d);
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Chunk {
    tag: [u8; 4],
    flags: u32,
    offset: usize,
    len: usize,
}

fn chunk_entry(bytes: &[u8], i: usize) -> Result<Chunk, Error> {
    let e = add(HEADER_BYTES, mul(i, CHUNK_ENTRY_BYTES)?)?;
    let tag = *bytes
        .get(e..add(e, 4)?)
        .and_then(|t| t.first_chunk::<4>())
        .ok_or(Error::Invalid)?;
    Ok(Chunk {
        tag,
        flags: le32(bytes, e + 4)?,
        offset: size(le32(bytes, e + 8)?)?,
        len: size(le32(bytes, e + 12)?)?,
    })
}

/// Validated resident prefix of an MDAG 2 file, borrowing its bytes.
/// Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    resident_len: usize,
    minor: u32,
    limits: PageLimits,
    build_key: u64,
    resident_hash: u64,
    mesh: MeshInfo,
    chunk_count: usize,
    groups_at: usize,
    nodes_at: usize,
    pages_at: usize,
    deps_at: usize,
    dep_count: usize,
    spheres_at: usize,
}

impl<'a> Reader<'a> {
    /// Open from the resident prefix: `bytes` must hold at least
    /// [`resident_len`] bytes; pages are not read. Validates the header, chunk
    /// table and every resident table (rules 1-5 of `docs/DAG_FORMAT.md`).
    /// Allocates scratch proportional to the node and group counts, which are
    /// bounded by the prefix length.
    ///
    /// # Errors
    /// `Invalid` for a truncated or inconsistent prefix (including a resident
    /// hash mismatch); `Unsupported` for another major version, an unknown
    /// required chunk, unknown chunk flags or attribute channels;
    /// `Limit` for oversized counts; `Allocation` if scratch allocation fails.
    pub fn open(bytes: &'a [u8]) -> Result<Self, Error> {
        let resident_len = resident_len(bytes)?;
        let prefix = bytes.get(..resident_len).ok_or(Error::Invalid)?;
        let limits = PageLimits {
            page_capacity: size(le32(prefix, 20)?)?,
            max_clusters: size(le32(prefix, 28)?)?,
            max_vertices: size(le32(prefix, 48)?)?,
            max_triangles: size(le32(prefix, 52)?)?,
        };
        if !limits.valid() || prefix[56..HEADER_BYTES].iter().any(|&b| b != 0) {
            return Err(Error::Invalid);
        }
        let resident_hash = le64(prefix, 40)?;
        if xxh64(&prefix[HEADER_BYTES..], 0) != resident_hash {
            return Err(Error::Invalid);
        }
        let chunk_count = size(le32(prefix, 12)?)?;
        let mut known = [None::<(usize, usize)>; REQUIRED.len()];
        let mut cursor = HEADER_BYTES + chunk_count * CHUNK_ENTRY_BYTES;
        for i in 0..chunk_count {
            let Chunk {
                tag,
                flags,
                offset,
                len,
            } = chunk_entry(prefix, i)?;
            let end = add(offset, len)?;
            let duplicate = (0..i).any(|j| chunk_entry(prefix, j).is_ok_and(|c| c.tag == tag));
            if !tag.iter().all(u8::is_ascii_graphic)
                || !offset.is_multiple_of(CHUNK_ALIGN)
                || offset < cursor
                || end > resident_len
                || duplicate
            {
                return Err(Error::Invalid);
            }
            if flags & !CHUNK_REQUIRED != 0 {
                return Err(Error::Unsupported);
            }
            if prefix[cursor..offset].iter().any(|&b| b != 0) {
                return Err(Error::Invalid);
            }
            cursor = end;
            match REQUIRED.iter().position(|&t| t == tag) {
                Some(k) if flags & CHUNK_REQUIRED != 0 => known[k] = Some((offset, len)),
                Some(_) => return Err(Error::Invalid),
                None if flags & CHUNK_REQUIRED != 0 => return Err(Error::Unsupported),
                None => {}
            }
        }
        if prefix[cursor..].iter().any(|&b| b != 0) {
            return Err(Error::Invalid);
        }
        let mut at = [(0usize, 0usize); REQUIRED.len()];
        for (slot, k) in at.iter_mut().zip(known) {
            *slot = k.ok_or(Error::Invalid)?;
        }
        let [mesh_at, groups_at, nodes_at, pages_at, deps_at, attr_at, spheres_at] = at;
        let mesh = parse_mesh(prefix, mesh_at, le32(prefix, 16)?)?;
        let lengths = [
            (groups_at.1, GROUP_BYTES, mesh.group_count),
            (nodes_at.1, NODE_BYTES, mesh.node_count),
            (pages_at.1, PAGE_BYTES, mesh.page_count),
            (spheres_at.1, CLUSTER_SPHERE_BYTES, mesh.cluster_count),
        ];
        for (len, record, count) in lengths {
            if len != mul(record, size(count)?)? {
                return Err(Error::Invalid);
            }
        }
        if !deps_at.1.is_multiple_of(DEPENDENCY_BYTES) {
            return Err(Error::Invalid);
        }
        let attr = &prefix[attr_at.0..attr_at.0 + attr_at.1];
        let channels = size(le32(attr, 0)?)?;
        if channels != 0 {
            // Attribute channels are defined by a later minor version.
            return Err(Error::Unsupported);
        }
        if attr.len() != ATTRIBUTE_HEADER_BYTES
            || le32(attr, 8)? != 0
            || le32(attr, 12)? != 0
            || le32(attr, 4)? != attribute_layout_hash(&attr[ATTRIBUTE_HEADER_BYTES..])
            || le32(attr, 4)? != mesh.attribute_layout_hash
        {
            return Err(Error::Invalid);
        }
        let reader = Self {
            bytes,
            resident_len,
            minor: le32(prefix, 8)?,
            limits,
            build_key: le64(prefix, 32)?,
            resident_hash,
            mesh,
            chunk_count,
            groups_at: groups_at.0,
            nodes_at: nodes_at.0,
            pages_at: pages_at.0,
            deps_at: deps_at.0,
            dep_count: deps_at.1 / DEPENDENCY_BYTES,
            spheres_at: spheres_at.0,
        };
        for c in 0..mesh.cluster_count as usize {
            let (center, radius) = reader.cluster_sphere(c)?;
            if !sphere_ok(center, radius, 0.0) {
                return Err(Error::Invalid);
            }
        }
        reader.validate_pages()?;
        reader.validate_groups()?;
        reader.validate_nodes()?;
        Ok(reader)
    }

    /// Rule 4: pages.
    fn validate_pages(&self) -> Result<(), Error> {
        let mut disk_end = self.resident_len as u64;
        let (mut clusters, mut deps, mut pinned) = (0usize, 0usize, 0u32);
        let mut unpinned_seen = false;
        for p in 0..self.mesh.page_count as usize {
            let b = self.record(self.pages_at, PAGE_BYTES, p)?;
            let r = parse_page(b)?;
            let cc = size(r.cluster_count)?;
            let min_words = add(page::PAGE_HEADER_WORDS, mul(cc, page::CLUSTER_WORDS)?)?;
            let disk_end_p = r
                .disk_offset
                .checked_add(u64::from(r.disk_length))
                .ok_or(Error::Limit)?;
            if le32(b, 44)? != 0
                || le32(b, 40)? & !PAGE_PINNED != 0
                || r.disk_offset < disk_end
                || cc == 0
                || cc > self.limits.max_clusters
                || size(r.first_cluster)? != clusters
                || size(r.memory_words)? < min_words
                || mul(size(r.memory_words)?, 4)? > self.limits.page_capacity
                || u64::from(r.disk_length) < mul(cc, 64)? as u64
                || size(r.disk_length)? > MAX_BUFFER_VIEW_BYTES
                || size(r.first_dependency)? != deps
                || r.dependency_count as usize > p
            {
                return Err(Error::Invalid);
            }
            disk_end = disk_end_p;
            clusters = add(clusters, cc)?;
            deps = add(deps, size(r.dependency_count)?)?;
            if deps > self.dep_count {
                return Err(Error::Invalid);
            }
            let mut previous = None;
            for d in self.dependencies(p)? {
                // Sorted, unique and below the page: acyclic by one comparison,
                // and a pinned page (a prefix) depends only on pinned pages.
                if d as usize >= p || previous.is_some_and(|v| d <= v) {
                    return Err(Error::Invalid);
                }
                previous = Some(d);
            }
            if r.pinned {
                if unpinned_seen {
                    return Err(Error::Invalid);
                }
                pinned += 1;
            } else {
                unpinned_seen = true;
            }
        }
        if clusters != self.mesh.cluster_count as usize
            || deps != self.dep_count
            || pinned != self.mesh.pinned_page_count
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }

    /// Rule 2: groups.
    fn validate_groups(&self) -> Result<(), Error> {
        let mut clusters = 0usize;
        let mut previous_depth = u32::MAX;
        let pages = self.mesh.page_count;
        for g in 0..self.mesh.group_count as usize {
            let b = self.record(self.groups_at, GROUP_BYTES, g)?;
            let r = parse_group(b)?;
            let span_end = r
                .first_page
                .checked_add(r.page_count)
                .ok_or(Error::Invalid)?;
            if le32(b, 44)? != 0
                || !sphere_ok(r.center, r.radius, r.error)
                || r.depth >= self.mesh.level_count
                || r.depth > previous_depth
                || r.cluster_count == 0
                || size(r.first_cluster)? != clusters
                || r.page_count == 0
                || span_end > pages
                || r.parent_count as usize > g
                || (r.parent_count == 0 && span_end > self.mesh.pinned_page_count)
            {
                return Err(Error::Invalid);
            }
            previous_depth = r.depth;
            clusters = add(clusters, size(r.cluster_count)?)?;
            // The span is exactly the pages that hold one of the group's clusters.
            let first = self.page(r.first_page as usize)?;
            let last = self.page(span_end as usize - 1)?;
            let last_cluster = r
                .first_cluster
                .checked_add(r.cluster_count - 1)
                .ok_or(Error::Invalid)?;
            let holds = |p: &PageRecord, c: u32| {
                p.first_cluster <= c && c - p.first_cluster < p.cluster_count
            };
            if !holds(&first, r.first_cluster) || !holds(&last, last_cluster) {
                return Err(Error::Invalid);
            }
        }
        if clusters != self.mesh.cluster_count as usize {
            return Err(Error::Invalid);
        }
        Ok(())
    }

    /// Rule 3: the culling hierarchy is a forest with one root per level,
    /// children after their parent, every group in exactly one leaf, and bounds
    /// that contain their children's.
    fn validate_nodes(&self) -> Result<(), Error> {
        let n = self.mesh.node_count as usize;
        let levels = self.mesh.level_count as usize;
        let mut level = filled(n, UNSET)?;
        let mut depth = filled(n, 0u32)?;
        let mut seen = filled(self.mesh.group_count as usize, false)?;
        if n < levels {
            return Err(Error::Invalid);
        }
        for (i, (l, d)) in level.iter_mut().zip(&mut depth).take(levels).enumerate() {
            *l = i as u8;
            *d = 1;
        }
        let mut longest = 0u32;
        let mut leaves = 0usize;
        for i in 0..n {
            let r = self.node(i)?;
            if level[i] == UNSET || !sphere_ok(r.center, r.radius, r.error) {
                return Err(Error::Invalid);
            }
            let contains = |center: [f32; 3], radius: f32, error: f32| {
                containing_radius(r.center, center, radius) <= f64::from(r.radius)
                    && error <= r.error
            };
            if r.group >= 0 {
                let g = r.group as usize;
                let group = self.group(g)?;
                if r.child_offset != 0
                    || r.child_count != 0
                    || seen[g]
                    || group.depth != u32::from(level[i])
                    || !contains(group.center, group.radius, group.error)
                {
                    return Err(Error::Invalid);
                }
                seen[g] = true;
                leaves += 1;
                longest = longest.max(depth[i]);
                continue;
            }
            let first = size(r.child_offset)?;
            let end = add(first, size(r.child_count)?)?;
            if r.group != -1
                || r.child_count == 0
                || r.child_count > self.mesh.node_width
                || first <= i
                || end > n
            {
                return Err(Error::Invalid);
            }
            for k in first..end {
                let child = self.node(k)?;
                if level[k] != UNSET || !contains(child.center, child.radius, child.error) {
                    return Err(Error::Invalid);
                }
                level[k] = level[i];
                depth[k] = depth[i] + 1;
            }
        }
        if leaves != seen.len() || longest != self.mesh.bvh_depth {
            return Err(Error::Invalid);
        }
        Ok(())
    }

    fn record(&self, at: usize, record: usize, index: usize) -> Result<&'a [u8], Error> {
        let start = add(at, mul(record, index)?)?;
        self.bytes
            .get(start..add(start, record)?)
            .ok_or(Error::Invalid)
    }

    /// Length of the resident prefix in bytes.
    pub fn resident_len(&self) -> usize {
        self.resident_len
    }
    /// Minor version of the writer (informational).
    pub fn minor(&self) -> u32 {
        self.minor
    }
    /// Page capacity in bytes of the memory form.
    pub fn page_capacity(&self) -> usize {
        self.limits.page_capacity
    }
    /// Page format parameters from the header.
    pub fn page_limits(&self) -> PageLimits {
        self.limits
    }

    /// Culling sphere (center, radius) of cluster `c`, readable while its page
    /// is absent; it equals the tight sphere of the cluster's page descriptor
    /// and contains every decoded vertex of the cluster.
    ///
    /// # Errors
    /// `Invalid` if `c` is out of range.
    pub fn cluster_sphere(&self, c: usize) -> Result<([f32; 3], f32), Error> {
        if c >= self.mesh.cluster_count as usize {
            return Err(Error::Invalid);
        }
        let b = self.record(self.spheres_at, CLUSTER_SPHERE_BYTES, c)?;
        Ok((vec3(b, 0)?, fl(b, 12)?))
    }
    /// Build key the writer recorded (zero when none was given).
    pub fn build_key(&self) -> u64 {
        self.build_key
    }
    /// XXH64 (seed 0) of the resident prefix after the header.
    pub fn resident_hash(&self) -> u64 {
        self.resident_hash
    }
    /// The `MESH` record.
    pub fn mesh(&self) -> MeshInfo {
        self.mesh
    }

    /// Group `g` (coarse-first order).
    ///
    /// # Errors
    /// `Invalid` if `g` is out of range.
    pub fn group(&self, g: usize) -> Result<GroupRecord, Error> {
        if g >= self.mesh.group_count as usize {
            return Err(Error::Invalid);
        }
        parse_group(self.record(self.groups_at, GROUP_BYTES, g)?)
    }

    /// Hierarchy node `n`; nodes `0..level_count` are the per-level roots.
    ///
    /// # Errors
    /// `Invalid` if `n` is out of range.
    pub fn node(&self, n: usize) -> Result<NodeRecord, Error> {
        if n >= self.mesh.node_count as usize {
            return Err(Error::Invalid);
        }
        parse_node(self.record(self.nodes_at, NODE_BYTES, n)?)
    }

    /// Page `p`.
    ///
    /// # Errors
    /// `Invalid` if `p` is out of range.
    pub fn page(&self, p: usize) -> Result<PageRecord, Error> {
        if p >= self.mesh.page_count as usize {
            return Err(Error::Invalid);
        }
        parse_page(self.record(self.pages_at, PAGE_BYTES, p)?)
    }

    /// Pages that page `p` depends on, ascending; load them before `p` and
    /// evict `p` before them.
    ///
    /// # Errors
    /// `Invalid` if `p` is out of range.
    pub fn dependencies(&self, p: usize) -> Result<Dependencies<'a>, Error> {
        let r = self.page(p)?;
        let start = add(
            self.deps_at,
            mul(size(r.first_dependency)?, DEPENDENCY_BYTES)?,
        )?;
        let end = add(start, mul(size(r.dependency_count)?, DEPENDENCY_BYTES)?)?;
        let b = self.bytes.get(start..end).ok_or(Error::Invalid)?;
        Ok(Dependencies(b.as_chunks::<4>().0.iter()))
    }

    fn depends_on(&self, page: usize, on: u32) -> Result<bool, Error> {
        let deps = self.dependencies(page)?.0.as_slice();
        Ok(deps
            .binary_search_by(|d| u32::from_le_bytes(*d).cmp(&on))
            .is_ok())
    }

    /// Bytes of the first chunk tagged `tag`, including optional chunks this
    /// version skips (such as the reserved [`TAG_NODE_BOXES`], [`TAG_FAR_FIELD`]
    /// and [`TAG_SOURCE_IDS`]).
    pub fn chunk(&self, tag: [u8; 4]) -> Option<&'a [u8]> {
        let c = (0..self.chunk_count)
            .filter_map(|i| chunk_entry(self.bytes, i).ok())
            .find(|c| c.tag == tag)?;
        self.bytes.get(c.offset..c.offset.checked_add(c.len)?)
    }

    /// Validate the disk form of page `p` (rules 6-9) and write its memory form
    /// into `out`, which needs [`PageRecord::memory_words`] words. Returns a view
    /// of the written words. Uses no heap allocation; codec work is charged to
    /// `workspace`.
    ///
    /// # Errors
    /// `Invalid` for a length, hash or content mismatch; `Limit` if `out` is too
    /// short; `Unsupported` or `Codec` for codec failures.
    pub fn transcode_page<'o>(
        &self,
        p: usize,
        disk: &[u8],
        out: &'o mut [u32],
        workspace: &mut Workspace,
    ) -> Result<PageView<'o>, Error> {
        let r = self.page(p)?;
        if disk.len() as u64 != u64::from(r.disk_length) || xxh64(disk, 0) != r.hash {
            return Err(Error::Invalid);
        }
        if out.len() < r.memory_words as usize {
            return Err(Error::Limit);
        }
        let grid = self.mesh.grid;
        let m = page::transcode_page(disk, grid, out, workspace)?;
        let out: &'o [u32] = out;
        if m != r.memory_words as usize {
            return Err(Error::Invalid);
        }
        let view = PageView::new(&out[..m], grid)?;
        if view.cluster_count() != r.cluster_count as usize
            || view.attribute_layout_hash() != self.mesh.attribute_layout_hash
        {
            return Err(Error::Invalid);
        }
        self.validate_page_clusters(p, &r, &view)?;
        Ok(view)
    }

    /// Rules 7 and 8 for the clusters of one transcoded page.
    fn validate_page_clusters(
        &self,
        p: usize,
        r: &PageRecord,
        view: &PageView<'_>,
    ) -> Result<(), Error> {
        let page_id = p as u32;
        let mut owner = self.group_of_cluster(r.first_cluster)?;
        let mut owner_group = self.group(owner)?;
        // Recently checked (owner, refined) pairs: clusters of one owner usually
        // share a few refined groups, so each pair's page walk runs once.
        let mut checked = [(u32::MAX, -1i32); 16];
        let mut positions = [[0f32; 3]; page::MAX_CLUSTER_VERTICES];
        let (lo, hi) = (self.mesh.aabb_min, self.mesh.aabb_max);
        for k in 0..view.cluster_count() {
            let c = r.first_cluster + k as u32;
            while c - owner_group.first_cluster >= owner_group.cluster_count {
                owner += 1;
                owner_group = self.group(owner)?;
            }
            let d = view.cluster(k)?;
            let (center, radius) = self.cluster_sphere(c as usize)?;
            let same_sphere = center.map(f32::to_bits) == d.center.map(f32::to_bits)
                && radius.to_bits() == d.radius.to_bits();
            if d.group as usize != owner
                || !owner_group.pages().contains(&page_id)
                || d.material >= self.mesh.material_count
                || d.vertex_count as usize > self.limits.max_vertices
                || d.triangle_count as usize > self.limits.max_triangles
                || !same_sphere
            {
                return Err(Error::Invalid);
            }
            if d.refined >= 0 && !checked.contains(&(d.group, d.refined)) {
                checked[k % checked.len()] = (d.group, d.refined);
                let q = d.refined as usize;
                if q <= owner {
                    return Err(Error::Invalid);
                }
                let child = self.group(q)?;
                if child.depth >= owner_group.depth
                    || child.error > owner_group.error
                    || containing_radius(owner_group.center, child.center, child.radius)
                        > f64::from(owner_group.radius)
                    || child.parent_count == 0
                {
                    return Err(Error::Invalid);
                }
                for qp in child.pages() {
                    if qp != page_id && !self.depends_on(qp as usize, page_id)? {
                        return Err(Error::Invalid);
                    }
                }
            }
            let n = view.decode_positions(k, &mut positions)?;
            for v in &positions[..n] {
                let inside = (0..3).all(|i| lo[i] <= v[i] && v[i] <= hi[i]);
                if !inside
                    || containing_radius(owner_group.center, *v, 0.0)
                        > f64::from(owner_group.radius)
                {
                    return Err(Error::Invalid);
                }
            }
        }
        Ok(())
    }

    /// Group holding cluster `c` (groups partition the clusters in order).
    fn group_of_cluster(&self, c: u32) -> Result<usize, Error> {
        let (mut lo, mut hi) = (0usize, self.mesh.group_count as usize);
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if self.group(mid)?.first_cluster <= c {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        if self.group(lo)?.clusters().contains(&c) {
            Ok(lo)
        } else {
            Err(Error::Invalid)
        }
    }

    /// Open a whole file and validate every page as [`Self::transcode_page`]
    /// does, plus the cross-page facts a single page cannot show: exact parent
    /// counts, exact dependency lists, triangle totals, and that the last page
    /// ends the file. For tools and fuzzing. Scratch is bounded by the file
    /// length (every cluster occupies at least 64 disk bytes).
    ///
    /// # Errors
    /// As [`Self::open`] and [`Self::transcode_page`].
    pub fn read_all(file: &'a [u8], workspace: &mut Workspace) -> Result<Self, Error> {
        let r = Self::open(file)?;
        let mesh = r.mesh;
        let clusters = mesh.cluster_count as usize;
        if mul(clusters, 64)? > file.len() {
            return Err(Error::Invalid);
        }
        let mut words = filled(r.limits.page_capacity / 4, 0u32)?;
        let mut links = Vec::new();
        links
            .try_reserve_exact(clusters)
            .map_err(|_| Error::Allocation)?;
        let mut end = r.resident_len as u64;
        let (mut dag, mut leaf) = (0u64, 0u64);
        for p in 0..mesh.page_count as usize {
            let range = r.page(p)?.disk_range();
            let disk = usize::try_from(range.start)
                .ok()
                .zip(usize::try_from(range.end).ok())
                .and_then(|(s, e)| file.get(s..e))
                .ok_or(Error::Invalid)?;
            end = range.end;
            let view = r.transcode_page(p, disk, &mut words, workspace)?;
            for k in 0..view.cluster_count() {
                let d = view.cluster(k)?;
                let t = u64::from(d.triangle_count);
                dag += t;
                if d.refined < 0 {
                    leaf += t;
                }
                links.push((d.group, d.refined));
            }
        }
        if end != file.len() as u64 || dag != mesh.dag_triangles || leaf != mesh.leaf_triangles {
            return Err(Error::Invalid);
        }
        let (offsets, parents) = parent_lists(mesh.group_count as usize, &links)?;
        for g in 0..mesh.group_count as usize {
            if r.group(g)?.parent_count != offsets[g + 1] - offsets[g] {
                return Err(Error::Invalid);
            }
        }
        let mut expected = Vec::new();
        for p in 0..mesh.page_count as usize {
            let rec = r.page(p)?;
            let first = links
                .get(rec.first_cluster as usize)
                .ok_or(Error::Invalid)?
                .0;
            let last = links
                .get((rec.first_cluster + rec.cluster_count - 1) as usize)
                .ok_or(Error::Invalid)?
                .0;
            page_dependencies(
                p as u32,
                first..last + 1,
                &offsets,
                &parents,
                |g| Ok(r.group(g as usize)?.pages()),
                &mut expected,
            )?;
            if !r.dependencies(p)?.eq(expected.iter().copied()) {
                return Err(Error::Invalid);
            }
        }
        Ok(r)
    }
}

/// Rule 5: the `MESH` record.
fn parse_mesh(prefix: &[u8], (at, len): (usize, usize), pages: u32) -> Result<MeshInfo, Error> {
    if len != MESH_BYTES {
        return Err(Error::Invalid);
    }
    let b = &prefix[at..at + len];
    let w = |i: usize| le32(b, i * 4);
    let grid = QuantGrid::new(w(22)? as i32).map_err(|_| Error::Invalid)?;
    let m = MeshInfo {
        level_count: w(0)?,
        bvh_depth: w(1)?,
        node_width: w(2)?,
        group_count: w(3)?,
        cluster_count: w(4)?,
        node_count: w(5)?,
        page_count: w(6)?,
        pinned_page_count: w(7)?,
        dag_triangles: le64(b, 32)?,
        leaf_triangles: le64(b, 40)?,
        aabb_min: vec3(b, 48)?,
        aabb_max: vec3(b, 60)?,
        sphere_center: vec3(b, 72)?,
        sphere_radius: fl(b, 84)?,
        grid,
        max_displacement: fl(b, 92)?,
        attribute_layout_hash: w(24)?,
        material_count: w(25)?,
    };
    let empty = m.group_count == 0;
    let ok = m.page_count == pages
        && m.level_count <= MAX_LEVELS
        && m.node_width >= 2
        && (m.level_count == 0) == empty
        && if empty {
            m.cluster_count == 0 && m.node_count == 0 && m.page_count == 0 && m.bvh_depth == 0
        } else {
            m.cluster_count >= m.group_count && m.page_count >= 1
        }
        && m.pinned_page_count <= m.page_count
        && m.leaf_triangles <= m.dag_triangles
        && finite3(m.aabb_min)
        && finite3(m.aabb_max)
        && (0..3).all(|i| m.aabb_min[i] <= m.aabb_max[i])
        && finite3(m.sphere_center)
        && m.sphere_radius.is_finite()
        && m.sphere_radius >= 0.0
        && m.max_displacement.to_bits() == grid.max_displacement().to_bits()
        && w(26)? == 0
        && w(27)? == 0;
    if ok {
        Ok(m)
    } else {
        Err(Error::Invalid)
    }
}
