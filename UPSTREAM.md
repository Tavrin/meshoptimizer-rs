# Upstream and provenance

Algorithm authority: meshoptimizer 1.3 at
`4c203430ca565cb59a468a91922c76c208169536`. This is three commits after tag
v1.3 (`9e1f07b159d3cb777f1c67ed31fc11fd117986f4`); the `src` tree is identical.
The harness rejects any other or modified reference checkout.

| Rust module | Upstream source | Implemented surface |
|---|---|---|
| `src/cache.rs` | `src/vcacheoptimizer.cpp` | standard `meshopt_optimizeVertexCache` |
| `src/overdraw.rs` | `src/overdrawoptimizer.cpp` | `meshopt_optimizeOverdraw` |
| `src/math.rs` | scalar `sqrtf` usage | pinned libm 0.2.16 backend |

The recovered implementations were checked against the pinned source's
tables, adjacency traversal, score updates, float summation order, cache
timestamps, boundary generation and stable counting sort. The original
algorithms and upstream fixtures retain the MIT notice in LICENSE. Original
Rust infrastructure is also MIT. No code from meshopt-rs or optimesh was
adopted; no locally qualified reusable implementation of these 1.3 modules
was available. The existing Moss decoder concerns the later codec lane and
does not implement these optimizers.

Intentional API differences:

- Checked slices and borrowed views replace raw pointer/count layouts.
- Results replace assertions and unrecoverable allocation behavior.
- Overflow, non-finite geometry/intermediates and explicit work/storage
  exhaustion produce checked errors rather than executing undefined cases.
- Reusable workspaces and per-call limits replace global allocator control.
- In-place calls allocate a temporary output and are unchanged on failure.
- Caller-buffer tails are preserved; late errors may partially write the prefix.

Valid outputs must remain exact. No welding, compaction, implicit cache pass,
general sorting, wider float accumulation, or fused arithmetic is introduced.
The overdraw centroid includes unused supplied vertices, as upstream does.

The published package includes only Rust and documentation. C++ exists solely
in the unpublished parity harness. See parity/COVERAGE.md for future API
milestones and parity/DECISIONS.md for foundation choices.
