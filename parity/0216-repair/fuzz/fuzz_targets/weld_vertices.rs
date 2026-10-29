#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .chunks_exact(12)
        .take(64)
        .map(|b| {
            core::array::from_fn(|k| {
                f32::from_bits(u32::from_le_bytes(b[k * 4..k * 4 + 4].try_into().unwrap()))
            })
        })
        .collect();
    let tolerance = data.first().copied().unwrap_or(0) as f32 / 255.;
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    if let Ok(r) = weld_vertices(
        Positions::from_packed(&p),
        tolerance,
        WeldPolicy::PositionsOnly,
        &mut ws,
    ) {
        for (i, &j) in r.remap.iter().enumerate() {
            assert!(j as usize <= i);
            assert_eq!(r.remap[j as usize], j);
            let d: f64 = (0..3)
                .map(|k| (f64::from(p[i][k]) - f64::from(p[j as usize][k])).powi(2))
                .sum();
            assert!(d <= f64::from(tolerance).powi(2));
        }
    }
});
