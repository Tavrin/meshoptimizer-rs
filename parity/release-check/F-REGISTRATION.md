# Fallback campaign registration

UTC: 2026-10-25T10:29:19.408309+00:00

Candidate campaign completed; registered decision FALLBACK triggered by case regression. Exactly one new campaign on a7688ef0c8d13b8dfeb57c2cb1ce325a76d3dfab vs afe803bbf60796a7af6b5001594a979af009cfac. No candidate rerun. Same binaries/profile bindings, CPU 8, unchanged 204 inputs, 1+12 pairs, pair order F B / B F, bootstrap seed 20261025, 10000 resamples, thresholds and absolute-bar rule from REGISTRATION.md applied unchanged. Coordinator pause/resume rule remains in force.

# Release check registration

Candidate: f40f5af32e7283d1a2b59ea7358a81fe2ecaa982
Baseline: afe803bbf60796a7af6b5001594a979af009cfac
UTC: 2026-10-25T10:07:33.182526+00:00

## Pre-registered measurement and decision (registered 2026-10-25, before any sample; apply verbatim)
- **Arms:** candidate (C) vs 0.2.3 (afe803b, B) directly.
  - Both are Rust, built with the parity fat-LTO profile, scalar-strict.
  - They run in the same session, on the same pinned cores, as interleaved ABBA pairs per case.
  - Use the repo's frozen performance matrix: every family and every case, with its unchanged inputs
    and modes.
  - If the harness can only pair Rust with C++, add a scratch driver, not a repo change. It runs C
    and B as the two backends of each pair, with the harness's case inputs and timing method.
    Record how it does so.
- **Samples per case:** 1 warm-up pair, then exactly 12 measured pairs. There is no adaptive
  stopping.
- **Statistic:**
  - Per case: the median of the 12 paired ratios C/B.
  - Per family: the geometric mean of the case medians.
  - Intervals: bootstrap 95%, 10,000 resamples, seed 20261025. Resample pairs within each case, then
    recompute the case medians and the family GM.
- **Threshold T = 1.02.** It is the same 2% tolerance registered before r1's data, and is not
  derived from any r1 or r2 sample.
  - A family regresses if its GM point estimate is > T.
  - A case regresses if the lower bound of its 95% interval is > T. This covers
    `million/disconnected/allocating` overdraw like every other case.
- **Absolute bars:** family GM ≤1.25 and case max ≤1.50 vs C++ 1.3. They are judged from r2's
  complete final frozen matrix, already registered in the r2 spec, for every case except the named
  one.
- **Infrastructure failure:** a harness timeout or crash, not a measured value, allows one full rerun
  of that job, which is recorded. A second infrastructure failure means NO RELEASE.
- **Verdict:**
  - RELEASE (candidate): no family or case regresses, and the absolute bars hold.
  - FALLBACK: the candidate regresses somewhere. Run this same check once on a7688ef, as a new
    campaign under these exact rules. It releases if it passes. Otherwise the verdict is NO RELEASE.
  - NO RELEASE: anything else.
  - No retuning, and no reruns to change a verdict.

