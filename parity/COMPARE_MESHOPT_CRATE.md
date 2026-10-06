# meshoptimizer-rs 0.2.0 versus meshopt 0.6.2

Measured source: `dcaf997ef55d2c4ccc05ce0ef885d2d9a16018d0` (`cmp/meshopt-crate`); Linux x86-64, 2026-10-06. This is a measurement/documentation comparison, not release or cross-platform acceptance.

## Versions and scope

`meshoptimizer-rs` ports upstream meshoptimizer **1.3**, pinned at `4c203430ca565cb59a468a91922c76c208169536` (the `src` tree equals tag v1.3). `meshopt 0.6.2` bundles **v0.25** (`6daea4695c48338363b08022d2fb15deaef6ac09`): all 17 packaged `vendor/src` files match that tag after CRLF normalization. Raw file hashes and normalized comparisons are retained in `vendor-version.json`; the version is verified from source, not inferred from crate numbering.

The [crates.io API](https://crates.io/api/v1/crates/meshopt) confirmed 0.6.2 as the latest stable version at collection time, published 2025-10-09, with 151,163 recent downloads. That count is a dated metadata observation, not a quality measure. Package sources: [meshopt 0.6.2](https://docs.rs/crate/meshopt/0.6.2/source/).

## Feature matrix

Available means callable in the published crate, including its public unsafe FFI module. FFI-only is explicitly distinguished from an idiomatic Rust wrapper. All upstream header families appear below; the exhaustive 89-name ledger follows in the appendix.

| Family / capability | meshoptimizer-rs 0.2.0 | meshopt 0.6.2 |
|---|---|---|
| Vertex remapping, multi-stream/custom, remap buffers | Available, checked slices/Results | Available; custom is FFI-only |
| Shadow, position, adjacency/tessellation/provoking index generation | Available | Available; some helpers FFI-only |
| Degenerate/duplicate index filtering | Available, single/multi-stream | Missing |
| Vertex cache standard/strip/FIFO; overdraw; vertex fetch/remap | Available | Available; strip cache FFI-only |
| Vertex/index/sequence codecs, bounds, versions and compression levels | Available; versions/levels per call | Available; sequence/levels/versions chiefly FFI; global defaults |
| Oct/Quat/Exp/Color filter encoders/decoders | Available | Available through FFI; no idiomatic filter wrappers |
| Meshlet encode/decode/raw/bounds | Available | Missing |
| Simplify, attributes/update/sloppy/prune/points/scale | Available; typed settings and mutable views | Available; update/prune/points FFI-only |
| Simplification flags | Includes 1.3 PreserveFolds, ErrorClamped, RegularizeLight and Priority | 0.25 flags; later flags missing |
| Stripify/unstripify and bounds | Available | Available |
| Cache/fetch/overdraw/coverage analysis | Available | Available; coverage FFI-only |
| Meshlet builders standard/scan/flex/spatial and bounds | Available; tightly packed local triangles | Available; 4-byte padded local triangles |
| Meshlet optimization / level / local-index extraction | All available | Basic optimization available; level/extraction missing |
| Cluster/meshlet/sphere bounds | Available | Available |
| Partition clusters, spatial sort vertices/triangles/points | Available | Available; spatial helpers FFI-only |
| Opacity micromaps; tangents | Available | Missing |
| Normal generation; voxel remesh | Available behind experimental | Missing |
| Quantize Unorm/Snorm/Half/Float; dequantize Half | Available; checked bit settings | Available; Unorm/Snorm implemented in Rust |
| Position exponent selection | Available | Missing |
| Cluster LOD and hierarchy | Available behind clusterlod; port of upstream demo/clusterlod.h | Missing; not part of 0.25 C header |
| Parallel batch encode/decode/meshlets/LOD/hierarchy | Available behind parallel, Rayon; deterministic per-input results | Missing built-in batch API; caller can schedule independent calls |
| SIMD decoding | Default simd; audited x86 SSE2/SSSE3/SSE4.1, AArch64 NEON, wasm SIMD; canonical scalar filters | Bundled C++ SIMD; selected by compiler/architecture; no meshlet codec |
| no_std | Available with alloc; disable default features; parallel requires std | Missing; Rust wrappers/bindings depend on std |
| Allocation control | Fallible Vec reservations, reusable Workspace, per-call work/byte Limits, caller buffers | Global C++ allocator callbacks via FFI; Rust Vec allocations remain separate; many wrappers allocate |
| wasm32 without a C/C++ toolchain | Available | Missing |
| wasm32 with a compatible C toolchain | Available | Build script supplies stripped headers; needs Wasm-capable compiler/archiver |

## Build and footprint

One clean and one touched-consumer rebuild were measured per backend/profile on the same host. Clean means an empty lane Cargo target with Cargo sources already cached; it does not mean cold filesystem caches. Rebuild means touching `src/decode.rs`, recompiling/relinking only the consumer, with `CARGO_INCREMENTAL=0`; it is not a warm no-op build or Rust incremental compilation. Queue waiting is excluded. No completed clean measurement was repeated to select a faster result.

The initial all-feature build was exploratory and excluded. Driver compile fixes and the rejected stale decoder capture are preserved in MILESTONES.md. Later footprint-only rebuilds do not replace the original time samples.

Moss: opt-level 3, thin LTO, one codegen unit. Defaults: opt-level 3, LTO off, sixteen codegen units. Both strip symbols. Rust flags/wrappers are empty. The C++ crate uses its published `cc` build configuration at O3, with no added C++ LTO or CPU-tuning flags. Its dependency profile does not inherit the crate-local release profile.

| Decode consumer build | Ours clean / rebuild (s) | Theirs clean / rebuild (s) |
|---|---|---|
| defaults | 7.954 / 0.192 | 6.632 / 0.202 |
| moss | 12.305 / 2.344 | 8.373 / 2.450 |

| Property | Ours | Theirs |
|---|---|---|
| C/C++ toolchain for a consumer | No | Yes; `cc` compiles 16 C++ translation units |
| Own Rust code unsafe keyword tokens | 23, all in audited SIMD; 0 with SIMD source excluded | 140, including 66 hand-written tokens and 74 generated tokens (71 extern blocks plus callback types) |
| MSRV | Declared Rust 1.88 | No rust-version declared; see compile checks below; no minimum inferred from success |
| Licence | MIT, including upstream-derived algorithms | Rust MIT OR Apache-2.0; bundled C++ MIT |
| Native targets | x86_64 and AArch64 source support | x86_64 and AArch64 with a suitable C/C++ toolchain |

Unsafe counts are lexical keyword tokens after excluding comments and string literals, not unsafe-line counts or a safety proof. Default SIMD means ours is pure Rust, but not entirely safe Rust. Scalar/no-SIMD builds forbid unsafe; SIMD uses a documented intrinsic boundary. The C++ source is outside the Rust unsafe-token metric.

Compile-only checks (execution/linking on other targets is not established):

| Check | Exit | Interpretation |
|---|---|---|
| ours-no-std-wasm-no-c | 0 | PASS, compile only |
| ours-aarch64 | 0 | PASS, compile only |
| theirs-wasm-no-c | 101 | Expected missing C compiler |
| theirs-aarch64 | 101 | Environment/toolchain unavailable; see retained log |
| ours-msrv | 0 | PASS, compile only |
| theirs-rust-1.88 | 0 | PASS, compile only |

Stripped executable sizes in bytes. Contribution is decode/full minus the matched empty consumer; text/data/bss deltas are retained in footprint.json. This includes required Rust support code but excludes dynamic system libraries, heap/stack allocations and Cargo artifacts. The full program retains upstream-equivalent API roots plus ours’ caller forms, experimental APIs, cluster LOD and parallel APIs; it is intentionally a broader surface than the older crate. It retains callback implementations with simple concrete callbacks, not every possible monomorphization.

| Profile / program | Ours file / contribution | Theirs file / contribution |
|---|---|---|
| defaults / decode | 470,736 / +123,600 | 369,832 / +22,696 |
| defaults / full | 1,966,176 / +1,619,040 | 518,056 / +170,920 |
| moss / decode | 434,368 / +101,504 | 355,400 / +22,536 |
| moss / full | 1,828,208 / +1,495,344 | 503,992 / +171,128 |

## Shared-function performance

Comparators are resident single-thread drivers pinned to one physical CPU selected from three one-second utilization samples. The machine is an AMD Ryzen 9 7945HX, performance governor, rustc 1.98.1, GCC C++. Shared-load telemetry is retained; these are not isolated-machine results. Allocation, validation, required copies, algorithm scratch and output destruction are timed. Geometry generation, input parsing, fixture encoding for decodes, I/O, serialization, meshlet-padding normalization and independent validity checks are excluded. Workspace capacity is warm.

Theirs uses allocating adapters around the published crate’s public FFI entry points, not an independently rebuilt upstream checkout. Idiomatic wrappers sometimes copy data, change return types, or optimize each generated meshlet; those additional steps are not included. Thus ratios describe matched allocating algorithm adapters, not every wrapper/application. SIMD stays enabled on both sides. Explicit v0 and v1 encodes separate version effects from the default-version comparison.

Five alternating pairs per case, extended once to twenty only when the initial interval straddles 1.0. Case point estimates are medians of paired elapsed-time ratios **ours/theirs** (below 1 is faster for ours). Intervals are nominal paired Student-t intervals on mean log ratios, not confidence intervals for the displayed median; no simultaneous/sequential adjustment or significance claim is made. No load-driven repeat or favorable sample removal occurs.

Five frozen geometry inputs reused from `parity/performance.py` and `runner.grid`: 32 triangles smooth; 8,192 smooth, seam-heavy and sparse; 1,000,000 smooth. Codec/filter fixtures are frozen from those inputs, with varied normalized filter vectors. Bounds helpers use at most the first 124 frozen triangles; local meshlet utility helpers use one explicit local triangle and an initialized vertex-reference prefix. Sphere bounds uses all points. Frozen hashes and generator identities are retained. This is a comparison subset of the existing corpus, not the complete release qualification matrix.

Operation `vertex-encode` explicitly selects v1 on both sides; `vertex-encode-v0` selects v0. `vertex-encode-default` retains each library's published default. Filter encoders use matched precision settings and verified decoder fixtures. The timing consumer enables ours' full feature surface, while decode-only footprint uses default std/SIMD features.

Complete timing corpus: 29 operations × five inputs × two profiles = 290 cases, 2,905 retained pairs. The untimed ledger covers 71 operations × five inputs = 355 rows; 330 have canonically identical outputs. Timed output hashes are invariant across the two profiles.

Operation summary: geometric mean of five case medians; bracketed range is the **envelope of case 95% intervals**, not an interval for the aggregate. Individual case intervals follow in the appendix.

| Operation | Defaults GM [case CI envelope] | Moss GM [case CI envelope] |
|---|---|---|
| vertex-decode | 1.368 [1.024, 2.488] | 1.267 [0.850, 2.065] |
| vertex-encode | 0.937 [0.830, 1.070] | 0.923 [0.754, 1.091] |
| vertex-encode-v0 | 1.267 [1.094, 1.512] | 1.326 [1.096, 1.635] |
| vertex-encode-default | 1.637 [1.237, 2.335] | 1.669 [1.445, 3.090] |
| index-decode | 0.942 [0.744, 1.288] | 0.987 [0.879, 1.184] |
| index-encode | 1.207 [0.955, 1.502] | 1.197 [0.932, 1.577] |
| sequence-decode | 0.950 [0.802, 1.238] | 1.011 [0.818, 1.686] |
| sequence-encode | 1.369 [1.172, 1.674] | 1.346 [1.159, 1.629] |
| oct-decode | 1.079 [0.890, 1.195] | 1.076 [0.904, 1.200] |
| quat-decode | 1.179 [1.067, 1.234] | 1.180 [1.042, 1.273] |
| exp-decode | 0.940 [0.875, 1.106] | 0.958 [0.858, 1.192] |
| color-decode | 0.939 [0.850, 1.032] | 0.918 [0.851, 0.978] |
| oct-encode | 1.394 [1.058, 2.250] | 1.491 [1.111, 3.025] |
| quat-encode | 1.919 [1.524, 2.149] | 1.919 [1.342, 2.291] |
| exp-encode | 3.403 [2.479, 4.365] | 3.389 [2.808, 3.837] |
| color-encode | 0.865 [0.759, 0.969] | 0.839 [0.787, 0.969] |
| cache | 1.239 [0.933, 1.538] | 1.125 [0.787, 1.382] |
| cache-strip | 1.162 [0.777, 1.558] | 1.091 [0.592, 1.396] |
| cache-fifo | 1.190 [0.682, 1.573] | 1.265 [0.722, 1.868] |
| overdraw | 1.203 [0.951, 1.686] | 1.562 [1.032, 2.195] |
| fetch | 1.136 [0.887, 1.428] | 1.102 [0.884, 1.369] |
| fetch-remap | 1.298 [1.009, 1.661] | 1.237 [1.049, 1.414] |
| simplify | 0.985 [0.760, 1.217] | 0.941 [0.717, 1.090] |
| stripify | 1.032 [0.764, 1.314] | 0.970 [0.833, 1.045] |
| unstripify | 0.992 [0.867, 1.090] | 0.952 [0.880, 1.127] |
| meshlets | 1.126 [0.942, 1.775] | 1.075 [0.955, 1.512] |
| meshlets-scan | 0.858 [0.682, 1.091] | 0.717 [0.440, 1.015] |
| meshlets-flex | 1.095 [0.953, 1.612] | 1.074 [0.920, 1.427] |
| meshlets-spatial | 1.080 [0.903, 1.174] | 1.107 [0.968, 1.290] |

Version effects are a plausible contributor to builder/bounds/partition/filter differences: upstream [v1.0](https://github.com/zeux/meshoptimizer/releases/tag/v1.0) changed the vertex default to v1, removed meshlet padding, improved sparse clusterization and update simplification; [v1.1](https://github.com/zeux/meshoptimizer/releases/tag/v1.1) improved bounds and Color alpha precision; [v1.2](https://github.com/zeux/meshoptimizer/releases/tag/v1.2) accelerated native vertex decoding and partitioning; [v1.3](https://github.com/zeux/meshoptimizer/releases/tag/v1.3) tightened vertex-encoding capacity bounds and improved spatial builders, partitioning and permissive simplification. These changelog links explain documented changes; the crate-to-crate timings do not isolate compiler, language, wrapper, version or safety-check costs.

## Output differences and validity

Every tested shared header function is represented by the operation ledger below, including bounds/version helpers and two Rust inline-equivalent quantizers. Encoder default setters are represented by explicit configurations. Global allocator mutation is intentionally not exercised: ours replaces that API with per-call controls, and it has no comparable output.

Identity is meaningful serialized output, not native struct padding. Meshlet raw identity includes the older local-triangle padding and native offsets; canonical identity removes only padding, preserving partitions, local order and referenced vertices.

| Operation | Exact on frozen cases | Difference / independent validity |
|---|---|---|
| vertex-decode | 5/5 | lossless original bytes |
| vertex-encode | 5/5 | both decoders; exact vertices/sequence or oriented triangle multiset |
| vertex-encode-v0 | 5/5 | both decoders; exact vertices/sequence or oriented triangle multiset |
| vertex-encode-default | 0/5 | v1.0 default changed v0 → v1. both decoders; exact vertices/sequence or oriented triangle multiset |
| index-decode | 5/5 | oriented triangle multiset |
| index-encode | 5/5 | both decoders; exact vertices/sequence or oriented triangle multiset |
| sequence-decode | 5/5 | lossless sequence |
| sequence-encode | 5/5 | both decoders; exact vertices/sequence or oriented triangle multiset |
| oct-decode | 1/5 | Canonical scalar Rust versus C++ SIMD rounding on the same encoded fixture; not attributed to a version change. expected layout; unit decoded directions/quaternions or finite exponential values |
| quat-decode | 5/5 | expected layout; unit decoded directions/quaternions or finite exponential values |
| exp-decode | 5/5 | expected layout; unit decoded directions/quaternions or finite exponential values |
| color-decode | 1/5 | Canonical scalar Rust versus C++ SIMD rounding on the same encoded fixture; not attributed to a version change. expected layout; unit decoded directions/quaternions or finite exponential values |
| oct-encode | 5/5 | expected encoded layout; decoded filter validity covered separately |
| quat-encode | 5/5 | expected encoded layout; decoded filter validity covered separately |
| exp-encode | 5/5 | expected encoded layout; decoded filter validity covered separately |
| color-encode | 5/5 | expected encoded layout; decoded filter validity covered separately |
| cache | 5/5 | index-valid; oriented triangle multiset |
| cache-strip | 5/5 | index-valid; oriented triangle multiset |
| cache-fifo | 5/5 | index-valid; oriented triangle multiset |
| overdraw | 5/5 | index-valid; oriented triangle multiset |
| fetch | 5/5 | vertex bytes reconstruct original indexed stream |
| fetch-remap | 5/5 | referenced vertices mapped to compact contiguous ids |
| simplify | 5/5 | index/count-valid; finite reported error (no quality equivalence claim) |
| stripify | 5/5 | both unstripifiers; oriented triangle multiset |
| unstripify | 5/5 | index-valid; oriented triangle multiset |
| meshlets | Raw 1/5; canonical 5/5 | v1.0 padding removal; builder changes can also alter partition/order. descriptor bounds; local/global indices; oriented triangle multiset |
| meshlets-scan | Raw 1/5; canonical 5/5 | v1.0 padding removal; builder changes can also alter partition/order. descriptor bounds; local/global indices; oriented triangle multiset |
| meshlets-flex | Raw 1/5; canonical 5/5 | v1.0 padding removal; builder changes can also alter partition/order. descriptor bounds; local/global indices; oriented triangle multiset |
| meshlets-spatial | Raw 1/5; canonical 1/5 | v1.0 padding removal; builder changes can also alter partition/order. descriptor bounds; local/global indices; oriented triangle multiset |
| vertex-version | 5/5 | supported codec version |
| index-version | 5/5 | supported codec version |
| vertex-bound | 0/5 | v1.3 deliberately reduced conservative capacity bound. positive representable capacity |
| index-bound | 5/5 | positive representable capacity |
| sequence-bound | 5/5 | positive representable capacity |
| strip-bound | 5/5 | positive representable capacity |
| unstrip-bound | 5/5 | positive representable capacity |
| meshlet-bound | 5/5 | positive representable capacity |
| remap | 5/5 | referenced vertices mapped to compact contiguous ids |
| remap-multi | 5/5 | referenced vertices mapped to compact contiguous ids |
| remap-custom | 5/5 | referenced vertices mapped to compact contiguous ids |
| remap-vertices | 5/5 | exact referenced unique vertex records |
| remap-indices | 5/5 | valid compact indices; paired remap bytes checked separately |
| shadow | 5/5 | each rewritten index preserves vertex bytes |
| shadow-multi | 5/5 | each rewritten index preserves vertex bytes |
| position-remap | 5/5 | each representative has identical position bytes |
| adjacency | 5/5 | expected index count and bounds |
| tessellation | 5/5 | expected index count and bounds |
| provoking | 5/5 | reorder reconstructs oriented original triangles |
| simplify-attributes | 5/5 | index/count-valid; finite reported error (no quality equivalence claim) |
| simplify-sloppy | 5/5 | index/count-valid; finite reported error (no quality equivalence claim) |
| simplify-prune | 5/5 | valid subset of original oriented triangles |
| simplify-points | 5/5 | unique valid selected vertex ids |
| simplify-update | 5/5 | index/count-valid; finite mutated geometry/error |
| analyze-cache | 5/5 | defined statistics; exact comparison to peer, not performance acceptance |
| analyze-fetch | 5/5 | defined statistics; exact comparison to peer, not performance acceptance |
| analyze-overdraw | 5/5 | defined statistics; exact comparison to peer, not performance acceptance |
| analyze-coverage | 5/5 | finite numeric result |
| cluster-bounds | 5/5 | finite fields/nonnegative radius; sphere contains all points when applicable |
| sphere-bounds | 5/5 | finite fields/nonnegative radius; sphere contains all points when applicable |
| meshlet-bounds | 5/5 | finite fields/nonnegative radius; sphere contains all points when applicable |
| optimize-meshlet | 5/5 | local indices reconstruct original triangle |
| partition | 2/5 | v1.0 spatial merging and v1.2/v1.3 partition changes. one in-range partition id per input cluster |
| spatial-remap | 5/5 | vertex permutation |
| spatial-triangles | 5/5 | index-valid; oriented triangle multiset |
| spatial-points | 5/5 | vertex permutation |
| quantize-unorm | 5/5 | 12-bit unsigned normalized range |
| quantize-snorm | 5/5 | 12-bit signed normalized range |
| quantize-half | 5/5 | 16-bit range |
| dequantize-half | 5/5 | half bit pattern maps to expected finite/Inf/NaN category and sign |
| quantize-float | 5/5 | finite numeric result |
| simplify-scale | 5/5 | finite numeric result |

Both codecs cross-decode each encoder’s output. Index codecs may cyclically rotate a triangle, so validation compares oriented triangle multisets. Cache/overdraw/spatial triangle ordering, meshlets, strips and reconstructed provoking indices must preserve that multiset. Fetch must reconstruct the original indexed vertex byte stream. Simplification checks bounded valid indices and finite error/mutated geometry, not equal quality; prune must be a subset and point selection must be unique and at most the target. Sphere checks prove containment on the frozen points. SIMD filter validity checks unit vectors/quaternions and finite exponential results. The audit verifies that all 40 filter encoder outputs are exactly the frozen fixtures exercised by both decoders; layout alone does not establish error or image-quality parity. Analyzer floating statistics must be finite and nonnegative.

The independent audit also reconstructs indexed bytes from each paired vertex/index remap output. Cluster/meshlet bounding spheres contain the tested nondegenerate triangles; degenerate triangles are ignored by this upstream contract, and the sparse single-triangle helper correctly returns canonical zero bounds.

## Against pinned upstream C++ 1.3

The port covers equivalents for all **88 algorithm/helper/configuration names** in the 1.3 header; the remaining name, `meshopt_setAllocator`, is intentionally replaced. Additional demo `clusterlod.h` routines and Rust batch APIs are outside that 89-name C/C++ header count. Missing identical upstream API shapes: raw pointer/count layouts, C++ typed-index overloads, process-global allocator callbacks and process-global encoding-version setters. Checked slices/views, typed `Result` errors, per-call settings and Workspace/Limits are intentional replacements.

Valid-input parity targets the canonical scalar 1.3 output. SIMD filters deliberately keep scalar operation order rather than C++ approximation/FMA choices. On hostile input, finite/layout/overflow/quota checks can return typed errors where upstream asserts, assumes pointer validity or has undefined conversions. Some late failures can write a caller-buffer prefix; tails remain preserved. In-place paths and atomic staging guarantees are documented per API.

Known malformed meshlet tail: [UPSTREAM_NOTES.md](UPSTREAM_NOTES.md) freezes `seed-20261005-op20-9264`, corrupted final byte `0x1c`. Upstream scalar accepts and emits 24 bytes; upstream SSE rejects with -3. Rust scalar/native SIMD/executed Wasm accept and match the canonical scalar bytes. This intentional oracle choice is not universal scalar/SIMD conformance; the expanded strict sweep remains failed on this upstream discrepancy. No hostile pointers were passed to the older FFI crate in this comparison.

### Existing qualification ratios — not retimed

The following values come from the committed 0.2.0 qualification records and retain their original source/corpus/API epochs. Family GM bar is ≤1.25, case maximum bar ≤1.50, requested output-plus-scratch memory ≤1.25; P07 uses its documented A/A resolution and interval rules. This table does not turn old complete matrices or selected later diagnostics into acceptance of this entire integrated tree. P05 `overdraw`, `coverage`, `vertex_cache` and `vertex_fetch` below are **analyzers**, not the optimization APIs in Phase 0.1/0.1.x.

| Record / family | Moss GM (max) | Defaults GM (max) | Status / source |
|---|---|---|
| 0.1 / vertex_cache | 1.184 (1.330) | 1.231 (1.331) | [records](results/benchmark-moss.json); D146 preserves overdraw maximum INCONCLUSIVE |
| 0.1 / overdraw | 1.101 (1.522) | 1.135 (1.471) | [records](results/benchmark-moss.json); D146 preserves overdraw maximum INCONCLUSIVE |
| 0.1 / simplify | 1.049 (1.304) | 1.060 (1.237) | [records](results/benchmark-moss.json); D146 preserves overdraw maximum INCONCLUSIVE |
| 0.1 / simplify_with_attributes | 1.072 (1.231) | 1.092 (1.315) | [records](results/benchmark-moss.json); D146 preserves overdraw maximum INCONCLUSIVE |
| 0.1 / simplify_scale | 0.915 (1.067) | 0.855 (1.010) | [records](results/benchmark-moss.json); D146 preserves overdraw maximum INCONCLUSIVE |
| 0.1.x / vertex_cache_strip | 1.157 (1.241) | 1.157 (1.299) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / vertex_cache_fifo | 1.235 (1.466) | 1.208 (1.398) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_vertex_remap | 1.054 (1.195) | 1.022 (1.124) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_vertex_remap_multi | 0.961 (1.074) | 0.940 (1.023) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_vertex_remap_custom | 0.978 (1.179) | 1.087 (1.419) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / remap_vertex_buffer | 0.843 (1.125) | 0.865 (1.154) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / remap_index_buffer | 1.009 (1.170) | 1.143 (1.409) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / filter_index_buffer | 0.969 (1.091) | 0.993 (1.174) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / filter_index_buffer_multi | 0.918 (1.115) | 0.957 (1.094) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_shadow_index_buffer | 1.077 (1.224) | 1.004 (1.151) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_shadow_index_buffer_multi | 0.995 (1.146) | 0.915 (1.133) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_position_remap | 0.792 (0.940) | 0.797 (1.007) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_adjacency_index_buffer | 1.127 (1.238) | 1.178 (1.290) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_tessellation_index_buffer | 1.130 (1.246) | 1.195 (1.287) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / generate_provoking_index_buffer | 1.100 (1.215) | 1.182 (1.322) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / vertex_fetch | 0.974 (1.726) | 0.989 (1.852) | maximum gap; D146 fetch FAIL; [source](MEASURED_P01X.json) |
| 0.1.x / vertex_fetch_remap | 0.936 (1.174) | 1.024 (1.159) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_sloppy | 1.200 (1.481) | 1.184 (1.421) | D146 default maxima INCONCLUSIVE; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_prune | 1.056 (1.312) | 1.038 (1.387) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_points | 1.027 (1.274) | 0.995 (1.230) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_with_update | 1.025 (1.401) | 1.038 (1.419) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / quantize_unorm | 0.899 (0.946) | 0.896 (0.936) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / quantize_snorm | 1.125 (1.185) | 1.147 (1.184) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / quantize_half | 0.169 (0.198) | 0.185 (0.220) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / quantize_float | 0.717 (0.751) | 0.736 (0.829) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / dequantize_half | 0.150 (0.191) | 0.159 (0.286) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / compute_position_exponent | 0.965 (1.097) | 1.007 (1.114) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_sparse | 1.030 (1.340) | 1.037 (1.361) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_prune_option | 1.006 (1.272) | 1.017 (1.345) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_preserve_folds | 1.043 (1.329) | 1.048 (1.391) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_error_clamped | 0.994 (1.341) | 1.009 (1.393) | PASS record; [source](MEASURED_P01X.json) |
| 0.1.x / simplify_regularize_light | 1.015 (1.402) | 1.006 (1.416) | PASS record; [source](MEASURED_P01X.json) |
| 0.3 / build_meshlets | 1.126 (1.324) | 1.148 (1.405) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / build_meshlets_scan | 1.076 (1.153) | 1.139 (1.215) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / build_meshlets_flex | 1.107 (1.371) | 1.157 (1.423) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / build_meshlets_spatial | 1.077 (1.284) | 1.043 (1.242) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / build_meshlets_bound | 0.644 (0.657) | 0.647 (0.665) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / compute_cluster_bounds | 1.133 (1.290) | 1.084 (1.187) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / compute_meshlet_bounds | 1.168 (1.299) | 1.178 (1.328) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / compute_sphere_bounds | 1.006 (1.230) | 1.076 (1.360) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / optimize_meshlet | 0.953 (1.365) | 0.916 (1.209) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / optimize_meshlet_level | 0.897 (1.002) | 0.922 (1.051) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / extract_meshlet_indices | 0.977 (1.194) | 0.938 (1.334) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / partition_clusters | 1.229 (1.367) | 1.450 (1.778) | FAIL default partition mean/max; [source](MEASURED_P03.json) |
| 0.3 / spatial_sort_remap | 1.031 (1.427) | 0.990 (1.302) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / spatial_sort_triangles | 0.993 (1.316) | 0.896 (1.275) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.3 / spatial_cluster_points | 1.113 (1.212) | 1.159 (1.374) | PASS scalar record; [source](MEASURED_P03.json) |
| 0.4 / vertex_encode / allocating | 0.967 (1.255) | 0.981 (1.247) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / index_encode / allocating | 1.125 (1.200) | 1.138 (1.198) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / sequence_encode / allocating | 1.217 (1.391) | 1.234 (1.378) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / oct_encode / allocating | 1.056 (1.445) | 0.914 (1.347) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / quat_encode / allocating | 0.589 (1.457) | 0.622 (1.213) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / exp_encode / allocating | 1.009 (1.284) | 1.025 (1.231) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / color_encode / allocating | 0.772 (0.801) | 0.709 (0.822) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / color_decode / allocating | 0.757 (0.960) | 0.779 (1.203) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_encode / allocating | 1.117 (1.176) | 1.162 (1.172) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_decode / allocating | 1.075 (1.167) | 1.169 (1.499) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_decode_raw / allocating | 1.117 (1.157) | 1.230 (1.270) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / bounds / allocating | 0.665 (1.272) | 0.675 (1.292) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / vertex_encode / caller_buffer | 1.019 (1.288) | 1.027 (1.364) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / index_encode / caller_buffer | 1.099 (1.193) | 1.116 (1.284) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / sequence_encode / caller_buffer | 1.124 (1.387) | 1.164 (1.398) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / oct_encode / caller_buffer | 0.976 (1.248) | 0.964 (1.357) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / quat_encode / caller_buffer | 0.555 (1.105) | 0.618 (1.444) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / exp_encode / caller_buffer | 1.074 (1.276) | 1.082 (1.366) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / color_encode / caller_buffer | 0.649 (0.673) | 0.769 (0.798) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / color_decode / caller_buffer | 0.824 (1.174) | 0.756 (0.975) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_encode / caller_buffer | 1.085 (1.115) | 1.182 (1.270) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_decode / caller_buffer | 1.148 (1.351) | 1.085 (1.185) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / meshlet_decode_raw / caller_buffer | 1.101 (1.176) | 1.079 (1.180) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.4 / bounds / caller_buffer | 0.665 (1.272) | 0.675 (1.292) | Historical scalar record; decoders superseded by P07; [source](results/benchmark-0.4-moss.json) |
| 0.5 / stripify / allocating | 0.962 (1.221) | 0.989 (1.459) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / stripify / caller | 0.974 (1.183) | 0.967 (1.233) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / stripify_bound / allocating | 0.570 (0.573) | 0.556 (0.579) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / unstripify / allocating | 0.826 (1.054) | 0.861 (1.149) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / unstripify / caller | 0.713 (1.186) | 0.815 (1.287) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / unstripify_bound / allocating | 0.539 (0.553) | 0.496 (0.565) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / vertex_cache / allocating | 1.077 (1.234) | 1.085 (1.302) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / vertex_fetch / allocating | 0.959 (1.085) | 0.924 (0.997) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / overdraw / allocating | 0.886 (1.051) | 0.998 (1.230) | Latest last3 selected scope PASS; [state](P05_CURRENT_STATE.md) |
| 0.5 / coverage / allocating | 0.913 (1.102) | 0.994 (1.227) | Latest last3 selected scope PASS; [state](P05_CURRENT_STATE.md) |
| 0.5 / omm_measure / allocating | 1.172 (1.293) | 1.177 (1.347) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / omm_rasterize / allocating | 1.110 (1.310) | 1.095 (1.268) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / omm_rasterize / caller | 1.169 (1.312) | 1.115 (1.268) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / omm_entry_size / allocating | 0.470 (0.473) | 0.471 (0.476) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / omm_compact / allocating | 0.557 (0.584) | 0.578 (0.624) | Latest last3 selected scope PASS; [state](P05_CURRENT_STATE.md) |
| 0.5 / tangents / allocating | 1.135 (1.259) | 1.171 (1.343) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / tangents / caller | 1.145 (1.285) | 1.128 (1.252) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / normals / allocating | 1.194 (1.282) | 1.194 (1.310) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / normals / caller | 1.179 (1.260) | 1.213 (1.302) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / remesh / allocating | 1.125 (1.256) | 1.133 (1.238) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |
| 0.5 / remesh / caller | 1.101 (1.295) | 1.124 (1.321) | Older full-matrix source-bound record; [state](P05_CURRENT_STATE.md) |

P02 historical raw-decoder scalar GM/max: allocating 1.010/1.375, caller-buffer 0.973/1.266; those registered minima passed, but current SIMD/native decoder status is P07 below. [P02 record](P02_RESULTS.json). Parallel batch scaling is versus our serial implementation, not upstream C++; [P06](MEASURED_P06.json) preserves those thread-count curves. Cluster LOD/hierarchy has separate [RFC113 evidence](rfc113/README.md); no new upstream timing is claimed here.

Latest P07 native/wasm results and unresolved A/A rows are reproduced from [P07_RELEASE_QUALIFICATION.md](P07_RELEASE_QUALIFICATION.md):

| Platform / family | Allocating mean / resolved max / verdict | Caller mean / resolved max / verdict | Resolved A / C |
|---|---|---|---|
| native / color | 0.996 / 1.364 / PASS | 1.105 / 1.401 / PASS | 5/6 / 6/6 |
| native / exp | 0.896 / 1.465 / PASS | 0.895 / 1.270 / PASS | 10/12 / 8/12 |
| native / index | 1.187 / 3.625 / **FAIL** | 1.142 / 1.444 / PASS | 9/14 / 9/14 |
| native / meshlet | 1.132 / 1.431 / PASS | 1.240 / 1.483 / PASS | 9/12 / 5/12 |
| native / meshlet-raw | 1.171 / 1.353 / PASS | 1.233 / 1.490 / PASS | 3/3 / 2/3 |
| native / oct | 0.931 / 1.492 / PASS | 0.932 / 1.496 / PASS | 8/12 / 8/12 |
| native / quat | 0.886 / 1.460 / PASS | 0.868 / 1.467 / PASS | 3/6 / 4/6 |
| native / sequence | 1.131 / 1.457 / PASS | 1.080 / 1.477 / PASS | 9/12 / 10/12 |
| native / vertex | 1.116 / 2.118 / **FAIL** | 1.051 / 1.440 / PASS | 16/22 / 13/22 |
| native / view-filtered | 1.089 / 1.636 / **FAIL** | 1.028 / 1.436 / PASS | 19/30 / 23/30 |
| native / view-none | 1.040 / 1.266 / PASS | 1.033 / 1.447 / PASS | 6/9 / 5/9 |
| wasm / index | 0.877 / 1.422 / PASS | 1.042 / 1.337 / PASS | 3/14 / 11/14 |
| wasm / sequence | 0.864 / 1.049 / PASS | 0.815 / 0.956 / PASS | 5/12 / 8/12 |
| wasm / vertex | 0.913 / 1.481 / PASS | 0.858 / 1.260 / PASS | 11/22 / 19/22 |
| wasm / view-filtered | 0.889 / 1.471 / PASS | 0.945 / 1.466 / PASS | 9/30 / 26/30 |
| wasm / view-none | 0.954 / 1.311 / PASS | 0.838 / 0.963 / PASS | 6/9 / 6/9 |

Cluster LOD recovery retains scalar sum-of-medians aggregate 1.196 Moss / 1.090 defaults, with the distinct RFC113 aggregate bar 1.2 and per-mesh bar 1.5: [recovery record](results/rfc113-clod-recover.md). The maximum stage-one per-mesh median is 1.269 / 1.315. These aggregates are not family geometric means. The later P06 small-hierarchy fallback retains serial/batch medians 0.878/0.836/0.775/0.545/0.725 at 1/2/4/8/16 threads under its recorded shared load; it does not claim a universal parallel gain (D158 in DECISIONS.md).

Families still above the requested 0.3 target bars, or not demonstrated closed:

- **Cluster partitioning, Cargo defaults:** GM 1.450, maximum 1.778; Moss GM 1.229/maximum 1.367 passes that record.
- **Native allocating index, vertex and filtered-view decode:** P07 resolved maximum upper bounds 3.625, 2.118 and 1.636 respectively. Family means pass, caller-buffer rows pass under the registered resolution rule. Allocating varied filtered-view stride-32 streaming remains unresolvable, interval 1.619–2.590.
- **Older 0.1.x allocating fetch maximum:** sparse-million medians 1.726 Moss / 1.852 defaults; both D146 streams FAIL. **Sloppy simplify, defaults** and **overdraw optimization, Moss** retain D146 INCONCLUSIVE maxima. These older records are not cleared by unrelated selected P05 analyzer fixes.
- **Whole P05 integrated performance acceptance:** latest compact/coverage/raster-overdraw selected scope passes; the full all-family matrix was not refreshed after that candidate. The prior failing compact/raster rows are historical, not current proven failures.

### Where the Rust port provides different capabilities

Initialized checked slices and typed errors provide a memory-safe public boundary with checked layout/index/finite/quota handling; SIMD internals still require the documented unsafe audit. No C/C++ compiler or raw-pointer interface is needed by consumers. `no_std + alloc`, per-call versions/limits, retained caller workspaces, and built-in deterministic parallel batches are available. Per-item typed errors do not cover Rayon global-pool initialization failure, which can panic; small hierarchy fallback avoids that pool. Those are capability differences, not evidence of a universal speed or safety advantage over every correctly used C++ path.

### Priorities for 0.3

1. **Encoder performance first:** close the Oct/Quat/Exp encoding gaps against meshopt, starting with Exp at about 3.4×. Keep matched versions and precision settings explicit; default-version comparisons measure different encoding choices.
2. Close default partitioning and the resolved native allocating codec maximum gaps; retain the unresolvable view case and canonical malformed-tail disagreement as explicit blockers.
3. Requalify the existing fetch/sloppy/optimization-overdraw maximum gaps against the integrated source, then refresh the full P05 matrix; keep compiler/API/corpus epochs distinct.
4. Extend execution and performance evidence to AArch64 and other native operating systems; compile-only support is not native runtime qualification.
5. Broaden shared-input coverage beyond this comparison subset, especially quantization extrema, attributed/permissive simplification, partition sizes, filter precision and application wrapper/allocation shapes.
6. Keep global allocator callbacks, raw C ABI and process-global codec versions as intentional API omissions; add adapters only if a concrete interoperability need warrants them.

## Choice guide for users

- meshopt had lower five-case geometric mean times for allocating vertex decode and Oct/Quat/Exp encoding in both profiles; compare individual cases for your workload and provide a C/C++ toolchain.
- Choose meshoptimizer-rs for 1.3 families absent from the older wrapper, including meshlet codecs, opacity maps, tangents, normals and remeshing.
- The Rust port offers no_std + alloc, work/byte limits, caller workspaces and built-in batch scheduling; the older crate has a global C++ allocator hook.
- Ours had lower five-case geometric mean times for Color encoding and scan meshlets in both profiles; that does not establish a gain for every input or API shape.
- Default encoded vertex bytes differ: ours defaults to v1, meshopt’s bundled 0.25 to v0. Select a version explicitly for compatibility.
- Meshlet layout, partitioning and simplification can differ because upstream changed between 0.25 and 1.3; valid output does not imply equal partitions or quality.
- The older crate has a smaller retained full-surface binary in this experiment; ours’ broader API, parallel runtime and checks have costs.
- Neither the shared-host timings nor these build checks establish all-platform release acceptance; use the retained maximum gaps when selecting an allocation/API path.

## Evidence and reproduction

All raw evidence is in `/mnt/linux-extra/moss-scratch/meshopt-cmp`, including MILESTONES.md, source/scout inventories, vendor hashes, input/output hashes, retained executables, build/target logs, host metadata, samples and admission receipts. Reproduce with [compare/README.md](compare/README.md). The specified comparison Cargo target is disposable and is deleted after final audit. No library changes, subagents, push, or upstream C++ re-timing.

## Appendix: per-case ratios and intervals

| Frozen case / operation | Defaults median [nominal log-mean CI] | Moss median [nominal log-mean CI] |
|---|---|---|
| tiny-smooth / vertex-decode | 1.747 [1.474, 2.488] (n=5) | 1.661 [1.325, 2.065] (n=5) |
| tiny-smooth / vertex-encode | 0.842 [0.830, 0.893] (n=20) | 0.847 [0.754, 0.886] (n=20) |
| tiny-smooth / vertex-encode-v0 | 1.187 [1.094, 1.322] (n=5) | 1.457 [1.183, 1.635] (n=5) |
| tiny-smooth / vertex-encode-default | 2.230 [2.163, 2.335] (n=5) | 2.106 [1.754, 3.090] (n=5) |
| tiny-smooth / index-decode | 1.221 [1.142, 1.288] (n=5) | 1.104 [0.949, 1.184] (n=20) |
| tiny-smooth / index-encode | 0.974 [0.955, 0.998] (n=20) | 0.996 [0.932, 1.053] (n=20) |
| tiny-smooth / sequence-decode | 1.173 [1.122, 1.238] (n=5) | 1.367 [1.170, 1.686] (n=5) |
| tiny-smooth / sequence-encode | 1.368 [1.172, 1.466] (n=5) | 1.249 [1.189, 1.439] (n=20) |
| tiny-smooth / oct-decode | 0.932 [0.890, 0.972] (n=20) | 0.956 [0.904, 0.994] (n=20) |
| tiny-smooth / quat-decode | 1.138 [1.067, 1.192] (n=5) | 1.096 [1.042, 1.155] (n=20) |
| tiny-smooth / exp-decode | 1.034 [0.983, 1.071] (n=20) | 1.096 [1.038, 1.192] (n=5) |
| tiny-smooth / color-decode | 0.989 [0.929, 1.032] (n=20) | 0.911 [0.859, 0.978] (n=5) |
| tiny-smooth / oct-encode | 1.242 [1.058, 1.353] (n=5) | 1.176 [1.111, 1.266] (n=5) |
| tiny-smooth / quat-encode | 1.760 [1.524, 2.015] (n=5) | 1.718 [1.342, 1.969] (n=5) |
| tiny-smooth / exp-encode | 3.062 [2.479, 3.256] (n=5) | 3.015 [2.808, 3.169] (n=5) |
| tiny-smooth / color-encode | 0.878 [0.835, 0.960] (n=5) | 0.844 [0.805, 0.955] (n=20) |
| tiny-smooth / cache | 1.206 [1.118, 1.281] (n=5) | 1.137 [1.049, 1.206] (n=5) |
| tiny-smooth / cache-strip | 1.212 [1.042, 1.371] (n=5) | 1.183 [1.150, 1.203] (n=5) |
| tiny-smooth / cache-fifo | 1.301 [1.187, 1.397] (n=5) | 1.283 [1.200, 1.429] (n=5) |
| tiny-smooth / overdraw | 0.974 [0.951, 1.060] (n=20) | 1.090 [1.032, 1.183] (n=5) |
| tiny-smooth / fetch | 0.948 [0.887, 0.986] (n=20) | 0.921 [0.884, 0.962] (n=5) |
| tiny-smooth / fetch-remap | 1.282 [1.142, 1.487] (n=5) | 1.324 [1.209, 1.414] (n=5) |
| tiny-smooth / simplify | 1.040 [1.015, 1.085] (n=20) | 1.019 [0.980, 1.057] (n=20) |
| tiny-smooth / stripify | 0.809 [0.764, 0.909] (n=5) | 0.874 [0.833, 0.913] (n=20) |
| tiny-smooth / unstripify | 0.952 [0.895, 1.042] (n=20) | 1.046 [1.006, 1.127] (n=20) |
| tiny-smooth / meshlets | 1.607 [1.330, 1.775] (n=5) | 1.420 [1.283, 1.512] (n=5) |
| tiny-smooth / meshlets-scan | 1.056 [1.032, 1.091] (n=20) | 0.983 [0.909, 1.015] (n=20) |
| tiny-smooth / meshlets-flex | 1.454 [1.261, 1.612] (n=5) | 1.376 [1.252, 1.427] (n=20) |
| tiny-smooth / meshlets-spatial | 1.074 [1.021, 1.172] (n=5) | 1.147 [1.120, 1.155] (n=5) |
| medium-smooth / vertex-decode | 1.449 [1.379, 1.569] (n=5) | 1.453 [1.373, 1.525] (n=5) |
| medium-smooth / vertex-encode | 0.896 [0.869, 0.911] (n=5) | 0.902 [0.768, 0.987] (n=5) |
| medium-smooth / vertex-encode-v0 | 1.216 [1.184, 1.258] (n=5) | 1.270 [1.241, 1.312] (n=5) |
| medium-smooth / vertex-encode-default | 1.472 [1.439, 1.508] (n=5) | 1.470 [1.450, 1.518] (n=5) |
| medium-smooth / index-decode | 0.845 [0.816, 0.885] (n=5) | 0.913 [0.900, 0.945] (n=20) |
| medium-smooth / index-encode | 1.212 [1.142, 1.272] (n=5) | 1.252 [1.176, 1.352] (n=5) |
| medium-smooth / sequence-decode | 0.870 [0.802, 0.967] (n=5) | 0.869 [0.856, 0.889] (n=5) |
| medium-smooth / sequence-encode | 1.376 [1.338, 1.448] (n=5) | 1.397 [1.311, 1.484] (n=5) |
| medium-smooth / oct-decode | 1.140 [1.093, 1.173] (n=5) | 1.072 [1.016, 1.143] (n=5) |
| medium-smooth / quat-decode | 1.208 [1.190, 1.228] (n=5) | 1.191 [1.159, 1.229] (n=5) |
| medium-smooth / exp-decode | 0.919 [0.891, 0.940] (n=5) | 0.912 [0.858, 0.940] (n=5) |
| medium-smooth / color-decode | 0.893 [0.850, 0.941] (n=5) | 0.907 [0.851, 0.940] (n=20) |
| medium-smooth / oct-encode | 1.277 [1.241, 1.341] (n=5) | 1.353 [1.225, 1.547] (n=5) |
| medium-smooth / quat-encode | 1.906 [1.781, 2.017] (n=5) | 2.041 [1.882, 2.291] (n=5) |
| medium-smooth / exp-encode | 3.529 [3.360, 3.836] (n=5) | 3.504 [3.248, 3.609] (n=5) |
| medium-smooth / color-encode | 0.870 [0.777, 0.941] (n=5) | 0.815 [0.787, 0.864] (n=5) |
| medium-smooth / cache | 1.399 [1.299, 1.437] (n=5) | 1.205 [1.073, 1.382] (n=5) |
| medium-smooth / cache-strip | 1.347 [1.275, 1.525] (n=5) | 1.324 [1.279, 1.396] (n=5) |
| medium-smooth / cache-fifo | 1.338 [1.264, 1.442] (n=5) | 1.475 [1.365, 1.708] (n=5) |
| medium-smooth / overdraw | 1.228 [1.169, 1.305] (n=5) | 1.613 [1.554, 1.802] (n=5) |
| medium-smooth / fetch | 1.252 [1.214, 1.343] (n=5) | 1.206 [1.129, 1.369] (n=5) |
| medium-smooth / fetch-remap | 1.421 [1.259, 1.591] (n=5) | 1.251 [1.169, 1.343] (n=5) |
| medium-smooth / simplify | 0.841 [0.805, 0.882] (n=20) | 0.771 [0.717, 0.812] (n=5) |
| medium-smooth / stripify | 1.078 [1.023, 1.147] (n=5) | 1.018 [1.006, 1.045] (n=20) |
| medium-smooth / unstripify | 1.009 [0.968, 1.090] (n=20) | 0.945 [0.926, 0.958] (n=5) |
| medium-smooth / meshlets | 1.019 [0.995, 1.036] (n=20) | 1.013 [0.964, 1.042] (n=20) |
| medium-smooth / meshlets-scan | 0.877 [0.832, 0.924] (n=5) | 0.673 [0.589, 0.718] (n=5) |
| medium-smooth / meshlets-flex | 1.034 [1.006, 1.079] (n=20) | 1.021 [1.006, 1.045] (n=20) |
| medium-smooth / meshlets-spatial | 1.110 [1.063, 1.147] (n=20) | 1.092 [1.054, 1.125] (n=20) |
| medium-seams / vertex-decode | 1.292 [1.024, 1.838] (n=5) | 1.233 [1.195, 1.270] (n=5) |
| medium-seams / vertex-encode | 0.934 [0.925, 0.975] (n=20) | 0.934 [0.905, 0.952] (n=5) |
| medium-seams / vertex-encode-v0 | 1.282 [1.097, 1.512] (n=5) | 1.179 [1.115, 1.305] (n=5) |
| medium-seams / vertex-encode-default | 1.403 [1.237, 1.567] (n=5) | 1.490 [1.445, 1.529] (n=5) |
| medium-seams / index-decode | 0.935 [0.908, 0.985] (n=20) | 1.012 [0.991, 1.060] (n=20) |
| medium-seams / index-encode | 1.399 [1.226, 1.502] (n=5) | 1.281 [1.206, 1.319] (n=5) |
| medium-seams / sequence-decode | 1.011 [0.937, 1.050] (n=20) | 1.036 [1.006, 1.078] (n=20) |
| medium-seams / sequence-encode | 1.258 [1.199, 1.344] (n=20) | 1.283 [1.236, 1.349] (n=5) |
| medium-seams / oct-decode | 1.132 [1.097, 1.147] (n=5) | 1.116 [1.021, 1.148] (n=20) |
| medium-seams / quat-decode | 1.193 [1.176, 1.225] (n=5) | 1.166 [1.131, 1.224] (n=5) |
| medium-seams / exp-decode | 0.910 [0.875, 0.987] (n=5) | 0.936 [0.923, 0.944] (n=5) |
| medium-seams / color-decode | 0.931 [0.875, 0.979] (n=20) | 0.930 [0.914, 0.968] (n=20) |
| medium-seams / oct-encode | 1.283 [1.117, 1.409] (n=5) | 1.380 [1.337, 1.459] (n=5) |
| medium-seams / quat-encode | 1.953 [1.858, 2.003] (n=5) | 1.997 [1.943, 2.120] (n=5) |
| medium-seams / exp-encode | 3.577 [2.981, 4.365] (n=5) | 3.565 [3.379, 3.703] (n=5) |
| medium-seams / color-encode | 0.805 [0.759, 0.848] (n=5) | 0.816 [0.800, 0.829] (n=5) |
| medium-seams / cache | 1.300 [1.063, 1.538] (n=5) | 1.206 [1.065, 1.311] (n=5) |
| medium-seams / cache-strip | 1.281 [1.196, 1.419] (n=5) | 1.205 [1.098, 1.241] (n=20) |
| medium-seams / cache-fifo | 1.467 [1.331, 1.573] (n=5) | 1.610 [1.313, 1.868] (n=5) |
| medium-seams / overdraw | 1.228 [1.174, 1.331] (n=5) | 1.737 [1.547, 2.195] (n=5) |
| medium-seams / fetch | 1.139 [1.083, 1.179] (n=20) | 1.103 [1.063, 1.178] (n=20) |
| medium-seams / fetch-remap | 1.331 [1.009, 1.661] (n=5) | 1.260 [1.168, 1.405] (n=5) |
| medium-seams / simplify | 0.920 [0.760, 0.963] (n=20) | 0.859 [0.817, 0.956] (n=5) |
| medium-seams / stripify | 0.990 [0.967, 1.023] (n=20) | 0.990 [0.951, 1.013] (n=20) |
| medium-seams / unstripify | 1.028 [0.984, 1.072] (n=20) | 0.914 [0.883, 1.012] (n=20) |
| medium-seams / meshlets | 1.024 [0.942, 1.085] (n=20) | 0.979 [0.957, 1.001] (n=20) |
| medium-seams / meshlets-scan | 0.778 [0.692, 0.855] (n=5) | 0.673 [0.641, 0.739] (n=5) |
| medium-seams / meshlets-flex | 1.007 [0.975, 1.050] (n=20) | 0.994 [0.946, 1.030] (n=20) |
| medium-seams / meshlets-spatial | 1.079 [1.013, 1.155] (n=5) | 1.056 [1.002, 1.150] (n=20) |
| medium-sparse / vertex-decode | 1.272 [1.162, 1.340] (n=5) | 1.225 [1.135, 1.406] (n=5) |
| medium-sparse / vertex-encode | 1.017 [0.972, 1.058] (n=20) | 0.945 [0.900, 1.091] (n=20) |
| medium-sparse / vertex-encode-v0 | 1.420 [1.348, 1.469] (n=5) | 1.415 [1.363, 1.463] (n=5) |
| medium-sparse / vertex-encode-default | 1.592 [1.414, 1.951] (n=5) | 1.688 [1.581, 1.724] (n=5) |
| medium-sparse / index-decode | 0.916 [0.905, 0.969] (n=20) | 1.013 [0.980, 1.038] (n=20) |
| medium-sparse / index-encode | 1.241 [1.185, 1.343] (n=5) | 1.247 [1.166, 1.402] (n=5) |
| medium-sparse / sequence-decode | 0.893 [0.871, 0.962] (n=20) | 0.926 [0.818, 0.995] (n=5) |
| medium-sparse / sequence-encode | 1.352 [1.256, 1.421] (n=5) | 1.390 [1.359, 1.435] (n=5) |
| medium-sparse / oct-decode | 1.120 [1.076, 1.151] (n=20) | 1.111 [1.070, 1.200] (n=5) |
| medium-sparse / quat-decode | 1.184 [1.137, 1.232] (n=5) | 1.220 [1.160, 1.254] (n=5) |
| medium-sparse / exp-decode | 0.910 [0.896, 0.931] (n=5) | 0.907 [0.894, 0.918] (n=5) |
| medium-sparse / color-decode | 0.939 [0.927, 0.969] (n=20) | 0.920 [0.895, 0.955] (n=20) |
| medium-sparse / oct-encode | 1.267 [1.068, 1.375] (n=5) | 1.301 [1.182, 1.407] (n=5) |
| medium-sparse / quat-encode | 2.085 [1.927, 2.149] (n=5) | 1.959 [1.934, 1.986] (n=5) |
| medium-sparse / exp-encode | 3.654 [3.123, 3.944] (n=5) | 3.662 [3.470, 3.742] (n=5) |
| medium-sparse / color-encode | 0.896 [0.798, 0.969] (n=5) | 0.822 [0.800, 0.862] (n=5) |
| medium-sparse / cache | 1.392 [1.274, 1.491] (n=5) | 1.181 [1.144, 1.227] (n=5) |
| medium-sparse / cache-strip | 1.190 [1.081, 1.558] (n=5) | 1.182 [1.079, 1.338] (n=5) |
| medium-sparse / cache-fifo | 1.280 [1.144, 1.419] (n=5) | 1.397 [1.357, 1.432] (n=5) |
| medium-sparse / overdraw | 1.117 [1.040, 1.274] (n=5) | 1.699 [1.551, 1.952] (n=5) |
| medium-sparse / fetch | 1.237 [1.123, 1.428] (n=5) | 1.147 [1.087, 1.168] (n=20) |
| medium-sparse / fetch-remap | 1.322 [1.291, 1.384] (n=5) | 1.266 [1.229, 1.334] (n=5) |
| medium-sparse / simplify | 1.171 [1.068, 1.217] (n=20) | 1.083 [1.074, 1.090] (n=5) |
| medium-sparse / stripify | 1.188 [1.041, 1.314] (n=5) | 0.990 [0.979, 1.001] (n=20) |
| medium-sparse / unstripify | 1.023 [1.004, 1.081] (n=20) | 0.941 [0.892, 0.971] (n=5) |
| medium-sparse / meshlets | 1.031 [1.004, 1.078] (n=20) | 0.991 [0.955, 1.031] (n=20) |
| medium-sparse / meshlets-scan | 0.804 [0.729, 0.919] (n=5) | 0.692 [0.640, 0.749] (n=5) |
| medium-sparse / meshlets-flex | 1.007 [0.980, 1.052] (n=20) | 1.014 [0.991, 1.052] (n=20) |
| medium-sparse / meshlets-spatial | 1.125 [1.071, 1.174] (n=5) | 1.223 [1.089, 1.290] (n=5) |
| million-smooth / vertex-decode | 1.154 [1.118, 1.244] (n=20) | 0.898 [0.850, 0.951] (n=5) |
| million-smooth / vertex-encode | 1.010 [0.965, 1.070] (n=20) | 0.991 [0.905, 1.025] (n=20) |
| million-smooth / vertex-encode-v0 | 1.243 [1.160, 1.332] (n=5) | 1.326 [1.096, 1.509] (n=5) |
| million-smooth / vertex-encode-default | 1.604 [1.497, 1.700] (n=5) | 1.664 [1.519, 1.802] (n=5) |
| million-smooth / index-decode | 0.837 [0.744, 0.955] (n=5) | 0.905 [0.879, 0.995] (n=20) |
| million-smooth / index-encode | 1.249 [1.069, 1.335] (n=5) | 1.235 [1.002, 1.577] (n=5) |
| million-smooth / sequence-decode | 0.841 [0.808, 0.892] (n=5) | 0.926 [0.887, 0.964] (n=20) |
| million-smooth / sequence-encode | 1.504 [1.330, 1.674] (n=5) | 1.417 [1.159, 1.629] (n=5) |
| million-smooth / oct-decode | 1.087 [1.027, 1.195] (n=5) | 1.135 [1.039, 1.167] (n=20) |
| million-smooth / quat-decode | 1.172 [1.113, 1.234] (n=20) | 1.232 [1.157, 1.273] (n=5) |
| million-smooth / exp-decode | 0.933 [0.890, 1.106] (n=20) | 0.953 [0.897, 1.022] (n=20) |
| million-smooth / color-decode | 0.947 [0.895, 0.990] (n=20) | 0.923 [0.891, 0.956] (n=20) |
| million-smooth / oct-encode | 2.041 [1.744, 2.250] (n=5) | 2.580 [2.426, 3.025] (n=5) |
| million-smooth / quat-encode | 1.907 [1.775, 1.965] (n=5) | 1.899 [1.719, 2.054] (n=5) |
| million-smooth / exp-encode | 3.230 [3.040, 3.467] (n=5) | 3.238 [2.874, 3.837] (n=5) |
| million-smooth / color-encode | 0.876 [0.809, 0.965] (n=5) | 0.901 [0.830, 0.969] (n=20) |
| million-smooth / cache | 0.955 [0.933, 0.986] (n=20) | 0.922 [0.787, 0.993] (n=5) |
| million-smooth / cache-strip | 0.852 [0.777, 0.939] (n=5) | 0.692 [0.592, 0.862] (n=5) |
| million-smooth / cache-fifo | 0.729 [0.682, 0.784] (n=5) | 0.762 [0.722, 0.838] (n=5) |
| million-smooth / overdraw | 1.537 [1.164, 1.686] (n=5) | 1.791 [1.247, 2.062] (n=5) |
| million-smooth / fetch | 1.132 [1.004, 1.197] (n=5) | 1.158 [1.044, 1.261] (n=5) |
| million-smooth / fetch-remap | 1.150 [1.037, 1.276] (n=5) | 1.094 [1.049, 1.149] (n=5) |
| million-smooth / simplify | 0.985 [0.832, 1.097] (n=20) | 1.007 [0.952, 1.042] (n=20) |
| million-smooth / stripify | 1.140 [1.064, 1.213] (n=20) | 0.987 [0.952, 1.033] (n=20) |
| million-smooth / unstripify | 0.950 [0.867, 1.016] (n=20) | 0.920 [0.880, 0.965] (n=20) |
| million-smooth / meshlets | 1.046 [1.022, 1.090] (n=20) | 1.030 [0.987, 1.077] (n=20) |
| million-smooth / meshlets-scan | 0.805 [0.682, 0.853] (n=20) | 0.616 [0.440, 0.838] (n=5) |
| million-smooth / meshlets-flex | 1.032 [0.953, 1.062] (n=20) | 1.008 [0.920, 1.124] (n=20) |
| million-smooth / meshlets-spatial | 1.015 [0.903, 1.052] (n=20) | 1.026 [0.968, 1.061] (n=20) |

## Appendix: every upstream 1.3 public header name

A = available; D = different API shape; M = missing. Ours generally exposes checked slices, a Result and caller/workspace variants instead of raw pointers. Theirs FFI-only entry points are callable but require unsafe Rust. Inline quantizers are Rust helpers in both crates.

| Upstream name | Ours | Theirs |
|---|---|---|
| `meshopt_generateVertexRemap` | A; checked API | A; wrapper + FFI |
| `meshopt_generateVertexRemapMulti` | A; checked API | A; wrapper + FFI |
| `meshopt_generateVertexRemapCustom` | A; checked API | D; FFI only |
| `meshopt_remapVertexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_remapIndexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_filterIndexBuffer` | A; checked API | M |
| `meshopt_filterIndexBufferMulti` | A; checked API | M |
| `meshopt_generateShadowIndexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_generateShadowIndexBufferMulti` | A; checked API | A; wrapper + FFI |
| `meshopt_generatePositionRemap` | A; checked API | A; wrapper + FFI |
| `meshopt_generateAdjacencyIndexBuffer` | A; checked API | D; FFI only |
| `meshopt_generateTessellationIndexBuffer` | A; checked API | D; FFI only |
| `meshopt_generateProvokingIndexBuffer` | A; checked API | D; FFI only |
| `meshopt_optimizeVertexCache` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeVertexCacheStrip` | A; checked API | D; FFI only |
| `meshopt_optimizeVertexCacheFifo` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeOverdraw` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeVertexFetch` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeVertexFetchRemap` | A; checked API | A; wrapper + FFI |
| `meshopt_encodeIndexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_encodeIndexBufferBound` | A; checked API | A; wrapper + FFI |
| `meshopt_encodeIndexVersion` | D; per-call encoding configuration | D; FFI only |
| `meshopt_decodeIndexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_decodeIndexVersion` | A; checked API | D; FFI only |
| `meshopt_encodeIndexSequence` | A; checked API | D; FFI only |
| `meshopt_encodeIndexSequenceBound` | A; checked API | D; FFI only |
| `meshopt_decodeIndexSequence` | A; checked API | D; FFI only |
| `meshopt_encodeMeshlet` | A; checked API | M |
| `meshopt_encodeMeshletBound` | A; checked API | M |
| `meshopt_decodeMeshlet` | A; checked API | M |
| `meshopt_decodeMeshletRaw` | A; checked API | M |
| `meshopt_encodeVertexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_encodeVertexBufferBound` | A; checked API | A; wrapper + FFI |
| `meshopt_encodeVertexBufferLevel` | A; checked API | D; FFI only |
| `meshopt_encodeVertexVersion` | D; per-call encoding configuration | D; FFI only |
| `meshopt_decodeVertexBuffer` | A; checked API | A; wrapper + FFI |
| `meshopt_decodeVertexVersion` | A; checked API | D; FFI only |
| `meshopt_decodeFilterOct` | A; checked API | D; FFI only |
| `meshopt_decodeFilterQuat` | A; checked API | D; FFI only |
| `meshopt_decodeFilterExp` | A; checked API | D; FFI only |
| `meshopt_decodeFilterColor` | A; checked API | D; FFI only |
| `meshopt_encodeFilterOct` | A; checked API | D; FFI only |
| `meshopt_encodeFilterQuat` | A; checked API | D; FFI only |
| `meshopt_encodeFilterExp` | A; checked API | D; FFI only |
| `meshopt_encodeFilterColor` | A; checked API | D; FFI only |
| `meshopt_simplify` | A; checked API | A; wrapper + FFI |
| `meshopt_simplifyWithAttributes` | A; checked API | A; wrapper + FFI |
| `meshopt_simplifyWithUpdate` | A; checked API | D; FFI only |
| `meshopt_simplifySloppy` | A; checked API | A; wrapper + FFI |
| `meshopt_simplifyPrune` | A; checked API | D; FFI only |
| `meshopt_simplifyPoints` | A; checked API | D; FFI only |
| `meshopt_simplifyScale` | A; checked API | A; wrapper + FFI |
| `meshopt_stripify` | A; checked API | A; wrapper + FFI |
| `meshopt_stripifyBound` | A; checked API | D; FFI only |
| `meshopt_unstripify` | A; checked API | A; wrapper + FFI |
| `meshopt_unstripifyBound` | A; checked API | D; FFI only |
| `meshopt_analyzeVertexCache` | A; checked API | A; wrapper + FFI |
| `meshopt_analyzeVertexFetch` | A; checked API | A; wrapper + FFI |
| `meshopt_analyzeOverdraw` | A; checked API | A; wrapper + FFI |
| `meshopt_analyzeCoverage` | A; checked API | D; FFI only |
| `meshopt_buildMeshlets` | A; checked API | A; wrapper + FFI |
| `meshopt_buildMeshletsScan` | A; checked API | D; FFI only |
| `meshopt_buildMeshletsBound` | A; checked API | A; wrapper + FFI |
| `meshopt_buildMeshletsFlex` | A; checked API | A; wrapper + FFI |
| `meshopt_buildMeshletsSpatial` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeMeshlet` | A; checked API | A; wrapper + FFI |
| `meshopt_optimizeMeshletLevel` | A; checked API | M |
| `meshopt_computeClusterBounds` | A; checked API | A; wrapper + FFI |
| `meshopt_computeMeshletBounds` | A; checked API | A; wrapper + FFI |
| `meshopt_computeSphereBounds` | A; checked API | A; wrapper + FFI |
| `meshopt_extractMeshletIndices` | A; checked API | M |
| `meshopt_partitionClusters` | A; checked API | A; wrapper + FFI |
| `meshopt_spatialSortRemap` | A; checked API | D; FFI only |
| `meshopt_spatialSortTriangles` | A; checked API | D; FFI only |
| `meshopt_spatialClusterPoints` | A; checked API | D; FFI only |
| `meshopt_opacityMapMeasure` | A; checked API | M |
| `meshopt_opacityMapRasterize` | A; checked API | M |
| `meshopt_opacityMapEntrySize` | A; checked API | M |
| `meshopt_opacityMapCompact` | A; checked API | M |
| `meshopt_generateTangents` | A; checked API | M |
| `meshopt_generateNormals` | A; checked API | M |
| `meshopt_remesh` | A; checked API | M |
| `meshopt_quantizeHalf` | A; checked API | D; FFI only |
| `meshopt_quantizeFloat` | A; checked API | D; FFI only |
| `meshopt_dequantizeHalf` | A; checked API | D; FFI only |
| `meshopt_computePositionExponent` | A; checked API | M |
| `meshopt_setAllocator` | D; Workspace/Limits, no global callback | D; FFI only |
| `meshopt_quantizeUnorm` | A; checked API | D; Rust inline-equivalent helper |
| `meshopt_quantizeSnorm` | A; checked API | D; Rust inline-equivalent helper |
