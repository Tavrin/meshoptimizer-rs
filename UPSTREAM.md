# Upstream and provenance

The reference implementation is meshoptimizer 1.3 at
`4c203430ca565cb59a468a91922c76c208169536`. This is three commits after tag
v1.3 (`9e1f07b159d3cb777f1c67ed31fc11fd117986f4`), with an identical `src`
tree. The harness rejects any other or modified reference checkout.

| Rust module | Upstream source | Implemented surface |
|---|---|---|
| `src/cache.rs` | `src/vcacheoptimizer.cpp` | standard `meshopt_optimizeVertexCache` |
| `src/overdraw.rs` | `src/overdrawoptimizer.cpp` | `meshopt_optimizeOverdraw` |
| `src/simplify.rs` | `src/simplifier.cpp` | `meshopt_simplify`, `meshopt_simplifyWithAttributes`, `meshopt_simplifyScale` |
| `src/math.rs` | scalar `sqrtf` usage | pinned libm 0.2.16 backend |

The ported implementations were checked against the pinned source's tables,
adjacency traversal, score updates, float summation order, cache timestamps,
boundary generation and stable counting sort. The original algorithms and
upstream fixtures keep the MIT notice in LICENSE. The Rust code written for
this crate is also MIT. No code from meshopt-rs or optimesh was used, and no
locally qualified, reusable implementation of these 1.3 modules was
available. The existing Moss decoder belongs to the later codec work and does
not implement these optimizers.

The API differs from upstream on purpose in these ways:

- Checked slices and borrowed views replace raw pointer/count layouts.
- Results replace assertions and unrecoverable allocation behavior.
- Overflow, non-finite geometry/intermediates and explicit work/storage
  exhaustion return checked errors instead of running into undefined
  behaviour.
- Reusable workspaces and per-call limits replace global allocator control.
- In-place calls allocate a temporary output and are unchanged on failure.
- Caller-buffer tails are preserved; late errors may partially write the prefix.

Output for valid input must stay exact. The port adds no welding, compaction,
implicit cache pass, general sorting, wider float accumulation or fused
arithmetic. The overdraw centroid includes unused supplied vertices, as
upstream does.

The published package contains only Rust code and documentation. The only C++
is in the parity harness, which is not published. See parity/COVERAGE.md for future API
milestones and parity/DECISIONS.md for foundation choices.
