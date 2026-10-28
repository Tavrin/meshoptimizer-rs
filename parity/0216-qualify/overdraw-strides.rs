//! Scratch consumer comparing two separately compiled crate versions.

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
fn main() {
    let mut cases = Vec::new();
    let mut p = Vec::new();
    let mut indices = Vec::new();
    for i in 0..24 {
        let x = (i % 6) as f32 * 10.;
        let y = (i / 6) as f32 * 10.;
        p.extend([[x, y, 0.], [x + 1., y, 0.], [x, y + 1., 0.]]);
        indices.extend([3 * i, 3 * i + 1, 3 * i + 2]);
    }
    cases.push(Case { p: p.clone(), indices: indices.clone(), threshold: 1.05 });
    cases.push(Case { p: vec![[0.; 3]; p.len()], indices: indices.clone(), threshold: 1.05 });
    cases.push(Case { p: vec![[0.; 3]; 3], indices: vec![0, 1, 2], threshold: 1.05 });
    p[0][0] = 1.0e9;
    cases.push(Case { p, indices, threshold: 1.05 });
    let mut checks = 0u64;
    for (case, c) in cases.iter().enumerate() {
        for kind in 1..=3usize {
            let strides: Vec<_> = if kind == 1 { (3..=64).collect() } else { (12..=256).step_by(4).collect() };
            for stride in strides {
                for offset in 0..4 {
                    for short in [false, true] {
                        let layout = 8 + (kind - 1) * 4096 + usize::from(short) * 2048 + offset * 512 + stride;
                        for api in 0..4 {
                            let full = observe!(baseline, c, layout, api, u64::MAX, usize::MAX);
                            let other = observe!(candidate, c, layout, api, u64::MAX, usize::MAX);
                            assert_eq!(full, other, "unlimited case={case} layout={layout} api={api}");
                            for fuel in 0..=full.work + 1 {
                                let a = observe!(baseline, c, layout, api, fuel, usize::MAX);
                                let b = observe!(candidate, c, layout, api, fuel, usize::MAX);
                                assert_eq!(a, b, "case={case} layout={layout} api={api} fuel={fuel}");
                                checks += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("case={case} cumulative_observations={checks}");
    }
    println!("PASS observations={checks}");
}
