#[cfg(feature = "ours")]
fn custom_root(
    i: Option<&[u32]>,
    p: meshoptimizer_rs::Positions<'_>,
    w: &mut meshoptimizer_rs::Workspace,
) -> Result<meshoptimizer_rs::VertexRemap, meshoptimizer_rs::Error> {
    meshoptimizer_rs::generate_vertex_remap_custom(i, p, |_, _| true, w)
}
#[cfg(feature = "ours")]
fn clod_callback_root(
    c: meshoptimizer_rs::clusterlod::Config,
    m: meshoptimizer_rs::clusterlod::Mesh<'_>,
    w: &mut meshoptimizer_rs::Workspace,
) -> Result<(), meshoptimizer_rs::Error> {
    meshoptimizer_rs::clusterlod::build_with_output(c, m, |_, _| Ok(0), w)
}
fn main() {
    #[cfg(feature = "ours")]
    {
        std::hint::black_box(custom_root as *const ());
        std::hint::black_box(clod_callback_root as *const ());
        std::hint::black_box(meshoptimizer_rs::analyze_coverage as *const ());
        std::hint::black_box(meshoptimizer_rs::analyze_overdraw as *const ());
        std::hint::black_box(meshoptimizer_rs::analyze_vertex_cache as *const ());
        std::hint::black_box(meshoptimizer_rs::analyze_vertex_fetch as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_flex as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_flex_into as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_into as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_scan as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_scan_into as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_spatial as *const ());
        std::hint::black_box(meshoptimizer_rs::build_meshlets_spatial_into as *const ());
        std::hint::black_box(meshoptimizer_rs::compute_cluster_bounds as *const ());
        std::hint::black_box(meshoptimizer_rs::compute_meshlet_bounds as *const ());
        std::hint::black_box(meshoptimizer_rs::compute_position_exponent as *const ());
        std::hint::black_box(meshoptimizer_rs::compute_sphere_bounds as *const ());
        std::hint::black_box(meshoptimizer_rs::dequantize_half as *const ());
        std::hint::black_box(meshoptimizer_rs::extract_meshlet_indices as *const ());
        std::hint::black_box(meshoptimizer_rs::extract_meshlet_indices_into as *const ());
        std::hint::black_box(meshoptimizer_rs::filter_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::filter_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::filter_index_buffer_multi as *const ());
        std::hint::black_box(meshoptimizer_rs::filter_index_buffer_multi_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_adjacency_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_adjacency_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_normals as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_normals_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_position_remap as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_position_remap_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_provoking_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_provoking_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_shadow_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_shadow_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_shadow_index_buffer_multi as *const ());
        std::hint::black_box(
            meshoptimizer_rs::generate_shadow_index_buffer_multi_into as *const (),
        );
        std::hint::black_box(meshoptimizer_rs::generate_tangents as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_tangents_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_tessellation_index_buffer as *const ());
        std::hint::black_box(
            meshoptimizer_rs::generate_tessellation_index_buffer_into as *const (),
        );
        std::hint::black_box(meshoptimizer_rs::generate_vertex_remap as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_vertex_remap_into as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_vertex_remap_multi as *const ());
        std::hint::black_box(meshoptimizer_rs::generate_vertex_remap_multi_into as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_compact as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_entry_size as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_measure as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_measure_into as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_rasterize as *const ());
        std::hint::black_box(meshoptimizer_rs::opacity_map_rasterize_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet_level as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet_level_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_meshlet_level_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_overdraw as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_overdraw_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_overdraw_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_fifo as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_fifo_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_fifo_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_strip as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_strip_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_cache_strip_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_fetch as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_fetch_into as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_fetch_remap as *const ());
        std::hint::black_box(meshoptimizer_rs::optimize_vertex_fetch_remap_into as *const ());
        std::hint::black_box(meshoptimizer_rs::partition_clusters as *const ());
        std::hint::black_box(meshoptimizer_rs::partition_clusters_into as *const ());
        std::hint::black_box(meshoptimizer_rs::quantize_float as *const ());
        std::hint::black_box(meshoptimizer_rs::quantize_half as *const ());
        std::hint::black_box(meshoptimizer_rs::quantize_snorm as *const ());
        std::hint::black_box(meshoptimizer_rs::quantize_unorm as *const ());
        std::hint::black_box(meshoptimizer_rs::remap_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::remap_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::remap_vertex_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::remap_vertex_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::remesh as *const ());
        std::hint::black_box(meshoptimizer_rs::remesh_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::remesh_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_points as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_points_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_prune as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_prune_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_scale as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_sloppy as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_sloppy_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_with_attributes as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_with_attributes_into as *const ());
        std::hint::black_box(meshoptimizer_rs::simplify_with_update as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_cluster_points as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_cluster_points_into as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_sort_remap as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_sort_remap_into as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_sort_triangles as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_sort_triangles_in_place as *const ());
        std::hint::black_box(meshoptimizer_rs::spatial_sort_triangles_into as *const ());
        std::hint::black_box(meshoptimizer_rs::stripify as *const ());
        std::hint::black_box(meshoptimizer_rs::stripify_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::stripify_into as *const ());
        std::hint::black_box(meshoptimizer_rs::unstripify as *const ());
        std::hint::black_box(meshoptimizer_rs::unstripify_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::unstripify_into as *const ());
        std::hint::black_box(meshoptimizer_rs::validate_vertex_flags as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_meshlet_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_meshlet_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_meshlet as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_meshlet_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_meshlet_raw_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_meshlet as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_meshlet_raw as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_vertex_buffer_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_buffer_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_sequence_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_vertex_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_vertex_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_sequence as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_index_sequence_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_oct as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_oct_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_quat as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_quat_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_exp as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_exp_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_color as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::encode_filter_color_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_vertex_version as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_index_version as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_vertex_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_vertex_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_index_buffer as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_index_buffer_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_index_sequence as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_index_sequence_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_buffer_view as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_buffer_view_into as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_filter_oct as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_filter_quat as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_filter_exp as *const ());
        std::hint::black_box(meshoptimizer_rs::codec::decode_filter_color as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::simplify_lod_chain as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::simplify_lod_chains_batch as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::encode_buffers_batch as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::decode_buffer_views_batch as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::build_meshlets_batch as *const ());
        std::hint::black_box(meshoptimizer_rs::parallel::build_cluster_lod_batch as *const ());
        std::hint::black_box(
            meshoptimizer_rs::parallel::build_cluster_hierarchies_batch as *const (),
        );
        std::hint::black_box(meshoptimizer_rs::clusterlod::default_config as *const ());
        std::hint::black_box(meshoptimizer_rs::clusterlod::default_config_rt as *const ());
        std::hint::black_box(meshoptimizer_rs::clusterlod::build as *const ());
        std::hint::black_box(meshoptimizer_rs::clusterlod::local_indices as *const ());
        std::hint::black_box(meshoptimizer_rs::clusterlod::build_hierarchy_bound as *const ());
        std::hint::black_box(meshoptimizer_rs::clusterlod::build_hierarchy as *const ());
    }
    #[cfg(feature = "theirs")]
    {
        std::hint::black_box(meshopt::ffi::meshopt_generateVertexRemap as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateVertexRemapMulti as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateVertexRemapCustom as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_remapVertexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_remapIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateShadowIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateShadowIndexBufferMulti as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generatePositionRemap as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateAdjacencyIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateTessellationIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_generateProvokingIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeVertexCache as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeVertexCacheStrip as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeVertexCacheFifo as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeOverdraw as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeVertexFetch as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeVertexFetchRemap as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeIndexBufferBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeIndexVersion as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeIndexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeIndexVersion as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeIndexSequence as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeIndexSequenceBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeIndexSequence as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeVertexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeVertexBufferBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeVertexBufferLevel as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeVertexVersion as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeVertexBuffer as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeVertexVersion as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeFilterOct as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeFilterQuat as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeFilterExp as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_decodeFilterColor as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeFilterOct as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeFilterQuat as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeFilterExp as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_encodeFilterColor as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplify as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifyWithAttributes as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifyWithUpdate as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifySloppy as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifyPrune as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifyPoints as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_simplifyScale as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_stripify as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_stripifyBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_unstripify as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_unstripifyBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_analyzeVertexCache as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_analyzeVertexFetch as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_analyzeOverdraw as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_analyzeCoverage as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_buildMeshlets as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_buildMeshletsScan as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_buildMeshletsBound as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_buildMeshletsFlex as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_buildMeshletsSpatial as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_optimizeMeshlet as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_computeClusterBounds as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_computeMeshletBounds as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_computeSphereBounds as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_partitionClusters as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_spatialSortRemap as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_spatialSortTriangles as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_spatialClusterPoints as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_quantizeHalf as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_quantizeFloat as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_dequantizeHalf as *const ());
        std::hint::black_box(meshopt::ffi::meshopt_setAllocator as *const ());
        std::hint::black_box(meshopt::quantize_unorm as *const ());
        std::hint::black_box(meshopt::quantize_snorm as *const ());
    }
}
