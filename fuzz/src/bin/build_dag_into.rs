//! Seeded safe mutation target for caller-buffer DAG construction.
use meshoptimizer_rs::{clusterlod::*, Attributes, Limits, Workspace};
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
    fn pick(&mut self, n: usize) -> usize {
        self.next() as usize % n
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let seconds: u64 = args.get(1).map_or(Ok(600), |v| v.parse())?;
    let seed: u64 = args.get(2).map_or(Ok(20261025), |v| v.parse())?;
    let count: u64 = args.get(3).map_or(Ok(u64::MAX), |v| v.parse())?;
    assert_ne!(seed, 0);
    let current = Arc::new(AtomicU64::new(0));
    let hook = current.clone();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!(
            "build_dag_into seed={seed} execution={}: {info}",
            hook.load(Ordering::Relaxed)
        );
    }));
    let mut rng = Rng(seed);
    let started = Instant::now();
    let (mut executions, mut successes, mut too_small) = (0u64, 0u64, 0u64);
    while started.elapsed() < Duration::from_secs(seconds) && executions < count {
        current.store(executions, Ordering::Relaxed);
        let side = 2 + rng.pick(16);
        let nv = side * side;
        let mut positions: Vec<_> = (0..nv)
            .map(|i| {
                [
                    (i % side) as f32,
                    (i / side) as f32,
                    (rng.pick(20) as f32) * 0.01,
                ]
            })
            .collect();
        if executions.is_multiple_of(7) {
            for p in &mut positions {
                for v in p {
                    *v = f32::from_bits(rng.next());
                }
            }
        }
        let mut indices = Vec::new();
        for y in 0..side - 1 {
            for x in 0..side - 1 {
                let a = (y * side + x) as u32;
                indices.extend([
                    a,
                    a + 1,
                    a + side as u32,
                    a + 1,
                    a + side as u32 + 1,
                    a + side as u32,
                ]);
            }
        }
        if executions.is_multiple_of(5) {
            for i in &mut indices {
                *i = rng.pick(nv) as u32;
            }
        }
        if executions.is_multiple_of(13) {
            indices[0] = nv as u32;
        }
        if executions.is_multiple_of(17) {
            indices.pop();
        }
        let mut config = default_config([4, 8, 32, 64, 128, 256][rng.pick(6)])?;
        config.simplify_dilate_borders = rng.pick(2) == 0;
        config.partition_sort = rng.pick(2) == 0;
        config.cluster_spatial = rng.pick(2) == 0;
        config.optimize_clusters = rng.pick(2) == 0;
        config.simplify_fallback_permissive = rng.pick(2) == 0;
        config.simplify_fallback_sloppy = rng.pick(2) == 0;
        if executions.is_multiple_of(19) {
            config.simplify_ratio = f32::from_bits(rng.next());
        }
        if executions.is_multiple_of(23) {
            config.max_vertices = rng.pick(300);
        }
        let node_width = [2, 4, 8, 16, 0, 1, 3][rng.pick(7)];
        let attributes: Vec<f32> = (0..nv * 9).map(|_| rng.pick(10) as f32 * 0.1).collect();
        let weights = [0.5, 0.5, 0.5, 0., 0., 0., 0., 0., 0.];
        let use_attrs = rng.pick(2) == 0;
        let attrs = Attributes::from_interleaved(&attributes, nv, 9, 9, 0)?;
        let mask = if use_attrs {
            (1 << 3) | (1 << 4) | (1 << 8)
        } else {
            0
        };
        let limits = Limits {
            max_bytes: if executions.is_multiple_of(11) {
                rng.pick(65536)
            } else {
                32 << 20
            },
            max_work: if executions.is_multiple_of(29) {
                rng.next() as u64 % 1000
            } else {
                1 << 28
            },
        };
        let full = !executions.is_multiple_of(3);
        let mut cap = |n| if full { n } else { rng.pick(n + 1) };
        let capacities = [
            cap(nv * 16),
            cap(nv * 16),
            cap(indices.len() * 8),
            cap(4096),
            cap(4096),
            cap(8192),
        ];
        let mut vs = vec![u32::MAX; capacities[0] + 1];
        let mut vp = vec![[f32::from_bits(0x7f123456); 3]; capacities[1] + 1];
        let mut ix = vec![u32::MAX; capacities[2] + 1];
        let mut sentinel_c = FlatDagCluster::default();
        sentinel_c.first_index = u32::MAX;
        let mut sentinel_g = FlatDagGroup::default();
        sentinel_g.first_cluster = u32::MAX;
        let sentinel_n = Node {
            child_offset: u32::MAX,
            ..Node::default()
        };
        let mut cl = vec![sentinel_c; capacities[3] + 1];
        let mut gr = vec![sentinel_g; capacities[4] + 1];
        let mut nd = vec![sentinel_n; capacities[5] + 1];
        let mut ws = Workspace::new(limits);
        let result = build_dag_into(
            config,
            Mesh {
                indices: &indices,
                positions: &mut positions,
                attributes: if use_attrs { Some(attrs) } else { None },
                vertex_lock: None,
                attribute_weights: if use_attrs { &weights } else { &[] },
                attribute_protect_mask: mask,
            },
            node_width,
            DagBuffers {
                vertex_source: &mut vs[..capacities[0]],
                vertex_positions: &mut vp[..capacities[1]],
                indices: &mut ix[..capacities[2]],
                clusters: &mut cl[..capacities[3]],
                groups: &mut gr[..capacities[4]],
                nodes: &mut nd[..capacities[5]],
            },
            &mut ws,
        );
        assert_eq!(vs[capacities[0]], u32::MAX);
        assert_eq!(vp[capacities[1]].map(f32::to_bits), [0x7f123456; 3]);
        assert_eq!(ix[capacities[2]], u32::MAX);
        assert_eq!(cl[capacities[3]].first_index, u32::MAX);
        assert_eq!(gr[capacities[4]].first_cluster, u32::MAX);
        assert_eq!(nd[capacities[5]].child_offset, u32::MAX);
        assert!(ws.usage().bytes <= limits.max_bytes && ws.usage().work <= limits.max_work);
        if let Ok(c) = result {
            successes += 1;
            assert!(c.vertices <= capacities[0].min(capacities[1]));
            assert!(
                c.indices <= capacities[2]
                    && c.clusters <= capacities[3]
                    && c.groups <= capacities[4]
                    && c.nodes <= capacities[5]
            );
            assert!(vs[..c.vertices].iter().all(|&v| (v as usize) < nv));
            assert!(ix[..c.indices].iter().all(|&v| (v as usize) < c.vertices));
            assert!(vs[c.vertices..].iter().all(|&v| v == u32::MAX));
            assert!(vp[c.vertices..]
                .iter()
                .all(|v| v.map(f32::to_bits) == [0x7f123456; 3]));
            assert!(ix[c.indices..].iter().all(|&v| v == u32::MAX));
            assert!(cl[c.clusters..].iter().all(|v| v.first_index == u32::MAX));
            assert!(gr[c.groups..].iter().all(|v| v.first_cluster == u32::MAX));
            assert!(nd[c.nodes..].iter().all(|v| v.child_offset == u32::MAX));
            for v in &cl[..c.clusters] {
                assert!((v.first_index as usize) + v.index_count as usize <= c.indices);
                assert!((v.group as usize) < c.groups && v.index_count.is_multiple_of(3));
                assert!(v.refined < 0 || (v.refined as usize) < c.groups);
            }
            for g in &gr[..c.groups] {
                assert!((g.first_cluster as usize) + g.cluster_count as usize <= c.clusters);
            }
            for n in &nd[..c.nodes] {
                if n.group < 0 {
                    assert!((n.child_offset as usize) + n.child_count as usize <= c.nodes);
                } else {
                    assert!((n.group as usize) < c.groups);
                }
            }
        } else if matches!(result, Err(meshoptimizer_rs::Error::BufferTooSmall)) {
            too_small += 1;
        }
        executions += 1;
    }
    println!("{{\"executions\":{executions},\"successes\":{successes},\"too_small\":{too_small},\"elapsed_seconds\":{}}}", started.elapsed().as_secs_f64());
    Ok(())
}
