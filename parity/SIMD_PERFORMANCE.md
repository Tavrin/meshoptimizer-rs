# P07 individual SIMD performance

Raw native evidence: `/mnt/linux-extra/meshopt-artifacts/p07r3/performance.json`; raw Node evidence: `/mnt/linux-extra/meshopt-artifacts/p07r3/wasm-performance.json`.

Time ratios: lower is faster. Family means use stage 1; only borderline case maxima use fresh D146 intervals. Repeated standalone filters are reported separately, outside the SIMD filter bar. See SIMD_RESULTS.md for shared-host and release limits.

| API | Family | Cases | Time Rust/C++ SIMD GM | Worst stage-1 | Time Rust/scalar Rust GM |
|---|---|---:|---:|---:|---:|
| allocating | color | 6 | 1.540 | 1.918 | 0.750 |
| allocating | exp | 12 | 0.886 | 1.057 | 0.990 |
| allocating | index | 14 | 1.144 | 1.966 | 0.957 |
| allocating | meshlet | 12 | 1.329 | 1.520 | 0.447 |
| allocating | meshlet-raw | 3 | 1.228 | 1.369 | 0.421 |
| allocating | oct | 6 | 1.492 | 1.948 | 0.136 |
| allocating | oct (repeated) | 6 | 1.016 | 1.202 | 1.254 |
| allocating | quat | 3 | 1.237 | 1.358 | 0.135 |
| allocating | quat (repeated) | 3 | 1.123 | 1.231 | 1.086 |
| allocating | sequence | 12 | 1.298 | 2.888 | 0.998 |
| allocating | vertex | 22 | 1.832 | 2.428 | 0.581 |
| allocating | view-filtered | 30 | 1.841 | 3.003 | 0.478 |
| allocating | view-none | 9 | 1.702 | 2.376 | 0.606 |
| caller-buffer | color | 6 | 1.496 | 1.748 | 0.751 |
| caller-buffer | exp | 12 | 0.935 | 1.028 | 1.003 |
| caller-buffer | index | 14 | 1.075 | 1.291 | 1.015 |
| caller-buffer | meshlet | 12 | 1.534 | 1.933 | 0.458 |
| caller-buffer | meshlet-raw | 3 | 1.315 | 1.410 | 0.401 |
| caller-buffer | oct | 6 | 1.535 | 1.885 | 0.165 |
| caller-buffer | oct (repeated) | 6 | 1.024 | 1.233 | 1.369 |
| caller-buffer | quat | 3 | 1.336 | 1.371 | 0.156 |
| caller-buffer | quat (repeated) | 3 | 1.159 | 1.324 | 1.030 |
| caller-buffer | sequence | 12 | 1.058 | 1.140 | 0.993 |
| caller-buffer | vertex | 22 | 1.967 | 2.287 | 0.541 |
| caller-buffer | view-filtered | 30 | 1.826 | 2.352 | 0.460 |
| caller-buffer | view-none | 9 | 1.872 | 2.263 | 0.542 |

## Native individual cases

| API | Case | Rust MB/s | C++ SIMD MB/s | Time Rust/C++ SIMD | Time Rust/scalar | Pairs | Final interval | Case bar | S3 |
|---|---|---:|---:|---:|---:|---:|---|---|---|
| allocating | vertex-v0-tiny-s4 | 164.632 | 271.792 | 1.651 | 0.764 | 5 | 1.643–1.781 | fail | pass |
| allocating | view-none-tiny-s4 | 191.510 | 324.315 | 1.693 | 0.811 | 5 | 1.636–1.728 | fail | pass |
| allocating | vertex-v0-tiny-s12 | 241.655 | 446.366 | 1.847 | 0.850 | 5 | 1.769–1.978 | fail | pass |
| allocating | view-none-tiny-s12 | 248.256 | 464.981 | 1.873 | 0.858 | 5 | 1.609–2.615 | fail | pass |
| allocating | vertex-v0-tiny-s32 | 235.954 | 472.163 | 2.001 | 0.928 | 5 | 1.952–2.143 | fail | pass |
| allocating | view-none-tiny-s32 | 248.083 | 514.727 | 2.075 | 0.958 | 5 | 1.979–2.212 | fail | pass |
| allocating | filter-1-tiny-s4 | 405.570 | 487.673 | 1.202 | 1.114 | 5 | 1.116–1.279 | pass | n/a |
| allocating | view-1-tiny-s4 | 148.787 | 231.716 | 1.557 | 1.069 | 13 | 1.501–1.674 | fail | pass |
| allocating | filter-1-tiny-s8 | 750.008 | 814.089 | 1.085 | 1.354 | 5 | 1.044–1.079 | pass | n/a |
| allocating | view-1-tiny-s8 | 178.114 | 291.783 | 1.638 | 1.301 | 13 | 1.501–1.750 | fail | FAIL |
| allocating | filter-2-tiny-s8 | 671.199 | 825.978 | 1.231 | 1.353 | 5 | 1.176–1.324 | pass | n/a |
| allocating | view-2-tiny-s8 | 169.071 | 276.488 | 1.635 | 0.981 | 5 | 1.633–1.910 | fail | pass |
| allocating | filter-3-tiny-s12 | 2817.579 | 2670.640 | 0.948 | 1.045 | 5 | 0.736–1.137 | pass | pass |
| allocating | view-3-tiny-s12 | 203.761 | 346.687 | 1.701 | 1.044 | 6 | 1.554–1.897 | fail | FAIL |
| allocating | index-2-v0-tiny-s2 | 236.527 | 283.901 | 1.200 | 1.034 | 5 | 1.133–1.207 | pass | n/a |
| allocating | index-2-v0-tiny-s4 | 540.975 | 640.107 | 1.183 | 0.943 | 5 | 1.151–1.223 | pass | n/a |
| allocating | index-2-v1-tiny-s2 | 244.636 | 231.789 | 0.947 | 0.976 | 5 | 0.803–1.487 | pass | n/a |
| allocating | index-2-v1-tiny-s4 | 426.539 | 477.952 | 1.121 | 0.949 | 5 | 0.992–1.328 | pass | n/a |
| allocating | index-3-v0-tiny-s2 | 256.942 | 330.209 | 1.285 | 0.985 | 6 | 1.038–1.465 | pass | n/a |
| allocating | index-3-v0-tiny-s4 | 637.601 | 562.580 | 0.882 | 0.815 | 5 | 0.856–1.022 | pass | n/a |
| allocating | index-3-v1-tiny-s2 | 253.953 | 327.219 | 1.289 | 0.972 | 5 | 1.249–1.327 | pass | n/a |
| allocating | index-3-v1-tiny-s4 | 517.572 | 452.379 | 0.874 | 1.053 | 5 | 0.805–0.929 | pass | n/a |
| allocating | vertex-v0-resident-s4 | 829.207 | 1241.090 | 1.497 | 0.281 | 5 | 1.463–1.703 | fail | pass |
| allocating | view-none-resident-s4 | 835.927 | 1262.000 | 1.510 | 0.296 | 5 | 1.482–1.648 | fail | pass |
| allocating | vertex-v0-resident-s12 | 508.157 | 1059.456 | 2.085 | 0.512 | 5 | 1.955–2.555 | fail | pass |
| allocating | view-none-resident-s12 | 476.989 | 1133.428 | 2.376 | 0.524 | 5 | 2.035–2.603 | fail | pass |
| allocating | vertex-v0-resident-s32 | 489.501 | 805.696 | 1.646 | 0.515 | 5 | 1.559–2.426 | fail | pass |
| allocating | view-none-resident-s32 | 487.466 | 1039.403 | 2.132 | 0.577 | 5 | 1.354–2.855 | fail | pass |
| allocating | filter-1-resident-s4 | 1186.376 | 1086.261 | 0.916 | 1.201 | 5 | 0.910–0.956 | pass | n/a |
| allocating | view-1-resident-s4 | 596.082 | 766.516 | 1.286 | 0.490 | 5 | 1.243–1.380 | pass | pass |
| allocating | filter-1-resident-s8 | 1695.621 | 1631.691 | 0.962 | 1.876 | 5 | 0.879–1.029 | pass | n/a |
| allocating | view-1-resident-s8 | 461.852 | 907.322 | 1.965 | 0.603 | 5 | 1.811–2.189 | fail | pass |
| allocating | filter-2-resident-s8 | 2023.778 | 2036.415 | 1.006 | 0.971 | 5 | 0.928–1.064 | pass | n/a |
| allocating | view-2-resident-s8 | 562.772 | 1178.636 | 2.094 | 0.562 | 5 | 2.068–2.113 | fail | pass |
| allocating | filter-3-resident-s12 | 5574.160 | 5753.328 | 1.032 | 1.113 | 5 | 0.798–1.206 | pass | pass |
| allocating | view-3-resident-s12 | 583.594 | 1456.311 | 2.495 | 0.466 | 5 | 2.462–2.608 | fail | pass |
| allocating | index-2-v0-resident-s2 | 583.219 | 583.578 | 1.001 | 0.988 | 5 | 0.960–1.065 | pass | n/a |
| allocating | index-2-v0-resident-s4 | 1156.467 | 1077.359 | 0.932 | 0.958 | 5 | 0.874–0.988 | pass | n/a |
| allocating | index-2-v1-resident-s2 | 700.577 | 684.871 | 0.978 | 0.982 | 5 | 0.944–0.984 | pass | n/a |
| allocating | index-2-v1-resident-s4 | 1143.323 | 1150.150 | 1.006 | 1.011 | 5 | 0.958–1.008 | pass | n/a |
| allocating | index-3-v0-resident-s2 | 571.688 | 639.368 | 1.118 | 1.024 | 5 | 1.044–1.224 | pass | n/a |
| allocating | index-3-v0-resident-s4 | 1329.030 | 1460.737 | 1.099 | 1.034 | 5 | 0.980–1.174 | pass | n/a |
| allocating | index-3-v1-resident-s2 | 583.457 | 645.465 | 1.106 | 0.998 | 5 | 1.048–1.208 | pass | n/a |
| allocating | index-3-v1-resident-s4 | 1139.858 | 1305.502 | 1.145 | 1.036 | 5 | 1.093–1.332 | pass | n/a |
| allocating | vertex-v0-streaming-s4 | 478.851 | 570.937 | 1.192 | 0.489 | 11 | 1.026–1.288 | pass | pass |
| allocating | view-none-streaming-s4 | 420.606 | 499.819 | 1.188 | 0.477 | 10 | 1.016–1.288 | pass | pass |
| allocating | vertex-v0-streaming-s12 | 341.453 | 521.656 | 1.528 | 0.595 | 10 | 1.333–1.931 | fail | pass |
| allocating | view-none-streaming-s12 | 408.505 | 610.193 | 1.494 | 0.645 | 10 | 1.310–2.072 | fail | pass |
| allocating | vertex-v0-streaming-s32 | 386.882 | 530.684 | 1.372 | 0.631 | 11 | 1.309–1.680 | fail | pass |
| allocating | view-none-streaming-s32 | 377.160 | 505.986 | 1.342 | 0.602 | 16 | 1.301–1.520 | fail | pass |
| allocating | filter-1-streaming-s4 | 574.772 | 577.431 | 1.005 | 1.055 | 5 | 0.629–1.266 | pass | n/a |
| allocating | view-1-streaming-s4 | 399.863 | 822.336 | 2.057 | 0.613 | 5 | 1.673–2.591 | fail | pass |
| allocating | filter-1-streaming-s8 | 834.054 | 795.363 | 0.954 | 1.084 | 5 | 0.874–1.099 | pass | n/a |
| allocating | view-1-streaming-s8 | 402.455 | 616.245 | 1.531 | 0.730 | 20+30 | 1.317–1.558 | borderline | pass |
| allocating | filter-2-streaming-s8 | 535.791 | 612.495 | 1.143 | 0.973 | 5 | 1.032–1.219 | pass | n/a |
| allocating | view-2-streaming-s8 | 242.282 | 393.397 | 1.624 | 0.708 | 20+30 | 1.349–1.556 | borderline | pass |
| allocating | filter-3-streaming-s12 | 387.872 | 409.877 | 1.057 | 0.997 | 5 | 0.891–1.174 | pass | pass |
| allocating | view-3-streaming-s12 | 357.640 | 582.729 | 1.629 | 0.638 | 20+30 | 1.309–1.546 | borderline | pass |
| allocating | index-2-v0-streaming-s2 | 193.143 | 224.997 | 1.165 | 1.031 | 12 | 1.058–1.478 | pass | n/a |
| allocating | index-2-v0-streaming-s4 | 471.771 | 927.717 | 1.966 | 0.854 | 7 | 1.592–2.572 | fail | n/a |
| allocating | index-2-v1-streaming-s2 | 631.441 | 666.533 | 1.056 | 0.895 | 6 | 0.686–1.482 | pass | n/a |
| allocating | index-2-v1-streaming-s4 | 551.104 | 1071.047 | 1.943 | 0.879 | 5 | 1.649–2.769 | fail | n/a |
| allocating | index-3-v0-streaming-s2 | 581.205 | 705.171 | 1.213 | 1.091 | 9 | 1.038–1.487 | pass | n/a |
| allocating | index-3-v0-streaming-s4 | 436.823 | 1258.714 | 2.882 | 0.977 | 5 | 2.126–2.962 | fail | n/a |
| allocating | index-3-v1-streaming-s2 | 607.726 | 693.367 | 1.141 | 1.052 | 10 | 1.037–1.478 | pass | n/a |
| allocating | index-3-v1-streaming-s4 | 475.557 | 1373.339 | 2.888 | 0.970 | 5 | 2.286–2.944 | fail | n/a |
| allocating | vertex-v1-tiny-s4 | 191.419 | 318.808 | 1.665 | 0.838 | 5 | 1.576–1.830 | fail | pass |
| allocating | vertex-v1-tiny-s12 | 183.196 | 366.002 | 1.998 | 0.954 | 5 | 1.866–2.005 | fail | FAIL |
| allocating | vertex-v1-tiny-s32 | 218.844 | 458.993 | 2.097 | 1.018 | 5 | 1.959–2.335 | fail | pass |
| allocating | vertex-v1-resident-s4 | 827.133 | 1291.517 | 1.561 | 0.333 | 6 | 1.303–1.735 | fail | pass |
| allocating | vertex-v1-resident-s12 | 623.109 | 1305.208 | 2.095 | 0.464 | 5 | 2.084–2.237 | fail | pass |
| allocating | vertex-v1-resident-s32 | 609.617 | 1322.170 | 2.169 | 0.489 | 5 | 2.096–2.255 | fail | pass |
| allocating | vertex-v1-streaming-s4 | 640.625 | 1342.190 | 2.095 | 0.540 | 5 | 2.004–2.777 | fail | pass |
| allocating | vertex-v1-streaming-s12 | 377.776 | 575.788 | 1.524 | 0.567 | 13 | 1.304–1.728 | fail | pass |
| allocating | vertex-v1-streaming-s32 | 324.060 | 533.491 | 1.646 | 0.710 | 14 | 1.335–1.823 | fail | pass |
| allocating | vertex-v0-resident-s12-level0 | 410.432 | 944.320 | 2.301 | 0.494 | 5 | 2.248–2.367 | fail | pass |
| allocating | vertex-v0-resident-s12-level9 | 454.678 | 1030.541 | 2.267 | 0.481 | 5 | 2.225–2.378 | fail | pass |
| allocating | vertex-v1-resident-s12-level0 | 399.511 | 945.758 | 2.367 | 0.529 | 5 | 2.083–2.407 | fail | pass |
| allocating | vertex-v1-resident-s12-level9 | 554.507 | 1346.260 | 2.428 | 0.471 | 5 | 1.783–3.300 | fail | pass |
| allocating | index-2-v0-millions-s4 | 254.215 | 325.933 | 1.282 | 1.078 | 5 | 1.125–1.483 | pass | n/a |
| allocating | index-2-v1-millions-s4 | 458.550 | 361.339 | 0.788 | 0.858 | 5 | 0.690–1.205 | pass | n/a |
| allocating | varied-filter-1-tiny-s4 | 189.595 | 289.991 | 1.530 | 0.218 | 20+30 | 1.385–1.746 | borderline | pass |
| allocating | varied-view-1-tiny-s4 | 95.198 | 187.738 | 1.972 | 0.439 | 5 | 1.718–2.452 | fail | pass |
| allocating | varied-filter-1-tiny-s8 | 441.925 | 567.024 | 1.283 | 0.235 | 5 | 1.250–1.354 | pass | pass |
| allocating | varied-view-1-tiny-s8 | 114.606 | 203.909 | 1.779 | 0.511 | 5 | 1.541–2.115 | fail | pass |
| allocating | varied-filter-2-tiny-s8 | 477.725 | 648.985 | 1.358 | 0.242 | 5 | 1.301–1.390 | pass | pass |
| allocating | varied-view-2-tiny-s8 | 107.447 | 205.601 | 1.914 | 0.558 | 5 | 1.787–1.986 | fail | pass |
| allocating | varied-filter-3-tiny-s4 | 735.055 | 639.458 | 0.870 | 1.052 | 5 | 0.852–0.900 | pass | pass |
| allocating | varied-view-3-tiny-s4 | 112.880 | 174.132 | 1.543 | 0.854 | 5 | 1.544–1.666 | fail | pass |
| allocating | varied-filter-3-tiny-s12 | 1756.856 | 1597.851 | 0.909 | 0.982 | 5 | 0.876–0.911 | pass | pass |
| allocating | varied-view-3-tiny-s12 | 144.696 | 268.607 | 1.856 | 0.870 | 5 | 1.766–2.018 | fail | pass |
| allocating | varied-filter-3-tiny-s32 | 3452.320 | 3198.096 | 0.926 | 0.920 | 5 | 0.889–0.987 | pass | pass |
| allocating | varied-view-3-tiny-s32 | 152.091 | 293.424 | 1.929 | 0.811 | 5 | 1.650–2.020 | fail | FAIL |
| allocating | varied-filter-4-tiny-s4 | 305.655 | 465.019 | 1.521 | 0.808 | 20+30 | 1.482–1.557 | borderline | pass |
| allocating | varied-filter-4-tiny-s8 | 565.354 | 775.009 | 1.371 | 0.856 | 5 | 1.367–1.416 | pass | pass |
| allocating | varied-filter-1-resident-s4 | 547.706 | 941.479 | 1.719 | 0.102 | 5 | 1.683–1.836 | fail | pass |
| allocating | varied-view-1-resident-s4 | 318.410 | 602.958 | 1.894 | 0.140 | 5 | 1.752–2.033 | fail | pass |
| allocating | varied-filter-1-resident-s8 | 1113.170 | 1425.924 | 1.281 | 0.110 | 5 | 1.085–1.385 | pass | pass |
| allocating | varied-view-1-resident-s8 | 447.668 | 887.652 | 1.983 | 0.222 | 5 | 1.808–2.180 | fail | pass |
| allocating | varied-filter-2-resident-s8 | 1307.574 | 1532.143 | 1.172 | 0.108 | 5 | 1.161–1.217 | pass | pass |
| allocating | varied-view-2-resident-s8 | 366.653 | 765.454 | 2.088 | 0.267 | 5 | 2.007–2.165 | fail | pass |
| allocating | varied-filter-3-resident-s4 | 6284.089 | 4907.599 | 0.781 | 0.864 | 5 | 0.762–0.868 | pass | pass |
| allocating | varied-view-3-resident-s4 | 839.844 | 1250.454 | 1.489 | 0.347 | 20+30 | 1.387–1.507 | borderline | pass |
| allocating | varied-filter-3-resident-s12 | 7872.257 | 6029.748 | 0.766 | 1.015 | 5 | 0.736–0.904 | pass | pass |
| allocating | varied-view-3-resident-s12 | 568.745 | 1244.427 | 2.188 | 0.459 | 5 | 2.119–2.216 | fail | pass |
| allocating | varied-filter-3-resident-s32 | 6254.179 | 5531.852 | 0.885 | 1.119 | 5 | 0.849–0.966 | pass | pass |
| allocating | varied-view-3-resident-s32 | 536.887 | 1169.925 | 2.179 | 0.470 | 5 | 1.831–2.670 | fail | pass |
| allocating | varied-filter-4-resident-s4 | 811.692 | 1344.524 | 1.656 | 0.665 | 6 | 1.527–1.820 | fail | pass |
| allocating | varied-filter-4-resident-s8 | 1399.507 | 1839.986 | 1.315 | 0.712 | 6 | 1.219–1.480 | pass | pass |
| allocating | varied-filter-1-streaming-s4 | 629.891 | 1227.333 | 1.948 | 0.102 | 5 | 1.718–2.044 | fail | pass |
| allocating | varied-view-1-streaming-s4 | 408.249 | 686.614 | 1.682 | 0.121 | 5 | 1.590–1.819 | fail | pass |
| allocating | varied-filter-1-streaming-s8 | 1289.247 | 1688.796 | 1.310 | 0.105 | 5 | 1.295–1.459 | pass | pass |
| allocating | varied-view-1-streaming-s8 | 449.549 | 903.052 | 2.009 | 0.197 | 5 | 1.665–2.906 | fail | pass |
| allocating | varied-filter-2-streaming-s8 | 1655.386 | 1967.613 | 1.189 | 0.095 | 6 | 1.048–1.453 | pass | pass |
| allocating | varied-view-2-streaming-s8 | 498.252 | 934.869 | 1.876 | 0.181 | 5 | 1.755–2.342 | fail | pass |
| allocating | varied-filter-3-streaming-s4 | 7130.148 | 6401.349 | 0.898 | 1.009 | 5 | 0.749–1.037 | pass | pass |
| allocating | varied-view-3-streaming-s4 | 1177.654 | 1696.479 | 1.441 | 0.336 | 6 | 1.367–1.489 | pass | pass |
| allocating | varied-filter-3-streaming-s12 | 8548.929 | 7251.110 | 0.848 | 1.023 | 5 | 0.752–1.018 | pass | pass |
| allocating | varied-view-3-streaming-s12 | 833.020 | 1692.423 | 2.032 | 0.328 | 5 | 1.870–2.180 | fail | pass |
| allocating | varied-filter-3-streaming-s32 | 1258.875 | 968.760 | 0.770 | 0.797 | 5 | 0.524–1.395 | pass | pass |
| allocating | varied-view-3-streaming-s32 | 487.328 | 1463.376 | 3.003 | 0.480 | 5 | 2.409–3.394 | fail | pass |
| allocating | varied-filter-4-streaming-s4 | 782.783 | 1501.654 | 1.918 | 0.755 | 5 | 1.538–1.909 | fail | pass |
| allocating | varied-filter-4-streaming-s8 | 1491.682 | 2287.144 | 1.533 | 0.723 | 20+30 | 1.381–1.528 | borderline | pass |
| allocating | meshlet-1-1-v2-t3 | 44.152 | 55.326 | 1.253 | 0.973 | 20+30 | 1.259–1.349 | borderline | pass |
| allocating | meshlet-1-1-v2-t4 | 49.695 | 66.887 | 1.346 | 1.027 | 20+30 | 1.206–1.332 | borderline | pass |
| allocating | meshlet-1-1-v4-t3 | 59.988 | 77.608 | 1.294 | 0.999 | 20+30 | 1.279–1.380 | borderline | pass |
| allocating | meshlet-1-1-v4-t4 | 72.343 | 89.223 | 1.233 | 0.968 | 8 | 1.215–1.300 | pass | pass |
| allocating | meshlet-raw-1-1 | 86.335 | 92.749 | 1.074 | 0.837 | 5 | 1.027–1.119 | pass | pass |
| allocating | meshlet-64-126-v2-t3 | 1075.533 | 1568.498 | 1.458 | 0.366 | 12 | 1.303–1.544 | fail | pass |
| allocating | meshlet-64-126-v2-t4 | 1703.082 | 2113.962 | 1.241 | 0.308 | 20+30 | 1.259–1.318 | borderline | pass |
| allocating | meshlet-64-126-v4-t3 | 941.557 | 1372.095 | 1.457 | 0.324 | 5 | 1.450–1.656 | fail | pass |
| allocating | meshlet-64-126-v4-t4 | 2081.126 | 2427.790 | 1.167 | 0.276 | 20+30 | 1.346–1.464 | fail | pass |
| allocating | meshlet-raw-64-126 | 1561.642 | 2138.517 | 1.369 | 0.327 | 10 | 1.302–1.514 | fail | pass |
| allocating | meshlet-256-256-v2-t3 | 925.648 | 1407.399 | 1.520 | 0.290 | 5 | 1.497–1.579 | fail | pass |
| allocating | meshlet-256-256-v2-t4 | 1051.761 | 1441.461 | 1.371 | 0.291 | 11 | 1.312–1.856 | fail | pass |
| allocating | meshlet-256-256-v4-t3 | 1200.410 | 1728.871 | 1.440 | 0.264 | 17 | 1.302–1.521 | fail | pass |
| allocating | meshlet-256-256-v4-t4 | 1460.341 | 1784.468 | 1.222 | 0.291 | 9 | 1.076–1.294 | pass | pass |
| allocating | meshlet-raw-256-256 | 2127.605 | 2680.780 | 1.260 | 0.272 | 12 | 1.140–1.296 | pass | pass |
| caller-buffer | vertex-v0-tiny-s4 | 267.530 | 500.111 | 1.869 | 0.841 | 5 | 1.794–1.939 | fail | pass |
| caller-buffer | view-none-tiny-s4 | 281.565 | 508.252 | 1.805 | 0.808 | 5 | 1.748–1.915 | fail | pass |
| caller-buffer | vertex-v0-tiny-s12 | 219.025 | 481.779 | 2.200 | 0.990 | 5 | 1.619–2.369 | fail | pass |
| caller-buffer | view-none-tiny-s12 | 186.746 | 364.609 | 1.952 | 0.824 | 5 | 1.918–1.998 | fail | pass |
| caller-buffer | vertex-v0-tiny-s32 | 284.935 | 546.903 | 1.919 | 0.856 | 5 | 1.860–1.986 | fail | pass |
| caller-buffer | view-none-tiny-s32 | 211.848 | 441.220 | 2.083 | 0.942 | 5 | 1.749–2.248 | fail | pass |
| caller-buffer | filter-1-tiny-s4 | 448.374 | 552.886 | 1.233 | 1.178 | 5 | 1.206–1.290 | pass | n/a |
| caller-buffer | view-1-tiny-s4 | 121.180 | 195.311 | 1.612 | 1.046 | 10 | 1.510–1.759 | fail | FAIL |
| caller-buffer | filter-1-tiny-s8 | 658.711 | 671.618 | 1.020 | 1.285 | 5 | 0.875–1.438 | pass | n/a |
| caller-buffer | view-1-tiny-s8 | 151.328 | 236.814 | 1.565 | 1.183 | 6 | 1.525–1.743 | fail | FAIL |
| caller-buffer | filter-2-tiny-s8 | 571.149 | 756.263 | 1.324 | 1.090 | 6 | 1.088–1.434 | pass | n/a |
| caller-buffer | view-2-tiny-s8 | 180.058 | 301.054 | 1.672 | 1.128 | 5 | 1.602–1.856 | fail | FAIL |
| caller-buffer | filter-3-tiny-s12 | 3248.012 | 3037.284 | 0.935 | 1.041 | 5 | 0.865–0.993 | pass | pass |
| caller-buffer | view-3-tiny-s12 | 225.930 | 382.543 | 1.693 | 1.020 | 9 | 1.530–2.013 | fail | FAIL |
| caller-buffer | index-2-v0-tiny-s2 | 379.414 | 413.457 | 1.090 | 1.062 | 5 | 0.978–1.259 | pass | n/a |
| caller-buffer | index-2-v0-tiny-s4 | 2369.714 | 2794.155 | 1.179 | 0.993 | 5 | 1.127–1.244 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s2 | 1194.259 | 1378.099 | 1.154 | 1.018 | 5 | 1.101–1.177 | pass | n/a |
| caller-buffer | index-2-v1-tiny-s4 | 2418.398 | 2832.760 | 1.171 | 0.997 | 5 | 1.069–1.363 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s2 | 1342.230 | 1289.592 | 0.961 | 1.000 | 5 | 0.912–1.066 | pass | n/a |
| caller-buffer | index-3-v0-tiny-s4 | 2711.549 | 2501.409 | 0.923 | 0.994 | 5 | 0.863–1.033 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s2 | 1352.060 | 1298.197 | 0.960 | 0.974 | 5 | 0.874–1.212 | pass | n/a |
| caller-buffer | index-3-v1-tiny-s4 | 2577.302 | 2511.460 | 0.974 | 0.977 | 5 | 0.919–0.981 | pass | n/a |
| caller-buffer | vertex-v0-resident-s4 | 2610.006 | 3956.073 | 1.516 | 0.340 | 5 | 1.495–1.554 | fail | pass |
| caller-buffer | view-none-resident-s4 | 2623.855 | 3952.044 | 1.506 | 0.340 | 5 | 1.423–1.572 | fail | pass |
| caller-buffer | vertex-v0-resident-s12 | 1788.842 | 3986.546 | 2.229 | 0.487 | 5 | 2.177–2.268 | fail | pass |
| caller-buffer | view-none-resident-s12 | 1834.864 | 4049.825 | 2.207 | 0.482 | 5 | 2.141–2.325 | fail | pass |
| caller-buffer | vertex-v0-resident-s32 | 1776.072 | 4061.802 | 2.287 | 0.499 | 5 | 2.196–2.284 | fail | pass |
| caller-buffer | view-none-resident-s32 | 1676.833 | 3794.131 | 2.263 | 0.485 | 5 | 2.075–2.345 | fail | pass |
| caller-buffer | filter-1-resident-s4 | 3692.169 | 3575.241 | 0.968 | 1.164 | 5 | 0.937–0.987 | pass | n/a |
| caller-buffer | view-1-resident-s4 | 1987.405 | 2493.900 | 1.255 | 0.444 | 5 | 1.201–1.415 | pass | pass |
| caller-buffer | filter-1-resident-s8 | 5853.462 | 5775.316 | 0.987 | 1.833 | 5 | 0.815–1.371 | pass | n/a |
| caller-buffer | view-1-resident-s8 | 1787.423 | 3377.528 | 1.890 | 0.572 | 5 | 1.862–1.932 | fail | pass |
| caller-buffer | filter-2-resident-s8 | 6056.926 | 6597.625 | 1.089 | 1.018 | 5 | 1.076–1.151 | pass | n/a |
| caller-buffer | view-2-resident-s8 | 1821.768 | 3763.867 | 2.066 | 0.533 | 5 | 2.021–2.135 | fail | pass |
| caller-buffer | filter-3-resident-s12 | 19620.474 | 18194.081 | 0.927 | 1.006 | 5 | 0.859–1.068 | pass | pass |
| caller-buffer | view-3-resident-s12 | 2179.190 | 5126.249 | 2.352 | 0.465 | 5 | 2.382–2.575 | fail | pass |
| caller-buffer | index-2-v0-resident-s2 | 2184.145 | 2118.365 | 0.970 | 0.998 | 5 | 0.887–1.047 | pass | n/a |
| caller-buffer | index-2-v0-resident-s4 | 4373.280 | 4111.614 | 0.940 | 0.988 | 5 | 0.911–0.965 | pass | n/a |
| caller-buffer | index-2-v1-resident-s2 | 2145.667 | 2119.629 | 0.988 | 0.996 | 5 | 0.971–1.024 | pass | n/a |
| caller-buffer | index-2-v1-resident-s4 | 4332.591 | 4153.786 | 0.959 | 1.003 | 5 | 0.889–1.017 | pass | n/a |
| caller-buffer | index-3-v0-resident-s2 | 2084.568 | 2376.279 | 1.140 | 1.002 | 5 | 1.101–1.178 | pass | n/a |
| caller-buffer | index-3-v0-resident-s4 | 4021.895 | 4486.239 | 1.115 | 1.000 | 5 | 1.036–1.232 | pass | n/a |
| caller-buffer | index-3-v1-resident-s2 | 2072.630 | 2253.264 | 1.087 | 0.982 | 5 | 1.059–1.136 | pass | n/a |
| caller-buffer | index-3-v1-resident-s4 | 4145.689 | 4602.182 | 1.110 | 0.994 | 5 | 1.101–1.155 | pass | n/a |
| caller-buffer | vertex-v0-streaming-s4 | 2688.504 | 4069.498 | 1.514 | 0.340 | 5 | 1.492–1.530 | fail | pass |
| caller-buffer | view-none-streaming-s4 | 2733.090 | 4092.422 | 1.497 | 0.337 | 5 | 1.473–1.521 | fail | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1690.457 | 3343.024 | 1.978 | 0.509 | 5 | 1.310–2.811 | fail | pass |
| caller-buffer | view-none-streaming-s12 | 1778.814 | 3119.138 | 1.753 | 0.490 | 5 | 1.688–1.858 | fail | pass |
| caller-buffer | vertex-v0-streaming-s32 | 1884.091 | 3746.889 | 1.989 | 0.487 | 5 | 1.924–2.059 | fail | pass |
| caller-buffer | view-none-streaming-s32 | 1882.212 | 3660.548 | 1.945 | 0.488 | 5 | 1.932–2.021 | fail | pass |
| caller-buffer | filter-1-streaming-s4 | 3989.141 | 3912.016 | 0.981 | 1.175 | 5 | 0.959–0.996 | pass | n/a |
| caller-buffer | view-1-streaming-s4 | 2198.614 | 2843.623 | 1.293 | 0.428 | 5 | 1.263–1.306 | pass | pass |
| caller-buffer | filter-1-streaming-s8 | 5541.923 | 5436.526 | 0.981 | 1.732 | 5 | 0.947–1.020 | pass | n/a |
| caller-buffer | view-1-streaming-s8 | 1897.102 | 3830.075 | 2.019 | 0.562 | 5 | 1.982–2.044 | fail | pass |
| caller-buffer | filter-2-streaming-s8 | 5835.764 | 6306.447 | 1.081 | 0.986 | 5 | 1.050–1.115 | pass | n/a |
| caller-buffer | view-2-streaming-s8 | 1943.162 | 4246.432 | 2.185 | 0.515 | 5 | 2.143–2.210 | fail | pass |
| caller-buffer | filter-3-streaming-s12 | 4132.692 | 4026.350 | 0.974 | 0.973 | 5 | 0.963–0.999 | pass | pass |
| caller-buffer | view-3-streaming-s12 | 2287.224 | 5139.460 | 2.247 | 0.464 | 5 | 1.982–2.364 | fail | pass |
| caller-buffer | index-2-v0-streaming-s2 | 1108.661 | 1422.311 | 1.283 | 1.051 | 5 | 1.235–1.324 | pass | n/a |
| caller-buffer | index-2-v0-streaming-s4 | 2168.458 | 2799.507 | 1.291 | 1.058 | 5 | 1.196–1.341 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s2 | 2198.460 | 2135.924 | 0.972 | 1.002 | 5 | 0.966–0.985 | pass | n/a |
| caller-buffer | index-2-v1-streaming-s4 | 4397.620 | 4200.882 | 0.955 | 0.996 | 5 | 0.948–0.973 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s2 | 2110.579 | 2286.123 | 1.083 | 0.982 | 5 | 1.064–1.135 | pass | n/a |
| caller-buffer | index-3-v0-streaming-s4 | 4215.639 | 4752.188 | 1.127 | 1.008 | 5 | 1.118–1.140 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s2 | 2093.561 | 2370.188 | 1.132 | 1.004 | 5 | 1.126–1.140 | pass | n/a |
| caller-buffer | index-3-v1-streaming-s4 | 4000.374 | 4475.799 | 1.119 | 1.001 | 5 | 1.105–1.151 | pass | n/a |
| caller-buffer | vertex-v1-tiny-s4 | 671.351 | 1231.279 | 1.834 | 0.858 | 5 | 1.714–1.971 | fail | pass |
| caller-buffer | vertex-v1-tiny-s12 | 759.774 | 1484.824 | 1.954 | 0.901 | 5 | 1.608–2.166 | fail | FAIL |
| caller-buffer | vertex-v1-tiny-s32 | 816.422 | 1532.874 | 1.878 | 0.874 | 5 | 1.769–1.952 | fail | FAIL |
| caller-buffer | vertex-v1-resident-s4 | 2701.235 | 4105.975 | 1.520 | 0.341 | 5 | 1.486–1.542 | fail | pass |
| caller-buffer | vertex-v1-resident-s12 | 1874.238 | 4155.021 | 2.217 | 0.490 | 5 | 2.197–2.236 | fail | pass |
| caller-buffer | vertex-v1-resident-s32 | 1880.452 | 4199.461 | 2.233 | 0.492 | 5 | 2.188–2.316 | fail | pass |
| caller-buffer | vertex-v1-streaming-s4 | 2698.508 | 3909.486 | 1.449 | 0.334 | 5 | 1.343–1.551 | fail | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1820.001 | 3939.380 | 2.164 | 0.488 | 5 | 2.034–2.335 | fail | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1643.247 | 3498.673 | 2.129 | 0.512 | 5 | 1.776–3.240 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1893.637 | 4197.255 | 2.217 | 0.491 | 5 | 2.196–2.292 | fail | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1840.991 | 4023.023 | 2.185 | 0.493 | 5 | 2.169–2.262 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1861.557 | 4199.265 | 2.256 | 0.500 | 5 | 2.120–2.349 | fail | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1898.479 | 4104.735 | 2.162 | 0.491 | 5 | 2.110–2.299 | fail | pass |
| caller-buffer | index-2-v0-millions-s4 | 2200.581 | 2823.256 | 1.283 | 1.045 | 5 | 1.255–1.317 | pass | n/a |
| caller-buffer | index-2-v1-millions-s4 | 4359.486 | 4065.974 | 0.933 | 1.001 | 5 | 0.908–0.974 | pass | n/a |
| caller-buffer | varied-filter-1-tiny-s4 | 907.823 | 1711.233 | 1.885 | 0.319 | 5 | 1.740–2.047 | fail | pass |
| caller-buffer | varied-view-1-tiny-s4 | 438.027 | 743.479 | 1.697 | 0.464 | 5 | 1.657–1.768 | fail | pass |
| caller-buffer | varied-filter-1-tiny-s8 | 2006.325 | 2290.292 | 1.142 | 0.295 | 5 | 1.115–1.151 | pass | pass |
| caller-buffer | varied-view-1-tiny-s8 | 566.865 | 921.163 | 1.625 | 0.579 | 5 | 1.526–1.731 | fail | pass |
| caller-buffer | varied-filter-2-tiny-s8 | 2027.137 | 2593.039 | 1.279 | 0.290 | 5 | 1.223–1.310 | pass | pass |
| caller-buffer | varied-view-2-tiny-s8 | 582.748 | 1012.408 | 1.737 | 0.567 | 5 | 1.693–1.796 | fail | pass |
| caller-buffer | varied-filter-3-tiny-s4 | 4148.517 | 3670.553 | 0.885 | 0.982 | 5 | 0.682–1.093 | pass | pass |
| caller-buffer | varied-view-3-tiny-s4 | 669.676 | 1042.481 | 1.557 | 0.912 | 20+30 | 1.458–1.677 | borderline | pass |
| caller-buffer | varied-filter-3-tiny-s12 | 10230.205 | 9592.115 | 0.938 | 0.898 | 5 | 0.765–1.254 | pass | pass |
| caller-buffer | varied-view-3-tiny-s12 | 731.801 | 1319.509 | 1.803 | 0.947 | 5 | 1.755–1.869 | fail | FAIL |
| caller-buffer | varied-filter-3-tiny-s32 | 20501.754 | 21079.747 | 1.028 | 0.995 | 5 | 0.929–1.273 | pass | pass |
| caller-buffer | varied-view-3-tiny-s32 | 810.591 | 1491.288 | 1.840 | 0.837 | 5 | 1.693–1.927 | fail | FAIL |
| caller-buffer | varied-filter-4-tiny-s4 | 1472.378 | 2095.695 | 1.423 | 0.832 | 5 | 1.375–1.446 | pass | pass |
| caller-buffer | varied-filter-4-tiny-s8 | 2528.983 | 2939.664 | 1.162 | 0.872 | 5 | 1.072–1.245 | pass | pass |
| caller-buffer | varied-filter-1-resident-s4 | 2132.110 | 3773.645 | 1.770 | 0.145 | 5 | 1.737–1.824 | fail | pass |
| caller-buffer | varied-view-1-resident-s4 | 1381.053 | 2395.863 | 1.735 | 0.167 | 5 | 1.731–1.826 | fail | pass |
| caller-buffer | varied-filter-1-resident-s8 | 3810.760 | 5269.529 | 1.383 | 0.149 | 5 | 1.333–1.455 | pass | pass |
| caller-buffer | varied-view-1-resident-s8 | 1305.793 | 2699.900 | 2.068 | 0.262 | 5 | 2.002–2.114 | fail | pass |
| caller-buffer | varied-filter-2-resident-s8 | 4761.793 | 6529.676 | 1.371 | 0.130 | 5 | 1.353–1.384 | pass | pass |
| caller-buffer | varied-view-2-resident-s8 | 1467.678 | 3038.187 | 2.070 | 0.249 | 5 | 1.947–2.129 | fail | pass |
| caller-buffer | varied-filter-3-resident-s4 | 17139.912 | 16858.315 | 0.984 | 1.070 | 5 | 0.939–1.020 | pass | FAIL |
| caller-buffer | varied-view-3-resident-s4 | 2634.162 | 3934.657 | 1.494 | 0.339 | 20+30 | 1.458–1.533 | borderline | pass |
| caller-buffer | varied-filter-3-resident-s12 | 17856.801 | 17188.117 | 0.963 | 1.045 | 5 | 0.926–0.999 | pass | pass |
| caller-buffer | varied-view-3-resident-s12 | 1724.295 | 3738.481 | 2.168 | 0.482 | 5 | 2.145–2.225 | fail | pass |
| caller-buffer | varied-filter-3-resident-s32 | 19638.988 | 15806.852 | 0.805 | 1.001 | 5 | 0.767–0.902 | pass | pass |
| caller-buffer | varied-view-3-resident-s32 | 1815.275 | 3893.302 | 2.145 | 0.455 | 5 | 2.038–2.228 | fail | pass |
| caller-buffer | varied-filter-4-resident-s4 | 2543.655 | 4445.343 | 1.748 | 0.689 | 5 | 1.687–1.777 | fail | pass |
| caller-buffer | varied-filter-4-resident-s8 | 4600.423 | 6942.997 | 1.509 | 0.734 | 20+30 | 1.458–1.504 | borderline | pass |
| caller-buffer | varied-filter-1-streaming-s4 | 2162.848 | 3781.218 | 1.748 | 0.095 | 5 | 1.500–1.947 | fail | pass |
| caller-buffer | varied-view-1-streaming-s4 | 1419.892 | 2483.360 | 1.749 | 0.124 | 7 | 1.511–1.875 | fail | pass |
| caller-buffer | varied-filter-1-streaming-s8 | 4159.106 | 5913.244 | 1.422 | 0.104 | 5 | 1.379–1.435 | pass | pass |
| caller-buffer | varied-view-1-streaming-s8 | 1461.436 | 2996.484 | 2.050 | 0.189 | 5 | 2.026–2.094 | fail | pass |
| caller-buffer | varied-filter-2-streaming-s8 | 5076.098 | 6908.316 | 1.361 | 0.101 | 5 | 1.334–1.379 | pass | pass |
| caller-buffer | varied-view-2-streaming-s8 | 1566.906 | 3283.150 | 2.095 | 0.185 | 5 | 2.056–2.127 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s4 | 20542.737 | 19332.886 | 0.941 | 1.013 | 5 | 0.913–0.958 | pass | pass |
| caller-buffer | varied-view-3-streaming-s4 | 2672.474 | 3954.203 | 1.480 | 0.322 | 7 | 1.443–1.495 | pass | pass |
| caller-buffer | varied-filter-3-streaming-s12 | 21627.749 | 20301.786 | 0.939 | 1.017 | 5 | 0.919–0.949 | pass | FAIL |
| caller-buffer | varied-view-3-streaming-s12 | 1918.355 | 4138.360 | 2.157 | 0.344 | 5 | 2.104–2.218 | fail | pass |
| caller-buffer | varied-filter-3-streaming-s32 | 21187.857 | 19558.377 | 0.923 | 1.010 | 5 | 0.880–0.966 | pass | pass |
| caller-buffer | varied-view-3-streaming-s32 | 1929.354 | 4188.648 | 2.171 | 0.342 | 5 | 2.141–2.203 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s4 | 2828.510 | 4896.383 | 1.731 | 0.685 | 5 | 1.704–1.763 | fail | pass |
| caller-buffer | varied-filter-4-streaming-s8 | 5203.225 | 7721.986 | 1.484 | 0.713 | 5 | 1.482–1.498 | pass | pass |
| caller-buffer | meshlet-1-1-v2-t3 | 257.383 | 497.637 | 1.933 | 1.143 | 5 | 1.521–2.559 | fail | pass |
| caller-buffer | meshlet-1-1-v2-t4 | 314.743 | 606.031 | 1.925 | 1.125 | 5 | 1.761–2.291 | fail | pass |
| caller-buffer | meshlet-1-1-v4-t3 | 354.409 | 678.545 | 1.915 | 1.162 | 5 | 1.499–2.314 | fail | pass |
| caller-buffer | meshlet-1-1-v4-t4 | 412.298 | 794.650 | 1.927 | 1.165 | 5 | 1.485–2.276 | fail | pass |
| caller-buffer | meshlet-raw-1-1 | 601.204 | 847.421 | 1.410 | 0.848 | 18 | 1.303–1.597 | fail | pass |
| caller-buffer | meshlet-64-126-v2-t3 | 4506.051 | 6774.527 | 1.503 | 0.316 | 5 | 1.379–1.583 | fail | pass |
| caller-buffer | meshlet-64-126-v2-t4 | 6378.284 | 8605.622 | 1.349 | 0.291 | 5 | 1.334–1.362 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t3 | 5628.587 | 8448.529 | 1.501 | 0.314 | 5 | 1.396–1.714 | fail | pass |
| caller-buffer | meshlet-64-126-v4-t4 | 7428.938 | 10305.730 | 1.387 | 0.303 | 6 | 1.316–1.418 | fail | pass |
| caller-buffer | meshlet-raw-64-126 | 7956.006 | 10472.921 | 1.316 | 0.286 | 20+30 | 1.281–1.340 | borderline | pass |
| caller-buffer | meshlet-256-256-v2-t3 | 5114.463 | 7118.494 | 1.392 | 0.283 | 5 | 1.367–1.426 | fail | pass |
| caller-buffer | meshlet-256-256-v2-t4 | 6873.676 | 8535.220 | 1.242 | 0.259 | 5 | 1.154–1.294 | pass | pass |
| caller-buffer | meshlet-256-256-v4-t3 | 7368.290 | 9970.219 | 1.353 | 0.276 | 18 | 1.302–1.378 | fail | pass |
| caller-buffer | meshlet-256-256-v4-t4 | 8565.597 | 10727.784 | 1.252 | 0.276 | 19 | 1.196–1.295 | pass | pass |
| caller-buffer | meshlet-raw-256-256 | 9020.714 | 11057.851 | 1.226 | 0.265 | 8 | 1.138–1.297 | pass | pass |

## S4 scalar C++ case intervals

The original family means and medians stay fixed. Only scalar-baseline borderline cases receive additional pairs, up to twenty total; remaining borderline cases alone receive thirty fresh D146 pairs. No clear failure is retried.

| API | Case | Original pairs | Additional stage-1 pairs | Fresh stage-2 pairs | Final interval | S4 maximum |
|---|---|---:|---:|---:|---|---|
| allocating | index-2-v0-tiny-s2 | 5 | 0 | 0 | 1.068–1.185 | pass |
| allocating | index-2-v0-tiny-s4 | 5 | 0 | 0 | 1.044–1.233 | pass |
| allocating | index-2-v1-tiny-s2 | 5 | 0 | 0 | 1.001–1.426 | pass |
| allocating | index-2-v1-tiny-s4 | 5 | 0 | 0 | 0.921–1.314 | pass |
| allocating | index-3-v0-tiny-s2 | 6 | 0 | 0 | 0.978–1.482 | pass |
| allocating | index-3-v0-tiny-s4 | 5 | 1 | 0 | 0.897–1.456 | pass |
| allocating | index-3-v1-tiny-s2 | 5 | 0 | 0 | 1.231–1.317 | pass |
| allocating | index-3-v1-tiny-s4 | 5 | 0 | 0 | 1.240–1.359 | pass |
| allocating | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.899–1.021 | pass |
| allocating | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.721–0.999 | pass |
| allocating | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.888–0.965 | pass |
| allocating | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.926–0.989 | pass |
| allocating | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.073–1.218 | pass |
| allocating | index-3-v0-resident-s4 | 5 | 0 | 0 | 0.917–1.191 | pass |
| allocating | index-3-v1-resident-s2 | 5 | 0 | 0 | 1.049–1.254 | pass |
| allocating | index-3-v1-resident-s4 | 5 | 0 | 0 | 0.914–1.332 | pass |
| allocating | index-2-v0-streaming-s2 | 12 | 0 | 0 | 1.062–1.379 | pass |
| allocating | index-2-v0-streaming-s4 | 7 | 0 | 0 | 1.528–2.549 | fail |
| allocating | index-2-v1-streaming-s2 | 6 | 0 | 0 | 0.710–1.393 | pass |
| allocating | index-2-v1-streaming-s4 | 5 | 9 | 0 | 0.969–1.456 | pass |
| allocating | index-3-v0-streaming-s2 | 9 | 0 | 0 | 1.033–1.405 | pass |
| allocating | index-3-v0-streaming-s4 | 5 | 0 | 0 | 1.732–3.461 | fail |
| allocating | index-3-v1-streaming-s2 | 10 | 0 | 0 | 0.932–1.479 | pass |
| allocating | index-3-v1-streaming-s4 | 5 | 0 | 0 | 1.664–3.861 | fail |
| allocating | index-2-v0-millions-s4 | 5 | 10 | 0 | 1.139–1.496 | pass |
| allocating | index-2-v1-millions-s4 | 5 | 0 | 0 | 0.778–1.205 | pass |
| caller-buffer | index-2-v0-tiny-s2 | 5 | 0 | 0 | 0.898–1.359 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 5 | 0 | 0 | 1.111–1.182 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 5 | 0 | 0 | 1.096–1.160 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 5 | 0 | 0 | 0.970–1.305 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 5 | 0 | 0 | 0.862–1.063 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 5 | 0 | 0 | 0.783–1.086 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 5 | 0 | 0 | 0.818–1.227 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 5 | 0 | 0 | 0.904–0.952 | pass |
| caller-buffer | index-2-v0-resident-s2 | 5 | 0 | 0 | 0.943–1.031 | pass |
| caller-buffer | index-2-v0-resident-s4 | 5 | 0 | 0 | 0.898–0.948 | pass |
| caller-buffer | index-2-v1-resident-s2 | 5 | 0 | 0 | 0.962–1.008 | pass |
| caller-buffer | index-2-v1-resident-s4 | 5 | 0 | 0 | 0.859–1.002 | pass |
| caller-buffer | index-3-v0-resident-s2 | 5 | 0 | 0 | 1.071–1.179 | pass |
| caller-buffer | index-3-v0-resident-s4 | 5 | 0 | 0 | 1.035–1.216 | pass |
| caller-buffer | index-3-v1-resident-s2 | 5 | 0 | 0 | 1.108–1.129 | pass |
| caller-buffer | index-3-v1-resident-s4 | 5 | 0 | 0 | 1.079–1.178 | pass |
| caller-buffer | index-2-v0-streaming-s2 | 5 | 0 | 0 | 1.263–1.307 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 5 | 0 | 0 | 1.273–1.322 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 5 | 0 | 0 | 0.960–0.989 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 5 | 0 | 0 | 0.918–0.952 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 5 | 0 | 0 | 1.090–1.150 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 5 | 0 | 0 | 1.077–1.136 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 5 | 0 | 0 | 1.122–1.135 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 5 | 0 | 0 | 1.106–1.170 | pass |
| caller-buffer | index-2-v0-millions-s4 | 5 | 0 | 0 | 1.267–1.318 | pass |
| caller-buffer | index-2-v1-millions-s4 | 5 | 0 | 0 | 0.899–0.965 | pass |

## Node shipped SIMD decoder

| API | Family | Cases | Time Rust/upstream SIMD GM | Worst | Time Rust/scalar GM |
|---|---|---:|---:|---:|---:|
| allocating | index | 14 | 1.080 | 1.542 | 0.963 |
| allocating | sequence | 12 | 1.001 | 1.136 | 0.986 |
| allocating | vertex | 22 | 1.313 | 1.698 | 0.519 |
| allocating | view-filtered | 30 | 1.615 | 2.537 | 0.514 |
| allocating | view-none | 9 | 1.255 | 1.568 | 0.535 |
| caller-buffer | index | 14 | 1.071 | 1.554 | 0.994 |
| caller-buffer | sequence | 12 | 0.936 | 1.028 | 0.999 |
| caller-buffer | vertex | 22 | 1.370 | 1.838 | 0.488 |
| caller-buffer | view-filtered | 30 | 1.778 | 2.740 | 0.485 |
| caller-buffer | view-none | 9 | 1.317 | 1.625 | 0.487 |

| API | Case | Time Rust/upstream SIMD | Time Rust/scalar | Pairs | Final interval | Case bar |
|---|---|---:|---:|---:|---|---|
| allocating | vertex-v0-tiny-s4 | 1.228 | 0.868 | 20+30 | 0.977–1.115 | pass |
| allocating | view-none-tiny-s4 | 1.101 | 0.864 | 5 | 0.788–1.446 | pass |
| allocating | vertex-v0-tiny-s12 | 1.320 | 0.813 | 5 | 1.122–1.456 | pass |
| allocating | view-none-tiny-s12 | 1.518 | 0.855 | 6 | 1.322–1.577 | pass |
| allocating | vertex-v0-tiny-s32 | 1.332 | 0.740 | 5 | 1.319–1.485 | pass |
| allocating | view-none-tiny-s32 | 1.568 | 0.758 | 20+30 | 1.496–1.685 | fail |
| allocating | view-1-tiny-s4 | 0.834 | 0.947 | 5 | 0.579–1.095 | pass |
| allocating | view-1-tiny-s8 | 1.380 | 1.048 | 20+30 | 1.427–1.649 | fail |
| allocating | view-2-tiny-s8 | 1.105 | 0.870 | 14 | 0.599–1.597 | pass |
| allocating | view-3-tiny-s12 | 1.306 | 0.922 | 5 | 1.194–1.527 | pass |
| allocating | index-2-v0-tiny-s2 | 0.545 | 0.912 | 5 | 0.176–1.289 | pass |
| allocating | index-2-v0-tiny-s4 | 1.025 | 0.988 | 7 | 0.353–1.596 | pass |
| allocating | index-2-v1-tiny-s2 | 0.940 | 0.919 | 5 | 0.858–1.175 | pass |
| allocating | index-2-v1-tiny-s4 | 0.956 | 0.983 | 5 | 0.807–1.181 | pass |
| allocating | index-3-v0-tiny-s2 | 0.984 | 0.822 | 7 | 0.267–1.438 | pass |
| allocating | index-3-v0-tiny-s4 | 1.136 | 0.964 | 5 | 0.901–1.467 | pass |
| allocating | index-3-v1-tiny-s2 | 0.938 | 0.921 | 5 | 0.782–1.024 | pass |
| allocating | index-3-v1-tiny-s4 | 0.921 | 0.958 | 5 | 0.834–1.059 | pass |
| allocating | vertex-v0-resident-s4 | 1.232 | 0.350 | 5 | 0.965–1.297 | pass |
| allocating | view-none-resident-s4 | 1.218 | 0.418 | 5 | 1.055–1.263 | pass |
| allocating | vertex-v0-resident-s12 | 1.436 | 0.438 | 5 | 1.357–1.541 | pass |
| allocating | view-none-resident-s12 | 1.451 | 0.443 | 5 | 1.382–1.530 | pass |
| allocating | vertex-v0-resident-s32 | 1.333 | 0.420 | 5 | 1.217–1.497 | pass |
| allocating | view-none-resident-s32 | 1.300 | 0.401 | 5 | 1.207–1.514 | pass |
| allocating | view-1-resident-s4 | 1.802 | 0.611 | 6 | 1.623–2.011 | fail |
| allocating | view-1-resident-s8 | 2.290 | 0.637 | 5 | 1.808–2.781 | fail |
| allocating | view-2-resident-s8 | 1.980 | 0.639 | 5 | 1.746–3.020 | fail |
| allocating | view-3-resident-s12 | 2.344 | 0.566 | 5 | 2.080–2.482 | fail |
| allocating | index-2-v0-resident-s2 | 1.194 | 0.908 | 5 | 0.992–1.329 | pass |
| allocating | index-2-v0-resident-s4 | 0.964 | 0.917 | 5 | 0.867–1.383 | pass |
| allocating | index-2-v1-resident-s2 | 1.192 | 0.966 | 5 | 1.039–1.351 | pass |
| allocating | index-2-v1-resident-s4 | 1.090 | 0.788 | 5 | 0.969–1.291 | pass |
| allocating | index-3-v0-resident-s2 | 0.975 | 1.071 | 5 | 0.808–1.138 | pass |
| allocating | index-3-v0-resident-s4 | 0.853 | 1.029 | 5 | 0.814–1.436 | pass |
| allocating | index-3-v1-resident-s2 | 1.090 | 1.075 | 5 | 0.936–1.213 | pass |
| allocating | index-3-v1-resident-s4 | 1.056 | 1.098 | 7 | 0.722–1.517 | pass |
| allocating | vertex-v0-streaming-s4 | 1.144 | 0.365 | 12 | 0.840–1.582 | pass |
| allocating | view-none-streaming-s4 | 1.093 | 0.392 | 11 | 0.797–1.585 | pass |
| allocating | vertex-v0-streaming-s12 | 1.354 | 0.450 | 10 | 1.069–1.578 | pass |
| allocating | view-none-streaming-s12 | 1.020 | 0.405 | 5 | 0.892–1.514 | pass |
| allocating | vertex-v0-streaming-s32 | 1.127 | 0.564 | 5 | 1.004–1.344 | pass |
| allocating | view-none-streaming-s32 | 1.154 | 0.540 | 5 | 0.970–1.263 | pass |
| allocating | view-1-streaming-s4 | 1.811 | 0.598 | 5 | 1.637–2.527 | fail |
| allocating | view-1-streaming-s8 | 2.537 | 0.653 | 5 | 2.258–3.162 | fail |
| allocating | view-2-streaming-s8 | 2.320 | 0.635 | 5 | 2.152–2.401 | fail |
| allocating | view-3-streaming-s12 | 1.448 | 0.654 | 20+30 | 1.420–1.589 | pass |
| allocating | index-2-v0-streaming-s2 | 1.110 | 0.997 | 5 | 0.950–1.495 | pass |
| allocating | index-2-v0-streaming-s4 | 1.144 | 0.977 | 6 | 0.713–1.448 | pass |
| allocating | index-2-v1-streaming-s2 | 1.196 | 0.920 | 7 | 1.004–1.589 | pass |
| allocating | index-2-v1-streaming-s4 | 1.257 | 0.938 | 8 | 0.882–1.555 | pass |
| allocating | index-3-v0-streaming-s2 | 1.055 | 1.007 | 5 | 0.651–1.494 | pass |
| allocating | index-3-v0-streaming-s4 | 1.059 | 0.981 | 9 | 0.707–1.562 | pass |
| allocating | index-3-v1-streaming-s2 | 0.988 | 0.960 | 5 | 0.723–1.097 | pass |
| allocating | index-3-v1-streaming-s4 | 0.987 | 0.974 | 11 | 0.719–1.542 | pass |
| allocating | vertex-v1-tiny-s4 | 1.034 | 0.819 | 8 | 0.543–1.500 | pass |
| allocating | vertex-v1-tiny-s12 | 1.388 | 0.861 | 12 | 1.086–1.598 | pass |
| allocating | vertex-v1-tiny-s32 | 1.698 | 0.840 | 5 | 1.642–1.759 | fail |
| allocating | vertex-v1-resident-s4 | 1.189 | 0.384 | 5 | 1.128–1.353 | pass |
| allocating | vertex-v1-resident-s12 | 1.437 | 0.434 | 5 | 1.281–1.570 | pass |
| allocating | vertex-v1-resident-s32 | 1.353 | 0.438 | 5 | 1.299–1.537 | pass |
| allocating | vertex-v1-streaming-s4 | 1.206 | 0.389 | 11 | 0.905–1.560 | pass |
| allocating | vertex-v1-streaming-s12 | 1.463 | 0.451 | 12 | 1.106–1.586 | pass |
| allocating | vertex-v1-streaming-s32 | 1.144 | 0.590 | 5 | 0.998–1.446 | pass |
| allocating | vertex-v0-resident-s12-level0 | 1.331 | 0.446 | 20+30 | 1.304–1.463 | pass |
| allocating | vertex-v0-resident-s12-level9 | 1.426 | 0.436 | 7 | 1.340–1.572 | pass |
| allocating | vertex-v1-resident-s12-level0 | 1.407 | 0.438 | 5 | 1.270–1.497 | pass |
| allocating | vertex-v1-resident-s12-level9 | 1.488 | 0.449 | 20+30 | 1.379–1.508 | pass |
| allocating | index-2-v0-millions-s4 | 1.542 | 1.398 | 20+30 | 0.850–1.049 | pass |
| allocating | index-2-v1-millions-s4 | 1.337 | 0.980 | 12 | 1.134–1.572 | pass |
| allocating | varied-view-1-tiny-s4 | 0.989 | 0.593 | 5 | 0.835–1.373 | pass |
| allocating | varied-view-1-tiny-s8 | 1.369 | 0.646 | 7 | 1.065–1.577 | pass |
| allocating | varied-view-2-tiny-s8 | 1.396 | 0.709 | 14 | 1.173–1.571 | pass |
| allocating | varied-view-3-tiny-s4 | 1.125 | 0.883 | 5 | 1.071–1.201 | pass |
| allocating | varied-view-3-tiny-s12 | 1.422 | 0.856 | 5 | 1.195–1.579 | pass |
| allocating | varied-view-3-tiny-s32 | 1.636 | 0.817 | 17 | 1.603–1.720 | fail |
| allocating | varied-view-1-resident-s4 | 1.874 | 0.211 | 5 | 1.752–1.928 | fail |
| allocating | varied-view-1-resident-s8 | 1.938 | 0.276 | 5 | 1.787–2.074 | fail |
| allocating | varied-view-2-resident-s8 | 1.916 | 0.285 | 5 | 1.706–2.095 | fail |
| allocating | varied-view-3-resident-s4 | 1.285 | 0.403 | 5 | 1.085–1.470 | pass |
| allocating | varied-view-3-resident-s12 | 1.615 | 0.476 | 20+30 | 1.644–1.877 | fail |
| allocating | varied-view-3-resident-s32 | 1.731 | 0.461 | 7 | 1.606–1.933 | fail |
| allocating | varied-view-1-streaming-s4 | 1.970 | 0.151 | 7 | 1.658–2.552 | fail |
| allocating | varied-view-1-streaming-s8 | 1.818 | 0.195 | 10 | 1.602–2.026 | fail |
| allocating | varied-view-2-streaming-s8 | 1.853 | 0.218 | 5 | 1.634–2.430 | fail |
| allocating | varied-view-3-streaming-s4 | 1.266 | 0.386 | 8 | 1.188–1.585 | pass |
| allocating | varied-view-3-streaming-s12 | 1.985 | 0.461 | 20+30 | 1.500–1.920 | fail |
| allocating | varied-view-3-streaming-s32 | 1.726 | 0.412 | 20+30 | 1.648–1.757 | fail |
| caller-buffer | vertex-v0-tiny-s4 | 0.989 | 0.721 | 7 | 0.482–1.583 | pass |
| caller-buffer | view-none-tiny-s4 | 1.012 | 0.705 | 5 | 0.953–1.234 | pass |
| caller-buffer | vertex-v0-tiny-s12 | 1.331 | 0.705 | 5 | 1.240–1.395 | pass |
| caller-buffer | view-none-tiny-s12 | 1.319 | 0.725 | 5 | 1.260–1.487 | pass |
| caller-buffer | vertex-v0-tiny-s32 | 1.618 | 0.693 | 20+30 | 1.512–1.574 | pass |
| caller-buffer | view-none-tiny-s32 | 1.625 | 0.710 | 7 | 1.601–1.680 | fail |
| caller-buffer | view-1-tiny-s4 | 1.229 | 1.038 | 5 | 0.846–1.365 | pass |
| caller-buffer | view-1-tiny-s8 | 1.604 | 1.093 | 20+30 | 1.515–1.639 | fail |
| caller-buffer | view-2-tiny-s8 | 1.657 | 1.084 | 13 | 1.604–1.681 | fail |
| caller-buffer | view-3-tiny-s12 | 1.644 | 0.990 | 20+30 | 1.545–1.625 | fail |
| caller-buffer | index-2-v0-tiny-s2 | 0.822 | 0.964 | 5 | 0.792–0.849 | pass |
| caller-buffer | index-2-v0-tiny-s4 | 0.838 | 0.979 | 5 | 0.651–1.191 | pass |
| caller-buffer | index-2-v1-tiny-s2 | 0.821 | 0.955 | 5 | 0.684–0.891 | pass |
| caller-buffer | index-2-v1-tiny-s4 | 0.834 | 0.953 | 5 | 0.677–1.104 | pass |
| caller-buffer | index-3-v0-tiny-s2 | 0.789 | 0.960 | 5 | 0.703–0.836 | pass |
| caller-buffer | index-3-v0-tiny-s4 | 0.931 | 1.137 | 5 | 0.766–1.030 | pass |
| caller-buffer | index-3-v1-tiny-s2 | 0.749 | 0.925 | 5 | 0.661–0.793 | pass |
| caller-buffer | index-3-v1-tiny-s4 | 0.793 | 0.963 | 5 | 0.722–0.832 | pass |
| caller-buffer | vertex-v0-resident-s4 | 1.192 | 0.349 | 5 | 1.165–1.216 | pass |
| caller-buffer | view-none-resident-s4 | 1.169 | 0.352 | 5 | 1.140–1.216 | pass |
| caller-buffer | vertex-v0-resident-s12 | 1.423 | 0.417 | 5 | 1.360–1.487 | pass |
| caller-buffer | view-none-resident-s12 | 1.410 | 0.422 | 5 | 1.325–1.541 | pass |
| caller-buffer | vertex-v0-resident-s32 | 1.414 | 0.416 | 5 | 1.396–1.433 | pass |
| caller-buffer | view-none-resident-s32 | 1.387 | 0.404 | 5 | 1.339–1.495 | pass |
| caller-buffer | view-1-resident-s4 | 1.815 | 0.574 | 5 | 1.796–2.038 | fail |
| caller-buffer | view-1-resident-s8 | 2.740 | 0.626 | 5 | 2.621–2.812 | fail |
| caller-buffer | view-2-resident-s8 | 2.393 | 0.601 | 5 | 2.347–2.666 | fail |
| caller-buffer | view-3-resident-s12 | 2.497 | 0.535 | 5 | 2.346–2.626 | fail |
| caller-buffer | index-2-v0-resident-s2 | 1.342 | 1.137 | 5 | 1.052–1.483 | pass |
| caller-buffer | index-2-v0-resident-s4 | 1.187 | 0.927 | 5 | 1.107–1.404 | pass |
| caller-buffer | index-2-v1-resident-s2 | 1.068 | 1.021 | 5 | 0.856–1.460 | pass |
| caller-buffer | index-2-v1-resident-s4 | 1.299 | 1.028 | 5 | 1.167–1.430 | pass |
| caller-buffer | index-3-v0-resident-s2 | 0.990 | 1.010 | 5 | 0.880–1.079 | pass |
| caller-buffer | index-3-v0-resident-s4 | 1.007 | 0.982 | 5 | 0.980–1.027 | pass |
| caller-buffer | index-3-v1-resident-s2 | 0.997 | 1.010 | 5 | 0.937–1.055 | pass |
| caller-buffer | index-3-v1-resident-s4 | 0.985 | 1.006 | 5 | 0.887–1.128 | pass |
| caller-buffer | vertex-v0-streaming-s4 | 1.245 | 0.390 | 5 | 1.123–1.316 | pass |
| caller-buffer | view-none-streaming-s4 | 1.179 | 0.356 | 5 | 1.084–1.390 | pass |
| caller-buffer | vertex-v0-streaming-s12 | 1.577 | 0.477 | 20+30 | 1.588–1.634 | fail |
| caller-buffer | view-none-streaming-s12 | 1.544 | 0.472 | 20+30 | 1.533–1.657 | fail |
| caller-buffer | vertex-v0-streaming-s32 | 1.639 | 0.450 | 20+30 | 1.790–1.959 | fail |
| caller-buffer | view-none-streaming-s32 | 1.322 | 0.424 | 20+30 | 1.314–1.370 | pass |
| caller-buffer | view-1-streaming-s4 | 1.985 | 0.587 | 5 | 1.973–2.002 | fail |
| caller-buffer | view-1-streaming-s8 | 2.578 | 0.633 | 5 | 2.479–2.653 | fail |
| caller-buffer | view-2-streaming-s8 | 2.366 | 0.609 | 5 | 2.324–2.537 | fail |
| caller-buffer | view-3-streaming-s12 | 2.279 | 0.532 | 5 | 1.921–2.589 | fail |
| caller-buffer | index-2-v0-streaming-s2 | 1.020 | 1.015 | 5 | 0.994–1.077 | pass |
| caller-buffer | index-2-v0-streaming-s4 | 1.016 | 0.979 | 5 | 0.974–1.054 | pass |
| caller-buffer | index-2-v1-streaming-s2 | 1.242 | 0.975 | 5 | 1.208–1.248 | pass |
| caller-buffer | index-2-v1-streaming-s4 | 1.247 | 1.023 | 5 | 1.186–1.346 | pass |
| caller-buffer | index-3-v0-streaming-s2 | 1.027 | 1.003 | 5 | 1.005–1.055 | pass |
| caller-buffer | index-3-v0-streaming-s4 | 1.028 | 0.998 | 5 | 1.009–1.044 | pass |
| caller-buffer | index-3-v1-streaming-s2 | 0.989 | 1.017 | 5 | 0.978–1.015 | pass |
| caller-buffer | index-3-v1-streaming-s4 | 1.012 | 0.991 | 5 | 0.969–1.094 | pass |
| caller-buffer | vertex-v1-tiny-s4 | 0.998 | 0.819 | 5 | 0.535–1.323 | pass |
| caller-buffer | vertex-v1-tiny-s12 | 1.480 | 0.846 | 10 | 1.368–1.597 | pass |
| caller-buffer | vertex-v1-tiny-s32 | 1.838 | 0.790 | 6 | 1.626–1.957 | fail |
| caller-buffer | vertex-v1-resident-s4 | 1.199 | 0.360 | 5 | 1.168–1.276 | pass |
| caller-buffer | vertex-v1-resident-s12 | 1.447 | 0.423 | 5 | 1.405–1.481 | pass |
| caller-buffer | vertex-v1-resident-s32 | 1.439 | 0.421 | 5 | 1.408–1.451 | pass |
| caller-buffer | vertex-v1-streaming-s4 | 1.184 | 0.361 | 5 | 1.160–1.232 | pass |
| caller-buffer | vertex-v1-streaming-s12 | 1.446 | 0.444 | 5 | 1.408–1.462 | pass |
| caller-buffer | vertex-v1-streaming-s32 | 1.362 | 0.441 | 5 | 1.320–1.367 | pass |
| caller-buffer | vertex-v0-resident-s12-level0 | 1.419 | 0.416 | 5 | 1.402–1.445 | pass |
| caller-buffer | vertex-v0-resident-s12-level9 | 1.401 | 0.423 | 5 | 1.280–1.496 | pass |
| caller-buffer | vertex-v1-resident-s12-level0 | 1.427 | 0.421 | 5 | 1.355–1.465 | pass |
| caller-buffer | vertex-v1-resident-s12-level9 | 1.390 | 0.419 | 8 | 1.078–1.578 | pass |
| caller-buffer | index-2-v0-millions-s4 | 1.001 | 0.971 | 5 | 0.963–1.117 | pass |
| caller-buffer | index-2-v1-millions-s4 | 1.554 | 1.002 | 11 | 1.515–1.599 | pass |
| caller-buffer | varied-view-1-tiny-s4 | 1.252 | 0.546 | 9 | 0.752–1.550 | pass |
| caller-buffer | varied-view-1-tiny-s8 | 1.508 | 0.584 | 7 | 1.415–1.597 | pass |
| caller-buffer | varied-view-2-tiny-s8 | 1.479 | 0.571 | 5 | 1.438–1.546 | pass |
| caller-buffer | varied-view-3-tiny-s4 | 1.078 | 0.711 | 5 | 1.036–1.145 | pass |
| caller-buffer | varied-view-3-tiny-s12 | 1.501 | 0.785 | 5 | 1.416–1.528 | pass |
| caller-buffer | varied-view-3-tiny-s32 | 1.864 | 0.776 | 5 | 1.717–1.887 | fail |
| caller-buffer | varied-view-1-resident-s4 | 2.052 | 0.199 | 6 | 1.676–2.475 | fail |
| caller-buffer | varied-view-1-resident-s8 | 1.943 | 0.268 | 5 | 1.918–1.997 | fail |
| caller-buffer | varied-view-2-resident-s8 | 1.835 | 0.254 | 5 | 1.791–1.959 | fail |
| caller-buffer | varied-view-3-resident-s4 | 1.295 | 0.372 | 5 | 1.237–1.357 | pass |
| caller-buffer | varied-view-3-resident-s12 | 1.649 | 0.451 | 5 | 1.607–1.684 | fail |
| caller-buffer | varied-view-3-resident-s32 | 1.641 | 0.441 | 5 | 1.607–1.732 | fail |
| caller-buffer | varied-view-1-streaming-s4 | 1.964 | 0.149 | 5 | 1.957–2.021 | fail |
| caller-buffer | varied-view-1-streaming-s8 | 1.961 | 0.188 | 5 | 1.912–2.069 | fail |
| caller-buffer | varied-view-2-streaming-s8 | 1.889 | 0.199 | 5 | 1.855–1.944 | fail |
| caller-buffer | varied-view-3-streaming-s4 | 1.535 | 0.341 | 16 | 1.301–1.584 | pass |
| caller-buffer | varied-view-3-streaming-s12 | 1.774 | 0.370 | 5 | 1.737–1.795 | fail |
| caller-buffer | varied-view-3-streaming-s32 | 1.676 | 0.393 | 6 | 1.601–1.806 | fail |
