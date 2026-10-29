# Simplifier profile and instruction gate

Baseline: v0.2.15, `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
C++: 1.3, pinned oracle `4c203430ca565cb59a468a91922c76c208169536`;
the peer crate's `simplifier.cpp` is byte-identical to that oracle.
No algorithm, output, API, resource-accounting or failure-prefix changes.

Profile all four functions on real-214e, real-a082 and real-b6fd before editing
library source. Sample `instructions:u` at period 200003, retaining source-line
reports and `perf annotate`. Write PROFILE.md before the first experiment.
Sample attribution is statistical, including skid; phase estimates are not
exact per-function hardware counters or proof of a unique causal decomposition.

Measure base and candidate with std/scalar and std/default, C++ scalar and
C++ default. Rust scalar disables the simd feature; default enables it.
C++ scalar defines MESHOPTIMIZER_NO_SIMD. Both C++ arms define NDEBUG and
use no fast math or floating-point contraction. All arms use the same peer
consumer's call shapes, owned outputs and warmed workspace; update includes
its index/position input copies. Parsing, synthetic attribute generation,
warmup and serialization are outside counters. Output allocation and drop
are inside. Instruction and loop wall-time samples cover the same calls.
Wall time is disclosed, with no timing acceptance claim on this shared host.

Eight inputs are the frozen 0.2.15 corpus: tiny-smooth, medium-smooth,
medium-seams, medium-sparse, million-smooth and the three real meshes.
Input hashes must match the retained input inventory. All six arms' complete
serialized outputs (including error bits and updated positions) must match.
Three repetitions per cell, arm order reversed in repetition 2; median per
cell, geometric mean of ratios. A function SHIPS only if default candidate/base
real-mesh GM <= 0.95 and every case <= 1.01. Otherwise record NO-GAIN and
reject that function's independent experiment. A shared edit may remain when it
meets a covered function's gate and all consumers pass their regression ceiling;
other functions still receive their own NO-GAIN verdict. Keep each edit only with measured gain.
No SIMD experiment unless the profile identifies arithmetic as the limiting
loop and the scalar experiment leaves an attributable opportunity.

Exact proof compares base and candidate results, complete destinations/tails,
mutable position/attribute bits and both usage fields. Every work limit from
zero through unrestricted usage + 1 is covered on the selected small cases;
repeat calls cover cold, warm and failed workspaces, with an additional
warm-success/late-numerical-failure primer on layout/width cases. Cover owned/into/update APIs,
all option masks on one small mesh, targeted option combinations, storage
budgets, finite extremes, subnormals, signed zero and rejected values.
Run both all-features and no-default modes. Supplement with a seeded stream
for at least 300 child CPU seconds total and the existing pinned C++ protocol
corpus in both feature modes. The boundary corpus is finite, not an exhaustive
proof of arbitrary inputs or configuration combinations. Budget targeted
execution below two CPU hours; finish each selected case, never terminate a
queued job. Retain sources, binaries and receipts outside the disposable
Cargo target, then remove that target after all required checks finish.
