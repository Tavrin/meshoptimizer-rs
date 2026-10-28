# simplify_with_attributes: attribution at v0.2.14

All counts are `instructions:u` from the parity/peers consumer
(`simplify-attributes`: four synthetic floats per vertex, unit weights, half
the index count, error 0.5). No wall time.

## Starting point (one counted call, perf stat)

| Mesh | v0.2.14 | C++ 1.3 | optimesh 1.1.0 | base/optimesh | base/C++ |
|---|---:|---:|---:|---:|---:|
| real-214e (6.80 M vertices, 2.85 M triangles) | 7,120,666,450 | 4,926,656,821 | 5,657,336,724 | 1.259 | 1.445 |
| real-a082 (7.24 M, 3.29 M) | 10,366,856,462 | 7,595,292,148 | 9,113,906,873 | 1.137 | 1.365 |
| real-b6fd (1.81 M, 0.82 M) | 2,589,256,818 | 1,892,981,408 | 2,275,719,249 | 1.138 | 1.368 |

The real meshes carry more than two vertices per triangle (split wedges), so
per-vertex passes weigh as much as per-triangle passes.

## Where v0.2.14 spends instructions (real-b6fd)

Sampled `instructions:u` (period 10^6, warm-up plus one counted call, so two
simplifications), by symbol, in millions:

| Symbol | M | Content |
|---|---:|---|
| `State::quadrics` | 1075 | positional, edge and attribute accumulation, final attribute finite scan |
| `run_state` | 834 | position welding hash, classification (inlined) |
| `State::attribute_quadric` | 727 | per-triangle attribute gradients |
| `run_state` normalization closure | 684 | position rescale and attribute weighting, per vertex |
| `State::pick` | 455 | candidate edges per round |
| `State::adjacency` (welded) | 396 | per-round adjacency |
| `validate` | 382 | finite scan of positions and attributes, per-vertex work charges |
| `State::update` | 306 | quadric merges, loop remaps |
| `State::rank` | 112 | collapse costs |

C++ on the same mesh spends its samples in `meshopt_simplifyEdge` (inlined
loop phases, 44%), position hashing (13%), edge adjacency (12%),
`fillAttributeQuadrics` (11%), classification (5%) and ranking (5%). optimesh
spends them in `simplify_edge` (58%), position remapping (18% with its
hasher) and edge adjacency (13%). Neither validates inputs or checks quadrics
for finiteness, and both rescale attributes in a plain nested loop.

## Exact cost of the contract checks (scratch ablations, v0.2.14)

Each variant removed one check in a scratch copy and was counted with
`perf stat` over the whole process. Differences are halved to one call:

| Removed in scratch | real-b6fd / call | real-214e / call |
|---|---:|---:|
| attribute-path `validate` loop (finite tests and per-vertex charges) | 185.4 M | 693.4 M |
| normalization finite test on weighted attributes | 66.0 M | 244.7 M |
| final attribute quadric and gradient finite scan | 155.5 M | 580.4 M |
| finite tests after quadric merges in `update` | 11.2 M | 0.0 M |
| finite test in attribute ranking | 3.6 M | 0.0 M |
| edge-kind `contains` replaced by a bit mask | -0.8 M | 0.0 M |
| all of the above | 416.5 M | 1518.6 M |

These ablations are not admissible changes: attribute gradients divide by the
squared triangle area, so finite inputs can still produce non-finite
quadrics, and the attribute and weight tests are part of the contract. They
show where the cost sits. It is in the per-vertex loop structure around the
tests (closure dispatch, view lookups and per-element charges), not in the
comparisons themselves. The candidate keeps every test and replaces those
loops; see PROOFS.md.
