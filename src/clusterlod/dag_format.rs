//! Stable, independently decodable DAG pages; not in C++ meshoptimizer.
//!
//! Wire specification: `docs/DAG_FORMAT.md`. No native struct layout is serialized.
use super::{FlatDagCluster, FlatDagGroup};
use crate::{codec, Workspace};
use alloc::vec::Vec;

/// Default maximum geometry page size, 128 KiB.
pub const DEFAULT_PAGE_SIZE: usize = 128 * 1024;
/// Vertex codec version pinned by format v1 (independent of library defaults).
pub const V1_VERTEX_CODEC_VERSION: u8 = 1;
/// Vertex codec compression level pinned by format v1.
pub const V1_VERTEX_CODEC_LEVEL: u8 = 2;
/// Index-sequence codec version pinned by format v1.
pub const V1_INDEX_CODEC_VERSION: u8 = 1;
/// Format failures; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Truncated or structurally inconsistent input.
    Invalid,
    /// Unknown major version or required flag.
    Unsupported,
    /// Integer overflow or a format limit exceeded.
    Limit,
    /// Allocation failed.
    Allocation,
    /// A crate codec rejected a stream or exhausted its workspace.
    Codec(crate::Error),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DAG format: {self:?}")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}
impl From<crate::Error> for Error {
    fn from(e: crate::Error) -> Self {
        Self::Codec(e)
    }
}
fn add(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_add(b).ok_or(Error::Limit)
}
fn mul(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_mul(b).ok_or(Error::Limit)
}
fn uint(v: usize) -> Result<u32, Error> {
    u32::try_from(v).map_err(|_| Error::Limit)
}
fn size(v: u32) -> Result<usize, Error> {
    usize::try_from(v).map_err(|_| Error::Limit)
}
fn push(v: &mut Vec<u8>, b: &[u8]) -> Result<(), Error> {
    v.try_reserve(b.len()).map_err(|_| Error::Allocation)?;
    v.extend_from_slice(b);
    Ok(())
}
fn copy_bytes(b: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    push(&mut out, b)?;
    Ok(out)
}
fn put(v: &mut Vec<u8>, x: u32) -> Result<(), Error> {
    push(v, &x.to_le_bytes())
}
fn word(b: &[u8], at: usize) -> Result<u32, Error> {
    let a = b.get(at..add(at, 4)?).ok_or(Error::Invalid)?;
    Ok(u32::from_le_bytes([a[0], a[1], a[2], a[3]]))
}
fn words(b: &[u8], at: usize, n: usize) -> Result<Vec<u32>, Error> {
    let end = add(at, mul(n, 4)?)?;
    let bytes = b.get(at..end).ok_or(Error::Invalid)?;
    let mut out = Vec::new();
    out.try_reserve_exact(n).map_err(|_| Error::Allocation)?;
    for v in bytes.as_chunks::<4>().0 {
        out.push(u32::from_le_bytes([v[0], v[1], v[2], v[3]]));
    }
    Ok(out)
}
fn bounds(w: &[u32]) -> Result<(), Error> {
    if w[..4].iter().any(|&v| !f32::from_bits(v).is_finite()) || f32::from_bits(w[3]) < 0. {
        return Err(Error::Invalid);
    }
    Ok(())
}
fn floats(v: &mut Vec<u8>, center: [f32; 3], radius: f32) -> Result<(), Error> {
    for f in center.into_iter().chain([radius]) {
        put(v, f.to_bits())?;
    }
    Ok(())
}
/// Decoded whole cluster; vertices carry original flat snapshot identifiers.
/// Not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct GeometryGeneric<I: crate::Index = u32> {
    /// Flat DAG cluster identifier.
    pub cluster: u32,
    /// Pairs of flat vertex identifier and bit-exact XYZ position.
    pub vertices: Vec<(u32, [f32; 3])>,
    /// Triangle indices into this cluster's local vertex array.
    pub indices: Vec<I>,
}
/// Validated resident metadata borrowing the serialized file; not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct Reader<'a> {
    bytes: &'a [u8],
    groups: usize,
    clusters: usize,
    pages: usize,
    page_table: usize,
}
impl<'a> Reader<'a> {
    /// Validate all tables and geometry before returning. Work and temporary memory
    /// are bounded by file size (clusters have at most 768 indices/vertices).
    /// Not in C++ meshoptimizer.
    /// # Errors
    /// Returns typed structural, version, resource or codec failures.
    pub fn read(bytes: &'a [u8], workspace: &mut Workspace) -> Result<Self, Error> {
        if bytes.get(..4) != Some(b"MDAG") {
            return Err(Error::Invalid);
        }
        if word(bytes, 4)? != 1 || word(bytes, 8)? != 0 {
            return Err(Error::Unsupported);
        }
        let groups = size(word(bytes, 12)?)?;
        let clusters = size(word(bytes, 16)?)?;
        let pages = size(word(bytes, 20)?)?;
        let page_size = size(word(bytes, 24)?)?;
        if size(word(bytes, 28)?)? != bytes.len() || page_size == 0 {
            return Err(Error::Invalid);
        }
        let page_table = add(add(32, mul(groups, 32)?)?, mul(clusters, 64)?)?;
        let mut end = add(page_table, mul(pages, 16)?)?;
        if end > bytes.len() {
            return Err(Error::Invalid);
        }
        let r = Self {
            bytes,
            groups,
            clusters,
            pages,
            page_table,
        };
        let mut next = 0;
        for g in 0..groups {
            let w = words(bytes, add(32, mul(g, 32)?)?, 8)?;
            bounds(&w)?;
            let error = f32::from_bits(w[4]);
            if !error.is_finite() || error < 0. || w[5] >= 64 || size(w[6])? != next || w[7] == 0 {
                return Err(Error::Invalid);
            }
            next = add(next, size(w[7])?)?;
            if next > clusters {
                return Err(Error::Invalid);
            }
            for c in size(w[6])?..next {
                let cl = r.cluster(c)?;
                if size(cl[0])? != g {
                    return Err(Error::Invalid);
                }
            }
        }
        if next != clusters {
            return Err(Error::Invalid);
        }
        let mut next_cluster = 0;
        let mut next_index = 0usize;
        for p in 0..pages {
            let w = r.page(p)?;
            if size(w[0])? != end
                || w[1] == 0
                || size(w[1])? > page_size
                || size(w[2])? != next_cluster
                || w[3] == 0
            {
                return Err(Error::Invalid);
            }
            end = add(end, size(w[1])?)?;
            if end > bytes.len() {
                return Err(Error::Invalid);
            }
            next_cluster = add(next_cluster, size(w[3])?)?;
            if next_cluster > clusters {
                return Err(Error::Invalid);
            }
            for c in size(w[2])?..next_cluster {
                let cl = r.cluster(c)?;
                if size(cl[14])? != next_index {
                    return Err(Error::Invalid);
                }
                next_index = add(next_index, size(cl[7])?)?;
                uint(next_index)?;
            }
            r.decode_page(p, &bytes[size(w[0])?..end], workspace)?;
        }
        if end != bytes.len() || next_cluster != clusters {
            return Err(Error::Invalid);
        }
        Ok(r)
    }
    fn cluster(&self, c: usize) -> Result<Vec<u32>, Error> {
        if c >= self.clusters {
            return Err(Error::Invalid);
        }
        words(
            self.bytes,
            add(add(32, mul(self.groups, 32)?)?, mul(c, 64)?)?,
            16,
        )
    }
    fn page(&self, p: usize) -> Result<Vec<u32>, Error> {
        if p >= self.pages {
            return Err(Error::Invalid);
        }
        words(self.bytes, add(self.page_table, mul(p, 16)?)?, 4)
    }
    /// Number of independently decodable pages.
    pub fn page_count(&self) -> usize {
        self.pages
    }
    /// Find the page to fetch for a cluster using resident metadata only.
    /// A group's `first_cluster..first_cluster + cluster_count` range can be
    /// mapped through this method before any geometry page is fetched.
    /// # Errors
    /// Invalid cluster identifier.
    pub fn cluster_page(&self, c: usize) -> Result<usize, Error> {
        size(self.cluster(c)?[2])
    }
    /// Read a resident cluster record with its original flat index offset.
    /// # Errors
    /// Invalid cluster identifier.
    pub fn cluster_record(&self, c: usize) -> Result<FlatDagCluster, Error> {
        let w = self.cluster(c)?;
        Ok(FlatDagCluster {
            first_index: w[14],
            index_count: w[7],
            group: w[0],
            refined: i32::from_le_bytes(w[1].to_le_bytes()),
            center: [
                f32::from_bits(w[10]),
                f32::from_bits(w[11]),
                f32::from_bits(w[12]),
            ],
            radius: f32::from_bits(w[13]),
        })
    }
    /// Resident group count.
    pub fn group_count(&self) -> usize {
        self.groups
    }
    /// Resident cluster count.
    pub fn cluster_count(&self) -> usize {
        self.clusters
    }
    /// Return the exact byte range to fetch for a page.
    /// # Errors
    /// Invalid page identifier.
    pub fn page_range(&self, p: usize) -> Result<core::ops::Range<usize>, Error> {
        let w = self.page(p)?;
        let start = size(w[0])?;
        Ok(start..add(start, size(w[1])?)?)
    }
    /// Read a resident group record.
    /// # Errors
    /// Invalid group identifier.
    pub fn group(&self, g: usize) -> Result<FlatDagGroup, Error> {
        if g >= self.groups {
            return Err(Error::Invalid);
        }
        let w = words(self.bytes, add(32, mul(g, 32)?)?, 8)?;
        Ok(FlatDagGroup {
            center: [
                f32::from_bits(w[0]),
                f32::from_bits(w[1]),
                f32::from_bits(w[2]),
            ],
            radius: f32::from_bits(w[3]),
            error: f32::from_bits(w[4]),
            depth: i32::try_from(w[5]).map_err(|_| Error::Invalid)?,
            first_cluster: w[6],
            cluster_count: w[7],
        })
    }
    /// Decode only the supplied page bytes, consulting resident tables only.
    /// Not in C++ meshoptimizer.
    /// # Errors
    /// Invalid identifier, malformed geometry, allocation or codec failure.
    /// Source-compatible u32 operation.
    #[inline]
    pub fn decode_page(
        &self,
        p: usize,
        bytes: &[u8],
        workspace: &mut Workspace,
    ) -> Result<Vec<Geometry>, Error> {
        self.decode_page_generic::<u32>(p, bytes, workspace)
    }
    /// Select u16 or u32 output through the sealed Index trait.
    pub fn decode_page_generic<I: crate::Index>(
        &self,
        p: usize,
        bytes: &[u8],
        workspace: &mut Workspace,
    ) -> Result<Vec<GeometryGeneric<I>>, Error> {
        let page = self.page(p)?;
        if bytes.len() != size(page[1])? {
            return Err(Error::Invalid);
        }
        let mut out = Vec::new();
        let count = size(page[3])?;
        if count > bytes.len() / 16 {
            return Err(Error::Invalid);
        }
        out.try_reserve_exact(count)
            .map_err(|_| Error::Allocation)?;
        let mut next = 0;
        for c in size(page[2])?..add(size(page[2])?, count)? {
            let w = self.cluster(c)?;
            bounds(&w[10..14])?;
            let group = self.group(size(w[0])?)?;
            let refined = i32::from_le_bytes(w[1].to_le_bytes());
            if refined != -1 {
                let child_id = usize::try_from(refined).map_err(|_| Error::Invalid)?;
                if child_id >= size(w[0])? {
                    return Err(Error::Invalid);
                }
                let child = self.group(child_id)?;
                if child.depth >= group.depth || child.error > group.error {
                    return Err(Error::Invalid);
                }
            } else if group.depth != 0 {
                return Err(Error::Invalid);
            }
            if size(w[2])? != p || size(w[3])? != next || w[9] & !3 != 0 || w[15] != 0 {
                return Err(Error::Invalid);
            }
            let nv = size(w[6])?;
            let ni = size(w[7])?;
            if nv == 0
                || nv > 768
                || ni == 0
                || ni > 768
                || !ni.is_multiple_of(3)
                || w[8] != uint(add(mul(nv, 16)?, mul(ni, 4)?)?)?
            {
                return Err(Error::Invalid);
            }
            let split = add(next, size(w[4])?)?;
            let end = add(split, size(w[5])?)?;
            let vb = bytes.get(next..split).ok_or(Error::Invalid)?;
            let ib = bytes.get(split..end).ok_or(Error::Invalid)?;
            if (w[9] & 1 == 0 && vb.len() != mul(nv, 16)?)
                || (w[9] & 2 == 0 && ib.len() != mul(ni, 4)?)
            {
                return Err(Error::Invalid);
            }
            if (w[9] & 1 != 0 && codec::decode_vertex_version(vb)? != V1_VERTEX_CODEC_VERSION)
                || (w[9] & 2 != 0 && codec::decode_index_version(ib)? != V1_INDEX_CODEC_VERSION)
            {
                return Err(Error::Unsupported);
            }
            let vertices = if w[9] & 1 != 0 {
                codec::decode_vertex_buffer(nv, 16, vb, workspace)?
            } else {
                copy_bytes(vb)?
            };
            let indices = if w[9] & 2 != 0 {
                codec::decode_index_sequence(ni, 4, ib, workspace)?
            } else {
                copy_bytes(ib)?
            };
            if vertices.len() != mul(nv, 16)? || indices.len() != mul(ni, 4)? {
                return Err(Error::Invalid);
            }
            let mut geometry = GeometryGeneric {
                cluster: uint(c)?,
                vertices: Vec::new(),
                indices: I::from_vec(words(&indices, 0, ni)?, workspace)?,
            };
            geometry
                .vertices
                .try_reserve_exact(nv)
                .map_err(|_| Error::Allocation)?;
            for v in vertices.as_chunks::<16>().0 {
                let w = words(v, 0, 4)?;
                let pos = [
                    f32::from_bits(w[1]),
                    f32::from_bits(w[2]),
                    f32::from_bits(w[3]),
                ];
                if pos.iter().any(|v| !v.is_finite()) {
                    return Err(Error::Invalid);
                }
                geometry.vertices.push((w[0], pos));
            }
            if geometry
                .indices
                .iter()
                .any(|&i| size(i.to_u32()).map_or(true, |i| i >= nv))
            {
                return Err(Error::Invalid);
            }
            // Local ids must appear in first-occurrence order and all be used.
            let mut seen = 0usize;
            for &i in &geometry.indices {
                let i = size(i.to_u32())?;
                if i > seen {
                    return Err(Error::Invalid);
                }
                if i == seen {
                    seen = add(seen, 1)?;
                }
            }
            if seen != nv {
                return Err(Error::Invalid);
            }
            // Snapshot ids are unique within a cluster.
            let mut ids = Vec::new();
            ids.try_reserve_exact(nv).map_err(|_| Error::Allocation)?;
            ids.extend(geometry.vertices.iter().map(|v| v.0));
            ids.sort_unstable();
            if ids.windows(2).any(|p| p[0] == p[1]) {
                return Err(Error::Invalid);
            }
            out.push(geometry);
            next = end;
        }
        if next != bytes.len() {
            return Err(Error::Invalid);
        }
        Ok(out)
    }
}

/// Serialize flat snapshot geometry into deterministic little-endian pages.
/// `page_size` is a hard maximum; clusters never straddle pages. Pass
/// [`DEFAULT_PAGE_SIZE`] for the default. Not in C++ meshoptimizer.
/// # Examples
/// ```
/// use meshoptimizer_rs::{clusterlod::dag_format::{write, Reader, DEFAULT_PAGE_SIZE}, Workspace};
/// let mut workspace = Workspace::default();
/// let bytes = write(&[], &[], &[], &[], DEFAULT_PAGE_SIZE, &mut workspace)?;
/// assert_eq!(Reader::read(&bytes, &mut workspace)?.page_count(), 0);
/// # Ok::<(), meshoptimizer_rs::clusterlod::dag_format::Error>(())
/// ```
/// # Errors
/// Invalid DAG/topology, oversized cluster/page/file, allocation or codec failure.
pub fn write_generic<I: crate::Index>(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    positions: &[[f32; 3]],
    indices: &[I],
    page_size: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    uint(page_size)?;
    uint(groups.len())?;
    uint(clusters.len())?;
    if page_size == 0 {
        return Err(Error::Invalid);
    }
    let mut tables = Vec::new();
    let mut payload = Vec::new();
    let mut pages = Vec::<[u32; 4]>::new();
    let mut used = 0usize;
    let mut first = 0usize;
    let mut expected_index = 0usize;
    for (c, cluster) in clusters.iter().enumerate() {
        let start = size(cluster.first_index)?;
        let ni = size(cluster.index_count)?;
        if start != expected_index {
            return Err(Error::Invalid);
        }
        expected_index = add(start, ni)?;
        if ni == 0 || ni > 768 || !ni.is_multiple_of(3) {
            return Err(Error::Invalid);
        }
        let src = indices.get(start..add(start, ni)?).ok_or(Error::Invalid)?;
        let mut map = Vec::<u32>::new();
        map.try_reserve_exact(ni).map_err(|_| Error::Allocation)?;
        let mut vb = Vec::new();
        let mut local = Vec::new();
        local.try_reserve_exact(ni).map_err(|_| Error::Allocation)?;
        for &id in src {
            let id = id.to_u32();
            let next = uint(map.len())?;
            let index = match map.iter().position(|&v| v == id) {
                Some(i) => uint(i)?,
                None => {
                    map.push(id);
                    next
                }
            };
            if index == next {
                let pos = positions.get(size(id)?).ok_or(Error::Invalid)?;
                put(&mut vb, id)?;
                for f in pos {
                    put(&mut vb, f.to_bits())?;
                }
            }
            local.push(index);
        }
        let nv = map.len();
        let mut ib = Vec::new();
        for &i in &local {
            put(&mut ib, i)?;
        }
        let raw = add(vb.len(), ib.len())?;
        let mut flags = 0;
        let encoded = codec::encode_vertex_buffer(
            &vb,
            nv,
            16,
            codec::VertexEncoding::new(V1_VERTEX_CODEC_VERSION, V1_VERTEX_CODEC_LEVEL)?,
            workspace,
        )?;
        if encoded.len() < vb.len() {
            vb = encoded;
            flags |= 1;
        }
        let encoded = codec::encode_index_sequence_generic(
            &local,
            codec::IndexEncoding::new(V1_INDEX_CODEC_VERSION)?,
            workspace,
        )?;
        if encoded.len() < ib.len() {
            ib = encoded;
            flags |= 2;
        }
        let length = add(vb.len(), ib.len())?;
        if length > page_size {
            return Err(Error::Limit);
        }
        if add(used, length)? > page_size {
            pages.try_reserve(1).map_err(|_| Error::Allocation)?;
            pages.push([
                uint(payload.len() - used)?,
                uint(used)?,
                uint(first)?,
                uint(c - first)?,
            ]);
            first = c;
            used = 0;
        }
        for w in [
            cluster.group,
            u32::from_le_bytes(cluster.refined.to_le_bytes()),
            uint(pages.len())?,
            uint(used)?,
            uint(vb.len())?,
            uint(ib.len())?,
            uint(nv)?,
            uint(ni)?,
            uint(raw)?,
            flags,
        ] {
            put(&mut tables, w)?;
        }
        floats(&mut tables, cluster.center, cluster.radius)?;
        put(&mut tables, cluster.first_index)?;
        put(&mut tables, 0)?;
        push(&mut payload, &vb)?;
        push(&mut payload, &ib)?;
        used = add(used, length)?;
    }
    if expected_index != indices.len() {
        return Err(Error::Invalid);
    }
    if used != 0 {
        pages.try_reserve(1).map_err(|_| Error::Allocation)?;
        pages.push([
            uint(payload.len() - used)?,
            uint(used)?,
            uint(first)?,
            uint(clusters.len() - first)?,
        ]);
    }
    let base = add(
        add(add(32, mul(groups.len(), 32)?)?, tables.len())?,
        mul(pages.len(), 16)?,
    )?;
    let mut out = Vec::new();
    push(&mut out, b"MDAG")?;
    for w in [
        1,
        0,
        uint(groups.len())?,
        uint(clusters.len())?,
        uint(pages.len())?,
        uint(page_size)?,
        uint(add(base, payload.len())?)?,
    ] {
        put(&mut out, w)?;
    }
    for g in groups {
        floats(&mut out, g.center, g.radius)?;
        for w in [
            g.error.to_bits(),
            u32::try_from(g.depth).map_err(|_| Error::Invalid)?,
            g.first_cluster,
            g.cluster_count,
        ] {
            put(&mut out, w)?;
        }
    }
    push(&mut out, &tables)?;
    for p in pages {
        for w in [uint(add(base, size(p[0])?)?)?, p[1], p[2], p[3]] {
            put(&mut out, w)?;
        }
    }
    push(&mut out, &payload)?;
    Reader::read(&out, workspace)?;
    Ok(out)
}

/// The source-compatible u32 entry point; see [`write_generic`] for selectable index widths.
#[inline]
pub fn write(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    positions: &[[f32; 3]],
    indices: &[u32],
    page_size: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    write_generic::<u32>(groups, clusters, positions, indices, page_size, workspace)
}

/// The source-compatible u32 form of [`GeometryGeneric`].
pub type Geometry = GeometryGeneric<u32>;
