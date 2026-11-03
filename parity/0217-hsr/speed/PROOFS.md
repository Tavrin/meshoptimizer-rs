# Conservative software visibility speed follow-up

This record supersedes the outside-view implementation described in the parent
proof record. The parent receipts and `before.rs` retain the original F8 source
and evidence. Finite cameras still use its BVH and barycentric sample rays.
The extension remains always available, without dependency or feature changes.

## Coverage and depth argument

The 26 directions, original ordinal duplicate policy and stable output order
are unchanged. Each view fits the referenced bounding box inside a square of
width twice its longest side. `pixel_size=0` selects side/128; positive sizes
select ceil(2*side/pixel_size), bounded to 1..=2048 without silently coarsening
requests above that bound. Invalid requests fail atomically. All storage and
work use the existing Budget/Workspace, including buffer initialization,
projection visits, rectangle visits and bounded union certificate attempts.

Screen coordinates have eight fractional bits. Edges use bounded i64
arithmetic: the largest coordinate is 2048*256, so products cannot overflow.
Each true projected vertex differs by less than one subpixel plus floating
roundoff. Strict inward edge tests subtract an error margin of 8*grid-width+16.
A square is an occluder only if its entirety lies inside one triangle, or inside
a union of at most eight triangles sharing original 3D edges. Edges are
normalized to counterclockwise screen order. Internal edges cancel only when
there is exactly one reversed match and no same-oriented match. Every remaining
external edge must strictly contain the square; incomplete fans and holes fail.
Candidates are ordered by their conservative cell depth; missed combinations
only retain more geometry. A 3D area upper bound excludes triangles that cannot
participate in any certificate, avoiding repeated projections of tiny faces.
No global occluder scan occurs inside a triangle query.

Each item cell stores the greatest certified lower depth and the ordinal that
provided it. The depth plane interpolates original depths at quantized screen
vertices. Its error at an actual triangle point is bounded by |dx|+|dy| through
barycentric interpolation. Cell extrema expand the half-width from 128 to 130
subpixels; original triangle depth extrema further tighten these bounds.
A shared union uses the minimum lower bound of all participating triangles.
Edge-on triangles query an outward-expanded pixel by clipping a bounded
original depth-carrying polygon; their global extrema would otherwise be too
loose. The clipping uses two twelve-vertex stack arrays, never heap storage. A query uses
an outward rectangle, outward edge overlap and an upper depth bound. Its bound
also receives the caller tolerance, the original numerical guard and 1e-12*side.

If any actual point of a nonduplicate triangle is visible from a tested view,
its outward cell overlaps the triangle and no certified occluder can be nearer
than that point. Its upper bound therefore passes and retains the whole
triangle. This includes every visible lattice sample and every visible region
of at least one requested pixel-area, plus smaller slivers. Uncertain coverage,
edge-on projections and contacts retain geometry. Degenerate 3D triangles stay.
The argument uses ordinary guarded f64 predicates on finite f32 input; it does
not claim adaptive exact geometry, visibility between views, transparency,
watertightness or continuous equivalence after removal. Same-winding duplicates
remain the explicit exception, retaining only the earliest copy.

The buffer uses contiguous 8x8 pixel tiles. Views run serially even with the
existing parallel feature; pool size cannot change traversal or reduction order.
The direction loop ends only when all nonduplicate triangles have been retained.
Raster work is O(n log n + views*(n + raster rectangle area)); resolution is
bounded independently of n. A restore-only BVH compatibility pass tests just
raster-hidden candidates with the original lattice predicates. These ordinary
f64 rays can miss exact shared-edge hits; preserving such apparent visibility
is required by the old-set conservativeness gate. The pass can never delete a
raster-kept triangle, preserving both continuous coverage and old sample sets.
It does not run when all triangles are already retained. On meshes with many
certified hidden candidates it can still approach quadratic work; this bounded
campaign does not establish a global speed guarantee for that case. Finite-camera
worst-case BVH work is unchanged.

## Receipts

All original F8 behaviors pass, plus new pixel-option/atomicity, subpixel
aperture, legacy shared-edge rounding, hidden/finite-view limit recovery, and
1/2/4-thread pool determinism tests. The final release suite has 20 tests with
all features and 19 without default features. All-feature and no-default
unit/integration tests and doctests, Clippy with warnings denied, both WASM
builds, unsafe/package boundary, exact baseline comparisons and public-text
checks pass. WASM was built, not executed. No new unsafe code.

The exact drivers retain the original 87,272 observations per feature mode,
174,544 total, checking old APIs' bytes, errors, output tails and Usage across
all work limits and byte boundaries. Other algorithm sources remain byte-identical
to v0.2.16; their retained C++ cache qualification is unaffected by this extension.
The independent C++ HSR oracle is unchanged. Eighty scenes x three densities
compare the saved ray implementation exactly with C++; the three candidate
feature modes must keep a superset and agree exactly with each other: 960 total
comparisons. Full real/corpus outputs also agree in all three candidate modes,
and all old-kept real triangles remain in the candidate output.

Three repetitions use one pinned CPU, acknowledged perf controls and
instructions:u around warm resident calls. Arms are interleaved and reversed
in repetition two. Parsing, serialization and warmup are excluded; allocations,
classification and destruction remain counted. Simplify targets half the input
index count, relative error 0.5, no flags, on the identical complete input.
The speed gate uses wall time; counters are retained alongside it. All completed
observations and initial failed fixture attempts are preserved.

| Input | Before s | After s | Simplify s | HSR/simplify | Before removed | After removed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| real-214e | 75.136950 | 0.624958 | 1.225918 | 0.509788 | 70275 (2.464924%) | 0 (0.000000%) |
| real-a082 | 82.167631 | 0.788949 | 2.254854 | 0.349889 | 87292 (2.653474%) | 0 (0.000000%) |
| real-b6fd | 16.995897 | 0.154990 | 0.245984 | 0.630081 | 18643 (2.266805%) | 0 (0.000000%) |

The release N=128 versus 8N=1024 fixture charges 2,816 versus 25,600 work
units: **9.090909x** (assertion <=10x). The larger N=32768 versus 8N=262144
fixture charges 983040 versus 8650752:
**8.800000x** work and **8.885201x** median
wall time (0.002824036 versus 0.025092123 s).
The larger final scaling samples use 256-call resident batches on the same
pinned CPU as the real-mesh timings; all three repetitions are retained.
An initial pinned single-call measurement (about 2 ms at N) reported 15x wall
scaling; it is retained in `scaling-pinned-single.json`, not silently discarded.
The longer batch protocol amortizes the short-call clock/allocator behavior
without selecting favorable pairs. Earlier unpinned observations remain in
`scaling-unpinned.json`; all completed samples are retained.
These separated tiny triangles cannot certify whole-pixel occlusion, so the
area filter skips raster allocation. This bounded result is not a worst-case
complexity claim. The raster deliberately trades removal aggressiveness for coverage
certainty: meshes dominated by subpixel triangles may retain most or all faces.
No promise of a minimum removal share is made.

`exits.tsv` and `checks/exits.tsv` retain gate exit codes. The initial
fixture-failure runs and first C++ differential failure are retained rather
than counted as passes. No queued job
was canceled. No other model, AI CLI, dependency change, Git integration or
publication was used. The dedicated fresh target is deleted only after all
admitted jobs have completed, with a separate cleanup receipt.
