//! Hand-built MDAG 2 files for the review probes (E2-E4): groups, pages and
//! nodes are given directly, every sphere is centred at the origin, and pages
//! hold single-vertex clusters at the origin.
#![allow(dead_code)]
use meshoptimizer_rs::clusterlod::{
    dag_format::v2,
    page::{self, DiskCodec, PageCluster, QuantGrid},
};
use meshoptimizer_rs::Workspace;

pub const GRID: i32 = -8;
/// Attribute layout hash with no channels.
pub const LAYOUT: u32 = 0x51D8_E999;

pub struct Group {
    pub error: f32,
    pub depth: u32,
    pub first_cluster: u32,
    pub clusters: u32,
    pub first_page: u32,
    pub pages: u32,
    pub parents: u32,
}
pub struct Page {
    pub len: u32,
    pub words: u32,
    pub first_cluster: u32,
    pub clusters: u32,
    pub first_dep: u32,
    pub deps: u32,
    pub hash: u64,
    pub pinned: bool,
}
pub struct Node {
    pub error: f32,
    pub group: i32,
    pub child_offset: u32,
    pub child_count: u32,
}
#[derive(Default)]
pub struct Craft {
    pub groups: Vec<Group>,
    pub pages: Vec<Page>,
    pub deps: Vec<u32>,
    pub nodes: Vec<Node>,
    pub spheres: Vec<[u32; 4]>,
    pub refined: Vec<i32>,
    pub levels: u32,
    pub bvh: u32,
    pub width: u32,
    pub capacity: u32,
    pub max_clusters: u32,
    pub dag: u64,
    pub leaf: u64,
    pub disk: Vec<u8>,
}

/// Memory and disk form of one page of `(owner, refined)` clusters, each one
/// triangle on a single vertex at the origin.
pub fn page(clusters: &[(u32, i32)]) -> (Vec<u32>, Vec<u8>) {
    let grid = QuantGrid::new(GRID).unwrap();
    let (pos, idx) = ([[0f32; 3]], [0u32, 0, 0]);
    let pcs: Vec<PageCluster> = clusters
        .iter()
        .map(|&(g, r)| PageCluster::new(g, r, &pos, &idx))
        .collect();
    let mut ws = Workspace::default();
    let mem = page::encode_page(grid, &pcs, LAYOUT, &mut ws).unwrap();
    let disk = page::encode_disk_page(&mem, grid, DiskCodec::None, &mut ws).unwrap();
    (mem, disk)
}

/// Descriptor tight sphere of cluster `k` of a memory-form page.
pub fn sphere_words(mem: &[u32], k: usize) -> [u32; 4] {
    let at = page::PAGE_HEADER_WORDS + page::CLUSTER_WORDS * k + 10;
    [mem[at], mem[at + 1], mem[at + 2], mem[at + 3]]
}

/// Page record for `mem`/`disk` holding clusters from `first_cluster`.
pub fn page_record(
    mem: &[u32],
    disk: &[u8],
    first_cluster: u32,
    first_dep: u32,
    deps: u32,
    pinned: bool,
) -> Page {
    Page {
        len: disk.len() as u32,
        words: mem.len() as u32,
        first_cluster,
        clusters: mem[0],
        first_dep,
        deps,
        hash: v2::xxh64(disk, 0),
        pinned,
    }
}

/// Serialize: header, eight required chunks, then `disk`.
pub fn craft(c: &Craft) -> Vec<u8> {
    let grid = QuantGrid::new(GRID).unwrap();
    let lens = [
        v2::MESH_BYTES,
        v2::GROUP_BYTES * c.groups.len(),
        v2::NODE_BYTES * c.nodes.len(),
        v2::PAGE_BYTES * c.pages.len(),
        v2::DEPENDENCY_BYTES * c.deps.len(),
        v2::ATTRIBUTE_HEADER_BYTES,
        v2::CLUSTER_SPHERE_BYTES * c.spheres.len(),
        v2::CLUSTER_REFINED_BYTES * c.refined.len(),
    ];
    let tags = [
        v2::TAG_MESH,
        v2::TAG_GROUPS,
        v2::TAG_NODES,
        v2::TAG_PAGES,
        v2::TAG_DEPENDENCIES,
        v2::TAG_ATTRIBUTES,
        v2::TAG_CLUSTER_SPHERES,
        v2::TAG_CLUSTER_REFINED,
    ];
    let mut at = [0usize; 8];
    let mut cursor = v2::HEADER_BYTES + tags.len() * v2::CHUNK_ENTRY_BYTES;
    for (a, len) in at.iter_mut().zip(lens) {
        *a = cursor;
        cursor = (cursor + len).next_multiple_of(v2::CHUNK_ALIGN);
    }
    let resident = cursor;
    let mut o: Vec<u8> = Vec::with_capacity(resident + c.disk.len());
    let put = |o: &mut Vec<u8>, w: &[u32]| w.iter().for_each(|x| o.extend(x.to_le_bytes()));
    let align = |o: &mut Vec<u8>| o.resize(o.len().next_multiple_of(16), 0);
    o.extend_from_slice(b"MDAG");
    let header = [
        2,
        0,
        tags.len() as u32,
        c.pages.len() as u32,
        c.capacity,
        resident as u32,
        c.max_clusters,
        0,
        0,
        0,
        0,
        256,
        256,
    ];
    put(&mut o, &header);
    o.resize(v2::HEADER_BYTES, 0);
    for ((tag, a), len) in tags.iter().zip(at).zip(lens) {
        o.extend_from_slice(tag);
        put(&mut o, &[v2::CHUNK_REQUIRED, a as u32, len as u32]);
    }
    let clusters: u32 = c.pages.iter().map(|p| p.clusters).sum();
    let pinned = c.pages.iter().filter(|p| p.pinned).count() as u32;
    let f = f32::to_bits;
    let mut mesh = vec![
        c.levels,
        c.bvh,
        c.width,
        c.groups.len() as u32,
        clusters,
        c.nodes.len() as u32,
        c.pages.len() as u32,
        pinned,
        c.dag as u32,
        (c.dag >> 32) as u32,
        c.leaf as u32,
        (c.leaf >> 32) as u32,
    ];
    // Box and sphere of every vertex: all at the origin.
    mesh.extend([0; 10]);
    mesh.extend([
        grid.exponent() as u32,
        f(grid.max_displacement()),
        LAYOUT,
        1,
        0,
        0,
    ]);
    put(&mut o, &mesh);
    for g in &c.groups {
        put(
            &mut o,
            &[
                0,
                0,
                0,
                f(1.0),
                f(g.error),
                g.depth,
                g.first_cluster,
                g.clusters,
                g.first_page,
                g.pages,
                g.parents,
                0,
            ],
        );
    }
    align(&mut o);
    for n in &c.nodes {
        let w = [
            0,
            0,
            0,
            f(1.0),
            f(n.error),
            n.group as u32,
            n.child_offset,
            n.child_count,
        ];
        put(&mut o, &w);
    }
    align(&mut o);
    let mut offset = resident as u64;
    for p in &c.pages {
        put(
            &mut o,
            &[
                offset as u32,
                (offset >> 32) as u32,
                p.len,
                p.words,
                p.first_cluster,
                p.clusters,
                p.first_dep,
                p.deps,
                p.hash as u32,
                (p.hash >> 32) as u32,
                u32::from(p.pinned),
                0,
            ],
        );
        offset += u64::from(p.len);
    }
    align(&mut o);
    put(&mut o, &c.deps);
    align(&mut o);
    put(&mut o, &[0, LAYOUT, 0, 0]);
    align(&mut o);
    for s in &c.spheres {
        put(&mut o, s);
    }
    align(&mut o);
    for &r in &c.refined {
        o.extend(r.to_le_bytes());
    }
    align(&mut o);
    assert_eq!(o.len(), resident);
    let h = v2::prefix_hash(&o).unwrap();
    o[v2::RESIDENT_HASH_FIELD].copy_from_slice(&h.to_le_bytes());
    o.extend_from_slice(&c.disk);
    o
}

/// The review's E3 input: page 0 holds `k` clusters of group 0 cycling over
/// 17 refined groups, each of `span` single-cluster pages that are not in the
/// file (only the prefix and page 0's bytes are). Returns the file and page 0.
pub fn e3(k: usize, span: u32) -> (Vec<u8>, Vec<u8>) {
    let nq = 17u32;
    let cl: Vec<(u32, i32)> = (0..k).map(|i| (0, 1 + (i as u32 % nq) as i32)).collect();
    let (mem, disk) = page(&cl);
    let mut c = Craft {
        levels: 2,
        bvh: 2,
        width: nq,
        capacity: 1 << 20,
        max_clusters: v2::PageLimits::cluster_bound(1 << 20) as u32,
        dag: k as u64 + u64::from(nq * span),
        leaf: u64::from(nq * span),
        ..Default::default()
    };
    c.groups.push(Group {
        error: f32::MAX,
        depth: 1,
        first_cluster: 0,
        clusters: k as u32,
        first_page: 0,
        pages: 1,
        parents: 0,
    });
    for i in 0..nq {
        c.groups.push(Group {
            error: 1.0,
            depth: 0,
            first_cluster: k as u32 + i * span,
            clusters: span,
            first_page: 1 + i * span,
            pages: span,
            parents: 1,
        });
    }
    c.pages.push(page_record(&mem, &disk, 0, 0, 0, true));
    for j in 0..nq * span {
        c.pages.push(Page {
            len: 64,
            words: 20,
            first_cluster: k as u32 + j,
            clusters: 1,
            first_dep: j,
            deps: 1,
            hash: 0,
            pinned: false,
        });
    }
    c.deps = vec![0; (nq * span) as usize];
    c.nodes.push(Node {
        error: 1.0,
        group: -1,
        child_offset: 2,
        child_count: nq,
    });
    c.nodes.push(Node {
        error: f32::MAX,
        group: 0,
        child_offset: 0,
        child_count: 0,
    });
    for i in 0..nq {
        c.nodes.push(Node {
            error: 1.0,
            group: 1 + i as i32,
            child_offset: 0,
            child_count: 0,
        });
    }
    c.spheres = (0..k).map(|i| sphere_words(&mem, i)).collect();
    c.spheres.resize(k + (nq * span) as usize, [0; 4]);
    c.refined = cl.iter().map(|&(_, r)| r).collect();
    c.refined.resize(k + (nq * span) as usize, -1);
    c.disk = disk.clone();
    (craft(&c), disk)
}

/// The review's E4 input: a parent group of `np` one-cluster pinned pages
/// (the first `k` refine one child each) and a last page holding `k`
/// one-cluster child groups, so the last page depends on all `np` pages.
pub fn e4(k: usize, np: u32) -> Vec<u8> {
    let mut c = Craft {
        levels: 2,
        bvh: 2,
        width: k as u32,
        capacity: 1 << 20,
        max_clusters: v2::PageLimits::cluster_bound(1 << 20) as u32,
        dag: u64::from(np) + k as u64,
        leaf: u64::from(np),
        ..Default::default()
    };
    c.groups.push(Group {
        error: f32::MAX,
        depth: 1,
        first_cluster: 0,
        clusters: np,
        first_page: 0,
        pages: np,
        parents: 0,
    });
    for i in 0..k as u32 {
        c.groups.push(Group {
            error: 1.0,
            depth: 0,
            first_cluster: np + i,
            clusters: 1,
            first_page: np,
            pages: 1,
            parents: 1,
        });
    }
    let (plain_mem, plain_disk) = page(&[(0, -1)]);
    for j in 0..np {
        let refined = if (j as usize) < k { 1 + j as i32 } else { -1 };
        let (mem, disk) = if refined >= 0 {
            page(&[(0, refined)])
        } else {
            (plain_mem.clone(), plain_disk.clone())
        };
        c.pages.push(page_record(&mem, &disk, j, 0, 0, true));
        c.spheres.push(sphere_words(&mem, 0));
        c.refined.push(refined);
        c.disk.extend_from_slice(&disk);
    }
    let cl: Vec<(u32, i32)> = (0..k).map(|i| (1 + i as u32, -1)).collect();
    let (mem, disk) = page(&cl);
    c.pages.push(page_record(&mem, &disk, np, 0, np, false));
    c.spheres.extend((0..k).map(|i| sphere_words(&mem, i)));
    c.refined.extend(cl.iter().map(|_| -1));
    c.disk.extend_from_slice(&disk);
    c.deps = (0..np).collect();
    c.nodes.push(Node {
        error: 1.0,
        group: -1,
        child_offset: 2,
        child_count: k as u32,
    });
    c.nodes.push(Node {
        error: f32::MAX,
        group: 0,
        child_offset: 0,
        child_count: 0,
    });
    for i in 0..k {
        c.nodes.push(Node {
            error: 1.0,
            group: 1 + i as i32,
            child_offset: 0,
            child_count: 0,
        });
    }
    craft(&c)
}
