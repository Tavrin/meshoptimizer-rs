# Contract simplification qualification

The exact comparison consumers require separately named baseline and candidate
crates. `exact.rs` covers the simplify families, and `overdraw-exact.rs`
covers overdraw. They compare complete output buffers, results and work/storage
usage. `differential.py` uses the pinned C++ reference and a retained Rust
protocol executable. `fuzz.py` resumes an append-only CPU-budgeted stream.

`measure.py` consumes frozen peers-format inputs and four previously built
resident drivers under CPU-only admission. Set `MESHOPT_SCRATCH` to the
private evidence directory and `MESHOPT_PERF` to perf 6.17; `MESHOPT_CPU`
selects an allowed CPU. Finished samples append to `full-counts.jsonl` and
are skipped on restart; incomplete files are moved aside. `results.py` checks
sample and output identity and applies REGISTRATION.md. Raw drivers, inputs,
outputs and admission logs remain in scratch. The two scratch driver sources
must expose the same operations and use the same profile and call shapes.
