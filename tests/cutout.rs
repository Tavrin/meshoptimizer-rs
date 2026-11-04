//! Hand-authored and generated test data is CC0-1.0.
//!
//! The coverage check rasterizes each mask: for every visible texel and every
//! card triangle it meets, each corner of (padded footprint ∩ triangle) must lie
//! in an output triangle that maps it to the same position as the input.
//! Convex outputs per source region make the corner test sufficient.
use meshoptimizer_rs::*;

type P = [f64; 2];
const STRIDE: usize = 8; // x, y, z, nx, ny, nz, u, v
const UV: usize = 6;
const TOL: f64 = 1e-4; // texels

fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1]]
}
fn cross(a: P, b: P) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

struct Mask {
    data: Vec<u8>,
    w: u32,
    h: u32,
}
impl Mask {
    fn view(&self) -> AlphaMask<'_> {
        AlphaMask::new(&self.data, self.w, self.h).unwrap()
    }
    fn at(&self, x: i64, y: i64, wrap: AlphaWrap) -> u8 {
        let (w, h) = (i64::from(self.w), i64::from(self.h));
        let (x, y) = match wrap {
            AlphaWrap::Repeat => (x.rem_euclid(w), y.rem_euclid(h)),
            _ => (x.clamp(0, w - 1), y.clamp(0, h - 1)),
        };
        self.data[(y * w + x) as usize]
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn f(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Soft-edged ellipses; `edge` lets blobs reach the mask border.
fn blobs(rng: &mut Rng, w: u32, h: u32, count: usize) -> Mask {
    let mut data = vec![0u8; (w * h) as usize];
    for _ in 0..count {
        let (cx, cy) = (rng.f() * w as f64, rng.f() * h as f64);
        let (rx, ry) = (
            1.0 + rng.f() * w as f64 * 0.3,
            1.0 + rng.f() * h as f64 * 0.3,
        );
        for y in 0..h {
            for x in 0..w {
                let dx = (x as f64 + 0.5 - cx) / rx;
                let dy = (y as f64 + 0.5 - cy) / ry;
                let d = (dx * dx + dy * dy).sqrt();
                let a = ((1.2 - d) * 255.0 / 0.4).clamp(0.0, 255.0) as u8;
                let cell = &mut data[(y * w + x) as usize];
                *cell = (*cell).max(a);
            }
        }
    }
    Mask { data, w, h }
}

#[derive(Default)]
struct Mesh {
    v: Vec<f32>,
    i: Vec<u32>,
}
impl Mesh {
    fn vertex(&mut self, pos: [f64; 3], n: [f64; 3], uv: P) -> u32 {
        let id = (self.v.len() / STRIDE) as u32;
        for x in [pos[0], pos[1], pos[2], n[0], n[1], n[2], uv[0], uv[1]] {
            self.v.push(x as f32);
        }
        id
    }
    /// Planar card: position is an affine function of UV.
    fn quad(&mut self, uv: [P; 4], flip: bool) {
        let ids = uv.map(|t| {
            self.vertex(
                [t[0] * 3.0 + 1.0, t[1] * 2.0 - t[0], 0.5],
                [0.0, 0.0, 1.0],
                t,
            )
        });
        if flip {
            self.i
                .extend([ids[0], ids[2], ids[1], ids[0], ids[3], ids[2]]);
        } else {
            self.i
                .extend([ids[0], ids[1], ids[2], ids[0], ids[2], ids[3]]);
        }
    }
    /// Non-affine card: trapezoid positions, varying normals.
    fn trapezoid(&mut self, uv: [P; 4]) {
        let pos = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [1.6, 1.0, 0.3],
            [0.4, 1.0, 0.3],
        ];
        let nrm = [
            [-0.5, 0.0, 0.8],
            [0.5, 0.0, 0.8],
            [0.3, 0.3, 0.9],
            [-0.3, 0.3, 0.9],
        ];
        let ids: Vec<u32> = (0..4).map(|k| self.vertex(pos[k], nrm[k], uv[k])).collect();
        self.i
            .extend([ids[0], ids[1], ids[2], ids[0], ids[2], ids[3]]);
    }
    /// Three bent quads sharing edges (a grass blade), UV column `u0..u1`.
    fn strip(&mut self, u0: f64, u1: f64, v0: f64, v1: f64) {
        let mut rows = vec![];
        for r in 0..4 {
            let t = r as f64 / 3.0;
            let v = v0 + (v1 - v0) * t;
            let z = t * t;
            rows.push([
                self.vertex([0.0, t, z], [0.0, -z, 1.0], [u0, v]),
                self.vertex([1.0, t, z], [0.0, -z, 1.0], [u1, v]),
            ]);
        }
        for r in 0..3 {
            let (a, b) = (rows[r], rows[r + 1]);
            self.i.extend([a[0], a[1], b[1], a[0], b[1], b[0]]);
        }
    }
    fn triangle(&mut self, uv: [P; 3]) {
        let ids = uv.map(|t| self.vertex([t[0], t[1], t[0] * t[1]], [0.0, 1.0, 0.0], t));
        self.i.extend(ids);
    }
}

fn rect(u0: f64, v0: f64, u1: f64, v1: f64) -> [P; 4] {
    [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]
}

fn options(budget: u32, padding: f32, threshold: u8, wrap: AlphaWrap) -> CardTrimOptions {
    let mut o = CardTrimOptions::default();
    o.max_vertices = budget;
    o.padding = padding;
    o.threshold = threshold;
    o.wrap = wrap;
    o
}

fn trim(mesh: &Mesh, mask: &Mask, o: CardTrimOptions) -> TrimmedCards {
    trim_alpha_cards(
        &mesh.i,
        &mesh.v,
        STRIDE,
        UV,
        mask.view(),
        o,
        &mut Workspace::default(),
    )
    .unwrap()
}

fn texel(v: &[f32], i: u32, mask: &Mask) -> P {
    let b = i as usize * STRIDE + UV;
    [
        f64::from(v[b]) * f64::from(mask.w),
        f64::from(v[b + 1]) * f64::from(mask.h),
    ]
}
fn position(v: &[f32], i: u32) -> [f64; 3] {
    let b = i as usize * STRIDE;
    [v[b], v[b + 1], v[b + 2]].map(f64::from)
}
fn bary(p: P, t: [P; 3]) -> [f64; 3] {
    let area = cross(sub(t[1], t[0]), sub(t[2], t[0]));
    let l1 = cross(sub(p, t[0]), sub(t[2], t[0])) / area;
    let l2 = cross(sub(t[1], t[0]), sub(p, t[0])) / area;
    [1.0 - l1 - l2, l1, l2]
}
fn segment_distance(p: P, a: P, b: P) -> f64 {
    let d = sub(b, a);
    let t = ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / (d[0] * d[0] + d[1] * d[1]);
    let t = t.clamp(0.0, 1.0);
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
fn interpolate(v: &[f32], ids: [u32; 3], w: [f64; 3]) -> [f64; 3] {
    let p = ids.map(|i| position(v, i));
    [0, 1, 2].map(|c| w[0] * p[0][c] + w[1] * p[1][c] + w[2] * p[2][c])
}

/// Clip a convex polygon to a triangle (either orientation).
fn clip(mut poly: Vec<P>, t: [P; 3]) -> Vec<P> {
    let s = cross(sub(t[1], t[0]), sub(t[2], t[0])).signum();
    for k in 0..3 {
        let (a, b) = (t[k], t[(k + 1) % 3]);
        let mut out = vec![];
        for m in 0..poly.len() {
            let (p, q) = (poly[m], poly[(m + 1) % poly.len()]);
            let (dp, dq) = (
                s * cross(sub(b, a), sub(p, a)),
                s * cross(sub(b, a), sub(q, a)),
            );
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

/// Every visible footprint point inside an input triangle is covered by an
/// output triangle with the same surface position. Returns checked points.
fn check_coverage(mesh: &Mesh, mask: &Mask, o: CardTrimOptions, out: &TrimmedCards) -> usize {
    let pad = f64::from(o.padding);
    let out_tris: Vec<([P; 3], [u32; 3])> = out
        .indices
        .chunks(3)
        .map(|c| {
            let ids = [c[0], c[1], c[2]];
            (ids.map(|i| texel(&out.vertices, i, mask)), ids)
        })
        .collect();
    // Bucket output triangles by 8-texel cells of their padded bounds.
    let cell = |x: f64| (x / 8.0).floor() as i64;
    let mut grid = std::collections::HashMap::<(i64, i64), Vec<usize>>::new();
    for (k, (q, _)) in out_tris.iter().enumerate() {
        let lo = [0, 1].map(|c| cell(q.iter().map(|p| p[c]).fold(f64::INFINITY, f64::min) - TOL));
        let hi =
            [0, 1].map(|c| cell(q.iter().map(|p| p[c]).fold(f64::NEG_INFINITY, f64::max) + TOL));
        for y in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                grid.entry((x, y)).or_default().push(k);
            }
        }
    }
    let mut checked = 0;
    for c in mesh.i.chunks(3) {
        let ids = [c[0], c[1], c[2]];
        let t = ids.map(|i| texel(&mesh.v, i, mask));
        if cross(sub(t[1], t[0]), sub(t[2], t[0])) == 0.0 {
            continue;
        }
        let x0 = t.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let x1 = t.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let y0 = t.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
        let y1 = t.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
        for j in (y0 - 1.0 - pad).floor() as i64..=(y1 + pad).ceil() as i64 {
            for i in (x0 - 1.0 - pad).floor() as i64..=(x1 + pad).ceil() as i64 {
                if mask.at(i, j, o.wrap) < o.threshold {
                    continue;
                }
                let (a, b) = (i as f64 - pad, i as f64 + 1.0 + pad);
                let (e, f) = (j as f64 - pad, j as f64 + 1.0 + pad);
                for p in clip(vec![[a, e], [b, e], [b, f], [a, f]], t) {
                    let want = interpolate(&mesh.v, ids, bary(p, t));
                    let near = grid.get(&(cell(p[0]), cell(p[1]))).map_or(&[][..], |v| v);
                    let ok = near.iter().map(|&k| &out_tris[k]).any(|(q, qid)| {
                        distance(p, *q) <= TOL && {
                            let got = interpolate(&out.vertices, *qid, bary(p, *q));
                            (0..3).all(|k| (got[k] - want[k]).abs() <= 1e-4)
                        }
                    });
                    assert!(ok, "visible point {p:?} of texel ({i}, {j}) not covered");
                    checked += 1;
                }
            }
        }
    }
    checked
}

fn uv_area(v: &[f32], idx: &[u32], mask: &Mask) -> f64 {
    idx.chunks(3)
        .map(|c| {
            let t = [c[0], c[1], c[2]].map(|i| texel(v, i, mask));
            cross(sub(t[1], t[0]), sub(t[2], t[0])).abs() * 0.5
        })
        .sum()
}

#[test]
fn blob_card_is_trimmed_within_budget_and_covered() {
    let mut rng = Rng(7);
    let mask = blobs(&mut rng, 64, 64, 3);
    let mut mesh = Mesh::default();
    mesh.quad(rect(0.0, 0.0, 1.0, 1.0), false);
    let mut last = f64::INFINITY;
    for budget in 3..=8 {
        let o = options(budget, 0.5, 128, AlphaWrap::Clamp);
        let out = trim(&mesh, &mask, o);
        let r = out.report;
        if budget == 3 && r.unchanged_cards == 1 {
            continue; // No enclosing triangle inside the card.
        }
        assert_eq!((r.cards, r.trimmed_cards, r.split_cards), (1, 1, 0));
        assert!(out.vertices.len() / STRIDE <= budget as usize);
        assert!(r.area_after <= last && r.area_after < r.area_before);
        assert!((uv_area(&out.vertices, &out.indices, &mask) - r.area_after).abs() < 1e-3);
        last = r.area_after;
        assert!(check_coverage(&mesh, &mask, o, &out) > 0);
    }
}

#[test]
fn generated_cards_are_covered() {
    let mut rng = Rng(0x5eed);
    let mut checked = 0;
    for case in 0..24 {
        let (w, h) = (8 + (rng.next() % 56) as u32, 8 + (rng.next() % 56) as u32);
        let count = 1 + (rng.next() % 4) as usize;
        let mask = blobs(&mut rng, w, h, count);
        let wrap = if case % 3 == 0 {
            AlphaWrap::Repeat
        } else {
            AlphaWrap::Clamp
        };
        let mut mesh = Mesh::default();
        let lo = if wrap == AlphaWrap::Repeat { -0.6 } else { 0.0 };
        let corner = |rng: &mut Rng| [lo + rng.f() * 0.5, lo + rng.f() * 0.5];
        let a = corner(&mut rng);
        mesh.quad(
            rect(
                a[0],
                a[1],
                a[0] + 0.2 + rng.f() * 0.5,
                a[1] + 0.2 + rng.f() * 0.5,
            ),
            case % 2 == 1,
        );
        // A rotated planar quad.
        let (cx, cy, r, ang) = (
            0.3 + rng.f() * 0.4,
            0.3 + rng.f() * 0.4,
            0.1 + rng.f() * 0.2,
            rng.f() * 3.0,
        );
        let rot = |k: f64| [cx + r * (ang + k).cos(), cy + r * (ang + k).sin()];
        mesh.quad([rot(0.0), rot(1.6), rot(3.1), rot(4.7)], false);
        let b = corner(&mut rng);
        mesh.trapezoid(rect(b[0], b[1], b[0] + 0.4, b[1] + 0.5));
        mesh.strip(0.1 + rng.f() * 0.3, 0.6 + rng.f() * 0.3, 0.0, 1.0);
        mesh.triangle([[0.1, 0.1], [0.9, 0.2], [0.4, 0.9]]);
        // An L of three planar quads: affine but not convex.
        let base = mesh.v.len() / STRIDE;
        let grid = [
            [0.0, 0.0],
            [0.5, 0.0],
            [1.0, 0.0],
            [0.0, 0.5],
            [0.5, 0.5],
            [1.0, 0.5],
            [0.0, 1.0],
            [0.5, 1.0],
        ];
        for g in grid {
            mesh.vertex([g[0], g[1], 0.0], [0.0, 0.0, 1.0], g);
        }
        let q =
            |a: usize, b: usize, c: usize, d: usize| [a, b, c, a, c, d].map(|x| (base + x) as u32);
        mesh.i.extend(q(0, 1, 4, 3));
        mesh.i.extend(q(1, 2, 5, 4));
        mesh.i.extend(q(3, 4, 7, 6));
        for (budget, padding, threshold) in [(4, 0.5, 128), (6, 0.0, 1), (8, 1.5, 200)] {
            let o = options(budget, padding, threshold, wrap);
            let out = trim(&mesh, &mask, o);
            let r = out.report;
            assert_eq!(r.cards, 6);
            assert_eq!(r.trimmed_cards + r.removed_cards + r.unchanged_cards, 6);
            assert!(r.area_after <= r.area_before);
            assert!(
                (uv_area(&out.vertices, &out.indices, &mask) - r.area_after).abs()
                    < 1e-3 * r.area_before.max(1.0)
            );
            checked += check_coverage(&mesh, &mask, o, &out);
        }
    }
    assert!(checked > 10_000, "{checked}");
}

#[test]
fn non_affine_cards_split_and_planar_cards_do_not() {
    let mut rng = Rng(3);
    let mask = blobs(&mut rng, 32, 32, 2);
    let o = options(6, 0.5, 128, AlphaWrap::Clamp);
    let mut planar = Mesh::default();
    planar.quad(rect(0.0, 0.0, 1.0, 1.0), true);
    let r = trim(&planar, &mask, o).report;
    assert_eq!((r.trimmed_cards, r.split_cards), (1, 0));
    let mut bent = Mesh::default();
    bent.trapezoid(rect(0.0, 0.0, 1.0, 1.0));
    let out = trim(&bent, &mask, o);
    assert_eq!((out.report.trimmed_cards, out.report.split_cards), (1, 1));
    check_coverage(&bent, &mask, o, &out);
}

#[test]
fn empty_full_and_degenerate_inputs() {
    let empty = Mask {
        data: vec![0; 256],
        w: 16,
        h: 16,
    };
    let full = Mask {
        data: vec![255; 256],
        w: 16,
        h: 16,
    };
    let mut mesh = Mesh::default();
    mesh.quad(rect(0.0, 0.0, 1.0, 1.0), false);
    mesh.triangle([[0.2, 0.2], [0.4, 0.4], [0.6, 0.6]]); // zero texture area
    let o = CardTrimOptions::default();
    let out = trim(&mesh, &empty, o);
    let r = out.report;
    assert_eq!(
        (r.cards, r.removed_cards, r.passthrough_triangles),
        (1, 1, 1)
    );
    assert_eq!((out.indices.len(), out.vertices.len()), (3, 3 * STRIDE));
    assert_eq!((r.area_after, r.opaque_texels), (0.0, 0));
    let out = trim(&mesh, &full, o);
    assert_eq!(
        (out.report.unchanged_cards, out.report.output_triangles),
        (1, 3)
    );
    assert_eq!(out.vertices, mesh.v[..7 * STRIDE]);
    assert_eq!(out.report.opaque_texels, 256);
    // A single visible texel: the result covers it at every budget.
    let mut one = empty.data.clone();
    one[5 * 16 + 9] = 255;
    let one = Mask {
        data: one,
        w: 16,
        h: 16,
    };
    for budget in [3, 4, 16] {
        let o = options(budget, 0.0, 128, AlphaWrap::Clamp);
        let out = trim(&mesh, &one, o);
        assert_eq!(out.report.trimmed_cards, 1);
        // The footprint is a 1.008 texel square; the best candidate triangle is 2.25x.
        assert!(out.report.area_after < if budget == 3 { 2.4 } else { 1.1 });
        check_coverage(&mesh, &one, o, &out);
    }
    let none = trim_alpha_cards(
        &[],
        &[],
        STRIDE,
        UV,
        one.view(),
        o,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(none.report, CardTrimReport::default());
}

#[test]
fn deterministic_and_index_widths_agree() {
    let mut rng = Rng(11);
    let mask = blobs(&mut rng, 40, 24, 3);
    let mut mesh = Mesh::default();
    for k in 0..20 {
        let u = (k % 5) as f64 * 0.2;
        mesh.quad(rect(u, 0.0, u + 0.2, 1.0), k % 2 == 0);
        mesh.strip(u, u + 0.2, 0.0, 1.0);
    }
    let o = options(5, 0.5, 100, AlphaWrap::Clamp);
    let a = trim(&mesh, &mask, o);
    let b = trim(&mesh, &mask, o);
    assert_eq!(a, b);
    let narrow: Vec<u16> = mesh.i.iter().map(|&i| i as u16).collect();
    let c = trim_alpha_cards_generic(
        &narrow,
        &mesh.v,
        STRIDE,
        UV,
        mask.view(),
        o,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(c.vertices, a.vertices);
    assert_eq!(
        c.indices.iter().map(|&i| u32::from(i)).collect::<Vec<_>>(),
        a.indices
    );
    assert_eq!(c.report, a.report);
}

#[test]
fn invalid_inputs_are_rejected() {
    let mask = Mask {
        data: vec![255; 16],
        w: 4,
        h: 4,
    };
    let mut mesh = Mesh::default();
    mesh.quad(rect(0.0, 0.0, 1.0, 1.0), false);
    let call = |i: &[u32], v: &[f32], stride: usize, uv: usize, o: CardTrimOptions| {
        trim_alpha_cards(i, v, stride, uv, mask.view(), o, &mut Workspace::default()).map(|_| ())
    };
    let o = CardTrimOptions::default();
    assert_eq!(
        call(
            &mesh.i,
            &mesh.v,
            STRIDE,
            UV,
            options(2, 0.5, 128, AlphaWrap::Clamp)
        ),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        call(
            &mesh.i,
            &mesh.v,
            STRIDE,
            UV,
            options(17, 0.5, 128, AlphaWrap::Clamp)
        ),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        call(
            &mesh.i,
            &mesh.v,
            STRIDE,
            UV,
            options(8, f32::NAN, 128, AlphaWrap::Clamp)
        ),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        call(
            &mesh.i,
            &mesh.v,
            STRIDE,
            UV,
            options(8, -1.0, 128, AlphaWrap::Clamp)
        ),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        call(&mesh.i, &mesh.v, STRIDE, 7, o),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        call(&mesh.i, &mesh.v[..31], STRIDE, UV, o),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        call(&mesh.i[..4], &mesh.v, STRIDE, UV, o),
        Err(Error::InvalidTopology)
    );
    assert_eq!(
        call(&[0, 1, 9], &mesh.v, STRIDE, UV, o),
        Err(Error::IndexOutOfBounds)
    );
    let mut nan = mesh.v.clone();
    nan[2] = f32::NAN;
    assert_eq!(
        call(&mesh.i, &nan, STRIDE, UV, o),
        Err(Error::NumericalFailure)
    );
    let mut far = mesh.v.clone();
    far[UV] = 1e7;
    assert_eq!(
        call(&mesh.i, &far, STRIDE, UV, o),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        AlphaMask::new(&[], 0, 1).map(|_| ()),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        AlphaMask::new(&[0; 3], 2, 2).map(|_| ()),
        Err(Error::InvalidLayout)
    );
    let rgba = [0u8, 0, 0, 7, 0, 0, 0, 9];
    let view = AlphaMask::with_layout(&rgba[3..], 4, 8, 2, 1).unwrap();
    assert_eq!(
        (view.alpha(0, 0), view.alpha(1, 0), view.alpha(2, 0)),
        (Some(7), Some(9), None)
    );
    let mut ws = Workspace::new(Limits {
        max_work: 10,
        ..Limits::default()
    });
    assert_eq!(
        trim_alpha_cards(&mesh.i, &mesh.v, STRIDE, UV, mask.view(), o, &mut ws).map(|_| ()),
        Err(Error::LimitExceeded)
    );
}

/// Real masks prepared outside the repository: set `CARD_TRIM_REAL_DIR` to a
/// directory of `*.cards` files (u32 width, height, vertex count, index count;
/// width*height alpha bytes; x, y, z, u, v f32 per vertex; u32 indices).
/// Cards with distinct UV layouts (an evenly spaced sample of at most
/// `CARD_TRIM_REAL_LIMIT`, default 200, per file) are checked at budgets 4, 6 and 8.
#[test]
fn real_masks_are_covered() {
    let Some(dir) = std::env::var_os("CARD_TRIM_REAL_DIR") else {
        return;
    };
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.retain(|p| p.extension().is_some_and(|e| e == "cards"));
    files.sort();
    assert!(!files.is_empty());
    for path in files {
        let bytes = std::fs::read(&path).unwrap();
        let word = |k: usize| u32::from_le_bytes(bytes[k * 4..k * 4 + 4].try_into().unwrap());
        let (w, h, nv, ni) = (word(0), word(1), word(2) as usize, word(3) as usize);
        let mut at = 16;
        let mask = Mask {
            data: bytes[at..at + (w * h) as usize].to_vec(),
            w,
            h,
        };
        at += (w * h) as usize;
        let float =
            |k: usize| f32::from_le_bytes(bytes[at + k * 4..at + k * 4 + 4].try_into().unwrap());
        let mut mesh = Mesh::default();
        for v in 0..nv {
            let f: Vec<f32> = (0..5).map(|c| float(v * 5 + c)).collect();
            mesh.v.extend([f[0], f[1], f[2], 0.0, 0.0, 1.0, f[3], f[4]]);
        }
        at += nv * 20;
        let all: Vec<u32> = (0..ni)
            .map(|k| u32::from_le_bytes(bytes[at + k * 4..at + k * 4 + 4].try_into().unwrap()))
            .collect();
        let mut seen = std::collections::BTreeSet::new();
        let mut distinct = vec![];
        for card in cards(&all) {
            let uv_key: Vec<u32> = card
                .iter()
                .flat_map(|&t| all[t * 3..t * 3 + 3].to_vec())
                .flat_map(|i| {
                    [
                        mesh.v[i as usize * STRIDE + UV].to_bits(),
                        mesh.v[i as usize * STRIDE + UV + 1].to_bits(),
                    ]
                })
                .collect();
            if seen.insert(uv_key) {
                distinct.push(card);
            }
        }
        // An evenly spaced sample of at most CARD_TRIM_REAL_LIMIT (default 200) layouts.
        let limit: usize =
            std::env::var("CARD_TRIM_REAL_LIMIT").map_or(200, |s| s.parse().unwrap());
        let step = distinct.len().div_ceil(limit.max(1)).max(1);
        let (mut checked, mut sampled, mut trimmed) = (0, 0, 0);
        for card in distinct.iter().step_by(step) {
            let mut one = Mesh {
                v: mesh.v.clone(),
                i: vec![],
            };
            for &t in card {
                one.i.extend_from_slice(&all[t * 3..t * 3 + 3]);
            }
            for budget in [4, 6, 8] {
                let o = options(budget, 0.5, 128, AlphaWrap::Repeat);
                let out = trim(&one, &mask, o);
                trimmed += out.report.trimmed_cards;
                checked += check_coverage(&one, &mask, o, &out);
            }
            sampled += 1;
        }
        eprintln!(
            "{}: {sampled} of {} layouts, {trimmed} trimmed runs, {checked} footprint corners covered",
            path.display(),
            distinct.len()
        );
    }
}

/// Edge-connected triangle groups, as the library forms cards.
fn cards(indices: &[u32]) -> Vec<Vec<usize>> {
    let n = indices.len() / 3;
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(p: &mut [usize], mut x: usize) -> usize {
        while p[x] != x {
            p[x] = p[p[x]];
            x = p[x];
        }
        x
    }
    let mut edges = std::collections::BTreeMap::new();
    for t in 0..n {
        for k in 0..3 {
            let (a, b) = (indices[t * 3 + k], indices[t * 3 + (k + 1) % 3]);
            let key = (a.min(b), a.max(b));
            if let Some(&u) = edges.get(&key) {
                let (ra, rb) = (find(&mut parent, t), find(&mut parent, u));
                parent[ra.max(rb)] = ra.min(rb);
            } else {
                edges.insert(key, t);
            }
        }
    }
    let mut groups = std::collections::BTreeMap::<usize, Vec<usize>>::new();
    for t in 0..n {
        let r = find(&mut parent, t);
        groups.entry(r).or_default().push(t);
    }
    groups.into_values().collect()
}

#[test]
fn vertex_cost_stops_adding_vertices_that_save_little() {
    let mut rng = Rng(7);
    let mask = blobs(&mut rng, 64, 64, 3);
    let mut mesh = Mesh::default();
    mesh.quad(rect(0.0, 0.0, 1.0, 1.0), false);
    let run = |budget, cost| {
        let mut o = options(budget, 0.5, 128, AlphaWrap::Clamp);
        o.vertex_cost = cost;
        trim(&mesh, &mask, o)
    };
    // Zero cost is the default: the full budget.
    let full = run(8, 0.0);
    assert_eq!(
        full,
        trim(&mesh, &mask, options(8, 0.5, 128, AlphaWrap::Clamp))
    );
    assert_eq!(full.card_vertices.len(), full.report.cards);
    assert_eq!(full.card_vertices[0] as usize, full.vertices.len() / STRIDE);
    // Area saved by each extra vertex, from the budget sweep.
    let areas: Vec<f64> = (4..=8).map(|b| run(b, 0.0).report.area_after).collect();
    let gains: Vec<f64> = areas.windows(2).map(|w| w[0] - w[1]).collect();
    assert!(
        gains.iter().all(|&g| g >= 0.0) && gains[0] > 1.0,
        "{gains:?}"
    );
    // A cost between two gains stops at the vertex count whose next vertex saves less.
    let mut stops = 0;
    for k in 1..gains.len() {
        if gains[k - 1] > gains[k] + 1.0 {
            let cost = ((gains[k - 1] + gains[k]) / 2.0) as f32;
            let out = run(8, cost);
            let at_budget = run(4 + k as u32, 0.0);
            assert_eq!(out.card_vertices, at_budget.card_vertices);
            assert_eq!(out.report.area_after, at_budget.report.area_after);
            stops += 1;
        }
    }
    assert!(stops > 0, "{gains:?}");
    // More cost never adds vertices or removes area.
    let (mut vertices, mut area) = (u32::MAX, 0.0);
    for cost in [0.0, 1.0, 10.0, 100.0, 1000.0, 1e9] {
        let out = run(8, cost);
        assert!(out.card_vertices[0] <= vertices && out.report.area_after >= area);
        (vertices, area) = (out.card_vertices[0], out.report.area_after);
    }
    for cost in [-1.0, f32::NAN, f32::INFINITY] {
        let mut o = CardTrimOptions::default();
        o.vertex_cost = cost;
        let r = trim_alpha_cards(
            &mesh.i,
            &mesh.v,
            STRIDE,
            UV,
            mask.view(),
            o,
            &mut Workspace::default(),
        );
        assert_eq!(r.map(|_| ()), Err(Error::InvalidParameter));
    }
}

/// A folded card (one triangle inside another in UV, other positions) whose
/// fitted corner lies on both triangles' outlines at the same UV: each piece
/// keeps its own triangle's attributes (fuzz finding, 0.2.18 trim_cards).
#[test]
fn folded_card_corner_on_both_outlines_keeps_each_triangles_attributes() {
    let v1 = (104.0f32 / 176.0) as f64;
    let mut mesh = Mesh::default();
    mesh.vertex([-1.0, -1.0, 0.0], [7.47, 0.9, 0.0], [0.0, 0.0]);
    mesh.vertex([1.0, -1.0, 5.5], [6.06, 0.48, 0.0], [0.0, v1]);
    mesh.vertex([0.0, 1.0, 0.0], [6.49, 8.26, 0.0], [1.0, 1.0]);
    mesh.vertex([-1.0, 1.0, 0.0], [7.61, 0.75, 0.0], [0.0, 1.0]);
    mesh.i.extend([0, 1, 2, 0, 2, 3]);
    let data = vec![
        45, 73, 83, 83, 99, 68, 27, 130, 58, 91, 0, 0, 0, 0, 0, 0, 232, 173, 232, 104, 124, 60,
        15, 37, 159, 111, 247, 116, 58, 203, 186, 20, 43, 248, 66, 100, 145, 134, 23, 55, 0, 0, 0,
        2, 245, 8, 253, 84, 20, 68, 55, 207, 55, 53, 179, 64, 90, 168, 29, 29, 110, 90, 218, 26,
    ];
    let mask = Mask { data, w: 8, h: 8 };
    let o = options(8, 128.0 / 255.0 * 3.0, 100, AlphaWrap::Clamp);
    let out = trim(&mesh, &mask, o);
    assert_eq!((out.report.trimmed_cards, out.report.split_cards), (1, 1));
    assert!(check_coverage(&mesh, &mask, o, &out) > 0);
}

/// A sliver card at threshold 0 is copied, not reported as trimmed because two
/// area sums of the same triangle round differently (fuzz finding, 0.2.18).
#[test]
fn sliver_card_at_threshold_zero_is_unchanged() {
    let c = |b: f32| f64::from((b - 40.0) / 176.0);
    let mut mesh = Mesh::default();
    mesh.vertex([0.0; 3], [0.0, 0.0, 1.0], [c(0.0), c(0.0)]);
    mesh.vertex([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [c(124.0), c(62.0)]);
    mesh.vertex([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [c(116.0), c(58.0)]);
    mesh.i.extend([0, 1, 2]);
    let mask = Mask { data: vec![0; 8 * 15], w: 8, h: 15 };
    let mut o = options(6, 0.0, 0, AlphaWrap::Clamp);
    o.vertex_cost = 0.0;
    let r = trim(&mesh, &mask, o).report;
    assert_eq!((r.unchanged_cards, r.trimmed_cards, r.removed_cards), (1, 0, 0), "{r:?}");
}
