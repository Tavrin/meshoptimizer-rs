# Lane 1 coverage

The complete upstream release 0.1 call set is listed here. This foundation
lane implements two functions; it does not qualify the whole 0.1 release.

| Upstream public function | Rust API | Status |
|---|---|---|
| `meshopt_optimizeVertexCache` | `optimize_vertex_cache`, `_into`, `_in_place` | implemented; exact corpus and sweep |
| `meshopt_optimizeOverdraw` | `optimize_overdraw`, `_into`, `_in_place` | implemented; exact corpus and sweep |
| `meshopt_simplify` | scheduled `simplify` | lane 2 |
| `meshopt_simplifyWithAttributes` | scheduled `simplify_with_attributes` | lane 2; flags and Permissive |
| `meshopt_simplifyScale` | scheduled `simplify_scale` | lane 2 |

Remaining stable preprocessing belongs to 0.1.x; codecs, meshlets, analysis
and experimental algorithms belong to later RFC milestones. The full 89-name
release ledger is outside lane 1's requested 0.1 function inventory.

## Fixture applicability

| Upstream suite | Applicable cases for the two shipped modules | Treatment |
|---|---|---|
| `demo/tests.cpp` | `emptyMesh`: standard cache and overdraw empty calls | both exchanged through C++ and Rust; no other direct calls to the shipped modules |
| `js/meshopt_encoder.test.js` | `reorderMesh` uses `optsize=true`, selecting cache Strip plus fetch remapping | unchanged upstream sanity; topology reused as an additional standard-cache/overdraw input, not claimed as Strip/fetch output parity |
| `js/meshopt_decoder.test.js` | none; decoding/filtering | unchanged upstream sanity only |
| `js/meshopt_simplifier.test.js` | none; simplification | unchanged upstream sanity only |
| `js/meshopt_clusterizer.test.js` | none; meshlets/clusters | unchanged upstream sanity only |
| `js/meshopt_tangents.test.js` | none; tangents | unchanged upstream sanity only |

The upstream decoder suite's asynchronous calls are not a Rust differential
coverage count. All five unmodified suites run as sanity evidence; no direct
shipped-module async fixture is omitted. Actual Rust differential cases have
identifiers and awaited native/WASM responses recorded individually.

The lane corpus also contains disconnected and unused vertices, repeated and
degenerate triangles, signed zero, subnormals, threshold values below one,
Moss's 1.05 threshold, and a 1,000,002-index mesh. The seeded sweep repeats the
large case and varies topology, valence, scale and thresholds. Overdraw inputs
are first standard-cache optimized, matching its documented upstream usage.

## Targets and variants

| Target or variant | Evidence |
|---|---|
| Linux x86-64, allocating, no default core features | exact C++ comparison, retained buffers and source/executable identities |
| wasm32, no default core features | executed Node WASM adapter; exact native-Rust message/output identity |
| default std core features | focused API tests and stable mutation smoke; same pinned math backend |
| caller-buffer and in-place | focused contract tests and successful-output comparisons in fuzz smoke |
| Linux/macOS/Windows | CI fmt/clippy/test jobs; only local Linux execution is recorded here |
| AArch64 | later release execution qualification |

Invalid layouts, indices, thresholds, geometry, destination sizes, overflow and
limits are Rust robustness cases. The protocol also rejects malformed headers,
sizes, topology and non-finite inputs on both drivers. Undefined C++ numerical
cases are not executed for comparison bytes.
