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
