use meshoptimizer_rs::{codec::*, Error, Limits, Workspace};
use std::sync::Mutex;
fn word(b: &[u8], i: usize) -> Result<u32, Error> {
    Ok(u32::from_le_bytes(
        b.get(i..i + 4)
            .ok_or(Error::InvalidStream)?
            .try_into()
            .map_err(|_| Error::InvalidStream)?,
    ))
}
pub fn execute(b: &[u8]) -> Result<Vec<u8>, Error> {
    if b.len() < 44 || &b[..4] != b"MC02" || b.len() > 128 * 1024 * 1024 {
        return Err(Error::InvalidStream);
    }
    let op = word(b, 4)?;
    let count = word(b, 8)? as usize;
    let stride = word(b, 12)? as usize;
    let m = word(b, 16)?;
    let f = word(b, 20)?;
    let samples = word(b, 32)?;
    let iterations = word(b, 36)?;
    if word(b, 40)? as usize != b.len() - 44
        || samples > 100
        || iterations == 0
        || iterations > 1000000
    {
        return Err(Error::InvalidStream);
    }
    let source = &b[44..];
    let mut workspace = Workspace::new(Limits {
        max_bytes: 128 * 1024 * 1024,
        max_work: 1 << 34,
    });
    if op >= 11 {
        let fields = Fields {
            op,
            count,
            stride,
            mode: m,
            filter: f,
            version: word(b, 24)?,
            level: word(b, 28)?,
        };
        return codec04(&fields, source, &mut workspace, samples, iterations);
    }
    let bytes = count.checked_mul(stride).ok_or(Error::SizeOverflow)?;
    if bytes > 128 * 1024 * 1024 {
        return Err(Error::LimitExceeded);
    }
    let into = m & 128 != 0;
    let mut destination = vec![0u8; bytes + 16];
    let mode = match m & 127 {
        0 => Mode::Attributes,
        1 => Mode::Triangles,
        2 => Mode::Indices,
        _ => return Err(Error::InvalidParameter),
    };
    let filter = match f {
        0 => Filter::None,
        1 => Filter::Octahedral,
        2 => Filter::Quaternion,
        3 => Filter::Exponential,
        _ => return Err(Error::InvalidParameter),
    };
    let mut once = || -> Result<Vec<u8>, Error> {
        if op == 8 || op == 9 {
            let v = if op == 8 {
                decode_vertex_version(source)
            } else {
                decode_index_version(source)
            };
            return Ok(v.map_or(u32::MAX, u32::from).to_le_bytes().to_vec());
        }
        if (4..=6).contains(&op) {
            if source.len() != bytes {
                return Err(Error::InvalidLayout);
            }
            let mut allocated = Vec::new();
            let target = if into {
                destination[..bytes].copy_from_slice(source);
                &mut destination[..bytes]
            } else {
                allocated = source.to_vec();
                &mut allocated[..]
            };
            match op {
                4 => decode_filter_oct(target, count, stride, &mut workspace)?,
                5 => decode_filter_quat(target, count, stride, &mut workspace)?,
                _ => decode_filter_exp(target, count, stride, &mut workspace)?,
            };
            return Ok(if into { Vec::new() } else { allocated });
        }
        if into {
            match op {
                1 => decode_vertex_buffer_into(
                    &mut destination,
                    count,
                    stride,
                    source,
                    &mut workspace,
                )?,
                2 => decode_index_buffer_into(
                    &mut destination,
                    count,
                    stride,
                    source,
                    &mut workspace,
                )?,
                3 => decode_index_sequence_into(
                    &mut destination,
                    count,
                    stride,
                    source,
                    &mut workspace,
                )?,
                7 => decode_buffer_view_into(
                    &mut destination,
                    mode,
                    filter,
                    count,
                    stride,
                    source,
                    &mut workspace,
                )?,
                _ => return Err(Error::InvalidParameter),
            };
            Ok(Vec::new())
        } else {
            match op {
                1 => decode_vertex_buffer(count, stride, source, &mut workspace),
                2 => decode_index_buffer(count, stride, source, &mut workspace),
                3 => decode_index_sequence(count, stride, source, &mut workspace),
                7 => decode_buffer_view(mode, filter, count, stride, source, &mut workspace),
                _ => Err(Error::InvalidParameter),
            }
        }
    };
    let mut result = once();
    let mut times = Vec::new();
    for _ in 0..samples {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            result = std::hint::black_box(once());
        }
        times.push(start.elapsed().as_secs_f64() / f64::from(iterations));
    }
    let (status, mut data) = match result {
        Ok(v) => (0i32, v),
        Err(_) => (-1i32, Vec::new()),
    };
    if status == 0 && into && op != 8 && op != 9 {
        data.extend_from_slice(&destination[..bytes]);
    }
    let mut out = b"MD02".to_vec();
    out.extend_from_slice(&status.to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&(times.len() as u32).to_le_bytes());
    out.extend_from_slice(&data);
    for t in times {
        out.extend_from_slice(&t.to_le_bytes());
    }
    Ok(out)
}
/// Request header fields of a 0.4 operation (op 11 and above).
struct Fields {
    op: u32,
    count: usize,
    stride: usize,
    mode: u32,
    filter: u32,
    version: u32,
    level: u32,
}
enum Output {
    Owned(Vec<u8>),
    // Allocated meshlet outputs, serialized after timing.
    Meshlet(DecodedMeshlet),
    Raw(RawMeshlet),
    Prefix(usize),
    Value(u64),
}
fn words(b: &[u8]) -> Result<Vec<u32>, Error> {
    if !b.len().is_multiple_of(4) {
        return Err(Error::InvalidLayout);
    }
    Ok(b.as_chunks::<4>()
        .0
        .iter()
        .map(|c| u32::from_le_bytes(*c))
        .collect())
}
fn small(v: u32) -> Result<u8, Error> {
    u8::try_from(v).map_err(|_| Error::InvalidParameter)
}
fn exp_mode(v: u32) -> Result<ExpMode, Error> {
    Ok(match v {
        0 => ExpMode::Separate,
        1 => ExpMode::SharedVector,
        2 => ExpMode::SharedComponent,
        3 => ExpMode::Clamped,
        _ => return Err(Error::InvalidParameter),
    })
}
/// Operations 11-25: encoders, bounds, filter encoders, Color decoding and the
/// meshlet codec. Bit 128 of `mode` selects the caller-buffer API; a nonzero
/// `filter` gives an explicit encoder capacity plus one.
fn codec04(
    f: &Fields,
    source: &[u8],
    workspace: &mut Workspace,
    samples: u32,
    iterations: u32,
) -> Result<Vec<u8>, Error> {
    let into = f.mode & 128 != 0;
    let capacity = (f.filter != 0).then(|| f.filter as usize - 1);
    let indices = if matches!(f.op, 12 | 13) {
        words(source)?
    } else {
        Vec::new()
    };
    let floats: Vec<f32> = if (14..=17).contains(&f.op) {
        words(source)?.into_iter().map(f32::from_bits).collect()
    } else {
        Vec::new()
    };
    let split = f.count.saturating_mul(4);
    let meshlet_vertices = if f.op == 19 {
        words(source.get(..split).ok_or(Error::InvalidLayout)?)?
    } else {
        Vec::new()
    };
    let meshlet_triangles = if f.op == 19 {
        &source[split..]
    } else {
        &[][..]
    };
    // Caller buffers are sized before timing.
    let bound = match f.op {
        11 => encode_vertex_buffer_bound(f.count, f.stride).unwrap_or(0),
        12 => encode_index_buffer_bound(f.count, u32::MAX as usize).unwrap_or(0),
        13 => encode_index_sequence_bound(f.count, u32::MAX as usize).unwrap_or(0),
        19 => encode_meshlet_bound(f.count, f.version as usize).unwrap_or(0),
        14..=18 => f.count.saturating_mul(f.stride).min(128 * 1024 * 1024),
        20 | 21 => split.saturating_add((f.version as usize).saturating_mul(4)),
        _ => 0,
    };
    let mut destination = vec![0u8; capacity.unwrap_or(bound).min(256 * 1024 * 1024)];
    let mut raw_vertices = vec![0u32; if f.op == 21 { f.count } else { 0 }];
    let mut raw_triangles = vec![0u32; if f.op == 21 { f.version as usize } else { 0 }];
    let mut once = || -> Result<Output, Error> {
        let encoder_into = into || capacity.is_some();
        Ok(match f.op {
            11 => {
                let e = VertexEncoding::new(small(f.version)?, small(f.level)?)?;
                if encoder_into {
                    Output::Prefix(encode_vertex_buffer_into(
                        &mut destination,
                        source,
                        f.count,
                        f.stride,
                        e,
                        workspace,
                    )?)
                } else {
                    Output::Owned(encode_vertex_buffer(
                        source, f.count, f.stride, e, workspace,
                    )?)
                }
            }
            12 | 13 => {
                let e = IndexEncoding::new(small(f.version)?)?;
                if indices.len() != f.count {
                    return Err(Error::InvalidLayout);
                }
                match (f.op, encoder_into) {
                    (12, true) => Output::Prefix(encode_index_buffer_into(
                        &mut destination,
                        &indices,
                        e,
                        workspace,
                    )?),
                    (12, false) => Output::Owned(encode_index_buffer(&indices, e, workspace)?),
                    (_, true) => Output::Prefix(encode_index_sequence_into(
                        &mut destination,
                        &indices,
                        e,
                        workspace,
                    )?),
                    _ => Output::Owned(encode_index_sequence(&indices, e, workspace)?),
                }
            }
            14..=17 => {
                let (c, s, bits) = (f.count, f.stride, f.level);
                let d = &floats;
                if into {
                    let n = c.checked_mul(s).ok_or(Error::SizeOverflow)?;
                    let out = destination.get_mut(..n).ok_or(Error::BufferTooSmall)?;
                    match f.op {
                        14 => encode_filter_oct_into(out, c, s, bits, d, workspace)?,
                        15 => encode_filter_quat_into(out, c, s, bits, d, workspace)?,
                        16 => encode_filter_exp_into(
                            out,
                            c,
                            s,
                            bits,
                            d,
                            exp_mode(f.mode & 127)?,
                            workspace,
                        )?,
                        _ => encode_filter_color_into(out, c, s, bits, d, workspace)?,
                    }
                    Output::Prefix(n)
                } else {
                    Output::Owned(match f.op {
                        14 => encode_filter_oct(c, s, bits, d, workspace)?,
                        15 => encode_filter_quat(c, s, bits, d, workspace)?,
                        16 => encode_filter_exp(c, s, bits, d, exp_mode(f.mode & 127)?, workspace)?,
                        _ => encode_filter_color(c, s, bits, d, workspace)?,
                    })
                }
            }
            18 => {
                if source.len() != f.count.checked_mul(f.stride).ok_or(Error::SizeOverflow)? {
                    return Err(Error::InvalidLayout);
                }
                if into {
                    let out = &mut destination[..source.len()];
                    out.copy_from_slice(source);
                    decode_filter_color(out, f.count, f.stride, workspace)?;
                    Output::Prefix(source.len())
                } else {
                    let mut out = source.to_vec();
                    decode_filter_color(&mut out, f.count, f.stride, workspace)?;
                    Output::Owned(out)
                }
            }
            19 => {
                if encoder_into {
                    Output::Prefix(encode_meshlet_into(
                        &mut destination,
                        &meshlet_vertices,
                        meshlet_triangles,
                        workspace,
                    )?)
                } else {
                    Output::Owned(encode_meshlet(
                        &meshlet_vertices,
                        meshlet_triangles,
                        workspace,
                    )?)
                }
            }
            20 => {
                let (vc, vs) = (f.count, f.stride);
                let (tc, ts) = (f.version as usize, f.level as usize);
                if into {
                    let vb = vc.saturating_mul(vs).min(destination.len());
                    let (v, t) = destination.split_at_mut(vb);
                    decode_meshlet_into(v, vc, vs, t, tc, ts, source, workspace)?;
                    Output::Prefix(vb + tc * ts)
                } else {
                    Output::Meshlet(decode_meshlet(vc, vs, tc, ts, source, workspace)?)
                }
            }
            21 => {
                let (vc, tc) = (f.count, f.version as usize);
                if into {
                    decode_meshlet_raw_into(
                        &mut raw_vertices,
                        vc,
                        &mut raw_triangles,
                        tc,
                        source,
                        workspace,
                    )?;
                    Output::Prefix(0)
                } else {
                    Output::Raw(decode_meshlet_raw(vc, tc, source, workspace)?)
                }
            }
            22..=25 => {
                // One bound is the result; 255 more (count + 3i) amortize the
                // per-request overhead identically in both drivers.
                let vertices = if matches!(f.op, 23 | 24) {
                    let v =
                        u64::from_le_bytes(source.try_into().map_err(|_| Error::InvalidLayout)?);
                    usize::try_from(v).map_err(|_| Error::SizeOverflow)?
                } else {
                    0
                };
                let bound = |count: usize| match f.op {
                    22 => encode_vertex_buffer_bound(count, f.stride),
                    23 => encode_index_buffer_bound(count, vertices),
                    24 => encode_index_sequence_bound(count, vertices),
                    _ => encode_meshlet_bound(count, f.level as usize),
                };
                let first = bound(f.count)?;
                let mut sink = 0usize;
                for i in 1..256 {
                    sink ^= bound(f.count.wrapping_add(3 * i)).unwrap_or(0);
                }
                std::hint::black_box(sink);
                Output::Value(first as u64)
            }
            _ => return Err(Error::InvalidParameter),
        })
    };
    let mut result = once();
    let mut times = Vec::new();
    for _ in 0..samples {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            result = std::hint::black_box(once());
        }
        times.push(start.elapsed().as_secs_f64() / f64::from(iterations));
    }
    let (status, data) = match result {
        Ok(Output::Owned(v)) => (0i32, v),
        Ok(Output::Meshlet(d)) => (0, [d.vertices, d.triangles].concat()),
        Ok(Output::Raw(d)) => (
            0,
            d.vertices
                .iter()
                .chain(&d.triangles)
                .flat_map(|v| v.to_le_bytes())
                .collect(),
        ),
        Ok(Output::Prefix(_)) if f.op == 21 => {
            let mut v: Vec<u8> = raw_vertices.iter().flat_map(|v| v.to_le_bytes()).collect();
            v.extend(raw_triangles.iter().flat_map(|v| v.to_le_bytes()));
            (0, v)
        }
        Ok(Output::Prefix(n)) => (0, destination[..n].to_vec()),
        Ok(Output::Value(v)) => (0, v.to_le_bytes().to_vec()),
        Err(_) => (-1, Vec::new()),
    };
    let mut out = b"MD02".to_vec();
    out.extend_from_slice(&status.to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&(times.len() as u32).to_le_bytes());
    out.extend_from_slice(&data);
    for t in times {
        out.extend_from_slice(&t.to_le_bytes());
    }
    Ok(out)
}
static INPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
#[no_mangle]
pub extern "C" fn request(size: u32) -> u32 {
    let Ok(mut output) = OUTPUT.lock() else {
        return 1;
    };
    output.clear();
    let Ok(mut input) = INPUT.lock() else {
        return 1;
    };
    input.clear();
    if size > 128 * 1024 * 1024 {
        return 1;
    }
    if input.try_reserve_exact(size as usize).is_err() {
        return 1;
    }
    input.resize(size as usize, 0);
    0
}
#[no_mangle]
pub extern "C" fn put_byte(index: u32, value: u32) -> u32 {
    if value > 255 {
        return 1;
    }
    let Ok(mut input) = INPUT.lock() else {
        return 1;
    };
    if let Some(b) = input.get_mut(index as usize) {
        *b = value as u8;
        0
    } else {
        1
    }
}
#[no_mangle]
pub extern "C" fn run() -> u32 {
    let Ok(mut output) = OUTPUT.lock() else {
        return 1;
    };
    output.clear();
    let Ok(input) = INPUT.lock() else { return 1 };
    match execute(&input) {
        Ok(b) => {
            *output = b;
            0
        }
        Err(_) => 1,
    }
}
#[no_mangle]
pub extern "C" fn output_len() -> u32 {
    OUTPUT.lock().map_or(0, |b| b.len() as u32)
}
#[no_mangle]
pub extern "C" fn get_byte(index: u32) -> u32 {
    OUTPUT.lock().map_or(256, |b| {
        b.get(index as usize).map_or(256, |v| u32::from(*v))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transport_failures_clear_prior_state() {
        let mut b = b"MC02".to_vec();
        for v in [8u32, 0, 4, 0, 0, 0, 2, 0, 1, 1] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.push(0xa1);
        assert_eq!(request(b.len() as u32), 0);
        for (i, v) in b.iter().enumerate() {
            assert_eq!(put_byte(i as u32, u32::from(*v)), 0);
        }
        assert_eq!(run(), 0);
        assert!(output_len() > 0);
        assert_eq!(request(u32::MAX), 1);
        assert_eq!(output_len(), 0);
        assert_eq!(run(), 1);
        assert_eq!(request(44), 0);
        assert_eq!(put_byte(0, 256), 1);
        assert_eq!(put_byte(44, 0), 1);
        assert_eq!(run(), 1);
        assert_eq!(get_byte(0), 256);
    }
}
