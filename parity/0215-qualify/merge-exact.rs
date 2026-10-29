//! Targeted exact comparison of the hand-merged normalization region of
//! `src/simplify.rs` on the 0.2.15 train against v0.2.14.
//!
//! The merge put #66's no-attribute packed fast path and #61's attribute
//! prefix path one after the other inside the metered normalization closure,
//! and #61's validation prefix next to #66's packed validation. This driver
//! crosses the inputs that select or skip each path: position storage (packed,
//! mapped by SPARSE, byte views, strided floats, short final records),
//! attribute storage (contiguous floats, strided floats, byte views), every
//! attribute width that has its own unrolled row (1..=8) plus 9 and 32, all
//! positive or mixed zero weights, finite and overflowing scales, non-finite
//! positions and attributes at the first, a middle and the last vertex,
//! attribute products that overflow during normalization, and coordinates
//! whose extent overflows. Each observation compares the Result, the output
//! buffer (including caller-buffer tails), positions and attributes written
//! by simplify_with_update, and Usage (work and bytes). Every work limit from
//! 0 to success + 1 runs on a cold workspace (then reused warm after the
//! limited call) and on a workspace primed by an unlimited call of the same
//! case before the limit is set. Families: 0 simplify, 1
//! simplify_with_attributes, 2 simplify_with_update; APIs: allocating,
//! caller buffer, too-small caller buffer.
//!
//! Usage: merge-exact [quick]. Prints `PASS observations=N cells=M`.
use std::env;

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(usize, u32), String>,
    output: Vec<u32>,
    positions: Vec<u32>,
    attributes: Vec<u32>,
    usage: (u64, usize),
}

#[derive(Clone, Debug)]
struct Case {
    p: Vec<[f32; 3]>,
    ix: Vec<u32>,
    /// `ac` values per vertex.
    a: Vec<f32>,
    weights: Vec<f32>,
    ac: usize,
    error: f32,
    target: usize,
    flags: bool,
    /// 0 packed; 1 bytes LE; 2 bytes BE; 3 bytes LE short final record;
    /// 4 bytes LE offset 1; 5 bytes BE short; 6 strided floats; 7 strided
    /// floats short final record.
    layout: u8,
    /// 0 contiguous floats; 1 strided floats; 2 bytes LE; 3 bytes BE.
    attr_layout: u8,
    options: u32,
}

macro_rules! observe {
    ($lib:ident, $c:expr, $family:expr, $api:expr, $fuel:expr, $prime:expr, $lean:expr) => {{
        use $lib as m;
        let c: &Case = $c;
        let limited = m::Limits {
            max_work: $fuel,
            max_bytes: 1 << 26,
        };
        let mut ws = m::Workspace::new(if $prime {
            m::Limits {
                max_work: u64::MAX,
                max_bytes: 1 << 26,
            }
        } else {
            limited
        });
        if $lean {
            ws.set_retention(m::Retention::Lean);
        }
        let mut observations = Vec::new();
        for round in 0..if $prime { 3 } else { 2 } {
            if $prime && round == 1 {
                ws.set_limits(limited);
            }
            let n = c.p.len();
            let mut p = c.p.clone();
            let mut pf = vec![f32::NAN; 1 + n * 4];
            for (v, dst) in p.iter().zip(pf[1..].chunks_exact_mut(4)) {
                dst[..3].copy_from_slice(v);
            }
            if c.layout == 7 && n != 0 {
                pf.pop();
            }
            let offset = usize::from(c.layout == 4);
            let big = c.layout == 2 || c.layout == 5;
            let mut pb = vec![0xa5u8; offset + n * 64];
            for (v, row) in p.iter().zip(pb[offset..].chunks_exact_mut(64)) {
                for (x, dst) in v.iter().zip(row.chunks_exact_mut(4)) {
                    dst.copy_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() });
                }
            }
            if (c.layout == 3 || c.layout == 5) && n != 0 {
                pb.truncate(pb.len() - 52);
            }
            let order = if big {
                m::ByteOrder::BigEndian
            } else {
                m::ByteOrder::LittleEndian
            };
            let astride = c.ac + 1;
            let mut a = c.a.clone();
            let mut af = vec![f32::NAN; 1 + n * astride];
            for (i, x) in c.a.iter().enumerate() {
                af[1 + i / c.ac * astride + i % c.ac] = *x;
            }
            let mut ab = vec![0xa5u8; 1 + n * astride * 4];
            for (i, x) in c.a.iter().enumerate() {
                let o = 1 + (i / c.ac * astride + i % c.ac) * 4;
                ab[o..o + 4].copy_from_slice(&if c.attr_layout == 3 {
                    x.to_be_bytes()
                } else {
                    x.to_le_bytes()
                });
            }
            let aorder = if c.attr_layout == 3 {
                m::ByteOrder::BigEndian
            } else {
                m::ByteOrder::LittleEndian
            };
            let flags: Vec<_> = (0..n)
                .map(|i| m::VertexFlags::from_bits((i % 8) as u8).unwrap())
                .collect();
            let flags = if c.flags { Some(flags.as_slice()) } else { None };
            let settings = m::SimplifySettings {
                target_index_count: c.target,
                target_error: c.error,
                options: m::SimplifyOptions::from_bits(c.options).unwrap(),
            };
            let limited_round = !$prime || round > 0;
            let mut output = if $family == 2 {
                c.ix.clone()
            } else if $api == 1 && limited_round {
                vec![0xdeadbeef; c.ix.len() + n + 3]
            } else {
                Vec::new()
            };
            let api = if limited_round { $api } else { 0 };
            let result = (|| -> Result<(usize, u32), m::Error> {
                if $family == 2 {
                    let mut pv = match c.layout {
                        0 => m::PositionsMut::from_packed(&mut p),
                        6 | 7 => m::PositionsMut::from_interleaved(&mut pf, n, 4, 1)?,
                        _ => m::PositionsMut::from_bytes(&mut pb, n, 64, offset, order)?,
                    };
                    let mut av = match c.attr_layout {
                        0 => m::AttributesMut::from_interleaved(&mut a, n, c.ac, c.ac, 0)?,
                        1 => m::AttributesMut::from_interleaved(&mut af, n, c.ac, astride, 1)?,
                        _ => m::AttributesMut::from_bytes(&mut ab, n, c.ac, astride * 4, 1, aorder)?,
                    };
                    let r = m::simplify_with_update(
                        &mut output,
                        &mut pv,
                        if c.ac == 0 { None } else { Some(&mut av) },
                        &c.weights,
                        flags,
                        settings,
                        &mut ws,
                    )?;
                    return Ok((r.index_count, r.error.to_bits()));
                }
                let positions = match c.layout {
                    0 => m::Positions::from_packed(&p),
                    6 | 7 => m::Positions::from_interleaved(&pf, n, 4, 1)?,
                    _ => m::Positions::from_bytes(&pb, n, 64, offset, order)?,
                };
                let attributes = match c.attr_layout {
                    0 => m::Attributes::from_interleaved(&a, n, c.ac, c.ac, 0)?,
                    1 => m::Attributes::from_interleaved(&af, n, c.ac, astride, 1)?,
                    _ => m::Attributes::from_bytes(&ab, n, c.ac, astride * 4, 1, aorder)?,
                };
                if api == 0 {
                    let r = if $family == 0 {
                        m::simplify(&c.ix, positions, settings, &mut ws)
                    } else {
                        m::simplify_with_attributes(
                            &c.ix, positions, attributes, &c.weights, flags, settings, &mut ws,
                        )
                    }?;
                    let result = (r.indices.len(), r.error.to_bits());
                    output = r.indices;
                    Ok(result)
                } else {
                    let r = if $family == 0 {
                        m::simplify_into(&mut output, &c.ix, positions, settings, &mut ws)
                    } else {
                        m::simplify_with_attributes_into(
                            &mut output,
                            &c.ix,
                            positions,
                            attributes,
                            &c.weights,
                            flags,
                            settings,
                            &mut ws,
                        )
                    }?;
                    Ok((r.index_count, r.error.to_bits()))
                }
            })();
            observations.push(Observation {
                result: result.map_err(|e| format!("{e:?}")),
                output,
                positions: match c.layout {
                    0 => p.iter().flatten().map(|x| x.to_bits()).collect(),
                    6 | 7 => pf.iter().map(|x| x.to_bits()).collect(),
                    _ => pb.iter().map(|&b| u32::from(b)).collect(),
                },
                attributes: match c.attr_layout {
                    0 => a.iter().map(|x| x.to_bits()).collect(),
                    1 => af.iter().map(|x| x.to_bits()).collect(),
                    _ => ab.iter().map(|&b| u32::from(b)).collect(),
                },
                usage: (ws.usage().work, ws.usage().bytes),
            });
        }
        observations
    }};
}

/// Every API of the family, unlimited and at every work limit 0..=success+1,
/// cold and primed. Returns compared observations.
fn compare(c: &Case, family: usize, exhaustive: bool, lean: bool) -> u64 {
    // Masks with feature-gated bits are rejected identically (checked in
    // main) and have nothing further to compare.
    if baseline::SimplifyOptions::from_bits(c.options).is_err() {
        return 0;
    }
    let mut checks = 0;
    // simplify_with_update has one in-place entry point.
    for api in 0..if family == 2 { 1 } else { 3 } {
        let mut success = 0;
        for prime in [false, true] {
            let a = observe!(baseline, c, family, api, u64::MAX, prime, lean);
            let b = observe!(candidate, c, family, api, u64::MAX, prime, lean);
            assert_eq!(a, b, "unlimited family={family} api={api} prime={prime} {c:?}");
            checks += a.len() as u64;
            success = success.max(a.iter().map(|o| o.usage.0).max().unwrap());
        }
        let fuels: Vec<u64> = if exhaustive {
            (0..=success + 1).collect()
        } else {
            vec![0, 1, success / 2, success.saturating_sub(1), success, success + 1]
        };
        for fuel in fuels {
            for prime in [false, true] {
                let a = observe!(baseline, c, family, api, fuel, prime, lean);
                let b = observe!(candidate, c, family, api, fuel, prime, lean);
                assert_eq!(
                    a, b,
                    "family={family} api={api} fuel={fuel} prime={prime} lean={lean} {c:?}"
                );
                checks += a.len() as u64;
            }
        }
    }
    checks
}

/// Fan plus reversed, duplicate and degenerate faces over `n` vertices on a
/// small grid, so simplification makes progress and also meets bad faces.
fn mesh(n: usize, ac: usize) -> Case {
    let p = (0..n)
        .map(|i| [(i % 3) as f32, (i / 3) as f32, (i % 2) as f32 * 0.125])
        .collect();
    let mut ix = Vec::new();
    for i in 1..n.saturating_sub(1) {
        let i = i as u32;
        ix.extend_from_slice(&[0, i, i + 1, i + 1, i, 0, 0, 0, i]);
    }
    Case {
        p,
        ix,
        a: (0..n * ac).map(|i| (i % 7) as f32 * 0.125).collect(),
        weights: vec![1.0; ac],
        ac,
        error: 1.0,
        target: 0,
        flags: false,
        layout: 0,
        attr_layout: 0,
        options: 0,
    }
}

/// Input variants around the two normalization fast paths and the two
/// validation paths.
fn variant(c: &mut Case, v: usize) -> bool {
    let n = c.p.len();
    let last = n - 1;
    match v {
        // Finite scale, accepted everywhere.
        0 => {}
        // Reciprocal overflow on either side of the finite boundary.
        1 | 2 | 3 => {
            c.p.fill([0.0; 3]);
            c.p[1][0] = f32::from_bits([1, 0x0020_0000, 0x0020_0001][v - 1]);
        }
        // Non-finite position at the first, a middle and the last vertex.
        4 | 5 | 6 => c.p[[0, n / 2, last][v - 4]][1] = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY][v - 4],
        // Non-finite attribute at the first, a middle and the last vertex.
        7 | 8 | 9 => {
            if c.ac == 0 {
                return false;
            }
            let k = [0, n / 2, last][v - 7];
            c.a[k * c.ac + c.ac - 1] = [f32::INFINITY, f32::NAN, f32::NEG_INFINITY][v - 7];
        }
        // Finite attribute whose weighted product overflows during
        // normalization at the first, a middle or the last vertex.
        10 | 11 | 12 => {
            if c.ac == 0 {
                return false;
            }
            let k = [0, n / 2, last][v - 10];
            c.weights.fill(f32::MAX / 8.0);
            c.a[k * c.ac] = -16.0;
        }
        // A zero weight: the attribute path is not dense.
        13 => {
            if c.ac == 0 {
                return false;
            }
            c.weights[0] = 0.0;
        }
        // Finite coordinates whose extent overflows.
        14 => {
            c.p[0] = [f32::MAX, 0.0, 0.0];
            c.p[last] = [-f32::MAX, 0.0, 0.0];
        }
        // Near-maximal finite extent with a finite reciprocal.
        15 => {
            c.p[0] = [f32::MAX, 1.0, 0.0];
            c.p[last] = [0.0, -1.0, 0.0];
        }
        _ => unreachable!(),
    }
    true
}

const VARIANTS: usize = 16;

fn main() {
    let quick = env::args().nth(1).is_some_and(|a| a == "quick");
    for options in 0..512 {
        assert_eq!(
            baseline::SimplifyOptions::from_bits(options).is_ok(),
            candidate::SimplifyOptions::from_bits(options).is_ok(),
            "option mask acceptance differs: {options}"
        );
    }
    let mut checks = 0u64;
    let mut cells = 0u64;
    let widths: &[usize] = if quick { &[0, 3] } else { &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
    // Grid A: every work limit, every variant, every storage pair.
    for &ac in widths {
        for layout in 0..8u8 {
            for attr_layout in 0..if ac == 0 { 1 } else { 4 } {
                for v in 0..VARIANTS {
                    for options in [0u32, 2, 256] {
                        for flags in [false, true] {
                            let mut c = mesh(6, ac);
                            c.layout = layout;
                            c.attr_layout = attr_layout;
                            c.options = options;
                            c.flags = flags;
                            if !variant(&mut c, v) {
                                continue;
                            }
                            for family in 0..3 {
                                if family == 0 && (ac != 0 || flags) {
                                    continue;
                                }
                                checks += compare(&c, family, true, false);
                                cells += 1;
                            }
                        }
                    }
                }
            }
        }
        if quick {
            break;
        }
    }
    // Grid B: the generic (non-unrolled) row width, both retention modes.
    for layout in [0u8, 1, 6] {
        for attr_layout in [0u8, 2] {
            for v in 0..VARIANTS {
                for lean in [false, true] {
                    let mut c = mesh(6, 32);
                    c.layout = layout;
                    c.attr_layout = attr_layout;
                    if !variant(&mut c, v) {
                        continue;
                    }
                    for family in 1..3 {
                        checks += compare(&c, family, !quick, lean);
                        cells += 1;
                    }
                }
            }
        }
    }
    // Grid C: every option mask on accepted and rejected inputs, boundary
    // work limits; SPARSE maps packed positions so the packed paths skip.
    for options in 0..512u32 {
        if baseline::SimplifyOptions::from_bits(options).is_err() {
            continue;
        }
        for (layout, ac, v) in [(0u8, 0usize, 0usize), (0, 3, 0), (6, 3, 0), (0, 3, 1), (0, 3, 11), (0, 0, 5)] {
            let mut c = mesh(7, ac);
            c.layout = layout;
            c.options = options;
            c.target = 6;
            c.error = 0.5;
            if !variant(&mut c, v) {
                continue;
            }
            for family in 0..3 {
                if family == 0 && ac != 0 {
                    continue;
                }
                checks += compare(&c, family, false, false);
                cells += 1;
            }
        }
        if quick && options > 8 {
            break;
        }
    }
    // Empty and single-triangle meshes.
    for n in [0usize, 3] {
        for ac in [0usize, 3] {
            let mut c = mesh(n.max(3), ac);
            if n == 0 {
                c.p.clear();
                c.a.clear();
                c.ix.clear();
            } else {
                c.ix = vec![0, 1, 2];
            }
            for family in 0..3 {
                checks += compare(&c, family, true, false);
                cells += 1;
            }
        }
    }
    println!("PASS observations={checks} cells={cells}");
}
