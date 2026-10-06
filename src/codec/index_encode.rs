// meshoptimizer 1.3 index buffer and sequence encoders (indexcodec.cpp),
// MIT, Arseny Kapoulkine. Capacity failures match the reference's zero returns.
use crate::Error;

const INDEX_HEADER: u8 = 0xe0;
const SEQUENCE_HEADER: u8 = 0xd0;
const CODE_AUX: [u8; 16] = [
    0x00, 0x76, 0x87, 0x56, 0x67, 0x78, 0xa9, 0x86, 0x65, 0x89, 0x68, 0x98, 0x01, 0x69, 0, 0,
];
/// Corners starting at `r` (upstream's rotations table {0, 1, 2, 0, 1}).
#[inline(always)]
pub(super) fn rotate(t: &[u32; 3], r: usize) -> (u32, u32, u32) {
    match r {
        0 => (t[0], t[1], t[2]),
        1 => (t[1], t[2], t[0]),
        _ => (t[2], t[0], t[1]),
    }
}

/// Upstream's loop: the smallest bits >= 1 with vertex_count <= 2^bits,
/// capped at 32, computed from the bit length of vertex_count - 1.
#[inline]
fn vertex_bits(vertex_count: u64) -> u32 {
    (64 - vertex_count.saturating_sub(1).leading_zeros()).clamp(1, 32)
}

/// meshopt_encodeIndexBufferBound for a 64-bit vertex count.
#[inline]
pub(super) fn index_bound(index_count: usize, vertex_count: u64) -> Result<usize, Error> {
    // Worst case: two header bytes plus three varint-7 deltas of bits + 1.
    let groups = (vertex_bits(vertex_count) + 1).div_ceil(7);
    (index_count / 3)
        .checked_mul(2 + 3 * groups as usize)
        .and_then(|n| n.checked_add(17))
        .ok_or(Error::SizeOverflow)
}

/// meshopt_encodeIndexSequenceBound for a 64-bit vertex count.
#[inline]
pub(super) fn sequence_bound(index_count: usize, vertex_count: u64) -> Result<usize, Error> {
    // One varint-7 delta of bits + 1, plus the baseline bit.
    let groups = (vertex_bits(vertex_count) + 2).div_ceil(7);
    index_count
        .checked_mul(groups as usize)
        .and_then(|n| n.checked_add(5))
        .ok_or(Error::SizeOverflow)
}

/// Varint-7 into a five-byte window; returns the bytes used.
#[inline(always)]
fn sequence_vbyte5(out: &mut [u8; 5], mut v: u32) -> usize {
    // Constant byte offsets let each terminating branch eliminate the
    // dynamic window index and loop counter. A u32 needs at most five bytes.
    macro_rules! emit {
        ($i:literal) => {
            if v < 128 {
                out[$i] = v as u8;
                return $i + 1;
            }
            out[$i] = v as u8 | 128;
            v >>= 7;
        };
    }
    emit!(0);
    emit!(1);
    emit!(2);
    emit!(3);
    out[4] = v as u8;
    5
}

#[inline(always)]
fn vbyte5(out: &mut [u8; 5], mut v: u32) -> usize {
    let mut n = 0;
    loop {
        out[n] = (v & 127) as u8 | if v > 127 { 128 } else { 0 };
        n += 1;
        v >>= 7;
        if v == 0 || n == 5 {
            return n;
        }
    }
}

/// Zigzag delta varint. Callers guarantee sixteen bytes per triangle after
/// the reference's slack check, so the five-byte window is always in range.
#[inline]
fn encode_index(out: &mut [u8], pos: &mut usize, index: u32, last: u32) {
    let d = index.wrapping_sub(last);
    if let Ok(window) = <&mut [u8; 5]>::try_from(&mut out[*pos..*pos + 5]) {
        *pos += vbyte5(window, (d << 1) ^ ((d as i32) >> 31) as u32);
    }
}

#[inline]
fn edge_pair(a: u32, b: u32) -> u64 {
    u64::from(a) | (u64::from(b) << 32)
}

#[inline]
fn edge_fifo(fifo: &[u64; 16], a: u32, b: u32, c: u32, offset: usize) -> Option<usize> {
    let (ab, bc, ca) = (edge_pair(a, b), edge_pair(b, c), edge_pair(c, a));
    for i in 0..16 {
        let edge = fifo[offset.wrapping_sub(1 + i) & 15];
        if edge == ab {
            return Some(i << 2);
        }
        if edge == bc {
            return Some((i << 2) | 1);
        }
        if edge == ca {
            return Some((i << 2) | 2);
        }
    }
    None
}

#[inline]
fn vertex_fifo(fifo: &[u32; 16], v: u32, offset: usize) -> Option<usize> {
    (0..16).find(|&i| fifo[offset.wrapping_sub(1 + i) & 15] == v)
}

struct Fifos {
    edges: [u64; 16],
    vertices: [u32; 16],
    edge_offset: usize,
    vertex_offset: usize,
}
impl Fifos {
    #[inline]
    fn push_edge(&mut self, a: u32, b: u32) {
        self.edges[self.edge_offset] = edge_pair(a, b);
        self.edge_offset = (self.edge_offset + 1) & 15;
    }
    #[inline]
    fn push_vertex(&mut self, v: u32) {
        self.vertices[self.vertex_offset] = v;
        self.vertex_offset = (self.vertex_offset + 1) & 15;
    }
}

/// meshopt_encodeIndexBuffer with an explicit version. indices.len() is a
/// multiple of three (checked by the caller).
pub(super) fn encode_index_buffer(
    out: &mut [u8],
    indices: &[u32],
    version: u8,
) -> Result<usize, Error> {
    let triangles = indices.len() / 3;
    // Header, one code byte per triangle and the sixteen-byte code table.
    if out.len() < 1 + triangles + 16 {
        return Err(Error::BufferTooSmall);
    }
    out[0] = INDEX_HEADER | version;
    let mut f = Fifos {
        edges: [u64::MAX; 16],
        vertices: [u32::MAX; 16],
        edge_offset: 0,
        vertex_offset: 0,
    };
    let mut next = 0u32;
    let mut last = 0u32;
    let safe_end = out.len() - 16;
    let (head, data_region) = out.split_at_mut(1 + triangles);
    let codes = &mut head[1..];
    let mut data = 0usize;
    // Offsets in data_region are relative to 1 + triangles.
    let safe = safe_end - (1 + triangles);
    let fec_max = if version >= 1 { 13 } else { 15 };
    for (tri, code) in indices.as_chunks::<3>().0.iter().zip(codes.iter_mut()) {
        // Each triangle writes at most sixteen bytes after this check.
        if data > safe {
            return Err(Error::BufferTooSmall);
        }
        let fer = edge_fifo(&f.edges, tri[0], tri[1], tri[2], f.edge_offset);
        match fer {
            Some(fer) if (fer >> 2) < 15 => {
                let (a, b, c) = rotate(tri, fer & 3);
                let fe = fer >> 2;
                let fc = vertex_fifo(&f.vertices, c, f.vertex_offset);
                let mut fec = match fc {
                    Some(fc) if fc >= 1 && fc < fec_max => fc,
                    _ if c == next => {
                        next = next.wrapping_add(1);
                        0
                    }
                    _ => 15,
                };
                if fec == 15 && version >= 1 {
                    // Encode last-1 and last+1 for strip-like sequences.
                    if c.wrapping_add(1) == last {
                        fec = 13;
                        last = c;
                    }
                    if c == last.wrapping_add(1) {
                        fec = 14;
                        last = c;
                    }
                }
                *code = ((fe << 4) | fec) as u8;
                if fec == 15 {
                    encode_index(data_region, &mut data, c, last);
                    last = c;
                }
                if fec == 0 || fec >= fec_max {
                    f.push_vertex(c);
                }
                f.push_edge(c, b);
                f.push_edge(a, c);
            }
            _ => {
                let rotation = if tri[1] == next {
                    1
                } else if tri[2] == next {
                    2
                } else {
                    0
                };
                let (a, b, c) = rotate(tri, rotation);
                let mut reset = false;
                if a == 0 && b == 1 && c == 2 && next > 0 && version >= 1 {
                    reset = true;
                    next = 0;
                    // Never reference vertices from before the restart.
                    f.vertices = [u32::MAX; 16];
                }
                let fb = vertex_fifo(&f.vertices, b, f.vertex_offset);
                let fc = vertex_fifo(&f.vertices, c, f.vertex_offset);
                // a is almost always next after rotation; it uses no FIFO slot.
                let fea = if a == next {
                    next = next.wrapping_add(1);
                    0
                } else {
                    15
                };
                let mut take = |v: u32, cached: Option<usize>| match cached {
                    Some(i) if i < 14 => i + 1,
                    _ if v == next => {
                        next = next.wrapping_add(1);
                        0
                    }
                    _ => 15,
                };
                let feb = take(b, fb);
                let fec = take(c, fc);
                let aux = ((feb << 4) | fec) as u8;
                let aux_index = CODE_AUX.iter().position(|&t| t == aux);
                match aux_index {
                    Some(i) if fea == 0 && i < 14 && !reset => *code = 0xf0 | i as u8,
                    _ => {
                        *code = 0xf0 | 14 | fea as u8;
                        data_region[data] = aux;
                        data += 1;
                    }
                }
                if fea == 15 {
                    encode_index(data_region, &mut data, a, last);
                    last = a;
                }
                if feb == 15 {
                    encode_index(data_region, &mut data, b, last);
                    last = b;
                }
                if fec == 15 {
                    encode_index(data_region, &mut data, c, last);
                    last = c;
                }
                if fea == 0 || fea == 15 {
                    f.push_vertex(a);
                }
                if feb == 0 || feb == 15 {
                    f.push_vertex(b);
                }
                if fec == 0 || fec == 15 {
                    f.push_vertex(c);
                }
                f.push_edge(b, a);
                f.push_edge(c, b);
                f.push_edge(a, c);
            }
        }
    }
    if data > safe {
        return Err(Error::BufferTooSmall);
    }
    data_region[data..data + 16].copy_from_slice(&CODE_AUX);
    Ok(1 + triangles + data + 16)
}

/// meshopt_encodeIndexSequence with an explicit version.
pub(super) fn encode_index_sequence(
    out: &mut [u8],
    indices: &[u32],
    version: u8,
) -> Result<usize, Error> {
    // Header, one byte per index and a four-byte tail.
    if out.len() < 1 + indices.len() + 4 {
        return Err(Error::BufferTooSmall);
    }
    out[0] = SEQUENCE_HEADER | version;
    let mut last = [0u32; 2];
    let mut current = 0usize;
    let mut pos = 1usize;
    let safe_end = out.len() - 4;
    for &index in indices {
        // Each index writes at most five bytes before the four-byte tail.
        if pos >= safe_end {
            return Err(Error::BufferTooSmall);
        }
        // Switch baselines when the delta would not fit in one byte. C++
        // negation of i32::MIN is undefined; the pinned GCC builds (-O0 and
        // -O3) wrap it to a negative value and do not switch, which is kept.
        let cd = index.wrapping_sub(last[current & 1]) as i32;
        current ^= usize::from(cd.wrapping_abs() >= 30);
        let d = index.wrapping_sub(last[current & 1]);
        let v = (d << 1) ^ ((d as i32) >> 31) as u32;
        // The low bit selects the baseline used for reconstruction. The check
        // above leaves at least five bytes: one bounded window per index.
        let window: &mut [u8; 5] = (&mut out[pos..pos + 5])
            .try_into()
            .map_err(|_| Error::BufferTooSmall)?;
        pos += sequence_vbyte5(window, (v << 1) | current as u32);
        last[current & 1] = index;
    }
    if pos > safe_end {
        return Err(Error::BufferTooSmall);
    }
    out[pos..pos + 4].fill(0);
    Ok(pos + 4)
}
