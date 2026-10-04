# Measured release performance

Rust/C++ time ratios; each value is the family geometric mean of case median paired ratios, followed by the maximum case in parentheses.

| Function | Fat LTO (historical lane 3b) | Moss thin LTO | Cargo release defaults |
|---|---:|---:|---:|
| vertex_cache | 1.123 (1.201) | 1.184 (1.330) | 1.231 (1.331) |
| overdraw | 1.098 (1.417) | 1.101 (1.522) | 1.135 (1.471) |
| simplify | 1.212 (1.385) | 1.049 (1.304) | 1.060 (1.237) |
| simplify_with_attributes | 1.208 (1.342) | 1.072 (1.231) | 1.092 (1.315) |
| simplify_scale | 0.748 (0.884) | 0.915 (1.067) | 0.855 (1.010) |

RFC bars are unchanged: family geometric mean <= 1.25, maximum case <= 1.50, and requested output-plus-scratch memory <= 1.25 times C++.

Moss uses thin LTO, one codegen unit and opt-level 3. Cargo defaults use LTO disabled, 16 codegen units and opt-level 3. Both consumer records disable release debug. The crate retains its own fat-LTO profile; dependents do not inherit it.

The complete matrix has 204 allocating, caller-buffer and allocation-free cases, covering tiny, medium and million-triangle smooth, seam-heavy and sparse meshes. The single-thread resident drivers share one physical core, one warm-up each and 20 or 30 alternating pairs. Validation, required copies, allocation and scratch are timed.

These are shared-load measurements. Per-case timings, dispersion, memory, load telemetry and source/executable identities are in the external records. This table does not claim quiet-host performance or a universal speedup.

See results/benchmark.json, results/benchmark-moss.json and results/benchmark-default.json for per-function summaries and SHA-256 identities. report.sh --verify-artifacts verifies available data and identifies absent historical artifacts.

D146 final integration: the full stage-1 matrices retain their case maxima and family bars. Seven maxima received 30 fresh pinned-core pairs each; two passed the upper 95% confidence bound and five remain documented residuals (including three inconclusive). See results/stage2-0.1.json and results/requalification-0.1.json.

## 0.5 local benchmark verdict (not accepted)

Time ratio is Rust/scalar-strict C++ family geometric mean, with the largest representative case in parentheses. The unchanged RFC limits are 1.25 and 1.50. Each case has ten interleaved paired samples and exact output checks.

| Family and API | Moss-like | Cargo defaults |
|---|---:|---:|
| `stripify:allocating` | 1.346 (1.527) fail | 1.108 (1.363) pass |
| `stripify:caller` | 1.303 (1.712) fail | 1.266 (1.358) fail |
| `stripify_bound:allocating` | 2.803 (2.818) fail | 3.009 (3.498) fail |
| `unstripify:allocating` | 1.149 (1.472) pass | 1.154 (1.403) pass |
| `unstripify:caller` | 1.045 (1.184) pass | 1.105 (1.363) pass |
| `unstripify_bound:allocating` | 2.416 (2.419) fail | 3.287 (4.022) fail |
| `vertex_cache:allocating` | 1.817 (2.196) fail | 1.857 (1.916) fail |
| `vertex_fetch:allocating` | 1.432 (1.646) fail | 1.309 (1.372) fail |
| `overdraw:allocating` | 1.166 (1.269) pass | 1.770 (1.855) fail |
| `coverage:allocating` | 1.260 (1.322) fail | 1.725 (1.874) fail |
| `omm_measure:allocating` | 2.545 (3.128) fail | 2.152 (2.301) fail |
| `omm_rasterize:allocating` | 1.421 (1.636) fail | 1.706 (1.772) fail |
| `omm_rasterize:caller` | 1.434 (1.699) fail | 1.973 (2.155) fail |
| `omm_entry_size:allocating` | 1.877 (1.944) fail | 2.674 (2.717) fail |
| `omm_compact:allocating` | 1.390 (1.521) fail | 1.148 (1.189) pass |
| `tangents:allocating` | 2.838 (3.217) fail | 2.750 (2.991) fail |
| `tangents:caller` | 2.938 (3.252) fail | 2.869 (3.017) fail |
| `normals:allocating` | 3.530 (3.605) fail | 3.214 (3.277) fail |
| `normals:caller` | 3.450 (3.844) fail | 3.197 (3.364) fail |
| `remesh:allocating` | 1.613 (1.694) fail | 1.732 (1.833) fail |
| `remesh:caller` | 1.527 (1.602) fail | 1.528 (1.629) fail |

The time bar fails in 18/21 Moss-like and 17/21 Cargo-default groups. Comparable C++ peak output-plus-scratch storage and the RFC million-triangle/geometry-variety corpus are absent, so the overall benchmark gate remains failed even if an individual time row passes. Raw paired samples and binary hashes are in the external artifacts linked by `results/benchmark-*-0.5.json`.
