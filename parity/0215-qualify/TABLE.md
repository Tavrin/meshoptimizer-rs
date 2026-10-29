# 0.2.15 release table (scalar, instructions:u)

Median of three per case; GM = geometric mean of per-case ratios (train / peer).
Real = real-214e, real-a082, real-b6fd; corpus = tiny, medium-smooth, medium-seams,
medium-sparse, million-smooth. SIMD off in every arm: meshoptimizer-rs without `simd`,
optimesh 1.1.0 without `simd`, C++ 1.3 (meshopt 0.7.0) built with MESHOPTIMIZER_NO_SIMD.
Train and v0.2.14 outputs are byte-identical in every cell.

| Function | vs 0.2.14 real | vs 0.2.14 corpus | max case vs 0.2.14 | vs optimesh real | vs optimesh corpus | vs C++ real | vs C++ corpus | same output as C++ | same as optimesh |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| simplify | 0.919 | 0.984 | 0.993 | 0.898 | 0.920 | 1.137 | 1.047 | 8/8 | 8/8 |
| simplify-attributes | 0.787 | 0.951 | 0.969 | 0.926 | 0.917 | 1.095 | 1.065 | 8/8 | 8/8 |
| simplify-sloppy | 0.845 | 0.916 | 0.946 | 0.706 | 0.882 | 1.275 | 1.268 | 8/8 | 8/8 |
| simplify-update | 0.936 | 0.987 | 0.994 | 1.008 | 0.972 | 1.280 | 1.085 | 8/8 | 8/8 |
| simplify-prune | 0.620 | 0.711 | 0.780 | 0.763 | 0.852 | 0.995 | 1.071 | 8/8 | 8/8 |
| simplify-points | 0.601 | 0.686 | 0.804 | 0.632 | 0.665 | 1.014 | 1.066 | 8/8 | 8/8 |
| cache | 0.995 | 0.966 | 0.996 | 0.957 | 0.934 | 1.306 | 1.273 | 8/8 | 8/8 |
| overdraw | 0.753 | 0.946 | 0.973 | 0.884 | 0.865 | 1.090 | 0.939 | 8/8 | 8/8 |
| fetch | 1.000 | 1.000 | 1.000 | 0.716 | 0.962 | 0.952 | 1.171 | 8/8 | 8/8 |
| meshlets | 0.826 | 0.975 | 0.976 | 0.784 | 0.778 | 1.001 | 1.068 | 8/8 | 8/8 |
| meshlets-flex | 0.826 | 0.975 | 0.976 | 0.784 | 0.778 | 1.002 | 1.068 | 8/8 | 8/8 |
| meshlets-spatial | 0.876 | 0.906 | 1.000 | 0.835 | 0.872 | 0.965 | 0.972 | 8/8 | 8/8 |
| cluster-lod | 0.909 | 0.973 | 0.985 | — | — | 1.189 | 1.176 | 8/8 | 0/0 |
| vertex-encode | 1.000 | 1.000 | 1.000 | 0.455 | 0.466 | 0.554 | 0.578 | 8/8 | 8/8 |
| vertex-decode | 1.000 | 1.000 | 1.000 | 0.544 | 0.644 | 1.061 | 1.114 | 8/8 | 8/8 |
| index-encode | 1.000 | 1.000 | 1.001 | 0.712 | 0.752 | 0.771 | 0.933 | 8/8 | 8/8 |
| index-decode | 1.000 | 1.000 | 1.000 | 0.907 | 0.915 | 0.994 | 1.044 | 8/8 | 8/8 |
| remap-vertex | 0.465 | 0.521 | 0.684 | 0.453 | 0.515 | 1.394 | 1.328 | 8/8 | 8/8 |
| remap-index | 0.772 | 0.791 | 0.866 | 0.975 | 0.986 | 1.625 | 1.570 | 8/8 | 8/8 |

Cases above 1.01 vs v0.2.14: none
