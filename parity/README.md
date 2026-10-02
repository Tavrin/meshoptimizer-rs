# Differential harness

This unpublished package compares the two foundation modules against the
unmodified meshoptimizer 1.3 source checkout at
`4c203430ca565cb59a468a91922c76c208169536`. No network is used by the scripts.
Set `MESHOPT_REFERENCE` to that checkout and `CARGO_TARGET_DIR` to an isolated
temporary build directory outside both source trees.

Requirements: Rust with wasm32-unknown-unknown installed, Python 3, a GCC/Clang
C++17 compiler, Node.js and `/usr/bin/time` for fuzz resource measurements.
All Cargo builds are offline and locked. The compiler is `CXX` or `c++`.

```sh
parity/check-reference.sh
parity/run.sh --phase 0.1 --profile scalar-strict
parity/sweep.sh --phase 0.1 --cases-per-family 2000 --seed 20261002
parity/js.sh --phase 0.1 --require-coverage
parity/fuzz.sh --seconds 600 --seed 20261002
parity/benchmark.sh --phase 0.1
parity/gates.sh
parity/report.sh --phase 0.1 --verify-artifacts
```

`run.sh` executes the corpus and a 65,543-value sqrtf bit probe against C++
and native Rust, then compares executed wasm32 Rust on the same messages.
`sweep.sh` defaults to 2,000 cases per function; it accepts larger budgets and
requires at least 1,000 for CI. Case identifiers, seeds, options, exact inputs
and all output bytes are retained. A failure stops immediately without skipping
or regenerating the seed.

The C++ profile is `-std=c++17 -O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD
-fno-fast-math -ffp-contract=off`. Rust uses a release generic-target build
with empty Rust flags. Both preserve scalar f32 order; no fused operation or
parallel reduction is used. The driver checks nearest rounding and gradual
underflow. Compiler, dependency, source, binary, runtime and hardware identities
are captured before execution and checked afterward.

`benchmark.sh` records allocating API timings on 1,000,000 triangles. It
alternates Rust/C++ order for ten paired samples; each timed call follows a
warm-up within its process. Input generation, I/O and startup are excluded.
Validation, allocation and execution are included. `--enforce` fails because
release performance acceptance is outside this lane.

`report.sh` verifies current source and dependency identities, required counts
and every archived buffer hash before generating the measured reports.
`qualify.sh` provides the same lane-specific verification; neither command
claims complete release 0.1 or unavailable target qualification. Unimplemented
phases, profiles and command options are rejected.

Records default to `parity/results/`; `MESHOPT_RESULTS` can override the retained
destination. ZIP artifacts preserve explicit little-endian input and response
bytes. Build outputs stay in `CARGO_TARGET_DIR`. After verification, delete that
temporary directory; retained source identities and buffers remain reviewable.

`gates.sh` records fmt, clippy, tests, no_std/WASM, MSRV, rustdoc and package
checks. It disables inherited compiler wrappers and uses isolated temporary
storage. For an uncommitted checkout, package listing uses a source-identical
temporary export; the real working-tree inventory must match. Cargo publish
dry-run uses offline-vendored dependencies and a loopback sparse registry when
the default offline registry refuses configuration access. No remote content
is fetched or package uploaded. See decisions D8/D9 for the resulting limits:
these are package/build checks, not crates.io naming or publication readiness.

## Wire protocol

Requests begin with `MO01` followed by six little-endian u32 words: operation,
vertex count, index/word count, threshold f32 bits, mode, and sample count.
Packed XYZ f32 bits precede u32 indices. Operation 1 is standard cache, 2 is
overdraw, and 3 is a bounded sqrtf probe with zero vertices. Mode 0 compares
outputs; mode 1 records one to 100 native timing samples. Exact message sizes
and a 128 MiB ceiling are checked before allocation.

Responses begin with `MR01`, then u32 status (zero), output count and timing
count, followed by u32 output values and little-endian f64 timing samples.
Errors emit no response and exit nonzero; failed WASM requests return nonzero
and clear prior output. Unsupported versions, operations, modes and lengths
fail closed. The safe unpublished WASM adapter transports bytes through
checked exports without pointer reinterpretation.

See [COVERAGE.md](COVERAGE.md) for scope and upstream fixture applicability,
[DECISIONS.md](DECISIONS.md) for choices, and the measured reports for results.
