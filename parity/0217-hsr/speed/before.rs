//! Sampled opaque-surface visibility. An extension, not in C++ meshoptimizer.
use crate::budget::Budget;
use crate::sanitize::sort_work;
use crate::workspace::Work;
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

type Point = [f64; 3];

/// Views for hidden-surface removal. Not in C++ meshoptimizer.
/// Surfaces occlude from both sides; winding does not affect visibility.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum HiddenViews<'a> {
    /// 26 orthographic directions: every nonzero vector in {-1,0,1}³,
    /// normalized, in lexicographic order. Rays extend to infinity.
    Outside,
    /// Perspective views at each supplied finite point. An empty list is invalid.
    Points(&'a [[f32; 3]]),
    /// Perspective views at the eight corners and center of this finite box.
    /// Each minimum component must be at most the corresponding maximum.
    Bounds {
        /// Inclusive minimum corner.
        min: [f32; 3],
        /// Inclusive maximum corner.
        max: [f32; 3],
    },
}

/// Sampling controls. Not in C++ meshoptimizer. Always available, including no_std.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct HiddenOptions<'a> {
    /// Requested views; defaults to [`HiddenViews::Outside`].
    pub views: HiddenViews<'a>,
    /// Barycentric lattice denominator, 1..=64 (default 2). All lattice
    /// points, including vertices and edges, and the centroid are tested.
    /// There are (s+1)(s+2)/2 + 1 samples, including a repeated centroid
    /// when s is divisible by three.
    pub subdivisions: u32,
    /// World-space distance ignored next to each sample and viewpoint.
    /// Must be finite and nonnegative; default zero. A numerical guard of
    /// 64*f64::EPSILON times the largest absolute referenced coordinate
    /// (at least one) is added. Larger values can only retain more faces.
    pub tolerance: f64,
}
impl Default for HiddenOptions<'_> {
    fn default() -> Self {
        Self {
            views: HiddenViews::Outside,
            subdivisions: 2,
            tolerance: 0.0,
        }
    }
}

/// Counts by disjoint removal reason. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct HiddenReport {
    /// Number of input triangles.
    pub input_triangles: usize,
    /// Number of retained triangles.
    pub output_triangles: usize,
    /// Later exact position duplicates with the same cyclic winding.
    pub duplicate: usize,
    /// Occluded representatives with an exact opposite-wound counterpart.
    /// Both orientations are tested; exposed back-to-back walls are retained.
    pub enclosed_back_to_back: usize,
    /// Other triangles with every requested sample occluded in every view.
    pub occluded: usize,
}
impl HiddenReport {
    /// Total triangles removed.
    pub const fn removed(&self) -> usize {
        self.duplicate + self.enclosed_back_to_back + self.occluded
    }
    /// Written index count.
    pub const fn index_count(&self) -> usize {
        self.output_triangles * 3
    }
}

/// Filtered mesh in original triangle order. Not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct VisibleMesh {
    /// Original indices of retained triangles; positions are unchanged.
    pub indices: Vec<u32>,
    /// Disjoint removal counts.
    pub report: HiddenReport,
}

/// Remove hidden triangles using deterministic BVH ray casts.
/// An extension, not in C++ meshoptimizer.
///
/// A triangle is retained if **any** lattice sample or its centroid has an
/// unobstructed segment to **any** requested view. Occluders are the original
/// opaque, two-sided triangles, including triangles subsequently removed.
/// Intersections closer than the tolerance plus numerical guard are ignored;
/// coplanar contacts never occlude each other. Exact same-winding duplicates
/// retain the earliest copy (signed zeros compare equal), independently of
/// sampling. Degenerate triangles are retained, except for exact duplicates.
///
/// The conservative guarantee is at the stated sampling density: a triangle
/// with a visible tested sample is never removed. Finite samples do **not**
/// prove continuous invisibility. Slivers, openings between views/samples,
/// transparency, animation and newly exposed geometry after filtering are not
/// covered. This is not a watertightness, topology or exact-predicate API.
/// For resting surfaces, coplanar interfaces remain visible from below;
/// restrict viewpoints to the application's accessible region when appropriate.
///
/// BVH construction uses balanced median splits; query cost is data-dependent,
/// worst-case quadratic for heavily overlapping geometry. Every node and leaf
/// test is metered. All allocations are fallible and charged to the workspace.
///
/// ```
/// use meshoptimizer_rs::{remove_hidden_triangles, HiddenOptions, Positions, Workspace};
/// let p = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.]];
/// let mesh = remove_hidden_triangles(&[0,1,2, 1,2,0], Positions::from_packed(&p),
///     HiddenOptions::default(), &mut Workspace::default())?;
/// assert_eq!(mesh.indices, [0,1,2]);
/// assert_eq!(mesh.report.duplicate, 1);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// `InvalidTopology` for an incomplete triangle, `IndexOutOfBounds` for an
/// invalid index, `NumericalFailure` for a non-finite referenced position,
/// `InvalidParameter` for invalid options/views, and the usual `SizeOverflow`,
/// `LimitExceeded` or `AllocationFailed`. No partial result is returned.
pub fn remove_hidden_triangles(
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<VisibleMesh, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (keep, report) = plan(indices, positions, options, &mut budget, &mut work)?;
        work.add(indices.len())?;
        let mut out = budget.reserved(report.index_count())?;
        for (t, &k) in indices.as_chunks::<3>().0.iter().zip(&keep) {
            if k {
                out.extend_from_slice(t);
            }
        }
        Ok(VisibleMesh {
            indices: out,
            report,
        })
    })();
    workspace.finish(&work);
    result
}

/// [`remove_hidden_triangles`] into caller storage. Not in C++ meshoptimizer.
/// At most `indices.len()` entries are written. Every error leaves the entire
/// destination unchanged; success writes [`HiddenReport::index_count`] entries
/// and preserves the tail. Options, sampling and errors are identical to the
/// owned form, with `BufferTooSmall` when the retained indices do not fit.
pub fn remove_hidden_triangles_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<HiddenReport, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (keep, report) = plan(indices, positions, options, &mut budget, &mut work)?;
        if destination.len() < report.index_count() {
            return Err(Error::BufferTooSmall);
        }
        work.add(indices.len())?;
        let mut cursor = 0;
        for (t, &k) in indices.as_chunks::<3>().0.iter().zip(&keep) {
            if k {
                destination[cursor..cursor + 3].copy_from_slice(t);
                cursor += 3;
            }
        }
        Ok(report)
    })();
    workspace.finish(&work);
    result
}

fn sub(a: Point, b: Point) -> Point {
    core::array::from_fn(|k| a[k] - b[k])
}
fn dot(a: Point, b: Point) -> f64 {
    (a[0] * b[0] + a[1] * b[1]) + a[2] * b[2]
}
fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn point(p: [f32; 3]) -> Point {
    p.map(f64::from)
}
fn finite(p: [f32; 3]) -> bool {
    p.iter().all(|x| x.is_finite())
}

#[derive(Clone, Copy)]
struct Triangle {
    p: [Point; 3],
    id: usize,
}
#[derive(Clone, Copy)]
struct Face {
    key: [[u32; 3]; 3],
    reverse: bool,
    id: usize,
}
#[derive(Clone, Copy)]
struct Node {
    min: Point,
    max: Point,
    start: usize,
    count: usize,
    end: usize,
}

fn views(options: HiddenOptions<'_>, work: &mut Work) -> Result<usize, Error> {
    if !(1..=64).contains(&options.subdivisions)
        || !options.tolerance.is_finite()
        || options.tolerance < 0.0
    {
        return Err(Error::InvalidParameter);
    }
    match options.views {
        HiddenViews::Outside => Ok(26),
        HiddenViews::Points(p) => {
            if p.is_empty() {
                return Err(Error::InvalidParameter);
            }
            work.add(p.len())?;
            if !p.iter().copied().all(finite) {
                return Err(Error::InvalidParameter);
            }
            Ok(p.len())
        }
        HiddenViews::Bounds { min, max } => {
            if !finite(min) || !finite(max) || (0..3).any(|k| min[k] > max[k]) {
                return Err(Error::InvalidParameter);
            }
            Ok(9)
        }
    }
}
fn direction(views: HiddenViews<'_>, view: usize, sample: Point) -> (Point, f64) {
    let (d, outside) = match views {
        HiddenViews::Outside => {
            let code = if view < 13 { view } else { view + 1 };
            let component = |v: usize| [-1.0, 0.0, 1.0][v % 3];
            (
                [component(code / 9), component(code / 3), component(code)],
                true,
            )
        }
        HiddenViews::Points(p) => (sub(point(p[view]), sample), false),
        HiddenViews::Bounds { min, max } => {
            let p = core::array::from_fn(|k| {
                if view == 8 {
                    (f64::from(min[k]) + f64::from(max[k])) * 0.5
                } else if view & (1 << k) == 0 {
                    f64::from(min[k])
                } else {
                    f64::from(max[k])
                }
            });
            (sub(p, sample), false)
        }
    };
    let length = libm::sqrt(dot(d, d));
    if length == 0.0 {
        return ([0.; 3], 0.0);
    }
    (
        d.map(|x| x / length),
        if outside { f64::INFINITY } else { length },
    )
}

fn plan(
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(Vec<bool>, HiddenReport), Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    let view_count = views(options, work)?;
    let n = indices.len() / 3;
    work.add(n)?;
    let mut triangles = budget.reserved::<Triangle>(n)?;
    let mut scale = 1.0_f64;
    for (id, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        let mut p = [[0.; 3]; 3];
        for k in 0..3 {
            let vertex = usize::try_from(t[k]).map_err(|_| Error::SizeOverflow)?;
            let v = positions.get(vertex).ok_or(Error::IndexOutOfBounds)?;
            if !finite(v) {
                return Err(Error::NumericalFailure);
            }
            p[k] = point(v);
            for x in p[k] {
                scale = scale.max(x.abs());
            }
        }
        triangles.push(Triangle { p, id });
    }
    work.add(n.checked_mul(3).ok_or(Error::SizeOverflow)?)?;
    work.add(sort_work(n)?)?;
    let mut keep = budget.filled(n, true)?;
    let mut paired = budget.filled(n, false)?;
    let mut faces = budget.reserved::<Face>(n)?;
    for t in &triangles {
        let p =
            t.p.map(|v| v.map(|x| if x == 0.0 { 0 } else { (x as f32).to_bits() }));
        let rotate = |p: [[u32; 3]; 3]| p.min([p[1], p[2], p[0]]).min([p[2], p[0], p[1]]);
        let a = rotate(p);
        let b = rotate([p[0], p[2], p[1]]);
        faces.push(Face {
            key: a.min(b),
            reverse: b < a,
            id: t.id,
        });
    }
    faces.sort_unstable_by_key(|f| (f.key, f.reverse, f.id));
    let mut report = HiddenReport {
        input_triangles: n,
        ..HiddenReport::default()
    };
    work.add(n)?;
    let mut first: Option<Face> = None;
    // Two orientation representatives per position class, found in one scan.
    let mut opposite: Option<Face> = None;
    for f in &faces {
        if let Some(a) = first.filter(|a| a.key == f.key) {
            if a.reverse == f.reverse {
                keep[f.id] = false;
                report.duplicate += 1;
            } else if let Some(b) = opposite {
                keep[f.id] = false;
                report.duplicate += 1;
                paired[b.id] = true;
            } else {
                paired[a.id] = true;
                paired[f.id] = true;
                opposite = Some(*f);
            }
        } else {
            first = Some(*f);
            opposite = None;
        }
    }
    budget.release(faces);
    let cap = n.checked_mul(2).ok_or(Error::SizeOverflow)?;
    work.add(cap)?;
    let mut nodes = budget.reserved::<Node>(cap)?;
    if n != 0 {
        build(&mut triangles, 0, &mut nodes, work)?;
    }
    let epsilon = options.tolerance + 64.0 * f64::EPSILON * scale;
    let s = options.subdivisions;
    for t in &triangles {
        if !keep[t.id] {
            continue;
        }
        let normal = cross(sub(t.p[1], t.p[0]), sub(t.p[2], t.p[0]));
        if dot(normal, normal) == 0.0 {
            continue;
        }
        let mut visible = false;
        'samples: for row in 0..=s + 1 {
            let width = if row == s + 1 { 0 } else { s - row };
            for col in 0..=width {
                work.add(1)?;
                let (u, v) = if row == s + 1 {
                    (1.0 / 3.0, 1.0 / 3.0)
                } else {
                    (f64::from(row) / f64::from(s), f64::from(col) / f64::from(s))
                };
                let sample = core::array::from_fn(|k| {
                    t.p[0][k] * (1.0 - u - v) + t.p[1][k] * u + t.p[2][k] * v
                });
                for view in 0..view_count {
                    work.add(1)?;
                    let (d, distance) = direction(options.views, view, sample);
                    if !blocked(
                        Ray {
                            origin: sample,
                            direction: d,
                            distance,
                            epsilon,
                        },
                        t.id,
                        &triangles,
                        &nodes,
                        work,
                    )? {
                        visible = true;
                        break 'samples;
                    }
                }
            }
        }
        if !visible {
            keep[t.id] = false;
            if paired[t.id] {
                report.enclosed_back_to_back += 1;
            } else {
                report.occluded += 1;
            }
        }
    }
    report.output_triangles = n - report.removed();
    budget.release(paired);
    budget.release(triangles);
    budget.release(nodes);
    Ok((keep, report))
}

fn build(
    triangles: &mut [Triangle],
    start: usize,
    nodes: &mut Vec<Node>,
    work: &mut Work,
) -> Result<(), Error> {
    work.add(triangles.len())?;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for t in triangles.iter() {
        for p in t.p {
            for k in 0..3 {
                min[k] = min[k].min(p[k]);
                max[k] = max[k].max(p[k]);
            }
        }
    }
    let at = nodes.len();
    nodes.push(Node {
        min,
        max,
        start,
        count: triangles.len(),
        end: 0,
    });
    if triangles.len() > 4 {
        let mut axis = 0;
        for k in 1..3 {
            if max[k] - min[k] > max[axis] - min[axis] {
                axis = k;
            }
        }
        work.add(sort_work(triangles.len())?)?;
        let middle = triangles.len() / 2;
        triangles.select_nth_unstable_by(middle, |a, b| {
            let centroid = |t: &Triangle| (t.p[0][axis] + t.p[1][axis]) + t.p[2][axis];
            centroid(a).total_cmp(&centroid(b)).then(a.id.cmp(&b.id))
        });
        let (a, b) = triangles.split_at_mut(middle);
        build(a, start, nodes, work)?;
        build(b, start + middle, nodes, work)?;
        nodes[at].count = 0;
    }
    nodes[at].end = nodes.len();
    Ok(())
}
fn box_hit(node: &Node, origin: Point, d: Point, max_t: f64, epsilon: f64) -> bool {
    let mut lo = epsilon;
    let mut hi = max_t - epsilon;
    for k in 0..3 {
        if d[k] == 0.0 {
            if origin[k] < node.min[k] - epsilon || origin[k] > node.max[k] + epsilon {
                return false;
            }
        } else {
            let a = (node.min[k] - epsilon - origin[k]) / d[k];
            let b = (node.max[k] + epsilon - origin[k]) / d[k];
            lo = lo.max(a.min(b));
            hi = hi.min(a.max(b));
            if lo > hi {
                return false;
            }
        }
    }
    true
}
// Inclusive barycentrics are essential at shared edges. Parallel rays and
// hits outside the triangle are rejected. Predicates use ordinary f64.
fn hit(t: &Triangle, o: Point, d: Point, max: f64, epsilon: f64) -> bool {
    let e1 = sub(t.p[1], t.p[0]);
    let e2 = sub(t.p[2], t.p[0]);
    let h = cross(d, e2);
    let determinant = dot(e1, h);
    if determinant == 0.0 {
        return false;
    }
    let q = sub(o, t.p[0]);
    let u = dot(q, h) / determinant;
    let r = cross(q, e1);
    let v = dot(d, r) / determinant;
    let distance = dot(e2, r) / determinant;
    u >= 0.0 && v >= 0.0 && u + v <= 1.0 && distance > epsilon && distance < max - epsilon
}
struct Ray {
    origin: Point,
    direction: Point,
    distance: f64,
    epsilon: f64,
}
fn blocked(
    ray: Ray,
    id: usize,
    triangles: &[Triangle],
    nodes: &[Node],
    work: &mut Work,
) -> Result<bool, Error> {
    let Ray {
        origin: o,
        direction: d,
        distance: max,
        epsilon,
    } = ray;
    let mut at = 0;
    while at < nodes.len() {
        work.add(1)?;
        let node = &nodes[at];
        if !box_hit(node, o, d, max, epsilon) {
            at = node.end;
            continue;
        }
        for t in &triangles[node.start..node.start + node.count] {
            work.add(1)?;
            if t.id != id && hit(t, o, d, max, epsilon) {
                return Ok(true);
            }
        }
        at += 1;
    }
    Ok(false)
}
