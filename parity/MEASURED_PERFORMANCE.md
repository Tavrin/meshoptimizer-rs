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
