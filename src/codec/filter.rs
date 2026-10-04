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
/// Scalar meshopt_decodeFilterColor. Returns NumericalFailure where the C++
/// float-to-int conversion is undefined: a zero alpha word (infinite scale)
/// or a 16-bit record whose scaled component leaves the i32 range.
pub(super) fn color(data: &mut [u8], stride: usize) -> Result<(), Error> {
    // Chunks of up to 64 records take a vectorizable path whose conversion is
    // exact below 2^22; a chunk with any larger (or non-finite) value is
    // restored from its copy and redone with the per-record checked path.
    const RECORDS: usize = 64;
    let mut valid = true;
    let mut copy = [0u8; RECORDS * 8];
    for chunk in data.chunks_mut(RECORDS * stride) {
        let saved = &mut copy[..chunk.len()];
        saved.copy_from_slice(chunk);
        let small = if stride == 8 {
            color_fast::<8>(chunk)
        } else {
            color_fast::<4>(chunk)
        };
        if !small {
            chunk.copy_from_slice(saved);
            valid &= color_checked(chunk, stride);
        }
    }
    if valid {
        Ok(())
    } else {
        Err(Error::NumericalFailure)
    }
}

#[inline(always)]
fn color_fields<const W: usize>(e: &[u8; W]) -> (i32, i32, i32, i32) {
    if W == 8 {
        let lo = u32::from_le_bytes([e[0], e[1], e[2], e[3]]);
        let hi = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        (
            (lo & 0xffff) as i32,
            (lo as i32) >> 16,
            ((hi << 16) as i32) >> 16,
            (hi >> 16) as i32,
        )
    } else {
        (
            i32::from(e[0]),
            i32::from(e[1] as i8),
            i32::from(e[2] as i8),
            i32::from(e[3]),
        )
    }
}

#[inline(always)]
fn color_store<const W: usize>(e: &mut [u8; W], v: [i32; 4]) {
    if W == 8 {
        let lo = (v[0] as u32 & 0xffff) | (v[1] as u32) << 16;
        let hi = (v[2] as u32 & 0xffff) | (v[3] as u32) << 16;
        e[..4].copy_from_slice(&lo.to_le_bytes());
        e[4..].copy_from_slice(&hi.to_le_bytes());
    } else {
        for (b, v) in e.iter_mut().zip(v) {
            *b = v as u8;
        }
    }
}

/// Plain record loop without saturating conversions or early exits, so it
/// vectorizes across records. Returns false if any value was not below 2^22.
fn color_fast<const W: usize>(data: &mut [u8]) -> bool {
    let max = if W == 8 { 65535.0 } else { 255.0 };
    let mut small = true;
    let data = if W == 8 {
        // 16-bit records: four records per step in lanes (the record loop
        // below does not vectorize for this layout).
        let (blocks, rest) = data.as_chunks_mut::<32>();
        for block in blocks {
            let words: [u32; 8] = core::array::from_fn(|i| word(block, i));
            let lo: [u32; 4] = core::array::from_fn(|r| words[r * 2]);
            let hi: [u32; 4] = core::array::from_fn(|r| words[r * 2 + 1]);
            let mut f = [[0f32; 4]; 4];
            for l in 0..4 {
                let y = (lo[l] & 0xffff) as i32;
                let co = (lo[l] as i32) >> 16;
                let cg = ((hi[l] << 16) as i32) >> 16;
                let alpha = (hi[l] >> 16) as i32;
                let mut scale = alpha;
                scale |= scale >> 1;
                scale |= scale >> 2;
                scale |= scale >> 4;
                scale |= scale >> 8;
                let ss = max / scale as f32;
                f[0][l] = (y + co - cg) as f32 * ss + 0.5;
                f[1][l] = (y + cg) as f32 * ss + 0.5;
                f[2][l] = (y - co - cg) as f32 * ss + 0.5;
                f[3][l] = (((alpha << 1) & scale) | (alpha & 1)) as f32 * ss + 0.5;
            }
            for c in &f {
                for &v in c {
                    small &= v.abs() < 4194304.0;
                }
            }
            let v = f.map(|c| c.map(trunc_small));
            let out: [u32; 8] = core::array::from_fn(|i| {
                let l = i / 2;
                let (a, b) = if i % 2 == 0 {
                    (v[0][l], v[1][l])
                } else {
                    (v[2][l], v[3][l])
                };
                (a as u32 & 0xffff) | (b as u32) << 16
            });
            for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(out) {
                *dst = w.to_le_bytes();
            }
        }
        rest
    } else {
        data
    };
    for e in data.as_chunks_mut::<W>().0 {
        let (y, co, cg, alpha) = color_fields(e);
        let mut scale = alpha;
        scale |= scale >> 1;
        scale |= scale >> 2;
        scale |= scale >> 4;
        scale |= scale >> 8;
        let a = ((alpha << 1) & scale) | (alpha & 1);
        let ss = max / scale as f32;
        let f = [y + co - cg, y + cg, y - co - cg, a].map(|c| c as f32 * ss + 0.5);
        for v in f {
            small &= v.abs() < 4194304.0;
        }
        color_store(e, f.map(trunc_small));
    }
    small
}

/// Per-record checked path; false where a reference conversion is undefined.
fn color_checked(data: &mut [u8], stride: usize) -> bool {
    let mut valid = true;
    if stride == 8 {
        for e in data.as_chunks_mut::<8>().0 {
            let (y, co, cg, alpha) = color_fields(e);
            let (v, ok) = color_record(y, co, cg, alpha, 65535.0);
            valid &= ok;
            color_store(e, v);
        }
    } else {
        for e in data.as_chunks_mut::<4>().0 {
            let (y, co, cg, alpha) = color_fields(e);
            let (v, ok) = color_record(y, co, cg, alpha, 255.0);
            valid &= ok;
            color_store(e, v);
        }
    }
    valid
}

#[inline(always)]
fn word(block: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([
        block[i * 4],
        block[i * 4 + 1],
        block[i * 4 + 2],
        block[i * 4 + 3],
    ])
}

/// Exact truncating float-to-int conversion for |f| < 2^22, built from float
/// adds and bit casts so that four lanes vectorize on baseline x86-64 (the
/// saturating `as` conversion does not). Equal to `f as i32` on that domain.
#[inline(always)]
pub(super) fn trunc_small(f: f32) -> i32 {
    const MAGIC: f32 = 12582912.0; // 1.5 * 2^23: unit spacing, exact sums
    let nearest = (f + MAGIC) - MAGIC;
    let toward_zero = if nearest.abs() > f.abs() {
        nearest - 1f32.copysign(f)
    } else {
        nearest
    };
    // Wrapping: callers may evaluate out-of-domain lanes and discard them.
    ((toward_zero + MAGIC).to_bits() as i32).wrapping_sub(MAGIC.to_bits() as i32)
}

/// One Color record; false when a reference conversion would be undefined.
#[inline(always)]
fn color_record(y: i32, co: i32, cg: i32, alpha: i32, max: f32) -> ([i32; 4], bool) {
    // Recover the scale from the alpha high bit.
    let mut scale = alpha;
    scale |= scale >> 1;
    scale |= scale >> 2;
    scale |= scale >> 4;
    scale |= scale >> 8;
    // Expand alpha by one bit to match the other components.
    let a = ((alpha << 1) & scale) | (alpha & 1);
    let ss = max / scale as f32;
    let mut valid = true;
    let values = [y + co - cg, y + cg, y - co - cg, a].map(|c| {
        let f = c as f32 * ss + 0.5;
        valid &= (-2147483648.0..2147483648.0).contains(&f);
        f as i32
    });
    (values, valid)
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

#[cfg(test)]
mod tests {
    use super::trunc_small;

    #[test]
    fn small_truncation_matches_the_reference_conversion() {
        // Every float near integers, halves and the domain edge, both signs.
        let mut values = alloc::vec![0.0f32, -0.0, 0.49999997, 0.5, 0.50000006, 4194303.5];
        for i in 0..20000 {
            let base = (i as f32) * 209.71;
            values.extend([base, base.next_up(), base.next_down(), base + 0.5]);
        }
        for v in values {
            for v in [v, -v] {
                if v.abs() < 4194304.0 {
                    assert_eq!(trunc_small(v), v as i32, "{v}");
                }
            }
        }
    }
}
