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

Overall release qualification: incomplete or blocked; see MEASURED_RESULTS.json.

Phase 0.5 historical evidence is assessed separately below.

## 0.5 local qualification

Scalar-strict pinned C++ versus native Rust and executed wasm32 Rust. The six focused fixtures pass; each row has 2,000 seeded exact native comparisons and 2,000 executed WASM identity comparisons.

| Upstream function | Native cases | Native mismatches | WASM cases | WASM mismatches | 300 s ASan smoke |
|---|---:|---:|---:|---:|---|
| `stripify` | 2000 | 0 | 2000 | 0 | pass |
| `stripify_bound` | 2000 | 0 | 2000 | 0 | pass |
| `unstripify` | 2000 | 0 | 2000 | 0 | pass |
| `unstripify_bound` | 2000 | 0 | 2000 | 0 | pass |
| `vertex_cache` | 2000 | 0 | 2000 | 0 | pass |
| `vertex_fetch` | 2000 | 0 | 2000 | 0 | pass |
| `overdraw` | 2000 | 0 | 2000 | 0 | pass |
| `coverage` | 2000 | 0 | 2000 | 0 | pass |
| `omm_measure` | 2000 | 0 | 2000 | 0 | pass |
| `omm_rasterize` | 2000 | 0 | 2000 | 0 | pass |
| `omm_entry_size` | 2000 | 0 | 2000 | 0 | pass |
| `omm_compact` | 2000 | 0 | 2000 | 0 | pass |
| `tangents` | 2000 | 0 | 2000 | 0 | pass |
| `normals` | 2000 | 0 | 2000 | 0 | pass |
| `remesh` | 2000 | 0 | 2000 | 0 | pass |

The normal and remesh rows are gated by `experimental`. These checks establish exactness on the recorded corpus, not all possible inputs. The separate RFC performance qualification is open.
