# Spatial SIMD qualification

Verdict: **SHIPS**. Ratios below use median instructions:u, three samples per arm.

| Case | Default / v0.2.15 | Scalar / scalar C++ | Default / default C++ |
|---|---:|---:|---:|
| tiny-smooth | 0.999999 | 0.984282 | 0.984282 |
| medium-smooth | 0.942989 | 0.974121 | 1.082013 |
| medium-seams | 0.943000 | 0.976394 | 1.084450 |
| medium-sparse | 0.943155 | 0.977003 | 1.085411 |
| million-smooth | 0.934606 | 0.946830 | 1.064425 |
| real-214e | 0.936966 | 0.964758 | 1.082779 |
| real-a082 | 0.936691 | 0.964089 | 1.081991 |
| real-b6fd | 0.938098 | 0.966809 | 1.083547 |
| **Real-mesh geometric mean** | **0.937252** | **0.965218** | **1.082772** |

SHIPS requires real-mesh default/base <=0.95 and every case <=1.01.
The C++ wrapper uses version 1.3, with clusterizer.cpp identical to the pinned oracle.
The timed Rust driver enables std/simd and clusterlod, with diagnostic parity-internals disabled.
Scalar C++ explicitly defines MESHOPTIMIZER_NO_SIMD; default C++ enables its SSE path.
All floating-point C++ builds use -fno-fast-math -ffp-contract=off.

Wall seconds per call (disclosure only):

| Case | Base | Default | Scalar | Scalar C++ | Default C++ |
|---|---:|---:|---:|---:|---:|
| tiny-smooth | 0.000002 | 0.000003 | 0.000008 | 0.000001 | 0.000003 |
| medium-smooth | 0.003000 | 0.001333 | 0.001333 | 0.001333 | 0.001000 |
| medium-seams | 0.002000 | 0.001333 | 0.001333 | 0.002000 | 0.001333 |
| medium-sparse | 0.001333 | 0.001333 | 0.001333 | 0.002000 | 0.003000 |
| million-smooth | 0.753952 | 0.680957 | 0.796949 | 0.731954 | 0.598962 |
| real-214e | 5.855628 | 4.631706 | 6.318598 | 5.671639 | 3.831757 |
| real-a082 | 5.454652 | 5.055678 | 5.708636 | 5.934622 | 3.824756 |
| real-b6fd | 1.074931 | 0.934940 | 0.677957 | 0.655958 | 0.389975 |

Performance: 120 samples, CPU 0; every counter reports 100% running.
Every measured arm and repetition has the same output hash within each case.

Exact limits: 179,229 no-feature cells and 716,916 SIMD/forced-ceiling cells.
Each cell compares base/candidate owned and caller-buffer APIs twice, with complete errors, Usage and sentinels.
Cold, successful-prime and invalid-index-prime states; 13 inputs and four settings.
Both exact runs exit 0; execution CPU totals are in exits.jsonl and are below two hours.

C++/native/wasm differential: 2028 corpus/boundary messages plus three full real meshes.
Native forced Scalar/SSE2/SSSE3/SSE4.1, no-feature Rust, scalar/default C++, and scalar/SIMD wasm all agree.
Both wasm artifacts execute in Node. Three focused direct SIMD kernel tests also execute under Node WASI.
NEON compiles for aarch64-unknown-linux-gnu; no native/user-mode runner is installed, so execution is unverified.
No new sanitizer or Miri run was performed; finite tests supplement the structural proof.

Full tests (including doctests), all-features and no-default-features: exit 0. Clippy, fmt, repository/package boundary,
public-text, AArch64 check and -Funsafe_code no-feature check: exit 0. See receipts and exits.jsonl.
Unsafe inventory: 23 -> 28 blocks (+5; the wasm entry needs no unsafe block), one allowance. No non-SIMD unsafe code.

The first frozen-candidate build exited 101 because the harness omitted included documentation.
The harness copy was corrected. A later real-mesh differential exceeded the inherited C++ protocol 128 MiB cap.
Only scratch transport limits were widened to 512 MiB for all protocol arms; algorithms are unchanged.
That failed stage, original corpus and original C++ binaries are retained. The corrected full differential passed.

The final crate doc comment corrects one comma after the frozen build; executable tokens are unchanged, and both source hashes are retained.
Source, binary, input, output, raw-log and sanitized-log hashes are retained in the binding JSON files.
The complete raw artifacts and executables remain under the caller-supplied scratch directory; only the dedicated
Cargo target is removed at completion. Git commits, refs and index are not intentionally modified.

Reproduction: set MESHOPT_SCRATCH, CARGO_TARGET_DIR, MESHOPT_REFERENCE, MESHOPT_INPUTS (a directory containing
inputs/ and inputs.json), and PEERS_PERF; run run.py stages prepare, checks, build, exact, p03build, differential,
and measure sequentially through the prescribed CPU admission wrapper. Run retain.py after completion.
PROOFS.md contains the arithmetic, memory and failure-state argument.
