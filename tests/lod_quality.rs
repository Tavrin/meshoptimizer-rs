use meshoptimizer_rs::*;

fn mesh<'a>(ix: &'a [u32], p: &'a [[f32; 3]]) -> LodQualityMesh<'a> {
    LodQualityMesh::new(ix, Positions::from_packed(p))
}
fn measure(a: (&[u32], &[[f32; 3]]), b: (&[u32], &[[f32; 3]])) -> LodQuality {
    measure_lod_quality(
        mesh(a.0, a.1),
        mesh(b.0, b.1),
        &LodQualityOptions::default(),
        &mut Workspace::default(),
    )
    .unwrap()
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-7, "{a} != {b}");
}
const IX: [u32; 3] = [0, 1, 2];
const P: [[f32; 3]; 3] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];

#[test]
fn parallel_planes_have_exact_max_and_rms() {
    let q = P.map(|p| [p[0], p[1], 2.]);
    let r = measure((&IX, &P), (&IX, &q));
    for d in [r.source_to_level, r.level_to_source, r.symmetric] {
        close(d.max, 2.);
        close(d.rms, 2.);
    }
    assert_eq!(r.symmetric.samples, 8);
}
#[test]
fn bump_against_plane_has_known_height() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [0.5, 0.5, 0.25],
    ];
    let a = [0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4];
    let b = [0, 1, 2, 2, 3, 0];
    let r = measure((&a, &p), (&b, &p));
    close(r.source_to_level.max, 0.25);
    close(r.symmetric.max, 0.25);
    assert!(r.symmetric.rms > 0. && r.symmetric.rms < 0.25);
}
fn octahedron() -> (Vec<u32>, Vec<[f32; 3]>) {
    (
        vec![
            0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 4, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3, 5,
        ],
        vec![
            [1., 0., 0.],
            [-1., 0., 0.],
            [0., 1., 0.],
            [0., -1., 0.],
            [0., 0., 1.],
            [0., 0., -1.],
        ],
    )
}
#[test]
fn sphere_tessellations_have_known_face_sagitta() {
    let (ix, mut p) = octahedron();
    let mut fine = Vec::new();
    for t in ix.as_chunks::<3>().0 {
        let v = core::array::from_fn::<_, 3, _>(|k| {
            (p[t[0] as usize][k] + p[t[1] as usize][k] + p[t[2] as usize][k]) / 3.0f32.sqrt()
        });
        let center = u32::try_from(p.len()).unwrap();
        p.push(v);
        fine.extend_from_slice(&[t[0], t[1], center, t[1], t[2], center, t[2], t[0], center]);
    }
    let r = measure((&fine, &p), (&ix, &p));
    close(r.source_to_level.max, 1. - 1. / 3.0f64.sqrt());
    close(r.symmetric.max, 1. - 1. / 3.0f64.sqrt());
    let half = p.iter().map(|p| p.map(|x| x * 0.5)).collect::<Vec<_>>();
    close(measure((&ix, &p), (&ix, &half)).symmetric.max, 0.5);
}
#[test]
fn cube_and_decimated_open_cube_find_missing_face_interior() {
    let p = [
        [-1., -1., -1.],
        [1., -1., -1.],
        [1., 1., -1.],
        [-1., 1., -1.],
        [-1., -1., 1.],
        [1., -1., 1.],
        [1., 1., 1.],
        [-1., 1., 1.],
        [0., 0., 1.],
    ];
    let coarse = [
        0, 2, 1, 0, 3, 2, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 0, 4, 3, 4, 7,
    ];
    let mut full = coarse.to_vec();
    full.extend_from_slice(&[4, 5, 8, 5, 6, 8, 6, 7, 8, 7, 4, 8]);
    let r = measure((&full, &p), (&coarse, &p));
    close(r.symmetric.max, 1.);
    close(r.level_to_source.max, 0.);
}
#[test]
fn barycentric_sampling_detects_missing_interior_without_extra_vertices() {
    let p = [[0., 0., 0.], [2., 0., 0.], [0., 2., 0.]];
    let boundary = [0, 1, 1, 1, 2, 2, 2, 0, 0];
    let mut options = LodQualityOptions::default();
    options.samples_per_unit_area = 0.;
    let r = measure_lod_quality(
        mesh(&IX, &p),
        mesh(&boundary, &p),
        &options,
        &mut Workspace::default(),
    )
    .unwrap();
    close(r.symmetric.max, 0.);
    options.samples_per_unit_area = 32.;
    let r = measure_lod_quality(
        mesh(&IX, &p),
        mesh(&boundary, &p),
        &options,
        &mut Workspace::default(),
    )
    .unwrap();
    assert!(r.symmetric.max > 0.4);
}
#[test]
fn supplied_normals_compare_normalized_interpolants() {
    let a = [[0., 0., 2.]; 3];
    let b = [[0., 3., 0.]; 3];
    let mut source = mesh(&IX, &P);
    source.normals = Some(&a);
    let mut level = mesh(&IX, &P);
    level.normals = Some(&b);
    let r = measure_lod_quality(
        source,
        level,
        &LodQualityOptions::default(),
        &mut Workspace::default(),
    )
    .unwrap();
    let n = r.normals.unwrap();
    close(n.max, core::f64::consts::FRAC_PI_2);
    close(n.mean, n.max);
    let b = [[0., 0., -1.]; 3];
    level.normals = Some(&b);
    close(
        measure_lod_quality(
            source,
            level,
            &LodQualityOptions::default(),
            &mut Workspace::default(),
        )
        .unwrap()
        .normals
        .unwrap()
        .max,
        core::f64::consts::PI,
    );
}
#[test]
fn deterministic_ties_use_original_triangle_for_normals() {
    let p = [P[0], P[1], P[2], P[0], P[1], P[2]];
    let normals = [
        [0., 0., 1.],
        [0., 0., 1.],
        [0., 0., 1.],
        [0., 1., 0.],
        [0., 1., 0.],
        [0., 1., 0.],
    ];
    let mut source = mesh(&IX, &p);
    source.normals = Some(&normals);
    let ix = [0, 1, 2, 3, 4, 5];
    let mut target = mesh(&ix, &p);
    target.normals = Some(&normals);
    let r = measure_lod_quality(
        source,
        target,
        &LodQualityOptions::default(),
        &mut Workspace::default(),
    )
    .unwrap();
    close(r.normals.unwrap().mean, core::f64::consts::FRAC_PI_2 / 3.);
}
#[test]
fn degenerates_and_extreme_finite_coordinates_are_supported() {
    let a = [[0., 0., 0.]; 3];
    let b = [[2., 0., 0.]; 3];
    let r = measure((&IX, &a), (&IX, &b));
    close(r.symmetric.max, 2.);
    assert_eq!(r.symmetric.samples, 6);
    let a = [[f32::MAX, 0., 0.]; 3];
    let b = [[-f32::MAX, 0., 0.]; 3];
    let r = measure((&IX, &a), (&IX, &b));
    assert!(r.symmetric.max.is_finite());
    close(r.symmetric.max / f64::from(f32::MAX), 2.);
}
#[test]
fn invalid_inputs_return_typed_errors() {
    let options = LodQualityOptions::default();
    let mut w = Workspace::default();
    let call = |m, w: &mut Workspace| measure_lod_quality(m, mesh(&IX, &P), &options, w);
    assert_eq!(call(mesh(&[0], &P), &mut w), Err(Error::InvalidTopology));
    assert_eq!(call(mesh(&[], &P), &mut w), Err(Error::InvalidParameter));
    assert_eq!(
        call(mesh(&[0, 1, u32::MAX], &P), &mut w),
        Err(Error::IndexOutOfBounds)
    );
    let mut p = P;
    p[0][0] = f32::NAN;
    assert_eq!(call(mesh(&IX, &p), &mut w), Err(Error::NumericalFailure));
    let normals = [[0.; 3]; 3];
    let mut a = mesh(&IX, &P);
    a.normals = Some(&normals);
    let mut b = a;
    assert_eq!(
        measure_lod_quality(a, b, &options, &mut w),
        Err(Error::InvalidParameter)
    );
    let normals = [[f32::INFINITY, 0., 0.]; 3];
    a.normals = Some(&normals);
    b.normals = Some(&normals);
    assert_eq!(
        measure_lod_quality(a, b, &options, &mut w),
        Err(Error::NumericalFailure)
    );
    assert_eq!(call(a, &mut w), Err(Error::InvalidParameter));
    for d in [-1., f64::NAN, f64::INFINITY] {
        let mut o = options;
        o.samples_per_unit_area = d;
        assert_eq!(
            measure_lod_quality(mesh(&IX, &P), mesh(&IX, &P), &o, &mut w),
            Err(Error::InvalidParameter)
        );
    }
    let mut o = options;
    o.samples_per_unit_area = f64::MAX;
    assert_eq!(
        measure_lod_quality(mesh(&IX, &P), mesh(&IX, &P), &o, &mut w),
        Err(Error::SizeOverflow)
    );
}
#[test]
fn camera_distance_matches_pixel_model_and_rejects_invalid_parameters() {
    let camera = LodCamera::new(core::f64::consts::FRAC_PI_2, 1080., 2.).unwrap();
    close(camera.switch_distance(1.).unwrap(), 270.);
    close(camera.switch_distance(0.).unwrap(), 0.);
    for x in [0., -1., f64::NAN, f64::INFINITY] {
        assert!(LodCamera::new(1., x, 1.).is_err());
        assert!(LodCamera::new(1., 1., x).is_err());
        assert!(LodCamera::new(x, 1., 1.).is_err());
    }
    assert!(LodCamera::new(core::f64::consts::PI, 1., 1.).is_err());
    assert_eq!(camera.switch_distance(-1.), Err(Error::InvalidParameter));
}
#[test]
fn chain_measures_each_level_against_source_under_one_budget() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let ix = [0, 1, 2, 2, 1, 3];
    let chain = build_lod_chain(
        &ix,
        Positions::from_packed(&p),
        &LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(1.)),
        &mut Workspace::default(),
    )
    .unwrap();
    assert!(!chain.levels[0].indices.is_empty());
    let camera = LodCamera::new(1., 1080., 1.).unwrap();
    let result = measure_lod_chain_quality(
        mesh(&ix, &p),
        &chain,
        &LodQualityOptions::default(),
        Some(camera),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0].quality,
        measure((&ix, &p), (&chain.levels[0].indices, &p))
    );
    assert_eq!(
        result[0].switch_distance,
        Some(
            camera
                .switch_distance(result[0].quality.symmetric.max)
                .unwrap()
        )
    );
}
#[test]
fn every_work_limit_and_byte_boundary_is_deterministic_after_failure() {
    let opts = LodQualityOptions::default();
    let m = mesh(&IX, &P);
    let mut w = Workspace::default();
    let expected = measure_lod_quality(m, m, &opts, &mut w).unwrap();
    let usage = w.usage();
    for limit in 0..=usage.work + 1 {
        let limits = Limits {
            max_work: limit,
            max_bytes: usize::MAX,
        };
        let mut w = Workspace::new(limits);
        for _ in 0..3 {
            let r = measure_lod_quality(m, m, &opts, &mut w);
            assert_eq!(
                r,
                if limit < usage.work {
                    Err(Error::LimitExceeded)
                } else {
                    Ok(expected)
                }
            );
            assert!(w.usage().work <= limit);
            assert_eq!(w.limits(), limits);
        }
    }
    for bytes in [0, usage.bytes - 1, usage.bytes, usage.bytes + 1] {
        let mut w = Workspace::new(Limits {
            max_bytes: bytes,
            max_work: u64::MAX,
        });
        for _ in 0..2 {
            assert_eq!(
                measure_lod_quality(m, m, &opts, &mut w),
                if bytes < usage.bytes {
                    Err(Error::LimitExceeded)
                } else {
                    Ok(expected)
                }
            );
        }
    }
    w.set_limits(Limits {
        max_bytes: usize::MAX,
        max_work: 1,
    });
    assert!(measure_lod_quality(m, m, &opts, &mut w).is_err());
    w.set_limits(Limits::default());
    assert_eq!(measure_lod_quality(m, m, &opts, &mut w), Ok(expected));
    for _ in 0..5 {
        assert_eq!(measure_lod_quality(m, m, &opts, &mut w), Ok(expected));
        assert_eq!(w.usage(), usage);
    }
}
fn grid(n: u32) -> (Vec<u32>, Vec<[f32; 3]>) {
    let mut p = Vec::new();
    let mut ix = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            p.push([x as f32, y as f32, 0.]);
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            ix.extend_from_slice(&[a, a + 1, a + n + 1, a + 1, a + n + 2, a + n + 1]);
        }
    }
    (ix, p)
}
#[test]
fn eight_times_triangles_scales_near_n_log_n() {
    let mut work = Vec::new();
    // 256 vs 2048 triangles, equal triangle area/sample density.
    for (x, y) in [(16, 8), (32, 32)] {
        let (mut ix, p) = grid(x);
        ix.truncate(usize::try_from(x * y * 6).unwrap());
        let mut w = Workspace::default();
        measure_lod_quality(
            mesh(&ix, &p),
            mesh(&ix, &p),
            &LodQualityOptions::default(),
            &mut w,
        )
        .unwrap();
        work.push(w.usage().work);
    }
    println!(
        "N_vs_8N work={work:?} ratio={}",
        work[1] as f64 / work[0] as f64
    );
    assert!(work[1] < work[0] * 14, "{work:?}");
}

#[test]
fn normal_interpolation_uses_closest_point_barycentrics() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let point = [[0.25, 0.25, 0.]; 3];
    // At (0.25,0.25), the target interpolant is (0.5,0.25,0.25).
    let sn = [[2., 1., 1.]; 3];
    let tn = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let mut source = mesh(&IX, &point);
    source.normals = Some(&sn);
    let mut level = mesh(&IX, &p);
    level.normals = Some(&tn);
    let q = measure_lod_quality(
        source,
        level,
        &LodQualityOptions::default(),
        &mut Workspace::default(),
    )
    .unwrap();
    // Reverse samples at target corners include the exact axis angles to sn.
    close(q.normals.unwrap().max, (1.0 / 6.0f64.sqrt()).acos());
    assert!(q.normals.unwrap().mean < q.normals.unwrap().max);
}

#[test]
fn retained_scratch_and_chain_failure_obey_call_limits() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let ix = [0, 1, 2, 2, 1, 3];
    let m = mesh(&ix, &p);
    let o = LodQualityOptions::default();
    let mut cold = Workspace::default();
    let expected = measure_lod_quality(m, m, &o, &mut cold).unwrap();
    let footprint = cold.usage().bytes;
    let mut warm = Workspace::new(Limits {
        max_bytes: footprint,
        max_work: u64::MAX,
    });
    optimize_vertex_cache(&ix, p.len(), &mut warm).unwrap();
    assert_eq!(measure_lod_quality(m, m, &o, &mut warm), Ok(expected));
    assert!(warm.usage().bytes <= footprint);
    let chain = build_lod_chain(
        &ix,
        Positions::from_packed(&p),
        &LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(1.)),
        &mut Workspace::default(),
    )
    .unwrap();
    let mut w = Workspace::default();
    let result = measure_lod_chain_quality(m, &chain, &o, None, &mut w).unwrap();
    let usage = w.usage();
    for fuel in 0..=usage.work + 1 {
        let mut w = Workspace::new(Limits {
            max_bytes: usize::MAX,
            max_work: fuel,
        });
        for _ in 0..2 {
            assert_eq!(
                measure_lod_chain_quality(m, &chain, &o, None, &mut w),
                if fuel < usage.work {
                    Err(Error::LimitExceeded)
                } else {
                    Ok(result.clone())
                }
            );
            assert!(w.usage().work <= fuel);
        }
    }
    for bytes in [0, usage.bytes - 1, usage.bytes, usage.bytes + 1] {
        let mut w = Workspace::new(Limits {
            max_bytes: bytes,
            max_work: u64::MAX,
        });
        assert_eq!(
            measure_lod_chain_quality(m, &chain, &o, None, &mut w),
            if bytes < usage.bytes {
                Err(Error::LimitExceeded)
            } else {
                Ok(result.clone())
            }
        );
    }
}

#[test]
fn cancelling_interpolated_normals_are_rejected() {
    let point = [[0.5, 0., 0.]; 3];
    let a = [[0., 0., 1.]; 3];
    let b = [[1., 0., 0.], [-1., 0., 0.], [0., 0., 1.]];
    let mut source = mesh(&IX, &point);
    source.normals = Some(&a);
    let mut target = mesh(&IX, &P);
    target.normals = Some(&b);
    assert_eq!(
        measure_lod_quality(
            source,
            target,
            &LodQualityOptions::default(),
            &mut Workspace::default()
        ),
        Err(Error::NumericalFailure)
    );
}
