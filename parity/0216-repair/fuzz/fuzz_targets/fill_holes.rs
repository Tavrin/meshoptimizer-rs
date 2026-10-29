#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .chunks_exact(3)
        .take(32)
        .map(|b| [b[0] as f32, b[1] as f32, b[2] as f32])
        .collect();
    let indices: Vec<u32> = data.iter().take(96).map(|b| u32::from(*b) % 32).collect();
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    if let Ok(r) = fill_holes(&indices, Positions::from_packed(&p), 16, &mut ws) {
        assert_eq!(&r.indices[..indices.len()], &indices);
        assert!(r.indices.iter().all(|&v| (v as usize) < p.len()));
        assert_eq!(r.indices.len(), r.report.output_triangles * 3);
    }
});
