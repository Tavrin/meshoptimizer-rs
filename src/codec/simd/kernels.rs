// Arithmetic order is the canonical scalar filter's order. Each backend
// supplies exact IEEE vector operations; no estimates, FMA or reassociation.
#[cfg(target_arch = "aarch64")]
macro_rules! filter_kernel {
    ($feature:literal) => {
        #[target_feature(enable = $feature)]
        fn filter_kernel(kind: u8, data: &mut [u8], stride: usize) -> Result<(), crate::Error> {
            use crate::codec::filter as scalar;
            if kind == 3 {
                exp_kernel(data);
                return Ok(());
            }
            if kind == 4 {
                return if stride == 4 {
                    color_kernel::<4, 16>(data)
                } else {
                    color_kernel::<8, 32>(data)
                };
            }
            let records = if kind == 3 {
                data.len() / 4
            } else {
                data.len() / stride
            };
            let width = if kind == 3 { 4 } else { stride };
            let mut offset = 0;
            while offset + 4 <= records {
                let chunk = &mut data[offset * width..(offset + 4) * width];
                let mut fields = [[0f32; 4]; 4];
                let mut raw = [[0i32; 4]; 4];
                for lane in 0..4 {
                    let e = &chunk[lane * width..(lane + 1) * width];
                    for c in 0..if kind == 3 { 1 } else { 4 } {
                        raw[c][lane] = if kind == 3 {
                            i32::from_le_bytes(e[..4].try_into().unwrap())
                        } else if width == 4 {
                            if kind == 4 && (c == 0 || c == 3) {
                                i32::from(e[c])
                            } else {
                                i32::from(e[c] as i8)
                            }
                        } else if kind == 4 && (c == 0 || c == 3) {
                            i32::from(u16::from_le_bytes([e[c * 2], e[c * 2 + 1]]))
                        } else {
                            i32::from(i16::from_le_bytes([e[c * 2], e[c * 2 + 1]]))
                        };
                        fields[c][lane] = raw[c][lane] as f32;
                    }
                }
                let [mut x, mut y, z0, d] = fields.map(|v| fload(v));
                let zero = splat(0.0);
                let half = splat(0.5);
                let rounded = |v, sign| trunc(add(v, select(ge(sign, zero), half, splat(-0.5))));
                let mut values = [[0i32; 4]; 4];
                match kind {
                    1 => {
                        let z = sub(sub(z0, abs(x)), abs(y));
                        let t = select(ge(z, zero), zero, z);
                        x = add(x, select(ge(x, zero), t, neg(t)));
                        y = add(y, select(ge(y, zero), t, neg(t)));
                        let length = sqrt(add(add(mul(x, x), mul(y, y)), mul(z, z)));
                        if fout(length).contains(&0.0) {
                            return Err(crate::Error::NumericalFailure);
                        }
                        let ss = div(splat(if width == 4 { 127.0 } else { 32767.0 }), length);
                        values[0] = rounded(mul(x, ss), x);
                        values[1] = rounded(mul(y, ss), y);
                        values[2] = rounded(mul(z, ss), z);
                    }
                    2 => {
                        let s = fload(raw[3].map(|v| (v | 3) as f32));
                        let ww = sub(
                            sub(sub(mul(mul(s, s), splat(2.0)), mul(x, x)), mul(y, y)),
                            mul(z0, z0),
                        );
                        let w = sqrt(select(ge(ww, zero), ww, zero));
                        let ss = div(splat(32767.0 / crate::math::sqrt(2.0)), s);
                        values[0] = rounded(mul(x, ss), x);
                        values[1] = rounded(mul(y, ss), y);
                        values[2] = rounded(mul(z0, ss), z0);
                        values[3] = trunc(add(mul(w, ss), half));
                    }
                    3 => {
                        let exponent =
                            fload(raw[0].map(|v| {
                                f32::from_bits(((v >> 24).wrapping_add(127) as u32) << 23)
                            }));
                        let mantissa = fload(raw[0].map(|v| ((v << 8) >> 8) as f32));
                        let result = fout(mul(exponent, mantissa));
                        for (lane, value) in result.into_iter().enumerate() {
                            chunk[lane * 4..lane * 4 + 4]
                                .copy_from_slice(&value.to_bits().to_le_bytes());
                        }
                        offset += 4;
                        continue;
                    }
                    _ => {
                        let scale = raw[3].map(|mut v| {
                            v |= v >> 1;
                            v |= v >> 2;
                            v |= v >> 4;
                            v |= v >> 8;
                            v
                        });
                        let alpha = core::array::from_fn(|l| {
                            (((raw[3][l] << 1) & scale[l]) | (raw[3][l] & 1)) as f32
                        });
                        let ss = div(
                            splat(if width == 4 { 255.0 } else { 65535.0 }),
                            fload(scale.map(|v| v as f32)),
                        );
                        // Integer sums are exact before conversion (Color uses integer fields).
                        let rgb = [
                            core::array::from_fn(|l| (raw[0][l] + raw[1][l] - raw[2][l]) as f32),
                            core::array::from_fn(|l| (raw[0][l] + raw[2][l]) as f32),
                            core::array::from_fn(|l| (raw[0][l] - raw[1][l] - raw[2][l]) as f32),
                            alpha,
                        ];
                        let f = rgb.map(|c| add(mul(fload(c), ss), half));
                        if f.iter()
                            .flat_map(|&v| fout(v))
                            .any(|v| !(-2147483648.0..2147483648.0).contains(&v))
                        {
                            // No output has been stored; scalar owns exceptional conversions/errors.
                            scalar::scalar_color(chunk, stride)?;
                            offset += 4;
                            continue;
                        }
                        values = f.map(|v| trunc(v));
                    }
                }
                let _ = d;
                for lane in 0..4 {
                    for (c, v) in values
                        .iter()
                        .enumerate()
                        .take(if kind == 1 { 3 } else { 4 })
                    {
                        let slot = if kind == 2 {
                            ((raw[3][lane] & 3) as usize + c + 1) & 3
                        } else {
                            c
                        };
                        let e = &mut chunk[lane * width..(lane + 1) * width];
                        if width == 4 {
                            e[slot] = v[lane] as u8;
                        } else {
                            e[slot * 2..slot * 2 + 2]
                                .copy_from_slice(&(v[lane] as i16).to_le_bytes());
                        }
                    }
                }
                offset += 4;
            }
            let tail = &mut data[offset * width..];
            match kind {
                1 => scalar::scalar_oct(tail, stride),
                2 => scalar::scalar_quat(tail),
                3 => {
                    scalar::scalar_exp(tail);
                    Ok(())
                }
                _ => scalar::scalar_color(tail, stride),
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! portable_group {
    ($feature:literal) => {
        #[target_feature(enable = $feature)]
        fn group_kernel(data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
            if bits == 0 {
                out.fill(0);
                return 0;
            }
            if bits == 8 {
                out.copy_from_slice(&data[..16]);
                return 16;
            }
            let escape = (1u8 << bits) - 1;
            let mut fields = [0; 16];
            let mut mask = 0usize;
            for (i, v) in fields.iter_mut().enumerate() {
                let shift = if bits == 1 {
                    i % 8
                } else {
                    8 - bits as usize - (i % (8 / bits as usize)) * bits as usize
                };
                *v = (data[i / (8 / bits as usize)] >> shift) & escape;
                if *v == escape {
                    mask |= 1 << i;
                    *v = 0;
                }
            }
            let lo = mask & 255;
            let hi = mask >> 8;
            let mut shuf = super::MASKS[lo];
            let n = lo.count_ones() as u8;
            for i in 0..8 {
                shuf[8 + i] = super::MASKS[hi][i] + n;
            }
            let skip = (bits * 2) as usize;
            expand(
                data[skip..].first_chunk::<16>().unwrap(),
                &shuf,
                &fields,
                out,
            );
            skip + mask.count_ones() as usize
        }
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! triangle_kernel {
    ($feature:literal) => {
        #[target_feature(enable = $feature)]
        fn triangles_kernel(
            source: &[u8],
            bound: usize,
            codes: &[u8],
            mut data: usize,
            count: usize,
            out: &mut impl FnMut(usize, u32),
        ) -> Option<Result<usize, crate::Error>> {
            let mut state = [0u8; 16];
            for i in (0..count).step_by(2) {
                if data > bound {
                    return Some(Err(crate::Error::InvalidStream));
                }
                let (shuf, next, used, first_used) = super::TRIANGLE_TABLES[codes[i / 2] as usize];
                // SIMD byte history is equivalent only before the next counter
                // wraps. Malformed but scalar-defined restarts use the reference.
                if usize::from(state[15]) + usize::from(next[15]) >= 256 {
                    return None;
                }
                if i + 1 < count && data + first_used > bound {
                    return Some(Err(crate::Error::InvalidStream));
                }
                let Some(window) = source.get(data..).and_then(|b| b.first_chunk::<16>()) else {
                    return Some(Err(crate::Error::InvalidStream));
                };
                triangle_step(&mut state, window, &shuf, &next);
                for k in 0..(count - i).min(2) {
                    let a = u32::from(state[9 + k * 3]);
                    let b = u32::from(state[10 + k * 3]);
                    let c = u32::from(state[11 + k * 3]);
                    out(i + k, c | (a << 8) | (b << 16) | (c << 24));
                }
                data += if i + 1 < count { used } else { first_used };
            }
            Some(Ok(data))
        }
    };
}
