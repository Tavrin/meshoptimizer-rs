# Simplifier instruction counts: experiment2

Median of three samples per cell. Ratios are Rust/peer instructions:u.
Scalar Rust uses std without simd; default Rust uses std+simd. C++ scalar
disables explicit SIMD; default C++ enables it. All six serialized outputs
match exactly in each of the 32 cells. Counts include allocation and output
drop, use warmed workspaces and include input copies in update. Setup, input
parsing, attribute generation, warmup and serialization are outside counters.
Wall times are from the same counted loops on a shared host, disclosed only.

| Function | Default/base real | Default/base corpus | Max case | Scalar/scalar C++ real / corpus | Default/default C++ real / corpus | Verdict |
|---|---:|---:|---:|---:|---:|---|
| simplify | 1.001465 | 1.001161 | 1.001949 | 1.122735 / 1.038428 | 1.122735 / 1.038428 | NO-GAIN |
| simplify-attributes | 1.000945 | 1.000608 | 1.001166 | 1.085550 / 1.056330 | 1.085550 / 1.056330 | NO-GAIN |
| simplify-sloppy | 1.000000 | 1.000001 | 1.000002 | 1.268729 / 1.255807 | 1.268729 / 1.255807 | NO-GAIN |
| simplify-update | 0.939939 | 0.986373 | 0.992023 | 1.184924 / 1.059948 | 1.184924 / 1.059949 | SHIPS |

| Case | Function | Default/base | Scalar/base | Scalar/scalar C++ | Default/default C++ | Default instructions (base / candidate / C++) |
|---|---|---:|---:|---:|---:|---:|
| tiny-smooth | simplify | 1.000198 | 1.000199 | 0.913228 | 0.913228 | 110523 / 110545 / 121049 |
| tiny-smooth | simplify-attributes | 1.000176 | 1.000176 | 0.973188 | 0.973189 | 124931 / 124953 / 128395 |
| tiny-smooth | simplify-update | 0.989872 | 0.989871 | 0.935642 | 0.935643 | 121828 / 120594 / 128889 |
| tiny-smooth | simplify-sloppy | 1.000001 | 0.999995 | 1.246423 | 1.246424 | 19010 / 19010 / 15252 |
| medium-smooth | simplify | 1.001698 | 1.001695 | 1.082528 | 1.082528 | 11959180 / 11979489 / 11066216 |
| medium-smooth | simplify-attributes | 1.000722 | 1.000721 | 1.074892 | 1.074893 | 28144658 / 28164980 / 26202584 |
| medium-smooth | simplify-update | 0.987536 | 0.987536 | 1.103521 | 1.103521 | 15319425 / 15128479 / 13709276 |
| medium-smooth | simplify-sloppy | 1.000002 | 0.999998 | 1.237873 | 1.237877 | 3898464 / 3898470 / 3149318 |
| medium-seams | simplify | 1.001306 | 1.001308 | 1.063918 | 1.063917 | 15226118 / 15246001 / 14330067 |
| medium-seams | simplify-attributes | 1.000701 | 1.000700 | 1.082422 | 1.082421 | 28411724 / 28431636 / 26266701 |
| medium-seams | simplify-update | 0.986611 | 0.986611 | 1.086844 | 1.086844 | 17870456 / 17631192 / 16222379 |
| medium-seams | simplify-sloppy | 0.999999 | 0.999998 | 1.373918 | 1.373917 | 7076112 / 7076103 / 5150312 |
| medium-sparse | simplify | 1.001571 | 1.001569 | 1.081131 | 1.081132 | 12940842 / 12961174 / 11988525 |
| medium-sparse | simplify-attributes | 1.000815 | 1.000816 | 1.074556 | 1.074556 | 24912021 / 24932318 / 23202434 |
| medium-sparse | simplify-update | 0.975902 | 0.975901 | 1.105836 | 1.105839 | 16689807 / 16287623 / 14728751 |
| medium-sparse | simplify-sloppy | 1.000001 | 1.000003 | 1.232740 | 1.232736 | 4773182 / 4773189 / 3872027 |
| million-smooth | simplify | 1.001033 | 1.001033 | 1.061882 | 1.061882 | 2417179040 / 2419676373 / 2278667716 |
| million-smooth | simplify-attributes | 1.000628 | 1.000628 | 1.080962 | 1.080962 | 3974933891 / 3977430706 / 3679527564 |
| million-smooth | simplify-update | 0.992023 | 0.992023 | 1.078139 | 1.078139 | 2837729958 / 2815093175 / 2611067379 |
| million-smooth | simplify-sloppy | 1.000000 | 1.000000 | 1.195200 | 1.195200 | 408963914 / 408963966 / 342171888 |
| real-214e | simplify | 1.001949 | 1.001949 | 1.106133 | 1.106132 | 3150525972 / 3156665795 / 2853787281 |
| real-214e | simplify-attributes | 1.001166 | 1.001166 | 1.060034 | 1.060033 | 5266265233 / 5272404914 / 4973810020 |
| real-214e | simplify-update | 0.922469 | 0.922469 | 1.192091 | 1.192091 | 4305157439 / 3971375887 / 3331437661 |
| real-214e | simplify-sloppy | 1.000000 | 1.000000 | 1.275663 | 1.275663 | 2947576401 / 2947576477 / 2310624005 |
| real-a082 | simplify | 1.001222 | 1.001222 | 1.129557 | 1.129557 | 5744853031 / 5751871196 / 5092146309 |
| real-a082 | simplify-attributes | 1.000834 | 1.000834 | 1.097548 | 1.097548 | 8413250997 / 8420269938 / 7671889181 |
| real-a082 | simplify-update | 0.948875 | 0.948875 | 1.180175 | 1.180175 | 6940780552 / 6585932860 / 5580472784 |
| real-a082 | simplify-sloppy | 1.000000 | 1.000000 | 1.250043 | 1.250043 | 2910437071 / 2910437171 / 2328268862 |
| real-b6fd | simplify | 1.001224 | 1.001224 | 1.132704 | 1.132704 | 1433656139 / 1435410669 / 1267242665 |
| real-b6fd | simplify-attributes | 1.000835 | 1.000835 | 1.099527 | 1.099527 | 2100678372 / 2102432863 / 1912124926 |
| real-b6fd | simplify-update | 0.948721 | 0.948722 | 1.182539 | 1.182539 | 1730016870 / 1641304124 / 1387949662 |
| real-b6fd | simplify-sloppy | 1.000000 | 1.000000 | 1.280695 | 1.280695 | 818980354 / 818980284 / 639480933 |

Wall time (milliseconds/call; no timing gate):

| Case | Function | Rust base default | Rust candidate default | C++ default | Rust base scalar | Rust candidate scalar | C++ scalar |
|---|---|---:|---:|---:|---:|---:|---:|
| tiny-smooth | simplify | 0.013 | 0.014 | 0.015 | 0.013 | 0.013 | 0.016 |
| tiny-smooth | simplify-attributes | 0.016 | 0.015 | 0.015 | 0.016 | 0.015 | 0.016 |
| tiny-smooth | simplify-update | 0.015 | 0.015 | 0.016 | 0.015 | 0.015 | 0.015 |
| tiny-smooth | simplify-sloppy | 0.002 | 0.002 | 0.002 | 0.002 | 0.002 | 0.002 |
| medium-smooth | simplify | 1.399 | 1.475 | 1.978 | 1.444 | 1.496 | 2.001 |
| medium-smooth | simplify-attributes | 3.716 | 3.733 | 4.418 | 3.620 | 3.632 | 4.258 |
| medium-smooth | simplify-update | 2.048 | 1.960 | 2.322 | 1.967 | 2.006 | 2.304 |
| medium-smooth | simplify-sloppy | 0.380 | 0.342 | 0.363 | 0.386 | 0.393 | 0.288 |
| medium-seams | simplify | 2.089 | 2.183 | 2.645 | 2.146 | 2.042 | 2.653 |
| medium-seams | simplify-attributes | 3.718 | 3.322 | 4.079 | 3.863 | 3.574 | 4.072 |
| medium-seams | simplify-update | 2.472 | 2.412 | 2.693 | 2.622 | 2.380 | 2.819 |
| medium-seams | simplify-sloppy | 0.686 | 0.659 | 0.694 | 0.716 | 0.654 | 0.706 |
| medium-sparse | simplify | 1.510 | 1.577 | 1.821 | 1.519 | 1.324 | 1.874 |
| medium-sparse | simplify-attributes | 3.028 | 2.945 | 3.683 | 2.986 | 2.977 | 4.388 |
| medium-sparse | simplify-update | 1.872 | 1.761 | 2.177 | 1.883 | 1.842 | 2.160 |
| medium-sparse | simplify-sloppy | 0.457 | 0.455 | 0.651 | 0.483 | 0.449 | 0.651 |
| million-smooth | simplify | 431.017 | 398.467 | 478.302 | 426.993 | 452.273 | 469.787 |
| million-smooth | simplify-attributes | 683.935 | 616.985 | 727.939 | 640.035 | 645.311 | 666.234 |
| million-smooth | simplify-update | 443.557 | 463.687 | 490.718 | 441.713 | 465.907 | 465.677 |
| million-smooth | simplify-sloppy | 68.517 | 64.414 | 83.233 | 65.007 | 69.569 | 82.999 |
| real-214e | simplify | 1636.115 | 1634.181 | 1926.607 | 1542.338 | 1600.602 | 1844.121 |
| real-214e | simplify-attributes | 1888.557 | 1801.528 | 2597.760 | 1998.785 | 1940.023 | 2662.763 |
| real-214e | simplify-update | 2041.807 | 1890.004 | 2268.150 | 2133.200 | 1908.816 | 2237.343 |
| real-214e | simplify-sloppy | 1044.377 | 1078.717 | 1121.278 | 988.509 | 990.007 | 1131.785 |
| real-a082 | simplify | 3937.862 | 4204.434 | 4627.531 | 4132.297 | 4077.550 | 4470.320 |
| real-a082 | simplify-attributes | 5442.337 | 6034.490 | 6532.099 | 5585.799 | 5598.521 | 6599.343 |
| real-a082 | simplify-update | 4696.723 | 4665.831 | 5106.648 | 4795.473 | 4893.625 | 5200.157 |
| real-a082 | simplify-sloppy | 1580.892 | 1474.368 | 1549.492 | 1487.304 | 1569.338 | 1521.215 |
| real-b6fd | simplify | 460.936 | 446.022 | 512.017 | 444.839 | 462.995 | 485.068 |
| real-b6fd | simplify-attributes | 518.823 | 480.462 | 707.671 | 539.874 | 547.935 | 770.444 |
| real-b6fd | simplify-update | 451.300 | 482.546 | 536.237 | 437.552 | 484.280 | 532.826 |
| real-b6fd | simplify-sloppy | 172.963 | 180.955 | 165.141 | 148.765 | 162.883 | 183.679 |
