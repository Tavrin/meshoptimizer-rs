#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{clusterlod::dag_format::Reader, Workspace};

fuzz_target!(|data: &[u8]| {
    let mut ws = Workspace::default();
    let Ok(r) = Reader::read(data, &mut ws) else {
        return;
    };
    // `read` validates every table and page: each page must then decode, with
    // identical results for u32 and u16 whenever the u16 form is accepted.
    for p in 0..r.page_count() {
        let range = r.page_range(p).expect("page range of a validated file");
        let bytes = &data[range];
        let a = r.decode_page(p, bytes, &mut ws).expect("validated page decodes");
        let b = r.decode_page_generic::<u16>(p, bytes, &mut ws);
        if let Ok(b) = b {
            assert_eq!(a.len(), b.len());
            for (x, y) in a.iter().zip(&b) {
                assert_eq!(x.cluster, y.cluster);
                assert_eq!(x.vertices, y.vertices);
                assert!(x.indices.iter().copied().eq(y.indices.iter().map(|&i| u32::from(i))));
                assert!(x.indices.iter().all(|&i| (i as usize) < x.vertices.len()));
            }
        }
        // A page decoded from truncated bytes must fail cleanly.
        if !bytes.is_empty() {
            assert!(r.decode_page(p, &bytes[..bytes.len() - 1], &mut ws).is_err());
        }
    }
    for c in 0..r.cluster_count() {
        r.cluster_record(c).expect("cluster record");
        r.cluster_page(c).expect("cluster page");
    }
    for g in 0..r.group_count() {
        r.group(g).expect("group record");
    }
});
