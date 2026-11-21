#![no_main]

use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{
    clusterlod::{
        dag_format::v2::{self, Reader},
        v2::health_from_reader,
    },
    Workspace,
};

fuzz_target!(|data: &[u8]| {
    let Ok(len) = v2::resident_len(data) else {
        return;
    };
    let Some(prefix) = data.get(..len) else {
        return;
    };
    let Some(hash) = v2::prefix_hash(prefix) else {
        return;
    };
    let mut prefix = prefix.to_vec();
    // Keep the checksum valid so mutations can exercise the opened tables.
    prefix[v2::RESIDENT_HASH_FIELD].copy_from_slice(&hash.to_le_bytes());
    let Ok(reader) = Reader::open(&prefix) else {
        return;
    };
    let count = reader.mesh().cluster_count as usize;
    // Counts are supplied by the caller, so fuzz them independently of page decoding.
    let counts: Vec<u32> = data
        .iter()
        .rev()
        .cycle()
        .take(count)
        .map(|&b| u32::from(b))
        .collect();
    let minimum_levels = u32::from(data[data.len() - 1]) % (v2::MAX_LEVELS + 2);
    let _ = health_from_reader(&reader, &counts, minimum_levels, &mut Workspace::default());
});
