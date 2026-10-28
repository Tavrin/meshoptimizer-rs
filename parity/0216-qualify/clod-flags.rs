//! Selected cluster-LOD meshes crossed with all configuration booleans.
macro_rules! observe {
    ($lib:ident, $p:expr, $indices:expr, $bits:expr, $limit:expr) => {{
        use $lib as m;
        let mut p = $p.to_vec();
        let indices = $indices;
        let bits = $bits;
        let mut c = m::clusterlod::default_config(4).unwrap();
        c.partition_spatial = bits & 1 != 0;
        c.partition_sort = bits & 2 != 0;
        c.cluster_spatial = bits & 4 != 0;
        c.simplify_error_clamped = bits & 8 != 0;
        c.simplify_permissive = bits & 16 != 0;
        c.simplify_fallback_permissive = bits & 32 != 0;
        c.simplify_fallback_sloppy = bits & 64 != 0;
        c.simplify_regularize = bits & 128 != 0;
        c.simplify_preserve_folds = bits & 256 != 0;
        c.simplify_dilate_borders = bits & 512 != 0;
        c.optimize_bounds = bits & 1024 != 0;
        c.optimize_clusters = bits & 2048 != 0;
        let mut w = m::Workspace::new(m::Limits { max_work: $limit, ..Default::default() });
        let mut records = Vec::<u32>::new();
        let result = m::clusterlod::build_with_output(c, m::clusterlod::Mesh {
            indices, positions: &mut p, attributes: None, vertex_lock: None,
            attribute_weights: &[], attribute_protect_mask: 0,
        }, |g, clusters| {
            let id = records.len() as i32;
            records.extend([g.depth as u32, clusters.len() as u32]);
            records.extend(g.simplified.center.map(f32::to_bits));
            records.extend([g.simplified.radius.to_bits(), g.simplified.error.to_bits()]);
            for c in clusters {
                records.extend([c.refined as u32, c.vertex_count as u32, c.indices.len() as u32]);
                records.extend(c.bounds.center.map(f32::to_bits));
                records.extend([c.bounds.radius.to_bits(), c.bounds.error.to_bits()]);
                records.extend(&c.indices);
            }
            Ok(id)
        }, &mut w);
        let positions: Vec<_> = p.iter().flat_map(|v| v.map(f32::to_bits)).collect();
        (format!("{result:?} {:?} {records:?} {positions:?}", w.usage()), w.usage().work)
    }};
}
fn main() {
    let mut p = Vec::new();
    let mut indices = Vec::new();
    for i in 0..24 {
        let x = (i % 6) as f32 * 10.;
        let y = (i / 6) as f32 * 10.;
        p.extend([[x,y,0.], [x+1.,y,0.], [x,y+1.,0.]]);
        indices.extend([3*i,3*i+1,3*i+2]);
    }
    let mut cases = vec![(p.clone(), indices.clone()), (vec![[0.;3];p.len()], indices.clone()), (vec![[0.;3];3], vec![0,1,2])];
    p[0][0] = 1.0e9;
    cases.push((p,indices));
    let mut checks = 0u64;
    for (case, (p, indices)) in cases.iter().enumerate() {
        for bits in 0..4096u32 {
            let a = observe!(baseline, p, indices, bits, u64::MAX);
            let b = observe!(candidate, p, indices, bits, u64::MAX);
            assert_eq!(a, b, "unlimited case={case} bits={bits}");
            for limit in 0..=a.1 + 1 {
                let a = observe!(baseline, p, indices, bits, limit);
                let b = observe!(candidate, p, indices, bits, limit);
                assert_eq!(a, b, "case={case} bits={bits} limit={limit}");
                checks += 1;
            }
            if bits % 256 == 255 { println!("case={case} boolean_masks={} observations={checks}", bits + 1); }
        }
    }
    println!("PASS observations={checks}");
}
