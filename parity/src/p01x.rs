//! Version two preprocessing protocol. All observable buffers are little-endian words.
use super::{push, word, MAX};
use meshoptimizer_rs::*;
#[path = "p01x_output.rs"]
mod output;
use output::Payload;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

pub(super) fn execute(input: &[u8], paired: bool) -> Result<Vec<u8>, String> {
    if input.len() < 56 || input.len() > MAX {
        return Err("bad preprocessing length".into());
    }
    let op = word(input, 4)?;
    let vc = word(input, 8)? as usize;
    let ic = word(input, 12)? as usize;
    let mode = word(input, 16)?;
    let samples = word(input, 20)? as usize;
    let size = word(input, 24)? as usize;
    let stride = word(input, 28)? as usize;
    let sc = word(input, 32)? as usize;
    let options = word(input, 36)?;
    let target = word(input, 40)? as usize;
    let p0 = word(input, 44)?;
    let p1 = word(input, 48)?;
    let ac = word(input, 52)? as usize;
    if !(7..=38).contains(&op)
        || mode > 2
        || samples > 100
        || ac > 32
        || sc > 16
        || size == 0
        || size > 256
        || stride < size
        || stride > MAX
        || (mode == 0 && samples != 0)
        || (mode != 0 && samples == 0)
    {
        return Err("invalid preprocessing mode".into());
    }
    if matches!(op, 9 | 12 | 14 | 16 | 22) && sc == 0 {
        return Err("missing vertex stream".into());
    }
    let length = 56usize
        .checked_add(vc.checked_mul(12).ok_or("overflow")?)
        .and_then(|n| n.checked_add(ic.checked_mul(4)?))
        .and_then(|n| n.checked_add(sc.checked_mul(vc)?.checked_mul(stride)?))
        .and_then(|n| n.checked_add(ac.checked_mul(4)?))
        .and_then(|n| n.checked_add(vc.checked_mul(ac)?.checked_mul(4)?))
        .and_then(|n| n.checked_add(vc.checked_mul(4)?))
        .ok_or("overflow")?;
    if length != input.len() {
        return Err("preprocessing payload length".into());
    }
    let mut at = 56;
    let mut positions = Vec::with_capacity(vc);
    for _ in 0..vc {
        positions.push([
            f32::from_bits(word(input, at)?),
            f32::from_bits(word(input, at + 4)?),
            f32::from_bits(word(input, at + 8)?),
        ]);
        at += 12;
    }
    let mut indices = Vec::with_capacity(ic);
    for _ in 0..ic {
        indices.push(word(input, at)?);
        at += 4;
    }
    let mut streams = Vec::new();
    for _ in 0..sc {
        streams.push(
            VertexStream::new(&input[at..at + vc * stride], vc, size, stride, 0)
                .map_err(|e| e.to_string())?,
        );
        at += vc * stride;
    }
    let mut weights = Vec::with_capacity(ac);
    for _ in 0..ac {
        weights.push(f32::from_bits(word(input, at)?));
        at += 4;
    }
    let mut attributes = Vec::with_capacity(vc * ac);
    for _ in 0..vc * ac {
        attributes.push(f32::from_bits(word(input, at)?));
        at += 4;
    }
    let mut flags = Vec::with_capacity(vc);
    for _ in 0..vc {
        flags.push(
            VertexFlags::from_bits(u8::try_from(word(input, at)?).map_err(|_| "flag width")?)
                .map_err(|e| e.to_string())?,
        );
        at += 4;
    }
    if op == 33 && vc != 2 {
        return Err("exponent requires two bounds".into());
    }
    let mut ws = Workspace::default();
    let capacity = match op {
        19 => ic.checked_mul(2),
        20 => ic.checked_mul(4),
        9..=11 | 18 | 23 => Some(vc),
        13 => Some(ic.max(vc)),
        26 => Some(target),
        12 | 22 | 27 | 33 => Some(0),
        _ => Some(ic),
    }
    .ok_or("overflow")?;
    let byte_capacity = if matches!(op, 12 | 22) {
        vc.checked_mul(size).ok_or("overflow")?
    } else {
        0
    };
    if capacity.checked_mul(4).ok_or("overflow")? > MAX || byte_capacity > MAX {
        return Err("transport output limit".into());
    }
    let mut destination = vec![0u32; capacity];
    let mut bytes = vec![0u8; byte_capacity];
    let mut buffer_remap: Vec<_> = (0..vc)
        .map(|i| {
            if op == 12 && i % 5 == 0 {
                u32::MAX
            } else {
                (vc - 1 - i) as u32
            }
        })
        .collect();
    // Output transport length, matching the reference's pre-timing metadata.
    let remapped_bytes = buffer_remap
        .iter()
        .filter(|&&i| i != u32::MAX)
        .max()
        .map_or(0, |&i| i as usize + 1)
        * size;
    let mut colors: Vec<_> = (0..vc)
        .map(|i| [i as f32 % 3.0, i as f32 % 5.0, i as f32 % 7.0])
        .collect();
    if op == 13 && p1 & 2 != 0 {
        if sc != 1 || size != 4 || stride != 4 {
            return Err("explicit remap layout".into());
        }
        for (i, value) in buffer_remap.iter_mut().enumerate() {
            *value = word(input, 56 + vc * 12 + ic * 4 + i * 4)?;
        }
    }
    if op == 26 && p1 & 4 != 0 {
        if ac != 3 {
            return Err("explicit color layout".into());
        }
        for (color, values) in colors.iter_mut().zip(attributes.as_chunks::<3>().0) {
            *color = *values;
        }
    }
    let mut reorder = vec![0u32; vc + ic / 3];
    let mut operation = || -> Result<Payload, Error> {
        let p = Positions::from_packed(&positions);
        let optional = if p1 & 1 != 0 {
            None
        } else {
            Some(indices.as_slice())
        };
        let caller = mode == 2 || options & 0x80000000 != 0;
        let options = SimplifyOptions::from_bits(options & 0x7fffffff)?;
        let settings = SimplifySettings {
            target_index_count: target,
            target_error: f32::from_bits(p0),
            options,
        };
        let output = match op {
            7 => {
                if caller {
                    optimize_vertex_cache_strip_into(&mut destination, &indices, vc, &mut ws)?;
                    Payload::Caller {
                        count: ic,
                        header: None,
                    }
                } else {
                    Payload::Words(optimize_vertex_cache_strip(&indices, vc, &mut ws)?)
                }
            }
            8 => {
                if caller {
                    optimize_vertex_cache_fifo_into(&mut destination, &indices, vc, p0, &mut ws)?;
                    Payload::Caller {
                        count: ic,
                        header: None,
                    }
                } else {
                    Payload::Words(optimize_vertex_cache_fifo(&indices, vc, p0, &mut ws)?)
                }
            }
            9 | 10 => {
                let stream = if op == 9 { &streams[..1] } else { &streams };
                if caller {
                    let count = generate_vertex_remap_multi_into(
                        &mut destination,
                        optional,
                        stream,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count: vc,
                        header: Some(count as u32),
                    }
                } else {
                    Payload::Remap(generate_vertex_remap_multi(optional, stream, &mut ws)?)
                }
            }
            11 => {
                let mut calls = Vec::new();
                let equal = |a: u32, b: u32| {
                    if p1 & 4 == 0 {
                        calls.extend([a, b]);
                    }
                    if p1 & (4 | 8) != 0 {
                        true
                    } else if p1 & 16 != 0 {
                        false
                    } else {
                        a % 3 == b % 3
                    }
                };
                if caller {
                    let count = generate_vertex_remap_custom_into(
                        &mut destination,
                        optional,
                        p,
                        equal,
                        &mut ws,
                    )?;
                    Payload::CallerCustom(count, calls, vc)
                } else {
                    Payload::Custom(
                        generate_vertex_remap_custom(optional, p, equal, &mut ws)?,
                        calls,
                    )
                }
            }
            12 => {
                let remap = &buffer_remap;
                if caller {
                    bytes.fill(0);
                    remap_vertex_buffer_into(&mut bytes, streams[0], remap, &mut ws)?;
                    Payload::CallerBytes(remapped_bytes)
                } else {
                    Payload::Bytes(remap_vertex_buffer(streams[0], remap, &mut ws)?)
                }
            }
            13 => {
                let remap = &buffer_remap;
                if caller {
                    remap_index_buffer_into(&mut destination, optional, remap, &mut ws)?;
                    Payload::Caller {
                        count: optional.map_or(vc, <[u32]>::len),
                        header: None,
                    }
                } else {
                    Payload::Words(remap_index_buffer(optional, remap, &mut ws)?)
                }
            }
            14 => {
                if caller {
                    let count =
                        filter_index_buffer_into(&mut destination, &indices, streams[0], &mut ws)?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(filter_index_buffer(&indices, streams[0], &mut ws)?)
                }
            }
            15 => {
                if caller {
                    let count = filter_index_buffer_multi_into(
                        &mut destination,
                        &indices,
                        &streams,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(filter_index_buffer_multi(&indices, &streams, &mut ws)?)
                }
            }
            16 => {
                if caller {
                    let count = generate_shadow_index_buffer_into(
                        &mut destination,
                        &indices,
                        streams[0],
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(generate_shadow_index_buffer(&indices, streams[0], &mut ws)?)
                }
            }
            17 => {
                if caller {
                    let count = generate_shadow_index_buffer_multi_into(
                        &mut destination,
                        &indices,
                        &streams,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(generate_shadow_index_buffer_multi(
                        &indices, &streams, &mut ws,
                    )?)
                }
            }
            18 => {
                if caller {
                    generate_position_remap_into(&mut destination, p, &mut ws)?;
                    Payload::Caller {
                        count: vc,
                        header: None,
                    }
                } else {
                    Payload::Words(generate_position_remap(p, &mut ws)?)
                }
            }
            19 => {
                if caller {
                    generate_adjacency_index_buffer_into(&mut destination, &indices, p, &mut ws)?;
                    Payload::Caller {
                        count: ic * 2,
                        header: None,
                    }
                } else {
                    Payload::Words(generate_adjacency_index_buffer(&indices, p, &mut ws)?)
                }
            }
            20 => {
                if caller {
                    generate_tessellation_index_buffer_into(
                        &mut destination,
                        &indices,
                        p,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count: ic * 4,
                        header: None,
                    }
                } else {
                    Payload::Words(generate_tessellation_index_buffer(&indices, p, &mut ws)?)
                }
            }
            21 => {
                if caller {
                    let count = generate_provoking_index_buffer_into(
                        &mut destination,
                        &mut reorder,
                        &indices,
                        vc,
                        &mut ws,
                    )?;
                    Payload::CallerProvoking(count, ic)
                } else {
                    Payload::Provoking(generate_provoking_index_buffer(&indices, vc, &mut ws)?)
                }
            }
            22 => {
                if caller {
                    let mut topology = indices.clone();
                    let count = optimize_vertex_fetch_in_place(
                        &mut bytes,
                        &mut topology,
                        streams[0],
                        &mut ws,
                    )?;
                    Payload::CallerFetch(count, topology, count * size)
                } else {
                    Payload::Fetch(optimize_vertex_fetch(&indices, streams[0], &mut ws)?)
                }
            }
            23 => {
                if caller {
                    let count =
                        optimize_vertex_fetch_remap_into(&mut destination, &indices, vc, &mut ws)?;
                    Payload::Caller {
                        count: vc,
                        header: Some(count as u32),
                    }
                } else {
                    Payload::Remap(optimize_vertex_fetch_remap(&indices, vc, &mut ws)?)
                }
            }
            24 => {
                if caller {
                    let r = simplify_sloppy_into(
                        &mut destination,
                        &indices,
                        p,
                        if p1 & 2 != 0 { Some(&flags) } else { None },
                        target,
                        f32::from_bits(p0),
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count: r.index_count,
                        header: Some(r.error.to_bits()),
                    }
                } else {
                    Payload::Simplified(simplify_sloppy(
                        &indices,
                        p,
                        if p1 & 2 != 0 { Some(&flags) } else { None },
                        target,
                        f32::from_bits(p0),
                        &mut ws,
                    )?)
                }
            }
            25 => {
                if caller {
                    let count = simplify_prune_into(
                        &mut destination,
                        &indices,
                        p,
                        f32::from_bits(p0),
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(simplify_prune(&indices, p, f32::from_bits(p0), &mut ws)?)
                }
            }
            26 => {
                let c = if p1 & 2 != 0 {
                    Some(Positions::from_packed(&colors))
                } else {
                    None
                };
                if caller {
                    let count = simplify_points_into(
                        &mut destination,
                        p,
                        c,
                        f32::from_bits(p0),
                        target,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count,
                        header: None,
                    }
                } else {
                    Payload::Words(simplify_points(p, c, f32::from_bits(p0), target, &mut ws)?)
                }
            }
            27 => {
                let mut pp = positions.clone();
                let mut aa = attributes.clone();
                let mut ii = indices.clone();
                let r = simplify_with_update_in_place(
                    &mut ii,
                    &mut PositionsMut::from_packed(&mut pp),
                    Some(&mut AttributesMut::from_interleaved(
                        &mut aa, vc, ac, ac, 0,
                    )?),
                    &weights,
                    Some(&flags),
                    settings,
                    &mut ws,
                )?;
                Payload::Updated(r, ii, pp, aa)
            }
            28..=32 => {
                macro_rules! quantize {
                    ($convert:expr) => {{
                        for (out, &i) in destination[..ic].iter_mut().zip(&indices) {
                            *out = $convert(i)?;
                        }
                    }};
                }
                match op {
                    28 => quantize!(
                        |i| quantize_unorm(f32::from_bits(i), target as u32).map(|v| v as u32)
                    ),
                    29 => quantize!(
                        |i| quantize_snorm(f32::from_bits(i), target as u32).map(|v| v as u32)
                    ),
                    30 => {
                        quantize!(|i| Ok::<_, Error>(u32::from(quantize_half(f32::from_bits(i)))))
                    }
                    31 => quantize!(
                        |i| quantize_float(f32::from_bits(i), target as u32).map(f32::to_bits)
                    ),
                    _ => quantize!(|i| Ok::<_, Error>(dequantize_half(i as u16).to_bits())),
                }
                Payload::Caller {
                    count: ic,
                    header: None,
                }
            }
            33 => Payload::Scalar(compute_position_exponent(
                positions[0],
                positions[1],
                p0 as i32,
                p1,
            )? as u32),
            34..=38 => {
                let a = Attributes::from_interleaved(&attributes, vc, ac, ac, 0)?;
                if caller {
                    let r = simplify_with_attributes_into(
                        &mut destination,
                        &indices,
                        p,
                        a,
                        &weights,
                        Some(&flags),
                        settings,
                        &mut ws,
                    )?;
                    Payload::Caller {
                        count: r.index_count,
                        header: Some(r.error.to_bits()),
                    }
                } else {
                    Payload::Simplified(simplify_with_attributes(
                        &indices,
                        p,
                        a,
                        &weights,
                        Some(&flags),
                        settings,
                        &mut ws,
                    )?)
                }
            }
            _ => unreachable!(),
        };
        Ok(output)
    };
    #[allow(unused_mut)]
    let mut result = operation().map_err(|e| e.to_string())?;
    #[allow(unused_mut)]
    let mut times = Vec::<f64>::new();
    #[cfg(target_arch = "wasm32")]
    if mode != 0 || paired {
        return Err("native timing only".into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    if mode != 0 {
        use std::io::{Read, Write};
        if paired {
            std::io::stdout()
                .write_all(b"R")
                .and_then(|()| std::io::stdout().flush())
                .map_err(|e| e.to_string())?;
        }
        for _ in 0..samples {
            if paired {
                let mut cmd = [0];
                std::io::stdin()
                    .read_exact(&mut cmd)
                    .map_err(|e| e.to_string())?;
                if cmd[0] == b'S' {
                    break;
                }
                if cmd[0] != b'R' {
                    return Err("paired command".into());
                }
            }
            let start = Instant::now();
            let repeats = if op == 33 {
                262144
            } else if ic < 3000 {
                32768
            } else if ic < 300000 {
                16
            } else {
                1
            };
            for _ in 0..repeats {
                result = operation().map_err(|e| e.to_string())?;
                std::hint::black_box(&result);
            }
            let t = start.elapsed().as_secs_f64() / f64::from(repeats);
            times.push(t);
            if paired {
                std::io::stdout()
                    .write_all(b"T")
                    .and_then(|()| std::io::stdout().write_all(&t.to_le_bytes()))
                    .and_then(|()| std::io::stdout().flush())
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    let result = result.words(&destination, &bytes, &reorder);
    let mut output = Vec::new();
    output.extend_from_slice(b"MR01");
    push(&mut output, 0);
    push(&mut output, result.len() as u32);
    push(&mut output, times.len() as u32);
    for value in result {
        push(&mut output, value);
    }
    for t in times {
        output.extend_from_slice(&t.to_le_bytes());
    }
    if mode != 0 {
        output.extend_from_slice(&(ws.usage().bytes as u64).to_le_bytes());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn irrelevant_mutated_width_is_rejected_before_transport_allocation() {
        for op in [7u32, 18, 26, 33, 34] {
            let mut data = b"MO02".to_vec();
            for v in [op, 2, 0, 0, 0, 0x8000000c, 12, 0, 0, 0, 0, 2, 0] {
                data.extend_from_slice(&v.to_le_bytes());
            }
            data.resize(56 + 2 * 12 + 2 * 4, 0);
            assert!(execute(&data, false).is_err());
        }
    }
}
