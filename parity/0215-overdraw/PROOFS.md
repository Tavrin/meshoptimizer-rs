# optimize_overdraw contract-cost removals: proofs

Each section removes one per-step cost and shows that results, errors and
their order, `Usage.work`, storage accounting and partial writes equal
v0.2.14. No check is removed without a bound; where none exists the checked
path remains. No new `unsafe`, no unchecked indexing, no API change. Line
numbers refer to `src/overdraw.rs` on this branch.

Two facts about IEEE binary32 arithmetic (round to nearest) are used
throughout:

- (P) Propagation. If `x` is infinite or NaN then `x + y`, `x - y`, `y - x`
  and `x * y` are infinite or NaN for every `y` (`inf * 0` and `inf - inf`
  are NaN). A nonfinite value therefore stays nonfinite through any chain of
  additions, subtractions and multiplications. Only a reciprocal maps a
  nonfinite value (`1 / inf = 0`) back to a finite one.
- (M) Monotone sums. For `a, b >= 0`, `fl(a + b) >= max(a, b)`; a rounded sum
  of nonnegative terms is at least its largest term.

## F1. Validation: per-coordinate finite checks (lines 115-127)

0.2.14 checks `is_finite` on each coordinate of each vertex inside a counted
scan and adds the coordinate to the centroid only after the check passes.

For packed positions, when the remaining work covers the `v` visits of that
scan, the candidate sums all coordinates first. By (P) a nonfinite
coordinate makes its running sum nonfinite for the rest of the scan, so three
finite sums prove every coordinate finite. In that case 0.2.14 would pass
every check and compute the same three sums in the same order, then charge
exactly `v`; the candidate charges `v` (`add_covered`, line 124). Otherwise
(some sum nonfinite, possibly only by overflow of finite coordinates, or the
budget does not cover `v`) the candidate runs the unchanged checked scan from
the start, which reproduces the 0.2.14 error, its failure position and its
charged prefix. Nothing is written or charged by the discarded sums.

## F2. Cluster score: thirteen checks become three (lines 216-238)

0.2.14 checks, in order, `area_sum`, `1/area_sum`, the three scaled centroid
components, the normal's squared norm, `1/length`, the three scaled normal
components, the three centroid offsets and the dot product, and fails with
`NumericalFailure` on the first nonfinite one. All failures are the same
error, nothing is charged or written inside `score`, and the root function is
pure (the no_std memo only caches results), so 0.2.14 fails if and only if at
least one of these values is nonfinite.

The candidate computes the same values with the same operations in the same
order and fails when `area_sum`, the squared norm or the dot product is
nonfinite. Equivalence:

- If the candidate fails, one of those three values is nonfinite, and 0.2.14
  checks each of them (if it reaches it, it fails there; if it fails earlier,
  it fails anyway).
- If 0.2.14 fails at a value other than `area_sum` and the squared norm, that
  value is a reciprocal, a scaled component, an offset or the dot product.
  Every one of them reaches the dot product only through multiplications,
  additions and subtractions, and each enters it (a scaled centroid through
  its offset, an offset or a scaled normal component through its product
  term, a reciprocal through the scaled components it multiplies). By (P) the
  dot product is then nonfinite, so the candidate fails too.

The reciprocals are the only escape (`1 / inf = 0`), which is why their
inputs, `area_sum` and the squared norm, are checked directly; `length` is
nonfinite exactly when the squared norm is (a square root of a finite
nonnegative value is finite, of infinity or NaN is not). The exact driver
`exact-extra.rs` pins the reachable escape: two coplanar triangles whose
normals cancel, so the dot product stays finite while the squared norm of a
single triangle overflows, and subnormal areas and normals.

## F3. Soft-cluster target on the prepaid path (line 315)

The prepaid path (unchanged in 0.2.14, line 372 onward of `kernel`) runs only
when `|threshold| <= f32::MAX / 4`. Validation has already proved the
threshold finite. Each hard cluster is nonempty (`end > start`), and
`misses <= 3 * (end - start)` because each triangle misses at most three
vertices. Both are integers below 2^32, so `misses as f32 / (end - start) as f32`
is a correctly rounded quotient of a ratio at most 3, hence at most
`3 * (1 + 2^-23)^2 < 4`. The product with the threshold has magnitude below
`f32::MAX`, and a correctly rounded value below `f32::MAX` in magnitude is
finite. The check cannot fail on this path and is omitted there
(`M::PREPAID`); the per-step path keeps it.

## W. Work accounting after soft clustering (lines 422-460, 525-531)

After soft clustering, 0.2.14 charges, in order: `v` for the retained
centroid scan; one visit per cluster plus one per face of that cluster in the
geometry phase; then `soft_count` each for the maximum, key and order scans,
2048 for the histogram, and per emitted cluster one visit plus its index
count. With contiguous clusters (`soft[0] = 0`, each cluster ends where the
next starts, the last at `faces`, so the clusters partition the faces) the
total is `remaining_charges = v + faces + 5 * soft_count + 2048 + n`, computed
with checked arithmetic; an overflow keeps the per-step path.

When positions are packed and that total fits the remaining budget, no charge
can exhaust it, so `LimitExceeded` cannot occur anywhere below, and the only
possible failures are the centroid overflow check and a cluster score. The
candidate then charges with plain subtractions that sum to what 0.2.14
charges before each of those outcomes:

- centroid overflow: `v` is charged before the check, as in 0.2.14;
- score failure at cluster `c`: `v + sum over k <= c of (1 + faces_k)`, the
  running `visits` count (line 447), which is the 0.2.14 per-cluster `add(1)`
  plus the per-face scan of each cluster up to and including `c`;
- success: `v + visits` and the emission total
  `4 * soft_count + 2048 + 3 * (visits - soft_count)`, which equals the
  per-cluster emission charges because each cluster emits `3 * faces_k`
  indices (line 456).

Packed triangle reads index the validated position slice directly; every
index is below `v` after topology validation, so the indexing cannot fail,
just as the 0.2.14 checked read could not. The destination is written only by
emission, which cannot fail on this path, so partial writes are unchanged
(none on the covered path; the per-step path is unchanged). Storage was
reserved before this point and is not touched. When the budget does not cover
the total, or positions are not packed, the 0.2.14 per-step path runs
unchanged, with only `score` (F2) shared.

Emission is the 0.2.14 code moved into `emit` (line 533), generic over the
meter: the per-step meter delegates to `Work`, the prepaid meter visits
without charging. A one-triangle cluster copies its three indices with a
constant-length copy of the same slice (line 582).

## R. Roots

No change. The std build already uses `f32::sqrt`, proved bit-identical to
the pinned backend for finite nonnegative operands (the module test and the
0.2.10 exhaustive proof); no_std keeps the cached portable backend. `score`
may now evaluate the root of a nonfinite squared norm before failing; both
backends return a nonfinite value there and the result is discarded.
