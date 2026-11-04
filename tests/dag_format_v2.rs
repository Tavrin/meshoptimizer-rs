#![cfg(feature = "clusterlod")]
#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::clusterlod::{
    self,
    cut::{self, CutError, CutView, Projection, Threshold},
    dag_format::{v2, Error},
    page::{DiskCodec, PageView, QuantGrid},
    DagBuffers, FlatDagCluster, FlatDagGroup, LodBounds, Mesh, Node,
};
use meshoptimizer_rs::Workspace;

const GRID: i32 = -8;

struct Built {
    groups: Vec<FlatDagGroup>,
    clusters: Vec<FlatDagCluster>,
    snapshot: Vec<[f32; 3]>,
    flat: Vec<u32>,
}

fn grid(n: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..(n + 1) * (n + 1))
        .map(|i| {
            let (x, y) = ((i % (n + 1)) as f32, (i / (n + 1)) as f32);
            [x, y, (x * 0.3).sin() + (y * 0.2).cos()]
        })
        .collect();
    let mut idx = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            idx.extend([a, a + 1, a + n + 1, a + n + 1, a + 1, a + n + 2]);
        }
    }
    (p, idx)
}
fn sphere(rows: u32, cols: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = Vec::new();
    for r in 0..=rows {
        let t = r as f32 * core::f32::consts::PI / rows as f32;
        for c in 0..cols {
            let a = c as f32 * core::f32::consts::TAU / cols as f32;
            p.push([
                10. * t.sin() * a.cos(),
                10. * t.cos(),
                10. * t.sin() * a.sin(),
            ]);
        }
    }
    let mut idx = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let (a, b) = (r * cols + c, r * cols + (c + 1) % cols);
            idx.extend([a, a + cols, b, b, a + cols, b + cols]);
        }
    }
    (p, idx)
}

fn build((mut positions, indices): (Vec<[f32; 3]>, Vec<u32>)) -> Built {
    QuantGrid::new(GRID)
        .unwrap()
        .snap_positions(&mut positions)
        .unwrap();
    let mut node = Node::default();
    node.group = -1;
    let (mut source, mut snapshot, mut flat) =
        (vec![0; 1 << 16], vec![[0.; 3]; 1 << 16], vec![0; 1 << 17]);
    let (mut clusters, mut groups) = (
        vec![FlatDagCluster::default(); 1 << 12],
        vec![FlatDagGroup::default(); 1 << 12],
    );
    let mut nodes = vec![node; 1 << 12];
    let counts = clusterlod::build_dag_into(
        clusterlod::default_config(16).unwrap(),
        Mesh {
            indices: &indices,
            positions: &mut positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        4,
        DagBuffers {
            vertex_source: &mut source,
            vertex_positions: &mut snapshot,
            indices: &mut flat,
            clusters: &mut clusters,
            groups: &mut groups,
            nodes: &mut nodes,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    groups.truncate(counts.groups);
    clusters.truncate(counts.clusters);
    snapshot.truncate(counts.vertices);
    flat.truncate(counts.indices);
    Built {
        groups,
        clusters,
        snapshot,
        flat,
    }
}

fn write(b: &Built, codec: DiskCodec, capacity: usize) -> (Vec<u8>, v2::WriteReport) {
    let mut o = v2::WriteOptions::new(QuantGrid::new(GRID).unwrap());
    o.disk_codec = codec;
    o.page_capacity = capacity;
    let source = v2::Source::new(&b.groups, &b.clusters, &b.snapshot, &b.flat);
    v2::write(&source, &o, &mut Workspace::default()).unwrap()
}

/// Memory form of every page, transcoded from the file's own bytes.
fn transcode_all(r: &v2::Reader, file: &[u8]) -> Vec<Vec<u32>> {
    (0..r.mesh().page_count as usize)
        .map(|p| {
            let rec = r.page(p).unwrap();
            let disk = &file[rec.disk_range().start as usize..rec.disk_range().end as usize];
            let mut out = vec![0; rec.memory_words as usize];
            r.transcode_page(p, disk, &mut out, &mut Workspace::default())
                .unwrap();
            assert!(r
                .transcode_page(p, &disk[1..], &mut out, &mut Workspace::default())
                .is_err());
            out
        })
        .collect()
}

/// Fine-to-coarse tables rebuilt from the file, the form `clusterlod::cut` takes.
fn fine_tables(r: &v2::Reader, pages: &[Vec<u32>]) -> (Vec<FlatDagGroup>, Vec<FlatDagCluster>) {
    let grid = r.mesh().grid;
    let mut refined = Vec::new();
    for words in pages {
        let view = PageView::new(words, grid).unwrap();
        refined.extend((0..view.cluster_count()).map(|k| view.cluster(k).unwrap().refined));
    }
    let n = r.mesh().group_count as usize;
    let (mut groups, mut clusters) = (Vec::new(), Vec::new());
    for i in 0..n {
        let rec = r.group(n - 1 - i).unwrap();
        let mut g = FlatDagGroup::default();
        (g.center, g.radius, g.error, g.depth) =
            (rec.center, rec.radius, rec.error, rec.depth as i32);
        (g.first_cluster, g.cluster_count) = (clusters.len() as u32, rec.cluster_count);
        for c in rec.clusters() {
            let mut cl = FlatDagCluster::default();
            cl.group = i as u32;
            let q = refined[c as usize];
            cl.refined = if q < 0 {
                -1
            } else {
                (n - 1 - q as usize) as i32
            };
            clusters.push(cl);
        }
        groups.push(g);
    }
    (groups, clusters)
}

fn canonical(tris: impl Iterator<Item = [[u32; 3]; 3]>) -> Vec<[[u32; 3]; 3]> {
    let mut v: Vec<_> = tris
        .map(|t| {
            // Smallest rotation, so equal positions (pole vertices) rotate alike.
            (0..3)
                .map(|k| [t[k], t[(k + 1) % 3], t[(k + 2) % 3]])
                .min()
                .unwrap()
        })
        .collect();
    v.sort();
    v
}

#[test]
fn xxh64_vectors_and_build_key() {
    for (input, hash) in [
        (&b""[..], 0xef46_db37_51d8_e999u64),
        (b"a", 0xd24e_c4f1_a98c_6e5b),
        (b"as", 0x1c33_0fb2_d66b_e179),
        (b"asd", 0x631c_37ce_72a9_7393),
        (b"asdf", 0x4158_72f5_99ce_a71e),
        (
            b"Call me Ishmael. Some years ago--never mind how long precisely-",
            0x02a2_e854_70d6_fd96,
        ),
    ] {
        assert_eq!(v2::xxh64(input, 0), hash);
        let mut h = v2::Xxh64::new(0);
        input.chunks(5).for_each(|c| h.update(c));
        assert_eq!(h.finish(), hash);
    }
    let (positions, indices) = grid(4);
    let config = clusterlod::default_config(16).unwrap();
    let options = v2::WriteOptions::new(QuantGrid::new(GRID).unwrap());
    let key = |c: &clusterlod::Config, o: &v2::WriteOptions, p: &[[f32; 3]]| {
        v2::build_key(&v2::KeyInputs::new(c, o, p, &indices))
    };
    let base = key(&config, &options, &positions);
    let narrow: Vec<u16> = indices.iter().map(|&i| i as u16).collect();
    assert_eq!(
        base,
        v2::build_key(&v2::KeyInputs::new(&config, &options, &positions, &narrow))
    );
    let mut moved = positions.clone();
    moved[3][1] = f32::from_bits(moved[3][1].to_bits() ^ 1);
    let mut c2 = config;
    c2.simplify_ratio = 0.6;
    let mut o2 = options;
    o2.page_capacity = 4096;
    // Vertex flags, attributes, weights and the protect mask change the build.
    let mut locks = vec![meshoptimizer_rs::VertexFlags::EMPTY; positions.len()];
    locks[5] = meshoptimizer_rs::VertexFlags::LOCK;
    let uv: Vec<f32> = positions.iter().flat_map(|p| [p[0], p[1]]).collect();
    let attributes =
        meshoptimizer_rs::Attributes::from_interleaved(&uv, positions.len(), 2, 2, 0).unwrap();
    let plain = v2::KeyInputs::new(&config, &options, &positions, &indices);
    let (mut a, mut b, mut c, mut d) = (plain, plain, plain, plain);
    a.vertex_lock = Some(&locks);
    b.attributes = Some(attributes);
    c.attribute_weights = &[1.0, 1.0];
    d.attribute_protect_mask = 1;
    let extended = [a, b, c, d].map(|k| v2::build_key(&k));
    for other in [
        key(&config, &options, &moved),
        key(&c2, &options, &positions),
        key(&config, &o2, &positions),
    ]
    .into_iter()
    .chain(extended)
    {
        assert_ne!(other, base);
    }
}

#[test]
fn round_trip_matches_builder() {
    for mesh in [grid(32), sphere(24, 40)] {
        let b = build(mesh);
        for (codec, capacity) in [
            (DiskCodec::Compressed, 4096),
            (DiskCodec::None, v2::DEFAULT_PAGE_CAPACITY),
        ] {
            let (file, report) = write(&b, codec, capacity);
            assert_eq!(write(&b, codec, capacity).0, file, "deterministic");
            let r = v2::Reader::read_all(&file, &mut Workspace::default()).unwrap();
            let resident = v2::resident_len(&file).unwrap();
            assert_eq!(
                (r.resident_len(), report.resident_bytes),
                (resident, resident)
            );
            let open = v2::Reader::open(&file[..resident]).unwrap();
            let m = open.mesh();
            assert_eq!(m.group_count as usize, b.groups.len());
            assert_eq!(m.cluster_count as usize, b.clusters.len());
            assert_eq!(m.dag_triangles as usize * 3, b.flat.len());
            assert_eq!(report.pinned_pages, m.pinned_page_count as usize);
            assert!(report.pinned_pages >= 1 && m.page_count as usize == report.pages);
            let n = b.groups.len();
            let mut order = Vec::new();
            for g in 0..n {
                let (src, rec) = (b.groups[n - 1 - g], open.group(g).unwrap());
                assert_eq!(
                    (rec.depth as i32, rec.cluster_count),
                    (src.depth, src.cluster_count)
                );
                assert!(rec.error >= src.error && rec.radius >= src.radius);
                order.extend(src.first_cluster..src.first_cluster + src.cluster_count);
            }
            let pages = transcode_all(&open, &file);
            let (mut pos, mut tri) = ([[0f32; 3]; 256], [[0u8; 3]; 256]);
            for (p, words) in pages.iter().enumerate() {
                let view = PageView::new(words, m.grid).unwrap();
                let first = open.page(p).unwrap().first_cluster as usize;
                for k in 0..view.cluster_count() {
                    let src = b.clusters[order[first + k] as usize];
                    let d = view.cluster(k).unwrap();
                    let want = if src.refined < 0 {
                        -1
                    } else {
                        (n - 1 - src.refined as usize) as i32
                    };
                    assert_eq!(
                        (d.group as usize, d.refined),
                        (n - 1 - src.group as usize, want)
                    );
                    view.decode_positions(k, &mut pos).unwrap();
                    let nt = view.decode_triangles(k, &mut tri).unwrap();
                    let bits = |v: [f32; 3]| v.map(f32::to_bits);
                    let got = canonical(tri[..nt].iter().map(|t| t.map(|i| bits(pos[i as usize]))));
                    let span = &b.flat
                        [src.first_index as usize..(src.first_index + src.index_count) as usize];
                    let want = canonical(
                        span.chunks(3)
                            .map(|t| [0, 1, 2].map(|j| bits(b.snapshot[t[j] as usize]))),
                    );
                    assert_eq!(got, want);
                }
            }
        }
    }
}

struct Rng(u64);
impl Rng {
    fn unit(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
    fn view(&mut self) -> CutView {
        let position = [0, 1, 2].map(|_| 80. * self.unit() - 40.);
        let projection = Projection::Perspective {
            proj_y: 1.5,
            znear: 0.1,
        };
        CutView {
            position,
            projection,
            viewport_height: 1080.,
            scale: 1.,
        }
    }
}

/// Every resident set that is a page prefix covering the pinned pages, or is
/// closed under page dependencies, gives a cut equal to sequential refinement.
#[test]
fn page_prefixes_and_dependency_closed_sets_are_valid_cuts() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for mesh in [grid(32), sphere(24, 40)] {
        let b = build(mesh);
        let (file, _) = write(&b, DiskCodec::Compressed, 4096);
        let r = v2::Reader::open(&file).unwrap();
        let (groups, clusters) = fine_tables(&r, &transcode_all(&r, &file));
        assert_eq!(cut::check_monotonic(&groups, &clusters), Ok(()));
        let (n, m) = (groups.len(), r.mesh());
        let deps: Vec<Vec<u32>> = (0..m.page_count as usize)
            .map(|p| r.dependencies(p).unwrap().collect())
            .collect();
        let resident_groups = |loaded: &[bool]| -> Vec<bool> {
            (0..n)
                .map(|i| {
                    r.group(n - 1 - i)
                        .unwrap()
                        .pages()
                        .all(|p| loaded[p as usize])
                })
                .collect()
        };
        let check = |resident: &[bool], rng: &mut Rng| {
            let (mut a, mut s) = (vec![0; clusters.len()], vec![0; clusters.len()]);
            for _ in 0..4 {
                let (view, t) = (rng.view(), Threshold::Pixels(0.25 + 8. * rng.unit()));
                let k =
                    cut::select_cut(&groups, &clusters, &view, t, Some(resident), &mut a).unwrap();
                let j =
                    cut::refine_sequential(&groups, &clusters, &view, t, Some(resident), &mut s)
                        .unwrap();
                assert_eq!(a[..k], s[..j]);
            }
        };
        assert!(m.pinned_page_count >= 1 && m.page_count > m.pinned_page_count + 4);
        for k in m.pinned_page_count..=m.page_count {
            let loaded: Vec<bool> = (0..m.page_count).map(|p| p < k).collect();
            check(&resident_groups(&loaded), &mut rng);
        }
        for _ in 0..40 {
            let mut loaded: Vec<bool> = (0..m.page_count)
                .map(|p| p < m.pinned_page_count || rng.unit() < 0.3)
                .collect();
            for p in (0..loaded.len()).rev() {
                if loaded[p] {
                    deps[p].iter().for_each(|&d| loaded[d as usize] = true);
                }
            }
            check(&resident_groups(&loaded), &mut rng);
        }
        // Dropping one dependency of a loaded page leaves a hole the oracle reports.
        let p = (0..deps.len())
            .rev()
            .find(|&p| !deps[p].is_empty())
            .unwrap();
        let mut loaded = vec![true; deps.len()];
        loaded[*deps[p].last().unwrap() as usize] = false;
        let mut out = vec![0; clusters.len()];
        let e = cut::refine_sequential(
            &groups,
            &clusters,
            &rng.view(),
            Threshold::Finest,
            Some(&resident_groups(&loaded)),
            &mut out,
        );
        assert!(matches!(e, Err(CutError::Incomplete { .. })), "{e:?}");
    }
}

/// Review probe E1: a dependency list with one entry replaced (still sorted,
/// unique and below the page) is rejected by `open`, before any page is read,
/// so a runtime never loads a set the lie leaves unclosed.
#[test]
fn open_rejects_any_replaced_dependency_entry() {
    for mesh in [grid(32), sphere(24, 40)] {
        let b = build(mesh);
        let (file, _) = write(&b, DiskCodec::Compressed, 4096);
        let r = v2::Reader::open(&file).unwrap();
        let (pdep, pages_at) = (chunk_at(&file, b"PDEP"), chunk_at(&file, b"PAGE"));
        let mut replaced = 0;
        for p in 0..r.mesh().page_count as usize {
            let deps: Vec<u32> = r.dependencies(p).unwrap().collect();
            let first = word(&file, pages_at + 48 * p + 24) as usize;
            let Some(other) = (0..p as u32).find(|f| !deps.contains(f)) else {
                continue;
            };
            for i in 0..deps.len() {
                let mut list = deps.clone();
                list[i] = other;
                list.sort_unstable();
                let mut bad = file.clone();
                for (j, &d) in list.iter().enumerate() {
                    set(&mut bad, pdep + 4 * (first + j), d);
                }
                reseal(&mut bad);
                let open = v2::Reader::open(&bad);
                assert_eq!(
                    open.err(),
                    Some(Error::Invalid),
                    "page {p}: {deps:?} -> {list:?}"
                );
                replaced += 1;
            }
        }
        assert!(replaced >= 20, "{replaced}");
    }
}

/// Review probe E5: over every parent link of writer output, the crate's
/// projected error of the parent is at least the child's, for cameras on the
/// line through both centres and at random, so a cut never draws both.
#[test]
fn projected_error_is_monotone_over_parent_links() {
    let b = build(grid(32));
    let (file, _) = write(&b, DiskCodec::Compressed, 4096);
    let r = v2::Reader::open(&file).unwrap();
    let mut links = Vec::new();
    for g in 0..r.mesh().group_count as usize {
        for c in r.group(g).unwrap().clusters() {
            let q = r.cluster_refined(c as usize).unwrap();
            if q >= 0 {
                links.push((g, q as usize));
            }
        }
    }
    links.sort_unstable();
    links.dedup();
    assert!(links.len() >= 20);
    let lod = |g: &v2::GroupRecord| LodBounds {
        center: g.center,
        radius: g.radius,
        error: g.error,
    };
    let mut rng = Rng(12345);
    for &(p, q) in &links {
        let (gp, gq) = (r.group(p).unwrap(), r.group(q).unwrap());
        let dir = [0, 1, 2].map(|i| gq.center[i] - gp.center[i]);
        let len = dir.iter().map(|d| d * d).sum::<f32>().sqrt().max(1e-6);
        for k in 0..400 {
            let mut view = rng.view();
            if k % 2 == 0 {
                let t = gq.radius + 0.01 + 200. * rng.unit();
                view.position = [0, 1, 2].map(|i| gq.center[i] + dir[i] / len * t);
            }
            let (ep, eq) = (
                cut::projected_error(lod(&gp), &view),
                cut::projected_error(lod(&gq), &view),
            );
            assert!(
                ep >= eq,
                "link {p}->{q} at {:?}: {ep} < {eq}",
                view.position
            );
        }
    }
}

fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn set(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes());
}
/// Byte offset of chunk `tag`'s data.
fn chunk_at(b: &[u8], tag: &[u8; 4]) -> usize {
    (0..word(b, 12) as usize)
        .map(|i| 64 + 16 * i)
        .find(|&e| &b[e..e + 4] == tag)
        .map(|e| word(b, e + 8) as usize)
        .unwrap()
}
/// Recompute page hashes (where the range is in the file) and the resident hash.
fn reseal(b: &mut [u8]) {
    let pages = chunk_at(b, b"PAGE");
    for p in 0..word(b, 16) as usize {
        let rec = pages + 48 * p;
        let (at, len) = (word(b, rec) as usize, word(b, rec + 8) as usize);
        if let Some(disk) = b.get(at..at + len) {
            let h = v2::xxh64(disk, 0);
            b[rec + 32..rec + 40].copy_from_slice(&h.to_le_bytes());
        }
    }
    let resident = word(b, 24) as usize;
    if let Some(h) = b.get(..resident).and_then(v2::prefix_hash) {
        b[40..48].copy_from_slice(&h.to_le_bytes());
    }
}
/// Insert one more chunk at the end of the resident prefix.
fn with_chunk(b: &[u8], tag: &[u8; 4], flags: u32, payload: &[u8]) -> Vec<u8> {
    let (n, resident) = (word(b, 12) as usize, word(b, 24) as usize);
    let padded = payload.len().next_multiple_of(16);
    let mut out = b[..64].to_vec();
    set(&mut out, 12, n as u32 + 1);
    set(&mut out, 24, (resident + 16 + padded) as u32);
    for e in (0..n).map(|i| 64 + 16 * i) {
        out.extend_from_slice(&b[e..e + 8]);
        out.extend((word(b, e + 8) + 16).to_le_bytes());
        out.extend_from_slice(&b[e + 12..e + 16]);
    }
    out.extend(tag);
    for v in [flags, resident as u32 + 16, payload.len() as u32] {
        out.extend(v.to_le_bytes());
    }
    out.extend_from_slice(&b[64 + 16 * n..resident]);
    out.extend(payload);
    out.resize(resident + 16 + padded, 0);
    out.extend_from_slice(&b[resident..]);
    let pages = chunk_at(&out, b"PAGE");
    for p in 0..word(&out, 16) as usize {
        let at = pages + 48 * p;
        let moved = word(&out, at) + 16 + padded as u32;
        set(&mut out, at, moved);
    }
    reseal(&mut out);
    out
}

#[derive(Debug, PartialEq)]
enum Stage {
    Open,
    Unsupported,
    Limit,
    Pages,
}

/// Each rule of the validation list, reached by a mutation with hashes resealed.
#[test]
fn rejects_each_validation_rule() {
    let b = build(grid(16));
    let (file, _) = write(&b, DiskCodec::None, 4096);
    let r = v2::Reader::read_all(&file, &mut Workspace::default()).unwrap();
    let m = r.mesh();
    let [mesh, groups, nodes, pages, deps, attr, spheres, cref] = [
        "MESH", "GRPS", "NODE", "PAGE", "PDEP", "ATTR", "CSPH", "CREF",
    ]
    .map(|t| chunk_at(&file, t.as_bytes().try_into().unwrap()));
    // A parent link whose child error is above `q`, as (cluster, owner).
    let (link_cluster, link_owner) = (0..m.group_count as usize)
        .flat_map(|g| r.group(g).unwrap().clusters().map(move |c| (c as usize, g)))
        .find(|&(c, _)| {
            let q = r.cluster_refined(c).unwrap();
            q >= 0 && r.group(q as usize).unwrap().error > m.max_displacement
        })
        .unwrap();
    let last_group = m.group_count as usize - 1;
    let internal = (0..m.node_count as usize)
        .find(|&i| r.node(i).unwrap().group < 0)
        .unwrap();
    let leaf = (0..m.node_count as usize)
        .find(|&i| r.node(i).unwrap().group > 0)
        .unwrap();
    let (gr, nd, pg) = (
        |g: usize, w: usize| groups + 48 * g + 4 * w,
        |i: usize, w: usize| nodes + 32 * i + 4 * w,
        |p: usize, w: usize| pages + 48 * p + 4 * w,
    );
    let last_page = m.page_count as usize - 1;
    let page0 = r.page(0).unwrap().disk_offset as usize;
    // DiskCodec::None: the memory form follows a 4-byte codec word.
    let desc = |p: usize, k: usize, w: usize| {
        r.page(p).unwrap().disk_offset as usize + 4 + 4 * (4 + 16 * k + w)
    };
    let refining = (0..m.cluster_count as usize)
        .find(|&c| word(&file, desc(0, c, 1)) as i32 >= 0)
        .unwrap();
    let parent = word(&file, desc(0, refining, 0)) as usize;
    let table = |tag: usize, w: usize| 64 + 16 * tag + 4 * w;
    let f = f32::to_bits;
    let cases: Vec<(&str, usize, u32, Stage)> = vec![
        // 1. header and chunk table
        ("magic", 0, 0, Stage::Open),
        ("major", 4, 3, Stage::Unsupported),
        ("header reserved", 56, 1, Stage::Open),
        ("header tail", 60, 1, Stage::Open),
        ("capacity", 20, 3000, Stage::Open),
        ("resident above 128 MiB", 24, (128 << 20) + 16, Stage::Limit),
        ("cluster cap zero", 28, 0, Stage::Open),
        ("cluster cap above capacity", 28, 256, Stage::Open),
        ("pages above cluster cap", 28, 1, Stage::Open),
        ("vertex cap above format", 48, 257, Stage::Open),
        ("vertex cap", 48, 3, Stage::Pages),
        ("triangle cap", 52, 1, Stage::Pages),
        ("cluster sphere NaN", spheres + 12, f(f32::NAN), Stage::Open),
        (
            "cluster sphere differs from page",
            spheres + 12,
            word(&file, spheres + 12) + 1,
            Stage::Pages,
        ),
        ("required bit cleared", table(2, 1), 0, Stage::Open),
        ("unknown flag", table(2, 1), 3, Stage::Unsupported),
        (
            "misaligned chunk",
            table(2, 2),
            nodes as u32 + 4,
            Stage::Open,
        ),
        ("overlapping chunk", table(2, 2), mesh as u32, Stage::Open),
        ("chunk past prefix", table(2, 3), 1 << 20, Stage::Open),
        (
            "unknown required chunk",
            table(2, 0),
            u32::from_le_bytes(*b"ZZZZ"),
            Stage::Unsupported,
        ),
        (
            "missing required chunk",
            table(2, 0),
            u32::from_le_bytes(*b"ZZZZ") | 1 << 31,
            Stage::Open,
        ),
        // 2. groups
        ("negative error", gr(0, 4), f(-1.), Stage::Open),
        ("error below q", gr(last_group, 4), f(0.), Stage::Open),
        ("NaN radius", gr(1, 3), f(f32::NAN), Stage::Open),
        ("depth past levels", gr(0, 5), m.level_count, Stage::Open),
        (
            "depth increases",
            gr(last_group, 5),
            m.level_count - 1,
            Stage::Open,
        ),
        (
            "cluster ranges",
            gr(1, 6),
            word(&file, gr(1, 6)) + 1,
            Stage::Open,
        ),
        (
            "page span",
            gr(last_group, 8),
            word(&file, gr(last_group, 8)) - 1,
            Stage::Open,
        ),
        ("parent count above index", gr(0, 10), 1, Stage::Open),
        (
            "parentless and not pinned",
            gr(last_group, 10),
            0,
            Stage::Open,
        ),
        // 3. nodes
        (
            "child before parent",
            nd(internal, 6),
            internal as u32,
            Stage::Open,
        ),
        ("child not contained", nd(internal, 3), 0, Stage::Open),
        ("child error above", nd(internal, 4), 0, Stage::Open),
        ("group in two leaves", nd(leaf, 5), 0, Stage::Open),
        ("bvh depth", mesh + 4, m.bvh_depth + 1, Stage::Open),
        // 4. pages
        ("page inside prefix", pg(0, 0), 0, Stage::Open),
        ("pages overlap", pg(1, 0), page0 as u32, Stage::Open),
        ("memory above capacity", pg(0, 3), 1025, Stage::Open),
        (
            "page clusters",
            pg(1, 4),
            word(&file, pg(1, 4)) + 1,
            Stage::Open,
        ),
        ("parentless page unpinned", pg(0, 10), 0, Stage::Open),
        ("pinned after unpinned", pg(last_page, 10), 1, Stage::Open),
        ("dependency not below page", deps, 0xffff, Stage::Open),
        // 5. mesh
        ("group count", mesh + 12, m.group_count + 1, Stage::Open),
        ("NaN box", mesh + 48, f(f32::NAN), Stage::Open),
        ("exponent", mesh + 88, 200, Stage::Open),
        (
            "displacement",
            mesh + 92,
            f(m.max_displacement) + 1,
            Stage::Open,
        ),
        ("mesh reserved", mesh + 104, 1, Stage::Open),
        // 6. refinement relation
        (
            "refined not after owner",
            cref + 4 * link_cluster,
            link_owner as u32,
            Stage::Open,
        ),
        (
            "refined below -1",
            cref + 4 * link_cluster,
            -2i32 as u32,
            Stage::Open,
        ),
        (
            "parent error below child",
            gr(link_owner, 4),
            f(m.max_displacement),
            Stage::Open,
        ),
        (
            "child sphere outside parent",
            gr(link_owner, 3),
            0,
            Stage::Open,
        ),
        (
            "parent count",
            gr(last_group, 10),
            word(&file, gr(last_group, 10)) + 1,
            Stage::Open,
        ),
        // 7. page bytes
        (
            "disk length",
            pg(0, 2),
            word(&file, pg(0, 2)) - 4,
            Stage::Pages,
        ),
        // 8. clusters
        (
            "owner group",
            desc(0, 0, 0),
            last_group as u32,
            Stage::Pages,
        ),
        (
            "descriptor refined differs from CREF",
            desc(0, refining, 1),
            parent as u32,
            Stage::Pages,
        ),
        // 9. vertices
        (
            "vertex outside box",
            mesh + 60,
            f(m.aabb_min[0]),
            Stage::Pages,
        ),
        (
            "vertex outside group sphere",
            gr(last_group, 3),
            0,
            Stage::Pages,
        ),
        ("vertex outside mesh sphere", mesh + 84, 0, Stage::Pages),
        // 10. attributes
        ("attribute count without records", attr, 1, Stage::Open),
    ];
    for (name, at, value, stage) in cases {
        let mut bad = file.clone();
        set(&mut bad, at, value);
        reseal(&mut bad);
        let open = v2::Reader::open(&bad);
        match stage {
            Stage::Open => assert_eq!(open.err(), Some(Error::Invalid), "{name}"),
            Stage::Unsupported => assert_eq!(open.err(), Some(Error::Unsupported), "{name}"),
            Stage::Limit => assert_eq!(open.err(), Some(Error::Limit), "{name}"),
            Stage::Pages => {
                assert!(open.is_ok(), "{name}: {:?}", open.err());
                assert!(
                    v2::Reader::read_all(&bad, &mut Workspace::default()).is_err(),
                    "{name}"
                );
            }
        }
    }
    // Corruption without a reseal: resident (header included) and page hashes.
    for at in [groups, 32, 8] {
        let mut bad = file.clone();
        bad[at] ^= 1;
        assert_eq!(
            v2::Reader::open(&bad).err(),
            Some(Error::Invalid),
            "byte {at}"
        );
    }
    let mut bad = file.clone();
    bad[page0 + 40] ^= 1;
    assert!(v2::Reader::open(&bad).is_ok());
    assert!(v2::Reader::read_all(&bad, &mut Workspace::default()).is_err());
    // Chunk extension: unknown optional chunks are skipped, unknown required ones rejected.
    for (tag, flags, ok) in [(b"ZOPT", 0, true), (b"NAAB", 0, true), (b"ZREQ", 1, false)] {
        let ext = with_chunk(&file, tag, flags, b"payload");
        match v2::Reader::read_all(&ext, &mut Workspace::default()) {
            Ok(r) => assert!(ok && r.chunk(*tag) == Some(&b"payload"[..])),
            Err(e) => assert!(!ok && e == Error::Unsupported),
        }
    }
    for end in [0, 63, 64, r.resident_len() - 16] {
        assert!(v2::Reader::open(&file[..end]).is_err());
    }
}

#[test]
fn empty_mesh_and_seeds() {
    let mut ws = Workspace::default();
    let quant = QuantGrid::new(GRID).unwrap();
    let source = v2::Source::<u32>::new(&[], &[], &[], &[]);
    let (empty, report) = v2::write(&source, &v2::WriteOptions::new(quant), &mut ws).unwrap();
    assert_eq!(
        (
            report.pages,
            v2::Reader::read_all(&empty, &mut ws)
                .unwrap()
                .mesh()
                .group_count
        ),
        (0, 0)
    );
    let mut tiny = v2::WriteOptions::new(quant);
    tiny.page_capacity = 1000;
    assert_eq!(
        v2::write(&source, &tiny, &mut ws).err(),
        Some(Error::Invalid)
    );
    // Page limits are writer parameters recorded in the header.
    let b = build(grid(8));
    let src = v2::Source::new(&b.groups, &b.clusters, &b.snapshot, &b.flat);
    let mut o = v2::WriteOptions::new(quant);
    o.max_page_clusters = 3;
    let (file, _) = v2::write(&src, &o, &mut ws).unwrap();
    let r = v2::Reader::read_all(&file, &mut ws).unwrap();
    assert_eq!(r.page_limits().max_clusters, 3);
    assert!((0..r.mesh().page_count as usize).all(|p| r.page(p).unwrap().cluster_count <= 3));
    o.max_cluster_vertices = 4;
    assert_eq!(v2::write(&src, &o, &mut ws).err(), Some(Error::Invalid));
    // DAG_V2_SEED_DIR=<dir> writes the fuzz seeds of the `dag_format_v2` target.
    if let Ok(dir) = std::env::var("DAG_V2_SEED_DIR") {
        std::fs::write(format!("{dir}/empty.bin"), &empty).unwrap();
        for (name, n, codec, capacity) in [
            ("grid2-none", 2, DiskCodec::None, 4096),
            ("grid6-none", 6, DiskCodec::None, 4096),
            ("grid6-compressed", 6, DiskCodec::Compressed, 4096),
            (
                "grid8-compressed",
                8,
                DiskCodec::Compressed,
                v2::DEFAULT_PAGE_CAPACITY,
            ),
        ] {
            let b = build(grid(n));
            std::fs::write(format!("{dir}/{name}.bin"), write(&b, codec, capacity).0).unwrap();
        }
    }
}
