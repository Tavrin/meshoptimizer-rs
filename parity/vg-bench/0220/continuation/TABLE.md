# Rejected distance-tree leaf experiment

**Rejected.** The retained builder uses its original single-triangle leaves.
The experimental source diff against the continuation HEAD is retained in
[the patch](leaf-experiment.patch); its binary and complete source snapshot remain
in caller-owned evidence. No further policy variant or timing campaign is scheduled.

Five counterbalanced interleaved repeats at equal threads, pinned to the first
one/eight admitted CPUs. Build plus serialization; ratios below 1 favor the
experiment. Previous means the retained weighted-region candidate at the lane
continuation HEAD. The pinned reference and complete input hashes are unchanged.
All experiment and previous MDAG 2 files match the prior campaign byte for byte
at both thread counts. Shape, size, error and cut rows are therefore reused,
not newly sampled; no geometry or error floor changes.

| Mesh | Build/reference 1T previous → experiment | 8T previous → experiment | Experiment/previous 1T / 8T | Cut ratios 5 / 8 px | File B/tri experiment / reference | Resident B/tri experiment / reference |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 2.014 → 2.073 | 0.678 → 0.687 | 1.029 / 1.013 | 0.721 / 0.794 | 13.130 / 13.011 | 14.918 / 13.782 |
| dead_tree_trunk_02 | 1.405 → 1.584 | 0.724 → 0.672 | 1.127 / 0.928 | 0.709 / 0.781 | 13.357 / 13.837 | 15.037 / 14.522 |
| namaqualand_boulder_02 | 1.543 → 1.574 | 0.625 → 0.641 | 1.021 / 1.026 | 0.929 / 0.943 | 13.396 / 13.928 | 15.146 / 14.671 |
| coast_rocks_01 | 1.246 → 1.235 | 0.432 → 0.412 | 0.991 / 0.954 | 1.093 / 1.675 | 12.567 / 12.487 | 14.177 / 13.233 |
| dragon | 1.237 → 1.243 | 0.433 → 0.434 | 1.004 / 1.001 | 0.761 / 1.072 | 12.623 / 11.684 | 14.411 / 13.322 |

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
| bunny | 1 | 1053.33 / 1103.27 / 71.11 | 1084.08 / 1171.29 / 100.47 | 522.96 / 547.85 / 49.09 |
| bunny | 8 | 357.53 / 426.86 / 108.59 | 362.11 / 382.18 / 28.74 | 526.95 / 588.13 / 94.44 |
| dead_tree_trunk_02 | 1 | 1210.84 / 1314.06 / 156.25 | 1365.11 / 1485.22 / 267.37 | 861.56 / 956.47 / 300.01 |
| dead_tree_trunk_02 | 8 | 476.55 / 577.22 / 128.30 | 442.35 / 527.86 / 106.74 | 657.93 / 768.21 / 123.96 |
| namaqualand_boulder_02 | 1 | 1409.87 / 1773.25 / 393.56 | 1438.98 / 1750.80 / 326.97 | 913.99 / 1256.26 / 437.55 |
| namaqualand_boulder_02 | 8 | 512.42 / 801.83 / 302.57 | 525.75 / 1030.12 / 544.78 | 820.42 / 1065.45 / 287.44 |
| coast_rocks_01 | 1 | 9948.33 / 11239.42 / 1707.25 | 9857.46 / 12319.45 / 2957.13 | 7984.12 / 9412.02 / 1788.93 |
| coast_rocks_01 | 8 | 3516.59 / 4140.72 / 926.79 | 3355.13 / 4120.67 / 1071.20 | 8135.33 / 10064.41 / 2705.25 |
| dragon | 1 | 13634.43 / 14024.15 / 1035.64 | 13693.03 / 14571.46 / 1143.94 | 11019.32 / 11379.79 / 1149.15 |
| dragon | 8 | 4563.43 / 4830.52 / 393.42 | 4569.47 / 4614.57 / 176.39 | 10528.87 / 10891.68 / 682.33 |
