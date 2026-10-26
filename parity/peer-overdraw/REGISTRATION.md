# SPEC-027 lane B — registered final campaign

Written before any wall-clock sample. Baseline: v0.2.6,
33556c48efc13f3f36a9293aa5fbf451c0884fe3. Exported comparator:
bench/peer-screen e8827e4de9b69c98b9a96c6f49f8192a6b045ff7.
Only lane B, overdraw and real-overdraw. Exactly one wall campaign after
instruction-count iteration and targeted proofs. No result-driven reruns or
post-campaign implementation changes.

## Inputs and arms

The frozen five synthetic cases are tiny-smooth, medium-smooth, medium-seams,
medium-sparse and million-smooth. The three real overdraw captures are 214e,
b6fd and a082, with their full stride-64 captured vertices and thresholds.
`frozen-inputs.json` binds lossless copies from the retained bet 0 corpus.
No cache prepass, replacement geometry or changed threshold.

Moss scalar profile: opt-level 3, thin LTO, one codegen unit, stripped;
no peer-simd; default generic target; empty Rust flags and compiler wrappers;
M7 CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD. Candidate and v0.2.6 use std with
crate default features disabled, as in the comparator. X is optimesh 1.1.0;
M7 is meshopt 0.7.0 / C++ 1.3. Candidate source, drivers, dependency lock,
inputs and executable SHA-256 hashes are sealed before sampling and checked
afterward. Two independently started candidate processes supply A/A.

## Sampling and analysis

CPU-only moss-heavy, MOSS_HEAVY_GPU=0, MOSS_HEAVY_MEASUREMENT=1, declaration
4 GB, target `/mnt/linux-extra/moss-cargo-targets/meshopt-peer-b`.
Pin every resident process to CPU 20 (physical core 10, sibling 21).
Do not wait for other lanes or seek a quiet host. Retain host/load telemetry.

For each case/comparison: one unrecorded warm-up batch per arm, then exactly
10 alternating interleaved pairs. Even pairs run candidate first; odd pairs
run comparator first. Fixed batch iterations: tiny 10,000, medium 100,
million 2, real 4. Driver-reported nanoseconds per call include allocation,
validation, scratch use and destruction; parsing, serialization and process
startup stay outside. No calibration samples, early stop or pair exclusion.
Each case runs A/A, candidate/X, candidate/v0.2.6, then (real only)
candidate/M7. A/A processes use the identical candidate executable.

All ratios are candidate/comparator (lower is faster). Case point estimate
is the geometric mean of its ten paired ratios. Two-sided percentile 95%
intervals resample the ten paired log ratios 10,000 times, seed 20261029;
quantiles use sorted positions floor(0.025*N) and floor(0.975*N).
Family GM gives equal weight to each frozen case. Family intervals resample
pairs independently within each case, then average case log means.

Same-session floor F is the maximum over all eight A/A cases of
`max(0, upper95 - 1, 1/lower95 - 1)`. Report it without capping or tuning.
For each family, all three bars must pass:

- candidate/X family GM <= 1.00;
- candidate/v0.2.6 family GM <= 1.00;
- every case upper95 against v0.2.6 <= 1.10 + F.

Verdict PASS if all pass. PARTIAL if the X bar misses while both v0.2.6 bars
pass. FAIL otherwise. Report candidate/M7 for all three real meshes and
the real-family GM; those values are disclosure only. No peer output parity
claim follows merely from a timing bar. Exact correctness is a separate gate.

Infrastructure failures keep all samples and stop the campaign as incomplete;
there is no timing retry. Instruction-count probes are diagnostic only and
never substitute for this registered wall campaign.

## Proof and retention

Run cargo test --all-features, all-target/all-feature clippy -D warnings, fmt;
existing overdraw C++ differential; frozen corpus C++ byte comparison; and
v0.2.6/candidate exact Result/output comparison over the frozen corpus and
existing seeded overdraw mutations, including all API forms, layouts,
resource limits, work usage, partial output and untouched tails. No 10k
sweep or long fuzz. Also check std-disabled output for the preserved software
backend. Keep raw sources, scripts, receipts and binaries in lane scratch;
remove only the designated Cargo target after retaining required artifacts.
