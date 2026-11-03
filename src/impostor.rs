//! Octahedral impostor baking. An extension, not in C++ meshoptimizer.
//!
//! A distant object is replaced by a camera-facing card textured from an atlas
//! of orthographic views captured on an octahedral grid of directions, and the
//! runtime blends the three views nearest to the camera direction. The layout
//! and the blend follow Ryan Brucks' octahedral impostors
//! (<https://shaderbits.com/blog/octahedral-impostors>, 2018): frames sit on
//! the vertices of a square grid over the octahedral or hemi-octahedral map,
//! and a direction is blended from the three corners of the grid triangle it
//! falls in. The octahedral map is the one surveyed by Cigolle et al.,
//! "A Survey of Efficient Representations for Independent Unit Vectors",
//! JCGT 3(2), 2014 (<https://jcgt.org/published/0003/02/01/>).
//!
//! Baking is a scalar software rasterizer with exact integer edge functions
//! and supersampling. The upstream overdraw rasterizer in `raster.rs` keeps
//! upstream's fixed 256-pixel viewport and per-pixel record for byte parity
//! with C++, so it is not reused here.
use crate::budget::Budget;
use crate::workspace::Work;
use crate::{Error, LodCamera, Positions, Workspace};
use alloc::vec::Vec;
use core::fmt;

/// Largest views per grid axis.
const MAX_VIEWS_PER_AXIS: u32 = 64;
/// Largest view resolution in texels.
const MAX_VIEW_RESOLUTION: u32 = 4096;
/// Largest supersampling factor per axis.
const MAX_SAMPLES_PER_AXIS: u32 = 8;
/// Largest dilation pass count.
const MAX_DILATION: u32 = 64;
/// Subsample positions are snapped to 1/256 of a sample.
const FIX: i64 = 256;
/// Largest snapped coordinate magnitude; keeps edge products inside i64.
const MAX_FIX: f64 = 268_435_456.0;
/// Barycentric subdivisions per grid triangle for the view error estimate.
const CHORD_STEPS: u32 = 4;
/// Normal and depth of a texel that no surface covers: zero normal, far depth.
const EMPTY_NORMAL_DEPTH: [u8; 4] = [128, 128, 128, 255];

type V = [f64; 3];

/// Direction set covered by the view grid. Not in C++ meshoptimizer.
///
/// Directions point from the object toward the viewer, in the mesh frame with
/// +Y up. The octahedral point `o = [u, v]` lies in `[-1, 1]^2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImpostorLayout {
    /// Upper hemisphere (`y >= 0`), the usual choice for trees and props seen
    /// from above the horizon. With `s = |x| + y + |z|`, `a = x / s` and
    /// `b = z / s`, the point is `[a + b, a - b]`: `[0, 0]` is straight up and
    /// the square's border is the horizon. Directions below the horizon are
    /// projected onto it; straight down maps to the +X horizon. Default.
    #[default]
    Hemisphere,
    /// Full sphere. With `s = |x| + |y| + |z|` the point is `[x / s, z / s]`
    /// for `y >= 0`; below the horizon each coordinate folds outward as
    /// `[(1 - |b|) sign(a), (1 - |a|) sign(b)]`, with `sign(0) = 1`. The inner
    /// diamond is the upper hemisphere, the corners are straight down.
    Sphere,
}

impl ImpostorLayout {
    /// The octahedral point of a direction. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for a zero or non-finite direction.
    pub fn encode(self, direction: [f32; 3]) -> Result<[f32; 2], Error> {
        let o = encode(self, widen(direction))?;
        Ok([o[0] as f32, o[1] as f32])
    }

    /// The unit direction of an octahedral point. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for a point outside `[-1, 1]^2` or a non-finite one.
    pub fn decode(self, point: [f32; 2]) -> Result<[f32; 3], Error> {
        let o = [f64::from(point[0]), f64::from(point[1])];
        if !o.iter().all(|x| (-1.0..=1.0).contains(x)) {
            return Err(Error::InvalidParameter);
        }
        Ok(narrow(decode(self, o)))
    }
}

/// The square grid of view directions. Not in C++ meshoptimizer.
///
/// View `(i, j)` for `i, j < views_per_axis` has index
/// `j * views_per_axis + i` and octahedral point
/// `[2 i / (N - 1) - 1, 2 j / (N - 1) - 1]` with `N = views_per_axis`, so the
/// border of the octahedral square carries views. Its atlas tile has origin
/// `(i * view_resolution, j * view_resolution)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ImpostorGrid {
    /// Direction set (default [`ImpostorLayout::Hemisphere`]).
    pub layout: ImpostorLayout,
    /// Views per axis, 2 to 64 (default 8, 64 views).
    pub views_per_axis: u32,
}

impl Default for ImpostorGrid {
    fn default() -> Self {
        Self {
            layout: ImpostorLayout::Hemisphere,
            views_per_axis: 8,
        }
    }
}

impl ImpostorGrid {
    /// A validated grid. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` unless `2 <= views_per_axis <= 64`.
    pub fn new(layout: ImpostorLayout, views_per_axis: u32) -> Result<Self, Error> {
        let grid = Self {
            layout,
            views_per_axis,
        };
        grid.check()?;
        Ok(grid)
    }

    fn check(self) -> Result<u32, Error> {
        if (2..=MAX_VIEWS_PER_AXIS).contains(&self.views_per_axis) {
            Ok(self.views_per_axis)
        } else {
            Err(Error::InvalidParameter)
        }
    }

    /// Number of views, `views_per_axis^2`.
    ///
    /// # Errors
    /// `InvalidParameter` for an invalid grid.
    pub fn view_count(self) -> Result<u32, Error> {
        let n = self.check()?;
        n.checked_mul(n).ok_or(Error::SizeOverflow)
    }

    /// The octahedral point of a view. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for an invalid grid or view index.
    pub fn view_point(self, view: u32) -> Result<[f32; 2], Error> {
        let o = self.point(view)?;
        Ok([o[0] as f32, o[1] as f32])
    }

    /// The capture frame of a view. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for an invalid grid or view index.
    pub fn view_frame(self, view: u32) -> Result<ImpostorFrame, Error> {
        Ok(Frame::new(decode(self.layout, self.point(view)?)).narrow())
    }

    /// The three views to blend for a direction toward the viewer, and their
    /// weights. Not in C++ meshoptimizer.
    ///
    /// With `g = (o + 1) / 2 * (N - 1)` clamped to `[0, N - 1]`, cell
    /// `c = min(floor(g), N - 2)` and `f = g - c`, the views are `c`,
    /// `c + (1, 0)` and `c + (1, 1)` with weights `1 - f.x`, `f.x - f.y`,
    /// `f.y` when `f.x >= f.y`, and `c`, `c + (0, 1)`, `c + (1, 1)` with
    /// weights `1 - f.y`, `f.y - f.x`, `f.x` otherwise: the barycentric
    /// coordinates in the grid triangle split along the cell's diagonal.
    /// Weights are nonnegative and sum to one up to rounding. A direction on a
    /// view's frame returns that view with weight one.
    ///
    /// # Errors
    /// `InvalidParameter` for an invalid grid or a zero or non-finite direction.
    pub fn blend(self, direction: [f32; 3]) -> Result<ImpostorBlend, Error> {
        let (views, weights) = self.blend_point(encode(self.layout, widen(direction))?)?;
        Ok(ImpostorBlend {
            views,
            weights: [weights[0] as f32, weights[1] as f32, weights[2] as f32],
        })
    }

    fn point(self, view: u32) -> Result<[f64; 2], Error> {
        let n = self.check()?;
        if view >= self.view_count()? {
            return Err(Error::InvalidParameter);
        }
        let m = f64::from(n - 1);
        let (i, j) = (f64::from(view % n), f64::from(view / n));
        Ok([(2.0 * i - m) / m, (2.0 * j - m) / m])
    }

    fn blend_point(self, o: [f64; 2]) -> Result<([u32; 3], [f64; 3]), Error> {
        let n = self.check()?;
        let m = f64::from(n - 1);
        let g = o.map(|x| ((x + 1.0) * 0.5 * m).clamp(0.0, m));
        let c = g.map(|x| libm::floor(x).min(m - 1.0));
        let (fx, fy) = (g[0] - c[0], g[1] - c[1]);
        // c is an integer in 0..=n-2 by the clamps above.
        let (i, j) = (c[0] as u32, c[1] as u32);
        let at = |di: u32, dj: u32| (j + dj) * n + i + di;
        Ok(if fx >= fy {
            ([at(0, 0), at(1, 0), at(1, 1)], [1.0 - fx, fx - fy, fy])
        } else {
            ([at(0, 0), at(0, 1), at(1, 1)], [1.0 - fy, fy - fx, fx])
        })
    }
}

/// Orthographic capture frame of one view. Not in C++ meshoptimizer.
///
/// `direction` points from the object toward the viewer, `right = normalize(Y
/// x direction)` (or +X when the direction is vertical) and
/// `up = direction x right`, so `right x up = direction`. With bounds center
/// `c` and radius `r`, a point `p` lands at tile coordinates
/// `x = ((p - c) . right / r + 1) / 2` and `y = (1 - (p - c) . up / r) / 2` in
/// `[0, 1]` (row 0 at the top), and its stored depth is
/// `(1 - (p - c) . direction / r) / 2`: 0 at the bounds' near side, 1 at the
/// far side.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct ImpostorFrame {
    /// Unit direction from the object toward the viewer.
    pub direction: [f32; 3],
    /// Unit tile +x axis.
    pub right: [f32; 3],
    /// Unit tile up axis (tile rows grow downward).
    pub up: [f32; 3],
}

impl ImpostorFrame {
    /// The frame used for a direction toward the viewer. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for a zero or non-finite direction.
    pub fn from_direction(direction: [f32; 3]) -> Result<Self, Error> {
        Ok(Frame::new(unit(widen(direction))?).narrow())
    }
}

/// Views and weights from [`ImpostorGrid::blend`]. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ImpostorBlend {
    /// View indices; one view may repeat where the grid folds.
    pub views: [u32; 3],
    /// Nonnegative weights summing to one up to rounding.
    pub weights: [f32; 3],
}

/// Bounding sphere shared by every view. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ImpostorBounds {
    /// Sphere center in mesh units.
    pub center: [f32; 3],
    /// Sphere radius; zero only in the atlas of an empty or degenerate mesh.
    pub radius: f32,
}

impl ImpostorBounds {
    /// A validated sphere. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for a non-finite center or a radius that is not
    /// finite and positive.
    pub fn new(center: [f32; 3], radius: f32) -> Result<Self, Error> {
        let bounds = Self { center, radius };
        bounds.check()?;
        Ok(bounds)
    }

    /// The sphere through the corners of a box. Not in C++ meshoptimizer.
    ///
    /// # Errors
    /// `InvalidParameter` for non-finite corners, `min > max` on any axis or a
    /// box with zero extent.
    pub fn from_box(min: [f32; 3], max: [f32; 3]) -> Result<Self, Error> {
        let (lo, hi) = (widen(min), widen(max));
        if !lo.iter().chain(&hi).all(|x| x.is_finite()) || (0..3).any(|k| lo[k] > hi[k]) {
            return Err(Error::InvalidParameter);
        }
        let center = [0, 1, 2].map(|k| (lo[k] + hi[k]) * 0.5);
        let half = [0, 1, 2].map(|k| hi[k] - center[k]);
        Self::new(narrow(center), round_up(length(half)))
    }

    fn check(self) -> Result<(), Error> {
        if self.center.iter().all(|x| x.is_finite()) && self.radius.is_finite() && self.radius > 0.0
        {
            Ok(())
        } else {
            Err(Error::InvalidParameter)
        }
    }
}

/// Surface colour and alpha source. Not in C++ meshoptimizer.
///
/// Colour and alpha are in `[0, 1]`; colour is clamped and stored as given
/// (the baker does not convert colour spaces). Alpha only decides whether a
/// fragment passes [`ImpostorOptions::alpha_cutoff`].
#[derive(Clone, Copy, Default)]
#[non_exhaustive]
pub enum ImpostorAlbedo<'a> {
    /// White and opaque. Default.
    #[default]
    White,
    /// One RGBA value per vertex, interpolated.
    VertexColors(&'a [[f32; 4]]),
    /// A texture lookup per covered sample: `sampler(triangle, uv)` returns RGBA
    /// for triangle `triangle` (index into the triangle list) at the
    /// interpolated `uv`. The sampler must be deterministic for a
    /// deterministic bake.
    Texture {
        /// One UV pair per vertex, interpolated.
        uvs: &'a [[f32; 2]],
        /// Texture lookup.
        sampler: &'a dyn Fn(usize, [f32; 2]) -> [f32; 4],
    },
}

impl fmt::Debug for ImpostorAlbedo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => f.write_str("White"),
            Self::VertexColors(c) => f.debug_tuple("VertexColors").field(&c.len()).finish(),
            Self::Texture { uvs, .. } => f
                .debug_struct("Texture")
                .field("uvs", &uvs.len())
                .finish_non_exhaustive(),
        }
    }
}

/// Settings for [`bake_impostor`]. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct ImpostorOptions {
    /// View directions (default hemisphere, 8 x 8 views).
    pub grid: ImpostorGrid,
    /// Texels per view side, 1 to 4096 (default 128).
    pub view_resolution: u32,
    /// Supersamples per texel axis, 1 to 8 (default 2, four samples).
    pub samples_per_axis: u32,
    /// Fragments with source alpha below this value are discarded before the
    /// depth test (default 0.5), as glTF `MASK` materials do. Within `[0, 1]`.
    pub alpha_cutoff: f32,
    /// Conservative alpha: a texel whose covered sample fraction reaches this
    /// value is fully opaque, and below it alpha is `fraction / threshold`
    /// (default 0.5). Within `(0, 1]`; 1 stores plain coverage.
    pub coverage_threshold: f32,
    /// Render back faces with reversed normals (default true). Front faces are
    /// counter-clockwise seen from the viewer. When false, back faces are culled.
    pub double_sided: bool,
    /// Passes that copy colour, normal and depth into empty texels from their
    /// filled 4-neighbours in the same view, so filtering does not pull in
    /// black (default 4, at most 64). Alpha stays zero.
    pub dilation: u32,
    /// Fixed bounding sphere for every view (default `None`: the sphere around
    /// the referenced vertices' box center). Geometry outside it is clipped
    /// to the tile; a vertex projecting more than `2^20` samples from the
    /// tile corner is rejected.
    pub bounds: Option<ImpostorBounds>,
}

impl Default for ImpostorOptions {
    fn default() -> Self {
        Self {
            grid: ImpostorGrid::default(),
            view_resolution: 128,
            samples_per_axis: 2,
            alpha_cutoff: 0.5,
            coverage_threshold: 0.5,
            double_sided: true,
            dilation: 4,
            bounds: None,
        }
    }
}

/// Counts from [`bake_impostor`]. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ImpostorReport {
    /// Texels with at least one covered sample, over all views.
    pub covered_texels: u64,
    /// Covered texels with alpha 255.
    pub opaque_texels: u64,
    /// Views without covered texels.
    pub empty_views: u32,
    /// Texture sampler calls.
    pub sampler_calls: u64,
}

/// Baked atlas. Not in C++ meshoptimizer.
///
/// Both planes are `size * size` RGBA8 texels, row-major with row 0 at the
/// top. View `j * N + i` occupies the tile at
/// `(i * view_resolution, j * view_resolution)`; see [`ImpostorGrid`] and
/// [`ImpostorFrame`] for the mapping.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ImpostorAtlas {
    /// The view grid.
    pub grid: ImpostorGrid,
    /// Texels per view side.
    pub view_resolution: u32,
    /// Atlas width and height, `views_per_axis * view_resolution`.
    pub size: u32,
    /// The sphere every view frames; tile width is `2 * radius`.
    pub bounds: ImpostorBounds,
    /// Averaged colour of covered samples and conservative coverage alpha.
    /// Empty texels have alpha 0.
    pub albedo_alpha: Vec<[u8; 4]>,
    /// Mesh-frame unit normal as `n * 0.5 + 0.5` in RGB and depth in A
    /// (nearest covered sample, see [`ImpostorFrame`]). Empty texels without
    /// dilation hold `[128, 128, 128, 255]`.
    pub normal_depth: Vec<[u8; 4]>,
    /// Counts.
    pub report: ImpostorReport,
}

impl ImpostorAtlas {
    /// Top-left texel of a view's tile, or `None` for an invalid view.
    pub fn tile_origin(&self, view: u32) -> Option<[u32; 2]> {
        let n = self.grid.check().ok()?;
        if view >= self.grid.view_count().ok()? {
            return None;
        }
        Some([
            (view % n).checked_mul(self.view_resolution)?,
            (view / n).checked_mul(self.view_resolution)?,
        ])
    }
}

/// Bake an octahedral impostor atlas. Not in C++ meshoptimizer.
///
/// Every view is an orthographic capture of the bounding sphere along its
/// [`ImpostorFrame`], rasterized at `view_resolution * samples_per_axis`
/// samples per side with sample centers at half-integer positions, a
/// top-left fill rule on positions snapped to 1/256 sample, a strict
/// nearer-wins depth test (ties keep the earlier triangle) and the source alpha
/// test. A texel averages the colour of its covered samples, sums and
/// renormalizes their normals and keeps the nearest depth; its alpha is the
/// covered fraction divided by `coverage_threshold`, clamped to one. Normals
/// come from `normals` (interpolated and renormalized) or the face, and are
/// reversed on back faces. Only referenced vertices are read, validated and
/// bounded. Triangles with zero area are skipped, so an empty index list or a
/// mesh without area gives a fully empty atlas.
///
/// Work is about `views * (vertices + triangles + covered bounding-box
/// samples + samples + dilation * texels)` and storage is two atlas planes
/// plus one view of 32-byte samples; both are charged to the workspace. The
/// result is bit-identical on every platform for a deterministic sampler.
///
/// ```
/// use meshoptimizer_rs::{bake_impostor, ImpostorAlbedo, ImpostorGrid, ImpostorLayout,
///     ImpostorOptions, Positions, Workspace};
/// // A unit quad facing +Z.
/// let p = [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [1.0, 1.0, 0.0], [-1.0, 1.0, 0.0]];
/// let mut options = ImpostorOptions::default();
/// options.grid = ImpostorGrid::new(ImpostorLayout::Sphere, 3)?;
/// options.view_resolution = 16;
/// let atlas = bake_impostor(&[0, 1, 2, 0, 2, 3], Positions::from_packed(&p), None,
///     ImpostorAlbedo::White, options, &mut Workspace::default())?;
/// assert_eq!((atlas.size, atlas.albedo_alpha.len()), (48, 48 * 48));
/// // +Z is the octahedral point [0, 1]: view (i, j) = (1, 2), index 7.
/// let blend = atlas.grid.blend([0.0, 0.0, 1.0])?;
/// let weight: f32 = (0..3).filter(|&k| blend.views[k] == 7).map(|k| blend.weights[k]).sum();
/// assert_eq!(weight, 1.0);
/// let [x, y] = atlas.tile_origin(7).unwrap();
/// let center = ((y + 8) * atlas.size + x + 8) as usize;
/// assert_eq!(atlas.albedo_alpha[center], [255, 255, 255, 255]);
/// // The view from straight above (index 4) sees the quad edge-on.
/// assert!(atlas.report.empty_views > 0);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// `InvalidTopology` for an incomplete triangle, `IndexOutOfBounds` for an
/// invalid index, `InvalidLayout` when `normals`, vertex colours or UVs do not
/// have one entry per position, `InvalidParameter` for invalid options or a
/// vertex too far outside fixed bounds, `NumericalFailure` for a non-finite
/// referenced input or sampler result, and the usual `SizeOverflow`,
/// `LimitExceeded` or `AllocationFailed`. No partial result is returned.
#[inline]
pub fn bake_impostor_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    normals: Option<&[[f32; 3]]>,
    albedo: ImpostorAlbedo<'_>,
    options: ImpostorOptions,
    workspace: &mut Workspace,
) -> Result<ImpostorAtlas, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        bake_u32(indices, positions, normals, albedo, options, workspace)
    })
}

/// The u32 entry point; see [`bake_impostor_generic`] for selectable index widths.
#[inline]
pub fn bake_impostor(
    indices: &[u32],
    positions: Positions<'_>,
    normals: Option<&[[f32; 3]]>,
    albedo: ImpostorAlbedo<'_>,
    options: ImpostorOptions,
    workspace: &mut Workspace,
) -> Result<ImpostorAtlas, Error> {
    bake_u32(indices, positions, normals, albedo, options, workspace)
}

/// Impostor switch distance and its error terms. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ImpostorSwitch {
    /// Camera-to-center distance beyond which both errors project below the
    /// camera's pixel tolerance: [`LodCamera::switch_distance`] of the larger
    /// error, plus the radius.
    pub distance: f64,
    /// One texel's world size, `2 * radius / view_resolution`.
    pub texel_error: f64,
    /// `radius` times the largest blend-weighted chord `sum w_k |d - d_k|`
    /// between a direction and its blended view directions, sampled on a
    /// barycentric lattice of every grid triangle: the screen displacement
    /// bound of a surface point on the bounding sphere when the runtime blends
    /// views without depth-based parallax correction.
    pub view_error: f64,
}

/// Recommend the distance from which an impostor may replace the mesh.
/// Not in C++ meshoptimizer.
///
/// Both terms are world-space deviations at the bounding sphere; the switch
/// uses the larger. A runtime that corrects parallax from the stored depth
/// sees less than `view_error`; it can call
/// [`LodCamera::switch_distance`] with `texel_error` and add the radius.
/// Hemisphere grids cover views from above the horizon only.
///
/// # Errors
/// `InvalidParameter` for invalid bounds, grid or a view resolution outside
/// 1 to 4096, and the errors of [`LodCamera::switch_distance`].
pub fn impostor_switch_distance(
    bounds: ImpostorBounds,
    grid: ImpostorGrid,
    view_resolution: u32,
    camera: LodCamera,
) -> Result<ImpostorSwitch, Error> {
    bounds.check()?;
    grid.check()?;
    if !(1..=MAX_VIEW_RESOLUTION).contains(&view_resolution) {
        return Err(Error::InvalidParameter);
    }
    let radius = f64::from(bounds.radius);
    let texel_error = 2.0 * radius / f64::from(view_resolution);
    let view_error = radius * max_view_chord(grid)?;
    let distance = camera.switch_distance(texel_error.max(view_error))? + radius;
    if !distance.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok(ImpostorSwitch {
        distance,
        texel_error,
        view_error,
    })
}

// ---------------------------------------------------------------------------
// Direction math, all in f64.

#[inline]
fn widen(v: [f32; 3]) -> V {
    v.map(f64::from)
}

#[inline]
fn narrow(v: V) -> [f32; 3] {
    v.map(|x| x as f32)
}

#[inline]
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn length(v: V) -> f64 {
    libm::sqrt(dot(v, v))
}

fn unit(v: V) -> Result<V, Error> {
    let l = length(v);
    if !l.is_finite() || l == 0.0 {
        return Err(Error::InvalidParameter);
    }
    Ok(v.map(|x| x / l))
}

/// Smallest f32 not below `x` (for finite nonnegative `x`).
fn round_up(x: f64) -> f32 {
    let r = x as f32;
    if f64::from(r) < x {
        r.next_up()
    } else {
        r
    }
}

#[inline]
fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}

fn encode(layout: ImpostorLayout, d: V) -> Result<[f64; 2], Error> {
    if !d.iter().all(|x| x.is_finite()) || d == [0.0; 3] {
        return Err(Error::InvalidParameter);
    }
    // Scale first so the L1 norm cannot overflow.
    let m = d[0].abs().max(d[1].abs()).max(d[2].abs());
    let d = d.map(|x| x / m);
    Ok(match layout {
        ImpostorLayout::Hemisphere => {
            let y = d[1].max(0.0);
            let s = d[0].abs() + y + d[2].abs();
            if s == 0.0 {
                [1.0, 1.0]
            } else {
                let (a, b) = (d[0] / s, d[2] / s);
                [a + b, a - b]
            }
        }
        ImpostorLayout::Sphere => {
            let s = d[0].abs() + d[1].abs() + d[2].abs();
            let (a, b) = (d[0] / s, d[2] / s);
            if d[1] < 0.0 {
                [(1.0 - b.abs()) * sign(a), (1.0 - a.abs()) * sign(b)]
            } else {
                [a, b]
            }
        }
    })
}

fn decode(layout: ImpostorLayout, o: [f64; 2]) -> V {
    let (mut a, mut b) = match layout {
        ImpostorLayout::Hemisphere => ((o[0] + o[1]) * 0.5, (o[0] - o[1]) * 0.5),
        ImpostorLayout::Sphere => (o[0], o[1]),
    };
    let y = 1.0 - a.abs() - b.abs();
    if y < 0.0 {
        (a, b) = ((1.0 - b.abs()) * sign(a), (1.0 - a.abs()) * sign(b));
    }
    let v = [a, y, b];
    // |a| + |y| + |b| >= 1 on the square, so the length is at least 1/sqrt(3).
    let l = length(v);
    v.map(|x| x / l)
}

#[derive(Clone, Copy)]
struct Frame {
    direction: V,
    right: V,
    up: V,
}

impl Frame {
    fn new(direction: V) -> Self {
        let r = [direction[2], 0.0, -direction[0]];
        let l = length(r);
        let right = if l > 1e-9 {
            r.map(|x| x / l)
        } else {
            [1.0, 0.0, 0.0]
        };
        Self {
            direction,
            right,
            up: cross(direction, right),
        }
    }

    fn narrow(self) -> ImpostorFrame {
        ImpostorFrame {
            direction: narrow(self.direction),
            right: narrow(self.right),
            up: narrow(self.up),
        }
    }
}

fn max_view_chord(grid: ImpostorGrid) -> Result<f64, Error> {
    let n = grid.check()?;
    let m = f64::from(n - 1);
    let k = f64::from(CHORD_STEPS);
    let dir = |gx: f64, gy: f64| decode(grid.layout, [2.0 * gx / m - 1.0, 2.0 * gy / m - 1.0]);
    let mut worst = 0.0f64;
    for j in 0..n - 1 {
        for i in 0..n - 1 {
            let (x, y) = (f64::from(i), f64::from(j));
            let corners = [
                [[x, y], [x + 1.0, y], [x + 1.0, y + 1.0]],
                [[x, y], [x, y + 1.0], [x + 1.0, y + 1.0]],
            ];
            for tri in corners {
                let frames = tri.map(|c| dir(c[0], c[1]));
                for a in 0..=CHORD_STEPS {
                    for b in 0..=CHORD_STEPS - a {
                        let w = [
                            f64::from(CHORD_STEPS - a - b) / k,
                            f64::from(a) / k,
                            f64::from(b) / k,
                        ];
                        let gx = w[0] * tri[0][0] + w[1] * tri[1][0] + w[2] * tri[2][0];
                        let gy = w[0] * tri[0][1] + w[1] * tri[1][1] + w[2] * tri[2][1];
                        let d = dir(gx, gy);
                        let chord: f64 = (0..3).map(|t| w[t] * length(sub(d, frames[t]))).sum();
                        worst = worst.max(chord);
                    }
                }
            }
        }
    }
    Ok(worst)
}

// ---------------------------------------------------------------------------
// Baking.

#[derive(Clone, Copy)]
struct Sample {
    /// Normalized depth, `INFINITY` when empty.
    z: f64,
    rgb: [f32; 3],
    normal: [f32; 3],
}

const EMPTY_SAMPLE: Sample = Sample {
    z: f64::INFINITY,
    rgb: [0.0; 3],
    normal: [0.0; 3],
};

#[derive(Clone, Copy, Default)]
struct Projected {
    x: i64,
    y: i64,
    z: f64,
}

struct Bake<'a> {
    indices: &'a [u32],
    normals: Option<&'a [[f32; 3]]>,
    albedo: ImpostorAlbedo<'a>,
    cutoff: f64,
    double_sided: bool,
    /// Positions relative to the center, for referenced vertices.
    points: Vec<V>,
    /// Referenced vertices in ascending order.
    unique: Vec<u32>,
    radius: f64,
}

#[inline]
fn unorm8(x: f64) -> u8 {
    let c = if x > 0.0 { x.min(1.0) } else { 0.0 };
    // c is in [0, 1], so the result is in [0, 255.5).
    libm::floor(c * 255.0 + 0.5) as u8
}

/// Twice the signed area of `a, b, p` in snapped units; positive inside a
/// triangle with positive orientation.
#[inline(always)]
fn edge(a: Projected, b: Projected, px: i64, py: i64) -> i64 {
    (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

/// Whether the directed edge `a -> b` owns samples exactly on it. Opposite
/// directions of one edge decide differently, so shared edges are drawn once.
#[inline(always)]
fn owns(a: Projected, b: Projected) -> bool {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    dy > 0 || (dy == 0 && dx < 0)
}

fn bake_u32(
    indices: &[u32],
    positions: Positions<'_>,
    normals: Option<&[[f32; 3]]>,
    albedo: ImpostorAlbedo<'_>,
    options: ImpostorOptions,
    workspace: &mut Workspace,
) -> Result<ImpostorAtlas, Error> {
    let n = options.grid.check()?;
    if !(1..=MAX_VIEW_RESOLUTION).contains(&options.view_resolution)
        || !(1..=MAX_SAMPLES_PER_AXIS).contains(&options.samples_per_axis)
        || options.dilation > MAX_DILATION
        || !(0.0..=1.0).contains(&options.alpha_cutoff)
        || !(options.coverage_threshold > 0.0 && options.coverage_threshold <= 1.0)
    {
        return Err(Error::InvalidParameter);
    }
    if let Some(bounds) = options.bounds {
        bounds.check()?;
    }
    let vertex_count = positions.len();
    let per_vertex = |len: usize| {
        if len == vertex_count {
            Ok(())
        } else {
            Err(Error::InvalidLayout)
        }
    };
    if let Some(normals) = normals {
        per_vertex(normals.len())?;
    }
    match albedo {
        ImpostorAlbedo::White => {}
        ImpostorAlbedo::VertexColors(colors) => per_vertex(colors.len())?,
        ImpostorAlbedo::Texture { uvs, .. } => per_vertex(uvs.len())?,
    }
    let mut work = workspace.begin();
    let result = (|| {
        crate::workspace::topology(indices, vertex_count, &mut work)?;
        let mut budget = Budget::new(workspace);
        run(
            indices,
            positions,
            normals,
            albedo,
            options,
            n,
            &mut budget,
            &mut work,
        )
    })();
    workspace.finish(&work);
    result
}

#[allow(clippy::too_many_arguments)]
fn run(
    indices: &[u32],
    positions: Positions<'_>,
    normals: Option<&[[f32; 3]]>,
    albedo: ImpostorAlbedo<'_>,
    options: ImpostorOptions,
    n: u32,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<ImpostorAtlas, Error> {
    let res = usize::try_from(options.view_resolution).map_err(|_| Error::SizeOverflow)?;
    let spa = usize::try_from(options.samples_per_axis).map_err(|_| Error::SizeOverflow)?;
    let views = usize::try_from(n).map_err(|_| Error::SizeOverflow)?;
    let size = views.checked_mul(res).ok_or(Error::SizeOverflow)?;
    let size32 = u32::try_from(size).map_err(|_| Error::SizeOverflow)?;
    let texels = size.checked_mul(size).ok_or(Error::SizeOverflow)?;
    let side = res.checked_mul(spa).ok_or(Error::SizeOverflow)?;
    let samples = side.checked_mul(side).ok_or(Error::SizeOverflow)?;
    let tile = res.checked_mul(res).ok_or(Error::SizeOverflow)?;

    // Validate referenced inputs and find the box of referenced positions.
    let vertex_count = positions.len();
    let mut points = budget.filled(vertex_count, [0.0f64; 3])?;
    let mut referenced = budget.filled(vertex_count, false)?;
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    let mut unique_count = 0usize;
    work.add(indices.len())?;
    for &index in indices {
        let v = index as usize;
        if referenced[v] {
            continue;
        }
        referenced[v] = true;
        unique_count += 1;
        let p = widen(positions.get(v).ok_or(Error::IndexOutOfBounds)?);
        let finite = p.iter().all(|x| x.is_finite())
            && normals.is_none_or(|nm| nm[v].iter().all(|x| x.is_finite()))
            && match albedo {
                ImpostorAlbedo::White => true,
                ImpostorAlbedo::VertexColors(c) => c[v].iter().all(|x| x.is_finite()),
                ImpostorAlbedo::Texture { uvs, .. } => uvs[v].iter().all(|x| x.is_finite()),
            };
        if !finite {
            return Err(Error::NumericalFailure);
        }
        points[v] = p;
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let mut unique: Vec<u32> = budget.reserved(unique_count)?;
    work.add(vertex_count)?;
    for (v, &r) in referenced.iter().enumerate() {
        if r {
            // v < positions.len(), and topology() bounds the count by u32.
            unique.push(u32::try_from(v).map_err(|_| Error::SizeOverflow)?);
        }
    }
    budget.release(referenced);

    // Bounds: the caller's sphere, or the box center rounded to f32 and the
    // radius around that center rounded up, so the stored sphere contains
    // every referenced vertex.
    let bounds = if let Some(bounds) = options.bounds {
        bounds
    } else if indices.is_empty() {
        ImpostorBounds::default()
    } else {
        let center = widen(narrow([0, 1, 2].map(|k| (lo[k] + hi[k]) * 0.5)));
        let mut radius = 0.0f64;
        work.add(unique.len())?;
        for &v in &unique {
            radius = radius.max(length(sub(points[v as usize], center)));
        }
        if !radius.is_finite() {
            return Err(Error::NumericalFailure);
        }
        ImpostorBounds {
            center: narrow(center),
            radius: round_up(radius),
        }
    };
    let center = widen(bounds.center);
    work.add(unique.len())?;
    for &v in &unique {
        let p = &mut points[v as usize];
        *p = sub(*p, center);
    }

    let mut albedo_alpha = budget.filled(texels, [0u8; 4])?;
    let mut normal_depth = budget.filled(texels, EMPTY_NORMAL_DEPTH)?;
    let mut report = ImpostorReport::default();
    let bake = Bake {
        indices,
        normals,
        albedo,
        cutoff: f64::from(options.alpha_cutoff),
        double_sided: options.double_sided,
        points,
        unique,
        radius: f64::from(bounds.radius),
    };
    if bake.radius > 0.0 && !indices.is_empty() {
        let mut buffer = budget.filled(samples, EMPTY_SAMPLE)?;
        let mut projected = budget.filled(vertex_count, Projected::default())?;
        let mut marks = budget.filled(tile, 0u8)?;
        let view_count = options.grid.view_count()?;
        for view in 0..view_count {
            let frame = Frame::new(decode(options.grid.layout, options.grid.point(view)?));
            work.add(samples)?;
            buffer.fill(EMPTY_SAMPLE);
            bake.project(frame, side, &mut projected, work)?;
            bake.raster(frame, side, &projected, &mut buffer, &mut report, work)?;
            let origin = [
                usize::try_from(view % n).map_err(|_| Error::SizeOverflow)? * res,
                usize::try_from(view / n).map_err(|_| Error::SizeOverflow)? * res,
            ];
            let mut target = Target {
                albedo_alpha: &mut albedo_alpha,
                normal_depth: &mut normal_depth,
                marks: &mut marks,
                size,
                origin,
                res,
            };
            let covered =
                target.resolve(&buffer, spa, options.coverage_threshold, &mut report, work)?;
            if covered == 0 {
                report.empty_views = report
                    .empty_views
                    .checked_add(1)
                    .ok_or(Error::SizeOverflow)?;
            } else {
                target.dilate(options.dilation, work)?;
            }
        }
    } else {
        report.empty_views = options.grid.view_count()?;
    }
    Ok(ImpostorAtlas {
        grid: options.grid,
        view_resolution: options.view_resolution,
        size: size32,
        bounds,
        albedo_alpha,
        normal_depth,
        report,
    })
}

impl Bake<'_> {
    fn project(
        &self,
        frame: Frame,
        side: usize,
        projected: &mut [Projected],
        work: &mut Work,
    ) -> Result<(), Error> {
        // Samples per world unit, in snapped units.
        let scale = side as f64 * FIX as f64 / (2.0 * self.radius);
        work.add(self.unique.len())?;
        for &index in &self.unique {
            let v = index as usize;
            let q = self.points[v];
            let x = (dot(q, frame.right) + self.radius) * scale;
            let y = (self.radius - dot(q, frame.up)) * scale;
            if !(x.abs() <= MAX_FIX && y.abs() <= MAX_FIX) {
                return Err(if x.is_finite() && y.is_finite() {
                    Error::InvalidParameter
                } else {
                    Error::NumericalFailure
                });
            }
            projected[v] = Projected {
                // Bounded by MAX_FIX above.
                x: libm::round(x) as i64,
                y: libm::round(y) as i64,
                z: 0.5 - dot(q, frame.direction) / (2.0 * self.radius),
            };
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn raster(
        &self,
        frame: Frame,
        side: usize,
        projected: &[Projected],
        buffer: &mut [Sample],
        report: &mut ImpostorReport,
        work: &mut Work,
    ) -> Result<(), Error> {
        let side64 = i64::try_from(side).map_err(|_| Error::SizeOverflow)?;
        let (triangles, _) = self.indices.as_chunks::<3>();
        work.add(triangles.len())?;
        for (t, tri) in triangles.iter().enumerate() {
            // Triangles without area in the mesh are skipped in every view,
            // even where snapping would give them a sliver of screen area.
            let q = tri.map(|i| self.points[i as usize]);
            let face = cross(sub(q[1], q[0]), sub(q[2], q[0]));
            if face == [0.0; 3] {
                continue;
            }
            let mut ids = tri.map(|i| i as usize);
            let mut p = ids.map(|i| projected[i]);
            let area = edge(p[0], p[1], p[2].x, p[2].y);
            if area == 0 {
                continue;
            }
            // Front faces are counter-clockwise seen from the viewer, which
            // is negative area with rows growing downward.
            let front = area < 0;
            if !front && !self.double_sided {
                continue;
            }
            if area < 0 {
                ids.swap(1, 2);
                p.swap(1, 2);
            }
            let area = area.abs() as f64;
            let min_x = p[0].x.min(p[1].x).min(p[2].x);
            let max_x = p[0].x.max(p[1].x).max(p[2].x);
            let min_y = p[0].y.min(p[1].y).min(p[2].y);
            let max_y = p[0].y.max(p[1].y).max(p[2].y);
            // Sample s has center s * FIX + FIX / 2.
            let x0 = (min_x - FIX / 2 + FIX - 1).div_euclid(FIX).max(0);
            let x1 = (max_x - FIX / 2).div_euclid(FIX).min(side64 - 1);
            let y0 = (min_y - FIX / 2 + FIX - 1).div_euclid(FIX).max(0);
            let y1 = (max_y - FIX / 2).div_euclid(FIX).min(side64 - 1);
            if x0 > x1 || y0 > y1 {
                continue;
            }
            let visits =
                u64::try_from((x1 - x0 + 1) * (y1 - y0 + 1)).map_err(|_| Error::SizeOverflow)?;
            work.add(usize::try_from(visits).map_err(|_| Error::SizeOverflow)?)?;
            let own = [owns(p[1], p[2]), owns(p[2], p[0]), owns(p[0], p[1])];
            for sy in y0..=y1 {
                let py = sy * FIX + FIX / 2;
                // sy and sx are within 0..side, so the index fits.
                let row = sy as usize * side;
                for sx in x0..=x1 {
                    let px = sx * FIX + FIX / 2;
                    let w = [
                        edge(p[1], p[2], px, py),
                        edge(p[2], p[0], px, py),
                        edge(p[0], p[1], px, py),
                    ];
                    if (0..3).any(|k| w[k] < 0 || (w[k] == 0 && !own[k])) {
                        continue;
                    }
                    let b = w.map(|x| x as f64 / area);
                    let z = b[0] * p[0].z + b[1] * p[1].z + b[2] * p[2].z;
                    let slot = &mut buffer[row + sx as usize];
                    // Depths are finite; empty samples hold infinity.
                    if z >= slot.z {
                        continue;
                    }
                    let Some(rgba) = self.shade(t, ids, b, report)? else {
                        continue;
                    };
                    let normal = self.normal(ids, b, front, frame, face);
                    *slot = Sample {
                        z,
                        rgb: [rgba[0] as f32, rgba[1] as f32, rgba[2] as f32],
                        normal: narrow(normal),
                    };
                }
            }
        }
        Ok(())
    }

    /// Colour and alpha at barycentrics `b` of vertices `ids`, or `None` when
    /// the alpha test discards the fragment.
    fn shade(
        &self,
        triangle: usize,
        ids: [usize; 3],
        b: [f64; 3],
        report: &mut ImpostorReport,
    ) -> Result<Option<[f64; 4]>, Error> {
        let rgba = match self.albedo {
            ImpostorAlbedo::White => [1.0; 4],
            ImpostorAlbedo::VertexColors(colors) => {
                let c = ids.map(|i| colors[i]);
                [0, 1, 2, 3].map(|k| {
                    b[0] * f64::from(c[0][k])
                        + b[1] * f64::from(c[1][k])
                        + b[2] * f64::from(c[2][k])
                })
            }
            ImpostorAlbedo::Texture { uvs, sampler } => {
                let t = ids.map(|i| uvs[i]);
                let uv = [0, 1].map(|k| {
                    (b[0] * f64::from(t[0][k])
                        + b[1] * f64::from(t[1][k])
                        + b[2] * f64::from(t[2][k])) as f32
                });
                report.sampler_calls = report
                    .sampler_calls
                    .checked_add(1)
                    .ok_or(Error::SizeOverflow)?;
                let value = sampler(triangle, uv);
                if !value.iter().all(|x| x.is_finite()) {
                    return Err(Error::NumericalFailure);
                }
                value.map(f64::from)
            }
        };
        Ok((rgba[3] >= self.cutoff).then_some(rgba))
    }

    /// Interpolated vertex normal, or the face normal when there are no
    /// vertex normals or they cancel, reversed on back faces.
    fn normal(&self, ids: [usize; 3], b: [f64; 3], front: bool, frame: Frame, face: V) -> V {
        let mut n = [0.0; 3];
        if let Some(normals) = self.normals {
            let v = ids.map(|i| widen(normals[i]));
            n = [0, 1, 2].map(|k| b[0] * v[0][k] + b[1] * v[1][k] + b[2] * v[2][k]);
        }
        let l = length(n);
        let n = if l > 1e-30 && l.is_finite() {
            n.map(|x| x / l)
        } else {
            let l = length(face);
            if l > 0.0 && l.is_finite() {
                face.map(|x| x / l)
            } else {
                // The cross product underflowed or overflowed; face the viewer.
                if front {
                    frame.direction
                } else {
                    frame.direction.map(|x| -x)
                }
            }
        };
        if front {
            n
        } else {
            n.map(|x| -x)
        }
    }
}

struct Target<'a> {
    albedo_alpha: &'a mut [[u8; 4]],
    normal_depth: &'a mut [[u8; 4]],
    /// Per tile texel: 0 empty, 1 covered, `p + 1` filled by dilation pass `p`.
    marks: &'a mut [u8],
    size: usize,
    origin: [usize; 2],
    res: usize,
}

impl Target<'_> {
    #[inline]
    fn at(&self, x: usize, y: usize) -> usize {
        (self.origin[1] + y) * self.size + self.origin[0] + x
    }

    fn resolve(
        &mut self,
        buffer: &[Sample],
        spa: usize,
        threshold: f32,
        report: &mut ImpostorReport,
        work: &mut Work,
    ) -> Result<u64, Error> {
        work.add(buffer.len())?;
        let side = self.res * spa;
        let total = (spa * spa) as f64;
        let threshold = f64::from(threshold);
        let (mut covered, mut opaque) = (0u64, 0u64);
        for y in 0..self.res {
            for x in 0..self.res {
                let mut count = 0usize;
                let mut rgb = [0.0f64; 3];
                let mut normal = [0.0f64; 3];
                let mut z = f64::INFINITY;
                for sy in 0..spa {
                    let row = (y * spa + sy) * side + x * spa;
                    for s in &buffer[row..row + spa] {
                        if s.z.is_finite() {
                            count += 1;
                            for k in 0..3 {
                                rgb[k] += f64::from(s.rgb[k]);
                                normal[k] += f64::from(s.normal[k]);
                            }
                            z = z.min(s.z);
                        }
                    }
                }
                let at = self.at(x, y);
                self.marks[y * self.res + x] = u8::from(count > 0);
                if count == 0 {
                    continue;
                }
                covered += 1;
                let c = count as f64;
                let alpha = unorm8((c / total) / threshold);
                let l = length(normal);
                let normal = if l > 0.0 {
                    normal.map(|x| x / l)
                } else {
                    normal
                };
                self.albedo_alpha[at] = [
                    unorm8(rgb[0] / c),
                    unorm8(rgb[1] / c),
                    unorm8(rgb[2] / c),
                    alpha,
                ];
                self.normal_depth[at] = [
                    unorm8(normal[0] * 0.5 + 0.5),
                    unorm8(normal[1] * 0.5 + 0.5),
                    unorm8(normal[2] * 0.5 + 0.5),
                    unorm8(z),
                ];
                opaque += u64::from(alpha == 255);
            }
        }
        report.covered_texels = report
            .covered_texels
            .checked_add(covered)
            .ok_or(Error::SizeOverflow)?;
        report.opaque_texels = report
            .opaque_texels
            .checked_add(opaque)
            .ok_or(Error::SizeOverflow)?;
        Ok(covered)
    }

    fn dilate(&mut self, passes: u32, work: &mut Work) -> Result<(), Error> {
        let res = self.res;
        for pass in 1..=passes {
            // pass <= 64, so marks fit in u8.
            let pass = pass as u8;
            work.add(res * res)?;
            let mut changed = false;
            for y in 0..res {
                for x in 0..res {
                    if self.marks[y * res + x] != 0 {
                        continue;
                    }
                    let mut sum_c = [0u32; 3];
                    let mut sum_n = [0u32; 4];
                    let mut count = 0u32;
                    let neighbours = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ];
                    for (nx, ny) in neighbours {
                        if nx >= res || ny >= res {
                            continue;
                        }
                        let mark = self.marks[ny * res + nx];
                        if mark == 0 || mark > pass {
                            continue;
                        }
                        let at = self.at(nx, ny);
                        for (s, &c) in sum_c.iter_mut().zip(&self.albedo_alpha[at]) {
                            *s += u32::from(c);
                        }
                        for (s, &c) in sum_n.iter_mut().zip(&self.normal_depth[at]) {
                            *s += u32::from(c);
                        }
                        count += 1;
                    }
                    if count == 0 {
                        continue;
                    }
                    // Averages of at most four bytes stay below 256.
                    let avg = |s: u32| ((s + count / 2) / count) as u8;
                    let at = self.at(x, y);
                    self.albedo_alpha[at] = [avg(sum_c[0]), avg(sum_c[1]), avg(sum_c[2]), 0];
                    self.normal_depth[at] = sum_n.map(avg);
                    self.marks[y * res + x] = pass + 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        Ok(())
    }
}
