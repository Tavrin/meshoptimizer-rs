use meshoptimizer_rs::*;
fn settings() -> SimplifySettings {
    SimplifySettings {
        target_index_count: 3,
        target_error: 100.,
        options: SimplifyOptions::EMPTY,
    }
}
const P: [[f32; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
const I: [u32; 6] = [0, 1, 2, 2, 1, 3];
#[test]
fn empty_matches_plain_and_chain_reports() {
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&P);
    let a = simplify(&I, p, settings(), &mut ws).unwrap();
    let b = simplify_character(
        &I,
        p,
        &CharacterInputs::new(),
        &CharacterOptions::default(),
        settings(),
        &mut ws,
    )
    .unwrap();
    assert_eq!(a.indices, b.indices);
    assert_eq!(a.error.to_bits(), b.error.to_bits());
    let co = LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(100.));
    let c = build_character_lod_chain(
        &I,
        p,
        &CharacterInputs::new(),
        &CharacterOptions::default(),
        &co,
        &LodQualityOptions::default(),
        &mut ws,
    )
    .unwrap();
    assert_eq!(c.quality.len(), 1);
    assert_eq!(c.quality[0].len(), 1);
}
#[test]
fn validates_skin_shapes_and_limits() {
    let joints = [0; 32];
    let mut weights = [0.; 32];
    for i in 0..4 {
        weights[i * 8] = 1.;
    }
    let importance = [1.];
    let mut d = CharacterInputs::new();
    d.skin = Some(CharacterSkin::new(&joints, &weights, 8, &importance));
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&P);
    let o = CharacterOptions::default();
    assert!(simplify_character(&I, p, &d, &o, settings(), &mut ws).is_ok());
    let bad = [f32::NAN; 32];
    d.skin = Some(CharacterSkin::new(&joints, &bad, 8, &importance));
    assert_eq!(
        simplify_character(&I, p, &d, &o, settings(), &mut ws).unwrap_err(),
        Error::InvalidParameter
    );
    d.skin = None;
    let shape = [[0., 0., 1.]; 3];
    let shapes = [shape.as_slice()];
    d.blend_shapes = &shapes;
    assert_eq!(
        simplify_character(&I, p, &d, &o, settings(), &mut ws).unwrap_err(),
        Error::InvalidLayout
    );
    let importance = [1.; 33];
    d.blend_shapes = &[];
    d.skin = Some(CharacterSkin::new(&joints, &weights, 8, &importance));
    assert_eq!(
        simplify_character(&I, p, &d, &o, settings(), &mut ws).unwrap_err(),
        Error::InvalidLayout
    );
    let mut ws = Workspace::new(Limits {
        max_bytes: 0,
        max_work: 0,
    });
    assert_eq!(
        simplify_character(&I, p, &CharacterInputs::new(), &o, settings(), &mut ws).unwrap_err(),
        Error::LimitExceeded
    );
}
#[test]
fn reports_full_morph_and_linear_skin_pose() {
    let joints = [0; 16];
    let weights = [
        1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0.,
    ];
    let importance = [2.];
    let shape = [[0., 0., 1.]; 4];
    let shapes = [shape.as_slice()];
    let matrix = [[[1., 0., 0., 3.], [0., 1., 0., 0.], [0., 0., 1., 0.]]];
    let poses = [matrix.as_slice()];
    let mut d = CharacterInputs::new();
    d.skin = Some(CharacterSkin::new(&joints, &weights, 4, &importance));
    d.blend_shapes = &shapes;
    d.joint_poses = &poses;
    let co = LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(100.));
    let mut ws = Workspace::default();
    let a = build_character_lod_chain(
        &I,
        Positions::from_packed(&P),
        &d,
        &CharacterOptions::default(),
        &co,
        &LodQualityOptions::default(),
        &mut ws,
    )
    .unwrap();
    assert_eq!(a.quality.len(), 3);
    for pose in &a.quality {
        assert!((pose[0].symmetric.max - a.quality[0][0].symmetric.max).abs() < 1e-10);
    }
    let usage = ws.usage();
    let mut limited = Workspace::new(Limits {
        max_bytes: usize::MAX,
        max_work: usage.work - 1,
    });
    assert_eq!(
        build_character_lod_chain(
            &I,
            Positions::from_packed(&P),
            &d,
            &CharacterOptions::default(),
            &co,
            &LodQualityOptions::default(),
            &mut limited
        )
        .unwrap_err(),
        Error::LimitExceeded
    );
}
#[test]
fn channels_match_exact_bone_identity_importance_and_full_target_metric() {
    let joints = [0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0];
    let weights = [
        0.8, 0.2, 0., 0., 0.7, 0.3, 0., 0., 0.9, 0.1, 0., 0., 0.6, 0.4, 0., 0.,
    ];
    let importance = [2., 3.];
    let shape = [[0., 0., 0.], [0., 0., 0.], [0., 0., 2.], [0., 0., 1.]];
    let shapes = [shape.as_slice()];
    let mut d = CharacterInputs::new();
    d.skin = Some(CharacterSkin::new(&joints, &weights, 4, &importance));
    d.blend_shapes = &shapes;
    let mut attrs = Vec::new();
    for i in 0..4 {
        let mut row = [0.; 5];
        for k in 0..4 {
            row[joints[i * 4 + k] as usize] +=
                weights[i * 4 + k] * importance[joints[i * 4 + k] as usize];
        }
        for k in 0..3 {
            row[2 + k] = P[i][k] + shape[i][k];
        }
        attrs.extend_from_slice(&row);
    }
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&P);
    let a =
        simplify_character(&I, p, &d, &CharacterOptions::default(), settings(), &mut ws).unwrap();
    let b = simplify_with_attributes(
        &I,
        p,
        Attributes::from_interleaved(&attrs, 4, 5, 5, 0).unwrap(),
        &[1.; 5],
        None,
        settings(),
        &mut ws,
    )
    .unwrap();
    assert_eq!(a.indices, b.indices);
    assert_eq!(a.error.to_bits(), b.error.to_bits());
    let usage = ws.usage();
    assert!(usage.work > 0);
}
