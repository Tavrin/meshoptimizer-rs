# Measured geometry performance

Single thread, 1,000,000 triangles; medians of ten interleaved pairs after warm-up.

Validation, output and scratch allocation and execution are timed; generation, I/O and process startup are excluded.

| Function | Rust (ms) | C++ (ms) | Rust/C++ |
|---|---:|---:|---:|
| vertex_cache | 163.656 | 124.018 | 1.320 |
| overdraw | 126.163 | 52.431 | 2.406 |
| simplify_scale | 5.204 | 3.901 | 1.334 |
| simplify-ratio-0.5 | 981.503 | 620.221 | 1.583 |
| simplify-ratio-0.25 | 1009.012 | 649.619 | 1.553 |
| simplify-ratio-0.125 | 991.972 | 592.293 | 1.675 |
| simplify_with_attributes-ratio-0.5 | 1378.986 | 924.551 | 1.492 |
| simplify_with_attributes-ratio-0.25 | 1438.122 | 980.245 | 1.467 |
| simplify_with_attributes-ratio-0.125 | 1225.238 | 699.940 | 1.750 |

Allocating APIs against scalar-strict C++. These modules have no explicit upstream SIMD paths. Raw samples, dispersion, hardware and executable identities are in results/benchmark.json.

Recorded only: this lane does not enforce the RFC's later release performance bars or qualify all workload sizes and API variants.
