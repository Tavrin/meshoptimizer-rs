#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let p: Vec<[f32; 3]> = data
        .chunks_exact(3)
        .take(24)
        .map(|p| {
            [
                f32::from(p[0]) / 255.,
                f32::from(p[1]) / 255.,
                f32::from(p[2]) / 255.,
            ]
        })
        .collect();
    let bytes: Vec<_> = p
        .iter()
        .flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes()))
        .collect();
    let i: Vec<u32> = data
        .iter()
        .skip(1)
        .take(72)
        .map(|&v| u32::from(v) % u32::try_from(p.len().max(1)).unwrap())
        .collect();
    let stream = VertexStream::packed(&bytes, 12).unwrap();
    let positions = Positions::from_packed(&p);
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    if let Ok(r) = split_non_manifold_vertices(&i, stream, &mut ws) {
        let pos: Vec<_> = r
            .vertices
            .chunks_exact(12)
            .map(|p| [0, 1, 2].map(|a| f32::from_le_bytes(p[a * 4..a * 4 + 4].try_into().unwrap())))
            .collect();
        let a = analyze_mesh(
            &r.indices,
            Positions::from_packed(&pos),
            AnalyzeOptions::EMPTY,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(a.non_manifold_vertices, 0);
    }
    let tol = data.first().map(|&v| f32::from(v) / 255.);
    let r = stitch_cracks(&i, stream, positions, tol, &mut ws);
    if let Ok(r) = r {
        assert!(r.indices.iter().all(|&v| (v as usize) < p.len()));
        let edges = |i: &[u32]| {
            analyze_mesh(
                i,
                positions,
                AnalyzeOptions::EMPTY,
                &mut Workspace::default(),
            )
            .map(|a| a.non_manifold_edges)
        };
        if let (Ok(before), Ok(after)) = (edges(&i), edges(&r.indices)) {
            assert!(after <= before);
        }
        let again = stitch_cracks(&i, stream, positions, tol, &mut ws).unwrap();
        assert_eq!(r, again);
    }
});
