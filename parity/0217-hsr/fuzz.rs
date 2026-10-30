// Source-only fuzz target; intentionally not built or run in this campaign.
#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .chunks_exact(12)
        .take(64)
        .map(|v| {
            core::array::from_fn(|k| f32::from_le_bytes(v[k * 4..k * 4 + 4].try_into().unwrap()))
        })
        .collect();
    let i: Vec<u32> = data.iter().take(96).map(|&x| u32::from(x) % 65).collect();
    let i = &i[..i.len() / 3 * 3];
    let mut ws = Workspace::new(Limits {
        max_work: 100_000,
        max_bytes: 1 << 20,
    });
    let o = HiddenOptions::default();
    let a = remove_hidden_triangles(i, Positions::from_packed(&p), o, &mut ws);
    let mut out = vec![u32::MAX; i.len() + 3];
    let b = remove_hidden_triangles_into(&mut out, i, Positions::from_packed(&p), o, &mut ws);
    if let Ok(report) = b {
        assert!(out[report.index_count()..].iter().all(|&x| x == u32::MAX));
        if let Ok(mesh) = a {
            assert_eq!(mesh.report, report);
            assert_eq!(&out[..report.index_count()], mesh.indices);
        }
    } else {
        assert!(out.iter().all(|&x| x == u32::MAX));
    }
});
