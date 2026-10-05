# meshoptimizer-rs

A pure safe Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3.

Every ported function produces byte-identical output to meshoptimizer 1.3
(scalar build): the same index order, the same error bits, the same encoded
bytes. Differential runs against the C++ library, seeded sweeps, fuzzing and a
wasm32 identity check prove this on the recorded inputs.

The crate is safe Rust (`#![forbid(unsafe_code)]`) and needs no C++ toolchain.
It supports `no_std` with `alloc`. Invalid input returns a typed `Error`
instead of undefined behaviour. A reusable `Workspace` holds scratch memory,
makes every allocation fallible and enforces per-call memory and work limits.

This is an independent project. It is not affiliated with meshoptimizer or its
author, Arseny Kapoulkine.

## Why use it

Compared with the C++ library, there is no C++ compiler or build script to
set up, results are the same on every target Rust supports (including
`wasm32-unknown-unknown`), and `no_std` builds work.

Compared with the [meshopt](https://crates.io/crates/meshopt) bindings, you
get checked slices instead of raw pointer/count pairs, `Result` instead of
assertions, explicit memory and work limits, and no global allocator hook.

Speed is not yet a reason to switch. The ports are measured against C++ and
are usually within 1.25× of its time, but they are scalar code. See
[Performance](#performance).

## Install

```sh
cargo add meshoptimizer-rs
```

Rust 1.88 or later. The library is imported as `meshoptimizer_rs`.

| Feature | Default | Effect |
|---|---|---|
| `std` | yes | `std::error::Error` for `Error`. Disable default features for `no_std`; an allocator is still required. |
| `clusterlod` | no | The cluster-LOD builder from upstream's `demo/clusterlod.h`. It reproduces the pinned demo exactly; the demo is not a stable upstream API. |
| `experimental` | no | Upstream functions and options marked experimental. |
| `parallel` | no | Rayon batches of independent meshes and buffer views; enables `std`. |

## Quickstart

All examples use this quad:

```rust
let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
let indices = [0, 1, 2, 2, 1, 3];
let mut workspace = meshoptimizer_rs::Workspace::default();
```

Simplify to a target index count and error:

```rust
use meshoptimizer_rs::{simplify, Positions, SimplifyOptions, SimplifySettings};

let settings = SimplifySettings {
    target_index_count: 3,
    target_error: 0.01,
    options: SimplifyOptions::EMPTY,
};
let lod = simplify(&indices, Positions::from_packed(&positions), settings, &mut workspace)?;
println!("{} indices, relative error {}", lod.indices.len(), lod.error);
```

Optimize for the vertex cache, then for overdraw:

```rust
use meshoptimizer_rs::{optimize_overdraw, optimize_vertex_cache, Positions};

let cached = optimize_vertex_cache(&indices, positions.len(), &mut workspace)?;
let ordered = optimize_overdraw(&cached, Positions::from_packed(&positions), 1.05, &mut workspace)?;
```

Build meshlets (64 vertices and 124 triangles at most, cone weight 0.25):

```rust
use meshoptimizer_rs::{build_meshlets, MeshletSettings, Positions};

let built = build_meshlets(
    &indices,
    Positions::from_packed(&positions),
    MeshletSettings::default(),
    0.25,
    &mut workspace,
)?;
for meshlet in &built.meshlets {
    let start = meshlet.vertex_offset as usize;
    let vertices = &built.vertices[start..start + meshlet.vertex_count as usize];
    println!("{} triangles over vertices {:?}", meshlet.triangle_count, vertices);
}
```

Compress vertex and index buffers (the `EXT_meshopt_compression` formats;
buffers are little-endian bytes):

```rust
use meshoptimizer_rs::codec::{
    decode_index_buffer, decode_vertex_buffer, encode_index_buffer, encode_vertex_buffer,
    IndexEncoding, VertexEncoding,
};

let vertex_bytes: Vec<u8> = positions.iter().flatten().flat_map(|v: &f32| v.to_le_bytes()).collect();
let packed = encode_vertex_buffer(&vertex_bytes, 4, 12, VertexEncoding::DEFAULT, &mut workspace)?;
assert_eq!(decode_vertex_buffer(4, 12, &packed, &mut workspace)?, vertex_bytes);

let packed = encode_index_buffer(&indices, IndexEncoding::DEFAULT, &mut workspace)?;
let decoded = decode_index_buffer(indices.len(), 4, &packed, &mut workspace)?; // u32 bytes
```

These examples are compiled and run as doc tests in `src/lib.rs`.

### Parallel batches

Enable `parallel` to use `meshoptimizer_rs::parallel`. Batch functions take
slices of descriptors and return results in input order. Each successful item
has the same bytes as its sequential call, regardless of the number of threads.

```rust
use meshoptimizer_rs::parallel::{encode_buffers_batch, EncodeInput};
use meshoptimizer_rs::{codec::IndexEncoding, Limits};

let indices = [0, 1, 2, 2, 1, 3];
let inputs = [EncodeInput::Triangles {
    indices: &indices,
    encoding: IndexEncoding::DEFAULT,
}];
let results = encode_buffers_batch(&inputs, Limits::default())?;
let encoded = results[0].as_ref()?;
# Ok::<(), meshoptimizer_rs::Error>(())
```

`simplify_lod_chains_batch` simplifies each chain's levels in order, using the
previous level's indices and the original positions and optional attributes.
It returns the sequential simplifier's error values unchanged. It adds no
compaction, cache optimization or consumer-specific LOD policy.
`encode_buffers_batch` accepts vertex, triangle and index-sequence buffers;
`decode_buffer_views_batch` decodes complete EXT views.
`build_meshlets_batch` selects the scan, standard, flex or spatial builder per
mesh. With `clusterlod`, `build_cluster_lod_batch` builds independent DAGs and
`build_cluster_hierarchies_batch` builds their spatial forests. A DAG retains
sequential group order and refinement IDs; its shared border dilation prevents
independent execution of groups within that mesh. Dilation can change an item's
positions even on failure, as in the sequential API.

The return type is `Result<Vec<Result<T, Error>>, Error>`. The outer error
means result-slot allocation failed before any item ran. An inner error belongs
to that item; other items still run. Panics propagate through Rayon.
Every executing thread has a private `Workspace`, cleared between items so
retained scratch cannot make limit outcomes depend on scheduling. Limits apply
per item; a chain charges all live level outputs and cumulative work. The
batch's result slots and Rayon infrastructure are outside those limits. All
successful outputs remain live together, so callers should bound batch size
when aggregate memory matters.

Calls use Rayon's global pool, or the pool selected by
`pool.install(|| encode_buffers_batch(&inputs, limits))`. The library does not
explicitly create a pool. Rayon lazily initializes its global pool on first use
when no pool was configured. If that initialization
fails, Rayon panics; this infrastructure failure is distinct from the per-item
`Error` isolation described above. Default and `no_std` builds do not compile
Rayon or these APIs. Small hierarchy batches run sequentially inside the batch
API to avoid dispatch overhead; larger forests use Rayon (see the function rustdoc).
Parallel execution requires native threads; the scalar WASM build is unchanged.
See [the p06 record](parity/p06/README.md) for thread-count determinism and the
measured speed curve against sequential Rust.

### API conventions

- Positions and attributes are borrowed views: `Positions::from_packed`,
  `from_interleaved` or `from_bytes` (explicit byte order); `Attributes` for
  weighted attribute streams.
- Most operations have three forms: allocating (`simplify`), caller buffer
  (`simplify_into`) and, where upstream rewrites in place, `_in_place`.
  In-place calls leave the input unchanged on failure. Caller-buffer calls keep
  the unused tail but may have written part of the used prefix when they fail
  late.
- `Workspace::default()` allows 1 GiB of crate-owned output plus retained
  scratch and 2^34 work units per call. Set explicit `Limits` to change this.
  The limits cover allocations the crate makes, not the operating system or an
  aborting allocator.
- Codec format versions and levels are passed per call (`VertexEncoding`,
  `IndexEncoding`) rather than set globally.
- Unknown option or flag bits are rejected.

## Coverage

Status of the meshoptimizer 1.3 public API (`src/meshoptimizer.h` at the pinned
commit). "Exact" means byte-identical to scalar C++ on every recorded fixture,
seeded sweep case and executed wasm32 run, with zero mismatches. Sources:
[parity/COVERAGE.md](parity/COVERAGE.md) and the 0.1.x records
([MEASURED_P01X.json](parity/MEASURED_P01X.json),
[DECISIONS.md](parity/DECISIONS.md) D145).

### Indexing and vertex processing (phases 0.1 and 0.1.x)

| Upstream | Rust | Status |
|---|---|---|
| `meshopt_generateVertexRemap`, `…Multi`, `…Custom` | `generate_vertex_remap`, `generate_vertex_remap_multi`, `generate_vertex_remap_custom` | exact |
| `meshopt_remapVertexBuffer`, `meshopt_remapIndexBuffer` | `remap_vertex_buffer`, `remap_index_buffer` | exact |
| `meshopt_filterIndexBuffer`, `…Multi` | `filter_index_buffer`, `filter_index_buffer_multi` | exact |
| `meshopt_generateShadowIndexBuffer`, `…Multi` | `generate_shadow_index_buffer`, `generate_shadow_index_buffer_multi` | exact |
| `meshopt_generatePositionRemap` | `generate_position_remap` | exact |
| `meshopt_generateAdjacencyIndexBuffer` | `generate_adjacency_index_buffer` | exact |
| `meshopt_generateTessellationIndexBuffer` | `generate_tessellation_index_buffer` | exact |
| `meshopt_generateProvokingIndexBuffer` | `generate_provoking_index_buffer` | exact |
| `meshopt_optimizeVertexCache` | `optimize_vertex_cache` | exact |
| `meshopt_optimizeVertexCacheStrip` | `optimize_vertex_cache_strip` | exact |
| `meshopt_optimizeVertexCacheFifo` | `optimize_vertex_cache_fifo` | exact |
| `meshopt_optimizeOverdraw` | `optimize_overdraw` | exact |
| `meshopt_optimizeVertexFetch`, `…Remap` | `optimize_vertex_fetch`, `optimize_vertex_fetch_remap` | exact; one timing residual |
| `meshopt_quantizeUnorm`, `…Snorm`, `…Half`, `…Float`, `meshopt_dequantizeHalf` | `quantize_unorm`, `quantize_snorm`, `quantize_half`, `quantize_float`, `dequantize_half` | exact |
| `meshopt_computePositionExponent` | `compute_position_exponent` | exact |

### Simplification (phases 0.1 and 0.1.x)

| Upstream | Rust | Status |
|---|---|---|
| `meshopt_simplify` | `simplify` | exact indices and error bits |
| `meshopt_simplifyWithAttributes` | `simplify_with_attributes` | exact; weights and LOCK/PROTECT/PRIORITY flags |
| `meshopt_simplifyWithUpdate` | `simplify_with_update` | exact, including updated positions and attributes |
| `meshopt_simplifySloppy` | `simplify_sloppy` | exact |
| `meshopt_simplifyPrune` | `simplify_prune` | exact |
| `meshopt_simplifyPoints` | `simplify_points` | exact |
| `meshopt_simplifyScale` | `simplify_scale` | exact |
| Options | `SimplifyOptions::LOCK_BORDER`, `SPARSE`, `ERROR_ABSOLUTE`, `PRUNE`, `REGULARIZE`, `REGULARIZE_LIGHT`, `PERMISSIVE` | exact |
| Experimental options | `PRESERVE_FOLDS`, `ERROR_CLAMPED` | exact; behind `experimental` |

### Meshlets and spatial ordering (phase 0.3)

| Upstream | Rust | Status |
|---|---|---|
| `meshopt_buildMeshlets`, `…Scan`, `…Flex`, `…Spatial`, `…Bound` | `build_meshlets`, `build_meshlets_scan`, `build_meshlets_flex`, `build_meshlets_spatial`, `build_meshlets_bound` | exact |
| `meshopt_computeClusterBounds`, `…MeshletBounds`, `…SphereBounds` | `compute_cluster_bounds`, `compute_meshlet_bounds`, `compute_sphere_bounds` | exact |
| `meshopt_optimizeMeshlet`, `…Level` | `optimize_meshlet`, `optimize_meshlet_level` | exact |
| `meshopt_extractMeshletIndices` | `extract_meshlet_indices` | exact |
| `meshopt_partitionClusters` | `partition_clusters` | exact; one timing residual |
| `meshopt_spatialSortRemap`, `…Triangles`, `meshopt_spatialClusterPoints` | `spatial_sort_remap`, `spatial_sort_triangles`, `spatial_cluster_points` | exact |
| `demo/clusterlod.h` | `clusterlod` module (feature) | exact against the pinned demo; RFC 113 recovery measured on the frozen S2 corpus |

### Compression codecs (phases 0.2 and 0.4, `codec` module)

| Upstream | Rust | Status |
|---|---|---|
| `meshopt_encodeVertexBuffer`, `…Level`, `…Bound`; `meshopt_decodeVertexBuffer` | `encode_vertex_buffer`, `encode_vertex_buffer_bound`, `decode_vertex_buffer` | exact; versions 0/1, levels 0–9 |
| `meshopt_encodeIndexBuffer`, `…Bound`; `meshopt_decodeIndexBuffer` | `encode_index_buffer`, `encode_index_buffer_bound`, `decode_index_buffer` | exact; versions 0/1 |
| `meshopt_encodeIndexSequence`, `…Bound`; `meshopt_decodeIndexSequence` | `encode_index_sequence`, `encode_index_sequence_bound`, `decode_index_sequence` | exact; versions 0/1 |
| `meshopt_encodeVertexVersion`, `meshopt_encodeIndexVersion` | `VertexEncoding`, `IndexEncoding` (per call) | intentional replacement of global setters |
| `meshopt_decodeVertexVersion`, `meshopt_decodeIndexVersion` | `decode_vertex_version`, `decode_index_version` | exact |
| `meshopt_encodeFilterOct`, `…Quat`, `…Exp`, `…Color` | `encode_filter_oct`, `encode_filter_quat`, `encode_filter_exp` (`ExpMode`), `encode_filter_color` | exact |
| `meshopt_decodeFilterOct`, `…Quat`, `…Exp`, `…Color` | `decode_filter_oct`, `decode_filter_quat`, `decode_filter_exp`, `decode_filter_color` | exact (scalar output) |
| `meshopt_encodeMeshlet`, `…Bound`; `meshopt_decodeMeshlet`, `…Raw` | `encode_meshlet`, `encode_meshlet_bound`, `decode_meshlet`, `decode_meshlet_raw` | exact |
| glTF `EXT_meshopt_compression` buffer view | `BufferView`, `decode_buffer_view` | exact; extension rules enforced |

Caller-buffer (`_into`) and in-place forms are listed in the rustdoc.
`meshopt_setAllocator` is intentionally replaced by `Workspace` and `Limits`.

### Parallel batches (phase 0.6, `parallel` feature)

These batch APIs compose the sequential Rust operations. Their recorded tests
compare output at 1, 2, 8 and N threads; output order and per-item errors do not
depend on thread count.

| Operation | Batch API | Status |
|---|---|---|
| LOD chains | `simplify_lod_chains_batch` | sequential indices and error bits |
| Vertex/index encoding | `encode_buffers_batch` | sequential bytes |
| EXT view decoding | `decode_buffer_views_batch` | sequential bytes and errors |
| Scan, standard, flex and spatial meshlets | `build_meshlets_batch` | sequential meshlet fields and buffers |
| Cluster-LOD DAGs | `build_cluster_lod_batch` | sequential DAGs and position mutation; also requires `clusterlod` |
| Hierarchy forests | `build_cluster_hierarchies_batch` | sequential hierarchy fields; small batches run sequentially; also requires `clusterlod` |

### Not yet ported

- Phase 0.5: the analyzers (`meshopt_analyzeVertexCache`, `…Overdraw`,
  `…VertexFetch`, `…Coverage`), opacity maps, tangent and normal generation, and
  remeshing.
- Phase 0.7 work (see [Roadmap](#roadmap)).
- SIMD decoders and filters. The codecs produce upstream's scalar output and
  are scalar code.

## Performance

Ratios are Rust time divided by C++ time for the same call on the same input.
Smaller is faster; 1.00 is parity. The baseline is scalar C++ 1.3
(`MESHOPTIMIZER_NO_SIMD`), the same build the parity proof uses.

The pass bar is defined in RFC §6.1 and has not changed since it was
registered. For each function family, the geometric mean over cases must be
≤ 1.25 and no case may exceed 1.50. This must hold under both consumer
profiles:

- thin LTO: thin LTO, one codegen unit, `opt-level = 3` (named `moss` in the
  records);
- Cargo defaults: the default `release` profile (no LTO, 16 codegen units).

The crate's own fat-LTO profile does not apply to dependents, so the bar does
not use it.

All timings were taken on one AMD Ryzen 9 7945HX core under Linux x86-64,
with other load on the host (recorded per sample). Each case runs one warm-up
and then 10–30 alternating Rust/C++ pairs on the same core; the reported ratio
is the median paired ratio. Timing includes validation, required copies,
allocation and the algorithm itself. Raw samples, load, and source and
executable hashes are kept with the records. The host was not quiet, and the
figures are not general speed claims.

Results, as family geometric mean / worst case:

| Phase | Families | Thin LTO | Cargo defaults | Record |
|---|---:|---|---|---|
| 0.1 cache, overdraw, simplify | 5 | GM 0.83–1.19, max 1.42; all pass | GM 0.96–1.22, max 1.36; all pass | [MEASURED_PERFORMANCE.md](parity/MEASURED_PERFORMANCE.md) |
| 0.1.x preprocessing, simplify variants, quantization | 32 | all family means pass; case residuals below | all family means pass; case residuals below | [DECISIONS.md](parity/DECISIONS.md) D145, D146 |
| 0.2 decoders (raw codecs) | 2 APIs | one profile recorded: allocating GM 1.01, max 1.38; caller-buffer GM 0.97, max 1.27; pass | not recorded | [P02_RESULTS.md](parity/P02_RESULTS.md) |
| 0.3 meshlets, partitioning, spatial | 15 | all pass; `partition_clusters` 1.23 / 1.37 | 14 pass; `partition_clusters` **1.45 / 1.78** | [MEASURED_PERFORMANCE.md](parity/MEASURED_PERFORMANCE.md) |
| 0.4 encoders, filters, meshlet codec | 24 | GM 0.61–1.24, max 1.46; all pass | GM 0.59–1.24, max 1.47; all pass | [benchmark-0.4-moss.json](parity/results/benchmark-0.4-moss.json), [benchmark-0.4-default.json](parity/results/benchmark-0.4-default.json) |

The 0.1, 0.1.x and 0.3 records also check memory (requested output plus
scratch ≤ 1.25× C++); every family passes. The 0.2 and 0.4 records time only.

### Known residuals

The final 0.1 requalification (2026-10-04, AMD Ryzen 9 7945HX, Linux x86-64)
passes every family mean and memory bar under both profiles. Cases whose
maximum was above 1.5× went to a pre-registered second stage
([D146](parity/DECISIONS.md)): 30 fresh interleaved pairs on one quiet core,
decided on a 95% interval. Two of the flagged cases passed. The other five
remain residuals, and the bar has not been relaxed for them
([stage2-0.1.json](parity/results/stage2-0.1.json)):

| Case (million-vertex inputs) | Profile | 95% interval | Verdict |
|---|---|---:|---|
| allocating `optimize_vertex_fetch`, sparse | thin LTO | 1.55–2.07× | fail |
| allocating `optimize_vertex_fetch`, sparse | Cargo defaults | 1.54–2.09× | fail |
| allocating `optimize_overdraw`, disconnected | thin LTO | 1.48–1.58× | inconclusive |
| `simplify_sloppy`, disconnected, mode 1 | Cargo defaults | 1.44–1.51× | inconclusive |
| `simplify_sloppy`, disconnected, mode 2 | Cargo defaults | 1.39–1.52× | inconclusive |

The means of the affected families pass (`vertex_fetch` 0.97 / 0.99).

`partition_clusters` misses the bar under Cargo defaults, with a family mean
of 1.45× and a worst case of 1.78× (`medium-seams-into`). It passes under thin
LTO (1.23 / 1.37). Profiling shows more instructions and branches in the
adjacency and partition code; see
[MEASURED_PERFORMANCE.md](parity/MEASURED_PERFORMANCE.md).

### Cluster-LOD build

The `clusterlod` build path borrows validated attributes and boundary flags
without per-group copies, preserves bounded scratch, and invalidates the
position range after dilation. On the frozen S2 corpus, aggregate Rust/scalar-C++
time ratios are 1.196× with the Moss-like consumer profile (`opt-level = 3`,
thin LTO, one codegen unit) and 1.090× with Cargo release defaults
(`opt-level = 3`, no LTO, 16 codegen units). These are ratios of summed per-mesh
medians. Measurements used a pinned physical core on the AMD Ryzen 9 7945HX
Linux x86-64 host, with shared-machine load recorded per burst.

All 15 supported layouts and all 90 codec comparisons matched byte for byte.
The two stride-32 S2 setups remain invalid because their protect mask exceeds
the layout. See [the recovery record](parity/results/rfc113-clod-recover.md)
for raw timing identities, adaptive sampling and per-mesh results. These
measurements cover that corpus; they do not establish Moss runtime performance.

### Parallel batch speed

The following full-family curve compares parallel batches with sequential Rust
on an AMD Ryzen 9 7945HX (16 physical cores / 32 logical CPUs), using Cargo
release defaults (`opt-level = 3`, 16 codegen units, LTO false). Inputs are
sixteen translated/scaled variants of the pinned upstream `demo/pirate.obj`,
2,889 vertices / 5,010 triangles per mesh. This is one authored mesh, not a
diverse asset suite or a Moss cook benchmark. The shared host's one-minute
load was 15.46–18.00.

Each speed-up is the median of five paired sequential/parallel time ratios;
values below one mean batching is slower. Serial milliseconds are the median
baseline in the one-thread row. Validation, required copies, allocation and
execution are timed; input generation and pool creation are outside timing.

| Family | Serial ms | 1 thread | 2 | 4 | 8 | 16 |
|---|---:|---:|---:|---:|---:|---:|
| LOD chains | 32.898 | 1.026× | 2.001× | 3.211× | 6.216× | 8.201× |
| Mixed encoding | 3.421 | 1.207× | 2.173× | 3.453× | 5.445× | 7.599× |
| EXT view decoding | 0.715 | 1.193× | 2.094× | 3.031× | 4.261× | 3.284× |
| Standard meshlets | 30.022 | 1.112× | 1.553× | 4.102× | 3.872× | 6.136× |
| Cluster LOD | 126.427 | 0.991× | 1.911× | 3.913× | 5.505× | 7.818× |
| Hierarchy forests | 0.086 | 0.845× | 0.817× | 0.852× | 0.988× | 0.623× |

This curve predates the hierarchy dispatch review fix. Small hierarchy batches
now run sequentially inside the API. The final hierarchy-only follow-up has
paired medians of 0.878/0.836/0.775/0.545/0.725x at 1/2/4/8/16 threads,
including caller-side `pool.install` overhead. The fallback does not eliminate
these measured slowdowns.

Decoding gains decline after eight threads. Dispersion is material: at sixteen
threads, cluster LOD ranges 1.82–10.23× and meshlets 4.71–9.81× across the five
pairs. These medians are not universal gains or C++ algorithmic comparisons.
See [the P06 record](parity/p06/README.md) for raw pairs, identities,
thread-count determinism tests and the hierarchy follow-up.

The bar compares against scalar C++. Upstream's SIMD vertex decoder is 2–5×
faster than this crate on the recorded cases (Rust/SIMD throughput
0.20–0.47), and index decoding runs at 0.7–1.0× of SIMD throughput. The full
table is in [P02_PERFORMANCE.md](parity/P02_PERFORMANCE.md). SIMD is planned
for phase 0.7.

## Parity and verification

The oracle is meshoptimizer commit `4c203430ca565cb59a468a91922c76c208169536`.
Its `src` tree is identical to tag v1.3. The harness refuses any other or
modified checkout. C++ is built scalar-strict: SIMD off, no FMA contraction.

Exactness is checked in five ways. Each one compares every meaningful output
byte (index order, counts, f32 error bits, encoded bytes, status) with no
tolerance.

1. Fixtures: upstream's own `demo/tests.cpp` bodies and JS test calls,
   extracted unchanged, plus generated edge cases, run through C++, native Rust
   and wasm32 Rust.
2. Seeded sweeps: 10,000 cases per 0.1 function and 2,000 per later
   function, covering grids, seamed spheres, degenerate and disconnected
   meshes, extreme scales, and every option and flag combination.
3. wasm32 identity: the `wasm32-unknown-unknown` build runs in Node on the
   same inputs and must match native output.
4. Fuzzing: stable seeded mutation targets and cargo-fuzz targets with
   invariant checks. The 0.1 release budget is four CPU-hours per target, with
   zero findings ([fuzz.json](parity/results/fuzz.json)).
5. Cross-platform replay: CI replays the 0.1 Linux x86-64 output corpus
   on macOS arm64, Windows x86-64 and Linux arm64 and compares bytes exactly.
   Only Linux x86-64 and wasm32 results are verified locally; the other
   platforms' results come from those CI runs.

Counts and identities are in [MEASURED_PARITY.md](parity/MEASURED_PARITY.md).
They come from finite tests on recorded inputs and do not prove exactness for
all inputs.

To reproduce, check out meshoptimizer at the pinned commit and run:

```sh
export MESHOPT_REFERENCE=/path/to/meshoptimizer
export CARGO_TARGET_DIR=/path/to/build
export MESHOPT_ARTIFACTS=/path/to/artifacts
parity/check-reference.sh
parity/run.sh --phase 0.1          # fixtures: C++, native Rust, wasm32
parity/sweep.sh --phase 0.1        # seeded sweep
parity/benchmark.sh --phase 0.1 --consumer-profile moss --enforce
parity/benchmark.sh --phase 0.1 --consumer-profile default --enforce
parity/report.sh --phase 0.1 --verify-artifacts
```

Replace `0.1` with `0.2`, `0.3` or `0.4` for later phases. The harness and its
options are documented in [parity/README.md](parity/README.md); every decision
behind it is in [parity/DECISIONS.md](parity/DECISIONS.md). The harness and the
C++ reference are not part of the published package.

## Relationship to upstream

meshoptimizer is written by Arseny Kapoulkine and released under the MIT
licence. This crate is an independent port of its algorithms. It is not
affiliated with or endorsed by the upstream project. Upstream recommends the
[meshopt](https://crates.io/crates/meshopt) bindings for Rust; use them if you
want the C++ code itself, including its SIMD paths.

[UPSTREAM.md](UPSTREAM.md) records provenance and the intentional API
differences. [LICENSE](LICENSE) keeps upstream's MIT notice.

Each crate version names the upstream commit it matches. A new upstream
release is ported in a new crate version, and the pinned commit moves only
when every ported function passes the full parity run against it.
Upstream functions marked experimental stay behind the `experimental` feature.

Other pure-Rust projects exist ([meshopt-rs](https://crates.io/crates/meshopt-rs),
[optimesh](https://crates.io/crates/optimesh)). This project has not qualified
them, and none of their code is used.

## Roadmap

- 0.5 next: the rest of the 1.3 API (analyzers, opacity maps, tangents, normals and
  remeshing).
- 0.7: SIMD decoders and filters (SSE2/SSSE3/SSE4.1, NEON, wasm simd128) in
  one audited `unsafe` module with runtime dispatch, checked against the scalar
  path.

Exact parity with upstream stays the default. An algorithm that produces
different output for better quality or speed would be opt-in, and would not
change the output of any existing call.

## Licence

MIT. See [LICENSE](LICENSE), which includes upstream meshoptimizer's MIT notice.

See [CONTRIBUTING.md](CONTRIBUTING.md) for checks and DCO sign-off, and
[SECURITY.md](SECURITY.md) for private vulnerability reports.
