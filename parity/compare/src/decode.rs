fn main() {
    let a: Vec<String> = std::env::args().collect();
    let src = std::fs::read(&a[1]).unwrap();
    let count: usize = a[2].parse().unwrap();
    #[cfg(feature = "ours")]
    let out = meshoptimizer_rs::codec::decode_vertex_buffer(
        count,
        12,
        &src,
        &mut meshoptimizer_rs::Workspace::default(),
    )
    .unwrap();
    #[cfg(feature = "theirs")]
    let out = meshopt::decode_vertex_buffer::<[f32; 3]>(&src, count).unwrap();
    std::hint::black_box(out);
}
