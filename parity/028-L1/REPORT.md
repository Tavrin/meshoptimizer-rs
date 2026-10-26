# SPEC-028 L1 — cluster LOD paths

Base e516898 (v0.2.7), branch perf/clod-path. Scope is the six L1 families only.


## Final result

Performance verdict: **PARTIAL**. This is the sole registered wall campaign;
no samples were discarded and no verdict reruns occurred. A PARTIAL family is
a measured speedup versus 0.2.7 without a C++ performance-parity claim. A FAIL
family is not performance-qualified. All proof requirements passed independently.

| Family | O/M7 GM | O/0.2.7 GM | O/X GM | Max case upper95 vs 0.2.7 | Verdict |
|---|---:|---:|---:|---:|---|
| partition | 1.450544 | 0.989458 | 1.106790 | 1.056675 | PARTIAL |
| meshlets-scan | 1.067374 | 0.801371 | 0.700001 | 0.920768 | PARTIAL |
| meshlets | 1.022411 | 0.930124 | 0.861130 | 1.065561 | PARTIAL |
| meshlets-spatial | 1.121742 | 0.992160 | 1.092985 | 1.157240 | PARTIAL |
| meshlets-flex | 1.007063 | 0.933526 | 0.876291 | 0.987662 | PARTIAL |
| spatial-triangles | 1.026324 | 0.928209 | 0.925812 | 1.024605 | PARTIAL |

Same-session A/A floor F=0.170512; every per-case regression limit is
1.10+F=1.270512. All 120 streams have exactly one explicit warm-up and ten
alternating pairs (30 A/A and 90 comparisons). Driver startup performs its
existing untimed output/serialization check before the explicit warm-up.
Paired bootstrap: 10,000 resamples, seed 20261030. Raw pairs and case intervals
are in RESULTS.json; X is disclosure only. Ratios are candidate/comparator.

RFC113 disclosure (sum of case medians, 10 pairs/case): combined
**1.040298x C++**, S1 1.037887x (41 cases),
S2 1.053315x (nine valid cases), maximum case
1.188750x. This is disclosure, not a bar. Scalar C++ and the
original cluster-C input/configuration/timer boundary were retained. All 50
preflight outputs matched C++; all 41 S1 outputs also matched retained oracles.

## Retained fixes and causes

- partition: repeated local-table clearing; dense-input generation tags in the existing u8 allocation; `src/partition.rs:418`.
- meshlets-scan: per-face descriptor publication and output bounds; local descriptor and sequential output iterator; `src/meshlet.rs:1453`.
- meshlets: runtime zero-weight cone arithmetic; entry specialization folds the bounded search factors; `src/meshlet.rs:1166`.
- meshlets-spatial: BVH integer-division latency and indexed partitioning; exact invariant quotient and stable branchless partition; `src/meshlet_spatial.rs:104`.
- meshlets-flex: same hot loop as meshlets; zero-weight specialization and shared append/position preparation; `src/meshlet.rs:1166`.
- spatial-triangles: per-corner view dispatch and generic Morton interleave; one storage dispatch, moderate-coordinate centroid path and 16-bit interleave; `src/spatial.rs:101`.

The exact divider is `src/meshlet_division.rs`; no floating-point backend,
public API, unsafe library code, output allocation policy, resource threshold,
encoder or SIMD path changed. The generation-map and divider changes introduce
no Workspace allocations and retain the original Result/work/byte accounting.

## Required proof exits

| Gate | Exit |
|---|---:|
| cargo test --locked --all-features | 0 |
| cargo test --locked --no-default-features | 0 |
| cargo clippy --locked --all-features --all-targets -- -D warnings | 0 |
| cargo fmt --all -- --check | 0 |
| cargo fmt --manifest-path <repo>/parity/028-L1/Cargo.toml -- --check | 0 |
| Existing C++ L1 differential, 2,000 seeds/family, alloc/into plus 64 wide partition seeds | 0 |
| v0.2.7 exact Results/work/output/caller-buffer contents, 34,172 comparisons | 0 |
| Frozen five-case corpus against M7 and v0.2.7, all six families | 0 |
| RFC113 50-case preflight | 0 |
| Widened layout/topology/resource differential fuzz | 0 |
| Full-width production-divider property fuzz | 0 |
| Single registered wall campaign | 0 |

Mesh differential fuzz: 5,130,240 executions, target CPU
300.010 s (total CPU 938.708 s),
seed 20261030, zero mismatches. Divider fuzz: 71,960,000,000 cases,
300.067 target CPU seconds, zero mismatches against native
integer division. Both are stable seeded mutation, without sanitizer or
coverage instrumentation. Replay descriptors, stream hashes and executable
identities are retained in the exported receipts and scratch.

## Evidence boundary

Measurements are native Linux x86-64 on a shared host, pinned to CPU 2 for the
interleaved wall campaign; all load observations are retained. Claims are limited
to the registered corpus/profile. No a downstream engine runtime/GPU acceptance, other-platform
performance, release, merge, tag or publication is claimed. All lane additions
are under parity/028-L1; only required library edits and the existing changelog
are outside it. Scratch and binaries are retained at `<scratch>/meshopt-028-L1`. The designated
Cargo target is deleted only after retention; cleanup is a separate receipt.

## Adapter diagnosis before library edits

The bet-0 comparator was exported from bench/peer-screen to scratch by git archive.
All L1 position adapters already use contiguous packed slices; the quantize
iter().flatten() artifact is outside L1. The original meshlet adapters allocate
different output extents: ours allocates index_count vertices/triangle bytes,
peers bound*64 vertices and bound*124*3 triangle bytes. Equalization uses public
caller-buffer Rust APIs and identical zeroed descriptor/vertex/triangle extents
(max(index_count, peer bound extent)), within the timed call, in all arms.
For partition and spatial-triangles the existing contiguous inputs and allocating
outputs already match, so their adapter remains unchanged. counts-original.json
and counts-equalized.json will retain before/after instructions:u for all 30
family/corpus cells and O/M7/X. No wall values from diagnosis are retained.

No library edits until equalized counts and named causes are recorded here.

## Preparation receipts and decisions

- Initial generic-python admission inferred an 8 GiB reservation despite the
  4 GiB declaration; canceled while queued and used the explicit declared-size
  mode supported by <job-runner>. The first admitted compile failed before codegen
  because the frozen comparator lock still named its historical library version.
  Regenerated only the lane/scratch lockfiles offline, then kept --locked builds.
- Switched the preparation batch to one Cargo worker and a 2 GiB declaration
  (within the spec's <=4 GiB cap); measurements are separately marked with
  MOSS_HEAVY_MEASUREMENT=1. Earlier queued jobs were canceled and logs retained.
- Existing CHANGELOG.md will receive the specifically required measured-only
  Unreleased entry. The no-root-files rule is applied to newly created lane
  artifacts; this required edit of the existing changelog is not a new root file.
- No subagents, no other lane worktree reads/writes, no SIMD/encoder/a downstream engine/version
  edits. Existing comparator FFI code is reused for trusted frozen inputs; no new
  unsafe library code or optimesh implementation is introduced.

## Equalized-count milestone (before library edits)

Instruction-only diagnosis, installed perf 6.17, pinned CPU 2, user instructions;
all 180 cells completed. Counts include identical allocations but exclude loading
and serialization. No gap vanished: none of the six L1 gaps is an adapter artifact.

| Family | Equalized/original ours instructions GM | Equalized ours/M7 instructions GM |
|---|---:|---:|
| partition | 1.000097 | 1.492510 |
| meshlets-scan | 1.011728 | 1.564736 |
| meshlets | 1.000653 | 1.226673 |
| meshlets-spatial | 1.001212 | 1.112309 |
| meshlets-flex | 1.000696 | 1.226661 |
| spatial-triangles | 1.000001 | 1.289062 |

Named source causes to investigate, with instruction profiles as the next causal check:

- partition: adjacency bounds/accounting and repeated reciprocal square-root
  lookup/division in `pick`; the output adapter is already equal.
- meshlets-scan: generic append repeats local-vertex lookups and charges work per
  triangle; no geometry calculation or input adapter explains the gap.
- meshlets: the shared flex builder repeats generic position decoding per corner,
  then checked candidate traversal and append work.
- meshlets-spatial: shared generic append plus bounds-checked indexed BVH/radix
  traversal; its median-size instruction cost remains about 13% above M7.
- meshlets-flex: same flex hot loop as meshlets (matched zero split/weight on the
  frozen corpus); almost identical counters establish the shared cause.
- spatial-triangles: repeated position-view dispatch per centroid corner and
  safe radix scatter/index checks. The tiny case already beats M7 instructions,
  while each larger case retains a substantial gap.

Raw before/after counts and every perf stat receipt remain in scratch. These
counts are diagnostics, not wall-clock speed claims. No library edits have yet
been made at this milestone. Follow-up ablations will distinguish successful
fixes from residual causes rather than asserting that every overhead is removable.

## Instruction profile disposition

The pre-edit instruction profiles use v0.2.7 with equalized adapters and retained
symbols (strip disabled only for symbolization). Medium-smooth user-instruction
samples put scan 95.75% in execute; meshlets/flex 83.2-83.3% in flex plus ~12% in
kd_build; spatial meshlets 75.22% in BVH split and 14.50% in execute; partition
44.49% in its outer routine and 39.24% in adjacency; spatial-triangles 40.66% in
remap, 28.01% in keys and 27.21% in centroid generation. Software sqrt is only
0.37-0.52% in the relevant profiles: do not change the pinned math backend.
This refines the earlier source hypotheses: reciprocal roots are not the primary
partition cause, and position decoding is only one component of flex cost.

First candidate targets common append table loads, covered-budget scan charging,
once-per-loop packed-position dispatch, adjacency budget loop unswitching, and
branchless BVH stable partitioning. Every fallback retains the old charge/error
order. No storage bounds or output allocation policy changes in the library.

Trial 1 instruction GMs versus equalized v0.2.7: partition 0.999997, scan
0.970766, meshlets 0.994859, spatial meshlets 0.974038, flex 0.994862,
spatial-triangles 0.871142. Exact trial-1 proof passed 33,660 comparisons,
including frozen meshes, 2,000 existing seeds/family, caller-buffer tails and
resource failures; raw corpus and both binaries retained. The packed partition
and adjacency loop refactors alone do not establish a speedup.

Trial 2 additionally replaces partition's relative adjacency slots with absolute
slots: prior-cluster slots precede the current written prefix, eliminating a
second indexed load solely to identify membership. Stable deduplication uses an
unconditional scratch write and advances only for a new vertex. The scan keeps
its descriptor local and zips sequential triangle outputs. Bounded flex searches
with zero cone weight use the exact constant cone factor 1, avoiding irrelevant
normal interval arithmetic; nonbounded/NaN behavior stays on the old path.

The lane seed generator is widened to packed XYZ, offset/stride floats and both
byte orders so the new storage dispatch arms are reachable. A five CPU-minute
seeded differential campaign is required and will retain seed/count replay,
executable/source identities and exact Result/work/output witnesses.

Trial 2 exact proof passed 33,660 comparisons, including the widened four storage
layouts. Instruction GMs were partition 1.114719 (rejected), scan 0.796881,
meshlets 0.930384, spatial meshlets 0.987236, flex 0.930390 and spatial-triangles
0.849996. The unconditional deduplication scratch write was removed before the
next trial; it cannot ship on this evidence. Trial 3 retains absolute adjacency
slots and specializes zero-weight flex at entry so its inlined search can fold
constant score factors. All trial receipts/source patches/binaries are retained;
there has still been no wall sample.

Trial 3 instruction GMs: partition 1.076911 (absolute slots rejected), scan
0.796890, meshlets 0.911096, spatial meshlets 0.986853, flex 0.911105 and
spatial-triangles 0.849995. Trial 4's equal-length adjacency slices produced
partition 1.000002 and were also removed. Both exact proofs passed.

Trial 5 replaces per-cluster clears of partition's existing u8 table with
nonzero generation tags on dense inputs, clearing at generation wrap; sparse
inputs keep local clears. No extra storage or Workspace accounting changes.
It reduces partition instructions to 0.948211 of v0.2.7 (every case improves),
while scan is 0.796888, meshlets/flex ~0.91110, spatial meshlets 0.986853 and
spatial-triangles 0.849996. Its exact proof passed 34,172 comparisons. The
original small seed corpus is retained in full, plus 64 generation-wrap cases
in all four layouts and both allocating/into forms.

Trial 6 addresses the BVH pivot's repeated integer divisions with an exact
invariant reciprocal and correction, implemented in safe Rust for full-width
usize and divisors 1..512. This is integer arithmetic, not a change to the
pinned floating-point backend. It costs 6.148% more spatial-meshlet instructions
than trial 5. Noninterleaved cycle data showed large drift even on unchanged
families and is not causal speed evidence. Five interleaved counter pairs per
case then gave spatial-meshlet cycle GM 0.950158, identical-binary A/A GM
1.005898; individual A/A cases varied up to ~8%. This supports taking the
latency tradeoff to the registered wall check, not a wall performance claim.
Trial 6 exact proof passed 34,172 comparisons. The arithmetic generator includes
the actual production helper and full-width random/MAX/zero inputs; its required
five CPU-minutes are separate from the mesh/layout differential fuzz budget.

The candidate is now frozen for the required gates and the single final wall
campaign. Only registered wall results decide performance acceptance.

## Semantic invariants checked before wall sampling

- The covered scan's upper bound is triangle_count + index_count: each flushed
  vertex was inserted by a previous corner. Its final charge equals the old
  per-triangle/per-flush sum; short budgets take the old charge points. Output
  tails and the conservative duplicate-corner flush rule are preserved.
- Partition generations are nonzero, clear on every 255-cluster wrap with cost bounded by touched entries, and use
  the existing byte table. Sparse inputs keep the old local-clear strategy;
  omitting the final clear affects only discarded private scratch.
- Packed position dispatch retains mapped/strided/byte fallbacks. Centroid
  finiteness checks are omitted only when all inputs are bounded by 1e8, so
  each ordered partial sum is bounded by 3e8. Morton quantization remains
  below 65536, making the removed 20-bit interleave stage an identity.
- The BVH divisor's high product is the exact quotient or one greater. A
  wrapped negative remainder identifies the latter for divisors <=512; value
  1 is handled directly. The generator includes full-width integers that the
  mesh-size generator cannot reach, and compares the actual production helper
  against native division. All BVH visit counts and split tie rules are retained.

## Reset-bound correction before wall sampling

Static review found that trial 5/6's full-table generation resets could take
O(vertex_count * cluster_count / 255) work on large dense domains with many
small clusters, despite preserving the old visit counter. That candidate was
withdrawn before any wall request. Qualification was deliberately stopped during
mesh fuzzing; its completed gates/differentials/preflight, partial fuzz progress,
source snapshot and binaries remain in qualification-before-reset-bound/.
The interrupted fuzz run does not count toward the required final budget.

Each reset now clears at most the number of filtered entries touched by that
generation: use a whole-table clear only when the table is no larger; otherwise
clear through the generation's filtered prefix. Prefixes are disjoint across
wraps, so total reset visits are <= filtered index count and remain O(N), with
no allocation or public work-usage change. Wide seed cases now include both
14x14 and 24x24 grids with one index per cluster, exercising both reset branches.

The restarted fuzz generator retains hostile/error samples and reserves half
its samples for ample resources, increasing coverage of successful new paths.
CPU accounting is polled every 512 cases rather than per case, without changing
the minimum 300 target CPU seconds. A fresh complete proof budget is required.

The corrected source's fresh tests/clippy/fmt all passed. A locked offline peer
build then exited 101 because autocfg 1.5.1 was absent from the shared cache;
`cargo fetch --locked` restored the exact locked dependencies, with no lockfile
change. The continued differential and 50-case preflight passed.

The mixed fetch/build/qualification submission was then classified as a short
build and admitted into an EASY backfill window. The wrapper enforced that
window and exited 124 during fuzzing. This was an admission deadline, not a
reported mismatch; the incomplete prefix/source/binary binding is retained in
fuzz-backfill-timeout-1/ and does not satisfy the final budget. Long phases now
have distinct scripts and explicit honest duration forecasts: mesh fuzz 2,400 s,
divider fuzz 600 s, final wall campaign 3,600 s. RAM declarations remain <=4 GiB
(2 GiB mesh/wall, 1 GiB divider; at most 3 GiB concurrently). Queue policy and
backfill deadlines are unchanged. Required completed proof is source-bound;
only the unfinished fuzz budgets are restarted. No wall sample has been taken.
