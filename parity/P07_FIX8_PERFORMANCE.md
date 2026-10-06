# P07 fix-eight — continued, validated and measured

**Overall RFC closure is not established.** The continuation executed all reopened
counter analyses, validated the Exp/Oct candidates, retained the proven WASM
reconstruction fix and completed the ONE final paired timing epoch. Native
allocating vertex retains its prior failed maximum after fresh analysis found
no isolated allocating cause. All other failures below are current and retained.

A = allocating, C = caller-buffer. Cells contain geometric mean / largest final
95% maximum upper bound / verdict. RFC: mean <=1.25 and every final maximum
upper <=1.50. Stage-one medians define means; only borderline maxima receive
30 fresh D146 pairs. New/old is paired in the same bursts. An unresolved interval
fails. Historical round-seven results remain in P07_FIX7_PERFORMANCE.md.

| Platform / family | A mean / max upper / RFC | C mean / max upper / RFC | Paired new/old A / C |
|---|---|---|---|
| native / exp | 0.942 / 1.403 / pass | 0.945 / 1.392 / pass | 0.977 / 0.989 |
| native / oct | 0.958 / 1.538 / **FAIL** | 0.914 / 1.556 / **FAIL** | 0.945 / 0.948 |
| native / view-filtered | 1.084 / 2.316 / **FAIL** | 0.965 / 1.485 / pass | 0.998 / 0.969 |
| wasm / vertex | 0.989 / 2.065 / **FAIL** | 0.977 / 1.498 / pass | 0.877 / 0.851 |
| wasm / view-none | 1.140 / 2.148 / **FAIL** | 1.002 / 1.465 / pass | 0.865 / 0.879 |
| wasm / view-filtered | 0.996 / 1.754 / **FAIL** | 0.979 / 1.747 / **FAIL** | 0.814 / 0.756 |
| native / vertex (prior only) | 1.124 / 2.847 / **FAIL** | 1.081 / <=1.50 / prior pass | unmeasured |

## Current failed maximum intervals

| Platform | API | Case | Final 95% Rust/C++ | Final 95% new/old |
|---|---|---|---|---|
| native | allocating | varied-filter-1-streaming-s4 | 1.3843–1.5384 | 0.9922–1.0720 |
| native | caller-buffer | varied-filter-1-streaming-s4 | 1.3493–1.5559 | 0.9269–1.0329 |
| native | allocating | varied-view-3-streaming-s32 | 2.0078–2.3157 | 0.8593–1.0500 |
| wasm | allocating | vertex-v0-resident-s32 | 1.4739–2.0650 | 0.8094–1.4237 |
| wasm | allocating | vertex-v1-resident-s32 | 1.3259–1.5219 | 1.0721–1.1655 |
| wasm | allocating | view-none-tiny-s32 | 1.3767–1.5100 | 0.5641–1.0892 |
| wasm | allocating | view-none-resident-s32 | 1.3510–2.1479 | 0.7795–1.4967 |
| wasm | allocating | varied-view-3-resident-s32 | 1.6741–1.7539 | 1.0479–1.1867 |
| wasm | caller-buffer | varied-view-3-resident-s32 | 1.5159–1.7472 | 1.0880–1.1557 |

The unchanged native allocating vertex-v1-streaming-s4 interval is historical
1.538–2.847. Its instruction work is 6.1067/byte versus caller 6.1065 and
C++ ~5.8312; both baselines initialize output. Retained asm and fresh compiled
counters isolate no allocating mechanism. The inherited first-analysis kill
rule applies now on fresh evidence; its prior failure is not a new timing result.
Native filtered-view allocation similarly has no isolated throughput cause.
It is included in the final epoch because Oct's actual tail implementation changed.

## Final S3

- caller-buffer / varied-filter-3-tiny-s32: 1.0390–1.1085.

No S3 rescue or second timing epoch was taken. The native Exp size branch was
removed: allocating tiny12 instructions 2.7059 ->2.6813, caller 2.0392 ->2.0148.
Oct's exact hardware short-tail root reduces caller tiny4 instructions
12.0443 ->11.0442 and allocating 14.0444 ->13.0441, with frozen bytes unchanged.
These instruction effects do not independently establish elapsed acceptance.

## Reopened WASM diagnosis and retained fix

FIFO-controlled perf counters start after eager TurboFan warmup and stop before
hashing; CPU26, raw/public, both APIs, N/2N. The prior tiering/GC-contaminated
process subtraction was not reused as a kill decision. Decoder work dominates
the much smaller public-copy/allocation seam. Retained instruction sampling
attributes 23.13% of unfiltered streaming12 work to an out-of-line scatter4 helper.
Its JIT frame, call setup, dynamic stride branch and four checks are retained.

Plain store inlining was rejected: 12.2264 ->14.6708 instructions/byte on
unfiltered allocating streaming12. The retained candidate also propagates the
existing fixed vertex stride through reconstruction. All 32 residual comparisons
(eight cases x A/C x raw/public) reduce instructions and match frozen hashes:
raw A unfiltered12 12.2264 ->8.5161; repeated Oct8 9.1261 ->5.4060;
repeated Exp12 8.7965 ->5.0839; varied Oct4 13.4230 ->10.5966,
Oct8 13.1384 ->9.4188, Quat8 13.4533 ->9.7267,
Exp12 resident 11.4762 ->7.7643 and streaming 11.4232 ->7.7105.
No arithmetic reassociation, approximation, new unsafe block or allocation
contract change. Generic strides and checked stores remain.

The successful WASM counter effects cover reopened strides 4/8/12 shapes.
The remaining maxima are stride32 cases using generic reconstruction, whose
JIT body retains dynamic stride work. Those counters do not establish a cause
or a fix for the new stride32 maxima. Paired controls establish regressions for
WASM allocating v1 resident32 (1.0721–1.1655 new/old), and varied Exp
resident32 A/C (1.0479–1.1867 / 1.0880–1.1557). These are retained failures;
the candidate is not an RFC-qualified release fix. No source edit or rescue timing follows
the single epoch. The original tiny Exp12 S3 clears; tiny Exp32 C remains.

## Qualification and receipts

Source fixes: 5b9da67 (Exp), 513340c (Oct), 0531e15 (WASM reconstruction).
Final source/binary manifests, counters, assembly and raw timing pairs are in
`/mnt/linux-extra/meshopt-artifacts/p07-fix8`. Verified frozen execution covers
869 fixtures, 7653 malformed cases and 138 benchmark identities on all local
native ceilings and scalar/SIMD WASM, both APIs. 438 exact timing-module outputs
include the retained Node control. Ordinary/all-feature and unsafe-free tests,
native/WASM Clippy, MSRV/no-std SIMD, scoped Oct-tail Miri, formatting, the
23-block safety audit and package inventory audit pass.

The single final epoch uses declared 4GB heavy/GPU admission, 840-second
timeouts, 690-second pair checkpoints, 5–20 early stops, borderline-only fresh
D146, and independently verified round-seven controls in every pair.
- native: native-final-burst1.log, wrapper 845.4s including queue, exit 0.
- wasm: wasm-final-burst1.log, wrapper 940.9s including queue, exit 0.

The previous disk-derived abandonments are void. The 25-GiB gate reopened
during a bounded 120-second/90-minute wait; every Cargo build ran sequentially
with -j4 and no incremental artifacts. Only the exact fix8 target is deleted
at final cleanup. Final absence and process/exit receipts are bound by
`continuation-receipt.json`; milestones and scout remain under
`/mnt/linux-extra/moss-scratch/p07-fix8`.

No native vertex retiming, untouched-family retiming, ARM/other-platform,
release, Moss integration or full-matrix acceptance is implied.
