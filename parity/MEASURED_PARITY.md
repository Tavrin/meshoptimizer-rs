# Measured geometry parity

Final lane 3 sources: exact scalar-strict C++ and executed wasm32/native Rust identity.

| Function | Corpus | Seeded sweep | Mismatches |
|---|---:|---:|---:|
| vertex_cache | 50 | 2000 | 0 |
| overdraw | 50 | 2000 | 0 |
| simplify | 65 | 2000 | 0 |
| simplify_with_attributes | 62 | 2000 | 0 |
| simplify_scale | 52 | 2000 | 0 |

Square-root probe: 65543 exact results. Both feature modes pass all 45 API tests.
Five stable fuzz targets ran 300 seconds each with zero crashes (554,090,714 executions).
All five unchanged upstream JS suites passed. Build/format/clippy and the wasm32 build passed.
See `results/lane3-gates.json`, `results/fuzz.json`, `results/run.json` and `results/sweep.json` for identities and command evidence.
Performance has a separate verdict in `MEASURED_PERFORMANCE.md`. AArch64 and release qualification are not claimed.
