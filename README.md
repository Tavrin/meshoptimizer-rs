# meshoptimizer-rs

Pure-Rust port of meshoptimizer 1.3, with no C/C++ in the published package.
This is an independent project, unaffiliated with upstream meshoptimizer.
The crate forbids unsafe code and supports `no_std` with `alloc`.

Work in progress, not yet published. Implements `simplify`,
`simplify_with_attributes`, `simplify_scale`, standard vertex-cache optimization
and overdraw optimization. This is not a complete meshoptimizer 1.3 replacement.

```rust
use meshoptimizer_rs::{optimize_vertex_cache, optimize_overdraw, Positions, Workspace};

let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
let mut workspace = Workspace::default();
let cached = optimize_vertex_cache(&[0, 1, 2], positions.len(), &mut workspace)?;
let indices = optimize_overdraw(&cached, Positions::from_packed(&positions), 1.05, &mut workspace)?;
# Ok::<(), meshoptimizer_rs::Error>(())
```

Simplifiers and optimizers have allocating and caller-buffer `_into` variants.
The cache and overdraw optimizers also have `_in_place` variants. In-place calls leave input unchanged on
failure. Caller-buffer calls preserve the unused tail but may partially write
the used prefix on a later work-limit or numerical failure.

`Positions` accepts packed XYZ, interleaved floats, or initialized bytes with
explicit byte order. `Attributes` supplies checked strided views for attribute-aware
simplification. `VertexFlags` represents `LOCK`, `PROTECT`, and `PRIORITY`
and rejects unknown bits. Primary topology uses `u32` indices.

Simplification returns original vertex references and a linear result error.
Targets are index counts and need not be multiples of three; topology or error
constraints may stop a call before its target. `SimplifySettings` carries the
target, error limit and `SimplifyOptions`. Supported options are `EMPTY`,
`PERMISSIVE`, `LOCK_BORDER`, `ERROR_ABSOLUTE`, `REGULARIZE`, and
`REGULARIZE_LIGHT`. Unknown or unimplemented bits are rejected.
`LOCK` prevents movement, `PROTECT` protects discontinuities under permissive
simplification, and `PRIORITY` increases positional preference without a
preservation guarantee. Scale returns the maximum axis extent without clamping.

Invalid topology, layout, indices, non-finite geometry, overflow and resource
failures return `Error`. `Workspace` reuses scratch and defaults to 1 GiB of
crate-owned output plus retained scratch and 2^34 counted work units per call.
Caller buffers, allocator overhead and fixed stack storage are excluded.
Callers can set explicit limits. The guarantee covers fallible crate-controlled
allocation, not operating-system kills or allocators that abort.

Rust 1.88 and edition 2021 are required. The default `std` feature provides
standard error integration. Disable defaults for `no_std` with an allocator.
`experimental` is reserved and exposes no additional functions in this lane.
Both builds use pinned `libm` 0.2.16 through the same internal math interface.

## Evidence

The oracle is meshoptimizer commit
`4c203430ca565cb59a468a91922c76c208169536`; its `src` tree matches v1.3.
Qualification uses scalar-strict C++ with SIMD disabled and FMA contraction
disabled. Measured results and exact input/output records live in
[parity](parity/README.md), outside the published package:

- [API and target coverage](parity/COVERAGE.md)
- [Measured parity](parity/MEASURED_PARITY.md)
- [Measured performance](parity/MEASURED_PERFORMANCE.md)
- [API and qualification decisions](parity/DECISIONS.md)

Local execution qualifies Linux x86-64 against C++ and wasm32 against the
same native Rust buffers. Other platforms are covered by the CI build/test
matrix; no parity execution is claimed for them. Full release qualification,
AArch64 execution, the 24-CPU-hour fuzz budgets, performance acceptance and
Moss migration remain later work.

## Credit and alternatives

[meshoptimizer](https://github.com/zeux/meshoptimizer), by Arseny Kapoulkine,
is the algorithmic source. [UPSTREAM.md](UPSTREAM.md) records provenance and
intentional API differences; [LICENSE](LICENSE) retains upstream's MIT notice.

[meshopt](https://crates.io/crates/meshopt) provides the established C++ binding
route recommended by upstream. [meshopt-rs](https://crates.io/crates/meshopt-rs)
and [optimesh](https://crates.io/crates/optimesh) are other pure-Rust projects.
This project's local evidence does not qualify those implementations.

See [CONTRIBUTING.md](CONTRIBUTING.md) for checks and DCO sign-off and
[SECURITY.md](SECURITY.md) for private reports.
