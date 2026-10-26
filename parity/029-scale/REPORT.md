# SPEC-029 scale strided

## Layout identification (recorded before library edits)

Baseline v0.2.8, d38ef8d; branch perf/scale-strided, initially clean.
The retained r2 harness `src/backend.rs:145` constructs Positions::from_bytes
with MeshPositions bytes/count/stride/offset and LittleEndian.
The committed L2 replay constructs the same byte view (not a float view).
All three chains use 64-byte records, XYZ byte offset 0, little endian.
Ordinary and legacy scale calls share this layout. Frozen inputs are packed.

Source evidence: parity/028-L2/stages.json and stage-support.rs; original
r2 harness at <scratch>/meshopt-simplify-perf/r2/harness/.
The specified NAS archive path was absent on first access. The original L2
scratch is still available locally; use it read-only and copy required sources
into this lane's scratch. Chain blobs remain read in place.

Decision: specialize only unmapped 64-byte little-endian byte views. All other
views, including floats, big endian, other byte strides and remapped views,
retain their existing paths. Support unaligned offsets and a final XYZ-only
record as before. The public packed fast path remains unchanged.

Cause: the generic byte view uses dynamic record strides/bounds and the bounds
callback has three early finite-error branches per vertex. Separating the
finite reduction from bounds with fixed-width decoding removes those branches.
No Workspace or Work is used by simplify_scale, so accounting remains zero.

## Pre-registered measurement

Retired user instructions (`perf stat -e instructions:u`), CPU 9; median of
three resident repetitions per arm, same a downstream engine scalar profile (opt-level 3,
thin LTO, codegen-units 1). Input preparation and warm-up precede perf enable.
Candidate/v0.2.8 <=0.90 on each real chain: PASS; improvement missing 0.90:
PARTIAL; any ratio >1.00: FAIL. Report candidate/M7 and packed frozen ratios
(<=1.01). Also disclose frozen positions repadded to the real 64-byte layout.
No wall timing campaign on the loaded host. No favorable-sample selection.

- 214e85c568a64e8a: stride 64, scale calls 2; blob 143dc6ef8a0d6d85e1b904ce0ae8ddbd3ad347c97d52fe4413977902a393170b
- b6fdb93d8a1709e2: stride 64, scale calls 2; blob c0f13e870a5e86879b698d6fc84fb6b46439a08e4e1a154ace0dba8b3326264c
- a082ca1fd19bb86a: stride 64, scale calls 2; blob e98dd4889394bdf4aa6671dff7931ffe55a0fee1bf57306f75bf578d125d5448

Admission preparation: the generic `python3 build.py` submission inherited the
shared Python class's 8 GiB historical reservation despite declaring 4 GiB.
Canceled that queued submission before any build or measurement, and submitted
the named lane job.sh instead (normal 4 GiB declaration, no class override).
Host load average was 31.65 at 14:27 CET; omit wall runs as specified.

## Implementation and proof scope

- `src/input.rs:568`: dispatch once to fixed 64-byte little-endian XYZ records;
  unmapped views only, checked first-12-byte decode, count-limited scan.
- `src/simplify.rs:190` / `:231` / `:237`: separate finite and bounds reductions only for
  this reader. Keep strict `>`/`<` updates and `>=` extent selection (including
  signed-zero ordering), InvalidParameter before extent overflow, empty +0.0.
- `fuzz/src/lib.rs:339`: retain the same RNG/position stream while cycling
  packed, 16/64-byte padded float and both byte orders; offsets 0..=3 and
  final XYZ-only records. Arbitrary float mutations reach the new path.
- `parity/029-scale/exact.rs`: compare v0.2.8/candidate Result discriminants,
  error names and successful float bits across every required stride, offset,
  order and edge value; padding contains nonfinite data. Additional 68/256-byte
  controls retain the old reader. Run with std and no_std library builds.
- `parity/029-scale/differential.py`: use the existing native C++ scale
  protocol/fixtures and 128 existing seeded meshes against the pinned oracle.
  The resident instruction driver also compares all real/frozen stage bits.

The C++ driver is a standalone executable, not a Rust FFI driver. No new unsafe
Rust, API change, explicit SIMD, memory policy change or version bump.

Qualification declaration: retained <job-runner> build receipt 2893334 reported
882 MiB peak and 27 s runtime, exit 0. The qualification phases are sequential,
the largest byte view is 463,188,736 bytes, and preparations precede each driver.
Use a 2 GiB declaration for qualification (within the <=4 GiB spec ceiling),
with ordinary CPU-only admission and no priority/history/declaration override.
The earlier 4 GiB qualification submission was canceled while queued, before
any samples or tests. Its receipt is retained separately.

## Continuation review and rejected candidate

The prior run completed `instructions-final` (117 observations), and fmt,
all-feature and no-default-feature tests returned 0 before interruption.
A duplicate qualify launch subsequently returned 1 because the campaign
output directory already existed; it did not collect another sample.
The retained initial candidate improved each real chain to 0.7353x v0.2.8
but regressed tiny packed input to 1.028033x (limit 1.01), so it is rejected.
Its counters and source/binary binding remain in scratch `instructions-final/`
and `prior-run/`; no failed receipt is cleared by the continuation.

Decision before revised samples: outline the non-packed dispatch with
`#[inline(never)]` so its reader state does not enlarge the packed fast path.
Keep the algorithm, inputs, median-of-three rule and thresholds unchanged.
Rebuild both Rust arms and C++, then collect the complete campaign under
`instructions-revised/` and run every proof gate on the revised sources.
The final verdict must include the packed <=1.01 gate; the broad aspiration
of C++ parity is reported separately from the registered relative gate.

Revised campaign disposition: all 117 counter observations completed, with
exact real-stage output hashes and cross-arm float bits verified. The three
real-chain ratios are 0.735294–0.735296 against v0.2.8 and
2.083313–2.083329 against M7. Packed cases are 1.000000093–1.001557060,
within the unchanged 1.01 limit. The original packed regression remains a
rejected candidate; outlining reduced tiny-packed overhead from about 18 to
about 1 retired instruction per call. No further tuning or sample selection.

C++ parity in instruction count is **not achieved**: the registered relative
improvement gate passes, while candidate/M7 remains about 2.0833x on the real
chains. Wall measurements were omitted under the owner's loaded-host rule.
Both exact comparison configurations pass 294,462 observations, including
nonfinite precedence, signed zero, subnormals, overflowing extents and empty
views; the existing C++ differential passes 178 cases. Final source identity
is verified against `build.json` before report generation.

## Final measured results

**PASS** under the registered instruction gate, including the packed non-regression limit.

| Case | v0.2.8 instructions | Candidate | M7 | Candidate/base | Candidate/M7 |
|---|---:|---:|---:|---:|---:|
| 214e | 462294385.00 | 339922471.00 | 163163203.75 | 0.735294 | 2.083328 |
| b6fd | 123035767.25 | 90467600.25 | 43424855.75 | 0.735295 | 2.083314 |
| a082 | 492138431.50 | 361866625.00 | 173696388.50 | 0.735294 | 2.083328 |

Both ordinary and legacy scale stages use the same input; each row measures
the shared stage call. Frozen corpus (12 = packed, 64 = real byte layout):

| Case | v0.2.8 instructions | Candidate | M7 | Candidate/base | Candidate/M7 |
|---|---:|---:|---:|---:|---:|
| tiny-smooth-12 | 642.11 | 643.11 | 631.30 | 1.001557 | 1.018704 |
| tiny-smooth-64 | 1821.11 | 1396.11 | 631.30 | 0.766626 | 2.211482 |
| medium-smooth-12 | 90427.32 | 90428.33 | 101457.29 | 1.000011 | 0.891295 |
| medium-smooth-64 | 287431.33 | 211406.33 | 101457.30 | 0.735502 | 2.083698 |
| medium-seams-12 | 110906.31 | 110907.32 | 124473.29 | 1.000009 | 0.891013 |
| medium-seams-64 | 352643.33 | 259356.33 | 124473.29 | 0.735464 | 2.083630 |
| medium-sparse-12 | 180754.32 | 180755.32 | 202857.30 | 1.000006 | 0.891047 |
| medium-sparse-64 | 574731.35 | 422656.33 | 202857.29 | 0.735398 | 2.083516 |
| million-smooth-12 | 10745162.25 | 10745163.25 | 12064940.75 | 1.000000 | 0.890611 |
| million-smooth-64 | 34182678.50 | 25134445.25 | 12064941.25 | 0.735298 | 2.083263 |

Counts exclude parsing, conversion, input allocation and warm-up. Rust and C++
call the nonallocating public scale API and consume its result; no result
transport allocation or importer/non-mesh stage is included. All resident
outputs agree bit-for-bit across all three arms. No whole-cook latency claim.

## Gate exit codes

| Gate | Exit |
|---|---:|
| instructions | 0 |
| fmt | 0 |
| all-features | 0 |
| no-default-features | 0 |
| clippy | 0 |
| boundary | 0 |
| exact-build-std | 0 |
| exact-std | 0 |
| exact-build-no-std | 0 |
| exact-no-std | 0 |
| cpp-differential | 0 |
| fuzz | 0 |

Raw perf counters, stderr, input and binary hashes, exact checker binaries,
C++ protocol observations and fuzz receipts are retained under
`<scratch>/meshopt-029-scale/`. `results.json` binds the
source hashes, compiler, CPU, profile and per-case instruction medians.

Final proof: all-feature tests 219 passed, no-default-feature tests 173 passed
(each configuration has 5 intentionally ignored documentation examples).
Seeded scale fuzzing completed 359.98 CPU-seconds,
432,719,674 executions, 0 crashes; source and executable
identities are unchanged. Six 60-second shards were needed because the first
five totaled 299.99 CPU-seconds; the 300-second threshold was not rounded down.

Designated target `<target-dir>` deleted after retaining drivers, exact checkers,
fuzz binary/corpus, logs, counters and snapshots in lane scratch.
`CLEANUP.json` records 712,124,546 bytes removed; CPU-only <job-runner> receipts
are in `heavy-receipts.tsv`. No wall campaign or broader C++ parity claim.
