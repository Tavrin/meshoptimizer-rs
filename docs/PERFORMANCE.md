# Performance

The 0.2.15 tables report scalar instruction counts against meshoptimizer-rs
0.2.14, `meshopt` 0.7.0 (C++ meshoptimizer 1.3), and optimesh 1.1.0.
Ratios are 0.2.15 / comparison, aggregated as geometric means of per-mesh
ratios. A value below 1 means fewer instructions. These are instruction counts,
not elapsed times; they do not measure the default SIMD build.

## Real meshes

Geometric means over the three production LOD-chain meshes `real-214e`,
`real-a082` and `real-b6fd`, scalar vs scalar.

| Function | vs 0.2.14 | vs optimesh 1.1.0 | vs `meshopt` 0.7.0 (scalar C++ 1.3) |
|---|---:|---:|---:|
| `simplify` | 0.919 | 0.898 | 1.137 |
| `simplify_with_attributes` | 0.787 | 0.926 | 1.095 |
| `simplify_sloppy` | 0.845 | 0.706 | 1.275 |
| `simplify_with_update` | 0.936 | 1.008 | 1.280 |
| `simplify_prune` | 0.620 | 0.763 | 0.995 |
| `simplify_points` | 0.601 | 0.632 | 1.014 |
| `optimize_vertex_cache` | 0.955 | 0.919 | 1.254 |
| `optimize_overdraw` | 0.753 | 0.884 | 1.090 |
| `optimize_vertex_fetch` | 1.000 | 0.716 | 0.952 |
| `build_meshlets` | 0.826 | 0.784 | 1.001 |
| `build_meshlets_flex` | 0.826 | 0.784 | 1.002 |
| `build_meshlets_spatial` | 0.876 | 0.835 | 0.965 |
| cluster LOD | 0.909 | — | 1.189 |
| `encode_vertex_buffer` | 1.000 | 0.455 | 0.554 |
| `decode_vertex_buffer` | 1.000 | 0.544 | 1.061 |
| `encode_index_buffer` | 1.000 | 0.712 | 0.771 |
| `decode_index_buffer` | 1.000 | 0.907 | 0.994 |
| `remap_vertex_buffer` | 0.465 | 0.453 | 1.394 |
| `remap_index_buffer` | 0.772 | 0.975 | 1.625 |

## Corpus

Geometric means over `tiny-smooth`, `medium-smooth`, `medium-seams`,
`medium-sparse` and `million-smooth`, scalar vs scalar.

| Function | vs 0.2.14 | vs optimesh 1.1.0 | vs `meshopt` 0.7.0 (scalar C++ 1.3) |
|---|---:|---:|---:|
| `simplify` | 0.984 | 0.920 | 1.047 |
| `simplify_with_attributes` | 0.951 | 0.917 | 1.065 |
| `simplify_sloppy` | 0.916 | 0.882 | 1.268 |
| `simplify_with_update` | 0.987 | 0.972 | 1.085 |
| `simplify_prune` | 0.711 | 0.852 | 1.071 |
| `simplify_points` | 0.686 | 0.665 | 1.066 |
| `optimize_vertex_cache` | 0.943 | 0.911 | 1.242 |
| `optimize_overdraw` | 0.946 | 0.865 | 0.939 |
| `optimize_vertex_fetch` | 1.000 | 0.962 | 1.171 |
| `build_meshlets` | 0.975 | 0.778 | 1.068 |
| `build_meshlets_flex` | 0.975 | 0.778 | 1.068 |
| `build_meshlets_spatial` | 0.906 | 0.872 | 0.972 |
| cluster LOD | 0.973 | — | 1.176 |
| `encode_vertex_buffer` | 1.000 | 0.466 | 0.578 |
| `decode_vertex_buffer` | 1.000 | 0.644 | 1.114 |
| `encode_index_buffer` | 1.000 | 0.752 | 0.933 |
| `decode_index_buffer` | 1.000 | 0.915 | 1.044 |
| `remap_vertex_buffer` | 0.521 | 0.515 | 1.328 |
| `remap_index_buffer` | 0.791 | 0.986 | 1.570 |

Cluster LOD has no optimesh comparison. Every measured output matches 0.2.14
and C++ 1.3, and matches optimesh wherever that comparison is available.

## Method and host

The host was an AMD Ryzen 9 7945HX running Linux x86-64. The campaigns used
Rust 1.98.1 (LLVM 22.1.8), GCC 13.3 and perf 6.17.13, with resident-loop
`instructions:u` counts on pinned CPU 9. Arms were interleaved across three
repetitions, reversing arm order on the second repetition; each case uses the
median instruction count per call before ratios and geometric means are computed.
Every accepted counter sample had 100% counter running time.

All arms were scalar: meshoptimizer-rs and optimesh had `simd` disabled, and
C++ 1.3 was built with `MESHOPTIMIZER_NO_SIMD`. The Rust consumer release profile
used thin LTO and one codegen unit. This profile is distinct from Cargo's default
consumer profile. No default-build speed comparison is claimed.

The sources for all numerical ratios are the merged
[qualification table](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.15/parity/0215-qualify/TABLE.md)
and [qualification report](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.15/parity/0215-qualify/REPORT.md),
with the `optimize_vertex_cache` row replaced by the later
[cache restoration qualification](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.15/parity/0215-cache-restore/REPORT.md).
The cache campaign used the same eight inputs and scalar comparisons and
checked output identity for every arm and repetition. It changed a function's
inlining boundary, without changing its algorithm or output.

## Added API costs

Against `filter_index_buffer`, corpus geometric-mean instruction ratios are
`sanitize` 2.33×, with position welding 9.03×, and with all options 9.07×;
`analyze_mesh` 14.8×, with position welding 17.3×. These APIs perform different
work from filtering and are optional; the qualification report documents the
comparison. Custom cache-table performance was not measured.

## Scope and planned work

These measurements cover the listed scalar functions and inputs. They do not
establish wall-time speed, SIMD performance, no_std performance, other
architectures, or exhaustive per-case limits. Earlier timing campaigns used
different versions, input sets and API forms; their results are retained in the
repository's parity records and are not substituted for these release ratios.

SIMD work is planned for a later 0.2.x release. AVX2, AVX-512 and 32-bit ARM SIMD
are not implemented; index and sequence decoding remain scalar.
