//! Independent seeded mutation smoke for every preprocessing function/option.
use std::time::{Duration, Instant};
const NAMES: [&str; 32] = [
    "vertex_cache_strip",
    "vertex_cache_fifo",
    "generate_vertex_remap",
    "generate_vertex_remap_multi",
    "generate_vertex_remap_custom",
    "remap_vertex_buffer",
    "remap_index_buffer",
    "filter_index_buffer",
    "filter_index_buffer_multi",
    "generate_shadow_index_buffer",
    "generate_shadow_index_buffer_multi",
    "generate_position_remap",
    "generate_adjacency_index_buffer",
    "generate_tessellation_index_buffer",
    "generate_provoking_index_buffer",
    "vertex_fetch",
    "vertex_fetch_remap",
    "simplify_sloppy",
    "simplify_prune",
    "simplify_points",
    "simplify_with_update",
    "quantize_unorm",
    "quantize_snorm",
    "quantize_half",
    "quantize_float",
    "dequantize_half",
    "compute_position_exponent",
    "simplify_sparse",
    "simplify_prune_option",
    "simplify_preserve_folds",
    "simplify_error_clamped",
    "simplify_regularize_light",
];
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }
}
fn word(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub fn run(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let name = target;
    let op = NAMES
        .iter()
        .position(|&n| n == name)
        .ok_or("unknown target")? as u32
        + 7;
    let args: Vec<_> = std::env::args().collect();
    let seconds: u64 = args.get(1).map_or(Ok(300), |s| s.parse())?;
    let seed: u64 = args
        .get(2)
        .map_or(Ok(20261003 + u64::from(op)), |s| s.parse())?;
    if seed == 0 {
        return Err("zero seed".into());
    }
    let mut rng = Rng(seed);
    let start = Instant::now();
    let mut executions = 0u64;
    let mut successes = 0u64;
    while start.elapsed() < Duration::from_secs(seconds) {
        let vc = if (28..=32).contains(&op) {
            0
        } else if op == 33 {
            2
        } else {
            rng.next() % 33
        };
        let ic = if op == 33 || (vc == 0 && op < 28) {
            0
        } else {
            (rng.next() % 33) * 3
        };
        let size = 1 + rng.next() % 32;
        let stride = size
            + if matches!(op, 9 | 12 | 22) {
                0
            } else {
                rng.next() % 17
            };
        let sc = if matches!(op, 9 | 10 | 12 | 14 | 15 | 16 | 17 | 22) {
            if matches!(op, 10 | 15 | 17) {
                1 + rng.next() % 4
            } else {
                1
            }
        } else {
            0
        };
        let ac = if op == 27 || op >= 34 {
            rng.next() % 9
        } else {
            0
        };
        let mut options = if op >= 34 {
            [2, 8, 128, 256, 64][(op - 34) as usize]
        } else if op == 27 {
            rng.next() & 511
        } else {
            0
        };
        if op != 27 && !(28..=33).contains(&op) && rng.next() & 1 != 0 {
            options |= 0x80000000;
        }
        let target = if op == 28 {
            rng.next() % 31
        } else if op == 29 {
            1 + rng.next() % 31
        } else if op == 31 {
            rng.next() % 24
        } else if op == 26 {
            rng.next() % (vc + 1)
        } else {
            rng.next() % (ic + 1)
        };
        let p0 = if op == 8 {
            3 + rng.next() % 64
        } else if op == 33 {
            (-10i32) as u32
        } else {
            [0.0f32, 0.001, 0.1, 1.0][rng.next() as usize % 4].to_bits()
        };
        let p1 = if op == 33 { 16 } else { rng.next() & 3 };
        let mut input = b"MO02".to_vec();
        for v in [
            op, vc, ic, 0, 0, size, stride, sc, options, target, p0, p1, ac,
        ] {
            word(&mut input, v);
        }
        for i in 0..vc {
            for k in 0..3 {
                let v = if op == 33 {
                    if i == 0 {
                        -1.0
                    } else {
                        1.0
                    }
                } else {
                    (rng.next() % 17) as f32 / 4.0
                };
                let raw = if executions.is_multiple_of(7) && op != 33 {
                    rng.next()
                } else {
                    v.to_bits()
                };
                let _ = k;
                word(&mut input, raw);
            }
        }
        for _ in 0..ic {
            word(
                &mut input,
                if (28..=32).contains(&op) {
                    rng.next()
                } else {
                    rng.next() % vc.max(1)
                },
            );
        }
        for _ in 0..sc * vc * stride {
            input.push(rng.next() as u8);
        }
        for _ in 0..ac {
            word(
                &mut input,
                if executions.is_multiple_of(11) {
                    rng.next()
                } else {
                    [0.0f32, 0.1, 1.0, 2.0][rng.next() as usize % 4].to_bits()
                },
            );
        }
        for _ in 0..vc * ac {
            word(
                &mut input,
                if executions.is_multiple_of(13) {
                    rng.next()
                } else {
                    ((rng.next() % 17) as f32).to_bits()
                },
            );
        }
        for _ in 0..vc {
            word(
                &mut input,
                if op == 24 {
                    rng.next() & 1
                } else {
                    rng.next() & 7
                },
            );
        }
        if executions.is_multiple_of(3) {
            let position = 8 + rng.next() as usize % (input.len() - 8);
            input[position] ^= 1 << (rng.next() % 8);
        }
        let result = meshoptimizer_parity::execute(&input);
        let replay = meshoptimizer_parity::execute(&input);
        if result != replay {
            return Err(format!("determinism seed={seed} execution={executions}").into());
        }
        if result.is_ok() {
            successes += 1;
        }
        executions += 1;
    }
    println!("{{\"target\":\"{name}\",\"seed\":{seed},\"seconds\":{},\"executions\":{executions},\"successes\":{successes},\"crashes\":0}}",start.elapsed().as_secs_f64());
    Ok(())
}
