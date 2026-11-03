//! v0.2.16/candidate exact contracts for unchanged preprocessing and repair APIs.
use std::fmt::Debug;
#[derive(Clone)]
struct Case {
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
}
#[derive(Debug, PartialEq, Eq)]
struct Observation {
    result: String,
    destination: Vec<u32>,
    work: u64,
    bytes: usize,
}
fn debug<T: Debug, E: Debug>(r: Result<T, E>) -> String {
    format!("{r:?}")
}
macro_rules! arm {
    ($m:ident,$c:expr,$api:expr,$limit:expr,$bytes:expr) => {{
        use $m as m;
        let c: &Case = $c;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $limit,
            max_bytes: $bytes,
        });
        let p = m::Positions::from_packed(&c.positions);
        let mut observations = Vec::new();
        // Cold, repeat (warm or after error), injected buffer error, then retry.
        for attempt in 0..4 {
            let small = attempt == 2;
            let mut out = vec![
                0xdeadbeef;
                if small {
                    0
                } else {
                    c.indices.len() + c.positions.len() + 7
                }
            ];
            let result = match $api {
                0..=15 => debug(m::sanitize(
                    &c.indices,
                    p,
                    m::SanitizeOptions::from_bits($api as u32).unwrap(),
                    &mut ws,
                )),
                16..=31 => debug(m::sanitize_into(
                    &mut out,
                    &c.indices,
                    p,
                    m::SanitizeOptions::from_bits(($api - 16) as u32).unwrap(),
                    &mut ws,
                )),
                32..=35 => debug(m::analyze_mesh(
                    &c.indices,
                    p,
                    m::AnalyzeOptions::EMPTY
                        .with_weld_positions(($api - 32) & 1 != 0)
                        .with_list_limit(if ($api - 32) & 2 != 0 { 100 } else { 0 }),
                    &mut ws,
                )),
                36 => debug(m::generate_position_remap(p, &mut ws)),
                37 => debug(m::generate_position_remap_into(&mut out, p, &mut ws)),
                38 => debug(m::optimize_vertex_cache(
                    &c.indices,
                    c.positions.len(),
                    &mut ws,
                )),
                39 => debug(m::optimize_vertex_cache_into(
                    &mut out,
                    &c.indices,
                    c.positions.len(),
                    &mut ws,
                )),
                40 => debug(m::weld_vertices(
                    p,
                    0.0,
                    m::WeldPolicy::PositionsOnly,
                    &mut ws,
                )),
                41 => debug(m::weld_vertices_into(
                    &mut out,
                    p,
                    0.0,
                    m::WeldPolicy::PositionsOnly,
                    &mut ws,
                )),
                42 => debug(m::orient_faces(&c.indices, c.positions.len(), &mut ws)),
                43 => debug(m::orient_faces_into(
                    &mut out,
                    &c.indices,
                    c.positions.len(),
                    &mut ws,
                )),
                44 => debug(m::fill_holes(&c.indices, p, 6, &mut ws)),
                _ => debug(m::fill_holes_into(&mut out, &c.indices, p, 6, &mut ws)),
            };
            observations.push(Observation {
                result,
                destination: out,
                work: ws.usage().work,
                bytes: ws.usage().bytes,
            });
        }
        observations
    }};
}
fn main() {
    let q = vec![
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [-0., 0., 0.],
    ];
    let mut cases = Vec::new();
    for indices in [
        vec![],
        vec![0, 1, 2],
        vec![0, 1, 2, 0, 3, 2],
        vec![0, 1, 2, 1, 2, 0, 0, 2, 1],
        vec![0, 0, 1, 0, 1, 2],
        vec![0, 1, 99, 0, 1, 2],
        vec![0, 1],
        vec![4, 1, 2, 0, 1, 2],
    ] {
        cases.push(Case {
            positions: q.clone(),
            indices,
        });
    }
    let mut nonfinite = q.clone();
    nonfinite[2][0] = f32::NAN;
    cases.push(Case {
        positions: nonfinite,
        indices: vec![0, 1, 2, 0, 1, 9],
    });
    let mut comparisons = 0u64;
    for (case, c) in cases.iter().enumerate() {
        for api in 0..46 {
            let full = arm!(baseline, c, api, u64::MAX, usize::MAX);
            assert_eq!(
                full,
                arm!(candidate, c, api, u64::MAX, usize::MAX),
                "full case={case} api={api}"
            );
            let work = full.iter().map(|o| o.work).max().unwrap();
            let bytes = full.iter().map(|o| o.bytes).max().unwrap();
            for limit in 0..=work + 1 {
                assert_eq!(
                    arm!(baseline, c, api, limit, usize::MAX),
                    arm!(candidate, c, api, limit, usize::MAX),
                    "case={case} api={api} work={limit}"
                );
                comparisons += 4;
            }
            for limit in [0, 1, bytes.saturating_sub(1), bytes, bytes + 1] {
                assert_eq!(
                    arm!(baseline, c, api, u64::MAX, limit),
                    arm!(candidate, c, api, u64::MAX, limit),
                    "case={case} api={api} bytes={limit}"
                );
                comparisons += 4;
            }
        }
    }
    println!("PASS {comparisons} exact observations; 9 fixtures, 46 API/policy combinations; all work limits, byte boundaries, cold/repeat/after-failure");
}
