# Simplify third-pass registration

Entry: clean fe108a0, perf/simplify-3, release v0.2.12. No subagents.
All builds and measurements require the shared job runner, 4 GB declaration,
GPU disabled, one compiler job, dedicated disposable target directory.
Machine-specific paths and runner adaptation stay in private scratch.

Attribution first: fresh release and scalar M7 binaries; perf 6.17
instructions:u stat, record and annotate for all 18 frozen real-chain stages.
Per-function sampled instruction estimates are diagnostic, with source and
assembly analysis identifying layout, extra traversals, checks and codegen.
Attribution is bounded to 1.5 active hours, excluding admission holds.

Gate: fresh base (v0.2.12), candidate and M7 (C++ 1.3), unchanged release
resident-loop harness and frozen 30 simplify-family corpus cases plus 18 real
stages, CPU9, acknowledged counter enable/disable, median of three per arm.
Real-chain ratios use sums of six stage medians. Corpus summaries use equal-case
geometric means. No wall timings. Every finished sample appends a record and is
skipped on restart; interrupted artifacts are retained separately.

Real chains: PASS if candidate/M7 <=1.15 on all three; PARTIAL if
candidate/base <=0.90 on all three; NO-GAIN otherwise. Any individual case
candidate/base >1.01 is FAIL: fix it or revert that change. No code edits after
a campaign's results except to revert; at most two full campaigns.

Output bytes, Result/error precedence, Usage and partial writes must match
v0.2.12 exactly. No new unsafe, unchecked access, API changes, altered charge
position, SIMD, multithreading or changes to other families. Full tests under
both feature modes, strict clippy, fmt, boundary/public-text checks, exhaustive
small work limits/options/variants, C++ differential and five child CPU minutes
of widened exact fuzzing are required. Stop near six active hours excluding
holds; report unmet targets explicitly.
