//! Measures the page triangle codec choice (meshlet codec against the
//! index-sequence codec) and page sizes per DAG triangle, on meshes in the parity
//! corpus's packed XYZ/u32 input format (two u32 counts, XYZ floats, u32 indices).
use meshoptimizer_rs::{clusterlod::page::*, clusterlod::*, codec, Workspace};
use std::fs;

const CLUSTERS_PER_PAGE: usize = 64;

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn main() {
    println!("mesh,grid_exponent,dag_triangles,clusters,pages,meshlet_bytes,index_sequence_bytes,memory_bytes,disk_bytes,memory_b_per_triangle,disk_b_per_triangle");
    for path in std::env::args().skip(1) {
        let b = fs::read(&path).unwrap();
        let nv = u32_at(&b, 0) as usize;
        let ni = u32_at(&b, 4) as usize;
        let mut positions: Vec<[f32; 3]> = (0..nv)
            .map(|i| core::array::from_fn(|k| f32::from_bits(u32_at(&b, 8 + i * 12 + k * 4))))
            .collect();
        let indices: Vec<_> = (0..ni).map(|i| u32_at(&b, 8 + nv * 12 + i * 4)).collect();
        drop(b);
        let (mut min, mut max) = ([f32::MAX; 3], [f32::MIN; 3]);
        for p in &positions {
            for ((lo, hi), &x) in min.iter_mut().zip(&mut max).zip(p) {
                *lo = lo.min(x);
                *hi = hi.max(x);
            }
        }
        let grid = QuantGrid::from_bounds(min, max, 16).unwrap();
        grid.snap_positions(&mut positions).unwrap();
        let mut source = vec![0; ni * 8];
        let mut vp = vec![[0.; 3]; ni * 8];
        let mut ix = vec![0; ni * 8];
        let mut cl = vec![FlatDagCluster::default(); ni / 3];
        let mut gr = vec![FlatDagGroup::default(); ni / 3];
        let mut nodes = vec![Node::default(); ni / 3 + 128];
        let mut ws = Workspace::default();
        let counts = build_dag_into(
            default_config(128).unwrap(),
            Mesh {
                indices: &indices,
                positions: &mut positions,
                attributes: None,
                vertex_lock: None,
                attribute_weights: &[],
                attribute_protect_mask: 0,
            },
            8,
            DagBuffers {
                vertex_source: &mut source,
                vertex_positions: &mut vp,
                indices: &mut ix,
                clusters: &mut cl,
                groups: &mut gr,
                nodes: &mut nodes,
            },
            &mut ws,
        )
        .unwrap();
        drop(source);
        drop(nodes);
        let (mut meshlet, mut sequence, mut memory, mut disk, mut pages) = (0, 0, 0, 0, 0);
        let mut locals = Vec::new();
        for c in &cl[..counts.clusters] {
            let tri = &ix[c.first_index as usize..][..c.index_count as usize];
            let (mut local, mut ids, mut pos) = (Vec::new(), Vec::new(), Vec::new());
            for &v in tri {
                let k = ids.iter().position(|&x| x == v).unwrap_or_else(|| {
                    ids.push(v);
                    pos.push(vp[v as usize]);
                    ids.len() - 1
                });
                local.push(k as u32);
            }
            locals.push((c.group, c.refined, pos, local));
        }
        for chunk in locals.chunks(CLUSTERS_PER_PAGE) {
            let clusters: Vec<_> = chunk
                .iter()
                .map(|(g, r, p, i)| PageCluster::new(*g, *r, p, i))
                .collect();
            let page = encode_page(grid, &clusters, 0, &mut ws).unwrap();
            let view = PageView::new(&page, grid).unwrap();
            for c in 0..view.cluster_count() {
                let mut t = [[0u8; 3]; 256];
                let n = view.decode_triangles(c, &mut t).unwrap();
                let nv = view.cluster(c).unwrap().vertex_count;
                let refs: Vec<u32> = (0..nv).collect();
                let flat: Vec<u8> = t[..n].iter().flatten().copied().collect();
                meshlet += codec::encode_meshlet(&refs, &flat, &mut ws).unwrap().len();
                let wide: Vec<u32> = flat.iter().map(|&i| u32::from(i)).collect();
                sequence += codec::encode_index_sequence(
                    &wide,
                    codec::IndexEncoding::new(1).unwrap(),
                    &mut ws,
                )
                .unwrap()
                .len();
            }
            memory += page.len() * 4;
            disk += encode_disk_page(&page, grid, DiskCodec::Compressed, &mut ws)
                .unwrap()
                .len();
            pages += 1;
        }
        let triangles = counts.indices / 3;
        println!(
            "{path},{},{triangles},{},{pages},{meshlet},{sequence},{memory},{disk},{:.3},{:.3}",
            grid.exponent(),
            counts.clusters,
            memory as f64 / triangles as f64,
            disk as f64 / triangles as f64,
        );
    }
}
