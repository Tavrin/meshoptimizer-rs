# S3 accounting and validation proofs

Reference: v0.2.14, `dbb75e03ec011ae04645e6d40b0039561991286d`.
All arithmetic expressions producing geometry, selection order, strict
comparisons and allocations retain their reference order. There is no API,
unsafe-code, root implementation or storage-layout change.

## Packed finite validation

`src/simplify_extra.rs:5`: packed bounds have one visit per vertex. The fast
scan is selected only if all those visits fit. It accumulates the conjunction
of all finite predicates and performs the original ordered min/max selects.
For finite input these selects are exactly the reference selects, including
signed zero; the total charge precedes the unchanged extent calculation.
For any non-finite input it resets both bounds and runs the reference scan,
which charges the same prefix and returns the same first error. Tight budgets
also use that scan. No allocation or caller write happens during validation.
No arithmetic finite check is removed: reciprocal and extent overflow errors
retain their original positions. This is branch consolidation in validation,
not a relaxation of the finite-input contract.

## Fixed scans and triangle pruning

`src/simplify_extra.rs:131`, `:144`, `:292`, `:349`: each selected loop has
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

`src/simplify_extra.rs:160`, `:198`, `:220`, `:253`: parents initially point
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

`src/simplify_extra.rs:426`, `:480`, `:509`, `:545`: one vertex charges one
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

`src/simplify_extra.rs:1110`, `:1126`, `:1232`: table initialization keeps
its original single charge before clearing. Each vertex charges one entry
and at most `table.len()` probes. `Work::batch` proves their complete bound
before running a locally tallied batch; the checked tail retains the original
per-entry charge and the original covered-versus-checked lookup decision.
The mapped table stores the same first source index; unmapped tables store
the same cell id. Thus cell numbering, duplicate selection and reservoir
inputs are unchanged. Even a hypothetical full-table failure settles the
entry and every failed probe before returning `NumericalFailure`. Batch
arithmetic cannot overflow because `Work::batch` caps it at `usize::MAX`.

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
validation and reservoir arithmetic checks remain in place because arbitrary
finite colors and weights do not provide a small global arithmetic bound.

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
