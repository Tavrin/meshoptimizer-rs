//! Exact v0.2.15 observations, every work limit on a bounded boundary corpus.
#![allow(dead_code)]
use std::env;

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
    layout: u8,
    attr_layout: u8,
    prime: bool,
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
        if c.prime {
            // Warm retained scratch, then fail after allocation/normalization.
            // Every call resets its work budget; no set_limits clears scratch.
            for fail in [false, true] {
                let edge = if fail { f32::from_bits(1) } else { 1.0 };
                let pp = [[0., 0., 0.], [edge, 0., 0.], [0., edge, 0.]];
                let mut out = [0xdeadbeef; 3];
                let r = m::simplify_into(&mut out, &[0, 1, 2], m::Positions::from_packed(&pp),
                    m::SimplifySettings { target_index_count: 0, target_error: 1., options: m::SimplifyOptions::EMPTY }, &mut ws);
                observations.push(Observation {
                    result: r.map(|r| (r.index_count, r.error.to_bits())).map_err(|e| format!("{e:?}")),
                    output: out.to_vec(),
                    positions: pp.iter().flatten().map(|x| x.to_bits()).collect(),
                    attributes: Vec::new(),
                    usage: (ws.usage().work, ws.usage().bytes),
                });
            }
        }
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
                .map(|i| m::VertexFlags::from_bits((i % 8) as u8).unwrap())
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
fn compare(c: &Case, options: u32, exhaustive: bool, bytes: usize, lean: bool) -> u64 {
    let mut checks = 0;
    // Feature-disabled option bits are not constructible in no-default mode.
    // Compare parser acceptance/errors before sweeping representable settings.
    let a = baseline::SimplifyOptions::from_bits(options)
        .map(|_| ()).map_err(|e| format!("{e:?}"));
    let b = candidate::SimplifyOptions::from_bits(options)
        .map(|_| ()).map_err(|e| format!("{e:?}"));
    assert_eq!(a, b, "option acceptance differs: {options}");
    if a.is_err() {
        return 1;
    }
    for family in 0..6 {
        for api in 0..if family == 2 { 1 } else { 3 } {
            let a = observe!(baseline, c, family, api, options, u64::MAX, bytes, lean);
            let b = observe!(candidate, c, family, api, options, u64::MAX, bytes, lean);
            assert_eq!(
                a, b,
                "unlimited family={family} api={api} options={options}"
            );
            // Warm allocation reuse can change visit totals. Cover the larger
            // of the two successful/failed unrestricted observations.
            let success = a.iter().map(|o| o.usage.0).max().unwrap();
            let fuels: Vec<_> = if exhaustive {
                (0..=success + 1).collect()
            } else {
                vec![
                    0,
                    1,
                    success / 2,
                    success.saturating_sub(1),
                    success,
                    success + 1,
                ]
            };
            for fuel in fuels {
                let a = observe!(baseline, c, family, api, options, fuel, bytes, lean);
                let b = observe!(candidate, c, family, api, options, fuel, bytes, lean);
                assert_eq!(a, b, "family={family} api={api} options={options} fuel={fuel} bytes={bytes} lean={lean}");
                checks += a.len() as u64;
            }
        }
    }
    checks
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
        layout: 0,
        attr_layout: 0,
        prime: false,
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
// Monotone multiplication then bounds every normalized difference d<=extent.
// Check every nonnegative finite extent, including subnormal reciprocals and
// both sides of reciprocal overflow. This is a proof sweep, not a timing arm.
fn finite_normalization_extent_bound() {
    use std::hint::black_box;
    for bits in 0..0x7f80_0000u32 {
        let extent = black_box(f32::from_bits(black_box(bits)));
        let scale = black_box(if extent == 0.0 { 0.0 } else { 1.0 / extent });
        if scale.is_finite() {
            let product = black_box(extent * scale);
            assert!(product.is_finite() && product < 2.0, "extent={bits:08x}");
        }
    }
}
fn main() {
    let mut checks = 0;
    let seed = env::args().nth(1).map(|s| s.parse::<u64>().unwrap());
    if seed.is_none() {
    for options in 0..512 {
        checks += compare(&mesh(0, 5), options, true, 1 << 26, false);
    }
    // The same workspace is called twice: success/reuse and failure/recovery
    // prefixes are compared, including full destination tails and mutable views.
    for kind in 0..6 {
        for lean in [false, true] {
            for options in [0, 1, 2, 4, 16, 32, 64, 128, 256, 511] {
                checks += compare(&mesh(kind, 5), options, true, 1 << 26, lean);
            }
        }
    }
    for layout in 0..8 {
        for ac in [0, 1, 3, 4, 8, 32] {
            let mut c = mesh(2, 5);
            c.layout = layout;
            c.ac = ac;
            c.attr_layout = layout % 3;
            c.prime = true;
            for options in [0, 2, 32, 511] {
                checks += compare(&c, options, true, 1 << 26, false);
            }
        }
    }
    for n in [0, 3, 16, 65] {
        for kind in [0, 1, 4, 5] {
            checks += compare(&mesh(kind, n), 0, true, 1 << 26, false);
        }
    }
    for bits in [0x7f7fffff, 0xff7fffff, 0x7fc12345, 0x7f800000, 0x80000000, 1, 0x001fffff, 0x00200000, 0x00200001] {
        let mut c = mesh(2, 5);
        c.p[1][0] = f32::from_bits(bits);
        for bytes in [0, 1, 128, 1024, 4096, 1 << 26] {
            checks += compare(&c, 0, true, bytes, false);
        }
    }
    }
    let mut rng = Rng(seed.unwrap_or(21640001));
    for i in 0..128 {
        let mut c = mesh((rng.next() % 6) as usize, 3 + (rng.next() % 64) as usize);
        c.layout = (rng.next() % 8) as u8;
        c.ac = [0, 1, 3, 4, 8, 32][(rng.next() % 6) as usize];
        c.attr_layout = (rng.next() % 3) as u8;
        c.target = rng.next() as usize % (c.ix.len() + 1);
        c.error = [0.0, 0.01, 1.0, f32::MAX, f32::NAN][(rng.next() % 5) as usize];
        c.weight = [0.0, 1.0, f32::MAX, f32::INFINITY][(rng.next() % 4) as usize];
        if i % 4 == 0 {
            for v in &mut c.p { for x in v { *x = f32::from_bits(rng.next()); } }
        }
        checks += compare(&c, rng.next() % 512, false, if i % 5 == 0 { 1024 } else { 1 << 26 }, i % 2 == 0);
    }
    println!("PASS observations={checks}");
}
