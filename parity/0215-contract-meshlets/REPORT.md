# Contract meshlets and cluster LOD

**Instruction verdict: NO-GAIN.** All three real meshes beat optimesh for
standard meshlets, but candidate/base cluster LOD is 0.9240–0.9244, above
both the PASS 0.85 and PARTIAL 0.90 limits. No case exceeds 1.01× base.

The 0214 attribution experiment independently removed work accounting and root
memoization, attributing 55.5% and 29.8% of the pooled meshlet gap, and 41.8%
and 25.7% of the cluster-LOD gap, respectively. Those removals were experimental
and are not the candidate’s measured savings.

The candidate bounds each KD search before using unchecked limit comparisons
while charging work at its original points (`src/meshlet.rs:518`,
`src/workspace.rs:643`). It uses a native root for bounded moderate distances
(`src/meshlet.rs:564`) and guarded nonnegative finite LOD simplifier operands
(`src/clusterlod/support/math.rs:19`). Portable and unproven paths are retained.
`PROOFS.md` records the bounds.

## Instruction counts

Medians of three `instructions:u` samples per cell, 100% running; lower is
better. M7 is C++ 1.3; opt is optimesh 1.1.0, which has no cluster LOD.
Builder settings: 64 vertices, 124 maximum triangles; flex and spatial use
minimum 41, split factor 2 or fill weight 0.5. No wall-time conclusion.

| Input | Workload | Base | Candidate | M7 | Opt | Candidate/base | Candidate/opt | Candidate/M7 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| tiny-smooth | meshlets | 62228 | 61480 | 44896 | 62032 | 0.9880 | 0.9911 | 1.3694 |
| tiny-smooth | meshlets-flex | 62251 | 61509 | 44925 | 62083 | 0.9881 | 0.9908 | 1.3691 |
| tiny-smooth | meshlets-spatial | 46942 | 46942 | 47710 | 46387 | 1.0000 | 1.0120 | 0.9839 |
| tiny-smooth | cluster-lod | 101543 | 100795 | 75718 | — | 0.9926 | — | 1.3312 |
| medium-smooth | meshlets | 28410517 | 27835449 | 26954169 | 36763985 | 0.9798 | 0.7571 | 1.0327 |
| medium-smooth | meshlets-flex | 28410673 | 27835575 | 26953666 | 36763475 | 0.9798 | 0.7572 | 1.0327 |
| medium-smooth | meshlets-spatial | 28186808 | 28186797 | 21769424 | 26018370 | 1.0000 | 1.0833 | 1.2948 |
| medium-smooth | cluster-lod | 118868154 | 116357332 | 99770069 | — | 0.9789 | — | 1.1663 |
| medium-seams | meshlets | 21809456 | 21330291 | 20692685 | 29322563 | 0.9780 | 0.7274 | 1.0308 |
| medium-seams | meshlets-flex | 21809838 | 21330647 | 20692368 | 29322330 | 0.9780 | 0.7275 | 1.0308 |
| medium-seams | meshlets-spatial | 28256989 | 28256981 | 21786094 | 26053197 | 1.0000 | 1.0846 | 1.2970 |
| medium-seams | cluster-lod | 117577971 | 113471638 | 94525047 | — | 0.9651 | — | 1.2004 |
| medium-sparse | meshlets | 28522995 | 27947920 | 26982012 | 36779147 | 0.9798 | 0.7599 | 1.0358 |
| medium-sparse | meshlets-flex | 28522758 | 27947683 | 26981474 | 36778653 | 0.9798 | 0.7599 | 1.0358 |
| medium-sparse | meshlets-spatial | 28259899 | 28259910 | 21768610 | 26017313 | 1.0000 | 1.0862 | 1.2982 |
| medium-sparse | cluster-lod | 120682183 | 118171387 | 100759856 | — | 0.9792 | — | 1.1728 |
| million-smooth | meshlets | 4131410130 | 4053995521 | 3870400716 | 5200575713 | 0.9813 | 0.7795 | 1.0474 |
| million-smooth | meshlets-flex | 4141109315 | 4063224970 | 3875326617 | 5206483950 | 0.9812 | 0.7804 | 1.0485 |
| million-smooth | meshlets-spatial | 5965982334 | 5965982197 | 4576738594 | 5397187607 | 1.0000 | 1.1054 | 1.3035 |
| million-smooth | cluster-lod | 15711233821 | 15275046622 | 13010266351 | — | 0.9722 | — | 1.1741 |
| real-214e | meshlets | 18710578153 | 16170126110 | 13286175142 | 16849219659 | 0.8642 | 0.9597 | 1.2171 |
| real-214e | meshlets-flex | 18718480270 | 16176052609 | 13285981323 | 16849022595 | 0.8642 | 0.9601 | 1.2175 |
| real-214e | meshlets-spatial | 22587191697 | 22587192737 | 17115097100 | 20495619040 | 1.0000 | 1.1020 | 1.3197 |
| real-214e | cluster-lod | 93214294681 | 86126319365 | 65734164303 | — | 0.9240 | — | 1.3102 |
| real-a082 | meshlets | 19807728101 | 17228134882 | 14278322809 | 18265515921 | 0.8698 | 0.9432 | 1.2066 |
| real-a082 | meshlets-flex | 19815632586 | 17234066135 | 14278104551 | 18265287514 | 0.8697 | 0.9435 | 1.2070 |
| real-a082 | meshlets-spatial | 26254018370 | 26254018945 | 19903773111 | 23831327304 | 1.0000 | 1.1017 | 1.3190 |
| real-a082 | cluster-lod | 105108808704 | 97163228736 | 74324040740 | — | 0.9244 | — | 1.3073 |
| real-b6fd | meshlets | 4785506387 | 4155799013 | 3427833907 | 4407790372 | 0.8684 | 0.9428 | 1.2124 |
| real-b6fd | meshlets-flex | 4787480189 | 4157279396 | 3427779191 | 4407735676 | 0.8684 | 0.9432 | 1.2128 |
| real-b6fd | meshlets-spatial | 5872779319 | 5872779307 | 4458459143 | 5346358521 | 1.0000 | 1.0985 | 1.3172 |
| real-b6fd | cluster-lod | 26039224359 | 24065479400 | 18418150701 | — | 0.9242 | — | 1.3066 |

## Proof and limits

All-feature tests, no-default tests, all-target all-feature clippy, format,
boundary and public-text checks each exited 0. The v0.2.13 comparison exited 0:
154,590 observations across every work limit through success+1 for the selected
small meshes and options, including caller-buffer tails, allocating results,
usage, cluster callbacks and position bits. A coincident disconnected mesh
exercises broad KD traversal. C++ differential exited 0 for 1,200 builder
cases and 80 LOD cases, each in std and no_std. The widened deterministic fuzz
run exited 0 after 300.006 CPU seconds and 51,472,000 cases.

The exact comparison does **not** exhaust all option combinations, input layouts,
attribute/lock configurations or byte-limit boundaries. Consequently the
specification’s full exact-comparison proof remains unproven; these passing
checks do not establish complete contract equivalence. Fuzz is a deterministic
mutation run, not a substitute for that missing coverage.

Campaign: exit 0, 360 accepted samples, zero candidate/base or candidate/M7
output hash differences across the 32 cells; optimesh output hashes also match
where available. No cell regressed more than 1.01×. Build and per-arm cleanup
exits were 0. `counts.jsonl`, `results.json`, `inputs.json`,
`build-binding.json` and `proof-results.json` retain compact evidence; complete
output records, perf logs, executable binaries and admission logs remain in
scratch. The target cleanup is recorded separately.
