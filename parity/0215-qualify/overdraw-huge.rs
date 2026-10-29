//! Huge-but-finite triangles for optimize_overdraw, compared against v0.2.14
//! at every work limit from 0 to success + 1, every entry point, packed and
//! strided positions, cold and warm workspaces.
//!
//! On the std path the train takes the native square root of a cluster normal
//! norm before its finite check (v0.2.14 checked first). Finite coordinates
//! such as [0,0,0], [1e20,0,0], [0,1e20,0] make that norm infinite, and a
//! cluster of several finite-norm faces can overflow its summed normal. These
//! cases put such clusters first, in the middle and last among ordinary
//! clusters, in both orientations, with near-f32::MAX coordinates, so any
//! difference in Result, output, partial writes or Usage would show here.
//!
//! Prints `PASS observations=N failing-cases=F cases=C`.
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
        [cold, warm]
    }};
}

fn check(c: &Case, label: &str) -> (u64, bool) {
    let full = observe!(baseline, c, false, 0, u64::MAX);
    let success = full.iter().map(|o| o.work).max().unwrap();
    let mut n = 0;
    for strided in [false, true] {
        for api in 0..4 {
            for fuel in (0..=success + 1).chain([u64::MAX]) {
                let a = observe!(candidate, c, strided, api, fuel);
                let b = observe!(baseline, c, strided, api, fuel);
                assert!(a == b, "{label} strided={strided} api={api} fuel={fuel}: {a:?} / {b:?}");
                n += 2;
            }
        }
    }
    (n, full[0].result.is_err())
}

fn push(c: &mut Case, p: [[f32; 3]; 3], reversed: bool) {
    let b = c.p.len() as u32;
    c.p.extend(p);
    c.indices
        .extend(if reversed { [b, b + 2, b + 1] } else { [b, b + 1, b + 2] });
}

/// A connected strip of `faces` triangles sharing edges, all scaled by `s`,
/// so they form one cluster whose summed normal can overflow even when each
/// face's normal is finite.
fn strip(c: &mut Case, s: f32, faces: usize, x0: f32) {
    let b = c.p.len() as u32;
    for i in 0..=faces / 2 + 1 {
        let x = x0 + i as f32;
        c.p.push([x * s, 0.0, 0.0]);
        c.p.push([x * s + s * 0.5, s, 0.0]);
    }
    for f in 0..faces as u32 {
        let k = b + f;
        c.indices
            .extend(if f % 2 == 0 { [k, k + 2, k + 1] } else { [k, k + 1, k + 2] });
    }
}

fn main() {
    let mut checks = 0;
    let mut failing = 0;
    let mut cases = 0;
    // 1e9..1e18 keep every intermediate finite (accepted controls); 1e19 and
    // above overflow the normal norm or later products.
    let huge = [1e9f32, 1e15, 1e18, 1e19, 1.8e19, 1.85e19, 1e20, 1e30, f32::MAX / 2.0, f32::MAX];
    // One huge triangle among ordinary unit triangles, at every position.
    for &s in &huge {
        for count in [1usize, 2, 4] {
            for bad in 0..count {
                for reversed in [false, true] {
                    for threshold in [-1.0f32, 1.05, 2.0] {
                        let mut c = Case {
                            p: vec![],
                            indices: vec![],
                            threshold,
                        };
                        for i in 0..count {
                            let x = i as f32 * 4.0;
                            if i == bad {
                                push(&mut c, [[x, 0., 0.], [x + s, 0., 0.], [x, s, 0.]], reversed);
                            } else {
                                push(&mut c, [[x, 0., 0.], [x + 1., 0., 0.], [x, 1., 0.]], false);
                            }
                        }
                        let (n, f) = check(&c, &format!("huge s={s} count={count} bad={bad} rev={reversed} t={threshold}"));
                        checks += n;
                        failing += usize::from(f);
                        cases += 1;
                    }
                }
            }
        }
    }
    // The example triangle itself, alone and next to an ordinary triangle.
    for extra in [false, true] {
        let mut c = Case {
            p: vec![],
            indices: vec![],
            threshold: 1.05,
        };
        push(&mut c, [[0., 0., 0.], [1e20, 0., 0.], [0., 1e20, 0.]], false);
        if extra {
            push(&mut c, [[5., 0., 0.], [6., 0., 0.], [5., 1., 0.]], false);
        }
        let (n, f) = check(&c, &format!("example extra={extra}"));
        checks += n;
        failing += usize::from(f);
        cases += 1;
    }
    // Multi-face clusters whose per-face normals are finite but whose sum
    // overflows (1.5e19^2 = 2.25e38 per face), before and after a unit strip.
    for &s in &[1e9f32, 1.2e19, 1.5e19, 1.8e19] {
        for faces in [2usize, 3, 5] {
            for first in [false, true] {
                let mut c = Case {
                    p: vec![],
                    indices: vec![],
                    threshold: 1.05,
                };
                if first {
                    strip(&mut c, s, faces, 0.0);
                    strip(&mut c, 1.0, faces, 100.0);
                } else {
                    strip(&mut c, 1.0, faces, 100.0);
                    strip(&mut c, s, faces, 0.0);
                }
                let (n, f) = check(&c, &format!("strip s={s} faces={faces} first={first}"));
                checks += n;
                failing += usize::from(f);
                cases += 1;
            }
        }
    }
    // Near-f32::MAX coordinates with opposite signs: finite inputs whose edge
    // differences, centroids or normals overflow.
    let m = f32::MAX;
    for p in [
        [[m, 0., 0.], [m, m / 2., 0.], [0., m, 0.]],
        [[-m, 0., 0.], [m, 0., 0.], [0., m, 0.]],
        [[m, m, m], [-m, -m, m], [m, -m, -m]],
        [[m / 3., 0., 0.], [0., m / 3., 0.], [0., 0., m / 3.]],
        [[1e-30, 0., 0.], [m, 1e-30, 0.], [0., m, 1e-30]],
    ] {
        for reversed in [false, true] {
            let mut c = Case {
                p: vec![],
                indices: vec![],
                threshold: 1.05,
            };
            push(&mut c, [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], false);
            push(&mut c, p, reversed);
            let (n, f) = check(&c, &format!("near-max {p:?} rev={reversed}"));
            checks += n;
            failing += usize::from(f);
            cases += 1;
        }
    }
    println!("PASS observations={checks} failing-cases={failing} cases={cases}");
}
