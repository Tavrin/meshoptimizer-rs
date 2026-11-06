//! Exact byte hashes for v2 build, wire format, attributes and alpha policy.
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        attributes::{self, Channel, Encoding},
        dag_format::v2 as wire,
        page::{self, DiskCodec, QuantGrid},
        v2::{self, AlphaCoverage, BuildMesh},
    },
    *,
};
pub const CASES: &[&str] = &[
    "v2-build-write",
    "v2-material-attributes",
    "v2-alpha-coverage",
    "attribute-encodings",
];
fn hash(b: &[u8]) -> u64 {
    wire::xxh64(b, 0)
}
fn fixture() -> (Vec<[f32; 3]>, Vec<u32>, Vec<f32>) {
    let n = 12usize;
    let mut p = Vec::new();
    let mut uv = Vec::new();
    let mut ix = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            p.push([x as f32, y as f32, ((x * 17 + y * 13) % 7) as f32 * 0.125]);
            uv.extend([x as f32 / n as f32, y as f32 / n as f32]);
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = (y * (n + 1) + x) as u32;
            let b = a + (n + 1) as u32;
            ix.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    (p, ix, uv)
}
fn file(case: u32) -> Vec<u8> {
    let (p, ix, uv) = fixture();
    let grid = QuantGrid::new(-10).unwrap();
    let mut c = cl::default_config(32).unwrap();
    c.partition_size = 2;
    c.optimize_clusters = case != 2;
    let mats: Vec<u16> = (0..ix.len() / 3).map(|t| ((t / 24) % 2) as u16).collect();
    let mask: Vec<u8> = (0..64).map(|i| if i % 3 == 0 { 255 } else { 0 }).collect();
    let masks = [Some(AlphaMask::new(&mask, 8, 8).unwrap())];
    let mut m = BuildMesh::new(&ix, &p);
    if case > 0 {
        m.attributes = Some(Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap());
        m.attribute_weights = &[0., 0.];
    }
    if case == 1 {
        m.materials = Some(&mats);
    }
    if case == 2 {
        let mut a = AlphaCoverage::new(&masks, 0);
        a.ratio_band = [1., 1.];
        a.blurred_difference = 0.;
        m.alpha_coverage = Some(a);
    }
    let mut ws = Workspace::new(Limits {
        max_bytes: usize::MAX,
        max_work: u64::MAX,
    });
    let b = v2::build(c, m, grid, &mut ws).unwrap();
    if case == 2 {
        assert!(b.alpha_coverage_stuck_groups > 0);
    }
    if case == 1 {
        assert!(b.vertex_material.contains(&1));
    }
    let mut s = wire::Source::new(&b.groups, &b.clusters, &b.positions, &b.indices);
    let channels = [Channel {
        tag: 1,
        encoding: Encoding::UnormRange16x2,
        source_offset: 0,
    }];
    if case > 0 {
        s.channels = &channels;
        s.attributes = Some(
            Attributes::from_interleaved(
                &b.attributes,
                b.positions.len(),
                b.attribute_stride,
                b.attribute_stride,
                0,
            )
            .unwrap(),
        );
    }
    if case == 1 {
        s.materials = Some(&b.vertex_material);
    }
    let mut o = wire::WriteOptions::new(grid);
    o.disk_codec = DiskCodec::Compressed;
    if case == 1 {
        let g = ImpostorGrid::new(ImpostorLayout::Sphere, 3).unwrap();
        let sw = impostor_switch_distance(
            ImpostorBounds::new([0.; 3], 2.).unwrap(),
            g,
            32,
            LodCamera::new(1., 1080., 1.).unwrap(),
        )
        .unwrap();
        o.far_field = Some(wire::FarField::from_switch(17, b"external atlas", g, 32, sw).unwrap());
    }
    let bytes = wire::write(&s, &o, &mut ws).unwrap().0;
    let r = wire::Reader::read_all(&bytes, &mut ws).unwrap();
    let mut all = bytes.clone();
    for p in &b.positions {
        for x in p {
            all.extend(x.to_bits().to_le_bytes());
        }
    }
    for x in &b.attributes {
        all.extend(x.to_bits().to_le_bytes());
    }
    for x in b.indices.iter().chain(&b.vertex_source) {
        all.extend(x.to_le_bytes());
    }
    for x in &b.vertex_material {
        all.extend(x.to_le_bytes());
    }
    for g in &b.groups {
        for x in g.center.iter().chain([&g.radius, &g.error]) {
            all.extend(x.to_bits().to_le_bytes());
        }
        for x in [g.depth as u32, g.first_cluster, g.cluster_count] {
            all.extend(x.to_le_bytes());
        }
    }
    for c in &b.clusters {
        for x in [c.first_index, c.index_count, c.group, c.refined as u32] {
            all.extend(x.to_le_bytes());
        }
        for x in c.center.iter().chain([&c.radius]) {
            all.extend(x.to_bits().to_le_bytes());
        }
    }
    all.extend((b.attribute_stride as u64).to_le_bytes());
    all.extend((b.alpha_coverage_raised_groups as u64).to_le_bytes());
    all.extend((b.material_stuck_groups as u64).to_le_bytes());
    for p in 0..r.mesh().page_count as usize {
        let pr = r.page(p).unwrap();
        let mut out = vec![0; pr.memory_words as usize];
        let v = r
            .transcode_page(
                p,
                &bytes[pr.disk_range().start as usize..pr.disk_range().end as usize],
                &mut out,
                &mut ws,
            )
            .unwrap();
        for &w in v.words() {
            all.extend(w.to_le_bytes());
        }
        for c in 0..v.cluster_count() {
            let mut xyz = [[0.; 3]; 256];
            let n = v.decode_positions(c, &mut xyz).unwrap();
            for pt in &xyz[..n] {
                for x in pt {
                    all.extend(x.to_bits().to_le_bytes());
                }
            }
            let mut tri = [[0; 3]; 256];
            let n = v.decode_triangles(c, &mut tri).unwrap();
            for t in &tri[..n] {
                all.extend(t);
            }
        }
    }
    all.extend((b.stuck_groups as u64).to_le_bytes());
    all.extend((b.alpha_coverage_stuck_groups as u64).to_le_bytes());
    all
}
#[no_mangle]
pub extern "C" fn run_case(case: u32) -> u64 {
    if case < 3 {
        return hash(&file(case));
    }
    let mut out = Vec::new();
    for enc in [
        Encoding::Oct16x2,
        Encoding::Oct12x2,
        Encoding::OctSign,
        Encoding::UnormRange16x2,
        Encoding::F16x2,
        Encoding::F16x4,
        Encoding::Unorm8x4,
        Encoding::Raw32xN(1),
        Encoding::Raw32xN(2),
        Encoding::Raw32xN(3),
        Encoding::Raw32xN(4),
    ] {
        let data: Vec<f32> = (0..32)
            .flat_map(|i| {
                let f = i as f32 / 32.;
                let a = if matches!(
                    enc,
                    Encoding::Oct16x2 | Encoding::Oct12x2 | Encoding::OctSign
                ) {
                    [0., 0., 1., 1.]
                } else {
                    [f, 1. - f, 0.00001, -0.]
                };
                a[..enc.components()].to_vec()
            })
            .collect();
        let attrs =
            Attributes::from_interleaved(&data, 32, enc.components(), enc.components(), 0).unwrap();
        let ids: Vec<u32> = (0..32).collect();
        let ch = [Channel {
            tag: 42,
            encoding: enc,
            source_offset: 0,
        }];
        let words = attributes::encode(&ch, attrs, &ids, &mut Workspace::default()).unwrap();
        for w in &words {
            out.extend(w.to_le_bytes());
        }
        let block = attributes::Block::new(&words, 32).unwrap();
        for v in 0..32 {
            for x in block.decode(0, v).unwrap() {
                out.extend(x.to_bits().to_le_bytes());
            }
        }
    }
    hash(&out)
}
#[no_mangle]
pub extern "C" fn decode_bench(iter: u32) -> u64 {
    let (p, ix, _) = fixture();
    let grid = QuantGrid::new(-10).unwrap();
    let local: Vec<(Vec<[f32; 3]>, Vec<u32>)> = ix
        .chunks(600)
        .map(|chunk| {
            let mut ids = Vec::new();
            let mut pos = Vec::new();
            let mut indices = Vec::new();
            for &id in chunk {
                let k = ids.iter().position(|&v| v == id).unwrap_or_else(|| {
                    ids.push(id);
                    pos.push(p[id as usize]);
                    ids.len() - 1
                });
                indices.push(k as u32);
            }
            (pos, indices)
        })
        .collect();
    let clusters: Vec<_> = local
        .iter()
        .enumerate()
        .map(|(i, (p, ix))| page::PageCluster::new(i as u32, -1, p, ix))
        .collect();
    let mut ws = Workspace::default();
    let words = page::encode_page(grid, &clusters, 0, &mut ws).unwrap();
    let disk = page::encode_disk_page(&words, grid, DiskCodec::Compressed, &mut ws).unwrap();
    let mut out = vec![0; words.len()];
    let mut sum = 0u64;
    for _ in 0..iter {
        let n = page::transcode_page(std::hint::black_box(&disk), grid, &mut out, &mut ws).unwrap();
        let view = page::PageView::new(&out[..n], grid).unwrap();
        let mut xyz = [[0.; 3]; 256];
        let mut triangles = [[0u8; 3]; 256];
        for c in 0..view.cluster_count() {
            let nv = view.decode_positions(c, &mut xyz).unwrap();
            let nt = view.decode_triangles(c, &mut triangles).unwrap();
            sum = sum.wrapping_add(nv as u64 + nt as u64 + out[0] as u64);
            std::hint::black_box(&xyz);
            std::hint::black_box(&triangles);
        }
    }
    sum
}
