# MDAG 1: independently streamable cluster LOD geometry

This Rust extension is **not in C++ meshoptimizer**. It is always available as
`clusterlod::dag_format`, including without default features. The existing
`clusterlod` Cargo feature enables DAG building; serialization needs no feature.

`write(groups, clusters, positions, indices, page_size, workspace)` consumes the
successful prefixes from `build_dag_into`. `Reader::read` validates the complete
file, including codec payloads, before publishing a reader. `group` and
`cluster_record` expose resident metadata; `cluster_page` maps clusters to pages,
`page_range` identifies IO ranges and
`decode_page` decodes a separately fetched page without consulting other pages.
Pass `DEFAULT_PAGE_SIZE` (131072 bytes) to select the default page capacity.
The caller retains the resident metadata through the reader's borrowed backing
file. Opening currently requires the full file; this is not a remote directory
reader. Subsequent decode accesses only tables and the supplied page.

## Wire layout

All fields are unsigned 32-bit little-endian words unless stated otherwise.
No padding, native layout, pointer, usize, or host-endian bytes appear. Floating
point fields are IEEE binary32 bit patterns, signed integers use two's complement.
Offsets and lengths are byte counts from the beginning of the file unless noted.

The 32-byte header contains, in order:

| Byte | Field |
| --- | --- |
| 0 | ASCII `MDAG` |
| 4 | major version, exactly 1 |
| 8 | required flags, exactly 0 |
| 12 | group count |
| 16 | cluster count |
| 20 | page count |
| 24 | nonzero maximum page payload size |
| 28 | exact file length |

Immediately following are `group_count` 32-byte records, `cluster_count` 64-byte
records, `page_count` 16-byte records, then tightly concatenated page payloads.
There is no alignment padding, trailer or implicit page rounding.

Group record words: center XYZ, radius, accumulated error, depth, first cluster,
cluster count. Groups partition the entire cluster table, in order, with nonempty
ranges. Depth is 0..63; sphere components and errors are finite, radius and error
nonnegative. Terminal error `f32::MAX` is preserved.

Cluster record words:

| Word | Field |
| --- | --- |
| 0 | owning group ID |
| 1 | signed refined group ID; -1 for original geometry |
| 2 | page ID |
| 3 | offset within page |
| 4, 5 | stored vertex stream and index stream lengths |
| 6, 7 | local vertex count and triangle index count |
| 8 | uncompressed stream bytes: vertices * 16 + indices * 4 |
| 9 | encoding flags: bit 0 vertex codec, bit 1 index-sequence codec |
| 10..13 | sphere center XYZ and radius |
| 14 | original flat first-index offset |
| 15 | reserved, zero |

The group range must agree with ownership. Refinement references must point to
earlier groups with strictly smaller depth and error no greater than the owner;
this proves acyclicity without recursion. Original clusters belong to depth zero.
Flat index offsets partition the original topology, in cluster order. Geometry
has 1..768 local vertices and 3..768 indices, a multiple of three (the builder's
maximum 256 triangles). Every index is below its cluster's local vertex count.
These format limits bound hostile codec expansion and decoder scratch.

Page record words: file offset, byte length, first cluster, cluster count.
Pages partition the cluster table in order. Each is nonempty and no larger than
the caller's page capacity. A cluster is never split across pages. Groups may
span pages, and pages may contain multiple groups. Packing is deterministic first
fit in emission order; a cluster larger than the capacity fails with `Limit`.

Each cluster stores its vertex stream immediately followed by its index stream.
Clusters exactly fill their page with no gaps, overlaps or unreferenced bytes.
Raw vertices have four words: original flat snapshot vertex ID, position XYZ.
Vertices are remapped to local IDs in first-index-occurrence order; readers
reject pages whose index stream violates that order, repeats a snapshot ID
within a cluster, or leaves a vertex record unreferenced. Snapshot IDs
are opaque labels, not offsets into an external vertex buffer; positions are
self-contained, finite and bit-exact. Cross-cluster deduplication is deliberately
absent so every page can decode independently. Index streams are local u32 values.

A compressed vertex stream uses vertex codec version 1 at level 2 with stride
16. A compressed index stream uses index-sequence codec version 1 with stride 4.
These are pinned by named constants (`V1_VERTEX_CODEC_VERSION`,
`V1_VERTEX_CODEC_LEVEL`, `V1_INDEX_CODEC_VERSION`) and do not follow library
defaults; readers reject any other codec version for format v1. Sequence encoding preserves triangle/index order exactly.
Each codec is used only if strictly smaller than the raw stream. Stored and raw
sizes are recorded independently. Compression selection and bytes are deterministic
for v1; changing a pinned codec setting requires a format version decision.

## Compatibility, validation and resource bounds

Readers reject unknown major versions, header flags, cluster encoding flags and
nonzero reserved fields. **No unknown flags may be ignored in v1.** Incompatible
layout or interpretation changes require a new major. Additive library APIs do
not change the on-disk version. This format is separate from unpublished parity
harness formats and does not claim C++ format parity.

The empty file is a 32-byte header with zero counts and a nonzero page capacity.
The file and all counts/offsets must fit u32; arithmetic is checked on every host.
Table extents are checked against input length before count-driven loops.
`Reader::read` uses bounded table scans and decodes one page at a time, discarding
decoded geometry. Each cluster is capped at 768 vertices/indices; memory and work
are bounded by input size with a fixed codec expansion factor. There is no
recursion or allocation driven solely by an unvalidated header count. Workspace
limits still apply to each codec call; the format-owned metadata and output vectors
are outside that workspace. Returned errors distinguish invalid input, unsupported
versions, limits, allocation failure and crate codec errors. No checksum or
cryptographic authenticity is provided.

## Runtime use: projected error, cuts and residency

The tables above carry what a renderer needs to choose clusters; this section
states the rules. They follow the pinned upstream `demo/clusterlod.h`. The
module `clusterlod::cut` implements them over the flat group and cluster tables
(builder output, or `Reader::group` and `Reader::cluster_record`) as a reference
selector and validators. A GPU or streaming implementation should be compared
against it.

Terms: cluster `c` belongs to group `G(c)` and was produced by simplifying group
`R(c)` (`refined`, -1 for source geometry). The clusters of group `Q` are `A(Q)`;
the clusters with `R(c) = Q` are `B(Q)` and together replace `A(Q)`. Group `P` is
a *parent* of `Q` when a cluster lies in both `B(Q)` and `A(P)`. A group without
children `B` is *parentless*: a group that could not be simplified further, or
the final group. All bounds used below are the **group records**: a cluster's
own error is the error of `R(c)`, and the cluster's stored sphere is not used for
level selection.

**Projected error.** Errors and radii are lengths in the mesh frame. For an
instance whose linear transform has largest singular value `s`, multiply error
and radius by `s` (`cut::max_scale` returns a conservative value; centers and
the camera are taken in one frame). With `d` the distance from the camera to the
group center, `proj_y = 1 / tan(fov_y / 2)` and the viewport height in pixels:

```
perspective:  error * s / max(d - radius * s, znear) * (proj_y / 2) * viewport_height
orthographic: error * s * pixels_per_unit        (independent of depth)
```

**Cut rule.** For a pixel threshold `t`, draw cluster `c` iff the projected error
of `G(c)` is greater than `t` and (`R(c) = -1` or the projected error of `R(c)`
is at most `t`). Terminal groups carry error `f32::MAX`, so their clusters are
drawn whenever their producer is within the threshold. A threshold that must give
the finest level draws exactly the clusters with `refined == -1`; do not obtain
it by passing zero, because groups can have zero error.

**Unique cut.** The rule draws exactly one cluster per surface region for every
camera only if, for every parent `P` of every group `Q`, the error of `P` is at
least that of `Q` and the sphere of `P` contains the sphere of `Q`. Containment
makes `d - radius` of the parent no larger than the child's from any camera, so
the projected error never decreases from child to parent. The builder gives
monotonic errors when `simplify_error_merge_previous` is at least one. Merged
spheres are computed in `f32`, and the builder does not guarantee containment
after that rounding: on builder output, `cut::check_monotonic` can report a
parent that misses a child sphere by rounding. `check_monotonic` verifies both
properties in upward-rounded `f64` (a miss of one `f32` ulp is reported) and
`cut::enforce_monotonic` repairs a DAG by raising a parent's error and growing its
radius; apply it to builder output before relying on a unique cut. A fixed-error cut for proxies and fallbacks, independent of any camera,
is the same rule applied to the raw group errors (`cut::select_error_cut`). The
error is the builder's estimate; it is not a certificate of surface distance.

**Resident cut.** The unit of residency is the group: all of `A(Q)` or none. The
resident set must be closed under parents (`cut::group_parents` lists them) and
contain every parentless group. With a partial set, draw `c` iff `G(c)` is
resident, the projected error of `G(c)` exceeds `t` or `G(c)` is parentless, and
(`R(c) = -1`, or the projected error of `R(c)` is at most `t`, or `R(c)` is not
resident). This equals refining a drawn set from the parentless groups downward,
replacing `B(Q)` by `A(Q)` whenever resident `Q` exceeds the threshold
(`cut::refine_sequential`). That procedure fails with `Incomplete` when a cluster
of `B(Q)` is not drawn, which is exactly a closure or monotonicity violation.

Not provided by MDAG 1: tight per-cluster culling bounds (the stored cluster
sphere is the producer's level-selection sphere unless `optimize_bounds` was
set, and no box is stored), an all-level instance bound, and any page-dependency
or residency table. `Reader` gives no knowledge of which groups span which pages.

## Layout reference

Brian Karis, Rune Stubbe and Graham Wihlidal, [Nanite: A Deep Dive, SIGGRAPH 2021](https://advances.realtimerendering.com/s2021/Karis_Nanite_SIGGRAPH_Advances_2021_final.pdf),
particularly “Streaming unit”, motivates packing variable-size whole clusters into
bounded pages while keeping hierarchy metadata resident. This format is an
independent design; no code or Nanite wire representation is copied.

## Reproduction

Run `cargo test --all-features --test meshlets --test dag_format` for the existing
DAG fixture round trip and malformed-input tests. The seeded mutation campaign is
`cargo run --release --manifest-path fuzz/Cargo.toml --bin dag_format -- 600`.
It measures process CPU time on Linux (100 clock ticks per second), records the
seed, execution count and accepted files, and includes page decode on successes.
`DAG_SEED_PATH=fuzz/seeds/dag_format/triangle.bin cargo test --test dag_format emit_seed_if_requested`
regenerates the checked-in valid seed. Native/Wasm identity and real-mesh timings are qualified below.

## Qualification: 2026-10-31

Native and wasm32 WASI re-encoding match byte for byte for both existing DAG
fixtures (81-vertex grid and 17-vertex disk), plus the fixed synthetic hierarchy.
The no-default-feature wasm32-unknown-unknown build also passes.

Five-pass in-memory medians on the three real meshes used by the parity corpus:

| Mesh | Raw geometry MiB | Stored geometry MiB | File MiB | Write ms | Read ms | Page decode us |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| real-214e | 271.237 | 120.770 | 127.855 | 1547.463 | 357.754 | 367.662 |
| real-a082 | 303.885 | 135.285 | 143.156 | 1701.658 | 381.055 | 355.208 |
| real-b6fd | 75.981 | 35.123 | 37.080 | 445.512 | 108.886 | 371.869 |

Raw geometry includes per-cluster duplicated vertices and local indices. File
size includes metadata. Read includes upfront validation and decoding every page.
Page decode is the median over all pages and passes, with metadata already open.
Mesh IO and DAG construction are excluded. CPU affinity was not pinned and the
shared host had concurrent work; these are characterization numbers. No C++
format, performance ratio or runtime rendering claim is made. Input hashes,
exact byte counts, gate exits and identity output are retained in
`parity/dag-format/final-receipts/`. Initial failed attempts are retained separately
from successful retries; see `parity/dag-format/README.md`.

Reader mutation campaign: seed 217, 53,608,094 executions, 5,107,128 accepted inputs, 600 CPU seconds, 600.064 wall seconds, 0 crashes; exit 0. All accepted inputs were independently page-decoded again.
