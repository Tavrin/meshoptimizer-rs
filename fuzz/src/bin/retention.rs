//! Replayable random call sequences comparing Reuse and Lean workspaces.
use meshoptimizer_rs::*;
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
fn mesh(result: Result<SimplifiedMesh, Error>) -> Result<(Vec<u32>, u32), Error> {
    result.map(|m| (m.indices, m.error.to_bits()))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let seconds: u64 = args.get(1).map_or(Ok(300), |s| s.parse())?;
    let seed: u64 = args.get(2).map_or(Ok(20261026), |s| s.parse())?;
    let count: u64 = args.get(3).map_or(Ok(u64::MAX), |s| s.parse())?;
    if seed == 0 {
        return Err("nonzero seed required".into());
    }
    let mut rng = Rng(seed);
    let mut reuse = Workspace::default();
    let mut lean = Workspace::default();
    lean.set_retention(Retention::Lean);
    let start = Instant::now();
    let mut executions = 0u64;
    let mut successes = 0u64;
    while start.elapsed() < Duration::from_secs(seconds) && executions < count {
        let n = rng.next() as usize % 257;
        let p: Vec<_> = (0..n)
            .map(|i| {
                [
                    (i % 7) as f32,
                    (i / 7) as f32,
                    (rng.next() % 17) as f32 / 16.,
                ]
            })
            .collect();
        let mut indices = Vec::new();
        for i in 0..n.saturating_sub(8) {
            indices.extend_from_slice(&[
                i as u32,
                i as u32 + 1,
                i as u32 + 7,
                i as u32 + 1,
                i as u32 + 8,
                i as u32 + 7,
            ]);
        }
        let mut pa = p.clone();
        let mut pb = p;
        let mut ia = indices.clone();
        let mut ib = indices;
        let ac = rng.next() as usize % 5;
        let mut aa: Vec<_> = (0..n * ac)
            .map(|_| (rng.next() % 17) as f32 / 16.)
            .collect();
        let mut ab = aa.clone();
        let weights = vec![0.5; ac];
        let steps = 8 + rng.next() % 25;
        for step in 0..steps {
            let op = rng.next() % 8;
            let context = format!("seed={seed} execution={executions} step={step} op={op}");
            // Keep a full input witness for any panic, including the call history
            // identified by seed/execution/step. Normal runs produce only statistics.
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let limits = Limits {
                    max_bytes: 1 << 26,
                    max_work: if rng.next().is_multiple_of(7) {
                        (rng.next() % 4096).into()
                    } else {
                        1 << 24
                    },
                };
                reuse.set_limits(limits);
                lean.set_limits(limits);
                let mut settings = SimplifySettings {
                    target_index_count: rng.next() as usize % (ia.len() + 2),
                    target_error: [0., 0.01, 0.5, 1., -1., f32::NAN][rng.next() as usize % 6],
                    options: SimplifyOptions::from_bits(
                        [0, 1, 2, 4, 8, 16, 32, 64][rng.next() as usize % 8],
                    )
                    .unwrap(),
                };
                // Mostly valid calls, interspersed with invalid topology/positions.
                if rng.next().is_multiple_of(13) && !ia.is_empty() {
                    ia[0] = n as u32;
                    ib[0] = n as u32;
                }
                if rng.next().is_multiple_of(17) && n > 0 {
                    pa[0][0] = f32::from_bits(rng.next());
                    pb[0][0] = pa[0][0];
                }
                let (a, b) = match op {
                    0 => (
                        mesh(simplify(
                            &ia,
                            Positions::from_packed(&pa),
                            settings,
                            &mut reuse,
                        )),
                        mesh(simplify(
                            &ib,
                            Positions::from_packed(&pb),
                            settings,
                            &mut lean,
                        )),
                    ),
                    1 => (
                        mesh(simplify_with_attributes(
                            &ia,
                            Positions::from_packed(&pa),
                            Attributes::from_interleaved(&aa, n, ac, ac, 0).unwrap(),
                            &weights,
                            None,
                            settings,
                            &mut reuse,
                        )),
                        mesh(simplify_with_attributes(
                            &ib,
                            Positions::from_packed(&pb),
                            Attributes::from_interleaved(&ab, n, ac, ac, 0).unwrap(),
                            &weights,
                            None,
                            settings,
                            &mut lean,
                        )),
                    ),
                    2 => {
                        // Update mutates the topology, positions and attributes in each chain.
                        settings.options = SimplifyOptions::EMPTY;
                        let a = simplify_with_update(
                            &mut ia,
                            &mut PositionsMut::from_packed(&mut pa),
                            Some(
                                &mut AttributesMut::from_interleaved(&mut aa, n, ac, ac, 0)
                                    .unwrap(),
                            ),
                            &weights,
                            None,
                            settings,
                            &mut reuse,
                        )
                        .map(|r| (r.index_count, r.error.to_bits()));
                        let b = simplify_with_update(
                            &mut ib,
                            &mut PositionsMut::from_packed(&mut pb),
                            Some(
                                &mut AttributesMut::from_interleaved(&mut ab, n, ac, ac, 0)
                                    .unwrap(),
                            ),
                            &weights,
                            None,
                            settings,
                            &mut lean,
                        )
                        .map(|r| (r.index_count, r.error.to_bits()));
                        assert_eq!(a, b, "{context}");
                        if let Ok((count, _)) = a {
                            ia.truncate(count);
                            ib.truncate(count);
                        }
                        (
                            a.map(|(_, e)| (ia.clone(), e)),
                            b.map(|(_, e)| (ib.clone(), e)),
                        )
                    }
                    3 => (
                        mesh(simplify_sloppy(
                            &ia,
                            Positions::from_packed(&pa),
                            None,
                            settings.target_index_count,
                            settings.target_error,
                            &mut reuse,
                        )),
                        mesh(simplify_sloppy(
                            &ib,
                            Positions::from_packed(&pb),
                            None,
                            settings.target_index_count,
                            settings.target_error,
                            &mut lean,
                        )),
                    ),
                    4 => (
                        simplify_prune(
                            &ia,
                            Positions::from_packed(&pa),
                            settings.target_error,
                            &mut reuse,
                        )
                        .map(|v| (v, 0)),
                        simplify_prune(
                            &ib,
                            Positions::from_packed(&pb),
                            settings.target_error,
                            &mut lean,
                        )
                        .map(|v| (v, 0)),
                    ),
                    5 => (
                        simplify_points(
                            Positions::from_packed(&pa),
                            None,
                            0.,
                            settings.target_index_count % (n + 1),
                            &mut reuse,
                        )
                        .map(|v| (v, 0)),
                        simplify_points(
                            Positions::from_packed(&pb),
                            None,
                            0.,
                            settings.target_index_count % (n + 1),
                            &mut lean,
                        )
                        .map(|v| (v, 0)),
                    ),
                    6 => (
                        optimize_vertex_cache(&ia, n, &mut reuse).map(|v| (v, 0)),
                        optimize_vertex_cache(&ib, n, &mut lean).map(|v| (v, 0)),
                    ),
                    _ => (
                        optimize_overdraw(&ia, Positions::from_packed(&pa), 1.05, &mut reuse)
                            .map(|v| (v, 0)),
                        optimize_overdraw(&ib, Positions::from_packed(&pb), 1.05, &mut lean)
                            .map(|v| (v, 0)),
                    ),
                };
                assert_eq!(a, b, "{context}");
                assert_eq!(reuse.usage().work, lean.usage().work, "{context}");
                assert_eq!(ia, ib, "{context}");
                assert!(
                    pa.iter()
                        .flatten()
                        .map(|x| x.to_bits())
                        .eq(pb.iter().flatten().map(|x| x.to_bits())),
                    "{context}"
                );
                assert!(
                    aa.iter()
                        .map(|x| x.to_bits())
                        .eq(ab.iter().map(|x| x.to_bits())),
                    "{context}"
                );
                if let Ok((out, _)) = a {
                    successes += 1;
                    if op != 5 {
                        ia = out.clone();
                        ib = out;
                    }
                }
            }));
            if let Err(payload) = outcome {
                eprintln!(
                    "{context}\nindices={ia:?}\nposition_bits={:?}\nattribute_bits={:?}",
                    pa.iter().flatten().map(|x| x.to_bits()).collect::<Vec<_>>(),
                    aa.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                );
                std::panic::resume_unwind(payload);
            }
        }
        executions += 1;
    }
    println!("{{\"target\":\"retention\",\"seed\":{seed},\"seconds\":{},\"executions\":{executions},\"successes\":{successes},\"crashes\":0}}", start.elapsed().as_secs_f64());
    Ok(())
}
