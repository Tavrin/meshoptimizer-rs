# P4 preservation arguments

The experiments change scalar code only. There is no new unsafe code, ISA
dispatch, intrinsic, FMA, approximate operation or parallel reduction. Baseline
is v0.2.15. Exact observations supplement the arguments below; they do not
establish universal coverage of all meshes or configurations.

## Rejected experiment 1: eager position equality

At the original `src/simplify.rs:1538`, both `candidate` and `v` are already
initialized three-float values. Array equality is the conjunction of the three
component comparisons. Evaluating all three comparisons and combining their
booleans with `&` gives the same boolean, including either sign of zero and
NaNs. Comparisons have no Rust-visible side effects and none can fail. The
candidate read (including its possible error and work charge) still precedes
all comparisons. Table probes, first-match choice, insertion order, wedge
construction, bounds accumulation and work accounting remain unchanged.
No float arithmetic or storage layout changes. This helper is shared by
simplify, attributes and update, including sparse and mapped position paths.

## Rejected experiment 1: grid integer conversion

At the original `src/simplify_extra.rs:1238`, the existing kernel contract is
`0.5 <= value < 1024`. The former add-2^23 implementation rounds to an integer
and subtracts one exactly when that integer is above the input, yielding floor.
On this finite positive interval the defined Rust `as i32` conversion truncates
toward zero, which is floor, without reaching either saturation endpoint.
The final `as u32` preserves a nonnegative integer below 1024. The preceding
multiply and add, bit packing, lock selection, visit order and work charge are
unchanged. No float contribution, quadric accumulation or evaluation changes.
The helper is also used by simplify_points; the exact driver therefore retains
its prune/points coverage in addition to the four measured functions.

## Rejected experiment 1: dense packed update bounds

At the original `src/simplify_extra.rs:1098`, the fast branch requires an empty
sparse map and a packed mutable position view. State::new sets vertex_count to
the view length; without SPARSE, normalized state positions have that same
length. The former loop consequently reads packed vertices `0..len` in order.
Those validated indices cannot cause InvalidLayout or IndexOutOfBounds.
The new branch visits that exact slice in the same order and keeps both ordered
comparisons per component, including zero ties. Work::scan uses the identical
per-visit add-before-callback path when the whole scan does not fit. When it
fits, the callback cannot fail, so deferring the exact total charge until after
the scan preserves both usage counters and error ordering. Bounds are local;
no caller write occurs before the next unchanged step. The sparse and general
view branches retain the original reads and charges. No allocation changes.

## Check design

`exact.rs` compares complete results, float bits, destinations including unused
tails, mutable views and both usage counters. Every work limit through success
plus one is visited on the selected small cases. Repeated calls cover cold,
warm and after-failure workspace contents; the layout/width matrix also primes
with a successful into call followed by reciprocal-overflow failure after allocation. Special coordinates, storage limits, zero/maximum
weights, all 512 option-mask acceptance results and every accepted mask on a small mesh, strided and byte-backed views,
sparse paths, duplicate/degenerate faces, welded wedges and colliding hashes
are represented. All-features and no-default binaries compare separately to
the same renamed baseline. The supplementary seeded stream has a 300 child
CPU-second budget split equally between modes. Pinned C++ protocol comparisons
include simplify, attributes, sloppy and update, with serialized error bits and
mutable output. The performance campaign separately compares exact outputs
across all six arms on all eight retained meshes.

Execution receipts and the final experiment disposition are recorded in REPORT.md.

## Retained experiment 2: reuse normalization's original-space bounds

Source anchors: `src/simplify.rs:943` (private bounds result), `:2638` and
`:2989` (compute/return), `src/simplify_extra.rs:751` (sparse propagation),
`:1103` (unchanged work visits), and `src/simplify.rs:2999` (focused test).
Shared helpers changed: private run_state/sparse_run result plumbing only;
workspace.rs, processing.rs, math.rs and every quadric operation are unchanged.

`run_state` already reduces every original-space position, in iteration order,
in `position_remap_kernel`. No caller position is modified by run_state or its
solve_state phase: solving writes the private normalized State::p only.
Passing the original lo/hi alongside the existing result and State therefore
supplies exactly the ordered comparison results that update finalization
would recompute. Equal zero operands keep the first zero's sign in both scans.
Empty inputs preserve the initial extrema in both scans. For SPARSE,
position_remap reads mapped positions in sparse order; sparse_run returns that
same ordered map in State::sparse, and the former finalization loop reads the
same sequence through it. Propagating the bounds through sparse_run is thus
also exact. Attribute and flag values do not change this read sequence.

The removed reads cannot fail: mutable position constructors validate their
layout, dense indices are below the view length, and sparse_run creates every
sparse index from previously validated topology. Caller positions remain
exclusively borrowed and untouched until finalization. The removed comparisons
have no errors or caller side effects. The next unchanged extent calculation
uses exactly the same bound bits. No float sum or quadric contribution changes.

The replacement `work.scan(s.p.iter(), |_| Ok(()))` keeps one visit per original
finalization iteration. On insufficient work its existing per-visit checked
branch preserves the exact consumed prefix before LimitExceeded. When the full
scan fits, the no-op callbacks cannot fail and LLVM may erase the traversal;
the same total charge is subtracted. There is no intervening fallible read or
caller write whose error precedence could change. Heap allocations, vector
sizes/capacities, reclamation and retention are unchanged; the extra private
return value has fixed stack size and does not enter heap accounting.

One focused test exercises original bounds after normalization/solve with dense
and sparse orders, unused extrema and opposite signed-zero ties. The exact
baseline driver then checks every work boundary, including byte-backed and
strided mutable views and sparse options, in both feature modes.

The first no-default attempt stopped in the driver while unwrapping an
experimental option mask rejected by that feature mode (UnknownFlags). No
baseline/candidate mismatch was reported. The failed exit/log and old driver
are retained. The corrected driver compares parser acceptance and errors in
both crates, then sweeps only constructible settings. This does not mask a
feature-acceptance difference or reduce work-boundary coverage of accepted masks.

## Execution disposition

The retained second experiment passes the instruction gate for update. The
corrected full supervisor passed all 19 stages: exact all/none, both seeded
streams, pinned C++ all/none, tests default/all/none, clippy, fmt, unsafe/package
boundary and public text. Exact checks: 8,970,468 all features, 5,769,468 no
default features. The latter includes feature-mask rejection comparisons;
accepted settings retain full selected work-limit sweeps. Each C++ mode passed
1,246 comparisons. Targeted child CPU was 513.063 seconds. See proof-summary.json
for source/binary bindings and proof-receipts.json for exits and log hashes.
The first attempt's no-default harness failure remains explicitly retained.
