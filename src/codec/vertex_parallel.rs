// Parallel vertex-codec decoding, not in C++ meshoptimizer.
//
// The stream is a chain of blocks. A block starts from the last vertex of the
// previous block, so blocks cannot be decoded independently as they stand.
// Every channel is a prefix sum (byte or 16-bit addition) or prefix xor of its
// per-block deltas, which makes a decoded block linear in its starting vertex:
// block(base) = base `op` block(0). The decoder therefore
//
// 1. finds the start of every run of blocks with a serial scan that measures
//    each group (its selector and escape count) without expanding any delta,
//    and hands each run to a worker as soon as the scan has passed it,
// 2. decodes each run from a zero baseline (the first run from the stored
//    first vertex),
// 3. chains the last vertex of every run serially, and
// 4. applies each run's true baseline to its records concurrently.
//
// Any malformed or unusual stream (a bound or consistency check failing) makes
// the caller run the serial decoder, which reports the exact serial error.
use super::vertex::stream_block;
use crate::Error;
use alloc::vec::Vec;
use rayon::prelude::*;

/// Fewest blocks in a parallel work unit.
pub(super) const MIN_CHUNK_BLOCKS: usize = 4;

/// `len` zero bytes whose pages the pool's workers touch first. A serial
/// `resize` (or a recycled `calloc` block) would zero them on one thread.
pub(super) fn zeroed(len: usize) -> Result<Vec<u8>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    v.par_extend((0..len).into_par_iter().map(|_| 0u8));
    Ok(v)
}

/// Which of `[0, 2 + p1, 4 + p2, 8 + p4, 16]` encoded bytes a selector means,
/// by stream kind: version 0, version 1 with control 0, version 1 with control 1.
/// The widths are `[0, 2, 4, 8]`, `[0, 1, 2, 4]` and `[1, 2, 4, 8]` bits.
const SELECTOR_CASE: [[usize; 4]; 3] = [[0, 2, 3, 4], [0, 1, 2, 3], [1, 2, 3, 4]];

/// Encoded bytes of the group at `pos`: its packed header plus one literal for
/// every escape code in it, as in the serial decoder's `packed`. `None` if
/// the header lies outside `data`. Branch-free in the selector, which varies
/// unpredictably from group to group.
#[inline(always)]
fn group_len(data: &[u8], pos: usize, case: usize) -> Option<usize> {
    let x = u64::from_le_bytes(*data.get(pos..)?.first_chunk()?);
    let pairs = x & (x >> 1);
    let p1 = (x & 0xffff).count_ones() as usize;
    let p2 = (pairs & 0x5555_5555).count_ones() as usize;
    let p4 = (pairs & (pairs >> 2) & 0x1111_1111_1111_1111).count_ones() as usize;
    Some([0, 2 + p1, 4 + p2, 8 + p4, 16][case])
}

/// Offset after the block starting at `pos`, or `None` if it runs past `data`.
fn skip_block(
    data: &[u8],
    mut pos: usize,
    block: usize,
    stride: usize,
    version: u8,
) -> Option<usize> {
    let groups = block.div_ceil(16);
    let header = groups.div_ceil(4);
    let controls = if version == 0 {
        &[][..]
    } else {
        let c = data.get(pos..pos + stride / 4)?;
        pos += stride / 4;
        c
    };
    for k in 0..stride / 4 {
        // Version 0 has no control bytes: every stream is control 0.
        let control = controls.get(k).copied().unwrap_or(0);
        for j in 0..4 {
            match (control >> (j * 2)) & 3 {
                3 => pos = pos.checked_add(block)?,
                2 => {}
                ctrl => {
                    let cases = &SELECTOR_CASE[if version == 0 {
                        0
                    } else {
                        1 + usize::from(ctrl)
                    }];
                    let selectors = data.get(pos..pos + header)?;
                    pos += header;
                    for g in 0..groups {
                        let selector = (selectors[g / 4] >> (2 * (g % 4))) & 3;
                        pos += group_len(data, pos, cases[usize::from(selector)])?;
                    }
                }
            }
        }
    }
    (pos <= data.len()).then_some(pos)
}

/// Per four-byte channel group: mask of the low bits of each lane and mask of
/// the lane top bits, for the carry-free lane addition
/// `((a & lo) + (b & lo)) ^ ((a ^ b) & hi)`. Exclusive-or is the same
/// expression with an empty `lo` and a full `hi`.
#[derive(Clone, Copy)]
struct Lane {
    lo: u32,
    hi: u32,
}
fn lane(version: u8, channel: u8) -> Lane {
    match if version == 0 { 0 } else { channel & 3 } {
        0 => Lane {
            lo: 0x7f7f_7f7f,
            hi: 0x8080_8080,
        },
        1 => Lane {
            lo: 0x7fff_7fff,
            hi: 0x8000_8000,
        },
        _ => Lane {
            lo: 0,
            hi: u32::MAX,
        },
    }
}
#[inline(always)]
fn combine(l: Lane, a: u32, b: u32) -> u32 {
    ((a & l.lo).wrapping_add(b & l.lo)) ^ ((a ^ b) & l.hi)
}
fn word(record: &[u8], k: usize) -> u32 {
    u32::from_le_bytes([record[k], record[k + 1], record[k + 2], record[k + 3]])
}

/// Everything a worker needs besides its own run.
#[derive(Clone, Copy)]
struct Stream<'a> {
    data: &'a [u8],
    channels: &'a [u8],
    first: &'a [u8; 256],
    stride: usize,
    block: usize,
    version: u8,
}

/// Decode the blocks of one run, which occupy `data[pos..end]`, and return its
/// last record. Only the first run starts from the stored first vertex.
fn decode_run(
    stream: Stream<'_>,
    first_run: bool,
    mut pos: usize,
    end: usize,
    out: &mut [u8],
) -> Result<[u8; 256], Error> {
    let Stream {
        data,
        channels,
        first,
        stride,
        block,
        version,
    } = stream;
    let mut last = if first_run { *first } else { [0u8; 256] };
    let mut done = 0;
    while done < out.len() {
        let size = block.min((out.len() - done) / stride);
        let record = &mut out[done..done + size * stride];
        let used = stream_block(&data[pos..], record, size, stride, version, &last, channels)?;
        pos += used;
        last[..stride].copy_from_slice(&record[(size - 1) * stride..]);
        done += size * stride;
    }
    if pos != end {
        return Err(Error::InvalidStream);
    }
    Ok(last)
}

/// Decode `count` records of `stride` bytes into `output`, which holds exactly
/// `count * stride` bytes. Returns `Err` for anything the serial decoder
/// should judge instead; `Ok` output is byte-identical to the serial result.
/// Callers have validated the layout and stream length like the serial path.
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
    let block = ((8192 / stride) & !15).min(256);
    let blocks = count.div_ceil(block);
    let chunk_blocks = blocks
        .div_ceil(rayon::current_num_threads() * 4)
        .max(MIN_CHUNK_BLOCKS);
    let runs = blocks.div_ceil(chunk_blocks);
    let mut first = [0u8; 256];
    first[..stride].copy_from_slice(&data[start..start + stride]);
    let stream = Stream {
        data,
        channels: &data[start + stride..],
        first: &first,
        stride,
        block,
        version,
    };
    let lanes: Vec<Lane> = (0..stride / 4)
        .map(|k| lane(version, stream.channels.get(k).copied().unwrap_or(0)))
        .collect();
    let mut lasts: Vec<Result<[u8; 256], Error>> = Vec::new();
    lasts
        .try_reserve_exact(runs)
        .map_err(|_| Error::AllocationFailed)?;
    lasts.resize(runs, Err(Error::InvalidStream));
    // The scan runs on this thread and releases each run to the pool once it
    // has found where the run ends.
    let mut body_end = None;
    rayon::scope(|scope| {
        let mut chunks = output.chunks_mut(chunk_blocks * block * stride);
        let mut slots = lasts.iter_mut();
        let (mut run_start, mut pos) = (1, 1);
        let mut run = 0;
        for b in 0..blocks {
            if b % chunk_blocks == 0 && b > 0 {
                if let (Some(out), Some(slot)) = (chunks.next(), slots.next()) {
                    let (from, to, first_run) = (run_start, pos, run == 0);
                    scope.spawn(move |_| *slot = decode_run(stream, first_run, from, to, out));
                }
                run += 1;
                run_start = pos;
            }
            let size = block.min(count - b * block);
            match skip_block(data, pos, size, stride, version) {
                Some(next) => pos = next,
                None => return,
            }
        }
        if let (Some(out), Some(slot)) = (chunks.next(), slots.next()) {
            let (from, to, first_run) = (run_start, pos, run == 0);
            scope.spawn(move |_| *slot = decode_run(stream, first_run, from, to, out));
        }
        body_end = Some(pos);
    });
    if body_end.and_then(|pos| data.len().checked_sub(pos)) != Some(padded) {
        return Err(Error::InvalidStream);
    }
    // True baseline of every run: the previous run's true last vertex.
    let mut bases: Vec<[u32; 64]> = Vec::new();
    bases
        .try_reserve_exact(runs)
        .map_err(|_| Error::AllocationFailed)?;
    let mut previous = [0u32; 64];
    for (run, last) in lasts.into_iter().enumerate() {
        let last = last?;
        bases.push(previous);
        for (k, l) in lanes.iter().enumerate() {
            let local = word(&last, k * 4);
            previous[k] = if run == 0 {
                local
            } else {
                combine(*l, previous[k], local)
            };
        }
    }
    // Apply the baselines. Run 0 already started from the true first vertex.
    output
        .par_chunks_mut(chunk_blocks * block * stride)
        .enumerate()
        .skip(1)
        .for_each(|(run, out)| {
            let base = &bases[run][..lanes.len()];
            for record in out.chunks_exact_mut(stride) {
                for ((word, l), b) in record
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(&lanes)
                    .zip(base)
                {
                    *word = combine(*l, u32::from_le_bytes(*word), *b).to_le_bytes();
                }
            }
        });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{encode_vertex_buffer, VertexEncoding};
    use crate::Workspace;
    use alloc::vec;

    // Calls the parallel decoder directly: the public entry point would hide a
    // failed parallel attempt behind the serial fallback.
    #[test]
    fn parallel_path_decodes_without_falling_back() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let mut state = 12345u32;
        for (stride, version, level, count) in [
            (4, 0, 2, 3000),
            (12, 1, 2, 5000),
            (12, 1, 0, 4999),
            (28, 1, 5, 4100),
            (64, 1, 9, 2500),
        ] {
            let vertices: Vec<u8> = (0..count * stride / 4)
                .flat_map(|i| {
                    state ^= state << 13;
                    state ^= state >> 17;
                    state ^= state << 5;
                    let word = match i % 3 {
                        0 => (i as u32).wrapping_mul(7) + (state & 3),
                        1 => ((i as f32 * 0.01).sin() * 50.).to_bits(),
                        _ => state,
                    };
                    word.to_le_bytes()
                })
                .collect();
            let encoding = VertexEncoding::new(version, level).unwrap();
            let encoded = encode_vertex_buffer(
                &vertices,
                count,
                stride,
                encoding,
                &mut Workspace::default(),
            )
            .unwrap();
            let mut out = vec![0u8; vertices.len()];
            let result = pool.install(|| decode(&mut out, count, stride, &encoded));
            assert_eq!(result, Ok(()), "stride {stride} version {version}");
            assert!(out == vertices, "stride {stride} version {version}");
        }
    }
}
