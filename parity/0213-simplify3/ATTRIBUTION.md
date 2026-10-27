# Residual instruction attribution

Attribution completed before library edits. Fresh v0.2.12 and scalar M7/C++ 1.3 binaries; all 18 frozen real-chain stages. perf 6.17 instructions:u stat plus record/annotate on CPU9, acknowledged resident-loop enable/disable. Source, binaries, raw profiles and complete DWARF inline stacks are retained in private scratch. No wall-performance claim.

The initial 100,000-instruction sampling period throttled despite zero lost samples. All 36 initial profiles are retained and rejected for function attribution. Replacement period: 1,000,000 instructions. All 36 replacement profiles have zero THROTTLE/LOST records, and recorded instruction periods cover each corresponding stat total within 2%. This is an instrumentation repair, not a gate campaign or a selected favorable sample.

Numbers below are sampled instruction estimates calibrated to stat totals, in billions, summed across the six stages of each chain. They are diagnostic estimates, not isolated ablation effects. Inline stacks resolve helpers inside run_state and meshopt_simplifyEdge; unresolved/unannotatable instructions remain in other. Sampling skid and finite sample counts limit fine attribution.

| Port function/phase | 214e Rust | 214e C++ | a082 Rust | a082 C++ | b6fd Rust | b6fd C++ |
|---|---:|---:|---:|---:|---:|---:|
| bounds/normalization | 4.326 | 1.425 | 4.605 | 1.521 | 1.152 | 0.382 |
| welding | 5.262 | 3.351 | 5.579 | 3.508 | 1.390 | 0.875 |
| quadrics/finite-inverse | 1.578 | 0.000 | 1.686 | 0.000 | 0.426 | 0.000 |
| validation | 1.476 | 0.000 | 1.583 | 0.000 | 0.396 | 0.000 |
| adjacency | 2.699 | 2.009 | 5.104 | 3.727 | 1.280 | 0.929 |
| classification | 3.815 | 2.999 | 4.319 | 3.353 | 1.080 | 0.840 |
| index-compaction | 0.000 | 0.000 | 1.450 | 0.720 | 0.366 | 0.180 |
| quadric-update | 0.000 | 0.000 | 0.957 | 0.444 | 0.230 | 0.112 |
| loop-remap | 0.000 | 0.000 | 1.363 | 1.164 | 0.350 | 0.290 |
| picker | 1.661 | 1.452 | 4.370 | 3.812 | 1.092 | 0.956 |
| ranking | 0.000 | 0.000 | 0.302 | 0.410 | 0.078 | 0.102 |
| sorting | 0.000 | 0.000 | 0.120 | 0.122 | 0.030 | 0.030 |
| collapse/update | 0.048 | 0.000 | 0.626 | 0.410 | 0.142 | 0.102 |
| initialization/other | 0.186 | 0.351 | 0.192 | 0.488 | 0.048 | 0.100 |
| other | 0.057 | 0.170 | 0.073 | 0.345 | 0.009 | 0.185 |
| quadrics/face | 3.392 | 2.770 | 4.173 | 3.253 | 0.954 | 0.878 |
| quadrics/regularize | 0.269 | 0.918 | 0.090 | 1.043 | 0.050 | 0.264 |
| quadrics/edge | 1.583 | 1.434 | 3.097 | 2.944 | 0.697 | 0.555 |
| quadrics/attributes | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| quadrics/other | 0.300 | 0.000 | 0.089 | 0.000 | 0.158 | 0.000 |

Function correspondence:

- bounds_impl/run_state normalization -> rescalePositions; position_remap_kernel -> buildPositionRemap/hashLookup2/PositionHasher.
- State::quadrics -> fillFaceQuadrics, fillVertexQuadrics, fillEdgeQuadrics and fillAttributeQuadrics. The finite/inverse phase has no direct C++ pass; its cached reciprocal partly saves work in ranking, so its entire count is not removable overhead.
- State::adjacency -> updateEdgeAdjacency; State::classify/has_edge -> classifyVertices/hasEdge.
- State::pick/rank/sort/perform/flips -> pickEdgeCollapses/rankEdgeCollapses/sortEdgeCollapses/performEdgeCollapses/hasTriangleFlips.
- State::update -> updateQuadrics and remapEdgeLoops; run_state output compaction -> remapIndexBuffer.

Largest residual causes and implementation order:

1. Extra traversal and generic reader/codegen in bounds/normalization: Rust/C++ 4.326/1.425, 4.605/1.521 and 1.152/0.382 billion instructions. Rust decodes/caches coordinates during welding, rereads the strided input for bounds, then traverses cached coordinates for normalization. C++ combines copying and bounds. Fuse the independent bounds reduction into welding, but retain its work charge and extent-overflow error at the original position after classification. Normalize through an initialized fixed-length destination rather than per-vertex Vec growth.
2. Welding table construction: excess 1.912/2.071/0.515 billion instructions. General indexed position reads and three reserved-vector push paths remain inside the loop. Dispatch full byte records once; initialize reserved prefixes and use disjoint fixed-length slices. Keep table size, hash/probe order, canonical representatives, wedge order and every counted probe unchanged.
3. Remaining mandatory checks: finite/inverse 1.578/1.686/0.426 billion and input validation 1.476/1.583/0.396 billion. Keep all guards/error positions. Express the quadric finite reduction as a maximum unsigned absolute bit pattern, and use the full-record iterator for counted validation. Reciprocal caching/overflow behavior stays unchanged.
4. Adjacency/classification have excess 0.690/1.377/0.351 and 0.816/0.966/0.240 billion instructions. Shared slice extents and direct edge-slice traversal can remove repeated state/table reloads and bounds checks without changing traversal order.
5. Real a082/b6fd index compaction has excess 0.730/0.186 billion; quadric update 0.513/0.118 billion. Use checked triangle records and a common vertex prefix. Loop remapping similarly retains exact in-place order and visits.

Layout and algorithms: both implementations use AoS positions (12 bytes), directed edges (8), collapses (12), u32 indices/order and the same power-of-two quadratic-probing position table; both use 2,560 sort buckets. Rust positional quadrics are 48 bytes versus C++ 44 due to the cached inverse weight. There is no HashMap-versus-flat-table or heap-queue discrepancy to repair. Changing allocated record sizes would change Usage.bytes, so the candidate retains storage sizes/capacities and fallible reservation order. No new sorting, allocation, unsafe, SIMD or threading is proposed.

Exit codes: both arm builds=0; 36 replacement stat/record/report/annotate/source-resolution operations=0; sampling-quality checks=0; attribution summarization=0. Initial profiling processes exited 0 but fail sampling quality and are not qualified attribution evidence.
