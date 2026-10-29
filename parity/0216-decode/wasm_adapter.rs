use meshoptimizer_rs::{codec::*,Workspace};
use std::sync::Mutex;
struct Bench {
    source: Vec<u8>,
    output: Vec<u8>,
    workspace: Workspace,
    op: u32,
    count: usize,
    stride: usize,
    mode: Mode,
    filter: Filter,
}
static BENCH: Mutex<Option<Bench>> = Mutex::new(None);
#[no_mangle]
pub extern "C" fn bench_prepare(
    length: u32,
    op: u32,
    count: u32,
    stride: u32,
    mode: u32,
    filter: u32,
) -> u32 {
    let mode = match mode {
        0 => Mode::Attributes,
        1 => Mode::Triangles,
        2 => Mode::Indices,
        _ => return 1,
    };
    let filter = match filter {
        0 => Filter::None,
        1 => Filter::Octahedral,
        2 => Filter::Quaternion,
        3 => Filter::Exponential,
        _ => return 1,
    };
    let Some(bytes) = (count as usize).checked_mul(stride as usize) else {
        return 1;
    };
    if length > 128 * 1024 * 1024 || bytes > 128 * 1024 * 1024 || !matches!(op, 1 | 2 | 3 | 7) {
        return 1;
    }
    *BENCH.lock().unwrap() = Some(Bench {
        source: vec![0; length as usize],
        output: vec![0; bytes],
        workspace: Workspace::default(),
        op,
        count: count as usize,
        stride: stride as usize,
        mode,
        filter,
    });
    0
}
#[no_mangle]
pub extern "C" fn bench_source_ptr() -> usize {
    BENCH
        .lock()
        .unwrap()
        .as_ref()
        .map_or(0, |b| b.source.as_ptr() as usize)
}
#[no_mangle]
pub extern "C" fn bench_output_ptr() -> usize {
    BENCH
        .lock()
        .unwrap()
        .as_ref()
        .map_or(0, |b| b.output.as_ptr() as usize)
}
#[no_mangle]
pub extern "C" fn bench_run(into: u32) -> u32 {
    bench_run_inner(into)
}
fn bench_run_inner(into: u32) -> u32 {
    let mut lock = BENCH.lock().unwrap();
    let Some(b) = lock.as_mut() else {
        return 1;
    };
    let Bench {
        source,
        output,
        workspace,
        op,
        count,
        stride,
        mode,
        filter,
    } = b;
    let result = if into != 0 {
        match *op {
            1 => decode_vertex_buffer_into(output, *count, *stride, source, workspace),
            2 => decode_index_buffer_into(output, *count, *stride, source, workspace),
            3 => decode_index_sequence_into(output, *count, *stride, source, workspace),
            _ => {
                decode_buffer_view_into(output, *mode, *filter, *count, *stride, source, workspace)
            }
        }
    } else {
        let result = match *op {
            1 => decode_vertex_buffer(*count, *stride, source, workspace),
            2 => decode_index_buffer(*count, *stride, source, workspace),
            3 => decode_index_sequence(*count, *stride, source, workspace),
            _ => decode_buffer_view(*mode, *filter, *count, *stride, source, workspace),
        };
        result.map(|out| *output = out)
    };
    std::hint::black_box(result).is_err() as u32
}
