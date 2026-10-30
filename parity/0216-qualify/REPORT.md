# SPEC-0216-qualify — 0.2.16 combined train qualification

**PASS for the specified combined qualification. No lane reverted.**

Candidate `ff38162d6de2c6c543a45536733442939100801b`; base v0.2.15 `1ecf69abe9f92c17e13dc38fa97be659aee44582`. Production source, Cargo manifest and lockfile are unchanged. This report uses only the current [combined campaign](combined-ff38162/REGISTRATION.md). The earlier, different-baseline qualification is preserved in [REPORT-pre-train.md](REPORT-pre-train.md); its receipts and broader proof requirements are not substituted for this specification.

## Exact merged-code equivalence

Candidate source: `ff38162`; base: v0.2.15
(`1ecf69abe9f92c17e13dc38fa97be659aee44582`). All rows below completed with
exit 0 and zero baseline differences. Counters retain each lane driver's
original meaning; overlapping observations are not presented as independent
coverage. Full logs and process exits are in `logs/` and `exits.jsonl`.

| Lane driver | All features | Scalar/no defaults |
|---|---:|---:|
| remap | 10,811,136 | 10,811,136 |
| repair prerequisites | 68,872 | 68,872 |
| simplify family | 8,970,468 | 5,769,468 |
| decoder (NEON lane's exact driver) | 3,505,440 | 3,505,440 |
| stride-12 decoder exact driver | 5,114,700 | 5,114,700 |
| bounds | 228,760 | 228,760 |
| supplemental geometry | 1,020,255 | 1,020,255 |
| spatial | 4 × 358,458 | 358,458 |
| streaming: unchanged one-shot baseline comparisons | 164,442,780 | 164,442,780 |
| parallel targeted contracts | 86,371 | 86,371 |
| parallel original worker grid, LOD corpus | 720 | 720 |
| parallel 1/4/16 workers, LOD corpus | 450 | 450 |
| parallel 1/4/16 workers, eight frozen inputs | 80 | 80 |

Spatial's four all-feature runs force Scalar, SSE2, SSSE3, and SSE4.1.
The scalar bounds/geometry consumers enable `clusterlod` as required by those
drivers; their `std` and `simd` features are disabled. The scalar parallel
consumer enables `parallel`, `clusterlod`, and `parity-internals`, which also
enables the mandatory `std` dependency; it disables `simd`. See the retained
consumer manifests and binary hashes in `exact-binding-*.json` and
`par-spec-binding-*.json`.

Each streaming run also completed 9,990 C++ wire checks, 2,703,362 chunkings,
and 2,284,337 streaming budget checks. Each remap C++ run completed 2,592
comparisons with zero differences. Each repair property run passed all 45
shared generated meshes.

Cargo tests (including doctests), Clippy, and the LOD identity test on both
the generated corpus and all eight frozen inputs returned 0 in both modes.
The all-feature test invocation reports 350 passed / 0 failed / 8 ignored;
no defaults reports 285 passed / 0 failed / 8 ignored. The required ignored
LOD identity test was separately executed on each input set in each mode.
Formatting and the package/unsafe boundary gate returned 0. The package audit
found exactly one unsafe-code allowance and all 37 audited blocks.

The core job used **2,726.259658 CPU seconds**, below its 6,000-second size
estimate and the 7,200-second maximum. Queue waiting is excluded from CPU
usage. The executable supervisor copy is retained by SHA-256 in `runners/`.
The production source, Cargo manifest and lockfile remained unchanged.

The source-identical Linux/Apple arm64 CI receipts are documented separately
in [CI.md](combined-ff38162/CI.md). Measurement and the remaining merged C++ differentials are
separate stages; this proof record does not assert their acceptance.

## C++ differentials and arm64 rule

All stages returned 0. The reference is clean at
`4c203430ca565cb59a468a91922c76c208169536`. The bundled meshopt 0.7.0 core
and cluster-LOD header were checked byte-for-byte against that reference;
source hashes are retained in `environment.json`.

| Family / protocol | Cases | Result |
|---|---:|---|
| simplify/update/prune/points, cache, overdraw, fetch protocol | 1,246 per feature mode | equal |
| remap C++ driver | 2,592 per feature mode | equal |
| index-sequence wire driver | 9,990 per feature mode | equal under its wire/error contract |
| meshlets, owned/into | 1,024 | equal across seven clients |
| flexible meshlets, owned/into | 1,024 | equal across seven clients |
| spatial meshlets, owned/into | 2,000 | equal across seven clients |
| spatial finite extremes / signed zeros | 28 | equal across seven clients |
| spatial production meshes | 3 | equal across seven clients |
| cluster bounds | 512 | equal across seven clients |
| meshlet bounds | 512 | equal across seven clients |
| sphere bounds | 512 | equal across seven clients |
| cluster LOD | 24 | equal across seven clients |
| decoder and stride-12 lane matrices | 318,608 × 4 | see malformed-output disclosure |

The seven geometry clients are scalar/default C++ 1.3, no-default Rust with
`clusterlod`, and all-feature Rust forced to Scalar/SSE2/SSSE3/SSE4.1. The
geometry matrix totals 5,636 synthetic messages plus three production spatial
messages. The transport's message cap is raised from 128 to 512 MiB solely to
admit those frozen production records; algorithm settings and comparisons are
unchanged. The synthetic input/output corpus hash and production output hashes
are in `p03-differential.json`.

Each decoder lane ran its original fixed differential matrix with std/SIMD
and no-std/SIMD, forcing all four x86 ceilings. All four receipts report zero
forced-level differences and zero C++ successful-byte differences. Status
agreement is asserted for every message. Each reports **74,863 malformed
failure-write differences** across the scalar/default C++ references; failed
destination prefixes are disclosed rather than claimed byte-identical.
The NEON-lane x86 receipts additionally report zero C++ default/scalar ISA
divergences. See `decode-*/` and `vdecode-*/`.

Repair APIs have no matching C++ 1.3 entry points. Their inherited prerequisites
and new-API corpus properties are covered by [PROOF.md](combined-ff38162/PROOF.md), along with
parallel-versus-serial identity. Arm64 uses the documented CI rule and retained
source-identical receipts in [CI.md](combined-ff38162/CI.md).

All peer builds, cost-driver build and C++ differentials together used
**717.1837 CPU seconds**, below the 6,000-second estimate and 7,200-second
maximum. No measurement result is inferred from these proof timings.

## Release measurements

Median of three instructions:u samples per case. Each entry is real-mesh GM / corpus GM. Ratios below 1 mean fewer instructions. Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. optimesh 1.1.0 is scalar; C++ 1.3 is matched scalar/default. No optimesh cluster LOD API.

| Function | scalar / 0.2.15 | default / 0.2.15 | scalar / optimesh | default / optimesh | scalar / scalar C++ | default / default C++ | max case scalar / default vs base |
|---|---:|---:|---:|---:|---:|---:|---:|
| simplify | 1.001 / 1.001 | 1.001 / 1.001 | 0.899 / 0.921 | 0.899 / 0.921 | 1.138 / 1.048 | 1.138 / 1.048 | 1.001950 / 1.001944 |
| simplify-attributes | 1.001 / 1.001 | 1.001 / 1.001 | 0.927 / 0.918 | 0.927 / 0.918 | 1.096 / 1.066 | 1.096 / 1.066 | 1.001165 / 1.001166 |
| simplify-sloppy | 1.000 / 1.000 | 1.000 / 1.000 | 0.706 / 0.882 | 0.706 / 0.882 | 1.275 / 1.268 | 1.275 / 1.268 | 1.000003 / 1.000001 |
| simplify-update | 0.940 / 0.986 | 0.940 / 0.986 | 0.947 / 0.959 | 0.947 / 0.959 | 1.203 / 1.070 | 1.203 / 1.070 | 0.992023 / 0.992023 |
| simplify-prune | 1.000 / 1.000 | 1.000 / 1.000 | 0.763 / 0.852 | 0.763 / 0.852 | 0.995 / 1.071 | 0.995 / 1.071 | 1.000005 / 1.000003 |
| simplify-points | 1.000 / 1.000 | 1.000 / 1.000 | 0.632 / 0.665 | 0.632 / 0.665 | 1.014 / 1.066 | 1.014 / 1.066 | 1.000003 / 1.000003 |
| cache | 1.000 / 1.000 | 1.000 / 1.000 | 0.919 / 0.911 | 0.919 / 0.911 | 1.254 / 1.242 | 1.254 / 1.242 | 1.000001 / 1.000002 |
| overdraw | 1.000 / 1.000 | 1.000 / 1.000 | 0.884 / 0.865 | 0.884 / 0.865 | 1.090 / 0.939 | 1.090 / 0.939 | 1.000137 / 1.000004 |
| fetch | 1.000 / 1.000 | 1.000 / 1.000 | 0.716 / 0.962 | 0.716 / 0.962 | 0.952 / 1.171 | 0.952 / 1.171 | 1.000004 / 1.000007 |
| meshlets | 1.000 / 1.000 | 1.000 / 1.000 | 0.784 / 0.778 | 0.784 / 0.778 | 1.001 / 1.068 | 1.001 / 1.068 | 1.000117 / 1.000001 |
| meshlets-flex | 1.000 / 1.000 | 1.000 / 1.000 | 0.784 / 0.778 | 0.784 / 0.778 | 1.002 / 1.068 | 1.002 / 1.068 | 1.000117 / 1.000000 |
| meshlets-spatial | 1.000 / 1.000 | 0.937 / 0.952 | 0.835 / 0.872 | 0.783 / 0.830 | 0.965 / 0.972 | 1.083 / 1.059 | 1.000149 / 1.000001 |
| cluster-lod | 1.000 / 0.997 | 0.999 / 0.993 | — | — | 1.189 / 1.172 | 1.188 / 1.167 | 0.999717 / 0.998673 |
| vertex-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.455 / 0.466 | 0.455 / 0.466 | 0.554 / 0.578 | 0.554 / 0.578 | 1.000003 / 1.000004 |
| vertex-decode | 1.000 / 1.000 | 0.911 / 0.880 | 0.544 / 0.644 | 0.174 / 0.200 | 1.061 / 1.114 | 1.145 / 1.237 | 1.000010 / 0.929348 |
| index-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.712 / 0.753 | 0.712 / 0.753 | 0.771 / 0.933 | 0.771 / 0.933 | 1.000006 / 1.000005 |
| index-decode | 1.000 / 1.000 | 1.000 / 1.000 | 0.907 / 0.915 | 0.934 / 0.951 | 0.994 / 1.044 | 1.024 / 1.085 | 1.000018 / 1.000015 |
| remap-vertex | 0.779 / 0.806 | 0.779 / 0.806 | 0.353 / 0.416 | 0.353 / 0.416 | 1.087 / 1.069 | 1.087 / 1.069 | 0.925878 / 0.925827 |
| remap-index | 0.885 / 0.883 | 0.885 / 0.883 | 0.863 / 0.871 | 0.863 / 0.871 | 1.437 / 1.386 | 1.437 / 1.386 | 0.885800 / 0.885842 |
| cluster-bounds | 0.981 / 0.950 | 0.932 / 0.889 | 0.734 / 0.801 | 0.697 / 0.750 | 1.138 / 1.146 | 1.081 / 1.072 | 0.982439 / 0.932300 |
| meshlet-bounds | 0.982 / 0.951 | 0.933 / 0.892 | 0.731 / 0.811 | 0.695 / 0.760 | 1.162 / 1.189 | 1.104 / 1.114 | 0.982738 / 0.933299 |

Campaign samples: 3504. Base/train output identities: 336/336.
Cases above 1.01: 0. Full per-case medians and ratios: [release-results.json](combined-ff38162/release-results.json).

Maximum individual base ratio: **1.001949930**, `real-214e` / `simplify` / `scalar_base`. No case exceeds 1.01. All 336 base/train and 336 matched-mode train/C++ output comparisons are equal. Ratios describe these registered inputs and owned-call workloads; no wall-time speedup is inferred from instruction counts.

## Parallel cluster LOD wall disclosure

Five interleaved AB/BA repetitions on each real mesh; medians in seconds. One and four workers use one and four distinct physical cores respectively. All 30 serialized callback outputs are equal to each other per mesh and to default serial cluster LOD. The clock surrounds the owned call, including output destruction, and excludes loading and serialization.

| Input | 1 worker, seconds | 4 workers, seconds | Speedup |
|---|---:|---:|---:|
| real-214e | 8.414459 | 5.243663 | 1.605x |
| real-a082 | 9.100415 | 5.417363 | 1.680x |
| real-b6fd | 2.399846 | 1.348241 | 1.780x |

[Raw wall samples](combined-ff38162/wall.jsonl) and [medians](combined-ff38162/wall-results.json).

## Streaming and repair cost disclosure

Corpus only, default build, median of three instructions:u samples. The repair ratios compare distinct calls with sanitize EMPTY on the same raw mesh: positions-only zero-tolerance weld, face orientation, and fill with a 16-edge maximum. Streaming decodes version-1 index sequences in 4096-byte chunks into owned u32 output; one-shot produces owned packed 32-bit bytes. Setup asserts equal decoded bytes. Allocation and destruction are counted; encoding, loading, validation of equal outputs and serialization are outside counters. These calls provide different functionality.

| Corpus input | weld / sanitize | orient / sanitize | fill / sanitize | streaming / one-shot |
|---|---:|---:|---:|---:|
| tiny-smooth | 1.328 | 3.321 | 8.646 | 2.521 |
| medium-smooth | 0.886 | 3.589 | 5.674 | 2.813 |
| medium-seams | 1.066 | 3.612 | 6.194 | 2.813 |
| medium-sparse | 89.939 | 3.589 | 5.675 | 2.813 |
| million-smooth | 0.860 | 3.776 | 5.755 | 2.823 |
| Corpus GM | 2.497 | 3.574 | 6.300 | 2.754 |

[Per-case instruction medians](combined-ff38162/cost-results.json).

## Gates, retained attempts and limits

| Gate | Exit |
|---|---:|
| All-feature and no-default exact lane drivers | 0 each |
| x86 C++ differentials, including forced SIMD | 0 each |
| LOD corpus identity, both input sets and feature modes | 0 each |
| Parallel identity: original grid and 1/4/16 workers, both modes | 0 each |
| Cargo tests and doctests, both feature modes | 0 each |
| Clippy all targets, both feature modes | 0 each |
| Formatting; package/unsafe boundary | 0 each |
| Instruction regression gate | 0; no ratio above 1.01 |

Final source/reference, public-text and whitespace checks, retained counter-file verification and binary/input bindings are recorded in [final-verification.json](combined-ff38162/final-verification.json) and [exits.jsonl](combined-ff38162/exits.jsonl). Disposable-target removal is recorded in [cleanup-final.json](combined-ff38162/cleanup-final.json).

The full staged whitespace audit exits 2 because six raw Cargo test/identity logs end with an extra blank line. Their recorded bytes and hashes are preserved. The staged audit excluding raw logs and the final public-text check exit 0; see [staged-review.json](combined-ff38162/staged-review.json).

Every build and execution entered CPU-only admission with declaration 4. Pre-run sizes and actual child/supervisor CPU are retained in [sizes.jsonl](combined-ff38162/sizes.jsonl), [job-sizing.jsonl](combined-ff38162/job-sizing.jsonl) and [jobs.jsonl](combined-ff38162/jobs.jsonl). Builds pause below the registered 100 GiB free-space floor. The largest completed campaign/proof job used 2726.259658 CPU seconds, below the 7200-second maximum.

The interrupted orchestration was resumed by waiting for its live processes; no duplicate campaign was launched and no queued or running job was canceled. Its counts were reused unchanged. The initial preparation attempt exited 1 before any build or comparison because retained old input links were broken; the hash-verified replacement input copy and successful retry are disclosed in [attempts.json](combined-ff38162/attempts.json). That infrastructure attempt is retained and is not a failed exact or measurement gate.

No fuzzers ran in this qualification. Fuzzing is a separate lane and is not verified here. Arm64 is supported by the source-identical Linux/Apple CI receipts, not local execution. Malformed C++ destination prefixes and upstream C++ NEON ISA divergences remain explicit disclosures above; no C++ failed-buffer byte-identity claim is made. Exact counts describe the original finite lane drivers rather than an exhaustive public-input universe.
