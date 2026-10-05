# P07 SIMD implementation and qualification

The first amendment scope is implemented: vertex groups and reconstruction,
Oct/Quat/Exp/Color filters, and meshlet vertex/triangle decoding. Index and
sequence decoding remain scalar, as required by the accepted amendment.
Default Cargo features include `simd`; disabling default features removes all
unsafe code. Runtime x86 detection requires SSSE3 + POPCNT for vertex groups,
and additionally SSE4.1 for meshlets. SSE2 filters work on baseline x86-64.
AArch64 uses NEON; wasm uses simd128 only when compiled with that feature.
Wasm meshlets retain the scalar path. AVX2 and AVX-512 are deferred.

This is an implementation qualification, not a release qualification. No
upstream-parity or complete release-safety claim is made. The missing records
listed below must be supplied before any SIMD release ships.

## Evidence and identities

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07`. The committed preregistration
is `179ba52`; `SIMD_BAR.md` SHA-256 is
`b5730ff38c5b03ed0870004842a775fd286cc3545d4d354b3bd697f3074edca6`.
The C++ 1.3 oracle is commit `4c203430ca565cb59a468a91922c76c208169536`.
Build/source/binary hashes, every input/output archive, raw samples, admission
telemetry and logs are retained. `parity/verify-simd.py` validates current source
and binary identities and every case hash independently. The final artifact
manifest also covers the ASan executable and source archive.

- fmt; root all-target/all-feature clippy with `-D warnings`; all-feature and
  no-default-feature tests; unsafe-free tests with and without std.
- MSRV 1.88 all-feature check; AArch64 all-feature clippy; wasm all-feature
  clippy with and without simd128; unsafe-free wasm build; rustdoc with warnings
  denied. `no_std` SIMD + diagnostic configurations also pass clippy on x86
  (compile-time SSSE3/POPCNT/SSE4.1) and wasm (+simd128).
- One private unsafe module, one allowance, 17 individually audited blocks.
  The token-aware repository and published-package boundary gates pass.
- Miri: safe scalar codecs, dispatch lowering/unwind, SSE2/SSSE3/SSE4.1
  integer kernels, filters and meshlets. Kernel runs use
  `-C target-feature=+ssse3,+popcnt,+sse4.1`. No unsupported intrinsic was
  encountered in these runs. NEON is not Miri-covered.
- Exact canonical parity: 869 fixtures, 7,653 malformed cases, 22,000 seeded
  cases and 138 benchmark inputs, both APIs, scalar/SSE2/SSSE3/SSE4.1 and
  executed wasm with/without simd128. Rust `Result` and successful destination
  bytes, including caller-buffer tails, match exactly. Scalar C++ matches
  successful bytes; C++ SIMD Oct/Quat conformance permits one decoded unit.
  Raw Color SIMD distance is reported separately under P07-D3.
- All 87 eligible benchmark inputs also conform through the pinned shipped JS
  SIMD decoder's public API, with destination tails preserved: Oct/Quat within
  the separate one-unit allowance, all other outputs exact. This is independent
  of the Node timing loop (`wasm-upstream-conformance.json`).
- x86 and executed wasm exhaustive checks: all 2^32 sqrt and Exp patterns,
  all 2^24 Oct8 inputs, 10^8 seeded Oct16 and Quat records each, plus edge
  combinations and every Quat scale word. P07-D2 explains canonical NaN
  normalization around hardware sqrt. This qualifies the adapter, not the
  bare hardware instruction on NaNs.
- Combined ASan/libFuzzer smoke: 1,552,417 executions in 301 wall seconds,
  no crashes, one pinned core, peak RSS 493 MB. Execution checkpoint lower
  bounds: 1,531,904 each for scalar/SSE2/SSSE3/SSE4.1. CPU time was not
  measured; this gives no credit toward the per-target release budget.
  Version inspectors were then added to the differential harness and passed
  separate ten-second ASan smokes: 497,664 and 477,184 checkpointed calls
  per supported ceiling. Original combined-harness source objects are retained
  by their recorded hashes; the production kernels did not change.
  MSan/HWASan were not required or run.

## Performance method

The owner's binding directive replaces the amendment's sample counts.
Iteration measures only changed cases, five interleaved pairs. The complete
final native and wasm matrices each run once, with 5–20 paired samples and
95% Student-t intervals on log ratios. Clear cases stop immediately;
only borderline cases receive 30 fresh D146 pairs. Stage-1 means remain
unchanged by stage 2. All backends rotate order on one pinned CPU. GPU leases
or active scoreboard measurement pause timing; overlapping pairs are discarded
and retained. Each burst is bounded below 15 minutes and releases its core. After 111
completed rows, the native controller continued from its checkpoint with a
two-second admission poll; it retained every completed row and skipped those
API/case keys. Both controller hashes and the pre-resume record are retained
(P07-D4). The binaries and numeric protocol remained unchanged.

Native completes all 276 rows and validates source/binary identities before
the handoff watcher stops this lane's controller. Its original parent exits
143 deliberately during Node bootstrap, with zero completed Node rows; this
is recorded in `native-handoff.json`. The Node reporting adapter retains every
discarded pair and stage-2 admission/load record, hashes the exact executed
controller text, and completes all 174 rows with exit 0 (P07-D5). The S4
assessment also exits 0. No source or measured binary changes across these
controllers.

The raw native harness retains its coarse legacy `s3_applicable` flag.
Final assessment includes P02 Exp's varying inputs in S2/S3; only repeated
Oct/Quat records are exempt. S3 assesses both stages independently: a stage-2
pass cannot clear a stage-1 regression, and either significant slowdown fails.
The raw helper's stage-2-only S3 summary is retained, but the final report
recomputes both intervals from the raw samples. This changes no input, sample
or numeric bar.

S4 uses the separate scalar C++ binary. Four scalar-baseline intervals remained
borderline when the native controller stopped on its C++ SIMD comparison.
Only these receive additional pairs to twenty total, with thirty fresh D146
pairs only if still borderline. The supplemental record keeps original
matrix samples, family means, medians and frozen minima unchanged. Clear
scalar failures are not retried (P07-D6).
All four borderline cases clear with 5, 2, 1 and 1 additional pairs: nine in
total, with no S4 second stage needed. The full matrices use four native and
twelve Node D146 cases. The verifier checks every admitted pair's telemetry.

Native Rust and C++ use release/fat-LTO/one-codegen-unit and strict `-O3`
respectively, generic CPU settings and matching validation, copies and
allocations. The canonical C++ scalar executable is separate from the SIMD
comparison. Native SIMD enables `std,simd,parity-internals`; its lowering-only
diagnostics add dispatch checks. The separate safe scalar library disables
default features. The recorded ratios describe these driver builds, rather
than an uninstrumented consumer build. Node uses the pinned upstream shipped JS SIMD decoder through its
public API, matching source/output transfers and allocating/caller-buffer forms.
The Node scalar timing is a diagnostic lowering within the SIMD-enabled
module; it does not qualify performance against an unsafe-free wasm build.
Standalone filters and meshlets have no upstream JS API, so S5 covers raw
vertex/index/sequence and buffer-view cases. Shared-host load is recorded;
these results do not establish dedicated-host or Moss integration performance.

## Native measured gaps

Geometric means of Rust time / C++ SIMD time; lower is faster. Both APIs use the complete frozen matrix. Repeated standalone Oct/Quat are excluded from these two varied-filter rows and remain in the individual record.

| Function | Allocating | Caller buffer |
|---|---:|---:|
| vertex | 2.283 | 2.610 |
| view-none | 2.121 | 2.583 |
| view-filtered | 2.255 | 2.617 |
| oct | 2.662 | 2.668 |
| quat | 2.607 | 3.218 |
| exp | 0.951 | 0.970 |
| color | 1.511 | 1.492 |
| meshlet | 2.296 | 3.115 |
| meshlet-raw | 1.830 | 2.181 |
| index | 1.154 | 1.061 |
| sequence | 1.094 | 1.082 |

S1 and S2 fail. S3 also fails: 49 case/ceiling comparisons show a significant
slowdown against safe scalar Rust (18 allocating, 31 caller-buffer). S4 passes
for caller-buffer. Allocating S4 fails `index-2-v0-streaming-s4`: median time
ratio 2.268 against scalar C++, paired 95% interval 1.674–2.494, wholly above
1.50. All frozen P02 index/sequence minima pass under both APIs. These misses
are retained; no upstream-parity or release claim is made.

| Bar | Allocating GM | Caller-buffer GM | Verdict |
|---|---:|---:|---|
| S1 | 2.221 | 2.696 | fail both APIs |
| S2 | 1.849 | 2.028 | fail both APIs |
| S3 | — | — | 49 significant case/ceiling regressions |
| S4 (scalar C++) | 1.158 | 1.057 | allocating fail; caller-buffer pass |

[SIMD_PERFORMANCE.md](SIMD_PERFORMANCE.md) publishes every case, throughput,
interval, pair count and independently assessed S3/S4 result. Raw records and
`performance-summary.json` retain all descriptive maxima and failures.

## Executed wasm measured gaps

The one Node matrix contains 174 unique API/case rows and exits successfully.
S5 fails under both APIs. Ratios use stage-1 backend medians; case decisions
use paired log-ratio confidence intervals, including fresh D146 data only for
borderline maxima. These statistics can differ from the displayed median ratio.
The upstream shipped JS source hash is unchanged before/after the run.

| API | Cases | Rust/upstream SIMD GM | Worst stage-1 ratio | Failed case maxima |
|---|---:|---:|---:|---:|
| allocating | 87 | 2.457 | 6.732 | 59 |
| caller-buffer | 87 | 2.769 | 7.281 | 60 |

## Remaining release and CI records

Linux AArch64 execution must supply NEON parity, exhaustive arithmetic and ASan
fuzz evidence. Windows x86-64 and macOS AArch64 must execute identity against
qualified Linux outputs. The workflow provides those jobs and manual native/
wasm exhaustive runs, but this local host does not establish their results.
Every existing differential decoder fuzz target needs 24 CPU-hours on EACH
native host for the first SIMD release; the smoke is insufficient. Native
nightly CI records measured process CPU seconds and per-level checkpoints in
bursts of at most ten minutes. wasm needs 10^7 seeded differential cases; the streaming runner
`parity/wasm-simd-seeded.py` passes its 1,000-case smoke and the manual CI job
requests the full budget;
release sweeps need 10,000 cases per family. Neither budget is claimed here.
AArch64 timing needs a dedicated named host to gate, or shared-CI ratios for
reporting only. P1 is unestablished: the phase 0.6 parallel APIs are absent in
this checkout; inventing them would exceed P07's scope. There is no thread-count
or wasm-worker composition qualification in this record.
