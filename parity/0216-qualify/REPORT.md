# SPEC 0216 qualification

**UNQUALIFIED: complete exact proof not established.** No function has a SHIPS verdict. The 0216 campaign found no instruction regression, but the proof requirement is not satisfied. Earlier campaigns are not reused as verdicts.

Candidate: `4d14205cf201715a3df3debd795705ea0b1d904f`; baseline `9c26f3e`.

| Function | Three-mesh candidate/base geometric mean | Verdict |
|---|---:|---|
| build_meshlets | 0.867465 | no claim; exact proof incomplete |
| build_meshlets_flex | 0.867417 | no claim; exact proof incomplete |
| build_meshlets_spatial | 1.000000 | no claim; exact proof incomplete |
| cluster LOD | 0.924189 | no claim; exact proof incomplete |
| simplify | 0.932214 | no claim; exact proof incomplete |
| simplify_with_attributes | 0.961486 | no claim; exact proof incomplete |
| simplify_with_update | 0.944959 | no claim; exact proof incomplete |
| simplify_sloppy | 1.000000 | no claim; exact proof incomplete |
| simplify_prune | 1.000000 | no claim; exact proof incomplete |
| simplify_points | 1.000000 | no claim; exact proof incomplete |
| optimize_overdraw | 1.000000 | no claim; exact proof incomplete |

## Campaign

One resumable instruction-only campaign, median of three, four arms except optimesh has no cluster LOD. Five frozen corpus meshes and three production LOD-chain meshes yielded 1,032 accepted samples and 88 cells. All samples reported 100% counter running. Maximum candidate/base was 1.0001434102 (tiny-smooth spatial meshlets); no case exceeded 1.01. All 88 base/candidate and candidate/C++ canonical output hashes matched. See `results.json`, `counts.jsonl`, `inputs.json`, and `REGISTRATION.md` for exact cases, adapter repair, and retained bindings. CPU-only admission declared 4 GB for each job; the wrapper selected a 6 GB scope cap and logged 2.0 GB peak for the completed continuation measurement job. No wall-time claim.

## Proof status

The all-features and no-default tests, all-target all-feature clippy, fmt, unsafe-boundary and public-text checks each exited 0 on the candidate before this report was written. An interim public-text check found private filesystem literals in two helpers (exit 1); those helpers were made portable and the recheck exited 0. The exact executed helper versions are retained and hash-bound in `executed-helper-hashes.json`. The final public-text check exited 0. The pinned C++ meshlet and cluster-LOD differential exited 0 for 1,280 cases in each std and no_std mode. The C++ simplify-family and overdraw differential exited 0 for 1,246 cases. Meshlet fuzz exited 0 after 41,302,000 cases and 300.020 candidate CPU seconds; simplify differential fuzz exited 0 after 1,325 seeds and 300.013 child CPU seconds. Cluster-LOD protocol fuzz exited 0 after 1,449,000 generated cases and 300.01 candidate CPU seconds. Instrumented representative seeds reached the changed paths; LLVM reported five mismatched functions in the simplify coverage report and three in a candidate-only replay without identifying them, so simplify reach evidence carries that limitation; the meshlet and cluster-LOD coverage reports had no warnings. See `reach.json`. The finite all-features simplify comparison exited 0 with 1,200,243,710 observations. The proof supervisor then exited 101 because the overdraw driver was invoked without its required input directory; the failure receipt is retained. Corrected all-features overdraw exited 0 with 20,558,564 observations. The next supervisor exited 101 because the no-default simplify driver unwrapped an unsupported option mask; the driver now checks baseline/candidate mask acceptance and skips unsupported masks, and the corrected no-default simplify run exited 0 with 273,906,930 observations, and no-default overdraw exited 0 with 20,558,564 observations. Finite meshlet and cluster-LOD baseline comparisons each exited 0 with 154,590 observations in all-features and no-default modes. These are subsets, and a retry does not erase the failed full proof-supervisor receipt. These finite checks do not establish the requested complete comparison. See `ENUMERATION.md`.

Scratch retains full inputs, binaries, bindings, outputs, admission logs, failed adapter build and raw perf records. Dedicated target cleanup is recorded in `cleanup.json`.
