#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .get(..data.len().min(384))
        .unwrap()
        .chunks_exact(12)
        .map(|b| {
            core::array::from_fn(|k| f32::from_le_bytes(b[k * 4..k * 4 + 4].try_into().unwrap()))
        })
        .collect();
    let ix: Vec<u32> = data.iter().take(96).map(|&v| u32::from(v)).collect();
    let source = LodQualityMesh::new(&ix, Positions::from_packed(&p));
    let mut options = LodQualityOptions::default();
    options.samples_per_unit_area = f64::from(data.first().copied().unwrap_or(0));
    let mut w = Workspace::new(Limits {
        max_bytes: 65536,
        max_work: 10000,
    });
    let a = measure_lod_quality(source, source, &options, &mut w);
    let usage = w.usage();
    let b = measure_lod_quality(source, source, &options, &mut w);
    assert_eq!(a, b);
    assert_eq!(usage, w.usage());
});
