//! Exact v0.2.14-vs-candidate observations for the attribute validation,
//! normalization and quadric-table paths changed by SPEC 0217.
//!
//! Each cell fixes a six-vertex mesh, an attribute width, an attribute value
//! pattern, a weight pattern, a position/attribute layout, an option mask and
//! an API. Both libraries run unlimited and at every work limit from zero to
//! the largest unlimited usage plus one, from four workspace states (fresh,
//! primed by a successful positional call, primed by a successful attribute
//! call, primed by a failing attribute call), with two consecutive target
//! calls per workspace. Results, output buffers, mutated positions and
//! attributes, Usage.work and Usage.bytes must be identical.
//!
//! Usage: targeted-attributes START END   (cells START..END of `cells()`)

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(usize, u32), String>,
    output: Vec<u32>,
    positions: Vec<u32>,
    attributes: Vec<u32>,
    usage: (u64, usize),
}

#[derive(Clone, Debug)]
struct Cell {
    mesh: usize,
    width: usize,
    values: usize,
    weights: usize,
    layout: usize,
    mask: u32,
    api: usize,
}

const MESHES: usize = 6;
const WIDTHS: [usize; 6] = [1, 3, 4, 8, 9, 32];
const VALUES: usize = 9;
const WEIGHTS: usize = 8;
const LAYOUTS: usize = 3;
const MASKS: [u32; 3] = [0, 2, 32];
const APIS: usize = 4;

fn cells() -> Vec<Cell> {
    let mut out = Vec::new();
    for mesh in 0..MESHES {
        for &width in &WIDTHS {
            for values in 0..VALUES {
                for weights in 0..WEIGHTS {
                    for layout in 0..LAYOUTS {
                        for &mask in &MASKS {
                            for api in 0..APIS {
                                out.push(Cell { mesh, width, values, weights, layout, mask, api });
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

fn positions(mesh: usize) -> Vec<[f32; 3]> {
    match mesh {
        // A fan with a seam-free interior.
        0 => vec![[0., 4., 0.], [0., 1., 0.], [2., 2., 0.], [0., 0., 0.], [1., 0., 0.], [4., 0., 0.]],
        // Coincident positions: welded wedges, seams.
        1 => vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        // Subnormal extent: the reciprocal overflows (non-finite scale).
        2 => vec![
            [0., 0., 0.],
            [f32::from_bits(1), 0., 0.],
            [0., f32::from_bits(1), 0.],
            [f32::from_bits(2), 0., 0.],
            [0., 0., f32::from_bits(1)],
            [0., f32::from_bits(2), 0.],
        ],
        // Large finite extent.
        3 => vec![[0., 0., 0.], [1.0e30, 0., 0.], [-1.0e30, 0., 0.], [0., 1.0e30, 0.], [0., -1.0e30, 0.], [0., 0., 1.]],
        // Overflowing extent (rejected before normalization).
        4 => vec![[0., 0., 0.], [f32::MAX, 0., 0.], [-f32::MAX, 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., -1., 0.]],
        // A NaN position at vertex 3.
        _ => vec![[0., 4., 0.], [0., 1., 0.], [2., 2., 0.], [0., f32::NAN, 0.], [1., 0., 0.], [4., 0., 0.]],
    }
}

const INDICES: [u32; 12] = [0, 2, 1, 1, 2, 3, 3, 2, 4, 2, 5, 4];

fn attribute_values(pattern: usize, width: usize) -> Vec<f32> {
    let mut a: Vec<f32> = (0..6 * width).map(|i| (i % 7) as f32 * 0.125).collect();
    match pattern {
        0 => {}
        1 => a[3 * width + width / 2] = f32::NAN,
        2 => a[5 * width + width - 1] = f32::INFINITY,
        3 => a[0] = f32::NEG_INFINITY,
        4 => a[2 * width] = f32::MAX,
        5 => a.iter_mut().for_each(|x| *x = 1.0e30 * (*x + 1.0)),
        6 => a.iter_mut().enumerate().for_each(|(i, x)| *x = f32::from_bits(i as u32 + 1)),
        7 => a.iter_mut().enumerate().for_each(|(i, x)| *x = if i % 2 == 0 { -0.0 } else { -f32::MAX }),
        _ => a.iter_mut().enumerate().for_each(|(i, x)| *x = (i as f32 - 20.0) * 3.0e18),
    }
    a
}

fn weight_values(pattern: usize, width: usize) -> Vec<f32> {
    let mut w = vec![1.0; width];
    match pattern {
        0 => {}
        1 => w.iter_mut().for_each(|x| *x = 1.0e10),
        2 => w[width - 1] = 0.0,
        3 => w[0] = -0.0,
        4 => w.iter_mut().for_each(|x| *x = f32::MAX),
        5 => w.iter_mut().for_each(|x| *x = f32::from_bits(1)),
        6 => w[width / 2] = f32::NAN,
        _ => w[0] = -1.0,
    }
    w
}

macro_rules! observe {
    ($lib:ident, $c:expr, $fuel:expr, $prime:expr) => {{
        use $lib as m;
        let c: &Cell = $c;
        let mut ws = m::Workspace::new(m::Limits { max_work: $fuel, max_bytes: 1 << 26 });
        let primed = match $prime {
            1 => {
                let pp = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
                let mut out = [0u32; 3];
                let s = m::SimplifySettings { target_index_count: 0, target_error: 1.0, options: m::SimplifyOptions::EMPTY };
                let r = m::simplify_into(&mut out, &[0, 1, 2], m::Positions::from_packed(&pp), s, &mut ws);
                Some((format!("{r:?}"), out.to_vec(), ws.usage().work, ws.usage().bytes))
            }
            2 | 3 => {
                let pp = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
                let mut aa = [0.5f32; 8];
                if $prime == 3 {
                    aa[5] = f32::NAN;
                }
                let mut out = [0u32; 6];
                let s = m::SimplifySettings { target_index_count: 0, target_error: 1.0, options: m::SimplifyOptions::EMPTY };
                let r = m::simplify_with_attributes_into(
                    &mut out,
                    &[0, 1, 2, 2, 1, 3],
                    m::Positions::from_packed(&pp),
                    m::Attributes::from_interleaved(&aa, 4, 2, 2, 0).unwrap(),
                    &[1.0, 2.0],
                    None,
                    s,
                    &mut ws,
                );
                Some((format!("{r:?}"), out.to_vec(), ws.usage().work, ws.usage().bytes))
            }
            _ => None,
        };
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut p = positions(c.mesh);
            let mut pf = vec![f32::NAN; 1 + p.len() * 4];
            for (v, dst) in p.iter().zip(pf[1..].chunks_exact_mut(4)) {
                dst[..3].copy_from_slice(v);
            }
            let mut a = attribute_values(c.values, c.width);
            let stride = c.width + 1;
            let mut af = vec![7.0f32; 2 + 6 * stride];
            for (row, dst) in a.chunks_exact(c.width).zip(af[2..].chunks_exact_mut(stride)) {
                dst[..c.width].copy_from_slice(row);
            }
            let weights = weight_values(c.weights, c.width);
            let s = m::SimplifySettings {
                target_index_count: 3,
                target_error: 1.0,
                options: m::SimplifyOptions::from_bits(c.mask).unwrap(),
            };
            // Layout 0: packed positions, contiguous attributes (prefix scans).
            // Layout 1: strided positions, strided attributes (general loops).
            // Layout 2: packed positions, strided attributes (mixed).
            let packed_p = c.layout != 1;
            let packed_a = c.layout == 0;
            let mut output = match c.api {
                0 => Vec::new(),
                1 => vec![0xdeadbeef; INDICES.len() + 3],
                2 => vec![0xdeadbeef; 0],
                _ => INDICES.to_vec(),
            };
            let result = (|| -> Result<(usize, u32), m::Error> {
                if c.api == 3 {
                    let mut pv = if packed_p {
                        m::PositionsMut::from_packed(&mut p)
                    } else {
                        m::PositionsMut::from_interleaved(&mut pf, 6, 4, 1)?
                    };
                    let mut av = if packed_a {
                        m::AttributesMut::from_interleaved(&mut a, 6, c.width, c.width, 0)?
                    } else {
                        m::AttributesMut::from_interleaved(&mut af, 6, c.width, stride, 2)?
                    };
                    let r = m::simplify_with_update(&mut output, &mut pv, Some(&mut av), &weights, None, s, &mut ws)?;
                    return Ok((r.index_count, r.error.to_bits()));
                }
                let pv = if packed_p {
                    m::Positions::from_packed(&p)
                } else {
                    m::Positions::from_interleaved(&pf, 6, 4, 1)?
                };
                let av = if packed_a {
                    m::Attributes::from_interleaved(&a, 6, c.width, c.width, 0)?
                } else {
                    m::Attributes::from_interleaved(&af, 6, c.width, stride, 2)?
                };
                if c.api == 0 {
                    let r = m::simplify_with_attributes(&INDICES, pv, av, &weights, None, s, &mut ws)?;
                    let result = (r.indices.len(), r.error.to_bits());
                    output = r.indices;
                    Ok(result)
                } else {
                    let r = m::simplify_with_attributes_into(&mut output, &INDICES, pv, av, &weights, None, s, &mut ws)?;
                    Ok((r.index_count, r.error.to_bits()))
                }
            })();
            observations.push(Observation {
                result: result.map_err(|e| format!("{e:?}")),
                output,
                positions: if packed_p {
                    p.iter().flatten().map(|x| x.to_bits()).collect()
                } else {
                    pf.iter().map(|x| x.to_bits()).collect()
                },
                attributes: if packed_a {
                    a.iter().map(|x| x.to_bits()).collect()
                } else {
                    af.iter().map(|x| x.to_bits()).collect()
                },
                usage: (ws.usage().work, ws.usage().bytes),
            });
        }
        (observations, primed)
    }};
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cells = cells();
    if args.len() < 3 {
        println!("cells={}", cells.len());
        return;
    }
    let start: usize = args[1].parse().unwrap();
    let end: usize = args[2].parse::<usize>().unwrap().min(cells.len());
    let mut compared = 0u64;
    let mut primed_ok = [0u64; 4];
    let mut errors = 0u64;
    for c in &cells[start..end] {
        let a = observe!(baseline, c, u64::MAX, 0);
        let b = observe!(candidate, c, u64::MAX, 0);
        assert_eq!(a, b, "unlimited {c:?}");
        compared += 2;
        let success = a.0.iter().map(|o| o.usage.0).max().unwrap();
        for fuel in 0..=success + 1 {
            for prime in 0..4 {
                let a = observe!(baseline, c, fuel, prime);
                let b = observe!(candidate, c, fuel, prime);
                assert_eq!(a, b, "fuel={fuel} prime={prime} {c:?}");
                compared += 2;
                errors += a.0.iter().filter(|o| o.result.is_err()).count() as u64;
                if a.1.as_ref().is_some_and(|p| p.0.starts_with("Ok")) {
                    primed_ok[prime] += 1;
                }
            }
        }
    }
    println!(
        "PASS cells={start}..{end} compared_calls={compared} failed_calls={errors} primed_success={primed_ok:?}"
    );
}
