//! Byte/Result/Usage observations against a separately named v0.2.11 package.
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
}
macro_rules! observe {
    ($lib:ident, $c:expr, $family:expr, $api:expr, $options:expr, $fuel:expr, $bytes:expr, $lean:expr) => {{
        use $lib as m;
        let c = $c;
        let mut ws = m::Workspace::new(m::Limits { max_work: $fuel, max_bytes: $bytes });
        if $lean { ws.set_retention(m::Retention::Lean); }
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut p = c.p.clone();
            let mut a: Vec<f32> = (0..p.len()*c.ac).map(|i| (i % 7) as f32 * 0.125).collect();
            let weights = vec![c.weight; c.ac];
            let flags: Vec<_> = (0..p.len()).map(|i| m::VertexFlags::from_bits((i%8) as u8).unwrap()).collect();
            let flags = if c.flags { Some(flags.as_slice()) } else { None };
            let settings = m::SimplifySettings {
                target_index_count: c.target,
                target_error: c.error,
                options: m::SimplifyOptions::from_bits($options).unwrap(),
            };
            let mut output = if $family == 2 { c.ix.clone() } else if $api == 0 { Vec::new() }
                else { vec![0xdeadbeef; if $api == 1 { c.ix.len()+p.len()+3 } else { 0 }] };
            let result = (|| -> Result<(usize, u32), m::Error> {
                if $family == 2 {
                    let mut pv = m::PositionsMut::from_packed(&mut p);
                    let mut av = m::AttributesMut::from_interleaved(&mut a, c.p.len(), c.ac, c.ac, 0)?;
                    let r = m::simplify_with_update(&mut output, &mut pv,
                        if c.ac == 0 { None } else { Some(&mut av) }, &weights, flags, settings, &mut ws)?;
                    return Ok((r.index_count, r.error.to_bits()));
                }
                let positions = m::Positions::from_packed(&p);
                let attributes = m::Attributes::from_interleaved(&a, p.len(), c.ac, c.ac, 0)?;
                if $api == 0 {
                    match $family {
                        0 | 1 | 3 => {
                            let r = match $family {
                                0 => m::simplify(&c.ix, positions, settings, &mut ws),
                                1 => m::simplify_with_attributes(&c.ix, positions, attributes, &weights, flags, settings, &mut ws),
                                _ => m::simplify_sloppy(&c.ix, positions, flags, c.target, c.error, &mut ws),
                            }?;
                            let result = (r.indices.len(), r.error.to_bits());
                            output = r.indices;
                            Ok(result)
                        }
                        4 | 5 => {
                            output = if $family == 4 { m::simplify_prune(&c.ix, positions, c.error, &mut ws)? }
                                else { m::simplify_points(positions, if c.flags { Some(positions) } else { None }, c.weight, c.target.min(p.len()), &mut ws)? };
                            Ok((output.len(), 0))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    match $family {
                        0 | 1 | 3 => {
                            let r = match $family {
                                0 => m::simplify_into(&mut output, &c.ix, positions, settings, &mut ws),
                                1 => m::simplify_with_attributes_into(&mut output, &c.ix, positions, attributes, &weights, flags, settings, &mut ws),
                                _ => m::simplify_sloppy_into(&mut output, &c.ix, positions, flags, c.target, c.error, &mut ws),
                            }?;
                            Ok((r.index_count, r.error.to_bits()))
                        }
                        4 => m::simplify_prune_into(&mut output, &c.ix, positions, c.error, &mut ws).map(|n| (n, 0)),
                        5 => m::simplify_points_into(&mut output, positions, if c.flags { Some(positions) } else { None }, c.weight, c.target.min(p.len()), &mut ws).map(|n| (n, 0)),
                        _ => unreachable!(),
                    }
                }
            })();
            observations.push(Observation {
                result: result.map_err(|e| format!("{e:?}")), output,
                positions: p.iter().flatten().map(|x| x.to_bits()).collect(),
                attributes: a.iter().map(|x| x.to_bits()).collect(),
                usage: (ws.usage().work, ws.usage().bytes),
            });
        }
        observations
    }};
}
fn compare(c: &Case, options: u32, exhaustive: bool, bytes: usize, lean: bool) -> u64 {
    let mut checks = 0;
    for family in 0..6 {
        for api in 0..if family == 2 { 1 } else { 3 } {
            let a = observe!(baseline, c, family, api, options, u64::MAX, bytes, lean);
            let b = observe!(candidate, c, family, api, options, u64::MAX, bytes, lean);
            assert_eq!(a, b, "unlimited family={family} api={api} options={options}");
            // Warm allocation reuse can change visit totals. Cover the larger
            // of the two successful/failed unrestricted observations.
            let success = a.iter().map(|o| o.usage.0).max().unwrap();
            let fuels: Vec<_> = if exhaustive { (0..=success+1).collect() }
                else { vec![0, 1, success/2, success.saturating_sub(1), success, success+1] };
            for fuel in fuels {
                let a = observe!(baseline, c, family, api, options, fuel, bytes, lean);
                let b = observe!(candidate, c, family, api, options, fuel, bytes, lean);
                assert_eq!(a, b, "family={family} api={api} options={options} fuel={fuel} bytes={bytes} lean={lean}");
                checks += 2;
            }
        }
    }
    checks
}
// High valence, welded wedges, reversed/duplicate faces, degenerate faces and
// disconnected components stress pick's upper bound and capacity early stop.
fn mesh(kind: usize, n: usize) -> Case {
    let p: Vec<_> = (0..n).map(|i| if kind == 1 { [0.0; 3] } else {
        [(i%3) as f32, (i/3) as f32, (i%2) as f32 * 0.125]
    }).collect();
    let mut ix = Vec::new();
    for i in 1..n.saturating_sub(1) {
        ix.extend_from_slice(&[0, i as u32, i as u32+1]);
        if kind >= 2 { ix.extend_from_slice(&[i as u32+1, i as u32, 0, 0, 0, i as u32]); }
    }
    if kind == 3 {
        ix = (0..n.saturating_sub(2)).step_by(3).flat_map(|i| [i as u32, i as u32+1, i as u32+2]).collect();
    }
    Case { p, ix, ac: if kind == 0 { 0 } else { 3 }, weight: 1.0, error: 1.0, target: 0, flags: kind == 3 }
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 as u32
    }
}
fn main() {
    let args: Vec<_> = env::args().collect();
    let mut checks = 0;
    if args.get(1).is_some_and(|x| x == "fuzz") {
        let seed = args[2].parse().unwrap();
        let mut rng = Rng(seed);
        // Unlike the legacy simplify generator, this reaches the native-root
        // path and prepaid pick with unlimited work, plus their exact boundary.
        for i in 0..64 {
            let mut c = mesh((rng.next()%4) as usize, 3+(rng.next()%30) as usize);
            c.ac = [0, 1, 3, 32][(rng.next()%4) as usize];
            c.target = (rng.next() as usize) % (c.ix.len()+1);
            c.error = [0.0, 0.01, 1.0, f32::MAX, f32::NAN][(rng.next()%5) as usize];
            c.weight = [0.0, 1.0, f32::MAX, f32::INFINITY][(rng.next()%4) as usize];
            for p in &mut c.p {
                for x in p { *x = if i%4 == 0 { f32::from_bits(rng.next()) } else { (rng.next()%128) as f32 * 0.03125 }; }
            }
            if i%7 == 0 { c.ix[0] = u32::MAX; }
            checks += compare(&c, rng.next()%512, false, if i%5 == 0 { (rng.next()%4096) as usize } else { 1<<26 }, i%2 == 0);
        }
        println!("seed={seed} observations={checks}");
        return;
    }
    for kind in 0..4 {
        let c = mesh(kind, 5);
        for options in 0..512 {
            checks += compare(&c, options, true, 1<<26, false);
        }
    }
    for n in [0, 3] {
        checks += compare(&mesh(0, n), 0, true, 1<<26, false);
    }
    // Worst-case picker bounds in both retention modes, every work limit.
    for kind in 0..4 {
        for lean in [false, true] {
            checks += compare(&mesh(kind, 16), 0, true, 1<<26, lean);
        }
    }
    // Error precedence and arithmetic overflow on finite positions; include
    // non-finite validation failures and signed/subnormal special values.
    for bits in [0x7f7fffff, 0xff7fffff, 0x7fc12345, 0x7f800000, 0x80000000, 1] {
        let mut c = mesh(2, 5);
        c.p[0][0] = f32::from_bits(bits);
        for bytes in [0, 1024, 1<<26] {
            checks += compare(&c, 0, true, bytes, false);
        }
    }
    println!("PASS observations={checks}");
}
