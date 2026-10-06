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
