# Equivalence arguments for SPEC 0217 (simplify_with_attributes)

No check is removed. Attribute gradients divide by the squared area of each
triangle (`src/simplify.rs` `attribute_quadric`), so finite, validated inputs
can still produce infinite or NaN attribute quadrics; no static bound exists
and every finite test stays. Each change below computes the same predicate
with fewer instructions, or skips per-visit bookkeeping whose total is
already known, and hands control back to the unchanged loop at the first
input that loop would reject. Line numbers refer to the candidate.

## Meter capability

`Meter::PREPAID` (`src/simplify.rs:51`, implementations at :61 and :81) is
`true` only for `Prepaid`, whose `add` is an unconditional addition that
cannot fail. `metered!` selects `Prepaid` only after `Work::covers` proved the
phase's upper bound fits the remaining budget, and charges the tally once at
the end of the phase or at its error. Under `Prepaid`, the order and grouping
of charges within a phase are unobservable; only their sum is.

## Validation prefix

`validate` (`src/simplify.rs:564-577`). The per-vertex loop that follows is
unchanged. For vertex `i` it charges one visit, rejects a non-finite position
with `InvalidParameter`, then scans the attribute record, charging one visit
per component and rejecting the first non-finite component. It has no other
side effect.

The new prefix applies only when positions are packed and unmapped
(`Positions::packed_values`) and attributes are an unmapped float view whose
stride equals its width (`Attributes::contiguous_floats`, `src/input.rs:980`).
There, record `i` is `records[i * width..][..width]`, the same slice
`float_record(i)` returns, and position `i` is `values[i]`, the value
`p.at(i)` returns. The prefix also requires `work.covers(n * (width + 1))`.

`first_nonfinite_vertex` (`:2428`) returns the smallest `j` such that vertex
`j` has a non-finite position or attribute component, or `n`. The predicate
`nonfinite` (`:2417`) tests whether all exponent bits are set, which is
`!is_finite()`. For every `i < j` the original iteration succeeds and charges
exactly `width + 1`. Because the whole loop's charge fits the budget, none of
those charges can raise `LimitExceeded`. `add_covered(j * (width + 1))` leaves
`remaining` equal to its value after the original `j` iterations, and the
loop resumes at `j`. It therefore raises the same error, after the same
partial charge, or succeeds with the same total. When either condition fails,
`start` stays 0 and the loop is the original.

## Normalization prefix

`run_state` (`src/simplify.rs:2658-2677`), `normalize_prefix` (`:2446`),
`normalize_rows` (`:2512`), `normalized_position` (`:2493`). The per-vertex
loop is unchanged and resumes at `start`.

The prefix applies only under `Prepaid` (`prepaid(work)`, `:2437`), when
every weight is positive (`dense`, `:2603`: `ac == weights.len()`; validated
weights are finite and nonnegative, and `-0.0` is not positive), when
attributes are contiguous floats, and when positions are cached in `s.p` or
packed. For vertex `i` the original iteration:

1. charges one visit;
2. reads the cached coordinates from `s.p[i]`, or `positions.at(i)`, which is
   `packed[i]` for packed unmapped positions;
3. writes `(p - lo) * scale` to `s.p[i]`;
4. if the scale is not finite and a written coordinate is not finite, returns
   `NumericalFailure`;
5. in `normalize_attribute_row`, reads `float_record(i)`, the contiguous row.
   With every weight positive it computes `value * weight` per component in
   order and returns `NumericalFailure` at the first non-finite product.
   Otherwise it writes the product to `s.a[i * ac + k]`, and the scan charges
   one visit per component.

`normalize_prefix` computes the same coordinates and products with the same
IEEE operations. It writes them only when step 4 and step 5 would both pass,
and stops at the first vertex where one would fail, without writing that
vertex. The fixed-width instances (1 to 8 components) and the general loop
compute the same values; only the unrolling differs. `run_state` then charges
`start * (weights.len() + 1)`, the sum of the skipped iterations' charges,
which is safe under `Prepaid`. The resumed loop repeats the original
iteration for the stopping vertex, so its writes, error and partial charge
are unchanged. Values written to `s.p` and `s.a` for accepted vertices equal
the original writes. No other state is touched.

## Quadric tables

`State::quadrics` (`src/simplify.rs:1899-1915`). The original returned
`NumericalFailure` if any attribute quadric had a non-finite field among its
eleven accumulated fields (`Q::finite`, now written through `Q::fields`,
`:759`), or any gradient component was non-finite. Both scans are free of
side effects and run at the same point, before the same return.

The replacement sums `x * 0.0` per field lane over all records. For finite
`x` the product is exactly a signed zero, and a sum of signed zeros is a
signed zero. For infinite or NaN `x` the product is NaN, and NaN persists
through addition. A lane is therefore NaN exactly when one of its values is
non-finite. Lane 11 of the quadric fold adds the constant 0, so
`inverse_weight` stays outside the predicate as before. The error, its
position and all state are unchanged. Only the early exit within the scan is
gone, and it affected nothing but time.

## Positions and options

Plain `simplify` (no attribute view) never enters either prefix. The sparse
path (`SimplifyOptions::SPARSE`) passes through `validate` but leaves
`run_state` before the normalization prefix. Every caller that reaches
`validate`, the normalization loop or `State::quadrics`, including
`simplify_with_update`, gets the same behaviour: the arguments above do not
depend on the caller.
