use meshoptimizer_rs::*;

const QUAD: [[f32; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];

fn run(indices: &[u32], positions: &[[f32; 3]], options: SanitizeOptions) -> SanitizedMesh {
    sanitize(
        indices,
        Positions::from_packed(positions),
        options,
        &mut Workspace::default(),
    )
    .unwrap()
}

#[test]
fn invalid_triangles_are_dropped_and_counted_by_default() {
    let mut positions = QUAD.to_vec();
    positions.push([f32::INFINITY, 0., 0.]);
    let indices = [0, 1, 9, 0, 1, 2, 0, 4, 1, 4, 9, 1, 2, 1, 3];
    let mesh = run(&indices, &positions, SanitizeOptions::EMPTY);
    assert_eq!(mesh.indices, [0, 1, 2, 2, 1, 3]);
    let r = mesh.report;
    assert_eq!((r.out_of_range, r.non_finite), (2, 1));
    assert_eq!(
        (r.input_triangles, r.output_triangles, r.removed()),
        (5, 2, 3)
    );
}

#[test]
fn reject_invalid_returns_the_first_invalid_triangle_error() {
    let mut positions = QUAD.to_vec();
    positions[3][1] = f32::NAN;
    let mut ws = Workspace::default();
    let p = Positions::from_packed(&positions);
    let reject = SanitizeOptions::REJECT_INVALID;
    for (indices, error) in [
        (vec![0, 1, 2, 0, 1, 7, 0, 1, 3], Error::IndexOutOfBounds),
        (vec![0, 1, 2, 0, 1, 3, 0, 1, 7], Error::NumericalFailure),
    ] {
        assert_eq!(sanitize(&indices, p, reject, &mut ws), Err(error));
        let mut out = [9; 9];
        assert_eq!(
            sanitize_into(&mut out, &indices, p, reject, &mut ws),
            Err(error)
        );
        assert_eq!(out, [9; 9]);
    }
    // Repeated indices are not invalid: they are still dropped.
    assert_eq!(
        sanitize(&[0, 0, 1], p, reject, &mut ws)
            .unwrap()
            .report
            .repeated_index,
        1
    );
}

#[test]
fn incomplete_triangle_is_invalid_topology_under_every_policy() {
    let p = Positions::from_packed(&QUAD);
    for options in [SanitizeOptions::EMPTY, SanitizeOptions::REJECT_INVALID] {
        let result = sanitize(&[0, 1, 2, 3], p, options, &mut Workspace::default());
        assert_eq!(result, Err(Error::InvalidTopology));
    }
}

#[test]
fn repeated_index_triangles_are_removed() {
    let mesh = run(
        &[0, 0, 1, 0, 1, 2, 2, 1, 2, 3, 3, 3],
        &QUAD,
        SanitizeOptions::EMPTY,
    );
    assert_eq!(mesh.indices, [0, 1, 2]);
    assert_eq!(mesh.report.repeated_index, 3);
}

#[test]
fn zero_area_is_opt_in_and_compares_exact_positions() {
    // Vertex 4 equals vertex 1 up to the sign of zero; vertex 3 is collinear with 0 and 1.
    let positions = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [2., 0., 0.],
        [1., -0., 0.],
    ];
    let indices = [0, 1, 2, 0, 1, 4, 0, 1, 3];
    assert_eq!(
        run(&indices, &positions, SanitizeOptions::EMPTY).indices,
        indices
    );
    let mesh = run(&indices, &positions, SanitizeOptions::ZERO_AREA);
    assert_eq!(mesh.indices, [0, 1, 2, 0, 1, 3]);
    assert_eq!(mesh.report.zero_area, 1);
}

#[test]
fn rotated_duplicates_are_removed_and_reverse_winding_is_kept_by_default() {
    let indices = [0, 1, 2, 1, 2, 0, 2, 0, 1, 0, 2, 1, 2, 1, 0];
    let mesh = run(&indices, &QUAD, SanitizeOptions::EMPTY);
    assert_eq!(mesh.indices, [0, 1, 2, 0, 2, 1]);
    assert_eq!(
        (mesh.report.duplicate, mesh.report.reversed_duplicate),
        (3, 0)
    );
}

#[test]
fn ignore_orientation_removes_reverse_wound_copies() {
    let indices = [0, 2, 1, 0, 1, 2, 2, 1, 3, 1, 2, 3];
    let mesh = run(&indices, &QUAD, SanitizeOptions::IGNORE_ORIENTATION);
    assert_eq!(mesh.indices, [0, 2, 1, 2, 1, 3]);
    assert_eq!(
        (mesh.report.duplicate, mesh.report.reversed_duplicate),
        (0, 2)
    );
}

#[test]
fn weld_positions_finds_duplicates_across_split_vertices_without_rewriting() {
    // Vertices 4..6 repeat 0..2, as at a UV seam.
    let mut positions = QUAD.to_vec();
    positions.extend_from_slice(&QUAD[..3]);
    let indices = [0, 1, 2, 5, 6, 4, 2, 1, 3];
    assert_eq!(
        run(&indices, &positions, SanitizeOptions::EMPTY).indices,
        indices
    );
    let mesh = run(&indices, &positions, SanitizeOptions::WELD_POSITIONS);
    assert_eq!(mesh.indices, [0, 1, 2, 2, 1, 3]);
    assert_eq!(mesh.report.duplicate, 1);
    // The same weld through an interleaved view.
    let floats: Vec<f32> = positions
        .iter()
        .flat_map(|p| [p[0], p[1], p[2], 7.])
        .collect();
    let view = Positions::from_interleaved(&floats, positions.len(), 4, 0).unwrap();
    let options = SanitizeOptions::WELD_POSITIONS;
    let viewed = sanitize(&indices, view, options, &mut Workspace::default()).unwrap();
    assert_eq!(viewed, mesh);
}

#[test]
fn into_checks_capacity_before_writing_and_matches_the_allocating_form() {
    let indices = [0, 1, 2, 1, 2, 0, 2, 1, 3];
    let p = Positions::from_packed(&QUAD);
    let mut ws = Workspace::default();
    let expected = sanitize(&indices, p, SanitizeOptions::EMPTY, &mut ws).unwrap();
    let mut short = [7; 5];
    let result = sanitize_into(&mut short, &indices, p, SanitizeOptions::EMPTY, &mut ws);
    assert_eq!(result, Err(Error::BufferTooSmall));
    assert_eq!(short, [7; 5]);
    let mut out = [7; 10];
    assert!(out.len() >= sanitize_bound(indices.len()));
    let report = sanitize_into(&mut out, &indices, p, SanitizeOptions::EMPTY, &mut ws).unwrap();
    assert_eq!(report, expected.report);
    assert_eq!(out[..report.index_count()], expected.indices[..]);
    assert_eq!(out[6..], [7; 4]);
}

#[test]
fn limits_are_enforced_without_partial_output() {
    let indices = [0, 1, 2, 2, 1, 3];
    let p = Positions::from_packed(&QUAD);
    let mut ws = Workspace::default();
    sanitize(&indices, p, SanitizeOptions::EMPTY, &mut ws).unwrap();
    let used = ws.usage();
    for limits in [
        Limits {
            max_work: used.work - 1,
            ..Limits::default()
        },
        Limits {
            max_bytes: used.bytes - 1,
            ..Limits::default()
        },
    ] {
        let mut ws = Workspace::new(limits);
        assert_eq!(
            sanitize(&indices, p, SanitizeOptions::EMPTY, &mut ws),
            Err(Error::LimitExceeded)
        );
        let mut out = [9; 6];
        let _ = sanitize_into(&mut out, &indices, p, SanitizeOptions::EMPTY, &mut ws);
        assert_eq!(out, [9; 6]);
    }
}

#[test]
fn unknown_option_bits_are_rejected() {
    assert_eq!(SanitizeOptions::from_bits(16), Err(Error::UnknownFlags));
    let all = SanitizeOptions::from_bits(15).unwrap();
    assert!(all.contains(SanitizeOptions::WELD_POSITIONS | SanitizeOptions::ZERO_AREA));
}

/// Seeded property check over random corrupted and clean grids: the output is
/// an in-order subsequence of input triangles, counts add up, and a clean mesh
/// passes through unchanged with zero removals.
#[test]
fn property_output_is_an_ordered_subset_and_clean_meshes_pass_unchanged() {
    let mut seed = 20261028u32;
    let mut random = |n: u32| {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed >> 8) % n
    };
    let all = [0, 1, 2, 3, 5, 9, 15];
    for round in 0..200 {
        let side = 2 + random(6);
        let mut positions = Vec::new();
        for y in 0..side {
            for x in 0..side {
                positions.push([x as f32, y as f32, (x * y) as f32]);
            }
        }
        let mut clean = Vec::new();
        for y in 0..side - 1 {
            for x in 0..side - 1 {
                let v = y * side + x;
                clean.extend_from_slice(&[v, v + 1, v + side, v + side, v + 1, v + side + 1]);
            }
        }
        for bits in all {
            let options = SanitizeOptions::from_bits(bits).unwrap();
            let mesh = run(&clean, &positions, options);
            assert_eq!(mesh.indices, clean, "round {round} options {bits}");
            assert_eq!(mesh.report.removed(), 0);
        }
        let n = positions.len() as u32;
        let mut noisy = clean.clone();
        for _ in 0..random(12) {
            let at = random(noisy.len() as u32 / 3) as usize * 3;
            let triangle: [u32; 3] = noisy[at..at + 3].try_into().unwrap();
            let corrupt = match random(5) {
                0 => [triangle[1], triangle[2], triangle[0]],
                1 => [triangle[0], triangle[2], triangle[1]],
                2 => [triangle[0], triangle[0], triangle[1]],
                3 => [triangle[0], n + random(3), triangle[2]],
                _ => [random(n), random(n), random(n)],
            };
            let to = random(noisy.len() as u32 / 3 + 1) as usize * 3;
            noisy.splice(to..to, corrupt);
        }
        if random(2) == 0 {
            positions[random(n) as usize][1] = f32::NAN;
        }
        for bits in all {
            let options = SanitizeOptions::from_bits(bits & !1).unwrap();
            let mesh = run(&noisy, &positions, options);
            let r = mesh.report;
            assert_eq!(r.input_triangles, noisy.len() / 3);
            assert_eq!(r.output_triangles + r.removed(), r.input_triangles);
            assert_eq!(mesh.indices.len(), r.index_count());
            let mut source = noisy.as_chunks::<3>().0.iter();
            for kept in mesh.indices.as_chunks::<3>().0 {
                assert!(
                    source.any(|t| t == kept),
                    "round {round}: not an ordered subset"
                );
            }
        }
    }
}
