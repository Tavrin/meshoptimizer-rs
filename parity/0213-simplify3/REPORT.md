# Simplify third pass

**PARTIAL:** all three chains meet candidate/base <=0.90. The candidate/M7 <=1.15 PASS target remains unmet. Keep the qualified gains; no individual case exceeds 1.01. One full campaign, no code edits after its results.

Instruction counts only. Fresh v0.2.12, candidate and scalar M7/C++ 1.3 arms; median of three for 48 cases. Real-chain ratios use sums of six stage medians; corpus summaries use equal-case geometric means. No wall-performance claim.

Gate applied verbatim: PASS if candidate/M7 <=1.15 on all three real chains; PARTIAL if candidate/base <=0.90 on all three; NO-GAIN otherwise. Any case candidate/base >1.01 is FAIL before aggregation. At most two campaigns; no implementation edits after a campaign result except reversions.

Attribution and technique reasoning: [ATTRIBUTION.md](ATTRIBUTION.md) and [DECISIONS.md](DECISIONS.md).

| Family | Cases/stages | candidate/base | candidate/M7 | Verdict |
|---|---:|---:|---:|---|
| real-214e | 6 | 0.857726 | 1.354247 | PARTIAL |
| real-a082 | 6 | 0.879489 | 1.283100 | PARTIAL |
| real-b6fd | 6 | 0.879310 | 1.287363 | PARTIAL |
| simplify | 5 | 0.968722 | 1.082802 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes | 5 | 0.988673 | 1.131154 | WITHIN-REGRESSION-LIMIT |
| simplify-points | 5 | 0.999999 | 1.548011 | WITHIN-REGRESSION-LIMIT |
| simplify-prune | 5 | 1.000000 | 1.504005 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy | 5 | 1.000000 | 1.383381 | WITHIN-REGRESSION-LIMIT |
| simplify-update | 5 | 0.973831 | 1.114968 | WITHIN-REGRESSION-LIMIT |

Overall: **PARTIAL**. Worst individual candidate/base ratio: 1.000000426.

| Family / case | base instructions | candidate | M7 | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| real-214e / real-214e-s01 | 4442078607 | 3810084392 | 2813433364 | 0.857726 | 1.354247 |
| real-214e / real-214e-s02 | 4442078630 | 3810084189 | 2813433503 | 0.857726 | 1.354247 |
| real-214e / real-214e-s05 | 4442078324 | 3810086169 | 2813433946 | 0.857726 | 1.354248 |
| real-214e / real-214e-s09 | 4442078122 | 3810084416 | 2813436112 | 0.857726 | 1.354246 |
| real-214e / real-214e-s10 | 4442078458 | 3810084547 | 2813433646 | 0.857726 | 1.354247 |
| real-214e / real-214e-s13 | 4442078264 | 3810084404 | 2813433296 | 0.857726 | 1.354247 |
| real-a082 / real-a082-s01 | 7283006073 | 6426639522 | 5024122774 | 0.882416 | 1.279157 |
| real-a082 / real-a082-s04 | 6303237197 | 5532974210 | 4304470020 | 0.877799 | 1.285402 |
| real-a082 / real-a082-s07 | 6303236619 | 5532973637 | 4304469526 | 0.877799 | 1.285402 |
| real-a082 / real-a082-s11 | 7283006750 | 6426638701 | 5024123218 | 0.882416 | 1.279156 |
| real-a082 / real-a082-s14 | 6303235251 | 5532973132 | 4304469161 | 0.877799 | 1.285402 |
| real-a082 / real-a082-s17 | 6303234931 | 5532972541 | 4304469053 | 0.877799 | 1.285402 |
| real-b6fd / real-b6fd-s01 | 1818165741 | 1604087762 | 1250251998 | 0.882256 | 1.283012 |
| real-b6fd / real-b6fd-s04 | 1573297240 | 1380738260 | 1070418649 | 0.877608 | 1.289905 |
| real-b6fd / real-b6fd-s07 | 1573297400 | 1380738168 | 1070418923 | 0.877608 | 1.289904 |
| real-b6fd / real-b6fd-s11 | 1818165816 | 1604087934 | 1250251960 | 0.882256 | 1.283012 |
| real-b6fd / real-b6fd-s14 | 1573297208 | 1380738184 | 1070418735 | 0.877608 | 1.289905 |
| real-b6fd / real-b6fd-s17 | 1573297119 | 1380738160 | 1070418693 | 0.877608 | 1.289905 |
| simplify / medium-seams | 16274040 | 15721548 | 14172972 | 0.966051 | 1.109263 |
| simplify / medium-smooth | 12770746 | 12372340 | 10952353 | 0.968803 | 1.129651 |
| simplify / medium-sparse | 14251266 | 13797366 | 11862337 | 0.968150 | 1.163124 |
| simplify / million-smooth | 2553237035 | 2460057947 | 2256448456 | 0.963506 | 1.090234 |
| simplify / tiny-smooth | 115340 | 112705 | 120316 | 0.977155 | 0.936742 |
| simplify-attributes / medium-seams | 30405871 | 30015847 | 26011380 | 0.987173 | 1.153951 |
| simplify-attributes / medium-smooth | 29828254 | 29443664 | 25952031 | 0.987107 | 1.134542 |
| simplify-attributes / medium-sparse | 27839078 | 27587316 | 22980204 | 0.990956 | 1.200482 |
| simplify-attributes / million-smooth | 4187040263 | 4127829639 | 3643746832 | 0.985859 | 1.132853 |
| simplify-attributes / tiny-smooth | 133489 | 132459 | 127353 | 0.992284 | 1.040093 |
| simplify-points / medium-seams | 4390850 | 4390846 | 2944398 | 0.999999 | 1.491254 |
| simplify-points / medium-smooth | 1442300 | 1442300 | 887424 | 0.999999 | 1.625265 |
| simplify-points / medium-sparse | 2447164 | 2447162 | 1465329 | 0.999999 | 1.670042 |
| simplify-points / million-smooth | 197712996 | 197712937 | 124010912 | 1.000000 | 1.594319 |
| simplify-points / tiny-smooth | 12356 | 12356 | 8970 | 1.000000 | 1.377499 |
| simplify-prune / medium-seams | 3031232 | 3031231 | 2026464 | 1.000000 | 1.495823 |
| simplify-prune / medium-smooth | 2742172 | 2742172 | 1855310 | 1.000000 | 1.478013 |
| simplify-prune / medium-sparse | 3924142 | 3924136 | 2545508 | 0.999999 | 1.541592 |
| simplify-prune / million-smooth | 329498585 | 329498470 | 223400058 | 1.000000 | 1.474926 |
| simplify-prune / tiny-smooth | 18787 | 18787 | 12272 | 1.000000 | 1.530903 |
| simplify-sloppy / medium-seams | 7479221 | 7479218 | 5114662 | 1.000000 | 1.462309 |
| simplify-sloppy / medium-smooth | 4225267 | 4225269 | 3113082 | 1.000000 | 1.357262 |
| simplify-sloppy / medium-sparse | 5436333 | 5436328 | 3836460 | 0.999999 | 1.417016 |
| simplify-sloppy / million-smooth | 448021176 | 448021182 | 338170605 | 1.000000 | 1.324838 |
| simplify-sloppy / tiny-smooth | 20644 | 20644 | 15182 | 1.000000 | 1.359780 |
| simplify-update / medium-seams | 18937248 | 18384758 | 16062125 | 0.970825 | 1.144603 |
| simplify-update / medium-smooth | 16145737 | 15747346 | 13586234 | 0.975325 | 1.159066 |
| simplify-update / medium-sparse | 18026962 | 17573073 | 14601021 | 0.974822 | 1.203551 |
| simplify-update / million-smooth | 2976489531 | 2883310424 | 2587707859 | 0.968695 | 1.114233 |
| simplify-update / tiny-smooth | 127959 | 125339 | 129413 | 0.979525 | 0.968519 |

Exit codes: boundary=0, clippy=0, cpp=0, exact-build-no-std=0, exact-build-std=0, exact-lock=0, exact-no-std=0, exact-std=0, fmt=0, fuzz=0, public-text=0, test-all=0, test-no-default=0; three arm builds=0; 432 instruction samples=0; instruction gate=0; evidence audit=0.

Exact v0.2.12 comparisons: 20,401,022 paired observations under std and 20,401,022 under no_std. Result/error bits, complete outputs and unused tails, partial writes, mutable positions/attributes and both Usage fields match. Coverage includes cold/warm workspaces, all 512 option masks, all six simplify families, every small-mesh work limit 0..success+1, both retention modes, hash collisions and batch boundaries, high valence, duplicate/degenerate faces, welded wedges, wide attributes, byte limits, endian byte views, sparse mappings and finite overflow/special values. Pinned C++ differential: 356 comparisons. Exact fuzz: 2,540 seeds, 300.045471 child CPU-seconds, zero mismatches. CPU accounting is a fuzz work budget only.

Protocol: unchanged release resident-loop harness, perf 6.17 instructions:u, CPU9, acknowledged counter enable/disable, 100% counter running for every sample. Scalar Rust opt3/thin-LTO/one codegen unit; scalar C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls, validation and serialization are outside counters. Destination allocation, required copies, scratch acquisition and destruction are inside. Every build and measurement uses shared admission, GPU disabled, 4 GB declaration.

Raw inputs, frozen sources, binaries, annotated profiles, samples, outputs, commands and admissions remain in `<scratch>/meshopt-0213-simplify3/`. Portable bindings and compact instruction/proof records are retained beside this report. No unsafe, API/version, SIMD, threading or other-family changes.

Baseline attribution summary: bounds/normalization uses about 3x C++ instructions,
and welding 1.57-1.59x, across the three chains. Mandatory finite/inverse and
input-validation passes contribute additional residuals. Complete per-function
estimates, inline mapping and limitations are in ATTRIBUTION.md. These are
source/profile attributions, not isolated causal ablation percentages.

Techniques and final source locations:

- `src/simplify.rs:1364`: Full-record sequential welding reader.
- `src/simplify.rs:1389`: Fused original-coordinate bounds reduction and initialized reserved hash/remap/wedge/cache prefixes; unchanged table and probe order.
- `src/simplify.rs:2452`: Extent overflow and the bounds charge remain after classification.
- `src/simplify.rs:2478`: Fixed-length normalization destination; identical arithmetic, guards and visits.
- `src/simplify.rs:536`: Full-record counted validation with original failure-prefix settlement.
- `src/simplify.rs:734`: Exact unsigned absolute-bit maximum finite predicate over the same eleven quadric fields.
- `src/simplify.rs:1293`: Shared adjacency edge prefix.
- `src/simplify.rs:1497`: Disjoint classification tables and checked edge-slice searches, in the original visit order.
- `src/simplify.rs:2250`: Common-prefix positional quadric updates and original in-place loop remapping order.
- `src/simplify.rs:2627`: Checked triangle-record compaction with the original three-unit charge and unused tails.
- `src/input.rs:618`: Expose the existing full-record iterator's exact length internally; no public API or other-family implementation change.

Retained preparation/instrumentation failures: the first public-text check exited
1 for a private profile-section label in the preparation tool; generic section
renaming repaired it to 0 before proof. The initial 100,000-period record stream
throttled: all 36 process exits were 0, but those function samples are rejected.
All 36 replacement profiles at period 1,000,000 pass explicit throttle/loss and
stat-coverage checks. Every gate instruction sample is a separate perf stat
count; none comes from either sampling run. No gate threshold or sample changed.

All operations holds were honored. Four admissions declared 4 GB with GPU
disabled. The runner adapted the final reservation from its recorded history;
the declaration stayed 4 GB. See heavy-receipts.tsv and private admission logs.
Final qualification wrapper exit=0; final audit=0. All source, tool, input, binary,
output and raw-sample hashes are bound; no tests were substituted for a failed gate.

Dedicated target cleanup=0, verified absent after all owned jobs ended.
