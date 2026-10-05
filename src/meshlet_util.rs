// Port of meshoptimizer 1.3 meshletutils.cpp (MIT, Arseny Kapoulkine).
use crate::math::{sqrt8, LANES};
use crate::processing::{cross, dot, finite, point, sub, Context};
use crate::{Attributes, Error, Positions, Workspace};
use alloc::vec::Vec;

/// Meaningful fields of `meshopt_Bounds`, without native padding.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bounds {
    /// Sphere center.
    pub center: [f32; 3],
    /// Sphere radius.
    pub radius: f32,
    /// Perspective normal-cone apex.
    pub cone_apex: [f32; 3],
    /// Unit normal-cone axis.
    pub cone_axis: [f32; 3],
    /// Cone culling cutoff.
    pub cone_cutoff: f32,
    /// Conservative signed-normalized axis.
    pub cone_axis_s8: [i8; 3],
    /// Conservative signed-normalized cutoff.
    pub cone_cutoff_s8: i8,
}
/// Meshlet-local topology with first-visit vertex references.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct LocalMeshlet {
    /// References to the original vertex buffer.
    pub vertices: Vec<u32>,
    /// Three local byte indices per triangle.
    pub triangles: Vec<u8>,
}

// The bounded fast path has finite intermediates by construction. Its largest
// product is a squared cross product of differences at most 2e8 (below 1e35).
// Larger finite inputs retain every original intermediate check.
#[inline(always)]
fn number<const CHECK: bool>(value: f32) -> Result<f32, Error> {
    if CHECK {
        finite(value)
    } else {
        Ok(value)
    }
}
trait SphereSource {
    fn sample(&self, index: usize) -> ([f32; 3], f32);
}
struct PointSource<'a> {
    p: Positions<'a>,
    indices: Option<&'a [u32]>,
    radii: Option<Attributes<'a>>,
}
impl SphereSource for PointSource<'_> {
    #[inline(always)]
    fn sample(&self, index: usize) -> ([f32; 3], f32) {
        let v = self.indices.map_or(index as u32, |i| i[index]);
        (
            point(self.p, v),
            self.radii
                .map_or(0., |r| r.get(v as usize, 0).expect("validated radius")),
        )
    }
}
struct NormalSource<'a>(&'a [[f32; 4]]);
impl SphereSource for NormalSource<'_> {
    #[inline(always)]
    fn sample(&self, index: usize) -> ([f32; 3], f32) {
        let p = self.0[index];
        ([p[0], p[1], p[2]], 0.)
    }
}
#[inline(always)]
fn sphere<const A: usize, const CHECK: bool>(
    count: usize,
    get: impl SphereSource,
    ctx: &mut Context<'_>,
) -> Result<([f32; 3], f32), Error> {
    if count == 0 {
        return Ok(([0.0; 3], 0.0));
    }
    let axes = [
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        [0.57735026; 3],
        [-0.57735026, 0.57735026, 0.57735026],
        [0.57735026, -0.57735026, 0.57735026],
        [0.57735026, 0.57735026, -0.57735026],
    ];
    let mut pmin = [0u32; 7];
    let mut pmax = [0u32; 7];
    let mut tmin = [f32::MAX; 7];
    let mut tmax = [-f32::MAX; 7];
    ctx.tick(count * 2)?;
    for i in 0..count {
        let (p, r) = get.sample(i);
        for k in 0..A {
            let tp = number::<CHECK>(dot(axes[k], p))?;
            let lo = number::<CHECK>(tp - r)?;
            let hi = number::<CHECK>(tp + r)?;
            let lower = lo < tmin[k];
            let upper = hi > tmax[k];
            pmin[k] = if lower { i as u32 } else { pmin[k] };
            pmax[k] = if upper { i as u32 } else { pmax[k] };
            tmin[k] = if lower { lo } else { tmin[k] };
            tmax[k] = if upper { hi } else { tmax[k] };
        }
    }
    // The axis extents are independent: take their roots together. Failures
    // are all NumericalFailure, so the grouped check order is unobservable.
    let mut squares = [1f32; LANES];
    for k in 0..A {
        let (a, _) = get.sample(pmin[k] as usize);
        let (b, _) = get.sample(pmax[k] as usize);
        let d = sub(b, a);
        squares[k] = number::<CHECK>(dot(d, d))?;
    }
    let lengths = sqrt8(squares);
    let mut axis = 0;
    let mut dr = 0.;
    for k in 0..A {
        let (_, ar) = get.sample(pmin[k] as usize);
        let (_, br) = get.sample(pmax[k] as usize);
        let v = number::<CHECK>(lengths[k] + ar + br)?;
        if v > dr {
            dr = v;
            axis = k;
        }
    }
    let (a, ar) = get.sample(pmin[axis] as usize);
    let (b, br) = get.sample(pmax[axis] as usize);
    let delta = sub(b, a);
    // The same difference as the selected axis extent: reuse its root.
    let d = lengths[axis];
    let k = if d > 0. { (d + br - ar) / (2. * d) } else { 0. };
    let mut center = [
        a[0] + delta[0] * k,
        a[1] + delta[1] * k,
        a[2] + delta[2] * k,
    ];
    let mut radius = dr / 2.;
    for i in 0..count {
        let (p, r) = get.sample(i);
        let dp = sub(p, center);
        let d2 = number::<CHECK>(dot(dp, dp))?;
        // sqrt(round(x*x)) rounds to x when x*x is normal. Lower the
        // rounded radius-r by one representable value, so adding r cannot
        // exceed radius. This only avoids a sqrt whose update would be false.
        let limit = radius - r;
        if limit >= 1.0e-18 {
            let lower = f32::from_bits(limit.to_bits() - 1);
            if d2 <= lower * lower {
                continue;
            }
        }
        let d = ctx.sqrt(d2);
        if d + r > radius {
            let k = if d > 0. {
                (d + r - radius) / (2. * d)
            } else {
                0.
            };
            for j in 0..3 {
                center[j] += k * (p[j] - center[j]);
            }
            radius = (radius + d + r) / 2.;
        }
    }
    for v in center {
        number::<CHECK>(v)?;
    }
    number::<CHECK>(radius)?;
    Ok((center, radius))
}
fn bounds<const CHECK: bool, const N: usize>(
    indices: &[u32],
    corners: &[u32],
    p: Positions<'_>,
    ctx: &mut Context<'_>,
) -> Result<Bounds, Error> {
    let mut normals = [[0f32; 4]; N];
    let mut n = 0;
    ctx.tick(indices.len() / 3)?;
    // Triangle areas are independent: take their roots eight at a time. Every
    // failure is the same NumericalFailure and work is charged above, so the
    // grouped check order is unobservable.
    for group in indices.as_chunks::<3>().0.chunks(LANES) {
        let mut corners = [[0f32; 3]; LANES];
        let mut normals_raw = [[0f32; 3]; LANES];
        let mut squares = [1f32; LANES];
        for (k, t) in group.iter().enumerate() {
            let a = point(p, t[0]);
            let normal = cross(sub(point(p, t[1]), a), sub(point(p, t[2]), a));
            squares[k] = number::<CHECK>(dot(normal, normal))?;
            corners[k] = a;
            normals_raw[k] = normal;
        }
        let areas = sqrt8(squares);
        for k in 0..group.len() {
            let area = areas[k];
            if area == 0. {
                continue;
            }
            let mut normal = normals_raw[k];
            for v in &mut normal {
                *v /= area;
            }
            normals[n] = [
                normal[0],
                normal[1],
                normal[2],
                -number::<CHECK>(dot(normal, corners[k]))?,
            ];
            n += 1;
        }
    }
    let mut b = Bounds::default();
    if n == 0 {
        return Ok(b);
    }
    let (center, radius) = sphere::<7, CHECK>(
        corners.len(),
        PointSource {
            p,
            indices: Some(corners),
            radii: None,
        },
        ctx,
    )?;
    b.center = center;
    b.radius = radius;
    let (mut axis, _) = sphere::<3, CHECK>(n, NormalSource(&normals[..n]), ctx)?;
    let len = ctx.sqrt(number::<CHECK>(dot(axis, axis))?);
    let inv = if len == 0. { 0. } else { 1. / len };
    for a in &mut axis {
        *a *= inv;
    }
    let mut mindp = 1.;
    ctx.tick(n * 2)?;
    for normal in &normals[..n] {
        let dp = dot([normal[0], normal[1], normal[2]], axis);
        if dp < mindp {
            mindp = dp;
        }
    }
    if mindp <= 0.1 {
        b.cone_cutoff = 1.;
        b.cone_cutoff_s8 = 127;
        return Ok(b);
    }
    let mut maxt = 0.;
    for normal in &normals[..n] {
        let nn = [normal[0], normal[1], normal[2]];
        let dc = dot(center, nn) + normal[3];
        let dn = dot(axis, nn);
        let t = number::<CHECK>(dc / dn)?;
        if t > maxt {
            maxt = t;
        }
    }
    for j in 0..3 {
        b.cone_apex[j] = number::<CHECK>(center[j] - axis[j] * maxt)?;
        b.cone_axis[j] = axis[j];
        let v = axis[j].clamp(-1., 1.);
        b.cone_axis_s8[j] = (v * 127. + if v >= 0. { 0.5 } else { -0.5 }) as i8;
    }
    b.cone_cutoff = ctx.sqrt(number::<CHECK>(1. - mindp * mindp)?);
    let e0 = (b.cone_axis_s8[0] as f32 / 127. - axis[0]).abs();
    let e1 = (b.cone_axis_s8[1] as f32 / 127. - axis[1]).abs();
    let e2 = (b.cone_axis_s8[2] as f32 / 127. - axis[2]).abs();
    b.cone_cutoff_s8 = ((127. * (b.cone_cutoff + e0 + e1 + e2) + 1.) as i32).min(127) as i8;
    Ok(b)
}
/// `meshopt_computeClusterBounds`: sphere and conservative normal cone.
pub fn compute_cluster_bounds(
    indices: &[u32],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Bounds, Error> {
    let mut ctx = Context::new(workspace)?;
    if indices.len() > 1536 {
        return Err(Error::InvalidParameter);
    }
    if indices.len() <= 96 {
        cluster_bounds::<96, 32>(indices, p, &mut ctx, None)
    } else if indices.len() <= 384 {
        cluster_bounds::<384, 128>(indices, p, &mut ctx, None)
    } else {
        cluster_bounds::<1536, 512>(indices, p, &mut ctx, None)
    }
}
#[cfg(feature = "clusterlod")]
pub(crate) fn compute_cluster_bounds_validated(
    indices: &[u32],
    p: Positions<'_>,
    moderate: bool,
    workspace: &mut Workspace,
) -> Result<Bounds, Error> {
    let mut ctx = Context::new(workspace)?;
    if indices.len() > 1536 {
        return Err(Error::InvalidParameter);
    }
    if indices.len() <= 96 {
        cluster_bounds::<96, 32>(indices, p, &mut ctx, Some(moderate))
    } else if indices.len() <= 384 {
        cluster_bounds::<384, 128>(indices, p, &mut ctx, Some(moderate))
    } else {
        cluster_bounds::<1536, 512>(indices, p, &mut ctx, Some(moderate))
    }
}
fn cluster_bounds<const M: usize, const N: usize>(
    indices: &[u32],
    p: Positions<'_>,
    ctx: &mut Context<'_>,
    trusted_moderate: Option<bool>,
) -> Result<Bounds, Error> {
    ctx.topology(indices, p.len())?;
    if let Some(moderate) = trusted_moderate {
        ctx.moderate = moderate;
    } else {
        ctx.positions(p)?;
    }
    let mut cache = [u32::MAX; 512];
    let mut corners = [0u32; M];
    let mut n = 0;
    ctx.tick(indices.len())?;
    for &v in indices {
        let c = &mut cache[v as usize & 511];
        if *c != v {
            corners[n] = v;
            n += 1;
        }
        *c = v;
    }
    if ctx.moderate {
        bounds::<false, N>(indices, &corners[..n], p, ctx)
    } else {
        bounds::<true, N>(indices, &corners[..n], p, ctx)
    }
}
/// `meshopt_computeMeshletBounds`: bounds of byte-local triangles.
pub fn compute_meshlet_bounds(
    vertices: &[u32],
    triangles: &[u8],
    p: Positions<'_>,
    workspace: &mut Workspace,
) -> Result<Bounds, Error> {
    let mut ctx = Context::new(workspace)?;
    validate_local(vertices, triangles)?;
    if triangles.len() <= 96 {
        meshlet_bounds::<96, 32>(vertices, triangles, p, &mut ctx)
    } else if triangles.len() <= 384 {
        meshlet_bounds::<384, 128>(vertices, triangles, p, &mut ctx)
    } else {
        meshlet_bounds::<1536, 512>(vertices, triangles, p, &mut ctx)
    }
}
fn meshlet_bounds<const M: usize, const N: usize>(
    vertices: &[u32],
    triangles: &[u8],
    p: Positions<'_>,
    ctx: &mut Context<'_>,
) -> Result<Bounds, Error> {
    ctx.positions(p)?;
    ctx.tick(vertices.len() + triangles.len())?;
    if vertices.iter().any(|&v| v as usize >= p.len()) {
        return Err(Error::IndexOutOfBounds);
    }
    let mut indices = [0u32; M];
    let mut n = 0;
    for (i, &v) in triangles.iter().enumerate() {
        indices[i] = vertices[v as usize];
        n = n.max(v as usize + 1);
    }
    if ctx.moderate {
        bounds::<false, N>(&indices[..triangles.len()], &vertices[..n], p, ctx)
    } else {
        bounds::<true, N>(&indices[..triangles.len()], &vertices[..n], p, ctx)
    }
}
/// `meshopt_computeSphereBounds`. Optional radii use a checked one-component view.
pub fn compute_sphere_bounds(
    p: Positions<'_>,
    radii: Option<Attributes<'_>>,
    workspace: &mut Workspace,
) -> Result<Bounds, Error> {
    let mut ctx = Context::new(workspace)?;
    ctx.positions(p)?;
    if let Some(r) = radii {
        if r.len() != p.len() || r.components() != 1 {
            return Err(Error::InvalidLayout);
        }
        ctx.tick(r.len())?;
        for i in 0..r.len() {
            let v = r.get(i, 0).ok_or(Error::InvalidLayout)?;
            ctx.moderate &= v <= 1.0e8;
            if !v.is_finite() || v < 0. {
                return Err(Error::InvalidParameter);
            }
        }
    }
    let get = PointSource {
        p,
        indices: None,
        radii,
    };
    let (center, radius) = if ctx.moderate {
        sphere::<7, false>(p.len(), get, &mut ctx)?
    } else {
        sphere::<7, true>(p.len(), get, &mut ctx)?
    };
    Ok(Bounds {
        center,
        radius,
        ..Bounds::default()
    })
}
fn validate_local(v: &[u32], t: &[u8]) -> Result<(), Error> {
    if !t.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    if v.len() > 256 || t.len() > 1536 {
        return Err(Error::InvalidParameter);
    }
    if t.iter().any(|&i| i as usize >= v.len()) {
        return Err(Error::IndexOutOfBounds);
    }
    Ok(())
}
fn extract<const V: usize, const T: usize>(
    indices: &[u32],
    vertices: &mut [u32; V],
    triangles: &mut [u8; T],
    ctx: &mut Context<'_>,
) -> Result<usize, Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    if indices.len() > 1536 {
        return Err(Error::InvalidParameter);
    }
    let mut n = 0;
    let mut cache = [-1i16; 1024];
    ctx.tick(indices.len())?;
    for (i, &v) in indices.iter().enumerate() {
        let key = v as usize & 1023;
        let c = cache[key];
        if c >= 0 && vertices[c as u8 as usize] == v {
            triangles[i] = c as u8;
            continue;
        }
        let pos = if c < 0 {
            n
        } else {
            ctx.tick(n)?;
            vertices[..n].iter().position(|&x| x == v).unwrap_or(n)
        };
        if pos == n {
            if n == 256 {
                return Err(Error::InvalidParameter);
            }
            vertices[n] = v;
            n += 1;
        }
        cache[key] = pos as i16;
        triangles[i] = pos as u8;
    }
    Ok(n)
}
/// `meshopt_extractMeshletIndices`: preserve first-visit vertex order.
pub fn extract_meshlet_indices(
    indices: &[u32],
    workspace: &mut Workspace,
) -> Result<LocalMeshlet, Error> {
    let mut ctx = Context::new(workspace)?;
    extracted(indices, &mut ctx, |v, t, ctx| {
        Ok(LocalMeshlet {
            vertices: ctx.copy(v)?,
            triangles: ctx.copy(t)?,
        })
    })
}
// Run `extract` with stack scratch sized to the input (no larger than the
// 256-vertex and 1536-index limits), so small meshlets do not clear the
// full-size arrays. Validation order and results are unchanged.
#[inline(always)]
fn extracted<R>(
    indices: &[u32],
    ctx: &mut Context<'_>,
    finish: impl FnOnce(&[u32], &[u8], &mut Context<'_>) -> Result<R, Error>,
) -> Result<R, Error> {
    if indices.len() <= 96 {
        let (mut v, mut t) = ([0; 96], [0; 96]);
        let n = extract(indices, &mut v, &mut t, ctx)?;
        finish(&v[..n], &t[..indices.len()], ctx)
    } else if indices.len() <= 384 {
        let (mut v, mut t) = ([0; 256], [0; 384]);
        let n = extract(indices, &mut v, &mut t, ctx)?;
        finish(&v[..n], &t[..indices.len()], ctx)
    } else {
        let (mut v, mut t) = ([0; 256], [0; 1536]);
        let n = extract(indices, &mut v, &mut t, ctx)?;
        finish(&v[..n], &t[..indices.len()], ctx)
    }
}
/// Caller-buffer extraction. Errors leave both destinations unchanged; tails are preserved.
pub fn extract_meshlet_indices_into(
    vertices: &mut [u32],
    triangles: &mut [u8],
    indices: &[u32],
    workspace: &mut Workspace,
) -> Result<usize, Error> {
    let mut ctx = Context::new(workspace)?;
    if triangles.len() < indices.len() {
        return Err(Error::BufferTooSmall);
    }
    extracted(indices, &mut ctx, |v, t, _| {
        if vertices.len() < v.len() {
            return Err(Error::BufferTooSmall);
        }
        vertices[..v.len()].copy_from_slice(v);
        triangles[..t.len()].copy_from_slice(t);
        Ok(v.len())
    })
}
// All entry points validate sizes and local references before copying/mutation.
fn optimize(v: &mut [u32], t: &mut [u8], level: u8, ctx: &mut Context<'_>) -> Result<(), Error> {
    if level > 9 {
        return Err(Error::InvalidParameter);
    }
    if level == 0 {
        optimize_body::<false>(v, t, level, ctx)
    } else {
        optimize_body::<true>(v, t, level, ctx)
    }
}
fn optimize_body<const ADVANCED: bool>(
    v: &mut [u32],
    t: &mut [u8],
    level: u8,
    ctx: &mut Context<'_>,
) -> Result<(), Error> {
    let n = t.len() / 3;
    let mut cache = [0u8; 256];
    let mut last = 128u8;
    let mut valence = [0u8; 256];
    ctx.tick(t.len())?;
    for &i in t.iter() {
        valence[i as usize] = valence[i as usize].wrapping_add(1);
    }
    for i in 0..n {
        let mut next = i;
        let mut best = -1;
        let mut edges = 0;
        ctx.search(
            t.as_chunks::<3>().0[i..].iter().enumerate(),
            |(offset, tri)| {
                let (a, b, c) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                let (ad, bd, cd) = (
                    last.wrapping_sub(cache[a]),
                    last.wrapping_sub(cache[b]),
                    last.wrapping_sub(cache[c]),
                );
                let matches = i32::from(ad < 3) + i32::from(bd < 3) + i32::from(cd < 3);
                let score = if ADVANCED {
                    (matches * 1024 + (1023 - ad as i32 - bd as i32 - cd as i32)) * 256
                        + (255 - valence[a].min(valence[b]).min(valence[c]) as i32)
                } else {
                    matches
                };
                if score > best {
                    best = score;
                    next = i + offset;
                }
                if matches >= 2 {
                    edges += 1;
                    if !ADVANCED || edges >= level {
                        return true;
                    }
                }
                false
            },
        )?;
        ctx.tick(next - i + 1)?;
        let tri = [t[next * 3], t[next * 3 + 1], t[next * 3 + 2]];
        t.copy_within(i * 3..next * 3, (i + 1) * 3);
        t[i * 3..i * 3 + 3].copy_from_slice(&tri);
        last = last.wrapping_add(1);
        for x in tri {
            cache[x as usize] = last;
            valence[x as usize] = valence[x as usize].wrapping_sub(1);
        }
    }
    if ADVANCED {
        cache.fill(0);
        for i in 0..n {
            ctx.tick(1)?;
            let mut tri = [t[i * 3], t[i * 3 + 1], t[i * 3 + 2]];
            let [a, b, c] = tri;
            if cache[a as usize] == 0 && cache[b as usize] != 0 && cache[c as usize] == 0 {
                tri.rotate_left(1);
            } else if cache[a as usize] == 0 && cache[b as usize] == 0 && cache[c as usize] == 0 {
                let (mut ab, mut bc, mut ca) = (false, false, false);
                for j in i + 1..n.min(i + 4) {
                    ctx.tick(1)?;
                    let [oa, ob, oc] = [t[j * 3], t[j * 3 + 1], t[j * 3 + 2]];
                    ab |= (oa == b && ob == a) || (ob == b && oc == a) || (oc == b && oa == a);
                    bc |= (oa == c && ob == b) || (ob == c && oc == b) || (oc == c && oa == b);
                    ca |= (oa == a && ob == c) || (ob == a && oc == c) || (oc == a && oa == c);
                }
                if ab && !bc {
                    tri.rotate_left(1);
                } else if (ab && !ca) || (!ab && bc && !ca) {
                    tri.rotate_right(1);
                }
            }
            t[i * 3..i * 3 + 3].copy_from_slice(&tri);
            for x in tri {
                cache[x as usize] = 1;
            }
        }
    }
    let mut remap = [-1i16; 256];
    let mut order = [0u32; 256];
    let mut count = 0;
    ctx.tick(t.len())?;
    for x in t {
        let r = &mut remap[*x as usize];
        if *r < 0 {
            *r = count as i16;
            order[count] = v[*x as usize];
            count += 1;
        }
        *x = *r as u8;
    }
    v[..count].copy_from_slice(&order[..count]);
    Ok(())
}
/// `meshopt_optimizeMeshletLevel`: atomic destructive variant, levels 0 through 9.
pub fn optimize_meshlet_level_in_place(
    vertices: &mut [u32],
    triangles: &mut [u8],
    level: u8,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    validate_local(vertices, triangles)?;
    let mut v = [0; 256];
    let mut t = [0; 1536];
    v[..vertices.len()].copy_from_slice(vertices);
    t[..triangles.len()].copy_from_slice(triangles);
    optimize(
        &mut v[..vertices.len()],
        &mut t[..triangles.len()],
        level,
        &mut ctx,
    )?;
    vertices.copy_from_slice(&v[..vertices.len()]);
    triangles.copy_from_slice(&t[..triangles.len()]);
    Ok(())
}
/// Atomic destructive `meshopt_optimizeMeshlet` (level zero).
pub fn optimize_meshlet_in_place(
    vertices: &mut [u32],
    triangles: &mut [u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    optimize_meshlet_level_in_place(vertices, triangles, 0, workspace)
}
/// Allocating `meshopt_optimizeMeshletLevel`; unused vertex suffix stays unchanged.
pub fn optimize_meshlet_level(
    vertices: &[u32],
    triangles: &[u8],
    level: u8,
    workspace: &mut Workspace,
) -> Result<LocalMeshlet, Error> {
    let mut ctx = Context::new(workspace)?;
    validate_local(vertices, triangles)?;
    let mut v = ctx.copy(vertices)?;
    let mut t = ctx.copy(triangles)?;
    optimize(&mut v, &mut t, level, &mut ctx)?;
    Ok(LocalMeshlet {
        vertices: v,
        triangles: t,
    })
}
/// Allocating `meshopt_optimizeMeshlet` (level zero).
pub fn optimize_meshlet(
    vertices: &[u32],
    triangles: &[u8],
    workspace: &mut Workspace,
) -> Result<LocalMeshlet, Error> {
    optimize_meshlet_level(vertices, triangles, 0, workspace)
}

/// Caller-buffer meshlet optimization. Errors leave destinations unchanged;
/// suffixes beyond the supplied vertex and triangle lengths are preserved.
pub fn optimize_meshlet_level_into(
    out_vertices: &mut [u32],
    out_triangles: &mut [u8],
    vertices: &[u32],
    triangles: &[u8],
    level: u8,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let mut ctx = Context::new(workspace)?;
    if out_vertices.len() < vertices.len() || out_triangles.len() < triangles.len() {
        return Err(Error::BufferTooSmall);
    }
    validate_local(vertices, triangles)?;
    let mut v = [0; 256];
    let mut t = [0; 1536];
    v[..vertices.len()].copy_from_slice(vertices);
    t[..triangles.len()].copy_from_slice(triangles);
    optimize(
        &mut v[..vertices.len()],
        &mut t[..triangles.len()],
        level,
        &mut ctx,
    )?;
    out_vertices[..vertices.len()].copy_from_slice(&v[..vertices.len()]);
    out_triangles[..triangles.len()].copy_from_slice(&t[..triangles.len()]);
    Ok(())
}
/// Caller-buffer level-zero meshlet optimization, preserving unused tails.
pub fn optimize_meshlet_into(
    out_vertices: &mut [u32],
    out_triangles: &mut [u8],
    vertices: &[u32],
    triangles: &[u8],
    workspace: &mut Workspace,
) -> Result<(), Error> {
    optimize_meshlet_level_into(
        out_vertices,
        out_triangles,
        vertices,
        triangles,
        0,
        workspace,
    )
}
