# P07 fix-four scoped final performance — 2026-10-05

All ratios are Rust time / C++ time; lower is faster. Means and displayed
case ratios use stage-one medians. Maxima are gated by final paired 95%
log-ratio intervals; a D146 interval supersedes stage one. Five–twenty pairs
stop on the first resolved registered/brief maximum; only borderline cases
receive thirty fresh pairs. Before is the matching p07r3 row, a separate
historical epoch. Only sequence also interleaves the starting Rust binary.

Candidate fixes: a48d060, 70727c3, 72e03af, f72221b, b0f0377, eb6bac6.
Tests and scoped controllers: 52f22d5. Evidence:
`/mnt/linux-extra/meshopt-artifacts/p07-fix4`; summary.json and verify.log
retain independent checks. Source/kernel identity is frozen before timing.

## Family means and verdicts

| Family | Allocating before → after | Caller-buffer before → after | Registered family bar A/C | Brief 1.25/1.50 A/C |
|---|---:|---:|---|---|
| vertex | 1.832 → 1.201 | 1.967 → 1.103 | FAIL / FAIL | FAIL / FAIL |
| view-none | 1.702 → 1.155 | 1.872 → 1.183 | FAIL / FAIL | pass / FAIL |
| view-filtered | 1.841 → 1.295 | 1.826 → 1.185 | FAIL / FAIL | FAIL / FAIL |
| color | 1.540 → 1.397 | 1.496 → 1.395 | FAIL / FAIL | FAIL / FAIL |
| meshlet | 1.329 → 1.332 | 1.534 → 1.564 | FAIL / FAIL | FAIL / FAIL |
| meshlet-raw | 1.228 → 1.182 | 1.315 → 1.299 | FAIL / FAIL | pass / FAIL |
| sequence | 1.298 → 1.237 | 1.058 → 1.078 | FAIL / pass | FAIL / pass |
| wasm touched scope | 1.360 → 1.151 | 1.426 → 1.156 | FAIL / FAIL | FAIL / FAIL |

S1 remains FAIL for both APIs (registered 1.10 geomean / 1.30 maximum).
S2 touched Color/views remain FAIL; standalone Oct/Quat/Exp is outside this
timing scope. S5 remains FAIL for both APIs (registered 1.25 / 1.60).
The brief states 1.25 / 1.50 uniformly; both sets are assessed without
changing SIMD_BAR.md. Family verdicts include means and every case maximum.

S3 has **0 final-stage significant comparisons** in the touched scope
and 0 distinct comparisons when either stage is considered.
The unchanged standalone caller-buffer Exp resident-s4 and streaming-s12
historical regressions are unclosed. Scope-limited zero does not qualify full S3.

S4 sequence-only:

- allocating: scalar-C++ geomean 1.244, 2 maximum failures, 0 frozen minimum failures: **FAIL**.
- caller-buffer: scalar-C++ geomean 1.073, 0 maximum failures, 0 frozen minimum failures: **pass**.

Triangle-index S4 is not requalified. No release, ARM execution, dedicated
host, Windows/macOS, Moss integration or phase-0.6 qualification is claimed.

## Sequence allocation investigation

| API / streaming case | Starting/C++ in same pass | New/C++ | New/starting |
|---|---:|---:|---:|
| allocating / index-3-v0-streaming-s2 | 1.792 | 0.795 | 0.444 |
| allocating / index-3-v0-streaming-s4 | 2.650 | 2.523 | 0.952 |
| allocating / index-3-v1-streaming-s2 | 1.130 | 0.971 | 0.859 |
| allocating / index-3-v1-streaming-s4 | 3.205 | 2.876 | 0.897 |
| caller-buffer / index-3-v0-streaming-s2 | 1.060 | 1.000 | 0.943 |
| caller-buffer / index-3-v0-streaming-s4 | 1.206 | 0.970 | 0.804 |
| caller-buffer / index-3-v1-streaming-s2 | 1.537 | 1.239 | 0.806 |
| caller-buffer / index-3-v1-streaming-s4 | 1.122 | 1.027 | 0.916 |

The allocating v1 stride-four maximum persists at 2.876× despite reduced
instructions. Warm differential sequence instructions fall to about 0.808
starting Rust; cold allocating new Rust/C++ is 1.045. Cold page-fault counts
are lower in Rust, and both drivers fault during initialization and copies.
Fault stacks/counts and user-only cycles do not establish kernel or allocator
latency. Cause attribution is explicitly abandoned within this pass with
sequence-packets.json and sequence-fault-summary.json retained. No failed
case is retried for selection or cause-isolation timing.

## Correctness and safety

Frozen 869 fixture, 7,653 malformed and 138 benchmark identity cases pass:
both APIs, native scalar/SSE2/SSSE3/SSE4.1, executed wasm without/with SIMD.
Canonical scalar bytes/errors/tails match; the separate C++ SIMD floating
filter conformance contract is unchanged. Node preflight passes 292 exact
output hashes. All-feature/unsafe-free tests, native and wasm SIMD Clippy,
five SIMD integration Miri tests, three integer-kernel Miri tests and the
sequence fixture Miri test pass. MSRV 1.88/no-std and wasm SIMD/no-std compile
checks pass with the retained existing unused-helper warnings. Boundary and
package gates retain 23 unsafe blocks in the single audited module.

Optional expanded seeds: 70,000 P02 cases pass. Strict P04 is **FAIL** after
99,264 completed cases at seed-20261005-op20-9264, corrupted odd meshlet tail
0x1c: upstream SIMD returns -3 while upstream scalar and both Rust epochs
accept identical output. The partial archive/failure/reproduction are retained;
no input exclusion, altered oracle or replacement green sweep is used.

## Timing receipts and residuals

- native native-timing-burst1.log: wrapper 694 seconds, exit zero; resumes 0 completed rows.
- native native-timing-burst2.log: wrapper 65 seconds, exit zero; resumes 144 completed rows.
- wasm wasm-timing-burst1.log: wrapper 153 seconds, exit zero; resumes 0 completed rows.

Native has 28 fresh D146 rows; Node has 17.
Native resume drops the first fractional lease_bursts receipt, but preserves
all samples, controller segments and per-pair admission telemetry. Original
wrapper logs and timing-bursts.json retain each whole-second duration and
zero exit; do not claim the lost exact fractional value. All bursts are
under 840 seconds, admitted by the required heavy wrapper and scoreboard
exclusion. No accepted pair or completed row is repeated.

Native reconstruction frame shrinks 0x168→0x98, vector stack store sites
12→1, stack operand sites 104→37. Checked address/loop work remains.
Resident vertex instructions are about 0.49 starting Rust; wasm public/raw
resident probes are about 0.66–0.67. V8 work moves into the larger vertex
decode and helper bodies, so raw-function size alone does not prove whole
path code-size/spill improvement. Color instructions fall to 0.86–0.87
with exact arithmetic preserved. Typed/raw meshlet instructions are 0.95/0.86
starting Rust; typed family wall means remain slightly worse and fail.
Retain the safe packing implementation and failures; no late unmeasured revert.

Oct/Quat/Exp arithmetic bodies remain unchanged. Abandon speculative cheaper
round/divide/sqrt approximations because exact parity has no proof; historical
exhaustive filter runs are not credited as new evidence. Tiny scalar tails
remove partial-vector copies, but do not clear all native maxima. No extra
unmeasured threshold or unsafe pointer path is introduced.

## Every native touched row

A = allocating; C = caller-buffer. A maximum crossing its bar fails even
when its median is lower. D146 is shown as stage1+30; unresolved intervals fail.

| API | Case | Before | After | Final 95% interval | Pairs | Registered max | Brief max |
|---|---|---:|---:|---|---:|---|---|
| A | vertex-v0-tiny-s4 | 1.651 | 1.495 | 1.259–1.452 | 20+30 | FAIL | pass |
| A | view-none-tiny-s4 | 1.693 | 1.447 | 1.331–1.499 | 16 | FAIL | pass |
| A | vertex-v0-tiny-s12 | 1.847 | 1.234 | 1.330–1.469 | 20+30 | FAIL | pass |
| A | view-none-tiny-s12 | 1.873 | 1.494 | 1.265–1.410 | 20+30 | FAIL | pass |
| A | vertex-v0-tiny-s32 | 2.001 | 1.389 | 1.166–1.331 | 20+30 | FAIL | pass |
| A | view-none-tiny-s32 | 2.075 | 1.394 | 1.220–1.381 | 20+30 | FAIL | pass |
| A | view-1-tiny-s4 | 1.557 | 1.533 | 1.449–1.655 | 20+30 | FAIL | FAIL |
| A | view-1-tiny-s8 | 1.638 | 1.402 | 1.234–1.497 | 11 | pass | pass |
| A | view-2-tiny-s8 | 1.635 | 1.453 | 1.428–1.549 | 20+30 | FAIL | FAIL |
| A | view-3-tiny-s12 | 1.701 | 1.343 | 1.293–1.364 | 5 | pass | pass |
| A | index-3-v0-tiny-s2 | 1.285 | 1.207 | 1.080–1.451 | 7 | pass | pass |
| A | index-3-v0-tiny-s4 | 0.882 | 1.230 | 0.973–1.405 | 5 | pass | pass |
| A | index-3-v1-tiny-s2 | 1.289 | 1.223 | 1.175–1.263 | 5 | pass | pass |
| A | index-3-v1-tiny-s4 | 0.874 | 1.239 | 1.178–1.419 | 6 | pass | pass |
| A | vertex-v0-resident-s4 | 1.497 | 1.104 | 1.059–1.286 | 10 | pass | pass |
| A | view-none-resident-s4 | 1.510 | 1.076 | 0.946–1.108 | 5 | pass | pass |
| A | vertex-v0-resident-s12 | 2.085 | 1.108 | 0.902–1.290 | 6 | pass | pass |
| A | view-none-resident-s12 | 2.376 | 1.173 | 1.055–1.259 | 5 | pass | pass |
| A | vertex-v0-resident-s32 | 1.646 | 1.103 | 1.006–1.299 | 18 | pass | pass |
| A | view-none-resident-s32 | 2.132 | 1.109 | 1.023–1.188 | 5 | pass | pass |
| A | view-1-resident-s4 | 1.286 | 1.179 | 1.093–1.310 | 5 | pass | pass |
| A | view-1-resident-s8 | 1.965 | 1.212 | 1.118–1.374 | 5 | pass | pass |
| A | view-2-resident-s8 | 2.094 | 1.154 | 1.141–1.213 | 5 | pass | pass |
| A | view-3-resident-s12 | 2.495 | 1.060 | 1.002–1.152 | 5 | pass | pass |
| A | index-3-v0-resident-s2 | 1.118 | 0.982 | 0.947–0.987 | 5 | pass | pass |
| A | index-3-v0-resident-s4 | 1.099 | 0.966 | 0.947–1.020 | 5 | pass | pass |
| A | index-3-v1-resident-s2 | 1.106 | 1.073 | 0.782–1.135 | 5 | pass | pass |
| A | index-3-v1-resident-s4 | 1.145 | 0.998 | 0.771–1.311 | 5 | pass | pass |
| A | vertex-v0-streaming-s4 | 1.192 | 0.873 | 0.708–1.279 | 6 | pass | pass |
| A | view-none-streaming-s4 | 1.188 | 0.746 | 0.631–1.005 | 5 | pass | pass |
| A | vertex-v0-streaming-s12 | 1.528 | 0.968 | 0.788–1.093 | 5 | pass | pass |
| A | view-none-streaming-s12 | 1.494 | 1.009 | 0.896–1.139 | 5 | pass | pass |
| A | vertex-v0-streaming-s32 | 1.372 | 1.062 | 0.843–1.177 | 5 | pass | pass |
| A | view-none-streaming-s32 | 1.342 | 1.147 | 1.021–1.195 | 5 | pass | pass |
| A | view-1-streaming-s4 | 2.057 | 1.990 | 1.538–2.163 | 10 | FAIL | FAIL |
| A | view-1-streaming-s8 | 1.531 | 0.994 | 0.807–1.146 | 5 | pass | pass |
| A | view-2-streaming-s8 | 1.624 | 1.053 | 0.989–1.228 | 5 | pass | pass |
| A | view-3-streaming-s12 | 1.629 | 1.100 | 0.926–1.316 | 5 | pass | pass |
| A | index-3-v0-streaming-s2 | 1.213 | 0.795 | 0.766–1.356 | 5 | pass | pass |
| A | index-3-v0-streaming-s4 | 2.882 | 2.523 | 1.680–3.197 | 10 | FAIL | FAIL |
| A | index-3-v1-streaming-s2 | 1.141 | 0.971 | 0.794–1.352 | 9 | pass | pass |
| A | index-3-v1-streaming-s4 | 2.888 | 2.876 | 1.841–3.803 | 5 | FAIL | FAIL |
| A | vertex-v1-tiny-s4 | 1.665 | 1.408 | 1.369–1.512 | 20+30 | FAIL | FAIL |
| A | vertex-v1-tiny-s12 | 1.998 | 1.429 | 1.369–1.494 | 15 | FAIL | pass |
| A | vertex-v1-tiny-s32 | 2.097 | 1.391 | 1.352–1.402 | 5 | FAIL | pass |
| A | vertex-v1-resident-s4 | 1.561 | 1.153 | 1.046–1.199 | 5 | pass | pass |
| A | vertex-v1-resident-s12 | 2.095 | 1.142 | 1.066–1.180 | 5 | pass | pass |
| A | vertex-v1-resident-s32 | 2.169 | 1.250 | 1.100–1.293 | 10 | pass | pass |
| A | vertex-v1-streaming-s4 | 2.095 | 2.050 | 1.893–2.208 | 5 | FAIL | FAIL |
| A | vertex-v1-streaming-s12 | 1.524 | 1.071 | 0.859–1.264 | 6 | pass | pass |
| A | vertex-v1-streaming-s32 | 1.646 | 1.044 | 0.923–1.200 | 5 | pass | pass |
| A | vertex-v0-resident-s12-level0 | 2.301 | 1.184 | 0.942–1.208 | 5 | pass | pass |
| A | vertex-v0-resident-s12-level9 | 2.267 | 1.139 | 1.115–1.165 | 5 | pass | pass |
| A | vertex-v1-resident-s12-level0 | 2.367 | 1.115 | 1.028–1.188 | 5 | pass | pass |
| A | vertex-v1-resident-s12-level9 | 2.428 | 1.132 | 1.126–1.195 | 5 | pass | pass |
| A | varied-view-1-tiny-s4 | 1.972 | 1.570 | 1.555–1.700 | 5 | FAIL | FAIL |
| A | varied-view-1-tiny-s8 | 1.779 | 1.405 | 1.326–1.437 | 5 | pass | pass |
| A | varied-view-2-tiny-s8 | 1.914 | 1.413 | 1.351–1.438 | 20+30 | pass | pass |
| A | varied-view-3-tiny-s4 | 1.543 | 1.408 | 1.357–1.477 | 6 | pass | pass |
| A | varied-view-3-tiny-s12 | 1.856 | 1.330 | 1.281–1.370 | 5 | pass | pass |
| A | varied-view-3-tiny-s32 | 1.929 | 1.324 | 1.273–1.444 | 5 | pass | pass |
| A | varied-filter-4-tiny-s4 | 1.521 | 1.378 | 1.210–1.482 | 7 | pass | pass |
| A | varied-filter-4-tiny-s8 | 1.371 | 1.352 | 1.324–1.468 | 5 | pass | pass |
| A | varied-view-1-resident-s4 | 1.894 | 1.435 | 1.316–1.498 | 7 | pass | pass |
| A | varied-view-1-resident-s8 | 1.983 | 1.312 | 1.299–1.389 | 5 | pass | pass |
| A | varied-view-2-resident-s8 | 2.088 | 1.243 | 0.810–1.481 | 5 | pass | pass |
| A | varied-view-3-resident-s4 | 1.489 | 1.057 | 0.973–1.074 | 5 | pass | pass |
| A | varied-view-3-resident-s12 | 2.188 | 1.085 | 1.030–1.319 | 5 | pass | pass |
| A | varied-view-3-resident-s32 | 2.179 | 1.124 | 1.054–1.170 | 5 | pass | pass |
| A | varied-filter-4-resident-s4 | 1.656 | 1.480 | 1.489–1.621 | 20+30 | FAIL | FAIL |
| A | varied-filter-4-resident-s8 | 1.315 | 1.267 | 1.111–1.482 | 8 | pass | pass |
| A | varied-view-1-streaming-s4 | 1.682 | 1.370 | 1.326–1.423 | 5 | pass | pass |
| A | varied-view-1-streaming-s8 | 2.009 | 1.196 | 1.116–1.296 | 5 | pass | pass |
| A | varied-view-2-streaming-s8 | 1.876 | 1.237 | 1.105–1.426 | 5 | pass | pass |
| A | varied-view-3-streaming-s4 | 1.441 | 1.089 | 0.919–1.178 | 5 | pass | pass |
| A | varied-view-3-streaming-s12 | 2.032 | 1.195 | 1.063–1.286 | 5 | pass | pass |
| A | varied-view-3-streaming-s32 | 3.003 | 2.261 | 1.605–2.688 | 8 | FAIL | FAIL |
| A | varied-filter-4-streaming-s4 | 1.918 | 1.565 | 1.516–1.663 | 20+30 | FAIL | FAIL |
| A | varied-filter-4-streaming-s8 | 1.533 | 1.360 | 1.354–1.436 | 20+30 | pass | pass |
| A | meshlet-1-1-v2-t3 | 1.253 | 1.383 | 1.294–1.416 | 20+30 | FAIL | pass |
| A | meshlet-1-1-v2-t4 | 1.346 | 1.384 | 1.303–1.386 | 19 | FAIL | pass |
| A | meshlet-1-1-v4-t3 | 1.294 | 1.353 | 1.302–1.480 | 18 | FAIL | pass |
| A | meshlet-1-1-v4-t4 | 1.233 | 1.319 | 1.203–1.368 | 20+30 | FAIL | pass |
| A | meshlet-raw-1-1 | 1.074 | 1.188 | 1.162–1.246 | 5 | pass | pass |
| A | meshlet-64-126-v2-t3 | 1.458 | 1.558 | 1.502–1.628 | 18 | FAIL | FAIL |
| A | meshlet-64-126-v2-t4 | 1.241 | 1.220 | 1.088–1.299 | 9 | pass | pass |
| A | meshlet-64-126-v4-t3 | 1.457 | 1.479 | 1.410–1.567 | 20+30 | FAIL | FAIL |
| A | meshlet-64-126-v4-t4 | 1.167 | 1.278 | 1.179–1.299 | 7 | pass | pass |
| A | meshlet-raw-64-126 | 1.369 | 1.214 | 0.869–1.291 | 12 | pass | pass |
| A | meshlet-256-256-v2-t3 | 1.520 | 1.433 | 1.333–1.491 | 14 | FAIL | pass |
| A | meshlet-256-256-v2-t4 | 1.371 | 1.148 | 0.962–1.191 | 5 | pass | pass |
| A | meshlet-256-256-v4-t3 | 1.440 | 1.402 | 1.311–1.471 | 6 | FAIL | pass |
| A | meshlet-256-256-v4-t4 | 1.222 | 1.105 | 1.069–1.239 | 5 | pass | pass |
| A | meshlet-raw-256-256 | 1.260 | 1.144 | 1.116–1.148 | 5 | pass | pass |
| C | vertex-v0-tiny-s4 | 1.869 | 1.508 | 1.502–1.654 | 20+30 | FAIL | FAIL |
| C | view-none-tiny-s4 | 1.805 | 1.547 | 1.503–1.547 | 20+30 | FAIL | FAIL |
| C | vertex-v0-tiny-s12 | 2.200 | 1.430 | 1.356–1.496 | 8 | FAIL | pass |
| C | view-none-tiny-s12 | 1.952 | 1.433 | 1.333–1.483 | 5 | FAIL | pass |
| C | vertex-v0-tiny-s32 | 1.919 | 1.363 | 1.311–1.360 | 20+30 | FAIL | pass |
| C | view-none-tiny-s32 | 2.083 | 1.321 | 1.305–1.364 | 6 | FAIL | pass |
| C | view-1-tiny-s4 | 1.612 | 1.543 | 1.504–1.591 | 20 | FAIL | FAIL |
| C | view-1-tiny-s8 | 1.565 | 1.436 | 1.415–1.452 | 5 | pass | pass |
| C | view-2-tiny-s8 | 1.672 | 1.475 | 1.459–1.500 | 13 | pass | pass |
| C | view-3-tiny-s12 | 1.693 | 1.378 | 1.325–1.449 | 5 | pass | pass |
| C | index-3-v0-tiny-s2 | 0.961 | 1.299 | 1.157–1.331 | 5 | pass | pass |
| C | index-3-v0-tiny-s4 | 0.923 | 1.240 | 1.155–1.282 | 5 | pass | pass |
| C | index-3-v1-tiny-s2 | 0.960 | 1.287 | 1.192–1.337 | 5 | pass | pass |
| C | index-3-v1-tiny-s4 | 0.974 | 1.207 | 1.193–1.248 | 5 | pass | pass |
| C | vertex-v0-resident-s4 | 1.516 | 1.073 | 1.054–1.091 | 5 | pass | pass |
| C | view-none-resident-s4 | 1.506 | 1.062 | 1.044–1.081 | 5 | pass | pass |
| C | vertex-v0-resident-s12 | 2.229 | 1.127 | 1.085–1.156 | 5 | pass | pass |
| C | view-none-resident-s12 | 2.207 | 1.108 | 1.080–1.149 | 5 | pass | pass |
| C | vertex-v0-resident-s32 | 2.287 | 1.127 | 1.105–1.197 | 5 | pass | pass |
| C | view-none-resident-s32 | 2.263 | 1.164 | 1.024–1.207 | 5 | pass | pass |
| C | view-1-resident-s4 | 1.255 | 1.278 | 1.160–1.291 | 5 | pass | pass |
| C | view-1-resident-s8 | 1.890 | 1.058 | 0.970–1.300 | 5 | pass | pass |
| C | view-2-resident-s8 | 2.066 | 1.076 | 0.962–1.338 | 5 | pass | pass |
| C | view-3-resident-s12 | 2.352 | 1.035 | 0.851–1.239 | 5 | pass | pass |
| C | index-3-v0-resident-s2 | 1.140 | 0.994 | 0.919–1.082 | 5 | pass | pass |
| C | index-3-v0-resident-s4 | 1.115 | 0.987 | 0.983–1.016 | 5 | pass | pass |
| C | index-3-v1-resident-s2 | 1.087 | 0.786 | 0.682–1.160 | 5 | pass | pass |
| C | index-3-v1-resident-s4 | 1.110 | 1.037 | 0.881–1.288 | 5 | pass | pass |
| C | vertex-v0-streaming-s4 | 1.514 | 0.877 | 0.813–0.986 | 5 | pass | pass |
| C | view-none-streaming-s4 | 1.497 | 0.864 | 0.774–1.100 | 5 | pass | pass |
| C | vertex-v0-streaming-s12 | 1.978 | 0.973 | 0.917–1.275 | 8 | pass | pass |
| C | view-none-streaming-s12 | 1.753 | 1.074 | 1.028–1.109 | 5 | pass | pass |
| C | vertex-v0-streaming-s32 | 1.989 | 0.987 | 0.828–1.270 | 11 | pass | pass |
| C | view-none-streaming-s32 | 1.945 | 1.220 | 1.017–1.297 | 17 | pass | pass |
| C | view-1-streaming-s4 | 1.293 | 0.923 | 0.870–0.996 | 5 | pass | pass |
| C | view-1-streaming-s8 | 2.019 | 0.975 | 0.886–1.178 | 5 | pass | pass |
| C | view-2-streaming-s8 | 2.185 | 0.945 | 0.770–1.306 | 5 | pass | pass |
| C | view-3-streaming-s12 | 2.247 | 0.875 | 0.614–1.307 | 5 | pass | pass |
| C | index-3-v0-streaming-s2 | 1.083 | 1.000 | 0.725–1.062 | 5 | pass | pass |
| C | index-3-v0-streaming-s4 | 1.127 | 0.970 | 0.774–1.262 | 5 | pass | pass |
| C | index-3-v1-streaming-s2 | 1.132 | 1.239 | 0.983–1.465 | 9 | pass | pass |
| C | index-3-v1-streaming-s4 | 1.119 | 1.027 | 0.819–1.188 | 5 | pass | pass |
| C | vertex-v1-tiny-s4 | 1.834 | 1.509 | 1.333–1.500 | 20+30 | FAIL | FAIL |
| C | vertex-v1-tiny-s12 | 1.954 | 1.230 | 1.260–1.422 | 20+30 | FAIL | pass |
| C | vertex-v1-tiny-s32 | 1.878 | 1.183 | 1.146–1.263 | 5 | pass | pass |
| C | vertex-v1-resident-s4 | 1.520 | 1.030 | 0.931–1.209 | 5 | pass | pass |
| C | vertex-v1-resident-s12 | 2.217 | 1.121 | 1.023–1.276 | 8 | pass | pass |
| C | vertex-v1-resident-s32 | 2.233 | 1.102 | 0.941–1.195 | 5 | pass | pass |
| C | vertex-v1-streaming-s4 | 1.449 | 0.745 | 0.613–0.802 | 5 | pass | pass |
| C | vertex-v1-streaming-s12 | 2.164 | 0.655 | 0.610–1.151 | 5 | pass | pass |
| C | vertex-v1-streaming-s32 | 2.129 | 0.946 | 0.829–1.281 | 16 | pass | pass |
| C | vertex-v0-resident-s12-level0 | 2.217 | 1.167 | 1.154–1.181 | 5 | pass | pass |
| C | vertex-v0-resident-s12-level9 | 2.185 | 1.175 | 1.079–1.232 | 5 | pass | pass |
| C | vertex-v1-resident-s12-level0 | 2.256 | 1.225 | 1.136–1.255 | 5 | pass | pass |
| C | vertex-v1-resident-s12-level9 | 2.162 | 1.173 | 1.136–1.286 | 13 | pass | pass |
| C | varied-view-1-tiny-s4 | 1.697 | 1.670 | 1.503–1.862 | 5 | FAIL | FAIL |
| C | varied-view-1-tiny-s8 | 1.625 | 1.372 | 1.249–1.495 | 6 | pass | pass |
| C | varied-view-2-tiny-s8 | 1.737 | 1.229 | 1.183–1.327 | 5 | pass | pass |
| C | varied-view-3-tiny-s4 | 1.557 | 1.192 | 1.112–1.459 | 7 | pass | pass |
| C | varied-view-3-tiny-s12 | 1.803 | 1.153 | 1.149–1.206 | 5 | pass | pass |
| C | varied-view-3-tiny-s32 | 1.840 | 1.162 | 1.075–1.225 | 5 | pass | pass |
| C | varied-filter-4-tiny-s4 | 1.423 | 1.432 | 1.392–1.499 | 8 | pass | pass |
| C | varied-filter-4-tiny-s8 | 1.162 | 1.215 | 1.187–1.298 | 5 | pass | pass |
| C | varied-view-1-resident-s4 | 1.735 | 1.447 | 1.505–1.551 | 20+30 | FAIL | FAIL |
| C | varied-view-1-resident-s8 | 2.068 | 1.236 | 1.222–1.345 | 5 | pass | pass |
| C | varied-view-2-resident-s8 | 2.070 | 1.246 | 1.113–1.355 | 5 | pass | pass |
| C | varied-view-3-resident-s4 | 1.494 | 0.990 | 0.942–1.074 | 5 | pass | pass |
| C | varied-view-3-resident-s12 | 2.168 | 0.961 | 0.997–1.225 | 5 | pass | pass |
| C | varied-view-3-resident-s32 | 2.145 | 1.160 | 1.108–1.215 | 5 | pass | pass |
| C | varied-filter-4-resident-s4 | 1.748 | 1.519 | 1.506–1.652 | 12 | FAIL | FAIL |
| C | varied-filter-4-resident-s8 | 1.509 | 1.369 | 1.326–1.448 | 5 | pass | pass |
| C | varied-view-1-streaming-s4 | 1.749 | 1.490 | 1.470–1.568 | 20+30 | FAIL | FAIL |
| C | varied-view-1-streaming-s8 | 2.050 | 1.274 | 1.258–1.301 | 5 | pass | pass |
| C | varied-view-2-streaming-s8 | 2.095 | 1.218 | 1.170–1.266 | 5 | pass | pass |
| C | varied-view-3-streaming-s4 | 1.480 | 1.041 | 1.018–1.048 | 5 | pass | pass |
| C | varied-view-3-streaming-s12 | 2.157 | 1.117 | 0.968–1.465 | 5 | pass | pass |
| C | varied-view-3-streaming-s32 | 2.171 | 1.097 | 1.051–1.150 | 5 | pass | pass |
| C | varied-filter-4-streaming-s4 | 1.731 | 1.543 | 1.515–1.621 | 6 | FAIL | FAIL |
| C | varied-filter-4-streaming-s8 | 1.484 | 1.317 | 1.339–1.394 | 20+30 | pass | pass |
| C | meshlet-1-1-v2-t3 | 1.933 | 1.904 | 1.763–1.940 | 5 | FAIL | FAIL |
| C | meshlet-1-1-v2-t4 | 1.925 | 2.028 | 1.553–2.284 | 7 | FAIL | FAIL |
| C | meshlet-1-1-v4-t3 | 1.915 | 2.202 | 1.738–2.239 | 5 | FAIL | FAIL |
| C | meshlet-1-1-v4-t4 | 1.927 | 1.882 | 1.696–2.021 | 5 | FAIL | FAIL |
| C | meshlet-raw-1-1 | 1.410 | 1.578 | 1.507–1.685 | 8 | FAIL | FAIL |
| C | meshlet-64-126-v2-t3 | 1.503 | 1.532 | 1.504–1.690 | 20+30 | FAIL | FAIL |
| C | meshlet-64-126-v2-t4 | 1.349 | 1.242 | 1.216–1.319 | 20+30 | FAIL | pass |
| C | meshlet-64-126-v4-t3 | 1.501 | 1.650 | 1.503–1.690 | 8 | FAIL | FAIL |
| C | meshlet-64-126-v4-t4 | 1.387 | 1.344 | 1.202–1.329 | 20+30 | FAIL | pass |
| C | meshlet-raw-64-126 | 1.316 | 1.214 | 1.174–1.247 | 20+30 | pass | pass |
| C | meshlet-256-256-v2-t3 | 1.392 | 1.483 | 1.347–1.461 | 20+30 | FAIL | pass |
| C | meshlet-256-256-v2-t4 | 1.242 | 1.141 | 1.136–1.213 | 5 | pass | pass |
| C | meshlet-256-256-v4-t3 | 1.353 | 1.443 | 1.316–1.465 | 6 | FAIL | pass |
| C | meshlet-256-256-v4-t4 | 1.252 | 1.298 | 1.063–1.289 | 5 | pass | pass |
| C | meshlet-raw-256-256 | 1.226 | 1.146 | 1.124–1.149 | 5 | pass | pass |

## Every Node touched row

| API | Case | Before | After | Final 95% interval | Pairs | Registered 1.60 max | Brief 1.50 max |
|---|---|---:|---:|---|---:|---|---|
| A | vertex-v0-tiny-s4 | 1.228 | 1.173 | 0.991–1.279 | 20+30 | pass | pass |
| A | view-none-tiny-s4 | 1.101 | 1.201 | 0.762–1.464 | 5 | pass | pass |
| A | vertex-v0-tiny-s12 | 1.320 | 1.108 | 1.019–1.370 | 5 | pass | pass |
| A | view-none-tiny-s12 | 1.518 | 1.225 | 1.039–1.453 | 5 | pass | pass |
| A | vertex-v0-tiny-s32 | 1.332 | 1.135 | 0.981–1.285 | 5 | pass | pass |
| A | view-none-tiny-s32 | 1.568 | 0.943 | 0.881–1.225 | 5 | pass | pass |
| A | view-1-tiny-s4 | 0.834 | 0.931 | 0.581–1.490 | 19 | pass | pass |
| A | view-1-tiny-s8 | 1.380 | 1.016 | 0.988–1.483 | 7 | pass | pass |
| A | view-2-tiny-s8 | 1.105 | 0.897 | 0.496–1.492 | 14 | pass | pass |
| A | view-3-tiny-s12 | 1.306 | 1.121 | 0.842–1.445 | 8 | pass | pass |
| A | index-3-v0-tiny-s2 | 0.984 | 0.686 | 0.256–1.290 | 8 | pass | pass |
| A | index-3-v0-tiny-s4 | 1.136 | 0.920 | 0.673–1.111 | 5 | pass | pass |
| A | index-3-v1-tiny-s2 | 0.938 | 1.074 | 0.839–1.384 | 6 | pass | pass |
| A | index-3-v1-tiny-s4 | 0.921 | 0.989 | 0.844–1.309 | 5 | pass | pass |
| A | vertex-v0-resident-s4 | 1.232 | 0.877 | 0.886–1.087 | 5 | pass | pass |
| A | view-none-resident-s4 | 1.218 | 1.136 | 0.934–1.356 | 5 | pass | pass |
| A | vertex-v0-resident-s12 | 1.436 | 1.085 | 0.808–1.365 | 5 | pass | pass |
| A | view-none-resident-s12 | 1.451 | 1.219 | 0.987–1.299 | 5 | pass | pass |
| A | vertex-v0-resident-s32 | 1.333 | 1.401 | 1.194–1.498 | 6 | pass | pass |
| A | view-none-resident-s32 | 1.300 | 1.152 | 1.098–1.185 | 5 | pass | pass |
| A | view-1-resident-s4 | 1.802 | 1.575 | 1.460–1.574 | 20+30 | pass | FAIL |
| A | view-1-resident-s8 | 2.290 | 1.748 | 1.626–1.835 | 7 | FAIL | FAIL |
| A | view-2-resident-s8 | 1.980 | 1.683 | 1.601–1.755 | 7 | FAIL | FAIL |
| A | view-3-resident-s12 | 2.344 | 1.515 | 1.464–1.582 | 20+30 | pass | FAIL |
| A | index-3-v0-resident-s2 | 0.975 | 0.858 | 0.821–0.914 | 5 | pass | pass |
| A | index-3-v0-resident-s4 | 0.853 | 0.830 | 0.759–0.946 | 5 | pass | pass |
| A | index-3-v1-resident-s2 | 1.090 | 0.958 | 0.846–0.959 | 5 | pass | pass |
| A | index-3-v1-resident-s4 | 1.056 | 0.835 | 0.662–1.173 | 5 | pass | pass |
| A | vertex-v0-streaming-s4 | 1.144 | 0.977 | 0.640–1.476 | 6 | pass | pass |
| A | view-none-streaming-s4 | 1.093 | 1.068 | 0.790–1.467 | 12 | pass | pass |
| A | vertex-v0-streaming-s12 | 1.354 | 1.237 | 1.017–1.489 | 10 | pass | pass |
| A | view-none-streaming-s12 | 1.020 | 0.916 | 0.687–1.291 | 5 | pass | pass |
| A | vertex-v0-streaming-s32 | 1.127 | 1.022 | 0.833–1.332 | 5 | pass | pass |
| A | view-none-streaming-s32 | 1.154 | 1.086 | 0.978–1.172 | 5 | pass | pass |
| A | view-1-streaming-s4 | 1.811 | 1.510 | 1.507–1.565 | 20+30 | pass | FAIL |
| A | view-1-streaming-s8 | 2.537 | 1.741 | 1.671–2.087 | 5 | FAIL | FAIL |
| A | view-2-streaming-s8 | 2.320 | 1.709 | 1.604–1.865 | 7 | FAIL | FAIL |
| A | view-3-streaming-s12 | 1.448 | 1.005 | 0.908–1.419 | 6 | pass | pass |
| A | index-3-v0-streaming-s2 | 1.055 | 0.606 | 0.479–1.244 | 5 | pass | pass |
| A | index-3-v0-streaming-s4 | 1.059 | 1.026 | 0.679–1.485 | 6 | pass | pass |
| A | index-3-v1-streaming-s2 | 0.988 | 0.835 | 0.591–1.010 | 5 | pass | pass |
| A | index-3-v1-streaming-s4 | 0.987 | 0.899 | 0.612–1.461 | 5 | pass | pass |
| A | vertex-v1-tiny-s4 | 1.034 | 1.121 | 0.529–1.490 | 8 | pass | pass |
| A | vertex-v1-tiny-s12 | 1.388 | 1.309 | 1.152–1.480 | 15 | pass | pass |
| A | vertex-v1-tiny-s32 | 1.698 | 1.128 | 0.992–1.344 | 5 | pass | pass |
| A | vertex-v1-resident-s4 | 1.189 | 1.056 | 0.971–1.263 | 5 | pass | pass |
| A | vertex-v1-resident-s12 | 1.437 | 1.226 | 1.117–1.447 | 5 | pass | pass |
| A | vertex-v1-resident-s32 | 1.353 | 1.191 | 1.092–1.302 | 5 | pass | pass |
| A | vertex-v1-streaming-s4 | 1.206 | 1.157 | 0.851–1.458 | 12 | pass | pass |
| A | vertex-v1-streaming-s12 | 1.463 | 0.869 | 0.668–1.214 | 5 | pass | pass |
| A | vertex-v1-streaming-s32 | 1.144 | 0.900 | 0.788–1.328 | 5 | pass | pass |
| A | vertex-v0-resident-s12-level0 | 1.331 | 1.238 | 1.140–1.368 | 5 | pass | pass |
| A | vertex-v0-resident-s12-level9 | 1.426 | 1.144 | 0.946–1.450 | 11 | pass | pass |
| A | vertex-v1-resident-s12-level0 | 1.407 | 1.218 | 0.890–1.460 | 7 | pass | pass |
| A | vertex-v1-resident-s12-level9 | 1.488 | 1.165 | 1.000–1.364 | 5 | pass | pass |
| A | varied-view-1-tiny-s4 | 0.989 | 1.282 | 1.161–1.482 | 7 | pass | pass |
| A | varied-view-1-tiny-s8 | 1.369 | 1.245 | 1.171–1.433 | 5 | pass | pass |
| A | varied-view-2-tiny-s8 | 1.396 | 1.227 | 1.197–1.422 | 5 | pass | pass |
| A | varied-view-3-tiny-s4 | 1.125 | 1.336 | 1.124–1.478 | 10 | pass | pass |
| A | varied-view-3-tiny-s12 | 1.422 | 1.218 | 0.912–1.412 | 6 | pass | pass |
| A | varied-view-3-tiny-s32 | 1.636 | 1.129 | 1.017–1.404 | 5 | pass | pass |
| A | varied-view-1-resident-s4 | 1.874 | 1.515 | 1.566–1.711 | 20+30 | FAIL | FAIL |
| A | varied-view-1-resident-s8 | 1.938 | 1.558 | 1.435–1.556 | 20+30 | pass | FAIL |
| A | varied-view-2-resident-s8 | 1.916 | 1.385 | 1.300–1.495 | 19 | pass | pass |
| A | varied-view-3-resident-s4 | 1.285 | 1.066 | 0.979–1.288 | 5 | pass | pass |
| A | varied-view-3-resident-s12 | 1.615 | 1.206 | 1.094–1.468 | 5 | pass | pass |
| A | varied-view-3-resident-s32 | 1.731 | 1.350 | 1.212–1.423 | 5 | pass | pass |
| A | varied-view-1-streaming-s4 | 1.970 | 1.649 | 1.632–2.269 | 10 | FAIL | FAIL |
| A | varied-view-1-streaming-s8 | 1.818 | 1.824 | 1.267–1.699 | 20+30 | FAIL | FAIL |
| A | varied-view-2-streaming-s8 | 1.853 | 1.639 | 1.195–1.635 | 20+30 | FAIL | FAIL |
| A | varied-view-3-streaming-s4 | 1.266 | 1.072 | 0.979–1.210 | 5 | pass | pass |
| A | varied-view-3-streaming-s12 | 1.985 | 1.438 | 1.111–1.549 | 20+30 | pass | FAIL |
| A | varied-view-3-streaming-s32 | 1.726 | 1.266 | 1.182–1.495 | 18 | pass | pass |
| C | vertex-v0-tiny-s4 | 0.989 | 0.892 | 0.352–1.449 | 6 | pass | pass |
| C | view-none-tiny-s4 | 1.012 | 1.069 | 0.866–1.146 | 5 | pass | pass |
| C | vertex-v0-tiny-s12 | 1.331 | 1.083 | 0.965–1.205 | 5 | pass | pass |
| C | view-none-tiny-s12 | 1.319 | 1.029 | 0.882–1.206 | 5 | pass | pass |
| C | vertex-v0-tiny-s32 | 1.618 | 1.058 | 0.979–1.222 | 5 | pass | pass |
| C | view-none-tiny-s32 | 1.625 | 1.148 | 1.097–1.274 | 5 | pass | pass |
| C | view-1-tiny-s4 | 1.229 | 1.108 | 0.943–1.207 | 5 | pass | pass |
| C | view-1-tiny-s8 | 1.604 | 1.297 | 1.277–1.351 | 5 | pass | pass |
| C | view-2-tiny-s8 | 1.657 | 1.276 | 1.114–1.410 | 5 | pass | pass |
| C | view-3-tiny-s12 | 1.644 | 1.303 | 1.001–1.415 | 6 | pass | pass |
| C | index-3-v0-tiny-s2 | 0.789 | 0.837 | 0.707–1.106 | 5 | pass | pass |
| C | index-3-v0-tiny-s4 | 0.931 | 0.781 | 0.634–0.884 | 5 | pass | pass |
| C | index-3-v1-tiny-s2 | 0.749 | 0.691 | 0.472–1.247 | 5 | pass | pass |
| C | index-3-v1-tiny-s4 | 0.793 | 0.881 | 0.622–1.199 | 5 | pass | pass |
| C | vertex-v0-resident-s4 | 1.192 | 0.953 | 0.866–1.219 | 5 | pass | pass |
| C | view-none-resident-s4 | 1.169 | 1.004 | 0.992–1.108 | 5 | pass | pass |
| C | vertex-v0-resident-s12 | 1.423 | 1.163 | 1.113–1.191 | 5 | pass | pass |
| C | view-none-resident-s12 | 1.410 | 1.183 | 1.135–1.181 | 5 | pass | pass |
| C | vertex-v0-resident-s32 | 1.414 | 1.102 | 0.908–1.321 | 5 | pass | pass |
| C | view-none-resident-s32 | 1.387 | 1.235 | 1.116–1.474 | 8 | pass | pass |
| C | view-1-resident-s4 | 1.815 | 1.578 | 1.573–1.614 | 20+30 | FAIL | FAIL |
| C | view-1-resident-s8 | 2.740 | 2.007 | 1.916–2.101 | 5 | FAIL | FAIL |
| C | view-2-resident-s8 | 2.393 | 1.850 | 1.758–1.930 | 5 | FAIL | FAIL |
| C | view-3-resident-s12 | 2.497 | 1.612 | 1.550–1.669 | 20+30 | FAIL | FAIL |
| C | index-3-v0-resident-s2 | 0.990 | 0.902 | 0.800–0.968 | 5 | pass | pass |
| C | index-3-v0-resident-s4 | 1.007 | 0.819 | 0.730–0.904 | 5 | pass | pass |
| C | index-3-v1-resident-s2 | 0.997 | 0.842 | 0.813–0.864 | 5 | pass | pass |
| C | index-3-v1-resident-s4 | 0.985 | 0.842 | 0.810–0.879 | 5 | pass | pass |
| C | vertex-v0-streaming-s4 | 1.245 | 1.086 | 1.043–1.154 | 5 | pass | pass |
| C | view-none-streaming-s4 | 1.179 | 1.078 | 1.061–1.104 | 5 | pass | pass |
| C | vertex-v0-streaming-s12 | 1.577 | 1.235 | 1.159–1.416 | 5 | pass | pass |
| C | view-none-streaming-s12 | 1.544 | 1.304 | 1.241–1.408 | 5 | pass | pass |
| C | vertex-v0-streaming-s32 | 1.639 | 1.102 | 1.050–1.156 | 5 | pass | pass |
| C | view-none-streaming-s32 | 1.322 | 1.174 | 0.924–1.264 | 5 | pass | pass |
| C | view-1-streaming-s4 | 1.985 | 1.601 | 1.567–1.601 | 20+30 | FAIL | FAIL |
| C | view-1-streaming-s8 | 2.578 | 1.913 | 1.906–1.970 | 5 | FAIL | FAIL |
| C | view-2-streaming-s8 | 2.366 | 1.826 | 1.778–1.873 | 5 | FAIL | FAIL |
| C | view-3-streaming-s12 | 2.279 | 1.471 | 1.456–1.522 | 20+30 | pass | FAIL |
| C | index-3-v0-streaming-s2 | 1.027 | 0.844 | 0.837–0.876 | 5 | pass | pass |
| C | index-3-v0-streaming-s4 | 1.028 | 0.874 | 0.853–0.906 | 5 | pass | pass |
| C | index-3-v1-streaming-s2 | 0.989 | 0.847 | 0.844–0.859 | 5 | pass | pass |
| C | index-3-v1-streaming-s4 | 1.012 | 0.868 | 0.789–0.916 | 5 | pass | pass |
| C | vertex-v1-tiny-s4 | 0.998 | 0.921 | 0.261–1.372 | 5 | pass | pass |
| C | vertex-v1-tiny-s12 | 1.480 | 1.135 | 1.086–1.177 | 5 | pass | pass |
| C | vertex-v1-tiny-s32 | 1.838 | 1.273 | 1.196–1.373 | 5 | pass | pass |
| C | vertex-v1-resident-s4 | 1.199 | 1.058 | 0.978–1.114 | 5 | pass | pass |
| C | vertex-v1-resident-s12 | 1.447 | 1.120 | 1.102–1.207 | 5 | pass | pass |
| C | vertex-v1-resident-s32 | 1.439 | 1.159 | 1.136–1.185 | 5 | pass | pass |
| C | vertex-v1-streaming-s4 | 1.184 | 1.068 | 1.032–1.081 | 5 | pass | pass |
| C | vertex-v1-streaming-s12 | 1.446 | 1.152 | 1.084–1.166 | 5 | pass | pass |
| C | vertex-v1-streaming-s32 | 1.362 | 1.095 | 1.053–1.132 | 5 | pass | pass |
| C | vertex-v0-resident-s12-level0 | 1.419 | 1.103 | 0.676–1.398 | 6 | pass | pass |
| C | vertex-v0-resident-s12-level9 | 1.401 | 1.171 | 1.112–1.186 | 5 | pass | pass |
| C | vertex-v1-resident-s12-level0 | 1.427 | 1.216 | 0.910–1.446 | 5 | pass | pass |
| C | vertex-v1-resident-s12-level9 | 1.390 | 1.147 | 1.039–1.350 | 5 | pass | pass |
| C | varied-view-1-tiny-s4 | 1.252 | 1.043 | 0.966–1.083 | 5 | pass | pass |
| C | varied-view-1-tiny-s8 | 1.508 | 1.187 | 1.091–1.265 | 5 | pass | pass |
| C | varied-view-2-tiny-s8 | 1.479 | 1.062 | 1.033–1.236 | 5 | pass | pass |
| C | varied-view-3-tiny-s4 | 1.078 | 0.905 | 0.864–0.984 | 5 | pass | pass |
| C | varied-view-3-tiny-s12 | 1.501 | 0.988 | 0.994–1.127 | 5 | pass | pass |
| C | varied-view-3-tiny-s32 | 1.864 | 1.115 | 1.024–1.175 | 5 | pass | pass |
| C | varied-view-1-resident-s4 | 2.052 | 1.674 | 1.610–1.714 | 20+30 | FAIL | FAIL |
| C | varied-view-1-resident-s8 | 1.943 | 1.585 | 1.550–1.583 | 20+30 | pass | FAIL |
| C | varied-view-2-resident-s8 | 1.835 | 1.476 | 1.456–1.499 | 14 | pass | pass |
| C | varied-view-3-resident-s4 | 1.295 | 1.088 | 1.026–1.223 | 5 | pass | pass |
| C | varied-view-3-resident-s12 | 1.649 | 1.279 | 1.210–1.351 | 5 | pass | pass |
| C | varied-view-3-resident-s32 | 1.641 | 1.266 | 1.246–1.277 | 5 | pass | pass |
| C | varied-view-1-streaming-s4 | 1.964 | 1.680 | 1.609–1.727 | 9 | FAIL | FAIL |
| C | varied-view-1-streaming-s8 | 1.961 | 1.617 | 1.544–1.614 | 20+30 | FAIL | FAIL |
| C | varied-view-2-streaming-s8 | 1.889 | 1.495 | 1.430–1.512 | 20+30 | pass | FAIL |
| C | varied-view-3-streaming-s4 | 1.535 | 1.119 | 1.097–1.133 | 5 | pass | pass |
| C | varied-view-3-streaming-s12 | 1.774 | 1.377 | 1.338–1.380 | 5 | pass | pass |
| C | varied-view-3-streaming-s32 | 1.676 | 1.310 | 1.250–1.482 | 6 | pass | pass |
