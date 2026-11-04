#![no_main]
//! `trim_alpha_cards` / `trim_alpha_cards_generic`: no panic, determinism, u16 == u32,
//! budgeted == unbudgeted, report consistency, and conservative coverage (every visible
//! texel footprint inside an input card triangle lies inside an output triangle with the
//! same surface position).
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

type P = [f64; 2];
const TOL: f64 = 1e-4;

fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1]]
}
fn cross(a: P, b: P) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
fn bary(p: P, t: [P; 3]) -> [f64; 3] {
    let area = cross(sub(t[1], t[0]), sub(t[2], t[0]));
    let l1 = cross(sub(p, t[0]), sub(t[2], t[0])) / area;
    let l2 = cross(sub(t[1], t[0]), sub(p, t[0])) / area;
    [1.0 - l1 - l2, l1, l2]
}
fn segment_distance(p: P, a: P, b: P) -> f64 {
    let d = sub(b, a);
    let len = d[0] * d[0] + d[1] * d[1];
    let t = if len > 0.0 {
        (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / len).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let q = [a[0] + t * d[0], a[1] + t * d[1]];
    ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2)).sqrt()
}
fn distance(p: P, t: [P; 3]) -> f64 {
    let b = bary(p, t);
    if b.iter().all(|&x| x >= 0.0) {
        return 0.0;
    }
    (0..3)
        .map(|k| segment_distance(p, t[k], t[(k + 1) % 3]))
        .fold(f64::INFINITY, f64::min)
}
/// Clip a convex polygon to a triangle of either orientation.
fn clip(mut poly: Vec<P>, t: [P; 3]) -> Vec<P> {
    let s = cross(sub(t[1], t[0]), sub(t[2], t[0])).signum();
    for k in 0..3 {
        let (a, b) = (t[k], t[(k + 1) % 3]);
        let mut out = vec![];
        for m in 0..poly.len() {
            let (p, q) = (poly[m], poly[(m + 1) % poly.len()]);
            let (dp, dq) = (s * cross(sub(b, a), sub(p, a)), s * cross(sub(b, a), sub(q, a)));
            if dp >= 0.0 {
                out.push(p);
            }
            if (dp > 0.0 && dq < 0.0) || (dp < 0.0 && dq > 0.0) {
                let u = dp / (dp - dq);
                out.push([p[0] + u * (q[0] - p[0]), p[1] + u * (q[1] - p[1])]);
            }
        }
        poly = out;
    }
    poly
}

struct Geo<'a> {
    v: &'a [f32],
    stride: usize,
    w: u32,
    h: u32,
}
impl Geo<'_> {
    fn texel(&self, i: u32) -> P {
        let b = i as usize * self.stride + 3;
        [f64::from(self.v[b]) * f64::from(self.w), f64::from(self.v[b + 1]) * f64::from(self.h)]
    }
    fn pos(&self, i: u32) -> [f64; 3] {
        let b = i as usize * self.stride;
        [self.v[b], self.v[b + 1], self.v[b + 2]].map(f64::from)
    }
    fn interpolate(&self, ids: [u32; 3], w: [f64; 3]) -> [f64; 3] {
        let p = ids.map(|i| self.pos(i));
        [0, 1, 2].map(|c| w[0] * p[0][c] + w[1] * p[1][c] + w[2] * p[2][c])
    }
}

fn at(mask: &[u8], w: u32, h: u32, x: i64, y: i64, wrap: AlphaWrap) -> u8 {
    let (w, h) = (i64::from(w), i64::from(h));
    let (x, y) = match wrap {
        AlphaWrap::Repeat => (x.rem_euclid(w), y.rem_euclid(h)),
        _ => (x.clamp(0, w - 1), y.clamp(0, h - 1)),
    };
    mask[(y * w + x) as usize]
}

/// Coverage of every visible footprint point. Panics (a finding) when a point is uncovered.
fn check_coverage(
    input: (&[u32], &[f32]),
    mask: &[u8],
    dims: (u32, u32),
    stride: usize,
    o: CardTrimOptions,
    out: &TrimmedCards,
) {
    let (iv, ii) = (input.1, input.0);
    let (w, h) = dims;
    let pad = f64::from(o.padding);
    let gi = Geo { v: iv, stride, w, h };
    let go = Geo { v: &out.vertices, stride, w, h };
    let out_tris: Vec<([P; 3], [u32; 3])> = out
        .indices
        .chunks(3)
        .map(|c| {
            let ids = [c[0], c[1], c[2]];
            (ids.map(|i| go.texel(i)), ids)
        })
        .collect();
    let cell = |x: f64| (x / 8.0).floor() as i64;
    let mut grid = std::collections::HashMap::<(i64, i64), Vec<usize>>::new();
    for (k, (q, _)) in out_tris.iter().enumerate() {
        let lo = [0, 1].map(|c| cell(q.iter().map(|p| p[c]).fold(f64::INFINITY, f64::min) - TOL));
        let hi = [0, 1].map(|c| cell(q.iter().map(|p| p[c]).fold(f64::NEG_INFINITY, f64::max) + TOL));
        for y in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                grid.entry((x, y)).or_default().push(k);
            }
        }
    }
    // Position fidelity holds for folded cards too: every piece keeps its own
    // triangle's attributes, including corners where folded triangles touch in UV.
    for c in ii.chunks(3) {
        let ids = [c[0], c[1], c[2]];
        let t = ids.map(|i| gi.texel(i));
        // A point is accepted within TOL texels of an output triangle (f32 UV rounding),
        // so its expected position is only known up to |grad position| * TOL; thin input
        // triangles with steep attributes make that exceed a fixed tolerance.
        let tol: [f64; 3] = {
            let (e1, e2) = (sub(t[1], t[0]), sub(t[2], t[0]));
            let area = cross(e1, e2);
            let p = ids.map(|i| gi.pos(i));
            [0, 1, 2].map(|c| {
                let (d1, d2) = (p[1][c] - p[0][c], p[2][c] - p[0][c]);
                let g = [(d1 * e2[1] - d2 * e1[1]) / area, (d2 * e1[0] - d1 * e2[0]) / area];
                1e-3 + 4.0 * TOL * (g[0] * g[0] + g[1] * g[1]).sqrt()
            })
        };
        // Slivers make barycentrics unstable; the guarantee is about real texture area.
        if cross(sub(t[1], t[0]), sub(t[2], t[0])).abs() < 0.05 {
            continue;
        }
        let x0 = t.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let x1 = t.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let y0 = t.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
        let y1 = t.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
        for j in (y0 - 1.0 - pad).floor() as i64..=(y1 + pad).ceil() as i64 {
            for i in (x0 - 1.0 - pad).floor() as i64..=(x1 + pad).ceil() as i64 {
                if at(mask, w, h, i, j, o.wrap) < o.threshold {
                    continue;
                }
                let (a, b) = (i as f64 - pad, i as f64 + 1.0 + pad);
                let (e, f) = (j as f64 - pad, j as f64 + 1.0 + pad);
                for p in clip(vec![[a, e], [b, e], [b, f], [a, f]], t) {
                    let want = gi.interpolate(ids, bary(p, t));
                    let near = grid.get(&(cell(p[0]), cell(p[1]))).map_or(&[][..], |v| v);
                    let ok = near.iter().map(|&k| &out_tris[k]).any(|(q, qid)| {
                        distance(p, *q) <= TOL && {
                            let got = go.interpolate(*qid, bary(p, *q));
                            (0..3).all(|k| (got[k] - want[k]).abs() <= tol[k])
                        }
                    });
                    if !ok {
                        let uv_covered = near.iter().any(|&k| distance(p, out_tris[k].0) <= TOL);
                        let best = near
                            .iter()
                            .filter(|&&k| distance(p, out_tris[k].0) <= TOL)
                            .map(|&k| go.interpolate(out_tris[k].1, bary(p, out_tris[k].0)))
                            .collect::<Vec<_>>();
                        panic!(
                            "visible point {p:?} of texel ({i}, {j}) not covered: uv_covered={uv_covered} want={want:?} got={best:?} input_tri={t:?} ids={ids:?} out_tris={out_tris:?} wrap={:?} pad={pad}",
                            o.wrap
                        );
                    }
                }
            }
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let flags = c.u8();
    let nv = 1 + usize::from(c.u8()) % 24;
    let nt = usize::from(c.u8()) % 41;
    let (w, h) = (1 + u32::from(c.u8()) % 16, 1 + u32::from(c.u8()) % 16);
    let mode = c.u8();
    let threshold = c.u8();
    let maxv = 2 + u32::from(c.u8()) % 16;
    let pad_b = c.u8();
    let cost_b = c.u8();
    let stride = 5 + usize::from(c.u8()) % 3;
    let limits = budget(&mut c);
    let raw = c.take(256);
    let special = flags & 0x80 != 0;

    // Mask: raw bytes, binarised bytes, fully visible, or an ellipse.
    let mut mask = vec![0u8; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let b = raw[(y * 16 + x) as usize];
            mask[(y * w + x) as usize] = match mode & 3 {
                0 => b,
                1 => {
                    if b >= 128 {
                        255
                    } else {
                        0
                    }
                }
                2 => 255,
                _ => {
                    let (dx, dy) = (
                        (f64::from(x) + 0.5) / f64::from(w) - 0.5,
                        (f64::from(y) + 0.5) / f64::from(h) - 0.5,
                    );
                    if dx * dx + dy * dy < 0.16 {
                        255
                    } else {
                        0
                    }
                }
            };
        }
    }

    // Vertices: position, uv, filler attributes.
    let planar = flags & 0x10 != 0;
    let mut verts = vec![0f32; nv * stride];
    for k in 0..nv {
        let (px, py, pz) = (c.u8(), c.u8(), c.u8());
        let (ub, vb) = (c.u8(), c.u8());
        let b = k * stride;
        verts[b] = coord(px, special, 16., 128.);
        verts[b + 1] = coord(py, special, 16., 128.);
        verts[b + 2] = coord(pz, special, 16., 128.);
        if planar {
            // uv affine in position: a planar card with linear attributes (the one-fan path).
            verts[b + 3] = (verts[b] + 8.) / 16.;
            verts[b + 4] = (verts[b + 1] + 8.) / 16.;
        } else {
            verts[b + 3] = coord(ub, special, 176., 40.);
            verts[b + 4] = coord(vb, special, 176., 40.);
        }
        for e in 5..stride {
            verts[b + e] = (mix(k as u64, e as u64) % 1000) as f32 / 100.;
        }
    }
    let mut idx: Vec<u32> = Vec::new();
    if flags & 0x20 != 0 {
        // Quads over consecutive vertex quadruples: whole-card trimming paths.
        for q in 0..nv / 4 {
            let q = (q * 4) as u32;
            idx.extend_from_slice(&[q, q + 1, q + 2, q, q + 2, q + 3]);
        }
        let keep = nt.min(idx.len() / 3);
        idx.truncate(keep * 3);
    } else {
        let span = nv + if flags & 0x40 != 0 { 2 } else { 0 };
        for _ in 0..nt * 3 {
            idx.push((usize::from(c.u8()) % span) as u32);
        }
    }
    if flags & 0x08 != 0 && !idx.is_empty() {
        idx.pop(); // incomplete final triangle
    }

    let mut o = CardTrimOptions::default();
    o.threshold = threshold;
    o.max_vertices = maxv;
    o.padding = if flags & 0x04 != 0 { f32::from(pad_b) / 3.0 } else { f32::from(pad_b) / 255. * 3. };
    o.wrap = if mode & 4 != 0 { AlphaWrap::Repeat } else { AlphaWrap::Clamp };
    o.vertex_cost = match mode >> 4 {
        0 => 0.0,
        15 => -1.0,
        k => f32::from(k) * f32::from(cost_b) * 0.5,
    };
    let Ok(m) = AlphaMask::new(&mask, w, h) else {
        panic!("a valid mask was rejected");
    };

    let run = |ws: &mut Workspace| trim_alpha_cards(&idx, &verts, stride, 3, m, o, ws);
    let a = run(&mut Workspace::default());
    // Determinism.
    assert_eq!(a, run(&mut Workspace::default()), "non-deterministic");
    // Budgeted == unbudgeted unless the budget ran out.
    match (&a, run(&mut workspace(limits))) {
        (Ok(a), Ok(b)) => assert_eq!(*a, b, "budgeted result differs"),
        (Err(x), Err(y)) => assert!(x == &y || resource(&y), "budgeted error {y:?} vs {x:?}"),
        (Ok(_), Err(y)) => assert!(resource(&y), "budgeted failed with {y:?} where unbudgeted succeeded"),
        (Err(x), Ok(_)) => panic!("budgeted succeeded where unbudgeted failed with {x:?}"),
    }
    // u16 equals u32 (every index fits).
    let i16: Vec<u16> = idx.iter().map(|&i| i as u16).collect();
    let b = trim_alpha_cards_generic::<u16>(&i16, &verts, stride, 3, m, o, &mut Workspace::default());
    match (&a, &b) {
        (Ok(a), Ok(b)) => {
            let widened: Vec<u32> = b.indices.iter().map(|&i| u32::from(i)).collect();
            assert_eq!(a.indices, widened, "u16 indices differ");
            assert_eq!(a.vertices, b.vertices, "u16 vertices differ");
            assert_eq!(a.report, b.report, "u16 report differs");
            assert_eq!(a.card_vertices, b.card_vertices, "u16 card_vertices differ");
        }
        (Err(x), Err(y)) => assert_eq!(x, y, "u16 error differs"),
        _ => panic!("u16 and u32 disagree: {a:?} vs {b:?}"),
    }

    let Ok(t) = a else { return };
    let r = t.report;
    assert_eq!(t.vertices.len() % stride, 0);
    let nvo = t.vertices.len() / stride;
    assert_eq!(r.output_vertices, nvo);
    assert_eq!(r.input_triangles, idx.len() / 3);
    assert_eq!(r.output_triangles * 3, t.indices.len());
    assert!(t.indices.iter().all(|&i| (i as usize) < nvo), "output index out of range");
    assert!(t.vertices.iter().all(|x| x.is_finite()), "non-finite output vertex");
    assert_eq!(t.card_vertices.len(), r.cards);
    assert_eq!(r.trimmed_cards + r.unchanged_cards + r.removed_cards, r.cards);
    if threshold == 0 {
        assert_eq!((r.trimmed_cards, r.removed_cards), (0, 0), "threshold 0 must keep cards");
    }
    assert!(r.area_after <= r.area_before * (1.0 + 1e-9) + 1e-6, "area grew: {r:?}");

    // Coverage on bounded geometry only: the check rasterizes every footprint.
    let used = |i: &u32| *i as usize;
    let bounded = idx.iter().map(used).all(|i| {
        let v = &verts[i * stride..i * stride + 5];
        v[..3].iter().all(|x| x.abs() <= 64.) && v[3..5].iter().all(|x| (-1.5..=2.5).contains(x))
    });
    if threshold > 0 && bounded && idx.len() % 3 == 0 {
        check_coverage((&idx, &verts), &mask, (w, h), stride, o, &t);
    }
});
