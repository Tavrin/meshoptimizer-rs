# meshoptimizer-rs

[![CI](https://img.shields.io/github/actions/workflow/status/Tavrin/meshoptimizer-rs/ci.yml?branch=main&label=CI)](https://github.com/Tavrin/meshoptimizer-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/meshoptimizer-rs.svg)](https://crates.io/crates/meshoptimizer-rs)
[![docs.rs](https://img.shields.io/docsrs/meshoptimizer-rs)](https://docs.rs/meshoptimizer-rs)

A pure-Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3. Its output is
byte-identical to the C++ library: index order, error values and encoded bytes match. It needs no
C++ toolchain and supports `no_std` with `alloc`.

```toml
[dependencies]
meshoptimizer-rs = "0.2.5"
```

The minimum Rust version is 1.88. The library name is `meshoptimizer_rs`.

## Usage

Coming from the `meshopt` crate? See [docs/MIGRATING_FROM_MESHOPT.md](https://github.com/Tavrin/meshoptimizer-rs/blob/main/docs/MIGRATING_FROM_MESHOPT.md).

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

## Features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | `std::error::Error` for `Error`. |
| `simd` | yes | SIMD vertex, filter and meshlet decoding for SSE2/SSSE3/SSE4.1, NEON and wasm simd128. |
| `parallel` | no | Rayon batch APIs in the `parallel` module; enables `std`. |
| `clusterlod` | no | The `clusterlod` module, a port of upstream `demo/clusterlod.h`. |
| `experimental` | no | Upstream experimental functions, including normal generation and remeshing. |

For `no_std`, set `default-features = false`; an allocator is required. Without `simd` the crate
contains no `unsafe` code. Cargo feature unification applies: if another dependency enables
`simd`, it is on for your build too.

## Correctness and performance

Default output is byte-identical to meshoptimizer 1.3 at commit
`4c203430ca565cb59a468a91922c76c208169536`, whose `src` tree equals tag v1.3. Fixtures,
differential sweeps, fuzzing and WASM runs check this on recorded inputs; they don't prove it for
every input. SIMD decoding matches the scalar bytes. See
[docs/VERIFICATION.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/VERIFICATION.md)
and [UPSTREAM.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/UPSTREAM.md). Timings against C++ 1.3 and the meshopt crate are in
[docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/PERFORMANCE.md),
with links to the parity records. Development phases and planned work are in
[docs/ROADMAP.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/ROADMAP.md).

## Known exceptions

Intervals and method are in
[docs/PERFORMANCE.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/docs/PERFORMANCE.md#known-exceptions-in-detail).

- Overdraw optimization is about 1.47–1.60× C++ 1.3 (scalar) on the largest real meshes measured.
- Allocating vertex decode, `vertex-v1-streaming-s4`: 1.529–2.118× in 0.2.0; 1.000× and 1.058× when
  re-run for 0.2.2. The caller-buffer form, `decode_vertex_buffer_into`, is about 1.04×.
- Allocating index decode maxima: `index-2-v0-streaming-s4` 1.508–3.625×,
  `index-2-v1-streaming-s4` 2.189–2.465×.
- Allocating EXT view maxima: `view-1-streaming-s4` 1.343–1.636×; `varied-view-3-streaming-s32`
  1.619–2.590× (unresolved); the filtered-view maximum is near 1.64×.
- `partition_clusters` with Cargo defaults: mean 1.450×, maximum 1.778×.
- Older records keep a sparse allocating vertex-fetch maximum failure and inconclusive overdraw and
  sloppy-simplification maxima.
- A malformed meshlet tail is accepted by scalar upstream and by this crate, but rejected by
  upstream SSE.
- AVX2, AVX-512 and 32-bit ARM SIMD are not implemented; index and sequence decoding is scalar.

## Licence and acknowledgements

MIT; see [LICENSE](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.8/LICENSE). meshoptimizer
is written by Arseny Kapoulkine. This independent port keeps upstream's licence notice and credits
its algorithms; it is not affiliated with or endorsed by the upstream project. UPSTREAM.md records
provenance and intentional API differences.
