#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let indices: Vec<u32> = data.iter().take(192).map(|b| u32::from(*b) % 32).collect();
    // The trailing byte picks the vertex count (some too small), the work
    // limit and the destination size.
    let mode = data.last().copied().unwrap_or(0);
    let vertex_count = if mode & 1 == 0 {
        32
    } else {
        usize::from(mode >> 4) + 1
    };
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    let full = orient_faces(&indices, vertex_count, &mut ws);
    assert_eq!(orient_faces_bound(indices.len()), indices.len());
    let len = if mode & 2 == 0 {
        indices.len() + 3
    } else {
        indices.len().saturating_sub(1)
    };
    let mut out = vec![u32::MAX; len];
    let before = out.clone();
    match orient_faces_into(&mut out, &indices, vertex_count, &mut ws) {
        Err(_) => assert_eq!(out, before),
        Ok(r) => {
            for (f, (a, b)) in indices.chunks_exact(3).zip(out.chunks_exact(3)).enumerate() {
                assert_eq!(a[0], b[0]);
                assert_eq!(
                    r.flipped_faces.binary_search(&(f as u32)).is_ok(),
                    a[1] == b[2]
                );
            }
            assert!(out[indices.len()..].iter().all(|&v| v == u32::MAX));
            let alloc = full.as_ref().expect("allocating form succeeds too");
            assert_eq!(&out[..indices.len()], &alloc.indices[..]);
            assert_eq!(r, alloc.report);
            // Oriented output keeps every face's corner set.
            for (a, b) in indices.chunks_exact(3).zip(alloc.indices.chunks_exact(3)) {
                let (mut x, mut y) = ([a[0], a[1], a[2]], [b[0], b[1], b[2]]);
                x.sort_unstable();
                y.sort_unstable();
                assert_eq!(x, y);
            }
        }
    }
    let mut tight = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: u64::from(mode >> 2) * 16,
    });
    if let (Ok(limited), Ok(full)) = (orient_faces(&indices, vertex_count, &mut tight), &full) {
        assert_eq!(&limited, full);
    }
});
