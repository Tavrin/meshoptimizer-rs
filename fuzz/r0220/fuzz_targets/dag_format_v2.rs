#![no_main]
//! MDAG 2 reader: hostile resident prefixes and pages. Each input runs as given
//! and again with its page and resident hashes recomputed, so mutations reach
//! the structural rules behind the hash checks. An input that opens is also
//! run with three review-style edits (one dependency entry replaced, one parent
//! count raised, one refined group changed), resealed. For every file `open`
//! accepts, every page set holding the pinned pages and closed under
//! dependencies must give a valid cut: `refine_sequential` never `Incomplete`,
//! equal to `select_cut`, and every parentless group resident.
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{
    clusterlod::{
        cut::{self, CutView, Projection, Threshold},
        dag_format::v2,
        FlatDagCluster, FlatDagGroup, LodBounds,
    },
    Workspace,
};

fn word(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

fn disk<'a>(data: &'a [u8], range: core::ops::Range<u64>) -> Option<&'a [u8]> {
    data.get(usize::try_from(range.start).ok()?..usize::try_from(range.end).ok()?)
}

/// Data offset and length of chunk `tag`.
fn chunk(b: &[u8], tag: &[u8; 4]) -> Option<(usize, usize)> {
    let chunks = word(b, 12)? as usize;
    let entry = (0..chunks.min(v2::MAX_CHUNKS))
        .map(|i| 64 + 16 * i)
        .find(|&e| b.get(e..e + 4) == Some(tag.as_slice()))?;
    Some((word(b, entry + 8)? as usize, word(b, entry + 12)? as usize))
}

/// Recompute every page hash whose range lies in the input, then the resident hash.
fn reseal(b: &mut [u8]) -> Option<()> {
    let resident = word(b, 24)? as usize;
    if !(64..=b.len()).contains(&resident) {
        return None;
    }
    let (at, len) = chunk(b, b"PAGE")?;
    for p in 0..len / v2::PAGE_BYTES {
        let rec = at + p * v2::PAGE_BYTES;
        let start = u64::from(word(b, rec)?) | u64::from(word(b, rec + 4)?) << 32;
        let end = start.checked_add(u64::from(word(b, rec + 8)?))?;
        if let Some(h) = disk(b, start..end).map(|d| v2::xxh64(d, 0)) {
            b.get_mut(rec + 32..rec + 40)?
                .copy_from_slice(&h.to_le_bytes());
        }
    }
    let h = v2::prefix_hash(b.get(..resident)?)?;
    b.get_mut(v2::RESIDENT_HASH_FIELD)?
        .copy_from_slice(&h.to_le_bytes());
    Some(())
}

/// Fine-to-coarse tables (the form `cut` takes) from the resident prefix alone.
fn fine_tables(r: &v2::Reader) -> (Vec<FlatDagGroup>, Vec<FlatDagCluster>) {
    let n = r.mesh().group_count as usize;
    let (mut groups, mut clusters) = (Vec::with_capacity(n), Vec::new());
    for i in 0..n {
        let rec = r.group(n - 1 - i).expect("validated group");
        let mut g = FlatDagGroup::default();
        (g.center, g.radius, g.error, g.depth) =
            (rec.center, rec.radius, rec.error, rec.depth as i32);
        (g.first_cluster, g.cluster_count) = (clusters.len() as u32, rec.cluster_count);
        for c in rec.clusters() {
            let q = r.cluster_refined(c as usize).expect("validated cluster");
            let mut cl = FlatDagCluster::default();
            cl.group = i as u32;
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

/// Every page set holding the pinned pages and closed under dependencies is a
/// valid residency set: sequential refinement never misses a cluster, equals
/// the direct cut, and every parentless group is resident.
fn check_cuts(r: &v2::Reader, data: &[u8]) {
    let m = r.mesh();
    let (n, np) = (m.group_count as usize, m.page_count as usize);
    if n == 0 || m.cluster_count > 1 << 16 || np > 1 << 14 {
        return;
    }
    let (groups, clusters) = fine_tables(r);
    assert_eq!(cut::check_monotonic(&groups, &clusters), Ok(()));
    let mut parentless = vec![true; n];
    for c in &clusters {
        if c.refined >= 0 {
            parentless[c.refined as usize] = false;
        }
    }
    let seed = data
        .iter()
        .fold(0u64, |h, &b| h.rotate_left(5) ^ u64::from(b));
    let mut sets = vec![
        (0..np)
            .map(|p| p < m.pinned_page_count as usize)
            .collect::<Vec<bool>>(),
        vec![true; np],
    ];
    for k in 0..2u64 {
        let mut loaded: Vec<bool> = (0..np)
            .map(|p| {
                let bit = seed.rotate_left((p as u32 + 17 * k as u32) % 64) & 1;
                p < m.pinned_page_count as usize || bit == 1
            })
            .collect();
        for p in (0..np).rev() {
            if loaded[p] {
                for d in r.dependencies(p).expect("validated dependencies") {
                    loaded[d as usize] = true;
                }
            }
        }
        sets.push(loaded);
    }
    let mut rng = seed | 1;
    let mut random = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng as u32
    };
    let ortho = CutView {
        position: [0.0; 3],
        projection: Projection::Orthographic {
            pixels_per_unit: 1.0,
        },
        viewport_height: 1.0,
        scale: 1.0,
    };
    let mut views = vec![ortho];
    // Random finite camera bits cover ordinary and extremely far distances.
    // Keep the exact review camera as well, including its projection scale.
    for far in [false, true] {
        let position = core::array::from_fn(|_| {
            let bits = random();
            if far {
                f32::from_bits(bits & 0xfeffffff)
            } else {
                (bits as i32 as f32) / 1e7
            }
        });
        views.push(CutView {
            position,
            projection: Projection::Perspective {
                proj_y: 0.5 + (random() % 1000) as f32 / 200.0,
                znear: 0.01 + (random() % 100) as f32 / 10.0,
            },
            ..ortho
        });
    }
    views.push(CutView {
        position: [f32::from_bits(0xda00_0000), 0.0, 0.0],
        projection: Projection::Perspective {
            proj_y: 1.5,
            znear: 1.0,
        },
        ..ortho
    });
    let mut produced = vec![Vec::new(); n];
    for (i, c) in clusters.iter().enumerate() {
        if c.refined >= 0 {
            produced[c.refined as usize].push(i);
        }
    }
    let (mut a, mut s) = (vec![0u32; clusters.len()], vec![0u32; clusters.len()]);
    for loaded in &sets {
        // Group i of the fine tables is file group n - 1 - i.
        let resident: Vec<bool> = (0..n)
            .map(|i| {
                let g = r.group(n - 1 - i).expect("validated group");
                g.pages().all(|p| loaded[p as usize])
            })
            .collect();
        assert!(
            (0..n).all(|i| !parentless[i] || resident[i]),
            "parentless group not resident"
        );
        for view in &views {
            let g = &groups[random() as usize % n];
            let pixels = cut::projected_error(
                LodBounds {
                    center: g.center,
                    radius: g.radius,
                    error: g.error,
                },
                view,
            );
            let edge = if pixels.is_finite() { pixels } else { f32::MAX };
            for t in [
                Threshold::Finest,
                Threshold::Pixels(0.0),
                Threshold::Pixels(edge),
                Threshold::Pixels(edge.next_down().max(0.0)),
                Threshold::Pixels(f32::MAX),
            ] {
                let k = cut::select_cut(&groups, &clusters, view, t, Some(&resident), &mut a)
                    .expect("select_cut");
                let j =
                    cut::refine_sequential(&groups, &clusters, view, t, Some(&resident), &mut s)
                        .expect("refine_sequential on a dependency-closed set");
                assert_eq!(a[..k], s[..j]);
                // Independently count selected clusters along every source-to-root
                // chain, retaining both extremes through DAG joins.
                let mut selected = vec![0u32; clusters.len()];
                for &c in &a[..k] {
                    selected[c as usize] += 1;
                }
                let mut range = vec![(0u32, 0u32); clusters.len()];
                for i in (0..clusters.len()).rev() {
                    let above = &produced[clusters[i].group as usize];
                    let lo = above.iter().map(|&c| range[c].0).min().unwrap_or(0);
                    let hi = above.iter().map(|&c| range[c].1).max().unwrap_or(0);
                    range[i] = (lo + selected[i], hi + selected[i]);
                    if clusters[i].refined < 0 {
                        assert_eq!(range[i], (1, 1), "exactly-once coverage");
                    }
                }
            }
        }
    }
}

fn check(data: &[u8]) {
    let mut ws = Workspace::default();
    if let Ok(r) = v2::Reader::open(data) {
        // Every accessor of a validated prefix succeeds; pages named by the
        // records either validate or fail cleanly.
        let m = r.mesh();
        for g in 0..m.group_count as usize {
            r.group(g).expect("validated group");
        }
        for n in 0..m.node_count as usize {
            r.node(n).expect("validated node");
        }
        let mut out = vec![0u32; r.page_capacity() / 4];
        for p in 0..m.page_count as usize {
            let rec = r.page(p).expect("validated page");
            assert!(r
                .dependencies(p)
                .expect("dependencies")
                .all(|d| (d as usize) < p));
            if let Some(bytes) = disk(data, rec.disk_range()) {
                let _ = r.transcode_page(p, bytes, &mut out, &mut ws);
            }
        }
        let _ = r.chunk(v2::TAG_FAR_FIELD);
        check_cuts(&r, data);
    }
    let Ok(all) = v2::Reader::read_all(data, &mut ws) else {
        return;
    };
    // A whole valid file opens from its resident prefix alone, and every page
    // transcodes from exactly its own bytes and fails when truncated.
    let prefix = &data[..v2::resident_len(data).expect("resident length")];
    let r = v2::Reader::open(prefix).expect("resident prefix opens");
    assert_eq!(r.mesh(), all.mesh());
    let mut out = vec![0u32; r.page_capacity() / 4];
    let (mut pos, mut tri) = ([[0f32; 3]; 256], [[0u8; 3]; 256]);
    for p in 0..r.mesh().page_count as usize {
        let bytes = disk(data, r.page(p).expect("page").disk_range()).expect("page bytes");
        let view = r
            .transcode_page(p, bytes, &mut out, &mut ws)
            .expect("validated page transcodes");
        for k in 0..view.cluster_count() {
            let d = view.cluster(k).expect("descriptor");
            let c = r.page(p).expect("page").first_cluster as usize + k;
            assert_eq!(d.refined, r.cluster_refined(c).expect("refined"));
            let nv = view.decode_positions(k, &mut pos).expect("positions");
            let nt = view.decode_triangles(k, &mut tri).expect("triangles");
            assert!(tri[..nt].iter().flatten().all(|&i| (i as usize) < nv));
        }
        assert!(r
            .transcode_page(p, &bytes[..bytes.len() - 1], &mut out, &mut ws)
            .is_err());
    }
}

/// Review-style edits of an accepted file: a replaced dependency entry
/// (B1), a raised parent count (B2) and a changed refined group, each
/// resealed so only the structural rules can reject it.
fn edits(data: &[u8]) -> Vec<Vec<u8>> {
    let pick = data
        .iter()
        .fold(0x9e37_79b9u32, |h, &b| h.rotate_left(3) ^ u32::from(b));
    let mut out = Vec::new();
    let mut edit = |tag: &[u8; 4], record: usize, field: usize, value: &dyn Fn(u32) -> u32| {
        let Some((at, len)) = chunk(data, tag) else {
            return;
        };
        let count = len / record;
        if count == 0 {
            return;
        }
        let i = pick as usize % count;
        let pos = at + i * record + field;
        let Some(old) = word(data, pos) else { return };
        let mut b = data.to_vec();
        b[pos..pos + 4].copy_from_slice(&value(old).to_le_bytes());
        if reseal(&mut b).is_some() {
            out.push(b);
        }
    };
    edit(b"PDEP", 4, 0, &|d| d ^ (1 + (pick >> 8) % 4));
    edit(b"GRPS", v2::GROUP_BYTES, 40, &|n| n.wrapping_add(1));
    edit(b"MATS", 16, 0, &|n| n.wrapping_add(1));
    edit(b"MATS", 16, 4, &|n| n ^ 1);
    edit(b"ATTR", 4, 0, &|n| n ^ (1 + (pick >> 12) % 16));
    edit(b"MESH", v2::MESH_BYTES, 100, &|n| {
        n ^ (1 + (pick >> 12) % 65536)
    });
    edit(b"CREF", 4, 0, &|q| q.wrapping_add(1 + (pick >> 16) % 3));
    out
}

fuzz_target!(|data: &[u8]| {
    check(data);
    // Exercise attribute block validation independently of resident/page hashes.
    if data.len() <= 1 << 18 && data.len().is_multiple_of(4) {
        let words: Vec<u32> = data
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        let vertices = words.get(3).copied().unwrap_or(0) as usize;
        if let Ok(block) = meshoptimizer_rs::clusterlod::attributes::Block::new(&words, vertices) {
            for c in 0..block.channel_count() {
                for v in 0..vertices {
                    assert!(block.decode(c, v).unwrap().iter().all(|x| x.is_finite()));
                }
            }
        }
        if let Ok(grid) = meshoptimizer_rs::clusterlod::page::QuantGrid::new(-10) {
            let _ = meshoptimizer_rs::clusterlod::page::PageView::new(&words, grid);
        }
    }
    let mut sealed = data.to_vec();
    if reseal(&mut sealed).is_some() && sealed != data {
        check(&sealed);
    }
    if v2::Reader::open(&sealed).is_ok() {
        for edited in edits(&sealed) {
            check(&edited);
        }
    }
});
