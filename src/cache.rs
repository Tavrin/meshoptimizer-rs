// Translation of meshoptimizer 1.3 src/vcacheoptimizer.cpp; see LICENSE.
use crate::workspace::{checked_bytes, output, topology, Work};
use crate::{Error, Workspace};
use alloc::vec::Vec;

const CACHE: [f32; 17] = [
    0., 0.779, 0.791, 0.789, 0.981, 0.843, 0.726, 0.847, 0.882, 0.867, 0.799, 0.642, 0.613, 0.600,
    0.568, 0.372, 0.234,
];
const LIVE: [f32; 9] = [0., 0.995, 0.713, 0.450, 0.404, 0.059, 0.005, 0.147, 0.006];
fn score(position: usize, live: u32) -> f32 {
    CACHE[position] + LIVE[live.min(8) as usize]
}

/// Reorder triangles using upstream `meshopt_optimizeVertexCache` (standard variant).
///
/// Returns exactly as many indices as supplied, preserving triangle corner
/// order and vertex references. Empty input is valid. No geometry is welded
/// or compacted. Output and scratch count toward the workspace byte limit.
pub fn optimize_vertex_cache(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        prepare(
            indices.len(),
            vertex_count,
            checked_bytes(indices.len(), 4)?,
            workspace,
        )?;
        let mut destination = output(indices.len())?;
        kernel(
            &mut destination,
            indices,
            vertex_count,
            workspace,
            &mut work,
        )?;
        Ok(destination)
    })();
    workspace.finish(&work);
    result
}

/// Write upstream standard cache ordering to the used prefix of `destination`.
///
/// The unused tail is untouched. Validation and reservation precede writes.
/// Counted-work exhaustion during optimization can leave the used prefix
/// partially modified. Caller-owned destination storage is excluded from limits.
pub fn optimize_vertex_cache_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        if destination.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        prepare(indices.len(), vertex_count, 0, workspace)?;
        kernel(destination, indices, vertex_count, workspace, &mut work)
    })();
    workspace.finish(&work);
    result
}

/// Destructively apply upstream standard cache ordering, leaving input unchanged on error.
/// A fallibly allocated temporary output counts toward the byte limit.
pub fn optimize_vertex_cache_in_place(
    indices: &mut [u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = optimize_vertex_cache(indices, vertex_count, workspace)?;
    indices.copy_from_slice(&result);
    Ok(())
}

fn prepare(n: usize, v: usize, out: usize, ws: &mut Workspace) -> Result<(), Error> {
    if n == 0 {
        return ws.prepare([0; 4], out);
    }
    let faces = n / 3;
    ws.prepare_cache([n, faces, faces, 0], v, out)
}

fn kernel(
    dest: &mut [u32],
    indices: &[u32],
    v: usize,
    ws: &mut Workspace,
    work: &mut Work,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let faces = indices.len() / 3;
    let vertices = &mut ws.vertices[..v];
    let adjacency = &mut ws.integers;
    let triangle_scores = &mut ws.floats;
    let emitted = &mut ws.flags;
    work.add(v)?;
    for vertex in vertices.iter_mut() {
        vertex.live = 0;
    }
    for &index in indices {
        work.add(1)?;
        vertices[index as usize].live += 1;
    }
    let mut offset = 0;
    for vertex in vertices.iter_mut() {
        work.add(1)?;
        vertex.offset = offset;
        offset += vertex.live;
    }
    for (i, triangle) in indices.as_chunks::<3>().0.iter().enumerate() {
        for &index in triangle {
            work.add(1)?;
            let j = index as usize;
            let vertex = &mut vertices[j];
            adjacency[vertex.offset as usize] = i as u32;
            vertex.offset += 1;
        }
    }
    for vertex in vertices.iter_mut() {
        work.add(1)?;
        vertex.offset -= vertex.live;
        vertex.score = score(0, vertex.live);
    }
    work.add(faces)?;
    emitted.fill(0);
    for i in 0..faces {
        work.add(1)?;
        triangle_scores[i] = vertices[indices[i * 3] as usize].score
            + vertices[indices[i * 3 + 1] as usize].score
            + vertices[indices[i * 3 + 2] as usize].score;
    }
    let mut cache = [0u32; 20];
    let mut next = [0u32; 20];
    let mut cache_count = 0;
    let mut current = 0usize;
    let mut cursor = 1usize;
    let mut emitted_count = 0;
    while current != usize::MAX {
        work.add(1)?;
        let triangle = &indices[current * 3..current * 3 + 3];
        dest[emitted_count * 3..emitted_count * 3 + 3].copy_from_slice(triangle);
        emitted_count += 1;
        emitted[current] = 1;
        triangle_scores[current] = 0.;
        next[..3].copy_from_slice(triangle);
        let mut write = 3;
        for &index in &cache[..cache_count] {
            work.add(1)?;
            next[write] = index;
            if !triangle.contains(&index) {
                write += 1;
            }
        }
        core::mem::swap(&mut cache, &mut next);
        cache_count = write.min(16);
        for &index in triangle {
            let j = index as usize;
            let vertex = &mut vertices[j];
            let start = vertex.offset as usize;
            let count = vertex.live as usize;
            let adjacent = &mut adjacency[start..start + count];
            let mut found = None;
            for (i, &triangle) in adjacent.iter().enumerate() {
                work.add(1)?;
                if triangle == current as u32 {
                    found = Some(i);
                    break;
                }
            }
            if let Some(i) = found {
                adjacent[i] = adjacent[count - 1];
                vertex.live -= 1;
            }
        }
        let mut best = usize::MAX;
        let mut best_score = 0.;
        for (i, &index) in cache[..write].iter().enumerate() {
            work.add(1)?;
            let j = index as usize;
            let vertex = &mut vertices[j];
            if vertex.live == 0 {
                continue;
            }
            let s = score(if i >= 16 { 0 } else { i + 1 }, vertex.live);
            let diff = s - vertex.score;
            vertex.score = s;
            for &tri in
                &adjacency[vertex.offset as usize..vertex.offset as usize + vertex.live as usize]
            {
                work.add(1)?;
                let tri = tri as usize;
                let s = triangle_scores[tri] + diff;
                if best_score < s {
                    best = tri;
                    best_score = s;
                }
                triangle_scores[tri] = s;
            }
        }
        current = best;
        if current == usize::MAX {
            while cursor < faces {
                work.add(1)?;
                if emitted[cursor] == 0 {
                    current = cursor;
                    break;
                }
                cursor += 1;
            }
        }
    }
    Ok(())
}
