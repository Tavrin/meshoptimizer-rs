# SPEC 0216 qualification

**UNQUALIFIED: the SPEC-0216b release proof is incomplete.** No function has a SHIPS verdict. The 0216 campaign found no instruction regression, but the corrected finite proof has not been implemented and run completely. Earlier campaigns are not reused as verdicts.

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

The all-features and no-default tests, all-target all-feature clippy, fmt, unsafe-boundary and public-text checks each exited 0 on the candidate before this report was written. An interim public-text check found private filesystem literals in two helpers (exit 1); those helpers were made portable and the recheck exited 0. The exact executed helper versions are retained and hash-bound in `executed-helper-hashes.json`. The final public-text check exited 0. The pinned C++ meshlet and cluster-LOD differential exited 0 for 1,280 cases in each std and no_std mode. The C++ simplify-family and overdraw differential exited 0 for 1,246 cases. Meshlet fuzz exited 0 after 41,302,000 cases and 300.020 candidate CPU seconds; simplify differential fuzz exited 0 after 1,325 seeds and 300.013 child CPU seconds. Cluster-LOD protocol fuzz exited 0 after 1,449,000 generated cases and 300.01 candidate CPU seconds. Instrumented representative seeds reached the changed paths; LLVM reported five mismatched functions in the simplify coverage report and three in a candidate-only replay without identifying them, so simplify reach evidence carries that limitation; the meshlet and cluster-LOD coverage reports had no warnings. See `reach.json`. The finite all-features simplify comparison exited 0 with 1,200,243,710 observations. The proof supervisor then exited 101 because the overdraw driver was invoked without its required input directory; the failure receipt is retained. Corrected all-features overdraw exited 0 with 20,558,564 observations. The next supervisor exited 101 because the no-default simplify driver unwrapped an unsupported option mask; the driver now checks baseline/candidate mask acceptance and skips unsupported masks, and the corrected no-default simplify run exited 0 with 273,906,930 observations, and no-default overdraw exited 0 with 20,558,564 observations. Finite meshlet and cluster-LOD baseline comparisons each exited 0 with 154,590 observations in all-features and no-default-plus-clusterlod modes. These are subsets, and a retry does not erase the failed full proof-supervisor receipt. These finite checks do not establish the corrected complete comparison. See `ENUMERATION.md` and `enumeration-0216b.json`.

Scratch retains full inputs, binaries, bindings, outputs, admission logs, failed adapter build and raw perf records. Dedicated target cleanup is recorded in `cleanup.json`.

## SPEC-0216b proof continuation

The corrected standard supersedes the earlier unbounded requirement; the issue is the unexecuted finite product, not an assertion that the corrected task is impossible. Baseline scratch `src/` matched all 53 files at `9c26f3e`; candidate scratch `src/` matched the frozen `4d14205` worktree (`source-binding-0216b.json`). A newly built scalar C++ protocol differential for simplify and overdraw passed 1,246 cases in each all-features and no-default mode, exit 0 in each (`protocol-0216b.json`). The no-default corpus masks off experimental option bits.

The additional overdraw stride-by-work sweep passed 46,392,864 comparisons in
each feature mode, exit 0 (`overdraw-strides-0216b.json`). It covers four small
meshes, all supported position strides, byte orders, offsets 0–3 and short or
full final records for four APIs. Each API is swept through its unrestricted
usage plus one; the deliberately short output API fails even unrestricted.
It is a subset and does not supply the full release enumeration size.

Overdraw differential fuzz exited 0 after 3,621 seeds and 300.014852 child CPU
seconds (`overdraw-fuzz-0216b.json`). This is separate from the retained
simplify-family, meshlet and cluster-LOD five-minute receipts.

The meshlet and cluster-LOD C++ protocol differential was rebuilt and passed
1,280 cases per mode, exit 0, with all features and with no defaults plus the
required `clusterlod` feature (`meshlet-protocol-0216b.json`).

The additional meshlet caller-buffer stride-by-work sweep passed in each
feature mode, exit 0: 11,100,480 `build_meshlets`, 11,091,552
`build_meshlets_flex` and 10,533,552 `build_meshlets_spatial` comparisons
(`meshlet-strides-0216b.json`). It covers four small meshes, five selected
builder settings, all position strides and both byte orders, offsets 0–3 and
short or full final records. These are subset sizes, not the full release
enumeration sizes.

The three-vertex simplify flag sweep passed in both modes, exit 0. It crosses
all 512 assignments of flag values 0–7 with all accepted simplify masks and
work limits, using one packed mesh and selected attributes. Its per-function
observation counts are in `simplify-flags-0216b.json`; they are subset sizes,
not the required full small-mesh and stride product.

An additional meshlet sweep included the allocating APIs alongside the full
caller buffers. It passed the same four meshes and stride/work domain in both
feature modes, exit 0, with the same per-function comparison counts
(`meshlet-strides-owned-0216b.json`). The domain remains a subset.

The initial cluster-LOD boolean sweep was deliberately interrupted (exit 130)
before its first 256-mask checkpoint to replace full-vector debug formatting
with direct structured equality. Its incomplete log, source binding and binary
remain in scratch; it is not counted as a pass.

The corrected affected-suite supervisor reran to completion, exit 0 in both
modes (`affected-rerun-0216b.json`). Simplify produced 1,200,243,710
all-feature and 273,906,930 no-default observations; overdraw produced
20,558,564 observations in each mode. Both drivers exited 0 in each mode.
The supervisor used the required input directory and the corrected option-mask
acceptance check. Its executed script is retained and hash-bound separately
from the portable checked-in version. The two earlier exit 101 receipts remain
failures, and this successful rerun remains a subset of the corrected standard.

### Recorded per-function subset sizes

These are comparisons/observations from the specific new domains described
above, not the unknown full release enumeration. Completed entries exited 0.

| Function | All features | No defaults |
|---|---:|---:|
| build_meshlets | 11,100,480 | 11,100,480 |
| build_meshlets_flex | 11,091,552 | 11,091,552 |
| build_meshlets_spatial | 10,533,552 | 10,533,552 |
| cluster LOD | 130,418,688 | 130,418,688 |
| simplify | 116,719,616 | 28,655,616 |
| simplify_with_attributes | 165,453,824 | 40,839,168 |
| simplify_with_update | 100,069,376 | 24,755,200 |
| simplify_sloppy | 8,042,496 | 2,010,624 |
| simplify_prune | 48,234,496 | 12,058,624 |
| simplify_points | 12,582,912 | 3,145,728 |
| optimize_overdraw | 46,392,864 | 46,392,864 |

The structured cluster-LOD all-feature mode passed all 4,096 boolean masks
on each of four small meshes, 130,418,688 comparisons and mode exit 0. Its
supervisor was then deliberately interrupted (exit 130) during a duplicate
no-default continuation; the separately admitted no-default run carries that
mode. This boolean sweep uses selected numeric configuration and no attributes
or vertex locks, so it is not the full release proof.

The independent no-default cluster-LOD sweep also passed all four meshes and
4,096 masks per mesh, 130,418,688 comparisons and supervisor exit 0
(`clod-flags-0216b.json`). The all-feature mode had the identical count.

The dedicated qualification target was removed after all admitted proof jobs
exited (`cleanup-0216b.json`); retained inputs, binaries and logs remain in
scratch. The source tree still matches `4d14205`.
The final public-text check exited 0.

## SPEC-0216c gaps — in progress

The new gap-1 grid and count semantics are documented in `GAP1.md`. A first
ordering was deliberately interrupted (supervisor exit 130) to spread a
bounded prefix across the input dimensions. Its last checkpoints report 70
completed cells per mode and 9,250,250 all-feature / 9,166,328 no-default
compared observations, including comparisons in the next unfinished cell.
They are checkpoint counts, not exact totals for the interrupted processes.
The reordered driver has been admitted and is running in both modes. No gap-1
completion or per-function SHIPS verdict has been established. Gaps 2–4 have
not yet been rerun under this specification. The earlier affected-suite
rerun and its two original failures remain documented above.
