# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-10-07

Performance-only release. There are no API changes. Encoded output is
byte-identical to 0.2.0 and to pinned meshoptimizer 1.3.

### Changed

- Faster encoders. Common vertex, Oct and Exp encoder layouts are specialized.
  Vertex group measurements are compacted, exact quaternion swizzles are
  batched, and sequence encoding uses constant varint offsets and packed
  directed edge comparisons. Calls on 32 or fewer Oct or Quat records use
  specialized checked paths with less fixed setup cost.
- Large owned stride-four vertex decodes are staged in initialized blocks.
  Resource limits and malformed-tail checks are unchanged.

### Performance

Geometric mean of Rust time / C++ meshoptimizer 1.3 time per encoder family,
allocating API. Lower is faster; 1.00 is equal.

| Encoder | Before | After |
|---|---:|---:|
| vertex | 0.904 | 0.658 |
| index | 1.027 | 0.920 |
| sequence | 1.228 | 0.967 |
| oct | 1.107 | 0.622¹ |
| quat | 0.787 | 0.789¹ |
| exp | 0.949 | 0.607 |

- Hardware and profile: native AMD Ryzen 9 7945HX, pinned to CPU 26,
  performance governor, boost enabled. Rust release build with fat LTO, one
  codegen unit and a generic CPU target. C++ 1.3 built with GCC 13.3.0 at -O3,
  with its normal runtime SIMD dispatch. Before and after use the same frozen
  inputs and the same benchmark consumer on a shared host.
- ¹ Oct and Quat show the released code, measured in a separate confirmation
  stream after the change for tiny Oct and Quat calls (the first round measured
  0.539 and 0.673 before that change).
  In that stream the Quat mean moved from 0.747 to 0.789: a measured slowdown
  of the family mean, still inside the 1.25 bar.
- The tiny Oct and Quat maxima are now certified at or below 1.5× C++ time.
  The largest 95% upper bound over the four tiny rows is 1.461.
- Output is byte-identical: 17,264 exact parity comparisons against scalar and
  SIMD meshoptimizer 1.3 cover vertex levels 0–9, both stream versions, both
  APIs, varied layouts and filter precisions.
- The 0.2.0 known exception `vertex-v1-streaming-s4` with the allocating API
  now measures 0.958× C++ time (95% CI 0.797–1.140) in this profile. The
  matched baseline without the staging change also passed in this measurement,
  at 0.952×. This does not show that the historical 1.529–2.118× gap is fixed.
  That gap was recorded in other consumer profiles, which were not rerun.
- Cargo-default and thin-LTO consumer profiles were not rerun for 0.2.1, and
  neither was the 0.2.0 comparison with the meshopt crate.

Full rows, caller-buffer results and evidence:
[parity/P03_ENCODER_PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.1/parity/P03_ENCODER_PERFORMANCE.md).

### Known exceptions

The 0.2.0 known exceptions still apply, apart from the new
`vertex-v1-streaming-s4` measurement above:

- Allocating index streaming maxima: `index-2-v0-streaming-s4` 1.508–3.625 and
  `index-2-v1-streaming-s4` 2.189–2.465. Filtered view `view-1-streaming-s4`
  1.343–1.636.
- Allocating `varied-view-3-streaming-s32` is unresolved, interval 1.619–2.590.
- Older allocating fetch failure, inconclusive overdraw and sloppy
  simplification maxima, and Cargo-default `partition_clusters` mean / max of
  1.450 / 1.778.
- A malformed meshlet tail accepted by upstream scalar and rejected by upstream
  SSE remains a strict-sweep discrepancy; Rust follows the scalar output.

## [0.2.0] - 2026-10-06

### Added

- Scalar APIs for stripification, vertex-cache/fetch and raster analysis,
  opacity micromaps and tangent generation. Normal generation and remeshing
  require `experimental`. These APIs also have checked limits and caller-buffer
  forms.
- Optional `parallel` feature using Rayon and `std`: ordered batches for LOD
  chains, vertex/index encoding, EXT view decoding and all four meshlet builders.
  With `clusterlod`, batches also build cluster-LOD DAGs and hierarchy forests.
  Successful output matches sequential calls; per-item errors and limits are
  checked with private worker workspaces.
- Default `simd` feature: audited vertex, filter and meshlet decoding for x86
  SSE2/SSSE3/SSE4.1, AArch64 NEON and wasm simd128. Runtime x86 dispatch uses
  `std`; scalar fallback remains the canonical reference. Disable defaults for
  an unsafe-free build, optionally adding `std`.

### Changed

- Cluster LOD reuses validated attributes and boundary flags, compacts sparse
  groups and invalidates the cached position range after dilation. The frozen
  RFC 113 recovery corpus records aggregate Rust/scalar-C++ ratios of 1.196
  with thin LTO and 1.090 with Cargo defaults; 15 supported layouts and 90
  codec comparisons match byte for byte.
- P05's final selected compact/coverage/raster-overdraw scope passes its bars.
  Opacity-map compact family means are 0.557 / 0.578× scalar C++ time under
  thin-LTO / default consumer profiles. The full matrix was not refreshed.
- Current native SIMD family means and caller-buffer families pass the
  registered 0.2 rule; allocating index, vertex and filtered-view maxima fail.
  Symmetric JavaScript WASM adapters pass that rule, without qualifying the
  Vec-returning Rust WASM API. Unresolved A/A rows retain their interval widths.
- Small hierarchy batches run sequentially to avoid Rayon dispatch overhead;
  recorded caller-side pool overhead can still make batches slower.
- README now records the meshopt 0.6.2 comparison. meshopt is faster at
  allocating vertex decode and Oct/Quat/Exp encoding (Exp about 3.4×).
  meshoptimizer-rs has lower five-case mean times for matched-v1 vertex encoding, Color
  encoding, Exp/Color decoding and scan meshlets. These are shared-host
  measurements on a Ryzen 9 7945HX, not universal speed claims.
- MSRV remains **Rust 1.88**, edition 2021. Crate version is 0.2.0; internal
  development phase numbers do not name published releases.

### Known exceptions

- Owner-approved allocating `vertex-v1-streaming-s4` exception: 1.529–2.118×
  C++ time; use the caller-buffer API (owner records about 1.04×). The fix is
  planned for 0.2.1. The later complete report also records resolved allocating index
  maxima of 1.508–3.625 and 2.189–2.465, and filtered-view 1.343–1.636.
- Allocating varied filtered-view stride-32 streaming remains unresolved,
  interval 1.619–2.590; A/A upper bounds are 1.284 / 1.026. The roadmap names
  the filtered-view maximum near 1.64 and a small S3 case; the latest S3 report
  records caller-buffer `varied-filter-3-resident-s32` at 1.026–1.124× scalar Rust.
- Older allocating fetch failure, inconclusive overdraw/sloppy maxima and
  default partitioning 1.450 / 1.778 mean / max remain in their original scope.
  Stricter SIMD amendment targets continue in 0.3, with encoder performance
  first among 0.3 priorities.
- The malformed meshlet tail accepted by upstream scalar and rejected by SSE
  remains a strict-sweep discrepancy; Rust follows canonical scalar output.
  Two stride-32 RFC 113 layouts are invalid because of their protect masks.
- AVX2, AVX-512 and 32-bit ARM SIMD are absent. Compile-only checks and short
  fuzz runs do not establish full per-target runtime or release fuzz budgets.

See [README.md](README.md) for workarounds and links to the qualification records.

## [0.1.0] - 2026-10-04

First public release. Ports meshoptimizer 1.3 (commit
`4c203430ca565cb59a468a91922c76c208169536`, `src` identical to tag v1.3) with
byte-identical output to scalar C++ on every recorded fixture, sweep case and
wasm32 run.

### Added

- Foundation: checked borrowed views (`Positions`, `Attributes`, their mutable
  forms), `VertexFlags`, a typed `Error`, a reusable `Workspace` with fallible
  allocation and per-call memory and work `Limits`, and `no_std` with `alloc`.
- Vertex processing: vertex cache (standard, strip, FIFO), overdraw and vertex
  fetch optimization; vertex remap generation (single, multi-stream, custom),
  remap of vertex and index buffers, index filtering, shadow, adjacency,
  tessellation and provoking index buffers, and position remap.
- Quantization: `quantize_unorm`, `quantize_snorm`, `quantize_half`,
  `quantize_float`, `dequantize_half` and `compute_position_exponent`.
- Simplification: `simplify`, `simplify_with_attributes`,
  `simplify_with_update`, `simplify_sloppy`, `simplify_prune`,
  `simplify_points` and `simplify_scale`, with the LockBorder, Sparse,
  ErrorAbsolute, Prune, Regularize, RegularizeLight and Permissive options,
  and LOCK/PROTECT/PRIORITY vertex flags. PreserveFolds and ErrorClamped are
  behind the `experimental` feature.
- Meshlets: scan, standard, flex and spatial builders and their bound; cluster,
  meshlet and sphere bounds; meshlet optimization (all levels); meshlet index
  extraction; cluster partitioning; spatial sorting and point clustering.
- `clusterlod` feature: the cluster-LOD builder from upstream's
  `demo/clusterlod.h`, exact against the pinned demo.
- `codec` module: vertex buffer, index buffer and index sequence encoding and
  decoding (versions 0 and 1, vertex levels 0–9); Oct, Quat, Exp and Color
  filter encoding and decoding; meshlet encoding and decoding; and checked
  `EXT_meshopt_compression` buffer-view decoding.
- Allocating, caller-buffer (`_into`) and, where upstream works in place,
  `_in_place` forms of each operation.
- Parity harness (not published): C++ differential runs, seeded sweeps,
  wasm32 identity, fuzzing, cross-platform output replay in CI and paired
  benchmarks under thin-LTO and Cargo-default consumer profiles.

### Changed (relative to the C++ API)

- Codec format versions and levels are per-call values (`VertexEncoding`,
  `IndexEncoding`) instead of global setters.
- `meshopt_setAllocator` is replaced by `Workspace` and `Limits`.
- Invalid input, overflow, non-finite geometry and exhausted limits return
  `Error` instead of asserting or invoking undefined behaviour.

### Known limitations

- Not yet ported: analyzers, opacity maps, tangents, normals and remeshing.
- Five million-vertex cases stay above the 1.5× per-case bar after the
  second-stage retest (see the README's performance section). Allocating
  `optimize_vertex_fetch` on a sparse mesh fails it (95% interval about
  1.55–2.09× of C++ time). Allocating `optimize_overdraw` and two
  `simplify_sloppy` cases on disconnected meshes are inconclusive, with
  intervals that reach 1.51–1.58×.
- `partition_clusters` under Cargo defaults: family mean 1.45×, worst case
  1.78× of C++ time.
- The `clusterlod` build path is not yet competitive with C++.
- Codecs are scalar; upstream's SIMD decoders are faster.

[Unreleased]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Tavrin/meshoptimizer-rs/releases/tag/v0.1.0
