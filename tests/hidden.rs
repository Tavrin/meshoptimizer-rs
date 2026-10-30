//! Hand-authored test geometry is CC0-1.0.
use meshoptimizer_rs::*;

fn cube(p: &mut Vec<[f32; 3]>, i: &mut Vec<u32>, lo: f32, hi: f32) {
    let base = u32::try_from(p.len()).unwrap();
    p.extend([
        [lo, lo, lo],
        [hi, lo, lo],
        [hi, hi, lo],
        [lo, hi, lo],
        [lo, lo, hi],
        [hi, lo, hi],
        [hi, hi, hi],
        [lo, hi, hi],
    ]);
    i.extend(
        [
            0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7,
            6, 3, 0, 4, 3, 4, 7,
        ]
        .map(|v| v + base),
    );
}
fn run(p: &[[f32; 3]], i: &[u32], o: HiddenOptions<'_>) -> VisibleMesh {
    remove_hidden_triangles(i, Positions::from_packed(p), o, &mut Workspace::default()).unwrap()
}
#[test]
fn inner_cube_removed_outer_cube_kept() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let m = run(&p, &i, HiddenOptions::default());
    assert_eq!(m.indices, i[..36]);
    assert_eq!(m.report.occluded, 12);
}
#[test]
fn opening_exposes_inside_from_outside() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    // An independent interior surface is hidden when the lid is present.
    p.extend([[-0.5, -0.5, 0.], [0.5, -0.5, 0.], [0., 0.5, 0.]]);
    i.extend([8, 9, 10]);
    assert_eq!(run(&p, &i, HiddenOptions::default()).report.occluded, 1);
    i.drain(6..12); // The interior surface can now be seen through the top.
    let m = run(&p, &i, HiddenOptions::default());
    assert_eq!(m.indices, i);
}
#[test]
fn resting_box_from_accessible_viewpoints_only_removes_fully_covered_plane() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, 0., 2.);
    // Coplanar bottom square and partly covered triangle on the support plane.
    p.extend([
        [0., 0., 0.],
        [2., 0., 0.],
        [2., 2., 0.],
        [0., 2., 0.],
        [-2., 0., 0.],
    ]);
    i.extend([8, 9, 10, 8, 10, 11, 12, 9, 10]);
    let mut o = HiddenOptions::default();
    let points = [[1., 1., 5.]];
    o.views = HiddenViews::Points(&points);
    let m = run(&p, &i, o);
    assert!(!m
        .indices
        .as_chunks::<3>()
        .0
        .iter()
        .any(|t| *t == [0, 2, 1] || *t == [0, 3, 2] || *t == [8, 9, 10] || *t == [8, 10, 11]));
    assert!(m.indices.as_chunks::<3>().0.contains(&[12, 9, 10]));
}
#[test]
fn exposed_coplanar_back_to_back_walls_do_not_mutually_occlude() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [-0., 0., 0.]];
    let i = [0, 1, 2, 1, 2, 3, 0, 2, 1];
    let m = run(&p, &i, HiddenOptions::default());
    assert_eq!(m.indices, [0, 1, 2, 0, 2, 1]);
    assert_eq!(m.report.duplicate, 1);
    assert_eq!(m.report.enclosed_back_to_back, 0);
}
#[test]
fn enclosed_opposite_faces_have_separate_reason() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    p.extend([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]);
    i.extend([8, 9, 10, 8, 10, 9]);
    let m = run(&p, &i, HiddenOptions::default());
    assert_eq!(m.report.enclosed_back_to_back, 2);
    assert_eq!(m.indices, i[..36]);
}
#[test]
fn any_visible_vertex_keeps_triangle_even_with_hidden_centroid() {
    let p = [
        [-3., 0., 0.],
        [3., 0., 0.],
        [0., 3., 0.],
        [-1., -1., 1.],
        [1., -1., 1.],
        [0., 2., 1.],
    ];
    let mut o = HiddenOptions::default();
    let points = [[0., 0., 10.]];
    o.views = HiddenViews::Points(&points);
    let m = run(&p, &[0, 1, 2, 3, 4, 5], o);
    assert_eq!(m.indices, [0, 1, 2, 3, 4, 5]);
}
#[test]
fn tolerance_only_retains_more_and_view_endpoint_is_not_an_occluder() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let mut o = HiddenOptions::default();
    o.tolerance = 10.;
    assert_eq!(run(&p, &i, o).indices, i);
    let points = [[0., 0., 0.]];
    o.views = HiddenViews::Points(&points);
    o.tolerance = 0.;
    // An interior viewpoint can see the inner shell.
    assert!(run(&p, &i, o)
        .indices
        .as_chunks::<3>()
        .0
        .iter()
        .any(|t| t[0] >= 8));
}
#[test]
fn bounds_views_are_exactly_corners_and_center() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let mut o = HiddenOptions::default();
    o.views = HiddenViews::Bounds {
        min: [-5.; 3],
        max: [5.; 3],
    };
    let a = run(&p, &i, o);
    let mut v = vec![];
    for j in 0..8 {
        v.push(core::array::from_fn(|k| {
            if j & (1 << k) == 0 {
                -5.
            } else {
                5.
            }
        }));
    }
    v.push([0.; 3]);
    o.views = HiddenViews::Points(&v);
    assert_eq!(a, run(&p, &i, o));
}
#[test]
fn deterministic_warm_after_failure_and_nonpacked() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let i = [0, 1, 2, 1, 2, 0];
    let bytes: Vec<u8> = p
        .iter()
        .flat_map(|v| [99_f32, v[0], v[1], v[2]])
        .flat_map(f32::to_le_bytes)
        .collect();
    let view = Positions::from_bytes(&bytes, 3, 16, 4, ByteOrder::LittleEndian).unwrap();
    let expected = run(&p, &i, HiddenOptions::default());
    let mut ws = Workspace::default();
    for _ in 0..3 {
        assert_eq!(
            remove_hidden_triangles(&i, view, HiddenOptions::default(), &mut ws).unwrap(),
            expected
        );
        let mut out = [];
        assert_eq!(
            remove_hidden_triangles_into(&mut out, &i, view, HiddenOptions::default(), &mut ws),
            Err(Error::BufferTooSmall)
        );
    }
}
#[test]
fn invalid_input_and_options_are_checked() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let mut ws = Workspace::default();
    let mut call =
        |i: &[u32], o| remove_hidden_triangles(i, Positions::from_packed(&p), o, &mut ws);
    assert_eq!(
        call(&[0, 1], HiddenOptions::default()),
        Err(Error::InvalidTopology)
    );
    assert_eq!(
        call(&[0, 1, u32::MAX], HiddenOptions::default()),
        Err(Error::IndexOutOfBounds)
    );
    for subdivisions in [0, 65, u32::MAX] {
        let mut o = HiddenOptions::default();
        o.subdivisions = subdivisions;
        assert_eq!(call(&[], o), Err(Error::InvalidParameter));
    }
    for tolerance in [-1., f64::INFINITY, f64::NAN] {
        let mut o = HiddenOptions::default();
        o.tolerance = tolerance;
        assert_eq!(call(&[], o), Err(Error::InvalidParameter));
    }
    for views in [
        HiddenViews::Points(&[]),
        HiddenViews::Points(&[[f32::NAN, 0., 0.]]),
        HiddenViews::Bounds {
            min: [1.; 3],
            max: [0.; 3],
        },
    ] {
        let mut o = HiddenOptions::default();
        o.views = views;
        assert_eq!(call(&[], o), Err(Error::InvalidParameter));
    }
    let bad = [[f32::INFINITY, 0., 0.]];
    assert_eq!(
        remove_hidden_triangles(
            &[0, 0, 0],
            Positions::from_packed(&bad),
            HiddenOptions::default(),
            &mut ws
        ),
        Err(Error::NumericalFailure)
    );
}
#[test]
fn empty_and_degenerate_extreme_finite_geometry_is_retained() {
    assert!(run(&[], &[], HiddenOptions::default()).indices.is_empty());
    for value in [f32::MAX, f32::MIN_POSITIVE, f32::from_bits(1)] {
        let p = [[value, 0., 0.], [0., value, 0.], [0., 0., value]];
        assert_eq!(
            run(&p, &[0, 1, 2, 0, 0, 0], HiddenOptions::default()).indices,
            [0, 1, 2, 0, 0, 0]
        );
    }
}
#[test]
fn every_work_limit_and_byte_boundaries_preserve_destination_and_recover() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let i = [0, 1, 2, 1, 2, 0, 0, 2, 1];
    let mut o = HiddenOptions::default();
    o.pixel_size = 4.0; // One pixel: sweep every work limit of the raster path.
    for into in [false, true] {
        let call = |ws: &mut Workspace, out: &mut [u32]| {
            if into {
                remove_hidden_triangles_into(out, &i, Positions::from_packed(&p), o, ws)
            } else {
                remove_hidden_triangles(&i, Positions::from_packed(&p), o, ws).map(|m| m.report)
            }
        };
        let mut ws = Workspace::default();
        let mut out = [99; 12];
        let expected = call(&mut ws, &mut out).unwrap();
        let usage = ws.usage();
        for limit in 0..=usage.work + 1 {
            let mut ws = Workspace::new(Limits {
                max_work: limit,
                ..Limits::default()
            });
            for _ in 0..2 {
                let mut out = [99; 12];
                let result = call(&mut ws, &mut out);
                if limit < usage.work {
                    assert_eq!(result, Err(Error::LimitExceeded));
                    assert_eq!(out, [99; 12]);
                } else {
                    assert_eq!(result, Ok(expected));
                    if into {
                        assert_eq!(&out[6..], &[99; 6]);
                    }
                }
            }
        }
        for bytes in [0, usage.bytes - 1, usage.bytes, usage.bytes + 1] {
            let mut ws = Workspace::new(Limits {
                max_bytes: bytes,
                ..Limits::default()
            });
            let mut out = [99; 12];
            let result = call(&mut ws, &mut out);
            if bytes < usage.bytes {
                assert_eq!(result, Err(Error::LimitExceeded));
                assert_eq!(out, [99; 12]);
            } else {
                assert_eq!(result, Ok(expected));
            }
        }
    }
}
#[test]
fn raster_work_scales_on_separated_triangles_n_vs_8n() {
    let measure = |n: usize| {
        let mut p = vec![];
        let mut i = vec![];
        for j in 0..n {
            let x = u32::try_from(j).unwrap() as f32 * 3.;
            let b = u32::try_from(p.len()).unwrap();
            p.extend([[x, 0., 0.], [x + 1., 0., 0.], [x, 1., 0.]]);
            i.extend([b, b + 1, b + 2]);
        }
        let mut ws = Workspace::default();
        assert_eq!(
            remove_hidden_triangles(
                &i,
                Positions::from_packed(&p),
                HiddenOptions::default(),
                &mut ws
            )
            .unwrap()
            .indices,
            i
        );
        ws.usage().work
    };
    let a = measure(128);
    let b = measure(1024);
    println!(
        "HSR scaling work N={a} 8N={b} ratio={}",
        b as f64 / a as f64
    );
    assert!(b <= 10 * a);
}

#[test]
fn retained_cache_and_failure_do_not_change_visibility() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let expected = run(&p, &i, HiddenOptions::default());
    let mut ws = Workspace::default();
    optimize_vertex_cache(&i, p.len(), &mut ws).unwrap();
    let mut small = [99; 3];
    assert_eq!(
        remove_hidden_triangles_into(
            &mut small,
            &i,
            Positions::from_packed(&p),
            HiddenOptions::default(),
            &mut ws
        ),
        Err(Error::BufferTooSmall)
    );
    assert_eq!(small, [99; 3]);
    for _ in 0..2 {
        assert_eq!(
            remove_hidden_triangles(
                &i,
                Positions::from_packed(&p),
                HiddenOptions::default(),
                &mut ws
            )
            .unwrap(),
            expected
        );
    }
}
#[test]
fn inclusive_sampling_density_limits_and_zero_volume_views() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    for subdivisions in [1, 3, 64] {
        let mut o = HiddenOptions::default();
        o.subdivisions = subdivisions;
        o.views = HiddenViews::Bounds {
            min: [0.; 3],
            max: [0.; 3],
        };
        assert_eq!(run(&p, &[0, 1, 2], o).indices, [0, 1, 2]);
    }
}

#[test]
fn raster_pixel_options_and_failure_are_atomic() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    for pixel_size in [-1., f64::NAN, f64::INFINITY, 1e-30] {
        let mut o = HiddenOptions::default();
        o.pixel_size = pixel_size;
        let mut out = [99; 6];
        assert_eq!(
            remove_hidden_triangles_into(
                &mut out,
                &[0, 1, 2],
                Positions::from_packed(&p),
                o,
                &mut Workspace::default()
            ),
            Err(Error::InvalidParameter)
        );
        assert_eq!(out, [99; 6]);
    }
    for pixel_size in [1e30, 2.0 / 2048.0] {
        let mut o = HiddenOptions::default();
        o.pixel_size = pixel_size;
        assert_eq!(run(&p, &[0, 1, 2], o).indices, [0, 1, 2]);
    }
}

#[test]
fn raster_preserves_visible_slivers_and_subpixel_triangles() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    // An aperture much narrower than one pixel exposes an interior triangle.
    // Shrink the +Z lid away from its diagonal, keeping its other vertices.
    let b = u32::try_from(p.len()).unwrap();
    p.extend([
        [-2., -2., 2.],
        [2., -2., 2.],
        [2., 1.999, 2.],
        [-2., -1.999, 2.],
        [2., 2., 2.],
        [-2., 2., 2.],
        [-0.0001, -0.0001, 0.],
        [0.0001, -0.0001, 0.],
        [0., 0.0001, 0.],
    ]);
    i[6..12].copy_from_slice(&[b, b + 1, b + 2, b + 3, b + 4, b + 5]);
    i.extend([b + 6, b + 7, b + 8]);
    let mut o = HiddenOptions::default();
    o.pixel_size = 0.125;
    assert!(run(&p, &i, o).indices.ends_with(&[b + 6, b + 7, b + 8]));
}

#[cfg(feature = "parallel")]
#[test]
fn raster_is_identical_in_one_two_and_four_thread_pools() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let expected = run(&p, &i, HiddenOptions::default());
    for n in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build()
            .unwrap();
        assert_eq!(
            pool.install(|| run(&p, &i, HiddenOptions::default())),
            expected
        );
    }
}

#[test]
fn original_sample_rounding_seam_can_only_restore_triangles() {
    let p = [
        [0.0, 1.5, 1.0],
        [-0.25, 2.0, 1.5],
        [-0.25, -0.5, 0.75],
        [-0.5, 1.0, -0.25],
        [0.75, 0.25, 2.0],
        [1.5, -1.25, 0.0],
        [0.25, 0.75, 0.25],
        [1.75, -0.75, -1.75],
        [-1.5, 2.0, 1.75],
        [-2.0, 1.25, -1.75],
        [-1.0, -1.5, 0.0],
        [0.75, -0.75, 1.0],
        [1.5, -0.5, 0.0],
        [0.75, -1.75, -0.75],
        [0.25, 0.5, -0.75],
        [-4.0, -4.0, -4.0],
        [4.0, -4.0, -4.0],
        [-4.0, 4.0, -4.0],
        [4.0, 4.0, -4.0],
        [-4.0, -4.0, 4.0],
        [4.0, -4.0, 4.0],
        [-4.0, 4.0, 4.0],
        [4.0, 4.0, 4.0],
    ];
    let i = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 18, 16, 15, 17, 18, 19, 20, 22, 19,
        22, 21, 15, 16, 20, 15, 20, 19, 16, 18, 22, 16, 22, 20, 18, 17, 21, 18, 21, 22, 17, 15, 19,
        17, 19, 21,
    ];
    assert!(run(&p, &i, HiddenOptions::default())
        .indices
        .as_chunks::<3>()
        .0
        .contains(&[3, 4, 5]));
}

#[test]
fn hidden_candidate_and_finite_view_limits_preserve_destination_and_recover() {
    let (mut p, mut i) = (vec![], vec![]);
    cube(&mut p, &mut i, -2., 2.);
    cube(&mut p, &mut i, -1., 1.);
    let cameras = [[0., 0., 5.]];
    for views in [HiddenViews::Outside, HiddenViews::Points(&cameras)] {
        let mut o = HiddenOptions::default();
        o.views = views;
        let mut ws = Workspace::default();
        let mut expected = vec![99; i.len() + 3];
        let report =
            remove_hidden_triangles_into(&mut expected, &i, Positions::from_packed(&p), o, &mut ws)
                .unwrap();
        assert!(report.removed() > 0);
        let usage = ws.usage();
        let limits = [
            Limits {
                max_work: usage.work / 2,
                ..Limits::default()
            },
            Limits {
                max_work: usage.work - 1,
                ..Limits::default()
            },
            Limits {
                max_bytes: usage.bytes - 1,
                ..Limits::default()
            },
        ];
        for limits in limits {
            let mut ws = Workspace::new(limits);
            let mut out = vec![99; expected.len()];
            assert_eq!(
                remove_hidden_triangles_into(&mut out, &i, Positions::from_packed(&p), o, &mut ws,),
                Err(Error::LimitExceeded)
            );
            assert!(out.iter().all(|&v| v == 99));
            ws.set_limits(Limits::default());
            assert_eq!(
                remove_hidden_triangles_into(&mut out, &i, Positions::from_packed(&p), o, &mut ws,),
                Ok(report)
            );
            assert_eq!(out, expected);
        }
    }
}

#[test]
fn extreme_coordinates_and_density_requests_are_bounded_and_atomic() {
    let p = [
        [-f32::MAX, -f32::MAX, -f32::MAX],
        [f32::MAX, -f32::MAX, f32::MAX],
        [-f32::MAX, f32::MAX, f32::MAX],
    ];
    let i = [0, 1, 2];
    let side = 2.0 * f64::from(f32::MAX);
    let mut ws = Workspace::default();
    // Exercise resolution 1 and the exact maximum, on full-range f32 geometry.
    for pixel_size in [f64::MAX, side * 2.0 / 2048.0] {
        let mut o = HiddenOptions::default();
        o.pixel_size = pixel_size;
        assert_eq!(run(&p, &i, o).indices, i);
    }
    // Both an infinite requested resolution and just-over-limit sizes fail
    // before raster allocation, preserving the caller's destination.
    for pixel_size in [f64::from_bits(1), side * 2.0 / 2049.0] {
        let mut o = HiddenOptions::default();
        o.pixel_size = pixel_size;
        let mut out = [99; 6];
        assert_eq!(
            remove_hidden_triangles_into(&mut out, &i, Positions::from_packed(&p), o, &mut ws),
            Err(Error::InvalidParameter)
        );
        assert_eq!(out, [99; 6]);
        assert_eq!(
            remove_hidden_triangles(
                &i,
                Positions::from_packed(&p),
                HiddenOptions::default(),
                &mut ws
            )
            .unwrap()
            .indices,
            i
        );
    }
}
