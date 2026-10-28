//! Byte/Result/Usage observations against a separately named v0.2.13 package.

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(usize, u32), String>,
    output: Vec<u32>,
    positions: Vec<u32>,
    attributes: Vec<u32>,
    usage: (u64, usize),
}
struct Case {
    p: Vec<[f32; 3]>,
    ix: Vec<u32>,
    ac: usize,
    weight: f32,
    error: f32,
    target: usize,
    flags: bool,
    flag_values: Vec<u8>,
    layout: u8,
    attr_layout: u8,
}
macro_rules! observe {
    ($lib:ident, $c:expr, $family:expr, $api:expr, $options:expr, $fuel:expr, $bytes:expr, $lean:expr) => {{
        use $lib as m;
        let c = $c;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: $bytes,
        });
        if $lean {
            ws.set_retention(m::Retention::Lean);
        }
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut p = c.p.clone();
            let mut pf = vec![f32::NAN; 1 + p.len() * 4];
            for (v, dst) in p.iter().zip(pf[1..].chunks_exact_mut(4)) {
                dst[..3].copy_from_slice(v);
            }
            if c.layout == 7 && !p.is_empty() {
                pf.pop();
            }
            let offset = usize::from(c.layout == 4);
            let mut pb = vec![0xa5u8; offset + p.len() * 64];
            for (v, row) in p.iter().zip(pb[offset..].chunks_exact_mut(64)) {
                for (x, dst) in v.iter().zip(row.chunks_exact_mut(4)) {
                    dst.copy_from_slice(&if c.layout == 2 || c.layout == 5 {
                        x.to_be_bytes()
                    } else {
                        x.to_le_bytes()
                    });
                }
            }
            let order = if c.layout == 2 || c.layout == 5 {
                m::ByteOrder::BigEndian
            } else {
                m::ByteOrder::LittleEndian
            };
            // Full records, legal short final records, and unaligned starts
            // must reach both the specialized scan and its general fallback.
            if (c.layout == 3 || c.layout == 5) && !p.is_empty() {
                pb.truncate(pb.len() - 52);
            }
            let mut a: Vec<f32> = (0..p.len() * c.ac)
                .map(|i| (i % 7) as f32 * 0.125)
                .collect();
            let astride = c.ac + 1;
            let mut ab = vec![0xa5; 1 + p.len() * astride * 4];
            for (i, x) in a.iter().enumerate() {
                let offset = 1 + (i / c.ac * astride + i % c.ac) * 4;
                ab[offset..offset + 4].copy_from_slice(&if c.attr_layout == 2 {
                    x.to_be_bytes()
                } else {
                    x.to_le_bytes()
                });
            }
            let aorder = if c.attr_layout == 2 {
                m::ByteOrder::BigEndian
            } else {
                m::ByteOrder::LittleEndian
            };
            let weights = vec![c.weight; c.ac];
            let flags: Vec<_> = (0..p.len())
                .map(|i| m::VertexFlags::from_bits(c.flag_values[i]).unwrap())
                .collect();
            let flags = if c.flags {
                Some(flags.as_slice())
            } else {
                None
            };
            let settings = m::SimplifySettings {
                target_index_count: c.target,
                target_error: c.error,
                options: m::SimplifyOptions::from_bits($options).unwrap(),
            };
            let mut output = if $family == 2 {
                c.ix.clone()
            } else if $api == 0 {
                Vec::new()
            } else {
                vec![
                    0xdeadbeef;
                    if $api == 1 {
                        c.ix.len() + p.len() + 3
                    } else {
                        0
                    }
                ]
            };
            let result = (|| -> Result<(usize, u32), m::Error> {
                if $family == 2 {
                    let mut pv = if c.layout == 0 {
                        m::PositionsMut::from_packed(&mut p)
                    } else if c.layout >= 6 {
                        m::PositionsMut::from_interleaved(&mut pf, c.p.len(), 4, 1)?
                    } else {
                        m::PositionsMut::from_bytes(&mut pb, c.p.len(), 64, offset, order)?
                    };
                    let mut av = if c.attr_layout == 0 {
                        m::AttributesMut::from_interleaved(&mut a, c.p.len(), c.ac, c.ac, 0)?
                    } else {
                        m::AttributesMut::from_bytes(
                            &mut ab,
                            c.p.len(),
                            c.ac,
                            astride * 4,
                            1,
                            aorder,
                        )?
                    };
                    let r = m::simplify_with_update(
                        &mut output,
                        &mut pv,
                        if c.ac == 0 { None } else { Some(&mut av) },
                        &weights,
                        flags,
                        settings,
                        &mut ws,
                    )?;
                    return Ok((r.index_count, r.error.to_bits()));
                }
                let positions = if c.layout == 0 {
                    m::Positions::from_packed(&p)
                } else if c.layout >= 6 {
                    m::Positions::from_interleaved(&pf, p.len(), 4, 1)?
                } else {
                    m::Positions::from_bytes(&pb, p.len(), 64, offset, order)?
                };
                let attributes = if c.attr_layout == 0 {
                    m::Attributes::from_interleaved(&a, p.len(), c.ac, c.ac, 0)?
                } else {
                    m::Attributes::from_bytes(&ab, p.len(), c.ac, astride * 4, 1, aorder)?
                };
                if $api == 0 {
                    match $family {
                        0 | 1 | 3 => {
                            let r = match $family {
                                0 => m::simplify(&c.ix, positions, settings, &mut ws),
                                1 => m::simplify_with_attributes(
                                    &c.ix, positions, attributes, &weights, flags, settings,
                                    &mut ws,
                                ),
                                _ => m::simplify_sloppy(
                                    &c.ix, positions, flags, c.target, c.error, &mut ws,
                                ),
                            }?;
                            let result = (r.indices.len(), r.error.to_bits());
                            output = r.indices;
                            Ok(result)
                        }
                        4 | 5 => {
                            output = if $family == 4 {
                                m::simplify_prune(&c.ix, positions, c.error, &mut ws)?
                            } else {
                                m::simplify_points(
                                    positions,
                                    if c.flags { Some(positions) } else { None },
                                    c.weight,
                                    c.target.min(p.len()),
                                    &mut ws,
                                )?
                            };
                            Ok((output.len(), 0))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    match $family {
                        0 | 1 | 3 => {
                            let r = match $family {
                                0 => m::simplify_into(
                                    &mut output,
                                    &c.ix,
                                    positions,
                                    settings,
                                    &mut ws,
                                ),
                                1 => m::simplify_with_attributes_into(
                                    &mut output,
                                    &c.ix,
                                    positions,
                                    attributes,
                                    &weights,
                                    flags,
                                    settings,
                                    &mut ws,
                                ),
                                _ => m::simplify_sloppy_into(
                                    &mut output,
                                    &c.ix,
                                    positions,
                                    flags,
                                    c.target,
                                    c.error,
                                    &mut ws,
                                ),
                            }?;
                            Ok((r.index_count, r.error.to_bits()))
                        }
                        4 => {
                            m::simplify_prune_into(&mut output, &c.ix, positions, c.error, &mut ws)
                                .map(|n| (n, 0))
                        }
                        5 => m::simplify_points_into(
                            &mut output,
                            positions,
                            if c.flags { Some(positions) } else { None },
                            c.weight,
                            c.target.min(p.len()),
                            &mut ws,
                        )
                        .map(|n| (n, 0)),
                        _ => unreachable!(),
                    }
                }
            })();
            observations.push(Observation {
                result: result.map_err(|e| format!("{e:?}")),
                output,
                positions: if c.layout == 0 {
                    p.iter().flatten().map(|x| x.to_bits()).collect()
                } else if c.layout >= 6 {
                    pf.iter().map(|x| x.to_bits()).collect()
                } else {
                    pb.iter().map(|&b| u32::from(b)).collect()
                },
                attributes: if c.attr_layout == 0 {
                    a.iter().map(|x| x.to_bits()).collect()
                } else {
                    ab.iter().map(|&x| u32::from(x)).collect()
                },
                usage: (ws.usage().work, ws.usage().bytes),
            });
        }
        observations
    }};
}
fn compare(c: &Case, options: u32, counts: &mut [u64; 3], checkpoint: &mut Checkpoint) -> bool {
    let bytes = 1 << 26;
    let lean = false;
    let base_valid = baseline::SimplifyOptions::from_bits(options).is_ok();
    let candidate_valid = candidate::SimplifyOptions::from_bits(options).is_ok();
    assert_eq!(base_valid, candidate_valid, "option mask acceptance differs: {options}");
    if !base_valid {
        return true;
    }
    for family in 0..3 {
        for api in 0..if family == 2 { 1 } else { 3 } {
            let a = observe!(baseline, c, family, api, options, u64::MAX, bytes, lean);
            let b = observe!(candidate, c, family, api, options, u64::MAX, bytes, lean);
            assert_eq!(a, b, "unlimited family={family} api={api} options={options} n={} width={} layout={} attr_layout={} flags={:?}", c.p.len(), c.ac, c.layout, c.attr_layout, c.flag_values);
            counts[family] += 2;
            let success = a.iter().map(|o| o.usage.0).max().unwrap();
            for fuel in 0..=success + 1 {
                let a = observe!(baseline, c, family, api, options, fuel, bytes, lean);
                let b = observe!(candidate, c, family, api, options, fuel, bytes, lean);
                assert_eq!(a, b, "family={family} api={api} options={options} fuel={fuel} n={} width={} layout={} attr_layout={} flags={:?}", c.p.len(), c.ac, c.layout, c.attr_layout, c.flag_values);
                counts[family] += 2;
                if checkpoint.poll(counts) { return false; }
            }
        }
    }
    true
}
struct Checkpoint {
    start: std::time::Instant,
    last: std::time::Instant,
    limit_seconds: u64,
    completed_cases: u64,
    planned_cases: u64,
}
impl Checkpoint {
    fn poll(&mut self, counts: &[u64; 3]) -> bool {
        let now = std::time::Instant::now();
        let done = now.duration_since(self.start).as_secs() >= self.limit_seconds;
        if done || now.duration_since(self.last).as_secs() >= 10 {
            println!("progress completed_cases={} planned_cases={} observations={},{},{} elapsed_seconds={}", self.completed_cases, self.planned_cases, counts[0], counts[1], counts[2], now.duration_since(self.start).as_secs());
            self.last = now;
        }
        done
    }
}
// High valence, welded wedges, reversed/duplicate faces, degenerate faces and
// disconnected components stress pick's upper bound and capacity early stop.
fn mesh(kind: usize, n: usize) -> Case {
    let mut p: Vec<_> = (0..n)
        .map(|i| {
            if kind == 1 {
                [0.0; 3]
            } else if kind == 4 {
                // Distinct finite values with identical low hash bucket bits.
                [f32::from_bits(0x3f800000 + ((i as u32) << 10)), 0.0, 0.0]
            } else if kind == 5 {
                [(i % 3) as f32, (i % 2) as f32, 0.0]
            } else {
                [(i % 3) as f32, (i / 3) as f32, (i % 2) as f32 * 0.125]
            }
        })
        .collect();
    if kind == 4 {
        let mask = (n + n / 4).next_power_of_two() - 1;
        let mut bits = 0x3f800000u32;
        for v in &mut p {
            while (((bits ^ (bits >> 17)).wrapping_mul(73856093)) as usize & mask) != 0 {
                bits += 1;
            }
            *v = [f32::from_bits(bits), 0.0, 0.0];
            bits += 1;
        }
    }
    let mut ix = Vec::new();
    for i in 1..n.saturating_sub(1) {
        ix.extend_from_slice(&[0, i as u32, i as u32 + 1]);
        if kind >= 2 {
            ix.extend_from_slice(&[i as u32 + 1, i as u32, 0, 0, 0, i as u32]);
        }
    }
    if kind == 3 {
        ix = (0..n.saturating_sub(2))
            .step_by(3)
            .flat_map(|i| [i as u32, i as u32 + 1, i as u32 + 2])
            .collect();
    }
    Case {
        p,
        ix,
        ac: if kind == 0 { 0 } else { 3 },
        weight: 1.0,
        error: 1.0,
        target: 0,
        flags: kind == 3,
        flag_values: vec![0; n],
        layout: 0,
        attr_layout: 0,
    }
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { (a, b) = (b, a % b); }
    a
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let start: usize = args[1].parse().unwrap();
    let end: usize = args[2].parse().unwrap();
    assert!(start < end && end <= 384);
    let masks = [0u32, 1, 2, 4, 16, 32, 3, 5, 17, 33, 6, 18, 34, 20, 36, 48];
    let mut counts = [0u64; 3];
    let now = std::time::Instant::now();
    let mut checkpoint = Checkpoint { start: now, last: now, limit_seconds: 6900, completed_cases: 0, planned_cases: (end - start) as u64 };
    for cell in start..end {
        let kind = cell / 48;
        let width = [0, 3, 32][cell / 16 % 3];
        let mask = masks[cell % 16];
        let mut c = mesh(0, 6);
        c.ac = width;
        c.p = match kind {
            0 => vec![[0.,0.,0.], [f32::MAX/2.,0.,0.], [-f32::MAX/2.,0.,0.], [0.,f32::MAX/2.,0.], [0.,-f32::MAX/2.,0.], [0.,0.,1.]],
            1 => vec![[0.,0.,0.], [1.0e8,0.,0.], [-1.0e8,0.,0.], [0.,1.0e8,0.], [0.,-1.0e8,0.], [0.,0.,1.]],
            2 => vec![[0.,0.,0.], [f32::from_bits(1),0.,0.], [0.,f32::from_bits(1),0.], [f32::from_bits(2),0.,0.], [0.,0.,f32::from_bits(1)], [0.,f32::from_bits(2),0.]],
            3 => vec![[0.,0.,0.], [1.0e8,0.,0.], [0.,1.0e-20,0.], [1.0e8,1.0e-20,0.], [0.,2.0e-20,0.], [1.0e8,2.0e-20,0.]],
            4 => vec![[0.;3];6],
            5 => (0..6).map(|i| [i as f32,0.,0.]).collect(),
            6 => vec![[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [0.,0.,0.], [1.,0.,0.], [0.,1.,0.]],
            7 => vec![[0.,0.,0.], [1.0e8+8.,0.,0.], [-1.0e8-8.,0.,0.], [0.,1.,0.], [0.,0.,1.], [0.,-1.,0.]],
            _ => unreachable!(),
        };
        if !compare(&c, mask, &mut counts, &mut checkpoint) {
            println!("TIME_LIMIT cell={cell} kind={kind} width={width} mask={mask}"); std::process::exit(124);
        }
        checkpoint.completed_cases += 1;
        println!("cell={cell} observations={counts:?}");
    }
    println!("PASS completed_cases={} observations={counts:?}", checkpoint.completed_cases);
}
