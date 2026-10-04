# Measured release performance

Rust/C++ time ratios; each value is the family geometric mean of case median paired ratios, followed by the maximum case in parentheses.

| Function | Fat LTO (historical lane 3b) | Moss thin LTO | Cargo release defaults |
|---|---:|---:|---:|
| vertex_cache | 1.123 (1.201) | 1.176 (1.344) | 1.240 (1.310) |
| overdraw | 1.098 (1.417) | 1.079 (1.422) | 1.063 (1.398) |
| simplify | 1.212 (1.385) | 1.111 (1.201) | 1.284 (1.420) |
| simplify_with_attributes | 1.208 (1.342) | 1.185 (1.335) | 1.257 (1.651) |
| simplify_scale | 0.748 (0.884) | 0.829 (1.292) | 1.030 (1.187) |

RFC bars are unchanged: family geometric mean <= 1.25, maximum case <= 1.50, and requested output-plus-scratch memory <= 1.25 times C++.

Moss uses thin LTO, one codegen unit and opt-level 3. Cargo defaults use LTO disabled, 16 codegen units and opt-level 3. Both consumer records disable release debug. The crate retains its own fat-LTO profile; dependents do not inherit it.

The complete matrix has 204 allocating, caller-buffer and allocation-free cases, covering tiny, medium and million-triangle smooth, seam-heavy and sparse meshes. The single-thread resident drivers share one physical core, one warm-up each and 20 or 30 alternating pairs. Validation, required copies, allocation and scratch are timed.

These are shared-load measurements. Per-case timings, dispersion, memory, load telemetry and source/executable identities are in the external records. This table does not claim quiet-host performance or a universal speedup.

See results/benchmark.json, results/benchmark-moss.json and results/benchmark-default.json for per-function summaries and SHA-256 identities. report.sh --verify-artifacts verifies available data and identifies absent historical artifacts.
