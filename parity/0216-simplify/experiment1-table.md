# Simplifier instruction counts: experiment1

Median of three samples per cell. Ratios are Rust/peer instructions:u.
Scalar Rust uses std without simd; default Rust uses std+simd. C++ scalar
disables explicit SIMD; default C++ enables it. All six serialized outputs
match exactly in each of the 32 cells. Counts include allocation and output
drop, use warmed workspaces and include input copies in update. Setup, input
parsing, attribute generation, warmup and serialization are outside counters.
Wall times are from the same counted loops on a shared host, disclosed only.

| Function | Default/base real | Default/base corpus | Max case | Scalar/scalar C++ real / corpus | Default/default C++ real / corpus | Verdict |
|---|---:|---:|---:|---:|---:|---|
| simplify | 1.008289 | 1.002931 | 1.011346 | 1.130385 / 1.040286 | 1.130385 / 1.040286 | NO-GAIN |
| simplify-attributes | 1.005340 | 1.001661 | 1.006787 | 1.090316 / 1.057441 | 1.090316 / 1.057441 | NO-GAIN |
| simplify-sloppy | 1.377033 | 1.158245 | 1.404782 | 1.747083 / 1.454535 | 1.747082 / 1.454536 | NO-GAIN |
| simplify-update | 0.964907 | 0.992584 | 0.995743 | 1.216398 / 1.066602 | 1.216398 / 1.066623 | NO-GAIN |

| Case | Function | Default/base | Scalar/base | Scalar/scalar C++ | Default/default C++ | Default instructions (base / candidate / C++) |
|---|---|---:|---:|---:|---:|---:|
| tiny-smooth | simplify | 1.001629 | 1.001737 | 0.914633 | 0.914632 | 110535 / 110715 / 121049 |
| tiny-smooth | simplify-attributes | 1.001633 | 1.001633 | 0.974606 | 0.974606 | 124931 / 125135 / 128395 |
| tiny-smooth | simplify-update | 0.994911 | 0.994812 | 0.940313 | 0.940406 | 121828 / 121208 / 128889 |
| tiny-smooth | simplify-sloppy | 1.125196 | 1.125195 | 1.402478 | 1.402479 | 19010 / 21390 / 15251 |
| medium-smooth | simplify | 1.003469 | 1.003467 | 1.084443 | 1.084443 | 11959135 / 12000619 / 11066159 |
| medium-smooth | simplify-attributes | 1.001474 | 1.001474 | 1.075701 | 1.075701 | 28144622 / 28186101 / 26202550 |
| medium-smooth | simplify-update | 0.993332 | 0.993332 | 1.109998 | 1.109998 | 15319384 / 15217235 / 13709241 |
| medium-smooth | simplify-sloppy | 1.126780 | 1.126788 | 1.394824 | 1.394824 | 3898443 / 4392686 / 3149276 |
| medium-seams | simplify | 1.002977 | 1.002980 | 1.065695 | 1.065691 | 15226051 / 15271386 / 14330030 |
| medium-seams | simplify-attributes | 1.001595 | 1.001595 | 1.083390 | 1.083389 | 28411694 / 28457007 / 26266649 |
| medium-seams | simplify-update | 0.992675 | 0.992673 | 1.093523 | 1.093524 | 17870421 / 17739528 / 16222348 |
| medium-seams | simplify-sloppy | 1.242891 | 1.242892 | 1.707631 | 1.707637 | 7076075 / 8794793 / 5150270 |
| medium-sparse | simplify | 1.004511 | 1.004510 | 1.084304 | 1.084307 | 12940817 / 12999190 / 11988479 |
| medium-sparse | simplify-attributes | 1.002344 | 1.002344 | 1.076196 | 1.076197 | 24911955 / 24970348 / 23202405 |
| medium-sparse | simplify-update | 0.986286 | 0.986287 | 1.117606 | 1.117607 | 16689782 / 16460898 / 14728700 |
| medium-sparse | simplify-sloppy | 1.207091 | 1.207089 | 1.488023 | 1.488023 | 4773148 / 5761625 / 3871999 |
| million-smooth | simplify | 1.002073 | 1.002074 | 1.062986 | 1.062985 | 2417178984 / 2422189737 / 2278667437 |
| million-smooth | simplify-attributes | 1.001260 | 1.001261 | 1.081645 | 1.081645 | 3974933509 / 3979943727 / 3679527310 |
| million-smooth | simplify-update | 0.995743 | 0.995743 | 1.082182 | 1.082181 | 2837729675 / 2825649299 / 2611067848 |
| million-smooth | simplify-sloppy | 1.095875 | 1.095874 | 1.309789 | 1.309790 | 408963752 / 448173072 / 342171807 |
| real-214e | simplify | 1.011346 | 1.011346 | 1.116506 | 1.116507 | 3150525456 / 3186270963 / 2853785721 |
| real-214e | simplify-attributes | 1.006787 | 1.006788 | 1.065986 | 1.065986 | 5266265225 / 5302008773 / 4973808853 |
| real-214e | simplify-update | 0.954613 | 0.954613 | 1.233629 | 1.233629 | 4305156626 / 4109756624 / 3331436381 |
| real-214e | simplify-sloppy | 1.404782 | 1.404782 | 1.792028 | 1.792028 | 2947575756 / 4140702170 / 2310623353 |
| real-a082 | simplify | 1.006757 | 1.006757 | 1.135803 | 1.135803 | 5744850843 / 5783670448 / 5092144364 |
| real-a082 | simplify-attributes | 1.004614 | 1.004614 | 1.101693 | 1.101692 | 8413248407 / 8452067413 / 7671894502 |
| real-a082 | simplify-update | 0.970140 | 0.970140 | 1.206624 | 1.206624 | 6940778742 / 6733528918 / 5580470684 |
| real-a082 | simplify-sloppy | 1.339432 | 1.339432 | 1.674348 | 1.674348 | 2910435990 / 3898331000 / 2328268008 |
| real-b6fd | simplify | 1.006769 | 1.006769 | 1.138977 | 1.138978 | 1433655744 / 1443360607 / 1267242354 |
| real-b6fd | simplify-attributes | 1.004620 | 1.004620 | 1.103685 | 1.103685 | 2100678073 / 2110382722 / 1912124660 |
| real-b6fd | simplify-update | 0.970051 | 0.970051 | 1.209125 | 1.209124 | 1730016500 / 1678203475 / 1387949350 |
| real-b6fd | simplify-sloppy | 1.387726 | 1.387726 | 1.777255 | 1.777254 | 818980108 / 1136520194 / 639480912 |

Wall time (milliseconds/call; no timing gate):

| Case | Function | Rust base default | Rust candidate default | C++ default | Rust base scalar | Rust candidate scalar | C++ scalar |
|---|---|---:|---:|---:|---:|---:|---:|
| tiny-smooth | simplify | 0.007 | 0.007 | 0.008 | 0.007 | 0.007 | 0.007 |
| tiny-smooth | simplify-attributes | 0.008 | 0.008 | 0.008 | 0.008 | 0.009 | 0.008 |
| tiny-smooth | simplify-update | 0.008 | 0.007 | 0.008 | 0.007 | 0.008 | 0.008 |
| tiny-smooth | simplify-sloppy | 0.001 | 0.001 | 0.001 | 0.001 | 0.002 | 0.001 |
| medium-smooth | simplify | 0.855 | 0.907 | 1.114 | 0.847 | 0.908 | 1.194 |
| medium-smooth | simplify-attributes | 1.837 | 1.924 | 2.242 | 2.140 | 1.876 | 2.181 |
| medium-smooth | simplify-update | 1.073 | 1.126 | 1.381 | 1.331 | 1.126 | 1.400 |
| medium-smooth | simplify-sloppy | 0.241 | 0.291 | 0.328 | 0.251 | 0.280 | 0.339 |
| medium-seams | simplify | 1.118 | 1.172 | 1.575 | 1.188 | 1.185 | 1.476 |
| medium-seams | simplify-attributes | 1.840 | 1.796 | 2.504 | 1.772 | 1.840 | 2.354 |
| medium-seams | simplify-update | 1.340 | 1.271 | 1.581 | 1.346 | 1.318 | 1.581 |
| medium-seams | simplify-sloppy | 0.376 | 0.664 | 0.463 | 0.408 | 0.521 | 0.452 |
| medium-sparse | simplify | 1.052 | 0.833 | 1.199 | 0.907 | 1.165 | 1.165 |
| medium-sparse | simplify-attributes | 1.607 | 1.728 | 2.503 | 1.635 | 1.632 | 2.544 |
| medium-sparse | simplify-update | 1.134 | 1.141 | 1.338 | 1.148 | 1.139 | 1.370 |
| medium-sparse | simplify-sloppy | 0.301 | 0.377 | 0.462 | 0.340 | 0.381 | 0.463 |
| million-smooth | simplify | 290.951 | 309.782 | 335.587 | 297.011 | 308.499 | 329.891 |
| million-smooth | simplify-attributes | 420.192 | 405.360 | 533.064 | 415.058 | 421.604 | 466.358 |
| million-smooth | simplify-update | 345.460 | 326.811 | 376.133 | 352.611 | 345.402 | 354.012 |
| million-smooth | simplify-sloppy | 48.895 | 59.027 | 58.151 | 49.039 | 52.177 | 64.115 |
| real-214e | simplify | 1065.818 | 1136.996 | 1377.242 | 1018.198 | 1064.792 | 1246.432 |
| real-214e | simplify-attributes | 1156.395 | 1132.685 | 1716.882 | 1070.204 | 1159.592 | 1748.836 |
| real-214e | simplify-update | 1173.017 | 1246.986 | 1361.081 | 1199.684 | 1199.807 | 1404.345 |
| real-214e | simplify-sloppy | 598.000 | 718.526 | 679.286 | 607.445 | 720.304 | 673.411 |
| real-a082 | simplify | 2527.907 | 2883.931 | 2945.041 | 2717.553 | 2749.238 | 3038.589 |
| real-a082 | simplify-attributes | 3673.328 | 3744.643 | 7320.909 | 3563.664 | 3877.452 | 4895.859 |
| real-a082 | simplify-update | 3064.515 | 2913.072 | 3412.636 | 3194.163 | 3223.876 | 3202.415 |
| real-a082 | simplify-sloppy | 950.804 | 970.932 | 1094.616 | 988.095 | 1017.545 | 1046.537 |
| real-b6fd | simplify | 303.260 | 295.923 | 365.363 | 317.750 | 299.314 | 365.865 |
| real-b6fd | simplify-attributes | 382.176 | 348.307 | 609.894 | 387.930 | 366.662 | 621.423 |
| real-b6fd | simplify-update | 302.579 | 312.403 | 379.324 | 297.119 | 327.608 | 348.187 |
| real-b6fd | simplify-sloppy | 114.741 | 138.712 | 112.792 | 100.395 | 138.198 | 112.646 |
