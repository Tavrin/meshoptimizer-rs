The L1 cluster path spends time clearing partition maps, publishing scan descriptors per triangle, evaluating zero-weight cone arithmetic, and dividing inside BVH pivots. This change reuses generation tags, keeps scan output state local, specializes the zero-weight search, and uses an exact invariant integer quotient. Position preparation and Morton interleave are also tightened. Public outputs, errors, work usage and APIs remain unchanged.

Registered performance verdict: **PARTIAL**. The single final campaign used the equalized Moss scalar adapters, 1 warm-up + 10 alternating pairs, 10,000 bootstrap resamples, seed 20261030, with A/A floor 0.170512. X is disclosure only.

| Family | O/M7 GM | O/0.2.7 GM | O/X GM | Max case upper95 vs 0.2.7 | Verdict |
|---|---:|---:|---:|---:|---|
| partition | 1.450544 | 0.989458 | 1.106790 | 1.056675 | PARTIAL |
| meshlets-scan | 1.067374 | 0.801371 | 0.700001 | 0.920768 | PARTIAL |
| meshlets | 1.022411 | 0.930124 | 0.861130 | 1.065561 | PARTIAL |
| meshlets-spatial | 1.121742 | 0.992160 | 1.092985 | 1.157240 | PARTIAL |
| meshlets-flex | 1.007063 | 0.933526 | 0.876291 | 0.987662 | PARTIAL |
| spatial-triangles | 1.026324 | 0.928209 | 0.925812 | 1.024605 | PARTIAL |

RFC113 aggregate: 1.040298x C++ (S1 1.037887x; S2 1.053315x), disclosure only. PARTIAL families have no C++ performance-parity claim; FAIL families are not performance-qualified.

Validation: both required feature-configuration tests, clippy -D warnings and formatting passed. Existing C++ differential and 34,172 baseline exact comparisons passed, including Results, work accounting and caller-buffer failure contents. Both widened mesh/layout fuzz and full-width divider fuzz exceeded five target CPU-minutes with zero mismatches. All 50 RFC113 preflight cases matched.

See parity/028-L1/REPORT.md, REGISTRATION.md and RESULTS.json for causes, source locations, raw pairs, intervals, exits and retained evidence. No merge, tag, publication or version bump.
