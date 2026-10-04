use meshoptimizer_rs::*;
fn canonical(indices: &[u32]) -> Vec<[u32; 3]> {
    let mut result = indices
        .as_chunks::<3>()
        .0
        .iter()
        .map(|&t| t.min([t[1], t[2], t[0]]).min([t[2], t[0], t[1]]))
        .collect::<Vec<_>>();
    result.sort_unstable();
    result
}
fn permutation(order: &[u32], n: usize) {
    assert_eq!(order.len(), n);
    let mut v = order.to_vec();
    v.sort_unstable();
    assert!(v.iter().copied().eq(0..n as u32));
}
fn check_meshlets(m: Meshlets, idx: &[u32], s: MeshletSettings) {
    let (mut vo, mut to) = (0, 0);
    let mut decoded = Vec::new();
    for d in m.meshlets {
        assert_eq!(d.vertex_offset as usize, vo);
        assert_eq!(d.triangle_offset as usize, to);
        assert!((1..=s.max_vertices).contains(&(d.vertex_count as usize)));
        assert!((1..=s.max_triangles).contains(&(d.triangle_count as usize)));
        let v = &m.vertices[vo..vo + d.vertex_count as usize];
        let t = &m.triangles[to..to + d.triangle_count as usize * 3];
        for &i in t {
            assert!((i as usize) < v.len());
            decoded.push(v[i as usize]);
        }
        vo += v.len();
        to += t.len();
    }
    assert_eq!(vo, m.vertices.len());
    assert_eq!(to, m.triangles.len());
    assert_eq!(canonical(&decoded), canonical(idx));
}
fn check_bounds(b: Bounds) {
    assert!(b
        .center
        .into_iter()
        .chain([b.radius])
        .chain(b.cone_apex)
        .chain(b.cone_axis)
        .chain([b.cone_cutoff])
        .all(f32::is_finite));
    assert!(b.radius >= 0.);
}
/// Exercise one checked boundary with valid and malformed, finite and nonfinite data.
pub fn exercise(op: u8, data: &[u8]) {
    let byte = |i: usize| data.get(i).copied().unwrap_or(0);
    let n = byte(0) as usize;
    let nt = byte(1) as usize * 2;
    let mut p = Vec::with_capacity(n);
    for i in 0..n {
        let xyz = if byte(2) & 1 == 0 {
            [
                byte(8 + i * 3) as f32 - 128.,
                byte(9 + i * 3) as f32 - 128.,
                byte(10 + i * 3) as f32 - 128.,
            ]
        } else {
            let bits = |k| u32::from_le_bytes([byte(k), byte(k + 1), byte(k + 2), byte(k + 3)]);
            [
                f32::from_bits(bits(8 + i * 12)),
                f32::from_bits(bits(12 + i * 12)),
                f32::from_bits(bits(16 + i * 12)),
            ]
        };
        p.push(xyz);
    }
    let mut idx = (0..nt * 3)
        .map(|i| {
            if byte(2) & 2 == 0 && n > 0 {
                byte(7 + i) as u32 % n as u32
            } else {
                byte(7 + i) as u32
            }
        })
        .collect::<Vec<_>>();
    if op == 11 && byte(2) & 16 != 0 {
        // Exercise full-width global IDs and deliberate low-10-bit collisions.
        for v in &mut idx {
            *v = v.wrapping_mul(1024).wrapping_add(0xffff0000);
        }
    }
    if op == 11 && byte(2) & 32 != 0 {
        for (i, v) in idx.iter_mut().enumerate() {
            *v = u32::from_le_bytes([
                byte(i * 4 + 10),
                byte(i * 4 + 11),
                byte(i * 4 + 12),
                byte(i * 4 + 13),
            ]);
        }
    }
    if byte(2) & 64 != 0 {
        idx.pop();
    }
    let positions = Positions::from_packed(&p);
    let mut ws = Workspace::new(Limits {
        max_bytes: if byte(3) & 1 != 0 {
            byte(4) as usize * 64
        } else {
            1 << 24
        },
        max_work: if byte(3) & 2 != 0 {
            byte(4) as u64
        } else {
            1 << 24
        },
    });
    let s = MeshletSettings {
        max_vertices: byte(5) as usize + 1,
        max_triangles: byte(6) as usize * 2 + 1,
    };
    let min = byte(7) as usize + 1;
    let weight = (byte(8) as f32 - 32.) / 128.;
    let split = byte(9) as f32 / 16.;
    match op {
        1 => {
            if let Ok(m) = build_meshlets(&idx, positions, s, weight, &mut ws) {
                check_meshlets(m, &idx, s);
            }
        }
        2 => {
            if let Ok(m) = build_meshlets_scan(&idx, n, s, &mut ws) {
                check_meshlets(m, &idx, s);
            }
        }
        3 => {
            if let Ok(m) = build_meshlets_flex(&idx, positions, s, min, weight, split, &mut ws) {
                check_meshlets(m, &idx, s);
            }
        }
        4 => {
            if let Ok(m) = build_meshlets_spatial(&idx, positions, s, min, split, &mut ws) {
                check_meshlets(m, &idx, s);
            }
        }
        5 => {
            let _ = build_meshlets_bound(
                if byte(2) & 16 != 0 {
                    usize::MAX - byte(10) as usize
                } else {
                    idx.len() + usize::from(byte(2) & 4 != 0)
                },
                s.max_vertices,
                s.max_triangles,
            );
        }
        6 => {
            if let Ok(b) = compute_cluster_bounds(&idx, positions, &mut ws) {
                check_bounds(b);
            }
        }
        7 => {
            let vertices = (0..n)
                .map(|i| {
                    if byte(2) & 4 == 0 {
                        i as u32
                    } else {
                        byte(i + 10) as u32
                    }
                })
                .collect::<Vec<_>>();
            let mut triangles = (0..nt * 3)
                .map(|i| {
                    if n > 0 && byte(2) & 8 == 0 {
                        byte(i + 20) % n as u8
                    } else {
                        byte(i + 20)
                    }
                })
                .collect::<Vec<_>>();
            if byte(2) & 64 != 0 {
                triangles.pop();
            }
            if let Ok(b) = compute_meshlet_bounds(&vertices, &triangles, positions, &mut ws) {
                check_bounds(b);
            }
        }
        8 => {
            let r = (0..n)
                .map(|i| {
                    if byte(2) & 16 == 0 {
                        byte(i + 10) as f32 / 32.
                    } else {
                        f32::from_bits(u32::from_le_bytes([
                            byte(i * 4 + 10),
                            byte(i * 4 + 11),
                            byte(i * 4 + 12),
                            byte(i * 4 + 13),
                        ]))
                    }
                })
                .collect::<Vec<_>>();
            let r = Attributes::from_interleaved(&r, n, 1, 1, 0).unwrap();
            let result = compute_sphere_bounds(
                positions,
                if byte(2) & 4 == 0 { Some(r) } else { None },
                &mut ws,
            );
            if let Ok(b) = result {
                check_bounds(b);
            }
        }
        9 | 10 => {
            let vertices = (0..n).map(|i| i as u32).collect::<Vec<_>>();
            let mut triangles = (0..nt * 3)
                .map(|i| {
                    if n > 0 && byte(2) & 8 == 0 {
                        byte(i + 20) % n as u8
                    } else {
                        byte(i + 20)
                    }
                })
                .collect::<Vec<_>>();
            if byte(2) & 64 != 0 {
                triangles.pop();
            }
            let mut v = vertices.clone();
            let mut t = triangles.clone();
            let level = if op == 9 { 0 } else { byte(7) % 16 };
            let before = (v.clone(), t.clone());
            let result = optimize_meshlet_level_in_place(&mut v, &mut t, level, &mut ws);
            if result.is_err() {
                assert_eq!((v, t), before);
            } else {
                let decoded = t.iter().map(|&i| v[i as usize]).collect::<Vec<_>>();
                let original = before
                    .1
                    .iter()
                    .map(|&i| before.0[i as usize])
                    .collect::<Vec<_>>();
                assert_eq!(canonical(&decoded), canonical(&original));
            }
        }
        11 => {
            if let Ok(local) = extract_meshlet_indices(&idx, &mut ws) {
                for (i, &v) in idx.iter().enumerate() {
                    assert_eq!(local.vertices[local.triangles[i] as usize], v);
                }
            }
        }
        12 => {
            let mut counts = Vec::new();
            let mut at = 0;
            while at < idx.len() {
                let size = (byte(at + 10) as usize % 32 + 1).min(idx.len() - at);
                counts.push(size as u32);
                at += size;
            }
            let result = partition_clusters(
                &idx,
                &counts,
                n,
                if byte(2) & 4 == 0 {
                    Some(positions)
                } else {
                    None
                },
                byte(7) as usize,
                &mut ws,
            );
            if let Ok(r) = result {
                assert_eq!(r.assignments.len(), counts.len());
                assert!(r.assignments.iter().all(|&v| (v as usize) < r.count));
            }
        }
        13 => {
            if let Ok(v) = spatial_sort_remap(positions, &mut ws) {
                permutation(&v, n);
            }
        }
        14 => {
            let mut v = idx.clone();
            let before = v.clone();
            let result = spatial_sort_triangles_in_place(&mut v, positions, &mut ws);
            if result.is_err() {
                assert_eq!(v, before);
            } else {
                assert_eq!(canonical(&v), canonical(&before));
            }
        }
        15 => {
            if let Ok(v) = spatial_cluster_points(positions, byte(7) as usize, &mut ws) {
                permutation(&v, n);
            }
        }
        _ => {}
    }
    assert!(ws.usage().work <= ws.limits().max_work);
    assert!(ws.usage().bytes <= ws.limits().max_bytes);
}
