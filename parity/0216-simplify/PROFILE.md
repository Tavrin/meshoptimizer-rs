# Simplifier instruction attribution (before experiments)

All four functions, both scalar arms, all three real meshes. Two warmed calls per profile,
`instructions:u`, period 200003. Source and binary bindings are retained in scratch.
`profile.json` records successful executions. Full reports, source-line reports and
`perf annotate -l` are retained with the raw recordings. Zero lost samples in every report.

The table pools the three meshes and divides sampled periods by two, reporting estimated
millions of user instructions per call summed over the three meshes. These are sampling
attributions, affected by skid and inline debug locations, **not** exact phase counters.
Rows are the five largest resolved Rust loop groups; the corresponding C++ group is shown
even when it is faster or larger. Unresolved locations are not silently allocated to a cause.
Positional and attribute arithmetic are grouped because inline shared Q/G helpers
cannot always be separated by their leaf source line. Deltas are diagnostic, not
isolated savings forecasts; the exact whole-call counts make the shipping decision.
`profile-lines.json` preserves each reported source-line attribution for inspection.

| Function | Cost | Rust est. M | C++ est. M | Rust - C++ est. M |
|---|---|---:|---:|---:|
| simplify | quadric construction/evaluation | 2117.2 | 2157.1 | -39.9 |
| simplify | position welding | 1507.9 | 1221.9 | +286.0 |
| simplify | adjacency | 1154.3 | 1082.7 | +71.6 |
| simplify | collapse picking | 1004.8 | 1022.9 | -18.1 |
| simplify | classification | 408.0 | 562.8 | -154.8 |
| simplify | other/unresolved (excluded from ranking) | 753.4 | 336.0 | — |
| simplify-attributes | quadric construction/evaluation | 4995.0 | 5069.3 | -74.3 |
| simplify-attributes | position welding | 1487.7 | 1163.3 | +324.4 |
| simplify-attributes | adjacency | 1135.7 | 1084.4 | +51.3 |
| simplify-attributes | collapse picking | 993.2 | 1011.6 | -18.4 |
| simplify-attributes | classification | 393.4 | 454.9 | -61.5 |
| simplify-attributes | other/unresolved (excluded from ranking) | 826.3 | 354.0 | — |
| simplify-update | quadric construction/evaluation | 2119.1 | 2128.7 | -9.6 |
| simplify-update | adjacency | 1612.7 | 1445.6 | +167.1 |
| simplify-update | position welding | 1487.2 | 1223.8 | +263.4 |
| simplify-update | collapse picking | 1000.1 | 1007.0 | -6.9 |
| simplify-update | solve/finalize | 531.1 | 199.3 | +331.8 |
| simplify-update | other/unresolved (excluded from ranking) | 1041.6 | 473.9 | — |
| simplify-sloppy | grid conversion | 786.7 | 670.0 | +116.7 |
| simplify-sloppy | quadric construction/evaluation | 764.8 | 1366.4 | -601.6 |
| simplify-sloppy | grid triangle counts | 719.0 | 476.6 | +242.4 |
| simplify-sloppy | cell hashing | 627.2 | 352.6 | +274.6 |
| simplify-sloppy | triangle deduplication | 388.7 | 78.7 | +310.0 |
| simplify-sloppy | other/unresolved (excluded from ranking) | 1266.4 | 166.7 | — |

Per-mesh leading source locations (five per arm, estimated M instructions/call):

| Function / mesh | Rust source locations | C++ source locations |
|---|---|---|
| simplify / real-214e | simplify.rs:724 356.5; simplifier.cpp:706 319.3; cmp.rs:1880 278.1; simplifier.cpp:146 256.2; simplify.rs:1527 254.7 | simplify.rs:724 356.5; simplifier.cpp:706 319.3; cmp.rs:1880 278.1; simplifier.cpp:146 256.2; simplify.rs:1527 254.7 |
| simplify / real-a082 | simplify.rs:1995 556.5; simplify.rs:1527 555.9; simplifier.cpp:1373 541.4; simplifier.cpp:706 499.4; simplifier.cpp:187 416.6 | simplify.rs:1995 556.5; simplify.rs:1527 555.9; simplifier.cpp:1373 541.4; simplifier.cpp:706 499.4; simplifier.cpp:187 416.6 |
| simplify / real-b6fd | simplify.rs:724 95.6; simplifier.cpp:706 88.7; cmp.rs:1880 56.7; simplify.rs:1403 56.0; non_null.rs:1650 54.0 | simplify.rs:724 95.6; simplifier.cpp:706 88.7; cmp.rs:1880 56.7; simplify.rs:1403 56.0; non_null.rs:1650 54.0 |
| simplify-attributes / real-214e | simplifier.cpp:731 472.2; simplify.rs:724 394.0; simplifier.cpp:706 312.7; cmp.rs:1880 292.4; simplifier.cpp:146 252.1 | simplifier.cpp:731 472.2; simplify.rs:724 394.0; simplifier.cpp:706 312.7; cmp.rs:1880 292.4; simplifier.cpp:146 252.1 |
| simplify-attributes / real-a082 | simplifier.cpp:706 893.2; simplify.rs:724 878.9; simplify.rs:834 743.0; simplifier.cpp:899 599.3; simplify.rs:1527 556.9 | simplifier.cpp:706 893.2; simplify.rs:724 878.9; simplify.rs:834 743.0; simplifier.cpp:899 599.3; simplify.rs:1527 556.9 |
| simplify-attributes / real-b6fd | simplifier.cpp:731 127.5; simplify.rs:724 101.1; simplifier.cpp:706 98.7; simplify.rs:1403 57.4; simplify.rs:1527 56.6 | simplifier.cpp:731 127.5; simplify.rs:724 101.1; simplifier.cpp:706 98.7; simplify.rs:1403 57.4; simplify.rs:1527 56.6 |
| simplify-update / real-214e | simplify.rs:724 354.4; simplifier.cpp:706 297.8; cmp.rs:1880 286.5; simplifier.cpp:146 255.2; simplify.rs:1527 253.6 | simplify.rs:724 354.4; simplifier.cpp:706 297.8; cmp.rs:1880 286.5; simplifier.cpp:146 255.2; simplify.rs:1527 253.6 |
| simplify-update / real-a082 | simplify.rs:1527 551.5; simplify.rs:1995 527.5; simplifier.cpp:1373 519.7; simplifier.cpp:706 507.1; simplify.rs:724 419.2 | simplify.rs:1527 551.5; simplify.rs:1995 527.5; simplifier.cpp:1373 519.7; simplifier.cpp:706 507.1; simplify.rs:724 419.2 |
| simplify-update / real-b6fd | simplify.rs:724 101.1; simplifier.cpp:706 84.0; simplifier.cpp:1036 60.5; simplify.rs:1403 57.7; cmp.rs:1880 53.6 | simplify.rs:724 101.1; simplifier.cpp:706 84.0; simplifier.cpp:1036 60.5; simplify.rs:1403 57.7; cmp.rs:1880 53.6 |
| simplify-sloppy / real-214e | simplify_extra.rs:1466 293.3; simplifier.cpp:2184 234.0; simplifier.cpp:744 223.8; simplify_extra.rs:1183 121.5; simplifier.cpp:706 119.2 | simplify_extra.rs:1466 293.3; simplifier.cpp:2184 234.0; simplifier.cpp:744 223.8; simplify_extra.rs:1183 121.5; simplifier.cpp:706 119.2 |
| simplify-sloppy / real-a082 | simplify_extra.rs:1277 308.0; simplify_extra.rs:1466 268.4; simplifier.cpp:2184 252.4; simplify_extra.rs:1182 211.8; simplifier.cpp:2109 151.3 | simplify_extra.rs:1277 308.0; simplify_extra.rs:1466 268.4; simplifier.cpp:2184 252.4; simplify_extra.rs:1182 211.8; simplifier.cpp:2109 151.3 |
| simplify-sloppy / real-b6fd | simplifier.cpp:2184 96.2; simplify_extra.rs:1466 41.4; simplify_extra.rs:1279 29.3; simplifier.cpp:744 25.7; simplifier.cpp:2092 25.3 | simplifier.cpp:2184 96.2; simplify_extra.rs:1466 41.4; simplify_extra.rs:1279 29.3; simplifier.cpp:744 25.7; simplifier.cpp:2092 25.3 |

Source/disassembly interpretation and first experiments:

- Welding: `simplify.rs:1527–1555`, C++ `simplifier.cpp:131–241`. Rust array equality
  at the candidate comparison emits three short-circuit `ucomiss` chains (the first
  comparison maps to `cmp.rs:1880`). Try eager independent XYZ comparisons; all reads
  have already completed, so error ordering and probe order need not change.
- Positional quadrics: `simplify.rs:719–832,1801–1977`, C++ `simplifier.cpp:704–718,1034–1145`.
  Rust accumulation already uses packed `movups/addps` for the first eight fields.
  The largest sampled locations are loads into dependent scatter destinations, not
  evidence of a missing four-field arithmetic backend. Keep sequential contributions.
- Attribute quadrics: `simplify.rs:1710–1800`, C++ `simplifier.cpp:847–928,1208–1230`.
  Gradient fields already receive packed additions. Checked disjoint records and
  indexed gradient writes remain costs; finite guards remain necessary for attributes.
- Adjacency/picking/classification: `simplify.rs:1360–1700,1979–2042`, C++
  `simplifier.cpp:52–130,337–536,1353–1450`. Bounds checks, remapped index gathers
  and ordered candidate predicates dominate these traversal loops. SIMD arithmetic
  does not remove their dependent accesses; unchecked indexing is not an experiment.
- Sloppy: `simplify_extra.rs:1238–1266`, C++ `simplifier.cpp:2070–2105`. The grid
  conversion expands each coordinate into add/mask/compare/correct operations;
  try the defined signed truncating cast on the existing positive bounded domain.
  Cell hashing (`:1178–1357`) and triangle counting (`:1268–1283`) retain all visits.

No explicit SIMD extension is justified by these profiles: the quadric field adds
already pack, while traversal, comparisons and conversion offer scalar opportunities.
This decision makes no claim about other compilers or architectures.

Update-specific scalar experiment: the original `simplify_extra.rs:1098–1121`
post-solve bounds pass repeatedly calls the mutable position view's checked reader
and work counter. It is included in solve/finalize above. For dense packed views,
try a direct ordered slice scan with the existing Work::scan fallback semantics;
sparse and strided views retain the original loop. No bounds reduction reorder.

Additional disassembly resolution: in real-214e sloppy, the large leaf location
`simplify_extra.rs:0` at baseline instruction `0x7a4f1` is the first multiply in
the scalar cell-quadric error evaluation, immediately after loading a coefficient.
The surrounding annotated block corresponds to original `:1768–1774`, not an
unknown algorithm phase. Other line-zero samples remain unresolved in the table;
no periods have been reassigned from a single-address observation. Eval uses
ordered scalar products and adds; accumulation's existing packed adds do not
imply a packed eval. A packed eval would require gathering/permuting three
coefficient/position lanes and extracting terms for the same ordered sum.
These instruction samples alone do not establish that arithmetic rather than
coefficient access is the limiting cost. No explicit eval backend is claimed
or shipped on that evidence.

For update, C++ `finalizeVertices` at `simplifier.cpp:611` receives the offset
and scale already produced by rescalePositions. Rust instead recomputes bounds
at original `simplify_extra.rs:1098–1121`. Reusing the already ordered bounds
is the second scalar experiment targeting this attributable extra pass.
