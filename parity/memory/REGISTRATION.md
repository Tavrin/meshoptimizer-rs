# registered protocol registered check

Written before any timing sample or candidate-library edit. Diagnostic RSS /
allocation probes are excluded from timing evidence. No reruns to change the
verdict. Source revisions, source hashes, binaries, input identities, order and
all samples are retained under <scratch>/meshopt-memory/spec027.

## Arms and workloads

A: v0.2.5, bea3e04bf22063988a80db4764e80c3dd422a634, unmodified library.
B: candidate in perf/scratch-scope, merged with origin/main f1ca898; source
hashes frozen after the required gates. Version numbers remain release-owned.
Identical release compiler flags and identical step-1 full driver for both.
No allocator probe or per-call clear in timed binaries.

Three complete cooked entity chains: 214e85c568a64e8a, b6fdb93d8a1709e2,
a082ca1fd19bb86a, using step-1 driver, including pruning, packaging and legacy
chain. Use actual vertex/index counts and an input-byte bound covering those buffers
as admission limits (the old full manifest's 1M-vertex placeholder must not turn
these into validation errors).
Fixture paths and SHA256 remain those of the retained source corpus.

Microbench: 214e entity input, six consecutive allocating `simplify` calls on
its original indices and positions through one workspace; target indices
`(index_count / 6) * 3`, target_error 0.01, EMPTY options, default limits.
Read/decode and hash outputs outside its measured region. Measure wall time
of all six calls, retaining all six returned outputs until the bracket ends. Compare exact
output SHA256, index count and error bits for each call.

## Sampling and estimator

One admitted CPU-only <job-runner> job, declared 4 GB, PARITY_RUNNER_MEASUREMENT=1,
core 8 affinity inherited by all children, sequential processes. Same compiler,
inputs, host session and environment. No background measurements in this job.
Workload order: 214e, b6fd, a082, micro. For each workload:

1. A/A floor: one warm-up pair excluded, then ten pairs using the A binary.
   Alternate logical order A1,A2 / A2,A1; label the two identical arms.
2. A/B: one warm-up pair excluded, then ten interleaved pairs; alternate A,B /
   B,A. Samples are immutable and a failed execution is a failed check, not
   an invitation to replace the sample.

Chain wall time is monotonic elapsed process wall time of the full step-1
executable, including the same input/output work in both arms. Peak RSS uses
/usr/bin/time -v. Micro time is its internally bracketed six-call wall time;
process RSS is still recorded. Untimed exact verification follows each run.
Deduplicate identical sidecars after measurement with SHA256 verification and
hardlinks; retain every sample's JSON, stderr, time and execution receipt.

Paired estimator: geometric mean of B/A wall-time ratios. Resample ten whole
pairs with replacement 10,000 times using Python Random(seed=20261027), in
workload order (A/A then A/B); upper95 = sorted bootstrap means at index
ceil(0.95*10000)-1. For each workload, F=max(0, A/A upper95 - 1). Report F and
A/B upper95, with timing acceptance upper95 <= 1.02+F. No outlier removal,
extra pairs, adaptive extension or rerun. RSS uses the maximum measured value
across ten B samples (and max A for the saving), excluding warm-ups.

## Bars and verbatim verdict

- 214e peak RSS <=1.10x C++ 1.3 and <=2.318 GiB, and >=0.50 GiB below A.
  C++ 1.3 retained step-1 reference: 2,209,608 KiB (2.107246 GiB).
- Wall-time upper95 <=1.02+F on all three chains and on the microbench.
- Exact output and errors for every measured and warm-up run, including all
  stage records, scale/error bits, packaged levels and legacy outputs. Only
  explicitly measured elapsed fields are excluded.
- All targeted proof gates exit 0.

Verdict text: "PASS" if and only if every bar passes. Otherwise "FAIL".
Apply that verdict verbatim, with no reruns to change it. Incomplete or failed
execution also yields FAIL, with the reason retained; do not claim qualification.
The long sweeps and fuzz remain nightly.

## Pre-sampling amendment

The acquisition-only candidate failed its diagnostic RSS probe at 2,548,556
KiB, because caller pruning precedes the next family acquisition. Before any
timed sample, release two simplify transient arrays too and change the micro
mesh from b6fd to 214e to expose the largest per-call allocation cost. Preserve
iteration1-REGISTRATION.md and its seal; all bars, sampling and estimators are
unchanged. The old setup is not a completed timing campaign or a verdict.

## Final pre-sampling mechanism amendment

Iteration 2 freed 136,848,048 requested bytes but only 33,272 KiB of RSS.
Keep candidate/order capacities; reserve order's full bound but initialize
only picked entries. Release normalized position and remap/wedge rows on
return instead. Preserve iteration2 registration/seal and probe; retain the
214e six-call microbench and all original bars/estimators. No timed samples
have been collected, and there is no timing verdict to replace.

## Last pre-sampling mechanism amendment

Iteration 3 remains above the RSS bar at 2,452,120 KiB. Release adjacency edges
on return too; they are fully rebuilt each call. Preserve iteration3 source,
registration/seal and RSS probe. This is the final candidate before the one
registered timing run. The 214e six-call microbench, all bars, sample counts
and estimators remain unchanged; no timed sample has been collected.

## Residency-based amendment before the sole timing run

The fourth RSS probe still misses the bar. A mincore probe proves q has
326,328,320 resident bytes; candidate/order capacities each touch only 4 KiB,
and glibc retains 151,855,696 free arena bytes. Restore edge/position/remap/wedge
retention. Release q on simplify return and initialize it once per call; keep
its layout and all arithmetic/visits. Preserve iteration4 registration/seal,
source and probes. This supersedes the premature last-candidate plan, before
any timing sample. Run the registered campaign once on this candidate and
apply its verdict without further tuning. All workloads, bars, sample counts,
bootstrap rules and exact-output/error requirements remain unchanged.
