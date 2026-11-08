//! Upward-rounded `f64` arithmetic shared by the cut reference and the MDAG 2
//! writer and reader, so that a bound written by one passes the other's test.
//!
//! Each helper returns a value no smaller than the exact result and the exact
//! result itself whenever it is representable, so equal spheres still contain
//! each other.
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}
pub(crate) fn add_up(a: f64, b: f64) -> f64 {
    let (s, e) = two_sum(a, b);
    if e > 0.0 {
        s.next_up()
    } else {
        s
    }
}
fn mul_up(a: f64, b: f64) -> f64 {
    let p = a * b;
    if libm::fma(a, b, -p) > 0.0 {
        p.next_up()
    } else {
        p
    }
}
pub(crate) fn sqrt_up(x: f64) -> f64 {
    let r = libm::sqrt(x);
    if libm::fma(r, r, -x) < 0.0 {
        r.next_up()
    } else {
        r
    }
}
/// Upper bound of `|a - b|`.
fn abs_diff_up(a: f64, b: f64) -> f64 {
    let (s, e) = two_sum(a, -b);
    add_up(s.abs(), e.abs())
}
/// Upper bound of `|p0 + p1 + p2|`.
#[cfg_attr(not(feature = "clusterlod"), allow(dead_code))]
pub(crate) fn abs_sum3_up(p: [f64; 3]) -> f64 {
    let (s1, e1) = two_sum(p[0], p[1]);
    let (s2, e2) = two_sum(s1, p[2]);
    add_up(add_up(s2.abs(), e1.abs()), e2.abs())
}
/// Smallest `f32` not below `v`.
#[cfg_attr(not(feature = "clusterlod"), allow(dead_code))]
pub(crate) fn f32_up(v: f64) -> f32 {
    let f = v as f32;
    if f64::from(f) < v {
        f.next_up()
    } else {
        f
    }
}

/// A sphere containing every sphere of `spheres` (center, radius), centered on
/// the middle of their bounding box, with the radius from
/// [`containing_radius`] rounded up to `f32`. `None` when `spheres` is empty
/// or the result is not finite.
pub(crate) fn enclosing_sphere<I>(spheres: I) -> Option<([f32; 3], f32)>
where
    I: Iterator<Item = ([f32; 3], f32)> + Clone,
{
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for (center, radius) in spheres.clone() {
        for i in 0..3 {
            let (c, r) = (f64::from(center[i]), f64::from(radius));
            lo[i] = lo[i].min(c - r);
            hi[i] = hi[i].max(c + r);
        }
    }
    if !(lo.iter().chain(&hi).all(|v| v.is_finite())) {
        return None;
    }
    let center = [0, 1, 2].map(|i| (0.5 * (lo[i] + hi[i])) as f32);
    let mut radius = 0.0f64;
    for (c, r) in spheres {
        radius = radius.max(containing_radius(center, c, r));
    }
    let radius = f32_up(radius);
    (finite_center(center) && radius.is_finite()).then_some((center, radius))
}

fn finite_center(c: [f32; 3]) -> bool {
    c.iter().all(|v| v.is_finite())
}

/// Upper bound of `|center_p - center_q| + radius_q`: the radius a sphere at
/// `center_p` needs to contain the sphere at `center_q` with `radius_q`.
pub(crate) fn containing_radius(center_p: [f32; 3], center_q: [f32; 3], radius_q: f32) -> f64 {
    let mut d2 = 0.0;
    for i in 0..3 {
        let d = abs_diff_up(f64::from(center_p[i]), f64::from(center_q[i]));
        d2 = add_up(d2, mul_up(d, d));
    }
    add_up(sqrt_up(d2), f64::from(radius_q))
}
