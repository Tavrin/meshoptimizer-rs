//! Stable seeded mutation smoke targets; errors and invariant failures are retained.
use meshoptimizer_rs::*;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }
}
/// Run one bounded target for the requested elapsed budget and emit JSON statistics.
pub fn run(overdraw: bool) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let seconds: u64 = args.get(1).map_or(Ok(600), |s| s.parse())?;
    let seed: u64 = args.get(2).map_or(Ok(20261002), |s| s.parse())?;
    if seed == 0 {
        return Err("seed must be nonzero".into());
    }
    let count: u64 = args.get(3).map_or(Ok(u64::MAX), |s| s.parse())?;
    let current = Arc::new(AtomicU64::new(0));
    let hook_counter = current.clone();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!(
            "seed={seed} execution={} overdraw={overdraw}: {info}",
            hook_counter.load(Ordering::Relaxed)
        );
    }));
    let mut rng = Rng(seed);
    let start = Instant::now();
    let mut executions = 0u64;
    let mut successes = 0u64;
    while start.elapsed() < Duration::from_secs(seconds) && executions < count {
        current.store(executions, Ordering::Relaxed);
        let v = (rng.next() % 65) as usize;
        let n = (rng.next() % 257) as usize;
        let mut p = vec![[0f32; 3]; v];
        for point in &mut p {
            for x in point {
                let b = rng.next();
                *x = if executions.is_multiple_of(4) {
                    f32::from_bits(b)
                } else {
                    (b as i32 % 10000) as f32 / 100.
                };
            }
        }
        let mut indices = vec![0u32; n];
        for index in &mut indices {
            *index = if v == 0 {
                0
            } else if executions.is_multiple_of(5) {
                rng.next()
            } else {
                rng.next() % v as u32
            };
        }
        if !executions.is_multiple_of(3) {
            indices.truncate(n / 3 * 3);
        }
        let max_work = if executions.is_multiple_of(7) {
            u64::from(rng.next() % 5000)
        } else {
            1 << 20
        };
        let max_bytes = if executions.is_multiple_of(11) {
            (rng.next() % 4096) as usize
        } else {
            1 << 20
        };
        let limits = Limits {
            max_bytes,
            max_work,
        };
        let mut ws = Workspace::new(limits);
        let threshold = if executions.is_multiple_of(6) {
            f32::from_bits(rng.next())
        } else {
            [0., 0.5, 1., 1.05, 1.1, 2.][(rng.next() % 6) as usize]
        };
        let result = if overdraw {
            optimize_overdraw(&indices, Positions::from_packed(&p), threshold, &mut ws)
        } else {
            optimize_vertex_cache(&indices, v, &mut ws)
        };
        if let Ok(ref out) = result {
            successes += 1;
            let mut a: Vec<_> = indices.as_chunks::<3>().0.to_vec();
            let mut b: Vec<_> = out.as_chunks::<3>().0.to_vec();
            a.sort();
            b.sort();
            if a != b || indices.len() != out.len() {
                return Err(
                    format!("triangle invariant seed={seed} execution={executions}").into(),
                );
            }
            if ws.usage().bytes > max_bytes || ws.usage().work > max_work {
                return Err(
                    format!("resource invariant seed={seed} execution={executions}").into(),
                );
            }
        }
        let original = indices.clone();
        let atomic = if overdraw {
            optimize_overdraw_in_place(
                &mut indices,
                Positions::from_packed(&p),
                threshold,
                &mut Workspace::new(limits),
            )
        } else {
            optimize_vertex_cache_in_place(&mut indices, v, &mut Workspace::new(limits))
        };
        if atomic.is_err() && indices != original {
            return Err(format!("atomic error seed={seed} execution={executions}").into());
        }
        if let Ok(out) = result {
            if atomic.is_err() || out != indices {
                return Err(format!("variant mismatch seed={seed} execution={executions}").into());
            }
        }
        let mut destination = vec![u32::MAX; original.len() + 1];
        let into = if overdraw {
            optimize_overdraw_into(
                &mut destination,
                &original,
                Positions::from_packed(&p),
                threshold,
                &mut Workspace::new(limits),
            )
        } else {
            optimize_vertex_cache_into(&mut destination, &original, v, &mut Workspace::new(limits))
        };
        if destination[original.len()] != u32::MAX {
            return Err(format!("destination tail seed={seed} execution={executions}").into());
        }
        if atomic.is_ok() && (into.is_err() || destination[..original.len()] != indices) {
            return Err(
                format!("caller-buffer mismatch seed={seed} execution={executions}").into(),
            );
        }
        let count = if executions.is_multiple_of(2) {
            usize::MAX
        } else {
            v
        };
        let _ = Positions::from_interleaved(
            &[],
            count,
            (rng.next() % 70) as usize,
            rng.next() as usize,
        );
        let _ = Positions::from_bytes(
            &[],
            count,
            (rng.next() % 270) as usize,
            rng.next() as usize,
            ByteOrder::BigEndian,
        );
        let _ = Attributes::from_interleaved(
            &[],
            count,
            (rng.next() % 35) as usize,
            (rng.next() % 70) as usize,
            0,
        );
        let _ = Attributes::from_bytes(
            &[],
            count,
            (rng.next() % 35) as usize,
            (rng.next() % 270) as usize,
            rng.next() as usize,
            ByteOrder::LittleEndian,
        );
        let _ = VertexFlags::from_bits(rng.next() as u8);
        executions += 1;
    }
    println!("{{\"target\":\"{}\",\"seconds\":{},\"executions\":{},\"successes\":{},\"crashes\":0,\"seed\":{}}}",if overdraw {"overdraw"} else {"vertex_cache"},start.elapsed().as_secs_f64(),executions,successes,seed);
    Ok(())
}

/// Mutate simplification parameters, layouts, attributes and flags for a bounded budget.
pub fn run_simplifier(kind: u32) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let seconds: u64 = args.get(1).map_or(Ok(600), |s| s.parse())?;
    let seed: u64 = args.get(2).map_or(Ok(20261002), |s| s.parse())?;
    let limit: u64 = args.get(3).map_or(Ok(u64::MAX), |s| s.parse())?;
    if seed == 0 {
        return Err("nonzero seed required".into());
    }
    let current = Arc::new(AtomicU64::new(0));
    let hook = current.clone();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!(
            "seed={seed} execution={} kind={kind}: {info}",
            hook.load(Ordering::Relaxed)
        )
    }));
    let mut rng = Rng(seed);
    let start = Instant::now();
    let mut executions = 0u64;
    let mut successes = 0u64;
    while start.elapsed() < Duration::from_secs(seconds) && executions < limit {
        current.store(executions, Ordering::Relaxed);
        let n = (rng.next() % 49) as usize;
        let ac = if kind == 5 {
            (rng.next() % 33) as usize
        } else {
            0
        };
        let p: Vec<[f32; 3]> = (0..n)
            .map(|i| [i as f32 % 7., (i / 7) as f32, (rng.next() % 8) as f32 / 16.])
            .collect();
        let mut p = p;
        if executions.is_multiple_of(4) {
            for v in &mut p {
                for x in v {
                    *x = f32::from_bits(rng.next());
                }
            }
        }
        if executions.is_multiple_of(9) {
            for i in 1..n {
                p[i] = p[i % 3];
            }
        }
        let mut ib = Vec::new();
        if n > 8 {
            for i in 0..n - 8 {
                ib.extend_from_slice(&[
                    i as u32,
                    i as u32 + 1,
                    i as u32 + 7,
                    i as u32 + 1,
                    i as u32 + 8,
                    i as u32 + 7,
                ]);
            }
        }
        if executions.is_multiple_of(5) && !ib.is_empty() {
            ib[0] = rng.next();
        }
        if executions.is_multiple_of(7) {
            ib.push(0);
        }
        let data: Vec<f32> = (0..n * ac)
            .map(|_| {
                if executions.is_multiple_of(6) {
                    f32::from_bits(rng.next())
                } else {
                    (rng.next() % 100) as f32 / 100.
                }
            })
            .collect();
        let weights: Vec<f32> = (0..ac)
            .map(|_| {
                if executions.is_multiple_of(11) {
                    f32::from_bits(rng.next())
                } else {
                    (rng.next() % 8) as f32 / 4.
                }
            })
            .collect();
        let flags: Vec<VertexFlags> = (0..n)
            .map(|_| VertexFlags::from_bits((rng.next() % 8) as u8))
            .collect::<Result<_, _>>()?;
        let options =
            SimplifyOptions::from_bits([0, 1, 4, 16, 32, 64, 33, 36][rng.next() as usize % 8])?;
        let settings = SimplifySettings {
            target_index_count: rng.next() as usize % (ib.len() + 2),
            target_error: if executions.is_multiple_of(8) {
                f32::from_bits(rng.next())
            } else {
                (rng.next() % 100) as f32 / 100.
            },
            options,
        };
        let limits = Limits {
            max_bytes: if executions.is_multiple_of(13) {
                (rng.next() % 4096) as usize
            } else {
                1 << 20
            },
            max_work: if executions.is_multiple_of(17) {
                (rng.next() % 4096).into()
            } else {
                1 << 20
            },
        };
        let mut ws = Workspace::new(limits);
        let p = Positions::from_packed(&p);
        if kind == 6 {
            if let Ok(scale) = simplify_scale(p) {
                successes += 1;
                if !scale.is_finite() || scale < 0. {
                    return Err(
                        format!("scale invariant seed={seed} execution={executions}").into(),
                    );
                }
            }
        } else {
            let a = Attributes::from_interleaved(&data, n, ac, ac, 0)?;
            let out = if kind == 4 {
                simplify(&ib, p, settings, &mut ws)
            } else {
                simplify_with_attributes(&ib, p, a, &weights, Some(&flags), settings, &mut ws)
            };
            let mut dest = vec![u32::MAX; ib.len() + 1];
            let into = if kind == 4 {
                simplify_into(&mut dest, &ib, p, settings, &mut Workspace::new(limits))
            } else {
                simplify_with_attributes_into(
                    &mut dest,
                    &ib,
                    p,
                    a,
                    &weights,
                    Some(&flags),
                    settings,
                    &mut Workspace::new(limits),
                )
            };
            if dest[ib.len()] != u32::MAX {
                return Err(format!("tail invariant seed={seed} execution={executions}").into());
            }
            if let Ok(out) = out {
                successes += 1;
                if out.indices.len() > ib.len()
                    || !out.indices.len().is_multiple_of(3)
                    || out.indices.iter().any(|&i| i as usize >= n)
                    || !out.error.is_finite()
                    || ws.usage().bytes > limits.max_bytes
                    || ws.usage().work > limits.max_work
                {
                    return Err(
                        format!("output invariant seed={seed} execution={executions}").into(),
                    );
                }
                let into = into?;
                if out.indices != dest[..into.index_count]
                    || out.error.to_bits() != into.error.to_bits()
                {
                    return Err(
                        format!("variant identity seed={seed} execution={executions}").into(),
                    );
                }
            }
        }
        let _ = SimplifyOptions::from_bits(rng.next());
        let _ = Attributes::from_interleaved(
            &data,
            n,
            (rng.next() % 35) as usize,
            (rng.next() % 70) as usize,
            rng.next() as usize,
        );
        executions += 1;
    }
    let name = match kind {
        4 => "simplify",
        5 => "simplify_with_attributes",
        _ => "simplify_scale",
    };
    println!("{{\"target\":\"{name}\",\"seconds\":{},\"executions\":{executions},\"successes\":{successes},\"crashes\":0,\"seed\":{seed}}}",start.elapsed().as_secs_f64());
    Ok(())
}
