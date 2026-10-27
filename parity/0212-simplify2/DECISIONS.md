# Decisions and proof boundaries

The entry is c396a697, treated as post-#39 main. The v0.2.11 tag peels to
40dbf9e. Sqrt is explicitly outside this job; its #39 implementation is kept.
No API, version, shared-math, unsafe, SIMD implementation or unrelated family
changes are included.

Attempt 1 adds adjacency/quadric/normalization/update release bounds, bounded
hash batches, fixed-length remap/counter/wedge slices and the validated bounds
scan. It is retained in full under `<scratch>/meshopt-0212-simplify2/attempt1/`.
Its completed 214e stages do not meet the registered PARTIAL threshold. This
motivates a refinement; no sample is selected or replaced in place.

The refinement adds bounded attribute ranking and finite-predicate reductions.
For each ranked candidate, one initial visit and 2ac base visits are followed
by either 2ac seam visits or at most two wedge cycles. Each wedge is part of a
permutation of n vertices, so a traversal excluding its starting vertex visits
at most n-1 entries. The conservative bound 1+2ac*(n+1) also covers the seam
branch. Batches contain at most 64 candidates. The insufficient-budget path
retains the original charge before each side effect and potential error.

For normalization, validate has already checked all coordinates and bounds has
checked finite extent. Monotonic correctly rounded subtraction gives
0 <= round(p-lo) <= extent (signed zero does not affect finiteness). If the
rounded reciprocal is finite, extent times that reciprocal is below 2,
including subnormal extent/reciprocal rounding; normalized products cannot
overflow. A zero extent uses zero scale. Only this proven path omits the
per-coordinate guard. If the reciprocal overflows, the original guard executes
after the same vertex charge/write; it is not an early scale error. Exact cases
straddle this boundary at operand bits 0x00200000 and 0x00200001 and include the
minimum subnormal under packed, little-endian and big-endian byte readers.
The exact proof also sweeps all 0x7f800000 nonnegative finite extent patterns,
checking extent times its finite reciprocal is finite and below 2. Black-box
operands/results prevent constant folding; this sweep is outside timing arms.

Other finite predicates are unchanged. Independent is_finite predicates use
bitwise boolean reductions so the compiler can combine checks. These operations
have no side effects, charge no work and issue no errors individually; the
same enclosing guard returns the same error at the same point. Arithmetic
order, NaN/overflow handling, cached reciprocals and special-value root behavior
remain unchanged. No finite-input assumption is used to drop quadric, gradient,
ranked-error or update guards.

All bounds use checked arithmetic. Overflow declines the covered path. A
covered release tallies and settles the actual visited prefix on success/error;
the original low-budget path remains responsible for exact exhaustion and
partial writes. No reservation is converted into consumed work.

The exact oracle is the separately named but source-identical v0.2.11 package.
It compares Result/error float bits, full destination words including unused
tails and failure prefixes, updated coordinates/attributes including byte-view
padding, and complete Usage, on cold and reused workspaces. Worst cases include
hash collisions, batches crossing vertex 64, high valence, reversed/duplicate
and degenerate triangles, repeated welded wedges, permissive classification,
32 attributes, all option masks, and exhaustive small-mesh work budgets. Fuzz
also preserves difficult meshes and forces reciprocal-boundary cases, rather
than depending on random raw floats to reach them.

Every attempt's raw machine-specific logs and admission receipts remain private
in scratch. Public scripts obtain local paths from environment variables and
public bindings replace paths with placeholders. Only final, complete proof
and fresh four-arm samples may support the final report. The 0.80 / 0.95 /
1.01 gates are unchanged.

Final decision: keep the fully qualified candidate. All three real chains meet PARTIAL; the PASS target is unmet. Worst individual post39 ratio 1.000001641; no rejected case, threshold change or favorable-sample replacement. Complete proof, four-arm measurement and audit exited 0.
