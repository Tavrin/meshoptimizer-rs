//! Quantised GPU pages for cluster DAGs; not in C++ meshoptimizer.
//!
//! A page has a **memory form** that a GPU binds as plain 32-bit words and a
//! **disk form** that [`transcode_page`] turns back into the memory form.
//! Positions are integers on a power-of-two [`QuantGrid`] shared by the whole
//! mesh (or a whole modular kit), so equal source positions decode to equal bits
//! in every cluster and level. Decoding needs only 32-bit loads, shifts and masks:
//! no 8- or 16-bit storage, no 64-bit integers and no subgroup operations.
//!
//! Layout, the reference WGSL decoder and its test vectors: `docs/PAGE_FORMAT.md`.
use super::dag_format::Error;
use crate::{codec, Positions, Workspace};
use alloc::vec::Vec;

/// Words in the page header: cluster count, memory words, attribute layout hash, reserved.
pub const PAGE_HEADER_WORDS: usize = 4;
/// Words in one cluster descriptor.
pub const CLUSTER_WORDS: usize = 16;
/// Most vertices in one cluster (local indices are bytes).
pub const MAX_CLUSTER_VERTICES: usize = 256;
/// Most triangles in one cluster.
pub const MAX_CLUSTER_TRIANGLES: usize = 256;
/// Most bits per position channel.
pub const MAX_CHANNEL_BITS: u32 = 24;
/// Largest absolute grid coordinate. Every coordinate converts to `f32` exactly
/// and every per-cluster channel range fits [`MAX_CHANNEL_BITS`].
pub const MAX_GRID_COORDINATE: i32 = (1 << 23) - 1;
/// Largest memory form, in words (1 MiB).
pub const MAX_PAGE_WORDS: usize = 1 << 18;
/// Descriptor word 2 flag: the cluster uses one material slot (word 15).
pub const FLAG_SINGLE_MATERIAL: u32 = 1 << 18;
/// Descriptor word 2 flag: the normal cone in word 14 can be used for culling.
pub const FLAG_CONE_VALID: u32 = 1 << 19;
/// Vertex codec version pinned by the compressed disk form.
pub const PAGE_VERTEX_CODEC_VERSION: u8 = 1;
/// Vertex codec level pinned by the compressed disk form.
pub const PAGE_VERTEX_CODEC_LEVEL: u8 = 2;

const DISK_NONE: u32 = 0;
const DISK_COMPRESSED: u32 = 1;
/// `sqrt(3) / 2`, rounded to nearest.
const HALF_DIAGONAL: f64 = 0.866_025_403_784_438_6;

/// Power-of-two position grid: coordinates are `integer * 2^exponent`, with the
/// origin at the mesh-space origin. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QuantGrid {
    exponent: i32,
}

impl QuantGrid {
    /// Smallest supported exponent (steps stay normal `f32` values).
    pub const MIN_EXPONENT: i32 = -126;
    /// Largest supported exponent.
    pub const MAX_EXPONENT: i32 = 127;

    /// Grid with step `2^exponent`; use one grid for every mesh whose borders must meet.
    ///
    /// # Errors
    /// `Invalid` if `exponent` is outside [`Self::MIN_EXPONENT`]..=[`Self::MAX_EXPONENT`].
    pub const fn new(exponent: i32) -> Result<Self, Error> {
        if exponent < Self::MIN_EXPONENT || exponent > Self::MAX_EXPONENT {
            return Err(Error::Invalid);
        }
        Ok(Self { exponent })
    }

    /// Finest grid on which the box `min..max` fits `bits` signed bits per channel,
    /// from [`crate::compute_position_exponent`].
    ///
    /// # Errors
    /// `Invalid` for non-finite or unordered bounds, or `bits` outside 2..=24.
    pub fn from_bounds(min: [f32; 3], max: [f32; 3], bits: u32) -> Result<Self, Error> {
        let exponent = crate::compute_position_exponent(min, max, Self::MIN_EXPONENT, bits)
            .map_err(|e| match e {
                crate::Error::InvalidParameter => Error::Invalid,
                _ => Error::Limit,
            })?;
        Self::new(exponent)
    }

    /// Coarsest grid whose [`Self::max_displacement`] is at most `max_error`.
    ///
    /// # Errors
    /// `Invalid` if `max_error` is not finite and positive, or is below the finest
    /// grid's displacement.
    pub fn from_max_error(max_error: f32) -> Result<Self, Error> {
        if !max_error.is_finite() || max_error <= 0. {
            return Err(Error::Invalid);
        }
        let mut exponent = Self::MAX_EXPONENT;
        loop {
            let grid = Self { exponent };
            if grid.max_displacement() <= max_error {
                return Ok(grid);
            }
            if exponent == Self::MIN_EXPONENT {
                return Err(Error::Invalid);
            }
            exponent -= 1;
        }
    }

    /// Grid exponent.
    pub const fn exponent(self) -> i32 {
        self.exponent
    }

    /// Grid step, `2^exponent`.
    pub const fn step(self) -> f32 {
        // MIN_EXPONENT..=MAX_EXPONENT keeps the biased exponent in 1..=254.
        f32::from_bits(((self.exponent + 127) as u32) << 23)
    }

    /// Upper bound on the distance between a position and its snapped position,
    /// `2^exponent * sqrt(3) / 2` rounded up to `f32`. Add it to every finite
    /// group error; thresholds below it select source clusters only.
    pub fn max_displacement(self) -> f32 {
        let exact = f64::from(self.step()) * HALF_DIAGONAL;
        let value = exact as f32;
        // The constant may round down by half an f64 ulp; one f32 step up covers it.
        if f64::from(value) <= exact {
            value.next_up()
        } else {
            value
        }
    }

    /// Nearest grid coordinate of `value`, ties away from zero.
    ///
    /// # Errors
    /// `Invalid` for a non-finite value; `Limit` if the coordinate exceeds
    /// [`MAX_GRID_COORDINATE`] or its position overflows `f32`.
    pub fn snap_coordinate(self, value: f32) -> Result<i32, Error> {
        if !value.is_finite() {
            return Err(Error::Invalid);
        }
        // Scaling by a power of two is exact in f64 for every f32 and exponent.
        let scaled = f64::from(value) * f64::from(self.step()).recip();
        let rounded = libm::round(scaled);
        if rounded.abs() > f64::from(MAX_GRID_COORDINATE) {
            return Err(Error::Limit);
        }
        let q = rounded as i32;
        if !self.coordinate(q).is_finite() {
            return Err(Error::Limit);
        }
        Ok(q)
    }

    /// Grid coordinates of a position; see [`Self::snap_coordinate`].
    ///
    /// # Errors
    /// As [`Self::snap_coordinate`].
    pub fn snap(self, position: [f32; 3]) -> Result<[i32; 3], Error> {
        Ok([
            self.snap_coordinate(position[0])?,
            self.snap_coordinate(position[1])?,
            self.snap_coordinate(position[2])?,
        ])
    }

    /// Position of a grid coordinate. Exact for coordinates within
    /// [`MAX_GRID_COORDINATE`]; this is the value every decoder returns.
    pub fn coordinate(self, q: i32) -> f32 {
        q as f32 * self.step()
    }

    /// Position of grid coordinates; see [`Self::coordinate`].
    pub fn position(self, q: [i32; 3]) -> [f32; 3] {
        [
            self.coordinate(q[0]),
            self.coordinate(q[1]),
            self.coordinate(q[2]),
        ]
    }

    /// Snap positions in place before building a DAG, so that every cluster and
    /// level selects vertices already on the grid and shared positions decode
    /// identically. Nothing is written unless every position snaps.
    /// Returns the largest measured displacement, at most [`Self::max_displacement`].
    ///
    /// # Errors
    /// As [`Self::snap_coordinate`].
    pub fn snap_positions(self, positions: &mut [[f32; 3]]) -> Result<f32, Error> {
        let mut worst = 0f64;
        for p in positions.iter() {
            let s = self.position(self.snap(*p)?);
            worst = worst.max(distance(*p, s));
        }
        for p in positions.iter_mut() {
            *p = self.position(self.snap(*p)?);
        }
        Ok(round_up(worst))
    }
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f64 {
    let s: f64 = a
        .iter()
        .zip(&b)
        .map(|(&x, &y)| {
            let d = f64::from(x) - f64::from(y);
            d * d
        })
        .sum();
    libm::sqrt(s)
}

fn round_up(v: f64) -> f32 {
    let f = v as f32;
    if f64::from(f) < v {
        f.next_up()
    } else {
        f
    }
}

fn uint(v: usize) -> Result<u32, Error> {
    u32::try_from(v).map_err(|_| Error::Limit)
}

fn size(v: u32) -> Result<usize, Error> {
    usize::try_from(v).map_err(|_| Error::Limit)
}

fn reserve<T>(v: &mut Vec<T>, n: usize) -> Result<(), Error> {
    v.try_reserve(n).map_err(|_| Error::Allocation)
}

fn bit_words(vertices: usize, bits: u32) -> usize {
    (vertices * bits as usize).div_ceil(32)
}

fn triangle_words(triangles: usize) -> usize {
    (triangles * 3).div_ceil(4)
}

/// Reads `count` (at most 24) bits LSB-first at bit address `bit`; mirrors the WGSL decoder.
fn read_bits(words: &[u32], bit: usize, count: u32) -> Result<u32, Error> {
    if count == 0 {
        return Ok(0);
    }
    let at = bit >> 5;
    let shift = (bit & 31) as u32;
    let mut value = words.get(at).ok_or(Error::Invalid)? >> shift;
    if shift + count > 32 {
        value |= words.get(at + 1).ok_or(Error::Invalid)? << (32 - shift);
    }
    Ok(value & (u32::MAX >> (32 - count)))
}

fn write_bits(words: &mut [u32], bit: usize, count: u32, value: u32) {
    if count == 0 {
        return;
    }
    let at = bit >> 5;
    let shift = (bit & 31) as u32;
    words[at] |= value << shift;
    if shift + count > 32 {
        words[at + 1] |= value >> (32 - shift);
    }
}

/// One cluster to place in a page: cluster-local positions and triangle indices
/// into them. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct PageCluster<'a> {
    /// Owning group.
    pub group: u32,
    /// Refined group, or -1 for source geometry.
    pub refined: i32,
    /// Cluster-local positions, 1 to 256; snapped to the page grid on encode.
    pub positions: &'a [[f32; 3]],
    /// Triangle indices into `positions`, 1 to 256 triangles; every vertex must be used.
    pub indices: &'a [u32],
}

impl<'a> PageCluster<'a> {
    /// Cluster record for [`encode_page`].
    pub const fn new(
        group: u32,
        refined: i32,
        positions: &'a [[f32; 3]],
        indices: &'a [u32],
    ) -> Self {
        Self {
            group,
            refined,
            positions,
            indices,
        }
    }
}

/// Decoded cluster descriptor. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct ClusterDesc {
    /// Owning group.
    pub group: u32,
    /// Refined group, or -1 for source geometry.
    pub refined: i32,
    /// Vertex count, 1 to 256.
    pub vertex_count: u32,
    /// Triangle count, 1 to 256.
    pub triangle_count: u32,
    /// Bits per position channel.
    pub bits: [u32; 3],
    /// Grid coordinates added to every decoded channel value.
    pub base: [i32; 3],
    /// Tight sphere center; the sphere contains every decoded vertex.
    pub center: [f32; 3],
    /// Tight sphere radius.
    pub radius: f32,
    /// Normal cone axis, signed-normalized (from [`crate::compute_cluster_bounds`]).
    pub cone_axis_s8: [i8; 3],
    /// Normal cone cutoff, signed-normalized.
    pub cone_cutoff_s8: i8,
    /// Whether the cone can be used for culling.
    pub cone_valid: bool,
    /// Material slot of a single-material cluster.
    pub material: u32,
}

struct Layout {
    desc: ClusterDesc,
    position_offset: usize,
    triangle_offset: usize,
}

fn parse_descriptor(d: &[u32]) -> Result<Layout, Error> {
    let flags = d[2];
    if flags >> 20 != 0 || d[4] >> 15 != 0 || d[9] != 0 {
        return Err(Error::Invalid);
    }
    if flags & FLAG_SINGLE_MATERIAL == 0 {
        // Material range tables are not defined yet.
        return Err(Error::Unsupported);
    }
    let vertex_count = flags & 0x1ff;
    let triangle_count = (flags >> 9) & 0x1ff;
    let bits = [d[4] & 31, (d[4] >> 5) & 31, (d[4] >> 10) & 31];
    let base = [d[5] as i32, d[6] as i32, d[7] as i32];
    let center = [
        f32::from_bits(d[10]),
        f32::from_bits(d[11]),
        f32::from_bits(d[12]),
    ];
    let radius = f32::from_bits(d[13]);
    if !(1..=MAX_CLUSTER_VERTICES as u32).contains(&vertex_count)
        || !(1..=MAX_CLUSTER_TRIANGLES as u32).contains(&triangle_count)
        || bits.iter().any(|&b| b > MAX_CHANNEL_BITS)
        || base
            .iter()
            .any(|b| b.unsigned_abs() > MAX_GRID_COORDINATE as u32)
        || (d[1] as i32) < -1
        || !center.iter().all(|c| c.is_finite())
        || !radius.is_finite()
        || radius < 0.
    {
        return Err(Error::Invalid);
    }
    let cone = d[14].to_le_bytes();
    Ok(Layout {
        desc: ClusterDesc {
            group: d[0],
            refined: d[1] as i32,
            vertex_count,
            triangle_count,
            bits,
            base,
            center,
            radius,
            cone_axis_s8: [cone[0] as i8, cone[1] as i8, cone[2] as i8],
            cone_cutoff_s8: cone[3] as i8,
            cone_valid: flags & FLAG_CONE_VALID != 0,
            material: d[15],
        },
        position_offset: size(d[3])?,
        triangle_offset: size(d[8])?,
    })
}

/// Validated memory-form page. Not in C++ meshoptimizer.
///
/// Validation checks the header, every descriptor, the canonical stream layout
/// (streams back to back in cluster order, zero padding), local indices below
/// the vertex count, that every vertex is referenced, and that every decoded
/// vertex lies in its cluster's tight sphere.
#[derive(Clone, Copy, Debug)]
pub struct PageView<'a> {
    words: &'a [u32],
    grid: QuantGrid,
    clusters: usize,
}

impl<'a> PageView<'a> {
    /// Validate a memory-form page decoded on `grid`.
    ///
    /// # Errors
    /// `Invalid` for any inconsistency; `Unsupported` for multi-material clusters;
    /// `Limit` for a page above [`MAX_PAGE_WORDS`].
    pub fn new(words: &'a [u32], grid: QuantGrid) -> Result<Self, Error> {
        if words.len() > MAX_PAGE_WORDS {
            return Err(Error::Limit);
        }
        if words.len() < PAGE_HEADER_WORDS
            || size(words[1])? != words.len()
            || words[3] != 0
            || words[0] == 0
        {
            return Err(Error::Invalid);
        }
        let clusters = size(words[0])?;
        let mut cursor = clusters
            .checked_mul(CLUSTER_WORDS)
            .and_then(|v| v.checked_add(PAGE_HEADER_WORDS))
            .ok_or(Error::Invalid)?;
        if cursor > words.len() {
            return Err(Error::Invalid);
        }
        let view = Self {
            words,
            grid,
            clusters,
        };
        for c in 0..clusters {
            let l = view.layout(c)?;
            let nv = l.desc.vertex_count as usize;
            let nt = l.desc.triangle_count as usize;
            let stride: u32 = l.desc.bits.iter().sum();
            let pw = bit_words(nv, stride);
            if l.position_offset != cursor || l.triangle_offset != cursor + pw {
                return Err(Error::Invalid);
            }
            cursor = l.triangle_offset + triangle_words(nt);
            if cursor > words.len() {
                return Err(Error::Invalid);
            }
            let used_bits = nv * stride as usize;
            if pw > 0
                && !used_bits.is_multiple_of(32)
                && words[l.position_offset + pw - 1] >> (used_bits % 32) != 0
            {
                return Err(Error::Invalid);
            }
            let tri = &words[l.triangle_offset..cursor];
            let mut seen = [false; MAX_CLUSTER_VERTICES];
            for k in 0..nt * 3 {
                let i = (tri[k >> 2] >> ((k & 3) * 8)) as u8 as usize;
                if i >= nv {
                    return Err(Error::Invalid);
                }
                seen[i] = true;
            }
            if seen[..nv].iter().any(|s| !s) {
                return Err(Error::Invalid);
            }
            let tail = (nt * 3) % 4;
            if tail != 0 && tri[tri.len() - 1] >> (tail * 8) != 0 {
                return Err(Error::Invalid);
            }
            for v in 0..nv {
                let q = view.vertex(&l, v)?;
                if q.iter()
                    .any(|x| x.unsigned_abs() > MAX_GRID_COORDINATE as u32)
                {
                    return Err(Error::Invalid);
                }
                let p = grid.position(q);
                if distance(p, l.desc.center) > f64::from(l.desc.radius) {
                    return Err(Error::Invalid);
                }
            }
        }
        if cursor != words.len() {
            return Err(Error::Invalid);
        }
        Ok(view)
    }

    fn layout(&self, c: usize) -> Result<Layout, Error> {
        if c >= self.clusters {
            return Err(Error::Invalid);
        }
        let at = PAGE_HEADER_WORDS + c * CLUSTER_WORDS;
        parse_descriptor(&self.words[at..at + CLUSTER_WORDS])
    }

    fn vertex(&self, l: &Layout, v: usize) -> Result<[i32; 3], Error> {
        let b = l.desc.bits;
        let mut bit = l.position_offset * 32 + v * (b[0] + b[1] + b[2]) as usize;
        let mut q = l.desc.base;
        for (q, &bits) in q.iter_mut().zip(&b) {
            let delta = read_bits(self.words, bit, bits)?;
            bit += bits as usize;
            // Deltas are below 2^24 and bases within MAX_GRID_COORDINATE: no overflow.
            *q += delta as i32;
        }
        Ok(q)
    }

    /// The validated memory words.
    pub fn words(&self) -> &'a [u32] {
        self.words
    }

    /// Grid the page decodes on.
    pub fn grid(&self) -> QuantGrid {
        self.grid
    }

    /// Number of clusters.
    pub fn cluster_count(&self) -> usize {
        self.clusters
    }

    /// Attribute layout hash from the page header.
    pub fn attribute_layout_hash(&self) -> u32 {
        self.words[2]
    }

    /// Descriptor of cluster `c`.
    ///
    /// # Errors
    /// `Invalid` if `c` is out of range.
    pub fn cluster(&self, c: usize) -> Result<ClusterDesc, Error> {
        Ok(self.layout(c)?.desc)
    }

    /// Decode cluster `c`'s grid coordinates into `out`; returns the vertex count.
    ///
    /// # Errors
    /// `Invalid` if `c` is out of range; `Limit` if `out` is shorter than the vertex count.
    pub fn decode_coordinates(&self, c: usize, out: &mut [[i32; 3]]) -> Result<usize, Error> {
        let l = self.layout(c)?;
        let nv = l.desc.vertex_count as usize;
        let out = out.get_mut(..nv).ok_or(Error::Limit)?;
        for (v, o) in out.iter_mut().enumerate() {
            *o = self.vertex(&l, v)?;
        }
        Ok(nv)
    }

    /// Decode cluster `c`'s positions into `out`; returns the vertex count.
    ///
    /// # Errors
    /// `Invalid` if `c` is out of range; `Limit` if `out` is shorter than the vertex count.
    pub fn decode_positions(&self, c: usize, out: &mut [[f32; 3]]) -> Result<usize, Error> {
        let l = self.layout(c)?;
        let nv = l.desc.vertex_count as usize;
        let out = out.get_mut(..nv).ok_or(Error::Limit)?;
        for (v, o) in out.iter_mut().enumerate() {
            *o = self.grid.position(self.vertex(&l, v)?);
        }
        Ok(nv)
    }

    /// Decode cluster `c`'s triangles (local indices) into `out`; returns the triangle count.
    ///
    /// # Errors
    /// `Invalid` if `c` is out of range; `Limit` if `out` is shorter than the triangle count.
    pub fn decode_triangles(&self, c: usize, out: &mut [[u8; 3]]) -> Result<usize, Error> {
        let l = self.layout(c)?;
        let nt = l.desc.triangle_count as usize;
        let out = out.get_mut(..nt).ok_or(Error::Limit)?;
        for (t, o) in out.iter_mut().enumerate() {
            for (k, x) in o.iter_mut().enumerate() {
                let byte = t * 3 + k;
                *x = (self.words[l.triangle_offset + (byte >> 2)] >> ((byte & 3) * 8)) as u8;
            }
        }
        Ok(nt)
    }
}

const MESHLET_BOUND: usize = 2048;

/// Encode `triangles` through the meshlet codec with sequential vertex references.
fn meshlet_encode(
    vertex_count: usize,
    triangles: &[u8],
    out: &mut [u8; MESHLET_BOUND],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut refs = [0u32; MAX_CLUSTER_VERTICES];
    for (i, r) in refs.iter_mut().enumerate() {
        *r = i as u32;
    }
    Ok(codec::encode_meshlet_into(
        out,
        &refs[..vertex_count],
        triangles,
        workspace,
    )?)
}

/// Decode a meshlet stream; the vertex references must be sequential.
fn meshlet_decode(
    vertex_count: usize,
    triangles: &mut [u8],
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut refs = [0u8; MAX_CLUSTER_VERTICES * 4];
    let refs = &mut refs[..vertex_count * 4];
    codec::decode_meshlet_into(
        refs,
        vertex_count,
        4,
        triangles,
        triangles.len() / 3,
        3,
        source,
        workspace,
    )?;
    for (i, r) in refs.as_chunks::<4>().0.iter().enumerate() {
        if u32::from_le_bytes(*r) as usize != i {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}

/// Encode clusters into one memory-form page. Positions are snapped to `grid`;
/// triangles are stored in the order the meshlet codec reproduces (rotations of
/// the input, same winding), so [`transcode_page`] restores these exact words.
/// The tight sphere and normal cone are computed from the decoded positions.
/// Not in C++ meshoptimizer.
///
/// # Errors
/// `Invalid` for empty input, counts outside 1..=256, out-of-range or unused
/// vertices, non-finite positions, or `refined < -1`; `Limit` for coordinates
/// beyond [`MAX_GRID_COORDINATE`] or a page above [`MAX_PAGE_WORDS`];
/// `Codec` or `Allocation` for codec or allocation failures.
pub fn encode_page(
    grid: QuantGrid,
    clusters: &[PageCluster<'_>],
    attribute_layout_hash: u32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    if clusters.is_empty() {
        return Err(Error::Invalid);
    }
    let head = clusters
        .len()
        .checked_mul(CLUSTER_WORDS)
        .and_then(|v| v.checked_add(PAGE_HEADER_WORDS))
        .filter(|&v| v <= MAX_PAGE_WORDS)
        .ok_or(Error::Limit)?;
    let mut words = Vec::new();
    reserve(&mut words, head)?;
    words.resize(head, 0);
    words[0] = uint(clusters.len())?;
    words[2] = attribute_layout_hash;
    for (c, cluster) in clusters.iter().enumerate() {
        let nv = cluster.positions.len();
        let ni = cluster.indices.len();
        let nt = ni / 3;
        if !(1..=MAX_CLUSTER_VERTICES).contains(&nv)
            || ni % 3 != 0
            || !(1..=MAX_CLUSTER_TRIANGLES).contains(&nt)
            || cluster.refined < -1
        {
            return Err(Error::Invalid);
        }
        let mut triangles = [0u8; MAX_CLUSTER_TRIANGLES * 3];
        let mut seen = [false; MAX_CLUSTER_VERTICES];
        for (t, &i) in triangles.iter_mut().zip(cluster.indices) {
            let i = size(i)?;
            if i >= nv {
                return Err(Error::Invalid);
            }
            seen[i] = true;
            *t = i as u8;
        }
        if seen[..nv].iter().any(|s| !s) {
            return Err(Error::Invalid);
        }
        let triangles = &mut triangles[..ni];
        let mut stream = [0u8; MESHLET_BOUND];
        let n = meshlet_encode(nv, triangles, &mut stream, workspace)?;
        meshlet_decode(nv, triangles, &stream[..n], workspace)?;

        let mut q = [[0i32; 3]; MAX_CLUSTER_VERTICES];
        let mut decoded = [[0f32; 3]; MAX_CLUSTER_VERTICES];
        for ((q, d), &p) in q.iter_mut().zip(&mut decoded).zip(cluster.positions) {
            *q = grid.snap(p)?;
            *d = grid.position(*q);
        }
        let (q, decoded) = (&q[..nv], &decoded[..nv]);
        let mut base = q[0];
        let mut high = q[0];
        for p in q {
            for ((lo, hi), &x) in base.iter_mut().zip(&mut high).zip(p) {
                *lo = (*lo).min(x);
                *hi = (*hi).max(x);
            }
        }
        let bits = [0, 1, 2].map(|k| 32 - ((high[k] - base[k]) as u32).leading_zeros());
        let mut local = [0u32; MAX_CLUSTER_TRIANGLES * 3];
        for (l, &t) in local.iter_mut().zip(triangles.iter()) {
            *l = u32::from(t);
        }
        let bounds = crate::compute_cluster_bounds(
            &local[..ni],
            Positions::from_packed(decoded),
            workspace,
        )?;
        let center = bounds.center;
        if !center.iter().all(|c| c.is_finite()) {
            return Err(Error::Invalid);
        }
        let radius = round_up(
            decoded
                .iter()
                .map(|&p| distance(p, center))
                .fold(0., f64::max),
        );
        if !radius.is_finite() {
            return Err(Error::Limit);
        }

        let stride: u32 = bits.iter().sum();
        let position_offset = words.len();
        let pw = bit_words(nv, stride);
        let triangle_offset = position_offset + pw;
        let end = triangle_offset + triangle_words(nt);
        if end > MAX_PAGE_WORDS {
            return Err(Error::Limit);
        }
        reserve(&mut words, end - position_offset)?;
        words.resize(end, 0);
        let mut bit = position_offset * 32;
        for p in q {
            for ((&x, &lo), &n) in p.iter().zip(&base).zip(&bits) {
                write_bits(&mut words, bit, n, (x - lo) as u32);
                bit += n as usize;
            }
        }
        for (b, &t) in triangles.iter().enumerate() {
            words[triangle_offset + (b >> 2)] |= u32::from(t) << ((b & 3) * 8);
        }
        let cone_valid = bounds.cone_cutoff_s8 != 127;
        let a = bounds.cone_axis_s8;
        let d = &mut words[PAGE_HEADER_WORDS + c * CLUSTER_WORDS..][..CLUSTER_WORDS];
        d[0] = cluster.group;
        d[1] = cluster.refined as u32;
        d[2] = nv as u32
            | (nt as u32) << 9
            | FLAG_SINGLE_MATERIAL
            | if cone_valid { FLAG_CONE_VALID } else { 0 };
        d[3] = uint(position_offset)?;
        d[4] = bits[0] | bits[1] << 5 | bits[2] << 10;
        d[5] = base[0] as u32;
        d[6] = base[1] as u32;
        d[7] = base[2] as u32;
        d[8] = uint(triangle_offset)?;
        d[9] = 0;
        d[10] = center[0].to_bits();
        d[11] = center[1].to_bits();
        d[12] = center[2].to_bits();
        d[13] = radius.to_bits();
        d[14] = u32::from_le_bytes([
            a[0] as u8,
            a[1] as u8,
            a[2] as u8,
            bounds.cone_cutoff_s8 as u8,
        ]);
        d[15] = 0;
    }
    words[1] = uint(words.len())?;
    PageView::new(&words, grid)?;
    Ok(words)
}

/// Disk-form encoding of a page. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiskCodec {
    /// The memory form stored as little-endian words, for fast loads and GPU transcoders.
    None,
    /// Raw descriptors, then per cluster grid-relative positions through the vertex
    /// codec and triangles through the meshlet codec.
    Compressed,
}

fn put(out: &mut Vec<u8>, w: u32) -> Result<(), Error> {
    reserve(out, 4)?;
    out.extend_from_slice(&w.to_le_bytes());
    Ok(())
}

fn word(b: &[u8], at: usize) -> Result<u32, Error> {
    let a = b
        .get(at..at.checked_add(4).ok_or(Error::Limit)?)
        .ok_or(Error::Invalid)?;
    Ok(u32::from_le_bytes([a[0], a[1], a[2], a[3]]))
}

/// Encode a memory-form page (from [`encode_page`]) into its disk form.
/// Not in C++ meshoptimizer.
///
/// # Errors
/// Any [`PageView::new`] error for `memory`; `Codec` or `Allocation` for codec
/// or allocation failures.
pub fn encode_disk_page(
    memory: &[u32],
    grid: QuantGrid,
    disk_codec: DiskCodec,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let view = PageView::new(memory, grid)?;
    let mut out = Vec::new();
    match disk_codec {
        DiskCodec::None => {
            put(&mut out, DISK_NONE)?;
            for &w in memory {
                put(&mut out, w)?;
            }
        }
        DiskCodec::Compressed => {
            put(&mut out, DISK_COMPRESSED)?;
            put(&mut out, memory[1])?;
            let n = view.cluster_count();
            for &w in &memory[..PAGE_HEADER_WORDS + n * CLUSTER_WORDS] {
                put(&mut out, w)?;
            }
            let table = out.len();
            for _ in 0..n {
                put(&mut out, 0)?;
            }
            let encoding =
                codec::VertexEncoding::new(PAGE_VERTEX_CODEC_VERSION, PAGE_VERTEX_CODEC_LEVEL)?;
            let mut coords = [[0i32; 3]; MAX_CLUSTER_VERTICES];
            let mut deltas = [0u8; MAX_CLUSTER_VERTICES * 12];
            let mut triangles = [[0u8; 3]; MAX_CLUSTER_TRIANGLES];
            for c in 0..n {
                let desc = view.cluster(c)?;
                let nv = view.decode_coordinates(c, &mut coords)?;
                let channels = coords[..nv].iter().flatten().zip(desc.base.iter().cycle());
                for (out, (&x, &lo)) in deltas.as_chunks_mut::<4>().0.iter_mut().zip(channels) {
                    *out = ((x - lo) as u32).to_le_bytes();
                }
                let positions =
                    codec::encode_vertex_buffer(&deltas[..nv * 12], nv, 12, encoding, workspace)?;
                let nt = view.decode_triangles(c, &mut triangles)?;
                let mut flat = [0u8; MAX_CLUSTER_TRIANGLES * 3];
                for (f, t) in flat.as_chunks_mut::<3>().0.iter_mut().zip(&triangles[..nt]) {
                    *f = *t;
                }
                let mut stream = [0u8; MESHLET_BOUND];
                let tn = meshlet_encode(nv, &flat[..nt * 3], &mut stream, workspace)?;
                let (pl, tl) = (
                    u16::try_from(positions.len()).map_err(|_| Error::Limit)?,
                    u16::try_from(tn).map_err(|_| Error::Limit)?,
                );
                let entry = u32::from(pl) | u32::from(tl) << 16;
                out[table + c * 4..][..4].copy_from_slice(&entry.to_le_bytes());
                reserve(&mut out, positions.len() + tn)?;
                out.extend_from_slice(&positions);
                out.extend_from_slice(&stream[..tn]);
            }
        }
    }
    Ok(out)
}

/// Memory words a disk page transcodes to, read from its header, so callers can
/// size the output of [`transcode_page`]. Not in C++ meshoptimizer.
///
/// # Errors
/// `Invalid` for a truncated header; `Unsupported` for an unknown disk codec;
/// `Limit` above [`MAX_PAGE_WORDS`].
pub fn disk_page_memory_words(disk: &[u8]) -> Result<usize, Error> {
    let words = match word(disk, 0)? {
        DISK_NONE => {
            if !disk.len().is_multiple_of(4) {
                return Err(Error::Invalid);
            }
            disk.len() / 4 - 1
        }
        DISK_COMPRESSED => size(word(disk, 4)?)?,
        _ => return Err(Error::Unsupported),
    };
    if words > MAX_PAGE_WORDS {
        return Err(Error::Limit);
    }
    Ok(words)
}

/// Validate a disk page and write its memory form into `out` (no heap allocation;
/// codec work is charged to `workspace`). Returns the memory word count; the
/// written words pass [`PageView::new`]. Not in C++ meshoptimizer.
///
/// # Errors
/// `Limit` if `out` is shorter than [`disk_page_memory_words`]; `Unsupported`
/// for an unknown disk codec or codec version; `Invalid` or `Codec` for
/// malformed input. On error the contents of `out` are unspecified.
pub fn transcode_page(
    disk: &[u8],
    grid: QuantGrid,
    out: &mut [u32],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let m = disk_page_memory_words(disk)?;
    let out = out.get_mut(..m).ok_or(Error::Limit)?;
    if word(disk, 0)? == DISK_NONE {
        for (i, o) in out.iter_mut().enumerate() {
            *o = word(disk, 4 + i * 4)?;
        }
        PageView::new(out, grid)?;
        return Ok(m);
    }
    if m < PAGE_HEADER_WORDS {
        return Err(Error::Invalid);
    }
    for (i, o) in out[..PAGE_HEADER_WORDS].iter_mut().enumerate() {
        *o = word(disk, 8 + i * 4)?;
    }
    let n = size(out[0])?;
    let head = n
        .checked_mul(CLUSTER_WORDS)
        .and_then(|v| v.checked_add(PAGE_HEADER_WORDS))
        .filter(|&v| v <= m)
        .ok_or(Error::Invalid)?;
    for (i, o) in out[PAGE_HEADER_WORDS..head].iter_mut().enumerate() {
        *o = word(disk, 8 + (PAGE_HEADER_WORDS + i) * 4)?;
    }
    let table = 8 + head * 4;
    let mut at = table + n * 4;
    if at > disk.len() {
        return Err(Error::Invalid);
    }
    let mut cursor = head;
    let mut deltas = [0u8; MAX_CLUSTER_VERTICES * 12];
    let mut triangles = [0u8; MAX_CLUSTER_TRIANGLES * 3];
    for c in 0..n {
        let d = PAGE_HEADER_WORDS + c * CLUSTER_WORDS;
        let l = parse_descriptor(&out[d..d + CLUSTER_WORDS])?;
        let nv = l.desc.vertex_count as usize;
        let nt = l.desc.triangle_count as usize;
        let b = l.desc.bits;
        let stride = (b[0] + b[1] + b[2]) as usize;
        let pw = bit_words(nv, stride as u32);
        let end = cursor + pw + triangle_words(nt);
        if l.position_offset != cursor || l.triangle_offset != cursor + pw || end > m {
            return Err(Error::Invalid);
        }
        let entry = word(disk, table + c * 4)?;
        let (pl, tl) = ((entry & 0xffff) as usize, (entry >> 16) as usize);
        let positions = disk.get(at..at + pl).ok_or(Error::Invalid)?;
        let tris = disk.get(at + pl..at + pl + tl).ok_or(Error::Invalid)?;
        at += pl + tl;
        if codec::decode_vertex_version(positions)? != PAGE_VERTEX_CODEC_VERSION {
            return Err(Error::Unsupported);
        }
        codec::decode_vertex_buffer_into(&mut deltas[..nv * 12], nv, 12, positions, workspace)?;
        out[cursor..end].fill(0);
        let mut bit = cursor * 32;
        for (x, &n) in deltas[..nv * 12]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(b.iter().cycle())
        {
            let delta = u32::from_le_bytes(*x);
            if delta >> n != 0 {
                return Err(Error::Invalid);
            }
            write_bits(out, bit, n, delta);
            bit += n as usize;
        }
        meshlet_decode(nv, &mut triangles[..nt * 3], tris, workspace)?;
        for (i, &t) in triangles[..nt * 3].iter().enumerate() {
            out[l.triangle_offset + (i >> 2)] |= u32::from(t) << ((i & 3) * 8);
        }
        cursor = end;
    }
    if at != disk.len() || cursor != m {
        return Err(Error::Invalid);
    }
    PageView::new(out, grid)?;
    Ok(m)
}
