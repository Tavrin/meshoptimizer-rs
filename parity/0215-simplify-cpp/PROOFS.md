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

## Check-lane review of the two routines above

Both safety arguments hold. In the cell table, `Work::batch` returns at most
`min(remaining, usize::MAX) / (table.len() + 1)` vertices, so every vertex of a
batch would have passed the 0.2.14 per-vertex charge and its covered-probe
test; the batch tally is the sum of the charges those calls make, the failure
path charges the same entry plus full table, and writes to the table and the
output happen in the same order. A zero batch replays the checked path for
the rest. In packed bounds, a lane minimum or maximum of finite values equals
the ordered-select result except for the sign of a zero extreme; both bounds
are restored to the first zero of their component, which is the value the
ordered selects keep (an earlier value lies strictly on the other side). The
sign of the upper bound matters: `hi - lo` with two zeros can give `-0`, which
`d >= extent` accepts. These routines are identical, byte for byte, to the
ones on the prune/points branch, whose tests also cover the zero signs.

## Packed finite validation in simplify

`src/simplify.rs:30`, `:569`: without attributes, packed positions take a
fast path only when the whole scan fits the remaining work. `x - x` is +0 for
a finite `x` and NaN for an infinite or NaN `x`; NaN persists through the
eight lane sums and the tail sum, and sums of +0 stay +0, so the total is
zero exactly when every coordinate is finite. The path then charges one visit
per vertex, the same total the counted scan charges on success. If a value is
not finite or the scan does not fit, nothing has been charged and the counted
scan runs, returning the same first error and failure prefix. The covering
test raises the same size error the scan would raise first. Attribute
validation and mapped or strided positions keep the counted scan.

## Packed normalization in simplify

`src/simplify.rs:2503`: the normalization pass has three fallible steps per
vertex: its work charge, a position read and, when the reciprocal scale is
not finite, the finite guard; with attributes it also normalizes the
attribute row. For packed positions without a mapping, without attributes,
when positions were not cached by the remap pass and the scale is finite,
reads are in bounds and the guard cannot fire, so only the charge can fail.
`precharge(n, 1)` charges all `n` visits at once only when they fit; the
checked loop would then have charged the same visits without exhausting the
budget. The normalized values use the same expression and operand order. If
the visits do not fit, nothing is charged and the 0.2.14 loop runs.

## Compaction charge in simplify

`src/simplify.rs:2673`: the triangle compaction pass charges three visits per
triangle before reading it; its reads and writes cannot fail. When the whole
pass fits, the charge is made once before the first write; otherwise the
per-triangle charge runs as in 0.2.14. Writes, their order and the resulting
count are unchanged.

## Edge-kind test in quadric construction

`src/simplify.rs:1847`: the test that one edge end is a border, seam or fringe
vertex is a `matches!` pattern instead of a three-element byte-slice
`contains`, which compiled to a memchr scan. It accepts the same constants.
