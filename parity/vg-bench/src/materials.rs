//! Real multi-material asset round-trip and error receipt.
use crate::{report::Json, source};
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        attributes::{Channel, Encoding},
        dag_format::v2,
        page::QuantGrid,
        v2::{self as builder, BuildMesh},
    },
    Attributes, Limits, Workspace,
};
use std::path::Path;

pub fn check(path: &str) -> Result<(), String> {
    let mut src = source::read(Path::new(path))?;
    src.fit(2.0);
    if src.materials.len() != src.indices.len() / 3 || src.materials.iter().copied().max().unwrap_or(0) == 0 {
        return Err("a multi-material triangle mesh is required".into());
    }
    let mut input = BuildMesh::new(&src.indices, &src.positions);
    input.normals = Some(&src.normals);
    input.materials = Some(&src.materials);
    let uvs: Vec<f32> = src.uvs.iter().flatten().copied().collect();
    input.attributes =
        Some(Attributes::from_interleaved(&uvs, src.positions.len(), 2, 2, 0).map_err(|e| format!("{e:?}"))?);
    input.attribute_weights = &[0., 0.];
    let mut ws = Workspace::new(Limits { max_bytes: usize::MAX, max_work: u64::MAX });
    let grid = QuantGrid::new(-22).map_err(|e| format!("{e:?}"))?;
    let built = builder::build(cl::default_config(128).map_err(|e| format!("{e:?}"))?, input, grid, &mut ws)
        .map_err(|e| format!("{e:?}"))?;
    let channels = [
        Channel { tag: 1, encoding: Encoding::UnormRange16x2, source_offset: 0 },
        Channel { tag: 2, encoding: Encoding::Oct16x2, source_offset: 2 },
    ];
    let values = Attributes::from_interleaved(
        &built.attributes,
        built.positions.len(),
        built.attribute_stride,
        built.attribute_stride,
        0,
    )
    .map_err(|e| format!("{e:?}"))?;
    let mut source = v2::Source::new(&built.groups, &built.clusters, &built.positions, &built.indices);
    source.channels = &channels;
    source.attributes = Some(values);
    source.materials = Some(&built.vertex_material);
    let (file, report) = v2::write(&source, &v2::WriteOptions::new(grid), &mut ws).map_err(|e| format!("{e:?}"))?;
    let r = v2::Reader::read_all(&file, &mut ws).map_err(|e| format!("{e:?}"))?;
    let mut checked = 0u64;
    let mut max_uv = 0f32;
    let mut max_normal = 0f32;
    let mut order = Vec::new();
    for group in built.groups.iter().rev() {
        order.extend(group.first_cluster as usize..(group.first_cluster + group.cluster_count) as usize);
    }
    for p in 0..r.mesh().page_count as usize {
        let meta = r.page(p).map_err(|e| format!("{e:?}"))?;
        let range = meta.disk_range();
        let mut words = vec![0; meta.memory_words as usize];
        let view = r
            .transcode_page(p, &file[range.start as usize..range.end as usize], &mut words, &mut ws)
            .map_err(|e| format!("{e:?}"))?;
        for c in 0..view.cluster_count() {
            let d = view.cluster(c).map_err(|e| format!("{e:?}"))?;
            let b = view.attributes(c).map_err(|e| format!("{e:?}"))?.ok_or("missing attributes")?;
            let mut pos = [[0f32; 3]; 256];
            view.decode_positions(c, &mut pos).map_err(|e| format!("{e:?}"))?;
            let cluster = &built.clusters[order[meta.first_cluster as usize + c]];
            let input =
                &built.indices[cluster.first_index as usize..(cluster.first_index + cluster.index_count) as usize];
            let mut local = Vec::new();
            for &i in input {
                if !local.contains(&i) {
                    local.push(i);
                }
            }
            if local.len() != d.vertex_count as usize {
                return Err("vertex count mismatch".into());
            }
            let mut lo = [f32::MAX; 2];
            let mut hi = [f32::MIN; 2];
            for &src in &local {
                for k in 0..2 {
                    let value = built.attributes[src as usize * built.attribute_stride + k];
                    lo[k] = lo[k].min(value);
                    hi[k] = hi[k].max(value);
                }
            }
            for (v, &src) in local.iter().enumerate() {
                if pos[v].map(f32::to_bits) != built.positions[src as usize].map(f32::to_bits) {
                    return Err("position mismatch".into());
                }
                let uv = b.decode(0, v).map_err(|e| format!("{e:?}"))?;
                let normal = b.decode(1, v).map_err(|e| format!("{e:?}"))?;
                let a = &built.attributes[src as usize * built.attribute_stride..];
                let u = (uv[0] - a[0]).abs().max((uv[1] - a[1]).abs());
                let len = a[2..5].iter().map(|x| x * x).sum::<f32>().sqrt();
                let n = normal[..3].iter().zip(&a[2..5]).map(|(x, y)| (x - y / len).powi(2)).sum::<f32>().sqrt();
                if n > 0.000491 {
                    return Err(format!("normal error {n}"));
                }
                let bound = (hi[0] - lo[0]).max(hi[1] - lo[1]) / 131070.
                    + lo.iter().chain(&hi).map(|x| x.abs() * f32::EPSILON * 4.).sum::<f32>();
                if u > bound {
                    return Err(format!("UV error {u} > {bound}"));
                }
                max_uv = max_uv.max(u);
                max_normal = max_normal.max(n);
                checked += 1;
            }
            for (t, triangle) in input.as_chunks::<3>().0.iter().enumerate() {
                let expected = built.vertex_material[triangle[0] as usize];
                if triangle.iter().any(|&v| built.vertex_material[v as usize] != expected)
                    || view.triangle_material(c, t).map_err(|e| format!("{e:?}"))? != expected
                {
                    return Err("material mismatch".into());
                }
            }
        }
    }
    println!(
        "{}",
        Json::new()
            .text("kind", "material_roundtrip")
            .with("source_triangles", src.triangles())
            .with("material_count", r.mesh().material_count)
            .with("vertices_checked", checked)
            .with("max_uv_error", max_uv)
            .with("max_normal_error", max_normal)
            .with("stuck_groups", built.stuck_groups)
            .with("material_stuck_groups", built.material_stuck_groups)
            .with("writer_stuck_groups", report.stuck_groups)
            .with("file_bytes", file.len())
    );
    Ok(())
}
