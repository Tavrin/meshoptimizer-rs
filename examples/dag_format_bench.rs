//! Measures full real meshes in the parity corpus's packed XYZ/u32 input format.
use meshoptimizer_rs::{clusterlod::dag_format::*, clusterlod::*, Workspace};
use std::{fs, hint::black_box, time::Instant};
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn main() {
    println!("mesh,vertices,indices,clusters,pages,raw_geometry,stored_geometry,file_bytes,write_ms,read_ms,page_decode_us");
    for path in std::env::args().skip(1) {
        let b = fs::read(&path).unwrap();
        let nv = u32_at(&b, 0) as usize;
        let ni = u32_at(&b, 4) as usize;
        let mut positions: Vec<[f32; 3]> = (0..nv)
            .map(|i| core::array::from_fn(|k| f32::from_bits(u32_at(&b, 8 + i * 12 + k * 4))))
            .collect();
        let indices: Vec<_> = (0..ni).map(|i| u32_at(&b, 8 + nv * 12 + i * 4)).collect();
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
        drop(positions);
        drop(indices);
        drop(nodes);
        drop(b);
        cl.truncate(counts.clusters);
        cl.shrink_to_fit();
        gr.truncate(counts.groups);
        gr.shrink_to_fit();
        vp.truncate(counts.vertices);
        vp.shrink_to_fit();
        ix.truncate(counts.indices);
        ix.shrink_to_fit();
        let (cl, gr, vp, ix) = (
            &cl[..counts.clusters],
            &gr[..counts.groups],
            &vp[..counts.vertices],
            &ix[..counts.indices],
        );
        let mut writes = Vec::new();
        let mut reads = Vec::new();
        let mut decodes = Vec::new();
        let mut bytes = Vec::new();
        for _ in 0..5 {
            let t = Instant::now();
            let b = write(gr, cl, vp, ix, DEFAULT_PAGE_SIZE, &mut ws).unwrap();
            writes.push(t.elapsed().as_secs_f64() * 1000.);
            if !bytes.is_empty() {
                assert_eq!(bytes, b);
            }
            bytes = b;
            let t = Instant::now();
            let r = Reader::read(&bytes, &mut ws).unwrap();
            reads.push(t.elapsed().as_secs_f64() * 1000.);
            for page in 0..r.page_count() {
                let t = Instant::now();
                black_box(
                    r.decode_page(page, &bytes[r.page_range(page).unwrap()], &mut ws)
                        .unwrap(),
                );
                decodes.push(t.elapsed().as_secs_f64() * 1e6);
            }
        }
        writes.sort_by(f64::total_cmp);
        reads.sort_by(f64::total_cmp);
        decodes.sort_by(f64::total_cmp);
        let mut raw = 0;
        let mut stored = 0;
        for c in 0..counts.clusters {
            let at = 32 + counts.groups * 32 + c * 64;
            raw += u32_at(&bytes, at + 32) as usize;
            stored += (u32_at(&bytes, at + 16) + u32_at(&bytes, at + 20)) as usize;
        }
        println!(
            "{},{nv},{ni},{},{},{raw},{stored},{},{:.3},{:.3},{:.3}",
            std::path::Path::new(&path)
                .file_stem()
                .unwrap()
                .to_string_lossy(),
            counts.clusters,
            Reader::read(&bytes, &mut ws).unwrap().page_count(),
            bytes.len(),
            writes[2],
            reads[2],
            decodes[decodes.len() / 2]
        );
    }
}
