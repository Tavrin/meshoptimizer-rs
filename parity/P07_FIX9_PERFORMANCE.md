# P07 fix9 — one paired epoch

**Scoped RFC/S3 pass: False.** No untouched-family, other-platform, release or Moss integration acceptance is implied.

RFC: family geometric mean <=1.25; every final 95% maximum upper bound <=1.50. Unresolved intervals fail. Stage-one medians define means; borderline maxima alone receive 30 fresh D146 pairs. Fix8 control (6035fc3 source identity) is interleaved within each burst.

| Platform / family | A mean / max upper / verdict | C mean / max upper / verdict | Paired new/old A / C |
|---|---|---|---|
| native / vertex | 1.099 / 2.103 / **FAIL** | 1.044 / 1.467 / pass | 0.961 / 0.973 |
| native / view-none | 1.102 / 1.452 / pass | 1.063 / 1.438 / pass | 1.009 / 0.971 |
| native / exp | 0.903 / 1.428 / pass | 0.922 / 1.114 / pass | 0.993 / 0.955 |
| native / oct | 0.971 / 1.389 / pass | 0.839 / 1.475 / pass | 1.005 / 0.987 |
| native / view-filtered | 1.083 / 2.122 / **FAIL** | 0.969 / 1.453 / pass | 0.988 / 0.980 |
| wasm / vertex | 0.924 / 1.511 / **FAIL** | 0.911 / 1.442 / pass | 0.958 / 0.917 |
| wasm / view-none | 0.812 / 2.475 / **FAIL** | 0.942 / 1.485 / pass | 0.980 / 0.899 |
| wasm / view-filtered | 0.954 / 1.488 / pass | 0.989 / 1.498 / pass | 0.981 / 0.975 |

## Requested residual cases

| Platform | Case | API | Final Rust/C++ interval | Paired new/old interval | Verdict |
|---|---|---|---|---|---|
| native | varied-filter-1-streaming-s4 | allocating | 1.3122–1.3888 | 0.8839–0.9884 | pass |
| native | varied-view-3-streaming-s32 | allocating | 1.7607–2.1222 | 0.8640–1.0066 | **FAIL** |
| native | varied-filter-1-streaming-s4 | caller-buffer | 1.1620–1.4748 | 0.9411–0.9710 | pass |
| native | varied-view-3-streaming-s32 | caller-buffer | 0.7456–1.4420 | 0.8203–1.3414 | pass |
| wasm | view-none-tiny-s32 | allocating | 0.7207–0.8663 | 0.7890–0.9687 | pass |
| wasm | vertex-v0-resident-s32 | allocating | 0.7253–1.0974 | 0.5496–0.7543 | pass |
| wasm | view-none-resident-s32 | allocating | 0.8316–0.9877 | 0.5625–0.8436 | pass |
| wasm | vertex-v1-resident-s32 | allocating | 0.8638–0.9398 | 0.5326–0.8438 | pass |
| wasm | varied-view-3-resident-s32 | allocating | 0.7811–0.9270 | 0.6318–0.7258 | pass |
| wasm | view-none-tiny-s32 | caller-buffer | 1.0378–1.4851 | 0.7256–1.0696 | pass |
| wasm | vertex-v0-resident-s32 | caller-buffer | 0.8849–0.9865 | 0.6038–0.6618 | pass |
| wasm | view-none-resident-s32 | caller-buffer | 0.8101–0.9208 | 0.6057–0.7979 | pass |
| wasm | vertex-v1-resident-s32 | caller-buffer | 0.7760–1.1429 | 0.6307–0.6903 | pass |
| wasm | varied-view-3-resident-s32 | caller-buffer | 0.7773–0.9755 | 0.5836–0.6967 | pass |

## Every measured failed maximum

| Platform | API | Case | Final interval |
|---|---|---|---|
| native | allocating | vertex-v1-streaming-s4 | 1.8501–2.1032 |
| native | allocating | varied-view-3-streaming-s32 | 1.7607–2.1222 |
| wasm | allocating | vertex-v0-tiny-s4 | 0.7535–1.5114 |
| wasm | allocating | view-none-tiny-s12 | 1.2442–2.4748 |

## Native S3

- tiny Exp32 allocating: Rust/current scalar 0.9450–1.1191.
- tiny Exp32 caller-buffer: Rust/current scalar 0.9713–1.1112.

Applicable significant slowdowns: 1.

- allocating / varied-filter-3-tiny-s12: 1.0190–1.2718.

## Diagnosis and retained fixes

Four-byte lane stores already match upstream; widening stores would overwrite other components. Generic stride32 addresses, checks, dynamic stride branches and spilled native addresses account for the proven instruction work. Constant32 propagation retains checked slices and exact arithmetic. Native target instructions 6.7338 ->6.2174/byte; WASM resident v0/v1/varied Exp ~14.7/14.9/13.9 ->8.58/8.74/7.76, both APIs, with exact frozen hashes. The native A/C counters are equal and do not independently explain the historical allocating-only elapsed maximum.

Oct retains correctly rounded sqrt/div and canonical operation order. Integer-derived components have +0 for every zero, so sign-bit rounding bias removes three redundant comparisons. Native Oct4 instructions 4.9847 ->4.7348/byte. Native Exp already uses canonical scalar at every size; no redundant size threshold was added. Its public filter wrapper now specializes the filter kind before validation/apply, reducing tiny caller instructions 1.1912 ->1.0809/byte.

Fix commits 8ffa2b9 (stride32), 14069f8 (Oct), e94365e (Exp wrapper). No new unsafe blocks or arithmetic approximation.

## Qualification and receipts

Fresh frozen execution: 869 fixtures, 7653 malformed inputs, 138 benchmark identities on all local native ceilings and scalar/SIMD WASM, both APIs, plus retained native control. 438 exact Node timing-module outputs include retained fix8 control. Native Oct8 all 16,777,216 triples, one million seeded Oct16 records, arithmetic edges and extended group/tail regression pass. Required all-feature/unsafe-free tests, native/WASM Clippy, native/WASM MSRV no-std SIMD, Miri/full group, formatting, 23-block boundary and package inventory pass. The initially missing Rust1.88 WASM target is recorded in failed-check-attempts.json and corrected by installing it and rerunning the check.

Artifacts, bound native asm/WASM disassembly and WAT, raw counters, raw timing pairs, controller checksums and manifests: `/mnt/linux-extra/meshopt-artifacts/p07-fix9`. Milestones/scout: `/mnt/linux-extra/moss-scratch/p07-fix9`. Build gate 25 GiB, target codex-p07-fix9, sequential -j4/no incremental. One final epoch uses declared 4GB heavy/GPU admission, 840-second timeouts and 690-second pair checkpoints. Accepted pairs are resumed without repetition.

- native burst1: wrapper 1091.8s including queue; exit 0; native-final-burst1.log.
- wasm burst1: wrapper 650.1s including queue; exit 0; wasm-final-burst1.log.

The requested WASM stride32 targets all pass, with paired significant
improvements on both resident vertex versions, resident view-none and varied
Exp-view32. The new final maximum failures elsewhere are allocating WASM
vertex-v0-tiny-s4 (upper1.5114) and view-none-tiny-s12 (upper2.4748).
Their paired new/old intervals include1, so this epoch does not prove a
regression against the control. Allocating native vertex-v1-streaming-s4
remains failed on fresh measurement; allocating filtered streaming32 remains
failed despite the counter effect. All measured family means pass.

Tiny Exp32 C clears S3, but allocating varied-filter3 tiny12 has S3
1.0190–1.2718; its paired new/old0.8614–1.0388 does not prove regression
against the control. These are retained failures. No second timing epoch,
post-timing engine edit or performance rescue is taken. RFC/S3 closure is
not established and this is not a release-qualified fix.

Cleanup: exact codex-p07-fix9 target removed after all owned jobs were reaped.
Final absence, exits and artifact hashes are bound in final-receipt.json.
