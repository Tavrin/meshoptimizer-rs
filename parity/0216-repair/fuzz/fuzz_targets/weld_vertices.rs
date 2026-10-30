#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .chunks_exact(12)
        .take(64)
        .map(|b| {
            core::array::from_fn(|k| {
                f32::from_bits(u32::from_le_bytes(b[k * 4..k * 4 + 4].try_into().unwrap()))
            })
        })
        .collect();
    let tolerance = data.first().copied().unwrap_or(0) as f32 / 255.;
    // The trailing byte picks the policy, a work limit and the destination size.
    let mode = data.last().copied().unwrap_or(0);
    let class = |i: u32| data.get(i as usize).copied().unwrap_or(0) % 3;
    let same_class = |a: u32, b: u32| class(a) == class(b);
    let policy = if mode & 1 == 0 {
        WeldPolicy::PositionsOnly
    } else {
        WeldPolicy::AttributeEquality(&same_class)
    };
    let positions = Positions::from_packed(&p);
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    let result = weld_vertices(positions, tolerance, policy, &mut ws);
    if let Ok(r) = &result {
        for (i, &j) in r.remap.iter().enumerate() {
            assert!(j as usize <= i);
            assert_eq!(r.remap[j as usize], j);
            let d: f64 = (0..3)
                .map(|k| (f64::from(p[i][k]) - f64::from(p[j as usize][k])).powi(2))
                .sum();
            assert!(d <= f64::from(tolerance).powi(2));
            if mode & 1 != 0 {
                assert_eq!(class(i as u32), class(j));
            }
        }
        assert_eq!(r.remap.len(), p.len());
        assert_eq!(
            r.report.representatives + r.report.merged_vertices,
            r.report.input_vertices
        );
    }
    // A caller buffer of the bound gives the same mapping; a short one leaves
    // the destination unchanged.
    let bound = weld_vertices_bound(p.len());
    assert_eq!(bound, p.len());
    let len = if mode & 2 == 0 {
        bound + 2
    } else {
        bound.saturating_sub(1)
    };
    let mut out = vec![u32::MAX; len];
    let before = out.clone();
    match weld_vertices_into(&mut out, positions, tolerance, policy, &mut ws) {
        Ok(report) => {
            let r = result.as_ref().expect("allocating form succeeds too");
            assert_eq!(&out[..p.len()], &r.remap[..]);
            assert_eq!(report, r.report);
            assert!(out[p.len()..].iter().all(|&v| v == u32::MAX));
        }
        Err(_) => assert_eq!(out, before),
    }
    // Under a small work limit the call fails or equals the unlimited result.
    let mut tight = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: u64::from(mode >> 2) * 16,
    });
    if let (Ok(limited), Ok(full)) = (
        weld_vertices(positions, tolerance, policy, &mut tight),
        &result,
    ) {
        assert_eq!(&limited, full);
    }
});
