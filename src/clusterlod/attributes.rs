//! MDAG 2 attribute layout and bounded CPU reference decoding.
use super::dag_format::Error;
use crate::{codec, Attributes, Workspace};
use alloc::vec::Vec;

/// Maximum channels per layout; source components are limited to 32.
pub const MAX_CHANNELS: usize = 16;
/// Attribute encoding. All payloads occupy whole 32-bit words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    /// Unit vector, signed octahedral 16+16 bits; Euclidean error <= 0.000491.
    Oct16x2,
    /// Unit vector, signed octahedral 12+12 bits; Euclidean error <= 0.00782.
    Oct12x2,
    /// Oct16x2 tangent plus exact handedness (-1 or +1), in two words.
    OctSign,
    /// UV pair; component error <= range/131070 plus 4 f32 ulps of the range and minimum.
    UnormRange16x2,
    /// Two finite half floats; component error <= 16, absolute input <= 65504.
    F16x2,
    /// Four finite half floats; component error <= 16, absolute input <= 65504.
    F16x4,
    /// Four components in `[0, 1]`; component error <= 1/510 + 1e-7.
    Unorm8x4,
    /// One to four raw finite f32 components; bit-exact.
    Raw32xN(u8),
}
impl Encoding {
    /// Number of source components.
    pub const fn components(self) -> usize {
        match self {
            Self::Oct16x2 | Self::Oct12x2 => 3,
            Self::OctSign | Self::F16x4 | Self::Unorm8x4 => 4,
            Self::F16x2 | Self::UnormRange16x2 => 2,
            Self::Raw32xN(n) => n as usize,
        }
    }
    /// Words per vertex.
    pub const fn words(self) -> usize {
        match self {
            Self::OctSign | Self::F16x4 => 2,
            Self::Raw32xN(n) => n as usize,
            _ => 1,
        }
    }
    pub(crate) fn code(self) -> Result<u32, Error> {
        Ok(match self {
            Self::Oct16x2 => 1,
            Self::Oct12x2 => 2,
            Self::OctSign => 3,
            Self::UnormRange16x2 => 4,
            Self::F16x2 => 5,
            Self::F16x4 => 6,
            Self::Unorm8x4 => 7,
            Self::Raw32xN(n @ 1..=4) => 7 + u32::from(n),
            _ => return Err(Error::Invalid),
        })
    }
    fn from_code(code: u32) -> Result<Self, Error> {
        Ok(match code {
            1 => Self::Oct16x2,
            2 => Self::Oct12x2,
            3 => Self::OctSign,
            4 => Self::UnormRange16x2,
            5 => Self::F16x2,
            6 => Self::F16x4,
            7 => Self::Unorm8x4,
            8..=11 => Self::Raw32xN((code - 7) as u8),
            _ => return Err(Error::Unsupported),
        })
    }
}
/// One channel declared by ATTR.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Channel {
    /// Opaque caller semantic tag.
    pub tag: u32,
    /// Wire encoding.
    pub encoding: Encoding,
    /// Component offset in the source attribute view.
    pub source_offset: u32,
}
impl Channel {
    pub(crate) fn record(self) -> Result<[u32; 4], Error> {
        Ok([self.tag, self.encoding.code()?, self.source_offset, 0])
    }
    pub(crate) fn parse(r: [u32; 4]) -> Result<Self, Error> {
        let ch = Self {
            tag: r[0],
            encoding: Encoding::from_code(r[1])?,
            source_offset: r[2],
        };
        if r[3] != 0
            || ch.source_offset > 32
            || ch.source_offset as usize + ch.encoding.components() > 32
        {
            return Err(Error::Invalid);
        }
        Ok(ch)
    }
}
/// Validate channel records; unique semantic tags and bounded source offsets.
pub fn validate_layout(channels: &[Channel]) -> Result<(), Error> {
    if channels.len() > MAX_CHANNELS {
        return Err(Error::Limit);
    }
    for (i, &ch) in channels.iter().enumerate() {
        Channel::parse(ch.record()?)?;
        if channels[..i].iter().any(|p| p.tag == ch.tag) {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}
/// Layout hash, identical to the ATTR chunk hash.
pub fn layout_hash(channels: &[Channel]) -> Result<u32, Error> {
    validate_layout(channels)?;
    let mut h = super::dag_format::v2::Xxh64::new(0);
    for &ch in channels {
        for w in ch.record()? {
            h.update(&w.to_le_bytes());
        }
    }
    Ok(h.finish() as u32)
}
fn reserve<T>(v: &mut Vec<T>, n: usize) -> Result<(), Error> {
    v.try_reserve_exact(n).map_err(|_| Error::Allocation)
}

/// Encode a per-cluster attribute block. Headers carry each channel and four
/// range words (minimum.xy, scale.xy); non-range channel headers are zero.
/// # Errors
/// Invalid layout, non-finite or out-of-domain source values, allocation or codec errors.
pub fn encode(
    channels: &[Channel],
    source: Attributes<'_>,
    vertices: &[u32],
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    validate_layout(channels)?;
    if channels.is_empty() || vertices.is_empty() || vertices.len() > 256 {
        return Err(Error::Invalid);
    }
    let stride: usize = channels.iter().map(|ch| ch.encoding.words()).sum();
    let mut out = Vec::new();
    let head = 4 + channels.len() * 8;
    let len = head + stride * vertices.len();
    reserve(&mut out, len)?;
    out.resize(len, 0);
    out[..4].copy_from_slice(&[
        len as u32,
        channels.len() as u32,
        stride as u32,
        vertices.len() as u32,
    ]);
    let mut offset = 0;
    for (k, &ch) in channels.iter().enumerate() {
        let at = 4 + k * 8;
        out[at..at + 4].copy_from_slice(&ch.record()?);
        let e = ch.encoding;
        let mut lo = [f32::MAX; 2];
        let mut hi = [f32::MIN; 2];
        for &v in vertices {
            for c in 0..e.components() {
                let x = source
                    .get(v as usize, ch.source_offset as usize + c)
                    .ok_or(Error::Invalid)?;
                if !x.is_finite() {
                    return Err(Error::Invalid);
                }
                if e == Encoding::UnormRange16x2 {
                    lo[c] = lo[c].min(x);
                    hi[c] = hi[c].max(x);
                }
            }
        }
        let mut scale = [0f32; 2];
        if e == Encoding::UnormRange16x2 {
            for c in 0..2 {
                scale[c] = (f64::from(hi[c]) - f64::from(lo[c])) as f32;
                if !scale[c].is_finite() {
                    return Err(Error::Limit);
                }
            }
            out[at + 4..at + 8].copy_from_slice(&[
                lo[0].to_bits(),
                lo[1].to_bits(),
                scale[0].to_bits(),
                scale[1].to_bits(),
            ]);
        }
        for (v, &src) in vertices.iter().enumerate() {
            let mut x = [0f32; 4];
            for (c, dst) in x.iter_mut().enumerate().take(e.components()) {
                *dst = source
                    .get(src as usize, ch.source_offset as usize + c)
                    .ok_or(Error::Invalid)?;
            }
            let dst = &mut out[head + v * stride + offset..][..e.words()];
            match e {
                Encoding::Oct16x2 | Encoding::Oct12x2 | Encoding::OctSign => {
                    let norm = libm::sqrt(
                        x[..3]
                            .iter()
                            .map(|&x| f64::from(x) * f64::from(x))
                            .sum::<f64>(),
                    );
                    if norm == 0.0 {
                        return Err(Error::Invalid);
                    }
                    for a in &mut x[..3] {
                        *a = (f64::from(*a) / norm) as f32;
                    }
                    if e == Encoding::OctSign && x[3] != -1.0 && x[3] != 1.0 {
                        return Err(Error::Invalid);
                    }
                    let mut packed = [0u8; 8];
                    let bits = if e == Encoding::Oct12x2 { 12 } else { 16 };
                    codec::encode_filter_oct_into(&mut packed, 1, 8, bits, &x, workspace)?;
                    let a = i16::from_le_bytes([packed[0], packed[1]]) as u32;
                    let b = i16::from_le_bytes([packed[2], packed[3]]) as u32;
                    let mask = (1 << bits) - 1;
                    dst[0] = (a & mask) | ((b & mask) << bits);
                    if e == Encoding::OctSign {
                        dst[1] = u32::from(x[3] < 0.0);
                    }
                }
                Encoding::UnormRange16x2 => {
                    let q = [0, 1].map(|c| {
                        if scale[c] == 0.0 {
                            0
                        } else {
                            libm::round(
                                ((f64::from(x[c]) - f64::from(lo[c])) / f64::from(scale[c])
                                    * 65535.0)
                                    .clamp(0.0, 65535.0),
                            ) as u32
                        }
                    });
                    dst[0] = q[0] | q[1] << 16;
                }
                Encoding::F16x2 | Encoding::F16x4 => {
                    for (d, pair) in dst.iter_mut().zip(x.as_chunks::<2>().0) {
                        if pair.iter().any(|x| x.abs() > 65504.0) {
                            return Err(Error::Invalid);
                        }
                        *d = u32::from(crate::quantize_half(pair[0]))
                            | u32::from(crate::quantize_half(pair[1])) << 16;
                    }
                }
                Encoding::Unorm8x4 => {
                    for (c, &x) in x.iter().enumerate() {
                        if !(0.0..=1.0).contains(&x) {
                            return Err(Error::Invalid);
                        }
                        dst[0] |= (libm::roundf(x * 255.0) as u32) << (c * 8);
                    }
                }
                Encoding::Raw32xN(n) => {
                    for (d, x) in dst.iter_mut().zip(&x[..n as usize]) {
                        *d = x.to_bits();
                    }
                }
            }
        }
        offset += e.words();
    }
    Block::new(&out, vertices.len())?;
    Ok(out)
}
/// Validated attribute block; all offsets are bounded before decoding.
#[derive(Clone, Copy, Debug)]
pub struct Block<'a> {
    words: &'a [u32],
    channels: usize,
    stride: usize,
    vertices: usize,
}
impl<'a> Block<'a> {
    /// Validate the complete block, finite decoded values and canonical padding.
    /// # Errors
    /// Typed invalid/unsupported/limit errors for malformed bytes. Work is linear
    /// in block length, with at most 16 channels and 256 vertices.
    pub fn new(words: &'a [u32], vertices: usize) -> Result<Self, Error> {
        if words.len() < 4
            || words[0] as usize != words.len()
            || words[1] == 0
            || words[1] as usize > MAX_CHANNELS
            || !(1..=256).contains(&vertices)
            || words[3] as usize != vertices
        {
            return Err(Error::Invalid);
        }
        let channels = words[1] as usize;
        let head = 4 + channels * 8;
        if head > words.len() {
            return Err(Error::Invalid);
        }
        let mut stride = 0;
        for k in 0..channels {
            let at = 4 + k * 8;
            let ch = Channel::parse(words[at..at + 4].try_into().map_err(|_| Error::Invalid)?)?;
            for j in 0..k {
                if words[4 + j * 8] == ch.tag {
                    return Err(Error::Invalid);
                }
            }
            stride += ch.encoding.words();
            let r = &words[at + 4..at + 8];
            if ch.encoding == Encoding::UnormRange16x2 {
                if r.iter().any(|&x| !f32::from_bits(x).is_finite())
                    || r[2..].iter().any(|&x| f32::from_bits(x) < 0.0)
                {
                    return Err(Error::Invalid);
                }
            } else if r.iter().any(|&x| x != 0) {
                return Err(Error::Invalid);
            }
        }
        if words[2] as usize != stride || head + vertices * stride != words.len() {
            return Err(Error::Invalid);
        }
        let block = Self {
            words,
            channels,
            stride,
            vertices,
        };
        for v in 0..vertices {
            for k in 0..channels {
                block.decode(k, v)?;
            }
        }
        Ok(block)
    }
    /// Number of channels.
    pub fn channel_count(self) -> usize {
        self.channels
    }
    /// Validated channel declaration.
    /// # Errors
    /// Invalid channel index.
    pub fn channel(self, k: usize) -> Result<Channel, Error> {
        if k >= self.channels {
            return Err(Error::Invalid);
        }
        let at = 4 + k * 8;
        Channel::parse(
            self.words[at..at + 4]
                .try_into()
                .map_err(|_| Error::Invalid)?,
        )
    }
    /// Decode one vertex/channel into four components; unused components are zero.
    /// # Errors
    /// Invalid index, noncanonical oct/sign bits, non-finite decoded values.
    pub fn decode(self, k: usize, v: usize) -> Result<[f32; 4], Error> {
        if v >= self.vertices {
            return Err(Error::Invalid);
        }
        let e = self.channel(k)?.encoding;
        let mut offset = 0;
        for j in 0..k {
            offset += self.channel(j)?.encoding.words();
        }
        let at = 4 + self.channels * 8 + v * self.stride + offset;
        let p = &self.words[at..at + e.words()];
        let mut x = [0f32; 4];
        match e {
            Encoding::Oct16x2 | Encoding::Oct12x2 | Encoding::OctSign => {
                let bits = if e == Encoding::Oct12x2 { 12 } else { 16 };
                if bits == 12 && p[0] >> 24 != 0 {
                    return Err(Error::Invalid);
                }
                let sign = |q: u32| ((q << (32 - bits)) as i32) >> (32 - bits);
                let m = ((1 << (bits - 1)) - 1) as f32;
                let a = sign(p[0]);
                let b = sign(p[0] >> bits);
                if a == -(1 << (bits - 1)) || b == -(1 << (bits - 1)) {
                    return Err(Error::Invalid);
                }
                x[0] = a as f32 / m;
                x[1] = b as f32 / m;
                x[2] = 1.0 - x[0].abs() - x[1].abs();
                if x[2] < 0.0 {
                    let a = x[0];
                    x[0] = (1.0 - x[1].abs()) * if a < 0.0 { -1.0 } else { 1.0 };
                    x[1] = (1.0 - a.abs()) * if x[1] < 0.0 { -1.0 } else { 1.0 };
                }
                let len = libm::sqrtf(x[..3].iter().map(|a| a * a).sum());
                for a in &mut x[..3] {
                    *a /= len;
                }
                if e == Encoding::OctSign {
                    if p[1] > 1 {
                        return Err(Error::Invalid);
                    }
                    x[3] = if p[1] == 0 { 1.0 } else { -1.0 };
                }
            }
            Encoding::UnormRange16x2 => {
                let r = &self.words[4 + k * 8 + 4..][..4];
                for c in 0..2 {
                    x[c] = f32::from_bits(r[c])
                        + f32::from_bits(r[c + 2])
                            * (((p[0] >> (c * 16)) & 65535) as f32 / 65535.0);
                }
            }
            Encoding::F16x2 | Encoding::F16x4 => {
                for c in 0..e.components() {
                    let half = (p[c / 2] >> ((c % 2) * 16)) as u16;
                    // The legacy quantization API intentionally flushes subnormals.
                    // Attribute payloads use IEEE half, matching the WGSL decoder.
                    x[c] = if half & 0x7c00 == 0 {
                        let magnitude = f32::from(half & 0x03ff) * (1.0 / 16777216.0);
                        f32::from_bits(magnitude.to_bits() | (u32::from(half & 0x8000) << 16))
                    } else {
                        crate::dequantize_half(half)
                    };
                }
            }
            Encoding::Unorm8x4 => {
                for (c, x) in x.iter_mut().enumerate() {
                    *x = ((p[0] >> (c * 8)) & 255) as f32 / 255.0;
                }
            }
            Encoding::Raw32xN(_) => {
                for (x, &p) in x.iter_mut().zip(p) {
                    *x = f32::from_bits(p);
                }
            }
        }
        if x.iter().any(|x| !x.is_finite()) {
            return Err(Error::Invalid);
        }
        Ok(x)
    }
}
