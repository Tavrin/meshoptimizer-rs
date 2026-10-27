# Peer comparison and C++ 1.3 feature inventory

Entry: meshoptimizer-rs 0.2.13, commit 42782f5. Peers: meshopt 0.7.0 (vendored C++ 1.3), optimesh 1.1.0. No library changes.

Metric: median of three instructions:u samples per cell, 100% counter running. Ratios are peer / ours: below 1 means the peer executes fewer user instructions. Corpus ratios are equal-case geometric means; each real-mesh column is one retained input. No wall-time performance conclusion.

Completed: 840 main samples, 162 stage samples; 280 main and 54 stage outputs validated, 144 encoder/decoder cross-checks. Wall seconds/call and process peak RSS are disclosure only. All build/fixture/collection/validation jobs used CPU-only admission with 4 GB declared. Maximum measured process RSS: 2026784 KiB.

Both Rust ports enable SIMD; C++ uses its default native dispatch. All consumers use opt-level 3, thin LTO, one codegen unit and no target-cpu override. Ours uses std+simd+clusterlod and a 3 GiB Workspace storage cap; default counted-work limit. C++ uses the public ffi API, except the public cluster-LOD callback wrapper. These are allocating consumer calls, not a separate benchmark of meshopt idiomatic wrappers or Cargo default feature sets.

All arms count output allocation, required copies, scratch and destruction after one warmup. Ours can retain Workspace capacity; neither peer exposes an equivalent scratch handle. Parsing, serialization, meshlet padding normalization and validation are outside counters. Cluster-LOD callback collection and a position clone are counted in both arms; the C++ wrapper additionally creates borrowed-cluster descriptor vectors per callback. These API costs are included, not subtracted.

API adaptation: ours returns owned checked results and takes Workspace/views; C++/optimesh caller destinations are zero-initialized inside each counted call. Simplify result structs, tuples and C++ out-error pointers serialize to the same indices-plus-f32 record. Attributes use four float elements versus the peers’ 16-byte stride. optimesh index decode returns u32 elements; ours/C++ return four-byte element buffers, with serialization outside counters. Codec global index version is set once outside counters; ours selects it per call. The C++ cluster-LOD wrapper accepts immutable positions and rejects dilation, so shared raster defaults keep dilation disabled; ours’ callback returns Ok(id) where the peer returns id. These are the call-shape mismatches in the measured surface.

The existing five corpus meshes and three real meshes are unchanged packed XYZ/index projections of retained inputs. The original real vertex layout is 64 bytes; all three arms use the same 12-byte XYZ layout here. Attribute calls use the existing four-float synthetic formula, not authored real attributes. No quality-equivalence, GPU, whole-chain throughput or strided-layout performance claim follows. The separate eighteen-stage supplement retains original simplify targets/errors and indices while using the same packed position projection.

The first build failed with exit 101 on a callback return type and was corrected. The first collection stopped with harness exit 1 / child exit 101 at the default 1 GiB Workspace cap on real-214e attribute simplification. Its 535 finished samples and failed cell remain retained and are excluded in full. The final matrix was rebuilt and recollected with the explicit 3 GiB cap; no library limit or threshold changed.

## Peer table

| Function | Peer | Corpus ratio | real-214e | real-a082 | real-b6fd | Output vs C++ (identical/cases) | API/settings notes |
|---|---|---:|---:|---:|---:|---|---|
| simplify | meshopt | 0.926 | 0.733 | 0.778 | 0.776 | ours 8/8; peer 8/8 | Half index count; error 0.5; no flags/locks; includes returned f32 error. |
| simplify | optimesh | 1.049 | 0.914 | 0.991 | 0.991 | ours 8/8; peer 8/8 | Half index count; error 0.5; no flags/locks; includes returned f32 error. |
| simplify-attributes | meshopt | 0.890 | 0.680 | 0.723 | 0.722 | ours 8/8; peer 8/8 | Four synthetic floats/vertex, unit weights; same target/error. |
| simplify-attributes | optimesh | 1.032 | 0.783 | 0.865 | 0.865 | ours 8/8; peer 8/8 | Four synthetic floats/vertex, unit weights; same target/error. |
| simplify-sloppy | meshopt | 0.725 | 0.661 | 0.666 | 0.663 | ours 8/8; peer 8/8 | No locks; same target/error. |
| simplify-sloppy | optimesh | 1.057 | 1.238 | 1.159 | 1.227 | ours 8/8; peer 8/8 | No locks; same target/error. |
| cache | meshopt | 0.759 | 0.764 | 0.761 | 0.761 | ours 8/8; peer 8/8 | Default score preset. |
| cache | optimesh | 1.034 | 1.039 | 1.040 | 1.040 | ours 8/8; peer 8/8 | Default score preset. |
| overdraw | meshopt | 1.008 | 0.681 | 0.696 | 0.696 | ours 8/8; peer 8/8 | Threshold 1.05; input order unchanged. |
| overdraw | optimesh | 1.094 | 0.842 | 0.856 | 0.856 | ours 8/8; peer 8/8 | Threshold 1.05; input order unchanged. |
| fetch | meshopt | 0.854 | 1.057 | 1.047 | 1.047 | ours 8/8; peer 8/8 | 12-byte vertices; peer in-place APIs require an index copy. |
| fetch | optimesh | 1.040 | 1.408 | 1.389 | 1.392 | ours 8/8; peer 8/8 | 12-byte vertices; peer in-place APIs require an index copy. |
| meshlets | meshopt | 0.895 | 0.710 | 0.721 | 0.716 | ours 8/8; peer 8/8 | 64/124/0; C++/optimesh four-byte triangle padding; normalized outside counters. |
| meshlets | optimesh | 1.220 | 0.890 | 0.911 | 0.909 | ours 8/8; peer 8/8 | 64/124/0; C++/optimesh four-byte triangle padding; normalized outside counters. |
| vertex-encode | meshopt | 1.730 | 1.837 | 1.726 | 1.852 | ours 8/8; peer 8/8 | Version 1, level 2; explicit per-call setting. |
| vertex-encode | optimesh | 2.147 | 2.242 | 2.115 | 2.245 | ours 8/8; peer 8/8 | Version 1, level 2; explicit per-call setting. |
| vertex-decode | meshopt | 0.711 | 0.792 | 0.801 | 0.795 | ours 8/8; peer 8/8 | Same C++ 1.3 stream; 12-byte vertices. |
| vertex-decode | optimesh | 1.344 | 1.411 | 1.412 | 1.408 | ours 8/8; peer 8/8 | Same C++ 1.3 stream; 12-byte vertices. |
| index-encode | meshopt | 1.071 | 1.538 | 0.922 | 1.540 | ours 8/8; peer 8/8 | Version 1; u32 source. |
| index-encode | optimesh | 1.329 | 1.677 | 0.975 | 1.697 | ours 8/8; peer 8/8 | Version 1; u32 source. |
| index-decode | meshopt | 0.921 | 1.046 | 0.851 | 1.045 | ours 8/8; peer 8/8 | Same C++ 1.3 stream; 32-bit result. |
| index-decode | optimesh | 1.045 | 1.121 | 0.958 | 1.118 | ours 8/8; peer 8/8 | Same C++ 1.3 stream; 32-bit result. |
| cluster-lod | meshopt | 0.810 | 0.705 | 0.707 | 0.707 | ours 8/8; peer 8/8 | Ours/C++ only; raster defaults 128, positions only; callback record construction counted. |

## Eighteen retained simplify stages

Ratios below use sums of six stage instruction medians per chain. Settings come from each original stage manifest; repetitions are three per arm. Identity includes index bytes and reported error.

| Chain | meshopt / ours | optimesh / ours | Ours identity | optimesh identity |
|---|---:|---:|---|---|
| real-214e | 0.733 | 0.914 | 6/6 | 6/6 |
| real-a082 | 0.775 | 0.972 | 6/6 | 6/6 |
| real-b6fd | 0.772 | 0.972 | 6/6 | 6/6 |

## Gaps ranked by user value

1. Typed index/output adapters (M): asset importers with u16 topology and generic decoded storage can avoid manual conversions. C++ has templated index overloads; optimesh has typed codec output traits but u32 processing. Ours has 16/32-bit byte-width codec output and u32 processing; the typed facade is partial. This is API work, not a missing codec.

2. Caller-defined cache score tables (S): expose cache-position and live-valence scores for engine/hardware tuning. optimesh exposes VertexScoreTable and optimize_vertex_cache_table. Ours exposes the default and strip presets only. Validate tuned tables against downstream cache/raster workloads before claiming benefit.

3. Library-specific allocator hooks (L): hosts using arenas, per-library telemetry or allocator isolation can choose transient allocation separately from the application global allocator. C++ exposes setAllocator; neither Rust port does. Ours provides Workspace limits/retention, which is a different control.

No core C++ geometry-processing function is missing by API name. Experimental normals/remesh and the two experimental simplify flags require the experimental feature; cluster LOD requires clusterlod. Global codec-version setters have explicit per-call equivalents and are not format gaps. Presence below is source/API coverage, not a new exhaustive parity proof over every option. S: local API/kernel extension; M: API/storage plumbing across families; L: new algorithm family or library-wide allocation/ownership refactor.

## Output differences

All identities use complete little-endian outputs; simplify includes error, fetch includes indices and vertices, cluster LOD includes all callback records. First byte offsets are zero-based; null means end-of-buffer. Raw meshlet layout is checked separately from normalized packed topology.

| Case | Function | Arm | Compared representation | First difference / identity |
|---|---|---|---|---|
| medium-smooth | meshlets | ours | raw layout | differs: byte 47748, end vs 0; lengths 47748/47750 |
| medium-seams | meshlets | ours | raw layout | differs: byte 50108, end vs 0; lengths 50108/50109 |
| medium-sparse | meshlets | ours | raw layout | differs: byte 47748, end vs 0; lengths 47748/47750 |
| million-smooth | meshlets | ours | raw layout | differs: byte 5818968, end vs 0; lengths 5818968/5818970 |
| real-214e | meshlets | ours | raw layout | differs: byte 37587099, end vs 0; lengths 37587099/37587102 |
| real-b6fd | meshlets | ours | raw layout | differs: byte 10214377, end vs 0; lengths 10214377/10214379 |

## Complete function inventory

C++ references are relative to the meshopt 0.7.0 crate; optimesh references are relative to optimesh 1.1.0. Port references are relative to this repository. The pinned source hashes and machine-readable rows are in inventory.json. Individual C++ template overloads and implicitly public Rust trait methods are included; cfg(test) helpers and private SIMD modules are excluded.

| Source surface/function | Source file:line | Port status | Port function/type and file:line | optimesh | Notes |
|---|---|---|---|---|---|
| C++ 1.3 meshopt_generateVertexRemap | vendor/src/meshoptimizer.h:62 | present | generate_vertex_remap; src/remap.rs:575 | src/indexgenerator.rs:310 |  |
| C++ 1.3 meshopt_generateVertexRemapMulti | vendor/src/meshoptimizer.h:75 | present | generate_vertex_remap_multi; src/remap.rs:587 | src/indexgenerator.rs:328 |  |
| C++ 1.3 meshopt_generateVertexRemapCustom | vendor/src/meshoptimizer.h:88 | present | generate_vertex_remap_custom; src/remap.rs:696 | src/indexgenerator.rs:344 |  |
| C++ 1.3 meshopt_remapVertexBuffer | vendor/src/meshoptimizer.h:96 | present | remap_vertex_buffer; src/remap.rs:890 | src/indexgenerator.rs:366 |  |
| C++ 1.3 meshopt_remapIndexBuffer | vendor/src/meshoptimizer.h:104 | present | remap_index_buffer; src/remap.rs:1026 | src/indexgenerator.rs:386 |  |
| C++ 1.3 meshopt_filterIndexBuffer | vendor/src/meshoptimizer.h:114 | present | filter_index_buffer; src/remap.rs:1245 | src/indexgenerator.rs:443 |  |
| C++ 1.3 meshopt_filterIndexBufferMulti | vendor/src/meshoptimizer.h:125 | present | filter_index_buffer_multi; src/remap.rs:1258 | src/indexgenerator.rs:463 |  |
| C++ 1.3 meshopt_generateShadowIndexBuffer | vendor/src/meshoptimizer.h:135 | present | generate_shadow_index_buffer; src/remap.rs:1218 | src/indexgenerator.rs:505 |  |
| C++ 1.3 meshopt_generateShadowIndexBufferMulti | vendor/src/meshoptimizer.h:146 | present | generate_shadow_index_buffer_multi; src/remap.rs:1231 | src/indexgenerator.rs:523 |  |
| C++ 1.3 meshopt_generatePositionRemap | vendor/src/meshoptimizer.h:156 | present | generate_position_remap; src/remap.rs:729 | src/indexgenerator.rs:540 |  |
| C++ 1.3 meshopt_generateAdjacencyIndexBuffer | vendor/src/meshoptimizer.h:169 | present | generate_adjacency_index_buffer; src/remap.rs:1545 | src/indexgenerator.rs:592 |  |
| C++ 1.3 meshopt_generateTessellationIndexBuffer | vendor/src/meshoptimizer.h:185 | present | generate_tessellation_index_buffer; src/remap.rs:1558 | src/indexgenerator.rs:657 |  |
| C++ 1.3 meshopt_generateProvokingIndexBuffer | vendor/src/meshoptimizer.h:198 | present | generate_provoking_index_buffer; src/remap.rs:1571 | src/indexgenerator.rs:722 |  |
| C++ 1.3 meshopt_optimizeVertexCache | vendor/src/meshoptimizer.h:207 | present | optimize_vertex_cache; src/cache.rs:34 | src/vcacheoptimizer.rs:120 |  |
| C++ 1.3 meshopt_optimizeVertexCacheStrip | vendor/src/meshoptimizer.h:216 | present | optimize_vertex_cache_strip; src/cache.rs:314 | src/vcacheoptimizer.rs:127 |  |
| C++ 1.3 meshopt_optimizeVertexCacheFifo | vendor/src/meshoptimizer.h:227 | present | optimize_vertex_cache_fifo; src/cache.rs:381 | src/vcacheoptimizer.rs:302 |  |
| C++ 1.3 meshopt_optimizeOverdraw | vendor/src/meshoptimizer.h:239 | present | optimize_overdraw; src/overdraw.rs:18 | src/overdrawoptimizer.rs:332 |  |
| C++ 1.3 meshopt_optimizeVertexFetch | vendor/src/meshoptimizer.h:250 | present | optimize_vertex_fetch; src/remap.rs:2048 | src/vfetchoptimizer.rs:44 |  |
| C++ 1.3 meshopt_optimizeVertexFetchRemap | vendor/src/meshoptimizer.h:260 | present | optimize_vertex_fetch_remap; src/remap.rs:1830 | src/vfetchoptimizer.rs:16 |  |
| C++ 1.3 meshopt_encodeIndexBuffer | vendor/src/meshoptimizer.h:271 | present | encode_index_buffer; src/codec/encode.rs:271 | src/indexcodec.rs:280 |  |
| C++ 1.3 meshopt_encodeIndexBufferBound | vendor/src/meshoptimizer.h:272 | present | encode_index_buffer_bound; src/codec/encode.rs:120 | src/indexcodec.rs:263 |  |
| C++ 1.3 meshopt_encodeIndexVersion | vendor/src/meshoptimizer.h:280 | present | IndexEncoding; src/codec/encode.rs:53 | src/indexcodec.rs:36 | Explicit per-call version replaces the global setter. |
| C++ 1.3 meshopt_decodeIndexBuffer | vendor/src/meshoptimizer.h:290 | present | decode_index_buffer; src/codec/mod.rs:443 | src/indexcodec.rs:481 |  |
| C++ 1.3 meshopt_decodeIndexVersion | vendor/src/meshoptimizer.h:297 | present | decode_index_version; src/codec/mod.rs:245 | src/indexcodec.rs:465 |  |
| C++ 1.3 meshopt_encodeIndexSequence | vendor/src/meshoptimizer.h:307 | present | encode_index_sequence; src/codec/encode.rs:319 | src/indexcodec.rs:666 |  |
| C++ 1.3 meshopt_encodeIndexSequenceBound | vendor/src/meshoptimizer.h:308 | present | encode_index_sequence_bound; src/codec/encode.rs:132 | src/indexcodec.rs:651 |  |
| C++ 1.3 meshopt_decodeIndexSequence | vendor/src/meshoptimizer.h:318 | present | decode_index_sequence; src/codec/mod.rs:490 | src/indexcodec.rs:713 |  |
| C++ 1.3 meshopt_encodeMeshlet | vendor/src/meshoptimizer.h:331 | present | encode_meshlet; src/codec/meshlet.rs:224 | src/meshletcodec.rs:1078 |  |
| C++ 1.3 meshopt_encodeMeshletBound | vendor/src/meshoptimizer.h:332 | present | encode_meshlet_bound; src/codec/meshlet.rs:25 | src/meshletcodec.rs:1061 |  |
| C++ 1.3 meshopt_decodeMeshlet | vendor/src/meshoptimizer.h:349 | present | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:1125 |  |
| C++ 1.3 meshopt_decodeMeshletRaw | vendor/src/meshoptimizer.h:350 | present | decode_meshlet_raw; src/codec/meshlet.rs:756 | src/meshletcodec.rs:1175 |  |
| C++ 1.3 meshopt_encodeVertexBuffer | vendor/src/meshoptimizer.h:363 | present | encode_vertex_buffer; src/codec/encode.rs:195 | src/vertexcodec.rs:697 |  |
| C++ 1.3 meshopt_encodeVertexBufferBound | vendor/src/meshoptimizer.h:364 | present | encode_vertex_buffer_bound; src/codec/encode.rs:107 | src/vertexcodec.rs:674 |  |
| C++ 1.3 meshopt_encodeVertexBufferLevel | vendor/src/meshoptimizer.h:377 | present | VertexEncoding; src/codec/encode.rs:10 | src/vertexcodec.rs:716 | Level and version configured per call; encode_vertex_buffer executes it. |
| C++ 1.3 meshopt_encodeVertexVersion | vendor/src/meshoptimizer.h:385 | present | VertexEncoding; src/codec/encode.rs:10 | src/vertexcodec.rs:42 | Explicit per-call version replaces the global setter. |
| C++ 1.3 meshopt_decodeVertexBuffer | vendor/src/meshoptimizer.h:396 | present | decode_vertex_buffer; src/codec/mod.rs:396 | src/vertexcodec.rs:1703 |  |
| C++ 1.3 meshopt_decodeVertexVersion | vendor/src/meshoptimizer.h:403 | present | decode_vertex_version; src/codec/mod.rs:237 | src/vertexcodec.rs:1687 |  |
| C++ 1.3 meshopt_decodeFilterOct | vendor/src/meshoptimizer.h:421 | present | decode_filter_oct; src/codec/mod.rs:622 | absent |  |
| C++ 1.3 meshopt_decodeFilterQuat | vendor/src/meshoptimizer.h:422 | present | decode_filter_quat; src/codec/mod.rs:636 | src/vertexfilter.rs:788 |  |
| C++ 1.3 meshopt_decodeFilterExp | vendor/src/meshoptimizer.h:423 | present | decode_filter_exp; src/codec/mod.rs:649 | src/vertexfilter.rs:1069 |  |
| C++ 1.3 meshopt_decodeFilterColor | vendor/src/meshoptimizer.h:424 | present | decode_filter_color; src/codec/mod.rs:667 | absent |  |
| C++ 1.3 meshopt_encodeFilterOct | vendor/src/meshoptimizer.h:458 | present | encode_filter_oct; src/codec/encode.rs:509 | absent |  |
| C++ 1.3 meshopt_encodeFilterQuat | vendor/src/meshoptimizer.h:459 | present | encode_filter_quat; src/codec/encode.rs:555 | src/vertexfilter.rs:1677 |  |
| C++ 1.3 meshopt_encodeFilterExp | vendor/src/meshoptimizer.h:460 | present | encode_filter_exp; src/codec/encode.rs:604 | src/vertexfilter.rs:1733 |  |
| C++ 1.3 meshopt_encodeFilterColor | vendor/src/meshoptimizer.h:461 | present | encode_filter_color; src/codec/encode.rs:645 | absent |  |
| C++ 1.3 meshopt_simplify | vendor/src/meshoptimizer.h:517 | present | simplify; src/clusterlod/support/simplify.rs:140, src/simplify.rs:360 | src/simplifier.rs:2941 |  |
| C++ 1.3 meshopt_simplifyWithAttributes | vendor/src/meshoptimizer.h:541 | present | simplify_with_attributes; src/clusterlod/support/simplify.rs:152, src/simplify.rs:375 | src/simplifier.rs:2952 |  |
| C++ 1.3 meshopt_simplifyWithUpdate | vendor/src/meshoptimizer.h:564 | present | simplify_with_update; src/clusterlod/support/simplify_extra.rs:547, src/simplify_extra.rs:688 | src/simplifier.rs:2997 |  |
| C++ 1.3 meshopt_simplifySloppy | vendor/src/meshoptimizer.h:580 | present | simplify_sloppy; src/clusterlod/support/simplify_extra.rs:766, src/simplify_extra.rs:975 | src/simplifier.rs:3058 |  |
| C++ 1.3 meshopt_simplifyPrune | vendor/src/meshoptimizer.h:593 | present | simplify_prune; src/clusterlod/support/simplify_extra.rs:146, src/simplify_extra.rs:224 | src/simplifier.rs:3316 |  |
| C++ 1.3 meshopt_simplifyPoints | vendor/src/meshoptimizer.h:607 | present | simplify_points; src/clusterlod/support/simplify_extra.rs:973, src/simplify_extra.rs:1475 | src/simplifier.rs:3196 |  |
| C++ 1.3 meshopt_simplifyScale | vendor/src/meshoptimizer.h:615 | present | simplify_scale; src/clusterlod/support/simplify.rs:103, src/simplify.rs:226 | src/simplifier.rs:3365 |  |
| C++ 1.3 meshopt_stripify | vendor/src/meshoptimizer.h:627 | present | stripify; src/strip.rs:171 | src/stripifier.rs:76 |  |
| C++ 1.3 meshopt_stripifyBound | vendor/src/meshoptimizer.h:628 | present | stripify_bound; src/strip.rs:85 | src/stripifier.rs:266 |  |
| C++ 1.3 meshopt_unstripify | vendor/src/meshoptimizer.h:637 | present | unstripify; src/strip.rs:361 | src/stripifier.rs:278 |  |
| C++ 1.3 meshopt_unstripifyBound | vendor/src/meshoptimizer.h:638 | present | unstripify_bound; src/strip.rs:101 | src/stripifier.rs:311 |  |
| C++ 1.3 meshopt_analyzeVertexCache | vendor/src/meshoptimizer.h:653 | present | analyze_vertex_cache; src/analyze.rs:103 | src/indexanalyzer.rs:42 |  |
| C++ 1.3 meshopt_analyzeVertexFetch | vendor/src/meshoptimizer.h:666 | present | analyze_vertex_fetch; src/analyze.rs:320 | src/indexanalyzer.rs:130 |  |
| C++ 1.3 meshopt_analyzeOverdraw | vendor/src/meshoptimizer.h:682 | present | analyze_overdraw; src/raster.rs:276 | src/rasterizer.rs:252 |  |
| C++ 1.3 meshopt_analyzeCoverage | vendor/src/meshoptimizer.h:696 | present | analyze_coverage; src/raster.rs:290 | src/rasterizer.rs:302 |  |
| C++ 1.3 meshopt_buildMeshlets | vendor/src/meshoptimizer.h:731 | present | build_meshlets; src/meshlet.rs:1707 | src/clusterizer.rs:1534 |  |
| C++ 1.3 meshopt_buildMeshletsScan | vendor/src/meshoptimizer.h:732 | present | build_meshlets_scan; src/meshlet.rs:1685 | src/clusterizer.rs:1554 |  |
| C++ 1.3 meshopt_buildMeshletsBound | vendor/src/meshoptimizer.h:733 | present | build_meshlets_bound; src/meshlet.rs:62 | src/clusterizer.rs:1301 |  |
| C++ 1.3 meshopt_buildMeshletsFlex | vendor/src/meshoptimizer.h:748 | present | build_meshlets_flex; src/meshlet.rs:1722 | src/clusterizer.rs:1323 |  |
| C++ 1.3 meshopt_buildMeshletsSpatial | vendor/src/meshoptimizer.h:761 | present | build_meshlets_spatial; src/meshlet.rs:1776 | src/clusterizer.rs:1596 |  |
| C++ 1.3 meshopt_optimizeMeshlet | vendor/src/meshoptimizer.h:770 | present | optimize_meshlet; src/meshlet_util.rs:971 | src/meshletutils.rs:632 |  |
| C++ 1.3 meshopt_optimizeMeshletLevel | vendor/src/meshoptimizer.h:779 | present | optimize_meshlet_level; src/meshlet_util.rs:951 | src/meshletutils.rs:473 |  |
| C++ 1.3 meshopt_computeClusterBounds | vendor/src/meshoptimizer.h:821 | present | compute_cluster_bounds; src/meshlet_util.rs:361 | src/meshletutils.rs:358 |  |
| C++ 1.3 meshopt_computeMeshletBounds | vendor/src/meshoptimizer.h:822 | present | compute_meshlet_bounds; src/meshlet_util.rs:493 | src/meshletutils.rs:392 |  |
| C++ 1.3 meshopt_computeSphereBounds | vendor/src/meshoptimizer.h:831 | present | compute_sphere_bounds; src/meshlet_util.rs:615 | src/meshletutils.rs:427 |  |
| C++ 1.3 meshopt_extractMeshletIndices | vendor/src/meshoptimizer.h:842 | present | extract_meshlet_indices; src/meshlet_util.rs:708 | src/meshletutils.rs:653 |  |
| C++ 1.3 meshopt_partitionClusters | vendor/src/meshoptimizer.h:856 | present | partition_clusters; src/partition.rs:575 | src/partition.rs:519 |  |
| C++ 1.3 meshopt_spatialSortRemap | vendor/src/meshoptimizer.h:866 | present | spatial_sort_remap; src/spatial.rs:85 | src/spatialorder.rs:268 |  |
| C++ 1.3 meshopt_spatialSortTriangles | vendor/src/meshoptimizer.h:875 | present | spatial_sort_triangles; src/spatial.rs:159 | src/spatialorder.rs:321 |  |
| C++ 1.3 meshopt_spatialClusterPoints | vendor/src/meshoptimizer.h:885 | present | spatial_cluster_points; src/spatial.rs:298 | src/spatialorder.rs:378 |  |
| C++ 1.3 meshopt_opacityMapMeasure | vendor/src/meshoptimizer.h:899 | present | opacity_map_measure; src/opacity.rs:348 | src/opacitymap.rs:447 |  |
| C++ 1.3 meshopt_opacityMapRasterize | vendor/src/meshoptimizer.h:913 | present | opacity_map_rasterize; src/opacity.rs:824 | src/opacitymap.rs:533 |  |
| C++ 1.3 meshopt_opacityMapEntrySize | vendor/src/meshoptimizer.h:914 | present | opacity_map_entry_size; src/opacity.rs:22 | src/opacitymap.rs:521 |  |
| C++ 1.3 meshopt_opacityMapCompact | vendor/src/meshoptimizer.h:928 | present | opacity_map_compact; src/opacity.rs:515 | src/opacitymap.rs:572 |  |
| C++ 1.3 meshopt_generateTangents | vendor/src/meshoptimizer.h:955 | present | generate_tangents; src/tangent.rs:494 | src/tangentspace.rs:482 |  |
| C++ 1.3 meshopt_generateNormals | vendor/src/meshoptimizer.h:970 | present | generate_normals; src/normal.rs:409 | absent | Requires experimental feature. |
| C++ 1.3 meshopt_remesh | vendor/src/meshoptimizer.h:995 | present | remesh; src/remesh.rs:811 | absent | Requires experimental feature. |
| C++ 1.3 meshopt_quantizeHalf | vendor/src/meshoptimizer.h:1003 | present | quantize_half; src/quantize.rs:49 | src/quantization.rs:58 |  |
| C++ 1.3 meshopt_quantizeFloat | vendor/src/meshoptimizer.h:1010 | present | quantize_float; src/quantize.rs:68 | src/quantization.rs:127 |  |
| C++ 1.3 meshopt_dequantizeHalf | vendor/src/meshoptimizer.h:1016 | present | dequantize_half; src/quantize.rs:87 | src/quantization.rs:94 |  |
| C++ 1.3 meshopt_computePositionExponent | vendor/src/meshoptimizer.h:1033 | present | compute_position_exponent; src/quantize.rs:108 | src/quantization.rs:164 |  |
| C++ 1.3 meshopt_setAllocator | vendor/src/meshoptimizer.h:1044 | missing | Workspace; src/clusterlod/support/workspace.rs:54, src/workspace.rs:75 | absent | Workspace controls limits/retention, not allocator selection; Rust global allocator remains process-wide. Install library-specific allocate/free callbacks for transient storage. Users: Hosts with per-library arenas, allocation telemetry or allocator isolation. Size: L. |
| C++ 1.3 meshopt_quantizeUnorm | vendor/src/meshoptimizer.h:1057 | present | quantize_unorm; src/quantize.rs:10 | src/quantization.rs:16 |  |
| C++ 1.3 meshopt_quantizeSnorm | vendor/src/meshoptimizer.h:1064 | present | quantize_snorm; src/quantize.rs:35 | src/quantization.rs:39 |  |
| C++ cluster LOD demo clodDefaultConfig | vendor/demo/clusterlod.h:171 | present | default_config; src/clusterlod/mod.rs:72 | absent | clusterlod feature; equivalent explicit Rust API. |
| C++ cluster LOD demo clodDefaultConfigRT | vendor/demo/clusterlod.h:172 | present | default_config_rt; src/clusterlod/mod.rs:108 | absent | clusterlod feature; equivalent explicit Rust API. |
| C++ cluster LOD demo clodBuild | vendor/demo/clusterlod.h:175 | present | build_with_output; src/clusterlod/mod.rs:1023 | absent | clusterlod feature; equivalent explicit Rust API. |
| C++ cluster LOD demo clodLocalIndices | vendor/demo/clusterlod.h:180 | present | extract_meshlet_indices; src/meshlet_util.rs:708 | absent | clusterlod feature; equivalent explicit Rust API. |
| C++ cluster LOD demo clodBuildHierarchyBound | vendor/demo/clusterlod.h:184 | present | build_hierarchy_bound; src/clusterlod/mod.rs:1087 | absent | clusterlod feature; equivalent explicit Rust API. |
| C++ cluster LOD demo clodBuildHierarchy | vendor/demo/clusterlod.h:191 | present | build_hierarchy; src/clusterlod/mod.rs:1114 | absent | clusterlod feature; equivalent explicit Rust API. |
| optimesh 1.1.0 clusterizer::build_meshlets_bound | src/clusterizer.rs:1301 | present | build_meshlets_bound; src/meshlet.rs:62 | src/clusterizer.rs:1301 |  |
| optimesh 1.1.0 clusterizer::build_meshlets_flex | src/clusterizer.rs:1323 | present | build_meshlets_flex; src/meshlet.rs:1722 | src/clusterizer.rs:1323 |  |
| optimesh 1.1.0 clusterizer::build_meshlets | src/clusterizer.rs:1534 | present | build_meshlets; src/meshlet.rs:1707 | src/clusterizer.rs:1534 |  |
| optimesh 1.1.0 clusterizer::build_meshlets_scan | src/clusterizer.rs:1554 | present | build_meshlets_scan; src/meshlet.rs:1685 | src/clusterizer.rs:1554 |  |
| optimesh 1.1.0 clusterizer::build_meshlets_spatial | src/clusterizer.rs:1596 | present | build_meshlets_spatial; src/meshlet.rs:1776 | src/clusterizer.rs:1596 |  |
| optimesh 1.1.0 indexanalyzer::analyze_vertex_cache | src/indexanalyzer.rs:42 | present | analyze_vertex_cache; src/analyze.rs:103 | src/indexanalyzer.rs:42 |  |
| optimesh 1.1.0 indexanalyzer::analyze_vertex_fetch | src/indexanalyzer.rs:130 | present | analyze_vertex_fetch; src/analyze.rs:320 | src/indexanalyzer.rs:130 |  |
| optimesh 1.1.0 indexcodec::set_encode_index_version | src/indexcodec.rs:36 | present | IndexEncoding; src/codec/encode.rs:53 | src/indexcodec.rs:36 | Explicit per-call configuration replaces the global setter. |
| optimesh 1.1.0 indexcodec::encode_index_buffer_bound | src/indexcodec.rs:263 | present | encode_index_buffer_bound; src/codec/encode.rs:120 | src/indexcodec.rs:263 |  |
| optimesh 1.1.0 indexcodec::encode_index_buffer | src/indexcodec.rs:280 | present | encode_index_buffer; src/codec/encode.rs:271 | src/indexcodec.rs:280 |  |
| optimesh 1.1.0 indexcodec::decode_index_version | src/indexcodec.rs:465 | present | decode_index_version; src/codec/mod.rs:245 | src/indexcodec.rs:465 |  |
| optimesh 1.1.0 indexcodec::decode_index_buffer | src/indexcodec.rs:481 | present | decode_index_buffer; src/codec/mod.rs:443 | src/indexcodec.rs:481 |  |
| optimesh 1.1.0 indexcodec::encode_index_sequence_bound | src/indexcodec.rs:651 | present | encode_index_sequence_bound; src/codec/encode.rs:132 | src/indexcodec.rs:651 |  |
| optimesh 1.1.0 indexcodec::encode_index_sequence | src/indexcodec.rs:666 | present | encode_index_sequence; src/codec/encode.rs:319 | src/indexcodec.rs:666 |  |
| optimesh 1.1.0 indexcodec::decode_index_sequence | src/indexcodec.rs:713 | present | decode_index_sequence; src/codec/mod.rs:490 | src/indexcodec.rs:713 |  |
| optimesh 1.1.0 indexgenerator::generate_vertex_remap | src/indexgenerator.rs:310 | present | generate_vertex_remap; src/remap.rs:575 | src/indexgenerator.rs:310 |  |
| optimesh 1.1.0 indexgenerator::generate_vertex_remap_multi | src/indexgenerator.rs:328 | present | generate_vertex_remap_multi; src/remap.rs:587 | src/indexgenerator.rs:328 |  |
| optimesh 1.1.0 indexgenerator::generate_vertex_remap_custom | src/indexgenerator.rs:344 | present | generate_vertex_remap_custom; src/remap.rs:696 | src/indexgenerator.rs:344 |  |
| optimesh 1.1.0 indexgenerator::remap_vertex_buffer | src/indexgenerator.rs:366 | present | remap_vertex_buffer; src/remap.rs:890 | src/indexgenerator.rs:366 |  |
| optimesh 1.1.0 indexgenerator::remap_index_buffer | src/indexgenerator.rs:386 | present | remap_index_buffer; src/remap.rs:1026 | src/indexgenerator.rs:386 |  |
| optimesh 1.1.0 indexgenerator::filter_index_buffer | src/indexgenerator.rs:443 | present | filter_index_buffer; src/remap.rs:1245 | src/indexgenerator.rs:443 |  |
| optimesh 1.1.0 indexgenerator::filter_index_buffer_multi | src/indexgenerator.rs:463 | present | filter_index_buffer_multi; src/remap.rs:1258 | src/indexgenerator.rs:463 |  |
| optimesh 1.1.0 indexgenerator::generate_shadow_index_buffer | src/indexgenerator.rs:505 | present | generate_shadow_index_buffer; src/remap.rs:1218 | src/indexgenerator.rs:505 |  |
| optimesh 1.1.0 indexgenerator::generate_shadow_index_buffer_multi | src/indexgenerator.rs:523 | present | generate_shadow_index_buffer_multi; src/remap.rs:1231 | src/indexgenerator.rs:523 |  |
| optimesh 1.1.0 indexgenerator::generate_position_remap | src/indexgenerator.rs:540 | present | generate_position_remap; src/remap.rs:729 | src/indexgenerator.rs:540 |  |
| optimesh 1.1.0 indexgenerator::generate_adjacency_index_buffer | src/indexgenerator.rs:592 | present | generate_adjacency_index_buffer; src/remap.rs:1545 | src/indexgenerator.rs:592 |  |
| optimesh 1.1.0 indexgenerator::generate_tessellation_index_buffer | src/indexgenerator.rs:657 | present | generate_tessellation_index_buffer; src/remap.rs:1558 | src/indexgenerator.rs:657 |  |
| optimesh 1.1.0 indexgenerator::generate_provoking_index_buffer | src/indexgenerator.rs:722 | present | generate_provoking_index_buffer; src/remap.rs:1571 | src/indexgenerator.rs:722 |  |
| optimesh 1.1.0 meshletcodec::encode_meshlet_bound | src/meshletcodec.rs:1061 | present | encode_meshlet_bound; src/codec/meshlet.rs:25 | src/meshletcodec.rs:1061 |  |
| optimesh 1.1.0 meshletcodec::encode_meshlet | src/meshletcodec.rs:1078 | present | encode_meshlet; src/codec/meshlet.rs:224 | src/meshletcodec.rs:1078 |  |
| optimesh 1.1.0 meshletcodec::decode_meshlet | src/meshletcodec.rs:1125 | present | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:1125 |  |
| optimesh 1.1.0 meshletcodec::decode_meshlet_raw | src/meshletcodec.rs:1175 | present | decode_meshlet_raw; src/codec/meshlet.rs:756 | src/meshletcodec.rs:1175 |  |
| optimesh 1.1.0 meshletutils::compute_cluster_bounds | src/meshletutils.rs:358 | present | compute_cluster_bounds; src/meshlet_util.rs:361 | src/meshletutils.rs:358 |  |
| optimesh 1.1.0 meshletutils::compute_meshlet_bounds | src/meshletutils.rs:392 | present | compute_meshlet_bounds; src/meshlet_util.rs:493 | src/meshletutils.rs:392 |  |
| optimesh 1.1.0 meshletutils::compute_sphere_bounds | src/meshletutils.rs:427 | present | compute_sphere_bounds; src/meshlet_util.rs:615 | src/meshletutils.rs:427 |  |
| optimesh 1.1.0 meshletutils::optimize_meshlet_level | src/meshletutils.rs:473 | present | optimize_meshlet_level; src/meshlet_util.rs:951 | src/meshletutils.rs:473 |  |
| optimesh 1.1.0 meshletutils::optimize_meshlet | src/meshletutils.rs:632 | present | optimize_meshlet; src/meshlet_util.rs:971 | src/meshletutils.rs:632 |  |
| optimesh 1.1.0 meshletutils::extract_meshlet_indices | src/meshletutils.rs:653 | present | extract_meshlet_indices; src/meshlet_util.rs:708 | src/meshletutils.rs:653 |  |
| optimesh 1.1.0 opacitymap::opacity_map_measure | src/opacitymap.rs:447 | present | opacity_map_measure; src/opacity.rs:348 | src/opacitymap.rs:447 |  |
| optimesh 1.1.0 opacitymap::opacity_map_entry_size | src/opacitymap.rs:521 | present | opacity_map_entry_size; src/opacity.rs:22 | src/opacitymap.rs:521 |  |
| optimesh 1.1.0 opacitymap::opacity_map_rasterize | src/opacitymap.rs:533 | present | opacity_map_rasterize; src/opacity.rs:824 | src/opacitymap.rs:533 |  |
| optimesh 1.1.0 opacitymap::opacity_map_compact | src/opacitymap.rs:572 | present | opacity_map_compact; src/opacity.rs:515 | src/opacitymap.rs:572 |  |
| optimesh 1.1.0 overdrawoptimizer::optimize_overdraw | src/overdrawoptimizer.rs:332 | present | optimize_overdraw; src/overdraw.rs:18 | src/overdrawoptimizer.rs:332 |  |
| optimesh 1.1.0 partition::partition_clusters | src/partition.rs:519 | present | partition_clusters; src/partition.rs:575 | src/partition.rs:519 |  |
| optimesh 1.1.0 quantization::quantize_unorm | src/quantization.rs:16 | present | quantize_unorm; src/quantize.rs:10 | src/quantization.rs:16 |  |
| optimesh 1.1.0 quantization::quantize_snorm | src/quantization.rs:39 | present | quantize_snorm; src/quantize.rs:35 | src/quantization.rs:39 |  |
| optimesh 1.1.0 quantization::quantize_half | src/quantization.rs:58 | present | quantize_half; src/quantize.rs:49 | src/quantization.rs:58 |  |
| optimesh 1.1.0 quantization::dequantize_half | src/quantization.rs:94 | present | dequantize_half; src/quantize.rs:87 | src/quantization.rs:94 |  |
| optimesh 1.1.0 quantization::quantize_float | src/quantization.rs:127 | present | quantize_float; src/quantize.rs:68 | src/quantization.rs:127 |  |
| optimesh 1.1.0 quantization::compute_position_exponent | src/quantization.rs:164 | present | compute_position_exponent; src/quantize.rs:108 | src/quantization.rs:164 |  |
| optimesh 1.1.0 rasterizer::analyze_overdraw | src/rasterizer.rs:252 | present | analyze_overdraw; src/raster.rs:276 | src/rasterizer.rs:252 |  |
| optimesh 1.1.0 rasterizer::analyze_coverage | src/rasterizer.rs:302 | present | analyze_coverage; src/raster.rs:290 | src/rasterizer.rs:302 |  |
| optimesh 1.1.0 simplifier::simplify | src/simplifier.rs:2941 | present | simplify; src/clusterlod/support/simplify.rs:140, src/simplify.rs:360 | src/simplifier.rs:2941 |  |
| optimesh 1.1.0 simplifier::simplify_with_attributes | src/simplifier.rs:2952 | present | simplify_with_attributes; src/clusterlod/support/simplify.rs:152, src/simplify.rs:375 | src/simplifier.rs:2952 |  |
| optimesh 1.1.0 simplifier::simplify_with_update | src/simplifier.rs:2997 | present | simplify_with_update; src/clusterlod/support/simplify_extra.rs:547, src/simplify_extra.rs:688 | src/simplifier.rs:2997 |  |
| optimesh 1.1.0 simplifier::simplify_sloppy | src/simplifier.rs:3058 | present | simplify_sloppy; src/clusterlod/support/simplify_extra.rs:766, src/simplify_extra.rs:975 | src/simplifier.rs:3058 |  |
| optimesh 1.1.0 simplifier::simplify_points | src/simplifier.rs:3196 | present | simplify_points; src/clusterlod/support/simplify_extra.rs:973, src/simplify_extra.rs:1475 | src/simplifier.rs:3196 |  |
| optimesh 1.1.0 simplifier::simplify_prune | src/simplifier.rs:3316 | present | simplify_prune; src/clusterlod/support/simplify_extra.rs:146, src/simplify_extra.rs:224 | src/simplifier.rs:3316 |  |
| optimesh 1.1.0 simplifier::simplify_scale | src/simplifier.rs:3365 | present | simplify_scale; src/clusterlod/support/simplify.rs:103, src/simplify.rs:226 | src/simplifier.rs:3365 |  |
| optimesh 1.1.0 spatialorder::spatial_sort_remap | src/spatialorder.rs:268 | present | spatial_sort_remap; src/spatial.rs:85 | src/spatialorder.rs:268 |  |
| optimesh 1.1.0 spatialorder::spatial_sort_triangles | src/spatialorder.rs:321 | present | spatial_sort_triangles; src/spatial.rs:159 | src/spatialorder.rs:321 |  |
| optimesh 1.1.0 spatialorder::spatial_cluster_points | src/spatialorder.rs:378 | present | spatial_cluster_points; src/spatial.rs:298 | src/spatialorder.rs:378 |  |
| optimesh 1.1.0 stripifier::stripify | src/stripifier.rs:76 | present | stripify; src/strip.rs:171 | src/stripifier.rs:76 |  |
| optimesh 1.1.0 stripifier::stripify_bound | src/stripifier.rs:266 | present | stripify_bound; src/strip.rs:85 | src/stripifier.rs:266 |  |
| optimesh 1.1.0 stripifier::unstripify | src/stripifier.rs:278 | present | unstripify; src/strip.rs:361 | src/stripifier.rs:278 |  |
| optimesh 1.1.0 stripifier::unstripify_bound | src/stripifier.rs:311 | present | unstripify_bound; src/strip.rs:101 | src/stripifier.rs:311 |  |
| optimesh 1.1.0 tangentspace::generate_tangents | src/tangentspace.rs:482 | present | generate_tangents; src/tangent.rs:494 | src/tangentspace.rs:482 |  |
| optimesh 1.1.0 vcacheoptimizer::optimize_vertex_cache | src/vcacheoptimizer.rs:120 | present | optimize_vertex_cache; src/cache.rs:34 | src/vcacheoptimizer.rs:120 |  |
| optimesh 1.1.0 vcacheoptimizer::optimize_vertex_cache_strip | src/vcacheoptimizer.rs:127 | present | optimize_vertex_cache_strip; src/cache.rs:314 | src/vcacheoptimizer.rs:127 |  |
| optimesh 1.1.0 vcacheoptimizer::optimize_vertex_cache_table | src/vcacheoptimizer.rs:143 | missing | optimize_vertex_cache; src/cache.rs:34 | src/vcacheoptimizer.rs:143 | Only the standard and strip score presets are exposed. Use caller-supplied cache/valence score tables for triangle ordering. Users: Engine or hardware tuning tools and research optimizers. Size: S. |
| optimesh 1.1.0 vcacheoptimizer::optimize_vertex_cache_fifo | src/vcacheoptimizer.rs:302 | present | optimize_vertex_cache_fifo; src/cache.rs:381 | src/vcacheoptimizer.rs:302 |  |
| optimesh 1.1.0 vertexcodec::set_encode_vertex_version | src/vertexcodec.rs:42 | present | VertexEncoding; src/codec/encode.rs:10 | src/vertexcodec.rs:42 | Explicit per-call configuration replaces the global setter. |
| optimesh 1.1.0 vertexcodec::encode_vertex_buffer_bound | src/vertexcodec.rs:674 | present | encode_vertex_buffer_bound; src/codec/encode.rs:107 | src/vertexcodec.rs:674 |  |
| optimesh 1.1.0 vertexcodec::encode_vertex_buffer | src/vertexcodec.rs:697 | present | encode_vertex_buffer; src/codec/encode.rs:195 | src/vertexcodec.rs:697 |  |
| optimesh 1.1.0 vertexcodec::encode_vertex_buffer_level | src/vertexcodec.rs:716 | present | VertexEncoding; src/codec/encode.rs:10 | src/vertexcodec.rs:716 | Pass VertexEncoding to encode_vertex_buffer. |
| optimesh 1.1.0 vertexcodec::decode_vertex_version | src/vertexcodec.rs:1687 | present | decode_vertex_version; src/codec/mod.rs:237 | src/vertexcodec.rs:1687 |  |
| optimesh 1.1.0 vertexcodec::decode_vertex_buffer | src/vertexcodec.rs:1703 | present | decode_vertex_buffer; src/codec/mod.rs:396 | src/vertexcodec.rs:1703 |  |
| optimesh 1.1.0 vertexfilter::decode_filter_oct8 | src/vertexfilter.rs:366 | present | decode_filter_oct; src/codec/mod.rs:622 | src/vertexfilter.rs:366 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::decode_filter_oct16 | src/vertexfilter.rs:656 | present | decode_filter_oct; src/codec/mod.rs:622 | src/vertexfilter.rs:656 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::decode_filter_quat | src/vertexfilter.rs:788 | present | decode_filter_quat; src/codec/mod.rs:636 | src/vertexfilter.rs:788 |  |
| optimesh 1.1.0 vertexfilter::decode_filter_exp | src/vertexfilter.rs:1069 | present | decode_filter_exp; src/codec/mod.rs:649 | src/vertexfilter.rs:1069 |  |
| optimesh 1.1.0 vertexfilter::decode_filter_color8 | src/vertexfilter.rs:1333 | present | decode_filter_color; src/codec/mod.rs:667 | src/vertexfilter.rs:1333 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::decode_filter_color16 | src/vertexfilter.rs:1603 | present | decode_filter_color; src/codec/mod.rs:667 | src/vertexfilter.rs:1603 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::encode_filter_oct8 | src/vertexfilter.rs:1661 | present | encode_filter_oct; src/codec/encode.rs:509 | src/vertexfilter.rs:1661 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::encode_filter_oct16 | src/vertexfilter.rs:1667 | present | encode_filter_oct; src/codec/encode.rs:509 | src/vertexfilter.rs:1667 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::encode_filter_quat | src/vertexfilter.rs:1677 | present | encode_filter_quat; src/codec/encode.rs:555 | src/vertexfilter.rs:1677 |  |
| optimesh 1.1.0 vertexfilter::encode_filter_exp | src/vertexfilter.rs:1733 | present | encode_filter_exp; src/codec/encode.rs:604 | src/vertexfilter.rs:1733 |  |
| optimesh 1.1.0 vertexfilter::encode_filter_color8 | src/vertexfilter.rs:1835 | present | encode_filter_color; src/codec/encode.rs:645 | src/vertexfilter.rs:1835 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vertexfilter::encode_filter_color16 | src/vertexfilter.rs:1841 | present | encode_filter_color; src/codec/encode.rs:645 | src/vertexfilter.rs:1841 | Single byte-buffer API selects the 4/8-byte representation by stride. |
| optimesh 1.1.0 vfetchoptimizer::optimize_vertex_fetch_remap | src/vfetchoptimizer.rs:16 | present | optimize_vertex_fetch_remap; src/remap.rs:1830 | src/vfetchoptimizer.rs:16 |  |
| optimesh 1.1.0 vfetchoptimizer::optimize_vertex_fetch | src/vfetchoptimizer.rs:44 | present | optimize_vertex_fetch; src/remap.rs:2048 | src/vfetchoptimizer.rs:44 |  |
| C++ 1.3 C++ templates meshopt_IndexAdapter<T> and templated index overloads | vendor/src/meshoptimizer.h:1072 | partial | u32 processing / byte-oriented codecs; src/clusterlod/support/simplify.rs:140, src/simplify.rs:360; src/codec/mod.rs:443 | indexcodec::IndexWrite and meshletcodec::{MeshletVertex,MeshletTriangle}; processing uses u32 | Processing accepts u32 slices. Codecs expose 16/32-bit destinations as bytes; no general typed processing adapter. Convert other integer index types around the u32 algorithm. Users: Importers retaining 16-bit topology and generic mesh containers. Size: M. |
| C++ template overload meshopt_generateVertexRemap | vendor/src/meshoptimizer.h:1077 | partial | generate_vertex_remap; src/remap.rs:575 | src/indexgenerator.rs:310 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateVertexRemapMulti | vendor/src/meshoptimizer.h:1079 | partial | generate_vertex_remap_multi; src/remap.rs:587 | src/indexgenerator.rs:328 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateVertexRemapCustom | vendor/src/meshoptimizer.h:1081 | present | generate_vertex_remap_custom; src/remap.rs:696 | src/indexgenerator.rs:344 | Rust closures replace callback templates. |
| C++ template overload meshopt_generateVertexRemapCustom | vendor/src/meshoptimizer.h:1083 | partial | generate_vertex_remap_custom; src/remap.rs:696 | src/indexgenerator.rs:344 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_remapIndexBuffer | vendor/src/meshoptimizer.h:1085 | partial | remap_index_buffer; src/remap.rs:1026 | src/indexgenerator.rs:386 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_filterIndexBuffer | vendor/src/meshoptimizer.h:1087 | partial | filter_index_buffer; src/remap.rs:1245 | src/indexgenerator.rs:443 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_filterIndexBufferMulti | vendor/src/meshoptimizer.h:1089 | partial | filter_index_buffer_multi; src/remap.rs:1258 | src/indexgenerator.rs:463 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateShadowIndexBuffer | vendor/src/meshoptimizer.h:1091 | partial | generate_shadow_index_buffer; src/remap.rs:1218 | src/indexgenerator.rs:505 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateShadowIndexBufferMulti | vendor/src/meshoptimizer.h:1093 | partial | generate_shadow_index_buffer_multi; src/remap.rs:1231 | src/indexgenerator.rs:523 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateAdjacencyIndexBuffer | vendor/src/meshoptimizer.h:1095 | partial | generate_adjacency_index_buffer; src/remap.rs:1545 | src/indexgenerator.rs:592 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateTessellationIndexBuffer | vendor/src/meshoptimizer.h:1097 | partial | generate_tessellation_index_buffer; src/remap.rs:1558 | src/indexgenerator.rs:657 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateProvokingIndexBuffer | vendor/src/meshoptimizer.h:1099 | partial | generate_provoking_index_buffer; src/remap.rs:1571 | src/indexgenerator.rs:722 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeVertexCache | vendor/src/meshoptimizer.h:1101 | partial | optimize_vertex_cache; src/cache.rs:34 | src/vcacheoptimizer.rs:120 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeVertexCacheStrip | vendor/src/meshoptimizer.h:1103 | partial | optimize_vertex_cache_strip; src/cache.rs:314 | src/vcacheoptimizer.rs:127 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeVertexCacheFifo | vendor/src/meshoptimizer.h:1105 | partial | optimize_vertex_cache_fifo; src/cache.rs:381 | src/vcacheoptimizer.rs:302 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeOverdraw | vendor/src/meshoptimizer.h:1107 | partial | optimize_overdraw; src/overdraw.rs:18 | src/overdrawoptimizer.rs:332 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeVertexFetchRemap | vendor/src/meshoptimizer.h:1109 | partial | optimize_vertex_fetch_remap; src/remap.rs:1830 | src/vfetchoptimizer.rs:16 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_optimizeVertexFetch | vendor/src/meshoptimizer.h:1111 | partial | optimize_vertex_fetch; src/remap.rs:2048 | src/vfetchoptimizer.rs:44 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_encodeIndexBuffer | vendor/src/meshoptimizer.h:1113 | partial | encode_index_buffer; src/codec/encode.rs:271 | src/indexcodec.rs:280 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_decodeIndexBuffer | vendor/src/meshoptimizer.h:1115 | partial | decode_index_buffer; src/codec/mod.rs:443 | src/indexcodec.rs:481 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_encodeIndexSequence | vendor/src/meshoptimizer.h:1117 | partial | encode_index_sequence; src/codec/encode.rs:319 | src/indexcodec.rs:666 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_decodeIndexSequence | vendor/src/meshoptimizer.h:1119 | partial | decode_index_sequence; src/codec/mod.rs:490 | src/indexcodec.rs:713 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_decodeMeshlet | vendor/src/meshoptimizer.h:1121 | partial | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:1125 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_encodeVertexBufferLevel | vendor/src/meshoptimizer.h:1122 | present | VertexEncoding; src/codec/encode.rs:10 | src/vertexcodec.rs:716 | Rust closures replace callback templates. |
| C++ template overload meshopt_simplify | vendor/src/meshoptimizer.h:1124 | partial | simplify; src/clusterlod/support/simplify.rs:140, src/simplify.rs:360 | src/simplifier.rs:2941 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_simplifyWithAttributes | vendor/src/meshoptimizer.h:1126 | partial | simplify_with_attributes; src/clusterlod/support/simplify.rs:152, src/simplify.rs:375 | src/simplifier.rs:2952 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_simplifyWithUpdate | vendor/src/meshoptimizer.h:1128 | partial | simplify_with_update; src/clusterlod/support/simplify_extra.rs:547, src/simplify_extra.rs:688 | src/simplifier.rs:2997 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_simplifySloppy | vendor/src/meshoptimizer.h:1130 | partial | simplify_sloppy; src/clusterlod/support/simplify_extra.rs:766, src/simplify_extra.rs:975 | src/simplifier.rs:3058 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_simplifySloppy | vendor/src/meshoptimizer.h:1132 | partial | simplify_sloppy; src/clusterlod/support/simplify_extra.rs:766, src/simplify_extra.rs:975 | src/simplifier.rs:3058 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_simplifyPrune | vendor/src/meshoptimizer.h:1134 | partial | simplify_prune; src/clusterlod/support/simplify_extra.rs:146, src/simplify_extra.rs:224 | src/simplifier.rs:3316 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_stripify | vendor/src/meshoptimizer.h:1136 | partial | stripify; src/strip.rs:171 | src/stripifier.rs:76 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_unstripify | vendor/src/meshoptimizer.h:1138 | partial | unstripify; src/strip.rs:361 | src/stripifier.rs:278 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_analyzeVertexCache | vendor/src/meshoptimizer.h:1140 | partial | analyze_vertex_cache; src/analyze.rs:103 | src/indexanalyzer.rs:42 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_analyzeVertexFetch | vendor/src/meshoptimizer.h:1142 | partial | analyze_vertex_fetch; src/analyze.rs:320 | src/indexanalyzer.rs:130 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_analyzeOverdraw | vendor/src/meshoptimizer.h:1144 | partial | analyze_overdraw; src/raster.rs:276 | src/rasterizer.rs:252 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_analyzeCoverage | vendor/src/meshoptimizer.h:1146 | partial | analyze_coverage; src/raster.rs:290 | src/rasterizer.rs:302 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_buildMeshlets | vendor/src/meshoptimizer.h:1148 | partial | build_meshlets; src/meshlet.rs:1707 | src/clusterizer.rs:1534 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_buildMeshletsScan | vendor/src/meshoptimizer.h:1150 | partial | build_meshlets_scan; src/meshlet.rs:1685 | src/clusterizer.rs:1554 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_buildMeshletsFlex | vendor/src/meshoptimizer.h:1152 | partial | build_meshlets_flex; src/meshlet.rs:1722 | src/clusterizer.rs:1323 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_buildMeshletsSpatial | vendor/src/meshoptimizer.h:1154 | partial | build_meshlets_spatial; src/meshlet.rs:1776 | src/clusterizer.rs:1596 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_computeClusterBounds | vendor/src/meshoptimizer.h:1156 | partial | compute_cluster_bounds; src/meshlet_util.rs:361 | src/meshletutils.rs:358 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_partitionClusters | vendor/src/meshoptimizer.h:1158 | partial | partition_clusters; src/partition.rs:575 | src/partition.rs:519 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_spatialSortTriangles | vendor/src/meshoptimizer.h:1160 | partial | spatial_sort_triangles; src/spatial.rs:159 | src/spatialorder.rs:321 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateTangents | vendor/src/meshoptimizer.h:1162 | partial | generate_tangents; src/tangent.rs:494 | src/tangentspace.rs:482 | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_generateNormals | vendor/src/meshoptimizer.h:1164 | partial | generate_normals; src/normal.rs:409 | absent | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| C++ template overload meshopt_remesh | vendor/src/meshoptimizer.h:1166 | partial | remesh; src/remesh.rs:811 | absent | Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs. |
| optimesh public trait IndexWrite::from_u32 | src/indexcodec.rs:44 | partial | decode_index_buffer; src/codec/mod.rs:443 | src/indexcodec.rs:44 | Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature. Expose typed/custom decoder output conversion without a separate byte-buffer conversion. Users: Generic asset importers and custom decoded index storage. Size: M. |
| optimesh public trait MeshletVertex::from_index | src/meshletcodec.rs:77 | partial | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:77 | Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature. Expose typed/custom decoder output conversion without a separate byte-buffer conversion. Users: Generic asset importers and custom decoded index storage. Size: M. |
| optimesh public trait MeshletVertex::decode_simd | src/meshletcodec.rs:83 | partial | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:83 | Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature. Expose typed/custom decoder output conversion without a separate byte-buffer conversion. Users: Generic asset importers and custom decoded index storage. Size: M. |
| optimesh public trait MeshletTriangle::write_packed | src/meshletcodec.rs:153 | partial | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:153 | Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature. Expose typed/custom decoder output conversion without a separate byte-buffer conversion. Users: Generic asset importers and custom decoded index storage. Size: M. |
| optimesh public trait MeshletTriangle::decode_simd | src/meshletcodec.rs:159 | partial | decode_meshlet; src/codec/meshlet.rs:717 | src/meshletcodec.rs:159 | Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature. Expose typed/custom decoder output conversion without a separate byte-buffer conversion. Users: Generic asset importers and custom decoded index storage. Size: M. |

## Complete option inventory

Includes named flags/enums, every clodConfig field, optimesh input/config fields and score-table controls, plus numeric/layout domains. Counts/strides are represented by checked views; optimesh attribute strides are bytes while ours uses float elements. Output/statistic/error fields are not options. optimesh error enum variants are retained in inventory.json for completeness. Cargo features: optimesh simd maps to ours simd; oracle/oracle-simd only enable test-oracle builds and are not runtime geometry options.

| Surface/option | Source file:line | Status | Port and file:line | optimesh | Notes |
|---|---|---|---|---|---|
| C++ 1.3 meshopt_EncodeExpSeparate | vendor/src/meshoptimizer.h:449 | present | Separate; src/codec/encode.rs:84, src/codec/encode.rs:89 | present |  |
| C++ 1.3 meshopt_EncodeExpSharedVector | vendor/src/meshoptimizer.h:451 | present | SharedVector; src/codec/encode.rs:86 | present |  |
| C++ 1.3 meshopt_EncodeExpSharedComponent | vendor/src/meshoptimizer.h:453 | present | SharedComponent; src/codec/encode.rs:88 | present |  |
| C++ 1.3 meshopt_EncodeExpClamped | vendor/src/meshoptimizer.h:455 | present | Clamped; src/codec/encode.rs:90 | present |  |
| C++ 1.3 meshopt_SimplifyLockBorder | vendor/src/meshoptimizer.h:469 | present | LOCK_BORDER; src/clusterlod/support/simplify.rs:22, src/simplify.rs:136 | present |  |
| C++ 1.3 meshopt_SimplifySparse | vendor/src/meshoptimizer.h:471 | present | SPARSE; src/clusterlod/support/simplify.rs:24, src/simplify.rs:138 | present |  |
| C++ 1.3 meshopt_SimplifyErrorAbsolute | vendor/src/meshoptimizer.h:473 | present | ERROR_ABSOLUTE; src/clusterlod/support/simplify.rs:26, src/simplify.rs:140 | present |  |
| C++ 1.3 meshopt_SimplifyPrune | vendor/src/meshoptimizer.h:475 | present | PRUNE; src/clusterlod/support/simplify.rs:28, src/simplify.rs:142 | present |  |
| C++ 1.3 meshopt_SimplifyRegularize | vendor/src/meshoptimizer.h:477 | present | REGULARIZE; src/clusterlod/support/simplify.rs:30, src/simplify.rs:144 | present |  |
| C++ 1.3 meshopt_SimplifyPermissive | vendor/src/meshoptimizer.h:479 | present | PERMISSIVE; src/clusterlod/support/simplify.rs:32, src/simplify.rs:146 | present |  |
| C++ 1.3 meshopt_SimplifyRegularizeLight | vendor/src/meshoptimizer.h:481 | present | REGULARIZE_LIGHT; src/clusterlod/support/simplify.rs:34, src/simplify.rs:148 | present |  |
| C++ 1.3 meshopt_SimplifyPreserveFolds | vendor/src/meshoptimizer.h:483 | present | PRESERVE_FOLDS; src/clusterlod/support/simplify.rs:37, src/simplify.rs:151 | absent | experimental feature |
| C++ 1.3 meshopt_SimplifyErrorClamped | vendor/src/meshoptimizer.h:485 | present | ERROR_CLAMPED; src/clusterlod/support/simplify.rs:40, src/simplify.rs:154 | absent | experimental feature |
| C++ 1.3 meshopt_SimplifyVertex_Lock | vendor/src/meshoptimizer.h:494 | present | LOCK; src/clusterlod/support/input.rs:207, src/input.rs:224 | present |  |
| C++ 1.3 meshopt_SimplifyVertex_Protect | vendor/src/meshoptimizer.h:496 | present | PROTECT; src/clusterlod/support/input.rs:209, src/input.rs:226 | present |  |
| C++ 1.3 meshopt_SimplifyVertex_Priority | vendor/src/meshoptimizer.h:498 | present | PRIORITY; src/clusterlod/support/input.rs:211, src/input.rs:228 | present |  |
| C++ 1.3 meshopt_TangentCompatible | vendor/src/meshoptimizer.h:936 | present | TANGENT_COMPATIBLE; src/tangent.rs:9 | present |  |
| C++ 1.3 meshopt_TangentZeroFallback | vendor/src/meshoptimizer.h:938 | present | TANGENT_ZERO_FALLBACK; src/tangent.rs:11 | present |  |
| C++ 1.3 meshopt_RemeshShell | vendor/src/meshoptimizer.h:978 | present | REMESH_SHELL; src/remesh.rs:11 | absent | experimental feature |
| C++ 1.3 meshopt_RemeshSolve | vendor/src/meshoptimizer.h:980 | present | REMESH_SOLVE; src/remesh.rs:13 | absent | experimental feature |
| C++ cluster LOD demo clodConfig::max_vertices | vendor/demo/clusterlod.h:18 | present | Config::max_vertices; src/clusterlod/mod.rs:18 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::min_triangles | vendor/demo/clusterlod.h:19 | present | Config::min_triangles; src/clusterlod/mod.rs:20 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::max_triangles | vendor/demo/clusterlod.h:20 | present | Config::max_triangles; src/clusterlod/mod.rs:22 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::partition_spatial | vendor/demo/clusterlod.h:24 | present | Config::partition_spatial; src/clusterlod/mod.rs:24 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::partition_sort | vendor/demo/clusterlod.h:25 | present | Config::partition_sort; src/clusterlod/mod.rs:26 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::partition_size | vendor/demo/clusterlod.h:26 | present | Config::partition_size; src/clusterlod/mod.rs:28 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::cluster_spatial | vendor/demo/clusterlod.h:29 | present | Config::cluster_spatial; src/clusterlod/mod.rs:30 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::cluster_fill_weight | vendor/demo/clusterlod.h:30 | present | Config::cluster_fill_weight; src/clusterlod/mod.rs:32 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::cluster_split_factor | vendor/demo/clusterlod.h:31 | present | Config::cluster_split_factor; src/clusterlod/mod.rs:34 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_ratio | vendor/demo/clusterlod.h:34 | present | Config::simplify_ratio; src/clusterlod/mod.rs:36 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_threshold | vendor/demo/clusterlod.h:35 | present | Config::simplify_threshold; src/clusterlod/mod.rs:38 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_error_merge_previous | vendor/demo/clusterlod.h:39 | present | Config::simplify_error_merge_previous; src/clusterlod/mod.rs:40 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_error_merge_additive | vendor/demo/clusterlod.h:40 | present | Config::simplify_error_merge_additive; src/clusterlod/mod.rs:42 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_error_factor_sloppy | vendor/demo/clusterlod.h:43 | present | Config::simplify_error_factor_sloppy; src/clusterlod/mod.rs:44 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_error_edge_limit | vendor/demo/clusterlod.h:46 | present | Config::simplify_error_edge_limit; src/clusterlod/mod.rs:46 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_error_clamped | vendor/demo/clusterlod.h:49 | present | Config::simplify_error_clamped; src/clusterlod/mod.rs:48 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_permissive | vendor/demo/clusterlod.h:52 | present | Config::simplify_permissive; src/clusterlod/mod.rs:50 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_fallback_permissive | vendor/demo/clusterlod.h:55 | present | Config::simplify_fallback_permissive; src/clusterlod/mod.rs:52 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_fallback_sloppy | vendor/demo/clusterlod.h:56 | present | Config::simplify_fallback_sloppy; src/clusterlod/mod.rs:54 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_regularize | vendor/demo/clusterlod.h:59 | present | Config::simplify_regularize; src/clusterlod/mod.rs:56 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_preserve_folds | vendor/demo/clusterlod.h:62 | present | Config::simplify_preserve_folds; src/clusterlod/mod.rs:58 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::simplify_dilate_borders | vendor/demo/clusterlod.h:66 | present | Config::simplify_dilate_borders; src/clusterlod/mod.rs:60 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::optimize_bounds | vendor/demo/clusterlod.h:69 | present | Config::optimize_bounds; src/clusterlod/mod.rs:62 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::optimize_clusters | vendor/demo/clusterlod.h:72 | present | Config::optimize_clusters; src/clusterlod/mod.rs:64 | absent | clusterlod feature |
| C++ cluster LOD demo clodConfig::optimize_clusters_level | vendor/demo/clusterlod.h:73 | present | Config::optimize_clusters_level; src/clusterlod/mod.rs:66 | absent | clusterlod feature |
| optimesh 1.1.0 MeshletOptions::max_vertices | src/clusterizer.rs:38 | present | MeshletSettings / build_meshlets_flex; src/meshlet.rs:32 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 MeshletOptions::min_triangles | src/clusterizer.rs:40 | present | MeshletSettings / build_meshlets_flex; src/meshlet.rs:32 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 MeshletOptions::max_triangles | src/clusterizer.rs:42 | present | MeshletSettings / build_meshlets_flex; src/meshlet.rs:32 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 MeshletOptions::cone_weight | src/clusterizer.rs:44 | present | MeshletSettings / build_meshlets_flex; src/meshlet.rs:32 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 MeshletOptions::split_factor | src/clusterizer.rs:47 | present | MeshletSettings / build_meshlets_flex; src/meshlet.rs:32 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Positions::data | src/clusterizer.rs:54 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Positions::count | src/clusterizer.rs:56 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Positions::stride | src/clusterizer.rs:58 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Stream::data | src/indexgenerator.rs:15 | present | VertexStream; src/remap.rs:13 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Stream::size | src/indexgenerator.rs:17 | present | VertexStream; src/remap.rs:13 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Stream::stride | src/indexgenerator.rs:19 | present | VertexStream; src/remap.rs:13 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 simplifier::SIMPLIFY_LOCK_BORDER | src/simplifier.rs:14 | present | LOCK_BORDER; src/clusterlod/support/simplify.rs:22, src/simplify.rs:136 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_SPARSE | src/simplifier.rs:16 | present | SPARSE; src/clusterlod/support/simplify.rs:24, src/simplify.rs:138 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_ERROR_ABSOLUTE | src/simplifier.rs:19 | present | ERROR_ABSOLUTE; src/clusterlod/support/simplify.rs:26, src/simplify.rs:140 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_PRUNE | src/simplifier.rs:21 | present | PRUNE; src/clusterlod/support/simplify.rs:28, src/simplify.rs:142 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_REGULARIZE | src/simplifier.rs:23 | present | REGULARIZE; src/clusterlod/support/simplify.rs:30, src/simplify.rs:144 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_PERMISSIVE | src/simplifier.rs:25 | present | PERMISSIVE; src/clusterlod/support/simplify.rs:32, src/simplify.rs:146 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_REGULARIZE_LIGHT | src/simplifier.rs:27 | present | REGULARIZE_LIGHT; src/clusterlod/support/simplify.rs:34, src/simplify.rs:148 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_VERTEX_LOCK | src/simplifier.rs:30 | present | LOCK; src/clusterlod/support/input.rs:207, src/input.rs:224 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_VERTEX_PROTECT | src/simplifier.rs:32 | present | PROTECT; src/clusterlod/support/input.rs:209, src/input.rs:226 | present | Same flag value. |
| optimesh 1.1.0 simplifier::SIMPLIFY_VERTEX_PRIORITY | src/simplifier.rs:34 | present | PRIORITY; src/clusterlod/support/input.rs:211, src/input.rs:228 | present | Same flag value. |
| optimesh 1.1.0 VertexData::positions | src/simplifier.rs:47 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 VertexData::count | src/simplifier.rs:49 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 VertexData::stride | src/simplifier.rs:51 | present | Positions; src/clusterlod/support/input.rs:403, src/input.rs:459 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Attributes::data | src/simplifier.rs:57 | present | Attributes; src/clusterlod/support/input.rs:516, src/input.rs:889 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Attributes::stride | src/simplifier.rs:59 | present | Attributes; src/clusterlod/support/input.rs:516, src/input.rs:889 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Attributes::weights | src/simplifier.rs:61 | present | Attributes; src/clusterlod/support/input.rs:516, src/input.rs:889 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 Attributes::count | src/simplifier.rs:63 | present | Attributes; src/clusterlod/support/input.rs:516, src/input.rs:889 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 SimplifyTarget::target_index_count | src/simplifier.rs:69 | present | SimplifySettings; src/clusterlod/support/simplify.rs:76, src/simplify.rs:193 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 SimplifyTarget::target_error | src/simplifier.rs:71 | present | SimplifySettings; src/clusterlod/support/simplify.rs:76, src/simplify.rs:193 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 SimplifyTarget::options | src/simplifier.rs:73 | present | SimplifySettings; src/clusterlod/support/simplify.rs:76, src/simplify.rs:193 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 VertexDataMut::positions | src/simplifier.rs:2975 | present | PositionsMut; src/clusterlod/support/input.rs:12, src/input.rs:12 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 VertexDataMut::count | src/simplifier.rs:2977 | present | PositionsMut; src/clusterlod/support/input.rs:12, src/input.rs:12 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 VertexDataMut::stride | src/simplifier.rs:2979 | present | PositionsMut; src/clusterlod/support/input.rs:12, src/input.rs:12 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 AttributesMut::data | src/simplifier.rs:2985 | present | AttributesMut; src/clusterlod/support/input.rs:104, src/input.rs:113 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 AttributesMut::stride | src/simplifier.rs:2987 | present | AttributesMut; src/clusterlod/support/input.rs:104, src/input.rs:113 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 AttributesMut::weights | src/simplifier.rs:2989 | present | AttributesMut; src/clusterlod/support/input.rs:104, src/input.rs:113 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 AttributesMut::count | src/simplifier.rs:2991 | present | AttributesMut; src/clusterlod/support/input.rs:104, src/input.rs:113 | present | Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views. |
| optimesh 1.1.0 tangentspace::TANGENT_COMPATIBLE | src/tangentspace.rs:15 | present | TANGENT_COMPATIBLE; src/tangent.rs:9 | present | Same flag value. |
| optimesh 1.1.0 tangentspace::TANGENT_ZERO_FALLBACK | src/tangentspace.rs:18 | present | TANGENT_ZERO_FALLBACK; src/tangent.rs:11 | present | Same flag value. |
| optimesh 1.1.0 vcacheoptimizer::VERTEX_SCORE_TABLE | src/vcacheoptimizer.rs:35 | partial | optimize_vertex_cache; src/cache.rs:34 | present | Preset behavior present; public data table and caller tuning absent (S). |
| optimesh 1.1.0 vcacheoptimizer::VERTEX_SCORE_TABLE_STRIP | src/vcacheoptimizer.rs:45 | partial | optimize_vertex_cache_strip; src/cache.rs:314 | present | Preset behavior present; public data table and caller tuning absent (S). |
| Shared numeric/layout controls vertex format versions 0/1, encoder levels 0..9 | See corresponding function declaration in tables | present | VertexEncoding; src/codec/encode.rs:10 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls index versions 0/1 | See corresponding function declaration in tables | present | IndexEncoding; src/codec/encode.rs:53 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls index decode widths 2/4 bytes | See corresponding function declaration in tables | present | decode_index_buffer; src/codec/mod.rs:443 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls meshlet decode vertex widths 2/4 and triangle widths 3/4 bytes | See corresponding function declaration in tables | present | decode_meshlet; src/codec/meshlet.rs:717 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls meshlet max vertices/triangles, cone weight, split/fill factors | See corresponding function declaration in tables | present | build_meshlets_flex; src/meshlet.rs:1722 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls filter strides and bit precision; all four exponent modes | See corresponding function declaration in tables | present | ExpMode; src/codec/encode.rs:82 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls opacity levels, 2/4 states, strides, dimensions, target edge | See corresponding function declaration in tables | present | opacity_map_rasterize; src/opacity.rs:824 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls normal crease angle/smoothing | See corresponding function declaration in tables | present | generate_normals; src/normal.rs:409 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls remesh resolution and max triangle count | See corresponding function declaration in tables | present | remesh; src/remesh.rs:811 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls cache FIFO size; cache analysis warp/primitive-group sizes | See corresponding function declaration in tables | present | analyze_vertex_cache; src/analyze.rs:103 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls partition target size and optional positions | See corresponding function declaration in tables | present | partition_clusters; src/partition.rs:575 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls strip restart sentinel; spatial cluster size | See corresponding function declaration in tables | present | stripify; src/strip.rs:171 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| Shared numeric/layout controls simplification attribute weights, locks, target/error; point colors/weight | See corresponding function declaration in tables | present | simplify_with_attributes; src/clusterlod/support/simplify.rs:152, src/simplify.rs:375 | absent for normals/remesh; otherwise present | Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing. |
| optimesh 1.1.0 vertexfilter::ExpMode::Separate | src/vertexfilter.rs:1709 | present | Separate; src/codec/encode.rs:84, src/codec/encode.rs:89 | present | Equivalent ExpMode variant. |
| optimesh 1.1.0 vertexfilter::ExpMode::SharedVector | src/vertexfilter.rs:1711 | present | SharedVector; src/codec/encode.rs:86 | present | Equivalent ExpMode variant. |
| optimesh 1.1.0 vertexfilter::ExpMode::SharedComponent | src/vertexfilter.rs:1713 | present | SharedComponent; src/codec/encode.rs:88 | present | Equivalent ExpMode variant. |
| optimesh 1.1.0 vertexfilter::ExpMode::Clamped | src/vertexfilter.rs:1715 | present | Clamped; src/codec/encode.rs:90 | present | Equivalent ExpMode variant. |
| optimesh 1.1.0 VertexScoreTable::cache | src/vcacheoptimizer.rs:28 | missing | optimize_vertex_cache / optimize_vertex_cache_strip; src/cache.rs:34 | present | Caller-defined cache-position/live-valence scores; engine tuning users; S, same gap as optimize_vertex_cache_table. |
| optimesh 1.1.0 VertexScoreTable::live | src/vcacheoptimizer.rs:30 | missing | optimize_vertex_cache / optimize_vertex_cache_strip; src/cache.rs:34 | present | Caller-defined cache-position/live-valence scores; engine tuning users; S, same gap as optimize_vertex_cache_table. |

## Per-case measurements

Instructions and wall seconds/call are medians of three. RSS KiB is the maximum of three process peaks, including startup/warmup/serialization; it is not incremental algorithm memory. Each peer ratio uses that case/function's ours median. Per-repetition integer counters, elapsed times, RSS, hashes and binary bindings are in counts.jsonl and stage-counts.jsonl.

| Case | Function | Arm | Instructions/call | Ratio vs ours | Wall s/call | Peak RSS KiB | Output vs C++ |
|---|---|---|---:|---:|---:|---:|---|
| tiny-smooth | simplify | cpp | 120268.776 | 1.070 | 0.000006819 | 15424 | byte-identical |
| tiny-smooth | simplify | ours | 112424.131 | 1.000 | 0.000006442 | 15664 | byte-identical |
| tiny-smooth | simplify | rust | 115872.287 | 1.031 | 0.000006504 | 15488 | byte-identical |
| tiny-smooth | simplify-attributes | cpp | 127506.582 | 0.969 | 0.000006909 | 15460 | byte-identical |
| tiny-smooth | simplify-attributes | ours | 131562.088 | 1.000 | 0.000006801 | 15488 | byte-identical |
| tiny-smooth | simplify-attributes | rust | 133082.233 | 1.012 | 0.000006958 | 15704 | byte-identical |
| tiny-smooth | simplify-sloppy | cpp | 15116.751 | 0.736 | 0.000000865 | 15392 | byte-identical |
| tiny-smooth | simplify-sloppy | ours | 20534.130 | 1.000 | 0.000001014 | 15480 | byte-identical |
| tiny-smooth | simplify-sloppy | rust | 21230.400 | 1.034 | 0.000001107 | 15388 | byte-identical |
| tiny-smooth | cache | cpp | 23701.697 | 0.766 | 0.000001091 | 15468 | byte-identical |
| tiny-smooth | cache | ours | 30948.123 | 1.000 | 0.000001127 | 15728 | byte-identical |
| tiny-smooth | cache | rust | 33387.536 | 1.079 | 0.000001439 | 15468 | byte-identical |
| tiny-smooth | overdraw | cpp | 20220.821 | 1.262 | 0.000001304 | 15436 | byte-identical |
| tiny-smooth | overdraw | ours | 16018.085 | 1.000 | 0.000000887 | 15692 | byte-identical |
| tiny-smooth | overdraw | rust | 17980.383 | 1.123 | 0.000000916 | 15448 | byte-identical |
| tiny-smooth | fetch | cpp | 2619.795 | 0.815 | 0.000000140 | 15784 | byte-identical |
| tiny-smooth | fetch | ours | 3215.129 | 1.000 | 0.000000130 | 15488 | byte-identical |
| tiny-smooth | fetch | rust | 3056.645 | 0.951 | 0.000000142 | 15460 | byte-identical |
| tiny-smooth | meshlets | cpp | 44896.312 | 0.721 | 0.000002214 | 15488 | byte-identical |
| tiny-smooth | meshlets | ours | 62229.124 | 1.000 | 0.000002950 | 15540 | byte-identical |
| tiny-smooth | meshlets | rust | 60844.068 | 0.978 | 0.000002899 | 15452 | byte-identical |
| tiny-smooth | vertex-encode | cpp | 33068.159 | 1.803 | 0.000001417 | 15448 | byte-identical |
| tiny-smooth | vertex-encode | ours | 18338.185 | 1.000 | 0.000000925 | 15644 | byte-identical |
| tiny-smooth | vertex-encode | rust | 34645.206 | 1.889 | 0.000001490 | 15604 | byte-identical |
| tiny-smooth | vertex-decode | cpp | 2824.723 | 0.643 | 0.000000125 | 15744 | byte-identical |
| tiny-smooth | vertex-decode | ours | 4395.129 | 1.000 | 0.000000203 | 15764 | byte-identical |
| tiny-smooth | vertex-decode | rust | 4295.663 | 0.977 | 0.000000368 | 15432 | byte-identical |
| tiny-smooth | index-encode | cpp | 5811.691 | 1.252 | 0.000000232 | 15404 | byte-identical |
| tiny-smooth | index-encode | ours | 4643.075 | 1.000 | 0.000000185 | 15160 | byte-identical |
| tiny-smooth | index-encode | rust | 6985.547 | 1.505 | 0.000000274 | 15208 | byte-identical |
| tiny-smooth | index-decode | cpp | 2159.751 | 0.887 | 0.000000084 | 15488 | byte-identical |
| tiny-smooth | index-decode | ours | 2434.129 | 1.000 | 0.000000102 | 15464 | byte-identical |
| tiny-smooth | index-decode | rust | 2421.681 | 0.995 | 0.000000096 | 15600 | byte-identical |
| tiny-smooth | cluster-lod | cpp | 75739.400 | 0.746 | 0.000005977 | 15708 | byte-identical |
| tiny-smooth | cluster-lod | ours | 101569.200 | 1.000 | 0.000006277 | 15512 | byte-identical |
| medium-smooth | simplify | cpp | 10954805.333 | 0.888 | 0.000969401 | 15476 | byte-identical |
| medium-smooth | simplify | ours | 12335163.333 | 1.000 | 0.000763280 | 15428 | byte-identical |
| medium-smooth | simplify | rust | 12910012.000 | 1.047 | 0.001037716 | 15580 | byte-identical |
| medium-smooth | simplify-attributes | cpp | 25953941.667 | 0.886 | 0.001977976 | 15600 | byte-identical |
| medium-smooth | simplify-attributes | ours | 29300852.333 | 1.000 | 0.001808560 | 15432 | byte-identical |
| medium-smooth | simplify-attributes | rust | 30880382.667 | 1.054 | 0.002151545 | 15616 | byte-identical |
| medium-smooth | simplify-sloppy | cpp | 3116588.333 | 0.740 | 0.000277339 | 15624 | byte-identical |
| medium-smooth | simplify-sloppy | ours | 4211315.667 | 1.000 | 0.000229014 | 15624 | byte-identical |
| medium-smooth | simplify-sloppy | rust | 4310208.667 | 1.023 | 0.000317103 | 15424 | byte-identical |
| medium-smooth | cache | cpp | 8527069.333 | 0.758 | 0.000433506 | 15672 | byte-identical |
| medium-smooth | cache | ours | 11245041.000 | 1.000 | 0.000427722 | 15300 | byte-identical |
| medium-smooth | cache | rust | 11504383.667 | 1.023 | 0.000565217 | 15680 | byte-identical |
| medium-smooth | overdraw | cpp | 1600962.667 | 0.968 | 0.000095236 | 15436 | byte-identical |
| medium-smooth | overdraw | ours | 1653498.333 | 1.000 | 0.000084432 | 15648 | byte-identical |
| medium-smooth | overdraw | rust | 1810834.000 | 1.095 | 0.000096372 | 15704 | byte-identical |
| medium-smooth | fetch | cpp | 364551.000 | 0.860 | 0.000032421 | 15636 | byte-identical |
| medium-smooth | fetch | ours | 424011.667 | 1.000 | 0.000021370 | 15472 | byte-identical |
| medium-smooth | fetch | rust | 446346.000 | 1.053 | 0.000022382 | 15504 | byte-identical |
| medium-smooth | meshlets | cpp | 26954271.667 | 0.949 | 0.002470479 | 15512 | byte-identical |
| medium-smooth | meshlets | ours | 28410630.000 | 1.000 | 0.002273812 | 15640 | byte-identical |
| medium-smooth | meshlets | rust | 36636503.667 | 1.290 | 0.002637854 | 15488 | byte-identical |
| medium-smooth | vertex-encode | cpp | 2202650.667 | 1.792 | 0.000098065 | 15384 | byte-identical |
| medium-smooth | vertex-encode | ours | 1229148.333 | 1.000 | 0.000050585 | 15444 | byte-identical |
| medium-smooth | vertex-encode | rust | 2923245.667 | 2.378 | 0.000127423 | 15568 | byte-identical |
| medium-smooth | vertex-decode | cpp | 173357.333 | 0.752 | 0.000008686 | 15632 | byte-identical |
| medium-smooth | vertex-decode | ours | 230421.333 | 1.000 | 0.000010560 | 15464 | byte-identical |
| medium-smooth | vertex-decode | rust | 330536.333 | 1.434 | 0.000016077 | 15608 | byte-identical |
| medium-smooth | index-encode | cpp | 1183898.333 | 1.039 | 0.000054917 | 15332 | byte-identical |
| medium-smooth | index-encode | ours | 1139294.333 | 1.000 | 0.000042206 | 15484 | byte-identical |
| medium-smooth | index-encode | rust | 1530113.333 | 1.343 | 0.000065370 | 15668 | byte-identical |
| medium-smooth | index-decode | cpp | 398844.333 | 0.922 | 0.000026587 | 15516 | byte-identical |
| medium-smooth | index-decode | ours | 432666.333 | 1.000 | 0.000020425 | 15564 | byte-identical |
| medium-smooth | index-decode | rust | 452271.333 | 1.045 | 0.000019717 | 15400 | byte-identical |
| medium-smooth | cluster-lod | cpp | 99770297.000 | 0.839 | 0.009644348 | 15484 | byte-identical |
| medium-smooth | cluster-lod | ours | 118868404.000 | 1.000 | 0.010110085 | 15232 | byte-identical |
| medium-seams | simplify | cpp | 14175027.333 | 0.904 | 0.001487793 | 15440 | byte-identical |
| medium-seams | simplify | ours | 15680852.333 | 1.000 | 0.001066443 | 15520 | byte-identical |
| medium-seams | simplify | rust | 16608103.333 | 1.059 | 0.001404052 | 15536 | byte-identical |
| medium-seams | simplify-attributes | cpp | 26013797.000 | 0.872 | 0.002257194 | 15732 | byte-identical |
| medium-seams | simplify-attributes | ours | 29845569.667 | 1.000 | 0.002026073 | 15424 | byte-identical |
| medium-seams | simplify-attributes | rust | 30914518.000 | 1.036 | 0.002269394 | 15688 | byte-identical |
| medium-seams | simplify-sloppy | cpp | 5117541.333 | 0.686 | 0.000403676 | 15452 | byte-identical |
| medium-seams | simplify-sloppy | ours | 7465281.000 | 1.000 | 0.000367358 | 15676 | byte-identical |
| medium-seams | simplify-sloppy | rust | 9019927.333 | 1.208 | 0.000592575 | 15436 | byte-identical |
| medium-seams | cache | cpp | 7418315.333 | 0.752 | 0.000452585 | 15584 | byte-identical |
| medium-seams | cache | ours | 9860618.000 | 1.000 | 0.000402157 | 15476 | byte-identical |
| medium-seams | cache | rust | 10141802.000 | 1.029 | 0.000752556 | 15448 | byte-identical |
| medium-seams | overdraw | cpp | 1594540.667 | 0.951 | 0.000096141 | 15440 | byte-identical |
| medium-seams | overdraw | ours | 1676545.667 | 1.000 | 0.000093269 | 15468 | byte-identical |
| medium-seams | overdraw | rust | 1828403.667 | 1.091 | 0.000095754 | 15588 | byte-identical |
| medium-seams | fetch | cpp | 395886.667 | 0.882 | 0.000029803 | 15352 | byte-identical |
| medium-seams | fetch | ours | 448949.667 | 1.000 | 0.000021708 | 15576 | byte-identical |
| medium-seams | fetch | rust | 491091.333 | 1.094 | 0.000025692 | 15660 | byte-identical |
| medium-seams | meshlets | cpp | 20692739.667 | 0.949 | 0.001596591 | 15416 | byte-identical |
| medium-seams | meshlets | ours | 21809534.333 | 1.000 | 0.001471145 | 15640 | byte-identical |
| medium-seams | meshlets | rust | 29007402.667 | 1.330 | 0.001956692 | 15736 | byte-identical |
| medium-seams | vertex-encode | cpp | 2673634.000 | 1.696 | 0.000119087 | 15736 | byte-identical |
| medium-seams | vertex-encode | ours | 1576646.000 | 1.000 | 0.000086606 | 15472 | byte-identical |
| medium-seams | vertex-encode | rust | 3438541.667 | 2.181 | 0.000153068 | 15780 | byte-identical |
| medium-seams | vertex-decode | cpp | 200255.667 | 0.745 | 0.000010052 | 15456 | byte-identical |
| medium-seams | vertex-decode | ours | 268909.667 | 1.000 | 0.000012413 | 15644 | byte-identical |
| medium-seams | vertex-decode | rust | 386329.667 | 1.437 | 0.000020906 | 15544 | byte-identical |
| medium-seams | index-encode | cpp | 1495904.000 | 1.045 | 0.000064228 | 15576 | byte-identical |
| medium-seams | index-encode | ours | 1431774.667 | 1.000 | 0.000057735 | 15488 | byte-identical |
| medium-seams | index-encode | rust | 1788660.667 | 1.249 | 0.000084125 | 15464 | byte-identical |
| medium-seams | index-decode | cpp | 496057.667 | 0.923 | 0.000022466 | 15388 | byte-identical |
| medium-seams | index-decode | ours | 537251.000 | 1.000 | 0.000022469 | 15348 | byte-identical |
| medium-seams | index-decode | rust | 557366.667 | 1.037 | 0.000037096 | 15688 | byte-identical |
| medium-seams | cluster-lod | cpp | 94525113.000 | 0.804 | 0.008356034 | 15632 | byte-identical |
| medium-seams | cluster-lod | ours | 117578918.000 | 1.000 | 0.009519534 | 15508 | byte-identical |
| medium-sparse | simplify | cpp | 11864420.333 | 0.865 | 0.001069339 | 15516 | byte-identical |
| medium-sparse | simplify | ours | 13721337.667 | 1.000 | 0.000825270 | 15488 | byte-identical |
| medium-sparse | simplify | rust | 14069847.333 | 1.025 | 0.001071015 | 15560 | byte-identical |
| medium-sparse | simplify-attributes | cpp | 22982303.000 | 0.842 | 0.002201653 | 15680 | byte-identical |
| medium-sparse | simplify-attributes | ours | 27300038.000 | 1.000 | 0.001605024 | 15436 | byte-identical |
| medium-sparse | simplify-attributes | rust | 27116800.333 | 0.993 | 0.002315097 | 15548 | byte-identical |
| medium-sparse | simplify-sloppy | cpp | 3839299.000 | 0.708 | 0.000408823 | 15400 | byte-identical |
| medium-sparse | simplify-sloppy | ours | 5422362.667 | 1.000 | 0.000301724 | 15712 | byte-identical |
| medium-sparse | simplify-sloppy | rust | 5792914.667 | 1.068 | 0.000532282 | 15524 | byte-identical |
| medium-sparse | cache | cpp | 8598039.333 | 0.760 | 0.000514291 | 15824 | byte-identical |
| medium-sparse | cache | ours | 11309443.000 | 1.000 | 0.000463642 | 15416 | byte-identical |
| medium-sparse | cache | rust | 11551958.000 | 1.021 | 0.000651215 | 15364 | byte-identical |
| medium-sparse | overdraw | cpp | 1630259.333 | 0.922 | 0.000103344 | 15500 | byte-identical |
| medium-sparse | overdraw | ours | 1767516.667 | 1.000 | 0.000094928 | 15396 | byte-identical |
| medium-sparse | overdraw | rust | 1877951.667 | 1.062 | 0.000115684 | 15492 | byte-identical |
| medium-sparse | fetch | cpp | 364453.667 | 0.859 | 0.000047897 | 15788 | byte-identical |
| medium-sparse | fetch | ours | 424190.333 | 1.000 | 0.000021150 | 15496 | byte-identical |
| medium-sparse | fetch | rust | 449247.667 | 1.059 | 0.000046755 | 15436 | byte-identical |
| medium-sparse | meshlets | cpp | 26982087.000 | 0.946 | 0.002390736 | 15492 | byte-identical |
| medium-sparse | meshlets | ours | 28523021.000 | 1.000 | 0.002315374 | 15492 | byte-identical |
| medium-sparse | meshlets | rust | 36651749.333 | 1.285 | 0.002754770 | 15428 | byte-identical |
| medium-sparse | vertex-encode | cpp | 3657790.667 | 1.719 | 0.000152220 | 15428 | byte-identical |
| medium-sparse | vertex-encode | ours | 2128089.667 | 1.000 | 0.000092992 | 15480 | byte-identical |
| medium-sparse | vertex-encode | rust | 4721431.000 | 2.219 | 0.000209023 | 15448 | byte-identical |
| medium-sparse | vertex-decode | cpp | 219088.667 | 0.683 | 0.000011495 | 15560 | byte-identical |
| medium-sparse | vertex-decode | ours | 320730.667 | 1.000 | 0.000014143 | 15664 | byte-identical |
| medium-sparse | vertex-decode | rust | 480162.667 | 1.497 | 0.000022723 | 15488 | byte-identical |
| medium-sparse | index-encode | cpp | 1396718.667 | 1.004 | 0.000054640 | 15180 | byte-identical |
| medium-sparse | index-encode | ours | 1390893.667 | 1.000 | 0.000052626 | 15660 | byte-identical |
| medium-sparse | index-encode | rust | 1698343.667 | 1.221 | 0.000073081 | 15528 | byte-identical |
| medium-sparse | index-decode | cpp | 531274.667 | 0.953 | 0.000020832 | 15412 | byte-identical |
| medium-sparse | index-decode | ours | 557601.667 | 1.000 | 0.000021430 | 15448 | byte-identical |
| medium-sparse | index-decode | rust | 613638.667 | 1.100 | 0.000023851 | 15400 | byte-identical |
| medium-sparse | cluster-lod | cpp | 100760086.000 | 0.835 | 0.009115333 | 15404 | byte-identical |
| medium-sparse | cluster-lod | ours | 120685289.000 | 1.000 | 0.010337262 | 15464 | byte-identical |
| million-smooth | simplify | cpp | 2256441913.000 | 0.919 | 0.306435199 | 131144 | byte-identical |
| million-smooth | simplify | ours | 2455428135.000 | 1.000 | 0.267446089 | 143704 | byte-identical |
| million-smooth | simplify | rust | 2665342931.000 | 1.085 | 0.322998117 | 129576 | byte-identical |
| million-smooth | simplify-attributes | cpp | 3643738776.000 | 0.886 | 0.415691288 | 192040 | byte-identical |
| million-smooth | simplify-attributes | ours | 4110632307.000 | 1.000 | 0.385240986 | 206924 | byte-identical |
| million-smooth | simplify-attributes | rust | 4385835344.000 | 1.067 | 0.425431547 | 190536 | byte-identical |
| million-smooth | simplify-sloppy | cpp | 338172403.000 | 0.758 | 0.052880000 | 68972 | byte-identical |
| million-smooth | simplify-sloppy | ours | 446146386.000 | 1.000 | 0.043230611 | 79148 | byte-identical |
| million-smooth | simplify-sloppy | rust | 431763636.000 | 0.968 | 0.057669623 | 67428 | byte-identical |
| million-smooth | cache | cpp | 1058999801.000 | 0.759 | 0.095712391 | 66436 | byte-identical |
| million-smooth | cache | ours | 1395908326.000 | 1.000 | 0.078058370 | 76908 | byte-identical |
| million-smooth | cache | rust | 1425849095.000 | 1.021 | 0.110021668 | 65120 | byte-identical |
| million-smooth | overdraw | cpp | 191786674.000 | 0.971 | 0.019322370 | 55560 | byte-identical |
| million-smooth | overdraw | ours | 197575548.000 | 1.000 | 0.013430985 | 64636 | byte-identical |
| million-smooth | overdraw | rust | 217331167.000 | 1.100 | 0.017888994 | 54096 | byte-identical |
| million-smooth | fetch | cpp | 44118675.000 | 0.857 | 0.005895775 | 84980 | byte-identical |
| million-smooth | fetch | ours | 51484119.000 | 1.000 | 0.005963552 | 85540 | byte-identical |
| million-smooth | fetch | rust | 53951253.000 | 1.048 | 0.005889543 | 83592 | byte-identical |
| million-smooth | meshlets | cpp | 3870400907.000 | 0.937 | 0.362340684 | 108484 | byte-identical |
| million-smooth | meshlets | ours | 4131410486.000 | 1.000 | 0.341175815 | 108152 | byte-identical |
| million-smooth | meshlets | rust | 5189479173.000 | 1.256 | 0.410461487 | 113880 | byte-identical |
| million-smooth | vertex-encode | cpp | 243856073.000 | 1.647 | 0.011989853 | 39672 | byte-identical |
| million-smooth | vertex-encode | ours | 148085579.000 | 1.000 | 0.006672005 | 38472 | byte-identical |
| million-smooth | vertex-encode | rust | 310578207.000 | 2.097 | 0.015991633 | 38640 | byte-identical |
| million-smooth | vertex-decode | cpp | 17353379.000 | 0.738 | 0.001430322 | 44276 | byte-identical |
| million-smooth | vertex-decode | ours | 23503537.000 | 1.000 | 0.000981296 | 42980 | byte-identical |
| million-smooth | vertex-decode | rust | 34125326.000 | 1.452 | 0.002084503 | 42732 | byte-identical |
| million-smooth | index-encode | cpp | 141296406.000 | 1.033 | 0.009786536 | 44020 | byte-identical |
| million-smooth | index-encode | ours | 136822709.000 | 1.000 | 0.005912476 | 42508 | byte-identical |
| million-smooth | index-encode | rust | 183774502.000 | 1.343 | 0.011407216 | 42508 | byte-identical |
| million-smooth | index-decode | cpp | 48042306.000 | 0.923 | 0.006291229 | 55536 | byte-identical |
| million-smooth | index-decode | ours | 52051068.000 | 1.000 | 0.006726278 | 54044 | byte-identical |
| million-smooth | index-decode | rust | 54544400.000 | 1.048 | 0.007436845 | 53996 | byte-identical |
| million-smooth | cluster-lod | cpp | 13010266674.000 | 0.828 | 1.191882126 | 115948 | byte-identical |
| million-smooth | cluster-lod | ours | 15711240234.000 | 1.000 | 1.359566276 | 168704 | byte-identical |
| real-214e | simplify | cpp | 2813433357.000 | 0.733 | 1.092747539 | 924392 | byte-identical |
| real-214e | simplify | ours | 3840016644.000 | 1.000 | 1.001087823 | 1048676 | byte-identical |
| real-214e | simplify | rust | 3510227120.000 | 0.914 | 1.165408852 | 1022916 | byte-identical |
| real-214e | simplify-attributes | cpp | 4926656932.000 | 0.680 | 1.536225556 | 1747692 | byte-identical |
| real-214e | simplify-attributes | ours | 7245934164.000 | 1.000 | 1.148306223 | 1898468 | byte-identical |
| real-214e | simplify-attributes | rust | 5677184641.000 | 0.783 | 1.690067976 | 1846172 | byte-identical |
| real-214e | simplify-sloppy | cpp | 2299205414.000 | 0.661 | 0.636701283 | 561896 | byte-identical |
| real-214e | simplify-sloppy | ours | 3478559530.000 | 1.000 | 0.654371595 | 593732 | byte-identical |
| real-214e | simplify-sloppy | rust | 4305397063.000 | 1.238 | 0.753738675 | 560372 | byte-identical |
| real-214e | cache | cpp | 1803471321.000 | 0.764 | 0.170462629 | 440016 | byte-identical |
| real-214e | cache | ours | 2361193145.000 | 1.000 | 0.124852299 | 472112 | byte-identical |
| real-214e | cache | rust | 2453329397.000 | 1.039 | 0.187112891 | 438512 | byte-identical |
| real-214e | overdraw | cpp | 992615861.000 | 0.681 | 0.115662856 | 374208 | byte-identical |
| real-214e | overdraw | ours | 1458435688.000 | 1.000 | 0.111458334 | 405312 | byte-identical |
| real-214e | overdraw | rust | 1227652034.000 | 0.842 | 0.124046342 | 372692 | byte-identical |
| real-214e | fetch | cpp | 305130973.000 | 1.057 | 0.089453235 | 505596 | byte-identical |
| real-214e | fetch | ours | 288540173.000 | 1.000 | 0.079114319 | 530936 | byte-identical |
| real-214e | fetch | rust | 406312813.000 | 1.408 | 0.094205447 | 504020 | byte-identical |
| real-214e | meshlets | cpp | 13286173018.000 | 0.710 | 2.284336483 | 526200 | byte-identical |
| real-214e | meshlets | ours | 18710574387.000 | 1.000 | 2.320071394 | 567708 | byte-identical |
| real-214e | meshlets | rust | 16651671965.000 | 0.890 | 2.178531260 | 544148 | byte-identical |
| real-214e | vertex-encode | cpp | 4731050516.000 | 1.837 | 0.263912476 | 386036 | byte-identical |
| real-214e | vertex-encode | ours | 2575258648.000 | 1.000 | 0.174726784 | 412376 | byte-identical |
| real-214e | vertex-encode | rust | 5773706465.000 | 2.242 | 0.321030362 | 384580 | byte-identical |
| real-214e | vertex-decode | cpp | 365793104.000 | 0.792 | 0.057079574 | 438804 | byte-identical |
| real-214e | vertex-decode | ours | 461997147.000 | 1.000 | 0.059624863 | 437512 | byte-identical |
| real-214e | vertex-decode | rust | 651683954.000 | 1.411 | 0.073391359 | 437212 | byte-identical |
| real-214e | index-encode | cpp | 1089077280.000 | 1.538 | 0.060587285 | 284924 | byte-identical |
| real-214e | index-encode | ours | 707961959.000 | 1.000 | 0.052925887 | 319968 | byte-identical |
| real-214e | index-encode | rust | 1187106691.000 | 1.677 | 0.063039249 | 283372 | byte-identical |
| real-214e | index-decode | cpp | 175022877.000 | 1.046 | 0.029214256 | 346404 | byte-identical |
| real-214e | index-decode | ours | 167347862.000 | 1.000 | 0.029529740 | 345080 | byte-identical |
| real-214e | index-decode | rust | 187523258.000 | 1.121 | 0.030105062 | 344828 | byte-identical |
| real-214e | cluster-lod | cpp | 65734150381.000 | 0.705 | 21.302599974 | 643060 | byte-identical |
| real-214e | cluster-lod | ours | 93214330529.000 | 1.000 | 9.350876366 | 708616 | byte-identical |
| real-a082 | simplify | cpp | 5024122900.000 | 0.778 | 2.451928241 | 994340 | byte-identical |
| real-a082 | simplify | ours | 6458992632.000 | 1.000 | 2.211079763 | 1122132 | byte-identical |
| real-a082 | simplify | rust | 6399032761.000 | 0.991 | 2.565106762 | 1098124 | byte-identical |
| real-a082 | simplify-attributes | cpp | 7595291764.000 | 0.723 | 3.809289145 | 1870836 | byte-identical |
| real-a082 | simplify-attributes | ours | 10500555913.000 | 1.000 | 3.150898908 | 2026784 | byte-identical |
| real-a082 | simplify-attributes | rust | 9086074882.000 | 0.865 | 3.701099031 | 1974676 | byte-identical |
| real-a082 | simplify-sloppy | cpp | 2315103730.000 | 0.666 | 0.863477744 | 589688 | byte-identical |
| real-a082 | simplify-sloppy | ours | 3474691335.000 | 1.000 | 0.872656006 | 626804 | byte-identical |
| real-a082 | simplify-sloppy | rust | 4026367259.000 | 1.159 | 0.976723956 | 588192 | byte-identical |
| real-a082 | cache | cpp | 2079324689.000 | 0.761 | 0.695233233 | 463488 | byte-identical |
| real-a082 | cache | ours | 2734108181.000 | 1.000 | 0.693761404 | 500492 | byte-identical |
| real-a082 | cache | rust | 2844728686.000 | 1.040 | 0.714616620 | 461772 | byte-identical |
| real-a082 | overdraw | cpp | 1076432085.000 | 0.696 | 0.372604425 | 386988 | byte-identical |
| real-a082 | overdraw | ours | 1546984769.000 | 1.000 | 0.448795705 | 426540 | byte-identical |
| real-a082 | overdraw | rust | 1324635884.000 | 0.856 | 0.334616157 | 385492 | byte-identical |
| real-a082 | fetch | cpp | 332041778.000 | 1.047 | 0.208995418 | 532152 | byte-identical |
| real-a082 | fetch | ours | 317149373.000 | 1.000 | 0.168310205 | 559008 | byte-identical |
| real-a082 | fetch | rust | 440519432.000 | 1.389 | 0.220560632 | 530544 | byte-identical |
| real-a082 | meshlets | cpp | 14278322363.000 | 0.721 | 2.167098957 | 561240 | byte-identical |
| real-a082 | meshlets | ours | 19807727052.000 | 1.000 | 2.397135754 | 608204 | byte-identical |
| real-a082 | meshlets | rust | 18036881077.000 | 0.911 | 2.408515086 | 582028 | byte-identical |
| real-a082 | vertex-encode | cpp | 5085026640.000 | 1.726 | 0.274231032 | 346568 | byte-identical |
| real-a082 | vertex-encode | ours | 2946391978.000 | 1.000 | 0.185549968 | 400688 | byte-identical |
| real-a082 | vertex-encode | rust | 6232422708.000 | 2.115 | 0.308797381 | 345064 | byte-identical |
| real-a082 | vertex-decode | cpp | 410937389.000 | 0.801 | 0.068766759 | 455112 | byte-identical |
| real-a082 | vertex-decode | ours | 513099511.000 | 1.000 | 0.070865307 | 453688 | byte-identical |
| real-a082 | vertex-decode | rust | 724684221.000 | 1.412 | 0.083036332 | 453412 | byte-identical |
| real-a082 | index-encode | cpp | 1301755803.000 | 0.922 | 0.081376118 | 313808 | byte-identical |
| real-a082 | index-encode | ours | 1412361189.000 | 1.000 | 0.090014594 | 342832 | byte-identical |
| real-a082 | index-encode | rust | 1377353709.000 | 0.975 | 0.082682477 | 312072 | byte-identical |
| real-a082 | index-decode | cpp | 333349592.000 | 0.851 | 0.038511072 | 362264 | byte-identical |
| real-a082 | index-decode | ours | 391562238.000 | 1.000 | 0.043805486 | 361000 | byte-identical |
| real-a082 | index-decode | rust | 374955723.000 | 0.958 | 0.041457086 | 360744 | byte-identical |
| real-a082 | cluster-lod | cpp | 74324030912.000 | 0.707 | 8.771620797 | 689188 | byte-identical |
| real-a082 | cluster-lod | ours | 105108793867.000 | 1.000 | 9.498722983 | 779104 | byte-identical |
| real-b6fd | simplify | cpp | 1250240984.000 | 0.776 | 0.297517967 | 258160 | byte-identical |
| real-b6fd | simplify | ours | 1612173318.000 | 1.000 | 0.267240479 | 289336 | byte-identical |
| real-b6fd | simplify | rust | 1597148262.000 | 0.991 | 0.325699901 | 283064 | byte-identical |
| real-b6fd | simplify-attributes | cpp | 1892982009.000 | 0.722 | 0.420779908 | 477364 | byte-identical |
| real-b6fd | simplify-attributes | ours | 2622682195.000 | 1.000 | 0.312623964 | 515348 | byte-identical |
| real-b6fd | simplify-attributes | rust | 2268784880.000 | 0.865 | 0.458225044 | 502220 | byte-identical |
| real-b6fd | simplify-sloppy | cpp | 636190856.000 | 0.663 | 0.095888248 | 161424 | byte-identical |
| real-b6fd | simplify-sloppy | ours | 960031117.000 | 1.000 | 0.095512881 | 164896 | byte-identical |
| real-b6fd | simplify-sloppy | rust | 1177886973.000 | 1.227 | 0.125754182 | 159912 | byte-identical |
| real-b6fd | cache | cpp | 519828138.000 | 0.761 | 0.026290685 | 124608 | byte-identical |
| real-b6fd | cache | ours | 683531718.000 | 1.000 | 0.029819586 | 137000 | byte-identical |
| real-b6fd | cache | rust | 710726118.000 | 1.040 | 0.036030063 | 123152 | byte-identical |
| real-b6fd | overdraw | cpp | 269119661.000 | 0.696 | 0.017979505 | 108032 | byte-identical |
| real-b6fd | overdraw | ours | 386758335.000 | 1.000 | 0.024578884 | 119580 | byte-identical |
| real-b6fd | overdraw | rust | 331167755.000 | 0.856 | 0.020585759 | 106516 | byte-identical |
| real-b6fd | fetch | cpp | 82384495.000 | 1.047 | 0.007045970 | 168888 | byte-identical |
| real-b6fd | fetch | ours | 78663341.000 | 1.000 | 0.005773716 | 167448 | byte-identical |
| real-b6fd | fetch | rust | 109504106.000 | 1.392 | 0.007086266 | 167152 | byte-identical |
| real-b6fd | meshlets | cpp | 3427834119.000 | 0.716 | 0.513654615 | 148476 | byte-identical |
| real-b6fd | meshlets | ours | 4785506802.000 | 1.000 | 0.613174036 | 159492 | byte-identical |
| real-b6fd | meshlets | rust | 4350614062.000 | 0.909 | 0.524840365 | 152312 | byte-identical |
| real-b6fd | vertex-encode | cpp | 1277173404.000 | 1.852 | 0.064616048 | 117760 | byte-identical |
| real-b6fd | vertex-encode | ours | 689523079.000 | 1.000 | 0.037924910 | 123128 | byte-identical |
| real-b6fd | vertex-encode | rust | 1548272293.000 | 2.245 | 0.080013564 | 116208 | byte-identical |
| real-b6fd | vertex-decode | cpp | 100124427.000 | 0.795 | 0.014259133 | 130484 | byte-identical |
| real-b6fd | vertex-decode | ours | 125963489.000 | 1.000 | 0.014799550 | 129072 | byte-identical |
| real-b6fd | vertex-decode | rust | 177400073.000 | 1.408 | 0.017683718 | 128748 | byte-identical |
| real-b6fd | index-encode | cpp | 285558034.000 | 1.540 | 0.018010433 | 99300 | byte-identical |
| real-b6fd | index-encode | ours | 185403964.000 | 1.000 | 0.008095325 | 98004 | byte-identical |
| real-b6fd | index-encode | rust | 314671981.000 | 1.697 | 0.019000937 | 97780 | byte-identical |
| real-b6fd | index-decode | cpp | 49353009.000 | 1.045 | 0.002668602 | 107256 | byte-identical |
| real-b6fd | index-decode | ours | 47215016.000 | 1.000 | 0.002787646 | 105776 | byte-identical |
| real-b6fd | index-decode | rust | 52807287.000 | 1.118 | 0.002956273 | 105716 | byte-identical |
| real-b6fd | cluster-lod | cpp | 18418152480.000 | 0.707 | 2.068048777 | 185796 | byte-identical |
| real-b6fd | cluster-lod | ours | 26039279131.000 | 1.000 | 2.542518291 | 219524 | byte-identical |
| real-214e-s01 | simplify | cpp | 2813433559.000 | 0.733 | 1.036574108 | 868332 | byte-identical |
| real-214e-s01 | simplify | ours | 3840017696.000 | 1.000 | 1.000173969 | 992544 | byte-identical |
| real-214e-s01 | simplify | rust | 3510227468.000 | 0.914 | 1.168429532 | 966916 | byte-identical |
| real-214e-s02 | simplify | cpp | 2813433580.000 | 0.733 | 1.056418624 | 868336 | byte-identical |
| real-214e-s02 | simplify | ours | 3840017337.000 | 1.000 | 0.996762168 | 992544 | byte-identical |
| real-214e-s02 | simplify | rust | 3510228241.000 | 0.914 | 1.248988744 | 966872 | byte-identical |
| real-214e-s05 | simplify | cpp | 2813433802.000 | 0.733 | 1.091653862 | 868316 | byte-identical |
| real-214e-s05 | simplify | ours | 3840017435.000 | 1.000 | 0.987135694 | 992464 | byte-identical |
| real-214e-s05 | simplify | rust | 3510227268.000 | 0.914 | 1.168296042 | 966912 | byte-identical |
| real-214e-s09 | simplify | cpp | 2813433378.000 | 0.733 | 0.996577963 | 868340 | byte-identical |
| real-214e-s09 | simplify | ours | 3840016841.000 | 1.000 | 0.902202983 | 992312 | byte-identical |
| real-214e-s09 | simplify | rust | 3510227276.000 | 0.914 | 1.157119399 | 966880 | byte-identical |
| real-214e-s10 | simplify | cpp | 2813433454.000 | 0.733 | 1.075825286 | 868324 | byte-identical |
| real-214e-s10 | simplify | ours | 3840017816.000 | 1.000 | 1.001030492 | 992308 | byte-identical |
| real-214e-s10 | simplify | rust | 3510227674.000 | 0.914 | 1.156205070 | 966908 | byte-identical |
| real-214e-s13 | simplify | cpp | 2813433971.000 | 0.733 | 1.360289340 | 868320 | byte-identical |
| real-214e-s13 | simplify | ours | 3840018701.000 | 1.000 | 1.163757934 | 992292 | byte-identical |
| real-214e-s13 | simplify | rust | 3510227976.000 | 0.914 | 1.308617580 | 966884 | byte-identical |
| real-a082-s01 | simplify | cpp | 5024124600.000 | 0.778 | 3.860457622 | 949632 | byte-identical |
| real-a082-s01 | simplify | ours | 6458997358.000 | 1.000 | 3.934583188 | 1077144 | byte-identical |
| real-a082-s01 | simplify | rust | 6399033493.000 | 0.991 | 4.107814175 | 1053252 | byte-identical |
| real-a082-s04 | simplify | cpp | 4304470529.000 | 0.773 | 3.609752564 | 949636 | byte-identical |
| real-a082-s04 | simplify | ours | 5565328542.000 | 1.000 | 2.924227378 | 1077140 | byte-identical |
| real-a082-s04 | simplify | rust | 5351880345.000 | 0.962 | 3.789127395 | 1053268 | byte-identical |
| real-a082-s07 | simplify | cpp | 4304475427.000 | 0.773 | 3.980076079 | 949456 | byte-identical |
| real-a082-s07 | simplify | ours | 5565331636.000 | 1.000 | 3.204555232 | 1077316 | byte-identical |
| real-a082-s07 | simplify | rust | 5351880940.000 | 0.962 | 4.521552264 | 1053168 | byte-identical |
| real-a082-s11 | simplify | cpp | 5024124215.000 | 0.778 | 3.006681801 | 949492 | byte-identical |
| real-a082-s11 | simplify | ours | 6458995696.000 | 1.000 | 2.742026645 | 1077128 | byte-identical |
| real-a082-s11 | simplify | rust | 6399034499.000 | 0.991 | 3.008641689 | 1053280 | byte-identical |
| real-a082-s14 | simplify | cpp | 4304469646.000 | 0.773 | 2.462522890 | 949476 | byte-identical |
| real-a082-s14 | simplify | ours | 5565326587.000 | 1.000 | 2.170274732 | 1077144 | byte-identical |
| real-a082-s14 | simplify | rust | 5351877368.000 | 0.962 | 2.737454732 | 1053232 | byte-identical |
| real-a082-s17 | simplify | cpp | 4304471159.000 | 0.773 | 2.532592358 | 949472 | byte-identical |
| real-a082-s17 | simplify | ours | 5565326781.000 | 1.000 | 2.118288572 | 1077160 | byte-identical |
| real-a082-s17 | simplify | rust | 5351875730.000 | 0.962 | 2.397211111 | 1053192 | byte-identical |
| real-b6fd-s01 | simplify | cpp | 1250240729.000 | 0.776 | 0.306810007 | 241244 | byte-identical |
| real-b6fd-s01 | simplify | ours | 1612173379.000 | 1.000 | 0.301078363 | 272196 | byte-identical |
| real-b6fd-s01 | simplify | rust | 1597148104.000 | 0.991 | 0.338288611 | 266020 | byte-identical |
| real-b6fd-s04 | simplify | cpp | 1070407462.000 | 0.771 | 0.295460267 | 241240 | byte-identical |
| real-b6fd-s04 | simplify | ours | 1388823855.000 | 1.000 | 0.297951350 | 272236 | byte-identical |
| real-b6fd-s04 | simplify | rust | 1335462215.000 | 0.962 | 0.329941246 | 266004 | byte-identical |
| real-b6fd-s07 | simplify | cpp | 1070407785.000 | 0.771 | 0.327753533 | 241220 | byte-identical |
| real-b6fd-s07 | simplify | ours | 1388823919.000 | 1.000 | 0.297699913 | 272220 | byte-identical |
| real-b6fd-s07 | simplify | rust | 1335462412.000 | 0.962 | 0.350009725 | 266040 | byte-identical |
| real-b6fd-s11 | simplify | cpp | 1250240707.000 | 0.776 | 0.336923977 | 241248 | byte-identical |
| real-b6fd-s11 | simplify | ours | 1612173400.000 | 1.000 | 0.300279354 | 272244 | byte-identical |
| real-b6fd-s11 | simplify | rust | 1597148070.000 | 0.991 | 0.346916916 | 266016 | byte-identical |
| real-b6fd-s14 | simplify | cpp | 1070407483.000 | 0.771 | 0.297697609 | 241292 | byte-identical |
| real-b6fd-s14 | simplify | ours | 1388823759.000 | 1.000 | 0.263288294 | 272216 | byte-identical |
| real-b6fd-s14 | simplify | rust | 1335462145.000 | 0.962 | 0.312054594 | 266008 | byte-identical |
| real-b6fd-s17 | simplify | cpp | 1070407436.000 | 0.771 | 0.296967531 | 241240 | byte-identical |
| real-b6fd-s17 | simplify | ours | 1388823758.000 | 1.000 | 0.268193168 | 272224 | byte-identical |
| real-b6fd-s17 | simplify | rust | 1335462184.000 | 0.962 | 0.332209712 | 266044 | byte-identical |

## Evidence and reproduction

README.md gives the admitted build/prepare/measure/validate sequence. REGISTRATION.md fixes allocation policy and settings. binding.json seals source, binaries, input hashes, compiler/perf versions and CPU description. audit.json and receipts.json retain exit codes and raw-evidence hashes; cleanup.json binds deletion of the dedicated target. Full binaries, private assets, output buffers, raw perf/time logs, admission logs and excluded attempts remain in the retained scratch directory. No private paths or private names are required to inspect the public tables.
