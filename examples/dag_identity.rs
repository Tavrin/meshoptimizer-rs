#![allow(clippy::field_reassign_with_default)]
//! Native/wasm deterministic writer identity probe (stdout is hex bytes).
use meshoptimizer_rs::{
    clusterlod::{dag_format::*, FlatDagCluster, FlatDagGroup},
    Workspace,
};
fn main() {
    let mut groups = Vec::new();
    let mut clusters = Vec::new();
    let mut indices = Vec::new();
    for g in 0..3 {
        let mut group = FlatDagGroup::default();
        group.first_cluster = g;
        group.cluster_count = 1;
        group.depth = g as i32;
        group.error = if g == 2 { f32::MAX } else { g as f32 };
        groups.push(group);
        let mut c = FlatDagCluster::default();
        c.group = g;
        c.refined = g as i32 - 1;
        c.first_index = g * 6;
        c.index_count = 6;
        clusters.push(c);
        indices.extend([0, 1, 2, 2, 1, 0]);
    }
    let b = write(
        &groups,
        &clusters,
        &[[0., 0., -0.], [1., 2., 3.], [4., 5., 6.]],
        &indices,
        128,
        &mut Workspace::default(),
    )
    .unwrap();
    for byte in b {
        print!("{byte:02x}");
    }
    println!();
    for path in std::env::args().skip(1) {
        let original = std::fs::read(path).unwrap();
        let mut ws = Workspace::default();
        let reader = Reader::read(&original, &mut ws).unwrap();
        let groups: Vec<_> = (0..reader.group_count())
            .map(|g| reader.group(g).unwrap())
            .collect();
        let clusters: Vec<_> = (0..reader.cluster_count())
            .map(|c| reader.cluster_record(c).unwrap())
            .collect();
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for p in 0..reader.page_count() {
            for geometry in reader
                .decode_page(p, &original[reader.page_range(p).unwrap()], &mut ws)
                .unwrap()
            {
                for &i in &geometry.indices {
                    indices.push(geometry.vertices[i as usize].0);
                }
                for (id, pos) in geometry.vertices {
                    let id = id as usize;
                    if positions.len() <= id {
                        positions.resize(id + 1, [0.; 3]);
                    }
                    positions[id] = pos;
                }
            }
        }
        let capacity = u32::from_le_bytes(original[24..28].try_into().unwrap()) as usize;
        let encoded = write(&groups, &clusters, &positions, &indices, capacity, &mut ws).unwrap();
        assert_eq!(original, encoded);
        for byte in encoded {
            print!("{byte:02x}");
        }
        println!();
    }
}
