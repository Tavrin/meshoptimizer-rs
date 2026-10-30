> Historical F8 proof record. The current outside-view implementation and speed
> qualification are documented in [speed/PROOFS.md](speed/PROOFS.md). The original
> receipts below remain intact; their timings and sampled-ray design describe
> the saved implementation, not the new software item buffer.

# Hidden-surface removal proof record

Base: v0.2.16, `09da597f438471d0ad9c16e460f3598e9356e9ea`.
This is an always-available extension, not a port of an upstream C++ API.
No Cargo feature, package-version change, dependency change, Git state mutation,
or publication is part of this lane.

## Visibility contract and design

`remove_hidden_triangles(indices, positions, options, workspace)` returns a
`VisibleMesh`. The `_into` form takes destination storage first and returns a
`HiddenReport`. New public types are non-exhaustive. The report distinguishes
same-winding exact position duplicates, occluded faces with exact reverse-wound
counterparts, and other sampled-occluded triangles. Output is a stable subsequence
of the input, retaining original indices and winding.

The default tests seven samples per triangle: the denominator-two barycentric
lattice (vertices and edge midpoints) plus centroid. Each sample casts toward
26 normalized nonzero vectors in {-1,0,1} cubed, in lexicographic order. Options
also accept finite camera points or a box sampled at its eight corners and
center, and lattice denominators 1 through 64. Directions are orthographic for
outside views, perspective for finite viewpoints. There is no orientation cull.

A balanced median-split BVH uses a total `(centroid, original ordinal)` split
order, fixed leaf size four, and preorder escape indices for stackless queries.
A sorted exact-position/cyclic-winding pass identifies duplicate classes and
back-to-back representatives first. Signed zeros match. Later same-winding
copies are removed; opposite copies require actual visibility testing and are
never deleted solely because their normals oppose. No per-triangle global
linear occluder scan is used in the implementation. Worst-case query work can
still be quadratic with spatially overlapping surfaces; limits apply to it.

A triangle with any visible tested sample is retained. This is a sampling
guarantee, not a continuous-visibility proof: small openings, visible slivers
between samples, views between the 26 directions, textures, transparency and
motion are outside it. Triangle intersection uses f64 on finite f32 coordinates,
not adaptive exact predicates. The absolute world tolerance plus
64*f64::EPSILON*max(1, largest referenced coordinate magnitude) ignores near
contacts and viewpoint endpoints. Increasing tolerance retains more geometry.
Degenerate triangles are retained apart from exact duplicates.

Occluders are the complete original mesh. The resulting filtered mesh is not
certified to have identical continuous visibility, topology or watertightness.
Coplanar contacts do not mutually occlude. An opaque two-sided support plane is
visible from below with unrestricted outside views. The resting-box fixture
therefore explicitly requests accessible cameras above the support: the box
bottom and fully covered support triangles disappear; a partially covered
triangle with a visible sampled corner stays. Exposed back-to-back walls stay.
This view-domain distinction is part of the API contract, rather than a claim
that sampled geometry can establish physical inaccessibility of camera regions.

## Resource and error argument

All dynamic arrays use the existing fallible Budget, including triangle records,
keep/pair flags, duplicate-sort entries, BVH nodes and owned output. Counts and
byte arithmetic are checked. Work is charged before corresponding allocation,
sort/selection, position reads, sample/view evaluation, node visit, triangle
intersection and output emission. Balanced construction recursion uses only
logarithmic stack depth. Scratch is call-local; existing retained workspace
storage remains included in the byte budget and can be evicted by the existing
workspace machinery.

Caller-buffer writes follow complete classification, capacity validation and
reservation of all emission work. No fallible operation follows the first write.
Errors therefore preserve the complete destination, and successes preserve its
tail. Workspace usage is finalized on success and failure. All pre-existing
algorithm implementations remain byte-identical to v0.2.16; the library root
only adds a private module and public exports.

## Evidence

`receipts/exits.tsv` retains final passing gates and the two pre-measurement
baseline-layout failures (exit 101). Earlier lint failures and their corrected
full reruns are retained in `initial/`, `second/`, and `third/`; bootstrap test
constructor corrections are described in `bootstrap.json`. No failed attempt is
presented as a passing gate. The definitive full run passes all-feature and
no-default-feature tests, Clippy with warnings denied, package/unsafe boundary,
WASM builds, exact comparisons, release feature tests and public-text checks.
Doctests pass 40 cases with all features and 37 without default features; five
pre-existing ignored examples remain in each mode.
WASM is build evidence only; no execution on WASM is claimed. The default-SIMD
WASM build retains five existing unused-dispatch warnings. No new unsafe code.

The 15 focused tests cover nested cubes, an independent interior surface hidden
by a closed lid and visible through its opening, a resting box with fully and
partially covered support triangles from accessible cameras, exposed and enclosed
back-to-back walls, split-index/signed-zero duplicates, visible sample retention,
tolerance, perspective and sampled-volume views, nonpacked positions, empty and
extreme finite geometry, invalid inputs/options, the inclusive density limits,
workspace recovery, output tails, every work limit through success + 1 and byte
boundaries. Every failing caller-buffer call preserves all sentinel entries.
The constructed fixtures and independent oracle code are dedicated to CC0-1.0.

`exact.rs` compares 46 unchanged preprocessing/repair API-policy combinations on
nine fixtures, cold/repeated/after-buffer-failure/retry, at every work limit
through success + 1 and at byte boundaries. Results/errors, complete destination
and Usage match v0.2.16: **87,272 observations per mode, 174,544 total**. The exact
executions each take less than a second. All pre-existing algorithm source files
except the export/documentation root are verified byte-identical to the base in
`source-binding.json`; existing algorithms are not altered by this feature.

`oracle.cpp` is an independent brute-force implementation of sampled visibility,
not an upstream HSR function. Eighty seeded scenes, half surrounded by a closed
shell, compare all kept index bytes at denominators 1, 2 and 4 against both Rust
feature modes: **480 comparisons passed**. This bounded differential and the
small exact sweeps remain well below two CPU-hours. `cpp-source-binding.json`
checks the vendored C++ source against the pinned 1.3 reference. The four cache
arms additionally agree on every complete real/corpus mesh, including the
no-default-feature Rust driver. The source-only fuzz target is `fuzz.rs`; it was
formatted but intentionally neither built nor run.

`campaign.py` measures warm resident calls with input parsing, initial call and
output serialization outside counters. It uses instructions:u, three interleaved
repetitions (reverse arm order in repetition two), a fixed CPU and median per
cell. The same default options apply to all HSR cases. Output hashes and reports
are identical across repetitions. Raw perf records and all 120 observations are
retained. Wall times are disclosure only. Counter enable/disable control overhead
is included; it is shared by all arms. HSR calls also include the driver's
small options-environment check and report formatting, so these are conservative
whole-driver call costs, not isolated kernel counts. `counts.jsonl` also retains complete
setup-and-run wall times, which include fixture loading and warmup.

The unchanged-cache candidate/base regression gate is <=1.01 for every input.

| Input | Base instructions | Candidate | C++ scalar | C++ default | Candidate/base |
| --- | ---: | ---: | ---: | ---: | ---: |
| tiny-smooth | 29532 | 29548 | 24192 | 24198 | 1.000555 |
| medium-smooth | 10427813 | 10427797 | 8525263 | 8525270 | 0.999999 |
| medium-seams | 9189144 | 9189132 | 7417153 | 7417155 | 0.999999 |
| medium-sparse | 10492251 | 10492235 | 8596322 | 8596326 | 0.999998 |
| million-smooth | 1294050477 | 1294051559 | 1058993823 | 1058993713 | 1.000001 |
| real-214e | 2285091676 | 2285091553 | 1803470683 | 1803470600 | 1.000000 |
| real-a082 | 2638705484 | 2638705015 | 2079323925 | 2079326116 | 1.000000 |
| real-b6fd | 659682696 | 659682799 | 519836992 | 519837041 | 1.000000 |

HSR disclosure with default 26 directions and seven samples per triangle:

| Input | Removed / input triangles | Share | Median call wall seconds | Median instructions | Counted work | Peak charged bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| tiny-smooth | 0 / 32 | 0.000000% | 0.000000 | 136929 | 1868 | 7232 |
| medium-smooth | 0 / 8192 | 0.000000% | 0.005000 | 73405857 | 1563332 | 1851392 |
| medium-seams | 0 / 8192 | 0.000000% | 0.005000 | 68736217 | 1542164 | 1851392 |
| medium-sparse | 0 / 8192 | 0.000000% | 0.005000 | 73405858 | 1563332 | 1851392 |
| million-smooth | 0 / 1000000 | 0.000000% | 1.075929 | 13759148753 | 344372691 | 226000000 |
| real-214e | 70275 / 2851001 | 2.464924% | 58.992144 | 735123832612 | 7213088239 | 644326226 |
| real-a082 | 87292 / 3289725 | 2.653474% | 77.953931 | 931689809765 | 9083058847 | 743477850 |
| real-b6fd | 18643 / 822435 | 2.266805% | 16.873903 | 195994101842 | 1911061507 | 185870310 |

The separated-triangle N versus 8N test (128 versus 1,024 triangles) records
**9,856 versus 124,928 work units: 12.6753x work for 8x input**, below the 16x
bound. This is a counted-work scaling result on the stated fixture, not a
worst-case linear-complexity or wall-time claim.

For performance, v0.2.16 and upstream C++ 1.3 have no HSR operation. HSR's three
peer ratios are explicitly N/A. The four-arm regression gate exercises the
unchanged cache consumer on all eight frozen real/corpus meshes; it cannot be
reported as an HSR speedup or as full-library performance qualification.

The complete final campaign consumed 1526.820 CPU seconds including its builds, oracle, fixture loading and measurements.

`run.sh` reproduces both phases inside a caller-supplied CPU admission wrapper,
using a dedicated target and the HSR_INPUTS, HSR_REFERENCE and HSR_PERF variables.
Its optional `checks` or `campaign` argument selects one phase. Source bindings
cover implementation, tests, guide, drivers and fuzz source; binary and input
hashes are retained. No queue was canceled, and no other model or AI CLI was
invoked. The dedicated target is removed after the last admitted job completes;
`receipts/cleanup.json` records the deletion. Final Git integration is left to
the operator.
