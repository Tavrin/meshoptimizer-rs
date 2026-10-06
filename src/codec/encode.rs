// Public encoder API: explicit per-call version/level configuration replaces
// upstream's global meshopt_encodeVertexVersion/meshopt_encodeIndexVersion.
use super::{filter_encode, index_encode, vertex_encode};
use crate::{workspace::checked_bytes, Error, Workspace};
use alloc::vec::Vec;

/// Vertex codec format version and compression level
/// (the `version` and `level` of meshopt_encodeVertexBufferLevel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexEncoding {
    version: u8,
    level: u8,
}
impl VertexEncoding {
    /// Upstream defaults of meshopt_encodeVertexBuffer: version 1, level 2.
    pub const DEFAULT: Self = Self {
        version: 1,
        level: 2,
    };
    /// Check a version (0 or 1) and level (0 through 9, the range the pinned
    /// reference accepts; 1.3 behaves identically for levels 3 through 9).
    /// Version 0 ignores the level, as upstream does.
    pub const fn new(version: u8, level: u8) -> Result<Self, Error> {
        if version > 1 {
            return Err(Error::UnsupportedVersion);
        }
        if level > 9 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self { version, level })
    }
    /// Format version.
    pub const fn version(self) -> u8 {
        self.version
    }
    /// Compression level.
    pub const fn level(self) -> u8 {
        self.level
    }
}
impl Default for VertexEncoding {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Index buffer and index sequence format version
/// (the setting of meshopt_encodeIndexVersion).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexEncoding {
    version: u8,
}
impl IndexEncoding {
    /// Upstream default: version 1.
    pub const DEFAULT: Self = Self { version: 1 };
    /// Check a version: 0 (decodable by all versions) or 1.
    pub const fn new(version: u8) -> Result<Self, Error> {
        if version > 1 {
            return Err(Error::UnsupportedVersion);
        }
        Ok(Self { version })
    }
    /// Format version.
    pub const fn version(self) -> u8 {
        self.version
    }
}
impl Default for IndexEncoding {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Exponent sharing of the exponential filter encoder (meshopt_EncodeExpMode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpMode {
    /// A separate exponent for each component (maximum quality).
    Separate,
    /// One exponent for all components of a vector.
    SharedVector,
    /// One exponent for each component across all vectors.
    SharedComponent,
    /// Separate exponents clamped to at least zero.
    Clamped,
}

#[inline]
fn vertex_layout(count: usize, stride: usize) -> Result<usize, Error> {
    if stride == 0 || stride > 256 || !stride.is_multiple_of(4) {
        return Err(Error::InvalidLayout);
    }
    checked_bytes(count, stride)
}

/// meshopt_encodeVertexBufferBound: worst-case encoded size for either version.
#[inline]
pub fn encode_vertex_buffer_bound(count: usize, stride: usize) -> Result<usize, Error> {
    if stride == 0 || stride > 256 || !stride.is_multiple_of(4) {
        return Err(Error::InvalidLayout);
    }
    vertex_encode::bound(count, stride)
}

/// meshopt_encodeIndexBufferBound. `index_count` must be a multiple of three.
#[inline]
pub fn encode_index_buffer_bound(index_count: usize, vertex_count: usize) -> Result<usize, Error> {
    if !index_count.is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    index_encode::index_bound(index_count, vertex_count as u64)
}

/// meshopt_encodeIndexSequenceBound.
#[inline]
pub fn encode_index_sequence_bound(
    index_count: usize,
    vertex_count: usize,
) -> Result<usize, Error> {
    index_encode::sequence_bound(index_count, vertex_count as u64)
}

/// Shared allocating driver: account the bound, reserve fallibly, encode and
/// truncate to the used length (the capacity stays at the accounted bound).
fn allocate(
    bound: usize,
    input: usize,
    workspace: &mut Workspace,
    encode: impl FnOnce(&mut [u8]) -> Result<usize, Error>,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let check = (|| {
        workspace.account_codec(bound)?;
        work.add(input)?;
        work.add(bound)
    })();
    workspace.finish(&work);
    check?;
    let mut out = Vec::new();
    out.try_reserve_exact(bound)
        .map_err(|_| Error::AllocationFailed)?;
    if out.capacity() != bound {
        workspace.account_codec(out.capacity())?;
    }
    out.resize(bound, 0);
    let used = encode(&mut out)?;
    out.truncate(used);
    Ok(out)
}

/// Shared caller-buffer driver. Work charges input bytes plus the bytes the
/// encoder may examine in the destination (at most the bound).
fn into(
    bound: usize,
    capacity: usize,
    input: usize,
    workspace: &mut Workspace,
    encode: impl FnOnce() -> Result<usize, Error>,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        workspace.account_codec(0)?;
        work.add(input)?;
        work.add(bound.min(capacity))?;
        encode()
    })();
    workspace.finish(&work);
    result
}

/// Encode a vertex buffer (meshopt_encodeVertexBufferLevel) with explicit
/// version and level into a vector. `vertices` holds exactly count * stride
/// bytes; all bytes, including padding, are encoded verbatim. stride is a
/// multiple of four from 4 to 256.
pub fn encode_vertex_buffer(
    vertices: &[u8],
    count: usize,
    stride: usize,
    encoding: VertexEncoding,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let _ = workspace.begin();
    if vertices.len() != vertex_layout(count, stride)? {
        return Err(Error::InvalidLayout);
    }
    let bound = vertex_encode::bound(count, stride)?;
    allocate(bound, vertices.len(), workspace, |out| {
        vertex_encode::encode(
            out,
            vertices,
            count,
            stride,
            encoding.version,
            encoding.level,
        )
    })
}

/// Encode a vertex buffer into a caller buffer and return the encoded length.
/// Fails with BufferTooSmall exactly where the reference returns 0; the
/// encoder needs slack beyond the final size, so use
/// [`encode_vertex_buffer_bound`]. On failure the buffer prefix may change.
pub fn encode_vertex_buffer_into(
    destination: &mut [u8],
    vertices: &[u8],
    count: usize,
    stride: usize,
    encoding: VertexEncoding,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let _ = workspace.begin();
    if vertices.len() != vertex_layout(count, stride)? {
        return Err(Error::InvalidLayout);
    }
    let bound = vertex_encode::bound(count, stride)?;
    let capacity = destination.len();
    into(bound, capacity, vertices.len(), workspace, || {
        vertex_encode::encode(
            destination,
            vertices,
            count,
            stride,
            encoding.version,
            encoding.level,
        )
    })
}

/// A vertex count giving the same bound as `max index + 1`: the bound only
/// depends on the bit length of the largest index, which the bitwise OR of
/// all indices shares (and an OR reduction vectorizes on baseline x86-64).
fn max_vertices(indices: &[u32]) -> u64 {
    if indices.is_empty() {
        return 0;
    }
    u64::from(indices.iter().fold(0, |a, &b| a | b)) + 1
}

/// Encode a triangle list (meshopt_encodeIndexBuffer) with an explicit version.
/// The length is a multiple of three. Corners may be rotated within each
/// triangle, as upstream does; decoding returns the rotated list.
pub fn encode_index_buffer(
    indices: &[u32],
    encoding: IndexEncoding,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let _ = workspace.begin();
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let bound = index_encode::index_bound(indices.len(), max_vertices(indices))?;
    let input = checked_bytes(indices.len(), 4)?;
    allocate(bound, input, workspace, |out| {
        index_encode::encode_index_buffer(out, indices, encoding.version)
    })
}

/// Encode a triangle list into a caller buffer and return the encoded length.
/// Fails with BufferTooSmall exactly where the reference returns 0; on
/// failure the buffer prefix may change.
pub fn encode_index_buffer_into(
    destination: &mut [u8],
    indices: &[u32],
    encoding: IndexEncoding,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let _ = workspace.begin();
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let bound = index_encode::index_bound(indices.len(), 1 << 32)?;
    let input = checked_bytes(indices.len(), 4)?;
    let capacity = destination.len();
    into(bound, capacity, input, workspace, || {
        index_encode::encode_index_buffer(destination, indices, encoding.version)
    })
}

/// Encode an arbitrary index sequence (meshopt_encodeIndexSequence) with an
/// explicit version.
pub fn encode_index_sequence(
    indices: &[u32],
    encoding: IndexEncoding,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let _ = workspace.begin();
    let bound = index_encode::sequence_bound(indices.len(), max_vertices(indices))?;
    let input = checked_bytes(indices.len(), 4)?;
    allocate(bound, input, workspace, |out| {
        index_encode::encode_index_sequence(out, indices, encoding.version)
    })
}

/// Encode an index sequence into a caller buffer and return the encoded
/// length. Fails with BufferTooSmall exactly where the reference returns 0;
/// on failure the buffer prefix may change.
pub fn encode_index_sequence_into(
    destination: &mut [u8],
    indices: &[u32],
    encoding: IndexEncoding,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let _ = workspace.begin();
    let bound = index_encode::sequence_bound(indices.len(), 1 << 32)?;
    let input = checked_bytes(indices.len(), 4)?;
    let capacity = destination.len();
    into(bound, capacity, input, workspace, || {
        index_encode::encode_index_sequence(destination, indices, encoding.version)
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Oct,
    Quat,
    Exp(ExpMode),
    Color,
}

/// Validate a filter encoder call; returns the output byte length.
fn filter_layout(
    kind: Kind,
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
) -> Result<usize, Error> {
    let (layout, floats, range) = match kind {
        Kind::Oct | Kind::Color => (stride == 4 || stride == 8, 4, 2..=16),
        Kind::Quat => (stride == 8, 4, 4..=16),
        Kind::Exp(_) => (
            stride > 0 && stride <= 256 && stride.is_multiple_of(4),
            stride / 4,
            1..=24,
        ),
    };
    if !layout {
        return Err(Error::InvalidLayout);
    }
    if !range.contains(&bits)
        || (stride == 4 && bits > 8 && matches!(kind, Kind::Oct | Kind::Color))
    {
        return Err(Error::InvalidParameter);
    }
    if data.len() != checked_bytes(count, floats)? {
        return Err(Error::InvalidLayout);
    }
    checked_bytes(count, stride)
}

fn filter_run(
    kind: Kind,
    out: &mut [u8],
    stride: usize,
    bits: u32,
    data: &[f32],
) -> Result<(), Error> {
    match kind {
        Kind::Oct => filter_encode::oct(out, stride, bits, data),
        Kind::Quat => filter_encode::quat(out, bits, data),
        Kind::Color => filter_encode::color(out, stride, bits, data),
        Kind::Exp(mode) => return filter_encode::exp(out, stride, bits, data, mode),
    }
    Ok(())
}

fn filter_into(
    kind: Kind,
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    filter_into_with(
        kind,
        destination,
        count,
        stride,
        bits,
        data,
        workspace,
        |out| filter_run(kind, out, stride, bits, data),
    )
}

// Monomorphize Oct/Quat calls so generic filter dispatch and unrelated kernel
// stack setup do not dominate small inputs. Validation/accounting is shared.
#[allow(clippy::too_many_arguments)]
fn filter_into_with(
    kind: Kind,
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
    run: impl FnOnce(&mut [u8]) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let bytes = filter_layout(kind, count, stride, bits, data)?;
        if destination.len() < bytes {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(0)?;
        work.add(data.len())?;
        work.add(bytes / 4)?;
        run(&mut destination[..bytes])
    })();
    workspace.finish(&work);
    result
}

fn filter_vec(
    kind: Kind,
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    filter_vec_with(kind, count, stride, bits, data, workspace, |out| {
        filter_run(kind, out, stride, bits, data)
    })
}

fn filter_vec_with(
    kind: Kind,
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
    run: impl FnOnce(&mut [u8]) -> Result<(), Error>,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let check = (|| {
        let bytes = filter_layout(kind, count, stride, bits, data)?;
        workspace.account_codec(bytes)?;
        work.add(data.len())?;
        work.add(bytes / 4)?;
        Ok(bytes)
    })();
    workspace.finish(&work);
    let bytes = check?;
    let mut out = Vec::new();
    out.try_reserve_exact(bytes)
        .map_err(|_| Error::AllocationFailed)?;
    if out.capacity() != bytes {
        workspace.account_codec(out.capacity())?;
    }
    out.resize(bytes, 0);
    run(&mut out)?;
    Ok(out)
}

/// Octahedral filter encoder (meshopt_encodeFilterOct): `data` holds four
/// floats per vector; stride 4 takes 2..=8 bits, stride 8 takes 2..=16.
/// Z stores 1.0 and W is quantized to the full byte width.
pub fn encode_filter_oct(
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    filter_vec_with(Kind::Oct, count, stride, bits, data, workspace, |out| {
        filter_encode::oct(out, stride, bits, data);
        Ok(())
    })
}
/// Caller-buffer form of [`encode_filter_oct`]; the destination tail is preserved.
pub fn encode_filter_oct_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    filter_into_with(
        Kind::Oct,
        destination,
        count,
        stride,
        bits,
        data,
        workspace,
        |out| {
            filter_encode::oct(out, stride, bits, data);
            Ok(())
        },
    )
}
/// Quaternion filter encoder (meshopt_encodeFilterQuat): four floats per
/// quaternion, stride 8, 4..=16 bits.
pub fn encode_filter_quat(
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    filter_vec_with(Kind::Quat, count, stride, bits, data, workspace, |out| {
        filter_encode::quat(out, bits, data);
        Ok(())
    })
}
/// Caller-buffer form of [`encode_filter_quat`]; the destination tail is preserved.
pub fn encode_filter_quat_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    filter_into_with(
        Kind::Quat,
        destination,
        count,
        stride,
        bits,
        data,
        workspace,
        |out| {
            filter_encode::quat(out, bits, data);
            Ok(())
        },
    )
}
/// Exponential filter encoder (meshopt_encodeFilterExp): stride / 4 floats
/// per vector, stride a multiple of four up to 256, 1..=24 mantissa bits.
/// Returns NumericalFailure where the reference conversion is undefined
/// (non-finite input; one-bit mantissas with exponent 128); the output may
/// then be partially written.
pub fn encode_filter_exp(
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    mode: ExpMode,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    filter_vec(Kind::Exp(mode), count, stride, bits, data, workspace)
}
/// Caller-buffer form of [`encode_filter_exp`]; the destination tail is preserved.
pub fn encode_filter_exp_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    mode: ExpMode,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    filter_into(
        Kind::Exp(mode),
        destination,
        count,
        stride,
        bits,
        data,
        workspace,
    )
}
/// Color filter encoder (meshopt_encodeFilterColor): four floats per RGBA
/// color into YCoCg-R plus alpha; stride 4 takes 2..=8 bits, stride 8 2..=16.
pub fn encode_filter_color(
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    filter_vec(Kind::Color, count, stride, bits, data, workspace)
}
/// Caller-buffer form of [`encode_filter_color`]; the destination tail is preserved.
pub fn encode_filter_color_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    bits: u32,
    data: &[f32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    filter_into(
        Kind::Color,
        destination,
        count,
        stride,
        bits,
        data,
        workspace,
    )
}
