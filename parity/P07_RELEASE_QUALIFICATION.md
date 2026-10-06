# P07 fix10 release qualification — 2026-10-06

RFC ratios Rust/C++: family geometric mean <=1.25 on **every** row;
case upper 95% bound <=1.50 only when **both** independent A/A upper bounds
are <=1.25. Unresolvable rows do not participate in case maxima and remain
visible below with both A/A widths. Stage-one medians feed means; final
intervals use fresh stage two only when stage one straddles 1.50. Intervals
are nominal paired Student-t log-ratio intervals under the registered
sequential stopping policy, not simultaneous or sequentially adjusted bounds.

| Platform / family | Allocating mean / resolved max / verdict | Caller mean / resolved max / verdict | Resolved A / C |
|---|---|---|---|
| native / color | 0.996 / 1.364 / PASS | 1.105 / 1.401 / PASS | 5/6 / 6/6 |
| native / exp | 0.896 / 1.465 / PASS | 0.895 / 1.270 / PASS | 10/12 / 8/12 |
| native / index | 1.187 / 3.625 / **FAIL** | 1.142 / 1.444 / PASS | 9/14 / 9/14 |
| native / meshlet | 1.132 / 1.431 / PASS | 1.240 / 1.483 / PASS | 9/12 / 5/12 |
| native / meshlet-raw | 1.171 / 1.353 / PASS | 1.233 / 1.490 / PASS | 3/3 / 2/3 |
| native / oct | 0.931 / 1.492 / PASS | 0.932 / 1.496 / PASS | 8/12 / 8/12 |
| native / quat | 0.886 / 1.460 / PASS | 0.868 / 1.467 / PASS | 3/6 / 4/6 |
| native / sequence | 1.131 / 1.457 / PASS | 1.080 / 1.477 / PASS | 9/12 / 10/12 |
| native / vertex | 1.116 / 2.118 / **FAIL** | 1.051 / 1.440 / PASS | 16/22 / 13/22 |
| native / view-filtered | 1.089 / 1.636 / **FAIL** | 1.028 / 1.436 / PASS | 19/30 / 23/30 |
| native / view-none | 1.040 / 1.266 / PASS | 1.033 / 1.447 / PASS | 6/9 / 5/9 |
| wasm / index | 0.877 / 1.422 / PASS | 1.042 / 1.337 / PASS | 3/14 / 11/14 |
| wasm / sequence | 0.864 / 1.049 / PASS | 0.815 / 0.956 / PASS | 5/12 / 8/12 |
| wasm / vertex | 0.913 / 1.481 / PASS | 0.858 / 1.260 / PASS | 11/22 / 19/22 |
| wasm / view-filtered | 0.889 / 1.471 / PASS | 0.945 / 1.466 / PASS | 9/30 / 26/30 |
| wasm / view-none | 0.954 / 1.311 / PASS | 0.838 / 0.963 / PASS | 6/9 / 6/9 |

This table is the requested RFC performance qualification on this Linux
x86-64/Node host and registered allocation epoch. It is not crates.io release,
ARM, Windows/macOS, integration, or every amendment gate acceptance. Native
allocating means the Vec-returning API; WASM allocating means the JavaScript
adapter described below. Native scalar C++ control remains in every burst;
the registered SIMD/JS comparators provide the RFC verdicts.

## Requested native allocating gaps

| Case | Prior final CI | Fix10 final CI | New/old paired CI | A/A upper Rust / C++ | Outcome |
|---|---|---|---|---|---|
| vertex-v1-streaming-s4 | 1.850–2.103 | 1.529–2.118 | 0.991–1.197 | 1.087 / 1.039 | FAIL |
| varied-view-3-streaming-s32 | 1.761–2.122 | 1.619–2.590 | 0.697–1.002 | 1.284 / 1.026 | unresolvable |

The allocating vertex gap remains a resolved FAIL. The allocating filtered-view
gap is unresolvable under its frozen A/A cutoff and is not demonstrated closed;
its final interval remains above the maximum bar. Block initialization did not
clear either requested gap. Further tuning is abandoned within this one-pass
specification; counters and paired old-control evidence are retained.

## Resolved maximum failures

| Platform / API | Case | Final CI |
|---|---|---|
| native / allocating | view-1-streaming-s4 | 1.343–1.636 |
| native / allocating | index-2-v0-streaming-s4 | 1.508–3.625 |
| native / allocating | index-2-v1-streaming-s4 | 2.189–2.465 |
| native / allocating | vertex-v1-streaming-s4 | 1.529–2.118 |

## Unresolvable rows

Width is upper minus lower in ratio units. Each row remains in its family
mean. No maximum-pass claim is made for an excluded row. Unrounded artifact
JSON values govern the cutoff; table intervals and widths are rounded.

| Platform / API | Case | Rust A/A CI (width) | C++ A/A CI (width) | Final R/C CI |
|---|---|---|---|---|
| native / allocating | vertex-v0-tiny-s4 | 1.014–1.413 (0.399) | 0.857–1.075 (0.218) | 1.027–1.071 |
| native / allocating | filter-1-tiny-s8 | 0.882–1.301 (0.420) | 0.841–1.262 (0.422) | 0.881–1.012 |
| native / allocating | index-2-v0-tiny-s4 | 0.770–1.212 (0.442) | 0.810–1.476 (0.666) | 1.233–1.336 |
| native / allocating | index-2-v1-tiny-s4 | 0.617–1.264 (0.646) | 0.964–1.301 (0.336) | 1.148–1.305 |
| native / allocating | vertex-v0-resident-s32 | 0.875–1.430 (0.555) | 0.975–1.230 (0.255) | 0.892–1.103 |
| native / allocating | view-none-resident-s32 | 0.991–1.043 (0.052) | 0.919–1.495 (0.577) | 0.874–1.420 |
| native / allocating | filter-1-resident-s4 | 0.910–1.292 (0.382) | 0.745–1.202 (0.458) | 0.460–0.634 |
| native / allocating | view-1-resident-s4 | 0.963–1.058 (0.096) | 0.914–1.319 (0.405) | 0.490–0.807 |
| native / allocating | filter-2-resident-s8 | 0.732–1.495 (0.763) | 0.926–1.059 (0.132) | 0.264–0.316 |
| native / allocating | index-3-v1-resident-s4 | 0.916–1.280 (0.364) | 0.948–1.024 (0.075) | 0.951–1.201 |
| native / allocating | view-none-streaming-s4 | 0.797–1.255 (0.457) | 0.592–1.479 (0.887) | 0.852–1.088 |
| native / allocating | vertex-v0-streaming-s12 | 0.858–1.051 (0.194) | 0.816–1.310 (0.494) | 1.008–1.247 |
| native / allocating | view-none-streaming-s12 | 0.847–1.267 (0.420) | 0.863–0.985 (0.122) | 0.915–1.256 |
| native / allocating | vertex-v0-streaming-s32 | 0.911–1.418 (0.507) | 0.946–1.110 (0.164) | 0.816–1.305 |
| native / allocating | filter-1-streaming-s8 | 0.954–1.243 (0.289) | 0.915–1.383 (0.469) | 0.778–1.074 |
| native / allocating | view-1-streaming-s8 | 0.670–1.480 (0.810) | 1.030–1.121 (0.090) | 0.747–0.933 |
| native / allocating | filter-2-streaming-s8 | 0.989–1.053 (0.064) | 0.939–1.385 (0.446) | 0.589–1.060 |
| native / allocating | index-2-v0-streaming-s2 | 0.813–1.453 (0.641) | 0.954–1.270 (0.316) | 1.217–1.322 |
| native / allocating | index-2-v1-streaming-s2 | 0.686–1.353 (0.667) | 0.886–1.498 (0.612) | 0.923–1.437 |
| native / allocating | index-3-v0-streaming-s2 | 0.841–1.438 (0.597) | 0.862–1.031 (0.169) | 0.892–1.410 |
| native / allocating | index-3-v1-streaming-s2 | 0.856–1.473 (0.617) | 0.966–0.994 (0.029) | 1.040–1.161 |
| native / allocating | vertex-v1-tiny-s12 | 0.870–1.465 (0.595) | 0.876–1.136 (0.261) | 0.890–1.394 |
| native / allocating | vertex-v1-streaming-s12 | 0.835–1.403 (0.568) | 0.910–1.099 (0.189) | 0.738–1.424 |
| native / allocating | index-2-v0-millions-s4 | 0.803–1.173 (0.370) | 0.839–1.419 (0.580) | 0.839–1.189 |
| native / allocating | varied-filter-2-tiny-s8 | 0.914–1.282 (0.368) | 0.983–1.041 (0.059) | 1.144–1.442 |
| native / allocating | varied-view-3-tiny-s12 | 0.954–1.075 (0.121) | 1.118–1.303 (0.185) | 1.114–1.477 |
| native / allocating | varied-filter-3-tiny-s32 | 0.854–1.263 (0.409) | 0.796–1.032 (0.236) | 0.717–0.927 |
| native / allocating | varied-view-3-tiny-s32 | 0.977–1.054 (0.077) | 1.054–1.297 (0.243) | 0.968–1.238 |
| native / allocating | varied-view-2-resident-s8 | 1.003–1.041 (0.039) | 0.865–1.476 (0.611) | 1.095–1.446 |
| native / allocating | varied-filter-3-resident-s4 | 0.950–1.214 (0.265) | 0.803–1.478 (0.675) | 0.950–1.114 |
| native / allocating | varied-view-3-resident-s12 | 0.772–1.058 (0.287) | 0.918–1.463 (0.545) | 1.044–1.166 |
| native / allocating | varied-view-3-resident-s32 | 0.928–1.019 (0.092) | 0.907–1.421 (0.514) | 0.826–1.215 |
| native / allocating | varied-filter-1-streaming-s8 | 0.972–1.090 (0.118) | 0.818–1.360 (0.542) | 1.068–1.112 |
| native / allocating | varied-view-2-streaming-s8 | 0.647–1.152 (0.505) | 0.927–1.469 (0.542) | 1.142–1.224 |
| native / allocating | varied-view-3-streaming-s4 | 0.911–1.048 (0.138) | 0.967–1.326 (0.359) | 0.893–0.981 |
| native / allocating | varied-view-3-streaming-s12 | 0.881–1.395 (0.514) | 0.971–1.253 (0.282) | 1.026–1.309 |
| native / allocating | varied-view-3-streaming-s32 | 0.904–1.284 (0.380) | 0.919–1.026 (0.108) | 1.619–2.590 |
| native / allocating | varied-filter-4-streaming-s8 | 0.849–1.408 (0.559) | 0.973–1.018 (0.045) | 0.686–1.166 |
| native / allocating | meshlet-1-1-v2-t4 | 0.627–1.271 (0.644) | 0.845–1.297 (0.452) | 1.079–1.235 |
| native / allocating | meshlet-1-1-v4-t3 | 0.955–1.075 (0.120) | 0.985–1.317 (0.332) | 0.992–1.233 |
| native / allocating | meshlet-256-256-v4-t4 | 0.998–1.008 (0.010) | 0.891–1.335 (0.444) | 0.978–1.339 |
| native / caller-buffer | vertex-v0-tiny-s4 | 0.768–1.258 (0.490) | 0.971–1.008 (0.037) | 1.100–1.267 |
| native / caller-buffer | vertex-v0-tiny-s32 | 0.551–1.353 (0.801) | 1.000–1.020 (0.020) | 1.066–1.271 |
| native / caller-buffer | filter-1-tiny-s8 | 0.839–1.453 (0.614) | 0.979–1.036 (0.057) | 0.663–1.125 |
| native / caller-buffer | view-1-tiny-s8 | 0.805–1.488 (0.683) | 0.763–1.196 (0.433) | 1.049–1.347 |
| native / caller-buffer | index-2-v0-tiny-s4 | 0.970–1.284 (0.314) | 0.805–1.193 (0.388) | 1.197–1.487 |
| native / caller-buffer | view-none-resident-s12 | 0.888–1.250 (0.362) | 0.851–1.089 (0.238) | 1.029–1.475 |
| native / caller-buffer | view-none-resident-s32 | 0.872–1.091 (0.219) | 0.863–1.468 (0.605) | 0.894–1.274 |
| native / caller-buffer | filter-1-resident-s8 | 0.846–1.387 (0.541) | 0.928–1.029 (0.101) | 0.324–0.701 |
| native / caller-buffer | index-3-v0-resident-s2 | 0.671–1.269 (0.598) | 0.943–1.032 (0.089) | 0.655–1.132 |
| native / caller-buffer | vertex-v0-streaming-s4 | 0.838–1.465 (0.628) | 0.976–1.051 (0.075) | 0.729–1.313 |
| native / caller-buffer | view-none-streaming-s4 | 0.915–1.067 (0.152) | 0.878–1.274 (0.396) | 0.834–0.997 |
| native / caller-buffer | view-none-streaming-s12 | 0.677–1.411 (0.735) | 0.604–1.113 (0.508) | 0.830–1.139 |
| native / caller-buffer | filter-1-streaming-s4 | 0.942–1.017 (0.075) | 0.962–1.493 (0.531) | 0.437–0.567 |
| native / caller-buffer | filter-2-streaming-s8 | 0.673–1.432 (0.759) | 0.752–1.335 (0.584) | 0.362–0.659 |
| native / caller-buffer | view-2-streaming-s8 | 0.700–1.458 (0.758) | 0.963–1.121 (0.158) | 0.588–0.780 |
| native / caller-buffer | filter-3-streaming-s12 | 0.977–1.058 (0.081) | 0.507–1.414 (0.907) | 0.566–1.262 |
| native / caller-buffer | index-2-v0-streaming-s4 | 0.854–1.428 (0.574) | 0.840–1.479 (0.639) | 0.970–1.492 |
| native / caller-buffer | index-2-v1-streaming-s2 | 0.813–1.269 (0.456) | 0.984–1.010 (0.027) | 0.969–1.098 |
| native / caller-buffer | index-3-v1-streaming-s4 | 0.844–1.363 (0.519) | 0.642–1.282 (0.640) | 0.918–1.057 |
| native / caller-buffer | vertex-v1-resident-s4 | 0.951–1.085 (0.134) | 0.930–1.484 (0.553) | 0.971–1.086 |
| native / caller-buffer | vertex-v1-streaming-s4 | 0.903–1.256 (0.353) | 0.943–1.044 (0.101) | 0.679–0.938 |
| native / caller-buffer | vertex-v1-streaming-s12 | 0.597–1.414 (0.817) | 0.749–1.288 (0.539) | 0.766–1.248 |
| native / caller-buffer | vertex-v1-streaming-s32 | 0.799–1.449 (0.650) | 0.847–0.975 (0.128) | 0.613–0.879 |
| native / caller-buffer | vertex-v0-resident-s12-level9 | 0.890–1.045 (0.155) | 0.621–1.436 (0.815) | 0.937–1.342 |
| native / caller-buffer | vertex-v1-resident-s12-level0 | 0.832–1.381 (0.549) | 0.830–1.438 (0.608) | 1.003–1.453 |
| native / caller-buffer | index-2-v0-millions-s4 | 0.687–1.471 (0.784) | 0.925–1.101 (0.176) | 0.966–1.463 |
| native / caller-buffer | index-2-v1-millions-s4 | 0.686–1.404 (0.718) | 0.942–1.063 (0.121) | 1.021–1.243 |
| native / caller-buffer | varied-filter-1-tiny-s4 | 0.774–1.486 (0.712) | 0.912–1.064 (0.152) | 1.260–1.333 |
| native / caller-buffer | varied-filter-2-tiny-s8 | 0.800–1.453 (0.653) | 0.986–1.019 (0.033) | 1.241–1.365 |
| native / caller-buffer | varied-view-2-tiny-s8 | 0.936–1.377 (0.441) | 0.979–1.055 (0.076) | 1.208–1.406 |
| native / caller-buffer | varied-filter-3-tiny-s4 | 0.867–1.279 (0.413) | 0.965–1.045 (0.080) | 0.822–0.905 |
| native / caller-buffer | varied-filter-3-resident-s4 | 0.814–1.476 (0.662) | 0.936–1.048 (0.112) | 0.826–1.003 |
| native / caller-buffer | varied-filter-3-resident-s32 | 1.001–1.276 (0.275) | 0.815–1.065 (0.250) | 0.912–1.062 |
| native / caller-buffer | varied-view-1-streaming-s8 | 0.881–1.108 (0.227) | 0.904–1.374 (0.470) | 0.919–1.452 |
| native / caller-buffer | varied-view-2-streaming-s8 | 0.948–1.127 (0.179) | 0.915–1.434 (0.519) | 0.908–1.360 |
| native / caller-buffer | varied-view-3-streaming-s12 | 0.938–1.086 (0.148) | 0.864–1.446 (0.581) | 0.999–1.166 |
| native / caller-buffer | varied-view-3-streaming-s32 | 0.845–1.450 (0.605) | 0.904–1.037 (0.133) | 0.925–1.160 |
| native / caller-buffer | meshlet-1-1-v2-t3 | 0.901–1.081 (0.180) | 0.755–1.254 (0.498) | 1.065–1.202 |
| native / caller-buffer | meshlet-1-1-v2-t4 | 0.910–1.247 (0.337) | 0.587–1.427 (0.840) | 1.139–1.305 |
| native / caller-buffer | meshlet-1-1-v4-t3 | 0.861–1.105 (0.244) | 0.752–1.371 (0.619) | 1.203–1.335 |
| native / caller-buffer | meshlet-1-1-v4-t4 | 0.985–1.061 (0.076) | 0.873–1.324 (0.450) | 1.043–1.269 |
| native / caller-buffer | meshlet-64-126-v2-t4 | 0.887–1.345 (0.459) | 0.959–1.088 (0.129) | 1.100–1.499 |
| native / caller-buffer | meshlet-64-126-v4-t4 | 0.658–1.372 (0.714) | 0.898–1.073 (0.175) | 1.183–1.358 |
| native / caller-buffer | meshlet-256-256-v2-t3 | 0.841–1.400 (0.559) | 0.885–1.069 (0.184) | 1.160–1.494 |
| native / caller-buffer | meshlet-raw-256-256 | 0.890–1.182 (0.292) | 0.909–1.304 (0.394) | 1.078–1.352 |
| wasm / allocating | vertex-v0-tiny-s4 | 0.132–1.259 (1.127) | 0.389–1.269 (0.881) | 0.345–1.433 |
| wasm / allocating | view-none-tiny-s4 | 0.962–1.324 (0.362) | 0.574–1.498 (0.924) | 0.478–1.404 |
| wasm / allocating | vertex-v0-tiny-s12 | 0.833–1.423 (0.591) | 0.955–1.114 (0.160) | 0.695–1.426 |
| wasm / allocating | view-none-tiny-s12 | 0.988–1.155 (0.167) | 0.917–1.438 (0.521) | 0.487–1.399 |
| wasm / allocating | vertex-v0-tiny-s32 | 0.904–1.259 (0.355) | 0.770–1.326 (0.556) | 0.949–1.480 |
| wasm / allocating | view-1-tiny-s4 | 0.859–1.440 (0.581) | 0.886–1.487 (0.601) | 0.460–0.915 |
| wasm / allocating | view-1-tiny-s8 | 0.877–1.303 (0.426) | 0.843–1.278 (0.435) | 0.557–1.299 |
| wasm / allocating | view-2-tiny-s8 | 0.860–1.350 (0.490) | 0.385–1.422 (1.037) | 0.181–1.271 |
| wasm / allocating | view-3-tiny-s12 | 0.815–1.150 (0.334) | 0.763–1.363 (0.599) | 0.258–1.367 |
| wasm / allocating | index-2-v0-tiny-s2 | 0.809–1.292 (0.483) | 0.705–1.224 (0.518) | 0.066–1.168 |
| wasm / allocating | index-2-v0-tiny-s4 | 0.867–1.435 (0.569) | 0.654–1.260 (0.606) | 0.491–1.383 |
| wasm / allocating | index-2-v1-tiny-s2 | 0.872–1.310 (0.438) | 0.726–1.343 (0.617) | 0.511–1.151 |
| wasm / allocating | index-2-v1-tiny-s4 | 0.783–1.467 (0.684) | 0.803–1.293 (0.490) | 0.787–1.032 |
| wasm / allocating | index-3-v0-tiny-s2 | 0.855–1.281 (0.426) | 0.848–1.478 (0.630) | 0.100–1.101 |
| wasm / allocating | index-3-v0-tiny-s4 | 0.793–1.145 (0.352) | 0.999–1.291 (0.292) | 0.821–1.415 |
| wasm / allocating | index-3-v1-tiny-s2 | 0.808–1.329 (0.521) | 0.892–1.385 (0.494) | 0.772–1.314 |
| wasm / allocating | index-3-v1-tiny-s4 | 0.806–1.171 (0.365) | 0.846–1.463 (0.617) | 0.730–0.954 |
| wasm / allocating | vertex-v0-resident-s12 | 0.873–1.033 (0.159) | 0.705–1.382 (0.677) | 0.749–1.273 |
| wasm / allocating | view-1-resident-s8 | 0.873–1.024 (0.150) | 0.720–1.308 (0.588) | 0.794–0.912 |
| wasm / allocating | view-3-resident-s12 | 0.830–1.205 (0.375) | 0.957–1.305 (0.348) | 0.832–1.041 |
| wasm / allocating | index-2-v0-resident-s4 | 0.716–1.164 (0.448) | 0.919–1.479 (0.560) | 0.637–1.320 |
| wasm / allocating | index-2-v1-resident-s4 | 0.866–1.325 (0.460) | 0.852–1.435 (0.583) | 0.889–1.276 |
| wasm / allocating | index-3-v1-resident-s2 | 0.855–1.185 (0.330) | 0.887–1.290 (0.403) | 0.744–0.928 |
| wasm / allocating | vertex-v0-streaming-s4 | 0.757–1.178 (0.420) | 0.825–1.403 (0.578) | 0.892–1.122 |
| wasm / allocating | vertex-v0-streaming-s12 | 0.778–1.441 (0.663) | 0.741–1.435 (0.694) | 0.652–1.399 |
| wasm / allocating | view-none-streaming-s12 | 0.929–1.486 (0.557) | 0.612–1.423 (0.811) | 0.789–1.393 |
| wasm / allocating | view-1-streaming-s4 | 0.906–1.474 (0.567) | 0.818–1.482 (0.665) | 0.504–1.056 |
| wasm / allocating | view-1-streaming-s8 | 0.607–1.338 (0.730) | 0.933–1.054 (0.122) | 0.748–0.958 |
| wasm / allocating | view-3-streaming-s12 | 0.550–1.297 (0.747) | 0.819–1.208 (0.389) | 0.769–1.460 |
| wasm / allocating | index-2-v0-streaming-s2 | 0.675–1.456 (0.780) | 0.722–1.445 (0.724) | 0.610–1.359 |
| wasm / allocating | index-2-v1-streaming-s2 | 0.795–1.261 (0.466) | 0.821–1.419 (0.598) | 0.656–1.408 |
| wasm / allocating | index-2-v1-streaming-s4 | 0.842–1.428 (0.586) | 0.664–1.340 (0.676) | 0.714–1.439 |
| wasm / allocating | index-3-v0-streaming-s2 | 0.646–1.355 (0.709) | 0.758–1.470 (0.712) | 0.589–1.423 |
| wasm / allocating | index-3-v1-streaming-s2 | 0.645–1.376 (0.731) | 0.618–1.435 (0.817) | 0.650–1.465 |
| wasm / allocating | vertex-v1-tiny-s4 | 0.528–1.332 (0.804) | 0.849–1.184 (0.334) | 0.358–1.377 |
| wasm / allocating | vertex-v1-tiny-s12 | 0.966–1.069 (0.103) | 0.964–1.359 (0.395) | 0.808–1.303 |
| wasm / allocating | vertex-v1-streaming-s12 | 0.706–1.492 (0.786) | 0.603–1.410 (0.807) | 0.666–1.410 |
| wasm / allocating | vertex-v1-streaming-s32 | 0.956–1.263 (0.307) | 0.905–1.257 (0.352) | 0.970–1.273 |
| wasm / allocating | vertex-v0-resident-s12-level0 | 0.878–1.412 (0.534) | 0.657–1.497 (0.841) | 0.563–1.201 |
| wasm / allocating | index-2-v0-millions-s4 | 0.694–1.443 (0.749) | 0.913–1.368 (0.455) | 0.660–1.165 |
| wasm / allocating | index-2-v1-millions-s4 | 0.649–1.305 (0.657) | 0.609–1.487 (0.878) | 0.825–1.464 |
| wasm / allocating | varied-view-1-tiny-s4 | 0.723–1.418 (0.696) | 0.859–1.480 (0.621) | 0.521–1.458 |
| wasm / allocating | varied-view-1-tiny-s8 | 0.621–1.302 (0.680) | 0.856–1.172 (0.316) | 1.070–1.094 |
| wasm / allocating | varied-view-2-tiny-s8 | 0.671–1.489 (0.818) | 0.919–1.091 (0.173) | 0.978–1.426 |
| wasm / allocating | varied-view-3-tiny-s4 | 0.786–1.158 (0.371) | 0.859–1.289 (0.430) | 0.834–1.311 |
| wasm / allocating | varied-view-3-tiny-s12 | 0.654–1.286 (0.633) | 0.902–1.242 (0.340) | 0.804–1.181 |
| wasm / allocating | varied-view-3-tiny-s32 | 0.899–1.071 (0.172) | 0.829–1.272 (0.442) | 0.860–1.198 |
| wasm / allocating | varied-view-1-streaming-s4 | 0.832–1.077 (0.245) | 0.778–1.494 (0.717) | 1.104–1.497 |
| wasm / allocating | varied-view-1-streaming-s8 | 0.642–1.392 (0.751) | 0.777–1.477 (0.700) | 0.995–1.424 |
| wasm / allocating | varied-view-2-streaming-s8 | 0.732–1.398 (0.666) | 0.652–1.458 (0.807) | 0.902–1.196 |
| wasm / allocating | varied-view-3-streaming-s4 | 0.708–1.251 (0.543) | 0.924–1.163 (0.240) | 0.731–1.088 |
| wasm / allocating | varied-view-3-streaming-s12 | 0.610–1.421 (0.811) | 0.774–1.461 (0.687) | 0.570–1.484 |
| wasm / allocating | varied-view-3-streaming-s32 | 0.915–1.480 (0.565) | 0.814–1.402 (0.589) | 0.895–1.372 |
| wasm / caller-buffer | vertex-v0-tiny-s4 | 0.989–1.011 (0.022) | 0.821–1.459 (0.638) | 0.250–1.421 |
| wasm / caller-buffer | view-none-tiny-s12 | 0.942–1.051 (0.110) | 0.410–1.455 (1.045) | 0.828–1.025 |
| wasm / caller-buffer | view-none-tiny-s32 | 0.896–1.140 (0.244) | 0.873–1.304 (0.431) | 0.449–1.385 |
| wasm / caller-buffer | view-1-tiny-s8 | 0.821–1.112 (0.291) | 0.984–1.335 (0.351) | 0.986–1.091 |
| wasm / caller-buffer | view-3-tiny-s12 | 0.902–1.117 (0.215) | 0.964–1.258 (0.294) | 0.774–1.140 |
| wasm / caller-buffer | index-2-v0-tiny-s2 | 0.767–1.219 (0.453) | 0.786–1.440 (0.654) | 0.715–1.088 |
| wasm / caller-buffer | index-2-v0-tiny-s4 | 0.475–1.388 (0.912) | 0.730–1.409 (0.680) | 0.648–1.228 |
| wasm / caller-buffer | index-3-v0-tiny-s2 | 0.795–1.107 (0.312) | 0.905–1.305 (0.400) | 0.677–0.979 |
| wasm / caller-buffer | index-3-v1-tiny-s2 | 0.815–1.361 (0.545) | 0.838–1.261 (0.423) | 0.651–0.866 |
| wasm / caller-buffer | index-3-v1-tiny-s4 | 0.942–1.138 (0.196) | 0.425–1.469 (1.044) | 0.766–0.966 |
| wasm / caller-buffer | vertex-v0-resident-s32 | 0.923–1.466 (0.543) | 0.988–1.068 (0.080) | 0.766–0.991 |
| wasm / caller-buffer | view-none-resident-s32 | 0.943–1.439 (0.495) | 0.982–1.044 (0.062) | 0.788–1.004 |
| wasm / caller-buffer | view-1-resident-s4 | 0.724–1.267 (0.543) | 0.910–1.111 (0.201) | 0.558–0.636 |
| wasm / caller-buffer | index-2-v1-resident-s4 | 0.958–1.037 (0.079) | 0.852–1.342 (0.490) | 1.061–1.207 |
| wasm / caller-buffer | index-3-v0-streaming-s2 | 0.875–1.483 (0.608) | 1.002–1.059 (0.057) | 0.800–0.920 |
| wasm / caller-buffer | vertex-v1-tiny-s4 | 0.886–1.062 (0.176) | 0.380–1.296 (0.916) | 0.293–1.388 |
| wasm / caller-buffer | varied-view-2-streaming-s8 | 0.935–1.262 (0.327) | 0.998–1.042 (0.045) | 1.119–1.153 |

## Independent S3 report

The RFC table above applies the coordinator's mean/maximum rule; S3 remains
reported independently and is not silently converted into a ratio pass.

| API | Varied filter case | Level | SIMD/scalar CI |
|---|---|---|---|
| caller-buffer | varied-filter-3-resident-s32 | rust | 1.026–1.124 |

## Diagnosis, allocation epoch and scope

The two requested streaming inputs are single ~8 MiB output calls, not many
small calls: 2,097,153 records at stride 4 and 262,145 at stride 32. N/2N user
counters find 50.161M Rust versus 48.916M C++ instructions per vertex call,
52.156M versus 47.914M per varied view. Subtracted page-fault counts are near
zero. N/2N cycle deltas do not reproduce the paired final result, so those
cycle measurements do not establish a speedup or identify the root cause.
These counters do not identify a 2.1x fixed setup gap. The retained candidate
reserves one exact-size Vec and initializes each bounded output block
immediately before reconstruction, avoiding a separate whole-output clear.
Instructions become 50.964M/52.192M (+1.6%/+0.07%); this is no instruction
speedup. Decoder arithmetic, stream checks, resource accounting and runtime
dispatch remain exact; no new unsafe block, uninitialized output or scalar
arithmetic change. Bound assembly retains the zero initialization inside
the allocating block kernels and independent caller-buffer kernels.

WASM **fix10-js-allocating** is a new measurement epoch: both public JS
adapters allocate one fresh zeroed returned Uint8Array per allocating call,
reuse WASM input/output scratch, copy source and output, and perform the
same decode/filter operation. Rust uses public decode_*_into, matching the
upstream target-buffer API. Caller-buffer keeps the JS target outside timing
on both sides. This removes the former extra Rust heap Vec allocation. It
does **not** establish speed qualification of the Vec-returning Rust API
compiled to WASM. The retained old-control module is also used through the
new symmetric adapter, so new/old compares these modules within this epoch;
historical WASM ratios are not directly comparable across epochs. A post-run
allocation-count probe verifies one returned array per allocating call and
none for caller-buffer on all four slots, with exact vertex/index/Exp-view
outputs. S3 covers the registered varied-filter scope and default/SSE2 slots.

All 138 frozen native cases (276 API rows), all 87 upstream-JS eligible cases
(174 API rows), every available family, and both allocation forms are
measured. JS exposes no standalone filters, Color, or meshlet API. Native
triangle-index is now measured; it was excluded in recent partial passes.
A/A is frozen over the entire scope independently of final failures. Only
native C++ rows reuse prior A/A, with comparator byte identity verified;
all Rust A/A and all WASM A/A are fresh.

## Correctness and run receipts

Frozen archives reproduce 869 fixtures, 7653 malformed inputs, 138 benchmark
identities on all local native ceilings and actual scalar/SIMD WASM, both
APIs; 522 exact timing-module output hashes include retained control. The
new allocation block/capacity/limit/filtered-output regression passes,
including Miri across a full block and tail. Exactness is against the retained
canonical scalar oracle: the existing upstream SIMD one-unit Oct/Quat
conformance allowance and raw Color scalar-wrap/SIMD-saturation distinction
are unchanged. No stronger upstream-SIMD bit-equality claim is made.
All-feature and unsafe-free tests, native/WASM Clippy, native/WASM MSRV/no-std
SIMD, formatting and the unchanged
23-block boundary/package inventory pass. Initial test-authoring failures
and a source-binding attempt overlapping a test edit are preserved. Native
and WASM runtime sources match compiled binaries; later non-runtime test
cfg(miri)/audit-line changes are bound in final-source-binding.json.

The first partial A/A attempt overlapped an owned CPU26 parity sweep. It
is wholly invalidated and retained as invalid-overlap-native-aa.json/log;
wrapper exit 143. No sample contributes to A/A resolution or final ratios.
The controller then added owned-proof admission exclusion before restarting
the entire preregistered A/A scope. This is not selective re-timing to pass.
No final R/C row is repeated. Initial native A/A and WASM A/A burst1 used
the legacy heavy/GPU admission. The coordinator corrected this to CPU-only:
all remaining bursts use MOSS_HEAVY_GPU=0 heavy4 timeout 840 taskset CPU26,
checkpoint 690. The next request was canceled while queued with no timing
child; no running burst was killed. Immutable controller/admission segments
bind preserved samples across this policy transition. The active entrypoint
is parity/measure-p07-fix10-cpu.py; the legacy controller stays immutable
solely to verify the earlier admissions.
Every accepted sample has before/after admission receipts and rotated slot
order. Discarded admissions and checkpoint resumptions are retained.

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07-fix10`; milestones/scout:
`/mnt/linux-extra/moss-scratch/p07-fix10`. Raw pairs, A/A widths, sources,
binary/controller hashes, counters, assembly and exit receipts are retained.
The dedicated codex-p07-fix10 target is removed after owned jobs exit;
final-receipt.json binds cleanup and SHA256.json inventories evidence.

| Platform / kind | Bursts | Completed rows | Burst command seconds |
|---|---|---:|---|
| native / aa | 3 | 552 | 691.3, 690.7, 591.4 |
| native / final | 2 | 276 | 693.3, 451.7 |
| wasm / aa | 2 | 348 | 690.8, 168.3 |
| wasm / final | 1 | 174 | 450.4 |
