//! Compare every observable result against the separately named baseline crate.
#[derive(Debug, PartialEq, Eq)]
struct Observation {
    result: Result<usize, String>,
    output: Vec<u32>,
    bytes: usize,
    work: u64,
}
struct Case {
    indices: Vec<u32>,
    counts: Vec<u32>,
    positions: Vec<[f32; 3]>,
    target: usize,
}
macro_rules! observe {
    ($lib:ident, $case:expr, $spatial:expr, $api:expr, $short:expr, $work:expr, $bytes:expr) => {{
        use $lib as m;
        let c = $case;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $work,
            max_bytes: $bytes,
        });
        let mut out = vec![
            0xdeadbeef;
            if $short {
                c.counts.len().saturating_sub(1)
            } else {
                c.counts.len() + 5
            }
        ];
        let p = if $spatial {
            Some(m::Positions::from_packed(&c.positions))
        } else {
            None
        };
        let result = if $api == 0 {
            m::partition_clusters(
                &c.indices,
                &c.counts,
                c.positions.len(),
                p,
                c.target,
                &mut ws,
            )
            .map(|r| {
                out = r.assignments;
                r.count
            })
        } else {
            m::partition_clusters_into(
                &mut out,
                &c.indices,
                &c.counts,
                c.positions.len(),
                p,
                c.target,
                &mut ws,
            )
        };
        let usage = ws.usage();
        Observation {
            result: result.map_err(|e| format!("{e:?}")),
            output: out,
            bytes: usage.bytes,
            work: usage.work,
        }
    }};
}
fn fuzz(seed: u64) {
    let mut state = seed | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for id in 0..2000 {
        let n = if id % 100 == 0 {
            260
        } else {
            (next() % 32 + 1) as usize
        };
        let groups = if id % 100 == 0 {
            270
        } else {
            (next() % 40 + 1) as usize
        };
        let mut indices = Vec::new();
        let mut counts = Vec::new();
        for _ in 0..groups {
            let len = (next() % 12 + 1) as usize;
            counts.push(len as u32);
            for _ in 0..len {
                indices.push((next() % n as u64) as u32);
            }
        }
        let positions = (0..n)
            .map(|_| {
                let mut p = [0.; 3];
                for v in &mut p {
                    *v = (next() % 17) as f32 - 8.;
                }
                if next() % 100 == 0 {
                    p[0] = f32::MAX;
                }
                p
            })
            .collect();
        let c = Case {
            indices,
            counts,
            positions,
            target: if id % 10 == 0 {
                100_000
            } else {
                (next() % 14 + 1) as usize
            },
        };
        let spatial = next() & 1 != 0;
        let api = (next() & 1) as usize;
        let short = api == 1 && next() % 20 == 0;
        let full = observe!(baseline, &c, spatial, api, short, u64::MAX, usize::MAX);
        let work = match next() % 4 {
            0 => u64::MAX,
            _ => next() % (full.work + 2),
        };
        let bytes = match next() % 4 {
            0 => usize::MAX,
            _ => (next() as usize) % (full.bytes + 2),
        };
        let a = observe!(baseline, &c, spatial, api, short, work, bytes);
        let b = observe!(candidate, &c, spatial, api, short, work, bytes);
        assert_eq!(a, b, "seed={seed} case={id} spatial={spatial} api={api} short={short} work={work} bytes={bytes}");
    }
    println!("PASS seed={seed} cases=2000");
}
fn main() {
    if let Some(seed) = std::env::args().nth(1) {
        fuzz(seed.parse().unwrap());
        return;
    }
    let cases = [
        Case {
            indices: vec![0, 1, 2, 2, 1, 3, 2, 3, 4, 4, 3, 5],
            counts: vec![3; 4],
            positions: (0..6).map(|i| [i as f32, (i % 2) as f32, 0.]).collect(),
            target: 2,
        },
        Case {
            indices: vec![0, 0, 1, 0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 1],
            counts: vec![3; 6],
            positions: (0..6).map(|i| [i as f32, 0., 0.]).collect(),
            target: 3,
        },
        Case {
            indices: (0..12).collect(),
            counts: vec![3; 4],
            positions: (0..12).map(|i| [i as f32, 0., 0.]).collect(),
            target: 2,
        },
        Case {
            indices: vec![0, 1, 2, 2, 1, 3],
            counts: vec![3, 3],
            positions: vec![
                [f32::MAX, 0., 0.],
                [0., 0., 0.],
                [-f32::MAX, 0., 0.],
                [1., 1., 1.],
            ],
            target: 2,
        },
        Case {
            indices: vec![0, 1],
            counts: vec![u32::MAX, u32::MAX],
            positions: vec![[0.; 3]; 2],
            target: 2,
        },
        Case {
            indices: vec![0, 1, 9],
            counts: vec![3],
            positions: vec![[0.; 3]; 2],
            target: 0,
        },
        Case {
            indices: vec![0, 1],
            counts: vec![0, 2],
            positions: vec![[0.; 3]; 2],
            target: 1,
        },
        Case {
            indices: vec![0, 1, 2],
            counts: vec![3],
            positions: vec![[0.; 3]; 3],
            target: usize::MAX,
        },
        Case {
            indices: vec![0, 1, 2, 2, 1, 3, 2, 3, 4],
            counts: vec![3; 3],
            positions: (0..5).map(|i| [i as f32, 0., 0.]).collect(),
            target: 100_000,
        },
    ];
    let mut comparisons = 0usize;
    for (id, c) in cases.iter().enumerate() {
        for spatial in [false, true] {
            for api in 0..2 {
                for short in [false, true] {
                    if api == 0 && short {
                        continue;
                    }
                    let reference =
                        observe!(baseline, c, spatial, api, short, u64::MAX, usize::MAX);
                    let mut budgets = Vec::new();
                    for work in 0..=reference.work.saturating_add(2) {
                        budgets.push((work, usize::MAX));
                    }
                    for bytes in 0..=reference.bytes.saturating_add(16) {
                        budgets.push((u64::MAX, bytes));
                    }
                    for (work, bytes) in budgets {
                        let a = observe!(baseline, c, spatial, api, short, work, bytes);
                        let b = observe!(candidate, c, spatial, api, short, work, bytes);
                        assert_eq!(a,b,"case={id} spatial={spatial} api={api} short={short} work={work} bytes={bytes}");
                        comparisons += 1;
                    }
                }
            }
        }
    }
    println!("PASS {comparisons} exact observations");
}
