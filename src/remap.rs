//! Index generation and fetch processing from upstream indexgenerator.cpp and
//! vfetchoptimizer.cpp. Hash probing and callback traversal match upstream.
use crate::budget::Budget;
use crate::workspace::{checked_bytes, topology, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

const NONE: u32 = u32::MAX;

/// Initialized byte records used as binary equality keys. Supplied padding is
/// part of equality; no typed object is reinterpreted as bytes.
#[derive(Clone, Copy, Debug)]
pub struct VertexStream<'a> {
    data: &'a [u8],
    count: usize,
    size: usize,
    stride: usize,
    offset: usize,
}
impl<'a> VertexStream<'a> {
    /// Validate a stream with a key size of 1 through 256 bytes. Stride may
    /// exceed 256; only the key bytes participate in equality.
    pub fn new(
        data: &'a [u8],
        count: usize,
        size: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        if size == 0 || size > 256 || stride < size {
            return Err(Error::InvalidLayout);
        }
        let end = if count == 0 {
            offset
        } else {
            (count - 1)
                .checked_mul(stride)
                .and_then(|v| v.checked_add(offset))
                .and_then(|v| v.checked_add(size))
                .ok_or(Error::SizeOverflow)?
        };
        if end > data.len() {
            return Err(Error::InvalidLayout);
        }
        if count > u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        Ok(Self {
            data,
            count,
            size,
            stride,
            offset,
        })
    }
    /// Validate tightly packed initialized records.
    pub fn packed(data: &'a [u8], size: usize) -> Result<Self, Error> {
        if size == 0 || !data.len().is_multiple_of(size) {
            return Err(Error::InvalidLayout);
        }
        Self::new(data, data.len() / size, size, size, 0)
    }
    /// Number of records.
    pub const fn len(self) -> usize {
        self.count
    }
    /// Whether there are no records.
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }
    #[inline(always)]
    fn key(self, i: u32) -> &'a [u8] {
        let start = self.offset + i as usize * self.stride;
        &self.data[start..start + self.size]
    }
    #[inline(always)]
    fn key_array<const SIZE: usize>(self, i: u32) -> &'a [u8; SIZE] {
        let start = self.offset + i as usize * self.stride;
        self.data[start..start + SIZE]
            .try_into()
            .expect("validated fixed-width key")
    }
}

/// A compact vertex numbering; unused vertices retain `u32::MAX`.
#[derive(Debug)]
pub struct VertexRemap {
    /// Original vertex to compact vertex index.
    pub remap: Vec<u32>,
    /// Number of unique referenced vertices.
    pub vertex_count: usize,
}
/// Fetch-ordered initialized vertex bytes and rewritten indices.
#[derive(Debug)]
pub struct FetchedMesh {
    /// Rewritten topology.
    pub indices: Vec<u32>,
    /// Used tightly packed vertex records.
    pub vertices: Vec<u8>,
    /// Number of used vertices.
    pub vertex_count: usize,
}
/// Unique first-corner indices and source vertices for provoking-vertex rendering.
#[derive(Debug)]
pub struct ProvokingMesh {
    /// Rotated and rewritten triangle indices.
    pub indices: Vec<u32>,
    /// Output vertex to original vertex mapping, including clones.
    pub reorder: Vec<u32>,
}

fn buckets(n: usize) -> Result<usize, Error> {
    n.checked_add(n / 4)
        .ok_or(Error::SizeOverflow)?
        .max(1)
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)
}
#[inline(always)]
fn hash_update(mut h: u32, mut key: &[u8]) -> u32 {
    const M: u32 = 0x5bd1e995;
    while key.len() >= 4 {
        let mut k = u32::from_le_bytes(key[..4].try_into().expect("four bytes"));
        k = k.wrapping_mul(M);
        k ^= k >> 24;
        k = k.wrapping_mul(M);
        h = h.wrapping_mul(M) ^ k;
        key = &key[4..];
    }
    if !key.is_empty() {
        h ^= u32::from(key[0])
            | (u32::from(key[key.len() / 2]) << 8)
            | (u32::from(key[key.len() - 1]) << 16);
        h = h.wrapping_mul(M);
        h ^= h >> 24;
    }
    h
}
#[inline]
fn spatial_hash(p: [u32; 3]) -> u32 {
    p[0].wrapping_mul(73856093) ^ p[1].wrapping_mul(19349663) ^ p[2].wrapping_mul(83492791)
}
#[inline(always)]
fn position_hash(p: Positions<'_>, i: u32) -> u32 {
    position_value_hash(p.get(i as usize).expect("validated position"))
}
#[inline(always)]
fn position_value_hash(p: [f32; 3]) -> u32 {
    spatial_hash(p.map(|v| {
        let mut x = v.to_bits();
        if x == 0x80000000 {
            x = 0;
        }
        x ^ (x >> 17)
    }))
}
#[inline(always)]
fn validate_indices(indices: Option<&[u32]>, count: usize, work: &mut Work) -> Result<(), Error> {
    if count > u32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    if let Some(indices) = indices {
        work.scan(indices.iter().copied(), |i| {
            if i as usize >= count {
                Err(Error::IndexOutOfBounds)
            } else {
                Ok(())
            }
        })?;
    }
    Ok(())
}
fn streams_valid(streams: &[VertexStream<'_>]) -> Result<usize, Error> {
    if streams.is_empty() || streams.len() > 16 {
        return Err(Error::InvalidLayout);
    }
    let count = streams[0].count;
    if streams.iter().any(|s| s.count != count) {
        return Err(Error::InvalidLayout);
    }
    Ok(count)
}
#[inline]
fn stream_hash(streams: &[VertexStream<'_>], i: u32) -> u32 {
    streams.iter().fold(0, |h, s| hash_update(h, s.key(i)))
}
#[inline]
fn stream_equal(streams: &[VertexStream<'_>], a: u32, b: u32) -> bool {
    streams.iter().all(|s| s.key(a) == s.key(b))
}
#[inline(always)]
fn generate_record<const CANONICAL: bool, const LIMITED: bool, const OUTLINE: bool>(
    index: u32,
    remap: &mut [u32],
    table: &mut [u32],
    hash: &impl Fn(u32) -> u32,
    equal: &mut impl FnMut(u32, u32) -> bool,
    next: &mut u32,
    budget: (&mut Work, &mut usize),
) -> Result<(), Error> {
    let (work, visits) = budget;
    if LIMITED {
        work.add(1)?;
    } else {
        *visits += 1;
    }
    if remap[index as usize] != NONE {
        return Ok(());
    }
    if OUTLINE {
        generate_unique_outlined::<CANONICAL, LIMITED>(
            index,
            remap,
            table,
            hash,
            equal,
            next,
            (work, visits),
        )
    } else {
        generate_unique::<CANONICAL, LIMITED>(
            index,
            remap,
            table,
            hash,
            equal,
            next,
            (work, visits),
        )
    }
}
// Keep the common already-mapped scan small for callback-heavy custom remaps.
#[inline(never)]
fn generate_unique_outlined<const CANONICAL: bool, const LIMITED: bool>(
    index: u32,
    remap: &mut [u32],
    table: &mut [u32],
    hash: &impl Fn(u32) -> u32,
    equal: &mut impl FnMut(u32, u32) -> bool,
    next: &mut u32,
    budget: (&mut Work, &mut usize),
) -> Result<(), Error> {
    generate_unique::<CANONICAL, LIMITED>(index, remap, table, hash, equal, next, budget)
}
#[inline(always)]
fn generate_unique<const CANONICAL: bool, const LIMITED: bool>(
    index: u32,
    remap: &mut [u32],
    table: &mut [u32],
    hash: &impl Fn(u32) -> u32,
    equal: &mut impl FnMut(u32, u32) -> bool,
    next: &mut u32,
    budget: (&mut Work, &mut usize),
) -> Result<(), Error> {
    let (work, visits) = budget;
    let mask = table.len() - 1;
    let mut bucket = hash(index) as usize & mask;
    let mut found = false;
    for probe in 0..table.len() {
        if LIMITED {
            work.add(1)?;
        } else {
            *visits += 1;
        }
        if table[bucket] == NONE || equal(table[bucket], index) {
            found = true;
            break;
        }
        bucket = (bucket + probe + 1) & mask;
    }
    if !found {
        return Err(Error::NumericalFailure);
    }
    if table[bucket] == NONE {
        table[bucket] = index;
        if CANONICAL {
            remap[index as usize] = index;
        } else {
            remap[index as usize] = *next;
            *next += 1;
        }
    } else if CANONICAL {
        remap[index as usize] = table[bucket];
    } else {
        remap[index as usize] = remap[table[bucket] as usize];
    }
    Ok(())
}
fn generate_general_kernel<const CANONICAL: bool, const PREPARED: bool, const OUTLINE: bool>(
    remap: &mut [u32],
    indices: Option<&[u32]>,
    table: &mut [u32],
    hash: impl Fn(u32) -> u32,
    mut equal: impl FnMut(u32, u32) -> bool,
    work: &mut Work,
) -> Result<usize, Error> {
    work.add(remap.len())?;
    if !PREPARED {
        remap.fill(NONE);
    }
    work.add(table.len())?;
    table.fill(NONE);
    let n = indices.map_or(remap.len(), <[u32]>::len);
    let bound = table.len().checked_add(1).ok_or(Error::SizeOverflow)?;
    let mut next = 0;
    let mut begin = 0;
    while begin < n {
        let count = work.batch(n - begin, bound)?;
        let mut visits = 0;
        if count == 0 {
            let index = indices.map_or(begin as u32, |v| v[begin]);
            generate_record::<CANONICAL, true, OUTLINE>(
                index,
                remap,
                table,
                &hash,
                &mut equal,
                &mut next,
                (work, &mut visits),
            )?;
            begin += 1;
        } else {
            let result = (|| {
                for i in begin..begin + count {
                    let index = indices.map_or(i as u32, |v| v[i]);
                    generate_record::<CANONICAL, false, OUTLINE>(
                        index,
                        remap,
                        table,
                        &hash,
                        &mut equal,
                        &mut next,
                        (work, &mut visits),
                    )?;
                }
                Ok(())
            })();
            work.add(visits)?;
            result?;
            begin += count;
        }
    }
    Ok(next as usize)
}
fn generate_kernel<const CANONICAL: bool, const PREPARED: bool, const OUTLINE: bool>(
    remap: &mut [u32],
    indices: Option<&[u32]>,
    table: &mut [u32],
    hash: impl Fn(u32) -> u32,
    equal: impl FnMut(u32, u32) -> bool,
    work: &mut Work,
) -> Result<usize, Error> {
    if OUTLINE {
        if let Some(indices) = indices {
            generate_custom_kernel::<CANONICAL, PREPARED, true>(
                remap, indices, table, hash, equal, work,
            )
        } else {
            generate_custom_kernel::<CANONICAL, PREPARED, false>(
                remap,
                &[],
                table,
                hash,
                equal,
                work,
            )
        }
    } else {
        generate_general_kernel::<CANONICAL, PREPARED, false>(
            remap, indices, table, hash, equal, work,
        )
    }
}
fn generate_custom_kernel<const CANONICAL: bool, const PREPARED: bool, const INDEXED: bool>(
    remap: &mut [u32],
    indices: &[u32],
    table: &mut [u32],
    hash: impl Fn(u32) -> u32,
    mut equal: impl FnMut(u32, u32) -> bool,
    work: &mut Work,
) -> Result<usize, Error> {
    work.add(remap.len())?;
    if !PREPARED {
        remap.fill(NONE);
    }
    work.add(table.len())?;
    table.fill(NONE);
    let n = if INDEXED { indices.len() } else { remap.len() };
    let bound = table.len().checked_add(1).ok_or(Error::SizeOverflow)?;
    let mut next = 0;
    let mut begin = 0;
    while begin < n {
        let count = work.batch(n - begin, bound)?;
        let mut visits = 0;
        if count == 0 {
            let index = if INDEXED {
                indices[begin]
            } else {
                begin as u32
            };
            generate_record::<CANONICAL, true, false>(
                index,
                remap,
                table,
                &hash,
                &mut equal,
                &mut next,
                (work, &mut visits),
            )?;
            begin += 1;
        } else {
            let result = (|| {
                if INDEXED {
                    for &index in &indices[begin..begin + count] {
                        generate_record::<CANONICAL, false, false>(
                            index,
                            remap,
                            table,
                            &hash,
                            &mut equal,
                            &mut next,
                            (work, &mut visits),
                        )?;
                    }
                } else {
                    for index in begin..begin + count {
                        generate_record::<CANONICAL, false, false>(
                            index as u32,
                            remap,
                            table,
                            &hash,
                            &mut equal,
                            &mut next,
                            (work, &mut visits),
                        )?;
                    }
                }
                Ok(())
            })();
            work.add(visits)?;
            result?;
            begin += count;
        }
    }
    Ok(next as usize)
}
fn generate<const OUTLINE: bool>(
    remap: &mut [u32],
    indices: Option<&[u32]>,
    table: &mut [u32],
    hash: impl Fn(u32) -> u32,
    equal: impl FnMut(u32, u32) -> bool,
    canonical: bool,
    work: &mut Work,
) -> Result<usize, Error> {
    if canonical {
        generate_kernel::<true, false, OUTLINE>(remap, indices, table, hash, equal, work)
    } else {
        generate_kernel::<false, false, OUTLINE>(remap, indices, table, hash, equal, work)
    }
}
fn generate_streams(
    remap: &mut [u32],
    indices: Option<&[u32]>,
    table: &mut [u32],
    streams: &[VertexStream<'_>],
    canonical: bool,
    work: &mut Work,
) -> Result<usize, Error> {
    if let [vertices] = streams {
        let vertices = *vertices;
        match vertices.size {
            4 => return generate_single::<4>(remap, indices, table, vertices, canonical, work),
            8 => return generate_single::<8>(remap, indices, table, vertices, canonical, work),
            12 => return generate_single::<12>(remap, indices, table, vertices, canonical, work),
            16 => return generate_single::<16>(remap, indices, table, vertices, canonical, work),
            _ => {}
        }
        generate::<false>(
            remap,
            indices,
            table,
            |i| hash_update(0, vertices.key(i)),
            |a, b| vertices.key(a) == vertices.key(b),
            canonical,
            work,
        )
    } else {
        generate::<false>(
            remap,
            indices,
            table,
            |i| stream_hash(streams, i),
            |a, b| stream_equal(streams, a, b),
            canonical,
            work,
        )
    }
}

fn generate_single<const SIZE: usize>(
    remap: &mut [u32],
    indices: Option<&[u32]>,
    table: &mut [u32],
    vertices: VertexStream<'_>,
    canonical: bool,
    work: &mut Work,
) -> Result<usize, Error> {
    generate::<false>(
        remap,
        indices,
        table,
        |i| hash_update(0, vertices.key_array::<SIZE>(i)),
        |a, b| vertices.key_array::<SIZE>(a) == vertices.key_array::<SIZE>(b),
        canonical,
        work,
    )
}

fn shadow_kernel(
    remap: &mut [u32],
    indices: &[u32],
    output: &mut [u32],
    table: &mut [u32],
    hash: impl Fn(u32) -> u32,
    mut equal: impl FnMut(u32, u32) -> bool,
    work: &mut Work,
) -> Result<(), Error> {
    // The shadow workspace prepares both ranges with the sentinel. Keep the
    // original logical initialization charges and their failure order.
    work.add(remap.len())?;
    work.add(table.len())?;
    let bound = table.len().checked_add(2).ok_or(Error::SizeOverflow)?;
    let mut next = 0;
    let mut begin = 0;
    while begin < indices.len() {
        let count = work.batch(indices.len() - begin, bound)?;
        let mut visits = 0;
        if count == 0 {
            let index = indices[begin];
            generate_record::<true, true, false>(
                index,
                remap,
                table,
                &hash,
                &mut equal,
                &mut next,
                (work, &mut visits),
            )?;
            work.add(1)?;
            output[begin] = remap[index as usize];
            begin += 1;
        } else {
            let result = (|| {
                for i in begin..begin + count {
                    let index = indices[i];
                    generate_record::<true, false, false>(
                        index,
                        remap,
                        table,
                        &hash,
                        &mut equal,
                        &mut next,
                        (work, &mut visits),
                    )?;
                    visits += 1;
                    output[i] = remap[index as usize];
                }
                Ok(())
            })();
            work.add(visits)?;
            result?;
            begin += count;
        }
    }
    Ok(())
}

fn shadow_streams(
    remap: &mut [u32],
    indices: &[u32],
    output: &mut [u32],
    table: &mut [u32],
    streams: &[VertexStream<'_>],
    work: &mut Work,
) -> Result<(), Error> {
    if let [vertices] = streams {
        let vertices = *vertices;
        match vertices.size {
            4 => return shadow_single::<4>(remap, indices, output, table, vertices, work),
            8 => return shadow_single::<8>(remap, indices, output, table, vertices, work),
            12 => return shadow_single::<12>(remap, indices, output, table, vertices, work),
            16 => return shadow_single::<16>(remap, indices, output, table, vertices, work),
            _ => {}
        }
        shadow_kernel(
            remap,
            indices,
            output,
            table,
            |i| hash_update(0, vertices.key(i)),
            |a, b| vertices.key(a) == vertices.key(b),
            work,
        )
    } else {
        shadow_kernel(
            remap,
            indices,
            output,
            table,
            |i| stream_hash(streams, i),
            |a, b| stream_equal(streams, a, b),
            work,
        )
    }
}

fn shadow_single<const SIZE: usize>(
    remap: &mut [u32],
    indices: &[u32],
    output: &mut [u32],
    table: &mut [u32],
    vertices: VertexStream<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    shadow_kernel(
        remap,
        indices,
        output,
        table,
        |i| hash_update(0, vertices.key_array::<SIZE>(i)),
        |a, b| vertices.key_array::<SIZE>(a) == vertices.key_array::<SIZE>(b),
        work,
    )
}

/// Generate compact numbering from tightly packed binary keys. `None` visits
/// every vertex in original order; indexed input need not be a triangle list.
pub fn generate_vertex_remap(
    indices: Option<&[u32]>,
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    generate_vertex_remap_multi(indices, &[vertices], workspace)
}
/// Generate compact numbering from one through sixteen initialized key streams.
pub fn generate_vertex_remap_multi(
    indices: Option<&[u32]>,
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    workspace.begin();
    let count = streams_valid(streams)?;
    if let [vertices] = streams {
        let vertices = *vertices;
        match vertices.size {
            4 => return remap_alloc_single::<4>(indices, vertices, workspace),
            8 => return remap_alloc_single::<8>(indices, vertices, workspace),
            12 => return remap_alloc_single::<12>(indices, vertices, workspace),
            16 => return remap_alloc_single::<16>(indices, vertices, workspace),
            _ => {}
        }
        remap_alloc::<false>(
            indices,
            count,
            |i| hash_update(0, vertices.key(i)),
            |a, b| vertices.key(a) == vertices.key(b),
            false,
            workspace,
        )
    } else {
        remap_alloc::<false>(
            indices,
            count,
            |i| stream_hash(streams, i),
            |a, b| stream_equal(streams, a, b),
            false,
            workspace,
        )
    }
}
fn remap_alloc_single<const SIZE: usize>(
    indices: Option<&[u32]>,
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    remap_alloc::<false>(
        indices,
        vertices.count,
        |i| hash_update(0, vertices.key_array::<SIZE>(i)),
        |a, b| vertices.key_array::<SIZE>(a) == vertices.key_array::<SIZE>(b),
        false,
        workspace,
    )
}
fn remap_alloc<const OUTLINE: bool>(
    indices: Option<&[u32]>,
    count: usize,
    hash: impl Fn(u32) -> u32,
    equal: impl FnMut(u32, u32) -> bool,
    canonical: bool,
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(indices, count, &mut work)?;
        let table_len = buckets(count)?;
        let output_bytes = checked_bytes(count, 4)?;
        workspace.prepare([table_len, 0, 0, 0], output_bytes)?;
        let mut remap = crate::workspace::output_sentinel(count)?;
        if remap.capacity() != count {
            workspace.prepare([table_len, 0, 0, 0], checked_bytes(remap.capacity(), 4)?)?;
        }
        let vertex_count = if canonical {
            generate_kernel::<true, true, OUTLINE>(
                &mut remap,
                indices,
                &mut workspace.integers[..table_len],
                hash,
                equal,
                &mut work,
            )?
        } else {
            generate_kernel::<false, true, OUTLINE>(
                &mut remap,
                indices,
                &mut workspace.integers[..table_len],
                hash,
                equal,
                &mut work,
            )?
        };
        Ok(VertexRemap {
            remap,
            vertex_count,
        })
    })();
    workspace.finish(&work);
    result
}
// Callback work must remain behind numerical position equality. Keeping the
// call out of the probe body also avoids spilling its state on unequal keys.
#[inline(never)]
fn custom_equal(equal: &mut impl FnMut(u32, u32) -> bool, a: u32, b: u32) -> bool {
    equal(a, b)
}
/// Generate compact numbering with a callback invoked only for numerically
/// equal positions, in exactly upstream's hash-probe order. Signed zeros weld.
pub fn generate_vertex_remap_custom(
    indices: Option<&[u32]>,
    positions: Positions<'_>,
    mut equal: impl FnMut(u32, u32) -> bool,
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    if let Some(p) = positions.packed_values() {
        return remap_alloc::<true>(
            indices,
            p.len(),
            |i| position_value_hash(p[i as usize]),
            |a, b| p[a as usize] == p[b as usize] && custom_equal(&mut equal, a, b),
            false,
            workspace,
        );
    }
    remap_alloc::<true>(
        indices,
        positions.len(),
        |i| position_hash(positions, i),
        |a, b| {
            positions.get(a as usize) == positions.get(b as usize) && custom_equal(&mut equal, a, b)
        },
        false,
        workspace,
    )
}
/// Map each position to its first numerically equal vertex. Signed zeros weld;
/// NaNs do not compare equal. The output refers to original indices.
pub fn generate_position_remap(
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    if let Some(p) = positions.packed_values() {
        return Ok(remap_alloc::<false>(
            None,
            p.len(),
            |i| position_value_hash(p[i as usize]),
            |a, b| p[a as usize] == p[b as usize],
            true,
            workspace,
        )?
        .remap);
    }
    Ok(remap_alloc::<false>(
        None,
        positions.len(),
        |i| position_hash(positions, i),
        |a, b| positions.get(a as usize) == positions.get(b as usize),
        true,
        workspace,
    )?
    .remap)
}
/// Generate compact numbering into caller storage. The tail is untouched;
/// work exhaustion may leave the used prefix partially modified.
pub fn generate_vertex_remap_multi_into(
    destination: &mut [u32],
    indices: Option<&[u32]>,
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let count = streams_valid(streams)?;
        validate_indices(indices, count, &mut work)?;
        if destination.len() < count {
            return Err(Error::BufferTooSmall);
        }
        let table_len = buckets(count)?;
        workspace.prepare([table_len, 0, 0, 0], 0)?;
        generate_streams(
            &mut destination[..count],
            indices,
            &mut workspace.integers[..table_len],
            streams,
            false,
            &mut work,
        )
    })();
    workspace.finish(&work);
    result
}
/// Single-stream caller-buffer compact numbering.
pub fn generate_vertex_remap_into(
    destination: &mut [u32],
    indices: Option<&[u32]>,
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    generate_vertex_remap_multi_into(destination, indices, &[vertices], workspace)
}

#[inline(always)]
fn copy_vertex<const SIZE: usize>(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    source: u32,
    target: usize,
) {
    let size = if SIZE == 0 { vertices.size } else { SIZE };
    let input = vertices.offset + source as usize * vertices.stride;
    let output = target * size;
    destination[output..output + size].copy_from_slice(&vertices.data[input..input + size]);
}
fn remap_vertex_copy<const SIZE: usize>(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &[u32],
    work: &mut Work,
) -> Result<(), Error> {
    if work.precharge(remap.len(), 1)? {
        for (i, &r) in remap.iter().enumerate() {
            if r != NONE {
                copy_vertex::<SIZE>(destination, vertices, i as u32, r as usize);
            }
        }
    } else {
        for (i, &r) in remap.iter().enumerate() {
            work.add(1)?;
            if r != NONE {
                copy_vertex::<SIZE>(destination, vertices, i as u32, r as usize);
            }
        }
    }
    Ok(())
}
fn remap_vertex_dispatch(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &[u32],
    work: &mut Work,
) -> Result<(), Error> {
    match vertices.size {
        4 => remap_vertex_copy::<4>(destination, vertices, remap, work),
        8 => remap_vertex_copy::<8>(destination, vertices, remap, work),
        12 => remap_vertex_copy::<12>(destination, vertices, remap, work),
        16 => remap_vertex_copy::<16>(destination, vertices, remap, work),
        _ => remap_vertex_copy::<0>(destination, vertices, remap, work),
    }
}

/// Remap initialized vertex bytes into caller storage. Repeated destinations
/// use the last source record, matching upstream; unmapped records are untouched.
/// Validation precedes writes; work exhaustion may leave partial output.
pub fn remap_vertex_buffer_into(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &[u32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if remap.len() != vertices.count {
            return Err(Error::InvalidLayout);
        }
        let mut required = 0;
        work.scan(remap.iter().copied(), |i| {
            if i != NONE {
                if i as usize >= vertices.count {
                    return Err(Error::IndexOutOfBounds);
                }
                required = required.max(i as usize + 1);
            }
            Ok(())
        })?;
        if destination.len() < checked_bytes(required, vertices.size)? {
            return Err(Error::BufferTooSmall);
        }
        workspace.charge_retained()?;
        remap_vertex_dispatch(destination, vertices, remap, &mut work)?;
        Ok(())
    })();
    workspace.finish(&work);
    result
}
/// Allocate remapped tightly packed bytes, zero-initializing unmapped records.
pub fn remap_vertex_buffer(
    vertices: VertexStream<'_>,
    remap: &[u32],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if remap.len() != vertices.count {
            return Err(Error::InvalidLayout);
        }
        let mut required = 0;
        work.scan(remap.iter().copied(), |i| {
            if i != NONE {
                if i as usize >= vertices.count {
                    return Err(Error::IndexOutOfBounds);
                }
                required = required.max(i as usize + 1);
            }
            Ok(())
        })?;
        let bytes = checked_bytes(required, vertices.size)?;
        let mut budget = Budget::new(workspace);
        let mut out = budget.filled(bytes, 0u8)?;
        remap_vertex_dispatch(&mut out, vertices, remap, &mut work)?;
        Ok(out)
    })();
    workspace.finish(&work);
    result
}
/// Remap arbitrary index sequences. `None` remaps all vertices in order;
/// references to the unused sentinel are rejected before writes.
pub fn remap_index_buffer_into(
    destination: &mut [u32],
    indices: Option<&[u32]>,
    remap: &[u32],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let count = indices.map_or(remap.len(), <[u32]>::len);
        if let Some(input) = indices.filter(|_| {
            count
                .checked_mul(2)
                .is_some_and(|n| work.covers(n).unwrap_or(false))
        }) {
            if remap.len() > u32::MAX as usize {
                return Err(Error::SizeOverflow);
            }
            let mut missing = None;
            for (at, &index) in input.iter().enumerate() {
                let Some(&value) = remap.get(index as usize) else {
                    work.add(at + 1)?;
                    return Err(Error::IndexOutOfBounds);
                };
                if value == NONE && missing.is_none() {
                    missing = Some(at);
                }
            }
            work.add(count)?;
            if destination.len() < count {
                return Err(Error::BufferTooSmall);
            }
            work.add(missing.map_or(count, |at| at + 1))?;
            if missing.is_some() {
                return Err(Error::InvalidParameter);
            }
        } else {
            validate_indices(indices, remap.len(), &mut work)?;
            if destination.len() < count {
                return Err(Error::BufferTooSmall);
            }
            work.scan(0..count, |i| {
                let index = indices.map_or(i as u32, |v| v[i]);
                if remap[index as usize] == NONE {
                    return Err(Error::InvalidParameter);
                }
                Ok(())
            })?;
        }
        workspace.charge_retained()?;
        if work.precharge(count, 1)? {
            if let Some(input) = indices {
                for (out, &index) in destination[..count].iter_mut().zip(input) {
                    *out = remap[index as usize];
                }
            } else {
                destination[..count].copy_from_slice(remap);
            }
        } else {
            for (i, out) in destination[..count].iter_mut().enumerate() {
                work.add(1)?;
                *out = remap[indices.map_or(i as u32, |v| v[i]) as usize];
            }
        }
        Ok(())
    })();
    workspace.finish(&work);
    result
}
#[inline(always)]
fn remap_index_copy(
    output: &mut [u32],
    input: impl ExactSizeIterator<Item = u32>,
    remap: &[u32],
    work: &mut Work,
) -> Result<(), Error> {
    let count = output.len();
    if work.covers(count)? {
        for (at, (out, index)) in output.iter_mut().zip(input).enumerate() {
            *out = remap[index as usize];
            if *out == NONE {
                work.add(at + 1)?;
                return Err(Error::InvalidParameter);
            }
        }
        work.add(count)?;
    } else {
        for (out, index) in output.iter_mut().zip(input) {
            work.add(1)?;
            *out = remap[index as usize];
            if *out == NONE {
                return Err(Error::InvalidParameter);
            }
        }
    }
    Ok(())
}

/// Allocate remapped indices.
pub fn remap_index_buffer(
    indices: Option<&[u32]>,
    remap: &[u32],
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(indices, remap.len(), &mut work)?;
        let count = indices.map_or(remap.len(), <[u32]>::len);
        let mut budget = Budget::new(workspace);
        let mut out = budget.filled(count, 0u32)?;
        if let Some(input) = indices {
            remap_index_copy(&mut out, input.iter().copied(), remap, &mut work)?;
        } else {
            remap_index_copy(&mut out, (0..count).map(|i| i as u32), remap, &mut work)?;
        }
        Ok(out)
    })();
    workspace.finish(&work);
    result
}

#[inline(always)]
fn filter_triangle<const LIMITED: bool>(
    source: &[u32; 3],
    remap: &[u32],
    table: &mut [[u32; 3]],
    out: &mut [u32],
    write: &mut usize,
    budget: (&mut Work, &mut usize),
) -> Result<(), Error> {
    let (work, visits) = budget;
    if LIMITED {
        work.add(1)?;
    } else {
        *visits += 1;
    }
    let mut key = [
        remap[source[0] as usize],
        remap[source[1] as usize],
        remap[source[2] as usize],
    ];
    if key[0] == key[1] || key[0] == key[2] || key[1] == key[2] {
        return Ok(());
    }
    if key[1] < key[0] && key[1] < key[2] {
        key = [key[1], key[2], key[0]];
    } else if key[2] < key[0] && key[2] < key[1] {
        key = [key[2], key[0], key[1]];
    }
    let mask = table.len() - 1;
    let mut bucket = spatial_hash(key) as usize & mask;
    for probe in 0..table.len() {
        if LIMITED {
            work.add(1)?;
        } else {
            *visits += 1;
        }
        if table[bucket] == key {
            return Ok(());
        }
        if table[bucket] == [NONE; 3] {
            table[bucket] = key;
            out[*write..*write + 3].copy_from_slice(source);
            *write += 3;
            return Ok(());
        }
        bucket = (bucket + probe + 1) & mask;
    }
    Err(Error::NumericalFailure)
}

fn filter_triangles(
    out: &mut [u32],
    indices: &[u32],
    remap: &[u32],
    table: &mut [[u32; 3]],
    work: &mut Work,
) -> Result<usize, Error> {
    work.add(table.len())?;
    table.fill([NONE; 3]);
    let source = indices.as_chunks::<3>().0;
    let bound = table.len().checked_add(1).ok_or(Error::SizeOverflow)?;
    let mut write = 0;
    let mut begin = 0;
    while begin < source.len() {
        let count = work.batch(source.len() - begin, bound)?;
        let mut visits = 0;
        if count == 0 {
            filter_triangle::<true>(
                &source[begin],
                remap,
                table,
                out,
                &mut write,
                (work, &mut visits),
            )?;
            begin += 1;
        } else {
            let result = (|| {
                for triangle in &source[begin..begin + count] {
                    filter_triangle::<false>(
                        triangle,
                        remap,
                        table,
                        out,
                        &mut write,
                        (work, &mut visits),
                    )?;
                }
                Ok(())
            })();
            work.add(visits)?;
            result?;
            begin += count;
        }
    }
    Ok(write)
}

fn shadow_filter<const FILTER: bool>(
    indices: &[u32],
    streams: &[VertexStream<'_>],
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<(Vec<u32>, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let count = streams_valid(streams)?;
        topology(indices, count, &mut work)?;
        if destination
            .as_ref()
            .is_some_and(|out| out.len() < indices.len())
        {
            return Err(Error::BufferTooSmall);
        }
        let table_len = buckets(count)?;
        let triangle_len = if FILTER {
            buckets(indices.len() / 3)?
        } else {
            0
        };
        let integer_len = triangle_len
            .checked_mul(3)
            .and_then(|v| v.checked_add(table_len))
            .and_then(|v| v.checked_add(count))
            .ok_or(Error::SizeOverflow)?;
        let output_bytes = if destination.is_none() {
            checked_bytes(indices.len(), 4)?
        } else {
            0
        };
        if FILTER {
            workspace.prepare([integer_len, 0, 0, 0], output_bytes)?;
        } else {
            workspace.prepare_remap(integer_len, output_bytes)?;
        }
        let mut owned = if destination.is_none() {
            crate::workspace::output(indices.len())?
        } else {
            Vec::new()
        };
        if destination.is_none() && owned.capacity() != indices.len() {
            let actual_output = checked_bytes(owned.capacity(), 4)?;
            if FILTER {
                workspace.prepare([integer_len, 0, 0, 0], actual_output)?;
            } else {
                workspace.prepare_remap(integer_len, actual_output)?;
            }
        }
        let out = destination.unwrap_or(&mut owned);
        let out = &mut out[..indices.len()];
        let (remap, rest) = workspace.integers[..integer_len].split_at_mut(count);
        let (table, triangles) = rest.split_at_mut(table_len);
        let triangles = triangles.as_chunks_mut::<3>().0;
        if !FILTER {
            shadow_streams(remap, indices, out, table, streams, &mut work)?;
            return Ok((owned, indices.len()));
        }
        generate_streams(remap, Some(indices), table, streams, false, &mut work)?;
        let write = filter_triangles(out, indices, remap, triangles, &mut work)?;
        owned.truncate(write);
        Ok((owned, write))
    })();
    workspace.finish(&work);
    result
}
/// Replace triangle references with first referenced binary-equal vertices.
pub fn generate_shadow_index_buffer(
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    generate_shadow_index_buffer_multi(indices, &[vertices], workspace)
}
/// Multi-stream shadow indices, preserving original vertex numbering.
pub fn generate_shadow_index_buffer_multi(
    indices: &[u32],
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    Ok(shadow_filter::<false>(indices, streams, None, workspace)?.0)
}
/// Remove binary-degenerate and duplicate triangles; preserve surviving source
/// corners and winding. Rotated duplicates are removed; reversed winding stays.
pub fn filter_index_buffer(
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    filter_index_buffer_multi(indices, &[vertices], workspace)
}
/// Multi-stream triangle filtering.
pub fn filter_index_buffer_multi(
    indices: &[u32],
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    Ok(shadow_filter::<true>(indices, streams, None, workspace)?.0)
}

#[inline(always)]
fn edge_hash(edge: u64, remap: &[u32]) -> u32 {
    let mut h1 = remap[(edge >> 32) as usize];
    let mut h2 = remap[edge as u32 as usize];
    const M: u32 = 0x5bd1e995;
    h1 ^= h2 >> 18;
    h1 = h1.wrapping_mul(M);
    h2 ^= h1 >> 22;
    h2 = h2.wrapping_mul(M);
    h1 ^= h2 >> 17;
    h1 = h1.wrapping_mul(M);
    h2 ^= h1 >> 19;
    h2.wrapping_mul(M)
}
fn patches<const TESS: bool>(
    indices: &[u32],
    positions: Positions<'_>,
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    if let Some(p) = positions.packed_values() {
        patches_kernel::<TESS>(indices, p.len(), |i| p[i as usize], destination, workspace)
    } else {
        patches_kernel::<TESS>(
            indices,
            positions.len(),
            |i| positions.get(i as usize).expect("checked position"),
            destination,
            workspace,
        )
    }
}
fn patches_kernel<const TESS: bool>(
    indices: &[u32],
    vertex_count: usize,
    position: impl Fn(u32) -> [f32; 3],
    destination: Option<&mut [u32]>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        let mut budget = Budget::new(workspace);
        let mut remap = budget.filled(vertex_count, NONE)?;
        let mut table = budget.filled(buckets(vertex_count)?, NONE)?;
        // Adjacency uses binary position equality (including signed zeros),
        // unlike the public numerical generatePositionRemap operation.
        generate::<false>(
            &mut remap,
            None,
            &mut table,
            |i| {
                let mut h = 0;
                for v in position(i) {
                    h = hash_update(h, &v.to_bits().to_le_bytes());
                }
                h
            },
            |a, b| position(a).map(f32::to_bits) == position(b).map(f32::to_bits),
            true,
            &mut work,
        )?;
        budget.release(table);
        let mut edges = budget.filled(buckets(indices.len())?, u64::MAX)?;
        let mut opposite = budget.filled(if TESS { 0 } else { edges.len() }, NONE)?;
        let factor = if TESS { 4 } else { 2 };
        let size = indices
            .len()
            .checked_mul(factor)
            .ok_or(Error::SizeOverflow)?;
        let mut owned = if destination.is_none() {
            budget.filled(size, 0u32)?
        } else {
            Vec::new()
        };
        let out = destination.unwrap_or(&mut owned);
        if out.len() < size {
            return Err(Error::BufferTooSmall);
        }
        let out = &mut out[..size];
        for tri in indices.as_chunks::<3>().0 {
            for e in 0..3 {
                work.add(1)?;
                let edge = (u64::from(tri[e]) << 32) | u64::from(tri[(e + 1) % 3]);
                let slot = edge_lookup(&edges, &remap, edge, &mut work)?;
                if edges[slot] == u64::MAX {
                    edges[slot] = edge;
                    if !TESS {
                        opposite[slot] = tri[(e + 2) % 3];
                    }
                }
            }
        }
        for (tri, patch) in indices
            .as_chunks::<3>()
            .0
            .iter()
            .zip(out.chunks_exact_mut(factor * 3))
        {
            for e in 0..3 {
                work.add(1)?;
                let a = tri[e];
                let b = tri[(e + 1) % 3];
                let edge = (u64::from(b) << 32) | u64::from(a);
                let slot = edge_lookup(&edges, &remap, edge, &mut work)?;
                if TESS {
                    let opp = if edges[slot] == u64::MAX {
                        edge
                    } else {
                        edges[slot]
                    };
                    patch[e] = a;
                    patch[3 + e * 2] = opp as u32;
                    patch[4 + e * 2] = (opp >> 32) as u32;
                    patch[9 + e] = remap[a as usize];
                } else {
                    patch[e * 2] = a;
                    patch[e * 2 + 1] = if edges[slot] == u64::MAX {
                        a
                    } else {
                        opposite[slot]
                    };
                }
            }
        }
        Ok(owned)
    })();
    workspace.finish(&work);
    result
}
#[inline(always)]
fn edge_lookup(table: &[u64], remap: &[u32], edge: u64, work: &mut Work) -> Result<usize, Error> {
    if work.covers(table.len())? {
        edge_lookup_kernel::<false>(table, remap, edge, work)
    } else {
        edge_lookup_kernel::<true>(table, remap, edge, work)
    }
}
#[inline(always)]
fn edge_lookup_kernel<const LIMITED: bool>(
    table: &[u64],
    remap: &[u32],
    edge: u64,
    work: &mut Work,
) -> Result<usize, Error> {
    let mask = table.len() - 1;
    let mut bucket = edge_hash(edge, remap) as usize & mask;
    let a = remap[(edge >> 32) as usize];
    let b = remap[edge as u32 as usize];
    for probe in 0..table.len() {
        if LIMITED {
            work.add(1)?;
        }
        let old = table[bucket];
        if old == u64::MAX || (remap[(old >> 32) as usize] == a && remap[old as u32 as usize] == b)
        {
            if !LIMITED {
                work.add(probe + 1)?;
            }
            return Ok(bucket);
        }
        bucket = (bucket + probe + 1) & mask;
    }
    if !LIMITED {
        work.add(table.len())?;
    }
    Err(Error::NumericalFailure)
}
/// Generate six adjacency indices per triangle using binary position welding.
pub fn generate_adjacency_index_buffer(
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    patches::<false>(indices, positions, None, workspace)
}
/// Generate twelve tessellation indices per triangle using binary position welding.
pub fn generate_tessellation_index_buffer(
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error> {
    patches::<true>(indices, positions, None, workspace)
}
/// Generate a unique first-corner vertex for each triangle, with the same
/// wrapping eight-bit valence heuristic and cyclic rotations as upstream.
pub fn generate_provoking_index_buffer(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<ProvokingMesh, Error> {
    Ok(provoking(indices, vertex_count, None, workspace)?.0)
}
#[inline(always)]
fn provoking_triangle(
    tri: [u32; 3],
    destination: &mut [u32; 3],
    remap: &mut [u32],
    valence: &mut [u8],
    reorder: &mut [u32],
    write: &mut usize,
) {
    let mut tri = tri;
    let rank = |i: u32| {
        if remap[i as usize] == NONE {
            u32::from(valence[i as usize])
        } else {
            NONE
        }
    };
    let v = [rank(tri[0]), rank(tri[1]), rank(tri[2])];
    if v[1] != NONE && v[1] <= v[0] && v[1] <= v[2] {
        tri = [tri[1], tri[2], tri[0]];
    } else if v[2] != NONE && v[2] <= v[0] && v[2] <= v[1] {
        tri = [tri[2], tri[0], tri[1]];
    }
    if remap[tri[0] as usize] == NONE {
        remap[tri[0] as usize] = *write as u32;
    }
    reorder[*write] = tri[0];
    *destination = [*write as u32, tri[1], tri[2]];
    *write += 1;
    for i in tri {
        valence[i as usize] = valence[i as usize].wrapping_sub(1);
    }
}
#[inline(always)]
fn provoking_corner(index: &mut u32, remap: &mut [u32], reorder: &mut [u32], write: &mut usize) {
    let r = &mut remap[*index as usize];
    if *r == NONE {
        *r = *write as u32;
        reorder[*write] = *index;
        *write += 1;
    }
    *index = *r;
}
fn provoking(
    indices: &[u32],
    vertex_count: usize,
    destination: Option<(&mut [u32], &mut [u32])>,
    workspace: &mut Workspace,
) -> Result<(ProvokingMesh, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        topology(indices, vertex_count, &mut work)?;
        let count = vertex_count
            .checked_add(indices.len() / 3)
            .ok_or(Error::SizeOverflow)?;
        if count > u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        workspace.prepare([vertex_count, 0, vertex_count, 0], 0)?;
        let mut budget = Budget::new(workspace);
        let caller = destination.is_some();
        let mut owned = if caller {
            Vec::new()
        } else {
            budget.filled(indices.len(), 0u32)?
        };
        let mut owned_reorder = if caller {
            Vec::new()
        } else {
            budget.filled(count, 0u32)?
        };
        let (out, reorder) = destination.unwrap_or((&mut owned, &mut owned_reorder));
        if out.len() < indices.len() || reorder.len() < count {
            return Err(Error::BufferTooSmall);
        }
        let out = &mut out[..indices.len()];
        let remap = &mut budget.ws.integers[..vertex_count];
        let valence = &mut budget.ws.flags[..vertex_count];
        remap.fill(NONE);
        valence.fill(0);
        if work.precharge(indices.len(), 1)? {
            for &i in indices {
                valence[i as usize] = valence[i as usize].wrapping_add(1);
            }
        } else {
            for &i in indices {
                work.add(1)?;
                valence[i as usize] = valence[i as usize].wrapping_add(1);
            }
        }
        let mut write = 0;
        if work.precharge(indices.len() / 3, 1)? {
            for (&tri, dest) in indices
                .as_chunks::<3>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<3>().0)
            {
                provoking_triangle(tri, dest, remap, valence, reorder, &mut write);
            }
        } else {
            for (&tri, dest) in indices
                .as_chunks::<3>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<3>().0)
            {
                work.add(1)?;
                provoking_triangle(tri, dest, remap, valence, reorder, &mut write);
            }
        }
        let corners = indices.len() / 3 * 2;
        if work.precharge(corners, 1)? {
            for tri in out.as_chunks_mut::<3>().0 {
                provoking_corner(&mut tri[1], remap, reorder, &mut write);
                provoking_corner(&mut tri[2], remap, reorder, &mut write);
            }
        } else {
            for tri in out.as_chunks_mut::<3>().0 {
                for i in &mut tri[1..] {
                    work.add(1)?;
                    provoking_corner(i, remap, reorder, &mut write);
                }
            }
        }
        owned_reorder.truncate(write);
        Ok((
            ProvokingMesh {
                indices: owned,
                reorder: owned_reorder,
            },
            write,
        ))
    })();
    workspace.finish(&work);
    result
}

/// Generate first-use fetch numbering for an arbitrary index sequence.
pub fn optimize_vertex_fetch_remap(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<VertexRemap, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(Some(indices), vertex_count, &mut work)?;
        let mut budget = Budget::new(workspace);
        let mut remap = budget.filled(vertex_count, NONE)?;
        let mut next = 0;
        for &i in indices {
            work.add(1)?;
            if remap[i as usize] == NONE {
                remap[i as usize] = next;
                next += 1;
            }
        }
        Ok(VertexRemap {
            remap,
            vertex_count: next as usize,
        })
    })();
    workspace.finish(&work);
    result
}
/// Fetch numbering into caller storage. Unused vertices are `u32::MAX`;
/// work exhaustion may partially modify the used prefix.
pub fn optimize_vertex_fetch_remap_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(Some(indices), vertex_count, &mut work)?;
        if destination.len() < vertex_count {
            return Err(Error::BufferTooSmall);
        }
        workspace.prepare([0; 4], 0)?;
        work.add(vertex_count)?;
        destination[..vertex_count].fill(NONE);
        let mut next = 0;
        for &i in indices {
            work.add(1)?;
            if destination[i as usize] == NONE {
                destination[i as usize] = next;
                next += 1;
            }
        }
        Ok(next as usize)
    })();
    workspace.finish(&work);
    result
}
#[inline(always)]
fn fetch_visit<const SIZE: usize>(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &mut [u32],
    next: &mut usize,
    index: u32,
    output: &mut u32,
) {
    let r = &mut remap[index as usize];
    if *r == NONE {
        copy_vertex::<SIZE>(destination, vertices, index, *next);
        *r = *next as u32;
        *next += 1;
    }
    *output = *r;
}
fn fetch_kernel<'a, const SIZE: usize>(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &mut [u32],
    indices: impl ExactSizeIterator<Item = (u32, &'a mut u32)>,
    work: &mut Work,
) -> Result<usize, Error> {
    let mut next = 0;
    if work.precharge(indices.len(), 1)? {
        for (index, output) in indices {
            fetch_visit::<SIZE>(destination, vertices, remap, &mut next, index, output);
        }
    } else {
        for (index, output) in indices {
            work.add(1)?;
            fetch_visit::<SIZE>(destination, vertices, remap, &mut next, index, output);
        }
    }
    Ok(next)
}
fn fetch_dispatch<'a>(
    destination: &mut [u8],
    vertices: VertexStream<'_>,
    remap: &mut [u32],
    indices: impl ExactSizeIterator<Item = (u32, &'a mut u32)>,
    work: &mut Work,
) -> Result<usize, Error> {
    match vertices.size {
        4 => fetch_kernel::<4>(destination, vertices, remap, indices, work),
        8 => fetch_kernel::<8>(destination, vertices, remap, indices, work),
        12 => fetch_kernel::<12>(destination, vertices, remap, indices, work),
        16 => fetch_kernel::<16>(destination, vertices, remap, indices, work),
        _ => fetch_kernel::<0>(destination, vertices, remap, indices, work),
    }
}

#[inline(always)]
fn fetch_owned_record<const SIZE: usize, const PACKED: bool>(
    vertices: VertexStream<'_>,
    source: &[[u8; SIZE]],
    remap: &mut [u32],
    index: &mut u32,
    next: &mut usize,
) -> Option<[u8; SIZE]> {
    let entry = &mut remap[*index as usize];
    if *entry == NONE {
        let record = if PACKED {
            source[*index as usize]
        } else {
            *vertices.key_array::<SIZE>(*index)
        };
        *entry = *next as u32;
        *next += 1;
        *index = *entry;
        Some(record)
    } else {
        *index = *entry;
        None
    }
}

fn fetch_owned<const SIZE: usize>(
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
    work: &mut Work,
) -> Result<FetchedMesh, Error> {
    let (mut out, mut records, owned) = {
        let mut budget = Budget::new(workspace);
        let out = budget.copied(indices)?;
        let records = budget.reserved::<[u8; SIZE]>(vertices.count)?;
        (out, records, budget.bytes())
    };
    workspace.prepare_remap(vertices.count, owned)?;
    let remap = &mut workspace.integers[..vertices.count];
    let vertex_count = if vertices.stride == SIZE {
        let source = &vertices.data[vertices.offset..].as_chunks::<SIZE>().0[..vertices.count];
        fetch_owned_kernel::<SIZE, true>(&mut records, vertices, source, remap, &mut out, work)?
    } else {
        fetch_owned_kernel::<SIZE, false>(&mut records, vertices, &[], remap, &mut out, work)?
    };
    debug_assert_eq!(records.len(), vertex_count);
    Ok(FetchedMesh {
        indices: out,
        vertices: records.into_flattened(),
        vertex_count,
    })
}

fn fetch_owned_kernel<const SIZE: usize, const PACKED: bool>(
    records: &mut Vec<[u8; SIZE]>,
    vertices: VertexStream<'_>,
    source: &[[u8; SIZE]],
    remap: &mut [u32],
    out: &mut [u32],
    work: &mut Work,
) -> Result<usize, Error> {
    let mut next = 0;
    if work.precharge(out.len(), 1)? {
        records.extend(out.iter_mut().filter_map(|index| {
            fetch_owned_record::<SIZE, PACKED>(vertices, source, remap, index, &mut next)
        }));
    } else {
        for index in out {
            work.add(1)?;
            if let Some(record) =
                fetch_owned_record::<SIZE, PACKED>(vertices, source, remap, index, &mut next)
            {
                records.push(record);
            }
        }
    }
    Ok(next)
}

/// Allocate first-use vertex bytes and rewritten arbitrary indices.
pub fn optimize_vertex_fetch(
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<FetchedMesh, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(Some(indices), vertices.count, &mut work)?;
        work.add(indices.len())?;
        if vertices.count >= 1024 {
            match vertices.size {
                4 => return fetch_owned::<4>(indices, vertices, workspace, &mut work),
                8 => return fetch_owned::<8>(indices, vertices, workspace, &mut work),
                12 => return fetch_owned::<12>(indices, vertices, workspace, &mut work),
                16 => return fetch_owned::<16>(indices, vertices, workspace, &mut work),
                _ => {}
            }
        }
        let (mut out, mut bytes, owned) = {
            let mut budget = Budget::new(workspace);
            let out = budget.copied(indices)?;
            let bytes = budget.filled(checked_bytes(vertices.count, vertices.size)?, 0u8)?;
            (out, bytes, budget.bytes())
        };
        workspace.prepare_remap(vertices.count, owned)?;
        let remap = &mut workspace.integers[..vertices.count];
        let next = fetch_dispatch(
            &mut bytes,
            vertices,
            remap,
            out.iter_mut().map(|i| (*i, i)),
            &mut work,
        )?;
        bytes.truncate(next * vertices.size);
        Ok(FetchedMesh {
            indices: out,
            vertices: bytes,
            vertex_count: next,
        })
    })();
    workspace.finish(&work);
    result
}
/// Write fetch-ordered bytes while rewriting indices in place. Validation and
/// allocation precede writes; counted-work exhaustion may modify both buffers.
pub fn optimize_vertex_fetch_into(
    destination: &mut [u8],
    indices: &mut [u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(Some(indices), vertices.count, &mut work)?;
        if destination.len() < checked_bytes(vertices.count, vertices.size)? {
            return Err(Error::BufferTooSmall);
        }
        workspace.prepare_remap(vertices.count, 0)?;
        let remap = &mut workspace.integers[..vertices.count];
        let next = fetch_dispatch(
            destination,
            vertices,
            remap,
            indices.iter_mut().map(|i| (*i, i)),
            &mut work,
        )?;
        Ok(next)
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer filtering. The tail is preserved; work exhaustion can leave partial output.
pub fn filter_index_buffer_into(
    destination: &mut [u32],
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    filter_index_buffer_multi_into(destination, indices, &[vertices], workspace)
}
/// Caller-buffer multi-stream filtering; late work exhaustion can leave partial output.
pub fn filter_index_buffer_multi_into(
    destination: &mut [u32],
    indices: &[u32],
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    Ok(shadow_filter::<true>(indices, streams, Some(destination), workspace)?.1)
}
/// Caller-buffer shadow indices; late work exhaustion can leave partial output.
pub fn generate_shadow_index_buffer_into(
    destination: &mut [u32],
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    generate_shadow_index_buffer_multi_into(destination, indices, &[vertices], workspace)
}
/// Caller-buffer multi-stream shadow indices; late work exhaustion can leave partial output.
pub fn generate_shadow_index_buffer_multi_into(
    destination: &mut [u32],
    indices: &[u32],
    streams: &[VertexStream<'_>],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    Ok(shadow_filter::<false>(indices, streams, Some(destination), workspace)?.1)
}
/// Caller-buffer adjacency; requires twice input-length capacity. Late work exhaustion can leave partial output.
pub fn generate_adjacency_index_buffer_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    patches::<false>(indices, positions, Some(destination), workspace)?;
    Ok(())
}
/// Caller-buffer tessellation; requires four times input-length capacity. Late work exhaustion can leave partial output.
pub fn generate_tessellation_index_buffer_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    patches::<true>(indices, positions, Some(destination), workspace)?;
    Ok(())
}
fn remap_into<const OUTLINE: bool>(
    destination: &mut [u32],
    indices: Option<&[u32]>,
    count: usize,
    hash: impl Fn(u32) -> u32,
    equal: impl FnMut(u32, u32) -> bool,
    canonical: bool,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_indices(indices, count, &mut work)?;
        if destination.len() < count {
            return Err(Error::BufferTooSmall);
        }
        let table_len = buckets(count)?;
        workspace.prepare([table_len, 0, 0, 0], 0)?;
        generate::<OUTLINE>(
            &mut destination[..count],
            indices,
            &mut workspace.integers[..table_len],
            hash,
            equal,
            canonical,
            &mut work,
        )
    })();
    workspace.finish(&work);
    result
}
/// Caller-buffer numerical position remap. The tail is preserved; work exhaustion can leave partial output.
pub fn generate_position_remap_into(
    destination: &mut [u32],
    positions: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    if let Some(p) = positions.packed_values() {
        return remap_into::<false>(
            destination,
            None,
            p.len(),
            |i| position_value_hash(p[i as usize]),
            |a, b| p[a as usize] == p[b as usize],
            true,
            workspace,
        )
        .map(|_| ());
    }
    remap_into::<false>(
        destination,
        None,
        positions.len(),
        |i| position_hash(positions, i),
        |a, b| positions.get(a as usize) == positions.get(b as usize),
        true,
        workspace,
    )?;
    Ok(())
}
/// Caller-buffer custom numbering with exact callback traversal. Work exhaustion can leave partial output.
pub fn generate_vertex_remap_custom_into(
    destination: &mut [u32],
    indices: Option<&[u32]>,
    positions: Positions<'_>,
    mut equal: impl FnMut(u32, u32) -> bool,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    if let Some(p) = positions.packed_values() {
        return remap_into::<true>(
            destination,
            indices,
            p.len(),
            |i| position_value_hash(p[i as usize]),
            |a, b| p[a as usize] == p[b as usize] && custom_equal(&mut equal, a, b),
            false,
            workspace,
        );
    }
    remap_into::<true>(
        destination,
        indices,
        positions.len(),
        |i| position_hash(positions, i),
        |a, b| {
            positions.get(a as usize) == positions.get(b as usize) && custom_equal(&mut equal, a, b)
        },
        false,
        workspace,
    )
}
/// Write provoking topology and used reorder prefix. Requires vertex-count plus triangle-count reorder capacity; work exhaustion can partially modify both outputs.
pub fn generate_provoking_index_buffer_into(
    destination: &mut [u32],
    reorder: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    Ok(provoking(
        indices,
        vertex_count,
        Some((destination, reorder)),
        workspace,
    )?
    .1)
}
