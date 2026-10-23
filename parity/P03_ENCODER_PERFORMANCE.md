# Encoder performance

## SPEC-03 finish: CI fixed; final performance blocked

**Final-source performance acceptance: UNVERIFIED.** Source candidate `8319f7d3624694f7f71f2fe1f45ff977c10e17d9` passes the exact local CI test job on Rust 1.99.1 and both MSRV commands on 1.88. Its [pushed source CI is green](https://github.com/Tavrin/meshoptimizer-rs/actions/runs/40001341141). The final documentation-only commit preserves measured source/consumer hashes. The PR remains draft.

The retained `seen-frontier/build.sh` rebuilt every binary and regenerated source guards and hashes. Both profiles have 17,264 exact parity comparisons, 369,098,752 exhaustive Exp component comparisons, exponent/fallback/zero-carry and capacity checks with zero mismatches. The only new source edit is a regression-test loop using `as_chunks::<4>()`; no encoded path, API, threshold or CI workflow changed.

Final measurement was invoked in the prescribed order through `measure.sh`, headlessly, without an outer heavy scope or quiet-machine wait. No borrowing was allowed. The first historical-request burst remained queued: an earlier delivery-gate hold released, but a renewed hold for train `97762443c8` blocks the lane until 23:49:50 CEST on 2026-10-23. The still-unstarted attempt was stopped; zero timing samples were collected, no completed stream was repeated and no interrupted measured case was discarded. This external admission blocker prevents completion of SPEC-03-finish. The four-lane-day cap is not declared exhausted.

| Family | Cargo defaults final GM / case CI | Moss final GM / case CI | GM ≤1.25 | Every resolved upper95 ≤1.50 |
|---|---|---|---|---|
| vertex | Not measured | Not measured | Unverified | Unverified |
| index | Not measured | Not measured | Unverified | Unverified |
| sequence | Not measured | Not measured | Unverified | Unverified |
| oct | Not measured | Not measured | Unverified | Unverified |
| quat | Not measured | Not measured | Unverified | Unverified |
| exp | Not measured | Not measured | Unverified | Unverified |

The exact historical owned request and complete shared-peer refresh are also unmeasured on this candidate. No final intervals or pass/fail performance verdict can be reported. Earlier `a6fc95f` numbers and the pre-CI scout are **superseded for final acceptance**, preserved below as historical evidence only. All bars remain unchanged: family GM ≤1.25, every resolved case upper95 ≤1.50 in both profiles; unresolved A/A controls block qualification. The separate peer-family target remains ≤1.00.

Fresh build, registration, parity and per-burst queue receipts: `/mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier`. Previous guarded epoch: `seen-frontier-pre-ci-0f04876`. Blocker and exact local gates: `/mnt/linux-extra/moss-scratch/meshopt-enc03/finish`. A redundant extra MSRV compile check was canceled while queued after both required MSRV test commands had passed; native tests/lint and both capacity matrices had completed. Local wasm compilation was not repeated by that extra controller; pushed source CI's wasm job passed. The Cargo target was below 10 GB and deleted, leaving all retained binaries available.

[Resume command and controls](encoders/ENC03_HANDOFF.md), [portable summary](results/enc03-allocating-profiles.json), [compressed raw receipts](results/enc03-allocating-profiles.json.gz). Not verified: final native/peer performance, downstream Moss runtime/cook, GPU/browser execution, release fuzz budgets or universal speed. Reversal cost: reverting one test-loop line restores the CI lint failure; historical receipts cannot qualify changed source.

## Superseded pre-finish snapshot

The following snapshot records the previous handoff and its earlier numbers. Its work-mode and pending-status statements describe that earlier handoff, not the current OFF-but-admission-blocked state.

## SPEC-03: allocating encoder candidate; final timings pending

**Final-source performance acceptance: PENDING.** Work mode was enabled before this candidate’s full native and peer confirmation streams began. No further performance run was started. Correctness work is complete. The four-lane-day cap was not exhausted; the spec’s performance done-when condition has not been established. This branch is a draft handoff, not a qualified release.

Ratios below are Rust elapsed time / comparator elapsed time; lower is faster. Cargo defaults means O3, no LTO, 16 codegen units; Moss means O3, thin LTO, one codegen unit. Native target: each allocating family GM ≤1.25 and every case upper95 ≤1.50 in both profiles, with all A/A controls resolved. Shared peer target is separately registered at each family GM ≤1.00.

### Last completed full native qualification — earlier source only

The following paired before/after stream belongs to `qualified2`, source `a6fc95fed7b0e4881c50127af20ca22aae00b58e`, before the final Exp mask and index frontier changes. Before is released source `d9395b92737ed012fef9a24e28ea42b4886b1db1`. All 111 allocating rows per profile resolved and met the native bars in that earlier stream. These numbers **do not qualify the final source**.

| Profile | Family | Before → earlier candidate GM / C++ 1.3 | Before case CI envelope | Earlier candidate case CI envelope | Final source |
|---|---|---:|---:|---:|---|
| defaults | vertex | 0.663 → 0.667 | 0.511–0.954 | 0.508–0.963 | pending |
| defaults | index | 1.025 → 1.040 | 0.817–1.233 | 0.796–1.284 | pending |
| defaults | sequence | 0.953 → 0.948 | 0.890–1.367 | 0.887–1.456 | pending |
| defaults | oct | 0.583 → 0.583 | 0.172–1.482 | 0.171–1.497 | pending |
| defaults | quat | 1.057 → 0.705 | 0.210–4.931 | 0.211–1.467 | pending |
| defaults | exp | 0.601 → 0.617 | 0.278–1.094 | 0.278–1.152 | pending |
| moss | vertex | 0.661 → 0.664 | 0.449–0.904 | 0.502–0.926 | pending |
| moss | index | 1.020 → 1.039 | 0.855–1.207 | 0.833–1.253 | pending |
| moss | sequence | 0.985 → 0.925 | 0.911–1.094 | 0.860–1.052 | pending |
| moss | oct | 0.570 → 0.567 | 0.155–1.455 | 0.157–1.428 | pending |
| moss | quat | 0.673 → 0.674 | 0.183–1.498 | 0.185–1.411 | pending |
| moss | exp | 0.589 → 0.593 | 0.278–1.033 | 0.279–1.065 | pending |

Intervals in family tables are envelopes of individual nominal paired-log 95% intervals, **not family-GM confidence intervals**. Case points are medians; case intervals estimate mean log ratios. Every frozen case is included.

### Final candidate diagnostic scout — not qualification

The final `seen-frontier` candidate completed a preplanned two-operation scout before work mode. The independently registered full 40-row peer matrix in each profile is still pending. Its confirmation stream, rather than the better of scout/confirmation, must determine acceptance.

| Profile | Family | Released GM / peer | Final scout GM / peer | Scout case CI envelope |
|---|---|---:|---:|---:|
| defaults | index | 1.107 | 0.852 | 0.547–1.152 |
| defaults | exp | 2.711 | 0.548 | 0.218–0.730 |
| moss | index | 1.072 | 0.853 | 0.651–0.984 |
| moss | exp | 2.682 | 0.576 | 0.516–0.749 |

Peer before and scout are separate refreshes, not paired intervention intervals. Each scout family has five frozen geometry cases; all output bytes match. Peer meshopt 0.6.2 bundles C++ 0.25, not the canonical C++ 1.3 oracle. The existing peer adapter, allocation and known-vertex-count workload are unchanged.

For completeness, the last full peer refresh also belongs to earlier source `a6fc95f`; it missed the strict index and Exp targets and motivated the final candidate:

| Profile | Shared family | Released → earlier candidate GM / peer | Earlier case CI envelope | Final full refresh |
|---|---|---:|---:|---|
| defaults | vertex | 0.715 → 0.715 | 0.554–0.929 | pending |
| defaults | index | 1.107 → 1.128 | 0.932–1.275 | pending |
| defaults | sequence | 0.983 → 0.982 | 0.919–1.011 | pending |
| defaults | oct | 0.872 → 0.870 | 0.795–1.112 | pending |
| defaults | quat | 1.904 → 0.884 | 0.801–1.276 | pending |
| defaults | exp | 2.711 → 1.304 | 1.253–1.344 | pending |
| defaults | color | 0.821 → 0.829 | 0.791–0.936 | pending |
| moss | vertex | 0.741 → 0.739 | 0.577–0.955 | pending |
| moss | index | 1.072 → 1.125 | 0.941–1.267 | pending |
| moss | sequence | 1.012 → 0.949 | 0.901–0.993 | pending |
| moss | oct | 0.872 → 0.874 | 0.812–1.096 | pending |
| moss | quat | 1.523 → 0.883 | 0.797–1.265 | pending |
| moss | exp | 2.682 → 1.297 | 1.262–1.326 | pending |
| moss | color | 0.828 → 0.828 | 0.782–0.874 | pending |

Vertex combines matched v0/v1 rows without double-weighting the family. Color is an additional shared encoder outside the six native families. Per-family misses cannot be hidden by a cross-family aggregate.

### Original owned request: not reproducible

Exact `vertex-v1-streaming-s4` request SHA-256 `13413c1e2c69f3de2f28a32005650fb46dfac1f92621cce393acefb3929040e9`, 2,097,153 records. The historical slowdown did not reproduce with symmetric calibration, including a separate pre-staging build (`eb5b1a2df97c6607123befc45bc830e2aeff2a76`):

| Profile | Pre-staging median / C++ | Nominal 95% CI | Released median / C++ | Nominal 95% CI |
|---|---:|---:|---:|---:|
| defaults | 0.868 | 0.814–1.038 | 0.868 | 0.792–1.044 |
| moss | 0.869 | 0.805–1.020 | 0.865 | 0.810–0.994 |

This is non-reproduction on the current toolchain, not evidence that staging or these encoder changes caused a fix. The decoder implementation is unchanged. Initial asymmetric-calibration A/A failures remain inconclusive and retained. Final-source repeat of the request is pending with the final native matrix.

### Implementation and correctness

- Special-case exactly `sqrt(2)` in the shared helper using its pinned rounded constant; other inputs retain libm. Tiny Oct/Quat function definitions remain byte-identical. Other geometry throughput is unmeasured.
- Encode a strictly dominant quaternion prefix in four-record blocks with constant rotation; ties, selector changes and unordered comparisons use the original bulk kernel for the remaining suffix. Tiny routing is unchanged.
- Encode ordinary stride-16 Separate Exp values using exact integer mantissa rounding for precisions 1..22. Per-component zeros inherit the previous exponent. Unsupported exponent/precision domains restart the original general kernel. Integer masks remove component eligibility branches without changing the mathematical domain.
- Skip a vertex FIFO search only when the queried index exceeds every real value inserted since reset. Preserve observable MAX sentinels and original cyclic lookup, storage and updates. A reset clears the bound; it can overestimate without rejecting a possible hit. Retain original triangle version dispatch/varints. Common one-byte sequences write directly.
- Preserve allocation initialization, fallible reservation, limits, work accounting and error order. Add budget-boundary, zero-carry golden and FIFO equivalence tests. No public API, parallelism, unsafe code or manual SIMD expansion.

Both final profiles pass 17,264 scalar/SIMD C++ parity comparisons with zero mismatches, 3,456 exponent-edge checks, 1,424 coherent-rotation/fallback checks, 386 zero-carry checks and exhaustive comparison of all 2^23 mantissas, both signs, all 22 precisions (369,098,752 components per profile). Full-u32/capacity checks cover 7,122 requests per profile, including 144 explicitly skipped upstream signed-negation undefined domains. Default crate tests, no-default codec/codec04 tests, strict library Clippy, formatting, Rust 1.88 no-std, wasm32 no-std compile and the unsafe-boundary check pass.

### Measurement, evidence and pending work

AMD Ryzen 9 7945HX, Linux x86-64, performance governor; Rust 1.98.1, generic CPU, GCC 13.3. Canonical C++ 1.3 pin `4c203430ca565cb59a468a91922c76c208169536`; scalar-strict canonical bytes and normal runtime SIMD for timing. No fast-math or FP contraction in the oracle. Source, consumer, binary and request hashes bind each epoch independently of a dirty build’s Git label.

All Cargo and timing commands use CPU-only `moss-heavy.sh`, the fixed target and disabled incremental compilation. Native streams pin CPU26 and rotate/reverse interleaved candidate, C++, duplicate A/A and released-source arms, each calibrated symmetrically to 25 ms. Registered sampling uses five complete pairs or exactly 256; no intermediate CI decisions. A/A intervals must lie inside [0.8,1.25]. Beyond 20 pairs the conservative 20-pair t critical value is retained. Intervals are nominal, without simultaneous/sequential adjustment. Bursts checkpoint after 690 seconds with an 840-second timeout. Peer retains its pinned interleaved 5-or-20-pair protocol. No quiet-machine waiting. Completed qualification streams are immutable; prior failed/unresolved epochs remain separate.

Native allocating C++ index/sequence now derives the same range bound inside the timed call, including the OR scan. Earlier mismatched worst-case allocation-bound epochs remain legacy evidence. Caller-buffer capacity semantics and published peer work are unchanged. C++ 0.25 and 1.3 differ on Exp zero-exponent carry; all selected frozen peer outputs still match. Broader cross-version identity is not claimed.

Wasm execution, other architectures, downstream Moss cook/runtime, release fuzz budgets, general geometry performance, and final-source native/complete-peer performance are not verified. Rejected FIFO, version specialization, wrapper inlining/precharge and generalized-arithmetic experiments remain in the scratch milestone ledger. The target was below 10 GB and deleted after correctness work; copied binaries, fixtures and notes remain available.

Exact pending commands and controls: [ENC03_HANDOFF.md](encoders/ENC03_HANDOFF.md). Run only after work mode is explicitly off. Portable [summary](results/enc03-allocating-profiles.json) and [raw receipts](results/enc03-allocating-profiles.json.gz) distinguish final correctness/scouts from earlier qualification. See [harness policy](encoders/README.md).

---


---

# Historical: 0.3 round 1

**Performance acceptance: FAIL — tiny Oct/Quat maxima.** All 224 native rows
resolve under the registered A/A rules. Every family mean is <=1.25, but four
tiny rows exceed or cross the required maximum upper95 <=1.5. No completed
stream was repeated and no implementation changed after timing began.

The exact allocating streaming exception passes the maximum bar in this
consumer profile; its matched baseline also passes. This is not evidence that
the historical Cargo-default/Moss-profile gap was removed by staging.

## Matched allocating means

Ratios are Rust/comparator; lower is faster. These are geometric means of
paired per-row median ratios over the same resolved corpus, before eb5b1a2
and after 3c0ec28, using identical corrected consumers.

| Encoder | Rows | C++ 1.3 before → after | meshopt 0.6.2 before → after | Max upper95 vs 1.3 | Bar |
|---|---:|---:|---:|---:|---|
| vertex | 62/62 | 0.904 → 0.658 | 0.909 → 0.662 | 0.934 | PASS |
| index | 14/14 | 1.027 → 0.920 | 1.054 → 0.946 | 1.237 | PASS |
| sequence | 14/14 | 1.228 → 0.967 | 1.229 → 0.970 | 1.065 | PASS |
| oct | 6/6 | 1.107 → 0.539 | 1.120 → 0.541 | 1.556 | FAIL max |
| quat | 3/3 | 0.787 → 0.673 | 0.763 → 0.650 | 1.521 | FAIL max |
| exp | 12/12 | 0.949 → 0.607 | 1.002 → 0.640 | 1.089 | PASS |

## Caller-buffer means

| Encoder | Rows | C++ 1.3 before → after | meshopt 0.6.2 before → after | Max upper95 vs 1.3 | Bar |
|---|---:|---:|---:|---:|---|
| vertex | 62/62 | 0.903 → 0.644 | 0.911 → 0.650 | 0.905 | PASS |
| index | 14/14 | 1.057 → 0.933 | 1.090 → 0.960 | 1.221 | PASS |
| sequence | 14/14 | 1.257 → 0.933 | 1.300 → 0.961 | 0.993 | PASS |
| oct | 6/6 | 1.118 → 0.511 | 1.131 → 0.518 | 1.532 | FAIL max |
| quat | 3/3 | 0.788 → 0.691 | 0.795 → 0.695 | 1.743 | FAIL max |
| exp | 12/12 | 0.912 → 0.560 | 0.960 → 0.594 | 1.049 | PASS |

## Maximum ledger

| Row | API | Median after/C++ | Nominal 95% interval | Result |
|---|---|---:|---:|---|
| `oct-tiny-s8` | allocating | 1.517 | 1.502–1.556 | resolved maximum failure |
| `oct-tiny-s8` | caller | 1.515 | 1.493–1.532 | maximum not certified (CI crosses 1.5) |
| `quat-tiny` | allocating | 1.495 | 1.475–1.521 | maximum not certified (CI crosses 1.5) |
| `quat-tiny` | caller | 1.663 | 1.543–1.743 | resolved maximum failure |

Quat allocating has a point maximum of 1.495, below 1.5; its upper bound is
1.521, so the stricter registered confidence rule does not certify it. Oct
caller also straddles the limit. Oct allocating and Quat caller have intervals
entirely above 1.5. All four pass A/A resolution. There are zero excluded rows.

## Exact historical streaming request

SHA-256 `13413c1e2c69f3de2f28a32005650fb46dfac1f92621cce393acefb3929040e9`;
2,097,153 records, stride4, frozen encoded v1. No similarly named replacement.

| API | Before/C++ → after/C++ | Before/meshopt → after/meshopt | After/C++ 95% interval | Maximum bar |
|---|---:|---:|---:|---|
| allocating | 0.952 → 0.958 | 0.930 → 0.904 | 0.797–1.140 | PASS |
| caller | 0.964 → 0.995 | 0.855 → 0.881 | 0.980–1.017 | PASS |

The allocating point ratio changes from 0.952 to 0.958: this stream establishes
no attributable elapsed gain from staging. Retain the initialized safe path,
its exact parity and counter/locality receipts, without claiming that it closed
the earlier 1.529–2.118 historical failure. Reproduction in the original
consumer profiles remains a separate qualification task.

## Method and retained identities

Native AMD Ryzen 9 7945HX, CPU26, performance governor and boost enabled. Rust 1.98.1 release/fat LTO/one codegen unit,
generic CPU and unchanged production validation/accounting. C++ 1.3 is pinned
to `4c203430ca565cb59a468a91922c76c208169536`, GCC 13.3.0, -O3, no fast math
or contraction; the timing oracle enables its normal runtime SIMD dispatch.
The peer adapter uses public meshopt 0.6.2 FFI (bundled 0.25) in the same Rust
release profile; its C++ archive is compiled by cc at Cargo release opt-level3.
These comparisons include checked Rust API work and wrapper/compiler/version
differences; they do not isolate a language-only cost.

112 frozen requests, both APIs: common strides, v0 and v1 levels0–3/default2,
three synthetic sizes, all Exp modes, archived medium smooth/seamed/sparse
and million-smooth cook inputs, plus the exact decoder exception. Every vertex
level0–9 and both versions are additionally covered by seeded exact parity.

5–20 rotating/reversing interleaved pairs, ~8ms batch calibration from C++,
1,229 paired rounds in total. Independent Rust and C++ duplicate processes
must each have nominal 95% log-ratio intervals inside [0.8,1.25]. Family mean
<=1.25 and each row upper95<=1.5. Student log-ratio intervals are nominal
under the registered early stopping. All samples, including failures, remain
in the machine receipt. The final stream took an admitted 7s prefix followed
by an 85.5s suffix; both CPU-only commands used timeout840.

The first stream stops before row45 samples because its peer validator wrongly
requires 0.25 Exp Separate equality. Read-only diagnosis verifies optlog2(0)=0:
0.25 resets a zero exponent while 1.3 inherits it. A strict predicate accepts
only input-zero fields with both mantissas zero and the exact old exponent;
all nonzero words remain equal. Vertex version differences are independently
lossless cross-decoded. Every final arm validates before suffix resumption.
The immutable first 44 rows and a hash-checked validation-only amendment prove
that no binary, input, threshold, sampling rule or completed sample changed.

Unused caller destinations in allocating requests and unused input conversions
in the peer adapter were removed before measurement. Both original/final
libraries use the identical final consumer. Original diagnostic binaries stay
retained; historical/default-profile measurements are not substituted into
this before/after table.

## Correctness and safety

17,264 exact comparisons for each final primary consumer against scalar/SIMD
1.3; levels0–9, both APIs and varied layouts/precision. Supplemental parity
has 6,978 defined-domain/full-u32/capacity rows plus 144 explicit no-oracle
rows for upstream INT_MIN negation; Rust before/after bytes stay unchanged
there, and the existing observed-GCC golden remains. 580 owned-sink replays
cover scalar/SSE2/SSSE3/SSE4.1, both stream versions, threshold boundaries and
malformed tails. All 224 timing arms and every timed output are validated.

19 codec tests, 9 SIMD tests, staging unit and focused scalar Miri pass.
Rust1.88 no-default and WASM no-default compile checks, strict library Clippy,
formatting, repository boundary and published-package boundary pass. The
production unsafe inventory remains 23 blocks in the existing private module.
No new production unsafe code. The unpublished old-peer FFI harness is
excluded from packaging and has the same narrowly named audit exception as
the existing compare harness. WASM compilation is not executed WASM parity;
remote/native architectures and downstream cook/runtime acceptance were not run.

The initial undefined-domain, lint, dirty-package-list and peer-validation
failures remain archived separately from subsequent passes. Rejected ordinary
snorm casts and the index unrolled varint writer remain recorded in
[DECISIONS.md](DECISIONS.md); counter diagnosis precedes every elapsed sample.

## Evidence

[Machine receipt, every sample and identity](results/p03-encoders-round1.json),
[instruction diagnosis](P03_ENCODER_DIAGNOSIS.md),
[decisions](DECISIONS.md), and [reproduction harness](encoders/run.py).

Durable directory: `/mnt/linux-extra/moss-scratch/meshopt-v03`. It retains
all original/final/candidate binaries, full source copies, upstream sources,
frozen input files, raw PMU/disassembly, pair samples, failed attempts and
admission receipts. The committed receipt binds raw files by SHA-256. The
named owned target used 154,753,713 bytes (<5 GiB) and was deleted after evidence retention. Its cleanup receipt confirms no live owned jobs.

Next diagnosis: reduce tiny Oct/Quat setup cost and reproduce the historical
streaming consumer profile. No further implementation edit or elapsed rerun
is part of this completed stream.

## Round 2 — SPEC-p03-tiny (2026-10-06)

**Tiny maximum acceptance: PASS.** All four required rows pass the registered upper95 <=1.5 rule, then all 18 Oct/Quat confirmation rows pass with both independent A/A intervals inside [0.8,1.25]. The earlier round 1 results and the failed round 2 attempt remain retained. Other encoder verdicts remain the round 1 verdicts; their elapsed rows were outside this spec and were not rerun.

Ratios are Rust/C++ 1.3. Before below is the published round 1 ledger; the receipt also retains matched baseline samples in each new stream. Each stream used the same frozen inputs and generic release/fat-LTO/one-codegen-unit consumers, CPU26, nominal Student log-ratio intervals and the unchanged 5–20-pair stopping rule.

| Row | API | Round 1 median (95%) | Tiny qualification median (95%) | Full confirmation median (95%) | Verdict |
|---|---|---:|---:|---:|---|
| `oct-tiny-s8` | allocating | 1.517 (1.502–1.556) | 1.417 (1.373–1.436) | 1.424 (1.342–1.461) | PASS |
| `oct-tiny-s8` | caller | 1.515 (1.493–1.532) | 1.387 (1.378–1.391) | 1.384 (1.354–1.422) | PASS |
| `quat-tiny` | allocating | 1.495 (1.475–1.521) | 1.376 (1.362–1.387) | 1.379 (1.359–1.401) | PASS |
| `quat-tiny` | caller | 1.663 (1.543–1.743) | 1.398 (1.376–1.416) | 1.401 (1.399–1.405) | PASS |

The four-row qualification took 0.420s; full confirmation took 4.719s. Every row stopped at five pairs. Full confirmation retains all 18 rows and validates every timed output; no row was excluded. The first failed implementation was changed on counter/disassembly evidence before a new immutable stream, rather than repeating its samples. The mandated full confirmation is the only repeat of the final implementation.

| API/family | Rows | Confirmation geometric mean | Maximum upper95 | Verdict |
|---|---:|---:|---:|---|
| allocating/oct | 6 | 0.622 | 1.461 | PASS |
| allocating/quat | 3 | 0.789 | 1.401 | PASS |
| caller/oct | 6 | 0.555 | 1.422 | PASS |
| caller/quat | 3 | 0.751 | 1.405 | PASS |

All family means stay <=1.25. In the matched confirmation stream, Quat allocating/caller means move from 0.747/0.707 to 0.789/0.751: this is a measured mean slowdown within the registered bar, retained without disguising it as a family gain. No other Oct/Quat row changes its PASS verdict.

### Cause and retained fix

The shared runtime filter dispatch reserves a 0x288-byte kernel frame and keeps unrelated setup in the tiny call. Zero-record caller probes measured 281/299 Rust instructions for Oct/Quat versus 107/105 C++; the specialized checked paths reduce these to 241/211. Validation, budget accounting, initialized allocation and tail preservation stay in the same shared helpers; only internal kernel closures specialize the Oct/Quat call.

Oct retains four-record arithmetic and uses direct integer repair of the bounded magic-float quantizer for <=32 records. Quat uses a separate <=32-record loop and a non-inlined per-record helper: it hoists scale setup, performs fixed cyclic rotations after the original first-strict-maximum selection, and limits live selector/swizzle state to one record. Conversion to i16 is exact because the original clamped float expression is finite and within [-32767.5,32767.5], whose truncated results fit i16. The original multiplication/rounding order is preserved. Bulk arithmetic stays unchanged; no unsafe code or public API changes were added.

Complete allocating/caller instruction counts (N=10,000/20,000 subtraction, 100% PMU scheduling): Oct 1872/1635 -> 1705/1465; Quat 2339/2102 -> 2337/2109. Quat wins elapsed time through shorter dependence/live-range costs despite nearly unchanged instruction counts. PMU cycles are diagnostic, not acceptance or a timer-floor claim.

### Failed attempts and correctness

Retain scalar Oct candidates (2031 and 2170 caller instructions, rejected), vector tiny Quat repair (2084, insufficient cause reduction), fixed rotations in a cross-record loop (2182, rejected), IEEE variable-shift truncation (2488, rejected), and the isolated magic-float record helper (2381). Two scalar Oct failures triggered the read-only follow-up in READONLY-AFTER-TWO.md. The isolated i16 record helper then reduced dependence cost and passed elapsed qualification. Every compiled candidate passed 17,264 exact scalar/SIMD-oracle comparisons, including both APIs, filter precisions and vertex levels0–9 in both versions.

The first elapsed attempt passes Oct but fails Quat allocating 1.657 (1.653–1.664), caller 1.664 (1.658–1.671). Its complete samples, source/binary hashes and A/A controls remain in the receipt. Initial launcher import and bytearray-hash exceptions exited1 before any samples; their causes/corrections are retained in MILESTONES.md. No timer-resolution exception was needed.

`cargo test --offline --locked`: exit0 (139 tests including four doctests). `cargo test --offline --locked --no-default-features --test codec`: exit0. Strict library Clippy, formatting and pinned clean C++ reference checks: exit0. The new regression covers counts0/1/3/4/17/31/32/33, every Oct/Quat precision, ties, zero signs, folds, clamps and non-finite inputs against the existing bulk kernels. Final frozen source/binary identities match both successful timing streams.

### Reproduction and retained evidence

[Machine receipt](results/p03-tiny-round2.json), [controller](encoders/tiny.py). Durable artifacts: `/mnt/linux-extra/moss-scratch/meshopt-v03-tiny`; baseline and C++ copies are hash-checked against the retained round 1 build/stream. It contains all candidate binaries, PMU/disassembly, parity receipts, failed/final samples, final source copy and admission receipts. The JSON binds 703 raw files by SHA-256.

Commands used `MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-tiny-* /mnt/linux-extra/moss-coord/bin/moss-heavy.sh <2 or 4> timeout 840 python3 parity/encoders/tiny.py <action> <epoch>`. Final actions: `build record16`, `parity record16`, `counters record16`, `verify`, `MESHOPT_TINY_FINAL=record16 ... freeze`, `measure tiny-record16`, then `measure confirmation`. Every admitted run finished in <15min. All Cargo work used `/mnt/linux-extra/moss-cargo-targets/codex-meshopt-tiny`, CARGO_INCREMENTAL=0 and CARGO_PROFILE_DEV_DEBUG=0; its final 86,792,129-byte footprint was <5GB and was deleted after all owned jobs ended.

Not run: other encoder elapsed rows, historical/default/Moss-profile streaming reproduction, GPU/downstream runtime, executed WASM, remote architectures or release qualification. Round 1 evidence for those separate boundaries is preserved.
