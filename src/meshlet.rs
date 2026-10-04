#![allow(clippy::too_many_arguments)]
// Port of meshoptimizer 1.3 clusterizer.cpp (MIT, Arseny Kapoulkine).
use crate::math::{root_at_least, sqrt8, RootEstimate, SqrtCache, LANES};
use crate::processing::{cross, dot, finite, point, sub, Context};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

/// Current 1.3 descriptor: triangle offsets are consecutive byte offsets, without padding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Meshlet {
    /// Offset in the vertex-reference array.
    pub vertex_offset: u32,
    /// Offset in the byte triangle array.
    pub triangle_offset: u32,
    /// Number of referenced vertices.
    pub vertex_count: u32,
    /// Number of triangles (three bytes each).
    pub triangle_count: u32,
}
/// Complete tightly packed meshlet output.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Meshlets {
    /// Meshlet descriptors in construction order.
    pub meshlets: Vec<Meshlet>,
    /// Original vertex references.
    pub vertices: Vec<u32>,
    /// Local triangle bytes, without per-meshlet padding.
    pub triangles: Vec<u8>,
}
/// Limits used by all meshlet builders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshletSettings {
    /// Maximum vertices, 3 through 256.
    pub max_vertices: usize,
    /// Maximum triangles, 1 through 512.
    pub max_triangles: usize,
}
impl Default for MeshletSettings {
    fn default() -> Self {
        Self {
            max_vertices: 64,
            max_triangles: 124,
        }
    }
}
/// Used prefixes after caller-buffer meshlet construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MeshletSizes {
    /// Number of descriptors.
    pub meshlets: usize,
    /// Number of vertex references.
    pub vertices: usize,
    /// Number of triangle bytes.
    pub triangles: usize,
}
/// `meshopt_buildMeshletsBound`: conservative descriptor capacity, with checked arithmetic.
#[inline(always)]
pub fn build_meshlets_bound(
    index_count: usize,
    max_vertices: usize,
    max_triangles: usize,
) -> Result<usize, Error> {
    if !index_count.is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    if !(3..=256).contains(&max_vertices) || !(1..=512).contains(&max_triangles) {
        return Err(Error::InvalidParameter);
    }
    let v = max_vertices - 2;
    let a = small_divide(
        index_count.checked_add(v - 1).ok_or(Error::SizeOverflow)?,
        v,
    );
    let b = small_divide(
        (index_count / 3)
            .checked_add(max_triangles - 1)
            .ok_or(Error::SizeOverflow)?,
        max_triangles,
    );
    Ok(a.max(b))
}
// ceil(2^32 / d) for every divisor 1..=512 (D78).
static RECIPROCALS: [u64; 513] = {
    let mut table = [0u64; 513];
    let mut d = 1;
    while d < 513 {
        table[d] = (1u64 << 32).div_ceil(d as u64);
        d += 1;
    }
    table
};
// x / d for 1 <= d <= 512. Below 2^22 a 64-bit multiply by ceil(2^32 / d)
// is exact: it exceeds x / d by less than 2^22 / 2^32 = 2^-10, while the
// fractional part of x / d is at most 1 - 1/512 = 1 - 2^-9.
#[inline(always)]
fn small_divide(x: usize, d: usize) -> usize {
    match RECIPROCALS.get(d) {
        Some(&m) if d != 0 && x < 1 << 22 => ((x as u64 * m) >> 32) as usize,
        _ => x / d,
    }
}
pub(crate) struct Output<'a> {
    pub(crate) descriptors: &'a mut [Meshlet],
    pub(crate) vertices: &'a mut [u32],
    pub(crate) triangles: &'a mut [u8],
    pub(crate) current: Meshlet,
    pub(crate) count: usize,
}
pub(crate) trait UsedVertices {
    fn local(&self, vertex: usize) -> i16;
    fn set_local(&mut self, vertex: usize, local: i16);
}
impl UsedVertices for [i16] {
    #[inline(always)]
    fn local(&self, vertex: usize) -> i16 {
        self[vertex]
    }
    #[inline(always)]
    fn set_local(&mut self, vertex: usize, local: i16) {
        self[vertex] = local;
    }
}
impl<T> UsedVertices for Vec<T>
where
    [T]: UsedVertices,
{
    #[inline(always)]
    fn local(&self, vertex: usize) -> i16 {
        self.as_slice().local(vertex)
    }
    #[inline(always)]
    fn set_local(&mut self, vertex: usize, local: i16) {
        self.as_mut_slice().set_local(vertex, local);
    }
}
impl Output<'_> {
    #[inline(always)]
    pub(crate) fn append<U: UsedVertices + ?Sized>(
        &mut self,
        t: &[u32; 3],
        used: &mut U,
        s: MeshletSettings,
        split: bool,
        ctx: &mut Context<'_>,
    ) -> Result<bool, Error> {
        ctx.tick(1)?;
        // Work on a local copy of the open descriptor: the output slices could
        // otherwise alias it for the optimizer, forcing a reload per access.
        let mut m = self.current;
        let extra = t.iter().filter(|&&v| used.local(v as usize) < 0).count();
        let mut flush = false;
        if m.vertex_count as usize + extra > s.max_vertices
            || m.triangle_count as usize >= s.max_triangles
            || split
        {
            self.descriptors[self.count] = m;
            self.count += 1;
            ctx.tick(m.vertex_count as usize)?;
            for &v in &self.vertices[m.vertex_offset as usize..][..m.vertex_count as usize] {
                used.set_local(v as usize, -1);
            }
            m.vertex_offset += m.vertex_count;
            m.triangle_offset += m.triangle_count * 3;
            m.vertex_count = 0;
            m.triangle_count = 0;
            flush = true;
        }
        let off = m.triangle_offset as usize + m.triangle_count as usize * 3;
        let triangle = &mut self.triangles[off..off + 3];
        for (slot, &v) in triangle.iter_mut().zip(t) {
            if used.local(v as usize) < 0 {
                used.set_local(v as usize, m.vertex_count as i16);
                self.vertices[m.vertex_offset as usize + m.vertex_count as usize] = v;
                m.vertex_count += 1;
            }
            *slot = used.local(v as usize) as u8;
        }
        m.triangle_count += 1;
        self.current = m;
        Ok(flush)
    }
    fn finish(&mut self) -> MeshletSizes {
        if self.current.triangle_count != 0 {
            self.descriptors[self.count] = self.current;
            self.count += 1;
        }
        MeshletSizes {
            meshlets: self.count,
            vertices: (self.current.vertex_offset + self.current.vertex_count) as usize,
            triangles: (self.current.triangle_offset + self.current.triangle_count * 3) as usize,
        }
    }
}
fn validate(s: MeshletSettings, min: usize, weight: f32, split: f32) -> Result<(), Error> {
    build_meshlets_bound(0, s.max_vertices, s.max_triangles)?;
    if min == 0
        || min > s.max_triangles
        || !weight.is_finite()
        || !(0.0..=1.0).contains(&weight)
        || !split.is_finite()
        || split < 0.
    {
        return Err(Error::InvalidParameter);
    }
    Ok(())
}
#[derive(Clone, Copy, Default)]
struct Cone {
    p: [f32; 3],
    n: [f32; 3],
}
// `getMeshletCone` position: the accumulated centroid over the count.
#[inline(always)]
fn cone_position(acc: Cone, count: u32) -> [f32; 3] {
    let scale = if count == 0 { 0. } else { 1. / count as f32 };
    acc.p.map(|v| v * scale)
}
// `getMeshletCone` normal: acc * (1 / sqrt(|acc|^2)), or zero for |acc| = 0.
struct MeshletNormal {
    low: [f32; 3],
    high: [f32; 3],
    exact: Option<[f32; 3]>,
}
impl MeshletNormal {
    fn exact(acc: [f32; 3], roots: &mut SqrtCache) -> Self {
        let l = dot(acc, acc);
        let scale = if l == 0. { 0. } else { 1. / roots.sqrt(l) };
        let n = acc.map(|v| v * scale);
        Self {
            low: n,
            high: n,
            exact: Some(n),
        }
    }
    // Bounds of each rounded component: the table bounds the rounded scale
    // fl(1 / fl(sqrt(l))) without a division (D79), and acc_k * scale is
    // monotone in scale with the sign of acc_k.
    #[inline(always)]
    fn bounds(acc: [f32; 3], roots: &mut SqrtCache) -> Self {
        let l = dot(acc, acc);
        let Some(root) = RootEstimate::new(l) else {
            return Self::exact(acc, roots);
        };
        let (scale_low, scale_high) = root.inverse_table();
        let mut low = [0f32; 3];
        let mut high = [0f32; 3];
        for k in 0..3 {
            let (a, b) = (acc[k] * scale_low, acc[k] * scale_high);
            (low[k], high[k]) = if acc[k] >= 0. { (a, b) } else { (b, a) };
        }
        Self {
            low,
            high,
            exact: None,
        }
    }
}
// Adjacency offset, live triangle count and meshlet-local index of one vertex,
// packed so that a meshlet-vertex visit touches one record.
#[derive(Clone, Copy)]
struct VertexState {
    offset: u32,
    live: u32,
    local: i16,
}
impl Default for VertexState {
    fn default() -> Self {
        Self {
            offset: 0,
            live: 0,
            local: -1,
        }
    }
}
impl UsedVertices for [VertexState] {
    #[inline(always)]
    fn local(&self, vertex: usize) -> i16 {
        self[vertex].local
    }
    #[inline(always)]
    fn set_local(&mut self, vertex: usize, local: i16) {
        self[vertex].local = local;
    }
}
struct Adjacency {
    vertices: Vec<VertexState>,
    data: Vec<u32>,
}
fn adjacency(indices: &[u32], n: usize, ctx: &mut Context<'_>) -> Result<Adjacency, Error> {
    let mut vertices = ctx.alloc::<VertexState>(n)?;
    let mut data = ctx.alloc::<u32>(indices.len())?;
    ctx.tick(n + indices.len() * 2)?;
    for &v in indices {
        vertices[v as usize].live += 1;
    }
    let mut offset = 0;
    for vertex in vertices.iter_mut() {
        vertex.offset = offset;
        offset += vertex.live;
    }
    for (i, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        for &v in t {
            let o = &mut vertices[v as usize].offset;
            data[*o as usize] = i as u32;
            *o += 1;
        }
    }
    for vertex in vertices.iter_mut() {
        vertex.offset -= vertex.live;
    }
    Ok(Adjacency { vertices, data })
}
// Preserve the upstream packed eight-byte KD node.
#[derive(Clone, Copy, Default)]
struct Node {
    data: u32,
    meta: u32,
}
impl Node {
    fn axis(self) -> usize {
        (self.meta & 3) as usize
    }
    fn children(self) -> usize {
        (self.meta >> 2) as usize
    }
    fn set(&mut self, axis: usize, children: usize) -> Result<(), Error> {
        if children > 0x3fffffff {
            return Err(Error::SizeOverflow);
        }
        self.meta = (children as u32) << 2 | axis as u32;
        Ok(())
    }
}
#[cfg(test)]
mod packed_tests {
    use super::*;
    #[test]
    fn packed_children_check_actual_metadata_without_truncation() {
        let mut node = Node::default();
        node.set(3, 0x3fffffff).unwrap();
        assert_eq!(node.meta, u32::MAX);
        assert_eq!(node.axis(), 3);
        assert_eq!(node.children(), 0x3fffffff);
        assert_eq!(node.set(0, 0x40000000), Err(Error::SizeOverflow));
        assert_eq!(node.meta, u32::MAX);
    }
    #[test]
    fn small_divide_matches_integer_division() {
        let mut x = 0x9e37_79b9u32;
        for d in 1..=512usize {
            for edge in [
                0,
                1,
                d - 1,
                d,
                d + 1,
                (1 << 22) - 1,
                1 << 22,
                u32::MAX as usize,
            ] {
                assert_eq!(small_divide(edge, d), edge / d);
            }
            for _ in 0..512 {
                x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                assert_eq!(small_divide(x as usize, d), x as usize / d);
                let small = (x >> 10) as usize;
                assert_eq!(small_divide(small, d), small / d);
            }
        }
        assert_eq!(small_divide(usize::MAX, 7), usize::MAX / 7);
    }
}
// Split statistics of one kd range: Welford mean and variance per axis.
#[derive(Clone, Copy, Default)]
struct KdSplit {
    mean: [f32; 3],
    axis: usize,
}
impl KdSplit {
    #[inline(always)]
    fn finish(mean: [f32; 3], vars: [f32; 3]) -> Self {
        let axis = if vars[0] >= vars[1] && vars[0] >= vars[2] {
            0
        } else if vars[1] >= vars[2] {
            1
        } else {
            2
        };
        Self { mean, axis }
    }
}
#[inline(always)]
fn welford(p: [f32; 3], mean: &mut [f32; 3], vars: &mut [f32; 3], runs: f32) {
    for k in 0..3 {
        let delta = p[k] - mean[k];
        mean[k] += delta * runs;
        vars[k] += delta * (p[k] - mean[k]);
    }
}
fn kd_stats(order: &[u32], cones: &[Cone]) -> KdSplit {
    let (mut mean, mut vars) = ([0.; 3], [0.; 3]);
    let (mut runc, mut runs) = (1f32, 1f32);
    for &v in order {
        welford(cones[v as usize].p, &mut mean, &mut vars, runs);
        runc += 1.;
        runs = 1. / runc;
    }
    KdSplit::finish(mean, vars)
}
// Statistics of two sibling ranges in one loop: the two Welford chains are
// independent, and both use the same running weight at the same position.
fn kd_stats2(a: &[u32], b: &[u32], cones: &[Cone]) -> (KdSplit, KdSplit) {
    let (mut am, mut av, mut bm, mut bv) = ([0.; 3], [0.; 3], [0.; 3], [0.; 3]);
    let (mut runc, mut runs) = (1f32, 1f32);
    let common = a.len().min(b.len());
    for (&x, &y) in a[..common].iter().zip(&b[..common]) {
        welford(cones[x as usize].p, &mut am, &mut av, runs);
        welford(cones[y as usize].p, &mut bm, &mut bv, runs);
        runc += 1.;
        runs = 1. / runc;
    }
    let (rest, mean, vars) = if a.len() > common {
        (&a[common..], &mut am, &mut av)
    } else {
        (&b[common..], &mut bm, &mut bv)
    };
    for &v in rest {
        welford(cones[v as usize].p, mean, vars, runs);
        runc += 1.;
        runs = 1. / runc;
    }
    (KdSplit::finish(am, av), KdSplit::finish(bm, bv))
}
// `kdtreeBuild`. A split node computes its children's statistics together
// (they depend only on the partition, which later recursion does not touch);
// each child still charges its work and checks its mean when it is built,
// so budget failures and errors occur exactly where the recursion did.
fn kd_build(
    nodes: &mut [Node],
    offset: usize,
    order: &mut [u32],
    cones: &[Cone],
    depth: usize,
    split: Option<KdSplit>,
    ctx: &mut Context<'_>,
) -> Result<usize, Error> {
    let n = order.len();
    let mut middle = 0;
    let mut stats = KdSplit::default();
    if n > 8 {
        ctx.tick(n * 2)?;
        stats = split.unwrap_or_else(|| kd_stats(order, cones));
        let (mean, axis) = (stats.mean, stats.axis);
        finite(mean[axis])?;
        for i in 0..n {
            let v = cones[order[i] as usize].p[axis];
            order.swap(middle, i);
            middle += usize::from(v < mean[axis]);
        }
    }
    if n <= 8 || middle <= 4 || middle >= n - 4 || depth >= 50 {
        if n > 0x3fffffff {
            return Err(Error::SizeOverflow);
        }
        ctx.tick(n)?;
        for (i, &v) in order.iter().enumerate() {
            nodes[offset + i] = Node {
                data: v,
                meta: 3 | ((if i == 0 { n as u32 } else { 0x3fffffff }) << 2),
            };
        }
        return Ok(offset + n);
    }
    nodes[offset].data = stats.mean[stats.axis].to_bits();
    let (left, right) = order.split_at_mut(middle);
    let (left_split, right_split) = match (left.len() > 8, right.len() > 8) {
        (true, true) => {
            let (l, r) = kd_stats2(left, right, cones);
            (Some(l), Some(r))
        }
        (true, false) => (Some(kd_stats(left, cones)), None),
        (false, true) => (None, Some(kd_stats(right, cones))),
        (false, false) => (None, None),
    };
    let next = kd_build(nodes, offset + 1, left, cones, depth + 1, left_split, ctx)?;
    nodes[offset].set(stats.axis, next - offset - 1)?;
    kd_build(nodes, next, right, cones, depth + 1, right_split, ctx)
}
fn nearest<const CHECK: bool>(
    nodes: &mut [Node],
    root: usize,
    cones: &[Cone],
    emitted: &[u8],
    p: [f32; 3],
    result: &mut u32,
    limit: &mut f32,
    roots: &mut SqrtCache,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    ctx.tick(1)?;
    let node = nodes[root];
    if node.children() == 0 {
        return Ok(());
    }
    if node.axis() == 3 && !CHECK && ctx.work.covers(node.children())? {
        // Moderate leaves cannot fail: one inline loop, charged once.
        let count = node.children();
        let mut inactive = true;
        for item in &nodes[root..root + count] {
            let v = item.data as usize;
            if emitted[v] == EMITTED {
                continue;
            }
            inactive = false;
            let d = sub(cones[v].p, p);
            let square = dot(d, d);
            if lower_distance(d) >= *limit || root_at_least(square, *limit) {
                continue;
            }
            let distance = roots.sqrt(square);
            if distance < *limit {
                *result = v as u32;
                *limit = distance;
            }
        }
        ctx.tick(count)?;
        if inactive {
            nodes[root].set(3, 0)?;
        }
    } else if node.axis() == 3 {
        let mut inactive = true;
        ctx.scan(0..node.children(), |_ctx, i| {
            let v = nodes[root + i].data as usize;
            if emitted[v] == EMITTED {
                return Ok(());
            }
            inactive = false;
            let d = sub(cones[v].p, p);
            if !CHECK && (lower_distance(d) >= *limit || root_at_least(dot(d, d), *limit)) {
                return Ok(());
            }
            let distance = roots.sqrt(number::<CHECK>(dot(d, d))?);
            if distance < *limit {
                *result = v as u32;
                *limit = distance;
            }
            Ok(())
        })?;
        if inactive {
            nodes[root].set(3, 0)?;
        }
    } else {
        let delta = p[node.axis()] - f32::from_bits(node.data);
        let first = if delta <= 0. { 0 } else { node.children() };
        let second = first ^ node.children();
        if nodes[root + 1 + first].children() | nodes[root + 1 + second].children() == 0 {
            nodes[root].set(node.axis(), 0)?;
        }
        nearest::<CHECK>(
            nodes,
            root + 1 + first,
            cones,
            emitted,
            p,
            result,
            limit,
            roots,
            ctx,
        )?;
        if delta.abs() <= *limit {
            nearest::<CHECK>(
                nodes,
                root + 1 + second,
                cones,
                emitted,
                p,
                result,
                limit,
                roots,
                ctx,
            )?;
        }
    }
    Ok(())
}
// Each absolute component is a conservative lower bound on the correctly
// rounded Euclidean distance. Use only after the finite-coordinate envelope.
#[inline(always)]
fn lower_distance(d: [f32; 3]) -> f32 {
    // Plain comparisons: moderate inputs are finite, so NaN-aware max is unneeded.
    let (x, y, z) = (d[0].abs(), d[1].abs(), d[2].abs());
    let m = if x > y { x } else { y };
    let m = if m > z { m } else { z };
    // Below this envelope a rounded square can be subnormal or zero.
    if m >= 1.0e-18 {
        m
    } else {
        0.0
    }
}
#[inline(always)]
fn number<const CHECK: bool>(value: f32) -> Result<f32, Error> {
    if CHECK {
        finite(value)
    } else {
        Ok(value)
    }
}
#[inline(always)]
fn distance<const CHECK: bool>(
    a: [f32; 3],
    b: [f32; 3],
    roots: &mut SqrtCache,
) -> Result<f32, Error> {
    let d = sub(a, b);
    Ok(roots.sqrt(number::<CHECK>(dot(d, d))?))
}
// The selected candidate's score is kept as a bracket [lo, hi] around the
// exact upstream score. `known` means lo == hi == the exact score. The exact
// root is computed only when a comparison cannot be decided from brackets.
struct Selection {
    triangle: u32,
    priority: i32,
    lo: f32,
    hi: f32,
    known: bool,
    square: f32,
}
impl Selection {
    fn new() -> Self {
        Self {
            triangle: u32::MAX,
            priority: 5,
            lo: f32::MAX,
            hi: f32::MAX,
            known: true,
            square: 0.,
        }
    }
}
struct CandidateInput<'a> {
    triangles: &'a [[u32; 3]],
    vertices: &'a [VertexState],
    cones: &'a [Cone],
    meshlet: Cone,
    weight: f32,
    radius: f32,
    // Reciprocal bounds around 1 / radius; `bounded` when both are finite
    // and the radius is a positive normal value.
    inverse_low: f32,
    inverse_high: f32,
    bounded: bool,
    // Bounded mode: bounds of the normalized meshlet normal, from root bounds
    // of its squared length; the exact normal is computed only on demand.
    accumulated: [f32; 3],
    normal_low: [f32; 3],
    normal_high: [f32; 3],
}
// Mutable state of one neighbor search: the root memo and the exact meshlet
// normal once computed. Kept out of CandidateInput so `&CandidateInput`
// stays freeze-only and its fields can be held in registers.
struct Search<'r> {
    roots: &'r mut SqrtCache,
    normal: Option<[f32; 3]>,
    // Candidates come from the meshlet's candidate list rather than in visit
    // order: exact ties are then broken by the upstream visit order.
    listed: bool,
    data: &'r [u32],
}
impl CandidateInput<'_> {
    #[inline(always)]
    fn one<const CHECK: bool>(
        &self,
        tri: u32,
        local: u16,
        selected: &mut Selection,
        search: &mut Search<'_>,
    ) -> Result<(), Error> {
        let [a, b, c] = self.triangles[tri as usize].map(|v| v as usize);
        let (av, bv, cv) = (self.vertices[a], self.vertices[b], self.vertices[c]);
        // A live triangle is listed under each of its meshlet vertices. When one
        // of them precedes the vertex being scanned, this visit repeats an
        // earlier one with identical priority and score; under the strict
        // lexicographic selection (including NaN scores) it cannot change it.
        if (av.local as u16).min(bv.local as u16).min(cv.local as u16) < local {
            return Ok(());
        }
        self.evaluate::<CHECK>(tri, [av, bv, cv], selected, search)
    }
    #[inline(always)]
    fn evaluate<const CHECK: bool>(
        &self,
        tri: u32,
        [av, bv, cv]: [VertexState; 3],
        selected: &mut Selection,
        search: &mut Search<'_>,
    ) -> Result<(), Error> {
        let extra = i32::from(av.local < 0) + i32::from(bv.local < 0) + i32::from(cv.local < 0);
        let (al, bl, cl) = (av.live, bv.live, cv.live);
        let priority = if extra == 0 {
            0
        } else if al == 1 || bl == 1 || cl == 1 {
            1
        } else if i32::from(al == 2) + i32::from(bl == 2) + i32::from(cl == 2) >= 2 {
            1 + extra
        } else {
            2 + extra
        };
        if priority > selected.priority {
            return Ok(());
        }
        let cone = self.cones[tri as usize];
        let dp = sub(cone.p, self.meshlet.p);
        let square = dot(dp, dp);
        let square = if CHECK { finite(square)? } else { square };
        if !CHECK && self.bounded {
            self.bracketed(tri, priority, cone.n, square, selected, search);
            return Ok(());
        }
        let clamped = self.clamped(dot(cone.n, self.meshlet.n));
        let distance = search.roots.sqrt(square);
        let score = self.score(distance, clamped);
        // Zero-area geometry retains the upstream NaN score/tie behavior.
        if priority < selected.priority || score < selected.hi {
            *selected = Selection {
                triangle: tri,
                priority,
                lo: score,
                hi: score,
                known: true,
                square,
            };
        }
        Ok(())
    }
    #[inline(always)]
    fn clamped(&self, spread: f32) -> f32 {
        let cc = 1. - spread * self.weight;
        if cc < 1e-3 {
            1e-3
        } else {
            cc
        }
    }
    #[inline(always)]
    fn score(&self, distance: f32, clamped: f32) -> f32 {
        (1. + distance / self.radius * (1. - self.weight)) * clamped
    }
    // Bounded mode (moderate inputs, positive normal radius with finite
    // reciprocal bounds): every operand is finite, the radius and cone factor
    // are positive and 0 <= weight <= 1. Each rounded operation of the score
    // is then monotone in each input: nondecreasing in the distance and in
    // the cone factor, the cone factor nonincreasing in the spread, and each
    // rounded product of the spread nondecreasing (cone component >= 0) or
    // nonincreasing (< 0) in the meshlet normal component. Scores from the
    // bounding inputs therefore bound the exact upstream score, and a
    // comparison decided by disjoint bounds equals the exact comparison.
    // Multiplying by a reciprocal rounded away from 1/radius by more than
    // its own rounding keeps the quotient on one side of the real d / radius.
    #[inline(always)]
    fn score_low(&self, distance: f32, clamped: f32) -> f32 {
        (1. + distance * self.inverse_low * (1. - self.weight)) * clamped
    }
    #[inline(always)]
    fn score_high(&self, distance: f32, clamped: f32) -> f32 {
        (1. + distance * self.inverse_high * (1. - self.weight)) * clamped
    }
    // Bounds of the cone factor from the meshlet normal bounds.
    #[inline(always)]
    fn clamped_bounds(&self, n: [f32; 3], low: bool) -> f32 {
        // Each exact term n_k * normal_k lies between the products with the
        // normal bounds, so their larger (smaller) values bound the spread.
        let (lo, hi) = (self.normal_low, self.normal_high);
        let term = |k: usize| {
            let (a, b) = (n[k] * lo[k], n[k] * hi[k]);
            if (a > b) == low {
                a
            } else {
                b
            }
        };
        // The cone factor decreases with the spread.
        self.clamped(term(0) + term(1) + term(2))
    }
    // The exact meshlet normal, computed at most once per neighbor search.
    #[cold]
    #[inline(never)]
    fn exact_normal(&self, search: &mut Search<'_>) -> [f32; 3] {
        if let Some(n) = search.normal {
            return n;
        }
        let l = dot(self.accumulated, self.accumulated);
        let scale = if l == 0. {
            0.
        } else {
            1. / search.roots.sqrt(l)
        };
        let n = self.accumulated.map(|v| v * scale);
        search.normal = Some(n);
        n
    }
    #[inline(always)]
    fn bracketed(
        &self,
        tri: u32,
        priority: i32,
        normal: [f32; 3],
        square: f32,
        selected: &mut Selection,
        search: &mut Search<'_>,
    ) {
        let clamped_low = self.clamped_bounds(normal, true);
        let (lo, hi) = match RootEstimate::new(square) {
            Some(root) => {
                // Division-free table bounds reject most losers (listed
                // candidates only on a strict loss: they may win a tie).
                let low = self.score_low(root.table().0, clamped_low);
                if priority == selected.priority
                    && (low > selected.hi || (low == selected.hi && !search.listed))
                {
                    return;
                }
                let (dlo, dhi) = root.bounds();
                (
                    self.score_low(dlo, clamped_low),
                    self.score_high(dhi, self.clamped_bounds(normal, false)),
                )
            }
            None => {
                let d = search.roots.sqrt(square);
                (
                    self.score_low(d, clamped_low),
                    self.score_high(d, self.clamped_bounds(normal, false)),
                )
            }
        };
        if priority == selected.priority {
            // Listed candidates may tie with an earlier-visited selection.
            if lo > selected.hi || (lo == selected.hi && !search.listed) {
                return;
            }
            if hi >= selected.lo {
                self.resolve(tri, priority, square, selected, search);
                return;
            }
        }
        *selected = Selection {
            triangle: tri,
            priority,
            lo,
            hi,
            known: false,
            square,
        };
    }
    // Upstream visit order of a candidate: its earliest meshlet vertex (by
    // local index), then its position in that vertex's live adjacency list.
    fn visit(&self, tri: u32, data: &[u32]) -> (u16, usize) {
        let mut first = (u16::MAX, 0usize);
        for &v in &self.triangles[tri as usize] {
            let state = self.vertices[v as usize];
            if (state.local as u16) < first.0 {
                first = (state.local as u16, v as usize);
            }
        }
        let state = self.vertices[first.1];
        let list = &data[state.offset as usize..][..state.live as usize];
        (
            first.0,
            list.iter().position(|&t| t == tri).unwrap_or(list.len()),
        )
    }
    // Undecided equal-priority comparison: compute both exact scores.
    #[cold]
    #[inline(never)]
    fn resolve(
        &self,
        tri: u32,
        priority: i32,
        square: f32,
        selected: &mut Selection,
        search: &mut Search<'_>,
    ) {
        let n = self.exact_normal(search);
        let mut exact = |tri: u32, square: f32| {
            let clamped = self.clamped(dot(self.cones[tri as usize].n, n));
            self.score(search.roots.sqrt(square), clamped)
        };
        if !selected.known {
            let score = exact(selected.triangle, selected.square);
            selected.lo = score;
            selected.hi = score;
            selected.known = true;
        }
        let score = exact(tri, square);
        if score < selected.hi
            || (search.listed
                && score == selected.hi
                && self.visit(tri, search.data) < self.visit(selected.triangle, search.data))
        {
            *selected = Selection {
                triangle: tri,
                priority,
                lo: score,
                hi: score,
                known: true,
                square,
            };
        }
    }
    // `getNeighborTriangle`: scan the live adjacency of every meshlet vertex.
    // Visits are charged once at the end while the budget covers the visited
    // prefix plus the next vertex's list (the per-vertex scans' condition);
    // a failing visit charges the visited prefix. Once a list is not covered,
    // the remaining visits keep the per-visit exhaustion point.
    //
    // In bounded mode the meshlet's candidate list (each live adjacent
    // triangle once) replaces the per-vertex scan: the same visits are
    // charged, and the selection is order-independent because scores are
    // finite and exact ties are broken by the upstream visit order.
    #[inline(always)]
    fn neighbors<const CHECK: bool>(
        &self,
        active: &mut Active,
        data: &[u32],
        roots: &mut SqrtCache,
        normal: Option<[f32; 3]>,
        listed: Option<&mut Candidates>,
        emitted: &[u8],
        ctx: &mut Context<'_>,
    ) -> Result<u32, Error> {
        if let Some(listed) = listed.filter(|listed| !CHECK && !listed.overflow) {
            let mut total = 0usize;
            let mut kept = 0;
            for k in 0..active.len {
                let v = active.list[k];
                let live = self.vertices[v as usize].live as usize;
                if live != 0 {
                    active.list[kept] = v;
                    kept += 1;
                    total += live;
                }
            }
            active.len = kept;
            if ctx.work.covers(total)? {
                let mut search = Search {
                    roots,
                    normal,
                    listed: true,
                    data,
                };
                let mut selected = Selection::new();
                let mut i = 0;
                while i < listed.len {
                    let tri = listed.list[i];
                    if emitted[tri as usize] == EMITTED {
                        listed.len -= 1;
                        listed.list[i] = listed.list[listed.len];
                        continue;
                    }
                    let [a, b, c] = self.triangles[tri as usize].map(|v| v as usize);
                    let states = [self.vertices[a], self.vertices[b], self.vertices[c]];
                    self.evaluate::<CHECK>(tri, states, &mut selected, &mut search)?;
                    i += 1;
                }
                ctx.tick(total)?;
                return Ok(selected.triangle);
            }
        }
        let mut search = Search {
            roots,
            normal,
            listed: false,
            data,
        };
        let mut selected = Selection::new();
        let mut visited = 0usize;
        let mut kept = 0;
        for k in 0..active.len {
            let v = active.list[k];
            let state = self.vertices[v as usize];
            let live = state.live as usize;
            // Lists only shrink: drop exhausted vertices. An empty list is
            // always covered, since visited never exceeds the budget.
            if live == 0 {
                continue;
            }
            active.list[kept] = v;
            kept += 1;
            let local = state.local as u16;
            if !ctx.work.covers(visited + live)? {
                ctx.tick(visited)?;
                for &v in &active.list[k..active.len] {
                    let state = self.vertices[v as usize];
                    let off = state.offset as usize;
                    for &tri in &data[off..off + state.live as usize] {
                        ctx.tick(1)?;
                        self.one::<CHECK>(tri, state.local as u16, &mut selected, &mut search)?;
                    }
                }
                return Ok(selected.triangle);
            }
            let off = state.offset as usize;
            for &tri in &data[off..off + live] {
                visited += 1;
                if let Err(error) = self.one::<CHECK>(tri, local, &mut selected, &mut search) {
                    ctx.tick(visited)?;
                    return Err(error);
                }
            }
        }
        active.len = kept;
        ctx.tick(visited)?;
        Ok(selected.triangle)
    }
}
// `emitted` states: free, emitted, or listed as a candidate of the meshlet.
const EMITTED: u8 = 1;
const LISTED: u8 = 2;
// Live triangles adjacent to the current meshlet, each once (bounded mode).
// A meshlet whose candidates exceed the fixed capacity uses the scan.
struct Candidates {
    list: [u32; 1024],
    len: usize,
    overflow: bool,
}
impl Candidates {
    fn new() -> Self {
        Self {
            list: [0; 1024],
            len: 0,
            overflow: false,
        }
    }
    fn clear(&mut self, emitted: &mut [u8]) {
        for &t in &self.list[..self.len] {
            if emitted[t as usize] == LISTED {
                emitted[t as usize] = 0;
            }
        }
        self.len = 0;
        self.overflow = false;
    }
    fn add(&mut self, live: &[u32], emitted: &mut [u8]) {
        for &t in live {
            if emitted[t as usize] == 0 {
                if self.len == self.list.len() {
                    self.overflow = true;
                    return;
                }
                emitted[t as usize] = LISTED;
                self.list[self.len] = t;
                self.len += 1;
            }
        }
    }
}
// Current-meshlet vertices whose live adjacency may be nonempty, in meshlet
// (local index) order. Vertices are appended as the meshlet grows and dropped
// once their lists are exhausted; live counts never increase again.
struct Active {
    list: [u32; 256],
    len: usize,
    offset: u32,
    synced: usize,
}
impl Active {
    fn new() -> Self {
        Self {
            list: [0; 256],
            len: 0,
            offset: 0,
            synced: 0,
        }
    }
    #[inline(always)]
    fn sync(
        &mut self,
        out: &Output<'_>,
        adj: &Adjacency,
        listed: Option<(&mut Candidates, &mut [u8])>,
    ) {
        let count = out.current.vertex_count as usize;
        let reset = out.current.vertex_offset != self.offset || count < self.synced;
        if reset {
            self.offset = out.current.vertex_offset;
            self.len = 0;
            self.synced = 0;
        }
        let added = &out.vertices[self.offset as usize..][self.synced..count];
        if let Some((candidates, emitted)) = listed {
            if reset {
                candidates.clear(emitted);
            }
            for &v in added {
                let state = adj.vertices[v as usize];
                if !candidates.overflow {
                    let off = state.offset as usize;
                    candidates.add(&adj.data[off..off + state.live as usize], emitted);
                }
            }
        }
        for &v in added {
            self.list[self.len] = v;
            self.len += 1;
        }
        self.synced = count;
    }
}
fn flex<const CHECK: bool>(
    out: &mut Output<'_>,
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min: usize,
    weight: f32,
    split_factor: f32,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    if indices.is_empty() {
        return Ok(());
    }
    let n = indices.len() / 3;
    let node_count = n.checked_mul(2).ok_or(Error::SizeOverflow)?;
    // One exact per-call memo with a fixed table: no per-lookup dispatch.
    let mut roots = SqrtCache::new();
    let mut adj = adjacency(indices, p.len(), ctx)?;
    let mut emitted = ctx.alloc::<u8>(n)?;
    let mut cones = ctx.alloc::<Cone>(n)?;
    let mut mesh_area = 0.;
    ctx.tick(n)?;
    // Triangle areas are independent: take their roots eight at a time. Every
    // failure is NumericalFailure and work is charged above, so the grouped
    // check order is unobservable; mesh_area still sums in triangle order.
    let triangles = indices.as_chunks::<3>().0;
    for (group, out_cones) in triangles.chunks(LANES).zip(cones.chunks_mut(LANES)) {
        let mut normals = [[0f32; 3]; LANES];
        let mut squares = [1f32; LANES];
        for (k, (t, cone)) in group.iter().zip(out_cones.iter_mut()).enumerate() {
            let a = point(p, t[0]);
            let b = point(p, t[1]);
            let c = point(p, t[2]);
            let normal = cross(sub(b, a), sub(c, a));
            squares[k] = number::<CHECK>(dot(normal, normal))?;
            normals[k] = normal;
            for j in 0..3 {
                cone.p[j] = number::<CHECK>((a[j] + b[j] + c[j]) / 3.)?;
            }
        }
        let areas = sqrt8(squares);
        for (k, cone) in out_cones.iter_mut().enumerate() {
            let area = areas[k];
            let inv = if area == 0. { 0. } else { 1. / area };
            for (n, &v) in cone.n.iter_mut().zip(&normals[k]) {
                *n = number::<CHECK>(v * inv)?;
            }
            mesh_area = number::<CHECK>(mesh_area + area)?;
        }
    }
    let radius = roots.sqrt(number::<CHECK>(
        mesh_area / n as f32 * 0.5 * s.max_triangles as f32,
    )?) * 0.5;
    let mut order = ctx.alloc(n)?;
    for (i, v) in order.iter_mut().enumerate() {
        *v = i as u32;
    }
    let mut nodes = ctx.alloc(node_count)?;
    kd_build(&mut nodes, 0, &mut order, &cones, 0, None, ctx)?;
    ctx.tick(n.checked_mul(2).ok_or(Error::SizeOverflow)?)?;
    let mut corner = [f32::MAX; 3];
    for c in &cones {
        for (k, v) in corner.iter_mut().enumerate() {
            if *v > c.p[k] {
                *v = c.p[k];
            }
        }
    }
    let mut initial = u32::MAX;
    let mut score = f32::MAX;
    for (i, c) in cones.iter().enumerate() {
        if !CHECK && initial != u32::MAX && {
            let d = sub(c.p, corner);
            lower_distance(d) >= score || root_at_least(dot(d, d), score)
        } {
            continue;
        }
        let d = distance::<CHECK>(c.p, corner, &mut roots)?;
        if initial == u32::MAX || d < score {
            initial = i as u32;
            score = d;
        }
    }
    let mut seeds = [0u32; 256];
    let mut seed_count = 0;
    let mut acc = Cone::default();
    let mut active = Active::new();
    // 1 / radius rounds by at most 2^-24; scaling by 1 -+ 2^-21 (rounding by
    // another 2^-24) moves each bound strictly past the real reciprocal.
    let inverse = 1. / radius;
    let inverse_low = inverse * 0.999_999_5;
    let inverse_high = inverse * 1.000_000_5;
    let bounded = !CHECK
        && radius.is_normal()
        && radius > 0.
        && inverse_low.is_normal()
        && inverse_high.is_finite();
    // Bounded mode lists candidates on meshes large enough to repay the list
    // upkeep.
    // (Not constructed otherwise: the list is 4 KiB of stack to clear.)
    let mut candidates = (bounded && n >= 256).then(Candidates::new);
    loop {
        ctx.tick(1)?;
        let mc_position = cone_position(acc, out.current.triangle_count);
        let mut best;
        if out.count == 0 && out.current.triangle_count == 0 {
            best = initial;
        } else {
            // In bounded mode the meshlet normal is only bounded here (its root
            // is taken on demand); otherwise it is exact, as upstream. The
            // initial seed does not search neighbors and needs no normal.
            let normal = if bounded {
                MeshletNormal::bounds(acc.n, &mut roots)
            } else {
                MeshletNormal::exact(acc.n, &mut roots)
            };
            let input = CandidateInput {
                triangles: indices.as_chunks::<3>().0,
                vertices: &adj.vertices,
                cones: &cones,
                meshlet: Cone {
                    p: mc_position,
                    n: normal.exact.unwrap_or_default(),
                },
                weight,
                radius,
                inverse_low,
                inverse_high,
                bounded,
                accumulated: acc.n,
                normal_low: normal.low,
                normal_high: normal.high,
            };
            active.sync(
                out,
                &adj,
                candidates
                    .as_mut()
                    .map(|candidates| (candidates, emitted.as_mut_slice())),
            );
            best = input.neighbors::<CHECK>(
                &mut active,
                &adj.data,
                &mut roots,
                normal.exact,
                candidates.as_mut(),
                &emitted,
                ctx,
            )?;
        }
        let mut split = false;
        if best == u32::MAX {
            let mut d = f32::MAX;
            nearest::<CHECK>(
                &mut nodes,
                0,
                &cones,
                &emitted,
                mc_position,
                &mut best,
                &mut d,
                &mut roots,
                ctx,
            )?;
            split = out.current.triangle_count as usize >= min
                && split_factor > 0.
                && d > radius * split_factor;
        }
        if best == u32::MAX {
            break;
        }
        let extra = indices[best as usize * 3..][..3]
            .iter()
            .filter(|&&v| adj.vertices[v as usize].local < 0)
            .count();
        if split
            || out.current.vertex_count as usize + extra > s.max_vertices
            || out.current.triangle_count as usize >= s.max_triangles
        {
            let mut keep = 0;
            for i in 0..seed_count {
                if emitted[seeds[i] as usize] != EMITTED {
                    seeds[keep] = seeds[i];
                    keep += 1;
                }
            }
            seed_count = keep.min(252);
            let mut candidates = [u32::MAX; 4];
            let mut lives = [u32::MAX; 4];
            let mut scores = [f32::MAX; 4];
            for &v in &out.vertices[out.current.vertex_offset as usize..]
                [..out.current.vertex_count as usize]
            {
                let off = adj.vertices[v as usize].offset as usize;
                let mut neighbor = u32::MAX;
                let mut live = u32::MAX;
                ctx.scan(
                    adj.data[off..off + adj.vertices[v as usize].live as usize]
                        .iter()
                        .copied(),
                    |_ctx, tri| {
                        let t = &indices[tri as usize * 3..][..3];
                        let l = adj.vertices[t[0] as usize].live
                            + adj.vertices[t[1] as usize].live
                            + adj.vertices[t[2] as usize].live;
                        if l < live {
                            neighbor = tri;
                            live = l;
                        }
                        Ok(())
                    },
                )?;
                if neighbor == u32::MAX {
                    continue;
                }
                let d = distance::<CHECK>(cones[neighbor as usize].p, corner, &mut roots)?;
                for j in 0..4 {
                    if live < lives[j] || (live == lives[j] && d <= scores[j]) {
                        candidates[j] = neighbor;
                        lives[j] = live;
                        scores[j] = d;
                        break;
                    }
                }
            }
            for tri in candidates {
                if tri != u32::MAX {
                    seeds[seed_count] = tri;
                    seed_count += 1;
                }
            }
            let mut seed = u32::MAX;
            let mut live = u32::MAX;
            let mut score = f32::MAX;
            ctx.scan(seeds[..seed_count].iter().copied(), |_ctx, tri| {
                let t = &indices[tri as usize * 3..][..3];
                let l = adj.vertices[t[0] as usize].live
                    + adj.vertices[t[1] as usize].live
                    + adj.vertices[t[2] as usize].live;
                if !CHECK
                    && (l > live
                        || (l == live && {
                            let d = sub(cones[tri as usize].p, corner);
                            lower_distance(d) >= score || root_at_least(dot(d, d), score)
                        }))
                {
                    return Ok(());
                }
                let d = distance::<CHECK>(cones[tri as usize].p, corner, &mut roots)?;
                if l < live || (l == live && d < score) {
                    seed = tri;
                    live = l;
                    score = d;
                }
                Ok(())
            })?;
            if seed != u32::MAX {
                best = seed;
            }
        }
        let t = &indices.as_chunks::<3>().0[best as usize];
        if out.append(t, &mut adj.vertices, s, split, ctx)? {
            acc = Cone::default();
        }
        for &v in t {
            let state = &mut adj.vertices[v as usize];
            let list = &mut adj.data[state.offset as usize..][..state.live as usize];
            // Charge the visited prefix of the search (the whole list when
            // absent); a short budget keeps the per-visit exhaustion point.
            let found = if ctx.work.covers(list.len())? {
                let found = list.iter().position(|&tri| tri == best);
                ctx.tick(found.map_or(list.len(), |i| i + 1))?;
                found
            } else {
                let mut found = None;
                for (i, &tri) in list.iter().enumerate() {
                    ctx.tick(1)?;
                    if tri == best {
                        found = Some(i);
                        break;
                    }
                }
                found
            };
            if let Some(i) = found {
                list[i] = list[list.len() - 1];
                state.live -= 1;
            }
        }
        for k in 0..3 {
            acc.p[k] = number::<CHECK>(acc.p[k] + cones[best as usize].p[k])?;
            acc.n[k] = number::<CHECK>(acc.n[k] + cones[best as usize].n[k])?;
        }
        emitted[best as usize] = EMITTED;
    }
    Ok(())
}
fn scan(
    out: &mut Output<'_>,
    indices: &[u32],
    n: usize,
    s: MeshletSettings,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let mut used = ctx.alloc::<i16>(n)?;
    used.fill(-1);
    for t in indices.as_chunks::<3>().0 {
        out.append(t, &mut used, s, false, ctx)?;
    }
    Ok(())
}
#[derive(Clone, Copy)]
pub(crate) enum Builder {
    Scan,
    Flex { min: usize, weight: f32, split: f32 },
    Spatial { min: usize, fill: f32 },
}
fn min_triangles(b: Builder, s: MeshletSettings) -> usize {
    match b {
        Builder::Scan => s.max_triangles,
        Builder::Flex { min, .. } | Builder::Spatial { min, .. } => min,
    }
}
fn execute(
    out: &mut Output<'_>,
    indices: &[u32],
    n: usize,
    p: Option<Positions<'_>>,
    s: MeshletSettings,
    b: Builder,
    ctx: &mut Context<'_>,
) -> Result<MeshletSizes, Error> {
    match b {
        Builder::Scan => scan(out, indices, n, s, ctx)?,
        Builder::Flex { min, weight, split } => {
            let p = p.expect("positions builder");
            if ctx.moderate {
                flex::<false>(out, indices, p, s, min, weight, split, ctx)?;
            } else {
                flex::<true>(out, indices, p, s, min, weight, split, ctx)?;
            }
        }
        Builder::Spatial { min, fill } => crate::meshlet_spatial::spatial(
            out,
            indices,
            p.expect("positions builder"),
            s,
            min,
            fill,
            ctx,
        )?,
    }
    Ok(out.finish())
}
fn check(s: MeshletSettings, b: Builder) -> Result<(), Error> {
    match b {
        Builder::Scan => validate(s, s.max_triangles, 0., 0.),
        Builder::Flex { min, weight, split } => validate(s, min, weight, split),
        Builder::Spatial { min, fill } => {
            validate(s, min, 0., 0.)?;
            if !fill.is_finite() || fill < 0. {
                return Err(Error::InvalidParameter);
            }
            Ok(())
        }
    }
}
fn build(
    indices: &[u32],
    n: usize,
    p: Option<Positions<'_>>,
    s: MeshletSettings,
    b: Builder,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error> {
    let mut ctx = Context::new(workspace)?;
    check(s, b)?;
    ctx.topology(indices, n)?;
    if let Some(p) = p {
        ctx.positions(p)?;
    }
    let bound = build_meshlets_bound(indices.len(), s.max_vertices, min_triangles(b, s))?;
    let mut result = Meshlets {
        meshlets: ctx.alloc(bound)?,
        vertices: ctx.alloc(indices.len())?,
        triangles: ctx.alloc(indices.len())?,
    };
    let sizes = execute(
        &mut Output {
            descriptors: &mut result.meshlets,
            vertices: &mut result.vertices,
            triangles: &mut result.triangles,
            current: Meshlet::default(),
            count: 0,
        },
        indices,
        n,
        p,
        s,
        b,
        &mut ctx,
    )?;
    result.meshlets.truncate(sizes.meshlets);
    result.vertices.truncate(sizes.vertices);
    result.triangles.truncate(sizes.triangles);
    Ok(result)
}
fn build_into(
    descriptors: &mut [Meshlet],
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    n: usize,
    p: Option<Positions<'_>>,
    s: MeshletSettings,
    b: Builder,
    workspace: &mut Workspace,
) -> Result<MeshletSizes, Error> {
    let mut ctx = Context::new(workspace)?;
    check(s, b)?;
    let bound = build_meshlets_bound(indices.len(), s.max_vertices, min_triangles(b, s))?;
    if descriptors.len() < bound
        || vertices.len() < indices.len()
        || triangles.len() < indices.len()
    {
        return Err(Error::BufferTooSmall);
    }
    ctx.topology(indices, n)?;
    if let Some(p) = p {
        ctx.positions(p)?;
    }
    execute(
        &mut Output {
            descriptors,
            vertices,
            triangles,
            current: Meshlet::default(),
            count: 0,
        },
        indices,
        n,
        p,
        s,
        b,
        &mut ctx,
    )
}
/// `meshopt_buildMeshletsScan`: consume triangles in input order.
pub fn build_meshlets_scan(
    indices: &[u32],
    vertex_count: usize,
    s: MeshletSettings,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error> {
    build(indices, vertex_count, None, s, Builder::Scan, workspace)
}
/// `meshopt_buildMeshlets`: connectivity and cone-aware meshlets.
pub fn build_meshlets(
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    cone_weight: f32,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error> {
    build_meshlets_flex(indices, p, s, s.max_triangles, cone_weight, 0., workspace)
}
/// `meshopt_buildMeshletsFlex`: variable triangle counts and optional spatial splits.
pub fn build_meshlets_flex(
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min_triangles: usize,
    cone_weight: f32,
    split_factor: f32,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error> {
    build(
        indices,
        p.len(),
        Some(p),
        s,
        Builder::Flex {
            min: min_triangles,
            weight: cone_weight,
            split: split_factor,
        },
        workspace,
    )
}
/// `meshopt_buildMeshletsSpatial`: surface-area heuristic meshlets for ray tracing.
pub fn build_meshlets_spatial(
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min_triangles: usize,
    fill_weight: f32,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error> {
    build(
        indices,
        p.len(),
        Some(p),
        s,
        Builder::Spatial {
            min: min_triangles,
            fill: fill_weight,
        },
        workspace,
    )
}
/// Caller-buffer scan builder; a late work-limit failure may modify output prefixes.
pub fn build_meshlets_scan_into(
    descriptors: &mut [Meshlet],
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    vertex_count: usize,
    s: MeshletSettings,
    workspace: &mut Workspace,
) -> Result<MeshletSizes, Error> {
    build_into(
        descriptors,
        vertices,
        triangles,
        indices,
        vertex_count,
        None,
        s,
        Builder::Scan,
        workspace,
    )
}
/// Caller-buffer standard builder; a late failure may modify output prefixes.
pub fn build_meshlets_into(
    descriptors: &mut [Meshlet],
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    cone_weight: f32,
    workspace: &mut Workspace,
) -> Result<MeshletSizes, Error> {
    build_meshlets_flex_into(
        descriptors,
        vertices,
        triangles,
        indices,
        p,
        s,
        s.max_triangles,
        cone_weight,
        0.,
        workspace,
    )
}
/// Caller-buffer flexible builder; a late failure may modify output prefixes.
pub fn build_meshlets_flex_into(
    descriptors: &mut [Meshlet],
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min_triangles: usize,
    cone_weight: f32,
    split_factor: f32,
    workspace: &mut Workspace,
) -> Result<MeshletSizes, Error> {
    build_into(
        descriptors,
        vertices,
        triangles,
        indices,
        p.len(),
        Some(p),
        s,
        Builder::Flex {
            min: min_triangles,
            weight: cone_weight,
            split: split_factor,
        },
        workspace,
    )
}
/// Caller-buffer spatial builder; a late failure may modify output prefixes.
pub fn build_meshlets_spatial_into(
    descriptors: &mut [Meshlet],
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min_triangles: usize,
    fill_weight: f32,
    workspace: &mut Workspace,
) -> Result<MeshletSizes, Error> {
    build_into(
        descriptors,
        vertices,
        triangles,
        indices,
        p.len(),
        Some(p),
        s,
        Builder::Spatial {
            min: min_triangles,
            fill: fill_weight,
        },
        workspace,
    )
}
