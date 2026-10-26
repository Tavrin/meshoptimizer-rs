# SPEC-027 lane A registration

Registered before collection of wall-clock samples. Exactly one final campaign;
no reruns, replacement pairs, adaptive stopping, or removal of outliers.

- Baseline: v0.2.6, 33556c4. Candidate: final src/quantize.rs SHA-256 sealed
  together with each executable before collection.
- Export: unchanged bench/peer-screen parity/compare, with scratch manifests
  redirected to this worktree or the archived baseline. Retain original adapters.
- Workloads: quantize-half and quantize-float on tiny-smooth, medium-smooth,
  medium-seams, medium-sparse, million-smooth; byte-identical frozen inputs from
  campaign-20261025T150615Z, checked against exported peer-results/results data.
- Moss profile: opt-level=3, thin LTO, one codegen unit; default SIMD features
  disabled, CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD, no target-cpu flags.
- Select the least-busy allowed physical core via existing quiet.physical_core;
  pin every resident driver to that CPU. No waits on other lanes.
- For every operation/case, use fixed iterations: tiny=10000, medium=500,
  million=5. One warm-up per arm, then 10 interleaved pairs per comparison,
  alternating order. Comparisons: candidate/X, candidate/v0.2.6, candidate/A/A
  (two independently started instances of the exact candidate executable).
- Same-session A/A floor F: maximum absolute deviation from 1 of either endpoint
  of the paired 95% bootstrap interval across all ten A/A operation/cases.
- Bootstrap 10,000, seed 20261029; paired log ratios, geometric means, percentile
  2.5%/97.5% intervals. Family GM weights all five cases equally. Resample pairs
  independently within each case for the family interval.
- Bars per family: candidate/X GM <=1.00; candidate/baseline GM <=1.00;
  each candidate/baseline case upper95 <=1.10+F.
- PASS if all bars pass. PARTIAL if candidate/X misses but both baseline bars
  pass; no parity claim. FAIL if either baseline bar fails. Verdicts retained
  verbatim irrespective of noise. No second campaign to change a verdict.
- All build, instruction, proof, and timing jobs use moss-heavy 4,
  MOSS_HEAVY_GPU=0, lane-local target and scratch. No SIMD paths or API changes.
