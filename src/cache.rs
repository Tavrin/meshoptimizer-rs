// Translation of meshoptimizer 1.3 src/vcacheoptimizer.cpp; see LICENSE.
use crate::budget::Budget;
use crate::workspace::{checked_bytes, output, topology, Work};
use crate::{Error, Workspace};
use alloc::vec::Vec;

const CACHE: [f32; 17] = [
    0., 0.779, 0.791, 0.789, 0.981, 0.843, 0.726, 0.847, 0.882, 0.867, 0.799, 0.642, 0.613, 0.600,
    0.568, 0.372, 0.234,
];
const LIVE: [f32; 9] = [0., 0.995, 0.713, 0.450, 0.404, 0.059, 0.005, 0.147, 0.006];
const STRIP_CACHE: [f32; 17] = [
    0., 1., 1., 1., 0.453, 0.561, 0.490, 0.459, 0.179, 0.526, 0., 0.227, 0.184, 0.490, 0.112,
    0.050, 0.131,
];
const STRIP_LIVE: [f32; 9] = [0., 0.956, 0.786, 0.577, 0.558, 0.618, 0.549, 0.499, 0.489];
/// Caller-defined vertex-cache scores; an opt-in extension not in C++ meshoptimizer.
///
/// Cache score zero is for a vertex outside the cache; entries 1 through 16
/// correspond to most to least recently used. Live-valence scores cover 0 through
/// 8 remaining triangle references; larger valences use entry 8. The cache size
/// and valence clamp cannot be changed. [`Default`] reproduces the standard table.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexScoreTable {
    cache: [f32; 17],
    live: [f32; 9],
}

impl VertexScoreTable {
    /// Copy and validate exactly 17 cache scores and 9 live-valence scores.
    /// Construction allocates no heap storage and needs no workspace.
    ///
    /// # Errors
    /// `InvalidParameter` if either length differs or any score is non-finite.
    pub fn new(cache: &[f32], live: &[f32]) -> Result<Self, Error> {
        if cache.len() != 17 || live.len() != 9 || !cache.iter().chain(live).all(|s| s.is_finite())
        {
            return Err(Error::InvalidParameter);
        }
        let mut table = Self::default();
        table.cache.copy_from_slice(cache);
        table.live.copy_from_slice(live);
        Ok(table)
    }

    /// Cache-position scores, including the outside-cache score at zero.
    pub fn cache_scores(&self) -> &[f32; 17] {
        &self.cache
    }

    /// Live-valence scores; valences above eight use the final entry.
    pub fn live_scores(&self) -> &[f32; 9] {
        &self.live
    }
}

impl Default for VertexScoreTable {
    fn default() -> Self {
        Self {
            cache: CACHE,
            live: LIVE,
        }
    }
}

trait Scores {
    fn score(&self, position: usize, live: u32) -> f32;
}

struct Preset<const STRIP: bool>;

impl<const STRIP: bool> Scores for Preset<STRIP> {
    fn score(&self, position: usize, live: u32) -> f32 {
        if STRIP {
            STRIP_CACHE[position] + STRIP_LIVE[live.min(8) as usize]
        } else {
            CACHE[position] + LIVE[live.min(8) as usize]
        }
    }
}

impl Scores for VertexScoreTable {
    fn score(&self, position: usize, live: u32) -> f32 {
        self.cache[position] + self.live[live.min(8) as usize]
    }
}

/// Reorder triangles with caller-defined scores; not in C++ meshoptimizer.
///
/// Uses the same workspace limits, topology validation and triangle preservation
/// as [`optimize_vertex_cache`]. The default table gives identical output.
/// Tuned tables must be validated on the target hardware; no improvement is
/// guaranteed. Extreme finite scores can overflow intermediate score arithmetic.
///
/// ```
/// use meshoptimizer_rs::{optimize_vertex_cache_with_table, VertexScoreTable, Workspace};
/// let defaults = VertexScoreTable::default();
/// let mut cache = *defaults.cache_scores();
/// cache[1] = 1.0;
/// let table = VertexScoreTable::new(&cache, defaults.live_scores())?;
/// let result = optimize_vertex_cache_with_table(&[0, 1, 2], 3, &table,
///                                               &mut Workspace::default())?;
/// assert_eq!(result, [0, 1, 2]);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// Same errors as [`optimize_vertex_cache`].
pub fn optimize_vertex_cache_with_table(
    indices: &[u32],
    vertex_count: usize,
    table: &VertexScoreTable,
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
            table,
        )?;
        Ok(destination)
    })();
    workspace.finish(&work);
    result
}

/// Write caller-defined cache ordering; not in C++ meshoptimizer.
///
/// The unused destination tail is untouched. Validation and reservation precede
/// writes; work exhaustion can partially modify the used prefix. Caller-owned
/// destination storage is excluded from workspace byte limits.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache_with_table`], plus `BufferTooSmall`
/// if the destination is shorter than the input.
pub fn optimize_vertex_cache_with_table_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    table: &VertexScoreTable,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        if destination.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        prepare(indices.len(), vertex_count, 0, workspace)?;
        kernel(
            destination,
            indices,
            vertex_count,
            workspace,
            &mut work,
            table,
        )
    })();
    workspace.finish(&work);
    result
}

/// Reorder triangles using upstream `meshopt_optimizeVertexCache` (standard variant).
///
/// Returns exactly as many indices as supplied, preserving triangle corner
/// order and vertex references. Empty input is valid. No geometry is welded
/// or compacted. Output and scratch count toward the workspace byte limit.
///
/// # Errors
/// `InvalidTopology` for incomplete triangles or `IndexOutOfBounds` for invalid vertex references.
/// `SizeOverflow` for size overflow, `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed reservation.
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
            &Preset::<false>,
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
///
/// # Errors
/// Same errors as [`optimize_vertex_cache`]. `BufferTooSmall` if the destination is shorter than the input.
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
        kernel(
            destination,
            indices,
            vertex_count,
            workspace,
            &mut work,
            &Preset::<false>,
        )
    })();
    workspace.finish(&work);
    result
}

/// Destructively apply upstream standard cache ordering, leaving input unchanged on error.
/// A fallibly allocated temporary output counts toward the byte limit.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache`].
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
    ws.acquire(crate::workspace::ScratchFamily::Cache);
    if n == 0 {
        return ws.prepare([0; 4], out);
    }
    let faces = n / 3;
    ws.prepare_cache([n, faces, faces, 0], v, out)
}

fn kernel<S: Scores>(
    dest: &mut [u32],
    indices: &[u32],
    v: usize,
    ws: &mut Workspace,
    work: &mut Work,
    scores: &S,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let faces = indices.len() / 3;
    let vertices = &mut ws.vertices[..v];
    let adjacency = &mut ws.integers[..indices.len()];
    let triangle_scores = &mut ws.floats[..faces];
    let emitted = &mut ws.flags[..faces];
    let triangles = indices.as_chunks::<3>().0;
    let destination = dest[..indices.len()].as_chunks_mut::<3>().0;
    assert_eq!(triangles.len(), triangle_scores.len());
    assert_eq!(triangles.len(), emitted.len());
    work.add(v)?;
    for vertex in vertices.iter_mut() {
        vertex.live = 0;
    }
    work.scan(indices.iter().copied(), |index| {
        vertices[index as usize].live += 1;
        Ok(())
    })?;
    let mut offset = 0;
    work.scan(vertices.iter_mut(), |vertex| {
        vertex.offset = offset;
        offset += vertex.live;
        Ok(())
    })?;
    if work.precharge(indices.len(), 1)? {
        for (i, triangle) in triangles.iter().enumerate() {
            for &index in triangle {
                let vertex = &mut vertices[index as usize];
                adjacency[vertex.offset as usize] = i as u32;
                vertex.offset += 1;
            }
        }
    } else {
        for (i, triangle) in indices.as_chunks::<3>().0.iter().enumerate() {
            for &index in triangle {
                work.add(1)?;
                let j = index as usize;
                let vertex = &mut vertices[j];
                adjacency[vertex.offset as usize] = i as u32;
                vertex.offset += 1;
            }
        }
    }
    let mut max_live = 0usize;
    work.scan(vertices.iter_mut(), |vertex| {
        max_live = max_live.max(vertex.live as usize);
        vertex.offset -= vertex.live;
        vertex.score = scores.score(0, vertex.live);
        Ok(())
    })?;
    work.add(faces)?;
    emitted.fill(0);
    work.scan(
        triangle_scores[..faces].iter_mut().zip(triangles),
        |(s, triangle)| {
            *s = vertices[triangle[0] as usize].score
                + vertices[triangle[1] as usize].score
                + vertices[triangle[2] as usize].score;
            Ok(())
        },
    )?;
    // Per emission: one face, <=16 old cache entries, three removal searches
    // of <=max_live entries, <=19 score visits each scanning <=max_live. The
    // cursor advances <=faces-1 times and stops on <=faces-1 selections.
    let bound = max_live
        .checked_mul(22)
        .and_then(|n| n.checked_add(38))
        .and_then(|n| faces.checked_mul(n));
    if let Some(bound) = bound.filter(|&n| work.covers(n) == Ok(true)) {
        let mut counted = CacheWork::<true>::reserved(work, bound)?;
        let result = cache_loop::<S, true>(
            destination,
            triangles,
            vertices,
            adjacency,
            triangle_scores,
            emitted,
            &mut counted,
            scores,
        );
        counted.finish();
        result
    } else {
        cache_loop::<S, false>(
            destination,
            triangles,
            vertices,
            adjacency,
            triangle_scores,
            emitted,
            &mut CacheWork::<false>::new(work),
            scores,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn cache_loop<S: Scores, const FAST: bool>(
    destination: &mut [[u32; 3]],
    triangles: &[[u32; 3]],
    vertices: &mut [crate::workspace::CacheVertex],
    adjacency: &mut [u32],
    triangle_scores: &mut [f32],
    emitted: &mut [u8],
    work: &mut CacheWork<'_, FAST>,
    scores: &S,
) -> Result<(), Error> {
    let faces = triangles.len();
    assert_eq!(triangles.len(), triangle_scores.len());
    assert_eq!(triangles.len(), emitted.len());
    let mut cache_storage = [0u32; 20];
    let mut next_storage = [0u32; 20];
    // Upstream exchanges pointers, not the contents of its two cache arrays.
    let mut cache = &mut cache_storage;
    let mut next = &mut next_storage;
    let mut cache_count = 0;
    let mut current = 0usize;
    let mut cursor = 1usize;
    let mut emitted_count = 0;
    while current != usize::MAX {
        work.add(1)?;
        let triangle = &triangles[current];
        destination[emitted_count].copy_from_slice(triangle);
        emitted_count += 1;
        emitted[current] = 1;
        triangle_scores[current] = 0.;
        next[..3].copy_from_slice(triangle);
        let mut write = 3;
        assert!(cache_count <= 16);
        work.scan(cache[..cache_count].iter().copied(), |index| {
            next[write] = index;
            write += usize::from(!triangle.contains(&index));
            Ok(())
        })?;
        core::mem::swap(&mut cache, &mut next);
        cache_count = write.min(16);
        for &index in triangle {
            let j = index as usize;
            let vertex = &mut vertices[j];
            let start = vertex.offset as usize;
            let count = vertex.live as usize;
            let adjacent = &mut adjacency[start..start + count];
            let found = work.find(adjacent.iter().copied(), |tri| tri == current as u32)?;
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
            let s = scores.score(if i >= 16 { 0 } else { i + 1 }, vertex.live);
            let diff = s - vertex.score;
            vertex.score = s;
            work.scan(
                adjacency[vertex.offset as usize..vertex.offset as usize + vertex.live as usize]
                    .iter()
                    .copied(),
                |tri| {
                    let tri = tri as usize;
                    let s = triangle_scores[tri] + diff;
                    if best_score < s {
                        best = tri;
                        best_score = s;
                    }
                    triangle_scores[tri] = s;
                    Ok(())
                },
            )?;
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

/// Optimize cache ordering using upstream's strip-oriented score table.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache`].
pub fn optimize_vertex_cache_strip(
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
            &Preset::<true>,
        )?;
        Ok(destination)
    })();
    workspace.finish(&work);
    result
}
/// Write strip-oriented cache ordering; work exhaustion may partially modify
/// the used prefix. The unused destination tail is untouched.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache_strip`]. `BufferTooSmall` if the destination is shorter than the input.
pub fn optimize_vertex_cache_strip_into(
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
        kernel(
            destination,
            indices,
            vertex_count,
            workspace,
            &mut work,
            &Preset::<true>,
        )
    })();
    workspace.finish(&work);
    result
}
/// Destructively apply strip-oriented ordering, preserving indices on error.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache_strip`].
pub fn optimize_vertex_cache_strip_in_place(
    indices: &mut [u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = optimize_vertex_cache_strip(indices, vertex_count, workspace)?;
    indices.copy_from_slice(&result);
    Ok(())
}
/// Optimize using upstream's FIFO cache heuristic. Cache size is at least three.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache`]. `InvalidParameter` if the cache size is below three.
pub fn optimize_vertex_cache_fifo(
    indices: &[u32],
    vertex_count: usize,
    cache_size: u32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        if cache_size < 3 {
            return Err(Error::InvalidParameter);
        }
        workspace.acquire(crate::workspace::ScratchFamily::Cache);
        let mut budget = Budget::new(workspace);
        let mut out = budget.filled(indices.len(), 0u32)?;
        fifo(
            &mut out,
            indices,
            vertex_count,
            cache_size,
            &mut budget,
            &mut work,
        )?;
        Ok(out)
    })();
    workspace.finish(&work);
    result
}
/// Write FIFO cache ordering; work exhaustion may partially modify the prefix.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache_fifo`]. `BufferTooSmall` if the destination is shorter than the input.
pub fn optimize_vertex_cache_fifo_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    cache_size: u32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        if cache_size < 3 {
            return Err(Error::InvalidParameter);
        }
        if destination.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        workspace.acquire(crate::workspace::ScratchFamily::Cache);
        fifo(
            destination,
            indices,
            vertex_count,
            cache_size,
            &mut Budget::new(workspace),
            &mut work,
        )
    })();
    workspace.finish(&work);
    result
}
/// Destructively apply FIFO ordering, preserving indices on error.
///
/// # Errors
/// Same errors as [`optimize_vertex_cache_fifo`].
pub fn optimize_vertex_cache_fifo_in_place(
    indices: &mut [u32],
    vertex_count: usize,
    cache_size: u32,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let result = optimize_vertex_cache_fifo(indices, vertex_count, cache_size, workspace)?;
    indices.copy_from_slice(&result);
    Ok(())
}
fn fifo(
    out: &mut [u32],
    indices: &[u32],
    v: usize,
    cache_size: u32,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let length = v
        .checked_mul(4)
        .and_then(|n| indices.len().checked_mul(2).and_then(|m| n.checked_add(m)))
        .ok_or(Error::SizeOverflow)?;
    let owned = budget.bytes();
    let ws = &mut *budget.ws;
    let old_integer_len = ws.integers.len();
    let old_flag_len = ws.flags.len();
    ws.prepare([length, 0, indices.len() / 3, 0], owned)?;
    // prepare initializes every newly extended element to zero. Only retained
    // active elements can contain state from an earlier operation.
    ws.integers[..old_integer_len.min(v)].fill(0);
    let old_hot = old_integer_len.saturating_sub(v * 2).min(v * 2);
    ws.integers[v * 2..v * 2 + old_hot].fill(0);
    ws.flags[..old_flag_len.min(indices.len() / 3)].fill(0);
    let (counts, rest) = ws.integers[..length].split_at_mut(v);
    let (offsets, rest) = rest.split_at_mut(v);
    let (hot, rest) = rest.split_at_mut(v * 2);
    let hot = hot.as_chunks_mut::<2>().0;
    let (adjacency, dead) = rest.split_at_mut(indices.len());
    let emitted = &mut ws.flags[..indices.len() / 3];
    let charged = work.precharge(indices.len(), 1)?;
    for &i in indices {
        if !charged {
            work.add(1)?;
        }
        counts[i as usize] += 1;
    }
    let mut offset = 0;
    let charged = work.precharge(v, 1)?;
    for (dst, &count) in offsets.iter_mut().zip(counts.iter()) {
        if !charged {
            work.add(1)?;
        }
        *dst = offset;
        offset += count;
    }
    let triangles = indices.as_chunks::<3>().0;
    if work.precharge(indices.len(), 1)? {
        // The triangle id is shared by its three corners. Iterating records
        // avoids dividing an index-stream offset for every adjacency entry.
        for (i, triangle) in triangles.iter().enumerate() {
            for &j in triangle {
                adjacency[offsets[j as usize] as usize] = i as u32;
                offsets[j as usize] += 1;
            }
        }
    } else {
        for (i, &j) in indices.iter().enumerate() {
            work.add(1)?;
            adjacency[offsets[j as usize] as usize] = (i / 3) as u32;
            offsets[j as usize] += 1;
        }
    }
    let charged = work.precharge(v, 1)?;
    for ((offset, &count), hot) in offsets.iter_mut().zip(counts.iter()).zip(hot.iter_mut()) {
        if !charged {
            work.add(1)?;
        }
        *offset -= count;
        hot[0] = count;
    }
    let mut current = 0u32;
    let mut timestamp = cache_size.wrapping_add(1);
    let mut cursor = 1usize;
    let mut top = 0usize;
    let mut write = 0usize;
    let destination = out[..indices.len()].as_chunks_mut::<3>().0;
    while current != u32::MAX {
        work.add(1)?;
        let begin = top;
        let j = current as usize;
        let adjacent = &adjacency[offsets[j] as usize..(offsets[j] + counts[j]) as usize];
        let charged = work.precharge(adjacent.len(), 1)?;
        for &triangle in adjacent {
            if !charged {
                work.add(1)?;
            }
            let t = triangle as usize;
            if emitted[t] != 0 {
                continue;
            }
            let tri = &triangles[t];
            destination[write].copy_from_slice(tri);
            write += 1;
            dead[top..top + 3].copy_from_slice(tri);
            top += 3;
            for &index in tri {
                let vertex = &mut hot[index as usize];
                vertex[0] -= 1;
                if timestamp.wrapping_sub(vertex[1]) > cache_size {
                    vertex[1] = timestamp;
                    timestamp = timestamp.wrapping_add(1);
                }
            }
            emitted[t] = 1;
        }
        current = u32::MAX;
        let mut best = -1i32;
        let candidates = &dead[begin..top];
        let charged = work.precharge(candidates.len(), 1)?;
        for &candidate in candidates {
            if !charged {
                work.add(1)?;
            }
            let vertex = hot[candidate as usize];
            if vertex[0] != 0 {
                let age = timestamp.wrapping_sub(vertex[1]);
                let priority = if vertex[0].wrapping_mul(2).wrapping_add(age) <= cache_size {
                    age as i32
                } else {
                    0
                };
                if priority > best {
                    current = candidate;
                    best = priority;
                }
            }
        }
        if current == u32::MAX {
            work.search(top, |_| {
                top -= 1;
                let vertex = dead[top];
                if hot[vertex as usize][0] > 0 {
                    current = vertex;
                    true
                } else {
                    false
                }
            })?;
            if current == u32::MAX {
                work.search(v - cursor, |_| {
                    if hot[cursor][0] > 0 {
                        current = cursor as u32;
                        true
                    } else {
                        cursor += 1;
                        false
                    }
                })?;
            }
        }
    }
    Ok(())
}

// FAST prepays a proven whole-kernel bound and refunds unvisited work at the
// end. Exact visits are accumulated without budget checks. The limited monomorph keeps
// every original charge before its observable mutation.
struct CacheWork<'a, const FAST: bool> {
    work: &'a mut Work,
    spent: usize,
    reserved: usize,
}
impl<'a, const FAST: bool> CacheWork<'a, FAST> {
    fn new(work: &'a mut Work) -> Self {
        Self {
            work,
            spent: 0,
            reserved: 0,
        }
    }
    fn reserved(work: &'a mut Work, bound: usize) -> Result<Self, Error> {
        work.add(bound)?;
        Ok(Self {
            work,
            spent: 0,
            reserved: bound,
        })
    }
    #[inline(always)]
    fn add(&mut self, count: usize) -> Result<(), Error> {
        if FAST {
            self.spent += count;
            Ok(())
        } else {
            self.work.add(count)
        }
    }
    #[inline(always)]
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if FAST {
            self.spent += values.len();
            for value in values {
                visit(value)?;
            }
            Ok(())
        } else {
            self.work.scan(values, visit)
        }
    }
    #[inline(always)]
    fn find<T>(
        &mut self,
        mut values: impl ExactSizeIterator<Item = T>,
        mut found: impl FnMut(T) -> bool,
    ) -> Result<Option<usize>, Error> {
        if FAST {
            let count = values.len();
            let result = values.position(found);
            self.spent += result.map_or(count, |i| i + 1);
            Ok(result)
        } else {
            // search visits at most `len` items, so `next` is always Some.
            self.work
                .search(values.len(), |_| values.next().is_some_and(&mut found))
        }
    }
    fn finish(self) {
        // The dispatch bound covers every possible visit; all covered-path
        // callbacks are infallible and there are no early exits.
        debug_assert!(self.spent <= self.reserved, "cache work bound overrun");
        self.work
            .refund_cache(self.reserved.saturating_sub(self.spent));
    }
}
