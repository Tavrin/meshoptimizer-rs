# SPEC-028 L2 cook chain

## Final registered results

**Lane outcome: PARTIAL.** Packed scale meets the M7 bar; cache, FIFO and
overdraw pass the baseline bars but miss M7. The real-stage disclosure does
not establish a cook-chain speedup: 214e's candidate/C++ cache and overdraw
ratios are higher than the v0.2.7/C++ controls in this campaign. Retain that
unfavorable result; there is no end-to-end performance-parity claim.

Exactly one registered campaign, CPU 9 (sibling 8), Moss scalar profile,
one warm-up plus ten interleaved pairs; 10,000 bootstrap resamples, seed
20261030. Five equally weighted frozen cases per family; overdraw also
includes all three retained real-overdraw cases. Ratios are ours/comparator.
X is disclosed only. The table applies the registered verdicts without retries.

Same-session A/A floor F = 0.154641; per-case upper95 limit = 1.254641.

| Family | Ours/M7 GM | Ours/v0.2.7 GM | Worst baseline upper95 | Ours/X GM | Verdict |
|---|---:|---:|---:|---:|---|
| cache | 1.175791 | 0.958186 | 1.105484 | 0.870075 | **PARTIAL** |
| cache-fifo | 1.283521 | 0.965976 | 1.071065 | 1.058836 | **PARTIAL** |
| simplify-scale | 0.939862 | 0.698418 | 0.863719 | 0.793916 | **PASS** |
| overdraw | 1.108457 | 0.982910 | 1.087405 | 1.140006 | **PARTIAL** |

The GM bars are <=1.00 against both M7 and v0.2.7; every case must also
pass the baseline upper95 bar. PARTIAL means the baseline bars pass but
M7 is missed: a measured speedup, with no performance-parity claim. FAIL
means at least one baseline bar misses. No verdict is rounded into a pass.

### Three-chain disclosure (not a bar)

All ordinary and legacy r2 calls, with their actual pruned positions recovered
from the immutable compacted captures. The corrected candidate/base/M7 proof
reproduces every historical stage output. These are matched stage replays,
excluding importer, policy evaluation, serialization and non-mesh work;
no whole-Moss latency or production acceptance claim follows.

| Chain | Arm/CPP | Scale GM | Simplify GM | Cache GM | Overdraw GM |
|---|---|---:|---:|---:|---:|
| 214e | candidate/M7 | 1.807366 | 1.195469 | 1.095295 | 1.372237 |
| 214e | v0.2.7/M7 | 1.871022 | 0.998894 | 0.859353 | 1.138893 |
| b6fd | candidate/M7 | 1.820116 | 1.091920 | 1.393074 | 1.488683 |
| b6fd | v0.2.7/M7 | 1.908814 | 1.125920 | 1.548131 | 1.558940 |
| a082 | candidate/M7 | 1.710493 | 1.152125 | 1.146721 | 1.195416 |
| a082 | v0.2.7/M7 | 1.683823 | 1.142932 | 1.155284 | 1.205613 |

Every stage interval, paired call time and output hash is retained in
`results.json` / scratch `campaign.json`; family and per-case intervals are
retained there too. No samples were discarded, and no wall campaign was rerun.

### Causes and retained fix

- Cache: per-neighbor fuel checks in adjacency searches/score updates.
  `src/cache.rs:175`, `:190`, `:205` use exact-prefix scan/search batching.
- FIFO: per-corner division and dynamic triangle slice checks.
  `src/cache.rs:416`, `:445` use triangle records, retaining the original
  per-corner fuel-exhaustion path.
- Scale: finite checks interrupt the packed bounds reduction.
  `src/simplify.rs:188` separates its finite reduction; strided bounds and
  the shared simplifier helper remain unchanged.
- Overdraw: dynamic byte offset/count checks for each indexed XYZ corner.
  `src/input.rs:688` uses one checked full 64-byte record lookup. Mapped,
  other-stride and final-XYZ-only views retain the general reader.

Safe Rust only; no public API, explicit SIMD path, encoder, memory policy,
version or Moss source change. The shared cache-strip implementation is
covered by exact proof. Input preparation is equal in all arms; the final
baseline/M7/X instruction audit agrees with the initial audit to four decimal
places in family GM (see retained raw counts).

### Targeted gate exits

| Gate | Exit |
|---|---:|
| fmt | 0 |
| all-features | 0 |
| no-default-features | 0 |
| clippy | 0 |
| v0.2.7 exact std | 0 |
| v0.2.7 exact no_std | 0 |
| C++ differential | 0 |
| final family and historical-stage adapters | 0 |
| widened fuzz | 0 |
| registered campaign | 0 |
| evidence audit | 0 |

Exact proof: 13,125,751 observations plus 557,056 scale-seed observations in **each** of std/no_std; 662 existing strict C++ protocol cases; 4100 existing FIFO seed/native/JS cases; 23 final family cases and all 56 historical stage calls in each of candidate/base/M7.

- simplify_scale: 359.84 CPU-seconds, 469,888,969 executions, 0 crashes.
- overdraw: 334.81 CPU-seconds, 67,768,619 executions, 0 crashes.

This is stable seeded mutation proof, without sanitizer or coverage-guided
instrumentation. The widened layouts include 64-byte full/final-short records,
unaligned little/big endian storage and arbitrary float bits.

Earlier rejected probes, consumer compile error, wrong-vertex stage replay,
launcher exit and bounded EASY timeout are retained below and in scratch.
They are not cleared or recast as passing acceptance samples. The library
proofs/gates listed above bind the final source; no wall value caused a retry.


Baseline: v0.2.7, `e5168983067a5decfc5bf15605c7dae7113dba49`.
Worktree: `/home/etienne/dev/meshoptimizer-rs-wt/cook-chain`, branch `perf/cook-chain`.
Scratch: `/mnt/linux-extra/moss-scratch/meshopt-028-L2/`.
Target: `/mnt/linux-extra/moss-cargo-targets/meshopt-L2` (delete after retaining binaries).
All heavy work uses `admit.sh`: CPU only, 4 GiB, one build job. No subagents.

## Adapter audit (before library edits)

The kept `bench/peer-screen` comparator was exported as instructed. All four
L2 families already use borrowed contiguous indices and borrowed position
storage: ours uses `Positions::from_packed` or the captured byte view; M7
uses the corresponding pointer/stride; X uses `as_flattened()` or the captured
slice. No `iter().flatten().collect()` input exists in these four adapters.
Each reorder adapter allocates its output inside the call; simplify-scale
allocates only the same four-byte result transport. Reused library workspace
scratch remains part of the public allocating API. Thus adapter equalization
requires zero changes, and before/after instruction counts are the same
retained observation, not a second measurement. The quantize artifact belongs
to another family and is out of scope.

Add a `count N` command to the scratch driver for instruction diagnosis only;
this branch never reads a clock. Input parsing, first warm call, and output
serialization happen before perf is enabled. Retain the exported input bytes,
hashes, source and binary identities. Build scripts resolve the older exported
lock offline to the pinned source version without modifying the kept branch.

## Pre-edit cause hypotheses

- cache: dynamic adjacency searches and score updates charge fuel per visited
  neighbor and each cached vertex. The hot loops also retain bounds checks.
- cache-fifo: fixed scan precharging is present, but each adjacency step retains
  a conditional accounting branch and indexed bounds checks; measure before
  choosing any change.
- simplify-scale: the bounds reduction has an early finite-value error branch
  inside each axis update, obstructing a straight reduction.
- overdraw: repeated checked position loads and geometry arithmetic remain
  after 0.2.7; inspect retired counts before choosing a narrower change.

No library edit and no wall sample has been made at this point. The required
milestone is complete only once the baseline counts are appended below.

## Equalized-count milestone (before source edits)

Comparator export revision: `e8827e4de9b69c98b9a96c6f49f8192a6b045ff7`.
Counts are retired user instructions per resident call, CPU 9, perf 6.17;
raw counters and executable hashes are retained. Before = after adapter audit
for these frozen/overdraw adapters (no input-preparation change).

| Family | Case | v0.2.7 before/after | M7 | X |
|---|---|---:|---:|---:|
| cache | tiny-smooth | 37151 | 23704 | 35705 |
| cache | medium-smooth | 13631989 | 8523784 | 12707636 |
| cache | medium-seams | 11996397 | 7415275 | 11035370 |
| cache | medium-sparse | 13683749 | 8594556 | 12755172 |
| cache | million-smooth | 1690620199 | 1058993142 | 1578763833 |
| cache-fifo | tiny-smooth | 11486 | 8325 | 11709 |
| cache-fifo | medium-smooth | 2593097 | 1624711 | 2251176 |
| cache-fifo | medium-seams | 2621066 | 1635350 | 2254471 |
| cache-fifo | medium-sparse | 2657543 | 1682806 | 2296596 |
| cache-fifo | million-smooth | 316200800 | 197826466 | 274630574 |
| simplify-scale | tiny-smooth | 1094 | 854 | 1130 |
| simplify-scale | medium-smooth | 131304 | 101664 | 135540 |
| simplify-scale | medium-seams | 161033 | 124680 | 166228 |
| simplify-scale | medium-sparse | 262279 | 203064 | 270740 |
| simplify-scale | million-smooth | 15583659 | 12064826 | 16086350 |
| overdraw | tiny-smooth | 16570 | 20226 | 17996 |
| overdraw | medium-smooth | 1790700 | 1598662 | 1808373 |
| overdraw | medium-seams | 1818936 | 1592648 | 1826641 |
| overdraw | medium-sparse | 1904775 | 1627909 | 1875969 |
| overdraw | million-smooth | 214045791 | 191777642 | 217322522 |
| overdraw | real-214e-overdraw | 639462445 | 283854685 | 388500562 |
| overdraw | real-b6fd-overdraw | 459852387 | 248932738 | 308676298 |
| overdraw | real-a082-overdraw | 1839377838 | 995683072 | 1234669079 |

The equalized counts retain material gaps. The cache kernel has per-neighbor
fuel checks in its two dominant adjacency loops. FIFO already precharges scans,
but retains a conditional `work.add` in each iteration. Scale has three early
finite-value branches per position. Real overdraw still retires much more work
than C++, with sequential finite/centroid validation and checked strided
geometry reads as the remaining sources. These are source-supported causes;
individual gains must be isolated with instruction counts, not wall pilots.

Planned bounded fixes: use existing exact-prefix Work::search/scan for cache;
use scan instead of conditional precharge loops for FIFO where possible;
reduce finite flags without early exits for scale (same final error);
optimistically batch successful overdraw validation and replay its original
counted scan on nonfinite input. No public API, scratch memory policy, SIMD,
encoder, or other family change. Shared cache-strip behavior must remain exact.

The stage trace adapter will also correct the scratch comparator's hardcoded
scale stride 12 to the captured stride. Frozen cases use stride 12 and are
unchanged. Real stage disclosure uses matched clone, allocation and output-copy
boundaries in both arms, and retains every r2 ordinary/legacy stage call.

## Admission and proof preparation

The generic Python-class initial submission was canceled before admission
(exit 143, no build/count/sample) because its class reserved 8 GiB despite the
4 GiB declaration. The named `job.sh` submission reserves 4 GiB normally.
No admission bypass or declaration override was used.

The r2 replay adapter retains all ordinary and legacy stage calls and their
captured output bytes/error bits. It excludes importer, serialization and
non-mesh stages. Clone, allocating result and output-copy boundaries are
matched for both Rust and C++ reorder stages. The full Moss caller is not
rebuilt or modified; this is a stage-trace disclosure, not whole-Moss latency.

## Instruction-only rejection of first probe

Cache scan/search batching reduces counts to 0.899–0.923 of v0.2.7. Keep it.
FIFO replacing its existing precharge loops with Work::scan increases counts
1.004–1.012: reject that part. Deferred finite flags increase scale counts
1.144–1.194 and overdraw synthetic counts 1.004–1.014, with mixed real results:
reject both. No wall samples have been taken.

Second probe keeps cache batching, changes FIFO adjacency filling to triangle
records on the already-precharged path (original corner-by-corner fallback),
and writes fixed-size output triangle records. For scale and overdraw, combine
all three finite tests into one vertex predicate before arithmetic, preserving
the first failing vertex and every work charge. This is a narrower change to
branch shape; scale's shared simplifier bounds helper remains unchanged.

The existing scale fuzz target only constructed packed positions. Widen its
layout selection to padded floats and unaligned little/big endian byte views,
without consuming extra RNG values or changing the existing position seed
stream. Run the widened target for at least five CPU-minutes as required.
Overdraw's original layout cycle correlated all arbitrary-float mutations
with packed storage. Widen its cycle too, so arbitrary floats reach every byte
layout; run both changed targets for five CPU-minutes each. FIFO's existing MO02 target reaches both adjacency paths. The exact
checker also exhausts small-mesh fuel prefixes and replays 4096 existing
seed mutations in both std and no_std.

## Second probe and selected third diagnosis

Second-probe counts: cache 0.899–0.923 of baseline; FIFO 0.863–0.880.
Keep both. Combined per-vertex finite predicates increase scale instructions
1.072–1.097 and leave overdraw effectively unchanged (real +0.08–0.11%);
reject both. std exact checker passed 6,563,003 observations plus 278,528
scale-seed observations. The no_std checker build failed because its generated
harness tried to convert the crate's no_std Error into std::error::Error.
Retain logs in `probe2-proof/`; fix that generator's known-valid flag/option
unwraps. This was a proof-consumer compile failure, not a library mismatch.

Third probe: a separate contiguous packed finite reduction for scale, keeping
strided bounds and its error behavior unchanged. Overdraw's indexed byte reads
currently check count plus dynamic byte offsets/bounds per corner. For full
64-byte records, use a checked `[u8; 64]` record lookup whose type proves the
XYZ bytes exist. Mapped views, other strides and a final XYZ-only record keep
the general reader. This helper is consumed only by overdraw. No unchecked
read, API or memory-policy change. Widen overdraw's generator stride range to
reach 64 and its padding cases, and compare all error/work prefixes with the
original reader through the exact checker.

## Final source selection and pre-sample registration clarification

Third-probe ratios to v0.2.7: cache 0.899–0.923, FIFO 0.863–0.880,
scale 0.690 on medium/large and 0.789 tiny; overdraw packed 1.000,
real 0.967/0.955/0.955. Retain this source candidate. No wall sample yet.

Before the first wall sample, clarify that the overdraw family bar includes
all three retained real-overdraw inputs as well as all five frozen cases.
This strengthens case coverage; the separate full-chain stage disclosure
still has no bar. Remove unused synthetic attributes/codec transport from
real-input setup equally in all four scratch adapters to fit two resident
real drivers below the 4 GiB declaration. This preparation is outside the
counted/timed region and does not change the API inputs or allocation boundary.
Retain a fresh instruction audit and exact frozen/real adapter outputs after
this harness-only adjustment. No other family or threshold is changed.

## Pre-wall stage-reference diagnosis

The final 23 family cases match v0.2.7 and M7 byte-for-byte. All ordinary and
legacy stage calls also match across candidate/v0.2.7/M7. Twelve overdraw
stage outputs differ from the historical r2 descriptors in all three arms;
all other captured stage outputs match. A fresh standalone pinned C++ check
with the r2 compiler flags discriminates the old descriptor binding from the
current oracle. Do not treat the historical hashes as a candidate regression
or claim they were reproduced. Raw mismatch hashes are retained.

Existing strict C++ differential: 662 cases passed; existing FIFO fuzz seed
and native/JS fixture replay: 4,100 cases passed. A launcher exited 2 after
those proofs completed because its shell file was edited in place during the
run. The source/proof results are retained and unchanged; launcher replacement
is now atomic. No wall sample existed and no acceptance stream was rerun.

## Resolved stage mismatch: aggregate pruning changes positions

The historical capture is sound. The initial replay incorrectly reused the
original vertex buffer for every stage. Moss `gltf_mesh.rs:1279` passes
`pruned.vertex_bytes` to packaging; `prune_level` at :1710 rescales positions
of every kept component, including welded duplicate vertices. The cache
stage's count remains the full original count. Only overdraw reads those
rescaled positions, explaining why scale/simplify/cache matched but all three
implementations disagreed with the pruned overdraw descriptors.

Reconstruct that buffer without recomputing floating-point values: pair
captured pre-compaction global indices with captured compacted indices,
copy the captured compacted XYZ bytes to their original vertices, propagate
those bytes to signed-zero-normalized welded-position duplicates, and keep
all other original bytes. `welded_components` labels unused duplicates in the
same component; every kept component's indexed positions remain in the pruned
index stream. Assert that compacted non-position attributes are unchanged and
that each welded position has an unambiguous replacement. Bind all source
blobs and recovered-buffer hashes. The corrected no-clock proof must match
all historical outputs across candidate, v0.2.7 and M7.

The earlier failed raw-buffer replay and manifests remain under
`raw-vertices-replay-failed/`. A standalone strict C++ raw-buffer invocation
also reproduced the same raw-buffer result; its partial historical mismatch
was diagnostic evidence for the input-context error, not a library defect.
No library source or wall campaign was changed during this repair.

## EASY backfill proof interruption

The scheduler bounded the stage-rebuild job to its head-safe backfill window;
it returned 124 after 174 seconds. Builds, final-stage instruction counts and
all 23 frozen/real family comparisons completed. No corrected real stage arm
had completed, and no wall sample existed. Preserve that partial receipt in
`stage-backfill-timeout/` and continue only the unfinished stage proof using a
separate `proof-adapters.sh` class. The wall campaign likewise uses its own
`wall.sh` class so the mixed short build/count duration history cannot assign
it an unsuitable EASY deadline. Admission policy and RAM declarations remain
unchanged; own-lane admissions are limited to one going forward.

## Qualification scope of the frozen real-derived case

The kept `real-214e-overdraw` family input pairs original vertex positions
with a post-pruning index subset. It is a valid frozen real-derived geometry
case, but it is not the rescaled-position call made by Moss's pruning path.
Keep its bytes unchanged for the frozen family bar. The separate corrected
r2 stage replay uses the actual recovered rescaled vertices and reproduces
all historical outputs. Family ratios therefore must not be described as
whole-Moss latency or as a substitute for the real stage disclosure. The
packed scale speedup likewise leaves the original byte-view scale reduction
unchanged; its actual real-chain stage ratio is disclosed separately.
