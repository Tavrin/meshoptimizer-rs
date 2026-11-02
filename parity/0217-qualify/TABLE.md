# 0.2.17 combined release table

Median of three instructions:u samples per case. Each entry is real-mesh GM / corpus GM of train/other. Ratios below 1 mean fewer instructions. Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. C++ 1.3 is matched scalar/default. Base is v0.2.16. Flag column: `>1%` marks a function with any case, or either group GM, more than 1% above v0.2.16 in that mode.

| Function | scalar / 0.2.16 | default / 0.2.16 | scalar / scalar C++ | default / default C++ | max case scalar / default vs base | flag |
|---|---:|---:|---:|---:|---:|---|
| simplify | 0.964 / 0.970 | 0.964 / 0.970 | 1.174 / 1.058 | 1.174 / 1.058 | 0.977279 / 0.977278 |  |
| simplify-attributes | 0.976 / 0.984 | 0.976 / 0.984 | 1.118 / 1.068 | 1.118 / 1.068 | 0.988764 / 0.988764 |  |
| simplify-sloppy | 1.000 / 1.000 | 1.000 / 1.000 | 1.284 / 1.281 | 1.284 / 1.281 | 1.000783 / 1.000780 |  |
| simplify-update | 0.969 / 0.974 | 0.969 / 0.974 | 1.238 / 1.076 | 1.238 / 1.076 | 0.981588 / 0.981588 |  |
| simplify-prune | 1.000 / 1.000 | 1.000 / 1.000 | 0.971 / 1.056 | 0.971 / 1.056 | 1.000006 / 1.000004 |  |
| simplify-points | 1.000 / 1.000 | 1.000 / 1.000 | 1.032 / 1.084 | 1.032 / 1.084 | 1.001451 / 1.001449 |  |
| cache | 0.566 / 0.845 | 0.566 / 0.845 | 0.706 / 1.048 | 0.706 / 1.048 | 0.857301 / 0.857294 |  |
| overdraw | 1.000 / 1.000 | 1.000 / 1.000 | 1.096 / 0.939 | 1.096 / 0.939 | 1.000580 / 1.000579 |  |
| fetch | 1.000 / 1.001 | 1.000 / 1.001 | 0.908 / 1.148 | 0.908 / 1.148 | 1.002541 / 1.002539 |  |
| meshlets | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.057 | 1.005 / 1.057 | 1.000707 / 1.000708 |  |
| meshlets-flex | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.058 | 1.005 / 1.058 | 1.001602 / 1.001482 |  |
| meshlets-spatial | 0.982 / 0.985 | 0.991 / 0.991 | 0.955 / 0.961 | 1.062 / 1.041 | 0.995745 / 0.995680 |  |
| cluster-lod | 1.000 / 1.000 | 1.000 / 1.000 | 1.224 / 1.172 | 1.223 / 1.167 | 1.000011 / 1.000010 |  |
| vertex-encode | 0.992 / 0.998 | 0.992 / 0.998 | 0.545 / 0.542 | 0.545 / 0.542 | 1.003783 / 1.003783 |  |
| vertex-decode | 0.934 / 0.945 | 0.849 / 0.898 | 1.128 / 1.171 | 0.979 / 1.124 | 0.973127 / 0.955555 |  |
| index-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.770 / 0.940 | 0.770 / 0.940 | 1.000002 / 1.000006 |  |
| index-decode | 1.000 / 0.998 | 0.999 / 0.993 | 0.994 / 1.042 | 1.024 / 1.084 | 1.000000 / 1.000005 |  |
| remap-vertex | 1.000 / 1.000 | 1.000 / 1.000 | 1.087 / 1.068 | 1.087 / 1.068 | 1.000150 / 1.000159 |  |
| remap-index | 1.000 / 1.001 | 1.000 / 1.001 | 1.354 / 1.314 | 1.354 / 1.314 | 1.003125 / 1.003145 |  |
| cluster-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.089 / 1.122 | 1.039 / 1.048 | 1.000017 / 1.000056 |  |
| meshlet-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.122 / 1.184 | 1.072 / 1.110 | 1.000022 / 1.000009 |  |

Campaign samples: 3024. Base/train output identity cells (scalar and default both): 168/168. Train/C++ output identity cells: 168/168.
Cases above 1.01 vs v0.2.16: 0. Functions flagged: none. Per-case medians and ratios: [release-results.json](release-results.json).
