# Measured lane 1 parity

Exact output comparison with scalar-strict C++ 1.3 and executed wasm32 Rust.

| Function | Corpus | Seeded sweep | Mismatches |
|---|---:|---:|---:|
| vertex_cache | 17 | 2000 | 0 |
| overdraw | 17 | 2000 | 0 |

Seed: 20261002. Both functions include 1,000,002-index cases.

Square-root probe: 65543 exact f32 results, including signed zero and subnormals.

All five upstream JS suites passed unchanged; their applicable-input inventory is in COVERAGE.md.

Stable fuzz smoke ran 600 seconds per module. See results/fuzz.json for executions and the instrumentation limits.

These are Linux x86-64 and wasm32 lane records. AArch64, complete 0.1 simplification and the release fuzz/performance gates remain outside this lane.

The records and compressed input/output buffers in results/ retain SHA-256 identities. report.sh rejects stale source and missing or changed artifacts.
