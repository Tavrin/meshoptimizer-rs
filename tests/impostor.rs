//! Octahedral impostor baking. Generated test data is CC0-1.0.
use meshoptimizer_rs::*;

const QUAD: [[f32; 3]; 4] = [
    [-1.0, -1.0, 0.0],
    [1.0, -1.0, 0.0],
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
];
const QUAD_UV: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
const QUAD_I: [u32; 6] = [0, 1, 2, 0, 2, 3];
/// In a 3 x 3 sphere grid, view 7 looks from +Z and view 1 from -Z.
const FRONT: u32 = 7;
const BACK: u32 = 1;

fn options(n: u32, res: u32) -> ImpostorOptions {
    let mut o = ImpostorOptions::default();
    o.grid = ImpostorGrid::new(ImpostorLayout::Sphere, n).unwrap();
    o.view_resolution = res;
    o
}

/// Quad options framing exactly `[-1, 1]^2` so samples map to known UVs.
fn quad_options(res: u32) -> ImpostorOptions {
    let mut o = options(3, res);
    o.bounds = Some(ImpostorBounds::new([0.0; 3], 1.0).unwrap());
    o.dilation = 0;
    o
}

fn bake(
    indices: &[u32],
    p: &[[f32; 3]],
    albedo: ImpostorAlbedo<'_>,
    o: ImpostorOptions,
) -> ImpostorAtlas {
    bake_impostor(
        indices,
        Positions::from_packed(p),
        None,
        albedo,
        o,
        &mut Workspace::default(),
    )
    .unwrap()
}

fn tile(atlas: &ImpostorAtlas, view: u32) -> Vec<([u8; 4], [u8; 4])> {
    let [x0, y0] = atlas.tile_origin(view).unwrap();
    let r = atlas.view_resolution;
    let mut out = vec![];
    for y in y0..y0 + r {
        for x in x0..x0 + r {
            let at = (y * atlas.size + x) as usize;
            out.push((atlas.albedo_alpha[at], atlas.normal_depth[at]));
        }
    }
    out
}

fn close(a: [f32; 3], b: [f32; 3], tol: f32) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() <= tol)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32) / (1u64 << 24) as f32
    }
}

#[test]
fn octahedral_mapping_round_trips() {
    let mut rng = Rng(7);
    for layout in [ImpostorLayout::Hemisphere, ImpostorLayout::Sphere] {
        for _ in 0..2000 {
            let mut d = [
                rng.next() * 2.0 - 1.0,
                rng.next() * 2.0 - 1.0,
                rng.next() * 2.0 - 1.0,
            ];
            if layout == ImpostorLayout::Hemisphere {
                d[1] = d[1].abs();
            }
            let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let d = d.map(|x| x / l);
            let o = layout.encode(d).unwrap();
            assert!(o.iter().all(|x| x.abs() <= 1.0));
            assert!(
                close(layout.decode(o).unwrap(), d, 1e-6),
                "{layout:?} {d:?}"
            );
        }
        // Points of the square decode to unit directions that encode back.
        for j in 0..=20 {
            for i in 0..=20 {
                let o = [i as f32 / 10.0 - 1.0, j as f32 / 10.0 - 1.0];
                let d = layout.decode(o).unwrap();
                assert!(((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() - 1.0).abs() < 1e-6);
                let back = layout.decode(layout.encode(d).unwrap()).unwrap();
                assert!(close(back, d, 1e-6));
            }
        }
        assert_eq!(layout.decode([1.5, 0.0]), Err(Error::InvalidParameter));
        assert_eq!(layout.encode([0.0; 3]), Err(Error::InvalidParameter));
        assert_eq!(
            layout.encode([f32::NAN, 1.0, 0.0]),
            Err(Error::InvalidParameter)
        );
    }
    let h = ImpostorLayout::Hemisphere;
    assert_eq!(h.encode([0.0, 1.0, 0.0]).unwrap(), [0.0, 0.0]);
    assert_eq!(h.encode([1.0, 0.0, 0.0]).unwrap(), [1.0, 1.0]);
    assert_eq!(h.encode([0.0, 0.0, 1.0]).unwrap(), [1.0, -1.0]);
    // Below the horizon projects onto it.
    assert_eq!(h.encode([1.0, -0.5, 0.0]).unwrap(), [1.0, 1.0]);
    let s = ImpostorLayout::Sphere;
    assert_eq!(s.encode([0.0, -1.0, 0.0]).unwrap(), [1.0, 1.0]);
    assert_eq!(s.encode([0.0, 0.0, -1.0]).unwrap(), [0.0, -1.0]);
}

#[test]
fn views_blend_to_themselves_and_frames_are_orthonormal() {
    for layout in [ImpostorLayout::Hemisphere, ImpostorLayout::Sphere] {
        for n in [2, 3, 8, 12] {
            let grid = ImpostorGrid::new(layout, n).unwrap();
            let frames: Vec<_> = (0..n * n).map(|v| grid.view_frame(v).unwrap()).collect();
            for (v, f) in frames.iter().enumerate() {
                let (d, r, u) = (f.direction, f.right, f.up);
                let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
                assert!((dot(d, d) - 1.0).abs() < 1e-6 && (dot(r, r) - 1.0).abs() < 1e-6);
                assert!(dot(d, r).abs() < 1e-6 && dot(d, u).abs() < 1e-6 && dot(r, u).abs() < 1e-6);
                let rxu = [
                    r[1] * u[2] - r[2] * u[1],
                    r[2] * u[0] - r[0] * u[2],
                    r[0] * u[1] - r[1] * u[0],
                ];
                assert!(close(rxu, d, 1e-6));
                let g = ImpostorFrame::from_direction(d).unwrap();
                assert!(
                    close(g.direction, d, 1e-6) && close(g.right, r, 1e-6) && close(g.up, u, 1e-6)
                );
                assert!(close(
                    layout.decode(grid.view_point(v as u32).unwrap()).unwrap(),
                    d,
                    1e-6
                ));
                // The view's own direction, summed over duplicate frames.
                let b = grid.blend(d).unwrap();
                let w: f32 = (0..3)
                    .filter(|&k| close(frames[b.views[k] as usize].direction, d, 1e-6))
                    .map(|k| b.weights[k])
                    .sum();
                assert!(w > 1.0 - 1e-5, "{layout:?} n={n} view {v}: {b:?}");
            }
            let mut rng = Rng(u64::from(n));
            for _ in 0..500 {
                let d = [rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5];
                let b = grid.blend(d).unwrap();
                assert!(b.weights.iter().all(|&w| w >= 0.0));
                assert!((b.weights.iter().sum::<f32>() - 1.0).abs() < 1e-6);
                assert!(b.views.iter().all(|&v| v < n * n));
            }
        }
    }
    assert_eq!(
        ImpostorGrid::new(ImpostorLayout::Sphere, 1),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        ImpostorGrid::new(ImpostorLayout::Sphere, 65),
        Err(Error::InvalidParameter)
    );
    let grid = ImpostorGrid::default();
    assert_eq!(grid.view_frame(64), Err(Error::InvalidParameter));
    // Poles use +X as right.
    let top = ImpostorFrame::from_direction([0.0, 2.0, 0.0]).unwrap();
    assert_eq!((top.right, top.up), ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0]));
}

#[test]
fn quad_tiles_store_colour_normal_depth_and_facing() {
    let colors = [[1.0, 0.0, 0.0, 1.0]; 4];
    let normals = [[0.0, 0.0, 1.0]; 4];
    let mut o = quad_options(8);
    let atlas = bake_impostor(
        &QUAD_I,
        Positions::from_packed(&QUAD),
        Some(&normals),
        ImpostorAlbedo::VertexColors(&colors),
        o,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(atlas.bounds, ImpostorBounds::new([0.0; 3], 1.0).unwrap());
    // Front view: the quad fills the tile, normal +Z, depth at the center plane.
    let front = tile(&atlas, FRONT);
    assert!(front
        .iter()
        .all(|&t| t == ([255, 0, 0, 255], [128, 128, 255, 128])));
    // Back view: double-sided faces show the reversed normal.
    let back = tile(&atlas, BACK);
    assert!(back
        .iter()
        .all(|&t| t == ([255, 0, 0, 255], [128, 128, 0, 128])));
    // Edge-on from above is empty and keeps the empty encoding.
    assert!(tile(&atlas, 4)
        .iter()
        .all(|&t| t == ([0; 4], [128, 128, 128, 255])));
    o.double_sided = false;
    let atlas = bake(&QUAD_I, &QUAD, ImpostorAlbedo::White, o);
    assert!(tile(&atlas, BACK).iter().all(|&(c, _)| c[3] == 0));
    assert!(tile(&atlas, FRONT)
        .iter()
        .all(|&(c, nd)| c == [255; 4] && nd == [128, 128, 255, 128]));
}

#[test]
fn shared_edges_leave_no_gaps() {
    // Plain coverage alpha: any missed sample on the diagonal lowers alpha.
    let mut o = quad_options(32);
    o.coverage_threshold = 1.0;
    o.samples_per_axis = 3;
    let atlas = bake(&QUAD_I, &QUAD, ImpostorAlbedo::White, o);
    assert!(tile(&atlas, FRONT).iter().all(|&(c, _)| c[3] == 255));
    // A fan of eight triangles around the center, framed by the unit bounds
    // inside its inradius (2 cos 22.5 deg = 1.85 > sqrt 2).
    let mut p = vec![[0.0f32, 0.0, 0.0]];
    for k in 0..8 {
        let a = k as f32 * core::f32::consts::FRAC_PI_4;
        p.push([2.0 * a.cos(), 2.0 * a.sin(), 0.0]);
    }
    let i: Vec<u32> = (0..8).flat_map(|k| [0, 1 + k, 1 + (k + 1) % 8]).collect();
    let atlas = bake(&i, &p, ImpostorAlbedo::White, o);
    assert!(tile(&atlas, FRONT).iter().all(|&(c, _)| c[3] == 255));
}

#[test]
fn alpha_cutoff_and_conservative_coverage() {
    let res = 8u32;
    let side = (res * 2) as f32;
    // One of four samples per texel passes the alpha test.
    let one_in_four = |_: usize, uv: [f32; 2]| {
        let (x, y) = ((uv[0] * side).floor() as i32, (uv[1] * side).floor() as i32);
        [
            0.2,
            0.4,
            0.6,
            if x % 2 == 0 && y % 2 == 0 { 1.0 } else { 0.0 },
        ]
    };
    let albedo = ImpostorAlbedo::Texture {
        uvs: &QUAD_UV,
        sampler: &one_in_four,
    };
    let mut o = quad_options(res);
    for (threshold, alpha) in [(0.5, 128), (0.25, 255), (1.0, 64)] {
        o.coverage_threshold = threshold;
        let atlas = bake(&QUAD_I, &QUAD, albedo, o);
        let t = tile(&atlas, FRONT);
        assert!(
            t.iter().all(|&(c, _)| c == [51, 102, 153, alpha]),
            "{threshold}: {:?}",
            t[0]
        );
    }
    // Checkerboard samples: half coverage.
    let checker = |_: usize, uv: [f32; 2]| {
        let (x, y) = ((uv[0] * side).floor() as i32, (uv[1] * side).floor() as i32);
        [1.0, 1.0, 1.0, ((x + y) % 2) as f32]
    };
    o.coverage_threshold = 0.5;
    let atlas = bake(
        &QUAD_I,
        &QUAD,
        ImpostorAlbedo::Texture {
            uvs: &QUAD_UV,
            sampler: &checker,
        },
        o,
    );
    assert!(tile(&atlas, FRONT).iter().all(|&(c, _)| c[3] == 255));
    // Alpha below the cutoff is discarded; at the cutoff it passes.
    let faint = |_: usize, _: [f32; 2]| [1.0, 1.0, 1.0, 0.4];
    let albedo = ImpostorAlbedo::Texture {
        uvs: &QUAD_UV,
        sampler: &faint,
    };
    let atlas = bake(&QUAD_I, &QUAD, albedo, o);
    assert_eq!(
        (atlas.report.covered_texels, atlas.report.empty_views),
        (0, 9)
    );
    assert!(atlas.report.sampler_calls > 0);
    o.alpha_cutoff = 0.4;
    let atlas = bake(&QUAD_I, &QUAD, albedo, o);
    assert!(atlas.report.covered_texels >= u64::from(res * res));
    // A discarded front surface does not hide the one behind it.
    let mut p = QUAD.to_vec();
    p.extend(QUAD.map(|v| [v[0], v[1], -0.5]));
    let uvs: Vec<[f32; 2]> = QUAD_UV.iter().chain(&QUAD_UV).copied().collect();
    let i = [0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7];
    let near_hidden = |t: usize, _: [f32; 2]| {
        if t < 2 {
            [1.0, 0.0, 0.0, 0.0]
        } else {
            [0.0, 0.0, 1.0, 1.0]
        }
    };
    let atlas = bake(
        &i,
        &p,
        ImpostorAlbedo::Texture {
            uvs: &uvs,
            sampler: &near_hidden,
        },
        quad_options(res),
    );
    assert!(tile(&atlas, FRONT)
        .iter()
        .all(|&(c, nd)| c == [0, 0, 255, 255] && nd[3] == 191));
}

#[test]
fn dilation_fills_empty_texels_without_alpha() {
    // A small quad in the middle of the tile.
    let small = QUAD.map(|v| [v[0] * 0.25, v[1] * 0.25, 0.0]);
    let colors = [[0.0, 1.0, 0.0, 1.0]; 4];
    let mut o = quad_options(16);
    let plain = bake(&QUAD_I, &small, ImpostorAlbedo::VertexColors(&colors), o);
    o.dilation = 2;
    let dilated = bake(&QUAD_I, &small, ImpostorAlbedo::VertexColors(&colors), o);
    let (a, b) = (tile(&plain, FRONT), tile(&dilated, FRONT));
    let covered = a.iter().filter(|t| t.0[3] > 0).count();
    assert_eq!(covered, b.iter().filter(|t| t.0[3] > 0).count());
    let green = b.iter().filter(|t| t.0 == [0, 255, 0, 0]).count();
    // The 4 x 4 block grows by two rings, minus the corners a 4-neighbour
    // fill cannot reach in two passes: 8 x 8 - 16 - 4 * 3.
    assert_eq!((covered, green), (16, 64 - 16 - 12));
    assert!(b
        .iter()
        .all(|t| t.0[3] == 255 || t.0 == [0, 255, 0, 0] || t.0 == [0; 4]));
}

#[test]
fn empty_and_degenerate_meshes_give_empty_atlases() {
    let o = options(4, 8);
    let atlas = bake(&[], &QUAD, ImpostorAlbedo::White, o);
    assert_eq!(
        (atlas.size, atlas.bounds.radius, atlas.report.empty_views),
        (32, 0.0, 16)
    );
    assert!(atlas.albedo_alpha.iter().all(|&c| c == [0; 4]));
    // Collapsed to a point, and collinear.
    let point = [[1.0, 2.0, 3.0]; 3];
    let atlas = bake(&[0, 1, 2], &point, ImpostorAlbedo::White, o);
    assert_eq!((atlas.bounds.radius, atlas.report.covered_texels), (0.0, 0));
    let line = [[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]];
    let atlas = bake(&[0, 1, 2, 2, 1, 0], &line, ImpostorAlbedo::White, o);
    assert_eq!(
        (atlas.report.covered_texels, atlas.report.empty_views),
        (0, 16)
    );
    // Unreferenced vertices are not read or bounded.
    let mut p = QUAD.to_vec();
    p.push([f32::NAN, 1e30, 0.0]);
    let atlas = bake(&QUAD_I, &p, ImpostorAlbedo::White, o);
    assert_eq!(atlas.bounds.center, [0.0; 3]);
}

fn run(
    i: &[u32],
    n: Option<&[[f32; 3]]>,
    a: ImpostorAlbedo<'_>,
    o: ImpostorOptions,
) -> Result<(), Error> {
    bake_impostor(
        i,
        Positions::from_packed(&QUAD),
        n,
        a,
        o,
        &mut Workspace::default(),
    )
    .map(|_| ())
}

#[test]
fn invalid_inputs_are_rejected() {
    let good = ImpostorOptions::default();
    let white = ImpostorAlbedo::White;
    assert_eq!(run(&[0, 1], None, white, good), Err(Error::InvalidTopology));
    assert_eq!(
        run(&[0, 1, 4], None, white, good),
        Err(Error::IndexOutOfBounds)
    );
    assert_eq!(
        run(&QUAD_I, Some(&[[0.0; 3]; 3]), white, good),
        Err(Error::InvalidLayout)
    );
    let short = [[1.0; 4]; 3];
    assert_eq!(
        run(&QUAD_I, None, ImpostorAlbedo::VertexColors(&short), good),
        Err(Error::InvalidLayout)
    );
    let nan = [[0.0, 0.0, f32::NAN]; 4];
    assert_eq!(
        run(&QUAD_I, Some(&nan), white, good),
        Err(Error::NumericalFailure)
    );
    let bad_sampler = |_: usize, _: [f32; 2]| [f32::INFINITY, 0.0, 0.0, 1.0];
    let albedo = ImpostorAlbedo::Texture {
        uvs: &QUAD_UV,
        sampler: &bad_sampler,
    };
    assert_eq!(
        run(&QUAD_I, None, albedo, good),
        Err(Error::NumericalFailure)
    );
    let edits: [fn(&mut ImpostorOptions); 9] = [
        |o: &mut ImpostorOptions| o.view_resolution = 0,
        |o: &mut ImpostorOptions| o.view_resolution = 4097,
        |o: &mut ImpostorOptions| o.samples_per_axis = 9,
        |o: &mut ImpostorOptions| o.alpha_cutoff = 1.5,
        |o: &mut ImpostorOptions| o.coverage_threshold = 0.0,
        |o: &mut ImpostorOptions| o.coverage_threshold = f32::NAN,
        |o: &mut ImpostorOptions| o.dilation = 65,
        |o: &mut ImpostorOptions| o.grid.views_per_axis = 1,
        |o: &mut ImpostorOptions| o.bounds = Some(ImpostorBounds::default()),
    ];
    for edit in edits {
        let mut o = good;
        edit(&mut o);
        assert_eq!(run(&QUAD_I, None, white, o), Err(Error::InvalidParameter));
    }
    // Fixed bounds far smaller than the mesh.
    let mut o = good;
    o.bounds = Some(ImpostorBounds::new([0.0; 3], 1e-6).unwrap());
    assert_eq!(run(&QUAD_I, None, white, o), Err(Error::InvalidParameter));
    assert_eq!(
        ImpostorBounds::new([0.0; 3], -1.0),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        ImpostorBounds::from_box([1.0; 3], [0.0; 3]),
        Err(Error::InvalidParameter)
    );
    let b = ImpostorBounds::from_box([-1.0, -2.0, -2.0], [1.0, 2.0, 2.0]).unwrap();
    assert_eq!((b.center, b.radius), ([0.0; 3], 3.0));
}

#[test]
fn memory_and_work_limits_are_enforced() {
    let o = options(8, 64);
    let mut ws = Workspace::default();
    let pos = Positions::from_packed(&QUAD);
    bake_impostor(&QUAD_I, pos, None, ImpostorAlbedo::White, o, &mut ws).unwrap();
    let usage = ws.usage();
    // Two 512 x 512 RGBA8 planes plus one view of 128 x 128 samples.
    assert!(
        usage.bytes >= 2 * 512 * 512 * 4 + 128 * 128 * 32,
        "{usage:?}"
    );
    assert!(usage.work >= 64 * 128 * 128);
    let limited = |limits: Limits| {
        let mut ws = Workspace::new(limits);
        bake_impostor(&QUAD_I, pos, None, ImpostorAlbedo::White, o, &mut ws).map(|_| ())
    };
    assert_eq!(
        limited(Limits {
            max_bytes: usage.bytes,
            max_work: usage.work
        }),
        Ok(())
    );
    assert_eq!(
        limited(Limits {
            max_bytes: usage.bytes - 1,
            ..Limits::default()
        }),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        limited(Limits {
            max_work: usage.work - 1,
            ..Limits::default()
        }),
        Err(Error::LimitExceeded)
    );
    // The largest atlas is refused by the default 1 GiB limit, not allocated.
    let mut huge = options(64, 4096);
    huge.samples_per_axis = 8;
    let mut ws = Workspace::default();
    assert_eq!(
        bake_impostor(&QUAD_I, pos, None, ImpostorAlbedo::White, huge, &mut ws).map(|_| ()),
        Err(Error::LimitExceeded)
    );
}

/// A deterministic shrub: random leaf triangles around a sphere, with
/// normals and vertex colours.
type Mesh = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<u32>);

fn shrub() -> Mesh {
    let mut rng = Rng(42);
    let (mut p, mut n, mut c, mut i) = (vec![], vec![], vec![], vec![]);
    for t in 0..300u32 {
        let center = [
            rng.next() * 2.0 - 1.0,
            rng.next() * 1.5,
            rng.next() * 2.0 - 1.0,
        ];
        let normal = {
            let v = [rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5];
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
            v.map(|x| x / l)
        };
        for _ in 0..3 {
            p.push([0, 1, 2].map(|k| center[k] + (rng.next() - 0.5) * 0.4));
            n.push(normal);
            c.push([rng.next(), rng.next(), rng.next(), 1.0]);
        }
        i.extend([t * 3, t * 3 + 1, t * 3 + 2]);
    }
    (p, n, c, i)
}

fn fnv(atlas: &ImpostorAtlas) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in atlas
        .albedo_alpha
        .iter()
        .chain(&atlas.normal_depth)
        .flatten()
    {
        h = (h ^ u64::from(*b)).wrapping_mul(0x100000001b3);
    }
    h
}

#[test]
fn bakes_are_deterministic_across_runs_and_index_widths() {
    let (p, n, c, i) = shrub();
    let mut o = ImpostorOptions::default();
    o.view_resolution = 32;
    let run = |o: ImpostorOptions| {
        bake_impostor(
            &i,
            Positions::from_packed(&p),
            Some(&n),
            ImpostorAlbedo::VertexColors(&c),
            o,
            &mut Workspace::default(),
        )
        .unwrap()
    };
    let a = run(o);
    assert_eq!(a, run(o));
    let narrow: Vec<u16> = i.iter().map(|&x| x as u16).collect();
    let b = bake_impostor_generic(
        &narrow,
        Positions::from_packed(&p),
        Some(&n),
        ImpostorAlbedo::VertexColors(&c),
        o,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(a, b);
    assert_eq!(a.report.empty_views, 0);
    o.grid = ImpostorGrid::new(ImpostorLayout::Sphere, 5).unwrap();
    o.samples_per_axis = 3;
    let s = run(o);
    // Pinned bytes: the result is the same on every platform.
    assert_eq!(
        (
            fnv(&a),
            a.report.covered_texels,
            fnv(&s),
            s.report.covered_texels
        ),
        (13760548025172913131, 20332, 12514472493179137328, 8919)
    );
}

#[test]
fn switch_distance_combines_texel_and_view_error() {
    let camera = LodCamera::new(1.0, 1080.0, 1.0).unwrap();
    let bounds = ImpostorBounds::new([0.0; 3], 2.0).unwrap();
    let h = |n| ImpostorGrid::new(ImpostorLayout::Hemisphere, n).unwrap();
    let a = impostor_switch_distance(bounds, h(8), 128, camera).unwrap();
    assert_eq!(a.texel_error, 4.0 / 128.0);
    let worst = a.texel_error.max(a.view_error);
    assert_eq!(a.distance, camera.switch_distance(worst).unwrap() + 2.0);
    // More views shrink the view error; more texels shrink the texel error.
    let b = impostor_switch_distance(bounds, h(12), 256, camera).unwrap();
    assert!(
        b.view_error < a.view_error && b.texel_error < a.texel_error && b.distance < a.distance
    );
    // The bound covers the weighted chord at every cell center (a lattice point).
    let grid = h(8);
    let mut seen = 0.0f64;
    for j in 0..7 {
        for i in 0..7 {
            let o = [
                (2 * i + 1) as f32 / 7.0 - 1.0,
                (2 * j + 1) as f32 / 7.0 - 1.0,
            ];
            let d = grid.layout.decode(o).unwrap();
            let b = grid.blend(d).unwrap();
            let chord: f64 = (0..3)
                .map(|k| {
                    let f = grid.view_frame(b.views[k]).unwrap().direction;
                    let c = (0..3)
                        .map(|c| f64::from(d[c] - f[c]).powi(2))
                        .sum::<f64>()
                        .sqrt();
                    f64::from(b.weights[k]) * c
                })
                .sum();
            seen = seen.max(chord * 2.0);
        }
    }
    assert!(
        seen > 0.0 && seen <= a.view_error + 1e-5,
        "{seen} {}",
        a.view_error
    );
    assert_eq!(
        impostor_switch_distance(ImpostorBounds::default(), grid, 128, camera),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        impostor_switch_distance(bounds, grid, 0, camera),
        Err(Error::InvalidParameter)
    );
}
