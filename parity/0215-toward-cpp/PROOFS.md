# Contract-preserving changes

Baseline is v0.2.14. No finite arithmetic check is removed: the existing
moderate-input proof and all unbounded/nonfinite failure paths are retained.
No public API, allocation size or byte-accounting formula changes.

## Cache score visits (W)

`src/cache.rs:269`, `src/cache.rs:652`: the existing whole-kernel dispatch
bound is `faces * (22 * max_live + 38)`, computed with checked arithmetic.
An emission visits at most 16 old cache entries, then three removals of at
most `max_live` entries, then at most 19 score entries with at most
`max_live` adjacent faces each, plus the emission and bounded cursor search.
This bound already covers the `write` score visits. The covered monomorph
can tally those `write` visits once because there is no fallible operation
in the body; the adjacency scans' closures return only `Ok`. Its total
cannot exceed the reserved bound, hence cannot overflow `usize`. The
limited monomorph's tally is empty and its `visit` retains the original
charge before each mutation. Reservation/refund and the final `Usage` are
unchanged. Repeated triangles and high-valence fans are in `exact.rs`.

## Meshlet roots (R)

`src/meshlet.rs:19`, `src/meshlet.rs:30`: an unsigned f32 bit pattern no
larger than `0x7f7fffff` has sign zero and a non-all-ones exponent. Thus it
denotes a finite nonnegative number, including positive zero and subnormals.
For those inputs the native std root and pinned libm root both produce the
correctly rounded IEEE-754 square root, bit-identical under the ordinary
round-to-nearest, gradual-underflow environment. This is the same domain as
the retained 0.2.10 exhaustive root experiment cited by the baseline's
`parity/0215-contract-meshlets/PROOFS.md`; that exhaustive experiment is not
rerun here. Negative zero, negative values, infinities and NaNs still use
the pinned memo, preserving its bit behavior. no_std always uses it.
The eight-lane guard requires every operand to satisfy the same predicate;
otherwise the entire original eight-lane function runs. No work charge or
validation moves. The wrapper is local to the greedy/flex builder;
the spatial builder is in its own unchanged module.

## Exact roots instead of root brackets

`src/meshlet.rs:671`, `src/meshlet.rs:972`: std bypasses the conservative
root-bracket shortcuts. Each shortcut skipped a candidate only when its
rounded root could not improve a strict-less selection. Computing that exact
root yields the same decision. Bounded candidate scoring evaluates the same
expression and exact accumulated normal as `resolve`; its root is bit-identical
by the preceding proof. Selection remains lexicographic in priority and
score. Listed candidates break exact ties by the existing earliest-local-
vertex/adjacency visit order, matching `resolve`; direct candidates retain
their traversal order. The bounded domain excludes nonfinite scores and
zero radius; the existing checked/unbounded path retains NaN handling.
These shortcuts never charged work, so removing them changes no charge.
no_std keeps the original shortcuts and bracketed selection.

## Cluster-LOD phase work (W)

`src/clusterlod/support/workspace.rs:258` supplies `Meter`, its checked `Work`
implementation, local `Tally`, and the `metered!` dispatch. Dispatch only uses
a tally when a checked `usize` upper bound converts to u64 and fits remaining
work. Overflow or insufficient budget selects the original checked path;
the coverage test consumes no work and produces no new error. Tally additions
cannot overflow because every prefix is at most that bound. On success or
an arithmetic error the exact visited tally is charged once before propagating
the original result. No callback can observe the local work counter inside
these private phases. The bound guarantees that this final charge succeeds;
the checked path retains every original charge/error/partial-write ordering.

`src/clusterlod/support/simplify.rs:695`, `:1677`: adjacency clears/counts
`vertex_count` entries, then visits the input indices twice. The bound is
`vertex_count + 2 * index_count`, with checked multiplication/addition.

`src/clusterlod/support/simplify.rs:1102`, `:1679`: collapse picking charges
one per visited triangle edge, at most `index_count`. Its capacity stop can
only reduce this. Taking the existing collapse vector into a local and
restoring it on every result neither allocates nor changes its capacity or
partial prefix. Borrowed local u32 tables give the same indices as `get`'s
u32-to-usize conversion on the supported 32/64-bit address spaces.

`src/clusterlod/support/simplify.rs:1490`, `:1698`: updating visits each of
`p.len()` vertices once, and for each moved vertex with attributes charges
`ac` more; then it visits each forward and backward loop entry once. Thus
`p.len() * (1 + ac) + forward.len() + back.len()` bounds all charges. Existing
quadric finite checks remain at the same positions, including on failure.

`src/clusterlod/support/simplify.rs:1463`, `:1699`: filtering visits each
triangle once at charge three, at most the current index count (divisible by
three by topology validation). Mapping and compaction use identical stored
u32 values; writing never overtakes the reading cursor. The limited path
still charges three before each triangle's partial output writes.

Connected, repeated, coincident and large/nonfinite geometry, width 1/3/8/32
attributes, vertex locks, both LOD defaults, individual config options and
callback failure are exercised by `geometry-exact.rs`. Each selected case
sweeps every work limit 0 through unrestricted usage + 1 in both feature
modes; the inherited meshlet driver adds disconnected broad KD traversal.

## Covered KD search record (check lane, campaign 2)

`src/meshlet.rs` `nearest` now routes the covered moderate case
(`!CHECK`, budget covers `2 * nodes.len()`) to `Covered::nearest` instead of
`nearest_inner::<false, true>`. Attribution: in the campaign-1 candidate this
monomorph was 66% of `meshlets` instructions on real-b6fd; the recursion
passed nine arguments (five on the stack, spilled and reloaded per call) and
paid two index checks per leaf item, where C++ `kdtreeNearest` passes eight
and checks none.

Equivalence, statement by statement against `nearest_inner::<false, true>`:

- Same visit, node read, leaf/inner split, leaf loop (EMITTED skip, `inactive`,
  `PRUNE` test, `moderate_root`, strict `distance < limit` update), inner
  `delta`/`first`/`second`, both-children-inactive `set`, recursion order and
  `delta.abs() <= limit` test. `result` and `limit` live in the record during
  the search and are written back on every exit; the caller discards them on
  an error (`?`), as before.
- Work: the original charges `add_covered(1)` per visited node and
  `add_covered(count)` per leaf; the record adds the same counts to `visits`
  at the same points and the wrapper charges `add_covered(visits)` once on
  every exit. `add_covered` is a plain subtraction under the same covering
  bound, and nothing in the covered moderate search reads the budget, so
  `Usage.work` and the exhaustion point (none: covered) are unchanged. The
  only fallible statements are `Node::set(axis, 0)`, which cannot fail
  (`0 <= 0x3fffffff`); both versions count before them anyway.
- Leaf indexing: `cones` and `emitted` are resliced to their common length
  per leaf, so one bound check serves both lookups. KD items index both
  per-triangle tables (built over `cones`; `emitted` has one byte per
  triangle), so neither version can panic; no output, error or write changes.

No unsafe, no API change, no change to `CHECK` or uncovered paths. no_std
keeps the pinned memo root and pruning through the same record field.

## Scope and shared helpers

Shared-helper changes are limited to the private cluster-LOD snapshot's
`support/workspace.rs` (Meter/Tally/macro) and `support/simplify.rs` (four
metered phases and local table/vector borrows). The campaign-2 change is
local to `src/meshlet.rs`. Main `workspace.rs`,
`processing.rs`, `math.rs`, SIMD modules and all other function families
are unchanged. Vertex decoding has no retained source change.
