# P07 fix-six scoped final performance — 2026-10-05

Rust/C++ time ratios; lower is faster. Before is the identical frozen scope
from fix five. Means use stage-one medians; maximum and S3 verdicts use final
paired 95% Student-t log-ratio intervals. Borderline-only fresh D146 supersedes
stage one for maxima/S3. An unresolved maximum fails. Different shared-host
epochs do not isolate the cause of a latency change. No rescue timing.

A = allocating; C = caller-buffer. Verdicts include applicable native varied-
filter S3. RFC: mean <=1.25, maximum <=1.50. Registered S1: 1.10/1.30;
S2: 1.25/1.50; S5 WASM: 1.25/1.60. Measured source is frozen; BAR is unchanged.

| Family | A before → after | C before → after | RFC A / C | Registered A / C |
|---|---:|---:|---|---|
| oct | 1.318 → 1.125 | 1.285 → 1.193 | pass / **FAIL** | pass / **FAIL** |
| meshlet | 1.212 → 1.221 | 1.417 → 1.409 | pass / **FAIL** | **FAIL** / **FAIL** |
| meshlet-raw | 0.997 → 1.088 | 1.337 → 1.306 | pass / **FAIL** | pass / **FAIL** |
| exp | 1.003 → 0.917 | 0.910 → 1.065 | **FAIL** / **FAIL** | **FAIL** / **FAIL** |
| vertex | 1.090 → 1.141 | 1.055 → 1.087 | **FAIL** / pass | **FAIL** / pass |
| view-none | 1.122 → 1.061 | 1.025 → 1.073 | pass / pass | **FAIL** / **FAIL** |
| view-filtered | 1.189 → 1.162 | 1.113 → 1.107 | **FAIL** / pass | **FAIL** / pass |
| quat | 1.220 → 1.170 | 1.199 → 1.235 | pass / **FAIL** | pass / **FAIL** |
| wasm eligible scope | 1.179 → 1.203 | 1.168 → 1.153 | **FAIL** / **FAIL** | **FAIL** / **FAIL** |

Native scope is eight families, 212 API rows. Unchanged Color and native
sequence are not timed. WASM retains fix five's 73 eligible cases, 146 API
rows (vertex/views/sequence), using upstream shipped JS SIMD. JS exposes no
standalone filters or meshlet API. Aggregate S2 below is the touched subset;
this round does not requalify the full matrix or other platforms.

## S3 and maximum failures

| Prior S3 caller Exp case | Fix-five interval | Fix-six interval | Significant new slowdown |
|---|---|---|---|
| varied-filter-3-tiny-s32 | 1.0294–1.1441 | 1.8624–2.3610 | yes |
| varied-filter-3-resident-s32 | 1.0905–1.1188 | 1.0703–1.1177 | yes |

Final registered varied-filter S3 significant comparisons: **4**.
- allocating / varied-filter-3-streaming-s12 / rust: 1.0035–1.0578.
- caller-buffer / varied-filter-3-tiny-s12 / rust: 2.0598–2.7099.
- caller-buffer / varied-filter-3-tiny-s32 / rust: 1.8624–2.3610.
- caller-buffer / varied-filter-3-resident-s32 / rust: 1.0703–1.1177.
- S1 touched allocating: mean 1.141; 11 registered maximum failures; FAIL for ratio gates on these rows.
- S1 touched caller-buffer: mean 1.174; 10 registered maximum failures; FAIL for ratio gates on these rows.
- S2 touched allocating: mean 1.102; 2 registered maximum failures; FAIL for ratio gates on these rows.
- S2 touched caller-buffer: mean 1.127; 4 registered maximum failures; FAIL for ratio gates on these rows.
- S5 allocating: mean 1.203; 9 registered maximum failures; FAIL.
- S5 caller-buffer: mean 1.153; 8 registered maximum failures; FAIL.

## Development and decisions

Instruction-count bisect on retained r3/r4/r5 binaries finds identical Oct
work: tiny s4 13.265, resident s4 5.782, resident s8 3.016 instructions/byte
caller-buffer. Its kernel source is unchanged. These counters do not identify
a responsible round-four/five edit; code placement, scheduling and host latency
remain unisolated. Keep that attribution unresolved. Exact MIN/XOR reflection
and comparison-derived half bias reduce resident Oct work to 4.970/2.610 and
filtered Oct view work 9.800 → 8.988; no approximate arithmetic/reassociation.

Meshlet callers do no timed decoder clear or copy. Typed prior caller is faster
in absolute time (107.49ns vs 115.38ns allocating), with a stronger C++ caller
baseline. Raw prior caller is slower (186.99ns vs 108.33ns), but has lower
instruction work (3.314 vs 3.951/byte). Reject forced wrapper inlining: typed
4.281 vs 4.279, raw 3.362 vs 3.314. Reject packed-u32 metadata: typed 4.528,
raw 3.458. Restore original code and explicitly abandon further meshlet edits
for this round. The final measurements retain its residual failures.

x86 Exp runs the unchanged canonical safe loop, already baseline-SSE2-
vectorized by LLVM, without a separate runtime dispatch/manual 64-byte loop.
Resident work rises 0.674 → 0.720/byte (scalar comparator 0.721); this convergence
targets the two named S3 cases. Both remain failed, and the tiny s32 slowdown
is worse. Two additional Exp comparisons fail. Code-path convergence did not
fix S3; abandon further Exp tuning under the completed one-pass rule. No
post-measurement source change or rescue sample. Other ISA kernels/reference stay.
Tiny allocating preflight avoids two integer divisions for <=32 records, the
minimum valid block size; keep byte/work/error semantics. Regression covers
zero and 31/32/33, widths through 256, both versions, malformed lengths,
limits and tails. Its initial zero-count EXT assertion was corrected because
that metadata is invalid; retain the failed test log.

WASM restores one 1024-byte vertex scratch parser and canonical u64 Quat
rotations following SIMD math. Checked resident raw vertex instruction work
recovers from about 12.362/12.376 to 12.047/12.137 in the scratch-only trial,
versus round-four 12.024/12.129. Quat view work falls 13.988 → 13.324, versus
13.419 before round five. V8 counters include runtime/tiering; they are diagnostic.
Priority-two views, allocating vertex/Exp and WASM maxima receive these exact
changes. Further allocator/zero-fill or arbitrary size-crossover changes have
no demonstrated exact gain and are abandoned before final timing.

## Correctness, safety and identities

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07-fix6`; independently verified
summary.json and final-evidence.json retain identities and receipts. Frozen
fixture/malformed/benchmark proofs pass on both APIs, local native ceilings and
actual wasm scalar/SIMD. Node timing modules match 292 golden output hashes.
Native and actual SIMD wasm cover all 16,777,216 Oct8 direction words, one
million Oct16 and Quat records and all edge combinations. This is exhaustive
Oct8 plus finite larger-word proof, not a broader release requalification.
The existing valid-alpha Color proof and pinned odd-tail scalar/SIMD status
split are reproduced unchanged. Seven test/Clippy/Miri receipts, MSRV/no-std
portability and the 23-block single-unsafe-module/package gates pass. No new
unsafe block or scalar-reference arithmetic. No ARM execution, dedicated-host,
Windows/macOS, Moss integration, native triangle-index or phase-0.6/release
qualification is claimed.

## Timing receipts

One final epoch, both maximum sets assessed together. Five to twenty interleaved
pairs with early stopping; thirty fresh D146 pairs only when an applicable
maximum remains borderline. Accepted pairs/rows are not repeated. Visible
heavy-4GB timeout-840 bursts stop at a 690-second pair-boundary budget.
- native native-timing-burst1.log: wrapper 226s, exit zero.
- wasm wasm-timing-burst1.log: wrapper 188s, exit zero.

## Every native row

| API | Case | Before | After | Final 95% interval | Pairs | RFC max | Registered max |
|---|---|---:|---:|---|---:|---|---|
| allocating | vertex-v0-tiny-s4 | 1.281 | 1.348 | 1.304–1.474 | 14 | pass | FAIL |
| allocating | view-none-tiny-s4 | 1.394 | 1.327 | 1.269–1.373 | 20+30 | pass | FAIL |
| allocating | vertex-v0-tiny-s12 | 1.285 | 1.199 | 1.087–1.299 | 6 | pass | pass |
| allocating | view-none-tiny-s12 | 1.326 | 1.230 | 1.194–1.294 | 11 | pass | pass |
| allocating | vertex-v0-tiny-s32 | 1.277 | 1.150 | 0.937–1.293 | 10 | pass | pass |
| allocating | view-none-tiny-s32 | 1.240 | 1.109 | 1.063–1.163 | 5 | pass | pass |
| allocating | filter-1-tiny-s4 | 1.409 | 1.174 | 0.959–1.464 | 5 | pass | pass |
| allocating | view-1-tiny-s4 | 1.420 | 1.240 | 1.006–1.496 | 6 | pass | pass |
| allocating | filter-1-tiny-s8 | 1.239 | 1.054 | 0.825–1.405 | 5 | pass | pass |
| allocating | view-1-tiny-s8 | 1.274 | 1.186 | 1.151–1.205 | 5 | pass | pass |
| allocating | filter-2-tiny-s8 | 1.277 | 1.221 | 1.142–1.464 | 7 | pass | pass |
| allocating | view-2-tiny-s8 | 1.316 | 1.272 | 1.215–1.286 | 5 | pass | pass |
| allocating | filter-3-tiny-s12 | 0.863 | 1.004 | 0.931–1.193 | 5 | pass | pass |
| allocating | view-3-tiny-s12 | 1.286 | 1.163 | 1.144–1.161 | 5 | pass | pass |
| allocating | vertex-v0-resident-s4 | 1.006 | 1.054 | 0.926–1.163 | 5 | pass | pass |
| allocating | view-none-resident-s4 | 1.074 | 1.051 | 1.004–1.071 | 5 | pass | pass |
| allocating | vertex-v0-resident-s12 | 1.160 | 1.120 | 1.104–1.155 | 5 | pass | pass |
| allocating | view-none-resident-s12 | 1.119 | 1.112 | 1.109–1.126 | 5 | pass | pass |
| allocating | vertex-v0-resident-s32 | 1.028 | 1.161 | 1.109–1.203 | 5 | pass | pass |
| allocating | view-none-resident-s32 | 1.089 | 1.134 | 1.125–1.145 | 5 | pass | pass |
| allocating | filter-1-resident-s4 | 1.049 | 0.951 | 0.883–0.982 | 5 | pass | pass |
| allocating | view-1-resident-s4 | 1.022 | 0.958 | 0.913–1.042 | 5 | pass | pass |
| allocating | filter-1-resident-s8 | 1.136 | 0.838 | 0.823–0.880 | 5 | pass | pass |
| allocating | view-1-resident-s8 | 1.106 | 0.948 | 0.879–0.987 | 5 | pass | pass |
| allocating | filter-2-resident-s8 | 1.019 | 1.034 | 1.017–1.035 | 5 | pass | pass |
| allocating | view-2-resident-s8 | 1.159 | 1.072 | 1.039–1.135 | 5 | pass | pass |
| allocating | filter-3-resident-s12 | 0.744 | 0.911 | 0.897–0.922 | 5 | pass | pass |
| allocating | view-3-resident-s12 | 1.088 | 1.033 | 0.975–1.060 | 5 | pass | pass |
| allocating | vertex-v0-streaming-s4 | 0.941 | 0.986 | 0.948–1.046 | 5 | pass | pass |
| allocating | view-none-streaming-s4 | 0.947 | 0.948 | 0.922–1.033 | 5 | pass | pass |
| allocating | vertex-v0-streaming-s12 | 1.017 | 0.967 | 0.904–1.075 | 5 | pass | pass |
| allocating | view-none-streaming-s12 | 0.971 | 0.772 | 0.577–1.191 | 5 | pass | pass |
| allocating | vertex-v0-streaming-s32 | 1.110 | 0.961 | 0.922–1.230 | 5 | pass | pass |
| allocating | view-none-streaming-s32 | 1.023 | 0.968 | 0.943–1.071 | 5 | pass | pass |
| allocating | filter-1-streaming-s4 | 1.013 | 0.827 | 0.602–1.087 | 5 | pass | pass |
| allocating | view-1-streaming-s4 | 2.090 | 1.591 | 1.504–2.003 | 11 | FAIL | FAIL |
| allocating | filter-1-streaming-s8 | 0.993 | 1.089 | 0.915–1.235 | 5 | pass | pass |
| allocating | view-1-streaming-s8 | 1.036 | 0.873 | 0.755–0.946 | 5 | pass | pass |
| allocating | filter-2-streaming-s8 | 1.101 | 0.995 | 0.760–1.235 | 5 | pass | pass |
| allocating | view-2-streaming-s8 | 1.098 | 0.917 | 0.827–1.025 | 5 | pass | pass |
| allocating | filter-3-streaming-s12 | 0.984 | 0.942 | 0.698–1.157 | 5 | pass | pass |
| allocating | view-3-streaming-s12 | 1.042 | 1.023 | 0.682–1.382 | 5 | pass | pass |
| allocating | vertex-v1-tiny-s4 | 1.266 | 1.280 | 1.213–1.282 | 20+30 | pass | pass |
| allocating | vertex-v1-tiny-s12 | 1.215 | 1.229 | 1.240–1.344 | 20+30 | pass | FAIL |
| allocating | vertex-v1-tiny-s32 | 1.097 | 1.315 | 1.222–1.308 | 20+30 | pass | FAIL |
| allocating | vertex-v1-resident-s4 | 1.078 | 1.060 | 1.047–1.079 | 5 | pass | pass |
| allocating | vertex-v1-resident-s12 | 1.138 | 1.126 | 1.061–1.225 | 5 | pass | pass |
| allocating | vertex-v1-resident-s32 | 0.942 | 1.169 | 0.956–1.246 | 5 | pass | pass |
| allocating | vertex-v1-streaming-s4 | 0.916 | 2.065 | 1.506–2.410 | 12 | FAIL | FAIL |
| allocating | vertex-v1-streaming-s12 | 0.898 | 0.939 | 0.640–1.292 | 5 | pass | pass |
| allocating | vertex-v1-streaming-s32 | 0.963 | 0.953 | 0.865–1.137 | 5 | pass | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.152 | 1.098 | 1.036–1.183 | 5 | pass | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.165 | 1.116 | 0.844–1.275 | 6 | pass | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.074 | 1.094 | 1.063–1.186 | 5 | pass | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.109 | 1.097 | 1.012–1.287 | 5 | pass | pass |
| allocating | varied-filter-1-tiny-s4 | 1.667 | 1.476 | 1.426–1.467 | 20+30 | pass | pass |
| allocating | varied-view-1-tiny-s4 | 1.349 | 1.459 | 1.398–1.484 | 20+30 | pass | pass |
| allocating | varied-filter-1-tiny-s8 | 1.224 | 1.203 | 1.137–1.353 | 5 | pass | pass |
| allocating | varied-view-1-tiny-s8 | 1.181 | 1.219 | 1.139–1.340 | 5 | pass | pass |
| allocating | varied-filter-2-tiny-s8 | 1.378 | 1.363 | 1.238–1.418 | 5 | pass | pass |
| allocating | varied-view-2-tiny-s8 | 1.209 | 1.318 | 1.299–1.387 | 5 | pass | pass |
| allocating | varied-filter-3-tiny-s4 | 0.903 | 0.897 | 0.852–0.973 | 5 | pass | pass |
| allocating | varied-view-3-tiny-s4 | 1.142 | 1.245 | 1.121–1.352 | 5 | pass | pass |
| allocating | varied-filter-3-tiny-s12 | 0.977 | 0.971 | 0.922–0.988 | 5 | pass | pass |
| allocating | varied-view-3-tiny-s12 | 1.104 | 1.269 | 1.181–1.321 | 5 | pass | pass |
| allocating | varied-filter-3-tiny-s32 | 1.054 | 0.989 | 0.954–1.093 | 5 | pass | pass |
| allocating | varied-view-3-tiny-s32 | 1.122 | 1.183 | 0.709–1.462 | 5 | pass | pass |
| allocating | varied-filter-1-resident-s4 | 1.999 | 1.431 | 1.298–1.486 | 6 | pass | pass |
| allocating | varied-view-1-resident-s4 | 1.531 | 1.278 | 1.204–1.487 | 9 | pass | pass |
| allocating | varied-filter-1-resident-s8 | 1.303 | 1.070 | 0.924–1.173 | 5 | pass | pass |
| allocating | varied-view-1-resident-s8 | 1.274 | 1.124 | 0.975–1.404 | 6 | pass | pass |
| allocating | varied-filter-2-resident-s8 | 1.228 | 1.247 | 0.995–1.498 | 8 | pass | pass |
| allocating | varied-view-2-resident-s8 | 1.253 | 1.138 | 0.952–1.495 | 10 | pass | pass |
| allocating | varied-filter-3-resident-s4 | 0.894 | 0.818 | 0.742–0.906 | 5 | pass | pass |
| allocating | varied-view-3-resident-s4 | 1.045 | 1.027 | 0.616–1.215 | 5 | pass | pass |
| allocating | varied-filter-3-resident-s12 | 0.830 | 0.814 | 0.787–0.847 | 5 | pass | pass |
| allocating | varied-view-3-resident-s12 | 1.075 | 1.088 | 1.041–1.167 | 5 | pass | pass |
| allocating | varied-filter-3-resident-s32 | 0.850 | 0.882 | 0.815–0.960 | 5 | pass | pass |
| allocating | varied-view-3-resident-s32 | 0.952 | 1.062 | 0.997–1.166 | 5 | pass | pass |
| allocating | varied-filter-1-streaming-s4 | 1.771 | 1.486 | 1.443–1.487 | 20+30 | pass | pass |
| allocating | varied-view-1-streaming-s4 | 1.506 | 1.313 | 1.245–1.344 | 5 | pass | pass |
| allocating | varied-filter-1-streaming-s8 | 1.395 | 1.150 | 1.137–1.177 | 5 | pass | pass |
| allocating | varied-view-1-streaming-s8 | 1.243 | 1.132 | 1.097–1.172 | 5 | pass | pass |
| allocating | varied-filter-2-streaming-s8 | 1.362 | 1.197 | 1.188–1.321 | 5 | pass | pass |
| allocating | varied-view-2-streaming-s8 | 1.224 | 1.133 | 1.091–1.184 | 5 | pass | pass |
| allocating | varied-filter-3-streaming-s4 | 0.937 | 0.951 | 0.899–0.960 | 5 | pass | pass |
| allocating | varied-view-3-streaming-s4 | 1.010 | 1.010 | 0.983–1.035 | 5 | pass | pass |
| allocating | varied-filter-3-streaming-s12 | 0.918 | 0.930 | 0.907–0.974 | 5 | pass | pass |
| allocating | varied-view-3-streaming-s12 | 1.027 | 1.056 | 0.703–1.301 | 5 | pass | pass |
| allocating | varied-filter-3-streaming-s32 | 3.252 | 0.912 | 0.467–1.374 | 5 | pass | pass |
| allocating | varied-view-3-streaming-s32 | 0.978 | 2.060 | 1.982–2.093 | 5 | FAIL | FAIL |
| allocating | meshlet-1-1-v2-t3 | 1.425 | 1.365 | 1.310–1.452 | 6 | pass | FAIL |
| allocating | meshlet-1-1-v2-t4 | 1.310 | 1.383 | 1.300–1.442 | 18 | pass | FAIL |
| allocating | meshlet-1-1-v4-t3 | 1.296 | 1.367 | 1.341–1.400 | 5 | pass | FAIL |
| allocating | meshlet-1-1-v4-t4 | 1.273 | 1.363 | 1.306–1.380 | 8 | pass | FAIL |
| allocating | meshlet-raw-1-1 | 0.835 | 1.141 | 0.925–1.280 | 5 | pass | pass |
| allocating | meshlet-64-126-v2-t3 | 1.224 | 1.232 | 1.223–1.291 | 7 | pass | pass |
| allocating | meshlet-64-126-v2-t4 | 1.198 | 1.319 | 1.290–1.367 | 20+30 | pass | FAIL |
| allocating | meshlet-64-126-v4-t3 | 1.197 | 0.955 | 0.899–1.018 | 5 | pass | pass |
| allocating | meshlet-64-126-v4-t4 | 1.166 | 1.283 | 1.181–1.325 | 20+30 | pass | FAIL |
| allocating | meshlet-raw-64-126 | 1.128 | 1.087 | 1.056–1.283 | 7 | pass | pass |
| allocating | meshlet-256-256-v2-t3 | 1.199 | 1.235 | 1.224–1.298 | 20 | pass | pass |
| allocating | meshlet-256-256-v2-t4 | 1.104 | 1.142 | 1.120–1.298 | 8 | pass | pass |
| allocating | meshlet-256-256-v4-t3 | 1.125 | 1.020 | 0.935–1.117 | 5 | pass | pass |
| allocating | meshlet-256-256-v4-t4 | 1.075 | 1.090 | 1.052–1.115 | 5 | pass | pass |
| allocating | meshlet-raw-256-256 | 1.052 | 1.037 | 0.957–1.165 | 5 | pass | pass |
| caller-buffer | vertex-v0-tiny-s4 | 1.222 | 1.199 | 1.148–1.250 | 5 | pass | pass |
| caller-buffer | view-none-tiny-s4 | 1.240 | 1.247 | 1.195–1.308 | 20+30 | pass | FAIL |
| caller-buffer | vertex-v0-tiny-s12 | 1.151 | 1.195 | 1.036–1.280 | 5 | pass | pass |
| caller-buffer | view-none-tiny-s12 | 1.205 | 1.136 | 1.094–1.175 | 5 | pass | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.031 | 1.153 | 1.101–1.197 | 5 | pass | pass |
| caller-buffer | view-none-tiny-s32 | 1.030 | 1.122 | 1.079–1.268 | 5 | pass | pass |
| caller-buffer | filter-1-tiny-s4 | 1.286 | 1.237 | 1.195–1.469 | 7 | pass | pass |
| caller-buffer | view-1-tiny-s4 | 1.197 | 1.181 | 1.140–1.302 | 5 | pass | pass |
| caller-buffer | filter-1-tiny-s8 | 0.974 | 0.981 | 0.899–1.090 | 5 | pass | pass |
| caller-buffer | view-1-tiny-s8 | 1.121 | 1.139 | 1.067–1.206 | 5 | pass | pass |
| caller-buffer | filter-2-tiny-s8 | 1.188 | 1.147 | 1.097–1.365 | 5 | pass | pass |
| caller-buffer | view-2-tiny-s8 | 1.213 | 1.400 | 1.220–1.446 | 5 | pass | pass |
| caller-buffer | filter-3-tiny-s12 | 0.911 | 0.872 | 0.804–0.904 | 5 | pass | pass |
| caller-buffer | view-3-tiny-s12 | 1.094 | 1.040 | 0.951–1.227 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.060 | 1.147 | 0.965–1.278 | 5 | pass | pass |
| caller-buffer | view-none-resident-s4 | 1.056 | 1.076 | 0.978–1.295 | 7 | pass | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.128 | 1.039 | 0.962–1.215 | 5 | pass | pass |
| caller-buffer | view-none-resident-s12 | 1.096 | 1.117 | 1.053–1.298 | 9 | pass | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.094 | 1.133 | 1.116–1.139 | 5 | pass | pass |
| caller-buffer | view-none-resident-s32 | 1.140 | 1.084 | 1.065–1.139 | 5 | pass | pass |
| caller-buffer | filter-1-resident-s4 | 0.978 | 1.012 | 0.889–1.148 | 5 | pass | pass |
| caller-buffer | view-1-resident-s4 | 0.965 | 0.979 | 0.914–1.015 | 5 | pass | pass |
| caller-buffer | filter-1-resident-s8 | 0.886 | 1.127 | 0.838–1.331 | 5 | pass | pass |
| caller-buffer | view-1-resident-s8 | 0.985 | 0.992 | 0.980–0.994 | 5 | pass | pass |
| caller-buffer | filter-2-resident-s8 | 0.793 | 1.031 | 0.954–1.188 | 5 | pass | pass |
| caller-buffer | view-2-resident-s8 | 1.114 | 1.083 | 1.050–1.119 | 5 | pass | pass |
| caller-buffer | filter-3-resident-s12 | 0.807 | 0.961 | 0.906–0.984 | 5 | pass | pass |
| caller-buffer | view-3-resident-s12 | 1.056 | 1.108 | 1.032–1.135 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s4 | 0.947 | 0.968 | 0.982–1.096 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s4 | 0.906 | 0.962 | 0.909–1.096 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s12 | 0.821 | 0.842 | 0.726–0.924 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s12 | 0.764 | 0.902 | 0.871–0.940 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s32 | 0.952 | 1.029 | 0.978–1.078 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s32 | 0.886 | 1.050 | 0.954–1.098 | 5 | pass | pass |
| caller-buffer | filter-1-streaming-s4 | 1.032 | 0.999 | 0.950–1.063 | 5 | pass | pass |
| caller-buffer | view-1-streaming-s4 | 0.790 | 0.941 | 0.929–0.954 | 5 | pass | pass |
| caller-buffer | filter-1-streaming-s8 | 1.290 | 0.940 | 0.826–1.136 | 5 | pass | pass |
| caller-buffer | view-1-streaming-s8 | 0.953 | 0.967 | 0.812–1.360 | 5 | pass | pass |
| caller-buffer | filter-2-streaming-s8 | 1.385 | 1.082 | 0.979–1.272 | 5 | pass | pass |
| caller-buffer | view-2-streaming-s8 | 0.573 | 1.075 | 1.057–1.086 | 5 | pass | pass |
| caller-buffer | filter-3-streaming-s12 | 0.955 | 0.957 | 0.880–1.094 | 5 | pass | pass |
| caller-buffer | view-3-streaming-s12 | 0.756 | 0.740 | 0.654–0.805 | 5 | pass | pass |
| caller-buffer | vertex-v1-tiny-s4 | 1.240 | 1.256 | 1.232–1.294 | 5 | pass | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.273 | 1.206 | 1.200–1.238 | 5 | pass | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.015 | 1.199 | 1.188–1.254 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.041 | 1.008 | 0.983–1.044 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.177 | 1.118 | 1.086–1.177 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.126 | 1.156 | 1.031–1.205 | 5 | pass | pass |
| caller-buffer | vertex-v1-streaming-s4 | 0.934 | 1.047 | 0.939–1.244 | 6 | pass | pass |
| caller-buffer | vertex-v1-streaming-s12 | 0.830 | 0.860 | 0.677–0.958 | 5 | pass | pass |
| caller-buffer | vertex-v1-streaming-s32 | 0.976 | 0.996 | 0.970–1.088 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.097 | 1.127 | 1.100–1.217 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.175 | 1.123 | 1.056–1.195 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.079 | 1.116 | 1.073–1.207 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 0.996 | 1.115 | 1.036–1.193 | 5 | pass | pass |
| caller-buffer | varied-filter-1-tiny-s4 | 1.716 | 1.609 | 1.523–1.724 | 5 | FAIL | FAIL |
| caller-buffer | varied-view-1-tiny-s4 | 1.478 | 1.326 | 1.210–1.463 | 5 | pass | pass |
| caller-buffer | varied-filter-1-tiny-s8 | 1.307 | 1.391 | 1.335–1.429 | 5 | pass | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.339 | 1.297 | 1.249–1.351 | 5 | pass | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 1.334 | 1.639 | 1.519–1.719 | 10 | FAIL | FAIL |
| caller-buffer | varied-view-2-tiny-s8 | 1.251 | 1.325 | 1.283–1.446 | 5 | pass | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 0.887 | 0.918 | 0.706–1.121 | 5 | pass | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.105 | 1.070 | 0.939–1.147 | 5 | pass | pass |
| caller-buffer | varied-filter-3-tiny-s12 | 0.967 | 2.150 | 1.677–2.109 | 5 | FAIL | FAIL |
| caller-buffer | varied-view-3-tiny-s12 | 1.552 | 1.219 | 1.169–1.234 | 5 | pass | pass |
| caller-buffer | varied-filter-3-tiny-s32 | 1.051 | 1.865 | 1.712–2.148 | 5 | FAIL | FAIL |
| caller-buffer | varied-view-3-tiny-s32 | 1.262 | 1.206 | 1.117–1.273 | 5 | pass | pass |
| caller-buffer | varied-filter-1-resident-s4 | 1.778 | 1.457 | 1.425–1.499 | 12 | pass | pass |
| caller-buffer | varied-view-1-resident-s4 | 1.533 | 1.264 | 1.188–1.435 | 5 | pass | pass |
| caller-buffer | varied-filter-1-resident-s8 | 1.433 | 1.134 | 1.083–1.212 | 5 | pass | pass |
| caller-buffer | varied-view-1-resident-s8 | 1.276 | 1.121 | 1.115–1.128 | 5 | pass | pass |
| caller-buffer | varied-filter-2-resident-s8 | 1.359 | 1.264 | 1.251–1.273 | 5 | pass | pass |
| caller-buffer | varied-view-2-resident-s8 | 1.266 | 1.201 | 1.171–1.271 | 5 | pass | pass |
| caller-buffer | varied-filter-3-resident-s4 | 0.937 | 0.957 | 0.934–0.965 | 5 | pass | pass |
| caller-buffer | varied-view-3-resident-s4 | 1.006 | 1.028 | 0.933–1.063 | 5 | pass | pass |
| caller-buffer | varied-filter-3-resident-s12 | 0.834 | 0.932 | 0.911–0.955 | 5 | pass | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.109 | 1.038 | 1.034–1.082 | 5 | pass | pass |
| caller-buffer | varied-filter-3-resident-s32 | 0.911 | 0.983 | 0.939–1.015 | 5 | pass | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.102 | 0.972 | 0.994–1.104 | 5 | pass | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 1.783 | 1.465 | 1.420–1.480 | 5 | pass | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1.462 | 1.337 | 1.235–1.320 | 5 | pass | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 1.378 | 1.179 | 1.127–1.204 | 5 | pass | pass |
| caller-buffer | varied-view-1-streaming-s8 | 1.231 | 1.116 | 1.058–1.145 | 5 | pass | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 1.254 | 1.342 | 1.331–1.342 | 5 | pass | pass |
| caller-buffer | varied-view-2-streaming-s8 | 0.968 | 1.242 | 1.197–1.244 | 5 | pass | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 0.946 | 0.940 | 0.917–0.977 | 5 | pass | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1.248 | 0.978 | 0.963–0.989 | 5 | pass | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 0.778 | 0.924 | 0.921–0.933 | 5 | pass | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.041 | 1.052 | 1.030–1.112 | 5 | pass | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 0.978 | 0.950 | 0.892–1.060 | 5 | pass | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.039 | 1.047 | 1.008–1.066 | 5 | pass | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 1.845 | 2.017 | 1.982–2.038 | 5 | FAIL | FAIL |
| caller-buffer | meshlet-1-1-v2-t4 | 1.902 | 1.859 | 1.512–2.772 | 5 | FAIL | FAIL |
| caller-buffer | meshlet-1-1-v4-t3 | 1.716 | 2.002 | 1.726–2.082 | 5 | FAIL | FAIL |
| caller-buffer | meshlet-1-1-v4-t4 | 1.737 | 1.509 | 1.522–2.036 | 8 | FAIL | FAIL |
| caller-buffer | meshlet-raw-1-1 | 1.609 | 1.461 | 1.446–1.494 | 7 | pass | FAIL |
| caller-buffer | meshlet-64-126-v2-t3 | 1.409 | 1.222 | 1.196–1.294 | 8 | pass | pass |
| caller-buffer | meshlet-64-126-v2-t4 | 1.306 | 1.349 | 1.314–1.402 | 20+30 | pass | FAIL |
| caller-buffer | meshlet-64-126-v4-t3 | 1.307 | 1.158 | 1.146–1.296 | 11 | pass | pass |
| caller-buffer | meshlet-64-126-v4-t4 | 1.346 | 1.362 | 1.312–1.497 | 9 | pass | FAIL |
| caller-buffer | meshlet-raw-64-126 | 1.312 | 1.315 | 1.302–1.372 | 5 | pass | FAIL |
| caller-buffer | meshlet-256-256-v2-t3 | 1.281 | 1.200 | 1.243–1.343 | 20+30 | pass | FAIL |
| caller-buffer | meshlet-256-256-v2-t4 | 1.160 | 1.202 | 1.183–1.212 | 5 | pass | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 1.035 | 1.221 | 1.203–1.255 | 5 | pass | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 1.263 | 1.181 | 1.174–1.207 | 5 | pass | pass |
| caller-buffer | meshlet-raw-256-256 | 1.131 | 1.160 | 1.147–1.176 | 5 | pass | pass |

## Every Node row

| API | Case | Before | After | Final 95% interval | Pairs | RFC max | Registered max |
|---|---|---:|---:|---|---:|---|---|
| allocating | vertex-v0-tiny-s4 | 1.082 | 1.535 | 1.009–1.455 | 20+30 | pass | pass |
| allocating | view-none-tiny-s4 | 1.066 | 1.418 | 0.904–1.161 | 20+30 | pass | pass |
| allocating | vertex-v0-tiny-s12 | 1.193 | 1.179 | 1.064–1.476 | 13 | pass | pass |
| allocating | view-none-tiny-s12 | 1.238 | 1.176 | 1.007–1.290 | 5 | pass | pass |
| allocating | vertex-v0-tiny-s32 | 1.141 | 1.164 | 1.025–1.366 | 5 | pass | pass |
| allocating | view-none-tiny-s32 | 1.176 | 1.169 | 0.968–1.453 | 6 | pass | pass |
| allocating | view-1-tiny-s4 | 1.075 | 0.841 | 0.729–0.954 | 5 | pass | pass |
| allocating | view-1-tiny-s8 | 1.345 | 1.119 | 1.070–1.492 | 19 | pass | pass |
| allocating | view-2-tiny-s8 | 0.514 | 1.000 | 0.741–1.253 | 5 | pass | pass |
| allocating | view-3-tiny-s12 | 1.228 | 1.158 | 0.722–1.487 | 8 | pass | pass |
| allocating | index-3-v0-tiny-s2 | 0.697 | 0.561 | 0.389–1.302 | 5 | pass | pass |
| allocating | index-3-v0-tiny-s4 | 1.058 | 1.113 | 0.658–1.445 | 14 | pass | pass |
| allocating | index-3-v1-tiny-s2 | 0.999 | 0.980 | 0.925–1.226 | 5 | pass | pass |
| allocating | index-3-v1-tiny-s4 | 1.067 | 0.978 | 0.740–1.158 | 5 | pass | pass |
| allocating | vertex-v0-resident-s4 | 1.004 | 1.237 | 0.895–1.394 | 5 | pass | pass |
| allocating | view-none-resident-s4 | 1.158 | 1.010 | 0.970–1.100 | 5 | pass | pass |
| allocating | vertex-v0-resident-s12 | 1.064 | 1.069 | 0.969–1.294 | 5 | pass | pass |
| allocating | view-none-resident-s12 | 1.162 | 1.211 | 0.952–1.399 | 5 | pass | pass |
| allocating | vertex-v0-resident-s32 | 1.165 | 1.120 | 0.923–1.281 | 5 | pass | pass |
| allocating | view-none-resident-s32 | 1.103 | 1.015 | 0.984–1.187 | 5 | pass | pass |
| allocating | view-1-resident-s4 | 1.539 | 1.777 | 1.615–1.932 | 12 | FAIL | FAIL |
| allocating | view-1-resident-s8 | 1.887 | 1.853 | 1.760–2.128 | 5 | FAIL | FAIL |
| allocating | view-2-resident-s8 | 1.726 | 1.536 | 1.559–1.831 | 20+30 | FAIL | FAIL |
| allocating | view-3-resident-s12 | 1.502 | 1.663 | 1.203–1.513 | 20+30 | FAIL | pass |
| allocating | index-3-v0-resident-s2 | 0.967 | 0.958 | 0.836–1.264 | 5 | pass | pass |
| allocating | index-3-v0-resident-s4 | 0.955 | 0.946 | 0.654–1.447 | 6 | pass | pass |
| allocating | index-3-v1-resident-s2 | 0.987 | 1.075 | 0.963–1.366 | 5 | pass | pass |
| allocating | index-3-v1-resident-s4 | 0.996 | 0.881 | 0.646–1.105 | 5 | pass | pass |
| allocating | vertex-v0-streaming-s4 | 1.045 | 1.212 | 0.837–1.482 | 17 | pass | pass |
| allocating | view-none-streaming-s4 | 1.008 | 0.836 | 0.699–1.489 | 9 | pass | pass |
| allocating | vertex-v0-streaming-s12 | 1.120 | 1.103 | 0.886–1.473 | 7 | pass | pass |
| allocating | view-none-streaming-s12 | 1.732 | 0.929 | 0.759–1.387 | 5 | pass | pass |
| allocating | vertex-v0-streaming-s32 | 1.088 | 1.201 | 1.066–1.275 | 5 | pass | pass |
| allocating | view-none-streaming-s32 | 1.028 | 1.175 | 1.022–1.327 | 5 | pass | pass |
| allocating | view-1-streaming-s4 | 1.526 | 1.883 | 1.622–2.004 | 12 | FAIL | FAIL |
| allocating | view-1-streaming-s8 | 1.810 | 1.986 | 1.866–2.154 | 5 | FAIL | FAIL |
| allocating | view-2-streaming-s8 | 1.704 | 1.843 | 1.681–1.986 | 5 | FAIL | FAIL |
| allocating | view-3-streaming-s12 | 1.084 | 1.235 | 1.073–1.479 | 12 | pass | pass |
| allocating | index-3-v0-streaming-s2 | 0.697 | 0.971 | 0.809–1.128 | 5 | pass | pass |
| allocating | index-3-v0-streaming-s4 | 0.936 | 0.906 | 0.620–1.469 | 6 | pass | pass |
| allocating | index-3-v1-streaming-s2 | 0.890 | 0.970 | 0.948–1.134 | 5 | pass | pass |
| allocating | index-3-v1-streaming-s4 | 1.042 | 0.635 | 0.480–1.398 | 5 | pass | pass |
| allocating | vertex-v1-tiny-s4 | 1.170 | 1.086 | 0.628–1.353 | 5 | pass | pass |
| allocating | vertex-v1-tiny-s12 | 1.237 | 1.241 | 1.087–1.318 | 5 | pass | pass |
| allocating | vertex-v1-tiny-s32 | 1.291 | 1.302 | 1.134–1.365 | 5 | pass | pass |
| allocating | vertex-v1-resident-s4 | 1.044 | 1.370 | 1.123–1.481 | 7 | pass | pass |
| allocating | vertex-v1-resident-s12 | 1.167 | 1.301 | 1.169–1.487 | 6 | pass | pass |
| allocating | vertex-v1-resident-s32 | 1.169 | 1.375 | 1.225–1.497 | 12 | pass | pass |
| allocating | vertex-v1-streaming-s4 | 1.101 | 1.286 | 0.979–1.477 | 20 | pass | pass |
| allocating | vertex-v1-streaming-s12 | 1.126 | 1.475 | 1.146–1.469 | 9 | pass | pass |
| allocating | vertex-v1-streaming-s32 | 1.085 | 1.248 | 1.108–1.448 | 6 | pass | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.163 | 1.145 | 0.663–1.414 | 6 | pass | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.171 | 1.134 | 1.038–1.263 | 5 | pass | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.151 | 1.179 | 1.020–1.377 | 5 | pass | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.175 | 1.194 | 0.868–1.462 | 9 | pass | pass |
| allocating | varied-view-1-tiny-s4 | 1.341 | 1.278 | 1.062–1.457 | 8 | pass | pass |
| allocating | varied-view-1-tiny-s8 | 1.375 | 1.179 | 1.165–1.385 | 5 | pass | pass |
| allocating | varied-view-2-tiny-s8 | 1.315 | 1.212 | 1.096–1.474 | 11 | pass | pass |
| allocating | varied-view-3-tiny-s4 | 1.198 | 1.147 | 1.056–1.255 | 5 | pass | pass |
| allocating | varied-view-3-tiny-s12 | 1.200 | 1.275 | 1.098–1.348 | 5 | pass | pass |
| allocating | varied-view-3-tiny-s32 | 1.238 | 1.118 | 1.026–1.459 | 8 | pass | pass |
| allocating | varied-view-1-resident-s4 | 1.614 | 1.600 | 1.529–1.651 | 20+30 | FAIL | FAIL |
| allocating | varied-view-1-resident-s8 | 1.502 | 1.524 | 1.425–1.560 | 20+30 | FAIL | pass |
| allocating | varied-view-2-resident-s8 | 1.453 | 1.347 | 1.260–1.435 | 5 | pass | pass |
| allocating | varied-view-3-resident-s4 | 1.134 | 1.146 | 0.935–1.405 | 6 | pass | pass |
| allocating | varied-view-3-resident-s12 | 1.290 | 1.470 | 1.314–1.479 | 5 | pass | pass |
| allocating | varied-view-3-resident-s32 | 1.188 | 1.294 | 1.206–1.380 | 5 | pass | pass |
| allocating | varied-view-1-streaming-s4 | 1.615 | 1.567 | 1.538–1.616 | 20+30 | FAIL | FAIL |
| allocating | varied-view-1-streaming-s8 | 1.504 | 1.511 | 1.239–1.664 | 20+30 | FAIL | FAIL |
| allocating | varied-view-2-streaming-s8 | 1.570 | 1.545 | 1.169–1.588 | 20+30 | FAIL | pass |
| allocating | varied-view-3-streaming-s4 | 1.085 | 1.105 | 1.060–1.121 | 5 | pass | pass |
| allocating | varied-view-3-streaming-s12 | 1.288 | 1.430 | 1.093–1.476 | 20+30 | pass | pass |
| allocating | varied-view-3-streaming-s32 | 1.194 | 1.290 | 1.043–1.475 | 5 | pass | pass |
| caller-buffer | vertex-v0-tiny-s4 | 1.125 | 0.870 | 0.435–1.499 | 7 | pass | pass |
| caller-buffer | view-none-tiny-s4 | 1.002 | 0.906 | 0.799–1.108 | 5 | pass | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.024 | 1.010 | 0.911–1.190 | 5 | pass | pass |
| caller-buffer | view-none-tiny-s12 | 1.039 | 1.140 | 0.971–1.253 | 5 | pass | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.261 | 1.160 | 1.095–1.208 | 5 | pass | pass |
| caller-buffer | view-none-tiny-s32 | 1.103 | 1.035 | 0.957–1.162 | 5 | pass | pass |
| caller-buffer | view-1-tiny-s4 | 1.229 | 1.060 | 0.887–1.173 | 5 | pass | pass |
| caller-buffer | view-1-tiny-s8 | 1.338 | 1.205 | 1.164–1.308 | 5 | pass | pass |
| caller-buffer | view-2-tiny-s8 | 1.271 | 1.190 | 1.131–1.221 | 5 | pass | pass |
| caller-buffer | view-3-tiny-s12 | 0.898 | 1.139 | 1.111–1.176 | 5 | pass | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.812 | 0.755 | 0.726–0.922 | 5 | pass | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.800 | 0.816 | 0.581–0.886 | 5 | pass | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.822 | 0.818 | 0.745–0.854 | 5 | pass | pass |
| caller-buffer | index-3-v1-tiny-s4 | 0.769 | 0.829 | 0.808–0.877 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.086 | 1.080 | 0.926–1.271 | 5 | pass | pass |
| caller-buffer | view-none-resident-s4 | 1.026 | 1.185 | 0.985–1.320 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.256 | 1.284 | 1.001–1.463 | 6 | pass | pass |
| caller-buffer | view-none-resident-s12 | 1.175 | 1.314 | 1.172–1.376 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.209 | 1.255 | 1.161–1.346 | 5 | pass | pass |
| caller-buffer | view-none-resident-s32 | 1.147 | 1.384 | 1.326–1.416 | 5 | pass | pass |
| caller-buffer | view-1-resident-s4 | 1.593 | 1.753 | 1.611–1.861 | 9 | FAIL | FAIL |
| caller-buffer | view-1-resident-s8 | 1.919 | 2.074 | 1.707–2.255 | 5 | FAIL | FAIL |
| caller-buffer | view-2-resident-s8 | 1.855 | 1.640 | 1.616–1.807 | 6 | FAIL | FAIL |
| caller-buffer | view-3-resident-s12 | 1.583 | 1.545 | 1.507–1.585 | 7 | FAIL | pass |
| caller-buffer | index-3-v0-resident-s2 | 0.816 | 0.850 | 0.840–0.884 | 5 | pass | pass |
| caller-buffer | index-3-v0-resident-s4 | 0.887 | 0.862 | 0.809–0.901 | 5 | pass | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.822 | 0.876 | 0.757–0.986 | 5 | pass | pass |
| caller-buffer | index-3-v1-resident-s4 | 0.751 | 0.832 | 0.796–0.868 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s4 | 1.141 | 0.960 | 0.934–1.058 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s4 | 1.067 | 1.278 | 1.005–1.353 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.221 | 1.169 | 1.116–1.211 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s12 | 1.165 | 1.180 | 1.105–1.234 | 5 | pass | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1.157 | 1.106 | 1.025–1.186 | 5 | pass | pass |
| caller-buffer | view-none-streaming-s32 | 1.105 | 1.169 | 1.121–1.260 | 5 | pass | pass |
| caller-buffer | view-1-streaming-s4 | 1.555 | 1.543 | 1.501–1.579 | 15 | FAIL | pass |
| caller-buffer | view-1-streaming-s8 | 1.781 | 1.852 | 1.692–1.998 | 5 | FAIL | FAIL |
| caller-buffer | view-2-streaming-s8 | 1.654 | 1.755 | 1.620–1.897 | 6 | FAIL | FAIL |
| caller-buffer | view-3-streaming-s12 | 1.417 | 1.497 | 1.485–1.546 | 20+30 | FAIL | pass |
| caller-buffer | index-3-v0-streaming-s2 | 0.838 | 0.868 | 0.839–0.875 | 5 | pass | pass |
| caller-buffer | index-3-v0-streaming-s4 | 0.865 | 0.813 | 0.569–1.041 | 5 | pass | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.811 | 0.867 | 0.785–0.911 | 5 | pass | pass |
| caller-buffer | index-3-v1-streaming-s4 | 0.862 | 0.889 | 0.818–0.901 | 5 | pass | pass |
| caller-buffer | vertex-v1-tiny-s4 | 0.916 | 0.846 | 0.299–1.425 | 6 | pass | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.210 | 1.033 | 1.023–1.047 | 5 | pass | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.328 | 1.181 | 1.138–1.320 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.103 | 1.048 | 1.044–1.082 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.197 | 1.107 | 1.058–1.183 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.227 | 1.097 | 1.000–1.386 | 5 | pass | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1.094 | 1.074 | 1.004–1.107 | 5 | pass | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.246 | 1.116 | 1.068–1.197 | 5 | pass | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1.110 | 1.047 | 0.892–1.283 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.207 | 1.205 | 1.128–1.277 | 5 | pass | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.361 | 1.136 | 1.083–1.205 | 5 | pass | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.371 | 1.154 | 0.753–1.429 | 6 | pass | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.329 | 1.137 | 1.087–1.184 | 5 | pass | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.198 | 1.082 | 0.913–1.154 | 5 | pass | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.261 | 1.112 | 0.992–1.181 | 5 | pass | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.142 | 1.283 | 0.988–1.436 | 6 | pass | pass |
| caller-buffer | varied-view-3-tiny-s4 | 0.874 | 0.892 | 0.642–1.207 | 5 | pass | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.225 | 1.088 | 1.015–1.288 | 5 | pass | pass |
| caller-buffer | varied-view-3-tiny-s32 | 0.986 | 0.910 | 0.792–1.246 | 5 | pass | pass |
| caller-buffer | varied-view-1-resident-s4 | 1.662 | 1.579 | 1.575–1.685 | 20+30 | FAIL | FAIL |
| caller-buffer | varied-view-1-resident-s8 | 1.509 | 1.594 | 1.490–1.581 | 20+30 | FAIL | pass |
| caller-buffer | varied-view-2-resident-s8 | 1.527 | 1.541 | 1.438–1.543 | 20+30 | FAIL | pass |
| caller-buffer | varied-view-3-resident-s4 | 1.061 | 1.137 | 1.047–1.204 | 5 | pass | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.245 | 1.234 | 1.192–1.337 | 5 | pass | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.273 | 1.327 | 1.275–1.346 | 5 | pass | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1.648 | 1.679 | 1.592–1.676 | 20+30 | FAIL | FAIL |
| caller-buffer | varied-view-1-streaming-s8 | 1.495 | 1.577 | 1.543–1.601 | 20+30 | FAIL | FAIL |
| caller-buffer | varied-view-2-streaming-s8 | 1.543 | 1.534 | 1.498–1.540 | 20+30 | FAIL | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1.107 | 1.122 | 0.790–1.428 | 6 | pass | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.317 | 1.360 | 1.302–1.488 | 5 | pass | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.359 | 1.383 | 1.269–1.499 | 6 | pass | pass |
