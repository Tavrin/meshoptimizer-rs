# 0.2.20 current disposition

**BLOCKED / INCOMPLETE. Original qualification remains NOT QUALIFIED.**
Continuation 2 lifted the historical human hold. The new shading priority and
UV/tangent comparison are prepared but have not run. This report replaces the
older hold banner; archived reports remain historical evidence.

The required admission wrapper never started the first final compilation gate
during roughly two hours of waiting. It reported RAM/priority queues, admission
lock contention and renewed train windows, most recently until 21:49 UTC.
A 2 GiB measured reservation (9 GiB declaration) and the supported enforced
180-second CPU-proof deadline still did not gain admission. The queued pipeline
was canceled before compilation, exit 143, so it cannot apply changes unattended.
No admission control was bypassed. No final gate or new measurement is claimed.

## Retained calibration result

The caller-floor optimization is applied. It initializes the private sampled
maximum from a positional floor the caller already retains, preserving every
sample and avoiding queries that cannot raise stored errors. Five counterbalanced
pinned repetitions cover all five meshes at 1/8 workers against the unchanged
control and pinned reference. All ten complete MDAG 2 outputs match the previous
retained outputs exactly. The sum of per-mesh median build-plus-write ratios is
0.976438 at 1T (2.36% improvement), 0.992273 at 8T (0.77%). These satisfy the
bounded retention rule; no additional frozen qualification bar closes.

| Mesh | Candidate/control 1T / 8T | Candidate/reference 1T / 8T |
| --- | ---: | ---: |
| bunny | 1.01968 / 0.95506 | 2.121 / 0.732 |
| dead_tree_trunk_02 | 0.98432 / 1.03804 | 1.802 / 0.705 |
| namaqualand_boulder_02 | 0.98841 / 0.96805 | 1.749 / 0.614 |
| coast_rocks_01 | 0.98388 / 0.99239 | 1.290 / 0.447 |
| dragon | 0.96595 / 0.99353 | 1.286 / 0.448 |

[Raw timing, noise, identity and resource rows](continuation2/floor/) and the
[retention decision](continuation2/floor/decision.json) preserve the experiment.
The fresh retained 1T cycle profile still spends 35.70% in tree construction,
18.45% in nearest queries and 15.20% in closest-triangle arithmetic: 69.35%
together. Repeated original-source region searches remain dominant. This fresh
self-cycle profile differs from the earlier 69.54% target. IBS exits 255 and
supplies no IBS qualification.

The batch-scan/face-witness micro-optimization is rejected: no additional bar
passes, with 1T ratios 1.02236, 1.01437, 0.98509, 1.00820 and 1.02953.
[That experiment](continuation2/TABLE.md) retains its source patch, interleaved
rows, identity and diagnostics. The earlier four-triangle leaf experiment also
remains rejected. Neither is silently adopted.

| Frozen bar | Before continuation | Current retained calibration | Verdict |
| --- | ---: | ---: | --- |
| Build <= half reference, 1T | 0/5 | 0/5 | FAIL |
| Build <= half reference, 8T | 2/5 | 2/5 | FAIL |
| Cut triangles at equal deviation, 5 px | 4/5 | 4/5 | FAIL |
| Cut triangles at equal deviation, 8 px | 3/5 | 3/5 | FAIL |
| File bytes / DAG triangle | 2/5 | 2/5 | FAIL |
| Resident bytes / DAG triangle | 0/5 | 0/5 | FAIL |
| Sampled measured/claimed median and p95 | 5/5 | 5/5 | PASS under byte-identity reuse |

Unchanged files permit reuse of the existing cut/error/size rows; they were not
sampled again. Rocks remains 1.674574 at 8 px, dragon 1.0717. The earlier source
ownership, group-error distribution and terminal-geometry evidence remains in
[the historical report](continuation2/REPORT-before-admission-block.md).
No floor or mixed-cut oracle was weakened.

Storage-only dragon isolation succeeds at 512 MiB/high work: 170,885,388 bytes
and 1,647,221,498 visits. Its earlier low-work failure stops at exactly
268,435,456 visits. Disk accounting explains the rocks regression of
0.081452 B/DAG triangle against 0.000586 prior headroom: positions add 0.030959
and attribute extensions 0.043274, 91.1% of the increase.
[Disk accounting](continuation2/disk-accounting.json) is complete. The level-3
codec trial was interrupted when shading became priority; its failed diagnostic
header-key assertion is preserved and the proposed proof fix is unrun. No codec
win is claimed. The actual tree retains codec level 2 and builder revision 6.

## Shading request: unapplied draft

The [review patch](shading/DRAFT.patch) contains library, harness and regression
cases, but is **unapplied, type-unchecked and untested**. Static Rust syntax
parsing succeeds. No new shading field or builder revision is installed in the
library source. [Draft custody](shading/draft-review.json) binds the patch.

The proposed group term is `weight * L * chord = 2 * weight * L * sin(angle/2)`,
maximized with all positional and inherited floors. L is the largest triangle
edge in the local original-source/replacement region; separate regional maxima
are conservative. The unit-normal chord stays monotone through 180 degrees.
It uses the existing samples and nearest queries, squared chord comparisons
and one final square root. Repeated corners are deduplicated; constant-normal
and already-covered regions skip angular work. It applies only to supplied
normals; zero disables it. The proposed BuildMesh field and cook-key field share
a default, to be selected only after measurement. Revision 8 is proposed;
revision 7 was reserved for the abandoned codec trial. Legacy APIs stay unchanged.

Regression cases cover a planar patch with zero position error and lost tilted
center normal, linear weight scaling, finite overflow refusal, undefined
interpolants, no-normal byte identity, key binding and monotone errors with
unchanged topology. The independent decoded-cut evaluator includes BVH versus
brute-force nearest checks, sequential cut oracle and weighted quantile tests.
These tests have not run; parser success does not establish their correctness.

Prepared protocol: weights 0, 0.25, 0.5 and 1 on all five meshes; five interleaved
1/8T timings; 16 view directions at six projected sizes; 16,384 area probes in
each surface direction per unique one-pixel cut. Source probes are reused.
The default would be the smallest tested weight with dragon p95 normal angle
<=1.10 times weight one's result and mean triangles <=2 times weight zero.
If none qualifies, record why and keep the term disabled. No bar is relaxed.

The three scan meshes have authored UVs but no authored tangents. The prepared
matrix derives diagnostic UV-gradient tangents and compares all nine UV/tangent
weight pairs from {0.25,0.5,1}, with normals only through the dedicated field
(existing simplifier weight 0.5). Duplicated-normal controls model the accidental
consumer packing. A shared recommendation would preserve normal/UV/tangent p95
and handedness mismatch within 10% of correctly packed 1/1, then minimize
triangle count. No UV/tangent API is added. Derived tangents do not qualify
an authored or MikkTSpace tangent pipeline.

**The requested twenty-row shading table and UV/tangent numbers are UNMEASURED.
No default or weight recommendation is selected.** The disabled value in the
draft is a placeholder, not a measured decision. Exact replay/finalization
scripts and pending commands remain with the caller-owned evidence handoff.

## Gate and completion status

| Check | Current retained-floor source |
| --- | --- |
| fmt --check | 0 |
| Public text after blocked publication | 0 |
| Focused maximum and v2 tests | 0 |
| Final clippy | NOT RUN; admission wait canceled 143 |
| Final release / all-feature release | NOT RUN |
| Final rustdoc | NOT RUN |
| Final legacy clusterlod / MDAG 1 identity | NOT RUN |
| New shading / evaluator tests | NOT RUN |
| New shading default budgets / quality | NOT RUN |

The earlier retained-source full gate suite passed before the caller-floor
optimization; those exits do not certify the final source. The suggested helper
passed the retained floor on all five meshes. Its observations are recorded in
the API docs, but final source acceptance remains blocked. Fuzz sources, index and HEAD
are untouched. No commit or integration action was made.

No lane job is intentionally left queued. The dedicated target is retained for
the unfinished gates; deletion is pending completion and is not claimed. Resume
with retained-source gates, then apply/test/measure the draft, choose the default
and attribute weights, validate finite budgets and actual-default bytes, run all
final gates, publish measured results, and delete the target once jobs stop.
