# Measured geometry parity

Exact output comparison with scalar-strict C++ 1.3 and executed wasm32 Rust.

| Function | Corpus | Seeded sweep | Mismatches |
|---|---:|---:|---:|
| vertex_cache | 50 | 2000 | 0 |
| overdraw | 50 | 2000 | 0 |
| simplify | 65 | 2000 | 0 |
| simplify_with_attributes | 62 | 2000 | 0 |
| simplify_scale | 52 | 2000 | 0 |

Seed: 20261002. All five functions include 1,000,002-index cases.

Square-root probe: 65543 exact f32 results, including signed zero and subnormals.

All five upstream JS suites passed unchanged; their applicable-input inventory is in COVERAGE.md.

Stable fuzz smoke ran 600 seconds per module. See results/fuzz.json for executions and the instrumentation limits.

These are Linux x86-64 and wasm32 lane records. AArch64 and the release fuzz/performance gates remain outside this lane.

The records and external compressed input/output buffers retain SHA-256 identities. report.sh rejects stale source and missing or changed artifacts.
