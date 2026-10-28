# Using meshoptimizer-rs

These recipes use triangle lists of `u32` indices and borrowed vertex views.
`Positions::from_packed` borrows XYZ records (`&[[f32; 3]]`). For interleaved
floats, strides and offsets count `f32` components; for byte views they count
bytes and the byte order is explicit. Indices refer to the full position array.

## Tuning for a specific GPU cache

`VertexScoreTable` and `optimize_vertex_cache_with_table` are opt-in extensions
not in C++ meshoptimizer. Start from `VertexScoreTable::default()`, copy its
`cache_scores()` and `live_scores()`, adjust the scores, and validate them with
`VertexScoreTable::new(&cache, &live)?`. Pass the table before the workspace to
`optimize_vertex_cache_with_table`, or use `optimize_vertex_cache_with_table_into`
with a caller-owned destination. The constructor requires exactly 17 finite
cache scores and 9 finite live-valence scores; it does not change cache capacity.

Cache entry zero scores vertices outside the cache, entries 1–16 score cache
positions, and live entry eight also scores all higher valences. Only triangles
whose summed score is positive are picked by score; otherwise the next unemitted
triangle in input order follows, so keep the scores you want to favour positive.
Tuned tables
must be validated on the target hardware using representative meshes; a changed
ordering does not establish a GPU performance gain. The existing standard and
strip APIs retain their upstream presets.

## Non-packed positions

```text
pub fn from_interleaved(
    data: &[f32],
    count: usize,
    stride: usize,
    offset: usize,
) -> Result<Positions<'_>, Error>

pub fn from_bytes(
    data: &[u8],
    count: usize,
    stride: usize,
    offset: usize,
    order: ByteOrder,
) -> Result<Positions<'_>, Error>
```

Both are `Positions` constructors. For `from_interleaved`, `stride` (3 to 64) and
`offset` count `f32` components. For `from_bytes`, they count bytes: `stride` is a
multiple of four from 12 to 256, and `order` is `ByteOrder::LittleEndian` or
`ByteOrder::BigEndian`. `count` is the number of vertices. Both return
`InvalidLayout` if the layout or buffer length is invalid. The views borrow
`data` and copy nothing.

```rust
use meshoptimizer_rs::{
    simplify, ByteOrder, Positions, SimplifyOptions, SimplifySettings, Workspace,
};

let indices = [0, 1, 2, 2, 1, 3];
let settings = SimplifySettings {
    target_index_count: 3,
    target_error: 0.01,
    options: SimplifyOptions::EMPTY,
};
let mut workspace = Workspace::default();

// x, y, z, u, v per vertex: stride 5 floats, XYZ at offset 0.
let floats = [
    0., 0., 0., 0., 0.,
    1., 0., 0., 1., 0.,
    0., 1., 0., 0., 1.,
    1., 1., 0., 1., 1.,
];
let view = Positions::from_interleaved(&floats, 4, 5, 0)?;
simplify(&indices, view, settings, &mut workspace)?;

// The same vertices as big-endian bytes: stride 20 bytes, XYZ at byte offset 0.
let bytes: Vec<u8> = floats.iter().flat_map(|v| v.to_be_bytes()).collect();
let view = Positions::from_bytes(&bytes, 4, 20, 0, ByteOrder::BigEndian)?;
simplify(&indices, view, settings, &mut workspace)?;
# Ok::<(), meshoptimizer_rs::Error>(())
```

## Simplify one mesh

```text
pub fn simplify(
    indices: &[u32],
    positions: Positions<'_>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error>
```

Returns `SimplifiedMesh { indices, error }`. Indices still refer to the original
vertices; there is no implicit welding, fetch compaction or cache optimization.
The target counts indices, not triangles, and is a goal rather than a guarantee.
By default `error` is relative to the mesh extent; `ERROR_ABSOLUTE` selects position units.
`target_error` must be finite and nonnegative; all supplied positions must be finite.

```rust
use meshoptimizer_rs::{simplify, Positions, SimplifyOptions, SimplifySettings, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
let indices = [0, 1, 2, 2, 1, 3];
let settings = SimplifySettings {
    target_index_count: 3,
    target_error: 0.01,
    options: SimplifyOptions::EMPTY,
};
let mut workspace = Workspace::default();
let lod = simplify(&indices, Positions::from_packed(&positions), settings, &mut workspace)?;
assert!(lod.indices.len() <= indices.len());
# Ok::<(), meshoptimizer_rs::Error>(())
```

Errors: `InvalidTopology` for incomplete triangles, `IndexOutOfBounds` for invalid
references, `InvalidParameter` for invalid settings (including
`target_index_count > indices.len()`) or non-finite positions,
`NumericalFailure` for non-finite calculations, and the resource errors below.
Use `SimplifyOptions::from_bits` to validate raw options (`UnknownFlags` on rejection).

## Per-mesh LOD chain

```text
pub fn simplify_scale(positions: Positions<'_>) -> Result<f32, Error>

pub fn simplify(
    indices: &[u32],
    positions: Positions<'_>,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error>

pub fn optimize_vertex_cache(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error>

pub fn optimize_overdraw(
    indices: &[u32],
    positions: Positions<'_>,
    threshold: f32,
    workspace: &mut Workspace,
) -> Result<Vec<u32>, Error>
```

Keep the full vertex array for every level. Each target below simplifies the
previous level's topology, then orders it for the vertex cache and overdraw.
`scale` is the largest axis extent; multiply a relative error by it to get
position units. Each level's error is relative to that level's input, not an
accumulated bound against the original mesh. The optimizers return reordered
`Vec<u32>` triangle lists. Reuse one workspace for the whole chain: it keeps
scratch capacity between calls, so later calls skip reallocating.

```rust
use meshoptimizer_rs::{
    optimize_overdraw, optimize_vertex_cache, simplify, simplify_scale, Error,
    Positions, SimplifyOptions, SimplifySettings, Workspace,
};

fn main() -> Result<(), Error> {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let indices = [0, 1, 2, 2, 1, 3];
    let view = Positions::from_packed(&positions);
    let mut workspace = Workspace::default();
    let scale = simplify_scale(view)?;
    let mut current = indices.to_vec();
    for target_index_count in [6, 3, 0] {
        let settings = SimplifySettings {
            target_index_count,
            target_error: 0.01,
            options: SimplifyOptions::EMPTY,
        };
        let lod = simplify(&current, view, settings, &mut workspace)?;
        let cached = optimize_vertex_cache(&lod.indices, positions.len(), &mut workspace)?;
        current = optimize_overdraw(&cached, view, 1.05, &mut workspace)?;
        println!("{} indices, absolute error {}", current.len(), lod.error * scale);
    }
    Ok(())
}
```

Errors: simplification's errors above; `simplify_scale` returns `InvalidParameter`
for non-finite positions and `NumericalFailure` for an overflowing extent.
Cache ordering checks topology and indices; overdraw also rejects a non-finite
threshold with `InvalidParameter` and non-finite geometry/calculations with
`NumericalFailure`. Both optimizers can return the resource errors below.

## Meshlets with bounds

```text
pub fn build_meshlets(
    indices: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    cone_weight: f32,
    workspace: &mut Workspace,
) -> Result<Meshlets, Error>
```

```text
pub fn new(positions: Positions<'a>) -> Self // BoundsPositions<'a>
```

```text
pub fn compute_meshlet_bounds_with_positions(
    vertices: &[u32],
    triangles: &[u8],
    p: BoundsPositions<'_>,
    workspace: &mut Workspace,
) -> Result<Bounds, Error>
```

`build_meshlets` returns `Meshlets`: descriptors, original vertex references and
three local `u8` indices per triangle. Descriptor triangle offsets are byte
offsets, with no padding between meshlets. Bounds return a sphere and normal cone
in `Bounds`. Cache the full position validation once with `BoundsPositions` when
querying many meshlets; failures are stored and reported by the bounds operation.

`MeshletSettings::default()` is `max_vertices: 64`, `max_triangles: 124`. Valid
ranges are 3 to 256 vertices and 1 to 512 triangles; anything else returns
`InvalidParameter`. `cone_weight` (finite, 0.0 to 1.0) makes the builder prefer
triangles that face the same way, which helps cone culling; 0.0 ignores facing.
The example uses 0.25.

```rust
use meshoptimizer_rs::{
    build_meshlets, compute_meshlet_bounds_with_positions, BoundsPositions, Error,
    MeshletSettings, Positions, Workspace,
};

fn main() -> Result<(), Error> {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let indices = [0, 1, 2, 2, 1, 3];
    let view = Positions::from_packed(&positions);
    let mut workspace = Workspace::default();
    let built = build_meshlets(&indices, view, MeshletSettings::default(), 0.25, &mut workspace)?;
    let positions = BoundsPositions::new(view);
    for m in &built.meshlets {
        let v = m.vertex_offset as usize;
        let t = m.triangle_offset as usize;
        let bounds = compute_meshlet_bounds_with_positions(
            &built.vertices[v..v + m.vertex_count as usize],
            &built.triangles[t..t + m.triangle_count as usize * 3],
            positions,
            &mut workspace,
        )?;
        println!("center {:?}, radius {}", bounds.center, bounds.radius);
    }
    Ok(())
}
```

Errors: `InvalidTopology`, `IndexOutOfBounds`, `InvalidParameter` for unsupported
meshlet sizes or builder settings, `NumericalFailure` for non-finite geometry or
calculations, and the resource errors below. Bounds also validate meshlet-local
indices and supported vertex/triangle counts. `BoundsPositions::new` defers errors.

## Vertex and index encode and decode

```text
pub fn encode_vertex_buffer(
    vertices: &[u8],
    count: usize,
    stride: usize,
    encoding: VertexEncoding,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error>

pub fn decode_vertex_buffer(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error>

pub fn encode_index_buffer(
    indices: &[u32],
    encoding: IndexEncoding,
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error>

pub fn decode_index_buffer(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error>
```

Vertex input is `count * stride` initialized bytes; stride is a positive multiple
of four, at most 256. This recipe serializes packed XYZ as little-endian floats.
Index encoding takes a `u32` triangle list. Decoding returns little-endian bytes:
index stride 2 gives `u16` bytes and stride 4 gives `u32` bytes. At stride 2,
indices above 65535 wrap (upstream's truncating `u16` conversion); use stride 4
for such meshes. Counts and strides
come from your container format; encoded streams do not supply them for you.
Each allocating codec returns a `Vec<u8>`. Triangle encoding may rotate corners
within each triangle, preserving winding; decoded index order reflects those rotations.

```rust
use meshoptimizer_rs::codec::{
    decode_index_buffer, decode_vertex_buffer, encode_index_buffer, encode_vertex_buffer,
    IndexEncoding, VertexEncoding,
};
use meshoptimizer_rs::Workspace;

let positions: [[f32; 3]; 3] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
let indices = [0u32, 1, 2];
let vertices: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
let mut workspace = Workspace::default();
let encoded = encode_vertex_buffer(&vertices, 3, 12, VertexEncoding::DEFAULT, &mut workspace)?;
assert_eq!(decode_vertex_buffer(3, 12, &encoded, &mut workspace)?, vertices);
let encoded = encode_index_buffer(&indices, IndexEncoding::DEFAULT, &mut workspace)?;
let decoded = decode_index_buffer(3, 4, &encoded, &mut workspace)?;
let expected: Vec<u8> = indices.iter().flat_map(|v| v.to_le_bytes()).collect();
assert_eq!(decoded, expected);
# Ok::<(), meshoptimizer_rs::Error>(())
```

Errors: `InvalidLayout` for invalid vertex/index strides or vertex byte lengths,
`InvalidTopology` for incomplete triangles, `InvalidStream` for malformed data,
`UnsupportedVersion` for unsupported codec versions, and the resource errors below.
Encoding constructors validate versions and levels (`InvalidParameter` for an
unsupported vertex level), so an invalid encoding configuration cannot be built.

## Untrusted input with Limits

```text
pub fn new(limits: Limits) -> Self // Workspace
```

```text
pub fn decode_vertex_buffer(
    count: usize,
    stride: usize,
    source: &[u8],
    workspace: &mut Workspace,
) -> Result<Vec<u8>, Error>
```

The default `Limits` allow 1 GiB of crate-owned output plus retained scratch
capacity and 2^34 counted record visits per call. Caller buffers, allocator overhead,
stack storage and the application's retained outputs are outside this accounting.
Choose smaller limits for the request, and check transport sizes and decoded size
before allocating caller buffers. There is no wall-clock timeout in `Limits`.

```rust
use meshoptimizer_rs::{codec::decode_vertex_buffer, Error, Limits, Workspace};

fn main() -> Result<(), Error> {
    // Check transport sizes before buffering a request or creating its views.
    let source = [0u8; 4]; // Deliberately malformed stream supplied by a client.
    let count = 3usize;
    let stride = 12usize;
    let limits = Limits { max_bytes: 64 << 20, max_work: 1 << 20 };
    if source.len() > limits.max_bytes || count > limits.max_bytes / stride {
        return Err(Error::LimitExceeded);
    }
    let mut workspace = Workspace::new(limits);
    match decode_vertex_buffer(count, stride, &source, &mut workspace) {
        Ok(bytes) => println!("{} decoded bytes", bytes.len()),
        Err(Error::InvalidStream) => println!("rejected malformed input"),
        Err(error) => return Err(error),
    }
    Ok(())
}
```

Returns decoded bytes or an error; this example treats `InvalidStream` as a
malformed client stream to reject, not a failure. Decoding can also return `InvalidLayout`, `UnsupportedVersion`,
`SizeOverflow`, `LimitExceeded` or `AllocationFailed`. `Workspace::new` is infallible.
`BoundsPositions::new` and `simplify_scale` have no workspace budget: bound input
sizes before calling them on untrusted data.

## Parallel workers

Each worker calls `simplify` (signature above) on a shared immutable XYZ array
and triangle list, and owns its workspace and returned mesh. The following uses standard threads and needs no `parallel` feature.

```rust
use meshoptimizer_rs::{simplify, Error, Positions, SimplifyOptions, SimplifySettings, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
let indices = [0, 1, 2, 2, 1, 3];
let results = std::thread::scope(|scope| {
    let mut workers = Vec::new();
    for target_index_count in [6, 3] {
        let (positions, indices) = (&positions, &indices);
        workers.push(scope.spawn(move || {
            let mut workspace = Workspace::default();
            let settings = SimplifySettings {
                target_index_count, target_error: 0.01, options: SimplifyOptions::EMPTY,
            };
            // Reuse this workspace for further jobs handled by this worker.
            simplify(indices, Positions::from_packed(positions), settings, &mut workspace)
        }));
    }
    workers.into_iter().map(|worker| worker.join().expect("worker panicked"))
        .collect::<Result<Vec<_>, Error>>()
})?;
assert_eq!(results.len(), 2);
# Ok::<(), Error>(())
```

Returns one `SimplifiedMesh` per worker. With the `parallel` feature,
[`simplify_lod_chains_batch`](https://docs.rs/meshoptimizer-rs/latest/meshoptimizer_rs/parallel/fn.simplify_lod_chains_batch.html)
provides ordered per-mesh chains using Rayon. It panics if Rayon cannot start or
a worker panics. Its outer error is result-slot
`SizeOverflow` or `AllocationFailed`; item errors stay in input order and do not
cancel other items. Limits apply separately to each item, with no aggregate cap.

## Lower peak memory with Retention::Lean

```text
pub fn set_retention(&mut self, retention: Retention) // Workspace
```

Use the same position and index layouts as simplification. `set_retention` returns
`()` and cannot fail. With `Retention::Lean`, the workspace keeps less scratch
between calls, so peak memory is lower. Later calls have to reallocate that
scratch, so they are slower. It applies to simplify, vertex-cache and overdraw
calls; other operations keep their scratch.

```rust
use meshoptimizer_rs::{simplify, Positions, Retention, SimplifyOptions, SimplifySettings, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
let indices = [0, 1, 2, 2, 1, 3];
let mut workspace = Workspace::default();
workspace.set_retention(Retention::Lean);
let lod = simplify(&indices, Positions::from_packed(&positions), SimplifySettings {
    target_index_count: 3, target_error: 0.01, options: SimplifyOptions::EMPTY,
}, &mut workspace)?;
assert!(lod.indices.len() <= indices.len());
# Ok::<(), meshoptimizer_rs::Error>(())
```

Returns the same simplification result and errors as the first recipe. Lean is
opt-in: it lowers peak memory, but later calls reallocate scratch and run slower.
See the measured [memory and time results](https://github.com/Tavrin/meshoptimizer-rs/blob/main/docs/PERFORMANCE.md).

## Mesh repair

`sanitize` is an extension; it is not in C++ meshoptimizer.

```text
pub fn sanitize(
    indices: &[u32],
    positions: Positions<'_>,
    options: SanitizeOptions,
    workspace: &mut Workspace,
) -> Result<SanitizedMesh, Error>
```

Use it on uploaded or generated meshes before simplification or meshlet
building. It only removes triangles: out-of-range indices, non-finite positions,
repeated indices and later copies of the same face. Kept triangles stay in input
order with their original indices and winding, so vertex streams need no change.
`SanitizeOptions::COINCIDENT_CORNERS` also drops triangles with two corners at
bit-identical positions (collinear triangles are kept; no area is computed),
`WELD_POSITIONS` compares duplicate faces by position instead of by
index, `IGNORE_ORIENTATION` also drops reverse-wound copies, and `REJECT_INVALID`
returns an error instead of dropping invalid triangles.

```rust
use meshoptimizer_rs::{sanitize, Positions, SanitizeOptions, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 1., 0.]];
// A face, its rotated copy, its reverse-wound copy, and a face whose
// corners 2 and 3 share a position.
let indices = [0, 1, 2, 1, 2, 0, 0, 2, 1, 0, 2, 3];
let mut workspace = Workspace::default();
let options = SanitizeOptions::COINCIDENT_CORNERS | SanitizeOptions::IGNORE_ORIENTATION;
let mesh = sanitize(&indices, Positions::from_packed(&positions), options, &mut workspace)?;
assert_eq!(mesh.indices, [0, 1, 2]);
let report = mesh.report;
assert_eq!((report.duplicate, report.reversed_duplicate, report.coincident_corners), (1, 1, 1));
# Ok::<(), meshoptimizer_rs::Error>(())
```

Returns the kept indices and a `SanitizeReport` with one count per removal
reason. `sanitize_into` writes to a caller buffer of at least
`sanitize_bound(indices.len())` indices and leaves it unchanged on any error.
Sanitation does not weld vertices, fill holes or fix orientation; the result is
not guaranteed to be manifold or closed.

## Resource errors

Workspace operations can return `SizeOverflow` when size arithmetic or platform
allocation bounds overflow, `LimitExceeded` when the per-call bytes or counted work
exceed `Limits`, and `AllocationFailed` when fallible reservation fails.
Caller-buffer `_into` forms also return `BufferTooSmall` for insufficient capacity;
a late error may leave a used prefix modified. The `_into` functions are
`simplify_into`, `optimize_vertex_cache_into`, `optimize_overdraw_into` and
`build_meshlets_into`, among others.

## Gotchas

- **Indices:** `u32` only. Pass the full position count as `vertex_count`, not the number of vertices a LOD uses. An index at or beyond that count returns `IndexOutOfBounds`.
- **Simplify target:** `target_index_count` larger than `indices.len()` returns `InvalidParameter`.
- **Index decode at stride 2:** indices above 65535 wrap (upstream's truncating `u16` conversion). Use stride 4 for meshes with more than 65536 vertices.
- **Position layout:** use `Positions::from_packed(&[[f32; 3]])` when you can; it is the fastest layout. Other layouts work but read more slowly.
- **Workspace reuse:** use one `Workspace` per worker thread and reuse it across calls; see [Per-mesh LOD chain](#per-mesh-lod-chain).
- **Retention::Lean:** lowers peak memory at the cost of time; the default is `Retention::Reuse`.
- **Default Limits:** 1 GiB of crate-owned storage and 2^34 counted visits per call; set smaller `max_bytes` and `max_work` for untrusted input and check caller allocations separately.
- **Byte-identical output:** results match C++ meshoptimizer 1.3 (scalar build); recorded differential inputs and exceptions are described in the README.
- **Semver policy:** 0.2.x patch releases are drop-in, with no API break and the same output.
