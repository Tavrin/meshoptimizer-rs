# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - unreleased

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
- Allocating `optimize_vertex_fetch` on a million-vertex sparse mesh takes
  1.73× (thin LTO) and 1.85× (Cargo defaults) of C++ time, above the 1.5×
  per-case bar.
- `partition_clusters` under Cargo defaults: family mean 1.45×, worst case
  1.78× of C++ time.
- The `clusterlod` build path is not yet competitive with C++.
- Codecs are scalar; upstream's SIMD decoders are faster.

[Unreleased]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Tavrin/meshoptimizer-rs/releases/tag/v0.1.0
