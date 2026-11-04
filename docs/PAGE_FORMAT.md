# Quantised cluster pages

`clusterlod::page` defines the per-page geometry that a streaming cluster-LOD
runtime binds on the GPU. It is not in C++ meshoptimizer. A page has two forms:
the **memory form**, plain little-endian 32-bit words that a shader decodes, and
the **disk form**, which `transcode_page` turns back into the exact memory words.
The container that holds pages, their dependencies and the resident group tables
is specified separately; this document covers one page.

## Position grid

`QuantGrid { exponent }` places every position at `integer * 2^exponent`, with
the grid origin at the mesh-space origin. Exponents run from -126 to 127 and grid
coordinates are limited to `|q| <= 2^23 - 1`, so every coordinate converts to
`f32` exactly and the decoded position `f32(q) * 2^exponent` is exact.

- `QuantGrid::from_bounds(min, max, bits)` uses `compute_position_exponent`;
  `from_max_error(e)` returns the coarsest grid whose displacement is at most `e`;
  `new(exponent)` lets several meshes (a modular kit) share one grid.
- Snapping rounds to the nearest coordinate, ties away from zero. The distance
  from a position to its snapped position is at most
  `max_displacement() = 2^exponent * sqrt(3) / 2`, rounded up to `f32`.
- Snap **before** building the DAG (`QuantGrid::snap_positions`). Simplification
  selects existing vertices, so every cluster and level then references grid
  positions, and a source vertex decodes to the same bits wherever it appears.
  Copies placed by whole grid steps meet their neighbours exactly. Add
  `max_displacement()` to every finite group error; thresholds below it select
  source clusters only.
- `encode_page` snaps whatever it is given, so positions moved after the build
  (border dilation) are re-snapped there; snapping is idempotent.

## Memory form

All offsets are in words from the page header. Bit and byte streams are
little-endian and least-significant-bit first.

Page header, 4 words: cluster count (at least 1), total memory words, attribute
layout hash (opaque to the decoder), reserved 0. Cluster descriptors follow, 16
words each:

| Word | Field |
|---|---|
| 0 | owning group |
| 1 | refined group (i32, -1 for source geometry) |
| 2 | bits 0-8 vertex count (1-256), bits 9-17 triangle count (1-256), bit 18 single material, bit 19 cone valid; other bits 0 |
| 3 | position stream offset |
| 4 | bits per channel x, y, z (5 bits each, at most 24); other bits 0 |
| 5-7 | grid base x, y, z (i32 grid coordinates) |
| 8 | triangle stream offset |
| 9 | attribute block offset; 0 = no attributes |
| 10-13 | tight sphere center and radius (f32); the sphere contains every decoded vertex |
| 14 | normal cone axis x, y, z and cutoff as signed bytes (bytes 0-3), from `compute_cluster_bounds` on the decoded positions |
| 15 | material slot (bit 18 set), otherwise material range table offset |

Streams follow the descriptors, in cluster order, positions then triangles,
with no gaps:

- **Positions:** vertex `v`, channel `k` is `bits[k]` bits at bit address
  `32 * offset + v * (bx + by + bz) + (bits of the channels before k)`. The
  coordinate is `base[k] + delta`. The stream is padded with zero bits to a word.
- **Triangles:** three local-index bytes per triangle, byte `i` at bit
  `32 * offset + 8 * i`, padded with zero bytes to a word. Every vertex is used.
  Triangles are stored in the order the meshlet codec reproduces: each may be a
  rotation of the input triangle, with the same winding.

`PageView::new` checks all of the above, including the canonical stream offsets,
zero padding, index range, vertex use and sphere containment (computed in f64).
A cluster's group, refined group and cone are not cross-checked; that needs the
group tables of the container.

Size: positions cost `bx + by + bz` bits per vertex, triangles 3 bytes, plus 64
bytes per cluster. The measured bytes per DAG triangle are in the last section.

## Reference decoder (WGSL)

[`page_decode.wgsl`](page_decode.wgsl) decodes positions, triangles, the tight
sphere and the cone from a `array<u32>` storage buffer with only 32-bit loads,
shifts, masks and bitcasts: no 8- or 16-bit storage, no 64-bit integers or
atomics, no subgroup operations. The grid step is built from its bits
(`bitcast<f32>((exponent + 127) << 23)`), never with `exp2`, whose precision
WGSL does not guarantee. Its `replay_page` entry point writes, per cluster in
order, the position bits of every vertex and then the local indices of every
triangle.

[`page_vectors.txt`](page_vectors.txt) holds a two-cluster page: the grid
exponent, the memory words, the compressed disk bytes and the expected
`replay_page` output. `tests/page.rs` checks that the encoder reproduces the file
byte for byte, that the disk bytes transcode to the memory words, that the CPU
decoder produces the expected output, and that the WGSL module parses and
validates with no optional capabilities. A runtime replays the vectors on its
GPU to prove decoder parity.

## Disk form

First word: disk codec. `0` (`DiskCodec::None`) is followed by the memory words.
`1` (`DiskCodec::Compressed`) is followed by:

1. the memory word count;
2. the page header and descriptors, raw;
3. one word per cluster: position stream bytes (low 16 bits), triangle stream
   bytes (high 16 bits);
4. per cluster, the position stream then the triangle stream.

The position stream is the vertex codec (version 1, level 2, pinned) over
12-byte records of grid-relative deltas `(x - base_x, y - base_y, z - base_z)` as
u32. The triangle stream is the meshlet codec with sequential vertex references
`0..vertex_count`, which the transcoder requires. `transcode_page` rejects deltas
that do not fit the descriptor's bit counts, then validates the written words with
`PageView::new`. It does not allocate: codec work is charged to the `Workspace`
and scratch space is on the stack (under 72 KiB, including extension scratch).

## Triangle codec choice

The design started from the meshlet codec for triangles over the
index-sequence codec used by `dag_format` v1, to be switched if the
index-sequence codec was smaller on the corpus. `examples/page_codec_measure.rs`
snaps each mesh to `from_bounds(.., 16)`, builds the DAG with
`default_config(128)`, encodes pages of 64 clusters and sums, per cluster, the
meshlet stream (including its sequential vertex references) and the
index-sequence stream (version 1) over the same stored triangles.

Measured 2026-11-04 on the three real meshes of the parity corpus (grid
exponent -16 for all three; one run, sizes are deterministic):

| Mesh | DAG triangles | Clusters | Meshlet codec bytes | Index-sequence bytes | Ratio | Memory B/triangle | Disk B/triangle |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| real-214e | 5,249,295 | 102,900 | 6,826,484 | 16,262,385 | 2.38 | 12.83 | 10.41 |
| real-a082 | 6,147,766 | 114,956 | 7,701,101 | 19,018,230 | 2.47 | 12.32 | 9.91 |
| real-b6fd | 1,612,169 | 30,625 | 1,981,562 | 4,989,632 | 2.52 | 13.31 | 11.04 |

The meshlet codec is 2.4-2.5 times smaller (about 1.3 B per triangle against
3.1 B), so it stays the triangle codec. Positions dominate the remaining disk
size: with a 16-bit grid over the mesh extent, the vertex codec over 12-byte
delta records leaves the disk form only about 19% smaller than the memory form
overall. Disk size is above the 7 B per triangle
target; better position coding for the disk form is open work. Memory sizes
contain no attributes.

## Attributes and material ranges

Descriptor word 9 is zero for absent attributes, otherwise the word offset of
an attribute block immediately after that cluster's triangle words. Its four-word
header is `block_words, channel_count, vertex_stride_words, vertex_count`.
Each channel then occupies eight words: the four-word ATTR declaration followed
by `minimum.x, minimum.y, scale.x, scale.y` as f32 bits. Non-range channels use
four zero range words. Interleaved vertex words follow the headers. All lengths
are checked before indexing; channels are at most 16 and vertices at most 256.

| ID | Encoding | Words | Maximum decode error and input domain |
| --- | --- | ---: | --- |
| 1 | Oct16x2 | 1 | Euclidean vector error <= 0.000491 from the normalized input; finite nonzero XYZ |
| 2 | Oct12x2 | 1 | Euclidean vector error <= 0.00782 from the normalized input; finite nonzero XYZ |
| 3 | OctSign | 2 | Oct16x2 error for XYZ, exact handedness W in {-1,+1} |
| 4 | UnormRange16x2 | 1 | Each component <= scale/131070 + four f32 ulps each of minimum and scale; finite UV, finite representable range |
| 5 | F16x2 | 1 | Absolute component error <= 16; finite input with absolute value <= 65504 |
| 6 | F16x4 | 2 | Same bound/domain as F16x2 |
| 7 | Unorm8x4 | 1 | Absolute component error <= 1/510 + 1e-7; components in [0,1] |
| 8..11 | Raw32xN, N=1..4 | N | Bit-exact finite f32, including signed zero |

Oct encoders use `encode_filter_oct`'s quantization. X and Y are signed
components in the low/high 16-bit fields (12-bit fields for ID 2, with the upper
eight bits zero). The most negative signed value is rejected. Decode unfolds
the octahedron and normalizes XYZ. ID 3's second word is 0 for positive
handedness and 1 for negative; other values are invalid. The deliberately loose
vector bounds cover quantization, unfolding and f32 normalization: oct rounding
has at most 1/(2M) component error on the folded plane, normalization is bounded
away from zero by 1/sqrt(3), and `16/M + 2e-6` bounds the result
(M=32767 or 2047). Tests include both hemispheres and fold boundaries.

Range UVs store nearest unorm16 integers and decode `minimum + scale*(q/65535)`.
Scale is the per-cluster maximum minus minimum rounded to f32. Constant channels
use zero scale. Half floats occupy low then high 16 bits; colors occupy low to
high bytes. Decoders reject all NaNs and infinities, including half infinities.
`docs/attribute_vectors.txt` gives reference block words and decoded values;
`docs/page_decode.wgsl` implements the same address calculation and decode.
WGSL is syntax/type validated; execution on a GPU remains a consumer test.

If descriptor flag `FLAG_SINGLE_MATERIAL` is set, word 15 is a u16 slot. Otherwise
word 15 points immediately after attributes (or triangles if absent) to a table:
`range_count` followed by `(slot | first_triangle<<16)` words. Count is 2..triangle
count, first range starts at zero, starts strictly increase below triangle count,
and slots strictly increase. The last range ends at triangle count. These rules
cover every triangle exactly once. Positions, triangles, attributes and ranges
are back-to-back in cluster order, with no trailing words.

Compressed geometry-only pages retain their previous disk bytes. A cluster with
attributes or multiple materials appends an extension after its compressed
position/triangle streams: `extension_memory_words, extension_disk_bytes`, then
the extension's little-endian words compressed with vertex codec version 1,
level 2, stride 4. The extension is the complete attribute block and material
table. The transcoder uses fixed bounded byte scratch (at most 67092 bytes),
checks codec version and exact lengths, and revalidates the complete memory page.
This word-stream choice avoids a new codec or per-channel disk directory; a
future disk codec can specialize channel compression without changing memory
pages. No transcoder output allocation is required.

The `replay_attributes` compute entry point uses the same bindings as
`replay_page`. It writes vertex-major, then channel-major vec4 f32 bits for each
cluster, followed by one slot word for each triangle. The axis-aligned vectors
and endpoint UVs in the attribute fixture have exact expected float bits.

`attribute_page_vectors.txt` contains the complete valid input page and the
expected `replay_attributes` output. The focused attribute test reproduces it;
`ATTRIBUTE_VECTORS_PATH=docs/attribute_page_vectors.txt` regenerates the file
when running that admitted test. The smaller `attribute_vectors.txt` separately
pins the manually specified block and its endpoint values.
The `seams` fixture covers handedness signs and UV endpoints; the
`all-encodings` fixture covers every encoding ID (including all four raw widths)
with axis vectors and exactly representable attribute values.
