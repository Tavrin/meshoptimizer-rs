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
    // The trailing byte picks the loop size limit, a work limit and the
    // destination size.
    let mode = data.last().copied().unwrap_or(0);
    let max_edges = if mode & 1 == 0 {
        16
    } else {
        usize::from(mode >> 4) + 2
    };
    let positions = Positions::from_packed(&p);
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    let full = fill_holes(&indices, positions, max_edges, &mut ws);
    if let Ok(r) = &full {
        assert_eq!(&r.indices[..indices.len()], &indices);
        assert!(r.indices.iter().all(|&v| (v as usize) < p.len()));
        assert_eq!(r.indices.len(), r.report.output_triangles * 3);
        assert_eq!(r.report.input_triangles * 3, indices.len());
        assert!(r.indices.len() <= fill_holes_bound(indices.len()).unwrap());
    }
    // Caller-buffer form: same mesh and report; a short destination fails and
    // leaves the buffer unchanged.
    let bound = fill_holes_bound(indices.len()).unwrap();
    let len = if mode & 2 == 0 {
        bound + 3
    } else {
        indices.len()
    };
    let mut out = vec![u32::MAX; len];
    let before = out.clone();
    match fill_holes_into(&mut out, &indices, positions, max_edges, &mut ws) {
        Ok(report) => {
            let r = full.as_ref().expect("allocating form succeeds too");
            assert_eq!(&out[..r.indices.len()], &r.indices[..]);
            assert_eq!(report, r.report);
            assert!(out[r.indices.len()..].iter().all(|&v| v == u32::MAX));
        }
        Err(_) => assert_eq!(out, before),
    }
    let mut tight = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: u64::from(mode >> 2) * 64,
    });
    if let (Ok(limited), Ok(full)) = (
        fill_holes(&indices, positions, max_edges, &mut tight),
        &full,
    ) {
        assert_eq!(&limited, full);
    }
});
