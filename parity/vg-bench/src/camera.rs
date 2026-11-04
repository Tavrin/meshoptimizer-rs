//! Orbit views and the screen-space error rule shared by both cut selectors.
//!
//! The rule is the one in the reference builder's culling shader: a bounding
//! sphere with an error is imperceptible when
//! `error / max(distance - radius, near) * proj11 / 2 * viewport_height < threshold_px`.

/// Vertical field of view, 60 degrees.
pub const FOV_Y: f64 = std::f64::consts::FRAC_PI_3;
/// Viewport height in pixels. Chosen so that a mesh spanning 2048 px still
/// leaves the camera outside its bounding sphere.
pub const VIEWPORT_H: f64 = 2048.0;
/// Viewport aspect ratio.
pub const ASPECT: f64 = 16.0 / 9.0;
/// Near plane distance in scene units.
pub const NEAR: f64 = 0.1;

/// A pinhole camera looking at the origin.
#[derive(Clone, Copy)]
pub struct View {
    pub eye: [f64; 3],
    right: [f64; 3],
    up: [f64; 3],
    forward: [f64; 3],
}

fn norm(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / l, v[1] / l, v[2] / l]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl View {
    /// Camera at `dir * distance` looking at the origin.
    pub fn orbit(dir: [f64; 3], distance: f64) -> Self {
        let d = norm(dir);
        let forward = [-d[0], -d[1], -d[2]];
        let helper = if d[1].abs() > 0.99 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let right = norm(cross(forward, helper));
        let up = cross(right, forward);
        Self { eye: [d[0] * distance, d[1] * distance, d[2] * distance], right, up, forward }
    }

    /// Distance from the eye to a point.
    pub fn distance(&self, c: [f32; 3]) -> f64 {
        let v = [c[0] as f64 - self.eye[0], c[1] as f64 - self.eye[1], c[2] as f64 - self.eye[2]];
        dot(v, v).sqrt()
    }

    /// Projected error in pixels for a sphere and error, under the reference rule.
    pub fn error_px(&self, center: [f32; 3], radius: f32, error: f32) -> f64 {
        let d = self.distance(center) - radius as f64;
        error as f64 / d.max(NEAR) * proj11() * 0.5 * VIEWPORT_H
    }

    /// Sphere against the four side planes and the near plane.
    pub fn sphere_visible(&self, c: [f32; 3], r: f32) -> bool {
        let v = [c[0] as f64 - self.eye[0], c[1] as f64 - self.eye[1], c[2] as f64 - self.eye[2]];
        let (x, y, z) = (dot(v, self.right), dot(v, self.up), dot(v, self.forward));
        let th = (FOV_Y * 0.5).tan();
        let tw = th * ASPECT;
        let r = r as f64;
        let side_x = (tw * tw + 1.0).sqrt();
        let side_y = (th * th + 1.0).sqrt();
        z + r >= NEAR && x.abs() - tw * z <= r * side_x && y.abs() - th * z <= r * side_y
    }

    /// Axis-aligned box against the same planes (conservative corner test).
    pub fn aabb_visible(&self, center: [f32; 3], half: [f32; 3]) -> bool {
        let th = (FOV_Y * 0.5).tan();
        let tw = th * ASPECT;
        let planes = [
            sub_scale(self.right, self.forward, tw, 1.0),
            sub_scale(self.right, self.forward, -tw, -1.0),
            sub_scale(self.up, self.forward, th, 1.0),
            sub_scale(self.up, self.forward, -th, -1.0),
            self.forward,
        ];
        let c = [center[0] as f64 - self.eye[0], center[1] as f64 - self.eye[1], center[2] as f64 - self.eye[2]];
        for (i, p) in planes.iter().enumerate() {
            // Inside means dot(p, point) >= 0 for the side planes, >= near for the last.
            let reach = half[0] as f64 * p[0].abs() + half[1] as f64 * p[1].abs() + half[2] as f64 * p[2].abs();
            let limit = if i == 4 { NEAR } else { 0.0 };
            if dot(*p, c) + reach < limit {
                return false;
            }
        }
        true
    }
}

/// Inward normal of a side plane: `s * axis - t * forward` flipped to face the view volume.
fn sub_scale(axis: [f64; 3], fwd: [f64; 3], t: f64, s: f64) -> [f64; 3] {
    // The plane through the eye containing `axis` direction scaled by tan: n = -s*axis + |t|*forward.
    let k = t.abs();
    [-s * axis[0] + k * fwd[0], -s * axis[1] + k * fwd[1], -s * axis[2] + k * fwd[2]]
}

/// `1 / tan(fov / 2)`.
pub fn proj11() -> f64 {
    1.0 / (FOV_Y * 0.5).tan()
}

/// Pixels per scene unit at depth `d` for a transverse displacement.
pub fn px_per_unit(d: f64) -> f64 {
    proj11() * 0.5 * VIEWPORT_H / d.max(NEAR)
}

/// `n` directions spread over the sphere by a Fibonacci spiral.
pub fn directions(n: usize) -> Vec<[f64; 3]> {
    let golden = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let y = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
            let r = (1.0 - y * y).max(0.0).sqrt();
            let a = golden * i as f64;
            [r * a.cos(), y, r * a.sin()]
        })
        .collect()
}

/// Camera distance at which a sphere of `radius` spans `span_px` vertically.
pub fn distance_for_span(radius: f64, span_px: f64) -> f64 {
    radius * VIEWPORT_H * proj11() / span_px
}
