//! Exact Result/error and float-bit comparison against the separately built v0.2.9.
fn obs<T: std::fmt::Debug>(v: Result<f32, T>) -> Result<u32, String> {
    v.map(f32::to_bits).map_err(|e| format!("{e:?}"))
}
macro_rules! compare {
    ($a:expr, $b:expr, $count:ident) => {{
        assert_eq!(obs($a), obs($b));
        *$count += 1;
    }};
}
fn check(points: &[[f32; 3]], count: &mut usize) {
    compare!(
        candidate::simplify_scale(candidate::Positions::from_packed(points)),
        baseline::simplify_scale(baseline::Positions::from_packed(points)),
        count
    );
    for stride in 3..=64 {
        for offset in 0..=3 {
            for short in [false, true] {
                let len = if short && !points.is_empty() {
                    offset + (points.len() - 1) * stride + 3
                } else {
                    offset + points.len() * stride
                };
                // Padding is nonfinite: validation must examine XYZ only.
                let mut data = vec![f32::NAN; len];
                for (i, p) in points.iter().enumerate() {
                    data[offset + i * stride..offset + i * stride + 3].copy_from_slice(p);
                }
                let a = candidate::Positions::from_interleaved(&data, points.len(), stride, offset)
                    .unwrap();
                let b = baseline::Positions::from_interleaved(&data, points.len(), stride, offset)
                    .unwrap();
                compare!(
                    candidate::simplify_scale(a),
                    baseline::simplify_scale(b),
                    count
                );
            }
        }
    }
    // 68/256-byte controls are deliberately outside the newly specialized layout.
    for stride in (12..=256).step_by(4) {
        for offset in 0..=3 {
            for big in [false, true] {
                for short in [false, true] {
                    let len = if short && !points.is_empty() {
                        offset + (points.len() - 1) * stride + 12
                    } else {
                        offset + points.len() * stride
                    };
                    let mut data = vec![0xff; len];
                    for (i, p) in points.iter().enumerate() {
                        for (j, v) in p.iter().enumerate() {
                            let at = offset + i * stride + j * 4;
                            data[at..at + 4].copy_from_slice(&if big {
                                v.to_be_bytes()
                            } else {
                                v.to_le_bytes()
                            });
                        }
                    }
                    let a = candidate::Positions::from_bytes(
                        &data,
                        points.len(),
                        stride,
                        offset,
                        if big {
                            candidate::ByteOrder::BigEndian
                        } else {
                            candidate::ByteOrder::LittleEndian
                        },
                    )
                    .unwrap();
                    let b = baseline::Positions::from_bytes(
                        &data,
                        points.len(),
                        stride,
                        offset,
                        if big {
                            baseline::ByteOrder::BigEndian
                        } else {
                            baseline::ByteOrder::LittleEndian
                        },
                    )
                    .unwrap();
                    compare!(
                        candidate::simplify_scale(a),
                        baseline::simplify_scale(b),
                        count
                    );
                }
            }
        }
    }
}
fn main() {
    let mut count = 0;
    if let Some(dir) = std::env::args().nth(1) {
        for path in std::fs::read_dir(dir).unwrap() {
            let path = path.unwrap().path();
            if path.extension().is_none_or(|e| e != "bin")
                || path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("-strip")
                || path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("-simplify")
            {
                continue;
            }
            let b = std::fs::read(path).unwrap();
            let n = u32::from_le_bytes(b[..4].try_into().unwrap()) as usize;
            let p: Vec<[f32; 3]> = b[8..8 + n * 12]
                .chunks_exact(12)
                .map(|v| {
                    std::array::from_fn(|j| {
                        f32::from_le_bytes(v[j * 4..j * 4 + 4].try_into().unwrap())
                    })
                })
                .collect();
            let mut raw = vec![0xff; 3 + n * 64];
            for (i, p) in p.iter().enumerate() {
                for (j, x) in p.iter().enumerate() {
                    raw[3 + i * 64 + j * 4..3 + i * 64 + j * 4 + 4]
                        .copy_from_slice(&x.to_le_bytes());
                }
            }
            for short in [false, true] {
                if short && n > 0 {
                    raw.truncate(3 + (n - 1) * 64 + 12);
                }
                let a = candidate::Positions::from_bytes(
                    &raw,
                    n,
                    64,
                    3,
                    candidate::ByteOrder::LittleEndian,
                )
                .unwrap();
                let b = baseline::Positions::from_bytes(
                    &raw,
                    n,
                    64,
                    3,
                    baseline::ByteOrder::LittleEndian,
                )
                .unwrap();
                assert_eq!(
                    obs(candidate::simplify_scale(a)),
                    obs(baseline::simplify_scale(b))
                );
                count += 1;
            }
        }
    }
    check(&[], &mut count);
    let bits = [
        0,
        0x80000000,
        1,
        0x80000001,
        0x007fffff,
        0x807fffff,
        0x00800000,
        0x80800000,
        1e38f32.to_bits(),
        (-1e38f32).to_bits(),
        f32::MAX.to_bits(),
        (-f32::MAX).to_bits(),
        f32::INFINITY.to_bits(),
        f32::NEG_INFINITY.to_bits(),
        0x7fc00000,
        0x7f800001,
        0xffc12345,
    ];
    for &x in &bits {
        let x = f32::from_bits(x);
        check(&[[x; 3]], &mut count);
        for axis in 0..3 {
            for position in 0..4 {
                let mut p = vec![
                    [0.0, -0.0, 1.0],
                    [f32::MAX; 3],
                    [-f32::MAX; 3],
                    [1e38, -1e38, 0.0],
                ];
                p[position][axis] = x;
                check(&p, &mut count);
            }
        }
    }
    // Signed-zero insertion order, and mutations of arbitrary floating point bits.
    for mask in 0..64 {
        let p = std::array::from_fn::<_, 2, _>(|i| {
            std::array::from_fn(|j| {
                if mask & (1 << (i * 3 + j)) != 0 {
                    -0.0
                } else {
                    0.0
                }
            })
        });
        check(&p, &mut count);
    }
    let mut seed = 20261026u64;
    for n in 0..512 {
        let p: Vec<[f32; 3]> = (0..n % 19)
            .map(|_| {
                std::array::from_fn(|_| {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    f32::from_bits(seed as u32)
                })
            })
            .collect();
        check(&p, &mut count);
    }
    // Reproduce the current run_simplifier(6) seeded point stream, including
    // draws consumed by parameters, unused flags, and constructor mutations.
    let mut state = 20261002u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state as u32
    };
    for execution in 0usize..512 {
        let n = (next() % 49) as usize;
        let mut p: Vec<[f32; 3]> = (0..n)
            .map(|i| [i as f32 % 7., (i / 7) as f32, (next() % 8) as f32 / 16.])
            .collect();
        if execution.is_multiple_of(4) {
            for v in &mut p {
                for x in v {
                    *x = f32::from_bits(next());
                }
            }
        }
        if execution.is_multiple_of(9) {
            for i in 1..n {
                p[i] = p[i % 3];
            }
        }
        if execution.is_multiple_of(5) && n > 8 {
            next();
        }
        for _ in 0..n + 3 {
            next();
        }
        if execution.is_multiple_of(13) {
            next();
        }
        if execution.is_multiple_of(17) {
            next();
        }
        check(&p, &mut count);
        for _ in 0..4 {
            next();
        }
    }
    println!("PASS {count} exact Result/bit observations");
}
