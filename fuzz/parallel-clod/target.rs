#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{clusterlod as c, Error, Limits, Workspace};
use std::sync::OnceLock;
static POOLS: OnceLock<Vec<rayon::ThreadPool>> = OnceLock::new();
fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let n = 1 + (data[0] as usize % 48);
    let p: Vec<[f32; 3]> = (0..n)
        .map(|i| {
            std::array::from_fn(|j| {
                let k = 8 + (i * 3 + j) * 4;
                if let Some(b) = data.get(k..k + 4) {
                    f32::from_le_bytes(b.try_into().unwrap())
                } else {
                    (i % 7) as f32
                }
            })
        })
        .collect();
    let i: Vec<u32> = data[8..]
        .iter()
        .take(192)
        .map(|b| u32::from(*b) % (n as u32 + u32::from(data[1] & 1)))
        .collect();
    let mut cfg = c::default_config(4 + (data[2] % 29) as usize).unwrap();
    cfg.partition_size = 1 + (data[3] % 4) as usize;
    cfg.simplify_dilate_borders = data[4] & 1 != 0;
    cfg.cluster_spatial = data[4] & 2 != 0;
    cfg.optimize_bounds = data[4] & 4 != 0;
    let limits = Limits {
        max_bytes: if data[5] & 1 == 0 {
            usize::MAX
        } else {
            (data[5] as usize) * 128
        },
        max_work: if data[6] & 1 == 0 {
            u64::MAX
        } else {
            u64::from(data[6]) * 64
        },
    };
    let fail = data[7] as usize;
    let run = |parallel: bool| {
        let mut ws = Workspace::new(limits);
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut p = p.clone();
            let mut words = Vec::new();
            let mut count = 0;
            let mesh = c::Mesh {
                indices: &i,
                positions: &mut p,
                attributes: None,
                vertex_lock: None,
                attribute_weights: &[],
                attribute_protect_mask: 0,
            };
            let callback = |g: c::Group, cs: &[c::Cluster]| {
                words.extend([g.depth as u32, cs.len() as u32]);
                words.extend(g.simplified.center.map(f32::to_bits));
                words.extend([g.simplified.radius.to_bits(), g.simplified.error.to_bits()]);
                for cl in cs {
                    words.extend([
                        cl.refined as u32,
                        cl.vertex_count as u32,
                        cl.indices.len() as u32,
                    ]);
                    words.extend(cl.bounds.center.map(f32::to_bits));
                    words.extend([cl.bounds.radius.to_bits(), cl.bounds.error.to_bits()]);
                    words.extend(&cl.indices);
                }
                count += 1;
                if count == fail {
                    Err(Error::InvalidParameter)
                } else {
                    Ok(100 + count as i32 * 3)
                }
            };
            let r = if parallel {
                c::build_parallel_with_output(cfg, mesh, callback, &mut ws)
            } else {
                c::build_with_output(cfg, mesh, callback, &mut ws)
            };
            observations.push((
                r,
                words,
                p.iter()
                    .flat_map(|p| p.map(f32::to_bits))
                    .collect::<Vec<_>>(),
                ws.usage(),
            ));
        }
        observations
    };
    let pools = POOLS.get_or_init(|| {
        [1, 2, 4, 8, 32]
            .into_iter()
            .map(|n| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(n)
                    .build()
                    .unwrap()
            })
            .collect()
    });
    let expected = run(false);
    assert_eq!(
        expected,
        pools[data[1] as usize % pools.len()].install(|| run(true))
    );
});
