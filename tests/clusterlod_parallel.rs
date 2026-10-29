#![cfg(all(feature = "clusterlod", feature = "parallel"))]
use meshoptimizer_rs::{clusterlod as c, Error, Limits, Usage, Workspace};
use std::cell::RefCell;

fn grid(n: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..=n)
        .flat_map(|y| (0..=n).map(move |x| [x as f32, y as f32, ((x * y) % 7) as f32 * 0.1]))
        .collect();
    let i = (0..n)
        .flat_map(|y| {
            (0..n).flat_map(move |x| {
                let a = y * (n + 1) + x;
                [a, a + 1, a + n + 1, a + 1, a + n + 2, a + n + 1]
            })
        })
        .collect();
    (p, i)
}
fn unlimited() -> Limits {
    Limits {
        max_bytes: usize::MAX,
        max_work: u64::MAX,
    }
}
fn run(
    par: bool,
    config: c::Config,
    p: &[[f32; 3]],
    i: &[u32],
    ws: &mut Workspace,
    fail: usize,
) -> (Result<(), Error>, Vec<u32>, Vec<u32>, Usage) {
    let mut p = p.to_vec();
    // RefCell deliberately makes this callback !Sync. It executes serially.
    let records = RefCell::new(Vec::new());
    let caller = std::thread::current().id();
    let mut calls = 0;
    let callback = |g: c::Group, cs: &[c::Cluster]| {
        assert_eq!(std::thread::current().id(), caller);
        let mut o = records.borrow_mut();
        o.extend([g.depth as u32, cs.len() as u32]);
        o.extend(g.simplified.center.map(f32::to_bits));
        o.extend([g.simplified.radius.to_bits(), g.simplified.error.to_bits()]);
        for cl in cs {
            o.extend([
                cl.refined as u32,
                cl.vertex_count as u32,
                cl.indices.len() as u32,
            ]);
            o.extend(cl.bounds.center.map(f32::to_bits));
            o.extend([cl.bounds.radius.to_bits(), cl.bounds.error.to_bits()]);
            o.extend(&cl.indices);
        }
        calls += 1;
        if calls == fail {
            Err(Error::InvalidParameter)
        } else {
            Ok(1000 + calls as i32 * 7)
        }
    };
    let mesh = c::Mesh {
        indices: i,
        positions: &mut p,
        attributes: None,
        vertex_lock: None,
        attribute_weights: &[],
        attribute_protect_mask: 0,
    };
    let result = if par {
        c::build_parallel_with_output(config, mesh, callback, ws)
    } else {
        c::build_with_output(config, mesh, callback, ws)
    };
    (
        result,
        records.into_inner(),
        p.into_iter().flat_map(|p| p.map(f32::to_bits)).collect(),
        ws.usage(),
    )
}
#[test]
fn ordered_output_bits_ids_and_usage_across_pools() {
    let (p, i) = grid(18);
    let mut config = c::default_config(16).unwrap();
    config.partition_size = 2;
    let expected = run(
        false,
        config,
        &p,
        &i,
        &mut Workspace::new(unlimited()),
        usize::MAX,
    );
    assert!(expected.0.is_ok());
    for workers in [1, 2, 4, 8, 32] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .unwrap();
        let mut ws = Workspace::new(unlimited());
        for _ in 0..2 {
            assert_eq!(
                pool.install(|| run(true, config, &p, &i, &mut ws, usize::MAX)),
                expected,
                "workers {workers}"
            );
        }
    }
}
#[test]
fn callback_failure_keeps_prefix_and_reusable_workspace() {
    let (p, i) = grid(14);
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 2;
    for fail in [1, 2, 5] {
        let expected = run(
            false,
            config,
            &p,
            &i,
            &mut Workspace::new(unlimited()),
            fail,
        );
        let mut ws = Workspace::new(unlimited());
        assert_eq!(run(true, config, &p, &i, &mut ws, fail), expected);
        assert_eq!(
            run(true, config, &p, &i, &mut ws, usize::MAX),
            run(
                false,
                config,
                &p,
                &i,
                &mut Workspace::new(unlimited()),
                usize::MAX
            )
        );
    }
}
#[test]
fn finite_budgets_and_dilation_keep_serial_failures_and_positions() {
    let (p, i) = grid(8);
    let mut config = c::default_config_rt(8).unwrap();
    config.partition_size = 2;
    for dilate in [false, true] {
        config.simplify_dilate_borders = dilate;
        let full = run(
            false,
            config,
            &p,
            &i,
            &mut Workspace::new(unlimited()),
            usize::MAX,
        );
        for limits in [
            Limits::default(),
            unlimited(),
            Limits {
                max_bytes: full.3.bytes - 1,
                ..unlimited()
            },
            Limits {
                max_work: full.3.work - 1,
                ..unlimited()
            },
            Limits {
                max_work: 1000,
                ..unlimited()
            },
        ] {
            assert_eq!(
                run(
                    true,
                    config,
                    &p,
                    &i,
                    &mut Workspace::new(limits),
                    usize::MAX
                ),
                run(
                    false,
                    config,
                    &p,
                    &i,
                    &mut Workspace::new(limits),
                    usize::MAX
                )
            );
        }
    }
}
type Observed = (Result<(), Error>, Vec<u32>, Vec<u32>, Usage);
const THREADS: [usize; 6] = [1, 2, 3, 4, 8, 16];
#[cfg(feature = "parity-internals")]
const SCHEDULES: [&str; 4] = ["", "reverse", "zigzag", "fail"];
#[cfg(not(feature = "parity-internals"))]
const SCHEDULES: [&str; 1] = [""];
static SCHEDULE: std::sync::Mutex<()> = std::sync::Mutex::new(());

// Compares parallel with serial on cold, warm and after-failure workspaces for
// every pool size and forced schedule. Schedules only perturb timing or force
// the serial fallback, so the serial observation is the oracle for all of them.
fn matrix(config: c::Config, p: &[[f32; 3]], i: &[u32], limits: Limits, fails: &[usize]) {
    let _guard = SCHEDULE.lock().unwrap_or_else(|e| e.into_inner());
    let serial = |fail| run(false, config, p, i, &mut Workspace::new(limits), fail);
    let full = serial(usize::MAX);
    let expected: Vec<Observed> = fails.iter().map(|&f| serial(f)).collect();
    let pools: Vec<_> = THREADS
        .iter()
        .map(|&n| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build()
                .unwrap()
        })
        .collect();
    for schedule in SCHEDULES {
        std::env::set_var("MESHOPT_CLOD_SCHEDULE", schedule);
        for (pool, threads) in pools.iter().zip(THREADS) {
            let mut ws = Workspace::new(limits);
            for (&fail, expected) in fails.iter().zip(&expected) {
                let got = pool.install(|| run(true, config, p, i, &mut ws, fail));
                assert_eq!(
                    &got, expected,
                    "threads {threads} schedule {schedule:?} fail {fail}"
                );
                let got = pool.install(|| run(true, config, p, i, &mut ws, usize::MAX));
                assert_eq!(
                    got, full,
                    "threads {threads} schedule {schedule:?} after {fail}"
                );
            }
        }
    }
    std::env::remove_var("MESHOPT_CLOD_SCHEDULE");
}
fn calls(config: c::Config, p: &[[f32; 3]], i: &[u32]) -> usize {
    let mut n = 0;
    let mut p = p.to_vec();
    let mesh = c::Mesh {
        indices: i,
        positions: &mut p,
        attributes: None,
        vertex_lock: None,
        attribute_weights: &[],
        attribute_protect_mask: 0,
    };
    let _ = c::build_with_output(
        config,
        mesh,
        |_, _| {
            n += 1;
            Ok(0)
        },
        &mut Workspace::new(unlimited()),
    );
    n
}
#[test]
fn every_callback_failure_emits_only_the_serial_prefix() {
    let (p, i) = grid(14);
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 2;
    let n = calls(config, &p, &i);
    assert!(n > 8);
    let fails: Vec<usize> = (1..=n).collect();
    matrix(config, &p, &i, unlimited(), &fails);
}
#[test]
fn later_group_numerical_failure_keeps_serial_prefix() {
    // Flat groups have zero error; the bumpy half overflows the additive merge,
    // so the serial build fails after emitting the flat groups.
    let n = 16;
    let (mut p, i) = grid(n);
    for v in &mut p {
        v[2] = if v[0] > n as f32 / 2.0 {
            ((v[0] as u32 * v[1] as u32) % 7) as f32 * 3.0
        } else {
            0.0
        };
    }
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 2;
    config.simplify_error_merge_additive = f32::MAX;
    let serial = run(
        false,
        config,
        &p,
        &i,
        &mut Workspace::new(unlimited()),
        usize::MAX,
    );
    assert_eq!(serial.0, Err(Error::NumericalFailure));
    assert!(!serial.1.is_empty(), "failure must follow emitted groups");
    matrix(config, &p, &i, unlimited(), &[usize::MAX, 1, 2]);
}
#[test]
fn usage_matches_across_configs_and_retention() {
    let (p, i) = grid(16);
    let mut configs = [
        c::default_config(8).unwrap(),
        c::default_config_rt(8).unwrap(),
        c::default_config(16).unwrap(),
    ];
    configs[0].optimize_bounds = true;
    configs[2].cluster_spatial = true;
    configs[2].partition_spatial = !configs[2].partition_spatial;
    for (k, mut config) in configs.into_iter().enumerate() {
        config.partition_size = 2 + k;
        matrix(config, &p, &i, unlimited(), &[usize::MAX, 3]);
        for retention in [
            meshoptimizer_rs::Retention::Reuse,
            meshoptimizer_rs::Retention::Lean,
        ] {
            let mut a = Workspace::new(unlimited());
            let mut b = Workspace::new(unlimited());
            a.set_retention(retention);
            b.set_retention(retention);
            // Warm both on a different mesh first.
            let (q, j) = grid(9);
            let _ = run(false, config, &q, &j, &mut a, usize::MAX);
            let _ = run(true, config, &q, &j, &mut b, usize::MAX);
            for workers in THREADS {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(workers)
                    .build()
                    .unwrap();
                let s = run(false, config, &p, &i, &mut a, usize::MAX);
                let r = pool.install(|| run(true, config, &p, &i, &mut b, usize::MAX));
                assert_eq!(r, s, "config {k} {retention:?} workers {workers}");
            }
        }
    }
}
#[test]
fn fallback_triggers_match_serial_at_limit_boundaries() {
    let (p, i) = grid(10);
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 2;
    for dilate in [false, true] {
        config.simplify_dilate_borders = dilate;
        let full = run(
            false,
            config,
            &p,
            &i,
            &mut Workspace::new(unlimited()),
            usize::MAX,
        );
        assert!(full.0.is_ok());
        let (w, b) = (full.3.work, full.3.bytes);
        let mut works: Vec<u64> = (0..=32).map(|k| w * k / 32).collect();
        works.extend([w - 2, w - 1, w, w + 1, w + 2, u64::MAX - 1, u64::MAX]);
        let mut bytes: Vec<usize> = (0..=32).map(|k| b * k / 32).collect();
        bytes.extend([b - 2, b - 1, b, b + 1, b + 2, usize::MAX - 1, usize::MAX]);
        let mut limits: Vec<Limits> = works
            .iter()
            .map(|&max_work| Limits {
                max_work,
                ..unlimited()
            })
            .collect();
        limits.extend(bytes.iter().map(|&max_bytes| Limits {
            max_bytes,
            ..unlimited()
        }));
        limits.extend([
            Limits {
                max_bytes: b,
                max_work: w,
            },
            Limits {
                max_bytes: b - 1,
                max_work: w,
            },
            Limits {
                max_bytes: b,
                max_work: w - 1,
            },
        ]);
        for limits in limits {
            matrix(config, &p, &i, limits, &[usize::MAX, 2]);
        }
    }
    // Single-group levels: one partition holds every cluster of each level.
    let (p, i) = grid(6);
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 64;
    matrix(config, &p, &i, unlimited(), &[usize::MAX, 1, 2]);
}
#[test]
fn miri_small_identity() {
    let (p, i) = grid(5);
    let mut config = c::default_config(8).unwrap();
    config.partition_size = 2;
    let expected = run(
        false,
        config,
        &p,
        &i,
        &mut Workspace::new(unlimited()),
        usize::MAX,
    );
    assert!(expected.0.is_ok());
    for workers in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .unwrap();
        let mut ws = Workspace::new(unlimited());
        for fail in [2, usize::MAX] {
            let serial = run(
                false,
                config,
                &p,
                &i,
                &mut Workspace::new(unlimited()),
                fail,
            );
            assert_eq!(
                pool.install(|| run(true, config, &p, &i, &mut ws, fail)),
                serial
            );
        }
    }
}
#[test]
fn allocating_groups_keep_refinement_indices() {
    let (p, i) = grid(12);
    let mut config = c::default_config(16).unwrap();
    config.partition_size = 2;
    let mut a = p.clone();
    let mut b = p;
    let mesh = |p| c::Mesh {
        indices: &i,
        positions: p,
        attributes: None,
        vertex_lock: None,
        attribute_weights: &[],
        attribute_protect_mask: 0,
    };
    let mut wa = Workspace::new(unlimited());
    let mut wb = Workspace::new(unlimited());
    let serial = c::build(config, mesh(&mut a), &mut wa).unwrap();
    let parallel = c::build_parallel(config, mesh(&mut b), &mut wb).unwrap();
    assert_eq!(serial, parallel);
    assert_eq!(wa.usage(), wb.usage());
}
