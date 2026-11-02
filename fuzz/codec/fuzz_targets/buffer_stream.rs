#![no_main]
#[path = "../../../tests/support/buffer_stream.rs"]
mod support;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{codec::*, Workspace};
fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    let count = usize::from(data[0] % 65);
    let stride = (usize::from(data[1] % 64) + 1) * 4;
    let chunk = usize::from(data[2]) + 1;
    let width = usize::from(data[3] % 128) + 1;
    let source = &data[4..];
    assert_eq!(
        support::vertex(count, stride, source, chunk, width),
        decode_vertex_buffer(count, stride, source, &mut Workspace::default())
    );
    assert_eq!(
        support::triangles(count * 3, source, chunk, width),
        decode_index_buffer(count * 3, 4, source, &mut Workspace::default())
    );
});
