use baseline::{
    clusterlod::{dag_format::*, FlatDagCluster, FlatDagGroup},
    Workspace,
};
pub fn run() -> Vec<u8> {
    let mut groups = Vec::new();
    let mut clusters = Vec::new();
    let mut indices = Vec::new();
    for g in 0..3 {
        let mut group = FlatDagGroup::default();
        group.first_cluster = g;
        group.cluster_count = 1;
        group.depth = g as i32;
        group.error = if g == 2 { f32::MAX } else { g as f32 };
        groups.push(group);
        let mut c = FlatDagCluster::default();
        c.group = g;
        c.refined = g as i32 - 1;
        c.first_index = g * 6;
        c.index_count = 6;
        clusters.push(c);
        indices.extend([0, 1, 2, 2, 1, 0]);
    }
    let b = write(
        &groups,
        &clusters,
        &[[0., 0., -0.], [1., 2., 3.], [4., 5., 6.]],
        &indices,
        128,
        &mut Workspace::default(),
    )
    .unwrap();
    let mut b = b;
    for n in [4, 12, 24] {
        b.extend(grid_bytes(n));
    }
    b
}

fn grid_bytes(n: usize) -> Vec<u8> {
    use baseline::{clusterlod::*, Workspace};
    let mut p: Vec<[f32; 3]> = (0..=n)
        .flat_map(|y| {
            (0..=n).map(move |x| [x as f32, y as f32, ((x * 17 + y * 13) % 7) as f32 * 0.125])
        })
        .collect();
    let mut ix = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let a = (y * (n + 1) + x) as u32;
            let b = a + (n + 1) as u32;
            ix.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    let cap = ix.len() * 8;
    let mut vs = vec![0; cap];
    let mut vp = vec![[0.; 3]; cap];
    let mut ids = vec![0; cap];
    let mut cs = vec![FlatDagCluster::default(); cap];
    let mut gs = vec![FlatDagGroup::default(); cap];
    let mut ns = vec![Node::default(); cap];
    let mut cfg = default_config(32).unwrap();
    cfg.partition_size = 2;
    let k = build_dag_into(
        cfg,
        Mesh {
            indices: &ix,
            positions: &mut p,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        8,
        DagBuffers {
            vertex_source: &mut vs,
            vertex_positions: &mut vp,
            indices: &mut ids,
            clusters: &mut cs,
            groups: &mut gs,
            nodes: &mut ns,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    let mut out = baseline::clusterlod::dag_format::write(
        &gs[..k.groups],
        &cs[..k.clusters],
        &vp[..k.vertices],
        &ids[..k.indices],
        128 * 1024,
        &mut Workspace::default(),
    )
    .unwrap();
    for v in &vs[..k.vertices] {
        out.extend(v.to_le_bytes());
    }
    for pt in p.iter().chain(vp[..k.vertices].iter()) {
        for x in pt {
            out.extend(x.to_bits().to_le_bytes());
        }
    }
    for g in &gs[..k.groups] {
        for x in g.center.iter().chain([&g.radius, &g.error]) {
            out.extend(x.to_bits().to_le_bytes());
        }
        for x in [g.depth as u32, g.first_cluster, g.cluster_count] {
            out.extend(x.to_le_bytes());
        }
    }
    for c in &cs[..k.clusters] {
        for x in [c.first_index, c.index_count, c.group, c.refined as u32] {
            out.extend(x.to_le_bytes());
        }
        for x in c.center.iter().chain([&c.radius]) {
            out.extend(x.to_bits().to_le_bytes());
        }
    }
    for n in &ns[..k.nodes] {
        for x in n
            .bounds
            .center
            .iter()
            .chain([&n.bounds.radius, &n.bounds.error])
        {
            out.extend(x.to_bits().to_le_bytes());
        }
        for x in [n.group as u32, n.child_offset, n.child_count] {
            out.extend(x.to_le_bytes());
        }
    }
    for x in &ids[..k.indices] {
        out.extend(x.to_le_bytes());
    }
    for x in [
        k.vertices as u64,
        k.indices as u64,
        k.clusters as u64,
        k.groups as u64,
        k.nodes as u64,
        k.level_count as u64,
        k.bvh_depth as u64,
    ] {
        out.extend(x.to_le_bytes());
    }
    out
}
