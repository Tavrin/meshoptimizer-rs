# Measured lane 1 performance

Single thread, 1,000,000 triangles; medians of ten interleaved pairs after warm-up.

Validation, output and scratch allocation and execution are timed; generation, I/O and process startup are excluded.

| Function | Rust (ms) | C++ (ms) | Rust/C++ |
|---|---:|---:|---:|
| vertex_cache | 107.572 | 71.255 | 1.510 |
| overdraw | 47.758 | 18.204 | 2.624 |

Allocating APIs against scalar-strict C++. These modules have no explicit upstream SIMD paths. Raw samples, dispersion, hardware and executable identities are in results/benchmark.json.

Recorded only: this lane does not enforce the RFC's later release performance bars or qualify all workload sizes and API variants.
