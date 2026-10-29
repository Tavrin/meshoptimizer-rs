# Contract proofs

Baseline: v0.2.14. The packed bounds and cell-table routines are shared by
sloppy and point simplification; no public API or shared workspace helper changes.

## Packed bounds

`src/simplify_extra.rs:5`: the fast path requires enough work for every vertex.
Each lane applies the original ordered comparison to its subsequence. Finite
values have the same numerical minimum and maximum regardless of order; equal
nonzero values have equal bits. For a zero bound, the first zero in the input
is restored, preserving the original sign. The sum of `v - v` is zero for
finite values and NaN for any nonfinite value. A failed scan is discarded and
the original counted scan is replayed, preserving the first error and work
prefix. The successful scan charges exactly one per vertex before the same
extent calculation. There are no writes or allocation during the scan.

## Cell-table probes

`src/simplify_extra.rs:1044`: each vertex costs one entry visit and at most
`table.len()` probes. `Work::batch` bounds the entire batch by remaining work
and `usize::MAX`; its local tally cannot overflow. The covered batch charges
its exact visits once, including the full-table failure prefix. If a batch
does not fit, the original per-entry and per-probe checks run before writes.
The same first matching or empty slot wins, and table initialization retains
its original charge. The helper is also used by point simplification.
