//! Connected and pathological geometry: exact result, usage, caller tails,
//! callbacks and position bits at every work budget through success + 1.
macro_rules! mesh {
    ($m:ident, $p:expr, $i:expr, $variant:expr, $fuel:expr) => {{
        use $m as m;
        let mut w = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let (mv, mt, min, weight, split) = $variant;
        let s = m::MeshletSettings {
            max_vertices: mv,
            max_triangles: mt,
        };
        let p = m::Positions::from_packed($p);
        let bound = m::build_meshlets_bound($i.len(), mv, min).unwrap();
        let mut ms = vec![
            m::Meshlet {
                vertex_offset: 123,
                triangle_offset: 234,
                vertex_count: 45,
                triangle_count: 56
            };
            bound + 1
        ];
        let mut vs = vec![0xdeadbeef; $i.len() + 1];
        let mut ts = vec![0xa5; $i.len() + 1];
        let r = m::build_meshlets_flex_into(
            &mut ms, &mut vs, &mut ts, $i, p, s, min, weight, split, &mut w,
        );
        let first = format!("{r:?} {:?} {ms:?} {vs:?} {ts:?}", w.usage());
        let owned = m::build_meshlets_flex($i, p, s, min, weight, split, &mut w);
        (format!("{first} {owned:?} {:?}", w.usage()), w.usage().work)
    }};
}

macro_rules! lod {
    ($m:ident, $p:expr, $i:expr, $variant:expr, $fuel:expr) => {{
        use $m as m;
        let mut w = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut p = $p.to_vec();
        let mut c = if $variant % 2 == 0 {
            m::clusterlod::default_config(4)
        } else {
            m::clusterlod::default_config_rt(4)
        }
        .unwrap();
        let variant = $variant / 2;
        match variant {
            0 => {}
            1 => c.partition_spatial = !c.partition_spatial,
            2 => c.partition_sort = !c.partition_sort,
            3 => c.partition_size = 2,
            4 => c.cluster_spatial = !c.cluster_spatial,
            5 => c.cluster_fill_weight = 1.,
            6 => c.cluster_split_factor = 0.,
            7 => c.simplify_ratio = 0.75,
            8 => c.simplify_threshold = 0.5,
            9 => c.simplify_error_merge_previous = 0.5,
            10 => c.simplify_error_merge_additive = 0.5,
            11 => c.simplify_error_factor_sloppy = 1.,
            12 => c.simplify_error_edge_limit = 0.5,
            13 => c.simplify_error_clamped = !c.simplify_error_clamped,
            14 => c.simplify_permissive = !c.simplify_permissive,
            15 => c.simplify_fallback_permissive = !c.simplify_fallback_permissive,
            16 => c.simplify_fallback_sloppy = !c.simplify_fallback_sloppy,
            17 => c.simplify_regularize = !c.simplify_regularize,
            18 => c.simplify_preserve_folds = !c.simplify_preserve_folds,
            19 => c.simplify_dilate_borders = !c.simplify_dilate_borders,
            20 => c.optimize_bounds = !c.optimize_bounds,
            21 => c.optimize_clusters = !c.optimize_clusters,
            22 => c.optimize_clusters_level = 9,
            _ => {}
        }
        let width = match variant {
            23 => 1,
            24 => 3,
            25 => 8,
            26 => 32,
            _ => 0,
        };
        let a: Vec<f32> = (0..p.len() * width).map(|j| (j % 7) as f32 / 10.).collect();
        let weights = vec![0.5; width];
        let locks: Vec<_> = (0..p.len())
            .map(|j| m::VertexFlags::from_bits(if j % 3 == 0 { 1 } else { 0 }).unwrap())
            .collect();
        let mut records = Vec::new();
        let r = m::clusterlod::build_with_output(
            c,
            m::clusterlod::Mesh {
                indices: $i,
                positions: &mut p,
                attributes: if width == 0 {
                    None
                } else {
                    Some(m::Attributes::from_interleaved(&a, $p.len(), width, width, 0).unwrap())
                },
                attribute_weights: &weights,
                vertex_lock: if variant >= 24 { Some(&locks) } else { None },
                attribute_protect_mask: if width > 0 { 1 } else { 0 },
            },
            |g, clusters| {
                records.extend([g.depth as u32, clusters.len() as u32]);
                records.extend(g.simplified.center.map(f32::to_bits));
                records.extend([g.simplified.radius.to_bits(), g.simplified.error.to_bits()]);
                for c in clusters {
                    records.extend([
                        c.refined as u32,
                        c.vertex_count as u32,
                        c.indices.len() as u32,
                    ]);
                    records.extend(c.bounds.center.map(f32::to_bits));
                    records.extend([c.bounds.radius.to_bits(), c.bounds.error.to_bits()]);
                    records.extend(&c.indices);
                }
                if variant == 27 {
                    Err(m::Error::InvalidParameter)
                } else {
                    Ok(records.len() as i32)
                }
            },
            &mut w,
        );
        let bits: Vec<_> = p.iter().flat_map(|v| v.map(f32::to_bits)).collect();
        (
            format!("{r:?} {:?} {records:?} {bits:?}", w.usage()),
            w.usage().work,
        )
    }};
}

fn main() {
    let mut p = Vec::new();
    for y in 0..=2 {
        for x in 0..=3 {
            p.push([x as f32, y as f32, (x * y) as f32 / 10.]);
        }
    }
    let mut i = Vec::new();
    for y in 0..2 {
        for x in 0..3 {
            let a = y * 4 + x;
            i.extend([a, a + 1, a + 4, a + 4, a + 1, a + 5]);
        }
    }
    let mut cases = vec![
        (p.clone(), i.clone()),
        (p.clone(), [i.clone(), i.clone()].concat()),
        (vec![[0.; 3]; p.len()], i.clone()),
    ];
    p[0][0] = 3e9;
    cases.push((p.clone(), i.clone()));
    p[0][0] = f32::NAN;
    cases.push((p, i));
    let mut compared = 0u64;
    for (ci, (p, i)) in cases.iter().enumerate() {
        for settings in [
            (8, 4, 4, 0., 0.),
            (8, 4, 4, 1., 0.),
            (8, 4, 1, 0., 2.),
            (8, 4, 2, 0.75, 0.),
            (3, 1, 1, 0.5, 1.),
            (256, 512, 1, 0., 2.),
        ] {
            let work = mesh!(baseline, p, i, settings, u64::MAX).1;
            for fuel in 0..=work + 1 {
                assert_eq!(
                    mesh!(baseline, p, i, settings, fuel),
                    mesh!(candidate, p, i, settings, fuel),
                    "mesh {ci} {settings:?} {fuel}"
                );
                compared += 1;
            }
        }
        for variant in 0..56 {
            let work = lod!(baseline, p, i, variant, u64::MAX).1;
            for fuel in 0..=work + 1 {
                assert_eq!(
                    lod!(baseline, p, i, variant, fuel),
                    lod!(candidate, p, i, variant, fuel),
                    "lod {ci} {variant} {fuel}"
                );
                compared += 1;
            }
        }
        println!("case {ci} compared {compared} differences 0");
    }
    // Reach the >=256-triangle candidate list with ties and nonzero weight.
    let mut p = Vec::new();
    let mut i = Vec::new();
    for y in 0..=12 {
        for x in 0..=16 {
            p.push([x as f32, y as f32, 0.]);
        }
    }
    for y in 0..12 {
        for x in 0..16 {
            let a = y * 17 + x;
            i.extend([a, a + 1, a + 17, a + 17, a + 1, a + 18]);
        }
    }
    for settings in [(64, 124, 41, 0., 2.), (64, 124, 41, 0.75, 2.)] {
        let work = mesh!(baseline, &p, &i, settings, u64::MAX).1;
        for fuel in 0..=work + 1 {
            assert_eq!(
                mesh!(baseline, &p, &i, settings, fuel),
                mesh!(candidate, &p, &i, settings, fuel),
                "listed {settings:?} {fuel}"
            );
            compared += 1;
        }
        println!("listed {settings:?} compared {compared} differences 0");
    }
}
