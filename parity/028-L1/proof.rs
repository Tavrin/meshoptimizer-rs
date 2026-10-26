//! Exact output, Result, work and caller-buffer side-effect witness.
use meshoptimizer_rs as m;
use std::io::{self, Read, Write};
fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn execute(b: &[u8]) -> Vec<u8> {
    assert_eq!(&b[..4], b"MO03");
    let op = word(b, 4) & 0xffff;
    let into = word(b, 4) & 0x10000 != 0;
    let nv = word(b, 8) as usize;
    let ni = word(b, 12) as usize;
    let s = m::MeshletSettings {
        max_vertices: word(b, 16) as usize,
        max_triangles: word(b, 24) as usize,
    };
    let min = word(b, 20) as usize;
    let weight = f32::from_bits(word(b, 28));
    let split = f32::from_bits(word(b, 32));
    let target = word(b, 36);
    let nc = word(b, 40) as usize;
    let p: Vec<[f32; 3]> = (0..nv)
        .map(|i| std::array::from_fn(|j| f32::from_bits(word(b, 48 + i * 12 + j * 4))))
        .collect();
    let indices: Vec<_> = (0..ni).map(|i| word(b, 48 + nv * 12 + i * 4)).collect();
    let counts: Vec<_> = (0..nc)
        .map(|i| word(b, 48 + nv * 12 + ni * 4 + i * 4))
        .collect();
    let end = 48 + nv * 16 + ni * 4 + nc * 4;
    let limits = if b.len() == end {
        m::Limits::default()
    } else {
        m::Limits {
            max_work: u64::from_le_bytes(b[end..end + 8].try_into().unwrap()),
            max_bytes: u64::from_le_bytes(b[end + 8..end + 16].try_into().unwrap()) as usize,
        }
    };
    let short = b.get(end + 16).copied().unwrap_or(0) as usize;
    let mut w = m::Workspace::new(limits);
    // Widen the existing p03 seed generator to every storage-dispatch arm.
    // Field 44 is a layout selector here; this witness never requests timing.
    let layout = word(b, 44) % 4;
    let mut floats = Vec::new();
    let mut bytes = Vec::new();
    if layout == 1 {
        for xyz in &p {
            floats.extend([17., xyz[0], xyz[1], xyz[2], 19.]);
        }
    } else if layout >= 2 {
        for xyz in &p {
            bytes.extend(17f32.to_le_bytes());
            for v in xyz {
                bytes.extend(if layout == 2 {
                    v.to_le_bytes()
                } else {
                    v.to_be_bytes()
                });
            }
            bytes.extend(19f32.to_le_bytes());
        }
    }
    if nv == 0 {
        floats.resize(1, 0.);
        bytes.resize(4, 0);
    }
    let positions = match layout {
        0 => m::Positions::from_packed(&p),
        1 => m::Positions::from_interleaved(&floats, nv, 5, 1).unwrap(),
        _ => m::Positions::from_bytes(
            &bytes,
            nv,
            20,
            4,
            if layout == 2 {
                m::ByteOrder::LittleEndian
            } else {
                m::ByteOrder::BigEndian
            },
        )
        .unwrap(),
    };
    let text = match op {
        1..=4 => {
            if into {
                let bound = m::build_meshlets_bound(
                    ni,
                    s.max_vertices,
                    if op == 1 || op == 2 {
                        s.max_triangles
                    } else {
                        min
                    },
                )
                .unwrap_or(0);
                let mut ms = vec![
                    m::Meshlet {
                        vertex_offset: 17,
                        triangle_offset: 19,
                        vertex_count: 23,
                        triangle_count: 29
                    };
                    bound.saturating_sub(if short == 1 { 1 } else { 0 })
                        + if short == 0 { 2 } else { 0 }
                ];
                let mut v = vec![
                    0xfedcba98;
                    ni.saturating_sub(if short == 2 { 1 } else { 0 })
                        + if short == 0 { 3 } else { 0 }
                ];
                let mut t = vec![
                    0xa5;
                    ni.saturating_sub(if short == 3 { 1 } else { 0 })
                        + if short == 0 { 3 } else { 0 }
                ];
                let r = match op {
                    1 => m::build_meshlets_into(
                        &mut ms, &mut v, &mut t, &indices, positions, s, weight, &mut w,
                    ),
                    2 => m::build_meshlets_scan_into(
                        &mut ms, &mut v, &mut t, &indices, nv, s, &mut w,
                    ),
                    3 => m::build_meshlets_flex_into(
                        &mut ms, &mut v, &mut t, &indices, positions, s, min, weight, split, &mut w,
                    ),
                    _ => m::build_meshlets_spatial_into(
                        &mut ms, &mut v, &mut t, &indices, positions, s, min, split, &mut w,
                    ),
                };
                format!("{r:?}\n{ms:?}\n{v:?}\n{t:?}")
            } else {
                let r = match op {
                    1 => m::build_meshlets(&indices, positions, s, weight, &mut w),
                    2 => m::build_meshlets_scan(&indices, nv, s, &mut w),
                    3 => m::build_meshlets_flex(&indices, positions, s, min, weight, split, &mut w),
                    _ => m::build_meshlets_spatial(&indices, positions, s, min, split, &mut w),
                };
                format!("{r:?}")
            }
        }
        12 => {
            let p = if target & 0x80000000 == 0 {
                Some(positions)
            } else {
                None
            };
            let target = (target & 0x7fffffff) as usize;
            if into {
                let mut out = vec![
                    0xfedcba98;
                    nc.saturating_sub(short.min(1)) + if short == 0 { 3 } else { 0 }
                ];
                let r =
                    m::partition_clusters_into(&mut out, &indices, &counts, nv, p, target, &mut w);
                format!("{r:?}\n{out:?}")
            } else {
                format!(
                    "{:?}",
                    m::partition_clusters(&indices, &counts, nv, p, target, &mut w)
                )
            }
        }
        14 => {
            if into {
                let mut out = vec![
                    0xfedcba98;
                    ni.saturating_sub(short.min(1)) + if short == 0 { 3 } else { 0 }
                ];
                let r = m::spatial_sort_triangles_into(&mut out, &indices, positions, &mut w);
                format!("{r:?}\n{out:?}")
            } else {
                format!(
                    "{:?}",
                    m::spatial_sort_triangles(&indices, positions, &mut w)
                )
            }
        }
        _ => panic!("out of L1 scope"),
    };
    format!("{text}\nusage={:?}", w.usage()).into_bytes()
}
fn main() {
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    loop {
        let mut n = [0; 4];
        if input.read_exact(&mut n).is_err() {
            break;
        }
        let mut b = vec![0; u32::from_le_bytes(n) as usize];
        input.read_exact(&mut b).unwrap();
        let out = execute(&b);
        output.write_all(&(out.len() as u32).to_le_bytes()).unwrap();
        output.write_all(&out).unwrap();
        output.flush().unwrap();
    }
}
