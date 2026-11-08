# Sampled one-pixel shading comparison

Continuation 4 source base `de69ee32fe00b315491b01dcfea86246e8e28e6f`; builder revision 8, vertex codec version 1 / level 2. Container and page layouts are unchanged. Legacy API and output are checked separately. Cook keys bind the new normal-error weight when normals and the measured policy are present.

The displacement proxy is `2 * weight * L * sin(theta / 2)`: the unit-normal chord times a local lever arm. `L` is the largest source/replacement triangle edge in each original-source region, and `theta` is the largest symmetric sampled interpolated-normal angle there. Taking separate regional maxima is conservative. This stays monotone through 180 degrees; it is a shading proxy rather than a BRDF or texture certificate. The result is maximized with every existing geometric floor and inherited child error.

The angular and geometric maxima share the existing region samples and exact nearest queries. The private path compares squared unit-normal chords and takes one final square root, avoiding per-sample trigonometry. Repeated corners are deduplicated. Constant-normal regions and regions already covered by the maximum possible chord retain the positional-only path. Weight zero, or absent normals, disables the new term.

Protocol: 16 Fibonacci directions, projected bounding-sphere diameters 64/128/256/512/1024/2048 px, 60-degree vertical FOV, 2048 px viewport, one-pixel threshold; no visibility culling. Decode actual MDAG 2 pages before selecting cuts. Every artifact passes monotonicity, a finest triangle-count check and the sequential cut oracle. Each unique cut uses 16,384 deterministic area-stratified probes in each surface direction, reusing source probes across all variants. Normal p95 pools probes with view multiplicity; maximum is the observed maximum. These are sampled comparisons, not continuous certificates. Geometric ratios divide each cut's symmetric sampled maximum by its largest selected replacement claim (leaf quantization displacement when finest), then report view median/p95/max. The new proxy intentionally makes geometric claims more conservative.

Build times below are median build + hierarchy + serialization milliseconds over five counterbalanced repetitions, pinned 1T/8T. Channel packing is outside the build clock. Full timings, observed p95/noise, workspace usage, commands, source/binary/input hashes and identity receipts are retained in [campaign data](campaign/protocol.json). Weight-zero files must match the previous retained files exactly; each weight must match across 1/8 workers.

| Mesh | Weight | Mean cut triangles | Normal p95 / max degrees | Geometric measured/claimed median / p95 / max | Build ms 1T / 8T |
| --- | ---: | ---: | ---: | ---: | ---: |
| bunny | 0 | 22,671.65 | 9.34 / 61.86 | 0.9085 / 0.9758 / 1.0531 | 1,157.62 / 389.69 |
| bunny | 0.25 | 32,683.74 | 4.47 / 33.57 | 0.4606 / 0.8869 / 0.8869 | 2,320.68 / 638.89 |
| bunny | 0.5 | 42,171.55 | 3.37 / 26.00 | 0.3807 / 0.8538 / 0.8538 | 2,388.53 / 691.16 |
| bunny | 1 | 51,164.85 | 2.28 / 11.87 | 0.1979 / 0.8538 / 0.8538 | 2,325.34 / 652.12 |
| dead_tree_trunk_02 | 0 | 35,979.00 | 29.68 / 175.01 | 0.8325 / 0.9322 / 1.0309 | 1,200.59 / 478.22 |
| dead_tree_trunk_02 | 0.25 | 67,593.67 | 7.48 / 165.53 | 0.2093 / 0.8495 / 0.8495 | 2,974.02 / 1,019.70 |
| dead_tree_trunk_02 | 0.5 | 76,634.33 | 3.22 / 42.49 | 0.8495 / 0.8495 / 0.8495 | 2,860.30 / 1,011.23 |
| dead_tree_trunk_02 | 1 | 82,288.33 | 0.00 / 29.63 | 0.8495 / 0.8495 / 0.8495 | 2,990.04 / 980.09 |
| namaqualand_boulder_02 | 0 | 49,338.89 | 19.52 / 170.30 | 0.7999 / 0.8616 / 0.8616 | 1,409.30 / 491.11 |
| namaqualand_boulder_02 | 0.25 | 77,562.67 | 7.14 / 72.22 | 0.3652 / 0.8616 / 0.8616 | 3,226.41 / 997.42 |
| namaqualand_boulder_02 | 0.5 | 88,493.67 | 3.67 / 49.14 | 0.8616 / 0.8616 / 0.8616 | 3,216.22 / 982.20 |
| namaqualand_boulder_02 | 1 | 95,926.33 | 0.00 / 29.29 | 0.8616 / 0.8616 / 0.8616 | 3,247.08 / 922.71 |
| coast_rocks_01 | 0 | 84,123.96 | 34.81 / 172.84 | 0.7361 / 0.8439 / 1.0166 | 10,302.47 / 3,687.55 |
| coast_rocks_01 | 0.25 | 221,583.88 | 20.26 / 140.65 | 0.3257 / 0.5283 / 0.5622 | 28,742.17 / 9,434.03 |
| coast_rocks_01 | 0.5 | 324,687.86 | 14.06 / 168.65 | 0.2029 / 0.2644 / 0.2864 | 30,162.50 / 9,573.97 |
| coast_rocks_01 | 1 | 423,064.48 | 9.37 / 168.65 | 0.1014 / 0.1265 / 0.1371 | 30,664.34 / 9,627.53 |
| dragon | 0 | 85,376.79 | 10.71 / 176.91 | 0.8066 / 0.9300 / 1.0894 | 13,798.23 / 4,752.91 |
| dragon | 0.25 | 316,645.10 | 5.29 / 165.10 | 0.3758 / 0.5107 / 0.5176 | 27,868.06 / 8,476.26 |
| dragon | 0.5 | 441,581.76 | 3.85 / 28.35 | 0.1962 / 0.2583 / 0.2647 | 41,635.92 / 12,996.73 |
| dragon | 1 | 563,483.61 | 2.73 / 179.87 | 0.1009 / 0.9060 / 0.9060 | 41,381.00 / 13,419.17 |

Default decision: No tested weight satisfies both requested conditions; disabled default retained.
Selected default: **0**. Dragon limits: p95 <= 3.008072 degrees and mean cut triangles <= 170,753.58 (twice weight zero). Eligible tested weights: `[]`.

## Existing attribute weights

The matrix uses opt-in normal-error weight 1 because no tested weight met the default-selection rule. The library default remains zero. The recommendation below is scoped to that opt-in mode.

Normals are supplied through `BuildMesh::normals` only, retaining its fixed simplifier weight 0.5; the new field controls group error rather than that simplifier weight. Correctly packed attributes contain UV2 + tangent4 only. The duplicated controls additionally pack normal3 at weight 1, matching the accidental consumer duplication. `consumer-current` has shading weight zero; `duplicate-control` uses the comparison shading weight. No UV/tangent API is added.

The three scan meshes have authored UVs but no authored tangents. Diagnostic tangents come from area-weighted UV gradients, Gram-Schmidt orthogonalization and handedness, with deterministic fallback at degenerate charts. Counts are recorded in raw rows. These results do not qualify authored or MikkTSpace tangents. The two Stanford meshes have derived UVs and are excluded from this matrix.

Each cell has one 8T build, with no performance claim from that single observation. The same cut/probe protocol is used. UV deviation is Euclidean distance in source UV units; tangent deviation compares orthogonalized tangent directions, with handedness mismatches reported separately in raw rows.

| Mesh | Input | UV / tangent weights | Mean triangles | Normal p95 / max degrees | UV p95 | Tangent p95 / max degrees | Geometric ratio p95 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| dead_tree_trunk_02 | consumer-current | 1 / 1 | 52,097.71 | 17.96 / 175.46 | 0.001002 | 14.66 / 179.36 | 0.8495 |
| dead_tree_trunk_02 | duplicate-control | 1 / 1 | 82,145.67 | 0.00 / 34.55 | 0.000003 | 0.00 / 173.72 | 0.8495 |
| dead_tree_trunk_02 | u0p25-t0p25 | 0.25 / 0.25 | 82,391.00 | 0.00 / 34.68 | 0.000002 | 0.00 / 158.45 | 0.8495 |
| dead_tree_trunk_02 | u0p25-t0p5 | 0.25 / 0.5 | 82,391.00 | 0.00 / 26.02 | 0.000002 | 0.00 / 172.31 | 0.8495 |
| dead_tree_trunk_02 | u0p25-t1p0 | 0.25 / 1 | 82,299.00 | 0.00 / 44.34 | 0.000002 | 0.00 / 165.18 | 0.8495 |
| dead_tree_trunk_02 | u0p5-t0p25 | 0.5 / 0.25 | 82,391.00 | 0.00 / 34.68 | 0.000002 | 0.00 / 158.45 | 0.8495 |
| dead_tree_trunk_02 | u0p5-t0p5 | 0.5 / 0.5 | 82,391.00 | 0.00 / 26.02 | 0.000002 | 0.00 / 172.31 | 0.8495 |
| dead_tree_trunk_02 | u0p5-t1p0 | 0.5 / 1 | 82,299.00 | 0.00 / 44.34 | 0.000002 | 0.00 / 165.18 | 0.8495 |
| dead_tree_trunk_02 | u1p0-t0p25 | 1 / 0.25 | 82,391.00 | 0.00 / 34.68 | 0.000002 | 0.00 / 158.45 | 0.8495 |
| dead_tree_trunk_02 | u1p0-t0p5 | 1 / 0.5 | 82,391.00 | 0.00 / 26.02 | 0.000002 | 0.00 / 172.31 | 0.8495 |
| dead_tree_trunk_02 | u1p0-t1p0 | 1 / 1 | 82,299.00 | 0.00 / 44.34 | 0.000002 | 0.00 / 158.45 | 0.8495 |
| namaqualand_boulder_02 | consumer-current | 1 / 1 | 65,031.64 | 11.50 / 133.03 | 0.000312 | 8.03 / 178.29 | 0.8616 |
| namaqualand_boulder_02 | duplicate-control | 1 / 1 | 96,029.33 | 0.00 / 27.55 | 0.000003 | 0.00 / 176.47 | 0.8616 |
| namaqualand_boulder_02 | u0p25-t0p25 | 0.25 / 0.25 | 95,969.00 | 0.00 / 26.09 | 0.000003 | 0.00 / 134.53 | 0.8616 |
| namaqualand_boulder_02 | u0p25-t0p5 | 0.25 / 0.5 | 95,673.33 | 0.00 / 29.96 | 0.000003 | 0.00 / 170.64 | 0.8616 |
| namaqualand_boulder_02 | u0p25-t1p0 | 0.25 / 1 | 95,823.67 | 0.00 / 25.89 | 0.000003 | 0.00 / 176.22 | 0.8616 |
| namaqualand_boulder_02 | u0p5-t0p25 | 0.5 / 0.25 | 95,969.00 | 0.00 / 26.09 | 0.000003 | 0.00 / 134.53 | 0.8616 |
| namaqualand_boulder_02 | u0p5-t0p5 | 0.5 / 0.5 | 95,673.33 | 0.00 / 34.76 | 0.000003 | 0.00 / 165.41 | 0.8616 |
| namaqualand_boulder_02 | u0p5-t1p0 | 0.5 / 1 | 95,601.00 | 0.00 / 25.89 | 0.000003 | 0.00 / 176.22 | 0.8616 |
| namaqualand_boulder_02 | u1p0-t0p25 | 1 / 0.25 | 95,969.00 | 0.00 / 26.09 | 0.000003 | 0.00 / 134.53 | 0.8616 |
| namaqualand_boulder_02 | u1p0-t0p5 | 1 / 0.5 | 95,673.33 | 0.00 / 29.88 | 0.000003 | 0.00 / 133.03 | 0.8616 |
| namaqualand_boulder_02 | u1p0-t1p0 | 1 / 1 | 95,823.67 | 0.00 / 25.89 | 0.000003 | 0.00 / 176.22 | 0.8616 |
| coast_rocks_01 | consumer-current | 1 / 1 | 160,671.24 | 29.98 / 167.08 | 0.000601 | 22.17 / 179.63 | 0.6072 |
| coast_rocks_01 | duplicate-control | 1 / 1 | 424,152.89 | 8.14 / 109.13 | 0.000089 | 5.60 / 179.74 | 0.3048 |
| coast_rocks_01 | u0p25-t0p25 | 0.25 / 0.25 | 423,071.77 | 9.34 / 85.18 | 0.000089 | 6.58 / 178.80 | 0.1325 |
| coast_rocks_01 | u0p25-t0p5 | 0.25 / 0.5 | 424,090.01 | 9.08 / 125.61 | 0.000088 | 6.06 / 178.40 | 0.1456 |
| coast_rocks_01 | u0p25-t1p0 | 0.25 / 1 | 432,675.68 | 8.98 / 80.65 | 0.000087 | 5.42 / 179.46 | 0.1550 |
| coast_rocks_01 | u0p5-t0p25 | 0.5 / 0.25 | 421,987.48 | 9.45 / 85.18 | 0.000091 | 6.71 / 179.09 | 0.1423 |
| coast_rocks_01 | u0p5-t0p5 | 0.5 / 0.5 | 422,595.39 | 9.14 / 118.09 | 0.000089 | 6.12 / 177.93 | 0.1432 |
| coast_rocks_01 | u0p5-t1p0 | 0.5 / 1 | 431,964.48 | 9.05 / 165.49 | 0.000087 | 5.47 / 179.30 | 0.1484 |
| coast_rocks_01 | u1p0-t0p25 | 1 / 0.25 | 423,487.33 | 9.29 / 87.79 | 0.000089 | 6.55 / 178.98 | 0.1356 |
| coast_rocks_01 | u1p0-t0p5 | 1 / 0.5 | 423,596.64 | 9.09 / 85.18 | 0.000088 | 6.11 / 179.73 | 0.1456 |
| coast_rocks_01 | u1p0-t1p0 | 1 / 1 | 431,834.66 | 9.08 / 106.49 | 0.000088 | 5.48 / 179.49 | 0.1463 |

Recommendation: **dedicated normals only; UV 1, tangent 1, normal-error weight 1**. All three meshes: normal, UV, tangent p95 and tangent handedness mismatch fraction <= 1.10 of correctly packed 1/1. Minimize geometric mean triangle ratio; ties prefer larger weights. Selected files also match across 1/8 workers. See [selection](campaign/attribute-selection.json).
