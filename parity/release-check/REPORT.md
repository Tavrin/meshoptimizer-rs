# downstream-qx5g3 release check

**NO RELEASE**. Candidate triggers FALLBACK; the single fallback campaign also fails the registered case no-regression rule. No timing-value reruns or retuning.

Candidate: `f40f5af32e7283d1a2b59ea7358a81fe2ecaa982`. Fallback: `a7688ef0c8d13b8dfeb57c2cb1ce325a76d3dfab`. Baseline 0.2.3: `afe803bbf60796a7af6b5001594a979af009cfac`. `git diff --stat d74b914..f40f5af -- src/` is empty.

| Family | Candidate GM [95%] | Fallback GM [95%] |
|---|---:|---:|
| overdraw | 0.988818 [0.982763, 0.993830] | 1.010854 [1.007023, 1.013664] |
| simplify | 0.993769 [0.991001, 0.995194] | 0.990768 [0.988892, 0.992891] |
| simplify_scale | 0.841878 [0.839127, 0.848344] | 0.845211 [0.842899, 0.849264] |
| simplify_with_attributes | 1.000917 [0.999578, 1.002662] | 1.000274 [0.998980, 1.001528] |
| vertex_cache | 1.007267 [1.001025, 1.011639] | 1.001089 [0.998374, 1.005696] |

All family point estimates are ≤1.02. Candidate case regressions: 6. Fallback case regressions: 4. A case fails when its lower 95% bound is >1.02. Every failed case and all intervals are retained in C-analysis.json / F-analysis.json.

## Raw samples and receipts

[All 4896 measured pairs, with both times, ratios and order](SAMPLES.csv). [Candidate analysis](C-analysis.json), [fallback analysis](F-analysis.json), [protocol verification](verification.json), [binary/source/profile bindings](identities.json), [inherited fuzz bindings](inherited-proof-binding.json), [method](METHOD.md), [verbatim initial registration](REGISTRATION.md), [fallback registration](F-REGISTRATION.md).

Full resident responses, raw JSON records, logs and scratch scripts remain at `<scratch>/meshopt-simplify-perf/release/`. `C-attempt-1/record.json` and `F-attempt-1/record.json` bind all 204 cases in each campaign; corresponding `samples.jsonl` files retain per-pair timestamps and loads. Both jobs exited 0; no infrastructure failure or rerun. Statistical failure is separate from harness exit status.

Initial queue waited behind train pid 3177400. Sampling began after that job was marked `held by the maintainer`; no queue bypass. Between-case RAM-waiter checks caused no PAUSE in either campaign. STEER.md and wrapper logs retain the policy and admission.

## Inherited parity and fuzz

No parity or fuzz rerun. Candidate: `../r2/parity-final/exits.tsv`, `../r2/final-tracked-summaries/run.json`, `../r2/final-tracked-summaries/sweep.json`, `../r2/fuzz-final/fuzz.json`, `../r2/source-continuity.json`, `../r2/REPORT.md`. Fallback: `../parity-final/exits.tsv`, `../final-parity-summaries/{run,sweep,js}.json`, `../fuzz-final/fuzz.json`, `../REPORT.md`. Paths in this paragraph are relative to the scratch release directory. Both inherited fuzz receipts pass with ≥4 CPU hours/target, zero crashes, unchanged identities and source hashes checked against each selected revision.

## Candidate campaign

### Worst five by median

| Case | Median | 95% interval | Regresses |
|---|---:|---|---|
| vertex_cache/million/smooth/allocating/a0/rNone | 1.131486 | [1.121119, 1.134970] | True |
| overdraw/million/smooth/allocating/a0/rNone | 1.054985 | [1.030063, 1.084386] | True |
| overdraw/million/smooth/caller-buffer/a0/rNone | 1.046035 | [1.032817, 1.051639] | True |
| overdraw/million/sparse/caller-buffer/a0/rNone | 1.038227 | [1.008096, 1.054089] | False |
| overdraw/million/sparse/allocating/a0/rNone | 1.037484 | [0.990039, 1.077710] | False |

### Worst five by lower bound

| Case | Median | 95% interval | Regresses |
|---|---:|---|---|
| vertex_cache/million/smooth/allocating/a0/rNone | 1.131486 | [1.121119, 1.134970] | True |
| overdraw/million/smooth/caller-buffer/a0/rNone | 1.046035 | [1.032817, 1.051639] | True |
| overdraw/million/smooth/allocating/a0/rNone | 1.054985 | [1.030063, 1.084386] | True |
| simplify_with_attributes/million/disconnected/allocating/a8/r0.25 | 1.030038 | [1.024872, 1.036393] | True |
| simplify_with_attributes/million/smooth/allocating/a12/r0.125 | 1.029997 | [1.021148, 1.043876] | True |

### Named case

`overdraw/million/disconnected/allocating/a0/rNone`: median 0.956099, 95% [0.954920, 0.961978], regresses=False. Absolute exemption applied only to this case.

### Absolute bars

Source: `<scratch>/meshopt-simplify-perf/r2/frozen-final-1/benchmark.json`, SHA-256 `c3034c1bc6b3130d9c91fe3a704859505bf1f60a84522809036d033a37245fbd`. Complete 204/204; family GM ≤1.25 and each non-exempt case ≤1.50: **True**.

| Family | GM vs C++ | Max vs C++ |
|---|---:|---:|
| vertex_cache | 1.124791 | 1.195407 |
| overdraw | 1.086187 | 1.444646 |
| simplify | 1.043579 | 1.197683 |
| simplify_with_attributes | 1.071103 | 1.225558 |
| simplify_scale | 0.784758 | 0.915833 |



## Fallback campaign

### Worst five by median

| Case | Median | 95% interval | Regresses |
|---|---:|---|---|
| simplify_with_attributes/tiny/sparse/allocating/a1/r0.5 | 1.078535 | [1.071555, 1.092526] | True |
| simplify_with_attributes/tiny/sparse/caller-buffer/a12/r0.125 | 1.048470 | [1.044535, 1.053524] | True |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a8/r0.25 | 1.043776 | [1.037900, 1.051300] | True |
| overdraw/medium/smooth/caller-buffer/a0/rNone | 1.031615 | [1.028172, 1.041708] | True |
| overdraw/million/smooth/caller-buffer/a0/rNone | 1.030973 | [1.001838, 1.041839] | False |

### Worst five by lower bound

| Case | Median | 95% interval | Regresses |
|---|---:|---|---|
| simplify_with_attributes/tiny/sparse/allocating/a1/r0.5 | 1.078535 | [1.071555, 1.092526] | True |
| simplify_with_attributes/tiny/sparse/caller-buffer/a12/r0.125 | 1.048470 | [1.044535, 1.053524] | True |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a8/r0.25 | 1.043776 | [1.037900, 1.051300] | True |
| overdraw/medium/smooth/caller-buffer/a0/rNone | 1.031615 | [1.028172, 1.041708] | True |
| simplify_with_attributes/million/seam-heavy/allocating/a12/r0.125 | 1.023620 | [1.017203, 1.030858] | False |

### Named case

`overdraw/million/disconnected/allocating/a0/rNone`: median 1.005218, 95% [1.002290, 1.009993], regresses=False. Absolute exemption applied only to this case.

### Absolute bars

Source: `<scratch>/meshopt-simplify-perf/r2/frozen-final-1/benchmark.json`, SHA-256 `c3034c1bc6b3130d9c91fe3a704859505bf1f60a84522809036d033a37245fbd`. Complete 204/204; family GM ≤1.25 and each non-exempt case ≤1.50: **True**.

| Family | GM vs C++ | Max vs C++ |
|---|---:|---:|
| vertex_cache | 1.124791 | 1.195407 |
| overdraw | 1.086187 | 1.444646 |
| simplify | 1.043579 | 1.197683 |
| simplify_with_attributes | 1.071103 | 1.225558 |
| simplify_scale | 0.784758 | 0.915833 |



Absolute bars are judged from the registered r2 complete final matrix for both decisions, exactly as specified; no new C++ measurement was made for fallback. The named case is exempt only from its absolute case maximum, and passes the direct C/B interval test in both campaigns.

## Disposition

NO RELEASE. README known-exception and release CHANGELOG additions are conditional on RELEASE, so neither is added. Existing r1/r2 changelog entries remain. This is an evidence-only commit: no src, harness, corpus, bar, version, tag, merge or publish changes. Owner review remains pending.

## Not verified

Universal performance outside this matrix; other CPUs/platforms; individual-fix causal gains; optional independent r2 retained-buffer rehash; historical aggregate release report. Owner review, version bump and publication were not performed.
