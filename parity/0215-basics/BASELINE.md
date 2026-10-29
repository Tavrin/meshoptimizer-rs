# Baseline: train/0.2.15 head vs C++ 1.3 and optimesh 1.1.0 (scalar)

Arms and method: `REGISTRATION.md` (median of three `instructions:u` samples,
CPU 9, SIMD off in every arm). Campaign `baseline` (arms base, cpp, rust),
648 samples, exit 0. Every base output is byte-identical to the C++ output in
every cell (`results.py`: same output as C++ = True for all nine ops).

Three-real-mesh geometric means (lower is better for ours):

| op | base/C++ | base/optimesh |
|---|---|---|
| remap | 0.696 | 0.576 |
| remap-multi | 1.031 | 0.934 |
| remap-custom | 0.979 | 0.730 |
| position-remap | 0.865 | 0.623 |
| remap-vertex | 3.000 | 0.975 |
| remap-index | 2.104 | 1.262 |
| shadow | 0.729 | 0.626 |
| shadow-multi | 1.083 | 1.003 |
| index-decode | 0.994 | 0.907 |

Per case, base/C++ (corpus: tiny, medium smooth/seams/sparse, million; then
the three real meshes):

| op | tiny | m-smooth | m-seams | m-sparse | million | 214e | a082 | b6fd |
|---|---|---|---|---|---|---|---|---|
| remap | 0.934 | 0.888 | 0.855 | 0.891 | 0.892 | 0.692 | 0.698 | 0.698 |
| remap-multi | 1.159 | 1.106 | 1.108 | 1.089 | 1.104 | 1.028 | 1.033 | 1.032 |
| remap-custom | 0.981 | 1.086 | 1.048 | 1.111 | 1.098 | 0.978 | 0.982 | 0.977 |
| position-remap | 1.030 | 0.923 | 0.915 | 0.873 | 0.920 | 0.863 | 0.866 | 0.866 |
| remap-vertex | 1.616 | 2.917 | 2.932 | 2.605 | 2.997 | 3.000 | 3.000 | 2.999 |
| remap-index | 1.604 | 2.088 | 2.088 | 2.088 | 2.104 | 2.104 | 2.104 | 2.104 |
| shadow | 0.995 | 1.084 | 1.018 | 1.093 | 1.096 | 0.721 | 0.733 | 0.733 |
| shadow-multi | 1.245 | 1.349 | 1.301 | 1.347 | 1.357 | 1.076 | 1.086 | 1.086 |
| index-decode | 1.096 | 1.044 | 1.036 | 1.003 | 1.042 | 0.924 | 1.151 | 0.923 |

Gaps (C++ or optimesh cheaper on the real meshes): remap-vertex, remap-index,
remap-multi, shadow-multi; index-decode on real-a082 only.
