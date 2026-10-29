#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;
fuzz_target!(|data: &[u8]| {
    let indices: Vec<u32> = data.iter().take(192).map(|b| u32::from(*b) % 32).collect();
    let mut ws = Workspace::new(Limits {
        max_bytes: 1 << 20,
        max_work: 100_000,
    });
    let mut out = vec![u32::MAX; indices.len() + 3];
    let before = out.clone();
    match orient_faces_into(&mut out, &indices, 32, &mut ws) {
        Err(_) => assert_eq!(out, before),
        Ok(r) => {
            for (f, (a, b)) in indices.chunks_exact(3).zip(out.chunks_exact(3)).enumerate() {
                assert_eq!(a[0], b[0]);
                assert_eq!(
                    r.flipped_faces.binary_search(&(f as u32)).is_ok(),
                    a[1] == b[2]
                );
            }
            assert_eq!(&out[indices.len()..], &[u32::MAX; 3]);
        }
    }
});
