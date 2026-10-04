# Geometry coverage

| Upstream public function | Rust API | Status |
|---|---|---|
| `meshopt_optimizeVertexCache` | `optimize_vertex_cache`, `_into`, `_in_place` | exact corpus and sweep |
| `meshopt_optimizeOverdraw` | `optimize_overdraw`, `_into`, `_in_place` | exact corpus and sweep |
| `meshopt_simplify` | `simplify`, `simplify_into` | exact indices and error bits |
| `meshopt_simplifyWithAttributes` | `simplify_with_attributes`, `simplify_with_attributes_into` | exact indices/error; weights and all three flags |
| `meshopt_simplifyScale` | `simplify_scale` | exact extent bits |

The 0.1.x extension exposes stable Sparse and Prune and retains the existing
stable options. PreserveFolds and ErrorClamped require `experimental`. Unknown
bits remain checked errors. Codecs, meshlets, stripification and analysis retain
their later RFC milestones. Qualification records below are separate from the
existing 0.1 records.
The `experimental` feature currently exposes no additional functions.

## 0.1 fixture applicability (historical records)

`native-fixtures.py` extracts the unchanged bodies of these functions from the
pinned `demo/tests.cpp`, preserving their assertions and capturing inputs before
mutation: simplify, simplifyStuck, simplifyFlip, simplifyScale,
simplifyDegenerate, simplifyLockBorder, simplifyAttr (both zero-weight cases),
simplifyLockFlags, simplifyLockFlagsSeam, simplifyErrorAbsolute, simplifySeam,
simplifySeamFake and simplifySeamAttr. These yield 23 native calls. Empty inputs
are also represented in the shared corpus for all five functions.

Native Sloppy/Points/Prune/Update, Sparse, InternalDebug and PreserveFolds tests
use unimplemented functions/options; they do not establish shipped API coverage.
The native fixture exporter links unmodified upstream implementation sources.

All five upstream JS suites run unchanged as sanity checks. Additionally,
`js-fixtures.cjs` captures six applicable simplifier calls: simplify, simplify16
(explicit u32 widening), simplifyLockBorder, simplifyAttr, simplifyLockFlags and
getScale. Captured inputs are executed against C++, native Rust and wasm32 Rust.
The adapter awaits module readiness. JS compactMesh, simplifyUpdate,
simplifyPoints and simplifyPrune are later-scope functions. Encoder reorderMesh
uses Strip plus fetch; its topology remains an extra standard-cache/overdraw
input, with no claim of Strip/fetch parity. Decoder, clusterizer and tangent
suites are upstream sanity only; their asynchronous counts are not Rust coverage.

The shared corpus has 50 cases: grids, shared and fully split spheres with
normal/UV/tangent/color streams, locked/protected/priority vertices, non-manifold
edges, degeneracy, disconnected and unused vertices, signed zero/subnormals,
thresholds below one, and a 1,000,002-index grid. Seeded sweeps mix valid grids,
split/shared spheres and irregular topology; vary 0–32 attribute components,
zero and positive weights, all supported options, all flag combinations,
Moss target/error values and additional boundary values. Overdraw receives
standard-cache output as upstream recommends. Every result error is serialized
as f32 bits and compared along with exact output count/order.

## 0.1 targets and variants (historical records)

| Target or variant | Evidence |
|---|---|
| Linux x86-64, allocating, no default core features | C++ scalar-strict differential, retained buffers and identities |
| wasm32, no default core features | executed Node WASM adapter, native Rust byte identity |
| default std | focused API tests and mutation smoke; identical libm backend |
| caller-buffer simplification | focused tests and successful-output/error identity in fuzz smoke |
| optimizer caller-buffer/in-place | focused tests and fuzz variant/atomicity checks |
| Linux/macOS/Windows | CI build/test matrix; only local Linux execution claimed |
| AArch64 | later release execution qualification |

Invalid topology, indices, layouts, dimensions, weights, flags, finite-value
policies, destination sizes, checked overflow and exact resource boundaries are
Rust robustness tests, not undefined C++ comparison inputs. The stable fuzz
fallback runs all five entry points for 600 elapsed seconds apiece. It has no
sanitizer or coverage instrumentation and does not replace release fuzz gates.

## P02 codec coverage addition

| Upstream operation | Rust operation in codec | Scope |
|---|---|---|
| meshopt_decodeVertexBuffer | decode_vertex_buffer / decode_vertex_buffer_into | Raw v0/v1 |
| meshopt_decodeIndexBuffer | decode_index_buffer / decode_index_buffer_into | Raw v0/v1, u16/u32 output |
| meshopt_decodeIndexSequence | decode_index_sequence / decode_index_sequence_into | Raw v0/v1, u16/u32 output |
| meshopt_decodeVertexVersion | decode_vertex_version | Header inspection |
| meshopt_decodeIndexVersion | decode_index_version | Triangle and sequence headers |
| meshopt_decodeFilterOct | decode_filter_oct | Scalar canonical, strides 4/8 |
| meshopt_decodeFilterQuat | decode_filter_quat | Scalar canonical, stride 8 |
| meshopt_decodeFilterExp | decode_filter_exp | Scalar canonical, four-byte words |
| Consumer helper | BufferView / decode_buffer_view / decode_buffer_view_into | EXT rules, raw v1 excluded from attributes |

P02 does not modify geometry implementations or Moss integration. See
codec/README.md, DECODER_BAR.md and P02_PERFORMANCE.md for evidence, APIs and
qualification limits.

## 0.4 codec completion

| Upstream operation | Rust operation in codec | Scope |
|---|---|---|
| meshopt_encodeVertexBuffer, meshopt_encodeVertexBufferLevel | encode_vertex_buffer / encode_vertex_buffer_into with VertexEncoding | v0/v1, levels 0-9 |
| meshopt_encodeVertexBufferBound | encode_vertex_buffer_bound | Checked arithmetic |
| meshopt_encodeVertexVersion | VertexEncoding (per call) | Intentional replacement of the global setter |
| meshopt_encodeIndexBuffer | encode_index_buffer / encode_index_buffer_into with IndexEncoding | v0/v1, u32 input |
| meshopt_encodeIndexBufferBound | encode_index_buffer_bound | Checked arithmetic |
| meshopt_encodeIndexSequence | encode_index_sequence / encode_index_sequence_into | v0/v1, u32 input |
| meshopt_encodeIndexSequenceBound | encode_index_sequence_bound | Checked arithmetic |
| meshopt_encodeIndexVersion | IndexEncoding (per call) | Intentional replacement of the global setter |
| meshopt_encodeFilterOct | encode_filter_oct / _into | Strides 4/8 |
| meshopt_encodeFilterQuat | encode_filter_quat / _into | Stride 8 |
| meshopt_encodeFilterExp | encode_filter_exp / _into with ExpMode | All four exponent modes |
| meshopt_encodeFilterColor | encode_filter_color / _into | Strides 4/8 |
| meshopt_decodeFilterColor | decode_filter_color | Scalar canonical, outside the EXT helper |
| meshopt_encodeMeshlet | encode_meshlet / encode_meshlet_into | 0.3 layout slices, at most 256/256 |
| meshopt_encodeMeshletBound | encode_meshlet_bound | Checked arithmetic |
| meshopt_decodeMeshlet | decode_meshlet / decode_meshlet_into | Vertex size 2/4, triangle size 3/4 |
| meshopt_decodeMeshletRaw | decode_meshlet_raw / decode_meshlet_raw_into | u32 references and packed triangles |

Evidence: `parity/results/run-0.4.json`, `sweep-0.4.json`,
`benchmark-0.4-moss.json`, `benchmark-0.4-default.json` and `fuzz-0.4.json`,
verified by `parity/report.sh --phase 0.4 --verify-artifacts`. Decisions
D59-D66 record the domains, undefined-reference cases and the meshlet layout
dependency.

## Phase 0.3 inventory (p03, pinned C++ 1.3)

This additive section supersedes the earlier statement that meshlets are
scheduled. The exact inventory below was checked against `src/meshoptimizer.h`
at `4c203430ca565cb59a468a91922c76c208169536`; its `src` tree matches v1.3.
Meshlet codecs remain phase 0.4. No earlier phase's status is changed here.

| Upstream function | Rust API | Meaningful parity fields |
|---|---|---|
| `meshopt_buildMeshlets` | `build_meshlets`, `_into` | descriptors, vertices, triangle bytes |
| `meshopt_buildMeshletsScan` | `build_meshlets_scan`, `_into` | descriptors, vertices, triangle bytes |
| `meshopt_buildMeshletsFlex` | `build_meshlets_flex`, `_into` | descriptors, vertices, triangle bytes |
| `meshopt_buildMeshletsSpatial` | `build_meshlets_spatial`, `_into` | descriptors, vertices, triangle bytes |
| `meshopt_buildMeshletsBound` | `build_meshlets_bound` | capacity |
| `meshopt_computeClusterBounds` | `compute_cluster_bounds` | 11 f32 fields and 4 signed bytes |
| `meshopt_computeMeshletBounds` | `compute_meshlet_bounds` | 11 f32 fields and 4 signed bytes |
| `meshopt_computeSphereBounds` | `compute_sphere_bounds` | 11 f32 fields and 4 signed bytes |
| `meshopt_optimizeMeshlet` | `optimize_meshlet`, `_into`, `_in_place` | vertex order, triangle bytes, unused suffix |
| `meshopt_optimizeMeshletLevel` | `optimize_meshlet_level`, `_into`, `_in_place` | levels 0–9, rotations and unused suffix |
| `meshopt_extractMeshletIndices` | `extract_meshlet_indices`, `_into` | first-visit vertices and local triangle bytes |
| `meshopt_partitionClusters` | `partition_clusters`, `_into` | partition count and assignments |
| `meshopt_spatialSortRemap` | `spatial_sort_remap`, `_into` | old-to-new remap |
| `meshopt_spatialSortTriangles` | `spatial_sort_triangles`, `_into`, `_in_place` | ordered triangle indices |
| `meshopt_spatialClusterPoints` | `spatial_cluster_points`, `_into` | new-to-old clustered point order |

`parity/p03` captures 31 native calls from 13 unchanged fixture bodies and
16 calls from four unchanged JS clusterizer tests. Native bodies: clusterBoundsDegenerate,
sphereBounds, meshletsEmpty, meshletsDense, meshletsSparse, meshletsFlex,
meshletsMax, extractMeshlet, meshletsSpatial, meshletsSpatialDeep,
partitionBasic, partitionSpatial, partitionSpatialMerge. JS tests: buildMeshlets,
computeClusterBounds, computeMeshletBounds, computeSphereBounds. The exact
47-file inventory is checked, rather than just its cardinality.

All 15 functions also receive deterministic generated fixtures and 2,000 seeded
cases each. Buffer-returning functions compare allocating and caller-buffer
forms separately. Native Rust and executed wasm32 Rust both compare with the
unmodified scalar-strict C++ implementation. Invalid inputs, exact limits,
strided and byte-position layouts, hash collisions, wrapping valence and atomic
errors are checked on the Rust boundary without invoking undefined C++ behavior.
Actual run status is recorded in the phase 0.3 measured-result additions.

The optional `clusterlod` feature contains the demo builder, configuration
presets, callback and allocating outputs, bound optimization, cluster
optimization, and hierarchy bound/build helpers. Its 80 comparison fixtures
cover RT and standard presets, every configuration switch, custom callback IDs,
attribute widths 1/2/8/12/32, protects/flags, shared and split geometry, dilation,
optimization levels, hierarchy sizes and empty input. This reproduces the
pinned demo; it is not a stable upstream library-output contract.

Every listed function has its own cargo-fuzz target. Five-minute lane smokes
use coverage instrumentation and no sanitizer; they do not establish the
amended four-CPU-hour release gate, nightly/weekly sustained fuzzing, 10,000-case
release sweeps, AArch64 execution or non-Linux platform acceptance.

## 0.1.x header inventory

Pinned `src/meshoptimizer.h` at 4c203430ca565cb59a468a91922c76c208169536
contains the following additional preprocessing functions. All are implemented
in safe Rust and pass the final local gates recorded in `MEASURED_P01X.json` and
DECISIONS D145.

| Upstream function | Rust API / record family |
|---|---|
| `meshopt_optimizeVertexCacheStrip` | `vertex_cache_strip` |
| `meshopt_optimizeVertexCacheFifo` | `vertex_cache_fifo` |
| `meshopt_generateVertexRemap` | `generate_vertex_remap` |
| `meshopt_generateVertexRemapMulti` | `generate_vertex_remap_multi` |
| `meshopt_generateVertexRemapCustom` | `generate_vertex_remap_custom` |
| `meshopt_remapVertexBuffer` | `remap_vertex_buffer` |
| `meshopt_remapIndexBuffer` | `remap_index_buffer` |
| `meshopt_filterIndexBuffer` | `filter_index_buffer` |
| `meshopt_filterIndexBufferMulti` | `filter_index_buffer_multi` |
| `meshopt_generateShadowIndexBuffer` | `generate_shadow_index_buffer` |
| `meshopt_generateShadowIndexBufferMulti` | `generate_shadow_index_buffer_multi` |
| `meshopt_generatePositionRemap` | `generate_position_remap` |
| `meshopt_generateAdjacencyIndexBuffer` | `generate_adjacency_index_buffer` |
| `meshopt_generateTessellationIndexBuffer` | `generate_tessellation_index_buffer` |
| `meshopt_generateProvokingIndexBuffer` | `generate_provoking_index_buffer` |
| `meshopt_optimizeVertexFetch` | `vertex_fetch` |
| `meshopt_optimizeVertexFetchRemap` | `vertex_fetch_remap` |
| `meshopt_simplifySloppy` | `simplify_sloppy` |
| `meshopt_simplifyPrune` | `simplify_prune` |
| `meshopt_simplifyPoints` | `simplify_points` |
| `meshopt_simplifyWithUpdate` | `simplify_with_update` |
| `meshopt_quantizeUnorm` | `quantize_unorm` |
| `meshopt_quantizeSnorm` | `quantize_snorm` |
| `meshopt_quantizeHalf` | `quantize_half` |
| `meshopt_quantizeFloat` | `quantize_float` |
| `meshopt_dequantizeHalf` | `dequantize_half` |
| `meshopt_computePositionExponent` | `compute_position_exponent` |

The five additional option families are Sparse, Prune, PreserveFolds,
ErrorClamped and RegularizeLight. Each has independent fixture, seeded sweep,
WASM, fuzz and benchmark records. Protocol 2 includes complete update positions
and attributes, used fetch bytes, provoking reorder entries and custom callback
visits. Native fixture capture retains unchanged inputs before mutation.

No extra provoking helpers exist in this header. C++ typed index overloads are
adapters to these functions. `setAllocator` is intentionally replaced by
per-call Workspace and Limits; it is not a global Rust callback API.

Caller-buffer forms that use an atomic temporary document that allocation and
charge it to limits. Direct forms document partial writes on work exhaustion.
Remap and fetch accept arbitrary index sequences; triangle-only operations
validate divisibility by three. Initialized padding participates in byte keys.

Additional work units count index validation, each hash probe, visited record,
cell/triangle scan, union-find traversal, adjacency edge and update solve visit.
Scalar arithmetic and bytes within one validated record are not separate units.
Heap reservations and size checks are fallible. Quantization and exponent
helpers have bounded scalar work and allocate no heap storage.

## 0.1.x final local qualification

All 32 new record families pass fixtures, 2,000 seeded cases each, executed WASM
identity and a 300-second mutation smoke. Both complete 912-case consumer timing
matrices are verified; the unconditional performance verdict is
`documented fetch maximum residual; coordinator decision pending`. Existing 0.1 fixture and seeded regressions also pass: 924 fixture
comparisons and 74,000 seeded comparisons total, with zero mismatches. All eleven
local checks pass, including 78 root tests in each feature mode. The full per-family
bar verdicts, source/executable identities and retained artifact hashes are in
`MEASURED_P01X.json`; DECISIONS D145 records the decisive verdict and proof limits.

The additional large fixed-width fetch fixtures cover all 4-, 8-, 12- and
16-byte owned-record paths in C++, native Rust and executed WASM. The independent
verifier checks every archive member and exact matrix membership, including D144's
complete predetermined workload order. Every current measured sample is retained.

This establishes the brief's local Linux x86-64 and executed WASM gates. It does
not establish other native-platform execution, release fuzz, GPU or Moss
integration acceptance. D84 records the earlier artifact placement limitation;
integration pointers use the requested destination pending the coordinator's move.
