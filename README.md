# meshoptimizer-rs

[![CI](https://img.shields.io/github/actions/workflow/status/Tavrin/meshoptimizer-rs/ci.yml?branch=main&label=CI)](https://github.com/Tavrin/meshoptimizer-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/meshoptimizer-rs.svg)](https://crates.io/crates/meshoptimizer-rs)
[![docs.rs](https://img.shields.io/docsrs/meshoptimizer-rs)](https://docs.rs/meshoptimizer-rs)
[![License: MIT](https://img.shields.io/crates/l/meshoptimizer-rs)](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/LICENSE)
[![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/Cargo.toml)

`meshoptimizer-rs` is a Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3 for asset pipelines without a C++ toolchain and for wasm applications.
It checks untrusted meshes with per-call work and byte limits and supports `no_std` with `alloc`.

```toml
[dependencies]
meshoptimizer-rs = "0.2"
```

The minimum Rust version is 1.88. The library name is `meshoptimizer_rs`.

## Usage

Task recipes: [the `guide` module](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html) ([source](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/USING.md)).

Coming from the `meshopt` crate? See [docs/MIGRATING_FROM_MESHOPT.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/MIGRATING_FROM_MESHOPT.md).

Every operation takes a `&mut Workspace` and returns `Result<_, meshoptimizer_rs::Error>`. Most
have an allocating form, a caller-buffer `_into` form and, where it applies, an `_in_place` form.

### Simplify and optimize a mesh

Simplify to a quarter of the triangles, reorder triangles for the vertex cache and overdraw, then
reorder vertices for fetch.

```rust
use meshoptimizer_rs::{
    optimize_overdraw, optimize_vertex_cache, optimize_vertex_fetch, simplify, Error, FetchedMesh,
    Positions, SimplifyOptions, SimplifySettings, VertexStream, Workspace,
};

fn make_lod(positions: &[[f32; 3]], indices: &[u32]) -> Result<FetchedMesh, Error> {
    let mut workspace = Workspace::default();
    let view = Positions::from_packed(positions);
    let target_index_count = indices.len() / 4;
    let settings = SimplifySettings { target_index_count, target_error: 0.01, options: SimplifyOptions::EMPTY };
    let lod = simplify(indices, view, settings, &mut workspace)?;
    let lod = optimize_vertex_cache(&lod.indices, positions.len(), &mut workspace)?;
    let lod = optimize_overdraw(&lod, view, 1.05, &mut workspace)?;

    let bytes: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
    optimize_vertex_fetch(&lod, VertexStream::packed(&bytes, 12)?, &mut workspace)
}
```

To account for attributes such as normals, call `simplify_with_attributes` with an `Attributes`
view and one weight per component, for example
`Attributes::from_interleaved(&normals, vertex_count, 3, 3, 0)?` and `&[0.5; 3]`. Interleaved
positions use `Positions::from_interleaved`; stride and offset are in `f32` elements.

For repeated fetch-remap calls, reuse `FetchIndices::new(indices, positions.len())` with
`optimize_vertex_fetch_remap_with_indices` or its `_into` form to avoid repeated index validation.

### Meshlets and bounds

```rust
use meshoptimizer_rs::{
    build_meshlets, compute_meshlet_bounds, Bounds, Error,
    MeshletSettings, Positions, Workspace,
};

fn meshlet_bounds(positions: &[[f32; 3]], indices: &[u32]) -> Result<Vec<Bounds>, Error> {
    let mut workspace = Workspace::default();
    let view = Positions::from_packed(positions);
    let meshlets = build_meshlets(indices, view, MeshletSettings::default(), 0.25, &mut workspace)?;

    let mut bounds = Vec::new();
    for m in &meshlets.meshlets {
        let (v, t) = (m.vertex_offset as usize, m.triangle_offset as usize);
        let vertices = &meshlets.vertices[v..v + m.vertex_count as usize];
        let triangles = &meshlets.triangles[t..t + m.triangle_count as usize * 3];
        bounds.push(compute_meshlet_bounds(vertices, triangles, view, &mut workspace)?);
    }
    Ok(bounds)
}
```

`build_meshlets_scan`, `build_meshlets_flex` and `build_meshlets_spatial` cover the other upstream
builders. `compute_cluster_bounds` computes bounds for index-list clusters.
Reuse `BoundsPositions` for repeated bounds queries over the same positions.

### Cluster LOD DAG

With the `clusterlod` feature, `build_dag_into` builds the cluster DAG and its per-level node
forest into buffers you own. Allocate each `DagBuffers` slice, for example with
`vec![Default::default(); capacity]`. The returned `DagCounts` gives the length of each filled
prefix, such as `indices[..counts.indices]`.

```rust
#[cfg(feature = "clusterlod")]
use meshoptimizer_rs::{clusterlod::{self, DagBuffers, DagCounts, Mesh}, Workspace};

#[cfg(feature = "clusterlod")]
fn build_dag(
    indices: &[u32],
    positions: &mut [[f32; 3]],
    buffers: DagBuffers<'_>,
) -> Result<DagCounts, meshoptimizer_rs::Error> {
    let mesh = Mesh { indices, positions, attributes: None, vertex_lock: None,
                      attribute_weights: &[], attribute_protect_mask: 0 };
    let config = clusterlod::default_config(128)?;
    clusterlod::build_dag_into(config, mesh, 8, buffers, &mut Workspace::default())
}
```

Only the returned prefixes are output. If a call fails, including with `BufferTooSmall`, discard
the output, restore the original positions (border dilation may have moved them) and retry with
larger buffers. Caller buffers don't count toward `Workspace` limits.

With `parallel` as well, `parallel::build_dags_batch_into(&mut inputs, Limits::default())` builds
several meshes at once. Each `DagBuildInput` holds the `config`, `mesh`, `node_width` and
`destination` arguments above. It returns one `Result<DagCounts, Error>` per input, in input
order, and each item has its own limits.

### Vertex and index compression

The `codec` module implements the codecs used by `EXT_meshopt_compression`, plus index sequence
and meshlet codecs and the Oct, Quat, Exp and Color filters. `VertexEncoding` and `IndexEncoding`
set the format version and level per call. `decode_buffer_view` decodes a glTF bufferView using the
mode and filter names from its JSON.

```rust
use meshoptimizer_rs::codec::{
    decode_buffer_view, decode_vertex_buffer, encode_index_buffer, encode_vertex_buffer, Filter,
    IndexEncoding, Mode, VertexEncoding,
};
use meshoptimizer_rs::{Error, Workspace};

fn compress(vertices: &[u8], stride: usize, indices: &[u32]) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let mut workspace = Workspace::default();
    let count = vertices.len() / stride;
    let encoding = VertexEncoding::DEFAULT;
    let packed_vertices = encode_vertex_buffer(vertices, count, stride, encoding, &mut workspace)?;
    assert_eq!(decode_vertex_buffer(count, stride, &packed_vertices, &mut workspace)?, vertices);

    let packed_indices = encode_index_buffer(indices, IndexEncoding::DEFAULT, &mut workspace)?;
    Ok((packed_vertices, packed_indices))
}

fn decode_view(mode: &str, filter: &str, n: usize, stride: usize, data: &[u8]) -> Result<Vec<u8>, Error> {
    let (mode, filter) = (Mode::parse(mode)?, Filter::parse(filter)?);
    decode_buffer_view(mode, filter, n, stride, data, &mut Workspace::default())
}
```

### Workspace and limits

Keep one `Workspace` per thread and reuse it: it keeps scratch memory between calls. Its `Limits`
cap the bytes the crate allocates and the work units a call may spend; the defaults are 1 GiB and
2^34. Set them with `Workspace::new(Limits { max_bytes: 64 << 20, max_work: 1 << 30 })`. A call
over a limit returns `Error::LimitExceeded`, and `Workspace::usage` reports the last call's bytes
and work.
Call `workspace.set_retention(Retention::Lean)` for lower peak memory in long simplify chains at some time cost.

### Beyond upstream

- `remove_hidden_triangles` and `_into`: deterministic tiled software visibility (BVH rays for finite cameras),
  exact duplicate removal, caller viewpoints, and checked resource limits.
  See the [hidden-surface recipe](docs/USING.md#hidden-surface-removal); finite
  samples do not prove continuous invisibility.

These opt-in APIs extend C++ meshoptimizer.

- [`codec::IndexSequenceDecoder`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#streaming-index-sequences) incrementally decodes arriving index-sequence bytes into caller buffers with a whole-stream work limit. Output remains provisional until `finish()` validates EOF.
- [`weld_vertices`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) maps nearby vertices to original representatives with an explicit seam policy.
- [`orient_faces`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) propagates winding across two-face edges and reports unresolved conflicts.
- [`fill_holes`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) triangulates small planar boundary loops using existing vertices and reports refused loops.
- [`clusterlod::build_parallel`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#parallel-groups-within-one-cluster-lod-mesh) processes groups within one mesh using the current Rayon pool (`parallel` + `clusterlod`).
- [`sanitize`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#mesh-repair) removes invalid, repeated-index and duplicate triangles and reports counts by reason.
- [`analyze_mesh`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#mesh-topology-analysis) reports boundary edges, non-manifold topology, orientation conflicts and connected components.
- [`VertexScoreTable` and `optimize_vertex_cache_with_table`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#tuning-for-a-specific-gpu-cache) accept caller-defined vertex-cache scores.
- [Typed vertex layouts](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#typed-rust-vertex-fields) build position and attribute views from initialized Rust struct fields.
- [`build_lod_chain`](https://docs.rs/meshoptimizer-rs/0.2.16/meshoptimizer_rs/guide/index.html#one-call-lod-chain) builds successive or independent simplification levels with whole-call limits and per-level errors.

## Features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | `std::error::Error` for `Error`. |
| `simd` | yes | SIMD vertex, filter and meshlet decoding for SSE2/SSSE3/SSE4.1, NEON and wasm simd128. |
| `parallel` | no | Rayon batch APIs and, with `clusterlod`, parallel groups within one mesh; enables `std`. |
| `clusterlod` | no | The `clusterlod` module, a port of upstream `demo/clusterlod.h`. |
| `experimental` | no | Upstream experimental functions, including normal generation and remeshing. |

For `no_std`, set `default-features = false`; an allocator is required. Without `simd` the crate
contains no `unsafe` code. Cargo feature unification applies: if another dependency enables
`simd`, it is on for your build too.

## Correctness and performance

On the recorded x86-64 differential inputs, successful outputs match scalar
and default C++ meshoptimizer 1.3 at commit
`4c203430ca565cb59a468a91922c76c208169536`, whose `src` tree equals tag v1.3,
for matching settings. Failed vertex decodes can leave different destination
prefixes; the recorded error statuses agree. Arm64 CI checks Rust
NEON against Rust scalar and C++ scalar; C++ NEON divergences are disclosed
separately. These checks do not prove equivalence for every input; see
[docs/VERIFICATION.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/VERIFICATION.md)
and [UPSTREAM.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/UPSTREAM.md).

The table gives 0.2.16 instruction-count ratios on three real meshes as
geometric means of per-mesh ratios (0.2.16 / comparison). Both comparisons are
shown: scalar vs scalar C++ 1.3 and default vs default C++ 1.3, via `meshopt`
0.7.0. optimesh 1.1.0 is scalar in both columns. Values below 1 mean fewer
instructions; instruction counts do not establish wall-time speedups.

| Function | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 | scalar vs optimesh 1.1.0 | default vs optimesh 1.1.0 |
|---|---:|---:|---:|---:|
| `simplify` | 1.138 | 1.138 | 0.899 | 0.899 |
| `simplify_with_attributes` | 1.096 | 1.096 | 0.927 | 0.927 |
| `simplify_sloppy` | 1.275 | 1.275 | 0.706 | 0.706 |
| `simplify_with_update` | 1.203 | 1.203 | 0.947 | 0.947 |
| `simplify_prune` | 0.995 | 0.995 | 0.763 | 0.763 |
| `simplify_points` | 1.014 | 1.014 | 0.632 | 0.632 |
| `optimize_vertex_cache` | 1.254 | 1.254 | 0.919 | 0.919 |
| `optimize_overdraw` | 1.090 | 1.090 | 0.884 | 0.884 |
| `optimize_vertex_fetch` | 0.952 | 0.952 | 0.716 | 0.716 |
| `build_meshlets` | 1.001 | 1.001 | 0.784 | 0.784 |
| `build_meshlets_flex` | 1.002 | 1.002 | 0.784 | 0.784 |
| `build_meshlets_spatial` | 0.965 | 1.083 | 0.835 | 0.783 |
| cluster LOD | 1.189 | 1.188 | — | — |
| `encode_vertex_buffer` | 0.554 | 0.554 | 0.455 | 0.455 |
| `decode_vertex_buffer` | 1.061 | 1.145 | 0.544 | 0.174 |
| `encode_index_buffer` | 0.771 | 0.771 | 0.712 | 0.712 |
| `decode_index_buffer` | 0.994 | 1.024 | 0.907 | 0.934 |
| `remap_vertex_buffer` | 1.087 | 1.087 | 0.353 | 0.353 |
| `remap_index_buffer` | 1.437 | 1.437 | 0.863 | 0.863 |
| `compute_cluster_bounds` | 1.138 | 1.081 | 0.734 | 0.697 |
| `compute_meshlet_bounds` | 1.162 | 1.104 | 0.731 | 0.695 |

Cluster LOD has no optimesh comparison. In separate CPU-only wall measurements,
parallel cluster LOD had a 1.605–1.780× speedup with four workers versus one
on these three meshes (one-worker time / four-worker time).
See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/PERFORMANCE.md)
for real-mesh and corpus tables, the comparison with 0.2.15, parallel wall times,
method and host.
Planned work is in [docs/ROADMAP.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/docs/ROADMAP.md).

## Known exceptions

- A malformed meshlet tail is accepted by scalar upstream and by this crate, but rejected by upstream SSE.
- AVX2, AVX-512 and 32-bit ARM SIMD are not implemented; index and sequence decoding is scalar.
- The instruction counts above describe the listed x86-64 workloads; they do not establish performance on other architectures or per-case wall-time limits.

## Licence and acknowledgements

MIT; see [LICENSE](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.16/LICENSE). meshoptimizer
is written by Arseny Kapoulkine. This independent port keeps upstream's licence notice and credits
its algorithms; it is not affiliated with or endorsed by the upstream project. UPSTREAM.md records
provenance and intentional API differences.

Beyond upstream: `measure_lod_quality` and `measure_lod_chain_quality` measure sampled surface/normal deviation and camera LOD switch distances; this API is not in C++ meshoptimizer. See the [recipe](docs/USING.md#lod-quality-and-switch-distances).
