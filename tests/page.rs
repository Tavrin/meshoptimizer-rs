use meshoptimizer_rs::{
    clusterlod::dag_format::Error, clusterlod::page::*, compute_cluster_bounds, Positions,
    Workspace,
};

/// Height field on an `n` x `n` quad grid, periodic in x so copies tile by translation.
fn height(i: usize, j: usize, n: usize) -> f32 {
    ((i % n) * 7 + j * 13 % 11) as f32 * 0.093_1 + (j as f32) * 0.011_7
}

fn vertex(i: usize, j: usize, n: usize) -> [f32; 3] {
    [
        i as f32 * 0.371_3,
        height(i, j, n),
        j as f32 * 0.290_9 - 0.77,
    ]
}

/// Square blocks of `b` x `b` quads covering an `n` x `n` grid; neighbours share
/// boundary vertices. Each block: local positions, local indices, global ids.
fn blocks(n: usize, b: usize, map: impl Fn([f32; 3]) -> [f32; 3]) -> Vec<Block> {
    let mut out = Vec::new();
    for bi in (0..n).step_by(b) {
        for bj in (0..n).step_by(b) {
            let (mut pos, mut ids, mut idx) = (Vec::new(), Vec::new(), Vec::new());
            let local = |i: usize, j: usize, pos: &mut Vec<[f32; 3]>, ids: &mut Vec<usize>| {
                let id = i * (n + 1) + j;
                match ids.iter().position(|&v| v == id) {
                    Some(k) => k as u32,
                    None => {
                        ids.push(id);
                        pos.push(map(vertex(i, j, n)));
                        (ids.len() - 1) as u32
                    }
                }
            };
            for i in bi..bi + b {
                for j in bj..bj + b {
                    let a = local(i, j, &mut pos, &mut ids);
                    let c = local(i + 1, j, &mut pos, &mut ids);
                    let d = local(i + 1, j + 1, &mut pos, &mut ids);
                    let e = local(i, j + 1, &mut pos, &mut ids);
                    idx.extend([a, c, d, a, d, e]);
                }
            }
            out.push((pos, idx, ids));
        }
    }
    out
}

type Block = (Vec<[f32; 3]>, Vec<u32>, Vec<usize>);

fn encode(grid: QuantGrid, blocks: &[Block]) -> Vec<u32> {
    let clusters: Vec<_> = blocks
        .iter()
        .enumerate()
        .map(|(c, (p, i, _))| PageCluster::new(c as u32, -1, p, i))
        .collect();
    encode_page(grid, &clusters, 0x5eed, &mut Workspace::default()).unwrap()
}

/// Rotate a triangle so its smallest index comes first (winding preserved).
fn canonical(t: [u32; 3]) -> [u32; 3] {
    let r = (0..3).min_by_key(|&k| t[k]).unwrap();
    [t[r], t[(r + 1) % 3], t[(r + 2) % 3]]
}

#[test]
fn grid_contract() {
    let grid = QuantGrid::new(-10).unwrap();
    assert_eq!(grid.step(), 1. / 1024.);
    let q = grid.max_displacement();
    assert!(f64::from(q) >= f64::from(grid.step()) * 3f64.sqrt() / 2.);
    assert!(q < grid.step());
    assert_eq!(QuantGrid::from_max_error(q).unwrap(), grid);
    assert_eq!(
        QuantGrid::from_max_error(q.next_down()).unwrap().exponent(),
        -11
    );
    assert_eq!(QuantGrid::new(-127), Err(Error::Invalid));
    let g = QuantGrid::from_bounds([-3., 0., 0.], [5., 1., 1.], 16).unwrap();
    assert!(g
        .snap([5., 1., 1.])
        .unwrap()
        .iter()
        .all(|v| v.unsigned_abs() <= 1 << 15));

    let mut positions: Vec<_> = (0..=8)
        .flat_map(|i| (0..=8).map(move |j| vertex(i, j, 8)))
        .collect();
    let source = positions.clone();
    let worst = grid.snap_positions(&mut positions).unwrap();
    assert!(worst > 0. && worst <= q);
    for (p, s) in positions.iter().zip(&source) {
        assert_eq!(*p, grid.position(grid.snap(*s).unwrap()));
        assert_eq!(
            grid.snap(*p).unwrap(),
            grid.snap(*s).unwrap(),
            "snapping is idempotent"
        );
    }
    assert_eq!(grid.snap_coordinate(f32::NAN), Err(Error::Invalid));
    assert_eq!(grid.snap_coordinate(8192.), Err(Error::Limit));
    let mut bad = [[0.; 3], [f32::INFINITY, 0., 0.]];
    assert!(grid.snap_positions(&mut bad).is_err());
    assert_eq!(bad[0], [0.; 3], "nothing written on failure");
}

#[test]
fn memory_form_round_trip_and_seams() {
    let n = 16;
    let grid = QuantGrid::new(-12).unwrap();
    let parts = blocks(n, 8, |p| p);
    let page = encode(grid, &parts);
    let view = PageView::new(&page, grid).unwrap();
    assert_eq!(view.cluster_count(), 4);
    assert_eq!(view.attribute_layout_hash(), 0x5eed);
    let mut ws = Workspace::default();
    let mut shared = std::collections::HashMap::new();
    for (c, (pos, idx, ids)) in parts.iter().enumerate() {
        let d = view.cluster(c).unwrap();
        assert_eq!(
            (d.group, d.refined, d.vertex_count, d.triangle_count),
            (c as u32, -1, 81, 128)
        );
        let mut p = [[0f32; 3]; 256];
        let mut t = [[0u8; 3]; 256];
        assert_eq!(view.decode_positions(c, &mut p).unwrap(), 81);
        assert_eq!(view.decode_triangles(c, &mut t).unwrap(), 128);
        assert_eq!(view.decode_positions(c, &mut p[..80]), Err(Error::Limit));
        for (v, s) in pos.iter().enumerate() {
            assert_eq!(
                p[v],
                grid.position(grid.snap(*s).unwrap()),
                "decode equals snapped source"
            );
            let prev = shared.insert(ids[v], p[v].map(f32::to_bits));
            assert!(
                prev.is_none_or(|b| b == p[v].map(f32::to_bits)),
                "shared vertex decodes identically"
            );
            let dist = (0..3)
                .map(|k| f64::from(p[v][k] - d.center[k]).powi(2))
                .sum::<f64>()
                .sqrt();
            assert!(dist <= f64::from(d.radius));
        }
        let mut got: Vec<_> = t[..128]
            .iter()
            .map(|t| canonical(t.map(u32::from)))
            .collect();
        let mut want: Vec<_> = idx
            .chunks(3)
            .map(|t| canonical([t[0], t[1], t[2]]))
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "same triangles and winding");
        let flat: Vec<u32> = t[..128].iter().flat_map(|t| t.map(u32::from)).collect();
        let b = compute_cluster_bounds(&flat, Positions::from_packed(&p[..81]), &mut ws).unwrap();
        assert_eq!(d.cone_axis_s8, b.cone_axis_s8);
        assert_eq!(d.cone_cutoff_s8, b.cone_cutoff_s8);
        assert_eq!(d.cone_valid, b.cone_cutoff_s8 != 127);
    }
    // A copy placed by a whole number of grid steps meets the original exactly.
    let mut snapped = vec![vertex(n, 0, n), vertex(0, 0, n)];
    grid.snap_positions(&mut snapped).unwrap();
    let shift = snapped[0][0] - snapped[1][0];
    let copy = blocks(n, 8, |p| {
        let s = grid.position(grid.snap(p).unwrap());
        [s[0] + shift, s[1], s[2]]
    });
    let copy_page = encode(grid, &copy);
    let copy_view = PageView::new(&copy_page, grid).unwrap();
    let mut met = 0;
    for (c, (_, _, ids)) in copy.iter().enumerate() {
        let mut p = [[0f32; 3]; 256];
        copy_view.decode_positions(c, &mut p).unwrap();
        for (v, &id) in ids.iter().enumerate() {
            if id / (n + 1) == 0 {
                let right = shared[&(n * (n + 1) + id % (n + 1))];
                assert_eq!(p[v].map(f32::to_bits), right, "modular seam");
                met += 1;
            }
        }
    }
    assert_eq!(met, n + 2, "both left-edge blocks share the middle vertex");
}

#[test]
fn rejects_invalid_clusters_and_pages() {
    let grid = QuantGrid::new(-8).unwrap();
    let p = [[0.; 3], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let mut ws = Workspace::default();
    let bad = |pos: &[[f32; 3]], idx: &[u32], refined: i32, ws: &mut Workspace| {
        encode_page(grid, &[PageCluster::new(0, refined, pos, idx)], 0, ws)
    };
    assert_eq!(
        bad(&p, &[0, 1, 2], 0, &mut ws),
        Err(Error::Invalid),
        "unused vertex"
    );
    assert_eq!(
        bad(&p, &[0, 1, 4, 1, 2, 3], 0, &mut ws),
        Err(Error::Invalid),
        "index range"
    );
    assert_eq!(
        bad(&p, &[0, 1, 2, 1, 3], 0, &mut ws),
        Err(Error::Invalid),
        "partial triangle"
    );
    assert_eq!(
        bad(&p, &[0, 1, 2, 1, 3, 2], -2, &mut ws),
        Err(Error::Invalid),
        "refined"
    );
    assert_eq!(
        bad(&[[1e9, 0., 0.], p[1], p[2]], &[0, 1, 2], 0, &mut ws),
        Err(Error::Limit)
    );
    assert_eq!(encode_page(grid, &[], 0, &mut ws), Err(Error::Invalid));
    let page = bad(&p, &[0, 1, 2, 1, 3, 2], 0, &mut ws).unwrap();
    assert!(PageView::new(&page, grid).is_ok());
    for at in 0..page.len() {
        for flip in [1u32, 1 << 9, 1 << 23, 1 << 31] {
            let mut m = page.clone();
            m[at] ^= flip;
            if let Ok(v) = PageView::new(&m, grid) {
                // Accepted mutations still decode within bounds.
                assert!(at != 3 && at != 1, "word {at}");
                let mut t = [[0u8; 3]; 256];
                assert_eq!(v.decode_triangles(0, &mut t).unwrap(), 2);
                assert!(t[..2].iter().flatten().all(|&i| i < 4));
            }
        }
    }
    assert!(PageView::new(&page[..page.len() - 1], grid).is_err());
}

fn fixture() -> (QuantGrid, Vec<u32>) {
    let grid = QuantGrid::new(-8).unwrap();
    let parts = blocks(4, 2, |p| p);
    (grid, encode(grid, &parts[..2]))
}

#[test]
fn disk_forms_transcode_exactly() {
    let mut ws = Workspace::default();
    let grid = QuantGrid::new(-12).unwrap();
    let page = encode(grid, &blocks(16, 8, |p| p));
    for codec in [DiskCodec::None, DiskCodec::Compressed] {
        let disk = encode_disk_page(&page, grid, codec, &mut ws).unwrap();
        assert_eq!(disk_page_memory_words(&disk).unwrap(), page.len());
        let mut out = vec![0xdead_beef; page.len() + 3];
        assert_eq!(
            transcode_page(&disk, grid, &mut out, &mut ws).unwrap(),
            page.len()
        );
        assert_eq!(out[..page.len()], page[..]);
        assert_eq!(
            transcode_page(&disk, grid, &mut out[..page.len() - 1], &mut ws),
            Err(Error::Limit)
        );
        if codec == DiskCodec::Compressed {
            assert!(
                disk.len() < page.len() * 4,
                "{} >= {}",
                disk.len(),
                page.len() * 4
            );
        }
    }
    let (grid, page) = fixture();
    for codec in [DiskCodec::None, DiskCodec::Compressed] {
        let disk = encode_disk_page(&page, grid, codec, &mut ws).unwrap();
        let mut out = vec![0; page.len()];
        for end in 0..disk.len() {
            assert!(transcode_page(&disk[..end], grid, &mut out, &mut ws).is_err());
        }
        for at in 0..disk.len() {
            for x in [1u8, 0x10, 0x80, 0xff] {
                let mut m = disk.clone();
                m[at] ^= x;
                let mut out = vec![0; MAX_PAGE_WORDS.min(page.len() * 4)];
                if let Ok(words) = transcode_page(&m, grid, &mut out, &mut ws) {
                    assert!(PageView::new(&out[..words], grid).is_ok());
                }
            }
        }
    }
}

fn hex_lines(text: &mut String, label: &str, items: &[String], per_line: usize) {
    text.push_str(&format!("{label} {}\n", items.len()));
    for line in items.chunks(per_line) {
        text.push_str(&line.join(" "));
        text.push('\n');
    }
}

fn vectors() -> String {
    let (grid, page) = fixture();
    let disk = encode_disk_page(
        &page,
        grid,
        DiskCodec::Compressed,
        &mut Workspace::default(),
    )
    .unwrap();
    let view = PageView::new(&page, grid).unwrap();
    let mut decoded = Vec::new();
    for c in 0..view.cluster_count() {
        let mut p = [[0f32; 3]; 256];
        let mut t = [[0u8; 3]; 256];
        let nv = view.decode_positions(c, &mut p).unwrap();
        let nt = view.decode_triangles(c, &mut t).unwrap();
        decoded.extend(p[..nv].iter().flat_map(|p| p.map(f32::to_bits)));
        decoded.extend(t[..nt].iter().flat_map(|t| t.map(u32::from)));
    }
    let words = |w: &[u32]| w.iter().map(|w| format!("{w:08x}")).collect::<Vec<_>>();
    let mut text = String::from(
        "# Page memory-form test vectors (docs/PAGE_FORMAT.md). Generated by tests/page.rs;\n\
         # regenerate with PAGE_VECTORS_PATH=docs/page_vectors.txt cargo test --test page.\n",
    );
    text.push_str(&format!("exponent {}\n", grid.exponent()));
    hex_lines(&mut text, "memory", &words(&page), 8);
    hex_lines(
        &mut text,
        "disk",
        &disk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>(),
        32,
    );
    hex_lines(&mut text, "decoded", &words(&decoded), 8);
    text
}

#[test]
fn reference_vectors_and_wgsl() {
    let root = env!("CARGO_MANIFEST_DIR");
    let text = vectors();
    if let Ok(path) = std::env::var("PAGE_VECTORS_PATH") {
        std::fs::write(path, &text).unwrap();
    }
    let stored = std::fs::read_to_string(format!("{root}/docs/page_vectors.txt")).unwrap();
    assert_eq!(
        stored, text,
        "encoder output changed; regenerate the vectors deliberately"
    );
    // The stored memory words decode to the stored output through the CPU decoder,
    // and the stored disk bytes transcode to the stored memory words.
    let section = |label: &str| -> Vec<u32> {
        let mut lines = stored.lines().skip_while(|l| !l.starts_with(label));
        let count: usize = lines.next().unwrap()[label.len() + 1..].parse().unwrap();
        let radix = if label == "disk" { 2 } else { 8 };
        let values: Vec<u32> = lines
            .take(count.div_ceil(if radix == 2 { 32 } else { 8 }))
            .flat_map(|l| {
                l.split(' ')
                    .map(|h| u32::from_str_radix(h, 16).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len(), count);
        values
    };
    let grid = QuantGrid::new(-8).unwrap();
    let memory = section("memory");
    let view = PageView::new(&memory, grid).unwrap();
    let disk: Vec<u8> = section("disk").into_iter().map(|b| b as u8).collect();
    let mut out = vec![0; memory.len()];
    transcode_page(&disk, grid, &mut out, &mut Workspace::default()).unwrap();
    assert_eq!(out, memory);
    let mut decoded = Vec::new();
    for c in 0..view.cluster_count() {
        let mut p = [[0f32; 3]; 256];
        let mut t = [[0u8; 3]; 256];
        let nv = view.decode_positions(c, &mut p).unwrap();
        let nt = view.decode_triangles(c, &mut t).unwrap();
        decoded.extend(p[..nv].iter().flat_map(|p| p.map(f32::to_bits)));
        decoded.extend(t[..nt].iter().flat_map(|t| t.map(u32::from)));
    }
    assert_eq!(decoded, section("decoded"));
}

#[cfg(feature = "clusterlod")]
#[test]
fn snapped_dag_decodes_shared_vertices_identically() {
    use meshoptimizer_rs::clusterlod::*;
    let n = 24;
    let grid = QuantGrid::new(-10).unwrap();
    let mut positions: Vec<_> = (0..=n)
        .flat_map(|i| (0..=n).map(move |j| vertex(i, j, n)))
        .collect();
    let q = grid.snap_positions(&mut positions).unwrap();
    assert!(q <= grid.max_displacement());
    let mut indices = Vec::new();
    for i in 0..n as u32 {
        for j in 0..n as u32 {
            let v = |a: u32, b: u32| a * (n as u32 + 1) + b;
            indices.extend([
                v(i, j),
                v(i + 1, j),
                v(i + 1, j + 1),
                v(i, j),
                v(i + 1, j + 1),
                v(i, j + 1),
            ]);
        }
    }
    let ni = indices.len();
    let (mut source, mut vp, mut ix) = (vec![0; ni * 8], vec![[0.; 3]; ni * 8], vec![0; ni * 8]);
    let mut cl = vec![FlatDagCluster::default(); ni / 3];
    let mut gr = vec![FlatDagGroup::default(); ni / 3];
    let mut nodes = vec![Node::default(); ni / 3 + 128];
    let mut ws = Workspace::default();
    let counts = build_dag_into(
        default_config(64).unwrap(),
        Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        8,
        DagBuffers {
            vertex_source: &mut source,
            vertex_positions: &mut vp,
            indices: &mut ix,
            clusters: &mut cl,
            groups: &mut gr,
            nodes: &mut nodes,
        },
        &mut ws,
    )
    .unwrap();
    assert!(
        gr[..counts.groups].iter().any(|g| g.depth > 0),
        "fixture must simplify"
    );
    let mut decoded = std::collections::HashMap::new();
    let mut checked = 0;
    for c in &cl[..counts.clusters] {
        let tri = &ix[c.first_index as usize..][..c.index_count as usize];
        let (mut local, mut ids, mut pos) = (Vec::new(), Vec::new(), Vec::new());
        for &v in tri {
            let k = ids.iter().position(|&x| x == v).unwrap_or_else(|| {
                ids.push(v);
                pos.push(vp[v as usize]);
                ids.len() - 1
            });
            local.push(k as u32);
        }
        let page = encode_page(
            grid,
            &[PageCluster::new(c.group, c.refined, &pos, &local)],
            0,
            &mut ws,
        )
        .unwrap();
        let view = PageView::new(&page, grid).unwrap();
        let mut p = [[0f32; 3]; 256];
        view.decode_positions(0, &mut p).unwrap();
        for (k, &v) in ids.iter().enumerate() {
            let key = source[v as usize];
            let bits = p[k].map(f32::to_bits);
            let prev = decoded.insert(key, bits);
            if prev.is_some() {
                checked += 1;
            }
            assert!(
                prev.is_none_or(|b| b == bits),
                "source vertex {key} decodes identically in every cluster and level"
            );
        }
    }
    assert!(
        checked > counts.clusters,
        "vertices are shared across clusters"
    );
}
