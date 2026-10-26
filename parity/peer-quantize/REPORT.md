# SPEC-027 lane A

Baseline: clean perf/quantize at 33556c4 (v0.2.6). Only lane A is owned.
Scratch: <scratch>/meshopt-peer-a.

## Causes (milestone, recorded before implementation)

- quantize-half: three magnitude fixups select underflow, overflow and NaN values;
  the scalar tail uses three comparisons and conditional moves. The frozen ours
  adapter also collects x.p.iter().flatten(), whereas X iterates a contiguous
  slice (as_flattened). This creates extra iterator/collection control work and
  different vector-loop shapes. Million-case baseline instructions: ours
  1,737,399,414; X 1,369,694,024; M7 3,435,118,191 (100 iterations plus startup).
- quantize-float: two exponent-dependent selections protect special values and
  flush exponent-zero values. The same nested-iterator versus contiguous-slice
  adapter difference contributes to the measured gap; it is not attributable
  solely to Result validation (constant width is optimized away). Million-case
  baseline instructions: ours 1,309,399,801; X 960,728,979; M7 4,224,761,913.

Evidence: baseline-instructions.json, instructions-baseline-*.txt and retained
B/X/M7 executables in scratch. All 30 perf stat instructions:u runs exit 0.
Counts include initialization/serialization and 100 resident iterations;
no wall-clock acceptance claim follows from these diagnostics. The system perf
launcher lacks tools for kernel 7.0, so the installed 6.17 perf binary is used.
The original failed launcher attempt and admission cancellation are retained.
No optimesh source is copied. No API addition is justified by an iterator
shape that callers can already express with as_flattened(). Frozen adapters
remain intact for the registered campaign.

## Fix and selection

- src/quantize.rs:49 clamps magnitude to the first overflowing exponent before
  rebiasing; :51 masks underflows and :52 sets the quiet NaN bit independently.
- src/quantize.rs:68 disables rounding/truncation for exponent-all-ones values;
  :71 flushes exponent-zero inputs with a mask. Width validation is unchanged.
- Safe Rust only; no API, SIMD-path, encoder, version or other-family changes.
- Candidate v1 million-case instructions: half 1,585,088,117 (0.912 of baseline),
  float 1,271,322,723 (0.971). Candidate v2 changed the underflow mask and used
  two output masks for float: same half count, 1.060x v1 float instructions.
  Rejected v2 solely on instruction counts; restored v1 before final gates and
  the sole wall campaign. No acceptance timing was used to select the candidate.

## Registered results — verbatim, no reruns

Ratios are candidate/comparator (lower is faster), five equal-weight frozen
cases. One warm-up, ten alternating pairs, a downstream engine scalar profile, CPU 23;
bootstrap 10,000, seed 20261029. See REGISTRATION.md and results.json for exact
core metadata, pairs, intervals and hashes. Same-session A/A F=0.309994;
per-case baseline upper95 allowance=1.409994. This noisy shared-host floor is
reported as measured, without relaxing either GM bar.

| Family | ours/X GM [95%] | ours/v0.2.6 GM [95%] | worst case upper95 vs baseline | Verdict |
|---|---|---|---|---|
| quantize-half | 1.438703 [1.312049, 1.560020] | 0.951706 [0.924654, 0.983932] | 1.100363 | PARTIAL |
| quantize-float | 1.341685 [1.301321, 1.383065] | 1.018728 [0.993115, 1.047523] | 1.204041 | FAIL |

**Overall FAIL; draft is unqualified and must not ship as-is.** Half passes the
baseline bars but misses X, so it is only a speedup with no peer-parity claim.
Float fails the strict baseline GM bar even though its interval includes 1;
its instruction reduction does not establish a wall-time speedup. Keep the
measured source intact for review; do not silently replace the failed arm or
claim a post-campaign source change inherits these numbers. No further timing
campaign or source iteration is performed after these verdicts.

## Targeted proof and gates

| Gate | Exit | Evidence |
|---|---:|---|
| cargo test --all-features | 0 | gate-test.log |
| cargo clippy --all-targets --all-features -- -D warnings | 0 | gate-clippy.log |
| cargo fmt --all -- --check | 0 | gate-fmt.log |
| protocol builds + pinned C++ source check | 0 | gate-protocol-build.log, reference-pin.log |
| frozen corpus + existing differential fixtures + fuzz seeds + boundaries | 0 | proof.json: 717 cases, 660 C++ checks |
| actual Rust Result/Error and successful output bits vs archived v0.2.6 | 0 | direct.json: 1,575,007 values, 34 widths |
| single registered wall collection | 0 | wall.json, completed=true |
| registered acceptance | 1 | half PARTIAL; float FAIL |

The direct comparison includes all frozen position bit patterns, exponent and
rounding boundaries, NaN payloads, signed zero, both seeded streams, widths
0..32 and u32::MAX. It compares Result<u32, Error> after mapping successful
floats to their bits, not merely ok/err. Half outputs compare as u16.
The existing fuzz generator is copied unchanged except for a fixed 256-message
bound and retaining generated inputs: seeds 20261033 (half) and 20261034 (float).
All 512 inputs compare exact native protocol outputs/errors to v0.2.6; C++ is
checked where Rust accepts the input. Invalid inputs are never passed to an
unchecked C++ API. Existing p01x differential fixtures remain byte-exact.
No 10k sweeps or long fuzz runs; those remain nightly.

## Provenance and decisions

- Comparator branch bench/peer-screen is unchanged at
  e8827e4de9b69c98b9a96c6f49f8192a6b045ff7. Scratch manifests alone redirect
  dependencies. All frozen input hashes match its results.json.
- Upstream C++ oracle is clean at 4c203430ca565cb59a468a91922c76c208169536,
  identical to v1.3; existing check-reference.sh exits 0.
- Every heavy job declares 4 GB through <job-runner>, MOSS_HEAVY_GPU=0. Generic
  python admission initially inherited a 9 GB historical reservation; that
  queued attempt was canceled before execution and requeued with the documented
  lane-specific MOSS_HEAVY_CLASS=meshopt-peer-a. No admission gate was disabled.
- Initial scratch build exited 1 because exported lockfile path identity needed
  updating; --offline regenerated scratch lockfiles only. All final builds exit
  0. Initial perf launcher failed before executing the driver; the installed
  6.17 perf tool collected all final instruction diagnostics successfully.
- All raw logs, inputs, fuzz messages, source snapshots, binaries, intermediate
  instruction candidates and sole wall stream are retained in lane scratch.
  Compact results and SHA-256 manifest are committed under parity/peer-quantize.
- Commit/push as Tavrin; draft PR only. No merge, tag, publication or version bump.

The lane Cargo target was deleted after all builds/checks/collection finished;
parity/peer-quantize/cleanup.json records the path and bytes removed. Raw scratch
receipts and executables remain. Scope is lane A only; no other worktree touched.

## SPEC-027 quantize finish — half only

The failed quantize-float optimization is reverted verbatim to v0.2.6.
`git diff v0.2.6 -- src/quantize.rs` now contains only quantize_half lines.
The registered results above remain frozen: half is a speedup (0.951706×,
95% GM interval [0.924654, 0.983932]), with no X timing-parity claim; the
measured float candidate remains FAIL and is excluded from the shipped code.
The historical overall FAIL describes that two-family candidate, not this
half-only selection. No wall-clock campaign was run for this finish.

### Fair-adapter instruction disclosure

Scratch-only adapter copies use `x.p.as_flattened().iter()` for ours (baseline
and candidate) and M7; X retains its existing contiguous-slice adapter. Frozen
repository and bet 0 harnesses are unchanged. Same retained million-smooth
input (SHA-256 f3a568950e936cdc8e0311e96dabf847ecb368d0cb1ac7644c90fb6e7cb301bf),
same a downstream engine scalar profile, 100 resident iterations plus startup/serialization,
one `perf stat -e instructions:u` run per cell, CPU affinity pinned to the
first allowed CPU. Installed 6.17 perf used as in the original diagnosis.
Counts are diagnostic only: no acceptance bar, timing inference, or revision
of the registered verdicts. M7's adapter is flattened too so all four arms
iterate contiguous positions; scratch diffs retain the exact changes.

| Family | v0.2.6 | Candidate (half only) | X 1.1.0 | M7 0.7.0 |
|---|---:|---:|---:|---:|
| quantize-half | 1,356,610,288 | 1,280,454,393 | 1,369,695,290 | 3,693,533,036 |
| quantize-float | 947,655,255 | 947,654,166 | 960,733,814 | 4,478,634,647 |

All eight perf exits are 0; all four serialized outputs match within each family.
The reverted float's small baseline/candidate count difference includes process
startup and binary-layout effects; its library source is identical.

### Final proof, gates and retention

All exits are 0: diagnostic builds, protocol/C++ builds and source-pin check,
existing lane A exactness proof (717 cases, 660 C++ checks), direct Rust
Result/Error/output-bit proof (1,575,007 values, 34 widths),
`cargo test --all-features`,
`cargo clippy --all-targets --all-features -- -D warnings`, and
`cargo fmt --all -- --check`. Both families were rechecked after the revert.

Every heavy action ran under CPU-only <job-runner>, declaring 4 GB.
Raw logs, copied adapters, executables, proof rows and final source are retained
under `<scratch>/meshopt-peer-a/finish/`; compact receipts,
adapter diffs and SHA-256 inventory are in `parity/peer-quantize/finish/`.
Original receipts remain retained. The designated target
`<target-dir>` was deleted after validation;
`finish/cleanup.json` records deletion. No merge, tag, publication or version bump.
