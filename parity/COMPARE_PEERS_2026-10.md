# Peer screen — meshoptimizer-rs 0.2.4, meshopt 0.7.0, optimesh 1.1.0

Campaign: `/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150615Z`. Draft PR #12. Safety: PASS under the 2026-10-25 amendment; first review reused with identical dependency tree and reviewed-source hashes.

**Optimesh beats ours anywhere (registered rule, updated by rerun below): NO.** Cluster LOD is unavailable from its published public API. Browser decode and non-equivalent quality checks are assessed below. No general Rust-library ranking or Moss/product claim follows.

A/A floor F = 0.011548126031385553; complete = True. Thresholds: peer upper < 1-F-0.02; ours lower > 1+F+0.02. Other equivalent results are EQUAL WITHIN NOISE. Non-equivalent outputs keep that label regardless of speed.

Ratios are peer/O; GM, nominal paired 95% interval, and max case. Five alternating pairs extend once to twenty iff the initial interval straddles equality. Family aggregation is registered in REGISTRATION.md. Shared host, selected physical CPU, warm workspace, allocation/copies/scratch/destruction included. No adjusted significance claim.

Coverage: defaults scalar/SIMD M7 and X, and Moss scalar M7 and X are complete (438 family rows). Moss SIMD M7 retains 19 operation/case rows but no complete family; Moss SIMD X has no samples. Both browser profiles are unmeasured. All 150 missing family rows and both unavailable cluster-LOD arms have explicit reasons below.

Audit: PASS (`audit.json`), 36,790 retained native arm samples including A/A. Original safety review and library source identities match.

## Correctness

- O: identical: 1444.
- M7: different but valid: 24, identical: 1420.
- X: different but valid: 44, identical: 1400.

Full per-case labels, hashes, triangle counts and reported errors: `correctness.json`. Scalar C++ 1.3 from meshopt 0.7.0 is the oracle; M7 SIMD filters can differ from canonical scalar bytes. Meshlet padding normalization is outside timing. A valid simplifier output does not establish equal geometric quality.

## Per-peer family table

| Peer | Profile | Mode | Family | GM | 95% interval | Max case | Verdict | Output metrics |
|---|---|---|---|---:|---|---:|---|---|
| M7 | defaults | scalar | vertex-decode | 0.882145 | [0.867295, 0.897249] | 0.972744 | PEER FASTER |  |
| M7 | defaults | scalar | vertex-encode | 1.669965 | [1.629944, 1.710969] | 1.791277 | OURS FASTER |  |
| M7 | defaults | scalar | vertex-encode-v0 | 1.166673 | [1.141050, 1.192872] | 1.218147 | OURS FASTER |  |
| M7 | defaults | scalar | vertex-encode-default | 1.851218 | [1.437996, 2.383182] | 2.588238 | OURS FASTER |  |
| M7 | defaults | scalar | index-decode | 0.944726 | [0.931256, 0.958390] | 1.006617 | PEER FASTER |  |
| M7 | defaults | scalar | index-encode | 1.204524 | [1.178106, 1.231534] | 1.467921 | OURS FASTER |  |
| M7 | defaults | scalar | sequence-decode | 1.039360 | [1.029074, 1.049750] | 1.148364 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | sequence-encode | 1.043899 | [1.029305, 1.058699] | 1.061665 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | oct-decode | 0.223069 | [0.214884, 0.231566] | 0.267514 | PEER FASTER |  |
| M7 | defaults | scalar | quat-decode | 0.283116 | [0.275076, 0.291391] | 0.304270 | PEER FASTER |  |
| M7 | defaults | scalar | exp-decode | 1.041553 | [1.023121, 1.060318] | 1.082050 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | color-decode | 0.590300 | [0.580732, 0.600025] | 0.639721 | PEER FASTER |  |
| M7 | defaults | scalar | oct-encode | 1.160295 | [1.133486, 1.187740] | 1.304820 | OURS FASTER |  |
| M7 | defaults | scalar | quat-encode | 1.138557 | [1.108770, 1.169145] | 1.251284 | OURS FASTER |  |
| M7 | defaults | scalar | exp-encode | 5.933374 | [5.766229, 6.105365] | 6.771389 | OURS FASTER |  |
| M7 | defaults | scalar | color-encode | 1.270206 | [1.243203, 1.297796] | 1.322721 | OURS FASTER |  |
| M7 | defaults | scalar | cache | 0.769141 | [0.740719, 0.798654] | 0.831468 | PEER FASTER |  |
| M7 | defaults | scalar | cache-strip | 0.728526 | [0.560195, 0.947438] | 0.993811 | PEER FASTER |  |
| M7 | defaults | scalar | cache-fifo | 0.824954 | [0.797454, 0.853403] | 1.272474 | PEER FASTER |  |
| M7 | defaults | scalar | overdraw | 0.848844 | [0.815258, 0.883814] | 1.282320 | PEER FASTER |  |
| M7 | defaults | scalar | fetch | 0.847592 | [0.809229, 0.887774] | 0.976971 | PEER FASTER |  |
| M7 | defaults | scalar | fetch-remap | 0.573244 | [0.564832, 0.581782] | 0.629998 | PEER FASTER |  |
| M7 | defaults | scalar | simplify | 1.036313 | [1.014491, 1.058605] | 1.223394 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | stripify | 0.915922 | [0.897931, 0.934273] | 1.059938 | PEER FASTER |  |
| M7 | defaults | scalar | unstripify | 1.007229 | [0.984523, 1.030458] | 1.022656 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | meshlets | 0.907620 | [0.893289, 0.922181] | 0.992677 | PEER FASTER |  |
| M7 | defaults | scalar | meshlets-scan | 0.641010 | [0.604567, 0.679651] | 0.735264 | PEER FASTER |  |
| M7 | defaults | scalar | meshlets-flex | 0.866697 | [0.751182, 0.999976] | 1.002116 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | meshlets-spatial | 0.896362 | [0.876579, 0.916591] | 0.932054 | PEER FASTER |  |
| M7 | defaults | scalar | vertex-version | 1.002151 | [0.988288, 1.016209] | 1.016015 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | index-version | 1.003677 | [0.987649, 1.019964] | 1.046543 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | vertex-bound | 1.016171 | [0.995480, 1.037292] | 1.045138 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | index-bound | 1.056371 | [1.027216, 1.086353] | 1.132115 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | sequence-bound | 1.083024 | [1.057886, 1.108758] | 1.196703 | OURS FASTER |  |
| M7 | defaults | scalar | strip-bound | 0.970915 | [0.950208, 0.992073] | 1.005905 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | unstrip-bound | 0.960562 | [0.916571, 1.006664] | 1.228029 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | meshlet-bound | 1.019681 | [0.998050, 1.041781] | 1.165604 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | remap | 0.880156 | [0.863662, 0.896966] | 0.926922 | PEER FASTER |  |
| M7 | defaults | scalar | remap-multi | 0.973134 | [0.954419, 0.992216] | 1.058535 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | remap-custom | 0.888846 | [0.862208, 0.916307] | 1.072987 | PEER FASTER |  |
| M7 | defaults | scalar | remap-vertices | 0.845538 | [0.814759, 0.877480] | 0.946532 | PEER FASTER |  |
| M7 | defaults | scalar | remap-indices | 0.704718 | [0.684076, 0.725983] | 0.847671 | PEER FASTER |  |
| M7 | defaults | scalar | shadow | 0.901837 | [0.880211, 0.923995] | 0.958079 | PEER FASTER |  |
| M7 | defaults | scalar | shadow-multi | 0.981116 | [0.960462, 1.002215] | 1.118683 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | position-remap | 1.211817 | [1.171029, 1.254026] | 1.326492 | OURS FASTER |  |
| M7 | defaults | scalar | adjacency | 0.752620 | [0.736459, 0.769135] | 0.804636 | PEER FASTER |  |
| M7 | defaults | scalar | tessellation | 0.803528 | [0.787106, 0.820292] | 0.855561 | PEER FASTER |  |
| M7 | defaults | scalar | provoking | 0.862675 | [0.845085, 0.880632] | 0.959599 | PEER FASTER |  |
| M7 | defaults | scalar | simplify-attributes | 1.070685 | [1.052294, 1.089399] | 1.223574 | OURS FASTER |  |
| M7 | defaults | scalar | simplify-sloppy | 0.769181 | [0.739830, 0.799698] | 0.834106 | PEER FASTER |  |
| M7 | defaults | scalar | simplify-prune | 0.836042 | [0.822994, 0.849297] | 0.937526 | PEER FASTER |  |
| M7 | defaults | scalar | simplify-points | 0.861268 | [0.814413, 0.910818] | 1.344742 | PEER FASTER |  |
| M7 | defaults | scalar | simplify-update | 0.972588 | [0.962015, 0.983277] | 1.063037 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | analyze-cache | 0.820148 | [0.802601, 0.838079] | 0.837280 | PEER FASTER |  |
| M7 | defaults | scalar | analyze-fetch | 1.053460 | [1.037621, 1.069541] | 1.070978 | OURS FASTER |  |
| M7 | defaults | scalar | analyze-overdraw | 0.965556 | [0.945096, 0.986459] | 1.166310 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | analyze-coverage | 0.955034 | [0.940118, 0.970187] | 1.124602 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | cluster-bounds | 0.187997 | [0.182923, 0.193211] | 0.957810 | PEER FASTER |  |
| M7 | defaults | scalar | sphere-bounds | 1.058668 | [1.033469, 1.084482] | 1.136584 | OURS FASTER |  |
| M7 | defaults | scalar | meshlet-bounds | 0.023421 | [0.022273, 0.024627] | 0.589921 | PEER FASTER |  |
| M7 | defaults | scalar | optimize-meshlet | 0.725323 | [0.708976, 0.742046] | 0.761438 | PEER FASTER |  |
| M7 | defaults | scalar | partition | 0.645948 | [0.625804, 0.666740] | 0.775953 | PEER FASTER |  |
| M7 | defaults | scalar | spatial-remap | 0.952544 | [0.931286, 0.974287] | 1.742033 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | spatial-triangles | 0.921152 | [0.903624, 0.939020] | 1.759989 | PEER FASTER |  |
| M7 | defaults | scalar | spatial-points | 0.856865 | [0.830521, 0.884045] | 0.956042 | PEER FASTER |  |
| M7 | defaults | scalar | quantize-unorm | 1.647704 | [1.344384, 2.019460] | 1.748274 | OURS FASTER |  |
| M7 | defaults | scalar | quantize-snorm | 1.015792 | [0.913521, 1.129513] | 1.031027 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | quantize-half | 2.495731 | [2.213179, 2.814356] | 2.821754 | OURS FASTER |  |
| M7 | defaults | scalar | dequantize-half | 3.386018 | [3.153213, 3.636012] | 3.408494 | OURS FASTER |  |
| M7 | defaults | scalar | quantize-float | 2.286682 | [2.013387, 2.597074] | 3.233839 | OURS FASTER |  |
| M7 | defaults | scalar | simplify-scale | 0.577359 | [0.501232, 0.665048] | 0.652421 | PEER FASTER |  |
| M7 | defaults | scalar | real-simplify | 0.949935 | [0.912596, 0.988802] | 0.986220 | EQUAL WITHIN NOISE |  |
| M7 | defaults | scalar | real-overdraw | 0.682062 | [0.665839, 0.698680] | 0.686241 | PEER FASTER |  |
| X | defaults | scalar | vertex-decode | 1.398470 | [1.343740, 1.455429] | 1.704249 | OURS FASTER |  |
| X | defaults | scalar | vertex-encode | 2.108784 | [2.022311, 2.198954] | 2.548402 | OURS FASTER |  |
| X | defaults | scalar | vertex-encode-v0 | 1.461641 | [1.441595, 1.481965] | 1.532120 | OURS FASTER |  |
| X | defaults | scalar | vertex-encode-default | 2.061642 | [2.033100, 2.090583] | 2.392333 | OURS FASTER |  |
| X | defaults | scalar | index-decode | 1.227977 | [1.217362, 1.238684] | 1.534868 | OURS FASTER |  |
| X | defaults | scalar | index-encode | 1.374421 | [1.349064, 1.400255] | 1.541204 | OURS FASTER |  |
| X | defaults | scalar | sequence-decode | 0.957101 | [0.942895, 0.971521] | 0.999177 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | sequence-encode | 1.124580 | [1.106555, 1.142899] | 1.158936 | OURS FASTER |  |
| X | defaults | scalar | oct-decode | 0.380813 | [0.373145, 0.388638] | 0.686020 | PEER FASTER |  |
| X | defaults | scalar | quat-decode | 0.464097 | [0.449268, 0.479416] | 0.774235 | PEER FASTER |  |
| X | defaults | scalar | exp-decode | 2.094437 | [1.989146, 2.205301] | 14.822759 | OURS FASTER |  |
| X | defaults | scalar | color-decode | 1.371414 | [1.331389, 1.412643] | 2.497868 | OURS FASTER |  |
| X | defaults | scalar | oct-encode | 2.063653 | [2.027965, 2.099969] | 4.115531 | OURS FASTER |  |
| X | defaults | scalar | quat-encode | 2.382079 | [2.330114, 2.435204] | 4.340863 | OURS FASTER |  |
| X | defaults | scalar | exp-encode | 6.304202 | [6.220551, 6.388978] | 6.990592 | OURS FASTER |  |
| X | defaults | scalar | color-encode | 1.582425 | [1.562850, 1.602246] | 1.604063 | OURS FASTER |  |
| X | defaults | scalar | cache | 1.033102 | [1.020031, 1.046341] | 1.212947 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | cache-strip | 1.019166 | [1.010540, 1.027867] | 1.167486 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | cache-fifo | 0.971078 | [0.955487, 0.986924] | 1.409484 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | overdraw | 0.799197 | [0.782600, 0.816146] | 0.925253 | PEER FASTER |  |
| X | defaults | scalar | fetch | 0.868472 | [0.854046, 0.883141] | 1.011479 | PEER FASTER |  |
| X | defaults | scalar | fetch-remap | 0.559522 | [0.538589, 0.581267] | 0.794445 | PEER FASTER |  |
| X | defaults | scalar | simplify | 1.081737 | [1.064070, 1.099697] | 1.333248 | OURS FASTER |  |
| X | defaults | scalar | stripify | 0.870974 | [0.844144, 0.898656] | 1.092967 | PEER FASTER |  |
| X | defaults | scalar | unstripify | 1.230576 | [1.193394, 1.268917] | 1.310769 | OURS FASTER |  |
| X | defaults | scalar | meshlets | 1.036031 | [1.022185, 1.050064] | 1.202588 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | meshlets-scan | 1.017879 | [1.001422, 1.034606] | 1.084964 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | meshlets-flex | 1.022356 | [1.010069, 1.034793] | 1.196417 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | meshlets-spatial | 0.942431 | [0.930676, 0.954334] | 1.003887 | PEER FASTER |  |
| X | defaults | scalar | vertex-version | 0.985520 | [0.969814, 1.001480] | 1.005132 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | index-version | 0.908843 | [0.882238, 0.936251] | 0.929547 | PEER FASTER |  |
| X | defaults | scalar | vertex-bound | 1.022009 | [1.007767, 1.036453] | 1.038019 | NOT EQUIVALENT | tiny-smooth: value 3189/3156; medium-smooth: value 53685/53124; medium-seams: value 66309/65616; medium-sparse: value 107337/106215; million-smooth: value 6198417/6133605 |
| X | defaults | scalar | index-bound | 1.121580 | [1.098771, 1.144862] | 1.331841 | OURS FASTER |  |
| X | defaults | scalar | sequence-bound | 1.143522 | [1.117601, 1.170044] | 1.244623 | OURS FASTER |  |
| X | defaults | scalar | strip-bound | 1.018855 | [1.000893, 1.037139] | 1.054381 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | unstrip-bound | 1.033084 | [1.018146, 1.048242] | 1.068181 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | meshlet-bound | 0.963455 | [0.936278, 0.991421] | 1.019412 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | remap | 0.942178 | [0.927935, 0.956641] | 1.030288 | PEER FASTER |  |
| X | defaults | scalar | remap-multi | 0.980351 | [0.962080, 0.998969] | 1.074803 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | remap-custom | 0.962166 | [0.944154, 0.980522] | 1.335231 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | remap-vertices | 0.985468 | [0.963070, 1.008387] | 1.099877 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | remap-indices | 0.774767 | [0.759299, 0.790550] | 0.881981 | PEER FASTER |  |
| X | defaults | scalar | shadow | 0.932185 | [0.917955, 0.946636] | 1.036941 | PEER FASTER |  |
| X | defaults | scalar | shadow-multi | 0.976941 | [0.953098, 1.001381] | 1.108422 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | position-remap | 1.389784 | [1.365563, 1.414435] | 1.621378 | OURS FASTER |  |
| X | defaults | scalar | adjacency | 0.719246 | [0.705808, 0.732941] | 0.785624 | PEER FASTER |  |
| X | defaults | scalar | tessellation | 0.778683 | [0.764430, 0.793201] | 0.840815 | PEER FASTER |  |
| X | defaults | scalar | provoking | 0.919892 | [0.905324, 0.934694] | 0.970959 | PEER FASTER |  |
| X | defaults | scalar | simplify-attributes | 1.089199 | [1.081338, 1.097118] | 1.249301 | OURS FASTER |  |
| X | defaults | scalar | simplify-sloppy | 1.009678 | [0.996563, 1.022965] | 1.159672 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | simplify-prune | 0.840983 | [0.831102, 0.850981] | 0.908122 | PEER FASTER |  |
| X | defaults | scalar | simplify-points | 1.238017 | [1.216720, 1.259686] | 1.411952 | OURS FASTER |  |
| X | defaults | scalar | simplify-update | 1.003375 | [0.988425, 1.018551] | 1.162995 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | analyze-cache | 0.995598 | [0.958327, 1.034318] | 1.018132 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | analyze-fetch | 1.217755 | [1.165011, 1.272886] | 1.278813 | OURS FASTER |  |
| X | defaults | scalar | analyze-overdraw | 1.815387 | [1.673556, 1.969238] | 2.769752 | OURS FASTER |  |
| X | defaults | scalar | analyze-coverage | 1.851268 | [1.800554, 1.903410] | 2.754575 | OURS FASTER |  |
| X | defaults | scalar | cluster-bounds | 0.199987 | [0.196329, 0.203713] | 0.991296 | PEER FASTER |  |
| X | defaults | scalar | sphere-bounds | 1.256275 | [1.241716, 1.271004] | 1.355830 | OURS FASTER |  |
| X | defaults | scalar | meshlet-bounds | 0.029426 | [0.020972, 0.041286] | 0.845310 | PEER FASTER |  |
| X | defaults | scalar | optimize-meshlet | 0.759980 | [0.748725, 0.771404] | 0.780801 | PEER FASTER |  |
| X | defaults | scalar | partition | 0.774486 | [0.751927, 0.797721] | 0.865702 | PEER FASTER |  |
| X | defaults | scalar | spatial-remap | 1.085191 | [1.055634, 1.115575] | 1.623719 | OURS FASTER |  |
| X | defaults | scalar | spatial-triangles | 0.977371 | [0.945666, 1.010139] | 1.575946 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | spatial-points | 0.895518 | [0.865324, 0.926764] | 0.957237 | PEER FASTER |  |
| X | defaults | scalar | quantize-unorm | 1.669184 | [1.388226, 2.007004] | 1.816878 | OURS FASTER |  |
| X | defaults | scalar | quantize-snorm | 1.028098 | [0.927308, 1.139843] | 1.064903 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | quantize-half | 0.660090 | [0.591966, 0.736054] | 0.811794 | PEER FASTER |  |
| X | defaults | scalar | dequantize-half | 0.947925 | [0.915928, 0.981039] | 0.997646 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | quantize-float | 0.691268 | [0.598120, 0.798923] | 0.973215 | PEER FASTER |  |
| X | defaults | scalar | simplify-scale | 0.676713 | [0.613838, 0.746029] | 0.813326 | PEER FASTER |  |
| X | defaults | scalar | real-simplify | 1.013124 | [0.985518, 1.041503] | 1.104800 | EQUAL WITHIN NOISE |  |
| X | defaults | scalar | real-overdraw | 0.681195 | [0.655770, 0.707606] | 0.726704 | PEER FASTER |  |
| M7 | defaults | simd | vertex-decode | 0.785830 | [0.756306, 0.816506] | 0.901030 | PEER FASTER |  |
| M7 | defaults | simd | vertex-encode | 1.695450 | [1.659921, 1.731739] | 1.798449 | OURS FASTER |  |
| M7 | defaults | simd | vertex-encode-v0 | 1.173477 | [1.154339, 1.192933] | 1.224504 | OURS FASTER |  |
| M7 | defaults | simd | vertex-encode-default | 1.660124 | [1.612884, 1.708748] | 1.767547 | OURS FASTER |  |
| M7 | defaults | simd | index-decode | 0.955679 | [0.949715, 0.961681] | 1.019990 | PEER FASTER |  |
| M7 | defaults | simd | index-encode | 1.169455 | [1.151244, 1.187954] | 1.408529 | OURS FASTER |  |
| M7 | defaults | simd | sequence-decode | 1.058856 | [1.033478, 1.084856] | 1.198024 | OURS FASTER |  |
| M7 | defaults | simd | sequence-encode | 1.043158 | [1.035706, 1.050665] | 1.057574 | OURS FASTER |  |
| M7 | defaults | simd | oct-decode | 0.922389 | [0.914853, 0.929987] | 1.065470 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA 2eb57e89/2eb57e89; medium-smooth: 33800 bytes, SHA 6f30ca1f/d3cc5cbd; medium-seams: 41472 bytes, SHA d510948d/2849811d; medium-sparse: 67600 bytes, SHA fc87093c/e33f49c1; million-smooth: 4021448 bytes, SHA 4d172c17/a446e4a7 |
| M7 | defaults | simd | quat-decode | 0.760511 | [0.755466, 0.765590] | 0.804937 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA 27c350ac/27c350ac; medium-smooth: 33800 bytes, SHA 83cdcd1d/a01bee8b; medium-seams: 41472 bytes, SHA dee8c3bb/f0162e34; medium-sparse: 67600 bytes, SHA 5a76235d/35d9bf78; million-smooth: 4021448 bytes, SHA d8a7e4a9/179baa9d |
| M7 | defaults | simd | exp-decode | 1.056631 | [1.030553, 1.083370] | 1.092345 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | color-decode | 1.076924 | [1.056410, 1.097836] | 1.109478 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA a6afb10f/a6afb10f; medium-smooth: 33800 bytes, SHA 87f29967/0959d662; medium-seams: 41472 bytes, SHA a7d7162e/2b3f2d54; medium-sparse: 67600 bytes, SHA 719b1634/46d56078; million-smooth: 4021448 bytes, SHA 6c391c91/65521c27 |
| M7 | defaults | simd | oct-encode | 1.151891 | [1.126297, 1.178066] | 1.255382 | OURS FASTER |  |
| M7 | defaults | simd | quat-encode | 1.136063 | [1.104663, 1.168355] | 1.308202 | OURS FASTER |  |
| M7 | defaults | simd | exp-encode | 5.809212 | [5.666195, 5.955838] | 6.772965 | OURS FASTER |  |
| M7 | defaults | simd | color-encode | 1.176869 | [1.157736, 1.196319] | 1.226607 | OURS FASTER |  |
| M7 | defaults | simd | cache | 0.853055 | [0.702977, 1.035173] | 1.463114 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | cache-strip | 0.813975 | [0.803937, 0.824138] | 1.085492 | PEER FASTER |  |
| M7 | defaults | simd | cache-fifo | 0.816126 | [0.802411, 0.830075] | 1.278667 | PEER FASTER |  |
| M7 | defaults | simd | overdraw | 0.795324 | [0.785935, 0.804825] | 0.986186 | PEER FASTER |  |
| M7 | defaults | simd | fetch | 0.892339 | [0.876577, 0.908384] | 0.951151 | PEER FASTER |  |
| M7 | defaults | simd | fetch-remap | 0.789603 | [0.779254, 0.800089] | 0.877282 | PEER FASTER |  |
| M7 | defaults | simd | simplify | 1.015115 | [0.996757, 1.033811] | 1.197751 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | stripify | 0.926713 | [0.903729, 0.950281] | 1.050495 | PEER FASTER |  |
| M7 | defaults | simd | unstripify | 0.981456 | [0.966487, 0.996657] | 1.026332 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | meshlets | 0.902155 | [0.893633, 0.910759] | 1.003339 | PEER FASTER |  |
| M7 | defaults | simd | meshlets-scan | 0.646182 | [0.611640, 0.682676] | 0.741515 | PEER FASTER |  |
| M7 | defaults | simd | meshlets-flex | 0.896928 | [0.892184, 0.901698] | 0.983636 | PEER FASTER |  |
| M7 | defaults | simd | meshlets-spatial | 0.799975 | [0.790516, 0.809548] | 0.887784 | PEER FASTER |  |
| M7 | defaults | simd | vertex-version | 1.053946 | [1.032311, 1.076035] | 1.118948 | OURS FASTER |  |
| M7 | defaults | simd | index-version | 0.992156 | [0.975559, 1.009036] | 1.010398 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | vertex-bound | 1.076482 | [1.051457, 1.102103] | 1.110635 | OURS FASTER |  |
| M7 | defaults | simd | index-bound | 1.077311 | [1.050189, 1.105133] | 1.156481 | OURS FASTER |  |
| M7 | defaults | simd | sequence-bound | 1.114827 | [1.079982, 1.150796] | 1.164077 | OURS FASTER |  |
| M7 | defaults | simd | strip-bound | 0.932115 | [0.902957, 0.962214] | 0.976099 | PEER FASTER |  |
| M7 | defaults | simd | unstrip-bound | 0.960938 | [0.944945, 0.977202] | 0.997824 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | meshlet-bound | 0.966777 | [0.942238, 0.991956] | 0.997049 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | remap | 0.828585 | [0.804737, 0.853140] | 0.967350 | PEER FASTER |  |
| M7 | defaults | simd | remap-multi | 0.885909 | [0.864244, 0.908118] | 1.050974 | PEER FASTER |  |
| M7 | defaults | simd | remap-custom | 0.881531 | [0.868139, 0.895130] | 1.043345 | PEER FASTER |  |
| M7 | defaults | simd | remap-vertices | 0.782906 | [0.762253, 0.804119] | 0.953122 | PEER FASTER |  |
| M7 | defaults | simd | remap-indices | 0.766789 | [0.752189, 0.781672] | 0.909530 | PEER FASTER |  |
| M7 | defaults | simd | shadow | 0.871472 | [0.841369, 0.902653] | 0.912574 | PEER FASTER |  |
| M7 | defaults | simd | shadow-multi | 0.906111 | [0.862762, 0.951637] | 1.031650 | PEER FASTER |  |
| M7 | defaults | simd | position-remap | 1.227978 | [1.199633, 1.256993] | 1.320309 | OURS FASTER |  |
| M7 | defaults | simd | adjacency | 0.724051 | [0.712266, 0.736032] | 0.800579 | PEER FASTER |  |
| M7 | defaults | simd | tessellation | 0.793890 | [0.776805, 0.811351] | 0.854483 | PEER FASTER |  |
| M7 | defaults | simd | provoking | 0.794670 | [0.781211, 0.808361] | 0.872330 | PEER FASTER |  |
| M7 | defaults | simd | simplify-attributes | 1.015691 | [0.998228, 1.033460] | 1.165227 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | simplify-sloppy | 0.801545 | [0.783152, 0.820370] | 0.884419 | PEER FASTER |  |
| M7 | defaults | simd | simplify-prune | 0.859486 | [0.849575, 0.869513] | 0.970995 | PEER FASTER |  |
| M7 | defaults | simd | simplify-points | 0.856174 | [0.836194, 0.876631] | 1.303447 | PEER FASTER |  |
| M7 | defaults | simd | simplify-update | 0.955253 | [0.943520, 0.967132] | 1.040350 | PEER FASTER |  |
| M7 | defaults | simd | analyze-cache | 0.841898 | [0.833788, 0.850086] | 0.871167 | PEER FASTER |  |
| M7 | defaults | simd | analyze-fetch | 1.043035 | [1.022144, 1.064353] | 1.084902 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | analyze-overdraw | 0.959908 | [0.938102, 0.982221] | 1.120875 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | analyze-coverage | 0.925596 | [0.894269, 0.958020] | 1.089771 | PEER FASTER |  |
| M7 | defaults | simd | cluster-bounds | 0.185945 | [0.164732, 0.209891] | 0.895983 | PEER FASTER |  |
| M7 | defaults | simd | sphere-bounds | 1.045911 | [1.015997, 1.076706] | 1.114930 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | meshlet-bounds | 0.022156 | [0.021269, 0.023079] | 0.600327 | PEER FASTER |  |
| M7 | defaults | simd | optimize-meshlet | 0.730619 | [0.719957, 0.741439] | 0.766778 | PEER FASTER |  |
| M7 | defaults | simd | partition | 0.647525 | [0.629227, 0.666355] | 0.759212 | PEER FASTER |  |
| M7 | defaults | simd | spatial-remap | 0.932793 | [0.922846, 0.942847] | 1.687281 | PEER FASTER |  |
| M7 | defaults | simd | spatial-triangles | 0.894800 | [0.885569, 0.904127] | 1.635857 | PEER FASTER |  |
| M7 | defaults | simd | spatial-points | 0.923926 | [0.914640, 0.933306] | 1.030778 | PEER FASTER |  |
| M7 | defaults | simd | quantize-unorm | 1.657175 | [1.359293, 2.020336] | 1.821658 | OURS FASTER |  |
| M7 | defaults | simd | quantize-snorm | 1.025325 | [0.921682, 1.140623] | 1.057811 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | quantize-half | 2.592998 | [2.303255, 2.919189] | 3.049632 | OURS FASTER |  |
| M7 | defaults | simd | dequantize-half | 3.365847 | [3.177356, 3.565520] | 3.481908 | OURS FASTER |  |
| M7 | defaults | simd | quantize-float | 2.552609 | [2.270705, 2.869510] | 3.423009 | OURS FASTER |  |
| M7 | defaults | simd | simplify-scale | 0.606214 | [0.537589, 0.683599] | 0.706812 | PEER FASTER |  |
| M7 | defaults | simd | real-simplify | 0.944337 | [0.911984, 0.977838] | 0.993649 | EQUAL WITHIN NOISE |  |
| M7 | defaults | simd | real-overdraw | 0.694644 | [0.672967, 0.717019] | 0.716493 | PEER FASTER |  |
| X | defaults | simd | vertex-decode | 1.597874 | [1.574457, 1.621640] | 1.978834 | OURS FASTER |  |
| X | defaults | simd | vertex-encode | 2.046269 | [2.024646, 2.068122] | 2.347643 | OURS FASTER |  |
| X | defaults | simd | vertex-encode-v0 | 1.445063 | [1.427479, 1.462864] | 1.539283 | OURS FASTER |  |
| X | defaults | simd | vertex-encode-default | 2.042411 | [2.019112, 2.065979] | 2.352671 | OURS FASTER |  |
| X | defaults | simd | index-decode | 1.227957 | [1.202028, 1.254447] | 1.565181 | OURS FASTER |  |
| X | defaults | simd | index-encode | 1.403019 | [1.363208, 1.443994] | 1.526854 | OURS FASTER |  |
| X | defaults | simd | sequence-decode | 0.964598 | [0.954603, 0.974697] | 1.032169 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | sequence-encode | 1.125441 | [1.104953, 1.146310] | 1.162265 | OURS FASTER |  |
| X | defaults | simd | oct-decode | 1.421694 | [1.390646, 1.453436] | 4.504782 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA 2eb57e89/2eb57e89; medium-smooth: 33800 bytes, SHA 6f30ca1f/d3cc5cbd; medium-seams: 41472 bytes, SHA d510948d/2849811d; medium-sparse: 67600 bytes, SHA fc87093c/e33f49c1; million-smooth: 4021448 bytes, SHA 4d172c17/a446e4a7 |
| X | defaults | simd | quat-decode | 1.319843 | [1.055801, 1.649917] | 4.454554 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA 27c350ac/27c350ac; medium-smooth: 33800 bytes, SHA 83cdcd1d/a01bee8b; medium-seams: 41472 bytes, SHA dee8c3bb/f0162e34; medium-sparse: 67600 bytes, SHA 5a76235d/35d9bf78; million-smooth: 4021448 bytes, SHA d8a7e4a9/179baa9d |
| X | defaults | simd | exp-decode | 2.114021 | [2.010186, 2.223219] | 14.379936 | OURS FASTER |  |
| X | defaults | simd | color-decode | 1.734747 | [1.671199, 1.800711] | 6.726986 | NOT EQUIVALENT | tiny-smooth: 200 bytes, SHA a6afb10f/a6afb10f; medium-smooth: 33800 bytes, SHA 87f29967/0959d662; medium-seams: 41472 bytes, SHA a7d7162e/2b3f2d54; medium-sparse: 67600 bytes, SHA 719b1634/46d56078; million-smooth: 4021448 bytes, SHA 6c391c91/65521c27 |
| X | defaults | simd | oct-encode | 2.118000 | [2.079566, 2.157145] | 4.384830 | OURS FASTER |  |
| X | defaults | simd | quat-encode | 2.401665 | [2.370503, 2.433237] | 4.412356 | OURS FASTER |  |
| X | defaults | simd | exp-encode | 6.276135 | [6.100691, 6.456625] | 6.957354 | OURS FASTER |  |
| X | defaults | simd | color-encode | 1.529992 | [1.448444, 1.616131] | 1.583968 | OURS FASTER |  |
| X | defaults | simd | cache | 1.026367 | [1.018089, 1.034712] | 1.192796 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | cache-strip | 1.032395 | [1.019654, 1.045295] | 1.183993 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | cache-fifo | 0.988458 | [0.975583, 1.001503] | 1.505592 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | overdraw | 0.790956 | [0.776561, 0.805617] | 0.920516 | PEER FASTER |  |
| X | defaults | simd | fetch | 0.938148 | [0.912759, 0.964243] | 1.101104 | PEER FASTER |  |
| X | defaults | simd | fetch-remap | 0.584565 | [0.575540, 0.593732] | 0.629816 | PEER FASTER |  |
| X | defaults | simd | simplify | 1.080817 | [1.062095, 1.099870] | 1.381808 | OURS FASTER |  |
| X | defaults | simd | stripify | 0.881952 | [0.859755, 0.904723] | 1.074865 | PEER FASTER |  |
| X | defaults | simd | unstripify | 1.206992 | [1.179404, 1.235226] | 1.290284 | OURS FASTER |  |
| X | defaults | simd | meshlets | 1.078460 | [1.068104, 1.088917] | 1.194840 | OURS FASTER |  |
| X | defaults | simd | meshlets-scan | 0.975818 | [0.962936, 0.988871] | 1.032888 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | meshlets-flex | 1.076537 | [1.064774, 1.088431] | 1.203105 | OURS FASTER |  |
| X | defaults | simd | meshlets-spatial | 0.864592 | [0.855268, 0.874017] | 0.911499 | PEER FASTER |  |
| X | defaults | simd | vertex-version | 0.968531 | [0.956434, 0.980781] | 0.996170 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | index-version | 0.891905 | [0.861465, 0.923420] | 0.928900 | PEER FASTER |  |
| X | defaults | simd | vertex-bound | 1.005503 | [0.986349, 1.025028] | 1.133247 | NOT EQUIVALENT | tiny-smooth: value 3189/3156; medium-smooth: value 53685/53124; medium-seams: value 66309/65616; medium-sparse: value 107337/106215; million-smooth: value 6198417/6133605 |
| X | defaults | simd | index-bound | 1.145832 | [1.130056, 1.161828] | 1.234530 | OURS FASTER |  |
| X | defaults | simd | sequence-bound | 1.185932 | [1.126175, 1.248860] | 1.284327 | OURS FASTER |  |
| X | defaults | simd | strip-bound | 0.986348 | [0.966373, 1.006735] | 1.110087 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | unstrip-bound | 0.950987 | [0.928118, 0.974420] | 1.024245 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | meshlet-bound | 1.026096 | [1.008827, 1.043661] | 1.088297 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | remap | 0.851508 | [0.830392, 0.873162] | 0.946065 | PEER FASTER |  |
| X | defaults | simd | remap-multi | 0.883542 | [0.870213, 0.897076] | 0.996610 | PEER FASTER |  |
| X | defaults | simd | remap-custom | 0.950260 | [0.933537, 0.967282] | 1.266405 | PEER FASTER |  |
| X | defaults | simd | remap-vertices | 0.935309 | [0.916544, 0.954457] | 1.042379 | PEER FASTER |  |
| X | defaults | simd | remap-indices | 0.744960 | [0.715356, 0.775790] | 0.875831 | PEER FASTER |  |
| X | defaults | simd | shadow | 0.913184 | [0.885339, 0.941905] | 1.004762 | PEER FASTER |  |
| X | defaults | simd | shadow-multi | 0.923077 | [0.867371, 0.982361] | 1.071113 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | position-remap | 1.365006 | [1.344035, 1.386305] | 1.556443 | OURS FASTER |  |
| X | defaults | simd | adjacency | 0.676811 | [0.668717, 0.685004] | 0.747804 | PEER FASTER |  |
| X | defaults | simd | tessellation | 0.775623 | [0.760237, 0.791320] | 0.828944 | PEER FASTER |  |
| X | defaults | simd | provoking | 0.877632 | [0.868044, 0.887327] | 0.954320 | PEER FASTER |  |
| X | defaults | simd | simplify-attributes | 1.082307 | [1.064581, 1.100328] | 1.237948 | OURS FASTER |  |
| X | defaults | simd | simplify-sloppy | 1.001088 | [0.990832, 1.011449] | 1.161053 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | simplify-prune | 0.854197 | [0.842402, 0.866157] | 0.938223 | PEER FASTER |  |
| X | defaults | simd | simplify-points | 1.248244 | [1.209738, 1.287974] | 1.486020 | OURS FASTER |  |
| X | defaults | simd | simplify-update | 1.009851 | [0.997499, 1.022356] | 1.135172 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | analyze-cache | 1.009738 | [1.002395, 1.017135] | 1.030709 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | analyze-fetch | 1.220020 | [1.207662, 1.232504] | 1.275139 | OURS FASTER |  |
| X | defaults | simd | analyze-overdraw | 1.877333 | [1.824932, 1.931239] | 2.935235 | OURS FASTER |  |
| X | defaults | simd | analyze-coverage | 1.829534 | [1.801377, 1.858132] | 2.817075 | OURS FASTER |  |
| X | defaults | simd | cluster-bounds | 0.192112 | [0.187407, 0.196935] | 1.006188 | PEER FASTER |  |
| X | defaults | simd | sphere-bounds | 1.272343 | [1.254309, 1.290636] | 1.372828 | OURS FASTER |  |
| X | defaults | simd | meshlet-bounds | 0.031539 | [0.030969, 0.032120] | 0.790409 | PEER FASTER |  |
| X | defaults | simd | optimize-meshlet | 0.757690 | [0.741336, 0.774404] | 0.838474 | PEER FASTER |  |
| X | defaults | simd | partition | 0.830370 | [0.822022, 0.838802] | 0.934124 | PEER FASTER |  |
| X | defaults | simd | spatial-remap | 1.014568 | [0.999504, 1.029860] | 1.513554 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | spatial-triangles | 0.972187 | [0.956839, 0.987781] | 1.552344 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | spatial-points | 0.903041 | [0.894398, 0.911767] | 0.970794 | PEER FASTER |  |
| X | defaults | simd | quantize-unorm | 1.615266 | [1.321018, 1.975056] | 1.681515 | OURS FASTER |  |
| X | defaults | simd | quantize-snorm | 1.022471 | [0.919797, 1.136606] | 1.056193 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | quantize-half | 0.662900 | [0.577662, 0.760715] | 0.753604 | PEER FASTER |  |
| X | defaults | simd | dequantize-half | 0.955640 | [0.925016, 0.987278] | 0.969409 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | quantize-float | 0.641381 | [0.546345, 0.752948] | 0.972629 | PEER FASTER |  |
| X | defaults | simd | simplify-scale | 0.697177 | [0.631509, 0.769673] | 0.851246 | PEER FASTER |  |
| X | defaults | simd | real-simplify | 1.013685 | [0.982462, 1.045900] | 1.076305 | EQUAL WITHIN NOISE |  |
| X | defaults | simd | real-overdraw | 0.695117 | [0.674238, 0.716642] | 0.718089 | PEER FASTER |  |
| M7 | moss | scalar | vertex-decode | 0.893941 | [0.851584, 0.938405] | 0.984065 | PEER FASTER |  |
| M7 | moss | scalar | vertex-encode | 1.645183 | [1.597230, 1.694577] | 1.733134 | OURS FASTER |  |
| M7 | moss | scalar | vertex-encode-v0 | 1.139605 | [1.109978, 1.170024] | 1.210497 | OURS FASTER |  |
| M7 | moss | scalar | vertex-encode-default | 1.666873 | [1.632895, 1.701559] | 1.735098 | OURS FASTER |  |
| M7 | moss | scalar | index-decode | 1.007276 | [0.965452, 1.050913] | 1.247501 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | index-encode | 1.181922 | [1.099440, 1.270593] | 1.549318 | OURS FASTER |  |
| M7 | moss | scalar | sequence-decode | 1.062125 | [1.049576, 1.074825] | 1.180173 | OURS FASTER |  |
| M7 | moss | scalar | sequence-encode | 1.030706 | [1.024341, 1.037110] | 1.038655 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | oct-decode | 0.225063 | [0.223243, 0.226899] | 0.269726 | PEER FASTER |  |
| M7 | moss | scalar | quat-decode | 0.285732 | [0.282511, 0.288990] | 0.312073 | PEER FASTER |  |
| M7 | moss | scalar | exp-decode | 1.032744 | [1.006636, 1.059529] | 1.102984 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | color-decode | 0.975443 | [0.969417, 0.981507] | 1.029783 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | oct-encode | 1.141698 | [1.121201, 1.162570] | 1.232584 | OURS FASTER |  |
| M7 | moss | scalar | quat-encode | 1.134918 | [1.112369, 1.157924] | 1.253030 | OURS FASTER |  |
| M7 | moss | scalar | exp-encode | 5.791149 | [5.549818, 6.042975] | 6.755453 | OURS FASTER |  |
| M7 | moss | scalar | color-encode | 1.233470 | [1.203903, 1.263764] | 1.282855 | OURS FASTER |  |
| M7 | moss | scalar | cache | 0.880058 | [0.868062, 0.892219] | 1.128935 | PEER FASTER |  |
| M7 | moss | scalar | cache-strip | 0.886963 | [0.875445, 0.898632] | 1.171371 | PEER FASTER |  |
| M7 | moss | scalar | cache-fifo | 0.792639 | [0.783030, 0.802366] | 1.263806 | PEER FASTER |  |
| M7 | moss | scalar | overdraw | 0.760196 | [0.755649, 0.764770] | 1.001534 | PEER FASTER |  |
| M7 | moss | scalar | fetch | 0.870328 | [0.850670, 0.890440] | 1.044513 | PEER FASTER |  |
| M7 | moss | scalar | fetch-remap | 0.612635 | [0.607730, 0.617580] | 0.654217 | PEER FASTER |  |
| M7 | moss | scalar | simplify | 1.063900 | [1.051131, 1.076824] | 1.291399 | OURS FASTER |  |
| M7 | moss | scalar | stripify | 0.985879 | [0.973522, 0.998393] | 1.142143 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | unstripify | 1.027003 | [1.001966, 1.052665] | 1.110772 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | meshlets | 0.914421 | [0.897302, 0.931866] | 0.993538 | PEER FASTER |  |
| M7 | moss | scalar | meshlets-scan | 0.752711 | [0.736119, 0.769677] | 0.803905 | PEER FASTER |  |
| M7 | moss | scalar | meshlets-flex | 0.923477 | [0.906295, 0.940985] | 1.003759 | PEER FASTER |  |
| M7 | moss | scalar | meshlets-spatial | 0.897294 | [0.873432, 0.921808] | 0.922285 | PEER FASTER |  |
| M7 | moss | scalar | vertex-version | 0.989085 | [0.975843, 1.002507] | 1.031284 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | index-version | 0.979101 | [0.957960, 1.000708] | 1.007043 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | vertex-bound | 1.038602 | [1.020412, 1.057116] | 1.070202 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | index-bound | 1.034794 | [1.012465, 1.057614] | 1.046039 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | sequence-bound | 1.092658 | [1.066535, 1.119421] | 1.144241 | OURS FASTER |  |
| M7 | moss | scalar | strip-bound | 1.080078 | [1.051010, 1.109949] | 1.123259 | OURS FASTER |  |
| M7 | moss | scalar | unstrip-bound | 1.083220 | [1.048497, 1.119092] | 1.164750 | OURS FASTER |  |
| M7 | moss | scalar | meshlet-bound | 1.042861 | [1.024520, 1.061530] | 1.097241 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | remap | 0.884948 | [0.875022, 0.894987] | 0.944743 | PEER FASTER |  |
| M7 | moss | scalar | remap-multi | 0.956185 | [0.939764, 0.972893] | 1.021294 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | remap-custom | 0.879748 | [0.842544, 0.918595] | 1.107346 | PEER FASTER |  |
| M7 | moss | scalar | remap-vertices | 0.828574 | [0.787943, 0.871301] | 0.924273 | PEER FASTER |  |
| M7 | moss | scalar | remap-indices | 0.704840 | [0.697241, 0.712522] | 0.807714 | PEER FASTER |  |
| M7 | moss | scalar | shadow | 0.918003 | [0.906744, 0.929401] | 0.961154 | PEER FASTER |  |
| M7 | moss | scalar | shadow-multi | 0.992670 | [0.978218, 1.007337] | 1.054698 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | position-remap | 1.228071 | [1.201042, 1.255707] | 1.288021 | OURS FASTER |  |
| M7 | moss | scalar | adjacency | 0.797707 | [0.769381, 0.827076] | 0.856509 | PEER FASTER |  |
| M7 | moss | scalar | tessellation | 0.877229 | [0.860185, 0.894610] | 0.918160 | PEER FASTER |  |
| M7 | moss | scalar | provoking | 0.853371 | [0.834617, 0.872546] | 0.936419 | PEER FASTER |  |
| M7 | moss | scalar | simplify-attributes | 1.103075 | [1.089588, 1.116728] | 1.238870 | OURS FASTER |  |
| M7 | moss | scalar | simplify-sloppy | 0.766319 | [0.718140, 0.817730] | 0.824215 | PEER FASTER |  |
| M7 | moss | scalar | simplify-prune | 0.959534 | [0.926118, 0.994155] | 1.082495 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | simplify-points | 0.819818 | [0.780833, 0.860750] | 1.351411 | PEER FASTER |  |
| M7 | moss | scalar | simplify-update | 1.019088 | [0.998511, 1.040089] | 1.182615 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | analyze-cache | 0.923017 | [0.904867, 0.941532] | 0.968211 | PEER FASTER |  |
| M7 | moss | scalar | analyze-fetch | 1.049812 | [1.039697, 1.060025] | 1.091879 | OURS FASTER |  |
| M7 | moss | scalar | analyze-overdraw | 1.022873 | [1.006364, 1.039654] | 1.105211 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | analyze-coverage | 0.980926 | [0.961704, 1.000532] | 1.118217 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | cluster-bounds | 0.177588 | [0.117740, 0.267857] | 0.898028 | PEER FASTER |  |
| M7 | moss | scalar | sphere-bounds | 1.033924 | [1.007551, 1.060987] | 1.099530 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | meshlet-bounds | 0.024904 | [0.023816, 0.026043] | 0.613093 | PEER FASTER |  |
| M7 | moss | scalar | optimize-meshlet | 0.729544 | [0.681107, 0.781426] | 0.778081 | PEER FASTER |  |
| M7 | moss | scalar | partition | 0.701226 | [0.686366, 0.716409] | 0.776185 | PEER FASTER |  |
| M7 | moss | scalar | spatial-remap | 0.947614 | [0.925064, 0.970715] | 1.692728 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | spatial-triangles | 0.874975 | [0.854397, 0.896048] | 1.586821 | PEER FASTER |  |
| M7 | moss | scalar | spatial-points | 0.901950 | [0.884746, 0.919489] | 0.974226 | PEER FASTER |  |
| M7 | moss | scalar | quantize-unorm | 1.540870 | [1.296487, 1.831319] | 1.696593 | OURS FASTER |  |
| M7 | moss | scalar | quantize-snorm | 0.999735 | [0.911657, 1.096321] | 1.035731 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | quantize-half | 2.174295 | [1.953730, 2.419761] | 2.674699 | OURS FASTER |  |
| M7 | moss | scalar | dequantize-half | 3.677189 | [3.508511, 3.853977] | 3.838552 | OURS FASTER |  |
| M7 | moss | scalar | quantize-float | 2.790141 | [2.702626, 2.880491] | 4.178689 | OURS FASTER |  |
| M7 | moss | scalar | simplify-scale | 0.756745 | [0.690630, 0.829189] | 0.809508 | PEER FASTER |  |
| M7 | moss | scalar | real-simplify | 0.971991 | [0.934944, 1.010506] | 1.053525 | EQUAL WITHIN NOISE |  |
| M7 | moss | scalar | real-overdraw | 0.686840 | [0.663116, 0.711413] | 0.712689 | PEER FASTER |  |
| X | moss | scalar | vertex-decode | 1.370686 | [1.352898, 1.388708] | 1.694950 | OURS FASTER |  |
| X | moss | scalar | vertex-encode | 2.017635 | [1.996409, 2.039086] | 2.287867 | OURS FASTER |  |
| X | moss | scalar | vertex-encode-v0 | 1.395798 | [1.367589, 1.424588] | 1.483458 | OURS FASTER |  |
| X | moss | scalar | vertex-encode-default | 2.022204 | [1.987452, 2.057563] | 2.268361 | OURS FASTER |  |
| X | moss | scalar | index-decode | 1.109705 | [1.098002, 1.121534] | 1.251314 | OURS FASTER |  |
| X | moss | scalar | index-encode | 1.292315 | [1.262394, 1.322945] | 1.414472 | OURS FASTER |  |
| X | moss | scalar | sequence-decode | 0.965117 | [0.943681, 0.987041] | 1.015636 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | sequence-encode | 1.138355 | [1.112056, 1.165276] | 1.170002 | OURS FASTER |  |
| X | moss | scalar | oct-decode | 0.378383 | [0.371271, 0.385631] | 0.729576 | PEER FASTER |  |
| X | moss | scalar | quat-decode | 0.462379 | [0.449363, 0.475771] | 0.863212 | PEER FASTER |  |
| X | moss | scalar | exp-decode | 2.191996 | [2.067715, 2.323747] | 14.246066 | OURS FASTER |  |
| X | moss | scalar | color-decode | 2.380714 | [2.317293, 2.445871] | 5.276276 | OURS FASTER |  |
| X | moss | scalar | oct-encode | 2.072129 | [2.048298, 2.096238] | 4.489633 | OURS FASTER |  |
| X | moss | scalar | quat-encode | 2.399252 | [2.365809, 2.433168] | 4.670703 | OURS FASTER |  |
| X | moss | scalar | exp-encode | 6.371333 | [6.284341, 6.459530] | 7.198672 | OURS FASTER |  |
| X | moss | scalar | color-encode | 1.604929 | [1.569779, 1.640865] | 1.712527 | OURS FASTER |  |
| X | moss | scalar | cache | 1.161107 | [1.144492, 1.177963] | 1.310010 | OURS FASTER |  |
| X | moss | scalar | cache-strip | 1.161656 | [1.139411, 1.184335] | 1.376622 | OURS FASTER |  |
| X | moss | scalar | cache-fifo | 0.984186 | [0.964860, 1.003899] | 1.411208 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | overdraw | 0.683833 | [0.528448, 0.884906] | 0.779344 | PEER FASTER |  |
| X | moss | scalar | fetch | 0.804850 | [0.786764, 0.823351] | 0.869354 | PEER FASTER |  |
| X | moss | scalar | fetch-remap | 0.351885 | [0.342263, 0.361776] | 0.440533 | PEER FASTER |  |
| X | moss | scalar | simplify | 1.110633 | [1.063601, 1.159743] | 1.356220 | OURS FASTER |  |
| X | moss | scalar | stripify | 0.913215 | [0.894971, 0.931831] | 1.015886 | PEER FASTER |  |
| X | moss | scalar | unstripify | 1.305299 | [1.267146, 1.344600] | 1.430229 | OURS FASTER |  |
| X | moss | scalar | meshlets | 1.108331 | [1.086726, 1.130364] | 1.240695 | OURS FASTER |  |
| X | moss | scalar | meshlets-scan | 1.203639 | [1.138654, 1.272334] | 1.431563 | OURS FASTER |  |
| X | moss | scalar | meshlets-flex | 1.101840 | [1.087374, 1.116499] | 1.250926 | OURS FASTER |  |
| X | moss | scalar | meshlets-spatial | 0.833141 | [0.663637, 1.045940] | 0.954835 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | vertex-version | 0.948259 | [0.935727, 0.960958] | 0.999545 | PEER FASTER |  |
| X | moss | scalar | index-version | 0.953871 | [0.929434, 0.978950] | 1.005165 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | vertex-bound | 1.069123 | [1.039635, 1.099446] | 1.280832 | NOT EQUIVALENT | tiny-smooth: value 3189/3156; medium-smooth: value 53685/53124; medium-seams: value 66309/65616; medium-sparse: value 107337/106215; million-smooth: value 6198417/6133605 |
| X | moss | scalar | index-bound | 1.144316 | [1.115801, 1.173561] | 1.292820 | OURS FASTER |  |
| X | moss | scalar | sequence-bound | 1.132841 | [1.109308, 1.156874] | 1.283687 | OURS FASTER |  |
| X | moss | scalar | strip-bound | 1.246680 | [1.217229, 1.276843] | 2.579418 | OURS FASTER |  |
| X | moss | scalar | unstrip-bound | 1.042333 | [1.011908, 1.073672] | 1.097938 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | meshlet-bound | 1.068591 | [1.047243, 1.090374] | 1.082593 | OURS FASTER |  |
| X | moss | scalar | remap | 0.949436 | [0.924956, 0.974565] | 1.067680 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | remap-multi | 0.960569 | [0.930919, 0.991164] | 1.101020 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | remap-custom | 0.952965 | [0.933672, 0.972658] | 1.225364 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | remap-vertices | 1.070431 | [1.039366, 1.102425] | 1.191158 | OURS FASTER |  |
| X | moss | scalar | remap-indices | 0.813874 | [0.803231, 0.824658] | 0.894708 | PEER FASTER |  |
| X | moss | scalar | shadow | 1.102656 | [0.855151, 1.421795] | 1.624058 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | shadow-multi | 0.967017 | [0.949266, 0.985100] | 1.100648 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | position-remap | 1.370573 | [1.335929, 1.406116] | 1.562045 | OURS FASTER |  |
| X | moss | scalar | adjacency | 0.763524 | [0.744318, 0.783225] | 0.825639 | PEER FASTER |  |
| X | moss | scalar | tessellation | 0.856596 | [0.832858, 0.881012] | 0.943301 | PEER FASTER |  |
| X | moss | scalar | provoking | 0.980573 | [0.943997, 1.018567] | 1.060544 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | simplify-attributes | 1.122814 | [1.093923, 1.152468] | 1.279043 | OURS FASTER |  |
| X | moss | scalar | simplify-sloppy | 0.970346 | [0.957069, 0.983806] | 1.141944 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | simplify-prune | 0.944824 | [0.933444, 0.956343] | 0.961350 | PEER FASTER |  |
| X | moss | scalar | simplify-points | 1.168815 | [1.125682, 1.213602] | 1.420156 | OURS FASTER |  |
| X | moss | scalar | simplify-update | 1.023579 | [1.010760, 1.036561] | 1.224297 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | analyze-cache | 1.095836 | [1.085752, 1.106013] | 1.107919 | OURS FASTER |  |
| X | moss | scalar | analyze-fetch | 1.231002 | [1.214905, 1.247313] | 1.289111 | OURS FASTER |  |
| X | moss | scalar | analyze-overdraw | 2.031145 | [2.002166, 2.060543] | 3.376093 | OURS FASTER |  |
| X | moss | scalar | analyze-coverage | 2.039799 | [2.014964, 2.064939] | 3.427904 | OURS FASTER |  |
| X | moss | scalar | cluster-bounds | 0.221720 | [0.216885, 0.226663] | 0.995117 | PEER FASTER |  |
| X | moss | scalar | sphere-bounds | 1.244320 | [1.204806, 1.285129] | 1.353836 | OURS FASTER |  |
| X | moss | scalar | meshlet-bounds | 0.036419 | [0.035044, 0.037847] | 0.799949 | PEER FASTER |  |
| X | moss | scalar | optimize-meshlet | 0.782483 | [0.766949, 0.798331] | 0.813512 | PEER FASTER |  |
| X | moss | scalar | partition | 0.939434 | [0.925553, 0.953524] | 1.057586 | PEER FASTER |  |
| X | moss | scalar | spatial-remap | 1.039971 | [1.025313, 1.054838] | 1.667787 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | spatial-triangles | 0.994920 | [0.971930, 1.018453] | 1.551256 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | spatial-points | 0.985526 | [0.975833, 0.995315] | 1.045042 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | quantize-unorm | 1.654259 | [1.382395, 1.979588] | 1.820415 | OURS FASTER |  |
| X | moss | scalar | quantize-snorm | 1.027904 | [0.926827, 1.140004] | 1.039968 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | quantize-half | 0.675376 | [0.604040, 0.755136] | 0.834567 | PEER FASTER |  |
| X | moss | scalar | dequantize-half | 1.015601 | [0.991904, 1.039863] | 1.061639 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | quantize-float | 0.716105 | [0.685221, 0.748381] | 0.958341 | PEER FASTER |  |
| X | moss | scalar | simplify-scale | 0.913920 | [0.893859, 0.934430] | 0.930947 | PEER FASTER |  |
| X | moss | scalar | real-simplify | 0.981669 | [0.950809, 1.013530] | 1.045429 | EQUAL WITHIN NOISE |  |
| X | moss | scalar | real-overdraw | 0.719918 | [0.661582, 0.783398] | 0.751523 | PEER FASTER |  |

## Unavailable and unmeasured

- M7 / all / all / cluster-LOD: UNAVAILABLE — UNAVAILABLE by registered scope: spec excludes M7 cluster LOD; its published build does contain clusterlod bindings, which are not tested here.
- X / all / all / cluster-LOD: UNAVAILABLE — published optimesh 1.1.0 archive exposes no public clusterlod module; README describes repository demo, absent from the published archive.
- M7 / moss / simd / vertex-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / vertex-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / vertex-encode-v0: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / vertex-encode-default: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / index-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / index-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / sequence-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / sequence-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / oct-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quat-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / exp-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / color-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / oct-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quat-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / exp-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / color-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / cache: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / cache-strip: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / cache-fifo: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / fetch: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / fetch-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / stripify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / unstripify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlets: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlets-scan: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlets-flex: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlets-spatial: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / vertex-version: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / index-version: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / vertex-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / index-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / sequence-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / strip-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / unstrip-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlet-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / remap-multi: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / remap-custom: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / remap-vertices: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / remap-indices: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / shadow: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / shadow-multi: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / position-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / adjacency: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / tessellation: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / provoking: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-attributes: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-sloppy: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-prune: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-points: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-update: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / analyze-cache: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / analyze-fetch: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / analyze-overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / analyze-coverage: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / cluster-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / sphere-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / meshlet-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / optimize-meshlet: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / partition: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / spatial-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / spatial-triangles: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / spatial-points: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quantize-unorm: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quantize-snorm: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quantize-half: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / dequantize-half: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / quantize-float: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / simplify-scale: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / real-simplify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- M7 / moss / simd / real-overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-encode-v0: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-encode-default: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / index-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / index-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / sequence-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / sequence-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / oct-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quat-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / exp-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / color-decode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / oct-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quat-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / exp-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / color-encode: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / cache: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / cache-strip: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / cache-fifo: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / fetch: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / fetch-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / stripify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / unstripify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlets: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlets-scan: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlets-flex: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlets-spatial: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-version: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / index-version: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / vertex-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / index-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / sequence-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / strip-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / unstrip-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlet-bound: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / remap-multi: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / remap-custom: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / remap-vertices: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / remap-indices: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / shadow: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / shadow-multi: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / position-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / adjacency: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / tessellation: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / provoking: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-attributes: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-sloppy: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-prune: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-points: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-update: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / analyze-cache: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / analyze-fetch: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / analyze-overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / analyze-coverage: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / cluster-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / sphere-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / meshlet-bounds: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / optimize-meshlet: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / partition: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / spatial-remap: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / spatial-triangles: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / spatial-points: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quantize-unorm: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quantize-snorm: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quantize-half: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / dequantize-half: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / quantize-float: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / simplify-scale: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / real-simplify: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / simd / real-overdraw: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / defaults / wasm-simd128 / browser-attributes: UNMEASURED — CDP Execution context was destroyed before byte checks on both permitted attempts; family retry allowance exhausted; no third attempt or timing.
- X / defaults / wasm-simd128 / browser-triangles: UNMEASURED — CDP Execution context was destroyed before byte checks on both permitted attempts; family retry allowance exhausted; no third attempt or timing.
- X / moss / wasm-simd128 / browser-attributes: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.
- X / moss / wasm-simd128 / browser-triangles: UNMEASURED — external lane hold until 2026-10-26 00:57:19 CET for live-session ng-wave2 build and iteration-speed final batch; terminal campaign stop while queued, no pair interrupted; admitted-time cap not exhausted.

## Owner review

[]

## Receipts and limits

`REGISTRATION.md`, `SAFETY.md`, `safety-reuse.json`, per-arm cargo trees, build/admission logs, retained native/Wasm binaries, `epoch.json`, `inputs.json`, `real-inputs.json`, `correctness.json`, `timing-*.json`, browser stream/reference hashes and `browser-progress.jsonl` are in the campaign directory. The old peer-screen and meshopt-cmp evidence is untouched.

Real operation replays retain the full captured stride-64 vertex records and the exact first simplify/overdraw record parameters. The packed XYZ prefix is used only by independent validators. Each operation has a fresh resident workspace; this is not a cooker-chain replay. Browser reuses REPORT-3 inputs/transports and headless runner, with registered pairs and fresh scalar C++ reference bytes. O/X decode to allocating output, then copy to caller storage; upstream JS retains its shipped source-copy transport.

Not verified: production/Moss acceptance, cluster-LOD performance, other hosts/architectures/browsers, geometric quality beyond recorded validity/error/count, full library qualification, or unmeasured rows. No library src/, README or Moss change.

## Infrastructure and retained observations

Terminal external hold: at 2026-10-25T19:58:45.405659+00:00 the lane was held until 2026-10-26 00:57:19 CET for the live ng-wave2 session and iteration-speed final batch. The queued campaign was stopped at a case boundary, with no active pair. Six native combinations are complete; Moss SIMD M7 retains 19 partial-case rows and Moss SIMD X and both Moss browser families were not collected. Partial cases remain in raw receipts and do not form a complete family GM. The admitted-time cap was not exhausted. No resumed or canceled queue wait is timing evidence.

Default-profile browser decode is unmeasured: CDP Execution context was destroyed before byte checks on both permitted attempts; family retry allowance exhausted; no third attempt or timing. Logs: `browser-defaults/correctness-attempt-0.log`, `correctness-attempt-1.log`. Moss browser is unmeasured under the external hold. Its prepared navigation fix was never exercised in this campaign.

A default scalar M7 medium-sparse vertex-encode-default pair retained a peer/O ratio of 13937.279530240616. The initial registered interval straddled equality, so the case extended once to twenty pairs; its final GM is 2.588237962555155, CI [1.0041501053820887, 6.671289197607199]. Cause is unverified. No sample was dropped or repeated to improve a verdict. All train pauses and admission waits are retained.

`SAFETY.md` existed before the first build (filesystem birth time retained in `audit.json`); later append-only confirmations explain its newer modification time. `REGISTRATION.md` and its amendment predate every sample. Source-epoch history retains all pre-sample replay-layout corrections and the later resource-accounting-only correction. The lane cap includes admitted builds/preflights and measurements, excluding queue waits.

## Upstream JS SIMD reference

| Profile | Family | JS/O GM | 95% interval | Max case |
|---|---|---:|---|---:|

Unmeasured in this campaign: default-profile byte checks failed twice, and Moss browser collection was blocked by the external hold. Reference transport is retained from REPORT-3; it is not a verdict about X.

## Rerun — 2026-10-25

Receipts: `/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150615Z/rerun-20261025T200837Z`. Original completed rows and all parent timing files are unchanged; the 19 partial M7 cases are retained verbatim. Only the 146 previously unmeasured Moss SIMD families and the browser extension were eligible. Unchanged registration, frozen inputs, source epoch, retained executables, CPU pinning and paired protocol. Owner Etienne removed wait 2 (lane C and validate-once) on 2026-10-25. Only the external ops hold remained; wrapper admission enforced it. Initial blocking polls, the owner amendment and admission receipts are retained.

New same-session A/A floor **F = 0.010147978433 (1.014798%)**, applied only to rerun verdicts. Peer upper < 0.969852021567; ours lower > 1.030147978433. Original session verdicts retain their original floor.

**Updated registered optimesh beats ours anywhere: NO.** The wasm decode answer follows the rows below. Cluster LOD remains unavailable; non-equivalent quality conditions are applied literally.

| A/A family | GM | 95% interval | Max | Load1 median [min,max] |
|---|---:|---|---:|---|
| simplify | 0.989852 | [0.977594, 1.002264] | 1.006975 | 9.19 [8.86, 10.36] |
| meshlets | 0.999555 | [0.990618, 1.008573] | 1.010840 | 9.20 [8.86, 10.01] |
| meshlets-spatial | 0.995098 | [0.987654, 1.002597] | 1.013493 | 9.19 [8.86, 9.63] |

| Peer/profile/mode | Verdict counts |
|---|---|
| M7/moss/simd | EQUAL WITHIN NOISE: 12, NOT EQUIVALENT: 3, OURS FASTER: 24, PEER FASTER: 34 |
| X/moss/simd | EQUAL WITHIN NOISE: 13, NOT EQUIVALENT: 4, OURS FASTER: 33, PEER FASTER: 23 |
| X/defaults/wasm-simd128 | OURS FASTER: 2 |
| JS/defaults/wasm-simd128 | PEER FASTER: 2 |
| X/moss/wasm-simd128 | EQUAL WITHIN NOISE: 1, OURS FASTER: 1 |
| JS/moss/wasm-simd128 | EQUAL WITHIN NOISE: 2 |

| Peer | Profile | Mode | Family | Correctness | Output metrics | GM | 95% interval | Max | Verdict | Load1 median [min,max] |
|---|---|---|---|---|---|---:|---|---:|---|---|
| M7 | moss | simd | vertex-decode | identical | — | 0.913273 | [0.893751, 0.933221] | 1.417063 | PEER FASTER | 6.27 [5.99, 16.25] |
| M7 | moss | simd | vertex-encode | identical | — | 1.639487 | [1.609682, 1.669843] | 1.769626 | OURS FASTER | 6.27 [5.99, 16.25] |
| M7 | moss | simd | vertex-encode-v0 | identical | — | 1.143323 | [1.129698, 1.157113] | 1.185469 | OURS FASTER | 6.27 [5.99, 16.25] |
| M7 | moss | simd | vertex-encode-default | identical | — | 1.642792 | [1.579075, 1.709081] | 1.764123 | OURS FASTER | 6.27 [5.99, 16.25] |
| M7 | moss | simd | index-decode | identical | — | 0.897244 | [0.864142, 0.931614] | 0.961303 | PEER FASTER | 6.80 [5.99, 16.07] |
| M7 | moss | simd | index-encode | identical | — | 1.140529 | [1.110159, 1.171731] | 1.291691 | OURS FASTER | 11.43 [5.99, 16.07] |
| M7 | moss | simd | sequence-decode | identical | — | 1.055433 | [1.027722, 1.083891] | 1.199558 | EQUAL WITHIN NOISE | 6.27 [5.99, 16.07] |
| M7 | moss | simd | sequence-encode | identical | — | 1.053109 | [1.043363, 1.062946] | 1.064154 | OURS FASTER | 6.81 [5.99, 16.07] |
| M7 | moss | simd | oct-decode | different but valid | [{"key":["moss","simd","tiny-smooth","oct-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","tiny-smooth","oct-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","medium-smooth","oct-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"d3cc5cbdf1a0"},{"key":["moss","simd","medium-smooth","oct-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"6f30ca1fba1c"},{"key":["moss","simd","medium-seams","oct-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2849811d9079"},{"key":["moss","simd","medium-seams","oct-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"d510948da1ae"},{"key":["moss","simd","medium-sparse","oct-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"e33f49c1bef7"},{"key":["moss","simd","medium-sparse","oct-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"fc87093c3600"},{"key":["moss","simd","million-smooth","oct-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"a446e4a744a5"},{"key":["moss","simd","million-smooth","oct-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"4d172c17508c"}] | 0.918228 | [0.901442, 0.935326] | 1.058824 | NOT EQUIVALENT | 11.44 [5.99, 16.07] |
| M7 | moss | simd | quat-decode | different but valid | [{"key":["moss","simd","tiny-smooth","quat-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","tiny-smooth","quat-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","medium-smooth","quat-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"a01bee8bc173"},{"key":["moss","simd","medium-smooth","quat-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"83cdcd1de1ff"},{"key":["moss","simd","medium-seams","quat-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"f0162e347409"},{"key":["moss","simd","medium-seams","quat-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"dee8c3bb2c59"},{"key":["moss","simd","medium-sparse","quat-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"35d9bf78dcb6"},{"key":["moss","simd","medium-sparse","quat-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"5a76235dadd8"},{"key":["moss","simd","million-smooth","quat-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"179baa9dec7d"},{"key":["moss","simd","million-smooth","quat-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"d8a7e4a95b31"}] | 0.773265 | [0.763011, 0.783657] | 0.834996 | NOT EQUIVALENT | 6.27 [5.99, 16.07] |
| M7 | moss | simd | exp-decode | identical | — | 1.038012 | [1.002753, 1.074511] | 1.086425 | EQUAL WITHIN NOISE | 6.81 [5.99, 16.07] |
| M7 | moss | simd | color-decode | different but valid | [{"key":["moss","simd","tiny-smooth","color-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","tiny-smooth","color-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","medium-smooth","color-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"0959d662b556"},{"key":["moss","simd","medium-smooth","color-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"87f29967705f"},{"key":["moss","simd","medium-seams","color-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2b3f2d54e513"},{"key":["moss","simd","medium-seams","color-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"a7d7162e42e3"},{"key":["moss","simd","medium-sparse","color-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"46d56078ce38"},{"key":["moss","simd","medium-sparse","color-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"719b1634a0eb"},{"key":["moss","simd","million-smooth","color-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"65521c277333"},{"key":["moss","simd","million-smooth","color-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"6c391c916089"}] | 1.063176 | [1.050564, 1.075940] | 1.106428 | NOT EQUIVALENT | 11.44 [5.99, 16.07] |
| M7 | moss | simd | oct-encode | identical | — | 1.185803 | [1.142065, 1.231217] | 1.326470 | OURS FASTER | 11.44 [5.99, 16.07] |
| M7 | moss | simd | quat-encode | identical | — | 1.148743 | [1.131726, 1.166015] | 1.242578 | OURS FASTER | 6.25 [5.99, 16.07] |
| M7 | moss | simd | exp-encode | identical | — | 6.011890 | [4.743042, 7.620178] | 6.861335 | OURS FASTER | 6.25 [5.99, 16.07] |
| M7 | moss | simd | color-encode | identical | — | 1.223123 | [1.181962, 1.265717] | 1.267104 | OURS FASTER | 6.79 [6.08, 16.07] |
| M7 | moss | simd | cache | identical | — | 0.879423 | [0.861284, 0.897944] | 1.097266 | PEER FASTER | 6.59 [6.08, 16.07] |
| M7 | moss | simd | cache-strip | identical | — | 0.901843 | [0.883607, 0.920456] | 1.160821 | PEER FASTER | 11.43 [6.08, 16.07] |
| M7 | moss | simd | cache-fifo | identical | — | 0.801372 | [0.780403, 0.822904] | 1.279283 | PEER FASTER | 11.43 [6.08, 16.07] |
| M7 | moss | simd | overdraw | identical | — | 0.755873 | [0.748451, 0.763370] | 0.992209 | PEER FASTER | 6.08 [5.92, 6.79] |
| M7 | moss | simd | fetch | identical | — | 0.791991 | [0.769768, 0.814856] | 0.978623 | PEER FASTER | 6.08 [5.92, 6.79] |
| M7 | moss | simd | fetch-remap | identical | — | 0.816613 | [0.801317, 0.832200] | 0.886332 | PEER FASTER | 6.55 [5.92, 6.79] |
| M7 | moss | simd | simplify | identical | — | 1.048066 | [1.031154, 1.065256] | 1.270768 | OURS FASTER | 6.46 [5.92, 6.79] |
| M7 | moss | simd | stripify | identical | — | 0.992899 | [0.976441, 1.009634] | 1.121930 | EQUAL WITHIN NOISE | 6.51 [5.92, 6.79] |
| M7 | moss | simd | unstripify | identical | — | 1.078974 | [1.060477, 1.097793] | 1.121017 | OURS FASTER | 6.51 [5.92, 6.79] |
| M7 | moss | simd | meshlets | identical | — | 0.913350 | [0.904048, 0.922748] | 0.998386 | PEER FASTER | 6.55 [5.92, 6.79] |
| M7 | moss | simd | meshlets-scan | identical | — | 0.765371 | [0.745613, 0.785652] | 0.816605 | PEER FASTER | 6.47 [5.92, 6.79] |
| M7 | moss | simd | meshlets-flex | identical | — | 0.913651 | [0.905715, 0.921656] | 0.995818 | PEER FASTER | 6.59 [5.92, 6.88] |
| M7 | moss | simd | meshlets-spatial | identical | — | 0.807688 | [0.799952, 0.815498] | 0.880782 | PEER FASTER | 6.55 [5.92, 6.89] |
| M7 | moss | simd | vertex-version | identical | — | 0.954611 | [0.937725, 0.971801] | 0.982689 | EQUAL WITHIN NOISE | 6.55 [5.92, 6.74] |
| M7 | moss | simd | index-version | identical | — | 1.016533 | [0.998274, 1.035126] | 1.053729 | EQUAL WITHIN NOISE | 6.55 [5.92, 6.74] |
| M7 | moss | simd | vertex-bound | identical | — | 1.086735 | [1.067494, 1.106323] | 1.117814 | OURS FASTER | 5.99 [5.92, 6.74] |
| M7 | moss | simd | index-bound | identical | — | 1.113795 | [1.091416, 1.136633] | 1.167120 | OURS FASTER | 6.69 [5.92, 6.74] |
| M7 | moss | simd | sequence-bound | identical | — | 1.100379 | [1.072247, 1.129249] | 1.151297 | OURS FASTER | 5.95 [5.92, 6.74] |
| M7 | moss | simd | strip-bound | identical | — | 1.093466 | [1.068480, 1.119036] | 1.178310 | OURS FASTER | 6.65 [5.92, 6.74] |
| M7 | moss | simd | unstrip-bound | identical | — | 1.096641 | [1.069424, 1.124550] | 1.114682 | OURS FASTER | 6.51 [5.92, 6.74] |
| M7 | moss | simd | meshlet-bound | identical | — | 1.061804 | [1.044209, 1.079695] | 1.078782 | OURS FASTER | 6.51 [5.92, 6.74] |
| M7 | moss | simd | remap | identical | — | 0.875969 | [0.866251, 0.885795] | 0.933899 | PEER FASTER | 6.69 [5.92, 6.74] |
| M7 | moss | simd | remap-multi | identical | — | 0.946426 | [0.927956, 0.965262] | 1.003936 | PEER FASTER | 5.99 [5.92, 6.76] |
| M7 | moss | simd | remap-custom | identical | — | 0.892481 | [0.879251, 0.905910] | 1.076830 | PEER FASTER | 6.70 [5.92, 6.76] |
| M7 | moss | simd | remap-vertices | identical | — | 0.837907 | [0.822270, 0.853841] | 0.914241 | PEER FASTER | 6.70 [5.92, 6.76] |
| M7 | moss | simd | remap-indices | identical | — | 0.830145 | [0.814166, 0.846438] | 0.879353 | PEER FASTER | 6.51 [5.92, 6.76] |
| M7 | moss | simd | shadow | identical | — | 0.918779 | [0.896995, 0.941093] | 0.960104 | PEER FASTER | 6.67 [5.92, 6.70] |
| M7 | moss | simd | shadow-multi | identical | — | 0.987392 | [0.969184, 1.005943] | 1.062281 | EQUAL WITHIN NOISE | 6.51 [5.92, 6.70] |
| M7 | moss | simd | position-remap | identical | — | 1.190687 | [1.155322, 1.227133] | 1.286513 | OURS FASTER | 6.67 [5.92, 6.70] |
| M7 | moss | simd | adjacency | identical | — | 0.754342 | [0.682141, 0.834185] | 0.894614 | PEER FASTER | 6.51 [5.92, 6.80] |
| M7 | moss | simd | tessellation | identical | — | 0.863447 | [0.831559, 0.896558] | 0.916970 | PEER FASTER | 6.08 [5.92, 7.62] |
| M7 | moss | simd | provoking | identical | — | 0.824887 | [0.808001, 0.842126] | 0.944159 | PEER FASTER | 6.51 [5.99, 7.49] |
| M7 | moss | simd | simplify-attributes | identical | — | 1.084089 | [1.061152, 1.107523] | 1.232198 | OURS FASTER | 7.04 [5.99, 8.23] |
| M7 | moss | simd | simplify-sloppy | identical | — | 0.782112 | [0.773411, 0.790910] | 0.856608 | PEER FASTER | 6.51 [5.99, 8.05] |
| M7 | moss | simd | simplify-prune | identical | — | 0.941189 | [0.917069, 0.965944] | 1.078295 | PEER FASTER | 6.51 [5.99, 8.13] |
| M7 | moss | simd | simplify-points | identical | — | 0.816810 | [0.796405, 0.837738] | 1.241893 | PEER FASTER | 6.51 [5.99, 8.13] |
| M7 | moss | simd | simplify-update | identical | — | 0.959632 | [0.930471, 0.989707] | 1.108748 | EQUAL WITHIN NOISE | 6.51 [5.99, 8.13] |
| M7 | moss | simd | analyze-cache | identical | — | 0.908255 | [0.894318, 0.922409] | 0.961601 | PEER FASTER | 6.39 [5.99, 8.12] |
| M7 | moss | simd | analyze-fetch | identical | — | 1.034969 | [1.024061, 1.045994] | 1.072050 | EQUAL WITHIN NOISE | 6.04 [5.99, 8.12] |
| M7 | moss | simd | analyze-overdraw | identical | — | 1.004109 | [0.996872, 1.011398] | 1.146900 | EQUAL WITHIN NOISE | 6.39 [5.99, 8.11] |
| M7 | moss | simd | analyze-coverage | identical | — | 0.964456 | [0.948582, 0.980595] | 1.095094 | EQUAL WITHIN NOISE | 6.04 [5.99, 8.11] |
| M7 | moss | simd | cluster-bounds | identical | — | 0.191781 | [0.184650, 0.199188] | 0.918988 | PEER FASTER | 6.39 [5.99, 8.11] |
| M7 | moss | simd | sphere-bounds | identical | — | 1.078713 | [1.036041, 1.123142] | 1.215123 | OURS FASTER | 6.59 [5.99, 7.78] |
| M7 | moss | simd | meshlet-bounds | identical | — | 0.021556 | [0.020876, 0.022257] | 0.606306 | PEER FASTER | 6.39 [5.99, 8.04] |
| M7 | moss | simd | optimize-meshlet | identical | — | 0.762020 | [0.747346, 0.776982] | 0.777496 | PEER FASTER | 6.39 [6.07, 8.00] |
| M7 | moss | simd | partition | identical | — | 0.663303 | [0.651345, 0.675481] | 0.717618 | PEER FASTER | 6.39 [6.07, 8.00] |
| M7 | moss | simd | spatial-remap | identical | — | 0.963157 | [0.953944, 0.972459] | 1.802849 | EQUAL WITHIN NOISE | 6.39 [6.07, 8.00] |
| M7 | moss | simd | spatial-triangles | identical | — | 0.877418 | [0.859196, 0.896025] | 1.545296 | PEER FASTER | 6.39 [6.07, 8.00] |
| M7 | moss | simd | spatial-points | identical | — | 0.942879 | [0.937786, 0.948000] | 1.015054 | PEER FASTER | 6.39 [6.07, 8.00] |
| M7 | moss | simd | quantize-unorm | identical | — | 1.632636 | [1.336204, 1.994832] | 1.701640 | OURS FASTER | 6.08 [6.07, 8.00] |
| M7 | moss | simd | quantize-snorm | identical | — | 1.019376 | [0.920840, 1.128457] | 1.033961 | EQUAL WITHIN NOISE | 6.39 [6.07, 8.00] |
| M7 | moss | simd | quantize-half | identical | — | 2.497246 | [2.223676, 2.804473] | 2.886405 | OURS FASTER | 6.27 [6.07, 8.00] |
| M7 | moss | simd | dequantize-half | identical | — | 4.383220 | [4.299640, 4.468424] | 4.470427 | OURS FASTER | 6.27 [6.07, 8.00] |
| M7 | moss | simd | quantize-float | identical | — | 2.915091 | [2.856262, 2.975132] | 4.372566 | OURS FASTER | 6.27 [6.07, 7.92] |
| M7 | moss | simd | simplify-scale | identical | — | 0.774636 | [0.738762, 0.812252] | 0.796352 | PEER FASTER | 6.27 [5.99, 7.92] |
| M7 | moss | simd | real-simplify | identical | — | 0.900439 | [0.855719, 0.947497] | 0.949966 | PEER FASTER | 10.36 [7.59, 14.37] |
| M7 | moss | simd | real-overdraw | identical | — | 0.664502 | [0.602794, 0.732528] | 0.721347 | PEER FASTER | 11.61 [10.65, 14.37] |
| X | moss | simd | vertex-decode | identical | — | 1.678678 | [1.604397, 1.756399] | 2.030716 | OURS FASTER | 11.04 [6.68, 12.82] |
| X | moss | simd | vertex-encode | identical | — | 2.039042 | [1.989043, 2.090298] | 2.336476 | OURS FASTER | 11.04 [6.68, 12.82] |
| X | moss | simd | vertex-encode-v0 | identical | — | 1.348454 | [1.293026, 1.406258] | 1.479780 | OURS FASTER | 12.64 [6.68, 12.82] |
| X | moss | simd | vertex-encode-default | identical | — | 2.037404 | [1.984605, 2.091607] | 2.361863 | OURS FASTER | 11.04 [6.68, 12.82] |
| X | moss | simd | index-decode | identical | — | 1.049192 | [1.023678, 1.075343] | 1.208970 | EQUAL WITHIN NOISE | 11.04 [6.68, 12.82] |
| X | moss | simd | index-encode | identical | — | 1.283709 | [1.247507, 1.320962] | 1.455734 | OURS FASTER | 11.04 [6.68, 12.82] |
| X | moss | simd | sequence-decode | identical | — | 0.949398 | [0.930421, 0.968762] | 0.986283 | PEER FASTER | 11.04 [6.63, 12.82] |
| X | moss | simd | sequence-encode | identical | — | 1.159547 | [1.136219, 1.183354] | 1.176975 | OURS FASTER | 12.45 [6.63, 12.82] |
| X | moss | simd | oct-decode | different but valid | [{"key":["moss","simd","tiny-smooth","oct-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","tiny-smooth","oct-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","medium-smooth","oct-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"d3cc5cbdf1a0"},{"key":["moss","simd","medium-smooth","oct-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"6f30ca1fba1c"},{"key":["moss","simd","medium-seams","oct-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2849811d9079"},{"key":["moss","simd","medium-seams","oct-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"d510948da1ae"},{"key":["moss","simd","medium-sparse","oct-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"e33f49c1bef7"},{"key":["moss","simd","medium-sparse","oct-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"fc87093c3600"},{"key":["moss","simd","million-smooth","oct-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"a446e4a744a5"},{"key":["moss","simd","million-smooth","oct-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"4d172c17508c"}] | 1.394375 | [1.362112, 1.427402] | 4.512655 | NOT EQUIVALENT | 10.64 [6.63, 12.82] |
| X | moss | simd | quat-decode | different but valid | [{"key":["moss","simd","tiny-smooth","quat-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","tiny-smooth","quat-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","medium-smooth","quat-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"a01bee8bc173"},{"key":["moss","simd","medium-smooth","quat-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"83cdcd1de1ff"},{"key":["moss","simd","medium-seams","quat-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"f0162e347409"},{"key":["moss","simd","medium-seams","quat-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"dee8c3bb2c59"},{"key":["moss","simd","medium-sparse","quat-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"35d9bf78dcb6"},{"key":["moss","simd","medium-sparse","quat-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"5a76235dadd8"},{"key":["moss","simd","million-smooth","quat-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"179baa9dec7d"},{"key":["moss","simd","million-smooth","quat-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"d8a7e4a95b31"}] | 1.256111 | [1.219488, 1.293834] | 5.020122 | NOT EQUIVALENT | 12.45 [6.63, 12.60] |
| X | moss | simd | exp-decode | identical | — | 2.339294 | [1.832058, 2.986967] | 14.912033 | OURS FASTER | 6.63 [5.56, 12.60] |
| X | moss | simd | color-decode | different but valid | [{"key":["moss","simd","tiny-smooth","color-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","tiny-smooth","color-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","medium-smooth","color-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"0959d662b556"},{"key":["moss","simd","medium-smooth","color-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"87f29967705f"},{"key":["moss","simd","medium-seams","color-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2b3f2d54e513"},{"key":["moss","simd","medium-seams","color-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"a7d7162e42e3"},{"key":["moss","simd","medium-sparse","color-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"46d56078ce38"},{"key":["moss","simd","medium-sparse","color-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"719b1634a0eb"},{"key":["moss","simd","million-smooth","color-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"65521c277333"},{"key":["moss","simd","million-smooth","color-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"6c391c916089"}] | 1.734526 | [1.672180, 1.799197] | 6.765494 | NOT EQUIVALENT | 7.35 [5.56, 12.60] |
| X | moss | simd | oct-encode | identical | — | 2.044445 | [2.010577, 2.078882] | 4.289880 | OURS FASTER | 7.35 [5.56, 12.60] |
| X | moss | simd | quat-encode | identical | — | 2.434003 | [2.382284, 2.486845] | 5.024875 | OURS FASTER | 7.35 [5.56, 12.60] |
| X | moss | simd | exp-encode | identical | — | 6.306913 | [6.121300, 6.498155] | 7.079242 | OURS FASTER | 7.35 [6.00, 12.60] |
| X | moss | simd | color-encode | identical | — | 1.530035 | [1.500164, 1.560500] | 1.593597 | OURS FASTER | 7.35 [6.00, 12.60] |
| X | moss | simd | cache | identical | — | 1.180722 | [1.160943, 1.200837] | 1.376598 | OURS FASTER | 7.00 [6.00, 12.60] |
| X | moss | simd | cache-strip | identical | — | 1.160936 | [1.136223, 1.186186] | 1.306306 | OURS FASTER | 7.00 [6.00, 12.60] |
| X | moss | simd | cache-fifo | identical | — | 0.943853 | [0.922548, 0.965649] | 1.323731 | PEER FASTER | 12.45 [6.00, 12.60] |
| X | moss | simd | overdraw | identical | — | 0.753076 | [0.739103, 0.767313] | 0.890628 | PEER FASTER | 7.00 [6.00, 12.60] |
| X | moss | simd | fetch | identical | — | 0.748797 | [0.716625, 0.782412] | 0.811438 | PEER FASTER | 12.45 [6.00, 12.60] |
| X | moss | simd | fetch-remap | identical | — | 0.486204 | [0.472690, 0.500104] | 0.583212 | PEER FASTER | 7.33 [6.00, 13.70] |
| X | moss | simd | simplify | identical | — | 1.119256 | [1.096252, 1.142742] | 1.458998 | OURS FASTER | 7.33 [6.00, 13.70] |
| X | moss | simd | stripify | identical | — | 0.946734 | [0.933185, 0.960479] | 1.116487 | PEER FASTER | 7.46 [6.00, 13.70] |
| X | moss | simd | unstripify | identical | — | 1.334825 | [1.309905, 1.360219] | 1.405912 | OURS FASTER | 12.96 [6.00, 13.70] |
| X | moss | simd | meshlets | identical | — | 1.108498 | [1.089912, 1.127401] | 1.241948 | OURS FASTER | 7.50 [6.00, 13.70] |
| X | moss | simd | meshlets-scan | identical | — | 1.241997 | [1.188531, 1.297869] | 1.356255 | OURS FASTER | 7.50 [6.00, 13.70] |
| X | moss | simd | meshlets-flex | identical | — | 1.112265 | [1.092673, 1.132208] | 1.215270 | OURS FASTER | 7.30 [6.00, 13.70] |
| X | moss | simd | meshlets-spatial | identical | — | 0.883875 | [0.864064, 0.904139] | 0.954444 | PEER FASTER | 12.96 [6.00, 13.70] |
| X | moss | simd | vertex-version | identical | — | 1.019439 | [1.004902, 1.034185] | 1.041573 | EQUAL WITHIN NOISE | 7.28 [6.00, 13.70] |
| X | moss | simd | index-version | identical | — | 0.993074 | [0.974073, 1.012447] | 1.023239 | EQUAL WITHIN NOISE | 7.10 [6.32, 13.70] |
| X | moss | simd | vertex-bound | different but valid | [{"key":["moss","simd","tiny-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"21598d236237","capacity":3156},{"key":["moss","simd","tiny-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"7ef62e8e8b27","capacity":3189},{"key":["moss","simd","medium-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"f0fd58164d7c","capacity":53124},{"key":["moss","simd","medium-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"e5412ab2f934","capacity":53685},{"key":["moss","simd","medium-seams","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"643edcdb3895","capacity":65616},{"key":["moss","simd","medium-seams","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"6fd92eec93a8","capacity":66309},{"key":["moss","simd","medium-sparse","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"2fe4e42b13e0","capacity":106215},{"key":["moss","simd","medium-sparse","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"23e3210734fc","capacity":107337},{"key":["moss","simd","million-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"f20e6a7a90c9","capacity":6133605},{"key":["moss","simd","million-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"e716b9043846","capacity":6198417}] | 1.053713 | [1.020594, 1.087906] | 1.173774 | NOT EQUIVALENT | 7.10 [6.32, 13.70] |
| X | moss | simd | index-bound | identical | — | 1.211393 | [1.180077, 1.243540] | 1.299161 | OURS FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | sequence-bound | identical | — | 1.239080 | [1.192200, 1.287805] | 1.312463 | OURS FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | strip-bound | identical | — | 0.922051 | [0.894979, 0.949941] | 1.056946 | PEER FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | unstrip-bound | identical | — | 1.119057 | [1.094801, 1.143851] | 1.234176 | OURS FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | meshlet-bound | identical | — | 1.074869 | [1.052440, 1.097775] | 1.168080 | OURS FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | remap | identical | — | 0.943209 | [0.918695, 0.968377] | 1.053222 | PEER FASTER | 7.10 [6.32, 13.70] |
| X | moss | simd | remap-multi | identical | — | 0.975790 | [0.944433, 1.008189] | 1.098107 | EQUAL WITHIN NOISE | 12.23 [6.32, 13.70] |
| X | moss | simd | remap-custom | identical | — | 0.947918 | [0.897561, 1.001102] | 1.357362 | EQUAL WITHIN NOISE | 6.75 [6.32, 13.70] |
| X | moss | simd | remap-vertices | identical | — | 1.037039 | [1.019999, 1.054364] | 1.129748 | EQUAL WITHIN NOISE | 7.01 [6.32, 13.70] |
| X | moss | simd | remap-indices | identical | — | 0.896271 | [0.874733, 0.918338] | 1.043974 | PEER FASTER | 12.87 [6.32, 13.70] |
| X | moss | simd | shadow | identical | — | 0.905085 | [0.862205, 0.950097] | 1.032746 | PEER FASTER | 12.05 [6.32, 13.70] |
| X | moss | simd | shadow-multi | identical | — | 0.960087 | [0.926905, 0.994457] | 1.099303 | EQUAL WITHIN NOISE | 12.05 [6.32, 13.70] |
| X | moss | simd | position-remap | identical | — | 1.365401 | [1.336946, 1.394462] | 1.606742 | OURS FASTER | 7.01 [6.32, 13.70] |
| X | moss | simd | adjacency | identical | — | 0.762348 | [0.726436, 0.800036] | 0.821548 | PEER FASTER | 7.25 [6.32, 13.70] |
| X | moss | simd | tessellation | identical | — | 0.865954 | [0.842642, 0.889911] | 0.946617 | PEER FASTER | 12.65 [6.32, 13.24] |
| X | moss | simd | provoking | identical | — | 0.914000 | [0.891399, 0.937173] | 0.949530 | PEER FASTER | 7.47 [6.32, 13.24] |
| X | moss | simd | simplify-attributes | identical | — | 1.164093 | [1.137373, 1.191441] | 1.419775 | OURS FASTER | 7.72 [6.62, 13.24] |
| X | moss | simd | simplify-sloppy | identical | — | 0.987378 | [0.969693, 1.005385] | 1.170637 | EQUAL WITHIN NOISE | 7.01 [6.62, 13.24] |
| X | moss | simd | simplify-prune | identical | — | 0.948307 | [0.934271, 0.962553] | 0.998706 | PEER FASTER | 12.65 [6.62, 13.24] |
| X | moss | simd | simplify-points | identical | — | 1.188979 | [1.172066, 1.206135] | 1.401256 | OURS FASTER | 7.54 [6.62, 13.24] |
| X | moss | simd | simplify-update | identical | — | 1.043432 | [1.021118, 1.066233] | 1.210775 | EQUAL WITHIN NOISE | 7.34 [6.62, 13.24] |
| X | moss | simd | analyze-cache | identical | — | 1.094235 | [1.070322, 1.118682] | 1.103616 | OURS FASTER | 12.65 [6.62, 13.24] |
| X | moss | simd | analyze-fetch | identical | — | 1.196490 | [1.175827, 1.217516] | 1.266462 | OURS FASTER | 12.40 [6.62, 13.24] |
| X | moss | simd | analyze-overdraw | identical | — | 2.101126 | [2.053271, 2.150096] | 3.455618 | OURS FASTER | 7.81 [6.62, 13.24] |
| X | moss | simd | analyze-coverage | identical | — | 2.116836 | [2.034383, 2.202630] | 3.759775 | OURS FASTER | 7.81 [6.62, 13.24] |
| X | moss | simd | cluster-bounds | identical | — | 0.210909 | [0.202924, 0.219207] | 1.059354 | PEER FASTER | 12.40 [6.62, 13.24] |
| X | moss | simd | sphere-bounds | identical | — | 1.295300 | [1.262055, 1.329421] | 1.397309 | OURS FASTER | 7.84 [6.62, 13.24] |
| X | moss | simd | meshlet-bounds | identical | — | 0.032225 | [0.031279, 0.033201] | 0.812504 | PEER FASTER | 12.40 [6.57, 13.24] |
| X | moss | simd | optimize-meshlet | identical | — | 0.795318 | [0.780042, 0.810893] | 0.827365 | PEER FASTER | 8.09 [6.57, 13.24] |
| X | moss | simd | partition | identical | — | 0.894962 | [0.876266, 0.914056] | 1.008653 | PEER FASTER | 7.33 [6.57, 13.24] |
| X | moss | simd | spatial-remap | identical | — | 1.054357 | [1.037024, 1.071979] | 1.664492 | OURS FASTER | 7.33 [6.57, 13.24] |
| X | moss | simd | spatial-triangles | identical | — | 0.974606 | [0.958384, 0.991103] | 1.560536 | EQUAL WITHIN NOISE | 8.40 [6.57, 13.24] |
| X | moss | simd | spatial-points | identical | — | 0.986166 | [0.971578, 1.000973] | 1.044739 | EQUAL WITHIN NOISE | 8.40 [6.57, 13.24] |
| X | moss | simd | quantize-unorm | identical | — | 1.636205 | [1.363086, 1.964048] | 1.782860 | OURS FASTER | 8.40 [6.57, 13.24] |
| X | moss | simd | quantize-snorm | identical | — | 1.012899 | [0.913511, 1.123099] | 1.044510 | EQUAL WITHIN NOISE | 8.61 [6.57, 12.82] |
| X | moss | simd | quantize-half | identical | — | 0.684106 | [0.601146, 0.778515] | 0.823591 | PEER FASTER | 8.61 [6.68, 12.82] |
| X | moss | simd | dequantize-half | identical | — | 1.047985 | [1.033029, 1.063157] | 1.074798 | OURS FASTER | 7.33 [6.68, 12.82] |
| X | moss | simd | quantize-float | identical | — | 0.730767 | [0.715027, 0.746853] | 0.911069 | PEER FASTER | 11.93 [6.68, 12.82] |
| X | moss | simd | simplify-scale | identical | — | 0.868196 | [0.846275, 0.890684] | 0.917730 | PEER FASTER | 11.93 [6.68, 12.82] |
| X | moss | simd | real-simplify | identical | — | 1.024112 | [0.986787, 1.062849] | 1.105457 | EQUAL WITHIN NOISE | 8.32 [7.82, 8.46] |
| X | moss | simd | real-overdraw | identical | — | 0.664637 | [0.650436, 0.679147] | 0.696663 | PEER FASTER | 8.01 [7.92, 8.34] |
| X | defaults | wasm-simd128 | browser-attributes | identical to fresh scalar C++ 1.3 bytes | — | 1.425285 | [1.405650, 1.445194] | 1.681455 | OURS FASTER | 8.96 [7.66, 9.51] |
| X | defaults | wasm-simd128 | browser-triangles | identical to fresh scalar C++ 1.3 bytes | — | 1.163473 | [1.140644, 1.186759] | 1.921873 | OURS FASTER | 9.02 [7.66, 9.51] |
| JS | defaults | wasm-simd128 | browser-attributes | identical to fresh scalar C++ 1.3 bytes | — | 0.861314 | [0.849580, 0.873209] | 1.023523 | PEER FASTER | 9.13 [7.66, 9.51] |
| JS | defaults | wasm-simd128 | browser-triangles | identical to fresh scalar C++ 1.3 bytes | — | 0.941564 | [0.937033, 0.946117] | 1.074612 | PEER FASTER | 8.96 [7.66, 9.51] |
| X | moss | wasm-simd128 | browser-attributes | identical to fresh scalar C++ 1.3 bytes | — | 1.625023 | [1.600451, 1.649973] | 1.937180 | OURS FASTER | 10.39 [6.48, 13.43] |
| X | moss | wasm-simd128 | browser-triangles | identical to fresh scalar C++ 1.3 bytes | — | 0.998450 | [0.982980, 1.014163] | 1.094189 | EQUAL WITHIN NOISE | 10.39 [6.48, 13.43] |
| JS | moss | wasm-simd128 | browser-attributes | identical to fresh scalar C++ 1.3 bytes | — | 0.973263 | [0.963310, 0.983320] | 1.169900 | EQUAL WITHIN NOISE | 9.72 [6.48, 13.43] |
| JS | moss | wasm-simd128 | browser-triangles | identical to fresh scalar C++ 1.3 bytes | — | 0.927864 | [0.879507, 0.978880] | 1.505335 | EQUAL WITHIN NOISE | 9.72 [6.00, 13.43] |

Non-equivalent output metrics (same count/error fields as the original screen):

- M7 / oct-decode: `[{"key":["moss","simd","tiny-smooth","oct-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","tiny-smooth","oct-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","medium-smooth","oct-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"d3cc5cbdf1a0"},{"key":["moss","simd","medium-smooth","oct-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"6f30ca1fba1c"},{"key":["moss","simd","medium-seams","oct-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2849811d9079"},{"key":["moss","simd","medium-seams","oct-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"d510948da1ae"},{"key":["moss","simd","medium-sparse","oct-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"e33f49c1bef7"},{"key":["moss","simd","medium-sparse","oct-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"fc87093c3600"},{"key":["moss","simd","million-smooth","oct-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"a446e4a744a5"},{"key":["moss","simd","million-smooth","oct-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"4d172c17508c"}]`.
- M7 / quat-decode: `[{"key":["moss","simd","tiny-smooth","quat-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","tiny-smooth","quat-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","medium-smooth","quat-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"a01bee8bc173"},{"key":["moss","simd","medium-smooth","quat-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"83cdcd1de1ff"},{"key":["moss","simd","medium-seams","quat-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"f0162e347409"},{"key":["moss","simd","medium-seams","quat-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"dee8c3bb2c59"},{"key":["moss","simd","medium-sparse","quat-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"35d9bf78dcb6"},{"key":["moss","simd","medium-sparse","quat-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"5a76235dadd8"},{"key":["moss","simd","million-smooth","quat-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"179baa9dec7d"},{"key":["moss","simd","million-smooth","quat-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"d8a7e4a95b31"}]`.
- M7 / color-decode: `[{"key":["moss","simd","tiny-smooth","color-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","tiny-smooth","color-decode","M7"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","medium-smooth","color-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"0959d662b556"},{"key":["moss","simd","medium-smooth","color-decode","M7"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"87f29967705f"},{"key":["moss","simd","medium-seams","color-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2b3f2d54e513"},{"key":["moss","simd","medium-seams","color-decode","M7"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"a7d7162e42e3"},{"key":["moss","simd","medium-sparse","color-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"46d56078ce38"},{"key":["moss","simd","medium-sparse","color-decode","M7"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"719b1634a0eb"},{"key":["moss","simd","million-smooth","color-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"65521c277333"},{"key":["moss","simd","million-smooth","color-decode","M7"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"6c391c916089"}]`.
- X / oct-decode: `[{"key":["moss","simd","tiny-smooth","oct-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","tiny-smooth","oct-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"2eb57e899955"},{"key":["moss","simd","medium-smooth","oct-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"d3cc5cbdf1a0"},{"key":["moss","simd","medium-smooth","oct-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"6f30ca1fba1c"},{"key":["moss","simd","medium-seams","oct-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2849811d9079"},{"key":["moss","simd","medium-seams","oct-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"d510948da1ae"},{"key":["moss","simd","medium-sparse","oct-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"e33f49c1bef7"},{"key":["moss","simd","medium-sparse","oct-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"fc87093c3600"},{"key":["moss","simd","million-smooth","oct-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"a446e4a744a5"},{"key":["moss","simd","million-smooth","oct-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"4d172c17508c"}]`.
- X / quat-decode: `[{"key":["moss","simd","tiny-smooth","quat-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","tiny-smooth","quat-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"27c350ac15c3"},{"key":["moss","simd","medium-smooth","quat-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"a01bee8bc173"},{"key":["moss","simd","medium-smooth","quat-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"83cdcd1de1ff"},{"key":["moss","simd","medium-seams","quat-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"f0162e347409"},{"key":["moss","simd","medium-seams","quat-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"dee8c3bb2c59"},{"key":["moss","simd","medium-sparse","quat-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"35d9bf78dcb6"},{"key":["moss","simd","medium-sparse","quat-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"5a76235dadd8"},{"key":["moss","simd","million-smooth","quat-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"179baa9dec7d"},{"key":["moss","simd","million-smooth","quat-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"d8a7e4a95b31"}]`.
- X / color-decode: `[{"key":["moss","simd","tiny-smooth","color-decode","O"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","tiny-smooth","color-decode","X"],"status":"identical","bytes":200,"output_sha256_prefix":"a6afb10f8d3f"},{"key":["moss","simd","medium-smooth","color-decode","O"],"status":"identical","bytes":33800,"output_sha256_prefix":"0959d662b556"},{"key":["moss","simd","medium-smooth","color-decode","X"],"status":"different but valid","bytes":33800,"output_sha256_prefix":"87f29967705f"},{"key":["moss","simd","medium-seams","color-decode","O"],"status":"identical","bytes":41472,"output_sha256_prefix":"2b3f2d54e513"},{"key":["moss","simd","medium-seams","color-decode","X"],"status":"different but valid","bytes":41472,"output_sha256_prefix":"a7d7162e42e3"},{"key":["moss","simd","medium-sparse","color-decode","O"],"status":"identical","bytes":67600,"output_sha256_prefix":"46d56078ce38"},{"key":["moss","simd","medium-sparse","color-decode","X"],"status":"different but valid","bytes":67600,"output_sha256_prefix":"719b1634a0eb"},{"key":["moss","simd","million-smooth","color-decode","O"],"status":"identical","bytes":4021448,"output_sha256_prefix":"65521c277333"},{"key":["moss","simd","million-smooth","color-decode","X"],"status":"different but valid","bytes":4021448,"output_sha256_prefix":"6c391c916089"}]`.
- X / vertex-bound: `[{"key":["moss","simd","tiny-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"21598d236237","capacity":3156},{"key":["moss","simd","tiny-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"7ef62e8e8b27","capacity":3189},{"key":["moss","simd","medium-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"f0fd58164d7c","capacity":53124},{"key":["moss","simd","medium-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"e5412ab2f934","capacity":53685},{"key":["moss","simd","medium-seams","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"643edcdb3895","capacity":65616},{"key":["moss","simd","medium-seams","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"6fd92eec93a8","capacity":66309},{"key":["moss","simd","medium-sparse","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"2fe4e42b13e0","capacity":106215},{"key":["moss","simd","medium-sparse","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"23e3210734fc","capacity":107337},{"key":["moss","simd","million-smooth","vertex-bound","O"],"status":"identical","bytes":8,"output_sha256_prefix":"f20e6a7a90c9","capacity":6133605},{"key":["moss","simd","million-smooth","vertex-bound","X"],"status":"different but valid","bytes":8,"output_sha256_prefix":"e716b9043846","capacity":6198417}]`.

Still unmeasured in this rerun: 0.


Load conditions are shown beside every new timing: native per-pair /proc load averages; browser pairs matched to the nearest five-second host observation (within 30 seconds). Full load1/load5/load15 ranges and pressure/memory/frequency snapshots are retained in results.json and host-load.jsonl. Submissions declare at most 4 GB. The final browser-only admission declares 2 GB with a specific browser class, supported by the identical default-browser job's 0.5481 GiB measured peak; generic Python history had reserved 9 GB. The queued generic browser job was cancelled before timing, with its receipt retained. Wrapper admission and NG priority stayed enabled. The shared host was noisy; no quiet-host inference is made. Headless Chromium only. Browser O/X/JS bytes must match fresh scalar C++ 1.3 references before timing; JS is a reference comparator. Full labels, sample ratios, pair counts, input/binary identities, failures and PAUSEs are retained in rerun results and raw receipts. No production, other-host, architecture, browser or cluster-LOD performance acceptance is inferred.
