#![cfg(feature = "clusterlod")]
#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        dag_format::v2 as wire,
        page::QuantGrid,
        v2::{self, AlphaCoverage, BuildMesh},
    },
    *,
};

fn fixture() -> (Vec<[f32; 3]>, Vec<u32>, Vec<f32>) {
    let n = 12;
    let mut p = Vec::new();
    let mut uv = Vec::new();
    let mut idx = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            p.push([x as f32, y as f32, ((x * 17 + y * 13) % 7) as f32 * 0.1]);
            uv.extend([x as f32 / n as f32, y as f32 / n as f32]);
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = (y * (n + 1) + x) as u32;
            let b = a + (n + 1) as u32;
            idx.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    (p, idx, uv)
}
fn config() -> cl::Config {
    let mut c = cl::default_config(32).unwrap();
    c.partition_size = 2;
    c.optimize_clusters = false;
    c
}
fn build(m: BuildMesh<'_>) -> v2::Built {
    v2::build(
        config(),
        m,
        QuantGrid::new(-10).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap()
}
fn bytes(b: &v2::Built, far: Option<wire::FarField>, mats: bool) -> Vec<u8> {
    let mut s = wire::Source::new(&b.groups, &b.clusters, &b.positions, &b.indices);
    if mats {
        s.materials = Some(&b.vertex_material);
    }
    let mut o = wire::WriteOptions::new(QuantGrid::new(-10).unwrap());
    o.far_field = far;
    wire::write(&s, &o, &mut Workspace::default()).unwrap().0
}
fn far() -> wire::FarField {
    let grid = ImpostorGrid::new(ImpostorLayout::Sphere, 3).unwrap();
    let switch = impostor_switch_distance(
        ImpostorBounds::new([0.; 3], 2.).unwrap(),
        grid,
        32,
        LodCamera::new(1.0, 1080., 1.).unwrap(),
    )
    .unwrap();
    wire::FarField::from_switch(17, b"external atlas", grid, 32, switch).unwrap()
}
#[test]
fn alpha_rejections_retain_regions_and_are_deterministic() {
    let (p, i, uv) = fixture();
    let mask_bytes: Vec<u8> = (0..64).map(|i| if i % 3 == 0 { 255 } else { 0 }).collect();
    let masks = [Some(AlphaMask::new(&mask_bytes, 8, 8).unwrap())];
    let attrs = Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap();
    let run = |enabled, unlimited| {
        let mut m = BuildMesh::new(&i, &p);
        m.attributes = Some(attrs);
        m.attribute_weights = &[0., 0.];
        if enabled {
            let mut a = AlphaCoverage::new(&masks, 0);
            a.ratio_band = [1., 1.];
            a.blurred_difference = 0.;
            m.alpha_coverage = Some(a);
        }
        let mut ws = if unlimited {
            Workspace::new(Limits {
                max_bytes: usize::MAX,
                max_work: u64::MAX,
            })
        } else {
            Workspace::default()
        };
        v2::build(config(), m, QuantGrid::new(-10).unwrap(), &mut ws).unwrap()
    };
    let off = run(false, false);
    let on = run(true, false);
    let again = run(true, false);
    assert!(on.alpha_coverage_stuck_groups > 0);
    assert_eq!(on.alpha_coverage_raised_groups, 0);
    assert_eq!(bytes(&on, None, false), bytes(&again, None, false));
    let q = LodQualityOptions::default();
    let off_q = v2::measure_fallback(&off, &q, &mut Workspace::default()).unwrap();
    let on_q = v2::measure_fallback(&on, &q, &mut Workspace::default()).unwrap();
    assert!(on_q.indices.len() >= off_q.indices.len());
    assert!(on_q.quality.symmetric.max <= off_q.quality.symmetric.max);
    #[cfg(feature = "parallel")]
    for workers in [1, 4] {
        let parallel = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .unwrap()
            .install(|| run(true, true));
        assert_eq!(bytes(&on, None, false), bytes(&parallel, None, false));
    }
    // No alpha slots: policy must not disturb otherwise identical geometry.
    let mut opaque = BuildMesh::new(&i, &p);
    opaque.attributes = Some(attrs);
    opaque.attribute_weights = &[0., 0.];
    opaque.alpha_coverage = Some(AlphaCoverage::new(&[], 0));
    assert_eq!(bytes(&off, None, false), bytes(&build(opaque), None, false));
    let transparent = [Some(AlphaMask::new(&[0], 1, 1).unwrap())];
    let mut empty = BuildMesh::new(&i, &p);
    empty.attributes = Some(attrs);
    empty.attribute_weights = &[0., 0.];
    empty.alpha_coverage = Some(AlphaCoverage::new(&transparent, 0));
    let empty = build(empty);
    assert_eq!(empty.alpha_coverage_stuck_groups, 0);
    assert_eq!(bytes(&off, None, false), bytes(&empty, None, false));
    let slots = vec![1u16; i.len() / 3];
    let slot_masks = [None, masks[0]];
    let mut material = BuildMesh::new(&i, &p);
    material.attributes = Some(attrs);
    material.attribute_weights = &[0., 0.];
    material.materials = Some(&slots);
    let mut a = AlphaCoverage::new(&slot_masks, 0);
    a.ratio_band = [1., 1.];
    a.blurred_difference = 0.;
    material.alpha_coverage = Some(a);
    assert!(build(material).alpha_coverage_stuck_groups > 0);
}
#[test]
fn alpha_layout_limits_and_identity_fail_closed() {
    let (p, i, uv) = fixture();
    let grid = QuantGrid::new(-10).unwrap();
    let masks = [Some(AlphaMask::new(&[255], 1, 1).unwrap())];
    let mut m = BuildMesh::new(&i, &p);
    m.alpha_coverage = Some(AlphaCoverage::new(&masks, 0));
    assert!(v2::build(config(), m, grid, &mut Workspace::default()).is_err());
    for resolution in [0, 7, 129, u32::MAX] {
        let mut m = BuildMesh::new(&i, &p);
        m.attributes = Some(Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap());
        m.attribute_weights = &[0., 0.];
        let mut a = AlphaCoverage::new(&masks, 0);
        a.resolution = resolution;
        m.alpha_coverage = Some(a);
        assert!(v2::build(config(), m, grid, &mut Workspace::default()).is_err());
    }
    let c = config();
    let o = wire::WriteOptions::new(grid);
    let mut key = wire::KeyInputs::new(&c, &o, &p, &i);
    let off = wire::build_key(&key);
    key.alpha_coverage = Some(AlphaCoverage::new(&masks, 0));
    let on = wire::build_key(&key);
    assert_ne!(off, on);
    let other = [Some(AlphaMask::new(&[0], 1, 1).unwrap())];
    key.alpha_coverage = Some(AlphaCoverage::new(&other, 0));
    assert_ne!(on, wire::build_key(&key));
    let mut m = BuildMesh::new(&i, &p);
    m.attributes = Some(Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap());
    m.attribute_weights = &[0., 0.];
    m.alpha_coverage = Some(AlphaCoverage::new(&masks, 0));
    assert!(v2::build(
        c,
        m,
        grid,
        &mut Workspace::new(Limits {
            max_bytes: 1 << 20,
            max_work: 100
        })
    )
    .is_err());
}
#[test]
fn farf_roundtrip_malformed_optional_and_payload_identity() {
    let (p, i, _) = fixture();
    let b = build(BuildMesh::new(&i, &p));
    let far = far();
    assert_eq!(wire::FarField::decode(&far.encode().unwrap()).unwrap(), far);
    for materials in [false, true] {
        let file = bytes(&b, Some(far), materials);
        if !materials {
            if let Some(dir) = std::env::var_os("DAG_V2_SEED_DIR") {
                std::fs::write(std::path::Path::new(&dir).join("far-field.mdag"), &file).unwrap();
            }
        }
        let r = wire::Reader::read_all(&file, &mut Workspace::default()).unwrap();
        assert_eq!(r.far_field(), Some(far));
        assert_eq!(
            wire::Reader::open(&file[..wire::resident_len(&file).unwrap()])
                .unwrap()
                .far_field(),
            Some(far)
        );
        let count = u32::from_le_bytes(file[12..16].try_into().unwrap()) as usize;
        let entry = (0..count)
            .map(|k| 64 + 16 * k)
            .find(|&e| &file[e..e + 4] == b"FARF")
            .unwrap();
        let at = u32::from_le_bytes(file[entry + 8..entry + 12].try_into().unwrap()) as usize;
        let seal = |f: &mut Vec<u8>| {
            let len = wire::resident_len(f).unwrap();
            let h = wire::prefix_hash(&f[..len]).unwrap();
            f[40..48].copy_from_slice(&h.to_le_bytes());
        };
        for (offset, word) in [
            (4, 1u32),
            (16, 2),
            (20, 1),
            (24, 0),
            (28, 1),
            (32, 0x7fc00000),
            (48, 0),
            (52, 1),
        ] {
            let mut bad = file.clone();
            bad[at + offset..at + offset + 4].copy_from_slice(&word.to_le_bytes());
            seal(&mut bad);
            // Offset 32 changes only half of an f64; use an actual NaN below.
            if offset != 32 {
                assert!(wire::Reader::open(&bad).is_err(), "{offset}");
            }
        }
        let mut bad = file.clone();
        bad[at + 32..at + 40].copy_from_slice(&f64::NAN.to_le_bytes());
        seal(&mut bad);
        assert!(wire::Reader::open(&bad).is_err());
        let mut future = file.clone();
        future[entry..entry + 4].copy_from_slice(b"FUTR");
        seal(&mut future);
        assert!(wire::Reader::read_all(&future, &mut Workspace::default()).is_ok());
        future[entry + 4..entry + 8].copy_from_slice(&1u32.to_le_bytes());
        seal(&mut future);
        assert!(wire::Reader::open(&future).is_err());
    }
    let mut invalid = far;
    invalid.world_error = 0.;
    assert!(invalid.encode().is_err());
    let grid = QuantGrid::new(-10).unwrap();
    let c = config();
    let mut o = wire::WriteOptions::new(grid);
    let key = wire::build_key(&wire::KeyInputs::new(&c, &o, &p, &i));
    o.far_field = Some(far);
    assert_ne!(key, wire::build_key(&wire::KeyInputs::new(&c, &o, &p, &i)));
}
