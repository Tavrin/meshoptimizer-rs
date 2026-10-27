# Contract simplification qualification

Status: **IN PROGRESS**. Instruction campaign and C++ differential have not
completed; the pilot below is diagnostic and is not the registered verdict.

The preceding contract-cost experiment attributed the largest conditional cost
in the three-real-mesh aggregate to finite checks: 44.2% of the simplify gap,
52.8% for attributes, and 68.0% for overdraw. Work accounting explained 40.4%
of the sloppy gap. These are conditional experimental attributions, not an
independent decomposition of every check.

The candidate replaces the positional quadric finite scan with the bound in
PROOFS.md and batches bounded grid-cell work accounting. The attribute and
gradient finite checks and the limited work path remain. The earlier overdraw
bound variants failed their registered regression gate and are not carried
forward. The native root paths already present in the baseline are unchanged.

A diagnostic median-of-three candidate/base pilot for sloppy simplification
was 0.9972, 0.9964, 0.9974, 0.9940 and 0.9960 across the five corpus inputs,
and 0.9925, 0.9921 and 0.9928 across the three real inputs. Outputs matched
between those two arms. This does not determine the four-arm verdict.

| Proof or gate | Exit |
|---|---:|
| all-features tests | 0 |
| no-default-features tests | 0 |
| clippy, fmt, boundary, public text | 0 each |
| simplify exact comparison | 0; 20,401,022 observations |
| overdraw exact comparison | 0; 20,558,816 observations |
| differential fuzz | 0; 3,649 seeds, 300.058 child CPU seconds |
| C++ differential | pending |
| registered instruction campaign | pending |
