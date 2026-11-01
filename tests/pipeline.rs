use meshoptimizer_rs::*;

fn pos(v: &[u8], stride: usize) -> Positions<'_> {
    Positions::from_bytes(v, v.len() / stride, stride, 0, ByteOrder::LittleEndian).unwrap()
}
fn bytes(p: &[[f32; 3]]) -> Vec<u8> {
    p.iter().flatten().flat_map(|x| x.to_le_bytes()).collect()
}

// Independent public-API sequence. Also used by the retained qualification driver.
pub fn qualification_workspace() -> Workspace {
    Workspace::new(Limits {
        max_bytes: usize::MAX,
        max_work: u64::MAX,
    })
}
pub fn equivalent(ix: &[u32], input: &[u8], stride: usize, o: &ProcessOptions<'_>) {
    let actual = process_mesh(ix, input, stride, 0, o, &mut qualification_workspace()).unwrap();
    let manual = manual(ix, input, stride, o);
    assert_eq!(actual.levels.len(), manual.len());
    for (level, (expected, quality)) in actual.levels.iter().zip(manual) {
        assert_eq!(level.indices, expected.0);
        assert_eq!(level.vertices, expected.1);
        assert_eq!(format!("{:?}", level.meshlets), expected.2);
        assert_eq!(level.encoded_indices, expected.3);
        assert_eq!(level.encoded_vertices, expected.4);
        let i = actual
            .levels
            .iter()
            .position(|p| core::ptr::eq(p, level))
            .unwrap();
        assert_eq!(actual.report.quality[i], quality);
    }
    assert_eq!(
        actual.report.stages.iter().map(|s| s.work).sum::<u64>(),
        actual.report.usage.work
    );
}
type Buffers = (Vec<u32>, Vec<u8>, String, Option<Vec<u8>>, Option<Vec<u8>>);
pub fn manual(
    ix: &[u32],
    input: &[u8],
    stride: usize,
    o: &ProcessOptions<'_>,
) -> Vec<(Buffers, Option<LodQuality>)> {
    let mut ws = qualification_workspace();

    let mut ix = o.sanitize.map_or_else(
        || ix.to_vec(),
        |s| {
            sanitize(ix, pos(input, stride), s, &mut ws)
                .unwrap()
                .indices
        },
    );
    let mut v = input.to_vec();
    if o.weld {
        let r = generate_vertex_remap(
            Some(&ix),
            VertexStream::packed(&v, stride).unwrap(),
            &mut ws,
        )
        .unwrap();
        ix = remap_index_buffer(Some(&ix), &r.remap, &mut ws).unwrap();
        v = remap_vertex_buffer(VertexStream::packed(&v, stride).unwrap(), &r.remap, &mut ws)
            .unwrap();
    }
    if o.orient {
        ix = orient_faces(&ix, v.len() / stride, &mut ws)
            .unwrap()
            .indices;
    }
    if let Some(n) = o.fill_holes {
        ix = fill_holes(&ix, pos(&v, stride), n, &mut ws)
            .unwrap()
            .indices;
    }
    let mut lists = vec![ix.clone()];
    if let Some(mut l) = o.lod {
        if let Some((a, w)) = o.attributes {
            l.attributes = Some(LodChainAttributes::new(
                Attributes::from_bytes(
                    &v,
                    v.len() / stride,
                    w.len(),
                    stride,
                    a,
                    ByteOrder::LittleEndian,
                )
                .unwrap(),
                w,
            ));
        }
        let chain = if let Some(s) = o.skin {
            let mut joints = Vec::new();
            let mut weights = Vec::new();
            for r in v.chunks_exact(stride) {
                for i in 0..s.influences {
                    let a = s.joints_offset + i * 4;
                    let b = s.weights_offset + i * 4;
                    joints.push(u32::from_le_bytes(r[a..a + 4].try_into().unwrap()));
                    weights.push(f32::from_le_bytes(r[b..b + 4].try_into().unwrap()));
                }
            }
            let mut d = CharacterInputs::new();
            d.skin = Some(CharacterSkin::new(
                &joints,
                &weights,
                s.influences,
                s.importance,
            ));
            build_character_lod_chain(
                &ix,
                pos(&v, stride),
                &d,
                &s.options,
                &l,
                &o.quality,
                &mut ws,
            )
            .unwrap()
            .chain
        } else {
            build_lod_chain(&ix, pos(&v, stride), &l, &mut ws).unwrap()
        };
        lists.extend(chain.levels.into_iter().map(|l| l.indices));
    }
    lists
        .into_iter()
        .map(|mut indices| {
            let mut vertices = v.clone();
            if o.cache {
                indices =
                    optimize_vertex_cache(&indices, vertices.len() / stride, &mut ws).unwrap();
            }
            if let Some(t) = o.overdraw {
                indices = optimize_overdraw(&indices, pos(&vertices, stride), t, &mut ws).unwrap();
            }
            if o.fetch {
                let f = optimize_vertex_fetch(
                    &indices,
                    VertexStream::packed(&vertices, stride).unwrap(),
                    &mut ws,
                )
                .unwrap();
                indices = f.indices;
                vertices = f.vertices;
            }
            let q = if ix.is_empty() || indices.is_empty() {
                None
            } else {
                Some(
                    measure_lod_quality(
                        LodQualityMesh::new(&ix, pos(&v, stride)),
                        LodQualityMesh::new(&indices, pos(&vertices, stride)),
                        &o.quality,
                        &mut ws,
                    )
                    .unwrap(),
                )
            };
            let m = o.meshlets.map(|(s, c)| {
                build_meshlets(&indices, pos(&vertices, stride), s, c, &mut ws).unwrap()
            });
            let ei = o
                .index_encoding
                .map(|e| codec::encode_index_buffer(&indices, e, &mut ws).unwrap());
            let ev = o.vertex_encoding.map(|e| {
                codec::encode_vertex_buffer(&vertices, vertices.len() / stride, stride, e, &mut ws)
                    .unwrap()
            });
            ((indices, vertices, format!("{m:?}"), ei, ev), q)
        })
        .collect()
}

pub fn grid(n: u32) -> (Vec<u32>, Vec<u8>) {
    let p: Vec<_> = (0..=n)
        .flat_map(|y| (0..=n).map(move |x| [x as f32, y as f32, ((x + y) % 3) as f32 * 0.01]))
        .collect();
    let ix = (0..n)
        .flat_map(|y| {
            (0..n).flat_map(move |x| {
                let a = y * (n + 1) + x;
                [a, a + 1, a + n + 1, a + 1, a + n + 2, a + n + 1]
            })
        })
        .collect();
    (ix, bytes(&p))
}
#[test]
fn sequence_corpus() {
    for n in 1..=6 {
        let (mut ix, v) = grid(n);
        ix.extend_from_within(..3);
        ix.extend_from_slice(&[u32::MAX, 0, 1, 0, 0, 0]);
        let mut o = ProcessOptions::default();
        o.meshlets = Some((MeshletSettings::default(), 0.25));
        o.index_encoding = Some(codec::IndexEncoding::DEFAULT);
        o.vertex_encoding = Some(codec::VertexEncoding::DEFAULT);
        equivalent(&ix, &v, 12, &o);
        o.orient = true;
        o.fill_holes = Some(4);
        equivalent(&ix, &v, 12, &o);
        o.lod = None;
        o.weld = false;
        o.fetch = false;
        o.cache = false;
        o.overdraw = None;
        equivalent(&ix, &v, 12, &o);
    }
}
#[test]
fn skin_and_attribute_seams() {
    let (mut ix, v) = grid(3);
    let mut records = Vec::new();
    for (i, p) in v.as_chunks::<12>().0.iter().enumerate() {
        records.extend_from_slice(p);
        for j in 0..4u32 {
            records.extend_from_slice(
                &(if j == 0 {
                    u32::try_from(i % 2).unwrap()
                } else {
                    0
                })
                .to_le_bytes(),
            );
        }
        for j in 0..4 {
            records.extend_from_slice(&(if j == 0 { 1f32 } else { 0f32 }).to_le_bytes());
        }
    }
    let seam_id = u32::try_from(records.len() / 44).unwrap();
    let mut seam = records[..44].to_vec();
    seam[12..16].copy_from_slice(&1u32.to_le_bytes());
    seam[28..32].copy_from_slice(&0.5f32.to_le_bytes());
    seam[32..36].copy_from_slice(&0.5f32.to_le_bytes());
    records.extend_from_slice(&seam);
    ix.extend_from_slice(&[seam_id, 1, 4]);
    let mut o = ProcessOptions::default();
    o.skin = Some(ProcessSkin::new(12, 28, 4, &[1., 2.]));
    equivalent(&ix, &records, 44, &o);
    o.skin = None;
    o.attributes = Some((28, &[1., 1., 1., 1.]));
    equivalent(&ix, &records, 44, &o);
}
#[test]
fn budgets_errors_and_empty() {
    let (ix, v) = grid(2);
    let o = ProcessOptions::default();
    let mut ws = Workspace::default();
    let a = process_mesh(&ix, &v, 12, 0, &o, &mut ws).unwrap();
    let usage = ws.usage();
    assert_eq!(a.report.usage, usage);
    for limits in [
        Limits {
            max_work: 0,
            max_bytes: usize::MAX,
        },
        Limits {
            max_work: u64::MAX,
            max_bytes: 0,
        },
        Limits {
            max_work: usage.work - 1,
            max_bytes: usize::MAX,
        },
    ] {
        ws.set_limits(limits);
        assert_eq!(
            process_mesh(&ix, &v, 12, 0, &o, &mut ws).unwrap_err().error,
            Error::LimitExceeded
        );
        assert_eq!(ws.limits(), limits);
    }
    ws = Workspace::default();
    assert_eq!(
        process_mesh(&ix, &v, 0, 0, &o, &mut ws).unwrap_err().stage,
        ProcessStage::Validate
    );
    let mut o = o;
    o.lod = None;
    o.sanitize = None;
    assert_eq!(
        process_mesh(&[999, 0, 1], &v, 12, 0, &o, &mut ws)
            .unwrap_err()
            .error,
        Error::IndexOutOfBounds
    );
    o.sanitize = Some(SanitizeOptions::default());
    equivalent(&[], &[], 12, &o);
}

#[test]
#[cfg(feature = "clusterlod")]
fn cluster_equivalence() {
    let (ix, v) = grid(8);
    let mut o = ProcessOptions::default();
    o.lod = None;
    let mut config = clusterlod::default_config(32).unwrap();
    config.simplify_dilate_borders = true;
    o.clusters = Some(config);
    let a = process_mesh(&ix, &v, 12, 0, &o, &mut Workspace::default()).unwrap();
    let l = &a.levels[0];
    let view = pos(&l.vertices, 12);
    let mut p: Vec<_> = (0..view.len()).map(|i| view.get(i).unwrap()).collect();
    let b = clusterlod::build(
        config,
        clusterlod::Mesh {
            indices: &l.indices,
            positions: &mut p,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    let clusters = a.levels[0].clusters.as_ref().unwrap();
    assert_eq!(clusters.groups, b);
    assert_eq!(clusters.positions, p);
}

#[test]
fn nonzero_position_offset_preserves_opaque_fields() {
    let p = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let marker = f32::NAN.to_le_bytes();
    let mut v = Vec::new();
    for position in p {
        v.extend_from_slice(&marker);
        v.extend(position.into_iter().flat_map(f32::to_le_bytes));
    }
    let mut options = ProcessOptions::default();
    options.meshlets = Some((MeshletSettings::default(), 0.25));
    options.vertex_encoding = Some(codec::VertexEncoding::DEFAULT);
    let mesh = process_mesh(&[0, 1, 2], &v, 16, 4, &options, &mut Workspace::default()).unwrap();
    assert_eq!(mesh.levels[0].indices.len(), 3);
    assert!(mesh.report.quality[0].unwrap().symmetric.max < 1e-6);
    for level in &mesh.levels {
        for record in level.vertices.as_chunks::<16>().0 {
            assert_eq!(&record[..4], &marker);
        }
    }
}
