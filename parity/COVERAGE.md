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
