# SPEC-027 lane C: single final wall-clock campaign

Written before any wall-clock sample. Instruction experiments discard the
resident driver's elapsed-time replies and use only `instructions:u`.

Candidate versus optimesh 1.1.0 (X), and candidate versus v0.2.6 (33556c4).
Exported `bench/peer-screen` adapters and input files are frozen; only scratch
manifest dependency paths and lockfiles are adapted. a downstream engine profile: opt-level 3,
thin LTO, one codegen unit, scalar adapters, no peer SIMD features, C++ NO_SIMD.

Families: adjacency, optimize-meshlet, fetch, remap-indices. Each uses all five
frozen cases: tiny-smooth, medium-smooth, medium-seams, medium-sparse,
million-smooth. In particular the frozen optimize-meshlet adapter optimizes
one triangle and at most 64 vertices; no broader meshlet timing claim follows.
The remap-indices adapter includes vertex-remap generation as well as applying it.

Pin every resident driver to CPU 7. First measure same-session baseline versus
an independently running identical baseline driver, on all 20 family/case cells.
For each cell/comparison, calibrate once to at least 25 ms per arm (maximum
1,000,000 iterations), run one unrecorded warm-up pair, then exactly ten
interleaved pairs, alternating AB and BA. No adaptive stopping or retries.
Calibration is part of this one final campaign, never used in instruction tuning.

Ratios are candidate time / comparator time. Case point estimates are the
geometric mean of ten paired ratios. Compute percentile 95% bootstrap intervals
using 10,000 resamples and seed 20261029. Family GM weights the five cases equally.
Bootstrap the family by independently resampling pairs within each fixed case.
F = maximum across all A/A cells of max(abs(lower95 - 1), abs(upper95 - 1)).
Report this floor without modifying any samples.

Bars, per family, verbatim:
- ours/X GM <=1.00;
- ours/v0.2.6 GM <=1.00;
- every case's upper95 <=1.10+F against v0.2.6.

PASS requires all three bars. A family that misses ours/X but passes against
v0.2.6 is PARTIAL. Any baseline GM or per-case bar failure is FAIL. Any missing
measurement is UNMEASURED. Retain and report the first verdict; no reruns to
change it. Proof failures block shipping independently of performance verdicts.

All execution uses <job-runner>, MOSS_HEAVY_GPU=0, declaration <=4 GB; target
`<target-dir>` is deleted after retaining
executables and receipts. Scratch: `<scratch>/meshopt-peer-c`.
