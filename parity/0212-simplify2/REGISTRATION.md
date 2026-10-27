# SPEC-0212 simplify2 registration

Entry: clean c396a697 (post-#39 main), perf/simplify-2. No subagents.
No sqrt changes. Every build/check/measurement uses admit.sh: <job-runner> 4 GB,
MOSS_HEAVY_GPU=0, dedicated target meshopt-simplify-2, one compiler job.
Scratch: <scratch>/meshopt-0212-simplify2/.

Four fresh arms: v0.2.11, post39 c396a697, candidate, scalar M7 C++ 1.3.
Frozen 30 corpus cases and 18 real stages, attribution resident-loop harness,
CPU9 perf 6.17 instructions:u, median of three. No wall timings.
Real chains: PASS candidate/base <=0.80; otherwise PARTIAL candidate/post39
<=0.95; otherwise NO-GAIN. Any individual candidate/post39 >1.01 rejects
that candidate before aggregate verdicts. Report candidate/M7 throughout.

Bounds: adjacency n+2m; quadrics 2m+n+[ac>0]*m*(ac+1), disabling the
covered path for fold option 128; normalization n*(1+weights.len()); update
n*(3+ac). Hash batches of at most 64 vertices: b*(table.len()+1), since
each vertex charges one read and at most table.len() probes. All products
are checked; overflow uses the original per-item path. Metered kernels settle
the actual visited prefix on success/error; insufficient budgets retain
original charge granularity and writes. The remap linking pass uses Work.scan.
In attempt 1, input-only finite guards are omitted solely in run_state bounds after validate;
scale keeps them. Extent, normalization, quadric and ranked-error guards stay.

Exact proof compares v0.2.11 Result/output/Usage, all variants/options and every
small work budget, cold/warm and both retention modes. Extended generators
exercise 64-vertex batch boundaries, coincident/repeated wedges, high valence,
duplicates/degenerates, hash clustering, overflow and exact exhaustion. Fuzz
runs for >=300 child CPU seconds. Complete tests, clippy/fmt/boundary and C++
differential required. Preserve failed attempts; do not select favorable samples.

Initial candidate preflight: all-feature tests=0. The first completed real-214e
stage uses 0.863889x base / 0.968409x post39 / 1.617544x M7, below the gain
threshold. Finish and retain this complete exploratory attempt. Refinement
will preserve every sample and rerun all four arms and the complete proof;
no favorable subset or replacement samples will be accepted.

Coordinator public-file rule applied during attempt 1: scripts now read local
paths from environment variables; public evidence uses placeholders. The queued
proof admission was canceled before execution to resubmit with the new environment.
Cancellation exit was 143; no proof result is claimed for that canceled admission. Complete raw receipts
remain private in scratch.

Attempt 1 completed: 576 instruction samples, four arm builds and counter/output
validation exited 0. Real-chain candidate/base, candidate/post39, candidate/M7:
214e 0.863889 / 0.968409 / 1.617544; a082 0.831200 / 0.958221 / 1.484397;
b6fd 0.828533 / 0.958177 / 1.489672. All three are NO-GAIN; no individual
case exceeds 1.01 post39. The unqualified attempt remains private and intact.

Refinement registered: bounded attribute ranking, bitwise finite-predicate
reductions and finite-reciprocal normalization proof. Complete proof and fresh
four-arm campaign are queued through <job-runner>; an operations train-18 hold
prevents admission. No final proof or refined instruction result is claimed
until that queue admits the job and every required check completes.

Final continuation completed: all proof gates, four builds, all 576 fresh samples, instruction gate, source/evidence audit and target cleanup exited 0. Retain the qualified gains: all three real chains PARTIAL, <=0.80 target unmet; see REPORT.md. The historical queued/unqualified status is superseded by the final report.
