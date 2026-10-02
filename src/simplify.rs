//! Scalar edge-collapse simplification translated from upstream simplifier.cpp.
use crate::math::sqrt;
use crate::workspace::{checked_bytes, topology, Work};
use crate::{validate_vertex_flags, Attributes, Error, Positions, VertexFlags, Workspace};
use alloc::vec::Vec;

/// Stable simplification options supported by this port.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimplifyOptions(u32);
impl SimplifyOptions {
    /// Default topology and relative-error behavior.
    pub const EMPTY: Self = Self(0);
    /// Prevent movement of geometric border vertices.
    pub const LOCK_BORDER: Self = Self(1);
    /// Interpret the error limit and result in position units.
    pub const ERROR_ABSOLUTE: Self = Self(4);
    /// Apply stronger positional regularization.
    pub const REGULARIZE: Self = Self(16);
    /// Allow collapses across unprotected attribute discontinuities.
    pub const PERMISSIVE: Self = Self(32);
    /// Apply light positional regularization.
    pub const REGULARIZE_LIGHT: Self = Self(64);
    /// Validate a raw option mask; unsupported options are rejected.
    pub const fn from_bits(bits: u32) -> Result<Self, Error> {
        if bits & !(1 | 4 | 16 | 32 | 64) != 0 {
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
    for i in 0..p.len() {
        let v = p.at(i)?;
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
    }
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
    for &w in weights {
        work.add(1)?;
        if !w.is_finite() || w < 0.0 {
            return Err(Error::InvalidParameter);
        }
    }
    for i in 0..p.len() {
        work.add(1)?;
        if p.at(i)?.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidParameter);
        }
        if let Some(a) = a {
            for k in 0..a.components() {
                work.add(1)?;
                if !a.get(i, k).ok_or(Error::InvalidLayout)?.is_finite() {
                    return Err(Error::InvalidParameter);
                }
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
impl V {
    fn sub(self, b: Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
    fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    fn cross(self, b: Self) -> Self {
        Self {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
    fn normalize(&mut self) -> f32 {
        let l = sqrt(self.dot(*self));
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
}
impl Q {
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
    fn eval(self, v: V) -> f32 {
        let mut rx = (self.b0 + self.a10 * v.y) * 2.0;
        let mut ry = (self.b1 + self.a21 * v.z) * 2.0;
        let mut rz = (self.b2 + self.a20 * v.x) * 2.0;
        rx += self.a00 * v.x;
        ry += self.a11 * v.y;
        rz += self.a22 * v.z;
        self.c + rx * v.x + ry * v.y + rz * v.z
    }
    fn error(self, v: V) -> f32 {
        self.eval(v).abs() * if self.w == 0.0 { 0.0 } else { 1.0 / self.w }
    }
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
        }
    }
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
    fn triangle(a: V, b: V, c: V, w: f32) -> Self {
        let mut n = b.sub(a).cross(c.sub(a));
        let area = n.normalize();
        Self::plane(n, -n.dot(a), sqrt(area) * w)
    }
    fn edge(a: V, b: V, c: V, w: f32) -> Self {
        let p10 = b.sub(a);
        let ls = p10.dot(p10);
        let p20 = c.sub(a);
        let proj = p20.dot(p10);
        let mut perp = V {
            x: p20.x * ls - p10.x * proj,
            y: p20.y * ls - p10.y * proj,
            z: p20.z * ls - p10.z * proj,
        };
        perp.normalize();
        Self::plane(perp, -perp.dot(a), sqrt(ls) * w)
    }
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
    next: usize,
    prev: usize,
}
#[derive(Clone, Copy, Default)]
struct Collapse {
    a: usize,
    b: usize,
    bidi: bool,
    error: f32,
}
const NONE: usize = usize::MAX;
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
struct State {
    offsets: Vec<usize>,
    edges: Vec<Edge>,
    remap: Vec<usize>,
    wedge: Vec<usize>,
    kind: Vec<u8>,
    forward: Vec<usize>,
    back: Vec<usize>,
    p: Vec<V>,
    a: Vec<f32>,
    q: Vec<Q>,
    aq: Vec<Q>,
    g: Vec<G>,
    collapses: Vec<Collapse>,
    order: Vec<usize>,
    cr: Vec<usize>,
    locked: Vec<bool>,
    table: Vec<usize>,
    ac: usize,
}
struct Budget<'a> {
    ws: &'a mut Workspace,
    bytes: usize,
}
impl Budget<'_> {
    fn vec<T: Default + Clone>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        let b = checked_bytes(n, core::mem::size_of::<T>())?;
        self.ws.prepare(
            [0; 4],
            self.bytes.checked_add(b).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        self.ws.prepare([0; 4], self.bytes)?;
        v.resize(n, T::default());
        Ok(v)
    }
}
impl State {
    fn new(
        n: usize,
        m: usize,
        ac: usize,
        ws: &mut Workspace,
        output: usize,
    ) -> Result<Self, Error> {
        let mut b = Budget { ws, bytes: output };
        let na = n.checked_mul(ac).ok_or(Error::SizeOverflow)?;
        let buckets = n
            .checked_add(n / 4)
            .ok_or(Error::SizeOverflow)?
            .checked_next_power_of_two()
            .ok_or(Error::SizeOverflow)?;
        Ok(Self {
            offsets: b.vec(n + 1)?,
            edges: b.vec(m)?,
            remap: b.vec(n)?,
            wedge: b.vec(n)?,
            kind: b.vec(n)?,
            forward: b.vec(n)?,
            back: b.vec(n)?,
            p: b.vec(n)?,
            a: b.vec(na)?,
            q: b.vec(n)?,
            aq: b.vec(if ac > 0 { n } else { 0 })?,
            g: b.vec(na)?,
            collapses: b.vec(m.checked_add(3).ok_or(Error::SizeOverflow)?)?,
            order: b.vec(m + 3)?,
            cr: b.vec(n)?,
            locked: b.vec(n)?,
            table: b.vec(buckets)?,
            ac,
        })
    }
    fn adjacency(&mut self, indices: &[u32], weld: bool, work: &mut Work) -> Result<(), Error> {
        work.add(self.p.len())?;
        self.offsets.fill(0);
        for &i in indices {
            work.add(1)?;
            let v = if weld {
                self.remap[i as usize]
            } else {
                i as usize
            };
            self.offsets[v + 1] += 1;
        }
        let mut offset = 0;
        for v in &mut self.offsets[1..] {
            let count = *v;
            *v = offset;
            offset += count;
        }
        for t in indices.as_chunks::<3>().0 {
            work.add(3)?;
            let mut v = [t[0] as usize, t[1] as usize, t[2] as usize];
            if weld {
                for x in &mut v {
                    *x = self.remap[*x];
                }
            }
            for e in 0..3 {
                let a = v[e];
                self.edges[self.offsets[a + 1]] = Edge {
                    next: v[(e + 1) % 3],
                    prev: v[(e + 2) % 3],
                };
                self.offsets[a + 1] += 1;
            }
        }
        Ok(())
    }
    fn has_edge(&self, a: usize, b: usize, weld: bool, work: &mut Work) -> Result<bool, Error> {
        let mut v = a;
        loop {
            for e in &self.edges[self.offsets[v]..self.offsets[v + 1]] {
                work.add(1)?;
                if if weld {
                    self.remap[e.next] == self.remap[b]
                } else {
                    e.next == b
                } {
                    return Ok(true);
                }
            }
            if !weld {
                break;
            }
            v = self.wedge[v];
            if v == a {
                break;
            }
        }
        Ok(false)
    }
    fn position_remap(&mut self, p: Positions<'_>, work: &mut Work) -> Result<(), Error> {
        self.table.fill(NONE);
        let mask = self.table.len() - 1;
        for i in 0..p.len() {
            work.add(1)?;
            let v = p.at(i)?;
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
                let j = self.table[bucket];
                if j == NONE {
                    self.table[bucket] = i;
                    break;
                }
                if p.at(j)? == v {
                    break;
                }
                bucket = (bucket + probe + 1) & mask;
            }
            self.remap[i] = self.table[bucket];
            self.wedge[i] = i;
        }
        for i in 0..p.len() {
            work.add(1)?;
            let r = self.remap[i];
            if r != i {
                self.wedge[i] = self.wedge[r];
                self.wedge[r] = i;
            }
        }
        Ok(())
    }
    fn classify(
        &mut self,
        flags: Option<&[VertexFlags]>,
        options: SimplifyOptions,
        work: &mut Work,
    ) -> Result<(), Error> {
        let n = self.p.len();
        self.forward.fill(NONE);
        self.back.fill(NONE);
        for i in 0..n {
            work.add(1)?;
            for j in self.offsets[i]..self.offsets[i + 1] {
                work.add(1)?;
                let t = self.edges[j].next;
                if t == i {
                    self.forward[i] = i;
                    self.back[i] = i;
                } else if !self.has_edge(t, i, false, work)? {
                    self.back[t] = if self.back[t] == NONE { i } else { t };
                    self.forward[i] = if self.forward[i] == NONE { t } else { i };
                }
            }
        }
        for i in 0..n {
            work.add(1)?;
            self.kind[i] = if self.remap[i] != i {
                self.kind[self.remap[i]]
            } else if self.wedge[i] == i {
                let a = self.back[i];
                let b = self.forward[i];
                if a == NONE && b == NONE {
                    MAN
                } else if a != NONE && b != NONE && self.remap[a] == self.remap[b] && a != i {
                    SEAM
                } else if a != i && b != i {
                    BORDER
                } else {
                    LOCKED
                }
            } else if self.wedge[self.wedge[i]] == i {
                let w = self.wedge[i];
                let a = self.back[i];
                let b = self.forward[i];
                let c = self.back[w];
                let d = self.forward[w];
                if a != NONE
                    && a != i
                    && b != NONE
                    && b != i
                    && c != NONE
                    && c != w
                    && d != NONE
                    && d != w
                    && self.remap[a] == self.remap[d]
                    && self.remap[b] == self.remap[c]
                    && self.remap[a] != self.remap[b]
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
                if self.remap[i] != i {
                    self.kind[i] = self.kind[self.remap[i]];
                    continue;
                }
                let mut protect = false;
                let mut border = false;
                let mut v = i;
                loop {
                    work.add(1)?;
                    protect |= flag(flags, v, VertexFlags::PROTECT);
                    for j in self.offsets[v]..self.offsets[v + 1] {
                        work.add(1)?;
                        border |= !self.has_edge(self.edges[j].next, v, true, work)?;
                    }
                    v = self.wedge[v];
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
                    self.kind[self.remap[i]] = LOCKED;
                }
            }
            for i in 0..n {
                work.add(1)?;
                if self.kind[self.remap[i]] == LOCKED {
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
impl State {
    fn attribute_quadric(&self, t: [usize; 3]) -> (Q, [G; 32]) {
        let [i0, i1, i2] = t;
        let p0 = self.p[i0];
        let v0 = self.p[i1].sub(p0);
        let v1 = self.p[i2].sub(p0);
        let normal = v0.cross(v1);
        let w = sqrt(normal.dot(normal)) * 0.5;
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
        let mut g = [G::default(); 32];
        for (k, g) in g.iter_mut().enumerate().take(self.ac) {
            let a0 = self.a[i0 * self.ac + k];
            let a1 = self.a[i1 * self.ac + k];
            let a2 = self.a[i2 * self.ac + k];
            let gx = gx1 * (a1 - a0) + gx2 * (a2 - a0);
            let gy = gy1 * (a1 - a0) + gy2 * (a2 - a0);
            let gz = gz1 * (a1 - a0) + gz2 * (a2 - a0);
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
            *g = G {
                x: w * gx,
                y: w * gy,
                z: w * gz,
                w: w * gw,
            };
        }
        (q, g)
    }
    fn quadrics(
        &mut self,
        indices: &[u32],
        flags: Option<&[VertexFlags]>,
        options: SimplifyOptions,
        work: &mut Work,
    ) -> Result<(), Error> {
        for t in indices.as_chunks::<3>().0 {
            work.add(3)?;
            let [a, b, c] = [t[0] as usize, t[1] as usize, t[2] as usize];
            let q = Q::triangle(self.p[a], self.p[b], self.p[c], 1.0);
            for i in [a, b, c] {
                self.q[self.remap[i]].add(q);
            }
        }
        let factor = if options.contains(SimplifyOptions::REGULARIZE_LIGHT) {
            1e-2
        } else if options.contains(SimplifyOptions::REGULARIZE) {
            1e-1
        } else {
            1e-7
        };
        for i in 0..self.p.len() {
            work.add(1)?;
            if self.remap[i] != i {
                continue;
            }
            let w = self.q[i].w
                * if flag(flags, i, VertexFlags::PRIORITY) {
                    1.0
                } else {
                    factor
                };
            self.q[i].add(Q::point(self.p[i], w));
        }
        for t in indices.as_chunks::<3>().0 {
            for e in 0..3 {
                work.add(1)?;
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
                if (k0 == BORDER || k0 == SEAM) && self.forward[a] != b {
                    continue;
                }
                if (k1 == BORDER || k1 == SEAM) && self.back[b] != a {
                    continue;
                }
                if (k0 == FRINGE || k1 == FRINGE) && (k0 == COMPLEX || k1 == COMPLEX) {
                    continue;
                }
                let w = if k0 == SEAM || k1 == SEAM { 0.5 } else { 10.0 };
                let mut q = Q::edge(self.p[a], self.p[b], self.p[c], w);
                let mut qt = Q::triangle(self.p[a], self.p[b], self.p[c], w);
                qt.w = 0.0;
                q.add(qt);
                self.q[self.remap[a]].add(q);
                self.q[self.remap[b]].add(q);
            }
        }
        if self.ac > 0 {
            for t in indices.as_chunks::<3>().0 {
                work.add(3 + self.ac * 3)?;
                let t = [t[0] as usize, t[1] as usize, t[2] as usize];
                let (q, g) = self.attribute_quadric(t);
                for i in t {
                    self.aq[i].add(q);
                    for (k, &g) in g.iter().enumerate().take(self.ac) {
                        self.g[i * self.ac + k].add(g);
                    }
                }
            }
        }
        if self.q.iter().chain(&self.aq).any(|q| !q.finite())
            || self
                .g
                .iter()
                .any(|g| ![g.x, g.y, g.z, g.w].iter().all(|x| x.is_finite()))
        {
            return Err(Error::NumericalFailure);
        }
        Ok(())
    }
    fn pick(&mut self, indices: &[u32], capacity: usize, work: &mut Work) -> Result<usize, Error> {
        let mut count = 0;
        for t in indices.as_chunks::<3>().0 {
            if count + 3 > capacity {
                break;
            }
            for e in 0..3 {
                work.add(1)?;
                let a = t[e] as usize;
                let b = t[(e + 1) % 3] as usize;
                if self.remap[a] == self.remap[b] {
                    continue;
                }
                let k0 = self.kind[a] as usize;
                let k1 = self.kind[b] as usize;
                if CAN[k0][k1] | CAN[k1][k0] == 0 {
                    continue;
                }
                if OPP[k0][k1] != 0 && self.remap[b] > self.remap[a] {
                    continue;
                }
                if (k0 == BORDER as usize || k0 == SEAM as usize)
                    && k1 != MAN as usize
                    && self.forward[a] != b
                {
                    continue;
                }
                if (k1 == BORDER as usize || k1 == SEAM as usize)
                    && k0 != MAN as usize
                    && self.back[b] != a
                {
                    continue;
                }
                self.collapses[count] = if CAN[k0][k1] & CAN[k1][k0] != 0 {
                    Collapse {
                        a,
                        b,
                        bidi: true,
                        error: 0.0,
                    }
                } else if CAN[k0][k1] != 0 {
                    Collapse {
                        a,
                        b,
                        ..Collapse::default()
                    }
                } else {
                    Collapse {
                        a: b,
                        b: a,
                        ..Collapse::default()
                    }
                };
                count += 1;
            }
        }
        Ok(count)
    }
    fn complex_target(&self, v: usize, t: usize) -> usize {
        let r = self.remap[t];
        if self.forward[v] != NONE && self.remap[self.forward[v]] == r {
            self.forward[v]
        } else if self.back[v] != NONE && self.remap[self.back[v]] == r {
            self.back[v]
        } else {
            t
        }
    }
    fn seam_target(&self, a: usize, b: usize) -> (usize, usize) {
        let s0 = self.wedge[a];
        let s1 = if self.forward[a] == b {
            self.back[s0]
        } else {
            self.forward[s0]
        };
        (s0, if s1 != NONE { s1 } else { self.wedge[b] })
    }
    fn rank(&mut self, count: usize, work: &mut Work) -> Result<(), Error> {
        for i in 0..count {
            work.add(1)?;
            let c = self.collapses[i];
            let a = c.a;
            let b = c.b;
            let mut ei = self.q[self.remap[a]].error(self.p[b]);
            let mut ej = if c.bidi {
                self.q[self.remap[b]].error(self.p[a])
            } else {
                f32::MAX
            };
            if self.ac > 0 {
                work.add(self.ac * 2)?;
                ei += self.attribute_error(a, b);
                ej += if c.bidi {
                    self.attribute_error(b, a)
                } else {
                    0.0
                };
                if self.kind[a] == SEAM {
                    work.add(self.ac * 2)?;
                    let (s0, s1) = self.seam_target(a, b);
                    ei += self.attribute_error(s0, s1);
                    ej += if c.bidi {
                        self.attribute_error(s1, s0)
                    } else {
                        0.0
                    };
                } else {
                    if self.kind[a] == COMPLEX || self.kind[a] == FRINGE {
                        let mut v = self.wedge[a];
                        while v != a {
                            work.add(self.ac)?;
                            ei += self.attribute_error(v, self.complex_target(v, b));
                            v = self.wedge[v];
                        }
                    }
                    if (self.kind[b] == COMPLEX || self.kind[b] == FRINGE) && c.bidi {
                        let mut v = self.wedge[b];
                        while v != b {
                            work.add(self.ac)?;
                            ej += self.attribute_error(v, self.complex_target(v, a));
                            v = self.wedge[v];
                        }
                    }
                }
            }
            if !ei.is_finite() || !ej.is_finite() {
                return Err(Error::NumericalFailure);
            }
            let rev = c.bidi && ej < ei;
            self.collapses[i] = Collapse {
                a: if rev { b } else { a },
                b: if rev { a } else { b },
                error: if ej < ei { ej } else { ei },
                bidi: c.bidi,
            };
        }
        Ok(())
    }
    fn attribute_error(&self, source: usize, target: usize) -> f32 {
        let v = self.p[target];
        let q = self.aq[source];
        let mut r = q.eval(v);
        for k in 0..self.ac {
            let a = self.a[target * self.ac + k];
            let g = self.g[source * self.ac + k];
            let g = v.x * g.x + v.y * g.y + v.z * g.z + g.w;
            r += a * (a * q.w - 2.0 * g);
        }
        r.abs()
    }
    fn sort(&mut self, count: usize, work: &mut Work) -> Result<(), Error> {
        let mut h = [0usize; 2560];
        for c in &self.collapses[..count] {
            work.add(1)?;
            h[sort_key(c.error)] += 1;
        }
        let mut sum = 0;
        for v in &mut h {
            let n = *v;
            *v = sum;
            sum += n;
        }
        for i in 0..count {
            work.add(1)?;
            let k = sort_key(self.collapses[i].error);
            self.order[h[k]] = i;
            h[k] += 1;
        }
        Ok(())
    }
    fn flips(&self, a: usize, b: usize, work: &mut Work) -> Result<bool, Error> {
        for e in &self.edges[self.offsets[a]..self.offsets[a + 1]] {
            work.add(1)?;
            let x = self.cr[e.next];
            let y = self.cr[e.prev];
            if x == b || y == b || x == y {
                continue;
            }
            let pa = self.p[x];
            let eb = self.p[y].sub(pa);
            let ec = self.p[a].sub(pa);
            let ed = self.p[b].sub(pa);
            let nbc = eb.cross(ec);
            let nbd = eb.cross(ed);
            if nbc.dot(nbd) <= 0.25 * sqrt(nbc.dot(nbc) * nbd.dot(nbd)) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn perform(
        &mut self,
        count: usize,
        goal: usize,
        limit: f32,
        error: &mut f32,
        work: &mut Work,
    ) -> Result<usize, Error> {
        let mut edges = 0;
        let mut triangles = 0;
        let mut edge_goal = goal / 2;
        for i in 0..self.p.len() {
            work.add(1)?;
            self.cr[i] = i;
            self.locked[i] = false;
        }
        for i in 0..count {
            work.add(1)?;
            let c = self.collapses[self.order[i]];
            if c.error > limit || triangles >= goal {
                break;
            }
            let error_goal = if edge_goal < count {
                1.5 * self.collapses[self.order[edge_goal]].error
            } else {
                f32::MAX
            };
            if c.error > error_goal && c.error > *error && triangles > goal / 6 {
                break;
            }
            let a = c.a;
            let b = c.b;
            let r0 = self.remap[a];
            let r1 = self.remap[b];
            let kind = self.kind[a];
            if self.locked[r0] || self.locked[r1] {
                continue;
            }
            if self.flips(r0, r1, work)? {
                edge_goal += 1;
                continue;
            }
            self.cr[a] = b;
            if kind == COMPLEX || kind == FRINGE {
                let mut v = self.wedge[a];
                while v != a {
                    work.add(1)?;
                    self.cr[v] = self.complex_target(v, b);
                    v = self.wedge[v];
                }
            } else if kind == SEAM {
                let (s0, s1) = self.seam_target(a, b);
                self.cr[s0] = s1;
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
    fn update(&mut self, work: &mut Work) -> Result<(), Error> {
        for i in 0..self.p.len() {
            work.add(1)?;
            let t = self.cr[i];
            if t == i {
                continue;
            }
            let r0 = self.remap[i];
            let r1 = self.remap[t];
            if i == r0 {
                let q = self.q[r0];
                self.q[r1].add(q);
                if !self.q[r1].finite() {
                    return Err(Error::NumericalFailure);
                }
            }
            if self.ac > 0 {
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
                let l = loops[i];
                if l != NONE {
                    let r = self.cr[l];
                    loops[i] = if i == r {
                        if loops[l] != NONE {
                            self.cr[loops[l]]
                        } else {
                            NONE
                        }
                    } else {
                        r
                    };
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
    let n = positions.len();
    let m = indices.len();
    let ac = weights.iter().filter(|&&w| w > 0.0).count();
    let mut s = State::new(n, m, ac, ws, output)?;
    s.adjacency(indices, false, work)?;
    s.position_remap(positions, work)?;
    s.classify(flags, settings.options, work)?;
    work.add(n)?;
    let (lo, extent) = bounds(positions)?;
    let scale = if extent == 0.0 { 0.0 } else { 1.0 / extent };
    for i in 0..n {
        work.add(1)?;
        let p = positions.at(i)?;
        s.p[i] = V {
            x: (p[0] - lo[0]) * scale,
            y: (p[1] - lo[1]) * scale,
            z: (p[2] - lo[2]) * scale,
        };
        if ![s.p[i].x, s.p[i].y, s.p[i].z].iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
        if let Some(a) = attributes {
            let mut k = 0;
            for (j, &w) in weights.iter().enumerate() {
                work.add(1)?;
                if w > 0.0 {
                    let v = a.get(i, j).ok_or(Error::InvalidLayout)? * w;
                    if !v.is_finite() {
                        return Err(Error::NumericalFailure);
                    }
                    s.a[i * ac + k] = v;
                    k += 1;
                }
            }
        }
    }
    s.quadrics(indices, flags, settings.options, work)?;
    let mut dual = 0;
    for i in 0..n {
        work.add(1)?;
        if s.kind[i] == MAN || s.kind[i] == SEAM {
            dual += s.offsets[i + 1] - s.offsets[i];
        }
    }
    let capacity = m - dual / 2 + 3;
    let mut count = m;
    let mut error = 0.0;
    let error_scale = if settings.options.contains(SimplifyOptions::ERROR_ABSOLUTE) {
        extent
    } else {
        1.0
    };
    let limit = (settings.target_error * settings.target_error) / (error_scale * error_scale);
    work.add(m)?;
    out[..m].copy_from_slice(indices);
    while count > settings.target_index_count {
        s.adjacency(&out[..count], true, work)?;
        let nc = s.pick(&out[..count], capacity, work)?;
        if nc == 0 {
            break;
        }
        s.rank(nc, work)?;
        s.sort(nc, work)?;
        if s.perform(
            nc,
            (count - settings.target_index_count) / 3,
            limit,
            &mut error,
            work,
        )? == 0
        {
            break;
        }
        s.update(work)?;
        let mut write = 0;
        for i in (0..count).step_by(3) {
            work.add(3)?;
            let a = s.cr[out[i] as usize];
            let b = s.cr[out[i + 1] as usize];
            let c = s.cr[out[i + 2] as usize];
            let r0 = s.remap[a];
            let r1 = s.remap[b];
            let r2 = s.remap[c];
            if r0 != r1 && r0 != r2 && r1 != r2 {
                out[write] = a as u32;
                out[write + 1] = b as u32;
                out[write + 2] = c as u32;
                write += 3;
            }
        }
        count = write;
    }
    let error = sqrt(error) * error_scale;
    if !error.is_finite() {
        return Err(Error::NumericalFailure);
    }
    Ok(SimplifyResult {
        index_count: count,
        error,
    })
}
