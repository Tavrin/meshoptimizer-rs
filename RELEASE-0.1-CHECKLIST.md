# Release 0.1 checklist

Status values: **pass** (evidence verified by `parity/report.sh --phase 0.1
--verify-artifacts`, exit 0), **pending human** (owner action), **pending Moss
migration proof** (RFC section 7; needs the Moss-side harness). Evidence paths
are relative to the repository; large per-case records live in
`$MESHOPT_ARTIFACTS` and are identified by SHA-256 in the JSON summaries.
Source baseline: `src/simplify.rs` after d1230c3. Decisions: `parity/DECISIONS.md` D50, D51.

| # | RFC 0.1 requirement | Evidence | Status |
|---|---|---|---|
| 1 | Scope: simplify, simplify_with_attributes (flags, Permissive), simplify_scale, vertex cache, overdraw (RFC 3) | `parity/COVERAGE.md`, `tests/api.rs` | pass |
| 2 | Pinned oracle (meshoptimizer 1.3, unmodified) is verified (5.4) | `parity/check-reference.sh` | pass |
| 3 | Exact native C++ / native Rust / wasm32 parity on fixtures and differential cases (5.3) | `parity/results/run.json` (279 cases, 0 mismatches) | pass |
| 4 | Exact math (`sqrtf` probe, 65,543 values) and no-FMA arithmetic (5.2) | `parity/results/run.json` (math probe) | pass |
| 5 | 10,000 seeded cases per function, no failed seed skipped (5.4) | `parity/results/sweep.json` (5 x 10,000, 0 mismatches) | pass |
| 6 | Upstream JS suites unchanged, plus Rust adapter (5.4) | `parity/results/js.json` | pass |
| 7 | Executed wasm32 identical to native Rust (5.3) | `parity/results/run.json`, `sweep.json` | pass |
| 8 | Linux AArch64 execution, exact against x86-64 Rust (5.3) | `.github/workflows/ci.yml`: `cross-target-identity (ubuntu-24.04-arm)` passed in CI run 37171297561 at 5a895d3 | pass; re-confirm on the release commit |
| 9 | macOS arm64 / Windows x86-64 execution before advertising parity there (5.3) | `.github/workflows/ci.yml`: `cross-target-identity (macos-14)` and `(windows-2022)` passed in CI run 37171297561 at 5a895d3 | pass; re-confirm on the release commit |
| 10 | Flags, errors, limits (1 GiB, 2^34 work), failure-side guarantees (4.3, 4.4) | `tests/api.rs`, unit tests via `parity/results/gates.json` | pass |
| 11 | No `unsafe`, `no_std` + wasm32 build, MSRV 1.88, both feature modes (4.5) | `parity/results/gates.json` (16 commands exit 0) | pass |
| 12 | fmt, clippy `-D warnings`, rustdoc warnings denied | `parity/results/gates.json` | pass |
| 13 | Fuzz robustness, 4 CPU-hours per target (RFC 14); 5 targets | `parity/results/fuzz.json`: 4.111-4.242 CPU-h, 0 findings, corpus hashes recorded | pass |
| 14 | Weekly/nightly background fuzzing; any crash blocks the next release (RFC 14) | `.github/workflows/fuzz.yml`, `parity/fuzz-continuous.sh` | pass (configured) |
| 15 | Performance: family GM <= 1.25, case max <= 1.50, memory <= 1.25 (6.1), Moss thin-LTO profile | `parity/results/benchmark-moss.json` (204 cases, all families pass) | pass |
| 16 | Same bars under Cargo release defaults | `parity/results/benchmark-default.json` (204 cases, all families pass) | pass |
| 17 | Moss integration: cook time <= 1.10x, mesh stage <= 1.25x baseline (6.1) | none yet | pending Moss migration proof |
| 18 | Three-way output proof (cpp 0.25, cpp 1.3, Rust) and intentional-deviation records (7.2) | none yet | pending Moss migration proof |
| 19 | Package contents and publish dry run | `parity/results/gates.json` (`cargo package --list`, `cargo publish --dry-run --allow-dirty`; offline loopback registry, D9) | pass |
| 20 | Licence and upstream notices, `UPSTREAM.md`, README scope/limits/alternatives (8) | `LICENSE`, `UPSTREAM.md`, `README.md` | pass |
| 21 | `MEASURED_PARITY.md`, `MEASURED_PERFORMANCE.md`, `MEASURED_RESULTS.json`, API and target coverage table (8) | `parity/` | pass |
| 22 | Moss distributed notices carry upstream attribution (8) | Moss repository | pending Moss migration proof |
| 23 | Crate name availability, naming approval, trademark clearance (2, 13) | none | pending human |
| 24 | Publishing to crates.io and tagging | none | pending human |

Not established: coverage-guided or sanitizer fuzzing (coverage unavailable);
quiet-host performance (measurements are shared-host with retained load
telemetry); any C++ wasm32 oracle (not claimed); absent historical lane 3
buffers (listed as such by the report).
