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

/// Analyze a triangle list using upstream's simplified FIFO transform cache.
pub fn analyze_vertex_cache(
    indices: &[u32],
    vertex_count: usize,
    cache_size: u32,
    warp_size: u32,
    primgroup_size: u32,
    workspace: &mut Workspace,
) -> Result<VertexCacheStatistics, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if cache_size < 3 || (warp_size != 0 && warp_size < 3) {
            return Err(Error::InvalidParameter);
        }
        topology(indices, vertex_count, &mut work)?;
        checked_bytes(vertex_count, 4)?;
        workspace.prepare([vertex_count, 0, 0, 0], 0)?;
        let timestamps = &mut workspace.integers[..vertex_count];
        timestamps.fill(0);
        let mut result = VertexCacheStatistics::default();
        let mut timestamp = cache_size.wrapping_add(1);
        let (mut warp_offset, mut primgroup_offset) = (0u32, 0u32);
        for triangle in indices.as_chunks::<3>().0 {
            work.add(3)?;
            let misses = triangle
                .iter()
                .filter(|&&v| timestamp.wrapping_sub(timestamps[v as usize]) > cache_size)
                .count() as u32;
            if (primgroup_size != 0 && primgroup_offset == primgroup_size)
                || (warp_size != 0 && warp_offset + misses > warp_size)
            {
                result.warps_executed += u32::from(warp_offset > 0);
                warp_offset = 0;
                primgroup_offset = 0;
                timestamp = timestamp.wrapping_add(cache_size).wrapping_add(1);
            }
            for &index in triangle {
                let entry = &mut timestamps[index as usize];
                if timestamp.wrapping_sub(*entry) > cache_size {
                    *entry = timestamp;
                    timestamp = timestamp.wrapping_add(1);
                    result.vertices_transformed = result
                        .vertices_transformed
                        .checked_add(1)
                        .ok_or(Error::SizeOverflow)?;
                    warp_offset += 1;
                }
            }
            primgroup_offset += 1;
        }
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
    })();
    workspace.finish(&work);
    result
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
        work.scan(indices.iter().copied(), |index| {
            if index as usize >= vertex_count {
                Err(Error::IndexOutOfBounds)
            } else {
                Ok(())
            }
        })?;
        workspace.prepare([0, 0, vertex_count, 0], 0)?;
        let visited = &mut workspace.flags[..vertex_count];
        visited.fill(0);
        let mut cache = [0usize; 2048];
        let mut result = VertexFetchStatistics::default();
        for &index in indices {
            work.add(1)?;
            visited[index as usize] = 1;
            let start = index as usize * vertex_size;
            let end = start + vertex_size;
            for tag in start / 64..end.div_ceil(64) {
                work.add(1)?;
                let line = &mut cache[tag % 2048];
                if *line != tag + 1 {
                    result.bytes_fetched = result
                        .bytes_fetched
                        .checked_add(64)
                        .ok_or(Error::SizeOverflow)?;
                }
                *line = tag + 1;
            }
        }
        let unique = visited.iter().filter(|&&value| value != 0).count();
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
