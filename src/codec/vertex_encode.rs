// meshoptimizer 1.3 scalar vertex encoder (vertexcodec.cpp), MIT, Arseny Kapoulkine.
//
// Every capacity check follows the reference in the same order, so a caller
// buffer is rejected exactly where meshopt_encodeVertexBufferLevel returns 0.
// Deltas for the four bytes of one channel group are computed together; the
// reference computes the same bytes one component at a time, in the same order.
use crate::Error;

const GROUP: usize = 16;
const DECODE_LIMIT: usize = 24;
const BLOCK_MAX: usize = 256;
const HEADER: u8 = 0xa0;
const TAIL_V0: usize = 32;
const TAIL_V1: usize = 24;

#[inline]
pub(super) fn block_size(stride: usize) -> usize {
    ((8192 / stride) & !(GROUP - 1)).min(BLOCK_MAX)
}

/// Upstream meshopt_encodeVertexBufferBound, with checked arithmetic.
#[inline]
pub(super) fn bound(count: usize, stride: usize) -> Result<usize, Error> {
    let block = block_size(stride);
    let blocks = count / block + usize::from(!count.is_multiple_of(block));
    let header = (block / GROUP).div_ceil(4);
    let tail = (stride + stride / 4).max(TAIL_V0.max(TAIL_V1));
    // stride <= 256, so the per-block size cannot overflow.
    let per = stride * (header + block) + stride / 4;
    blocks
        .checked_mul(per)
        .and_then(|n| n.checked_add(1 + tail))
        .ok_or(Error::SizeOverflow)
}

struct Writer<'a> {
    out: &'a mut [u8],
    pos: usize,
}
impl Writer<'_> {
    #[inline]
    fn remaining(&self) -> usize {
        self.out.len() - self.pos
    }
}

/// Encoded group sizes for every bit width, from one pass over the group.
#[derive(Clone, Copy, Default)]
struct Sizes {
    zero: bool,
    ge1: u8,
    ge3: u8,
    ge15: u8,
}
impl Sizes {
    #[inline]
    fn new(group: &[u8; GROUP]) -> Self {
        // Byte counters (at most 16) keep the comparisons in byte lanes.
        let (mut ge1, mut ge3, mut ge15) = (0u8, 0u8, 0u8);
        for &b in group {
            ge1 += u8::from(b >= 1);
            ge3 += u8::from(b >= 3);
            ge15 += u8::from(b >= 15);
        }
        Self {
            zero: ge1 == 0,
            ge1,
            ge3,
            ge15,
        }
    }
    // encodeBytesGroupMeasure; size_t(-1) marks an impossible zero group.
    #[inline]
    fn of(self, bits: u32) -> usize {
        match bits {
            0 => {
                if self.zero {
                    0
                } else {
                    usize::MAX
                }
            }
            1 => 2 + usize::from(self.ge1),
            2 => 4 + usize::from(self.ge3),
            4 => 8 + usize::from(self.ge15),
            _ => GROUP,
        }
    }
}

/// Smallest of the 1/2/4/8-bit group sizes (estimateChannel's measure),
/// computed in byte arithmetic: every candidate is at most 24.
#[inline(always)]
fn best_size_1248(group: &[u8; GROUP]) -> u8 {
    let (mut ge1, mut ge3, mut ge15) = (0u8, 0u8, 0u8);
    for &b in group {
        ge1 += u8::from(b >= 1);
        ge3 += u8::from(b >= 3);
        ge15 += u8::from(b >= 15);
    }
    (2 + ge1).min(4 + ge3).min(8 + ge15).min(GROUP as u8)
}

#[inline]
fn encode_group(dst: &mut [u8; DECODE_LIMIT], group: &[u8; GROUP], bits: u32) -> usize {
    match bits {
        0 => 0,
        1 => packed_group::<1>(dst, group),
        2 => packed_group::<2>(dst, group),
        4 => packed_group::<4>(dst, group),
        _ => {
            dst[..GROUP].copy_from_slice(group);
            GROUP
        }
    }
}

#[inline(always)]
fn packed_group<const BITS: u32>(dst: &mut [u8; DECODE_LIMIT], group: &[u8; GROUP]) -> usize {
    let per = (8 / BITS) as usize;
    let sentinel = (1u8 << BITS) - 1;
    let packed = GROUP / per;
    for (i, slot) in dst[..packed].iter_mut().enumerate() {
        let mut byte = 0u8;
        for &v in &group[i * per..(i + 1) * per] {
            byte = (byte << BITS) | v.min(sentinel);
        }
        // One-bit groups are stored in reverse bit order.
        *slot = if BITS == 1 { byte.reverse_bits() } else { byte };
    }
    let mut n = packed;
    for &v in group {
        // Branchless append of out-of-range values, as upstream: n <= 23
        // here, and a byte written past the final literal is overwritten by
        // later output (every stream ends with its tail).
        dst[n] = v;
        n += usize::from(v >= sentinel);
    }
    n
}

fn encode_bytes(
    w: &mut Writer<'_>,
    buffer: &[u8],
    measured: &[Sizes],
    bits: &[u32],
) -> Result<(), Error> {
    let header_size = (buffer.len() / GROUP).div_ceil(4);
    if w.remaining() < header_size {
        return Err(Error::BufferTooSmall);
    }
    let header = w.pos;
    w.out[header..header + header_size].fill(0);
    w.pos += header_size;
    let mut last_bits = u32::MAX;
    for (i, (group, &sizes)) in buffer
        .as_chunks::<GROUP>()
        .0
        .iter()
        .zip(measured)
        .enumerate()
    {
        if w.remaining() < DECODE_LIMIT {
            return Err(Error::BufferTooSmall);
        }
        let mut best_k = 3;
        let mut best_size = sizes.of(bits[3]);
        for (k, &b) in bits[..3].iter().enumerate() {
            let size = sizes.of(b);
            // Favor consistent selection across groups, but never replace literals.
            if size < best_size || (size == best_size && b == last_bits && bits[best_k] != 8) {
                best_k = k;
                best_size = size;
            }
        }
        w.out[header + i / 4] |= (best_k as u8) << ((i % 4) * 2);
        let dst: &mut [u8; DECODE_LIMIT] = (&mut w.out[w.pos..w.pos + DECODE_LIMIT])
            .try_into()
            .map_err(|_| Error::BufferTooSmall)?;
        w.pos += encode_group(dst, group, bits[best_k]);
        last_bits = bits[best_k];
    }
    Ok(())
}

#[inline]
fn word(data: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(
        *data[at..]
            .first_chunk::<4>()
            .expect("validated vertex word"),
    )
}

/// encodeDeltas for the four byte lanes k4..k4+4, which share one channel.
fn deltas(
    lanes: &mut [[u8; BLOCK_MAX]; 4],
    data: &[u8],
    count: usize,
    stride: usize,
    last: &[u8],
    k4: usize,
    channel: u8,
) {
    match stride {
        4 => deltas_fixed::<4>(lanes, data, count, stride, last, k4, channel),
        12 => deltas_fixed::<12>(lanes, data, count, stride, last, k4, channel),
        32 => deltas_fixed::<32>(lanes, data, count, stride, last, k4, channel),
        _ => deltas_fixed::<0>(lanes, data, count, stride, last, k4, channel),
    }
}

fn deltas_fixed<const STRIDE: usize>(
    lanes: &mut [[u8; BLOCK_MAX]; 4],
    data: &[u8],
    count: usize,
    stride: usize,
    last: &[u8],
    k4: usize,
    channel: u8,
) {
    let stride = if STRIDE == 0 { stride } else { STRIDE };
    let k4 = if STRIDE == 4 { 0 } else { k4 };
    let mut p = word(last, k4);
    let [l0, l1, l2, l3] = lanes;
    let rows = data.chunks_exact(stride).take(count);
    let lanes = l0[..count]
        .iter_mut()
        .zip(&mut l1[..count])
        .zip(&mut l2[..count])
        .zip(&mut l3[..count]);
    match channel & 3 {
        0 => {
            for (row, (((a, b), c), d)) in rows.zip(lanes) {
                let v = word(row, k4);
                let x = v.to_le_bytes();
                let y = p.to_le_bytes();
                let z = |j: usize| {
                    let t = x[j].wrapping_sub(y[j]);
                    (t << 1) ^ 0u8.wrapping_sub(t >> 7)
                };
                (*a, *b, *c, *d) = (z(0), z(1), z(2), z(3));
                p = v;
            }
        }
        1 => {
            for (row, (((a, b), c), d)) in rows.zip(lanes) {
                let v = word(row, k4);
                let z = |shift: u32| {
                    let t = ((v >> shift) as u16).wrapping_sub((p >> shift) as u16);
                    (t << 1) ^ 0u16.wrapping_sub(t >> 15)
                };
                let (lo, hi) = (z(0), z(16));
                (*a, *b, *c, *d) = (lo as u8, (lo >> 8) as u8, hi as u8, (hi >> 8) as u8);
                p = v;
            }
        }
        _ => {
            // Channel values are produced internally: 0, 1 or 2 | rotation << 4.
            let rot = u32::from(channel >> 4);
            for (row, (((a, b), c), d)) in rows.zip(lanes) {
                let v = word(row, k4);
                let [x0, x1, x2, x3] = (v ^ p).rotate_left(rot).to_le_bytes();
                (*a, *b, *c, *d) = (x0, x1, x2, x3);
                p = v;
            }
        }
    }
}

fn estimate_bits(v: u8) -> usize {
    if v <= 15 {
        if v <= 3 {
            if v == 0 {
                0
            } else {
                2
            }
        } else {
            4
        }
    } else {
        8
    }
}

fn estimate_rotate(vertices: &[u8], count: usize, stride: usize, k: usize) -> u8 {
    pick_rotation(&rotate_sizes(
        vertices,
        count,
        stride,
        k,
        0..count.div_ceil(GROUP),
    ))
}

/// Encoded-size estimate of each rotation over the sixteen-vertex groups in
/// `groups`. Group sums are integers, so ranges may be summed in any order.
fn rotate_sizes(
    vertices: &[u8],
    count: usize,
    stride: usize,
    k: usize,
    groups: core::ops::Range<usize>,
) -> [usize; 8] {
    let mut sizes = [0usize; 8];
    // The first delta of the buffer is zero; later ranges start from the
    // vertex before their first group.
    let before = (groups.start * GROUP).saturating_sub(1);
    let mut last = word(&vertices[before * stride..], k);
    let end = (groups.end * GROUP).min(count);
    for group in vertices[groups.start * GROUP * stride..end * stride].chunks(stride * GROUP) {
        let mut bitg = 0u32;
        for row in group.chunks_exact(stride) {
            let v = word(row, k);
            bitg |= v ^ last;
            last = v;
        }
        for (j, size) in sizes.iter_mut().enumerate() {
            let [a, b, c, d] = bitg.rotate_left(j as u32).to_le_bytes();
            *size += estimate_bits(a) + estimate_bits(b) + estimate_bits(c) + estimate_bits(d);
        }
    }
    sizes
}

fn pick_rotation(sizes: &[usize; 8]) -> u8 {
    let mut best = 0;
    for rot in 1..8 {
        if sizes[rot] < sizes[best] {
            best = rot;
        }
    }
    best as u8
}

#[allow(clippy::too_many_arguments)]
fn estimate_channel(
    lanes: &mut [[u8; BLOCK_MAX]; 4],
    vertices: &[u8],
    count: usize,
    stride: usize,
    k: usize,
    block_size: usize,
    skip: usize,
    max_channel: u8,
    rot: u8,
) -> u8 {
    let sizes = channel_sizes(
        lanes,
        vertices,
        count,
        stride,
        k,
        block_size,
        skip,
        max_channel,
        rot,
        0..sample_count(count, block_size, skip),
    );
    pick_channel(&sizes, max_channel, rot)
}

/// Number of sampled blocks: every `skip`-th block, starting with the first.
fn sample_count(count: usize, block_size: usize, skip: usize) -> usize {
    count.div_ceil(block_size * skip)
}

/// Estimated encoded size of each delta channel over the sampled blocks
/// numbered `samples`. Block sums are integers, so ranges may be summed in
/// any order.
#[allow(clippy::too_many_arguments)]
fn channel_sizes(
    lanes: &mut [[u8; BLOCK_MAX]; 4],
    vertices: &[u8],
    count: usize,
    stride: usize,
    k: usize,
    block_size: usize,
    skip: usize,
    max_channel: u8,
    rot: u8,
    samples: core::ops::Range<usize>,
) -> [usize; 3] {
    let mut sizes = [0usize; 3];
    for sample in samples {
        let i = sample * block_size * skip;
        let block = block_size.min(count - i);
        let aligned = block.div_ceil(GROUP) * GROUP;
        let start = if i == 0 { 0 } else { i - 1 };
        // Only bytes k..k+4 of the previous vertex are read.
        let last = &vertices[start * stride..(start + 1) * stride];
        for lane in lanes.iter_mut() {
            lane[block..aligned].fill(0);
        }
        let data = &vertices[i * stride..(i + block) * stride];
        for channel in 0..max_channel {
            deltas(lanes, data, block, stride, last, k, channel | (rot << 4));
            for lane in lanes.iter() {
                let mut total = 0u32;
                for group in lane[..aligned].as_chunks::<GROUP>().0 {
                    total += u32::from(best_size_1248(group));
                }
                sizes[channel as usize] += total as usize;
            }
        }
    }
    sizes
}

fn pick_channel(sizes: &[usize; 3], max_channel: u8, rot: u8) -> u8 {
    let mut best = 0;
    for channel in 1..max_channel as usize {
        if sizes[channel] < sizes[best] {
            best = channel;
        }
    }
    if best == 2 {
        2 | (rot << 4)
    } else {
        best as u8
    }
}

fn estimate_control(measured: &[Sizes], count: usize, level: u8) -> u8 {
    if measured.iter().all(|s| s.zero) {
        return 2;
    }
    if level == 0 {
        return 1;
    }
    let header = measured.len().div_ceil(4);
    let (mut est0, mut est1) = (header, header);
    for &s in measured {
        let s124 = s.of(1).min(s.of(2)).min(s.of(4));
        est0 += s124.min(s.of(0));
        est1 += s124.min(s.of(8));
    }
    if est0 < count || est1 < count {
        if est0 < est1 {
            0
        } else {
            1
        }
    } else {
        3
    }
}

#[allow(clippy::too_many_arguments)]
fn encode_block(
    w: &mut Writer<'_>,
    lanes: &mut [[u8; BLOCK_MAX]; 4],
    data: &[u8],
    count: usize,
    stride: usize,
    last: &mut [u8; 256],
    channels: &[u8; 64],
    version: u8,
    level: u8,
) -> Result<(), Error> {
    let aligned = count.div_ceil(GROUP) * GROUP;
    // The reference zeroes its block buffer; only the first count bytes of a
    // component are written, so the aligned tail of every component encodes as zero.
    for lane in lanes.iter_mut() {
        lane[count..aligned].fill(0);
    }
    let control_size = if version == 0 { 0 } else { stride / 4 };
    if w.remaining() < control_size {
        return Err(Error::BufferTooSmall);
    }
    let control = w.pos;
    w.out[control..control + control_size].fill(0);
    w.pos += control_size;
    for k4 in (0..stride).step_by(4) {
        let channel = if version == 0 { 0 } else { channels[k4 / 4] };
        deltas(lanes, data, count, stride, &last[..], k4, channel);
        for (j, lane) in lanes.iter().enumerate() {
            let buffer = &lane[..aligned];
            // Measure each group once for control selection and encoding.
            let mut measured = [Sizes::default(); BLOCK_MAX / GROUP];
            let measured = &mut measured[..aligned / GROUP];
            for (s, group) in measured.iter_mut().zip(buffer.as_chunks::<GROUP>().0) {
                *s = Sizes::new(group);
            }
            let mut ctrl = 0;
            if version != 0 {
                ctrl = estimate_control(measured, count, level);
                w.out[control + k4 / 4] |= ctrl << (j * 2);
            }
            if ctrl == 3 {
                if w.remaining() < count {
                    return Err(Error::BufferTooSmall);
                }
                w.out[w.pos..w.pos + count].copy_from_slice(&buffer[..count]);
                w.pos += count;
            } else if ctrl != 2 {
                let bits: &[u32] = match (version, ctrl) {
                    (0, _) => &[0, 2, 4, 8],
                    (_, 0) => &[0, 1, 2, 4],
                    _ => &[1, 2, 4, 8],
                };
                encode_bytes(w, buffer, measured, bits)?;
            }
        }
    }
    last[..stride].copy_from_slice(&data[(count - 1) * stride..count * stride]);
    Ok(())
}

/// meshopt_encodeVertexBufferLevel. Parameters are validated by the caller:
/// stride is 4..=256 and a multiple of four, vertices holds count * stride
/// bytes, version is 0 or 1 and level is 0..=9.
pub(super) fn encode(
    out: &mut [u8],
    vertices: &[u8],
    count: usize,
    stride: usize,
    version: u8,
    level: u8,
) -> Result<usize, Error> {
    let mut w = Writer { out, pos: 0 };
    if w.remaining() < 1 {
        return Err(Error::BufferTooSmall);
    }
    w.out[0] = HEADER | version;
    w.pos = 1;
    let mut first = [0u8; 256];
    if count > 0 {
        first[..stride].copy_from_slice(&vertices[..stride]);
    }
    let mut last = first;
    let block = block_size(stride);
    let mut channels = [0u8; 64];
    let mut lanes = [[0u8; BLOCK_MAX]; 4];
    if version != 0 && level > 1 && count > 1 {
        for k in (0..stride).step_by(4) {
            let rot = if level >= 3 {
                estimate_rotate(vertices, count, stride, k)
            } else {
                0
            };
            let max_channel = if level >= 3 { 3 } else { 2 };
            channels[k / 4] = estimate_channel(
                &mut lanes,
                vertices,
                count,
                stride,
                k,
                block,
                3,
                max_channel,
                rot,
            );
        }
    }
    let mut offset = 0;
    while offset < count {
        let size = block.min(count - offset);
        encode_block(
            &mut w,
            &mut lanes,
            &vertices[offset * stride..(offset + size) * stride],
            size,
            stride,
            &mut last,
            &channels,
            version,
            level,
        )?;
        offset += size;
    }
    let tail = stride + if version == 0 { 0 } else { stride / 4 };
    let padded = tail.max(if version == 0 { TAIL_V0 } else { TAIL_V1 });
    if w.remaining() < padded {
        return Err(Error::BufferTooSmall);
    }
    let mut pos = w.pos;
    w.out[pos..pos + padded - tail].fill(0);
    pos += padded - tail;
    w.out[pos..pos + stride].copy_from_slice(&first[..stride]);
    pos += stride;
    if version != 0 {
        w.out[pos..pos + stride / 4].copy_from_slice(&channels[..stride / 4]);
        pos += stride / 4;
    }
    Ok(pos)
}

/// Fewest blocks in a parallel work unit; smaller buffers encode serially.
#[cfg(feature = "parallel")]
pub(super) const MIN_CHUNK_BLOCKS: usize = 4;
#[cfg(feature = "parallel")]
const ROTATE_TASK_GROUPS: usize = 2048;
#[cfg(feature = "parallel")]
const CHANNEL_TASK_SAMPLES: usize = 32;

#[cfg(feature = "parallel")]
fn ranges(total: usize, per: usize) -> impl Iterator<Item = core::ops::Range<usize>> {
    (0..total)
        .step_by(per)
        .map(move |start| start..(start + per).min(total))
}

#[cfg(feature = "parallel")]
fn zeroed(len: usize) -> Result<alloc::vec::Vec<u8>, Error> {
    let mut v = alloc::vec::Vec::new();
    v.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(len, 0);
    Ok(v)
}

/// Per-channel-group control bytes (version 1, level above 1), computed with
/// the estimators split into integer-summed ranges that run concurrently.
#[cfg(feature = "parallel")]
fn estimate_channels_parallel(
    vertices: &[u8],
    count: usize,
    stride: usize,
    level: u8,
    block: usize,
) -> [u8; 64] {
    use rayon::prelude::*;
    let groups4 = stride / 4;
    let mut rotations = [0u8; 64];
    if level >= 3 {
        let tasks: alloc::vec::Vec<_> = (0..groups4)
            .flat_map(|g| {
                ranges(count.div_ceil(GROUP), ROTATE_TASK_GROUPS).map(move |range| (g, range))
            })
            .collect();
        let sums: alloc::vec::Vec<[usize; 8]> = tasks
            .par_iter()
            .map(|(g, range)| rotate_sizes(vertices, count, stride, g * 4, range.clone()))
            .collect();
        let mut totals = [[0usize; 8]; 64];
        for ((g, _), sum) in tasks.iter().zip(&sums) {
            for (total, value) in totals[*g].iter_mut().zip(sum) {
                *total += value;
            }
        }
        for g in 0..groups4 {
            rotations[g] = pick_rotation(&totals[g]);
        }
    }
    let max_channel = if level >= 3 { 3 } else { 2 };
    let tasks: alloc::vec::Vec<_> = (0..groups4)
        .flat_map(|g| {
            ranges(sample_count(count, block, 3), CHANNEL_TASK_SAMPLES).map(move |range| (g, range))
        })
        .collect();
    let sums: alloc::vec::Vec<[usize; 3]> = tasks
        .par_iter()
        .map(|(g, range)| {
            let mut lanes = [[0u8; BLOCK_MAX]; 4];
            channel_sizes(
                &mut lanes,
                vertices,
                count,
                stride,
                g * 4,
                block,
                3,
                max_channel,
                rotations[*g],
                range.clone(),
            )
        })
        .collect();
    let mut totals = [[0usize; 3]; 64];
    for ((g, _), sum) in tasks.iter().zip(&sums) {
        for (total, value) in totals[*g].iter_mut().zip(sum) {
            *total += value;
        }
    }
    let mut channels = [0u8; 64];
    for g in 0..groups4 {
        channels[g] = pick_channel(&totals[g], max_channel, rotations[g]);
    }
    channels
}

/// Parallel [`encode`] into an exactly sized vector, with the same bytes.
///
/// Block bodies depend only on the input (the baseline of a block is the last
/// input vertex of the previous block) and on the per-channel estimates, so
/// runs of blocks encode independently and concatenate in order. The
/// parameters are validated by the caller as for [`encode`].
#[cfg(feature = "parallel")]
pub(super) fn encode_parallel(
    vertices: &[u8],
    count: usize,
    stride: usize,
    version: u8,
    level: u8,
) -> Result<alloc::vec::Vec<u8>, Error> {
    use rayon::prelude::*;
    let block = block_size(stride);
    let channels = if version != 0 && level > 1 && count > 1 {
        estimate_channels_parallel(vertices, count, stride, level, block)
    } else {
        [0u8; 64]
    };
    let blocks = count.div_ceil(block);
    let per_block = stride * ((block / GROUP).div_ceil(4) + block) + stride / 4;
    let chunk_blocks = blocks
        .div_ceil(rayon::current_num_threads() * 4)
        .max(MIN_CHUNK_BLOCKS);
    let pieces: alloc::vec::Vec<Result<alloc::vec::Vec<u8>, Error>> = ranges(blocks, chunk_blocks)
        .collect::<alloc::vec::Vec<_>>()
        .into_par_iter()
        .map(|run| {
            // Slack beyond the worst case keeps every capacity check passing.
            let mut buffer = zeroed(run.len() * per_block + 64)?;
            let mut w = Writer {
                out: &mut buffer,
                pos: 0,
            };
            let mut lanes = [[0u8; BLOCK_MAX]; 4];
            let mut offset = run.start * block;
            let end = (run.end * block).min(count);
            let mut last = [0u8; 256];
            let previous = offset.saturating_sub(1);
            last[..stride].copy_from_slice(&vertices[previous * stride..(previous + 1) * stride]);
            while offset < end {
                let size = block.min(count - offset);
                encode_block(
                    &mut w,
                    &mut lanes,
                    &vertices[offset * stride..(offset + size) * stride],
                    size,
                    stride,
                    &mut last,
                    &channels,
                    version,
                    level,
                )?;
                offset += size;
            }
            let used = w.pos;
            buffer.truncate(used);
            buffer.shrink_to_fit();
            Ok(buffer)
        })
        .collect();
    let tail = stride + if version == 0 { 0 } else { stride / 4 };
    let padded = tail.max(if version == 0 { TAIL_V0 } else { TAIL_V1 });
    let mut total = 1 + padded;
    for piece in &pieces {
        match piece {
            Ok(piece) => total += piece.len(),
            Err(error) => return Err(*error),
        }
    }
    let mut out = super::vertex_parallel::zeroed(total)?;
    out[0] = HEADER | version;
    let (body, rest) = out[1..].split_at_mut(total - 1 - padded);
    let mut targets = alloc::vec::Vec::new();
    let mut remaining = body;
    for piece in pieces.iter().flatten() {
        let (target, after) = remaining.split_at_mut(piece.len());
        targets.push((target, piece));
        remaining = after;
    }
    targets
        .par_iter_mut()
        .for_each(|(target, piece)| target.copy_from_slice(piece));
    // The padding stays zero; the first vertex and the channel bytes follow it.
    let trailer = &mut rest[padded - tail..];
    if count > 0 {
        trailer[..stride].copy_from_slice(&vertices[..stride]);
    }
    if version != 0 {
        trailer[stride..].copy_from_slice(&channels[..stride / 4]);
    }
    Ok(out)
}
