//! This crate's parallel cluster-DAG build, its MDAG 1 file, and cut selection.
#![allow(clippy::field_reassign_with_default)]
use crate::{
    camera::View,
    cut::{CutMesh, Dag, Selection},
    report::{zstd19, Json},
    source::Source,
};
use meshoptimizer_rs::{
    clusterlod::{
        build_hierarchy, build_parallel_with_output,
        dag_format::{write, Reader, DEFAULT_PAGE_SIZE},
        default_config, Config, FlatDagCluster, FlatDagGroup, Group, LodBounds, Mesh, Node,
    },
    Attributes, Limits, Workspace,
};
use std::time::Instant;

/// Children per hierarchy node.
pub const NODE_WIDTH: usize = 8;

/// A built DAG in flat form plus its per-level spatial forest.
pub struct Ours {
    pub groups: Vec<FlatDagGroup>,
    pub clusters: Vec<FlatDagCluster>,
    pub indices: Vec<u32>,
    pub nodes: Vec<Node>,
    pub level_count: usize,
    pub positions: Vec<[f32; 3]>,
    pub v2_file: bool,
}

/// Wall times of the build phases in milliseconds.
pub struct Timing {
    pub dag_ms: f64,
    pub hierarchy_ms: f64,
}

fn unlimited() -> Workspace {
    Workspace::new(Limits { max_bytes: usize::MAX, max_work: u64::MAX })
}

/// Weight of the vertex normals as simplification attributes: the value the upstream
/// cluster-LOD demo (`nanite.cpp`) and the reference builder use, so both builders see the
/// same attribute error.
pub const NORMAL_WEIGHT: f32 = 0.5;

/// Build with the default configuration (128 triangles per cluster, normals weighted by
/// [`NORMAL_WEIGHT`]) on a pool of `threads` workers.
pub fn build(src: &Source, threads: usize) -> Result<(Ours, Timing), String> {
    build_with(src, threads, default_config(128).map_err(|e| format!("{e:?}"))?, Some(NORMAL_WEIGHT))
}

/// Build with an explicit configuration on a pool of `threads` workers. With
/// `normal_weight`, the vertex normals are simplification attributes of that weight.
pub fn build_with(
    src: &Source,
    threads: usize,
    config: Config,
    normal_weight: Option<f32>,
) -> Result<(Ours, Timing), String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().map_err(|e| e.to_string())?;
    pool.install(|| {
        let t0 = Instant::now();
        let normals: Vec<f32> = src.normals.iter().flatten().copied().collect();
        let weights = [normal_weight.unwrap_or(0.0); 3];
        let attributes = match normal_weight {
            Some(_) => {
                Some(Attributes::from_interleaved(&normals, src.normals.len(), 3, 3, 0).map_err(|e| format!("{e:?}"))?)
            }
            None => None,
        };
        let attribute_weights: &[f32] = if normal_weight.is_some() { &weights } else { &[] };
        let mut positions = src.positions.clone();
        meshoptimizer_rs::clusterlod::page::QuantGrid::new(-22)
            .map_err(|e| format!("{e:?}"))?
            .snap_positions(&mut positions)
            .map_err(|e| format!("{e:?}"))?;
        let mut ws = unlimited();
        let mut groups = Vec::new();
        let mut meta: Vec<Group> = Vec::new();
        let mut clusters = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        build_parallel_with_output(
            config,
            Mesh {
                indices: &src.indices,
                positions: &mut positions,
                attributes,
                vertex_lock: None,
                attribute_weights,
                attribute_protect_mask: 0,
            },
            |g, cs| {
                let gid = groups.len() as i32;
                let mut fg = FlatDagGroup::default();
                fg.center = g.simplified.center;
                fg.radius = g.simplified.radius;
                fg.error = g.simplified.error;
                fg.depth = g.depth;
                fg.first_cluster = clusters.len() as u32;
                fg.cluster_count = cs.len() as u32;
                groups.push(fg);
                meta.push(g);
                for c in cs {
                    let mut fc = FlatDagCluster::default();
                    fc.first_index = indices.len() as u32;
                    fc.index_count = c.indices.len() as u32;
                    fc.group = gid as u32;
                    fc.refined = c.refined;
                    fc.center = c.bounds.center;
                    fc.radius = c.bounds.radius;
                    clusters.push(fc);
                    indices.extend_from_slice(&c.indices);
                }
                Ok(gid)
            },
            &mut ws,
        )
        .map_err(|e| format!("{e:?}"))?;
        let dag_ms = t0.elapsed().as_secs_f64() * 1e3;
        let level_count = meta.iter().map(|g| g.depth as usize + 1).max().unwrap_or(0);
        let t1 = Instant::now();
        let nodes = build_hierarchy(&meta, NODE_WIDTH, level_count, &mut ws).map_err(|e| format!("{e:?}"))?;
        let hierarchy_ms = t1.elapsed().as_secs_f64() * 1e3;
        Ok((
            Ours { groups, clusters, indices, nodes, level_count, positions, v2_file: true },
            Timing { dag_ms, hierarchy_ms },
        ))
    })
}

impl Ours {
    /// Serialise as MDAG 1; returns the file and the write time in milliseconds.
    pub fn write_file(&self, src: &Source) -> Result<(Vec<u8>, f64), String> {
        let mut ws = unlimited();
        let t = Instant::now();
        let bytes = if self.v2_file {
            use meshoptimizer_rs::clusterlod::{
                attributes::{Channel, Encoding},
                dag_format::v2,
            };
            let channels = [
                Channel { tag: 1, encoding: Encoding::Oct16x2, source_offset: 0 },
                Channel { tag: 2, encoding: Encoding::UnormRange16x2, source_offset: 3 },
            ];
            let values: Vec<f32> =
                src.normals.iter().zip(&src.uvs).flat_map(|(n, u)| [n[0], n[1], n[2], u[0], u[1]]).collect();
            let attrs =
                Attributes::from_interleaved(&values, src.positions.len(), 5, 5, 0).map_err(|e| format!("{e:?}"))?;
            let mut source = v2::Source::new(&self.groups, &self.clusters, &self.positions, &self.indices);
            source.channels = &channels;
            source.attributes = Some(attrs);
            v2::write(
                &source,
                &v2::WriteOptions::new(
                    meshoptimizer_rs::clusterlod::page::QuantGrid::new(-22).map_err(|e| format!("{e:?}"))?,
                ),
                &mut ws,
            )
            .map_err(|e| format!("{e:?}"))?
            .0
        } else {
            write(&self.groups, &self.clusters, &self.positions, &self.indices, DEFAULT_PAGE_SIZE, &mut ws)
                .map_err(|e| format!("{e:?}"))?
        };
        Ok((bytes, t.elapsed().as_secs_f64() * 1e3))
    }

    fn tris(&self, c: usize) -> u64 {
        u64::from(self.clusters[c].index_count) / 3
    }

    /// Depth of the deepest root-to-leaf chain of hierarchy nodes.
    pub fn hierarchy_depth(&self) -> usize {
        let mut best = 0;
        let mut stack: Vec<(usize, usize)> = (0..self.level_count.min(self.nodes.len())).map(|r| (r, 1)).collect();
        while let Some((n, d)) = stack.pop() {
            best = best.max(d);
            let node = &self.nodes[n];
            if node.group < 0 {
                for k in 0..node.child_count as usize {
                    stack.push((node.child_offset as usize + k, d + 1));
                }
            }
        }
        best
    }

    fn ok(&self, v: &View, thr: f64, c: [f32; 3], r: f32, e: f32) -> bool {
        v.error_px(c, r, e) < thr
    }

    fn self_ok(&self, v: &View, thr: f64, c: usize) -> (bool, f64) {
        let cl = &self.clusters[c];
        if cl.refined < 0 {
            return (true, 0.0);
        }
        let g = &self.groups[cl.refined as usize];
        let px = v.error_px(g.center, g.radius, g.error);
        (px < thr, px)
    }

    /// Rebuild the spatial forest from the current group bounds (after errors change).
    pub fn rebuild_hierarchy(&mut self) -> Result<(), String> {
        let meta: Vec<Group> = self
            .groups
            .iter()
            .map(|g| {
                let mut m = Group::default();
                m.depth = g.depth;
                m.simplified = LodBounds { center: g.center, radius: g.radius, error: g.error };
                m
            })
            .collect();
        let mut ws = unlimited();
        self.nodes = build_hierarchy(&meta, NODE_WIDTH, self.level_count, &mut ws).map_err(|e| format!("{e:?}"))?;
        Ok(())
    }

    /// Self error of a cluster in scene units (zero for source clusters).
    fn own_error(&self, c: usize) -> f64 {
        let r = self.clusters[c].refined;
        if r < 0 {
            0.0
        } else {
            f64::from(self.groups[r as usize].error)
        }
    }

    fn parent_ok(&self, v: &View, thr: f64, g: usize) -> bool {
        let g = &self.groups[g];
        self.ok(v, thr, g.center, g.radius, g.error)
    }
}

impl Dag for Ours {
    fn cut(&self, v: &View, thr: f64) -> Selection {
        let mut s = Selection { min_parent_error: f32::MAX, ..Selection::default() };
        for c in 0..self.clusters.len() {
            let g = self.clusters[c].group as usize;
            if self.parent_ok(v, thr, g) {
                continue;
            }
            let (ok, px) = self.self_ok(v, thr, c);
            if ok {
                s.drawn.push(c as u32);
                s.claimed_px = s.claimed_px.max(px);
                s.claimed_world = s.claimed_world.max(self.own_error(c));
                s.min_parent_error = s.min_parent_error.min(self.groups[g].error);
            }
        }
        s.tested = self.clusters.len() as u64;
        s
    }

    fn traverse(&self, v: &View, thr: f64, frustum: bool) -> Selection {
        let mut s = Selection { min_parent_error: f32::MAX, ..Selection::default() };
        let mut stack: Vec<usize> = (0..self.level_count.min(self.nodes.len())).collect();
        while let Some(n) = stack.pop() {
            s.nodes += 1;
            let node = &self.nodes[n];
            let b = node.bounds;
            if frustum && !v.sphere_visible(b.center, b.radius) {
                continue;
            }
            if self.ok(v, thr, b.center, b.radius, b.error) {
                continue;
            }
            if node.group < 0 {
                for k in 0..node.child_count as usize {
                    stack.push(node.child_offset as usize + k);
                }
                continue;
            }
            let g = node.group as usize;
            if self.parent_ok(v, thr, g) {
                continue;
            }
            let fg = &self.groups[g];
            for c in fg.first_cluster as usize..(fg.first_cluster + fg.cluster_count) as usize {
                s.tested += 1;
                let (ok, px) = self.self_ok(v, thr, c);
                if ok {
                    s.drawn.push(c as u32);
                    s.claimed_px = s.claimed_px.max(px);
                    s.claimed_world = s.claimed_world.max(self.own_error(c));
                    s.min_parent_error = s.min_parent_error.min(fg.error);
                }
            }
        }
        s
    }

    fn triangles(&self, drawn: &[u32]) -> u64 {
        drawn.iter().map(|&c| self.tris(c as usize)).sum()
    }

    fn cut_mesh(&self, drawn: &[u32]) -> CutMesh {
        let mut indices = Vec::new();
        for &c in drawn {
            let cl = &self.clusters[c as usize];
            let a = cl.first_index as usize;
            indices.extend_from_slice(&self.indices[a..a + cl.index_count as usize]);
        }
        CutMesh { positions: Some(self.positions.clone()), indices }
    }

    fn is_leaf(&self, c: u32) -> bool {
        self.clusters[c as usize].refined < 0
    }
}

/// Static metrics of an MDAG 1 build: sizes, pages, DAG shape, page decode speed.
pub fn shape(o: &Ours, src: &Source, label: &Json) -> Result<String, String> {
    let (file, _) = o.write_file(src)?;
    let mut ws = unlimited();
    if o.v2_file {
        return shape_v2(o, src, label, &file);
    }
    let r = Reader::read(&file, &mut ws).map_err(|e| format!("{e:?}"))?;
    let pages = r.page_count();
    let mut fill = 0f64;
    let mut stored = 0u64;
    for p in 0..pages {
        let range = r.page_range(p).map_err(|e| format!("{e:?}"))?;
        fill += range.len() as f64 / DEFAULT_PAGE_SIZE as f64;
        stored += range.len() as u64;
    }
    let (mut raw, mut total_tris, mut leaf_tris) = (0u64, 0u64, 0u64);
    let (mut tri_counts, mut vert_counts) = (Vec::new(), Vec::new());
    for (c, cl) in o.clusters.iter().enumerate() {
        let tris = o.tris(c);
        total_tris += tris;
        if cl.refined < 0 {
            leaf_tris += tris;
        }
        tri_counts.push(tris as f64);
        let mut v: Vec<u32> = o.indices[cl.first_index as usize..(cl.first_index + cl.index_count) as usize].to_vec();
        v.sort_unstable();
        v.dedup();
        vert_counts.push(v.len() as f64);
        raw += v.len() as u64 * 16 + u64::from(cl.index_count) * 4;
    }
    let metadata = 32 + 32 * o.groups.len() + 64 * o.clusters.len() + 16 * pages;
    let stuck = o.groups.iter().filter(|g| g.error == f32::MAX).count();
    // Page decode speed: median over pages of raw bytes per second.
    let mut rates = Vec::new();
    for p in 0..pages {
        let range = r.page_range(p).map_err(|e| format!("{e:?}"))?;
        let t = Instant::now();
        let page = r.decode_page(p, &file[range], &mut ws).map_err(|e| format!("{e:?}"))?;
        std::hint::black_box(&page);
        rates.push(t.elapsed().as_secs_f64());
    }
    rates.sort_by(f64::total_cmp);
    let page_decode_us = rates.get(rates.len() / 2).copied().unwrap_or(0.0) * 1e6;
    let decode_mb_s = if page_decode_us > 0.0 { (raw as f64 / pages.max(1) as f64) / page_decode_us } else { 0.0 };
    tri_counts.sort_by(f64::total_cmp);
    vert_counts.sort_by(f64::total_cmp);
    let p5 = |v: &[f64]| v.get(((v.len().max(1) - 1) as f64 * 0.05) as usize).copied().unwrap_or(0.0);
    let avg = |v: &[f64]| v.iter().sum::<f64>() / v.len().max(1) as f64;
    Ok(label
        .clone()
        .text("kind", "shape")
        .with("source_triangles", src.triangles())
        .with("clusters", o.clusters.len())
        .with("groups", o.groups.len())
        .with("levels", o.level_count)
        .with("hierarchy_depth", o.hierarchy_depth())
        .with("total_triangles", total_tris)
        .with("leaf_triangles", leaf_tris)
        .with("total_over_leaf", total_tris as f64 / leaf_tris.max(1) as f64)
        .with("tris_per_cluster_mean", avg(&tri_counts))
        .with("tris_per_cluster_p5", p5(&tri_counts))
        .with("verts_per_cluster_mean", avg(&vert_counts))
        .with("verts_per_cluster_p5", p5(&vert_counts))
        .with("pages", pages)
        .with("page_fill_mean", fill / pages.max(1) as f64)
        .with("page_payload_bytes", stored)
        .with("stuck_groups", stuck)
        .with("file_bytes", file.len())
        .with("file_zstd19_bytes", zstd19(&file))
        .with("metadata_bytes", metadata)
        .with("raw_geometry_bytes", raw)
        .with("resident_bytes", metadata as u64 + raw)
        .with("page_decode_median_us", page_decode_us)
        .with("page_decode_mb_per_s", decode_mb_s)
        .to_string())
}

/// Build through the library's default MDAG 2 policy, with the measured floor.
pub fn build_v2(src: &Source, threads: usize, config: Config) -> Result<(Ours, Timing), String> {
    use meshoptimizer_rs::clusterlod::{
        page::QuantGrid,
        v2::{self, BuildMesh},
    };
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().map_err(|e| e.to_string())?;
    pool.install(|| {
        let mut input = BuildMesh::new(&src.indices, &src.positions);
        input.normals = Some(&src.normals);
        let mut ws = unlimited();
        let t = Instant::now();
        let built = v2::build(config, input, QuantGrid::new(-22).map_err(|e| format!("{e:?}"))?, &mut ws)
            .map_err(|e| format!("{e:?}"))?;
        let dag_ms = t.elapsed().as_secs_f64() * 1e3;
        let mut positions = src.positions.clone();
        for (v, &s) in built.vertex_source.iter().enumerate() {
            positions[s as usize] = built.positions[v];
        }
        let indices = built.indices.iter().map(|&i| built.vertex_source[i as usize]).collect();
        let level_count = built.groups.iter().map(|g| g.depth as usize + 1).max().unwrap_or(0);
        let mut o = Ours {
            groups: built.groups,
            clusters: built.clusters,
            indices,
            nodes: Vec::new(),
            level_count,
            positions,
            v2_file: true,
        };
        let t = Instant::now();
        o.rebuild_hierarchy()?;
        let hierarchy_ms = t.elapsed().as_secs_f64() * 1e3;
        Ok((o, Timing { dag_ms, hierarchy_ms }))
    })
}
fn shape_v2(o: &Ours, src: &Source, label: &Json, file: &[u8]) -> Result<String, String> {
    use meshoptimizer_rs::clusterlod::dag_format::v2;
    let mut ws = unlimited();
    let r = v2::Reader::read_all(file, &mut ws).map_err(|e| format!("{e:?}"))?;
    let m = r.mesh();
    let mut words = 0u64;
    let mut pinned = 0u64;
    for p in 0..m.page_count as usize {
        let page = r.page(p).map_err(|e| format!("{e:?}"))?;
        words += u64::from(page.memory_words);
        if p < m.pinned_page_count as usize {
            pinned += u64::from(page.memory_words) * 4;
        }
    }
    Ok(label
        .clone()
        .text("kind", "shape")
        .with("clusters", o.clusters.len())
        .with("source_triangles", src.triangles())
        .with("total_triangles", m.dag_triangles)
        .with("levels", m.level_count)
        .with("hierarchy_depth", m.bvh_depth)
        .with("pages", m.page_count)
        .with("pinned_bytes", pinned)
        .with("file_bytes", file.len())
        .with("file_zstd19_bytes", zstd19(file))
        .with("resident_bytes", v2::resident_len(file).map_err(|e| format!("{e:?}"))? as u64 + words * 4)
        .to_string())
}
