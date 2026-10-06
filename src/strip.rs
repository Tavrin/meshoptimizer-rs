//! Triangle strip conversion, following the pinned `stripifier.cpp` order.
use crate::workspace::{checked_bytes, topology};
use crate::{Error, Workspace};
use alloc::vec::Vec;

trait StripOutput {
    fn limit(&self) -> usize;
    fn count(&self) -> usize;
    fn emit(&mut self, values: &[u32]);
}

impl StripOutput for Vec<u32> {
    #[inline(always)]
    fn limit(&self) -> usize {
        self.capacity()
    }
    #[inline(always)]
    fn count(&self) -> usize {
        self.len()
    }
    #[inline(always)]
    fn emit(&mut self, values: &[u32]) {
        debug_assert!(self.len() + values.len() <= self.capacity());
        self.extend_from_slice(values);
    }
}

struct SliceOutput<'a> {
    data: &'a mut [u32],
    count: usize,
}
impl StripOutput for SliceOutput<'_> {
    #[inline(always)]
    fn limit(&self) -> usize {
        self.data.len()
    }
    #[inline(always)]
    fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    fn emit(&mut self, values: &[u32]) {
        self.data[self.count..self.count + values.len()].copy_from_slice(values);
        self.count += values.len();
    }
}

// Triangle emission advances whole checked chunks; it needs no growing
// offset/range addition in the inner caller-buffer decode loop.
struct TriangleOutput<'a> {
    chunks: core::slice::IterMut<'a, [u32; 3]>,
    initial_chunks: usize,
    capacity: usize,
}
impl StripOutput for TriangleOutput<'_> {
    #[inline(always)]
    fn limit(&self) -> usize {
        self.capacity
    }
    #[inline(always)]
    fn count(&self) -> usize {
        (self.initial_chunks - self.chunks.len()) * 3
    }
    #[inline(always)]
    fn emit(&mut self, values: &[u32]) {
        self.chunks
            .next()
            .expect("validated triangle bound")
            .copy_from_slice(values);
    }
}

#[inline(always)]
fn reserve_output(output: &mut Vec<u32>, bound: usize) -> Result<(), Error> {
    output
        .try_reserve_exact(bound)
        .map_err(|_| Error::AllocationFailed)
}

/// Worst-case output index count for `meshopt_stripify`.
#[inline(always)]
pub fn stripify_bound(index_count: usize) -> Result<usize, Error> {
    if !index_count.is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let triangles = index_count / 3;
    if triangles > isize::MAX as usize / 20 {
        return Err(Error::SizeOverflow);
    }
    Ok(triangles * 5)
}

/// Worst-case output index count for `meshopt_unstripify`.
#[inline(always)]
pub fn unstripify_bound(index_count: usize) -> Result<usize, Error> {
    if index_count == 1 || index_count == 2 {
        return Err(Error::InvalidTopology);
    }
    let triangles = index_count.saturating_sub(2);
    if triangles > isize::MAX as usize / 12 {
        return Err(Error::SizeOverflow);
    }
    Ok(triangles * 3)
}

#[inline(always)]
fn first(buffer: &[[u32; 3]; 8], count: usize, valence: &[u8]) -> usize {
    let mut selected = 0;
    let mut minimum = u32::MAX;
    for (i, t) in buffer[..count].iter().enumerate() {
        let (a, b, c) = (
            valence[t[0] as usize] as u32,
            valence[t[1] as usize] as u32,
            valence[t[2] as usize] as u32,
        );
        let value = if a < b && a < c {
            a
        } else if b < c {
            b
        } else {
            c
        };
        if value < minimum {
            selected = i;
            minimum = value;
        }
    }
    selected
}

#[inline(always)]
fn remove(buffer: &mut [[u32; 3]; 8], index: usize, count: usize) {
    for i in index..count - 1 {
        buffer[i] = buffer[i + 1];
    }
}

#[inline(always)]
fn next(buffer: &[[u32; 3]; 8], count: usize, e0: u32, e1: u32) -> Option<(usize, usize)> {
    for (i, &[a, b, c]) in buffer[..count].iter().enumerate() {
        if e0 == a && e1 == b {
            return Some((i, 2));
        }
        if e0 == b && e1 == c {
            return Some((i, 0));
        }
        if e0 == c && e1 == a {
            return Some((i, 1));
        }
    }
    None
}

#[inline(always)]
fn key(value: Option<(usize, usize)>) -> usize {
    value.map_or(usize::MAX, |(i, corner)| i * 4 + corner)
}

/// Convert triangles to an exact upstream strip. A nonzero `restart_index` enables restarts.
#[inline(always)]
pub fn stripify(
    indices: &[u32],
    vertex_count: usize,
    restart_index: u32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let bound = stripify_bound(indices.len())?;
    let output_bytes = checked_bytes(bound, 4)?;
    if output_bytes > workspace.limits().max_bytes {
        return Err(Error::LimitExceeded);
    }
    let mut out = Vec::new();
    reserve_output(&mut out, bound)?;
    out.resize(bound, 0);
    let count = stripify_core(
        &mut SliceOutput {
            data: &mut out,
            count: 0,
        },
        indices,
        vertex_count,
        restart_index,
        workspace,
        output_bytes,
    )?;
    out.truncate(count);
    Ok(out)
}

/// Caller-buffer form of `meshopt_stripify`; destination needs the bound capacity.
pub fn stripify_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    restart_index: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut output = SliceOutput {
        data: destination,
        count: 0,
    };
    stripify_core(
        &mut output,
        indices,
        vertex_count,
        restart_index,
        workspace,
        0,
    )
}

fn stripify_core(
    destination: &mut impl StripOutput,
    indices: &[u32],
    vertex_count: usize,
    restart_index: u32,
    workspace: &mut Workspace,
    output_bytes: usize,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let bound = stripify_bound(indices.len())?;
        if destination.limit() < bound {
            return Err(Error::BufferTooSmall);
        }
        topology(indices, vertex_count, &mut work)?;
        let retained = workspace.flags.len().min(vertex_count);
        workspace.prepare([0, 0, vertex_count, 0], output_bytes)?;
        let valence = &mut workspace.flags[..vertex_count];
        valence[..retained].fill(0);
        let bulk_valence = work.covers(indices.len())?;
        if bulk_valence {
            work.add(indices.len())?;
        }
        for &index in indices {
            if !bulk_valence {
                work.add(1)?;
            }
            valence[index as usize] = valence[index as usize].wrapping_add(1);
        }

        let mut buffer = [[0u32; 3]; 8];
        let mut count = 0usize;
        let mut offset = 0usize;
        let mut strip = [0u32; 2];
        let mut parity = 0usize;
        let mut continuation: Option<(usize, usize)> = None;
        let triangles = indices.len() / 3;
        let bulk_strip = work.covers(triangles)?;
        if bulk_strip {
            work.add(triangles)?;
        }
        while count > 0 || offset < indices.len() {
            if !bulk_strip {
                work.add(1)?;
            }
            while count < 8 && offset < indices.len() {
                buffer[count].copy_from_slice(&indices[offset..offset + 3]);
                count += 1;
                offset += 3;
            }
            let size = destination.count();
            if let Some((i, corner)) = continuation {
                let [a, b, c] = buffer[i];
                let v = buffer[i][corner];
                remove(&mut buffer, i, count);
                count -= 1;
                for index in [a, b, c] {
                    valence[index as usize] = valence[index as usize].wrapping_sub(1);
                }
                let cont = if parity != 0 {
                    next(&buffer, count, strip[1], v)
                } else {
                    next(&buffer, count, v, strip[1])
                };
                let swap = if cont.is_none() {
                    if parity != 0 {
                        next(&buffer, count, v, strip[0])
                    } else {
                        next(&buffer, count, strip[0], v)
                    }
                } else {
                    None
                };
                if let Some(swap) = swap {
                    destination.emit(&[strip[0], v]);
                    strip[1] = v;
                    continuation = Some(swap);
                } else {
                    destination.emit(&[v]);
                    strip = [strip[1], v];
                    parity ^= 1;
                    continuation = cont;
                }
            } else {
                let i = first(&buffer, count, valence);
                let [mut a, mut b, mut c] = buffer[i];
                remove(&mut buffer, i, count);
                count -= 1;
                for index in [a, b, c] {
                    valence[index as usize] = valence[index as usize].wrapping_sub(1);
                }
                let ea = next(&buffer, count, c, b);
                let eb = next(&buffer, count, a, c);
                let ec = next(&buffer, count, b, a);
                let minimum = key(ea).min(key(eb)).min(key(ec));
                continuation = if key(ea) == minimum {
                    ea
                } else if key(eb) == minimum {
                    (a, b, c) = (b, c, a);
                    eb
                } else if key(ec) == minimum {
                    (a, b, c) = (c, a, b);
                    ec
                } else {
                    None
                };
                if restart_index != 0 {
                    if size > 0 {
                        destination.emit(&[restart_index]);
                    }
                    destination.emit(&[a, b, c]);
                    strip = [b, c];
                    parity = 1;
                } else {
                    if size > 0 {
                        destination.emit(&[strip[1], a]);
                    }
                    let (e0, e1) = if parity != 0 { (c, b) } else { (b, c) };
                    destination.emit(&[a, e0, e1]);
                    strip = [e0, e1];
                    parity ^= 1;
                }
            }
        }
        Ok(destination.count())
    })();
    workspace.finish(&work);
    result
}

/// Convert a strip to triangles, skipping degenerate triangles as upstream does.
#[inline(always)]
pub fn unstripify(
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let bound = unstripify_bound(indices.len())?;
    let output_bytes = checked_bytes(bound, 4)?;
    if output_bytes > workspace.limits().max_bytes {
        return Err(Error::LimitExceeded);
    }
    let mut out = Vec::new();
    reserve_output(&mut out, bound)?;
    // Keep allocation metadata outside the emitting decoder, using validated
    // chunks for every size. Match C++'s initialized requested output storage.
    out.resize(bound, 0);
    let count = {
        let chunks = out.as_chunks_mut::<3>().0;
        let mut output = TriangleOutput {
            initial_chunks: chunks.len(),
            capacity: bound,
            chunks: chunks.iter_mut(),
        };
        unstripify_core(&mut output, indices, restart_index, workspace, output_bytes)?
    };
    out.truncate(count);
    Ok(out)
}

/// Caller-buffer form of `meshopt_unstripify`; destination needs the bound capacity.
pub fn unstripify_into(
    destination: &mut [u32],
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let capacity = destination.len();
    let chunks = destination.as_chunks_mut::<3>().0;
    let mut output = TriangleOutput {
        initial_chunks: chunks.len(),
        capacity,
        chunks: chunks.iter_mut(),
    };
    unstripify_core(&mut output, indices, restart_index, workspace, 0)
}

fn unstripify_core(
    destination: &mut impl StripOutput,
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
    output_bytes: usize,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = unstripify_checked(
        destination,
        indices,
        restart_index,
        workspace,
        output_bytes,
        &mut work,
    );
    workspace.finish(&work);
    result
}

#[inline(always)]
fn unstripify_checked(
    destination: &mut impl StripOutput,
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
    output_bytes: usize,
    work: &mut crate::workspace::Work,
) -> Result<usize, Error> {
    let bound = unstripify_bound(indices.len())?;
    if destination.limit() < bound {
        return Err(Error::BufferTooSmall);
    }
    workspace.account_codec(output_bytes)?;
    if work.covers(indices.len())? {
        work.add(indices.len())?;
        unstripify_loop::<false>(destination, indices, restart_index, work)
    } else {
        unstripify_loop::<true>(destination, indices, restart_index, work)
    }
}

#[inline(always)]
fn unstripify_loop<const CHARGE_WORK: bool>(
    destination: &mut impl StripOutput,
    indices: &[u32],
    restart_index: u32,
    work: &mut crate::workspace::Work,
) -> Result<usize, Error> {
    if restart_index == 0 {
        return unstripify_plain::<CHARGE_WORK>(destination, indices, work);
    }
    let mut start = 0;
    let mut previous2 = 0;
    let mut previous1 = 0;
    for (i, &c) in indices.iter().enumerate() {
        if CHARGE_WORK {
            work.add(1)?;
        }
        if restart_index != 0 && c == restart_index {
            start = i + 1;
        } else if i - start >= 2 {
            // Degeneracy is invariant under winding; avoid swapping values
            // or making branch outcomes depend on parity for rejected corners.
            if (previous2 != previous1) & (previous2 != c) & (previous1 != c) {
                let (mut a, mut b) = (previous2, previous1);
                if (i - start) & 1 != 0 {
                    core::mem::swap(&mut a, &mut b);
                }
                destination.emit(&[a, b, c]);
            }
        }
        previous2 = previous1;
        previous1 = c;
    }
    Ok(destination.count())
}

// Without a restart marker, each possible triangle is one adjacent window.
// Charge the first two input visits individually on the tight path, retaining
// the exact failure prefix before any output can be emitted.
#[inline(always)]
fn unstripify_plain<const CHARGE_WORK: bool>(
    destination: &mut impl StripOutput,
    indices: &[u32],
    work: &mut crate::workspace::Work,
) -> Result<usize, Error> {
    if CHARGE_WORK {
        for _ in 0..indices.len().min(2) {
            work.add(1)?;
        }
    }
    for (i, triangle) in indices.windows(3).enumerate() {
        if CHARGE_WORK {
            work.add(1)?;
        }
        let (mut a, mut b, c) = (triangle[0], triangle[1], triangle[2]);
        if (a != b) & (a != c) & (b != c) {
            if i & 1 != 0 {
                core::mem::swap(&mut a, &mut b);
            }
            destination.emit(&[a, b, c]);
        }
    }
    Ok(destination.count())
}
