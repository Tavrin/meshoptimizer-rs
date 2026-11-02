//! Compare caller-buffer streaming and one-shot codec throughput.
use meshoptimizer_rs::{codec::*, Limits, Workspace};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
fn measure(mut f: impl FnMut(), bytes: usize) -> f64 {
    let start = Instant::now();
    let mut runs = 0u32;
    while start.elapsed() < Duration::from_millis(500) {
        f();
        runs += 1;
    }
    bytes as f64 * f64::from(runs) / start.elapsed().as_secs_f64() / 1e6
}
fn main() {
    let n = 65536;
    let stride = 32;
    // Smooth per-vertex attributes with small deterministic noise.
    let mut rng = 1u32;
    let vertices: Vec<u8> = (0..n)
        .flat_map(|v| {
            (0..stride / 4)
                .flat_map(|k| {
                    rng ^= rng << 13;
                    rng ^= rng >> 17;
                    rng ^= rng << 5;
                    let value = (v * (k + 1) * 3) as u32 + (rng & 7);
                    value.to_le_bytes()
                })
                .collect::<Vec<_>>()
        })
        .collect();
    // A 256 x 256 vertex grid of quads, two triangles each, in row order.
    let indices: Vec<u32> = (0..255u32)
        .flat_map(|y| (0..255u32).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let a = y * 256 + x;
            [a, a + 1, a + 256, a + 1, a + 257, a + 256]
        })
        .collect();
    for version in 0..=1 {
        let v = encode_vertex_buffer(
            &vertices,
            n,
            stride,
            VertexEncoding::new(version, 2).unwrap(),
            &mut Workspace::default(),
        )
        .unwrap();
        let t = encode_index_buffer(
            &indices,
            IndexEncoding::new(version).unwrap(),
            &mut Workspace::default(),
        )
        .unwrap();
        let mut vo = vec![0; vertices.len()];
        let mut to = vec![0; indices.len() * 4];
        let vb = vo.len();
        let tb = to.len();
        let vm = measure(
            || {
                decode_vertex_buffer_into(
                    black_box(&mut vo),
                    n,
                    stride,
                    &v,
                    &mut Workspace::default(),
                )
                .unwrap()
            },
            vb,
        );
        let tm = measure(
            || {
                decode_index_buffer_into(
                    black_box(&mut to),
                    indices.len(),
                    4,
                    &t,
                    &mut Workspace::default(),
                )
                .unwrap()
            },
            tb,
        );
        println!("v{version} one-shot vertex={vm:.2} triangle={tm:.2} MB/s decoded");
        for chunk in [4096, 65536] {
            let tail = (stride + if version == 0 { 0 } else { stride / 4 }).max(if version == 0 {
                32
            } else {
                24
            });
            let end = v.len() - tail;
            let size = vo.len();
            let rate = measure(
                || {
                    let mut d = VertexBufferDecoder::new(
                        n,
                        stride,
                        v[0],
                        &v[end..],
                        end - 1,
                        Limits::default(),
                    )
                    .unwrap();
                    let (mut at, mut out) = (1, 0);
                    loop {
                        let p = d
                            .push(&v[at..(at + chunk).min(end)], &mut vo[out..])
                            .unwrap();
                        at += p.consumed;
                        out += p.written;
                        if p.status == Status::Done {
                            break;
                        }
                    }
                    d.finish().unwrap();
                    black_box(&vo);
                },
                size,
            );
            let mut output = vec![0u32; indices.len()];
            let rate_t = measure(
                || {
                    let start = 1 + indices.len() / 3;
                    let end = t.len() - 16;
                    let mut d = IndexBufferDecoder::new(
                        indices.len(),
                        &t[..start],
                        &t[end..],
                        end - start,
                        Limits::default(),
                    )
                    .unwrap();
                    let (mut at, mut out) = (start, 0);
                    loop {
                        let p = d
                            .push(&t[at..(at + chunk).min(end)], &mut output[out..])
                            .unwrap();
                        at += p.consumed;
                        out += p.written;
                        if p.status == Status::Done {
                            break;
                        }
                    }
                    d.finish().unwrap();
                    black_box(&output);
                },
                to.len(),
            );
            println!(
                "v{version} chunk={chunk} vertex={rate:.2} ({:.3}x) triangle={rate_t:.2} ({:.3}x)",
                rate / vm,
                rate_t / tm
            );
        }
    }
}
