# DAG format qualification

The library wire specification is [DAG_FORMAT.md](../../docs/DAG_FORMAT.md).
No upstream algorithms or exact-check harnesses are modified by this feature.

- `cargo test --all-features --test meshlets --test dag_format` round-trips the
  existing caller-buffer DAG fixture and checks truncations and table corruption.
- `cargo run --example dag_identity --no-default-features` emits a fixed synthetic
  DAG as hex. Build the same example for `wasm32-wasip1`, run it with
  `node --no-warnings parity/dag-format/wasm.mjs <wasm-file>`, and compare stdout.
  WASI supplies stdout only; serialized library bytes use explicit LE operations.
- `cargo run --release --features clusterlod --example dag_format_bench -- <input> ...` takes the three
  existing parity corpus real-mesh inputs (two u32 counts, packed XYZ float words,
  then triangle u32 indices; any trailing auxiliary data is ignored). It builds once and measures five write/read passes.
  Reported times are median write, median validated read, and median page decode
  over all pages/passes. Writes include full validation; read includes all page
  decodes. Mesh loading and DAG build time are excluded. Write/read measure in-memory
  serialization/validation; no filesystem IO is included in those timers. Per-cluster duplicated raw geometry is the
  compression baseline, not the deduplicated original mesh. It checks byte identity
  across all five writes. This is a size/speed characterization, not a performance
  acceptance threshold.
- `cargo run --release --manifest-path fuzz/Cargo.toml --bin dag_format -- 600`
  mutates checked-in valid seeds and runs for ten minutes of process CPU time.
  It covers header/table/payload mutations and truncations, then page-decodes any
  accepted inputs. This is a seeded mutation smoke campaign, not libFuzzer coverage.

Execution receipts and exit codes belong under `receipts/` after execution.

## Completed qualification (2026-10-31)

API: `clusterlod::dag_format::{write, Reader, Geometry, Error}`; flat records and
serialization are available with no Cargo features. DAG construction retains its
existing `clusterlod` feature. MDAG v1 uses LE header/group/cluster/page tables,
whole-cluster pages, strict flags and independently decodable geometry.

All final code checks passed: fmt, all-target/all-feature Clippy with warnings
denied, all-feature/default/no-default-feature tests and doctests, no-default
wasm32-unknown-unknown and wasm32 WASI builds, exact native/Wasm output comparison,
public-text check, pinned upstream reference check, and the unchanged 0.1 C++
exact gate (279 cases across five families, zero mismatches). The identity probe
compares both existing DAG fixture files and a fixed synthetic hierarchy.

Benchmarks use AMD Ryzen 9 7945HX, 32 logical CPUs. CPU affinity was not pinned;
shared-host concurrent work was present. Five serialization writes per input were
byte-identical. Times characterize this run; no speedup or rendering claim is made.

| Mesh | Raw geometry MiB | Stored geometry MiB | File MiB | Write ms | Read ms | Page decode us |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| real-214e | 271.237 | 120.770 | 127.855 | 1547.463 | 357.754 | 367.662 |
| real-a082 | 303.885 | 135.285 | 143.156 | 1701.658 | 381.055 | 355.208 |
| real-b6fd | 75.981 | 35.123 | 37.080 | 445.512 | 108.886 | 371.869 |

Reader mutation campaign: seed 217, 53,608,094 executions, 5,107,128 accepted inputs, 600 CPU seconds, 600.064 wall seconds, 0 crashes; exit 0. All accepted inputs were independently page-decoded again.

`final-receipts/exits.txt` retains every attempt in that sequence. The initial
benchmark/hash attempts used dangling corpus symlinks and failed before measurement;
`bench-retry=0` and `input-hashes-retry=0` use retained copies. Input SHA-256 values
and exact byte counts are in `final-receipts/benchmark.json`. Earlier implementation
and test-assertion failures remain in `receipts/`: two Clippy chunk lints, feature
wiring that initially exposed builder helpers without their feature, and corruption
assertions that modified a valid page capacity or rewrote an unchanged -1 sentinel.
The feature wiring was narrowed, the assertions corrected, and the full matrix was
rerun successfully. No existing exact check was weakened.

`final-receipts/fuzz.json`, `cpp-exact-summary.json`, and the native/Wasm hex
outputs retain the executed evidence. Private machine paths in logs are replaced
with `<local-path>` to satisfy the public-text gate. Source/receipt hashes are in
`final-receipts/binding.json`. Dedicated build target removal is recorded in
`final-receipts/cleanup.json`. Git state was not changed.
