# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-10-05

### Added

- Optional `parallel` feature, using Rayon and enabling `std`: ordered batch
  APIs for LOD chains, vertex/index encoding, EXT view decoding and all four
  meshlet builders. With `clusterlod`, batches of independent cluster-LOD DAGs
  and hierarchy forests are also available.
- Batches check errors and limits per item and give each worker a private
  workspace. Output is deterministic at any thread count; tests compare it with
  sequential output at 1, 2, 8 and N threads, including failure isolation and
  late position mutation.
- RFC 113 harness: three-way Rust/scalar-C++/Moss-style-C++ comparisons,
  portable layout and codec checks, scratch-budget regressions, profiling and
  adaptive paired timing with source and binary identities.
- Parallel batch harness and recorded speed curves against sequential Rust on
  sixteen variants of one authored mesh. The full-family curve records
  16-thread medians of 8.201× for LOD chains, 7.599× for mixed encoding,
  3.284× for EXT view decoding, 6.136× for standard meshlets and 7.818× for
  cluster LOD. These results come from one corpus on a shared host. See
  [the P06 record](parity/p06/README.md) for the full table, dispersion and
  hierarchy follow-up.

### Changed

- Cluster-LOD builds reuse validated data and compact sparse groups. Validated
  attributes and boundary flags are borrowed instead of copied per group, and
  simplification scratch stays bounded. Dilation invalidates the cached position
  range before further simplification.
- The final RFC 113 recovery measured aggregate Rust/scalar-C++ time ratios of
  1.196× with the Moss-like consumer profile (thin LTO) and 1.090× with Cargo
  release defaults. All 15 supported S2 layouts and all 90 codec comparisons
  are byte-identical. See [the recovery record](parity/results/rfc113-clod-recover.md).
- Small hierarchy batches run sequentially to avoid Rayon dispatch overhead.
  The hierarchy follow-up still records slowdowns, including caller-side
  `pool.install`.

### Known limitations

- Phase 0.5 (analyzers, opacity maps, tangents, normals and remeshing) is not
  included. SIMD work remains planned for phase 0.7.
- The two stride-32 S2 setups remain invalid because their protect mask exceeds
  the layout. Existing preprocessing and partitioning timing residuals remain;
  see the README. These records do not cover Moss runtime or GPU integration.

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

[Unreleased]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Tavrin/meshoptimizer-rs/releases/tag/v0.1.0
