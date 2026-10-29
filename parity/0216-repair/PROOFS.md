# Opt-in local repair proof record

Base: `1ecf69abe9f92c17e13dc38fa97be659aee44582` (0.2.15).
The repair APIs are exported ungated, like `sanitize` and `analyze_mesh`; there
is no `repair` Cargo feature (see "Check-lane amendments" below). The package
version and dependency lockfile are unchanged. All existing algorithm source
files are byte-identical to the base; `src/lib.rs` only adds the module and its
exports.
`receipts/source-binding.json` binds every existing algorithm file, the new
implementation, tests, guide, drivers and fuzz sources by SHA-256. No Git ref,
checkout, staging, commit, tag or publication operation was performed.

## API and decision contracts

All functions take `&mut Workspace` and return checked `Result<_, Error>`.
Caller-buffer variants have an initial `destination: &mut [u32]` argument.

| Operation | Remaining arguments | Owned result / caller-buffer report |
| --- | --- | --- |
| `weld_vertices` / `weld_vertices_into` | `Positions<'_>, f32, WeldPolicy<'_>` | `WeldedVertices` / `WeldReport` |
| `orient_faces` / `orient_faces_into` | `&[u32], usize` (vertex count) | `OrientedMesh` / `OrientReport` |
| `fill_holes` / `fill_holes_into` | `&[u32], Positions<'_>, usize` (max loop edges) | `FilledMesh` / `FillHolesReport` |

Each operation also has a `_bound` sizing function. Weld maps are original
indices, never compact numbering. Welding visits original vertices in order,
selects the lowest eligible representative and never uses a merged vertex as a
representative. The policy requires either position-only matching or caller
attribute equality. Distance arithmetic uses f64 on finite f32 inputs. Zero
uses numeric position equality, including signed zero. The sorted grid uses
cells at least twice the tolerance, widens extreme-ratio cells to bound keys
by 2^50, visits the fixed 27-cell neighborhood, and compares actual distances.
The cell width leaves margin for division rounding. Wider cells may increase
work, but every candidate visit is metered. The guide applies the mapping and
reruns sanitation before further topology repair.

Orientation constructs sorted undirected edge groups. Exactly-two-face groups
supply parity constraints; other groups do not connect faces. Seed faces are
selected in original order, then breadth-first traversal follows corner order.
The report lists all flipped faces, every residual contradictory edge, and
component seeds touching non-manifold edges or contradictory constraints.
Non-manifold edges never provide propagation links. Faces incident to them
can still flip through other two-face constraints; there is no outward policy.

Hole filling connects boundary edges at shared original vertices. It checks
boundary direction/degree, defective-edge incidence and disconnected face fans.
Each boundary component gets exactly one decision: too large, ambiguous,
non-simple, or filled. Simple loops are projected onto the dominant area-normal
plane and ear-triangulated in deterministic order with opposite boundary winding.
Nonplanar loops beyond the documented relative 1e-6 plane tolerance are refused.
No vertices are inserted, moved, compacted or interpolated. Original triangles
are copied unchanged and fills are appended. Predicates use f64, not adaptive
exact arithmetic. There is no intersection test against other mesh surfaces,
T-junction splitting, outward orientation or watertightness guarantee.

This is an independent implementation. Ear clipping cites David Eberly's
published algorithm description in rustdoc. No GPL implementation was opened
or copied. The three source-only fuzz targets were added and formatted; they
were deliberately neither built nor run in this lane.

## Resource and failure argument

Every dynamic array uses the existing fallible `Budget` allocator, including
reports, remaps, sorted grid/edge entries, component queues and union-find
scratch. Size arithmetic is checked before allocation. Existing retained
workspace storage participates in the budget and can be evicted under pressure.
Repair scratch itself is call-local. Usage records peak capacity and charged
work on both success and failure. Dense weld cells and large/concave loops
can consume substantial work; counted limits remain effective.

Validation, sorting/search bounds, grid candidates, edge groups, graph visits,
union-find parent visits, polygon intersections, ear candidates/containment
and emission are charged. All caller-buffer writes occur only after complete
planning, capacity validation and emission work reservation. No fallible action
follows the first destination write. Hence every returned error leaves the
entire caller buffer unchanged, and successful writes preserve its tail.

## Retained results

`receipts/exits.tsv` contains exit 0 for every gate and exact/corpus step.
The complete logs are retained with local filesystem prefixes redacted.

- `test-all`: all features, including 22 focused repair tests, passed.
- `test-none`: default features disabled, existing API tests passed.
- `test-repair-no-std`: default features disabled with repair enabled, including
  the same 22 repair tests, passed.
- `doctest-all` and `doctest-repair-no-std`: 36 and 33 passed respectively;
  five pre-existing documentation examples remain ignored in each mode.
- `clippy-all` and `clippy-repair-no-std`: all targets with `-D warnings`, passed.
- `fmt`, unsafe source/package `boundary`, and `public-text`: passed.

The 22 focused tests cover lowest representative/non-transitivity, seam equality,
T-junction-free within-tolerance crack welding followed by sanitation, exact-zero
position grouping, extreme/subnormal coordinates, a seeded exhaustive weld oracle,
Euclidean/inclusive distance boundaries, nonpacked input, invalid tolerances and
non-finite positions; a flipped patch, independent seeds, non-manifold adjacency
barriers and a half-twist Möbius-like strip; a cube missing its top face, inclusive
hole-size limits, deterministic concave ears, crossing/touching/coincident/zero-area
loops, branched/inconsistent/nonplanar boundaries and a boundary touching a
separate closed fan; malformed topology, unrepresentable vertex counts without allocating,
output/tail identity, and limits.
All hand-authored fixture geometry in `tests/repair.rs` is dedicated to CC0-1.0
by its contributors; no external asset is redistributed here.

Both owned and caller-buffer forms sweep **every work limit from zero through
success + 1** and byte limits at zero, success footprint - 1, exact footprint,
and footprint + 1. Repeated successes/failures and a retained-cache workspace
followed by an injected buffer failure verify deterministic recovery. Failing
caller-buffer calls retain their sentinel destinations exactly.

`exact.rs` compares base and candidate at 40 prerequisite API/policy combinations:
all 16 sanitation policies, all 16 caller-buffer sanitation policies, four
analysis/list modes, both position-remap forms, and both cache-optimization forms.
Nine fixtures cover empty, valid, flipped, duplicated/reversed, repeated-index,
invalid-index, incomplete, signed-zero/split-index and non-finite cases. Every
work limit through the observed maximum + 1 and byte boundaries are checked.
Each comparison covers cold, repeat, an injected small destination, and retry;
results/errors, reports, complete destination including tails, and both Usage
fields must match. **68,872 exact observations per mode**, **137,744 total**,
passed against the base with all features and with no default features. The new
repair feature is enabled on the candidate in both modes. New APIs have no base
counterpart; their behavioral/oracle/atomicity tests provide their evidence.

`corpus.rs` checks **45 shared generated runner meshes per mode**, including the
million-index grid, shared/split-attribute spheres, duplicate triangles and a
non-manifold-edge mesh. It checks zero-tolerance grouping against
`generate_position_remap`, no flips on consistent manifold-edge input, a no-op
second orientation pass where constraints are consistent, unchanged original
triangles after filling, and original vertex bounds for every added index.
`receipts/corpus.json` binds all generated inputs; no external-real-asset or
self-intersection claim is made. Boundary/exact cases are deliberately small;
the entire validation campaign completed well below two CPU-hours.

`run.sh` reproduces the gates and read-only base archive under a caller-supplied
CPU-only admission wrapper and dedicated build target. No queue was canceled.
The dedicated target was removed after extracting the retained receipts; see
`receipts/cleanup.json`. No timings or performance improvements are claimed.

## Check-lane amendments

These changes were made after the first receipts. `run.sh` was rerun in full on
6106b11, so `receipts/source-binding.json`, `exits.tsv` and the logs now
describe 6106b11. (The binding records `PROOFS.md`'s own hash, so it matches
the file as of the commit that refreshed the receipts.) The `*-repair-no-std`
logs and `summary.json` counts are from the earlier amended-tree run and were not
regenerated. Fuzzing: `receipts/fuzz.json`.

**Feature removed.** The `repair` Cargo feature no longer exists. The APIs are
exported unconditionally from `src/lib.rs`, like `sanitize` and
`analyze_mesh`; every item's rustdoc still says it is not in C++ meshoptimizer.
`tests/repair.rs`, the guide recipe, `run.sh` and the fuzz manifest no longer
name the feature. With no default features, the repair tests now run in the
`test-none` gate.

**No `unwrap` in library code.** Each former call is now a `?` on an error
that cannot occur, so no reachable behaviour changed:

- `weld_plan`, three reads of `positions.get(i)` / `get(j)` with indices below
  `positions.len()`: the validation scan reads every such index first and
  returns `InvalidParameter` if a read is `None`. `Positions::get` is pure, so
  the later reads are `Some`. The fallback error is the same `InvalidParameter`.
- `fill_plan`, the finiteness scan over referenced positions: `edges` has
  already returned `IndexOutOfBounds` for any index `>= positions.len()`, and
  a public `Positions` returns `Some` below its length. Fallback:
  `IndexOutOfBounds`, the error `edges` would give.
- `fill_plan`, the outgoing edge on the loop walk: a component reaches the walk
  only if every vertex has exactly two boundary ends and exactly one incoming
  end, so it has exactly one outgoing edge, which `find` returns. The walk
  starts in the component and only follows its edges. Fallback:
  `InvalidTopology`.
- `triangulate`, the position closure: polygon vertices are input indices that
  `edges` bounds-checked. Fallback: `IndexOutOfBounds`.

**Work charged before allocation.** As in `sanitize`, the work for filling and
sorting the weld grid, the edge list, fill's boundary scan and endpoint list,
and orientation's group scan is now charged before the matching buffer is
reserved. Totals are unchanged; a tight work limit now fails before that
allocation rather than after it.

**Attack tests added.** `weld_cell_boundaries_match_oracle_for_nondyadic_tolerance_and_offsets`
places points on and one ulp either side of multiples of non-dyadic
tolerances, across zero and at offsets up to 1e6, and compares with the
exhaustive greedy oracle and, at tolerance 0, with `generate_position_remap`.
`orient_is_noop_on_closed_cube_and_stays_within_components` checks a closed
consistent cube is unchanged and that a flipped patch in a second component
is the only face flipped.
