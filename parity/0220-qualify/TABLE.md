# 0.2.20 combined release table

One instructions:u sample per case; fixed baseline/C++ arms cached by binary and input hash. Each entry is real-mesh GM / corpus GM of train/other. Ratios below 1 mean fewer instructions. Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. C++ 1.3 is matched scalar/default. Base is v0.2.19. Flag column: `>1%` marks a function with any case, or either group GM, more than 1% above v0.2.19 in that mode.

| Function | scalar / 0.2.19 | default / 0.2.19 | scalar / scalar C++ | default / default C++ | max case scalar / default vs base | flag |
|---|---:|---:|---:|---:|---:|---|
| simplify | 1.000 / 1.000 | 1.000 / 1.000 | 1.174 / 1.058 | 1.174 / 1.058 | 1.000001 / 1.000003 |  |
| simplify-attributes | 1.000 / 1.000 | 1.000 / 1.000 | 1.118 / 1.068 | 1.118 / 1.068 | 1.000000 / 1.000000 |  |
| simplify-sloppy | 1.000 / 1.000 | 1.000 / 1.000 | 1.284 / 1.281 | 1.284 / 1.281 | 1.000002 / 1.000005 |  |
| simplify-update | 1.000 / 1.000 | 1.000 / 1.000 | 1.238 / 1.076 | 1.238 / 1.076 | 1.000001 / 1.000001 |  |
| simplify-prune | 1.000 / 1.000 | 1.000 / 1.000 | 0.971 / 1.056 | 0.971 / 1.056 | 1.000001 / 1.000003 |  |
| simplify-points | 1.000 / 1.000 | 1.000 / 1.000 | 1.032 / 1.084 | 1.032 / 1.084 | 1.000004 / 1.000005 |  |
| cache | 1.000 / 1.000 | 1.000 / 1.000 | 0.706 / 1.048 | 0.706 / 1.048 | 1.000000 / 1.000001 |  |
| overdraw | 1.000 / 1.000 | 1.000 / 1.000 | 1.096 / 0.939 | 1.096 / 0.939 | 1.000008 / 1.000004 |  |
| fetch | 1.000 / 1.000 | 1.000 / 1.000 | 0.908 / 1.148 | 0.908 / 1.148 | 1.000014 / 1.000027 |  |
| meshlets | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.057 | 1.005 / 1.057 | 1.000000 / 1.000001 |  |
| meshlets-flex | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.058 | 1.005 / 1.058 | 1.000000 / 1.000001 |  |
| meshlets-spatial | 1.000 / 1.000 | 1.000 / 1.000 | 0.955 / 0.961 | 1.062 / 1.041 | 1.000001 / 1.000001 |  |
| cluster-lod | 1.000 / 1.000 | 1.000 / 1.000 | 1.224 / 1.172 | 1.223 / 1.167 | 1.000073 / 1.000029 |  |
| vertex-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.545 / 0.542 | 0.545 / 0.542 | 1.000009 / 1.000005 |  |
| vertex-decode | 1.000 / 1.000 | 1.000 / 1.000 | 1.128 / 1.171 | 0.979 / 1.124 | 1.001121 / 1.000012 |  |
| index-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.770 / 0.940 | 0.770 / 0.940 | 1.000000 / 1.000014 |  |
| index-decode | 1.000 / 1.000 | 1.000 / 1.000 | 0.994 / 1.042 | 1.024 / 1.084 | 1.000033 / 1.000007 |  |
| remap-vertex | 1.000 / 1.000 | 1.000 / 1.000 | 1.087 / 1.068 | 1.087 / 1.068 | 1.000001 / 1.000040 |  |
| remap-index | 1.000 / 1.000 | 1.000 / 1.000 | 1.354 / 1.314 | 1.354 / 1.314 | 1.000030 / 1.000045 |  |
| cluster-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.089 / 1.122 | 1.039 / 1.048 | 1.000003 / 1.000012 |  |
| meshlet-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.122 / 1.184 | 1.072 / 1.110 | 1.000011 / 1.000003 |  |

Campaign samples: 1008. Base/train output identity cells (scalar and default both): 168/168. Train/C++ output identity cells: 168/168.
Cases above 1.01 vs v0.2.19: 0. Functions flagged: none. Per-case medians and ratios: [release-results.json](release-results.json).
