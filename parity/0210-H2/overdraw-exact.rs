//! Scratch consumer comparing two separately compiled crate versions.
use std::{fs, path::Path};

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(), String>,
    output: Vec<u32>,
    work: u64,
    bytes: usize,
}
struct Case {
    p: Vec<[f32; 3]>,
    indices: Vec<u32>,
    threshold: f32,
}
macro_rules! observe {
    ($m:ident, $case:expr, $layout:expr, $api:expr, $fuel:expr, $bytes:expr) => {{
        use $m as m;
        let c = $case;
        let layout = $layout;
        let (kind, stride, offset, short) = match layout {
            0 => (0, 3, 0, false),
            1 => (1, 4, 1, false),
            2 | 3 => (layout, 16, 1, false),
            4..=7 => (2 + (layout % 2), 64, 1, layout >= 6),
            _ => {
                let code = layout - 8;
                (
                    1 + code / 4096,
                    code % 512,
                    code / 512 % 4,
                    code % 4096 >= 2048,
                )
            }
        };
        let mut floats = vec![
            f32::NAN;
            if kind == 1 {
                offset + c.p.len() * stride
            } else {
                0
            }
        ];
        let mut bytes = vec![
            0xff;
            if kind >= 2 {
                offset + c.p.len() * stride
            } else {
                0
            }
        ];
        for (i, p) in c.p.iter().enumerate() {
            if kind == 1 {
                floats[offset + i * stride..offset + i * stride + 3].copy_from_slice(p);
            }
            if kind >= 2 {
                for (j, x) in p.iter().enumerate() {
                    bytes[offset + i * stride + j * 4..offset + i * stride + j * 4 + 4]
                        .copy_from_slice(&if kind == 3 {
                            x.to_be_bytes()
                        } else {
                            x.to_le_bytes()
                        });
                }
            }
        }
        if short && !c.p.is_empty() {
            if kind == 1 {
                floats.truncate(offset + (c.p.len() - 1) * stride + 3);
            } else {
                bytes.truncate(offset + (c.p.len() - 1) * stride + 12);
            }
        }
        let p = match kind {
            0 => m::Positions::from_packed(&c.p),
            1 => m::Positions::from_interleaved(&floats, c.p.len(), stride, offset).unwrap(),
            _ => m::Positions::from_bytes(
                &bytes,
                c.p.len(),
                stride,
                offset,
                if kind == 3 {
                    m::ByteOrder::BigEndian
                } else {
                    m::ByteOrder::LittleEndian
                },
            )
            .unwrap(),
        };
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: $bytes,
        });
        let mut output = Vec::new();
        let result = match $api {
            0 => m::optimize_overdraw(&c.indices, p, c.threshold, &mut ws).map(|v| output = v),
            1 => {
                output = vec![0xdeadbeef; c.indices.len() + 7];
                m::optimize_overdraw_into(&mut output, &c.indices, p, c.threshold, &mut ws)
            }
            2 => {
                output = c.indices.clone();
                m::optimize_overdraw_in_place(&mut output, p, c.threshold, &mut ws)
            }
            3 => {
                output = vec![0xdeadbeef; c.indices.len().saturating_sub(1)];
                m::optimize_overdraw_into(&mut output, &c.indices, p, c.threshold, &mut ws)
            }
            _ => unreachable!(),
        }
        .map_err(|e| format!("{e:?}"));
        Observation {
            result,
            output,
            work: ws.usage().work,
            bytes: ws.usage().bytes,
        }
    }};
}
fn check(c: &Case, label: &str, fuels: &[u64], budgets: &[usize], layouts: usize) -> usize {
    let mut n = 0;
    for layout in if layouts == 2 {
        vec![0, 4]
    } else {
        (0..layouts).collect()
    } {
        for api in 0..4 {
            for &fuel in fuels {
                for &bytes in budgets {
                    let a = observe!(candidate, c, layout, api, fuel, bytes);
                    let b = observe!(baseline, c, layout, api, fuel, bytes);
                    // Vec equality compares each output word, including failure prefixes/tails.
                    assert!(a == b, "{label} layout={layout} api={api} fuel={fuel} bytes={bytes}; results {:?}/{:?}, work {}/{}, bytes {}/{}",
                        a.result, b.result, a.work, b.work, a.bytes, b.bytes);
                    n += 1;
                }
            }
        }
    }
    n
}
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn frozen(path: &Path) -> Case {
    let b = fs::read(path).unwrap();
    let nv = u32_at(&b, 0) as usize;
    let ni = u32_at(&b, 4) as usize;
    let p = b[8..8 + nv * 12]
        .chunks_exact(12)
        .map(|v| {
            [
                f32::from_bits(u32_at(v, 0)),
                f32::from_bits(u32_at(v, 4)),
                f32::from_bits(u32_at(v, 8)),
            ]
        })
        .collect();
    let indices = b[8 + nv * 12..8 + nv * 12 + ni * 4]
        .chunks_exact(4)
        .map(|v| u32_at(v, 0))
        .collect();
    let mut at = 8 + nv * 12 + ni * 4;
    for _ in 0..7 {
        at += 4 + u32_at(&b, at) as usize;
    }
    let threshold = if at == b.len() {
        1.05
    } else {
        assert_eq!(u32_at(&b, at), 0x52504c59);
        let raw = u32_at(&b, at + 8) as usize;
        f32::from_bits(u32_at(&b, at + 12 + raw + 12))
    };
    Case {
        p,
        indices,
        threshold,
    }
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }
}
fn main() {
    let mut checks = 0;
    let directory = std::env::args().nth(1).unwrap();
    let mut paths: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "bin")
                && !p.file_name().unwrap().to_str().unwrap().contains("-strip")
                && !p
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("-simplify")
        })
        .collect();
    paths.sort();
    for p in paths {
        let c = frozen(&p);
        let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
        assert!(full.result.is_ok());
        checks += check(
            &c,
            p.to_str().unwrap(),
            &[u64::MAX, full.work.saturating_sub(1), full.work],
            &[usize::MAX],
            2,
        );
        checks += check(
            &c,
            "frozen-byte-limit",
            &[u64::MAX],
            &[0, full.bytes.saturating_sub(1), full.bytes],
            1,
        );
        println!("exact {} {checks}", p.display());
    }
    // Exact generator and seed used by fuzz/src/lib.rs::run(true).
    let seed = 20261002;
    let mut rng = Rng(seed);
    for execution in 0usize..4096 {
        let v = (rng.next() % 65) as usize;
        let n = (rng.next() % 257) as usize;
        let mut p = vec![[0.0; 3]; v];
        for p in &mut p {
            for x in p {
                let b = rng.next();
                *x = if execution.is_multiple_of(4) {
                    f32::from_bits(b)
                } else {
                    (b as i32 % 10000) as f32 / 100.0
                };
            }
        }
        let mut indices = vec![0; n];
        for index in &mut indices {
            *index = if v == 0 {
                0
            } else if execution.is_multiple_of(5) {
                rng.next()
            } else {
                rng.next() % v as u32
            };
        }
        if !execution.is_multiple_of(3) {
            indices.truncate(n / 3 * 3);
        }
        let fuel = if execution.is_multiple_of(7) {
            u64::from(rng.next() % 5000)
        } else {
            1 << 20
        };
        let bytes = if execution.is_multiple_of(11) {
            (rng.next() % 4096) as usize
        } else {
            1 << 20
        };
        let threshold = if execution.is_multiple_of(6) {
            f32::from_bits(rng.next())
        } else {
            [0.0, 0.5, 1.0, 1.05, 1.1, 2.0][(rng.next() % 6) as usize]
        };
        checks += check(
            &Case {
                p,
                indices,
                threshold,
            },
            &format!("seed={seed} execution={execution}"),
            &[fuel],
            &[bytes],
            8,
        );
        // Two layout draws, then ten constructor-mutation draws, as in run(true).
        for _ in 0..12 {
            rng.next();
        }
    }
    // Exhaust every work prefix on finite, nonfinite, overflow and subnormal geometry.
    for scale in [0.0f32, 1.0, 1e-20, 1e19, f32::INFINITY, f32::NAN] {
        for threshold in [
            -1.0,
            0.0,
            0.5,
            1.05,
            2.0,
            f32::MAX / 4.0,
            -f32::MAX / 4.0,
            f32::from_bits((f32::MAX / 4.0).to_bits() + 1),
            f32::MAX / 2.0,
            f32::MAX,
            -f32::MAX,
            f32::NAN,
        ] {
            let c = Case {
                p: vec![[0.0; 3], [scale, 0.0, 0.0], [0.0, scale, 0.0]],
                indices: vec![0, 1, 2, 0, 2, 1],
                threshold,
            };
            let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
            let fuels: Vec<_> = (0..=full.work + 1).collect();
            checks += check(&c, "every-work-prefix", &fuels, &[usize::MAX], 8);
        }
    }
    for n in [1, 2, 4, 8] {
        let mut c = Case {
            p: vec![],
            indices: vec![],
            threshold: 1.05,
        };
        for i in 0..n {
            let b = c.p.len() as u32;
            c.p.extend([[i as f32, 0., 0.], [i as f32, 1., 0.], [i as f32, 0., 1.]]);
            c.indices.extend([b, b + 1, b + 2]);
        }
        for t in [-1.0, 0.5, 1.05, f32::MAX] {
            c.threshold = t;
            let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
            let fuels: Vec<_> = (0..=full.work + 1).collect();
            checks += check(&c, "disconnected-all-prefixes", &fuels, &[usize::MAX], 8);
        }
    }
    let c = Case {
        p: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 1.]],
        indices: vec![0, 1, 2, 1, 3, 2],
        threshold: 1.05,
    };
    let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
    for kind in 1..=3usize {
        let strides: Vec<_> = if kind == 1 {
            (3..=64).collect()
        } else {
            (12..=256).step_by(4).collect()
        };
        for stride in strides {
            for offset in 0..4 {
                for short in [false, true] {
                    let layout =
                        8 + (kind - 1) * 4096 + usize::from(short) * 2048 + offset * 512 + stride;
                    for api in 0..4 {
                        for fuel in [
                            0,
                            16,
                            full.work / 2,
                            full.work - 1,
                            full.work,
                            full.work + 1,
                            u64::MAX,
                        ] {
                            let a = observe!(candidate, &c, layout, api, fuel, usize::MAX);
                            let b = observe!(baseline, &c, layout, api, fuel, usize::MAX);
                            assert_eq!(a,b,"all-layouts kind={kind} stride={stride} offset={offset} short={short} api={api} fuel={fuel}");
                            checks += 1;
                        }
                    }
                }
            }
        }
    }
    println!("PASS: {checks} exact observations; frozen corpus + 4096 existing seed mutations + every small-mesh work prefix");
}
