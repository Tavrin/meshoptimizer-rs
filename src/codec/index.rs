// Adapted from Moss dc4af42a; MIT, Moss contributors and Arseny Kapoulkine.
use super::byte_at;
use crate::Error;
const INDEX_HEADER: u8 = 0xe0;
const SEQUENCE_HEADER: u8 = 0xd0;
#[inline]
fn decode_vbyte(data: &[u8], position: &mut usize) -> Result<u32, Error> {
    // The four-byte sequence tail and sixteen-byte triangle tail make a
    // five-byte checked lookahead equivalent to the reference's cursor guard.
    let bytes: &[u8; 5] = data
        .get(*position..*position + 5)
        .ok_or(Error::InvalidStream)?
        .try_into()
        .map_err(|_| Error::InvalidStream)?;
    *position += 1;
    let lead = bytes[0];
    if lead < 128 {
        return Ok(u32::from(lead));
    }
    let mut result = u32::from(lead & 127);
    for (i, &group) in bytes[1..].iter().enumerate() {
        *position += 1;
        result |= u32::from(group & 127) << ((i + 1) * 7);
        if group < 128 {
            break;
        }
    }
    Ok(result)
}

fn decode_index(data: &[u8], position: &mut usize, last: u32) -> Result<u32, Error> {
    let value = decode_vbyte(data, position)?;
    let delta = (value >> 1) ^ 0u32.wrapping_sub(value & 1);
    Ok(last.wrapping_add(delta))
}

const CODE_AUX_TABLE_SIZE: usize = 16;

fn triangles_width<const STRIDE: usize, const TRI_BYTES: usize>(
    output: &mut [u8],
    count: usize,
    data: &[u8],
) -> Result<(), Error> {
    if !count.is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    if data.len() < 1 + count / 3 + CODE_AUX_TABLE_SIZE {
        return Err(Error::InvalidStream);
    }
    if data[0] & 0xf0 != INDEX_HEADER {
        return Err(Error::InvalidStream);
    }
    let version = data[0] & 0x0f;
    if version > 1 {
        return Err(Error::UnsupportedVersion);
    }
    let mut edge_fifo = [[u32::MAX; 2]; 16];
    let mut vertex_fifo = [u32::MAX; 16];
    let mut edge_offset = 0usize;
    let mut vertex_offset = 0usize;
    let push_edge = |fifo: &mut [[u32; 2]; 16], offset: &mut usize, a: u32, b: u32| {
        fifo[*offset] = [a, b];
        *offset = (*offset + 1) & 15;
    };
    let push_vertex = |fifo: &mut [u32; 16], offset: &mut usize, v: u32, advance: bool| {
        fifo[*offset] = v;
        *offset = (*offset + usize::from(advance)) & 15;
    };
    let mut next = 0u32;
    let mut last = 0u32;
    let fec_max = if version >= 1 { 13 } else { 15 };
    let mut position = 1 + count / 3;
    let safe_end = data.len() - CODE_AUX_TABLE_SIZE;
    let code_aux_table = &data[safe_end..];
    // Public entry points supply exactly count * STRIDE bytes. Checked
    // code and output records make all three stores bounded by one array.
    let codes = &data[1..position];
    for (&code_tri, triangle) in codes.iter().zip(output.as_chunks_mut::<TRI_BYTES>().0) {
        if position > safe_end {
            return Err(Error::InvalidStream);
        }
        let (a, b, c);
        if code_tri < 0xf0 {
            let fe = usize::from(code_tri >> 4);
            let edge = edge_fifo[(edge_offset.wrapping_sub(1 + fe)) & 15];
            a = edge[0];
            b = edge[1];
            let fec = usize::from(code_tri & 15);
            if fec < fec_max {
                let cached = vertex_fifo[(vertex_offset.wrapping_sub(1 + fec)) & 15];
                c = if fec == 0 { next } else { cached };
                if fec == 0 {
                    next = next.wrapping_add(1);
                }
                push_vertex(&mut vertex_fifo, &mut vertex_offset, c, fec == 0);
            } else {
                c = if fec != 15 {
                    // 13 and 14 encode -1 and +1 relative to the last free index.
                    let delta = fec as i32 - (fec ^ 3) as i32;
                    last.wrapping_add_signed(delta)
                } else {
                    decode_index(data, &mut position, last)?
                };
                last = c;
                push_vertex(&mut vertex_fifo, &mut vertex_offset, c, true);
            }
            push_edge(&mut edge_fifo, &mut edge_offset, c, b);
            push_edge(&mut edge_fifo, &mut edge_offset, a, c);
        } else if code_tri < 0xfe {
            let code_aux = code_aux_table[usize::from(code_tri & 15)];
            let feb = usize::from(code_aux >> 4);
            let fec = usize::from(code_aux & 15);
            a = next;
            next = next.wrapping_add(1);
            let cached_b = vertex_fifo[(vertex_offset.wrapping_sub(feb)) & 15];
            b = if feb == 0 { next } else { cached_b };
            if feb == 0 {
                next = next.wrapping_add(1);
            }
            let cached_c = vertex_fifo[(vertex_offset.wrapping_sub(fec)) & 15];
            c = if fec == 0 { next } else { cached_c };
            if fec == 0 {
                next = next.wrapping_add(1);
            }
            push_vertex(&mut vertex_fifo, &mut vertex_offset, a, true);
            push_vertex(&mut vertex_fifo, &mut vertex_offset, b, feb == 0);
            push_vertex(&mut vertex_fifo, &mut vertex_offset, c, fec == 0);
            push_edge(&mut edge_fifo, &mut edge_offset, b, a);
            push_edge(&mut edge_fifo, &mut edge_offset, c, b);
            push_edge(&mut edge_fifo, &mut edge_offset, a, c);
        } else {
            let code_aux = byte_at(data, position)?;
            position += 1;
            let fea = if code_tri == 0xfe { 0 } else { 15 };
            let feb = usize::from(code_aux >> 4);
            let fec = usize::from(code_aux & 15);
            if code_aux == 0 {
                next = 0;
            }
            let mut take_next = || {
                let value = next;
                next = next.wrapping_add(1);
                value
            };
            let mut first = if fea == 0 { take_next() } else { 0 };
            let mut second = if feb == 0 {
                take_next()
            } else {
                vertex_fifo[(vertex_offset.wrapping_sub(feb)) & 15]
            };
            let mut third = if fec == 0 {
                take_next()
            } else {
                vertex_fifo[(vertex_offset.wrapping_sub(fec)) & 15]
            };
            if fea == 15 {
                first = decode_index(data, &mut position, last)?;
                last = first;
            }
            if feb == 15 {
                second = decode_index(data, &mut position, last)?;
                last = second;
            }
            if fec == 15 {
                third = decode_index(data, &mut position, last)?;
                last = third;
            }
            a = first;
            b = second;
            c = third;
            push_vertex(&mut vertex_fifo, &mut vertex_offset, a, true);
            push_vertex(
                &mut vertex_fifo,
                &mut vertex_offset,
                b,
                feb == 0 || feb == 15,
            );
            push_vertex(
                &mut vertex_fifo,
                &mut vertex_offset,
                c,
                fec == 0 || fec == 15,
            );
            push_edge(&mut edge_fifo, &mut edge_offset, b, a);
            push_edge(&mut edge_fifo, &mut edge_offset, c, b);
            push_edge(&mut edge_fifo, &mut edge_offset, a, c);
        }
        triangle[..STRIDE].copy_from_slice(&a.to_le_bytes()[..STRIDE]);
        triangle[STRIDE..STRIDE * 2].copy_from_slice(&b.to_le_bytes()[..STRIDE]);
        triangle[STRIDE * 2..].copy_from_slice(&c.to_le_bytes()[..STRIDE]);
    }
    if position != safe_end {
        return Err(Error::InvalidStream);
    }
    Ok(())
}

fn sequence_width<const STRIDE: usize>(
    output: &mut [u8],
    count: usize,
    data: &[u8],
) -> Result<(), Error> {
    if data.len() < 1 + count + 4 {
        return Err(Error::InvalidStream);
    }
    if data[0] & 0xf0 != SEQUENCE_HEADER {
        return Err(Error::InvalidStream);
    }
    let version = data[0] & 0x0f;
    if version > 1 {
        return Err(Error::UnsupportedVersion);
    }
    let safe_end = data.len() - 4;
    let mut position = 1usize;
    let mut last = [0u32; 2];
    for output in output.as_chunks_mut::<STRIDE>().0 {
        let mut value = decode_vbyte(data, &mut position)?;
        let baseline = (value & 1) as usize;
        value >>= 1;
        let delta = (value >> 1) ^ 0u32.wrapping_sub(value & 1);
        let index = last[baseline].wrapping_add(delta);
        last[baseline] = index;
        output.copy_from_slice(&index.to_le_bytes()[..STRIDE]);
    }
    if position != safe_end {
        return Err(Error::InvalidStream);
    }
    Ok(())
}

pub(super) fn triangles(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), Error> {
    if stride == 2 {
        triangles_width::<2, 6>(output, count, data)
    } else {
        triangles_width::<4, 12>(output, count, data)
    }
}
pub(super) fn sequence(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), Error> {
    if stride == 2 {
        sequence_width::<2>(output, count, data)
    } else {
        sequence_width::<4>(output, count, data)
    }
}
