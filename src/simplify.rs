//! Scalar edge-collapse simplification translated from upstream simplifier.cpp.
use crate::math::sqrt;
use crate::workspace::{checked_bytes, topology, Work};
use crate::{validate_vertex_flags, Attributes, Error, Positions, VertexFlags, Workspace};
use alloc::vec::Vec;
#[path = "simplify_extra.rs"]
mod extra;
pub(crate) use extra::SloppyScratch;
pub use extra::{
    simplify_points, simplify_points_into, simplify_prune, simplify_prune_into, simplify_sloppy,
    simplify_sloppy_into, simplify_with_update,
};

// Counted work for one simplifier phase. `Work` checks every visit against the
// remaining budget. `Prepaid` is used only after `metered!` has established that
// the remaining budget covers an upper bound of the phase's visits: no visit can
// then exhaust the budget, so the phase counts locally and charges its exact
// total (or, on an error, the visited prefix) once. Errors, consumed work and
// partial side effects are identical to visit-by-visit checking.
trait Meter {
    fn add(&mut self, count: usize) -> Result<(), Error>;
    fn precharge(&mut self, count: usize, units: usize) -> Result<bool, Error>;
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error>;
}
impl Meter for Work {
    #[inline(always)]
    fn add(&mut self, count: usize) -> Result<(), Error> {
        Work::add(self, count)
    }
    #[inline(always)]
    fn precharge(&mut self, count: usize, units: usize) -> Result<bool, Error> {
        Work::precharge(self, count, units)
    }
    #[inline(always)]
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        Work::scan(self, values, visit)
    }
}
struct Prepaid(usize);
impl Meter for Prepaid {
    #[inline(always)]
    fn add(&mut self, count: usize) -> Result<(), Error> {
        self.0 += count;
        Ok(())
    }
    #[inline(always)]
    fn precharge(&mut self, count: usize, units: usize) -> Result<bool, Error> {
        self.0 += count.checked_mul(units).ok_or(Error::SizeOverflow)?;
        Ok(true)
    }
    #[inline(always)]
    fn scan<T>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
        mut visit: impl FnMut(T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = values.len();
        for (i, value) in values.enumerate() {
            if let Err(error) = visit(value) {
                self.0 += i + 1;
                return Err(error);
            }
        }
        self.0 += count;
        Ok(())
    }
}
// Evaluate `$body` with `$m` bound to a meter. `$bound` is an upper bound of the
// visits `$body` counts, or `None` when no cheap bound exists.
macro_rules! metered {
    ($work:expr, $bound:expr, |$m:ident| $body:expr) => {{
        let work: &mut Work = $work;
        match $bound {
            Some(bound) if work.covers(bound)? => {
                let mut tally = Prepaid(0);
                let result = {
                    let $m = &mut tally;
                    $body
                };
                debug_assert!(tally.0 <= bound);
                work.add(tally.0)?;
                result
            }
            _ => {
                let $m = work;
                $body
            }
        }
    }};
}

/// Stable simplification options supported by this port.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimplifyOptions(u32);
impl SimplifyOptions {
    /// Default topology and relative-error behavior.
    pub const EMPTY: Self = Self(0);
    /// Prevent movement of geometric border vertices.
    pub const LOCK_BORDER: Self = Self(1);
    /// Use first-referenced vertices and subset extents for simplification.
    pub const SPARSE: Self = Self(2);
    /// Interpret the error limit and result in position units.
    pub const ERROR_ABSOLUTE: Self = Self(4);
    /// Remove disconnected components incrementally within the error budget.
    pub const PRUNE: Self = Self(8);
    /// Apply stronger positional regularization.
    pub const REGULARIZE: Self = Self(16);
    /// Allow collapses across unprotected attribute discontinuities.
    pub const PERMISSIVE: Self = Self(32);
    /// Apply light positional regularization.
    pub const REGULARIZE_LIGHT: Self = Self(64);
    /// Preserve geometric folds using additional edge quadrics.
    #[cfg(feature = "experimental")]
    pub const PRESERVE_FOLDS: Self = Self(128);
    /// Clamp each attribute quadric error to its accumulated area.
    #[cfg(feature = "experimental")]
    pub const ERROR_CLAMPED: Self = Self(256);
    /// Validate a raw option mask; unsupported options are rejected.
    pub const fn from_bits(bits: u32) -> Result<Self, Error> {
        let mask = if cfg!(feature = "experimental") {
            511
        } else {
            127
        };
        if bits & !mask != 0 {
            Err(Error::UnknownFlags)
        } else {
            Ok(Self(bits))
        }
    }
    /// Return the upstream option mask.
    pub const fn bits(self) -> u32 {
        self.0
    }
    /// Whether all bits in `other` are present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SimplifyOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for SimplifyOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
/// Parameters for edge-collapse simplification.
#[derive(Clone, Copy, Debug)]
pub struct SimplifySettings {
    /// Requested index count, at most the input count; need not be divisible by three.
    pub target_index_count: usize,
    /// Nonnegative finite linear error limit; relative to the mesh extent by default.
    pub target_error: f32,
    /// Topology, regularization and error-unit options.
    pub options: SimplifyOptions,
}
/// Simplified topology referencing the original vertex array.
#[derive(Debug)]
pub struct SimplifiedMesh {
    /// Remaining triangles, in upstream order. The requested target may not be reached.
    pub indices: Vec<u32>,
    /// Linear error in the units selected by the settings.
    pub error: f32,
}
/// Caller-buffer simplification result.
#[derive(Clone, Copy, Debug)]
pub struct SimplifyResult {
    /// Number of initialized result indices in the destination prefix.
    pub index_count: usize,
    /// Linear error in the units selected by the settings.
    pub error: f32,
}
/// Return meshopt_simplifyScale's maximum axis extent, without an epsilon clamp.
/// Empty and coincident positions return zero; non-finite inputs and overflowing
/// extents return an error. This operation allocates no memory.
pub fn simplify_scale(positions: Positions<'_>) -> Result<f32, Error> {
    Ok(bounds(positions)?.1)
}
fn bounds(p: Positions<'_>) -> Result<([f32; 3], f32), Error> {
    let mut lo = [f32::MAX; 3];
    let mut hi = [-f32::MAX; 3];
    p.for_each(|v| {
        for j in 0..3 {
            if !v[j].is_finite() {
                return Err(Error::InvalidParameter);
            }
            if lo[j] > v[j] {
                lo[j] = v[j];
            }
            if hi[j] < v[j] {
                hi[j] = v[j];
            }
        }
        Ok(())
    })?;
    if p.is_empty() {
        return Ok((lo, 0.0));
    }
    let mut extent = 0.0;
    for j in 0..3 {
        let d = hi[j] - lo[j];
        if d >= extent {
            extent = d;
        }
    }
    if !extent.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok((lo, extent))
}
/// Allocate meshopt_simplify's result, preserving upstream ordering and error bits.
/// No welding, fetch compaction or cache optimization is applied to the result.
pub fn simplify(
    indices: &[u32],
    positions: Positions<'_>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    allocating(indices, positions, None, &[], None, settings, workspace)
}
/// Allocate meshopt_simplifyWithAttributes's result. All three vertex flags are
/// supported; weights must be finite and nonnegative, with one per component.
/// Zero-weight components do not contribute to error. All supplied values must
/// be finite, including unused vertices and zero-weight components.
pub fn simplify_with_attributes(
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Attributes<'_>,
    weights: &[f32],
    vertex_flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    allocating(
        indices,
        positions,
        Some(attributes),
        weights,
        vertex_flags,
        settings,
        workspace,
    )
}
fn allocating(
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    ws: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    let mut work = ws.begin();
    let result = (|| {
        validate(
            indices, positions, attributes, weights, flags, settings, &mut work,
        )?;
        ws.prepare([0; 4], checked_bytes(indices.len(), 4)?)?;
        let mut out = crate::workspace::output(indices.len())?;
        let output_bytes = checked_bytes(out.capacity(), 4)?;
        let r = run(
            &mut out,
            indices,
            positions,
            attributes,
            weights,
            flags,
            settings,
            ws,
            &mut work,
            output_bytes,
        )?;
        out.truncate(r.index_count);
        Ok(SimplifiedMesh {
            indices: out,
            error: r.error,
        })
    })();
    ws.finish(&work);
    result
}
/// Write meshopt_simplify's result into a caller buffer of at least input length.
/// The tail after the input length is preserved. A late resource or numerical
/// error may modify the input-length prefix; use the allocating API for atomicity.
pub fn simplify_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    into(
        destination,
        indices,
        positions,
        None,
        &[],
        None,
        settings,
        workspace,
    )
}
/// Caller-buffer attribute simplification, with the same contracts as
/// [`simplify_with_attributes`] and the mutation guarantee of [`simplify_into`].
#[allow(clippy::too_many_arguments)]
pub fn simplify_with_attributes_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Attributes<'_>,
    weights: &[f32],
    vertex_flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    into(
        destination,
        indices,
        positions,
        Some(attributes),
        weights,
        vertex_flags,
        settings,
        workspace,
    )
}
#[allow(clippy::too_many_arguments)]
fn into(
    out: &mut [u32],
    indices: &[u32],
    p: Positions<'_>,
    a: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    ws: &mut Workspace,
) -> Result<SimplifyResult, Error> {
    let mut work = ws.begin();
    let result = (|| {
        if out.len() < indices.len() {
            return Err(Error::BufferTooSmall);
        }
        validate(indices, p, a, weights, flags, settings, &mut work)?;
        run(
            out, indices, p, a, weights, flags, settings, ws, &mut work, 0,
        )
    })();
    ws.finish(&work);
    result
}
#[allow(clippy::too_many_arguments)]
fn validate(
    indices: &[u32],
    p: Positions<'_>,
    a: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    s: SimplifySettings,
    work: &mut Work,
) -> Result<(), Error> {
    topology(indices, p.len(), work)?;
    if s.target_index_count > indices.len() || !s.target_error.is_finite() || s.target_error < 0.0 {
        return Err(Error::InvalidParameter);
    }
    validate_vertex_flags(flags, p.len())?;
    if let Some(a) = a {
        if a.len() != p.len() || a.components() != weights.len() {
            return Err(Error::InvalidLayout);
        }
    }
    work.scan(weights.iter().copied(), |w| {
        if !w.is_finite() || w < 0.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(())
    })?;
    if a.is_none() {
        return p.for_each_counted(work, |v| {
            if v.iter().any(|v| !v.is_finite()) {
                Err(Error::InvalidParameter)
            } else {
                Ok(())
            }
        });
    }
    for i in 0..p.len() {
        work.add(1)?;
        if p.at(i)?.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidParameter);
        }
        if let Some(a) = a {
            if let Some(values) = a.float_record(i) {
                work.scan(values.iter().copied(), |value| {
                    if value.is_finite() {
                        Ok(())
                    } else {
                        Err(Error::InvalidParameter)
                    }
                })?;
            } else {
                work.scan(0..a.components(), |k| {
                    if !a.get(i, k).ok_or(Error::InvalidLayout)?.is_finite() {
                        return Err(Error::InvalidParameter);
                    }
                    Ok(())
                })?;
            }
        }
    }
    Ok(())
}
#[derive(Clone, Copy, Default, Debug)]
struct V {
    x: f32,
    y: f32,
    z: f32,
}
impl From<[f32; 3]> for V {
    fn from(v: [f32; 3]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}
impl V {
    #[inline(always)]
    fn sub(self, b: Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
    #[inline(always)]
    fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    #[inline(always)]
    fn cross(self, b: Self) -> Self {
        Self {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
    #[inline(always)]
    fn normalize(&mut self, cache: &mut crate::math::SqrtCache) -> f32 {
        let l = cache.sqrt(self.dot(*self));
        if l > 0.0 {
            self.x /= l;
            self.y /= l;
            self.z /= l;
        }
        l
    }
}
#[derive(Clone, Copy, Default)]
struct Q {
    a00: f32,
    a11: f32,
    a22: f32,
    a10: f32,
    a20: f32,
    a21: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    c: f32,
    w: f32,
    // Used only by positional quadrics after their accumulation phase.
    inverse_weight: f32,
}
impl Q {
    #[inline(always)]
    fn add(&mut self, r: Self) {
        self.a00 += r.a00;
        self.a11 += r.a11;
        self.a22 += r.a22;
        self.a10 += r.a10;
        self.a20 += r.a20;
        self.a21 += r.a21;
        self.b0 += r.b0;
        self.b1 += r.b1;
        self.b2 += r.b2;
        self.c += r.c;
        self.w += r.w;
    }
    #[inline(always)]
    fn eval(self, v: V) -> f32 {
        let mut rx = (self.b0 + self.a10 * v.y) * 2.0;
        let mut ry = (self.b1 + self.a21 * v.z) * 2.0;
        let mut rz = (self.b2 + self.a20 * v.x) * 2.0;
        rx += self.a00 * v.x;
        ry += self.a11 * v.y;
        rz += self.a22 * v.z;
        self.c + rx * v.x + ry * v.y + rz * v.z
    }
    #[inline(always)]
    fn error(self, v: V) -> f32 {
        self.eval(v).abs() * self.inverse_weight
    }
    fn cache_inverse_weight(&mut self) {
        self.inverse_weight = if self.w == 0.0 { 0.0 } else { 1.0 / self.w };
    }
    #[inline(always)]
    fn plane(n: V, d: f32, w: f32) -> Self {
        let aw = n.x * w;
        let bw = n.y * w;
        let cw = n.z * w;
        let dw = d * w;
        Self {
            a00: n.x * aw,
            a11: n.y * bw,
            a22: n.z * cw,
            a10: n.x * bw,
            a20: n.x * cw,
            a21: n.y * cw,
            b0: n.x * dw,
            b1: n.y * dw,
            b2: n.z * dw,
            c: d * dw,
            w,
            inverse_weight: 0.0,
        }
    }
    #[inline(always)]
    fn point(p: V, w: f32) -> Self {
        Self {
            a00: w,
            a11: w,
            a22: w,
            b0: -p.x * w,
            b1: -p.y * w,
            b2: -p.z * w,
            c: p.dot(p) * w,
            w,
            ..Self::default()
        }
    }
    #[inline(always)]
    fn triangle(a: V, b: V, c: V, w: f32, cache: &mut crate::math::SqrtCache) -> Self {
        let mut n = b.sub(a).cross(c.sub(a));
        let area = n.normalize(cache);
        Self::plane(n, -n.dot(a), cache.sqrt(area) * w)
    }
    #[inline(always)]
    fn edge(a: V, b: V, c: V, w: f32, cache: &mut crate::math::SqrtCache) -> Self {
        let p10 = b.sub(a);
        let ls = p10.dot(p10);
        let p20 = c.sub(a);
        let proj = p20.dot(p10);
        let mut perp = V {
            x: p20.x * ls - p10.x * proj,
            y: p20.y * ls - p10.y * proj,
            z: p20.z * ls - p10.z * proj,
        };
        perp.normalize(cache);
        Self::plane(perp, -perp.dot(a), cache.sqrt(ls) * w)
    }
    #[inline(always)]
    fn finite(self) -> bool {
        [
            self.a00, self.a11, self.a22, self.a10, self.a20, self.a21, self.b0, self.b1, self.b2,
            self.c, self.w,
        ]
        .iter()
        .all(|x| x.is_finite())
    }
}
#[derive(Clone, Copy, Default)]
struct G {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl G {
    fn add(&mut self, r: Self) {
        self.x += r.x;
        self.y += r.y;
        self.z += r.z;
        self.w += r.w;
    }
}
#[derive(Clone, Copy, Default)]
struct Edge {
    next: u32,
    prev: u32,
}
#[derive(Clone, Copy, Default)]
struct Collapse {
    a: u32,
    b: u32,
    error: f32,
}
const NONE: usize = u32::MAX as usize;
const MAN: u8 = 0;
const BORDER: u8 = 1;
const SEAM: u8 = 2;
const COMPLEX: u8 = 3;
const FRINGE: u8 = 4;
const LOCKED: u8 = 5;
const CAN: [[u8; 6]; 6] = [
    [1, 1, 1, 1, 1, 1],
    [0, 1, 0, 0, 1, 1],
    [0, 0, 1, 0, 0, 1],
    [0, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 1, 0],
    [0; 6],
];
const OPP: [[u8; 6]; 6] = [
    [1; 6],
    [1, 0, 1, 0, 0, 0],
    [1, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0],
];
// Collapse-candidate rules for an ordered kind pair, precomputed from CAN/OPP.
const PICK_ANY: u8 = 1;
const PICK_BOTH: u8 = 2;
const PICK_FORWARD: u8 = 4;
const PICK_OPPOSITE: u8 = 8;
const PICK_CHECK_FORWARD: u8 = 16;
const PICK_CHECK_BACK: u8 = 32;
const PICK: [u8; 64] = {
    let mut table = [0; 64];
    let mut k0 = 0;
    while k0 < 6 {
        let mut k1 = 0;
        while k1 < 6 {
            let forward = CAN[k0][k1] != 0;
            let reverse = CAN[k1][k0] != 0;
            let mut rule = 0;
            if forward || reverse {
                rule |= PICK_ANY;
            }
            if forward && reverse {
                rule |= PICK_BOTH;
            }
            if forward {
                rule |= PICK_FORWARD;
            }
            if OPP[k0][k1] != 0 {
                rule |= PICK_OPPOSITE;
            }
            let loop0 = k0 == BORDER as usize || k0 == SEAM as usize;
            let loop1 = k1 == BORDER as usize || k1 == SEAM as usize;
            if loop0 && k1 != MAN as usize {
                rule |= PICK_CHECK_FORWARD;
            }
            if loop1 && k0 != MAN as usize {
                rule |= PICK_CHECK_BACK;
            }
            table[k0 * 8 + k1] = rule;
            k1 += 1;
        }
        k0 += 1;
    }
    table
};
// Kinds are always below six, so masking never changes a valid table index.
#[inline(always)]
fn pair(k0: u8, k1: u8) -> usize {
    ((k0 as usize) << 3 | k1 as usize) & 63
}
// Topology validation bounds every stored index by u32::MAX. Conversions
// occur at slice access boundaries, keeping heap records identical in width to C++.
struct Indices(Vec<u32>);
impl Indices {
    #[inline]
    fn get(&self, i: usize) -> usize {
        self.0[i] as usize
    }
    #[inline]
    fn set(&mut self, i: usize, value: usize) {
        self.0[i] = value as u32;
    }
    fn fill(&mut self, value: usize) {
        self.0.fill(value as u32);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}
pub(crate) struct State {
    vertex_count: usize,
    sqrt_cache: crate::math::SqrtCache,
    offsets: Indices,
    edges: Vec<Edge>,
    remap: Indices,
    wedge: Indices,
    kind: Vec<u8>,
    forward: Indices,
    back: Indices,
    p: Vec<V>,
    a: Vec<f32>,
    q: Vec<Q>,
    aq: Vec<Q>,
    g: Vec<G>,
    vg: Vec<G>,
    sparse: Vec<u32>,
    original: Vec<u32>,
    clamped: bool,
    collapses: Vec<Collapse>,
    order: Indices,
    cr: Indices,
    locked: Vec<bool>,
    table: Indices,
    // Largest per-vertex edge count in the current adjacency.
    max_degree: usize,
    ac: usize,
    bytes: usize,
}
struct Budget<'a> {
    ws: &'a mut Workspace,
    bytes: usize,
    retained: Result<usize, Error>,
    charged: bool,
}
impl<'a> Budget<'a> {
    fn new(ws: &'a mut Workspace, bytes: usize) -> Self {
        let retained = ws.retained_bytes();
        Self {
            ws,
            bytes,
            retained,
            charged: false,
        }
    }
    fn charge_current(&mut self) -> Result<(), Error> {
        if !self.charged {
            self.ws.charge_owned(self.retained?, self.bytes)?;
            self.charged = true;
        }
        Ok(())
    }
    fn reuse_capacity<T>(&mut self, v: &mut Vec<T>, n: usize) -> Result<(), Error> {
        let size = core::mem::size_of::<T>();
        let old = checked_bytes(v.capacity(), size)?;
        if self.bytes < old {
            return Err(Error::SizeOverflow);
        }
        if n <= v.capacity() {
            self.charge_current()?;
            return Ok(());
        }
        let minimum = n;
        let base = self.bytes.checked_sub(old).ok_or(Error::SizeOverflow)?;
        let requested = checked_bytes(minimum, size)?;
        self.ws.charge_owned(
            self.retained?,
            base.checked_add(requested).ok_or(Error::SizeOverflow)?,
        )?;
        if n > v.capacity() {
            v.try_reserve_exact(n.saturating_sub(v.len()))
                .map_err(|_| Error::AllocationFailed)?;
        }
        self.bytes = base
            .checked_add(checked_bytes(v.capacity(), size)?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != minimum {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        self.charged = true;
        Ok(())
    }
    fn reuse<T: Default + Clone>(&mut self, v: &mut Vec<T>, n: usize) -> Result<(), Error> {
        self.reuse_capacity(v, n)?;
        v.resize(n, T::default());
        Ok(())
    }
    fn vec<T: Default + Clone>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        self.filled(n, T::default())
    }
    fn filled<T: Clone>(&mut self, n: usize, value: T) -> Result<Vec<T>, Error> {
        let mut v = self.reserve(n)?;
        v.resize(n, value);
        Ok(v)
    }
    fn release<T>(&mut self, value: Vec<T>) {
        self.bytes -= value.capacity() * core::mem::size_of::<T>();
        drop(value);
    }
    fn reserve<T>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        let b = checked_bytes(n, core::mem::size_of::<T>())?;
        self.ws.charge_owned(
            self.retained?,
            self.bytes.checked_add(b).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != n {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        self.charged = true;
        Ok(v)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimplifyScratch")
            .field("vertices", &self.vertex_count)
            .field("heap_bytes", &self.capacity_bytes())
            .finish()
    }
}
impl State {
    pub(crate) fn capacity_bytes(&self) -> Result<usize, Error> {
        let fields = [
            (self.offsets.0.capacity(), 4),
            (self.edges.capacity(), core::mem::size_of::<Edge>()),
            (self.remap.0.capacity(), 4),
            (self.wedge.0.capacity(), 4),
            (self.kind.capacity(), 1),
            (self.forward.0.capacity(), 4),
            (self.back.0.capacity(), 4),
            (self.p.capacity(), core::mem::size_of::<V>()),
            (self.a.capacity(), 4),
            (self.q.capacity(), core::mem::size_of::<Q>()),
            (self.aq.capacity(), core::mem::size_of::<Q>()),
            (self.g.capacity(), core::mem::size_of::<G>()),
            (self.vg.capacity(), core::mem::size_of::<G>()),
            (self.sparse.capacity(), 4),
            (self.original.capacity(), 4),
            (self.collapses.capacity(), core::mem::size_of::<Collapse>()),
            (self.order.0.capacity(), 4),
            (self.cr.0.capacity(), 4),
            (self.locked.capacity(), 1),
            (self.table.0.capacity(), 4),
        ];
        fields.into_iter().try_fold(0usize, |sum, (n, size)| {
            sum.checked_add(checked_bytes(n, size)?)
                .ok_or(Error::SizeOverflow)
        })
    }
    fn empty() -> Self {
        Self {
            vertex_count: 0,
            sqrt_cache: crate::math::SqrtCache::new(),
            offsets: Indices(Vec::new()),
            edges: Vec::new(),
            remap: Indices(Vec::new()),
            wedge: Indices(Vec::new()),
            kind: Vec::new(),
            forward: Indices(Vec::new()),
            back: Indices(Vec::new()),
            p: Vec::new(),
            a: Vec::new(),
            q: Vec::new(),
            aq: Vec::new(),
            g: Vec::new(),
            vg: Vec::new(),
            sparse: Vec::new(),
            original: Vec::new(),
            clamped: false,
            collapses: Vec::new(),
            order: Indices(Vec::new()),
            cr: Indices(Vec::new()),
            locked: Vec::new(),
            table: Indices(Vec::new()),
            max_degree: 0,
            ac: 0,
            bytes: 0,
        }
    }
    fn new(
        n: usize,
        m: usize,
        ac: usize,
        ws: &mut Workspace,
        output: usize,
    ) -> Result<Self, Error> {
        let na = n.checked_mul(ac).ok_or(Error::SizeOverflow)?;
        let buckets = n
            .checked_add(n / 4)
            .ok_or(Error::SizeOverflow)?
            .checked_next_power_of_two()
            .ok_or(Error::SizeOverflow)?;
        let mut state = ws.simplify.take().unwrap_or_else(Self::empty);
        let bytes = output
            .checked_add(state.capacity_bytes()?)
            .ok_or(Error::SizeOverflow)?;
        let mut b = Budget::new(ws, bytes);
        b.reuse(&mut state.offsets.0, n + 1)?;
        b.reuse(&mut state.edges, m)?;
        state.remap.0.clear();
        b.reuse_capacity(&mut state.remap.0, n)?;
        state.wedge.0.clear();
        b.reuse_capacity(&mut state.wedge.0, n)?;
        b.reuse(&mut state.kind, n)?;
        state.kind.fill(MAN);
        b.reuse(&mut state.forward.0, n)?;
        state.forward.fill(NONE);
        b.reuse(&mut state.back.0, n)?;
        state.back.fill(NONE);
        state.p.clear();
        b.reuse_capacity(&mut state.p, n)?;
        b.reuse(&mut state.a, na)?;
        b.reuse(&mut state.q, n)?;
        state.q.fill(Q::default());
        b.reuse(&mut state.aq, if ac > 0 { n } else { 0 })?;
        state.aq.fill(Q::default());
        b.reuse(&mut state.g, na)?;
        state.g.fill(G::default());
        state.vg.clear();
        state.sparse.clear();
        state.original.clear();
        state.collapses.clear();
        b.reuse(&mut state.cr.0, n)?;
        b.reuse(&mut state.locked, n)?;
        state.locked.fill(false);
        b.reuse(&mut state.table.0, buckets)?;
        state.table.fill(NONE);
        state.ac = ac;
        state.vertex_count = n;
        state.clamped = false;
        state.max_degree = 0;
        state.bytes = b.bytes;
        Ok(state)
    }
    fn adjacency<const WELD: bool>(
        &mut self,
        indices: &[u32],
        work: &mut Work,
    ) -> Result<(), Error> {
        work.add(self.vertex_count)?;
        self.offsets.fill(0);
        let counters = &mut self.offsets.0[1..];
        work.scan(indices.iter().copied(), |i| {
            let v = if WELD {
                self.remap.get(i as usize)
            } else {
                i as usize
            };
            counters[v] += 1;
            Ok(())
        })?;
        let mut offset = 0;
        let mut max_degree = 0;
        for v in counters.iter_mut() {
            let count = *v;
            *v = offset;
            offset += count;
            max_degree = max_degree.max(count);
        }
        self.max_degree = max_degree as usize;
        let triangles = indices.as_chunks::<3>().0;
        let charged = work.precharge(triangles.len(), 3)?;
        for t in triangles {
            if !charged {
                work.add(3)?;
            }
            let mut v = [t[0] as usize, t[1] as usize, t[2] as usize];
            if WELD {
                for x in &mut v {
                    *x = self.remap.get(*x);
                }
            }
            for e in 0..3 {
                let a = v[e];
                let offset = &mut counters[a];
                self.edges[*offset as usize] = Edge {
                    next: v[(e + 1) % 3] as u32,
                    prev: v[(e + 2) % 3] as u32,
                };
                *offset += 1;
            }
        }
        Ok(())
    }
    fn has_edge<const WELD: bool, M: Meter>(
        &self,
        a: usize,
        b: usize,
        work: &mut M,
    ) -> Result<bool, Error> {
        let mut v = a;
        loop {
            for e in &self.edges[self.offsets.get(v)..self.offsets.get(v + 1)] {
                work.add(1)?;
                if if WELD {
                    self.remap.get(e.next as usize) == self.remap.get(b)
                } else {
                    e.next as usize == b
                } {
                    return Ok(true);
                }
            }
            if !WELD {
                break;
            }
            v = self.wedge.get(v);
            if v == a {
                break;
            }
        }
        Ok(false)
    }
    fn position_remap(&mut self, p: Positions<'_>, work: &mut Work) -> Result<(), Error> {
        if let Some((values, mapping)) = p.packed_source() {
            if let Some(mapping) = mapping {
                self.position_remap_kernel(mapping.len(), |i| Ok(values[mapping[i] as usize]), work)
            } else {
                self.position_remap_kernel(values.len(), |i| Ok(values[i]), work)
            }
        } else {
            self.position_remap_kernel(p.len(), |i| p.at(i), work)
        }
    }
    fn position_remap_kernel(
        &mut self,
        count: usize,
        read: impl Fn(usize) -> Result<[f32; 3], Error>,
        work: &mut Work,
    ) -> Result<(), Error> {
        let mask = self.table.len() - 1;
        for i in 0..count {
            work.add(1)?;
            let v = read(i)?;
            let mut bits = v.map(|v| if v == 0.0 { 0 } else { v.to_bits() });
            for x in &mut bits {
                *x ^= *x >> 17;
            }
            let hash = bits[0].wrapping_mul(73856093)
                ^ bits[1].wrapping_mul(19349663)
                ^ bits[2].wrapping_mul(83492791);
            let mut bucket = hash as usize & mask;
            for probe in 0..=mask {
                work.add(1)?;
                let j = self.table.get(bucket);
                if j == NONE {
                    self.table.set(bucket, i);
                    break;
                }
                if read(j)? == v {
                    break;
                }
                bucket = (bucket + probe + 1) & mask;
            }
            self.remap.0.push(self.table.get(bucket) as u32);
            self.wedge.0.push(i as u32);
        }
        for i in 0..count {
            work.add(1)?;
            let r = self.remap.get(i);
            if r != i {
                self.wedge.set(i, self.wedge.get(r));
                self.wedge.set(r, i);
            }
        }
        Ok(())
    }
    // Visits without PERMISSIVE: each vertex and unwelded edge, an unwelded edge
    // search of at most max_degree per edge, and at most four further vertex
    // passes. PERMISSIVE welded searches have no cheap bound.
    fn classify_work(&self, edges: usize, options: SimplifyOptions) -> Option<usize> {
        if options.contains(SimplifyOptions::PERMISSIVE) {
            return None;
        }
        edges
            .checked_mul(self.max_degree.checked_add(1)?)?
            .checked_add(self.vertex_count.checked_mul(5)?)
    }
    fn classify<M: Meter>(
        &mut self,
        flags: Option<&[VertexFlags]>,
        options: SimplifyOptions,
        work: &mut M,
    ) -> Result<(), Error> {
        let n = self.vertex_count;
        for i in 0..n {
            work.add(1)?;
            for j in self.offsets.get(i)..self.offsets.get(i + 1) {
                work.add(1)?;
                let t = self.edges[j].next as usize;
                if t == i {
                    self.forward.set(i, i);
                    self.back.set(i, i);
                } else if !self.has_edge::<false, M>(t, i, work)? {
                    self.back
                        .set(t, if self.back.get(t) == NONE { i } else { t });
                    self.forward
                        .set(i, if self.forward.get(i) == NONE { t } else { i });
                }
            }
        }
        for i in 0..n {
            work.add(1)?;
            self.kind[i] = if self.remap.get(i) != i {
                self.kind[self.remap.get(i)]
            } else if self.wedge.get(i) == i {
                let a = self.back.get(i);
                let b = self.forward.get(i);
                if a == NONE && b == NONE {
                    MAN
                } else if a != NONE && b != NONE && self.remap.get(a) == self.remap.get(b) && a != i
                {
                    SEAM
                } else if a != i && b != i {
                    BORDER
                } else {
                    LOCKED
                }
            } else if self.wedge.get(self.wedge.get(i)) == i {
                let w = self.wedge.get(i);
                let a = self.back.get(i);
                let b = self.forward.get(i);
                let c = self.back.get(w);
                let d = self.forward.get(w);
                if a != NONE
                    && a != i
                    && b != NONE
                    && b != i
                    && c != NONE
                    && c != w
                    && d != NONE
                    && d != w
                    && self.remap.get(a) == self.remap.get(d)
                    && self.remap.get(b) == self.remap.get(c)
                    && self.remap.get(a) != self.remap.get(b)
                {
                    SEAM
                } else {
                    LOCKED
                }
            } else {
                LOCKED
            };
        }
        if options.contains(SimplifyOptions::PERMISSIVE) {
            for i in 0..n {
                work.add(1)?;
                if self.kind[i] != SEAM && self.kind[i] != LOCKED {
                    continue;
                }
                if self.remap.get(i) != i {
                    self.kind[i] = self.kind[self.remap.get(i)];
                    continue;
                }
                let mut protect = false;
                let mut border = false;
                let mut v = i;
                loop {
                    work.add(1)?;
                    protect |= flag(flags, v, VertexFlags::PROTECT);
                    for j in self.offsets.get(v)..self.offsets.get(v + 1) {
                        work.add(1)?;
                        border |=
                            !self.has_edge::<true, M>(self.edges[j].next as usize, v, work)?;
                    }
                    v = self.wedge.get(v);
                    if v == i {
                        break;
                    }
                }
                if !protect {
                    self.kind[i] = if border { FRINGE } else { COMPLEX };
                }
            }
        }
        if let Some(flags) = flags {
            for (i, &f) in flags.iter().enumerate() {
                work.add(1)?;
                if f.contains(VertexFlags::LOCK) {
                    self.kind[self.remap.get(i)] = LOCKED;
                }
            }
            for i in 0..n {
                work.add(1)?;
                if self.kind[self.remap.get(i)] == LOCKED {
                    self.kind[i] = LOCKED;
                }
            }
        }
        if options.contains(SimplifyOptions::LOCK_BORDER) {
            for k in &mut self.kind {
                work.add(1)?;
                if *k == BORDER || *k == FRINGE {
                    *k = LOCKED;
                }
            }
        }
        Ok(())
    }
}
fn flag(flags: Option<&[VertexFlags]>, i: usize, f: VertexFlags) -> bool {
    flags.is_some_and(|flags| flags[i].contains(f))
}
#[inline(always)]
fn attribute_gradient(q: &mut Q, values: [f32; 3], p0: V, basis: [V; 2], w: f32) -> G {
    let [a0, a1, a2] = values;
    let gx = basis[0].x * (a1 - a0) + basis[1].x * (a2 - a0);
    let gy = basis[0].y * (a1 - a0) + basis[1].y * (a2 - a0);
    let gz = basis[0].z * (a1 - a0) + basis[1].z * (a2 - a0);
    let gw = a0 - p0.x * gx - p0.y * gy - p0.z * gz;
    q.a00 += w * (gx * gx);
    q.a11 += w * (gy * gy);
    q.a22 += w * (gz * gz);
    q.a10 += w * (gy * gx);
    q.a20 += w * (gz * gx);
    q.a21 += w * (gz * gy);
    q.b0 += w * (gx * gw);
    q.b1 += w * (gy * gw);
    q.b2 += w * (gz * gw);
    q.c += w * (gw * gw);
    G {
        x: w * gx,
        y: w * gy,
        z: w * gz,
        w: w * gw,
    }
}
fn record_triplet<T>(
    records: &mut [T],
    width: usize,
    indices: [usize; 3],
) -> Option<[&mut [T]; 3]> {
    let [a, b, c] = indices;
    if a == b || a == c || b == c {
        return None;
    }
    records
        .get_disjoint_mut([
            a * width..(a + 1) * width,
            b * width..(b + 1) * width,
            c * width..(c + 1) * width,
        ])
        .ok()
}
impl State {
    fn attribute_quadric(&mut self, t: [usize; 3]) -> Q {
        let [i0, i1, i2] = t;
        let p0 = self.p[i0];
        let v0 = self.p[i1].sub(p0);
        let v1 = self.p[i2].sub(p0);
        let normal = v0.cross(v1);
        let w = self.sqrt_cache.sqrt(normal.dot(normal)) * 0.5;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let denom = d00 * d11 - d01 * d01;
        let dr = if denom == 0.0 { 0.0 } else { 1.0 / denom };
        let gx1 = (d11 * v0.x - d01 * v1.x) * dr;
        let gx2 = (d00 * v1.x - d01 * v0.x) * dr;
        let gy1 = (d11 * v0.y - d01 * v1.y) * dr;
        let gy2 = (d00 * v1.y - d01 * v0.y) * dr;
        let gz1 = (d11 * v0.z - d01 * v1.z) * dr;
        let gz2 = (d00 * v1.z - d01 * v0.z) * dr;
        let mut q = Q { w, ..Q::default() };
        let attributes0 = &self.a[i0 * self.ac..][..self.ac];
        let attributes1 = &self.a[i1 * self.ac..][..self.ac];
        let attributes2 = &self.a[i2 * self.ac..][..self.ac];
        let basis = [
            V {
                x: gx1,
                y: gy1,
                z: gz1,
            },
            V {
                x: gx2,
                y: gy2,
                z: gz2,
            },
        ];
        if let Some([g0, g1, g2]) = record_triplet(&mut self.g, self.ac, t) {
            for (((&a0, &a1), &a2), ((g0, g1), g2)) in attributes0
                .iter()
                .zip(attributes1)
                .zip(attributes2)
                .zip(g0.iter_mut().zip(g1).zip(g2))
            {
                let g = attribute_gradient(&mut q, [a0, a1, a2], p0, basis, w);
                g0.add(g);
                g1.add(g);
                g2.add(g);
            }
        } else {
            for (k, ((&a0, &a1), &a2)) in attributes0
                .iter()
                .zip(attributes1)
                .zip(attributes2)
                .enumerate()
            {
                let g = attribute_gradient(&mut q, [a0, a1, a2], p0, basis, w);
                for i in t {
                    self.g[i * self.ac + k].add(g);
                }
            }
        }
        q
    }
    fn quadrics<M: Meter>(
        &mut self,
        indices: &[u32],
        flags: Option<&[VertexFlags]>,
        options: SimplifyOptions,
        work: &mut M,
    ) -> Result<(), Error> {
        let n = self.p.len();
        let p = &self.p[..n];
        let remap = &self.remap.0[..n];
        let qv = &mut self.q[..n];
        let triangles = indices.as_chunks::<3>().0;
        let charged = work.precharge(triangles.len(), 3)?;
        for t in triangles {
            if !charged {
                work.add(3)?;
            }
            let [a, b, c] = [t[0] as usize, t[1] as usize, t[2] as usize];
            let mut normal = p[b].sub(p[a]).cross(p[c].sub(p[a]));
            let area = normal.normalize(&mut self.sqrt_cache);
            let q = Q::plane(normal, -normal.dot(p[a]), self.sqrt_cache.sqrt(area) * 1.0);
            for i in [a, b, c] {
                qv[remap[i] as usize].add(q);
            }
            if !self.vg.is_empty() {
                // The volume gradient uses the same normalized triangle plane.
                let area = area * 0.5;
                let g = G {
                    x: normal.x * area,
                    y: normal.y * area,
                    z: normal.z * area,
                    w: (-p[a].x * normal.x - p[a].y * normal.y - p[a].z * normal.z) * area,
                };
                for i in [a, b, c] {
                    self.vg[remap[i] as usize].add(g);
                }
            }
        }
        let factor = if options.contains(SimplifyOptions::REGULARIZE_LIGHT) {
            1e-2
        } else if options.contains(SimplifyOptions::REGULARIZE) {
            1e-1
        } else {
            1e-7
        };
        work.scan(
            qv.iter_mut().zip(p).zip(remap).enumerate(),
            |(i, ((q, &position), &r))| {
                if r as usize == i {
                    let w = q.w
                        * if flag(flags, i, VertexFlags::PRIORITY) {
                            1.0
                        } else {
                            factor
                        };
                    q.add(Q::point(position, w));
                }
                Ok(())
            },
        )?;
        let charged_edges = work.precharge(indices.len(), 1)?;
        for t in indices.as_chunks::<3>().0 {
            for e in 0..3 {
                if !charged_edges {
                    work.add(1)?;
                }
                let a = t[e] as usize;
                let b = t[(e + 1) % 3] as usize;
                let c = t[(e + 2) % 3] as usize;
                let k0 = self.kind[a];
                let k1 = self.kind[b];
                if k0 == MAN || k1 == MAN {
                    continue;
                }
                if ![BORDER, SEAM, FRINGE].contains(&k0) && ![BORDER, SEAM, FRINGE].contains(&k1) {
                    continue;
                }
                if (k0 == BORDER || k0 == SEAM) && self.forward.get(a) != b {
                    continue;
                }
                if (k1 == BORDER || k1 == SEAM) && self.back.get(b) != a {
                    continue;
                }
                if (k0 == FRINGE || k1 == FRINGE) && (k0 == COMPLEX || k1 == COMPLEX) {
                    continue;
                }
                let w = if k0 == SEAM || k1 == SEAM { 0.5 } else { 10.0 };
                let mut q = Q::edge(p[a], p[b], p[c], w, &mut self.sqrt_cache);
                let mut qt = Q::triangle(p[a], p[b], p[c], w, &mut self.sqrt_cache);
                qt.w = 0.0;
                q.add(qt);
                qv[remap[a] as usize].add(q);
                qv[remap[b] as usize].add(q);
            }
        }
        if self.ac > 0 {
            let units = 3 + self.ac * 3;
            let charged = work.precharge(triangles.len(), units)?;
            for t in triangles {
                if !charged {
                    work.add(units)?;
                }
                let t = [t[0] as usize, t[1] as usize, t[2] as usize];
                let q = self.attribute_quadric(t);
                for i in t {
                    self.aq[i].add(q);
                }
            }
        }
        if options.bits() & 128 != 0 {
            extra::fold_quadrics(self, work)?;
        }
        if self.q.iter().chain(&self.aq).any(|q| !q.finite())
            || self
                .g
                .iter()
                .any(|g| ![g.x, g.y, g.z, g.w].iter().all(|x| x.is_finite()))
        {
            return Err(Error::NumericalFailure);
        }
        // Ranking reads each weight many times. Cache the identical division
        // once, then refresh it whenever update merges positional quadrics.
        // An overflowing reciprocal is checked when its error is ranked, as
        // before; it is not an additional quadric validation condition.
        for q in &mut self.q {
            q.cache_inverse_weight();
        }
        Ok(())
    }
    // Visits at most one per index.
    fn pick<M: Meter>(
        &mut self,
        indices: &[u32],
        capacity: usize,
        work: &mut M,
    ) -> Result<usize, Error> {
        let mut count = 0;
        // Disjoint borrows keep table bases in registers across pushes.
        let remap = &self.remap.0[..];
        let kind = &self.kind[..];
        let forward = &self.forward.0[..];
        let back = &self.back.0[..];
        let collapses = &mut self.collapses;
        collapses.clear();
        for t in indices.as_chunks::<3>().0 {
            if count + 3 > capacity {
                break;
            }
            for e in 0..3 {
                work.add(1)?;
                let a = t[e] as usize;
                let b = t[(e + 1) % 3] as usize;
                let ra = remap[a];
                let rb = remap[b];
                if ra == rb {
                    continue;
                }
                let rule = PICK[pair(kind[a], kind[b])];
                if rule & PICK_ANY == 0 {
                    continue;
                }
                if rule & PICK_OPPOSITE != 0 && rb > ra {
                    continue;
                }
                if rule & PICK_CHECK_FORWARD != 0 && forward[a] as usize != b {
                    continue;
                }
                if rule & PICK_CHECK_BACK != 0 && back[b] as usize != a {
                    continue;
                }
                collapses.push(if rule & PICK_BOTH != 0 {
                    Collapse {
                        a: a as u32,
                        b: b as u32,
                        error: f32::from_bits(1),
                    }
                } else if rule & PICK_FORWARD != 0 {
                    Collapse {
                        a: a as u32,
                        b: b as u32,
                        ..Collapse::default()
                    }
                } else {
                    Collapse {
                        a: b as u32,
                        b: a as u32,
                        ..Collapse::default()
                    }
                });
                count += 1;
            }
        }
        Ok(count)
    }
    fn complex_target(&self, v: usize, t: usize) -> usize {
        let r = self.remap.get(t);
        if self.forward.get(v) != NONE && self.remap.get(self.forward.get(v)) == r {
            self.forward.get(v)
        } else if self.back.get(v) != NONE && self.remap.get(self.back.get(v)) == r {
            self.back.get(v)
        } else {
            t
        }
    }
    fn seam_target(&self, a: usize, b: usize) -> (usize, usize) {
        let s0 = self.wedge.get(a);
        let s1 = if self.forward.get(a) == b {
            self.back.get(s0)
        } else {
            self.forward.get(s0)
        };
        (s0, if s1 != NONE { s1 } else { self.wedge.get(b) })
    }
    fn rank(&mut self, count: usize, work: &mut Work) -> Result<(), Error> {
        if self.ac == 0 {
            // All three vertex tables have the same length. Re-borrow that
            // common prefix once, and iterate the initialized candidate slice.
            let n = self.p.len();
            let p = &self.p[..n];
            let q = &self.q[..n];
            let remap = &self.remap.0[..n];
            #[inline(always)]
            fn rank_one(c: &mut Collapse, p: &[V], q: &[Q], remap: &[u32]) -> Result<(), Error> {
                let a = c.a as usize;
                let b = c.b as usize;
                let bidirectional = c.error.to_bits() != 0;
                let ei = q[remap[a] as usize].error(p[b]);
                let ej = if bidirectional {
                    q[remap[b] as usize].error(p[a])
                } else {
                    f32::MAX
                };
                if !ei.is_finite() || !ej.is_finite() {
                    return Err(Error::NumericalFailure);
                }
                if bidirectional && ej < ei {
                    core::mem::swap(&mut c.a, &mut c.b);
                }
                c.error = if ej < ei { ej } else { ei };
                Ok(())
            }
            let values = &mut self.collapses[..count];
            return if work.covers(count)? {
                for (i, c) in values.iter_mut().enumerate() {
                    if let Err(error) = rank_one(c, p, q, remap) {
                        work.add(i + 1)?;
                        return Err(error);
                    }
                }
                work.add(count)
            } else {
                for c in values {
                    work.add(1)?;
                    rank_one(c, p, q, remap)?;
                }
                Ok(())
            };
        }
        for i in 0..count {
            work.add(1)?;
            let c = self.collapses[i];
            let a = c.a as usize;
            let b = c.b as usize;
            let mut ei = self.q[self.remap.get(a)].error(self.p[b]);
            let mut ej = if c.error.to_bits() != 0 {
                self.q[self.remap.get(b)].error(self.p[a])
            } else {
                f32::MAX
            };
            if self.ac > 0 {
                work.add(self.ac * 2)?;
                ei += self.attribute_error(a, b);
                ej += if c.error.to_bits() != 0 {
                    self.attribute_error(b, a)
                } else {
                    0.0
                };
                if self.kind[a] == SEAM {
                    work.add(self.ac * 2)?;
                    let (s0, s1) = self.seam_target(a, b);
                    ei += self.attribute_error(s0, s1);
                    ej += if c.error.to_bits() != 0 {
                        self.attribute_error(s1, s0)
                    } else {
                        0.0
                    };
                } else {
                    if self.kind[a] == COMPLEX || self.kind[a] == FRINGE {
                        let mut v = self.wedge.get(a);
                        while v != a {
                            work.add(self.ac)?;
                            ei += self.attribute_error(v, self.complex_target(v, b));
                            v = self.wedge.get(v);
                        }
                    }
                    if (self.kind[b] == COMPLEX || self.kind[b] == FRINGE)
                        && (c.error.to_bits() != 0)
                    {
                        let mut v = self.wedge.get(b);
                        while v != b {
                            work.add(self.ac)?;
                            ej += self.attribute_error(v, self.complex_target(v, a));
                            v = self.wedge.get(v);
                        }
                    }
                }
            }
            if !ei.is_finite() || !ej.is_finite() {
                return Err(Error::NumericalFailure);
            }
            let rev = (c.error.to_bits() != 0) && ej < ei;
            self.collapses[i] = Collapse {
                a: (if rev { b } else { a }) as u32,
                b: (if rev { a } else { b }) as u32,
                error: if ej < ei { ej } else { ei },
            };
        }
        Ok(())
    }
    #[inline(always)]
    fn attribute_error(&self, source: usize, target: usize) -> f32 {
        let v = self.p[target];
        let q = self.aq[source];
        let mut r = q.eval(v);
        let attributes = &self.a[target * self.ac..][..self.ac];
        let gradients = &self.g[source * self.ac..][..self.ac];
        let term = |a: f32, g: G| {
            let g = v.x * g.x + v.y * g.y + v.z * g.z + g.w;
            a * (a * q.w - 2.0 * g)
        };
        let (a4, at) = attributes.as_chunks::<4>();
        let (g4, gt) = gradients.as_chunks::<4>();
        for (a, g) in a4.iter().zip(g4) {
            // Independent terms can vectorize; accumulation remains in the
            // exact upstream component order, with no horizontal reduction.
            let terms = [
                term(a[0], g[0]),
                term(a[1], g[1]),
                term(a[2], g[2]),
                term(a[3], g[3]),
            ];
            for t in terms {
                r += t;
            }
        }
        for (&a, &g) in at.iter().zip(gt) {
            r += term(a, g);
        }
        let e = r.abs();
        if self.clamped && e >= q.w {
            q.w
        } else {
            e
        }
    }
    fn sort(&mut self, count: usize, work: &mut Work) -> Result<(), Error> {
        // pick emits at most one candidate per input index; topology already
        // bounds that count by u32::MAX. Match upstream's bucket storage width.
        let mut h = [0u32; 2560];
        let collapses = &self.collapses[..count];
        work.scan(collapses.iter(), |c| {
            h[sort_key(c.error)] += 1;
            Ok(())
        })?;
        let mut sum = 0;
        for v in &mut h {
            let n = *v;
            *v = sum as u32;
            sum += n as usize;
        }
        let order = &mut self.order.0[..count];
        work.scan(collapses.iter().enumerate(), |(i, c)| {
            let k = sort_key(c.error);
            order[h[k] as usize] = i as u32;
            h[k] += 1;
            Ok(())
        })?;
        Ok(())
    }
    fn flips<M: Meter>(&mut self, a: usize, b: usize, work: &mut M) -> Result<bool, Error> {
        for e in &self.edges[self.offsets.get(a)..self.offsets.get(a + 1)] {
            work.add(1)?;
            let x = self.cr.get(e.next as usize);
            let y = self.cr.get(e.prev as usize);
            if x == b || y == b || x == y {
                continue;
            }
            let pa = self.p[x];
            let eb = self.p[y].sub(pa);
            let ec = self.p[a].sub(pa);
            let ed = self.p[b].sub(pa);
            let nbc = eb.cross(ec);
            let nbd = eb.cross(ed);
            if nbc.dot(nbd) <= 0.25 * self.sqrt_cache.sqrt(nbc.dot(nbc) * nbd.dot(nbd)) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    // Visits: the vertex reset, then per candidate one visit and a flip check
    // of at most max_degree edges. PERMISSIVE wedge walks are not bounded here.
    fn perform_work(&self, count: usize, options: SimplifyOptions) -> Option<usize> {
        if options.contains(SimplifyOptions::PERMISSIVE) {
            return None;
        }
        count
            .checked_mul(self.max_degree.checked_add(1)?)?
            .checked_add(self.p.len())
    }
    fn perform<M: Meter>(
        &mut self,
        count: usize,
        goal: usize,
        limit: f32,
        error: &mut f32,
        work: &mut M,
    ) -> Result<usize, Error> {
        let n = self.p.len();
        work.scan(
            self.cr.0[..n]
                .iter_mut()
                .zip(&mut self.locked[..n])
                .enumerate(),
            |(i, (remap, locked))| {
                *remap = i as u32;
                *locked = false;
                Ok(())
            },
        )?;
        let mut edges = 0;
        let mut triangles = 0;
        let mut edge_goal = goal / 2;
        for i in 0..count {
            work.add(1)?;
            let c = self.collapses[self.order.get(i)];
            if c.error > limit || triangles >= goal {
                break;
            }
            let error_goal = if edge_goal < count {
                1.5 * self.collapses[self.order.get(edge_goal)].error
            } else {
                f32::MAX
            };
            if c.error > error_goal && c.error > *error && triangles > goal / 6 {
                break;
            }
            let a = c.a as usize;
            let b = c.b as usize;
            let r0 = self.remap.get(a);
            let r1 = self.remap.get(b);
            let kind = self.kind[a];
            if self.locked[r0] || self.locked[r1] {
                continue;
            }
            if self.flips(r0, r1, work)? {
                edge_goal += 1;
                continue;
            }
            self.cr.set(a, b);
            if kind == COMPLEX || kind == FRINGE {
                let mut v = self.wedge.get(a);
                while v != a {
                    work.add(1)?;
                    self.cr.set(v, self.complex_target(v, b));
                    v = self.wedge.get(v);
                }
            } else if kind == SEAM {
                let (s0, s1) = self.seam_target(a, b);
                self.cr.set(s0, s1);
            }
            self.locked[r0] = true;
            self.locked[r1] = true;
            triangles += if kind == BORDER { 1 } else { 2 };
            edges += 1;
            if *error < c.error {
                *error = c.error;
            }
        }
        Ok(edges)
    }
    fn update(&mut self, vertex_error: &mut f32, work: &mut Work) -> Result<(), Error> {
        for i in 0..self.p.len() {
            work.add(1)?;
            let t = self.cr.get(i);
            if t == i {
                continue;
            }
            let r0 = self.remap.get(i);
            let r1 = self.remap.get(t);
            if i == r0 {
                let q = self.q[r0];
                self.q[r1].add(q);
                if !self.q[r1].finite() {
                    return Err(Error::NumericalFailure);
                }
                self.q[r1].cache_inverse_weight();
                if !self.vg.is_empty() {
                    let g = self.vg[r0];
                    self.vg[r1].add(g);
                }
            }
            if self.ac > 0 {
                if i == r0 {
                    let e = self.q[r0].error(self.p[r1]);
                    if *vertex_error < e {
                        *vertex_error = e;
                    }
                }
                work.add(self.ac)?;
                let q = self.aq[i];
                self.aq[t].add(q);
                if !self.aq[t].finite() {
                    return Err(Error::NumericalFailure);
                }
                for k in 0..self.ac {
                    let g = self.g[i * self.ac + k];
                    self.g[t * self.ac + k].add(g);
                }
            }
        }
        for loops in [&mut self.forward, &mut self.back] {
            for i in 0..loops.len() {
                work.add(1)?;
                let l = loops.get(i);
                if l != NONE {
                    let r = self.cr.get(l);
                    loops.set(
                        i,
                        if i == r {
                            if loops.get(l) != NONE {
                                self.cr.get(loops.get(l))
                            } else {
                                NONE
                            }
                        } else {
                            r
                        },
                    );
                }
            }
        }
        Ok(())
    }
}
fn sort_key(error: f32) -> usize {
    ((error.to_bits() << 1) >> 20).min(2559) as usize
}
#[allow(clippy::too_many_arguments)]
fn run(
    out: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    ws: &mut Workspace,
    work: &mut Work,
    output: usize,
) -> Result<SimplifyResult, Error> {
    let (result, state) = run_state(
        out, indices, positions, attributes, weights, flags, settings, ws, work, output, false,
    )?;
    ws.has_retained = true;
    ws.simplify = Some(state);
    Ok(result)
}
#[inline(always)]
fn normalize_attribute_row(
    destination: &mut [f32],
    source: Attributes<'_>,
    vertex: usize,
    weights: &[f32],
    work: &mut Work,
) -> Result<(), Error> {
    let mut write = 0;
    if let Some(values) = source.float_record(vertex) {
        work.scan(values.iter().zip(weights), |(&value, &weight)| {
            if weight > 0.0 {
                let value = value * weight;
                if !value.is_finite() {
                    return Err(Error::NumericalFailure);
                }
                destination[write] = value;
                write += 1;
            }
            Ok(())
        })?;
    } else {
        for (component, &weight) in weights.iter().enumerate() {
            work.add(1)?;
            if weight > 0.0 {
                let value = source.get(vertex, component).ok_or(Error::InvalidLayout)? * weight;
                if !value.is_finite() {
                    return Err(Error::NumericalFailure);
                }
                destination[write] = value;
                write += 1;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_state(
    out: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    attributes: Option<Attributes<'_>>,
    weights: &[f32],
    flags: Option<&[VertexFlags]>,
    settings: SimplifySettings,
    ws: &mut Workspace,
    work: &mut Work,
    output: usize,
    solve: bool,
) -> Result<(SimplifyResult, State), Error> {
    if settings.options.contains(SimplifyOptions::SPARSE) {
        return extra::sparse_run(
            out, indices, positions, attributes, weights, flags, settings, ws, work, output, solve,
        );
    }
    let n = positions.len();
    let m = indices.len();
    let ac = weights.iter().filter(|&&w| w > 0.0).count();
    let mut s = State::new(n, m, ac, ws, output)?;
    s.clamped = settings.options.bits() & 256 != 0;
    if solve && ac > 0 {
        let mut b = Budget::new(ws, s.bytes);
        b.reuse(&mut s.vg, n)?;
        s.vg.fill(G::default());
        s.bytes = b.bytes;
    }
    s.adjacency::<false>(indices, work)?;
    s.position_remap(positions, work)?;
    metered!(work, s.classify_work(m, settings.options), |w| s.classify(
        flags,
        settings.options,
        w
    ))?;
    work.add(n)?;
    let (lo, extent) = bounds(positions)?;
    let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
    for i in 0..n {
        work.add(1)?;
        let p = positions.at(i)?;
        s.p.push(V {
            x: (p[0] - lo[0]) * scale,
            y: (p[1] - lo[1]) * scale,
            z: (p[2] - lo[2]) * scale,
        });
        if ![s.p[i].x, s.p[i].y, s.p[i].z].iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
        if let Some(a) = attributes {
            normalize_attribute_row(&mut s.a[i * ac..(i + 1) * ac], a, i, weights, work)?;
        }
    }
    let (components, component_errors) = if settings.options.contains(SimplifyOptions::PRUNE) {
        let mut b = crate::budget::Budget::with_bytes(ws, s.bytes);
        let value = extra::components(&s.p, &s.remap.0, indices, &mut b, work)?;
        s.bytes = b.bytes();
        value
    } else {
        (Vec::new(), Vec::new())
    };
    s.quadrics(indices, flags, settings.options, work)?;
    let mut dual = 0;
    work.scan(s.kind.iter().zip(s.offsets.0.windows(2)), |(&k, o)| {
        if k == MAN || k == SEAM {
            dual += o[1] as usize - o[0] as usize;
        }
        Ok(())
    })?;
    let capacity = m - dual / 2 + 3;
    let mut budget = Budget::new(ws, s.bytes);
    budget.reuse_capacity(&mut s.collapses, capacity)?;
    budget.reuse(&mut s.order.0, capacity)?;
    let mut count = m;
    let mut error = 0.0;
    let mut vertex_error = 0.0;
    let mut component_next = 0.0;
    let error_scale = if settings.options.contains(SimplifyOptions::ERROR_ABSOLUTE) {
        extent
    } else {
        1.0
    };
    let limit = (settings.target_error * settings.target_error) / (error_scale * error_scale);
    work.add(m)?;
    out[..m].copy_from_slice(indices);
    while count > settings.target_index_count {
        s.adjacency::<true>(&out[..count], work)?;
        let nc = s.pick(&out[..count], capacity, work)?;
        if nc == 0 {
            break;
        }
        s.rank(nc, work)?;
        s.sort(nc, work)?;
        let goal = (count - settings.target_index_count) / 3;
        let edges = metered!(work, s.perform_work(nc, settings.options), |w| s
            .perform(nc, goal, limit, &mut error, w))?;
        if edges == 0 {
            break;
        }
        if ac == 0 {
            vertex_error = error;
        }
        s.update(&mut vertex_error, work)?;
        let mut write = 0;
        metered!(work, Some(count), |w| (|| {
            for i in (0..count).step_by(3) {
                w.add(3)?;
                let a = s.cr.get(out[i] as usize);
                let b = s.cr.get(out[i + 1] as usize);
                let c = s.cr.get(out[i + 2] as usize);
                let r0 = s.remap.get(a);
                let r1 = s.remap.get(b);
                let r2 = s.remap.get(c);
                if r0 != r1 && r0 != r2 && r1 != r2 {
                    out[write] = a as u32;
                    out[write + 1] = b as u32;
                    out[write + 2] = c as u32;
                    write += 3;
                }
            }
            Ok(())
        })())?;
        count = write;
        if settings.options.contains(SimplifyOptions::PRUNE)
            && count > settings.target_index_count
            && component_next <= vertex_error
        {
            (count, component_next) = extra::prune(
                out,
                count,
                &components,
                &component_errors,
                vertex_error,
                work,
            )?;
        }
    }
    let mut stale = true;
    while settings.options.contains(SimplifyOptions::PRUNE)
        && count > settings.target_index_count
        && component_next <= limit
    {
        let cutoff = if component_next * 1.5 < limit {
            component_next * 1.5
        } else {
            limit
        };
        let mut max_error = 0.0;
        for &e in &component_errors {
            work.add(1)?;
            if e > max_error && e <= cutoff {
                max_error = e;
            }
        }
        let (new_count, next) =
            extra::prune(out, count, &components, &component_errors, cutoff, work)?;
        component_next = next;
        if new_count == count && !stale {
            break;
        }
        stale = false;
        count = new_count;
        if error < max_error {
            error = max_error;
        }
    }
    if solve {
        extra::solve_state(&mut s, &out[..count], work)?;
    }
    let error = sqrt(error) * error_scale;
    if !error.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok((
        SimplifyResult {
            index_count: count,
            error,
        },
        s,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn prepaid_phases_match_checked_visits() {
        // Five visits of one record each, then a two-record scan; a numerical
        // failure may occur at any visit. Bounds: exact, loose, or absent.
        fn phase<M: Meter>(
            fail_at: usize,
            seen: &mut Vec<usize>,
            work: &mut M,
        ) -> Result<(), Error> {
            for i in 0..5 {
                work.add(1)?;
                seen.push(i);
                if i == fail_at {
                    return Err(Error::NumericalFailure);
                }
            }
            work.scan(5..7, |i| {
                seen.push(i);
                if i == fail_at {
                    return Err(Error::NumericalFailure);
                }
                Ok(())
            })
        }
        fn run_phase(
            limit: u64,
            fail_at: usize,
            bound: Option<usize>,
        ) -> (Result<(), Error>, u64, Vec<usize>) {
            let mut ws = Workspace::new(Limits {
                max_bytes: 0,
                max_work: limit,
            });
            let mut work = ws.begin();
            let mut seen = Vec::new();
            let result = (|| metered!(&mut work, bound, |w| phase(fail_at, &mut seen, w)))();
            ws.finish(&work);
            (result, ws.usage().work, seen)
        }
        for limit in 0..=9 {
            for fail_at in 0..=7 {
                let checked = run_phase(limit, fail_at, None);
                assert_eq!(run_phase(limit, fail_at, Some(7)), checked);
                assert_eq!(run_phase(limit, fail_at, Some(9)), checked);
            }
        }
    }

    #[test]
    fn ranking_preserves_numerical_failure_and_budget_prefix() {
        for limit in 0..=6 {
            for fail_at in 0..=5 {
                let mut ws = Workspace::new(Limits {
                    max_bytes: 1 << 20,
                    max_work: limit,
                });
                let mut state = State::new(2, 0, 0, &mut ws, 0).unwrap();
                state.p.extend([V::default(); 2]);
                state.remap.0.extend([0, 1]);
                state.q[0] = Q {
                    c: f32::MAX,
                    w: 0.5,
                    inverse_weight: 2.0,
                    ..Q::default()
                };
                state.q[1] = Q {
                    w: 1.0,
                    inverse_weight: 1.0,
                    ..Q::default()
                };
                for i in 0..5 {
                    state.collapses.push(Collapse {
                        a: if i == fail_at { 0 } else { 1 },
                        b: 1,
                        error: f32::from_bits(1),
                    });
                }
                let mut work = ws.begin();
                let result = state.rank(5, &mut work);
                let (expected, visited, changed) = if limit <= fail_at as u64 && limit < 5 {
                    (Err(Error::LimitExceeded), limit, limit as usize)
                } else if fail_at < 5 {
                    (Err(Error::NumericalFailure), fail_at as u64 + 1, fail_at)
                } else {
                    (Ok(()), 5, 5)
                };
                assert_eq!(result, expected);
                ws.finish(&work);
                assert_eq!(ws.usage().work, visited);
                for (i, c) in state.collapses.iter().enumerate() {
                    assert_eq!(c.error.to_bits(), if i < changed { 0 } else { 1 });
                }
            }
        }
    }
}
