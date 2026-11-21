# 0.2.20 disposition after shading priority

**NOT QUALIFIED against the original complete bars.** The shading request
superseded further rocks/dragon cut and storage experiments after calibration
was already complete. No commit or integration action is made in this lane.

Retain sampled normal-loss pricing in `clusterlod::v2`, with default weight
**0**. No tested weight satisfies both requested conditions; disabled default retained.
The full [five-mesh weight table and UV/tangent matrix](shading/TABLE.md) records
one-pixel triangle counts, pooled normal p95 and observed maximum, geometric
measured/claimed ratios, and pinned build timings. Source, binary, workload,
command, timing-noise, cut-oracle and worker-byte receipts accompany that table.
The selected default is also checked against explicit-weight artifact bytes
using the final implementation and fresh finite suggested budgets.

## Shading decision

Geometry-only sampled position error cannot price lost interpolated normal
variation on a flat patch. The regression fixture has zero positional error
and positive normal-chord error. The new term samples original-region normals
against nearest simplified interpolants in both directions using the existing
region point stream and queries. It maximizes local normal-chord displacement
with every retained geometric and inherited floor. It changes stored errors
and cuts, without changing topology. The API field is finite, nonnegative,
normal-only, and zero-disabled. Builder revision **8** binds the weight in cook
keys; MDAG 2 layouts and the level-2 vertex codec remain unchanged. Revision 7
was reserved for an abandoned codec trial. Legacy APIs are unchanged.

| Dragon at 1 px | Weight zero | Weight one | Selected default |
| --- | ---: | ---: | ---: |
| Mean cut triangles | 85,376.79 | 563,483.61 | 85,376.79 |
| Normal p95 degrees | 10.708272 | 2.734611 | 10.708272 |
| Maximum normal degrees | 176.911477 | 179.867146 | 176.911477 |

The requested quality ceiling is 3.008072 degrees
and the triangle ceiling is 170,753.58, twice weight
zero. Eligible tested weights: `[]`. These are CPU sampled cut
measurements; no new GPU-frame acceptance is claimed.
Normal maxima are not uniformly improved: dragon reaches 179.867 degrees at
weight one versus 176.911 at zero (weight 0.5 observes 28.352). The table retains
these tails rather than treating p95 improvement as a maximum-error guarantee.

Recommend **normals through the dedicated field only; UV 1,
tangent 1, normal-error weight 1**.
This is an opt-in shading-mode recommendation; the library default remains zero.
The comparison uses authored UVs on the three scan meshes, with diagnostic
UV-gradient tangents derived deterministically. It includes all nine UV/tangent
pairs and controls with accidentally duplicated normals. The table states the
recommendation rule and per-mesh triangle/deviation numbers. No UV/tangent API
is added. Derived tangents do not establish authored or MikkTSpace quality.

## Calibration and unresolved original bars

Retain the caller-floor maximum-query optimization: a caller's already retained
f32 floor initializes the private squared maximum, with no removed sample or
weakened floor. All five complete MDAG 2 files remain byte-identical at both
worker counts. Sum of per-mesh median candidate/control build-plus-write ratios
is **0.976438 at 1T** and
**0.992273 at 8T**. This is a bounded variant
retention result, not closure of the half-reference build bar. Raw floor timings
and the retained/rejected variants are preserved under continuation evidence.

The earlier batch-scan/face-witness micro-optimization is rejected: it closes no
additional bar and its 1T per-mesh ratios are 1.02236, 1.01437, 0.98509, 1.00820,
1.02953. The prior four-triangle leaf trial is also rejected. No variant is
silently adopted from those trials. Fresh cycle profiles are diagnostic; IBS
attempts exit 255 and supply no IBS qualification. The retained floor's fresh
1T cycle profile still spends 35.70% in search-tree construction, 18.45% in
nearest queries and 15.20% in closest-triangle arithmetic (69.35% together).
Repeated original-source region searches remain the dominant cost; the caller
floor saves queries without removing samples or weakening original-source error.
It does not eliminate that cost. These self-cycle shares are a different fresh
profile from the earlier 69.54% target, not an IBS or total-cycle speedup proof.

| Original bar | Before this continuation | Disposition |
| --- | --- | --- |
| Build <= half reference, 1T / 8T | 0/5 / 2/5 | FAIL; not closed, shading timings reported separately |
| Cut triangles at equal deviation, 5 / 8 px | 4/5 / 3/5 | FAIL; not closed, further cut work superseded |
| Disk bytes per DAG triangle | 2/5 | FAIL, 2/5; core disk lengths unchanged |
| Resident bytes per DAG triangle | 0/5 | FAIL, 0/5; topology/payload precision unchanged |
| Geometric measured/claimed median <= 1.3, p95 <= 1.5 | 5/5 PASS | PASS for all 20 shading rows; maximum median/p95 0.908530 / 0.975753 |
| Legacy clusterlod / MDAG 1 identity | Required | PASS, final frozen comparison |

At weight zero the retained rocks 8px ratio is 1.674574 and dragon is 1.0717.
Those original equal-deviation cuts were not remeasured for a nonzero shading
weight. Error distributions, source ownership and terminal geometry remain
recorded in the [historical report](continuation2/REPORT-before-shading.md).
No geometric floor or triangle oracle is relaxed to close a bar.

The rocks disk regression is +0.081452 B/DAG triangle, against only 0.000586
prior headroom. Position streams (+0.030959) and attribute extensions (+0.043274)
explain 91.1% of the increase. The level-3 codec trial was interrupted before
completion when shading became priority; its failed diagnostic header-key
assertion is preserved, the proposed proof fix is unrun, and no codec win is
claimed. The selected codec remains level 2.

## Budgets, gates and custody

The storage-only dragon isolation succeeds at 512 MiB/high work, using
170,885,388 accounted bytes and 1,647,221,498 visits on the original retained
policy. Its earlier low-work failure stops at exactly 268,435,456 visits.
This distinguishes the work limit from storage. Selected-default empirical
sizing is recorded in the API docs and [finite validation](shading/selected-validation/limits.json).
It is scoped to dedicated normals and this corpus; extra attributes, materials,
locks and arbitrary topology still require profiling.

Final gate exits: `fmt` **0**, `clippy` **0**, `release` **0**, `release-all-features` **0**, `doc` **0**, `public-text` **0**, `legacy-identity` **0**, `harness-tests` **0**, `all-features-doctests` **0**.
The first clippy attempt exited 101 on the new fixture tuple type; a named
fixture struct repaired it. An all-features compilation attempt exited 137
when systemd-oomd killed its scope. Limiting Cargo to two concurrent build jobs
allowed the retry to pass. Both failed attempts remain recorded separately.
The initial gates are a separate receipt. No new
unsafe code or library unwrap is introduced; fuzz sources are unchanged.
The dedicated target was removed after verifying no target environment holders
or live lane wrappers. [Final status and cleanup receipt](shading/continuation4-status.json).
Raw binary/measurement logs remain caller-owned; public JSON and replay scripts
use portable evidence/target variables. The source base and exact post-edit
hashes are retained with the experiment. Git remains operator-owned.
