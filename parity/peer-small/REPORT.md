# SPEC-027 lane C

Baseline: v0.2.6, `33556c4`; branch `perf/small-gaps`, clean at entry.
Scratch: `<scratch>/meshopt-peer-c`.

## Cause milestone (before library edits)

`instructions-baseline-control-fixed.json` retains all 60 admitted, CPU-7
resident instruction measurements for ours, X (optimesh 1.1.0), and M7
(meshopt 0.7.0 / C++ 1.3). Counts exclude input loading and serialization.
Elapsed replies were discarded. Representative medium-smooth instructions/call:

| Family | v0.2.6 | X | M7 | Cause identified in source |
|---|---:|---:|---:|---|
| adjacency | 5,317,522 | 3,642,976 | 3,598,901 | Per-edge work checks and repeated dynamic probe-bound checks in both passes; position welding also uses the general checked hash kernel. |
| optimize-meshlet | 4,177 | 3,396 | 3,386 | General cache/valence/search and 256-entry remap setup for the frozen single-triangle workload; checked allocating wrapper overhead. |
| fetch | 516,961 | 444,555 | 362,437 | Scalar index validation plus separate copied-index scan and iterator-based owned-record growth. |
| remap-indices | 1,214,676 | 1,028,287 | 813,044 | Adapter includes vertex-remap generation; scalar validation and bounded hash batches precede the checked remap application. |

These are source-supported instruction-cost hypotheses, not cycle attribution.
Only these failing families are pursued. No optimesh code is copied. No API,
unsafe, SIMD, encoders, memory-policy or version changes are planned.

## Execution decisions

- Exported comparator manifests in scratch point at separate baseline and
  candidate source trees. The original comparator branch is untouched.
- First build failed closed with exit 101 because adapted paths required an
  offline lock refresh. Retained as `build-base-0.*`; subsequent builds use
  offline regeneration followed by `--locked`.
- Installed perf 6.17 is used because `/usr/bin/perf` cannot find the running
  kernel's tools. Initial counter-control attempts failed (exit 1); retained.
  The repaired control protocol strips perf's NUL-terminated acknowledgments
  and launches the resident child under perf rather than attaching a sibling.
- Registration precedes all wall samples. Exactly one final wall campaign,
  no favorable-verdict reruns. The tiny frozen meshlet and composed remap
  workloads are disclosed in REGISTRATION.md.

## Final implementation

- `src/remap.rs:158`: reuse the existing vectorizable index validator; invalid
  inputs and tight work limits replay the checked scalar visit sequence.
- `src/remap.rs:283`: select indexed versus linear remap traversal before the
  hash batches, reusing the existing specialized kernel. This is part of the
  frozen remap-indices producer chain; no public API changes.
- `src/remap.rs:1343`: bounded adjacency batches charge the same actual edge and
  probe visits once; insufficient bounds retain per-edge/per-probe checks.
  Canonical welded edge keys remove repeated remap loads during probing.
  First-insertion opposite vertices, hash order and signed-zero welding stay exact.
- `src/meshlet_util.rs:758`: zero/single-triangle path avoids cache/valence setup,
  preserving the exact charge sequence, duplicate corners and unused suffix.
- `src/remap.rs:1921,1961`: write packed fetch records through the existing
  fixed-width destination path; strided owned records use explicit growth.
  Allocation accounting, Result values and used output bytes remain exact.

No unsafe code, API, SIMD, encoder, memory-policy or version changes.

## Rejected instruction experiments

`instructions.json` retains every completed baseline and candidate experiment.
Fusing owned fetch index/record growth increased medium-smooth instructions to
523,911 and was reverted. Gathering remap output before sentinel validation
increased that family to 1,129,533 and was reverted. Passing only Work into
meshlet internals saved about three instructions and was dropped. No wall
samples informed these choices. The final candidate is `driver-candidate-4`.

Final medium-smooth instructions/call: adjacency 3,669,374; optimize-meshlet
3,946; fetch 422,252; remap-indices 901,790. These are diagnostics; the table
below determines acceptance.

## Registered result (first and only campaign)

Ratios are **ours/comparator elapsed time**, geometric means over five equally
weighted frozen cases. Lower is faster. Exactly one warm-up and ten interleaved
pairs per cell; bootstrap 10,000, seed 20261029, pinned CPU 7, a downstream engine scalar.
All 600 recorded pair samples and calibrations are retained in
`parity/peer-small/results.json` (20 A/A, 20 X and 20 baseline cells).

Same-session **F = 0.279861028**, so the registered case bar is
**upper95 <= 1.379861028**. The large floor reflects this shared-host
session; no quiet-host or tighter-noise claim is made. No rerun or threshold
change was used. Baseline GM <=1.00 and X GM <=1.00 are unchanged.

| Family | ours/X GM | ours/v0.2.6 GM | Worst baseline upper95 | Verdict |
|---|---:|---:|---:|---|
| adjacency | 1.011868 | 0.757203 | 0.816891 | **PARTIAL** |
| optimize-meshlet | 1.080442 | 0.891806 | 1.100304 | **PARTIAL** |
| fetch | 0.956749 | 0.754246 | 1.100486 | **PASS** |
| remap-indices | 1.041894 | 0.825730 | 1.017384 | **PARTIAL** |

All four families pass the baseline GM and every-case non-regression bars.
Only fetch clears X. The three PARTIAL families may ship as measured speedups,
with no claim that their performance equals X. This does not establish a
performance result for other families or broader meshlet workloads.

| Family | Case | ours/X GM | ours/v0.2.6 GM | Baseline 95% interval |
|---|---|---:|---:|---|
| adjacency | tiny-smooth | 0.941726 | 0.683136 | [0.629192, 0.729738] |
| adjacency | medium-smooth | 1.243563 | 0.810587 | [0.804074, 0.816891] |
| adjacency | medium-seams | 1.003296 | 0.768153 | [0.758554, 0.780799] |
| adjacency | medium-sparse | 0.911697 | 0.750325 | [0.745650, 0.755938] |
| adjacency | million-smooth | 0.990255 | 0.779933 | [0.732352, 0.814705] |
| optimize-meshlet | tiny-smooth | 1.034284 | 0.877203 | [0.872300, 0.882092] |
| optimize-meshlet | medium-smooth | 1.250797 | 0.932537 | [0.826962, 1.035871] |
| optimize-meshlet | medium-seams | 1.111799 | 0.874600 | [0.857546, 0.891545] |
| optimize-meshlet | medium-sparse | 0.994546 | 0.982201 | [0.879455, 1.100304] |
| optimize-meshlet | million-smooth | 1.029272 | 0.802742 | [0.721571, 0.878626] |
| fetch | tiny-smooth | 0.997899 | 0.946815 | [0.839011, 1.100486] |
| fetch | medium-smooth | 0.929276 | 0.642005 | [0.593410, 0.685502] |
| fetch | medium-seams | 0.909371 | 0.665306 | [0.591932, 0.748054] |
| fetch | medium-sparse | 0.989807 | 0.720881 | [0.683876, 0.765426] |
| fetch | million-smooth | 0.960436 | 0.837293 | [0.800382, 0.874777] |
| remap-indices | tiny-smooth | 0.992301 | 0.793676 | [0.728591, 0.844767] |
| remap-indices | medium-smooth | 1.013732 | 0.753678 | [0.636834, 0.853475] |
| remap-indices | medium-seams | 0.979086 | 0.843399 | [0.815199, 0.859694] |
| remap-indices | medium-sparse | 1.043864 | 0.832707 | [0.799121, 0.852614] |
| remap-indices | million-smooth | 1.194227 | 0.913767 | [0.831743, 1.017384] |

## Targeted proof and gate exits

- fmt: **0** (`cargo fmt --check`).
- tests: **0** (`cargo test --locked --all-features`).
- clippy: **0** (`cargo clippy --locked --all-targets --all-features -- -D warnings`).
- proof build: **0**; oracle checked clean at
  `4c203430ca565cb59a468a91922c76c208169536` (C++ 1.3).
- targeted differential and exact comparison: **0**, 1,489 retained cases,
  including 20 frozen C++/baseline/candidate cells and 188 existing native/JS/
  geometry/seed protocol C++ comparisons. Also 1,024 inputs from existing
  preprocessing fuzz seeds, and the existing p03 smoke seed plus 256 mutations.
  There were 241 Result Err cases; errors match the baseline exactly.
- exhaustive limits: **0**, 1,152,000 small API/variant/limit observations plus
  432 typed-fetch threshold observations, comparing public Results, usage,
  failure-side destinations and atomic in-place buffers. Both compressed streams
  have the same uncompressed SHA-256, recorded in `proof-summary.json`.
- registered campaign: **0**; source/proof/registration/600-sample audit: **0**.

The initial build (101) and counter setup failures (1, 1) are retained and
resolved as described above. The earlier successful gate run was repeated
because library changes followed it; only the final matching-source run is
used here. No 10k sweeps or long fuzz run; those remain nightly per spec.

`parity/peer-small/` retains reproducible proof/measurement scripts, all raw
wall samples, instruction counts, source/binary/input hashes and compact proof
receipts. Full raw proof inputs/outputs, limit streams, build logs and rejected
experiments remain in scratch. The candidate build receipt, proof source hashes
and live source were checked identical before the registered campaign and again
by the final audit. The comparator branch and other lanes were untouched.
