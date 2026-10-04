#![forbid(unsafe_code)]
use meshoptimizer_rs::{codec::*, Limits, Workspace};
pub fn exercise(entry: u8, data: &[u8]) {
    if data.len() < 9 {
        return;
    }
    let count = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
    let stride = u16::from_le_bytes(data[4..6].try_into().unwrap()) as usize;
    let mode = match data[6] % 3 {
        0 => Mode::Attributes,
        1 => Mode::Triangles,
        _ => Mode::Indices,
    };
    let filter = match data[7] % 4 {
        0 => Filter::None,
        1 => Filter::Octahedral,
        2 => Filter::Quaternion,
        _ => Filter::Exponential,
    };
    let source = &data[9..];
    let mut ws = Workspace::new(Limits {
        max_bytes: 65536,
        max_work: 131072,
    });
    // Small limits retain oversized headers as errors instead of masking them.
    let len = usize::from(data[8]) * 256;
    let mut out = vec![0xcc; len];
    match entry {
        0 => {
            let _ = decode_vertex_buffer(count, stride, source, &mut ws);
        }
        1 => {
            let _ = decode_vertex_buffer_into(&mut out, count, stride, source, &mut ws);
        }
        2 => {
            let _ = decode_index_buffer(count, stride, source, &mut ws);
        }
        3 => {
            let _ = decode_index_buffer_into(&mut out, count, stride, source, &mut ws);
        }
        4 => {
            let _ = decode_index_sequence(count, stride, source, &mut ws);
        }
        5 => {
            let _ = decode_index_sequence_into(&mut out, count, stride, source, &mut ws);
        }
        6..=8 => {
            let n = source.len().min(out.len());
            out[..n].copy_from_slice(&source[..n]);
            match entry {
                6 => {
                    let _ = decode_filter_oct(&mut out, count, stride, &mut ws);
                }
                7 => {
                    let _ = decode_filter_quat(&mut out, count, stride, &mut ws);
                }
                _ => {
                    let _ = decode_filter_exp(&mut out, count, stride, &mut ws);
                }
            }
        }
        9 => {
            let _ = decode_vertex_version(source);
        }
        10 => {
            let _ = decode_index_version(source);
        }
        11 => {
            let _ = decode_buffer_view(mode, filter, count, stride, source, &mut ws);
        }
        _ => {
            let _ = decode_buffer_view_into(&mut out, mode, filter, count, stride, source, &mut ws);
        }
    }
}
