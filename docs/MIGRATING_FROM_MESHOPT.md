# Migrating from meshopt

This guide maps `meshopt` 0.7.0 to `meshoptimizer-rs` 0.2.16 (`meshoptimizer_rs` in Rust).
Change the Cargo dependency to `meshoptimizer-rs = "0.2"`. Both crates produce the same output: `meshopt` 0.7.0 wraps
meshoptimizer 1.3, and this crate is a byte-identical Rust port of it.

0.2.16 adds `weld_vertices`, `orient_faces` and `fill_holes` for opt-in repair; see the [repair recipe](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes).
With `parallel` + `clusterlod`, `clusterlod::build_parallel` processes groups within one mesh; see the [parallel recipe](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#parallel-groups-within-one-cluster-lod-mesh).
`codec::IndexSequenceDecoder` accepts arriving chunks; keep output provisional until `finish()` succeeds, as in the [streaming recipe](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#streaming-index-sequences).

## API map

The table covers every public item of `meshopt` 0.7.0, including its raw `ffi` module (285 entries). Names on the
left are relative to `meshopt`, names on the right to `meshoptimizer_rs`. Where a row lists several items separated
by semicolons, they map in order. Raw `ffi` functions map to checked Rust functions; this crate has no C ABI.
`_decoder` variants map to the plain function: decode your positions into a slice first. `clusterlod::` items
need the `clusterlod` feature. See the [rustdoc](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/) for
signatures.

| meshopt 0.7.0 | meshoptimizer-rs 0.2.16 equivalent or reason |
|---|---|
| `VertexCacheStatistics`, `ffi::meshopt_VertexCacheStatistics`; `VertexFetchStatistics`, `ffi::meshopt_VertexFetchStatistics`; `OverdrawStatistics`, `ffi::meshopt_OverdrawStatistics`; `CoverageStatistics`, `ffi::meshopt_CoverageStatistics` | `VertexCacheStatistics`; `VertexFetchStatistics`; `OverdrawStatistics`; `CoverageStatistics` (respectively) |
| `analyze_vertex_cache`, `ffi::meshopt_analyzeVertexCache`; `analyze_vertex_fetch`, `ffi::meshopt_analyzeVertexFetch`; `analyze_overdraw_decoder`, `analyze_overdraw`, `ffi::meshopt_analyzeOverdraw`; `analyze_coverage_decoder`, `analyze_coverage`, `ffi::meshopt_analyzeCoverage` | `analyze_vertex_cache`; `analyze_vertex_fetch`; `analyze_overdraw`; `analyze_coverage` (respectively) |
| `Bounds`, `Sphere`, `ffi::meshopt_Bounds` | `Bounds`. Use center and radius. |
| `Meshlet`, `Meshlets::get`, `Meshlets::iter` | `LocalMeshlet`. Owns vertex and triangle vectors; our Meshlet is a descriptor. Slice vertices and triangles using each descriptor; see example. |
| `Meshlets` | `Meshlets` |
| `build_meshlets`, `ffi::meshopt_buildMeshlets` | `build_meshlets` |
| `build_meshlets_scan`, `ffi::meshopt_buildMeshletsScan` | `build_meshlets_scan` |
| `build_meshlets_flex`, `ffi::meshopt_buildMeshletsFlex` | `build_meshlets_flex` |
| `build_meshlets_spatial`, `ffi::meshopt_buildMeshletsSpatial` | `build_meshlets_spatial` |
| `optimize_meshlet_level`, `ffi::meshopt_optimizeMeshletLevel` | `optimize_meshlet_level_in_place` |
| `extract_meshlet_indices`, `ffi::meshopt_extractMeshletIndices` | `extract_meshlet_indices` |
| `partition_clusters`, `partition_clusters_with_positions`, `partition_clusters_with_decoder`, `ffi::meshopt_partitionClusters` | `partition_clusters`. Pass None or Some(positions); returns Partitions. |
| `compute_cluster_bounds`, `compute_cluster_bounds_decoder`, `ffi::meshopt_computeClusterBounds` | `compute_cluster_bounds` |
| `PositionDataAdapter`, `DecodePosition`, `VertexDataAdapter`, `DecodePosition::decode_position` | `Positions`. Use from_packed, from_interleaved (f32 units), or from_bytes (byte units). For DecodePosition, collect decoded positions first. |
| `RadiusDataAdapter` | `Attributes`. One component per radius. |
| `compute_sphere_bounds`, `ffi::meshopt_computeSphereBounds` | `compute_sphere_bounds` |
| `compute_meshlet_bounds`, `compute_meshlet_bounds_decoder`, `ffi::meshopt_computeMeshletBounds` | `compute_meshlet_bounds` |
| `ClodConfig`, `ffi::clodConfig`; `ClodBounds`, `ffi::clodBounds`; `ClodGroup`, `ffi::clodGroup`; `ClodNode`, `ffi::clodNode` | `clusterlod::Config`; `clusterlod::LodBounds`; `clusterlod::Group`; `clusterlod::Node` (respectively) |
| `ClodMesh`, `ffi::clodMesh`, `ClodMesh::new` | `clusterlod::Mesh`. Construct fields explicitly; positions are mutable. |
| `ClodCluster`, `ffi::clodCluster` | `clusterlod::Cluster` |
| `clod_default_config`, `ffi::clodDefaultConfig` | `clusterlod::default_config` |
| `clod_default_config_rt`, `ffi::clodDefaultConfigRT` | `clusterlod::default_config_rt` |
| `clod_build`, `ffi::clodBuild`, `ffi::clodOutput` | `clusterlod::build_with_output`. Closure takes Group and a cluster slice, returning Result&lt;i32, Error&gt;; clusterlod::build instead collects groups. |
| `clod_local_indices`, `ffi::clodLocalIndices` | `clusterlod::local_indices` |
| `clod_build_hierarchy`, `ffi::clodBuildHierarchy` | `clusterlod::build_hierarchy` |
| `encode_index_buffer`, `ffi::meshopt_encodeIndexBuffer` | `codec::encode_index_buffer` |
| `decode_index_buffer`, `ffi::meshopt_decodeIndexBuffer` | `codec::decode_index_buffer` |
| `encode_index_version`, `ffi::meshopt_encodeIndexVersion` | `codec::IndexEncoding`. Set new(version) per call; no global setter. |
| `decode_index_version`, `ffi::meshopt_decodeIndexVersion` | `codec::decode_index_version` |
| `encode_index_sequence`, `ffi::meshopt_encodeIndexSequence` | `codec::encode_index_sequence` |
| `decode_index_sequence`, `ffi::meshopt_decodeIndexSequence` | `codec::decode_index_sequence` |
| `encode_vertex_buffer`, `encode_vertex_buffer_level`, `ffi::meshopt_encodeVertexBuffer`, `ffi::meshopt_encodeVertexBufferLevel` | `codec::encode_vertex_buffer`. Set codec::VertexEncoding per call. |
| `decode_vertex_buffer`, `ffi::meshopt_decodeVertexBuffer` | `codec::decode_vertex_buffer` |
| `decode_vertex_version`, `ffi::meshopt_decodeVertexVersion` | `codec::decode_vertex_version` |
| `EncodeExpMode`, `ffi::meshopt_EncodeExpMode` | `codec::ExpMode` |
| `decode_filter_oct`, `ffi::meshopt_decodeFilterOct` | `codec::decode_filter_oct` |
| `decode_filter_quat`, `ffi::meshopt_decodeFilterQuat` | `codec::decode_filter_quat` |
| `decode_filter_exp`, `ffi::meshopt_decodeFilterExp` | `codec::decode_filter_exp` |
| `decode_filter_color`, `ffi::meshopt_decodeFilterColor` | `codec::decode_filter_color` |
| `encode_filter_oct`, `ffi::meshopt_encodeFilterOct` | `codec::encode_filter_oct` |
| `encode_filter_quat`, `ffi::meshopt_encodeFilterQuat` | `codec::encode_filter_quat` |
| `encode_filter_exp`, `ffi::meshopt_encodeFilterExp` | `codec::encode_filter_exp` |
| `encode_filter_color`, `ffi::meshopt_encodeFilterColor` | `codec::encode_filter_color` |
| `IndexType`, `FilterComponent` | not available. No typed codec traits; use byte buffers and explicit strides (2 or 4 for indices). |
| `OpacityStates` | `u8`. Pass 2 or 4. |
| `encode_meshlet`, `ffi::meshopt_encodeMeshlet` | `codec::encode_meshlet` |
| `decode_meshlet`, `ffi::meshopt_decodeMeshlet` | `codec::decode_meshlet` |
| `decode_meshlet_raw`, `ffi::meshopt_decodeMeshletRaw` | `codec::decode_meshlet_raw` |
| `EncodeHeader`, `EncodeObject` | not available. No OPTM container structs; raw buffer codecs only. |
| `calc_pos_offset_and_scale`, `calc_pos_offset_and_scale_inverse`, `calc_uv_offset_and_scale`, `calc_uv_offset_and_scale_inverse` | not available. No packing-range helpers; compute offsets and scales in application code. |
| `Result` | `core::result::Result`. No crate Result alias. |
| `Error` | `Error` |
| `VertexStream`, `ffi::meshopt_Stream` | `VertexStream` |
| `OpacityMapMeasure` | `OpacityMapMeasure`. indices becomes omm_indices. |
| `opacity_map_measure`, `ffi::meshopt_opacityMapMeasure` | `opacity_map_measure` |
| `opacity_map_entry_size`, `ffi::meshopt_opacityMapEntrySize` | `opacity_map_entry_size` |
| `opacity_map_layout` | not available. No layout helper; accumulate checked opacity_map_entry_size results. |
| `AlphaTexture` | not available. No texture descriptor; pass its data, stride, pitch, width and height to opacity_map_rasterize. |
| `opacity_map_rasterize`, `ffi::meshopt_opacityMapRasterize` | `opacity_map_rasterize` |
| `opacity_map_compact`, `ffi::meshopt_opacityMapCompact` | `opacity_map_compact` |
| `optimize_vertex_cache`, `ffi::meshopt_optimizeVertexCache` | `optimize_vertex_cache` |
| `optimize_vertex_cache_in_place` | `optimize_vertex_cache_in_place` |
| `optimize_vertex_cache_fifo`, `ffi::meshopt_optimizeVertexCacheFifo` | `optimize_vertex_cache_fifo` |
| `optimize_vertex_cache_fifo_in_place` | `optimize_vertex_cache_fifo_in_place` |
| `optimize_vertex_cache_strip`, `ffi::meshopt_optimizeVertexCacheStrip` | `optimize_vertex_cache_strip` |
| `optimize_vertex_cache_strip_in_place` | `optimize_vertex_cache_strip_in_place` |
| `optimize_vertex_fetch`, `ffi::meshopt_optimizeVertexFetch` | `optimize_vertex_fetch` |
| `optimize_vertex_fetch_in_place` | `optimize_vertex_fetch_in_place` |
| `optimize_vertex_fetch_remap`, `ffi::meshopt_optimizeVertexFetchRemap` | `optimize_vertex_fetch_remap` |
| `optimize_overdraw_in_place`, `optimize_overdraw_in_place_decoder` | `optimize_overdraw_in_place` |
| `FromVertex`, `PackedVertex`, `PackedVertexOct`, `Vertex`, `pack_vertices` | not available. No application vertex schema or packing trait; keep your own types and pack explicitly. |
| `generate_vertex_remap`, `ffi::meshopt_generateVertexRemap` | `generate_vertex_remap` |
| `generate_vertex_remap_multi`, `ffi::meshopt_generateVertexRemapMulti` | `generate_vertex_remap_multi` |
| `remap_index_buffer`, `ffi::meshopt_remapIndexBuffer` | `remap_index_buffer` |
| `remap_vertex_buffer`, `ffi::meshopt_remapVertexBuffer` | `remap_vertex_buffer` |
| `generate_position_remap`, `generate_position_remap_decoder`, `ffi::meshopt_generatePositionRemap` | `generate_position_remap` |
| `filter_index_buffer`, `ffi::meshopt_filterIndexBuffer` | `filter_index_buffer` |
| `filter_index_buffer_multi`, `ffi::meshopt_filterIndexBufferMulti` | `filter_index_buffer_multi` |
| `generate_vertex_remap_custom`, `generate_vertex_remap_custom_decoder`, `ffi::meshopt_generateVertexRemapCustom` | `generate_vertex_remap_custom` |
| `generate_adjacency_indices`, `generate_adjacency_indices_decoder`, `ffi::meshopt_generateAdjacencyIndexBuffer` | `generate_adjacency_index_buffer` |
| `generate_tessellation_indices`, `generate_tessellation_indices_decoder`, `ffi::meshopt_generateTessellationIndexBuffer` | `generate_tessellation_index_buffer` |
| `generate_provoking_indices`, `ffi::meshopt_generateProvokingIndexBuffer` | `generate_provoking_index_buffer`. Returns ProvokingMesh. |
| `RemeshOptions` | `REMESH_SHELL`, `REMESH_SOLVE`. Pass u32 flags; experimental feature. |
| `remesh`, `remesh_decoder`, `ffi::meshopt_remesh` | `remesh`. experimental feature. |
| `generate_shadow_indices`, `generate_shadow_indices_decoder`, `ffi::meshopt_generateShadowIndexBuffer` | `generate_shadow_index_buffer` |
| `generate_shadow_indices_multi`, `ffi::meshopt_generateShadowIndexBufferMulti` | `generate_shadow_index_buffer_multi` |
| `SimplifyOptions` | `SimplifyOptions`. Use EMPTY and uppercase flag names; experimental flags need experimental. |
| `SimplifyVertex`, `VertexLock` | `VertexFlags`. Convert bool locks to LOCK/EMPTY; other flags use PROTECT/PRIORITY. |
| `simplify`, `simplify_decoder`, `ffi::meshopt_simplify` | `simplify` |
| `simplify_with_locks`, `simplify_with_locks_decoder`, `simplify_with_attributes_and_locks`, `simplify_with_attributes_and_locks_decoder`, `ffi::meshopt_simplifyWithAttributes` | `simplify_with_attributes`. Use Attributes with zero components for locks alone; flags are VertexFlags. |
| `simplify_with_update`, `ffi::meshopt_simplifyWithUpdate` | `simplify_with_update` |
| `simplify_sloppy`, `simplify_sloppy_decoder`, `simplify_sloppy_with_locks`, `simplify_sloppy_with_locks_decoder`, `ffi::meshopt_simplifySloppy` | `simplify_sloppy`. Pass Some(flags); without locks pass None. |
| `simplify_prune`, `simplify_prune_decoder`, `ffi::meshopt_simplifyPrune` | `simplify_prune` |
| `simplify_points`, `simplify_points_decoder`, `ffi::meshopt_simplifyPoints` | `simplify_points` |
| `simplify_scale`, `simplify_scale_decoder`, `ffi::meshopt_simplifyScale` | `simplify_scale` |
| `spatial_sort_remap`, `spatial_sort_remap_decoder`, `ffi::meshopt_spatialSortRemap` | `spatial_sort_remap` |
| `spatial_sort_triangles`, `spatial_sort_triangles_decoder`, `ffi::meshopt_spatialSortTriangles` | `spatial_sort_triangles` |
| `spatial_cluster_points`, `spatial_cluster_points_decoder`, `ffi::meshopt_spatialClusterPoints` | `spatial_cluster_points` |
| `stripify`, `ffi::meshopt_stripify` | `stripify` |
| `unstripify`, `ffi::meshopt_unstripify` | `unstripify` |
| `TangentOptions` | `TANGENT_COMPATIBLE`, `TANGENT_ZERO_FALLBACK`. Pass u32 flags. |
| `generate_normals`, `generate_normals_decoder`, `ffi::meshopt_generateNormals` | `generate_normals`. experimental feature. |
| `generate_tangents`, `ffi::meshopt_generateTangents` | `generate_tangents` |
| `any_as_u8_slice`, `typed_to_bytes` | not available. No generic object-to-bytes cast; serialize initialized fields explicitly. |
| `convert_indices_32_to_16`, `convert_indices_16_to_32` | not available. Use u16::try_from or u32::from over the slice. |
| `quantize_unorm`; `quantize_snorm`; `quantize_half`, `ffi::meshopt_quantizeHalf`; `quantize_float`, `ffi::meshopt_quantizeFloat`; `dequantize_half`, `ffi::meshopt_dequantizeHalf`; `compute_position_exponent`, `ffi::meshopt_computePositionExponent` | `quantize_unorm`; `quantize_snorm`; `quantize_half`; `quantize_float`; `dequantize_half`; `compute_position_exponent` (respectively) |
| `rcp_safe` | not available. No public reciprocal helper; handle zero explicitly. |
| `ffi::meshopt_optimizeOverdraw` | `optimize_overdraw` |
| `ffi::meshopt_encodeIndexBufferBound`; `ffi::meshopt_encodeIndexSequenceBound`; `ffi::meshopt_encodeMeshletBound`; `ffi::meshopt_encodeVertexBufferBound` | `codec::encode_index_buffer_bound`; `codec::encode_index_sequence_bound`; `codec::encode_meshlet_bound`; `codec::encode_vertex_buffer_bound` (respectively) |
| `ffi::meshopt_encodeVertexVersion` | `codec::VertexEncoding` |
| `ffi::meshopt_stripifyBound` | `stripify_bound` |
| `ffi::meshopt_unstripifyBound` | `unstripify_bound` |
| `ffi::meshopt_Meshlet` | `Meshlet` |
| `ffi::meshopt_buildMeshletsBound` | `build_meshlets_bound` |
| `ffi::meshopt_optimizeMeshlet` | `optimize_meshlet_in_place` |
| `ffi::meshopt_setAllocator` | not available. No global allocator callback; allocations use Workspace and the Rust allocator. |
| `ffi::clodBuildHierarchyBound` | `clusterlod::build_hierarchy_bound` |
| `Meshlets::len` | `Meshlets.meshlets.len` |
| `Meshlets::is_empty` | `Meshlets.meshlets.is_empty` |
| `VertexStream::new`, `VertexStream::new_with_stride` | `VertexStream::new`. Checked byte slice, count, key size, stride and offset; packed for contiguous records. |
| `AlphaTexture::required_len` | not available. No descriptor method; validate the texture layout in opacity_map_rasterize. |
| `FromVertex::fill_from_vertex` | not available. No packing trait; pack fields in application code. |
| `VertexDataAdapter::new` | `Positions::from_bytes` |
| `VertexDataAdapter::xyz_f32_at` | `Positions::get` |
| `VertexDataAdapter::pos_ptr` | not available. No raw position pointer; pass Positions views. |

## Differences to keep in mind

- Invalid input returns typed `Result` errors instead of C++ asserts. Check errors with `?`; validate layouts through the input views.
- Reuse a `Workspace` across calls; set `Limits` for untrusted input to cap allocation and work. Caller-owned buffers are outside its memory limit.
- Caller-buffer `_into` variants return used lengths or counts. On failure, output can be partly modified; follow each function's rustdoc.
- No C/C++ toolchain is needed: `wasm32-unknown-unknown` builds directly with Rust's target installed.
- For `no_std` with `alloc`, set `default-features = false`; provide an allocator.
- Output is byte-identical to scalar C++ 1.3, also used by `meshopt` 0.7.0, for matching inputs and settings; [verification](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/VERIFICATION.md) records the tested domain.

## Before and after

Each pair starts with `meshopt` (ignored here because it is not a dependency), then a runnable Rust example.
The byte APIs use initialized little-endian records; serialize fields instead of casting arbitrary structs.

### Simplify
```ignore
let vertices = meshopt::VertexDataAdapter::new(meshopt::typed_to_bytes(&positions), 12, 0)?;
let lod = meshopt::simplify(&indices, &vertices, 3, 0.01, meshopt::SimplifyOptions::None, None);
```
```rust
use meshoptimizer_rs::{simplify, Positions, SimplifyOptions, SimplifySettings, Workspace};
# let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
# let indices = [0, 1, 2, 2, 1, 3];
let settings = SimplifySettings { target_index_count: 3, target_error: 0.01, options: SimplifyOptions::EMPTY };
let lod = simplify(&indices, Positions::from_packed(&positions), settings, &mut Workspace::default())?;
assert!(lod.indices.len() <= indices.len()); // lod.error replaces the result_error argument
# Ok::<(), meshoptimizer_rs::Error>(())
```
### Vertex cache, then fetch
```ignore
let cached = meshopt::optimize_vertex_cache(&indices, positions.len());
let mut fetched_indices = cached;
let fetched_vertices = meshopt::optimize_vertex_fetch(&mut fetched_indices, &positions);
```
```rust
use meshoptimizer_rs::{optimize_vertex_cache, optimize_vertex_fetch, VertexStream, Workspace};
# let positions = [[0.0_f32, 0., 0.], [1., 0., 0.], [0., 1., 0.]]; let indices = [0, 1, 2];
let mut ws = Workspace::default();
let bytes: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
let cached = optimize_vertex_cache(&indices, positions.len(), &mut ws)?;
let fetched = optimize_vertex_fetch(&cached, VertexStream::packed(&bytes, 12)?, &mut ws)?;
assert_eq!(fetched.indices.len(), indices.len()); // fetched.vertices contains packed bytes
# Ok::<(), meshoptimizer_rs::Error>(())
```
### Meshlets with bounds

`meshopt` builders also optimize each meshlet. Call `optimize_meshlet_in_place` to match that step.
Our triangle slices have no per-meshlet padding; use the descriptor offsets, not an alignment formula.
```ignore
let vertices = meshopt::VertexDataAdapter::new(meshopt::typed_to_bytes(&positions), 12, 0)?;
let built = meshopt::build_meshlets(&indices, &vertices, 64, 124, 0.25);
let bounds: Vec<_> = built.iter().map(|m| meshopt::compute_meshlet_bounds(m, &vertices)).collect();
```
```rust
use meshoptimizer_rs::{build_meshlets, compute_meshlet_bounds, optimize_meshlet_in_place, MeshletSettings, Positions, Workspace};
# let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]; let indices = [0, 1, 2];
let mut ws = Workspace::default();
let view = Positions::from_packed(&positions);
let mut built = build_meshlets(&indices, view, MeshletSettings::default(), 0.25, &mut ws)?;
let mut bounds = Vec::new();
for m in &built.meshlets {
    let (v, t) = (m.vertex_offset as usize, m.triangle_offset as usize);
    let vertices = &mut built.vertices[v..v + m.vertex_count as usize];
    let triangles = &mut built.triangles[t..t + m.triangle_count as usize * 3];
    optimize_meshlet_in_place(vertices, triangles, &mut ws)?;
    bounds.push(compute_meshlet_bounds(vertices, triangles, view, &mut ws)?);
}
assert_eq!(bounds.len(), built.meshlets.len());
# Ok::<(), meshoptimizer_rs::Error>(())
```
### Vertex and index buffer encode/decode

Versions and levels are per-call settings. Index decoding returns little-endian bytes at stride 2 or 4.
```ignore
let packed_v = meshopt::encode_vertex_buffer(&positions)?;
let vertices = meshopt::decode_vertex_buffer::<[f32; 3]>(&packed_v, positions.len())?;
let packed_i = meshopt::encode_index_buffer(&indices, positions.len())?;
let indices = meshopt::decode_index_buffer::<u32>(&packed_i, indices.len())?;
```
```rust
use meshoptimizer_rs::{codec::*, Workspace};
# let positions = [[0.0_f32, 0., 0.], [1., 0., 0.], [0., 1., 0.]]; let indices: [u32; 3] = [0, 1, 2];
let mut ws = Workspace::default();
let bytes: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
let packed_v = encode_vertex_buffer(&bytes, positions.len(), 12, VertexEncoding::DEFAULT, &mut ws)?;
assert_eq!(decode_vertex_buffer(positions.len(), 12, &packed_v, &mut ws)?, bytes);
let packed_i = encode_index_buffer(&indices, IndexEncoding::DEFAULT, &mut ws)?;
let decoded = decode_index_buffer(indices.len(), 4, &packed_i, &mut ws)?;
assert_eq!(decoded, indices.iter().flat_map(|i| i.to_le_bytes()).collect::<Vec<_>>());
# Ok::<(), meshoptimizer_rs::Error>(())
```
### Generate vertex remap
```ignore
let (vertex_count, remap) = meshopt::generate_vertex_remap(&positions, Some(&indices));
```
```rust
use meshoptimizer_rs::{generate_vertex_remap, VertexStream, Workspace};
# let positions = [[0.0_f32, 0., 0.], [1., 0., 0.], [0., 0., 0.]]; let indices = [0, 1, 2];
let bytes: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
let result = generate_vertex_remap(Some(&indices), VertexStream::packed(&bytes, 12)?, &mut Workspace::default())?;
assert_eq!(result.vertex_count, 2);
assert_eq!(result.remap, [0, 1, 0]);
# Ok::<(), meshoptimizer_rs::Error>(())
```
