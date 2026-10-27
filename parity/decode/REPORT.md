# registered protocol decode: PARTIAL

Cause: native filters pay for software `libm::sqrtf` (its architecture backend is disabled), repeated stride/bounds checks, memoization branches and safe float-to-int conversion. Those loop dependencies inhibit auto-vectorization. Scalar WASM pays for four strided reconstruction walks and a literal bounds check for each escape. An instructions-sampled WASM profile attributes 59.33% to byte-delta reconstruction and 37.84% to byte-group parsing.

Baseline: `bea3e04bf22063988a80db4764e80c3dd422a634` (v0.2.5), existing `perf/scalar-decode` worktree. Final scalar code uses the safe std square root only in filters, fixed-size records and ordinary Rust arrays for compiler vectorization, exact bounded truncation with canonical fallbacks, one four-plane byte-delta walk, and a proven 16-literal window. Common scalar WASM strides expose static destination bounds. no_std keeps the pinned software root. No explicit SIMD, unsafe scalar code, encoder, index/sequence algorithm or public-contract changes.

## Instruction evidence

Fixed 100 calls minus a zero-call process count, `instructions:u`, CPU 2, bet-0 a downstream engine scalar allocating/copy/drop boundaries. These are diagnostic counts, not wall-clock acceptance. Counts are instructions per call.

| Case | Family | v0.2.5 | Final | M7 | Reduction |
|---|---|---:|---:|---:|---:|
| medium-smooth | oct-decode | 955,529 | 233,629 | 155,316 | 75.55% |
| medium-smooth | quat-decode | 953,389 | 479,112 | 334,300 | 49.75% |
| medium-seams | oct-decode | 1,172,395 | 286,335 | 190,651 | 75.58% |
| medium-seams | quat-decode | 1,169,495 | 587,693 | 410,202 | 49.75% |
| medium-sparse | oct-decode | 1,910,526 | 466,492 | 310,105 | 75.58% |
| medium-sparse | quat-decode | 1,906,048 | 957,607 | 668,221 | 49.76% |

The root-only ablation reduced native instructions by approximately 39% (Oct) and 45% (Quat). Further scalar layouts/conversions were selected only by instruction counts. Rejected saturated Oct conversion and single-record Quat layouts remain in scratch; no timing-based selection occurred.

| Frozen stream | v0.2.5 WASM | Final WASM | Upstream scalar WASM | Final / upstream |
|---|---:|---:|---:|---:|
| 0 (vertex) | 132,754,898 | 84,679,363 | 80,125,159 | 1.057 |
| 1 (index) | 4,594,417 | 4,484,705 | 4,297,380 | 1.044 |
| 10 (vertex) | 25,845,483 | 17,062,879 | 15,755,186 | 1.083 |
| 11 (index) | 1,348,035 | 1,259,446 | 1,195,972 | 1.053 |

WASM instruction probes use Node/V8 fixed-call adapters with matched copies; they include startup/JIT effects after zero-call subtraction and are diagnostic only. Chromium establishes the browser verdict. Instruction profiles and before/after source/binary hashes are retained.

## Registered native result

One final campaign, same-session A/A first, five frozen bet-0 inputs, a downstream engine scalar profile, CPU 2. Five alternating pairs extend once to twenty only when the five-pair nominal paired log Student-t interval straddles equality. No native timing rerun. Symbols were retained for instruction profiling; the downstream engine optimization settings are unchanged. Post-strip checks verify identical .text, .rodata, .data, .eh_frame and program headers for all three measured binaries (`STRIP-EQUIVALENCE.json`).

| Family | ours / comparator GM | Bar | Verdict |
|---|---:|---:|---|
| oct-decode / M7 | 1.437751 | <=1.00 | PARTIAL |
| quat-decode / M7 | 1.654149 | <=1.00 | PARTIAL |
| vertex-decode / v0.2.5 | 0.910860 | <=1.046964 | PASS |

A/A family GMs: Oct 1.026964, Quat 1.007457, vertex 1.014664. F=0.026964; old-version bar=1.02+F=1.046964. Worst individual A/A case is 1.164921; the shared host is noisy. The instruction reduction does not establish a wall-time win over M7.

| Case | Family | ours / M7 | nominal paired 95% CI |
|---|---|---:|---|
| tiny-smooth | oct-decode | 1.259436 | [1.144823, 1.385525] |
| tiny-smooth | quat-decode | 1.524798 | [1.283119, 1.811997] |
| medium-smooth | oct-decode | 1.390817 | [1.326856, 1.457862] |
| medium-smooth | quat-decode | 1.604161 | [1.102154, 2.334822] |
| medium-seams | oct-decode | 1.390627 | [1.280701, 1.509988] |
| medium-seams | quat-decode | 2.148872 | [1.917612, 2.408022] |
| medium-sparse | oct-decode | 1.564476 | [1.405551, 1.741370] |
| medium-sparse | quat-decode | 1.244973 | [1.212750, 1.278052] |
| million-smooth | oct-decode | 1.612103 | [1.559300, 1.666693] |
| million-smooth | quat-decode | 1.892529 | [1.681379, 2.130196] |

## Registered Chromium result

Chrome/155.0.8059.84, headless, CPU 2, GPU/audio disabled. Exact REPORT-3 42-stream inventory and encoded bytes. Rust and shipped upstream JS adapters both time copy-in and copy-out. One warm-up, exactly 15 rotating calls, `performance.now`, original `wasm-opt -Oz` policy. Timer resolution: 0.100 ms. All 42 streams match across all seven arms; none are excluded from acceptance.

| Comparison | All-42 GM | Descriptive positive-case GM | Fully timed cases | Worst measured upper95 | Verdict |
|---|---|---:|---:|---:|---|
| after-scalar/upstream-scalar | unqualified | 1.229573 | 26/42 | 1.894426 | PARTIAL |
| after-scalar/before-scalar | unqualified | 0.867571 | 27/42 | 1.709094 | PARTIAL |
| after-simd/before-simd | unqualified | 0.999263 | 26/42 | 1.694225 | PARTIAL |

A case with any nonpositive timing observation cannot form the registered paired log interval. Those cases remain unqualified, rather than being excluded to create a passing GM. Positive-case summaries are descriptive, use different subsets, and are not all-42 acceptance results. Scalar/upstream requires GM <=1 and every case upper95 <=1.10; both remain unmet/unproven. SIMD aggregate is near baseline, but its per-case no-regression gate is unproven, including underresolved cases and measured upper95 values above 1.10. Native SIMD output remains byte-identical to v0.2.5; no claim of browser SIMD performance acceptance is made.

The first browser timing attempt failed with CDP `-32000: Execution context was destroyed` during initial navigation and returned no result file. The single allowed infrastructure retry waited for a stable loaded document before reading the same measurement promise. It changed no decode code, timer, call count, transport, module or input. Native results were retained and not repeated. Both attempt logs and the retry-driver hash/change record are retained. No further retries or implementation changes after observing verdicts.

## Targeted proof

| Gate | Exit |
|---|---:|
| fmt | 0 |
| test | 0 |
| clippy | 0 |
| scalar-nostd | 0 |
| scalar-std | 0 |
| scalar-tests | 0 |
| simd-protocol | 0 |
| simd-baseline | 0 |
| oracle-check | 0 |
| oracle | 0 |
| differential | 0 |
| wasm scalar/SIMD build and wasm-opt | 0 |
| Chromium 42-stream byte check | 0 |
| browser infrastructure retry | 0 |

The C++ 1.3 oracle is the unmodified `4c203430ca565cb59a468a91922c76c208169536` checkout. The targeted differential passes 853 cases / 90,661 records with zero mismatches: std/no_std scalar allocating and into APIs, both vertex versions, block/layout edges, truncations, filter precision/sign/integer extremes, all Quat output rotations, unchanged native SIMD baseline bytes, and canonical encoded-filter SIMD +/-1 conformance. All-features tests pass 195 native tests plus 11 doctests (all result groups retained). Final source, binaries, native inputs, browser adapters and modules are hash-bound in `MEASUREMENT-IDENTITY.json`.

Pre-sample setup failures are retained separately: standalone harness lockfile refresh, missing current-kernel perf launcher (the installed 6.17 perf executable worked), one type-inference compile error, and a caught/rebuilt SIMD flag-precedence issue. The initial registration draft proposed timer batching; it was corrected before any wall sample to honor exactly 15 calls. These did not produce selected wall-time results. During the post-measurement strip-equivalence audit, objcopy rewrote native ELF metadata; all three binaries were deterministically rebuilt and restored only after matching their original measured full-file SHA256 values exactly (`BINARY-RECOVERY.json`). No probes or timings were repeated for recovery. Final targeted proof gates above all exit 0.

## Decision and retained boundary

Overall **PARTIAL**. Keep the byte-exact scalar improvements in a draft PR for review, with no at-least-M7/upstream claim. No SIMD expansion or contract weakening. The registered wall campaign is terminal: neither another tuning round nor a verdict-driven rerun is authorized by this result. Long fuzzing/full sweeps remain nightly work. No merge, tag, publish, a downstream engine adoption, decoder-size acceptance, other-host or GPU claim.

Full evidence: `<scratch>/meshopt-decode/`. Compact analysis, timing rows, differential summary, registration and hashes are in this directory. The designated target is deleted after executable retention; see `CLEANUP.json`. `RESUME-2026-10-24.md` is updated under `0.2.6 decode`.
