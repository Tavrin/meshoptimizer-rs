# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- `optimize_overdraw` and its `_into` and `_in_place` variants check the finiteness of
  validated positions and of each cluster's sort value once instead of per
  coordinate and per intermediate, and charge work in totals when the
  remaining budget covers every charge. Output, errors and `Usage` are
  unchanged. On three production LOD-chain meshes they use 0.75× the
  instructions of 0.2.14 (0.74–0.76×), 0.88× those of optimesh 1.1.0
  (0.88–0.89×) and 1.09× those of scalar C++ 1.3 (1.088–1.091×). Across the
  benchmark corpus the ratios are 0.95× versus 0.2.14 (0.90–0.97×), 0.86×
  versus optimesh and 0.94× versus scalar C++ 1.3.
- `simplify_with_attributes` validates packed positions with contiguous
  attributes, and weights them, in branch-free scans that hand back to the
  checked loops at the first rejected vertex, and tests its attribute
  quadric tables without early exits. Results, errors, counted work and
  storage are unchanged. On three production LOD-chain meshes it uses
  0.79× the instructions of 0.2.14 (range 0.74–0.81×), 0.92× those of
  optimesh 1.1.0 (0.92–0.93×) and 1.09× those of scalar C++ 1.3
  (1.07–1.11×). Across the benchmark corpus the ratio versus 0.2.14 is
  0.92–0.97×.

## [0.2.14] - 2026-10-28

- `simplify`, `simplify_with_update` and `simplify_with_attributes` skip a
  positional-quadric finite check after input validation. On three production
  LOD-chain meshes, they use 0.93×, 0.94× and 0.96× the instructions of
  0.2.13, respectively, and 1.24×, 1.37× and 1.39× those of scalar C++ 1.3.
  The respective ranges versus 0.2.13 / scalar C++ 1.3 are
  0.91–0.94× / 1.22–1.26×, 0.93–0.95× / 1.34–1.43× and
  0.95–0.97× / 1.36–1.45×.
  Across the benchmark corpus, instruction ratios versus 0.2.13 / scalar C++ 1.3
  are `simplify` 0.98× / 1.06×, `simplify_with_attributes` 0.99× / 1.12×,
  `simplify_with_update` 0.99× / 1.10×, `simplify_sloppy` 1.00× / 1.38×,
  `simplify_prune` 1.00× / 1.51× and `simplify_points` 1.00× / 1.55×.
- `build_meshlets` and `build_meshlets_flex` charge a precomputed KD-tree
  search work bound in one step when it fits the remaining budget. On three
  production LOD-chain meshes, both use 0.87× the instructions of 0.2.13
  and 1.21× those of scalar C++ 1.3. Both ranges are
  0.86–0.87× versus 0.2.13 and 1.21–1.22× versus scalar C++ 1.3.
- `clusterlod::build_with_output` uses hardware square root for finite non-negative
  operands with `std`; `no_std` keeps the portable square root. On three
  production LOD-chain meshes, it uses 0.92× the instructions of 0.2.13
  and 1.31× those of scalar C++ 1.3.

Lower instruction ratios are better. All performance numbers are instruction counts.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.14/docs/PERFORMANCE.md) for method.

## [0.2.13] - 2026-10-27

- `simplify` uses 0.86–0.88× the instructions of 0.2.12 and 1.28–1.35×
  those of scalar C++ 1.3 on three production LOD-chain meshes.
  Across the benchmark corpus, instruction ratios versus 0.2.12 / scalar C++ 1.3
  are `simplify` 0.97× / 1.08×, `simplify_with_attributes` 0.99× / 1.13×,
  `simplify_with_update` 0.97× / 1.11×, `simplify_sloppy` 1.00× / 1.38×,
  `simplify_prune` 1.00× / 1.50× and `simplify_points` 1.00× / 1.55×.
  Lower is better.

All performance numbers are instruction counts.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.13/docs/PERFORMANCE.md) for method.

## [0.2.12] - 2026-10-27

- `simplify` uses 0.81–0.84× the instructions of 0.2.11 and 1.46–1.58×
  those of scalar C++ 1.3 on three production LOD-chain meshes.
  Across the benchmark corpus, instruction ratios versus 0.2.11 / scalar C++ 1.3
  are `simplify` 0.92× / 1.12×, `simplify_with_attributes` 0.94× / 1.14×,
  `simplify_with_update` 0.90× / 1.14×, `simplify_sloppy` 0.94× / 1.38×,
  `simplify_prune` 1.00× / 1.50× and `simplify_points` 1.00× / 1.55×.
  Lower is better.

All performance numbers are instruction counts.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.12/docs/PERFORMANCE.md) for method.

## [0.2.11] - 2026-10-26

- Add the `guide` module from docs/USING.md with task recipes, doctests and a
  Gotchas list.
- Add `# Errors` sections to every public fallible function.
- Add the `lod_chain`, `untrusted_input` and `meshlets` examples.
- Add README badges.
- Add llms.txt at the repository root; it is not included in the crate package.
- Remove internal phase labels from the crate documentation.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.11/docs/PERFORMANCE.md) for method.

## [0.2.10] - 2026-10-26

- `simplify_scale` on 64-byte little-endian byte views uses 0.52× the
  instructions of 0.2.9 and 1.07× those of C++ 1.3.
- `optimize_overdraw` uses 0.93× the instructions of 0.2.9 and 1.58× those
  of C++ 1.3 across the benchmark corpus and real meshes.
- `optimize_vertex_cache` uses 0.91× the instructions of 0.2.9 and 1.26×
  those of C++ 1.3 across the benchmark corpus and three real-mesh LOD chains.
  `optimize_vertex_cache_fifo` is unchanged.

No wall timing was taken because the host was loaded.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.10/docs/PERFORMANCE.md) for method.

## [0.2.9] - 2026-10-26

- `simplify_scale` on little-endian byte views with 64-byte records uses
  0.74× the instructions of 0.2.8 on three real-mesh LOD chains, and
  2.08× the instructions of C++ 1.3. Packed and other layouts are unchanged.
  No wall timing was taken because the host was loaded.
- Replace references to private design documents and an internal profile name
  in the documentation with plain descriptions.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.9/docs/PERFORMANCE.md) for method.

## [0.2.8] - 2026-10-26

- `build_meshlets_scan`, `build_meshlets`, `build_meshlets_flex`,
  `spatial_sort_triangles`, `partition_clusters` and `build_meshlets_spatial`
  take 0.80×, 0.93×, 0.93×, 0.93×, 0.99× and 0.99× the time of 0.2.7,
  respectively. Against C++ 1.3, they take 1.07×, 1.02×, 1.01×, 1.03×,
  1.45× and 1.12× the time. A full cluster-LOD build over a 41-mesh
  benchmark set takes 1.040× the time of C++ 1.3, compared with 1.05–1.07× in
  0.2.7.
- `simplify_scale` (packed positions), `optimize_vertex_cache`,
  `optimize_vertex_cache_fifo` and `optimize_overdraw` take 0.70×, 0.96×,
  0.97× and 0.98× the time of 0.2.7, respectively. Against C++ 1.3, they
  take 0.94×, 1.18×, 1.28× and 1.11× the time. On three real-mesh chains,
  instruction counts for `optimize_vertex_cache` and `optimize_overdraw`
  are 0.92–0.93× and 0.96–0.97× those of 0.2.7, respectively.
- Add [docs/MIGRATING_FROM_MESHOPT.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/MIGRATING_FROM_MESHOPT.md), a migration guide for users of the `meshopt` crate.
- CI now runs `cargo-semver-checks` on every pull request, so a change that
  breaks compatibility with the latest release fails before it ships.

Timings were measured on a shared host; regression checks used instruction counts.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/PERFORMANCE.md) for method.

## [0.2.7] - 2026-10-26

- Vertex fetch, adjacency, index remapping and one-triangle meshlet optimization
  take 0.75×, 0.76×, 0.83× and 0.89× the time of 0.2.6, respectively.
  Against optimesh 1.1.0, they take 0.96×, 1.01×, 1.04× and 1.08× the time.
- `quantize_half` takes 0.95× the time of 0.2.6 (95% geometric-mean interval
  [0.925, 0.984]); all 2^32 inputs produce the same output as 0.2.6.
- `optimize_overdraw` takes 0.85× the time of 0.2.6 on synthetic meshes and
  0.83× on three real meshes; against optimesh 1.1.0, it takes 1.12× and
  1.22× the time, respectively. Against C++ 1.3, the real meshes 214e, b6fd
  and a082 take 1.24×, 1.42× and 1.16× the time, down from 1.54×, 1.60×
  and 1.47×.
- Add opt-in `Workspace::set_retention(Retention::Lean)`. It releases scratch
  between algorithm families and after simplify. On the largest real-mesh LOD
  chain in the benchmark set (214e), peak RSS is 2.20 GiB instead of 2.85 GiB,
  and simplify chains take up to 1.20× the time. The default `Reuse` policy is unchanged;
  instruction counts are 1.000× those of 0.2.6.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.7/docs/PERFORMANCE.md) for method.

## [0.2.6] - 2026-10-26

- Add `BoundsPositions`, `compute_cluster_bounds_with_positions` and
  `compute_meshlet_bounds_with_positions` for repeated bounds queries over the
  same positions. The new API takes 1.24× (meshlet) and 0.99× (cluster) the time
  of C++ 1.3. Existing bounds API instruction counts are 1.00× those of 0.2.4.
- Native scalar vertex decode takes 0.91× the time of 0.2.5. Scalar Oct/Quat
  filters do less work. Output and errors are byte-identical to 0.2.5 across
  6,249,496 native cases and 3,740 wasm32 cases in each of debug and release.

Output, errors and existing APIs are unchanged; MSRV 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.6/docs/PERFORMANCE.md) for method.

## [0.2.5] - 2026-10-26

- Add `clusterlod::build_dag_into`, caller-owned `DagBuffers` and
  `parallel::build_dags_batch_into`. Output matches the C++ reference build
  byte for byte on 41/41 meshes. Across both mesh sets and consumer
  profiles, the Rust code takes 1.05–1.07× the time of scalar C++
  meshoptimizer 1.3 in aggregate, and at most 1.20× on any single mesh.
  Fuzzing ran for 4.6 CPU-hours with zero crashes.
- Add `FetchIndices`, `optimize_vertex_fetch_remap_with_indices` and
  `optimize_vertex_fetch_remap_with_indices_into`. In the thin-LTO scalar
  profile, the new API takes 0.62× the time of meshopt 0.7.0 (C++ 1.3),
  as a geometric mean, excluding handle construction. The existing
  fetch-remap API now takes 0.57× its 0.2.4 time. Output matches byte for
  byte.

Output, errors and existing APIs are unchanged. The minimum supported Rust
version remains 1.88.

See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.5/docs/PERFORMANCE.md) for method.

## [0.2.4] - 2026-10-25

Faster simplification and overdraw optimization. Output, errors, limits,
counted work and public APIs are unchanged. The minimum supported Rust
version is still 1.88.

Median simplify time as a ratio of Rust to C++ 1.3 (scalar) on the three real
meshes measured, 0.2.3 → 0.2.4. Lower is faster; 1.00 is equal.

| Mesh | 0.2.3 | 0.2.4 |
|---|---:|---:|
| 214e | 1.380 | 0.963 |
| b6fd | 1.298 | 0.947 |
| a082 | 1.052 | 0.941 |

See the [r2 measurements](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.4/parity/SIMPLIFY_LARGE_MESH_R2_PERFORMANCE.md).

Overdraw optimization remains a known exception: about 1.47–1.60× C++ 1.3
(scalar) on the largest real meshes measured.

0.2.4 shows no family regression against 0.2.3 on the frozen benchmark matrix.
See the [v2 release-check report](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.4/parity/release-check-v2/REPORT.md) for method and intervals.

## [0.2.3] - 2026-10-24

Memory-limit fix. There are no API changes. Output is byte-identical to 0.2.2
and to pinned meshoptimizer 1.3. The minimum supported Rust version is still
1.88.

A reused `Workspace` could fail a call with `LimitExceeded` that a fresh
`Workspace` would accept, because scratch kept from earlier calls counted
against `max_bytes`. For example, after simplifying a 7.2-million-vertex mesh,
`optimize_vertex_cache_in_place` on the same workspace failed under the
default 1 GiB limit. Now, when a reservation would exceed `max_bytes`, the
workspace first releases retained scratch the current call does not use, then
shrinks the buffers the call reuses to the size it needs. Storage in use is
still counted in full, and a call that does not fit on its own still fails.
Scratch is still reused without reallocation when it fits.

Simplification now frees its position hash table once position remapping is
done, as upstream does.

## [0.2.2] - 2026-10-24

Performance-only release. There are no API changes. Encoded output is
byte-identical to 0.2.1 and to pinned meshoptimizer 1.3. The minimum supported
Rust version is still 1.88.

### Changed

- Changes to the allocating encoders:
  - Index encoding skips the vertex FIFO search when the index is larger than
    every index seen since the last reset.
  - Index sequence encoding writes one-byte values directly.
  - Quaternion encoding handles runs where one component is clearly the
    largest in blocks of four records.
  - Separate Exp encoding rounds mantissas with exact integer arithmetic for
    the common stride-16 layout and precisions 1 to 22.
- Encoding of 32 or fewer Oct or Quat records is unchanged.
  Limits, work accounting and error order are unchanged.

### Performance

Geometric mean of Rust time / C++ meshoptimizer 1.3 time per encoder family,
allocating API, 0.2.1 → 0.2.2. Lower is faster; 1.00 is equal.

| Encoder | Cargo defaults | Thin-LTO profile |
|---|---:|---:|
| vertex | 0.666 → 0.667 | 0.660 → 0.657 |
| index | 1.019 → 0.773 | 1.016 → 0.764 |
| sequence | 0.931 → 0.924 | 0.986 → 0.940 |
| oct | 0.591 → 0.585 | 0.565 → 0.563 |
| quat | 1.055 → 0.700 | 0.681 → 0.669 |
| exp | 0.609 → 0.621 | 0.588 → 0.590 |

- Cargo defaults is a dependent's default release build: no LTO, 16 codegen
  units. The thin-LTO profile uses thin LTO and one codegen unit.
- In both profiles every family mean is at or below 1.25 and every one of the
  111 cases has a 95% upper bound at or below 1.50.
- Against the meshopt 0.6.2 crate, meshoptimizer-rs is faster on all seven
  shared encoder families (vertex, index, sequence, Oct, Quat, Exp, Color) in
  both profiles.
- Output is byte-identical: 17,264 exact comparisons against scalar and SIMD
  meshoptimizer 1.3 in each profile, with zero mismatches.

Intervals, per-case results and method:
[parity/P03_ENCODER_PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.2/parity/P03_ENCODER_PERFORMANCE.md).

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
  cluster-LOD recovery corpus records aggregate Rust/scalar-C++ ratios of 1.196
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
  Two stride-32 cluster-LOD test layouts are invalid because of their protect masks.
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

[0.2.14]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.13...v0.2.14
[0.2.13]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.12...v0.2.13
[0.2.12]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Tavrin/meshoptimizer-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Tavrin/meshoptimizer-rs/releases/tag/v0.1.0
