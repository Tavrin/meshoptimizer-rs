# Decoder qualification results

AVX-512 VBMI/VBMI2 vertex decoding is selected at runtime on supported x86-64 hardware. The implementation uses the same 128-bit multishift/expand operations as pinned upstream SIMD_AVX; the CPU supports the full required feature set. No emulation was needed and no native speed claim is made. AVX2 is a diagnostic ceiling using the existing SSSE3 codec kernels. Upstream has no AVX-512 index/filter decoder to port.

The wasm implementation now matches header codes directly to constant-width simd128 group kernels, allowing shifts and lookahead bounds to fold at compile time. All slices remain checked; it adds no unsafe code. An initial no/all-escape shortcut was rejected. The group census found 7,581,796 mixed groups out of 7,914,590 packed groups (95.8%). Its short-batch timing screen had zero/quantized samples and is retained as a rejected screen, not an acceptance result.

Final timing uses Node v22.22.0 and the pinned upstream shipped C++ wasm decoder, equal source/output copies and allocating output. Batches target 150 ms for the fastest arm; the shortest retained batch was 108.868 ms. Two warmup rounds precede seven rotated rounds per case. Times below are wall medians in milliseconds per call. Geometric means use the eight per-case median ratios for each operation.

| Decoder | Before / C++ | After / C++ | After / before |
|---|---:|---:|---:|
| vertex | 1.1839 | 1.1104 | 0.9379 |
| index | 1.0401 | 1.0618 | 1.0208 |

Vertex time fell by 6.2% geometrically in these measurements, narrowing its gap to C++. Index time rose by 2.1% geometrically; its gap remains open. Worst individual vertex after/before ratio was 1.0722 (tiny case); worst index ratio was 1.1634. These shared-host wall measurements do not establish a universal speed bound or an index speedup.

| Case | Decoder | Before ms | After ms | C++ ms |
|---|---|---:|---:|---:|
| tiny-smooth | vertex | 0.000798395 | 0.000856032 | 0.000695988 |
| tiny-smooth | index | 0.000609144 | 0.000621069 | 0.000586271 |
| medium-smooth | vertex | 0.0167785 | 0.0159022 | 0.0143169 |
| medium-smooth | index | 0.0517351 | 0.0511921 | 0.0453787 |
| medium-seams | vertex | 0.0271328 | 0.02083 | 0.0203301 |
| medium-seams | index | 0.0505999 | 0.0529337 | 0.0488472 |
| medium-sparse | vertex | 0.0262469 | 0.0262567 | 0.0233284 |
| medium-sparse | index | 0.0528955 | 0.0564935 | 0.0535265 |
| million-smooth | vertex | 2.31842 | 2.1281 | 2.15621 |
| million-smooth | index | 6.46229 | 6.51037 | 5.24934 |
| real-214e | vertex | 112.614 | 104.665 | 88.9487 |
| real-214e | index | 63.248 | 56.4726 | 51.4408 |
| real-a082 | vertex | 141.447 | 143.091 | 103.535 |
| real-a082 | index | 86.3242 | 86.3043 | 97.6976 |
| real-b6fd | vertex | 17.0636 | 15.1669 | 16.5979 |
| real-b6fd | index | 9.31888 | 10.8415 | 11.034 |

Identity: the final-source native differential passed 318,608 cases across forced scalar/SSE2/SSSE3/SSE4.1/AVX2/AVX-512, against C++ scalar/default/AVX2/AVX-512 builds. Rust statuses and full destination bytes matched across ceilings, and all successful C++ outputs matched. There were 149,727 C++ failure-prefix write differences across reference arms; these are disclosed inherited staging differences, not successful-output mismatches.

The broader pinned/generated/truncation corpus passed 4,430 cases per Rust ceiling and executed wasm, including index, sequence, vertex, filters, views and meshlets, with allocating/caller-buffer comparisons. A further 316,608 executed wasm/scalar observations matched complete results, partial writes, tails and workspace usage. These are finite qualification results, not a proof for every possible input.

Final gates passed: fmt, native and wasm32 simd128 clippy with warnings denied (all targets/all features), tests, no-default tests, doctests, docs with warnings denied, Rust 1.88, semver against 0.2.16 (202 applicable checks), pinned reference checks, exact corpus/differential assertions, package/unsafe boundary and current safety inventory (40 blocks). Initial native clippy exit 101 is retained in exits.json; corrected native-final-clippy and subsequent wasm clippy exit 0, without lint relaxation. All steps in wasm-exits.json and refine-exits.json exit 0. Final public-text/diff/cleanup receipts are separate.

No Cargo feature or dependency was added. Git history, branch and index remain operator-owned; only files were edited. The fresh build target is removed after evidence binding. See receipts/source-binding.json, summary.json, wasm.json, identity.json, differential.json and exit ledgers.
