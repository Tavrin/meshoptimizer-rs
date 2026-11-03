# meshoptimizer-rs

[![CI](https://img.shields.io/github/actions/workflow/status/Tavrin/meshoptimizer-rs/ci.yml?branch=main&label=CI)](https://github.com/Tavrin/meshoptimizer-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/meshoptimizer-rs.svg)](https://crates.io/crates/meshoptimizer-rs)
[![docs.rs](https://img.shields.io/docsrs/meshoptimizer-rs)](https://docs.rs/meshoptimizer-rs)
[![License: MIT](https://img.shields.io/crates/l/meshoptimizer-rs)](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/LICENSE)
[![MSRV: 1.88](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/Cargo.toml)

`meshoptimizer-rs` is a Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3 for asset pipelines without a C++ toolchain and for wasm applications.
It checks untrusted meshes with per-call work and byte limits and supports `no_std` with `alloc`.

```toml
[dependencies]
meshoptimizer-rs = "0.2"
```

The minimum Rust version is 1.88. The library name is `meshoptimizer_rs`.

## Usage

Task recipes: [the `guide` module](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html) ([source](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/docs/USING.md)).

Coming from the `meshopt` crate? See [docs/MIGRATING_FROM_MESHOPT.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/docs/MIGRATING_FROM_MESHOPT.md).

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

`clusterlod::dag_format::write` serializes those prefixes into versioned little-endian
MDAG files with independently decodable geometry pages (default capacity 128 KiB).
`dag_format::Reader::read` validates tables, refinement links and geometry;
`decode_page` decodes one fetched page. This extension is not in C++ meshoptimizer
and is available without Cargo features. See [DAG_FORMAT.md](docs/DAG_FORMAT.md)
for the wire layout, resource limits and compatibility policy.

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

These opt-in APIs extend C++ meshoptimizer.

- [`process_mesh`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#one-call-asset-pipeline) runs sanitation, welding, optional repair, LODs, cache, overdraw and fetch optimization, quality reports and optional meshlets and codecs in one call with a combined report.
- [`simplify_character` and `build_character_lod_chain`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#character-lods) simplify skinned and morphed meshes with bone-weight and blend-target metrics; `CharacterOptions::best_of` keeps the better of two candidates per level.
- [`remove_hidden_triangles`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#hidden-surface-removal) drops triangles that no tested view sees, with exact duplicate removal and checked resource limits. Finite samples do not prove continuous invisibility.
- [`measure_lod_quality` and `measure_lod_chain_quality`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#lod-quality-and-switch-distances) measure sampled surface and normal deviation and camera switch distances.
- [`clusterlod::dag_format`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#cluster-lod-dag-files) writes and reads cluster LOD DAGs as paged files that decode one page at a time.
- [`_generic` functions](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#16-bit-vertex-indices) accept `u16` or `u32` indices with checked narrowing.
- [`codec::VertexBufferDecoder` and `codec::IndexBufferDecoder`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#incremental-vertex-and-triangle-decoding) decode chunked input into chunked output with fixed storage.
- [`split_non_manifold_vertices` and `stitch_cracks`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#mesh-repair) separate disconnected face fans and merge boundary cracks, with counts of refused candidates.
- [`parallel::encode_vertex_buffer_parallel`, `decode_vertex_buffer_parallel` and `build_meshlets_spatial_parallel`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#parallel-vertex-codec-and-spatial-meshlets-for-one-large-mesh) split one large buffer or spatial meshlet build across the current Rayon pool with output identical to the serial functions.
- [`codec::IndexSequenceDecoder`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#streaming-index-sequences) incrementally decodes arriving index-sequence bytes into caller buffers with a whole-stream work limit. Output remains provisional until `finish()` validates EOF.
- [`weld_vertices`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) maps nearby vertices to original representatives with an explicit seam policy.
- [`orient_faces`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) propagates winding across two-face edges and reports unresolved conflicts.
- [`fill_holes`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#tolerance-welding-orientation-and-small-holes) triangulates small planar boundary loops using existing vertices and reports refused loops.
- [`clusterlod::build_parallel`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#parallel-groups-within-one-cluster-lod-mesh) processes groups within one mesh using the current Rayon pool (`parallel` + `clusterlod`).
- [`sanitize`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#mesh-repair) removes invalid, repeated-index and duplicate triangles and reports counts by reason.
- [`analyze_mesh`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#mesh-topology-analysis) reports boundary edges, non-manifold topology, orientation conflicts and connected components.
- [`VertexScoreTable` and `optimize_vertex_cache_with_table`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#tuning-for-a-specific-gpu-cache) accept caller-defined vertex-cache scores.
- [Typed vertex layouts](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#typed-rust-vertex-fields) build position and attribute views from initialized Rust struct fields.
- [`build_lod_chain`](https://docs.rs/meshoptimizer-rs/0.2.17/meshoptimizer_rs/guide/index.html#one-call-lod-chain) builds successive or independent simplification levels with whole-call limits and per-level errors.

## Features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | `std::error::Error` for `Error`. |
| `simd` | yes | SIMD vertex, filter and meshlet decoding for SSE2/SSSE3/SSE4.1, NEON and wasm simd128, plus AVX-512 VBMI/VBMI2 vertex decoding selected at run time. |
| `parallel` | no | Rayon batch APIs, parallel vertex encode and decode and spatial meshlets for one large mesh and, with `clusterlod`, parallel groups within one mesh; enables `std`. |
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
[docs/VERIFICATION.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/docs/VERIFICATION.md)
and [UPSTREAM.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/UPSTREAM.md).

The table gives 0.2.17 instruction-count ratios on three real meshes as
geometric means of per-mesh ratios (0.2.17 / C++ 1.3). Both comparisons are
shown: scalar vs scalar C++ 1.3 and default vs default C++ 1.3, via `meshopt`
0.7.0. Values below 1 mean fewer instructions; instruction counts do not
establish wall-time speedups. The default build selects its vertex-decode
kernel at run time; the measurement host reports AVX-512 VBMI/VBMI2.

| Function | scalar vs scalar C++ 1.3 | default vs default C++ 1.3 |
|---|---:|---:|
| `simplify` | 1.174 | 1.174 |
| `simplify_with_attributes` | 1.118 | 1.118 |
| `simplify_sloppy` | 1.284 | 1.284 |
| `simplify_with_update` | 1.238 | 1.238 |
| `simplify_prune` | 0.971 | 0.971 |
| `simplify_points` | 1.032 | 1.032 |
| `optimize_vertex_cache` | 0.706 | 0.706 |
| `optimize_overdraw` | 1.096 | 1.096 |
| `optimize_vertex_fetch` | 0.908 | 0.908 |
| `build_meshlets` | 1.005 | 1.005 |
| `build_meshlets_flex` | 1.005 | 1.005 |
| `build_meshlets_spatial` | 0.955 | 1.062 |
| cluster LOD | 1.224 | 1.223 |
| `encode_vertex_buffer` | 0.545 | 0.545 |
| `decode_vertex_buffer` | 1.128 | 0.979 |
| `encode_index_buffer` | 0.770 | 0.770 |
| `decode_index_buffer` | 0.994 | 1.024 |
| `remap_vertex_buffer` | 1.087 | 1.087 |
| `remap_index_buffer` | 1.354 | 1.354 |
| `compute_cluster_bounds` | 1.089 | 1.039 |
| `compute_meshlet_bounds` | 1.122 | 1.072 |

Against 0.2.16, `optimize_vertex_cache` takes 0.566× the instructions, `simplify`
0.964×, `decode_vertex_buffer` 0.934× (scalar) and 0.849× (default), and no case
is more than 1% higher. The 0.2.16 release tables came from a separate
campaign and are not comparable with this table: `remap_index_buffer` is
unchanged in 0.2.17 and has a ratio of 1.437 in the 0.2.16 table and 1.354 here.

In separate wall-time measurements on three large meshes, four workers took
2.96–4.34× less time than the serial function for `parallel::encode_vertex_buffer_parallel`
and 2.63–2.90× less for `parallel::build_meshlets_spatial_parallel`.
`parallel::decode_vertex_buffer_parallel` took 1.15–1.78× less with four or more
workers on outputs of at least 48 MiB, where it runs concurrently. Parallel
cluster LOD had a 1.605–1.780× speedup with four workers versus one.
See [docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/docs/PERFORMANCE.md)
for real-mesh and corpus tables for both builds, parallel wall times,
costs of the new APIs, method and host.
Planned work is in [docs/ROADMAP.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/docs/ROADMAP.md).

## Known exceptions

- A malformed meshlet tail is accepted by scalar upstream and by this crate, but rejected by upstream SSE.
- AVX2 and 32-bit ARM kernels are not implemented. AVX-512 is used for vertex decoding only; index and sequence decoding is scalar.
- The instruction counts above describe the listed x86-64 workloads; they do not establish performance on other architectures or per-case wall-time limits. Aarch64 timing was not measured for 0.2.17.

## Licence and acknowledgements

MIT; see [LICENSE](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.17/LICENSE). meshoptimizer
is written by Arseny Kapoulkine. This independent port keeps upstream's licence notice and credits
its algorithms; it is not affiliated with or endorsed by the upstream project. UPSTREAM.md records
provenance and intentional API differences.
