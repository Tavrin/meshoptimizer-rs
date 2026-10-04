# P02 registered decoder bar

Registered 2026-10-03T22:43:07Z, before port measurement.

Moss commit `dc4af42a5e94f8a0f22932c53977f66cd88aadc2`, source SHA-256 `c1aa44ee92e536f2d812510c23b06b4cd6aa87a842f446316d0fe0d251cfa8ef`.

Baseline evidence: `/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02/baseline.json`, SHA-256 `8ab17783c4237b52d9b6ce157dd5f7ae696dd126611d860692ec5c8a8fec5d13`.

minimum = 0.95 * decoded_bytes / maximum measured Moss seconds; 12 alternating paired samples; shared host load; worst sample plus 5% margin. Minima apply to allocating and caller-buffer APIs on this Linux x86-64 target. Raw v1 is measured separately against scalar C++; it has no Moss baseline.

Optimized C++ uses enabled upstream SIMD; scalar canonical output is separate. Below 80% ships safe scalar under RFC 6.2, subject to these measured non-regression minima.

| Case | Moss MB/s | SIMD MB/s | Moss/SIMD | Minimum MB/s |
|---|---:|---:|---:|---:|
| vertex-v0-tiny-s4 | 43.559557 | 200.545067 | 0.217206 | 31.449675 |
| view-none-tiny-s4 | 44.194887 | 235.727009 | 0.187483 | 28.687669 |
| vertex-v0-tiny-s12 | 38.189780 | 167.670579 | 0.227767 | 29.977849 |
| view-none-tiny-s12 | 39.301259 | 278.132558 | 0.141304 | 23.094938 |
| vertex-v0-tiny-s32 | 41.467695 | 237.047566 | 0.174934 | 25.568107 |
| view-none-tiny-s32 | 33.283657 | 127.297879 | 0.261463 | 27.434069 |
| filter-1-tiny-s4 | 71.650785 | 262.673875 | 0.272775 | 54.493693 |
| view-1-tiny-s4 | 32.236260 | 58.368652 | 0.552287 | 25.682545 |
| filter-1-tiny-s8 | 88.045856 | 464.033877 | 0.189740 | 62.448952 |
| view-1-tiny-s8 | 42.190835 | 80.558996 | 0.523726 | 29.893286 |
| filter-2-tiny-s8 | 89.150034 | 303.346964 | 0.293888 | 69.695226 |
| view-2-tiny-s8 | 49.662226 | 87.290203 | 0.568932 | 37.140053 |
| filter-3-tiny-s12 | 500.474899 | 1045.903634 | 0.478510 | 299.082705 |
| view-3-tiny-s12 | 65.998416 | 114.959838 | 0.574100 | 53.680240 |
| index-2-v0-tiny-s2 | 92.550346 | 138.946976 | 0.666084 | 50.314322 |
| index-2-v0-tiny-s4 | 225.714140 | 326.145991 | 0.692065 | 86.524645 |
| index-2-v1-tiny-s2 | 87.925751 | 127.705376 | 0.688505 | 33.772389 |
| index-2-v1-tiny-s4 | 197.630867 | 333.065678 | 0.593369 | 86.584598 |
| index-3-v0-tiny-s2 | 101.759177 | 132.472754 | 0.768152 | 56.970509 |
| index-3-v0-tiny-s4 | 323.123853 | 418.169266 | 0.772711 | 255.716137 |
| index-3-v1-tiny-s2 | 153.346978 | 192.391700 | 0.797056 | 70.230567 |
| index-3-v1-tiny-s4 | 199.941517 | 337.252070 | 0.592855 | 106.687542 |
| vertex-v0-resident-s4 | 49.269114 | 302.386200 | 0.162934 | 42.540464 |
| view-none-resident-s4 | 54.267682 | 299.368040 | 0.181274 | 45.106015 |
| vertex-v0-resident-s12 | 50.378052 | 358.476298 | 0.140534 | 41.401612 |
| view-none-resident-s12 | 55.528847 | 353.928941 | 0.156893 | 46.430042 |
| vertex-v0-resident-s32 | 45.854868 | 263.522933 | 0.174007 | 34.434508 |
| view-none-resident-s32 | 43.353795 | 367.010213 | 0.118127 | 31.062066 |
| filter-1-resident-s4 | 65.426923 | 283.826885 | 0.230517 | 54.046022 |
| view-1-resident-s4 | 43.873013 | 225.253476 | 0.194772 | 38.155265 |
| filter-1-resident-s8 | 80.440309 | 490.901929 | 0.163862 | 67.587991 |
| view-1-resident-s8 | 47.928872 | 272.699343 | 0.175757 | 40.755074 |
| filter-2-resident-s8 | 89.751262 | 537.349611 | 0.167026 | 78.022810 |
| view-2-resident-s8 | 48.071718 | 257.602878 | 0.186612 | 29.690370 |
| filter-3-resident-s12 | 387.007991 | 1273.612873 | 0.303866 | 261.555184 |
| view-3-resident-s12 | 74.664967 | 376.397420 | 0.198367 | 61.608252 |
| index-2-v0-resident-s2 | 84.588232 | 136.231572 | 0.620915 | 68.286732 |
| index-2-v0-resident-s4 | 195.161565 | 298.513032 | 0.653779 | 164.458866 |
| index-2-v1-resident-s2 | 95.483588 | 157.504095 | 0.606229 | 64.306004 |
| index-2-v1-resident-s4 | 199.248229 | 340.113000 | 0.585829 | 153.462728 |
| index-3-v0-resident-s2 | 139.714083 | 165.757776 | 0.842881 | 99.416658 |
| index-3-v0-resident-s4 | 211.391458 | 278.497871 | 0.759042 | 175.366576 |
| index-3-v1-resident-s2 | 110.133993 | 128.541104 | 0.856800 | 85.982248 |
| index-3-v1-resident-s4 | 229.512747 | 283.540509 | 0.809453 | 182.498253 |
| vertex-v0-streaming-s4 | 39.824763 | 123.739421 | 0.321844 | 29.471897 |
| view-none-streaming-s4 | 30.795105 | 103.784182 | 0.296723 | 23.726007 |
| vertex-v0-streaming-s12 | 34.568500 | 113.303879 | 0.305095 | 29.918674 |
| view-none-streaming-s12 | 33.946611 | 98.091467 | 0.346071 | 26.299879 |
| vertex-v0-streaming-s32 | 36.305293 | 103.360078 | 0.351251 | 16.386993 |
| view-none-streaming-s32 | 30.894640 | 97.227550 | 0.317756 | 20.641352 |
| filter-1-streaming-s4 | 49.982855 | 126.045519 | 0.396546 | 31.997494 |
| view-1-streaming-s4 | 46.033551 | 201.317296 | 0.228662 | 30.999614 |
| filter-1-streaming-s8 | 62.237544 | 149.909805 | 0.415167 | 45.307264 |
| view-1-streaming-s8 | 40.381759 | 125.027278 | 0.322984 | 25.090923 |
| filter-2-streaming-s8 | 68.747217 | 190.630671 | 0.360630 | 34.174145 |
| view-2-streaming-s8 | 37.821579 | 134.666386 | 0.280854 | 31.755568 |
| filter-3-streaming-s12 | 127.000576 | 187.254868 | 0.678223 | 104.122162 |
| view-3-streaming-s12 | 55.987545 | 141.203718 | 0.396502 | 43.687753 |
| index-2-v0-streaming-s2 | 72.415343 | 84.718068 | 0.854780 | 50.933805 |
| index-2-v0-streaming-s4 | 106.171686 | 166.321689 | 0.638351 | 74.994162 |
| index-2-v1-streaming-s2 | 92.974241 | 138.937943 | 0.669178 | 62.024313 |
| index-2-v1-streaming-s4 | 185.894028 | 299.552910 | 0.620572 | 91.627679 |
| index-3-v0-streaming-s2 | 169.619224 | 172.984395 | 0.980546 | 104.336956 |
| index-3-v0-streaming-s4 | 282.393566 | 367.713933 | 0.767971 | 101.144797 |
| index-3-v1-streaming-s2 | 197.873221 | 257.224363 | 0.769263 | 140.141445 |
| index-3-v1-streaming-s4 | 376.112965 | 483.899391 | 0.777254 | 236.671885 |
