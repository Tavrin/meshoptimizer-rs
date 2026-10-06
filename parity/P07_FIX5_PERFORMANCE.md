# P07 fix-five scoped final performance — 2026-10-05

Rust/C++ time ratios; lower is faster. Family means use stage-one median
ratios. A case passes only if its final paired 95% Student-t log-ratio
interval is wholly below its registered maximum. Borderline-only fresh
D146 supersedes stage one for maximum/S3 verdicts. Unresolved intervals fail.

Before: fix-four where measured; standalone Oct/Quat/Exp uses the explicitly
historical fix-three epoch. Sequence also interleaves fix-four Rust in the
same final pass. Family before/after values describe different shared-host
epochs. No changed bar, frozen input, maximum retry or full matrix.
Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07-fix5`; summary.json and
final-evidence.json retain verdicts and identities.

## Family results

A = allocating; C = caller-buffer.

| Family | A before → after | C before → after | Registered A / C |
|---|---:|---:|---|
| vertex | 1.201 → 1.090 | 1.103 → 1.055 | **FAIL** / pass |
| view-none | 1.155 → 1.122 | 1.183 → 1.025 | **FAIL** / pass |
| view-filtered | 1.295 → 1.189 | 1.185 → 1.113 | **FAIL** / **FAIL** |
| oct (fix-three before) | 1.231 → 1.318 | 1.254 → 1.285 | **FAIL** / **FAIL** |
| quat (fix-three before) | 1.178 → 1.220 | 1.245 → 1.199 | pass / pass |
| exp (fix-three before) | 0.886 → 1.003 | 0.935 → 0.910 | **FAIL** / pass |
| color | 1.397 → 1.015 | 1.395 → 1.064 | pass / pass |
| meshlet | 1.332 → 1.212 | 1.564 → 1.417 | **FAIL** / **FAIL** |
| meshlet-raw | 1.182 → 0.997 | 1.299 → 1.337 | pass / **FAIL** |
| sequence | 1.244 → 1.134 | 1.073 → 1.045 | pass / pass |
| wasm eligible scope | 1.151 → 1.179 | 1.156 → 1.168 | **FAIL** / **FAIL** |

S1 includes vertex, NONE views and both meshlet families (1.10 / 1.30).
S2 includes standalone filters and filtered views (1.25 / 1.50).
Sequence family ratios and verdicts use scalar C++ S4 (1.25 / 1.50)
plus frozen P02 minima. Other native ratios use C++ SIMD.
S5 uses shipped upstream JS SIMD (1.25 / 1.60).

S1 A/C: FAIL / FAIL; S2 A/C: FAIL / FAIL.

Registered varied-filter S3 final significant comparisons: **2**.
- caller-buffer / varied-filter-3-tiny-s32 / rust: 1.0294–1.1441
- caller-buffer / varied-filter-3-resident-s32 / rust: 1.0905–1.1188
- S1 allocating: mean 1.120; maximum failures 7; FAIL.
- S1 caller-buffer: mean 1.151; maximum failures 8; FAIL.
- S2 allocating: mean 1.160; maximum failures 7; FAIL.
- S2 caller-buffer: mean 1.104; maximum failures 5; FAIL.
- S5 allocating: mean 1.179; maximum failures 7; FAIL.
- S5 caller-buffer: mean 1.168; maximum failures 8; FAIL.

Historical standalone Exp S3 caller-buffer cases (Rust/scalar-Rust 95% intervals):

| Case | Fix-three | New | Significant new slowdown |
|---|---|---|---|
| varied-filter-3-resident-s4 | 1.0278–1.1123 | 0.9493–1.1323 | no |
| varied-filter-3-streaming-s12 | 1.0063–1.0223 | 0.6006–1.2079 | no |

The historical cases no longer show significant slowdown. Two different
stride-32 caller-buffer Exp cases still fail S3. Allocating Exp streaming-s32
also fails the C++ maximum: stage-one ratio 3.252, interval 1.611–4.250.
No rescue timing or post-measurement source change is selected.

- S4 allocating: mean 1.134; maximum failures 0; minimum failures 0; pass.
- S4 caller-buffer: mean 1.045; maximum failures 0; minimum failures 0; pass.

## Sequence maximum and attribution

| Streaming case / API | Fix-four/C++ SIMD in same pass | New/C++ SIMD | New/fix-four |
|---|---:|---:|---:|
| index-3-v0-streaming-s2 / allocating | 1.571 | 1.104 | 0.703 |
| index-3-v0-streaming-s4 / allocating | 1.795 | 0.976 | 0.544 |
| index-3-v1-streaming-s2 / allocating | 0.965 | 1.103 | 1.143 |
| index-3-v1-streaming-s4 / allocating | 1.815 | 0.933 | 0.514 |
| index-3-v0-streaming-s2 / caller-buffer | 1.026 | 1.093 | 1.065 |
| index-3-v0-streaming-s4 / caller-buffer | 1.033 | 1.044 | 1.010 |
| index-3-v1-streaming-s2 / caller-buffer | 1.005 | 0.991 | 0.986 |
| index-3-v1-streaming-s4 / caller-buffer | 1.039 | 1.056 | 1.016 |

Allocation traces show exact output capacities and different prior-result
lifetimes in the existing Rust/C++ drivers. Keep those drivers unchanged.
The candidate removes eager output zero-fill using initialized stack blocks,
with exact fallible reservation, byte/work limits and scalar error semantics.
Cold packet minor-fault counts remain about 3074 Rust versus 4081 C++;
warm counts approach zero. A fault-count explanation is contradicted by
those counts; kernel service/allocator latency is not isolated. All new
sequence S4 maxima and P02 minima pass. Matched v1 streaming-s4 allocating
new/fix-four is 0.514. The prior binary measures 1.815/C++ in this pass,
so the historical 2.876 maximum is not exactly reproduced. The initialization
change improves this matched case; the historical latency cause remains
unisolated.

## Development, correctness and safety

Color conversion bounds replace FP range reductions with a sufficient alpha
test and canonical fallback. Common encoded depths use an exact constant
scale; mixed depths retain division. Arithmetic order and truncation
remain. Meshlets pack low halfwords and eliminate the typed triangle spill,
with four-byte metadata and proven advances; counter wraps still fall back.
Tiny vertex calls use 128-byte planes for at most two format groups; larger
calls retain 1024. Exp unrolls exact four-wide arithmetic. Wasm Quat rotates
record bytes with two swizzles; no approximate normalize/divide is introduced.

Instruction and asm evidence is retained in counters.json, sequence-packets.json,
development wasm-counts.json and before/development assembly. The wasm
counter image predates the final common-depth Color change. Instructions do not
clear time bars. The per-record sequence append, raw consuming sink and
API-wrapper inlining trials were rejected before timing.

A pre-timing flag check discovered that empty CARGO_ENCODED_RUSTFLAGS
neutralized requested RUSTFLAGS. Those records are retained under
pre-effective-flags and not credited. Corrected builds assert simd128
feature markers and propagate effective flags to wasm/Miri/unsafe-free checks.
Historical fix-four Node timing images contain simd128; its codec wasm SIMD
fixture image lacks the marker and cannot establish actual SIMD execution.

Frozen fixture, malformed and benchmark proofs pass on both APIs, every
local ISA ceiling and executed wasm scalar/SIMD. The known seed odd-tail
input still matches upstream scalar exactly; upstream SIMD returns -3.
UPSTREAM_NOTES.md embeds the reproduction; no exclusion or altered acceptance.
The exhaustive valid-alpha Color proof covers extreme channels across native
levels and actual wasm SIMD/scalar. The seven static/test/Miri receipts,
portability checks and 23-block single
unsafe-module/package gates pass. Node preflight matches 292 golden hashes.

No ARM execution, dedicated host, broader release, Windows/macOS, Moss
integration, triangle-index S4 or phase-0.6 performance qualification is claimed.

## Timing receipts

One final epoch, checkpointed between sub-15-minute visible heavy 4GB bursts.
No accepted pair/completed row repeats. All prior lease receipts survive resumes.
- native native-timing-burst1.log: wrapper 316 s, exit zero.
- wasm wasm-timing-burst1.log: wrapper 117 s, exit zero.

## Every native row

| API | Case | Before | After | Final 95% interval | Pairs | Registered max |
|---|---|---:|---:|---|---:|---|
| allocating | vertex-v0-tiny-s4 | 1.495 | 1.281 | 1.256–1.299 | 8 | pass |
| allocating | view-none-tiny-s4 | 1.447 | 1.394 | 1.300–1.483 | 20+30 | FAIL |
| allocating | vertex-v0-tiny-s12 | 1.234 | 1.285 | 1.176–1.308 | 20+30 | FAIL |
| allocating | view-none-tiny-s12 | 1.494 | 1.326 | 1.218–1.365 | 20+30 | FAIL |
| allocating | vertex-v0-tiny-s32 | 1.389 | 1.277 | 1.176–1.294 | 5 | pass |
| allocating | view-none-tiny-s32 | 1.394 | 1.240 | 1.117–1.373 | 20+30 | FAIL |
| allocating | filter-1-tiny-s4 | 1.202 | 1.409 | 1.339–1.489 | 12 | pass |
| allocating | view-1-tiny-s4 | 1.533 | 1.420 | 1.334–1.482 | 5 | pass |
| allocating | filter-1-tiny-s8 | 1.085 | 1.239 | 1.137–1.312 | 5 | pass |
| allocating | view-1-tiny-s8 | 1.402 | 1.274 | 1.214–1.342 | 5 | pass |
| allocating | filter-2-tiny-s8 | 1.231 | 1.277 | 1.224–1.353 | 5 | pass |
| allocating | view-2-tiny-s8 | 1.453 | 1.316 | 1.236–1.461 | 5 | pass |
| allocating | filter-3-tiny-s12 | 0.948 | 0.863 | 0.837–0.955 | 5 | pass |
| allocating | view-3-tiny-s12 | 1.343 | 1.286 | 1.168–1.486 | 8 | pass |
| allocating | index-3-v0-tiny-s2 | 1.207 | 1.316 | 1.239–1.429 | 8 | pass |
| allocating | index-3-v0-tiny-s4 | 1.230 | 1.159 | 1.103–1.303 | 6 | pass |
| allocating | index-3-v1-tiny-s2 | 1.223 | 1.271 | 1.221–1.434 | 5 | pass |
| allocating | index-3-v1-tiny-s4 | 1.239 | 1.230 | 1.121–1.257 | 5 | pass |
| allocating | vertex-v0-resident-s4 | 1.104 | 1.006 | 0.974–1.267 | 7 | pass |
| allocating | view-none-resident-s4 | 1.076 | 1.074 | 0.852–1.165 | 5 | pass |
| allocating | vertex-v0-resident-s12 | 1.108 | 1.160 | 0.910–1.284 | 20+30 | pass |
| allocating | view-none-resident-s12 | 1.173 | 1.119 | 1.078–1.278 | 8 | pass |
| allocating | vertex-v0-resident-s32 | 1.103 | 1.028 | 1.032–1.292 | 14 | pass |
| allocating | view-none-resident-s32 | 1.109 | 1.089 | 0.891–1.272 | 5 | pass |
| allocating | filter-1-resident-s4 | 0.916 | 1.049 | 0.642–1.276 | 5 | pass |
| allocating | view-1-resident-s4 | 1.179 | 1.022 | 0.722–1.489 | 5 | pass |
| allocating | filter-1-resident-s8 | 0.962 | 1.136 | 0.863–1.331 | 5 | pass |
| allocating | view-1-resident-s8 | 1.212 | 1.106 | 0.696–1.325 | 5 | pass |
| allocating | filter-2-resident-s8 | 1.006 | 1.019 | 0.952–1.114 | 5 | pass |
| allocating | view-2-resident-s8 | 1.154 | 1.159 | 1.093–1.282 | 5 | pass |
| allocating | filter-3-resident-s12 | 1.032 | 0.744 | 0.690–0.861 | 5 | pass |
| allocating | view-3-resident-s12 | 1.060 | 1.088 | 0.745–1.428 | 8 | pass |
| allocating | index-3-v0-resident-s2 | 0.982 | 1.002 | 0.910–1.144 | 5 | pass |
| allocating | index-3-v0-resident-s4 | 0.966 | 1.127 | 0.883–1.419 | 5 | pass |
| allocating | index-3-v1-resident-s2 | 1.073 | 1.021 | 0.987–1.088 | 5 | pass |
| allocating | index-3-v1-resident-s4 | 0.998 | 1.225 | 1.092–1.479 | 17 | pass |
| allocating | vertex-v0-streaming-s4 | 0.873 | 0.941 | 0.835–1.219 | 5 | pass |
| allocating | view-none-streaming-s4 | 0.746 | 0.947 | 0.881–1.263 | 9 | pass |
| allocating | vertex-v0-streaming-s12 | 0.968 | 1.017 | 0.947–1.257 | 6 | pass |
| allocating | view-none-streaming-s12 | 1.009 | 0.971 | 0.670–1.250 | 8 | pass |
| allocating | vertex-v0-streaming-s32 | 1.062 | 1.110 | 0.844–1.292 | 7 | pass |
| allocating | view-none-streaming-s32 | 1.147 | 1.023 | 0.791–1.299 | 5 | pass |
| allocating | filter-1-streaming-s4 | 1.005 | 1.013 | 0.582–1.347 | 5 | pass |
| allocating | view-1-streaming-s4 | 1.990 | 2.090 | 1.607–2.713 | 5 | FAIL |
| allocating | filter-1-streaming-s8 | 0.954 | 0.993 | 0.749–1.435 | 5 | pass |
| allocating | view-1-streaming-s8 | 0.994 | 1.036 | 0.851–1.277 | 5 | pass |
| allocating | filter-2-streaming-s8 | 1.143 | 1.101 | 0.865–1.162 | 5 | pass |
| allocating | view-2-streaming-s8 | 1.053 | 1.098 | 0.969–1.158 | 5 | pass |
| allocating | filter-3-streaming-s12 | 1.057 | 0.984 | 0.858–1.033 | 5 | pass |
| allocating | view-3-streaming-s12 | 1.100 | 1.042 | 0.967–1.145 | 5 | pass |
| allocating | index-3-v0-streaming-s2 | 0.795 | 1.104 | 1.033–1.151 | 5 | pass |
| allocating | index-3-v0-streaming-s4 | 2.523 | 0.976 | 0.800–1.394 | 6 | pass |
| allocating | index-3-v1-streaming-s2 | 0.971 | 1.103 | 1.021–1.139 | 5 | pass |
| allocating | index-3-v1-streaming-s4 | 2.876 | 0.933 | 0.839–1.346 | 5 | pass |
| allocating | vertex-v1-tiny-s4 | 1.408 | 1.266 | 1.244–1.296 | 5 | pass |
| allocating | vertex-v1-tiny-s12 | 1.429 | 1.215 | 1.231–1.300 | 12 | pass |
| allocating | vertex-v1-tiny-s32 | 1.391 | 1.097 | 1.063–1.277 | 6 | pass |
| allocating | vertex-v1-resident-s4 | 1.153 | 1.078 | 0.938–1.111 | 5 | pass |
| allocating | vertex-v1-resident-s12 | 1.142 | 1.138 | 1.051–1.164 | 5 | pass |
| allocating | vertex-v1-resident-s32 | 1.250 | 0.942 | 0.903–1.263 | 5 | pass |
| allocating | vertex-v1-streaming-s4 | 2.050 | 0.916 | 0.720–1.290 | 6 | pass |
| allocating | vertex-v1-streaming-s12 | 1.071 | 0.898 | 0.663–1.104 | 5 | pass |
| allocating | vertex-v1-streaming-s32 | 1.044 | 0.963 | 0.839–1.160 | 5 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.184 | 1.152 | 1.096–1.173 | 5 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.139 | 1.165 | 1.096–1.272 | 5 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.115 | 1.074 | 1.067–1.207 | 5 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.132 | 1.109 | 0.811–1.296 | 11 | pass |
| allocating | varied-filter-1-tiny-s4 | 1.530 | 1.667 | 1.519–1.814 | 8 | FAIL |
| allocating | varied-view-1-tiny-s4 | 1.570 | 1.349 | 1.213–1.487 | 11 | pass |
| allocating | varied-filter-1-tiny-s8 | 1.283 | 1.224 | 1.206–1.243 | 5 | pass |
| allocating | varied-view-1-tiny-s8 | 1.405 | 1.181 | 0.960–1.344 | 5 | pass |
| allocating | varied-filter-2-tiny-s8 | 1.358 | 1.378 | 1.312–1.429 | 5 | pass |
| allocating | varied-view-2-tiny-s8 | 1.413 | 1.209 | 1.112–1.410 | 5 | pass |
| allocating | varied-filter-3-tiny-s4 | 0.870 | 0.903 | 0.894–0.919 | 5 | pass |
| allocating | varied-view-3-tiny-s4 | 1.408 | 1.142 | 1.108–1.158 | 5 | pass |
| allocating | varied-filter-3-tiny-s12 | 0.909 | 0.977 | 0.861–1.046 | 5 | pass |
| allocating | varied-view-3-tiny-s12 | 1.330 | 1.104 | 1.089–1.150 | 5 | pass |
| allocating | varied-filter-3-tiny-s32 | 0.926 | 1.054 | 0.923–1.037 | 5 | pass |
| allocating | varied-view-3-tiny-s32 | 1.324 | 1.122 | 1.105–1.159 | 5 | pass |
| allocating | varied-filter-4-tiny-s4 | 1.378 | 1.085 | 0.973–1.281 | 5 | pass |
| allocating | varied-filter-4-tiny-s8 | 1.352 | 1.110 | 1.074–1.193 | 5 | pass |
| allocating | varied-filter-1-resident-s4 | 1.719 | 1.999 | 1.783–1.990 | 5 | FAIL |
| allocating | varied-view-1-resident-s4 | 1.435 | 1.531 | 1.461–1.534 | 20+30 | FAIL |
| allocating | varied-filter-1-resident-s8 | 1.281 | 1.303 | 1.279–1.393 | 5 | pass |
| allocating | varied-view-1-resident-s8 | 1.312 | 1.274 | 1.153–1.361 | 5 | pass |
| allocating | varied-filter-2-resident-s8 | 1.172 | 1.228 | 1.203–1.310 | 5 | pass |
| allocating | varied-view-2-resident-s8 | 1.243 | 1.253 | 1.200–1.388 | 5 | pass |
| allocating | varied-filter-3-resident-s4 | 0.781 | 0.894 | 0.842–1.073 | 5 | pass |
| allocating | varied-view-3-resident-s4 | 1.057 | 1.045 | 0.990–1.063 | 5 | pass |
| allocating | varied-filter-3-resident-s12 | 0.766 | 0.830 | 0.779–1.059 | 5 | pass |
| allocating | varied-view-3-resident-s12 | 1.085 | 1.075 | 1.036–1.123 | 5 | pass |
| allocating | varied-filter-3-resident-s32 | 0.885 | 0.850 | 0.844–0.863 | 5 | pass |
| allocating | varied-view-3-resident-s32 | 1.124 | 0.952 | 0.915–1.208 | 5 | pass |
| allocating | varied-filter-4-resident-s4 | 1.480 | 1.174 | 0.986–1.169 | 5 | pass |
| allocating | varied-filter-4-resident-s8 | 1.267 | 0.863 | 0.769–0.961 | 5 | pass |
| allocating | varied-filter-1-streaming-s4 | 1.948 | 1.771 | 1.708–1.856 | 5 | FAIL |
| allocating | varied-view-1-streaming-s4 | 1.370 | 1.506 | 1.469–1.504 | 20+30 | FAIL |
| allocating | varied-filter-1-streaming-s8 | 1.310 | 1.395 | 1.220–1.497 | 5 | pass |
| allocating | varied-view-1-streaming-s8 | 1.196 | 1.243 | 1.191–1.312 | 5 | pass |
| allocating | varied-filter-2-streaming-s8 | 1.189 | 1.362 | 1.352–1.400 | 5 | pass |
| allocating | varied-view-2-streaming-s8 | 1.237 | 1.224 | 1.127–1.284 | 5 | pass |
| allocating | varied-filter-3-streaming-s4 | 0.898 | 0.937 | 0.905–0.985 | 5 | pass |
| allocating | varied-view-3-streaming-s4 | 1.089 | 1.010 | 0.909–1.081 | 5 | pass |
| allocating | varied-filter-3-streaming-s12 | 0.848 | 0.918 | 0.894–0.957 | 5 | pass |
| allocating | varied-view-3-streaming-s12 | 1.195 | 1.027 | 0.986–1.099 | 5 | pass |
| allocating | varied-filter-3-streaming-s32 | 0.770 | 3.252 | 1.611–4.250 | 10 | FAIL |
| allocating | varied-view-3-streaming-s32 | 2.261 | 0.978 | 0.843–1.424 | 7 | pass |
| allocating | varied-filter-4-streaming-s4 | 1.565 | 0.963 | 0.955–0.983 | 5 | pass |
| allocating | varied-filter-4-streaming-s8 | 1.360 | 0.930 | 0.893–0.957 | 5 | pass |
| allocating | meshlet-1-1-v2-t3 | 1.383 | 1.425 | 1.278–1.372 | 20+30 | FAIL |
| allocating | meshlet-1-1-v2-t4 | 1.384 | 1.310 | 1.221–1.410 | 20+30 | FAIL |
| allocating | meshlet-1-1-v4-t3 | 1.353 | 1.296 | 1.159–1.272 | 20+30 | pass |
| allocating | meshlet-1-1-v4-t4 | 1.319 | 1.273 | 1.264–1.368 | 20+30 | FAIL |
| allocating | meshlet-raw-1-1 | 1.188 | 0.835 | 0.893–1.268 | 9 | pass |
| allocating | meshlet-64-126-v2-t3 | 1.558 | 1.224 | 1.147–1.258 | 20+30 | pass |
| allocating | meshlet-64-126-v2-t4 | 1.220 | 1.198 | 1.184–1.227 | 5 | pass |
| allocating | meshlet-64-126-v4-t3 | 1.479 | 1.197 | 1.140–1.299 | 19 | pass |
| allocating | meshlet-64-126-v4-t4 | 1.278 | 1.166 | 1.102–1.190 | 5 | pass |
| allocating | meshlet-raw-64-126 | 1.214 | 1.128 | 1.112–1.153 | 5 | pass |
| allocating | meshlet-256-256-v2-t3 | 1.433 | 1.199 | 1.161–1.200 | 5 | pass |
| allocating | meshlet-256-256-v2-t4 | 1.148 | 1.104 | 0.967–1.184 | 5 | pass |
| allocating | meshlet-256-256-v4-t3 | 1.402 | 1.125 | 0.969–1.294 | 6 | pass |
| allocating | meshlet-256-256-v4-t4 | 1.105 | 1.075 | 1.078–1.089 | 5 | pass |
| allocating | meshlet-raw-256-256 | 1.144 | 1.052 | 1.014–1.077 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s4 | 1.508 | 1.222 | 1.125–1.286 | 7 | pass |
| caller-buffer | view-none-tiny-s4 | 1.547 | 1.240 | 1.165–1.285 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.430 | 1.151 | 1.086–1.228 | 5 | pass |
| caller-buffer | view-none-tiny-s12 | 1.433 | 1.205 | 1.161–1.297 | 6 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.363 | 1.031 | 1.022–1.048 | 5 | pass |
| caller-buffer | view-none-tiny-s32 | 1.321 | 1.030 | 1.016–1.057 | 5 | pass |
| caller-buffer | filter-1-tiny-s4 | 1.233 | 1.286 | 1.258–1.334 | 5 | pass |
| caller-buffer | view-1-tiny-s4 | 1.543 | 1.197 | 1.122–1.351 | 5 | pass |
| caller-buffer | filter-1-tiny-s8 | 1.020 | 0.974 | 0.968–0.983 | 5 | pass |
| caller-buffer | view-1-tiny-s8 | 1.436 | 1.121 | 1.103–1.158 | 5 | pass |
| caller-buffer | filter-2-tiny-s8 | 1.324 | 1.188 | 1.134–1.328 | 5 | pass |
| caller-buffer | view-2-tiny-s8 | 1.475 | 1.213 | 1.194–1.238 | 5 | pass |
| caller-buffer | filter-3-tiny-s12 | 0.935 | 0.911 | 0.835–1.064 | 5 | pass |
| caller-buffer | view-3-tiny-s12 | 1.378 | 1.094 | 0.819–1.310 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 1.299 | 0.898 | 0.872–0.919 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 1.240 | 0.982 | 0.753–1.226 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 1.287 | 1.297 | 1.189–1.372 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 1.207 | 1.275 | 1.116–1.423 | 7 | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.073 | 1.060 | 0.999–1.095 | 5 | pass |
| caller-buffer | view-none-resident-s4 | 1.062 | 1.056 | 1.047–1.073 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.127 | 1.128 | 1.074–1.150 | 5 | pass |
| caller-buffer | view-none-resident-s12 | 1.108 | 1.096 | 0.848–1.288 | 7 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.127 | 1.094 | 1.055–1.201 | 5 | pass |
| caller-buffer | view-none-resident-s32 | 1.164 | 1.140 | 1.098–1.196 | 5 | pass |
| caller-buffer | filter-1-resident-s4 | 0.968 | 0.978 | 0.888–1.138 | 5 | pass |
| caller-buffer | view-1-resident-s4 | 1.278 | 0.965 | 0.949–0.971 | 5 | pass |
| caller-buffer | filter-1-resident-s8 | 0.987 | 0.886 | 0.882–0.891 | 5 | pass |
| caller-buffer | view-1-resident-s8 | 1.058 | 0.985 | 0.962–1.031 | 5 | pass |
| caller-buffer | filter-2-resident-s8 | 1.089 | 0.793 | 0.844–1.174 | 5 | pass |
| caller-buffer | view-2-resident-s8 | 1.076 | 1.114 | 1.081–1.151 | 5 | pass |
| caller-buffer | filter-3-resident-s12 | 0.927 | 0.807 | 0.780–0.838 | 5 | pass |
| caller-buffer | view-3-resident-s12 | 1.035 | 1.056 | 0.742–1.268 | 5 | pass |
| caller-buffer | index-3-v0-resident-s2 | 0.994 | 1.009 | 0.959–1.011 | 5 | pass |
| caller-buffer | index-3-v0-resident-s4 | 0.987 | 0.993 | 0.990–1.007 | 5 | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.786 | 0.905 | 0.869–0.944 | 5 | pass |
| caller-buffer | index-3-v1-resident-s4 | 1.037 | 1.002 | 0.969–1.135 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 0.877 | 0.947 | 0.775–1.292 | 5 | pass |
| caller-buffer | view-none-streaming-s4 | 0.864 | 0.906 | 0.687–1.222 | 6 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 0.973 | 0.821 | 0.772–0.898 | 5 | pass |
| caller-buffer | view-none-streaming-s12 | 1.074 | 0.764 | 0.685–0.848 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s32 | 0.987 | 0.952 | 0.853–1.108 | 5 | pass |
| caller-buffer | view-none-streaming-s32 | 1.220 | 0.886 | 0.732–1.042 | 5 | pass |
| caller-buffer | filter-1-streaming-s4 | 0.981 | 1.032 | 0.877–1.418 | 7 | pass |
| caller-buffer | view-1-streaming-s4 | 0.923 | 0.790 | 0.648–1.249 | 5 | pass |
| caller-buffer | filter-1-streaming-s8 | 0.981 | 1.290 | 1.190–1.315 | 5 | pass |
| caller-buffer | view-1-streaming-s8 | 0.975 | 0.953 | 0.761–1.049 | 5 | pass |
| caller-buffer | filter-2-streaming-s8 | 1.081 | 1.385 | 1.024–1.486 | 14 | pass |
| caller-buffer | view-2-streaming-s8 | 0.945 | 0.573 | 0.587–1.174 | 5 | pass |
| caller-buffer | filter-3-streaming-s12 | 0.974 | 0.955 | 0.879–1.074 | 5 | pass |
| caller-buffer | view-3-streaming-s12 | 0.875 | 0.756 | 0.653–0.903 | 5 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 1.000 | 1.093 | 0.958–1.385 | 5 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 0.970 | 1.044 | 0.954–1.157 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 1.239 | 0.991 | 0.748–1.149 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 1.027 | 1.056 | 0.994–1.077 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 1.509 | 1.240 | 1.236–1.252 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.230 | 1.273 | 1.242–1.299 | 8 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.183 | 1.015 | 1.043–1.288 | 8 | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.030 | 1.041 | 0.907–1.141 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.121 | 1.177 | 1.155–1.201 | 5 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.102 | 1.126 | 1.062–1.160 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 0.745 | 0.934 | 0.804–1.041 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 0.655 | 0.830 | 0.726–0.970 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 0.946 | 0.976 | 0.872–1.021 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.167 | 1.097 | 1.016–1.187 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.175 | 1.175 | 1.092–1.241 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.225 | 1.079 | 1.042–1.172 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.173 | 0.996 | 0.991–1.267 | 8 | pass |
| caller-buffer | varied-filter-1-tiny-s4 | 1.885 | 1.716 | 1.692–1.845 | 5 | FAIL |
| caller-buffer | varied-view-1-tiny-s4 | 1.670 | 1.478 | 1.356–1.633 | 20+30 | FAIL |
| caller-buffer | varied-filter-1-tiny-s8 | 1.142 | 1.307 | 1.219–1.322 | 5 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.372 | 1.339 | 1.044–1.433 | 6 | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 1.279 | 1.334 | 1.286–1.485 | 10 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.229 | 1.251 | 1.220–1.305 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 0.885 | 0.887 | 0.810–1.055 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.192 | 1.105 | 1.040–1.120 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s12 | 0.938 | 0.967 | 0.874–1.116 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.153 | 1.552 | 1.067–1.500 | 5 | pass |
| caller-buffer | varied-filter-3-tiny-s32 | 1.028 | 1.051 | 0.806–1.218 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.162 | 1.262 | 1.199–1.377 | 5 | pass |
| caller-buffer | varied-filter-4-tiny-s4 | 1.432 | 1.358 | 1.330–1.381 | 5 | pass |
| caller-buffer | varied-filter-4-tiny-s8 | 1.215 | 1.288 | 1.266–1.329 | 5 | pass |
| caller-buffer | varied-filter-1-resident-s4 | 1.770 | 1.778 | 1.610–1.908 | 5 | FAIL |
| caller-buffer | varied-view-1-resident-s4 | 1.447 | 1.533 | 1.441–1.526 | 20+30 | FAIL |
| caller-buffer | varied-filter-1-resident-s8 | 1.383 | 1.433 | 1.367–1.491 | 10 | pass |
| caller-buffer | varied-view-1-resident-s8 | 1.236 | 1.276 | 1.250–1.320 | 5 | pass |
| caller-buffer | varied-filter-2-resident-s8 | 1.371 | 1.359 | 1.295–1.393 | 5 | pass |
| caller-buffer | varied-view-2-resident-s8 | 1.246 | 1.266 | 1.213–1.325 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s4 | 0.984 | 0.937 | 0.831–1.219 | 5 | pass |
| caller-buffer | varied-view-3-resident-s4 | 0.990 | 1.006 | 0.996–1.014 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s12 | 0.963 | 0.834 | 0.808–0.893 | 5 | pass |
| caller-buffer | varied-view-3-resident-s12 | 0.961 | 1.109 | 1.045–1.123 | 5 | pass |
| caller-buffer | varied-filter-3-resident-s32 | 0.805 | 0.911 | 0.903–0.916 | 5 | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.160 | 1.102 | 1.047–1.181 | 5 | pass |
| caller-buffer | varied-filter-4-resident-s4 | 1.519 | 0.982 | 0.934–1.097 | 5 | pass |
| caller-buffer | varied-filter-4-resident-s8 | 1.369 | 0.913 | 0.894–0.932 | 5 | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 1.748 | 1.783 | 1.684–2.034 | 5 | FAIL |
| caller-buffer | varied-view-1-streaming-s4 | 1.490 | 1.462 | 1.428–1.496 | 11 | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 1.422 | 1.378 | 1.342–1.455 | 5 | pass |
| caller-buffer | varied-view-1-streaming-s8 | 1.274 | 1.231 | 0.883–1.436 | 6 | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 1.361 | 1.254 | 1.253–1.298 | 5 | pass |
| caller-buffer | varied-view-2-streaming-s8 | 1.218 | 0.968 | 0.978–1.377 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 0.941 | 0.946 | 0.713–1.120 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1.041 | 1.248 | 0.911–1.435 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 0.939 | 0.778 | 0.674–0.862 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.117 | 1.041 | 0.935–1.215 | 5 | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 0.923 | 0.978 | 0.658–1.183 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.097 | 1.039 | 0.586–1.359 | 5 | pass |
| caller-buffer | varied-filter-4-streaming-s4 | 1.543 | 1.013 | 0.679–1.179 | 5 | pass |
| caller-buffer | varied-filter-4-streaming-s8 | 1.317 | 0.913 | 0.826–1.042 | 5 | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 1.904 | 1.845 | 1.749–1.993 | 5 | FAIL |
| caller-buffer | meshlet-1-1-v2-t4 | 2.028 | 1.902 | 1.774–1.998 | 5 | FAIL |
| caller-buffer | meshlet-1-1-v4-t3 | 2.202 | 1.716 | 1.345–2.040 | 6 | FAIL |
| caller-buffer | meshlet-1-1-v4-t4 | 1.882 | 1.737 | 1.716–1.775 | 5 | FAIL |
| caller-buffer | meshlet-raw-1-1 | 1.578 | 1.609 | 1.385–1.619 | 5 | FAIL |
| caller-buffer | meshlet-64-126-v2-t3 | 1.532 | 1.409 | 1.307–1.469 | 7 | FAIL |
| caller-buffer | meshlet-64-126-v2-t4 | 1.242 | 1.306 | 1.214–1.295 | 20+30 | pass |
| caller-buffer | meshlet-64-126-v4-t3 | 1.650 | 1.307 | 1.342–1.428 | 20+30 | FAIL |
| caller-buffer | meshlet-64-126-v4-t4 | 1.344 | 1.346 | 1.306–1.354 | 20+30 | FAIL |
| caller-buffer | meshlet-raw-64-126 | 1.214 | 1.312 | 1.200–1.270 | 20+30 | pass |
| caller-buffer | meshlet-256-256-v2-t3 | 1.483 | 1.281 | 1.207–1.291 | 20+30 | pass |
| caller-buffer | meshlet-256-256-v2-t4 | 1.141 | 1.160 | 1.008–1.276 | 5 | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 1.443 | 1.035 | 0.996–1.172 | 5 | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 1.298 | 1.263 | 1.150–1.286 | 9 | pass |
| caller-buffer | meshlet-raw-256-256 | 1.146 | 1.131 | 0.962–1.275 | 6 | pass |

## Every Node row

| API | Case | Before | After | Final 95% interval | Pairs | Registered max |
|---|---|---:|---:|---|---:|---|
| allocating | vertex-v0-tiny-s4 | 1.173 | 1.082 | 0.874–1.570 | 17 | pass |
| allocating | view-none-tiny-s4 | 1.201 | 1.066 | 0.495–1.471 | 6 | pass |
| allocating | vertex-v0-tiny-s12 | 1.108 | 1.193 | 0.980–1.225 | 5 | pass |
| allocating | view-none-tiny-s12 | 1.225 | 1.238 | 1.039–1.523 | 8 | pass |
| allocating | vertex-v0-tiny-s32 | 1.135 | 1.141 | 1.047–1.230 | 5 | pass |
| allocating | view-none-tiny-s32 | 0.943 | 1.176 | 1.047–1.343 | 5 | pass |
| allocating | view-1-tiny-s4 | 0.931 | 1.075 | 1.291–1.592 | 20+30 | pass |
| allocating | view-1-tiny-s8 | 1.016 | 1.345 | 1.203–1.490 | 5 | pass |
| allocating | view-2-tiny-s8 | 0.897 | 0.514 | 0.180–1.528 | 7 | pass |
| allocating | view-3-tiny-s12 | 1.121 | 1.228 | 1.043–1.428 | 5 | pass |
| allocating | index-3-v0-tiny-s2 | 0.686 | 0.697 | 0.602–1.467 | 6 | pass |
| allocating | index-3-v0-tiny-s4 | 0.920 | 1.058 | 0.821–1.205 | 5 | pass |
| allocating | index-3-v1-tiny-s2 | 1.074 | 0.999 | 0.884–1.160 | 5 | pass |
| allocating | index-3-v1-tiny-s4 | 0.989 | 1.067 | 0.861–1.332 | 5 | pass |
| allocating | vertex-v0-resident-s4 | 0.877 | 1.004 | 0.766–1.150 | 5 | pass |
| allocating | view-none-resident-s4 | 1.136 | 1.158 | 1.063–1.274 | 5 | pass |
| allocating | vertex-v0-resident-s12 | 1.085 | 1.064 | 0.891–1.246 | 5 | pass |
| allocating | view-none-resident-s12 | 1.219 | 1.162 | 1.131–1.223 | 5 | pass |
| allocating | vertex-v0-resident-s32 | 1.401 | 1.165 | 1.133–1.313 | 5 | pass |
| allocating | view-none-resident-s32 | 1.152 | 1.103 | 0.859–1.312 | 5 | pass |
| allocating | view-1-resident-s4 | 1.575 | 1.539 | 1.432–1.597 | 19 | pass |
| allocating | view-1-resident-s8 | 1.748 | 1.887 | 1.652–2.025 | 6 | FAIL |
| allocating | view-2-resident-s8 | 1.683 | 1.726 | 1.620–1.854 | 5 | FAIL |
| allocating | view-3-resident-s12 | 1.515 | 1.502 | 1.492–1.562 | 20+30 | pass |
| allocating | index-3-v0-resident-s2 | 0.858 | 0.967 | 0.947–1.061 | 5 | pass |
| allocating | index-3-v0-resident-s4 | 0.830 | 0.955 | 0.888–1.119 | 5 | pass |
| allocating | index-3-v1-resident-s2 | 0.958 | 0.987 | 0.961–1.014 | 5 | pass |
| allocating | index-3-v1-resident-s4 | 0.835 | 0.996 | 0.799–1.067 | 5 | pass |
| allocating | vertex-v0-streaming-s4 | 0.977 | 1.045 | 0.608–1.495 | 6 | pass |
| allocating | view-none-streaming-s4 | 1.068 | 1.008 | 0.706–1.544 | 9 | pass |
| allocating | vertex-v0-streaming-s12 | 1.237 | 1.120 | 0.844–1.544 | 9 | pass |
| allocating | view-none-streaming-s12 | 0.916 | 1.732 | 1.095–1.592 | 10 | pass |
| allocating | vertex-v0-streaming-s32 | 1.022 | 1.088 | 0.895–1.213 | 5 | pass |
| allocating | view-none-streaming-s32 | 1.086 | 1.028 | 1.023–1.166 | 5 | pass |
| allocating | view-1-streaming-s4 | 1.510 | 1.526 | 1.455–1.593 | 8 | pass |
| allocating | view-1-streaming-s8 | 1.741 | 1.810 | 1.640–2.346 | 5 | FAIL |
| allocating | view-2-streaming-s8 | 1.709 | 1.704 | 1.601–1.788 | 7 | FAIL |
| allocating | view-3-streaming-s12 | 1.005 | 1.084 | 0.943–1.233 | 5 | pass |
| allocating | index-3-v0-streaming-s2 | 0.606 | 0.697 | 0.555–1.362 | 5 | pass |
| allocating | index-3-v0-streaming-s4 | 1.026 | 0.936 | 0.651–1.438 | 5 | pass |
| allocating | index-3-v1-streaming-s2 | 0.835 | 0.890 | 0.709–1.079 | 5 | pass |
| allocating | index-3-v1-streaming-s4 | 0.899 | 1.042 | 0.631–1.413 | 6 | pass |
| allocating | vertex-v1-tiny-s4 | 1.121 | 1.170 | 0.669–1.538 | 8 | pass |
| allocating | vertex-v1-tiny-s12 | 1.309 | 1.237 | 1.096–1.398 | 5 | pass |
| allocating | vertex-v1-tiny-s32 | 1.128 | 1.291 | 1.243–1.480 | 5 | pass |
| allocating | vertex-v1-resident-s4 | 1.056 | 1.044 | 1.018–1.066 | 5 | pass |
| allocating | vertex-v1-resident-s12 | 1.226 | 1.167 | 1.117–1.218 | 5 | pass |
| allocating | vertex-v1-resident-s32 | 1.191 | 1.169 | 1.083–1.324 | 5 | pass |
| allocating | vertex-v1-streaming-s4 | 1.157 | 1.101 | 0.735–1.593 | 8 | pass |
| allocating | vertex-v1-streaming-s12 | 0.869 | 1.126 | 0.877–1.559 | 8 | pass |
| allocating | vertex-v1-streaming-s32 | 0.900 | 1.085 | 0.961–1.203 | 5 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.238 | 1.163 | 1.060–1.253 | 5 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.144 | 1.171 | 0.589–1.507 | 5 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.218 | 1.151 | 1.025–1.499 | 6 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.165 | 1.175 | 0.687–1.562 | 5 | pass |
| allocating | varied-view-1-tiny-s4 | 1.282 | 1.341 | 1.193–1.592 | 14 | pass |
| allocating | varied-view-1-tiny-s8 | 1.245 | 1.375 | 1.231–1.424 | 5 | pass |
| allocating | varied-view-2-tiny-s8 | 1.227 | 1.315 | 1.297–1.343 | 5 | pass |
| allocating | varied-view-3-tiny-s4 | 1.336 | 1.198 | 1.087–1.259 | 5 | pass |
| allocating | varied-view-3-tiny-s12 | 1.218 | 1.200 | 1.081–1.393 | 5 | pass |
| allocating | varied-view-3-tiny-s32 | 1.129 | 1.238 | 1.098–1.340 | 5 | pass |
| allocating | varied-view-1-resident-s4 | 1.515 | 1.614 | 1.553–1.647 | 20+30 | FAIL |
| allocating | varied-view-1-resident-s8 | 1.558 | 1.502 | 1.343–1.597 | 20 | pass |
| allocating | varied-view-2-resident-s8 | 1.385 | 1.453 | 1.335–1.592 | 5 | pass |
| allocating | varied-view-3-resident-s4 | 1.066 | 1.134 | 0.989–1.275 | 5 | pass |
| allocating | varied-view-3-resident-s12 | 1.206 | 1.290 | 1.095–1.578 | 14 | pass |
| allocating | varied-view-3-resident-s32 | 1.350 | 1.188 | 1.079–1.380 | 5 | pass |
| allocating | varied-view-1-streaming-s4 | 1.649 | 1.615 | 1.177–1.506 | 20+30 | pass |
| allocating | varied-view-1-streaming-s8 | 1.824 | 1.504 | 1.196–1.650 | 20+30 | FAIL |
| allocating | varied-view-2-streaming-s8 | 1.639 | 1.570 | 1.229–1.727 | 20+30 | FAIL |
| allocating | varied-view-3-streaming-s4 | 1.072 | 1.085 | 0.914–1.565 | 6 | pass |
| allocating | varied-view-3-streaming-s12 | 1.438 | 1.288 | 1.041–1.583 | 16 | pass |
| allocating | varied-view-3-streaming-s32 | 1.266 | 1.194 | 0.939–1.492 | 6 | pass |
| caller-buffer | vertex-v0-tiny-s4 | 0.892 | 1.125 | 0.589–1.396 | 6 | pass |
| caller-buffer | view-none-tiny-s4 | 1.069 | 1.002 | 0.809–1.127 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.083 | 1.024 | 0.963–1.069 | 5 | pass |
| caller-buffer | view-none-tiny-s12 | 1.029 | 1.039 | 0.855–1.391 | 5 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.058 | 1.261 | 1.110–1.262 | 5 | pass |
| caller-buffer | view-none-tiny-s32 | 1.148 | 1.103 | 1.017–1.189 | 5 | pass |
| caller-buffer | view-1-tiny-s4 | 1.108 | 1.229 | 0.848–1.384 | 5 | pass |
| caller-buffer | view-1-tiny-s8 | 1.297 | 1.338 | 1.128–1.457 | 5 | pass |
| caller-buffer | view-2-tiny-s8 | 1.276 | 1.271 | 1.119–1.551 | 8 | pass |
| caller-buffer | view-3-tiny-s12 | 1.303 | 0.898 | 0.813–1.220 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.837 | 0.812 | 0.706–1.179 | 5 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.781 | 0.800 | 0.649–0.985 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.691 | 0.822 | 0.665–1.302 | 5 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 0.881 | 0.769 | 0.698–0.894 | 5 | pass |
| caller-buffer | vertex-v0-resident-s4 | 0.953 | 1.086 | 1.038–1.260 | 5 | pass |
| caller-buffer | view-none-resident-s4 | 1.004 | 1.026 | 0.921–1.262 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.163 | 1.256 | 1.214–1.391 | 5 | pass |
| caller-buffer | view-none-resident-s12 | 1.183 | 1.175 | 1.112–1.270 | 5 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.102 | 1.209 | 1.181–1.255 | 5 | pass |
| caller-buffer | view-none-resident-s32 | 1.235 | 1.147 | 1.123–1.189 | 5 | pass |
| caller-buffer | view-1-resident-s4 | 1.578 | 1.593 | 1.529–1.625 | 20+30 | FAIL |
| caller-buffer | view-1-resident-s8 | 2.007 | 1.919 | 1.799–2.010 | 5 | FAIL |
| caller-buffer | view-2-resident-s8 | 1.850 | 1.855 | 1.610–2.015 | 7 | FAIL |
| caller-buffer | view-3-resident-s12 | 1.612 | 1.583 | 1.552–1.626 | 20+30 | FAIL |
| caller-buffer | index-3-v0-resident-s2 | 0.902 | 0.816 | 0.743–0.907 | 5 | pass |
| caller-buffer | index-3-v0-resident-s4 | 0.819 | 0.887 | 0.813–0.946 | 5 | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.842 | 0.822 | 0.814–0.862 | 5 | pass |
| caller-buffer | index-3-v1-resident-s4 | 0.842 | 0.751 | 0.637–0.954 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 1.086 | 1.141 | 1.048–1.238 | 5 | pass |
| caller-buffer | view-none-streaming-s4 | 1.078 | 1.067 | 0.977–1.196 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.235 | 1.221 | 1.165–1.318 | 5 | pass |
| caller-buffer | view-none-streaming-s12 | 1.304 | 1.165 | 1.120–1.206 | 5 | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1.102 | 1.157 | 0.705–1.480 | 6 | pass |
| caller-buffer | view-none-streaming-s32 | 1.174 | 1.105 | 1.089–1.136 | 5 | pass |
| caller-buffer | view-1-streaming-s4 | 1.601 | 1.555 | 1.480–1.590 | 9 | pass |
| caller-buffer | view-1-streaming-s8 | 1.913 | 1.781 | 1.713–1.896 | 5 | FAIL |
| caller-buffer | view-2-streaming-s8 | 1.826 | 1.654 | 1.606–1.816 | 11 | FAIL |
| caller-buffer | view-3-streaming-s12 | 1.471 | 1.417 | 1.247–1.553 | 7 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 0.844 | 0.838 | 0.811–0.925 | 5 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 0.874 | 0.865 | 0.849–0.916 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.847 | 0.811 | 0.750–0.855 | 5 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 0.868 | 0.862 | 0.817–0.920 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 0.921 | 0.916 | 0.345–1.418 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.135 | 1.210 | 1.099–1.288 | 5 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.273 | 1.328 | 1.259–1.582 | 7 | pass |
| caller-buffer | vertex-v1-resident-s4 | 1.058 | 1.103 | 1.061–1.154 | 5 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.120 | 1.197 | 1.137–1.296 | 5 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.159 | 1.227 | 1.018–1.347 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1.068 | 1.094 | 1.042–1.184 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.152 | 1.246 | 1.191–1.294 | 5 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1.095 | 1.110 | 1.052–1.176 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.103 | 1.207 | 1.187–1.232 | 5 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.171 | 1.361 | 0.823–1.587 | 7 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.216 | 1.371 | 1.117–1.591 | 6 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.147 | 1.329 | 1.231–1.575 | 9 | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.043 | 1.198 | 0.952–1.470 | 5 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.187 | 1.261 | 1.034–1.530 | 7 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.062 | 1.142 | 1.100–1.352 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 0.905 | 0.874 | 0.828–0.955 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 0.988 | 1.225 | 0.987–1.579 | 5 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.115 | 0.986 | 0.908–1.507 | 7 | pass |
| caller-buffer | varied-view-1-resident-s4 | 1.674 | 1.662 | 1.551–1.678 | 20+30 | FAIL |
| caller-buffer | varied-view-1-resident-s8 | 1.585 | 1.509 | 1.470–1.589 | 7 | pass |
| caller-buffer | varied-view-2-resident-s8 | 1.476 | 1.527 | 1.459–1.573 | 5 | pass |
| caller-buffer | varied-view-3-resident-s4 | 1.088 | 1.061 | 1.013–1.097 | 5 | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.279 | 1.245 | 1.200–1.276 | 5 | pass |
| caller-buffer | varied-view-3-resident-s32 | 1.266 | 1.273 | 1.111–1.512 | 5 | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1.680 | 1.648 | 1.606–1.685 | 7 | FAIL |
| caller-buffer | varied-view-1-streaming-s8 | 1.617 | 1.495 | 1.502–1.597 | 10 | pass |
| caller-buffer | varied-view-2-streaming-s8 | 1.495 | 1.543 | 1.535–1.598 | 10 | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1.119 | 1.107 | 1.101–1.113 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.377 | 1.317 | 1.299–1.354 | 5 | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1.310 | 1.359 | 1.305–1.390 | 5 | pass |
