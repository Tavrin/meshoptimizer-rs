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
