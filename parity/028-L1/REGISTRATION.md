# SPEC-028 L1 registration

Registered 2026-10-26 before any wall samples. Exactly one final wall campaign;
no reruns to change verdicts. Earlier instruction-only calls discard timer values.

- Base: v0.2.7 / e516898, archived to scratch. Candidate source and all executable
  hashes sealed before sampling in campaign-identity.json.
- Families: partition, meshlets-scan, meshlets, meshlets-spatial, meshlets-flex,
  spatial-triangles. Frozen bet-0 tiny-smooth, medium-smooth, medium-seams,
  medium-sparse, million-smooth inputs; SHA256s retained in inputs.json.
- a downstream engine scalar profile: opt-level=3, thin LTO, codegen-units=1; Rust SIMD disabled;
  M7 meshopt 0.7.0 / C++ 1.3 with MESHOPTIMIZER_NO_SIMD. X optimesh 1.1.0 is
  disclosed, not a bar. Same slices and output allocation boundaries for all arms.
- Resident drivers pinned to CPU 2 (all arms share that CPU). Timing includes
  output allocation/zeroing, algorithm, scratch and destruction; excludes load,
  serialization and process startup. Workspace reused, as in bet 0.
- Per case/family: one warm-up per arm, then exactly ten interleaved pairs of
  candidate versus M7, v0.2.7 and X separately; alternating AB/BA order. Fixed
  iterations per sample: tiny 5000, medium 16, million 2. No adaptive samples.
- Same-session A/A: two identical v0.2.7 resident drivers, one warm-up and ten
  alternating pairs per case/family, same iteration counts. F is the largest
  absolute distance from 1 of all A/A paired-log bootstrap 95% bounds across the
  30 cases. This single additive floor is used for every regression limit.
- Statistic: each case ratio is exp(mean(log(candidate_ns/comparator_ns))).
  Family GM is the equal-weight geometric mean of its five case ratios.
  Paired bootstrap: 10,000 resamples with replacement, seed 20261030; 2.5/97.5
  percentiles in log space. All raw samples retained; no exclusions.
- Bars, per family, verbatim: ours/M7 GM <=1.00 for PASS; ours/v0.2.7 GM <=1.00;
  every case's upper95 <=1.10+F against v0.2.7. Missing M7 while passing v0.2.7
  gives PARTIAL, a speedup with no performance parity claim. Failing either
  v0.2.7 condition gives FAIL. Proof failures block shipping independently.
- Lane disclosure only: RFC113 S1 41 and valid S2 nine cases, a downstream engine scalar;
  one warm-up plus ten alternating candidate/C++ pairs per case, one DAG per
  sample, using retained cluster-C inputs/configurations and original timer
  boundary. Aggregate = sum of candidate case medians / sum of C++ medians,
  separately S1/S2. No threshold, no reruns, invalid stride32 excluded as before.
- All heavy work admitted with MOSS_HEAVY_GPU=0, declaration at most 4 GiB (2 GiB for serial preparation); measurements
  additionally MOSS_HEAVY_MEASUREMENT=1. Target meshopt-L1 deleted after retaining
  binaries. Scratch retained at <scratch>/meshopt-028-L1.
