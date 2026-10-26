# SPEC-0212 simplify
Fresh user-instruction counts only. Median of three per arm. Real chains use the sum of six stage medians; corpus summaries use the equal-case geometric mean. Per-case regression rejection is applied before aggregation.
Techniques: simplify-local std sqrt for nonnegative finite operands, portable special-value/no_std fallback; std memo removal; shared-length picker/quadric slices; bounded prepaid picker with original low-budget fallback. All arithmetic finite guards remain.
Source locations:
- src/simplify.rs:17: `fn sqrt(value:`
- src/simplify.rs:31: `struct SqrtCache;`
- src/simplify.rs:1651: `fn quadrics<M:`
- src/simplify.rs:1787: `fn pick<M:`
- src/simplify.rs:2408: `let nc = metered!`
- src/simplify.rs:2534: `fn simplify_sqrt_exhaustive_identity`
- src/simplify_extra.rs:1106: sloppy scratch uses the simplify-local cache adapter.

Picker bound: each entered complete triangle charges exactly three corners, is entered once, and the capacity stop precedes entry. Visits are therefore at most the current index count. The covered path settles the exact tally; the insufficient-budget path retains the original checks and failure prefixes. There are no fallible arithmetic or reader callbacks in pick. See REGISTRATION.md and DECISIONS.md for the retained reasoning and worst-case sweep.

| Family | Cases/stages | candidate/base | candidate/M7 | Verdict |
|---|---:|---:|---:|---|
| real-214e | 6 | 0.892071 | 1.670312 | PARTIAL |
| real-a082 | 6 | 0.867441 | 1.549117 | PARTIAL |
| real-b6fd | 6 | 0.864697 | 1.554694 | PARTIAL |
| simplify | 5 | 0.944439 | 1.153435 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes | 5 | 0.966132 | 1.171960 | WITHIN-REGRESSION-LIMIT |
| simplify-points | 5 | 0.998950 | 1.548028 | WITHIN-REGRESSION-LIMIT |
| simplify-prune | 5 | 1.000000 | 1.504012 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy | 5 | 0.942416 | 1.383386 | WITHIN-REGRESSION-LIMIT |
| simplify-update | 5 | 0.919039 | 1.175510 | WITHIN-REGRESSION-LIMIT |

| Family / case | base instructions | candidate instructions | M7 instructions | candidate/base | candidate/M7 | Verdict |
|---|---:|---:|---:|---:|---:|---|
| real-214e / real-214e-s01 | 5267867698 | 4699310689 | 2813432752 | 0.892071 | 1.670312 | PARTIAL |
| real-214e / real-214e-s02 | 5267866817 | 4699310958 | 2813432849 | 0.892071 | 1.670312 | PARTIAL |
| real-214e / real-214e-s05 | 5267867062 | 4699310737 | 2813432747 | 0.892071 | 1.670312 | PARTIAL |
| real-214e / real-214e-s09 | 5267866869 | 4699310690 | 2813432637 | 0.892071 | 1.670312 | PARTIAL |
| real-214e / real-214e-s10 | 5267867346 | 4699310688 | 2813432664 | 0.892071 | 1.670312 | PARTIAL |
| real-214e / real-214e-s13 | 5267866935 | 4699310858 | 2813433142 | 0.892071 | 1.670312 | PARTIAL |
| real-a082 / real-a082-s01 | 8868551857 | 7754624171 | 5024123013 | 0.874396 | 1.543478 | PARTIAL |
| real-a082 / real-a082-s04 | 7739011809 | 6682291099 | 4304468332 | 0.863455 | 1.552408 | PARTIAL |
| real-a082 / real-a082-s07 | 7739012434 | 6682290768 | 4304467992 | 0.863455 | 1.552408 | PARTIAL |
| real-a082 / real-a082-s11 | 8868551074 | 7754624872 | 5024126420 | 0.874396 | 1.543477 | PARTIAL |
| real-a082 / real-a082-s14 | 7739012156 | 6682291687 | 4304468286 | 0.863455 | 1.552408 | PARTIAL |
| real-a082 / real-a082-s17 | 7739011929 | 6682290875 | 4304468295 | 0.863455 | 1.552408 | PARTIAL |
| real-b6fd / real-b6fd-s01 | 2220547916 | 1936040192 | 1250251374 | 0.871875 | 1.548521 | PARTIAL |
| real-b6fd / real-b6fd-s04 | 1938252287 | 1668031526 | 1070418115 | 0.860585 | 1.558299 | PARTIAL |
| real-b6fd / real-b6fd-s07 | 1938252816 | 1668031487 | 1070418196 | 0.860585 | 1.558299 | PARTIAL |
| real-b6fd / real-b6fd-s11 | 2220547893 | 1936040228 | 1250251435 | 0.871875 | 1.548521 | PARTIAL |
| real-b6fd / real-b6fd-s14 | 1938252262 | 1668031452 | 1070418137 | 0.860585 | 1.558299 | PARTIAL |
| real-b6fd / real-b6fd-s17 | 1938252256 | 1668031524 | 1070418107 | 0.860585 | 1.558299 | PARTIAL |
| simplify / medium-seams | 18246599 | 16822024 | 14172915 | 0.921927 | 1.186914 | WITHIN-REGRESSION-LIMIT |
| simplify / medium-smooth | 13670334 | 13163999 | 10952306 | 0.962961 | 1.201939 | WITHIN-REGRESSION-LIMIT |
| simplify / medium-sparse | 15365088 | 14858756 | 11862286 | 0.967047 | 1.252605 | WITHIN-REGRESSION-LIMIT |
| simplify / million-smooth | 2909136966 | 2621758462 | 2256447886 | 0.901215 | 1.161896 | WITHIN-REGRESSION-LIMIT |
| simplify / tiny-smooth | 121820 | 118306 | 120316 | 0.971154 | 0.983294 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-seams | 32527030 | 31123628 | 26011324 | 0.956854 | 1.196541 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-smooth | 31187980 | 30456524 | 25951978 | 0.976547 | 1.173572 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / medium-sparse | 29337936 | 28728855 | 22980149 | 0.979239 | 1.250160 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / million-smooth | 4549660487 | 4273880296 | 3643746741 | 0.939384 | 1.172936 | WITHIN-REGRESSION-LIMIT |
| simplify-attributes / tiny-smooth | 139632 | 136740 | 127353 | 0.979288 | 1.073709 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-seams | 4390855 | 4390792 | 2944340 | 0.999986 | 1.491266 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-smooth | 1442313 | 1442249 | 887366 | 0.999956 | 1.625314 | WITHIN-REGRESSION-LIMIT |
| simplify-points / medium-sparse | 2447177 | 2447106 | 1465275 | 0.999971 | 1.670066 | WITHIN-REGRESSION-LIMIT |
| simplify-points / million-smooth | 197712489 | 197712440 | 124010345 | 1.000000 | 1.594322 | WITHIN-REGRESSION-LIMIT |
| simplify-points / tiny-smooth | 12420 | 12356 | 8970 | 0.994847 | 1.377503 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-seams | 3031186 | 3031179 | 2026417 | 0.999998 | 1.495832 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-smooth | 2742117 | 2742119 | 1855256 | 1.000001 | 1.478027 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / medium-sparse | 3924085 | 3924084 | 2545458 | 1.000000 | 1.541603 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / million-smooth | 329497910 | 329497941 | 223399622 | 1.000000 | 1.474926 | WITHIN-REGRESSION-LIMIT |
| simplify-prune / tiny-smooth | 18787 | 18787 | 12272 | 1.000000 | 1.530905 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-seams | 7702328 | 7479167 | 5114603 | 0.971027 | 1.462316 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-smooth | 4389273 | 4225216 | 3113027 | 0.962623 | 1.357270 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / medium-sparse | 5600332 | 5436275 | 3836404 | 0.970706 | 1.417024 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / million-smooth | 524994918 | 448020615 | 338170048 | 0.853381 | 1.324838 | WITHIN-REGRESSION-LIMIT |
| simplify-sloppy / tiny-smooth | 21503 | 20644 | 15182 | 0.960051 | 1.359782 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-seams | 21627553 | 19485261 | 16062072 | 0.900946 | 1.213122 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-smooth | 17936622 | 16539036 | 13586181 | 0.922082 | 1.217343 | WITHIN-REGRESSION-LIMIT |
| simplify-update / medium-sparse | 20040558 | 18634502 | 14600972 | 0.929839 | 1.276251 | WITHIN-REGRESSION-LIMIT |
| simplify-update / million-smooth | 3488832018 | 3045010719 | 2587707352 | 0.872788 | 1.176721 | WITHIN-REGRESSION-LIMIT |
| simplify-update / tiny-smooth | 134678 | 130973 | 129413 | 0.972490 | 1.012054 | WITHIN-REGRESSION-LIMIT |

Exit codes: boundary=0, clippy=0, cpp=0, exact-build-no-std=0, exact-build-std=0, exact-lock=0, exact-no-std=0, exact-std=0, fmt=0, fuzz=0, sqrt=0, test-all=0, test-no-default=0; three arm builds=0; 432 stat samples=0.

Protocol: perf 6.17 instructions:u, CPU9, acknowledged resident-loop counter enable/disable, 100% running in every sample. Scalar opt3/thin-LTO/one-codegen-unit Rust arms; M7 uses scalar C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls and output validation stay outside counters; destination allocation, required copies, scratch acquisition and destruction stay inside.

Proof: exact baseline comparisons passed 10,058,846 paired observations under std and 10,058,846 under no_std. They compare Result discriminants/error bits, output words including failure prefixes and unused tails, updated positions/attributes, and both Usage fields, with cold/warm calls. Coverage includes all 512 option masks, all six families, every small-mesh work limit 0..unrestricted-work+1, worst-case picker meshes in both retention modes, byte limits and invalid/non-finite/overflow/signed-zero/subnormal inputs. The exhaustive adapter/backend sweep passed all 4,294,967,296 operand patterns with zero mismatches. Pinned C++ differential: 356 comparisons. Widened exact differential fuzz: 11,733 seeds / 750,912 generated cases / 300.005601 child CPU-seconds, zero mismatches. CPU duration is a fuzz work budget, not a performance result.

Retained failed attempts: launcher parse=2 before preparation; initial picker test compilation=101; second proof fmt=1 after test-all/test-no-default/clippy=0. The latter two attempts and their successful arm builds are retained in attempt1-test-compile/ and attempt2-test-format/. Repairs affect launcher/tests only. The final complete proof passed on the final source; every accepted instruction sample comes from that source. No failed attempt is relabeled as passing.

Raw counters, output buffers, commands, source/binary hashes and admission identities remain in `/mnt/linux-extra/moss-scratch/meshopt-0212-simplify/`. Compact per-case counts, results, proof exits and an independently checked hash binding are retained beside this report. No wall-time or production-performance claim is made. No new unsafe, API, version or shared-math changes.

Final evidence audit=0; target cleanup=0, verified absent. `binding.json` binds all 432 raw samples and their outputs, final library/tool sources, arm manifests/locks, proof binaries and proof logs. M7 source files and the fresh executable match the attribution binding independently. `cleanup.json` and `heavy-receipts.tsv` retain the cleanup and admission records.

All 11 admissions declared 4 GB and disabled GPU. Wrapper maximum sampled process-tree PSS: 1.4 GB; maximum total cgroup peak including charged cache: 3.8314 GB, below 4 GB, under the wrapper's 6 GB cap. Reservation adaptation never changed the 4 GB declaration. Wrapper wait/run fields are operational records, not performance timings.

Baseline: annotated tag object `4b0c94bcf57861d8f80bcfdedf7a482bd2e99ff3` peels to `40dbf9e42efc34d9292e439633e9ef52bc2248df`, tree `3ede4f74b0f5b560e74de50e47ec008c20709d7d`. The initial registration confused tag-object and peeled-commit identifiers; the actual archive, exact consumer and evidence audit all used the correct peeled release commit.

Decision: keep the qualified gains. All three real chains are PARTIAL; the <=0.85 PASS target remains unmet. Worst individual candidate/base ratio is 1.000000620 (simplify-prune / medium-smooth), below the strict >1.01 rejection threshold. No gate was relaxed and no measurement was selected or replaced.
