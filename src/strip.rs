//! Triangle strip conversion, following the pinned `stripifier.cpp` order.
use crate::workspace::{checked_bytes, output, topology};
use crate::{Error, Workspace};
use alloc::vec::Vec;

/// Worst-case output index count for `meshopt_stripify`.
pub fn stripify_bound(index_count: usize) -> Result<usize, Error> {
    if !index_count.is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let bound = (index_count / 3)
        .checked_mul(5)
        .ok_or(Error::SizeOverflow)?;
    checked_bytes(bound, 4)?;
    Ok(bound)
}

/// Worst-case output index count for `meshopt_unstripify`.
pub fn unstripify_bound(index_count: usize) -> Result<usize, Error> {
    if index_count == 1 || index_count == 2 {
        return Err(Error::InvalidTopology);
    }
    let bound = index_count
        .saturating_sub(2)
        .checked_mul(3)
        .ok_or(Error::SizeOverflow)?;
    checked_bytes(bound, 4)?;
    Ok(bound)
}

fn first(buffer: &[[u32; 3]; 8], count: usize, valence: &[u8]) -> usize {
    let mut selected = 0;
    let mut minimum = u32::MAX;
    for (i, t) in buffer[..count].iter().enumerate() {
        let [a, b, c] = t.map(|v| valence[v as usize] as u32);
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

fn key(value: Option<(usize, usize)>) -> usize {
    value.map_or(usize::MAX, |(i, corner)| i * 4 + corner)
}

/// Convert triangles to an exact upstream strip. A nonzero `restart_index` enables restarts.
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
    let mut out = output(bound)?;
    let used = stripify_core(
        &mut out,
        indices,
        vertex_count,
        restart_index,
        workspace,
        output_bytes,
    )?;
    out.truncate(used);
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
    stripify_core(
        destination,
        indices,
        vertex_count,
        restart_index,
        workspace,
        0,
    )
}

fn stripify_core(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    restart_index: u32,
    workspace: &mut Workspace,
    output_bytes: usize,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let bound = stripify_bound(indices.len())?;
        if destination.len() < bound {
            return Err(Error::BufferTooSmall);
        }
        topology(indices, vertex_count, &mut work)?;
        workspace.prepare([0, 0, vertex_count, 0], output_bytes)?;
        let valence = &mut workspace.flags[..vertex_count];
        valence.fill(0);
        for &index in indices {
            work.add(1)?;
            valence[index as usize] = valence[index as usize].wrapping_add(1);
        }

        let mut buffer = [[0u32; 3]; 8];
        let mut count = 0usize;
        let mut offset = 0usize;
        let mut strip = [0u32; 2];
        let mut parity = 0usize;
        let mut size = 0usize;
        let mut continuation: Option<(usize, usize)> = None;
        while count > 0 || offset < indices.len() {
            work.add(1)?;
            while count < 8 && offset < indices.len() {
                buffer[count].copy_from_slice(&indices[offset..offset + 3]);
                count += 1;
                offset += 3;
            }
            if let Some((i, corner)) = continuation {
                let [a, b, c] = buffer[i];
                let v = buffer[i][corner];
                buffer.copy_within(i + 1..count, i);
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
                    destination[size] = strip[0];
                    destination[size + 1] = v;
                    size += 2;
                    strip[1] = v;
                    continuation = Some(swap);
                } else {
                    destination[size] = v;
                    size += 1;
                    strip = [strip[1], v];
                    parity ^= 1;
                    continuation = cont;
                }
            } else {
                let i = first(&buffer, count, valence);
                let [mut a, mut b, mut c] = buffer[i];
                buffer.copy_within(i + 1..count, i);
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
                        destination[size] = restart_index;
                        size += 1;
                    }
                    destination[size..size + 3].copy_from_slice(&[a, b, c]);
                    size += 3;
                    strip = [b, c];
                    parity = 1;
                } else {
                    if size > 0 {
                        destination[size] = strip[1];
                        destination[size + 1] = a;
                        size += 2;
                    }
                    let (e0, e1) = if parity != 0 { (c, b) } else { (b, c) };
                    destination[size..size + 3].copy_from_slice(&[a, e0, e1]);
                    size += 3;
                    strip = [e0, e1];
                    parity ^= 1;
                }
            }
        }
        Ok(size)
    })();
    workspace.finish(&work);
    result
}

/// Convert a strip to triangles, skipping degenerate triangles as upstream does.
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
    let mut out = output(bound)?;
    let used = unstripify_core(&mut out, indices, restart_index, workspace, output_bytes)?;
    out.truncate(used);
    Ok(out)
}

/// Caller-buffer form of `meshopt_unstripify`; destination needs the bound capacity.
pub fn unstripify_into(
    destination: &mut [u32],
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    unstripify_core(destination, indices, restart_index, workspace, 0)
}

fn unstripify_core(
    destination: &mut [u32],
    indices: &[u32],
    restart_index: u32,
    workspace: &mut Workspace,
    output_bytes: usize,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let bound = unstripify_bound(indices.len())?;
        if destination.len() < bound {
            return Err(Error::BufferTooSmall);
        }
        workspace.account_codec(output_bytes)?;
        let mut size = 0;
        let mut start = 0;
        for (i, &c) in indices.iter().enumerate() {
            work.add(1)?;
            if restart_index != 0 && c == restart_index {
                start = i + 1;
            } else if i - start >= 2 {
                let (mut a, mut b) = (indices[i - 2], indices[i - 1]);
                if (i - start) & 1 != 0 {
                    core::mem::swap(&mut a, &mut b);
                }
                if a != b && a != c && b != c {
                    destination[size..size + 3].copy_from_slice(&[a, b, c]);
                    size += 3;
                }
            }
        }
        Ok(size)
    })();
    workspace.finish(&work);
    result
}
