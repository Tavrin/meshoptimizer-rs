# Foliage and far-field build policy

The alpha guard is opt-in through `v2::BuildMesh::alpha_coverage`. UVs are two
consecutive source attribute components. Masks are indexed by material slot;
unsupplied slots are opaque. `AlphaCoverage::new` selects a ratio band of
[0.93, 1.07], maximum blurred difference 0.12, and 32 pixels per view side.

The guard uses the crate's scalar impostor rasterizer, double sided, with a
fixed sphere grid of 3 by 3 views (seam directions repeat), two samples per
axis, alpha cutoff 0.5, nearest repeat texture sampling, coverage threshold 1,
and no dilation. Both meshes use the input mesh's framing sphere. Every view
must pass the covered-area ratio and mean absolute 3 by 3 box-blurred alpha
difference; the blur clamps to tile edges. Empty input views must remain empty.

Decision: reject a failing simplification step, retaining its input group as
stuck with error `f32::MAX`. Such regions are parentless and the writer pins
them. Raising an error from a silhouette mismatch would require an unsupported
mapping from raster disagreement to world displacement. The report counts
alpha-stuck groups and error raises separately; raises are always zero.
Already-stuck groups are not tested again. Groups with no supplied alpha mask
skip rendering. The disabled path keeps existing geometry, keys and file bytes.
An alpha-enabled key uses a separate versioned domain and includes every mask
texel, presence, dimension, UV component and threshold setting. Callers must
populate `KeyInputs::alpha_coverage` with the same build policy.

Per tested group the guard makes two scalar bakes. For resolution R (8 to 128)
and T triangles, worst-case raster work is O(9 T (2R)^2), with O(V + 9 R^2)
storage, where V is the build vertex count. It additionally scans input indices,
copies the V UV pairs, and compares nine tiles with nine taps per pixel. Raster,
copy, comparison, live atlas memory and parallel speculation are charged to
workspace limits. The caller can bound work and memory; exhaustion is an error,
never an unguarded accepted step. The topology-based measured error floor is
still applied to accepted steps.

`FARF` is optional and independent of material tables. It stores metadata only;
external payload allocation, decoding, authentication and texture lifetime are
caller responsibilities. A helper hashes the payload and rounds the larger
switch error upward to f32. The runtime projects that world error with the
all-level mesh sphere and uses the same <= pixel threshold handoff as clusters.
The caller must compute its `ImpostorSwitch` from that same sphere. Hemisphere
metadata is valid only for views in the upper hemisphere. Payload hashes detect
accidental changes, not tampering. Unknown optional tags remain skippable.

`measure_fallback` extracts every parentless region, including stuck regions,
and measures against all source clusters with `measure_lod_quality`. This is a
sampled surface distance, a lower estimate of Hausdorff distance, and cannot
certify ray errors. Interior sampling density belongs to the caller.

Residuals: the fixed-view proxy cannot promise coverage under arbitrary cameras,
filtering, mips, animation, or temporal reconstruction. Per-step coverage is
also not a global cumulative silhouette certificate. The benchmark records
whole-cut behavior at multiple distances; consumers still need their own view
gates. Skinned and morphed geometry, GPU validation and certified ray fallback
bounds remain outside this lane.
