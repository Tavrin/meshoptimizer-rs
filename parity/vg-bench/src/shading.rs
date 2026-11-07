//! Independent area-probe evaluator for decoded one-pixel cuts.
use crate::{camera, report::Json, source::Source};
use meshoptimizer_rs::{
    clusterlod::{
        cut::{self, CutView, Projection, Threshold},
        dag_format::v2::Reader,
        FlatDagCluster, FlatDagGroup,
    },
    Limits, Workspace,
};
use rayon::prelude::*;
use std::{collections::BTreeMap, fs, path::Path};

type V = [f64; 3];
type R<T> = Result<T, String>;
fn sub(a: V, b: V) -> V {
    core::array::from_fn(|k| a[k] - b[k])
}
fn dot(a: V, b: V) -> f64 {
    a.iter().zip(b).map(|(a, b)| a * b).sum()
}
fn cross(a: V, b: V) -> V {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: V) -> Option<V> {
    let l = dot(a, a).sqrt();
    (l > 0.0 && l.is_finite()).then(|| a.map(|x| x / l))
}
fn blend(p: [V; 3], w: V) -> V {
    core::array::from_fn(|k| p[0][k] * w[0] + p[1][k] * w[1] + p[2][k] * w[2])
}
fn angle(a: Option<V>, b: Option<V>) -> f64 {
    match (a, b) {
        (Some(a), Some(b)) => dot(cross(a, b), cross(a, b)).sqrt().atan2(dot(a, b).clamp(-1., 1.)).to_degrees(),
        _ => 180.0,
    }
}
#[derive(Clone, Copy)]
struct Probe {
    p: V,
    normal: Option<V>,
    uv: [f64; 2],
    tangent: Option<V>,
    sign: f64,
}
#[derive(Clone, Copy)]
struct Mesh<'a> {
    p: &'a [[f32; 3]],
    n: &'a [[f32; 3]],
    uv: &'a [[f32; 2]],
    t: &'a [[f32; 4]],
    ix: &'a [u32],
}
impl Mesh<'_> {
    fn triangle(&self, id: usize) -> [V; 3] {
        core::array::from_fn(|k| self.p[self.ix[id * 3 + k] as usize].map(f64::from))
    }
    fn at(&self, id: usize, w: V) -> Probe {
        let ids = [self.ix[id * 3] as usize, self.ix[id * 3 + 1] as usize, self.ix[id * 3 + 2] as usize];
        let normals = ids.map(|i| unit(self.n[i].map(f64::from)).unwrap_or([0.; 3]));
        let normal = unit(blend(normals, w));
        let uv = core::array::from_fn(|k| ids.iter().zip(w).map(|(&i, w)| f64::from(self.uv[i][k]) * w).sum());
        let mut tangent = None;
        let mut sign = 1.0;
        if !self.t.is_empty() {
            let raw: V = core::array::from_fn(|k| ids.iter().zip(w).map(|(&i, w)| f64::from(self.t[i][k]) * w).sum());
            if let Some(n) = normal {
                tangent = unit(sub(raw, n.map(|x| x * dot(raw, n))));
            }
            let value: f64 = ids.iter().zip(w).map(|(&i, w)| f64::from(self.t[i][3]) * w).sum();
            sign = if value < 0.0 { -1.0 } else { 1.0 };
        }
        Probe { p: blend(self.triangle(id), w), normal, uv, tangent, sign }
    }
    fn samples(&self, count: usize) -> R<Vec<Probe>> {
        let mut cdf = Vec::with_capacity(self.ix.len() / 3);
        let mut total = 0.0;
        for id in 0..self.ix.len() / 3 {
            let p = self.triangle(id);
            let n = cross(sub(p[1], p[0]), sub(p[2], p[0]));
            total += 0.5 * dot(n, n).sqrt();
            cdf.push(total);
        }
        if total <= 0.0 || !total.is_finite() {
            return Err("empty/nonfinite sampled surface".into());
        }
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let location = (i as f64 + 0.5) / count as f64 * total;
            let id = cdf.partition_point(|&x| x <= location).min(cdf.len() - 1);
            let r = ((f64::from((i as u32).reverse_bits()) + 0.5) / 4294967296.0).sqrt();
            let mut ordinal = i + 1;
            let mut factor = 1.0 / 3.0;
            let mut t = 0.0;
            while ordinal > 0 {
                t += (ordinal % 3) as f64 * factor;
                factor /= 3.0;
                ordinal /= 3;
            }
            result.push(self.at(id, [1.0 - r, r * (1.0 - t), r * t]));
        }
        Ok(result)
    }
}
#[derive(Clone, Copy)]
struct Node {
    lo: V,
    hi: V,
    start: usize,
    end: usize,
    left: usize,
    right: usize,
}
#[derive(Clone, Copy)]
struct Hit {
    d: f64,
    id: usize,
    w: V,
}
struct Tree<'a> {
    mesh: Mesh<'a>,
    order: Vec<usize>,
    nodes: Vec<Node>,
}
impl<'a> Tree<'a> {
    fn new(mesh: Mesh<'a>) -> R<Self> {
        if mesh.ix.is_empty() || !mesh.ix.len().is_multiple_of(3) {
            return Err("empty/invalid tree".into());
        }
        let mut result = Self { mesh, order: (0..mesh.ix.len() / 3).collect(), nodes: Vec::new() };
        result.build(0, result.order.len());
        Ok(result)
    }
    fn build(&mut self, start: usize, end: usize) -> usize {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for &id in &self.order[start..end] {
            for p in self.mesh.triangle(id) {
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        let index = self.nodes.len();
        self.nodes.push(Node { lo, hi, start, end, left: usize::MAX, right: usize::MAX });
        if end - start > 8 {
            let axis = (0..3).max_by(|&a, &b| (hi[a] - lo[a]).total_cmp(&(hi[b] - lo[b]))).unwrap_or(0);
            let mid = start + (end - start) / 2;
            let mesh = self.mesh;
            let key = |id: usize| mesh.triangle(id).iter().map(|p| p[axis]).sum::<f64>();
            self.order[start..end]
                .select_nth_unstable_by(mid - start, |&a, &b| key(a).total_cmp(&key(b)).then(a.cmp(&b)));
            let left = self.build(start, mid);
            let right = self.build(mid, end);
            self.nodes[index].left = left;
            self.nodes[index].right = right;
        }
        index
    }
    fn box_distance(&self, id: usize, p: V) -> f64 {
        let n = self.nodes[id];
        (0..3).map(|k| (n.lo[k] - p[k]).max(0.0).max(p[k] - n.hi[k]).powi(2)).sum()
    }
    fn query(&self, id: usize, p: V, best: &mut Hit) {
        if self.box_distance(id, p) > best.d {
            return;
        }
        let node = self.nodes[id];
        if node.left == usize::MAX {
            for &id in &self.order[node.start..node.end] {
                let (d, w) = closest(self.mesh.triangle(id), p);
                if d < best.d || (d == best.d && id < best.id) {
                    *best = Hit { d, w, id };
                }
            }
        } else {
            let (a, b) = if self.box_distance(node.left, p) <= self.box_distance(node.right, p) {
                (node.left, node.right)
            } else {
                (node.right, node.left)
            };
            self.query(a, p, best);
            self.query(b, p, best);
        }
    }
    fn nearest(&self, p: V) -> Hit {
        let mut hit = Hit { d: f64::INFINITY, id: usize::MAX, w: [1., 0., 0.] };
        self.query(0, p, &mut hit);
        assert!(hit.id != usize::MAX && hit.d.is_finite());
        hit
    }
}
fn closest(t: [V; 3], p: V) -> (f64, V) {
    let mut best = (f64::INFINITY, [1., 0., 0.]);
    for (a, b) in [(0, 1), (1, 2), (2, 0)] {
        let edge = sub(t[b], t[a]);
        let len = dot(edge, edge);
        let s = if len > 0.0 { (dot(sub(p, t[a]), edge) / len).clamp(0., 1.) } else { 0.0 };
        let mut w = [0.; 3];
        w[a] = 1. - s;
        w[b] = s;
        let d = if s == 0.0 {
            let x = sub(p, t[a]);
            dot(x, x)
        } else if s == 1.0 {
            let x = sub(p, t[b]);
            dot(x, x)
        } else {
            let x = sub(p, if s <= 0.5 { t[a] } else { t[b] });
            let v = cross(x, edge);
            dot(v, v) / len
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
                best = (d, [1. - v - w, v, w]);
            }
        }
    }
    best
}
struct Decoded {
    groups: Vec<FlatDagGroup>,
    clusters: Vec<FlatDagCluster>,
    p: Vec<[f32; 3]>,
    n: Vec<[f32; 3]>,
    uv: Vec<[f32; 2]>,
    t: Vec<[f32; 4]>,
    ix: Vec<u32>,
    q: f64,
}
impl Decoded {
    fn read(path: &Path) -> R<Self> {
        let file = fs::read(path).map_err(|e| e.to_string())?;
        let mut ws = Workspace::new(Limits { max_bytes: usize::MAX, max_work: u64::MAX });
        let r = Reader::read_all(&file, &mut ws).map_err(|e| format!("{e:?}"))?;
        let m = r.mesh();
        let mut old = vec![FlatDagCluster::default(); m.cluster_count as usize];
        let mut p = Vec::new();
        let mut n = Vec::new();
        let mut uv = Vec::new();
        let mut t = Vec::new();
        let mut ix = Vec::new();
        let with_tangent = (0..r.attribute_count()).any(|i| r.attribute_channel(i).is_ok_and(|c| c.tag == 3));
        for page in 0..m.page_count as usize {
            let rec = r.page(page).map_err(|e| format!("{e:?}"))?;
            let mut words = vec![0; rec.memory_words as usize];
            let view = r
                .transcode_page(
                    page,
                    &file[rec.disk_offset as usize..rec.disk_offset as usize + rec.disk_length as usize],
                    &mut words,
                    &mut ws,
                )
                .map_err(|e| format!("{e:?}"))?;
            for c in 0..view.cluster_count() {
                let desc = view.cluster(c).map_err(|e| format!("{e:?}"))?;
                let global = rec.first_cluster as usize + c;
                let base = u32::try_from(p.len()).map_err(|e| e.to_string())?;
                let mut positions = [[0f32; 3]; 256];
                let nv = view.decode_positions(c, &mut positions).map_err(|e| format!("{e:?}"))?;
                p.extend_from_slice(&positions[..nv]);
                let attrs = view.attributes(c).map_err(|e| format!("{e:?}"))?.ok_or("missing attributes")?;
                let channel = |tag| {
                    (0..attrs.channel_count())
                        .find(|&k| attrs.channel(k).is_ok_and(|c| c.tag == tag))
                        .ok_or("missing channel")
                };
                let nc = channel(1)?;
                let uc = channel(2)?;
                let tc = if with_tangent { Some(channel(3)?) } else { None };
                for v in 0..nv {
                    let normal = attrs.decode(nc, v).map_err(|e| format!("{e:?}"))?;
                    n.push([normal[0], normal[1], normal[2]]);
                    let value = attrs.decode(uc, v).map_err(|e| format!("{e:?}"))?;
                    uv.push([value[0], value[1]]);
                    if let Some(k) = tc {
                        t.push(attrs.decode(k, v).map_err(|e| format!("{e:?}"))?);
                    }
                }
                let start = u32::try_from(ix.len()).map_err(|e| e.to_string())?;
                let mut triangles = [[0u8; 3]; 256];
                let nt = view.decode_triangles(c, &mut triangles).map_err(|e| format!("{e:?}"))?;
                for tri in &triangles[..nt] {
                    ix.extend(tri.map(|v| base + u32::from(v)));
                }
                let (center, radius) = r.cluster_sphere(global).map_err(|e| format!("{e:?}"))?;
                let record = &mut old[global];
                record.first_index = start;
                record.index_count = (nt * 3) as u32;
                record.refined = r.cluster_refined(global).map_err(|e| format!("{e:?}"))?;
                record.center = center;
                record.radius = radius;
                assert_eq!(nt, desc.triangle_count as usize);
            }
        }
        // Resident groups are coarse-first; the public CPU cut tables are fine-first.
        let mut groups = Vec::new();
        let mut clusters = Vec::new();
        for old_id in (0..m.group_count as usize).rev() {
            let g = r.group(old_id).map_err(|e| format!("{e:?}"))?;
            let gid = groups.len() as u32;
            let first = clusters.len() as u32;
            for cid in g.first_cluster..g.first_cluster + g.cluster_count {
                let mut c = old[cid as usize];
                c.group = gid;
                if c.refined >= 0 {
                    c.refined = m.group_count as i32 - 1 - c.refined;
                }
                clusters.push(c);
            }
            let mut group = FlatDagGroup::default();
            group.center = g.center;
            group.radius = g.radius;
            group.error = g.error;
            group.depth = g.depth as i32;
            group.first_cluster = first;
            group.cluster_count = g.cluster_count;
            groups.push(group);
        }
        cut::check_monotonic(&groups, &clusters).map_err(|e| format!("decoded cut monotonicity: {e:?}"))?;
        Ok(Self { groups, clusters, p, n, uv, t, ix, q: f64::from(m.max_displacement) })
    }
    fn mesh<'a>(&'a self, ix: &'a [u32]) -> Mesh<'a> {
        Mesh { p: &self.p, n: &self.n, uv: &self.uv, t: &self.t, ix }
    }
    fn select(&self, view: &CutView, threshold: Threshold) -> R<Vec<u32>> {
        let mut out = vec![0; self.clusters.len()];
        let n = cut::select_cut(&self.groups, &self.clusters, view, threshold, None, &mut out)
            .map_err(|e| format!("{e:?}"))?;
        out.truncate(n);
        out.sort_unstable();
        Ok(out)
    }
    fn indices(&self, ids: &[u32]) -> Vec<u32> {
        ids.iter()
            .flat_map(|&id| {
                let c = &self.clusters[id as usize];
                self.ix[c.first_index as usize..(c.first_index + c.index_count) as usize].iter().copied()
            })
            .collect()
    }
    fn claim(&self, ids: &[u32]) -> f64 {
        ids.iter()
            .map(|&id| {
                let r = self.clusters[id as usize].refined;
                if r < 0 {
                    self.q
                } else {
                    f64::from(self.groups[r as usize].error)
                }
            })
            .fold(self.q, f64::max)
    }
}
struct Measured {
    frequency: usize,
    triangles: usize,
    normal: Vec<f32>,
    uv: Vec<f32>,
    tangent: Vec<f32>,
    normal_max: f64,
    tangent_max: f64,
    geometry: f64,
    ratio: f64,
    sign_mismatches: u64,
}
fn measure_cut(
    dag: &Decoded,
    ids: &[u32],
    frequency: usize,
    source_tree: &Tree<'_>,
    source_samples: &[Probe],
    samples: usize,
) -> R<Measured> {
    let indices = dag.indices(ids);
    let mesh = dag.mesh(&indices);
    let tree = Tree::new(mesh)?;
    let reverse = mesh.samples(samples)?;
    let mut row = Measured {
        frequency,
        triangles: indices.len() / 3,
        normal: Vec::with_capacity(samples * 2),
        uv: Vec::with_capacity(samples * 2),
        tangent: Vec::new(),
        normal_max: 0.,
        tangent_max: 0.,
        geometry: 0.,
        ratio: 0.,
        sign_mismatches: 0,
    };
    let tangents = !mesh.t.is_empty() && !source_tree.mesh.t.is_empty();
    for (points, target) in [(source_samples, &tree), (reverse.as_slice(), source_tree)] {
        for point in points {
            let hit = target.nearest(point.p);
            let other = target.mesh.at(hit.id, hit.w);
            let normal = angle(point.normal, other.normal);
            row.normal.push(normal as f32);
            row.normal_max = row.normal_max.max(normal);
            let uv = ((point.uv[0] - other.uv[0]).powi(2) + (point.uv[1] - other.uv[1]).powi(2)).sqrt();
            if !uv.is_finite() || !normal.is_finite() {
                return Err("nonfinite probe measurement".into());
            }
            row.uv.push(uv as f32);
            row.geometry = row.geometry.max(hit.d.sqrt());
            if tangents {
                let value = angle(point.tangent, other.tangent);
                row.tangent.push(value as f32);
                row.tangent_max = row.tangent_max.max(value);
                row.sign_mismatches += u64::from(point.sign != other.sign);
            }
        }
    }
    row.ratio = row.geometry / dag.claim(ids);
    if !row.ratio.is_finite() {
        return Err("nonfinite geometric ratio".into());
    }
    row.normal.sort_unstable_by(f32::total_cmp);
    row.uv.sort_unstable_by(f32::total_cmp);
    row.tangent.sort_unstable_by(f32::total_cmp);
    Ok(row)
}
fn quantile<'a>(values: impl Iterator<Item = (&'a [f32], usize)> + Clone, q: f64) -> f64 {
    let count: usize = values.clone().map(|(v, f)| v.len() * f).sum();
    if count == 0 {
        return 0.0;
    }
    let rank = ((count as f64 * q).ceil() as usize).max(1);
    let mut lo = values.clone().filter_map(|(v, _)| v.first()).map(|x| x.to_bits()).min().unwrap_or(0);
    let mut hi = values.clone().filter_map(|(v, _)| v.last()).map(|x| x.to_bits()).max().unwrap_or(0);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let n: usize = values.clone().map(|(v, f)| v.partition_point(|x| x.to_bits() <= mid) * f).sum();
        if n >= rank {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    f64::from(f32::from_bits(lo))
}
pub fn evaluate(args: &[String]) -> R<()> {
    let positional = crate::positional(args);
    let path = positional.first().ok_or("shade-eval source.vgm artifact...")?;
    if positional.len() < 2 {
        return Err("missing shade artifacts".into());
    }
    let samples = crate::num(args, "--samples", 65536usize);
    if samples == 0 {
        return Err("zero probes".into());
    }
    let dirs = crate::num(args, "--dirs", 64usize);
    let jobs = crate::num(args, "--jobs", 8usize);
    if dirs == 0 || jobs == 0 {
        return Err("zero directions/jobs".into());
    }
    let mut source = Source::load(Path::new(path.as_str()))?;
    let tangent_stats = match crate::flag(args, "--with-tangents").as_deref() {
        Some("1") => source.derive_tangents()?,
        None | Some("0") => (0, 0),
        _ => return Err("with-tangents needs 0 or 1".into()),
    };
    let src =
        Mesh { p: &source.positions, n: &source.normals, uv: &source.uvs, t: &source.tangents, ix: &source.indices };
    let source_tree = Tree::new(src)?;
    let probes = src.samples(samples)?;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(jobs).build().map_err(|e| e.to_string())?;
    let directions = camera::directions(dirs);
    let spans = [64., 128., 256., 512., 1024., 2048.];
    for artifact in &positional[1..] {
        let dag = Decoded::read(Path::new(artifact.as_str()))?;
        if !dag.t.is_empty() && source.tangents.is_empty() {
            return Err("artifact tangents require source tangents".into());
        }
        let mut unique: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
        for &span in &spans {
            for &direction in &directions {
                let eye =
                    camera::View::orbit(direction, camera::distance_for_span(f64::from(source.radius()), span)).eye;
                let view = CutView {
                    position: eye.map(|x| x as f32),
                    projection: Projection::Perspective { proj_y: camera::proj11() as f32, znear: camera::NEAR as f32 },
                    viewport_height: camera::VIEWPORT_H as f32,
                    scale: 1.0,
                };
                let ids = dag.select(&view, Threshold::Pixels(1.0))?;
                if unique.is_empty() {
                    let finest = dag.select(&view, Threshold::Finest)?;
                    if dag.indices(&finest).len() != source.indices.len() {
                        return Err("finest triangle identity".into());
                    }
                    let mut oracle = vec![0; dag.clusters.len()];
                    let n = cut::refine_sequential(
                        &dag.groups,
                        &dag.clusters,
                        &view,
                        Threshold::Pixels(1.0),
                        None,
                        &mut oracle,
                    )
                    .map_err(|e| format!("cut oracle: {e:?}"))?;
                    oracle.truncate(n);
                    oracle.sort_unstable();
                    if oracle != ids {
                        return Err("cut oracle mismatch".into());
                    }
                }
                *unique.entry(ids).or_default() += 1;
            }
        }
        let cases: Vec<_> = unique.into_iter().collect();
        let measured: Vec<Measured> = pool.install(|| {
            cases
                .par_iter()
                .map(|(ids, f)| measure_cut(&dag, ids, *f, &source_tree, &probes, samples))
                .collect::<R<Vec<_>>>()
        })?;
        let views = dirs * spans.len();
        let avg = measured.iter().map(|r| r.triangles * r.frequency).sum::<usize>() as f64 / views as f64;
        let ratios: Vec<f64> = measured.iter().flat_map(|r| std::iter::repeat_n(r.ratio, r.frequency)).collect();
        let mut ratios = ratios;
        ratios.sort_by(f64::total_cmp);
        let q = |v: &[f64], f: f64| v[((v.len() as f64 * f).ceil() as usize).saturating_sub(1)];
        let mut line = Json::new()
            .text("kind", "shading_cut")
            .text("artifact", artifact)
            .with("threshold_px", 1)
            .with("views", views)
            .with("unique_cuts", measured.len())
            .with("samples_each_direction", samples)
            .with("pooled_samples", 2 * samples * views)
            .with("mean_triangles", avg)
            .with("normal_p95_deg", quantile(measured.iter().map(|r| (r.normal.as_slice(), r.frequency)), 0.95))
            .with("normal_max_deg", measured.iter().map(|r| r.normal_max).fold(0., f64::max))
            .with("uv_p95", quantile(measured.iter().map(|r| (r.uv.as_slice(), r.frequency)), 0.95))
            .with("geometric_median_ratio", q(&ratios, 0.5))
            .with("geometric_p95_ratio", q(&ratios, 0.95))
            .with("geometric_max_ratio", q(&ratios, 1.0))
            .with("tangent_degenerate_triangles", tangent_stats.0)
            .with("tangent_fallback_vertices", tangent_stats.1)
            .with("cut_oracle_pass", true);
        if !dag.t.is_empty() {
            line = line
                .with("tangent_p95_deg", quantile(measured.iter().map(|r| (r.tangent.as_slice(), r.frequency)), 0.95))
                .with("tangent_max_deg", measured.iter().map(|r| r.tangent_max).fold(0., f64::max))
                .with(
                    "tangent_sign_mismatch_fraction",
                    measured.iter().map(|r| r.sign_mismatches * r.frequency as u64).sum::<u64>() as f64
                        / (2 * samples * views) as f64,
                );
        }
        println!("{line}");
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quantiles_keep_view_multiplicity() {
        let a = [1., 3.];
        let b = [2., 4.];
        let v = [(a.as_slice(), 1), (b.as_slice(), 3)];
        assert_eq!(quantile(v.iter().copied(), 0.5), 2.);
        assert_eq!(quantile(v.iter().copied(), 0.95), 4.);
    }
    #[test]
    fn nearest_interpolation_preserves_known_normal_angle() {
        let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
        let n = [[0., 0., 1.]; 3];
        let other = [[0.5, 0., 0.8660254]; 3];
        let uv = [[0., 0.], [1., 0.], [0., 1.]];
        let ix = [0, 1, 2];
        let a = Mesh { p: &p, n: &n, uv: &uv, t: &[], ix: &ix };
        let b = Mesh { n: &other, ..a };
        let tree = Tree::new(b).expect("tree");
        for point in a.samples(128).expect("samples") {
            let hit = tree.nearest(point.p);
            let target = tree.mesh.at(hit.id, hit.w);
            assert!(hit.d < 1e-28);
            assert!((angle(point.normal, target.normal) - 30.).abs() < 1e-5);
        }
    }
    #[test]
    fn gradient_tangent_matches_planar_uv_chart() {
        let mut src = Source {
            positions: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            normals: vec![[0., 0., 1.]; 3],
            uvs: vec![[0., 0.], [1., 0.], [0., 1.]],
            indices: vec![0, 1, 2],
            ..Source::default()
        };
        assert_eq!(src.derive_tangents().expect("tangents"), (0, 0));
        assert!(src.tangents.iter().all(|t| *t == [1., 0., 0., 1.]));
    }
    #[test]
    fn nearest_tree_matches_brute_force_with_ties_and_degenerates() {
        let mut p = Vec::new();
        for id in 0..32 {
            let x = (id % 4) as f32 * 0.7;
            let y = (id / 4) as f32 * 0.4;
            let z = ((id * 7) % 11) as f32 * 0.13;
            p.extend([[x, y, z], [x + 0.3, y, z + 0.1], [x, y + 0.2, z - 0.1]]);
        }
        let first = [p[0], p[1], p[2]];
        p.extend(first);
        p.extend([[2., 2., 2.]; 3]);
        let n = vec![[0., 0., 1.]; p.len()];
        let uv = vec![[0., 0.]; p.len()];
        let ix: Vec<_> = (0..p.len() as u32).collect();
        let mesh = Mesh { p: &p, n: &n, uv: &uv, t: &[], ix: &ix };
        let tree = Tree::new(mesh).expect("tree");
        let queries =
            p.iter()
                .map(|x| x.map(f64::from))
                .chain((0..101).map(|i| {
                    [(i % 17) as f64 * 0.23 - 0.4, (i % 13) as f64 * 0.31 - 0.2, (i % 7) as f64 * 0.21 - 0.3]
                }));
        for point in queries {
            let mut brute = Hit { d: f64::INFINITY, id: usize::MAX, w: [0.; 3] };
            for id in 0..ix.len() / 3 {
                let (d, w) = closest(mesh.triangle(id), point);
                if d < brute.d || (d == brute.d && id < brute.id) {
                    brute = Hit { d, id, w };
                }
            }
            let hit = tree.nearest(point);
            assert_eq!(hit.id, brute.id, "query {point:?}");
            assert_eq!(hit.d.to_bits(), brute.d.to_bits());
            assert_eq!(hit.w, brute.w);
        }
    }
}
