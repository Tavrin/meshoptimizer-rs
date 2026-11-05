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
level selection. The MDAG 2 builder measures each replacement `B(Q)` against
the union of original leaf triangles reachable through `A(Q)`, before snapping.
Shared refined groups are visited once per region. Its stored error bounds this
sampled symmetric deviation and every inherited child error; it is not a
continuous Hausdorff bound. Builder revision 5 keeps `B(Q)` together in one
parent partition so these source regions remain local. Readers and runtimes
continue to support shared DAGs and apply the ancestor clamp across every
parent. Legacy builds retain upstream error semantics.

**Projected error.** Errors and radii are lengths in the mesh frame. For an
instance whose linear transform has largest singular value `s`, multiply error
and radius by `s` (`cut::max_scale` returns a conservative value; centers and
the camera are taken in one frame). With `d` the distance from the camera to the
group center, `proj_y = 1 / tan(fov_y / 2)` and the viewport height in pixels:

```
perspective:  error * s / max(d - radius * s, znear) * (proj_y / 2) * viewport_height
orthographic: error * s * pixels_per_unit        (independent of depth)
```

**Cut rule.** For a pixel threshold `t`, every group `Q` refines iff its own
projected error exceeds `t` AND all its parent groups refine (vacuously true
for parentless groups). Draw cluster `c` iff its owner `G(c)` refines (or is parentless)
and (`R(c) = -1` or its producer `R(c)` does not refine). Each cluster has
one owner; a group can have multiple parents through `CREF`, and all must
pass before that group replaces its outputs. This ancestor clamp is required
even with ordered stored errors and containing spheres: floating-point
projection can invert the ordering, especially at very large camera distances.
`Threshold::Finest` bypasses error comparisons and draws exactly source clusters;
zero does not mean finest because groups can have zero error.

**Unique cut.** The clamped decisions never reopen a stopped region, so each
source-to-root chain contains exactly one drawn cluster independently of rounding.
GPU traversals must implement the same clamp. In a top-down hierarchy traversal,
start with the clusters of parentless groups, retaining those clusters as the
coarsest fallback, and never descend below a non-refining group; for a
DAG join, wait for all parent decisions before refining the shared group.
Hierarchy-node culling is separate from these group-level refinement decisions.
`cut::check_monotonic` still validates stored error ordering and sphere
containment in upward-rounded `f64`, and `cut::enforce_monotonic` raises parent
errors and grows radii. These geometric properties order ideal projections,
but do not replace the runtime clamp. The runtime clamp does not alter stored
builder output. A fixed-error
cut for proxies and fallbacks uses the same clamp with raw group errors
(`cut::select_error_cut`); the error is an estimate, not a surface-distance
certificate.

**Resident cut.** The unit of residency is the group: all of `A(Q)` or none. The
resident set must be closed under parents (`cut::group_parents` lists them) and
contain every parentless group. Apply the ancestor clamp, require `G(c)` to be
resident, and treat an absent producer `R(c)` as non-refining. This equals
starting with the resident parentless groups and replacing `B(Q)` by `A(Q)`
only when resident `Q` passes the clamped decision (`cut::refine_sequential`).
That procedure reports `Incomplete` when residency lacks a needed parent output;
a projected-error rounding inversion no longer causes `Incomplete`.

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

# MDAG 2: chunked container with streamed pages

MDAG 2 is a separate major version in `clusterlod::dag_format::v2`, also not in
C++ meshoptimizer. MDAG 1 (`dag_format::{write, Reader}`) is unchanged and its
reader rejects MDAG 2 files as `Unsupported`. The reader is available without
features; `v2::write` and `v2::build_key` need the `clusterlod` feature.

An MDAG 2 file is

```
header (64 B) | chunk table (16 B per chunk) | resident chunks | pages
```

The **resident prefix** is everything before the first page. `v2::resident_len`
returns its length from the 64-byte header alone, so a runtime fetches the
prefix, calls `Reader::open` on it, and fetches pages later, from the same file
or from a separate blob (page offsets count from the start of the file; subtract
`resident_len` for a blob that starts at the first page). `Reader::transcode_page`
validates one fetched page against the prefix and writes its memory form
(`docs/PAGE_FORMAT.md`) into a caller buffer without heap allocation.
`Reader::read_all` validates a whole file, every page included, for tools and
fuzzing.

## Writer

`v2::write(&Source, &WriteOptions, workspace)` takes the emission-order tables
of `build_dag_into` (groups, clusters, vertex snapshot, flat indices) and:

1. snaps every vertex to `WriteOptions::grid` (snap the mesh before building,
   with `QuantGrid::snap_positions`, so this is the identity);
2. grows each group's LOD sphere to contain its decoded vertices, then applies
   `cut::enforce_monotonic`, so a DAG built with `simplify_error_merge_previous`
   below one is repaired, not rejected;
3. adds `q = QuantGrid::max_displacement()`, rounded up, to every group error
   below `f32::MAX`;
4. stores groups **coarse-first** (the reverse of emission order) and clusters
   in group order;
5. packs clusters into pages in that order: capacity counts the memory form
   (`page_capacity`, a power of two from 4 KiB to 1 MiB, default 128 KiB); a
   group that fits an empty page is never split, a larger group is split at
   cluster boundaries over consecutive pages, a cluster never spans pages, and
   no page holds more than `max_page_clusters` clusters;
6. pins every parentless group, plus the leading pages while their memory form
   stays within `root_budget` (default one page);
7. records each cluster's refined group (`CREF`) and the page dependencies,
   and builds the culling hierarchy (node width `node_width`, default 4) over
   the stored groups.

`WriteReport` gives pages, pinned pages and bytes, resident and disk bytes, the
parentless and stuck group counts, the number of grown spheres and the
monotonicity repair. A large pinned set is a quality signal: a stuck group deep
in the DAG pins every page before it. Output is deterministic.

## Byte layout

All fields are little-endian `u32` unless stated; `f32` fields are binary32
bit patterns; `u64` fields are two words, low word first. Every reserved field
is zero and checked.

Header:

| Byte | Field |
| --- | --- |
| 0 | ASCII `MDAG` |
| 4 | major, exactly 2 |
| 8 | minor, informational (the newest chunk set the writer knew; 1) |
| 12 | chunk count, at most 64 |
| 16 | page count |
| 20 | page capacity in bytes of the memory form |
| 24 | resident length, a multiple of 16, at most 128 MiB |
| 28 | most clusters per page |
| 32 | build key (`u64`) |
| 40 | resident hash (`u64`): XXH64, seed 0, of bytes 0 to the resident length with bytes 40 to 47 read as zero (`v2::prefix_hash`), so every header byte is covered |
| 48 | most vertices per cluster (1 to 256) |
| 52 | most triangles per cluster (1 to 256) |
| 56 | reserved to byte 64 |

Page capacity and the three limits are format parameters chosen by the writer
(`WriteOptions::page_capacity`, `max_page_clusters`, `max_cluster_vertices`,
`max_cluster_triangles`; defaults 128 KiB, 256, 256 and 256) and returned by
`Reader::page_limits`, so a runtime sizes page slots and per-page tables from
the header. About 200 clusters of the average size measured in
`docs/PAGE_FORMAT.md` fill a 128 KiB page, so the default cluster limit rarely
binds. The writer clamps the clusters-per-page limit to what the capacity can
hold (`PageLimits::cluster_bound`, 63 for a 4 KiB page) and records the clamped
value.

**Buffer views.** The resident prefix and every page's disk form are at most
`MAX_BUFFER_VIEW_BYTES` (128 MiB, the smallest common storage-buffer binding
limit); page memory forms are at most 1 MiB. The writer returns `Limit` and the
reader rejects larger values, so the resident prefix can be bound as one buffer
and each page uploaded as one view.

Chunk table entry: tag (4 printable ASCII bytes), flags (bit 0 *required*; other
bits zero), offset, length. Entries are in increasing offset order; chunks are
16-byte aligned, do not overlap, lie in the resident prefix, and every byte
between them (and up to the resident length) is zero. Tags are unique.

| Tag | Required | Record | Content |
| --- | --- | --- | --- |
| `MESH` | yes | one, 112 B | see below |
| `GRPS` | yes | 48 B per group | LOD sphere (4 `f32`), error (`f32`), depth, first cluster, cluster count, first page, page count, parent count, reserved |
| `NODE` | yes | 32 B per node | sphere (4 `f32`), error (`f32`), group (`i32`, -1 for internal nodes), child offset, child count |
| `PAGE` | yes | 48 B per page | disk offset (`u64`), disk length, memory words, first cluster, cluster count, first dependency, dependency count, page hash (`u64`), flags (bit 0 pinned), reserved |
| `PDEP` | yes | 4 B per entry | page indices |
| `ATTR` | yes | 16 B + 16 B per channel | channel count, layout hash, reserved, reserved; then channel records |
| `MATS` | when materials are supplied | 16 B | material count, three reserved words; required bit set |
| `CSPH` | yes | 16 B per cluster | culling sphere (center, radius; 4 `f32`) |
| `CREF` | yes | 4 B per cluster | refined group (`i32`, -1 for source geometry), equal to the page descriptor's |
| `NAAB` | reserved, optional | | per-node boxes |
| `FARF` | optional | 64 | validated far-field (impostor) parameters |
| `SRCI` | reserved, optional | | source vertex identifiers |

`MESH` words: 0 level count, 1 `bvh_depth` (node count of the longest
root-to-leaf path), 2 node width, 3 group count, 4 cluster count, 5 node count,
6 page count, 7 pinned page count, 8-9 DAG triangles (`u64`), 10-11 source
triangles (`u64`), 12-17 box of every decoded vertex (min, max), 18-21 sphere
containing them (center, radius), 22 grid exponent (`i32`), 23 `q` (`f32`), 24
attribute layout hash, 25 material count, 26-27 reserved.

Cluster descriptors live in their page (owner group, refined group, counts,
grid base, tight sphere, normal cone, material slot; `docs/PAGE_FORMAT.md`).
What culling and cut selection need while pages are absent is resident: group
LOD spheres and errors (`GRPS`), node spheres and errors (`NODE`), one culling
sphere per cluster (`CSPH`, bit-equal to the page descriptor's tight sphere,
which contains every decoded vertex; `Reader::cluster_sphere`) and each
cluster's refined group (`CREF`, equal to the page descriptor's;
`Reader::cluster_refined`). `CREF` defines the parent relation: `P` is a parent
of `Q` when a cluster of `P` has refined group `Q`. Normal cones are in pages only
and are optional for a runtime. The attribute layout hash is the low 32
bits of XXH64 (seed 0) over the `ATTR` channel records; with no channels it is
`0x51D8E999`. Every page header repeats it.

**Ordering.** Groups are coarse-first, so a cluster's refined group has a larger
index than its owner, a group's parents have smaller indices, and a page's
dependencies are smaller page indices: the dependency graph is acyclic by one
comparison per entry. Pinned pages form a prefix starting at page 0.

**Dependencies.** Page `p` depends on every page of every parent group of every
group with clusters in `p`, excluding `p`, sorted and unique. A runtime loads a
page after its dependencies and evicts in reverse. `Reader::open` recomputes
every list and every parent count from `CREF` and rejects a file whose `PDEP` or
`GRPS` disagree, and `Reader::transcode_page` rejects a page whose descriptors
disagree with `CREF`. So for any file `open` accepts, any set of pages that
holds the pinned pages and is closed under dependencies (in particular every
prefix of the page table at least as long as the pinned prefix) makes a
resident group set (groups whose pages are all loaded) that is closed under
parents and holds every parentless group; with the ancestor clamp, `cut::select_cut` on it equals `cut::refine_sequential`, which
reports no `Incomplete`.

**Frames.** Positions, centers and boxes are in the mesh frame; the format
stores no handedness or up axis, and cut selection uses only distances, so it
does not depend on either. A runtime that renders right-handed with the view
looking down -Z passes the camera position and the group centers in one frame
(for example a camera-relative frame rebased in `f64` and stored as `f32`), with
`CutView::scale` an upper bound of the instance transform's operator norm.
`cut::max_scale` is such a bound for any 3x3 linear part, including nonuniform
scale and shear (the square root of the largest Gershgorin row bound of `MᵀM`;
the largest column length is not a bound under shear). Normal-cone tests need
the view direction in the same frame as the cone axis.

## Identity

`v2::xxh64` and the streaming `v2::Xxh64` implement XXH64 in the crate (no
dependency), checked against reference vectors. The page hash covers the disk
bytes of a page, the resident hash covers the whole resident prefix, header
included (its own field read as zero). They detect corruption, not tampering.

`v2::build_key(&KeyInputs)` is XXH64 over the tag `MDAG2KEY`,
`v2::BUILDER_REVISION`, every `Config` field in declaration order (`usize` as
`u64`, `bool` as `u32`, `f32` as bits), the grid exponent, the attribute layout
hash, the disk codec, page capacity, the three page limits, root budget and
node width, the input positions (bits) and indices (as `u32`, so `u16` and
`u32` inputs agree), then the other builder inputs that change simplification:
vertex flags (presence word, count, one byte each), attributes (presence word,
vertex count, component count, value bits), attribute weights (count, bits)
and the protect mask. A cooker can test its cache before building.
`BUILDER_REVISION` (5) is bumped whenever equal inputs produce different bytes.
The writer stores `WriteOptions::build_key` as given (zero when not set).

**Versioning.** The minor version is informational. A later minor may add
optional chunks, which older readers skip. Content an older reader cannot
decode (attribute channels, several material slots) comes with a required
chunk or a non-empty `ATTR`, which older readers reject as `Unsupported`, so
new required content needs a reader upgrade. Reserved fields and flag bits are
never given a meaning by a later minor; a non-zero value is corruption
(`Invalid`).

## Validation (v2 reader)

`Reader::open` reads only the resident prefix. Work is `O(n log n)` in its
length `n` (the logarithm is the sort of each page's distinct parent groups),
there is no recursion, and scratch (one byte and one word per node, one flag
and three words per group, two words per cluster and parent link, and one
page's dependency list) is allocated only after the tables are checked against
the prefix length.

1. Magic, major 2, reserved header fields zero, page capacity a power of two
   from 4 KiB to 1 MiB, page limits in range, resident length at most 128 MiB
   (`Limit`), resident hash over the whole prefix; chunk table as described
   above; the eight required chunks present with the required bit set; cluster
   spheres finite with radius not negative. An unknown chunk with the
   required bit, an unknown flag bit, or an unknown attribute encoding
   is `Unsupported`; unknown optional chunks are skipped and readable
   through `Reader::chunk`.
2. Groups: finite spheres and errors (`f32::MAX` allowed), radius not negative,
   error at least `q` (so any threshold below `q` selects source clusters
   only), depth below the level count (at most 64) and non-increasing,
   cluster ranges partition the clusters in order, each group has at least one
   cluster and its page span is exactly the pages that hold its clusters, parent
   count at most the group index, parentless groups within the pinned pages.
3. Nodes: one root per level at indices `0..level_count`, child ranges inside
   the table and after the parent, every node reached exactly once, internal
   nodes with 1 to `node_width` children, each group in exactly one leaf at its
   level, node spheres containing each child's sphere (the group's for a leaf)
   and errors at least each child's, both in upward-rounded `f64`, and
   `bvh_depth` equal to the longest root-to-leaf path.
4. Pages: disk ranges after the prefix, increasing and non-overlapping, each at
   most 128 MiB; 1 to the header's cluster limit clusters and 64 disk bytes per
   cluster; memory words at least the header
   and descriptors and at most the capacity; cluster and dependency ranges
   partition their tables in order; dependencies sorted, unique and below the
   page's index (so pinned pages depend only on pinned pages); pinned flags form
   a prefix whose length is the `MESH` pinned count.
5. `MESH`: counts match the header and chunk lengths, an empty mesh has zero
   counts, box and sphere finite (box ordered), exponent in -126..=127, `q`
   bit-equal to the value derived from the exponent, reserved words zero.
6. Refinement (`CREF`): each entry is -1 or a group index larger than the
   cluster's owner; for every parent link, the child has a smaller depth, an
   error at most the parent's and a sphere inside the parent's (upward-rounded
   `f64`); each group's parent count equals its number of distinct parents; and
   each page's `PDEP` list equals the list recomputed from the relation. The
   recomputation stops as soon as a list exceeds its declared length.

`Reader::transcode_page` (on load). Work is linear in the page's disk and
memory size: every check below is constant work per cluster or per decoded
vertex.

7. Disk length and page hash match the record; the disk form transcodes
   (`page::transcode_page`: pinned codec versions, descriptor counts at most
   256, bits per channel at most 24, every offset inside the page, local indices
   below the vertex count, every vertex referenced); memory words, cluster count
   and attribute layout hash match.
8. Each cluster's owner is the group whose range holds it, that group's page
   span includes this page, its refined group equals its `CREF` entry, its
   vertex and triangle counts are within the header limits, its `CSPH` sphere
   equals its descriptor's tight sphere bit for bit, and its material slot is
   below the material count.
9. Every decoded vertex lies in its cluster's tight sphere, its group's LOD
   sphere, the `MESH` box and the `MESH` sphere.
10. Attribute blocks match ATTR declarations exactly, contain canonical encoding bits
    and decode to finite values. Material ranges are sorted, cover every triangle,
    use slots below the declared count, and require MATS for multiple ranges.

`Reader::read_all` adds what no single page shows: triangle totals and that the
last page ends the file. Its scratch is that of `open` plus one page of the
memory form.

Not established: no GPU has decoded these pages (see `docs/PAGE_FORMAT.md`);
sizes and timings of MDAG 2 files are not measured here; the group error plus
`q` is an estimate, not a certified surface distance.

## Reproduction (MDAG 2)

`cargo test --release --features clusterlod --test dag_format_v2` covers round
trip against the builder, cuts over page prefixes and dependency-closed page
sets, and the rejection of each rule above, including every single replaced
dependency entry. `--test dag_format_v2_crafted` checks a phantom parent count
on a hand-built file and the transcode bound on a 117 MiB prefix whose page 0
holds 15,000 clusters refining groups of 100,000 pages each;
`--test dag_format_v2_scratch` checks the `read_all` scratch bound on an 18 MiB
file whose last page holds 15,000 children of a 100,000-page parent. Measured
on the development machine (release build): transcoding that page 0 takes
about 2 ms (2.7 s before `CREF`), and `read_all` of the 18 MiB file takes
about 31 ms with a process peak of 39 MiB, also under a 4 GB address-space
cap (before: `Allocation` under that cap, about 6 GB of scratch without it).

Fuzz campaign, 2026-11-04 (`dag_format_v2`, AddressSanitizer,
`-rss_limit_mb=512`, `ASAN_OPTIONS=quarantine_size_mb=16`, binary SHA-256
`3b84e98c…f45f0d`): three 600 s chunks, 1,785.9 CPU-s in all, 14.3 million
executions, coverage 1,653 → 2,054 → 2,060 → 2,060 (the third chunk added
none, so the run stopped coverage-flat), corpus 1,116 files, zero crashes,
timeouts or assertion failures.

Fuzz campaign after the monotone cut rule, 2026-11-04 (same target and options, the oracle
extended to perspective projections with random and far cameras and exactly-once coverage,
binary SHA-256 `8216a18e…7aa50f2c`): three 600 s chunks, 1,791.8 CPU-s in all, 6,878,653
executions, coverage 1,829 → 2,235 → 2,239 → 2,239 (the third chunk added none), corpus 1,214
files, zero crashes.
The fuzz target is
`fuzz/r0220/fuzz_targets/dag_format_v2.rs` (`fuzz/README.md`).

## Attribute and material extension (minor 1)

`clusterlod::v2::build` is the static build entry point for MDAG 2. It snaps
positions before building, uses supplied normals as three attributes of weight
0.5, and measures every accepted replacement against its original source region
with `measure_lod_quality`: 4000 area-scaled interior samples plus triangle
corners in each direction. The region is the union of source leaf triangles
under the replaced clusters, using positions before snapping. Shared refined
groups contribute their source triangles once. The maximum of the two one-sided
maxima floors the existing step estimate and inherited child errors. A final
fine-to-coarse pass restores monotonicity after these source measurements.
The density is 4000 divided by source-region area (clamped below at 1e-12);
each triangle rounds its interior count up, so this is not a hard 4000-sample
cap. Corners are always included, degenerates have corners only, and the same
density is used in both directions. Workspace memory/work limits still apply. This is sampled evidence, **not a continuous surface
error certificate**. The writer adds the grid displacement as before. Legacy
cluster-LOD APIs and MDAG 1 retain their upstream error semantics.

The builder retains source attributes and duplicates vertices by `(source ID,
material slot)`. The slot is an exactly represented f32 component with weight
zero and its protect bit set. Every simplified triangle is checked after all
fallbacks; a mixed-slot triangle makes its entire group stuck. Stuck groups stay
pinned. `Built::stuck_groups` reports all rejected simplification attempts and
`material_stuck_groups` counts mixed-slot rejections specifically.
`WriteReport::stuck_groups` reports parentless groups below the coarsest level. Within each
cluster triangles are stably sorted by slot, then optimized independently
inside each range when cluster optimization is enabled. Dilation is rejected by this entry point because attribute
references must remain attached to snapped source vertices. Error merge factors
below one are rejected to preserve monotonic accumulation.

`ATTR` now contains `channel_count, layout_hash, 0, 0`, followed by 16-byte
channel records: `caller_tag, encoding, source_component_offset, 0`. Tags are
opaque and unique. At most 16 channels are accepted; a channel's last source
component must be below 32. Encoding IDs and errors are in `PAGE_FORMAT.md`.
The hash is the low 32 bits of XXH64 of the channel records, seed zero. The
build key includes layout fields, source normals, source per-triangle slots,
and the measured-step policy. Builder revision is 5 (local source-region ownership requires a recook).

`Source` accepts this layout, an attribute view in snapshot-vertex order, and an
optional slot per vertex. The writer rejects mixed-slot source triangles and
unsorted slot ranges. The builder supplies the required ordering. Material count
is `max(slot)+1`, at most 65536; unused slots need no allocation or table.

Resident opening validates channel record sizes, encodings, offsets, reserved
words, uniqueness and layout hash before reading pages. Page loading validates
attribute lengths and finite decoded values, canonical oct/sign bits, exact
channel declarations against `ATTR`, and material ranges and slot limits. No
allocation is sized by a material slot. Both compressed and uncompressed pages
carry the same memory form. Major 2/minor 0 files remain readable. Readers that
only support position pages reject nonempty ATTR rather than misdecode it.

Material-bearing writes also carry a required `MATS` chunk, 16 bytes:
`material_count, 0, 0, 0`, with the count equal to `MESH`. This declaration lets
older readers reject material-only files while opening the resident prefix,
even when ATTR has no channels. The new reader accepts older single-material
files without MATS; multi-material page tables require the declaration. Unknown
required chunks remain `Unsupported`; a malformed MATS record is `Invalid`.


### Local source regions in the v2 builder

The v2 builder keeps all clusters emitted by one replacement in the same next
partition. Original leaf regions therefore stay local: following a refined
group does not pull source triangles owned by a sibling parent into a region.
This changes builder output and requires builder revision 5; the container
format and legacy builder are unchanged. Component pruning is not enabled.

The sampled symmetric error still bounds the original leaf union before
snapping, the measured step, the simplifier estimate and inherited child errors.
Each measurement builds nearest-triangle spatial indices in both directions,
uses 4000 area-scaled interior samples with per-triangle rounding plus every
corner, and rounds the maximum upward. This remains a sampled lower estimate
of continuous Hausdorff distance. With the parallel feature and unlimited
workspace, independent regions run in bounded worker-sized batches; simultaneous
scratch peaks and work are charged before ordered error assembly. Finite
workspace limits retain serial execution. Thread count does not affect output.

## Optional foliage and far-field policy

The v2 builder can reject alpha simplification steps using caller masks and UVs.
See [the policy decisions](VG_L5_DECISIONS.md) for fixed views, repeat sampling,
default coverage limits, deterministic workspace cost, and the stuck decision.
This is a cooker proxy; whole-cut coverage under runtime filtering needs a
consumer-side test. `v2::measure_fallback` reports the coarsest complete mesh's
sampled quality, including pinned stuck regions, without certifying ray error.

`FARF` has optional chunk flags zero and exactly 64 bytes, little endian:

| Offset | Type | Meaning |
| ---: | --- | --- |
| 0 | u32 | Opaque caller tag |
| 4 | u32 | Reserved zero |
| 8 | u64 | XXH64 of external caller payload, seed zero |
| 16 | u32 | Grid layout: 0 hemisphere, 1 sphere |
| 20 | u32 | Views per axis, 2 through 64 |
| 24 | u32 | View resolution, 1 through 4096 |
| 28 | u32 | Reserved zero |
| 32 | f64 | Texel world error |
| 40 | f64 | View world error |
| 48 | f32 | World handoff error, at least both preceding errors |
| 52 | 3 u32 | Reserved zero |

All error fields must be finite and nonnegative. The reader validates known
`FARF` records even when the caller does not consume them, including length,
flags, enum and reserved words. Older readers skip the optional chunk. No
payload bytes or counts drive allocations. Duplicate tags remain invalid.

Set `WriteOptions::far_field` from `FarField::from_switch`; it hashes caller
payload bytes and rounds max(texel_error, view_error) upward to f32. Compute the
switch from the all-level mesh sphere, then project the stored error with that
same sphere, instance scale and cut view. Replace the entire root cut, including
stuck regions, only when projected error <= the pixel threshold; `Finest` keeps
geometry. Hemisphere grids cover only upper-hemisphere views. The metadata is
not an executable payload and its hash is not authentication. When present it
participates in the writer key; absence preserves existing keys and bytes.
