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

Square-root probe: 65543 exact results. Both feature modes pass all 45 API tests.
Five stable fuzz targets ran 300 seconds each with zero crashes (554,090,714 executions).
All five unchanged upstream JS suites passed. Build/format/clippy and the wasm32 build passed.
See `results/lane3-gates.json`, `results/fuzz.json`, `results/run.json` and `results/sweep.json` for identities and command evidence.
Performance has a separate verdict in `MEASURED_PERFORMANCE.md`. AArch64 and release qualification are not claimed.

## Phase 0.3 — final implementation evidence (2026-10-04)

Pinned oracle: `4c203430ca565cb59a468a91922c76c208169536`, unmodified
meshoptimizer 1.3. All fifteen functions listed in COVERAGE.md use exact
meaningful-output equality against scalar-strict native C++ and executed
`wasm32-unknown-unknown` Rust. The current unpadded meshlet layout is covered.

| Evidence | Required cases | Compared messages | Mismatches |
|---|---:|---:|---:|
| Applicable unchanged native/JS fixtures and focused generated cases | 47 fixtures + 25 per function | 697 | 0 |
| Seeded sweeps, allocating and caller-buffer variants | 2,000 per function | 52,000 | 0 |
| Optional clusterlod demo | 80 nonempty extended fixtures | 80 | 0 |

The demo comparison includes presets, custom ratios and optimization levels,
partition ordering, fold preservation, border dilation, error limits, locks,
attribute widths through 32, protected attributes, arbitrary callback IDs,
changed positions, groups/clusters and hierarchy outputs. The empty-input demo
boundary is tested only in Rust because the C++ demo dereferences empty vectors.
This is identity against the pinned demo, not a stable upstream library contract.

The eleven build gates pass: formatting for the library and both unpublished
packages; clippy with `-D warnings`; all-feature tests (65), no-default-feature
tests (60), no-default-feature plus clusterlod tests (65); the no_std wasm32
build; and documentation. Rust 1.88 passes both primary feature-mode test runs.
The earlier phase 0.1 differential run and all five unchanged upstream JS suites
pass again on these sources. The package list includes every new source module
and tests/meshlets.rs, with no native oracle or fuzz harness in the package.

All fifteen cargo-fuzz targets complete instrumented coverage smokes with zero
failures, at least 301.42 elapsed seconds each, 57,682,971 total executions and
4,321.90 total CPU-seconds. This is the spec's smoke gate, not the RFC release's
four CPU-hours per target or its continuing nightly/weekly fuzzing requirement.

Inputs and meaningful C++ outputs are retained in length-delimited corpora.
Source, executable, compiler/runtime and corpus identities are in each record.
The available artifact root is
`/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p03`: the requested sibling
`/mnt/linux-extra/meshopt-artifacts/p03` is read-only in this sandbox (D71).
The external verifier checks all message counts, feature-mode commands, the
full fifteen-target smoke inventory and hashes independently of performance.
Performance acceptance has its own verdict. No AArch64, Windows/macOS execution,
10,000-case release sweep, publication or Moss integration is claimed.

### Final accounting correction and refreshed gates

D76 supersedes the preceding checkpoint counts: all-feature and no-default-plus-
clusterlod suites pass 66 tests; no-default passes 60. Eleven private unit tests
execute in WASM, including the failing-first child/external u64 work regression.
The refreshed fifteen instrumented smokes complete 52,334,430 executions, at least
309.11 elapsed seconds per target and 4,489.81 aggregate CPU-seconds, with zero
failures and unchanged source/binary identities. The 697/52,000/80 comparison
corpora pass again after the correction. The independent evidence verifier passes;
SPEC acceptance remains FAIL because the complete timing matrix fails its bars.
See MEASURED_P03.json and MEASURED_PERFORMANCE.md.

## Phase 0.3 combined-tree integration

After rebasing onto main with 0.2 and 0.4, `parity/run.sh` and `sweep.sh` pass
again for 0.3: 47 upstream fixtures, 25 generated cases and 2,000 seeded cases
per family, zero native C++ / Rust / executed-WASM mismatches. Slim summaries
are `results/run-0.3.json` and `results/sweep-0.3.json`; their complete corpora
and records are in `/mnt/linux-extra/meshopt-artifacts/p03` and pass
`parity/report.sh --phase 0.3 --verify-artifacts`. The earlier fallback path
above describes the original 0.3 lane, before this integration.
