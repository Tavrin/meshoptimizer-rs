# Performance

The 0.2.17 tables compare instruction counts with meshoptimizer-rs 0.2.16 and
with C++ meshoptimizer 1.3 (via `meshopt` 0.7.0). Ratios are 0.2.17 / comparison,
aggregated as geometric means of per-input ratios. Values below 1 mean fewer
instructions. Both C++ comparisons are shown by name: scalar vs scalar C++ 1.3
and default vs default C++ 1.3. Instruction counts do not establish wall-time
speedups. This release did not measure optimesh; the 0.2.16 tag's copy of this
file has the optimesh 1.1.0 comparison.

The default build of `decode_vertex_buffer` selects its kernel at run time. The
measurement host reports AVX-512 VBMI/VBMI2, so the default column for that
function reflects the AVX-512 kernel. The scalar column does not.

## Real meshes

Geometric means over `real-214e`, `real-a082` and `real-b6fd`.

| Function | scalar vs 0.2.16 | default vs 0.2.16 | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 |
|---|---:|---:|---:|---:|
| `simplify` | 0.964 | 0.964 | 1.174 | 1.174 |
| `simplify_with_attributes` | 0.976 | 0.976 | 1.118 | 1.118 |
| `simplify_sloppy` | 1.000 | 1.000 | 1.284 | 1.284 |
| `simplify_with_update` | 0.969 | 0.969 | 1.238 | 1.238 |
| `simplify_prune` | 1.000 | 1.000 | 0.971 | 0.971 |
| `simplify_points` | 1.000 | 1.000 | 1.032 | 1.032 |
| `optimize_vertex_cache` | 0.566 | 0.566 | 0.706 | 0.706 |
| `optimize_overdraw` | 1.000 | 1.000 | 1.096 | 1.096 |
| `optimize_vertex_fetch` | 1.000 | 1.000 | 0.908 | 0.908 |
| `build_meshlets` | 1.000 | 1.000 | 1.005 | 1.005 |
| `build_meshlets_flex` | 1.000 | 1.000 | 1.005 | 1.005 |
| `build_meshlets_spatial` | 0.982 | 0.991 | 0.955 | 1.062 |
| cluster LOD | 1.000 | 1.000 | 1.224 | 1.223 |
| `encode_vertex_buffer` | 0.992 | 0.992 | 0.545 | 0.545 |
| `decode_vertex_buffer` | 0.934 | 0.849 | 1.128 | 0.979 |
| `encode_index_buffer` | 1.000 | 1.000 | 0.770 | 0.770 |
| `decode_index_buffer` | 1.000 | 0.999 | 0.994 | 1.024 |
| `remap_vertex_buffer` | 1.000 | 1.000 | 1.087 | 1.087 |
| `remap_index_buffer` | 1.000 | 1.000 | 1.354 | 1.354 |
| `compute_cluster_bounds` | 1.000 | 1.000 | 1.089 | 1.039 |
| `compute_meshlet_bounds` | 1.000 | 1.000 | 1.122 | 1.072 |

## Corpus

Geometric means over `tiny-smooth`, `medium-smooth`, `medium-seams`,
`medium-sparse` and `million-smooth`.

| Function | scalar vs 0.2.16 | default vs 0.2.16 | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 |
|---|---:|---:|---:|---:|
| `simplify` | 0.970 | 0.970 | 1.058 | 1.058 |
| `simplify_with_attributes` | 0.984 | 0.984 | 1.068 | 1.068 |
| `simplify_sloppy` | 1.000 | 1.000 | 1.281 | 1.281 |
| `simplify_with_update` | 0.974 | 0.974 | 1.076 | 1.076 |
| `simplify_prune` | 1.000 | 1.000 | 1.056 | 1.056 |
| `simplify_points` | 1.000 | 1.000 | 1.084 | 1.084 |
| `optimize_vertex_cache` | 0.845 | 0.845 | 1.048 | 1.048 |
| `optimize_overdraw` | 1.000 | 1.000 | 0.939 | 0.939 |
| `optimize_vertex_fetch` | 1.001 | 1.001 | 1.148 | 1.148 |
| `build_meshlets` | 1.000 | 1.000 | 1.057 | 1.057 |
| `build_meshlets_flex` | 1.000 | 1.000 | 1.058 | 1.058 |
| `build_meshlets_spatial` | 0.985 | 0.991 | 0.961 | 1.041 |
| cluster LOD | 1.000 | 1.000 | 1.172 | 1.167 |
| `encode_vertex_buffer` | 0.998 | 0.998 | 0.542 | 0.542 |
| `decode_vertex_buffer` | 0.945 | 0.898 | 1.171 | 1.124 |
| `encode_index_buffer` | 1.000 | 1.000 | 0.940 | 0.940 |
| `decode_index_buffer` | 0.998 | 0.993 | 1.042 | 1.084 |
| `remap_vertex_buffer` | 1.000 | 1.000 | 1.068 | 1.068 |
| `remap_index_buffer` | 1.001 | 1.001 | 1.314 | 1.314 |
| `compute_cluster_bounds` | 1.000 | 1.000 | 1.122 | 1.048 |
| `compute_meshlet_bounds` | 1.000 | 1.000 | 1.184 | 1.110 |

The campaign took 3,024 samples (median of three `instructions:u` samples per
case). All 168 base/train and 168 train/C++ output comparisons were equal in
both builds. No case was more than 1.01 times its v0.2.16 count; the largest
case ratio was 1.003783, for `encode_vertex_buffer`. The tables describe the
registered inputs and owned-call workloads.

The 0.2.16 release tables came from a separate campaign. Their C++ ratios are
not comparable with these: `remap_index_buffer` is unchanged in 0.2.17 and has a
ratio of 1.437 in the 0.2.16 tables and 1.354 here.

## wasm32 decoding

Node v22, wall-time medians of seven rotated rounds on eight inputs, the shipped
C++ wasm decoder as reference, allocating output. Ratios are geometric means of
the per-input ratios.

| Decoder | 0.2.17 / 0.2.16 | worst input | 0.2.17 / C++ 1.3 |
|---|---:|---:|---:|
| vertex | 0.92 | 0.97 | 1.04 |
| index | 0.98 | 1.02 | 1.11 |

The index decoder was slower than 0.2.16 on two inputs (`tiny-smooth` 1.013,
`real-214e` 1.022). The host was shared, so these are not instruction counts.
No aarch64 timing was taken: there was no arm64 host.

## Parallel cluster LOD wall times

Measured for 0.2.16 and not repeated for 0.2.17. Five interleaved AB/BA
repetitions per real mesh, with medians in seconds.
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
[recipe](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#parallel-groups-within-one-cluster-lod-mesh).
Parallel group execution requires unlimited logical work/storage limits and
border dilation disabled. Finite limits, including the defaults, select serial
execution. Logical usage matches serial, while concurrent scratch and Rayon
infrastructure add physical memory use. These timings do not guarantee a
speedup on other meshes or hosts.

## Parallel vertex codec and spatial meshlets for one large mesh

Median of three calls per cell, wall seconds, on the three real meshes, with
a Rayon pool of the stated size. Every parallel output was asserted equal to
the serial bytes (vertex buffers, meshlets, vertices, triangles). The host was
shared, with a load average of 6.5 to 7.9 on a 16-core, 32-thread CPU; threads
were not pinned and the 16-worker column shares cores with other work. These
timings do not guarantee a speedup on other meshes or hosts.

The vertex buffers are the mesh positions as three `f32` (stride 12) and as
four 16-bit values quantized to the bounding box (stride 8), encoded with the
default version 1, level 2. Meshlets are `build_meshlets_spatial` with 64
vertices, 124 triangles, 64 minimum triangles and fill weight 0.5, on a
workspace with unlimited limits. The 1-worker column calls the parallel
function inside a one-worker pool, which takes the serial path.

| Mesh, operation | Serial, s | 1 | 2 | 4 | 8 | 16 |
|---|---:|---:|---:|---:|---:|---:|
| real-b6fd encode, stride 12 | 0.0438 | 1.02x | 2.15x | 3.90x | 6.42x | 8.32x |
| real-b6fd encode, stride 8 | 0.0253 | 1.01x | 1.72x | 3.47x | 4.61x | 6.89x |
| real-b6fd decode, stride 12 | 0.0161 | 1.11x | 1.65x | 1.66x | 2.08x | 2.11x |
| real-b6fd decode, stride 8 | 0.0048 | 0.53x | 0.65x | 0.81x | 0.86x | 0.91x |
| real-b6fd spatial meshlets | 0.4988 | 0.95x | 1.71x | 2.90x | 3.35x | 3.83x |
| real-214e encode, stride 12 | 0.1558 | 0.96x | 1.60x | 2.96x | 5.35x | 7.04x |
| real-214e encode, stride 8 | 0.1203 | 1.02x | 1.84x | 4.34x | 7.31x | 10.11x |
| real-214e decode, stride 12 | 0.0559 | 1.00x | 1.12x | 1.49x | 1.83x | 1.83x |
| real-214e decode, stride 8 | 0.0368 | 1.04x | 0.99x | 1.42x | 1.64x | 1.72x |
| real-214e spatial meshlets | 2.3155 | 1.03x | 1.70x | 2.64x | 3.19x | 3.20x |
| real-a082 encode, stride 12 | 0.1725 | 1.03x | 1.76x | 4.02x | 6.81x | 10.77x |
| real-a082 encode, stride 8 | 0.1276 | 0.99x | 2.09x | 4.09x | 7.87x | 9.85x |
| real-a082 decode, stride 12 | 0.0607 | 1.00x | 0.97x | 1.27x | 1.45x | 1.56x |
| real-a082 decode, stride 8 | 0.0400 | 0.99x | 0.97x | 1.30x | 1.50x | 1.59x |
| real-a082 spatial meshlets | 2.5794 | 1.01x | 1.67x | 2.63x | 3.36x | 3.45x |

Decoding takes the serial path, so it is never slower, unless the pool has at
least four workers and the output is at least 48 MiB (32 MiB for strides of 8
or less). The decode rows above were measured before that rule. The smallest
mesh's decode (21.7 MB at stride 12, 14.5 MB at stride 8) and every 2-worker
cell now run serially, because a re-measure with warm allocator state showed
the 21.7 MB stride-12 decode at 0.84x to 0.90x with 4 to 16 workers. The
remaining parallel cells are the 54 to 87 MB outputs of real-214e and real-a082
with 4 or more workers, re-measured as 1.15x to 1.78x (median of five). Cells
below 4 workers run the serial path, so their 0.84x to 1.03x spread is
measurement noise.

Decoding scales least. Each group's size depends on its escape codes, so the
block starts come from a serial scan of every group header, which costs about
a third of a serial decode and bounds the speedup near three; after the scan
the blocks decode concurrently. A buffer the serial decoder already handles at
several gigabytes per second (the stride-8 buffer of the smallest mesh) is
faster serially; the 0.53x cell in that row runs the serial path in a pool of
one and shows the allocator and page-fault variation of a 5 ms call. Meshlets are bounded by the serial top of the cut recursion
and the serial emission scan. Encoding is almost entirely concurrent.

## Costs of the 0.2.17 APIs

These APIs have no C++ 1.3 counterpart, so the figures compare each with the
calls a caller would otherwise make.

Streaming decoders against the one-shot decoders, instructions per decode, on a
65,536-vertex stride-32 buffer and a 390,150-index grid
(`examples/buffer_stream_bench.rs`), median of three samples with setup
subtracted. The chunk size is the input chunk.

| Decoder | One-shot, millions | 4 KiB chunks | 64 KiB chunks | 4 KiB / one-shot | 64 KiB / one-shot |
|---|---:|---:|---:|---:|---:|
| vertex, version 0 | 7.452 | 9.798 | 7.719 | 1.315 | 1.036 |
| vertex, version 1 | 5.183 | 6.549 | 5.393 | 1.264 | 1.041 |
| triangles, version 0 | 7.234 | 8.346 | 8.344 | 1.154 | 1.153 |
| triangles, version 1 | 6.974 | 7.172 | 7.172 | 1.028 | 1.028 |

`process_mesh` against the same steps called by hand in the same order, in
instructions: 1.0016 to 1.0095 per input, geometric mean 1.003, on the eight
registered inputs.

`remove_hidden_triangles`, median of three wall seconds, on three scene meshes:

| Mesh | Input triangles | Output triangles | Seconds |
|---|---:|---:|---:|
| breakfast_room | 269,764 | 155,018 | 1.83 |
| living_room | 580,637 | 576,956 | 0.73 |
| conference | 124,619 | 98,911 | 0.47 |

`build_character_lod_chain` with `CharacterOptions::best_of` took 1.99 times as
long as the single character pass (range 1.81 to 2.20 over 39 model and target
pairs on 12 characters), because it builds both candidates for each level.

## Repair and streaming costs

Measured for 0.2.16. Corpus only, default build, median of three `instructions:u` samples.
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
case and CPU-only execution on four physical cores. Host and toolchain
identities are in the campaign's
[environment record](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/parity/0217-qualify/environment.json).
The host was an AMD Ryzen 9 7945HX running Linux x86-64 (kernel 7.0.0-42),
with rustc 1.99.1 and GCC 13.3.0 for the C++ arms. No GPU execution
contributes to these measurements.

Scalar and default builds are compared separately against the corresponding
v0.2.16 and C++ 1.3 builds. The C++ reference is commit
`4c203430ca565cb59a468a91922c76c208169536`. The method follows the 0.2.16
campaign: interleaved repetitions and owned calls, allocation and output
destruction counted, the middle repetition reversing arm order, release thin
LTO and one codegen unit without native-CPU flags. Input loading, remap setup,
warmup and serialization are outside the counters. Logical work/storage limits
are unlimited in all arms; the physical admission memory cap still applies.

The numerical sources are the [release table](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/parity/0217-qualify/TABLE.md)
and [qualification report](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/parity/0217-qualify/REPORT.md).
Per-case medians are in
[release-results.json](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/parity/0217-qualify/release-results.json)
and raw counts in `counts.jsonl` next to it.

## Scope and C++ differences

These measurements cover the listed x86-64 inputs and API forms. They do not
establish performance on other architectures, exhaustive public-input
coverage or per-case wall-time bounds. Arm64 correctness is supported by the
source-identical Linux and Apple CI receipts linked from the report; these
x86-64 measurements do not measure NEON performance, and no aarch64 timing was
taken for 0.2.17.

Successful vertex-decoder bytes and status match the recorded C++ checks.
Malformed streams can leave different failed destination prefixes; those
prefixes are not claimed byte-identical. C++ NEON ISA divergences are disclosed
separately in the report. Repair APIs have no corresponding C++ 1.3 entry
points.

AVX-512 VBMI/VBMI2 is implemented for vertex decoding only. AVX2 and 32-bit ARM
SIMD are not implemented; index and sequence decoding remain scalar. Fuzzing is
reported separately in
[parity/0217-fuzz/RECEIPTS.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/parity/0217-fuzz/RECEIPTS.md):
18 targets, 62,963 CPU seconds, no crashes and no mismatches. The concurrent path
of `decode_vertex_buffer_parallel` is not reachable by the fuzz targets.
