# 0.2.19 combined release table

Median of three instructions:u samples per case. Each entry is real-mesh GM / corpus GM of train/other. Ratios below 1 mean fewer instructions. Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. C++ 1.3 is matched scalar/default. Base is v0.2.18. Flag column: `>1%` marks a function with any case, or either group GM, more than 1% above v0.2.18 in that mode.

| Function | scalar / 0.2.18 | default / 0.2.18 | scalar / scalar C++ | default / default C++ | max case scalar / default vs base | flag |
|---|---:|---:|---:|---:|---:|---|
| simplify | 1.000 / 1.000 | 1.000 / 1.000 | 1.174 / 1.058 | 1.174 / 1.058 | 1.000000 / 1.000001 |  |
| simplify-attributes | 1.000 / 1.000 | 1.000 / 1.000 | 1.118 / 1.068 | 1.118 / 1.068 | 1.000000 / 1.000000 |  |
| simplify-sloppy | 1.000 / 1.000 | 1.000 / 1.000 | 1.284 / 1.281 | 1.284 / 1.281 | 1.000002 / 1.000001 |  |
| simplify-update | 1.000 / 1.000 | 1.000 / 1.000 | 1.238 / 1.076 | 1.238 / 1.076 | 1.000000 / 1.000001 |  |
| simplify-prune | 1.000 / 1.000 | 1.000 / 1.000 | 0.971 / 1.056 | 0.971 / 1.056 | 1.000001 / 1.000004 |  |
| simplify-points | 1.000 / 1.000 | 1.000 / 1.000 | 1.032 / 1.084 | 1.032 / 1.084 | 1.000002 / 1.000006 |  |
| cache | 1.000 / 1.000 | 1.000 / 1.000 | 0.706 / 1.048 | 0.706 / 1.048 | 1.000002 / 1.000001 |  |
| overdraw | 1.000 / 1.000 | 1.000 / 1.000 | 1.096 / 0.939 | 1.096 / 0.939 | 1.000004 / 1.000002 |  |
| fetch | 1.000 / 1.000 | 1.000 / 1.000 | 0.908 / 1.148 | 0.908 / 1.148 | 1.000029 / 1.000007 |  |
| meshlets | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.057 | 1.005 / 1.057 | 1.000000 / 1.000000 |  |
| meshlets-flex | 1.000 / 1.000 | 1.000 / 1.000 | 1.005 / 1.058 | 1.005 / 1.058 | 1.000001 / 1.000000 |  |
| meshlets-spatial | 1.000 / 1.000 | 1.000 / 1.000 | 0.955 / 0.961 | 1.062 / 1.041 | 1.000000 / 1.000000 |  |
| cluster-lod | 1.000 / 1.000 | 1.000 / 1.000 | 1.224 / 1.172 | 1.223 / 1.167 | 1.000040 / 1.000066 |  |
| vertex-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.545 / 0.542 | 0.545 / 0.542 | 1.000005 / 1.000006 |  |
| vertex-decode | 1.000 / 1.000 | 1.000 / 1.000 | 1.128 / 1.171 | 0.979 / 1.124 | 1.001113 / 1.000023 |  |
| index-encode | 1.000 / 1.000 | 1.000 / 1.000 | 0.770 / 0.940 | 0.770 / 0.940 | 1.000008 / 1.000004 |  |
| index-decode | 1.000 / 1.000 | 1.000 / 1.000 | 0.994 / 1.042 | 1.024 / 1.084 | 1.000008 / 1.000004 |  |
| remap-vertex | 1.000 / 1.000 | 1.000 / 1.000 | 1.087 / 1.068 | 1.087 / 1.068 | 1.000098 / 1.000145 |  |
| remap-index | 1.000 / 1.000 | 1.000 / 1.000 | 1.354 / 1.314 | 1.354 / 1.314 | 1.000015 / 1.000015 |  |
| cluster-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.089 / 1.122 | 1.039 / 1.048 | 1.000001 / 1.000012 |  |
| meshlet-bounds | 1.000 / 1.000 | 1.000 / 1.000 | 1.122 / 1.184 | 1.072 / 1.110 | 1.000000 / 1.000005 |  |

Campaign samples: 3024. Base/train output identity cells (scalar and default both): 168/168. Train/C++ output identity cells: 168/168.
Cases above 1.01 vs v0.2.18: 0. Functions flagged: none. Per-case medians and ratios: [release-results.json](release-results.json).

# Final virtual-geometry qualification measurements

Candidate v2 builder and MDAG 2 attribute payloads against the pinned reference. Five interleaved repeats; median build plus serialise milliseconds. Ratios below 1 favor the candidate.

| Mesh | Build ratio 1T / 8T | File B/DAG tri ours / ref | Resident B/DAG tri ours / ref | Cut tris ratio at 5 / 8 px | Measured/claimed median ours / ref |
|---|---:|---:|---:|---:|---:|
| bunny | 2.023 / 0.863 | 13.06 / 13.01 | 14.84 / 13.78 | 0.676 / 0.928 | 1.000 / 1.161 |
| coast_rocks_01 | 1.352 / 0.566 | 12.49 / 12.49 | 14.09 / 13.23 | 1.114 / 1.671 | 0.980 / 1.166 |
| dead_tree_trunk_02 | 1.889 / 0.741 | 13.29 / 13.84 | 14.97 / 14.52 | 0.710 / 0.782 | 0.905 / 1.199 |
| dragon | 1.298 / 0.520 | 12.58 / 11.68 | 14.36 / 13.32 | 0.818 / 1.298 | 0.930 / 1.437 |
| namaqualand_boulder_02 | 1.715 / 0.656 | 13.31 / 13.93 | 15.06 / 14.67 | 0.952 / 0.971 | 0.979 / 1.100 |

## C++ decoder differential observations

| Run | Cases | Success differences | Forced ISA differences | Failed-call destination differences |
|---|---:|---:|---:|---:|
| decode-all/differential.json | 318608 | 0 | 0 | 74863 |
| decode-all/extended/differential.json | 318608 | 0 | 0 | 149727 |
| decode-none/differential-no-std.json | 318608 | 0 | 0 | 74863 |
| vdecode-all/differential.json | 318608 | 0 | 0 | 74863 |
| vdecode-none/differential-no-std.json | 318608 | 0 | 0 | 74863 |
