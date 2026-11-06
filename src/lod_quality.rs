//! Sampled LOD surface quality, an extension not in C++ meshoptimizer.
use crate::{
    budget::Budget, workspace::Work, Error, LodChain, LodChainGeneric, Positions, Workspace,
};
use alloc::vec::Vec;

type V = [f64; 3];

/// A borrowed triangle surface. Only referenced vertices are measured/validated.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodQualityMeshGeneric<'a, I: crate::Index = u32> {
    /// Triangle-list indices; at least one triangle is required.
    pub indices: &'a [I],
    /// Positions in the same world units as the reported distances.
    pub positions: Positions<'a>,
    /// Optional per-vertex normals, normalized before interpolation.
    pub normals: Option<&'a [[f32; 3]]>,
}
impl<'a, I: crate::Index> LodQualityMeshGeneric<'a, I> {
    /// Borrow a surface without normals. Set `normals` to compare shading normals.
    pub fn new_generic(indices: &'a [I], positions: Positions<'a>) -> Self {
        Self {
            indices,
            positions,
            normals: None,
        }
    }
}

/// Deterministic sampling settings (not in C++ meshoptimizer).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodQualityOptions {
    /// Interior samples per square world unit, default 1. Must be finite and
    /// nonnegative. Each triangle gets `ceil(area * density)` samples, plus its
    /// three corners (shared corners are repeated). Zero disables interiors.
    /// Degenerate triangles have corners only. More than `u32::MAX` interior
    /// samples on one triangle returns `SizeOverflow`; work limits apply first
    /// to every visit that is actually attempted.
    pub samples_per_unit_area: f64,
}
impl Default for LodQualityOptions {
    fn default() -> Self {
        Self {
            samples_per_unit_area: 1.0,
        }
    }
}

/// Sampled distance statistics in world units.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct LodDistance {
    /// Largest sampled nearest-surface distance, a lower estimate of Hausdorff distance.
    pub max: f64,
    /// Root mean square over samples, with equal weight per sample.
    pub rms: f64,
    /// Number of samples, including repeated triangle corners.
    pub samples: u64,
}
/// Shading-normal angular deviation in radians.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct LodNormalDeviation {
    /// Largest angle, in `[0, pi]`.
    pub max: f64,
    /// Arithmetic mean angle over samples in both directions.
    pub mean: f64,
}
/// Quality of a level relative to the source; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct LodQuality {
    /// One-sided source-to-level measurements.
    pub source_to_level: LodDistance,
    /// One-sided level-to-source measurements.
    pub level_to_source: LodDistance,
    /// Maximum and pooled RMS across both directions.
    pub symmetric: LodDistance,
    /// Pooled normal angles, present when both surfaces supply normals.
    pub normals: Option<LodNormalDeviation>,
}

/// Pinhole camera used to turn world-space deviation into switch distance.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodCamera {
    /// Vertical field of view in radians, strictly between zero and pi.
    /// All positive finite values below pi are valid, including subnormals.
    pub vertical_fov: f64,
    /// Positive viewport height in pixels.
    pub viewport_height: f64,
    /// Positive allowed projected deviation in pixels.
    pub pixel_tolerance: f64,
}
impl LodCamera {
    /// Construct and validate a camera.
    /// # Errors
    /// `InvalidParameter` for non-finite or out-of-domain parameters.
    pub fn new(
        vertical_fov: f64,
        viewport_height: f64,
        pixel_tolerance: f64,
    ) -> Result<Self, Error> {
        let camera = Self {
            vertical_fov,
            viewport_height,
            pixel_tolerance,
        };
        camera.switch_distance(0.0)?;
        Ok(camera)
    }
    /// Minimum camera-space depth in the pinhole model:
    /// `deviation * viewport_height / (2 * tan(vertical_fov/2) * pixel_tolerance)`.
    /// This is a depth threshold for a transverse displacement, not a raster
    /// error guarantee. Use nearest mesh depth (or add a bounding radius when
    /// measuring camera-to-center distance). Sampled error can miss extrema.
    /// # Errors
    /// `InvalidParameter` for invalid camera or negative/non-finite deviation;
    /// `NumericalFailure` if the result overflows or a positive threshold
    /// underflows to zero. A zero deviation always returns zero.
    pub fn switch_distance(&self, deviation: f64) -> Result<f64, Error> {
        if !self.vertical_fov.is_finite()
            || self.vertical_fov <= 0.0
            || self.vertical_fov >= core::f64::consts::PI
            || !self.viewport_height.is_finite()
            || self.viewport_height <= 0.0
            || !self.pixel_tolerance.is_finite()
            || self.pixel_tolerance <= 0.0
            || !deviation.is_finite()
            || deviation < 0.0
        {
            return Err(Error::InvalidParameter);
        }
        if deviation == 0.0 {
            return Ok(0.0);
        }
        // Logs avoid overflow/underflow in intermediate products of valid inputs.
        // For tiny angles, 2*tan(fov/2) rounds to fov; avoid halving a
        // subnormal before taking its logarithm. The relative approximation
        // error below this cutoff is smaller than f64 precision.
        let log_scale = if self.vertical_fov < 1e-8 {
            libm::log(self.vertical_fov)
        } else {
            libm::log(2.0) + libm::log(libm::tan(self.vertical_fov / 2.0))
        };
        let log = libm::log(deviation) + libm::log(self.viewport_height)
            - log_scale
            - libm::log(self.pixel_tolerance);
        let distance = libm::exp(log);
        if !distance.is_finite() || distance == 0.0 {
            return Err(Error::NumericalFailure);
        }
        Ok(distance)
    }
}
/// One chain level's quality and optional camera threshold.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct LodQualityLevel {
    /// Measured directly against the source, never against the previous level.
    pub quality: LodQuality,
    /// Minimum depth for this level's sampled symmetric maximum.
    pub switch_distance: Option<f64>,
}

/// Measure a source and a simplified surface, with a balanced triangle BVH.
/// This extension is not in C++ meshoptimizer and does not change simplification.
///
/// Each triangle contributes its corners in index order, then area-scaled
/// interiors. For interior ordinal `j` of `n`, let `r=sqrt((j+0.5)/n)` and
/// `t=(reverse_bits(j)+0.5)/2^32`; barycentric weights are `(1-r, r*(1-t), r*t)`.
/// Samples are streamed without allocating a sample array. Degenerate triangles
/// use segment/point distances. Normals are normalized per vertex, interpolated
/// at the sample and closest point, then normalized again. Nearest-point ties
/// choose the lowest original target triangle ordinal. Both or neither surface
/// must supply normals. Opposite normals give pi (no absolute dot product).
///
/// Maxima are sampled lower estimates, not certified continuous Hausdorff
/// bounds. RMS/mean weight repeated corners and interiors equally, not area
/// integrals. Higher density improves coverage but does not certify accuracy.
/// Deterministic for identical inputs on the same build/target.
///
/// Work counts validation corners/triangles, BVH bounding/partition visits,
/// samples and visited BVH nodes. Bounds and distances use `f64` arithmetic.
/// Typical query cost is samples × log(triangles); overlapping/adversarial
/// surfaces can visit more nodes, always bounded by `Limits::max_work`.
/// Heap storage is fallible, charged before allocation, and released on return;
/// existing workspace scratch is accounted for and may be evicted under pressure.
/// # Errors
/// `InvalidTopology`, `IndexOutOfBounds`, `InvalidParameter` (empty surface,
/// density, normal length or zero normals), `NumericalFailure` (non-finite
/// positions/normals or zero interpolated normal), or resource/size errors.
pub fn measure_lod_quality_generic<I: crate::Index>(
    source: LodQualityMeshGeneric<'_, I>,
    level: LodQualityMeshGeneric<'_, I>,
    options: &LodQualityOptions,
    workspace: &mut Workspace,
) -> Result<LodQuality, Error> {
    let mut work = workspace.begin();
    let result = measure(
        source,
        level,
        options,
        &mut Budget::new(workspace),
        &mut work,
    );
    workspace.finish(&work);
    result
}

/// Measure every `build_lod_chain` level directly against `source` using its
/// unchanged vertex/normal buffers. Limits cover the entire call, including
/// the returned metadata. An empty chain returns an empty vector.
/// Not in C++ meshoptimizer. Thresholds follow measured errors and are not
/// forced to increase: independent simplifications can have nonmonotone error.
/// # Errors
/// The errors of [`measure_lod_quality`] or [`LodCamera::switch_distance`].
pub fn measure_lod_chain_quality_generic<I: crate::Index>(
    source: LodQualityMeshGeneric<'_, I>,
    chain: &LodChainGeneric<I>,
    options: &LodQualityOptions,
    camera: Option<LodCamera>,
    workspace: &mut Workspace,
) -> Result<Vec<LodQualityLevel>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_options(options)?;
        if let Some(camera) = camera {
            camera.switch_distance(0.0)?;
        }
        work.add(chain.levels.len())?;
        let mut budget = Budget::new(workspace);
        let mut out = budget.reserved(chain.levels.len())?;
        for level in &chain.levels {
            let mesh = LodQualityMeshGeneric {
                indices: &level.indices,
                ..source
            };
            let quality = measure(source, mesh, options, &mut budget, &mut work)?;
            out.push(LodQualityLevel {
                quality,
                switch_distance: camera
                    .map(|c| c.switch_distance(quality.symmetric.max))
                    .transpose()?,
            });
        }
        Ok(out)
    })();
    workspace.finish(&work);
    result
}

fn validate_options(options: &LodQualityOptions) -> Result<(), Error> {
    if !options.samples_per_unit_area.is_finite() || options.samples_per_unit_area < 0.0 {
        return Err(Error::InvalidParameter);
    }
    Ok(())
}
fn position<I: crate::Index>(mesh: LodQualityMeshGeneric<'_, I>, index: I) -> Result<V, Error> {
    let i = usize::try_from(index.to_u32()).map_err(|_| Error::SizeOverflow)?;
    let p = mesh
        .positions
        .get(i)
        .ok_or(Error::IndexOutOfBounds)?
        .map(f64::from);
    if !p.iter().all(|v| v.is_finite()) {
        return Err(Error::NumericalFailure);
    }
    Ok(p)
}
fn unit(v: V) -> Result<V, Error> {
    let d = libm::sqrt(dot(v, v));
    if !d.is_finite() {
        return Err(Error::NumericalFailure);
    }
    if d == 0.0 {
        return Err(Error::InvalidParameter);
    }
    Ok(v.map(|x| x / d))
}
fn normal<I: crate::Index>(mesh: LodQualityMeshGeneric<'_, I>, index: I) -> Result<V, Error> {
    match mesh.normals {
        Some(normals) => unit(
            normals[usize::try_from(index.to_u32()).map_err(|_| Error::SizeOverflow)?]
                .map(f64::from),
        ),
        None => Ok([0.0; 3]),
    }
}
fn validate<I: crate::Index>(
    mesh: LodQualityMeshGeneric<'_, I>,
    work: &mut Work,
) -> Result<(), Error> {
    if !mesh.indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    if mesh.indices.is_empty()
        || mesh
            .normals
            .is_some_and(|n| n.len() != mesh.positions.len())
    {
        return Err(Error::InvalidParameter);
    }
    for &i in mesh.indices {
        work.add(1)?;
        position(mesh, i)?;
        normal(mesh, i)?;
    }
    Ok(())
}
fn measure<I: crate::Index>(
    source: LodQualityMeshGeneric<'_, I>,
    level: LodQualityMeshGeneric<'_, I>,
    options: &LodQualityOptions,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<LodQuality, Error> {
    validate_options(options)?;
    if source.normals.is_some() != level.normals.is_some() {
        return Err(Error::InvalidParameter);
    }
    validate(source, work)?;
    validate(level, work)?;
    let a = direction::<I, false, 3>(source, level, options, budget, work, 0.0)?;
    let b = direction::<I, false, 3>(level, source, options, budget, work, 0.0)?;
    let samples = a.count.checked_add(b.count).ok_or(Error::SizeOverflow)?;
    Ok(LodQuality {
        source_to_level: a.distance(),
        level_to_source: b.distance(),
        symmetric: LodDistance {
            max: libm::sqrt(a.max.max(b.max)),
            rms: libm::sqrt((a.sum + b.sum) / samples as f64),
            samples,
        },
        normals: source.normals.map(|_| LodNormalDeviation {
            max: a.angle_max.max(b.angle_max),
            mean: (a.angle_sum + b.angle_sum) / samples as f64,
        }),
    })
}
// The builder consumes only the symmetric maximum. Keep the public statistics
// path exact, including nearest-triangle ties, RMS and shading normals.
#[cfg(feature = "clusterlod")]
pub(crate) fn measure_max(
    source: LodQualityMesh<'_>,
    level: LodQualityMesh<'_>,
    options: &LodQualityOptions,
    workspace: &mut Workspace,
) -> Result<f64, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        validate_options(options)?;
        let source: LodQualityMeshGeneric<'_, u32> = source.into();
        let level: LodQualityMeshGeneric<'_, u32> = level.into();
        validate(source, &mut work)?;
        validate(level, &mut work)?;
        let mut budget = Budget::new(workspace);
        let a = direction::<u32, true, 0>(source, level, options, &mut budget, &mut work, 0.0)?;
        let b = direction::<u32, true, 0>(level, source, options, &mut budget, &mut work, a.max)?;
        Ok(libm::sqrt(b.max))
    })();
    workspace.finish(&work);
    result
}

fn sub(a: V, b: V) -> V {
    core::array::from_fn(|k| a[k] - b[k])
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn blend(p: [V; 3], w: V) -> V {
    core::array::from_fn(|k| p[0][k] * w[0] + p[1][k] * w[1] + p[2][k] * w[2])
}
#[derive(Clone, Copy)]
// Distance-only records omit unused normals; full statistics retain all three.
struct Triangle<const NORMALS: usize = 3> {
    p: [V; 3],
    n: [V; NORMALS],
    id: usize,
}
impl<const NORMALS: usize> Triangle<NORMALS> {
    fn read<I: crate::Index>(mesh: LodQualityMeshGeneric<'_, I>, i: usize) -> Result<Self, Error> {
        let ix = &mesh.indices[i * 3..i * 3 + 3];
        let mut triangle = Self {
            p: [
                position(mesh, ix[0])?,
                position(mesh, ix[1])?,
                position(mesh, ix[2])?,
            ],
            n: [[0.0; 3]; NORMALS],
            id: i,
        };
        for (value, &index) in triangle.n.iter_mut().zip(ix) {
            *value = normal(mesh, index)?;
        }
        Ok(triangle)
    }
    fn normals(&self) -> [V; 3] {
        let mut normals = [[0.0; 3]; 3];
        for (to, &from) in normals.iter_mut().zip(&self.n) {
            *to = from;
        }
        normals
    }
    fn center(&self, axis: usize) -> f64 {
        self.p[0][axis] + self.p[1][axis] + self.p[2][axis]
    }
}
#[derive(Clone, Copy)]
struct Node {
    lo: V,
    hi: V,
    left: usize,
    right: usize,
}
struct Tree<const NORMALS: usize = 3> {
    triangles: Vec<Triangle<NORMALS>>,
    nodes: Vec<Node>,
}
impl<const NORMALS: usize> Tree<NORMALS> {
    fn build<I: crate::Index>(
        mesh: LodQualityMeshGeneric<'_, I>,
        budget: &mut Budget<'_>,
        work: &mut Work,
    ) -> Result<Self, Error> {
        let n = mesh.indices.len() / 3;
        work.add(n)?; // initialization is charged before reserving/reading records
        let mut triangles = budget.reserved(n)?;
        for i in 0..n {
            triangles.push(Triangle::<NORMALS>::read(mesh, i)?);
        }
        let count = n
            .checked_mul(2)
            .and_then(|n| n.checked_sub(1))
            .ok_or(Error::SizeOverflow)?;
        work.add(count)?;
        let nodes = budget.reserved(count)?;
        let mut tree = Self { triangles, nodes };
        tree.build_range(0, n, work)?;
        Ok(tree)
    }
    fn build_range(&mut self, start: usize, end: usize, work: &mut Work) -> Result<usize, Error> {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for t in &self.triangles[start..end] {
            work.add(1)?;
            for p in t.p {
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        let id = self.nodes.len();
        self.nodes.push(Node {
            lo,
            hi,
            left: start,
            right: usize::MAX,
        });
        if end - start > 1 {
            let axis = (0..3)
                .max_by(|&a, &b| (hi[a] - lo[a]).total_cmp(&(hi[b] - lo[b])))
                .unwrap_or(0);
            let mid = start + (end - start) / 2;
            select(&mut self.triangles[start..end], mid - start, axis, work)?;
            let left = self.build_range(start, mid, work)?;
            let right = self.build_range(mid, end, work)?;
            self.nodes[id].left = left;
            self.nodes[id].right = right;
        }
        Ok(id)
    }
    fn query<const MAX_ONLY: bool>(
        &self,
        id: usize,
        p: V,
        best: &mut Hit,
        work: &mut Work,
        cutoff: f64,
    ) -> Result<(), Error> {
        // A witnessed distance below the running maximum cannot raise it.
        // Otherwise finish the exact nearest query with the original arithmetic.
        if MAX_ONLY && best.distance <= cutoff {
            return Ok(());
        }
        work.add(1)?;
        let n = self.nodes[id];
        if box_distance(n, p) > best.distance {
            return Ok(());
        }
        if n.right == usize::MAX {
            let t = self.triangles[n.left];
            let (distance, weights) = closest(t.p, p);
            if distance < best.distance || (distance == best.distance && t.id < best.id) {
                *best = Hit {
                    distance,
                    weights,
                    id: t.id,
                    normals: t.normals(),
                    triangle: n.left,
                };
            }
        } else {
            let (a, b) =
                if box_distance(self.nodes[n.left], p) <= box_distance(self.nodes[n.right], p) {
                    (n.left, n.right)
                } else {
                    (n.right, n.left)
                };
            self.query::<MAX_ONLY>(a, p, best, work, cutoff)?;
            self.query::<MAX_ONLY>(b, p, best, work, cutoff)?;
        }
        Ok(())
    }
}
// Metered median partition. A bounded number of quickselect passes falls back
// to heapsort, preventing adversarial quadratic build work. No hidden sort scans.
fn select<const NORMALS: usize>(
    a: &mut [Triangle<NORMALS>],
    mid: usize,
    axis: usize,
    work: &mut Work,
) -> Result<(), Error> {
    let key = |t: Triangle<NORMALS>| (t.center(axis), t.id);
    let cmp = |a: Triangle<NORMALS>, b: Triangle<NORMALS>| {
        key(a).0.total_cmp(&key(b).0).then(a.id.cmp(&b.id))
    };
    let mut lo = 0;
    let mut hi = a.len();
    let mut passes = 2 * (usize::BITS - a.len().leading_zeros());
    while hi - lo > 1 {
        if passes == 0 {
            heap_sort(&mut a[lo..hi], cmp, work)?;
            break;
        }
        passes -= 1;
        let pivot = a[lo + (hi - lo) / 2];
        let mut store = lo;
        a.swap(lo + (hi - lo) / 2, hi - 1);
        for i in lo..hi - 1 {
            work.add(1)?;
            if cmp(a[i], pivot).is_lt() {
                a.swap(i, store);
                store += 1;
            }
        }
        a.swap(store, hi - 1);
        if store == mid {
            break;
        }
        if store < mid {
            lo = store + 1;
        } else {
            hi = store;
        }
    }
    Ok(())
}
fn heap_sort<const NORMALS: usize>(
    a: &mut [Triangle<NORMALS>],
    cmp: impl Fn(Triangle<NORMALS>, Triangle<NORMALS>) -> core::cmp::Ordering + Copy,
    work: &mut Work,
) -> Result<(), Error> {
    fn sift<const INNER_NORMALS: usize>(
        a: &mut [Triangle<INNER_NORMALS>],
        mut root: usize,
        end: usize,
        cmp: impl Fn(Triangle<INNER_NORMALS>, Triangle<INNER_NORMALS>) -> core::cmp::Ordering + Copy,
        work: &mut Work,
    ) -> Result<(), Error> {
        while root < end / 2 {
            let mut child = root * 2 + 1;
            work.add(1)?;
            if child + 1 < end && cmp(a[child], a[child + 1]).is_lt() {
                child += 1;
            }
            if !cmp(a[root], a[child]).is_lt() {
                break;
            }
            a.swap(root, child);
            root = child;
        }
        Ok(())
    }
    for root in (0..a.len() / 2).rev() {
        sift(a, root, a.len(), cmp, work)?;
    }
    for end in (1..a.len()).rev() {
        a.swap(0, end);
        sift(a, 0, end, cmp, work)?;
    }
    Ok(())
}
fn box_distance(n: Node, p: V) -> f64 {
    (0..3)
        .map(|k| (n.lo[k] - p[k]).max(0.0).max(p[k] - n.hi[k]))
        .map(|d| d * d)
        .sum()
}
struct Hit {
    triangle: usize,
    distance: f64,
    weights: V,
    id: usize,
    normals: [V; 3],
}
// The minimum is on an edge or the orthogonal projection onto the face.
// Edge fallback also handles collinear and point triangles without division by zero.
fn closest(t: [V; 3], p: V) -> (f64, V) {
    let mut best = (f64::INFINITY, [1.0, 0.0, 0.0]);
    for (a, b) in [(0, 1), (1, 2), (2, 0)] {
        let edge = sub(t[b], t[a]);
        let len = dot(edge, edge);
        let s = if len > 0.0 {
            (dot(sub(p, t[a]), edge) / len).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let mut w = [0.0; 3];
        w[a] = 1.0 - s;
        w[b] = s;
        // Measure the residual directly, rather than reconstructing a world
        // point from rounded weights (which can erase small offsets on long
        // segments). Endpoints use their direct distance; interiors use the
        // perpendicular distance to the line, relative to the nearer endpoint.
        let d = if s == 0.0 {
            let delta = sub(p, t[a]);
            dot(delta, delta)
        } else if s == 1.0 {
            let delta = sub(p, t[b]);
            dot(delta, delta)
        } else {
            let offset = sub(p, if s <= 0.5 { t[a] } else { t[b] });
            let perpendicular = cross(offset, edge);
            dot(perpendicular, perpendicular) / len
        };
        if d < best.0 {
            best = (d, w);
        }
    }
    let ab = sub(t[1], t[0]);
    let ac = sub(t[2], t[0]);
    let n = cross(ab, ac);
    let nn = dot(n, n);
    if nn > 0.0 {
        let ap = sub(p, t[0]);
        let v = dot(cross(ap, ac), n) / nn;
        let w = dot(cross(ab, ap), n) / nn;
        if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
            let h = dot(ap, n);
            let d = h * h / nn;
            if d < best.0 {
                best = (d, [1.0 - v - w, v, w]);
            }
        }
    }
    best
}
// The face candidate is also tested by `closest`. A successful candidate is
// sufficient to bound this sample below the running maximum; a failed witness
// leaves the exact tree query unchanged. Degenerate/outside faces use edges.
fn witness(t: [V; 3], p: V, cutoff: f64) -> Option<(f64, V)> {
    let ab = sub(t[1], t[0]);
    let ac = sub(t[2], t[0]);
    let n = cross(ab, ac);
    let nn = dot(n, n);
    if nn > 0.0 {
        let ap = sub(p, t[0]);
        let v = dot(cross(ap, ac), n) / nn;
        let w = dot(cross(ab, ap), n) / nn;
        if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
            let h = dot(ap, n);
            let d = h * h / nn;
            return if d <= cutoff {
                Some((d, [1.0 - v - w, v, w]))
            } else {
                None
            };
        }
    }
    let (d, weights) = closest(t, p);
    if d <= cutoff {
        Some((d, weights))
    } else {
        None
    }
}
#[derive(Default)]
struct Stats {
    witness: Option<usize>,
    count: u64,
    max: f64,
    sum: f64,
    angle_max: f64,
    angle_sum: f64,
}
impl Stats {
    fn distance(&self) -> LodDistance {
        LodDistance {
            max: libm::sqrt(self.max),
            rms: libm::sqrt(self.sum / self.count as f64),
            samples: self.count,
        }
    }
    fn sample<const MAX_ONLY: bool, const NORMALS: usize>(
        &mut self,
        t: Triangle<NORMALS>,
        weights: V,
        tree: &Tree<NORMALS>,
        normals: bool,
        work: &mut Work,
    ) -> Result<(), Error> {
        work.add(1)?;
        let p = blend(t.p, weights);
        let mut hit = Hit {
            triangle: usize::MAX,
            distance: f64::INFINITY,
            weights: [0.0; 3],
            id: usize::MAX,
            normals: [[0.0; 3]; 3],
        };
        // Nearby samples often share a useful target triangle. Its distance is
        // an upper bound, so a successful witness needs no BVH traversal.
        if MAX_ONLY {
            if let Some(i) = self.witness {
                work.add(1)?;
                let t = tree.triangles[i];
                if let Some((distance, weights)) = witness(t.p, p, self.max) {
                    hit = Hit {
                        triangle: i,
                        distance,
                        weights,
                        id: t.id,
                        normals: t.normals(),
                    };
                }
            }
        }
        tree.query::<MAX_ONLY>(0, p, &mut hit, work, self.max)?;
        if MAX_ONLY && hit.triangle != usize::MAX {
            self.witness = Some(hit.triangle);
        }
        self.count = self.count.checked_add(1).ok_or(Error::SizeOverflow)?;
        self.max = self.max.max(hit.distance);
        if !MAX_ONLY {
            self.sum += hit.distance;
        }
        if !MAX_ONLY && normals {
            let a = unit(blend(t.normals(), weights)).map_err(|_| Error::NumericalFailure)?;
            let b = unit(blend(hit.normals, hit.weights)).map_err(|_| Error::NumericalFailure)?;
            // atan2 remains accurate near zero and pi, unlike acos(dot).
            let angle = libm::atan2(
                libm::sqrt(dot(cross(a, b), cross(a, b))),
                dot(a, b).clamp(-1.0, 1.0),
            );
            self.angle_max = self.angle_max.max(angle);
            self.angle_sum += angle;
        }
        Ok(())
    }
}
fn direction<I: crate::Index, const MAX_ONLY: bool, const NORMALS: usize>(
    source: LodQualityMeshGeneric<'_, I>,
    target: LodQualityMeshGeneric<'_, I>,
    options: &LodQualityOptions,
    budget: &mut Budget<'_>,
    work: &mut Work,
    initial_max: f64,
) -> Result<Stats, Error> {
    let tree = Tree::<NORMALS>::build(target, budget, work)?;
    // Always release local storage accounting, including after a failed query.
    let mut corners = Vec::<u64>::new();
    let result = (|| {
        if MAX_ONLY {
            let words = source.positions.len().div_ceil(64);
            work.add(words)?;
            corners = budget.reserved(words)?;
            corners.resize(words, 0);
        }
        let mut stats = Stats {
            max: initial_max,
            ..Stats::default()
        };
        for i in 0..source.indices.len() / 3 {
            work.add(1)?;
            let t = Triangle::<NORMALS>::read(source, i)?;
            let n = cross(sub(t.p[1], t.p[0]), sub(t.p[2], t.p[0]));
            let count = libm::ceil(0.5 * libm::sqrt(dot(n, n)) * options.samples_per_unit_area);
            if !count.is_finite() || count > f64::from(u32::MAX) {
                return Err(Error::SizeOverflow);
            }
            // Finite, nonnegative and checked <= u32::MAX: conversion cannot wrap.
            let count = count as u32;
            for (corner, w) in [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
                .into_iter()
                .enumerate()
            {
                let vertex = source.indices[i * 3 + corner].to_u32() as usize;
                let mask = 1u64 << (vertex % 64);
                if MAX_ONLY && corners[vertex / 64] & mask != 0 {
                    // This corner is already bounded by an earlier maximum.
                    // Keep the sample visit/count; its distance cannot raise max.
                    work.add(1)?;
                    stats.count = stats.count.checked_add(1).ok_or(Error::SizeOverflow)?;
                } else {
                    stats.sample::<MAX_ONLY, NORMALS>(
                        t,
                        w,
                        &tree,
                        source.normals.is_some(),
                        work,
                    )?;
                    if MAX_ONLY {
                        corners[vertex / 64] |= mask;
                    }
                }
            }
            for j in 0..count {
                let r = libm::sqrt((f64::from(j) + 0.5) / f64::from(count));
                let t2 = (f64::from(j.reverse_bits()) + 0.5) / 4294967296.0;
                stats.sample::<MAX_ONLY, NORMALS>(
                    t,
                    [1.0 - r, r * (1.0 - t2), r * t2],
                    &tree,
                    source.normals.is_some(),
                    work,
                )?;
            }
        }
        Ok(stats)
    })();
    if MAX_ONLY {
        budget.release(corners);
    }
    budget.release(tree.nodes);
    budget.release(tree.triangles);
    result
}

/// The source-compatible u32 entry point; see [`measure_lod_quality_generic`] for selectable index widths.
#[inline]
pub fn measure_lod_quality(
    source: LodQualityMesh<'_>,
    level: LodQualityMesh<'_>,
    options: &LodQualityOptions,
    workspace: &mut Workspace,
) -> Result<LodQuality, Error> {
    measure_lod_quality_generic::<u32>(source.into(), level.into(), options, workspace)
}

/// The source-compatible u32 entry point; see [`measure_lod_chain_quality_generic`] for selectable index widths.
#[inline]
pub fn measure_lod_chain_quality(
    source: LodQualityMesh<'_>,
    chain: &LodChain,
    options: &LodQualityOptions,
    camera: Option<LodCamera>,
    workspace: &mut Workspace,
) -> Result<Vec<LodQualityLevel>, Error> {
    let source: LodQualityMeshGeneric<'_, u32> = source.into();
    let mut work = workspace.begin();
    let result = (|| {
        validate_options(options)?;
        if let Some(camera) = camera {
            camera.switch_distance(0.0)?;
        }
        work.add(chain.levels.len())?;
        let mut budget = Budget::new(workspace);
        let mut out = budget.reserved(chain.levels.len())?;
        for level in &chain.levels {
            let mesh = LodQualityMeshGeneric {
                indices: &level.indices,
                ..source
            };
            let quality = measure(source, mesh, options, &mut budget, &mut work)?;
            out.push(LodQualityLevel {
                quality,
                switch_distance: camera
                    .map(|c| c.switch_distance(quality.symmetric.max))
                    .transpose()?,
            });
        }
        Ok(out)
    })();
    workspace.finish(&work);
    result
}

impl<'a> LodQualityMesh<'a> {
    /// Source-compatible u32 operation.
    #[inline]
    pub fn new(indices: &'a [u32], positions: Positions<'a>) -> Self {
        LodQualityMeshGeneric::new_generic(indices, positions).into()
    }
}

/// The source-compatible u32 form of [`LodQualityMeshGeneric`].
/// A borrowed triangle surface. Only referenced vertices are measured/validated.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodQualityMesh<'a> {
    /// Triangle-list indices; at least one triangle is required.
    pub indices: &'a [u32],
    /// Positions in the same world units as the reported distances.
    pub positions: Positions<'a>,
    /// Optional per-vertex normals, normalized before interpolation.
    pub normals: Option<&'a [[f32; 3]]>,
}
impl<'a> From<LodQualityMeshGeneric<'a, u32>> for LodQualityMesh<'a> {
    #[inline]
    fn from(value: LodQualityMeshGeneric<'a, u32>) -> Self {
        Self {
            indices: value.indices,
            positions: value.positions,
            normals: value.normals,
        }
    }
}

impl<'a> From<LodQualityMesh<'a>> for LodQualityMeshGeneric<'a, u32> {
    #[inline]
    fn from(value: LodQualityMesh<'a>) -> Self {
        Self {
            indices: value.indices,
            positions: value.positions,
            normals: value.normals,
        }
    }
}

#[cfg(all(test, feature = "clusterlod"))]
mod maximum_tests {
    use super::*;

    #[test]
    fn maximum_only_matches_uneven_surfaces() {
        let mut state = 17u32;
        let mut random = || {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 8) as f32 / 16777216.0
        };
        let indices: Vec<u32> = (0..36).collect();
        for scale in [1e-10, 1., 1e10] {
            for _ in 0..16 {
                let source: Vec<[f32; 3]> = (0..36)
                    .map(|_| core::array::from_fn(|_| random() * scale))
                    .collect();
                let target: Vec<[f32; 3]> = source
                    .iter()
                    .map(|p| core::array::from_fn(|k| p[k] + (random() - 0.5) * scale))
                    .collect();
                let options = LodQualityOptions {
                    samples_per_unit_area: 4. / f64::from(scale).powi(2),
                };
                let a = LodQualityMesh::new(&indices, Positions::from_packed(&source));
                let b = LodQualityMesh::new(&indices, Positions::from_packed(&target));
                let full = measure_lod_quality(a, b, &options, &mut Workspace::default())
                    .expect("full measurement");
                let maximum = measure_max(a, b, &options, &mut Workspace::default())
                    .expect("maximum measurement");
                assert_eq!(maximum.to_bits(), full.symmetric.max.to_bits());
            }
        }
    }

    #[test]
    fn maximum_only_matches_full_queries() {
        // Overlapping faces, repeated corners and degenerate segments exercise
        // witness exits and exact queries that must raise the running maximum.
        let indices = [0, 1, 2, 2, 1, 0, 0, 0, 3, 3, 4, 5];
        let source = [
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 2.],
            [1., 0., 2.],
            [0., 1., 2.],
        ];
        for offset in [0., 1e-12, 0.125, 4.] {
            let target = source.map(|p| [p[0], p[1], p[2] + offset]);
            for density in [0., 1., 100.] {
                let options = LodQualityOptions {
                    samples_per_unit_area: density,
                };
                let a = LodQualityMesh::new(&indices, Positions::from_packed(&source));
                let b = LodQualityMesh::new(&indices, Positions::from_packed(&target));
                let full = measure_lod_quality(a, b, &options, &mut Workspace::default())
                    .expect("full measurement");
                let maximum = measure_max(a, b, &options, &mut Workspace::default())
                    .expect("maximum measurement");
                assert_eq!(maximum.to_bits(), full.symmetric.max.to_bits());
            }
        }
    }
}
