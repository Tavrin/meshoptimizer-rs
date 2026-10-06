//! meshoptimizer 1.3 codecs, with explicit little-endian byte buffers.
//!
//! Raw vertex, triangle and sequence codecs support versions 0 and 1 for
//! both encoding and decoding. Encoders take explicit per-call
//! [`VertexEncoding`] / [`IndexEncoding`] configuration instead of upstream's
//! global version setters, and produce the reference bytes exactly. Filter
//! encoders cover Oct, Quat, Exp (all four exponent modes) and Color; Color
//! decoding is provided raw, outside the EXT helper. The meshlet codec
//! (encoding plus both decoding forms) works on the 0.3 meshlet layout's
//! vertex-reference and local-triangle slices.
//!
//! The checked EXT helper retains vertex version 0 and the existing Moss index
//! version 0/1 domain. Caller-buffer codecs allocate no heap storage. They
//! preserve unused destination bytes; a malformed body or filter, or an
//! encoder capacity failure, can modify the used prefix. Allocating
//! operations use fallible reservation and Workspace limits. Decoder work
//! charges source bytes plus decoded bytes once before raw decoding, and one
//! visit per four-byte word before filtering, including validation. Encoder
//! work charges input bytes plus at most the bound's output bytes; filter
//! encoders charge input floats plus output words. Fixed block arrays on the
//! stack and caller buffers are excluded from storage limits, consistently
//! with the geometry APIs.

mod encode;
#[cfg(feature = "simd")]
mod simd;

/// Diagnostic dispatch control; outside semver. Overrides only lower detected ISA.
#[cfg(all(feature = "simd", feature = "parity-internals"))]
#[doc(hidden)]
pub use simd::dispatch::{detected_level, with_level, Level};

mod filter;
mod filter_encode;
mod index;
mod index_encode;
mod meshlet;
mod vertex;
mod vertex_encode;
use crate::{workspace::checked_bytes, Error, Workspace};
use alloc::vec::Vec;
pub use encode::{
    encode_filter_color, encode_filter_color_into, encode_filter_exp, encode_filter_exp_into,
    encode_filter_oct, encode_filter_oct_into, encode_filter_quat, encode_filter_quat_into,
    encode_index_buffer, encode_index_buffer_bound, encode_index_buffer_into,
    encode_index_sequence, encode_index_sequence_bound, encode_index_sequence_into,
    encode_vertex_buffer, encode_vertex_buffer_bound, encode_vertex_buffer_into, ExpMode,
    IndexEncoding, VertexEncoding,
};
pub use meshlet::{
    decode_meshlet, decode_meshlet_into, decode_meshlet_raw, decode_meshlet_raw_into,
    encode_meshlet, encode_meshlet_bound, encode_meshlet_into, DecodedMeshlet, RawMeshlet,
};

/// EXT compression mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Interleaved attributes, decoded by the vertex codec.
    Attributes,
    /// Triangle-list indices.
    Triangles,
    /// Arbitrary index sequence.
    Indices,
}
impl Mode {
    /// Parse an EXT mode name, rejecting unknown names.
    pub fn parse(name: &str) -> Result<Self, Error> {
        match name {
            "ATTRIBUTES" => Ok(Self::Attributes),
            "TRIANGLES" => Ok(Self::Triangles),
            "INDICES" => Ok(Self::Indices),
            _ => Err(Error::InvalidParameter),
        }
    }
}
/// EXT post-decode filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    /// Preserve decoded bytes.
    None,
    /// Octahedral normals/tangents.
    Octahedral,
    /// Unit quaternions.
    Quaternion,
    /// Exponent/mantissa encoded floats.
    Exponential,
}
impl Filter {
    /// Parse an EXT filter name, rejecting unknown names.
    pub fn parse(name: &str) -> Result<Self, Error> {
        match name {
            "NONE" => Ok(Self::None),
            "OCTAHEDRAL" => Ok(Self::Octahedral),
            "QUATERNION" => Ok(Self::Quaternion),
            "EXPONENTIAL" => Ok(Self::Exponential),
            _ => Err(Error::InvalidParameter),
        }
    }
}
/// Validated EXT buffer-view layout, independent of glTF JSON and resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferView {
    mode: Mode,
    filter: Filter,
    count: usize,
    stride: usize,
    bytes: usize,
}
impl BufferView {
    /// Check mode, filter, count and stride. EXT count must be positive.
    pub fn new(mode: Mode, filter: Filter, count: usize, stride: usize) -> Result<Self, Error> {
        if count == 0 {
            return Err(Error::InvalidParameter);
        }
        let bytes = layout(mode, count, stride)?;
        if mode != Mode::Attributes && filter != Filter::None {
            return Err(Error::InvalidParameter);
        }
        filter_layout(filter, stride)?;
        Ok(Self {
            mode,
            filter,
            count,
            stride,
            bytes,
        })
    }
    /// Required uncompressed byte length.
    pub const fn byte_length(self) -> usize {
        self.bytes
    }
    /// Check parent bufferView byteLength and its optional byteStride.
    pub fn validate_parent(
        self,
        byte_length: usize,
        byte_stride: Option<usize>,
    ) -> Result<(), Error> {
        if byte_length != self.bytes || byte_stride.is_some_and(|s| s != self.stride) {
            return Err(Error::InvalidLayout);
        }
        Ok(())
    }
    /// Allocate and decode this EXT view using explicit per-call limits.
    pub fn decode(self, source: &[u8], workspace: &mut Workspace) -> Result<Vec<u8>, Error> {
        decode_buffer_view(
            self.mode,
            self.filter,
            self.count,
            self.stride,
            source,
            workspace,
        )
    }
    /// Decode into a caller buffer, without heap allocation.
    /// A late stream/filter error can modify the used destination prefix.
    pub fn decode_into(
        self,
        destination: &mut [u8],
        source: &[u8],
        workspace: &mut Workspace,
    ) -> Result<(), Error> {
        decode_buffer_view_into(
            destination,
            self.mode,
            self.filter,
            self.count,
            self.stride,
            source,
            workspace,
        )
    }
}
fn layout(mode: Mode, count: usize, stride: usize) -> Result<usize, Error> {
    match mode {
        Mode::Attributes if stride == 0 || stride > 256 || !stride.is_multiple_of(4) => {
            return Err(Error::InvalidLayout)
        }
        Mode::Triangles | Mode::Indices if stride != 2 && stride != 4 => {
            return Err(Error::InvalidLayout)
        }
        Mode::Triangles if !count.is_multiple_of(3) => return Err(Error::InvalidTopology),
        _ => {}
    }
    checked_bytes(count, stride)
}
fn filter_layout(filter: Filter, stride: usize) -> Result<(), Error> {
    let valid = match filter {
        Filter::None => true,
        Filter::Octahedral => stride == 4 || stride == 8,
        Filter::Quaternion => stride == 8,
        Filter::Exponential => stride > 0 && stride.is_multiple_of(4),
    };
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidLayout)
    }
}
pub(super) fn byte_at(data: &[u8], position: usize) -> Result<u8, Error> {
    data.get(position).copied().ok_or(Error::InvalidStream)
}
fn version(source: &[u8], headers: &[u8]) -> Result<u8, Error> {
    let header = byte_at(source, 0)?;
    if !headers.contains(&(header & 0xf0)) {
        return Err(Error::InvalidStream);
    }
    let v = header & 15;
    if v > 1 {
        Err(Error::UnsupportedVersion)
    } else {
        Ok(v)
    }
}
/// Inspect a raw vertex codec header (meshopt_decodeVertexVersion).
/// This does not validate the stream body.
pub fn decode_vertex_version(source: &[u8]) -> Result<u8, Error> {
    version(source, &[0xa0])
}
/// Inspect either triangle or sequence codec header (meshopt_decodeIndexVersion).
/// This does not validate the stream body.
pub fn decode_index_version(source: &[u8]) -> Result<u8, Error> {
    version(source, &[0xe0, 0xd0])
}

fn raw(
    mode: Mode,
    destination: &mut [u8],
    count: usize,
    stride: usize,
    source: &[u8],
) -> Result<(), Error> {
    match mode {
        Mode::Attributes => vertex::decode(destination, count, stride, source),
        Mode::Triangles => index::triangles(destination, count, stride, source),
        Mode::Indices => index::sequence(destination, count, stride, source),
    }
}
fn apply(filter: Filter, data: &mut [u8], stride: usize) -> Result<(), Error> {
    match filter {
        Filter::None => Ok(()),
        Filter::Octahedral => filter::oct(data, stride),
        Filter::Quaternion => filter::quat(data),
        Filter::Exponential => {
            filter::exp(data, stride);
            Ok(())
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn run(
    mode: Mode,
    filter: Filter,
    destination: &mut [u8],
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
    owned: usize,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let bytes = layout(mode, count, stride)?;
        if destination.len() < bytes {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(owned)?;
        work.add(source.len())?;
        work.add(bytes)?;
        if filter != Filter::None {
            work.add(bytes / 4)?;
        }
        raw(mode, &mut destination[..bytes], count, stride, source)?;
        apply(filter, &mut destination[..bytes], stride)
    })();
    workspace.finish(&work);
    result
}
fn allocate(
    mode: Mode,
    filter: Filter,
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let bytes = layout(mode, count, stride)?;
    // Check complete resources before requesting an allocation or mutating output.
    let check = (|| {
        workspace.account_codec(bytes)?;
        work.add(source.len())?;
        work.add(bytes)?;
        if filter != Filter::None {
            work.add(bytes / 4)?;
        }
        Ok(())
    })();
    workspace.finish(&work);
    check?;
    preflight(mode, count, stride, source)?;
    let mut out = Vec::new();
    out.try_reserve_exact(bytes)
        .map_err(|_| Error::AllocationFailed)?;
    // Account actual retained output capacity if the allocator rounded up.
    if out.capacity() != bytes {
        workspace.account_codec(out.capacity())?;
    }
    if mode == Mode::Attributes {
        vertex::decode(&mut out, count, stride, source)?;
    } else if mode == Mode::Indices {
        index::sequence_append(&mut out, count, stride, source)?;
    } else {
        out.resize(bytes, 0);
        raw(mode, &mut out, count, stride, source)?;
    }
    apply(filter, &mut out, stride)?;
    Ok(out)
}

fn preflight(mode: Mode, count: usize, stride: usize, source: &[u8]) -> Result<(), Error> {
    let minimum =
        match mode {
            Mode::Attributes => {
                let version = decode_vertex_version(source)?;
                // Valid vertex strides imply a block of at least 32 records.
                // Tiny calls therefore need no block-size or quotient division.
                let (full, rest, block) = if count <= 32 {
                    (0, count, 32)
                } else {
                    let block = ((8192 / stride) & !15).min(256);
                    (count / block, count % block, block)
                };
                let body = if version == 0 {
                    let header = |n: usize| n.div_ceil(16).div_ceil(4);
                    (full * header(block) + if rest > 0 { header(rest) } else { 0 })
                        .checked_mul(stride)
                        .ok_or(Error::SizeOverflow)?
                } else {
                    (full + usize::from(rest > 0))
                        .checked_mul(stride / 4)
                        .ok_or(Error::SizeOverflow)?
                };
                let tail = (stride + if version == 0 { 0 } else { stride / 4 })
                    .max(if version == 0 { 32 } else { 24 });
                body.checked_add(tail)
                    .and_then(|n| n.checked_add(1))
                    .ok_or(Error::SizeOverflow)?
            }
            Mode::Triangles => count / 3 + 17,
            Mode::Indices => count + 5,
        };
    if source.len() < minimum {
        return Err(Error::InvalidStream);
    }
    Ok(())
}
/// Decode raw vertex codec version 0/1 (meshopt_decodeVertexBuffer).
pub fn decode_vertex_buffer(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    allocate(
        Mode::Attributes,
        Filter::None,
        count,
        stride,
        source,
        workspace,
    )
}
/// Decode raw vertices into a caller buffer without heap allocation.
/// On a late error the used prefix can change; destination tail is preserved.
pub fn decode_vertex_buffer_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    run(
        Mode::Attributes,
        Filter::None,
        destination,
        count,
        stride,
        source,
        workspace,
        0,
    )
}
/// Decode triangle codec version 0/1 (meshopt_decodeIndexBuffer).
/// Two-byte output retains upstream truncating u16 conversion semantics.
pub fn decode_index_buffer(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    allocate(
        Mode::Triangles,
        Filter::None,
        count,
        stride,
        source,
        workspace,
    )
}
/// Decode triangle indices into a caller buffer without heap allocation.
/// On a late error the used prefix can change; destination tail is preserved.
pub fn decode_index_buffer_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    run(
        Mode::Triangles,
        Filter::None,
        destination,
        count,
        stride,
        source,
        workspace,
        0,
    )
}
/// Decode sequence codec version 0/1 (meshopt_decodeIndexSequence).
/// Two-byte output retains upstream truncating u16 conversion semantics.
pub fn decode_index_sequence(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    allocate(
        Mode::Indices,
        Filter::None,
        count,
        stride,
        source,
        workspace,
    )
}
/// Decode an index sequence into a caller buffer without heap allocation.
/// On a late error the used prefix can change; destination tail is preserved.
pub fn decode_index_sequence_into(
    destination: &mut [u8],
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    run(
        Mode::Indices,
        Filter::None,
        destination,
        count,
        stride,
        source,
        workspace,
        0,
    )
}
fn ext(
    mode: Mode,
    filter: Filter,
    count: usize,
    stride: usize,
    source: &[u8],
) -> Result<(), Error> {
    BufferView::new(mode, filter, count, stride)?;
    if mode == Mode::Attributes && decode_vertex_version(source)? != 0 {
        return Err(Error::UnsupportedVersion);
    }
    Ok(())
}
/// Decode one checked EXT view; vertex v1 remains raw-only.
/// Parent metadata can additionally be checked with BufferView::validate_parent.
pub fn decode_buffer_view(
    mode: Mode,
    filter: Filter,
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let _ = workspace.begin();
    ext(mode, filter, count, stride, source)?;
    allocate(mode, filter, count, stride, source, workspace)
}
/// Decode a checked EXT view into a caller buffer, without allocation.
/// On a late stream/filter error the used prefix can change.
#[allow(clippy::too_many_arguments)]
pub fn decode_buffer_view_into(
    destination: &mut [u8],
    mode: Mode,
    filter: Filter,
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let _ = workspace.begin();
    ext(mode, filter, count, stride, source)?;
    run(
        mode,
        filter,
        destination,
        count,
        stride,
        source,
        workspace,
        0,
    )
}
fn post<const EXP: bool>(
    filter: Filter,
    data: &mut [u8],
    count: usize,
    stride: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let filter = if EXP { Filter::Exponential } else { filter };
    let mut work = workspace.begin();
    let result = (|| {
        filter_layout(filter, stride)?;
        let bytes = checked_bytes(count, stride)?;
        if data.len() < bytes {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(0)?;
        work.add(bytes / 4)?;
        apply(filter, &mut data[..bytes], stride)
    })();
    workspace.finish(&work);
    result
}
/// Apply canonical scalar Oct decoding (meshopt_decodeFilterOct).
/// Zero-length normals return NumericalFailure instead of C++ undefined conversion.
/// A late numerical error can modify preceding records; the tail is preserved.
pub fn decode_filter_oct(
    data: &mut [u8],
    count: usize,
    stride: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    post::<false>(Filter::Octahedral, data, count, stride, workspace)
}
/// Apply canonical scalar Quat decoding (meshopt_decodeFilterQuat).
/// A late numerical error can modify preceding records; the tail is preserved.
pub fn decode_filter_quat(
    data: &mut [u8],
    count: usize,
    stride: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    post::<false>(Filter::Quaternion, data, count, stride, workspace)
}
/// Apply scalar Exp decoding (meshopt_decodeFilterExp), preserving float bits.
pub fn decode_filter_exp(
    data: &mut [u8],
    count: usize,
    stride: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    post::<true>(Filter::Exponential, data, count, stride, workspace)
}
/// Apply canonical scalar Color decoding (meshopt_decodeFilterColor): YCoCg-R
/// plus alpha back to RGBA, stride 4 (u8) or 8 (u16). Outside the EXT
/// minimum, so not offered by [`BufferView`]. Returns NumericalFailure where
/// the C++ float-to-int conversion is undefined (zero alpha word, or a 16-bit
/// component scaled outside the i32 range); the data may then be partially decoded.
pub fn decode_filter_color(
    data: &mut [u8],
    count: usize,
    stride: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if stride != 4 && stride != 8 {
            return Err(Error::InvalidLayout);
        }
        let bytes = checked_bytes(count, stride)?;
        if data.len() < bytes {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(0)?;
        work.add(bytes / 4)?;
        filter::color(&mut data[..bytes], stride)
    })();
    workspace.finish(&work);
    result
}

/// Hardware sqrt diagnostic for the exhaustive target qualification; outside semver.
#[cfg(all(feature = "simd", feature = "parity-internals"))]
#[doc(hidden)]
pub fn simd_sqrt4(values: [f32; 4]) -> Option<[f32; 4]> {
    simd::sqrt4(values)
}
