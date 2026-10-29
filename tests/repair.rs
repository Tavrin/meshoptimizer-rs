#![cfg(feature = "repair")]
use meshoptimizer_rs::*;

const QUAD: [[f32; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]];
const PATCH: [u32; 6] = [0, 1, 2, 0, 3, 2];
fn cube() -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = vec![
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        [1., 0., 1.],
        [1., 1., 1.],
        [0., 1., 1.],
    ];
    // Outward cube, missing its top quad.
    let i = vec![
        0, 2, 1, 0, 3, 2, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 0, 4, 3, 4, 7,
    ];
    (p, i)
}
fn weld(p: &[[f32; 3]], t: f32) -> WeldedVertices {
    weld_vertices(
        Positions::from_packed(p),
        t,
        WeldPolicy::PositionsOnly,
        &mut Workspace::default(),
    )
    .unwrap()
}
#[test]
fn weld_chooses_lowest_eligible_representative_without_chains() {
    let p = [[0., 0., 0.], [0.75, 0., 0.], [1.5, 0., 0.], [1., 0., 0.]];
    let r = weld(&p, 1.);
    assert_eq!(r.remap, [0, 0, 2, 0]);
    assert_eq!(
        (
            r.report.input_vertices,
            r.report.representatives,
            r.report.merged_vertices
        ),
        (4, 2, 2)
    );
}
#[test]
fn weld_attribute_equality_preserves_seams() {
    let p = [[0., 0., 0.], [0.001, 0., 0.], [0., 0., 0.], [0.001, 0., 0.]];
    let attrs = [0, 1, 1, 0];
    let r = weld_vertices(
        Positions::from_packed(&p),
        0.01,
        WeldPolicy::AttributeEquality(&|a, b| attrs[a as usize] == attrs[b as usize]),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(r.remap, [0, 1, 1, 0]);
}
#[test]
fn weld_crack_then_sanitize_and_analyze() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [0.001, 1., 0.],
        [1.001, 0., 0.],
        [1., 1., 0.],
    ];
    let r = weld(&p, 0.01);
    assert_eq!(r.remap, [0, 1, 2, 2, 1, 5]);
    let i: Vec<_> = [0, 1, 2, 3, 4, 5, 1, 4, 5].map(|v| r.remap[v]).to_vec();
    let p = Positions::from_packed(&p);
    let clean = sanitize(&i, p, SanitizeOptions::EMPTY, &mut Workspace::default()).unwrap();
    assert_eq!(clean.report.repeated_index, 1);
    let a = analyze_mesh(
        &clean.indices,
        p,
        AnalyzeOptions::EMPTY,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!((a.boundary_edges, a.orientation_conflicts), (4, 0));
}
#[test]
fn weld_zero_matches_position_remap_and_extreme_scales() {
    for p in [
        vec![],
        vec![[0., -0., 0.], [-0., 0., 0.], [1., 1., 1.], [1., 1., 1.]],
        vec![[f32::MAX, 0., 0.], [f32::MAX, 0., 0.], [-f32::MAX, 0., 0.]],
        vec![[f32::from_bits(1), 0., 0.], [0., 0., 0.]],
    ] {
        let expected =
            generate_position_remap(Positions::from_packed(&p), &mut Workspace::default()).unwrap();
        assert_eq!(weld(&p, 0.).remap, expected);
        let tiny = weld(&p, f32::from_bits(1));
        for (i, &j) in tiny.remap.iter().enumerate() {
            let d: f64 = (0..3)
                .map(|a| (f64::from(p[i][a]) - f64::from(p[j as usize][a])).powi(2))
                .sum();
            assert!(d <= f64::from(f32::from_bits(1)).powi(2));
        }
    }
}
#[test]
fn weld_grid_matches_exhaustive_greedy_oracle() {
    let mut seed = 42u32;
    let mut p = Vec::new();
    for _ in 0..160 {
        let mut point = [0.; 3];
        for v in &mut point {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            *v = ((seed >> 16) % 13) as f32 * 0.125 - 0.75;
        }
        p.push(point);
    }
    for t in [0., f32::from_bits(1), 0.125, 0.25, 1., f32::MAX] {
        let mut expected = Vec::new();
        for i in 0..p.len() {
            let r = (0..i).find(|&j| {
                expected[j] == j as u32
                    && (0..3)
                        .map(|k| (f64::from(p[i][k]) - f64::from(p[j][k])).powi(2))
                        .sum::<f64>()
                        <= f64::from(t).powi(2)
            });
            expected.push(r.unwrap_or(i) as u32);
        }
        assert_eq!(weld(&p, t).remap, expected);
    }
}
#[test]
fn weld_rejects_invalid_tolerance_and_all_nonfinite_positions() {
    for t in [-1., f32::NAN, f32::INFINITY] {
        assert_eq!(
            weld_vertices(
                Positions::from_packed(&QUAD),
                t,
                WeldPolicy::PositionsOnly,
                &mut Workspace::default()
            ),
            Err(Error::InvalidParameter)
        );
    }
    assert_eq!(
        weld_vertices(
            Positions::from_packed(&[[f32::NAN, 0., 0.]]),
            0.,
            WeldPolicy::PositionsOnly,
            &mut Workspace::default()
        ),
        Err(Error::NumericalFailure)
    );
}
#[test]
fn orient_flipped_patch_keeps_lowest_face_seed_and_component_order() {
    let r = orient_faces(&PATCH, 4, &mut Workspace::default()).unwrap();
    assert_eq!(r.indices, [0, 1, 2, 0, 2, 3]);
    assert_eq!(r.report.flipped_faces, [1]);
    assert_eq!(r.report.components, 1);
    assert!(r.report.conflicts.is_empty());
    let other = [0, 3, 2, 0, 1, 2, 4, 5, 6];
    let r = orient_faces(&other, 7, &mut Workspace::default()).unwrap();
    assert_eq!(&r.indices[..3], &other[..3]);
    assert_eq!(r.report.components, 2);
}
#[test]
fn orient_never_propagates_across_nonmanifold_edges() {
    let i = [0, 1, 2, 0, 1, 3, 0, 1, 4];
    let r = orient_faces(&i, 5, &mut Workspace::default()).unwrap();
    assert_eq!(r.indices, i);
    assert_eq!(r.report.components, 3);
    assert_eq!(r.report.ambiguous_components, [0, 1, 2]);
}
#[test]
fn orient_reports_mobius_strip_contradiction() {
    // Three quads in a strip, final pair glued with a half twist.
    let i = [0, 1, 2, 2, 1, 3, 2, 3, 4, 4, 3, 5, 4, 5, 1, 1, 5, 0];
    let r = orient_faces(&i, 6, &mut Workspace::default()).unwrap();
    assert_eq!(r.report.components, 1);
    assert!(!r.report.conflicts.is_empty());
    assert_eq!(r.report.ambiguous_components, [0]);
}
#[test]
fn fill_cube_missing_face_closes_boundary_with_original_vertices() {
    let (p, i) = cube();
    let r = fill_holes(&i, Positions::from_packed(&p), 4, &mut Workspace::default()).unwrap();
    assert_eq!(&r.indices[..i.len()], &i);
    assert_eq!(r.report.filled.len(), 1);
    assert_eq!(
        (r.report.filled[0].vertex, r.report.filled[0].edges),
        (4, 4)
    );
    assert_eq!(r.indices.len(), 36);
    assert!(r.indices.iter().all(|&v| v < p.len() as u32));
    let a = analyze_mesh(
        &r.indices,
        Positions::from_packed(&p),
        AnalyzeOptions::EMPTY,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(
        (
            a.boundary_edges,
            a.non_manifold_edges,
            a.orientation_conflicts
        ),
        (0, 0, 0)
    );
}
#[test]
fn fill_edge_limit_is_inclusive_and_reports_oversize() {
    let (p, i) = cube();
    for max in [0, 2, 3] {
        let r = fill_holes(
            &i,
            Positions::from_packed(&p),
            max,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(r.indices, i);
        assert_eq!(r.report.too_large.len(), 1);
    }
    assert_eq!(fill_holes_bound(6), Ok(24));
    assert_eq!(fill_holes_bound(usize::MAX), Err(Error::SizeOverflow));
}
// A disk triangulated around a lifted center; only the outer polygon is boundary.
fn disk(polygon: &[[f32; 3]]) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = polygon.to_vec();
    p.push([0., 0., 2.]);
    let mut i = Vec::new();
    for a in 0..polygon.len() {
        i.extend_from_slice(&[
            a as u32,
            ((a + 1) % polygon.len()) as u32,
            polygon.len() as u32,
        ]);
    }
    (p, i)
}
#[test]
fn fill_concave_loop_uses_ears_and_is_deterministic() {
    let (p, i) = disk(&[
        [0., 0., 0.],
        [2., 0., 0.],
        [2., 2., 0.],
        [1., 0.5, 0.],
        [0., 2., 0.],
    ]);
    let r = fill_holes(&i, Positions::from_packed(&p), 5, &mut Workspace::default()).unwrap();
    assert_eq!(r.report.filled.len(), 1);
    assert_eq!(r.indices.len(), i.len() + 9);
    assert_eq!(
        r,
        fill_holes(&i, Positions::from_packed(&p), 5, &mut Workspace::default()).unwrap()
    );
    assert!(r.indices[i.len()..].iter().all(|&v| v < 5));
}
#[test]
fn fill_refuses_crossing_coincident_and_degenerate_loops() {
    for polygon in [
        vec![[0., 0., 0.], [2., 2., 0.], [0., 2., 0.], [2., 0., 0.]],
        vec![[0., 0., 0.], [1., 0., 0.], [0., 0., 0.], [0., 1., 0.]],
        vec![[0., 0., 0.], [1., 0., 0.], [2., 0., 0.]],
        // Nonzero signed area, but one nonadjacent edge touches another.
        vec![
            [0., 0., 0.],
            [2., 0., 0.],
            [2., 2., 0.],
            [1., 0., 0.],
            [0., 2., 0.],
        ],
    ] {
        let (p, i) = disk(&polygon);
        let r = fill_holes(
            &i,
            Positions::from_packed(&p),
            10,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(r.indices, i);
        assert_eq!(r.report.non_simple.len(), 1);
    }
}
#[test]
fn fill_refuses_branched_inconsistently_directed_and_nonplanar_boundaries() {
    let mut p = QUAD.to_vec();
    p.extend_from_slice(&[[2., 0., 0.], [2., 1., 0.]]);
    for i in [
        vec![0, 1, 2, 0, 4, 5],
        PATCH.to_vec(),
        vec![0, 1, 2, 0, 1, 3, 0, 1, 4],
    ] {
        let r = fill_holes(
            &i,
            Positions::from_packed(&p),
            20,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(r.indices, i);
        assert!(!r.report.ambiguous.is_empty());
    }
    let (p, i) = disk(&[[0., 0., 0.], [1., 0., 0.], [1., 1., 0.25], [0., 1., 0.]]);
    let r = fill_holes(&i, Positions::from_packed(&p), 4, &mut Workspace::default()).unwrap();
    assert_eq!(r.indices, i);
    assert_eq!(r.report.ambiguous.len(), 1);
}
#[test]
fn topology_errors_and_nonfinite_references_are_explicit() {
    for (i, e) in [
        (vec![0], Error::InvalidTopology),
        (vec![0, 0, 1], Error::InvalidTopology),
        (vec![0, 1, 9], Error::IndexOutOfBounds),
    ] {
        assert_eq!(orient_faces(&i, 4, &mut Workspace::default()), Err(e));
        assert_eq!(
            fill_holes(
                &i,
                Positions::from_packed(&QUAD),
                4,
                &mut Workspace::default()
            ),
            Err(e)
        );
    }
    let p = [[0., 0., 0.], [1., 0., 0.], [0., f32::INFINITY, 0.]];
    assert_eq!(
        fill_holes(
            &[0, 1, 2],
            Positions::from_packed(&p),
            3,
            &mut Workspace::default()
        ),
        Err(Error::NumericalFailure)
    );
}
#[test]
fn caller_buffers_match_owned_forms_and_preserve_tails() {
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&QUAD);
    let mut out = [99; 30];
    let r = weld_vertices_into(&mut out, p, 0., WeldPolicy::PositionsOnly, &mut ws).unwrap();
    assert_eq!(&out[..4], &weld(&QUAD, 0.).remap);
    assert_eq!(r, weld(&QUAD, 0.).report);
    assert_eq!(&out[4..], &[99; 26]);
    out.fill(99);
    let r = orient_faces_into(&mut out, &PATCH, 4, &mut ws).unwrap();
    let expected = orient_faces(&PATCH, 4, &mut ws).unwrap();
    assert_eq!(&out[..6], &expected.indices);
    assert_eq!(r, expected.report);
    assert_eq!(&out[6..], &[99; 24]);
    out.fill(99);
    let i = [0, 1, 2, 0, 2, 3];
    let r = fill_holes_into(&mut out, &i, p, 4, &mut ws).unwrap();
    let expected = fill_holes(&i, p, 4, &mut ws).unwrap();
    assert_eq!(&out[..12], &expected.indices);
    assert_eq!(r, expected.report);
    assert_eq!(&out[12..], &[99; 18]);
    for api in 0..3 {
        let mut out = [99; 2];
        let r = match api {
            0 => {
                weld_vertices_into(&mut out, p, 0., WeldPolicy::PositionsOnly, &mut ws).map(|_| ())
            }
            1 => orient_faces_into(&mut out, &PATCH, 4, &mut ws).map(|_| ()),
            _ => fill_holes_into(&mut out, &i, p, 4, &mut ws).map(|_| ()),
        };
        assert_eq!(r, Err(Error::BufferTooSmall));
        assert_eq!(out, [99; 2]);
    }
}
fn into(api: usize, out: &mut [u32], ws: &mut Workspace) -> Result<(), Error> {
    let p = Positions::from_packed(&QUAD);
    match api {
        0 => weld_vertices_into(out, p, 0.1, WeldPolicy::PositionsOnly, ws).map(|_| ()),
        1 => orient_faces_into(out, &PATCH, 4, ws).map(|_| ()),
        _ => fill_holes_into(out, &[0, 1, 2, 0, 2, 3], p, 4, ws).map(|_| ()),
    }
}
#[test]
fn every_work_limit_and_byte_boundary_is_atomic_cold_warm_after_failure() {
    for api in 0..3 {
        let mut ws = Workspace::default();
        let mut expected = [77; 32];
        into(api, &mut expected, &mut ws).unwrap();
        let usage = ws.usage();
        for max_work in 0..=usage.work + 1 {
            let mut ws = Workspace::new(Limits {
                max_work,
                ..Limits::default()
            });
            for _ in 0..3 {
                let mut out = [77; 32];
                let result = into(api, &mut out, &mut ws);
                assert_eq!(
                    result.is_ok(),
                    max_work >= usage.work,
                    "api={api} work={max_work}"
                );
                if result.is_err() {
                    assert_eq!(out, [77; 32]);
                } else {
                    assert_eq!(out, expected);
                }
                assert!(ws.usage().work <= max_work);
            }
        }
        for max_bytes in [0, usage.bytes - 1, usage.bytes, usage.bytes + 1] {
            let mut ws = Workspace::new(Limits {
                max_bytes,
                ..Limits::default()
            });
            for _ in 0..3 {
                let mut out = [77; 32];
                let result = into(api, &mut out, &mut ws);
                assert_eq!(
                    result.is_ok(),
                    max_bytes >= usage.bytes,
                    "api={api} bytes={max_bytes}"
                );
                if result.is_err() {
                    assert_eq!(out, [77; 32]);
                } else {
                    assert_eq!(out, expected);
                }
            }
        }
        // A nonempty retained cache workspace and a prior buffer failure.
        let mut ws = Workspace::default();
        optimize_vertex_cache(&[0, 1, 2, 0, 2, 3], 4, &mut ws).unwrap();
        assert_eq!(into(api, &mut [], &mut ws), Err(Error::BufferTooSmall));
        let mut out = [77; 32];
        into(api, &mut out, &mut ws).unwrap();
        assert_eq!(out, expected);
    }
}
#[test]
fn corpus_properties_consistent_orientation_zero_weld_and_no_new_vertices() {
    let mut corpus = vec![(QUAD.to_vec(), vec![0, 1, 2, 0, 2, 3]), cube()];
    corpus.push(disk(&[[0., 0., 0.], [2., 0., 0.], [1., 1., 0.]]));
    for (p, i) in corpus {
        let pos = Positions::from_packed(&p);
        let mut ws = Workspace::default();
        assert_eq!(
            weld(&p, 0.).remap,
            generate_position_remap(pos, &mut ws).unwrap()
        );
        let oriented = orient_faces(&i, p.len(), &mut ws).unwrap();
        assert_eq!(oriented.indices, i);
        assert!(oriented.report.flipped_faces.is_empty());
        let filled = fill_holes(&i, pos, 8, &mut ws).unwrap();
        assert!(filled.indices.iter().all(|&v| v < p.len() as u32));
    }
}

#[test]
fn fill_refuses_boundary_touching_disconnected_closed_fan() {
    // An open triangle and a closed tetrahedron share only vertex zero.
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
        [0., -1., 0.],
        [0., 0., -1.],
    ];
    let i = [0, 1, 2, 0, 3, 4, 0, 5, 3, 0, 4, 5, 3, 5, 4];
    let r = fill_holes(&i, Positions::from_packed(&p), 3, &mut Workspace::default()).unwrap();
    assert_eq!(r.indices, i);
    assert_eq!(r.report.ambiguous.len(), 1);
}

#[test]
fn owned_apis_match_success_at_every_work_limit_and_byte_boundary() {
    for api in 0..3 {
        let run = |ws: &mut Workspace| -> Result<String, Error> {
            let p = Positions::from_packed(&QUAD);
            match api {
                0 => weld_vertices(p, 0.1, WeldPolicy::PositionsOnly, ws).map(|r| format!("{r:?}")),
                1 => orient_faces(&PATCH, 4, ws).map(|r| format!("{r:?}")),
                _ => fill_holes(&[0, 1, 2, 0, 2, 3], p, 4, ws).map(|r| format!("{r:?}")),
            }
        };
        let mut ws = Workspace::default();
        let expected = run(&mut ws).unwrap();
        let usage = ws.usage();
        for max_work in 0..=usage.work + 1 {
            let mut ws = Workspace::new(Limits {
                max_work,
                ..Limits::default()
            });
            for _ in 0..3 {
                let result = run(&mut ws);
                if max_work < usage.work {
                    assert_eq!(result, Err(Error::LimitExceeded));
                } else {
                    assert_eq!(result.as_ref().unwrap(), &expected);
                }
                assert!(ws.usage().work <= max_work);
            }
        }
        for max_bytes in [0, usage.bytes - 1, usage.bytes, usage.bytes + 1] {
            let mut ws = Workspace::new(Limits {
                max_bytes,
                ..Limits::default()
            });
            for _ in 0..3 {
                let result = run(&mut ws);
                if max_bytes < usage.bytes {
                    assert_eq!(result, Err(Error::LimitExceeded));
                } else {
                    assert_eq!(result.as_ref().unwrap(), &expected);
                }
            }
        }
    }
}

#[test]
fn welding_uses_euclidean_distance_inclusive_boundary_and_nonpacked_positions() {
    let p = [[0., 0., 0.], [0.6, 0.8, 0.], [0.9, 0.9, 0.], [-1., 0., 0.]];
    // The 0.6/0.8 f32 pair is slightly outside radius one in f64.
    let r = weld(&p, 1.);
    assert_eq!(r.remap, [0, 1, 1, 0]);
    let packed: Vec<f32> = p.iter().flat_map(|v| [7., v[0], v[1], v[2], 8.]).collect();
    let view = Positions::from_interleaved(&packed, p.len(), 5, 1).unwrap();
    assert_eq!(
        weld_vertices(
            view,
            1.,
            WeldPolicy::PositionsOnly,
            &mut Workspace::default()
        )
        .unwrap(),
        r
    );
    let p = [[-1., 0., 0.], [0., 0., 0.], [1., 0., 0.]];
    assert_eq!(weld(&p, 1.).remap, [0, 0, 2]);
}

#[test]
fn orientation_rejects_unrepresentable_vertex_count_without_allocating() {
    if usize::BITS > 32 {
        let mut out = [17; 3];
        let mut ws = Workspace::default();
        assert_eq!(
            orient_faces(&[], usize::MAX, &mut ws),
            Err(Error::SizeOverflow)
        );
        assert_eq!(
            orient_faces_into(&mut out, &[], usize::MAX, &mut ws),
            Err(Error::SizeOverflow)
        );
        assert_eq!(out, [17; 3]);
    }
}
