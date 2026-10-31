//! Conservative opaque-surface visibility. An extension, not in C++ meshoptimizer.
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
    /// normalized, in lexicographic order. Views extend to infinity.
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
    /// Barycentric denominator for finite views and the outside compatibility
    /// guard, 1..=64 (default 2). Raster coverage is independent of this value. All lattice
    /// points, including vertices and edges, and the centroid are tested.
    /// There are (s+1)(s+2)/2 + 1 samples, including a repeated centroid
    /// when s is divisible by three.
    pub subdivisions: u32,
    /// World-space depth tolerance (outside), or distance ignored next to
    /// each sample and viewpoint (finite views).
    /// Must be finite and nonnegative; default zero. A numerical guard of
    /// 64*f64::EPSILON times the largest absolute referenced coordinate
    /// (at least one) is added. Outside views add a relative depth guard of
    /// 1e-12 times the longest mesh side. Larger values only retain more faces.
    pub tolerance: f64,
    /// Outside-view pixel width in world units. Zero (default) selects
    /// longest referenced bounding-box side / 128. Positive values select
    /// ceil(2*side / pixel_size) pixels, at least one and at most 2048;
    /// requests above 2048 are invalid. Actual pixel width is 2*side / resolution.
    /// Coarser pixels may retain more uncertain geometry. Ignored for finite views.
    pub pixel_size: f64,
}
impl Default for HiddenOptions<'_> {
    fn default() -> Self {
        Self {
            views: HiddenViews::Outside,
            subdivisions: 2,
            tolerance: 0.0,
            pixel_size: 0.0,
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
pub struct VisibleMeshGeneric<I: crate::Index = u32> {
    /// Original indices of retained triangles; positions are unchanged.
    pub indices: Vec<I>,
    /// Disjoint removal counts.
    pub report: HiddenReport,
}

/// Remove hidden triangles using deterministic software item buffers (outside
/// views) or BVH ray casts (finite viewpoints). Not in C++ meshoptimizer.
///
/// Outside views use 26 orthographic directions and an 8x8 tiled depth/item
/// buffer. Fixed-point edges certify only fully covered pixels; bounded unions
/// sharing original edges may jointly cover a pixel. Depth bounds cover each cell.
/// Outward triangle rectangles query those bounds. An uncertain pixel retains
/// the triangle, including subpixel and edge-on geometry. At the requested
/// pixel size, every triangle visible over at least one pixel-area in any tested
/// direction is kept; in fact any visible projected portion is retained under
/// these coverage bounds. The existing lattice-sample guarantee also holds.
/// Raster-hidden candidates additionally run the original BVH sample checks:
/// floating-point shared-edge misses can restore faces, never remove more.
/// Exact same-winding duplicates retain only their earliest copy, independently
/// of visibility. Signed zeros match. Degenerate triangles otherwise stay.
///
/// Finite views retain a triangle if any barycentric lattice sample or centroid
/// has an unobstructed segment to any camera. Near contacts (within tolerance
/// plus numerical guard), coplanar contacts and viewpoint endpoints do not
/// occlude. Occluders are always the original opaque two-sided mesh.
///
/// Views between tested directions, transparency, motion, topology and newly
/// exposed geometry after filtering are outside this contract. Predicates use
/// ordinary f64 on finite f32 input, not adaptive exact arithmetic. Resting
/// surfaces may remain visible from below: restrict finite viewpoints to the
/// accessible region when appropriate.
///
/// Raster work is O(n log n + views * (n + rasterized rectangle area)), with
/// resolution bounded by 2048. Compatibility and finite-view BVH queries are
/// data-dependent and worst-case quadratic. Storage and work are fallibly charged to Workspace.
/// The implementation is serial even with `parallel`, giving identical results
/// in every thread pool; no extra feature is needed to use this API.
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
#[inline]
pub fn remove_hidden_triangles_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<VisibleMeshGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        remove_hidden_triangles_u32(indices, positions, options, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn remove_hidden_triangles_u32(
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<VisibleMeshGeneric, Error> {
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
        Ok(VisibleMeshGeneric {
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
#[inline]
pub fn remove_hidden_triangles_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<HiddenReport, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            remove_hidden_triangles_into_u32(destination, indices, positions, options, workspace)
        })
    })
}

fn remove_hidden_triangles_into_u32(
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
        || !options.pixel_size.is_finite()
        || options.pixel_size < 0.0
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
        // Coordinates originated as finite f32; this narrowing is exact.
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
    let epsilon = options.tolerance + 64.0 * f64::EPSILON * scale;
    if matches!(options.views, HiddenViews::Outside) {
        outside_visibility(
            &mut triangles,
            &mut keep,
            &paired,
            options,
            epsilon,
            budget,
            work,
        )?;
        work.add(n.checked_mul(2).ok_or(Error::SizeOverflow)?)?;
        for t in &triangles {
            if keep[t.id] {
                continue;
            }
            // Duplicates were counted before visibility; classify only representatives.
            if paired[t.id] {
                report.enclosed_back_to_back += 1;
            }
        }
        report.occluded =
            keep.iter().filter(|&&k| !k).count() - report.duplicate - report.enclosed_back_to_back;
    } else {
        finite_visibility(
            &mut triangles,
            &mut keep,
            &paired,
            &mut report,
            options,
            view_count,
            epsilon,
            budget,
            work,
        )?;
    }
    report.output_triangles = n - report.removed();
    budget.release(paired);
    budget.release(triangles);
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
#[derive(Clone, Copy)]
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
    witness: &mut [Option<(usize, usize)>; 4],
    work: &mut Work,
) -> Result<bool, Error> {
    let Ray {
        origin: o,
        direction: d,
        distance: max,
        epsilon,
    } = ray;
    for slot in 0..witness.len() {
        let Some((triangle, leaf)) = witness[slot] else {
            continue;
        };
        work.add(2)?;
        let t = &triangles[triangle];
        // The same hit and box predicates as the traversal below. Every
        // ancestor encloses this leaf: monotone floating-point subtraction
        // and division make its slab interval a superset of the leaf's.
        // Thus a successful leaf witness proves the original traversal would
        // return true too, including shared-edge rounding behavior.
        if t.id != id && hit(t, o, d, max, epsilon) && box_hit(&nodes[leaf], o, d, max, epsilon) {
            witness.swap(0, slot);
            return Ok(true);
        }
    }
    let mut at = 0;
    while at < nodes.len() {
        work.add(1)?;
        let node = &nodes[at];
        if !box_hit(node, o, d, max, epsilon) {
            at = node.end;
            continue;
        }
        for (offset, t) in triangles[node.start..node.start + node.count]
            .iter()
            .enumerate()
        {
            work.add(1)?;
            if t.id != id && hit(t, o, d, max, epsilon) {
                witness.rotate_right(1);
                witness[0] = Some((node.start + offset, at));
                return Ok(true);
            }
        }
        at += 1;
    }
    Ok(false)
}

#[allow(clippy::too_many_arguments)]
fn finite_visibility(
    triangles: &mut [Triangle],
    keep: &mut [bool],
    paired: &[bool],
    report: &mut HiddenReport,
    options: HiddenOptions<'_>,
    view_count: usize,
    epsilon: f64,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    let n = triangles.len();
    let cap = n.checked_mul(2).ok_or(Error::SizeOverflow)?;
    work.add(cap)?;
    let mut nodes = budget.reserved::<Node>(cap)?;
    if n != 0 {
        build(triangles, 0, &mut nodes, work)?;
    }
    let s = options.subdivisions;
    // Four nearby blockers avoid alternating BVH searches around wall seams.
    let mut witnesses = budget.filled(view_count, [None; 4])?;
    work.add(26)?;
    // Outside directions do not depend on the sample. Preserve the original
    // normalization and reuse it rather than repeating sqrt at every sample.
    let outside =
        core::array::from_fn::<_, 26, _>(|view| direction(HiddenViews::Outside, view, [0.; 3]));
    for t in triangles.iter() {
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
                for (view, witness) in witnesses.iter_mut().enumerate() {
                    work.add(1)?;
                    let (d, distance) = if matches!(options.views, HiddenViews::Outside) {
                        outside[view]
                    } else {
                        direction(options.views, view, sample)
                    };
                    if !blocked(
                        Ray {
                            origin: sample,
                            direction: d,
                            distance,
                            epsilon,
                        },
                        t.id,
                        triangles,
                        &nodes,
                        witness,
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
    budget.release(witnesses);
    budget.release(nodes);
    Ok(())
}

// Fixed-point coverage is deliberately inward for occluders. A pixel can
// certify occlusion only when a triangle (or a bounded adjacent union) covers its
// entire square. Queries use outward bounding rectangles and maximum depth.
// Thus quantization, subpixel slivers and incomplete cells can only retain.
const SUBPIXEL: i64 = 256;
const TILE: usize = 8;

#[derive(Clone, Copy)]
struct RasterGrid {
    size: usize,
    cells: usize,
    margin: i64,
}
impl RasterGrid {
    fn new(size: usize) -> Result<Self, Error> {
        if !(1..=2048).contains(&size) {
            return Err(Error::InvalidParameter);
        }
        let margin = i64::try_from(size)
            .ok()
            .and_then(|n| n.checked_mul(SUBPIXEL))
            .and_then(|n| n.checked_mul(8))
            .and_then(|n| n.checked_add(16))
            .ok_or(Error::SizeOverflow)?;
        let cells = size
            .div_ceil(TILE)
            .checked_mul(size.div_ceil(TILE))
            .and_then(|n| n.checked_mul(TILE * TILE))
            .ok_or(Error::SizeOverflow)?;
        Ok(Self {
            size,
            cells,
            margin,
        })
    }
    fn subpixel(self, cell: usize) -> i64 {
        // Only raster loop coordinates call this: cell < size <= 2048.
        // The cast and product are exact, with result below 2^19.
        cell as i64 * SUBPIXEL
    }
}
#[derive(Clone, Copy)]
struct Projection {
    center: Point,
    u: Point,
    v: Point,
    d: Point,
    scale: f64,
    grid: RasterGrid,
}
#[derive(Clone, Copy)]
struct Projected {
    xy: [[i64; 2]; 3],
    lo: f64,
    hi: f64,
    plane: [f64; 3],
    depths: [f64; 3],
}
// Projected vertices and cell corners lie in [0, 2048*256]. Each
// difference is at most 2^19 and each determinant at most 2^39, so all
// subtraction, multiplication, sign adjustment and abs operations fit i64.
fn edge(a: [i64; 2], b: [i64; 2], p: [i64; 2]) -> i64 {
    (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
}
impl Projected {
    fn depth_bounds(&self, x: usize, y: usize) -> (f64, f64) {
        let [a, b, c] = self.plane;
        if !c.is_finite() {
            return self.clipped_depth(x, y);
        }
        let center = a * (x as f64 * 256.0 + 128.0) + b * (y as f64 * 256.0 + 128.0) + c;
        // 128 subpixels to the cell corner + 2 for vertex quantization.
        // At any true projected triangle point the affine quantized plane
        // differs from the true depth by <= |a|+|b| (barycentric argument).
        let radius = (a.abs() + b.abs()) * 130.0;
        (self.lo.max(center - radius), self.hi.min(center + radius))
    }
    fn clipped_depth(&self, x: usize, y: usize) -> (f64, f64) {
        // Edge-on triangles have no screen-space depth plane. Clip their
        // original depth-carrying polygon to an outward-expanded pixel;
        // global triangle depth would falsely expose enclosed edge-on faces.
        let mut polygon = [[0.; 3]; 12];
        for (k, p) in polygon[..3].iter_mut().enumerate() {
            *p = [self.xy[k][0] as f64, self.xy[k][1] as f64, self.depths[k]];
        }
        let mut count = 3;
        for (axis, bound, sign) in [
            (0, x as f64 * 256.0 - 2.0, 1.0),
            (0, (x + 1) as f64 * 256.0 + 2.0, -1.0),
            (1, y as f64 * 256.0 - 2.0, 1.0),
            (1, (y + 1) as f64 * 256.0 + 2.0, -1.0),
        ] {
            if count == 0 {
                break;
            }
            let mut out = [[0.; 3]; 12];
            let mut n = 0;
            let mut prev = polygon[count - 1];
            for &cur in &polygon[..count] {
                let a = (prev[axis] - bound) * sign >= 0.0;
                let b = (cur[axis] - bound) * sign >= 0.0;
                if a != b {
                    let t = (bound - prev[axis]) / (cur[axis] - prev[axis]);
                    out[n] = core::array::from_fn(|k| prev[k] + (cur[k] - prev[k]) * t);
                    n += 1;
                }
                if b {
                    out[n] = cur;
                    n += 1;
                }
                prev = cur;
            }
            polygon = out;
            count = n;
        }
        polygon[..count]
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                (lo.min(p[2]), hi.max(p[2]))
            })
    }
    fn overlaps(&self, x: usize, y: usize, grid: RasterGrid) -> bool {
        let sign = edge(self.xy[0], self.xy[1], self.xy[2]).signum();
        if sign == 0 {
            return true;
        }
        let x = grid.subpixel(x);
        let y = grid.subpixel(y);
        let margin = grid.margin;
        for k in 0..3 {
            let a = self.xy[k];
            let b = self.xy[(k + 1) % 3];
            let ex = b[0] - a[0];
            let ey = b[1] - a[1];
            let corner = [
                if -ey * sign > 0 { x + SUBPIXEL } else { x },
                if ex * sign > 0 { y + SUBPIXEL } else { y },
            ];
            if edge(a, b, corner) * sign < -margin {
                return false;
            }
        }
        true
    }
}
impl Projection {
    fn project(&self, t: &Triangle) -> Projected {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        let mut depths = [0.; 3];
        let mut vertex = 0;
        let xy = t.p.map(|p| {
            let p = sub(p, self.center);
            let z = dot(p, self.d);
            lo = lo.min(z);
            hi = hi.max(z);
            depths[vertex] = z;
            vertex += 1;
            [self.u, self.v].map(|axis| {
                let x = dot(p, axis) * self.scale + self.grid.size as f64 * 128.0;
                // Finite normalized coordinates and size <=2048 cannot wrap.
                libm::floor(x.clamp(0.0, self.grid.size as f64 * 256.0)) as i64
            })
        });
        let dx1 = (xy[1][0] - xy[0][0]) as f64;
        let dy1 = (xy[1][1] - xy[0][1]) as f64;
        let dx2 = (xy[2][0] - xy[0][0]) as f64;
        let dy2 = (xy[2][1] - xy[0][1]) as f64;
        let area = dx1 * dy2 - dy1 * dx2;
        let plane = if area == 0.0 {
            [0., 0., f64::NAN]
        } else {
            let a = ((depths[1] - depths[0]) * dy2 - (depths[2] - depths[0]) * dy1) / area;
            let b = (dx1 * (depths[2] - depths[0]) - dx2 * (depths[1] - depths[0])) / area;
            [a, b, depths[0] - a * xy[0][0] as f64 - b * xy[0][1] as f64]
        };
        Projected {
            xy,
            lo,
            hi,
            plane,
            depths,
        }
    }
    fn rectangle(&self, p: &Projected) -> [usize; 4] {
        let bounds = |axis: usize| {
            let [a, b, c] = p.xy.map(|p| p[axis]);
            let lo = a.min(b).min(c);
            let hi = a.max(b).max(c);
            // One subpixel covers floor quantization and projection roundoff.
            // project() clamps vertices to [0, size*256], size <= 2048.
            // Expanded nonnegative pixel coordinates are <= 2048 and fit usize.
            let lo = ((lo - 1).max(0) / SUBPIXEL) as usize;
            let hi = (((hi + 1) / SUBPIXEL) as usize).min(self.grid.size - 1);
            (lo, hi)
        };
        let (x0, x1) = bounds(0);
        let (y0, y1) = bounds(1);
        [x0, y0, x1, y1]
    }
    fn covered(&self, p: &Projected, cell: [usize; 2], skip: usize) -> bool {
        let sign = edge(p.xy[0], p.xy[1], p.xy[2]).signum();
        if sign == 0 {
            return false;
        }
        // Each rounded vertex differs from its real projection by <= 1
        // subpixel. The edge determinant error at a cell corner is less
        // than 8*grid_width + 16 (including f64 projection roundoff).
        let margin = self.grid.margin;
        let x = self.grid.subpixel(cell[0]);
        let y = self.grid.subpixel(cell[1]);
        for k in 0..3 {
            if k == skip {
                continue;
            }
            let a = p.xy[k];
            let b = p.xy[(k + 1) % 3];
            let ex = b[0] - a[0];
            let ey = b[1] - a[1];
            // Minimum affine edge value over the four square corners.
            let corner = [
                if -ey * sign < 0 { x + SUBPIXEL } else { x },
                if ex * sign < 0 { y + SUBPIXEL } else { y },
            ];
            if edge(a, b, corner) * sign <= margin {
                return false;
            }
        }
        true
    }
    fn union_covers(
        &self,
        triangles: &[Triangle],
        ids: &[usize],
        projected: &[Projected],
        cell: [usize; 2],
    ) -> bool {
        // At most eight triangles (24 edges): a fixed-size certificate,
        // never a scan over mesh triangles. Normalize all edges to CCW.
        let mut boundary = 0;
        for (i, &id) in ids.iter().enumerate() {
            let p = &projected[i];
            let sign = edge(p.xy[0], p.xy[1], p.xy[2]).signum();
            for k in 0..3 {
                let (a, b) = if sign > 0 {
                    (k, (k + 1) % 3)
                } else {
                    ((k + 1) % 3, k)
                };
                let mut opposite = 0;
                let mut same = 0;
                for (j, &other) in ids.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let q = &projected[j];
                    let reverse = edge(q.xy[0], q.xy[1], q.xy[2]) < 0;
                    for e in 0..3 {
                        let (c, d) = if reverse {
                            ((e + 1) % 3, e)
                        } else {
                            (e, (e + 1) % 3)
                        };
                        if triangles[id].p[a] == triangles[other].p[d]
                            && triangles[id].p[b] == triangles[other].p[c]
                        {
                            opposite += 1;
                        }
                        if triangles[id].p[a] == triangles[other].p[c]
                            && triangles[id].p[b] == triangles[other].p[d]
                        {
                            same += 1;
                        }
                    }
                }
                if opposite == 1 && same == 0 {
                    continue;
                }
                boundary += 1;
                if !self.edge_covers(p.xy[a], p.xy[b], cell) {
                    return false;
                }
            }
        }
        // A positive-oriented union whose every external edge strictly
        // contains the square covers it. Internal original edges cancel;
        // an internal hole or an incomplete fan fails an external edge.
        boundary >= 3
    }
    fn edge_covers(&self, a: [i64; 2], b: [i64; 2], cell: [usize; 2]) -> bool {
        let margin = self.grid.margin;
        let x = self.grid.subpixel(cell[0]);
        let y = self.grid.subpixel(cell[1]);
        let corner = [
            if b[1] > a[1] { x + SUBPIXEL } else { x },
            if b[0] < a[0] { y + SUBPIXEL } else { y },
        ];
        edge(a, b, corner) > margin
    }
    fn address(&self, x: usize, y: usize) -> usize {
        // 8x8 pixel tiles are contiguous, including padded boundary tiles.
        ((y / TILE) * self.grid.size.div_ceil(TILE) + x / TILE) * TILE * TILE
            + (y % TILE) * TILE
            + x % TILE
    }
}
fn outside_visibility(
    triangles: &mut [Triangle],
    keep: &mut [bool],
    paired: &[bool],
    options: HiddenOptions<'_>,
    epsilon: f64,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    if triangles.is_empty() {
        return Ok(());
    }
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
    let side = (0..3).map(|k| max[k] - min[k]).fold(0.0, f64::max);
    if side == 0.0 {
        return Ok(());
    }
    let resolution = if options.pixel_size == 0.0 {
        256.0
    } else {
        libm::ceil((2.0 * side) / options.pixel_size).max(1.0)
    };
    if resolution > 2048.0 {
        return Err(Error::InvalidParameter);
    }
    // Resolution is checked before narrowing and all storage arithmetic.
    let size = resolution as usize;
    let grid = RasterGrid::new(size)?;
    let cells = grid.cells;
    // Select possible cell occluders before allocating the raster. Projected
    // area never exceeds 3D area, so excluding these tiny faces can only retain.
    work.add(triangles.len())?;
    let mut occluders = budget.reserved::<usize>(triangles.len())?;
    let screen_scale = size as f64 * 128.0 / side;
    let threshold = (size as f64 * 256.0 * 8.0 + 16.0) / (screen_scale * screen_scale);
    work.add(triangles.len())?;
    for t in triangles.iter() {
        if !keep[t.id] {
            continue;
        }
        let normal = cross(sub(t.p[1], t.p[0]), sub(t.p[2], t.p[0]));
        if dot(normal, normal) > threshold * threshold {
            occluders.push(t.id);
        }
    }
    if occluders.is_empty() {
        budget.release(occluders);
        return Ok(());
    }
    work.add(
        cells
            .checked_mul(3)
            .and_then(|n| n.checked_add(keep.len()))
            .ok_or(Error::SizeOverflow)?,
    )?;
    let mut depth = budget.filled(cells, f64::NEG_INFINITY)?;
    let mut owner = budget.filled(cells, usize::MAX)?;
    let mut partial = budget.filled(cells, [usize::MAX; 8])?;
    let mut visible = budget.filled(keep.len(), false)?;
    let mut remaining = 0;
    work.add(triangles.len())?;
    for t in triangles.iter() {
        if !keep[t.id] {
            continue;
        }
        let normal = cross(sub(t.p[1], t.p[0]), sub(t.p[2], t.p[0]));
        if dot(normal, normal) == 0.0 {
            visible[t.id] = true;
        } else {
            remaining += 1;
        }
    }
    let guard = epsilon + side * 1e-12;
    for view in 0..26 {
        if remaining == 0 {
            break;
        }
        work.add(cells.checked_mul(3).ok_or(Error::SizeOverflow)?)?;
        depth.fill(f64::NEG_INFINITY);
        owner.fill(usize::MAX);
        partial.fill([usize::MAX; 8]);
        let (d, _) = direction(HiddenViews::Outside, view, [0.; 3]);
        let axis = if d[0].abs() < 0.9 {
            [1., 0., 0.]
        } else {
            [0., 1., 0.]
        };
        let u = cross(d, axis);
        let length = libm::sqrt(dot(u, u));
        let u = u.map(|x| x / length);
        let projection = Projection {
            center: core::array::from_fn(|k| (min[k] + max[k]) * 0.5),
            u,
            v: cross(d, u),
            d,
            scale: (size as f64 * 128.0) / side,
            grid,
        };
        work.add(occluders.len())?;
        for &id in &occluders {
            let t = &triangles[id];
            let p = projection.project(t);
            let [x0, y0, x1, y1] = projection.rectangle(&p);
            // Charge the complete rectangle before touching any of its cells.
            work.add(
                (x1 - x0 + 1)
                    .checked_mul(y1 - y0 + 1)
                    .ok_or(Error::SizeOverflow)?,
            )?;
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let at = projection.address(x, y);
                    if p.hi <= depth[at] {
                        continue;
                    }
                    let mut z = p.depth_bounds(x, y).0;
                    let mut covered = projection.covered(&p, [x, y], 3);
                    // Small triangles cannot individually cover a pixel. Keeping
                    // them out of bounded union certificates only retains more.
                    let area = edge(p.xy[0], p.xy[1], p.xy[2]).abs();
                    if !covered
                        && p.overlaps(x, y, grid)
                        && area > (SUBPIXEL * SUBPIXEL / 8).max(grid.margin)
                    {
                        work.add(8 * 8 * 8 * 9 + 32)?;
                        let ids = &mut partial[at];
                        let mut ps = [p; 8];
                        let mut count = 0;
                        for &id in ids.iter() {
                            if id == usize::MAX {
                                break;
                            }
                            ps[count] = projection.project(&triangles[id]);
                            count += 1;
                        }
                        let upper = p.depth_bounds(x, y).1;
                        let mut insert = count;
                        for (j, q) in ps[..count].iter().enumerate() {
                            if upper > q.depth_bounds(x, y).1 {
                                insert = j;
                                break;
                            }
                        }
                        if insert < 8 {
                            count = (count + 1).min(8);
                            for j in (insert + 1..count).rev() {
                                ids[j] = ids[j - 1];
                                ps[j] = ps[j - 1];
                            }
                            ids[insert] = t.id;
                            ps[insert] = p;
                        }
                        z = f64::INFINITY;
                        for end in 1..=count {
                            z = z.min(ps[end - 1].depth_bounds(x, y).0);
                            if z <= depth[at] {
                                break;
                            }
                            if projection.union_covers(triangles, &ids[..end], &ps[..end], [x, y]) {
                                covered = true;
                                break;
                            }
                        }
                    }
                    if covered && z > depth[at] {
                        depth[at] = z;
                        owner[at] = t.id;
                    }
                }
            }
        }
        work.add(triangles.len())?;
        for t in triangles.iter() {
            if !keep[t.id] || visible[t.id] {
                continue;
            }
            let p = projection.project(t);
            let [x0, y0, x1, y1] = projection.rectangle(&p);
            work.add(
                (x1 - x0 + 1)
                    .checked_mul(y1 - y0 + 1)
                    .ok_or(Error::SizeOverflow)?,
            )?;
            'query: for y in y0..=y1 {
                for x in x0..=x1 {
                    let at = projection.address(x, y);
                    if p.depth_bounds(x, y).1 != f64::NEG_INFINITY
                        && p.overlaps(x, y, grid)
                        && (owner[at] == t.id || p.depth_bounds(x, y).1 + guard >= depth[at])
                    {
                        visible[t.id] = true;
                        remaining -= 1;
                        break 'query;
                    }
                }
            }
        }
    }
    budget.release(depth);
    budget.release(owner);
    budget.release(partial);
    budget.release(occluders);
    if remaining != 0 {
        // Preserve the original floating-point sample guarantee as well as
        // continuous pixel coverage: legacy rays can miss an exact shared-edge
        // intersection. Only raster-hidden candidates need this compatibility
        // check; it may restore triangles and can never remove a raster-kept one.
        work.add(keep.len())?;
        let mut legacy = budget.filled(keep.len(), false)?;
        for ((candidate, &k), &v) in legacy.iter_mut().zip(keep.iter()).zip(&visible) {
            *candidate = k && !v;
        }
        let mut ignored = HiddenReport::default();
        finite_visibility(
            triangles,
            &mut legacy,
            paired,
            &mut ignored,
            options,
            26,
            epsilon,
            budget,
            work,
        )?;
        work.add(keep.len())?;
        for (v, &legacy) in visible.iter_mut().zip(&legacy) {
            *v |= legacy;
        }
        budget.release(legacy);
    }
    work.add(keep.len())?;
    for (k, v) in keep.iter_mut().zip(&visible) {
        *k &= *v;
    }
    budget.release(visible);
    Ok(())
}

/// The source-compatible u32 entry point; see [`remove_hidden_triangles_generic`] for selectable index widths.
#[inline]
pub fn remove_hidden_triangles(
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<VisibleMesh, Error> {
    remove_hidden_triangles_generic::<u32>(indices, positions, options, workspace)
}

/// The source-compatible u32 entry point; see [`remove_hidden_triangles_into_generic`] for selectable index widths.
#[inline]
pub fn remove_hidden_triangles_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    options: HiddenOptions<'_>,
    workspace: &mut Workspace,
) -> Result<HiddenReport, Error> {
    remove_hidden_triangles_into_generic::<u32>(destination, indices, positions, options, workspace)
}

/// The source-compatible u32 form of [`VisibleMeshGeneric`].
pub type VisibleMesh = VisibleMeshGeneric<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_witness_matches_full_traversal_and_checks_leaf() -> Result<(), Error> {
        let mut ws = Workspace::default();
        let mut work = ws.begin();
        let mut triangles = Vec::new();
        for id in 0..64 {
            let x = (id % 8) as f64;
            let y = (id / 8) as f64;
            triangles.push(Triangle {
                p: [[x, y, 1.], [x + 2., y, 1.], [x, y + 2., 1.]],
                id,
            });
        }
        let mut nodes = Vec::new();
        build(&mut triangles, 0, &mut nodes, &mut work)?;
        let mut witness = [None; 4];
        let mut cached_hits = 0;
        for step in 0..256 {
            let origin = [(step % 16) as f64 * 0.5, (step / 16) as f64 * 0.5, 0.];
            for view in 0..28 {
                let ray = Ray {
                    origin,
                    direction: if view < 26 {
                        direction(HiddenViews::Outside, view, origin).0
                    } else if view == 26 {
                        [0.1, 0.2, 1.]
                    } else {
                        [-0.2, 0.1, 1.]
                    },
                    distance: if step % 2 == 0 { f64::INFINITY } else { 2. },
                    epsilon: 1e-12,
                };
                for id in [usize::MAX, step % 64] {
                    let expected = blocked(ray, id, &triangles, &nodes, &mut [None; 4], &mut work)?;
                    for _ in 0..2 {
                        cached_hits += usize::from(witness[0].is_some() && expected);
                        assert_eq!(
                            blocked(ray, id, &triangles, &nodes, &mut witness, &mut work)?,
                            expected
                        );
                    }
                }
            }
        }
        assert!(cached_hits > 0);
        // A triangle hit alone is insufficient: the legacy box predicate must
        // also accept. This deliberately inconsistent fixture exercises that guard.
        let t = Triangle {
            p: [[0., 0., 1.], [2., 0., 1.], [0., 2., 1.]],
            id: 0,
        };
        let node = Node {
            min: [10.; 3],
            max: [11.; 3],
            start: 0,
            count: 1,
            end: 1,
        };
        let ray = Ray {
            origin: [0.5, 0.5, 0.],
            direction: [0., 0., 1.],
            distance: f64::INFINITY,
            epsilon: 1e-12,
        };
        assert!(hit(
            &t,
            ray.origin,
            ray.direction,
            ray.distance,
            ray.epsilon
        ));
        assert!(!blocked(
            ray,
            usize::MAX,
            &[t],
            &[node],
            &mut [Some((0, 0)); 4],
            &mut work
        )?);
        Ok(())
    }

    #[test]
    fn raster_size_validation_precedes_arithmetic() {
        for size in [0, 2049, i64::MAX as usize, usize::MAX] {
            assert!(matches!(
                RasterGrid::new(size),
                Err(Error::InvalidParameter)
            ));
        }
        for size in [1, 8, 256, 2048] {
            let grid = RasterGrid::new(size).unwrap();
            assert_eq!(grid.margin, size as i64 * 256 * 8 + 16);
            assert_eq!(grid.cells, size.div_ceil(8).pow(2) * 64);
            assert_eq!(grid.subpixel(size - 1), (size as i64 - 1) * 256);
        }
        let bound = 2048 * SUBPIXEL;
        assert_eq!(edge([0, 0], [bound, 0], [0, bound]), bound * bound);
        assert_eq!(edge([0, 0], [0, bound], [bound, 0]), -bound * bound);
    }
}
