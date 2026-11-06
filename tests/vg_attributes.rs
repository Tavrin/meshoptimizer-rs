#![allow(clippy::field_reassign_with_default)]
#[cfg(feature = "clusterlod")]
use meshoptimizer_rs::clusterlod::dag_format::v2;
use meshoptimizer_rs::{
    clusterlod::{
        attributes::{self, Block, Channel, Encoding as E},
        dag_format::Error,
        page::{self, DiskCodec, PageCluster, PageView, QuantGrid},
    },
    Attributes, Workspace,
};

const ENCODINGS: [E; 11] = [
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

#[test]
fn encoding_bounds_and_rejection() {
    for encoding in ENCODINGS {
        let channel = Channel {
            tag: 99,
            encoding,
            source_offset: 0,
        };
        let mut data = Vec::new();
        for k in 0..256 {
            let f = k as f32 / 255.0;
            let v = match encoding {
                E::Oct16x2 | E::Oct12x2 | E::OctSign => {
                    let n = [f * 2. - 1., (f * 79.).sin(), (f * 37.).cos()];
                    let len = n.iter().map(|x| x * x).sum::<f32>().sqrt();
                    [
                        n[0] / len,
                        n[1] / len,
                        n[2] / len,
                        if k % 2 == 0 { 1. } else { -1. },
                    ]
                }
                E::UnormRange16x2 => [f * 41. - 20., f * 4. + 1., 0., 0.],
                E::F16x2 | E::F16x4 => [f * 65504., -f * 65504., f * 0.0001, 0.3333],
                E::Unorm8x4 => [f, 1. - f, 0.3333, 0.5],
                _ => [f, -f, f * 123., -0.],
            };
            data.extend_from_slice(&v[..encoding.components()]);
        }
        let view = Attributes::from_interleaved(
            &data,
            256,
            encoding.components(),
            encoding.components(),
            0,
        )
        .unwrap();
        let ids: Vec<_> = (0..256).collect();
        let words = attributes::encode(&[channel], view, &ids, &mut Workspace::default()).unwrap();
        let block = Block::new(&words, 256).unwrap();
        for k in 0..256 {
            let got = block.decode(0, k).unwrap();
            let src = &data[k * encoding.components()..][..encoding.components()];
            let bound = match encoding {
                E::Oct16x2 | E::OctSign => 0.000491,
                E::Oct12x2 => 0.00782,
                E::UnormRange16x2 => 41. / 131070. + 0.0001,
                E::F16x2 | E::F16x4 => 16.,
                E::Unorm8x4 => 1. / 510. + 1e-7,
                _ => 0.,
            };
            for c in 0..encoding.components() {
                assert!(
                    (got[c] - src[c]).abs() <= bound,
                    "{encoding:?}, {k}, {c}: {} {}",
                    got[c],
                    src[c]
                );
            }
            if matches!(encoding, E::Oct16x2 | E::Oct12x2 | E::OctSign) {
                let error = got[..3]
                    .iter()
                    .zip(src)
                    .map(|(a, b)| (a - b) * (a - b))
                    .sum::<f32>()
                    .sqrt();
                assert!(error <= bound);
                assert!((got[..3].iter().map(|x| x * x).sum::<f32>() - 1.).abs() < 1e-6);
            }
            if matches!(encoding, E::Raw32xN(_)) {
                for c in 0..encoding.components() {
                    assert_eq!(got[c].to_bits(), src[c].to_bits());
                }
            }
        }
        for n in 0..words.len() {
            assert!(Block::new(&words[..n], 256).is_err());
        }
        let mut bad = words.clone();
        bad[5] = 999;
        assert_eq!(Block::new(&bad, 256).unwrap_err(), Error::Unsupported);
        let mut bad = words.clone();
        bad[2] = u32::MAX;
        assert!(Block::new(&bad, 256).is_err());
        let mut bad = words.clone();
        bad[8] = f32::NAN.to_bits();
        assert!(Block::new(&bad, 256).is_err());
    }
    for encoding in [E::F16x2, E::F16x4] {
        let values = [65488.0, -65488.0, 65488.0, -65488.0];
        let n = encoding.components();
        let source = Attributes::from_interleaved(&values[..n], 1, n, n, 0).unwrap();
        let words = attributes::encode(
            &[Channel {
                tag: 0,
                encoding,
                source_offset: 0,
            }],
            source,
            &[0],
            &mut Workspace::default(),
        )
        .unwrap();
        let got = Block::new(&words, 1).unwrap().decode(0, 0).unwrap();
        for c in 0..n {
            assert_eq!((got[c] - values[c]).abs(), 16.0);
        }
    }
    for (e, data) in [
        (E::Oct16x2, vec![0., 0., 0.]),
        (E::OctSign, vec![1., 0., 0., 0.]),
        (E::F16x2, vec![65505., 0.]),
        (E::Unorm8x4, vec![2., 0., 0., 0.]),
        (E::Raw32xN(1), vec![f32::NAN]),
    ] {
        let a = Attributes::from_interleaved(&data, 1, data.len(), data.len(), 0).unwrap();
        assert!(attributes::encode(
            &[Channel {
                tag: 0,
                encoding: e,
                source_offset: 0
            }],
            a,
            &[0],
            &mut Workspace::default()
        )
        .is_err());
    }
    assert!(attributes::validate_layout(&[Channel {
        tag: 0,
        encoding: E::Raw32xN(0),
        source_offset: 0
    }])
    .is_err());
}

fn replay_vectors(name: &str, memory: &[u32], grid: QuantGrid) -> String {
    let view = PageView::new(memory, grid).unwrap();
    let mut decoded = Vec::new();
    for c in 0..view.cluster_count() {
        let block = view.attributes(c).unwrap().unwrap();
        let d = view.cluster(c).unwrap();
        for v in 0..d.vertex_count as usize {
            for k in 0..block.channel_count() {
                decoded.extend(block.decode(k, v).unwrap().map(f32::to_bits));
            }
        }
        for t in 0..d.triangle_count as usize {
            decoded.push(u32::from(view.triangle_material(c, t).unwrap()));
        }
    }
    let mut text = format!(
        "# Attribute page replay vectors. See PAGE_FORMAT.md.\nfixture {name}\nexponent {}\n",
        grid.exponent()
    );
    for (name, words) in [("memory", memory), ("decoded", decoded.as_slice())] {
        text.push_str(&format!("{name} {}\n", words.len()));
        for line in words.chunks(8) {
            text.push_str(
                &line
                    .iter()
                    .map(|w| format!("{w:08x}"))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            text.push('\n');
        }
    }
    text
}

#[test]
fn page_attributes_material_ranges_and_wgsl_vectors() {
    let grid = QuantGrid::new(-10).unwrap();
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let idx = [0, 1, 2, 1, 3, 2];
    let slots = [3, 65535];
    let ch = [
        Channel {
            tag: 0,
            encoding: E::OctSign,
            source_offset: 0,
        },
        Channel {
            tag: 1,
            encoding: E::UnormRange16x2,
            source_offset: 4,
        },
    ];
    let values = [
        0., 0., 1., 1., -2., 4., 0., 0., 1., -1., 2., 4., 0., 0., -1., 1., -2., 8., 1., 0., 0.,
        -1., 2., 8.,
    ];
    let a = Attributes::from_interleaved(&values, 4, 6, 6, 0).unwrap();
    let hash = attributes::layout_hash(&ch).unwrap();
    let block = attributes::encode(&ch, a, &[0, 1, 2, 3], &mut Workspace::default()).unwrap();
    let mut c = PageCluster::new(0, -1, &p, &idx);
    c.attributes = Some(&block);
    c.materials = Some(&slots);
    let memory = page::encode_page(grid, &[c], hash, &mut Workspace::default()).unwrap();
    let mut vectors = replay_vectors("seams", &memory, grid);
    let all: Vec<_> = ENCODINGS
        .iter()
        .enumerate()
        .map(|(tag, &encoding)| Channel {
            tag: 100 + tag as u32,
            encoding,
            source_offset: 0,
        })
        .collect();
    let axes = [
        1., 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1., 1., 0., 0., 1.,
    ];
    let a = Attributes::from_interleaved(&axes, 4, 4, 4, 0).unwrap();
    let encoded = attributes::encode(&all, a, &[0, 1, 2, 3], &mut Workspace::default()).unwrap();
    let mut cluster = PageCluster::new(0, -1, &p, &idx);
    cluster.attributes = Some(&encoded);
    cluster.materials = Some(&slots);
    let full = page::encode_page(
        grid,
        &[cluster],
        attributes::layout_hash(&all).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    vectors.push_str(&replay_vectors("all-encodings", &full, grid));
    if let Some(path) = std::env::var_os("ATTRIBUTE_VECTORS_PATH") {
        std::fs::write(path, &vectors).unwrap();
    }
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/attribute_page_vectors.txt");
    assert_eq!(std::fs::read_to_string(path).unwrap(), vectors);

    for codec in [DiskCodec::None, DiskCodec::Compressed] {
        let disk = page::encode_disk_page(&memory, grid, codec, &mut Workspace::default()).unwrap();
        let mut got = vec![0; memory.len()];
        page::transcode_page(&disk, grid, &mut got, &mut Workspace::default()).unwrap();
        assert_eq!(got, memory);
        let v = PageView::new(&got, grid).unwrap();
        assert_eq!(v.triangle_material(0, 0).unwrap(), 3);
        assert_eq!(v.triangle_material(0, 1).unwrap(), 65535);
        let b = v.attributes(0).unwrap().unwrap();
        assert_eq!(b.decode(0, 0).unwrap(), [0., 0., 1., 1.]);
        assert_eq!(b.decode(0, 1).unwrap(), [0., 0., 1., -1.]);
    }
    let vectors: Vec<u32> = include_str!("../docs/attribute_vectors.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .flat_map(|line| line.split_whitespace())
        .map(|w| u32::from_str_radix(w, 16).unwrap())
        .collect();
    assert_eq!(block, vectors);
    // Fixed words consumed by the WGSL vector replay: +Z/-Z, handedness, UV endpoints.
    assert_eq!(&block[20..26], &[0, 0, 0, 0, 1, 65535]);
    let desc = page::PAGE_HEADER_WORDS;
    let at = memory[desc + 9] as usize;
    let mut bad = memory.clone();
    bad[desc + 9] = u32::MAX;
    assert!(PageView::new(&bad, grid).is_err());
    let mut bad = memory.clone();
    bad[at] = u32::MAX;
    assert!(PageView::new(&bad, grid).is_err());
    let table = memory[desc + 15] as usize;
    let mut bad = memory.clone();
    bad[table + 2] = bad[table + 1];
    assert!(PageView::new(&bad, grid).is_err());
}

#[cfg(feature = "clusterlod")]
#[test]
fn builder_and_container_material_roundtrip_and_identity() {
    use meshoptimizer_rs::clusterlod::{
        self as cl,
        v2::{self as build, BuildMesh},
    };
    let mut p = Vec::new();
    let mut idx = Vec::new();
    let mut slots = Vec::new();
    for y in 0..=12 {
        for x in 0..=12 {
            p.push([x as f32, y as f32, 0.]);
        }
    }
    for y in 0..12 {
        for x in 0..12 {
            let a = y * 13 + x;
            idx.extend([a, a + 1, a + 13, a + 1, a + 14, a + 13]);
            slots.extend([((x + y) % 3) as u16; 2]);
        }
    }
    let uv: Vec<_> = p.iter().flat_map(|v| [v[0] / 12., v[1] / 12.]).collect();
    let normals = vec![[0., 0., 1.]; p.len()];
    let mut input = BuildMesh::new(&idx, &p);
    input.materials = Some(&slots);
    input.normals = Some(&normals);
    input.attributes = Some(Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap());
    input.attribute_weights = &[0., 0.];
    let c = cl::default_config(16).unwrap();
    let grid = QuantGrid::new(-12).unwrap();
    let built = build::build(c, input, grid, &mut Workspace::default()).unwrap();
    let mut strict_config = c;
    strict_config.simplify_fallback_sloppy = false;
    let mut strict_input = BuildMesh::new(&idx, &p);
    strict_input.materials = Some(&slots);
    strict_input.normals = Some(&normals);
    strict_input.attributes = Some(Attributes::from_interleaved(&uv, p.len(), 2, 2, 0).unwrap());
    strict_input.attribute_weights = &[0., 0.];
    let strict =
        build::build(strict_config, strict_input, grid, &mut Workspace::default()).unwrap();
    assert_eq!(strict.material_stuck_groups,0,"regular simplification preserves protected slots; enabling sloppy fallback must trigger the rejection fixture");
    assert!(built.positions.len() > p.len());
    assert!(built.stuck_groups > 0);
    assert!(
        built.material_stuck_groups > 0,
        "fixture must exercise mixed-slot sloppy rejection"
    );
    for t in built.indices.as_chunks::<3>().0 {
        assert_eq!(
            built.vertex_material[t[0] as usize],
            built.vertex_material[t[1] as usize]
        );
        assert_eq!(
            built.vertex_material[t[0] as usize],
            built.vertex_material[t[2] as usize]
        );
    }
    let channels = [
        Channel {
            tag: 1,
            encoding: E::UnormRange16x2,
            source_offset: 0,
        },
        Channel {
            tag: 2,
            encoding: E::Oct16x2,
            source_offset: 2,
        },
    ];
    let mut source = v2::Source::new(
        &built.groups,
        &built.clusters,
        &built.positions,
        &built.indices,
    );
    source.channels = &channels;
    source.attributes = Some(
        Attributes::from_interleaved(
            &built.attributes,
            built.positions.len(),
            built.attribute_stride,
            built.attribute_stride,
            0,
        )
        .unwrap(),
    );
    source.materials = Some(&built.vertex_material);
    for codec in [DiskCodec::None, DiskCodec::Compressed] {
        let mut options = v2::WriteOptions::new(grid);
        options.disk_codec = codec;
        let (file, report) = v2::write(&source, &options, &mut Workspace::default()).unwrap();
        let reader = v2::Reader::read_all(&file, &mut Workspace::default()).unwrap();
        assert_eq!(reader.attribute_count(), 2);
        assert_eq!(reader.chunk(v2::TAG_MATERIALS).unwrap().len(), 16);
        let prefix = v2::resident_len(&file).unwrap();
        let count = u32::from_le_bytes(file[12..16].try_into().unwrap()) as usize;
        let mats_entry = (0..count)
            .map(|i| v2::HEADER_BYTES + i * v2::CHUNK_ENTRY_BYTES)
            .find(|&at| &file[at..at + 4] == b"MATS")
            .unwrap();
        let mats =
            u32::from_le_bytes(file[mats_entry + 8..mats_entry + 12].try_into().unwrap()) as usize;
        for field in [0, 4, 8, 12] {
            let mut bad = file.clone();
            bad[mats + field..mats + field + 4].copy_from_slice(&u32::MAX.to_le_bytes());
            let hash = v2::prefix_hash(&bad[..prefix]).unwrap();
            bad[v2::RESIDENT_HASH_FIELD].copy_from_slice(&hash.to_le_bytes());
            assert_eq!(v2::Reader::open(&bad[..prefix]).err(), Some(Error::Invalid));
        }
        let mut unsupported = file.clone();
        unsupported[mats_entry..mats_entry + 4].copy_from_slice(b"MTS0");
        let hash = v2::prefix_hash(&unsupported[..prefix]).unwrap();
        unsupported[v2::RESIDENT_HASH_FIELD].copy_from_slice(&hash.to_le_bytes());
        assert_eq!(
            v2::Reader::open(&unsupported[..prefix]).err(),
            Some(Error::Unsupported)
        );
        let attr_entry = (0..count)
            .map(|i| v2::HEADER_BYTES + i * v2::CHUNK_ENTRY_BYTES)
            .find(|&at| &file[at..at + 4] == b"ATTR")
            .unwrap();
        let attr =
            u32::from_le_bytes(file[attr_entry + 8..attr_entry + 12].try_into().unwrap()) as usize;
        let attr_len =
            u32::from_le_bytes(file[attr_entry + 12..attr_entry + 16].try_into().unwrap()) as usize;
        let mesh_entry = (0..count)
            .map(|i| v2::HEADER_BYTES + i * v2::CHUNK_ENTRY_BYTES)
            .find(|&at| &file[at..at + 4] == b"MESH")
            .unwrap();
        let mesh_at =
            u32::from_le_bytes(file[mesh_entry + 8..mesh_entry + 12].try_into().unwrap()) as usize;
        for (field, expected) in [
            (0, Error::Limit),
            (16 + 4, Error::Unsupported),
            (16 + 8, Error::Invalid),
            (16 + 12, Error::Invalid),
        ] {
            let mut bad = file.clone();
            bad[attr + field..attr + field + 4].copy_from_slice(&u32::MAX.to_le_bytes());
            let layout = v2::xxh64(&bad[attr + 16..attr + attr_len], 0) as u32;
            bad[attr + 4..attr + 8].copy_from_slice(&layout.to_le_bytes());
            bad[mesh_at + 96..mesh_at + 100].copy_from_slice(&layout.to_le_bytes());
            let hash = v2::prefix_hash(&bad[..prefix]).unwrap();
            bad[v2::RESIDENT_HASH_FIELD].copy_from_slice(&hash.to_le_bytes());
            assert_eq!(v2::Reader::open(&bad[..prefix]).err(), Some(expected));
        }

        assert!(report.pinned_pages > 0);
        for page in 0..reader.mesh().page_count as usize {
            let pr = reader.page(page).unwrap();
            let mut words = vec![0; pr.memory_words as usize];
            let range = pr.disk_range();
            let view = reader
                .transcode_page(
                    page,
                    &file[range.start as usize..range.end as usize],
                    &mut words,
                    &mut Workspace::default(),
                )
                .unwrap();
            for c in 0..view.cluster_count() {
                let d = view.cluster(c).unwrap();
                let b = view.attributes(c).unwrap().unwrap();
                for v in 0..d.vertex_count as usize {
                    let uv = b.decode(0, v).unwrap();
                    assert!((0.0..=1.0).contains(&uv[0]));
                    assert!((0.0..=1.0).contains(&uv[1]));
                    assert_eq!(b.decode(1, v).unwrap(), [0., 0., 1., 0.]);
                }
                for t in 0..d.triangle_count as usize {
                    assert!(view.triangle_material(c, t).unwrap() < 3);
                }
            }
        }
    }
    let mut materials_only = v2::Source::new(
        &built.groups,
        &built.clusters,
        &built.positions,
        &built.indices,
    );
    materials_only.materials = Some(&built.vertex_material);
    let (file, _) = v2::write(
        &materials_only,
        &v2::WriteOptions::new(grid),
        &mut Workspace::default(),
    )
    .unwrap();
    let reader = v2::Reader::read_all(&file, &mut Workspace::default()).unwrap();
    assert_eq!(reader.attribute_count(), 0);
    assert!(reader.chunk(v2::TAG_MATERIALS).is_some());
    let options = v2::WriteOptions::new(grid);
    let mut key = v2::KeyInputs::new(&c, &options, &p, &idx);
    let original = v2::build_key(&key);
    key.measured_steps = false;
    assert_ne!(original, v2::build_key(&key));
    key.measured_steps = true;
    key.channels = &channels;
    assert_ne!(original, v2::build_key(&key));
    let ch = v2::build_key(&key);
    key.materials = Some(&slots);
    assert_ne!(ch, v2::build_key(&key));
    let mat = v2::build_key(&key);
    key.normals = Some(&normals);
    assert_ne!(mat, v2::build_key(&key));
}

#[test]
fn half_subnormals_match_ieee_attribute_vectors() {
    // The review reproduction: a single accepted smallest subnormal.
    let words = [13, 1, 1, 1, 99, 5, 0, 0, 0, 0, 0, 0, 0x0000_0001];
    assert_eq!(
        Block::new(&words, 1).unwrap().decode(0, 0).unwrap()[0].to_bits(),
        0x3380_0000
    );
    let words: Vec<u32> = include_str!("../docs/attribute_vectors.txt")
        .split("# Separate F16x2 block,")
        .nth(1)
        .unwrap()
        .lines()
        .filter_map(|line| line.strip_prefix("# "))
        .take_while(|line| !line.starts_with("Expected"))
        .flat_map(str::split_whitespace)
        .map(|word| u32::from_str_radix(word, 16).unwrap())
        .collect();
    let block = Block::new(&words, 2).unwrap();
    assert_eq!(
        block.decode(0, 0).unwrap().map(f32::to_bits),
        [0x3380_0000, 0x387f_c000, 0, 0]
    );
    assert_eq!(
        block.decode(0, 1).unwrap().map(f32::to_bits),
        [0xb380_0000, 0xb87f_c000, 0, 0]
    );
    // Exhaust the accepted half domain, including signed zeros, for both layouts.
    for encoding in [5, 6] {
        for half in 0..=u16::MAX {
            if half & 0x7c00 == 0x7c00 {
                continue;
            }
            let mut words = vec![
                14,
                1,
                2,
                1,
                99,
                encoding,
                0,
                0,
                0,
                0,
                0,
                0,
                u32::from(half) * 0x10001,
                0,
            ];
            if encoding == 5 {
                words.truncate(13);
                words[0] = 13;
                words[2] = 1;
            }
            let got = Block::new(&words, 1).unwrap().decode(0, 0).unwrap()[0];
            let sign = u32::from(half & 0x8000) << 16;
            let exp = (half >> 10) & 31;
            let expected = if exp == 0 {
                (f32::from(half & 1023) * 2f32.powi(-24)).to_bits() | sign
            } else {
                sign | ((u32::from(exp) + 112) << 23) | (u32::from(half & 1023) << 13)
            };
            assert_eq!(got.to_bits(), expected, "{half:04x}");
        }
    }
}
