# Measured geometry performance

Single-thread RFC 6.1 matrix. One warm-up and at least ten alternating paired samples per attempt.
High-load cases are repeated once; both attempts and all raw samples are retained. Persistent load above 4 limits timing confidence.

| Family | Cases | Geometric mean | Maximum | Maximum memory ratio | Verdict |
|---|---:|---:|---:|---:|---|
| vertex_cache | 24 | 1.322 | 1.477 | 1.000 | FAIL |
| overdraw | 24 | 1.272 | 1.477 | 0.999 | FAIL |
| simplify | 72 | 1.313 | 1.574 | 1.064 | FAIL |
| simplify_with_attributes | 72 | 1.313 | 1.490 | 1.043 | FAIL |
| simplify_scale | 12 | 0.929 | 1.063 | 1.000 | PASS |

Overall: **FAIL**. Required: geometric mean ≤1.25, every case ≤1.50, memory ≤1.25.
Load average start: (11.9150390625, 11.50048828125, 10.32421875); end: (6.701171875, 10.7138671875, 11.47607421875).

Scale has one allocation-free API, so it is measured once per geometry. Caller-buffer measurements reuse output and Workspace after warm-up; allocating measurements use fresh scratch.
Seam-heavy meshes have duplicate patch boundaries and discontinuous attributes; disconnected meshes have separated patches; sparse meshes reference half the supplied vertices.
Attribute counts 1, 8 and 12 are paired with target ratios 0.5, 0.25 and 0.125; plain simplification covers all three ratios on every geometry.
Raw samples, dispersion, per-attempt load, identities and per-case memory are in `results/benchmark.json`.

| Case | Rust ms | C++ ms | Time ratio | Memory ratio |
|---|---:|---:|---:|---:|
| vertex_cache/tiny/smooth/allocating/a0/rNone | 0.001563 | 0.001138 | 1.374 | 1.000 |
| vertex_cache/tiny/smooth/caller-buffer/a0/rNone | 0.001433 | 0.001309 | 1.094 | 1.000 |
| overdraw/tiny/smooth/allocating/a0/rNone | 0.001178 | 0.001141 | 1.032 | 0.995 |
| overdraw/tiny/smooth/caller-buffer/a0/rNone | 0.001110 | 0.001097 | 1.012 | 0.989 |
| simplify_scale/tiny/smooth/allocation-free/a0/rNone | 0.000058 | 0.000055 | 1.063 | 1.000 |
| simplify/tiny/smooth/allocating/a0/r0.5 | 0.008986 | 0.006420 | 1.400 | 1.029 |
| simplify/tiny/smooth/caller-buffer/a0/r0.5 | 0.009177 | 0.006653 | 1.379 | 1.032 |
| simplify/tiny/smooth/allocating/a0/r0.25 | 0.010876 | 0.008606 | 1.264 | 1.029 |
| simplify/tiny/smooth/caller-buffer/a0/r0.25 | 0.010827 | 0.008592 | 1.260 | 1.032 |
| simplify/tiny/smooth/allocating/a0/r0.125 | 0.009524 | 0.007616 | 1.251 | 1.029 |
| simplify/tiny/smooth/caller-buffer/a0/r0.125 | 0.009341 | 0.007043 | 1.326 | 1.032 |
| simplify_with_attributes/tiny/smooth/allocating/a1/r0.5 | 0.012471 | 0.009312 | 1.339 | 1.021 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a1/r0.5 | 0.012443 | 0.009368 | 1.328 | 1.023 |
| simplify_with_attributes/tiny/smooth/allocating/a8/r0.25 | 0.014800 | 0.009932 | 1.490 | 1.014 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a8/r0.25 | 0.013027 | 0.008991 | 1.449 | 1.014 |
| simplify_with_attributes/tiny/smooth/allocating/a12/r0.125 | 0.014927 | 0.010282 | 1.452 | 1.012 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a12/r0.125 | 0.014372 | 0.010240 | 1.404 | 1.013 |
| vertex_cache/tiny/seam-heavy/allocating/a0/rNone | 0.001361 | 0.000985 | 1.383 | 1.000 |
| vertex_cache/tiny/seam-heavy/caller-buffer/a0/rNone | 0.001430 | 0.000969 | 1.477 | 1.000 |
| overdraw/tiny/seam-heavy/allocating/a0/rNone | 0.001251 | 0.001146 | 1.092 | 0.981 |
| overdraw/tiny/seam-heavy/caller-buffer/a0/rNone | 0.001196 | 0.001133 | 1.055 | 0.964 |
| simplify_scale/tiny/seam-heavy/allocation-free/a0/rNone | 0.000089 | 0.000086 | 1.032 | 1.000 |
| simplify/tiny/seam-heavy/allocating/a0/r0.5 | 0.009044 | 0.006418 | 1.409 | 1.048 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.5 | 0.009038 | 0.006302 | 1.434 | 1.051 |
| simplify/tiny/seam-heavy/allocating/a0/r0.25 | 0.011367 | 0.008606 | 1.321 | 1.048 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.25 | 0.011288 | 0.008658 | 1.304 | 1.051 |
| simplify/tiny/seam-heavy/allocating/a0/r0.125 | 0.012522 | 0.009684 | 1.293 | 1.048 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.125 | 0.012326 | 0.009712 | 1.269 | 1.051 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a1/r0.5 | 0.014079 | 0.010335 | 1.362 | 1.033 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a1/r0.5 | 0.013400 | 0.010112 | 1.325 | 1.035 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a8/r0.25 | 0.014830 | 0.010292 | 1.441 | 1.020 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a8/r0.25 | 0.014754 | 0.010082 | 1.463 | 1.021 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a12/r0.125 | 0.016236 | 0.011239 | 1.445 | 1.018 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a12/r0.125 | 0.016394 | 0.011724 | 1.398 | 1.018 |
| vertex_cache/tiny/disconnected/allocating/a0/rNone | 0.001452 | 0.001052 | 1.380 | 1.000 |
| vertex_cache/tiny/disconnected/caller-buffer/a0/rNone | 0.001426 | 0.001089 | 1.309 | 1.000 |
| overdraw/tiny/disconnected/allocating/a0/rNone | 0.001381 | 0.001245 | 1.109 | 0.981 |
| overdraw/tiny/disconnected/caller-buffer/a0/rNone | 0.001278 | 0.001213 | 1.054 | 0.964 |
| simplify_scale/tiny/disconnected/allocation-free/a0/rNone | 0.000089 | 0.000089 | 0.999 | 1.000 |
| simplify/tiny/disconnected/allocating/a0/r0.5 | 0.010298 | 0.007211 | 1.428 | 1.047 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.5 | 0.010362 | 0.007230 | 1.433 | 1.050 |
| simplify/tiny/disconnected/allocating/a0/r0.25 | 0.014362 | 0.010929 | 1.314 | 1.047 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.25 | 0.013757 | 0.010383 | 1.325 | 1.050 |
| simplify/tiny/disconnected/allocating/a0/r0.125 | 0.013632 | 0.010568 | 1.290 | 1.047 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.125 | 0.013950 | 0.010707 | 1.303 | 1.050 |
| simplify_with_attributes/tiny/disconnected/allocating/a1/r0.5 | 0.013703 | 0.009886 | 1.386 | 1.033 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a1/r0.5 | 0.013766 | 0.009999 | 1.377 | 1.035 |
| simplify_with_attributes/tiny/disconnected/allocating/a8/r0.25 | 0.015778 | 0.010932 | 1.443 | 1.020 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a8/r0.25 | 0.015095 | 0.010326 | 1.462 | 1.021 |
| simplify_with_attributes/tiny/disconnected/allocating/a12/r0.125 | 0.018330 | 0.014030 | 1.306 | 1.018 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a12/r0.125 | 0.018584 | 0.012761 | 1.456 | 1.018 |
| vertex_cache/tiny/sparse/allocating/a0/rNone | 0.001635 | 0.001187 | 1.377 | 1.000 |
| vertex_cache/tiny/sparse/caller-buffer/a0/rNone | 0.001640 | 0.001236 | 1.326 | 1.000 |
| overdraw/tiny/sparse/allocating/a0/rNone | 0.001257 | 0.001209 | 1.040 | 0.995 |
| overdraw/tiny/sparse/caller-buffer/a0/rNone | 0.001243 | 0.001246 | 0.997 | 0.991 |
| simplify_scale/tiny/sparse/allocation-free/a0/rNone | 0.000116 | 0.000122 | 0.955 | 1.000 |
| simplify/tiny/sparse/allocating/a0/r0.5 | 0.010355 | 0.007582 | 1.366 | 1.040 |
| simplify/tiny/sparse/caller-buffer/a0/r0.5 | 0.010445 | 0.007727 | 1.352 | 1.042 |
| simplify/tiny/sparse/allocating/a0/r0.25 | 0.012229 | 0.009409 | 1.300 | 1.040 |
| simplify/tiny/sparse/caller-buffer/a0/r0.25 | 0.012315 | 0.009729 | 1.266 | 1.042 |
| simplify/tiny/sparse/allocating/a0/r0.125 | 0.011134 | 0.008286 | 1.344 | 1.040 |
| simplify/tiny/sparse/caller-buffer/a0/r0.125 | 0.011057 | 0.008265 | 1.338 | 1.042 |
| simplify_with_attributes/tiny/sparse/allocating/a1/r0.5 | 0.013392 | 0.010013 | 1.337 | 1.027 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a1/r0.5 | 0.013053 | 0.009993 | 1.306 | 1.028 |
| simplify_with_attributes/tiny/sparse/allocating/a8/r0.25 | 0.016792 | 0.012032 | 1.396 | 1.015 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a8/r0.25 | 0.016530 | 0.011566 | 1.429 | 1.016 |
| simplify_with_attributes/tiny/sparse/allocating/a12/r0.125 | 0.017065 | 0.011911 | 1.433 | 1.014 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a12/r0.125 | 0.016745 | 0.011531 | 1.452 | 1.014 |
| vertex_cache/medium/smooth/allocating/a0/rNone | 0.566723 | 0.429030 | 1.321 | 1.000 |
| vertex_cache/medium/smooth/caller-buffer/a0/rNone | 0.539241 | 0.404985 | 1.332 | 1.000 |
| overdraw/medium/smooth/allocating/a0/rNone | 0.142973 | 0.098044 | 1.458 | 0.999 |
| overdraw/medium/smooth/caller-buffer/a0/rNone | 0.134713 | 0.096847 | 1.391 | 0.998 |
| simplify_scale/medium/smooth/allocation-free/a0/rNone | 0.008476 | 0.009563 | 0.886 | 1.000 |
| simplify/medium/smooth/allocating/a0/r0.5 | 1.316534 | 0.973145 | 1.353 | 1.039 |
| simplify/medium/smooth/caller-buffer/a0/r0.5 | 1.333006 | 0.987913 | 1.349 | 1.044 |
| simplify/medium/smooth/allocating/a0/r0.25 | 1.536538 | 1.163338 | 1.321 | 1.039 |
| simplify/medium/smooth/caller-buffer/a0/r0.25 | 1.547052 | 1.187172 | 1.303 | 1.044 |
| simplify/medium/smooth/allocating/a0/r0.125 | 1.680283 | 1.297770 | 1.295 | 1.039 |
| simplify/medium/smooth/caller-buffer/a0/r0.125 | 1.701356 | 1.305785 | 1.303 | 1.044 |
| simplify_with_attributes/medium/smooth/allocating/a1/r0.5 | 1.862063 | 1.445126 | 1.289 | 1.029 |
| simplify_with_attributes/medium/smooth/caller-buffer/a1/r0.5 | 1.799300 | 1.332329 | 1.350 | 1.032 |
| simplify_with_attributes/medium/smooth/allocating/a8/r0.25 | 3.380432 | 2.505014 | 1.349 | 1.019 |
| simplify_with_attributes/medium/smooth/caller-buffer/a8/r0.25 | 3.541600 | 2.583777 | 1.371 | 1.020 |
| simplify_with_attributes/medium/smooth/allocating/a12/r0.125 | 3.983598 | 3.081160 | 1.293 | 1.017 |
| simplify_with_attributes/medium/smooth/caller-buffer/a12/r0.125 | 4.114404 | 3.180787 | 1.294 | 1.018 |
| vertex_cache/medium/seam-heavy/allocating/a0/rNone | 0.539217 | 0.409293 | 1.317 | 1.000 |
| vertex_cache/medium/seam-heavy/caller-buffer/a0/rNone | 0.503269 | 0.370800 | 1.357 | 1.000 |
| overdraw/medium/seam-heavy/allocating/a0/rNone | 0.143730 | 0.102908 | 1.397 | 0.999 |
| overdraw/medium/seam-heavy/caller-buffer/a0/rNone | 0.138515 | 0.099021 | 1.399 | 0.997 |
| simplify_scale/medium/seam-heavy/allocation-free/a0/rNone | 0.011111 | 0.012404 | 0.896 | 1.000 |
| simplify/medium/seam-heavy/allocating/a0/r0.5 | 2.006279 | 1.547789 | 1.296 | 1.035 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.5 | 1.943937 | 1.488022 | 1.306 | 1.039 |
| simplify/medium/seam-heavy/allocating/a0/r0.25 | 2.582585 | 2.015937 | 1.281 | 1.035 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.25 | 2.607447 | 2.041730 | 1.277 | 1.039 |
| simplify/medium/seam-heavy/allocating/a0/r0.125 | 3.563912 | 2.758154 | 1.292 | 1.035 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.125 | 2.964562 | 2.330879 | 1.272 | 1.039 |
| simplify_with_attributes/medium/seam-heavy/allocating/a1/r0.5 | 2.582234 | 2.084982 | 1.238 | 1.025 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a1/r0.5 | 2.740492 | 2.129054 | 1.287 | 1.027 |
| simplify_with_attributes/medium/seam-heavy/allocating/a8/r0.25 | 3.848254 | 2.801225 | 1.374 | 1.016 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a8/r0.25 | 4.008806 | 2.926290 | 1.370 | 1.017 |
| simplify_with_attributes/medium/seam-heavy/allocating/a12/r0.125 | 4.784782 | 3.716648 | 1.287 | 1.015 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a12/r0.125 | 5.294848 | 4.045104 | 1.309 | 1.015 |
| vertex_cache/medium/disconnected/allocating/a0/rNone | 0.677366 | 0.517441 | 1.309 | 1.000 |
| vertex_cache/medium/disconnected/caller-buffer/a0/rNone | 0.560252 | 0.410550 | 1.365 | 1.000 |
| overdraw/medium/disconnected/allocating/a0/rNone | 0.155271 | 0.107547 | 1.444 | 0.999 |
| overdraw/medium/disconnected/caller-buffer/a0/rNone | 0.143029 | 0.102296 | 1.398 | 0.997 |
| simplify_scale/medium/disconnected/allocation-free/a0/rNone | 0.010781 | 0.012278 | 0.878 | 1.000 |
| simplify/medium/disconnected/allocating/a0/r0.5 | 1.907728 | 1.506090 | 1.267 | 1.034 |
| simplify/medium/disconnected/caller-buffer/a0/r0.5 | 1.989111 | 1.522621 | 1.306 | 1.038 |
| simplify/medium/disconnected/allocating/a0/r0.25 | 2.792624 | 2.198479 | 1.270 | 1.034 |
| simplify/medium/disconnected/caller-buffer/a0/r0.25 | 2.838285 | 2.231116 | 1.272 | 1.038 |
| simplify/medium/disconnected/allocating/a0/r0.125 | 7.113941 | 5.160742 | 1.378 | 1.034 |
| simplify/medium/disconnected/caller-buffer/a0/r0.125 | 5.813276 | 4.385463 | 1.326 | 1.038 |
| simplify_with_attributes/medium/disconnected/allocating/a1/r0.5 | 5.634330 | 4.310036 | 1.307 | 1.025 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a1/r0.5 | 6.017458 | 4.267436 | 1.410 | 1.027 |
| simplify_with_attributes/medium/disconnected/allocating/a8/r0.25 | 5.585374 | 3.978114 | 1.404 | 1.016 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a8/r0.25 | 4.302977 | 2.994638 | 1.437 | 1.017 |
| simplify_with_attributes/medium/disconnected/allocating/a12/r0.125 | 5.979513 | 4.578570 | 1.306 | 1.015 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a12/r0.125 | 5.979522 | 4.483446 | 1.334 | 1.015 |
| vertex_cache/medium/sparse/allocating/a0/rNone | 0.753619 | 0.597661 | 1.261 | 1.000 |
| vertex_cache/medium/sparse/caller-buffer/a0/rNone | 0.751961 | 0.561083 | 1.340 | 1.000 |
| overdraw/medium/sparse/allocating/a0/rNone | 0.181741 | 0.132168 | 1.375 | 0.999 |
| overdraw/medium/sparse/caller-buffer/a0/rNone | 0.202680 | 0.146329 | 1.385 | 0.998 |
| simplify_scale/medium/sparse/allocation-free/a0/rNone | 0.019121 | 0.020779 | 0.920 | 1.000 |
| simplify/medium/sparse/allocating/a0/r0.5 | 1.804430 | 1.188089 | 1.519 | 1.055 |
| simplify/medium/sparse/caller-buffer/a0/r0.5 | 1.834466 | 1.165346 | 1.574 | 1.060 |
| simplify/medium/sparse/allocating/a0/r0.25 | 1.808102 | 1.253361 | 1.443 | 1.055 |
| simplify/medium/sparse/caller-buffer/a0/r0.25 | 1.752513 | 1.222223 | 1.434 | 1.060 |
| simplify/medium/sparse/allocating/a0/r0.125 | 1.868165 | 1.343465 | 1.391 | 1.055 |
| simplify/medium/sparse/caller-buffer/a0/r0.125 | 1.887486 | 1.327485 | 1.422 | 1.060 |
| simplify_with_attributes/medium/sparse/allocating/a1/r0.5 | 2.181157 | 1.707508 | 1.277 | 1.038 |
| simplify_with_attributes/medium/sparse/caller-buffer/a1/r0.5 | 2.258708 | 1.722887 | 1.311 | 1.040 |
| simplify_with_attributes/medium/sparse/allocating/a8/r0.25 | 5.013712 | 3.878366 | 1.293 | 1.022 |
| simplify_with_attributes/medium/sparse/caller-buffer/a8/r0.25 | 4.701140 | 3.544089 | 1.326 | 1.023 |
| simplify_with_attributes/medium/sparse/allocating/a12/r0.125 | 5.154410 | 3.881011 | 1.328 | 1.020 |
| simplify_with_attributes/medium/sparse/caller-buffer/a12/r0.125 | 11.782004 | 8.500537 | 1.386 | 1.021 |
| vertex_cache/million/smooth/allocating/a0/rNone | 331.994642 | 248.460318 | 1.336 | 1.000 |
| vertex_cache/million/smooth/caller-buffer/a0/rNone | 386.059251 | 307.361745 | 1.256 | 1.000 |
| overdraw/million/smooth/allocating/a0/rNone | 25.824518 | 19.440126 | 1.328 | 0.999 |
| overdraw/million/smooth/caller-buffer/a0/rNone | 25.143666 | 18.467075 | 1.362 | 0.998 |
| simplify_scale/million/smooth/allocation-free/a0/rNone | 1.219548 | 1.385945 | 0.880 | 1.000 |
| simplify/million/smooth/allocating/a0/r0.5 | 370.139929 | 294.236687 | 1.258 | 1.041 |
| simplify/million/smooth/caller-buffer/a0/r0.5 | 356.367221 | 271.938640 | 1.310 | 1.047 |
| simplify/million/smooth/allocating/a0/r0.25 | 472.393738 | 385.509343 | 1.225 | 1.041 |
| simplify/million/smooth/caller-buffer/a0/r0.25 | 472.935665 | 375.541022 | 1.259 | 1.047 |
| simplify/million/smooth/allocating/a0/r0.125 | 445.102198 | 359.908431 | 1.237 | 1.041 |
| simplify/million/smooth/caller-buffer/a0/r0.125 | 471.186383 | 370.945572 | 1.270 | 1.047 |
| simplify_with_attributes/million/smooth/allocating/a1/r0.5 | 568.975324 | 477.127403 | 1.193 | 1.031 |
| simplify_with_attributes/million/smooth/caller-buffer/a1/r0.5 | 599.480884 | 496.094932 | 1.208 | 1.035 |
| simplify_with_attributes/million/smooth/allocating/a8/r0.25 | 700.383898 | 554.980964 | 1.262 | 1.021 |
| simplify_with_attributes/million/smooth/caller-buffer/a8/r0.25 | 689.397973 | 543.999829 | 1.267 | 1.022 |
| simplify_with_attributes/million/smooth/allocating/a12/r0.125 | 840.073768 | 669.717937 | 1.254 | 1.019 |
| simplify_with_attributes/million/smooth/caller-buffer/a12/r0.125 | 778.374442 | 634.017476 | 1.228 | 1.020 |
| vertex_cache/million/seam-heavy/allocating/a0/rNone | 61.486075 | 44.453798 | 1.383 | 1.000 |
| vertex_cache/million/seam-heavy/caller-buffer/a0/rNone | 60.156582 | 43.649368 | 1.378 | 1.000 |
| overdraw/million/seam-heavy/allocating/a0/rNone | 17.991442 | 12.901228 | 1.395 | 0.999 |
| overdraw/million/seam-heavy/caller-buffer/a0/rNone | 18.011766 | 12.677669 | 1.421 | 0.997 |
| simplify_scale/million/seam-heavy/allocation-free/a0/rNone | 1.272397 | 1.420996 | 0.895 | 1.000 |
| simplify/million/seam-heavy/allocating/a0/r0.5 | 411.757907 | 313.460767 | 1.314 | 1.037 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.5 | 417.566309 | 321.551355 | 1.299 | 1.042 |
| simplify/million/seam-heavy/allocating/a0/r0.25 | 534.983905 | 420.398142 | 1.273 | 1.037 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.25 | 506.955122 | 398.037512 | 1.274 | 1.042 |
| simplify/million/seam-heavy/allocating/a0/r0.125 | 547.561021 | 432.690237 | 1.265 | 1.037 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.125 | 575.335131 | 457.833407 | 1.257 | 1.042 |
| simplify_with_attributes/million/seam-heavy/allocating/a1/r0.5 | 628.292687 | 549.521423 | 1.143 | 1.027 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a1/r0.5 | 593.464557 | 501.740489 | 1.183 | 1.029 |
| simplify_with_attributes/million/seam-heavy/allocating/a8/r0.25 | 870.455795 | 721.199620 | 1.207 | 1.017 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a8/r0.25 | 879.751359 | 743.930609 | 1.183 | 1.018 |
| simplify_with_attributes/million/seam-heavy/allocating/a12/r0.125 | 1091.695998 | 885.395799 | 1.233 | 1.015 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a12/r0.125 | 1003.088356 | 882.036546 | 1.137 | 1.016 |
| vertex_cache/million/disconnected/allocating/a0/rNone | 60.643445 | 43.071028 | 1.408 | 1.000 |
| vertex_cache/million/disconnected/caller-buffer/a0/rNone | 59.706437 | 42.387616 | 1.409 | 1.000 |
| overdraw/million/disconnected/allocating/a0/rNone | 17.413839 | 12.320764 | 1.413 | 0.999 |
| overdraw/million/disconnected/caller-buffer/a0/rNone | 17.730083 | 12.001726 | 1.477 | 0.997 |
| simplify_scale/million/disconnected/allocation-free/a0/rNone | 1.216983 | 1.365427 | 0.891 | 1.000 |
| simplify/million/disconnected/allocating/a0/r0.5 | 450.085968 | 353.089407 | 1.275 | 1.036 |
| simplify/million/disconnected/caller-buffer/a0/r0.5 | 485.335832 | 365.156852 | 1.329 | 1.040 |
| simplify/million/disconnected/allocating/a0/r0.25 | 547.608494 | 435.097187 | 1.259 | 1.036 |
| simplify/million/disconnected/caller-buffer/a0/r0.25 | 543.716052 | 422.755289 | 1.286 | 1.040 |
| simplify/million/disconnected/allocating/a0/r0.125 | 578.941882 | 456.701373 | 1.268 | 1.036 |
| simplify/million/disconnected/caller-buffer/a0/r0.125 | 556.848333 | 434.148423 | 1.283 | 1.040 |
| simplify_with_attributes/million/disconnected/allocating/a1/r0.5 | 595.771440 | 492.136952 | 1.211 | 1.027 |
| simplify_with_attributes/million/disconnected/caller-buffer/a1/r0.5 | 704.837684 | 577.287934 | 1.221 | 1.029 |
| simplify_with_attributes/million/disconnected/allocating/a8/r0.25 | 740.441047 | 583.449669 | 1.269 | 1.017 |
| simplify_with_attributes/million/disconnected/caller-buffer/a8/r0.25 | 768.823143 | 635.512907 | 1.210 | 1.018 |
| simplify_with_attributes/million/disconnected/allocating/a12/r0.125 | 824.302044 | 676.050792 | 1.219 | 1.015 |
| simplify_with_attributes/million/disconnected/caller-buffer/a12/r0.125 | 849.996353 | 683.878333 | 1.243 | 1.016 |
| vertex_cache/million/sparse/allocating/a0/rNone | 89.826077 | 76.450519 | 1.175 | 1.000 |
| vertex_cache/million/sparse/caller-buffer/a0/rNone | 84.915110 | 74.399557 | 1.141 | 1.000 |
| overdraw/million/sparse/allocating/a0/rNone | 22.108151 | 15.685678 | 1.409 | 0.999 |
| overdraw/million/sparse/caller-buffer/a0/rNone | 21.313841 | 15.546586 | 1.371 | 0.998 |
| simplify_scale/million/sparse/allocation-free/a0/rNone | 2.223912 | 2.542425 | 0.875 | 1.000 |
| simplify/million/sparse/allocating/a0/r0.5 | 604.303306 | 508.401885 | 1.189 | 1.059 |
| simplify/million/sparse/caller-buffer/a0/r0.5 | 659.709812 | 488.470223 | 1.351 | 1.064 |
| simplify/million/sparse/allocating/a0/r0.25 | 644.431970 | 515.001938 | 1.251 | 1.059 |
| simplify/million/sparse/caller-buffer/a0/r0.25 | 769.432441 | 615.077865 | 1.251 | 1.064 |
| simplify/million/sparse/allocating/a0/r0.125 | 596.716999 | 486.152916 | 1.227 | 1.059 |
| simplify/million/sparse/caller-buffer/a0/r0.125 | 657.773885 | 539.331501 | 1.220 | 1.064 |
| simplify_with_attributes/million/sparse/allocating/a1/r0.5 | 778.677446 | 632.863662 | 1.230 | 1.041 |
| simplify_with_attributes/million/sparse/caller-buffer/a1/r0.5 | 707.324589 | 588.040742 | 1.203 | 1.043 |
| simplify_with_attributes/million/sparse/allocating/a8/r0.25 | 1050.796774 | 836.899673 | 1.256 | 1.024 |
| simplify_with_attributes/million/sparse/caller-buffer/a8/r0.25 | 971.893122 | 799.492145 | 1.216 | 1.025 |
| simplify_with_attributes/million/sparse/allocating/a12/r0.125 | 1088.397275 | 900.202873 | 1.209 | 1.022 |
| simplify_with_attributes/million/sparse/caller-buffer/a12/r0.125 | 995.978347 | 845.311960 | 1.178 | 1.022 |
