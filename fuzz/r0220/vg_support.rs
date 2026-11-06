//! Shared bounded fixtures and independent decode oracles.
use meshoptimizer_rs::{
    clusterlod::{
        attributes::{self, Channel, Encoding as E},
        dag_format::v2 as wire,
        page::{self, DiskCodec, PageCluster, PageView, QuantGrid},
        FlatDagCluster, FlatDagGroup,
    },
    *,
};
pub const ENCODINGS: [E; 11] = [
    E::Oct16x2,
    E::Oct12x2,
    E::OctSign,
    E::UnormRange16x2,
    E::F16x2,
    E::F16x4,
    E::Unorm8x4,
    E::Raw32xN(1),
    E::Raw32xN(2),
    E::Raw32xN(3),
    E::Raw32xN(4),
];
pub fn ws() -> Workspace {
    Workspace::new(Limits {
        max_bytes: 64 << 20,
        max_work: 1 << 32,
    })
}
pub fn byte(data: &[u8], i: usize) -> u8 {
    data.get(i % data.len().max(1)).copied().unwrap_or(0)
}
pub fn words(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
pub fn snapshot(v: PageView<'_>) -> Vec<u32> {
    let mut result = vec![v.attribute_layout_hash(), v.grid().exponent() as u32];
    assert_eq!(v.words().len(), v.words()[1] as usize);
    for c in 0..v.cluster_count() {
        let d = v.cluster(c).unwrap();
        let (mut q, mut p, mut t) = ([[0; 3]; 256], [[0.; 3]; 256], [[0; 3]; 256]);
        let nv = v.decode_coordinates(c, &mut q).unwrap();
        assert_eq!(nv, v.decode_positions(c, &mut p).unwrap());
        let nt = v.decode_triangles(c, &mut t).unwrap();
        assert_eq!(nv, d.vertex_count as usize);
        assert_eq!(nt, d.triangle_count as usize);
        assert!(v.decode_positions(c, &mut []).is_err());
        assert!(v.decode_coordinates(c, &mut []).is_err());
        assert!(v.decode_triangles(c, &mut []).is_err());
        for i in 0..nv {
            assert_eq!(p[i], v.grid().position(q[i]));
            result.extend(p[i].map(f32::to_bits));
        }
        for (i, tri) in t[..nt].iter().enumerate() {
            assert!(tri.iter().all(|&i| usize::from(i) < nv));
            result.extend(tri.map(u32::from));
            result.push(v.triangle_material(c, i).unwrap().into());
        }
        assert!(v.triangle_material(c, nt).is_err());
        if let Some(a) = v.attributes(c).unwrap() {
            for k in 0..a.channel_count() {
                a.channel(k).unwrap();
                for i in 0..nv {
                    result.extend(a.decode(k, i).unwrap().map(f32::to_bits));
                }
                assert!(a.decode(k, nv).is_err());
            }
            assert!(a.channel(a.channel_count()).is_err());
        }
    }
    assert!(v.cluster(v.cluster_count()).is_err());
    assert!(v.attributes(v.cluster_count()).is_err());
    assert!(v.decode_positions(v.cluster_count(), &mut []).is_err());
    result
}
pub fn far() -> wire::FarField {
    let g = ImpostorGrid::new(ImpostorLayout::Sphere, 3).unwrap();
    let s = impostor_switch_distance(
        ImpostorBounds::new([0.; 3], 2.).unwrap(),
        g,
        32,
        LodCamera::new(1., 1080., 1.).unwrap(),
    )
    .unwrap();
    wire::FarField::from_switch(17, b"atlas", g, 32, s).unwrap()
}
/// Each input chooses encoding, vertex count, normals, half-float and range values.
pub fn attribute_fixture(data: &[u8]) -> (Vec<u32>, Vec<u32>, Vec<u8>) {
    let e = ENCODINGS[byte(data, 0) as usize % 11];
    let n = 3 + byte(data, 1) as usize % 254;
    let ch = [
        Channel {
            tag: 99,
            encoding: e,
            source_offset: 0,
        },
        Channel {
            tag: 100,
            encoding: E::Raw32xN(1),
            source_offset: 0,
        },
    ];
    let mut values = Vec::new();
    let mut positions = Vec::new();
    let mut idx = Vec::new();
    for i in 0..n {
        let f = byte(data, i + 2) as f32 / 255.;
        let mut x = match e {
            E::Oct16x2 | E::Oct12x2 | E::OctSign => {
                let a = [f * 2. - 1., (f * 79.).sin(), (f * 37.).cos()];
                let l = a.iter().map(|x| x * x).sum::<f32>().sqrt();
                [
                    a[0] / l,
                    a[1] / l,
                    a[2] / l,
                    if i % 2 == 0 { 1. } else { -1. },
                ]
            }
            E::UnormRange16x2 => [f * 41. - 20., f * 4. + 1., 0., 0.],
            E::F16x2 | E::F16x4 => [f * 65504., -f * 65504., f * 0.0001, 0.3333],
            E::Unorm8x4 => [f, 1. - f, 0.3333, 0.5],
            _ => [f, -f, f * 123., -0.],
        };
        // Exact half-way at the largest finite bin and signed subnormals.
        if matches!(e, E::F16x2 | E::F16x4) && i == 0 {
            x = [65488., -65488., 0.000000059604645, -0.000000059604645];
        }
        values.extend_from_slice(&x[..e.components()]);
        positions.push([
            i as f32 + f / 1000.,
            (i % 3) as f32,
            if byte(data, 2) & 1 == 0 {
                0.
            } else {
                (i % 7) as f32
            },
        ]);
    }
    for i in 1..n - 1 {
        idx.extend([0, i as u32, i as u32 + 1]);
    }
    let a = Attributes::from_interleaved(&values, n, e.components(), e.components(), 0).unwrap();
    let ids = (0..n as u32).collect::<Vec<_>>();
    let block = attributes::encode(&ch, a, &ids, &mut ws()).unwrap();
    let b = attributes::Block::new(&block, n).unwrap();
    for i in 0..n {
        let got = b.decode(0, i).unwrap();
        assert_eq!(
            b.decode(1, i).unwrap()[0].to_bits(),
            values[i * e.components()].to_bits()
        );
        let src = &values[i * e.components()..][..e.components()];
        let bound = match e {
            E::Oct16x2 | E::OctSign => 0.000491,
            E::Oct12x2 => 0.00782,
            E::F16x2 | E::F16x4 => 16.,
            E::Unorm8x4 => 1. / 510. + 1e-7,
            _ => 0.,
        };
        if matches!(e, E::Oct16x2 | E::Oct12x2 | E::OctSign) {
            assert!(
                got[..3]
                    .iter()
                    .zip(src)
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f32>()
                    .sqrt()
                    <= bound
            );
            if e == E::OctSign {
                assert_eq!(got[3], src[3]);
            }
        } else {
            for c in 0..e.components() {
                let limit = if e == E::UnormRange16x2 {
                    let lo = values
                        .chunks_exact(2)
                        .map(|v| v[c])
                        .fold(f32::INFINITY, f32::min);
                    let hi = values
                        .chunks_exact(2)
                        .map(|v| v[c])
                        .fold(f32::NEG_INFINITY, f32::max);
                    let r = hi - lo;
                    let ulp = |x: f32| (x.abs().next_up() - x.abs()) as f64;
                    r as f64 / 131070. + 4. * (ulp(r) + ulp(lo))
                } else {
                    bound as f64
                };
                assert!((got[c] as f64 - src[c] as f64).abs() <= limit);
                if matches!(e, E::Raw32xN(_)) {
                    assert_eq!(got[c].to_bits(), src[c].to_bits());
                }
            }
        }
    }
    let grid = QuantGrid::new(-10).unwrap();
    let mats: Vec<u16> = (0..idx.len() / 3)
        .map(|i| {
            if byte(data, 3) & 1 == 0 {
                0
            } else {
                (i * 3 / (idx.len() / 3)) as u16
            }
        })
        .collect();
    let mut pc = PageCluster::new(0, -1, &positions, &idx);
    pc.attributes = Some(&block);
    pc.materials = Some(&mats);
    let mem = page::encode_page(
        grid,
        &[pc],
        attributes::layout_hash(&ch).unwrap(),
        &mut ws(),
    )
    .unwrap();
    let view = PageView::new(&mem, grid).unwrap();
    let mut decoded = [[0.; 3]; 256];
    assert_eq!(view.decode_positions(0, &mut decoded).unwrap(), n);
    for (got, src) in decoded[..n].iter().zip(&positions) {
        let distance = got
            .iter()
            .zip(src)
            .map(|(a, b)| (*a as f64 - *b as f64).powi(2))
            .sum::<f64>()
            .sqrt();
        assert!(distance <= grid.max_displacement() as f64);
    }
    let expected = snapshot(view);
    for codec in [DiskCodec::None, DiskCodec::Compressed] {
        let disk = page::encode_disk_page(&mem, grid, codec, &mut ws()).unwrap();
        let mut out = vec![0; mem.len()];
        let count = page::transcode_page(&disk, grid, &mut out, &mut ws()).unwrap();
        assert_eq!(count, mem.len());
        assert_eq!(expected, snapshot(PageView::new(&out, grid).unwrap()));
    }
    // Writer fixture uses uniform per-vertex slots; multi-range pages above exercise mixed ranges.
    let mut group = FlatDagGroup::default();
    group.first_cluster = 0;
    group.cluster_count = 1;
    let groups = [group];
    let mut cluster = FlatDagCluster::default();
    cluster.group = 0;
    cluster.refined = -1;
    cluster.first_index = 0;
    // Use independent triangles for writer ranges so slots stay uniform on
    // every triangle while a cluster can contain several material ranges.
    let count = n / 3 * 3;
    let writer_idx: Vec<u32> = (0..count as u32).collect();
    let vm: Vec<u16> = (0..n).map(|i| (i / 3 / ((n / 9).max(1))) as u16).collect();
    cluster.index_count = count as u32;
    let clusters = [cluster];
    let mut source = wire::Source::new(&groups, &clusters, &positions, &writer_idx);
    source.channels = &ch;
    source.attributes = Some(a);
    source.materials = Some(&vm);
    let mut options = wire::WriteOptions::new(grid);
    options.far_field = Some(far());
    options.disk_codec = if byte(data, 3) & 1 == 0 {
        DiskCodec::None
    } else {
        DiskCodec::Compressed
    };
    let file = wire::write(&source, &options, &mut ws()).unwrap().0;
    let reader = wire::Reader::read_all(&file, &mut ws()).unwrap();
    for p in 0..reader.mesh().page_count as usize {
        let rec = reader.page(p).unwrap();
        let disk = &file
            [rec.disk_offset as usize..(rec.disk_offset + u64::from(rec.disk_length)) as usize];
        let mut a = vec![0; reader.page_capacity() / 4];
        let mut b = vec![0; a.len()];
        let expected = snapshot(reader.transcode_page(p, disk, &mut a, &mut ws()).unwrap());
        let m = page::transcode_page(disk, grid, &mut b, &mut ws()).unwrap();
        assert_eq!(expected, snapshot(PageView::new(&b[..m], grid).unwrap()));
    }
    (block, mem, file)
}
