# P4 simplifier performance qualification

Status: **complete**. Update **SHIPS**; simplify, attributes and sloppy are
**NO-GAIN**. The complete corrected proof supervisor exited 0.

Baseline: v0.2.15 (`1ecf69abe9f92c17e13dc38fa97be659aee44582`).
C++: clean pinned 1.3 oracle `4c203430ca565cb59a468a91922c76c208169536`.
All 22 vendored C++ source/header files match the oracle (`cpp-binding.json`).
Git references, index, commits and integration state have not been changed.

## Attribution before source changes

[PROFILE.md](PROFILE.md) gives the five leading resolved costs per function,
with Rust/C++ estimates and per-mesh source-line locations. All four functions
were profiled on all three real meshes before editing library source. All 24
instruction-sampling recordings completed, with zero lost samples and exact
outputs across the 12 Rust/C++ pairs (`profile-binding.json`). Raw recordings,
function reports, line reports and perf annotate output are retained in the
external scratch directory selected by P4_SCRATCH.

Quadrics already use packed field additions. The source/disassembly profiles
identify dependent scatter loads, welding comparisons, checked traversal,
grid conversion and update finalization as scalar opportunities. They do not
justify adding an unsafe arithmetic backend. Contributions and reductions
remain in their original order; there is no new SIMD dispatch or unsafe code.

## First experiment: rejected

All 576 samples completed; all six arms have exact output identity in every
one of the 32 workload/function cells. Full instruction ratios and wall times:
[experiment1-table.md](experiment1-table.md). Source/binary bindings and the
rejected patch are retained beside the results.

| Function | Default/base real GM | Maximum case | Verdict |
|---|---:|---:|---|
| simplify | 1.008289 | 1.011346 | NO-GAIN |
| simplify-attributes | 1.005340 | 1.006787 | NO-GAIN |
| simplify-update | 0.964907 | 0.995743 | NO-GAIN |
| simplify-sloppy | 1.377033 | 1.404782 | NO-GAIN |

Eager XYZ equality made welding slower, including a simplify regression above
1.01. The defined signed grid cast was much slower than the existing bit-round
conversion. Both are reverted. A direct packed update bounds scan helped but
did not reach the 0.95 real-mesh gate; that implementation is also replaced.
These results show why source-level instruction simplification alone is not
sufficient evidence of gain.

## Second experiment: performance gate passed

Only original-space bounds reuse remains in the candidate. run_state returns
the bounds it already computed during position remapping; sparse_run propagates
them in the same mapping order. Update finalization keeps the exact visit
charges through Work::scan and omits repeated, already validated position reads
and ordered comparisons. [PROOFS.md](PROOFS.md) gives the preservation argument.
One focused unit test covers dense/sparse order, unused extrema and signed-zero
ties. The baseline traversal, grid conversion, quadrics and evaluations remain
unchanged. Source and patch are frozen for this campaign.

The second campaign completed all 576 samples, with exact outputs in all 32
cells across all six arms. [TABLE.md](TABLE.md) gives every cell, scalar and
default C++ comparisons, real/corpus geometric means, and wall-time disclosure.
Update's default real GM is **0.939939** (6.01% fewer instructions), corpus GM
0.986373, worst case 0.992023. Its scalar/scalar C++ and default/default C++ real
ratios are both 1.184924. The performance verdict is **SHIPS** for update;
simplify, attributes and sloppy are **NO-GAIN**. The shared return-value
plumbing has at most 0.195% instruction overhead on the other measured cases;
all remain below the 1.01 ceiling. It is retained only to carry update's bounds.
No independent optimization or gain is claimed for those three functions.

The final proof exits are below. Queue waiting was not a technical failure or
measurement evidence. All queued jobs were allowed to finish; none was canceled.

Proof attempt 1 passed the all-features exact sweep (8,970,468 comparisons)
and 150.05 child CPU seconds of seeded checks, then exited 101 in the
no-default driver on an unwrapped feature-disabled option mask. No library
mismatch was reported. The failed logs, binaries and driver are retained.
The driver now compares option parser acceptance/errors in both crates and
sweeps constructible settings. The full supervisor rerun passed with distinct
receipt names. The library and measured source hashes are unchanged.

## Retained source

- `src/simplify.rs:943,2638,2989`: private original-space bounds result from
  run_state, discarded by callers that do not finalize updated positions.
- `src/simplify_extra.rs:681,751`: sparse result propagation in original map order.
- `src/simplify_extra.rs:1103`: preserve all work visits without repeating
  validated bounds reads. No allocation, quadric or float-expression changes.
- `src/simplify.rs:2999`: one focused dense/sparse/zero-sign regression test.

No shared workspace, processing or math helper changed. No new unsafe code,
ISA path or public API was introduced. See [PROOFS.md](PROOFS.md).

## Final correctness receipts

All 19 final supervisor stages exited 0; detailed hashes and codes are in
[proof-summary.json](proof-summary.json) and [proof-receipts.json](proof-receipts.json).
The original failed driver receipt remains present and is not counted as a pass.

| Check | All features | No default features |
|---|---:|---:|
| Exact results, tails, mutable views, usage and work boundaries | 8,970,468 checks; exit 0 | 5,769,468 checks; exit 0 |
| Seeded stream | 73,984 generated cases; 150.319 child CPU s; exit 0 | 306,304 generated cases; 150.018 child CPU s; exit 0 |
| Pinned C++ differential | 1,246 cases; exit 0 | 1,246 cases; exit 0 |

The exact driver compares option acceptance/errors for all represented masks,
then runs accepted settings over every work limit through unrestricted usage
plus one on the selected small meshes. Cold, warm and after-failure sequences,
byte-backed and strided mutable views, sparse mappings, storage caps, special
floats, attributes and flags are covered. Native/no-default numerical paths
are compared separately. Total targeted child CPU (including its builds) was
513.063 seconds, below the two-CPU-hour bound.

Default-feature tests, all-feature tests, no-default tests, all-target clippy
with warnings denied, formatting, unsafe/package boundary and public-text
checks each exited 0. This is finite coverage, supplemented by the structural
bounds-reuse argument; it is not an exhaustive test of every possible mesh or
layout. No new ISA-specific backend was added or claimed.

The final source binding matches both measured candidate binaries and the
proof source. Sources, inputs, binaries, raw recordings, failed attempt and
receipts remain in external scratch. The designated Cargo target is removed
at completion; [cleanup.json](cleanup.json) records that removal. Git state
changes and integration remain outside this work.
