use crate::support;
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        attributes::{Channel, Encoding},
        cut::{self, CutView, Projection, Threshold},
        dag_format::v2 as wire,
        page::{DiskCodec, QuantGrid},
        v2::{self, AlphaCoverage, BuildMesh},
    },
    *,
};
use std::sync::OnceLock;
fn pool(n: usize) -> &'static rayon::ThreadPool {
    static POOLS: OnceLock<[rayon::ThreadPool; 2]> = OnceLock::new();
    &POOLS.get_or_init(|| {
        [1, 2].map(|n| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build()
                .unwrap()
        })
    })[n - 1]
}
fn cuts(b: &v2::Built) {
    let mut producers = vec![Vec::new(); b.groups.len()];
    for (i, c) in b.clusters.iter().enumerate() {
        if c.refined >= 0 {
            assert!(b.groups[c.group as usize].error >= b.groups[c.refined as usize].error);
            producers[c.refined as usize].push(i);
        }
    }
    let mut out = vec![0; b.clusters.len()];
    for position in [[0.; 3], [10., 20., 30.], [-100., 0., 1.]] {
        for threshold in [
            Threshold::Finest,
            Threshold::Pixels(0.),
            Threshold::Pixels(1.),
            Threshold::Pixels(100000.),
        ] {
            let view = CutView {
                position,
                projection: Projection::Perspective {
                    proj_y: 1.5,
                    znear: 0.01,
                },
                viewport_height: 1080.,
                scale: 1.,
            };
            let n =
                cut::select_cut(&b.groups, &b.clusters, &view, threshold, None, &mut out).unwrap();
            let mut selected = vec![0u32; b.clusters.len()];
            for &i in &out[..n] {
                selected[i as usize] += 1;
            }
            let mut counts = vec![(0u32, 0u32); b.clusters.len()];
            for i in (0..b.clusters.len()).rev() {
                let above = &producers[b.clusters[i].group as usize];
                let lo = above.iter().map(|&j| counts[j].0).min().unwrap_or(0);
                let hi = above.iter().map(|&j| counts[j].1).max().unwrap_or(0);
                counts[i] = (lo + selected[i], hi + selected[i]);
                if b.clusters[i].refined < 0 {
                    assert_eq!(counts[i], (1, 1), "source region selected exactly once");
                }
            }
        }
    }
}
/// At most 169 source vertices and 288 triangles. Each byte changes geometry or policy.
pub fn exercise(data: &[u8]) -> Vec<u8> {
    let n = 2 + support::byte(data, 0) as usize % 11;
    let mode = support::byte(data, 1);
    let (mut p, mut uv, mut idx, mut mats) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for y in 0..=n {
        for x in 0..=n {
            p.push([
                x as f32,
                y as f32,
                support::byte(data, 2 + y * (n + 1) + x) as f32 / 255.
                    * if mode & 1 == 0 { 0.1 } else { 2. },
            ]);
            uv.extend([x as f32 / n as f32, y as f32 / n as f32]);
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = (y * (n + 1) + x) as u32;
            let b = a + (n + 1) as u32;
            idx.extend([a, b, a + 1, a + 1, b, b + 1]);
            let slot = if mode & 2 == 0 { 0 } else { (x % 2) as u16 };
            mats.extend([slot; 2]);
        }
    }
    let normals = vec![[0., 0., 1.]; p.len()];
    let attrs = Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap();
    let mask_bytes: Vec<u8> = (0..64)
        .map(|i| {
            if support::byte(data, i) & 1 == 0 {
                0
            } else {
                255
            }
        })
        .collect();
    let masks = [Some(AlphaMask::new(&mask_bytes, 8, 8).unwrap())];
    let grid = QuantGrid::new(-10).unwrap();
    let mut config = cl::default_config(16 + 16 * (usize::from(mode >> 5) % 3)).unwrap();
    config.partition_size = 2;
    config.optimize_clusters = mode & 4 != 0;
    config.cluster_spatial = mode & 8 != 0;
    let run = |parallel: bool| {
        let mut m = BuildMesh::new(&idx, &p);
        m.normals = Some(&normals);
        m.attributes = Some(attrs);
        m.attribute_weights = &[0., 0.];
        m.materials = Some(&mats);
        if mode & 16 != 0 {
            let mut a = AlphaCoverage::new(&masks, 0);
            a.resolution = 8;
            if mode & 32 != 0 {
                a.ratio_band = [1., 1.];
                a.blurred_difference = 0.;
            }
            m.alpha_coverage = Some(a);
        }
        let mut ws = if parallel {
            Workspace::new(Limits {
                max_bytes: usize::MAX,
                max_work: u64::MAX,
            })
        } else {
            support::ws()
        };
        let b = v2::build(config, m, grid, &mut ws).expect("bounded valid build");
        for t in b.indices.chunks_exact(3) {
            let slot = b.vertex_material[t[0] as usize];
            assert!(t.iter().all(|&v| b.vertex_material[v as usize] == slot));
        }
        cuts(&b);
        let fallback =
            v2::measure_fallback(&b, &LodQualityOptions::default(), &mut support::ws()).unwrap();
        assert!(!fallback.indices.is_empty());
        assert!(fallback.quality.symmetric.max.is_finite());
        let channels = [
            Channel {
                tag: 1,
                encoding: Encoding::UnormRange16x2,
                source_offset: 0,
            },
            Channel {
                tag: 2,
                encoding: Encoding::Oct16x2,
                source_offset: 2,
            },
        ];
        let mut s = wire::Source::new(&b.groups, &b.clusters, &b.positions, &b.indices);
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
        s.materials = Some(&b.vertex_material);
        let mut o = wire::WriteOptions::new(grid);
        o.disk_codec = if mode & 64 == 0 {
            DiskCodec::None
        } else {
            DiskCodec::Compressed
        };
        o.far_field = Some(support::far());
        let file = wire::write(&s, &o, &mut support::ws())
            .expect("writer accepts built DAG")
            .0;
        let r = wire::Reader::open(&file).expect("writer output opens");
        wire::Reader::read_all(&file, &mut support::ws()).expect("writer output reads");
        for page in 0..r.mesh().page_count as usize {
            let rec = r.page(page).unwrap();
            let disk = &file
                [rec.disk_offset as usize..(rec.disk_offset + u64::from(rec.disk_length)) as usize];
            let mut out = vec![0; r.page_capacity() / 4];
            let v = r
                .transcode_page(page, disk, &mut out, &mut support::ws())
                .unwrap();
            support::snapshot(v);
        }
        file
    };
    let bounded = run(false);
    let one = pool(1).install(|| run(true));
    let two = pool(2).install(|| run(true));
    assert_eq!(bounded, one);
    assert_eq!(one, two);
    one
}
