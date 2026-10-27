//! Unpublished S1 MCLD v1 oracle; wire-format policy stays outside the library.
use meshoptimizer_rs::{clusterlod::*, Attributes, Workspace};
use std::{collections::HashMap, env, fs};
fn word(b: &[u8], at: &mut usize) -> u32 {
    let v = u32::from_le_bytes(b[*at..*at + 4].try_into().unwrap());
    *at += 4;
    v
}
fn put(out: &mut Vec<u8>, v: u32) {
    out.extend(v.to_le_bytes());
}
fn floats(out: &mut Vec<u8>, values: &[f32]) {
    for v in values {
        put(out, v.to_bits());
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let b = fs::read(&args[1])?;
    assert_eq!(&b[..4], b"MMSH");
    let mut at = 4;
    let version = word(&b, &mut at);
    assert!((2..=4).contains(&version));
    let ll = u16::from_le_bytes(b[at..at + 2].try_into()?) as usize;
    at += 2;
    let layout = std::str::from_utf8(&b[at..at + ll])?;
    at += ll;
    let stride = match layout {
        "pos3-norm3-uv2-tangent4" => 12,
        "pos3-norm3-uv2-tangent4-color4" => 16,
        _ => panic!("unsupported layout"),
    };
    let nv = word(&b, &mut at) as usize;
    let ni = word(&b, &mut at) as usize;
    assert!(nv <= 4 << 20 && ni <= 16 << 20);
    if version >= 3 {
        at += 4;
    }
    let vb = u64::from_le_bytes(b[at..at + 8].try_into()?) as usize;
    at += 8;
    assert_eq!(vb, nv * stride * 4);
    let data: Vec<f32> = b[at..at + vb]
        .chunks_exact(4)
        .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    at += vb;
    let ib = u64::from_le_bytes(b[at..at + 8].try_into()?) as usize;
    at += 8;
    assert_eq!(ib, ni * 4);
    let indices: Vec<u32> = b[at..at + ib]
        .chunks_exact(4)
        .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    let original: Vec<[f32; 3]> = data
        .chunks_exact(stride)
        .map(|v| [v[0], v[1], v[2]])
        .collect();
    let mut positions = original.clone();
    let width = stride - 3;
    let mut weights = vec![0.; width];
    weights[..3].fill(0.5);
    let attributes =
        Attributes::from_interleaved(&data, nv, width, stride, 3).map_err(|e| e.to_string())?;
    let dilated = args.get(3).is_some_and(|s| s == "--dilate");
    let mut config = default_config(128).map_err(|e| e.to_string())?;
    config.max_vertices = 128;
    config.simplify_dilate_borders = dilated;
    let mut vs = vec![0; 4 << 20];
    let mut vp = vec![[0.; 3]; 4 << 20];
    let mut ix = vec![0; 16 << 20];
    let mut cl = vec![FlatDagCluster::default(); 1 << 20];
    let mut gr = vec![FlatDagGroup::default(); 1 << 20];
    let mut nd = vec![Node::default(); 1 << 20];
    let counts = build_dag_into(
        config,
        Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: Some(attributes),
            vertex_lock: None,
            attribute_weights: &weights,
            attribute_protect_mask: (1 << 3)
                | (1 << 4)
                | (1 << 8)
                | if stride == 16 { 15 << 9 } else { 0 },
        },
        8,
        DagBuffers {
            vertex_source: &mut vs,
            vertex_positions: &mut vp,
            indices: &mut ix,
            clusters: &mut cl,
            groups: &mut gr,
            nodes: &mut nd,
        },
        &mut Workspace::default(),
    )
    .map_err(|e| e.to_string())?;
    assert!(counts.level_count <= 64 && counts.bvh_depth <= 32);
    let mut hashes = HashMap::new();
    let mut collisions = 0;
    for i in 0..counts.vertices {
        let p = vp[i].map(f32::to_bits);
        let key = [vs[i], p[0], p[1], p[2]];
        let h = ((vs[i] as u64) << 32)
            ^ ((p[0] as u64).wrapping_mul(0x9E3779B97F4A7C15))
            ^ ((p[1] as u64).wrapping_mul(0xC2B2AE3D27D4EB4F))
            ^ ((p[2] as u64).wrapping_mul(0x165667B19E3779F9));
        if hashes.insert(h, key).is_some_and(|old| old != key) {
            collisions += 1;
        }
    }
    let source_positions = (0..counts.vertices)
        .all(|i| vp[i].map(f32::to_bits) == original[vs[i] as usize].map(f32::to_bits));
    let mut out = Vec::new();
    for v in [
        0x444c434d,
        1,
        nv as u32,
        ni as u32,
        u32::from(dilated) | if source_positions { 2 } else { 0 },
        counts.vertices as u32,
        counts.indices as u32,
        counts.clusters as u32,
        counts.groups as u32,
        counts.nodes as u32,
        counts.level_count,
        counts.bvh_depth,
    ] {
        put(&mut out, v);
    }
    for i in 0..counts.vertices {
        put(&mut out, vs[i]);
        floats(&mut out, &vp[i]);
    }
    for &i in &ix[..counts.indices] {
        put(&mut out, i);
    }
    for c in &cl[..counts.clusters] {
        for v in [c.first_index, c.index_count, c.group, c.refined as u32] {
            put(&mut out, v);
        }
        floats(&mut out, &c.center);
        floats(&mut out, &[c.radius]);
    }
    for g in &gr[..counts.groups] {
        floats(&mut out, &g.center);
        floats(&mut out, &[g.radius, g.error]);
        for v in [g.depth as u32, g.first_cluster, g.cluster_count] {
            put(&mut out, v);
        }
    }
    for n in &nd[..counts.nodes] {
        floats(&mut out, &n.bounds.center);
        floats(&mut out, &[n.bounds.radius, n.bounds.error]);
        for v in [n.group as u32, n.child_offset, n.child_count] {
            put(&mut out, v);
        }
    }
    assert!(out.len() <= 256 << 20);
    fs::write(&args[2], &out)?;
    println!("{{\"size\":{},\"collisions\":{collisions}}}", out.len());
    Ok(())
}
