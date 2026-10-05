# P07 SIMD implementation and qualification

Except for the explicitly current native measured-gaps table, the original
evidence below belongs to implementation `2feefb2`. The P07-perf
refresh is recorded separately at the end; historical ratios and safety-block
counts describe that earlier source, not the refreshed kernels.

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

## Native measured gaps — current round three

Current evidence is `/mnt/linux-extra/meshopt-artifacts/p07r3/performance.json`,
implementation `4e7eea0` (documented HEAD `002872f`). These are geometric means
of stage-1 Rust time / C++ SIMD time over the frozen matrix; lower is faster.
Repeated standalone Oct/Quat inputs remain separate from the varied-filter rows.

The former vertex 2.283 / 2.610 table described original implementation `2feefb2`
in `/mnt/linux-extra/meshopt-artifacts/p07`; it was not the round-three result.
The P07-perf and P07-R3 sections below preserve historical comparisons.
This table now agrees with [SIMD_PERFORMANCE.md](SIMD_PERFORMANCE.md).

| Function | Allocating | Caller buffer |
|---|---:|---:|
| vertex | 1.832 | 1.967 |
| view-none | 1.702 | 1.872 |
| view-filtered | 1.841 | 1.826 |
| oct | 1.492 | 1.535 |
| quat | 1.237 | 1.336 |
| exp | 0.886 | 0.935 |
| color | 1.540 | 1.496 |
| meshlet | 1.329 | 1.534 |
| meshlet-raw | 1.228 | 1.315 |
| index | 1.144 | 1.075 |
| sequence | 1.298 | 1.058 |

S1 and S2 fail both APIs. S3 fails with **19** significant case/ceiling
regressions (6 allocating, 13 caller-buffer), assessed against safe scalar Rust.
S4 passes caller-buffer and all frozen minima. Allocating S4 fails
`index-2-v0-streaming-s4`, `index-3-v0-streaming-s4` and
`index-3-v1-streaming-s4` against the unchanged 1.50 case bar.

| Bar | Allocating GM | Caller-buffer GM | Verdict |
|---|---:|---:|---|
| S1 | 1.618 | 1.778 | fail both APIs |
| S2 | 1.484 | 1.500 | fail both APIs |
| S3 | — | — | 19 significant case/ceiling regressions |
| S4 (scalar C++) | 1.238 | 1.062 | allocating fail; caller-buffer pass |

[SIMD_PERFORMANCE.md](SIMD_PERFORMANCE.md) retains every current case, interval,
pair count and independently assessed S3/S4 result. See
[P07_DIAGNOSIS.md](P07_DIAGNOSIS.md) for the epoch reconciliation, counters
and ranked structural fix plan. No upstream-parity or release claim is made.

The following wasm table belongs to the original epoch; current round-three
S5 means are 1.310 / 1.361, with worst stage-1 ratios 2.537 / 2.740.


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

## P07-perf refresh — 2026-10-05

Current evidence is retained in `/mnt/linux-extra/meshopt-artifacts/p07perf`;
the original `/mnt/linux-extra/meshopt-artifacts/p07` epoch is preserved.
Upstream, the 138 frozen benchmark inputs and `SIMD_BAR.md` are unchanged.
P07-P1/P2 in DECISIONS record the profile evidence, implementation choices,
rejected estimates/prefetch and maintenance tradeoffs.

The frozen candidate passes all-feature/scalar tests, the unsafe-free std
tests, MSRV 1.88, native/AArch64/wasm Clippy, x86/wasm no_std SIMD checks,
unsafe-free wasm, rustdoc, formatting and repository/package boundary checks.
The current audit covers 20 individually documented blocks and one allowance.
Miri passes four native SIMD integration tests, two integer-kernel tests and
eight scalar codec tests. The ordinary vertex matrix stays complete; only
Miri's matrix uses the bounded group/block-boundary subset described in P07-P2.

Final canonical comparisons pass 869 fixtures, 7,653 malformed cases,
22,000 seeded cases and 138 benchmark inputs under both APIs and every
supported native ceiling, plus executed wasm with/without simd128. The
independent verifier checks every archived input/output hash. Fresh checks of
all 87 shipped-JS eligible inputs preserve destination tails and meet the
separate upstream one-unit Oct/Quat conformance rule; other outputs are exact.
Native and wasm exhaustive checks again cover all 2^32 sqrt/Exp patterns,
2^24 Oct8 inputs, 10^8 seeded Oct16/Quat records each and edge combinations.

The final combined ASan smoke exits 0 after 573,532 executions in 301.288 wall
seconds and 289.093 process CPU seconds, with no crash artifacts. Checkpoint
lower bounds are 567,296 calls each at scalar/SSE2/SSSE3/SSE4.1. This is a
bounded smoke, not fulfillment of the per-target release CPU-hour budgets.
The remaining platform, release sweep/fuzz and phase-0.6 composition records
listed above remain unestablished by this local refresh.

### Final performance and queue policy

The owner replaced the lease-free/scoreboard gate with fair shared-lease
queueing during this run. The three completed native rows from the earlier
gate are preserved in `performance-pre-queue.json`; none is repeated.
All subsequent timed work runs through the specified Moss `gpu-lease.sh run
meshopt-timing:p07 -- ...`, with a 3,600-second queue timeout and retry on 75.
The leased native/Node adapters use the original numerical controllers, retain
completed keys and unfinished samples, and verify that the lease holder is an
ancestor of the measurement process. They checkpoint at a pair boundary after
690 seconds; a separate 840-second command limit bounds each lease. There is
no scoreboard check under the owner's replacement policy.

The one full native matrix completes 276 unique rows; Node completes 174.
Native holds its lease for 504.318 seconds, Node for 82.754 seconds. The
separate S4 borderline supplement holds its lease for about two seconds.
Each final matrix and timed supplement command exits 0. Native uses seven fresh D146 cases, Node
six; S4 needs twelve additional stage-1 pairs and no second stage. Raw samples,
lease-holder telemetry, queue receipts, and exact executed controller hashes
are retained. The unchanged controls remain in the final matrix as the owner
confirmed. All frozen index/sequence throughput minima pass.

Family geometric means of stage-1 Rust time / C++ time, allocating / caller
buffer; before/after are descriptive matched-backend epochs on this shared
host, not a dedicated-host A/B experiment:

| Function | Before | Final |
|---|---:|---:|
| vertex | 2.283 / 2.610 | 2.065 / 2.115 |
| view-none | 2.121 / 2.583 | 1.994 / 2.234 |
| view-filtered | 2.255 / 2.617 | 1.799 / 1.690 |
| Oct, varied | 2.662 / 2.668 | 1.538 / 1.557 |
| Quat, varied | 2.607 / 3.218 | 1.349 / 1.339 |
| Exp | 0.951 / 0.970 | 0.950 / 0.947 |
| Color | 1.511 / 1.492 | 1.508 / 1.547 |
| meshlet | 2.296 / 3.115 | 1.901 / 2.401 |
| meshlet-raw | 1.830 / 2.181 | 1.726 / 2.031 |
| index | 1.154 / 1.061 | 1.161 / 1.030 |
| sequence | 1.094 / 1.082 | 1.268 / 1.159 |
| Node S5, all eligible cases | 2.457 / 2.769 | 1.330 / 1.354 |

| Bar | Allocating GM | Caller-buffer GM | Current verdict |
|---|---:|---:|---|
| S1 | 1.984 | 2.204 | fail both |
| S2 | 1.496 | 1.451 | fail both |
| S3 | — | — | 67 significant case/ceiling regressions |
| S4, scalar C++ | 1.197 | 1.104 | allocating fail; caller buffer pass |
| S5, shipped JS SIMD | 1.330 | 1.354 | fail both; 19 / 22 failed maxima |

S4 allocating misses `index-2-v1-streaming-s2` (paired interval 1.531–1.823)
and `index-2-v1-streaming-s4` (1.570–1.739), against the unchanged 1.50 case
bar. S3 splits into 33 allocating and 34 caller-buffer comparisons; both the
default and SSE2 ceilings are assessed independently. These failures remain
failures. The current individual record is [SIMD_PERFORMANCE.md](SIMD_PERFORMANCE.md).
P07-P3 in DECISIONS records profile evidence and the best observed ratios for
the remaining gaps. The lane meets the brief's residual-evidence Done-when;
it does not qualify an upstream-parity or release claim.

## P07-R3 refresh — 2026-10-05

Start `e6eac35`; implementation `4e7eea0`, shared-admission controller
`88c80f7`. Evidence lives at `/mnt/linux-extra/meshopt-artifacts/p07r3`.
Only native vertex byte groups/reconstruction and meshlet decoding change;
filters, index/sequence, encoders, wasm algorithms and frozen bars are unchanged.
The detailed upstream instruction comparison and rejected trials are recorded
in DECISIONS, P07-R3.

The private module now has **23 audited unsafe blocks**: the additional seams
are a typed eight-byte unaligned load and two token-authorized whole-meshlet
calls. Repository and published-package boundary gates pass. Formatting,
all-target/all-feature clippy, all-feature tests, unsafe-free tests with and
without std, MSRV 1.88, AArch64 and wasm clippy, wasm simd128 clippy, rustdoc
and no_std SIMD configurations pass. AArch64 execution is not claimed.

Miri passes five SIMD integration tests, three integer-kernel tests and eight
scalar-codec tests. x86 runs use
`RUSTFLAGS="-C target-feature=+ssse3,+popcnt,+sse4.1"`; no unsupported intrinsic
is encountered. The new meshlet regression normally covers all 256 pair codes,
odd tails, counter wrap, both widths and APIs. Its documented Miri subset
covers every nibble plus mixed reuse/restart orders. Byte-header regressions
cover all 256 headers in every format mode and strict lookahead boundaries.

Exact differential archives pass: 869 fixtures, 7,653 malformed cases,
22,000 seeded cases and 138 benchmark inputs, both APIs, scalar/SSE2/SSSE3/
SSE4.1 and executed wasm without/with simd128. Successful destination bytes,
error variants and successful caller-buffer tails match the canonical scalar
Rust path. Separate C++ SIMD filter conformance rules are unchanged.
The bounded ASan/libFuzzer run completes 611,462 executions in 301.428 wall
seconds / 250.634 CPU seconds, peak RSS 453 MB, with no finding. Checkpointed
lower bounds are 606,208 per supported native ceiling. This combined smoke
does not qualify any per-target release fuzz budget. Historical exhaustive
filter records are not rerun or credited as fresh round-three evidence.

The one final native matrix completes **276 unique API/case rows**; the one
Node matrix completes **174**. Native's first burst checkpoints at 153 rows;
a continuation retains four more before an admission-file read fault. The
repaired continuation completes the remaining 119 rows. Node's initial
bootstrap failure accepts zero samples; the prepared timing modules then
complete its full matrix in 4m53s. Every successful burst stays under 15
minutes. DECISIONS records the actual shared-queue order, rejected/zero-sample
adapters, one interrupted unaccepted pair, and preservation of accepted rows.
Numeric inputs, source kernels, calibration and stopping policy stay fixed.

| Family | Before allocating / caller-buffer | After allocating / caller-buffer |
|---|---:|---:|
| vertex | 2.065 / 2.115 | **1.832 / 1.967** |
| meshlet | 1.901 / 2.401 | **1.329 / 1.534** |
| meshlet-raw | 1.726 / 2.031 | **1.228 / 1.315** |

Ratios are geometric means of stage-1 medians, Rust time / C++ SIMD time;
lower is faster. The meshlet probe separately interleaves the starting binary
and observes new/old 0.580–0.635 on its touched cases, exactly five pairs each.
The final instruction/cycle captures retain source/binary/lease identities and
exact outputs for both Rust epochs and C++. Meshlet instructions drop to about
63% of starting Rust, with cycles at 57–59%. The remaining vertex gap lies
primarily in packed reconstruction/scatter: 60.54% of sampled cycles in the
resident v1 case, and roughly 2.49 times C++ instructions at stride twelve.
Checked/counting loops and three-byte pair output also remain in meshlets.
DECISIONS gives the per-input counters, branch counts and upstream comparison.

The owner moves all new timing to the visible shared admission wrapper,
`MOSS_HEAVY_GPU=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ...`.
No lease wrapper is nested. Actual reservation, four-GB declaration, scope cap,
ancestor wrapper and owned GPU holder are recorded separately. The scoreboard
exclusion remains active; a sleeping when-idle service is eligible. New helper
versions are archived by hash for earlier segments. Node arithmetic timing
binaries live separately from the unchanged six native/parity binaries, and
87 eligible inputs match canonical golden outputs in untimed preflight through
both timing modules and APIs. Its scalar comparator remains diagnostic lowering.

The independent round-three verifier passes archive members and hashes, both
complete matrices, unique row membership, recomputed intervals and early
stopping, fresh D146 eligibility, Node timing-module/golden identities, queue
proofs, residual profile outputs, Miri and static receipts. S4 adds twenty
pairs across three scalar-baseline borderline candidates, with no S4 D146.
Native/Node use sixteen/seventeen fresh D146 cases. Every frozen S4 minimum
passes; allocating S4 fails three case maxima and caller-buffer S4 passes.
S1/S2 and S5 remain unmet for both APIs; S3 records nineteen significant
case/ceiling regressions. Detailed unchanged-control results are descriptive,
not attributed to these two kernel changes. SIMD_PERFORMANCE.md carries the
full current tables, including failures. No failed case is rerun for selection.

Done-when is satisfied by the two optimized kernels, exact proof and documented
residual profiles/best ratios. This is not SIMD release, ARM execution,
Windows/macOS, dedicated-host, Moss integration or phase-0.6 qualification.
Retain source/binary archives and receipts outside the exact target; delete
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p07r3` at completion. No push.


## P07 fix-four — 2026-10-05

Start `e41f4bb`; each ranked fix is separately committed: `a48d060`,
`70727c3`, `72e03af`, `f72221b`, `b0f0377`, `eb6bac6`. Production source
is frozen before one scoped timing epoch. Evidence:
`/mnt/linux-extra/meshopt-artifacts/p07-fix4`. The independent verifier passes.
[Every scoped native/Node row, failures and residuals](P07_FIX4_PERFORMANCE.md).

| Family | Before allocating / caller-buffer | After allocating / caller-buffer | Registered family bar A/C | Brief bar A/C |
|---|---:|---:|---|---|
| vertex | 1.832 / 1.967 | **1.201 / 1.103** | FAIL / FAIL | FAIL / FAIL |
| view-none | 1.702 / 1.872 | **1.155 / 1.183** | FAIL / FAIL | pass / FAIL |
| view-filtered | 1.841 / 1.826 | **1.295 / 1.185** | FAIL / FAIL | FAIL / FAIL |
| color | 1.540 / 1.496 | **1.397 / 1.395** | FAIL / FAIL | FAIL / FAIL |
| meshlet | 1.329 / 1.534 | **1.332 / 1.564** | FAIL / FAIL | FAIL / FAIL |
| meshlet-raw | 1.228 / 1.315 | **1.182 / 1.299** | FAIL / FAIL | pass / FAIL |
| sequence | 1.298 / 1.058 | **1.237 / 1.078** | FAIL / pass | FAIL / pass |
| wasm touched scope | 1.360 / 1.426 | **1.151 / 1.156** | FAIL / FAIL | FAIL / FAIL |

Ratios are geometric means of stage-one medians, Rust / C++ SIMD time
(upstream shipped JS SIMD for wasm); lower is faster. Before uses identical
case membership from p07r3, a separate historical epoch. Only sequence
interleaves starting Rust in the new pass. Registered S1 is 1.10/1.30,
S2 1.25/1.50 and S5 1.25/1.60; the brief's uniform 1.25/1.50 is separately
assessed. SIMD_BAR.md and frozen numeric inputs are unchanged. Per-family
verdicts include both mean and final interval maxima; unresolved intervals fail.

Native completes **188 unique rows** (94 touched cases / both APIs); Node
completes **146** (73 eligible touched cases / both APIs). Native's two bursts
release after 11m34s and 1m05s; Node after 2m33s, all exit zero under the exact
four-GB heavy admission wrapper. First native burst checkpoints at 144 rows;
the continuation resumes its accepted unfinished samples and remaining rows.
No accepted pair/completed row is repeated. Native has 28 fresh thirty-pair D146
rows; Node has 17. Telemetry proves holder ancestry, actual reservation/cap,
declaration and scoreboard exclusion. Native resume drops only the first
fractional top-level burst receipt; original wrapper logs and timing-bursts.json
retain every whole-second duration and exit, with all per-pair receipts intact.

S1 and touched S2 remain FAIL for both APIs. **Touched S3 has zero significant
regressions**, for both default and SSE2 ceilings, including either stage;
the two historical standalone Exp caller-buffer failures remain unclosed.
Triangle index and standalone Oct/Quat/Exp timing are outside this scope.
Sequence-only S4 allocating is FAIL (scalar-C++ geomean 1.244, two maxima);
caller-buffer passes (1.073, no failed maxima). Every sequence P02 minimum
passes. Full S4 is not requalified. S5 geomeans now pass but maxima fail:
eight allocating and ten caller-buffer cases exceed registered 1.60 intervals.

The historical allocating sequence gap persists: streaming v1 stride-four
is 2.876× C++ (starting Rust in the same pass 3.205×, new/starting 0.897).
Differential instructions fall to about 0.808 starting Rust. Cold-packet
new Rust/C++ instructions are 1.045; Rust incurs fewer page faults, and both
drivers fault during initialization and response copying. Counter/fault stacks
do not establish kernel/allocator latency. Cause attribution is explicitly
abandoned within this bounded pass with raw evidence retained; no extra
admitted timing experiment or selection retry is performed.

Frozen exact comparisons pass: 869 fixtures, 7,653 malformed cases and all
138 benchmark identities, both APIs, native scalar/SSE2/SSSE3/SSE4.1 and
executed wasm without/with SIMD. Separate upstream SIMD filter conformance
rules are unchanged; 292 Node preflight output hashes match canonical bytes.
All-feature/unsafe-free tests, native/wasm SIMD Clippy, five SIMD integration
Miri tests, three integer-kernel Miri tests and sequence-fixture Miri pass.
MSRV 1.88 no-std SIMD and wasm simd128 no-std compile checks pass (existing
unused-helper warnings retained). Boundary/package gates retain 23 unsafe
blocks and one module-level allowance. Oct/Quat/Exp arithmetic is unchanged;
historical exhaustive filter runs are not credited as fresh evidence.

The optional expanded P02 sweep passes 70,000 cases. Expanded strict P04 is
**FAIL** after 99,264 completed cases at `seed-20261005-op20-9264`: mutated
unused odd-tail meshlet nibble 0x1c is accepted by upstream scalar/current
Rust/starting Rust with identical bytes, but upstream SIMD returns -3.
The pre-existing disagreement, full partial archive and reproduction are
retained without an input exclusion, oracle change or smaller green sweep.

Native vertex instructions drop to about half of starting Rust, and its
reconstruction stack frame falls 0x168→0x98. Checked address/spill work remains.
Wasm probes use about two-thirds of starting instructions, but V8 work moved
to the larger vertex decode body/helpers; raw-function size is insufficient
to claim whole-path spill reduction. Color instruction reduction preserves
exact arithmetic. Meshlet packing helps raw instructions; typed family wall
means remain slightly worse and fail. All failures and residuals are retained.
Further speculative filter arithmetic/unsafe pointer paths are abandoned
without proof. No late unmeasured threshold or revert is introduced.

All six fixes are implemented; sequence cause attribution is the explicit
unresolved subgoal. Scoped timing and result recording satisfy the stop
condition. This does not qualify SIMD release, ARM execution, dedicated-host,
Windows/macOS, Moss integration or phase-0.6 acceptance. Binaries/evidence
are archived outside the exact build target. The prescribed
`/mnt/linux-extra/moss-cargo-targets/codex-p07-fix4` (1.1 GB) is deleted;
cleanup.json confirms absence and disk receipts.
No push, merge or rebase.


## Fix-five scoped result — 2026-10-05

See [P07_FIX5_PERFORMANCE.md](P07_FIX5_PERFORMANCE.md) for every row and
source-bound proof/limitations. Registered bars and frozen inputs are unchanged.

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


Registered varied-filter S3 final significant comparisons: 2.
Source-bound verifier passes; numeric failures are retained. No broader
release/platform/integration acceptance is claimed. The historical codec
wasm SIMD flag shortfall is corrected and disclosed in the new report.


## Fix-six scoped result — 2026-10-05

See [P07_FIX6_PERFORMANCE.md](P07_FIX6_PERFORMANCE.md) for both verdict sets,
all rows, rejected trials and exact proof limits. Registered bars and frozen
inputs are unchanged. A/C means allocating/caller-buffer.

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

Final native varied-filter S3 significant comparisons: 4.
Independent source-bound verification passes; numeric failures remain failures.
No broader release, platform or integration acceptance.

## Fix-seven scoped RFC result — 2026-10-05

See [P07_FIX7_PERFORMANCE.md](P07_FIX7_PERFORMANCE.md) for every row, all 37
previously unresolved/failed maxima, paired old controls and proof limits.
Owner RFC thresholds are geomean <=1.25 and maximum <=1.50 per family/API.
The registered stricter bar is deferred to 0.3; SIMD_BAR.md is unchanged.
A/C = allocating/caller-buffer. A failed final maximum or applicable S3
comparison prevents the family/API verdict from passing.

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

Final native varied-filter S3 significant comparisons: 1.
Independent source/binary/control-bound verification passes; numeric failures
remain failures. One final epoch, with checkpointed continuations of unaccepted
work only. No late source tuning or rescue timing. Exact proof and all test,
Clippy, Miri, MSRV/no-std and unsafe-boundary/package gates pass.
This does not establish release, ARM, other-host or Moss integration acceptance.
The prescribed fix-seven target is deleted after collecting final evidence.
No push, merge or rebase.

## P07 fix-eight — source candidates, disk gate closed

See `parity/P07_FIX8_PERFORMANCE.md` for first-analysis counters, explicit
kill decisions, candidate commits and the pending RFC table. Exp's native
shared scalar lowering and Oct's exact hardware short-tail root are source
candidates only. The first build declined before Cargo because free disk is
below 25 GiB. Ordinary/Miri/parity and after-counter gates have not executed;
no final timing pair exists. The same 23-block static unsafe boundary and
formatting pass. Retained control/source/frozen-archive hashes are checked,
without promoting them to changed-source runtime qualification. Native
allocating vertex/view and all WASM view residuals are abandoned under the
first-analysis no-identified-cause kill rule. RFC closure remains unestablished.

## Fix-eight continuation final paired epoch

Exp/Oct are now validated, disk-derived abandonments voided, WASM fixed-stride reconstruction retained. Overall RFC closure remains unestablished. Full receipts and failed rows: [P07_FIX8_PERFORMANCE.md](P07_FIX8_PERFORMANCE.md).

| Platform / family | A mean / max upper / RFC | C mean / max upper / RFC | Paired new/old A / C |
|---|---|---|---|
| native / exp | 0.942 / 1.403 / pass | 0.945 / 1.392 / pass | 0.977 / 0.989 |
| native / oct | 0.958 / 1.538 / **FAIL** | 0.914 / 1.556 / **FAIL** | 0.945 / 0.948 |
| native / view-filtered | 1.084 / 2.316 / **FAIL** | 0.965 / 1.485 / pass | 0.998 / 0.969 |
| wasm / vertex | 0.989 / 2.065 / **FAIL** | 0.977 / 1.498 / pass | 0.877 / 0.851 |
| wasm / view-none | 1.140 / 2.148 / **FAIL** | 1.002 / 1.465 / pass | 0.865 / 0.879 |
| wasm / view-filtered | 0.996 / 1.754 / **FAIL** | 0.979 / 1.747 / **FAIL** | 0.814 / 0.756 |

Native vertex A retains prior failed evidence1.538–2.847 after fresh built-code first analysis.

- caller-buffer / varied-filter-3-tiny-s32: 1.0390–1.1085.
