//! Phase 0.3 binary protocol and safe WASM adapter.
use meshoptimizer_rs::*;
use std::sync::Mutex;
use std::time::Instant;
fn word(b: &[u8], i: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        b.get(i..i + 4)
            .ok_or("short input")?
            .try_into()
            .map_err(|_| "word")?,
    ))
}
fn push(o: &mut Vec<u8>, v: u32) {
    o.extend_from_slice(&v.to_le_bytes());
}
fn lod_bounds(o: &mut Vec<u8>, b: clusterlod::LodBounds) {
    for v in b.center.into_iter().chain([b.radius, b.error]) {
        push(o, v.to_bits());
    }
}
fn bounds(o: &mut Vec<u8>, b: Bounds) {
    for v in b
        .center
        .into_iter()
        .chain([b.radius])
        .chain(b.cone_apex)
        .chain(b.cone_axis)
        .chain([b.cone_cutoff])
    {
        push(o, v.to_bits());
    }
    for v in b.cone_axis_s8.into_iter().chain([b.cone_cutoff_s8]) {
        o.push(v as u8);
    }
}
/// Execute a checked phase 0.3 request. Timing excludes protocol serialization.
pub fn execute(b: &[u8]) -> Result<Vec<u8>, String> {
    if b.get(..4) != Some(b"MO03") {
        return Err("version".into());
    }
    let timing_only = word(b, 4)? & 0x20000 != 0;
    let into = word(b, 4)? & 0x10000 != 0;
    let op = word(b, 4)? & 0xffff;
    let nv = word(b, 8)? as usize;
    let ni = word(b, 12)? as usize;
    let mv = word(b, 16)? as usize;
    let min = word(b, 20)? as usize;
    let mt = word(b, 24)? as usize;
    let weight = f32::from_bits(word(b, 28)?);
    let split = f32::from_bits(word(b, 32)?);
    let target = word(b, 36)? as usize;
    let nc = word(b, 40)? as usize;
    let repeats = word(b, 44)? as usize;
    let expected = 48usize
        .checked_add(nv.checked_mul(16).ok_or("overflow")?)
        .and_then(|n| n.checked_add(ni.checked_mul(4)?))
        .and_then(|n| n.checked_add(nc.checked_mul(4)?))
        .ok_or("overflow")?;
    if b.len() != expected || expected > 128 * 1024 * 1024 || repeats > 2000000 {
        return Err("length".into());
    }
    let mut p = Vec::with_capacity(nv);
    for i in 0..nv {
        p.push([
            f32::from_bits(word(b, 48 + i * 12)?),
            f32::from_bits(word(b, 52 + i * 12)?),
            f32::from_bits(word(b, 56 + i * 12)?),
        ]);
    }
    let at = 48 + nv * 12;
    let mut idx = Vec::with_capacity(ni);
    for i in 0..ni {
        idx.push(word(b, at + i * 4)?);
    }
    let at = at + ni * 4;
    let mut counts = Vec::with_capacity(nc);
    for i in 0..nc {
        counts.push(word(b, at + i * 4)?);
    }
    let at = at + nc * 4;
    let mut radii = Vec::with_capacity(nv);
    for i in 0..nv {
        radii.push(f32::from_bits(word(b, at + i * 4)?));
    }
    if op == 16 {
        let mut ws = Workspace::default();
        let mut c = if target & 1 != 0 {
            clusterlod::default_config_rt(mt)
        } else {
            clusterlod::default_config(mt)
        }
        .map_err(|e| e.to_string())?;
        c.partition_size = mv;
        c.partition_sort = target & 2 != 0;
        c.simplify_regularize = target & 4 != 0;
        c.simplify_preserve_folds = target & 8 != 0;
        c.simplify_dilate_borders = target & 16 != 0;
        c.simplify_error_edge_limit = if target & 32 != 0 { 1.5 } else { 0. };
        c.optimize_bounds = target & 64 != 0;
        c.simplify_permissive = target & 128 == 0;
        c.simplify_fallback_permissive = target & 128 != 0;
        c.simplify_fallback_sloppy = target & 256 == 0;
        let width = if nc == 0 {
            0
        } else {
            ((target >> 20) & 31) + 1
        };
        let mut attributes = Vec::new();
        if width > 0 {
            if width == 1 {
                attributes.extend_from_slice(&radii);
            } else {
                if counts.len() != nv * width {
                    return Err("attribute length".into());
                }
                attributes.extend(counts.iter().map(|&v| f32::from_bits(v)));
            }
        }
        let a = if width > 0 {
            Some(
                Attributes::from_interleaved(&attributes, nv, width, width, 0)
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };
        let weights = (0..width)
            .map(|k| split * (k + 1) as f32)
            .collect::<Vec<_>>();
        let flags = (0..nv)
            .map(|i| {
                VertexFlags::from_bits(if target & 1024 != 0 { (i % 8) as u8 } else { 0 })
                    .expect("flag")
            })
            .collect::<Vec<_>>();
        if target & 4096 != 0 {
            c.optimize_clusters_level = ((target >> 13) % 10) as u8;
            c.simplify_ratio = 0.25;
        }
        let mesh = clusterlod::Mesh {
            indices: &idx,
            positions: &mut p,
            attributes: a,
            vertex_lock: if target & 1024 != 0 {
                Some(&flags)
            } else {
                None
            },
            attribute_weights: &weights,
            attribute_protect_mask: if target & 512 != 0 { 1 } else { 0 },
        };
        let groups = if target & 2048 != 0 {
            let mut groups = Vec::new();
            clusterlod::build_with_output(
                c,
                mesh,
                |group, clusters| {
                    let id = groups.len() as i32 * 7 + 2;
                    let clusters = clusters
                        .iter()
                        .map(|cl| clusterlod::Cluster {
                            refined: cl.refined,
                            bounds: cl.bounds,
                            indices: cl.indices.to_vec(),
                            vertex_count: cl.vertex_count,
                        })
                        .collect();
                    groups.push(clusterlod::GroupOutput { group, clusters });
                    Ok(id)
                },
                &mut ws,
            )
            .map_err(|e| e.to_string())?;
            groups
        } else {
            clusterlod::build(c, mesh, &mut ws).map_err(|e| e.to_string())?
        };
        let mut o = Vec::new();
        push(&mut o, groups.len() as u32);
        for g in &groups {
            push(&mut o, g.group.depth as u32);
            lod_bounds(&mut o, g.group.simplified);
            push(&mut o, g.clusters.len() as u32);
            for cl in &g.clusters {
                push(&mut o, cl.refined as u32);
                lod_bounds(&mut o, cl.bounds);
                push(&mut o, cl.vertex_count as u32);
                push(&mut o, cl.indices.len() as u32);
                for &v in &cl.indices {
                    push(&mut o, v);
                }
            }
        }
        for xyz in &p {
            for &v in xyz {
                push(&mut o, v.to_bits());
            }
        }
        let meta = groups.iter().map(|g| g.group).collect::<Vec<_>>();
        let levels = meta.iter().map(|g| g.depth as usize + 1).max().unwrap_or(0);
        let bound = clusterlod::build_hierarchy_bound(meta.len(), min, levels)
            .map_err(|e| e.to_string())?;
        let nodes =
            clusterlod::build_hierarchy(&meta, min, levels, &mut ws).map_err(|e| e.to_string())?;
        push(&mut o, bound as u32);
        push(&mut o, nodes.len() as u32);
        for n in nodes {
            lod_bounds(&mut o, n.bounds);
            push(&mut o, n.group as u32);
            push(&mut o, n.child_offset);
            push(&mut o, n.child_count);
        }
        let mut out = b"MR03".to_vec();
        push(&mut out, o.len() as u32);
        out.extend(o);
        out.extend([0; 16]);
        return Ok(out);
    }
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&p);
    let s = MeshletSettings {
        max_vertices: mv,
        max_triangles: mt,
    };
    let local = if matches!(op, 7 | 9 | 10) {
        Some(extract_meshlet_indices(&idx, &mut ws).map_err(|e| e.to_string())?)
    } else {
        None
    };
    let mut out_meshlets = if into && op <= 4 {
        vec![
            Meshlet::default();
            build_meshlets_bound(ni, mv, if op <= 2 { mt } else { min })
                .map_err(|e| e.to_string())?
        ]
    } else {
        Vec::new()
    };
    let mut out_vertices = if into {
        vec![
            0;
            if op <= 4 || op == 14 {
                ni
            } else if matches!(op, 9..=11) {
                256
            } else if op == 12 {
                nc
            } else {
                nv
            }
        ]
    } else {
        Vec::new()
    };
    let mut out_triangles = if into && (op <= 4 || matches!(op, 9..=11)) {
        vec![0; ni]
    } else {
        Vec::new()
    };
    // D78: the closure fills a caller-owned transport buffer. Returning the
    // Vec by value made every timed repeat reload it through a partially
    // forwarded store, a driver cost that the C++ lambda (NRVO) does not pay.
    let mut run = |collect: bool, o: &mut Vec<u8>| -> Result<(), Error> {
        if into && !matches!(op, 5..=8) {
            match op {
                1..=4 => {
                    let used = match op {
                        1 => build_meshlets_into(
                            &mut out_meshlets,
                            &mut out_vertices,
                            &mut out_triangles,
                            &idx,
                            p,
                            s,
                            weight,
                            &mut ws,
                        )?,
                        2 => build_meshlets_scan_into(
                            &mut out_meshlets,
                            &mut out_vertices,
                            &mut out_triangles,
                            &idx,
                            nv,
                            s,
                            &mut ws,
                        )?,
                        3 => build_meshlets_flex_into(
                            &mut out_meshlets,
                            &mut out_vertices,
                            &mut out_triangles,
                            &idx,
                            p,
                            s,
                            min,
                            weight,
                            split,
                            &mut ws,
                        )?,
                        _ => build_meshlets_spatial_into(
                            &mut out_meshlets,
                            &mut out_vertices,
                            &mut out_triangles,
                            &idx,
                            p,
                            s,
                            min,
                            weight,
                            &mut ws,
                        )?,
                    };
                    if collect {
                        push(o, used.meshlets as u32);
                        push(o, used.vertices as u32);
                        push(o, used.triangles as u32);
                        for d in &out_meshlets[..used.meshlets] {
                            for v in [
                                d.vertex_offset,
                                d.triangle_offset,
                                d.vertex_count,
                                d.triangle_count,
                            ] {
                                push(o, v);
                            }
                        }
                        for &v in &out_vertices[..used.vertices] {
                            push(o, v);
                        }
                        o.extend_from_slice(&out_triangles[..used.triangles]);
                    }
                }
                9..=11 => {
                    let n = if op == 11 {
                        extract_meshlet_indices_into(
                            &mut out_vertices,
                            &mut out_triangles,
                            &idx,
                            &mut ws,
                        )?
                    } else {
                        let m = local.as_ref().expect("local");
                        optimize_meshlet_level_into(
                            &mut out_vertices,
                            &mut out_triangles,
                            &m.vertices,
                            &m.triangles,
                            if op == 9 { 0 } else { target as u8 },
                            &mut ws,
                        )?;
                        m.vertices.len()
                    };
                    if collect {
                        push(o, n as u32);
                        for &v in &out_vertices[..n] {
                            push(o, v);
                        }
                        o.extend_from_slice(&out_triangles);
                    }
                }
                12 => {
                    let n = partition_clusters_into(
                        &mut out_vertices,
                        &idx,
                        &counts,
                        nv,
                        if target & 0x80000000 == 0 {
                            Some(p)
                        } else {
                            None
                        },
                        target & 0x7fffffff,
                        &mut ws,
                    )?;
                    if collect {
                        push(o, n as u32);
                        for &v in &out_vertices {
                            push(o, v);
                        }
                    }
                }
                13..=15 => {
                    match op {
                        13 => spatial_sort_remap_into(&mut out_vertices, p, &mut ws)?,
                        14 => spatial_sort_triangles_into(&mut out_vertices, &idx, p, &mut ws)?,
                        _ => spatial_cluster_points_into(&mut out_vertices, p, target, &mut ws)?,
                    };
                    if collect {
                        for &v in &out_vertices {
                            push(o, v);
                        }
                    }
                }
                _ => return Err(Error::InvalidParameter),
            }
            std::hint::black_box((&out_meshlets, &out_vertices, &out_triangles));
            return Ok(());
        }
        match op {
            1..=4 => {
                let m = match op {
                    1 => build_meshlets(&idx, p, s, weight, &mut ws)?,
                    2 => build_meshlets_scan(&idx, nv, s, &mut ws)?,
                    3 => build_meshlets_flex(&idx, p, s, min, weight, split, &mut ws)?,
                    _ => build_meshlets_spatial(&idx, p, s, min, weight, &mut ws)?,
                };
                if !collect {
                    std::hint::black_box(m);
                    return Ok(());
                }
                push(o, m.meshlets.len() as u32);
                push(o, m.vertices.len() as u32);
                push(o, m.triangles.len() as u32);
                for d in m.meshlets {
                    for v in [
                        d.vertex_offset,
                        d.triangle_offset,
                        d.vertex_count,
                        d.triangle_count,
                    ] {
                        push(o, v);
                    }
                }
                for v in m.vertices {
                    push(o, v);
                }
                o.extend(m.triangles);
            }
            5 => {
                let n = std::hint::black_box(build_meshlets_bound(ni, mv, mt)?);
                if collect {
                    push(o, n as u32);
                }
            }
            6 => {
                let b = std::hint::black_box(compute_cluster_bounds(&idx, p, &mut ws)?);
                if collect {
                    bounds(o, b);
                }
            }
            7 => {
                let m = local.as_ref().expect("local");
                let b = std::hint::black_box(compute_meshlet_bounds(
                    &m.vertices,
                    &m.triangles,
                    p,
                    &mut ws,
                )?);
                if collect {
                    bounds(o, b);
                }
            }
            8 => {
                let r = Attributes::from_interleaved(&radii, nv, 1, 1, 0)?;
                let b = std::hint::black_box(compute_sphere_bounds(
                    p,
                    if target == 0 { None } else { Some(r) },
                    &mut ws,
                )?);
                if collect {
                    bounds(o, b);
                }
            }
            9..=11 => {
                let m = if op == 11 {
                    extract_meshlet_indices(&idx, &mut ws)?
                } else {
                    let m = local.as_ref().expect("local");
                    optimize_meshlet_level(
                        &m.vertices,
                        &m.triangles,
                        if op == 9 { 0 } else { target as u8 },
                        &mut ws,
                    )?
                };
                if !collect {
                    std::hint::black_box(m);
                    return Ok(());
                }
                push(o, m.vertices.len() as u32);
                for v in m.vertices {
                    push(o, v);
                }
                o.extend(m.triangles);
            }
            12 => {
                let a = partition_clusters(
                    &idx,
                    &counts,
                    nv,
                    if target & 0x80000000 == 0 {
                        Some(p)
                    } else {
                        None
                    },
                    target & 0x7fffffff,
                    &mut ws,
                )?;
                if !collect {
                    std::hint::black_box(a);
                    return Ok(());
                }
                push(o, a.count as u32);
                for v in a.assignments {
                    push(o, v);
                }
            }
            13..=15 => {
                let a = match op {
                    13 => spatial_sort_remap(p, &mut ws)?,
                    14 => spatial_sort_triangles(&idx, p, &mut ws)?,
                    _ => spatial_cluster_points(p, target, &mut ws)?,
                };
                if !collect {
                    std::hint::black_box(a);
                    return Ok(());
                }
                for v in a {
                    push(o, v);
                }
            }
            _ => return Err(Error::InvalidParameter),
        }
        Ok(())
    };
    let mut result = Vec::new();
    run(!timing_only, &mut result).map_err(|e| e.to_string())?;
    let start = if repeats == 0 {
        None
    } else {
        Some(Instant::now())
    };
    if op == 5 {
        // D78: the bound is ~20 ns of arithmetic, so both drivers time the
        // bare call (inputs and result opaque) instead of the dispatch closure.
        for _ in 0..repeats {
            let bound = build_meshlets_bound(
                std::hint::black_box(ni),
                std::hint::black_box(mv),
                std::hint::black_box(mt),
            );
            std::hint::black_box(bound).map_err(|e| e.to_string())?;
        }
    } else {
        for _ in 0..repeats {
            // The operation's real outputs are black-boxed inside run. Drop its
            // empty transport Vec here, just like the reference driver's loop.
            run(false, &mut Vec::new()).map_err(|e| e.to_string())?;
        }
    }
    let elapsed = if repeats == 0 {
        0.
    } else {
        start.expect("native timer").elapsed().as_secs_f64() / repeats as f64
    };
    let mut out = b"MR03".to_vec();
    push(&mut out, result.len() as u32);
    out.extend(result);
    out.extend(elapsed.to_le_bytes());
    out.extend((ws.usage().bytes as u64).to_le_bytes());
    Ok(out)
}
static INPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
/// Reserve bounded WASM transport.
#[no_mangle]
pub extern "C" fn request(n: u32) -> u32 {
    if n > 128 * 1024 * 1024 {
        return 1;
    }
    let Ok(mut b) = INPUT.lock() else { return 1 };
    b.clear();
    if b.try_reserve_exact(n as usize).is_err() {
        return 1;
    }
    b.resize(n as usize, 0);
    0
}
/// Write a transport byte.
#[no_mangle]
pub extern "C" fn put_byte(i: u32, v: u32) -> u32 {
    let Ok(mut b) = INPUT.lock() else { return 1 };
    if let Some(b) = b.get_mut(i as usize) {
        *b = v as u8;
        0
    } else {
        1
    }
}
/// Execute the current transport.
#[no_mangle]
pub extern "C" fn run() -> u32 {
    let Ok(i) = INPUT.lock() else { return 1 };
    let Ok(mut o) = OUTPUT.lock() else { return 1 };
    o.clear();
    match execute(&i) {
        Ok(b) => {
            *o = b;
            0
        }
        Err(_) => 1,
    }
}
/// Response length.
#[no_mangle]
pub extern "C" fn output_len() -> u32 {
    OUTPUT.lock().map_or(0, |o| o.len() as u32)
}
/// Response byte, or 256 for an invalid offset.
#[no_mangle]
pub extern "C" fn get_byte(i: u32) -> u32 {
    OUTPUT
        .lock()
        .map_or(256, |o| o.get(i as usize).map_or(256, |&b| b as u32))
}
