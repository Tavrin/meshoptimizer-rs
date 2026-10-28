# SPEC 0216 qualification

**UNQUALIFIED: the SPEC-0216c gaps remain open.** No function has a SHIPS verdict. The bounded gap-1 run reached its time limit before completing its grid; gaps 2–4 were not run under this specification. Earlier campaigns are not reused as complete gap verdicts.

Candidate: `4d14205cf201715a3df3debd795705ea0b1d904f`; baseline `9c26f3e`.

| Function | Three-mesh candidate/base geometric mean | Verdict |
|---|---:|---|
| build_meshlets | 0.867465 | UNQUALIFIED; exact proof incomplete |
| build_meshlets_flex | 0.867417 | UNQUALIFIED; exact proof incomplete |
| build_meshlets_spatial | 1.000000 | does not meet instruction-ratio threshold |
| cluster LOD | 0.924189 | UNQUALIFIED; exact proof incomplete |
| simplify | 0.932214 | UNQUALIFIED; exact proof incomplete |
| simplify_with_attributes | 0.961486 | UNQUALIFIED; exact proof incomplete |
| simplify_with_update | 0.944959 | UNQUALIFIED; exact proof incomplete |
| simplify_sloppy | 1.000000 | does not meet instruction-ratio threshold |
| simplify_prune | 1.000000 | does not meet instruction-ratio threshold |
| simplify_points | 1.000000 | does not meet instruction-ratio threshold |
| optimize_overdraw | 1.000000 | does not meet instruction-ratio threshold |

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

## SPEC-0216c gaps — time limit

Gap 1 reached the specified active-work cutoff before completing its grid. The
bounded reordered run was admitted CPU-only with a 4 GB declaration; the
wrapper selected a 6 GB scope cap. Each mode ran for 27,900 seconds and exited
124, as did its supervisor. No compared observations differed. The count includes
completed cold and reused-workspace comparisons in the unfinished cell; a reused
workspace after a failed call can be empty or only partially populated. This is
not a complete warm-workspace proof. Source and binary bindings, final counts,
unfinished-cell coordinates and retained-log hashes are in `gaps-0216c.json`;
the product and count semantics are documented in `GAP1.md`.

| Gap | All features: observations; completed cells | No defaults: observations; completed cells | Exit |
|---|---:|---:|---|
| 1: simplify quadrics | 220,975,530; 649 / 7,077,888 (0.0091694%) | 243,318,400; 1,117 / 1,769,472 (0.06312618%) | 124 / 124; incomplete |
| 2: meshlet nearest and work accounting | 0 | 0 | not run |
| 3: cluster LOD roots with attributes and locks | 0 | 0 | not run |
| 4: affected-suite supervisor reruns | 0 | 0 | not run under this specification |

Gap 1 counts by function, all-features / no-default: `simplify` 67,389,904 /
64,868,620; `simplify_with_attributes` 101,772,788 / 117,681,280;
`simplify_with_update` 51,812,838 / 60,768,500. The counts are from this
bounded run, not full enumeration sizes. The first ordering was deliberately
interrupted (exit 130) to spread a bounded prefix across input dimensions. Its
last checkpoints contain 70 completed cells per mode and 9,250,250 all-feature /
9,166,328 no-default observations, including partial-cell comparisons. They are
checkpoint counts, not exact totals for the interrupted processes, and are not
added to the new run. The two earlier supervisor exits 101 and their subsequent
0216b affected-suite rerun remain documented above; no new gap-4 rerun occurred.

All six functions whose instruction ratio is at most 0.97 remain UNQUALIFIED:
`build_meshlets`, `build_meshlets_flex`, `cluster LOD`, `simplify`,
`simplify_with_update`, and `simplify_with_attributes`. The other functions do
not meet the instruction-ratio threshold. There is no SHIPS verdict, no source
revert (no difference was observed), and no new changelog claim. The designated
build target was removed after the admitted job exited; the logs, binaries,
source hashes and interrupted-run evidence remain in scratch.

## SPEC-0216d targeted — partial, unqualified

The initial quadrics pilot used one six-vertex mesh, width 3, mask 0 and seven
API/buffer variants per feature mode. Both modes exited 0 with 4,604 cold and
repeated-workspace comparisons, 0 differences, and approximately 0.01 user CPU
seconds per mode. This is a sizing check, not the targeted proof.

A preliminary numeric grid covered 384 cells per mode: eight six-vertex
meshes, widths 0/3/32, the empty mask, the five individual option flags, and
their ten pairs. Each cell covered seven API/buffer variants and all integer
limits from zero through observed unlimited usage plus one, with a cold call
and a repeated call at each limit. Both feature modes exited 0, each recording
2,363,990 comparisons (494,688 simplify, 1,169,968 attributes, 699,334 update)
and zero differences. User CPU time was 12.44 seconds all features and 14.11
seconds no defaults. The first supervisor attempt exited 1 before building or
running due to a wrong script path in its source-hash command; the corrected
retry exited 0. These observations do not establish independently primed warm
states, or success after a prior failed call at a different input.

The later sections record the targeted meshlet and cluster-LOD sweeps and both
corrected supervisor reruns. Successful priming was not established at every
tested limit, so all changed paths remain UNQUALIFIED under this specification.
The separate adversarial review is not incorporated here. No 0216d SHIP or
FAIL verdict follows from these subsets. `TARGETED-PLAN.md` describes the sizes
and unresolved harness requirements.

The independently primed quadrics pilot exited 0 in each mode, with 10,582
compared target calls per mode. The corresponding 384-cell grid also exited 0
in each mode, with 4,722,604 target-call comparisons per mode: 987,072
simplify, 2,337,632 attributes and 1,397,900 update. Priming succeeded on
987,691 of 1,179,307 attempts per mode; it failed on 191,616 attempts. No
comparisons differed. Those failed priming attempts cannot establish a
successful-warm state. User CPU time was 31.43 seconds all-features and 33.69
seconds no-default. Both the priming observation and two subsequent target
calls are compared; the target-call count excludes the priming calls.

The cluster-LOD pilot covered one grid mesh, width 1, no locks and one
configuration mask, and exited 0 with 188 comparisons in each mode. It is a
harness pilot and does not independently prime a successful warm workspace.

The cluster-LOD grid used the pilot's mesh, two attribute widths, four lock
patterns and all 4,096 boolean configurations: 32,768 cells per mode. It
compared both revisions for the unlimited call, downward search for a failure,
zero, successful usage, success plus one, and limits at a stride of 64; each
observation sequence includes two consecutive calls on one workspace. Both
modes exited 0 with 6,901,896 comparisons and no differences. User CPU time
was 222.11 seconds all-features and 213.12 seconds no-default plus clusterlod.
The driver does not independently prime a successful warm state for work limits
where the first target call fails, so this is not the complete 0216d proof.

The preliminary meshlet sweep covered seven meshes, including the four earlier
meshes and collinear, clustered-pair and 2-by-2-by-2 coordinate patterns, four
builder settings and five variants: 140 cells per mode. For each it compared
unlimited execution and every limit from zero through the maximum observed
unlimited work plus four times the triangle count plus 16. Each limit compares
both allocated and sentinel-filled caller-buffer outputs and their usage. The
cold-workspace sweep exited 0 in both modes with 161,778 comparisons per mode
and no differences; user CPU time was 3.39 and 3.35 seconds respectively. The
traversal entry boundary was not instrumented, and warm workspaces are not
covered by this sweep; it is therefore not the full requested proof.

A separate repeated-workspace meshlet sweep used the same 140 cells and limits,
with two consecutive calls per workspace. Both modes exited 0, with 323,556
comparisons per mode and no differences; user CPU time was 6.68 and 6.42
seconds. Reuse after a failed first call is included. This does not establish
independent successful priming at failing limits or directly measure each
traversal entry's covered-work boundary.

A diagnostic-only candidate copy logged traversal entry work and node count for
140 meshlet cells in each mode, using two unlimited calls per API. Both modes
exited 0. Each logged 9,280 nearest entries across 112 cells; 28 spatial cells
had no nearest entry. The recorded boundary equals entry work plus twice the
node-slice length. The frozen-candidate broad sweep above covers zero through
unlimited usage plus four times the triangle count plus 16; since the node slice
has twice the triangle count, that range includes the measured entry boundaries
and their 16-limit neighborhoods for the traced executions. The trace copy has
logging changes and is not itself used as the differential candidate. Its
initial preparation exited 1 before building because the instrumentation
assumed a single work implementation block; the corrected trace exited 0.

The corrected overdraw supervisor was rerun against the retained, hash-matched
binaries with the input directory. Both modes exited 0 with 20,558,564 exact
observations each and no differences. User CPU time was 26.84 and 29.35
seconds. This clears the new overdraw rerun item; it does not erase the original
exit 101 receipt.

The independently primed meshlet sweep used the same 140 cells and broad
work-limit range, with a small valid meshlet call at each tested limit before
two target calls. Both modes exited 0 with 323,556 target-call comparisons
per mode and no differences; 315,996 of 323,556 priming calls succeeded in
each mode. The 7,560 failed priming calls do not establish successful-warm
coverage. User CPU time was 6.95 and 7.15 seconds.

The independently primed cluster grid covered the same 32,768 cells per mode
and selected work limits as the earlier grid. Both modes exited 0 with
6,901,896 target-call comparisons and no differences per mode; 3,418,180 of
3,450,948 priming calls succeeded per mode. The remaining 32,768 priming calls
failed, leaving successful-warm coverage unproven for those observations. User
CPU time was 229.75 and 223.74 seconds respectively.

The corrected simplify supervisor was rerun in 32 deterministic shards per
mode. The input enumeration contains 102,073 comparison cells and 2,139,095,040
normalization bit patterns; each shard receives 3,189 or 3,190 cells and
66,846,720 normalization values. Shard 0 ran first in each mode; the remaining
shards ran in admitted groups of at most eight, with a 600-second CPU cap per
shard and at most 4,800 seconds of capped child CPU per group. Later groups ran
up to four shards concurrently. All 32 shards per mode exited 0. Their summed
observations exactly match the corrected earlier supervisor: 1,200,243,710
all-features and 273,906,930 no-default. The 32 recorded normalization ranges
are contiguous, disjoint and cover all 2,139,095,040 patterns in each mode. No
differences were reported. The original exit 101 receipts remain retained.
