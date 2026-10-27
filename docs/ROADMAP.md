# Development phases and roadmap

## Phases

Crate 0.2.x includes the following development phases. Release numbers label stages of the port, not
published crate versions.

| Release | Included APIs |
|---|---|
| 0.1 / 0.1.x core | Checked views, errors, workspaces and limits; remapping, vertex cache/fetch and overdraw optimization, simplification and quantization. |
| 0.2 glTF decoding | Vertex/index decoding and checked `EXT_meshopt_compression` buffer views, including filters. |
| 0.3 meshlets | Scan, standard, flex and spatial builders; bounds, meshlet optimization, partitioning, spatial ordering and optional cluster LOD. |
| 0.4 codecs | Vertex/index/sequence encoders and decoders, versions and compression levels, Oct/Quat/Exp/Color filters and meshlet codecs. |
| 0.5 scalar algorithms | Stripification, cache/fetch/overdraw/coverage analysis, opacity micromaps, tangents, experimental normals and remeshing. |
| 0.6 parallel batches | Ordered batches for LOD chains, encoding, EXT views, meshlets, cluster-LOD DAGs and hierarchy forests. |
| 0.7 SIMD decoding | Vertex, filter and meshlet kernels for x86 SSE2/SSSE3/SSE4.1, AArch64 NEON and wasm simd128; scalar reference and fallback. |

## Cluster milestone priorities (additive 0.2.x)

1. Deliver canonical caller-buffer cluster DAGs and ordered batches. The new
   `clusterlod::build_dag_into` returns emission-time position snapshots, flat cluster/group
   records and the per-level forest in `DagBuffers`; `parallel::build_dags_batch_into` returns
   ordered counts or item errors. Caller capacities are excluded from Workspace limits. Publish
   successful prefixes only; late failures may write prefixes and mutate dilated positions.
2. Add bounded native codec cursors after exact legacy output/error and checkpoint qualification.
   Existing whole-buffer decoders remain supported.
3. Use qualified scratch control at mesh/release boundaries; retain existing `Workspace::clear` and
   limits semantics.
4. Reproduce codec, partition/fetch/sloppy/overdraw and SIMD residuals, then refresh integrated
   qualification. Existing unresolved rows remain unresolved. Performance and platform/release
   evidence are separate from DAG byte identity.

## 0.3

Reserve 0.3 for the first necessary breaking change. Serialization formats stay with applications; the
library remains format-agnostic. The stricter SIMD amendment targets are also left for 0.3.
