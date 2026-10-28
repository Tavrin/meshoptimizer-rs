# Observable preservation

## Check lane: spatial

These sections cover the check lane's spatial change (campaign 3). Update
source stays identical to v0.2.14. Shared helper changes: none.

### Spatial ties

`src/meshlet_spatial.rs:30`: the merge now keeps the accumulator on ties
(`acc < b ? acc : b`, `acc > b ? acc : b`), the operand order of upstream's
SSE `_mm_min_ps(acc, b)`, instead of v0.2.14's `b < acc ? b : acc`. Lane 3
is no longer merged; it never fed the area.

1. Every box reaching `Bvh::split` is finite. `spatial` checks
   `finite((lo + hi) / 2)` for every box and axis before the builder runs,
   with its original error and charge; a NaN or infinite bound makes that
   centroid NaN or infinite. Accumulators start at `f32::MAX`/`-f32::MAX`.
2. For finite operands the two selects differ only when `acc == b` with
   different bits, i.e. `+0`/`-0`. So each accumulated bound equals the
   v0.2.14 bound bitwise, except that a zero bound may have the other sign.
3. Extent `e = hi - lo`. If neither bound is zero, `e` is identical. If one
   is zero, `e` is `hi` or `-lo` exactly, whatever the zero's sign. If both
   are zero, `e` is a zero of either sign. So `e` differs at most in the
   sign of a zero.
4. Products `x*y`, `y*z`, `z*x` and the left-associated sum: a nonzero
   finite or infinite operand is identical; a zero operand gives a zero of
   either sign, or NaN when multiplied by an infinite extent in both
   versions. Sums of values differing only in zero signs differ only in
   the result's zero sign, and NaN/infinity outcomes match. So each area
   equals v0.2.14's bitwise, or both are zeros, or both are NaN.
5. Areas are consumed only by `Pivot` (`la * l + ra * r`, the fill term
   `cost + fill * (lr * la + rr * ra)`) and by the comparisons `cost <=
   best`, `cost < best`, `axiscost < bestcost`. No division, root,
   `to_bits`, sign test or output uses them. Multiplication and addition
   carry a zero-sign difference only into zero results, and IEEE comparison
   treats `+0 == -0` and is false on NaN in both versions. Therefore every
   comparison, hence every chosen axis and split, the partition, the
   boundary marks, and all output bytes are identical. Areas are internal
   scratch; nothing else reads them.

Work: `ctx.tick` calls and their positions are unchanged; no fallible
operation was added or moved. Storage: same allocations and sizes.

### Area loop indexing

`src/meshlet_spatial.rs:263`: the zipped forward/reverse iterator became an
index loop over `0..n` reading `order[i]` and `order[n - 1 - i]`. It visits
the same boxes in the same order and writes the same area slots. Bounds
checks remain.

### Partition with local slices

`src/meshlet_spatial.rs:312`: `sides`, `temp` and each axis order are
borrowed as local slices so their pointers and lengths stay in registers. The
side marks are written as two loops (`0` for the first `split` entries, `1`
for the rest), the same values v0.2.14 wrote with `u8::from(i >= split)`.
The stable partition writes the same `temp` entries; the copy back is the
same `n` values. The work charge `ctx.tick(n * 3)` stays before all of it.

### Vertex counting by triangle record

`src/meshlet_spatial.rs:61`: `count` indexes `indices.as_chunks::<3>()` by
triangle instead of slicing `indices[v * 3..][..3]`. Sorted orders contain
only triangle ids below `indices.len() / 3`, so the same three indices are
read. All three `used < 0` tests still precede the three marks, so repeated
vertices in a degenerate triangle are counted as before. Prefix counts and
the reset loop are unchanged.

Final disposition: both experiments were reverted; final library source and
manifests equal v0.2.14. The proofs and file:line anchors below document the
retained experimental patches, not a change in the final library.

Disposition after campaign 1: the W change below was reverted after the
medium-sparse update case exceeded the 1.01 regression ceiling. Its correctness
proof is retained as evidence for the rejected candidate. Campaign 2 retains
only the spatial transformation; update source is identical to v0.2.14.

## W: update bounds scan

`src/simplify_extra.rs:737`: the post-solve bounds pass has exactly `s.p.len()`
visits and one work unit before each visit. `Work::scan` proves once that
this count fits remaining work; on that branch it subtracts the exact count
after success, or the prefix including the failing visit after an error.
Otherwise it executes the original per-visit `add(1)` before the callback.
The count is a slice length and fits usize; the existing checked conversion
and multiplication in `src/workspace.rs:539` protect the accounting width.
No allocation or additional storage is introduced. The callback reads the same
positions in the same order and updates only local bounds. It cannot observe
the deferred counter. Position lookup errors and work-limit errors therefore
retain precedence, consumed work and partial caller writes. No precharge is
moved across a fallible read. Shared helper changes: none.

## Spatial area calculation and traversal

`src/meshlet_spatial.rs:25`: borrowing a Box3 avoids materializing a by-value
record. Each lane retains exactly the old ordered comparison, including its
tie choice for signed zero. The differences and products for lanes 0 through
2 remain `x*y`, `y*z`, `z*x`, with the same left-associated sum. No floating
point reassociation, min/max substitution, root substitution or finite-check
removal occurs. The fourth lane is private padding and contributes nothing to
the returned area; its extra arithmetic cannot alter any observable value.
Box layout, allocation type and capacity are unchanged.

`src/meshlet_spatial.rs:253`: forward and reverse accumulators are independent.
Each visits exactly the same box sequence and writes exactly the same area
sequence. Moving the reverse accumulator into its own loop changes neither
the charged `ctx.tick(n)` preceding both loops nor the next charge/error point.
No fallible operation exists between those charges. Sorted axes contain only
valid triangle IDs created in `spatial`; all box and area bounds therefore
remain valid. Rust bounds checks are retained. Internal area writes have no
caller alias. No extra storage or changed partial caller output follows.

## Retained F and R checks

The baseline already uses native roots locally in std simplification and
covered-work variants in the solver. Attribute-conditioned solver values and
de-normalized mutable outputs lack a sufficiently tight universal envelope
for removing their finite checks; those checks remain. The spatial SAH path
has no root. Its centroid finite check remains at its original location and
charge. There is no new root operand or new fuzz-unreachable numeric branch.
The lane does not import the contract-cost experiment's deliberately unchecked
arithmetic. That experiment supports investigating W/R/F costs; it does not
prove this lane's performance or numeric validity.
