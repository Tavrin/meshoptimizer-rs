# Exact-comparison domain and current coverage

SPEC-0216b replaces the earlier unbounded enumeration request with a release
standard: the existing small-mesh set and worst cases for precharge bounds,
each work limit through success plus one, every accepted option/flag value
combination and each supported index width and stride, in both feature modes.
The data inventory for this correction is `enumeration-0216b.json`; its required
per-function sizes remain unknown because the required product is not yet
implemented or executed. The earlier `enumeration.json` describes the
superseded wording, not a reason to reject the corrected standard.

The retained drivers establish subsets, not the corrected release proof:

* Meshlet and cluster LOD have 154,590 records in each mode, split into 7,460
  build_meshlets, 7,454 build_meshlets_flex, 7,079 build_meshlets_spatial and
  132,597 cluster LOD. Meshlet settings and layouts are selected; LOD options
  are varied individually instead of enumerating their combinations. The
  no_std LOD build enables `clusterlod`, which is required for that API.
* The simplify driver produces 1,200,243,710 all-feature and 273,906,930
  no-default observations across six functions combined. It crosses all
  accepted option masks with eight selected position layouts, three attribute
  layouts, widths 0/1/3/32 and two selected flag patterns. The totals are not
  per-function enumeration sizes and do not cover all supported strides or
  flag combinations on all meshes.
* The overdraw driver produces 20,558,564 observations in each mode. Its
  stride sweep uses selected work limits, while its every-work-limit sweeps
  use selected layouts. The two dimensions have not been crossed over the
  required small-mesh set.

These observations compare results, usage and relevant full output buffers;
they do not establish the missing cross-product. The supervisor failures and
corrected subset runs remain separately recorded in `proof-results.json`.
