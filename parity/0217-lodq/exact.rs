// Existing simplify/chain behavior: base versus opt-in candidate.
#[derive(Debug, PartialEq)]
struct Observation {
    result: String,
    output: Vec<u32>,
    usage: (usize, u64),
}
macro_rules! observe {
    ($m:ident,$p:expr,$ix:expr,$api:expr,$fuel:expr,$bytes:expr,$prime:expr) => {{
        use $m as m;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: $bytes,
        });
        let p = m::Positions::from_packed($p);
        let ix: &[u32] = $ix;
        if $prime == 1 {
            let _ = m::optimize_vertex_cache(ix, $p.len(), &mut ws);
        }
        if $prime == 2 {
            let _ = m::simplify(
                &[u32::MAX; 3],
                p,
                m::SimplifySettings {
                    target_index_count: 0,
                    target_error: 1.,
                    options: m::SimplifyOptions::EMPTY,
                },
                &mut ws,
            );
        }
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut output = vec![0xa5a5a5a5; ix.len() + 3];
            let settings = m::SimplifySettings {
                target_index_count: ix.len() / 6 * 3,
                target_error: 1.,
                options: m::SimplifyOptions::EMPTY,
            };
            let result = match $api {
                0 => format!("{:?}", m::simplify(ix, p, settings, &mut ws)),
                1 => format!(
                    "{:?}",
                    m::simplify_into(&mut output, ix, p, settings, &mut ws)
                ),
                _ => {
                    let targets = [ix.len() / 6 * 3, 0];
                    let mut options = m::LodChainOptions::new(
                        m::LodTargets::Counts(&targets),
                        m::LodErrors::Uniform(1.),
                    );
                    options.independent = $api == 3;
                    format!("{:?}", m::build_lod_chain(ix, p, &options, &mut ws))
                }
            };
            let u = ws.usage();
            observations.push(Observation {
                result,
                output,
                usage: (u.bytes, u.work),
            });
        }
        observations
    }};
}
fn main() {
    let mut comparisons = 0;
    for n in [1u32, 2, 3] {
        let p: Vec<_> = (0..=n)
            .flat_map(|y| (0..=n).map(move |x| [x as f32, y as f32, ((x + y) % 3) as f32 * 0.1]))
            .collect();
        let ix: Vec<_> = (0..n)
            .flat_map(|y| {
                (0..n).flat_map(move |x| {
                    let a = y * (n + 1) + x;
                    [a, a + 1, a + n + 1, a + 1, a + n + 2, a + n + 1]
                })
            })
            .collect();
        for variant in 0..4 {
            let mut input = ix.clone();
            match variant {
                1 => input.push(0),
                2 => input[0] = u32::MAX,
                3 => input.clear(),
                _ => (),
            };
            for api in 0..4 {
                let full = observe!(baseline, &p, &input, api, u64::MAX, usize::MAX, 0);
                let max = full.iter().map(|o| o.usage.1).max().unwrap();
                let bytes = full.iter().map(|o| o.usage.0).max().unwrap();
                for prime in 0..3 {
                    for fuel in 0..=max + 1 {
                        let a = observe!(baseline, &p, &input, api, fuel, usize::MAX, prime);
                        let b = observe!(candidate, &p, &input, api, fuel, usize::MAX, prime);
                        assert_eq!(
                            a, b,
                            "n={n} v={variant} api={api} fuel={fuel} prime={prime}"
                        );
                        comparisons += 2;
                    }
                    for limit in [0, bytes.saturating_sub(1), bytes, bytes + 1] {
                        assert_eq!(
                            observe!(baseline, &p, &input, api, u64::MAX, limit, prime),
                            observe!(candidate, &p, &input, api, u64::MAX, limit, prime)
                        );
                        comparisons += 2;
                    }
                }
            }
        }
    }
    println!("exact base/candidate observations={comparisons}; every work limit, byte boundaries, cold/warm/after-failure; differences=0");
}
