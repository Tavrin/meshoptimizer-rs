#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::{
    clusterlod::{dag_format::*, FlatDagCluster, FlatDagGroup},
    Workspace,
};
fn seed() -> Vec<u8> {
    let mut g = FlatDagGroup::default();
    g.cluster_count = 1;
    g.error = f32::MAX;
    let mut c = FlatDagCluster::default();
    c.index_count = 3;
    c.refined = -1;
    write(
        &[g],
        &[c],
        &[[0.; 3], [1., 0., 0.], [0., 1., 0.]],
        &[0, 1, 2],
        DEFAULT_PAGE_SIZE,
        &mut Workspace::default(),
    )
    .unwrap()
}
#[test]
fn rejects_truncations_and_corrupt_tables() {
    let bytes = seed();
    let mut ws = Workspace::default();
    for end in 0..bytes.len() {
        assert!(Reader::read(&bytes[..end], &mut ws).is_err());
    }
    for at in [
        4,
        8,
        12,
        16,
        20,
        28,
        32 + 24,
        32 + 28,
        64,
        72,
        76,
        80,
        84,
        88,
        92,
        96,
        100,
        120,
        124,
        128,
        132,
        136,
        140,
    ] {
        let mut bad = bytes.clone();
        bad[at..at + 4].fill(255);
        assert!(Reader::read(&bad, &mut ws).is_err(), "offset {at}");
    }
}
#[test]
fn empty_and_page_limits() {
    let mut ws = Workspace::default();
    let b = write(&[], &[], &[], &[], DEFAULT_PAGE_SIZE, &mut ws).unwrap();
    assert_eq!(Reader::read(&b, &mut ws).unwrap().page_count(), 0);
    assert!(write(&[], &[], &[], &[], 0, &mut ws).is_err());
    let b = seed();
    let r = Reader::read(&b, &mut ws).unwrap();
    assert!(r.decode_page(1, &[], &mut ws).is_err());
    assert!(r.decode_page(0, &[], &mut ws).is_err());
    assert_eq!(seed(), b);
}
#[test]
fn emit_seed_if_requested() {
    if let Ok(path) = std::env::var("DAG_SEED_PATH") {
        std::fs::write(path, seed()).unwrap();
    }
}

#[test]
fn rejects_cycles_nonmonotonic_errors_and_bad_page_geometry() {
    let mut groups = Vec::new();
    let mut clusters = Vec::new();
    for i in 0..2 {
        let mut group = FlatDagGroup::default();
        group.first_cluster = i;
        group.cluster_count = 1;
        group.depth = i as i32;
        group.error = (i + 1) as f32;
        groups.push(group);
        let mut cluster = FlatDagCluster::default();
        cluster.first_index = i * 3;
        cluster.index_count = 3;
        cluster.group = i;
        cluster.refined = i as i32 - 1;
        clusters.push(cluster);
    }
    let mut ws = Workspace::default();
    let bytes = write(
        &groups,
        &clusters,
        &[[0.; 3], [1., 0., 0.], [0., 1., 0.]],
        &[0, 1, 2, 0, 1, 2],
        60,
        &mut ws,
    )
    .unwrap();
    assert_eq!(Reader::read(&bytes, &mut ws).unwrap().page_count(), 2);
    for (at, value) in [
        (164, 1u32),
        (164, u32::from_le_bytes((-2i32).to_le_bytes())),
        (84, 0),
        (80, 0.5f32.to_bits()),
        (160, 0),
    ] {
        let mut bad = bytes.clone();
        bad[at..at + 4].copy_from_slice(&value.to_le_bytes());
        assert!(Reader::read(&bad, &mut ws).is_err(), "offset {at}");
    }
    let reader = Reader::read(&bytes, &mut ws).unwrap();
    let range = reader.page_range(1).unwrap();
    let mut page = bytes[range].to_vec();
    page[0..4].copy_from_slice(&u32::MAX.to_le_bytes());
    // Snapshot identifiers are opaque labels; changing position to NaN is invalid.
    page[4..8].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
    assert!(reader.decode_page(1, &page, &mut ws).is_err());
}

/// Single-cluster file with raw (uncompressed) streams, built from the writer's tables.
fn forge(ids: [u32; 3], indices: [u32; 3]) -> Vec<u8> {
    let mut out = seed()[..144].to_vec();
    let pos = [[0f32; 3], [1., 0., 0.], [0., 1., 0.]];
    let mut payload = Vec::new();
    for (id, p) in ids.iter().zip(pos) {
        payload.extend(id.to_le_bytes());
        for f in p {
            payload.extend(f.to_bits().to_le_bytes());
        }
    }
    for i in indices {
        payload.extend(i.to_le_bytes());
    }
    let put = |b: &mut Vec<u8>, at: usize, v: usize| {
        b[at..at + 4].copy_from_slice(&u32::try_from(v).unwrap().to_le_bytes())
    };
    put(&mut out, 28, 144 + payload.len());
    put(&mut out, 80, 48);
    put(&mut out, 84, 12);
    put(&mut out, 100, 0);
    put(&mut out, 132, payload.len());
    out.extend(payload);
    out
}

#[test]
fn rejects_non_canonical_vertex_order_duplicate_ids_and_unused_vertices() {
    let mut ws = Workspace::default();
    assert!(Reader::read(&forge([0, 1, 2], [0, 1, 2]), &mut ws).is_ok());
    assert!(Reader::read(&forge([0, 1, 2], [2, 1, 0]), &mut ws).is_err());
    assert!(Reader::read(&forge([0, 0, 2], [0, 1, 2]), &mut ws).is_err());
    assert!(Reader::read(&forge([0, 1, 2], [0, 1, 1]), &mut ws).is_err());
}
