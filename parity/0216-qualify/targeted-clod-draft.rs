//! Selected cluster-LOD meshes crossed with all configuration booleans.
#[derive(Debug, PartialEq)]
struct Observation {
    result: String,
    work: u64,
    bytes: usize,
    records: Vec<u32>,
    positions: Vec<u32>,
}
macro_rules! observe {
    ($lib:ident, $p:expr, $indices:expr, $bits:expr, $width:expr, $lock:expr, $limit:expr) => {{
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
        let width = $width;
        let values: Vec<f32> = (0..p.len() * width).map(|i| (i % 11) as f32 * 0.125).collect();
        let attributes = m::Attributes::from_interleaved(&values, p.len(), width, width, 0).unwrap();
        let weights = vec![1.0f32; width];
        let flags: Vec<_> = (0..p.len()).map(|i| {
            let locked = match $lock { 0 => false, 1 => true, 2 => i % 4 == 0,
                _ => { let v = p[i]; v[0] == 0. || v[1] == 0. || v[0] == 3. || v[1] == 3. } };
            m::VertexFlags::from_bits(if locked { 1 } else { 0 }).unwrap()
        }).collect();
        let mut w = m::Workspace::new(m::Limits { max_work: $limit, ..Default::default() });
        let mut observations = Vec::new();
        for _ in 0..2 {
        let mut p = $p.to_vec();
        let mut records = Vec::<u32>::new();
        let result = m::clusterlod::build_with_output(c, m::clusterlod::Mesh {
            indices, positions: &mut p, attributes: Some(attributes), vertex_lock: if $lock == 0 { None } else { Some(&flags) },
            attribute_weights: &weights, attribute_protect_mask: 0,
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
        observations.push(Observation { result: format!("{result:?}"), work: w.usage().work,
            bytes: w.usage().bytes, records, positions });
        }
        let max_work = observations.iter().map(|o| o.work).max().unwrap();
        (observations, max_work)
    }};
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let start: u32 = args[1].parse().unwrap();
    let end: u32 = args[2].parse().unwrap();
    assert!(start < end && end <= 32768);
    let mut p = Vec::new();
    for y in 0..4 { for x in 0..4 { p.push([x as f32, y as f32, ((x*y)%3) as f32 * 0.1]); } }
    let mut indices = Vec::new();
    for y in 0..3 { for x in 0..3 { let i = y*4+x; indices.extend([i,i+1,i+4,i+1,i+5,i+4]); } }
    let mut checks = 0u64;
    for cell in start..end {
        let bits = cell % 4096;
        let width = [1usize,3][(cell / 4096 % 2) as usize];
        let lock = cell / 8192;
        let a = observe!(baseline, &p, &indices, bits, width, lock, u64::MAX);
        let b = observe!(candidate, &p, &indices, bits, width, lock, u64::MAX);
        assert_eq!(a,b,"unlimited cell={cell}"); checks += 2;
        let success = a.1;
        assert!(a.0[0].result.starts_with("Ok"), "unlimited call failed cell={cell}: {:?}", a.0[0].result);
        let mut first_failure = None;
        for limit in (0..success).rev() {
            let a = observe!(baseline, &p, &indices, bits, width, lock, limit);
            let b = observe!(candidate, &p, &indices, bits, width, lock, limit);
            assert_eq!(a,b,"boundary search cell={cell} limit={limit}"); checks += 2;
            if a.0[0].result.starts_with("Err") { first_failure = Some(limit); break; }
        }
        let mut limits = vec![0, success, success.saturating_add(1)];
        if let Some(limit) = first_failure { limits.push(limit); }
        limits.extend((0..=success).step_by(64));
        limits.sort_unstable(); limits.dedup();
        for limit in limits {
            let a = observe!(baseline, &p, &indices, bits, width, lock, limit);
            let b = observe!(candidate, &p, &indices, bits, width, lock, limit);
            assert_eq!(a,b,"cell={cell} limit={limit}"); checks += 2;
        }
        if cell % 256 == 255 { println!("cell={cell} observations={checks}"); }
    }
    println!("PASS cells={} observations={checks}",end-start);
}
