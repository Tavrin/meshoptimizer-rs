//! Additional exact cases for the 0.2.15 overdraw paths, comparing two
//! separately compiled crate versions at every work limit from 0 to
//! success + 1 and every entry point. The 0.2.12 driver covers the rest.
#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(), String>,
    output: Vec<u32>,
    work: u64,
    bytes: usize,
}
struct Case {
    p: Vec<[f32; 3]>,
    indices: Vec<u32>,
    threshold: f32,
}
macro_rules! observe {
    ($m:ident, $case:expr, $strided:expr, $api:expr, $fuel:expr) => {{
        use $m as m;
        let c: &Case = $case;
        let floats: Vec<f32> = c.p.iter().flat_map(|p| [p[0], p[1], p[2], -7.0]).collect();
        let p = if $strided {
            m::Positions::from_interleaved(&floats, c.p.len(), 4, 0).unwrap()
        } else {
            m::Positions::from_packed(&c.p)
        };
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut run = || {
            let mut output = Vec::new();
            let result = match $api {
                0 => m::optimize_overdraw(&c.indices, p, c.threshold, &mut ws).map(|v| output = v),
                1 => {
                    output = vec![0xdeadbeef; c.indices.len() + 7];
                    m::optimize_overdraw_into(&mut output, &c.indices, p, c.threshold, &mut ws)
                }
                2 => {
                    output = c.indices.clone();
                    m::optimize_overdraw_in_place(&mut output, p, c.threshold, &mut ws)
                }
                _ => {
                    output = vec![0xdeadbeef; c.indices.len().saturating_sub(1)];
                    m::optimize_overdraw_into(&mut output, &c.indices, p, c.threshold, &mut ws)
                }
            }
            .map_err(|e| format!("{e:?}"));
            Observation {
                result,
                output,
                work: ws.usage().work,
                bytes: ws.usage().bytes,
            }
        };
        let cold = run();
        let warm = run();
        assert_eq!(cold, warm);
        cold
    }};
}

/// Every entry point, packed and strided storage, every limit 0..=success+1.
fn check(c: &Case, label: &str) -> (usize, Observation) {
    let full = observe!(baseline, c, false, 0, u64::MAX);
    let mut n = 0;
    for strided in [false, true] {
        for api in 0..4 {
            for fuel in (0..=full.work + 1).chain([u64::MAX]) {
                let a = observe!(candidate, c, strided, api, fuel);
                let b = observe!(baseline, c, strided, api, fuel);
                assert!(a == b, "{label} strided={strided} api={api} fuel={fuel}: {a:?} / {b:?}");
                n += 1;
            }
        }
    }
    (n, full)
}

fn triangle(c: &mut Case, p: [[f32; 3]; 3]) {
    let b = c.p.len() as u32;
    c.p.extend(p);
    c.indices.extend([b, b + 1, b + 2]);
}

fn main() {
    let mut checks = 0;
    let mut failures = 0;
    // A numerical failure in the k-th of several one-triangle clusters: the
    // covered path must charge exactly the clusters and faces visited so far.
    for count in [1, 2, 3, 5] {
        for bad in 0..count {
            for scale in [1e19f32, 2e19, f32::MAX, f32::INFINITY, f32::NAN] {
                let mut c = Case {
                    p: vec![],
                    indices: vec![],
                    threshold: 1.05,
                };
                for i in 0..count {
                    let s = if i == bad { scale } else { 1.0 };
                    let x = i as f32 * 4.0;
                    triangle(&mut c, [[x, 0., 0.], [x + s, 0., 0.], [x, s, 0.]]);
                }
                let (n, full) = check(&c, &format!("kth-cluster count={count} bad={bad} scale={scale}"));
                checks += n;
                failures += usize::from(full.result.is_err());
            }
        }
    }
    // Clusters spanning several faces, failing in a later cluster.
    for faces in [2, 4, 7] {
        for threshold in [-1.0, 0.5, 1.05, f32::MAX / 4.0] {
            let mut c = Case {
                p: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]],
                indices: vec![],
                threshold,
            };
            for f in 0..faces {
                c.indices.extend(if f % 2 == 0 { [0, 1, 2] } else { [1, 3, 2] });
            }
            triangle(&mut c, [[0., 0., 0.], [3e19, 0., 0.], [0., 3e19, 0.]]);
            for f in 0..faces {
                c.indices.extend(if f % 2 == 0 { [0, 1, 2] } else { [1, 3, 2] });
            }
            let (n, full) = check(&c, &format!("later-cluster faces={faces} threshold={threshold}"));
            checks += n;
            failures += usize::from(full.result.is_err());
        }
    }
    // Finite coordinates whose validated sum overflows: validation succeeds
    // through the replayed checked scan; the deferred centroid check fails.
    for (p, indices) in [
        (vec![[f32::MAX, 0., 0.], [f32::MAX, 0., 0.], [0., 1., 0.]], vec![0, 1, 2]),
        (vec![[0., -f32::MAX, 0.], [1., -f32::MAX, 0.], [0., 1., 0.]], vec![0, 1, 2]),
        (vec![[f32::MAX, 0., 0.], [-f32::MAX, 0., 0.], [0., 1., 0.]], vec![0, 1, 2]),
        (vec![[f32::MAX, 0., 0.], [f32::MAX, 0., 0.], [f32::NAN, 1., 0.]], vec![0, 1, 2]),
        (vec![[f32::MAX, 0., 0.], [f32::MAX, 0., 0.], [0., 1., f32::INFINITY]], vec![0, 1, 2]),
        (vec![[f32::INFINITY, 0., 0.], [f32::NEG_INFINITY, 0., 0.], [0., 1., 0.]], vec![0, 1, 2]),
    ] {
        let c = Case {
            p,
            indices,
            threshold: 1.05,
        };
        let (n, full) = check(&c, "validation-sum-overflow");
        checks += n;
        failures += usize::from(full.result.is_err());
    }
    // Reciprocal escapes: an overflowing normal norm whose components cancel
    // the dot product, and subnormal areas and normals.
    for s in [1e19f32, 1.9e19, f32::from_bits(1), 1e-23, 1e-30] {
        for reversed in [false, true] {
            let mut c = Case {
                p: vec![],
                indices: vec![],
                threshold: -1.0,
            };
            triangle(&mut c, [[s, 0., 0.], [-s, s, 0.], [0., -s, 0.]]);
            if reversed {
                triangle(&mut c, [[s, 0., 0.], [0., -s, 0.], [-s, s, 0.]]);
            } else {
                triangle(&mut c, [[s, 0., 0.], [-s, s, 0.], [0., -s, 0.]]);
            }
            let (n, full) = check(&c, &format!("reciprocal s={s} reversed={reversed}"));
            checks += n;
            failures += usize::from(full.result.is_err());
        }
    }
    assert!(failures > 0);
    println!("PASS: {checks} exact observations; {failures} failing cases among the new ones");
}
