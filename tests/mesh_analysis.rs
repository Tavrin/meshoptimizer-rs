use meshoptimizer_rs::*;

const CUBE_POSITIONS: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
];

/// Outward-wound closed cube; the top face (z = 1) is the second pair.
const CUBE: [u32; 36] = [
    0, 2, 1, 0, 3, 2, // bottom
    4, 5, 6, 4, 6, 7, // top
    0, 1, 5, 0, 5, 4, // front
    3, 7, 6, 3, 6, 2, // back
    0, 4, 7, 0, 7, 3, // left
    1, 2, 6, 1, 6, 5, // right
];

fn run(indices: &[u32], positions: &[[f32; 3]], options: AnalyzeOptions) -> MeshAnalysis {
    let mut ws = Workspace::default();
    analyze_mesh(indices, Positions::from_packed(positions), options, &mut ws).unwrap()
}

fn listing() -> AnalyzeOptions {
    AnalyzeOptions::EMPTY.with_list_limit(16)
}

#[test]
fn closed_cube_has_no_boundary_or_defect() {
    let a = run(&CUBE, &CUBE_POSITIONS, listing());
    assert_eq!((a.triangles, a.vertices, a.edges), (12, 8, 18));
    assert_eq!((a.boundary_edges, a.boundary_components), (0, 0));
    assert_eq!((a.non_manifold_edges, a.non_manifold_vertices), (0, 0));
    assert_eq!(a.orientation_conflicts, 0);
    assert_eq!(
        (a.components, a.largest_component, a.smallest_component),
        (1, 12, 12)
    );
    assert_eq!(a.lists, AnalysisLists::default());
}

#[test]
fn open_box_has_one_boundary_loop() {
    let open: Vec<u32> = CUBE[..6].iter().chain(&CUBE[12..]).copied().collect();
    let a = run(&open, &CUBE_POSITIONS, listing());
    assert_eq!((a.boundary_edges, a.boundary_components), (4, 1));
    assert_eq!(a.lists.boundary_edges, [[5, 4], [4, 7], [6, 5], [7, 6]]);
    assert_eq!(a.lists.boundary_components.len(), 1);
    assert_eq!(
        (
            a.lists.boundary_components[0].vertex,
            a.lists.boundary_components[0].edges
        ),
        (4, 4)
    );
    assert_eq!((a.non_manifold_vertices, a.orientation_conflicts), (0, 0));
}

#[test]
fn bowtie_vertex_has_two_fans() {
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, -1.0, 0.0],
    ];
    let a = run(&[0, 1, 2, 0, 3, 4], &positions, listing());
    assert_eq!(a.non_manifold_vertices, 1);
    assert_eq!(a.lists.non_manifold_vertices, [0]);
    assert_eq!(a.non_manifold_edges, 0);
    // Joined at the shared vertex: one component and one boundary loop.
    assert_eq!(
        (a.components, a.boundary_edges, a.boundary_components),
        (1, 6, 1)
    );
}

#[test]
fn t_shaped_edge_is_non_manifold() {
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.5, 1.0, 0.0],
        [0.5, -1.0, 0.0],
        [0.5, 0.0, 1.0],
    ];
    let a = run(&[0, 1, 2, 1, 0, 3, 0, 1, 4], &positions, listing());
    assert_eq!(a.non_manifold_edges, 1);
    assert_eq!(a.lists.non_manifold_edges, [[0, 1]]);
    assert_eq!(a.orientation_conflicts, 0);
    assert!(a.lists.orientation_conflicts.is_empty());
    assert_eq!((a.non_manifold_vertices, a.boundary_edges), (0, 6));
}

#[test]
fn flipped_face_conflicts_on_each_edge_and_lists_are_capped() {
    let mut indices = CUBE;
    indices.swap(1, 2);
    let a = run(&indices, &CUBE_POSITIONS, listing());
    assert_eq!(a.orientation_conflicts, 3);
    assert_eq!(a.lists.orientation_conflicts, [[0, 1], [0, 2], [1, 2]]);
    assert_eq!((a.boundary_edges, a.non_manifold_edges), (0, 0));
    let capped = run(
        &indices,
        &CUBE_POSITIONS,
        AnalyzeOptions::EMPTY.with_list_limit(1),
    );
    assert_eq!(capped.orientation_conflicts, 3);
    assert_eq!(capped.lists.orientation_conflicts, [[0, 1]]);
}

#[test]
fn components_are_counted_and_sized_in_triangles() {
    let positions: Vec<[f32; 3]> = (0..7).map(|i| [i as f32, (i * i) as f32, 0.0]).collect();
    let a = run(
        &[0, 1, 2, 0, 2, 3, 4, 5, 6],
        &positions,
        AnalyzeOptions::EMPTY,
    );
    assert_eq!(
        (a.components, a.largest_component, a.smallest_component),
        (2, 2, 1)
    );
    assert_eq!(a.lists, AnalysisLists::default());
}

#[test]
fn welded_positions_join_a_triangle_soup() {
    // Every corner of the cube gets its own vertex.
    let positions: Vec<[f32; 3]> = CUBE.iter().map(|&i| CUBE_POSITIONS[i as usize]).collect();
    let soup: Vec<u32> = (0..36).collect();
    let split = run(&soup, &positions, AnalyzeOptions::EMPTY);
    assert_eq!(
        (
            split.components,
            split.boundary_edges,
            split.boundary_components
        ),
        (12, 36, 12)
    );
    let welded = run(&soup, &positions, listing().with_weld_positions(true));
    assert_eq!(
        (welded.vertices, welded.edges, welded.components),
        (8, 18, 1)
    );
    assert_eq!(
        (welded.boundary_edges, welded.orientation_conflicts),
        (0, 0)
    );
    // Vertices are named by their smallest index: flip the first face.
    let mut flipped = soup.clone();
    flipped.swap(1, 2);
    let a = run(&flipped, &positions, listing().with_weld_positions(true));
    assert_eq!(a.lists.orientation_conflicts, [[0, 1], [0, 2], [1, 2]]);
}

#[test]
fn invalid_degenerate_and_duplicate_triangles_reuse_sanitize_classes() {
    let positions = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [f32::NAN, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ];
    let indices = [
        0, 1, 2, // first
        1, 2, 0, // duplicate
        0, 2, 1, // first reversed copy
        2, 1, 0, // duplicate of the reversed copy
        0, 0, 1, // repeated index
        0, 1, 9, // out of range
        0, 1, 3, // non-finite
        0, 1, 4, // coincident corners 1 and 4
    ];
    let a = run(&indices, &positions, AnalyzeOptions::EMPTY);
    assert_eq!(a.input_triangles, 8);
    assert_eq!((a.out_of_range, a.non_finite, a.repeated_index), (1, 1, 1));
    assert_eq!((a.coincident_corners, a.triangles), (1, 5));
    assert_eq!((a.duplicate, a.reversed_duplicate), (2, 1));
    let welded = run(
        &indices,
        &positions,
        AnalyzeOptions::EMPTY.with_weld_positions(true),
    );
    assert_eq!((welded.coincident_corners, welded.triangles), (1, 4));
    assert_eq!((welded.duplicate, welded.reversed_duplicate), (2, 1));
    let mut ws = Workspace::default();
    let result = analyze_mesh(
        &indices[..4],
        Positions::from_packed(&positions),
        AnalyzeOptions::EMPTY,
        &mut ws,
    );
    assert_eq!(result, Err(Error::InvalidTopology));
}

/// `n` faces on one edge make every pairwise scheme quadratic. The charges in
/// `analyze_mesh` sum to at most `n * (61 + 42 * s)` for any `n` triangles,
/// with `s = floor(log2(3n)) + 2` steps per sort element or union-find lookup;
/// both topologies stay within `n * (64 + 42 * s)`. A tiny work limit fails
/// cleanly.
#[test]
fn faces_sharing_one_edge_stay_within_the_n_log_n_work_bound() {
    let n = 1usize << 16;
    let positions: Vec<[f32; 3]> = (0..n + 2)
        .map(|i| [i as f32, (i % 7) as f32, 1.0])
        .collect();
    let mut indices = Vec::with_capacity(n * 3);
    for k in 2..n as u32 + 2 {
        if k % 2 == 0 {
            indices.extend_from_slice(&[0, 1, k]);
        } else {
            indices.extend_from_slice(&[1, 0, k]);
        }
    }
    let s = (usize::BITS - (3 * n).leading_zeros()) as u64 + 1;
    let bound = n as u64 * (64 + 42 * s);
    for weld in [false, true] {
        let mut ws = Workspace::default();
        let options = listing().with_weld_positions(weld);
        let a = analyze_mesh(
            &indices,
            Positions::from_packed(&positions),
            options,
            &mut ws,
        )
        .unwrap();
        assert_eq!(
            (a.edges, a.non_manifold_edges, a.boundary_edges),
            (2 * n + 1, 1, 2 * n)
        );
        assert_eq!(
            (a.components, a.boundary_components, a.non_manifold_vertices),
            (1, 1, 0)
        );
        assert_eq!(a.lists.boundary_edges.len(), 16);
        assert!(
            ws.usage().work <= bound,
            "work {} > {bound}",
            ws.usage().work
        );
    }
    let mut ws = Workspace::new(Limits {
        max_work: 1000,
        ..Limits::default()
    });
    let result = analyze_mesh(
        &indices,
        Positions::from_packed(&positions),
        AnalyzeOptions::EMPTY,
        &mut ws,
    );
    assert_eq!(result, Err(Error::LimitExceeded));
}

#[test]
fn welded_names_use_only_referenced_vertices() {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 0.]];
    let a = run(&[3, 1, 2], &positions, listing().with_weld_positions(true));
    assert_eq!(a.lists.boundary_edges, [[1, 2], [3, 1], [2, 3]]);
    assert_eq!(a.lists.boundary_components[0].vertex, 1);
}

#[test]
fn invalid_input_passes_are_charged_in_both_modes() {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let mut indices = vec![0, 1, 2];
    for _ in 0..1000 {
        indices.extend_from_slice(&[9, 9, 9]);
    }
    for weld in [false, true] {
        let options = AnalyzeOptions::EMPTY.with_weld_positions(weld);
        let mut ws = Workspace::default();
        analyze_mesh(
            &indices,
            Positions::from_packed(&positions),
            options,
            &mut ws,
        )
        .unwrap();
        assert!(ws.usage().work >= 2 * 1001);
        let mut limited = Workspace::new(Limits {
            max_work: 1002,
            ..Limits::default()
        });
        assert_eq!(
            analyze_mesh(
                &indices,
                Positions::from_packed(&positions),
                options,
                &mut limited
            ),
            Err(Error::LimitExceeded)
        );
    }
}

#[test]
fn all_invalid_needs_no_triangle_scratch() {
    let indices = vec![9; 3_000_000];
    let mut ws = Workspace::new(Limits {
        max_bytes: 64,
        ..Limits::default()
    });
    let a = analyze_mesh(
        &indices,
        Positions::from_packed(&[]),
        AnalyzeOptions::EMPTY,
        &mut ws,
    )
    .unwrap();
    assert_eq!(
        (a.input_triangles, a.out_of_range, a.triangles),
        (1_000_000, 1_000_000, 0)
    );
}

#[test]
fn duplicate_windings_differ_from_orientation_ignoring_sanitize() {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let indices = [0, 1, 2, 0, 2, 1, 0, 2, 1];
    let a = run(&indices, &positions, AnalyzeOptions::EMPTY);
    assert_eq!((a.duplicate, a.reversed_duplicate), (1, 1));
    let mut ws = Workspace::default();
    let sanitized = sanitize(
        &indices,
        Positions::from_packed(&positions),
        SanitizeOptions::IGNORE_ORIENTATION,
        &mut ws,
    )
    .unwrap();
    assert_eq!(
        (
            sanitized.report.duplicate,
            sanitized.report.reversed_duplicate
        ),
        (0, 2)
    );
}
