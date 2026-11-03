#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::codec::{decode_vertex_buffer, IndexEncoding, VertexEncoding};
use meshoptimizer_rs::*;

const STRIDE: usize = 48; // position, one f32 attribute, four joints, four weights
const BONES: u32 = 8;

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let m = mesh(&mut c, 80);
    let (a, b) = (c.u8(), c.u8());
    let mut records = Vec::with_capacity(m.pos.len() * STRIDE);
    for p in &m.pos {
        records.extend(bytes_of(std::slice::from_ref(p)));
        records.extend((c.unit() - 0.5).to_le_bytes());
        let raw: [u8; 4] = [c.u8(), c.u8(), c.u8(), c.u8()];
        let sum: f32 = raw.iter().map(|&x| f32::from(x)).sum();
        for _ in 0..4 {
            records.extend((u32::from(c.u8()) % BONES).to_le_bytes());
        }
        for (k, &x) in raw.iter().enumerate() {
            let w = if sum == 0. { f32::from(u8::from(k == 0)) } else { f32::from(x) / sum };
            records.extend(w.to_le_bytes());
        }
    }
    let importance: Vec<f32> = (0..BONES).map(|_| c.unit()).collect();
    let weights = [0.5f32];
    let ratios = [0.5f32, 0.25];
    let mut o = ProcessOptions::default();
    o.sanitize = (a & 1 != 0).then(SanitizeOptions::default);
    o.weld = a & 2 != 0;
    o.orient = a & 4 != 0;
    o.split_non_manifold_vertices = a & 8 != 0;
    o.stitch_cracks = (a & 16 != 0).then_some((a & 32 != 0).then_some(c.unit() * 0.1));
    o.fill_holes = (a & 64 != 0).then_some(1 + usize::from(c.u8() % 12));
    o.lod = (a & 128 != 0).then(|| LodChainOptions::new(LodTargets::Ratios(&ratios), LodErrors::Uniform(c.unit())));
    o.attributes = (b & 1 != 0).then_some((12, &weights[..]));
    o.skin = (b & 2 != 0).then(|| {
        let mut s = ProcessSkin::new(16, 32, 4, &importance);
        s.options.best_of = b & 4 != 0;
        s
    });
    o.cache = b & 8 != 0;
    o.overdraw = (b & 16 != 0).then_some(1.05);
    o.fetch = b & 32 != 0;
    o.meshlets = (b & 64 != 0).then(|| (MeshletSettings::default(), 0.25));
    o.index_encoding = (b & 128 != 0).then_some(IndexEncoding::DEFAULT);
    o.vertex_encoding = (c.u8() & 1 != 0).then_some(VertexEncoding::DEFAULT);
    o.clusters = (c.u8() & 1 != 0).then(|| clusterlod::default_config(124).expect("default config"));
    let limits = Limits { max_bytes: 64 << 20, max_work: 8_000_000 };
    let mut ws = Workspace::new(limits);
    let r = process_mesh(&m.idx, &records, STRIDE, 0, &o, &mut ws);
    let r2 = process_mesh(&m.idx, &records, STRIDE, 0, &o, &mut Workspace::new(limits));
    // Everything but the wall-clock stage timings.
    let key = |r: &Result<ProcessedMesh, ProcessError>| match r {
        Ok(p) => format!(
            "{:?} {:?} {:?} {:?}",
            p.levels,
            p.report.usage,
            p.report.quality,
            p.report.stages.iter().map(|s| (s.stage, s.level, s.work)).collect::<Vec<_>>()
        ),
        Err(e) => format!("{e:?}"),
    };
    assert_eq!(key(&r), key(&r2), "process_mesh is not deterministic");
    let Ok(p) = r else { return };
    assert!(!p.levels.is_empty());
    assert_eq!(p.report.quality.len(), p.levels.len());
    for l in &p.levels {
        assert_eq!(l.vertices.len() % STRIDE, 0);
        let n = l.vertices.len() / STRIDE;
        assert_eq!(l.indices.len() % 3, 0);
        assert!(l.indices.iter().all(|&i| (i as usize) < n));
        if let Some(e) = &l.encoded_vertices {
            let back = decode_vertex_buffer(n, STRIDE, e, &mut Workspace::default()).expect("encoded vertices decode");
            assert_eq!(back, l.vertices);
        }
        if let Some(e) = &l.encoded_indices {
            codec::decode_index_buffer(l.indices.len(), 4, e, &mut Workspace::default()).expect("encoded indices decode");
        }
    }
    assert!(p.report.usage.work <= limits.max_work);
    // The exact counted work is enough to reproduce the result.
    if p.report.usage.work > 0 {
        let tight = Limits { max_work: p.report.usage.work, ..limits };
        let t = process_mesh(&m.idx, &records, STRIDE, 0, &o, &mut Workspace::new(tight)).expect("exact work limit");
        assert_eq!(format!("{:?}", t.levels), format!("{:?}", p.levels));
    }
});
