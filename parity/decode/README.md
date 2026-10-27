# registered protocol decode evidence

`REGISTRATION.md` is the applied, pre-sample policy. `REPORT.md` and `ANALYSIS.json`
contain the terminal verdicts. This job changes safe scalar filter/reconstruction
code; it does not add ISA intrinsics, change codec contracts, or touch encoders.

Full receipts, binaries, optimized assembly, instruction profiles, frozen source,
inputs and Chromium results remain in
`<scratch>/meshopt-decode/`. `EVIDENCE-SHA256SUMS.json` binds
retained evidence. The designated Cargo target is removed after retention.

The native driver is copied from `bench/peer-screen`'s `parity/compare/src/main.rs`.
The only driver extension is a fixed-call command-line branch with no clock read.
Its normal resident timing path, allocating API/copy/drop boundaries and input
bytes are unchanged. `native.py` uses that path for A/A, M7 and old-version pairs.
`instructions.py` subtracts a zero-call process count from 100 calls. Raw values,
commands, hashes and failed perf-launch receipts are retained.

The REPORT-3 browser adapter remains allocating Rust versus shipped upstream
scalar WASM called through its JS adapter. `harness.mjs` adds Rust copy-in and
copy-out inside every call to match upstream's timed transport. All seven arms
load on one page. `drive.mjs` uses headless Chromium on CPU 2, with no GPU/audio.
There is one warm-up and exactly 15 rotating calls; no timer batches. Zero samples
remain unqualified for the case upper95 gate. Frozen inventory/bytes are copied
from REPORT-3, never re-extracted or resampled. Both scalar and SIMD modules are
built explicitly and optimized with the original `wasm-opt -Oz` policy.

`prepare.py` reconstructs a fresh scratch setup from pinned v0.2.5, the retained
peer-screen harness and REPORT-3 assets. It refuses existing scratch. The build,
proof, bytecheck and final measurement recipes are retained here for inspection;
each is invoked with `PARITY_RUNNER_GPU=0`, `PARITY_JOB=meshopt-decode`,
`CARGO_TARGET_DIR=<target-dir>` and
`<job-runner> 4 bash <recipe>`. Measurement also
sets `PARITY_RUNNER_MEASUREMENT=1`. Completed measurements must not be overwritten or
rerun to change a verdict. Historical iteration recipes and intermediate source
snapshots remain in scratch, outside the final implementation.

`differential.py` checks std/no_std allocating and caller-buffer scalar output
against the unmodified C++ 1.3 oracle, and candidate native SIMD against the
v0.2.5 SIMD binary. Valid encoded filters retain the original +/-1 C++ SIMD
conformance check. Vertex checks cover v0/v1, layout/block edges and malformed
truncations. Long fuzzing and the full qualification sweep remain nightly work.
