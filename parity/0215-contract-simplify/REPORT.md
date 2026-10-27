# Contract simplification qualification

**FAIL; final positional-only source is unqualified.** The complete first campaign rejected the combined positional and grid-meter candidate because all eight points cases exceeded the 1.01 regression limit. The grid-meter change has been reverted. The remaining positional-only revision has not been rebuilt or measured; the proof and measurements below bind the rejected combined revision, not the final source.

The prior contract-cost experiment attributed 44.2% of the pooled simplify gap, 52.8% of the attributes gap and 68.0% of the overdraw gap to finite checks conditional on the work/root ablation. Work accounting explained 40.4% of the sloppy gap. These are conditional instruction attributions, not a unique decomposition.

The retained positional change removes an unreachable quadric finite scan, using the bound in PROOFS.md (`src/simplify.rs:1741`, `src/simplify.rs:1859`, `src/simplify.rs:2466`). Attribute and gradient checks remain. The rejected grid-meter change batched bounded hash visits but increased the points cost. Native roots were already present; the portable path is unchanged. Earlier overdraw variants failed their regression gate and were not included.

Median instructions per call from three runs per arm; columns are candidate/base, candidate/C++ 1.3, and candidate/optimesh 1.1.0. Ratios below 1 mean fewer instructions. This table is for the rejected combined revision, with packed peers-format inputs and synthetic attributes.

| Input | Operation | Candidate/base | Candidate/C++ | Candidate/optimesh |
|---|---|---:|---:|---:|
| tiny-smooth | simplify | 0.9886 | 0.9265 | 0.9570 |
| tiny-smooth | simplify-attributes | 0.9904 | 1.0289 | 0.9840 |
| tiny-smooth | simplify-update | 0.9899 | 0.9593 | 0.9755 |
| tiny-smooth | simplify-sloppy | 0.9972 | 1.3596 | 0.9767 |
| tiny-smooth | simplify-prune | 1.0000 | 1.5372 | 1.1645 |
| tiny-smooth | simplify-points | 1.0503 | 1.4542 | 0.9268 |
| tiny-smooth | overdraw | 1.0000 | 0.7921 | 0.8905 |
| medium-smooth | simplify | 0.9823 | 1.1096 | 0.9377 |
| medium-smooth | simplify-attributes | 0.9926 | 1.1261 | 0.9436 |
| medium-smooth | simplify-update | 0.9861 | 1.1427 | 0.9980 |
| medium-smooth | simplify-sloppy | 0.9964 | 1.3513 | 0.9922 |
| medium-smooth | simplify-prune | 1.0000 | 1.4780 | 1.2048 |
| medium-smooth | simplify-points | 1.0306 | 1.6720 | 1.0515 |
| medium-smooth | overdraw | 1.0000 | 1.0328 | 0.9131 |
| medium-seams | simplify | 0.9830 | 1.0903 | 0.9246 |
| medium-seams | simplify-attributes | 0.9911 | 1.1436 | 0.9588 |
| medium-seams | simplify-update | 0.9855 | 1.1277 | 0.9705 |
| medium-seams | simplify-sloppy | 0.9974 | 1.4580 | 0.8417 |
| medium-seams | simplify-prune | 1.0000 | 1.4958 | 1.2043 |
| medium-seams | simplify-points | 1.0652 | 1.5874 | 0.8915 |
| medium-seams | overdraw | 1.0000 | 1.0513 | 0.9168 |
| medium-sparse | simplify | 0.9686 | 1.1265 | 0.9477 |
| medium-sparse | simplify-attributes | 0.9843 | 1.1816 | 1.0000 |
| medium-sparse | simplify-update | 0.9754 | 1.1736 | 1.0206 |
| medium-sparse | simplify-sloppy | 0.9940 | 1.4079 | 0.9438 |
| medium-sparse | simplify-prune | 1.0000 | 1.5412 | 1.2107 |
| medium-sparse | simplify-points | 1.0202 | 1.7015 | 1.1132 |
| medium-sparse | overdraw | 1.0000 | 1.0842 | 0.9412 |
| million-smooth | simplify | 0.9894 | 1.0787 | 0.9067 |
| million-smooth | simplify-attributes | 0.9937 | 1.1257 | 0.9321 |
| million-smooth | simplify-update | 0.9910 | 1.1041 | 0.9486 |
| million-smooth | simplify-sloppy | 0.9960 | 1.3196 | 1.0481 |
| million-smooth | simplify-prune | 1.0000 | 1.4749 | 1.2061 |
| million-smooth | simplify-points | 1.0386 | 1.6558 | 1.0130 |
| million-smooth | overdraw | 1.0000 | 1.0302 | 0.9091 |
| real-214e | simplify | 0.9107 | 1.2600 | 1.0173 |
| real-214e | simplify-attributes | 0.9535 | 1.4453 | 1.2587 |
| real-214e | simplify-update | 0.9318 | 1.4278 | 1.1445 |
| real-214e | simplify-sloppy | 0.9925 | 1.5039 | 0.8094 |
| real-214e | simplify-prune | 1.0000 | 1.6090 | 1.2302 |
| real-214e | simplify-points | 1.0212 | 1.7481 | 1.1146 |
| real-214e | overdraw | 1.0000 | 1.4693 | 1.1880 |
| real-a082 | simplify | 0.9432 | 1.2230 | 0.9571 |
| real-a082 | simplify-attributes | 0.9655 | 1.3649 | 1.1375 |
| real-a082 | simplify-update | 0.9523 | 1.3320 | 1.0415 |
| real-a082 | simplify-sloppy | 0.9921 | 1.4917 | 0.8645 |
| real-a082 | simplify-prune | 1.0000 | 1.6023 | 1.2312 |
| real-a082 | simplify-points | 1.0213 | 1.7498 | 1.1145 |
| real-a082 | overdraw | 1.0000 | 1.4371 | 1.1679 |
| real-b6fd | simplify | 0.9431 | 1.2266 | 0.9570 |
| real-b6fd | simplify-attributes | 0.9655 | 1.3678 | 1.1378 |
| real-b6fd | simplify-update | 0.9522 | 1.3354 | 1.0414 |
| real-b6fd | simplify-sloppy | 0.9928 | 1.5006 | 0.8172 |
| real-b6fd | simplify-prune | 1.0000 | 1.6030 | 1.2307 |
| real-b6fd | simplify-points | 1.0373 | 1.7005 | 1.0129 |
| real-b6fd | overdraw | 1.0000 | 1.4371 | 1.1679 |

Neither the real-mesh PASS rule nor the PARTIAL rule is met, and neither could override the eight 1.01 regression failures. Specifically, points ratios were 1.0503, 1.0306, 1.0652, 1.0202, 1.0386 on the corpus and 1.0212, 1.0213, 1.0373 on the real inputs. Baseline, candidate and C++ output hashes matched in all 56 cells and all three repetitions were stable. Optimesh output identity is not asserted by the gate.

| Gate on rejected combined revision | Exit |
|---|---:|
| all-features tests | 0 |
| no-default-features tests | 0 |
| clippy, fmt, boundary, public text | 0 each |
| exact simplify comparison | 0; 20,401,022 observations |
| exact overdraw comparison | 0; 20,558,816 observations |
| differential fuzz | 0; 3,649 seeds, 300.058 child CPU seconds |
| pinned C++ differential | 0; 1,246 comparisons |
| initial campaign attempt | 1; input directory removed externally |
| resumed collection | 0; 672 samples |
| results script | 0; verdict FAIL |

The eight inputs were regenerated under admission and matched the original SHA-256 binding before resumption. Finished samples were skipped; the interrupted sample is retained. The dedicated target was cleaned after the executables were copied and hashed. The later operational constraint prohibits building the same arm again, so a second candidate build and campaign were not run. The final positional-only revision has no fresh exact, C++ or instruction receipt. No performance claim is made for it.
