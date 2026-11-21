# Rejected batched-scan and face-witness experiment

**Rejected:** four of five 1T medians regress, and no frozen bar gains a pass. The source is restored. The next floor-based experiment is separate.

Five counterbalanced interleaved repeats at equal threads, pinned to the first
one/eight admitted CPUs. Build plus serialization; ratios below 1 favor the
experiment. Previous means the retained weighted-region candidate at the lane
continuation HEAD. The pinned reference and complete input hashes are unchanged.
All experiment and previous MDAG 2 files match the prior campaign byte for byte
at both thread counts. Shape, size, error and cut rows are therefore reused,
not newly sampled; no geometry or error floor changes.

| Mesh | Build/reference 1T previous → experiment | 8T previous → experiment | Experiment/previous 1T / 8T | Cut ratios 5 / 8 px | File B/tri experiment / reference | Resident B/tri experiment / reference |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 1.964 → 2.008 | 0.667 → 0.649 | 1.022 / 0.972 | 0.721 / 0.794 | 13.130 / 13.011 | 14.918 / 13.782 |
| dead_tree_trunk_02 | 1.699 → 1.723 | 0.664 → 0.658 | 1.014 / 0.992 | 0.709 / 0.781 | 13.357 / 13.837 | 15.037 / 14.522 |
| namaqualand_boulder_02 | 1.665 → 1.640 | 0.532 → 0.520 | 0.985 / 0.977 | 0.929 / 0.943 | 13.396 / 13.928 | 15.146 / 14.671 |
| coast_rocks_01 | 1.148 → 1.157 | 0.448 → 0.422 | 1.008 / 0.943 | 1.093 / 1.675 | 12.567 / 12.487 | 14.177 / 13.233 |
| dragon | 1.115 → 1.148 | 0.458 → 0.444 | 1.030 / 0.969 | 0.761 / 1.072 | 12.623 / 11.684 | 14.411 / 13.322 |

| Experimental bar | Passes / 5 | Verdict |
| --- | ---: | --- |
| file_per_triangle | 2 | FAIL |
| build_1T | 0 | FAIL |
| build_8T | 2 | FAIL |
| resident | 0 | FAIL |
| cut_5px | 4 | FAIL |
| cut_8px | 3 | FAIL |
| error_median | 5 | PASS |
| error_p95 | 5 | PASS |
| self_checks | 5 | PASS |
| thread_identity | 5 | PASS |

Milliseconds: median / nearest-rank p95 (maximum of five) / max-minus-min.
Shared-host variation is retained; there is no quiet-window claim.

| Mesh | Threads | Previous | Experiment | Reference |
| --- | ---: | ---: | ---: | ---: |
| bunny | 1 | 1236.02 / 1414.62 / 196.30 | 1263.66 / 1372.66 / 157.02 | 629.24 / 685.80 / 102.07 |
| bunny | 8 | 431.86 / 453.86 / 45.69 | 419.74 / 446.13 / 54.51 | 647.02 / 690.74 / 105.48 |
| dead_tree_trunk_02 | 1 | 1392.86 / 1427.62 / 77.93 | 1412.87 / 1528.59 / 173.47 | 819.88 / 903.88 / 128.34 |
| dead_tree_trunk_02 | 8 | 539.28 / 556.10 / 52.00 | 534.88 / 551.48 / 53.93 | 812.66 / 855.80 / 51.11 |
| namaqualand_boulder_02 | 1 | 1643.81 / 1773.40 / 233.15 | 1619.30 / 1744.90 / 169.03 | 987.48 / 1072.88 / 202.96 |
| namaqualand_boulder_02 | 8 | 575.45 / 703.30 / 167.86 | 562.22 / 597.36 / 36.91 | 1081.29 / 1120.14 / 218.86 |
| coast_rocks_01 | 1 | 12061.84 / 12534.53 / 790.10 | 12160.79 / 12555.26 / 794.98 | 10510.02 / 10688.87 / 489.47 |
| coast_rocks_01 | 8 | 4316.76 / 4329.83 / 557.06 | 4072.46 / 4242.11 / 283.08 | 9644.32 / 10110.36 / 763.80 |
| dragon | 1 | 16467.46 / 20984.74 / 5022.05 | 16953.76 / 21047.19 / 6378.46 | 14766.18 / 17436.49 / 4349.54 |
| dragon | 8 | 6285.50 / 7076.72 / 2289.39 | 6089.70 / 7205.36 / 2278.16 | 13730.34 / 17105.38 / 5799.47 |
