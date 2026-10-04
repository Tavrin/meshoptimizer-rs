# Geometry coverage

| Upstream public function | Rust API | Status |
|---|---|---|
| `meshopt_optimizeVertexCache` | `optimize_vertex_cache`, `_into`, `_in_place` | exact corpus and sweep |
| `meshopt_optimizeOverdraw` | `optimize_overdraw`, `_into`, `_in_place` | exact corpus and sweep |
| `meshopt_simplify` | `simplify`, `simplify_into` | exact indices and error bits |
| `meshopt_simplifyWithAttributes` | `simplify_with_attributes`, `simplify_with_attributes_into` | exact indices/error; weights and all three flags |
| `meshopt_simplifyScale` | `simplify_scale` | exact extent bits |

Supported options: empty, LockBorder, ErrorAbsolute, Regularize, Permissive,
RegularizeLight. Unsupported bits are rejected. Sparse and Prune, remaining
stable preprocessing, codecs, meshlets and experimental algorithms retain their
RFC later-milestone scheduling. PreserveFolds and ErrorClamped are not exposed.
The `experimental` feature currently exposes no additional functions.

## Fixture applicability

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

## Targets and variants

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
