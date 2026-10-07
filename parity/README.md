# Differential harness

## Fast performance qualification (provisional)

`python3 parity/fast_qualify.py --phase 0.1 --profile moss --full` runs
alternating C++/Rust pairs on separate physical cores, leaving four physical
cores free and excluding busy SMT groups. Set `CARGO_TARGET_DIR`,
`MESHOPT_ARTIFACTS` and `MESHOPT_REFERENCE` to external directories first.
Set `MESHOPT_FAST_GPU_LEASE` to the site's lease-status script and
`MESHOPT_FAST_SCOREBOARD_PATTERN` to its systemd unit pattern. Timing requires
a free lease, no matching active unit, and one-minute load below 12. Unknown
status fails closed. `MESHOPT_FAST_NOT_BEFORE` optionally sets a Unix deadline.
On shared machines run timing through the site's resource-admission wrapper.

Profiles are `moss`/`default` for 0.1, 0.1.x preprocessing and 0.4,
`consumer`/`release-defaults` for 0.3, `default` for 0.2, and `crate` for
historical 0.1 geometry. There is no 0.5 adapter. All results stay provisional
until [FAST_QUALIFY_VALIDATION.md](FAST_QUALIFY_VALIDATION.md) accepts the method.
Existing full qualification commands and bars are unchanged.

`--full` disables family reuse and is required for release runs. `--prior PATH`
can reuse complete, validated families only with identical source, build and
case inputs. `--resume PATH` retains completed cases from an identical-source
partial run; it is distinct from family reuse and works with `--full`.
`--smoke` selects a small subset and is **not a qualification**.
`--workers N` limits parallelism; default physical cores minus two is capped
to reserve four. Records retain raw timings, core load/frequency, input and
executable hashes, stopping intervals and reuse/resume origins.

Each case spends `0.05/case_count` on an anytime median interval using
look-n error `alpha/(n(n+1))`. Codec ratios split the budget between separate
backend medians; phase 0.2 also splits it with the registered floor. After
at least 12 pairs, stop when each applicable interval resolves its bar.
Unresolved cases continue to `--max-pairs` (default 80, range 20–90), then
retain a provisional point estimate. Coverage assumes independent stationary
samples and does not cover nonstationary load or the family geometric mean.
The family bar must be validated empirically against complete full matrices.
Phase 0.2 requires `MESHOPT_DECODER_BASELINE` to name the exact registered
`baseline.json`; rounded display tables cannot replace it.

`prepare_fast_validation.py LABEL --git-revision REV --full-record PATH`
creates a read-only source export. `--source-tar PATH` or `--p02-source-zip PATH`
can use retained sources. `run_fast_validation.py PLAN.json --wait --workers N`
serializes full/fast runs and stores logs and exit receipts under the artifact
directory. Plans supply source roots, archived records and codec input manifests;
phase 0.2 also needs `registered_baseline` and `full_input_manifest`.
`fast_validate.py MANIFEST.json` compares complete matrices and smoke noise
records, writes a report, and issues a receipt only on acceptance.
Receipts bind the runner, statistics and validator. Rejected validation exits 1.

Run the untimed self-tests with
`python3 -m unittest discover -s parity -p 'test_fast*.py'`.


This unpublished package compares the five geometry functions against the
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
requires at least 2,000, including in CI. Case identifiers, seeds, options, exact inputs
and all output bytes are retained. A failure stops immediately without skipping
or regenerating the seed.

The C++ profile is `-std=c++17 -O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD
-fno-fast-math -ffp-contract=off`. Rust uses a release generic-target build
with empty Rust flags. Both preserve scalar f32 order; no fused operation or
parallel reduction is used. The driver checks nearest rounding and gradual
underflow. Compiler, dependency, source, binary, runtime and hardware identities
are captured before execution and checked afterward.

`benchmark.sh` measures the full lane 3 geometry matrix described below.
Input generation, I/O and startup are excluded; validation, required copies,
allocation and execution are included. `--enforce` returns nonzero if any
registered time or memory bar fails.

The earlier lane 2 `report.sh` verifies current source and dependency identities, required counts
and every archived buffer hash before generating the measured reports.
`qualify.sh` provides the same lane-specific verification; neither command
claims complete release 0.1 or unavailable target qualification. Lane 3 keeps
its narrower command evidence in `results/lane3-gates.json` and its combined
verdict in `MEASURED_RESULTS.json`; the earlier release/package checks are
not silently promoted to current evidence. Unimplemented
phases, profiles and command options are rejected.

Records default to `parity/results/`; `MESHOPT_RESULTS` can override the retained
destination. Large ZIP artifacts go to `MESHOPT_ARTIFACTS`, defaulting to
`$CARGO_TARGET_DIR/parity-artifacts`, outside the repository. Set it to a durable
external directory before deleting the build target; this lane uses
`/mnt/linux-extra/moss-capture-archive/meshopt-lane3/final`. JSON records retain archive
and member hashes, not archive contents. Hostnames are excluded from hardware
metadata. ZIP artifacts preserve explicit little-endian input and response
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
overdraw, and 3 is a bounded sqrtf probe with zero vertices. Operations 4/5/6
are simplify/attributes/scale. They append target index count, error bits,
option bits, component count, component weights, packed attributes and one
u32 flag value per vertex. Simplifier responses contain error bits followed by
result indices; scale responds with one f32 bit pattern. Mode 0 compares
outputs; modes 1 and 2 record one to 100 native timing samples. Exact message sizes
and a 128 MiB ceiling are checked before allocation.

Responses begin with `MR01`, then u32 status (zero), output count and timing
count, followed by u32 output values and little-endian f64 timing samples.
Errors emit no response and exit nonzero; failed WASM requests return nonzero
and clear prior output. Unsupported versions, operations, modes and lengths
fail closed. The safe unpublished WASM adapter transports bytes through
checked exports without pointer reinterpretation.

See [COVERAGE.md](COVERAGE.md) for scope and upstream fixture applicability,
[DECISIONS.md](DECISIONS.md) for choices, and the measured reports for results.

## Lane 3 performance

`benchmark.sh --phase 0.1` measures the 204-case RFC geometry matrix and writes
`results/benchmark.json` and `MEASURED_PERFORMANCE.md`. Add `--enforce` to return
nonzero when the geometric-mean, per-case or memory bar fails. Each case records
raw paired timings, dispersion, peak output-plus-scratch bytes and start/end
load. High-load cases are repeated once; both attempts remain in the record.
Tiny timings average 64 calls per sample. See decisions D16 onward for limits.

Mode 1 selects allocating APIs; mode 2 selects caller-buffer APIs with a warm
Workspace and output. Timing responses append one little-endian u64 with peak
requested output-plus-scratch bytes after the f64 samples. Mode 0 is unchanged.
Serialization is outside timing. The C++ allocator callback counts requested
scratch during warm-up only; timing uses the same callback with tracking off.

`python3 parity/profile.py` profiles the slowest measured million-triangle case in each family
using gprofng and verifies the output against the benchmark. It requires a
current benchmark and its original executable. It collects 10 to 100 calls,
targeting approximately 30 seconds per profile. Release harness builds include
line-table debug information for attribution. Experiments and raw buffers go
under `$MESHOPT_ARTIFACTS`; compact records go in `results/profiles.json`.

The benchmark now launches each native driver with the retained input file as
its argument. The driver sends `R` after one warm-up. Each `R` command on stdin
runs one sample and returns `T` followed by a little-endian f64; `S` ends the attempt and returns
the ordinary response with its complete timing stream and peak memory. The
parent alternates backends and verifies streamed times against final responses.
Normal stdin-message execution and wasm32 mode-0 transport remain unchanged.
Timing replies are framed as `T` followed by the f64 value. Both resident
backends are pinned to the same allowed CPU, recorded in `benchmark.json`.
