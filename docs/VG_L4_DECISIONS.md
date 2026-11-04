# Static virtual geometry: error floor, attributes and materials

## Decisions

| Decision | Reason | Reversal cost |
| --- | --- | --- |
| Separate `clusterlod::v2::build` policy, legacy configuration unchanged | The upstream error estimate remains an upstream parity contract; static streaming builds need the measured floor | New API remains additive; changing its policy requires a builder revision and recook |
| Measure each accepted step in the existing bounded workspace, 4000 area-scaled samples plus corners in each direction | Prices lost geometry with the same quality oracle as the benchmark; parallel preparation preserves ordered assembly | Sample policy changes require a revision and recook; no claim of continuous certification |
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

## Completed validation

All required code gates exited zero, including the fuzz-target build without a campaign. The five-mesh cut bar and the dragon threshold trend passed. The real multi-material fixture checked 138,106 decoded vertices across 6 slots; maximum UV component error 7.5101852e-06, maximum normal vector error 6.1737235e-05. Full timing, size and quality tables are in `parity/vg-bench/RESULTS.md`.
