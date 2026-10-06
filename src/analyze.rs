//! Scalar cache and fetch analyzers from pinned `indexanalyzer.cpp`.
use crate::workspace::{checked_bytes, topology};
use crate::{Error, Workspace};

/// Result of upstream `meshopt_analyzeVertexCache`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VertexCacheStatistics {
    /// Number of vertex shader transforms.
    pub vertices_transformed: u32,
    /// Number of nonempty simulated warps.
    pub warps_executed: u32,
    /// Transforms divided by triangle count.
    pub acmr: f32,
    /// Transforms divided by unique vertex count.
    pub atvr: f32,
}

/// Result of upstream `meshopt_analyzeVertexFetch`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VertexFetchStatistics {
    /// Bytes read by the simulated cache.
    pub bytes_fetched: u32,
    /// Bytes fetched divided by referenced vertex bytes.
    pub overfetch: f32,
}

#[allow(clippy::too_many_arguments)]
fn cache_run<const WARP: bool, const GROUP: bool, const CHARGE: bool>(
    indices: &[u32],
    timestamps: &mut [u32],
    cache_size: u32,
    warp_size: u32,
    primgroup_size: u32,
    work: &mut crate::workspace::Work,
) -> Result<(VertexCacheStatistics, u32), Error> {
    let mut result = VertexCacheStatistics::default();
    let mut timestamp = cache_size.wrapping_add(1);
    let (mut warp_offset, mut primgroup_offset) = (0u32, 0u32);
    for triangle in indices.as_chunks::<3>().0 {
        if CHARGE {
            work.add(3)?;
        }
        let [a, b, c] = *triangle;
        let misses = if WARP {
            u32::from(timestamp.wrapping_sub(timestamps[a as usize]) > cache_size)
                + u32::from(timestamp.wrapping_sub(timestamps[b as usize]) > cache_size)
                + u32::from(timestamp.wrapping_sub(timestamps[c as usize]) > cache_size)
        } else {
            0
        };
        if (GROUP && primgroup_offset == primgroup_size)
            || (WARP && warp_offset + misses > warp_size)
        {
            result.warps_executed += u32::from(warp_offset > 0);
            warp_offset = 0;
            primgroup_offset = 0;
            timestamp = timestamp.wrapping_add(cache_size).wrapping_add(1);
        }
        for index in [a, b, c] {
            let entry = &mut timestamps[index as usize];
            if timestamp.wrapping_sub(*entry) > cache_size {
                *entry = timestamp;
                timestamp = timestamp.wrapping_add(1);
                // topology limits the index stream to u32::MAX entries;
                // one entry can trigger at most one transform here.
                result.vertices_transformed += 1;
                warp_offset += 1;
            }
        }
        if GROUP {
            primgroup_offset += 1;
        }
    }
    Ok((result, warp_offset))
}

#[inline(always)]
fn cache_simple(
    indices: &[u32],
    timestamps: &mut [u32],
    cache_size: u32,
) -> (VertexCacheStatistics, u32) {
    let mut result = VertexCacheStatistics::default();
    let mut timestamp = cache_size.wrapping_add(1);
    for &index in indices {
        let entry = &mut timestamps[index as usize];
        if timestamp.wrapping_sub(*entry) > cache_size {
            *entry = timestamp;
            timestamp = timestamp.wrapping_add(1);
            result.vertices_transformed += 1;
        }
    }
    let present = u32::from(result.vertices_transformed > 0);
    (result, present)
}

/// Analyze a triangle list using upstream's simplified FIFO transform cache.
#[inline(always)]
pub fn analyze_vertex_cache(
    indices: &[u32],
    vertex_count: usize,
    cache_size: u32,
    warp_size: u32,
    primgroup_size: u32,
    workspace: &mut Workspace,
) -> Result<VertexCacheStatistics, Error> {
    let mut work = workspace.begin();
    let result = vertex_cache_impl(
        indices,
        vertex_count,
        cache_size,
        warp_size,
        primgroup_size,
        workspace,
        &mut work,
    );
    workspace.finish(&work);
    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn vertex_cache_impl(
    indices: &[u32],
    vertex_count: usize,
    cache_size: u32,
    warp_size: u32,
    primgroup_size: u32,
    workspace: &mut Workspace,
    work: &mut crate::workspace::Work,
) -> Result<VertexCacheStatistics, Error> {
    if cache_size < 3 || (warp_size != 0 && warp_size < 3) {
        return Err(Error::InvalidParameter);
    }
    topology(indices, vertex_count, work)?;
    checked_bytes(vertex_count, 4)?;
    let retained = workspace.integers.len().min(vertex_count);
    workspace.prepare_timestamps(vertex_count)?;
    let timestamps = &mut workspace.integers[..vertex_count];
    // prepare zero-initializes newly added entries; only retained values
    // can contain timestamps from a previous call.
    timestamps[..retained].fill(0);
    let bulk_work = work.covers(indices.len())?;
    if bulk_work {
        work.add(indices.len())?;
    }
    let (mut result, warp_offset) = match (warp_size != 0, primgroup_size != 0, bulk_work) {
        (false, false, false) => cache_run::<false, false, true>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (false, false, true) => cache_simple(indices, timestamps, cache_size),
        (false, true, false) => cache_run::<false, true, true>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (false, true, true) => cache_run::<false, true, false>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (true, false, false) => cache_run::<true, false, true>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (true, false, true) => cache_run::<true, false, false>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (true, true, false) => cache_run::<true, true, true>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
        (true, true, true) => cache_run::<true, true, false>(
            indices,
            timestamps,
            cache_size,
            warp_size,
            primgroup_size,
            work,
        )?,
    };
    let unique = timestamps.iter().filter(|&&value| value > 0).count();
    work.add(vertex_count)?;
    result.warps_executed += u32::from(warp_offset > 0);
    result.acmr = if indices.is_empty() {
        0.0
    } else {
        result.vertices_transformed as f32 / (indices.len() / 3) as f32
    };
    result.atvr = if unique == 0 {
        0.0
    } else {
        result.vertices_transformed as f32 / unique as f32
    };
    Ok(result)
}

// With <=64-byte vertices and the proven fuel/u32 bounds, an index
// touches one or two lines. Avoid a general range loop for that common case.
fn fetch_small(
    indices: &[u32],
    visited: &mut [u8],
    vertex_size: usize,
    cache: &mut [usize; 2048],
    work: &mut crate::workspace::Work,
) -> Result<u32, Error> {
    let mut bytes_fetched = 0;
    let mut extra_lines = 0;
    for &index in indices {
        visited[index as usize] = 1;
        let start = index as usize * vertex_size;
        let tag = start / 64;
        let first = &mut cache[tag % 2048];
        bytes_fetched += u32::from(*first != tag + 1) * 64;
        *first = tag + 1;
        if (start & 63) + vertex_size > 64 {
            let second = &mut cache[(tag + 1) % 2048];
            bytes_fetched += u32::from(*second != tag + 2) * 64;
            *second = tag + 2;
            extra_lines += 1;
        }
    }
    // The caller proved 6*len visits fit, so this <=3*len count cannot overflow.
    work.add(indices.len() * 2 + extra_lines)?;
    Ok(bytes_fetched)
}

fn fetch_run<const CHARGE: bool, const CHECK: bool>(
    indices: &[u32],
    visited: &mut [u8],
    vertex_size: usize,
    cache: &mut [usize; 2048],
    work: &mut crate::workspace::Work,
) -> Result<u32, Error> {
    let mut bytes_fetched = 0u32;
    let mut visits = if CHECK { 0 } else { indices.len() };
    for &index in indices {
        if CHARGE {
            work.add(1)?;
        }
        if CHECK {
            visits += 1;
        }
        visited[index as usize] = 1;
        let start = index as usize * vertex_size;
        let end = start + vertex_size;
        // The validated vertex byte extent is at most isize::MAX, so
        // adding 63 cannot overflow usize on either supported word size.
        let start_tag = start / 64;
        #[allow(clippy::manual_div_ceil)] // Proven non-overflowing add has cheaper codegen.
        let end_tag = (end + 63) / 64;
        if !CHECK {
            visits += end_tag - start_tag;
        }
        for tag in start_tag..end_tag {
            if CHARGE {
                work.add(1)?;
            }
            if CHECK {
                visits += 1;
            }
            let line = &mut cache[tag % 2048];
            if CHECK {
                if *line != tag + 1 {
                    bytes_fetched = match bytes_fetched.checked_add(64) {
                        Some(bytes) => bytes,
                        None => {
                            if !CHARGE {
                                work.add(visits)?;
                            }
                            return Err(Error::SizeOverflow);
                        }
                    };
                }
            } else {
                bytes_fetched += u32::from(*line != tag + 1) * 64;
            }
            *line = tag + 1;
        }
    }
    if !CHARGE {
        work.add(visits)?;
    }
    Ok(bytes_fetched)
}

/// Analyze index order with upstream's direct-mapped 128 KiB fetch-cache model.
pub fn analyze_vertex_fetch(
    indices: &[u32],
    vertex_count: usize,
    vertex_size: usize,
    workspace: &mut Workspace,
) -> Result<VertexFetchStatistics, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !(1..=256).contains(&vertex_size) {
            return Err(Error::InvalidParameter);
        }
        if vertex_count > u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        checked_bytes(vertex_count, vertex_size)?;
        work.indices(indices, vertex_count)?;
        let retained = workspace.flags.len().min(vertex_count);
        workspace.prepare([0, 0, vertex_count, 0], 0)?;
        let visited = &mut workspace.flags[..vertex_count];
        visited[..retained].fill(0);
        let mut cache = [0usize; 2048];
        let mut result = VertexFetchStatistics::default();
        // An index visits at most five cache lines. With sufficient fuel,
        // count the visits locally and charge once; the fallback keeps the
        // exact exhaustion point and partial usage for low limits.
        let bulk_work = match indices.len().checked_mul(6) {
            Some(max_visits) => work.covers(max_visits)?,
            None => false,
        };
        // A vertex spans at most five 64-byte lines. This upper bound proves
        // u32 accumulation safe without weakening the overflow error contract.
        let bounded_bytes = indices.len() <= u32::MAX as usize / (5 * 64);
        result.bytes_fetched = if bulk_work && bounded_bytes && vertex_size <= 64 {
            fetch_small(indices, visited, vertex_size, &mut cache, &mut work)?
        } else {
            match (bulk_work, bounded_bytes) {
                (true, true) => {
                    fetch_run::<false, false>(indices, visited, vertex_size, &mut cache, &mut work)?
                }
                (true, false) => {
                    fetch_run::<false, true>(indices, visited, vertex_size, &mut cache, &mut work)?
                }
                (false, true) => {
                    fetch_run::<true, false>(indices, visited, vertex_size, &mut cache, &mut work)?
                }
                (false, false) => {
                    fetch_run::<true, true>(indices, visited, vertex_size, &mut cache, &mut work)?
                }
            }
        };
        // Every visited flag was initialized to zero and is written only as one.
        let unique: usize = visited.iter().map(|&value| usize::from(value)).sum();
        work.add(vertex_count)?;
        result.overfetch = if unique == 0 {
            0.0
        } else {
            result.bytes_fetched as f32 / (unique * vertex_size) as f32
        };
        Ok(result)
    })();
    workspace.finish(&work);
    result
}
