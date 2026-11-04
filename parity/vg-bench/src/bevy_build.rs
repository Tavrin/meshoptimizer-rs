//! Build with the reference meshlet builder (feature `bevy`) and save with its own asset saver.
use crate::source::Source;
use bevy_asset::{
    saver::{AssetSaver, SavedAsset},
    AssetPath, RenderAssetUsages,
};
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};
use bevy_pbr::experimental::meshlet::{MeshletMesh, MeshletMeshSaver};
use bevy_tasks::{block_on, AsyncComputeTaskPool, TaskPoolBuilder};
use std::time::Instant;

/// Result of one reference build: file bytes and phase times in milliseconds.
pub struct Built {
    pub file: Vec<u8>,
    pub build_ms: f64,
    pub save_ms: f64,
}

/// Convert `src` with `MeshletMesh::from_mesh` on a pool of `threads` workers.
pub fn build(src: &Source, threads: usize, quantization: u8) -> Result<Built, String> {
    AsyncComputeTaskPool::get_or_init(|| TaskPoolBuilder::new().num_threads(threads).build());
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, src.positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, src.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, src.uvs.clone());
    mesh.insert_indices(Indices::U32(src.indices.clone()));
    let t = Instant::now();
    let built = MeshletMesh::from_mesh(&mesh, quantization).map_err(|e| format!("{e:?}"))?;
    let build_ms = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let mut file: Vec<u8> = Vec::new();
    block_on(MeshletMeshSaver.save(&mut file, SavedAsset::from_asset(&built), &(), AssetPath::from("m.meshlet_mesh")))
        .map_err(|e| format!("{e:?}"))?;
    Ok(Built { file, build_ms, save_ms: t.elapsed().as_secs_f64() * 1e3 })
}
