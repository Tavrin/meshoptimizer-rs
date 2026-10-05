# P07 fix-seven RFC performance and paired controls

Owner RFC bar only: geometric mean <=1.25 and maximum <=1.50 per family/API.
An unresolved final 95% maximum interval fails. Applicable native varied-filter
S3 also requires no significant slowdown versus current safe scalar Rust.
Registered stricter thresholds are deferred to 0.3; SIMD_BAR.md is unchanged.

A = allocating; C = caller-buffer. Rust/C++ time ratios; lower is faster.
Before is the round-six historical epoch. New/old is measured within the same
admitted pairs as new/C++; stage-one medians determine family means, and only
borderline maxima receive thirty fresh D146 pairs. No S3 rescue sampling.

| Platform / family | A before → after | C before → after | Paired new/old A / C | RFC A / C |
|---|---:|---:|---:|---|
| native / exp | 0.917 → 0.996 | 1.065 → 0.935 | 0.999 / 0.989 | **FAIL** / pass |
| native / meshlet | 1.221 → 1.174 | 1.409 → 1.244 | 0.969 / 0.877 | pass / pass |
| native / meshlet-raw | 1.088 → 1.099 | 1.306 → 1.190 | 1.003 / 0.951 | pass / pass |
| native / oct | 1.125 → 0.975 | 1.193 → 0.882 | 0.848 / 0.758 | pass / **FAIL** |
| native / quat | 1.170 → 0.931 | 1.235 → 0.790 | 0.755 / 0.624 | pass / pass |
| native / vertex | 1.141 → 1.124 | 1.087 → 1.081 | 0.981 / 0.979 | **FAIL** / pass |
| native / view-filtered | 1.162 → 1.044 | 1.107 → 1.002 | 0.917 / 0.896 | **FAIL** / pass |
| native / view-none | 1.061 → 1.076 | 1.073 → 1.067 | 0.964 / 1.004 | pass / pass |
| wasm / sequence | 0.899 → 0.913 | 0.839 → 0.859 | 1.021 / 1.006 | pass / pass |
| wasm / vertex | 1.229 → 1.203 | 1.089 → 1.118 | 1.056 / 1.029 | pass / pass |
| wasm / view-filtered | 1.369 → 1.281 | 1.360 → 1.288 | 0.955 / 0.957 | **FAIL** / **FAIL** |
| wasm / view-none | 1.092 → 1.325 | 1.168 → 1.112 | 1.192 / 1.016 | **FAIL** / pass |

## Paired control baseline

Historical before/after ratios also contain epoch drift. The retained control
below is measured against C++ in the current pairs; new/old above isolates
the candidate relative to that control. Every admitted result remains valid
for this scoped epoch; regressions are not discarded as noise.

| Platform / family | Current old/C++ A | Current old/C++ C |
|---|---:|---:|
| native / exp | 0.999 | 0.947 |
| native / meshlet | 1.215 | 1.422 |
| native / meshlet-raw | 1.108 | 1.239 |
| native / oct | 1.145 | 1.170 |
| native / quat | 1.229 | 1.259 |
| native / vertex | 1.148 | 1.101 |
| native / view-filtered | 1.136 | 1.120 |
| native / view-none | 1.124 | 1.077 |
| wasm / sequence | 0.919 | 0.849 |
| wasm / vertex | 1.143 | 1.080 |
| wasm / view-filtered | 1.348 | 1.345 |
| wasm / view-none | 1.108 | 1.088 |

## Decisions and limits

Retained fixes: exact safe one-record meshlets; canonical shared Exp lowering
with <=4-MiB/stride12 scalar dispatch and large other-stride SSE2; packed
canonical repeated Oct/Quat run copies; native stride4 and wasm stride4/8/12
vertex specialization; forced canonical Oct/Quat tail inlining. No arithmetic
reassociation, approximation, frozen-input change or new unsafe block.

Rejected: meshlet byte-output/raw cursor rewrite (more raw instructions),
padded SIMD tail recursion (more tiny cycles and varied wasm work), ordinary
tail inlining (no instruction gain), and further unproven parser/filter tuning.
Every prior maximum receives a classification and an outcome below. The
single final pass is a scoped stop; any residual failure prevents RFC closure.

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07-fix7`. Verified old-control
hashes bind to source c87431d; current binaries bind to build.json. Frozen
869 fixture and 7653 malformed cases, all 138 benchmark identities, both APIs,
local native ceilings and actual scalar/SIMD wasm match round-six statuses and
successful bytes. Timing-module preflight also includes the retained Node
control. Safety/Clippy/Miri, MSRV/no-std and package receipts are retained.
The changed explicit-Exp Miri unit gate passes; the successful seven-test
ISA integration receipt is retained with independently checked unchanged
production/test code. Its Miri-only NaN comparison permits only a NaN sign
difference; native assertions retain byte equality. No production change.
Exhaustive Oct8 and finite Exp/Oct16/Quat checks are scoped arithmetic proof,
with separate native large-Exp threshold/frozen streaming checks. This does
not requalify ARM, other platforms, the full native matrix, Moss integration,
native triangle-index, phase-0.6, or release acceptance.

## Final S3 significant slowdowns

- allocating / varied-filter-3-tiny-s12: 1.1431–1.1751.

## The four original S3 cases

| API | Case | Final SIMD/scalar 95% | S3 |
|---|---|---|---|
| allocating | varied-filter-3-streaming-s12 | 0.9690–1.0270 | pass |
| caller-buffer | varied-filter-3-tiny-s12 | 0.9588–1.0210 | pass |
| caller-buffer | varied-filter-3-tiny-s32 | 0.9962–1.1304 | pass |
| caller-buffer | varied-filter-3-resident-s32 | 0.9919–1.0116 | pass |

All four original S3 rows pass their current comparison. The separate new
allocating tiny stride12 Exp slowdown remains a failure. Large explicit Exp
streaming32 also has a new allocating maximum failure. WASM allocating
view-none-streaming-s12 regresses (paired family new/old 1.192); no cause
beyond size-dependent decode/layout throughput is established. Retain it
and the remaining varied/repeated Exp view failures; further tuning is
abandoned at the single-epoch stop.

## Every prior unresolved/failed RFC maximum

| Platform | API | Case | Diagnosis / change | Final interval | RFC max |
|---|---|---|---|---|---|
| native | allocating | view-1-streaming-s4 | size-dependent repeat/filter throughput; packed run + shape | 1.568–1.746 | **FAIL** |
| native | allocating | vertex-v1-streaming-s4 | size-dependent stride4 decode; shape specialization | 1.538–2.847 | **FAIL** |
| native | allocating | varied-view-3-streaming-s32 | size-dependent decode/filter throughput; large Exp SSE2 | 0.744–1.308 | pass |
| native | caller-buffer | varied-filter-1-tiny-s4 | fixed tiny tail/setup; canonical tail inlining | 1.571–1.696 | **FAIL** |
| native | caller-buffer | varied-filter-2-tiny-s8 | fixed tiny tail/setup; canonical tail inlining | 1.323–1.343 | pass |
| native | caller-buffer | varied-filter-3-tiny-s12 | fixed tiny setup; canonical shared scalar dispatch | 0.822–0.867 | pass |
| native | caller-buffer | varied-filter-3-tiny-s32 | fixed tiny setup; canonical shared scalar dispatch | 1.069–1.173 | pass |
| native | caller-buffer | meshlet-1-1-v2-t3 | fixed setup; single-record decoder | 1.166–1.332 | pass |
| native | caller-buffer | meshlet-1-1-v2-t4 | fixed setup; single-record decoder | 1.221–1.262 | pass |
| native | caller-buffer | meshlet-1-1-v4-t3 | fixed setup; single-record decoder | 1.155–1.305 | pass |
| native | caller-buffer | meshlet-1-1-v4-t4 | fixed setup; single-record decoder | 1.215–1.251 | pass |
| wasm | allocating | view-1-resident-s4 | size-dependent repeat/filter throughput; packed run + shape | 0.975–1.325 | pass |
| wasm | allocating | view-1-resident-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.516–1.797 | **FAIL** |
| wasm | allocating | view-2-resident-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.477–1.609 | **FAIL** |
| wasm | allocating | view-3-resident-s12 | size-dependent decode/Exp throughput; shape; further tuning abandoned | 1.878–2.151 | **FAIL** |
| wasm | allocating | view-1-streaming-s4 | size-dependent repeat/filter throughput; packed run + shape | 0.690–1.411 | pass |
| wasm | allocating | view-1-streaming-s8 | size-dependent repeat/filter throughput; packed run + shape | 0.887–1.489 | pass |
| wasm | allocating | view-2-streaming-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.064–1.479 | pass |
| wasm | allocating | varied-view-1-resident-s4 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.504–1.673 | **FAIL** |
| wasm | allocating | varied-view-1-resident-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.518–1.629 | **FAIL** |
| wasm | allocating | varied-view-1-streaming-s4 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.388–1.703 | **FAIL** |
| wasm | allocating | varied-view-1-streaming-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.306–1.696 | **FAIL** |
| wasm | allocating | varied-view-2-streaming-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.290–1.679 | **FAIL** |
| wasm | caller-buffer | view-1-resident-s4 | size-dependent repeat/filter throughput; packed run + shape | 0.837–0.982 | pass |
| wasm | caller-buffer | view-1-resident-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.306–1.498 | pass |
| wasm | caller-buffer | view-2-resident-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.298–1.365 | pass |
| wasm | caller-buffer | view-3-resident-s12 | size-dependent decode/Exp throughput; shape; further tuning abandoned | 1.650–1.762 | **FAIL** |
| wasm | caller-buffer | view-1-streaming-s4 | size-dependent repeat/filter throughput; packed run + shape | 0.855–0.947 | pass |
| wasm | caller-buffer | view-1-streaming-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.285–1.356 | pass |
| wasm | caller-buffer | view-2-streaming-s8 | size-dependent repeat/filter throughput; packed run + shape | 1.091–1.400 | pass |
| wasm | caller-buffer | view-3-streaming-s12 | size-dependent decode/Exp throughput; shape; further tuning abandoned | 1.506–1.623 | **FAIL** |
| wasm | caller-buffer | varied-view-1-resident-s4 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.525–1.689 | **FAIL** |
| wasm | caller-buffer | varied-view-1-resident-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.500–1.660 | **FAIL** |
| wasm | caller-buffer | varied-view-2-resident-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.487–1.551 | **FAIL** |
| wasm | caller-buffer | varied-view-1-streaming-s4 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.580–1.693 | **FAIL** |
| wasm | caller-buffer | varied-view-1-streaming-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.578–1.628 | **FAIL** |
| wasm | caller-buffer | varied-view-2-streaming-s8 | size-dependent varied decode/filter throughput; shape; further tuning abandoned | 1.504–1.594 | **FAIL** |

## Timing receipts

- native: native-timing-burst1.log, wrapper 211s, exit zero.
- wasm: wasm-timing-burst1.log, wrapper 156s, exit zero.

## Every native row

| API | Case | New/C++ | Paired new/old | Final 95% new/C++ | Final 95% new/old | Pairs | RFC max |
|---|---|---:|---:|---|---|---|---|
| allocating | vertex-v0-tiny-s4 | 1.148 | 0.830 | 1.079–1.304 | 0.727–1.009 | 5 | pass |
| allocating | view-none-tiny-s4 | 1.136 | 0.878 | 1.095–1.183 | 0.837–0.896 | 5 | pass |
| allocating | vertex-v0-tiny-s12 | 1.172 | 1.032 | 1.070–1.301 | 0.959–1.105 | 5 | pass |
| allocating | view-none-tiny-s12 | 1.126 | 0.970 | 0.831–1.258 | 0.791–1.089 | 5 | pass |
| allocating | vertex-v0-tiny-s32 | 1.057 | 1.028 | 1.050–1.212 | 0.920–1.088 | 5 | pass |
| allocating | view-none-tiny-s32 | 1.246 | 1.007 | 1.079–1.430 | 0.918–1.123 | 5 | pass |
| allocating | filter-1-tiny-s4 | 1.085 | 0.891 | 1.049–1.135 | 0.633–1.028 | 5 | pass |
| allocating | view-1-tiny-s4 | 1.086 | 0.871 | 1.073–1.207 | 0.804–0.918 | 5 | pass |
| allocating | filter-1-tiny-s8 | 0.900 | 0.895 | 0.898–0.940 | 0.849–0.929 | 5 | pass |
| allocating | view-1-tiny-s8 | 1.059 | 0.916 | 0.967–1.114 | 0.753–1.003 | 5 | pass |
| allocating | filter-2-tiny-s8 | 1.012 | 0.751 | 0.934–1.136 | 0.569–0.943 | 5 | pass |
| allocating | view-2-tiny-s8 | 1.096 | 0.903 | 1.065–1.150 | 0.858–0.940 | 5 | pass |
| allocating | filter-3-tiny-s12 | 0.793 | 1.018 | 0.746–0.867 | 0.914–1.076 | 5 | pass |
| allocating | view-3-tiny-s12 | 1.037 | 1.006 | 0.995–1.351 | 0.965–1.052 | 5 | pass |
| allocating | vertex-v0-resident-s4 | 0.940 | 0.921 | 0.908–1.031 | 0.872–0.969 | 5 | pass |
| allocating | view-none-resident-s4 | 1.010 | 0.939 | 0.932–1.106 | 0.864–1.059 | 5 | pass |
| allocating | vertex-v0-resident-s12 | 1.148 | 0.996 | 0.999–1.369 | 0.931–1.097 | 5 | pass |
| allocating | view-none-resident-s12 | 1.131 | 0.990 | 0.951–1.491 | 0.854–1.261 | 5 | pass |
| allocating | vertex-v0-resident-s32 | 1.173 | 1.010 | 1.124–1.172 | 0.907–1.202 | 5 | pass |
| allocating | view-none-resident-s32 | 1.105 | 1.023 | 0.998–1.179 | 0.942–1.058 | 5 | pass |
| allocating | filter-1-resident-s4 | 0.510 | 0.543 | 0.494–0.527 | 0.507–0.565 | 5 | pass |
| allocating | view-1-resident-s4 | 0.658 | 0.641 | 0.541–1.078 | 0.493–0.941 | 5 | pass |
| allocating | filter-1-resident-s8 | 0.435 | 0.482 | 0.423–0.458 | 0.354–0.568 | 5 | pass |
| allocating | view-1-resident-s8 | 0.725 | 0.715 | 0.697–0.775 | 0.647–0.784 | 5 | pass |
| allocating | filter-2-resident-s8 | 0.323 | 0.292 | 0.312–0.358 | 0.250–0.331 | 5 | pass |
| allocating | view-2-resident-s8 | 0.724 | 0.603 | 0.585–1.055 | 0.539–0.765 | 5 | pass |
| allocating | filter-3-resident-s12 | 0.800 | 0.988 | 0.665–1.097 | 0.852–1.275 | 5 | pass |
| allocating | view-3-resident-s12 | 1.054 | 1.018 | 1.024–1.113 | 0.909–1.067 | 5 | pass |
| allocating | vertex-v0-streaming-s4 | 0.994 | 0.976 | 0.941–1.056 | 0.759–1.114 | 5 | pass |
| allocating | view-none-streaming-s4 | 0.971 | 0.917 | 0.883–1.044 | 0.662–1.067 | 5 | pass |
| allocating | vertex-v0-streaming-s12 | 1.023 | 0.931 | 0.890–1.288 | 0.843–1.109 | 5 | pass |
| allocating | view-none-streaming-s12 | 1.066 | 1.009 | 1.026–1.099 | 0.727–1.162 | 5 | pass |
| allocating | vertex-v0-streaming-s32 | 0.941 | 0.983 | 0.918–0.954 | 0.872–1.038 | 5 | pass |
| allocating | view-none-streaming-s32 | 0.928 | 0.956 | 0.757–1.021 | 0.902–1.002 | 5 | pass |
| allocating | filter-1-streaming-s4 | 0.770 | 0.792 | 0.416–1.042 | 0.623–0.896 | 5 | pass |
| allocating | view-1-streaming-s4 | 1.469 | 0.799 | 1.568–1.746 | 0.792–0.865 | 20+30 | **FAIL** |
| allocating | filter-1-streaming-s8 | 0.928 | 0.908 | 0.651–1.224 | 0.695–1.178 | 5 | pass |
| allocating | view-1-streaming-s8 | 0.837 | 0.879 | 0.754–0.897 | 0.843–0.898 | 5 | pass |
| allocating | filter-2-streaming-s8 | 0.875 | 0.860 | 0.716–1.213 | 0.783–1.067 | 5 | pass |
| allocating | view-2-streaming-s8 | 0.853 | 0.855 | 0.748–0.978 | 0.747–0.987 | 5 | pass |
| allocating | filter-3-streaming-s12 | 1.044 | 1.037 | 1.001–1.119 | 0.977–1.119 | 5 | pass |
| allocating | view-3-streaming-s12 | 0.976 | 1.004 | 0.942–0.993 | 0.983–1.023 | 5 | pass |
| allocating | vertex-v1-tiny-s4 | 1.127 | 0.905 | 1.104–1.189 | 0.879–0.941 | 5 | pass |
| allocating | vertex-v1-tiny-s12 | 1.151 | 0.987 | 1.106–1.221 | 0.958–1.054 | 5 | pass |
| allocating | vertex-v1-tiny-s32 | 1.124 | 0.962 | 0.966–1.249 | 0.870–1.041 | 5 | pass |
| allocating | vertex-v1-resident-s4 | 0.998 | 0.960 | 0.946–1.071 | 0.904–0.986 | 5 | pass |
| allocating | vertex-v1-resident-s12 | 1.130 | 1.011 | 1.098–1.163 | 0.958–1.043 | 5 | pass |
| allocating | vertex-v1-resident-s32 | 1.110 | 0.972 | 0.986–1.444 | 0.940–0.994 | 5 | pass |
| allocating | vertex-v1-streaming-s4 | 2.360 | 0.983 | 1.538–2.847 | 0.945–0.995 | 9 | **FAIL** |
| allocating | vertex-v1-streaming-s12 | 1.008 | 1.001 | 0.943–1.227 | 0.944–1.134 | 5 | pass |
| allocating | vertex-v1-streaming-s32 | 1.009 | 1.056 | 0.971–1.027 | 0.995–1.118 | 5 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.139 | 1.012 | 1.098–1.207 | 0.982–1.066 | 5 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.141 | 1.008 | 1.122–1.145 | 0.998–1.021 | 5 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.153 | 1.008 | 1.096–1.163 | 0.982–1.041 | 5 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.159 | 1.034 | 1.076–1.181 | 0.894–1.146 | 5 | pass |
| allocating | varied-filter-1-tiny-s4 | 1.484 | 0.995 | 1.447–1.498 | 0.981–1.022 | 9 | pass |
| allocating | varied-view-1-tiny-s4 | 1.183 | 0.935 | 1.149–1.246 | 0.915–0.974 | 5 | pass |
| allocating | varied-filter-1-tiny-s8 | 1.142 | 0.978 | 1.125–1.156 | 0.967–0.992 | 5 | pass |
| allocating | varied-view-1-tiny-s8 | 1.138 | 0.982 | 1.107–1.192 | 0.951–1.024 | 5 | pass |
| allocating | varied-filter-2-tiny-s8 | 1.326 | 0.989 | 1.308–1.348 | 0.959–1.006 | 5 | pass |
| allocating | varied-view-2-tiny-s8 | 1.213 | 0.994 | 1.178–1.223 | 0.964–1.035 | 5 | pass |
| allocating | varied-filter-3-tiny-s4 | 0.915 | 1.049 | 0.877–0.983 | 0.902–1.192 | 5 | pass |
| allocating | varied-view-3-tiny-s4 | 1.022 | 0.777 | 1.011–1.036 | 0.701–0.899 | 5 | pass |
| allocating | varied-filter-3-tiny-s12 | 0.888 | 1.029 | 0.868–0.926 | 1.010–1.050 | 5 | pass |
| allocating | varied-view-3-tiny-s12 | 1.126 | 0.998 | 1.121–1.159 | 0.978–1.042 | 5 | pass |
| allocating | varied-filter-3-tiny-s32 | 0.942 | 1.001 | 0.924–0.981 | 0.971–1.016 | 5 | pass |
| allocating | varied-view-3-tiny-s32 | 1.125 | 1.005 | 1.086–1.155 | 0.970–1.025 | 5 | pass |
| allocating | varied-filter-1-resident-s4 | 1.440 | 0.951 | 1.372–1.485 | 0.933–0.994 | 5 | pass |
| allocating | varied-view-1-resident-s4 | 1.253 | 0.965 | 1.232–1.293 | 0.956–0.978 | 5 | pass |
| allocating | varied-filter-1-resident-s8 | 1.167 | 1.008 | 1.154–1.176 | 0.984–1.023 | 5 | pass |
| allocating | varied-view-1-resident-s8 | 1.157 | 1.000 | 1.128–1.162 | 0.977–1.015 | 5 | pass |
| allocating | varied-filter-2-resident-s8 | 1.306 | 1.004 | 1.270–1.342 | 0.994–1.009 | 5 | pass |
| allocating | varied-view-2-resident-s8 | 1.221 | 0.987 | 1.109–1.284 | 0.948–1.068 | 5 | pass |
| allocating | varied-filter-3-resident-s4 | 0.889 | 0.861 | 0.881–0.889 | 0.854–0.870 | 5 | pass |
| allocating | varied-view-3-resident-s4 | 0.968 | 0.949 | 0.958–0.977 | 0.934–0.959 | 5 | pass |
| allocating | varied-filter-3-resident-s12 | 0.919 | 1.021 | 0.906–0.922 | 1.015–1.028 | 5 | pass |
| allocating | varied-view-3-resident-s12 | 1.101 | 0.998 | 1.093–1.111 | 0.990–1.005 | 5 | pass |
| allocating | varied-filter-3-resident-s32 | 0.900 | 1.010 | 0.895–0.909 | 0.999–1.019 | 5 | pass |
| allocating | varied-view-3-resident-s32 | 1.125 | 1.005 | 1.069–1.167 | 0.975–1.044 | 5 | pass |
| allocating | varied-filter-1-streaming-s4 | 1.447 | 1.000 | 1.430–1.456 | 0.997–1.004 | 5 | pass |
| allocating | varied-view-1-streaming-s4 | 1.240 | 0.972 | 1.173–1.320 | 0.741–1.112 | 5 | pass |
| allocating | varied-filter-1-streaming-s8 | 1.152 | 0.991 | 1.143–1.186 | 0.963–1.044 | 5 | pass |
| allocating | varied-view-1-streaming-s8 | 1.119 | 0.991 | 1.076–1.212 | 0.960–1.044 | 5 | pass |
| allocating | varied-filter-2-streaming-s8 | 1.312 | 0.993 | 1.285–1.367 | 0.967–1.016 | 5 | pass |
| allocating | varied-view-2-streaming-s8 | 1.207 | 0.983 | 1.184–1.232 | 0.938–1.012 | 5 | pass |
| allocating | varied-filter-3-streaming-s4 | 0.919 | 1.000 | 0.880–0.952 | 0.948–1.030 | 5 | pass |
| allocating | varied-view-3-streaming-s4 | 0.980 | 0.946 | 0.939–1.008 | 0.927–0.959 | 5 | pass |
| allocating | varied-filter-3-streaming-s12 | 0.930 | 0.994 | 0.904–0.957 | 0.974–1.015 | 5 | pass |
| allocating | varied-view-3-streaming-s12 | 1.080 | 1.005 | 1.073–1.102 | 0.936–1.051 | 5 | pass |
| allocating | varied-filter-3-streaming-s32 | 2.990 | 0.998 | 1.596–3.944 | 0.942–1.109 | 10 | **FAIL** |
| allocating | varied-view-3-streaming-s32 | 0.926 | 1.024 | 0.744–1.308 | 0.949–1.059 | 5 | pass |
| allocating | meshlet-1-1-v2-t3 | 1.196 | 0.900 | 1.159–1.230 | 0.868–0.961 | 5 | pass |
| allocating | meshlet-1-1-v2-t4 | 1.154 | 0.930 | 1.152–1.221 | 0.889–0.956 | 5 | pass |
| allocating | meshlet-1-1-v4-t3 | 1.178 | 0.922 | 1.180–1.227 | 0.905–0.951 | 5 | pass |
| allocating | meshlet-1-1-v4-t4 | 1.180 | 0.919 | 1.129–1.202 | 0.897–0.939 | 5 | pass |
| allocating | meshlet-raw-1-1 | 1.129 | 0.998 | 1.109–1.141 | 0.952–1.031 | 5 | pass |
| allocating | meshlet-64-126-v2-t3 | 1.254 | 1.006 | 1.211–1.280 | 0.974–1.062 | 5 | pass |
| allocating | meshlet-64-126-v2-t4 | 1.179 | 0.970 | 1.086–1.444 | 0.901–1.107 | 7 | pass |
| allocating | meshlet-64-126-v4-t3 | 1.200 | 1.019 | 1.200–1.227 | 0.996–1.037 | 5 | pass |
| allocating | meshlet-64-126-v4-t4 | 1.164 | 0.950 | 1.153–1.195 | 0.937–0.979 | 5 | pass |
| allocating | meshlet-raw-64-126 | 1.108 | 1.001 | 1.072–1.165 | 0.936–1.077 | 5 | pass |
| allocating | meshlet-256-256-v2-t3 | 1.183 | 0.982 | 1.157–1.218 | 0.900–1.042 | 5 | pass |
| allocating | meshlet-256-256-v2-t4 | 1.145 | 1.020 | 1.118–1.159 | 0.984–1.038 | 5 | pass |
| allocating | meshlet-256-256-v4-t3 | 1.167 | 1.026 | 1.104–1.317 | 0.988–1.101 | 5 | pass |
| allocating | meshlet-256-256-v4-t4 | 1.094 | 0.996 | 1.069–1.120 | 0.979–1.020 | 5 | pass |
| allocating | meshlet-raw-256-256 | 1.061 | 1.011 | 1.051–1.074 | 0.986–1.019 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s4 | 1.199 | 0.953 | 1.118–1.335 | 0.914–1.046 | 5 | pass |
| caller-buffer | view-none-tiny-s4 | 1.179 | 0.978 | 1.166–1.228 | 0.946–1.021 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.162 | 1.001 | 0.960–1.282 | 0.969–1.019 | 5 | pass |
| caller-buffer | view-none-tiny-s12 | 1.167 | 1.001 | 1.160–1.211 | 0.974–1.050 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.198 | 1.028 | 1.114–1.215 | 1.003–1.039 | 5 | pass |
| caller-buffer | view-none-tiny-s32 | 1.126 | 1.032 | 1.066–1.330 | 0.948–1.165 | 5 | pass |
| caller-buffer | filter-1-tiny-s4 | 1.025 | 0.819 | 0.960–1.282 | 0.710–1.093 | 5 | pass |
| caller-buffer | view-1-tiny-s4 | 1.044 | 0.922 | 0.906–1.201 | 0.912–0.934 | 5 | pass |
| caller-buffer | filter-1-tiny-s8 | 0.771 | 0.770 | 0.758–0.781 | 0.741–0.783 | 5 | pass |
| caller-buffer | view-1-tiny-s8 | 1.080 | 0.969 | 1.053–1.105 | 0.930–0.986 | 5 | pass |
| caller-buffer | filter-2-tiny-s8 | 0.767 | 0.617 | 0.746–0.775 | 0.609–0.625 | 5 | pass |
| caller-buffer | view-2-tiny-s8 | 1.096 | 0.918 | 1.081–1.115 | 0.896–0.944 | 5 | pass |
| caller-buffer | filter-3-tiny-s12 | 0.844 | 0.809 | 0.832–0.856 | 0.791–0.827 | 5 | pass |
| caller-buffer | view-3-tiny-s12 | 1.121 | 1.008 | 1.088–1.134 | 0.985–1.042 | 5 | pass |
| caller-buffer | vertex-v0-resident-s4 | 0.987 | 0.956 | 0.977–0.997 | 0.946–0.960 | 5 | pass |
| caller-buffer | view-none-resident-s4 | 0.989 | 0.939 | 0.972–1.000 | 0.932–0.952 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.158 | 1.002 | 1.102–1.165 | 0.981–1.058 | 5 | pass |
| caller-buffer | view-none-resident-s12 | 1.150 | 1.030 | 1.106–1.166 | 1.011–1.056 | 5 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.139 | 1.005 | 1.106–1.175 | 0.973–1.025 | 5 | pass |
| caller-buffer | view-none-resident-s32 | 1.162 | 1.017 | 1.136–1.180 | 0.990–1.040 | 5 | pass |
| caller-buffer | filter-1-resident-s4 | 0.493 | 0.501 | 0.467–0.519 | 0.320–0.626 | 5 | pass |
| caller-buffer | view-1-resident-s4 | 0.602 | 0.628 | 0.591–0.621 | 0.586–0.650 | 5 | pass |
| caller-buffer | filter-1-resident-s8 | 0.438 | 0.439 | 0.429–0.444 | 0.403–0.462 | 5 | pass |
| caller-buffer | view-1-resident-s8 | 0.717 | 0.726 | 0.704–0.725 | 0.689–0.744 | 5 | pass |
| caller-buffer | filter-2-resident-s8 | 0.340 | 0.282 | 0.327–0.354 | 0.273–0.293 | 5 | pass |
| caller-buffer | view-2-resident-s8 | 0.679 | 0.613 | 0.669–0.683 | 0.601–0.618 | 5 | pass |
| caller-buffer | filter-3-resident-s12 | 0.938 | 1.019 | 0.932–0.960 | 0.710–1.221 | 5 | pass |
| caller-buffer | view-3-resident-s12 | 1.063 | 0.987 | 1.027–1.115 | 0.950–1.009 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 0.997 | 0.963 | 0.959–1.016 | 0.907–0.997 | 5 | pass |
| caller-buffer | view-none-streaming-s4 | 0.985 | 0.971 | 0.970–0.991 | 0.855–1.012 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.059 | 1.000 | 1.044–1.101 | 0.966–1.066 | 5 | pass |
| caller-buffer | view-none-streaming-s12 | 1.061 | 0.990 | 1.035–1.079 | 0.685–1.179 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s32 | 0.873 | 0.986 | 0.822–0.916 | 0.617–1.238 | 5 | pass |
| caller-buffer | view-none-streaming-s32 | 0.839 | 1.079 | 0.600–1.206 | 0.871–1.380 | 5 | pass |
| caller-buffer | filter-1-streaming-s4 | 0.537 | 0.484 | 0.499–0.601 | 0.305–0.631 | 5 | pass |
| caller-buffer | view-1-streaming-s4 | 0.561 | 0.578 | 0.310–0.748 | 0.506–0.625 | 5 | pass |
| caller-buffer | filter-1-streaming-s8 | 0.479 | 0.578 | 0.317–0.772 | 0.334–0.848 | 5 | pass |
| caller-buffer | view-1-streaming-s8 | 0.683 | 0.706 | 0.598–0.739 | 0.637–0.881 | 5 | pass |
| caller-buffer | filter-2-streaming-s8 | 0.391 | 0.367 | 0.345–0.443 | 0.317–0.407 | 5 | pass |
| caller-buffer | view-2-streaming-s8 | 0.625 | 0.589 | 0.547–0.754 | 0.330–0.813 | 5 | pass |
| caller-buffer | filter-3-streaming-s12 | 1.014 | 0.996 | 0.928–1.054 | 0.956–1.063 | 5 | pass |
| caller-buffer | view-3-streaming-s12 | 1.021 | 1.003 | 0.924–1.173 | 0.956–1.085 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 1.182 | 0.949 | 1.103–1.372 | 0.673–1.213 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.176 | 0.958 | 1.147–1.192 | 0.880–1.014 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.196 | 0.800 | 1.154–1.204 | 0.746–0.876 | 5 | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.005 | 0.955 | 0.967–1.020 | 0.933–0.981 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.115 | 0.997 | 1.112–1.135 | 0.977–1.017 | 5 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.162 | 1.026 | 1.128–1.171 | 0.983–1.080 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 0.877 | 0.977 | 0.857–0.963 | 0.889–1.043 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.074 | 1.036 | 1.027–1.082 | 0.970–1.123 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 0.835 | 0.968 | 0.756–0.946 | 0.915–1.013 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.124 | 0.982 | 1.076–1.168 | 0.930–1.035 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.157 | 1.022 | 1.084–1.210 | 0.984–1.040 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.120 | 1.002 | 1.113–1.145 | 0.999–1.007 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.122 | 0.992 | 1.109–1.128 | 0.961–1.012 | 5 | pass |
| caller-buffer | varied-filter-1-tiny-s4 | 1.605 | 0.998 | 1.571–1.696 | 0.963–1.034 | 5 | **FAIL** |
| caller-buffer | varied-view-1-tiny-s4 | 1.261 | 0.948 | 1.240–1.275 | 0.939–0.953 | 5 | pass |
| caller-buffer | varied-filter-1-tiny-s8 | 1.134 | 0.994 | 1.119–1.148 | 0.985–1.001 | 5 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.164 | 0.984 | 1.143–1.181 | 0.971–1.010 | 5 | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 1.336 | 0.995 | 1.323–1.343 | 0.982–1.010 | 5 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.226 | 0.906 | 1.215–1.244 | 0.896–0.932 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 0.899 | 0.957 | 0.840–0.923 | 0.928–1.003 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.012 | 0.946 | 0.987–1.058 | 0.762–1.031 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s12 | 0.843 | 0.999 | 0.822–0.867 | 0.828–1.097 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.111 | 0.932 | 0.971–1.189 | 0.908–0.948 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s32 | 1.135 | 1.067 | 1.069–1.173 | 0.966–1.116 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.139 | 0.931 | 1.113–1.162 | 0.862–0.970 | 5 | pass |
| caller-buffer | varied-filter-1-resident-s4 | 1.441 | 1.006 | 1.430–1.455 | 0.996–1.022 | 5 | pass |
| caller-buffer | varied-view-1-resident-s4 | 1.249 | 0.999 | 1.223–1.288 | 0.761–1.114 | 5 | pass |
| caller-buffer | varied-filter-1-resident-s8 | 1.157 | 0.935 | 1.140–1.203 | 0.876–0.985 | 5 | pass |
| caller-buffer | varied-view-1-resident-s8 | 1.165 | 1.020 | 1.138–1.180 | 1.008–1.032 | 5 | pass |
| caller-buffer | varied-filter-2-resident-s8 | 1.331 | 0.923 | 1.298–1.352 | 0.886–0.944 | 5 | pass |
| caller-buffer | varied-view-2-resident-s8 | 1.204 | 1.006 | 1.202–1.280 | 0.959–1.091 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s4 | 0.917 | 1.027 | 0.797–1.017 | 1.017–1.043 | 5 | pass |
| caller-buffer | varied-view-3-resident-s4 | 0.971 | 0.954 | 0.956–0.982 | 0.943–0.977 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s12 | 0.932 | 1.000 | 0.906–0.963 | 0.988–1.020 | 5 | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.128 | 1.019 | 1.048–1.158 | 0.869–1.365 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s32 | 0.900 | 0.959 | 0.894–0.911 | 0.949–0.974 | 5 | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.123 | 1.009 | 1.099–1.139 | 0.988–1.024 | 5 | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 1.441 | 0.994 | 1.342–1.493 | 0.983–1.008 | 9 | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1.264 | 0.980 | 1.242–1.280 | 0.967–0.998 | 5 | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 1.162 | 0.993 | 1.158–1.180 | 0.974–1.011 | 5 | pass |
| caller-buffer | varied-view-1-streaming-s8 | 1.153 | 0.995 | 1.120–1.173 | 0.983–1.008 | 5 | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 1.339 | 1.002 | 1.334–1.358 | 0.993–1.020 | 5 | pass |
| caller-buffer | varied-view-2-streaming-s8 | 1.251 | 1.009 | 1.205–1.294 | 0.923–1.063 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 0.959 | 0.994 | 0.898–1.009 | 0.747–1.183 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s4 | 0.949 | 0.948 | 0.916–0.992 | 0.902–0.975 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 0.941 | 1.024 | 0.921–0.953 | 0.805–1.155 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.064 | 1.009 | 1.063–1.086 | 0.993–1.033 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 0.937 | 1.044 | 0.816–1.160 | 0.750–1.407 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.118 | 1.034 | 1.048–1.158 | 0.978–1.065 | 5 | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 1.240 | 0.667 | 1.166–1.332 | 0.638–0.724 | 5 | pass |
| caller-buffer | meshlet-1-1-v2-t4 | 1.251 | 0.650 | 1.221–1.262 | 0.564–0.709 | 5 | pass |
| caller-buffer | meshlet-1-1-v4-t3 | 1.233 | 0.678 | 1.155–1.305 | 0.536–0.746 | 5 | pass |
| caller-buffer | meshlet-1-1-v4-t4 | 1.226 | 0.662 | 1.215–1.251 | 0.643–0.675 | 5 | pass |
| caller-buffer | meshlet-raw-1-1 | 1.262 | 0.859 | 1.226–1.300 | 0.854–0.872 | 5 | pass |
| caller-buffer | meshlet-64-126-v2-t3 | 1.312 | 0.993 | 1.285–1.338 | 0.969–1.009 | 5 | pass |
| caller-buffer | meshlet-64-126-v2-t4 | 1.274 | 1.021 | 1.224–1.318 | 0.961–1.061 | 5 | pass |
| caller-buffer | meshlet-64-126-v4-t3 | 1.318 | 0.998 | 1.275–1.348 | 0.966–1.028 | 5 | pass |
| caller-buffer | meshlet-64-126-v4-t4 | 1.252 | 1.014 | 1.246–1.303 | 0.977–1.052 | 5 | pass |
| caller-buffer | meshlet-raw-64-126 | 1.180 | 1.009 | 1.164–1.205 | 0.988–1.042 | 5 | pass |
| caller-buffer | meshlet-256-256-v2-t3 | 1.244 | 1.014 | 1.202–1.284 | 0.998–1.034 | 5 | pass |
| caller-buffer | meshlet-256-256-v2-t4 | 1.191 | 1.016 | 1.182–1.228 | 0.994–1.046 | 5 | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 1.210 | 1.005 | 1.195–1.224 | 0.988–1.021 | 5 | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 1.186 | 1.003 | 1.176–1.198 | 0.971–1.032 | 5 | pass |
| caller-buffer | meshlet-raw-256-256 | 1.133 | 0.993 | 1.080–1.224 | 0.897–1.092 | 5 | pass |

## Every wasm row

| API | Case | New/C++ | Paired new/old | Final 95% new/C++ | Final 95% new/old | Pairs | RFC max |
|---|---|---:|---:|---|---|---|---|
| allocating | vertex-v0-tiny-s4 | 1.085 | 0.859 | 0.728–1.477 | 0.559–1.047 | 17 | pass |
| allocating | view-none-tiny-s4 | 0.983 | 1.003 | 0.519–1.421 | 0.810–1.107 | 10 | pass |
| allocating | vertex-v0-tiny-s12 | 1.230 | 1.089 | 1.082–1.477 | 0.917–1.434 | 10 | pass |
| allocating | view-none-tiny-s12 | 1.364 | 1.106 | 1.239–1.499 | 1.063–1.206 | 8 | pass |
| allocating | vertex-v0-tiny-s32 | 1.153 | 1.224 | 1.033–1.473 | 0.959–1.281 | 11 | pass |
| allocating | view-none-tiny-s32 | 1.319 | 1.215 | 1.245–1.393 | 1.065–1.353 | 5 | pass |
| allocating | view-1-tiny-s4 | 0.796 | 0.525 | 0.525–0.978 | 0.126–1.336 | 5 | pass |
| allocating | view-1-tiny-s8 | 1.060 | 0.814 | 0.720–1.383 | 0.559–2.034 | 6 | pass |
| allocating | view-2-tiny-s8 | 0.953 | 0.889 | 0.827–1.237 | 0.737–1.207 | 5 | pass |
| allocating | view-3-tiny-s12 | 1.040 | 1.047 | 0.690–1.419 | 0.847–1.306 | 8 | pass |
| allocating | index-3-v0-tiny-s2 | 1.027 | 1.283 | 0.605–1.482 | 0.920–1.248 | 15 | pass |
| allocating | index-3-v0-tiny-s4 | 0.888 | 0.937 | 0.795–1.198 | 0.643–1.119 | 5 | pass |
| allocating | index-3-v1-tiny-s2 | 0.925 | 1.095 | 0.749–1.411 | 0.974–1.274 | 5 | pass |
| allocating | index-3-v1-tiny-s4 | 1.086 | 0.974 | 0.844–1.137 | 0.872–1.008 | 5 | pass |
| allocating | vertex-v0-resident-s4 | 1.330 | 1.221 | 1.038–1.447 | 1.072–1.360 | 8 | pass |
| allocating | view-none-resident-s4 | 1.364 | 1.261 | 1.302–1.483 | 1.133–1.389 | 7 | pass |
| allocating | vertex-v0-resident-s12 | 1.386 | 1.163 | 1.183–1.478 | 1.119–1.215 | 6 | pass |
| allocating | view-none-resident-s12 | 1.404 | 1.218 | 1.340–1.497 | 1.007–1.602 | 5 | pass |
| allocating | vertex-v0-resident-s32 | 1.285 | 1.141 | 1.220–1.416 | 0.783–1.385 | 5 | pass |
| allocating | view-none-resident-s32 | 1.408 | 1.239 | 1.401–1.453 | 1.204–1.285 | 20+30 | pass |
| allocating | view-1-resident-s4 | 1.154 | 0.713 | 0.975–1.325 | 0.648–0.767 | 5 | pass |
| allocating | view-1-resident-s8 | 1.723 | 0.943 | 1.516–1.797 | 0.848–0.967 | 8 | **FAIL** |
| allocating | view-2-resident-s8 | 1.536 | 0.951 | 1.477–1.609 | 0.890–0.956 | 20+30 | **FAIL** |
| allocating | view-3-resident-s12 | 1.946 | 1.194 | 1.878–2.151 | 1.101–1.276 | 5 | **FAIL** |
| allocating | index-3-v0-resident-s2 | 0.957 | 1.002 | 0.931–1.039 | 0.817–1.245 | 5 | pass |
| allocating | index-3-v0-resident-s4 | 0.962 | 1.003 | 0.907–1.017 | 0.911–1.070 | 5 | pass |
| allocating | index-3-v1-resident-s2 | 0.998 | 0.971 | 0.902–1.045 | 0.832–1.052 | 5 | pass |
| allocating | index-3-v1-resident-s4 | 0.938 | 0.976 | 0.867–1.024 | 0.846–1.099 | 5 | pass |
| allocating | vertex-v0-streaming-s4 | 1.305 | 1.245 | 1.076–1.463 | 1.201–1.307 | 8 | pass |
| allocating | view-none-streaming-s4 | 1.394 | 1.305 | 1.325–1.495 | 1.224–1.339 | 9 | pass |
| allocating | vertex-v0-streaming-s12 | 1.109 | 1.074 | 0.924–1.332 | 0.969–1.282 | 5 | pass |
| allocating | view-none-streaming-s12 | 1.546 | 1.321 | 1.426–1.672 | 1.044–1.352 | 20+30 | **FAIL** |
| allocating | vertex-v0-streaming-s32 | 1.020 | 1.094 | 0.923–1.237 | 0.882–1.253 | 5 | pass |
| allocating | view-none-streaming-s32 | 1.221 | 1.103 | 0.983–1.470 | 0.944–1.211 | 5 | pass |
| allocating | view-1-streaming-s4 | 0.948 | 0.604 | 0.690–1.411 | 0.482–0.803 | 8 | pass |
| allocating | view-1-streaming-s8 | 1.251 | 0.696 | 0.887–1.489 | 0.589–0.894 | 10 | pass |
| allocating | view-2-streaming-s8 | 1.353 | 0.748 | 1.064–1.479 | 0.667–0.858 | 10 | pass |
| allocating | view-3-streaming-s12 | 1.052 | 1.003 | 1.181–1.763 | 0.935–1.304 | 20+30 | **FAIL** |
| allocating | index-3-v0-streaming-s2 | 0.719 | 1.017 | 0.521–1.437 | 0.559–1.880 | 5 | pass |
| allocating | index-3-v0-streaming-s4 | 0.940 | 0.953 | 0.872–1.068 | 0.738–1.098 | 5 | pass |
| allocating | index-3-v1-streaming-s2 | 0.665 | 1.029 | 0.562–1.399 | 0.697–1.627 | 5 | pass |
| allocating | index-3-v1-streaming-s4 | 0.943 | 1.059 | 0.846–1.068 | 0.966–1.104 | 5 | pass |
| allocating | vertex-v1-tiny-s4 | 1.119 | 1.038 | 0.700–1.492 | 0.987–1.095 | 10 | pass |
| allocating | vertex-v1-tiny-s12 | 1.282 | 1.093 | 0.776–1.494 | 1.018–1.161 | 7 | pass |
| allocating | vertex-v1-tiny-s32 | 1.405 | 1.021 | 1.347–1.485 | 0.715–1.277 | 5 | pass |
| allocating | vertex-v1-resident-s4 | 1.075 | 0.937 | 0.998–1.135 | 0.880–1.025 | 5 | pass |
| allocating | vertex-v1-resident-s12 | 1.189 | 1.003 | 1.088–1.254 | 0.952–1.052 | 5 | pass |
| allocating | vertex-v1-resident-s32 | 1.241 | 1.009 | 1.154–1.391 | 0.973–1.092 | 5 | pass |
| allocating | vertex-v1-streaming-s4 | 1.073 | 1.029 | 1.029–1.184 | 0.991–1.066 | 5 | pass |
| allocating | vertex-v1-streaming-s12 | 1.389 | 1.056 | 0.890–1.499 | 0.987–1.220 | 14 | pass |
| allocating | vertex-v1-streaming-s32 | 1.109 | 1.079 | 0.921–1.279 | 0.852–1.197 | 5 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.199 | 1.008 | 0.876–1.492 | 0.927–1.244 | 11 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.212 | 0.982 | 1.154–1.255 | 0.942–1.023 | 5 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.189 | 0.993 | 1.105–1.254 | 0.909–1.057 | 5 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.196 | 0.973 | 1.086–1.241 | 0.917–1.027 | 5 | pass |
| allocating | varied-view-1-tiny-s4 | 1.189 | 1.051 | 1.090–1.483 | 0.982–1.114 | 5 | pass |
| allocating | varied-view-1-tiny-s8 | 1.275 | 1.009 | 1.133–1.419 | 0.936–1.148 | 5 | pass |
| allocating | varied-view-2-tiny-s8 | 1.185 | 1.096 | 1.132–1.373 | 0.814–1.452 | 5 | pass |
| allocating | varied-view-3-tiny-s4 | 1.294 | 1.154 | 1.107–1.439 | 0.930–1.287 | 5 | pass |
| allocating | varied-view-3-tiny-s12 | 1.185 | 1.196 | 1.058–1.228 | 0.877–1.323 | 5 | pass |
| allocating | varied-view-3-tiny-s32 | 1.189 | 1.061 | 1.045–1.474 | 0.937–1.251 | 5 | pass |
| allocating | varied-view-1-resident-s4 | 1.628 | 1.019 | 1.504–1.673 | 0.978–1.073 | 17 | **FAIL** |
| allocating | varied-view-1-resident-s8 | 1.601 | 1.033 | 1.518–1.629 | 0.979–1.106 | 5 | **FAIL** |
| allocating | varied-view-2-resident-s8 | 1.475 | 1.039 | 1.433–1.487 | 1.002–1.040 | 20+30 | pass |
| allocating | varied-view-3-resident-s4 | 1.171 | 1.032 | 0.970–1.343 | 0.958–1.210 | 5 | pass |
| allocating | varied-view-3-resident-s12 | 1.317 | 1.058 | 1.204–1.478 | 0.843–1.265 | 6 | pass |
| allocating | varied-view-3-resident-s32 | 1.236 | 1.058 | 1.098–1.412 | 0.953–1.122 | 5 | pass |
| allocating | varied-view-1-streaming-s4 | 1.634 | 1.032 | 1.388–1.703 | 0.970–1.116 | 20+30 | **FAIL** |
| allocating | varied-view-1-streaming-s8 | 1.531 | 1.049 | 1.306–1.696 | 0.937–1.154 | 20+30 | **FAIL** |
| allocating | varied-view-2-streaming-s8 | 1.557 | 1.049 | 1.290–1.679 | 0.920–1.150 | 20+30 | **FAIL** |
| allocating | varied-view-3-streaming-s4 | 1.140 | 1.003 | 1.064–1.162 | 0.964–1.039 | 5 | pass |
| allocating | varied-view-3-streaming-s12 | 1.401 | 1.022 | 1.214–1.518 | 0.942–1.178 | 20+30 | **FAIL** |
| allocating | varied-view-3-streaming-s32 | 1.331 | 1.042 | 1.350–1.493 | 1.012–1.125 | 17 | pass |
| caller-buffer | vertex-v0-tiny-s4 | 0.831 | 0.987 | 0.359–1.351 | 0.935–1.026 | 6 | pass |
| caller-buffer | view-none-tiny-s4 | 0.872 | 0.999 | 0.844–0.900 | 0.958–1.023 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.024 | 1.049 | 0.990–1.098 | 1.025–1.072 | 5 | pass |
| caller-buffer | view-none-tiny-s12 | 1.122 | 1.003 | 0.806–1.446 | 0.882–1.059 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.184 | 1.087 | 1.064–1.223 | 1.013–1.145 | 5 | pass |
| caller-buffer | view-none-tiny-s32 | 1.176 | 1.075 | 1.174–1.222 | 1.013–1.132 | 5 | pass |
| caller-buffer | view-1-tiny-s4 | 0.960 | 0.912 | 0.949–0.962 | 0.900–0.935 | 5 | pass |
| caller-buffer | view-1-tiny-s8 | 1.204 | 1.015 | 1.097–1.329 | 0.887–1.147 | 5 | pass |
| caller-buffer | view-2-tiny-s8 | 1.233 | 1.006 | 1.136–1.380 | 0.947–1.150 | 5 | pass |
| caller-buffer | view-3-tiny-s12 | 1.254 | 1.176 | 1.227–1.419 | 1.117–1.312 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.954 | 1.004 | 0.758–1.041 | 0.951–1.215 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.817 | 1.019 | 0.758–0.942 | 0.949–1.152 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.814 | 0.990 | 0.758–0.840 | 0.748–1.079 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 0.817 | 0.998 | 0.761–0.968 | 0.922–1.162 | 5 | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.036 | 1.000 | 0.987–1.043 | 0.825–1.100 | 5 | pass |
| caller-buffer | view-none-resident-s4 | 1.072 | 1.004 | 1.049–1.078 | 0.977–1.040 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.172 | 1.044 | 1.160–1.191 | 1.012–1.080 | 5 | pass |
| caller-buffer | view-none-resident-s12 | 1.244 | 1.044 | 1.180–1.306 | 1.010–1.096 | 5 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.186 | 1.059 | 1.105–1.325 | 0.938–1.163 | 5 | pass |
| caller-buffer | view-none-resident-s32 | 1.207 | 1.012 | 1.192–1.227 | 0.991–1.040 | 5 | pass |
| caller-buffer | view-1-resident-s4 | 0.926 | 0.584 | 0.837–0.982 | 0.571–0.601 | 5 | pass |
| caller-buffer | view-1-resident-s8 | 1.367 | 0.696 | 1.306–1.498 | 0.672–0.738 | 9 | pass |
| caller-buffer | view-2-resident-s8 | 1.358 | 0.715 | 1.298–1.365 | 0.699–0.745 | 5 | pass |
| caller-buffer | view-3-resident-s12 | 1.736 | 1.008 | 1.650–1.762 | 0.968–1.113 | 5 | **FAIL** |
| caller-buffer | index-3-v0-resident-s2 | 0.833 | 1.000 | 0.820–0.847 | 0.987–1.007 | 5 | pass |
| caller-buffer | index-3-v0-resident-s4 | 0.849 | 0.990 | 0.826–0.876 | 0.964–1.026 | 5 | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.865 | 1.014 | 0.799–0.972 | 0.922–1.059 | 5 | pass |
| caller-buffer | index-3-v1-resident-s4 | 0.850 | 0.988 | 0.835–0.866 | 0.958–1.027 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 1.083 | 1.018 | 1.031–1.133 | 0.971–1.089 | 5 | pass |
| caller-buffer | view-none-streaming-s4 | 1.031 | 1.015 | 1.004–1.106 | 0.970–1.096 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.090 | 1.001 | 1.034–1.179 | 0.965–1.093 | 5 | pass |
| caller-buffer | view-none-streaming-s12 | 1.185 | 1.014 | 1.143–1.262 | 1.003–1.038 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1.151 | 1.020 | 1.104–1.180 | 0.963–1.122 | 5 | pass |
| caller-buffer | view-none-streaming-s32 | 1.148 | 0.979 | 1.078–1.364 | 0.921–1.151 | 5 | pass |
| caller-buffer | view-1-streaming-s4 | 0.886 | 0.573 | 0.855–0.947 | 0.394–0.687 | 5 | pass |
| caller-buffer | view-1-streaming-s8 | 1.322 | 0.691 | 1.285–1.356 | 0.663–0.719 | 5 | pass |
| caller-buffer | view-2-streaming-s8 | 1.259 | 0.730 | 1.091–1.400 | 0.702–0.753 | 5 | pass |
| caller-buffer | view-3-streaming-s12 | 1.572 | 1.054 | 1.506–1.623 | 0.963–1.117 | 7 | **FAIL** |
| caller-buffer | index-3-v0-streaming-s2 | 0.871 | 1.004 | 0.827–0.925 | 0.907–1.222 | 5 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 0.873 | 1.020 | 0.821–0.966 | 0.916–1.121 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.870 | 0.984 | 0.788–0.995 | 0.935–1.055 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 0.903 | 1.063 | 0.763–0.940 | 1.025–1.119 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 0.833 | 0.971 | 0.315–1.356 | 0.942–1.024 | 6 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.127 | 1.030 | 0.748–1.418 | 0.980–1.136 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.366 | 1.121 | 1.002–1.499 | 1.087–1.192 | 7 | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.048 | 0.971 | 0.949–1.067 | 0.882–1.018 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.257 | 1.041 | 1.138–1.294 | 0.978–1.139 | 5 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.171 | 1.059 | 1.097–1.254 | 0.987–1.117 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1.049 | 1.001 | 1.001–1.102 | 0.963–1.055 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.131 | 1.020 | 1.115–1.175 | 0.971–1.043 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1.260 | 1.044 | 1.101–1.256 | 1.000–1.129 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.192 | 1.022 | 1.178–1.199 | 0.973–1.058 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.222 | 1.024 | 1.120–1.293 | 0.903–1.084 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.191 | 1.020 | 1.149–1.251 | 0.925–1.077 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.151 | 1.064 | 1.074–1.254 | 0.971–1.128 | 5 | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.092 | 1.053 | 0.899–1.421 | 0.966–1.240 | 5 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.178 | 1.127 | 1.067–1.392 | 1.106–1.159 | 5 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.306 | 1.193 | 1.209–1.379 | 0.987–1.393 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.037 | 1.115 | 0.881–1.083 | 0.742–1.989 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.113 | 1.083 | 1.040–1.197 | 1.016–1.141 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.276 | 1.174 | 1.136–1.483 | 1.086–1.340 | 7 | pass |
| caller-buffer | varied-view-1-resident-s4 | 1.603 | 1.026 | 1.525–1.689 | 0.947–1.082 | 5 | **FAIL** |
| caller-buffer | varied-view-1-resident-s8 | 1.545 | 1.054 | 1.500–1.660 | 1.001–1.081 | 5 | **FAIL** |
| caller-buffer | varied-view-2-resident-s8 | 1.523 | 1.032 | 1.487–1.551 | 0.994–1.029 | 20+30 | **FAIL** |
| caller-buffer | varied-view-3-resident-s4 | 1.143 | 1.029 | 1.008–1.298 | 0.991–1.103 | 5 | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.312 | 1.029 | 1.268–1.352 | 1.004–1.070 | 5 | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.308 | 0.999 | 1.257–1.363 | 0.972–1.057 | 5 | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1.594 | 1.008 | 1.580–1.693 | 0.972–1.065 | 5 | **FAIL** |
| caller-buffer | varied-view-1-streaming-s8 | 1.606 | 1.023 | 1.578–1.628 | 0.984–1.057 | 5 | **FAIL** |
| caller-buffer | varied-view-2-streaming-s8 | 1.562 | 1.023 | 1.504–1.594 | 0.994–1.047 | 5 | **FAIL** |
| caller-buffer | varied-view-3-streaming-s4 | 1.129 | 0.987 | 1.092–1.174 | 0.916–1.038 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.399 | 1.052 | 1.373–1.465 | 1.029–1.077 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.379 | 1.049 | 1.304–1.496 | 1.006–1.081 | 5 | pass |
