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
        20..=22 => {
            use m::clusterlod as c;
            let mut positions = p.clone();
            let mut config = if target & 1 == 0 {
                c::default_config(4).unwrap()
            } else {
                c::default_config_rt(4).unwrap()
            };
            config.partition_spatial = target & 2 == 0;
            config.partition_sort = target & 4 != 0;
            config.optimize_clusters = target & 8 == 0;
            config.optimize_bounds = target & 16 != 0;
            config.simplify_dilate_borders = target & 32 != 0;
            config.simplify_permissive = target & 64 == 0;
            config.simplify_fallback_permissive = target & 128 != 0;
            config.simplify_fallback_sloppy = target & 256 == 0;
            config.simplify_regularize = target & 512 != 0;
            config.simplify_preserve_folds = target & 1024 != 0;
            config.simplify_error_clamped = target & 2048 == 0;
            config.cluster_split_factor = if target & 4096 != 0 { 0. } else { 2. };
            let attrs = vec![0.5f32; nv];
            let locks = vec![m::VertexFlags::LOCK; nv];
            let attribute_view = (target & 8192 != 0)
                .then(|| m::Attributes::from_interleaved(&attrs, nv, 1, 1, 0).unwrap());
            let mesh = c::Mesh {
                indices: &indices,
                positions: &mut positions,
                attributes: attribute_view,
                vertex_lock: (target & 16384 != 0).then_some(locks.as_slice()),
                attribute_weights: if attribute_view.is_some() { &[1.] } else { &[] },
                attribute_protect_mask: u32::from(attribute_view.is_some()),
            };
            let result = match op {
                20 => format!("{:?}", c::build(config, mesh, &mut w)),
                21 => {
                    let mut callbacks = Vec::new();
                    let result = c::build_with_output(config, mesh, |group, clusters| {
                        callbacks.push(format!("{group:?} {clusters:?}"));
                        if target & 32768 != 0 {
                            Err(m::Error::InvalidParameter)
                        } else {
                            Ok(callbacks.len() as i32 - 1)
                        }
                    }, &mut w);
                    format!("{result:?} {callbacks:?}")
                }
                _ => {
                    let mut source = vec![0xfedcba98; 32];
                    let mut vertices = vec![[17., 19., 23.]; 32];
                    let mut indices_out = vec![0xfedcba98; 96];
                    let mut clusters = vec![c::FlatDagCluster::default(); 32];
                    let mut groups = vec![c::FlatDagGroup::default(); 32];
                    let mut nodes = vec![c::Node::default(); 32];
                    let buffers = c::DagBuffers { vertex_source: &mut source,
                        vertex_positions: &mut vertices, indices: &mut indices_out,
                        clusters: &mut clusters, groups: &mut groups, nodes: &mut nodes };
                    let result = c::build_dag_into(config, mesh, 8, buffers, &mut w);
                    format!("{result:?} {source:?} {vertices:?} {indices_out:?} {clusters:?} {groups:?} {nodes:?}")
                }
            };
            format!("{result}\npositions={positions:?}")
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
