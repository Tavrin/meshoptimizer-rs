//! Valid encoded seeds for the codec fuzz targets (`vertex`, `vertex_into`,
//! `simd`, `sequence_stream`), produced with the crate's own encoders.
//!
//! Usage: seedgen OUT_DIR. Writes OUT_DIR/{vertex,vertex_into,simd,sequence_stream}/NNNN.
//! Vertex layout: u32 count, u16 stride, 3 parameter bytes, source. The `simd`
//! target prefixes a selector byte (0 = decode_vertex_buffer, 1 = _into).
//! Sequence-stream layout: count, chunk, width bytes, source.
use meshoptimizer_rs::codec::{
    encode_index_sequence, encode_vertex_buffer, IndexEncoding, VertexEncoding,
};
use meshoptimizer_rs::Workspace;
use std::{fs, path::PathBuf};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn byte(&mut self) -> u8 {
        (self.next() >> 24) as u8
    }
}

/// Vertex payload of `count` records of `stride` bytes in one of six styles.
fn payload(rng: &mut Rng, count: usize, stride: usize, style: usize) -> Vec<u8> {
    let mut v = vec![0u8; count * stride];
    for i in 0..count {
        let record = &mut v[i * stride..(i + 1) * stride];
        match style {
            // smooth float positions, one lattice step per vertex
            0 => {
                for (k, chunk) in record.chunks_exact_mut(4).enumerate() {
                    let x = (i as f32) * 0.25 + k as f32;
                    chunk.copy_from_slice(&x.to_le_bytes());
                }
            }
            // incompressible
            1 => record.iter_mut().for_each(|b| *b = rng.byte()),
            // constant
            2 => record.iter_mut().for_each(|b| *b = 7),
            // small integer ramps per byte lane
            3 => record
                .iter_mut()
                .enumerate()
                .for_each(|(k, b)| *b = (i * (k % 5 + 1)) as u8),
            // sparse: mostly zero, occasional noise
            4 => record.iter_mut().for_each(|b| {
                *b = if rng.byte() < 16 { rng.byte() } else { 0 };
            }),
            // quantised 16-bit lanes with small deltas
            _ => {
                for (k, chunk) in record.chunks_mut(2).enumerate() {
                    let x = (1000 + i * 3 + k + usize::from(rng.byte() & 3)) as u16;
                    let b = x.to_le_bytes();
                    chunk.copy_from_slice(&b[..chunk.len()]);
                }
            }
        }
    }
    v
}

fn put(dir: &PathBuf, name: &str, n: &mut usize, bytes: &[u8]) {
    let d = dir.join(name);
    fs::create_dir_all(&d).unwrap();
    fs::write(d.join(format!("{n:05}")), bytes).unwrap();
    *n += 1;
}

fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("OUT_DIR"));
    let mut rng = Rng(0x2026_1030_0216_0001);
    let mut n = 0usize;
    // Stride 12 gets every count class (the owned SSSE3 stride-12 kernel picks
    // by count <= 32), others a spread of counts around group boundaries.
    for stride in [4, 8, 12, 16, 20, 24, 32, 36, 48, 64, 128, 256] {
        let counts: &[usize] = if stride == 12 {
            &[
                1, 2, 5, 15, 16, 17, 31, 32, 33, 64, 100, 255, 256, 257, 1000, 2048,
            ]
        } else {
            &[1, 7, 16, 17, 33, 100, 255]
        };
        for &count in counts {
            if count * stride > 65_000 {
                continue;
            }
            for style in 0..6 {
                let (version, level) =
                    [(1u8, 2u8), (0, 0), (1, 0), (1, 9), (1, 5)][(style + count) % 5];
                let Ok(encoding) = VertexEncoding::new(version, level) else {
                    continue;
                };
                let data = payload(&mut rng, count, stride, style);
                let mut ws = Workspace::default();
                let Ok(encoded) = encode_vertex_buffer(&data, count, stride, encoding, &mut ws)
                else {
                    continue;
                };
                let out_len = (count * stride).div_ceil(256).clamp(1, 255) as u8;
                let mut seed = Vec::new();
                seed.extend((count as u32).to_le_bytes());
                seed.extend((stride as u16).to_le_bytes());
                seed.extend([0, 0, out_len]);
                seed.extend(&encoded);
                put(&out, "vertex", &mut n, &seed);
                put(&out, "vertex_into", &mut n, &seed);
                for selector in [0u8, 1] {
                    let mut s = vec![selector];
                    s.extend(&seed);
                    put(&out, "simd", &mut n, &s);
                }
            }
        }
    }
    // Index sequences for the streaming decoder.
    for count in [0usize, 1, 3, 8, 31, 32, 100, 128] {
        for style in 0..4 {
            let indices: Vec<u32> = (0..count)
                .map(|i| match style {
                    0 => i as u32,
                    1 => (i as u32).wrapping_mul(2654435761) >> 20,
                    2 => 5,
                    _ => ((i * 7) % 13) as u32 + (i as u32 / 5) * 100_000,
                })
                .collect();
            let mut ws = Workspace::default();
            for version in [1u8, 0] {
                let Ok(e) = IndexEncoding::new(version) else {
                    continue;
                };
                let Ok(encoded) = encode_index_sequence(&indices, e, &mut ws) else {
                    continue;
                };
                for (chunk, width) in [(0u8, 0u8), (3, 4), (16, 7), (1, 1)] {
                    let mut seed = vec![count as u8, chunk, width];
                    seed.extend(&encoded);
                    put(&out, "sequence_stream", &mut n, &seed);
                }
                // Declared count one off the encoded count exercises the error paths.
                let mut seed = vec![(count as u8).wrapping_add(1), 5, 2];
                seed.extend(&encoded);
                put(&out, "sequence_stream", &mut n, &seed);
            }
        }
    }
    println!("seeds written: {n}");
}
