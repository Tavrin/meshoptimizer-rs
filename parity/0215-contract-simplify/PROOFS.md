# Bounds

## Positional quadric finite scan

`src/simplify.rs:2466` normalizes each finite position only after the extent and
reciprocal checks. Each normalized coordinate is in [0, 2); the zero-extent
case gives zero. If the reciprocal overflows, the original per-vertex guard
returns before quadrics are built. The positional `Q` values in `quadrics`
are constructed at `src/simplify.rs:1741` from these coordinates and fixed weights no larger than 10.

A rounded coordinate difference is at most 2 and a cross-product component
is at most 8. Edge squared length and projection have magnitude at most 12;
the perpendicular components are at most 48, so their squared norm is below
2^13. The square-root operands in both constructors are therefore finite.
For a finite vector whose squared norm is positive, each normalized component
has magnitude below 16: rounding each square to a nonnegative f32 bounds its
exact square by twice the positive summed squares, and the rounded root is
at least half the exact root. If the squared norm rounds to zero, the vector
is not divided. Thus the normalized vector is finite in either branch. The
plane offset is at most 96 in magnitude, and all fields of one triangle, edge,
or folded edge contribution have magnitude below 2^20, including rounding.

For addends bounded by B=2^20, round-to-nearest accumulation cannot leave
[-2^45, 2^45]: the outward ulp at that boundary is 4B, so an outward add of B
rounds back to the boundary; monotonicity gives the induction. This applies
regardless of the number of triangles. The regularization point uses the
accumulated nonnegative weight and a factor no larger than one, and its fields
are below 2^50; adding it and all subsequent fold contributions leaves every
positional field below 2^60. Hence the removed positional `Q::finite` test at `src/simplify.rs:1859`
cannot fail. The attribute quadric and gradient checks are retained, with the
same error ordering; computing the reciprocal remains in the same loop, and
its use and any later numerical error remain unchanged. These bounds do not
assume finite input attributes or a finite inverse weight.
