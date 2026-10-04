# P02 measured decoder performance

Bar SHA-256 `b08c268e32baf1311fcac33ac19552c9a75185fb8cf5fa379d6e65d07e0dfccd`. Evidence `/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02/candidate.json`.

At least twelve paired samples, increasing to sixty when dispersion exceeds 15% or uncertainty crosses a gate. Samples target 40 ms and rotate all backend orders. Validation, required copies and allocation timed; driver startup, I/O, generation and serialization excluded. Caller buffers are allocated before timing. Source and executable identities checked before and after. Linux x86-64 only; shared host load.

Safe scalar remains the selected implementation under RFC 6.2. SIMD filter comparisons use decoded-unit conformance; canonical output follows scalar C++.

Standalone Oct/Quat timing inputs contain repeated encoded directions/quaternions. Exact last-record reuse explains their throughput; these ratios do not establish throughput on varied filter data. Varied filters are covered by correctness sweeps. The registered Moss minima remain unchanged. The C++ standalone filter allocation was corrected to copy-construct without redundant zeroing; the separate baseline correction record retains paired measurements and does not change any minimum.

| API | Case | Rust MB/s | SIMD MB/s | Rust/SIMD | Rust/scalar time | Min bar |
|---|---|---:|---:|---:|---:|---|
| allocating | vertex-v0-tiny-s4 | 159.722 | 340.602 | 0.469 | 1.031 | True |
| allocating | view-none-tiny-s4 | 124.384 | 284.357 | 0.437 | 1.037 | True |
| allocating | vertex-v0-tiny-s12 | 93.197 | 233.474 | 0.399 | 1.029 | True |
| allocating | view-none-tiny-s12 | 104.665 | 263.613 | 0.397 | 0.855 | True |
| allocating | vertex-v0-tiny-s32 | 143.045 | 362.816 | 0.394 | 0.937 | True |
| allocating | view-none-tiny-s32 | 136.834 | 322.614 | 0.424 | 0.901 | True |
| allocating | filter-1-tiny-s4 | 324.483 | 415.697 | 0.781 | 0.645 | True |
| allocating | view-1-tiny-s4 | 101.959 | 165.067 | 0.618 | 0.988 | True |
| allocating | filter-1-tiny-s8 | 704.371 | 719.014 | 0.980 | 0.880 | True |
| allocating | view-1-tiny-s8 | 146.287 | 226.076 | 0.647 | 1.088 | True |
| allocating | filter-2-tiny-s8 | 638.412 | 824.339 | 0.774 | 0.586 | True |
| allocating | view-2-tiny-s8 | 142.341 | 236.524 | 0.602 | 0.972 | True |
| allocating | filter-3-tiny-s12 | 1990.006 | 2011.076 | 0.990 | 1.138 | True |
| allocating | view-3-tiny-s12 | 170.188 | 294.814 | 0.577 | 1.167 | True |
| allocating | index-2-v0-tiny-s2 | 164.011 | 205.352 | 0.799 | 1.264 | True |
| allocating | index-2-v0-tiny-s4 | 341.974 | 441.637 | 0.774 | 1.286 | True |
| allocating | index-2-v1-tiny-s2 | 161.876 | 213.379 | 0.759 | 1.250 | True |
| allocating | index-2-v1-tiny-s4 | 351.919 | 467.923 | 0.752 | 1.290 | True |
| allocating | index-3-v0-tiny-s2 | 190.713 | 249.716 | 0.764 | 1.325 | True |
| allocating | index-3-v0-tiny-s4 | 362.656 | 477.630 | 0.759 | 1.332 | True |
| allocating | index-3-v1-tiny-s2 | 182.116 | 245.297 | 0.742 | 1.326 | True |
| allocating | index-3-v1-tiny-s4 | 350.703 | 490.734 | 0.715 | 1.375 | True |
| allocating | vertex-v0-resident-s4 | 187.987 | 833.159 | 0.226 | 0.978 | True |
| allocating | view-none-resident-s4 | 199.756 | 886.400 | 0.225 | 0.990 | True |
| allocating | vertex-v0-resident-s12 | 185.661 | 773.758 | 0.240 | 0.978 | True |
| allocating | view-none-resident-s12 | 264.736 | 1195.992 | 0.221 | 0.893 | True |
| allocating | vertex-v0-resident-s32 | 194.752 | 816.933 | 0.238 | 0.970 | True |
| allocating | view-none-resident-s32 | 186.131 | 830.209 | 0.224 | 1.021 | True |
| allocating | filter-1-resident-s4 | 951.566 | 1140.834 | 0.834 | 0.342 | True |
| allocating | view-1-resident-s4 | 193.091 | 574.837 | 0.336 | 0.783 | True |
| allocating | filter-1-resident-s8 | 2180.457 | 1399.844 | 1.558 | 0.365 | True |
| allocating | view-1-resident-s8 | 262.127 | 863.758 | 0.303 | 0.961 | True |
| allocating | filter-2-resident-s8 | 1643.865 | 1935.583 | 0.849 | 0.310 | True |
| allocating | view-2-resident-s8 | 225.445 | 826.954 | 0.273 | 0.831 | True |
| allocating | filter-3-resident-s12 | 5525.284 | 5010.649 | 1.103 | 0.878 | True |
| allocating | view-3-resident-s12 | 243.751 | 1156.645 | 0.211 | 1.190 | True |
| allocating | index-2-v0-resident-s2 | 411.517 | 408.211 | 1.008 | 1.093 | True |
| allocating | index-2-v0-resident-s4 | 782.512 | 830.765 | 0.942 | 1.079 | True |
| allocating | index-2-v1-resident-s2 | 394.343 | 322.183 | 1.224 | 0.977 | True |
| allocating | index-2-v1-resident-s4 | 866.622 | 880.904 | 0.984 | 1.010 | True |
| allocating | index-3-v0-resident-s2 | 422.370 | 427.249 | 0.989 | 1.070 | True |
| allocating | index-3-v0-resident-s4 | 1402.749 | 1573.822 | 0.891 | 1.089 | True |
| allocating | index-3-v1-resident-s2 | 798.802 | 909.747 | 0.878 | 1.147 | True |
| allocating | index-3-v1-resident-s4 | 1561.791 | 1781.517 | 0.877 | 1.083 | True |
| allocating | vertex-v0-streaming-s4 | 367.884 | 1262.095 | 0.291 | 0.831 | True |
| allocating | view-none-streaming-s4 | 426.660 | 1433.257 | 0.298 | 0.813 | True |
| allocating | vertex-v0-streaming-s12 | 383.766 | 1008.264 | 0.381 | 0.865 | True |
| allocating | view-none-streaming-s12 | 377.619 | 788.644 | 0.479 | 0.852 | True |
| allocating | vertex-v0-streaming-s32 | 365.096 | 818.518 | 0.446 | 0.877 | True |
| allocating | view-none-streaming-s32 | 357.954 | 792.026 | 0.452 | 0.854 | True |
| allocating | filter-1-streaming-s4 | 1708.077 | 1492.151 | 1.145 | 0.378 | True |
| allocating | view-1-streaming-s4 | 498.782 | 1448.427 | 0.344 | 0.785 | True |
| allocating | filter-1-streaming-s8 | 1991.007 | 1525.660 | 1.305 | 0.496 | True |
| allocating | view-1-streaming-s8 | 500.152 | 1289.229 | 0.388 | 0.978 | True |
| allocating | filter-2-streaming-s8 | 1596.356 | 1731.700 | 0.922 | 0.527 | True |
| allocating | view-2-streaming-s8 | 464.954 | 1326.927 | 0.350 | 0.912 | True |
| allocating | filter-3-streaming-s12 | 1184.552 | 1240.946 | 0.955 | 1.072 | True |
| allocating | view-3-streaming-s12 | 500.315 | 1285.000 | 0.389 | 1.075 | True |
| allocating | index-2-v0-streaming-s2 | 685.516 | 838.953 | 0.817 | 1.210 | True |
| allocating | index-2-v0-streaming-s4 | 1553.663 | 1853.969 | 0.838 | 1.258 | True |
| allocating | index-2-v1-streaming-s2 | 1159.171 | 1120.884 | 1.034 | 0.985 | True |
| allocating | index-2-v1-streaming-s4 | 2313.837 | 2258.512 | 1.024 | 0.956 | True |
| allocating | index-3-v0-streaming-s2 | 1106.305 | 1186.730 | 0.932 | 1.081 | True |
| allocating | index-3-v0-streaming-s4 | 2199.480 | 2403.312 | 0.915 | 1.092 | True |
| allocating | index-3-v1-streaming-s2 | 1023.299 | 1112.451 | 0.920 | 1.086 | True |
| allocating | index-3-v1-streaming-s4 | 2024.600 | 2271.605 | 0.891 | 1.090 | True |
| allocating | vertex-v1-tiny-s4 | 278.704 | 602.380 | 0.463 | 0.901 | Not registered |
| allocating | vertex-v1-tiny-s12 | 350.214 | 755.472 | 0.464 | 0.791 | Not registered |
| allocating | vertex-v1-tiny-s32 | 400.926 | 855.694 | 0.469 | 0.713 | Not registered |
| allocating | vertex-v1-resident-s4 | 464.379 | 2051.362 | 0.226 | 0.828 | Not registered |
| allocating | vertex-v1-resident-s12 | 472.517 | 2125.226 | 0.222 | 0.843 | Not registered |
| allocating | vertex-v1-resident-s32 | 261.284 | 1139.318 | 0.229 | 0.957 | Not registered |
| allocating | vertex-v1-streaming-s4 | 373.574 | 1176.547 | 0.318 | 0.789 | Not registered |
| allocating | vertex-v1-streaming-s12 | 425.324 | 1154.615 | 0.368 | 0.776 | Not registered |
| allocating | vertex-v1-streaming-s32 | 384.511 | 790.058 | 0.487 | 0.789 | Not registered |
| allocating | vertex-v0-resident-s12-level0 | 509.557 | 2306.142 | 0.221 | 0.836 | Not registered |
| allocating | vertex-v0-resident-s12-level9 | 551.828 | 2223.463 | 0.248 | 0.800 | Not registered |
| allocating | vertex-v1-resident-s12-level0 | 525.646 | 2330.081 | 0.226 | 0.842 | Not registered |
| allocating | vertex-v1-resident-s12-level9 | 336.377 | 1492.890 | 0.225 | 0.931 | Not registered |
| allocating | index-2-v0-millions-s4 | 582.075 | 656.252 | 0.887 | 1.144 | Not registered |
| allocating | index-2-v1-millions-s4 | 1416.217 | 1343.126 | 1.054 | 0.977 | Not registered |
| caller_buffer | vertex-v0-tiny-s4 | 290.417 | 653.325 | 0.445 | 0.776 | True |
| caller_buffer | view-none-tiny-s4 | 309.725 | 687.932 | 0.450 | 0.813 | True |
| caller_buffer | vertex-v0-tiny-s12 | 342.143 | 794.559 | 0.431 | 0.706 | True |
| caller_buffer | view-none-tiny-s12 | 345.048 | 731.903 | 0.471 | 0.719 | True |
| caller_buffer | vertex-v0-tiny-s32 | 401.981 | 932.147 | 0.431 | 0.677 | True |
| caller_buffer | view-none-tiny-s32 | 420.682 | 909.465 | 0.463 | 0.657 | True |
| caller_buffer | filter-1-tiny-s4 | 847.278 | 965.408 | 0.878 | 0.678 | True |
| caller_buffer | view-1-tiny-s4 | 221.078 | 311.570 | 0.710 | 0.983 | True |
| caller_buffer | filter-1-tiny-s8 | 1389.140 | 1095.070 | 1.269 | 0.662 | True |
| caller_buffer | view-1-tiny-s8 | 306.617 | 424.288 | 0.723 | 1.077 | True |
| caller_buffer | filter-2-tiny-s8 | 1167.082 | 1341.301 | 0.870 | 0.772 | True |
| caller_buffer | view-2-tiny-s8 | 323.077 | 497.371 | 0.650 | 1.102 | True |
| caller_buffer | filter-3-tiny-s12 | 4968.335 | 4766.334 | 1.042 | 1.517 | True |
| caller_buffer | view-3-tiny-s12 | 403.402 | 677.789 | 0.595 | 1.163 | True |
| caller_buffer | index-2-v0-tiny-s2 | 628.916 | 732.348 | 0.859 | 1.192 | True |
| caller_buffer | index-2-v0-tiny-s4 | 1291.562 | 1601.957 | 0.806 | 1.266 | True |
| caller_buffer | index-2-v1-tiny-s2 | 694.652 | 811.084 | 0.856 | 1.193 | True |
| caller_buffer | index-2-v1-tiny-s4 | 1273.961 | 1569.306 | 0.812 | 1.256 | True |
| caller_buffer | index-3-v0-tiny-s2 | 714.660 | 731.987 | 0.976 | 1.015 | True |
| caller_buffer | index-3-v0-tiny-s4 | 1413.347 | 1384.052 | 1.021 | 1.065 | True |
| caller_buffer | index-3-v1-tiny-s2 | 728.119 | 754.374 | 0.965 | 1.024 | True |
| caller_buffer | index-3-v1-tiny-s4 | 1405.388 | 1344.760 | 1.045 | 1.065 | True |
| caller_buffer | vertex-v0-resident-s4 | 529.487 | 2344.434 | 0.226 | 0.809 | True |
| caller_buffer | view-none-resident-s4 | 484.675 | 2019.660 | 0.240 | 0.813 | True |
| caller_buffer | vertex-v0-resident-s12 | 480.555 | 2085.627 | 0.230 | 0.769 | True |
| caller_buffer | view-none-resident-s12 | 504.576 | 2305.558 | 0.219 | 0.843 | True |
| caller_buffer | vertex-v0-resident-s32 | 546.061 | 2359.707 | 0.231 | 0.797 | True |
| caller_buffer | view-none-resident-s32 | 549.027 | 2335.960 | 0.235 | 0.785 | True |
| caller_buffer | filter-1-resident-s4 | 2552.625 | 2272.424 | 1.123 | 0.287 | True |
| caller_buffer | view-1-resident-s4 | 546.013 | 1555.862 | 0.351 | 0.812 | True |
| caller_buffer | filter-1-resident-s8 | 6230.675 | 3156.451 | 1.974 | 0.263 | True |
| caller_buffer | view-1-resident-s8 | 596.163 | 1974.571 | 0.302 | 0.964 | True |
| caller_buffer | filter-2-resident-s8 | 3553.479 | 3756.470 | 0.946 | 0.347 | True |
| caller_buffer | view-2-resident-s8 | 582.054 | 2204.175 | 0.264 | 0.915 | True |
| caller_buffer | filter-3-resident-s12 | 11886.964 | 11240.308 | 1.058 | 0.937 | True |
| caller_buffer | view-3-resident-s12 | 576.004 | 3300.782 | 0.175 | 1.278 | True |
| caller_buffer | index-2-v0-resident-s2 | 1188.472 | 1196.352 | 0.993 | 0.974 | True |
| caller_buffer | index-2-v0-resident-s4 | 2302.328 | 2221.380 | 1.036 | 0.947 | True |
| caller_buffer | index-2-v1-resident-s2 | 1206.815 | 1214.163 | 0.994 | 0.971 | True |
| caller_buffer | index-2-v1-resident-s4 | 2204.524 | 2121.718 | 1.039 | 0.946 | True |
| caller_buffer | index-3-v0-resident-s2 | 822.897 | 894.220 | 0.920 | 1.164 | True |
| caller_buffer | index-3-v0-resident-s4 | 1797.552 | 2070.719 | 0.868 | 1.139 | True |
| caller_buffer | index-3-v1-resident-s2 | 786.021 | 913.355 | 0.861 | 1.169 | True |
| caller_buffer | index-3-v1-resident-s4 | 1599.990 | 1895.454 | 0.844 | 1.214 | True |
| caller_buffer | vertex-v0-streaming-s4 | 194.682 | 788.638 | 0.247 | 1.003 | True |
| caller_buffer | view-none-streaming-s4 | 196.596 | 770.709 | 0.255 | 0.971 | True |
| caller_buffer | vertex-v0-streaming-s12 | 196.445 | 792.186 | 0.248 | 0.974 | True |
| caller_buffer | view-none-streaming-s12 | 191.055 | 786.791 | 0.243 | 0.989 | True |
| caller_buffer | vertex-v0-streaming-s32 | 524.615 | 2234.033 | 0.235 | 0.813 | True |
| caller_buffer | view-none-streaming-s32 | 490.737 | 2127.720 | 0.231 | 0.828 | True |
| caller_buffer | filter-1-streaming-s4 | 1442.227 | 1338.533 | 1.077 | 0.305 | True |
| caller_buffer | view-1-streaming-s4 | 399.098 | 1062.816 | 0.376 | 0.755 | True |
| caller_buffer | filter-1-streaming-s8 | 3878.485 | 2245.857 | 1.727 | 0.289 | True |
| caller_buffer | view-1-streaming-s8 | 354.288 | 971.245 | 0.365 | 0.960 | True |
| caller_buffer | filter-2-streaming-s8 | 2191.154 | 2308.577 | 0.949 | 0.363 | True |
| caller_buffer | view-2-streaming-s8 | 457.827 | 1524.482 | 0.300 | 0.951 | True |
| caller_buffer | filter-3-streaming-s12 | 2856.171 | 2780.320 | 1.027 | 0.999 | True |
| caller_buffer | view-3-streaming-s12 | 521.228 | 2260.855 | 0.231 | 1.098 | True |
| caller_buffer | index-2-v0-streaming-s2 | 541.725 | 657.399 | 0.824 | 1.172 | True |
| caller_buffer | index-2-v0-streaming-s4 | 1211.166 | 1500.029 | 0.807 | 1.237 | True |
| caller_buffer | index-2-v1-streaming-s2 | 1184.749 | 1150.970 | 1.029 | 0.940 | True |
| caller_buffer | index-2-v1-streaming-s4 | 2572.286 | 2629.756 | 0.978 | 1.015 | True |
| caller_buffer | index-3-v0-streaming-s2 | 1208.464 | 1376.450 | 0.878 | 1.100 | True |
| caller_buffer | index-3-v0-streaming-s4 | 2416.718 | 2722.603 | 0.888 | 1.123 | True |
| caller_buffer | index-3-v1-streaming-s2 | 891.159 | 971.611 | 0.917 | 1.172 | True |
| caller_buffer | index-3-v1-streaming-s4 | 2112.920 | 2329.329 | 0.907 | 1.148 | True |
| caller_buffer | vertex-v1-tiny-s4 | 288.645 | 687.078 | 0.420 | 0.894 | Not registered |
| caller_buffer | vertex-v1-tiny-s12 | 320.260 | 735.579 | 0.435 | 0.830 | Not registered |
| caller_buffer | vertex-v1-tiny-s32 | 174.033 | 455.444 | 0.382 | 0.872 | Not registered |
| caller_buffer | vertex-v1-resident-s4 | 206.459 | 1024.419 | 0.202 | 1.056 | Not registered |
| caller_buffer | vertex-v1-resident-s12 | 433.129 | 2009.607 | 0.216 | 0.845 | Not registered |
| caller_buffer | vertex-v1-resident-s32 | 221.386 | 1067.496 | 0.207 | 1.017 | Not registered |
| caller_buffer | vertex-v1-streaming-s4 | 426.346 | 1808.544 | 0.236 | 0.843 | Not registered |
| caller_buffer | vertex-v1-streaming-s12 | 437.242 | 1954.986 | 0.224 | 0.847 | Not registered |
| caller_buffer | vertex-v1-streaming-s32 | 457.976 | 1952.900 | 0.235 | 0.812 | Not registered |
| caller_buffer | vertex-v0-resident-s12-level0 | 423.367 | 1894.089 | 0.224 | 0.833 | Not registered |
| caller_buffer | vertex-v0-resident-s12-level9 | 461.957 | 2042.806 | 0.226 | 0.836 | Not registered |
| caller_buffer | vertex-v1-resident-s12-level0 | 472.603 | 2142.533 | 0.221 | 0.846 | Not registered |
| caller_buffer | vertex-v1-resident-s12-level9 | 412.998 | 1938.189 | 0.213 | 0.824 | Not registered |
| caller_buffer | index-2-v0-millions-s4 | 1291.942 | 1501.405 | 0.860 | 1.117 | Not registered |
| caller_buffer | index-2-v1-millions-s4 | 1875.827 | 1922.067 | 0.976 | 1.000 | Not registered |

Verdicts: `{"allocating": {"failed_registered_minima": [], "minimum_bar_pass": true, "raw_scalar_bar": {"geometric_mean_rust_scalar_time_ratio": 1.0097599364896501, "maximum_rust_scalar_time_ratio": 1.375314546014453, "pass": true}}, "caller_buffer": {"failed_registered_minima": [], "minimum_bar_pass": true, "raw_scalar_bar": {"geometric_mean_rust_scalar_time_ratio": 0.9726338624437444, "maximum_rust_scalar_time_ratio": 1.2662162320038504, "pass": true}}}`.
