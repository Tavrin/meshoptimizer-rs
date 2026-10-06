# Encoder performance: 0.3 round 1

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
