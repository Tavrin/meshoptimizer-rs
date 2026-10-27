//! Unpublished S1 MCLD v1 oracle; wire-format policy stays outside the library.
use meshoptimizer_rs::{clusterlod::*, Attributes, Workspace};
use std::io::{self, Read, Write};
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
struct Buffers {
    vs: Vec<u32>,
    vp: Vec<[f32; 3]>,
    ix: Vec<u32>,
    cl: Vec<FlatDagCluster>,
    gr: Vec<FlatDagGroup>,
    nd: Vec<Node>,
}
impl Buffers {
    fn new() -> Self {
        Self {
            vs: vec![0; 4 << 20],
            vp: vec![[0.; 3]; 4 << 20],
            ix: vec![0; 16 << 20],
            cl: vec![FlatDagCluster::default(); 1 << 20],
            gr: vec![FlatDagGroup::default(); 1 << 20],
            nd: vec![Node::default(); 1 << 20],
        }
    }
}
fn run(b: &[u8], buffers: &mut Buffers) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut at = 4;
    let nv = word(b, &mut at) as usize;
    let ni = word(b, &mut at) as usize;
    let stride_bytes = word(b, &mut at) as usize;
    let s1 = word(b, &mut at) != 0;
    let dilated = word(b, &mut at) != 0;
    let stride = stride_bytes / 4;
    assert!(nv <= 4 << 20 && ni <= 16 << 20 && (8..=64).contains(&stride));
    assert_eq!(b.len(), 24 + nv * stride_bytes + ni * 4 + nv);
    let data: Vec<f32> = b[at..at + nv * stride_bytes]
        .chunks_exact(4)
        .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    at += nv * stride_bytes;
    let indices: Vec<u32> = b[at..at + ni * 4]
        .chunks_exact(4)
        .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    at += ni * 4;
    let locks: Vec<_> = b[at..]
        .iter()
        .map(|&v| meshoptimizer_rs::VertexFlags::from_bits(v).unwrap())
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
    let mut config = default_config(128).map_err(|e| e.to_string())?;
    if s1 {
        config.max_vertices = 128;
    }
    config.simplify_dilate_borders = dilated;
    let Buffers {
        vs,
        vp,
        ix,
        cl,
        gr,
        nd,
    } = buffers;
    let counts = build_dag_into(
        config,
        Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: Some(attributes),
            vertex_lock: if s1 { None } else { Some(&locks) },
            attribute_weights: &weights,
            attribute_protect_mask: (if s1 {
                (1 << 3) | (1 << 4) | (1 << 8)
            } else {
                (1 << 9) - 1
            }) | if stride == 16 { 15 << 9 } else { 0 },
        },
        8,
        DagBuffers {
            vertex_source: vs,
            vertex_positions: vp,
            indices: ix,
            clusters: cl,
            groups: gr,
            nodes: nd,
        },
        &mut Workspace::default(),
    )
    .map_err(|e| e.to_string())?;
    assert!(counts.level_count <= 64 && counts.bvh_depth <= 32);
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
    Ok(out)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffers = Buffers::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    loop {
        let mut size = [0; 4];
        match stdin.read_exact(&mut size) {
            Ok(()) => (),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };
        let n = u32::from_le_bytes(size) as usize;
        assert!(n <= 256 << 20);
        let mut input = vec![0; n];
        stdin.read_exact(&mut input)?;
        let started = std::time::Instant::now();
        let mut out = run(&input, &mut buffers)?;
        if &input[..4] == b"R11T" {
            out.extend_from_slice(&(started.elapsed().as_nanos() as u64).to_le_bytes());
        }
        stdout.write_all(&(out.len() as u32).to_le_bytes())?;
        stdout.write_all(&out)?;
        stdout.flush()?;
    }
    Ok(())
}
