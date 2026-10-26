// Existing p03 generator and original smoke seed, with only observation wrappers.
include!(concat!(env!("H3_ROOT"), "/fuzz/p03/src.rs"));
fn build_meshlets_spatial(
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min: usize,
    fill: f32,
    ws: &mut Workspace,
) -> Result<Meshlets, Error> {
    let r = meshoptimizer_rs::build_meshlets_spatial(indices, p, s, min, fill, ws);
    println!("{r:?} {:?}", ws.usage());
    r
}
fn partition_clusters(
    indices: &[u32],
    counts: &[u32],
    n: usize,
    p: Option<Positions<'_>>,
    target: usize,
    ws: &mut Workspace,
) -> Result<Partitions, Error> {
    let r = meshoptimizer_rs::partition_clusters(indices, counts, n, p, target, ws);
    println!("{r:?} {:?}", ws.usage());
    r
}
fn main() {
    eprintln!("harness_sha256={}", env!("H3_PROOF_SHA256"));
    let seed: Vec<u8> = [
        vec![20, 20, 0, 0, 0, 63, 63, 16, 64, 32],
        (0..=255u8).cycle().take(1024).collect(),
    ]
    .concat();
    for op in [4, 12] {
        exercise(op, &seed);
        for value in 0..=255u8 {
            exercise(op, &vec![value; 4096]);
            let mut data = seed.clone();
            data[2] = value;
            exercise(op, &data);
            data[2] = 0;
            data[3] = value;
            exercise(op, &data);
        }
    }
}
