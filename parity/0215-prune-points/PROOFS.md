# S3 accounting and validation proofs

Reference: v0.2.14, `dbb75e03ec011ae04645e6d40b0039561991286d`.
All arithmetic expressions producing geometry, selection order, strict
comparisons and allocations retain their reference order. There is no API,
unsafe-code, root implementation or storage-layout change.

## Packed finite validation

`src/simplify_extra.rs:5`, `:68`, `:119`: packed bounds have one visit per
vertex. The fast scan is selected only if all those visits fit. It returns
`None` if any coordinate is not finite; the bounds are then discarded and the
reference scan runs, which charges the same prefix and returns the same first
error. Tight budgets also use that scan. Otherwise the total charge precedes
the unchanged extent calculation. No allocation or caller write happens
during validation. No arithmetic finite check is removed: reciprocal and
extent overflow errors retain their original positions. This is branch
consolidation in validation, not a relaxation of the finite-input contract.

The check-and-finish pass replaced the sequential fast scan by lanes. The
flattened coordinates are read twelve at a time; lane `l` of quad `k` sees
component `(4k + l) % 3`, in input order. Each lane keeps the reference
select `if lo > v { lo = v }` (and its maximum counterpart), and a sum of
`v - v`, which is `+0` for finite `v` and NaN for an infinity or NaN, so the
sum is zero exactly when every value is finite. The lanes and the tail are
then combined with the same selects. For finite values the selects return
the numerical minimum and maximum regardless of order; equal nonzero values
have equal bits, so order can change only the sign of a zero result. When the
reference minimum of a component is zero, every value before its first zero
is positive and later values are not below zero, so the reference keeps the
first zero in input order (symmetrically for a zero maximum). The final pass
restores exactly that value by finding the component's first zero. Empty
input keeps `f32::MAX`/`-f32::MAX`, as in the reference. The lane loop is out
of line so that the compiler keeps one vector register per accumulator.
`bounds_tests` compares bit patterns against the ordered selects for lengths
0 to 29 over single-sign, zero-only, extreme and non-finite pools, and the
exact driver adds signed-zero extremes across block boundaries.

## Fixed scans and triangle pruning

`src/simplify_extra.rs:199`, `:212`, `:360`, `:417`: each selected loop has
exactly one unit charge per record and no other fallible operation. Initialization
and component numbering visit `n` records; component-error passes visit
`n`, `nc`, `n`, `nc`; pruning visits `count / 3` complete triangles. The
existing `Work::precharge(count, 1)` selects prepayment only if the entire
loop fits. Otherwise the original individual charges precede every write.
The prepaid loop therefore cannot stop on fuel; charging its complete known
total at entry has no observable intermediate state. All caller-buffer writes
and failed prefixes are identical. Component-error records are grouped into
four-float slices with the same indices and operation order; this introduces
no arithmetic reassociation or removed finite predicate.

## Union-find traversal batches

`src/simplify_extra.rs:228`, `:266`, `:288`, `:321`: parents initially point
to themselves. Every union links the larger root to the smaller; compression
copies an ancestor. A nonterminal parent is therefore strictly smaller than
its child, so each follow visits at most `n - 1` links. Three edges per
triangle charge three visits plus at most six follows, bounded by `6*n + 3`.
One root-compression record charges one visit and at most `n - 1` links,
bounded by `n`. The existing `Work::batch` proves a batch fits both remaining
fuel and `usize::MAX` before any unchecked tally. If bound construction
overflows, or no record fits, the original checked traversal runs instead.
Actual visits, including path compression, are accumulated and charged once
per covered batch; no allocation, other error or public write intervenes.
The preceding allocation and following dense-numbering pass are unchanged.

## Position-weld probe batches

`src/simplify_extra.rs:494`, `:548`, `:577`, `:613`: one vertex charges one
entry plus at most `table.len()` probes. Packed batches use that bound with
`Work::batch`; general views use the full-product bound in `u128`, otherwise
the reference checked path. Position reads follow successful validation and
cannot fail on a valid view; the general kernel still propagates any read
error and settles its exact visited prefix. Normalized-zero hashing and
numerical equality are unchanged. The table contains earlier vertex indices
or the empty `u32::MAX` sentinel, and topology rejects vertex counts above
`u32::MAX`; consequently a failed `source.get(old)` means precisely empty.
Triangular probing visits a power-of-two table completely, exactly as in the
reference. The sufficient occupancy bound is `table.len() >= n` (equality is
possible for one or two vertices): before insertion `i`, at most `i < n`
slots are occupied. Thus every new vertex still finds space. The same first
match or empty slot wins. Global source indices are
retained across batches. Tally sums are within the previously covered bound.

## Point-cell probe batches

`src/simplify_extra.rs:1178`, `:1194`, `:1300`: table initialization keeps
its original single charge before clearing. Each vertex charges one entry
and at most `table.len()` probes. `Work::batch` proves their complete bound
before running a locally tallied batch; the checked tail retains the original
per-entry charge and the original covered-versus-checked lookup decision.
The mapped table stores the same first source index; unmapped tables store
the same cell id. Thus cell numbering, duplicate selection and reservoir
inputs are unchanged. Even a hypothetical full-table failure settles the
entry and every failed probe before returning `NumericalFailure`. Batch
arithmetic cannot overflow because `Work::batch` caps it at `usize::MAX`.

## Uncolored point errors

`src/simplify_extra.rs:2128`, `:2182` (check-and-finish pass): without
colors, the reservoir accumulation and the error pass charge one visit per
vertex and contain no fallible read. They now use the same `units!` prepay as
the fixed component scans, so a fitting scan is charged once before it runs
and a tight budget keeps the per-visit checked path, including its partial
writes to the caller's index buffer. The colored instantiation is unchanged.

The uncolored error pass drops its `!e.is_finite()` exit. Validation proved
every position finite and the extent finite with a finite reciprocal. Rounding
is monotone, so `0 <= a - lo <= hi - lo <= extent` and each normalized
coordinate lies in `[0, 1 + 2^-22]` (or is zero when the scale is zero). A
cell sum of at most `2^32` such values is below `2^34`; its count is at least
`1` (each cell owns a vertex) and at most `2^24` once rounding saturates, so
its mean lies in `[0, 2^34]`. Each difference is then below `2^35` in
magnitude and `e`, a sum of three squares, is below `2^72`, far below
`f32::MAX`; no operand is NaN. The removed exit therefore never fired, and
the error, its precedence and `Usage.work` are unchanged. The earlier
`weight` check, which the uncolored error does not use, is kept in place.

## Shared work helpers and observable state

`src/workspace.rs:643`, `:651`: these are the only shared-file additions.
`covers_wide` compares a conservative `u128` bound to the remaining `u64`
budget without narrowing. `settle` narrows and subtracts an exact tally only
when it fits. Every new use first proves the corresponding complete bound;
the subtraction is equivalent to the same sequence of successful `add(1)`
operations, and cannot change `Usage.work`. Budget allocation/release,
workspace acquisition/retention and all buffer capacities are untouched.
Shared component helpers also serve simplify with `PRUNE`; bounds and cells
also serve sloppy simplification. The exact driver covers those callers,
including update, in both feature modes and retention modes.

## Roots and remaining checks

No root call changes in this lane. The release already has native roots for
proven nonnegative finite operands with its portable no-default path. No
additional numeric bound or platform-wide root claim is needed here. Color
validation and the colored reservoir arithmetic check remain in place because
arbitrary finite colors and weights do not provide a small global arithmetic
bound; only the uncolored check is removed, by the bound above.

## Exact and adversarial domains

`prepare.py` derives its observation macro from the retained exact driver;
`exact-main.rs` replaces its enumeration. It compares full output buffers,
`Result` (including error variants), float bit patterns, `Usage.work` and
`Usage.bytes`, with two calls per workspace. Every small-case integer fuel
limit from zero through unrestricted usage plus one is tested. API variants
are owned, full sentinel buffer, empty destination, and destructive update.
Position variants include packed, interleaved, byte-backed, both byte orders,
unaligned starts and short final records. Colored/uncolored points, three
target sizes, three prune cutoffs, PRUNE on/off for shared callers, both
retention modes, low storage limits and exceptional float bit patterns are
included. Collision-heavy meshes, welded duplicates, disconnected faces and
descending chains exercise traversal bounds. Fuzzing keeps half its inputs
unmodified and valid, and now varies point targets and colors explicitly;
this avoids the old generator's predominantly zero-target point fast exits.

Read-only adversarial review traced sentinel validity, global batch offsets,
empty inputs, overflow on narrower targets, first failed charge, output copy
placement, partial caller writes, and shared callers. No counterexample was
found. This is a single-agent review, not an independent external review.
Runtime qualification remains limited to the tested host; cross-target
correctness additionally relies on the bounds above.
