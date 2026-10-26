# SPEC-026-decode registration

Frozen before any new wall-clock sample. Baseline: bea3e04bf22063988a80db4764e80c3dd422a634 (v0.2.5).
Source: /home/etienne/dev/meshoptimizer-rs-wt/peer-screen/parity/compare from bench/peer-screen.
Scratch: /mnt/linux-extra/moss-scratch/meshopt-decode. Target: /mnt/linux-extra/moss-cargo-targets/meshopt-decode; delete at completion.
All builds/probes/measurements use moss-heavy, MOSS_HEAVY_GPU=0, declaration 4 GiB. Headless Chromium only. No agents.

## Required policy (verbatim)
- **Native:** the bet 0 harness (parity/compare from `bench/peer-screen`), Moss scalar profile,
  oct-decode and quat-decode only, against M7. Same-session A/A floor first.
  - Bar: ours/M7 GM ≤1.00, the inverse of peer/ours.
  - Our old-version comparison (against v0.2.5) must be ≤1.02 + F for the vertex-decode family,
    meaning no regression.
- **Browser:** the 42 frozen streams, matched transport, 15 rotating calls after one warm-up, the
  registered timer policy.
  - Bar: Rust scalar against upstream scalar JS, GM ≤1.00 and each case upper95 ≤1.10.
  - No byte differences, and no SIMD regression.
- One infrastructure rerun is allowed. No reruns to change a verdict.
- **Verdict:** PASS or PARTIAL, per family.

## Implementation details fixed in advance
Instruction-only iteration first: perf stat -e instructions:u, fixed 100 calls per medium-smooth/medium-seams/medium-sparse filter and M7, including matched allocation/copy/drop from bet 0. No Instant calls on the fixed probe branch. Also inspect optimized assembly. Counter unavailability is recorded, never replaced by timing iteration.
Native final run: frozen five synthetic bet-0 inputs (tiny-smooth, medium-smooth, medium-seams, medium-sparse, million-smooth); same physical CPU; resident public allocating API driver and canonical Moss opt-level=3/thin-LTO/one-CGU profile. Same-session A/A before comparisons. Original harness calibration (25 ms minimum side), five alternating pairs, extend to twenty only if initial nominal paired log Student-t 95% interval straddles 1. F is maximum absolute A/A family GM departure from 1. Vertex old-version comparison only for the no-regression gate. Native filter outputs must equal scalar C++ 1.3.
Browser: reuse REPORT-3 streams.json and streams.bin byte-for-byte; reuse Rust allocating adapter and shipped upstream scalar WASM JS adapter. Match timed copy-in and copy-out for both: each call copies source to the existing linear memory and copies decoded bytes to an existing JS Uint8Array. Keep Rust's required internal allocation/write-out costs. One page, CPU 2, stream-index arm rotation and call-index rotation; one warm-up then 15 paired calls. Timer: performance.now; retain all 42 cases including zeros. Exactly one decode call per timed observation (15 calls per arm/case); no batching or adaptive browser calibration. Report resolution. A nonpositive observation makes that case unqualified (PARTIAL), never exclude it from acceptance. Case ratios use paired log means and nominal Student-t upper95 (df=14, t=2.1447867); family GM is equal-weighted across streams. Baseline scalar, candidate scalar and baseline/candidate SIMD arms all byte-check against the retained in-house decoder and upstream. SIMD no-regression bar: candidate/baseline GM <=1.02+F, each upper95 <=1.10 (same browser timers); no SIMD source edits.
Exactly one final registered native/browser campaign. Retain all partial and failed samples. A single infrastructure retry may resume incomplete work with prior evidence retained; no verdict-driven retries.
If safe scalar code cannot close a gap, retain PARTIAL; do not introduce SIMD or change contracts. No full sweep or long fuzz campaign.

Pre-sample correction: the initial draft proposed 100-call timer batches. That would not honor the required 15 calls verbatim. Removed before any wall-clock sample. Coarse-timer zeros remain visible and cannot qualify the per-case upper95 bar. The retained draft is not the applied policy.
