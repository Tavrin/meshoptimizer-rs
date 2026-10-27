# Peer comparison registration

Source entry: v0.2.13, 42782f5. Isolated consumers only. Exact registry
versions: meshopt 0.7.0 (C++ 1.3) and optimesh 1.1.0.

The existing five benchmark meshes and three retained real LOD-chain input
meshes supply unchanged packed XYZ and indices. This campaign compares the
common packed projection rather than the original 64-byte vertex layout.
Four synthetic attributes use the existing comparison harness formula and
unit weights. Decode inputs are generated once by the C++ 1.3 arm.
Simplification targets half the input index count, rounded down to triangles,
with relative error 0.5, no flags or locks. Overdraw threshold 1.05.
Meshlets: 64 vertices, 124 triangles, cone weight zero. Vertex encoding:
version 1, level 2; index encoding: version 1, 32-bit decoding.

All arms: release, opt-level 3, thin LTO, one codegen unit, one build job,
no target-cpu override. SIMD enabled in both Rust ports; C++ uses its default
build. Ours uses default std+simd plus clusterlod, with a 3 GiB Workspace storage
cap and the default counted-work limit. The default 1 GiB cap rejected one
real attribute case; the initial campaign is retained and excluded in full. Every build, fixture call,
validation and measurement requires CPU-only shared admission, <=4 GB declared.

Metric: instructions:u, acknowledged perf enable/disable around a resident
loop, median of three samples. A warmup first allocates workspace capacity.
Output allocation, required input copies, library scratch and destruction are
counted. Parsing, fixture encoding, serialization, meshlet normalization and
independent validation are excluded. Ours retains Workspace capacity; the
peers expose no equivalent reusable scratch handle. Meshlet descriptor
conversion is counted for all arms. Wall time per call and process peak RSS
(including parsing/warmup/serialization) are disclosures on a shared host.

Interleave the three arms, reverse order in the middle repetition. Pin one
allowed logical CPU. Reject missing/multiplexed counters, nonzero exits,
output instability or invalid output; preserve all rejected artifacts.
Keep every completed sample and resume missing keys only. No performance
threshold or favorable-sample selection. Report every output difference,
including first byte, lengths, and hashes. Meshlet raw layout and normalized
packed identity are separate. Cluster LOD is shared by ours and C++, absent
from optimesh; use rasterization defaults (128), positions only, no dilation,
and canonical callback records including bounds, indices and refinement IDs.

Work is bounded to about five active hours excluding admission holds. Any
unmeasured cells or incomplete API coverage remain explicit in the report.

Supplement: replay all eighteen retained simplify-chain stages as packed XYZ,
using each existing index blob, target and error (options zero). These records
use separate stage-counts and campaign files. No codec fixtures are needed for
these simplify-only cells. Chain ratios use sums of six per-stage medians.
