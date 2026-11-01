# Pipeline qualification

**Verdict: FAIL — performance target unqualified.**

`process_mesh` composes the existing functions through one workspace and returns
one stage/quality report with staged errors. Geometry equivalence passed on
all eleven grid/corpus cases, including all three retained real meshes.
Both exact regression modes and both independent C++ F9 oracle modes passed.

The original zero-duration tiny timings were rejected. Batched small-case
measurements and all seven million-triangle pairs remain retained. Ratios
below are medians of within-pair ratios; the limit remains 1.02.

| Case | Pairs | One-call/manual |
| --- | ---: | ---: |
| grid-4 | 7 | 0.832459 |
| grid-16 | 7 | 0.958203 |
| grid-32 | 7 | 0.973295 |
| tiny-smooth | 7 | 0.913254 |
| medium-smooth | 7 | 0.984282 |
| medium-seams | 7 | 1.043533 |
| medium-sparse | 7 | 0.980227 |
| million-smooth | 7 | 1.027049 |
| real-214e | 3 | 1.002486 |
| real-a082 | 3 | 0.996227 |
| real-b6fd | 3 | 0.976780 |

A separate CPU-accounted full-campaign attempt selected an idle allowed
physical core and stopped on its first rejected pair. The manual arm had
about 92% CPU occupancy; the conservative required lower bound was 99%.
No quiet-core performance pass is claimed. The cause of shared-host
above-limit medians is unproven. See `receipts/quiet.log` and `quiet-cpu.json`.

Wasm timing is absent where no std clock is available; default and no_std
runtime smoke checks exercise the new API. Retained scratch is counted
even on validation failure. Cluster outputs retain any dilated positions.

Final gate codes, samples and source/binary/log bindings are in
`receipts/final.json`, `exits.tsv` and the binding JSON files.
The dedicated build directory is removed after every admitted job exits.
Git state is unchanged.
