# Measured release performance

Rust/C++ time ratios; each value is the family geometric mean of case median paired ratios, followed by the maximum case in parentheses.

| Function | Fat LTO (historical lane 3b) | Moss thin LTO | Cargo release defaults |
|---|---:|---:|---:|
| vertex_cache | 1.123 (1.201) | 1.176 (1.344) | 1.189 (1.257) |
| overdraw | 1.098 (1.417) | 1.079 (1.422) | 1.048 (1.364) |
| simplify | 1.212 (1.385) | 1.111 (1.201) | 1.180 (1.306) |
| simplify_with_attributes | 1.208 (1.342) | 1.185 (1.335) | 1.217 (1.310) |
| simplify_scale | 0.748 (0.884) | 0.829 (1.292) | 0.963 (1.142) |

RFC bars are unchanged: family geometric mean <= 1.25, maximum case <= 1.50, and requested output-plus-scratch memory <= 1.25 times C++.

Moss uses thin LTO, one codegen unit and opt-level 3. Cargo defaults use LTO disabled, 16 codegen units and opt-level 3. Both consumer records disable release debug. The crate retains its own fat-LTO profile; dependents do not inherit it.

The complete matrix has 204 allocating, caller-buffer and allocation-free cases, covering tiny, medium and million-triangle smooth, seam-heavy and sparse meshes. The single-thread resident drivers share one physical core, one warm-up each and 20 or 30 alternating pairs. Validation, required copies, allocation and scratch are timed.

These are shared-load measurements. Per-case timings, dispersion, memory, load telemetry and source/executable identities are in the external records. This table does not claim quiet-host performance or a universal speedup.

See results/benchmark.json, results/benchmark-moss.json and results/benchmark-default.json for per-function summaries and SHA-256 identities. report.sh --verify-artifacts verifies available data and identifies absent historical artifacts.

## Phase 0.3 — complete 624-row matrix (2026-10-04)

**FAIL: timing acceptance.** Every requested-heap ratio passes. Each baseline
has eight failed timing families; nine distinct families fail at least one
baseline. All 624 scalar comparisons match exact meaningful output.

One warm-up per framed measurement, at least ten alternating same-core pairs,
increased to twenty/thirty when quartiles cross a bar. The minimum measured
batch is eight milliseconds, capped at two million repeats. CPU 0, siblings
0–1, AMD Ryzen 9 7945HX. Rust release fat LTO and scalar-strict C++ use comparable
baseline ISA availability; the second C++ binary enables upstream SIMD.
Validation, required copies, allocation and operation are timed. Generation,
I/O, serialization and process startup are excluded. Requested heap includes
owned output and live scratch; caller destinations, fixed stack and allocator
overhead are excluded on both sides. Raw times, paired ratios, dispersion,
per-pair load, requested storage and input/output hashes are retained.

| Function | Scalar GM | Scalar max | Scalar | SIMD GM | SIMD max | SIMD | Max heap ratio |
|---|---:|---:|---|---:|---:|---|---:|
| build_meshlets | 1.598 | 2.025 | FAIL | 1.575 | 1.987 | FAIL | 1.071 |
| build_meshlets_scan | 1.110 | 1.219 | PASS | 1.112 | 1.220 | PASS | 1.000 |
| build_meshlets_flex | 1.581 | 2.112 | FAIL | 1.571 | 2.054 | FAIL | 1.071 |
| build_meshlets_spatial | 1.264 | 1.431 | FAIL | 1.452 | 1.767 | FAIL | 1.167 |
| build_meshlets_bound | 1.810 | 1.973 | FAIL | 1.911 | 2.099 | FAIL | 1.000 |
| compute_cluster_bounds | 1.265 | 1.903 | FAIL | 1.310 | 1.646 | FAIL | 1.000 |
| compute_meshlet_bounds | 1.304 | 1.645 | FAIL | 1.297 | 1.740 | FAIL | 1.000 |
| compute_sphere_bounds | 1.064 | 1.345 | PASS | 1.106 | 1.735 | FAIL | 1.000 |
| optimize_meshlet | 1.077 | 1.420 | PASS | 1.046 | 1.415 | PASS | 1.000 |
| optimize_meshlet_level | 0.990 | 1.211 | PASS | 1.010 | 1.171 | PASS | 1.000 |
| extract_meshlet_indices | 1.310 | 1.817 | FAIL | 1.003 | 1.476 | PASS | 1.000 |
| partition_clusters | 1.362 | 1.652 | FAIL | 1.360 | 1.588 | FAIL | 1.000 |
| spatial_sort_remap | 1.152 | 1.354 | PASS | 1.140 | 1.396 | PASS | 1.125 |
| spatial_sort_triangles | 1.076 | 1.173 | PASS | 1.065 | 1.447 | PASS | 1.062 |
| spatial_cluster_points | 1.038 | 1.168 | PASS | 1.080 | 1.183 | PASS | 0.969 |

Required: GM <=1.25, every representative case <=1.50, requested heap <=1.25.
The unchanged enforced benchmark exits 1. The method records shared-load
dispersion and does not assert quiet-machine qualification or a universal
safe-Rust performance ceiling. Code-level profiling remains diagnostic.

The optional demo’s subsequent u64 work-accounting correction affects the
32-bit work-total path. Rebuilding the final native driver produces the exact
same executable SHA-256 as this full benchmark. The benchmark’s prior source
snapshot and both changed source files are retained by hash; the final fixture,
WASM and fuzz records bind the corrected source. The verifier requires identical
final/benchmarked native binaries, as well as the complete matrix and all hashes.

Artifacts: `/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p03`.
`benchmark/record.json` contains both per-family verdicts and every raw row;
`verification.json` independently recomputes the bars. The separate
`MEASURED_P03.json` summary preserves earlier phase records.

The next acceptance work is to reduce the nine failed families to the existing
bars, prioritizing candidate visits, bounds/extraction setup and the checked
bound-helper measurement. Neither thresholds nor canonical equality are relaxed.

## Phase 0.3 performance follow-up (2026-10-04; D78–D82)

**FAIL: Cargo-default partition timing.** The coordinator selected scalar C++
(`MESHOPTIMIZER_NO_SIMD`) as the 0.3 acceptance baseline, matching 0.2. The
upstream SIMD ratios are published below, not gated; clusterizer SIMD needs a
separate later amendment. The unchanged RFC §6.1 limits are family geometric
mean ≤1.25, every representative case ≤1.50, and requested output plus
scratch ≤1.25. No limit was relaxed.

Both final enforced profiles have 624 complete rows, at least twenty
interleaved same-core pairs per case (thirty near a bar), raw samples and load
in their records, exact scalar outputs and unchanged source snapshots. The
Moss-like consumer build uses thin LTO and one codegen unit; the second uses
Cargo release defaults. Consumer ran on CPU 17 (sibling 16), defaults on CPU
10 (sibling 11). Their start/end one-minute loads were 23.4/21.7 and
23.4/24.4 respectively; per-pair load and dispersion remain in the JSON.
Consumer enforcement exited 0. Defaults enforcement exited 1.

Scalar C++ timing below is geometric mean / maximum. “Before” is D77's full
fat-LTO matrix and therefore a historical build profile, not a paired
same-profile speedup. The partial earlier profile-specific baselines are in
`perf/before`; the complete final results are in `perf/final4`.

| Family | Before | Consumer | Cargo defaults | Max heap | Scalar verdict |
|---|---:|---:|---:|---:|---|
| build_meshlets | 1.598/2.025 | 1.126/1.324 | 1.148/1.405 | 1.071 | PASS/PASS |
| build_meshlets_scan | 1.110/1.219 | 1.076/1.153 | 1.139/1.215 | 1.000 | PASS/PASS |
| build_meshlets_flex | 1.581/2.112 | 1.107/1.371 | 1.157/1.423 | 1.071 | PASS/PASS |
| build_meshlets_spatial | 1.264/1.431 | 1.077/1.284 | 1.043/1.242 | 1.167 | PASS/PASS |
| build_meshlets_bound | 1.810/1.973 | 0.644/0.657 | 0.647/0.665 | 1.000 | PASS/PASS |
| compute_cluster_bounds | 1.265/1.903 | 1.133/1.290 | 1.084/1.187 | 1.000 | PASS/PASS |
| compute_meshlet_bounds | 1.304/1.645 | 1.168/1.299 | 1.178/1.328 | 1.000 | PASS/PASS |
| compute_sphere_bounds | 1.064/1.345 | 1.006/1.230 | 1.076/1.360 | 1.000 | PASS/PASS |
| optimize_meshlet | 1.077/1.420 | 0.953/1.365 | 0.916/1.209 | 1.000 | PASS/PASS |
| optimize_meshlet_level | 0.990/1.211 | 0.897/1.002 | 0.922/1.051 | 1.000 | PASS/PASS |
| extract_meshlet_indices | 1.310/1.817 | 0.977/1.194 | 0.938/1.334 | 1.000 | PASS/PASS |
| partition_clusters | 1.362/1.652 | 1.229/1.367 | **1.450/1.778** | 1.000 | PASS/FAIL |
| spatial_sort_remap | 1.152/1.354 | 1.031/1.427 | 0.990/1.302 | 1.125 | PASS/PASS |
| spatial_sort_triangles | 1.076/1.173 | 0.993/1.316 | 0.896/1.275 | 1.062 | PASS/PASS |
| spatial_cluster_points | 1.038/1.168 | 1.113/1.212 | 1.159/1.374 | 0.969 | PASS/PASS |

The separate upstream SIMD C++ comparison is also geometric mean / maximum:

| Family | Consumer vs SIMD | Defaults vs SIMD |
|---|---:|---:|
| build_meshlets | 1.121/1.333 | 1.143/1.408 |
| build_meshlets_scan | 1.082/1.185 | 1.144/1.209 |
| build_meshlets_flex | 1.106/1.357 | 1.152/1.426 |
| build_meshlets_spatial | 1.225/1.616 | 1.195/1.706 |
| build_meshlets_bound | 0.643/0.649 | 0.686/1.111 |
| compute_cluster_bounds | 1.141/1.287 | 1.115/1.257 |
| compute_meshlet_bounds | 1.171/1.292 | 1.184/1.310 |
| compute_sphere_bounds | 1.017/1.229 | 1.091/1.348 |
| optimize_meshlet | 0.956/1.355 | 0.911/1.226 |
| optimize_meshlet_level | 0.901/1.009 | 0.932/1.124 |
| extract_meshlet_indices | 1.139/1.392 | 1.140/1.480 |
| partition_clusters | 1.223/1.336 | 1.449/1.598 |
| spatial_sort_remap | 1.020/1.372 | 0.975/1.363 |
| spatial_sort_triangles | 0.999/1.346 | 0.922/1.236 |
| spatial_cluster_points | 1.103/1.207 | 1.173/1.409 |

The default-profile partition residual is not a heap failure. The worst row
is `medium-seams-into` (paired median 1.778, ratio CV 0.129); the complete
family's best final result is 1.450/1.778. On that same framed request, the
exact benchmark binaries retired 6.571G Rust versus 4.974G C++ instructions,
took 1.949G versus 1.133G cycles, and executed 1.577G versus 0.862G
branches. Tiny seams likewise used 7.934G versus 5.953G instructions and
2.076G versus 1.317G cycles. `perf record` lost zero samples: the exact Rust
binary attributes 46.3% of cycles to `adjacent` and 42.8% to `partition`. A
line-table build of the same source finds 15.1% at centroid accumulation,
8.9% at adjacency deduplication and 5.9% at initial vertex filtering. The
sampling and counter files, framed requests and binary hashes are retained
under `/mnt/linux-extra/meshopt-artifacts/p03/perf/final4`. This evidence
supports a broad default-profile residual, not a universal safe-Rust limit.

On the final source, fmt, both clippy modes with `-D warnings`, both test
modes including Rust 1.88, and the no-default wasm32 build exited 0. The 0.3
fixture/WASM run, 2,000-case sweep per family and 80-case `clusterlod` run
also exited 0 with zero mismatches. The exhaustive math check from D79 is
retained separately. The final benchmark records, gate logs and profile
evidence are under `/mnt/linux-extra/meshopt-artifacts/p03/perf/final4`;
`MEASURED_P03.json` indexes their hashes and verdicts. The build target was
deleted after measurement. All 2,496 generated benchmark input/output files
were checked against the row hashes before removal; the lean final artifact
folder retains the two complete records, gate/corpus evidence and profiles.
