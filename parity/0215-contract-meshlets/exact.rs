use meshoptimizer_rs as m;
use std::fmt::Write;

fn run_mesh(p: &[[f32; 3]], indices: &[u32], variant: usize, limit: u64) -> String {
    let mut w = m::Workspace::new(m::Limits {
        max_work: limit,
        ..Default::default()
    });
    let settings = m::MeshletSettings {
        max_vertices: 8,
        max_triangles: 4,
    };
    let bound = m::build_meshlets_bound(indices.len(), 8, 1).unwrap();
    let marker = m::Meshlet {
        vertex_offset: 123,
        triangle_offset: 234,
        vertex_count: 45,
        triangle_count: 56,
    };
    let mut ms = vec![marker; bound + 2];
    let mut vs = vec![0xdeadbeef; indices.len() + 2];
    let mut ts = vec![0xa5; indices.len() + 2];
    let p = m::Positions::from_packed(p);
    let r = match variant {
        0 => m::build_meshlets_into(&mut ms, &mut vs, &mut ts, indices, p, settings, 0., &mut w),
        1 => m::build_meshlets_into(
            &mut ms, &mut vs, &mut ts, indices, p, settings, 0.75, &mut w,
        ),
        2 => m::build_meshlets_flex_into(
            &mut ms, &mut vs, &mut ts, indices, p, settings, 1, 0., 2., &mut w,
        ),
        3 => m::build_meshlets_flex_into(
            &mut ms, &mut vs, &mut ts, indices, p, settings, 2, 0.75, 0., &mut w,
        ),
        _ => m::build_meshlets_spatial_into(
            &mut ms, &mut vs, &mut ts, indices, p, settings, 1, 0.5, &mut w,
        ),
    };
    let into_usage = w.usage();
    let mut owned = m::Workspace::new(m::Limits {
        max_work: limit,
        ..Default::default()
    });
    let allocated = match variant {
        0 => m::build_meshlets(indices, p, settings, 0., &mut owned),
        1 => m::build_meshlets(indices, p, settings, 0.75, &mut owned),
        2 => m::build_meshlets_flex(indices, p, settings, 1, 0., 2., &mut owned),
        3 => m::build_meshlets_flex(indices, p, settings, 2, 0.75, 0., &mut owned),
        _ => m::build_meshlets_spatial(indices, p, settings, 1, 0.5, &mut owned),
    };
    format!(
        "{r:?} {into_usage:?} {ms:?} {vs:?} {ts:?} {allocated:?} {:?}",
        owned.usage()
    )
}

fn run_lod(p: &[[f32; 3]], indices: &[u32], variant: usize, limit: u64) -> (String, u64) {
    let mut w = m::Workspace::new(m::Limits {
        max_work: limit,
        ..Default::default()
    });
    let mut p = p.to_vec();
    let mut c = if variant & 1 == 0 {
        m::clusterlod::default_config(4).unwrap()
    } else {
        m::clusterlod::default_config_rt(4).unwrap()
    };
    c.simplify_dilate_borders = variant & 2 != 0;
    match variant {
        4 => c.partition_spatial = !c.partition_spatial,
        5 => c.partition_sort = !c.partition_sort,
        6 => c.partition_size = 2,
        7 => c.cluster_spatial = !c.cluster_spatial,
        8 => c.cluster_fill_weight = 1.,
        9 => c.cluster_split_factor = 0.,
        10 => c.simplify_ratio = 0.75,
        11 => c.simplify_threshold = 0.5,
        12 => c.simplify_error_merge_previous = 0.5,
        13 => c.simplify_error_merge_additive = 0.5,
        14 => c.simplify_error_factor_sloppy = 1.,
        15 => c.simplify_error_edge_limit = 0.5,
        16 => c.simplify_error_clamped = !c.simplify_error_clamped,
        17 => c.simplify_permissive = !c.simplify_permissive,
        18 => c.simplify_fallback_permissive = !c.simplify_fallback_permissive,
        19 => c.simplify_fallback_sloppy = !c.simplify_fallback_sloppy,
        20 => c.simplify_regularize = !c.simplify_regularize,
        21 => c.simplify_preserve_folds = !c.simplify_preserve_folds,
        22 => c.optimize_bounds = !c.optimize_bounds,
        23 => c.optimize_clusters = !c.optimize_clusters,
        24 => c.optimize_clusters_level = 9,
        _ => {}
    }
    let mut records = Vec::<u32>::new();
    let result = m::clusterlod::build_with_output(
        c,
        m::clusterlod::Mesh {
            indices,
            positions: &mut p,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        |g, clusters| {
            let id = records.len() as i32;
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
            Ok(id)
        },
        &mut w,
    );
    let bits: Vec<_> = p.iter().flat_map(|v| v.map(f32::to_bits)).collect();
    (
        format!("{result:?} {:?} {records:?} {bits:?}", w.usage()),
        w.usage().work,
    )
}

fn main() {
    let mut p = Vec::new();
    let mut indices = Vec::new();
    for i in 0..24 {
        let x = (i % 6) as f32 * 10.;
        let y = (i / 6) as f32 * 10.;
        p.extend([[x, y, 0.], [x + 1., y, 0.], [x, y + 1., 0.]]);
        indices.extend([3 * i, 3 * i + 1, 3 * i + 2]);
    }
    let mut cases = vec![
        (p.clone(), indices.clone()),
        (vec![[0.; 3]; p.len()], indices.clone()),
        (vec![[0.; 3]; 3], vec![0, 1, 2]),
    ];
    p[0][0] = 1.0e9;
    cases.push((p, indices));
    for (ci, (p, indices)) in cases.iter().enumerate() {
        for variant in 0..5 {
            let mut w = m::Workspace::default();
            let settings = m::MeshletSettings {
                max_vertices: 8,
                max_triangles: 4,
            };
            let pos = m::Positions::from_packed(p);
            let _ = match variant {
                0 => m::build_meshlets(indices, pos, settings, 0., &mut w),
                1 => m::build_meshlets(indices, pos, settings, 0.75, &mut w),
                2 => m::build_meshlets_flex(indices, pos, settings, 1, 0., 2., &mut w),
                3 => m::build_meshlets_flex(indices, pos, settings, 2, 0.75, 0., &mut w),
                _ => m::build_meshlets_spatial(indices, pos, settings, 1, 0.5, &mut w),
            }
            .unwrap();
            let max = w.usage().work + 1;
            let mut output = String::new();
            for limit in 0..=max {
                writeln!(
                    output,
                    "{ci} {variant} {limit} {}",
                    run_mesh(p, indices, variant, limit)
                )
                .unwrap();
            }
            print!("{output}");
        }
    }
    for (ci, (p, indices)) in cases.iter().enumerate() {
        for variant in 0..4 {
            let (_, work) = run_lod(p, indices, variant, u64::MAX);
            for limit in 0..=work + 1 {
                println!(
                    "lod {ci} {variant} {limit} {}",
                    run_lod(p, indices, variant, limit).0
                );
            }
        }
    }
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let indices = [0, 1, 2, 2, 1, 3];
    for variant in 4..25 {
        let (_, work) = run_lod(&p, &indices, variant, u64::MAX);
        for limit in 0..=work + 1 {
            println!(
                "lod-option {variant} {limit} {}",
                run_lod(&p, &indices, variant, limit).0
            );
        }
    }
}
