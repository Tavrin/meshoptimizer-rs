# 0.2.19 candidate qualification

**Release disposition:** published as 0.2.19 because every correctness check on the existing API passes and the virtual-geometry API is additive. The benchmark goals listed below as unmet remain open targets for the next release; they are not met by this release.

**Qualification lane verdict against all frozen bars: NOT QUALIFIED.** Candidate `ca7ff9a31e9342bda30a403d66ea3ff05bfcd346`; baseline `v0.2.18` (`0ac5d7655dd5e372585cb2776d193092331267c5`). The manifest still declares version 0.2.18; release versioning is operator-owned.

Build-time bar passes: 0/5 at 1T and 0/5 at 8T. Resident-bytes bar passes: 0/5. These failed acceptance bars prevent qualification; complete results and the strict C++ difference bar are recorded below.

Frozen source, manifest, lockfile and toolchain custody is recorded in [final-custody.json](final-custody.json); any mismatch fails qualification. No source repairs, Git integration, or fuzz campaigns were performed. The disposable target was removed; binary hashes and caller-owned logs retain execution custody.

## Required gates

| Gate | Exit | Log |
|---|---:|---|
| fmt | 0 | [logs/fmt-1.log](logs/fmt-1.log) |
| clippy | 0 | [logs/clippy-1.log](logs/clippy-1.log) |
| release | 0 | [logs/release-1.log](logs/release-1.log) |
| release-all-features | 0 | [logs/release-all-features-1.log](logs/release-all-features-1.log) |
| public-text | 0 | [logs/public-text-1.log](logs/public-text-1.log) |
| public-text-final | 0 | [logs/public-text-final-1.log](logs/public-text-final-1.log) |
| public-text-tables | 0 | [logs/public-text-tables-1.log](logs/public-text-tables-1.log) |

## Preserved APIs and C++ 1.3

34 ordinary exact-driver runs completed; 0 failed. Modes are all features and no default features with clusterlod enabled. The drivers cover success, failure, workspace limits, callbacks, float words and retained output prefixes. Counts below are observations of the stated finite campaigns, not exhaustive proofs over all inputs.

Additional legacy cluster-LOD corpus and 1/4/16-worker checks, parallel/stream identity, C++ protocol, spatial/meshlet/cluster-LOD, decoder ISA and LOD-quality differentials are recorded in [exits.jsonl](exits.jsonl). The supplemental driver compares the eight 0.2.18 impostor/trim cases and complete MDAG 1 bytes plus direct build fields on 4x4, 12x12 and 24x24 grids, in both feature modes.

- `remap/all`: cases 4660 comparisons 10811136 differences 0 ([log](logs/exact-remap-all-1.log)).
- `stream/all`: corpus=21 generated=160 noncanonical=20 total_cases=9990 cpp_checks=9990 chunkings=2703362 stream_budgets=2284337 baseline_comparisons=164442780 differences=0 ([log](logs/exact-stream-all-1.log)).
- `decode/all`: exact observations=3505440 differences=0 ([log](logs/exact-decode-all-1.log)).
- `vdecode/all`: exact observations=5114700 differences=0 ([log](logs/exact-vdecode-all-1.log)).
- `simplify/all`: PASS observations=8970468 ([log](logs/exact-simplify-all-1.log)).
- `overdraw/all`: PASS: 20558672 exact observations; frozen corpus + 4096 existing seed mutations + every small-mesh work prefix ([log](logs/exact-overdraw-all-1.log)).
- `remap/none`: cases 4660 comparisons 10811136 differences 0 ([log](logs/exact-remap-none-1.log)).
- `stream/none`: corpus=21 generated=160 noncanonical=20 total_cases=9990 cpp_checks=9990 chunkings=2703362 stream_budgets=2284337 baseline_comparisons=164442780 differences=0 ([log](logs/exact-stream-none-1.log)).
- `decode/none`: exact observations=3505440 differences=0 ([log](logs/exact-decode-none-1.log)).
- `vdecode/none`: exact observations=5114700 differences=0 ([log](logs/exact-vdecode-none-1.log)).
- `simplify/none`: PASS observations=5769468 ([log](logs/exact-simplify-none-1.log)).
- `overdraw/none`: PASS: 20558672 exact observations; frozen corpus + 4096 existing seed mutations + every small-mesh work prefix ([log](logs/exact-overdraw-none-1.log)).

**Strict C++ zero-difference bar: False.** Decoder receipts contain 449179 failed-call destination-write observations that differ from C++. Success outputs, status comparisons and forced ISA differences are recorded separately. The inherited runner exits 0 while disclosing failure-buffer differences; this lane records them as a failure of the strict zero-difference bar in the brief. Exact Rust baseline comparisons remain unchanged, so these observations are not evidence of a new 0.2.19 regression. See [counts](cpp-observations.json) and the [byte counterexample](cpp-failure-counterexample.json).


C++ differential command runs: 10/10 exited 0. Spatial/meshlet/cluster-LOD generated-case counts are additionally retained in [p03-differential.json](p03-differential.json).


## Instructions

3024 instructions:u samples; 0 case/mode regressions above 1%. Medians, group geometric means, worst cases and baseline/C++ byte identity are in [TABLE.md](TABLE.md) and [release-results.json](release-results.json). This retains the 21-operation, eight-input campaign from 0.2.18; it does not claim instruction measurements for every public wrapper or new API.


## Semver

cargo-semver-checks against 0.2.18: see [default](logs/semver-default-1.log) and [all features](logs/semver-all-features-1.log). Both modes passed 202 rules (58 skipped), with no semver update required.

## New API determinism and wasm decode

Hash equality across 8 variants: `True`. Four cases cover v2 build and MDAG 2 write (opaque/material cases enable default cluster optimization; the alpha retention case disables it), ATTR/material/FARF payloads, active alpha coverage, all eleven attribute encodings and decoded float words. Native all-feature release/debug also compare 1/2/4/8/16 workers. The no_std label describes the library dependency; native and wasm test drivers use host support. Complete hashes and binary bindings are in [determinism.json](determinism.json).

Wasm page timing is a Node/V8 CPU microbenchmark on a geometry-only page: seven batches of 100,000 compressed-page transcodes plus position and triangle decodes, with warmup. Each page has 288 triangles and two clusters with 122 and 61 positions. Setup is included once per batch and amortized; these are wall timings on a shared host, not an instruction or browser/runtime acceptance gate.

- `wasm-nostd`: median 29.193 us/page, range 27.704 to 30.242 ([log](logs/wasm-timing-wasm-nostd-3.log)).
- `wasm-default`: median 28.122 us/page, range 27.465 to 28.808 ([log](logs/wasm-timing-wasm-default-3.log)).
- `wasm-default-simd128`: median 29.744 us/page, range 28.442 to 32.799 ([log](logs/wasm-timing-wasm-default-simd128-3.log)).

## Final virtual-geometry benchmark

Reference: Bevy MeshletMesh at commit 8cc060f01c3d4d5920781124cc8c32013bb47730. The locked dependency graph is retained in bench/Cargo.lock, and the reference source check and hashes are in logs/vg-reference-pin-1.log.

# Final virtual-geometry qualification measurements

Candidate v2 builder and MDAG 2 attribute payloads against the pinned reference. Five interleaved repeats; median build plus serialise milliseconds. Ratios below 1 favor the candidate.

| Mesh | Build ratio 1T / 8T | File B/DAG tri ours / ref | Resident B/DAG tri ours / ref | Cut tris ratio at 5 / 8 px | Measured/claimed median ours / ref |
|---|---:|---:|---:|---:|---:|
| bunny | 2.023 / 0.863 | 13.06 / 13.01 | 14.84 / 13.78 | 0.676 / 0.928 | 1.000 / 1.161 |
| coast_rocks_01 | 1.352 / 0.566 | 12.49 / 12.49 | 14.09 / 13.23 | 1.114 / 1.671 | 0.980 / 1.166 |
| dead_tree_trunk_02 | 1.889 / 0.741 | 13.29 / 13.84 | 14.97 / 14.52 | 0.710 / 0.782 | 0.905 / 1.199 |
| dragon | 1.298 / 0.520 | 12.58 / 11.68 | 14.36 / 13.32 | 0.818 / 1.298 | 0.930 / 1.437 |
| namaqualand_boulder_02 | 1.715 / 0.656 | 13.31 / 13.93 | 15.06 / 14.67 | 0.952 / 0.971 | 0.979 / 1.100 |


Bar passes out of five meshes:

- `build_half_reference_1T`: 0/5.
- `build_half_reference_8T`: 0/5.
- `file_per_dag_triangle`: 3/5.
- `resident_per_dag_triangle`: 0/5.
- `fewer_cut_triangles_5px`: 4/5.
- `fewer_cut_triangles_8px`: 3/5.
- `measured_claimed_median_1_3`: 5/5.
- `self_checks`: 5/5.
- `thread_byte_identity`: 5/5.

The build bar is at most half the reference at equal thread count. File and resident bars compare bytes per total DAG triangle. Cut efficiency requires fewer triangles at equal measured p95 deviation (log-log interpolation at 5 and 8 px; absent overlap does not pass). The sampled measured/claimed bar is median <= 1.3 at the 1 px threshold. These samples are not continuous error certificates. Reference position quantization and candidate normal/UV payload costs remain part of the measured formats. Raw per-mesh rows and input/binary hashes are in [bench](bench/); full tables are in [the benchmark log](logs/vg-report-1.log).

## Failures

Resolved harness/setup attempts are retained below: the initial timing fixture contained unused vertices, which the page encoder correctly rejected; the timing fixture now compacts cluster-local vertices. The copied benchmark lockfile still listed path package 0.2.17 and was refreshed to 0.2.18 before the locked build. A pre-final public-text check also found the resolved toolchain path in a panic backtrace; the public log copy was redacted and the unchanged guard rerun, while the original scratch log was retained. No library source was changed. Initial hash receipts and the initial timing source remain in determinism-initial.json and determinism-initial.rs.

7 failed attempts across 326 recorded commands; 1 command names still have a nonzero latest exit. Resolved setup attempts do not determine the candidate verdict. Remaining command and acceptance-bar failures do.

- `wasm-timing-wasm-nostd` exit 1: [logs/wasm-timing-wasm-nostd-1.log](logs/wasm-timing-wasm-nostd-1.log).
- `wasm-timing-wasm-default` exit 1: [logs/wasm-timing-wasm-default-1.log](logs/wasm-timing-wasm-default-1.log).
- `wasm-timing-wasm-default-simd128` exit 1: [logs/wasm-timing-wasm-default-simd128-1.log](logs/wasm-timing-wasm-default-simd128-1.log).
- `vg-build` exit 101: [logs/vg-build-1.log](logs/vg-build-1.log).
- `timing-native-smoke` exit -6: [logs/timing-native-smoke-1.log](logs/timing-native-smoke-1.log).
- `public-text-pre-final` exit 1: [logs/public-text-pre-final-1.log](logs/public-text-pre-final-1.log).
- `strict-cpp-zero` exit 1: [logs/strict-cpp-zero-1.log](logs/strict-cpp-zero-1.log).

## Not verified

- GPU decode replay and production renderer/residency integration.
- ARM timing (no ARM execution was performed).
- A fuzz campaign on this candidate; campaigns are owned by the separate lane.
- Continuous geometric error bounds; quality results are sampled CPU measurements.

Reproduction and environment requirements are in [README.md](README.md); admission and output directories are supplied by environment variables.
