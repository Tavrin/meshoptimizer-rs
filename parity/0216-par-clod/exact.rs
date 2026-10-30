//! Separate baseline/candidate crates, exact float words and observable prefixes.
use std::io::Read;
#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(), String>,
    words: Vec<u32>,
    positions: Vec<u32>,
    usage: (usize, u64),
}
#[derive(Clone)]
struct Case {
    p: Vec<[f32; 3]>,
    i: Vec<u32>,
    variant: usize,
}
macro_rules! runner {
    ($name:ident,$m:ident,$serial:path,$parallel:path) => {
        fn $name(c: &Case, par: bool, limits: (usize, u64), fail: usize) -> Vec<Observation> {
            use $m as m;
            let mut ws = m::Workspace::new(m::Limits {
                max_bytes: limits.0,
                max_work: limits.1,
            });
            let mut out = Vec::new();
            // Cold, warm, and after a failed invocation on the same workspace.
            for phase in 0..4 {
                let mut p = c.p.clone();
                let mut cfg = if c.variant % 2 == 0 {
                    m::clusterlod::default_config(4)
                } else {
                    m::clusterlod::default_config_rt(4)
                }
                .unwrap();
                cfg.partition_size = 2;
                match c.variant {
                    2 => cfg.simplify_dilate_borders = true,
                    3 => cfg.optimize_bounds = true,
                    4 => cfg.partition_sort = true,
                    5 => cfg.cluster_spatial = true,
                    6 => cfg.simplify_regularize = true,
                    7 => cfg.simplify_preserve_folds = true,
                    8 => cfg.simplify_error_edge_limit = 0.5,
                    9 => cfg.simplify_fallback_permissive = true,
                    10 => cfg.simplify_error_merge_additive = 0.5,
                    11 => cfg.simplify_ratio = 0.75,
                    12 => cfg.partition_size = 0,
                    13 => cfg.simplify_threshold = 0.0,
                    14 => cfg.simplify_permissive = false,
                    15 => cfg.max_triangles = 0,
                    16 => cfg.optimize_clusters = false,
                    17 => cfg.optimize_clusters_level = 9,
                    _ => {}
                }
                if c.variant == 100 {
                    cfg = m::clusterlod::default_config(128).unwrap();
                }
                let attrs: Vec<f32> = (0..p.len()).flat_map(|i| [i as f32 * 0.125, 1.0]).collect();
                let flags = vec![m::VertexFlags::EMPTY; p.len()];
                let mut words = Vec::new();
                let mut calls = 0;
                let mesh = m::clusterlod::Mesh {
                    indices: &c.i,
                    positions: &mut p,
                    attributes: if c.variant == 18 {
                        Some(m::Attributes::from_interleaved(&attrs, c.p.len(), 2, 2, 0).unwrap())
                    } else {
                        None
                    },
                    vertex_lock: if c.variant == 19 { Some(&flags) } else { None },
                    attribute_weights: if c.variant == 18 { &[0.5, 1.0] } else { &[] },
                    attribute_protect_mask: 0,
                };
                let callback = |g: m::clusterlod::Group, cs: &[m::clusterlod::Cluster]| {
                    words.extend([g.depth as u32, cs.len() as u32]);
                    words.extend(g.simplified.center.map(f32::to_bits));
                    words.extend([g.simplified.radius.to_bits(), g.simplified.error.to_bits()]);
                    for cl in cs {
                        words.extend([
                            cl.refined as u32,
                            cl.vertex_count as u32,
                            cl.indices.len() as u32,
                        ]);
                        words.extend(cl.bounds.center.map(f32::to_bits));
                        words.extend([cl.bounds.radius.to_bits(), cl.bounds.error.to_bits()]);
                        words.extend(&cl.indices);
                    }
                    calls += 1;
                    if calls == if phase == 2 { 1 } else { fail } {
                        Err(m::Error::InvalidParameter)
                    } else {
                        Ok(17 + calls as i32 * 7)
                    }
                };
                let result = if par {
                    $parallel(cfg, mesh, callback, &mut ws)
                } else {
                    $serial(cfg, mesh, callback, &mut ws)
                };
                out.push(Observation {
                    result: result.map_err(|e| format!("{e:?}")),
                    words,
                    positions: p.iter().flat_map(|p| p.map(f32::to_bits)).collect(),
                    usage: (ws.usage().bytes, ws.usage().work),
                });
                if c.variant == 100 {
                    break;
                } // Large corpus: one complete call per schedule/pool.
            }
            out
        }
    };
}
runner!(
    base,
    baseline,
    baseline::clusterlod::build_with_output,
    baseline::clusterlod::build_with_output
);
runner!(
    candidate,
    candidate,
    candidate::clusterlod::build_with_output,
    candidate::clusterlod::build_parallel_with_output
);
fn grid(n: u32) -> Case {
    Case {
        p: (0..=n)
            .flat_map(|y| (0..=n).map(move |x| [x as f32, y as f32, ((x * y) % 3) as f32 * 0.1]))
            .collect(),
        i: (0..n)
            .flat_map(|y| {
                (0..n).flat_map(move |x| {
                    let a = y * (n + 1) + x;
                    [a, a + 1, a + n + 1, a + 1, a + n + 2, a + n + 1]
                })
            })
            .collect(),
        variant: 0,
    }
}
fn read(path: &std::path::Path) -> Case {
    let mut f = std::fs::File::open(path).unwrap();
    let mut word = || {
        let mut b = [0; 4];
        f.read_exact(&mut b).unwrap();
        u32::from_le_bytes(b)
    };
    let n = word();
    let ni = word();
    let p = (0..n)
        .map(|_| std::array::from_fn(|_| f32::from_bits(word())))
        .collect();
    let i = (0..ni).map(|_| word()).collect();
    Case { p, i, variant: 100 }
}
fn main() {
    let unlimited = (usize::MAX, u64::MAX);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let mut comparisons = 0u64;
    if let Some(dir) = std::env::args().nth(1) {
        let mut files: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "bin"))
            .collect();
        files.sort();
        for path in files {
            let c = read(&path);
            let reference = base(&c, false, unlimited, usize::MAX);
            assert!(reference[0].result.is_ok(), "{path:?}");
            assert_eq!(reference, candidate(&c, false, unlimited, usize::MAX));
            comparisons += 1;
            for workers in [1, 2, 4, 8, 32] {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(workers)
                    .build()
                    .unwrap();
                for schedule in ["normal", "reverse", "zigzag"] {
                    std::env::set_var("MESHOPT_CLOD_SCHEDULE", schedule);
                    let got = pool.install(|| candidate(&c, true, unlimited, usize::MAX));
                    assert_eq!(reference, got, "{path:?} {workers} {schedule}");
                    comparisons += 1;
                }
            }
            println!(
                "corpus {:?} comparisons {comparisons}",
                path.file_name().unwrap()
            );
        }
    } else {
        let mut cases = Vec::new();
        for variant in 0..20 {
            let mut c = grid(2);
            c.variant = variant;
            cases.push(c);
        }
        for kind in 0..9 {
            let mut c = grid(2);
            match kind {
                0 => c.i.push(0),
                1 => c.i[0] = 999,
                2 => c.p[0][0] = f32::NAN,
                3 => c.p[0][0] = f32::MAX,
                4 => c.p.fill([0.0; 3]),
                5 => c.p[0][0] = -0.0,
                6 => {
                    c.i.clear();
                    c.p.clear();
                }
                7 => c.p[0][0] = f32::from_bits(1),
                _ => c.p[0][0] = 1.0e9,
            };
            cases.push(c);
        }
        for (ci, c) in cases.iter().enumerate() {
            let full = base(c, false, unlimited, usize::MAX);
            let work = full[0].usage.1;
            for w in 0..=work + 1 {
                let limits = (usize::MAX, w);
                let a = base(c, false, limits, usize::MAX);
                assert_eq!(
                    a,
                    candidate(c, false, limits, usize::MAX),
                    "serial case {ci} work {w}"
                );
                assert_eq!(
                    a,
                    pool.install(|| candidate(c, true, limits, usize::MAX)),
                    "parallel case {ci} work {w}"
                );
                comparisons += 2;
            }
            for bytes in [
                0,
                1,
                full[0].usage.0.saturating_sub(1),
                full[0].usage.0,
                full[0].usage.0 + 1,
            ] {
                let a = base(c, false, (bytes, u64::MAX), usize::MAX);
                assert_eq!(
                    a,
                    pool.install(|| candidate(c, true, (bytes, u64::MAX), usize::MAX)),
                    "case {ci} bytes {bytes}"
                );
                comparisons += 1;
            }
            for fail in [1, 2, 5, usize::MAX] {
                let a = base(c, false, unlimited, fail);
                assert_eq!(
                    a,
                    pool.install(|| candidate(c, true, unlimited, fail)),
                    "case {ci} fail {fail}"
                );
                comparisons += 1;
            }
            println!("targeted case {ci} work {work} comparisons {comparisons}");
        }
        // Genuine multi-group levels with all config variants and perturbed schedules.
        for variant in 0..20 {
            let mut c = grid(10);
            c.variant = variant;
            let a = base(&c, false, unlimited, usize::MAX);
            for workers in [1, 2, 4, 8, 32] {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(workers)
                    .build()
                    .unwrap();
                for schedule in ["normal", "reverse", "zigzag", "fail"] {
                    std::env::set_var("MESHOPT_CLOD_SCHEDULE", schedule);
                    assert_eq!(
                        a,
                        pool.install(|| candidate(&c, true, unlimited, usize::MAX)),
                        "variant {variant} workers {workers} {schedule}"
                    );
                    comparisons += 1;
                }
            }
        }
    }
    println!("PASS comparisons {comparisons} (exact results, float words, IDs, callbacks, positions, Usage)");
}
