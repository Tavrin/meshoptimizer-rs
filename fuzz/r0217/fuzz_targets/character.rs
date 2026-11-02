#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let m = mesh(&mut c, 64);
    let nv = m.pos.len();
    let mode = c.u8();
    let nb = 1 + usize::from(c.u8()) % 48;
    let influences = if mode & 1 == 0 { 4 } else { 8 };
    let loose = mode & 2 != 0;
    let mut joints = Vec::new();
    let mut weights = Vec::new();
    for _ in 0..nv {
        let raw: Vec<u8> = (0..influences).map(|_| c.u8()).collect();
        let sum: f32 = raw.iter().map(|&b| f32::from(b)).sum();
        for (k, &b) in raw.iter().enumerate() {
            joints.push(u32::from(c.u8()) % (nb as u32 + u32::from(loose)));
            let w = if sum == 0. {
                f32::from(u8::from(k == 0))
            } else if loose {
                f32::from(b) / 255.
            } else {
                f32::from(b) / sum
            };
            weights.push(w);
        }
    }
    let importance: Vec<f32> = (0..nb).map(|_| c.unit() * 2.).collect();
    let shapes: Vec<Vec<[f32; 3]>> = (0..usize::from(c.u8() % 3))
        .map(|_| (0..nv).map(|_| [0, 1, 2].map(|_| c.unit() - 0.5)).collect())
        .collect();
    let shape_refs: Vec<&[[f32; 3]]> = shapes.iter().map(Vec::as_slice).collect();
    let poses: Vec<Vec<[[f32; 4]; 3]>> = (0..usize::from(c.u8() % 2))
        .map(|_| {
            (0..nb)
                .map(|_| {
                    let mut mtx = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
                    for r in &mut mtx {
                        r[3] = c.unit() - 0.5;
                    }
                    mtx
                })
                .collect()
        })
        .collect();
    let pose_refs: Vec<&[[[f32; 4]; 3]]> = poses.iter().map(Vec::as_slice).collect();
    let mut inputs = CharacterInputs::new();
    if mode & 4 == 0 {
        inputs.skin = Some(CharacterSkin::new(&joints, &weights, influences, &importance));
    }
    inputs.blend_shapes = &shape_refs;
    inputs.joint_poses = &pose_refs;
    let mut opts = CharacterOptions::default();
    opts.skin_weight = c.unit() * 2.;
    opts.blend_shape_weight = c.unit() * 2.;
    opts.best_of = mode & 8 != 0;
    let settings = SimplifySettings {
        target_index_count: m.idx.len() * usize::from(c.u8()) / 255,
        target_error: c.unit(),
        options: SimplifyOptions::from_bits(u32::from(c.u8()) & 0x1ff).unwrap_or(SimplifyOptions::EMPTY),
    };
    let limits = Limits {
        max_bytes: 32 << 20,
        max_work: 3_000_000,
    };
    let p = || Positions::from_packed(&m.pos);

    let mut ws = Workspace::new(limits);
    let a = simplify_character(&m.idx, p(), &inputs, &opts, settings, &mut ws);
    let b = simplify_character(&m.idx, p(), &inputs, &opts, settings, &mut Workspace::new(limits));
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
    if let Ok(s) = &a {
        assert_eq!(s.indices.len() % 3, 0);
        assert!(s.indices.len() <= m.idx.len());
        assert!(s.indices.iter().all(|&i| (i as usize) < nv));
    }

    let ratios = [0.6f32, 0.3];
    let chain_options = LodChainOptions::new(LodTargets::Ratios(&ratios), LodErrors::Uniform(c.unit()));
    let q = LodQualityOptions::default();
    let r = build_character_lod_chain(&m.idx, p(), &inputs, &opts, &chain_options, &q, &mut ws);
    let r2 = build_character_lod_chain(&m.idx, p(), &inputs, &opts, &chain_options, &q, &mut Workspace::new(limits));
    assert_eq!(format!("{r:?}"), format!("{r2:?}"));
    if let Ok(r) = &r {
        for l in &r.chain.levels {
            assert_eq!(l.indices.len() % 3, 0);
            assert_eq!(l.triangle_count * 3, l.indices.len());
            assert!(l.indices.iter().all(|&i| (i as usize) < nv));
        }
        if opts.best_of {
            assert_eq!(r.selections.len(), r.chain.levels.len());
            for s in &r.selections {
                // The winner has the lower worst sampled deviation, so best-of
                // is never worse than plain simplification.
                if s.plain_max.is_finite() && s.character_max.is_finite() {
                    match s.method {
                        CharacterLodMethod::Plain => assert!(s.plain_max <= s.character_max),
                        CharacterLodMethod::Character => assert!(s.character_max <= s.plain_max),
                        _ => {}
                    }
                }
            }
        } else {
            assert!(r.selections.is_empty());
        }
    }
});
