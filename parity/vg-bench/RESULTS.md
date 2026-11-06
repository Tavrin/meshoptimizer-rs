# Virtual-geometry benchmark results

## Final 0.2.19 candidate qualification

Qualification verdict: **NOT QUALIFIED**. See the linked report for code gates and C++ failure-write observations.

Source `ca7ff9a31e9342bda30a403d66ea3ff05bfcd346`. Fresh measurements through the library v2 builder; historical tables below retain their earlier revisions.

# Final virtual-geometry qualification measurements

Candidate v2 builder and MDAG 2 attribute payloads against the pinned reference. Five interleaved repeats; median build plus serialise milliseconds. Ratios below 1 favor the candidate.

| Mesh | Build ratio 1T / 8T | File B/DAG tri ours / ref | Resident B/DAG tri ours / ref | Cut tris ratio at 5 / 8 px | Measured/claimed median ours / ref |
|---|---:|---:|---:|---:|---:|
| bunny | 2.023 / 0.863 | 13.06 / 13.01 | 14.84 / 13.78 | 0.676 / 0.928 | 1.000 / 1.161 |
| coast_rocks_01 | 1.352 / 0.566 | 12.49 / 12.49 | 14.09 / 13.23 | 1.114 / 1.671 | 0.980 / 1.166 |
| dead_tree_trunk_02 | 1.889 / 0.741 | 13.29 / 13.84 | 14.97 / 14.52 | 0.710 / 0.782 | 0.905 / 1.199 |
| dragon | 1.298 / 0.520 | 12.58 / 11.68 | 14.36 / 13.32 | 0.818 / 1.298 | 0.930 / 1.437 |
| namaqualand_boulder_02 | 1.715 / 0.656 | 13.31 / 13.93 | 15.06 / 14.67 | 0.952 / 0.971 | 0.979 / 1.100 |

Bar passes (out of five meshes): build_half_reference_1T: 0/5, build_half_reference_8T: 0/5, file_per_dag_triangle: 3/5, resident_per_dag_triangle: 0/5, fewer_cut_triangles_5px: 4/5, fewer_cut_triangles_8px: 3/5, measured_claimed_median_1_3: 5/5, self_checks: 5/5, thread_byte_identity: 5/5.

Full qualification: [report](../0219-qualify/REPORT.md), [table](../0219-qualify/TABLE.md).

Earlier policy rerun: base `420220c49b375936a3d47b45955638e86a99cf63`, builder revision 3.
The historical MDAG 1 tables below retain their original source revision and method.

## MDAG 2 compact distance records

Starting source: `ab13064a1cf8f02957663eacbc83cec001221f56` (builder revision 5).
The builder now measures only the symmetric maximum. Each sample first tries the
previous target triangle, then traverses the existing BVH until a distance witnesses
that it cannot raise the running maximum. Samples that can raise it finish the
original exact query. The reverse direction starts with the forward maximum.
Repeated corners reuse their earlier bound through a budgeted vertex bitset.
Distance-only BVH records omit unused normals, reducing each triangle record
from 152 to 80 bytes. Public statistics retain their complete normal records.
Cached witnesses try the existing face-distance candidate before edge distances;
samples that can raise the maximum still finish the unchanged exact query.
Every sample visit/count, interior sample, distance arithmetic, recorded error, grouping
and file format are retained. Public RMS and normal statistics use the full query.

Build plus write milliseconds: six counterbalanced interleaved repeats, rotating
the retained revision-3, current and reference binary order; core 2 at 1T and
cores 2–9 at 8T on a shared host. Cells are median / maximum (p95 of six).
The reference is built from the pinned harness dependencies and measured in
each repeat on the same inputs and cores. Profiles are separate.

| Mesh | Revision 3 1T | Current 1T | Reference 1T | Revision 3 8T | Current 8T | Reference 8T | 8T / revision 3 | 8T / reference |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 1024.8 / 1203.5 | 1182.8 / 1286.3 | 557.4 / 643.6 | 387.1 / 407.5 | 516.4 / 541.5 | 528.7 / 554.9 | 1.33x | 0.98x |
| dead_tree_trunk_02 | 1375.2 / 1486.4 | 1250.9 / 1326.1 | 640.9 / 675.5 | 497.0 / 531.5 | 552.7 / 586.1 | 651.7 / 659.4 | 1.11x | 0.85x |
| namaqualand_boulder_02 | 1461.8 / 1560.0 | 1416.5 / 1492.5 | 784.8 / 862.6 | 513.7 / 570.2 | 580.7 / 600.9 | 780.4 / 809.0 | 1.13x | 0.74x |
| coast_rocks_01 | 9706.5 / 10123.0 | 11605.6 / 12411.2 | 7688.5 / 8165.4 | 3078.4 / 3258.5 | 4868.3 / 5104.9 | 7576.8 / 8101.6 | 1.58x | 0.64x |
| dragon | 11454.6 / 11502.7 | 14742.5 / 15191.5 | 9716.4 / 12602.3 | 3642.2 / 3690.2 | 5510.8 / 5758.0 | 9728.6 / 9765.1 | 1.51x | 0.57x |

Complete output bytes match the pinned starting source at 1T and 8T on all five
meshes, and match between thread counts. SHA-256 of each complete file:

| Mesh | SHA-256 (1T = 8T = starting source) |
| --- | --- |
| bunny | `eaba70a062072139b2c536ccf46b7a109fed74695301740ae53cd09b8b7612d5` |
| dead_tree_trunk_02 | `80bb1fff3126c18609ee9a159b3d5a2da1003cc8aa4343cea95d85ee3d568108` |
| namaqualand_boulder_02 | `3ba6e6bcafc566fb5d37f2ca3f513a035032d8e5d2d182cff286ed6f4c32d98f` |
| coast_rocks_01 | `c510ac833c759d10abf712a61b811ecdc4509b5be18c467a37c218b28a23886e` |
| dragon | `a7f7931c35c3f5a5a206142470e9c13eca68bd40caf5d74db9a7596e3db4c50b` |

IBS op sampling uses dragon separately from timing. Percentages below are
self samples; cached-witness work appears in sample / face-witness routines.
A dash means no separate symbol entry (inlining can move attribution).
Kernel symbols may be restricted; raw profiles retain warnings and loss counts.

| Routine | Starting source | Current |
| --- | ---: | ---: |
| Tree query | 76.39% | 17.03% |
| Sample / cached witness | 0.88% | 2.08% |
| Closest helper | — | 10.17% |
| Face witness | — | 2.55% |
| BVH partition | 3.82% | 0.00% |
| BVH construction | 1.98% | 35.71% |
| Memory copies | 9.24% | 0.32% |

Decision: timing bars met on all five meshes.

## MDAG 2 corner-query reuse

Starting source: `ab13064a1cf8f02957663eacbc83cec001221f56` (builder revision 5).
The builder now measures only the symmetric maximum. Each sample first tries the
previous target triangle, then traverses the existing BVH until a distance witnesses
that it cannot raise the running maximum. Samples that can raise it finish the
original exact query. The reverse direction starts with the forward maximum.
Repeated corners reuse their earlier bound through a budgeted vertex bitset.
Every sample visit/count, interior sample, distance arithmetic, recorded error, grouping
and file format are retained. Public RMS and normal statistics use the full query.

Build plus write milliseconds: six counterbalanced interleaved repeats, rotating
the retained revision-3, current and reference binary order; core 2 at 1T and
cores 2–9 at 8T on a shared host. Cells are median / maximum (p95 of six).
The reference is built from the pinned harness dependencies and measured in
each repeat on the same inputs and cores. Profiles are separate.

| Mesh | Revision 3 1T | Current 1T | Reference 1T | Revision 3 8T | Current 8T | Reference 8T | 8T / revision 3 | 8T / reference |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 1199.6 / 1624.4 | 2042.1 / 2365.3 | 887.9 / 1173.5 | 526.9 / 824.7 | 1032.3 / 1320.2 | 824.4 / 1102.6 | 1.96x | 1.25x |
| dead_tree_trunk_02 | 1871.2 / 2051.8 | 1995.6 / 3090.9 | 858.9 / 1032.0 | 560.1 / 805.2 | 1009.7 / 1337.3 | 808.4 / 1005.1 | 1.80x | 1.25x |
| namaqualand_boulder_02 | 1939.7 / 2770.5 | 2550.7 / 3498.4 | 1068.4 / 1502.3 | 625.3 / 1076.8 | 1048.7 / 1323.6 | 1063.0 / 1350.6 | 1.68x | 0.99x |
| coast_rocks_01 | 10021.8 / 14328.9 | 18123.9 / 21470.4 | 8641.9 / 12996.6 | 3218.3 / 3434.2 | 7058.9 / 8644.5 | 8640.6 / 10243.3 | 2.19x | 0.82x |
| dragon | 13586.2 / 14883.2 | 24294.2 / 24688.6 | 13337.0 / 14379.0 | 4054.3 / 5089.4 | 9467.7 / 9804.2 | 11936.1 / 13437.6 | 2.34x | 0.79x |

Complete output bytes match the pinned starting source at 1T and 8T on all five
meshes, and match between thread counts. SHA-256 of each complete file:

| Mesh | SHA-256 (1T = 8T = starting source) |
| --- | --- |
| bunny | `eaba70a062072139b2c536ccf46b7a109fed74695301740ae53cd09b8b7612d5` |
| dead_tree_trunk_02 | `80bb1fff3126c18609ee9a159b3d5a2da1003cc8aa4343cea95d85ee3d568108` |
| namaqualand_boulder_02 | `3ba6e6bcafc566fb5d37f2ca3f513a035032d8e5d2d182cff286ed6f4c32d98f` |
| coast_rocks_01 | `c510ac833c759d10abf712a61b811ecdc4509b5be18c467a37c218b28a23886e` |
| dragon | `a7f7931c35c3f5a5a206142470e9c13eca68bd40caf5d74db9a7596e3db4c50b` |

IBS op sampling uses dragon separately from timing. Percentages below are
self samples; cached-witness distance work appears in the sample routine.
A dash means no separate symbol entry (inlining can move attribution).
Kernel symbols may be restricted; raw profiles retain warnings and loss counts.

| Routine | Starting source | Current |
| --- | ---: | ---: |
| Tree query | 75.81% | 10.95% |
| Sample / cached witness | 0.90% | 2.07% |
| Closest helper | — | 8.27% |
| BVH partition | 3.88% | 13.30% |
| BVH construction | 1.99% | 7.36% |
| Memory copies | 9.40% | 33.99% |

Decision: not qualified; retain the measured improvement and failed timing bars.

## MDAG 2 maximum-only distance queries

Starting source: `ab13064a1cf8f02957663eacbc83cec001221f56` (builder revision 5).
The builder now measures only the symmetric maximum. Each sample first tries the
previous target triangle, then traverses the existing BVH until a distance witnesses
that it cannot raise the running maximum. Samples that can raise it finish the
original exact query. The reverse direction starts with the forward maximum.
Every corner and interior sample, distance arithmetic, recorded error, grouping
and file format are retained. Public RMS and normal statistics use the full query.

Build plus write milliseconds: five interleaved repeats, alternating order against
a retained revision-3 binary; core 2 at 1T and cores 2–9 at 8T on a shared host.
Cells are median / maximum. The reference columns retain the earlier five-run
reference measurements from the build-time table below; they are historical controls,
not contemporaneous interleaved reference measurements. Profiles are separate.

| Mesh | Revision 3 1T | Current 1T | Reference 1T | Revision 3 8T | Current 8T | Reference 8T | 8T / revision 3 | 8T / reference |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 985.4 / 1217.0 | 1913.9 / 1971.7 | 516 | 361.6 / 375.3 | 744.3 / 838.5 | 511 | 2.06x | 1.46x |
| dead_tree_trunk_02 | 1392.2 / 1448.4 | 2023.3 / 2162.2 | 659 | 455.4 / 467.0 | 749.1 / 828.2 | 632 | 1.64x | 1.19x |
| namaqualand_boulder_02 | 2019.5 / 2383.0 | 2954.1 / 3347.5 | 993 | 990.5 / 1289.1 | 1773.0 / 1951.6 | 783 | 1.79x | 2.26x |
| coast_rocks_01 | 16059.6 / 19269.0 | 31254.3 / 39524.4 | 9464 | 4661.5 / 8555.5 | 16021.2 / 22069.2 | 7571 | 3.44x | 2.12x |
| dragon | 14540.0 / 24078.7 | 29524.7 / 37839.1 | 11661 | 4508.7 / 5504.8 | 11221.4 / 18773.6 | 11344 | 2.49x | 0.99x |

Complete output bytes match the pinned starting source at 1T and 8T on all five
meshes, and match between thread counts. SHA-256 of each complete file:

| Mesh | SHA-256 (1T = 8T = starting source) |
| --- | --- |
| bunny | `eaba70a062072139b2c536ccf46b7a109fed74695301740ae53cd09b8b7612d5` |
| dead_tree_trunk_02 | `80bb1fff3126c18609ee9a159b3d5a2da1003cc8aa4343cea95d85ee3d568108` |
| namaqualand_boulder_02 | `3ba6e6bcafc566fb5d37f2ca3f513a035032d8e5d2d182cff286ed6f4c32d98f` |
| coast_rocks_01 | `c510ac833c759d10abf712a61b811ecdc4509b5be18c467a37c218b28a23886e` |
| dragon | `a7f7931c35c3f5a5a206142470e9c13eca68bd40caf5d74db9a7596e3db4c50b` |

IBS op sampling uses dragon separately from timing. Percentages below are
self samples; cached-witness distance work appears in the sample routine.
A dash means no separate symbol entry (inlining can move attribution).
Kernel symbols may be restricted; raw profiles retain warnings and loss counts.

| Routine | Starting source | Current |
| --- | ---: | ---: |
| Tree query | 75.69% | 16.57% |
| Sample / cached witness | 0.88% | 3.75% |
| Closest helper | — | 13.67% |
| BVH partition | 3.89% | 11.27% |
| BVH construction | 2.00% | 5.97% |
| Memory copies | 9.45% | 28.10% |

Decision: not qualified; retain the measured improvement and failed timing bars.

## MDAG 2 local source regions (builder revision 5)

Candidate starts at `c7377dd7d96d02c394daaf34c6c1ce8b4422caf2`. The retained revision-3
binary from the earlier source-region comparison is the interleaved timing control.
Inputs, normals at 0.5, grid -22 and 128 triangles per cluster are unchanged.

Decision: keep replacement siblings in one next-level partition. Recursive source unions
otherwise include geometry assigned to sibling parents. This inflates local measurements
and inherited maxima spread that inflation. Component pruning was already disabled.
The original-source sampled bound and regression remain; errors are not scaled down.
Independent measurements run in worker-sized parallel batches for unlimited workspaces;
finite workspaces remain serial. Nearest-triangle spatial indices and the existing
4000 area-scaled interior sampling rule plus all corners remain in use. This is not a
continuous Hausdorff certificate or a hard total sample cap.

Quality: 64 directions, six spans, 200,000 source samples. Ratios use scene units at 1 px.

| Mesh | Median / p95 ratio | Mean triangles (1 px) | / revision 3 | P95 deviation at 0.5 / 1 / 2 / 4 px | Quality / triangle / trend |
| --- | ---: | ---: | ---: | --- | --- |
| bunny | 1.000 / 1.103 | 22865 | 1.02x | 0.723 / 2.511 / 4.500 / 8.434 | pass / pass / pass |
| dead_tree_trunk_02 | 0.905 / 1.061 | 36457 | 1.06x | 0.631 / 1.596 / 5.407 / 10.806 | pass / pass / pass |
| namaqualand_boulder_02 | 0.979 / 1.000 | 49815 | 1.02x | 0.534 / 1.188 / 2.859 / 9.623 | pass / pass / pass |
| coast_rocks_01 | 0.980 / 1.000 | 85340 | 1.28x | 1.459 / 2.811 / 5.420 / 10.240 | pass / FAIL / pass |
| dragon | 0.930 / 1.000 | 97201 | 1.21x | 1.531 / 2.831 / 4.575 / 9.805 | pass / pass / pass |

Build plus write ms: median / p95 (maximum of five). Five interleaved repeats,
alternating binary order; core 2 for 1T, cores 2–9 for 8T, shared host.
Profiling runs are excluded. Speedup compares candidate 1T and 8T medians.

| Mesh | Revision 3 1T | Candidate 1T | Revision 3 8T | Candidate 8T | 8T cost | Speedup | <= 2x |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| bunny | 951.3 / 980.9 | 3811.0 / 3902.3 | 370.7 / 400.3 | 1390.0 / 1418.2 | 3.75x | 2.74x | FAIL |
| dead_tree_trunk_02 | 1405.1 / 1459.9 | 5354.8 / 5380.9 | 473.0 / 498.9 | 1966.3 / 1987.4 | 4.16x | 2.72x | FAIL |
| namaqualand_boulder_02 | 1510.0 / 1578.8 | 5689.9 / 5935.2 | 530.7 / 538.3 | 1880.6 / 1947.4 | 3.54x | 3.03x | FAIL |
| coast_rocks_01 | 10006.1 / 10119.5 | 45298.1 / 48439.2 | 2988.8 / 3079.9 | 16548.3 / 17477.5 | 5.54x | 2.74x | FAIL |
| dragon | 12061.9 / 12874.9 | 73141.9 / 76878.4 | 3850.3 / 3977.1 | 29086.2 / 29870.4 | 7.55x | 2.51x | FAIL |

Complete candidate files compare byte-identical at 1T and 8T on all five meshes.
All 15 candidate cut self-checks passed. The earlier 30 revision-3/revision-4 checks
remain historical evidence, separate from this candidate.

Per-level diagnosis follows. Errors are in normalized scene units; finite groups exclude
pinned terminal/stuck groups. Diagnostic percentiles use round((n-1)*p).
Shared means replacement clusters appear in multiple
parent groups. Candidate shared count is zero by construction.

| Mesh | Policy | Level | Finite | Stuck/root | Shared | Median | P95 | Max |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | baseline | 0 | 33 | 0 | 29 | 0.00177223 | 0.00222124 | 0.00243506 |
| bunny | baseline | 1 | 18 | 0 | 15 | 0.526145 | 0.637505 | 0.658737 |
| bunny | baseline | 2 | 9 | 0 | 9 | 0.940634 | 1.08831 | 1.08831 |
| bunny | baseline | 3 | 5 | 0 | 5 | 1.13405 | 1.29861 | 1.29861 |
| bunny | baseline | 4 | 3 | 0 | 0 | 1.29861 | 1.33374 | 1.33374 |
| bunny | baseline | 5 | 1 | 0 | 0 | 1.33374 | 1.33374 | 1.33374 |
| bunny | baseline | 6 | 1 | 0 | 0 | 1.33374 | 1.33374 | 1.33374 |
| bunny | baseline | 7 | 1 | 0 | 0 | 1.33374 | 1.33374 | 1.33374 |
| bunny | baseline | 8 | 1 | 0 | 0 | 1.33374 | 1.33374 | 1.33374 |
| bunny | baseline | 9 | 1 | 0 | 0 | 1.33374 | 1.33374 | 1.33374 |
| bunny | baseline | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| bunny | candidate | 0 | 33 | 0 | 0 | 0.00177223 | 0.00222124 | 0.00243506 |
| bunny | candidate | 1 | 14 | 0 | 0 | 0.00373443 | 0.00466539 | 0.0046853 |
| bunny | candidate | 2 | 8 | 0 | 0 | 0.00834481 | 0.0154591 | 0.0154591 |
| bunny | candidate | 3 | 4 | 0 | 0 | 0.0187442 | 0.316141 | 0.316141 |
| bunny | candidate | 4 | 3 | 0 | 0 | 0.0427703 | 0.316141 | 0.316141 |
| bunny | candidate | 5 | 1 | 0 | 0 | 0.316141 | 0.316141 | 0.316141 |
| bunny | candidate | 6 | 1 | 0 | 0 | 0.316141 | 0.316141 | 0.316141 |
| bunny | candidate | 7 | 1 | 0 | 0 | 0.316141 | 0.316141 | 0.316141 |
| bunny | candidate | 8 | 1 | 0 | 0 | 0.316141 | 0.316141 | 0.316141 |
| bunny | candidate | 9 | 1 | 0 | 0 | 0.371513 | 0.371513 | 0.371513 |
| bunny | candidate | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| dead_tree_trunk_02 | baseline | 0 | 42 | 0 | 36 | 0.00208594 | 0.00251979 | 0.002854 |
| dead_tree_trunk_02 | baseline | 1 | 22 | 0 | 17 | 0.166303 | 0.306285 | 0.374169 |
| dead_tree_trunk_02 | baseline | 2 | 11 | 0 | 11 | 0.347631 | 0.504235 | 0.504235 |
| dead_tree_trunk_02 | baseline | 3 | 5 | 0 | 3 | 0.504235 | 0.671609 | 0.671609 |
| dead_tree_trunk_02 | baseline | 4 | 3 | 0 | 1 | 0.671609 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 5 | 2 | 0 | 0 | 1.18252 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 6 | 1 | 0 | 0 | 1.18252 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 7 | 1 | 0 | 0 | 1.18252 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 8 | 1 | 0 | 0 | 1.18252 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 9 | 1 | 0 | 0 | 1.18252 | 1.18252 | 1.18252 |
| dead_tree_trunk_02 | baseline | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| dead_tree_trunk_02 | candidate | 0 | 42 | 0 | 0 | 0.00208594 | 0.00251979 | 0.002854 |
| dead_tree_trunk_02 | candidate | 1 | 19 | 0 | 0 | 0.00407997 | 0.00456704 | 0.00458682 |
| dead_tree_trunk_02 | candidate | 2 | 10 | 0 | 0 | 0.00828203 | 0.00952198 | 0.00952198 |
| dead_tree_trunk_02 | candidate | 3 | 5 | 0 | 0 | 0.012856 | 0.0844187 | 0.0844187 |
| dead_tree_trunk_02 | candidate | 4 | 2 | 1 | 0 | 0.0388646 | 0.0388646 | 0.0388646 |
| dead_tree_trunk_02 | candidate | 5 | 1 | 0 | 0 | 0.0388646 | 0.0388646 | 0.0388646 |
| dead_tree_trunk_02 | candidate | 6 | 1 | 0 | 0 | 0.116048 | 0.116048 | 0.116048 |
| dead_tree_trunk_02 | candidate | 7 | 1 | 0 | 0 | 0.120019 | 0.120019 | 0.120019 |
| dead_tree_trunk_02 | candidate | 8 | 1 | 0 | 0 | 0.120047 | 0.120047 | 0.120047 |
| dead_tree_trunk_02 | candidate | 9 | 1 | 0 | 0 | 0.334763 | 0.334763 | 0.334763 |
| dead_tree_trunk_02 | candidate | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| namaqualand_boulder_02 | baseline | 0 | 48 | 0 | 39 | 0.00348059 | 0.00434942 | 0.00597123 |
| namaqualand_boulder_02 | baseline | 1 | 26 | 0 | 23 | 0.282625 | 0.4595 | 0.566619 |
| namaqualand_boulder_02 | baseline | 2 | 12 | 0 | 11 | 0.496391 | 0.709417 | 0.8042 |
| namaqualand_boulder_02 | baseline | 3 | 7 | 0 | 4 | 0.8042 | 1.02058 | 1.02058 |
| namaqualand_boulder_02 | baseline | 4 | 3 | 0 | 1 | 0.962413 | 1.02058 | 1.02058 |
| namaqualand_boulder_02 | baseline | 5 | 2 | 0 | 0 | 1.19399 | 1.19399 | 1.19399 |
| namaqualand_boulder_02 | baseline | 6 | 1 | 0 | 0 | 1.19399 | 1.19399 | 1.19399 |
| namaqualand_boulder_02 | baseline | 7 | 1 | 0 | 0 | 1.19399 | 1.19399 | 1.19399 |
| namaqualand_boulder_02 | baseline | 8 | 1 | 0 | 0 | 1.19399 | 1.19399 | 1.19399 |
| namaqualand_boulder_02 | baseline | 9 | 1 | 0 | 0 | 1.19399 | 1.19399 | 1.19399 |
| namaqualand_boulder_02 | baseline | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| namaqualand_boulder_02 | candidate | 0 | 48 | 0 | 0 | 0.00348059 | 0.00434942 | 0.00597123 |
| namaqualand_boulder_02 | candidate | 1 | 21 | 0 | 0 | 0.00660356 | 0.00889838 | 0.00970853 |
| namaqualand_boulder_02 | candidate | 2 | 10 | 0 | 0 | 0.0135642 | 0.0222201 | 0.0222201 |
| namaqualand_boulder_02 | candidate | 3 | 6 | 0 | 0 | 0.0371459 | 0.0932827 | 0.0932827 |
| namaqualand_boulder_02 | candidate | 4 | 3 | 0 | 0 | 0.0801461 | 0.283415 | 0.283415 |
| namaqualand_boulder_02 | candidate | 5 | 1 | 1 | 0 | 0.0801461 | 0.0801461 | 0.0801461 |
| namaqualand_boulder_02 | candidate | 6 | 1 | 0 | 0 | 0.0801461 | 0.0801461 | 0.0801461 |
| namaqualand_boulder_02 | candidate | 7 | 1 | 0 | 0 | 0.120418 | 0.120418 | 0.120418 |
| namaqualand_boulder_02 | candidate | 8 | 1 | 0 | 0 | 0.181246 | 0.181246 | 0.181246 |
| namaqualand_boulder_02 | candidate | 9 | 1 | 0 | 0 | 0.270338 | 0.270338 | 0.270338 |
| namaqualand_boulder_02 | candidate | 10 | 0 | 1 | 0 | 0 | 0 | 0 |
| coast_rocks_01 | baseline | 0 | 330 | 0 | 300 | 0.000590465 | 0.000777104 | 0.00104335 |
| coast_rocks_01 | baseline | 1 | 172 | 0 | 157 | 0.0725723 | 0.196015 | 0.907263 |
| coast_rocks_01 | baseline | 2 | 88 | 0 | 80 | 0.215401 | 0.903007 | 1.01785 |
| coast_rocks_01 | baseline | 3 | 45 | 0 | 41 | 0.47846 | 0.971783 | 1.01785 |
| coast_rocks_01 | baseline | 4 | 22 | 0 | 18 | 0.775523 | 1.01785 | 1.04986 |
| coast_rocks_01 | baseline | 5 | 11 | 0 | 8 | 0.971783 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 6 | 6 | 0 | 2 | 1.01785 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 7 | 3 | 0 | 1 | 1.01785 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 8 | 2 | 0 | 0 | 1.10531 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 9 | 1 | 0 | 0 | 1.10531 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 10 | 1 | 0 | 0 | 1.10531 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 11 | 1 | 0 | 0 | 1.10531 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 12 | 1 | 0 | 0 | 1.10531 | 1.10531 | 1.10531 |
| coast_rocks_01 | baseline | 13 | 0 | 1 | 0 | 0 | 0 | 0 |
| coast_rocks_01 | candidate | 0 | 330 | 0 | 0 | 0.000590465 | 0.000777104 | 0.00104335 |
| coast_rocks_01 | candidate | 1 | 139 | 0 | 0 | 0.00113766 | 0.00170308 | 0.00207366 |
| coast_rocks_01 | candidate | 2 | 65 | 0 | 0 | 0.0022303 | 0.00341727 | 0.00424364 |
| coast_rocks_01 | candidate | 3 | 32 | 0 | 0 | 0.00484461 | 0.0161506 | 0.0257305 |
| coast_rocks_01 | candidate | 4 | 16 | 0 | 0 | 0.0230879 | 0.0523281 | 0.0557702 |
| coast_rocks_01 | candidate | 5 | 5 | 4 | 0 | 0.0470242 | 0.139238 | 0.139238 |
| coast_rocks_01 | candidate | 6 | 3 | 0 | 0 | 0.051685 | 0.139238 | 0.139238 |
| coast_rocks_01 | candidate | 7 | 2 | 0 | 0 | 0.143412 | 0.143412 | 0.143412 |
| coast_rocks_01 | candidate | 8 | 1 | 0 | 0 | 0.143702 | 0.143702 | 0.143702 |
| coast_rocks_01 | candidate | 9 | 1 | 0 | 0 | 0.143702 | 0.143702 | 0.143702 |
| coast_rocks_01 | candidate | 10 | 1 | 0 | 0 | 0.156346 | 0.156346 | 0.156346 |
| coast_rocks_01 | candidate | 11 | 1 | 0 | 0 | 0.211475 | 0.211475 | 0.211475 |
| coast_rocks_01 | candidate | 12 | 1 | 0 | 0 | 0.211475 | 0.211475 | 0.211475 |
| coast_rocks_01 | candidate | 13 | 0 | 1 | 0 | 0 | 0 | 0 |
| dragon | baseline | 0 | 421 | 0 | 377 | 0.000297262 | 0.000448512 | 0.000836411 |
| dragon | baseline | 1 | 225 | 0 | 211 | 0.14593 | 0.310831 | 1.13113 |
| dragon | baseline | 2 | 119 | 0 | 108 | 0.311115 | 0.587344 | 1.32528 |
| dragon | baseline | 3 | 57 | 0 | 49 | 0.49583 | 1.23571 | 1.33616 |
| dragon | baseline | 4 | 30 | 0 | 26 | 0.727314 | 1.33616 | 1.34527 |
| dragon | baseline | 5 | 14 | 0 | 12 | 1.22074 | 1.33616 | 1.34527 |
| dragon | baseline | 6 | 7 | 0 | 3 | 1.34527 | 1.38236 | 1.38236 |
| dragon | baseline | 7 | 3 | 0 | 1 | 1.34527 | 1.38236 | 1.38236 |
| dragon | baseline | 8 | 2 | 0 | 0 | 1.38236 | 1.38236 | 1.38236 |
| dragon | baseline | 9 | 1 | 0 | 0 | 1.38236 | 1.38236 | 1.38236 |
| dragon | baseline | 10 | 1 | 0 | 0 | 1.38236 | 1.38236 | 1.38236 |
| dragon | baseline | 11 | 1 | 0 | 0 | 1.38236 | 1.38236 | 1.38236 |
| dragon | baseline | 12 | 1 | 0 | 0 | 1.38236 | 1.38236 | 1.38236 |
| dragon | baseline | 13 | 0 | 1 | 0 | 0 | 0 | 0 |
| dragon | candidate | 0 | 421 | 0 | 0 | 0.000297262 | 0.000448512 | 0.000836411 |
| dragon | candidate | 1 | 178 | 0 | 0 | 0.000632439 | 0.000884998 | 0.00704157 |
| dragon | candidate | 2 | 86 | 0 | 0 | 0.00132934 | 0.00288472 | 0.0201112 |
| dragon | candidate | 3 | 41 | 0 | 0 | 0.00386858 | 0.027323 | 0.1423 |
| dragon | candidate | 4 | 20 | 2 | 0 | 0.0316778 | 0.146025 | 0.15175 |
| dragon | candidate | 5 | 6 | 5 | 0 | 0.15175 | 0.340193 | 0.340193 |
| dragon | candidate | 6 | 3 | 1 | 0 | 0.24596 | 0.340193 | 0.340193 |
| dragon | candidate | 7 | 1 | 0 | 0 | 0.363541 | 0.363541 | 0.363541 |
| dragon | candidate | 8 | 1 | 0 | 0 | 0.363541 | 0.363541 | 0.363541 |
| dragon | candidate | 9 | 1 | 0 | 0 | 0.363541 | 0.363541 | 0.363541 |
| dragon | candidate | 10 | 1 | 0 | 0 | 0.363541 | 0.363541 | 0.363541 |
| dragon | candidate | 11 | 1 | 0 | 0 | 0.363541 | 0.363541 | 0.363541 |
| dragon | candidate | 12 | 1 | 0 | 0 | 0.418577 | 0.418577 | 0.418577 |
| dragon | candidate | 13 | 0 | 1 | 0 | 0 | 0 | 0 |

First reported group reaching each policy's largest finite error:

| Mesh | Policy | Group / level | Error / inherited | Original / input triangles | Split child links |
| --- | --- | --- | ---: | ---: | ---: |
| bunny | baseline | 65 / 4 | 1.33374 / 1.13405 | 64005 / 1471 | 14 |
| bunny | candidate | 66 / 9 | 0.371513 / 0.316141 | 69451 / 136 | 0 |
| dead_tree_trunk_02 | baseline | 80 / 4 | 1.18252 / 0.640142 | 83128 / 1800 | 6 |
| dead_tree_trunk_02 | candidate | 83 / 9 | 0.334763 / 0.120047 | 81896 / 158 | 0 |
| namaqualand_boulder_02 | baseline | 96 / 5 | 1.19399 / 0.962413 | 97964 / 1228 | 1 |
| namaqualand_boulder_02 | candidate | 87 / 4 | 0.283415 / 0.0932827 | 7424 / 484 | 0 |
| coast_rocks_01 | baseline | 664 / 5 | 1.10531 / 1.04986 | 196102 / 1897 | 7 |
| coast_rocks_01 | candidate | 599 / 11 | 0.211475 / 0.156346 | 616562 / 323 | 0 |
| dragon | baseline | 872 / 6 | 1.38236 / 1.32528 | 659946 / 1409 | 12 |
| dragon | candidate | 768 / 12 | 0.418577 / 0.363541 | 766224 / 202 | 0 |

Acceptance is not met; retain this candidate as the measured result, not a qualified replacement.

## MDAG 2 source-region repair (builder revision 4)

Before is revision 3 at `1ef3ecf72cdb0fe9f6d5056e76534c0690819568`; after is the repaired v2 path.
Both use identical normalized inputs, normals at weight 0.5, grid -22, and 128 triangles per cluster.
Errors now include the sampled symmetric deviation from the original leaf-triangle union, before snapping,
and inherited child errors. Shared refined groups are visited once per region. The previous step floor
remains an additional bound. This does not certify continuous Hausdorff distance.

Cut evaluation: 64 directions, six spans, 200,000 source samples. Ratios below are in scene units
over views at threshold 1 px; median and p95 use the harness order statistics. All 30 cut self-checks passed.

| Mesh | Before median / p95 | After median / p95 | Before / after mean cut triangles |
| --- | ---: | ---: | ---: |
| bunny | 1.055 / 1.140 | 1.026 / 1.036 | 22381 / 40166 |
| dead_tree_trunk_02 | 1.033 / 1.180 | 1.000 / 1.061 | 34241 / 53880 |
| namaqualand_boulder_02 | 0.919 / 1.000 | 1.000 / 1.000 | 48675 / 66314 |
| coast_rocks_01 | 1.039 / 1.233 | 1.000 / 1.000 | 66636 / 337586 |
| dragon | 1.222 / 1.728 | 1.000 / 1.000 | 80162 / 435717 |

P95 per-view maximum deviation in pixels, thresholds 0.5 / 1 / 2 / 4 px:

| Mesh | Before | After |
| --- | ---: | ---: |
| bunny | 0.723 / 2.511 / 4.500 / 8.104 | 0.723 / 2.511 / 4.500 / 4.500 |
| dead_tree_trunk_02 | 0.631 / 1.596 / 5.407 / 10.722 | 0.631 / 1.596 / 5.407 / 6.723 |
| namaqualand_boulder_02 | 0.534 / 1.188 / 2.859 / 9.623 | 0.534 / 1.188 / 2.859 / 9.623 |
| coast_rocks_01 | 1.459 / 2.909 / 5.298 / 10.224 | 1.459 / 2.088 / 2.088 / 2.088 |
| dragon | 1.671 / 3.286 / 8.377 / 12.704 | 1.285 / 1.880 / 1.880 / 1.880 |

The repaired rocks and dragon deviation curves plateau at coarser thresholds.
The higher selected-triangle counts above record the geometry retention cost.

Build plus write milliseconds: median / p95 (nearest rank, maximum of five). Five interleaved repeats,
alternating policy order, pinned to core 2 for 1T and cores 2–9 for 8T on a shared machine.
The added source-region pass is serial and retains workspace accounting. Cost ratios use these paired runs;
historical revision-3 timings below came from a different session.

| Mesh | 1T before | 1T after | Cost | 8T before | 8T after | Cost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 1145.1 / 1262.7 | 5417.1 / 5865.5 | 4.73x | 472.5 / 498.4 | 4473.0 / 4956.2 | 9.47x |
| dead_tree_trunk_02 | 1598.7 / 1983.8 | 7009.2 / 9214.7 | 4.38x | 551.0 / 638.8 | 5773.0 / 7806.0 | 10.48x |
| namaqualand_boulder_02 | 1638.7 / 1696.6 | 7670.0 / 8070.1 | 4.68x | 524.1 / 566.1 | 6510.6 / 6713.1 | 12.42x |
| coast_rocks_01 | 9903.4 / 10221.6 | 76837.2 / 81096.6 | 7.76x | 3073.3 / 3168.1 | 68553.2 / 70858.9 | 22.31x |
| dragon | 12778.0 / 13013.3 | 135646.0 / 136033.4 | 10.62x | 3978.4 / 4364.8 | 125110.1 / 131801.9 | 31.45x |

The old step-only floor is insufficient independently of sampling: A contains unit XY triangles at
z=0,1,2; B retains z=1,2; C retains z=2. Step deviations are 1 and 1, while original-to-final
deviation is 2. The regression verifies that the recorded final replacement error is 2.

Half attribute decoding now expands subnormals exactly, including both signs and signed zero.
The standalone legacy half helper retains its upstream flush behavior. WGSL is validated with naga.

## MDAG 2 measured-policy rerun

Both paths snap before building, use normals at weight 0.5, and write MDAG 2
with Oct16x2 normals and UnormRange16x2 UVs. After uses the library v2 builder;
before uses the unchanged upstream builder. Grid exponent -22, extent 2 m,
128 triangles/cluster. Cut rows: 64 directions, six spans, 200k source samples;
measured/claimed is the median over views at the 1 px threshold in scene units.

| Mesh | Before median | After median | Before cut triangles | After cut triangles |
| --- | ---: | ---: | ---: | ---: |
| bunny | 1.128 | 1.055 | 21618 | 22381 |
| dead_tree_trunk_02 | 1.094 | 1.033 | 33437 | 34241 |
| namaqualand_boulder_02 | 0.956 | 0.919 | 47731 | 48675 |
| coast_rocks_01 | 1.124 | 1.039 | 62004 | 66636 |
| dragon | 4.588 | 1.222 | 62840 | 80162 |

Dragon p95 maximum deviation (px), thresholds 0.5 / 1 / 2 / 4 px:

- baseline: 4.654 / 13.493 / 37.688 / 41.478
- v2: 1.671 / 3.286 / 8.377 / 12.704

Build plus write milliseconds, median / p95 / max-minus-min of five repeats.
Pinned to core 2 (1T), cores 2-9 (8T), alternating policy order, shared machine.

| Mesh | Policy | Threads | ms median / p95 / noise | Peak RSS MiB | File bytes |
| --- | --- | ---: | ---: | ---: | ---: |
| bunny | ours | 1 | 257.4 / 292.6 / 52.4 | 22.4 | 1794847 |
| bunny | ours | 8 | 198.6 / 270.3 / 99.9 | 25.1 | 1794847 |
| bunny | ours-v2 | 1 | 1441.8 / 1497.3 / 190.6 | 24.4 | 1794847 |
| bunny | ours-v2 | 8 | 529.5 / 616.1 / 125.0 | 28.8 | 1794847 |
| dead_tree_trunk_02 | ours | 1 | 497.5 / 703.3 / 314.2 | 28.0 | 2199288 |
| dead_tree_trunk_02 | ours | 8 | 360.9 / 520.4 / 230.3 | 31.1 | 2199288 |
| dead_tree_trunk_02 | ours-v2 | 1 | 3578.2 / 3879.1 / 1315.0 | 28.5 | 2199288 |
| dead_tree_trunk_02 | ours-v2 | 8 | 961.8 / 1657.0 / 850.0 | 33.0 | 2199288 |
| namaqualand_boulder_02 | ours | 1 | 311.4 / 465.3 / 160.6 | 32.9 | 2587962 |
| namaqualand_boulder_02 | ours | 8 | 232.6 / 375.8 / 162.3 | 35.7 | 2587962 |
| namaqualand_boulder_02 | ours-v2 | 1 | 1895.5 / 2606.5 / 751.6 | 31.4 | 2587962 |
| namaqualand_boulder_02 | ours-v2 | 8 | 649.1 / 892.2 / 295.5 | 36.6 | 2587962 |
| coast_rocks_01 | ours | 1 | 2420.8 / 2769.1 / 542.5 | 183.4 | 16713283 |
| coast_rocks_01 | ours | 8 | 1649.4 / 1793.8 / 238.3 | 192.0 | 16713283 |
| coast_rocks_01 | ours-v2 | 1 | 12811.1 / 13728.6 / 1426.8 | 165.3 | 16713283 |
| coast_rocks_01 | ours-v2 | 8 | 3793.7 / 4236.8 / 548.9 | 180.8 | 16713283 |
| dragon | ours | 1 | 2788.1 / 3017.5 / 417.2 | 214.5 | 21558708 |
| dragon | ours | 8 | 1899.8 / 1970.1 / 185.3 | 218.8 | 21558708 |
| dragon | ours-v2 | 1 | 15321.3 / 15598.5 / 1424.3 | 214.2 | 21558708 |
| dragon | ours-v2 | 8 | 4304.8 / 4499.5 / 567.6 | 224.3 | 21558708 |

Size rows include normal and UV payloads. Resident bytes mean resident prefix plus all memory pages.

| Mesh | Policy | DAG triangles | File B/triangle | Resident B/triangle | Pages | Pinned bytes |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| bunny | baseline | 138744 | 12.936 | 14.712 | 18 | 108284 |
| bunny | v2 | 138744 | 12.936 | 14.712 | 18 | 108284 |
| dead_tree_trunk_02 | baseline | 166072 | 13.243 | 14.915 | 21 | 125332 |
| dead_tree_trunk_02 | v2 | 166072 | 13.243 | 14.915 | 21 | 125332 |
| namaqualand_boulder_02 | baseline | 195705 | 13.224 | 14.962 | 25 | 105756 |
| namaqualand_boulder_02 | v2 | 195705 | 13.224 | 14.962 | 25 | 105756 |
| coast_rocks_01 | baseline | 1359178 | 12.297 | 13.887 | 158 | 120008 |
| coast_rocks_01 | v2 | 1359178 | 12.297 | 13.887 | 158 | 120008 |
| dragon | baseline | 1741770 | 12.377 | 14.138 | 208 | 127504 |
| dragon | v2 | 1741770 | 12.377 | 14.138 | 208 | 127504 |

Multi-material asset receipt:

```json
{"kind":"material_roundtrip","source_triangles":94722,"material_count":6,"vertices_checked":138106,"max_uv_error":0.0000075101852,"max_normal_error":0.000061737235,"stuck_groups":0,"material_stuck_groups":0,"writer_stuck_groups":0,"file_bytes":2623515}
```

Acceptance assertions: passed. These are sampled CPU quality measurements; no GPU timing or continuous error certificate is claimed.


## Validation and provenance

Exit 0: formatting; all-target/all-feature Clippy with warnings denied; release tests;
all-feature release tests; public-text check; fuzz-target build. No fuzz campaign ran.
The focused tests cover the measured floor, actual multi-group parallel preparation,
protected slots with and without sloppy fallback, all attribute bounds, malformed
chunks, page round trips and naga validation with no optional capabilities.

The 100 timing receipts and ten cut evaluations completed. Postprocessing and the
real-asset round trip ran separately after a driver-header metadata edit disrupted
the active shell's file offset. The original driver hash is reproduced by removing
only its two metadata lines. All production Rust, benchmark Rust, WGSL and manifest
hashes match the measurement-start snapshot. A strengthened validation fixture is
recorded as the other source difference and passed the current release matrix and
final Clippy check. The retained receipts are external to this public tree.

## Historical MDAG 1 evidence

The following tables and proposed opt-in describe the original diagnostic snapshot.
The current v2 policy and acceptance results are above.

# Results: MDAG 1 (current crate) against Bevy meshlets

First table, 2026-11-04. Crate at `cf119a96` (0.2.17 plus the 0.2.18 impostor work), MDAG 1 container,
`clusterlod::build_parallel_with_output` plus `build_hierarchy` (width 8, 128 triangles per cluster).
Reference: `MeshletMesh::from_mesh` at Bevy commit `8cc060f01c3d4d5920781124cc8c32013bb47730`,
quantisation factor 4, saved with its own asset saver. Method and definitions: `README.md`.
Regenerate the tables with `python3 -I parity/vg-bench/report.py --summary <work>/*.jsonl`.

Host: AMD Ryzen 9 7945HX (16 cores, 32 threads), performance governor, shared with other jobs while
the runs were admitted through the machine queue. Single-thread rows are pinned to one core, the eight-thread
rows to cores 2-9 (four physical cores with SMT siblings). Five interleaved repeats per cell; the
parenthesised value is max minus min. Noise is large (up to 40% of the median on the small meshes, and
the largest rows moved by more than a second), so ratios below are indicative; the ordering is not
in doubt for build time, memory and file size.

Corpus: the public set in `CORPUS.md`, each fitted to 2 m (longest side), normals and box-projected UVs
derived where absent. Five of the seven corpus meshes are in the table; Lucy (28M triangles) is fetched and
hash-checked but not built: the reference needs roughly 0.8 KiB of resident memory per source triangle,
about 22 GiB for Lucy, above the lane's memory budget. Reversal cost: free (run `bench.sh` on it with
`VG_REPEATS=1`, more memory, and the `--no-cuts` evaluation). The Khronos multi-material asset is fetched
for the attribute and material lane and not used here. Dragon is the `dragon_vrip` reconstruction.

## Build time and memory

Build plus serialise (ours: DAG, hierarchy and MDAG 1 write; reference: `from_mesh` and save), median
of five, milliseconds. RSS is the whole-process peak from `/usr/bin/time -v` (input load included).

| Mesh | Triangles | ours 1T | bevy 1T | ours/bevy | ours 8T | bevy 8T | ours/bevy | ours RSS MiB (8T) | bevy RSS MiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 69,451 | 118 (+-53) | 516 (+-95) | 0.23 | 77 (+-9) | 511 (+-52) | 0.15 | 31 | 55 |
| dead_tree_trunk_02 | 83,128 | 143 (+-25) | 659 (+-46) | 0.22 | 92 (+-13) | 632 (+-34) | 0.14 | 35 | 74 |
| namaqualand_boulder_02 | 97,964 | 230 (+-60) | 993 (+-219) | 0.23 | 112 (+-19) | 783 (+-22) | 0.14 | 38 | 79 |
| coast_rocks_01 | 679,936 | 1,410 (+-448) | 9,464 (+-1,984) | 0.15 | 735 (+-205) | 7,571 (+-2,461) | 0.10 | 151 | 477 |
| dragon | 871,414 | 1,643 (+-236) | 11,661 (+-1,086) | 0.14 | 1,012 (+-147) | 11,344 (+-393) | 0.09 | 198 | 555 |

The reference gains little from eight workers (its simplification is parallel per group, the rest serial);
ours gains 1.5x to 2x because the MDAG 1 write is serial and counted. Build alone (without the write) is
roughly 0.12 to 0.19 of the reference at one thread and 0.07 to 0.11 at eight.

## Size and shape

Bytes per DAG triangle use the triangles over all levels (about twice the source count in both).
Ours resident is MDAG 1 tables plus decoded f32 geometry with source IDs, which is the number MDAG 2 is meant to
reduce; the reference stores quantised, bit-packed positions.

| Mesh | file B/tri ours | bevy | file zstd-19 B/tri ours | bevy | resident B/tri ours | bevy | total/leaf ours | bevy | tris/cluster mean ours | bevy | p5 ours | bevy | hierarchy depth ours | bevy |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 10.6 | 13.0 | 8.4 | 8.9 | 23.1 | 13.8 | 2.00 | 2.00 | 122.0 | 119.1 | 67 | 102 | 3 | 5 |
| dead_tree_trunk_02 | 10.8 | 13.8 | 8.7 | 9.1 | 23.7 | 14.5 | 2.00 | 1.99 | 119.6 | 119.4 | 53 | 101 | 3 | 5 |
| namaqualand_boulder_02 | 10.8 | 13.9 | 8.7 | 9.3 | 23.6 | 14.7 | 2.00 | 1.99 | 120.8 | 119.9 | 58 | 101 | 3 | 5 |
| coast_rocks_01 | 10.6 | 12.5 | 8.1 | 7.9 | 23.2 | 13.2 | 2.00 | 2.00 | 121.9 | 118.9 | 64 | 101 | 4 | 6 |
| dragon | 10.1 | 11.7 | 7.5 | 7.9 | 23.1 | 13.3 | 2.00 | 2.00 | 121.8 | 118.9 | 64 | 101 | 4 | 6 |

Pages (ours only): 11 to 128 pages of 128 KiB per mesh, mean fill 0.925 to 0.994, one terminal group per
mesh (bunny: 11 pages, fill 0.967; dragon: 128 pages, fill 0.994). Page decode: 660 to 710 MB/s of decoded
geometry on one core (median page; bunny's earlier smoke run read 603, the full run 680). The reference's whole-asset LZ4 decode runs at 2.8 to 5.6 GB/s of decompressed bytes
(bunny, boulder). Disk: ours is smaller as a file, nearly equal after `zstd -19` (7.5 to 9.3 B per DAG
triangle for both), so the file-size gap is the entropy coder, not the layout. Resident is 1.6x to 1.8x the
reference. MDAG 2 targets are not measured here.

## Cut efficiency and traversal

64 Fibonacci directions x spans 64, 128, 256, 512, 1024, 2048 px x thresholds 0.5, 1, 2, 4 px (viewport 2048 px,
60 degrees); deviation from `measure_lod_quality` converted at the nearest depth, so it is a worst case.
"tris ours/bevy at p95 dev" is the mean cut triangle count of ours divided by the reference's at equal p95
of the per-view maximum deviation, log-log interpolated along the four thresholds (n/a when the curves do
not overlap); below 1 favours ours. Traversal columns are the 1 px rows, averaged over all views, with
frustum culling and no occlusion culling.

Second table, 2026-11-04, after the error-claim diagnosis below. Same crate output (the library is
unchanged), same views and artifacts. Ours is now built with the normals as attributes of weight 0.5,
like the reference (`ours`; `ours[normals=0]` is the first table's position-only build), and
`ours[measured=1]` is the proposed opt-in error rule, applied in the harness. Measured/claimed is in
scene units on both sides; "earlier form" is the first table's ratio, which mixed two depths (it
reproduces the first table exactly for the position-only build). The p95 deviation columns are over all
spans at each threshold. Regenerate with `python3 -I parity/vg-bench/report.py --cuts <work>/eval-*.jsonl`.

| Mesh | Builder | tris / bevy at p95 dev 5 px | at 8 px | measured/claimed median | p95 | earlier form (px) median | p95 max dev px at 0.5 / 1 / 2 / 4 px | nodes | clusters tested | drawn |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | ours | 0.60 | 0.81 | 1.15 | 1.62 | 1.34 | 0.9 / 3.0 / 4.7 / 10.8 | 38.1 | 362 | 176 |
| bunny | ours[measured=1] | 0.66 | 0.90 | 1.03 | 1.22 | 1.08 | 0.7 / 2.4 / 4.7 / 10.8 | 44.7 | 380 | 185 |
| bunny | ours[normals=0] | 0.49 | 0.62 | 1.71 | 1.86 | 1.73 | 2.0 / 3.6 / 6.9 / 19.7 | 31.9 | 214 | 105 |
| bunny | bevy | n/a | n/a | 1.16 | 1.77 | 1.31 | 2.8 / 4.6 / 7.4 / 10.6 | 10.7 | 372 | 179 |
| dead_tree_trunk_02 | ours | 0.70 | 0.85 | 1.13 | 1.44 | 1.44 | 0.8 / 2.0 / 6.7 / 12.2 | 56.1 | 573 | 277 |
| dead_tree_trunk_02 | ours[measured=1] | 0.67 | 0.71 | 1.04 | 1.13 | 1.11 | 0.6 / 1.5 / 5.4 / 10.4 | 56.9 | 593 | 286 |
| dead_tree_trunk_02 | ours[normals=0] | 0.71 | 0.83 | 4.00 | 10.63 | 4.59 | 3.7 / 7.6 / 15.8 / 34.2 | 35.4 | 293 | 142 |
| dead_tree_trunk_02 | bevy | n/a | n/a | 1.20 | 1.36 | 1.21 | 3.5 / 3.5 / 7.5 / 11.9 | 14.4 | 590 | 285 |
| namaqualand_boulder_02 | ours | 1.01 | 1.01 | 1.17 | 1.51 | 1.26 | 0.8 / 1.7 / 4.0 / 13.4 | 69.4 | 799 | 386 |
| namaqualand_boulder_02 | ours[measured=1] | 0.90 | 0.91 | 1.00 | 1.07 | 1.06 | 0.5 / 1.2 / 2.9 / 9.6 | 74.0 | 814 | 394 |
| namaqualand_boulder_02 | ours[normals=0] | 1.16 | 1.16 | 4.38 | 5.83 | 4.34 | 2.7 / 6.5 / 21.9 / 25.8 | 46.9 | 499 | 243 |
| namaqualand_boulder_02 | bevy | n/a | n/a | 1.10 | 1.38 | 1.14 | 3.4 / 3.4 / 3.8 / 12.7 | 18.0 | 821 | 395 |
| coast_rocks_01 | ours | 0.80 | 0.91 | 1.22 | 1.42 | 1.27 | 2.1 / 3.4 / 6.1 / 10.0 | 121.3 | 1,069 | 530 |
| coast_rocks_01 | ours[measured=1] | 0.66 | 0.74 | 1.01 | 1.21 | 1.27 | 1.5 / 2.9 / 5.0 / 8.6 | 136.9 | 1,148 | 568 |
| coast_rocks_01 | ours[normals=0] | 0.59 | 0.43 | 2.01 | 2.50 | 2.12 | 3.1 / 5.9 / 8.0 / 14.6 | 50.2 | 371 | 185 |
| coast_rocks_01 | bevy | n/a | n/a | 1.17 | 1.54 | 1.28 | 3.1 / 4.2 / 6.8 / 11.0 | 31.5 | 1,123 | 557 |
| dragon | ours | n/a | n/a | 4.73 | 14.24 | 5.67 | 11.1 / 37.2 / 37.7 / 45.9 | 115.2 | 1,088 | 544 |
| dragon | ours[measured=1] | 0.81 | 0.92 | 1.16 | 1.40 | 1.39 | 1.7 / 3.7 / 6.2 / 9.1 | 209.2 | 1,466 | 732 |
| dragon | ours[normals=0] | n/a | n/a | 14.78 | 29.23 | 13.91 | 60.7 / 65.4 / 65.4 / 65.4 | 58.8 | 466 | 233 |
| dragon | bevy | n/a | n/a | 1.44 | 3.02 | 1.75 | 3.7 / 5.7 / 14.2 / 14.2 | 36.0 | 1,146 | 574 |

Self-checks: 60 run, 0 failed. The build-time, memory and size tables above are from the first run, with
the position-only build; simplifying with normals adds attribute work and changes the DAG, and those
tables were not rerun (reversal: run `bench.sh`, which now builds with normals).

First table (position-only build, ratio with mixed depths):

| Mesh | tris ours/bevy at p95 dev 5 px | at 8 px | measured/claimed median ours | bevy | nodes ours | bevy | clusters tested ours | bevy | drawn ours | bevy | quantisation floor p95 px (bevy, source cut) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 0.49 | 0.62 | 1.73 | 1.31 | 31.9 | 10.7 | 214 | 372 | 105 | 179 | 2.76 |
| dead_tree_trunk_02 | 0.71 | 0.83 | 4.59 | 1.21 | 35.4 | 14.4 | 293 | 590 | 142 | 285 | 3.52 |
| namaqualand_boulder_02 | 1.16 | 1.16 | 4.34 | 1.14 | 46.9 | 18.0 | 499 | 821 | 243 | 395 | 3.42 |
| coast_rocks_01 | 0.59 | 0.43 | 2.12 | 1.28 | 50.2 | 31.5 | 371 | 1,123 | 185 | 557 | 2.86 |
| dragon | n/a | n/a | 13.91 | 1.75 | 58.8 | 36.0 | 466 | 1,146 | 233 | 574 | 3.40 |

Self-checks: 30 run, 0 failed (threshold 0 selects exactly the source triangles and only leaves; a huge
threshold selects only the root set; the hierarchical traversal equals the flat cut at 1 px; per builder
per mesh). They guard the port of the reference cut rule and our selector.

## Error-claim diagnosis

The first table's measured/claimed ratio (1.7 to 4.6, 13.9 on dragon) had three causes, two in
this harness and one in the library's default output, which is upstream behaviour.
`vg-bench diag` (see `README.md`) measured them without the pixel conversion: world-space cuts
at `t = radius x 2^-k`, k = 3..13, each measured one-sided in both directions against its largest
claimed error, and each group's single simplification step (its clusters against the clusters
simplified from them) against the recorded error.

1. **Harness input: no attributes.** Ours was built from positions only; the reference, and the
   upstream demo (`demo/nanite.cpp`), simplify with the normals as attributes of weight 0.5. The
   simplifier's position-only error under-reports the sampled distance of one step by a median of
   1.48 (bunny), 1.48 (dragon), 1.65 (trunk), 1.72 (boulder) and 1.51 (rocks), in both directions
   about equally. With normals at 0.5 the attribute error, clamped to the position scale, raises
   the claim and the per-step median becomes 0.94 to 1.00 (trunk, dragon). Fix: the harness builds
   ours with normals at 0.5 (`ours::NORMAL_WEIGHT`), so both builders see the same input.
2. **Harness ratio: two depths.** The ratio divided the deviation converted at the mesh's nearest
   depth by the claim projected at each drawn cluster's own depth, so a far cluster's claim was
   compared with a near conversion. Fix: both sides in scene units (largest self error among drawn
   clusters against the measured world maximum); the earlier form stays in the JSON as
   `median_measured_over_claimed_px`.
3. **Library: removed geometry priced at its own planes.** On dragon, with normals, the per-step
   median is 1.00 but the tail is long (p95 1.31, maximum 26.8): the worst groups lose geometry
   (source-to-level 23 mm against an error of 0.85 mm, level-to-source 0.9 mm). The quadric error of
   a collapse is the distance to the collapsed vertices' own planes, so a small or thin part that
   folds into the surface costs almost nothing while its points end far from what remains.
   Dragon (`dragon_vrip`) has 151 index-connected parts, 146 under 16 triangles, and the default
   cuts stop referencing 107 of them at `t = radius/256`. World-space cuts of the default build
   measure a source-to-level maximum of 3.1 times the claim (median over the eleven cuts, up to 16)
   while level-to-source stays at 1.14. `simplify_permissive` (on by default, as upstream) makes it
   worse: without it the median is 1.25, up to 3.0.

Not the cause: the sloppy fallback (`fallback_sloppy=0` gives identical cuts on bunny and dragon),
the error accumulation rule (`max(previous, current)`, the same as upstream `clusterlod.h` and as
the reference, which forces each group error to at least its meshlets' errors), and bound spheres
(the world-space cuts project every sphere at the same depth and still show cause 3).

The library's default output is unchanged. Its configuration (`default_config`) and error rule
are those of upstream `clodDefaultConfig` and `clodBuild`, and `parity/cluster-lod/differential.py`
compares the complete cluster-LOD result against the vendored C++ demo, so upstream has the same
claim. Changing the default would change output against upstream; the contract documents the
error as the simplification error, which it is.

**Proposed opt-in, prototyped in the harness (`--set measured=1`):** after simplifying a group,
measure the step with the crate's own `measure_lod_quality` (two-sided, about 4000 samples per
group) and take the group error as at least that distance, then restore monotonicity over the
DAG. In the library this would be a `Config` field (for example `simplify_error_measured`, default
off); `Config` is not `#[non_exhaustive]`, so adding a field breaks struct-literal construction
and belongs in a minor release (0.3) rather than a 0.2.x patch. A narrower rule that only priced
fully removed index-connected parts (`--set removed=1`) raised 6 to 11 groups on dragon and left
the median at 1.57: most of the lost geometry is not a whole disconnected part.

World-space cuts, largest sampled deviation over the largest claim (median and maximum over the
cuts where the claim is positive; triangles of the k = 8 cut, `t = radius/256`):

| Mesh | ours, positions only | ours, normals 0.5 (new default) | ours, normals 0.5 + measured | reference | k = 8 triangles: normals 0.5 / + measured / reference |
| --- | ---: | ---: | ---: | ---: | ---: |
| bunny | 1.71 (7.00) | 1.14 (1.31) | 1.07 (1.22) | 1.44 (2.53) | 16,558 / 18,445 / 16,918 |
| dead_tree_trunk_02 | 4.60 (7.48) | 1.29 (2.14) | 1.08 (1.11) | 1.02 (1.35) | 23,448 / 26,256 / 28,936 |
| namaqualand_boulder_02 | 4.38 (5.83) | 1.16 (1.51) | 0.99 (1.36) | 0.94 (1.38) | 50,160 / 53,371 / 51,092 |
| coast_rocks_01 | 1.98 (2.55) | 1.03 (1.44) | 0.98 (1.22) | 1.35 (10.09) | 34,625 / 36,764 / 35,150 |
| dragon | 14.78 (127.48) | 3.11 (15.95) | 1.10 (1.34) | 1.50 (11.61) | 33,400 / 75,496 / 43,050 |

The reference's ratio grows at the finest cuts because its measured deviation includes its
1/16 cm position quantisation and its claim does not. The measured rule costs dragon 2.3 times
the triangles at `t = radius/256` (1.09 times at `radius/2048`) and the other meshes 1.06 to 1.12
times: that is the geometry the default claim hides.

## Against the section 6 bar

- Build time at most half of the reference at equal thread count: met on every mesh at 1 and 8 threads
  (0.09 to 0.23 including the MDAG 1 write).
- Memory bytes per DAG triangle at or below the reference: not met. Ours is 23 resident B per DAG triangle against
  13 to 15. File bytes per DAG triangle are below the reference (10 to 11 against 12 to 14).
- Fewer cut triangles at equal measured p95 deviation (second table, both builders simplifying with normals):
  met on bunny, trunk, rocks (0.60 to 0.91 at 5 and 8 px), even on the boulder (1.01). Not decidable on dragon
  with the default rule (the curves do not overlap); with the measured rule 0.81 and 0.92.
- Measured over claimed error (second table, scene units): ours 1.13 to 1.22 on bunny, trunk, boulder and
  rocks against 1.10 to 1.20 for the reference; dragon 4.73 against 1.44, because the default (upstream)
  error does not price removed thin or small geometry. The measured rule gives 1.00 to 1.16 on all five.
  The first table's 1.7 to 4.6 came mostly from the harness (position-only input, mixed depths).

## Open findings

- Dragon: diagnosed in "Error-claim diagnosis". The flat 65 px p95 deviation was geometry removed at a near-zero
  quadric error (source-to-level only), upstream behaviour made worse by `simplify_permissive`. With the default
  rule the p95 deviation is still flat between 1 and 2 px (37.2, 37.7); with the measured rule it rises with the
  threshold (1.7, 3.7, 6.2, 9.1 px). The library opt-in (`Config` field, 0.3) is proposed, not implemented.
  The self-checks still do not prove the cut covers the surface exactly once.
- The reference's position quantisation (1/16 cm, factor 4) sets a floor of 2.8 to 3.5 px at 2048 px span on
  these 2 m meshes (last column). Comparing deviations below that floor favours ours by construction;
  the equal-deviation rows at 5 and 8 px sit above it.
- Not measured: pinned bytes and MDAG 2 pages, wasm32 page decode (needs the MDAG 2 transcoder and wasmtime),
  the Khronos multi-material mesh, Lucy, alpha coverage, GPU runtime timing (the engine's gate).

## L5 foliage: opt-in alpha guard

L5 working tree based on `be4d4e0f558bf0af91d86ff19d7636b1aadfadbf`.
CC0 Poly Haven `fern_02`, first primitive: 1,660 vertices, 2,384 triangles,
normalized to extent 2, original 1k Alpha map, nearest repeat sampling, cutoff
0.5, double sided. Grid -16, 32 triangles per cluster, partition size 2, UV
weights zero. Guard defaults: nine fixed sphere-grid views at 32 pixels per
side, 2x2 samples, ratio [0.93, 1.07], blurred difference <= 0.12.

Cuts use a +Z perspective camera, vertical FOV 1 radian, viewport 1080, near
0.01 and threshold 1 px. Both builds are written and every MDAG 2 page is read
back; cut spheres/errors come from those files, including monotonic repair and
the quantisation error floor. Coverage uses nine fixed orthographic captures
of the same cut, at resolution derived from its apparent perspective sphere
diameter (clamped to 8..512). Reference and cut share the source framing;
the reference is snapped source geometry. Ratios below are extrema across the
nine views, and blurred differences are the maximum view mean absolute 3x3
box-blurred alpha difference. This is a CPU proxy, not a runtime filtering gate.

| Distance | Raster side | Coverage off | Coverage on | Triangles off / on | Blur off / on |
| ---: | ---: | --- | --- | ---: | --- |
| 4 | 512 | 1.000000..1.000000 | 1.000000..1.000000 | 2384 / 2384 | 0 / 0 |
| 8 | 328 | 0.999737..1.000065 | 0.999737..1.000065 | 2335 / 2335 | 0.000062 / 0.000062 |
| 16 | 164 | 0.997321..1.003597 | 0.997072..1.001202 | 1894 / 2015 | 0.002042 / 0.001830 |
| 32 | 82 | 0.958935..1.004249 | 0.978378..1.001583 | 749 / 1446 | 0.010802 / 0.006432 |
| 64 | 41 | 0.958500..1.017033 | 0.972357..1.005382 | 345 / 1344 | 0.015994 / 0.007953 |

All on rows satisfy the default ratio and blurred-difference limits; these off
rows also happen to pass the global band. The guard improves the far silhouette
and blurred difference, at a triangle cost. At distance 64 that cost is +999
triangles (+289.6%); at 32 it is +697 (+93.1%); at 16 it is +121 (+6.4%).
There are 28 alpha-stuck groups, zero error raises, and 28 total stuck groups
with the guard on, versus one ordinary stuck group off. Total DAG triangles
fall from 4,668 to 3,434 because rejected coarse levels are not emitted; this
storage reduction does not imply a reduction in selected triangles.

Coarsest fallback report, sampled against snapped source geometry with density
1 interior sample per square mesh unit plus corners:

| Policy | Fallback triangles | Sampled symmetric max | Sampled symmetric RMS |
| --- | ---: | ---: | ---: |
| off | 29 | 0.59553633 | 0.17145327 |
| on | 1297 | 0.08589226 | 0.00697120 |

These distances are lower estimates of Hausdorff distance, not ray certificates.
No elapsed-time comparison is claimed. See `l5_inputs.py` and the
`vg_foliage_measure` crate example for reproduction with caller-owned assets.

Input SHA-256 binding:

- glTF: `33974295ee19670af4877705fc7d91f33f42d6302f0d44a625e072271223031f`
- binary buffer: `fe1abb762bcbbc7ad9e86925c8080437a62add257d9280e9142fdc85c226e64c`
- Alpha PNG: `ea3c64284c7ebdeedf484d2d816eae92825ec526140c3bb83859ddc7a64001f9`
- decoded L8 plane: `0d8db8510f400bad58cc7011a80ae01fdef864eb01242f6c3d1513f0255330b9`

Disabled-path identity was checked against a read-only archive of the starting
revision, with position-only and UV-input non-alpha fixtures. Complete MDAG files
compare byte-for-byte equal: 6,239 bytes in each case, SHA-256
`70c74d49fc2f8d5fcd9d5246e977b8373889f29e4f5d8b8c08892c377c9a31b0`.
Build keys also match: `96893b40f5e65797` (position-only) and
`69cf6d2d4d2f7202` (UV input). The focused tests additionally compare disabled
coverage with enabled-but-unmasked and fully transparent inputs.
