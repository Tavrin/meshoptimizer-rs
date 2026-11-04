//! Cut evaluation shared by both builders: error per triangle budget and
//! traversal work, over orbit views.
#![allow(clippy::field_reassign_with_default)]
use crate::{camera, report::Json, source::Source};
use meshoptimizer_rs::{measure_lod_quality, LodQualityMesh, LodQualityOptions, Positions, Workspace};
use rayon::prelude::*;
use std::collections::HashMap;

/// Result of selecting a cut for one view and threshold.
#[derive(Default, Clone)]
pub struct Selection {
    /// Ids of the drawn clusters.
    pub drawn: Vec<u32>,
    /// Hierarchy nodes visited.
    pub nodes: u64,
    /// Clusters whose own error was tested.
    pub tested: u64,
    /// Largest projected self error among drawn clusters, in pixels.
    pub claimed_px: f64,
    /// Largest self error among drawn clusters, in scene units.
    pub claimed_world: f64,
    /// Smallest parent error among drawn clusters (`f32::MAX` for the root set).
    pub min_parent_error: f32,
}

/// Triangle soup for a cut. `positions` is `None` when the cut indexes the source vertices.
pub struct CutMesh {
    pub positions: Option<Vec<[f32; 3]>>,
    pub indices: Vec<u32>,
}

/// A built cluster DAG that can select and materialise cuts.
pub trait Dag: Sync {
    /// Canonical cut without culling: the whole mesh is covered.
    fn cut(&self, view: &camera::View, threshold_px: f64) -> Selection;
    /// Hierarchical traversal, optionally with frustum culling; used for work counts.
    fn traverse(&self, view: &camera::View, threshold_px: f64, frustum: bool) -> Selection;
    /// Triangles in the drawn clusters.
    fn triangles(&self, drawn: &[u32]) -> u64;
    /// Geometry of the drawn clusters.
    fn cut_mesh(&self, drawn: &[u32]) -> CutMesh;
    /// Whether a cluster is original geometry.
    fn is_leaf(&self, cluster: u32) -> bool;
}

/// Evaluation settings.
pub struct Settings {
    pub dirs: usize,
    pub spans: Vec<f64>,
    pub thresholds: Vec<f64>,
    pub samples: f64,
    pub jobs: usize,
}

struct Row {
    span: f64,
    thr: f64,
    tris: u64,
    claimed_px: f64,
    claimed_world: f64,
    max_world: f64,
    max_px: f64,
    rms_px: f64,
    nodes: u64,
    tested: u64,
    drawn: u64,
    key: u64,
}

fn hash(ids: &[u32]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &i in ids {
        h = (h ^ u64::from(i)).wrapping_mul(0x100000001b3);
    }
    h ^ ids.len() as u64
}

fn p95(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * 0.95).ceil() as usize]
}
fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

/// Self-checks of the selector. Returns `(name, passed, detail)` lines.
pub fn self_checks(dag: &dyn Dag, src: &Source, clusters: usize) -> Vec<(String, bool, String)> {
    let view = camera::View::orbit([0.3, 0.5, 0.8], camera::distance_for_span(src.radius() as f64, 256.0));
    let mut out = Vec::new();
    let low = dag.cut(&view, 1e-12);
    let tris = dag.triangles(&low.drawn);
    let all_leaf = low.drawn.iter().all(|&c| dag.is_leaf(c));
    out.push((
        "threshold 0 selects exactly the source clusters".into(),
        tris == src.triangles() as u64 && all_leaf,
        format!("{tris} triangles of {} source, all leaves: {all_leaf}", src.triangles()),
    ));
    let high = dag.cut(&view, 1e15);
    out.push((
        "huge threshold selects only the root set".into(),
        !high.drawn.is_empty() && high.min_parent_error == f32::MAX && high.drawn.len() < clusters,
        format!("{} clusters of {clusters}, min parent error {:e}", high.drawn.len(), high.min_parent_error),
    ));
    let (mut a, mut b) = (dag.cut(&view, 1.0).drawn, dag.traverse(&view, 1.0, false).drawn);
    a.sort_unstable();
    b.sort_unstable();
    out.push((
        "hierarchical traversal equals the flat cut at 1 px".into(),
        a == b,
        format!("{} vs {} clusters", a.len(), b.len()),
    ));
    out
}

/// Run all views and thresholds; return JSON summary lines.
pub fn evaluate(dag: &dyn Dag, src: &Source, s: &Settings, label: &Json) -> Result<Vec<String>, String> {
    let radius = src.radius() as f64;
    let dirs = camera::directions(s.dirs);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(s.jobs).build().map_err(|e| e.to_string())?;
    let area = src.area();
    let mut opts = LodQualityOptions::default();
    opts.samples_per_unit_area = s.samples / area.max(1e-12);
    let views: Vec<(f64, camera::View, f64)> = dirs
        .iter()
        .flat_map(|&d| {
            s.spans.iter().map(move |&span| {
                let dist = camera::distance_for_span(radius, span);
                (span, camera::View::orbit(d, dist), dist)
            })
        })
        .collect();
    let mut rows: Vec<Row> = Vec::new();
    let mut cuts: HashMap<u64, Vec<u32>> = HashMap::new();
    let selected: Vec<Vec<(Row, Vec<u32>)>> = pool.install(|| {
        views
            .par_iter()
            .map(|(span, view, _)| {
                s.thresholds
                    .iter()
                    .map(|&thr| {
                        let mut cut = dag.cut(view, thr);
                        cut.drawn.sort_unstable();
                        let counted = dag.traverse(view, thr, true);
                        let row = Row {
                            span: *span,
                            thr,
                            tris: dag.triangles(&cut.drawn),
                            claimed_px: cut.claimed_px,
                            claimed_world: cut.claimed_world,
                            max_world: 0.0,
                            max_px: 0.0,
                            rms_px: 0.0,
                            nodes: counted.nodes,
                            tested: counted.tested,
                            drawn: counted.drawn.len() as u64,
                            key: hash(&cut.drawn),
                        };
                        (row, cut.drawn)
                    })
                    .collect()
            })
            .collect()
    });
    for per_view in selected {
        for (row, ids) in per_view {
            cuts.entry(row.key).or_insert(ids);
            rows.push(row);
        }
    }
    let unique: Vec<(u64, Vec<u32>)> = cuts.into_iter().collect();
    let measured: HashMap<u64, (f64, f64)> = pool.install(|| {
        unique
            .par_iter()
            .map(|(key, ids)| {
                let cut = dag.cut_mesh(ids);
                let pos: &[[f32; 3]] = cut.positions.as_deref().unwrap_or(&src.positions);
                let mut ws = Workspace::default();
                let q = measure_lod_quality(
                    LodQualityMesh::new(&src.indices, Positions::from_packed(&src.positions)),
                    LodQualityMesh::new(&cut.indices, Positions::from_packed(pos)),
                    &opts,
                    &mut ws,
                );
                match q {
                    Ok(q) => (*key, (q.symmetric.max, q.symmetric.rms)),
                    Err(_) => (*key, (f64::NAN, f64::NAN)),
                }
            })
            .collect()
    });
    for (row, (_, _, dist)) in
        rows.iter_mut().zip(views.iter().flat_map(|v| std::iter::repeat_n(v, s.thresholds.len())))
    {
        let (max_w, rms_w) = measured.get(&row.key).copied().unwrap_or((f64::NAN, f64::NAN));
        let scale = camera::px_per_unit(*dist - radius);
        row.max_world = max_w;
        row.max_px = max_w * scale;
        row.rms_px = rms_w * scale;
    }
    let mut lines = Vec::new();
    for &thr in &s.thresholds {
        for span in s.spans.iter().map(|x| Some(*x)).chain([None]) {
            let sel: Vec<&Row> = rows.iter().filter(|r| r.thr == thr && span.is_none_or(|x| r.span == x)).collect();
            let col = |f: &dyn Fn(&Row) -> f64| sel.iter().map(|r| f(r)).collect::<Vec<f64>>();
            // Both sides in scene units: every drawn cluster claims its own error as a
            // bound, so the cut's deviation should stay under the largest claim.
            let mut ratio: Vec<f64> =
                sel.iter().filter(|r| r.claimed_world > 0.0).map(|r| r.max_world / r.claimed_world).collect();
            // Earlier form: measured converted at the mesh's nearest depth over the claim
            // projected at each cluster's own depth (mixes two depths).
            let mut ratio_px: Vec<f64> =
                sel.iter().filter(|r| r.claimed_px > 0.0).map(|r| r.max_px / r.claimed_px).collect();
            lines.push(
                label
                    .clone()
                    .text("kind", "cut")
                    .with("threshold_px", thr)
                    .with("span_px", span.unwrap_or(-1.0))
                    .with("views", sel.len())
                    .with("mean_triangles", mean(&col(&|r| r.tris as f64)))
                    .with("p95_max_dev_px", p95(&mut col(&|r| r.max_px)))
                    .with("mean_max_dev_px", mean(&col(&|r| r.max_px)))
                    .with("p95_rms_dev_px", p95(&mut col(&|r| r.rms_px)))
                    .with("median_measured_over_claimed", median(&mut ratio))
                    .with("p95_measured_over_claimed", p95(&mut ratio))
                    .with("median_measured_over_claimed_px", median(&mut ratio_px))
                    .with("mean_nodes", mean(&col(&|r| r.nodes as f64)))
                    .with("mean_clusters_tested", mean(&col(&|r| r.tested as f64)))
                    .with("mean_clusters_drawn", mean(&col(&|r| r.drawn as f64)))
                    .to_string(),
            );
        }
    }
    lines.push(
        label
            .clone()
            .text("kind", "cut_meta")
            .with("views", rows.len())
            .with("unique_cuts", unique.len())
            .with("samples_per_cut_source", s.samples)
            .to_string(),
    );
    Ok(lines)
}
