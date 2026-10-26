//! Private exact-result transport, including errors and failure-side buffers.
use meshoptimizer_rs::*;
use std::io::{self, Read, Write};

fn limits() {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [-0., 0., 0.]];
    let raw = positions.iter().flatten().flat_map(|v: &f32| v.to_le_bytes()).collect::<Vec<_>>();
    for max_work in 0..1500 {
        for max_bytes in [0, 16, 32, 64, 128, 256, 512, 1 << 20] {
            for indices in [&[][..], &[0, 1, 2][..], &[0, 1, 2, 3, 2, 1][..], &[0, 0, 0][..], &[0, 4, 2][..], &[0, 1][..]] {
                let make = || Workspace::new(Limits { max_bytes, max_work });
                let mut w = make();
                let result = generate_adjacency_index_buffer(indices, Positions::from_packed(&positions), &mut w);
                println!("adj {result:?} {:?}", w.usage());
                let mut w = make();
                let mut out = [0xdeadbeef; 16];
                let result = generate_adjacency_index_buffer_into(&mut out, indices, Positions::from_packed(&positions), &mut w);
                println!("adj-into {result:?} {out:?} {:?}", w.usage());
                let vertices = VertexStream::packed(&raw, 12).unwrap();
                let mut w = make();
                let result = optimize_vertex_fetch(indices, vertices, &mut w);
                println!("fetch {result:?} {:?}", w.usage());
                let mut w = make();
                let mut out = [0xcd; 64];
                let mut ix = indices.to_vec();
                let result = optimize_vertex_fetch_into(&mut out, &mut ix, vertices, &mut w);
                println!("fetch-into {result:?} {out:?} {ix:?} {:?}", w.usage());
                for remap in [[2, 1, 0, 3], [u32::MAX, 1, 0, 3]] {
                    let mut w = make();
                    let result = remap_index_buffer(Some(indices), &remap, &mut w);
                    println!("remap {result:?} {:?}", w.usage());
                    let mut w = make();
                    let mut out = [0xdeadbeef; 16];
                    let result = remap_index_buffer_into(&mut out, Some(indices), &remap, &mut w);
                    println!("remap-into {result:?} {out:?} {:?}", w.usage());
                }
            }
            for triangles in [&[][..], &[0, 1, 2][..], &[2, 0, 2][..], &[3, 1, 0][..], &[4, 1, 2][..], &[0][..]] {
                for level in [0, 1, 9, 10] {
                    let mut w = Workspace::new(Limits { max_bytes, max_work });
                    let result = optimize_meshlet_level(&[42, 19, 8, 23], triangles, level, &mut w);
                    println!("meshlet {result:?} {:?}", w.usage());
                    let mut w = Workspace::new(Limits { max_bytes, max_work });
                    let mut v = [42, 19, 8, 23];
                    let mut t = triangles.to_vec();
                    let result = optimize_meshlet_level_in_place(&mut v, &mut t, level, &mut w);
                    println!("meshlet-in-place {result:?} {v:?} {t:?} {:?}", w.usage());
                }
            }
        }
    }
}

fn typed_limits() {
    for size in [4, 8, 12, 16] {
        for stride in [size, size + 3] {
            let raw = (0..1024 * stride).map(|i| (i % 251) as u8).collect::<Vec<_>>();
            let vertices = VertexStream::new(&raw, 1024, size, stride, 0).unwrap();
            let indices = (0..1024).flat_map(|i| [i, i, i]).collect::<Vec<u32>>();
            for max_work in [0, 3071, 3072, 6143, 6144, 6145, 9215, 9216, 100000] {
                for max_bytes in [0, 1024, 16384, 32768, 65536, 100000] {
                    let mut w = Workspace::new(Limits { max_bytes, max_work });
                    let result = optimize_vertex_fetch(&indices, vertices, &mut w);
                    println!("typed {size} {stride} {max_work} {max_bytes} {result:?} {:?}", w.usage());
                }
            }
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("limits") {
        limits();
        typed_limits();
        return;
    }
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    loop {
        let mut size = [0; 4];
        if input.read_exact(&mut size).is_err() { break; }
        let mut data = vec![0; u32::from_le_bytes(size) as usize];
        input.read_exact(&mut data).unwrap();
        let result = if data.starts_with(b"MFUZ") {
            let data = &data[4..];
            let byte = |i| data.get(i).copied().unwrap_or(0);
            let n = byte(0) as usize;
            let nt = byte(1) as usize * 2;
            let vertices = (0..n as u32).collect::<Vec<_>>();
            let mut triangles = (0..nt * 3).map(|i| {
                if n > 0 && byte(2) & 8 == 0 { byte(i + 20) % n as u8 } else { byte(i + 20) }
            }).collect::<Vec<_>>();
            if byte(2) & 64 != 0 { triangles.pop(); }
            let limits = Limits {
                max_bytes: if byte(3) & 1 != 0 { byte(4) as usize * 64 } else { 1 << 24 },
                max_work: if byte(3) & 2 != 0 { byte(4) as u64 } else { 1 << 24 },
            };
            let mut output = String::new();
            for level in [0, byte(7) % 16] {
                let mut w = Workspace::new(limits);
                let result = optimize_meshlet_level(&vertices, &triangles, level, &mut w);
                output.push_str(&format!("{result:?} {:?}\n", w.usage()));
                let mut w = Workspace::new(limits);
                let mut v = vertices.clone();
                let mut t = triangles.clone();
                let result = optimize_meshlet_level_in_place(&mut v, &mut t, level, &mut w);
                output.push_str(&format!("{result:?} {v:?} {t:?} {:?}\n", w.usage()));
            }
            Ok(output.into_bytes())
        } else if data.starts_with(b"MO03") {
            meshopt_p03_parity::execute(&data).map(|mut bytes| { bytes.truncate(bytes.len() - 16); bytes })
        } else {
            meshoptimizer_parity::execute(&data)
        };
        let response = match result {
            Ok(mut bytes) => { bytes.insert(0, 0); bytes },
            Err(error) => { let mut bytes = vec![1]; bytes.extend(error.as_bytes()); bytes }
        };
        output.write_all(&(response.len() as u32).to_le_bytes()).unwrap();
        output.write_all(&response).unwrap();
        output.flush().unwrap();
    }
}
