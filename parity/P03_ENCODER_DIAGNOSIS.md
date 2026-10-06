# Encoder diagnosis (0.3 round 1)

Worktree `/home/etienne/dev/meshopt-wt/v03`, baseline `eb5b1a2`, pinned
meshoptimizer 1.3 `4c203430ca565cb59a468a91922c76c208169536`.
Evidence: `/mnt/linux-extra/moss-scratch/meshopt-v03`.

No elapsed-time experiment preceded diagnosis. The read-only `SCOUT.md` maps
hot functions to source lines. `scout-counters.json` binds an existing comparison
binary to every library source hash plus its consumer, Cargo manifests and locks
in the current clean baseline. All seven output buffers agree byte-for-byte
with the verified upstream oracle on the same frozen medium-smooth geometry.
This reuse avoids a rebuild while the mandatory build disk floor is unavailable.
It does not transfer any historical timing verdict.

CPU 26, PMU user-mode instructions/cycles/branches/misses, N=1,000 and 2N=2,000;
subtract process startup and untimed input preparation. Every counter has 100%
event scheduling. Ratios below compare instructions per complete allocating call.
Cycles and old driver clock fields are retained but are not a timing verdict.
Negative subtracted branch misses are measurement noise. Raw `.stat` files,
read-only instruction samples and complete objdump disassembly are retained.

| Probe | Rust instructions/call | C++ instructions/call | Ratio | Branch ratio |
|---|---:|---:|---:|---:|
| vertex v1 level 2 | 1,837,555 | 2,200,725 | 0.835 | 0.660 |
| vertex v0 | 1,598,818 | 1,327,278 | 1.205 | 0.773 |
| index v1 | 1,432,177 | 1,181,965 | 1.212 | 0.972 |
| sequence v1 | 980,637 | 787,297 | 1.246 | 1.041 |
| Oct | 341,634 | 266,895 | 1.280 | 0.573 |
| Quat | 259,759 | 308,973 | 0.841 | 0.022 |
| Exp separate | 926,170 | 524,499 | 1.766 | 1.526 |

The default-profile consumer was compiled with generic CPU, opt-level 3,
16 codegen units and no LTO. The lane's further counter/correctness consumers
use generic release/fat LTO/one codegen unit. Keep these separate source/build
identities; neither compiler profile constitutes acceptance without its own
final paired stream.

## Ranked plan

1. Exp: fixed-stride mode specialization and exact bounded conversion. The
   dynamic record/component loops retain division/remainder and iterator
   decisions; safe `as i32` conversion adds saturation/NaN selection before a
   separate validity test. Preserve non-finite/undefined-conversion errors,
   zero exponent inheritance, signed zero, and operation order.
2. Quat: explicit component-major selector and cyclic swizzle. The existing
   loop auto-vectorizes comparisons, then spills packed component indices to
   the stack and reloads four scalar gathered components. Low instruction and
   branch totals do not clear this dependency chain. Preserve first maximum
   selection, strict comparisons, sign, swizzle and quantization.
3. Oct/Quat quantization: compare bounded ordinary casts with the existing
   multi-operation magic-float truncation. Retain whichever actually lowers
   instructions on the valid domain; never change decoder rounding.
4. Vertex: compact group measurements from three `usize` counters plus a
   boolean (32 bytes per group) to their proven byte-sized counts. Specialize
   common strides where that eliminates checked reads and divisions. Every
   version/level (v0/v1, levels 0 through 9) must retain exact encoded bytes,
   including equal-size previous-bit tie selection and capacity failure points.
5. Index/sequence: fixed-index varint emission instead of a dynamically indexed
   five-byte loop. Preserve delta wrapping, FIFO order, baseline switching,
   tail padding and short-buffer failures.
6. Native allocating vertex-v1 streaming-s4: isolate allocation/init from the
   decode body using the identical frozen historical request (SHA-256
   `13413c1e2c69f3de2f28a32005650fb46dfac1f92621cce393acefb3929040e9`).
   Safe initialized output and resource limits remain mandatory. Do not infer
   an allocator fix from instruction reductions alone.

## Diagnostic knockout and final measurement registration

`parity/encoders/knockouts.py` creates disposable copies outside the worktree.
Bounds experiments replace the hot vertex word/literal accesses and encoder
output windows, only on already validated fixtures. Their unchecked helpers
exist only in the disposable copy's existing SIMD module. They cannot be
packaged or merged. Validation experiments omit checked API resource/layout
work and Exp's numeric validity accumulation on valid inputs; output bytes
must still match. Selecting the existing caller-buffer API removes allocating
output from each repeated call. These are separate interventions, not an
additive prediction: API accounting and memory locality can interact.

After counter/parity validation of each fix, run one final immutable stream
against C++ 1.3 and public meshopt 0.6.2 FFI adapters, plus the retained baseline.
Two independent processes each for final Rust and C++ supply A/A resolution:
both upper nominal 95% log-ratio bounds <=1.25. Begin with five alternating
pairs; stop when the maximum bar is resolved and A/A passes, otherwise extend
to at most twenty. Retain unresolved rows and every pair; no repeat-to-pass.
Family bars on resolvable rows: geometric mean <=1.25, maximum <=1.50. Include
allocating and caller-buffer APIs, common vertex strides, versions/levels,
all Exp modes, and the exact streaming exception. Fixed CPU26, ~8ms batches,
CPU-only shared admission, each burst <=690s inside `timeout 840`.

The C++ lane adapter keeps caller destination lengths at their preallocated
bound during repeated calls and truncates only after the timed loop. This
removes the legacy adapter's extra slack reinitialization per caller-buffer
sample. Library code is unchanged. Public allocating Rust bound accounting
and max-index reduction stay timed; both C++ adapters use the explicit 32-bit
index bound. Old-crate vertex format differences are retained and validated
by lossless cross-decoding; performance does not assert byte identity to 0.25.

## Completed cost budget

Fresh generic release/fat-LTO baseline and both disposable knockouts pass
17,198 byte comparisons apiece, including both allocating/caller APIs, all
vertex levels 0-9 in both versions, random layouts/boundaries, all four Exp
modes and varied filter precisions. No production edit or timing preceded it.
All fresh PMU events were scheduled 100%; CPU26, N/2N subtraction. See
`build-{before,bounds,validation}.json`, `parity-*.json` and `counters-*.json`.

| Resident probe / allocating | Rust/C++ instructions | Bounds removed | Validation removed | Allocation removed |
|---|---:|---:|---:|---:|
| vertex v0 (s4/s12/s32) | 1.059-1.061 | 6.1% | 2.0% | 0.0-0.1% |
| vertex v1 level 2 (s4/s12/s32) | 0.802-0.803 | 6.5% | 1.5% | 0.0-0.1% |
| index v0/v1 | 1.280 | -1.7% | 0.0% | 0.5% |
| sequence v0/v1 | 1.243 | 0.0% | 0.0% | 5.0% |
| Oct s4/s8 | 1.105 / 1.217 | 0.0% | -0.3% / 0.0% | 0.1% |
| Quat | 0.772 | 0.0% | 0.0% | 0.2% |
| Exp modes 0/1/2/3 | 1.133 / 1.267 / 1.192 / 1.136 | 0.0% | 13.7 / 13.1 / 15.4 / 14.2% | 0.1% |
| exact streaming decode exception | 1.042 | 0.0% | 0.0% | 1.6% |

Percentages are observed complete-call instruction reductions for independent
interventions; retain negative regressions. The bounds experiment covers the
named hot accesses/windows, not every cold assertion. Allocation selection
also changes API accounting. Do not combine these as independent latency costs.
The streaming instruction budget leaves the allocator/memory-locality cause
open: 1.6% fewer instructions cannot explain a 1.5-2.1x elapsed gap.

Keep the ranked body/codegen fixes. Reject unchecked production access,
removing required validation, and the index bounds knockout's 1.7% regression.
Validate each retained fix with parity/counters before the one final timing pass.

Assembly addresses in the stripped consumer require adding 0x1000 to perf's
DSO file offsets (ELF executable LOAD VirtAddr minus Offset). Quat selector
stack stores/scalar gathers are at 0xa62d1-0xa6312 and 0xa6341-0xa6382;
corresponding sample offsets are 0xa52d1 and 0xa5341.

