# meshoptimizer-rs

[![CI](https://img.shields.io/github/actions/workflow/status/Tavrin/meshoptimizer-rs/ci.yml?branch=main&label=CI)](https://github.com/Tavrin/meshoptimizer-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/meshoptimizer-rs.svg)](https://crates.io/crates/meshoptimizer-rs)
[![docs.rs](https://img.shields.io/docsrs/meshoptimizer-rs)](https://docs.rs/meshoptimizer-rs)
[![Licence](https://img.shields.io/crates/l/meshoptimizer-rs.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue)](Cargo.toml)
[![no_std](https://img.shields.io/badge/no__std-alloc-green)](https://docs.rs/meshoptimizer-rs/latest/meshoptimizer_rs/)

A pure-Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3
with exact output parity against its scalar implementation on the recorded
inputs: index order, floating-point error bits and encoded bytes match.
It needs no C++ toolchain and supports `no_std` with `alloc`.
Default builds include SIMD in one audited `unsafe` module; disabling default
features gives an unsafe-free build.

The port targets commit `4c203430ca565cb59a468a91922c76c208169536`, whose
`src` tree equals upstream tag v1.3. Fixtures, differential sweeps, fuzzing
and executed WASM checks establish parity on finite recorded inputs.
They do not prove it for every possible input. See [UPSTREAM.md](UPSTREAM.md).

Crate **0.2.1** includes the following development phases. Phase numbers
label stages of the port, not published crate versions.

| Phase | Included APIs |
|---|---|
| 0.1 / 0.1.x core | Checked views, errors, workspaces and limits; remapping, vertex cache/fetch and overdraw optimization, simplification and quantization. |
| 0.2 glTF decoding | Vertex/index decoding and checked `EXT_meshopt_compression` buffer views, including filters. |
| 0.3 meshlets | Scan, standard, flex and spatial builders; bounds, meshlet optimization, partitioning, spatial ordering and optional cluster LOD. |
| 0.4 codecs | Vertex/index/sequence encoders and decoders, versions and compression levels, Oct/Quat/Exp/Color filters and meshlet codecs. |
| 0.5 scalar algorithms | Stripification, cache/fetch/overdraw/coverage analysis, opacity micromaps, tangents, experimental normals and remeshing. |
| 0.6 parallel batches | Ordered batches for LOD chains, encoding, EXT views, meshlets, cluster-LOD DAGs and hierarchy forests. |
| 0.7 SIMD decoding | Vertex, filter and meshlet kernels for x86 SSE2/SSSE3/SSE4.1, AArch64 NEON and wasm simd128; scalar reference and fallback. |

## Install and quick start

Requires Rust **1.88** or later. The library name is `meshoptimizer_rs`.

```toml
[dependencies]
meshoptimizer-rs = "0.2.1"
```

Simplify a mesh, then optimize its triangle order:

```rust
use meshoptimizer_rs::{
    optimize_vertex_cache, simplify, Positions, SimplifyOptions, SimplifySettings, Workspace,
};

fn main() -> Result<(), meshoptimizer_rs::Error> {
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let indices = [0, 1, 2, 2, 1, 3];
    let mut workspace = Workspace::default();
    let lod = simplify(
        &indices,
        Positions::from_packed(&positions),
        SimplifySettings {
            target_index_count: 3,
            target_error: 0.01,
            options: SimplifyOptions::EMPTY,
        },
        &mut workspace,
    )?;
    let cached = optimize_vertex_cache(&lod.indices, positions.len(), &mut workspace)?;
    println!("{} indices, relative error {}", cached.len(), lod.error);
    Ok(())
}
```

| Feature | Default | Effect |
|---|---|---|
| `std` | yes | Standard error integration. Disable defaults for `no_std`; an allocator is required. |
| `simd` | yes | Audited SIMD decoding. Runtime x86 dispatch with `std`, compile-time features without it. |
| `clusterlod` | no | Port of upstream `demo/clusterlod.h`; the demo is not a stable upstream API. |
| `experimental` | no | Experimental upstream functions and options, including normals and remeshing. |
| `parallel` | no | Rayon batches; enables `std` and requires native threads. |

For an unsafe-free `no_std` build:

```toml
meshoptimizer-rs = { version = "0.2.1", default-features = false }
```

Add `features = ["std"]` for an unsafe-free standard-library build.
Cargo feature unification applies: another dependency enabling `simd` enables
it for the shared crate too.

Most operations offer allocating, caller-buffer (`_into`) and, where relevant,
`_in_place` forms. Codec versions and levels are per-call settings.
`Workspace` reuses scratch storage and enforces per-call byte and work limits;
its defaults are 1 GiB and 2^34 work units. Late caller-buffer errors can leave
a written prefix; unused tails are preserved. In-place calls preserve their
input on failure. See the [API documentation](https://docs.rs/meshoptimizer-rs).

Parallel batches return results in input order, matching sequential bytes at
any thread count. Limits apply per item, outside batch result slots and Rayon
infrastructure; callers should bound batch size. Rayon pool initialization
can panic. Small hierarchy batches run sequentially to avoid dispatch overhead.

## Performance against C++

Recorded on an **AMD Ryzen 9 7945HX, Linux x86-64**, using a pinned core and
interleaved pairs on a shared host. Ratios are Rust time / C++ time: lower is
faster, 1.00 is equal. Validation, required copies and allocation are timed.
The scalar baseline is pinned meshoptimizer 1.3 with SIMD and FMA contraction
disabled. The records below differ in source, corpus and API scope.
For 0.2.1, only the encoders were timed again (see
[Encoder performance in 0.2.1](#encoder-performance-in-021)); no new full-tree
timing was run.

The Moss profile uses thin LTO and one codegen unit. The defaults profile uses
Cargo's consumer release settings: no LTO and sixteen codegen units. The
crate-local fat-LTO profile does not apply to dependents.

| Recorded scope | Moss ratio | Defaults ratio | Evidence |
|---|---:|---:|---|
| Core `simplify`, family geometric mean | 1.049 | 1.060 | [core records](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/MEASURED_PERFORMANCE.md) |
| Standard meshlet builder, mean / max | 1.126 / 1.324 | 1.148 / 1.405 | [P03](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/MEASURED_P03.json) |
| Scalar codecs and filters, historical family mean range | 0.61–1.24 | 0.59–1.24 | [P04 Moss](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/benchmark-0.4-moss.json), [defaults](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/benchmark-0.4-default.json) |
| Opacity-map compact, selected final scope mean | 0.557 | 0.578 | [P05 state](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/P05_CURRENT_STATE.md) |
| Cluster LOD, sum of per-mesh medians | 1.196 | 1.090 | [RFC 113 recovery](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/results/rfc113-clod-recover.md) |

Current SIMD timings use a separate registered driver and SIMD/JS comparators.
Native vertex decode has allocating / caller-buffer means of 1.116 / 1.051;
the resolved allocating maximum still fails. Every current family mean and
native caller-buffer family passes the registered 0.2 rule; the allocating
index, vertex and filtered-view maxima fail. The symmetric WASM JavaScript
adapters pass that rule, but that does not qualify the Vec-returning Rust WASM
API.
See the [full qualification and A/A widths](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/P07_RELEASE_QUALIFICATION.md).

The general family mean / case maximum bars are 1.25 / 1.50. For 0.2 SIMD,
maxima apply only when both independent A/A upper bounds are at most 1.25;
all rows still contribute to means. Unresolved rows remain listed.
Passing a family mean does not clear a failed or inconclusive maximum.

Parallel speed curves compare batches with sequential Rust, not C++.
Small hierarchy batches can be slower, including caller-side pool overhead.
See the [P06 record](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/p06/README.md).

## Comparison with the meshopt crate

The **0.2.0** comparison from 2026-10-06 uses meshopt **0.6.2** (bundled C++ **0.25**) and five frozen inputs on the Ryzen 9 7945HX in two consumer profiles.
**meshopt is faster at allocating vertex decode and Oct/Quat/Exp encoding** by five-case geometric mean in both profiles; **Exp encoding is about 3.4× faster**.
meshoptimizer-rs is faster by the same measure at matched-v1 vertex encoding, Color encoding, Exp/Color decoding and scan meshlet construction; individual cases vary.
meshoptimizer-rs has the 1.3 meshlet codecs, opacity maps, tangents, experimental normals/remeshing, cluster LOD and built-in parallel batches, which meshopt 0.6.2 lacks.
It also has `no_std + alloc`, checked views, typed errors, per-call work/byte limits and reusable workspaces without a C++ toolchain.
Default vertex encodings differ (ours v1, theirs v0), and some meshlet, partitioning and simplification outputs differ across upstream versions.
These shared-host timings do not isolate language, compiler, wrapper or version costs; see the [full comparison, output checks and case intervals](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/COMPARE_MESHOPT_CRATE.md).
This comparison was not rerun for 0.2.1. The 0.2.1 encoder run below also
measured meshopt 0.6.2, but in a different build profile, so it does not
replace the encoder results above.

### Encoder performance in 0.2.1

0.2.1 makes the encoders faster without changing their output. The matched
before/after run used a native AMD Ryzen 9 7945HX pinned to CPU 26, a Rust
release build with fat LTO, one codegen unit and a generic CPU target, the
allocating APIs, and frozen synthetic and cook inputs. Unlike the scalar
baseline above, C++ 1.3 here is built with GCC 13.3.0 at -O3 and uses its
normal runtime SIMD dispatch. Ratios below are family geometric means of Rust
time / comparator time; lower is faster. Before and after use the same
benchmark consumer.

| Encoder | C++ 1.3 before → after | meshopt 0.6.2 before → after |
|---|---:|---:|
| Vertex | 0.904 → 0.658 | 0.909 → 0.662 |
| Index | 1.027 → 0.920 | 1.054 → 0.946 |
| Sequence | 1.228 → 0.967 | 1.229 → 0.970 |
| Oct | 1.107 → 0.622¹ | not rerun¹ |
| Quat | 0.787 → 0.789¹ | not rerun¹ |
| Exp | 0.949 → 0.607 | 1.002 → 0.640 |

All 224 allocating/caller rows resolve under A/A rules. Vertex, index,
sequence and Exp pass the registered mean/max bars in both APIs. ¹ Oct and
Quat show the released code, measured in a separate confirmation run after a
later change reduced the fixed cost of tiny Oct and Quat calls (the first round
measured 0.539 and 0.673 before that change). In the confirmation run all Oct
and Quat rows pass, with a largest 95% upper bound of 1.461× C++, and the allocating family
means are 0.622 (Oct) and 0.789 (Quat). The Quat mean in that run moved from
0.747 to 0.789, a measured slowdown that stays inside the 1.25 bar.
Encoded bytes are unchanged: 17,264 exact comparisons against scalar and SIMD
meshoptimizer 1.3 cover levels 0–9 and both stream versions.

The allocating `vertex-v1-streaming-s4` decode case measures 0.958× C++ here
(95% interval 0.797–1.140), but its matched baseline already passed at
0.952×. This run does not show a gain from the 0.2.1 decode staging, and does
not show that the 0.2.0 gap in other consumer profiles is fixed. The old peer
has independently checked vertex/Exp-zero encoding differences. See the
[complete rows, caller results, profile boundaries and retained evidence](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.1/parity/P03_ENCODER_PERFORMANCE.md).

## Safety and verification

Public APIs check slices, layouts, indices, finite geometry, overflow and limits,
returning typed `Error` values. Crate-controlled allocations use fallible
reservations. The only unsafe allowance is the private `codec::simd` module;
each block has a safety argument in [SAFETY.md](src/codec/simd/SAFETY.md).
Simplification, optimization, encoders and index/sequence decoding are safe Rust.
Scalar builds forbid unsafe code. SIMD must match the canonical scalar bytes;
filters keep scalar arithmetic instead of upstream's SIMD approximations.

Recorded Miri runs cover scalar codecs, dispatch and unwind handling, x86
integer kernels, filters and meshlets. NEON is not Miri-covered. Differential
fixtures, seeded sweeps and fuzz targets compare outputs and malformed-input
handling; CI defines cross-target replay and SIMD safety checks. These checks
are bounded: a compile check or short smoke run does not establish full
per-target release fuzz budgets or execution on every supported platform.
See the [SIMD verification record](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/SIMD_RESULTS.md).

## Known exceptions

These 0.2.0 exceptions still apply to 0.2.1. The owner's 2026-10-06 roadmap
permitted the allocating vertex exception and scheduled its fix for 0.2.1.
0.2.1 stages large owned vertex decodes, and the 0.2.1 encoder run measures
that case at 0.958× C++ in its own profile, but the matched baseline also
passed there, so the original gap is not shown to be fixed. Use
`decode_vertex_buffer_into` where possible: the owner records about 1.04× for
the caller-buffer form of that case. The 0.2.0 complete qualification also
records these results:

| Native allocating case | Final Rust/C++ 95% interval | Status |
|---|---:|---|
| `vertex-v1-streaming-s4` | 1.529–2.118 | Resolved performance failure; caller-buffer is the workaround. |
| `index-2-v0-streaming-s4` | 1.508–3.625 | Resolved maximum failure. |
| `index-2-v1-streaming-s4` | 2.189–2.465 | Resolved maximum failure. |
| `view-1-streaming-s4` | 1.343–1.636 | Resolved maximum fails the upper-bound rule. |
| `varied-view-3-streaming-s32` | 1.619–2.590 | Unresolved: Rust/C++ A/A upper bounds 1.284 / 1.026. |

The roadmap also names the filtered-view maximum near 1.64 and a small S3
case. The latest independent S3 report records caller-buffer
`varied-filter-3-resident-s32` at 1.026–1.124× scalar Rust. The full report lists
all excluded A/A rows and their interval widths; an excluded row has not passed.

Older records retain a sparse allocating vertex-fetch maximum failure,
inconclusive overdraw/sloppy-simplification maxima, and Cargo-default
`partition_clusters` mean / max of 1.450 / 1.778. P05's final selected
compact/coverage/raster-overdraw scope passes; the full matrix was not refreshed
after those edits. The stricter SIMD amendment targets are left for 0.3.

A malformed meshlet tail is accepted by scalar upstream and Rust but rejected
by upstream SSE. Rust follows the scalar oracle; the expanded strict sweep
still fails on that upstream discrepancy. Two RFC 113 stride-32 setups are
invalid because their protect masks exceed the layout. AVX2, AVX-512 and
32-bit ARM SIMD are not implemented; index/sequence decoding remains scalar.
See the [comparison's exception ledger](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/COMPARE_MESHOPT_CRATE.md#against-pinned-upstream-c-13).

## Priorities for 0.3

1. Finish encoder qualification: validate the other consumer profiles.
   Reproduce the historical allocating streaming gap in its original profile;
   retain matched versions and precision settings.
2. Close resolved allocating codec maxima and default partitioning gaps;
   keep unresolved rows and the malformed-tail discrepancy listed.
3. Meet the stricter SIMD amendment targets, requalify fetch/sloppy/overdraw
   residuals, and refresh the full P05 matrix against the integrated source.
4. Extend runtime and performance evidence to AArch64 and other native systems,
   and broaden input, precision and application allocation coverage.

## Licence and acknowledgements

MIT; see [LICENSE](LICENSE). meshoptimizer is written by **Arseny Kapoulkine**.
This independent port preserves upstream's licence notice and credits its
algorithms. It is not affiliated with or endorsed by the upstream project.
[UPSTREAM.md](UPSTREAM.md) records provenance and intentional API differences.
