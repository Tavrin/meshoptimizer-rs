#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::*;
fn records(p: &[[f32; 3]]) -> Vec<u8> {
    p.iter()
        .flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes()))
        .collect()
}
fn stitch(i: &[u32], p: &[[f32; 3]], t: f32) -> TopologyRepairGeneric<StitchReport> {
    let data = records(p);
    stitch_cracks(
        i,
        VertexStream::packed(&data, 12).unwrap(),
        Positions::from_packed(p),
        Some(t),
        &mut Workspace::default(),
    )
    .unwrap()
}
#[test]
fn fans_copy_attributes_and_generic_matches() {
    let i = [0, 1, 2, 0, 3, 4];
    let data = [10, 20, 30, 40, 50];
    let stream = VertexStream::packed(&data, 1).unwrap();
    let r = split_non_manifold_vertices(&i, stream, &mut Workspace::default()).unwrap();
    assert_eq!(r.indices, [0, 1, 2, 5, 3, 4]);
    assert_eq!(r.vertices, [10, 20, 30, 40, 50, 10]);
    assert_eq!(r.report.split_vertices, 1);
    assert_eq!(r.report.duplicated_vertices, 1);
    let j = i.map(|x| x as u16);
    let s = split_non_manifold_vertices_generic(&j, stream, &mut Workspace::default()).unwrap();
    assert_eq!(
        s.indices.iter().map(|&x| u32::from(x)).collect::<Vec<_>>(),
        r.indices
    );
    let again = split_non_manifold_vertices(
        &r.indices,
        VertexStream::packed(&r.vertices, 1).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(again.report.duplicated_vertices, 0);
}
#[test]
fn gap_closes_without_flips() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [1.00001, 0., 0.],
        [0.00001, 1., 0.],
        [1., 1., 0.],
    ];
    let r = stitch(&[0, 1, 2, 4, 3, 5], &p, 0.0001);
    assert_eq!(r.report.stitched_pairs, 2);
    assert_eq!(r.indices, [0, 1, 2, 2, 1, 5]);
    assert!((r.report.remaining_boundary_length - 4.).abs() < 1e-6);
}
#[test]
fn t_junction_subdivides_and_shares_edges() {
    let p = [
        [0., 0., 0.],
        [2., 0., 0.],
        [0., 1., 0.],
        [1., 0., 0.],
        [1., -1., 0.],
    ];
    let r = stitch(&[0, 1, 2, 3, 0, 4, 1, 3, 4], &p, 0.);
    assert_eq!(r.report.t_junction_splits, 1);
    assert_eq!(r.indices.len(), 12);
    let a = analyze_mesh(
        &r.indices,
        Positions::from_packed(&p),
        AnalyzeOptions::EMPTY,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(a.non_manifold_edges, 0);
    assert_eq!(a.orientation_conflicts, 0);
}
#[test]
fn refuses_collapsed_faces_and_bad_tolerance() {
    let p = [[0., 0., 0.], [0.01, 0., 0.], [0., 1., 0.]];
    let r = stitch(&[0, 1, 2], &p, 0.1);
    assert_eq!(r.indices, [0, 1, 2]);
    assert!(r.report.refused > 0);
    let data = records(&p);
    assert_eq!(
        stitch_cracks(
            &[0, 1, 2],
            VertexStream::packed(&data, 12).unwrap(),
            Positions::from_packed(&p),
            Some(f32::NAN),
            &mut Workspace::default()
        ),
        Err(Error::InvalidParameter)
    );
}
#[test]
fn bounded_and_deterministic() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let data = records(&p);
    let stream = VertexStream::packed(&data, 12).unwrap();
    let run = |ws: &mut Workspace| {
        stitch_cracks(&[0, 1, 2], stream, Positions::from_packed(&p), None, ws)
    };
    let mut ws = Workspace::default();
    let expected = run(&mut ws).unwrap();
    let usage = ws.usage();
    for max_work in [0, usage.work - 1, usage.work] {
        let r = run(&mut Workspace::new(Limits {
            max_work,
            ..Limits::default()
        }));
        if max_work < usage.work {
            assert_eq!(r, Err(Error::LimitExceeded));
        } else {
            assert_eq!(r.unwrap(), expected);
        }
    }
    for max_bytes in [0, usage.bytes - 1, usage.bytes] {
        let r = run(&mut Workspace::new(Limits {
            max_bytes,
            ..Limits::default()
        }));
        if max_bytes < usage.bytes {
            assert_eq!(r, Err(Error::LimitExceeded));
        } else {
            assert_eq!(r.unwrap(), expected);
        }
    }
}
#[test]
fn stitch_preserves_split_bowtie() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
        [0., -1., 0.],
        [0., 0., 0.],
    ];
    let r = stitch(&[0, 1, 2, 5, 3, 4], &p, 0.);
    assert_eq!(r.report.stitched_pairs, 0);
    assert!(r.report.refused > 0);
}
#[test]
fn shared_edges_stay_shared_across_more_than_two_faces() {
    let data = [0u8; 5];
    let r = split_non_manifold_vertices(
        &[0, 1, 2, 1, 0, 3, 0, 1, 4],
        VertexStream::packed(&data, 1).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(r.indices, [0, 1, 2, 1, 0, 3, 0, 1, 4]);
    assert_eq!(r.report.duplicated_vertices, 0);
}
#[test]
fn existing_defects_do_not_block_an_unrelated_stitch() {
    // The first two triangles are the crack from `gap_closes_without_flips`;
    // the third pair is a bow tie that stays untouched.
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [1.00001, 0., 0.],
        [0.00001, 1., 0.],
        [1., 1., 0.],
        [10., 0., 0.],
        [11., 0., 0.],
        [10., 1., 0.],
        [9., 0., 0.],
        [10., -1., 0.],
    ];
    let r = stitch(&[0, 1, 2, 4, 3, 5, 6, 7, 8, 6, 9, 10], &p, 0.0001);
    assert_eq!(r.report.stitched_pairs, 2);
    assert_eq!(&r.indices[..6], [0, 1, 2, 2, 1, 5]);
    assert_eq!(&r.indices[6..], [6, 7, 8, 6, 9, 10]);
}
#[test]
fn pipeline_runs_options_and_copies_attribute_bytes() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
        [0., -1., 0.],
    ];
    let data = records(&p);
    let mut o = ProcessOptions::default();
    o.weld = false;
    o.sanitize = None;
    o.lod = None;
    o.cache = false;
    o.overdraw = None;
    o.fetch = false;
    o.split_non_manifold_vertices = true;
    o.stitch_cracks = Some(None);
    let r = process_mesh(
        &[0, 1, 2, 0, 3, 4],
        &data,
        12,
        0,
        &o,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(r.levels[0].indices, [0, 1, 2, 5, 3, 4]);
    assert_eq!(&r.levels[0].vertices[..data.len()], data);
    assert_eq!(&r.levels[0].vertices[data.len()..], &data[..12]);
}
#[test]
fn u16_appended_vertex_overflow_is_checked() {
    let data = vec![0u8; 65536];
    let stream = VertexStream::packed(&data, 1).unwrap();
    assert_eq!(
        split_non_manifold_vertices_generic(
            &[0u16, 1, 2, 0, 3, 4],
            stream,
            &mut Workspace::default()
        ),
        Err(Error::IndexOverflow)
    );
}
#[test]
fn stitch_generic_matches_u32() {
    let p = [
        [0., 0., 0.],
        [2., 0., 0.],
        [0., 1., 0.],
        [1., 0., 0.],
        [1., -1., 0.],
    ];
    let i = [0u16, 1, 2, 3, 0, 4, 1, 3, 4];
    let data = records(&p);
    let r = stitch_cracks_generic(
        &i,
        VertexStream::packed(&data, 12).unwrap(),
        Positions::from_packed(&p),
        Some(0.),
        &mut Workspace::default(),
    )
    .unwrap();
    let expected = stitch(&i.map(u32::from), &p, 0.);
    assert_eq!(
        r.indices.iter().copied().map(u32::from).collect::<Vec<_>>(),
        expected.indices
    );
    assert_eq!(r.vertices, expected.vertices);
    assert_eq!(r.report, expected.report);
}
