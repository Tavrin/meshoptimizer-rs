// meshoptimizer 1.3 canonical scalar filters, MIT, Arseny Kapoulkine.
use crate::{math::sqrt, Error};

// Integer-backed Oct components normalize into +/-32767; Quat's nonzero
// scale word bounds its scaled components below 2^30. Thus every conversion
// below is in i32 range; the zero Oct vector is checked before normalization.
fn rounded(v: f32, sign: f32) -> i32 {
    (v + if sign >= 0.0 { 0.5 } else { -0.5 }) as i32
}
pub(super) fn oct(data: &mut [u8], stride: usize) -> Result<(), Error> {
    let mut previous = ([0i16; 3], [0i32; 3]);
    let mut valid = false;
    for element in data.chunks_exact_mut(stride) {
        let key = if stride == 4 {
            [
                element[0] as i8 as i16,
                element[1] as i8 as i16,
                element[2] as i8 as i16,
            ]
        } else {
            [
                i16::from_le_bytes([element[0], element[1]]),
                i16::from_le_bytes([element[2], element[3]]),
                i16::from_le_bytes([element[4], element[5]]),
            ]
        };
        if valid && key == previous.0 {
            for (i, v) in previous.1.into_iter().enumerate() {
                if stride == 4 {
                    element[i] = v as i8 as u8;
                } else {
                    element[i * 2..i * 2 + 2].copy_from_slice(&(v as i16).to_le_bytes());
                }
            }
            continue;
        }
        let read = |i| {
            if stride == 4 {
                f32::from(element[i] as i8)
            } else {
                f32::from(i16::from_le_bytes([element[i * 2], element[i * 2 + 1]]))
            }
        };
        let mut x = read(0);
        let mut y = read(1);
        let z = read(2) - x.abs() - y.abs();
        let t = if z >= 0.0 { 0.0 } else { z };
        x += if x >= 0.0 { t } else { -t };
        y += if y >= 0.0 { t } else { -t };
        let length = sqrt(x * x + y * y + z * z);
        if length == 0.0 {
            return Err(Error::NumericalFailure);
        }
        let scale = if stride == 4 { 127.0 } else { 32767.0 } / length;
        let values = [
            rounded(x * scale, x),
            rounded(y * scale, y),
            rounded(z * scale, z),
        ];
        previous = (key, values);
        valid = true;
        for (i, v) in values.into_iter().enumerate() {
            if stride == 4 {
                element[i] = v as i8 as u8;
            } else {
                element[i * 2..i * 2 + 2].copy_from_slice(&(v as i16).to_le_bytes());
            }
        }
    }
    Ok(())
}
pub(super) fn quat(data: &mut [u8]) -> Result<(), Error> {
    let scale = 32767.0 / sqrt(2.0);
    let mut previous = ([0i16; 4], [0u8; 8]);
    let mut valid = false;
    for element in data.as_chunks_mut::<8>().0 {
        let read = |i: usize| i16::from_le_bytes([element[i * 2], element[i * 2 + 1]]);
        let [a, b, c, d] = [read(0), read(1), read(2), read(3)];
        let key = [a, b, c, d];
        if valid && key == previous.0 {
            *element = previous.1;
            continue;
        }
        let x = f32::from(a);
        let y = f32::from(b);
        let z = f32::from(c);
        let s = f32::from(d | 3);
        let ww = (s * s) * 2.0 - x * x - y * y - z * z;
        let w = sqrt(if ww >= 0.0 { ww } else { 0.0 });
        let ss = scale / s;
        let values = [
            rounded(x * ss, x),
            rounded(y * ss, y),
            rounded(z * ss, z),
            (w * ss + 0.5) as i32,
        ];
        let largest = (d & 3) as usize;
        for (i, v) in values.into_iter().enumerate() {
            let slot = (largest + i + 1) & 3;
            element[slot * 2..slot * 2 + 2].copy_from_slice(&(v as i16).to_le_bytes());
        }
        previous = (key, *element);
        valid = true;
    }
    Ok(())
}
pub(super) fn exp(data: &mut [u8]) {
    for word in data.as_chunks_mut::<4>().0 {
        let v = u32::from_le_bytes(*word);
        let m = ((v << 8) as i32) >> 8;
        let e = (v as i32) >> 24;
        let decoded = f32::from_bits((e.wrapping_add(127) as u32) << 23) * m as f32;
        word.copy_from_slice(&decoded.to_bits().to_le_bytes());
    }
}
