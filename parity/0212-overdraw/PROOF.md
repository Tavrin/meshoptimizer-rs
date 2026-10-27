# Equivalence argument

## Fixed-record validation

The layout constructor already validates offset, stride, count and every XYZ
boundary. The fixed-record iterator is available only for an unmapped,
little-endian byte view of stride 64 whose full `count * 64` bytes exist after
the offset. Checked multiplication and slicing select the original reader on
overflow or absent padding. It borrows existing input without allocation.

For every vertex i, both readers decode exactly bytes `offset + 64*i .. +12`,
in the same XYZ order and byte order. Padding is ignored. The original
`chunks(64).take(count)` and the new fixed-record iterator both have exactly
count elements. Exposing ExactSizeIterator therefore leaves Work::scan's count,
covered-path choice, per-vertex exhaustion checks and callback indices equal.
On a numerical error at vertex k, both charge the same k+1 validation visits;
earlier topology and threshold errors still precede this traversal. No new
work reservation or bound is introduced.

Each successful callback performs the same three additions in the same vertex
and coordinate order. Overflow of the accumulated centroid remains unchecked
until the original kernel check. In particular, an empty triangle list still
succeeds if finite supplied vertices overflow an unused centroid sum.

## Finite predicates

The original predicate requires all three already-loaded coordinates to be
finite, returning NumericalFailure before centroid addition otherwise. D takes
the Boolean conjunction of the same three pure is_finite classifications with
`&`. E clears each sign bit, takes their unsigned maximum and compares against
infinity's bit pattern. The latter works because finite magnitudes have bits
below `0x7f80_0000`, infinities equal it, and every NaN payload exceeds it.
Both predicates are equivalent for every f32 bit pattern, including both zeros,
subnormals, both infinities and signaling/quiet NaNs of either sign. Neither
changes a read, charge, arithmetic operation or output write around that exit.
Only the measured selected predicate is retained in the candidate.

The input-validation and shared-reader variants leave all area, normal,
centroid, threshold-target and score arithmetic guards unchanged. Finite inputs
alone do not establish finite intermediates. The
scratch A/AB ablations are attribution evidence only.

## Shared indexed-record bounds experiment

F converts each triangle index to usize exactly as the original lookups do,
then checks their maximum against the fixed-record slice length. The maximum
is in bounds exactly when all three indices are in bounds. All three decoded
records remain in the original order. On any bad index, both paths return
IndexOutOfBounds before invoking the callback, after the same per-triangle
Work::scan charge. Records are `[u8; 64]`, proving each `[u8; 12]` XYZ prefix
exists. The generic mapped/short/other-layout readers remain unchanged.

The pre-existing input unit sweep compares this reader with individual at()
lookups on packed/interleaved/full/short/endian/mapped views, invalid triangle
indices and all small work limits. The public differential sweep additionally
compares every entry point's outputs, errors and Usage against v0.2.11.
F remains an experiment unless selected by its pilot and final qualification.

## Required execution evidence

The source argument is supplemented by the gates in gates.sh, the independent
exact consumer, C++ differential, widened CPU-budgeted differential fuzz, and
fresh four-arm instruction campaign. The final REPORT.md and proof-exits.tsv
record executed outcomes; this argument alone is not a passing gate receipt.

## Reciprocal and normalization guards experiment

G removes only checks whose errors are unreachable by the following bounds.
No square-root backend changes. Both existing backends are IEEE correctly
rounded on the nonnegative operands used here. Let u = 2^-149, the smallest
positive f32. A nonzero root of a nonnegative f32 is at least 2^-75; every
finite such root is at most 2^64. Its reciprocal is therefore finite and normal.

Triangle areas are roots of sums of squares, hence zero, positive, infinity or
NaN. Infinity/NaN cannot disappear from their nonnegative accumulation. After
the existing finite(area_sum) check, a positive area sum is at least one
positive area, hence at least 2^-75. Its reciprocal cannot overflow. The zero
branch still returns zero. Removing its reciprocal guard removes no reachable
error or work charge.

For normals, the existing finite(sum of three squared components) check
remains before the root. Call the sum S. It proves each component and each
rounded square q finite. Nonnegative floating-point addition is monotone,
so S >= each q. If S=0, the inverse remains zero and every product is finite.
If S>0, each exact component square is at most 2*S: for a rounded-zero square
it is at most u/2 <= S/2; otherwise round-to-nearest gives square <= 2*q.
The rounded root L is at least sqrt(S)/2 (a deliberately loose bound), and
the finite rounded reciprocal I is at most 2/L. Thus each exact |component*I|
is at most 4*sqrt(2), and its rounded value is below 16, far below overflow.
Consequently the three normalized-component finite guards cannot fail.

The normal-length root and its reciprocal stay normal, so the loose factor-two
rounding bounds for normalization do not depend on subnormal relative-error
assumptions. The area-sum reciprocal may be subnormal, which is finite and is
retained bit-for-bit. The area-sum, centroid-product, squared-norm,
centroid-difference and final-score guards remain. Every reachable numerical
error therefore still exits at its original check, with identical work and
partial writes. The exact std/no_std sweep and widened fuzz must still pass.

## Bounded-coordinate scoring experiment H

H selects its bounded kernel only if the existing validation scan proves every
coordinate's magnitude at most P=2^30. The bound comparison also proves each
passing coordinate finite; a failing comparison still performs the original
finite check before the same centroid addition. Large finite values choose
the general path; nonfinite values fail at the original vertex. All supplied
vertices, including unreferenced ones, participate in the classification.

Useful accumulation lemma: if every signed addend has magnitude at most a
power of two B, and A=2^25*B is finite, round-to-nearest addition preserves the
interval [-A,A]. At A the outward ulp is 4*B, so fl(A+B)=A; symmetrically
fl(-A-B)=-A. Monotonicity gives the induction for every prior value inside the
interval. This bound does not assume a small number of additions or a small
accumulated relative error.

With |coordinate|<=2^30, each rounded edge is at most 2^31, each cross component
at most 2^63. Each rounded squared component is at most 2^126, and
their sum is at most 3*2^126, which is representable and below f32::MAX. Its area is finite
and at most 2^64. The nonnegative area sum is at most 2^89 by the lemma. Raw
normal components are at most 2^88 and finite. Their squared sum can overflow:
the original finite check on that sum is retained, in the same position.

A triangle's coordinate sum has magnitude at most 2^32. Let M be the largest
positive area in a cluster and U its power-of-two upper bound, with
M<=U<2*M. Each weighted centroid addend has magnitude at most B=2^32*U,
including rounding, because fl(area/3)<=area<=U and the bound B is representable.
The lemma bounds each raw centroid by 2^25*B <= 2^121. The area sum is at least
M, and its rounded reciprocal is at most 2/area_sum. Therefore each exact
normalized-centroid product has magnitude below 2^59 and its rounded value
is at most 2^60. If all areas are zero, the raw centroid and normalized
centroid are zero. No centroid-product error is reachable in either case.

The mesh-centroid sum is at most 2^55 by the same lemma and division by the
positive vertex count cannot increase that bound. Each centroid difference
is at most 2^61. The already-proven normalized normal components are below 16,
so the final three-term dot product is bounded by 2^67 and finite. Thus the
bounded path's removed area-sum, centroid-product, difference and final-score
checks cannot fail. All arithmetic operations retain their original order.
No overflow-capable normal-squared-sum check is removed or moved. Large finite
coordinates retain the general guarded score kernel.

The exact sweep must straddle P by one ulp, cover unused outlier vertices and
all entry points/work prefixes, and include enough bounded repeated faces to
overflow the accumulated normal's squared norm. The widened fuzz generator
must reach both bounded and general paths. At P, two repeated stress faces succeed, three overflow the squared-norm sum,
and four overflow individual squares; every work prefix is swept. H is
unqualified until those gates
and the frozen final instruction gate pass.

## Prepaid target check experiment I

The existing prepaid soft-cluster path is selected only when the exact
hard_count + 2*faces work reservation fits and |threshold|<=f32::MAX/4.
Each hard cluster is nonempty. Its exact miss count is at most three times its
face count, and both counts fit u32 by the existing topology size guard.
Integer-to-f32 rounding changes either nonzero count by at most 2^-24 relative;
rounding their division adds at most another 2^-24. The rounded ratio is
therefore strictly below four. Multiplication by a threshold of magnitude at
most MAX/4 has enough margin to stay finite under rounding. The target check
cannot fail on ClusterPrepaid. Work retains the original finite check, needed
for thresholds outside the admitted magnitude. Every work charge, scan and
reservation remains unchanged; this introduces no new work bound.

## Eight-record validation experiment J

Only unmapped, fully padded, little-endian stride-64 records use this path.
The iterator borrows fixed groups of eight records plus a tail of at most
seven; decoded XYZ values retain their vertex and coordinate order. Padding
is ignored. Other layouts use the existing checked reader.

A pure integer reduction over the group's 24 f32 bit patterns proves all
magnitudes <=P. Sign-cleared IEEE bits are ordered by magnitude; NaN/infinity
are above the finite bound. This reduction cannot return an error or perform
floating arithmetic, even if the work budget is already exhausted. A passing
group is then processed by Work::scan with length eight and unit one. Its
callback performs exactly the original centroid additions and cannot return
an error. If eight units fit, the exact covered bound is eight visits. If they
do not, the existing Work::scan fallback charges one before each callback and
fails at the original vertex. This introduces no global reservation formula.

A rejected group uses the original checked per-vertex callback: a numerical
error at offset j charges exactly j+1 visits, or the original earlier work
failure occurs. Previously completed groups charged exactly their lengths,
so the total prefix is identical whether the original whole-input scan would
have been covered or staged. The tail uses the same checked callback. Large
finite values still clear the whole-mesh bounded flag; admitted later groups
do not reset it. Every centroid addition remains in its original order,
including deferred overflow checks and empty-mesh behavior. There is no heap
allocation and the only extra storage is a fixed 24-float group.

The exact sweep exhausts budgets on record counts 7/8/9/15/16/17 and every
nonfinite coordinate position across 17 records. Those cases cross both group
boundaries, all tail lengths of interest, numerical errors and work failures.
The widened differential fuzz already generates 1..96 vertices in full and
short byte layouts, both bounded and general geometry, so it reaches this path.

## Packed-path isolation and rejected source identity

The third rejected source delegated packed calls to a source-identical private
module. Its full behavioral proof passed, but instruction counts regressed;
source identity did not establish instruction identity. Those receipts remain
rejected.

The current pending repair retains the original packed implementation at its
original module path and restores its original input reader. The three public
entry points inspect only the storage tag, delegate views to a private fast
module, and otherwise execute the original implementation. The selector
allocates nothing, charges no work and cannot fail. A single workspace
measurement still begins/finishes inside the selected operation. The packed
counter precheck and full differential/counter campaign have not run on this
repair. Its source argument is not a qualification receipt.
