//! Parallel versus serial identity at 1/4/16 workers on the eight frozen inputs:
//! vertex encode and decode (parallel) against serial candidate and serial v0.2.16 baseline,
//! and spatial meshlets (parallel) against serial candidate and serial baseline, bytes and work.
use std::hash::{Hash, Hasher};

fn read(path: &std::path::Path) -> (Vec<[f32; 3]>, Vec<u32>) {
    let data = std::fs::read(path).unwrap();
    let word = |a: usize| u32::from_le_bytes(data[a..a + 4].try_into().unwrap());
    let (n, ni) = (word(0) as usize, word(4) as usize);
    let p = (0..n).map(|v| core::array::from_fn(|k| f32::from_bits(word(8 + (v * 3 + k) * 4)))).collect();
    let i = (0..ni).map(|k| word(8 + n * 12 + k * 4)).collect();
    (p, i)
}

fn hash(bytes: &[u8]) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}

fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().is_some_and(|e| e == "bin")).collect();
    files.sort();
    let pools: Vec<_> = [1usize, 4, 16].iter().map(|&n| (n, rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap())).collect();
    let unlimited_c = candidate::Limits { max_bytes: usize::MAX, max_work: u64::MAX };
    let unlimited_b = baseline::Limits { max_bytes: usize::MAX, max_work: u64::MAX };
    let mut checks = 0u64;
    for path in &files {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let (p, ix) = read(path);
        let positions: Vec<u8> = p.iter().flatten().flat_map(|x| x.to_le_bytes()).collect();
        // A 24-byte attribute record derived deterministically from the positions.
        let attrs: Vec<u8> = p
            .iter()
            .enumerate()
            .flat_map(|(v, q)| {
                let mut r = Vec::with_capacity(24);
                for k in 0..3 {
                    r.extend_from_slice(&q[k].to_le_bytes());
                }
                r.extend_from_slice(&((v as u32).wrapping_mul(2654435761) >> 8).to_le_bytes());
                r.extend_from_slice(&(((v as u32) * 7 + 3) as u16).to_le_bytes());
                r.extend_from_slice(&[(v % 251) as u8, (v % 13) as u8]);
                r.extend_from_slice(&(v as f32 * 0.25).to_le_bytes());
                r
            })
            .collect();
        let n = p.len();
        for (stride, data) in [(12usize, &positions), (24usize, &attrs)] {
            for (ver, mode) in [(0u32, 2u32), (1, 0)] {
                let ce = candidate::codec::VertexEncoding::new(ver as _, mode as _).unwrap();
                let be = baseline::codec::VertexEncoding::new(ver as _, mode as _).unwrap();
                let serial_b = baseline::codec::encode_vertex_buffer(data, n, stride, be, &mut baseline::Workspace::new(unlimited_b)).unwrap();
                let serial_c = candidate::codec::encode_vertex_buffer(data, n, stride, ce, &mut candidate::Workspace::new(unlimited_c)).unwrap();
                assert_eq!(serial_b, serial_c, "serial encode {name} {stride} v{ver}");
                for (w, pool) in &pools {
                    pool.install(|| {
                        let par = candidate::parallel::encode_vertex_buffer_parallel(data, n, stride, ce, &mut candidate::Workspace::new(unlimited_c)).unwrap();
                        assert_eq!(par, serial_b, "parallel encode {name} stride {stride} v{ver} workers {w}");
                        let dec = candidate::parallel::decode_vertex_buffer_parallel(n, stride, &serial_b, &mut candidate::Workspace::new(unlimited_c)).unwrap();
                        assert_eq!(&dec, data, "parallel decode {name} stride {stride} v{ver} workers {w}");
                    });
                    checks += 2;
                }
            }
        }
        for (maxv, maxt) in [(64usize, 124usize), (255, 512)] {
            let sb = baseline::MeshletSettings { max_vertices: maxv, max_triangles: maxt };
            let sc = candidate::MeshletSettings { max_vertices: maxv, max_triangles: maxt };
            let mut wb = baseline::Workspace::new(unlimited_b);
            let serial_b = baseline::build_meshlets_spatial(&ix, baseline::Positions::from_packed(&p), sb, 16, 0.5, &mut wb).unwrap();
            let mut wc = candidate::Workspace::new(unlimited_c);
            let serial_c = candidate::build_meshlets_spatial(&ix, candidate::Positions::from_packed(&p), sc, 16, 0.5, &mut wc).unwrap();
            assert_eq!(format!("{serial_b:?}"), format!("{serial_c:?}"), "serial spatial {name} {maxv}");
            let key = hash(format!("{serial_b:?}").as_bytes());
            for (w, pool) in &pools {
                pool.install(|| {
                    let mut ws = candidate::Workspace::new(unlimited_c);
                    let par = candidate::parallel::build_meshlets_spatial_parallel(&ix, candidate::Positions::from_packed(&p), sc, 16, 0.5, &mut ws).unwrap();
                    assert_eq!(hash(format!("{par:?}").as_bytes()), key, "parallel spatial {name} {maxv} workers {w}");
                    assert_eq!(ws.usage().work, wc.usage().work, "parallel spatial work {name} {maxv} workers {w}");
                });
                checks += 1;
            }
        }
        println!("PASS {name} checks so far {checks}");
    }
    println!("TOTAL parallel-vs-serial checks {checks} differences 0");
}
