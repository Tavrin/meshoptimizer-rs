# P07 individual SIMD performance

Raw native evidence: `/mnt/linux-extra/meshopt-artifacts/p07/performance.json`; raw Node evidence: `/mnt/linux-extra/meshopt-artifacts/p07/wasm-performance.json`.

Time ratios: lower is faster. Family means use stage 1; only borderline case maxima use fresh D146 intervals. Repeated standalone filters are reported separately, outside the SIMD filter bar. See SIMD_RESULTS.md for shared-host and release limits.

| API | Family | Cases | Time Rust/C++ SIMD GM | Worst stage-1 | Time Rust/scalar Rust GM |
|---|---|---:|---:|---:|---:|
| allocating | color | 6 | 1.511 | 1.731 | 0.658 |
| allocating | exp | 12 | 0.951 | 1.132 | 0.999 |
| allocating | index | 14 | 1.154 | 2.160 | 1.016 |
| allocating | meshlet | 12 | 2.296 | 3.566 | 0.859 |
| allocating | meshlet-raw | 3 | 1.830 | 2.361 | 0.723 |
| allocating | oct | 6 | 2.662 | 3.582 | 0.275 |
| allocating | oct (repeated) | 6 | 1.146 | 1.410 | 1.289 |
| allocating | quat | 3 | 2.607 | 4.204 | 0.391 |
| allocating | quat (repeated) | 3 | 0.959 | 1.066 | 1.013 |
| allocating | sequence | 12 | 1.094 | 1.366 | 1.012 |
| allocating | vertex | 22 | 2.283 | 2.812 | 0.747 |
| allocating | view-filtered | 30 | 2.255 | 3.900 | 0.663 |
| allocating | view-none | 9 | 2.121 | 2.658 | 0.756 |
| caller-buffer | color | 6 | 1.492 | 1.697 | 0.741 |
| caller-buffer | exp | 12 | 0.970 | 1.125 | 1.010 |
| caller-buffer | index | 14 | 1.061 | 1.293 | 0.977 |
| caller-buffer | meshlet | 12 | 3.115 | 3.914 | 0.903 |
| caller-buffer | meshlet-raw | 3 | 2.181 | 2.489 | 0.652 |
| caller-buffer | oct | 6 | 2.668 | 3.418 | 0.280 |
| caller-buffer | oct (repeated) | 6 | 1.078 | 1.479 | 1.312 |
| caller-buffer | quat | 3 | 3.218 | 3.843 | 0.367 |
| caller-buffer | quat (repeated) | 3 | 1.133 | 1.213 | 1.047 |
| caller-buffer | sequence | 12 | 1.082 | 1.272 | 0.996 |
| caller-buffer | vertex | 22 | 2.610 | 2.827 | 0.708 |
| caller-buffer | view-filtered | 30 | 2.617 | 3.908 | 0.653 |
| caller-buffer | view-none | 9 | 2.583 | 2.803 | 0.741 |

## Native individual cases

| API | Case | Rust MB/s | C++ SIMD MB/s | Time Rust/C++ SIMD | Time Rust/scalar | Pairs | Final interval | Case bar | S3 |
|---|---|---:|---:|---:|---:|---:|---|---|---|
| allocating | vertex-v0-tiny-s4 | 293.182 | 769.047 | 2.623 | 1.730 | 5 | 2.230–2.575 | fail | FAIL |
| allocating | view-none-tiny-s4 | 301.891 | 695.341 | 2.303 | 0.993 | 5 | 2.075–2.562 | fail | FAIL |
| allocating | vertex-v0-tiny-s12 | 558.101 | 1401.738 | 2.512 | 1.094 | 5 | 2.179–3.183 | fail | pass |
| allocating | view-none-tiny-s12 | 529.778 | 1320.102 | 2.492 | 1.133 | 5 | 2.277–3.182 | fail | pass |
| allocating | vertex-v0-tiny-s32 | 368.081 | 886.159 | 2.408 | 0.861 | 5 | 2.086–2.508 | fail | pass |
| allocating | view-none-tiny-s32 | 396.747 | 903.548 | 2.277 | 0.884 | 5 | 2.237–2.428 | fail | FAIL |
| allocating | filter-1-tiny-s4 | 749.099 | 1056.242 | 1.410 | 1.181 | 5 | 1.341–1.494 | pass | n/a |
| allocating | view-1-tiny-s4 | 373.167 | 647.929 | 1.736 | 1.152 | 5 | 1.662–2.000 | fail | FAIL |
| allocating | filter-1-tiny-s8 | 1393.550 | 1730.134 | 1.242 | 1.390 | 5 | 1.016–1.338 | pass | n/a |
| allocating | view-1-tiny-s8 | 481.628 | 845.652 | 1.756 | 1.243 | 5 | 1.566–2.296 | fail | FAIL |
| allocating | filter-2-tiny-s8 | 1971.341 | 2101.839 | 1.066 | 1.083 | 5 | 1.056–1.465 | pass | n/a |
| allocating | view-2-tiny-s8 | 288.580 | 578.257 | 2.004 | 1.189 | 5 | 1.705–2.746 | fail | FAIL |
| allocating | filter-3-tiny-s12 | 4594.224 | 4503.982 | 0.980 | 1.021 | 5 | 0.909–1.039 | pass | pass |
| allocating | view-3-tiny-s12 | 373.450 | 712.206 | 1.907 | 1.055 | 5 | 1.849–1.960 | fail | FAIL |
| allocating | index-2-v0-tiny-s2 | 447.169 | 533.742 | 1.194 | 0.936 | 5 | 1.145–1.257 | pass | n/a |
| allocating | index-2-v0-tiny-s4 | 815.710 | 1044.213 | 1.280 | 1.111 | 5 | 0.927–1.422 | pass | n/a |
| allocating | index-2-v1-tiny-s2 | 418.943 | 530.119 | 1.265 | 1.023 | 5 | 1.105–1.387 | pass | n/a |
| allocating | index-2-v1-tiny-s4 | 787.083 | 1002.973 | 1.274 | 1.008 | 5 | 1.189–1.307 | pass | n/a |
| allocating | index-3-v0-tiny-s2 | 882.142 | 1142.534 | 1.295 | 0.993 | 8 | 1.113–1.469 | pass | n/a |
| allocating | index-3-v0-tiny-s4 | 1264.398 | 1545.495 | 1.222 | 1.074 | 10 | 1.148–1.497 | pass | n/a |
| allocating | index-3-v1-tiny-s2 | 895.555 | 751.490 | 0.839 | 1.048 | 5 | 0.826–1.435 | pass | n/a |
| allocating | index-3-v1-tiny-s4 | 1578.503 | 2156.346 | 1.366 | 1.088 | 6 | 1.250–1.471 | pass | n/a |
| allocating | vertex-v0-resident-s4 | 1390.202 | 3663.442 | 2.635 | 0.583 | 5 | 1.722–2.957 | fail | pass |
| allocating | view-none-resident-s4 | 1404.609 | 3733.186 | 2.658 | 0.606 | 5 | 2.121–3.228 | fail | pass |
| allocating | vertex-v0-resident-s12 | 1357.885 | 3818.373 | 2.812 | 0.631 | 5 | 2.492–3.821 | fail | pass |
| allocating | view-none-resident-s12 | 1357.698 | 3102.034 | 2.285 | 0.600 | 5 | 2.015–3.279 | fail | pass |
| allocating | vertex-v0-resident-s32 | 909.834 | 2239.962 | 2.462 | 0.522 | 5 | 2.469–2.635 | fail | pass |
| allocating | view-none-resident-s32 | 1345.913 | 3529.517 | 2.622 | 0.606 | 5 | 1.846–3.020 | fail | pass |
| allocating | filter-1-resident-s4 | 3029.396 | 3415.768 | 1.128 | 1.237 | 8 | 0.991–1.474 | pass | n/a |
| allocating | view-1-resident-s4 | 1238.850 | 2312.093 | 1.866 | 0.677 | 5 | 1.759–2.199 | fail | pass |
| allocating | filter-1-resident-s8 | 3383.760 | 3932.647 | 1.162 | 1.776 | 5 | 1.027–1.240 | pass | n/a |
| allocating | view-1-resident-s8 | 781.977 | 2082.818 | 2.664 | 0.717 | 5 | 2.253–2.803 | fail | pass |
| allocating | filter-2-resident-s8 | 5821.156 | 4915.744 | 0.844 | 0.992 | 5 | 0.774–1.425 | pass | n/a |
| allocating | view-2-resident-s8 | 1403.007 | 3512.858 | 2.504 | 0.658 | 5 | 2.441–2.639 | fail | pass |
| allocating | filter-3-resident-s12 | 19964.202 | 18252.600 | 0.914 | 1.008 | 5 | 0.859–0.940 | pass | pass |
| allocating | view-3-resident-s12 | 1403.693 | 5161.267 | 3.677 | 0.679 | 5 | 3.192–4.000 | fail | pass |
| allocating | index-2-v0-resident-s2 | 2033.075 | 2019.436 | 0.993 | 1.002 | 5 | 0.963–1.024 | pass | n/a |
| allocating | index-2-v0-resident-s4 | 3897.779 | 3759.434 | 0.965 | 1.005 | 5 | 0.928–1.009 | pass | n/a |
| allocating | index-2-v1-resident-s2 | 2035.110 | 2004.177 | 0.985 | 1.009 | 5 | 0.907–1.235 | pass | n/a |
| allocating | index-2-v1-resident-s4 | 4051.572 | 3910.763 | 0.965 | 1.030 | 5 | 0.939–1.017 | pass | n/a |
| allocating | index-3-v0-resident-s2 | 1996.539 | 2260.003 | 1.132 | 1.007 | 5 | 1.113–1.149 | pass | n/a |
| allocating | index-3-v0-resident-s4 | 3701.455 | 4237.965 | 1.145 | 1.003 | 5 | 1.067–1.212 | pass | n/a |
| allocating | index-3-v1-resident-s2 | 1884.654 | 2106.899 | 1.118 | 1.023 | 5 | 0.956–1.320 | pass | n/a |
| allocating | index-3-v1-resident-s4 | 3989.522 | 4472.006 | 1.121 | 0.923 | 5 | 1.103–1.140 | pass | n/a |
| allocating | vertex-v0-streaming-s4 | 929.713 | 1537.342 | 1.654 | 0.695 | 5 | 1.550–1.726 | fail | pass |
| allocating | view-none-streaming-s4 | 934.189 | 1527.010 | 1.635 | 0.704 | 5 | 1.636–1.682 | fail | pass |
| allocating | vertex-v0-streaming-s12 | 918.110 | 1501.608 | 1.636 | 0.700 | 5 | 1.429–1.779 | fail | pass |
| allocating | view-none-streaming-s12 | 943.434 | 1580.825 | 1.676 | 0.709 | 5 | 1.647–1.739 | fail | pass |
| allocating | vertex-v0-streaming-s32 | 863.460 | 1299.903 | 1.505 | 0.676 | 7 | 1.308–1.667 | fail | pass |
| allocating | view-none-streaming-s32 | 749.967 | 1142.961 | 1.524 | 0.739 | 5 | 1.327–1.785 | fail | pass |
| allocating | filter-1-streaming-s4 | 1155.054 | 1180.041 | 1.022 | 1.119 | 5 | 0.841–1.349 | pass | n/a |
| allocating | view-1-streaming-s4 | 749.261 | 1140.624 | 1.522 | 0.779 | 20+30 | 1.458–1.676 | borderline | pass |
| allocating | filter-1-streaming-s8 | 1514.284 | 1466.097 | 0.968 | 1.136 | 5 | 0.877–1.013 | pass | n/a |
| allocating | view-1-streaming-s8 | 768.653 | 1194.596 | 1.554 | 0.787 | 6 | 1.504–1.672 | fail | pass |
| allocating | filter-2-streaming-s8 | 1519.421 | 1489.212 | 0.980 | 0.967 | 5 | 0.880–1.030 | pass | n/a |
| allocating | view-2-streaming-s8 | 729.649 | 1263.423 | 1.732 | 0.724 | 5 | 1.572–1.929 | fail | pass |
| allocating | filter-3-streaming-s12 | 1054.592 | 1048.665 | 0.994 | 0.961 | 5 | 0.779–1.146 | pass | pass |
| allocating | view-3-streaming-s12 | 688.093 | 1288.051 | 1.872 | 0.745 | 7 | 1.518–2.125 | fail | pass |
| allocating | index-2-v0-streaming-s2 | 706.732 | 868.999 | 1.230 | 1.037 | 11 | 0.967–1.466 | pass | n/a |
| allocating | index-2-v0-streaming-s4 | 872.765 | 1885.314 | 2.160 | 1.018 | 7 | 1.543–2.600 | fail | n/a |
| allocating | index-2-v1-streaming-s2 | 1598.667 | 1714.510 | 1.072 | 0.983 | 5 | 0.764–1.356 | pass | n/a |
| allocating | index-2-v1-streaming-s4 | 2888.921 | 2828.515 | 0.979 | 1.043 | 5 | 0.707–1.458 | pass | n/a |
| allocating | index-3-v0-streaming-s2 | 1350.158 | 1559.598 | 1.155 | 1.102 | 5 | 0.761–1.449 | pass | n/a |
| allocating | index-3-v0-streaming-s4 | 2809.650 | 3048.445 | 1.085 | 0.878 | 5 | 0.722–1.367 | pass | n/a |
| allocating | index-3-v1-streaming-s2 | 1416.521 | 1108.111 | 0.782 | 1.044 | 5 | 0.768–1.393 | pass | n/a |
| allocating | index-3-v1-streaming-s4 | 2978.128 | 3038.147 | 1.020 | 0.980 | 5 | 0.949–1.238 | pass | n/a |
| allocating | vertex-v1-tiny-s4 | 383.263 | 879.897 | 2.296 | 1.037 | 5 | 2.162–2.357 | fail | pass |
| allocating | vertex-v1-tiny-s12 | 314.337 | 807.008 | 2.567 | 1.101 | 5 | 2.091–2.882 | fail | pass |
| allocating | vertex-v1-tiny-s32 | 427.106 | 857.353 | 2.007 | 0.861 | 5 | 1.618–3.323 | fail | FAIL |
| allocating | vertex-v1-resident-s4 | 1265.875 | 3252.568 | 2.569 | 0.554 | 5 | 2.495–2.694 | fail | pass |
| allocating | vertex-v1-resident-s12 | 1155.095 | 3233.241 | 2.799 | 0.618 | 5 | 2.566–3.277 | fail | pass |
| allocating | vertex-v1-resident-s32 | 1253.673 | 3031.268 | 2.418 | 0.591 | 5 | 2.443–2.831 | fail | pass |
| allocating | vertex-v1-streaming-s4 | 1253.974 | 3173.191 | 2.531 | 0.599 | 5 | 2.513–2.622 | fail | pass |
| allocating | vertex-v1-streaming-s12 | 767.299 | 1160.981 | 1.513 | 0.676 | 5 | 1.385–1.801 | fail | pass |
| allocating | vertex-v1-streaming-s32 | 709.844 | 1170.382 | 1.649 | 0.756 | 5 | 1.416–1.884 | fail | pass |
| allocating | vertex-v0-resident-s12-level0 | 1312.481 | 3551.290 | 2.706 | 0.637 | 5 | 2.221–2.981 | fail | pass |
| allocating | vertex-v0-resident-s12-level9 | 1230.398 | 3264.935 | 2.654 | 0.651 | 5 | 2.373–3.216 | fail | pass |
| allocating | vertex-v1-resident-s12-level0 | 1321.568 | 3650.597 | 2.762 | 0.631 | 5 | 2.336–3.082 | fail | pass |
| allocating | vertex-v1-resident-s12-level9 | 814.672 | 2065.458 | 2.535 | 1.010 | 5 | 1.619–4.040 | fail | pass |
| allocating | index-2-v0-millions-s4 | 1144.053 | 1272.095 | 1.112 | 1.039 | 5 | 1.121–1.270 | pass | n/a |
| allocating | index-2-v1-millions-s4 | 1236.812 | 1316.027 | 1.064 | 0.984 | 5 | 0.786–1.225 | pass | n/a |
| allocating | varied-filter-1-tiny-s4 | 430.082 | 1055.600 | 2.454 | 0.411 | 5 | 2.064–2.529 | fail | pass |
| allocating | varied-view-1-tiny-s4 | 233.153 | 478.297 | 2.051 | 0.531 | 6 | 1.597–2.737 | fail | pass |
| allocating | varied-filter-1-tiny-s8 | 1080.383 | 1832.059 | 1.696 | 0.305 | 5 | 1.652–2.383 | fail | pass |
| allocating | varied-view-1-tiny-s8 | 380.906 | 665.924 | 1.748 | 0.773 | 5 | 1.758–2.285 | fail | pass |
| allocating | varied-filter-2-tiny-s8 | 1137.912 | 2664.078 | 2.341 | 0.472 | 5 | 2.201–2.383 | fail | pass |
| allocating | varied-view-2-tiny-s8 | 207.709 | 475.574 | 2.290 | 0.744 | 5 | 2.276–2.504 | fail | pass |
| allocating | varied-filter-3-tiny-s4 | 1866.475 | 1713.182 | 0.918 | 1.029 | 5 | 0.895–0.946 | pass | pass |
| allocating | varied-view-3-tiny-s4 | 250.005 | 468.108 | 1.872 | 1.016 | 5 | 1.757–2.035 | fail | FAIL |
| allocating | varied-filter-3-tiny-s12 | 4041.924 | 4103.944 | 1.015 | 0.909 | 5 | 0.992–1.032 | pass | pass |
| allocating | varied-view-3-tiny-s12 | 301.115 | 612.269 | 2.033 | 1.011 | 5 | 1.888–2.390 | fail | pass |
| allocating | varied-filter-3-tiny-s32 | 7332.400 | 7526.592 | 1.026 | 0.985 | 5 | 0.989–1.103 | pass | pass |
| allocating | varied-view-3-tiny-s32 | 325.726 | 712.676 | 2.188 | 0.992 | 5 | 2.118–2.253 | fail | pass |
| allocating | varied-filter-4-tiny-s4 | 709.124 | 1037.775 | 1.463 | 0.768 | 9 | 1.429–1.494 | pass | pass |
| allocating | varied-filter-4-tiny-s8 | 1272.855 | 1768.581 | 1.389 | 0.780 | 5 | 1.292–1.410 | pass | pass |
| allocating | varied-filter-1-resident-s4 | 566.242 | 1972.054 | 3.483 | 0.231 | 5 | 3.322–3.842 | fail | pass |
| allocating | varied-view-1-resident-s4 | 321.535 | 1202.768 | 3.741 | 0.316 | 5 | 3.309–4.422 | fail | pass |
| allocating | varied-filter-1-resident-s8 | 908.213 | 3253.057 | 3.582 | 0.295 | 5 | 3.368–3.731 | fail | pass |
| allocating | varied-view-1-resident-s8 | 410.292 | 1153.167 | 2.811 | 0.357 | 6 | 1.735–4.565 | fail | pass |
| allocating | varied-filter-2-resident-s8 | 788.277 | 3313.920 | 4.204 | 0.398 | 5 | 3.986–4.418 | fail | pass |
| allocating | varied-view-2-resident-s8 | 383.796 | 1496.662 | 3.900 | 0.494 | 5 | 3.836–4.005 | fail | pass |
| allocating | varied-filter-3-resident-s4 | 11019.530 | 9096.345 | 0.825 | 1.046 | 5 | 0.810–0.834 | pass | FAIL |
| allocating | varied-view-3-resident-s4 | 669.877 | 1812.484 | 2.706 | 0.661 | 5 | 2.455–2.799 | fail | pass |
| allocating | varied-filter-3-resident-s12 | 11647.278 | 9881.935 | 0.848 | 1.042 | 5 | 0.767–0.888 | pass | FAIL |
| allocating | varied-view-3-resident-s12 | 631.657 | 1870.794 | 2.962 | 0.710 | 5 | 2.599–3.396 | fail | pass |
| allocating | varied-filter-3-resident-s32 | 11705.909 | 9897.344 | 0.845 | 1.009 | 5 | 0.669–1.373 | pass | pass |
| allocating | varied-view-3-resident-s32 | 639.679 | 1893.588 | 2.960 | 0.637 | 5 | 2.407–4.002 | fail | pass |
| allocating | varied-filter-4-resident-s4 | 1366.590 | 2354.453 | 1.723 | 0.554 | 20+30 | 1.452–1.766 | borderline | pass |
| allocating | varied-filter-4-resident-s8 | 3376.313 | 4490.919 | 1.330 | 0.623 | 20+30 | 1.456–1.560 | borderline | pass |
| allocating | varied-filter-1-streaming-s4 | 1007.876 | 2682.312 | 2.661 | 0.175 | 5 | 2.485–3.574 | fail | pass |
| allocating | varied-view-1-streaming-s4 | 716.633 | 2284.052 | 3.187 | 0.237 | 5 | 2.987–3.840 | fail | pass |
| allocating | varied-filter-1-streaming-s8 | 1375.677 | 3545.433 | 2.577 | 0.287 | 5 | 2.175–3.261 | fail | pass |
| allocating | varied-view-1-streaming-s8 | 648.728 | 1604.600 | 2.473 | 0.344 | 5 | 1.959–3.363 | fail | pass |
| allocating | varied-filter-2-streaming-s8 | 1274.166 | 2294.143 | 1.801 | 0.319 | 5 | 1.930–3.358 | fail | pass |
| allocating | varied-view-2-streaming-s8 | 443.161 | 1136.126 | 2.564 | 0.366 | 5 | 2.294–2.802 | fail | pass |
| allocating | varied-filter-3-streaming-s4 | 7847.302 | 8576.682 | 1.093 | 0.972 | 7 | 0.812–1.457 | pass | pass |
| allocating | varied-view-3-streaming-s4 | 807.843 | 2182.616 | 2.702 | 0.557 | 5 | 2.539–2.742 | fail | pass |
| allocating | varied-filter-3-streaming-s12 | 3590.665 | 4064.421 | 1.132 | 1.007 | 5 | 0.875–1.407 | pass | pass |
| allocating | varied-view-3-streaming-s12 | 555.999 | 931.133 | 1.675 | 0.627 | 18 | 1.500–1.869 | fail | pass |
| allocating | varied-filter-3-streaming-s32 | 1787.657 | 1569.026 | 0.878 | 1.014 | 5 | 0.715–1.396 | pass | pass |
| allocating | varied-view-3-streaming-s32 | 709.664 | 1121.059 | 1.580 | 0.640 | 6 | 1.506–1.739 | fail | pass |
| allocating | varied-filter-4-streaming-s4 | 1843.427 | 3191.869 | 1.731 | 0.597 | 5 | 1.658–1.773 | fail | pass |
| allocating | varied-filter-4-streaming-s8 | 3413.392 | 5032.461 | 1.474 | 0.659 | 20+30 | 1.432–1.518 | borderline | pass |
| allocating | meshlet-1-1-v2-t3 | 116.324 | 173.365 | 1.490 | 1.203 | 6 | 1.301–1.568 | fail | FAIL |
| allocating | meshlet-1-1-v2-t4 | 142.144 | 214.762 | 1.511 | 1.188 | 5 | 1.397–1.782 | fail | FAIL |
| allocating | meshlet-1-1-v4-t3 | 140.426 | 210.622 | 1.500 | 1.218 | 7 | 1.316–1.553 | fail | FAIL |
| allocating | meshlet-1-1-v4-t4 | 176.825 | 216.615 | 1.225 | 1.242 | 11 | 1.308–1.628 | fail | FAIL |
| allocating | meshlet-raw-1-1 | 241.846 | 289.604 | 1.197 | 1.077 | 6 | 1.145–1.289 | pass | FAIL |
| allocating | meshlet-64-126-v2-t3 | 2101.501 | 5392.972 | 2.566 | 0.672 | 5 | 1.826–3.676 | fail | pass |
| allocating | meshlet-64-126-v2-t4 | 2341.398 | 6771.050 | 2.892 | 0.778 | 5 | 2.714–3.219 | fail | pass |
| allocating | meshlet-64-126-v4-t3 | 2657.477 | 6739.272 | 2.536 | 0.651 | 5 | 1.992–2.875 | fail | pass |
| allocating | meshlet-64-126-v4-t4 | 2780.938 | 8040.292 | 2.891 | 0.772 | 5 | 2.548–3.588 | fail | pass |
| allocating | meshlet-raw-64-126 | 3573.729 | 7747.770 | 2.168 | 0.596 | 5 | 1.991–2.694 | fail | pass |
| allocating | meshlet-256-256-v2-t3 | 1678.072 | 5983.457 | 3.566 | 0.757 | 5 | 2.557–4.091 | fail | pass |
| allocating | meshlet-256-256-v2-t4 | 2301.490 | 7146.287 | 3.105 | 0.757 | 5 | 2.920–3.326 | fail | pass |
| allocating | meshlet-256-256-v4-t3 | 2878.882 | 8391.474 | 2.915 | 0.665 | 5 | 2.493–3.094 | fail | pass |
| allocating | meshlet-256-256-v4-t4 | 3061.713 | 9050.163 | 2.956 | 0.742 | 5 | 2.871–3.336 | fail | pass |
| allocating | meshlet-raw-256-256 | 3893.378 | 9191.856 | 2.361 | 0.588 | 5 | 2.155–2.477 | fail | pass |
| caller-buffer | vertex-v0-tiny-s4 | 490.320 | 1249.056 | 2.547 | 1.069 | 5 | 2.029–2.886 | fail | FAIL |
| caller-buffer | view-none-tiny-s4 | 491.864 | 1255.275 | 2.552 | 1.103 | 5 | 2.364–2.999 | fail | FAIL |
| caller-buffer | vertex-v0-tiny-s12 | 619.366 | 1530.655 | 2.471 | 1.065 | 5 | 2.418–2.518 | fail | FAIL |
| caller-buffer | view-none-tiny-s12 | 616.250 | 1505.954 | 2.444 | 1.074 | 5 | 2.226–2.598 | fail | FAIL |
| caller-buffer | vertex-v0-tiny-s32 | 658.254 | 1638.070 | 2.489 | 1.060 | 5 | 2.377–2.727 | fail | pass |
| caller-buffer | view-none-tiny-s32 | 647.583 | 1609.914 | 2.486 | 1.069 | 5 | 2.448–2.501 | fail | FAIL |
| caller-buffer | filter-1-tiny-s4 | 1380.530 | 1755.446 | 1.272 | 1.161 | 5 | 1.263–1.283 | pass | n/a |
| caller-buffer | view-1-tiny-s4 | 457.133 | 774.777 | 1.695 | 1.179 | 5 | 1.662–1.720 | fail | FAIL |
| caller-buffer | filter-1-tiny-s8 | 2528.638 | 2537.678 | 1.004 | 1.320 | 5 | 0.842–1.127 | pass | n/a |
| caller-buffer | view-1-tiny-s8 | 571.573 | 984.429 | 1.722 | 1.220 | 5 | 1.665–1.757 | fail | FAIL |
| caller-buffer | filter-2-tiny-s8 | 2472.115 | 2997.938 | 1.213 | 1.091 | 5 | 1.180–1.231 | pass | n/a |
| caller-buffer | view-2-tiny-s8 | 583.249 | 1051.164 | 1.802 | 1.163 | 5 | 1.705–1.868 | fail | FAIL |
| caller-buffer | filter-3-tiny-s12 | 10434.198 | 9767.456 | 0.936 | 1.012 | 5 | 0.752–1.059 | pass | pass |
| caller-buffer | view-3-tiny-s12 | 654.367 | 1319.442 | 2.016 | 1.233 | 5 | 1.853–2.138 | fail | FAIL |
| caller-buffer | index-2-v0-tiny-s2 | 1225.091 | 1370.631 | 1.119 | 0.980 | 5 | 1.036–1.339 | pass | n/a |
| caller-buffer | index-2-v0-tiny-s4 | 2310.632 | 2620.702 | 1.134 | 0.737 | 5 | 1.107–1.247 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s2 | 1249.124 | 1401.520 | 1.122 | 0.925 | 5 | 0.786–1.346 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s4 | 2485.115 | 2841.912 | 1.144 | 1.009 | 5 | 0.943–1.277 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s2 | 1339.598 | 1269.786 | 0.948 | 0.978 | 5 | 0.719–1.214 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s4 | 2744.334 | 2619.408 | 0.954 | 0.994 | 5 | 0.946–0.969 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s2 | 1366.406 | 1332.180 | 0.975 | 0.989 | 5 | 0.906–1.080 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s4 | 2754.361 | 2603.245 | 0.945 | 0.998 | 5 | 0.870–1.004 | pass | n/a |
| caller-buffer | vertex-v0-resident-s4 | 1471.722 | 3980.468 | 2.705 | 0.604 | 5 | 2.670–2.770 | fail | pass |
| caller-buffer | view-none-resident-s4 | 1405.630 | 3905.781 | 2.779 | 0.610 | 5 | 2.217–3.080 | fail | pass |
| caller-buffer | vertex-v0-resident-s12 | 1404.787 | 3894.696 | 2.772 | 0.609 | 5 | 2.527–2.876 | fail | pass |
| caller-buffer | view-none-resident-s12 | 1478.003 | 4009.585 | 2.713 | 0.611 | 5 | 2.524–3.296 | fail | pass |
| caller-buffer | vertex-v0-resident-s32 | 1475.062 | 4170.043 | 2.827 | 0.611 | 5 | 2.098–3.277 | fail | pass |
| caller-buffer | view-none-resident-s32 | 1498.864 | 4201.586 | 2.803 | 0.605 | 5 | 2.751–2.858 | fail | pass |
| caller-buffer | filter-1-resident-s4 | 3912.695 | 3703.181 | 0.946 | 1.161 | 5 | 0.937–0.961 | pass | n/a |
| caller-buffer | view-1-resident-s4 | 1327.601 | 2511.595 | 1.892 | 0.700 | 5 | 1.837–1.998 | fail | pass |
| caller-buffer | filter-1-resident-s8 | 6526.616 | 5798.320 | 0.888 | 1.672 | 5 | 0.868–0.914 | pass | n/a |
| caller-buffer | view-1-resident-s8 | 1555.986 | 3505.480 | 2.253 | 0.678 | 5 | 2.200–2.294 | fail | pass |
| caller-buffer | filter-2-resident-s8 | 6136.456 | 6441.368 | 1.050 | 1.003 | 5 | 0.945–1.313 | pass | n/a |
| caller-buffer | view-2-resident-s8 | 1527.837 | 3886.063 | 2.544 | 0.641 | 5 | 2.373–2.678 | fail | pass |
| caller-buffer | filter-3-resident-s12 | 20075.649 | 18734.918 | 0.933 | 1.005 | 5 | 0.861–1.070 | pass | pass |
| caller-buffer | view-3-resident-s12 | 1498.414 | 4882.528 | 3.258 | 0.647 | 5 | 3.167–3.774 | fail | pass |
| caller-buffer | index-2-v0-resident-s2 | 2086.457 | 1988.529 | 0.953 | 1.010 | 5 | 0.839–1.039 | pass | n/a |
| caller-buffer | index-2-v0-resident-s4 | 4360.461 | 3953.013 | 0.907 | 0.939 | 5 | 0.797–1.274 | pass | n/a |
| caller-buffer | index-2-v1-resident-s2 | 2122.586 | 2101.771 | 0.990 | 0.988 | 5 | 0.734–1.448 | pass | n/a |
| caller-buffer | index-2-v1-resident-s4 | 4211.197 | 4058.109 | 0.964 | 1.008 | 5 | 0.668–1.184 | pass | n/a |
| caller-buffer | index-3-v0-resident-s2 | 2083.258 | 2338.218 | 1.122 | 1.000 | 5 | 0.953–1.213 | pass | n/a |
| caller-buffer | index-3-v0-resident-s4 | 2764.060 | 3515.141 | 1.272 | 1.000 | 5 | 1.082–1.407 | pass | n/a |
| caller-buffer | index-3-v1-resident-s2 | 1890.756 | 2213.759 | 1.171 | 0.982 | 5 | 1.073–1.438 | pass | n/a |
| caller-buffer | index-3-v1-resident-s4 | 4019.138 | 4599.683 | 1.144 | 0.978 | 13 | 1.131–1.476 | pass | n/a |
| caller-buffer | vertex-v0-streaming-s4 | 1467.104 | 4001.236 | 2.727 | 0.608 | 5 | 2.659–2.820 | fail | pass |
| caller-buffer | view-none-streaming-s4 | 1420.690 | 3461.146 | 2.436 | 0.617 | 5 | 1.685–3.445 | fail | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1447.567 | 3958.886 | 2.735 | 0.613 | 5 | 2.279–3.456 | fail | pass |
| caller-buffer | view-none-streaming-s12 | 1458.637 | 4079.626 | 2.797 | 0.620 | 5 | 2.763–2.823 | fail | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1460.962 | 3391.177 | 2.321 | 0.617 | 5 | 2.289–2.345 | fail | pass |
| caller-buffer | view-none-streaming-s32 | 1466.920 | 3367.745 | 2.296 | 0.616 | 5 | 2.234–2.313 | fail | pass |
| caller-buffer | filter-1-streaming-s4 | 3713.269 | 3674.557 | 0.990 | 1.134 | 5 | 0.890–1.054 | pass | n/a |
| caller-buffer | view-1-streaming-s4 | 1098.682 | 2128.641 | 1.937 | 0.724 | 5 | 1.974–2.232 | fail | pass |
| caller-buffer | filter-1-streaming-s8 | 2737.554 | 4049.354 | 1.479 | 1.511 | 5 | 0.849–1.388 | pass | n/a |
| caller-buffer | view-1-streaming-s8 | 1467.614 | 3561.617 | 2.427 | 0.682 | 5 | 2.366–2.465 | fail | pass |
| caller-buffer | filter-2-streaming-s8 | 4597.161 | 5252.119 | 1.142 | 1.050 | 5 | 1.028–1.404 | pass | n/a |
| caller-buffer | view-2-streaming-s8 | 1500.146 | 4022.267 | 2.681 | 0.642 | 5 | 2.568–2.836 | fail | pass |
| caller-buffer | filter-3-streaming-s12 | 3947.434 | 3960.648 | 1.003 | 1.020 | 5 | 0.976–1.028 | pass | pass |
| caller-buffer | view-3-streaming-s12 | 1518.034 | 4908.359 | 3.233 | 0.670 | 5 | 3.069–3.488 | fail | pass |
| caller-buffer | index-2-v0-streaming-s2 | 1081.863 | 1339.926 | 1.239 | 1.054 | 5 | 1.223–1.263 | pass | n/a |
| caller-buffer | index-2-v0-streaming-s4 | 2159.794 | 2792.808 | 1.293 | 1.044 | 5 | 1.174–1.356 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s2 | 2099.744 | 1997.026 | 0.951 | 0.997 | 5 | 0.909–0.969 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s4 | 4309.973 | 3951.461 | 0.917 | 0.984 | 5 | 0.817–0.964 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s2 | 2006.732 | 2233.119 | 1.113 | 0.991 | 5 | 1.039–1.200 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s4 | 4091.416 | 4609.157 | 1.127 | 1.002 | 5 | 1.085–1.180 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s2 | 2041.147 | 2294.189 | 1.124 | 1.008 | 5 | 1.111–1.151 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s4 | 4066.590 | 4681.488 | 1.151 | 1.033 | 5 | 1.091–1.186 | pass | n/a |
| caller-buffer | vertex-v1-tiny-s4 | 511.541 | 1254.455 | 2.452 | 1.061 | 5 | 2.183–2.627 | fail | FAIL |
| caller-buffer | vertex-v1-tiny-s12 | 614.539 | 1499.575 | 2.440 | 1.071 | 5 | 2.357–2.600 | fail | pass |
| caller-buffer | vertex-v1-tiny-s32 | 661.040 | 1601.504 | 2.423 | 1.042 | 5 | 1.779–2.814 | fail | FAIL |
| caller-buffer | vertex-v1-resident-s4 | 1446.322 | 4077.625 | 2.819 | 0.616 | 5 | 2.712–2.903 | fail | pass |
| caller-buffer | vertex-v1-resident-s12 | 1459.871 | 4067.828 | 2.786 | 0.611 | 5 | 2.653–2.831 | fail | pass |
| caller-buffer | vertex-v1-resident-s32 | 1429.461 | 3974.959 | 2.781 | 0.604 | 5 | 2.725–2.837 | fail | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1455.701 | 3879.148 | 2.665 | 0.594 | 5 | 2.352–2.891 | fail | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1446.439 | 3197.031 | 2.210 | 0.606 | 5 | 1.663–2.541 | fail | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1468.039 | 3313.531 | 2.257 | 0.612 | 5 | 2.206–2.406 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1500.661 | 4203.586 | 2.801 | 0.604 | 5 | 2.742–2.905 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1511.927 | 4185.802 | 2.769 | 0.606 | 5 | 2.650–2.813 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1502.206 | 4216.600 | 2.807 | 0.609 | 5 | 2.780–2.854 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1512.750 | 4223.783 | 2.792 | 0.610 | 5 | 2.643–2.875 | fail | pass |
| caller-buffer | index-2-v0-millions-s4 | 2226.325 | 2844.633 | 1.278 | 1.056 | 5 | 1.269–1.307 | pass | n/a |
| caller-buffer | index-2-v1-millions-s4 | 4405.312 | 4232.040 | 0.961 | 0.995 | 5 | 0.943–0.963 | pass | n/a |
| caller-buffer | varied-filter-1-tiny-s4 | 852.086 | 1751.196 | 2.055 | 0.347 | 5 | 1.997–2.133 | fail | pass |
| caller-buffer | varied-view-1-tiny-s4 | 339.766 | 751.840 | 2.213 | 0.589 | 5 | 2.082–2.446 | fail | pass |
| caller-buffer | varied-filter-1-tiny-s8 | 1187.042 | 2372.712 | 1.999 | 0.459 | 5 | 1.960–2.035 | fail | pass |
| caller-buffer | varied-view-1-tiny-s8 | 437.908 | 973.003 | 2.222 | 0.747 | 5 | 2.144–2.304 | fail | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 1303.760 | 3008.400 | 2.307 | 0.493 | 5 | 2.236–2.455 | fail | pass |
| caller-buffer | varied-view-2-tiny-s8 | 438.506 | 1047.711 | 2.389 | 0.758 | 5 | 2.298–2.491 | fail | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 4006.533 | 3590.784 | 0.896 | 1.005 | 5 | 0.870–0.904 | pass | pass |
| caller-buffer | varied-view-3-tiny-s4 | 514.552 | 1040.136 | 2.021 | 1.107 | 5 | 1.886–2.254 | fail | FAIL |
| caller-buffer | varied-filter-3-tiny-s12 | 10860.734 | 9863.548 | 0.908 | 0.982 | 5 | 0.820–1.107 | pass | pass |
| caller-buffer | varied-view-3-tiny-s12 | 586.261 | 1390.943 | 2.373 | 1.184 | 5 | 2.260–2.419 | fail | FAIL |
| caller-buffer | varied-filter-3-tiny-s32 | 19607.104 | 22056.912 | 1.125 | 0.971 | 5 | 1.081–1.191 | pass | pass |
| caller-buffer | varied-view-3-tiny-s32 | 603.748 | 1553.860 | 2.574 | 1.128 | 5 | 2.445–2.624 | fail | FAIL |
| caller-buffer | varied-filter-4-tiny-s4 | 1542.870 | 2198.691 | 1.425 | 0.853 | 5 | 1.398–1.447 | pass | pass |
| caller-buffer | varied-filter-4-tiny-s8 | 2799.015 | 3568.917 | 1.275 | 0.864 | 5 | 0.955–1.467 | pass | pass |
| caller-buffer | varied-filter-1-resident-s4 | 1432.795 | 3931.737 | 2.744 | 0.224 | 5 | 2.521–2.931 | fail | pass |
| caller-buffer | varied-view-1-resident-s4 | 791.527 | 2646.924 | 3.344 | 0.313 | 5 | 3.262–3.447 | fail | pass |
| caller-buffer | varied-filter-1-resident-s8 | 1809.987 | 6058.068 | 3.347 | 0.353 | 5 | 3.294–3.373 | fail | pass |
| caller-buffer | varied-view-1-resident-s8 | 877.700 | 3154.790 | 3.594 | 0.445 | 5 | 3.575–3.648 | fail | pass |
| caller-buffer | varied-filter-2-resident-s8 | 1896.775 | 7130.232 | 3.759 | 0.358 | 5 | 3.696–3.875 | fail | pass |
| caller-buffer | varied-view-2-resident-s8 | 881.664 | 3403.326 | 3.860 | 0.455 | 5 | 3.795–3.912 | fail | pass |
| caller-buffer | varied-filter-3-resident-s4 | 20455.904 | 19296.948 | 0.943 | 1.026 | 5 | 0.838–1.054 | pass | pass |
| caller-buffer | varied-view-3-resident-s4 | 1450.135 | 4120.779 | 2.842 | 0.643 | 5 | 2.777–2.888 | fail | pass |
| caller-buffer | varied-filter-3-resident-s12 | 20919.149 | 19629.173 | 0.938 | 1.021 | 5 | 0.914–0.988 | pass | pass |
| caller-buffer | varied-view-3-resident-s12 | 1289.595 | 4097.057 | 3.177 | 0.666 | 5 | 3.117–3.194 | fail | pass |
| caller-buffer | varied-filter-3-resident-s32 | 20141.258 | 19681.466 | 0.977 | 0.970 | 5 | 0.943–1.055 | pass | pass |
| caller-buffer | varied-view-3-resident-s32 | 1367.435 | 3974.407 | 2.906 | 0.622 | 5 | 2.718–3.139 | fail | pass |
| caller-buffer | varied-filter-4-resident-s4 | 2624.763 | 4288.729 | 1.634 | 0.649 | 7 | 1.505–1.905 | fail | pass |
| caller-buffer | varied-filter-4-resident-s8 | 5180.431 | 7615.867 | 1.470 | 0.713 | 11 | 1.400–1.496 | pass | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 1432.568 | 4007.904 | 2.798 | 0.153 | 5 | 2.728–2.826 | fail | pass |
| caller-buffer | varied-view-1-streaming-s4 | 804.346 | 2650.073 | 3.295 | 0.229 | 5 | 3.272–3.361 | fail | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 1820.094 | 6221.231 | 3.418 | 0.250 | 5 | 3.399–3.433 | fail | pass |
| caller-buffer | varied-view-1-streaming-s8 | 860.736 | 3141.626 | 3.650 | 0.335 | 5 | 3.600–3.717 | fail | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 1895.180 | 7283.014 | 3.843 | 0.280 | 5 | 3.754–3.975 | fail | pass |
| caller-buffer | varied-view-2-streaming-s8 | 880.438 | 3440.438 | 3.908 | 0.343 | 5 | 3.868–3.914 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 21915.328 | 20937.593 | 0.955 | 1.020 | 5 | 0.925–0.976 | pass | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1458.533 | 4151.745 | 2.847 | 0.615 | 5 | 2.759–2.903 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 21603.080 | 20200.944 | 0.935 | 1.008 | 5 | 0.917–0.947 | pass | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1354.205 | 4183.225 | 3.089 | 0.489 | 5 | 3.000–3.185 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 16603.128 | 18544.920 | 1.117 | 1.090 | 5 | 0.857–1.264 | pass | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1316.890 | 4064.083 | 3.086 | 0.494 | 5 | 2.877–3.559 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s4 | 2865.483 | 4861.839 | 1.697 | 0.680 | 5 | 1.691–1.728 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s8 | 5253.236 | 7827.352 | 1.490 | 0.713 | 11 | 1.458–1.499 | pass | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 200.917 | 475.049 | 2.364 | 1.425 | 5 | 2.063–2.974 | fail | FAIL |
| caller-buffer | meshlet-1-1-v2-t4 | 170.338 | 553.940 | 3.252 | 1.965 | 5 | 2.167–3.141 | fail | FAIL |
| caller-buffer | meshlet-1-1-v4-t3 | 270.666 | 652.323 | 2.410 | 1.442 | 5 | 2.150–2.805 | fail | FAIL |
| caller-buffer | meshlet-1-1-v4-t4 | 296.756 | 759.706 | 2.560 | 1.533 | 5 | 2.050–2.615 | fail | FAIL |
| caller-buffer | meshlet-raw-1-1 | 470.055 | 822.766 | 1.750 | 1.038 | 5 | 1.402–1.991 | fail | pass |
| caller-buffer | meshlet-64-126-v2-t3 | 1490.601 | 4907.226 | 3.292 | 0.621 | 5 | 3.039–3.478 | fail | pass |
| caller-buffer | meshlet-64-126-v2-t4 | 1347.980 | 5276.619 | 3.914 | 0.703 | 5 | 2.688–9.403 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t3 | 2660.578 | 7686.405 | 2.889 | 0.618 | 5 | 2.613–4.154 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t4 | 3001.262 | 10178.074 | 3.391 | 0.721 | 5 | 3.300–3.525 | fail | pass |
| caller-buffer | meshlet-raw-64-126 | 4141.143 | 9854.834 | 2.380 | 0.505 | 5 | 2.010–3.343 | fail | pass |
| caller-buffer | meshlet-256-256-v2-t3 | 2166.835 | 7056.644 | 3.257 | 0.653 | 5 | 3.222–3.347 | fail | pass |
| caller-buffer | meshlet-256-256-v2-t4 | 2351.556 | 8461.903 | 3.598 | 0.753 | 5 | 3.549–3.715 | fail | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 2984.925 | 9951.225 | 3.334 | 0.669 | 5 | 3.229–3.401 | fail | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 3186.090 | 11374.664 | 3.570 | 0.747 | 5 | 3.528–3.704 | fail | pass |
| caller-buffer | meshlet-raw-256-256 | 4638.056 | 11545.229 | 2.489 | 0.528 | 5 | 2.322–2.666 | fail | pass |

## S4 scalar C++ case intervals

The original family means and medians stay fixed. Only scalar-baseline borderline cases receive additional pairs, up to twenty total; remaining borderline cases alone receive thirty fresh D146 pairs. No clear failure is retried.

| API | Case | Original pairs | Additional stage-1 pairs | Fresh stage-2 pairs | Final interval | S4 maximum |
|---|---|---:|---:|---:|---|---|
| allocating | index-2-v0-tiny-s2 | 5 | 0 | 0 | 1.047–1.290 | pass |
| allocating | index-2-v0-tiny-s4 | 5 | 0 | 0 | 1.022–1.269 | pass |
| allocating | index-2-v1-tiny-s2 | 5 | 0 | 0 | 1.126–1.258 | pass |
| allocating | index-2-v1-tiny-s4 | 5 | 5 | 0 | 1.056–1.473 | pass |
| allocating | index-3-v0-tiny-s2 | 8 | 0 | 0 | 1.143–1.409 | pass |
| allocating | index-3-v0-tiny-s4 | 10 | 0 | 0 | 1.218–1.418 | pass |
| allocating | index-3-v1-tiny-s2 | 5 | 0 | 0 | 1.074–1.480 | pass |
| allocating | index-3-v1-tiny-s4 | 6 | 0 | 0 | 1.292–1.466 | pass |
| allocating | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.883–1.032 | pass |
| allocating | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.908–0.995 | pass |
| allocating | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.922–1.240 | pass |
| allocating | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.823–0.993 | pass |
| allocating | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.079–1.236 | pass |
| allocating | index-3-v0-resident-s4 | 5 | 0 | 0 | 0.797–1.317 | pass |
| allocating | index-3-v1-resident-s2 | 5 | 0 | 0 | 0.904–1.346 | pass |
| allocating | index-3-v1-resident-s4 | 5 | 0 | 0 | 1.009–1.179 | pass |
| allocating | index-2-v0-streaming-s2 | 11 | 0 | 0 | 1.042–1.449 | pass |
| allocating | index-2-v0-streaming-s4 | 7 | 0 | 0 | 1.674–2.494 | fail |
| allocating | index-2-v1-streaming-s2 | 5 | 0 | 0 | 0.683–1.390 | pass |
| allocating | index-2-v1-streaming-s4 | 5 | 0 | 0 | 0.646–1.417 | pass |
| allocating | index-3-v0-streaming-s2 | 5 | 0 | 0 | 0.840–1.337 | pass |
| allocating | index-3-v0-streaming-s4 | 5 | 0 | 0 | 0.663–1.455 | pass |
| allocating | index-3-v1-streaming-s2 | 5 | 2 | 0 | 1.026–1.491 | pass |
| allocating | index-3-v1-streaming-s4 | 5 | 0 | 0 | 0.754–1.307 | pass |
| allocating | index-2-v0-millions-s4 | 5 | 0 | 0 | 1.099–1.184 | pass |
| allocating | index-2-v1-millions-s4 | 5 | 1 | 0 | 0.422–1.395 | pass |
| caller-buffer | index-2-v0-tiny-s2 | 5 | 0 | 0 | 0.999–1.483 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 5 | 0 | 0 | 0.766–1.452 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 5 | 0 | 0 | 0.714–1.286 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 5 | 0 | 0 | 0.979–1.239 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 5 | 0 | 0 | 0.856–1.166 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 5 | 0 | 0 | 0.913–0.977 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 5 | 0 | 0 | 0.908–1.095 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 5 | 0 | 0 | 0.923–0.971 | pass |
| caller-buffer | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.959–1.064 | pass |
| caller-buffer | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.779–1.264 | pass |
| caller-buffer | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.669–1.455 | pass |
| caller-buffer | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.922–1.030 | pass |
| caller-buffer | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.022–1.189 | pass |
| caller-buffer | index-3-v0-resident-s4 | 5 | 0 | 0 | 1.082–1.348 | pass |
| caller-buffer | index-3-v1-resident-s2 | 5 | 1 | 0 | 1.077–1.456 | pass |
| caller-buffer | index-3-v1-resident-s4 | 13 | 0 | 0 | 0.965–1.445 | pass |
| caller-buffer | index-2-v0-streaming-s2 | 5 | 0 | 0 | 1.282–1.314 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 5 | 0 | 0 | 1.261–1.311 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 5 | 0 | 0 | 0.971–1.016 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 5 | 0 | 0 | 0.858–0.939 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 5 | 0 | 0 | 1.090–1.182 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 5 | 0 | 0 | 1.088–1.188 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 5 | 0 | 0 | 1.118–1.150 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 5 | 0 | 0 | 1.102–1.167 | pass |
| caller-buffer | index-2-v0-millions-s4 | 5 | 0 | 0 | 1.278–1.305 | pass |
| caller-buffer | index-2-v1-millions-s4 | 5 | 0 | 0 | 0.914–0.930 | pass |

## Node shipped SIMD decoder

| API | Family | Cases | Time Rust/upstream SIMD GM | Worst | Time Rust/scalar GM |
|---|---|---:|---:|---:|---:|
| allocating | index | 14 | 1.112 | 1.497 | 1.014 |
| allocating | sequence | 12 | 0.962 | 1.084 | 0.963 |
| allocating | vertex | 22 | 3.924 | 5.600 | 1.526 |
| allocating | view-filtered | 30 | 3.200 | 6.732 | 1.006 |
| allocating | view-none | 9 | 3.885 | 5.610 | 1.532 |
| caller-buffer | index | 14 | 1.087 | 1.610 | 1.040 |
| caller-buffer | sequence | 12 | 0.962 | 1.225 | 1.013 |
| caller-buffer | vertex | 22 | 4.714 | 7.281 | 1.601 |
| caller-buffer | view-filtered | 30 | 3.834 | 6.976 | 1.010 |
| caller-buffer | view-none | 9 | 4.471 | 6.098 | 1.578 |

| API | Case | Time Rust/upstream SIMD | Time Rust/scalar | Pairs | Final interval | Case bar |
|---|---|---:|---:|---:|---|---|
| allocating | vertex-v0-tiny-s4 | 1.314 | 0.997 | 20+30 | 1.483–1.932 | fail |
| allocating | view-none-tiny-s4 | 1.527 | 1.266 | 20+30 | 1.414–1.998 | fail |
| allocating | vertex-v0-tiny-s12 | 2.365 | 1.641 | 6 | 1.700–2.979 | fail |
| allocating | view-none-tiny-s12 | 2.534 | 1.478 | 5 | 2.032–3.224 | fail |
| allocating | vertex-v0-tiny-s32 | 3.160 | 1.591 | 5 | 2.979–3.709 | fail |
| allocating | view-none-tiny-s32 | 3.624 | 1.620 | 5 | 2.506–4.571 | fail |
| allocating | view-1-tiny-s4 | 0.911 | 0.996 | 20+30 | 1.269–1.706 | fail |
| allocating | view-1-tiny-s8 | 1.348 | 0.979 | 9 | 1.174–1.562 | pass |
| allocating | view-2-tiny-s8 | 1.466 | 1.090 | 20+30 | 1.296–1.748 | fail |
| allocating | view-3-tiny-s12 | 1.599 | 1.040 | 20+30 | 1.494–1.631 | fail |
| allocating | index-2-v0-tiny-s2 | 0.529 | 1.059 | 8 | 0.335–1.461 | pass |
| allocating | index-2-v0-tiny-s4 | 1.028 | 0.991 | 7 | 0.407–1.461 | pass |
| allocating | index-2-v1-tiny-s2 | 1.099 | 1.099 | 5 | 0.911–1.167 | pass |
| allocating | index-2-v1-tiny-s4 | 1.056 | 1.040 | 5 | 0.890–1.119 | pass |
| allocating | index-3-v0-tiny-s2 | 0.874 | 0.967 | 8 | 0.291–1.383 | pass |
| allocating | index-3-v0-tiny-s4 | 0.998 | 0.901 | 5 | 0.763–1.532 | pass |
| allocating | index-3-v1-tiny-s2 | 0.938 | 0.891 | 6 | 0.734–1.422 | pass |
| allocating | index-3-v1-tiny-s4 | 1.038 | 0.918 | 5 | 0.785–1.313 | pass |
| allocating | vertex-v0-resident-s4 | 5.353 | 1.604 | 5 | 3.153–6.274 | fail |
| allocating | view-none-resident-s4 | 5.275 | 1.561 | 5 | 5.025–5.519 | fail |
| allocating | vertex-v0-resident-s12 | 5.503 | 1.602 | 5 | 3.545–6.633 | fail |
| allocating | view-none-resident-s12 | 5.430 | 1.625 | 5 | 4.749–5.983 | fail |
| allocating | vertex-v0-resident-s32 | 5.482 | 1.591 | 5 | 5.130–5.717 | fail |
| allocating | view-none-resident-s32 | 5.610 | 1.614 | 5 | 5.513–5.658 | fail |
| allocating | view-1-resident-s4 | 2.862 | 1.031 | 5 | 2.259–3.650 | fail |
| allocating | view-1-resident-s8 | 3.232 | 1.042 | 5 | 3.309–4.162 | fail |
| allocating | view-2-resident-s8 | 3.938 | 1.018 | 5 | 2.894–5.345 | fail |
| allocating | view-3-resident-s12 | 5.049 | 1.122 | 5 | 3.873–5.785 | fail |
| allocating | index-2-v0-resident-s2 | 1.409 | 1.178 | 11 | 0.990–1.571 | pass |
| allocating | index-2-v0-resident-s4 | 1.322 | 0.956 | 7 | 0.900–1.582 | pass |
| allocating | index-2-v1-resident-s2 | 1.177 | 1.023 | 5 | 1.043–1.241 | pass |
| allocating | index-2-v1-resident-s4 | 1.232 | 0.880 | 5 | 1.059–1.309 | pass |
| allocating | index-3-v0-resident-s2 | 1.084 | 1.036 | 5 | 0.982–1.124 | pass |
| allocating | index-3-v0-resident-s4 | 0.748 | 0.921 | 5 | 0.524–1.452 | pass |
| allocating | index-3-v1-resident-s2 | 1.008 | 0.986 | 5 | 0.883–1.101 | pass |
| allocating | index-3-v1-resident-s4 | 0.775 | 0.964 | 5 | 0.557–1.147 | pass |
| allocating | vertex-v0-streaming-s4 | 3.453 | 1.606 | 5 | 2.656–6.086 | fail |
| allocating | view-none-streaming-s4 | 5.123 | 1.602 | 5 | 2.843–6.889 | fail |
| allocating | vertex-v0-streaming-s12 | 5.248 | 1.576 | 5 | 3.672–6.603 | fail |
| allocating | view-none-streaming-s12 | 5.035 | 1.588 | 5 | 4.545–5.503 | fail |
| allocating | vertex-v0-streaming-s32 | 3.397 | 1.462 | 5 | 3.035–3.627 | fail |
| allocating | view-none-streaming-s32 | 3.467 | 1.468 | 5 | 3.036–3.717 | fail |
| allocating | view-1-streaming-s4 | 3.086 | 0.970 | 5 | 2.127–3.874 | fail |
| allocating | view-1-streaming-s8 | 4.063 | 1.011 | 5 | 4.004–4.185 | fail |
| allocating | view-2-streaming-s8 | 3.850 | 1.004 | 5 | 3.605–4.019 | fail |
| allocating | view-3-streaming-s12 | 2.849 | 1.098 | 5 | 2.367–3.145 | fail |
| allocating | index-2-v0-streaming-s2 | 1.026 | 1.000 | 5 | 1.016–1.045 | pass |
| allocating | index-2-v0-streaming-s4 | 0.999 | 1.023 | 6 | 0.671–1.447 | pass |
| allocating | index-2-v1-streaming-s2 | 1.266 | 0.955 | 7 | 1.064–1.566 | pass |
| allocating | index-2-v1-streaming-s4 | 1.314 | 0.997 | 5 | 1.098–1.443 | pass |
| allocating | index-3-v0-streaming-s2 | 1.025 | 1.002 | 5 | 0.859–1.137 | pass |
| allocating | index-3-v0-streaming-s4 | 1.054 | 0.991 | 5 | 0.700–1.553 | pass |
| allocating | index-3-v1-streaming-s2 | 1.017 | 0.996 | 5 | 0.626–1.444 | pass |
| allocating | index-3-v1-streaming-s4 | 1.064 | 0.994 | 5 | 0.752–1.490 | pass |
| allocating | vertex-v1-tiny-s4 | 1.722 | 1.312 | 20+30 | 1.375–1.682 | fail |
| allocating | vertex-v1-tiny-s12 | 2.357 | 1.419 | 5 | 1.936–2.606 | fail |
| allocating | vertex-v1-tiny-s32 | 3.292 | 1.532 | 5 | 3.066–4.007 | fail |
| allocating | vertex-v1-resident-s4 | 5.283 | 1.554 | 5 | 3.890–6.024 | fail |
| allocating | vertex-v1-resident-s12 | 5.547 | 1.622 | 5 | 5.225–5.919 | fail |
| allocating | vertex-v1-resident-s32 | 5.600 | 1.603 | 5 | 5.133–6.002 | fail |
| allocating | vertex-v1-streaming-s4 | 5.070 | 1.616 | 5 | 3.197–6.534 | fail |
| allocating | vertex-v1-streaming-s12 | 3.432 | 1.514 | 5 | 2.780–5.116 | fail |
| allocating | vertex-v1-streaming-s32 | 3.565 | 1.484 | 5 | 3.101–3.761 | fail |
| allocating | vertex-v0-resident-s12-level0 | 5.513 | 1.618 | 5 | 4.847–5.862 | fail |
| allocating | vertex-v0-resident-s12-level9 | 5.401 | 1.586 | 5 | 5.173–5.713 | fail |
| allocating | vertex-v1-resident-s12-level0 | 5.227 | 1.584 | 5 | 4.983–5.619 | fail |
| allocating | vertex-v1-resident-s12-level9 | 5.577 | 1.629 | 5 | 3.505–7.017 | fail |
| allocating | index-2-v0-millions-s4 | 1.015 | 1.035 | 6 | 0.851–1.582 | pass |
| allocating | index-2-v1-millions-s4 | 1.497 | 0.988 | 9 | 1.305–1.599 | pass |
| allocating | varied-view-1-tiny-s4 | 1.410 | 0.840 | 12 | 0.993–1.584 | pass |
| allocating | varied-view-1-tiny-s8 | 2.187 | 0.979 | 9 | 1.603–2.644 | fail |
| allocating | varied-view-2-tiny-s8 | 2.185 | 1.045 | 5 | 2.032–2.656 | fail |
| allocating | varied-view-3-tiny-s4 | 1.616 | 1.204 | 20+30 | 1.294–1.917 | fail |
| allocating | varied-view-3-tiny-s12 | 1.967 | 1.293 | 5 | 1.606–2.896 | fail |
| allocating | varied-view-3-tiny-s32 | 2.157 | 1.256 | 5 | 2.193–3.152 | fail |
| allocating | varied-view-1-resident-s4 | 4.478 | 0.495 | 5 | 3.847–4.964 | fail |
| allocating | varied-view-1-resident-s8 | 6.732 | 0.974 | 5 | 5.612–6.643 | fail |
| allocating | varied-view-2-resident-s8 | 5.596 | 0.939 | 5 | 4.780–7.231 | fail |
| allocating | varied-view-3-resident-s4 | 5.610 | 1.380 | 5 | 5.152–7.220 | fail |
| allocating | varied-view-3-resident-s12 | 5.279 | 1.416 | 5 | 4.039–6.513 | fail |
| allocating | varied-view-3-resident-s32 | 5.767 | 1.600 | 5 | 4.825–6.107 | fail |
| allocating | varied-view-1-streaming-s4 | 3.934 | 0.457 | 5 | 2.486–6.162 | fail |
| allocating | varied-view-1-streaming-s8 | 5.427 | 0.600 | 5 | 4.798–6.420 | fail |
| allocating | varied-view-2-streaming-s8 | 6.405 | 0.738 | 5 | 4.020–7.918 | fail |
| allocating | varied-view-3-streaming-s4 | 4.929 | 1.419 | 5 | 4.562–5.200 | fail |
| allocating | varied-view-3-streaming-s12 | 4.583 | 1.075 | 5 | 3.662–5.302 | fail |
| allocating | varied-view-3-streaming-s32 | 4.753 | 1.125 | 5 | 4.196–5.277 | fail |
| caller-buffer | vertex-v0-tiny-s4 | 1.524 | 1.238 | 20+30 | 1.939–2.280 | fail |
| caller-buffer | view-none-tiny-s4 | 1.527 | 1.110 | 7 | 1.652–2.116 | fail |
| caller-buffer | vertex-v0-tiny-s12 | 2.476 | 1.537 | 5 | 2.084–3.681 | fail |
| caller-buffer | view-none-tiny-s12 | 3.518 | 1.723 | 5 | 3.303–3.747 | fail |
| caller-buffer | vertex-v0-tiny-s32 | 4.282 | 1.854 | 5 | 4.174–4.406 | fail |
| caller-buffer | view-none-tiny-s32 | 4.268 | 1.860 | 5 | 3.946–4.608 | fail |
| caller-buffer | view-1-tiny-s4 | 1.237 | 1.060 | 5 | 0.829–1.466 | pass |
| caller-buffer | view-1-tiny-s8 | 1.278 | 0.929 | 20+30 | 1.450–1.873 | fail |
| caller-buffer | view-2-tiny-s8 | 1.654 | 1.072 | 20+30 | 1.555–1.685 | fail |
| caller-buffer | view-3-tiny-s12 | 1.916 | 1.153 | 5 | 1.873–2.015 | fail |
| caller-buffer | index-2-v0-tiny-s2 | 0.891 | 0.909 | 5 | 0.857–0.914 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 0.867 | 0.968 | 5 | 0.696–1.268 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 0.891 | 0.964 | 5 | 0.790–0.941 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 0.878 | 0.945 | 5 | 0.756–1.191 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.829 | 0.949 | 5 | 0.626–1.147 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.850 | 0.977 | 5 | 0.687–1.032 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.852 | 0.992 | 5 | 0.605–1.077 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 1.225 | 1.366 | 5 | 0.724–1.277 | pass |
| caller-buffer | vertex-v0-resident-s4 | 6.076 | 1.279 | 5 | 4.901–8.321 | fail |
| caller-buffer | view-none-resident-s4 | 5.683 | 1.518 | 5 | 5.324–7.252 | fail |
| caller-buffer | vertex-v0-resident-s12 | 7.281 | 1.517 | 5 | 4.798–8.629 | fail |
| caller-buffer | view-none-resident-s12 | 6.098 | 1.772 | 5 | 5.356–6.562 | fail |
| caller-buffer | vertex-v0-resident-s32 | 5.893 | 1.604 | 5 | 5.236–6.505 | fail |
| caller-buffer | view-none-resident-s32 | 5.954 | 1.605 | 5 | 5.823–6.019 | fail |
| caller-buffer | view-1-resident-s4 | 3.397 | 0.986 | 5 | 3.340–3.527 | fail |
| caller-buffer | view-1-resident-s8 | 4.498 | 1.008 | 5 | 4.419–4.665 | fail |
| caller-buffer | view-2-resident-s8 | 4.593 | 1.135 | 5 | 2.987–6.249 | fail |
| caller-buffer | view-3-resident-s12 | 4.777 | 1.139 | 5 | 4.241–6.676 | fail |
| caller-buffer | index-2-v0-resident-s2 | 1.610 | 1.583 | 5 | 0.977–1.551 | pass |
| caller-buffer | index-2-v0-resident-s4 | 1.272 | 1.149 | 8 | 0.938–1.522 | pass |
| caller-buffer | index-2-v1-resident-s2 | 1.191 | 1.005 | 5 | 1.000–1.261 | pass |
| caller-buffer | index-2-v1-resident-s4 | 1.332 | 1.182 | 5 | 1.102–1.411 | pass |
| caller-buffer | index-3-v0-resident-s2 | 0.963 | 0.997 | 5 | 0.873–1.044 | pass |
| caller-buffer | index-3-v0-resident-s4 | 1.039 | 0.943 | 8 | 0.774–1.481 | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.999 | 0.962 | 5 | 0.951–1.023 | pass |
| caller-buffer | index-3-v1-resident-s4 | 0.853 | 1.035 | 5 | 0.865–1.262 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 5.498 | 1.594 | 5 | 5.315–5.547 | fail |
| caller-buffer | view-none-streaming-s4 | 5.249 | 1.519 | 5 | 5.154–5.691 | fail |
| caller-buffer | vertex-v0-streaming-s12 | 5.112 | 1.575 | 5 | 4.864–6.015 | fail |
| caller-buffer | view-none-streaming-s12 | 5.443 | 1.634 | 5 | 5.177–5.646 | fail |
| caller-buffer | vertex-v0-streaming-s32 | 4.919 | 1.620 | 5 | 4.639–5.962 | fail |
| caller-buffer | view-none-streaming-s32 | 5.286 | 1.591 | 5 | 5.058–5.383 | fail |
| caller-buffer | view-1-streaming-s4 | 3.361 | 1.009 | 5 | 3.185–3.669 | fail |
| caller-buffer | view-1-streaming-s8 | 4.140 | 1.022 | 5 | 3.894–4.289 | fail |
| caller-buffer | view-2-streaming-s8 | 4.069 | 1.004 | 5 | 3.937–4.128 | fail |
| caller-buffer | view-3-streaming-s12 | 4.970 | 1.130 | 5 | 4.769–5.231 | fail |
| caller-buffer | index-2-v0-streaming-s2 | 0.957 | 1.013 | 5 | 0.949–1.031 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 0.957 | 0.982 | 5 | 0.943–0.999 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 1.256 | 0.996 | 5 | 1.201–1.295 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 1.244 | 0.995 | 5 | 1.182–1.281 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 0.984 | 0.991 | 5 | 0.975–0.997 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 1.002 | 1.002 | 5 | 0.967–1.083 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.996 | 0.987 | 5 | 0.952–1.028 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 1.016 | 1.007 | 5 | 0.929–1.046 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 1.839 | 1.771 | 20+30 | 1.544–2.227 | fail |
| caller-buffer | vertex-v1-tiny-s12 | 2.956 | 1.639 | 5 | 2.270–3.550 | fail |
| caller-buffer | vertex-v1-tiny-s32 | 4.270 | 1.760 | 5 | 2.238–6.345 | fail |
| caller-buffer | vertex-v1-resident-s4 | 5.854 | 1.637 | 5 | 5.777–5.918 | fail |
| caller-buffer | vertex-v1-resident-s12 | 5.886 | 1.640 | 5 | 5.458–6.915 | fail |
| caller-buffer | vertex-v1-resident-s32 | 6.007 | 1.644 | 5 | 5.863–6.155 | fail |
| caller-buffer | vertex-v1-streaming-s4 | 5.488 | 1.611 | 5 | 5.244–5.724 | fail |
| caller-buffer | vertex-v1-streaming-s12 | 5.448 | 1.610 | 5 | 5.183–5.692 | fail |
| caller-buffer | vertex-v1-streaming-s32 | 5.611 | 1.639 | 5 | 4.673–5.841 | fail |
| caller-buffer | vertex-v0-resident-s12-level0 | 6.089 | 1.589 | 5 | 4.553–7.304 | fail |
| caller-buffer | vertex-v0-resident-s12-level9 | 6.207 | 1.708 | 5 | 5.776–6.431 | fail |
| caller-buffer | vertex-v1-resident-s12-level0 | 5.903 | 1.670 | 5 | 5.805–6.028 | fail |
| caller-buffer | vertex-v1-resident-s12-level9 | 6.234 | 1.606 | 5 | 5.407–6.773 | fail |
| caller-buffer | index-2-v0-millions-s4 | 0.971 | 1.018 | 5 | 0.959–0.991 | pass |
| caller-buffer | index-2-v1-millions-s4 | 1.180 | 0.997 | 5 | 1.132–1.282 | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.696 | 0.767 | 20+30 | 1.411–1.842 | fail |
| caller-buffer | varied-view-1-tiny-s8 | 2.364 | 0.965 | 7 | 1.692–2.673 | fail |
| caller-buffer | varied-view-2-tiny-s8 | 2.737 | 1.021 | 5 | 1.854–3.318 | fail |
| caller-buffer | varied-view-3-tiny-s4 | 1.784 | 1.388 | 19 | 1.634–2.799 | fail |
| caller-buffer | varied-view-3-tiny-s12 | 2.733 | 1.368 | 5 | 2.516–3.041 | fail |
| caller-buffer | varied-view-3-tiny-s32 | 3.512 | 1.410 | 5 | 2.580–6.099 | fail |
| caller-buffer | varied-view-1-resident-s4 | 5.747 | 0.522 | 5 | 3.999–7.576 | fail |
| caller-buffer | varied-view-1-resident-s8 | 6.678 | 0.780 | 5 | 6.450–7.166 | fail |
| caller-buffer | varied-view-2-resident-s8 | 6.976 | 0.895 | 5 | 6.723–7.441 | fail |
| caller-buffer | varied-view-3-resident-s4 | 6.531 | 1.474 | 5 | 5.342–7.881 | fail |
| caller-buffer | varied-view-3-resident-s12 | 6.268 | 1.431 | 5 | 3.818–9.082 | fail |
| caller-buffer | varied-view-3-resident-s32 | 5.952 | 1.222 | 5 | 3.749–7.518 | fail |
| caller-buffer | varied-view-1-streaming-s4 | 5.197 | 0.397 | 5 | 4.717–5.684 | fail |
| caller-buffer | varied-view-1-streaming-s8 | 6.642 | 0.680 | 5 | 5.898–9.572 | fail |
| caller-buffer | varied-view-2-streaming-s8 | 6.871 | 0.734 | 5 | 6.315–7.294 | fail |
| caller-buffer | varied-view-3-streaming-s4 | 6.567 | 1.423 | 5 | 5.118–9.989 | fail |
| caller-buffer | varied-view-3-streaming-s12 | 6.172 | 1.143 | 5 | 5.599–6.241 | fail |
| caller-buffer | varied-view-3-streaming-s32 | 5.798 | 1.214 | 5 | 5.325–6.043 | fail |
