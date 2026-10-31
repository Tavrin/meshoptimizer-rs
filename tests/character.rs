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
    assert!(simplify_character(&I, p, &d, &o, settings(), &mut ws).is_ok());
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

#[test]
fn wide_bone_channels_preserve_identity_and_public_attribute_limit() {
    let weights = [1., 0., 0., 0.].repeat(4);
    let compact_joints = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
    let wide_joints = [0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0];
    let compact_importance = [1.; 2];
    let wide_importance = [1.; 65];
    let mut compact = CharacterInputs::new();
    compact.skin = Some(CharacterSkin::new(
        &compact_joints,
        &weights,
        4,
        &compact_importance,
    ));
    let mut wide = CharacterInputs::new();
    wide.skin = Some(CharacterSkin::new(
        &wide_joints,
        &weights,
        4,
        &wide_importance,
    ));
    let p = Positions::from_packed(&P);
    let a = simplify_character(
        &I,
        p,
        &compact,
        &CharacterOptions::default(),
        settings(),
        &mut Workspace::default(),
    )
    .unwrap();
    let mut ws = Workspace::default();
    let b = simplify_character(
        &I,
        p,
        &wide,
        &CharacterOptions::default(),
        settings(),
        &mut ws,
    )
    .unwrap();
    assert_eq!(a.indices, b.indices);
    assert_eq!(a.error.to_bits(), b.error.to_bits());
    let work = ws.usage().work;
    let mut exact = Workspace::new(Limits {
        max_work: work,
        ..Limits::default()
    });
    assert!(simplify_character(
        &I,
        p,
        &wide,
        &CharacterOptions::default(),
        settings(),
        &mut exact
    )
    .is_ok());
    let mut short = Workspace::new(Limits {
        max_work: work - 1,
        ..Limits::default()
    });
    assert_eq!(
        simplify_character(
            &I,
            p,
            &wide,
            &CharacterOptions::default(),
            settings(),
            &mut short
        )
        .unwrap_err(),
        Error::LimitExceeded
    );
    let mut no_bytes = Workspace::new(Limits {
        max_bytes: 0,
        ..Limits::default()
    });
    assert_eq!(
        simplify_character(
            &I,
            p,
            &wide,
            &CharacterOptions::default(),
            settings(),
            &mut no_bytes
        )
        .unwrap_err(),
        Error::LimitExceeded
    );
    assert_eq!(
        Attributes::from_interleaved(&[0.; 132], 4, 33, 33, 0).unwrap_err(),
        Error::InvalidLayout
    );
}

#[test]
fn wide_morph_chain_and_empty_wide_validation_are_metered() {
    let shapes: Vec<_> = (0..92)
        .map(|i| P.map(|p| [p[0] * (i as f32) * 0.001, 0., p[1] * 0.01]))
        .collect();
    let shape_refs: Vec<_> = shapes.iter().map(|s| s.as_slice()).collect();
    let mut data = CharacterInputs::new();
    data.blend_shapes = &shape_refs;
    let p = Positions::from_packed(&P);
    let mesh = simplify_character(
        &I,
        p,
        &data,
        &CharacterOptions::default(),
        settings(),
        &mut Workspace::default(),
    )
    .unwrap();
    let co = LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(100.));
    let result = build_character_lod_chain(
        &I,
        p,
        &data,
        &CharacterOptions::default(),
        &co,
        &LodQualityOptions::default(),
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(result.chain.levels[0].indices, mesh.indices);
    assert_eq!(result.quality.len(), 93);
    assert!(result.quality.iter().all(|q| q.len() == 1));
    let importance = [1.; 65];
    let mut empty = CharacterInputs::new();
    empty.skin = Some(CharacterSkin::new(&[], &[], 4, &importance));
    let mut ws = Workspace::new(Limits {
        max_work: 64,
        ..Limits::default()
    });
    assert_eq!(
        simplify_character(
            &[],
            Positions::from_packed(&[]),
            &empty,
            &CharacterOptions::default(),
            SimplifySettings {
                target_index_count: 0,
                ..settings()
            },
            &mut ws
        )
        .unwrap_err(),
        Error::LimitExceeded
    );
}
