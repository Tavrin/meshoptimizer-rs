# Fan splitting and crack stitching

These are opt-in extensions, not in C++ meshoptimizer. Existing algorithms and
pipeline defaults retain their behavior; all new options default off and all new
structs are non-exhaustive.

## Design

`split_non_manifold_vertices` joins the corners around a vertex across every
shared edge, which is the fan definition `analyze_mesh` uses, and gives each fan
its own record. The lowest input corner keeps the original vertex, other fans
get appended copies of the complete supplied record in original corner order.
Edges shared by more than two faces stay shared (that is a different defect), so
`non_manifold_vertices` is zero afterwards and the boundary does not grow.

`stitch_cracks` uses a sorted sparse spatial grid (the weld grid's coordinate
widening and neighbour order; no randomized hash, no dependency). Candidate
visits, scans, union-find steps, sorting and validation are charged to the
workspace work limit. A pair of matching boundary edges is joined atomically at
both endpoints; accepting one endpoint would create a disconnected fan. A
T-junction reuses an existing vertex and adds one triangle; every original
boundary vertex subdivides at most one edge per call. A candidate is accepted
when every changed face keeps a positive normal dot product with its
predecessor and the whole-mesh counts of (a) edges with more than two faces, (b)
equally directed two-face edges and (c) surplus vertex fans do not increase.
Refusals are counted. Defects already in the input do not block other stitches.
This is not a self-intersection test. Whole-mesh validation per candidate makes
dense cases worst-case quadratic (see the 68k and 99k triangle timings below).

The pipeline runs orientation, optional stitching, optional fan splitting, then
hole filling; welding and sanitation run earlier. Stitching first lets a
T-junction connect its endpoint fans before leftover disconnected fans are
split. Narrowing of appended u16 indices is checked. Position-only merging keeps
the representative's attributes and leaves unused records for later compaction.

## Changes made while finishing (2026-11-02)

The first implementation isolated every corner at the endpoints of an edge with
more than two faces. The short fuzz run found it left non-manifold vertices
(a shared two-face edge whose other endpoint was isolated stopped being shared
but kept its merged corners at the first endpoint), and on public meshes it
raised boundary edges 306 to 8203 on `thingi170991` and the triangles reached at 10% from 1653 to 4351.
It now uses the `analyze_mesh` fan definition above. Stitch validation used to
reject any candidate while the whole mesh held any defect or degenerate face
(all candidates were refused on every public mesh with a non-manifold vertex);
it now compares defect counts and checks only changed faces. Reversal cost:
cheap while unmerged.

## Corpus and measurements

`corpus.py` downloads four small MIT topology meshes from a pinned
geometry-central revision; `thingi.py` downloads five Thingi10K meshes (CC BY,
Hugging Face revision `2d5d3b2f3cd3711028ad75b12788c13b25559ec6`, URL, author,
sha256 in `receipts/thingi.json`) chosen from `metadata/geometry_data.csv` for
open boundaries plus non-manifold vertices or edges; STL triangle soup is
indexed by exact float32 position equality only. Both write the same little-endian
`u32 vertices, u32 indices, f32 xyz, u32 triangles` format into caller scratch.
Corpus manifests: `receipts/corpus.json`, `receipts/thingi.json`.

`repair4_measure` runs stitching then splitting. It first drops repeated-index
triangles (sanitation does this in the pipeline; `thingi78660` has 260), then
reports `analyze_mesh` counts, wall time of both repair calls with owned output
(one-shot, release build, shared CPU; excludes analysis and simplification), and
the triangles reached by `simplify` at 10% of the index count with
`target_error` 0.01. Tolerance `default` is the library default (1e-6 times the
largest bounds extent); `1e-4` sets `REPAIR4_TOLERANCE_REL=1e-4`.
Raw rows: `receipts/measure.log`, `receipts/measure-tol1e-4.log`.

| mesh (triangles) | boundary edges before > after (default / 1e-4) | non-manifold vertices before > after | non-manifold edges before > after | stitched pairs, T splits, refused (default / 1e-4) | repair ms (default / 1e-4) | triangles at 10% before > after (default / 1e-4) |
|---|---|---|---|---|---|---|
| thingi170991 Galileo spacecraft (16681) | 306 > 306 / 286 | 70 > 0 | 701 > 701 | 0,0,0 / 0,8,19 | 1008 / 981 | 1653 > 1661 / 1669 |
| thingi90248 rooster (32642) | 286 > 286 / 284 | 144 > 0 | 0 > 0 | 0,0,0 / 0,2,46 | 1535 / 1551 | 3247 > 3264 / 3264 |
| thingi83642 brick 2x3 (21123) | 352 > 352 / 352 | 239 > 0 | 160 > 160 | 0,0,2 / 0,0,4 | 1238 / 1242 | 2103 > 2111 / 2111 |
| thingi66345 glasses (68841) | 563 > 545 / 497 | 7 > 0 | 0 > 0 | 4,2,2 / 12,34,382 | 7487 / 13577 | 6884 > 6883 / 6887 |
| thingi78660 Mars globes (99262 after dropping 260) | 244 > 210 / 175 | 14 > 0 | 3 > 3 | 3,31,250 / 33,13,131 | 9043 / 6439 | 9911 > 9914 / 9911 |
| triple_vierbein (MIT, 108) | 60 > 60 | 0 > 0 | 20 > 20 | 0,0,0 | 0.6 | 39 > 39 |
| fan3 (3) | 6 > 6 | 0 > 0 | 1 > 1 | 0,0,0 | 0.01 | 3 > 3 |
| hourglass_ico (40) | 0 > 0 | 1 > 0 | 0 > 0 | 0,0,0 | 0.02 | 40 > 40 |
| synthetic bow tie | 6 > 6 | 1 > 0 | 0 > 0 | 0,0,0 | 0.005 | 2 > 2 |
| synthetic gap | 6 > 4 | 0 > 0 | 0 > 0 | 2,0,0 | 0.005 | 2 > 2 |
| synthetic T-junction | 7 > 4 | 2 > 0 | 0 > 0 | 0,1,0 | 0.005 | 2 > 2 |

Reading the table honestly: fan splitting removes every non-manifold vertex on
every mesh. Stitching rarely finds anything at the default tolerance because the
cracks in these files are genuine gaps wider than 1e-6 of the extent; at 1e-4 it
closes a minority of boundary edges (up to 28% on `thingi78660`) and refuses
many candidates (up to 382 on `thingi66345`). None
of the five meshes' simplification at 10% changes by more than 1%: the target is
reached before the boundary or non-manifold vertices matter, and appended
T-junction triangles add a few. A tighter target or border-locking settings were
not measured. The timings show the validation cost: 7 to 14 seconds for the
69k-triangle mesh at 1e-4. The synthetic `gap`, `t_junction` and `bow tie` rows show the intended behavior exactly.

## Gate script

`check.py` runs, under CPU admission with a fresh external `CARGO_TARGET_DIR`,
`REPAIR4_INPUTS` (the downloaded corpus, `thingi/` below it) and
`MESHOPT_REFERENCE` (pinned C++ reference): fmt, tests (all features and none),
clippy `-D warnings`, doctests, docs `-D warnings`, wasm32 builds (no_std and
default), `cargo semver-checks` against 0.2.16, the unsafe-boundary script, both
measurements, exact comparison of the original repair and LOD outputs against
archived 0.2.16 sources, the C++ reference protocol checks, a cargo-fuzz build
and a 320 second single-process fuzz of the new `repair4` target (CPU seconds
recorded), and the public-text guard. Logs and measured exit codes are in
`receipts/`.
