#![no_main]
//! MDAG 2 reader: hostile resident prefixes and pages. Each input runs as given
//! and again with its page and resident hashes recomputed, so mutations reach
//! the structural rules behind the hash checks.
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{clusterlod::dag_format::v2, Workspace};

fn word(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

fn disk<'a>(data: &'a [u8], range: core::ops::Range<u64>) -> Option<&'a [u8]> {
    data.get(usize::try_from(range.start).ok()?..usize::try_from(range.end).ok()?)
}

/// Recompute every page hash whose range lies in the input, then the resident hash.
fn reseal(b: &mut [u8]) -> Option<()> {
    let chunks = word(b, 12)? as usize;
    let resident = word(b, 24)? as usize;
    if chunks > v2::MAX_CHUNKS || !(64..=b.len()).contains(&resident) {
        return None;
    }
    let entry = (0..chunks)
        .map(|i| 64 + 16 * i)
        .find(|&e| b.get(e..e + 4) == Some(b"PAGE".as_slice()))?;
    let (at, len) = (word(b, entry + 8)? as usize, word(b, entry + 12)? as usize);
    for p in 0..len / v2::PAGE_BYTES {
        let rec = at + p * v2::PAGE_BYTES;
        let start = u64::from(word(b, rec)?) | u64::from(word(b, rec + 4)?) << 32;
        let end = start.checked_add(u64::from(word(b, rec + 8)?))?;
        if let Some(h) = disk(b, start..end).map(|d| v2::xxh64(d, 0)) {
            b.get_mut(rec + 32..rec + 40)?.copy_from_slice(&h.to_le_bytes());
        }
    }
    let h = v2::xxh64(b.get(64..resident)?, 0);
    b.get_mut(40..48)?.copy_from_slice(&h.to_le_bytes());
    Some(())
}

fn check(data: &[u8]) {
    let mut ws = Workspace::default();
    if let Ok(r) = v2::Reader::open(data) {
        // Every accessor of a validated prefix succeeds; pages named by the
        // records either validate or fail cleanly.
        let m = r.mesh();
        for g in 0..m.group_count as usize {
            r.group(g).expect("validated group");
        }
        for n in 0..m.node_count as usize {
            r.node(n).expect("validated node");
        }
        let mut out = vec![0u32; r.page_capacity() / 4];
        for p in 0..m.page_count as usize {
            let rec = r.page(p).expect("validated page");
            assert!(r.dependencies(p).expect("dependencies").all(|d| (d as usize) < p));
            if let Some(bytes) = disk(data, rec.disk_range()) {
                let _ = r.transcode_page(p, bytes, &mut out, &mut ws);
            }
        }
        let _ = r.chunk(v2::TAG_FAR_FIELD);
    }
    let Ok(all) = v2::Reader::read_all(data, &mut ws) else {
        return;
    };
    // A whole valid file opens from its resident prefix alone, and every page
    // transcodes from exactly its own bytes and fails when truncated.
    let prefix = &data[..v2::resident_len(data).expect("resident length")];
    let r = v2::Reader::open(prefix).expect("resident prefix opens");
    assert_eq!(r.mesh(), all.mesh());
    let mut out = vec![0u32; r.page_capacity() / 4];
    let (mut pos, mut tri) = ([[0f32; 3]; 256], [[0u8; 3]; 256]);
    for p in 0..r.mesh().page_count as usize {
        let bytes = disk(data, r.page(p).expect("page").disk_range()).expect("page bytes");
        let view = r
            .transcode_page(p, bytes, &mut out, &mut ws)
            .expect("validated page transcodes");
        for k in 0..view.cluster_count() {
            let nv = view.decode_positions(k, &mut pos).expect("positions");
            let nt = view.decode_triangles(k, &mut tri).expect("triangles");
            assert!(tri[..nt].iter().flatten().all(|&i| (i as usize) < nv));
        }
        assert!(r
            .transcode_page(p, &bytes[..bytes.len() - 1], &mut out, &mut ws)
            .is_err());
    }
}

fuzz_target!(|data: &[u8]| {
    check(data);
    let mut sealed = data.to_vec();
    if reseal(&mut sealed).is_some() && sealed != data {
        check(&sealed);
    }
});
