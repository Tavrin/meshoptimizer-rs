# Large-mesh simplify and overdraw qualification (qx5g3)

Overall verdict: **FAIL — not a release candidate.** Simplify meets its real-case target on all three cases, but overdraw misses 1.50 on 214e and b6fd. The final frozen overdraw maximum is 1.545072, exceeding the unchanged 1.50 bar. Separately, the single 0.2.3 frozen baseline timed out after 200/204 cases, leaving four attribute cases and the complete attribute no-regression proof unavailable. No rerun, bar change, timeout change or sample replacement was performed.

Base: `afe803bbf60796a7af6b5001594a979af009cfac` (0.2.3). Branch: `perf/simplify-large-mesh`. Corpus precheck: **695/695**, exit 0. Diagnosis: `/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/DIAGNOSIS.md`. Frozen failure: `/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/FROZEN_FAILURE.md`.

## Diagnosis ranking

User-cycle sample attribution within measured stage timers; estimated loop seconds are not independent wall timers. Primary stage wall: C++/Rust simplify 6.494831/10.391690 s; overdraw 0.200577/0.676869 s. Same CPU 8; parity fat-LTO release profile; C++ 1.3 scalar-strict; `perf record` with DWARF symbols. Primary replay records exactly match retained a082 B1.

| Rank | Rust loop/function | Estimated s |
|---|---|---:|
| 1 | Position remap/probes/position reads | 6.091 |
| 2 | Quadric construction/validation/weights | 1.526 |
| 3 | Vertex classification | 0.759 |
| 4 | Edge adjacency | 0.620 |
| 5 | Overdraw kernel | 0.466 |

Rust overdraw validation adds an estimated 0.201 s. Grouped unresolved simplify work: Rust 0.394 s, C++ 1.823 s. Sampling cannot separate all memory stalls, bounds checks, kernel time, allocation, or inline operations. See raw profiles, clock offsets and attribution scripts.

## Implemented fixes and rejected hypotheses

1. Cache original strided positions in the already-budgeted position allocation during hash probes; normalize at the original phase. Packed input keeps the existing path. Hash, probe order and counted visits are unchanged.
2. Load checked XYZ records together, preserving mappings, endian handling and unaligned byte offsets.
3. Dispatch sequential position scans by layout once; retain `Work::scan` and its exact failure prefix.
4. Validate positional quadrics and refresh inverse weights in one pass, with unchanged divisions and numerical checks.

Dead hypotheses: none rejected for byte parity; all four immutable fix snapshots passed all-feature tests and native/WASM differential checks. The hypothesis that these four fixes would close both real-case targets was not established: overdraw still misses on two cases. Per-fix timing gains were not isolated. Per-fix source exports, patches and test receipts are retained. Setup failures/canceled admission waits are described in BUILD_NOTES.md; they are not timing samples. No public signature, default Limits, accounting threshold, hash, algorithm, cache kernel, encoder, SIMD implementation or Moss source was changed. Quadric remains **48 bytes**; optional shrink not attempted; bytes saved: **0**.

## Registered real-case campaign

One warm-up pair plus ten measured pairs per baseline/final comparison, alternating AB/BA on CPU 8 in one campaign. No timing-based retries or exclusions. Ratios are medians of paired Rust/C++ stage-total ratios. Intervals use 10,000 paired bootstrap resamples, fixed seed 20261025. Raw backend medians, ratios of medians, intervals and samples are in timings.json and campaign/raw.jsonl.

| Case | Stage | 0.2.3 C/B [95%] | Final C/B [95%] | Baseline / final Rust median s | Baseline / final C++ median s |
|---|---|---:|---:|---:|---:|
| 214e | simplify | 1.385 [1.342, 1.411] | 0.948 [0.938, 0.977] | 8.670441 / 5.934655 | 6.283580 / 6.247219 |
| 214e | overdraw | 3.239 [3.153, 3.280] | 1.698 [1.669, 1.718] | 0.637769 / 0.332274 | 0.195407 / 0.196794 |
| 214e | scale_raw | 3.677 [3.548, 3.814] | 2.179 [2.156, 2.371] | 0.077974 / 0.045184 | 0.021346 / 0.020903 |
| 214e | cache | 0.931 [0.858, 0.984] | 0.938 [0.904, 0.986] | 0.242584 / 0.247138 | 0.264430 / 0.262837 |
| b6fd | simplify | 1.283 [1.240, 1.322] | 0.940 [0.925, 0.948] | 2.061770 / 1.500228 | 1.623618 / 1.604687 |
| b6fd | overdraw | 3.010 [2.935, 3.167] | 1.705 [1.698, 1.715] | 0.292296 / 0.163938 | 0.096437 / 0.095705 |
| b6fd | scale_raw | 3.081 [2.957, 3.116] | 1.840 [1.808, 1.932] | 0.020598 / 0.012244 | 0.006814 / 0.006670 |
| b6fd | cache | 1.409 [1.368, 1.435] | 1.404 [1.389, 1.420] | 0.122637 / 0.120290 | 0.087631 / 0.086240 |
| a082 | simplify | 1.060 [1.046, 1.063] | 0.942 [0.939, 0.949] | 12.922866 / 11.483619 | 12.249274 / 12.196399 |
| a082 | overdraw | 2.303 [2.251, 2.324] | 1.377 [1.309, 1.401] | 2.069357 / 1.250745 | 0.906240 / 0.908460 |
| a082 | scale_raw | 3.772 [3.743, 3.867] | 2.182 [2.096, 2.243] | 0.082318 / 0.047942 | 0.021697 / 0.021750 |
| a082 | cache | 1.092 [1.088, 1.096] | 1.084 [1.067, 1.109] | 1.626041 / 1.631412 | 1.480776 / 1.506048 |

Real-case targets met: **False**. Simplify ratio improved on all three: **True**. Overall FAIL also follows from incomplete frozen proof and the final frozen overdraw maximum.

## Frozen corpus

The original 204-case matrix and its sample/ordering policy are unchanged. Required time bars: family GM ≤1.25, maximum ≤1.50; final GM may worsen by at most 2% versus 0.2.3. No partial family is promoted to a complete gate.

| Family | Baseline cases / GM / max | Final cases / GM / max | Matched-case GM change (diagnostic) |
|---|---|---|---:|
| simplify | 72/72 / 1.055749 / 1.208898 | 72/72 / 1.044775 / 1.203760 | -1.039% |
| simplify_with_attributes | 68/72 / 1.075294 / 1.232378 | 72/72 / 1.073585 / 1.231086 | -0.156% |
| overdraw | 24/24 / 1.103294 / 1.512628 | 24/24 / 1.118966 / 1.545072 | +1.420% |

The baseline overdraw maximum is 1.512628 and the final maximum is 1.545072 at million/disconnected/allocating, both retained unchanged. The final maximum is a failing registered bar, even though the overdraw GM change is within 2%. Attribute baseline has only 68/72 cases; its displayed GM is partial. Failed input and SHA-256 survive in frozen-baseline-failed.input/json.

## Commands and exit receipts

Initial fix-1 check (`check-fix1.sh`): cargo fmt --check, cargo test --locked --all-features, and parity/run.sh --phase 0.1 --profile scalar-strict: exit 0. Differential counts 50/50/65/62/52, zero mismatches.

`checks-batch/exits.tsv`:

- check-fix1-contract: exit 0
- check-fix2: exit 0
- check-fix3: exit 0
- check-fix4: exit 0
- gates: exit 0

`parity-final/exits.tsv`:

- format: exit 0
- reference: exit 0
- gates: exit 0
- run: exit 0
- sweep: exit 0
- js: exit 0

`measurement-exits.tsv`:

- real-campaign: exit 0
- timing-analysis: exit 0
- frozen-final: exit 0
- frozen-no-regression: exit 1

`post-check-exits.tsv`:

- retained-integrity: exit 1
- aggregate-report-gate: exit 1

Corrected current-schema retained-buffer integrity: **exit 0**, 49 real blobs, all run/sweep buffers/source/binary hashes, 200 baseline and 204 final benchmark cases verified. Integrity verifies retained bytes, not qualification completeness.

Exact final gate commands:

- `cargo fmt --check`: exit 0
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: exit 0
- `cargo test --locked`: exit 0
- `cargo test --locked --all-features`: exit 0
- `cargo test --locked --no-default-features`: exit 0
- `cargo build --target wasm32-unknown-unknown --no-default-features`: exit 0
- `cargo build --locked --no-default-features`: exit 0
- `cargo +1.88 test --locked --all-features`: exit 0
- `cargo +1.88 test --locked --no-default-features`: exit 0
- `cargo package --list`: exit 0
- `cargo publish --dry-run --allow-dirty`: exit 0
- `cargo doc --locked --no-deps --all-features`: exit 0
- `cargo doc --locked --no-deps --no-default-features`: exit 0
- `cargo clippy --locked --manifest-path parity/Cargo.toml --all-targets -- -D warnings`: exit 0
- `cargo clippy --locked --manifest-path fuzz/Cargo.toml --all-targets -- -D warnings`: exit 0
- `cargo fmt --manifest-path parity/Cargo.toml --check`: exit 0
- `cargo fmt --manifest-path fuzz/Cargo.toml --check`: exit 0

Fuzz: passed; four process CPU-hours for each of five targets, including retained checkpoint/resume budget.

The aggregate report invocation used the retained historical tracked summaries against the fresh external artifact directory. It exits 1 at run/sources.tar.gz SHA binding; it does not verify the complete historical multi-phase release record set. Fresh individual gate/parity records remain separately retained. The first scratch integrity helper exits 1 because it assumed the older bare-ZIP inventory; corrected current-schema integrity is recorded separately and cannot clear either qualification failure.

## Decision table applied

- PASS requires all three real-case simplify medians ≤1.25 and overdraw medians ≤1.50, exact parity and complete frozen no-regression proof.
- PARTIAL requires exact parity, complete frozen no-regression proof, and simplify improvement on all three with a missed real-case target.
- FAIL applies otherwise. The frozen baseline is incomplete and the final frozen overdraw maximum fails: **FAIL**, regardless of subsequent focused results. Four implemented fixes reached the implementation cap; optional Quadric shrink was not attempted.

## Source, draft PR and not verified

Current checkout revision: `a7688ef0c8d13b8dfeb57c2cb1ce325a76d3dfab`. Source/executable hashes and retained code exports bind each run; the final evidence manifest records exact file checksums.


Completed fuzz: exit 0; source, binaries and dependency identities unchanged.

| Target | CPU hours | Executions | Crashes |
|---|---:|---:|---:|
| vertex_cache | 4.122197 | 1617763645 | 0 |
| overdraw | 4.144844 | 3300496258 | 0 |
| simplify | 4.312708 | 1710167431 | 0 |
| simplify_with_attributes | 4.216372 | 1427553976 | 0 |
| simplify_scale | 4.117581 | 39141831554 | 0 |

The first eight normal shards were checkpointed before official timings; the incomplete-budget receipt exits 1 with zero crashes. The same budget and retained shards resumed after the campaign on explicit CPUs 0,2,4,6,10,12,14,16, excluding timing CPU 8 and sibling 9. The top-level original worker list records the initial placement; transition and observed resumed affinity are retained separately.

This document accompanies the draft performance PR; it is not a release candidate.

Not verified: the complete 0.2.3 frozen attribute matrix and therefore its complete no-regression comparison; the historical aggregate release record set; individual per-fix timing gains. Pending or missing commands/campaigns above remain explicitly unverified; they must not be inferred from an isolated passing check. No release, tag, publish or merge was performed.

Portable raw timing samples, source/executable hashes, gate exits and frozen decision data: [SIMPLIFY_LARGE_MESH_RESULTS.json](SIMPLIFY_LARGE_MESH_RESULTS.json).

Implementation source revision: `a7688ef0c8d13b8dfeb57c2cb1ce325a76d3dfab`. Documentation-only commits do not alter the tested source files.
