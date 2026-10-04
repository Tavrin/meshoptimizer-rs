# meshoptimizer-rs

Pure-Rust port of meshoptimizer 1.3, with no C/C++ in the published package.
This is an independent project, not affiliated with meshoptimizer's author.
The crate forbids unsafe code and supports `no_std` with `alloc`.

Work in progress, not yet published. Implements `simplify`,
`simplify_with_attributes`, `simplify_scale`, standard vertex-cache optimization
and overdraw optimization. This is not a complete meshoptimizer 1.3 replacement.

The `codec` module encodes and decodes vertex buffers, triangle index buffers
and index sequences (format versions 0 and 1, every vertex level), applies and
encodes the Oct, Quat, Exp and Color filters, encodes and decodes single
meshlets, and checks EXT_meshopt_compression buffer views. Encoded bytes match
meshoptimizer 1.3 exactly. Version and level are explicit per call
(`VertexEncoding`, `IndexEncoding`) instead of global setters.

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

## Parity

The oracle is meshoptimizer commit
`4c203430ca565cb59a468a91922c76c208169536`; its `src` tree matches v1.3.
Qualification uses scalar-strict C++ with SIMD disabled and FMA contraction
disabled. Measured results and exact input/output records live in
[parity](parity/README.md), outside the published package:

- [API and target coverage](parity/COVERAGE.md)
- [Measured parity](parity/MEASURED_PARITY.md)
- [Measured performance](parity/MEASURED_PERFORMANCE.md)
- [API and qualification decisions](parity/DECISIONS.md)

The recorded fixture corpus compares every meaningful output bit: index order,
result-error bits, scale bits and statuses. WASM executes in Node and compares
with qualified native Rust on identical inputs; it does not use a C++ WASM oracle.

| Function | Fixture corpus | Release sweep | Mismatches |
|---|---:|---:|---:|
| Vertex-cache optimization | 50 | 10,000 | 0 |
| Overdraw optimization | 50 | 10,000 | 0 |
| Simplification | 65 | 10,000 | 0 |
| Attribute simplification | 62 | 10,000 | 0 |
| Simplifier scale | 52 | 10,000 | 0 |

The fixture counts come from [run.json](parity/results/run.json). The sweep
uses seed 20261002 and includes large and medium grids, spheres with seams,
degenerate geometry, small and large coordinate scales, disconnected and
sparsely referenced meshes. Records distinguish finite qualification from a
proof for all inputs.

CI records a qualified Linux x86-64 corpus after exact C++/Rust/WASM comparison.
`parity/report.sh --export-identity DIRECTORY` extracts its input bytes and
native Rust outputs into a deterministic ZIP and writes a manifest containing
the archive SHA-256, each member SHA-256, source hashes and per-function counts.
The macOS arm64, Windows x86-64 and Linux arm64 matrix downloads that same
artifact and executes `parity/report.sh --identity-check DIRECTORY`. These jobs
compare complete output bytes, including a 65,543-value square-root probe,
without tolerance or FMA contraction. C++ runs only in Linux jobs, including
a native arm64 comparison against the same recorded outputs.
Adding these jobs does not establish their execution results: only Linux
x86-64 and wasm32 are locally verified. Remote platform execution remains
separate release evidence.

## Performance

The single-thread benchmark includes validation, required copies, allocation,
scratch and the algorithm. Both resident drivers use one physical core, with
20 or 30 alternating sample pairs per case. Families cover tiny, medium and
million-triangle smooth, seam-heavy and disconnected meshes, allocating and
caller-buffer APIs, attribute widths and simplification ratios. Ratios below
are Rust/C++ time: smaller is faster. Parentheses give the maximum case ratio.

| Function | Fat LTO, lane 3b | Moss thin LTO | Cargo release defaults |
|---|---:|---:|---:|
| Vertex-cache optimization | 1.123 (1.201) | 1.176 (1.344) | 1.189 (1.257) |
| Overdraw optimization | 1.098 (1.417) | 1.079 (1.422) | 1.048 (1.364) |
| Simplification | 1.212 (1.385) | 1.111 (1.201) | 1.180 (1.306) |
| Attribute simplification | 1.208 (1.342) | 1.185 (1.335) | 1.217 (1.310) |
| Simplifier scale | 0.748 (0.884) | 0.829 (1.292) | 0.963 (1.142) |

The [fat-LTO record](parity/results/benchmark.json) uses a geometric mean of
per-case median paired ratios. The RFC bars are a family mean at most 1.25,
no case above 1.50, and requested output-plus-scratch memory at most 1.25
times C++. These are shared-host measurements with retained raw samples,
dispersion and load telemetry, not quiet-host or universal speedup claims.

The crate retains fat LTO and one codegen unit for its own release builds.
Cargo does not apply a dependency's profile to its consumer. Moss-like builds
use thin LTO, one codegen unit and optimization level 3; Cargo defaults use
LTO disabled, 16 codegen units and optimization level 3. Consumer results are
recorded separately and do not inherit the fat-LTO qualification claim.
Both consumer matrices contain 204 cases and were re-measured after the
simplifier's prepaid work accounting (the fat-LTO column predates it and is
historical). Every family passes the mean, maximum and memory bars under both
consumer profiles. The thresholds are unchanged.

## Qualification records

Repository JSON files contain per-function counts, mismatches, seeds, input
coverage and SHA-256 identities. Per-case records, input/output ZIPs, fuzz replay
corpora, crashes and executables live in `$MESHOPT_ARTIFACTS`, defaulting to
`$CARGO_TARGET_DIR/parity-artifacts` (or `target/parity-artifacts`). They are
excluded from the package. Release fuzz artifacts must be outside both the
repository and the disposable build target.

```sh
export MESHOPT_REFERENCE=/path/to/pinned/meshoptimizer
export CARGO_TARGET_DIR=/path/to/isolated/build
export MESHOPT_ARTIFACTS=/path/to/retained/artifacts
parity/run.sh --phase 0.1
parity/sweep.sh --phase 0.1                    # 10,000 cases per function
parity/benchmark.sh --phase 0.1 --consumer-profile moss --enforce
parity/benchmark.sh --phase 0.1 --consumer-profile default --enforce
parity/fuzz.sh --phase 0.1 --cpu-hours-per-target 4 --jobs 8
parity/report.sh --phase 0.1 --verify-artifacts
```

Existing detailed records can be moved with `parity/report.sh --archive-records`.
Existing immutable runs require a fresh artifact directory; interrupted fuzz
runs use `--resume` with identical algorithm sources, binaries, dependencies and seed. Fuzz
budgets accumulate user and system CPU seconds per target, rather than counting
wall time eight times. The existing targets use stable seeded mutations with
invariant checks and no sanitizer or coverage instrumentation. Replay descriptors
retain the seed, execution count and exact source/binary identity needed to
regenerate their corpus. Any target failure is a release blocker.

The owner set the release budget to four CPU-hours per target on 2026-10-04
(RFC amendment §14). Completed CPU time counts toward the amended budget.
The [release record](parity/results/fuzz.json) contains 4.114–4.242 process
CPU-hours per target with zero findings. This is finite invariant testing.

For a local nightly continuation, use the same retained artifact directory:

```sh
parity/fuzz-continuous.sh --wall-seconds 28800 --cores 4
```

The script runs at `nice -n 19`, retains saved seed/execution ranges, and starts
new ranges with unused seeds. It defaults to a four-core cap and an eight-hour
wall budget. SIGTERM stops new chunks and lets the bounded active chunks finish
so CPU usage and exact execution counts can be saved. Each completed chunk
appends cumulative CPU-hours, executions, findings and corpus identity to
`$MESHOPT_ARTIFACTS/fuzz-continuous/ledger.jsonl`. New coverage is `null` because
these existing targets have no coverage instrumentation. A panic, invariant
failure or chunk timeout saves its replay descriptor and logs in `crashes/`
and returns nonzero. Seeded replay regenerates inputs; this is not cargo-fuzz
coverage-guided testing.

[Background robustness](.github/workflows/fuzz.yml) runs weekly or through
`workflow_dispatch`, adding one measured CPU-hour per target with at most four
cores. It restores corpora bound to the sources and compiler, and uploads
replay descriptors, findings, source identities and the ledger. A finding
blocks the next release. The finite runs do not prove absence of bugs.

`report.sh --verify-artifacts` checks available SHA-256 identities and identifies
absent historical artifacts. Missing current release evidence, changed hashes,
stale source, incomplete budgets or incorrect outputs return nonzero. Complete
verified evidence exits zero even when the measured Moss performance bar fails;
`MEASURED_RESULTS.json` then retains `passed: false` and
`moss_performance_accepted: false`. Benchmark `--enforce` returns nonzero for
failed bars. Report completion does not establish release-performance acceptance,
remote platform results or Moss migration.

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
