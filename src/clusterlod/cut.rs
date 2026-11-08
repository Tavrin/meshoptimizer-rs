//! Reference cut selection and monotonicity validation for cluster-LOD DAGs.
//!
//! Not in C++ meshoptimizer. A *cut* is the set of clusters a renderer draws
//! for one view: exactly one cluster per surface region, chosen by comparing the
//! projected error of the groups that produced and consume each cluster with a
//! pixel threshold. The functions here work on the flat tables returned by
//! [`build_dag_into`](super::build_dag_into) and by
//! [`dag_format::Reader`](super::dag_format::Reader), allocate only small scratch
//! tables, and are meant as the oracle a GPU or streaming implementation is
//! compared with. The rules are stated in the runtime section of
//! `docs/DAG_FORMAT.md`.
//!
//! Terms. Cluster `c` belongs to group `G(c)` (its range in the group table) and
//! was produced by simplifying group `R(c)` (its `refined` field, `-1` for source
//! geometry). The clusters of group `Q` are `A(Q)`; the clusters with
//! `R(c) = Q` are `B(Q)`, which together replace `A(Q)`. A group `P` is a
//! *parent* of `Q` when some cluster lies in `B(Q)` and in `A(P)`; a group with
//! no children `B` is *parentless* (a stuck group or the final group).
use super::bounds_math::{abs_sum3_up, add_up, enclosing_sphere, f32_up, sqrt_up};
use super::{FlatDagCluster, FlatDagGroup, LodBounds};
use alloc::vec::Vec;

/// Failure of a cut operation; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CutError {
    /// Groups do not partition the clusters in order, a cluster names another
    /// group, a `refined` link does not point to an earlier group, or a group
    /// carries a non-finite or negative bound.
    InvalidTable,
    /// A view, threshold, error or residency slice is outside its domain.
    InvalidParameter,
    /// A caller destination is too small.
    BufferTooSmall,
    /// Sequential refinement needed a cluster that was not drawn: the residency
    /// set is not closed under parents. `group` is
    /// the group being refined.
    Incomplete {
        /// Group whose replacement clusters were not all drawn.
        group: u32,
    },
    /// A scratch allocation failed.
    Allocation,
}
impl core::fmt::Display for CutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "cut: {self:?}")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for CutError {}

/// First violation found by [`check_monotonic`]; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Violation {
    /// The tables are structurally invalid, see [`CutError::InvalidTable`].
    InvalidTable,
    /// A group has a non-finite center or radius, a negative radius, or an
    /// error that is NaN, infinite or negative.
    NonFinite {
        /// Offending group.
        group: u32,
    },
    /// A parent group has a smaller error than one of its children.
    ErrorDecreases {
        /// Child group `Q`.
        child: u32,
        /// Parent group `P`.
        parent: u32,
    },
    /// A parent group's sphere does not contain a child group's sphere.
    NotContained {
        /// Child group `Q`.
        child: u32,
        /// Parent group `P`.
        parent: u32,
    },
}

/// Counts of groups changed by [`enforce_monotonic`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MonotonicReport {
    /// Groups whose error or radius changed.
    pub groups_changed: usize,
    /// Groups whose error was raised.
    pub errors_raised: usize,
    /// Groups whose radius was grown.
    pub radii_grown: usize,
}

/// How a view maps a length in the mesh frame to pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Projection {
    /// Perspective camera. `proj_y` is the vertical projection scale
    /// `1 / tan(fov_y / 2)`; `znear` is the near distance, positive.
    Perspective {
        /// Vertical projection scale, positive.
        proj_y: f32,
        /// Near distance, positive, in the units of the camera frame.
        znear: f32,
    },
    /// Orthographic camera: error in pixels is `error * scale * pixels_per_unit`,
    /// independent of depth.
    Orthographic {
        /// Pixels per length unit of the camera frame, positive.
        pixels_per_unit: f32,
    },
}

/// A camera and instance scale for projecting group errors.
///
/// Group centers and `position` must be in one frame (the camera frame; the
/// mesh frame for an untransformed mesh, where `scale` is 1). Errors and radii are
/// lengths of the mesh frame and are multiplied by `scale` to reach that frame:
/// pass the largest singular value of the instance's linear transform (see
/// [`max_scale`]). For an instance transform, express the group centers in the
/// camera frame first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CutView {
    /// Camera position.
    pub position: [f32; 3],
    /// Projection.
    pub projection: Projection,
    /// Viewport height in pixels (perspective only), positive.
    pub viewport_height: f32,
    /// Factor applied to group errors and radii, positive and finite.
    pub scale: f32,
}

/// Error threshold of a cut.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Threshold {
    /// Largest accepted projected error in pixels, finite and not negative.
    Pixels(f32),
    /// Always the finest level: every cluster with `refined == -1` and nothing
    /// else, whatever the group errors are.
    Finest,
}

impl CutView {
    fn validate(&self) -> Result<(), CutError> {
        let positive = |v: f32| v.is_finite() && v > 0.0;
        let ok = self.position.iter().all(|v| v.is_finite())
            && positive(self.scale)
            && self.viewport_height.is_finite()
            && self.viewport_height >= 0.0
            && match self.projection {
                Projection::Perspective { proj_y, znear } => positive(proj_y) && positive(znear),
                Projection::Orthographic { pixels_per_unit } => positive(pixels_per_unit),
            };
        if ok {
            Ok(())
        } else {
            Err(CutError::InvalidParameter)
        }
    }
}

/// Projected error of a bounding sphere with an error, in pixels.
///
/// Perspective, with `d` the distance from the camera to the center and `s` =
/// `view.scale`:
/// `error * s / max(d - radius * s, znear) * (proj_y / 2) * viewport_height`.
/// Orthographic: `error * s * pixels_per_unit`. Evaluated in `f64` and rounded to
/// `f32`, so the result is the same on every host. The view is not validated
/// here; [`select_cut`] and [`refine_sequential`] reject invalid views.
pub fn projected_error(bounds: LodBounds, view: &CutView) -> f32 {
    let s = f64::from(view.scale);
    let error = f64::from(bounds.error) * s;
    let pixels = match view.projection {
        Projection::Perspective { proj_y, znear } => {
            let d = |i: usize| f64::from(bounds.center[i]) - f64::from(view.position[i]);
            let distance = libm::sqrt(d(0) * d(0) + d(1) * d(1) + d(2) * d(2));
            let denominator = (distance - f64::from(bounds.radius) * s).max(f64::from(znear));
            error / denominator * (f64::from(proj_y) * 0.5) * f64::from(view.viewport_height)
        }
        Projection::Orthographic { pixels_per_unit } => error * f64::from(pixels_per_unit),
    };
    pixels as f32
}

fn group_bounds(g: &FlatDagGroup) -> LodBounds {
    LodBounds {
        center: g.center,
        radius: g.radius,
        error: g.error,
    }
}

fn vec_filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>, CutError> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| CutError::Allocation)?;
    v.resize(n, value);
    Ok(v)
}

fn span(g: &FlatDagGroup) -> core::ops::Range<usize> {
    let first = g.first_cluster as usize;
    first..first + g.cluster_count as usize
}

/// Groups partition the clusters in order, every cluster names its group, and
/// every `refined` link points to an earlier group (so the DAG is acyclic and
/// groups can be visited fine to coarse in index order).
fn validate(groups: &[FlatDagGroup], clusters: &[FlatDagCluster]) -> Result<(), CutError> {
    if u32::try_from(clusters.len()).is_err() || i32::try_from(groups.len()).is_err() {
        return Err(CutError::InvalidTable);
    }
    let mut next = 0usize;
    for (g, group) in groups.iter().enumerate() {
        let end = (group.first_cluster as usize)
            .checked_add(group.cluster_count as usize)
            .ok_or(CutError::InvalidTable)?;
        if group.first_cluster as usize != next || end > clusters.len() {
            return Err(CutError::InvalidTable);
        }
        for c in &clusters[next..end] {
            let refined_ok = c.refined == -1 || (c.refined >= 0 && (c.refined as usize) < g);
            if c.group as usize != g || !refined_ok {
                return Err(CutError::InvalidTable);
            }
        }
        next = end;
    }
    if next == clusters.len() {
        Ok(())
    } else {
        Err(CutError::InvalidTable)
    }
}

/// For each group `Q`, the clusters of `B(Q)` in ascending order.
struct Children {
    start: Vec<u32>,
    list: Vec<u32>,
}
impl Children {
    fn build(group_count: usize, clusters: &[FlatDagCluster]) -> Result<Self, CutError> {
        let mut start = vec_filled(group_count + 1, 0u32)?;
        for c in clusters.iter().filter(|c| c.refined >= 0) {
            start[c.refined as usize + 1] += 1;
        }
        for i in 0..group_count {
            start[i + 1] += start[i];
        }
        let mut list = vec_filled(start[group_count] as usize, 0u32)?;
        let mut fill = vec_filled(group_count, 0u32)?;
        for (i, c) in clusters.iter().enumerate() {
            if c.refined >= 0 {
                let q = c.refined as usize;
                list[(start[q] + fill[q]) as usize] = i as u32;
                fill[q] += 1;
            }
        }
        Ok(Self { start, list })
    }
    fn of(&self, q: usize) -> &[u32] {
        &self.list[self.start[q] as usize..self.start[q + 1] as usize]
    }
}

fn check_resident(resident: Option<&[bool]>, groups: usize) -> Result<(), CutError> {
    match resident {
        Some(r) if r.len() != groups => Err(CutError::InvalidParameter),
        _ => Ok(()),
    }
}

/// A closure answering "does group `g` need to be replaced by its inputs".
fn needs_refinement<'a>(
    groups: &'a [FlatDagGroup],
    view: &'a CutView,
    threshold: Threshold,
) -> Result<impl Fn(usize) -> bool + 'a, CutError> {
    view.validate()?;
    let pixels = match threshold {
        Threshold::Finest => None,
        Threshold::Pixels(t) if t.is_finite() && t >= 0.0 => Some(t),
        Threshold::Pixels(_) => return Err(CutError::InvalidParameter),
    };
    Ok(move |g: usize| match pixels {
        None => true,
        Some(t) => projected_error(group_bounds(&groups[g]), view) > t,
    })
}

/// Clamp refinement coarse to fine: every group needs its own predicate
/// and all its parents to refine (vacuously true for parentless groups).
fn clamp_refinement(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    children: &Children,
    predicate: impl Fn(usize) -> bool,
) -> Result<Vec<bool>, CutError> {
    let mut refine = vec_filled(groups.len(), false)?;
    for q in (0..groups.len()).rev() {
        let parents = children.of(q);
        refine[q] = predicate(q)
            && parents
                .iter()
                .all(|&c| refine[clusters[c as usize].group as usize]);
    }
    Ok(refine)
}

/// Draw a resident owner's cluster when the owner refines (or is parentless)
/// and the producer does not refine (or is absent/source geometry).
fn select(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    resident: Option<&[bool]>,
    refine: impl Fn(usize) -> bool,
    out: &mut [u32],
) -> Result<usize, CutError> {
    validate(groups, clusters)?;
    check_resident(resident, groups.len())?;
    let children = Children::build(groups.len(), clusters)?;
    let refine = clamp_refinement(groups, clusters, &children, refine)?;
    let is_resident = |g: usize| resident.is_none_or(|r| r[g]);
    let mut n = 0;
    for (g, group) in groups.iter().enumerate() {
        if !is_resident(g) || !(children.of(g).is_empty() || refine[g]) {
            continue;
        }
        for c in span(group) {
            let r = clusters[c].refined;
            let draw = r < 0 || !refine[r as usize] || !is_resident(r as usize);
            if draw {
                *out.get_mut(n).ok_or(CutError::BufferTooSmall)? = c as u32;
                n += 1;
            }
        }
    }
    Ok(n)
}

/// The cut for one view: writes the indices of the clusters to draw, in
/// ascending order, and returns how many.
///
/// Every group refines only when
/// its projected error exceeds the threshold and all its parents refine.
/// Draw `c` iff `G(c)` refines (or is parentless) and either `c` is source
/// geometry or `R(c)` does not refine. This ancestor clamp prevents rounding
/// from reopening a stopped
/// region. With `resident`, `G(c)` must be resident and an absent `R(c)` stops
/// refinement. A resident set closed under parents and containing every
/// parentless group equals [`refine_sequential`] and covers every surface
/// region exactly once, independently of projected-error ordering.
///
/// # Errors
/// `InvalidTable`, `InvalidParameter` (invalid view, threshold, or a `resident`
/// slice of the wrong length), `BufferTooSmall`, `Allocation`.
pub fn select_cut(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    view: &CutView,
    threshold: Threshold,
    resident: Option<&[bool]>,
    out: &mut [u32],
) -> Result<usize, CutError> {
    let refine = needs_refinement(groups, view, threshold)?;
    select(groups, clusters, resident, refine, out)
}

/// The view-independent cut at a fixed group error, with the same ancestor
/// clamp as [`select_cut`], comparing raw group errors instead of projected
/// errors. Groups refine only if
/// their error exceeds `error` and all their parents refine.
/// Suitable for fixed-error proxies. Both the estimate and the cut are
/// approximations; neither is a certificate of geometric distance.
///
/// # Errors
/// `InvalidParameter` if `error` is negative or not finite, plus the errors of
/// [`select_cut`].
pub fn select_error_cut(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    error: f32,
    out: &mut [u32],
) -> Result<usize, CutError> {
    if !(error.is_finite() && error >= 0.0) {
        return Err(CutError::InvalidParameter);
    }
    select(groups, clusters, None, |g| groups[g].error > error, out)
}

/// The oracle for [`select_cut`]: refines a drawn set group by group, the way a
/// streaming runtime does.
///
/// Starts from the clusters of the resident parentless groups and visits groups
/// from the last to the first (coarse to fine; `refined` links point to earlier
/// groups). When resident group `Q` passes the ancestor-clamped threshold,
/// `B(Q)` is replaced by `A(Q)`; every cluster of `B(Q)` must be drawn at
/// that point. Writes the drawn
/// clusters in ascending order and returns how many.
///
/// # Errors
/// [`CutError::Incomplete`] if a cluster of `B(Q)` is not drawn, which is
/// a residency-closure violation; otherwise the errors of [`select_cut`].
pub fn refine_sequential(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    view: &CutView,
    threshold: Threshold,
    resident: Option<&[bool]>,
    out: &mut [u32],
) -> Result<usize, CutError> {
    let refine = needs_refinement(groups, view, threshold)?;
    validate(groups, clusters)?;
    check_resident(resident, groups.len())?;
    let children = Children::build(groups.len(), clusters)?;
    let refine = clamp_refinement(groups, clusters, &children, refine)?;
    let is_resident = |g: usize| resident.is_none_or(|r| r[g]);
    let mut drawn = vec_filled(clusters.len(), false)?;
    for (g, group) in groups.iter().enumerate() {
        if children.of(g).is_empty() && is_resident(g) {
            drawn[span(group)].fill(true);
        }
    }
    for q in (0..groups.len()).rev() {
        if !is_resident(q) || !refine[q] {
            continue;
        }
        for &c in children.of(q) {
            if !drawn[c as usize] {
                return Err(CutError::Incomplete { group: q as u32 });
            }
            drawn[c as usize] = false;
        }
        drawn[span(&groups[q])].fill(true);
    }
    let mut n = 0;
    for (c, _) in drawn.iter().enumerate().filter(|(_, d)| **d) {
        *out.get_mut(n).ok_or(CutError::BufferTooSmall)? = c as u32;
        n += 1;
    }
    Ok(n)
}

/// Parent groups of every group, as a compressed table.
///
/// `offsets` (at least `groups.len() + 1` entries) receives, for each group `Q`,
/// the start of its parents in `parents`; `offsets[groups.len()]` is the total.
/// Parents of one group are distinct and ascending. Returns the total number of
/// parent links.
///
/// # Errors
/// `InvalidTable`, `BufferTooSmall`, `Allocation`.
pub fn group_parents(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    offsets: &mut [u32],
    parents: &mut [u32],
) -> Result<usize, CutError> {
    validate(groups, clusters)?;
    if offsets.len() <= groups.len() {
        return Err(CutError::BufferTooSmall);
    }
    let children = Children::build(groups.len(), clusters)?;
    let mut n = 0usize;
    for (q, offset) in offsets.iter_mut().take(groups.len()).enumerate() {
        *offset = n as u32;
        let mut last = None;
        for &c in children.of(q) {
            let p = clusters[c as usize].group;
            if last != Some(p) {
                *parents.get_mut(n).ok_or(CutError::BufferTooSmall)? = p;
                n += 1;
                last = Some(p);
            }
        }
    }
    offsets[groups.len()] = n as u32;
    Ok(n)
}

/// Upper bound of `|center_p - center_q| + radius_q`: the radius a sphere at
/// `center_p` needs to contain sphere `q`.
fn containing_radius(p: &FlatDagGroup, q: &FlatDagGroup) -> f64 {
    super::bounds_math::containing_radius(p.center, q.center, q.radius)
}

fn group_is_finite(g: &FlatDagGroup) -> bool {
    g.center.iter().all(|v| v.is_finite())
        && g.radius.is_finite()
        && g.radius >= 0.0
        && g.error.is_finite()
        && g.error >= 0.0
}

/// Checks that every parent group has at least the error of each child group and
/// a sphere containing each child group's sphere.
///
/// These stored bounds order ideal projected errors, but floating-point
/// evaluation can invert that ordering. Cut selectors additionally clamp
/// refinement by ancestors; this validator does not replace that clamp.
/// Containment is evaluated in upward-rounded `f64`, so a miss of one `f32` ulp
/// is reported and equal spheres pass. Returns the first violation in cluster
/// order. Allocates nothing.
///
/// # Errors
/// The first [`Violation`] found.
pub fn check_monotonic(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
) -> Result<(), Violation> {
    validate(groups, clusters).map_err(|_| Violation::InvalidTable)?;
    if let Some(g) = groups.iter().position(|g| !group_is_finite(g)) {
        return Err(Violation::NonFinite { group: g as u32 });
    }
    for (p, parent) in groups.iter().enumerate() {
        for c in &clusters[span(parent)] {
            if c.refined < 0 {
                continue;
            }
            let child = &groups[c.refined as usize];
            let ids = (c.refined as u32, p as u32);
            if parent.error < child.error {
                return Err(Violation::ErrorDecreases {
                    child: ids.0,
                    parent: ids.1,
                });
            }
            if containing_radius(parent, child) > f64::from(parent.radius) {
                return Err(Violation::NotContained {
                    child: ids.0,
                    parent: ids.1,
                });
            }
        }
    }
    Ok(())
}

/// Repairs a DAG so that [`check_monotonic`] passes.
///
/// Walks groups fine to coarse; each parent's error is raised to its children's
/// and its radius grown to the distance between centers plus the child radius,
/// computed in upward-rounded `f64` and rounded up to `f32`. Centers and
/// cluster tables are untouched. A DAG that already passes is not modified. Use
/// it when the builder ran with `simplify_error_merge_previous` below one, which
/// does not give monotonic errors.
///
/// # Errors
/// `InvalidTable` for invalid tables, non-finite bounds, or a radius that would
/// overflow `f32`.
pub fn enforce_monotonic(
    groups: &mut [FlatDagGroup],
    clusters: &[FlatDagCluster],
) -> Result<MonotonicReport, CutError> {
    validate(groups, clusters)?;
    if !groups.iter().all(group_is_finite) {
        return Err(CutError::InvalidTable);
    }
    let mut report = MonotonicReport::default();
    for p in 0..groups.len() {
        let (mut raised, mut grown) = (false, false);
        for c in span(&groups[p]) {
            if clusters[c].refined < 0 {
                continue;
            }
            let child = groups[clusters[c].refined as usize];
            let parent = &mut groups[p];
            if parent.error < child.error {
                parent.error = child.error;
                raised = true;
            }
            let needed = containing_radius(parent, &child);
            if needed > f64::from(parent.radius) {
                let radius = f32_up(needed);
                if !radius.is_finite() {
                    return Err(CutError::InvalidTable);
                }
                parent.radius = radius;
                grown = true;
            }
        }
        report.errors_raised += usize::from(raised);
        report.radii_grown += usize::from(grown);
        report.groups_changed += usize::from(raised || grown);
    }
    Ok(report)
}

/// Producers of root clusters: `R(c)` for every cluster `c` of a parentless
/// group with `R(c) != -1`, once per cluster.
fn root_producers<'t>(
    groups: &'t [FlatDagGroup],
    clusters: &'t [FlatDagCluster],
    children: &'t Children,
) -> impl Iterator<Item = &'t FlatDagGroup> + Clone + 't {
    groups
        .iter()
        .enumerate()
        .filter(|(g, _)| children.of(*g).is_empty())
        .flat_map(move |(_, group)| clusters[span(group)].iter())
        .filter_map(move |c| usize::try_from(c.refined).ok().map(|q| &groups[q]))
}

/// Root bounds of a DAG for the instance fast path: `error` is `E_root`, the
/// largest error among the producers `R(c)` of the clusters of parentless
/// groups (zero when no root cluster has a producer), and the sphere `S`
/// contains the LOD sphere of every such producer (zero when there is none).
///
/// The root cut (every cluster of every parentless group, see
/// [`root_clusters`]) is the cut of a view exactly when no root producer
/// refines; [`is_root_cut`] with these bounds is a sufficient test. Parentless
/// group records are not used: their own spheres need not contain the
/// producers' and their errors are usually `f32::MAX`.
///
/// # Errors
/// `InvalidTable` for invalid tables or a non-finite producer bound;
/// `Allocation`.
pub fn root_bounds(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
) -> Result<LodBounds, CutError> {
    validate(groups, clusters)?;
    let children = Children::build(groups.len(), clusters)?;
    let producers = root_producers(groups, clusters, &children);
    let mut error = 0.0f32;
    for q in producers.clone() {
        if !group_is_finite(q) {
            return Err(CutError::InvalidTable);
        }
        error = error.max(q.error);
    }
    let none = producers.clone().next().is_none();
    let (center, radius) = match enclosing_sphere(producers.map(|q| (q.center, q.radius))) {
        Some(s) => s,
        None if none => ([0.0; 3], 0.0),
        None => return Err(CutError::InvalidTable),
    };
    Ok(LodBounds {
        center,
        radius,
        error,
    })
}

/// `E_root` of [`root_bounds`]: the largest error among the producers of root
/// clusters, zero when no root cluster has a producer.
///
/// # Errors
/// As [`root_bounds`].
pub fn root_error(groups: &[FlatDagGroup], clusters: &[FlatDagCluster]) -> Result<f32, CutError> {
    root_bounds(groups, clusters).map(|b| b.error)
}

/// The root cut: the clusters of every parentless group, in ascending order.
/// Writes them and returns how many. Every resident set contains them, and
/// they are what [`select_cut`] returns whenever [`is_root_cut`] holds.
///
/// # Errors
/// `InvalidTable`, `BufferTooSmall`, `Allocation`.
pub fn root_clusters(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    out: &mut [u32],
) -> Result<usize, CutError> {
    validate(groups, clusters)?;
    let children = Children::build(groups.len(), clusters)?;
    let mut n = 0;
    for (g, group) in groups.iter().enumerate() {
        if !children.of(g).is_empty() {
            continue;
        }
        for c in span(group) {
            *out.get_mut(n).ok_or(CutError::BufferTooSmall)? = c as u32;
            n += 1;
        }
    }
    Ok(n)
}

/// Relative margin taken off the root sphere's perspective denominator in
/// [`is_root_cut`]: it covers the `f64` rounding of the distance and radius
/// terms of [`projected_error`] (a few units of 2^-53 each), so that the test
/// stays sufficient after rounding and not only in exact arithmetic.
const ROOT_MARGIN: f64 = 1.0 / (1u64 << 44) as f64;

/// The instance fast path: true when every root producer projects at or below
/// the threshold, so the cut of this view is the root cut ([`root_clusters`])
/// and no group needs to be evaluated.
///
/// `sphere` is `[x, y, z, radius]` and `root_error` is `E_root`, both from
/// [`root_bounds`] or from the MDAG 2 reader (`Reader::root_sphere`,
/// `Reader::root_error`), with the sphere center in the frame of
/// `view.position` and `view.scale` set as for [`select_cut`]. Because the
/// sphere contains every producer's LOD sphere and `root_error` is at least
/// every producer's error, the projection of `(sphere, root_error)` is at least
/// each producer's [`projected_error`]; the perspective denominator is reduced
/// by a relative 2^-44 so that the inequality also holds after rounding. The
/// value is compared with `t` in `f64`. So when this returns true no producer
/// refines, and the ancestor clamp of [`select_cut`] then stops every chain at
/// the root, with or without residency.
///
/// The test is sufficient, not necessary: it returns false for
/// [`Threshold::Finest`], for an invalid view, threshold or bound, and near the
/// threshold where only some producers refine. A caller that gets false runs
/// the full cut. For a mesh with a far-field impostor, test the impostor's own
/// switch first: its world error is not ordered with `E_root`.
pub fn is_root_cut(sphere: [f32; 4], root_error: f32, view: &CutView, t: Threshold) -> bool {
    let Threshold::Pixels(t) = t else {
        return false;
    };
    let bound_ok = sphere.iter().all(|v| v.is_finite())
        && sphere[3] >= 0.0
        && root_error.is_finite()
        && root_error >= 0.0;
    if !(bound_ok && t.is_finite() && t >= 0.0) || view.validate().is_err() {
        return false;
    }
    let s = f64::from(view.scale);
    let error = f64::from(root_error) * s;
    let pixels = match view.projection {
        Projection::Perspective { proj_y, znear } => {
            let d = |i: usize| f64::from(sphere[i]) - f64::from(view.position[i]);
            let distance = libm::sqrt(d(0) * d(0) + d(1) * d(1) + d(2) * d(2));
            let reach = f64::from(sphere[3]) * s;
            let denominator =
                (distance - reach - ROOT_MARGIN * (distance + 2.0 * reach)).max(f64::from(znear));
            error / denominator * (f64::from(proj_y) * 0.5) * f64::from(view.viewport_height)
        }
        Projection::Orthographic { pixels_per_unit } => error * f64::from(pixels_per_unit),
    };
    pixels <= f64::from(t)
}

/// Gathers the fixed-error cut ([`select_error_cut`]) as one triangle list, for
/// baking a far-field impostor of a whole cell or assembly: writes the
/// `indices` spans of the cut clusters, concatenated in ascending cluster
/// order, into `out`, and returns the index count. With `triangle_clusters`,
/// writes the cut cluster of every written triangle, so a texture sampler
/// can map a triangle of the list back to its cluster's material.
///
/// `indices` is the index prefix the cluster table refers to (the flat output
/// of [`build_dag_into`](super::build_dag_into)).
///
/// # Errors
/// `InvalidTable` if a cut cluster's span is outside `indices` or not whole
/// triangles; `BufferTooSmall` if `out` or `triangle_clusters` is short; plus
/// the errors of [`select_error_cut`].
pub fn gather_error_cut(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    indices: &[u32],
    error: f32,
    out: &mut [u32],
    mut triangle_clusters: Option<&mut [u32]>,
) -> Result<usize, CutError> {
    let mut cut = vec_filled(clusters.len(), 0u32)?;
    let k = select_error_cut(groups, clusters, error, &mut cut)?;
    let (mut n, mut triangles) = (0usize, 0usize);
    for &c in &cut[..k] {
        let cluster = &clusters[c as usize];
        let first = cluster.first_index as usize;
        let count = cluster.index_count as usize;
        let span = first
            .checked_add(count)
            .and_then(|end| indices.get(first..end))
            .filter(|s| s.len() % 3 == 0)
            .ok_or(CutError::InvalidTable)?;
        out.get_mut(n..n + count)
            .ok_or(CutError::BufferTooSmall)?
            .copy_from_slice(span);
        n += count;
        if let Some(tc) = triangle_clusters.as_deref_mut() {
            let end = triangles + count / 3;
            tc.get_mut(triangles..end)
                .ok_or(CutError::BufferTooSmall)?
                .fill(c);
            triangles = end;
        }
    }
    Ok(n)
}

/// A conservative largest singular value of a 3x3 linear transform, never below
/// the true value, for scaling group errors and radii under an instance
/// transform (see [`CutView::scale`]).
///
/// Square root of the largest Gershgorin row bound of `MᵀM`, with every
/// operation rounded upward, so the identity, rotations of exact axes and
/// uniform scales give their exact value. Uses `linear[row][column]`. Returns
/// infinity for a non-finite matrix.
pub fn max_scale(linear: [[f32; 3]; 3]) -> f32 {
    if linear.iter().flatten().any(|v| !v.is_finite()) {
        return f32::INFINITY;
    }
    let m = |r: usize, c: usize| f64::from(linear[r][c]);
    let mut bound = 0.0f64;
    for i in 0..3 {
        let mut row = 0.0;
        for j in 0..3 {
            // Products of two f32 values are exact in f64.
            let g = abs_sum3_up([0, 1, 2].map(|k| m(k, i) * m(k, j)));
            row = add_up(row, g);
        }
        bound = bound.max(row);
    }
    f32_up(sqrt_up(bound))
}
