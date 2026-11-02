//! Reproduce topology repair measurements on external binary records.
use meshoptimizer_rs::*;
use std::time::Instant;
/// `REPAIR4_TOLERANCE_REL` sets an absolute tolerance of that fraction of the
/// largest bounds extent; unset selects the library default.
fn tolerance(p: &[[f32; 3]]) -> Option<f32> {
    let rel: f32 = std::env::var("REPAIR4_TOLERANCE_REL").ok()?.parse().ok()?;
    let extent = (0..3)
        .map(|a| {
            let lo = p.iter().map(|v| v[a]).fold(f32::INFINITY, f32::min);
            let hi = p.iter().map(|v| v[a]).fold(f32::NEG_INFINITY, f32::max);
            hi - lo
        })
        .fold(0., f32::max);
    Some(rel * extent)
}
fn measure(name: &str, p: &[[f32; 3]], i: &[u32]) {
    // Repeated-index triangles are rejected by the repair functions (sanitation
    // removes them first in `process_mesh`), so drop them and report the count.
    let kept: Vec<u32> = i
        .as_chunks::<3>()
        .0
        .iter()
        .filter(|t| t[0] != t[1] && t[1] != t[2] && t[2] != t[0])
        .flatten()
        .copied()
        .collect();
    if kept.len() != i.len() {
        eprintln!(
            "{name}: dropped {} degenerate triangles",
            (i.len() - kept.len()) / 3
        );
    }
    let i = &kept[..];
    let mut ws = Workspace::default();
    let before =
        analyze_mesh(i, Positions::from_packed(p), AnalyzeOptions::EMPTY, &mut ws).unwrap();
    let data: Vec<_> = p
        .iter()
        .flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes()))
        .collect();
    let start = Instant::now();
    let stitched = stitch_cracks(
        i,
        VertexStream::packed(&data, 12).unwrap(),
        Positions::from_packed(p),
        tolerance(p),
        &mut ws,
    );
    let stitched = match stitched {
        Ok(s) => s,
        Err(e) => return println!("{name}\tstitch error {e:?}"),
    };
    let split = split_non_manifold_vertices(
        &stitched.indices,
        VertexStream::packed(&stitched.vertices, 12).unwrap(),
        &mut ws,
    );
    let split = match split {
        Ok(s) => s,
        Err(e) => return println!("{name}\tsplit error {e:?}"),
    };
    let positions: Vec<[f32; 3]> = split
        .vertices
        .as_chunks::<12>()
        .0
        .iter()
        .map(|p| [0, 1, 2].map(|a| f32::from_le_bytes(p[a * 4..a * 4 + 4].try_into().unwrap())))
        .collect();
    let r = split;
    let time = start.elapsed().as_micros();
    let after = analyze_mesh(
        &r.indices,
        Positions::from_packed(&positions),
        AnalyzeOptions::EMPTY,
        &mut ws,
    )
    .unwrap();
    let simplify_to = |indices: &[u32], pos: &[[f32; 3]], ws: &mut Workspace| {
        simplify(
            indices,
            Positions::from_packed(pos),
            SimplifySettings {
                target_index_count: indices.len() / 10,
                target_error: 0.01,
                options: SimplifyOptions::EMPTY,
            },
            ws,
        )
        .unwrap()
        .indices
        .len()
            / 3
    };
    println!(
        "{name}\t{}\t{}\t{}\t{}\t{}\t{}\t{time}\t{}\t{}\t{}\t{}\t{}",
        before.boundary_edges,
        after.boundary_edges,
        before.non_manifold_vertices,
        after.non_manifold_vertices,
        before.non_manifold_edges,
        after.non_manifold_edges,
        simplify_to(i, p, &mut ws),
        simplify_to(&r.indices, &positions, &mut ws),
        stitched.report.stitched_pairs,
        stitched.report.t_junction_splits,
        stitched.report.refused
    );
}
fn main() {
    println!("mesh\tboundary_before\tboundary_after\tnm_vertex_before\tnm_vertex_after\tnm_edge_before\tnm_edge_after\trepair_us\tlod_before\tlod_after\tpairs\tt_splits\trefused");
    for path in std::env::args().skip(1) {
        let raw = std::fs::read(&path).unwrap();
        let words: Vec<_> = raw
            .as_chunks::<4>()
            .0
            .iter()
            .map(|w| u32::from_le_bytes(*w))
            .collect();
        let nv = words[0] as usize;
        let ni = words[1] as usize;
        let p: Vec<_> = words[2..2 + nv * 3]
            .as_chunks::<3>()
            .0
            .iter()
            .map(|v: &[u32; 3]| v.map(f32::from_bits))
            .collect();
        measure(
            std::path::Path::new(&path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
            &p,
            &words[2 + nv * 3..2 + nv * 3 + ni],
        );
    }
    measure(
        "bowtie",
        &[
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [-1., 0., 0.],
            [0., -1., 0.],
        ],
        &[0, 1, 2, 0, 3, 4],
    );
    measure(
        "gap",
        &[
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [1.0000001, 0., 0.],
            [0.0000001, 1., 0.],
            [1., 1., 0.],
        ],
        &[0, 1, 2, 4, 3, 5],
    );
    measure(
        "t_junction",
        &[
            [0., 0., 0.],
            [2., 0., 0.],
            [0., 1., 0.],
            [1., 0., 0.],
            [1., -1., 0.],
        ],
        &[0, 1, 2, 3, 0, 4, 1, 3, 4],
    );
}
