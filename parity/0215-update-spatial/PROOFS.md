# Observable preservation

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
