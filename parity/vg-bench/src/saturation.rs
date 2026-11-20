//! Offline diagnosis of an embedded container using the crate reader.
use crate::{report::Json, source};
use meshoptimizer_rs::{clusterlod::dag_format::v2::Reader, Workspace};
use std::{fs, path::Path};

pub fn raw_prep(args: &[String]) -> Result<(), String> {
    let [input, output] = args else {
        return Err("raw-prep input output".into());
    };
    let mut src = source::read(Path::new(input))?;
    if src.normals.is_empty() {
        src.derive_normals();
    }
    if src.uvs.is_empty() {
        src.derive_uvs();
    }
    println!(
        "{}",
        Json::new().text("kind", "raw_source").with("triangles", src.triangles()).with("radius", src.radius())
    );
    src.save(Path::new(output))
}

pub fn cooked(args: &[String]) -> Result<(), String> {
    let path = args.first().ok_or("cooked input")?;
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let start = bytes.windows(4).position(|w| w == b"MDAG").ok_or("no container")?;
    let file = &bytes[start..];
    let r = Reader::open(file).map_err(|e| format!("{e:?}"))?;
    let end = (0..r.mesh().page_count as usize)
        .try_fold(r.resident_len(), |end, p| r.page(p).map(|page| end.max(page.disk_range().end as usize)))
        .map_err(|e| format!("{e:?}"))?;
    let file = file.get(..end).ok_or("truncated container")?;
    let mut ws = Workspace::default();
    let r = Reader::read_all(file, &mut ws).map_err(|e| format!("{e:?}"))?;
    println!(
        "{}",
        Json::new()
            .text("kind", "cooked_mesh")
            .with("grid_exponent", r.mesh().grid.exponent())
            .with("max_displacement", r.mesh().max_displacement)
            .with("leaf_triangles", r.mesh().leaf_triangles)
            .with("sphere_radius", r.mesh().sphere_radius)
    );
    let mut triangles = vec![0u64; r.mesh().group_count as usize];
    let mut words = vec![0u32; r.page_capacity() / 4];
    let mut cluster_tris = vec![0u32; r.mesh().cluster_count as usize];
    let mut cluster_groups = vec![0usize; cluster_tris.len()];
    let mut geometry = crate::ours::Ours {
        groups: Vec::new(),
        clusters: vec![Default::default(); cluster_tris.len()],
        indices: Vec::new(),
        positions: Vec::new(),
        diagnostics: Vec::new(),
        nodes: Vec::new(),
        level_count: r.mesh().level_count as usize,
        v2_file: true,
        grid_exponent: r.mesh().grid.exponent(),
    };
    let steps = args.iter().any(|a| a == "--steps");
    for p in 0..r.mesh().page_count as usize {
        let range = r.page(p).map_err(|e| format!("{e:?}"))?.disk_range();
        let view = r
            .transcode_page(p, &file[range.start as usize..range.end as usize], &mut words, &mut ws)
            .map_err(|e| format!("{e:?}"))?;
        // Clusters in each page are consecutive in the resident group table.
        let page = r.page(p).map_err(|e| format!("{e:?}"))?;
        for k in 0..view.cluster_count() {
            let id = page.first_cluster as usize + k;
            let g = (0..triangles.len())
                .find(|&g| r.group(g).is_ok_and(|g| g.clusters().contains(&(id as u32))))
                .ok_or("missing group")?;
            cluster_tris[id] = view.cluster(k).map_err(|e| format!("{e:?}"))?.triangle_count;
            cluster_groups[id] = g;
            triangles[g] += u64::from(cluster_tris[id]);
            if steps {
                let desc = view.cluster(k).map_err(|e| format!("{e:?}"))?;
                let first = geometry.positions.len();
                geometry.positions.resize(first + desc.vertex_count as usize, [0.; 3]);
                view.decode_positions(k, &mut geometry.positions[first..]).map_err(|e| format!("{e:?}"))?;
                let mut local = vec![[0u8; 3]; desc.triangle_count as usize];
                view.decode_triangles(k, &mut local).map_err(|e| format!("{e:?}"))?;
                let cluster = &mut geometry.clusters[id];
                cluster.group = g as u32;
                cluster.refined = r.cluster_refined(id).map_err(|e| format!("{e:?}"))?;
                cluster.first_index = geometry.indices.len() as u32;
                cluster.index_count = desc.triangle_count * 3;
                geometry.indices.extend(local.into_iter().flatten().map(|v| first as u32 + u32::from(v)));
            }
        }
    }
    let health = meshoptimizer_rs::clusterlod::v2::health_from_reader(&r, &cluster_tris, 3, &mut ws)
        .map_err(|e| format!("{e:?}"))?;
    println!(
        "{}",
        Json::new()
            .text("kind", "cooked_health")
            .with("pass", health.passed)
            .with("chains", health.equal_error_chains.len())
            .with("pinned", health.pinned_groups.len())
    );
    for depth in 0..r.mesh().level_count {
        let mut errors = Vec::new();
        let mut tris = 0;
        let mut pinned = 0;
        for (id, &n) in triangles.iter().enumerate() {
            let g = r.group(id).map_err(|e| format!("{e:?}"))?;
            if g.depth == depth {
                tris += n;
                if g.error == f32::MAX {
                    pinned += 1;
                } else {
                    errors.push(g.error);
                }
                println!(
                    "{}",
                    Json::new()
                        .text("kind", "cooked_group")
                        .with("depth", depth)
                        .with("group", id)
                        .with("error", g.error)
                        .with("triangles", n)
                        .with("parents", g.parent_count)
                );
            }
        }
        errors.sort_by(f32::total_cmp);
        println!(
            "{}",
            Json::new()
                .text("kind", "cooked_level")
                .with("depth", depth)
                .with("groups", errors.len() + pinned)
                .with("triangles", tris)
                .with("pinned", pinned)
                .with("min", errors.first().copied().unwrap_or(0.0))
                .with("median", errors.get(errors.len() / 2).copied().unwrap_or(0.0))
                .with("max", errors.last().copied().unwrap_or(0.0))
        );
    }
    for span in [32.0, 64.0, 128.0] {
        let distance = crate::camera::distance_for_span(f64::from(r.mesh().sphere_radius), span);
        let mut total = 0u64;
        let dirs = crate::camera::directions(64);
        for dir in &dirs {
            let mut view = crate::camera::View::orbit(*dir, distance);
            for k in 0..3 {
                view.eye[k] += f64::from(r.mesh().sphere_center[k]);
            }
            for (c, &n) in cluster_tris.iter().enumerate() {
                let parent = r.group(cluster_groups[c]).map_err(|e| format!("{e:?}"))?;
                if view.error_px(parent.center, parent.radius, parent.error) < 1.0 {
                    continue;
                }
                let refined = r.cluster_refined(c).map_err(|e| format!("{e:?}"))?;
                let own_ok = if refined < 0 {
                    true
                } else {
                    let own = r.group(refined as usize).map_err(|e| format!("{e:?}"))?;
                    view.error_px(own.center, own.radius, own.error) < 1.0
                };
                if own_ok {
                    total += u64::from(n);
                }
            }
        }
        println!(
            "{}",
            Json::new()
                .text("kind", "far_field")
                .with("span", span)
                .with("threshold_px", 1)
                .with("distance", distance)
                .with("mean_triangles", total as f64 / dirs.len() as f64)
        );
    }
    if steps {
        for id in 0..r.mesh().group_count as usize {
            let g = r.group(id).map_err(|e| format!("{e:?}"))?;
            let mut group = meshoptimizer_rs::clusterlod::FlatDagGroup::default();
            group.center = g.center;
            group.radius = g.radius;
            group.error = g.error;
            group.depth = g.depth as i32;
            group.first_cluster = g.first_cluster;
            group.cluster_count = g.cluster_count;
            geometry.groups.push(group);
        }
        let src = source::Source { positions: geometry.positions.clone(), ..Default::default() };
        for line in crate::diag::group_steps(&geometry, &src, 8, &Json::new().text("mesh", "cooked"))? {
            println!("{line}");
        }
    }
    Ok(())
}

/// Rebuild a cached source at its cook grid, preserving coordinates and normal inputs.
pub fn rebuild(args: &[String]) -> Result<(), String> {
    use meshoptimizer_rs::{
        clusterlod::{
            dag_format::v2 as format,
            default_config,
            page::QuantGrid,
            v2::{self, BuildMesh},
        },
        Limits,
    };
    let positional: Vec<String> = args.iter().filter(|a| !a.starts_with("--")).cloned().collect();
    let [input, output, exponent] = positional.as_slice() else {
        return Err("rebuild input.vgm output.mdag grid-exponent [--no-normals]".into());
    };
    let src = source::Source::load(Path::new(input))?;
    let grid = QuantGrid::new(exponent.parse().map_err(|_| "invalid grid")?).map_err(|e| format!("{e:?}"))?;
    let mut mesh = BuildMesh::new(&src.indices, &src.positions);
    if !args.iter().any(|a| a == "--no-normals") {
        mesh.normals = Some(&src.normals);
    }
    let mut ws = Workspace::new(Limits { max_bytes: usize::MAX, max_work: u64::MAX });
    let config = default_config(128).map_err(|e| format!("{e:?}"))?;
    let (built, diagnostics) =
        v2::build_with_diagnostics(config, mesh, grid, &mut ws).map_err(|e| format!("{e:?}"))?;
    let file_source = format::Source::new(&built.groups, &built.clusters, &built.positions, &built.indices);
    let (file, _) =
        format::write(&file_source, &format::WriteOptions::new(grid), &mut ws).map_err(|e| format!("{e:?}"))?;
    fs::write(output, file).map_err(|e| e.to_string())?;
    let level_count = built.groups.iter().map(|g| g.depth as usize + 1).max().unwrap_or(0);
    let mut o = crate::ours::Ours {
        grid_exponent: grid.exponent(),
        groups: built.groups,
        clusters: built.clusters,
        indices: built.indices,
        positions: built.positions,
        diagnostics,
        nodes: Vec::new(),
        level_count,
        v2_file: true,
    };
    o.rebuild_hierarchy()?;
    let label = Json::new().text("mesh", "consumer-source");
    for line in crate::diag::plateau(&o, &label) {
        println!("{line}");
    }
    for line in crate::diag::group_steps(&o, &src, 8, &label)? {
        println!("{line}");
    }
    cooked(&[output.clone()])
}
