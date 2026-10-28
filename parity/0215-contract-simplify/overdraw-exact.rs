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
        let mut run = || {
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
        };
        let cold = run();
        let warm = run();
        assert_eq!(
            cold, warm,
            "cold/warm API={} layout={} fuel={}",
            $api, layout, $fuel
        );
        cold
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
    let fuzz = directory == "fuzz";
    let input_directory = if fuzz {
        format!(
            "{}/empty",
            std::env::var("MESHOPT_OVERDRAW_SCRATCH").unwrap()
        )
    } else {
        directory.clone()
    };
    let mut paths: Vec<_> = fs::read_dir(input_directory)
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
    let seed = std::env::args()
        .nth(2)
        .map_or(20261002, |s| s.parse().unwrap());
    let mut rng = Rng(seed);
    for execution in 0usize..if fuzz { 128 } else { 4096 } {
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
        // Ensure the new fixed-record validation path sees varied finite vertices
        // and exact exhaustion prefixes, rather than mostly invalid random indices.
        if execution.is_multiple_of(2) {
            let mut wide = Rng(seed
                .wrapping_add(0x9e37_79b9)
                .wrapping_add(execution as u64));
            let scale =
                [1e-14, 1e-13, 1e-12, 1e-11, 1e-6, 1.0, 1e6, 1e19][(wide.next() % 8) as usize];
            let v = 1 + (wide.next() % 96) as usize;
            let faces = (wide.next() % 96) as usize;
            let mut c = Case {
                p: vec![[0.; 3]; v],
                indices: vec![0; faces * 3],
                threshold,
            };
            for p in &mut c.p {
                for x in p {
                    *x = (wide.next() as i32 % 10000) as f32 / 100. * scale;
                }
            }
            for i in &mut c.indices {
                *i = wide.next() % v as u32;
            }
            if execution.is_multiple_of(8) {
                c.p[v - 1][(wide.next() % 3) as usize] = f32::from_bits(wide.next());
            }
            if execution == 0 {
                let bound = 1073741824.0;
                let faces = match seed % 3 {
                    0 => 2,
                    1 => 3,
                    _ => 4,
                };
                c = Case {
                    p: vec![
                        [-bound, -bound, -bound],
                        [bound, bound, -bound],
                        [bound, -bound, bound],
                    ],
                    indices: (0..faces).flat_map(|_| [0, 1, 2]).collect(),
                    threshold: -1.0,
                };
            }
            let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
            checks += check(
                &c,
                "widened-valid",
                &[
                    u64::MAX,
                    full.work / 2,
                    full.work.saturating_sub(1),
                    full.work,
                    full.work + 1,
                ],
                &[usize::MAX],
                8,
            );
        }
        // Two layout draws, then ten constructor-mutation draws, as in run(true).
        for _ in 0..12 {
            rng.next();
        }
    }
    if fuzz {
        println!("PASS fuzz seed={seed} checks={checks}");
        return;
    }
    // Pin each early-failure position in the fixed-record validation loop.
    for bad_vertex in 0..17 {
        for coordinate in 0..3 {
            for bad in [
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::from_bits(0x7f80_0001),
                f32::from_bits(0xffc1_2345),
            ] {
                let mut c = Case {
                    p: vec![[1., 2., 3.]; 17],
                    indices: vec![0, 1, 2],
                    threshold: 1.05,
                };
                c.p[bad_vertex][coordinate] = bad;
                let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
                checks += check(
                    &c,
                    "each-validation-failure-prefix",
                    &(0..=full.work + 1).collect::<Vec<_>>(),
                    &[usize::MAX],
                    8,
                );
            }
        }
    }
    for count in [7, 8, 9, 15, 16, 17] {
        let c = Case {
            p: vec![[1., 2., 3.]; count],
            indices: vec![0, 1, 2],
            threshold: 1.05,
        };
        let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
        checks += check(
            &c,
            "record-block-every-prefix",
            &(0..=full.work + 1).collect::<Vec<_>>(),
            &[usize::MAX],
            8,
        );
    }
    // Empty topology still validates every supplied vertex, but never checks
    // the accumulated centroid's overflow: preserve that intentional boundary.
    for positions in [
        vec![],
        vec![[f32::MAX; 3]; 9],
        vec![[0., -0., f32::from_bits(1)]; 9],
    ] {
        let c = Case {
            p: positions,
            indices: vec![],
            threshold: 1.05,
        };
        let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
        checks += check(
            &c,
            "empty-all-prefixes",
            &(0..=full.work + 1).collect::<Vec<_>>(),
            &[usize::MAX],
            8,
        );
    }
    // Exhaust every work prefix on finite, nonfinite, overflow and subnormal geometry.
    for scale in [
        0.0f32,
        -0.0,
        f32::from_bits(1),
        1.0,
        1e-20,
        1e-11,
        2e-11,
        1e-10,
        1e19,
        f32::INFINITY,
        f32::NAN,
    ] {
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
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
        ] {
            for indices in [vec![0, 1, 2], vec![0, 1, 2, 0, 2, 1]] {
                let c = Case {
                    p: vec![[0.0; 3], [scale, 0.0, 0.0], [0.0, scale, 0.0]],
                    indices,
                    threshold,
                };
                let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
                let fuels: Vec<_> = (0..=full.work + 1).collect();
                checks += check(&c, "every-work-prefix", &fuels, &[usize::MAX], 8);
            }
        }
    }
    // Straddle the coordinate admission bound, including unreferenced outliers.
    let bound = 1073741824.0f32;
    for magnitude in [
        f32::from_bits(bound.to_bits() - 1),
        bound,
        f32::from_bits(bound.to_bits() + 1),
        -bound,
        f32::MAX,
    ] {
        for outlier in [false, true] {
            let mut c = Case {
                p: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                indices: vec![0, 1, 2],
                threshold: 1.05,
            };
            if outlier {
                c.p.push([magnitude; 3]);
            } else {
                c.p[1][0] = magnitude;
            }
            let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
            checks += check(
                &c,
                "bounded-admission-every-prefix",
                &(0..=full.work + 1).collect::<Vec<_>>(),
                &[usize::MAX],
                8,
            );
        }
    }
    // A bounded mesh can still overflow the accumulated normal squared norm.
    // Never eliminate or move that error. Adjacent counts cover both outcomes.
    for faces in [2, 3, 4] {
        let c = Case {
            p: vec![
                [-bound, -bound, -bound],
                [bound, bound, -bound],
                [bound, -bound, bound],
            ],
            indices: (0..faces).flat_map(|_| [0, 1, 2]).collect(),
            threshold: -1.0,
        };
        let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
        assert_eq!(full.result.is_err(), faces >= 3);
        checks += check(
            &c,
            "bounded-normal-overflow-boundary",
            &(0..=full.work + 1).collect::<Vec<_>>(),
            &[usize::MAX],
            8,
        );
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
    for kind in 0..5 {
        for faces in [1, 2, 8, 17] {
            for threshold in [-1.0, -0.0, 0.5, 1.0, 1.05, 2.0, f32::MAX] {
                let mut c = Case {
                    p: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                    indices: vec![],
                    threshold,
                };
                for face in 0..faces {
                    match kind {
                        0 => c.indices.extend([0, 1, 2]),
                        1 => c.indices.extend([0, 0, 0]),
                        2 => c
                            .indices
                            .extend(if face % 2 == 0 { [0, 1, 2] } else { [2, 1, 0] }),
                        3 => {
                            let b = c.p.len() as u32;
                            c.p.extend([
                                [face as f32, 0., 0.],
                                [face as f32, 1., 0.],
                                [face as f32, 0., 1.],
                            ]);
                            c.indices.extend([b, b + 1, b + 2]);
                        }
                        _ => {
                            let b = c.p.len() as u32;
                            c.p.push([face as f32, 1., 0.]);
                            c.indices.extend([0, 1, b]);
                        }
                    }
                }
                let full = observe!(baseline, &c, 0, 0, u64::MAX, usize::MAX);
                checks += check(
                    &c,
                    "worst-topology-all-prefixes",
                    &(0..=full.work + 1).collect::<Vec<_>>(),
                    &[usize::MAX],
                    8,
                );
            }
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
