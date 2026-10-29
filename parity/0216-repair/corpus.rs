//! Properties on the shared, generated runner corpus (binary mesh records).
use candidate::*;
fn main() {
    let directory = std::env::args_os().nth(1).unwrap();
    let mut paths: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    let mut count = 0;
    for path in paths {
        let raw = std::fs::read(&path).unwrap();
        let words: Vec<_> = raw
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        let (nv, ni) = (words[0] as usize, words[1] as usize);
        let p: Vec<[f32; 3]> = words[2..2 + nv * 3]
            .chunks_exact(3)
            .map(|v| {
                [
                    f32::from_bits(v[0]),
                    f32::from_bits(v[1]),
                    f32::from_bits(v[2]),
                ]
            })
            .collect();
        let i = &words[2 + nv * 3..2 + nv * 3 + ni];
        let positions = Positions::from_packed(&p);
        let mut ws = Workspace::default();
        let expected = generate_position_remap(positions, &mut ws).unwrap();
        let welded = weld_vertices(positions, 0., WeldPolicy::PositionsOnly, &mut ws).unwrap();
        assert_eq!(welded.remap, expected);
        let clean = sanitize(i, positions, SanitizeOptions::EMPTY, &mut ws).unwrap();
        let oriented = orient_faces(&clean.indices, nv, &mut ws).unwrap();
        if oriented.report.conflicts.is_empty() {
            let again = orient_faces(&oriented.indices, nv, &mut ws).unwrap();
            assert_eq!(again.indices, oriented.indices);
            assert!(again.report.flipped_faces.is_empty());
        }
        let before =
            analyze_mesh(&clean.indices, positions, AnalyzeOptions::EMPTY, &mut ws).unwrap();
        if before.orientation_conflicts == 0 && before.non_manifold_edges == 0 {
            assert_eq!(oriented.indices, clean.indices);
        }
        let filled = fill_holes(&oriented.indices, positions, 16, &mut ws).unwrap();
        assert_eq!(&filled.indices[..oriented.indices.len()], &oriented.indices);
        assert!(filled.indices.iter().all(|&v| (v as usize) < nv));
        assert_eq!(filled.indices.len(), filled.report.output_triangles * 3);
        println!(
            "PASS {} vertices={nv} indices={ni} fills={} refused={}",
            path.file_name().unwrap().to_string_lossy(),
            filled.report.filled.len(),
            filled.report.too_large.len()
                + filled.report.ambiguous.len()
                + filled.report.non_simple.len()
        );
        count += 1;
    }
    assert!(count > 0);
    println!("PASS {count} shared corpus meshes; zero-tolerance grouping, consistent orientation no-op, fill original-vertex bounds");
}
