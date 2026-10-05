//! Opacity micromap generation translated from the pinned scalar reference.
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Workspace};
use alloc::vec::Vec;

/// Output of upstream `meshopt_opacityMapMeasure`.
#[derive(Clone, Debug, PartialEq)]
pub struct OpacityMapMeasure {
    /// Subdivision level for each unique entry.
    pub levels: Vec<u8>,
    /// Source triangle for each unique entry.
    pub sources: Vec<u32>,
    /// Entry index for each input triangle.
    pub omm_indices: Vec<i32>,
}

/// Packed byte count for one micromap, corresponding to `meshopt_opacityMapEntrySize`.
#[inline(always)]
pub fn opacity_map_entry_size(level: u8, states: u8) -> Result<usize, Error> {
    if level > 12 || (states != 2 && states != 4) {
        return Err(Error::InvalidParameter);
    }
    Ok(((1usize << (usize::from(level) * 2)) * usize::from(states / 2) + 7) >> 3)
}

fn table_size(count: usize) -> Result<usize, Error> {
    let target = count.checked_add(count / 4).ok_or(Error::SizeOverflow)?;
    target
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)
}

fn reserve<T>(count: usize) -> Result<Vec<T>, Error> {
    checked_bytes(count, core::mem::size_of::<T>())?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    Ok(value)
}

#[inline(always)]
fn hash_update(mut h: u32, values: &[i32]) -> u32 {
    for &value in values {
        let mut k = value as u32;
        k = k.wrapping_mul(0x5bd1e995);
        k ^= k >> 24;
        k = k.wrapping_mul(0x5bd1e995);
        h = h.wrapping_mul(0x5bd1e995) ^ k;
    }
    h
}

fn hash_bytes(mut h: u32, key: &[u8]) -> u32 {
    if key.len() < 4 {
        h ^= u32::from(key[0]) | (u32::from(key[key.len() - 1]) << 8);
    } else {
        for bytes in key.as_chunks::<4>().0 {
            let mut k = u32::from_le_bytes(*bytes);
            k = k.wrapping_mul(0x5bd1e995);
            k ^= k >> 24;
            k = k.wrapping_mul(0x5bd1e995);
            h = h.wrapping_mul(0x5bd1e995) ^ k;
        }
    }
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1e995);
    h ^ (h >> 15)
}

fn quantize(v: f32, size: u32) -> i32 {
    (v * ((size * 4) as f32) + if v >= 0.0 { 0.5 } else { -0.5 }) as i32
}

#[inline]
fn square_root(value: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        value.sqrt()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::sqrtf(value)
    }
}

// Scalar bit rounding follows libm's MIT-licensed generic floor (musl).
// Preserve the existing feature-specific handling of non-finite values.
#[inline(always)]
fn floor(value: f32) -> f32 {
    let bits = value.to_bits();
    let exponent = ((bits >> 23) & 255) as i32 - 127;
    if exponent >= 23 {
        if exponent == 128 {
            #[cfg(feature = "std")]
            {
                return value.floor();
            }
            #[cfg(not(feature = "std"))]
            {
                return libm::floorf(value);
            }
        }
        return value;
    }
    if exponent < 0 {
        return if bits >> 31 == 0 {
            0.0
        } else if bits << 1 != 0 {
            -1.0
        } else {
            value
        };
    }
    let mask = 0x007f_ffff >> exponent;
    if bits & mask == 0 {
        return value;
    }
    let rounded = if bits >> 31 != 0 { bits + mask } else { bits };
    f32::from_bits(rounded & !mask)
}

#[inline]
fn log2(value: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        value.log2()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::log2f(value)
    }
}

trait UvReader: Copy {
    fn read(self, index: usize) -> [f32; 2];
}
impl UvReader for &[[f32; 2]] {
    #[inline(always)]
    fn read(self, index: usize) -> [f32; 2] {
        self[index]
    }
}
#[derive(Clone, Copy)]
struct InterleavedUvs<'a> {
    data: &'a [f32],
    stride: usize,
}
impl UvReader for InterleavedUvs<'_> {
    #[inline(always)]
    fn read(self, index: usize) -> [f32; 2] {
        let pair = &self.data[index * self.stride..][..2];
        [pair[0], pair[1]]
    }
}

#[allow(clippy::too_many_arguments)]
fn measure_ranges(
    indices: &[u32],
    uvs: impl UvReader,
    texture_width: u32,
    texture_height: u32,
    max_level: u8,
    target_edge: f32,
    table: &mut [u32],
    keys: &mut Vec<([i32; 6], u8)>,
    levels: &mut Vec<u8>,
    sources: &mut Vec<u32>,
    omm_indices: &mut Vec<i32>,
    work: &mut Work,
) -> Result<(), Error> {
    let buckets = table.len();
    let covers = |count: usize, work: &Work| -> Result<bool, Error> {
        match count.checked_mul(buckets.saturating_add(1)) {
            Some(bound) => work.covers(bound),
            None => Ok(false),
        }
    };
    if covers(indices.len() / 3, work)? {
        return measure_triangles::<false>(
            indices,
            uvs,
            0,
            texture_width,
            texture_height,
            max_level,
            target_edge,
            table,
            keys,
            levels,
            sources,
            omm_indices,
            work,
        );
    }
    // Prove each small range independently; preserve exact per-probe fallback
    // when remaining fuel cannot cover its worst-case bound.
    for (range, slice) in indices.chunks(128 * 3).enumerate() {
        if covers(slice.len() / 3, work)? {
            measure_triangles::<false>(
                slice,
                uvs,
                range * 128,
                texture_width,
                texture_height,
                max_level,
                target_edge,
                table,
                keys,
                levels,
                sources,
                omm_indices,
                work,
            )?;
        } else {
            measure_triangles::<true>(
                slice,
                uvs,
                range * 128,
                texture_width,
                texture_height,
                max_level,
                target_edge,
                table,
                keys,
                levels,
                sources,
                omm_indices,
                work,
            )?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn measure_triangles<const CHARGE: bool>(
    indices: &[u32],
    uvs: impl UvReader,
    source_base: usize,
    texture_width: u32,
    texture_height: u32,
    max_level: u8,
    target_edge: f32,
    table: &mut [u32],
    keys: &mut Vec<([i32; 6], u8)>,
    levels: &mut Vec<u8>,
    sources: &mut Vec<u32>,
    omm_indices: &mut Vec<i32>,
    work: &mut Work,
) -> Result<(), Error> {
    let buckets = table.len();
    let mut visits = 0usize;
    let area = texture_width as f32 * texture_height as f32;
    for (i, triangle) in indices.as_chunks::<3>().0.iter().enumerate() {
        if CHARGE {
            work.add(1)?;
        } else {
            visits += 1;
        }
        let a = uvs.read(triangle[0] as usize);
        let b = uvs.read(triangle[1] as usize);
        let c = uvs.read(triangle[2] as usize);
        let uv = [[a[0], a[1]], [b[0], b[1]], [c[0], c[1]]];
        if !a[0].is_finite()
            || !a[1].is_finite()
            || !b[0].is_finite()
            || !b[1].is_finite()
            || !c[0].is_finite()
            || !c[1].is_finite()
        {
            if !CHARGE {
                work.add(visits)?;
            }
            return Err(Error::NumericalFailure);
        }
        let mut level = max_level;
        if target_edge > 0.0 {
            let uvarea = ((uv[1][0] - uv[0][0]) * (uv[2][1] - uv[0][1])
                - (uv[2][0] - uv[0][0]) * (uv[1][1] - uv[0][1]))
                .abs()
                * 0.5
                * area;
            let ratio = square_root(uvarea) / target_edge;
            let levelf = log2(ratio.max(1.0));
            level = ((levelf + 0.5) as u8).min(max_level);
        }
        let key = (
            [
                quantize(uv[0][0], texture_width),
                quantize(uv[0][1], texture_height),
                quantize(uv[1][0], texture_width),
                quantize(uv[1][1], texture_height),
                quantize(uv[2][0], texture_width),
                quantize(uv[2][1], texture_height),
            ],
            level,
        );
        let candidate = keys.len() as u32;
        let mut bucket = hash_update(level as u32, &key.0) as usize & (buckets - 1);
        let mut found = None;
        for probe in 0..buckets {
            if CHARGE {
                work.add(1)?;
            } else {
                visits += 1;
            }
            let entry = table[bucket];
            if entry == u32::MAX {
                table[bucket] = candidate;
                break;
            }
            if keys[entry as usize].1 == key.1 && keys[entry as usize].0 == key.0 {
                found = Some(entry);
                break;
            }
            bucket = (bucket + probe + 1) & (buckets - 1);
        }
        if let Some(existing) = found {
            omm_indices.push(existing as i32);
        } else {
            keys.push(key);
            levels.push(level);
            sources.push((source_base + i) as u32);
            omm_indices.push(candidate as i32);
        }
    }
    if !CHARGE {
        work.add(visits)?;
    }
    Ok(())
}

/// Deduplicate triangles by quantized UV and choose adaptive OMM levels.
///
/// `uvs` is an interleaved float view with `uv_stride` floats per vertex.
/// It reproduces upstream `meshopt_opacityMapMeasure` for finite inputs.
#[allow(clippy::too_many_arguments)]
pub fn opacity_map_measure(
    indices: &[u32],
    uvs: &[f32],
    vertex_count: usize,
    uv_stride: usize,
    texture_width: u32,
    texture_height: u32,
    max_level: u8,
    target_edge: f32,
    workspace: &mut Workspace,
) -> Result<OpacityMapMeasure, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if !indices.len().is_multiple_of(3) {
            return Err(Error::InvalidTopology);
        }
        if !(2..=64).contains(&uv_stride)
            || texture_width == 0
            || texture_width > 16384
            || texture_height == 0
            || texture_height > 16384
            || max_level > 12
            || !target_edge.is_finite()
            || target_edge < 0.0
        {
            return Err(Error::InvalidParameter);
        }
        if vertex_count > u32::MAX as usize || indices.len() / 3 > i32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        let needed = vertex_count
            .checked_sub(1)
            .and_then(|v| v.checked_mul(uv_stride))
            .and_then(|v| v.checked_add(2))
            .unwrap_or(0);
        if uvs.len() < needed {
            return Err(Error::InvalidLayout);
        }
        work.indices(indices, vertex_count)?;
        let count = indices.len() / 3;
        let buckets = table_size(count)?;
        let bytes = count
            .checked_mul(3 * 4 + 6 * 4 + 4 + 4)
            .and_then(|n| n.checked_add(buckets * 4))
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(bytes)?;
        let mut table = reserve::<u32>(buckets)?;
        table.resize(buckets, u32::MAX);
        let mut keys = reserve::<([i32; 6], u8)>(count)?;
        let mut levels = reserve::<u8>(count)?;
        let mut sources = reserve::<u32>(count)?;
        let mut omm_indices = reserve::<i32>(count)?;
        if uv_stride == 2 {
            measure_ranges(
                indices,
                uvs.as_chunks::<2>().0,
                texture_width,
                texture_height,
                max_level,
                target_edge,
                &mut table,
                &mut keys,
                &mut levels,
                &mut sources,
                &mut omm_indices,
                &mut work,
            )?;
        } else {
            measure_ranges(
                indices,
                InterleavedUvs {
                    data: uvs,
                    stride: uv_stride,
                },
                texture_width,
                texture_height,
                max_level,
                target_edge,
                &mut table,
                &mut keys,
                &mut levels,
                &mut sources,
                &mut omm_indices,
                &mut work,
            )?;
        }
        Ok(OpacityMapMeasure {
            levels,
            sources,
            omm_indices,
        })
    })();
    workspace.finish(&work);
    result
}

/// Caller-buffer form of `meshopt_opacityMapMeasure`; returns unique entry count.
/// The three destinations are unchanged when validation fails.
#[allow(clippy::too_many_arguments)]
pub fn opacity_map_measure_into(
    levels: &mut [u8],
    sources: &mut [u32],
    omm_indices: &mut [i32],
    indices: &[u32],
    uvs: &[f32],
    vertex_count: usize,
    uv_stride: usize,
    texture_width: u32,
    texture_height: u32,
    max_level: u8,
    target_edge: f32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let triangles = indices.len() / 3;
    if levels.len() < triangles || sources.len() < triangles || omm_indices.len() < triangles {
        return Err(Error::BufferTooSmall);
    }
    let result = opacity_map_measure(
        indices,
        uvs,
        vertex_count,
        uv_stride,
        texture_width,
        texture_height,
        max_level,
        target_edge,
        workspace,
    )?;
    let count = result.levels.len();
    levels[..count].copy_from_slice(&result.levels);
    sources[..count].copy_from_slice(&result.sources);
    omm_indices[..triangles].copy_from_slice(&result.omm_indices);
    Ok(count)
}

fn special_index(data: &[u8], level: u8, states: u8) -> i32 {
    let first = i32::from(data[0] & if states == 2 { 1 } else { 3 });
    let special = -(1 + first);
    if level == 0 {
        return special;
    }
    if level == 1 && states == 2 {
        return if i32::from(data[0] & 15) == ((-first) & 15) {
            special
        } else {
            0
        };
    }
    let expected = first * if states == 2 { 0xff } else { 0x55 };
    if data.iter().all(|&v| i32::from(v) == expected) {
        special
    } else {
        0
    }
}

/// Compact OMM data in place, as upstream `meshopt_opacityMapCompact` does.
///
/// Returns the remaining entry count and used byte count. Callers may trim the
/// arrays to those lengths. Existing negative triangle references are retained.
pub fn opacity_map_compact(
    data: &mut [u8],
    levels: &mut [u8],
    offsets: &mut [u32],
    omm_indices: &mut [i32],
    states: u8,
    workspace: &mut Workspace,
) -> Result<(usize, usize), Error> {
    let mut work = workspace.begin();
    let result = (|| {
        if states != 2 && states != 4 {
            return Err(Error::InvalidParameter);
        }
        if levels.len() != offsets.len() || levels.len() > i32::MAX as usize {
            return Err(Error::InvalidLayout);
        }
        let count = levels.len();
        let buckets = table_size(count)?;
        for (&level, &offset) in levels.iter().zip(offsets.iter()) {
            work.add(1)?;
            let size = opacity_map_entry_size(level, states)?;
            let end = (offset as usize)
                .checked_add(size)
                .ok_or(Error::SizeOverflow)?;
            if end > data.len() {
                return Err(Error::InvalidLayout);
            }
        }
        for &index in omm_indices.iter() {
            work.add(1)?;
            if index >= count as i32 {
                return Err(Error::IndexOutOfBounds);
            }
        }
        let bytes = data
            .len()
            .checked_add(buckets * 4)
            .and_then(|n| n.checked_add(count * 4))
            .ok_or(Error::SizeOverflow)?;
        workspace.account_codec(bytes)?;
        let mut old = reserve::<u8>(data.len())?;
        old.extend_from_slice(data);
        let mut table = reserve::<u32>(buckets)?;
        table.resize(buckets, u32::MAX);
        let mut remap = reserve::<i32>(count)?;
        remap.resize(count, 0);
        let (mut next, mut output_offset) = (0usize, 0usize);
        for i in 0..count {
            work.add(1)?;
            let level = levels[i];
            let size = opacity_map_entry_size(level, states)?;
            let source = &old[offsets[i] as usize..offsets[i] as usize + size];
            let special = special_index(source, level, states);
            if special < 0 {
                remap[i] = special;
                continue;
            }
            if output_offset.checked_add(size).ok_or(Error::SizeOverflow)? > data.len() {
                return Err(Error::BufferTooSmall);
            }
            data[output_offset..output_offset + size].copy_from_slice(source);
            offsets[next] = output_offset as u32;
            levels[next] = level;
            let candidate = next as u32;
            let mut bucket =
                hash_bytes(u32::from(level), &data[output_offset..output_offset + size]) as usize
                    & (buckets - 1);
            let mut found = None;
            for probe in 0..buckets {
                work.add(1)?;
                let entry = table[bucket];
                if entry == u32::MAX {
                    table[bucket] = candidate;
                    break;
                }
                let other = offsets[entry as usize] as usize;
                if levels[entry as usize] == level
                    && data[other..other + size] == data[output_offset..output_offset + size]
                {
                    found = Some(entry);
                    break;
                }
                bucket = (bucket + probe + 1) & (buckets - 1);
            }
            if let Some(existing) = found {
                remap[i] = existing as i32;
            } else {
                remap[i] = candidate as i32;
                next += 1;
                output_offset += size;
            }
        }
        for index in omm_indices.iter_mut() {
            if *index >= 0 {
                *index = remap[*index as usize];
            }
        }
        Ok((next, output_offset))
    })();
    workspace.finish(&work);
    result
}

#[inline(always)]
fn sample(
    data: &[u8],
    stride: usize,
    pitch: usize,
    width: u32,
    height: u32,
    u: f32,
    v: f32,
) -> f32 {
    let u = if (u - 0.5).abs() > 0.5 {
        u - floor(u)
    } else {
        u
    };
    let v = if (v - 0.5).abs() > 0.5 {
        v - floor(v)
    } else {
        v
    };
    let uf = ((u * (width as f32 * 256.0) - 127.5) as i32).max(0);
    let vf = ((v * (height as f32 * 256.0) - 127.5) as i32).max(0);
    let (x, y, rx, ry) = (uf >> 8, vf >> 8, uf & 255, vf & 255);
    if x as u32 >= width || y as u32 >= height {
        return 0.0;
    }
    let offset = y as usize * pitch + x as usize * stride;
    let ox = if x as u32 + 1 < width { stride } else { 0 };
    let oy = if y as u32 + 1 < height { pitch } else { 0 };
    let (a00, a10, a01, a11) = (
        data[offset] as i32,
        data[offset + ox] as i32,
        data[offset + oy] as i32,
        data[offset + ox + oy] as i32,
    );
    let ax0 = a00 * 256 + (a10 - a00) * rx;
    let ax1 = a01 * 256 + (a11 - a01) * rx;
    let axy = ax0 * 256 + (ax1 - ax0) * ry;
    axy as f32 * (1.0 / (255.0 * 65536.0))
}

#[inline(always)]
fn edge(texture: &Texture<'_>, a: [f32; 3], b: [f32; 3], resolution: i32) -> i32 {
    let step = 1.0 / (resolution + 1) as f32;
    let (du, dv) = ((b[0] - a[0]) * step, (b[1] - a[1]) * step);
    let (mut u, mut v) = (a[0], a[1]);
    let (mut mask, mut count) = (0, 0);
    for i in 0..resolution {
        u += du;
        v += dv;
        let alpha = texture.sample(u, v);
        mask |= i32::from(alpha >= 0.5) << i;
        count += i32::from(alpha >= 0.5);
    }
    mask | (count << 16)
}

struct Texture<'a> {
    data: &'a [u8],
    stride: usize,
    pitch: usize,
    width: u32,
    height: u32,
}
impl Texture<'_> {
    #[inline(always)]
    fn sample(&self, u: f32, v: f32) -> f32 {
        sample(
            self.data,
            self.stride,
            self.pitch,
            self.width,
            self.height,
            u,
            v,
        )
    }
}

fn emit<const STATES: u8>(
    result: &mut [u8],
    index: usize,
    points: [[f32; 3]; 3],
    center: f32,
    edges: [i32; 3],
    edgeres: i32,
) {
    let (a0, a1, a2) = (points[0][2], points[1][2], points[2][2]);
    let mut coverage = (a0 + a1 + a2) * 0.12 + center * 0.64;
    if edgeres != 0 {
        coverage = center * 0.22
            + ((edges[0] >> 16) + (edges[1] >> 16) + (edges[2] >> 16)) as f32
                * (1.0 / edgeres as f32)
                * 0.23
            + (a0 + a1 + a2) * 0.03;
    }
    if STATES == 2 {
        result[index / 8] |= u8::from(coverage >= 0.5) << (index % 8);
        return;
    }
    let transparent = a0 < 0.5 && a1 < 0.5 && a2 < 0.5 && center < 0.5;
    let opaque = a0 > 0.5 && a1 > 0.5 && a2 > 0.5 && center > 0.5;
    let unknown = 2 + i32::from(coverage >= 0.5);
    let mut state = if transparent || opaque {
        i32::from(opaque)
    } else {
        unknown
    };
    if edgeres != 0 && (transparent || opaque) {
        let expected = if opaque { (1 << edgeres) - 1 } else { 0 };
        if edges.iter().any(|e| (e & 0xffff) != expected) {
            state = unknown;
        }
    }
    result[index / 4] |= (state as u8) << ((index % 4) * 2);
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn raster_setup<'a>(
    level: u8,
    uvs: [[f32; 2]; 3],
    texture_data: &'a [u8],
    texture_stride: usize,
    texture_pitch: usize,
    texture_width: u32,
    texture_height: u32,
) -> Result<(Texture<'a>, i32, [[f32; 3]; 3]), Error> {
    if texture_width == 0
        || texture_width > 16384
        || texture_height == 0
        || texture_height > 16384
        || !(1..=4).contains(&texture_stride)
        || texture_pitch < texture_stride * texture_width as usize
        || uvs.iter().flatten().any(|v| !v.is_finite())
    {
        return Err(Error::InvalidParameter);
    }
    let required = (texture_height as usize - 1)
        .checked_mul(texture_pitch)
        .and_then(|n| n.checked_add((texture_width as usize - 1) * texture_stride))
        .and_then(|n| n.checked_add(1))
        .ok_or(Error::SizeOverflow)?;
    if texture_data.len() < required {
        return Err(Error::InvalidLayout);
    }
    let texture = Texture {
        data: texture_data,
        stride: texture_stride,
        pitch: texture_pitch,
        width: texture_width,
        height: texture_height,
    };
    let area = texture_width as f32 * texture_height as f32;
    let uvarea = ((uvs[1][0] - uvs[0][0]) * (uvs[2][1] - uvs[0][1])
        - (uvs[2][0] - uvs[0][0]) * (uvs[1][1] - uvs[0][1]))
        .abs()
        * 0.5
        * area;
    let uvedge = square_root(uvarea) / (1u32 << level) as f32;
    let edgeres = ((uvedge * 0.75) as i32).clamp(0, 7);
    let corners = [
        [uvs[0][0], uvs[0][1], texture.sample(uvs[0][0], uvs[0][1])],
        [uvs[1][0], uvs[1][1], texture.sample(uvs[1][0], uvs[1][1])],
        [uvs[2][0], uvs[2][1], texture.sample(uvs[2][0], uvs[2][1])],
    ];
    Ok((texture, edgeres, corners))
}

fn raster_visits(level: u8) -> usize {
    // Each level has four children. The level-one edge-specialized path
    // charges the same four children without recursing.
    ((1usize << ((usize::from(level) + 1) * 2)) - 1) / 3
}

/// Rasterize one upstream opacity entry using an alpha-channel byte view.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn opacity_map_rasterize(
    level: u8,
    states: u8,
    uvs: [[f32; 2]; 3],
    texture_data: &[u8],
    texture_stride: usize,
    texture_pitch: usize,
    texture_width: u32,
    texture_height: u32,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    let mut work = workspace.begin();
    let result = rasterize_impl(
        level,
        states,
        uvs,
        texture_data,
        texture_stride,
        texture_pitch,
        texture_width,
        texture_height,
        workspace,
        &mut work,
    );
    workspace.finish(&work);
    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn rasterize_impl(
    level: u8,
    states: u8,
    uvs: [[f32; 2]; 3],
    texture_data: &[u8],
    texture_stride: usize,
    texture_pitch: usize,
    texture_width: u32,
    texture_height: u32,
    workspace: &mut Workspace,
    work: &mut crate::workspace::Work,
) -> Result<Vec<u8>, Error> {
    let size = opacity_map_entry_size(level, states)?;
    let (texture, edgeres, corners) = raster_setup(
        level,
        uvs,
        texture_data,
        texture_stride,
        texture_pitch,
        texture_width,
        texture_height,
    )?;
    workspace.account_codec(size)?;
    let mut result = reserve::<u8>(size)?;
    result.resize(size, 0);
    // The state format is passed explicitly through the recursive path.
    raster_state(
        &mut result,
        0,
        level,
        states,
        edgeres,
        corners,
        &texture,
        work,
    )?;
    Ok(result)
}

/// Caller-buffer form of `meshopt_opacityMapRasterize`.
/// Returns the number of bytes written and preserves the destination on error.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn opacity_map_rasterize_into(
    destination: &mut [u8],
    level: u8,
    states: u8,
    uvs: [[f32; 2]; 3],
    texture_data: &[u8],
    texture_stride: usize,
    texture_pitch: usize,
    texture_width: u32,
    texture_height: u32,
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let size = opacity_map_entry_size(level, states)?;
    if destination.len() < size {
        return Err(Error::BufferTooSmall);
    }
    // Preserve the complete destination on a tight work budget by using the
    // staged path; the default budget covers every recursive visit upfront.
    if workspace.limits().max_work < raster_visits(level) as u64 {
        let result = opacity_map_rasterize(
            level,
            states,
            uvs,
            texture_data,
            texture_stride,
            texture_pitch,
            texture_width,
            texture_height,
            workspace,
        )?;
        destination[..size].copy_from_slice(&result);
        return Ok(size);
    }
    let mut work = workspace.begin();
    let result = rasterize_into_impl(
        destination,
        level,
        states,
        uvs,
        texture_data,
        texture_stride,
        texture_pitch,
        texture_width,
        texture_height,
        workspace,
        size,
        &mut work,
    );
    workspace.finish(&work);
    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn rasterize_into_impl(
    destination: &mut [u8],
    level: u8,
    states: u8,
    uvs: [[f32; 2]; 3],
    texture_data: &[u8],
    texture_stride: usize,
    texture_pitch: usize,
    texture_width: u32,
    texture_height: u32,
    workspace: &mut Workspace,
    size: usize,
    work: &mut crate::workspace::Work,
) -> Result<usize, Error> {
    let (texture, edgeres, corners) = raster_setup(
        level,
        uvs,
        texture_data,
        texture_stride,
        texture_pitch,
        texture_width,
        texture_height,
    )?;
    workspace.account_codec(0)?;
    let result = &mut destination[..size];
    result.fill(0);
    raster_state(result, 0, level, states, edgeres, corners, &texture, work)?;
    Ok(size)
}

#[inline(always)]
fn midpoint(a: [f32; 3], b: [f32; 3], texture: &Texture<'_>) -> [f32; 3] {
    let u = (a[0] + b[0]) / 2.0;
    let v = (a[1] + b[1]) / 2.0;
    [u, v, texture.sample(u, v)]
}

#[allow(clippy::too_many_arguments)]
fn raster_state(
    result: &mut [u8],
    index: usize,
    level: u8,
    states: u8,
    edgeres: i32,
    corners: [[f32; 3]; 3],
    texture: &Texture<'_>,
    work: &mut crate::workspace::Work,
) -> Result<(), Error> {
    let visits = raster_visits(level);
    let bulk = work.covers(visits)?;
    if bulk {
        work.add(visits)?;
    }
    match (states == 2, bulk) {
        (true, true) => {
            raster_state_core::<false, 2>(result, index, level, edgeres, corners, texture, work)
        }
        (false, true) => {
            raster_state_core::<false, 4>(result, index, level, edgeres, corners, texture, work)
        }
        (true, false) => {
            raster_state_core::<true, 2>(result, index, level, edgeres, corners, texture, work)
        }
        (false, false) => {
            raster_state_core::<true, 4>(result, index, level, edgeres, corners, texture, work)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn raster_state_core<const CHARGE_WORK: bool, const STATES: u8>(
    result: &mut [u8],
    index: usize,
    level: u8,
    edgeres: i32,
    corners: [[f32; 3]; 3],
    texture: &Texture<'_>,
    work: &mut crate::workspace::Work,
) -> Result<(), Error> {
    if CHARGE_WORK {
        work.add(1)?;
    }
    if level == 0 {
        let center = texture.sample(
            (corners[0][0] + corners[1][0] + corners[2][0]) * (1.0 / 3.0),
            (corners[0][1] + corners[1][1] + corners[2][1]) * (1.0 / 3.0),
        );
        let edges = if edgeres > 0 {
            [
                edge(texture, corners[0], corners[1], edgeres),
                edge(texture, corners[1], corners[2], edgeres),
                edge(texture, corners[2], corners[0], edgeres),
            ]
        } else {
            [0; 3]
        };
        emit::<STATES>(result, index, corners, center, edges, edgeres);
        return Ok(());
    }
    let mid = [
        midpoint(corners[0], corners[1], texture),
        midpoint(corners[1], corners[2], texture),
        midpoint(corners[2], corners[0], texture),
    ];
    if level == 1 && edgeres > 0 {
        let points = [corners[0], mid[0], corners[1], mid[1], corners[2], mid[2]];
        let edge_samples = [
            edge(texture, points[0], points[1], edgeres),
            edge(texture, points[1], points[2], edgeres),
            edge(texture, points[2], points[3], edgeres),
            edge(texture, points[3], points[4], edgeres),
            edge(texture, points[4], points[5], edgeres),
            edge(texture, points[5], points[0], edgeres),
            edge(texture, points[5], points[1], edgeres),
            edge(texture, points[1], points[3], edgeres),
            edge(texture, points[3], points[5], edgeres),
        ];
        let triangles = [
            [0, 1, 5, 0, 6, 5],
            [5, 3, 1, 8, 7, 6],
            [1, 2, 3, 1, 2, 7],
            [3, 5, 4, 8, 4, 3],
        ];
        for (child, [a, b, c, e0, e1, e2]) in triangles.into_iter().enumerate() {
            if CHARGE_WORK {
                work.add(1)?;
            }
            let tri = [points[a], points[b], points[c]];
            let center = texture.sample(
                (tri[0][0] + tri[1][0] + tri[2][0]) * (1.0 / 3.0),
                (tri[0][1] + tri[1][1] + tri[2][1]) * (1.0 / 3.0),
            );
            emit::<STATES>(
                result,
                index * 4 + child,
                tri,
                center,
                [edge_samples[e0], edge_samples[e1], edge_samples[e2]],
                edgeres,
            );
        }
        return Ok(());
    }
    for (child, tri) in [
        [corners[0], mid[0], mid[2]],
        [mid[2], mid[1], mid[0]],
        [mid[0], corners[1], mid[1]],
        [mid[1], mid[2], corners[2]],
    ]
    .into_iter()
    .enumerate()
    {
        raster_state_core::<CHARGE_WORK, STATES>(
            result,
            index * 4 + child,
            level - 1,
            edgeres,
            tri,
            texture,
            work,
        )?;
    }
    Ok(())
}
