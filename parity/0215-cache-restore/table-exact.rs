//! Differential table contract against the PR #59 head, including partial writes.
macro_rules! observe {
    ($m:ident, $i:expr, $n:expr, $c:expr, $l:expr, $fuel:expr, $bytes:expr, $len:expr) => {{
        use $m as m;
        let table = m::VertexScoreTable::new($c, $l).unwrap();
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: $bytes,
        });
        let result = m::optimize_vertex_cache_with_table($i, $n, &table, &mut ws);
        let owned = (format!("{result:?}"), ws.usage().work, ws.usage().bytes);
        let mut out = vec![0xdeadbeef; $len];
        let result = m::optimize_vertex_cache_with_table_into(&mut out, $i, $n, &table, &mut ws);
        (
            owned,
            format!("{result:?}"),
            ws.usage().work,
            ws.usage().bytes,
            out,
        )
    }};
}
fn main() {
    let mut meshes = vec![
        (vec![], 0),
        (vec![0, 1], 2),
        (vec![0, 1, 2], 2),
        (vec![0, 1, 2, 3, 4, 5, 2, 1, 6, 6, 6, 6], 7),
        (vec![1, 5, 4, 3, 1, 1, 1, 5, 2, 2, 0, 4, 4, 5, 0], 6),
        (vec![0; 48], 1),
    ];
    let mut seed = 215u32;
    for k in 0..24 {
        let n = 3 + k % 8;
        let i = (0..3 * (k + 1))
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                seed % n
            })
            .collect();
        meshes.push((i, n as usize));
    }
    let defaults = candidate::VertexScoreTable::default();
    let mut tables = vec![
        (*defaults.cache_scores(), *defaults.live_scores()),
        ([0.; 17], [0.; 9]),
        ([-1.; 17], [-2.; 9]),
        ([f32::MAX; 17], [f32::MAX; 9]),
        ([-f32::MAX; 17], [f32::MAX; 9]),
    ];
    let mut cache = [0.; 17];
    cache[2] = 1.;
    cache[4] = -1.;
    tables.push((cache, [1.; 9]));
    tables.push((
        core::array::from_fn(|k| (k as f32 - 8.) * 0.173),
        core::array::from_fn(|k| (4. - k as f32) * 0.219),
    ));
    let mut compared = 0;
    for (indices, n) in &meshes {
        for (cache, live) in &tables {
            let full = observe!(
                baseline,
                indices,
                *n,
                cache,
                live,
                u64::MAX,
                usize::MAX,
                indices.len() + 3
            );
            let fuel = full.0 .1.max(full.2);
            for limit in 0..=fuel + 1 {
                for bytes in [0, full.0 .2.saturating_sub(1), full.0 .2, usize::MAX] {
                    for len in [indices.len().saturating_sub(1), indices.len() + 3] {
                        let a = observe!(baseline, indices, *n, cache, live, limit, bytes, len);
                        let b = observe!(candidate, indices, *n, cache, live, limit, bytes, len);
                        assert_eq!(
                            a, b,
                            "indices={indices:?}, n={n}, fuel={limit}, bytes={bytes}, len={len}"
                        );
                        compared += 2;
                    }
                }
            }
        }
    }
    println!("table compared {compared} differences 0");
}
