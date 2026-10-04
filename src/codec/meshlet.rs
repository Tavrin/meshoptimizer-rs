// meshoptimizer 1.3 meshlet codec (meshletcodec.cpp, scalar paths), MIT,
// Arseny Kapoulkine.
//
// Kept in its own module: it consumes the 0.3 meshlet layout only as plain
// slices (u32 vertex references and three local u8 indices per triangle), so
// joining it to the 0.3 meshlet types is a re-export, not a format change.
use crate::{workspace::checked_bytes, Error, Workspace};
use alloc::vec::Vec;

const MAX: usize = 256;
use super::index_encode::rotate;

fn layout(vertex_count: usize, triangle_count: usize) -> (usize, usize, usize) {
    let codes = triangle_count.div_ceil(2);
    let ctrl = vertex_count.div_ceil(4);
    let gap = 16usize.saturating_sub(codes + ctrl);
    (codes, ctrl, gap)
}

/// meshopt_encodeMeshletBound: worst-case encoded size, with checked arithmetic.
#[inline]
pub fn encode_meshlet_bound(max_vertices: usize, max_triangles: usize) -> Result<usize, Error> {
    // Below 2^26 each, the result is below 2^31: plain arithmetic suffices.
    if (max_vertices | max_triangles) < 1 << 26 {
        let codes = max_triangles.div_ceil(2);
        let groups = max_vertices.div_ceil(4);
        let gap = 16usize.saturating_sub(codes + groups);
        return Ok(codes + max_triangles * 3 + groups + groups * 16 + gap);
    }
    let codes = max_triangles / 2 + max_triangles % 2;
    let extra = max_triangles.checked_mul(3).ok_or(Error::SizeOverflow)?;
    let groups = max_vertices / 4 + usize::from(!max_vertices.is_multiple_of(4));
    let data = groups.checked_mul(16).ok_or(Error::SizeOverflow)?;
    let fixed = codes.checked_add(groups).ok_or(Error::SizeOverflow)?;
    let gap = 16usize.saturating_sub(fixed);
    fixed
        .checked_add(extra)
        .and_then(|n| n.checked_add(data))
        .and_then(|n| n.checked_add(gap))
        .ok_or(Error::SizeOverflow)
}

fn edge_fifo(fifo: &[[u32; 2]; 8], a: u32, b: u32, c: u32, offset: usize) -> Option<usize> {
    for i in 0..8 {
        let [e0, e1] = fifo[offset.wrapping_sub(1 + i) & 7];
        if e0 == a && e1 == b {
            return Some(i << 2);
        }
        if e0 == b && e1 == c {
            return Some((i << 2) | 1);
        }
        if e0 == c && e1 == a {
            return Some((i << 2) | 2);
        }
    }
    None
}

fn encode_triangles(codes: &mut [u8], extra: &mut [u8], triangles: &[u8]) -> usize {
    let mut fifo = [[u32::MAX; 2]; 8];
    let mut offset = 0usize;
    let push = |fifo: &mut [[u32; 2]; 8], offset: &mut usize, a: u32, b: u32| {
        fifo[*offset] = [a, b];
        *offset = (*offset + 1) & 7;
    };
    let mut next = 0u32;
    let mut n = 0usize;
    codes.fill(0);
    for (i, t) in triangles.as_chunks::<3>().0.iter().enumerate() {
        let t = t.map(u32::from);
        let code;
        match edge_fifo(&fifo, t[0], t[1], t[2], offset) {
            Some(fer) if (fer >> 2) < 6 => {
                let (a, b, c) = rotate(&t, fer & 3);
                let fec = if c == next {
                    next += 1;
                    0
                } else {
                    1
                };
                code = (fer >> 2) * 2 + fec;
                if fec != 0 {
                    extra[n] = c as u8;
                    n += 1;
                }
                push(&mut fifo, &mut offset, c, b);
                push(&mut fifo, &mut offset, a, c);
            }
            _ => {
                // Rotate to minimize the need for extra vertices.
                let rotation = if t[0] > t[1] && t[0] > t[2] {
                    1
                } else if t[1] > t[2] {
                    2
                } else {
                    0
                };
                let (a, b, c) = rotate(&t, rotation);
                // Once a vertex uses next, all later vertices of the triangle do.
                let fea = if a == next && b == next + 1 && c == next + 2 {
                    next += 1;
                    0
                } else {
                    1
                };
                let feb = if b == next && c == next + 1 {
                    next += 1;
                    0
                } else {
                    1
                };
                let fec = if c == next {
                    next += 1;
                    0
                } else {
                    1
                };
                code = 12 + fea + feb + fec;
                for (flag, v) in [(fea, a), (feb, b), (fec, c)] {
                    if flag != 0 {
                        extra[n] = v as u8;
                        n += 1;
                    }
                }
                push(&mut fifo, &mut offset, c, b);
                push(&mut fifo, &mut offset, a, c);
            }
        }
        codes[i / 2] |= (code as u8) << ((i & 1) * 4);
    }
    n
}

/// Writes four bytes per value and advances by its length, so `data` needs
/// sixteen bytes per group of four (the bound's worst case) of room.
fn encode_vertices(ctrl: &mut [u8], data: &mut [u8], vertices: &[u32]) -> usize {
    ctrl.fill(0);
    let mut n = 0;
    let mut last = u32::MAX;
    for (control, group) in ctrl.iter_mut().zip(vertices.chunks(4)) {
        let mut gv = [0u32; 4];
        for (g, &v) in gv.iter_mut().zip(group) {
            let d = v.wrapping_sub(last).wrapping_sub(1);
            *g = (d << 1) ^ ((d as i32) >> 31) as u32;
            last = v;
        }
        // Four bytes for all values if any needs four or all need three.
        let use4 = (gv[0] | gv[1] | gv[2] | gv[3]) > 0xff_ffff || gv.iter().all(|&v| v > 0xffff);
        for (k, v) in gv.into_iter().enumerate() {
            let code: u8 = if use4 {
                3
            } else if v == 0 {
                0
            } else if v < 256 {
                1
            } else if v < 65536 {
                2
            } else {
                3
            };
            let length = if use4 { 4 } else { usize::from(code) };
            // Write all four bytes (the array has slack) and keep `length`.
            data[n..n + 4].copy_from_slice(&v.to_le_bytes());
            n += length;
            // Low and high code bits are split into two nibbles.
            *control |= ((code & 1) << k) | ((code >> 1) << (k + 4));
        }
    }
    n
}

#[inline(always)]
fn check_counts(vertex_count: usize, triangle_count: usize) -> Result<(), Error> {
    if vertex_count > MAX || triangle_count > MAX {
        return Err(Error::InvalidParameter);
    }
    Ok(())
}

/// Encode one meshlet (meshopt_encodeMeshlet) into a caller buffer and return
/// the encoded length. `triangles` holds three local u8 indices per triangle,
/// as in the 0.3 meshlet layout; both counts are at most 256. When the buffer
/// is too small, BufferTooSmall is returned and the buffer is unchanged.
pub fn encode_meshlet_into(
    destination: &mut [u8],
    vertices: &[u32],
    triangles: &[u8],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !triangles.len().is_multiple_of(3) {
            return Err(Error::InvalidTopology);
        }
        let triangle_count = triangles.len() / 3;
        check_counts(vertices.len(), triangle_count)?;
        workspace.account_codec(0)?;
        work.add(vertices.len() * 4 + triangles.len())?;
        if destination.len() >= encode_meshlet_bound(vertices.len(), triangle_count)? {
            Ok(encode_direct(destination, vertices, triangles))
        } else {
            // Stage so that a buffer found too small is left unchanged.
            Encoded::new(vertices, triangles).write(destination)
        }
    })();
    workspace.finish(&work);
    result
}

/// Encode one meshlet (meshopt_encodeMeshlet) into a vector of the encoded
/// length (capacity: [`encode_meshlet_bound`], accounted before reserving).
pub fn encode_meshlet(
    vertices: &[u32],
    triangles: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !triangles.len().is_multiple_of(3) {
            return Err(Error::InvalidTopology);
        }
        check_counts(vertices.len(), triangles.len() / 3)?;
        work.add(vertices.len() * 4 + triangles.len())?;
        let bound = encode_meshlet_bound(vertices.len(), triangles.len() / 3)?;
        workspace.account_codec(bound)?;
        let mut out = Vec::new();
        out.try_reserve_exact(bound)
            .map_err(|_| Error::AllocationFailed)?;
        if out.capacity() != bound {
            workspace.account_codec(out.capacity())?;
        }
        out.resize(bound, 0);
        let size = encode_direct(&mut out, vertices, triangles);
        out.truncate(size);
        Ok(out)
    })();
    workspace.finish(&work);
    result
}

/// Encode straight into `out`, which holds at least the bound: vertex data
/// first, then triangle extra bytes, the gap, control bytes and codes.
fn encode_direct(out: &mut [u8], vertices: &[u32], triangles: &[u8]) -> usize {
    let (codes_size, ctrl_size, gap) = layout(vertices.len(), triangles.len() / 3);
    let mut codes = [0u8; MAX / 2];
    let mut ctrl = [0u8; MAX / 4];
    let data = encode_vertices(&mut ctrl[..ctrl_size], out, vertices);
    let extra = encode_triangles(&mut codes[..codes_size], &mut out[data..], triangles);
    let mut pos = data + extra;
    for part in [&[0u8; 16][..gap], &ctrl[..ctrl_size], &codes[..codes_size]] {
        out[pos..pos + part.len()].copy_from_slice(part);
        pos += part.len();
    }
    pos
}

/// Encoded sections, staged on the stack exactly as upstream stages them.
struct Encoded {
    codes: [u8; MAX / 2],
    extra: [u8; MAX * 3],
    ctrl: [u8; MAX / 4],
    data: [u8; MAX * 4 + 4],
    sizes: [usize; 5],
}
impl Encoded {
    fn new(vertices: &[u32], triangles: &[u8]) -> Self {
        let mut e = Self {
            codes: [0; MAX / 2],
            extra: [0; MAX * 3],
            ctrl: [0; MAX / 4],
            data: [0; MAX * 4 + 4],
            sizes: [0; 5],
        };
        let (codes, ctrl, gap) = layout(vertices.len(), triangles.len() / 3);
        let extra = encode_triangles(&mut e.codes[..codes], &mut e.extra, triangles);
        let data = encode_vertices(&mut e.ctrl[..ctrl], &mut e.data, vertices);
        e.sizes = [data, extra, gap, ctrl, codes];
        e
    }
    fn len(&self) -> usize {
        self.sizes.iter().sum()
    }
    /// Variable-size data first, then the gap, then fixed-size control and codes.
    fn write(&self, out: &mut [u8]) -> Result<usize, Error> {
        let size = self.len();
        if size > out.len() {
            return Err(Error::BufferTooSmall);
        }
        let [data, extra, gap, ctrl, codes] = self.sizes;
        let mut pos = 0;
        for part in [
            &self.data[..data],
            &self.extra[..extra],
            &[0u8; 16][..gap],
            &self.ctrl[..ctrl],
            &self.codes[..codes],
        ] {
            out[pos..pos + part.len()].copy_from_slice(part);
            pos += part.len();
        }
        Ok(size)
    }
}

/// Output element widths for [`decode_meshlet_into`].
#[inline(always)]
fn sizes(vertex_size: usize, triangle_size: usize) -> Result<(), Error> {
    if (vertex_size != 2 && vertex_size != 4) || (triangle_size != 3 && triangle_size != 4) {
        return Err(Error::InvalidLayout);
    }
    Ok(())
}

struct Stream<'a> {
    codes: &'a [u8],
    ctrl: &'a [u8],
    source: &'a [u8],
    bound: usize,
    vertex_count: usize,
}

#[inline(always)]
fn stream(source: &[u8], vertex_count: usize, triangle_count: usize) -> Result<Stream<'_>, Error> {
    // Every caller has checked both counts before parsing the stream.
    let (codes, ctrl, gap) = layout(vertex_count, triangle_count);
    if source.len() < codes + ctrl + gap {
        return Err(Error::InvalidStream);
    }
    let end = source.len();
    Ok(Stream {
        codes: &source[end - codes..],
        ctrl: &source[end - codes - ctrl..end - codes],
        source,
        // At least sixteen readable bytes always follow the bound.
        bound: end - codes - ctrl - gap,
        vertex_count,
    })
}

/// Decode a whole stream, storing vertex references and edge-format
/// triangles (0xcbac) through the given writers. A malformed stream can leave
/// earlier elements written.
#[inline(always)]
fn decode_core(
    s: &Stream<'_>,
    triangle_count: usize,
    vertices: impl FnMut(usize, u32),
    triangles: impl FnMut(usize, u32),
) -> Result<(), Error> {
    let data = decode_vertices(s, vertices)?;
    let end = decode_triangles(s, data, triangle_count, triangles)?;
    if end != s.bound {
        return Err(Error::InvalidStream);
    }
    Ok(())
}

/// Byte-output decoding, monomorphized per element width.
#[inline(always)]
fn decode_bytes<const VS: usize, const TS: usize>(
    s: &Stream<'_>,
    triangle_count: usize,
    vertices: &mut [u8],
    triangles: &mut [u8],
) -> Result<(), Error> {
    let vertices = vertices.as_chunks_mut::<VS>().0;
    let triangles = triangles.as_chunks_mut::<TS>().0;
    decode_core(
        s,
        triangle_count,
        |i, r| {
            // Truncating u16 references, as upstream.
            vertices[i].copy_from_slice(&r.to_le_bytes()[..VS]);
        },
        |i, tri| {
            // Stored without the extra edge vertex: 0xcbac becomes a | b << 8 | c << 16.
            triangles[i].copy_from_slice(&(tri >> 8).to_le_bytes()[..TS]);
        },
    )
}

/// Decode vertex references into `out` (at most 256, one group of four per
/// control byte). Each group reads one bounded 16-byte window: the group
/// starts at or before `bound`, and sixteen readable bytes follow `bound`.
#[inline(always)]
fn decode_vertices(s: &Stream<'_>, mut out: impl FnMut(usize, u32)) -> Result<usize, Error> {
    let count = s.vertex_count;
    let mut data = 0usize;
    let mut last = u32::MAX;
    for (g, &code4) in s.ctrl.iter().enumerate() {
        if data > s.bound {
            return Err(Error::InvalidStream);
        }
        let window = s.source.get(data..data + 16).ok_or(Error::InvalidStream)?;
        let mut offset = 0usize;
        for k in 0..4 {
            let code = ((code4 >> k) & 1) | ((code4 >> (k + 3)) & 2);
            let length = if code4 == 0xff { 4 } else { u32::from(code) };
            let v = match length {
                0 => 0,
                1 => u32::from(window[offset]),
                2 => u32::from(u16::from_le_bytes([window[offset], window[offset + 1]])),
                3 => {
                    u32::from_le_bytes([window[offset], window[offset + 1], window[offset + 2], 0])
                }
                _ => u32::from_le_bytes([
                    window[offset],
                    window[offset + 1],
                    window[offset + 2],
                    window[offset + 3],
                ]),
            };
            let d = (v >> 1) ^ 0u32.wrapping_sub(v & 1);
            let r = last.wrapping_add(d).wrapping_add(1);
            if g * 4 + k < count {
                out(g * 4 + k, r);
            }
            offset += length as usize;
            last = r;
        }
        data += offset;
    }
    Ok(data)
}

/// Decode triangles in edge format (0xcbac) into `out`.
fn decode_triangles(
    s: &Stream<'_>,
    mut extra: usize,
    triangle_count: usize,
    mut out: impl FnMut(usize, u32),
) -> Result<usize, Error> {
    let mut next = 0u32;
    let mut fifo = [0u32; 3];
    for i in 0..triangle_count {
        if extra > s.bound {
            return Err(Error::InvalidStream);
        }
        // At most three extra bytes, all within the sixteen after `bound`.
        let w = s.source.get(extra..extra + 3).ok_or(Error::InvalidStream)?;
        let (w0, w1, w2) = (u32::from(w[0]), u32::from(w[1]), u32::from(w[2]));
        let code = u32::from(s.codes[i / 2] >> ((i & 1) * 4)) & 15;
        let tri = if code < 12 {
            // Reuse an edge of the last three triangles; bit 0 selects next or extra.
            let edge = fifo[(code / 4) as usize] >> ((code << 3) & 16);
            let fec = code & 1;
            let c = if fec != 0 { w0 } else { next };
            next = next.wrapping_add(1 - fec);
            extra += fec as usize;
            ((edge & 0xff) << 16) | (edge & 0xff00) | c | (c << 24)
        } else {
            // Restart: fea >= feb >= fec; extra bytes are consumed in order.
            let (fea, feb, fec) = (code > 12, code > 13, code > 14);
            let mut taken = 0;
            let mut take = |flag: bool| {
                if flag {
                    taken += 1;
                    [w0, w1, w2][taken - 1]
                } else {
                    let v = next;
                    next = next.wrapping_add(1);
                    v
                }
            };
            let a = take(fea);
            let b = take(feb);
            let c = take(fec);
            extra += taken;
            c | (a << 8) | (b << 16) | (c << 24)
        };
        out(i, tri);
        fifo = [tri, fifo[0], fifo[1]];
    }
    Ok(extra)
}

/// Decode one meshlet (meshopt_decodeMeshlet) into caller buffers without heap
/// allocation. vertex_size is 2 or 4 (little-endian, u16 output truncates as
/// upstream); triangle_size is 3 (three u8 indices) or 4 (packed
/// a | b << 8 | c << 16, little-endian). Exactly count * size bytes are written.
/// A malformed stream can leave the used prefix modified; tails are preserved.
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn decode_meshlet_into(
    vertices: &mut [u8],
    vertex_count: usize,
    vertex_size: usize,
    triangles: &mut [u8],
    triangle_count: usize,
    triangle_size: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        sizes(vertex_size, triangle_size)?;
        check_counts(vertex_count, triangle_count)?;
        let vb = vertex_count * vertex_size;
        let tb = triangle_count * triangle_size;
        if vertices.len() < vb || triangles.len() < tb {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(0)?;
        work.add(source.len() + vb + tb)?;
        let s = stream(source, vertex_count, triangle_count)?;
        let (v, t) = (&mut vertices[..vb], &mut triangles[..tb]);
        match (vertex_size, triangle_size) {
            (4, 4) => decode_bytes::<4, 4>(&s, triangle_count, v, t),
            (4, _) => decode_bytes::<4, 3>(&s, triangle_count, v, t),
            (_, 4) => decode_bytes::<2, 4>(&s, triangle_count, v, t),
            _ => decode_bytes::<2, 3>(&s, triangle_count, v, t),
        }
    })();
    workspace.finish(&work);
    result
}

/// Decode one meshlet (meshopt_decodeMeshletRaw) into u32 vertex references
/// and packed triangles (a | b << 8 | c << 16), without heap allocation.
/// Exactly the counted elements are written; no SIMD padding is required.
pub fn decode_meshlet_raw_into(
    vertices: &mut [u32],
    vertex_count: usize,
    triangles: &mut [u32],
    triangle_count: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        check_counts(vertex_count, triangle_count)?;
        if vertices.len() < vertex_count || triangles.len() < triangle_count {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(0)?;
        work.add(source.len() + vertex_count * 4 + triangle_count * 4)?;
        let s = stream(source, vertex_count, triangle_count)?;
        let (v, t) = (
            &mut vertices[..vertex_count],
            &mut triangles[..triangle_count],
        );
        decode_core(
            &s,
            triangle_count,
            |i, r| v[i] = r,
            |i, tri| t[i] = tri >> 8,
        )
    })();
    workspace.finish(&work);
    result
}

/// Allocated output of [`decode_meshlet`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DecodedMeshlet {
    /// vertex_count * vertex_size little-endian vertex reference bytes.
    pub vertices: Vec<u8>,
    /// triangle_count * triangle_size triangle bytes.
    pub triangles: Vec<u8>,
}

/// Allocated output of [`decode_meshlet_raw`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RawMeshlet {
    /// Vertex references.
    pub vertices: Vec<u32>,
    /// Packed triangles, a | b << 8 | c << 16.
    pub triangles: Vec<u32>,
}

fn allocate<T: Clone + Default>(len: usize) -> Result<Vec<T>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(len, T::default());
    Ok(v)
}

/// Allocating form of [`decode_meshlet_into`].
#[inline]
pub fn decode_meshlet(
    vertex_count: usize,
    vertex_size: usize,
    triangle_count: usize,
    triangle_size: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<DecodedMeshlet, Error> {
    let mut work = workspace.begin();
    sizes(vertex_size, triangle_size)?;
    check_counts(vertex_count, triangle_count)?;
    let vb = vertex_count * vertex_size;
    let tb = triangle_count * triangle_size;
    workspace.account_codec(vb + tb)?;
    work.add(source.len() + vb + tb)?;
    let s = stream(source, vertex_count, triangle_count)?;
    let mut out = DecodedMeshlet {
        vertices: allocate(vb)?,
        triangles: allocate(tb)?,
    };
    let result = match (vertex_size, triangle_size) {
        (4, 4) => decode_bytes::<4, 4>(&s, triangle_count, &mut out.vertices, &mut out.triangles),
        (4, _) => decode_bytes::<4, 3>(&s, triangle_count, &mut out.vertices, &mut out.triangles),
        (_, 4) => decode_bytes::<2, 4>(&s, triangle_count, &mut out.vertices, &mut out.triangles),
        _ => decode_bytes::<2, 3>(&s, triangle_count, &mut out.vertices, &mut out.triangles),
    };
    workspace.finish(&work);
    result?;
    if out.vertices.capacity() != vb || out.triangles.capacity() != tb {
        workspace.account_codec(out.vertices.capacity() + out.triangles.capacity())?;
    }
    Ok(out)
}

/// Allocating form of [`decode_meshlet_raw_into`].
pub fn decode_meshlet_raw(
    vertex_count: usize,
    triangle_count: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<RawMeshlet, Error> {
    let mut work = workspace.begin();
    check_counts(vertex_count, triangle_count)?;
    let bytes = checked_bytes(vertex_count + triangle_count, 4)?;
    workspace.account_codec(bytes)?;
    work.add(source.len() + bytes)?;
    let s = stream(source, vertex_count, triangle_count)?;
    let mut out = RawMeshlet {
        vertices: allocate(vertex_count)?,
        triangles: allocate(triangle_count)?,
    };
    let result = decode_core(
        &s,
        triangle_count,
        |i, r| out.vertices[i] = r,
        |i, tri| out.triangles[i] = tri >> 8,
    );
    workspace.finish(&work);
    result?;
    workspace.account_codec(checked_bytes(
        out.vertices.capacity() + out.triangles.capacity(),
        4,
    )?)?;
    Ok(out)
}
