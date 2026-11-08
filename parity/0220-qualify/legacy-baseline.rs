use baseline::*;

pub const CASES: &[&str] = &[
    "bake-vertex-colors-hemisphere-u32",
    "bake-texture-sphere-u16",
    "bake-fixed-bounds-single-sided-box",
    "bake-white-dense-grid",
    "impostor-math-and-reconstruct",
    "trim-default-u32",
    "trim-repeat-padding0-cost-stride8",
    "trim-clamp-fan-u16",
];

struct Hasher(u64);
impl Hasher {
    fn new() -> Self {
        Self(0xcbf29ce484222325)
    }
    fn bytes(&mut self, b: &[u8]) {
        for &x in b {
            self.0 = (self.0 ^ u64::from(x)).wrapping_mul(0x100000001b3);
        }
    }
    fn u64(&mut self, v: u64) {
        self.bytes(&v.to_le_bytes());
    }
    fn f32s(&mut self, v: &[f32]) {
        for x in v {
            self.bytes(&x.to_bits().to_le_bytes());
        }
    }
    fn f64(&mut self, v: f64) {
        self.bytes(&v.to_bits().to_le_bytes());
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545F4914F6CDD1D) >> 32) as u32
    }
    /// Uniform in [0, 1), exact in f32.
    fn unit(&mut self) -> f32 {
        (self.next() >> 8) as f32 / 16777216.0
    }
    fn signed(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit_vec(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / l, v[1] / l, v[2] / l]
}

struct Cards {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

/// Randomly oriented quad cards inside the unit cube, the shape of a plant billboard cloud.
fn cards(seed: u64, count: usize) -> Cards {
    let mut r = Rng(seed);
    let mut c = Cards {
        positions: vec![],
        normals: vec![],
        uvs: vec![],
        colors: vec![],
        indices: vec![],
    };
    for _ in 0..count {
        let center = [r.signed(), r.signed() * 0.8 + 0.8, r.signed()];
        let u = [r.signed() * 0.4, r.signed() * 0.4, r.signed() * 0.4];
        let v = [
            r.signed() * 0.4 + 0.1,
            r.signed() * 0.4 + 0.5,
            r.signed() * 0.4,
        ];
        let n = unit_vec(cross(u, v));
        let base = c.positions.len() as u32;
        for (su, sv, uv) in [
            (-1.0f32, -1.0f32, [0.0f32, 0.0f32]),
            (1.0, -1.0, [1.0, 0.0]),
            (1.0, 1.0, [1.0, 1.0]),
            (-1.0, 1.0, [0.0, 1.0]),
        ] {
            c.positions
                .push([0, 1, 2].map(|k| center[k] + su * u[k] + sv * v[k]));
            c.normals.push(n);
            c.uvs.push(uv);
            c.colors.push([
                r.unit(),
                r.unit(),
                r.unit(),
                if r.next() % 5 == 0 { 0.25 } else { 1.0 },
            ]);
        }
        c.indices
            .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    c
}

fn leaf_sampler(triangle: usize, uv: [f32; 2]) -> [f32; 4] {
    let (a, b) = (uv[0] - 0.5, uv[1] - 0.5);
    let alpha = if a * a * 2.0 + b * b * 0.5 < 0.1 && (triangle & 7) != 3 {
        1.0
    } else {
        0.0
    };
    [uv[0], uv[1], (triangle % 17) as f32 / 16.0, alpha]
}

fn hash_atlas(h: &mut Hasher, a: &ImpostorAtlas) {
    h.u64(u64::from(a.size));
    h.u64(u64::from(a.view_resolution));
    h.f32s(&[a.bounds.radius]);
    h.f32s(&a.bounds.center);
    for t in a.albedo_alpha.iter().chain(a.normal_depth.iter()) {
        h.bytes(t);
    }
    h.u64(a.report.covered_texels);
    h.u64(a.report.opaque_texels);
    h.u64(u64::from(a.report.empty_views));
    h.u64(a.report.sampler_calls);
}

fn bake_options(layout: ImpostorLayout, n: u32, res: u32) -> ImpostorOptions {
    let mut o = ImpostorOptions::default();
    o.grid = ImpostorGrid::new(layout, n).expect("grid");
    o.view_resolution = res;
    o
}

fn hash_trimmed(
    h: &mut Hasher,
    rep: &CardTrimReport,
    vertices: &[f32],
    indices: &[u32],
    card_vertices: &[u32],
) {
    for v in [
        rep.input_triangles,
        rep.output_triangles,
        rep.output_vertices,
        rep.cards,
        rep.trimmed_cards,
        rep.split_cards,
        rep.removed_cards,
        rep.unchanged_cards,
        rep.passthrough_triangles,
    ] {
        h.u64(v as u64);
    }
    h.f64(rep.area_before);
    h.f64(rep.area_after);
    h.u64(rep.opaque_texels);
    h.f32s(vertices);
    for i in indices {
        h.bytes(&i.to_le_bytes());
    }
    for v in card_vertices {
        h.bytes(&v.to_le_bytes());
    }
}

/// An 8-bit alpha plane of blobs with soft edges, integer arithmetic only.
fn mask(size: u32, seed: u64) -> Vec<u8> {
    let mut r = Rng(seed);
    let blobs: Vec<(i64, i64, i64, i64)> = (0..6)
        .map(|_| {
            (
                i64::from(r.next() % size),
                i64::from(r.next() % size),
                3 + i64::from(r.next() % 4),
                20 + i64::from(r.next() % 40),
            )
        })
        .collect();
    let mut out = vec![0u8; (size * size) as usize];
    for y in 0..i64::from(size) {
        for x in 0..i64::from(size) {
            let mut best = 0i64;
            for &(cx, cy, k, rad) in &blobs {
                let d2 = (x - cx) * (x - cx) * k + (y - cy) * (y - cy);
                best = best.max(255 - (d2 * 255 / (rad * rad)).min(255));
            }
            out[(y * i64::from(size) + x) as usize] = best as u8;
        }
    }
    out
}

fn case_bake_vertex_colors() -> u64 {
    let c = cards(0x0218_0001, 40);
    let o = bake_options(ImpostorLayout::Hemisphere, 4, 32);
    let a = bake_impostor(
        &c.indices,
        Positions::from_packed(&c.positions),
        Some(&c.normals),
        ImpostorAlbedo::VertexColors(&c.colors),
        o,
        &mut Workspace::default(),
    )
    .expect("bake");
    let mut h = Hasher::new();
    hash_atlas(&mut h, &a);
    h.0
}

fn case_bake_texture_u16() -> u64 {
    let c = cards(0x0218_0002, 30);
    let indices: Vec<u16> = c.indices.iter().map(|&i| i as u16).collect();
    let mut o = bake_options(ImpostorLayout::Sphere, 5, 24);
    o.samples_per_axis = 3;
    o.dilation = 6;
    o.coverage_threshold = 0.75;
    let sampler = |t: usize, uv: [f32; 2]| leaf_sampler(t, uv);
    let a = bake_impostor_generic(
        &indices,
        Positions::from_packed(&c.positions),
        Some(&c.normals),
        ImpostorAlbedo::Texture {
            uvs: &c.uvs,
            sampler: &sampler,
        },
        o,
        &mut Workspace::default(),
    )
    .expect("bake");
    let mut h = Hasher::new();
    hash_atlas(&mut h, &a);
    h.0
}

fn case_bake_fixed_bounds() -> u64 {
    let c = cards(0x0218_0003, 25);
    let mut o = bake_options(ImpostorLayout::Hemisphere, 3, 40);
    o.bounds = Some(ImpostorBounds::from_box([-2.0, -0.5, -2.0], [2.0, 2.5, 2.0]).expect("box"));
    o.double_sided = false;
    o.alpha_cutoff = 0.6;
    o.dilation = 0;
    o.samples_per_axis = 1;
    let a = bake_impostor(
        &c.indices,
        Positions::from_packed(&c.positions),
        Some(&c.normals),
        ImpostorAlbedo::VertexColors(&c.colors),
        o,
        &mut Workspace::default(),
    )
    .expect("bake");
    let mut h = Hasher::new();
    hash_atlas(&mut h, &a);
    h.0
}

fn case_bake_white_dense() -> u64 {
    let c = cards(0x0218_0004, 400);
    let o = bake_options(ImpostorLayout::Hemisphere, 8, 16);
    let a = bake_impostor(
        &c.indices,
        Positions::from_packed(&c.positions),
        None,
        ImpostorAlbedo::White,
        o,
        &mut Workspace::default(),
    )
    .expect("bake");
    let mut h = Hasher::new();
    hash_atlas(&mut h, &a);
    h.0
}

fn case_math() -> u64 {
    let c = cards(0x0218_0005, 20);
    let o = bake_options(ImpostorLayout::Sphere, 4, 12);
    let a = bake_impostor(
        &c.indices,
        Positions::from_packed(&c.positions),
        Some(&c.normals),
        ImpostorAlbedo::VertexColors(&c.colors),
        o,
        &mut Workspace::default(),
    )
    .expect("bake");
    let mut h = Hasher::new();
    let mut r = Rng(0x0218_0006);
    for layout in [ImpostorLayout::Hemisphere, ImpostorLayout::Sphere] {
        for _ in 0..64 {
            let d = [r.signed(), r.signed(), r.signed()];
            let p = layout.encode(d).expect("encode");
            h.f32s(&p);
            h.f32s(&layout.decode(p).expect("decode"));
        }
    }
    for view in 0..a.grid.view_count().expect("views") {
        let f = a.grid.view_frame(view).expect("frame");
        h.f32s(&[
            f.direction[0],
            f.direction[1],
            f.direction[2],
            f.right[0],
            f.right[1],
            f.right[2],
            f.up[0],
            f.up[1],
            f.up[2],
        ]);
    }
    let camera = LodCamera::new(1.0471975511965976, 1080.0, 1.0).expect("camera");
    let sw = impostor_switch_distance(a.bounds, a.grid, 12, camera).expect("switch");
    h.f64(sw.distance);
    h.f64(sw.texel_error);
    h.f64(sw.view_error);
    let mut out = vec![[0f32; 4]; 20 * 20];
    for steps in [0u32, 3, 8] {
        for _ in 0..4 {
            let d = [r.signed(), r.unit() + 0.1, r.signed()];
            let blend = a.grid.blend(d).expect("blend");
            h.f32s(&blend.weights);
            h.f32s(
                &a.reconstruct_texel(
                    ImpostorFrame::from_direction(d).expect("frame"),
                    blend,
                    [0.37, 0.61],
                    reconstruct_with(steps),
                )
                .expect("texel"),
            );
            a.reconstruct_view(d, 20, reconstruct_with(steps), &mut out)
                .expect("view");
            for t in &out {
                h.f32s(t);
            }
        }
    }
    h.0
}

/// Quad cards (stride 8: position, uv, a per-vertex attribute triple) with UVs spread over the mask.
fn trim_cards(seed: u64, count: usize, uv_span: f32, perturb: bool) -> (Vec<f32>, Vec<u32>) {
    let mut r = Rng(seed);
    let (mut v, mut idx) = (vec![], vec![]);
    for k in 0..count {
        let (u0, v0) = (r.signed() * uv_span, r.signed() * uv_span);
        let (du, dv) = (0.1 + r.unit() * 0.5, 0.1 + r.unit() * 0.5);
        let base = (v.len() / 8) as u32;
        for (i, (su, sv)) in [(0.0f32, 0.0f32), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .into_iter()
            .enumerate()
        {
            let (u, w) = (u0 + su * du, v0 + sv * dv);
            let x = k as f32 + su * 2.0;
            let extra = if perturb && k % 3 == 0 && i == 2 {
                0.5
            } else {
                0.0
            };
            v.extend([x, sv * 2.0, 0.0, u, w, su + extra, sv, 1.0]);
        }
        idx.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    (v, idx)
}

fn case_trim_default() -> u64 {
    let m = mask(256, 0x0218_0010);
    let alpha = AlphaMask::new(&m, 256, 256).expect("mask");
    let (v, idx) = trim_cards(0x0218_0011, 60, 0.45, false);
    let t = trim_alpha_cards(
        &idx,
        &v,
        8,
        3,
        alpha,
        CardTrimOptions::default(),
        &mut Workspace::default(),
    )
    .expect("trim");
    let mut h = Hasher::new();
    hash_trimmed(&mut h, &t.report, &t.vertices, &t.indices, &t.card_vertices);
    h.0
}

fn case_trim_repeat() -> u64 {
    let m = mask(192, 0x0218_0012);
    let alpha = AlphaMask::new(&m, 192, 192).expect("mask");
    let (v, idx) = trim_cards(0x0218_0013, 50, 1.6, true);
    let mut o = CardTrimOptions::default();
    o.wrap = AlphaWrap::Repeat;
    o.padding = 0.0;
    o.max_vertices = 16;
    o.vertex_cost = 25.0;
    o.threshold = 100;
    let t = trim_alpha_cards(&idx, &v, 8, 3, alpha, o, &mut Workspace::default()).expect("trim");
    let mut h = Hasher::new();
    hash_trimmed(&mut h, &t.report, &t.vertices, &t.indices, &t.card_vertices);
    h.0
}

fn case_trim_fan_u16() -> u64 {
    let m = mask(128, 0x0218_0014);
    let alpha = AlphaMask::with_layout(&m, 1, 128, 128, 128).expect("mask");
    // Fan cards of 5 vertices on a circle-ish loop built from fixed offsets, stride 5 (position, uv).
    let mut r = Rng(0x0218_0015);
    let (mut v, mut idx): (Vec<f32>, Vec<u16>) = (vec![], vec![]);
    let ring = [
        [1.0f32, 0.0],
        [0.31, 0.95],
        [-0.81, 0.59],
        [-0.81, -0.59],
        [0.31, -0.95],
    ];
    for k in 0..30u16 {
        let (cu, cv, rad) = (
            0.2 + r.unit() * 0.6,
            0.2 + r.unit() * 0.6,
            0.1 + r.unit() * 0.2,
        );
        let base = (v.len() / 5) as u16;
        v.extend([f32::from(k) * 3.0, 0.0, 0.0, cu, cv]);
        for p in ring {
            v.extend([
                f32::from(k) * 3.0 + p[0],
                p[1],
                0.0,
                cu + p[0] * rad,
                cv + p[1] * rad,
            ]);
        }
        for i in 0..5u16 {
            idx.extend([base, base + 1 + i, base + 1 + (i + 1) % 5]);
        }
    }
    let t = trim_alpha_cards_generic(
        &idx,
        &v,
        5,
        3,
        alpha,
        CardTrimOptions::default(),
        &mut Workspace::default(),
    )
    .expect("trim");
    let mut h = Hasher::new();
    let wide: Vec<u32> = t.indices.iter().map(|&i| u32::from(i)).collect();
    hash_trimmed(&mut h, &t.report, &t.vertices, &wide, &t.card_vertices);
    h.0
}

/// Hash of case `case` (an index into [`CASES`]); 0 for an unknown index.

pub fn run_case(case: u32) -> u64 {
    match case {
        0 => case_bake_vertex_colors(),
        1 => case_bake_texture_u16(),
        2 => case_bake_fixed_bounds(),
        3 => case_bake_white_dense(),
        4 => case_math(),
        5 => case_trim_default(),
        6 => case_trim_repeat(),
        7 => case_trim_fan_u16(),
        _ => 0,
    }
}

/// `ImpostorReconstruct` is non-exhaustive: build it from the default and set the public field.
fn reconstruct_with(steps: u32) -> ImpostorReconstruct {
    let mut r = ImpostorReconstruct::default();
    r.parallax_steps = steps;
    r
}
