# SPEC-0212 simplify2
Instruction counts only. Fresh v0.2.11 (40dbf9e), post-#39 (c396a697), candidate and scalar M7/C++ 1.3 arms; median of three for all 48 cases. Real-chain ratios use sums of six stage medians; corpus summaries use equal-case geometric means. No wall-performance claim.

Gate applied verbatim: real-chain PASS when candidate/v0.2.11 <=0.80; otherwise PARTIAL when candidate/post-#39 <=0.95; otherwise NO-GAIN. Any individual candidate/post-#39 >1.01 is FAIL before aggregation.

Techniques (src/simplify.rs):
- src/simplify.rs:317: Validated-only input finite-check elimination; scale retains validation and extent overflow is checked in both paths.
- src/simplify.rs:1268: Shared vertex-length remap/counter slices and bounded phase accounting.
- src/simplify.rs:1372: Bounded hash batches; exact visited-prefix settlement with checked fallback.
- src/simplify.rs:1421: Iterator over remap entries and a fixed-length wedge slice.
- src/simplify.rs:2364: Finite-reciprocal normalization proof; reciprocal overflow retains the original post-write guard.
- src/simplify.rs:1953: Batched attribute ranking bounded by complete wedge traversals.
- src/simplify.rs:725: Non-short-circuit finite reductions preserve the predicate and error position.
- src/simplify.rs:2367: Bounded normalization and attribute input accounting; overflow remains checked on the unproven path.
- src/simplify.rs:2419: Bounded quadric accounting; folding keeps the checked path.
- src/simplify.rs:2513: Bounded update accounting; finite checks stay at their original position.

Bounds: adjacency n+2m; quadrics 2m+n plus m*(ac+1) when ac>0; normalization n*(1+weights.len()); updates n*(3+ac); a hash batch b<=64 charges at most b*(table.len()+1). Hash probes run over 0..table.len(), preceded by one vertex read charge. Quadric triangles charge m, regularization n, edges m and attributes (m/3)*(3+3ac). Updates charge n vertices, at most n*ac attributes and two n-entry loop passes. Option 128 disables quadric prepayment. Checked bound overflow or insufficient remaining work selects the original per-item path. Metered success/error settles only the actual visits, preserving failure prefixes.

Positional rank retains the #39 kernel/accounting with a bitwise finite predicate; picker is unchanged. Attribute rank uses batches of at most 64 candidates, each bounded by 1+2*ac*(n+1): one candidate visit, 2ac base visits and at most two n-entry wedge traversals (or 2ac seam visits). Sqrt, public API, shared math, other families and unsafe boundaries are unchanged. For normalization, finite input validation and finite bounds imply each rounded coordinate difference lies in [0, extent]. A finite rounded reciprocal gives extent*scale <2, hence finite products. The guard is omitted only on this proven path; reciprocal overflow retains it at the original post-charge/post-write position. Independent exponent checks use bitwise reductions without removing any arithmetic guard. Quadric/rank/update guards likewise remain.

| Family | Cases/stages | candidate/v0.2.11 | candidate/post-#39 | candidate/M7 | Verdict |
|---|---:|---:|---:|---:|---|
| real-214e | 6 | 0.843240 | 0.945262 | 1.578882 | PARTIAL |
| real-a082 | 6 | 0.816931 | 0.941772 | 1.458915 | PARTIAL |
| real-b6fd | 6 | 0.814288 | 0.941704 | 1.464061 | PARTIAL |
| simplify | 5 | 0.915211 | 0.969013 | 1.117740 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes | 5 | 0.943160 | 0.976257 | 1.144094 | WITHIN-REGRESSION-LIMIT |
| simplify-points | 5 | 0.998949 | 0.999999 | 1.548023 | WITHIN-REGRESSION-LIMIT |
| simplify-prune | 5 | 1.000000 | 1.000000 | 1.504011 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy | 5 | 0.942415 | 1.000000 | 1.383384 | WITHIN-REGRESSION-LIMIT |
| simplify-update | 5 | 0.895131 | 0.973986 | 1.144930 | WITHIN-REGRESSION-LIMIT |

| Family / case | v0.2.11 instructions | post-#39 | candidate | M7 | candidate/v0.2.11 | candidate/post-#39 | candidate/M7 | Verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| real-214e / real-214e-s01 | 5267867098 | 4699310996 | 4442077901 | 2813432827 | 0.843240 | 0.945262 | 1.578882 | PARTIAL |
| real-214e / real-214e-s02 | 5267868324 | 4699311264 | 4442078151 | 2813432986 | 0.843240 | 0.945262 | 1.578882 | PARTIAL |
| real-214e / real-214e-s05 | 5267867244 | 4699311375 | 4442078239 | 2813433001 | 0.843240 | 0.945262 | 1.578882 | PARTIAL |
| real-214e / real-214e-s09 | 5267867277 | 4699312851 | 4442079543 | 2813433151 | 0.843241 | 0.945262 | 1.578882 | PARTIAL |
| real-214e / real-214e-s10 | 5267868519 | 4699312341 | 4442079334 | 2813433286 | 0.843240 | 0.945262 | 1.578882 | PARTIAL |
| real-214e / real-214e-s13 | 5267868974 | 4699313725 | 4442080862 | 2813433623 | 0.843241 | 0.945262 | 1.578882 | PARTIAL |
| real-a082 / real-a082-s01 | 8868553271 | 7754626604 | 7283005832 | 5024124524 | 0.821217 | 0.939182 | 1.449607 | PARTIAL |
| real-a082 / real-a082-s04 | 7739011784 | 6682290827 | 6303235428 | 4304468309 | 0.814475 | 0.943275 | 1.464347 | PARTIAL |
| real-a082 / real-a082-s07 | 7739011585 | 6682290700 | 6303234114 | 4304467737 | 0.814475 | 0.943274 | 1.464347 | PARTIAL |
| real-a082 / real-a082-s11 | 8868551349 | 7754624605 | 7283004602 | 5024122345 | 0.821217 | 0.939182 | 1.449607 | PARTIAL |
| real-a082 / real-a082-s14 | 7739013871 | 6682292233 | 6303235524 | 4304468370 | 0.814475 | 0.943274 | 1.464347 | PARTIAL |
| real-a082 / real-a082-s17 | 7739012252 | 6682290981 | 6303234429 | 4304468106 | 0.814475 | 0.943274 | 1.464347 | PARTIAL |
| real-b6fd / real-b6fd-s01 | 2220548038 | 1936040349 | 1818165186 | 1250251718 | 0.818791 | 0.939115 | 1.454239 | PARTIAL |
| real-b6fd / real-b6fd-s04 | 1938252363 | 1668031598 | 1573296738 | 1070418254 | 0.811709 | 0.943206 | 1.469796 | PARTIAL |
| real-b6fd / real-b6fd-s07 | 1938252410 | 1668031641 | 1573296718 | 1070418246 | 0.811709 | 0.943206 | 1.469796 | PARTIAL |
| real-b6fd / real-b6fd-s11 | 2220548073 | 1936040360 | 1818165271 | 1250251543 | 0.818791 | 0.939115 | 1.454240 | PARTIAL |
| real-b6fd / real-b6fd-s14 | 1938252374 | 1668031610 | 1573296624 | 1070418211 | 0.811709 | 0.943206 | 1.469796 | PARTIAL |
| real-b6fd / real-b6fd-s17 | 1938252354 | 1668031583 | 1573296670 | 1070418236 | 0.811709 | 0.943206 | 1.469796 | PARTIAL |
| simplify / medium-seams | 18246609 | 16822066 | 16274012 | 14172934 | 0.891892 | 0.967421 | 1.148246 | WITHIN-REGRESSION-LIMIT |
| simplify / medium-smooth | 13670347 | 13164013 | 12770693 | 10952310 | 0.934189 | 0.970122 | 1.166027 | WITHIN-REGRESSION-LIMIT |
| simplify / medium-sparse | 15365103 | 14858778 | 14251225 | 11862300 | 0.927506 | 0.959112 | 1.201388 | WITHIN-REGRESSION-LIMIT |
| simplify / million-smooth | 2909138026 | 2621759358 | 2553236876 | 2256449028 | 0.877661 | 0.973864 | 1.131529 | WITHIN-REGRESSION-LIMIT |
| simplify / tiny-smooth | 121820 | 118330 | 115328 | 120316 | 0.946708 | 0.974630 | 0.958543 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-seams | 32527047 | 31123638 | 30405828 | 26011348 | 0.934786 | 0.976937 | 1.168945 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-smooth | 31187993 | 30456552 | 29828218 | 25951993 | 0.956401 | 0.979370 | 1.149361 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-sparse | 29337951 | 28728864 | 27839046 | 22980161 | 0.948909 | 0.969027 | 1.211438 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / million-smooth | 4549660593 | 4273880044 | 4187039776 | 3643746554 | 0.920297 | 0.979681 | 1.149103 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / tiny-smooth | 139632 | 136716 | 133477 | 127353 | 0.955920 | 0.976308 | 1.048087 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-seams | 4390870 | 4390801 | 4390801 | 2944351 | 0.999984 | 1.000000 | 1.491263 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-smooth | 1442324 | 1442260 | 1442255 | 887381 | 0.999952 | 0.999996 | 1.625295 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-sparse | 2447187 | 2447121 | 2447120 | 1465288 | 0.999973 | 1.000000 | 1.670061 | WITHIN-REGRESSION-LIMIT |
| simplify-points / million-smooth | 197712639 | 197712545 | 197712581 | 124010472 | 1.000000 | 1.000000 | 1.594322 | WITHIN-REGRESSION-LIMIT |
| simplify-points / tiny-smooth | 12420 | 12356 | 12356 | 8970 | 0.994847 | 1.000000 | 1.377502 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-seams | 3031192 | 3031191 | 3031189 | 2026426 | 0.999999 | 0.999999 | 1.495830 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-smooth | 2742130 | 2742127 | 2742132 | 1855266 | 1.000001 | 1.000002 | 1.478026 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-sparse | 3924095 | 3924094 | 3924095 | 2545466 | 1.000000 | 1.000000 | 1.541602 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / million-smooth | 329498153 | 329498180 | 329498058 | 223399799 | 1.000000 | 1.000000 | 1.474925 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / tiny-smooth | 18787 | 18787 | 18787 | 12272 | 1.000000 | 1.000000 | 1.530904 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-seams | 7702344 | 7479182 | 7479181 | 5114626 | 0.971027 | 1.000000 | 1.462312 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-smooth | 4389286 | 4225229 | 4225224 | 3113037 | 0.962622 | 0.999999 | 1.357267 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-sparse | 5600346 | 5436284 | 5436286 | 3836414 | 0.970705 | 1.000000 | 1.417023 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / million-smooth | 524995200 | 448021047 | 448021046 | 338170420 | 0.853381 | 1.000000 | 1.324838 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / tiny-smooth | 21503 | 20644 | 20644 | 15182 | 0.960051 | 0.999999 | 1.359781 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-seams | 21627567 | 19485274 | 18937202 | 16062086 | 0.875605 | 0.971872 | 1.179000 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-smooth | 17936631 | 16539032 | 16145688 | 13586190 | 0.900152 | 0.976217 | 1.188390 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-sparse | 20040554 | 18634506 | 18026918 | 14600984 | 0.899522 | 0.967394 | 1.234637 | WITHIN-REGRESSION-LIMIT |
| simplify-update / million-smooth | 3488832021 | 3045010941 | 2976488872 | 2587707735 | 0.853148 | 0.977497 | 1.150242 | WITHIN-REGRESSION-LIMIT |
| simplify-update / tiny-smooth | 134678 | 130973 | 127959 | 129413 | 0.950110 | 0.976988 | 0.988765 | WITHIN-REGRESSION-LIMIT |

Exit codes: boundary=0, clippy=0, cpp=0, exact-build-no-std=0, exact-build-std=0, exact-lock=0, exact-no-std=0, exact-std=0, fmt=0, fuzz=0, test-all=0, test-no-default=0; four arm builds=0; 576 stat samples=0; instruction gate=0; evidence audit=0.

Exact v0.2.11 comparisons: 14,170,418 paired observations under std and 14,170,418 under no_std, including Result/error bits, full outputs and unused tails, partial writes, mutable positions/attributes and both Usage fields. Cold/warm workspaces, all 512 option masks, every small-mesh work limit 0..success+1, both retention modes, deliberate hash collisions, >64-vertex batches, high valence, duplicate/degenerate faces, welded wedges, 32 attributes, byte limits, little/big endian byte readers, sparse mappings and finite overflow/special values. Finite-reciprocal bound sweep: 2,139,095,040 nonnegative finite extent bit patterns in each exact mode, zero violations. Pinned C++ differential: 356 comparisons. Widened differential fuzz: 3,396 seeds / 217,344 generated cases, 300.126074 child CPU-seconds, zero mismatches. Fuzz CPU accounting is a work budget only.

Retained attempt 1: all four arm builds and 576 samples exited 0, with all three real chains NO-GAIN. Its all-feature preflight exited 0. The queued proof admission was canceled before execution (143) to adopt environment-only tooling; no proof pass is attributed to it. A report-tool syntax preflight exited 1 and was repaired to 0 before qualification; the private repair receipt is retained. Final proof and measurement use the refined source and independent fresh samples.

Protocol: attribution resident-loop harness, perf 6.17 instructions:u on CPU9, acknowledged counter enable/disable; all samples have 100% running. Scalar Rust opt3/thin-LTO/one codegen unit; scalar M7 C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls, validation and serialization are outside counters. Destination allocation, required copies, scratch acquisition and destruction are inside. Every build/measurement uses `<job-runner>` with MOSS_HEAVY_GPU=0 and a 4 GB declaration.

Full inputs, frozen sources, binaries, raw counters/output buffers, commands, logs and admissions are retained in `<scratch>/meshopt-0212-simplify2/`; binding.json and instructions.tsv retain independently checked hashes and compact counts. No thresholds, exhaustion charge granularity or observable error/partial-write order changed. No wall timings, API/version changes, new unsafe or sqrt changes.

Decision: retain the qualified gains. All three real chains are PARTIAL; the <=0.80 v0.2.11 PASS target remains unmet. Worst individual candidate/post-#39 ratio is 1.000001641, below the strict >1.01 rejection threshold. No sample was replaced or selected.

All 3 admitted jobs declared 4 GB with GPU disabled. Maximum process-tree PSS: 1.4 GB; maximum total cgroup peak including cache: 6.0000 GB. Numeric admission receipts are in admissions.json; complete machine-specific logs remain private. Target cleanup=0, verified absent (cleanup.json). The candidate source snapshot and executables remain in scratch.

Memory boundary: the <=4 GB declaration requirement was met and process-tree PSS stayed below 4 GB. A strict 4 GB total-cgroup ceiling was not maintained: charged file cache raised the peak to the wrapper cap of 6 GB. This is retained explicitly; process PSS is not substituted for total cgroup memory.
