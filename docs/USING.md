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

```rust
use meshoptimizer_rs::{
    optimize_vertex_cache, optimize_vertex_cache_with_table, VertexScoreTable, Workspace,
};

let indices = [0, 1, 2, 2, 1, 3];
let mut workspace = Workspace::default();
let preset = VertexScoreTable::default();
let mut cache = *preset.cache_scores();
let live = *preset.live_scores();
let table = VertexScoreTable::new(&cache, &live)?;
assert_eq!(
    optimize_vertex_cache_with_table(&indices, 4, &table, &mut workspace)?,
    optimize_vertex_cache(&indices, 4, &mut workspace)?,
);
cache[1] *= 1.1;
let tuned = VertexScoreTable::new(&cache, &live)?;
let reordered = optimize_vertex_cache_with_table(&indices, 4, &tuned, &mut workspace)?;
assert_eq!(reordered.len(), indices.len());
# Ok::<(), meshoptimizer_rs::Error>(())
```

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

## Typed Rust vertex fields

`VertexLayout` is an opt-in extension not in C++ meshoptimizer. Use `offset_of!`
and a typed selector to check a field against the struct's size and alignment.
See the [`VertexLayout`](crate::VertexLayout) doctest for a complete `#[repr(C)]`
vertex with position, normal and UV fields.

```rust
use core::mem::offset_of;
use meshoptimizer_rs::{VertexFieldSnapshot, VertexLayout, VertexLayoutError};

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}
let vertices = [Vertex {
    position: [0., 0., 0.], normal: [0., 0., 1.], uv: [0., 0.],
}];
let layout = VertexLayout::new(offset_of!(Vertex, position), |v: &Vertex| &v.position)?;
let data = layout.copy_from(&vertices)?;
let positions = data.positions()?;
assert_eq!(positions.get(0), Some([0., 0., 0.]));
let normals = VertexFieldSnapshot::from_vertices(
    &vertices, offset_of!(Vertex, normal), |v| &v.normal,
)?;
assert_eq!(normals.attributes()?.components(), 3);
# Ok::<(), VertexLayoutError>(())
```

Keep `data` alive while using its views. Construction copies selected floats into
a zero-filled, little-endian snapshot with the original stride and offset; it
never reads struct padding. The allocation is one full struct stride per vertex,
outside `Workspace` limits, so bound caller input sizes first. Later vertex edits
require a new snapshot. Selectors may be capturing closures and enforce float
element types at compile time;
layout, field-width and address failures return `VertexLayoutError`. Empty input
checks the layout but has no field address to verify. Packed position slices can
continue to use `Positions::from_packed` without a copy.

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

## Streaming index sequences

`codec::IndexSequenceDecoder` is an opt-in extension, not in C++ meshoptimizer.
It accepts ordinary version 0/1 index-sequence bytes as they arrive, without
heap allocation or a Cargo feature. The container supplies the index count.
Keep the decoder across transport chunks and resubmit the unconsumed input
suffix when output space runs out. Zero output space is valid and yields
`NeedOutput` while indices remain; use at least one slot to make progress.

```rust
use meshoptimizer_rs::{codec::{IndexSequenceDecoder, Status}, Limits};

let compressed = [0xd1, 0, 4, 0xcd, 1, 4, 7, 0x98, 0x1f, 0, 0, 0, 0];
let mut decoder = IndexSequenceDecoder::new(6, Limits::default())?;
let mut provisional = Vec::new();
for chunk in compressed.chunks(2) { // replace with your transport's chunks
    let mut input = chunk;
    loop {
        let mut output = [0u32; 1];
        let progress = decoder.push(input, &mut output)?;
        provisional.extend_from_slice(&output[..progress.written]);
        input = &input[progress.consumed..];
        // `Status` is non-exhaustive; anything but NeedOutput ends this chunk.
        if progress.status != Status::NeedOutput {
            assert!(input.is_empty());
            break;
        }
    }
}
decoder.finish()?; // transport EOF: only now commit provisional indices
assert_eq!(provisional, [0, 1, 51, 2, 49, 1000]);
# Ok::<(), meshoptimizer_rs::Error>(())
```

All output is provisional until `finish()` succeeds. Submit every byte of the
stream, including any bytes arriving after `Done`: these are rejected as extra
input. `finish()` rejects incomplete headers, varints, counts, and padding.
Padding is exactly four opaque bytes, as in the one-shot decoder. Noncanonical
varints retain upstream's five-byte/truncated-high-bit semantics. Header errors
are reported immediately, even before enough bytes exist for the one-shot
minimum-length check. Errors are terminal and repeat on subsequent calls; an
error may follow provisional writes in the same call without a progress receipt.

`new` checks decoded size and charges four work units per declared index.
`push` charges one per consumed compressed byte against the remaining budget;
chunking and empty calls do not reset it. Fixed state and caller buffers are
outside `max_bytes` (zero is allowed). This enables bounded staging; the
application chooses whether to retain provisional output. Vertex and triangle
streams require trailer information and are not supported by this decoder.

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

## Parallel groups within one cluster LOD mesh

Enable `parallel` and `clusterlod` for the opt-in `clusterlod::build_parallel`
(or `build_parallel_with_output`). This extension is not in C++ meshoptimizer.
Choose an existing Rayon pool and explicitly opt out of finite logical limits:

```rust
# #[cfg(all(feature = "parallel", feature = "clusterlod"))]
# {
use meshoptimizer_rs::{clusterlod, Limits, Workspace};
let mut positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
let indices = [0, 1, 2];
let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build()?;
let mut workspace = Workspace::new(Limits {
    max_bytes: usize::MAX,
    max_work: u64::MAX,
});
let config = clusterlod::default_config(128)?; // dilation disabled
let groups = pool.install(|| clusterlod::build_parallel(config, clusterlod::Mesh {
    indices: &indices,
    positions: &mut positions,
    attributes: None,
    vertex_lock: None,
    attribute_weights: &[],
    attribute_protect_mask: 0,
}, &mut workspace))?;
assert!(!groups.is_empty());
assert_eq!(groups[0].clusters[0].indices, indices);
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

Partitioning and boundary locks run serially each level; groups use the existing
serial kernels and assemble in ordinal order. Callbacks run serially on the
calling thread, including their custom IDs and failure prefixes. Dilation,
any finite work/storage limit (including `Limits::default()`), single-group
levels, and speculative worker failures use the serial algorithm. Existing
`build` and batch APIs retain their behavior.

`Usage` reports identical logical serial work and peak storage. Concurrent
worker scratch, held speculative results, and Rayon infrastructure consume
additional physical memory. Select a pool size that fits the mesh and available
memory. No parallel speedup is guaranteed.

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

### Tolerance welding, orientation and small holes

These are opt-in calls, like `sanitize`; they are extensions, not in C++
meshoptimizer.
Welding returns original representatives without moving or compacting vertices.
Choose an explicit seam policy; use `WeldPolicy::AttributeEquality` to compare
UVs, normals, materials or other caller attributes. Every merged vertex lies
within tolerance of its representative; nearby chains do not merge transitively.
Always apply the remap and sanitize faces again before proceeding.

```rust
use meshoptimizer_rs::{weld_vertices, orient_faces, fill_holes, sanitize,
    Positions, SanitizeOptions, WeldPolicy, Workspace};
let vertices = [[0.,0.,0.], [1.,0.,0.], [1.,1.,0.], [0.,1.,0.], [0.001,0.,0.]];
let uv_classes = [0, 0, 0, 0, 0];
let mut workspace = Workspace::default();
let positions = Positions::from_packed(&vertices);
let welded = weld_vertices(positions, 0.01,
    WeldPolicy::AttributeEquality(&|a,b| uv_classes[a as usize] == uv_classes[b as usize]),
    &mut workspace)?;
assert_eq!(welded.remap, [0,1,2,3,0]);
let indices: Vec<u32> = [4,1,2,0,3,2].map(|v| welded.remap[v]).to_vec();
let clean = sanitize(&indices, positions, SanitizeOptions::EMPTY, &mut workspace)?;
let oriented = orient_faces(&clean.indices, vertices.len(), &mut workspace)?;
assert_eq!(oriented.report.flipped_faces, [1]);
assert!(oriented.report.conflicts.is_empty());
let filled = fill_holes(&oriented.indices, positions, 4, &mut workspace)?;
assert_eq!(filled.report.filled.len(), 1);
assert_eq!(filled.report.output_triangles, 4);
assert!(filled.indices.iter().all(|&v| v < vertices.len() as u32));
# Ok::<(), meshoptimizer_rs::Error>(())
```

`orient_faces` preserves each component's lowest face as its winding seed;
it does not choose outward orientation. Inspect `conflicts` and
`ambiguous_components` before relying on the result. Non-manifold edges never
provide propagation links. `fill_holes` appends deterministic ear triangles for
simple, approximately planar boundary loops within the inclusive edge limit.
It reports oversized, non-simple and ambiguous components separately, uses no
new vertices, and does not test intersections with other mesh surfaces. Intended
openings can also be filled: the caller must choose whether to invoke it.

Each API has an `_into` variant and a bound function. Caller-buffer variants
stage decisions in checked heap scratch and leave the entire destination
unchanged on any error, including limits and insufficient capacity. Reports
list original face/vertex indices, so callers can inspect changes. Added faces
have no inherited material or face attributes; assign those explicitly.

## Mesh topology analysis

`analyze_mesh`, also an extension, reports what is wrong before anything is
removed. It changes nothing and returns a `MeshAnalysis`: triangle
classifications, then counts of boundary edges and components,
non-manifold edges (more than two faces) and vertices (more than one fan of
faces), orientation conflicts (exactly two incident faces using an edge in the
same direction) and
connected components. `AnalyzeOptions::with_weld_positions(true)` treats
vertices with equal positions as one, which shows whether split seams close a
surface; `with_list_limit(n)` also lists the first `n` defects of each kind.
Analyze, sanitize, then analyze the result to see what remains:

```rust
use meshoptimizer_rs::{analyze_mesh, sanitize, AnalyzeOptions, Positions, SanitizeOptions, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
// A tetrahedron plus a duplicate of its first face.
let indices = [0, 2, 1, 0, 1, 3, 1, 2, 3, 0, 3, 2, 2, 1, 0];
let mut workspace = Workspace::default();
let positions = Positions::from_packed(&positions);
let before = analyze_mesh(&indices, positions, AnalyzeOptions::EMPTY, &mut workspace)?;
assert_eq!((before.duplicate, before.non_manifold_edges), (1, 3));
let mesh = sanitize(&indices, positions, SanitizeOptions::EMPTY, &mut workspace)?;
let after = analyze_mesh(&mesh.indices, positions, AnalyzeOptions::EMPTY, &mut workspace)?;
assert_eq!((after.boundary_edges, after.non_manifold_edges, after.orientation_conflicts), (0, 0, 0));
# Ok::<(), meshoptimizer_rs::Error>(())
```

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

## One-call LOD chain

`build_lod_chain` is an extension not in C++ meshoptimizer. Targets are
relative to the original mesh; each level defaults to the previous output.

```rust
use meshoptimizer_rs::{build_lod_chain, LodChainOptions, LodTargets, LodErrors, Positions, Workspace};
let positions = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [1.,1.,0.]];
let options = LodChainOptions::new(LodTargets::Ratios(&[0.5]), LodErrors::Uniform(1.0));
let chain = build_lod_chain(&[0,1,2,2,1,3], Positions::from_packed(&positions), &options, &mut Workspace::default())?;
assert_eq!(chain.levels.len(), 1);
# Ok::<(), meshoptimizer_rs::LodChainError>(())
```

Options are built with `LodChainOptions::new` and the `with_simplify_options`,
`with_attributes`, `with_vertex_flags` and `with_independent` setters; the option
and enum types are `#[non_exhaustive]`. Vertex flags (locks) apply with or
without attributes. Each level owns a `Vec<u32>`; other outputs such as meshlets
or clusters would be separate APIs.

- **Targets:** ratios become `floor(ratio * source_triangles) * 3` indices. The
  ratios must decrease strictly, but two neighbouring ratios may round to the
  same count on a small mesh.
- **Error and scale:** `LodLevel::error` is the simplifier's own returned value
  for that level, unconverted. In successive mode it is relative to the previous
  level, not to the source and not cumulative; with attributes it includes the
  attribute terms. `LodLevel::scale` is the extent it was normalized by (the
  referenced vertices only, with `SPARSE`). Units follow `ERROR_ABSOLUTE`.
- **`stopped_at_error_limit`:** the simplifier stopped because the next collapse
  candidate exceeded the level's error limit. A looser limit is not guaranteed
  to reach the target; a missed target may also reflect topology constraints.
- **Limits and usage:** the workspace limits cover the whole call, including the
  builder's own scans, every level's work, and the chain metadata plus retained
  level outputs together with scratch. `Workspace::usage()` reports the call's
  total work and peak bytes, on success and on failure.
- **Failure:** the result is atomic. `LodChainError::level` is the zero-based
  failing level, or `None` for a failure of the call as a whole.

## LOD quality and switch distances

`measure_lod_quality` and `measure_lod_chain_quality` measure sampled surface
distances and shading-normal angles. This extension is not in C++ meshoptimizer. It does not change
`simplify` or `build_lod_chain`.

```rust
use meshoptimizer_rs::{measure_lod_quality, LodQualityMesh, LodQualityOptions,
    LodCamera, Positions, Workspace};
let source = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.]];
let level = [[0.,0.,0.1], [1.,0.,0.1], [0.,1.,0.1]];
let indices = [0, 1, 2];
let quality = measure_lod_quality(
    LodQualityMesh::new(&indices, Positions::from_packed(&source)),
    LodQualityMesh::new(&indices, Positions::from_packed(&level)),
    &LodQualityOptions::default(), &mut Workspace::default())?;
let camera = LodCamera::new(60_f64.to_radians(), 1080., 1.)?;
let depth = camera.switch_distance(quality.symmetric.max)?;
assert!(depth > 90. && depth < 100.);
# Ok::<(), meshoptimizer_rs::Error>(())
```

`measure_lod_chain_quality(source, &chain, &options, Some(camera), &mut workspace)`
returns one report per level, measured against the source using the chain's
shared vertex buffers. Empty simplified surfaces return `InvalidParameter`.
Assign `LodQualityMesh::normals` on both surfaces to compare interpolated shading
normals in radians. Normals must be finite and nonzero; cancelling interpolated
normals return `NumericalFailure`.

Every triangle contributes its three corners plus `ceil(area * density)`
barycentric samples. Shared corners repeat. The deterministic sequence and BVH
tie rule are specified on `measure_lod_quality`. RMS/mean weight samples equally.
Maxima are sampled estimates and can miss continuous surface extrema, including
with high density. Increasing density is useful for convergence checks.

Switch distances use a pinhole camera and the sampled symmetric maximum:
`error * height / (2 * tan(vertical_fov/2) * pixel_tolerance)`. The result is the
minimum camera-space depth for a transverse displacement of that magnitude.
For camera-to-mesh-center distances, add a conservative bounding radius. This
model excludes texture error, occlusion and rasterization; it is not a certified
screen-space bound. Chain thresholds may be nonmonotone. Limits bound the whole
measurement call, including all chain reports, validation, BVH and query visits.

## Hidden-surface removal

This extension is not in C++ meshoptimizer and needs no Cargo feature.
Use it on opaque, static imported geometry before simplification. The default
rasterizes 26 orthographic views into an 8x8 tiled item/depth buffer using
fixed-point edges. `pixel_size` is the world-space pixel width; zero selects
longest referenced mesh side / 128. The square view spans twice that side;
resolution is ceil(2*side / pixel_size), from 1 to 2048. Finer requests return
`InvalidParameter`. Fully covered pixels certify occlusion with conservative
depth bounds; uncertain pixels retain geometry. Every triangle visible over at
least a pixel-area from a tested direction is retained, including partially
visible triangles. The outward coverage rule additionally retains smaller
slivers and preserves the existing lattice-sample guarantee. Coarse meshes and
small openings can leave many uncertain triangles, which are retained. A final
BVH sample check only for raster-hidden candidates preserves the original
floating-point sample behavior, including shared-edge rounding misses; it only
restores faces. It can be expensive on meshes with many hidden candidates.
Finite cameras use BVH rays from a barycentric lattice (denominator 2: six
boundary points plus centroid); every visible tested sample keeps the triangle.
Use more subdivisions for finer finite-camera sampling. Faces are two-sided, and
coplanar contacts do not occlude each other. A support surface visible from below
remains present with outside views. Restrict views to reachable camera positions
when that is the intended visibility domain.

```rust
use meshoptimizer_rs::{remove_hidden_triangles, remove_hidden_triangles_into,
    HiddenOptions, HiddenViews, Positions, Workspace};
let positions = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.]];
let indices = [0,1,2, 1,2,0];
let cameras = [[0.,0.,5.], [2.,2.,3.]];
let mut options = HiddenOptions::default();
options.views = HiddenViews::Points(&cameras);
options.subdivisions = 4;
options.tolerance = 1e-6; // world units; near contacts retain more geometry
let mut workspace = Workspace::default();
let mesh = remove_hidden_triangles(&indices, Positions::from_packed(&positions),
    options, &mut workspace)?;
assert_eq!(mesh.report.duplicate, 1);
let mut output = [u32::MAX; 9];
let report = remove_hidden_triangles_into(&mut output, &indices,
    Positions::from_packed(&positions), options, &mut workspace)?;
assert_eq!(&output[..report.index_count()], mesh.indices);
assert_eq!(&output[report.index_count()..], &[u32::MAX; 6]);
# Ok::<(), meshoptimizer_rs::Error>(())
```

`HiddenViews::Bounds { min, max }` samples eight box corners and its center,
not every point in the volume. Reports distinguish exact same-winding duplicates,
occluded faces with exact reverse-wound counterparts, and other occluded faces.
Back-to-back walls that can be seen are retained. Indices and winding remain in
input order. The caller-buffer form leaves all storage untouched on error.
Limits meter sorting, projections, raster rectangles and buffer initialization;
finite cameras also meter BVH construction, node visits and intersections, which
can still approach quadratic work. Outside views execute serially with identical
output regardless of the `parallel` feature or thread-pool size. Occlusion is measured against
the original mesh; removing its sampled-hidden surfaces does not guarantee
identical continuous visibility of the filtered mesh.

## 16-bit vertex indices

Existing names retain their exact u32 signatures. Add `_generic` to any
index-processing function or width-aware method name to select an index width.
These additions accept a sealed `Index` element type: `u16` or
`u32`. The width-aware input and result structs have a `Generic` suffix,
for example `MeshletsGeneric<u16>` and `LodQualityMeshGeneric`. Original
struct names remain non-generic structs with their original fields and traits, preserving standalone literal inference.
Conversions between legacy structs and their `Generic<u32>` counterparts move owned index buffers.
`parallel::EncodeInputGeneric` is the width-aware descriptor enum; the original
`EncodeInput` enum and its variant imports remain available. Remap tables, face identifiers, offsets and packed
local triangle words retain their documented metadata types.

```rust
use meshoptimizer_rs::{optimize_vertex_cache_generic, Error, Workspace};
let indices = [0u16, 1, 2];
let ordered: Vec<u16> = optimize_vertex_cache_generic(&indices, 3, &mut Workspace::default())?;
assert_eq!(ordered, indices);
# Ok::<(), Error>(())
```

For output-only operations, select the element type with a return annotation
or a turbofish, for example `simplify_points_generic::<u16>`. Annotate otherwise
unconstrained integer literals (`0u32` or `0u16`). The u32 adapter borrows
buffers directly and moves owned output without conversion. The u16 adapter
uses fallible temporary u32 storage charged to workspace limits. An output
index above 65,535 returns `Error::IndexOverflow`; it is never truncated.
Caller-buffer tails are preserved. A failed narrowing conversion leaves its
index destination unchanged; mutable vertex data follows each operation's
existing partial-mutation contract. Byte-oriented index decoders retain their
explicit element-width argument and check two-byte output for overflow.
## Character LODs

`simplify_character` and `build_character_lod_chain` are opt-in extensions
not in C++ meshoptimizer. Existing simplification entry points are unchanged.
Pass `CharacterInputs` with an optional `CharacterSkin`, borrowed blend-shape
delta arrays, and optional joint sample matrices. Four or eight vertex-major
influences are supported. Weights must be nonnegative and sum to one within
0.001; duplicate bone IDs are summed, including importance. Each importance
and global metric weight must be finite and nonnegative.

Before simplifying an imported character, merge its triangle primitives in a
common model space, transforming positions, normals and morph deltas
consistently. Convert joints and inverse bind matrices to the same space. Weld
exact coincident vertices with `generate_vertex_remap_multi`, then apply the
remap to indices and all vertex streams. Include normals, tangents, UVs,
colors, material boundaries, skin indices and weights, and every morph
attribute in the equality keys. A rigid morph part mixed with skinned parts
needs one full influence on its actual node transform, rather than zero skin
weights; preserve that node's real animation when evaluating poses. Position-
only welding can merge different deformations or shading seams. Unwelded
triangle soup can prevent reduction; attribute-preserving welding can still
leave genuine seams.

Start with unit bone importance. Where authored bone names identify the head,
hands or fingers, try a higher value (the evaluation uses four), and measure it
over the asset's real clips and full morph targets. Compare at the triangles
actually reached: increasing importance can trade bind-pose accuracy or reduction
for deformation detail, and does not guarantee a lower worst-pose deviation.
For rigs with opaque names, retain unit importance rather than guessing bone IDs.

The metric uses one channel per bone with value
`weight * bone_importance * skin_weight`, plus three channels per blend target
containing `(rest_position + delta) / rest_extent * blend_shape_weight`.
The rest geometry contributes the usual position quadrics. Constant-position
meshes use extent one. Bone identity is exact: integer IDs never become scalar
attributes. Character calls retain all `bone_count + 3 * blend_count` channels,
including rigs and morph sets above 32 channels, bounded by `Workspace` memory
and work limits. There is no truncation or hashing. Prepared attributes and wide
metric weights are charged to those limits. The general `Attributes` constructors
retain their C++-compatible 32-component bound; the wider packed view is private
to character preparation. Existing character calls at 32 channels or fewer retain
their previous arithmetic and resource accounting.

The existing attribute quadrics sum residual energies across the rest and
target positions. This penalizes changes in bone influence and in any full-
weight blend target. The returned simplifier error includes the attribute
terms; it is not a maximum geometric error across poses. A high importance
increases a bone channel's squared contribution. It is a soft preference, not a
vertex lock. Constant bone weights inside a rigid region have no influence
gradient, so this knob does not directly preserve all geometry attached to that
bone. Names alone do not establish that higher importance improves the measured
maximum.
`LodChainOptions::vertex_flags` can enforce locks in the chain API. The chain's
existing `attributes` option must be absent in the default mode; character channels occupy it.

```rust
use meshoptimizer_rs::*;
let positions = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [1.,1.,0.]];
let delta = [[0.,0.,0.], [0.,0.,0.], [0.,0.,1.], [0.,0.,1.]];
let shapes = [delta.as_slice()];
let mut character = CharacterInputs::new();
character.blend_shapes = &shapes;
let chain_options = LodChainOptions::new(
    LodTargets::Ratios(&[0.5]), LodErrors::Uniform(1.0));
let result = build_character_lod_chain(
    &[0,1,2,2,1,3], Positions::from_packed(&positions), &character,
    &CharacterOptions::default(), &chain_options,
    &LodQualityOptions::default(), &mut Workspace::default())?;
assert_eq!(result.quality.len(), 2); // bind, full blend target
# Ok::<(), Error>(())
```

`CharacterLodChain::quality` is pose-major then level-major. It contains the F9
report against the original source for the bind pose, every blend target at
weight one, then each caller joint pose. Joint matrices are row-major affine
3-by-4 model-space skin transforms, already including inverse bind transforms.
Joint samples use linear blend skinning of rest positions. They affect quality
reports, not collapse costs. Morph and joint samples are separate: combined
poses, intermediate animation times, and simultaneous blend weights require
caller analysis. F9 maxima are deterministic sampled lower estimates, not
continuous Hausdorff bounds. Normals are not supplied by this API.

Set `CharacterOptions::best_of = true` to build both candidates per level and
select by lower worst sampled symmetric deviation, then fewer triangles, then
plain. `CharacterLodChain::selections` records `method`, `plain_max`, and
`character_max` per level (empty in default mode). The selected quality report
remains in `quality`. Successive levels start from the winner; independent mode
starts each level from the source. Optional chain attributes apply to the plain
candidate only; both candidates share flags, targets and caps. Default behavior
is unchanged. Both simplification and quality work count against whole-call limits.

`CharacterInputs::sample_poses` accepts additional pre-evaluated vertex positions,
including combined morph/animation poses; these follow joint poses in `quality`.
These samples affect selection and reports, but do not add metric channels.
The guarantee is against the plain candidate from the same level input under
these exact samples. It does not compare to a separately evolving plain chain
or bound unobserved poses. Independent-target corpus results and timing cost are
recorded in the character measurements.

Choose an error cap and validate the resulting levels in real poses; a ratio
alone is not an acceptance test. Aggressive targets with a loose error cap can
remove entire components. Default topology rules can also stall on hard normal
seams and complex wedges even after exact welding. An explicitly requested
`SimplifyOptions::PERMISSIVE` can change that tradeoff, but needs a separate
quality check; character simplification does not enable it implicitly.

Levels retain original vertex indices; apply the original skin and blend
streams when rendering them. Whole-call limits include prepared channels,
retained outputs, reports, pose buffers, and nested simplifier/quality work.
Failures return no partial result. See
[character measurements](../parity/0217-character/RESULTS.md) for the measured
tradeoffs and reproducible public-model comparison procedure.
