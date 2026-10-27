# SPEC bounds-layout-check — old bounds API vs v0.2.4 under the v2 LAYOUT rule (fresh registration)
Kind: perf
Effort: low
Heavy: yes

Repo: meshoptimizer-rs worktree ~/dev/meshoptimizer-rs-wt/validate-once, branch `perf/validate-once` at
a97301e (draft PR #14). You are a fresh agent.
- No source changes under src/.
- You may commit only parity/DECISIONS.md and records under parity/, with author Tavrin and no AI
  trailers, and push.
- <job-runner> with MOSS_HEAVY_GPU=0 and MOSS_HEAVY_MEASUREMENT=1 on timing jobs, declaring ≤4 GB.
- CARGO_TARGET_DIR=<target-dir>, deleted at the end.
- Scratch goes under <scratch>/meshopt-validate-once/layout-check/.
- Train-first yield between cases. No waits on other meshopt lanes. Do not spawn subagents.

## Why (disclosure)
Fix1 (a97301e) gave a registered FAIL (fix1/REPORT.md):
- The new bounds API passed: meshlet-bounds 1.244 and cluster-bounds 0.994 against M7, bar ≤1.25.
- The old API failed wall-clock against v0.2.4: 1.082 and 1.071, against a limit of 1.043.
- Yet the old API's instruction counts are identical to v0.2.4: 1.000 and 1.001.

That pattern is the LAYOUT class defined by the release-check v2 methodology
(SPEC-qx5g3-release-check-v2.md). The old-API check never had that rule. This check applies it. It
was written after the a97301e result and approved by the a downstream engine coordinator on 2026-10-26, and it is
disclosed as such in parity/DECISIONS.md before any sample. The new-API result stands and is not
re-measured.

## Pre-registered measurement and decision (write layout-check/REGISTRATION.md and the DECISIONS.md entry before any sample; apply verbatim)
- **Families:** meshlet-bounds and cluster-bounds, old API only: the existing
  `compute_meshlet_bounds` and `compute_cluster_bounds`, at a97301e against v0.2.4 (775ff8c). Use
  validate-once's harness, cases and pair protocol unchanged.
- **Instructions:** fixed-call `perf stat -e instructions:u` probes, as fix1 ran them, give a family
  instruction GM, old against v0.2.4.
- **A/A floor:** two builds of v0.2.4 from identical source in different paths, same session.
  F = the largest |family GM − 1| of A/A. T = ceil_to_0.001(1 + F + 0.02).
- **Wall-clock:**
  - Default build: family GM, old/v0.2.4, point estimate.
  - Alignment-pinned build: both arms rebuilt with `-C llvm-args=-align-all-functions=6`, the
    option v2 qualified. Same measurement.
- **Verdict per family:**
  - REGRESSION if the instruction GM is above 1.01, or if both the default and the pinned wall GM
    are above T.
  - LAYOUT if the instruction GM is 1.01 or below, the default wall GM is above T, and the pinned
    wall GM is T or below.
  - PASS otherwise.
- **Outcome:** both families PASS or LAYOUT means bounds is releasable in the next 0.2.x, together
  with the new-API result already measured. Any REGRESSION means bounds waits for new code.
- One infrastructure rerun is allowed. No retuning, and no reruns to change a verdict.

## Proof
The proof fails if any of these is true:
- REGISTRATION.md or the DECISIONS.md entry post-dates a sample;
- the A/A floor is missing;
- the pinned build doesn't record its flag and the symbol alignment of the two bounds kernels, as
  v2 did;
- the verdict is not applied verbatim.

## Done-when / Stop condition
layout-check/REPORT.md holds:
- instruction GMs, F and T;
- the default and pinned wall GMs;
- each family's verdict.

Then push the parity/ records and record the result in ~/dev/specs/meshopt/RESUME-2026-10-24.md
under "bounds layout check". Stop at about 90 min.

## Non-goals
src/ changes, the new API, fetch-remap, and the 0.2.5 release.

## Final message
Token-lean:
- the per-family table (instructions, default wall, pinned wall, verdict);
- F and T;
- receipts.

Never end on a question.
