//! Error-claim diagnosis: world-space cuts measured one-sided against the source,
//! per-group checks of each simplification step, and source components the cut drops.
//!
//! A world-space cut uses a camera far enough away that every sphere projects at the
//! same depth, so the screen rule reduces to `error < t` with `t` in scene units and
//! the pixel conversion of the evaluation drops out.
#![allow(clippy::field_reassign_with_default)]
use crate::{camera, cut::Dag, ours::Ours, report::Json, source::Source};
use meshoptimizer_rs::{measure_lod_quality, LodQuality, LodQualityMesh, LodQualityOptions, Positions, Workspace};
use rayon::prelude::*;

/// Camera distance for the world-space cuts, in scene units.
const FAR: f64 = 1e6;

fn measure(
    src_idx: &[u32],
    src_pos: &[[f32; 3]],
    lvl_idx: &[u32],
    lvl_pos: &[[f32; 3]],
    density: f64,
) -> Option<LodQuality> {
    let mut opts = LodQualityOptions::default();
    opts.samples_per_unit_area = density;
    let mut ws = Workspace::default();
    measure_lod_quality(
        LodQualityMesh::new(src_idx, Positions::from_packed(src_pos)),
        LodQualityMesh::new(lvl_idx, Positions::from_packed(lvl_pos)),
        &opts,
        &mut ws,
    )
    .ok()
}

fn area(idx: &[u32], pos: &[[f32; 3]]) -> f64 {
    idx.as_chunks::<3>()
        .0
        .iter()
        .map(|t| {
            let p = |i: u32| pos[i as usize].map(f64::from);
            let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
            let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
            let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
            0.5 * (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt()
        })
        .sum()
}

/// Connected components of the source by shared vertex index.
struct Components {
    /// Component of each vertex (`u32::MAX` for unreferenced vertices).
    of_vertex: Vec<u32>,
    /// Bounding-box diagonal of each component.
    extent: Vec<f64>,
    /// Triangles in each component.
    triangles: Vec<u32>,
}

fn components(src: &Source) -> Components {
    let n = src.positions.len();
    let mut parent: Vec<u32> = (0..n as u32).collect();
    fn find(p: &mut [u32], mut x: u32) -> u32 {
        while p[x as usize] != x {
            let up = p[p[x as usize] as usize];
            p[x as usize] = up;
            x = up;
        }
        x
    }
    for t in src.indices.as_chunks::<3>().0 {
        for k in 1..3 {
            let (a, b) = (find(&mut parent, t[0]), find(&mut parent, t[k]));
            if a != b {
                parent[a.max(b) as usize] = a.min(b);
            }
        }
    }
    let mut used = vec![false; n];
    for &i in &src.indices {
        used[i as usize] = true;
    }
    let mut id = vec![u32::MAX; n];
    let mut of_vertex = vec![u32::MAX; n];
    let mut lo: Vec<[f32; 3]> = Vec::new();
    let mut hi: Vec<[f32; 3]> = Vec::new();
    for v in 0..n {
        if !used[v] {
            continue;
        }
        let r = find(&mut parent, v as u32) as usize;
        if id[r] == u32::MAX {
            id[r] = lo.len() as u32;
            lo.push([f32::MAX; 3]);
            hi.push([f32::MIN; 3]);
        }
        let c = id[r];
        of_vertex[v] = c;
        for a in 0..3 {
            lo[c as usize][a] = lo[c as usize][a].min(src.positions[v][a]);
            hi[c as usize][a] = hi[c as usize][a].max(src.positions[v][a]);
        }
    }
    let extent =
        lo.iter().zip(&hi).map(|(l, h)| (0..3).map(|a| f64::from(h[a] - l[a]).powi(2)).sum::<f64>().sqrt()).collect();
    let mut triangles = vec![0u32; lo.len()];
    for t in src.indices.as_chunks::<3>().0 {
        triangles[of_vertex[t[0] as usize] as usize] += 1;
    }
    Components { of_vertex, extent, triangles }
}

/// Source summary: component count and size distribution.
pub fn source_lines(src: &Source, label: &Json) -> Vec<String> {
    let c = components(src);
    let small = |limit: u32| c.triangles.iter().filter(|&&t| t < limit).count();
    let mut ext = c.extent.clone();
    ext.sort_by(f64::total_cmp);
    vec![label
        .clone()
        .text("kind", "diag_source")
        .with("components", c.triangles.len())
        .with("components_under_16_tris", small(16))
        .with("components_under_128_tris", small(128))
        .with("largest_component_tris", c.triangles.iter().copied().max().unwrap_or(0))
        .with("second_extent", ext.iter().rev().nth(1).copied().unwrap_or(0.0))
        .to_string()]
}

/// World-space cuts at `t = radius * 2^-k` measured in both directions.
pub fn world_cuts(
    dag: &dyn Dag,
    src: &Source,
    samples: f64,
    jobs: usize,
    vertex_cut: bool,
    label: &Json,
) -> Result<Vec<String>, String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(jobs).build().map_err(|e| e.to_string())?;
    let radius = f64::from(src.radius());
    let view = camera::View::orbit([0.3, 0.5, 0.8], FAR);
    let k = camera::px_per_unit(FAR);
    let density = samples / src.area().max(1e-12);
    let comps = if vertex_cut { Some(components(src)) } else { None };
    let steps: Vec<i32> = (3..=13).collect();
    let lines: Vec<String> = pool.install(|| {
        steps
            .par_iter()
            .map(|&s| {
                let t = radius * 2f64.powi(-s);
                let sel = dag.cut(&view, t * k);
                let claimed = sel.claimed_px / k;
                let cut = dag.cut_mesh(&sel.drawn);
                let pos: &[[f32; 3]] = cut.positions.as_deref().unwrap_or(&src.positions);
                let q = measure(&src.indices, &src.positions, &cut.indices, pos, density);
                let mut j = label
                    .clone()
                    .text("kind", "diag_cut")
                    .with("step", s)
                    .with("t", t)
                    .with("triangles", dag.triangles(&sel.drawn))
                    .with("claimed", claimed);
                if let Some(q) = q {
                    j = j
                        .with("s2l_max", q.source_to_level.max)
                        .with("s2l_rms", q.source_to_level.rms)
                        .with("l2s_max", q.level_to_source.max)
                        .with("l2s_rms", q.level_to_source.rms)
                        .with("ratio_claimed", if claimed > 0.0 { q.symmetric.max / claimed } else { f64::NAN })
                        .with("ratio_t", q.symmetric.max / t);
                }
                if let Some(c) = &comps {
                    let mut seen = vec![false; c.triangles.len()];
                    for &i in &cut.indices {
                        let id = c.of_vertex[i as usize];
                        if id != u32::MAX {
                            seen[id as usize] = true;
                        }
                    }
                    let missing: Vec<usize> = (0..seen.len()).filter(|&i| !seen[i]).collect();
                    let max_ext = missing.iter().map(|&i| c.extent[i]).fold(0.0, f64::max);
                    let tris: u64 = missing.iter().map(|&i| u64::from(c.triangles[i])).sum();
                    j = j
                        .with("components_missing", missing.len())
                        .with("missing_tris", tris)
                        .with("missing_max_extent", max_ext);
                }
                j.to_string()
            })
            .collect()
    });
    Ok(lines)
}

fn closest_sq(p: [f64; 3], t: [[f64; 3]; 3]) -> f64 {
    // Closest point on a triangle (Ericson, Real-Time Collision Detection 5.1.5).
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let lerp = |a: [f64; 3], d: [f64; 3], s: f64| [a[0] + d[0] * s, a[1] + d[1] * s, a[2] + d[2] * s];
    let [a, b, c] = t;
    let (ab, ac, ap) = (sub(b, a), sub(c, a), sub(p, a));
    let q = 'q: {
        let (d1, d2) = (dot(ab, ap), dot(ac, ap));
        if d1 <= 0.0 && d2 <= 0.0 {
            break 'q a;
        }
        let bp = sub(p, b);
        let (d3, d4) = (dot(ab, bp), dot(ac, bp));
        if d3 >= 0.0 && d4 <= d3 {
            break 'q b;
        }
        let vc = d1 * d4 - d3 * d2;
        if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 && d1 - d3 > 0.0 {
            break 'q lerp(a, ab, d1 / (d1 - d3));
        }
        let cp = sub(p, c);
        let (d5, d6) = (dot(ab, cp), dot(ac, cp));
        if d6 >= 0.0 && d5 <= d6 {
            break 'q c;
        }
        let vb = d5 * d2 - d1 * d6;
        if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 && d2 - d6 > 0.0 {
            break 'q lerp(a, ac, d2 / (d2 - d6));
        }
        let va = d3 * d6 - d5 * d4;
        if va <= 0.0 && d4 - d3 >= 0.0 && d5 - d6 >= 0.0 && (d4 - d3) + (d5 - d6) > 0.0 {
            break 'q lerp(b, sub(c, b), (d4 - d3) / ((d4 - d3) + (d5 - d6)));
        }
        let sum = va + vb + vc;
        if sum <= 0.0 {
            // Degenerate triangle: fall back to the nearest corner.
            let best = [a, b, c]
                .into_iter()
                .min_by(|x, y| dot(sub(p, *x), sub(p, *x)).total_cmp(&dot(sub(p, *y), sub(p, *y))));
            break 'q best.unwrap_or(a);
        }
        let (v, w) = (vb / sum, vc / sum);
        [a[0] + ab[0] * v + ac[0] * w, a[1] + ab[1] * v + ac[1] * w, a[2] + ab[2] * v + ac[2] * w]
    };
    let d = sub(p, q);
    dot(d, d)
}

/// Prototype of an opt-in error rule: when simplifying a group removes a whole
/// connected component of its input, raise the group error to the largest distance
/// from that component (corners and triangle centroids) to the group's simplified
/// triangles, then restore monotonicity over the DAG. Returns the number of groups raised.
pub fn price_removed_components(o: &mut Ours, src: &Source) -> usize {
    use std::collections::{HashMap, HashSet};
    let g_count = o.groups.len();
    let mut members: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    let mut patch: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    for cl in &o.clusters {
        let tris = &o.indices[cl.first_index as usize..(cl.first_index + cl.index_count) as usize];
        members[cl.group as usize].extend_from_slice(tris);
        if cl.refined >= 0 {
            patch[cl.refined as usize].extend_from_slice(tris);
        }
    }
    let p64 = |i: u32| src.positions[i as usize].map(f64::from);
    let raised: Vec<f32> = (0..g_count)
        .into_par_iter()
        .map(|g| {
            let (m, p) = (&members[g], &patch[g]);
            if o.groups[g].error == f32::MAX || p.is_empty() {
                return 0.0;
            }
            let mut local: HashMap<u32, u32> = HashMap::new();
            for &v in m {
                let n = local.len() as u32;
                local.entry(v).or_insert(n);
            }
            let mut parent: Vec<u32> = (0..local.len() as u32).collect();
            fn find(p: &mut [u32], mut x: u32) -> u32 {
                while p[x as usize] != x {
                    let up = p[p[x as usize] as usize];
                    p[x as usize] = up;
                    x = up;
                }
                x
            }
            for t in m.as_chunks::<3>().0 {
                let l = |v: u32| local.get(&v).copied().unwrap_or(0);
                for k in 1..3 {
                    let (a, b) = (find(&mut parent, l(t[0])), find(&mut parent, l(t[k])));
                    if a != b {
                        parent[a.max(b) as usize] = a.min(b);
                    }
                }
            }
            let kept: HashSet<u32> = p.iter().copied().collect();
            let mut alive = vec![false; local.len()];
            for (&v, &l) in &local {
                if kept.contains(&v) {
                    let r = find(&mut parent, l);
                    alive[r as usize] = true;
                }
            }
            let out: Vec<[[f64; 3]; 3]> =
                p.as_chunks::<3>().0.iter().map(|t| [p64(t[0]), p64(t[1]), p64(t[2])]).collect();
            let dist = |q: [f64; 3]| out.iter().map(|t| closest_sq(q, *t)).fold(f64::INFINITY, f64::min).sqrt();
            let mut worst = 0f64;
            for t in m.as_chunks::<3>().0 {
                let l = local.get(&t[0]).copied().unwrap_or(0);
                if alive[find(&mut parent, l) as usize] {
                    continue;
                }
                let (a, b, c) = (p64(t[0]), p64(t[1]), p64(t[2]));
                let centroid = [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0, (a[2] + b[2] + c[2]) / 3.0];
                for q in [a, b, c, centroid] {
                    worst = worst.max(dist(q));
                }
            }
            worst as f32
        })
        .collect();
    raise(o, &raised)
}

/// Prototype of an opt-in error rule: each group's error is at least the sampled
/// two-sided distance between the group's clusters and the clusters simplified from
/// them (the step the error is meant to describe), then monotonicity is restored.
/// Returns the number of groups raised.
pub fn price_measured_steps(o: &mut Ours, src: &Source) -> usize {
    let g_count = o.groups.len();
    let mut members: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    let mut patch: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    for cl in &o.clusters {
        let tris = &o.indices[cl.first_index as usize..(cl.first_index + cl.index_count) as usize];
        members[cl.group as usize].extend_from_slice(tris);
        if cl.refined >= 0 {
            patch[cl.refined as usize].extend_from_slice(tris);
        }
    }
    let raised: Vec<f32> = (0..g_count)
        .into_par_iter()
        .map(|g| {
            if o.groups[g].error == f32::MAX || patch[g].is_empty() {
                return 0.0;
            }
            let a = area(&members[g], &src.positions).max(1e-12);
            measure(&members[g], &src.positions, &patch[g], &src.positions, STEP_SAMPLES / a)
                .map_or(0.0, |q| q.symmetric.max as f32)
        })
        .collect();
    raise(o, &raised)
}

/// Interior samples per group step (spread over the group's input area).
const STEP_SAMPLES: f64 = 4000.0;

fn raise(o: &mut Ours, raised: &[f32]) -> usize {
    let g_count = o.groups.len();
    let mut count = 0;
    for (g, &r) in raised.iter().enumerate() {
        if r > o.groups[g].error && o.groups[g].error < f32::MAX {
            o.groups[g].error = r;
            count += 1;
        }
    }
    // Monotonicity: a group's error is at least the self error of each of its clusters.
    let mut order: Vec<usize> = (0..g_count).collect();
    order.sort_by_key(|&g| o.groups[g].depth);
    for &g in &order {
        let (first, n) = (o.groups[g].first_cluster as usize, o.groups[g].cluster_count as usize);
        let mut e = 0f32;
        for c in first..first + n {
            let r = o.clusters[c].refined;
            if r >= 0 {
                e = e.max(o.groups[r as usize].error);
            }
        }
        if o.groups[g].error < e {
            o.groups[g].error = e;
        }
    }
    count
}

/// One simplification step per group: the group's clusters (input) against the
/// clusters simplified from it (output), both directions, against the recorded error.
pub fn group_steps(o: &Ours, src: &Source, jobs: usize, label: &Json) -> Result<Vec<String>, String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(jobs).build().map_err(|e| e.to_string())?;
    let g_count = o.groups.len();
    let mut members: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    let mut patch: Vec<Vec<u32>> = vec![Vec::new(); g_count];
    let mut own_max = vec![0f32; g_count];
    for cl in &o.clusters {
        let tris = &o.indices[cl.first_index as usize..(cl.first_index + cl.index_count) as usize];
        members[cl.group as usize].extend_from_slice(tris);
        if cl.refined >= 0 {
            patch[cl.refined as usize].extend_from_slice(tris);
            let e = o.groups[cl.refined as usize].error;
            own_max[cl.group as usize] = own_max[cl.group as usize].max(e);
        }
    }
    // (depth, error, inherited, local s2l, local l2s, input tris, output tris)
    type Row = (i32, f32, f32, f64, f64, usize, usize);
    let rows: Vec<Row> = pool.install(|| {
        (0..g_count)
            .into_par_iter()
            .filter(|&g| o.groups[g].error < f32::MAX && !patch[g].is_empty())
            .filter_map(|g| {
                let a = area(&members[g], &src.positions).max(1e-12);
                let q = measure(&members[g], &src.positions, &patch[g], &src.positions, STEP_SAMPLES / a)?;
                Some((
                    o.groups[g].depth,
                    o.groups[g].error,
                    own_max[g],
                    q.source_to_level.max,
                    q.level_to_source.max,
                    members[g].len() / 3,
                    patch[g].len() / 3,
                ))
            })
            .collect()
    });
    let mut lines = Vec::new();
    let max_depth = rows.iter().map(|r| r.0).max().unwrap_or(0);
    let pct = |v: &mut Vec<f64>, p: f64| {
        if v.is_empty() {
            return f64::NAN;
        }
        v.sort_by(f64::total_cmp);
        v[((v.len() - 1) as f64 * p).round() as usize]
    };
    for d in (0..=max_depth).map(Some).chain([None]) {
        let sel: Vec<&Row> = rows.iter().filter(|r| d.is_none_or(|d| r.0 == d)).collect();
        // Groups whose own step sets the error (error above every inherited error).
        let own: Vec<&&Row> = sel.iter().filter(|r| r.1 > r.2).collect();
        let mut step: Vec<f64> = own.iter().map(|r| r.3.max(r.4) / f64::from(r.1).max(1e-30)).collect();
        let mut s2l: Vec<f64> = own.iter().map(|r| r.3 / f64::from(r.1).max(1e-30)).collect();
        let mut l2s: Vec<f64> = own.iter().map(|r| r.4 / f64::from(r.1).max(1e-30)).collect();
        let zero = sel.iter().filter(|r| r.1 == 0.0).count();
        lines.push(
            label
                .clone()
                .text("kind", "diag_group")
                .with("depth", d.unwrap_or(-1))
                .with("groups", sel.len())
                .with("own_step_groups", own.len())
                .with("zero_error_groups", zero)
                .with("step_over_error_median", pct(&mut step, 0.5))
                .with("step_over_error_p95", pct(&mut step, 0.95))
                .with("step_over_error_max", pct(&mut step, 1.0))
                .with("s2l_over_error_median", pct(&mut s2l, 0.5))
                .with("l2s_over_error_median", pct(&mut l2s, 0.5))
                .to_string(),
        );
    }
    let mut worst: Vec<&Row> = rows.iter().filter(|r| r.1 > r.2).collect();
    worst.sort_by(|a, b| {
        (b.3.max(b.4) / f64::from(b.1).max(1e-30)).total_cmp(&(a.3.max(a.4) / f64::from(a.1).max(1e-30)))
    });
    for r in worst.iter().take(8) {
        lines.push(
            label
                .clone()
                .text("kind", "diag_group_worst")
                .with("depth", r.0)
                .with("error", r.1)
                .with("inherited", r.2)
                .with("s2l", r.3)
                .with("l2s", r.4)
                .with("in_tris", r.5)
                .with("out_tris", r.6)
                .to_string(),
        );
    }
    Ok(lines)
}

/// Per-level stored error distribution and ownership of replacement clusters.
pub fn plateau(o: &Ours, label: &Json) -> Vec<String> {
    let mut owners = vec![Vec::new(); o.groups.len()];
    let mut inherited = vec![0f32; o.groups.len()];
    for c in &o.clusters {
        if c.refined >= 0 {
            let child = c.refined as usize;
            if !owners[child].contains(&c.group) {
                owners[child].push(c.group);
            }
            inherited[c.group as usize] = inherited[c.group as usize].max(o.groups[child].error);
        }
    }
    let mut lines = Vec::new();
    for depth in 0..o.level_count {
        let mut groups: Vec<_> = o.groups.iter().enumerate().filter(|(_, g)| g.depth == depth as i32).collect();
        let stuck = groups.iter().filter(|(_, g)| g.error == f32::MAX).count();
        let shared = groups.iter().filter(|(i, _)| owners[*i].len() > 1).count();
        groups.retain(|(_, g)| g.error < f32::MAX);
        groups.sort_by(|a, b| a.1.error.total_cmp(&b.1.error));
        let pct = |p: f64| {
            groups.get(((groups.len().saturating_sub(1)) as f64 * p).round() as usize).map_or(0.0, |(_, g)| g.error)
        };
        lines.push(
            label
                .clone()
                .text("kind", "plateau_level")
                .with("depth", depth)
                .with("finite", groups.len())
                .with("stuck", stuck)
                .with("shared", shared)
                .with("median", pct(0.5))
                .with("p95", pct(0.95))
                .with("max", pct(1.0))
                .to_string(),
        );
        for &(id, g) in groups.iter().rev().take(3) {
            let mut seen = vec![false; o.groups.len()];
            let mut stack = vec![id];
            let mut source_triangles = 0usize;
            while let Some(region) = stack.pop() {
                if seen[region] {
                    continue;
                }
                seen[region] = true;
                let r = &o.groups[region];
                for c in &o.clusters[r.first_cluster as usize..(r.first_cluster + r.cluster_count) as usize] {
                    if c.refined < 0 {
                        source_triangles += c.index_count as usize / 3;
                    } else {
                        stack.push(c.refined as usize);
                    }
                }
            }
            let input = &o.clusters[g.first_cluster as usize..(g.first_cluster + g.cluster_count) as usize];
            let input_triangles: usize = input.iter().map(|c| c.index_count as usize / 3).sum();
            let split_children =
                input.iter().filter(|c| c.refined >= 0 && owners[c.refined as usize].len() > 1).count();
            lines.push(
                label
                    .clone()
                    .text("kind", "plateau_max")
                    .with("depth", depth)
                    .with("group", id)
                    .with("error", g.error)
                    .with("inherited", inherited[id])
                    .with("parents", owners[id].len())
                    .with("source_triangles", source_triangles)
                    .with("input_triangles", input_triangles)
                    .with("split_child_links", split_children)
                    .to_string(),
            );
        }
    }
    lines
}
