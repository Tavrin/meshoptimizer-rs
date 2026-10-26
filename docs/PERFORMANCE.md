# Performance

Timings of meshoptimizer-rs against C++ meshoptimizer 1.3 and the meshopt crate. This file holds
the detail that the README summarizes; the linked parity records hold the raw data.

## Against C++ 1.3

Recorded on an AMD Ryzen 9 7945HX, Linux x86-64, using a pinned core and interleaved pairs on a
shared host. Ratios are Rust time / C++ time: lower is faster, 1.00 is equal. Validation, required
copies and allocation are timed. The scalar baseline is pinned meshoptimizer 1.3 with SIMD and FMA
contraction disabled. The records below differ in source, corpus and API scope. For 0.2.1 and
0.2.2, only the encoders were timed again (see [Encoders in 0.2.2](#encoders-in-022)); no new
full-tree timing was run for those releases or for 0.2.3.

The Moss profile uses thin LTO and one codegen unit. The defaults profile uses Cargo's consumer
release settings: no LTO and sixteen codegen units. The crate-local fat-LTO profile does not apply
to dependents.

| Recorded scope | Moss ratio | Defaults ratio | Evidence |
|---|---:|---:|---|
| Core `simplify`, family geometric mean | 1.049 | 1.060 | [core records](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/MEASURED_PERFORMANCE.md) |
| Standard meshlet builder, mean / max | 1.126 / 1.324 | 1.148 / 1.405 | [P03](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/MEASURED_P03.json) |
| Scalar codecs and filters, historical family mean range | 0.61–1.24 | 0.59–1.24 | [P04 Moss](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/benchmark-0.4-moss.json), [defaults](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/benchmark-0.4-default.json) |
| Opacity-map compact, selected final scope mean | 0.557 | 0.578 | [P05 state](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/P05_CURRENT_STATE.md) |
| Cluster LOD, sum of per-mesh medians | 1.196 | 1.090 | [RFC 113 recovery](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/rfc113-clod-recover.md) |

### Simplify in 0.2.4

Median simplify time as a ratio of Rust to C++ 1.3 (scalar) on the three real meshes measured,
0.2.3 → 0.2.4: 214e 1.380 → 0.963, b6fd 1.298 → 0.947, a082 1.052 → 0.941. See the
[r2 measurements](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.4/parity/SIMPLIFY_LARGE_MESH_R2_PERFORMANCE.md).

### SIMD decoding

Current SIMD timings use a separate registered driver and SIMD/JS comparators. Native vertex decode
has allocating / caller-buffer means of 1.116 / 1.051; the resolved allocating maximum still fails.
Every current family mean and native caller-buffer family passes the registered 0.2 rule; the
allocating index, vertex and filtered-view maxima fail. The symmetric WASM JavaScript adapters pass
that rule, but that does not qualify the Vec-returning Rust WASM API. See the
[full qualification and A/A widths](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/P07_RELEASE_QUALIFICATION.md).

The general family mean / case maximum bars are 1.25 / 1.50. For 0.2 SIMD, maxima apply only when
both independent A/A upper bounds are at most 1.25; all rows still contribute to means. Unresolved
rows remain listed. Passing a family mean does not clear a failed or inconclusive maximum.

### Parallel batches

Parallel speed curves compare batches with sequential Rust, not C++. Small hierarchy batches can be
slower, including caller-side pool overhead. See the
[P06 record](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/p06/README.md).

## Against the meshopt crate

The 0.2.0 comparison from 2026-10-06 uses meshopt 0.6.2 (bundled C++ 0.25) and five frozen inputs
on the Ryzen 9 7945HX in two consumer profiles. meshopt is faster at allocating vertex decode and
Oct/Quat/Exp encoding by five-case geometric mean in both profiles; Exp encoding is about 3.4×
faster. meshoptimizer-rs is faster by the same measure at matched-v1 vertex encoding, Color
encoding, Exp/Color decoding and scan meshlet construction; individual cases vary.

meshoptimizer-rs has the 1.3 meshlet codecs, opacity maps, tangents, experimental
normals/remeshing, cluster LOD and built-in parallel batches, which meshopt 0.6.2 lacks. It also has
`no_std + alloc`, checked views, typed errors, per-call work/byte limits and reusable workspaces
without a C++ toolchain. Default vertex encodings differ (ours v1, theirs v0), and some meshlet,
partitioning and simplification outputs differ across upstream versions.

These shared-host timings do not isolate language, compiler, wrapper or version costs; see the
[full comparison, output checks and case intervals](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/COMPARE_MESHOPT_CRATE.md).
The 0.2.2 encoder run below measured meshopt 0.6.2 again in the same two consumer profiles. For
encoding, its results replace the encoder claims above.

## Encoders in 0.2.2

0.2.2 makes the allocating index, sequence and Quat encoders faster without changing their output.
Ratios are family geometric means of Rust time / comparator time with the allocating APIs,
0.2.1 → 0.2.2 for C++ 1.3 and 0.2.2 for meshopt 0.6.2; lower is faster.

| Encoder | C++ 1.3, defaults | C++ 1.3, Moss | meshopt 0.6.2, defaults | meshopt 0.6.2, Moss |
|---|---:|---:|---:|---:|
| Vertex | 0.666 → 0.667 | 0.660 → 0.657 | 0.710 | 0.733 |
| Index | 1.019 → 0.773 | 1.016 → 0.764 | 0.860 | 0.858 |
| Sequence | 0.931 → 0.924 | 0.986 → 0.940 | 0.980 | 0.952 |
| Oct | 0.591 → 0.585 | 0.565 → 0.563 | 0.845 | 0.868 |
| Quat | 1.055 → 0.700 | 0.681 → 0.669 | 0.868 | 0.868 |
| Exp | 0.609 → 0.621 | 0.588 → 0.590 | 0.575 | 0.579 |
| Color | — | — | 0.825 | 0.825 |

In both profiles every C++ family mean is at or below 1.25 and all 111 cases have a 95% upper
bound at or below 1.50. Encoded bytes are unchanged: 17,264 exact comparisons per profile against
scalar and SIMD meshoptimizer 1.3, with zero mismatches. The run used an AMD Ryzen 9 7945HX on
Linux, one pinned core, a generic CPU target and C++ built with GCC 13.3 using its normal runtime
SIMD dispatch, with interleaved Rust/C++ pairs and A/A controls. meshopt 0.6.2 bundles C++ 0.25, so
those ratios include version and wrapper differences. See the
[intervals, per-case results and method](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.2/parity/P03_ENCODER_PERFORMANCE.md).

## Known exceptions in detail

Overdraw optimization is about 1.47–1.60× C++ 1.3 (scalar) on the largest real meshes measured.
See the [r2 measurements](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.4/parity/SIMPLIFY_LARGE_MESH_R2_PERFORMANCE.md)
and the [v2 release check](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.4/parity/release-check-v2/REPORT.md).

The 0.2.0 exceptions below still applied at 0.2.4: 0.2.2 changed only encoders, 0.2.3 changed only
how `Workspace` handles retained scratch, and 0.2.4 changed only simplify and overdraw speed.

The owner's 2026-10-06 roadmap permitted the allocating vertex exception and scheduled its fix for
0.2.1, which stages large owned vertex decodes. The 0.2.2 run repeated the exact
`vertex-v1-streaming-s4` request in the defaults and Moss profiles and measured 1.000× C++ (95%
interval 0.850–1.206) and 1.058× (0.959–1.171). The historical gap did not reproduce, but that does
not show the staging fixed it. Use `decode_vertex_buffer_into` where possible: the owner records
about 1.04× for the caller-buffer form of that case. The 0.2.0 complete qualification also records
these results:

| Native allocating case | Final Rust/C++ 95% interval | Status |
|---|---:|---|
| `vertex-v1-streaming-s4` | 1.529–2.118 | Resolved performance failure; caller-buffer is the workaround. |
| `index-2-v0-streaming-s4` | 1.508–3.625 | Resolved maximum failure. |
| `index-2-v1-streaming-s4` | 2.189–2.465 | Resolved maximum failure. |
| `view-1-streaming-s4` | 1.343–1.636 | Resolved maximum fails the upper-bound rule. |
| `varied-view-3-streaming-s32` | 1.619–2.590 | Unresolved: Rust/C++ A/A upper bounds 1.284 / 1.026. |

The roadmap also names the filtered-view maximum near 1.64 and a small S3 case. The latest
independent S3 report records caller-buffer `varied-filter-3-resident-s32` at 1.026–1.124× scalar
Rust. The full report lists all excluded A/A rows and their interval widths; an excluded row has
not passed.

Older records retain a sparse allocating vertex-fetch maximum failure, inconclusive
overdraw/sloppy-simplification maxima, and Cargo-default `partition_clusters` mean / max of
1.450 / 1.778. P05's final selected compact/coverage/raster-overdraw scope passes; the full matrix
was not refreshed after those edits. The stricter SIMD amendment targets are left for 0.3.

See the [comparison's exception ledger](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/COMPARE_MESHOPT_CRATE.md#against-pinned-upstream-c-13).
