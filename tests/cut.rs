#![cfg(feature = "clusterlod")]
#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::clusterlod::{
    self,
    cut::{self, CutError, CutView, Projection, Threshold, Violation},
    DagBuffers, FlatDagCluster, FlatDagGroup, LodBounds, Mesh, Node,
};
use meshoptimizer_rs::Workspace;

type Dag = (Vec<FlatDagGroup>, Vec<FlatDagCluster>);

fn grid(n: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..(n + 1) * (n + 1))
        .map(|i| {
            let (x, y) = ((i % (n + 1)) as f32, (i / (n + 1)) as f32);
            [x, y, (x * 0.3).sin() + (y * 0.2).cos()]
        })
        .collect();
    let mut idx = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            idx.extend([a, a + 1, a + n + 1, a + n + 1, a + 1, a + n + 2]);
        }
    }
    (p, idx)
}
fn disk(n: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = vec![[0.; 3]];
    p.extend((0..n).map(|i| {
        let a = i as f32 * core::f32::consts::TAU / n as f32;
        [20. * a.cos(), 20. * a.sin(), 0.]
    }));
    (
        p,
        (0..n).flat_map(|i| [0, i + 1, (i + 1) % n + 1]).collect(),
    )
}
fn sphere(rows: u32, cols: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = Vec::new();
    for r in 0..=rows {
        let t = r as f32 * core::f32::consts::PI / rows as f32;
        for c in 0..cols {
            let a = c as f32 * core::f32::consts::TAU / cols as f32;
            p.push([
                10. * t.sin() * a.cos(),
                10. * t.cos(),
                10. * t.sin() * a.sin(),
            ]);
        }
    }
    let mut idx = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let (a, b) = (r * cols + c, r * cols + (c + 1) % cols);
            idx.extend([a, a + cols, b, b, a + cols, b + cols]);
        }
    }
    (p, idx)
}

fn build(mesh: (Vec<[f32; 3]>, Vec<u32>), merge_previous: f32) -> Dag {
    let (mut positions, indices) = mesh;
    let mut config = clusterlod::default_config(16).unwrap();
    config.simplify_error_merge_previous = merge_previous;
    let mut node = Node::default();
    node.group = -1;
    let (mut source, mut pos, mut idx) =
        (vec![0; 1 << 18], vec![[0.; 3]; 1 << 18], vec![0; 1 << 19]);
    let (mut clusters, mut groups) = (
        vec![FlatDagCluster::default(); 1 << 14],
        vec![FlatDagGroup::default(); 1 << 14],
    );
    let mut nodes = vec![node; 1 << 14];
    let counts = clusterlod::build_dag_into(
        config,
        Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        4,
        DagBuffers {
            vertex_source: &mut source,
            vertex_positions: &mut pos,
            indices: &mut idx,
            clusters: &mut clusters,
            groups: &mut groups,
            nodes: &mut nodes,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    groups.truncate(counts.groups);
    clusters.truncate(counts.clusters);
    (groups, clusters)
}
fn raw_fixtures() -> Vec<Dag> {
    vec![
        build(grid(32), 1.),
        build(disk(600), 1.),
        build(sphere(24, 40), 1.),
    ]
}
/// Builder output after the repair a container writer applies.
fn fixtures() -> Vec<Dag> {
    let mut dags = raw_fixtures();
    for (groups, clusters) in &mut dags {
        cut::enforce_monotonic(groups, clusters).unwrap();
    }
    dags
}

struct Rng(u64);
impl Rng {
    fn unit(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
    fn view(&mut self, scales: &[f32]) -> CutView {
        let mut position = [0.; 3];
        position
            .iter_mut()
            .for_each(|v| *v = self.unit() * 100. - 40.);
        let projection = if self.unit() < 0.5 {
            Projection::Perspective {
                proj_y: 0.7 + 1.8 * self.unit(),
                znear: 0.05 + 0.4 * self.unit(),
            }
        } else {
            Projection::Orthographic {
                pixels_per_unit: 0.5 + 30. * self.unit(),
            }
        };
        let scale = scales[(self.unit() * scales.len() as f32) as usize % scales.len()];
        CutView {
            position,
            projection,
            viewport_height: 1080.,
            scale,
        }
    }
}

/// Clusters each group produced: the next layer above any cluster of that group.
struct Cover {
    produced: Vec<Vec<u32>>,
}
fn leaves(groups: &[FlatDagGroup], clusters: &[FlatDagCluster]) -> Cover {
    let mut produced = vec![Vec::new(); groups.len()];
    for (i, c) in clusters.iter().enumerate().filter(|(_, c)| c.refined >= 0) {
        produced[c.refined as usize].push(i as u32);
    }
    Cover { produced }
}
/// Every chain from a source cluster upward (to a cluster produced by its group,
/// and so on until none is) contains exactly one cut cluster.
fn assert_unique_cover(cover: &Cover, clusters: &[FlatDagCluster], cut: &[u32]) {
    let mut in_cut = vec![0u32; clusters.len()];
    cut.iter().for_each(|&c| in_cut[c as usize] = 1);
    // (fewest, most) cut clusters over all chains from a cluster; producers
    // have larger indices, so one descending pass suffices.
    let mut range = vec![(0u32, 0u32); clusters.len()];
    for i in (0..clusters.len()).rev() {
        let above = &cover.produced[clusters[i].group as usize];
        let lo = above
            .iter()
            .map(|&y| range[y as usize].0)
            .min()
            .unwrap_or(0);
        let hi = above
            .iter()
            .map(|&y| range[y as usize].1)
            .max()
            .unwrap_or(0);
        range[i] = (lo + in_cut[i], hi + in_cut[i]);
        if clusters[i].refined < 0 {
            assert_eq!(range[i], (1, 1), "source cluster {i}");
        }
    }
}

#[test]
fn view_cut_is_unique_and_equals_sequential_refinement() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let shear = cut::max_scale([[1., 0.7, 0.], [0., 1., 0.3], [0.2, 0., 1.5]]);
    let mut sizes = std::collections::BTreeSet::new();
    for (groups, clusters) in fixtures() {
        let cover = leaves(&groups, &clusters);
        let mut out = vec![0; clusters.len()];
        let mut reference = vec![0; clusters.len()];
        for i in 0..150 {
            let view = rng.view(&[1., 2.5, shear]);
            let t = Threshold::Pixels([0.25, 0.5, 1., 2., 4., 8.][i % 6]);
            let n = cut::select_cut(&groups, &clusters, &view, t, None, &mut out).unwrap();
            let m =
                cut::refine_sequential(&groups, &clusters, &view, t, None, &mut reference).unwrap();
            assert_eq!(out[..n], reference[..m]);
            assert_unique_cover(&cover, &clusters, &out[..n]);
            sizes.insert(n);
        }
        let view = rng.view(&[1.]);
        let n =
            cut::select_cut(&groups, &clusters, &view, Threshold::Finest, None, &mut out).unwrap();
        let source: Vec<u32> = (0..clusters.len() as u32)
            .filter(|&i| clusters[i as usize].refined < 0)
            .collect();
        assert_eq!(out[..n], source[..]);
        let m = cut::refine_sequential(
            &groups,
            &clusters,
            &view,
            Threshold::Finest,
            None,
            &mut reference,
        )
        .unwrap();
        assert_eq!(reference[..m], source[..]);
    }
    assert!(sizes.len() > 20, "views must exercise many different cuts");
    let view = rng.view(&[1.]);
    let (groups, clusters) = build(grid(8), 1.);
    assert_eq!(
        cut::select_cut(&groups, &clusters, &view, Threshold::Finest, None, &mut []),
        Err(CutError::BufferTooSmall)
    );
    let bad = Threshold::Pixels(-1.);
    assert_eq!(
        cut::select_cut(&groups, &clusters, &view, bad, None, &mut [0; 8]),
        Err(CutError::InvalidParameter)
    );
}

#[test]
fn resident_cut_equals_sequential_only_for_closed_sets() {
    let mut rng = Rng(0x1234_5678_9abc_def1);
    for (groups, clusters) in fixtures() {
        let cover = leaves(&groups, &clusters);
        let (mut offsets, mut parents) = (vec![0; groups.len() + 1], vec![0; clusters.len()]);
        let links = cut::group_parents(&groups, &clusters, &mut offsets, &mut parents).unwrap();
        assert_eq!(links, offsets[groups.len()] as usize);
        for (q, w) in offsets.windows(2).enumerate() {
            let brute: std::collections::BTreeSet<u32> = clusters
                .iter()
                .filter(|c| c.refined == q as i32)
                .map(|c| c.group)
                .collect();
            assert!(parents[w[0] as usize..w[1] as usize]
                .iter()
                .copied()
                .eq(brute));
        }
        let parentless: Vec<bool> = (0..groups.len())
            .map(|q| !clusters.iter().any(|c| c.refined == q as i32))
            .collect();
        let (mut out, mut reference) = (vec![0; clusters.len()], vec![0; clusters.len()]);
        for _ in 0..60 {
            let view = rng.view(&[1., 1.7]);
            let t = Threshold::Pixels(0.5 + 4. * rng.unit());
            let mut resident: Vec<bool> =
                parentless.iter().map(|&p| p || rng.unit() < 0.5).collect();
            for q in 0..groups.len() {
                // Parents have larger indices, so one ascending pass closes the set.
                for p in &parents[offsets[q] as usize..offsets[q + 1] as usize] {
                    resident[*p as usize] |= resident[q];
                }
            }
            let n =
                cut::select_cut(&groups, &clusters, &view, t, Some(&resident), &mut out).unwrap();
            let m = cut::refine_sequential(
                &groups,
                &clusters,
                &view,
                t,
                Some(&resident),
                &mut reference,
            )
            .unwrap();
            assert_eq!(out[..n], reference[..m]);
            assert_unique_cover(&cover, &clusters, &out[..n]);
        }
        // Dropping a parent whose child must be refined leaves a hole the oracle reports.
        let q = parentless.iter().position(|&p| !p).unwrap();
        let mut resident = vec![true; groups.len()];
        resident[parents[offsets[q] as usize] as usize] = false;
        let view = rng.view(&[1.]);
        let r = cut::refine_sequential(
            &groups,
            &clusters,
            &view,
            Threshold::Finest,
            Some(&resident),
            &mut reference,
        );
        assert!(matches!(r, Err(CutError::Incomplete { .. })), "{r:?}");
    }
}

#[test]
fn builder_dags_are_monotonic_and_violations_are_caught() {
    // Builder errors are monotonic; merged spheres can miss containment by f32
    // rounding only, which the repair closes without touching errors.
    for (raw, clusters) in raw_fixtures() {
        let mut groups = raw.clone();
        let report = cut::enforce_monotonic(&mut groups, &clusters).unwrap();
        assert_eq!(report.errors_raised, 0);
        for (a, b) in raw.iter().zip(&groups) {
            assert!(b.radius <= a.radius * (1. + 1e-5) + 1e-6, "{a:?} -> {b:?}");
        }
        assert_eq!(cut::check_monotonic(&groups, &clusters), Ok(()));
        let mut same = groups.clone();
        assert_eq!(
            cut::enforce_monotonic(&mut same, &clusters).unwrap(),
            cut::MonotonicReport::default()
        );
        assert_eq!(format!("{same:?}"), format!("{groups:?}"));
    }
    // A build that lets errors shrink between levels is rejected, then repaired.
    let (mut groups, clusters) = build(grid(32), 0.5);
    assert!(matches!(
        cut::check_monotonic(&groups, &clusters),
        Err(Violation::ErrorDecreases { .. } | Violation::NotContained { .. })
    ));
    let report = cut::enforce_monotonic(&mut groups, &clusters).unwrap();
    assert!(report.groups_changed > 0, "{report:?}");
    assert_eq!(cut::check_monotonic(&groups, &clusters), Ok(()));
    // One ulp below containment or error is caught; equal spheres pass.
    let make = |g: u32, refined: i32| {
        let mut group = FlatDagGroup::default();
        (
            group.first_cluster,
            group.cluster_count,
            group.radius,
            group.error,
        ) = (g, 1, 1., 1.);
        let mut cluster = FlatDagCluster::default();
        (cluster.group, cluster.refined) = (g, refined);
        (group, cluster)
    };
    let ((q, qc), (mut p, pc)) = (make(0, -1), make(1, 0));
    let tables = |p: FlatDagGroup| (vec![q, p], vec![qc, pc]);
    let (g, c) = tables(p);
    assert_eq!(cut::check_monotonic(&g, &c), Ok(()));
    p.radius = p.radius.next_down();
    let (g, c) = tables(p);
    assert_eq!(
        cut::check_monotonic(&g, &c),
        Err(Violation::NotContained {
            child: 0,
            parent: 1
        })
    );
    p.radius = 1.;
    p.error = p.error.next_down();
    let (g, c) = tables(p);
    assert_eq!(
        cut::check_monotonic(&g, &c),
        Err(Violation::ErrorDecreases {
            child: 0,
            parent: 1
        })
    );
    let mut bad = c.clone();
    bad[1].refined = 1;
    assert_eq!(cut::check_monotonic(&g, &bad), Err(Violation::InvalidTable));
}

#[test]
fn fixed_error_cut_is_unique_and_shrinks_with_error() {
    for (groups, clusters) in fixtures() {
        let cover = leaves(&groups, &clusters);
        let mut out = vec![0; clusters.len()];
        let mut last = usize::MAX;
        for error in [0., 0.01, 0.05, 0.2, 1., 4., f32::MAX] {
            let n = cut::select_error_cut(&groups, &clusters, error, &mut out).unwrap();
            assert_unique_cover(&cover, &clusters, &out[..n]);
            assert!(n <= last);
            last = n;
        }
        let roots = groups
            .iter()
            .filter(|g| g.error == f32::MAX)
            .map(|g| g.cluster_count as usize)
            .sum::<usize>();
        assert_eq!(last, roots);
    }
}

#[test]
fn projection_formula_and_scale_bound() {
    let b = LodBounds {
        center: [0., 0., 10.],
        radius: 2.,
        error: 0.5,
    };
    let persp = CutView {
        position: [0.; 3],
        projection: Projection::Perspective {
            proj_y: 2.,
            znear: 0.1,
        },
        viewport_height: 1000.,
        scale: 1.,
    };
    let near = |a: f32, b: f32| assert!((a - b).abs() <= 1e-3, "{a} vs {b}");
    near(cut::projected_error(b, &persp), 62.5);
    let scaled = CutView { scale: 2., ..persp };
    near(cut::projected_error(b, &scaled), 1000. / 6.);
    let ortho = CutView {
        projection: Projection::Orthographic {
            pixels_per_unit: 8.,
        },
        ..scaled
    };
    near(cut::projected_error(b, &ortho), 8.);
    let inside = CutView {
        position: [0., 0., 9.],
        ..persp
    };
    near(cut::projected_error(b, &inside), 5000.);

    assert_eq!(
        cut::max_scale([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]),
        1.
    );
    assert_eq!(
        cut::max_scale([[3., 0., 0.], [0., 2., 0.], [0., 0., 1.]]),
        3.
    );
    let shear = cut::max_scale([[1., 1., 0.], [0., 1., 0.], [0., 0., 1.]]);
    assert!((1.618_034..=1.75).contains(&shear), "{shear}");
    let (s, c) = (0.6f32, 0.8f32);
    let turn = cut::max_scale([[c, -s, 0.], [s, c, 0.], [0., 0., 1.]]);
    assert!((1.0..=1.000_001).contains(&turn), "{turn}");
    assert_eq!(
        cut::max_scale([[f32::NAN, 0., 0.], [0., 1., 0.], [0., 0., 1.]]),
        f32::INFINITY
    );
}

#[test]
fn ancestor_clamp_handles_error_inversions_and_dag_joins() {
    // Q has two parents P1/P2. Only P2 exceeds the threshold, so Q must
    // remain stopped even though its own error exceeds the threshold.
    let mut groups: Vec<_> = [3., 1., 3., f32::MAX]
        .into_iter()
        .enumerate()
        .map(|(i, error)| {
            let mut g = FlatDagGroup::default();
            (g.first_cluster, g.cluster_count, g.radius, g.error) = (i as u32, 1, 1., error);
            g
        })
        .collect();
    let clusters: Vec<_> = [-1, 0, 0, 1]
        .into_iter()
        .enumerate()
        .map(|(i, refined)| {
            let mut c = FlatDagCluster::default();
            (c.group, c.refined) = (i as u32, refined);
            c
        })
        .collect();
    let view = CutView {
        position: [0.; 3],
        projection: Projection::Orthographic {
            pixels_per_unit: 1.,
        },
        viewport_height: 1.,
        scale: 1.,
    };
    let (mut raw, mut direct, mut sequential) = ([0; 4], [0; 4], [0; 4]);
    let n = cut::select_error_cut(&groups, &clusters, 2., &mut raw).unwrap();
    let k = cut::select_cut(
        &groups,
        &clusters,
        &view,
        Threshold::Pixels(2.),
        None,
        &mut direct,
    )
    .unwrap();
    let j = cut::refine_sequential(
        &groups,
        &clusters,
        &view,
        Threshold::Pixels(2.),
        None,
        &mut sequential,
    )
    .unwrap();
    assert_eq!(&raw[..n], &[2, 3]);
    assert_eq!(raw[..n], direct[..k]);
    assert_eq!(raw[..n], sequential[..j]);
    assert_unique_cover(&leaves(&groups, &clusters), &clusters, &raw[..n]);
    // A parentless group still tests its own error before descendants may
    // refine; its clusters remain the fallback when that test fails.
    groups[1].error = 3.;
    groups[3].error = 1.;
    let n = cut::select_error_cut(&groups, &clusters, 2., &mut raw).unwrap();
    assert_eq!(&raw[..n], &[2, 3]);
    let k = cut::select_cut(
        &groups,
        &clusters,
        &view,
        Threshold::Pixels(2.),
        None,
        &mut direct,
    )
    .unwrap();
    let j = cut::refine_sequential(
        &groups,
        &clusters,
        &view,
        Threshold::Pixels(2.),
        None,
        &mut sequential,
    )
    .unwrap();
    assert_eq!(raw[..n], direct[..k]);
    assert_eq!(raw[..n], sequential[..j]);
    assert_unique_cover(&leaves(&groups, &clusters), &clusters, &raw[..n]);
}
