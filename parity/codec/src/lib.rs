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
