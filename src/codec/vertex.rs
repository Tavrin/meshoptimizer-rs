// meshoptimizer 1.3 scalar decoder, MIT, Arseny Kapoulkine.
// Byte-group parsing also reuses the Moss dc4af42a decoder (MIT).
use crate::Error;

#[inline]
fn packed<const BITS: u32>(data: &[u8; 24], out: &mut [u8; 16]) -> usize {
    let mut literal = (BITS * 2) as usize;
    let escape = (1u8 << BITS) - 1;
    for (i, slot) in out.iter_mut().enumerate() {
        let byte = data[i / (8 / BITS as usize)];
        let shift = if BITS == 1 {
            i as u32 % 8
        } else {
            8 - BITS - (i as u32 % (8 / BITS)) * BITS
        };
        let value = (byte >> shift) & escape;
        *slot = if value == escape {
            let v = data[literal];
            literal += 1;
            v
        } else {
            value
        };
    }
    literal
}
#[inline]
fn group(data: &[u8], pos: usize, out: &mut [u8; 16], bits: u32) -> Result<usize, Error> {
    // Upstream's 24-byte lookahead becomes one checked slice per group.
    let source: &[u8; 24] = data
        .get(pos..pos + 24)
        .ok_or(Error::InvalidStream)?
        .try_into()
        .map_err(|_| Error::InvalidStream)?;
    let used = match bits {
        0 => {
            out.fill(0);
            0
        }
        1 => packed::<1>(source, out),
        2 => packed::<2>(source, out),
        4 => packed::<4>(source, out),
        8 => {
            out.copy_from_slice(&source[..16]);
            16
        }
        _ => return Err(Error::InvalidStream),
    };
    Ok(pos + used)
}
fn decode_deltas<const WIDTH: usize, const XOR: bool>(
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
    rot: u32,
) {
    for component in (0..4).step_by(WIDTH) {
        let mut previous = 0u32;
        for j in 0..WIDTH {
            previous |= u32::from(last[component + j]) << (8 * j);
        }
        let c0 = &buffer[component * count..(component + 1) * count];
        let c1 = if WIDTH >= 2 {
            &buffer[(component + 1) * count..(component + 2) * count]
        } else {
            c0
        };
        let c2 = if WIDTH == 4 {
            &buffer[(component + 2) * count..(component + 3) * count]
        } else {
            c0
        };
        let c3 = if WIDTH == 4 {
            &buffer[(component + 3) * count..(component + 4) * count]
        } else {
            c0
        };
        for (i, record) in target.chunks_mut(stride).enumerate() {
            let mut value = u32::from(c0[i]);
            if WIDTH >= 2 {
                value |= u32::from(c1[i]) << 8;
            }
            if WIDTH == 4 {
                value |= u32::from(c2[i]) << 16 | u32::from(c3[i]) << 24;
            }
            value = if XOR {
                value.rotate_left(rot) ^ previous
            } else {
                ((value >> 1) ^ 0u32.wrapping_sub(value & 1)).wrapping_add(previous)
            };
            let record = &mut record[..4];
            for j in 0..WIDTH {
                record[component + j] = (value >> (8 * j)) as u8;
            }
            previous = value;
        }
    }
}
fn bytes(data: &[u8], mut pos: usize, out: &mut [u8], bits: &[u32]) -> Result<usize, Error> {
    let header = pos;
    let headers = (out.len() / 16).div_ceil(4);
    if data.len().saturating_sub(pos) < headers {
        return Err(Error::InvalidStream);
    }
    pos += headers;
    for (i, out) in out.as_chunks_mut::<16>().0.iter_mut().enumerate() {
        if data.len().saturating_sub(pos) < 24 {
            return Err(Error::InvalidStream);
        }
        let selector = (data[header + i / 4] >> (2 * (i % 4))) & 3;
        pos = group(data, pos, out, bits[selector as usize])?;
    }
    Ok(pos)
}
pub(super) fn decode(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), Error> {
    let version = super::decode_vertex_version(data)?;
    let tail = stride + if version == 0 { 0 } else { stride / 4 };
    let padded = tail.max(if version == 0 { 32 } else { 24 });
    if data.len() < 1 + padded {
        return Err(Error::InvalidStream);
    }
    let start = data.len() - tail;
    let channels = &data[start + stride..];
    let mut last = [0u8; 256];
    last[..stride].copy_from_slice(&data[start..start + stride]);
    let block_size = ((8192 / stride) & !15).min(256);
    let mut pos = 1;
    let mut deltas = [0u8; 1024];
    let mut offset = 0;
    while offset < count {
        let block = block_size.min(count - offset);
        let aligned = block.div_ceil(16) * 16;
        let controls = if version == 0 {
            &[][..]
        } else {
            let c = data
                .get(pos..pos + stride / 4)
                .ok_or(Error::InvalidStream)?;
            pos += stride / 4;
            c
        };
        // Each bounded block fits 8 KiB. Write directly into its checked
        // destination slice, avoiding zeroing/copying an extra stack block.
        let block_output = &mut output[offset * stride..(offset + block) * stride];
        for k in (0..stride).step_by(4) {
            let control = if version == 0 { 0 } else { controls[k / 4] };
            for j in 0..4 {
                let ctrl = (control >> (j * 2)) & 3;
                let out = &mut deltas[j * block..j * block + aligned];
                match ctrl {
                    3 => {
                        out[..block].copy_from_slice(
                            data.get(pos..pos + block).ok_or(Error::InvalidStream)?,
                        );
                        pos += block;
                    }
                    2 => out[..block].fill(0),
                    _ => {
                        pos = bytes(
                            data,
                            pos,
                            out,
                            if version == 0 {
                                &[0, 2, 4, 8]
                            } else if ctrl == 0 {
                                &[0, 1, 2, 4]
                            } else {
                                &[1, 2, 4, 8]
                            },
                        )?
                    }
                }
            }
            let channel = if version == 0 { 0 } else { channels[k / 4] };
            let target = &mut block_output[k..];
            // k is a four-byte-aligned channel group; only complete records
            // belong to this group. Extend the target to keep chunk strides
            // complete without touching bytes beyond this group's four bytes.
            let target_len = (block - 1) * stride + 4;
            let target = &mut target[..target_len];
            match channel & 3 {
                0 => decode_deltas::<1, false>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                1 => decode_deltas::<2, false>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                2 => decode_deltas::<4, true>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    (32 - u32::from(channel >> 4)) & 31,
                ),
                _ => return Err(Error::InvalidStream),
            }
        }
        last[..stride].copy_from_slice(&block_output[(block - 1) * stride..]);
        offset += block;
    }
    if data.len().checked_sub(pos) != Some(padded) {
        return Err(Error::InvalidStream);
    }
    Ok(())
}
