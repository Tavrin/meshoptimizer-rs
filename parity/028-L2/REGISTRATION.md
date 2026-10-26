# SPEC-028 L2 — single wall campaign registration

Registered before any L2 wall sample. Baseline v0.2.7 =
`e5168983067a5decfc5bf15605c7dae7113dba49`; comparator exported from
`bench/peer-screen` = `e8827e4de9b69c98b9a96c6f49f8192a6b045ff7`.

- Families: cache, cache-fifo (16), simplify-scale, overdraw (1.05 or captured
  threshold). Frozen cases: tiny-smooth, medium-smooth, medium-seams,
  medium-sparse, million-smooth. Overdraw additionally includes the retained
  real-214e-overdraw, real-b6fd-overdraw and real-a082-overdraw inputs and their
  captured thresholds. Each case has equal weight within its family.
- Profile: Moss scalar, opt-level 3, thin LTO, one codegen unit; no default
  SIMD features; C++ `MESHOPTIMIZER_NO_SIMD`. Same input preparation and public
  allocating output boundaries for candidate, baseline, M7 and X.
- CPU 9, single selected logical CPU; retain sibling topology and affinity.
  Every comparison uses two resident processes pinned to that CPU. No waits on
  other lanes, no quiet-window selection, no timing-based exclusions.
- Comparisons, in order: v0.2.7/v0.2.7 A/A, candidate/M7 (meshopt 0.7.0,
  C++ 1.3), candidate/v0.2.7, candidate/X (optimesh 1.1.0, disclosure only).
  Every family/case: one warm-up pair, then exactly ten alternating AB/BA
  pairs. The sole warm-up pair also calibrates the iteration count: one
  timed call per arm estimates a 25 ms minimum arm batch (1..1,000,000
  iterations). There is no extra warm-up batch or pilot wall campaign.
- Paired ratios are candidate/comparator, geometric mean of ten ratios.
  Family GM is the equal-case geometric mean. Bootstrap paired log ratios
  within each case, 10,000 resamples, seed 20261030; percentile endpoints
  use sorted indices 249 and 9749 (95%). Family intervals resample within
  each case and keep all cases with equal weights.
- Same-session floor F = maximum absolute distance from 1 among all A/A
  case interval endpoints, over the four families. No subtraction from ratios.
- Per-family PASS: candidate/M7 GM <= 1.00 AND candidate/v0.2.7 GM <= 1.00
  AND each case's upper95 against v0.2.7 <= 1.10 + F.
  PARTIAL: M7 bar missed but both baseline bars met. FAIL: any baseline bar
  missed. Apply these literally, including results near equality. X is not a bar.
- Lane disclosure: replay all scale_raw, simplify, cache and overdraw calls in
  the three retained r2 real-mesh chains (214e, b6fd, a082), with their captured
  stride-64 vertices, input indices and parameters. For pruned packaging,
  recover the rescaled vertex buffer by inverting the captured compacted
  index mapping and propagating captured XYZ bytes to the corresponding
  welded-position classes; all other bytes remain the original capture.
  The no-clock three-way proof must reproduce every historical stage hash. Reuse the r2 stage adapter
  semantics and sum stage times over each chain. Timed reorder replay includes
  the saved-input clone, allocating result and output copy in both arms;
  serialization and returned-result destruction are outside the interval. One warm-up plus ten
  interleaved pairs, candidate/M7 and baseline/M7; disclose paired stage GM
  and bootstrap intervals, no acceptance bar. The immutable stage traces come
  from the named r2 harness; replay excludes importer, serialization and the
  rest of Moss, and must not be described as whole-application timing.
- Seal candidate source hashes, registration hash, binary and input hashes
  before the first sample. Exactly one campaign; no reruns to change verdicts.
  On infrastructure failure retain partial samples and disclose incomplete
  evidence. No release, merge, tag or version bump.
