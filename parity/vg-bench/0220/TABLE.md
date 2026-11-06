## 0.2.20 weighted replacement regions

Source base `5dbdc379581b6153e5a2b9e2fc61755778518366`, builder revision 6.
Container major/minor and all chunk layouts are unchanged. Cook keys change.
Complete raw rows, source/corpus/binary/lock hashes, diagnostic distributions and
per-mesh pass/fail booleans are retained in [summary](summary.json).

Five counterbalanced interleaved repeats of baseline, candidate and pinned reference,
build plus serialise time, 1T on CPU 0 and 8T on CPUs 0–7. Shared host. The reference is Bevy MeshletMesh at `8cc060f01c3d4d5920781124cc8c32013bb47730`;
its cached source is verified unchanged. Quality uses 64 directions, six spans,
200,000 source samples and the unchanged normal weight, grid and thresholds.
Cut ratios use log-log interpolation at equal measured p95 deviation.

| Mesh | Build/reference 1T | 8T | Resident B/DAG tri candidate / reference | File B/DAG tri candidate / reference | Cut/reference tris 5 / 8 px | Measured/claimed median / p95 (1 px) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 2.119 | 0.722 | 14.918 / 13.782 | 13.130 / 13.011 | 0.721 / 0.794 | 1.000 / 1.041 |
| dead_tree_trunk_02 | 1.872 | 0.680 | 15.037 / 14.522 | 13.357 / 13.837 | 0.709 / 0.781 | 0.967 / 1.061 |
| namaqualand_boulder_02 | 1.838 | 0.624 | 15.146 / 14.671 | 13.396 / 13.928 | 0.929 / 0.943 | 0.970 / 1.000 |
| coast_rocks_01 | 1.330 | 0.457 | 14.177 / 13.233 | 12.567 / 12.487 | 1.093 / 1.675 | 0.866 / 1.000 |
| dragon | 1.372 | 0.459 | 14.411 / 13.322 | 12.623 / 11.684 | 0.761 / 1.072 | 0.976 / 1.000 |

Bar passes out of five (all meshes must pass):

| Bar | Passes | Verdict |
| --- | ---: | --- |
| file_per_triangle | 2/5 | FAIL |
| build_1T | 0/5 | FAIL |
| build_8T | 2/5 | FAIL |
| resident | 0/5 | FAIL |
| cut_5px | 4/5 | FAIL |
| cut_8px | 3/5 | FAIL |
| error_median | 5/5 | PASS |
| error_p95 | 5/5 | PASS |
| self_checks | 5/5 | PASS |
| thread_identity | 5/5 | PASS |

Fresh timing controls (milliseconds, median / maximum / max-minus-min over five;
maximum is the nearest-rank p95). The last column is before → after divided by
the contemporaneous reference, rather than comparing different host sessions.

| Mesh | Threads | Baseline | Candidate | Reference | Ratio before → after |
| --- | ---: | ---: | ---: | ---: | ---: |
| bunny | 1 | 982.3 / 1135.9 / 187.0 | 971.4 / 1011.9 / 59.2 | 458.5 / 472.0 / 17.4 | 2.143 → 2.119 |
| bunny | 8 | 380.8 / 407.3 / 41.2 | 330.5 / 342.2 / 25.6 | 457.8 / 476.5 / 30.2 | 0.832 → 0.722 |
| dead_tree_trunk_02 | 1 | 1137.7 / 1222.4 / 97.4 | 1174.9 / 1208.3 / 76.8 | 627.5 / 697.1 / 77.5 | 1.813 → 1.872 |
| dead_tree_trunk_02 | 8 | 478.6 / 489.6 / 52.1 | 425.5 / 437.6 / 26.2 | 625.4 / 633.2 / 16.7 | 0.765 → 0.680 |
| namaqualand_boulder_02 | 1 | 1305.7 / 1362.8 / 73.8 | 1391.2 / 1551.9 / 235.7 | 757.1 / 907.1 / 169.7 | 1.725 → 1.838 |
| namaqualand_boulder_02 | 8 | 511.6 / 528.1 / 43.6 | 485.6 / 508.1 / 52.8 | 778.7 / 821.3 / 73.5 | 0.657 → 0.624 |
| coast_rocks_01 | 1 | 10119.8 / 10364.1 / 572.6 | 9367.7 / 9690.6 / 605.1 | 7041.3 / 7590.4 / 601.4 | 1.437 → 1.330 |
| coast_rocks_01 | 8 | 4011.8 / 4126.4 / 163.3 | 3155.9 / 3271.5 / 322.0 | 6910.6 / 7313.0 / 522.3 | 0.581 → 0.457 |
| dragon | 1 | 15096.6 / 16337.1 / 2589.3 | 13962.1 / 14314.2 / 1429.2 | 10175.9 / 11410.2 / 1717.9 | 1.484 → 1.372 |
| dragon | 8 | 5461.8 / 5808.4 / 558.9 | 4674.5 / 5373.9 / 916.2 | 10179.0 / 10699.2 / 1064.8 | 0.537 → 0.459 |

Resident accounting in bytes per total DAG triangle. Attribute bytes include
per-cluster declarations/ranges and vertex payload; descriptors are 64 bytes each.
The two-channel attribute header is another 80 bytes per cluster. Position grid -22 and the normal/UV encodings are unchanged.

| Mesh | Resident metadata | Descriptors | Positions | Triangle indices | Attributes | Page headers | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 0.222 | 0.524 | 5.100 | 3.001 | 6.068 | 0.0021 | 14.918 |
| dead_tree_trunk_02 | 0.228 | 0.540 | 4.931 | 3.002 | 6.334 | 0.0021 | 15.037 |
| namaqualand_boulder_02 | 0.223 | 0.529 | 5.109 | 3.002 | 6.282 | 0.0021 | 15.146 |
| coast_rocks_01 | 0.217 | 0.526 | 4.303 | 3.001 | 6.127 | 0.0019 | 14.177 |
| dragon | 0.217 | 0.526 | 4.594 | 3.001 | 6.071 | 0.0020 | 14.411 |

