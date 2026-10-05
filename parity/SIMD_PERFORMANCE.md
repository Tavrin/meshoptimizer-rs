# P07 individual SIMD performance

Raw native evidence: `/mnt/linux-extra/meshopt-artifacts/p07perf/performance.json`; raw Node evidence: `/mnt/linux-extra/meshopt-artifacts/p07perf/wasm-performance.json`.

Time ratios: lower is faster. Family means use stage 1; only borderline case maxima use fresh D146 intervals. Repeated standalone filters are reported separately, outside the SIMD filter bar. See SIMD_RESULTS.md for shared-host and release limits.

| API | Family | Cases | Time Rust/C++ SIMD GM | Worst stage-1 | Time Rust/scalar Rust GM |
|---|---|---:|---:|---:|---:|
| allocating | color | 6 | 1.508 | 1.716 | 0.716 |
| allocating | exp | 12 | 0.950 | 1.232 | 1.011 |
| allocating | index | 14 | 1.161 | 1.674 | 1.072 |
| allocating | meshlet | 12 | 1.901 | 2.401 | 0.689 |
| allocating | meshlet-raw | 3 | 1.726 | 2.062 | 0.678 |
| allocating | oct | 6 | 1.538 | 1.762 | 0.164 |
| allocating | oct (repeated) | 6 | 1.041 | 1.342 | 1.270 |
| allocating | quat | 3 | 1.349 | 1.407 | 0.155 |
| allocating | quat (repeated) | 3 | 1.159 | 1.280 | 1.044 |
| allocating | sequence | 12 | 1.268 | 1.356 | 1.083 |
| allocating | vertex | 22 | 2.065 | 3.080 | 0.655 |
| allocating | view-filtered | 30 | 1.799 | 2.939 | 0.477 |
| allocating | view-none | 9 | 1.994 | 2.850 | 0.711 |
| caller-buffer | color | 6 | 1.547 | 1.686 | 0.617 |
| caller-buffer | exp | 12 | 0.947 | 1.040 | 1.004 |
| caller-buffer | index | 14 | 1.030 | 1.337 | 0.982 |
| caller-buffer | meshlet | 12 | 2.401 | 2.615 | 0.714 |
| caller-buffer | meshlet-raw | 3 | 2.031 | 2.326 | 0.630 |
| caller-buffer | oct | 6 | 1.557 | 1.767 | 0.135 |
| caller-buffer | oct (repeated) | 6 | 1.007 | 1.151 | 1.338 |
| caller-buffer | quat | 3 | 1.339 | 1.393 | 0.136 |
| caller-buffer | quat (repeated) | 3 | 1.147 | 1.191 | 1.056 |
| caller-buffer | sequence | 12 | 1.159 | 1.264 | 1.132 |
| caller-buffer | vertex | 22 | 2.115 | 2.792 | 0.564 |
| caller-buffer | view-filtered | 30 | 1.690 | 2.189 | 0.425 |
| caller-buffer | view-none | 9 | 2.234 | 2.813 | 0.649 |

## Native individual cases

| API | Case | Rust MB/s | C++ SIMD MB/s | Time Rust/C++ SIMD | Time Rust/scalar | Pairs | Final interval | Case bar | S3 |
|---|---|---:|---:|---:|---:|---:|---|---|---|
| allocating | vertex-v0-tiny-s4 | 286.324 | 800.257 | 2.795 | 1.271 | 5 | 2.192–2.788 | fail | FAIL |
| allocating | view-none-tiny-s4 | 228.617 | 480.521 | 2.102 | 1.038 | 5 | 1.944–2.720 | fail | pass |
| allocating | vertex-v0-tiny-s12 | 272.256 | 838.573 | 3.080 | 1.499 | 5 | 2.547–3.255 | fail | FAIL |
| allocating | view-none-tiny-s12 | 142.948 | 384.543 | 2.690 | 1.256 | 5 | 2.576–2.850 | fail | FAIL |
| allocating | vertex-v0-tiny-s32 | 148.471 | 392.280 | 2.642 | 1.204 | 5 | 2.561–2.751 | fail | FAIL |
| allocating | view-none-tiny-s32 | 154.018 | 398.012 | 2.584 | 1.164 | 5 | 2.480–2.716 | fail | FAIL |
| allocating | filter-1-tiny-s4 | 229.653 | 308.110 | 1.342 | 1.289 | 5 | 1.120–1.403 | pass | n/a |
| allocating | view-1-tiny-s4 | 103.895 | 174.852 | 1.683 | 1.170 | 5 | 1.648–1.702 | fail | FAIL |
| allocating | filter-1-tiny-s8 | 612.768 | 660.253 | 1.077 | 1.281 | 5 | 1.064–1.088 | pass | n/a |
| allocating | view-1-tiny-s8 | 126.423 | 221.266 | 1.750 | 1.216 | 5 | 1.692–1.839 | fail | FAIL |
| allocating | filter-2-tiny-s8 | 604.279 | 773.189 | 1.280 | 1.075 | 5 | 1.232–1.311 | pass | n/a |
| allocating | view-2-tiny-s8 | 142.932 | 263.156 | 1.841 | 1.216 | 5 | 1.821–1.894 | fail | FAIL |
| allocating | filter-3-tiny-s12 | 1959.653 | 2415.017 | 1.232 | 1.302 | 5 | 1.217–1.256 | pass | FAIL |
| allocating | view-3-tiny-s12 | 153.024 | 327.828 | 2.142 | 1.300 | 5 | 2.089–2.139 | fail | FAIL |
| allocating | index-2-v0-tiny-s2 | 221.480 | 251.075 | 1.134 | 0.972 | 5 | 1.050–1.195 | pass | n/a |
| allocating | index-2-v0-tiny-s4 | 421.785 | 489.027 | 1.159 | 0.962 | 5 | 1.084–1.175 | pass | n/a |
| allocating | index-2-v1-tiny-s2 | 224.676 | 252.025 | 1.122 | 0.989 | 5 | 1.087–1.145 | pass | n/a |
| allocating | index-2-v1-tiny-s4 | 485.493 | 569.986 | 1.174 | 0.971 | 5 | 1.106–1.208 | pass | n/a |
| allocating | index-3-v0-tiny-s2 | 276.642 | 367.574 | 1.329 | 1.049 | 5 | 1.122–1.456 | pass | n/a |
| allocating | index-3-v0-tiny-s4 | 542.221 | 735.463 | 1.356 | 1.056 | 5 | 1.327–1.374 | pass | n/a |
| allocating | index-3-v1-tiny-s2 | 261.945 | 344.733 | 1.316 | 1.028 | 5 | 1.317–1.360 | pass | n/a |
| allocating | index-3-v1-tiny-s4 | 581.817 | 788.769 | 1.356 | 1.050 | 5 | 1.328–1.377 | pass | n/a |
| allocating | vertex-v0-resident-s4 | 771.790 | 1479.140 | 1.917 | 0.437 | 5 | 1.900–2.005 | fail | pass |
| allocating | view-none-resident-s4 | 519.596 | 1480.984 | 2.850 | 0.451 | 5 | 1.716–2.602 | fail | pass |
| allocating | vertex-v0-resident-s12 | 501.544 | 1024.386 | 2.042 | 0.503 | 5 | 2.099–2.471 | fail | pass |
| allocating | view-none-resident-s12 | 452.855 | 918.308 | 2.028 | 0.488 | 5 | 1.949–2.471 | fail | pass |
| allocating | vertex-v0-resident-s32 | 371.187 | 738.354 | 1.989 | 0.471 | 5 | 1.869–2.314 | fail | pass |
| allocating | view-none-resident-s32 | 394.279 | 938.228 | 2.380 | 0.561 | 5 | 1.766–2.554 | fail | pass |
| allocating | filter-1-resident-s4 | 959.540 | 898.959 | 0.937 | 1.126 | 5 | 0.909–0.979 | pass | n/a |
| allocating | view-1-resident-s4 | 630.477 | 672.136 | 1.066 | 0.400 | 5 | 1.020–1.125 | pass | pass |
| allocating | filter-1-resident-s8 | 1384.188 | 1335.900 | 0.965 | 1.822 | 5 | 0.921–1.125 | pass | n/a |
| allocating | view-1-resident-s8 | 745.758 | 1015.493 | 1.362 | 0.444 | 6 | 1.331–1.495 | pass | pass |
| allocating | filter-2-resident-s8 | 1629.399 | 1921.267 | 1.179 | 1.065 | 5 | 1.059–1.207 | pass | n/a |
| allocating | view-2-resident-s8 | 627.849 | 1022.805 | 1.629 | 0.384 | 5 | 1.502–1.662 | fail | pass |
| allocating | filter-3-resident-s12 | 7134.757 | 5997.941 | 0.841 | 0.970 | 5 | 0.792–0.876 | pass | pass |
| allocating | view-3-resident-s12 | 908.599 | 1667.168 | 1.835 | 0.343 | 5 | 1.802–1.881 | fail | pass |
| allocating | index-2-v0-resident-s2 | 878.460 | 864.295 | 0.984 | 0.973 | 5 | 0.928–0.976 | pass | n/a |
| allocating | index-2-v0-resident-s4 | 1403.685 | 1347.408 | 0.960 | 0.938 | 5 | 0.837–0.978 | pass | n/a |
| allocating | index-2-v1-resident-s2 | 882.284 | 881.604 | 0.999 | 1.001 | 5 | 0.813–1.320 | pass | n/a |
| allocating | index-2-v1-resident-s4 | 1558.068 | 1512.399 | 0.971 | 0.965 | 5 | 0.856–0.978 | pass | n/a |
| allocating | index-3-v0-resident-s2 | 618.379 | 761.320 | 1.231 | 1.079 | 5 | 1.190–1.324 | pass | n/a |
| allocating | index-3-v0-resident-s4 | 1576.737 | 1883.861 | 1.195 | 1.110 | 5 | 1.172–1.284 | pass | n/a |
| allocating | index-3-v1-resident-s2 | 942.612 | 1173.907 | 1.245 | 1.114 | 5 | 1.218–1.270 | pass | n/a |
| allocating | index-3-v1-resident-s4 | 1915.969 | 2374.435 | 1.239 | 1.102 | 5 | 1.237–1.244 | pass | n/a |
| allocating | vertex-v0-streaming-s4 | 559.402 | 705.546 | 1.261 | 0.452 | 15 | 1.303–1.428 | fail | pass |
| allocating | view-none-streaming-s4 | 683.612 | 945.107 | 1.383 | 0.593 | 8 | 1.316–1.475 | fail | pass |
| allocating | vertex-v0-streaming-s12 | 769.718 | 1226.628 | 1.594 | 0.623 | 5 | 1.442–1.831 | fail | pass |
| allocating | view-none-streaming-s12 | 976.124 | 1348.429 | 1.381 | 0.604 | 20+30 | 1.428–1.630 | fail | pass |
| allocating | vertex-v0-streaming-s32 | 850.590 | 1166.395 | 1.371 | 0.641 | 20+30 | 1.335–1.510 | fail | pass |
| allocating | view-none-streaming-s32 | 764.801 | 994.207 | 1.300 | 0.691 | 20+30 | 1.341–1.516 | fail | pass |
| allocating | filter-1-streaming-s4 | 1601.773 | 1602.923 | 1.001 | 1.075 | 5 | 0.990–1.042 | pass | n/a |
| allocating | view-1-streaming-s4 | 1290.271 | 2668.679 | 2.068 | 0.542 | 5 | 2.037–2.085 | fail | pass |
| allocating | filter-1-streaming-s8 | 1783.022 | 1737.145 | 0.974 | 1.154 | 5 | 0.909–1.014 | pass | n/a |
| allocating | view-1-streaming-s8 | 1265.933 | 1526.637 | 1.206 | 0.591 | 5 | 1.189–1.217 | pass | pass |
| allocating | filter-2-streaming-s8 | 1861.567 | 1921.953 | 1.032 | 0.993 | 5 | 0.959–1.066 | pass | n/a |
| allocating | view-2-streaming-s8 | 1189.282 | 1529.715 | 1.286 | 0.550 | 5 | 1.142–1.471 | pass | pass |
| allocating | filter-3-streaming-s12 | 1174.063 | 1185.272 | 1.010 | 1.019 | 5 | 0.939–1.030 | pass | pass |
| allocating | view-3-streaming-s12 | 1396.472 | 1842.816 | 1.320 | 0.508 | 5 | 1.220–1.396 | pass | pass |
| allocating | index-2-v0-streaming-s2 | 1062.488 | 1374.867 | 1.294 | 1.066 | 5 | 1.219–1.307 | pass | n/a |
| allocating | index-2-v0-streaming-s4 | 2091.735 | 2691.253 | 1.287 | 1.056 | 5 | 1.267–1.303 | pass | n/a |
| allocating | index-2-v1-streaming-s2 | 1225.884 | 2038.260 | 1.663 | 1.697 | 7 | 1.535–1.811 | fail | n/a |
| allocating | index-2-v1-streaming-s4 | 2289.640 | 3832.926 | 1.674 | 1.738 | 5 | 1.614–1.774 | fail | n/a |
| allocating | index-3-v0-streaming-s2 | 1838.443 | 2248.616 | 1.223 | 1.100 | 5 | 0.801–1.389 | pass | n/a |
| allocating | index-3-v0-streaming-s4 | 3446.798 | 4308.484 | 1.250 | 1.108 | 5 | 1.217–1.281 | pass | n/a |
| allocating | index-3-v1-streaming-s2 | 1807.237 | 2210.900 | 1.223 | 1.104 | 5 | 1.219–1.243 | pass | n/a |
| allocating | index-3-v1-streaming-s4 | 3239.150 | 4116.104 | 1.271 | 1.106 | 5 | 1.192–1.290 | pass | n/a |
| allocating | vertex-v1-tiny-s4 | 379.083 | 924.628 | 2.439 | 1.038 | 5 | 2.350–2.563 | fail | pass |
| allocating | vertex-v1-tiny-s12 | 454.554 | 1234.715 | 2.716 | 1.235 | 5 | 2.705–2.792 | fail | FAIL |
| allocating | vertex-v1-tiny-s32 | 552.972 | 1480.951 | 2.678 | 1.184 | 5 | 2.680–2.704 | fail | FAIL |
| allocating | vertex-v1-resident-s4 | 1990.853 | 3873.803 | 1.946 | 0.442 | 5 | 1.923–1.959 | fail | pass |
| allocating | vertex-v1-resident-s12 | 1734.773 | 3989.658 | 2.300 | 0.507 | 5 | 2.286–2.326 | fail | pass |
| allocating | vertex-v1-resident-s32 | 1796.078 | 4062.830 | 2.262 | 0.508 | 5 | 2.269–2.309 | fail | pass |
| allocating | vertex-v1-streaming-s4 | 2043.997 | 3782.895 | 1.851 | 0.437 | 5 | 1.849–1.921 | fail | pass |
| allocating | vertex-v1-streaming-s12 | 1054.374 | 1479.775 | 1.403 | 0.632 | 5 | 1.364–1.425 | fail | pass |
| allocating | vertex-v1-streaming-s32 | 978.812 | 1315.604 | 1.344 | 0.632 | 6 | 1.307–1.395 | fail | pass |
| allocating | vertex-v0-resident-s12-level0 | 1796.934 | 4129.859 | 2.298 | 0.511 | 5 | 2.266–2.359 | fail | pass |
| allocating | vertex-v0-resident-s12-level9 | 1775.346 | 4045.773 | 2.279 | 0.515 | 5 | 2.265–2.304 | fail | pass |
| allocating | vertex-v1-resident-s12-level0 | 1773.521 | 4065.802 | 2.293 | 0.514 | 5 | 2.277–2.312 | fail | pass |
| allocating | vertex-v1-resident-s12-level9 | 1803.011 | 4139.090 | 2.296 | 0.511 | 5 | 2.286–2.300 | fail | pass |
| allocating | index-2-v0-millions-s4 | 1180.419 | 1317.281 | 1.116 | 1.021 | 5 | 1.100–1.150 | pass | n/a |
| allocating | index-2-v1-millions-s4 | 1610.537 | 1581.424 | 0.982 | 0.985 | 5 | 0.954–1.016 | pass | n/a |
| allocating | varied-filter-1-tiny-s4 | 885.301 | 1451.322 | 1.639 | 0.313 | 5 | 1.628–1.663 | fail | pass |
| allocating | varied-view-1-tiny-s4 | 356.457 | 663.949 | 1.863 | 0.518 | 5 | 1.835–1.885 | fail | pass |
| allocating | varied-filter-1-tiny-s8 | 1759.688 | 2329.249 | 1.324 | 0.298 | 5 | 1.298–1.333 | pass | pass |
| allocating | varied-view-1-tiny-s8 | 444.827 | 846.815 | 1.904 | 0.692 | 5 | 1.894–1.910 | fail | pass |
| allocating | varied-filter-2-tiny-s8 | 1916.109 | 2695.346 | 1.407 | 0.288 | 5 | 1.397–1.421 | pass | pass |
| allocating | varied-view-2-tiny-s8 | 432.612 | 916.062 | 2.118 | 0.707 | 5 | 2.098–2.147 | fail | pass |
| allocating | varied-filter-3-tiny-s4 | 3739.138 | 3168.948 | 0.848 | 0.991 | 5 | 0.805–0.876 | pass | pass |
| allocating | varied-view-3-tiny-s4 | 451.555 | 897.715 | 1.988 | 1.047 | 5 | 1.986–1.998 | fail | FAIL |
| allocating | varied-filter-3-tiny-s12 | 8776.329 | 8618.082 | 0.982 | 0.979 | 5 | 0.902–1.212 | pass | pass |
| allocating | varied-view-3-tiny-s12 | 525.244 | 1238.295 | 2.358 | 1.220 | 5 | 2.327–2.373 | fail | FAIL |
| allocating | varied-filter-3-tiny-s32 | 16404.277 | 15730.402 | 0.959 | 0.984 | 5 | 0.945–0.972 | pass | pass |
| allocating | varied-view-3-tiny-s32 | 580.580 | 1404.318 | 2.419 | 1.110 | 5 | 2.383–2.431 | fail | FAIL |
| allocating | varied-filter-4-tiny-s4 | 1347.764 | 1828.645 | 1.357 | 0.779 | 5 | 1.351–1.384 | pass | pass |
| allocating | varied-filter-4-tiny-s8 | 2263.677 | 3023.904 | 1.336 | 0.795 | 5 | 1.291–1.343 | pass | pass |
| allocating | varied-filter-1-resident-s4 | 2014.132 | 3549.359 | 1.762 | 0.137 | 5 | 1.506–1.889 | fail | pass |
| allocating | varied-view-1-resident-s4 | 1314.150 | 2319.487 | 1.765 | 0.168 | 5 | 1.741–1.828 | fail | pass |
| allocating | varied-filter-1-resident-s8 | 3716.213 | 5299.234 | 1.426 | 0.149 | 5 | 1.418–1.431 | pass | pass |
| allocating | varied-view-1-resident-s8 | 1439.792 | 2684.337 | 1.864 | 0.239 | 5 | 1.826–1.872 | fail | pass |
| allocating | varied-filter-2-resident-s8 | 4094.723 | 5557.416 | 1.357 | 0.127 | 5 | 1.319–1.390 | pass | pass |
| allocating | varied-view-2-resident-s8 | 1260.789 | 2311.449 | 1.833 | 0.225 | 5 | 1.697–1.968 | fail | pass |
| allocating | varied-filter-3-resident-s4 | 17139.255 | 15658.318 | 0.914 | 0.877 | 5 | 0.872–1.029 | pass | pass |
| allocating | varied-view-3-resident-s4 | 2057.133 | 3462.402 | 1.683 | 0.400 | 5 | 1.682–1.765 | fail | pass |
| allocating | varied-filter-3-resident-s12 | 20274.848 | 17640.360 | 0.870 | 0.983 | 5 | 0.866–0.874 | pass | pass |
| allocating | varied-view-3-resident-s12 | 1936.991 | 3808.939 | 1.966 | 0.428 | 5 | 1.960–1.966 | fail | pass |
| allocating | varied-filter-3-resident-s32 | 20001.933 | 19019.929 | 0.951 | 1.044 | 5 | 0.811–1.065 | pass | pass |
| allocating | varied-view-3-resident-s32 | 1896.459 | 3819.800 | 2.014 | 0.418 | 5 | 1.998–2.027 | fail | pass |
| allocating | varied-filter-4-resident-s4 | 2664.030 | 4510.125 | 1.693 | 0.663 | 5 | 1.689–1.713 | fail | pass |
| allocating | varied-filter-4-resident-s8 | 4882.202 | 7353.046 | 1.506 | 0.687 | 20+30 | 1.498–1.503 | borderline | pass |
| allocating | varied-filter-1-streaming-s4 | 2096.200 | 3646.730 | 1.740 | 0.098 | 5 | 1.721–1.793 | fail | pass |
| allocating | varied-view-1-streaming-s4 | 1441.416 | 2489.433 | 1.727 | 0.123 | 5 | 1.718–1.766 | fail | pass |
| allocating | varied-filter-1-streaming-s8 | 3936.673 | 5494.476 | 1.396 | 0.104 | 5 | 1.355–1.500 | pass | pass |
| allocating | varied-view-1-streaming-s8 | 1597.695 | 2960.804 | 1.853 | 0.174 | 5 | 1.835–1.865 | fail | pass |
| allocating | varied-filter-2-streaming-s8 | 5032.225 | 6468.452 | 1.285 | 0.102 | 5 | 1.297–1.375 | pass | pass |
| allocating | varied-view-2-streaming-s8 | 1674.643 | 3135.921 | 1.873 | 0.169 | 5 | 1.616–2.422 | fail | pass |
| allocating | varied-filter-3-streaming-s4 | 20859.604 | 19486.961 | 0.934 | 1.007 | 5 | 0.928–0.948 | pass | pass |
| allocating | varied-view-3-streaming-s4 | 2242.140 | 3884.378 | 1.732 | 0.382 | 5 | 1.715–1.763 | fail | pass |
| allocating | varied-filter-3-streaming-s12 | 20233.498 | 19112.883 | 0.945 | 1.014 | 5 | 0.906–0.983 | pass | FAIL |
| allocating | varied-view-3-streaming-s12 | 1839.992 | 3700.482 | 2.011 | 0.330 | 5 | 1.979–2.040 | fail | pass |
| allocating | varied-filter-3-streaming-s32 | 2534.472 | 2463.987 | 0.972 | 1.013 | 5 | 0.862–1.019 | pass | pass |
| allocating | varied-view-3-streaming-s32 | 1140.925 | 3353.130 | 2.939 | 0.452 | 5 | 2.909–3.097 | fail | pass |
| allocating | varied-filter-4-streaming-s4 | 2676.231 | 4591.209 | 1.716 | 0.685 | 5 | 1.700–1.726 | fail | pass |
| allocating | varied-filter-4-streaming-s8 | 4826.358 | 7169.432 | 1.485 | 0.700 | 20+30 | 1.453–1.480 | pass | pass |
| allocating | meshlet-1-1-v2-t3 | 115.113 | 165.971 | 1.442 | 1.122 | 5 | 1.393–1.498 | fail | FAIL |
| allocating | meshlet-1-1-v2-t4 | 138.635 | 206.777 | 1.492 | 1.136 | 5 | 1.488–1.509 | fail | FAIL |
| allocating | meshlet-1-1-v4-t3 | 158.647 | 235.647 | 1.485 | 1.145 | 5 | 1.472–1.493 | fail | FAIL |
| allocating | meshlet-1-1-v4-t4 | 182.139 | 273.685 | 1.503 | 1.151 | 5 | 1.485–1.538 | fail | pass |
| allocating | meshlet-raw-1-1 | 225.427 | 277.846 | 1.233 | 1.081 | 5 | 1.214–1.242 | pass | FAIL |
| allocating | meshlet-64-126-v2-t3 | 2230.433 | 5202.439 | 2.332 | 0.585 | 5 | 2.318–2.359 | fail | pass |
| allocating | meshlet-64-126-v2-t4 | 3183.659 | 6395.202 | 2.009 | 0.536 | 5 | 1.990–2.043 | fail | pass |
| allocating | meshlet-64-126-v4-t3 | 2898.463 | 6576.677 | 2.269 | 0.576 | 5 | 2.252–2.288 | fail | pass |
| allocating | meshlet-64-126-v4-t4 | 3971.731 | 7648.580 | 1.926 | 0.526 | 5 | 1.911–1.939 | fail | pass |
| allocating | meshlet-raw-64-126 | 3770.841 | 7631.795 | 2.024 | 0.556 | 5 | 1.982–2.069 | fail | pass |
| allocating | meshlet-256-256-v2-t3 | 2499.082 | 5999.291 | 2.401 | 0.547 | 5 | 2.383–2.424 | fail | pass |
| allocating | meshlet-256-256-v2-t4 | 3373.742 | 6885.358 | 2.041 | 0.506 | 5 | 2.036–2.079 | fail | pass |
| allocating | meshlet-256-256-v4-t3 | 3566.778 | 8225.384 | 2.306 | 0.528 | 5 | 2.253–2.342 | fail | pass |
| allocating | meshlet-256-256-v4-t4 | 4522.296 | 9087.867 | 2.010 | 0.491 | 5 | 1.973–2.044 | fail | pass |
| allocating | meshlet-raw-256-256 | 4296.558 | 8860.361 | 2.062 | 0.518 | 5 | 2.034–2.087 | fail | pass |
| caller-buffer | vertex-v0-tiny-s4 | 426.889 | 1171.757 | 2.745 | 1.213 | 5 | 2.652–2.808 | fail | FAIL |
| caller-buffer | view-none-tiny-s4 | 425.565 | 1159.905 | 2.726 | 1.201 | 5 | 2.729–2.766 | fail | FAIL |
| caller-buffer | vertex-v0-tiny-s12 | 516.544 | 1438.366 | 2.785 | 1.227 | 5 | 2.754–2.855 | fail | FAIL |
| caller-buffer | view-none-tiny-s12 | 508.689 | 1431.034 | 2.813 | 1.225 | 5 | 2.778–2.839 | fail | FAIL |
| caller-buffer | vertex-v0-tiny-s32 | 524.361 | 1418.448 | 2.705 | 1.198 | 5 | 2.690–2.746 | fail | FAIL |
| caller-buffer | view-none-tiny-s32 | 518.384 | 1410.127 | 2.720 | 1.203 | 5 | 2.665–2.745 | fail | FAIL |
| caller-buffer | filter-1-tiny-s4 | 1198.279 | 1379.719 | 1.151 | 1.096 | 8 | 1.184–1.491 | pass | n/a |
| caller-buffer | view-1-tiny-s4 | 401.678 | 675.018 | 1.680 | 1.146 | 5 | 1.655–1.714 | fail | FAIL |
| caller-buffer | filter-1-tiny-s8 | 2170.226 | 2185.603 | 1.007 | 1.338 | 5 | 0.998–1.022 | pass | n/a |
| caller-buffer | view-1-tiny-s8 | 501.858 | 883.133 | 1.760 | 1.268 | 5 | 1.735–1.784 | fail | FAIL |
| caller-buffer | filter-2-tiny-s8 | 2247.806 | 2678.179 | 1.191 | 1.093 | 5 | 1.182–1.217 | pass | n/a |
| caller-buffer | view-2-tiny-s8 | 531.120 | 991.011 | 1.866 | 1.212 | 5 | 1.794–1.943 | fail | FAIL |
| caller-buffer | filter-3-tiny-s12 | 10201.169 | 9641.169 | 0.945 | 1.029 | 5 | 0.916–0.960 | pass | pass |
| caller-buffer | view-3-tiny-s12 | 631.332 | 1323.112 | 2.096 | 1.285 | 5 | 2.087–2.101 | fail | FAIL |
| caller-buffer | index-2-v0-tiny-s2 | 1225.151 | 1386.172 | 1.131 | 0.981 | 5 | 1.113–1.145 | pass | n/a |
| caller-buffer | index-2-v0-tiny-s4 | 2217.528 | 2611.946 | 1.178 | 1.071 | 5 | 0.865–1.383 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s2 | 1173.824 | 1323.796 | 1.128 | 0.978 | 5 | 1.121–1.137 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s4 | 2167.353 | 2611.339 | 1.205 | 1.056 | 5 | 1.176–1.244 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s2 | 1173.971 | 1180.316 | 1.005 | 1.049 | 5 | 0.977–1.020 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s4 | 2259.606 | 2249.904 | 0.996 | 1.030 | 5 | 0.967–1.021 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s2 | 1166.454 | 1177.121 | 1.009 | 1.049 | 5 | 1.003–1.028 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s4 | 2460.937 | 2495.891 | 1.014 | 1.059 | 5 | 1.002–1.036 | pass | n/a |
| caller-buffer | vertex-v0-resident-s4 | 2054.747 | 4076.561 | 1.984 | 0.438 | 5 | 1.967–1.993 | fail | pass |
| caller-buffer | view-none-resident-s4 | 2046.210 | 4040.684 | 1.975 | 0.439 | 5 | 1.959–1.993 | fail | pass |
| caller-buffer | vertex-v0-resident-s12 | 1779.175 | 4158.170 | 2.337 | 0.508 | 5 | 2.328–2.341 | fail | pass |
| caller-buffer | view-none-resident-s12 | 1771.323 | 4132.700 | 2.333 | 0.509 | 5 | 2.326–2.341 | fail | pass |
| caller-buffer | vertex-v0-resident-s32 | 1715.222 | 4054.870 | 2.364 | 0.510 | 5 | 2.263–2.386 | fail | pass |
| caller-buffer | view-none-resident-s32 | 1607.547 | 3646.280 | 2.268 | 0.506 | 5 | 2.286–2.383 | fail | pass |
| caller-buffer | filter-1-resident-s4 | 3665.425 | 3519.081 | 0.960 | 1.172 | 5 | 0.956–0.969 | pass | n/a |
| caller-buffer | view-1-resident-s4 | 2149.284 | 2306.665 | 1.073 | 0.394 | 5 | 1.058–1.077 | pass | pass |
| caller-buffer | filter-1-resident-s8 | 5502.757 | 5369.233 | 0.976 | 1.827 | 5 | 0.956–0.973 | pass | n/a |
| caller-buffer | view-1-resident-s8 | 2252.394 | 3147.836 | 1.398 | 0.427 | 5 | 1.405–1.453 | pass | pass |
| caller-buffer | filter-2-resident-s8 | 5840.606 | 6304.042 | 1.079 | 1.003 | 5 | 1.076–1.088 | pass | n/a |
| caller-buffer | view-2-resident-s8 | 2296.525 | 3572.037 | 1.555 | 0.390 | 5 | 1.554–1.559 | fail | pass |
| caller-buffer | filter-3-resident-s12 | 18532.037 | 17378.796 | 0.938 | 0.992 | 5 | 0.936–0.939 | pass | pass |
| caller-buffer | view-3-resident-s12 | 2842.570 | 5351.946 | 1.883 | 0.340 | 5 | 1.877–1.891 | fail | pass |
| caller-buffer | index-2-v0-resident-s2 | 2003.648 | 1942.684 | 0.970 | 0.968 | 5 | 0.947–0.978 | pass | n/a |
| caller-buffer | index-2-v0-resident-s4 | 3446.118 | 3177.379 | 0.922 | 0.962 | 5 | 0.905–0.924 | pass | n/a |
| caller-buffer | index-2-v1-resident-s2 | 1672.111 | 1538.903 | 0.920 | 0.948 | 5 | 0.847–1.211 | pass | n/a |
| caller-buffer | index-2-v1-resident-s4 | 3514.846 | 3206.551 | 0.912 | 0.958 | 5 | 0.834–0.954 | pass | n/a |
| caller-buffer | index-3-v0-resident-s2 | 1444.913 | 1779.143 | 1.231 | 1.103 | 5 | 1.236–1.271 | pass | n/a |
| caller-buffer | index-3-v0-resident-s4 | 3196.708 | 3967.148 | 1.241 | 1.103 | 5 | 1.242–1.263 | pass | n/a |
| caller-buffer | index-3-v1-resident-s2 | 1621.553 | 2030.152 | 1.252 | 1.109 | 5 | 1.239–1.262 | pass | n/a |
| caller-buffer | index-3-v1-resident-s4 | 3070.758 | 3850.813 | 1.254 | 1.099 | 5 | 1.248–1.285 | pass | n/a |
| caller-buffer | vertex-v0-streaming-s4 | 1751.404 | 3372.917 | 1.926 | 0.423 | 5 | 1.837–2.021 | fail | pass |
| caller-buffer | view-none-streaming-s4 | 1848.609 | 3608.430 | 1.952 | 0.432 | 5 | 1.337–2.314 | fail | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1624.171 | 2944.545 | 1.813 | 0.507 | 5 | 1.739–1.908 | fail | pass |
| caller-buffer | view-none-streaming-s12 | 1695.245 | 3112.616 | 1.836 | 0.484 | 5 | 1.671–2.094 | fail | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1657.434 | 2687.730 | 1.622 | 0.470 | 5 | 1.520–2.115 | fail | pass |
| caller-buffer | view-none-streaming-s32 | 1722.739 | 3061.828 | 1.777 | 0.487 | 5 | 1.576–1.942 | fail | pass |
| caller-buffer | filter-1-streaming-s4 | 3845.699 | 3768.975 | 0.980 | 1.155 | 5 | 0.946–0.998 | pass | n/a |
| caller-buffer | view-1-streaming-s4 | 2291.767 | 2607.565 | 1.138 | 0.383 | 11 | 0.968–1.473 | pass | pass |
| caller-buffer | filter-1-streaming-s8 | 4469.072 | 4378.418 | 0.980 | 1.581 | 5 | 0.826–1.091 | pass | n/a |
| caller-buffer | view-1-streaming-s8 | 2430.917 | 2825.455 | 1.162 | 0.422 | 7 | 0.944–1.467 | pass | pass |
| caller-buffer | filter-2-streaming-s8 | 4983.942 | 5853.807 | 1.175 | 1.073 | 7 | 0.991–1.478 | pass | n/a |
| caller-buffer | view-2-streaming-s8 | 2469.453 | 3925.056 | 1.589 | 0.387 | 20+30 | 1.382–1.603 | borderline | pass |
| caller-buffer | filter-3-streaming-s12 | 4041.680 | 4009.141 | 0.992 | 0.982 | 5 | 0.925–1.082 | pass | pass |
| caller-buffer | view-3-streaming-s12 | 2769.303 | 4092.081 | 1.478 | 0.348 | 20+30 | 1.397–1.570 | borderline | pass |
| caller-buffer | index-2-v0-streaming-s2 | 579.445 | 720.173 | 1.243 | 1.047 | 6 | 0.921–1.438 | pass | n/a |
| caller-buffer | index-2-v0-streaming-s4 | 1930.058 | 1872.906 | 0.970 | 1.128 | 6 | 0.830–1.421 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s2 | 1938.605 | 1836.629 | 0.947 | 0.976 | 5 | 0.928–1.027 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s4 | 3335.052 | 2437.037 | 0.731 | 0.768 | 5 | 0.658–0.933 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s2 | 1004.969 | 1269.978 | 1.264 | 1.137 | 5 | 1.188–1.361 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s4 | 2091.535 | 2570.741 | 1.229 | 1.799 | 6 | 0.923–1.480 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s2 | 1692.900 | 2098.221 | 1.239 | 1.117 | 5 | 1.184–1.276 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s4 | 3499.700 | 4330.993 | 1.238 | 1.087 | 9 | 1.133–1.487 | pass | n/a |
| caller-buffer | vertex-v1-tiny-s4 | 407.629 | 1101.321 | 2.702 | 1.201 | 5 | 2.642–2.818 | fail | FAIL |
| caller-buffer | vertex-v1-tiny-s12 | 483.751 | 1350.587 | 2.792 | 1.218 | 5 | 2.215–3.109 | fail | FAIL |
| caller-buffer | vertex-v1-tiny-s32 | 368.199 | 924.308 | 2.510 | 0.914 | 5 | 2.444–2.607 | fail | FAIL |
| caller-buffer | vertex-v1-resident-s4 | 1916.564 | 3784.135 | 1.974 | 0.436 | 5 | 1.884–2.105 | fail | pass |
| caller-buffer | vertex-v1-resident-s12 | 1649.059 | 3875.695 | 2.350 | 0.500 | 5 | 2.267–2.376 | fail | pass |
| caller-buffer | vertex-v1-resident-s32 | 1626.833 | 3830.696 | 2.355 | 0.309 | 5 | 1.926–2.927 | fail | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1915.848 | 3685.856 | 1.924 | 0.429 | 5 | 1.905–1.964 | fail | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1234.894 | 1784.093 | 1.445 | 0.394 | 5 | 1.398–1.787 | fail | pass |
| caller-buffer | vertex-v1-streaming-s32 | 896.922 | 1632.358 | 1.820 | 0.409 | 6 | 1.386–2.784 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 923.348 | 1665.903 | 1.804 | 0.427 | 5 | 1.714–1.938 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 876.096 | 1695.987 | 1.936 | 0.413 | 5 | 1.659–2.103 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 917.801 | 1591.354 | 1.734 | 0.396 | 5 | 1.635–2.155 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 991.399 | 1751.935 | 1.767 | 0.384 | 5 | 1.653–1.953 | fail | pass |
| caller-buffer | index-2-v0-millions-s4 | 782.551 | 1046.531 | 1.337 | 1.054 | 6 | 1.210–1.468 | pass | n/a |
| caller-buffer | index-2-v1-millions-s4 | 1750.810 | 1744.324 | 0.996 | 0.897 | 6 | 0.719–1.456 | pass | n/a |
| caller-buffer | varied-filter-1-tiny-s4 | 540.967 | 919.537 | 1.700 | 0.210 | 5 | 1.621–1.820 | fail | pass |
| caller-buffer | varied-view-1-tiny-s4 | 190.301 | 347.195 | 1.824 | 0.433 | 5 | 1.727–1.990 | fail | pass |
| caller-buffer | varied-filter-1-tiny-s8 | 1057.741 | 1457.003 | 1.377 | 0.210 | 5 | 1.272–1.496 | pass | pass |
| caller-buffer | varied-view-1-tiny-s8 | 217.099 | 430.659 | 1.984 | 0.567 | 5 | 1.806–2.129 | fail | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 1218.149 | 1697.261 | 1.393 | 0.244 | 7 | 1.321–1.481 | pass | pass |
| caller-buffer | varied-view-2-tiny-s8 | 256.704 | 552.512 | 2.152 | 0.737 | 5 | 1.981–2.653 | fail | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 2507.056 | 2312.491 | 0.922 | 1.037 | 5 | 0.897–1.019 | pass | pass |
| caller-buffer | varied-view-3-tiny-s4 | 243.229 | 475.194 | 1.954 | 1.015 | 5 | 1.744–2.037 | fail | FAIL |
| caller-buffer | varied-filter-3-tiny-s12 | 5214.917 | 5301.500 | 1.017 | 1.019 | 5 | 0.932–1.118 | pass | pass |
| caller-buffer | varied-view-3-tiny-s12 | 264.036 | 578.006 | 2.189 | 1.021 | 5 | 2.114–2.228 | fail | FAIL |
| caller-buffer | varied-filter-3-tiny-s32 | 8674.006 | 9020.428 | 1.040 | 0.935 | 5 | 0.985–1.071 | pass | pass |
| caller-buffer | varied-view-3-tiny-s32 | 296.605 | 646.312 | 2.179 | 0.951 | 5 | 2.058–2.351 | fail | FAIL |
| caller-buffer | varied-filter-4-tiny-s4 | 760.913 | 1254.917 | 1.649 | 0.766 | 5 | 1.539–1.674 | fail | pass |
| caller-buffer | varied-filter-4-tiny-s8 | 1492.155 | 2120.756 | 1.421 | 0.700 | 6 | 1.348–1.499 | pass | pass |
| caller-buffer | varied-filter-1-resident-s4 | 1611.724 | 2815.658 | 1.747 | 0.140 | 5 | 1.695–1.808 | fail | pass |
| caller-buffer | varied-view-1-resident-s4 | 1064.190 | 1911.571 | 1.796 | 0.167 | 5 | 1.579–1.927 | fail | pass |
| caller-buffer | varied-filter-1-resident-s8 | 2577.758 | 3647.959 | 1.415 | 0.128 | 5 | 1.336–1.459 | pass | pass |
| caller-buffer | varied-view-1-resident-s8 | 891.212 | 1759.845 | 1.975 | 0.226 | 5 | 1.721–2.309 | fail | pass |
| caller-buffer | varied-filter-2-resident-s8 | 3300.438 | 4457.939 | 1.351 | 0.120 | 12 | 1.269–1.482 | pass | pass |
| caller-buffer | varied-view-2-resident-s8 | 812.350 | 1346.208 | 1.657 | 0.201 | 5 | 1.561–1.797 | fail | pass |
| caller-buffer | varied-filter-3-resident-s4 | 8497.333 | 7880.494 | 0.927 | 1.006 | 5 | 0.910–0.946 | pass | pass |
| caller-buffer | varied-view-3-resident-s4 | 1150.725 | 1610.483 | 1.400 | 0.314 | 6 | 1.342–1.499 | pass | pass |
| caller-buffer | varied-filter-3-resident-s12 | 8761.116 | 8144.259 | 0.930 | 1.018 | 5 | 0.875–1.042 | pass | pass |
| caller-buffer | varied-view-3-resident-s12 | 932.730 | 1627.626 | 1.745 | 0.397 | 13 | 1.513–1.988 | fail | pass |
| caller-buffer | varied-filter-3-resident-s32 | 10614.806 | 9971.068 | 0.939 | 1.061 | 5 | 0.851–1.062 | pass | FAIL |
| caller-buffer | varied-view-3-resident-s32 | 958.667 | 1675.862 | 1.748 | 0.365 | 5 | 1.692–1.861 | fail | pass |
| caller-buffer | varied-filter-4-resident-s4 | 1554.264 | 2523.337 | 1.623 | 0.528 | 5 | 1.565–1.706 | fail | pass |
| caller-buffer | varied-filter-4-resident-s8 | 2744.248 | 3924.958 | 1.430 | 0.611 | 5 | 1.397–1.484 | pass | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 1198.995 | 2118.540 | 1.767 | 0.084 | 5 | 1.694–1.841 | fail | pass |
| caller-buffer | varied-view-1-streaming-s4 | 756.740 | 1254.846 | 1.658 | 0.118 | 5 | 1.634–1.724 | fail | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 2546.050 | 3542.186 | 1.391 | 0.092 | 9 | 1.358–1.490 | pass | pass |
| caller-buffer | varied-view-1-streaming-s8 | 874.900 | 1562.909 | 1.786 | 0.178 | 5 | 1.739–1.796 | fail | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 3113.943 | 3970.521 | 1.275 | 0.086 | 5 | 1.268–1.285 | pass | pass |
| caller-buffer | varied-view-2-streaming-s8 | 951.931 | 1666.856 | 1.751 | 0.166 | 5 | 1.715–1.781 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 12756.641 | 11164.405 | 0.875 | 0.991 | 5 | 0.872–0.942 | pass | pass |
| caller-buffer | varied-view-3-streaming-s4 | 1389.023 | 1990.693 | 1.433 | 0.303 | 5 | 1.371–1.459 | pass | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 13082.744 | 11810.747 | 0.903 | 1.001 | 5 | 0.883–0.933 | pass | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1101.039 | 1986.040 | 1.804 | 0.323 | 5 | 1.702–1.881 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 10735.052 | 10124.524 | 0.943 | 0.979 | 5 | 0.891–1.013 | pass | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1094.137 | 1934.006 | 1.768 | 0.317 | 5 | 1.707–1.824 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s4 | 1906.251 | 3213.675 | 1.686 | 0.531 | 5 | 1.674–1.739 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s8 | 3475.042 | 5200.562 | 1.497 | 0.599 | 10 | 1.484–1.500 | pass | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 171.522 | 409.126 | 2.385 | 1.428 | 5 | 2.330–2.446 | fail | FAIL |
| caller-buffer | meshlet-1-1-v2-t4 | 209.433 | 509.328 | 2.432 | 1.419 | 5 | 2.408–2.484 | fail | FAIL |
| caller-buffer | meshlet-1-1-v4-t3 | 240.282 | 565.654 | 2.354 | 1.431 | 5 | 2.321–2.416 | fail | FAIL |
| caller-buffer | meshlet-1-1-v4-t4 | 272.206 | 671.954 | 2.469 | 1.467 | 5 | 2.429–2.499 | fail | FAIL |
| caller-buffer | meshlet-raw-1-1 | 425.529 | 727.488 | 1.710 | 1.065 | 5 | 1.690–1.726 | fail | FAIL |
| caller-buffer | meshlet-64-126-v2-t3 | 2282.781 | 5856.997 | 2.566 | 0.545 | 5 | 2.076–2.891 | fail | pass |
| caller-buffer | meshlet-64-126-v2-t4 | 3130.733 | 6877.306 | 2.197 | 0.493 | 5 | 2.148–2.258 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t3 | 2871.810 | 7508.846 | 2.615 | 0.540 | 5 | 2.530–2.732 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t4 | 3978.199 | 8546.631 | 2.148 | 0.487 | 5 | 2.114–2.175 | fail | pass |
| caller-buffer | meshlet-raw-64-126 | 4174.132 | 8792.630 | 2.106 | 0.480 | 5 | 2.079–2.140 | fail | pass |
| caller-buffer | meshlet-256-256-v2-t3 | 2408.326 | 6246.636 | 2.594 | 0.517 | 5 | 2.555–2.623 | fail | pass |
| caller-buffer | meshlet-256-256-v2-t4 | 3217.980 | 7411.806 | 2.303 | 0.483 | 5 | 2.280–2.323 | fail | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 3400.273 | 8718.411 | 2.564 | 0.509 | 5 | 2.555–2.574 | fail | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 4111.702 | 9215.943 | 2.241 | 0.455 | 5 | 2.133–2.397 | fail | pass |
| caller-buffer | meshlet-raw-256-256 | 4080.806 | 9493.042 | 2.326 | 0.489 | 5 | 2.164–2.478 | fail | pass |

## S4 scalar C++ case intervals

The original family means and medians stay fixed. Only scalar-baseline borderline cases receive additional pairs, up to twenty total; remaining borderline cases alone receive thirty fresh D146 pairs. No clear failure is retried.

| API | Case | Original pairs | Additional stage-1 pairs | Fresh stage-2 pairs | Final interval | S4 maximum |
|---|---|---:|---:|---:|---|---|
| allocating | index-2-v0-tiny-s2 | 5 | 0 | 0 | 0.984–1.176 | pass |
| allocating | index-2-v0-tiny-s4 | 5 | 0 | 0 | 1.095–1.169 | pass |
| allocating | index-2-v1-tiny-s2 | 5 | 0 | 0 | 1.110–1.161 | pass |
| allocating | index-2-v1-tiny-s4 | 5 | 0 | 0 | 1.113–1.180 | pass |
| allocating | index-3-v0-tiny-s2 | 5 | 0 | 0 | 1.143–1.452 | pass |
| allocating | index-3-v0-tiny-s4 | 5 | 0 | 0 | 1.243–1.398 | pass |
| allocating | index-3-v1-tiny-s2 | 5 | 0 | 0 | 1.311–1.340 | pass |
| allocating | index-3-v1-tiny-s4 | 5 | 0 | 0 | 1.319–1.388 | pass |
| allocating | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.931–0.968 | pass |
| allocating | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.840–0.941 | pass |
| allocating | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.821–1.321 | pass |
| allocating | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.798–0.976 | pass |
| allocating | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.111–1.275 | pass |
| allocating | index-3-v0-resident-s4 | 5 | 0 | 0 | 1.197–1.290 | pass |
| allocating | index-3-v1-resident-s2 | 5 | 0 | 0 | 1.222–1.289 | pass |
| allocating | index-3-v1-resident-s4 | 5 | 0 | 0 | 1.220–1.246 | pass |
| allocating | index-2-v0-streaming-s2 | 5 | 0 | 0 | 0.980–1.465 | pass |
| allocating | index-2-v0-streaming-s4 | 5 | 0 | 0 | 1.264–1.312 | pass |
| allocating | index-2-v1-streaming-s2 | 7 | 0 | 0 | 1.531–1.823 | fail |
| allocating | index-2-v1-streaming-s4 | 5 | 0 | 0 | 1.570–1.739 | fail |
| allocating | index-3-v0-streaming-s2 | 5 | 0 | 0 | 0.930–1.410 | pass |
| allocating | index-3-v0-streaming-s4 | 5 | 0 | 0 | 1.220–1.288 | pass |
| allocating | index-3-v1-streaming-s2 | 5 | 0 | 0 | 1.217–1.248 | pass |
| allocating | index-3-v1-streaming-s4 | 5 | 0 | 0 | 1.226–1.299 | pass |
| allocating | index-2-v0-millions-s4 | 5 | 0 | 0 | 1.066–1.193 | pass |
| allocating | index-2-v1-millions-s4 | 5 | 0 | 0 | 0.917–1.023 | pass |
| caller-buffer | index-2-v0-tiny-s2 | 5 | 0 | 0 | 1.118–1.138 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 5 | 1 | 0 | 0.927–1.466 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 5 | 0 | 0 | 1.110–1.140 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 5 | 0 | 0 | 1.159–1.218 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 5 | 0 | 0 | 0.985–1.024 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 5 | 0 | 0 | 0.957–0.999 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 5 | 0 | 0 | 0.983–1.033 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 5 | 0 | 0 | 0.945–1.029 | pass |
| caller-buffer | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.926–0.984 | pass |
| caller-buffer | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.889–0.907 | pass |
| caller-buffer | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.845–1.205 | pass |
| caller-buffer | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.877–0.919 | pass |
| caller-buffer | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.230–1.276 | pass |
| caller-buffer | index-3-v0-resident-s4 | 5 | 0 | 0 | 1.235–1.267 | pass |
| caller-buffer | index-3-v1-resident-s2 | 5 | 0 | 0 | 1.246–1.259 | pass |
| caller-buffer | index-3-v1-resident-s4 | 5 | 0 | 0 | 1.206–1.273 | pass |
| caller-buffer | index-2-v0-streaming-s2 | 6 | 0 | 0 | 1.164–1.262 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 6 | 4 | 0 | 1.052–1.480 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 5 | 0 | 0 | 0.843–1.363 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 5 | 0 | 0 | 0.613–0.949 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 5 | 0 | 0 | 1.158–1.462 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 6 | 5 | 0 | 1.100–1.486 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 5 | 0 | 0 | 1.199–1.287 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 9 | 1 | 0 | 1.177–1.492 | pass |
| caller-buffer | index-2-v0-millions-s4 | 6 | 0 | 0 | 1.218–1.438 | pass |
| caller-buffer | index-2-v1-millions-s4 | 6 | 1 | 0 | 0.743–1.405 | pass |

## Node shipped SIMD decoder

| API | Family | Cases | Time Rust/upstream SIMD GM | Worst | Time Rust/scalar GM |
|---|---|---:|---:|---:|---:|
| allocating | index | 14 | 1.044 | 1.299 | 0.979 |
| allocating | sequence | 12 | 1.042 | 1.086 | 1.013 |
| allocating | vertex | 22 | 1.312 | 1.651 | 0.521 |
| allocating | view-filtered | 30 | 1.661 | 2.454 | 0.513 |
| allocating | view-none | 9 | 1.326 | 1.579 | 0.533 |
| caller-buffer | index | 14 | 1.064 | 1.267 | 0.978 |
| caller-buffer | sequence | 12 | 0.947 | 1.043 | 0.988 |
| caller-buffer | vertex | 22 | 1.335 | 1.830 | 0.484 |
| caller-buffer | view-filtered | 30 | 1.775 | 2.724 | 0.492 |
| caller-buffer | view-none | 9 | 1.329 | 1.588 | 0.492 |

| API | Case | Time Rust/upstream SIMD | Time Rust/scalar | Pairs | Final interval | Case bar |
|---|---|---:|---:|---:|---|---|
| allocating | vertex-v0-tiny-s4 | 1.170 | 0.852 | 15 | 0.646–1.576 | pass |
| allocating | view-none-tiny-s4 | 1.197 | 0.817 | 7 | 0.588–1.494 | pass |
| allocating | vertex-v0-tiny-s12 | 1.397 | 0.787 | 5 | 1.334–1.474 | pass |
| allocating | view-none-tiny-s12 | 1.462 | 0.799 | 11 | 1.090–1.590 | pass |
| allocating | vertex-v0-tiny-s32 | 1.482 | 0.735 | 5 | 1.420–1.573 | pass |
| allocating | view-none-tiny-s32 | 1.579 | 0.728 | 20 | 1.507–1.598 | pass |
| allocating | view-1-tiny-s4 | 1.444 | 0.989 | 20+30 | 1.224–1.468 | pass |
| allocating | view-1-tiny-s8 | 1.456 | 0.992 | 6 | 1.432–1.579 | pass |
| allocating | view-2-tiny-s8 | 0.884 | 1.039 | 7 | 0.336–1.575 | pass |
| allocating | view-3-tiny-s12 | 1.352 | 0.969 | 16 | 1.038–1.583 | pass |
| allocating | index-2-v0-tiny-s2 | 0.423 | 0.981 | 5 | 0.084–1.168 | pass |
| allocating | index-2-v0-tiny-s4 | 1.014 | 0.973 | 5 | 0.962–1.062 | pass |
| allocating | index-2-v1-tiny-s2 | 0.996 | 0.956 | 5 | 0.883–1.211 | pass |
| allocating | index-2-v1-tiny-s4 | 0.974 | 0.964 | 5 | 0.928–1.140 | pass |
| allocating | index-3-v0-tiny-s2 | 1.023 | 1.185 | 7 | 0.291–1.461 | pass |
| allocating | index-3-v0-tiny-s4 | 1.062 | 1.006 | 5 | 1.021–1.127 | pass |
| allocating | index-3-v1-tiny-s2 | 0.985 | 0.951 | 5 | 0.859–1.263 | pass |
| allocating | index-3-v1-tiny-s4 | 1.050 | 1.002 | 5 | 0.965–1.178 | pass |
| allocating | vertex-v0-resident-s4 | 1.039 | 0.455 | 5 | 0.800–1.204 | pass |
| allocating | view-none-resident-s4 | 1.170 | 0.372 | 5 | 1.163–1.187 | pass |
| allocating | vertex-v0-resident-s12 | 1.412 | 0.440 | 8 | 1.299–1.573 | pass |
| allocating | view-none-resident-s12 | 1.344 | 0.438 | 5 | 1.250–1.430 | pass |
| allocating | vertex-v0-resident-s32 | 1.475 | 0.457 | 6 | 1.327–1.554 | pass |
| allocating | view-none-resident-s32 | 1.407 | 0.443 | 6 | 1.107–1.577 | pass |
| allocating | view-1-resident-s4 | 1.798 | 0.609 | 5 | 1.652–1.938 | fail |
| allocating | view-1-resident-s8 | 2.437 | 0.629 | 5 | 2.271–2.609 | fail |
| allocating | view-2-resident-s8 | 2.454 | 0.633 | 5 | 2.204–2.633 | fail |
| allocating | view-3-resident-s12 | 2.269 | 0.551 | 5 | 2.040–2.411 | fail |
| allocating | index-2-v0-resident-s2 | 1.171 | 0.939 | 5 | 1.142–1.191 | pass |
| allocating | index-2-v0-resident-s4 | 1.181 | 0.954 | 5 | 1.147–1.302 | pass |
| allocating | index-2-v1-resident-s2 | 1.159 | 0.951 | 5 | 1.023–1.291 | pass |
| allocating | index-2-v1-resident-s4 | 1.144 | 0.938 | 5 | 1.017–1.216 | pass |
| allocating | index-3-v0-resident-s2 | 1.034 | 1.009 | 5 | 0.974–1.158 | pass |
| allocating | index-3-v0-resident-s4 | 1.046 | 1.010 | 5 | 0.944–1.208 | pass |
| allocating | index-3-v1-resident-s2 | 1.072 | 1.043 | 5 | 0.960–1.360 | pass |
| allocating | index-3-v1-resident-s4 | 1.031 | 0.920 | 5 | 0.966–1.062 | pass |
| allocating | vertex-v0-streaming-s4 | 1.165 | 0.381 | 9 | 0.836–1.545 | pass |
| allocating | view-none-streaming-s4 | 1.202 | 0.390 | 14 | 0.874–1.558 | pass |
| allocating | vertex-v0-streaming-s12 | 1.264 | 0.454 | 11 | 0.944–1.576 | pass |
| allocating | view-none-streaming-s12 | 1.384 | 0.461 | 5 | 1.067–1.488 | pass |
| allocating | vertex-v0-streaming-s32 | 1.224 | 0.550 | 5 | 1.067–1.343 | pass |
| allocating | view-none-streaming-s32 | 1.249 | 0.558 | 5 | 1.115–1.326 | pass |
| allocating | view-1-streaming-s4 | 1.947 | 0.616 | 5 | 1.797–2.040 | fail |
| allocating | view-1-streaming-s8 | 2.390 | 0.638 | 5 | 1.690–3.012 | fail |
| allocating | view-2-streaming-s8 | 2.375 | 0.628 | 5 | 2.190–2.520 | fail |
| allocating | view-3-streaming-s12 | 1.462 | 0.687 | 14 | 1.384–1.598 | pass |
| allocating | index-2-v0-streaming-s2 | 1.059 | 1.000 | 5 | 1.021–1.086 | pass |
| allocating | index-2-v0-streaming-s4 | 0.996 | 0.956 | 6 | 0.695–1.469 | pass |
| allocating | index-2-v1-streaming-s2 | 1.221 | 0.972 | 8 | 0.996–1.560 | pass |
| allocating | index-2-v1-streaming-s4 | 1.299 | 1.020 | 11 | 0.995–1.546 | pass |
| allocating | index-3-v0-streaming-s2 | 1.034 | 1.008 | 5 | 0.638–1.427 | pass |
| allocating | index-3-v0-streaming-s4 | 1.077 | 1.023 | 5 | 0.700–1.569 | pass |
| allocating | index-3-v1-streaming-s2 | 1.013 | 1.008 | 5 | 0.760–1.102 | pass |
| allocating | index-3-v1-streaming-s4 | 1.086 | 1.010 | 5 | 0.725–1.576 | pass |
| allocating | vertex-v1-tiny-s4 | 1.042 | 0.826 | 9 | 0.606–1.574 | pass |
| allocating | vertex-v1-tiny-s12 | 1.370 | 0.843 | 5 | 0.793–1.590 | pass |
| allocating | vertex-v1-tiny-s32 | 1.651 | 0.828 | 5 | 1.626–1.745 | fail |
| allocating | vertex-v1-resident-s4 | 1.210 | 0.384 | 5 | 1.158–1.264 | pass |
| allocating | vertex-v1-resident-s12 | 1.428 | 0.435 | 5 | 1.392–1.463 | pass |
| allocating | vertex-v1-resident-s32 | 1.429 | 0.436 | 5 | 1.374–1.496 | pass |
| allocating | vertex-v1-streaming-s4 | 1.184 | 0.380 | 9 | 0.887–1.580 | pass |
| allocating | vertex-v1-streaming-s12 | 1.277 | 0.452 | 6 | 0.851–1.548 | pass |
| allocating | vertex-v1-streaming-s32 | 1.207 | 0.527 | 5 | 1.065–1.334 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.402 | 0.435 | 5 | 1.326–1.475 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.426 | 0.437 | 10 | 0.992–1.555 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.418 | 0.435 | 5 | 1.266–1.584 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.377 | 0.431 | 5 | 1.334–1.425 | pass |
| allocating | index-2-v0-millions-s4 | 1.138 | 1.103 | 7 | 0.995–1.591 | pass |
| allocating | index-2-v1-millions-s4 | 1.259 | 1.012 | 10 | 0.937–1.599 | pass |
| allocating | varied-view-1-tiny-s4 | 1.247 | 0.626 | 7 | 0.728–1.562 | pass |
| allocating | varied-view-1-tiny-s8 | 1.425 | 0.675 | 5 | 1.379–1.486 | pass |
| allocating | varied-view-2-tiny-s8 | 1.414 | 0.670 | 5 | 1.317–1.464 | pass |
| allocating | varied-view-3-tiny-s4 | 1.095 | 0.819 | 5 | 0.868–1.309 | pass |
| allocating | varied-view-3-tiny-s12 | 1.387 | 0.797 | 5 | 1.288–1.518 | pass |
| allocating | varied-view-3-tiny-s32 | 1.717 | 0.798 | 5 | 1.657–1.744 | fail |
| allocating | varied-view-1-resident-s4 | 1.871 | 0.215 | 5 | 1.776–1.919 | fail |
| allocating | varied-view-1-resident-s8 | 1.914 | 0.281 | 5 | 1.889–1.966 | fail |
| allocating | varied-view-2-resident-s8 | 1.844 | 0.275 | 5 | 1.692–2.029 | fail |
| allocating | varied-view-3-resident-s4 | 1.298 | 0.400 | 5 | 1.161–1.457 | pass |
| allocating | varied-view-3-resident-s12 | 1.602 | 0.464 | 20+30 | 1.581–1.632 | fail |
| allocating | varied-view-3-resident-s32 | 1.676 | 0.473 | 5 | 1.625–1.728 | fail |
| allocating | varied-view-1-streaming-s4 | 1.892 | 0.160 | 20+30 | 1.830–1.920 | fail |
| allocating | varied-view-1-streaming-s8 | 1.918 | 0.201 | 7 | 1.617–2.173 | fail |
| allocating | varied-view-2-streaming-s8 | 1.885 | 0.208 | 20+30 | 1.498–1.987 | fail |
| allocating | varied-view-3-streaming-s4 | 1.365 | 0.395 | 14 | 1.144–1.559 | pass |
| allocating | varied-view-3-streaming-s12 | 1.707 | 0.392 | 20+30 | 1.403–1.839 | fail |
| allocating | varied-view-3-streaming-s32 | 1.708 | 0.406 | 5 | 1.622–1.828 | fail |
| caller-buffer | vertex-v0-tiny-s4 | 1.030 | 0.755 | 7 | 0.479–1.551 | pass |
| caller-buffer | view-none-tiny-s4 | 1.095 | 0.774 | 5 | 1.005–1.316 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.372 | 0.719 | 5 | 1.290–1.535 | pass |
| caller-buffer | view-none-tiny-s12 | 1.391 | 0.713 | 5 | 1.274–1.582 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.575 | 0.687 | 18 | 1.563–1.599 | pass |
| caller-buffer | view-none-tiny-s32 | 1.588 | 0.685 | 20+30 | 1.567–1.584 | pass |
| caller-buffer | view-1-tiny-s4 | 1.277 | 1.011 | 5 | 1.223–1.377 | pass |
| caller-buffer | view-1-tiny-s8 | 1.590 | 1.060 | 20 | 1.564–1.599 | pass |
| caller-buffer | view-2-tiny-s8 | 1.673 | 1.094 | 6 | 1.607–1.832 | fail |
| caller-buffer | view-3-tiny-s12 | 1.654 | 0.986 | 5 | 1.618–1.714 | fail |
| caller-buffer | index-2-v0-tiny-s2 | 0.864 | 0.960 | 5 | 0.859–0.883 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 0.882 | 0.980 | 5 | 0.862–0.902 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 0.867 | 0.968 | 5 | 0.822–0.943 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 0.864 | 0.957 | 5 | 0.768–1.075 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.823 | 0.957 | 5 | 0.789–0.906 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.839 | 0.954 | 5 | 0.811–0.852 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.825 | 0.961 | 5 | 0.779–0.971 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 0.842 | 0.962 | 5 | 0.823–0.868 | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.180 | 0.347 | 5 | 1.164–1.207 | pass |
| caller-buffer | view-none-resident-s4 | 1.185 | 0.353 | 5 | 1.151–1.204 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.442 | 0.421 | 5 | 1.414–1.474 | pass |
| caller-buffer | view-none-resident-s12 | 1.424 | 0.428 | 5 | 1.370–1.496 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.408 | 0.414 | 5 | 1.324–1.477 | pass |
| caller-buffer | view-none-resident-s32 | 1.415 | 0.421 | 5 | 1.372–1.440 | pass |
| caller-buffer | view-1-resident-s4 | 1.976 | 0.586 | 5 | 1.928–2.057 | fail |
| caller-buffer | view-1-resident-s8 | 2.724 | 0.624 | 5 | 2.704–2.769 | fail |
| caller-buffer | view-2-resident-s8 | 2.595 | 0.611 | 5 | 2.570–2.638 | fail |
| caller-buffer | view-3-resident-s12 | 2.584 | 0.534 | 5 | 2.518–2.615 | fail |
| caller-buffer | index-2-v0-resident-s2 | 1.168 | 1.000 | 5 | 1.131–1.228 | pass |
| caller-buffer | index-2-v0-resident-s4 | 1.143 | 0.966 | 5 | 1.107–1.180 | pass |
| caller-buffer | index-2-v1-resident-s2 | 1.154 | 0.873 | 5 | 1.138–1.176 | pass |
| caller-buffer | index-2-v1-resident-s4 | 1.145 | 0.963 | 5 | 1.129–1.174 | pass |
| caller-buffer | index-3-v0-resident-s2 | 0.998 | 1.003 | 5 | 0.904–1.037 | pass |
| caller-buffer | index-3-v0-resident-s4 | 1.015 | 1.002 | 5 | 0.995–1.025 | pass |
| caller-buffer | index-3-v1-resident-s2 | 1.002 | 0.993 | 5 | 0.979–1.016 | pass |
| caller-buffer | index-3-v1-resident-s4 | 1.004 | 1.004 | 5 | 0.991–1.027 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 1.161 | 0.365 | 5 | 1.122–1.301 | pass |
| caller-buffer | view-none-streaming-s4 | 1.183 | 0.364 | 5 | 1.078–1.290 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.384 | 0.443 | 5 | 1.364–1.401 | pass |
| caller-buffer | view-none-streaming-s12 | 1.389 | 0.438 | 5 | 1.358–1.435 | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1.329 | 0.443 | 5 | 1.308–1.388 | pass |
| caller-buffer | view-none-streaming-s32 | 1.359 | 0.437 | 5 | 1.328–1.373 | pass |
| caller-buffer | view-1-streaming-s4 | 2.013 | 0.593 | 5 | 1.962–2.040 | fail |
| caller-buffer | view-1-streaming-s8 | 2.562 | 0.635 | 5 | 2.477–2.665 | fail |
| caller-buffer | view-2-streaming-s8 | 2.326 | 0.626 | 5 | 2.277–2.425 | fail |
| caller-buffer | view-3-streaming-s12 | 1.684 | 0.582 | 5 | 1.652–1.735 | fail |
| caller-buffer | index-2-v0-streaming-s2 | 1.039 | 0.999 | 5 | 1.018–1.078 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 0.963 | 1.006 | 5 | 0.949–1.037 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 1.253 | 1.014 | 5 | 1.228–1.284 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 1.240 | 1.002 | 5 | 1.240–1.319 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 1.025 | 1.011 | 5 | 1.006–1.045 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 0.998 | 0.990 | 5 | 0.991–1.024 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.992 | 0.993 | 5 | 0.982–1.007 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 1.043 | 1.027 | 5 | 1.010–1.076 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 1.060 | 0.833 | 7 | 0.439–1.507 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.487 | 0.814 | 5 | 1.422–1.599 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.830 | 0.792 | 5 | 1.776–1.889 | fail |
| caller-buffer | vertex-v1-resident-s4 | 1.172 | 0.353 | 5 | 1.101–1.228 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.395 | 0.412 | 5 | 1.395–1.424 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.414 | 0.419 | 5 | 1.396–1.457 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1.214 | 0.371 | 5 | 1.139–1.290 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.086 | 0.441 | 5 | 1.081–1.140 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1.389 | 0.437 | 5 | 1.349–1.411 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.426 | 0.420 | 5 | 1.392–1.492 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.432 | 0.417 | 5 | 1.408–1.466 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.428 | 0.419 | 5 | 1.422–1.435 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.420 | 0.417 | 10 | 1.061–1.581 | pass |
| caller-buffer | index-2-v0-millions-s4 | 1.199 | 1.006 | 5 | 0.978–1.254 | pass |
| caller-buffer | index-2-v1-millions-s4 | 1.267 | 1.006 | 5 | 1.198–1.322 | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.247 | 0.533 | 9 | 0.769–1.579 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.468 | 0.575 | 5 | 1.438–1.498 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.507 | 0.590 | 8 | 1.450–1.587 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.093 | 0.814 | 5 | 1.065–1.137 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.486 | 0.761 | 5 | 1.444–1.559 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.834 | 0.773 | 5 | 1.750–1.978 | fail |
| caller-buffer | varied-view-1-resident-s4 | 1.973 | 0.207 | 5 | 1.839–2.271 | fail |
| caller-buffer | varied-view-1-resident-s8 | 1.971 | 0.271 | 5 | 1.965–1.984 | fail |
| caller-buffer | varied-view-2-resident-s8 | 1.899 | 0.262 | 5 | 1.888–1.915 | fail |
| caller-buffer | varied-view-3-resident-s4 | 1.307 | 0.371 | 5 | 1.299–1.323 | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.652 | 0.450 | 5 | 1.634–1.684 | fail |
| caller-buffer | varied-view-3-resident-s32 | 1.655 | 0.444 | 7 | 1.607–1.686 | fail |
| caller-buffer | varied-view-1-streaming-s4 | 1.966 | 0.152 | 5 | 1.960–1.991 | fail |
| caller-buffer | varied-view-1-streaming-s8 | 2.011 | 0.193 | 5 | 1.953–2.126 | fail |
| caller-buffer | varied-view-2-streaming-s8 | 2.084 | 0.207 | 5 | 1.884–2.080 | fail |
| caller-buffer | varied-view-3-streaming-s4 | 1.355 | 0.373 | 5 | 1.330–1.380 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.775 | 0.369 | 5 | 1.758–1.815 | fail |
| caller-buffer | varied-view-3-streaming-s32 | 1.712 | 0.395 | 5 | 1.655–1.835 | fail |
