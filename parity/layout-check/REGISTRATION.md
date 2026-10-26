# Bounds layout check registration

Registered 2026-10-25T23:34:19.339193+00:00 before any sample. Candidate a97301e; baseline 775ff8c.

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


Implementation binding: CPU 11; unchanged fix1 old consumer and five inputs (tiny-smooth, medium-smooth, medium-seams, medium-sparse, million-smooth). Calibration ceil(25,000,000/min(single-call ns)), capped 1..1,000,000; alternating five pairs, extending once to twenty iff five-pair Student-t log CI contains one; case geometric mean of paired ratios, family geometric mean of five case GMs. A/A uses the two bounds families on independently archived baseline builds, Bprime/B. Same session, A/A then default then pinned. Thin LTO, codegen-units=1, moss profile as existing validate-once harness; default flags empty, pinned exactly -C llvm-args=-align-all-functions=6. Kernel symbol addresses retained using linker map (strip setting unchanged). Fixed-call controlled-region perf stat instructions:u probes, 10,000 calls except million-smooth 100, as fix1. No new API measurement. Train-first yield between cases, retained PAUSE; one shared infrastructure retry only, no value reruns. All heavy jobs <=4 GB, GPU0, measurement1 for samples; designated target deleted after binary retention. Stop about 90 min.
