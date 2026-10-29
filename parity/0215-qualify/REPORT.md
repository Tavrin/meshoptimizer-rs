# 0.2.15 qualification

Candidate: `train/0.2.15` at 357b11f (twelve merged changes). Base: v0.2.14. Source was frozen during qualification; no change was reverted.

## Equivalence with 0.2.14

Every changed function was compared with v0.2.14 on the merged source, under all features and under no default features. Each comparison covers results, output bytes, unused output tails, partial writes and both `Usage` fields, over every work limit on the small meshes, cold and reused workspaces. Zero differences in every run:

| Driver | All features | No default features |
|---|---:|---:|
| simplify | 1.2 B | 274 M |
| simplify_with_attributes | 825 M (+117 M targeted) | 191 M |
| merged normalization paths (`merge-exact.rs`) | 263.4 M | 181.4 M |
| simplify_with_update | 44.5 M | 10.9 M |
| prune / points | 15.5 M | 15.5 M |
| overdraw | 20.6 M + 136 k | same |
| overdraw, very large finite coordinates (`overdraw-huge.rs`) | 2.07 M | 2.07 M |
| cache, vertex decode, meshlets | ≈2 M | ≈2 M |
| remap | 1.89 M | 1.89 M |
| spatial meshlets | 407 k | 407 k |

The merged normalization driver crosses every position layout, attribute widths 0–9 and 32, overflowing scales, and rejected values at the first, middle and last vertex. Corpus identity passes for the LOD chain builder on the three production meshes and for the default vertex-cache table. C++ 1.3 differentials: 1,246, 178, 1,932 and 1,280 cases, all equal.

In overdraw, a finite triangle with very large coordinates can make a cluster's squared norm infinite before its finite check, so the square root sees a non-finite operand. Infinite roots are identical in both backends and NaN results are rejected by the next check with the same error and work charge, so no observable output changes; `overdraw-huge.rs` exercises this case.

## Instruction counts

Scalar builds, `instructions:u`, median of three, over 1,800 samples. Geometric means over three production LOD-chain meshes; full table with corpus means in [TABLE.md](TABLE.md). Ratios below 1 mean fewer instructions.

| Function | vs 0.2.14 | vs optimesh 1.1.0 | vs scalar C++ 1.3 |
|---|---:|---:|---:|
| simplify | 0.919 | 0.898 | 1.137 |
| simplify_with_attributes | 0.787 | 0.926 | 1.095 |
| simplify_sloppy | 0.845 | 0.706 | 1.275 |
| simplify_with_update | 0.936 | 1.008 | 1.280 |
| simplify_prune | 0.620 | 0.763 | 0.995 |
| simplify_points | 0.601 | 0.632 | 1.014 |
| optimize_overdraw | 0.753 | 0.884 | 1.090 |
| build_meshlets / flex | 0.826 | 0.784 | 1.001 |
| build_meshlets_spatial | 0.876 | 0.835 | 0.965 |
| cluster LOD | 0.909 | — | 1.189 |
| optimize_vertex_cache | 0.995 | 0.957 | 1.306 |
| optimize_vertex_fetch | 1.000 | 0.716 | 0.952 |
| vertex decode | 1.000 | 0.544 | 1.061 |
| index decode | 1.000 | 0.907 | 0.994 |
| remap_vertex_buffer | 0.465 | 0.453 | 1.394 |
| remap_index_buffer | 0.772 | 0.975 | 1.625 |

No sample is above 1.01 against v0.2.14, and output equals v0.2.14 and C++ in every cell. The vertex-cache improvement measured on its own branch (0.942) is not present on the merged source (0.995 on the production meshes, 0.966 on the corpus) and is not claimed.

New functions, corpus geometric mean against `filter_index_buffer` ([campaign/sanitize-table.md](campaign/sanitize-table.md)): `sanitize` 2.33×, with position welding 9.03×, with all options 9.07×; `analyze_mesh` 14.8×, with welding 17.3×.

## Fuzzing

Ten targets ran until coverage stopped growing for ten minutes, seeded from the corpus, with generators that include coordinates near `f32::MAX`: zero crashes ([fuzz/README.md](fuzz/README.md)). CPU seconds: remap 630, overdraw 640, sanitize 770, vertex layouts 605, analyze 1,488, cache 2,014, sloppy/prune/points 2,560, simplify 4,442, cluster LOD 4,958, meshlets 6,335.

The LOD chain target found an integer overflow when converting a very large ratio target (`inf`, `1e30`) to an index count: a debug-build panic on invalid input. It is fixed separately with a regression test and its fuzz run is repeated before release.

## Not covered

Fuzzing without default features; MSRV, wasm and non-x86 targets in this run (covered by CI); an independent review of `merge-exact.rs` and `overdraw-huge.rs`.
