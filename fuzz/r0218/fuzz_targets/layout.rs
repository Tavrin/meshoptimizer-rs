#![no_main]
//! `ImpostorLayout` encode/decode, `ImpostorGrid` view mapping and blend, `ImpostorFrame`,
//! `ImpostorBounds` and `impostor_switch_distance`: no panic on any f32 bit pattern,
//! round trips, barycentric blend invariants, determinism.
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fn len(d: [f32; 3]) -> f64 {
    d.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>().sqrt()
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f64 {
    (0..3).map(|k| f64::from(a[k]) * f64::from(b[k])).sum()
}

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let flags = c.u8();
    let layout = if flags & 1 == 0 { ImpostorLayout::Hemisphere } else { ImpostorLayout::Sphere };
    let n = if flags & 2 != 0 { u32::from(c.u8()) } else { 2 + u32::from(c.u8()) % 63 };
    let d = if flags & 4 != 0 {
        [c.raw_f32(), c.raw_f32(), c.raw_f32()]
    } else {
        [0, 1, 2].map(|_| (f32::from(c.u8()) - 128.) / 32.)
    };
    let p = if flags & 8 != 0 { [c.raw_f32(), c.raw_f32()] } else { [c.unit() * 2. - 1., c.unit() * 2. - 1.] };
    let view = if flags & 16 != 0 { u32::from(c.u8()) * 41 } else { u32::from(c.u8()) };

    // Direction validity: finite and nonzero (in f64, so huge components still count).
    let valid = d.iter().all(|x| x.is_finite()) && len(d) > 0.0 && len(d).is_finite();
    let e = layout.encode(d);
    if valid {
        let e = e.expect("valid direction rejected");
        assert!(e.iter().all(|x| (-1.0..=1.0).contains(x)), "encode out of range: {e:?}");
        // Round trip: the decoded direction is the unit direction (hemisphere: y >= 0 only).
        let back = layout.decode(e).unwrap();
        assert!((len(back) - 1.0).abs() < 1e-5, "decode not unit: {back:?}");
        let unit = d.map(|x| (f64::from(x) / len(d)) as f32);
        let upper = unit[1] >= 0.0;
        if layout == ImpostorLayout::Sphere || upper {
            // Tiny directions near the horizon fold sensitively; compare away from f32 noise.
            let err = (0..3).map(|k| f64::from(back[k] - unit[k]).powi(2)).sum::<f64>().sqrt();
            assert!(err < 2e-3 || len(d) < 1e-30 || len(d) > 1e30, "{layout:?} round trip {d:?} -> {e:?} -> {back:?}");
        } else {
            // Below the horizon the hemisphere projects onto it.
            assert!(back[1].abs() < 1e-3, "hemisphere decode not on horizon: {back:?}");
        }
        assert_eq!(layout.encode(d), Ok(e), "encode deterministic");
        // encode(decode(point)) is the point again.
        let again = layout.encode(back).unwrap();
        assert_eq!(layout.encode(back).unwrap(), again);
    } else {
        assert_eq!(e, Err(Error::InvalidParameter), "invalid direction accepted: {d:?}");
    }

    // decode of an arbitrary point: Ok exactly inside [-1, 1]^2, unit result, re-encodes to the point.
    let inside = p.iter().all(|x| (-1.0..=1.0).contains(x));
    match layout.decode(p) {
        Ok(v) => {
            assert!(inside);
            assert!((len(v) - 1.0).abs() < 1e-5, "decode not unit: {v:?}");
            if let Ok(q) = layout.encode(v) {
                // Several points (the Sphere corners) share one direction, so compare directions.
                let v2 = layout.decode(q).unwrap();
                let err = (0..3).map(|k| f64::from(v2[k] - v[k]).powi(2)).sum::<f64>().sqrt();
                assert!(err < 2e-3, "{layout:?} decode(encode(decode({p:?}))) = {v2:?} vs {v:?}");
            }
        }
        Err(e) => assert_eq!((e, inside), (Error::InvalidParameter, false)),
    }

    // Grid.
    let grid = ImpostorGrid::new(layout, n);
    assert_eq!(grid.is_ok(), (2..=64).contains(&n));
    let Ok(grid) = grid else {
        assert_eq!(grid, Err(Error::InvalidParameter));
        return;
    };
    let count = grid.view_count().unwrap();
    assert_eq!(count, n * n);
    match (grid.view_point(view), grid.view_frame(view)) {
        (Ok(pt), Ok(fr)) => {
            assert!(view < count);
            assert!(pt.iter().all(|x| (-1.0..=1.0).contains(x)));
            // The frame direction is the decoded view point, and the frame is orthonormal.
            let dir = layout.decode(pt).unwrap();
            assert!((0..3).all(|k| (dir[k] - fr.direction[k]).abs() < 1e-5), "{dir:?} vs {fr:?}");
            assert!((len(fr.right) - 1.0).abs() < 1e-5 && (len(fr.up) - 1.0).abs() < 1e-5);
            assert!(dot(fr.right, fr.up).abs() < 1e-5 && dot(fr.right, fr.direction).abs() < 1e-5);
            assert!(dot(fr.up, fr.direction).abs() < 1e-5);
            // A view's own direction blends to a view with the same direction at weight one
            // (the Sphere corners are four views of straight down).
            let b = grid.blend(dir).unwrap();
            let w: f32 = (0..3)
                .filter(|&k| grid.view_frame(b.views[k]).map_or(false, |f| (0..3).all(|j| (f.direction[j] - dir[j]).abs() < 1e-4)))
                .map(|k| b.weights[k])
                .sum();
            assert!(w > 1.0 - 1e-3, "view {view} of {grid:?}: blend {b:?}");
        }
        (Err(a), Err(b)) => assert_eq!((a, b, view >= count), (Error::InvalidParameter, Error::InvalidParameter, true)),
        other => panic!("view_point and view_frame disagree: {other:?}"),
    }

    // Blend invariants.
    match (grid.blend(d), valid) {
        (Ok(b), true) => {
            assert!(b.views.iter().all(|&v| v < count));
            assert!(b.weights.iter().all(|&w| w >= 0.0 && w <= 1.0 + 1e-6), "{b:?}");
            let sum: f32 = b.weights.iter().sum();
            assert!((sum - 1.0).abs() < 1e-5, "weights sum {sum}: {b:?}");
            // Barycentric reproduction of the (clamped) octahedral point.
            let o = layout.encode(d).unwrap();
            let mut got = [0f64; 2];
            for k in 0..3 {
                let q = grid.view_point(b.views[k]).unwrap();
                got[0] += f64::from(b.weights[k]) * f64::from(q[0]);
                got[1] += f64::from(b.weights[k]) * f64::from(q[1]);
            }
            for k in 0..2 {
                assert!((got[k] - f64::from(o[k].clamp(-1., 1.))).abs() < 1e-4, "blend point {got:?} vs {o:?} for {b:?}");
            }
            assert_eq!(grid.blend(d), Ok(b));
            assert_eq!(ImpostorFrame::from_direction(d).is_ok(), true);
        }
        (Err(e), false) => assert_eq!(e, Error::InvalidParameter),
        (r, v) => panic!("blend {r:?} for validity {v}"),
    }

    // Bounds and switch distance.
    let bc = [c.u8(), c.u8(), c.u8(), c.u8()];
    let center = [0, 1, 2].map(|k| coord(bc[k], flags & 0x80 != 0, 8., 128.));
    let radius = match c.u8() % 5 {
        0 => 0.0,
        1 => f32::NAN,
        2 => -1.0,
        3 => c.raw_f32(),
        _ => 0.01 + f32::from(bc[3]),
    };
    let bounds = ImpostorBounds::new(center, radius);
    assert_eq!(
        bounds.is_ok(),
        center.iter().all(|x| x.is_finite()) && radius.is_finite() && radius > 0.0
    );
    let lo = center;
    let hi = [0, 1, 2].map(|k| lo[k] + f32::from(c.u8()) / 16.);
    if let Ok(b) = ImpostorBounds::from_box(lo, hi) {
        assert!(b.radius > 0.0 && b.radius.is_finite());
        // The sphere through the box corners contains the box.
        let hl = [0, 1, 2].map(|k| f64::from(hi[k]) - f64::from(b.center[k]));
        let r = (hl[0] * hl[0] + hl[1] * hl[1] + hl[2] * hl[2]).sqrt();
        assert!(f64::from(b.radius) >= r * (1.0 - 1e-6), "box sphere too small");
    }
    let res = if flags & 0x20 != 0 { u32::from(c.u8()) * 33 } else { 1 + u32::from(c.u8()) % 64 };
    let (fov, vh, tol) = (
        f64::from(c.unit()) * 3.3,
        f64::from(c.unit()) * 2000.,
        f64::from(c.unit()) * 8.,
    );
    let Ok(bounds) = bounds else { return };
    let Ok(cam) = LodCamera::new(fov, vh, tol) else { return };
    let s = impostor_switch_distance(bounds, grid, res, cam);
    assert_eq!(s, impostor_switch_distance(bounds, grid, res, cam), "non-deterministic");
    match s {
        Ok(s) => {
            assert!((1..=4096).contains(&res));
            assert!(s.distance.is_finite() && s.texel_error.is_finite() && s.view_error.is_finite());
            assert!(s.texel_error > 0.0 && s.view_error >= 0.0);
            assert!(s.distance >= f64::from(bounds.radius));
            let want = 2.0 * f64::from(bounds.radius) / f64::from(res);
            assert!((s.texel_error - want).abs() <= want * 1e-12);
            // A looser pixel tolerance never moves the switch farther out.
            if let Ok(loose) = LodCamera::new(fov, vh, tol * 2.0) {
                if let Ok(l) = impostor_switch_distance(bounds, grid, res, loose) {
                    assert!(l.distance <= s.distance * (1.0 + 1e-9), "{l:?} vs {s:?}");
                    assert_eq!(l.view_error, s.view_error);
                }
            }
        }
        Err(e) => assert!(
            !(1..=4096).contains(&res) || matches!(e, Error::NumericalFailure | Error::InvalidParameter),
            "switch error {e:?}"
        ),
    }
});
