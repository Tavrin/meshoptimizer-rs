# Performance

The 0.2.16 tables compare instruction counts with meshoptimizer-rs 0.2.15,
optimesh 1.1.0 and C++ meshoptimizer 1.3 (via `meshopt` 0.7.0). Ratios are
0.2.16 / comparison, aggregated as geometric means of per-mesh ratios.
Values below 1 mean fewer instructions. Both comparisons are shown by name:
scalar vs scalar C++ 1.3 and default vs default C++ 1.3. optimesh is scalar
in both comparisons. Instruction counts do not establish wall-time speedups.

## Real meshes

Geometric means over `real-214e`, `real-a082` and `real-b6fd`.

| Function | scalar vs 0.2.15 | default vs 0.2.15 | scalar vs optimesh 1.1.0 | default vs optimesh 1.1.0 | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 |
|---|---:|---:|---:|---:|---:|---:|
| `simplify` | 1.001 | 1.001 | 0.899 | 0.899 | 1.138 | 1.138 |
| `simplify_with_attributes` | 1.001 | 1.001 | 0.927 | 0.927 | 1.096 | 1.096 |
| `simplify_sloppy` | 1.000 | 1.000 | 0.706 | 0.706 | 1.275 | 1.275 |
| `simplify_with_update` | 0.940 | 0.940 | 0.947 | 0.947 | 1.203 | 1.203 |
| `simplify_prune` | 1.000 | 1.000 | 0.763 | 0.763 | 0.995 | 0.995 |
| `simplify_points` | 1.000 | 1.000 | 0.632 | 0.632 | 1.014 | 1.014 |
| `optimize_vertex_cache` | 1.000 | 1.000 | 0.919 | 0.919 | 1.254 | 1.254 |
| `optimize_overdraw` | 1.000 | 1.000 | 0.884 | 0.884 | 1.090 | 1.090 |
| `optimize_vertex_fetch` | 1.000 | 1.000 | 0.716 | 0.716 | 0.952 | 0.952 |
| `build_meshlets` | 1.000 | 1.000 | 0.784 | 0.784 | 1.001 | 1.001 |
| `build_meshlets_flex` | 1.000 | 1.000 | 0.784 | 0.784 | 1.002 | 1.002 |
| `build_meshlets_spatial` | 1.000 | 0.937 | 0.835 | 0.783 | 0.965 | 1.083 |
| cluster LOD | 1.000 | 0.999 | — | — | 1.189 | 1.188 |
| `encode_vertex_buffer` | 1.000 | 1.000 | 0.455 | 0.455 | 0.554 | 0.554 |
| `decode_vertex_buffer` | 1.000 | 0.911 | 0.544 | 0.174 | 1.061 | 1.145 |
| `encode_index_buffer` | 1.000 | 1.000 | 0.712 | 0.712 | 0.771 | 0.771 |
| `decode_index_buffer` | 1.000 | 1.000 | 0.907 | 0.934 | 0.994 | 1.024 |
| `remap_vertex_buffer` | 0.779 | 0.779 | 0.353 | 0.353 | 1.087 | 1.087 |
| `remap_index_buffer` | 0.885 | 0.885 | 0.863 | 0.863 | 1.437 | 1.437 |
| `compute_cluster_bounds` | 0.981 | 0.932 | 0.734 | 0.697 | 1.138 | 1.081 |
| `compute_meshlet_bounds` | 0.982 | 0.933 | 0.731 | 0.695 | 1.162 | 1.104 |

## Corpus

Geometric means over `tiny-smooth`, `medium-smooth`, `medium-seams`,
`medium-sparse` and `million-smooth`.

| Function | scalar vs 0.2.15 | default vs 0.2.15 | scalar vs optimesh 1.1.0 | default vs optimesh 1.1.0 | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 |
|---|---:|---:|---:|---:|---:|---:|
| `simplify` | 1.001 | 1.001 | 0.921 | 0.921 | 1.048 | 1.048 |
| `simplify_with_attributes` | 1.001 | 1.001 | 0.918 | 0.918 | 1.066 | 1.066 |
| `simplify_sloppy` | 1.000 | 1.000 | 0.882 | 0.882 | 1.268 | 1.268 |
| `simplify_with_update` | 0.986 | 0.986 | 0.959 | 0.959 | 1.070 | 1.070 |
| `simplify_prune` | 1.000 | 1.000 | 0.852 | 0.852 | 1.071 | 1.071 |
| `simplify_points` | 1.000 | 1.000 | 0.665 | 0.665 | 1.066 | 1.066 |
| `optimize_vertex_cache` | 1.000 | 1.000 | 0.911 | 0.911 | 1.242 | 1.242 |
| `optimize_overdraw` | 1.000 | 1.000 | 0.865 | 0.865 | 0.939 | 0.939 |
| `optimize_vertex_fetch` | 1.000 | 1.000 | 0.962 | 0.962 | 1.171 | 1.171 |
| `build_meshlets` | 1.000 | 1.000 | 0.778 | 0.778 | 1.068 | 1.068 |
| `build_meshlets_flex` | 1.000 | 1.000 | 0.778 | 0.778 | 1.068 | 1.068 |
| `build_meshlets_spatial` | 1.000 | 0.952 | 0.872 | 0.830 | 0.972 | 1.059 |
| cluster LOD | 0.997 | 0.993 | — | — | 1.172 | 1.167 |
| `encode_vertex_buffer` | 1.000 | 1.000 | 0.466 | 0.466 | 0.578 | 0.578 |
| `decode_vertex_buffer` | 1.000 | 0.880 | 0.644 | 0.200 | 1.114 | 1.237 |
| `encode_index_buffer` | 1.000 | 1.000 | 0.753 | 0.753 | 0.933 | 0.933 |
| `decode_index_buffer` | 1.000 | 1.000 | 0.915 | 0.951 | 1.044 | 1.085 |
| `remap_vertex_buffer` | 0.806 | 0.806 | 0.416 | 0.416 | 1.069 | 1.069 |
| `remap_index_buffer` | 0.883 | 0.883 | 0.871 | 0.871 | 1.386 | 1.386 |
| `compute_cluster_bounds` | 0.950 | 0.889 | 0.801 | 0.750 | 1.146 | 1.072 |
| `compute_meshlet_bounds` | 0.951 | 0.892 | 0.811 | 0.760 | 1.189 | 1.114 |

Cluster LOD has no optimesh comparison. All 336 base/train and 336 matched-mode
train/C++ output comparisons were equal. Across 3504 campaign samples, no
per-case median instruction ratio versus 0.2.15 exceeded 1.01; the maximum was
1.001949930 for `real-214e` / `simplify` / scalar. The tables describe the
registered inputs and owned-call workloads.

## Parallel cluster LOD wall times

Five interleaved AB/BA repetitions per real mesh, with medians in seconds.
One and four workers use one and four distinct physical CPU cores,
respectively. The clock surrounds the owned call, including output destruction,
and excludes loading and serialization. All 30 serialized callback outputs
per mesh match each other and default serial cluster LOD. These are CPU-only
measurements; no GPU is used. Four-worker speedups are 1.605–1.780×.

| Input | 1 worker, seconds | 4 workers, seconds | Speedup |
|---|---:|---:|---:|
| real-214e | 8.414459 | 5.243663 | 1.605x |
| real-a082 | 9.100415 | 5.417363 | 1.680x |
| real-b6fd | 2.399846 | 1.348241 | 1.780x |

Enable `parallel` and `clusterlod`; see the
[recipe](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#parallel-groups-within-one-cluster-lod-mesh).
Parallel group execution requires unlimited logical work/storage limits and
border dilation disabled. Finite limits, including the defaults, select serial
execution. Logical usage matches serial, while concurrent scratch and Rayon
infrastructure add physical memory use. These timings do not guarantee a
speedup on other meshes or hosts.

## Repair and streaming costs

Corpus only, default build, median of three `instructions:u` samples.
Repair ratios compare distinct calls with `sanitize` using `SanitizeOptions::EMPTY`
on the same raw mesh: positions-only zero-tolerance welding, face orientation,
and filling with a 16-edge maximum. Streaming decodes version-1 index sequences
in 4096-byte chunks into owned `u32` output; one-shot produces owned packed
32-bit bytes. Setup asserts equal decoded bytes. Allocation and destruction
are counted; encoding, loading, output validation and serialization are outside
the counters. These calls provide different functionality.

| Corpus input | weld / sanitize | orient / sanitize | fill / sanitize | streaming / one-shot |
|---|---:|---:|---:|---:|
| tiny-smooth | 1.328 | 3.321 | 8.646 | 2.521 |
| medium-smooth | 0.886 | 3.589 | 5.674 | 2.813 |
| medium-seams | 1.066 | 3.612 | 6.194 | 2.813 |
| medium-sparse | 89.939 | 3.589 | 5.675 | 2.813 |
| million-smooth | 0.860 | 3.776 | 5.755 | 2.823 |
| Corpus GM | 2.497 | 3.574 | 6.300 | 2.754 |

## Method and host

The qualification report records median-of-three `instructions:u` samples per
case and CPU-only execution. Host and toolchain identities are retained in the
campaign's [environment record](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/parity/0216-qualify/combined-ff38162/environment.json).
The host was an AMD Ryzen 9 7945HX running Linux x86-64.
No GPU execution contributes to these measurements.

Scalar and default builds are compared separately against the corresponding
0.2.15 and C++ 1.3 builds. The C++ reference is commit
`4c203430ca565cb59a468a91922c76c208169536`. The optimesh 1.1.0 comparison
uses its scalar build throughout. The campaign uses interleaved repetitions
and owned calls; allocation and output destruction are counted.
The middle repetition reverses arm order. Builds use release thin LTO and one
codegen unit without native-CPU flags. Input loading, remap setup, warmup and
serialization are outside the counters. Logical work/storage limits are
unlimited in all arms; the physical admission memory cap still applies.

The numerical sources are the [qualification table](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/parity/0216-qualify/TABLE.md)
and [qualification report](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/parity/0216-qualify/REPORT.md).
The retained [campaign registration](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/parity/0216-qualify/combined-ff38162/REGISTRATION.md)
documents arm ordering, release profiles, limits and counter boundaries.
The report links the per-case medians, raw wall samples and output bindings.

## Scope and C++ differences

These measurements cover the listed x86-64 inputs and API forms. They do not
establish performance on other architectures, exhaustive public-input
coverage or per-case wall-time bounds. Arm64 correctness is supported by the
source-identical Linux and Apple CI receipts linked from the report; these
x86-64 measurements do not measure NEON performance.

Successful vertex-decoder bytes and status match the recorded C++ checks.
Malformed streams can leave different failed destination prefixes; those
prefixes are not claimed byte-identical. C++ NEON ISA divergences are disclosed
separately in the report. Streaming reports header errors immediately and
charges work per consumed byte, which can change malformed-input error
precedence relative to one-shot decoding. Repair APIs have no corresponding
C++ 1.3 entry points.

AVX2, AVX-512 and 32-bit ARM SIMD are not implemented; index and sequence
decoding remain scalar. Fuzzing is a separate release gate; the combined
qualification report does not verify it.
