# Meshlet instruction qualification

**Instruction verdict: NO-GAIN. Proof: incomplete; not qualified for release.**

Base 0.2.13 at 8740a55. The candidate replaces the leaf-search root rejection bound with an exact f64 midpoint-square test (`src/meshlet.rs:622`, `nearest_root_at_least`); undecided cases still use the pinned root. It does not move work charges. The separately sampled attribution and source mapping are in ATTRIBUTION.md.

The full registered campaign completed 360 samples: median of three user-space instructions per arm, with 100% counter running. The peer consumer uses 64/124 for standard meshlets, flex min 64 / split 0 / weight 0, spatial min 64 / fill 0.5, and raster cluster-LOD defaults 128. Base and candidate normalized output hashes matched for every cell; raw meshlet hashes also matched. Values are ratios of medians, candidate divided by the named arm; lower is fewer instructions. The optimesh cluster-LOD arm is unavailable.

| Case | Workload | candidate/base | candidate/C++ 1.3 | candidate/optimesh |
|---|---|---:|---:|---:|
| tiny-smooth | meshlets | 1.00013 | 1.38454 | 1.00227 |
| tiny-smooth | meshlets-flex | 1.00024 | 1.38584 | 1.00331 |
| tiny-smooth | meshlets-spatial | 1.00015 | 0.98390 | 1.01253 |
| tiny-smooth | cluster-lod | 1.00004 | 1.34110 | — |
| medium-smooth | meshlets | 1.00006 | 1.05412 | 0.77284 |
| medium-smooth | meshlets-flex | 1.00006 | 1.05412 | 0.77284 |
| medium-smooth | meshlets-spatial | 1.00000 | 1.40946 | 1.18759 |
| medium-smooth | cluster-lod | 1.00004 | 1.19147 | — |
| medium-seams | meshlets | 1.00007 | 1.05407 | 0.74384 |
| medium-seams | meshlets-flex | 1.00007 | 1.05407 | 0.74384 |
| medium-seams | meshlets-spatial | 1.00000 | 1.41000 | 1.18757 |
| medium-seams | cluster-lod | 0.99999 | 1.24388 | — |
| medium-sparse | meshlets | 1.00006 | 1.05719 | 0.77558 |
| medium-sparse | meshlets-flex | 1.00007 | 1.05720 | 0.77558 |
| medium-sparse | meshlets-spatial | 1.00000 | 1.41272 | 1.19034 |
| medium-sparse | cluster-lod | 1.00004 | 1.19779 | — |
| million-smooth | meshlets | 1.00006 | 1.06752 | 0.79448 |
| million-smooth | meshlets-flex | 1.00006 | 1.06752 | 0.79448 |
| million-smooth | meshlets-spatial | 1.00000 | 1.47046 | 1.25111 |
| million-smooth | cluster-lod | 1.00004 | 1.20765 | — |
| real-214e | meshlets | 0.99518 | 1.40151 | 1.10514 |
| real-214e | meshlets-flex | 0.99518 | 1.40151 | 1.10514 |
| real-214e | meshlets-spatial | 1.00000 | 1.45898 | 1.22486 |
| real-214e | cluster-lod | 0.99940 | 1.41720 | — |
| real-a082 | meshlets | 0.99559 | 1.38117 | 1.07967 |
| real-a082 | meshlets-flex | 0.99559 | 1.38117 | 1.07967 |
| real-a082 | meshlets-spatial | 1.00000 | 1.45962 | 1.22558 |
| real-a082 | cluster-lod | 0.99956 | 1.41357 | — |
| real-b6fd | meshlets | 0.99520 | 1.38939 | 1.08049 |
| real-b6fd | meshlets-flex | 0.99520 | 1.38939 | 1.08049 |
| real-b6fd | meshlets-spatial | 1.00000 | 1.45028 | 1.21616 |
| real-b6fd | cluster-lod | 0.99951 | 1.41309 | — |

No case exceeded candidate/base 1.01. Candidate was not cheaper than optimesh for standard meshlets on any real mesh and did not reach candidate/base 0.90 for cluster LOD or standard meshlets on all real meshes. The registered result is therefore NO-GAIN.

## Proof and exit codes

- Repository tests with all features and no default features, clippy with warnings denied, formatting, boundary audit and public-text check: exit 0 each.
- Pinned C++ 1.3 differential: exit 0 for 2,000 seeds each of standard, flex and spatial meshlets, allocating and caller-buffer variants; 80 cluster-LOD cases: exit 0.
- Baseline/candidate small-mesh witness: exit 0, 51,739 work-limit comparisons in 441 cases. It covers selected layouts, error and short-buffer cases, cluster-LOD allocation/callback/DAG variants and an unsplittable KD leaf. A further 950 meshlet settings cases covered 115,830 work-limit comparisons: exit 0.
- Widened baseline/candidate differential fuzz: exit 0, 209,920 cases, 300.11 seconds of witness-process CPU, no mismatch. Inputs include valid disconnected meshes and malformed boundaries.
- Proof is not complete: the witness compares debug representations rather than raw floating-point bytes for cluster-LOD records, and does not exhaust the full cross-product of cluster-LOD options and input layouts. These results do not meet the spec's complete exact-proof requirement.
- Initial baseline witness build: exit 101 because the scratch archive omitted documentation; it succeeded after the archive was completed. The initial campaign launch exited 1 before collecting a sample because a binary name collided with a directory; the corrected campaign completed with exit 0. Earlier fuzz attempts were interrupted while widening the generator; their partial runs are not part of the five-minute result.

## Retained evidence

binding.json seals the candidate source, inputs, executable hashes and settings; counts.jsonl records every finished sample and verdict.json contains the medians. fuzz.json and options-result.json contain compact proof receipts. Raw inputs, profiles, executables, output buffers, failed attempts and full differential records remain in the job scratch area. The extra options sweep script is retained there as well. The target directory was removed after the jobs completed.
