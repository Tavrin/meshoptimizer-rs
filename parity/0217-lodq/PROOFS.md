# LOD quality: design and proof map

Base: v0.2.16, `09da597f438471d0ad9c16e460f3598e9356e9ea`.
The supplied scout contains no F9/quality section; the feature specification
and existing `Budget`, `Work` and `Workspace` contracts determine this design.

## API and accuracy boundary

Feature `lod-quality` is disabled by default, requires neither `std` nor SIMD,
and adds `measure_lod_quality`, `measure_lod_chain_quality` and
`LodCamera::switch_distance`. All new public structures are non-exhaustive.
This is an extension **not in C++ meshoptimizer**. Existing simplification and
chain construction do not invoke it. Both measurement directions are always
reported. Symmetric max is the maximum of the two maxima; symmetric RMS pools
squared distances by sample count. Normal max/mean pool both directions.

Every triangle contributes three corner samples (including repeated shared
vertices) and `ceil(area * samples_per_unit_area)` barycentric samples. For
ordinal j of n: r = sqrt((j+0.5)/n), t = (reverse_bits(j)+0.5)/2^32, and weights
are (1-r, r*(1-t), r*t). Zero density means corners only. Degenerates have no
interior samples. Sample arrays are never allocated. Float calculations use
f64 after exact conversion from f32. A per-triangle count above u32::MAX returns
SizeOverflow, never a wrapping integer conversion.

These are sampled Hausdorff-style lower estimates, not certified continuous
Hausdorff bounds. RMS and mean are equal-sample statistics, not surface area
integrals. Only referenced vertices are included. Identical inputs on the same
build/target give identical results, BVH construction, ties, and work charges.
No cross-platform bit identity is claimed for libm transcendental functions.

Normals are normalized per vertex, then barycentrically interpolated at each
sample and its closest target point, then normalized again. Angle is
atan2(length(cross), dot), in radians; opposite normals give pi. Both surfaces
must supply normals or both omit them. Nonfinite/zero input normals are rejected;
cancelling interpolated normals produce NumericalFailure. Distance ties choose
the lowest original triangle ordinal, independent of BVH layout.

The camera uses pinhole transverse-error projection and reports minimum
camera-space **depth**, not camera-to-center distance. A bounding radius can
convert that to a conservative center-distance heuristic. Log-space evaluation
avoids intermediate product overflow. Sampling, view direction, visibility,
normal error, textures and rasterization prevent treating the threshold as a
certified screen-space bound. Chain levels are each compared directly against
the source, share its vertex buffers, and may have nonmonotone errors. Empty
surfaces are rejected; empty chains return no reports.

## Spatial search and bounded resources

The BVH recursively divides triangle counts at the median of the longest box
axis. Partition ordering includes the original ordinal as a tie breaker.
Quickselect has a bounded pass count and a metered heapsort fallback; no library
sort or hidden unmetered comparator scan is used. Tree height is logarithmic in
triangle count even with coincident centroids. Each leaf owns one triangle.
Closest points test the three bounded segments and the orthogonal face
projection; zero-area triangles use segment/point fallback. Box lower bounds
prune queries, nearer child first, while equal bounds remain searchable for ties.

Each validation corner, tree initialization record, node initialization,
bounding/partition record, fallback heap step, sampled point and visited query
node is charged before its work. Fixed three-component/three-edge arithmetic is
part of a visit. `Work` enforces the full-call budget on every path. Tree vectors
and chain report vectors use fallible `Budget` reservations, charged before
allocation and checked against actual capacity. Local scratch is dropped on
all return paths; workspace-owned prior scratch is counted and can be evicted
under byte pressure. Chain limits cover all levels and the retained reports.
No per-sample linear scan of target triangles is used. Worst-case overlapping
geometry can visit all leaves; Limits bound that adversarial case. Typical
search scaling is measured, not asserted as a worst-case complexity guarantee.

## Tests and independent oracle

`tests/lod_quality.rs` has focused analytic, sampling, normals, tie, invalid-input,
camera, chain, determinism, byte/work-limit and scaling cases. The exact analytic
maxima are plane separation 2, bump height 0.25, unit sphere-octahedron face
sagitta 1-1/sqrt(3), concentric octahedra separation 0.5, and an open decimated
cube's removed-face distance 1. Tolerance is 2e-7 world units. Parallel planes
also have exact RMS 2. Sample maxima on other fixtures are not claimed to prove
continuous extrema beyond the fixture's analytic upper/lower argument.

The unit sphere fixture has axis vertices and normalized spherical face centers;
all eight coarse faces are equally distant planes, so distance at each center
is 1-1/sqrt(3), and convexity bounds the remaining fine triangles by that value.
The cube has a retained center vertex on the removed face, giving distance 1 to
the remaining side walls; its other removed-face points are at most that far.

`oracle.cpp` is an independently compiled exhaustive C++ oracle using Voronoi
region nearest-point tests, not a copy of the BVH query. `differential.rs` checks
256 deterministic random/planar/collinear/point surface pairs, density zero and
two, both directions, maximum/RMS/sample count, at 1e-7 tolerance. This oracle is
**not an upstream API** and is not an upstream quality-parity claim.

`exact.rs` compares the existing simplify, simplify_into (including caller tails),
and progressive/independent chain APIs to a separately named v0.2.16 crate.
Every work limit through success+1, byte thresholds, repeated cold/warm calls,
retained-cache priming, and calls after invalid-index failure are compared for
Result, output and Usage. Valid/invalid topology, bad indices and empty input
are included in both feature modes. `proof.py` also verifies that every shared
implementation source equals the base; only lib.rs feature wiring differs.

The libFuzzer target is source only in `fuzz/`; it was not run. All builds and
measurements use the specified admitted CPU runner and dedicated disposable
target directory. See receipts/exits.tsv for every executed check, including
any initial failure and its final rerun. Full tests include guide doctests;
wasm builds cover scalar/no-std and default feature modes.

## Performance scope

`perf.py` uses identical existing peer-consumer source across four arms:
v0.2.16, opt-in candidate, scalar C++ 1.3, and default C++ 1.3 (SIMD enabled).
The unchanged simplify operation guards the pipeline to which quality is added.
No base/upstream quality API exists, so its execution cost cannot be assigned
a base/C++ parity ratio. The feature's own N-versus-8N work scaling is tested
separately. Five corpus inputs and three real inputs are SHA-256 bound to the
existing peer corpus. Inputs are packed XYZ/index projections; no authored
normal or UV quality claim follows.

Instructions:u is collected in a resident warm loop with perf control FD,
100% event coverage, CPU affinity, three repetitions, and reversed second-arm
order. Fixture loading, warmup and serialization are outside the count. Every
arm's output must be byte-identical. Wall seconds from the same loop are
reported only for disclosure. Source and binary hashes, raw perf samples,
medians and output hashes are retained; transient trees/binaries/inputs reside
in the disposable target directory. No GPU or end-to-end rendering claim.

## Execution results

Final gates PASS (exit 0); 16 focused tests in each mode, full all-features
and no-default suites/doctests, Clippy, fmt, unsafe-boundary and both wasm builds.
Initial Clippy failures 101 are retained; both were corrected and the final
Clippy run exited 0. Final all-features tests also ran after the production fix.

- default: exact base/candidate observations=91290; every work limit, byte boundaries, cold/warm/after-failure; differences=0
- scalar: exact base/candidate observations=91290; every work limit, byte boundaries, cold/warm/after-failure; differences=0

Independent C++ oracle: 256 surface pairs in each mode, 1e-7 tolerance, zero differences.
Exact/oracle executable checks consumed 1.53 CPU seconds, below 2 CPU-hours.
Analytic max/RMS cases passed at 2e-7 tolerance. Scaling: 75,114 versus
773,568 work units for 256 versus 2,048 triangles (10.29859x).

Final performance: 96 samples; worst candidate/base cell 1.00000069x.
See TABLE.md for all instruction medians, C++ comparisons and wall-time disclosure.
The final candidate source hash was verified against the measured snapshot.
The first campaign was retained and repeated after a final-source binary hash change.
Transient target storage was deleted after verification. The fuzz target was not run.
