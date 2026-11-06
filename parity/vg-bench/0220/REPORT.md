# 0.2.20 qualification disposition

**Human HOLD:** no active or queued lane jobs. Final retained-source gates,
storage-only isolation, disk accounting, new cycle profiles and target cleanup
are deferred. See [status](HOLD.md) and caller-owned `HOLD-REMAINING.md`.

## Continuation decision

**NOT QUALIFIED.** The weighted-region builder remains unchanged. The
four-triangle distance-only leaf experiment is rejected: it adds no bar passes
and regresses 1-thread medians on four of five meshes. Dragon changes by +0.43%
at 1T and +0.13% at 8T. Rocks improves only 0.91% / 4.59%. The three smaller
1T changes are +2.92%, +12.74% and +2.06%. Complete counterbalanced timings,
p95/noise and experimental bar booleans are in
[the rejected experiment](continuation/TABLE.md). The source patch and frozen
binary are retained; no second variant or repeated timing job is scheduled.

All ten experimental files and all ten retained controls match the prior
weighted-region MDAG 2 artifacts exactly. Thus the original cut, error and size
measurements apply without resampling. The fresh retained-arm ratios appear in
[RESULTS](../RESULTS.md). The experiment's temporary storage reductions are not
resident-size wins and do not override its timing regressions. Builder revision
6, container layouts, grid -22 and attribute encodings remain unchanged.

### Cut and calibration limits

The cut policy is retained. Rocks remains 1.675 at 8 px, dragon 1.072; neither
bar is closed. Fine-level simplification already achieves the requested 0.5
retained ratio. The prior measured late-level error distributions, indivisible
source ownership and non-root terminal geometry below remain the evidence for
the limit. Reducing original-source, local-step or inherited floors has no
accepted mixed-cut proof here, so no floor is weakened. This disposition does
not claim that a different geometric algorithm cannot improve the cuts.

Calibration remains the sampled cost target: the retained candidate's
original diagnostic has 69.54% of 1T user-cycle self samples in construction,
query and closest-triangle work. IBS remains unavailable: the prior corrected
attempt exited 255; the current host still reports perf_event_paranoid=1.
Cycle samples are diagnostic, not IBS acceptance. The rejected leaf experiment
reduces counted work and scratch storage, but fails to produce a consistent
1T time win.

### Sizing and final validation

The larger finite-budget calibration completes at 138,275,489 B /
1,236,391,663 visits for rocks and 170,885,388 B / 1,647,221,498 visits for
dragon. The helper suggests 16 MiB + 512T bytes and 2^24 + 4096T visits, with
checked arithmetic and explicit empirical scope. It is not a guarantee for
arbitrary topology, locks, attributes, materials or unused vertices.

The original-policy dragon low-budget case exhausted exactly 268,435,456
visits at 85,938,804 accounted bytes. Its complete high-budget run used
170,885,388 bytes. These observations identify work as the failed low-budget
case's limiting counter; the separate 512 MiB / high-work isolation remains
unrun. [Completed calibration receipts](continuation/retained-calibration.json)
retain both successful runs and the failed prefix.

The helper passed all five builds on the rejected experiment, with peaks
16,097,179 / 17,611,839 / 19,111,021 / 98,836,385 / 134,357,388 bytes. These are
experimental scratch observations, not retained-policy validation or resident
wins. [Experimental helper rows](continuation/limits.json) are kept separately.

The original file-size regression is not recovered. Rocks had only 0.000586
B/DAG triangle headroom over the reference before a 0.081452 increase (+0.652%).
Its cluster count rises 11,120 to 11,188; dragon rises 14,244 to 14,337. All five
file-byte ratios increase by 0.355% to 0.652%. These establish the structural
regression and narrow margin; an exact position/triangle/attribute disk-stream
breakdown is authored but unrun. No unmeasured compression recovery or container
win is claimed. Resident bytes and precision remain unchanged from the retained
weighted candidate.

The final two jobs were cancelled before admission at the human's renewed HOLD.
No post/attribution, retained-helper corpus or final-gate command ran. The earlier
experimental maximum tests and v2 integration tests exited 0, and the one
interleaved measurement campaign exited 0. Those do not qualify the final helper
or documentation. Final fmt, clippy, release, all-feature release, rustdoc,
public-text and legacy identity gates are **NOT RUN on the final retained source**.
The previous milestone's zero exits below remain historical only.

All remaining exact commands are in the caller-owned `HOLD-REMAINING.md`, mirrored
with portable settings in [HOLD](HOLD.md). The target is retained to avoid IO during
quiet time. No further job is scheduled, and Git state remains operator-owned.

## Previous milestone record (historical)

The following diagnosis and gates belong to the original weighted-region
campaign. Its earlier hold is superseded by the continuation above.

### Decision and diagnosis

Retain the weighted-region candidate for its bounded group sizes and measured
8-thread build improvement. **NOT QUALIFIED** against the complete frozen bars.
No further measurement campaign or policy variant is scheduled in this lane.

The previous policy partitioned individual clusters and then moved all replacement
siblings to their first partition. At a target of 16, maximum observed groups grew
to 82 clusters on rocks and 71 on dragon. The candidate partitions indivisible
regions using cluster counts as weights: the observed maximum is 21 on both.
This preserves single-parent source ownership and all original-source, single-step,
simplifier and inherited error floors. The public partition API retains unit weights.

| Mesh | Level | P95 error before | After | Terminal triangles before | After |
| --- | ---: | ---: | ---: | ---: | ---: |
| coast_rocks_01 | 3 | 0.016150642 | 0.007299260 | 0 | 0 |
| dragon | 3 | 0.027323040 | 0.006537252 | 0 | 0 |

The later levels expose the tradeoff. Non-root terminal triangles grow from
2,320 to 2,462 on rocks and 3,664 to 4,443 on dragon. The level-5 median stored
error grows from 0.04702 to 0.06527 on rocks and 0.15175 to 0.17162 on dragon.
Terminal group triangles here describe retained geometry, rather than the
writer's page-pinning budget; raw shape rows also record actual pinned bytes.

The achieved retained ratio is 0.5 through the main fine levels in both policies.
This is not a failure to request simplification: the target is reached, but the
error distribution and ownership determine whether the replacements enter a cut.
For rocks, at the 4 px threshold the 1024/2048 px spans contribute 62.2% of the
candidate's aggregate triangles and barely change. The 64 px span rises from
2,966 to 9,202 triangles. Its error outlier reduction therefore does not imply a
win on the aggregate equal-deviation bar. Source topology has 97 connected
components by shared vertex index on rocks; dragon has 151, including 146 with fewer than 16 triangles.
Component count is context, not an isolated proof that every retained triangle
is required by a removed component.

The local-step / stored-error medians at level 4 are 0.494 → 0.645 on rocks and
0.633 → 0.761 on dragon; level 5 is 0.993 → 0.748 and 0.481 → 0.684 respectively.
These are diagnostic local comparisons on the original positions; the builder
also prices the full original leaf union before snapping and inherits child errors.
The floors are retained. The measurements are sampled, not continuous Hausdorff
certificates, and reducing a stored floor without proving every mixed cut is safe
would change the error contract.

IBS attribution is **unavailable on this host configuration**. The initial
per-thread invocation was rejected because the PMU requires per-CPU mode; the
corrected per-CPU attempt also exited 255, reporting unavailable events and
performance-monitoring access denied at `perf_event_paranoid=1`. No IBS samples
were collected. CPU-cycle sampling, if successful, is reported separately and
cannot be substituted for the required IBS evidence.

Resident accounting retains the original -22 position grid and normal/UV payloads.
The existing two-channel attribute header costs 80 bytes per cluster. The table
separates that total attribute block from positions, triangle indices, descriptors
and resident metadata. A container change has not been justified with a measured
win exceeding 10%; major/minor and every chunk layout remain unchanged. Builder
revision 6 records the changed topology policy in cook keys.

### Before and after

Timing before uses fresh interleaved controls in [TABLE](TABLE.md). The following
quality and resident before columns retain the 0.2.19 campaign; the source changes
between that candidate and this lane base are documentation-only. All source
inputs match the frozen corpus. Candidate and reference after rows are fresh.

| Mesh | Cut ratio 5 px before → after | 8 px before → after | Resident B/DAG tri before → after |
| --- | ---: | ---: | ---: |
| bunny | 0.676 → 0.721 | 0.928 → 0.794 | 14.844 → 14.918 |
| dead_tree_trunk_02 | 0.710 → 0.709 | 0.782 → 0.781 | 14.970 → 15.037 |
| namaqualand_boulder_02 | 0.952 → 0.929 | 0.971 → 0.943 | 15.061 → 15.146 |
| coast_rocks_01 | 1.114 → 1.093 | 1.671 → 1.675 | 14.089 → 14.177 |
| dragon | 0.818 → 0.761 | 1.298 → 1.072 | 14.363 → 14.411 |

Dragon's 8 px cut improves by 17.5%, while its ratio remains 1.072 and fails.
Rocks remains at 1.675. The 8-thread time bar passes on rocks (0.457) and dragon
(0.459); it fails on the three smaller meshes. The 1-thread ratio spans
1.330–2.119 and fails everywhere. Resident size regresses slightly on every mesh
and the inherited file-size bar falls from 3/5 to 2/5 passes. Full booleans,
measured error values and byte accounting are in [summary](summary.json) and
[the table](TABLE.md). All cuts retain the original normal weight and precision.

### CPU-cycle diagnostic attribution

Four separate dragon profiles use `cycles:u` at 997 Hz, pinned to CPU 0 or CPUs
0–7. They completed with exit 0 and zero reported lost samples. These are user
cycle self-sample percentages, **not IBS**, and remain diagnostic evidence on a
shared host. Inlining and sampling skid limit attribution. Raw symbol percentages,
approximate event counts and sample headers are in [profiles](profiles.json).

| Routine | Baseline 1T | Candidate 1T | Baseline 8T | Candidate 8T |
| --- | ---: | ---: | ---: | ---: |
| Spatial-tree construction | 39.51% | 34.91% | 36.38% | 30.47% |
| Spatial-tree query | 16.68% | 19.08% | 14.80% | 16.65% |
| Closest-triangle helper | 14.05% | 15.55% | 11.83% | 12.76% |
| Maximum sample | 5.34% | 5.37% | 4.06% | 3.77% |
| Direction traversal | 4.57% | 4.50% | 3.99% | 3.69% |
| Cached witness | 1.64% | 1.88% | 1.30% | 1.42% |

Tree construction, query and closest-triangle work account for 69.54% of candidate
1-thread user-cycle samples and 59.88% at 8 threads. This identifies sampled
error calibration as the main remaining build-cost target; it is not a claim of
IBS qualification or proof of a particular future speedup.

### Resident format limit

The saved reference slices are counted directly in [reference sizes](reference-sizes.json).
Position and attribute costs below are bytes per total DAG triangle. Candidate
attributes include their per-cluster headers; reference attributes are normal and
UV slices. The candidate retains grid -22, while the reference uses its pinned
factor-4 centimetre quantisation. This is a format/precision cost comparison on
the measured artifacts, rather than a production residency measurement.

| Mesh | Positions candidate / reference | Attributes candidate / reference |
| --- | ---: | ---: |
| bunny | 5.100 / 2.125 | 6.068 / 7.901 |
| dead_tree_trunk_02 | 4.931 / 1.948 | 6.334 / 8.822 |
| namaqualand_boulder_02 | 5.109 / 2.159 | 6.282 / 8.766 |
| coast_rocks_01 | 4.303 / 1.352 | 6.127 / 8.131 |
| dragon | 4.594 / 1.638 | 6.071 / 7.936 |

### Pre-sizing-addendum code gates

| Command | Exit |
| --- | ---: |
| `cargo fmt --check` | 0 |
| `cargo clippy --all-targets --all-features -- -D warnings` | 0 |
| `cargo test --release` | 0 |
| `cargo test --release --all-features` | 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` | 0 |
| `bash parity/tools/check_public_text.sh` | 0 |
| `dual-dependency legacy clusterlod / MDAG 1 byte comparison` | 0 |

The legacy comparison links frozen baseline library sources and the candidate in
one executable, compares complete MDAG 1 bytes and direct fields on three grids,
and compares MDAG 1 plus hierarchy words for all five corpus meshes at 1 and 8
threads. The frozen baseline package version has a diagnostic suffix solely so
both dependencies can link. The driver asserts exact byte equality and worker
identity. Its [sources and lock](legacy-driver/README.md), [exact comparison receipts](legacy-proof.json),
and source hashes in [custody](custody.json) are retained.

The full measurement campaign exited 0. All six cut self-checks passed per mesh;
complete candidate and baseline files match across 1/8 workers on all five meshes.
`CHANGELOG.md` has an Unreleased entry. No sources under `fuzz/` were changed.

One initial candidate compile exited 101 because a child accounting context cannot
return a scalar directly; the corrected closure returns unit and captures the
partition count. No test ran in that failed attempt. The initial legacy gate also exited 101 before compilation because the frozen
manifest named omitted example files, followed by omitted include-str documentation.
Copying those unchanged files completed
the frozen package. A later driver compile rejected moving indices from the borrowed
callback; the driver now copies the slice. Equality assertions remain intact. The
completed driver exited 0; all three setup/driver failures are retained separately. Both failed IBS invocation
forms and the corrective build logs remain in caller-owned evidence. No assertions
or acceptance thresholds were weakened.

Canonical execution logs, profiles, binaries, normalized inputs and the frozen
baseline copy live under the caller's `VG_EVIDENCE` directory. The published
[metric rows](bunny.jsonl), [source/corpus/binary custody](custody.json),
[diagnostic distributions](diagnostics.jsonl), [bar booleans](summary.json) and
[cycle samples](profiles.json) retain the reviewable numeric record. The locked
harness and `qualify0220.py` reproduce the single campaign using caller-supplied
admission, evidence, corpus and disposable-target settings.

### Qualification boundary

The candidate is not qualified against the complete performance bars. The measured
limits above close this lane; no additional candidate variant is scheduled.
IBS attribution remains unavailable. GPU decode replay, production residency and
continuous geometric-error certification were not performed in this CPU builder lane.
Git integration and release publication remain operator-owned.


### Sizing addendum interrupted by human HOLD

Three finite-budget builds completed; rocks was interrupted and dragon was not
measured. The completed peaks/work were 20,792,603 B / 121,001,449 visits,
24,057,919 B / 140,396,021 visits, and 27,456,109 B / 166,471,905 visits for
bunny, trunk, and boulder respectively. See [partial sizing](limits-partial.json)
and [the HOLD record and resume commands](HOLD.md). These measurements do not
calibrate the reported consumer dragon case or justify a larger-mesh helper.

The earlier performance and correctness campaign remains bound to its frozen
sources/binaries in custody. Subsequent changes add the sizing diagnostic and
resource-sizing documentation; they do not change the build algorithm. The heavy
code gates were not rerun after the latest documentation under HOLD. Target
cleanup is also deferred to keep IO quiet. No active or queued lane jobs remain.
