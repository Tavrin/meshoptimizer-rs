//! Review probes E2 and E3 on hand-built MDAG 2 files.
#![cfg(feature = "clusterlod")]
#[path = "support/mdag2_craft.rs"]
mod craft;

use craft::{page, page_record, sphere_words, Craft, Group, Node};
use meshoptimizer_rs::clusterlod::dag_format::{v2, Error};
use meshoptimizer_rs::Workspace;

/// Three groups: P (depth 1, pinned) refines Q; X is refined by nobody but
/// declares `x_parents` parents. With `x_parents = 0` X is pinned.
fn e2(x_parents: u32) -> Vec<u8> {
    let mut c = Craft {
        levels: 2,
        bvh: 2,
        width: 2,
        capacity: 4096,
        max_clusters: 63,
        dag: 3,
        leaf: 2,
        ..Default::default()
    };
    let clusters = [(0, 2), (1, -1), (2, -1)];
    let pages: Vec<_> = clusters.iter().map(|&cl| page(&[cl])).collect();
    for (i, (error, depth, parents)) in [(f32::MAX, 1, 0), (1.0, 0, x_parents), (1.0, 0, 1)]
        .into_iter()
        .enumerate()
    {
        let i = i as u32;
        c.groups.push(Group {
            error,
            depth,
            first_cluster: i,
            clusters: 1,
            first_page: i,
            pages: 1,
            parents,
        });
    }
    let x_pinned = x_parents == 0;
    c.pages
        .push(page_record(&pages[0].0, &pages[0].1, 0, 0, 0, true));
    c.pages
        .push(page_record(&pages[1].0, &pages[1].1, 1, 0, 0, x_pinned));
    c.pages
        .push(page_record(&pages[2].0, &pages[2].1, 2, 0, 1, false));
    c.deps = vec![0];
    for (group, child_offset, child_count, error) in [
        (-1, 2, 2, 1.0),
        (0, 0, 0, f32::MAX),
        (1, 0, 0, 1.0),
        (2, 0, 0, 1.0),
    ] {
        c.nodes.push(Node {
            error,
            group,
            child_offset,
            child_count,
        });
    }
    for ((mem, disk), &(_, refined)) in pages.iter().zip(&clusters) {
        c.spheres.push(sphere_words(mem, 0));
        c.refined.push(refined);
        c.disk.extend_from_slice(disk);
    }
    craft::craft(&c)
}

/// Review probe E2: a group nobody refines that declares a parent (so it need
/// not be pinned) is rejected by `open`, before any page; the honest file
/// (parent count zero, pinned) is accepted.
#[test]
fn open_rejects_a_phantom_parent_count() {
    let honest = e2(0);
    v2::Reader::read_all(&honest, &mut Workspace::default()).unwrap();
    let lie = e2(1);
    let resident = v2::resident_len(&lie).unwrap();
    assert_eq!(
        v2::Reader::open(&lie[..resident]).err(),
        Some(Error::Invalid)
    );
}

/// Review probe E3: page 0 holds 15,000 clusters cycling over 17 refined
/// groups of 100,000 pages each (a 117 MiB prefix). Transcoding page 0 is
/// linear in the page, independent of the refined groups' spans.
#[test]
fn transcode_work_is_linear_in_the_page() {
    let (file, disk) = craft::e3(15_000, 100_000);
    let resident = v2::resident_len(&file).unwrap();
    assert!(resident > 100 << 20 && resident <= v2::MAX_BUFFER_VIEW_BYTES);
    let r = v2::Reader::open(&file[..resident]).unwrap();
    let mut out = vec![0u32; r.page_capacity() / 4];
    let mut ws = Workspace::default();
    let start = std::time::Instant::now();
    let view = r.transcode_page(0, &disk, &mut out, &mut ws).unwrap();
    let elapsed = start.elapsed();
    assert_eq!(view.cluster_count(), 15_000);
    println!("E3 transcode_page(0): {elapsed:?}");
    // Before the fix this took 2.7 s; the bound only applies to optimized builds.
    if !cfg!(debug_assertions) {
        assert!(elapsed.as_millis() < 50, "{elapsed:?}");
    }
}

/// A contained, equal-error chain whose projected errors invert at a far camera.
#[test]
fn far_perspective_rounding_cannot_reopen_a_stopped_region() {
    use meshoptimizer_rs::clusterlod::{
        cut::{self, CutView, Projection, Threshold},
        page::{self, DiskCodec, PageCluster, QuantGrid},
        FlatDagCluster, FlatDagGroup, LodBounds,
    };
    let error = f32::from_bits(0x3f800003);
    let mut c = Craft {
        levels: 3,
        bvh: 1,
        width: 2,
        capacity: 4096,
        max_clusters: 63,
        dag: 3,
        leaf: 1,
        ..Default::default()
    };
    let pos = [[1., 0., 0.], [1., 0.25, 0.], [1., 0., 0.25]];
    let idx = [0, 1, 2];
    let grid = QuantGrid::new(craft::GRID).unwrap();
    let mut ws = Workspace::default();
    for (i, e) in [f32::MAX, error, error].into_iter().enumerate() {
        let refined = if i == 2 { -1 } else { i as i32 + 1 };
        let pc = PageCluster::new(i as u32, refined, &pos, &idx);
        let mem = page::encode_page(grid, &[pc], craft::LAYOUT, &mut ws).unwrap();
        let disk = page::encode_disk_page(&mem, grid, DiskCodec::None, &mut ws).unwrap();
        c.groups.push(Group {
            error: e,
            depth: 2 - i as u32,
            first_cluster: i as u32,
            clusters: 1,
            first_page: i as u32,
            pages: 1,
            parents: u32::from(i != 0),
        });
        c.nodes.push(Node {
            error: e,
            group: i as i32,
            child_offset: 0,
            child_count: 0,
        });
        c.pages.push(page_record(
            &mem,
            &disk,
            i as u32,
            i.saturating_sub(1) as u32,
            u32::from(i != 0),
            i == 0,
        ));
        if i != 0 {
            c.deps.push(i as u32 - 1);
        }
        c.spheres.push(sphere_words(&mem, 0));
        c.refined.push(refined);
        c.disk.extend(disk);
    }
    c.nodes.reverse(); // Per-level roots are ordered by increasing depth.
    let mut file = craft::craft(&c);
    let chunk_at = |tag: &[u8; 4]| {
        let entry = (64..192)
            .step_by(16)
            .find(|&e| &file[e..e + 4] == tag)
            .unwrap();
        u32::from_le_bytes(file[entry + 8..entry + 12].try_into().unwrap()) as usize
    };
    let (mesh, groups_at, nodes_at) = (chunk_at(b"MESH"), chunk_at(b"GRPS"), chunk_at(b"NODE"));
    let put = |b: &mut [u8], at: usize, v: f32| b[at..at + 4].copy_from_slice(&v.to_le_bytes());
    // Mesh box and sphere enclose all three copies of the triangle.
    for (j, v) in [1., 0., 0., 1., 0.25, 0.25, 1., 0.125, 0.125, 0.25]
        .into_iter()
        .enumerate()
    {
        put(&mut file, mesh + 48 + 4 * j, v);
    }
    for i in 0..3 {
        let r = if i == 2 { 1. } else { 2. };
        for at in [
            groups_at + i * v2::GROUP_BYTES,
            nodes_at + (2 - i) * v2::NODE_BYTES,
        ] {
            put(&mut file, at, r);
            put(&mut file, at + 12, r);
        }
    }
    let resident = v2::resident_len(&file).unwrap();
    let h = v2::prefix_hash(&file[..resident]).unwrap();
    file[v2::RESIDENT_HASH_FIELD].copy_from_slice(&h.to_le_bytes());
    let reader = v2::Reader::read_all(&file, &mut ws).unwrap();
    let mut groups = Vec::new();
    let mut clusters = Vec::new();
    for i in 0..3 {
        let g = reader.group(2 - i).unwrap();
        let mut group = FlatDagGroup::default();
        (
            group.center,
            group.radius,
            group.error,
            group.depth,
            group.first_cluster,
            group.cluster_count,
        ) = (g.center, g.radius, g.error, g.depth as i32, i as u32, 1);
        groups.push(group);
        let mut cluster = FlatDagCluster::default();
        (cluster.group, cluster.refined) = (i as u32, if i == 0 { -1 } else { i as i32 - 1 });
        clusters.push(cluster);
    }
    assert_eq!(cut::check_monotonic(&groups, &clusters), Ok(()));
    let view = CutView {
        position: [f32::from_bits(0xda00_0000), 0., 0.],
        scale: 1.,
        projection: Projection::Perspective {
            proj_y: 1.5,
            znear: 1.,
        },
        viewport_height: 1.,
    };
    for (i, bits) in [(1, 0x24c00004), (0, 0x24c00005)] {
        let g = groups[i];
        assert_eq!(
            cut::projected_error(
                LodBounds {
                    center: g.center,
                    radius: g.radius,
                    error: g.error
                },
                &view
            )
            .to_bits(),
            bits
        );
    }
    for resident in [
        [true, true, true],
        [false, true, true],
        [false, false, true],
    ] {
        let (mut direct, mut sequential) = ([0; 3], [0; 3]);
        let t = Threshold::Pixels(f32::from_bits(0x24c00004));
        let n =
            cut::select_cut(&groups, &clusters, &view, t, Some(&resident), &mut direct).unwrap();
        let m = cut::refine_sequential(
            &groups,
            &clusters,
            &view,
            t,
            Some(&resident),
            &mut sequential,
        )
        .unwrap();
        assert_eq!(&direct[..n], &[2]);
        assert_eq!(direct[..n], sequential[..m]);
    }
    if let Some(dir) = std::env::var_os("DAG_V2_SEED_DIR") {
        std::fs::write(std::path::Path::new(&dir).join("far-perspective.bin"), file).unwrap();
    }
}
