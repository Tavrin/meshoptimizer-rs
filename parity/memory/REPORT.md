# SPEC-027 memory chain

Registered verdict: **FAIL**. RSS passed; the default candidate fails the
wall-time bars and is not qualified for release. No timing reruns.

## Step 1 diagnosis (recorded before the library change)

The retained step-1 full-driver trace identifies the RSS peak at packaging,
not at an algorithm's maximum `Usage.bytes`. In the reuse arm, the second
`observe:packaged` raises VmHWM to 2,988,696 KiB with the following workspace
allocations still live (bytes):

| Owner / buffer | Retained bytes |
|---|---:|
| Simplify typed state | 789,938,002 |
| Cache vertices | 81,581,292 |
| Shared integers (grown by overdraw) | 32,895,784 |
| Shared floats | 2,851,008 |
| Shared flags | 712,752 |
| Shared keys | 986,870 |
| Total | 908,965,708 |

The per-call-clear control has no retained scratch during packaging and its
process peak is 2,205,172 KiB. The tracked maxima (1,010,286,584 versus
824,150,014 bytes) differ by only 186,136,570 bytes because those maxima occur
inside simplification, with its live state charged in **both** arms. Their
subtraction does not measure the scratch held during the later packaging peak.
The 789,938,002-byte dormant simplify state alone is 0.735687 GiB, compared
with 0.747227 GiB total RSS saved. Caller/cooker copies and packaging allocations
are outside `Usage`, but overlap the retained state in process RSS.

This identifies cross-algorithm retention as the mechanism. The diagnostic
allocator probe will check the possible residual contribution from realloc
and allocator retention; neither is needed to explain the large difference
between the two independently maximized metrics. No timing claim is drawn
from diagnostic probes. The original SPEC-026 report is in STEP1-026.md and
its receipts remain unchanged.

Decision: release scratch belonging to the other chain families when simplify,
cache or overdraw acquires scratch, while retaining scratch on consecutive
calls within one family. Do not change codecs, other algorithm families,
limits, quadric layout or public API. This is not evidence for the kill rule:
the registered timing check must measure the family-switch allocation cost.

## Allocator probe confirmation

The instrumented source copy reproduces the reuse peak at packaging:
2,986,576 KiB versus 2,126,424 KiB with per-call clear. Its live workspace
inventory matches the table above exactly. Simplify's 789,938,002 bytes are:

| Simplify buffer | Bytes |
|---|---:|
| q (quadrics, layout unchanged) | 326,325,168 |
| collapses | 102,636,036 |
| p | 81,581,292 |
| edges | 68,424,024 |
| order | 34,212,012 |
| offsets | 27,193,768 |
| remap, wedge, forward, back, cr (each) | 27,193,764 |
| kind, locked (each) | 6,798,441 |
| a, aq, g, vg, sparse, original, table | 0 |

At the first observation of final RSS HWM, the global allocator's current
requested capacity is 3,674,212,609 / 2,579,412,485 bytes (reuse / clear), and
peak requested capacity is 3,929,490,364 / 2,834,692,263. These include caller
and cooker storage, and virtual capacity need not be resident. glibc arena
free bytes are 36,098,912 / 48,154,384, not a 0.75 GiB reservoir of freed arena
storage. Moved-realloc old-byte totals are 686,132,576 / 317,087,241, cumulative
across the process, not simultaneously live storage. They are not added to
`Usage` or interpreted as RSS. The decisive overlap is live dormant simplify
scratch plus the caller's packaging copies; independently maximized tracked
and RSS metrics conceal that lifetime overlap.

Evidence: spec027/DIAGNOSIS.json, both diagnosis stderr logs, source snapshots
and setup.py under the designated scratch root. The initial probe build
exited 101 for missing rustdoc on its probe-only accessor; adding that comment
allowed the build and both diagnostic executions to exit 0. No library
behavior was changed to obtain the diagnosis.

## RSS iteration 1: acquisition alone is too late

The default retain-last candidate, before any registered timing, peaks at
2,548,556 KiB (2.430492 GiB), above 2.318 GiB. Packaging no longer dominates:
`observe:prune_decision` reaches 2,544,456 KiB with simplify state retained,
then cache's acquisition releases it. Caller-side pruning happens between
simplify returning and cache acquiring; acquisition cannot release scratch
retroactively. This is a newly exposed lifetime overlap, not an allocator
arena leak. The complete candidate and probe are retained as iteration 1.

Decision before the next library edit: also release simplify's transient
collapse-candidate and ordering arrays on return (102,636,036 + 34,212,012
bytes in 214e). Both are cleared before use on the next call; order is already
refilled, and collapse candidates are already rewritten. Keep the other typed
arrays and same-family reuse. This saves the required overlap without changing
quadric layout, output, error checks, work accounting or limits. This is a
specific exception to retaining every simplify buffer, justified by the RSS
probe; its per-call allocation cost must pass the registered wall bar.

Strengthen the repeated-call microbench to the largest 214e mesh before any
timing sample, to expose this cost directly rather than testing the smaller
b6fd case. Preserve the original registration and seal as superseded setup,
and seal the amendment before collecting any timed samples. No bar, sample
count, bootstrap rule or verdict rule changes. No timing verdict exists yet.

The first transient-release full gate exited 101: an existing pressure test's
fixture no longer produced pressure after retained storage decreased. The
library result assertions passed. The fixture now supplies unused cache
vertices so cache demand exceeds simplify's peak, retaining the explicit
`retained + cache_demand > max_bytes` premise and the fresh/reused output/error
checks. This is a test-fixture repair, not a relaxed budget assertion. The
failed full receipt is retained in spec027/transient-gate-failure; it is not
cleared by the passing focused test. The complete gates are rerun after repair.

## RSS iteration 2: capacity is not residency

The second probe exits 0 with peak 2,515,284 KiB (2.398762 GiB): releasing
136,848,048 requested bytes saves only 33,272 KiB. The collapse allocation is
reserved without initialization, whereas the ordering allocation was resized
and zeroed across its full upper bound. All three 214e simplifications return
the original index count and zero error. Requested capacity must not be
substituted for resident pages; freeing reserved candidate capacity did not
close the caller-pruning overlap.

Before another library edit, revise the mechanism: keep candidate/order
capacities reusable, but initialize order only for the actual picked count,
within the same already-reserved bound. Release normalized positions and the
position remap/wedge vectors on return instead (81,581,292 + 27,193,764 +
27,193,764 bytes for 214e). These three vectors are cleared before reservation
and fully rewritten on the next call anyway. Keep edge/quadric and ranking
capacities, the quadric layout and counted visits. The repeated-call wall bar
on 214e must still qualify the new page/allocation cost. No timed samples exist.

## RSS iteration 3: remaining pruning overlap

The third probe exits 0 at 2,452,120 KiB (2.338524 GiB), still above the bar.
The final high-water is again at prune/cache transition; acquisition alone
cannot solve that pre-acquisition overlap. The position/remap/wedge release
plus lazy order initialization reduced RSS by 63,164 KiB, less than its
requested-capacity change. Sub-page residency and allocator reuse/retention
remain relevant to the residual; no capacity subtraction is claimed as an
RSS prediction.

Before the next edit, release the 68,424,024-byte adjacency-edge vector on
return as well. Its contents are fully rebuilt on every call. Retain quadrics,
classification/offset arrays and candidate/order capacities; keep the same
registered 214e repeated-call cost test and every bar. This is the last
mechanism iteration before the registered timing verdict; do not tune against
that verdict. All earlier candidates, measurements and failed gate are retained.

## Residency diagnosis and final mechanism

Iteration 4 exits 0 at 2,451,804 KiB (2.338223 GiB). The additional `mincore`
probe reproduces its pruning peak at 2,451,684 KiB and directly measures every
retained allocation. `q` has **326,328,320 resident bytes** for 326,325,168
requested bytes. Collapse capacity is 102,636,036 bytes but only 48 bytes are
live and 4,096 bytes resident. Order capacity is 34,212,012 bytes, with 16 live
bytes and 4,096 resident after actual-count initialization. All mincore calls
exit 0. The remaining offset/kind/forward/back/cr/locked rows have about
122 MB resident. glibc also holds **151,855,696 free arena bytes** in a
397,852,672-byte arena. Thus the smaller-array releases partly move storage
into allocator retention, while the untouched candidate capacities are not
resident. This residual was not visible in requested-capacity arithmetic.

Decision before the final edit: restore retention of edges, positions and
remap/wedge arrays; release the fully rewritten **position-quadric vector q**
on simplify return. Its row layout is unchanged. Keep candidate/order capacity
and initialize only actual picked order entries. Clear q before its fallible
resize so it is initialized exactly once, replacing the old resize-then-fill
on a fresh vector. All counted visits and arithmetic in quadric accumulation
are unchanged. This directly releases the large resident owner while retaining
all the other reusable typed arrays. Its per-call cost is the explicit risk
for the registered six-call 214e microbench; if the bar fails, stop and report
it rather than building an opt-in API or changing the verdict.

The earlier “last candidate” declaration was premature because residency had
not yet been separated from capacity. Every pre-sampling amendment and probe
is retained, and no wall-time sample or timing verdict existed when making
this correction. RESIDENCY.json contains the complete final-pruning inventory.

The residency-based candidate's final diagnostic probe exits 0 at **2,302,956
KiB / 2.196270 GiB**; all stage/output/error records match the original 0.2.5
record exactly. This RSS probe is not timing evidence and does not replace the
registered maximum over ten measured candidate executions. The library source
is frozen in spec027/candidate-source.json before the sole timing campaign.

## Implementation and targeted proof

`src/workspace.rs:118` acquires a chain family and releases prior scratch without
resetting per-call measurements or limits. `src/simplify.rs:270` and `:357`
acquire simplify scratch after validation; `src/cache.rs:95` and FIFO entry
points acquire cache scratch; `src/overdraw.rs:114` acquires overdraw scratch.
`src/simplify.rs:931` releases position-quadric storage, `:1088` initializes
it once, and `:2229` reserves ordering capacity without touching unused pages.
Consecutive same-family calls retain the other typed and ranking capacities.
Simplify releases position quadrics on return before caller pruning, with no
row-layout change. Quadrics are initialized once. Ordering capacity is reserved
at the same fallible upper
bound but initialized only for actual picked candidates.
The RSS-iteration diagnosis above explains this exception. Workspace rustdoc states
the rule; codecs and other families keep their prior behavior. No public API,
quadric layout, limits, output or work-counting change is introduced. Version
numbers remain release-owned; this is the Unreleased 0.2.7 candidate.

The focused test in `src/workspace.rs` directly checks released capacities,
cache/overdraw pointer reuse, fresh-workspace output equality, simplify error
bits, FIFO switching and validation errors. Existing simplify pointer-reuse
coverage also passes, asserting quadric release and pointer reuse for edges,
positions, remap/wedge, collapse-candidate and ordering arrays.

| Required gate | Exit |
|---|---:|
| focused | 0 |
| test | 0 |
| clippy | 0 |
| fmt | 0 |

Initial gates were retained before adding FIFO entry-point coverage. All
required gates were rerun on the final frozen candidate, with the exits above.
Full sweeps and long fuzz remain nightly.

## Registered result

One warm-up pair and ten interleaved measured pairs per comparison, core 8,
same-session A/A floor, paired geometric-mean bootstrap (10,000 resamples,
seed 20261027), exactly as preregistered. No samples were discarded or rerun.
All stage records, scale/error bits, packaged and legacy outputs match; all
six microbench outputs match on every execution. Only elapsed fields differ.

| Workload | A RSS GiB | B RSS GiB | B/A wall GM | upper95 | F | 1.02+F | Wall |
|---|---:|---:|---:|---:|---:|---:|---|
| 214e | 2.850529 | 2.195950 | 1.067203 | 1.197656 | 0.055912 | 1.075912 | FAIL |
| b6fd | 0.993736 | 0.799938 | 1.018136 | 1.120309 | 0.029826 | 1.049826 | FAIL |
| a082 | 3.887260 | 3.100925 | 1.006775 | 1.034114 | 0.084696 | 1.104696 | PASS |
| micro | 1.365520 | 1.333755 | 1.192360 | 1.344914 | 0.057955 | 1.077955 | FAIL |

214e: B peak 2,302,620 KiB / 2.195950 GiB, 1.042094x C++ 1.3 (2,209,608 KiB); 0.654579 GiB below A. RSS bar: PASS.

Registered verdict: **FAIL**.

Timing is full-driver process wall time for chains and a six-call bracket for
the microbench. This is a shared-host result, not a production Moss acceptance
or a GPU result. The C++ RSS reference is the retained step-1 execution; it is
not a fresh timing comparison. Source, binaries, inputs, all samples and
SHA256-verified hardlinked byte sidecars remain in designated scratch.

## Stop decision and options

Stop under the spec's wall-time/kill rule. Memory is within the C++ bar, but
214e and b6fd upper95 exceed their registered allowances, and the six-call
microbench is 1.192360x baseline in geometric mean with upper95 1.344914,
against 1.077955 allowed. This is not a qualified default memory fix. The
shared-host floor and every outlier are retained; no quiet-host inference or
verdict-changing rerun is substituted.

The rejected candidate remains on draft PR #18 for review with all evidence.
Reverting to acquisition-only retention preserves more reuse but its retained
RSS probe is 2.430492 GiB, above 2.318 GiB. Releasing all simplify scratch has
the earlier 2.103016 GiB RSS control but no qualifying timing evidence. An
opt-in caller lifetime API or a different allocation/representation design is
an option for a future specification; neither is built here. No global
allocator trimming, limits change, quadric layout change, Moss change, merge,
tag or publication is performed.

Audit: 176 processes, including 160 measured processes and 16 warm-ups; ten
pairs for each workload/comparison, exact alternating order, one output/error
identity per workload, both source identities verified. All targeted gates
exit 0. The registered campaign exits 1 solely for its FAIL verdict. The
probe-only missing-doc build 101 and earlier full-gate fixture failure 101
remain retained and disclosed; the complete final full gate exits 0.

The designated Cargo target `/mnt/linux-extra/moss-cargo-targets/meshopt-memory`
is deleted; CLEANUP-027.json verifies absence and retained binary hashes. Raw
scratch is `/mnt/linux-extra/moss-scratch/meshopt-memory/spec027/`. Compact
registration, audit, source/input/binary bindings, sample execution receipts,
residency inventory, driver patch, rejected-probe summary and metadata hashes
are committed beside this report. Prior step-1 captures remain unchanged.
