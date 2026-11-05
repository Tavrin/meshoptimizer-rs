# Static virtual geometry: error floor, attributes and materials

## Decisions

| Decision | Reason | Reversal cost |
| --- | --- | --- |
| Separate `clusterlod::v2::build` policy, legacy configuration unchanged | The upstream error estimate remains an upstream parity contract; static streaming builds need the measured floor | New API remains additive; changing its policy requires a builder revision and recook |
| Measure each accepted replacement against its original leaf-triangle union in the bounded workspace, 4000 area-scaled samples plus corners in each direction; max with inherited errors | Step-only maxima do not compose: two unit deviations can accumulate to two. Source regions deduplicate shared refined groups and use positions before snapping; the final pass is serial | Sample policy changes require a revision and recook; no claim of continuous certification |
| Empty output and mixed-slot output pin the original group | Avoids unmeasurable or materially ambiguous replacements after fallback | Quality/storage tradeoff can change in a new builder revision |
| Duplicate by source ID and slot; protect the appended zero-weight slot component | Preserves exact slot identity while keeping one DAG | Replacing this with separate material DAGs changes cache keys and files |
| Stable slot ordering followed by optimization inside each range | Material ranges cover clusters without moving a triangle across slots | Cheap before release; changing triangle order requires a revision |
| Explicitly reject dilation in the new static build path | Its source attribute references and snapped positions must remain attached | A future path needs resnapping and attribute transport; legacy dilation stays available |
| ATTR declarations plus self-describing page attribute headers | Both standalone page validation and resident-layout matching can be checked on untrusted input | Header overhead is measurable; removing it needs a new format tag/version |
| Extend the existing compressed page stream using vertex codec v1/level 2 at stride 4 | Bounded caller-output transcoding, no new dependency; geometry-only page bytes stay identical | A dedicated disk codec can change channel compression without changing memory pages |
| Conservative explicit decode bounds, including normalization/rounding | Consumers can budget attribute loss independently of position displacement | Tighter bounds require new evidence; encoding changes require new IDs |

## Review targets

1. Sampling is a lower estimate of continuous surface distance. Attack thin,
   disconnected or nearly coincident surfaces, sample rotations and error
   propagation across several levels; inspect the cut receipts, especially the
   dragon threshold curve.
2. Attack ATTR header lengths, channel codes, source component offsets, range
   overflow, half infinities, oct/sign padding and page/resident layout mismatch.
   Hash resealing must not bypass structural rejection.
3. Attack sloppy fallback across alternating material seams, empty output,
   sparse slot IDs, material table coverage and coarse-first page ordering.
   Check that stuck reports and pinned residency remain consistent.

WGSL validation is through naga; no GPU execution or fuzz campaign is claimed.
The original position-page WGSL vectors and all MDAG 1 tests remain required.

The unpublished benchmark harness was imported from the diagnostic snapshot at
`f8240140e754614af053ad2a1e8aa58a71fc2d78`. This lane starts from
`420220c49b375936a3d47b45955638e86a99cf63`. Historical tables remain explicitly
labelled with their original revision and format; new rows must come from the
current library entry point rather than the harness's `measured=1` prototype.

A required 16-byte `MATS` declaration marks material-bearing writes, including
those without ATTR channels, so older readers fail at resident open. Reversal
cost: a new tag/version after release; position-only files need no declaration.

The private lane brief is retained with the external validation evidence rather
than the public source tree, because it contains machine-specific admission and
corpus paths. The public-text gate remains unchanged.

## Earlier revision 3 validation

All required code gates exited zero, including the fuzz-target build without a campaign. The five-mesh cut bar and the dragon threshold trend passed. The real multi-material fixture checked 138,106 decoded vertices across 6 slots; maximum UV component error 7.5101852e-06, maximum normal vector error 6.1737235e-05. Full timing, size and quality tables are in `parity/vg-bench/RESULTS.md`.

## Build-path review repair

Builder revision 4 adds original-region measurements after ordered DAG assembly.
The simplifier estimate and existing step floor remain additional conservative
floors; the legacy recurrence and MDAG 1 output remain unchanged. Errors bound
the sampled original-source deviation, not the continuous surface distance.
The final pass propagates raised child errors before measuring each parent.
Its cost is recorded against revision 3 in the benchmark results. Retaining
only `max(previous, step)` was rejected because it fails the two-unit
composition reproduction. Changing the legacy merge recurrence was rejected
because upstream parity must remain intact and the v2 requirement explicitly
asks for source-region measurements. The serial final pass preserves geometry
and parallel assembly order at the expense of build time.

Attribute F16 decoding expands every finite half subnormal exactly, including
signed zero, matching the WGSL reference. The legacy quantization helper keeps
its upstream flush policy. Fixed vectors cover both subnormal endpoints and
signs; the regression exhausts finite half encodings for F16x2 and F16x4.

Repair validation: formatting, all-target/all-feature Clippy with warnings denied,
release tests, all-feature release tests, and the public-text gate all exited 0.
This includes the original-region regression, parallel determinism, legacy and
MDAG 1 suites, half subnormal vectors, and naga validation. All 19 registered
fuzz targets built with address-sanitizer instrumentation; no campaign was run.
The source-region measurements remain sampled evidence. The five-mesh paired
benchmark reports their cooking and cut-triangle costs separately from code gates.
A direct archived-baseline comparison also passed on the existing nonplanar
4x4, 12x12 and 24x24 grid fixture family: complete legacy group/cluster output
and modified positions matched byte-for-byte in canonical debug form, and
serialized MDAG 1 files matched byte-for-byte at all three sizes.

The repaired rocks and dragon p95 deviation curves plateau at coarser
thresholds. The results retain those curves and the higher selected-triangle
counts alongside the increased cooking times; no cut-efficiency improvement
is inferred from the lower measured/claimed ratios.


## Local source-region ownership

Decision: keep each v2 replacement's sibling clusters in one next-level
partition. Recursive source regions otherwise include sibling geometry owned
by other parents, inflating source-to-replacement distances before inherited
maxima spread them further. Component pruning is already disabled; disabling
it again cannot address this ownership problem. Preserve the original-source
regression and upward-rounded sampled bound rather than reducing stored errors.

Builder revision 5 identifies the changed grouping; MDAG 2 records and the
legacy build path are unchanged. Independent source-region measurements use
worker-sized parallel batches only for unlimited workspaces, with summed live
scratch peaks and ordered inherited-error propagation. Finite limits stay
serial. The sample rule remains 4000 area-scaled interiors, per-triangle
rounding and all corners, using triangle spatial indices in both directions.

Qualification and any remaining acceptance failures are recorded in the
current section of `parity/vg-bench/RESULTS.md`.

Rejected alternatives: lowering or renormalizing measured errors would discard
the original-source guarantee; replacing it with step-only maxima fails the
successive-unit-step regression. Disabling component pruning cannot repair this
path because the simplifier does not enable the prune option. Assigning source
triangles to one of several sibling parents would need a new provenance rule;
keeping the already-defined replacement region intact preserves the existing
union-based regression and source-region meaning. Alpha and material guards are
absent from the five-mesh geometry benchmark inputs.


## Local source-region code validation

Formatting, all-target/all-feature Clippy with warnings denied, release tests,
all-feature release tests and public-text checks exited 0. Direct starting-source
comparisons matched complete legacy records and byte-identical MDAG 1 files on
the nonplanar 4/12/24 grid family. All 19 registered fuzz targets built with ASAN;
no campaigns ran. The first fuzz driver attempt compiled every target but exited
1 when writing its receipt into a missing scratch directory. After creating that
directory, the full build driver and receipt write exited 0. The initial failure
is retained with the external evidence.

## Local source-region qualification

The measured candidate does not pass all numerical acceptance bars. It is retained as an unqualified result.
Full quality, timing and per-level distributions are recorded in `parity/vg-bench/RESULTS.md`.

Failed numerical bars:
- bunny: timing.
- dead_tree_trunk_02: timing.
- namaqualand_boulder_02: timing.
- coast_rocks_01: triangles, timing.
- dragon: timing.

IBS op sampling is separate from wall-time repeats. Baseline attribution:
63.21% nearest-triangle queries, 16.81% memory moves, 7.08% BVH partition selection,
3.40% BVH range construction (562K samples, none lost). Kernel symbols were restricted;
user-space function attribution is available.
Candidate nearest-triangle query: 76.49% of samples.
Candidate BVH partition selection: 3.74% of samples.
Candidate BVH range construction: 1.93% of samples.

The source measurement retains every former sample: per-triangle-rounded
area-scaled interiors and all corners. Its cost is not hidden by lowering errors
or weakening the original-source regression. No continuous surface bound is claimed.
