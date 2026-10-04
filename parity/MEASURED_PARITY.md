# Measured geometry parity

Exact meaningful output bits: scalar-strict C++ 1.3, native Rust and executed wasm32.

| Function | Fixture corpus | Seeded release sweep | Mismatches |
|---|---:|---:|---:|
| vertex_cache | 50 | 10000 | 0 |
| overdraw | 50 | 10000 | 0 |
| simplify | 65 | 10000 | 0 |
| simplify_with_attributes | 62 | 10000 | 0 |
| simplify_scale | 52 | 10000 | 0 |

Seed: 20261002. Per-case records and buffers are external; their SHA-256 identities are in results/run.json and results/sweep.json.

Long fuzzing requires four process CPU-hours for each of the five targets (owner amendment, RFC section 14, 2026-10-04). See results/fuzz.json for executions, crashes, elapsed and CPU time, and replay-corpus identities.

Local checks do not establish remote macOS, Windows or Linux arm64 execution. CI compares a hashed native-output corpus exactly and keeps C++ on Linux.

Overall release qualification: passed.
