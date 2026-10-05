# P07 A/A noise and allocation audit

Measurement only, source HEAD `795e6a3b4496b1732c910bba5aa9f28a42f54c39`. No library or qualification-harness changes. Retained fix9 binaries and source hashes were verified before and after measurement.

Selection was frozen before timing: every fix9 raw final upper bound >=1.40, every case receiving fresh stage 2, the report's allocating S3 failure, and six additional passing controls. This deliberately includes more borderline cases than the report names. A/C mean allocating/caller-buffer; N/W mean native/Node WASM. “S3” is a Rust/scalar failure, not a Rust/C++ maximum failure. Prior column is always the fix9 Rust/C++ interval.

Same native binary against itself in independent persistent processes; same WASM binary/public function against itself in the retained Node harness. Only the two comparison slots are made identical: Rust A/A replaces the C++ slot with Rust; C++ A/A replaces the Rust slot with C++. Scalar, SSE2 where applicable, and old-control slots remain, with original slot rotation, warmup and timed boundaries. Each side has persistent processes across rows. Calibration uses the substituted denominator, targeting 4 ms, capped at native 1,000,000 / WASM 10,000 iterations. As in qualification: 5–20 pairs, paired two-sided 95% Student-t log-ratio intervals, stop when upper <=1.50 or lower >1.50, otherwise 30 fresh pairs replace stage 1. No repeat-to-pass. These nominal sequential intervals inherit qualification's stopping policy; they are not simultaneous or sequentially adjusted confidence bounds.

Resolution rule registered before timing: **both A/A upper bounds <=1.25**, leaving 0.25 headroom below 1.50. “No” means this run does not establish that resolution; it does not prove that the underlying slowdown is absent. Median times pool the two identical slots from stage 1. CI width Δ is upper minus lower in ratio units. Times and allocation-only medians are µs/call. Allocation probes use 20 alternating pairs at the same 4 ms target, include allocation/initialization/replacement/free plus wrapper overhead, and exclude decoding, validation and WASM source/output transfer. They are separate empty-call binaries, not modified qualification binaries. Native filter probes copy the source to the fresh output, matching both real harnesses; other native probes zero-fill it. WASM probes measure Rust reserve/zero/replacement plus JS output allocation versus upstream C++ output sbrk/rewind plus JS output allocation. They exclude C++ input-region reservation as it is not output allocation. Do not subtract these isolated costs from full-call timings as if effects were additive.

| Platform/API | Case (selection) | Prior R/C CI | Rust µs | Rust A/A CI (Δ) | C++ µs | C++ A/A CI (Δ) | Alloc-only R/C µs | Resolvable |
|---|---|---|---:|---|---:|---|---|---|
| N/A | vertex-v0-tiny-s4 (control) | 1.147–1.247 | 0.178 | 0.912–1.095 (0.183) | 0.153 | 0.857–1.075 (0.218) | 0.027/0.019 | yes |
| N/A | view-none-tiny-s4 (near bar) | 0.796–1.427 | 0.183 | 0.963–1.073 (0.110) | 0.150 | 0.914–1.073 (0.159) | 0.027/0.019 | yes |
| N/A | vertex-v0-tiny-s12 (near bar) | 0.852–1.488 | 0.505 | 0.862–1.106 (0.243) | 0.356 | 0.948–1.160 (0.211) | 0.031/0.020 | yes |
| N/A | view-none-tiny-s12 (near bar) | 1.351–1.452 | 0.471 | 0.960–1.089 (0.128) | 0.379 | 0.879–1.173 (0.294) | 0.031/0.020 | yes |
| N/A | vertex-v0-tiny-s32 (near bar) | 0.604–1.463 | 1.106 | 0.666–1.246 (0.580) | 0.890 | 0.917–1.031 (0.113) | 0.020/0.013 | yes |
| N/A | view-none-tiny-s32 (near bar) | 0.813–1.420 | 1.045 | 0.956–1.066 (0.110) | 0.880 | 0.747–1.212 (0.465) | 0.020/0.013 | yes |
| N/A | view-2-tiny-s8 (near bar) | 1.206–1.470 | 0.398 | 0.948–1.146 (0.199) | 0.314 | 0.920–1.214 (0.295) | 0.016/0.013 | yes |
| N/A | view-3-tiny-s12 (stage2) | 1.041–1.295 | 0.521 | 0.882–1.198 (0.316) | 0.400 | 0.876–1.037 (0.161) | 0.031/0.020 | yes |
| N/A | vertex-v0-resident-s12 (near bar) | 0.782–1.417 | 35.685 | 0.811–1.087 (0.277) | 28.698 | 0.893–1.106 (0.213) | 0.603/0.582 | yes |
| N/A | view-none-resident-s12 (near bar) | 0.862–1.436 | 35.028 | 0.847–1.073 (0.225) | 31.300 | 0.822–1.038 (0.216) | 0.603/0.582 | yes |
| N/A | vertex-v0-streaming-s4 (control) | 0.739–1.198 | 11664.502 | 0.891–1.273 (0.382) | 12709.042 | 0.936–1.061 (0.126) | 3899.078/3895.095 | **no** |
| N/A | view-1-streaming-s4 (near bar) | 1.369–1.476 | 12473.243 | 0.867–1.261 (0.394) | 13646.454 | 0.759–1.023 (0.264) | 3899.078/3895.095 | **no** |
| N/A | vertex-v1-tiny-s12 (near bar) | 1.000–1.499 | 0.520 | 0.756–1.146 (0.390) | 0.362 | 0.876–1.136 (0.261) | 0.031/0.020 | yes |
| N/A | vertex-v1-resident-s4 (near bar) | 0.617–1.439 | 11.158 | 0.690–1.224 (0.534) | 10.761 | 0.871–1.179 (0.309) | 0.155/0.150 | yes |
| N/A | vertex-v1-resident-s12 (near bar) | 0.657–1.405 | 36.288 | 0.905–1.229 (0.324) | 31.338 | 0.885–1.191 (0.306) | 0.603/0.582 | yes |
| N/A | vertex-v1-streaming-s4 (failed maximum) | 1.850–2.103 | 11419.816 | 0.968–1.155 (0.187) | 11781.031 | 0.921–1.039 (0.118) | 3899.078/3895.095 | yes |
| N/A | vertex-v1-resident-s12-level9 (near bar) | 0.971–1.474 | 33.146 | 0.930–1.116 (0.186) | 28.840 | 0.908–1.071 (0.162) | 0.603/0.582 | yes |
| N/A | varied-filter-3-tiny-s12 (S3 failure) | 0.879–1.040 | 0.051 | 0.850–1.143 (0.293) | 0.065 | 0.990–1.100 (0.110) | 0.018/0.012 | yes |
| N/A | varied-filter-3-tiny-s32 (control) | 0.822–0.996 | 0.076 | 0.912–1.099 (0.186) | 0.098 | 0.796–1.032 (0.236) | 0.017/0.012 | yes |
| N/A | varied-filter-3-streaming-s32 (near bar) | 0.527–1.428 | 7545.936 | 0.891–1.069 (0.177) | 7455.386 | 0.923–1.068 (0.145) | 4156.778/4169.462 | yes |
| N/A | varied-view-3-streaming-s32 (failed maximum) | 1.761–2.122 | 11756.595 | 0.751–1.075 (0.324) | 11106.608 | 0.919–1.026 (0.108) | 3950.429/4016.699 | yes |
| N/C | vertex-v0-tiny-s12 (near bar) | 1.082–1.467 | 0.338 | 0.731–1.137 (0.406) | 0.310 | 0.764–1.067 (0.304) | — | yes |
| N/C | vertex-v0-streaming-s12 (near bar) | 0.612–1.437 | 15533.212 | 0.886–1.114 (0.228) | 13468.149 | 0.803–1.053 (0.250) | — | yes |
| N/C | view-none-streaming-s12 (near bar) | 0.809–1.438 | 8411.704 | 0.725–1.109 (0.384) | 9435.330 | 0.604–1.113 (0.508) | — | yes |
| N/C | vertex-v1-tiny-s4 (near bar) | 1.081–1.448 | 0.081 | 0.950–0.996 (0.045) | 0.073 | 0.887–1.043 (0.157) | — | yes |
| N/C | varied-filter-1-tiny-s4 (near bar) | 1.251–1.405 | 0.064 | 0.991–1.018 (0.028) | 0.045 | 0.912–1.064 (0.152) | — | yes |
| N/C | varied-filter-1-resident-s4 (near bar) | 1.292–1.445 | 7.333 | 0.970–1.064 (0.095) | 5.499 | 0.968–1.099 (0.132) | — | yes |
| N/C | varied-filter-1-streaming-s4 (near bar) | 1.162–1.475 | 507.908 | 0.921–1.065 (0.143) | 344.202 | 0.897–1.067 (0.170) | — | yes |
| N/C | varied-view-1-streaming-s4 (near bar) | 0.921–1.436 | 628.256 | 0.916–1.184 (0.268) | 512.721 | 0.975–1.120 (0.145) | — | yes |
| N/C | varied-filter-1-streaming-s8 (near bar) | 0.923–1.406 | 529.183 | 0.953–1.142 (0.189) | 426.334 | 0.952–1.146 (0.195) | — | yes |
| N/C | varied-view-1-streaming-s8 (near bar) | 1.003–1.453 | 912.000 | 0.932–1.046 (0.114) | 782.434 | 0.904–1.374 (0.470) | — | **no** |
| N/C | varied-view-3-streaming-s32 (near bar) | 0.746–1.442 | 2483.726 | 0.708–1.128 (0.420) | 2477.409 | 0.904–1.037 (0.133) | — | yes |
| W/A | vertex-v0-tiny-s4 (failed maximum) | 0.754–1.511 | 0.577 | 0.828–1.226 (0.398) | 0.641 | 0.891–1.476 (0.585) | 0.234/0.257 | **no** |
| W/A | vertex-v0-tiny-s12 (near bar) | 0.363–1.490 | 0.916 | 0.872–1.263 (0.391) | 0.769 | 0.773–1.250 (0.477) | 0.279/0.258 | **no** |
| W/A | view-none-tiny-s12 (failed maximum) | 1.244–2.475 | 0.914 | 0.855–1.135 (0.279) | 0.718 | 0.514–1.393 (0.880) | 0.279/0.258 | **no** |
| W/A | view-none-tiny-s32 (control) | 0.721–0.866 | 1.211 | 0.802–1.452 (0.649) | 1.109 | 0.855–1.230 (0.375) | 0.398/0.218 | **no** |
| W/A | view-1-tiny-s8 (near bar) | 0.630–1.458 | 0.711 | 0.946–1.189 (0.243) | 0.684 | 0.431–1.302 (0.871) | 0.247/0.243 | **no** |
| W/A | vertex-v0-resident-s32 (control) | 0.725–1.097 | 89.589 | 0.780–1.209 (0.428) | 93.524 | 0.752–1.166 (0.415) | 7.794/6.091 | yes |
| W/A | view-2-resident-s8 (near bar) | 0.471–1.449 | 11.376 | 0.984–1.294 (0.310) | 15.714 | 0.909–1.261 (0.352) | 2.426/2.060 | **no** |
| W/A | vertex-v0-streaming-s12 (near bar) | 0.649–1.469 | 19880.684 | 0.700–1.476 (0.775) | 20116.006 | 0.769–1.390 (0.621) | 2892.576/1844.397 | **no** |
| W/A | view-none-streaming-s12 (near bar) | 0.815–1.435 | 19402.029 | 0.605–1.326 (0.721) | 20214.962 | 0.831–1.419 (0.589) | 2892.576/1844.397 | **no** |
| W/A | view-none-streaming-s32 (near bar) | 0.746–1.419 | 94924.821 | 0.934–1.443 (0.509) | 103340.508 | 0.847–1.158 (0.311) | 2934.912/1444.750 | **no** |
| W/A | view-1-streaming-s4 (near bar) | 0.462–1.433 | 4341.985 | 0.732–1.300 (0.568) | 5245.586 | 0.732–1.497 (0.765) | 375.898/191.216 | **no** |
| W/A | vertex-v1-tiny-s4 (near bar) | 0.467–1.406 | 0.473 | 0.617–1.398 (0.781) | 0.773 | 0.851–1.272 (0.421) | 0.234/0.257 | **no** |
| W/A | vertex-v1-tiny-s12 (near bar) | 0.803–1.428 | 0.631 | 0.894–1.032 (0.139) | 0.758 | 0.613–1.199 (0.586) | 0.279/0.258 | yes |
| W/A | vertex-v1-streaming-s12 (near bar) | 0.552–1.448 | 24071.635 | 0.627–1.449 (0.821) | 18267.194 | 0.852–1.111 (0.260) | 2892.576/1844.397 | **no** |
| W/A | varied-view-1-tiny-s4 (near bar) | 0.466–1.415 | 0.579 | 0.980–1.099 (0.119) | 0.464 | 0.868–1.498 (0.631) | 0.234/0.257 | **no** |
| W/A | varied-view-1-resident-s4 (near bar) | 1.256–1.477 | 14.937 | 0.853–1.483 (0.631) | 15.059 | 0.826–1.118 (0.292) | 1.260/1.078 | **no** |
| W/A | varied-view-3-resident-s32 (control) | 0.781–0.927 | 67.053 | 0.871–1.070 (0.199) | 74.773 | 0.904–1.130 (0.226) | 7.794/6.091 | yes |
| W/A | varied-view-1-streaming-s4 (near bar) | 1.290–1.488 | 1176.703 | 0.854–1.128 (0.274) | 722.773 | 0.919–1.123 (0.204) | 44.676/35.680 | yes |
| W/A | varied-view-1-streaming-s8 (near bar) | 0.931–1.471 | 1574.957 | 0.712–1.232 (0.520) | 1279.933 | 0.944–1.078 (0.134) | 123.085/80.900 | yes |
| W/A | varied-view-2-streaming-s8 (near bar) | 0.919–1.484 | 1520.595 | 0.635–1.406 (0.771) | 1389.570 | 0.760–1.488 (0.728) | 123.085/80.900 | **no** |
| W/A | varied-view-3-streaming-s12 (near bar) | 0.460–1.428 | 2307.081 | 0.741–1.490 (0.749) | 2159.506 | 0.645–1.447 (0.802) | 173.985/114.759 | **no** |
| W/C | vertex-v0-tiny-s4 (near bar) | 0.465–1.442 | 0.214 | 0.934–1.314 (0.380) | 0.253 | 0.761–1.482 (0.721) | — | **no** |
| W/C | view-none-tiny-s12 (near bar) | 0.655–1.412 | 0.346 | 0.945–1.209 (0.265) | 0.367 | 0.981–1.021 (0.039) | — | yes |
| W/C | view-none-tiny-s32 (near bar) | 1.038–1.485 | 0.741 | 0.882–1.039 (0.157) | 1.117 | 0.923–1.166 (0.243) | — | yes |
| W/C | varied-view-1-streaming-s4 (near bar) | 1.302–1.490 | 985.646 | 0.928–1.043 (0.115) | 757.449 | 0.868–1.043 (0.176) | — | yes |
| W/C | varied-view-1-streaming-s8 (near bar) | 1.101–1.498 | 1446.430 | 0.983–1.030 (0.047) | 1109.751 | 0.984–1.017 (0.032) | — | yes |

**Conclusion:** 38/58 rows resolve the bar; 20 do not. Both native failed maxima (`vertex-v1-streaming-s4`, `varied-view-3-streaming-s32`) resolve it; both WASM failed maxima (`vertex-v0-tiny-s4`, `view-none-tiny-s12`) do not. These are diagnostics of the prior failures, not a fresh Rust/C++ qualification. The two native historical intervals have lower bounds above 1.50; where A/A resolves the bar, they support a real end-to-end harness gap rather than an inability to measure that size. A/A cannot identify the fraction attributable to decoder code versus allocator implementation. The WASM historical intervals straddle 1.50 and cannot establish a real >1.50 slowdown by themselves. The S3 case resolves measurement size, but its Rust/scalar slowdown remains unadjudicated: SIMD A/A does not compare against scalar.

Native output allocation is structurally symmetric: fresh exact-size output, zero initialization for vertex/view, source-copy initialization for standalone filters, previous output released on replacement. Different Rust/C++ allocator implementations and emitted initialization code can still have different costs. Native allocating-only failures are not evidence of an unequal allocation count.

**WASM harness fairness defect:** Rust's allocating API creates and zero-initializes a fresh `count*stride` WASM heap Vec on each call, replaces/frees the previous Vec, and the JS harness additionally creates a fresh `Uint8Array`. C++ creates that JS array too, but its WASM output is a reusable bump region of `((count+3)&~3)*stride` bytes, obtained with `sbrk` and immediately rewound, without per-call output zero-fill/free-list allocation. Its input scratch is similarly bump-reserved and copied; Rust's input is prepared/reused outside timing and copied each call. The caller-buffer API keeps Rust output allocation outside timing, while the upstream wrapper still bump-reserves its output. Thus all allocating WASM rows, including both failed maxima, are confounded by asymmetric WASM allocation. At ~64 MiB, the isolated allocation medians are 2934.912 µs Rust versus 1444.750 µs C++. This audit establishes the defect, not that it alone caused either failure. No harness repair or performance rescue was attempted. Sources: `src/codec/mod.rs:301`, `parity/codec/src/lib.rs:78`, `parity/codec/reference.cpp:170`, `parity/simd/src/lib.rs:283`, `parity/wasm-p07-fix7-bench.mjs:15`, upstream `js/meshopt_decoder.mjs:48`.

Evidence: `/mnt/linux-extra/meshopt-artifacts/p07-aa` contains frozen selection, exact binary/source identities, controllers and allocation-probe source/binaries, all raw pairs/admission telemetry, burst logs/exits and final SHA-256 inventory. CPU 26, shared heavy/GPU admission, 4 GiB declaration, each command inside `timeout 840`; timing bursts checkpoint between rows at 690 seconds. Milestones: `/mnt/linux-extra/moss-scratch/p07-aa/MILESTONES.md`. The dedicated `/mnt/linux-extra/moss-cargo-targets/codex-p07-aa` target is removed after owned processes exit; retained probe binaries live only in the artifact directory. No push, release, other-platform, or integration claim.
