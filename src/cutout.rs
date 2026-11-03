//! Alpha-cutout card trimming. An extension, not in C++ meshoptimizer.
//!
//! Each card (edge-connected triangles) is replaced by a convex polygon in
//! texture space that contains every texel footprint whose alpha passes the
//! threshold, with at most a caller-chosen number of vertices: the problem
//! solved by Emil Persson's particle trimmer
//! (<http://www.humus.name/index.php?page=Cool&ID=8>, 2009). This
//! implementation takes the convex hull of the opaque texels, then chooses a
//! subset of its supporting lines (hull edges, corner bisectors and lines
//! parallel to the card outline) whose intersection has the smallest area. A
//! greedy pass first reduces long hulls and an exact dynamic program then
//! picks the best cycle of lines.
use crate::index::Convert;
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Workspace};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

type Point = [f64; 2];

/// Largest accepted texel coordinate magnitude (`2^24`).
const MAX_COORD: f64 = 16_777_216.0;
/// Largest accepted padding in texels.
const MAX_PADDING: f32 = 64.0;
/// Largest accepted vertex budget.
const MAX_BUDGET: u32 = 16;
/// Hulls with more edges are greedily reduced before the exact search.
const REDUCE_TO: usize = 24;
/// Cards with longer outlines skip the single-polygon output.
const MAX_OUTLINE: usize = 64;
/// Fixed footprint margin in texels on top of the caller padding.
const MARGIN: f64 = 1.0 / 256.0;
/// Scale-relative footprint margin, larger than f32 rounding of output UVs.
const MARGIN_SCALE: f64 = 1.0 / 4_194_304.0;
/// A card is trimmed only when its area shrinks by more than this fraction.
const MIN_GAIN: f64 = 1e-9;

/// Texel lookup outside `0..width` and `0..height`. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlphaWrap {
    /// Use the nearest edge texel (clamp to edge). Default.
    #[default]
    Clamp,
    /// Repeat the mask in both directions.
    Repeat,
}

/// Read-only 8-bit alpha plane for [`trim_alpha_cards`]. Not in C++ meshoptimizer.
///
/// Texel `(x, y)` is `data[y * pitch + x * stride]` and covers
/// `[x, x + 1) x [y, y + 1)` in texel space, which is `(u * width, v * height)`:
/// `v = 0` is the first row and texel centers lie at half-integer
/// coordinates, as in [`opacity_map_rasterize`](crate::opacity_map_rasterize).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct AlphaMask<'a> {
    data: &'a [u8],
    stride: usize,
    pitch: usize,
    width: u32,
    height: u32,
}

impl<'a> AlphaMask<'a> {
    /// A tightly packed plane of `width * height` alpha bytes.
    ///
    /// # Errors
    /// `InvalidParameter` for a zero dimension, `InvalidLayout` if `data` is
    /// too short, or `SizeOverflow`.
    #[inline]
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Result<Self, Error> {
        let pitch = usize::try_from(width).map_err(|_| Error::SizeOverflow)?;
        Self::with_layout(data, 1, pitch, width, height)
    }

    /// A strided view, such as the alpha byte of RGBA8 data
    /// (`&rgba[3..]`, stride 4, pitch `4 * width`).
    ///
    /// # Errors
    /// `InvalidParameter` for a zero dimension or stride, `InvalidLayout` if
    /// `data` does not contain the last texel, or `SizeOverflow`.
    #[inline]
    pub fn with_layout(
        data: &'a [u8],
        stride: usize,
        pitch: usize,
        width: u32,
        height: u32,
    ) -> Result<Self, Error> {
        if width == 0 || height == 0 || stride == 0 {
            return Err(Error::InvalidParameter);
        }
        let w = usize::try_from(width - 1).map_err(|_| Error::SizeOverflow)?;
        let h = usize::try_from(height - 1).map_err(|_| Error::SizeOverflow)?;
        let last = h
            .checked_mul(pitch)
            .and_then(|row| w.checked_mul(stride).and_then(|x| row.checked_add(x)))
            .ok_or(Error::SizeOverflow)?;
        if last >= data.len() {
            return Err(Error::InvalidLayout);
        }
        Ok(Self {
            data,
            stride,
            pitch,
            width,
            height,
        })
    }

    /// Width in texels.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Height in texels.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Alpha of texel `(x, y)`, or `None` outside the mask.
    #[inline]
    pub fn alpha(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        self.data.get(y * self.pitch + x * self.stride).copied()
    }

    fn texel(&self, x: i64, y: i64, wrap: AlphaWrap) -> u8 {
        let (w, h) = (i64::from(self.width), i64::from(self.height));
        let (x, y) = match wrap {
            AlphaWrap::Clamp => (x.clamp(0, w - 1), y.clamp(0, h - 1)),
            AlphaWrap::Repeat => (x.rem_euclid(w), y.rem_euclid(h)),
        };
        match (u32::try_from(x), u32::try_from(y)) {
            (Ok(x), Ok(y)) => self.alpha(x, y).unwrap_or(0),
            _ => 0,
        }
    }
}

/// Settings for [`trim_alpha_cards`]. Not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct CardTrimOptions {
    /// Texels with alpha at or above this value are visible (default 128,
    /// the byte form of a 0.5 cutoff). Zero keeps every card unchanged.
    pub threshold: u8,
    /// Most vertices of the fitted polygon per card, 3..=16 (default 8).
    pub max_vertices: u32,
    /// Footprint expansion in texels around each visible texel, finite and
    /// within 0..=64 (default 0.5). 0.5 covers bilinear sampling of the full
    /// resolution mask; 0 covers nearest sampling. For box-filtered mip level
    /// `L` use `1.5 * 2^L - 1`. Alpha rescaled per mip is not covered.
    pub padding: f32,
    /// Lookup outside the mask (default [`AlphaWrap::Clamp`]).
    pub wrap: AlphaWrap,
}

impl Default for CardTrimOptions {
    fn default() -> Self {
        Self {
            threshold: 128,
            max_vertices: 8,
            padding: 0.5,
            wrap: AlphaWrap::Clamp,
        }
    }
}

/// Counts and texture-space areas. Not in C++ meshoptimizer.
///
/// Areas are in square texels of the mask. Cards cover `area_before` before
/// and `area_after` after trimming; passthrough triangles are not counted.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CardTrimReport {
    /// Input triangles.
    pub input_triangles: usize,
    /// Output triangles.
    pub output_triangles: usize,
    /// Output vertices.
    pub output_vertices: usize,
    /// Cards: edge-connected triangles with nonzero texture area.
    pub cards: usize,
    /// Cards replaced by a smaller polygon.
    pub trimmed_cards: usize,
    /// Trimmed cards emitted as one clipped polygon per source triangle,
    /// because their attributes are not one affine function of UV or the
    /// card is not a convex disk in texture space.
    pub split_cards: usize,
    /// Cards without visible texels, dropped from the output.
    pub removed_cards: usize,
    /// Cards copied unchanged: no smaller polygon within the vertex budget.
    pub unchanged_cards: usize,
    /// Triangles with zero texture area, copied unchanged.
    pub passthrough_triangles: usize,
    /// Texture area of the cards before trimming.
    pub area_before: f64,
    /// Texture area of the cards after trimming.
    pub area_after: f64,
    /// Visible texels whose centers lie inside a card triangle, counted per
    /// triangle; an estimate of the visible area within `area_before`.
    pub opaque_texels: u64,
}

/// Trimmed vertex and index buffers. Not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct TrimmedCardsGeneric<I: crate::Index = u32> {
    /// Output vertices with the input stride, in first-use order.
    pub vertices: Vec<f32>,
    /// Output triangles.
    pub indices: Vec<I>,
    /// Counts and areas.
    pub report: CardTrimReport,
}

/// The source-compatible u32 form of [`TrimmedCardsGeneric`].
/// Trimmed vertex and index buffers. Not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct TrimmedCards {
    /// Output vertices with the input stride, in first-use order.
    pub vertices: Vec<f32>,
    /// Output triangles.
    pub indices: Vec<u32>,
    /// Counts and areas.
    pub report: CardTrimReport,
}

impl From<TrimmedCardsGeneric<u32>> for TrimmedCards {
    #[inline]
    fn from(value: TrimmedCardsGeneric<u32>) -> Self {
        Self {
            vertices: value.vertices,
            indices: value.indices,
            report: value.report,
        }
    }
}

impl From<TrimmedCards> for TrimmedCardsGeneric<u32> {
    #[inline]
    fn from(value: TrimmedCards) -> Self {
        Self {
            vertices: value.vertices,
            indices: value.indices,
            report: value.report,
        }
    }
}

impl<I: crate::Index> Convert<I> for TrimmedCardsGeneric {
    type Output = TrimmedCardsGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(TrimmedCardsGeneric {
            vertices: self.vertices,
            indices: I::from_vec(self.indices, ws)?,
            report: self.report,
        })
    }
}

/// Fit alpha-tested cards to their visible texels. Not in C++ meshoptimizer.
///
/// `vertices` holds `vertex_stride` floats per vertex with the UV pair at
/// float offset `uv_offset`; every float is an attribute. A card is a set of
/// triangles connected through shared edges (equal vertex index pairs) with
/// nonzero texture area; any triangle soup is accepted. For each card the
/// visible region is the union of the footprints of texels with
/// `alpha >= threshold`, each expanded by `padding` texels, intersected with
/// the card. It is replaced by a convex polygon with at most `max_vertices`
/// vertices that contains this region and lies inside the convex hull of the
/// card's UVs, chosen to minimize area among intersections of supporting
/// lines of the visible texels' convex hull (hull edges, corner bisectors and
/// lines parallel to the card outline), as in Persson's particle trimmer.
/// This is not guaranteed to be the global minimum. Cards without visible
/// texels are removed; cards without a smaller polygon are copied.
///
/// A card whose triangles form a convex disk in texture space and whose
/// attributes are one affine function of UV (a planar card with linear
/// attributes) becomes one polygon of at most `max_vertices` vertices,
/// fan-triangulated. Other cards are clipped per source triangle and each
/// piece is interpolated with that triangle's own barycentrics, so rendering
/// is unchanged inside the kept area; such cards may exceed the vertex budget.
/// Shared edge crossings are computed once per edge so pieces stay watertight.
/// Output winding matches each source triangle. Attributes are interpolated
/// linearly in f64; normals are not renormalized.
///
/// Coverage: every point of a visible footprint inside a card triangle lies
/// inside the output, including a margin of `1/256 + |texel coordinate| / 2^22`
/// texels that absorbs f32 rounding of output UVs. Where the footprint reaches
/// the card's own outline, the output edge follows the input edge up to f32
/// rounding of the new vertices. Visibility assumes the GPU alpha test sees
/// interpolated alpha no larger than the maximum of the texels it filters,
/// which holds for nearest and bilinear filtering of this mask.
///
/// Work is proportional to the padded texel area of each distinct card
/// (cards with identical UV triangles reuse one fit) plus at most
/// `O(max_vertices * n^3)` for `n` reduced hull edges. Storage and work are
/// fallibly charged to the workspace. The result is deterministic.
///
/// ```
/// use meshoptimizer_rs::{trim_alpha_cards, AlphaMask, CardTrimOptions, Workspace};
/// // An 8x8 mask with a visible 2x2 block in the middle.
/// let mut alpha = [0u8; 64];
/// for y in 3..5 {
///     for x in 3..5 {
///         alpha[y * 8 + x] = 255;
///     }
/// }
/// let mask = AlphaMask::new(&alpha, 8, 8)?;
/// // One quad card: x, y, z, u, v per vertex.
/// let vertices = [
///     0., 0., 0., 0., 0., 1., 0., 0., 1., 0., 1., 1., 0., 1., 1., 0., 1., 0., 0., 1.,
/// ];
/// let trimmed = trim_alpha_cards(&[0, 1, 2, 0, 2, 3], &vertices, 5, 3, mask,
///     CardTrimOptions::default(), &mut Workspace::default())?;
/// assert_eq!(trimmed.report.trimmed_cards, 1);
/// assert!(trimmed.report.area_after < 10.0 && trimmed.report.area_before == 64.0);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// `InvalidTopology` for an incomplete triangle, `IndexOutOfBounds` for an
/// invalid index, `InvalidLayout` when `uv_offset + 2 > vertex_stride` or the
/// vertex buffer is not a whole number of vertices, `InvalidParameter` for
/// invalid options or a texel coordinate beyond `2^24`, `NumericalFailure`
/// for a non-finite referenced float, `IndexOverflow` when the output does
/// not fit `I`, and the usual `SizeOverflow`, `LimitExceeded` or
/// `AllocationFailed`. No partial result is returned.
#[inline]
pub fn trim_alpha_cards_generic<I: crate::Index>(
    indices: &[I],
    vertices: &[f32],
    vertex_stride: usize,
    uv_offset: usize,
    mask: AlphaMask<'_>,
    options: CardTrimOptions,
    workspace: &mut Workspace,
) -> Result<TrimmedCardsGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        trim_u32(
            indices,
            vertices,
            vertex_stride,
            uv_offset,
            mask,
            options,
            workspace,
        )
        .and_then(|value| Convert::<I>::convert(value, workspace))
    })
}

/// The source-compatible u32 entry point; see [`trim_alpha_cards_generic`] for selectable index widths.
#[inline]
pub fn trim_alpha_cards(
    indices: &[u32],
    vertices: &[f32],
    vertex_stride: usize,
    uv_offset: usize,
    mask: AlphaMask<'_>,
    options: CardTrimOptions,
    workspace: &mut Workspace,
) -> Result<TrimmedCards, Error> {
    trim_alpha_cards_generic::<u32>(
        indices,
        vertices,
        vertex_stride,
        uv_offset,
        mask,
        options,
        workspace,
    )
    .map(Into::into)
}

// ---------------------------------------------------------------------------
// Geometry helpers.

#[inline(always)]
fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline(always)]
fn cross(a: Point, b: Point) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

#[inline(always)]
fn dot(a: Point, b: Point) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

#[inline(always)]
fn lerp(a: Point, b: Point, t: f64) -> Point {
    [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]
}

/// Canonical side of `p` relative to the directed line `lo -> hi`.
#[inline(always)]
fn side(lo: Point, hi: Point, p: Point) -> f64 {
    cross(sub(hi, lo), sub(p, lo))
}

/// Twice the signed area.
fn area2(poly: &[Point]) -> f64 {
    let Some(&origin) = poly.first() else {
        return 0.0;
    };
    let mut sum = 0.0;
    for pair in poly.windows(2) {
        sum += cross(sub(pair[0], origin), sub(pair[1], origin));
    }
    sum
}

fn to_i64(x: f64) -> Result<i64, Error> {
    if x.is_finite() && x.abs() < 9.0e15 {
        Ok(x as i64)
    } else {
        Err(Error::NumericalFailure)
    }
}

/// Is `p` inside the convex counter-clockwise polygon, within `eps`?
fn inside(poly: &[Point], p: Point, eps: f64) -> bool {
    let n = poly.len();
    (0..n).all(|i| {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let e = sub(b, a);
        cross(e, sub(p, a)) >= -eps * (e[0].abs() + e[1].abs())
    })
}

/// X range of a triangle within the horizontal strip `y0..=y1`.
fn strip_range(t: &[Point; 3], y0: f64, y1: f64) -> Option<(f64, f64)> {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for k in 0..3 {
        let a = t[k];
        let b = t[(k + 1) % 3];
        if a[1] >= y0 && a[1] <= y1 {
            lo = lo.min(a[0]);
            hi = hi.max(a[0]);
        }
        for y in [y0, y1] {
            if (a[1] < y && b[1] > y) || (a[1] > y && b[1] < y) {
                let x = a[0] + (y - a[1]) * (b[0] - a[0]) / (b[1] - a[1]);
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
    }
    (lo <= hi).then_some((lo, hi))
}

// ---------------------------------------------------------------------------
// Fallible storage charged to the workspace.

struct Store<'w> {
    ws: &'w mut Workspace,
    bytes: usize,
}

impl Store<'_> {
    fn charge(&mut self, bytes: usize) -> Result<(), Error> {
        let total = self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?;
        self.ws.charge_owned(total)?;
        self.bytes = total;
        Ok(())
    }

    fn grow<T>(&mut self, v: &mut Vec<T>, extra: usize) -> Result<(), Error> {
        let need = v.len().checked_add(extra).ok_or(Error::SizeOverflow)?;
        if need <= v.capacity() {
            return Ok(());
        }
        let target = need.max(v.capacity().saturating_mul(2)).max(4);
        self.charge(checked_bytes(
            target - v.capacity(),
            core::mem::size_of::<T>(),
        )?)?;
        v.try_reserve_exact(target - v.len())
            .map_err(|_| Error::AllocationFailed)
    }

    fn push<T>(&mut self, v: &mut Vec<T>, value: T) -> Result<(), Error> {
        self.grow(v, 1)?;
        v.push(value);
        Ok(())
    }

    fn fill<T: Clone>(&mut self, v: &mut Vec<T>, n: usize, value: T) -> Result<(), Error> {
        v.clear();
        self.grow(v, n)?;
        v.resize(n, value);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Input and per-card state.

struct Input<'a> {
    indices: &'a [u32],
    vertices: &'a [f32],
    stride: usize,
    uv: usize,
    mask: AlphaMask<'a>,
    threshold: u8,
    budget: usize,
    padding: f64,
    wrap: AlphaWrap,
    w: f64,
    h: f64,
}

impl Input<'_> {
    fn attrs(&self, v: u32) -> &[f32] {
        let base = v as usize * self.stride;
        &self.vertices[base..base + self.stride]
    }

    fn texel(&self, v: u32) -> Point {
        let a = self.attrs(v);
        [
            f64::from(a[self.uv]) * self.w,
            f64::from(a[self.uv + 1]) * self.h,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct Tri {
    v: [u32; 3],
    p: [Point; 3],
    area2: f64,
    /// Interior side of each edge `k` (`v[k] -> v[k + 1]`) relative to the
    /// canonical direction from the lower to the higher vertex index.
    sigma: [f64; 3],
}

impl Tri {
    fn new(input: &Input<'_>, t: usize) -> Self {
        let i = &input.indices[t * 3..t * 3 + 3];
        let v = [i[0], i[1], i[2]];
        let p = v.map(|v| input.texel(v));
        let area2 = cross(sub(p[1], p[0]), sub(p[2], p[0]));
        let mut tri = Self {
            v,
            p,
            area2,
            sigma: [0.0; 3],
        };
        for k in 0..3 {
            let (_, _, lo, hi) = tri.edge(k);
            let s = side(lo, hi, p[(k + 2) % 3]);
            tri.sigma[k] = if s > 0.0 {
                1.0
            } else if s < 0.0 {
                -1.0
            } else {
                0.0
            };
        }
        tri
    }

    fn valid(&self) -> bool {
        self.area2 != 0.0 && self.sigma.iter().all(|&s| s != 0.0)
    }

    /// Canonical edge `k`: lower index, higher index and their points.
    fn edge(&self, k: usize) -> (u32, u32, Point, Point) {
        let (a, b) = (k, (k + 1) % 3);
        if self.v[a] < self.v[b] {
            (self.v[a], self.v[b], self.p[a], self.p[b])
        } else {
            (self.v[b], self.v[a], self.p[b], self.p[a])
        }
    }

    fn point_of(&self, v: u32) -> Option<Point> {
        (0..3).find(|&k| self.v[k] == v).map(|k| self.p[k])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    /// An input vertex.
    Original(u32),
    /// Fitted-polygon edge `k` crossing the canonical input edge.
    Cross(u32, u32, u32),
    /// Fitted-polygon vertex inside one card triangle (local index), or on
    /// a triangle edge (`u32::MAX`).
    Corner(u32, u32),
}

#[derive(Clone, Copy, Debug)]
enum Line {
    Fit(u32),
    Edge(u32, u32),
}

#[derive(Clone, Copy, Debug)]
struct WVert {
    p: Point,
    key: Key,
    /// Line of the edge leaving this vertex.
    line: Line,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    s: Point,
    d: Point,
    e: Point,
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    ok: bool,
    cap: f64,
    apex: Point,
}

#[derive(Clone, Copy, Debug)]
enum FitKind {
    Empty,
    Keep,
    Polygon(usize, usize),
}

#[derive(Clone, Copy, Debug)]
struct Fit {
    kind: FitKind,
    opaque: u64,
}

#[derive(Default)]
struct Scratch {
    tris: Vec<Tri>,
    key: Vec<u32>,
    points: Vec<Point>,
    hull: Vec<Point>,
    dom: Vec<Point>,
    tmp: Vec<Point>,
    rows: Vec<(i64, i64)>,
    edges: Vec<Edge>,
    dirs: Vec<Point>,
    table: Vec<Pair>,
    cost: Vec<f64>,
    parent: Vec<u32>,
    seq: Vec<u32>,
    best: Vec<u32>,
    a: Vec<WVert>,
    b: Vec<WVert>,
    pieces: Vec<WVert>,
    ends: Vec<(usize, usize)>,
    directed: Vec<(u32, u32)>,
    boundary: Vec<(u32, u32)>,
    stats: Vec<[f64; 3]>,
    values: Vec<f32>,
    keymap: BTreeMap<Key, u32>,
}

struct Output {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    remap: Vec<u32>,
    report: CardTrimReport,
}

// ---------------------------------------------------------------------------
// Driver.

fn trim_u32(
    indices: &[u32],
    vertices: &[f32],
    stride: usize,
    uv: usize,
    mask: AlphaMask<'_>,
    options: CardTrimOptions,
    workspace: &mut Workspace,
) -> Result<TrimmedCardsGeneric, Error> {
    if !(3..=MAX_BUDGET).contains(&options.max_vertices)
        || !options.padding.is_finite()
        || !(0.0..=MAX_PADDING).contains(&options.padding)
    {
        return Err(Error::InvalidParameter);
    }
    let fits_uv = uv.checked_add(2).is_some_and(|end| end <= stride);
    if stride == 0 || !fits_uv || !vertices.len().is_multiple_of(stride) {
        return Err(Error::InvalidLayout);
    }
    let input = Input {
        indices,
        vertices,
        stride,
        uv,
        mask,
        threshold: options.threshold,
        budget: usize::try_from(options.max_vertices).map_err(|_| Error::SizeOverflow)?,
        padding: f64::from(options.padding) + MARGIN,
        wrap: options.wrap,
        w: f64::from(mask.width),
        h: f64::from(mask.height),
    };
    let mut work = workspace.begin();
    let result = {
        let mut store = Store {
            ws: workspace,
            bytes: 0,
        };
        run(&input, &mut store, &mut work)
    };
    workspace.finish(&work);
    result
}

fn run(
    input: &Input<'_>,
    store: &mut Store<'_>,
    work: &mut Work,
) -> Result<TrimmedCardsGeneric, Error> {
    let vertex_count = input.vertices.len() / input.stride;
    crate::workspace::topology(input.indices, vertex_count, work)?;
    work.add(input.indices.len().saturating_mul(input.stride))?;
    for &v in input.indices {
        let a = input.attrs(v);
        if !a.iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
        let p = input.texel(v);
        if p[0].abs() > MAX_COORD || p[1].abs() > MAX_COORD {
            return Err(Error::InvalidParameter);
        }
    }
    let n = input.indices.len() / 3;

    // Group valid triangles into cards through shared edges.
    let mut parent = Vec::new();
    store.fill(&mut parent, n, u32::MAX)?;
    let mut edges: Vec<(u32, u32, u32)> = Vec::new();
    store.grow(&mut edges, n.saturating_mul(3))?;
    for (t, slot) in parent.iter_mut().enumerate() {
        let tri = Tri::new(input, t);
        if !tri.valid() {
            continue;
        }
        let id = parent_id(t)?;
        *slot = id;
        for k in 0..3 {
            let (lo, hi, _, _) = tri.edge(k);
            edges.push((lo, hi, id));
        }
    }
    work.add(edges.len().saturating_mul(32))?;
    edges.sort_unstable();
    for pair in edges.windows(2) {
        if pair[0].0 == pair[1].0 && pair[0].1 == pair[1].1 {
            union(&mut parent, pair[0].2, pair[1].2);
        }
    }
    // Members of each card in triangle order, grouped by root.
    let mut offsets = Vec::new();
    store.fill(&mut offsets, n + 1, 0usize)?;
    for t in 0..n {
        if parent[t] != u32::MAX {
            let root = find(&mut parent, t);
            offsets[root + 1] += 1;
        }
    }
    for t in 0..n {
        offsets[t + 1] += offsets[t];
    }
    let mut fill = Vec::new();
    store.fill(&mut fill, n, 0usize)?;
    let mut members = Vec::new();
    store.fill(&mut members, offsets[n], 0u32)?;
    for t in 0..n {
        if parent[t] != u32::MAX {
            let root = find(&mut parent, t);
            members[offsets[root] + fill[root]] = parent_id(t)?;
            fill[root] += 1;
        }
    }
    drop(edges);

    let mut out = Output {
        vertices: Vec::new(),
        indices: Vec::new(),
        remap: Vec::new(),
        report: CardTrimReport {
            input_triangles: n,
            ..CardTrimReport::default()
        },
    };
    store.fill(&mut out.remap, vertex_count, u32::MAX)?;
    let mut scratch = Scratch::default();
    let mut memo: BTreeMap<Vec<u32>, Fit> = BTreeMap::new();
    let mut polygons: Vec<Point> = Vec::new();
    for t in 0..n {
        if parent[t] == u32::MAX {
            out.report.passthrough_triangles += 1;
            let tri = &input.indices[t * 3..t * 3 + 3];
            emit_original(input, store, &mut out, [tri[0], tri[1], tri[2]])?;
            continue;
        }
        if find(&mut parent, t) != t {
            continue;
        }
        let card = &members[offsets[t]..offsets[t + 1]];
        process_card(
            input,
            store,
            work,
            &mut out,
            &mut scratch,
            &mut memo,
            &mut polygons,
            card,
        )?;
    }
    out.report.output_triangles = out.indices.len() / 3;
    out.report.output_vertices = out.vertices.len() / input.stride;
    Ok(TrimmedCardsGeneric {
        vertices: out.vertices,
        indices: out.indices,
        report: out.report,
    })
}

fn parent_id(t: usize) -> Result<u32, Error> {
    u32::try_from(t).map_err(|_| Error::SizeOverflow)
}

fn find(parent: &mut [u32], t: usize) -> usize {
    let mut x = t;
    while parent[x] as usize != x {
        let up = parent[x] as usize;
        parent[x] = parent[up];
        x = parent[x] as usize;
    }
    x
}

fn union(parent: &mut [u32], a: u32, b: u32) {
    let ra = find(parent, a as usize);
    let rb = find(parent, b as usize);
    if ra < rb {
        parent[rb] = parent[ra];
    } else if rb < ra {
        parent[ra] = parent[rb];
    }
}

fn copy_vertex(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    v: u32,
) -> Result<u32, Error> {
    let slot = out.remap[v as usize];
    if slot != u32::MAX {
        return Ok(slot);
    }
    let index = new_index(input, out)?;
    store.grow(&mut out.vertices, input.stride)?;
    out.vertices.extend_from_slice(input.attrs(v));
    out.remap[v as usize] = index;
    Ok(index)
}

fn new_index(input: &Input<'_>, out: &Output) -> Result<u32, Error> {
    u32::try_from(out.vertices.len() / input.stride)
        .ok()
        .filter(|&i| i != u32::MAX)
        .ok_or(Error::SizeOverflow)
}

fn push_vertex(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    values: &[f32],
) -> Result<u32, Error> {
    let index = new_index(input, out)?;
    store.grow(&mut out.vertices, input.stride)?;
    out.vertices.extend_from_slice(values);
    Ok(index)
}

fn emit_original(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    v: [u32; 3],
) -> Result<(), Error> {
    let mapped = [
        copy_vertex(input, store, out, v[0])?,
        copy_vertex(input, store, out, v[1])?,
        copy_vertex(input, store, out, v[2])?,
    ];
    store.grow(&mut out.indices, 3)?;
    out.indices.extend_from_slice(&mapped);
    Ok(())
}

// ---------------------------------------------------------------------------
// One card.

#[allow(clippy::too_many_arguments)]
fn process_card(
    input: &Input<'_>,
    store: &mut Store<'_>,
    work: &mut Work,
    out: &mut Output,
    s: &mut Scratch,
    memo: &mut BTreeMap<Vec<u32>, Fit>,
    polygons: &mut Vec<Point>,
    card: &[u32],
) -> Result<(), Error> {
    s.tris.clear();
    s.key.clear();
    let mut before = 0.0;
    for &t in card {
        let tri = Tri::new(input, t as usize);
        before += tri.area2.abs() * 0.5;
        store.push(&mut s.tris, tri)?;
        for v in tri.v {
            let a = input.attrs(v);
            store.push(&mut s.key, a[input.uv].to_bits())?;
            store.push(&mut s.key, a[input.uv + 1].to_bits())?;
        }
    }
    work.add(card.len())?;
    out.report.cards += 1;
    out.report.area_before += before;
    let fit = match memo.get(s.key.as_slice()) {
        Some(&fit) => fit,
        None => {
            let fit = fit_card(input, store, work, s, polygons)?;
            store.charge(checked_bytes(s.key.len(), 4)?.saturating_add(64))?;
            let mut key = Vec::new();
            key.try_reserve_exact(s.key.len())
                .map_err(|_| Error::AllocationFailed)?;
            key.extend_from_slice(&s.key);
            memo.insert(key, fit);
            fit
        }
    };
    out.report.opaque_texels = out.report.opaque_texels.saturating_add(fit.opaque);
    let (start, len) = match fit.kind {
        FitKind::Empty => {
            out.report.removed_cards += 1;
            return Ok(());
        }
        FitKind::Keep => return keep_card(input, store, out, s, before),
        FitKind::Polygon(start, len) => (start, len),
    };
    let q = &polygons[start..start + len];
    if convex_disk(store, s)? {
        if let Some(r) = affine_reference(input, store, s)? {
            let after = area2(q) * 0.5;
            if after >= before * (1.0 - MIN_GAIN) {
                return keep_card(input, store, out, s, before);
            }
            emit_polygon(input, store, out, s, q, r)?;
            out.report.trimmed_cards += 1;
            out.report.area_after += after;
            return Ok(());
        }
    }
    let after = clip_pieces(store, work, s, q)?;
    if after >= before * (1.0 - MIN_GAIN) {
        return keep_card(input, store, out, s, before);
    }
    emit_pieces(input, store, out, s)?;
    out.report.trimmed_cards += 1;
    out.report.split_cards += 1;
    out.report.area_after += after;
    Ok(())
}

fn keep_card(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    s: &Scratch,
    before: f64,
) -> Result<(), Error> {
    for tri in &s.tris {
        emit_original(input, store, out, tri.v)?;
    }
    out.report.unchanged_cards += 1;
    out.report.area_after += before;
    Ok(())
}

/// Visible-texel hull, then the smallest polygon within the budget.
fn fit_card(
    input: &Input<'_>,
    store: &mut Store<'_>,
    work: &mut Work,
    s: &mut Scratch,
    polygons: &mut Vec<Point>,
) -> Result<Fit, Error> {
    let mut scale: f64 = 1.0;
    let (mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY);
    s.points.clear();
    for tri in &s.tris {
        for p in tri.p {
            scale = scale.max(p[0].abs()).max(p[1].abs());
            ymin = ymin.min(p[1]);
            ymax = ymax.max(p[1]);
            store.push(&mut s.points, p)?;
        }
    }
    let pad = input.padding + scale * MARGIN_SCALE;
    let eps = 1e-9 * scale;
    convex_hull(store, work, &mut s.points, &mut s.dom)?;
    if s.dom.len() < 3 {
        return Ok(Fit {
            kind: FitKind::Keep,
            opaque: 0,
        });
    }

    // Leftmost and rightmost visible texel per row whose padded footprint
    // meets a card triangle; texels with centers inside a triangle counted.
    let j0 = to_i64(libm::ceil(ymin - 1.0 - pad))?;
    let j1 = to_i64(libm::floor(ymax + pad))?;
    let rows = usize::try_from(j1 - j0 + 1).map_err(|_| Error::SizeOverflow)?;
    store.fill(&mut s.rows, rows, (i64::MAX, i64::MIN))?;
    let mut opaque = 0u64;
    let threshold = input.threshold;
    for tri in &s.tris {
        let ty0 = tri.p[0][1].min(tri.p[1][1]).min(tri.p[2][1]);
        let ty1 = tri.p[0][1].max(tri.p[1][1]).max(tri.p[2][1]);
        let r0 = to_i64(libm::ceil(ty0 - 1.0 - pad))?;
        let r1 = to_i64(libm::floor(ty1 + pad))?;
        for j in r0..=r1 {
            let y = j as f64;
            let row = usize::try_from(j - j0).map_err(|_| Error::SizeOverflow)?;
            if let Some((a, b)) = strip_range(&tri.p, y - pad, y + 1.0 + pad) {
                let i0 = to_i64(libm::ceil(a - 1.0 - pad))?;
                let i1 = to_i64(libm::floor(b + pad))?;
                if i1 >= i0 {
                    work.add(usize::try_from(i1 - i0 + 1).map_err(|_| Error::SizeOverflow)?)?;
                }
                let (mut lo, mut hi) = s.rows[row];
                for i in i0..=i1 {
                    if input.mask.texel(i, j, input.wrap) >= threshold {
                        lo = lo.min(i);
                        hi = hi.max(i);
                    }
                }
                s.rows[row] = (lo, hi);
            }
            if let Some((c, d)) = strip_range(&tri.p, y + 0.5, y + 0.5) {
                let i0 = to_i64(libm::ceil(c - 0.5))?;
                let i1 = to_i64(libm::ceil(d - 0.5))? - 1;
                for i in i0..=i1 {
                    if input.mask.texel(i, j, input.wrap) >= threshold {
                        opaque += 1;
                    }
                }
            }
        }
    }
    s.points.clear();
    for (row, &(lo, hi)) in s.rows.iter().enumerate() {
        if lo > hi {
            continue;
        }
        let y = (j0 + i64::try_from(row).map_err(|_| Error::SizeOverflow)?) as f64;
        let (x0, x1) = (lo as f64 - pad, hi as f64 + 1.0 + pad);
        let (y0, y1) = (y - pad, y + 1.0 + pad);
        for p in [[x0, y0], [x0, y1], [x1, y0], [x1, y1]] {
            store.push(&mut s.points, p)?;
        }
    }
    let empty = Fit {
        kind: FitKind::Empty,
        opaque,
    };
    if s.points.is_empty() {
        return Ok(empty);
    }
    convex_hull(store, work, &mut s.points, &mut s.hull)?;
    clip_convex(store, &mut s.hull, &s.dom, &mut s.tmp)?;
    simplify(&mut s.hull);
    if s.hull.len() < 3 || area2(&s.hull) <= 0.0 {
        return Ok(empty);
    }
    reduce(work, &mut s.hull, &s.dom, eps)?;
    let found = smallest_cycle(input.budget, store, work, s, eps)?;
    if !found {
        return Ok(Fit {
            kind: FitKind::Keep,
            opaque,
        });
    }
    // Snap to outline corners and drop repeated points.
    let start = polygons.len();
    for i in 0..s.best.len() {
        let next = s.best[(i + 1) % s.best.len()] as usize;
        let n = s.edges.len();
        let mut p = s.table[s.best[i] as usize * n + next].apex;
        for &d in &s.dom {
            if (p[0] - d[0]).abs() <= eps && (p[1] - d[1]).abs() <= eps {
                p = d;
            }
        }
        if polygons.len() > start && polygons[polygons.len() - 1] == p {
            continue;
        }
        store.push(polygons, p)?;
    }
    while polygons.len() > start + 1 && polygons[polygons.len() - 1] == polygons[start] {
        polygons.pop();
    }
    let len = polygons.len() - start;
    if len < 3 {
        polygons.truncate(start);
        return Ok(Fit {
            kind: FitKind::Keep,
            opaque,
        });
    }
    Ok(Fit {
        kind: FitKind::Polygon(start, len),
        opaque,
    })
}

/// Monotone-chain convex hull, counter-clockwise, collinear points removed.
fn convex_hull(
    store: &mut Store<'_>,
    work: &mut Work,
    points: &mut [Point],
    hull: &mut Vec<Point>,
) -> Result<(), Error> {
    work.add(points.len().saturating_mul(16))?;
    points.sort_unstable_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    hull.clear();
    store.grow(hull, points.len().saturating_mul(2).saturating_add(1))?;
    for &p in points.iter() {
        while hull.len() >= 2
            && cross(
                sub(hull[hull.len() - 1], hull[hull.len() - 2]),
                sub(p, hull[hull.len() - 2]),
            ) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }
    let lower = hull.len() + 1;
    for &p in points.iter().rev().skip(1) {
        while hull.len() >= lower
            && cross(
                sub(hull[hull.len() - 1], hull[hull.len() - 2]),
                sub(p, hull[hull.len() - 2]),
            ) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }
    hull.pop();
    Ok(())
}

/// Clip a convex polygon to a convex counter-clockwise domain.
fn clip_convex(
    store: &mut Store<'_>,
    poly: &mut Vec<Point>,
    dom: &[Point],
    tmp: &mut Vec<Point>,
) -> Result<(), Error> {
    let m = dom.len();
    for i in 0..m {
        let (a, b) = (dom[i], dom[(i + 1) % m]);
        tmp.clear();
        let n = poly.len();
        store.grow(tmp, n.saturating_add(1))?;
        for k in 0..n {
            let (p, q) = (poly[k], poly[(k + 1) % n]);
            let (dp, dq) = (side(a, b, p), side(a, b, q));
            if dp >= 0.0 {
                tmp.push(p);
            }
            if (dp > 0.0 && dq < 0.0) || (dp < 0.0 && dq > 0.0) {
                tmp.push(lerp(p, q, dp / (dp - dq)));
            }
        }
        core::mem::swap(poly, tmp);
        if poly.is_empty() {
            break;
        }
    }
    Ok(())
}

/// Drop repeated points and non-convex (collinear) corners.
fn simplify(poly: &mut Vec<Point>) {
    loop {
        let n = poly.len();
        if n < 3 {
            return;
        }
        let found = (0..n).find(|&i| {
            let prev = poly[(i + n - 1) % n];
            let next = poly[(i + 1) % n];
            poly[i] == prev || cross(sub(poly[i], prev), sub(next, poly[i])) <= 0.0
        });
        match found {
            Some(i) => {
                poly.remove(i);
            }
            None => return,
        }
    }
}

/// Greedily remove the edge whose removal adds the least area while the hull
/// has more than [`REDUCE_TO`] edges and the new corner stays in the domain.
fn reduce(work: &mut Work, poly: &mut Vec<Point>, dom: &[Point], eps: f64) -> Result<(), Error> {
    while poly.len() > REDUCE_TO {
        let n = poly.len();
        work.add(n.saturating_mul(dom.len() + 4))?;
        let mut best: Option<(f64, usize, Point)> = None;
        for i in 0..n {
            let prev = poly[(i + n - 1) % n];
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            let after = poly[(i + 2) % n];
            let (dp, dn) = (sub(a, prev), sub(after, b));
            let c = cross(dp, dn);
            if c <= 0.0 {
                continue;
            }
            let t = cross(sub(b, a), dn) / c;
            let apex = [a[0] + t * dp[0], a[1] + t * dp[1]];
            if !t.is_finite() || t < 0.0 || !inside(dom, apex, eps) {
                continue;
            }
            let added = cross(sub(b, a), sub(apex, a)).abs();
            if best.is_none_or(|(cost, _, _)| added < cost) {
                best = Some((added, i, apex));
            }
        }
        let Some((_, i, apex)) = best else {
            return Ok(());
        };
        poly[i] = apex;
        poly.remove((i + 1) % n);
    }
    Ok(())
}

/// Exact search over cycles of supporting lines: hull edges plus zero-length
/// edges parallel to the card outline or bisecting a hull corner. Leaves the
/// chosen edges in `s.best`.
fn smallest_cycle(
    budget: usize,
    store: &mut Store<'_>,
    work: &mut Work,
    s: &mut Scratch,
    eps: f64,
) -> Result<bool, Error> {
    let hull = &s.hull;
    let m = hull.len();
    let md = s.dom.len();
    s.edges.clear();
    for i in 0..m {
        let v = hull[i];
        let d_in = sub(v, hull[(i + m - 1) % m]);
        let d_out = sub(hull[(i + 1) % m], v);
        s.dirs.clear();
        // The exterior-angle bisector touches the hull at this vertex only.
        let (li, lo) = (libm::sqrt(dot(d_in, d_in)), libm::sqrt(dot(d_out, d_out)));
        let bisector = [d_in[0] / li + d_out[0] / lo, d_in[1] / li + d_out[1] / lo];
        for k in 0..=md {
            let e = if k == md {
                bisector
            } else {
                sub(s.dom[(k + 1) % md], s.dom[k])
            };
            if cross(d_in, e) > 0.0 && cross(e, d_out) > 0.0 {
                let at = s
                    .dirs
                    .iter()
                    .position(|&x| cross(e, x) > 0.0)
                    .unwrap_or(s.dirs.len());
                store.grow(&mut s.dirs, 1)?;
                s.dirs.insert(at, e);
            }
        }
        for &d in &s.dirs {
            store.push(&mut s.edges, Edge { s: v, d, e: v })?;
        }
        store.push(
            &mut s.edges,
            Edge {
                s: v,
                d: d_out,
                e: hull[(i + 1) % m],
            },
        )?;
    }
    let n = s.edges.len();
    let cells = n.checked_mul(n).ok_or(Error::SizeOverflow)?;
    work.add(cells.saturating_mul(n))?;
    store.fill(
        &mut s.table,
        cells,
        Pair {
            ok: false,
            cap: 0.0,
            apex: [0.0; 2],
        },
    )?;
    for i in 0..n {
        let ei = s.edges[i];
        for step in 1..n {
            let j = (i + step) % n;
            let ej = s.edges[j];
            let cell = &mut s.table[i * n + j];
            if step == 1 {
                *cell = Pair {
                    ok: true,
                    cap: 0.0,
                    apex: ei.e,
                };
                continue;
            }
            let c = cross(ei.d, ej.d);
            if c <= 0.0 {
                continue;
            }
            let t = cross(sub(ej.s, ei.s), ej.d) / c;
            let apex = [ei.s[0] + t * ei.d[0], ei.s[1] + t * ei.d[1]];
            if !t.is_finite() || !inside(&s.dom, apex, eps) {
                continue;
            }
            // Area between the hull chain e_i .. s_j and the apex.
            let mut sum = 0.0;
            let mut prev = sub(ei.e, apex);
            for k in 1..step {
                let p = sub(s.edges[(i + k) % n].e, apex);
                sum += cross(prev, p);
                prev = p;
            }
            *cell = Pair {
                ok: true,
                cap: sum.abs() * 0.5,
                apex,
            };
        }
    }

    let k = budget.min(n);
    let layers = k + 1;
    let size = layers.checked_mul(n).ok_or(Error::SizeOverflow)?;
    work.add(size.saturating_mul(n).saturating_mul(n))?;
    store.fill(&mut s.cost, size, f64::INFINITY)?;
    store.fill(&mut s.parent, size, u32::MAX)?;
    s.best.clear();
    let mut best = f64::INFINITY;
    for start in 0..n {
        s.cost.fill(f64::INFINITY);
        s.cost[n + start] = 0.0;
        for c in 1..k {
            for j in start..n {
                let f = s.cost[c * n + j];
                if !f.is_finite() {
                    continue;
                }
                for j2 in j + 1..n {
                    let pair = s.table[j * n + j2];
                    if !pair.ok {
                        continue;
                    }
                    let cand = f + pair.cap;
                    let slot = (c + 1) * n + j2;
                    if cand < s.cost[slot] {
                        s.cost[slot] = cand;
                        s.parent[slot] = u32::try_from(j).map_err(|_| Error::SizeOverflow)?;
                    }
                }
            }
        }
        for c in 3..=k {
            for j in start + 1..n {
                let f = s.cost[c * n + j];
                let pair = s.table[j * n + start];
                if !f.is_finite() || !pair.ok {
                    continue;
                }
                let total = f + pair.cap;
                if total < best {
                    best = total;
                    s.seq.clear();
                    let mut cur = j;
                    for level in (2..=c).rev() {
                        store.push(
                            &mut s.seq,
                            u32::try_from(cur).map_err(|_| Error::SizeOverflow)?,
                        )?;
                        cur = s.parent[level * n + cur] as usize;
                    }
                    store.push(
                        &mut s.seq,
                        u32::try_from(cur).map_err(|_| Error::SizeOverflow)?,
                    )?;
                    s.seq.reverse();
                    s.best.clear();
                    store.grow(&mut s.best, s.seq.len())?;
                    s.best.extend_from_slice(&s.seq);
                }
            }
        }
    }
    Ok(!s.best.is_empty())
}

// ---------------------------------------------------------------------------
// Output for one fitted card.

/// Triangles with one orientation whose boundary is one convex loop over
/// distinct vertices: they cover the convex hull of their UVs exactly once.
fn convex_disk(store: &mut Store<'_>, s: &mut Scratch) -> Result<bool, Error> {
    let positive = s.tris[0].area2 > 0.0;
    if s.tris.iter().any(|t| (t.area2 > 0.0) != positive) {
        return Ok(false);
    }
    s.directed.clear();
    for tri in &s.tris {
        for k in 0..3 {
            store.push(&mut s.directed, (tri.v[k], tri.v[(k + 1) % 3]))?;
        }
    }
    s.directed.sort_unstable();
    if s.directed.windows(2).any(|w| w[0] == w[1]) {
        return Ok(false);
    }
    s.boundary.clear();
    for &(a, b) in &s.directed {
        if s.directed.binary_search(&(b, a)).is_err() {
            store.push(&mut s.boundary, (a, b))?;
        }
    }
    let count = s.boundary.len();
    if !(3..=MAX_OUTLINE).contains(&count) || s.boundary.windows(2).any(|w| w[0].0 == w[1].0) {
        return Ok(false);
    }
    // One cycle through every boundary edge.
    let first = s.boundary[0].0;
    let mut cur = first;
    for step in 0..count {
        let Ok(at) = s.boundary.binary_search_by(|e| e.0.cmp(&cur)) else {
            return Ok(false);
        };
        cur = s.boundary[at].1;
        if cur == first && step + 1 != count {
            return Ok(false);
        }
    }
    if cur != first {
        return Ok(false);
    }
    // Convex: every boundary vertex on the inner side of every boundary edge.
    let point = |v: u32| s.tris.iter().find_map(|t| t.point_of(v));
    let sign = if positive { 1.0 } else { -1.0 };
    for &(a, b) in &s.boundary {
        let (Some(pa), Some(pb)) = (point(a), point(b)) else {
            return Ok(false);
        };
        for &(z, _) in &s.boundary {
            let Some(pz) = point(z) else {
                return Ok(false);
            };
            if sign * side(pa, pb, pz) < 0.0 {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Barycentric weights of `p` in `tri`.
fn bary(tri: &Tri, p: Point) -> [f64; 3] {
    let [a, b, c] = tri.p;
    let l1 = cross(sub(p, a), sub(c, a)) / tri.area2;
    let l2 = cross(sub(b, a), sub(p, a)) / tri.area2;
    [1.0 - l1 - l2, l1, l2]
}

/// Reference triangle when every card attribute is one affine function of UV.
fn affine_reference(
    input: &Input<'_>,
    store: &mut Store<'_>,
    s: &mut Scratch,
) -> Result<Option<usize>, Error> {
    let mut r = 0;
    for (i, tri) in s.tris.iter().enumerate() {
        if tri.area2.abs() > s.tris[r].area2.abs() {
            r = i;
        }
    }
    let stride = input.stride;
    store.fill(
        &mut s.stats,
        stride,
        [f64::INFINITY, f64::NEG_INFINITY, 0.0],
    )?;
    for tri in &s.tris {
        for v in tri.v {
            for (c, &x) in input.attrs(v).iter().enumerate() {
                let x = f64::from(x);
                let st = &mut s.stats[c];
                st[0] = st[0].min(x);
                st[1] = st[1].max(x);
                st[2] = st[2].max(x.abs());
            }
        }
    }
    let reference = s.tris[r];
    let rv = reference.v.map(|v| input.attrs(v));
    for tri in &s.tris {
        for (k, v) in tri.v.into_iter().enumerate() {
            let w = bary(&reference, tri.p[k]);
            for (c, &x) in input.attrs(v).iter().enumerate() {
                if c == input.uv || c == input.uv + 1 {
                    continue;
                }
                let predicted = w[0] * f64::from(rv[0][c])
                    + w[1] * f64::from(rv[1][c])
                    + w[2] * f64::from(rv[2][c]);
                let st = s.stats[c];
                let tolerance = 1e-5 * (st[1] - st[0]) + 4.0 * f64::from(f32::EPSILON) * st[2];
                if (predicted - f64::from(x)).abs() > tolerance {
                    return Ok(None);
                }
            }
        }
    }
    Ok(Some(r))
}

fn write_uv(input: &Input<'_>, values: &mut [f32], p: Point) {
    values[input.uv] = (p[0] / input.w) as f32;
    values[input.uv + 1] = (p[1] / input.h) as f32;
}

fn bary_vertex(
    input: &Input<'_>,
    store: &mut Store<'_>,
    values: &mut Vec<f32>,
    tri: &Tri,
    p: Point,
) -> Result<(), Error> {
    let w = bary(tri, p);
    let a = tri.v.map(|v| input.attrs(v));
    values.clear();
    store.grow(values, input.stride)?;
    for ((&x, &y), &z) in a[0].iter().zip(a[1]).zip(a[2]) {
        values.push((w[0] * f64::from(x) + w[1] * f64::from(y) + w[2] * f64::from(z)) as f32);
    }
    write_uv(input, values, p);
    Ok(())
}

fn emit_triangle(
    store: &mut Store<'_>,
    out: &mut Output,
    points: [Point; 3],
    ids: [u32; 3],
    positive: bool,
) -> Result<(), Error> {
    if cross(sub(points[1], points[0]), sub(points[2], points[0])) == 0.0 {
        return Ok(());
    }
    store.grow(&mut out.indices, 3)?;
    if positive {
        out.indices.extend_from_slice(&ids);
    } else {
        out.indices.extend_from_slice(&[ids[0], ids[2], ids[1]]);
    }
    Ok(())
}

/// One fan-triangulated polygon interpolated with the reference triangle.
fn emit_polygon(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    s: &mut Scratch,
    q: &[Point],
    r: usize,
) -> Result<(), Error> {
    let reference = s.tris[r];
    let positive = reference.area2 > 0.0;
    s.seq.clear();
    for &p in q {
        let corner = s
            .tris
            .iter()
            .find_map(|t| (0..3).find(|&k| t.p[k] == p).map(|k| t.v[k]));
        let id = match corner {
            Some(v) => copy_vertex(input, store, out, v)?,
            None => {
                bary_vertex(input, store, &mut s.values, &reference, p)?;
                push_vertex(input, store, out, &s.values)?
            }
        };
        store.push(&mut s.seq, id)?;
    }
    for m in 1..q.len() - 1 {
        emit_triangle(
            store,
            out,
            [q[0], q[m], q[m + 1]],
            [s.seq[0], s.seq[m], s.seq[m + 1]],
            positive,
        )?;
    }
    Ok(())
}

/// Clip the fitted polygon by each card triangle into `s.pieces`; returns the total area.
fn clip_pieces(
    store: &mut Store<'_>,
    work: &mut Work,
    s: &mut Scratch,
    q: &[Point],
) -> Result<f64, Error> {
    s.pieces.clear();
    s.ends.clear();
    let mut total = 0.0;
    for ti in 0..s.tris.len() {
        let tri = s.tris[ti];
        let local = u32::try_from(ti).map_err(|_| Error::SizeOverflow)?;
        work.add(q.len().saturating_add(6).saturating_mul(3))?;
        s.a.clear();
        for (k, &p) in q.iter().enumerate() {
            let k = u32::try_from(k).map_err(|_| Error::SizeOverflow)?;
            store.push(
                &mut s.a,
                WVert {
                    p,
                    key: Key::Corner(k, local),
                    line: Line::Fit(k),
                },
            )?;
        }
        for e in 0..3 {
            let (lo, hi, plo, phi) = tri.edge(e);
            let sign = tri.sigma[e];
            s.b.clear();
            let n = s.a.len();
            store.grow(&mut s.b, n.saturating_mul(2))?;
            for m in 0..n {
                let cur = s.a[m];
                let next = s.a[(m + 1) % n];
                let dc = sign * side(plo, phi, cur.p);
                let dn = sign * side(plo, phi, next.p);
                if dc >= 0.0 {
                    let line = if dc == 0.0 && dn < 0.0 {
                        Line::Edge(lo, hi)
                    } else {
                        cur.line
                    };
                    s.b.push(WVert { line, ..cur });
                }
                if (dc > 0.0 && dn < 0.0) || (dc < 0.0 && dn > 0.0) {
                    let (p, key) = crossing(q, &tri, cur, next, (lo, hi, plo, phi))?;
                    let line = if dc > 0.0 {
                        Line::Edge(lo, hi)
                    } else {
                        cur.line
                    };
                    s.b.push(WVert { p, key, line });
                }
            }
            core::mem::swap(&mut s.a, &mut s.b);
            if s.a.is_empty() {
                break;
            }
        }
        s.a.dedup_by(|x, y| x.key == y.key);
        while s.a.len() > 1 && s.a[0].key == s.a[s.a.len() - 1].key {
            s.a.pop();
        }
        if s.a.len() < 3 {
            continue;
        }
        for w in &mut s.a {
            if let Key::Corner(k, _) = w.key {
                if (0..3).any(|e| {
                    let (_, _, plo, phi) = tri.edge(e);
                    side(plo, phi, w.p) == 0.0
                }) {
                    w.key = Key::Corner(k, u32::MAX);
                }
            }
        }
        let mut twice = 0.0;
        let origin = s.a[0].p;
        for pair in s.a.windows(2) {
            twice += cross(sub(pair[0].p, origin), sub(pair[1].p, origin));
        }
        total += twice * 0.5;
        store.grow(&mut s.pieces, s.a.len())?;
        s.pieces.extend_from_slice(&s.a);
        store.push(&mut s.ends, (s.pieces.len(), ti))?;
    }
    Ok(total)
}

/// Intersection of the working edge `cur -> next` with a canonical triangle
/// edge, computed from the original lines so neighbours agree bit for bit.
fn crossing(
    q: &[Point],
    tri: &Tri,
    cur: WVert,
    next: WVert,
    edge: (u32, u32, Point, Point),
) -> Result<(Point, Key), Error> {
    let (lo, hi, plo, phi) = edge;
    match cur.line {
        Line::Fit(k) => {
            let m = q.len();
            let a = q[k as usize];
            let b = q[(k as usize + 1) % m];
            let (da, db) = (side(plo, phi, a), side(plo, phi, b));
            let p = if da != db {
                lerp(a, b, (da / (da - db)).clamp(0.0, 1.0))
            } else {
                let (dc, dn) = (side(plo, phi, cur.p), side(plo, phi, next.p));
                lerp(cur.p, next.p, dc / (dc - dn))
            };
            Ok((p, Key::Cross(k, lo, hi)))
        }
        Line::Edge(a, b) => {
            let common = if a == lo || a == hi { a } else { b };
            if common != lo && common != hi {
                return Err(Error::NumericalFailure);
            }
            let p = tri.point_of(common).ok_or(Error::NumericalFailure)?;
            Ok((p, Key::Original(common)))
        }
    }
}

fn emit_pieces(
    input: &Input<'_>,
    store: &mut Store<'_>,
    out: &mut Output,
    s: &mut Scratch,
) -> Result<(), Error> {
    s.keymap.clear();
    let mut begin = 0;
    for e in 0..s.ends.len() {
        let (end, ti) = s.ends[e];
        let tri = s.tris[ti];
        s.seq.clear();
        for w in begin..end {
            let vert = s.pieces[w];
            let id = match vert.key {
                Key::Original(v) => copy_vertex(input, store, out, v)?,
                key => match s.keymap.get(&key) {
                    Some(&id) => id,
                    None => {
                        match key {
                            Key::Cross(_, lo, hi) => {
                                let (plo, phi) = (input.texel(lo), input.texel(hi));
                                let d = sub(phi, plo);
                                let t = dot(sub(vert.p, plo), d) / dot(d, d);
                                let (alo, ahi) = (input.attrs(lo), input.attrs(hi));
                                s.values.clear();
                                store.grow(&mut s.values, input.stride)?;
                                for c in 0..input.stride {
                                    let (x, y) = (f64::from(alo[c]), f64::from(ahi[c]));
                                    s.values.push((x + t * (y - x)) as f32);
                                }
                                write_uv(input, &mut s.values, vert.p);
                            }
                            _ => bary_vertex(input, store, &mut s.values, &tri, vert.p)?,
                        }
                        let id = push_vertex(input, store, out, &s.values)?;
                        store.charge(64)?;
                        s.keymap.insert(key, id);
                        id
                    }
                },
            };
            store.push(&mut s.seq, id)?;
        }
        let positive = tri.area2 > 0.0;
        let piece = &s.pieces[begin..end];
        for m in 1..piece.len() - 1 {
            emit_triangle(
                store,
                out,
                [piece[0].p, piece[m].p, piece[m + 1].p],
                [s.seq[0], s.seq[m], s.seq[m + 1]],
                positive,
            )?;
        }
        begin = end;
    }
    Ok(())
}
