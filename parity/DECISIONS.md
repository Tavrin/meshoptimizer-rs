# Foundation decisions

## D1 — Retain the recovered translations

Keep the standard cache and overdraw implementations after checking the
upstream tables, traversal, arithmetic order and counting sort. Complete the
missing runner, provenance, docs and CI. Do not replace sound code merely to
restart the interrupted lane. No code from another Rust port was adopted.

The bounded reuse review found no locally qualified 1.3 implementation of
these modules. The Moss decoder is a later codec reuse candidate, not an
optimizer. Its current main source uses allocating Vec/String interfaces,
infallible output allocation and a narrower codec version domain; its reuse
still requires separate version, limits, no_std and parity qualification.
The external pure-Rust projects were not fetched or independently qualified.

## D2 — Checked borrowed views and explicit units

Primary indices are u32. Positions support packed XYZ, interleaved f32
elements and initialized bytes with explicit endianness. Float stride/offset
units are elements; byte units are bytes. Byte offsets may be unaligned;
strides retain upstream's multiple-of-four and 256-byte upper bound.
Attributes support up to 32 components and zero-component views. Validate
last-record addresses with checked arithmetic; never reinterpret arbitrary T.
Defer optional u16 convenience adapters to later demonstrated demand.

## D3 — Resource accounting and mutation guarantees

Preserve the RFC defaults: 1 GiB of crate-owned output plus retained scratch
capacity and 2^34 work units per call. Exclude caller-owned buffers, allocator
overhead and fixed stack arrays. Each algorithm counts validation and record
visits, as documented on Workspace; exact work boundaries have focused tests.
Use checked sizes and fallible reservation. Lowering limits releases scratch.

Allocating and in-place variants include a temporary output in the budget.
In-place errors are atomic. Caller-buffer variants avoid that output
allocation and preserve the tail; late work/numerical errors may modify the
used prefix, which is explicitly documented. Strong atomicity for `_into`
would require an unnecessary extra output allocation.

## D4 — Preserve accepted parameters and reject unsupported numbers

Retain all three opaque flags, combinations, empty flags and unknown-bit
rejection. Future simplifier consumers must enforce function-specific support.
Accept every finite overdraw threshold, including values below one; upstream
does not assert a minimum. Reject non-finite thresholds and all supplied
non-finite positions, including unused vertices. Reject overflowing geometric
intermediates instead of running undefined float-to-integer cases in C++.
The overdraw centroid still includes all finite supplied vertices.

## D5 — One math backend and scalar-strict oracle

Use pinned pure-Rust libm 0.2.16 through one internal interface in std and
no_std. The 65,543-value sqrtf probe agrees bit-for-bit with strict native C++
and executed wasm32 Rust, including signed zero and subnormals. Preserve f32
accumulation and tie order; use no fused or wider arithmetic.

Build unmodified upstream sources with the recorded scalar-strict flags.
The C++ driver checks nearest rounding and gradual underflow. Rust builds use
empty flags and compiler wrappers and a generic target. A stale inherited
compiler-cache temporary-directory setting initially broke MSRV compilation;
disabling the wrapper and using the isolated target's temporary directory
resolved it without changing the toolchain or source contract.

## D6 — Lane-specific fixture and target evidence

The 0.1 ledger contains all five required upstream geometry functions; two
are implemented here and the three simplification functions belong to lane 2.
The full later-release 89-name ledger is outside this lane's explicit scope.

Run every applicable upstream native fixture (the two emptyMesh calls), plus
17 corpus cases and 2,000 seeded cases per shipped function. Include meshes
with 1,000,002 indices, degeneracy, disconnection, unused vertices, varying
scales and thresholds. Read current Moss main at
`8d3cc0664948cfb7a779304382c1e8b79133fe2e` without changing its checkout;
the cooker and editor use threshold 1.05, matching the RFC's earlier baseline.
Overdraw comparisons use the verified standard-cache output as input.

Run all five upstream JS suites unchanged as sanity checks. Their only cache
fixture selects Strip, not the standard variant; retain that topology as an
extra Rust input without claiming Strip/fetch parity. No JS overdraw fixture
exists. The JS decoder's asynchronous count is not promoted to Rust coverage.

Execute identical messages in wasm32 and compare every response byte with
qualified native Rust. Retain inputs and C++/Rust/WASM outputs, seeds, counts,
statuses and SHA-256 source/dependency/binary identities. Reject changed
sources, missing backends, missing fixtures and missing or changed artifacts.
Linux AArch64 and complete release qualification remain later gates.

## D7 — Stable ten-minute fuzz fallback

Use two deterministic bounded mutation targets because cargo-fuzz and nightly
are not installed. Each ran 600 elapsed seconds with no crashes: 64,143,023
cache executions and 107,305,797 overdraw executions. Check successful triangle
invariants, resource bounds, destination tails, variant identity and atomic
in-place failures; exercise malformed topology, layouts, flags and float bits.
Retain seeds, iteration diagnostics, execution/success counts and actual
elapsed/CPU/RSS measurements. This lacks sanitizer and coverage instrumentation
and does not replace the later 24-CPU-hour gate.

## D8 — Package listing without Git changes

Cargo refuses even `cargo package --list` when the unpublished package's files
are uncommitted. Committing or changing Git state conflicts with the brief.
Run that exact command in a temporary source-identical export without Git
metadata, compare its inventory with `cargo package --list --allow-dirty` in
the real working tree, and verify all included original-file hashes. The
package excludes parity, fuzz and CI. Retain the direct refusal and the
successful adjusted gate in results/gates.json. No Git metadata is modified.

## D9 — Publish dry run without remote access

Cargo's default crates.io dry run fails before packaging in offline mode,
including with an isolated cache and Cargo 1.88. Preserve the no-remote-fetch
constraint by running the exact `cargo publish --dry-run --allow-dirty`
command against a temporary loopback sparse registry. Vendor locked libm
offline with Cargo, verify the actual package build, record the two local
metadata requests and confirm there is no upload. Retain the verified crate
archive and its SHA-256. This qualifies packaging, not crates.io availability
or registry readiness; the literal default-registry offline command remains
unavailable. `gates.sh` reproduces both D8 and D9 explicitly.

The RFC's dated name check is accepted prior evidence, not a reservation.
No external naming request was made in this lane. Repeat real registry name
checks before any publication, as the RFC requires.

## D10 — Record performance without enforcing a release bar

Measure allocating APIs single-threaded on a 1,000,000-triangle grid, with
ten interleaved Rust/C++ paired samples and a warm-up. Include validation,
scratch/output allocation and execution; exclude generation, I/O and process
startup. Record medians, dispersion, raw samples and identities. Scalar-strict
C++ is appropriate for these modules, which have no explicit SIMD paths.
The lane's requested benchmark is recorded only; `--enforce` rejects an
unsupported qualification claim. Broader workloads, caller-buffer benchmarks,
release performance bars and Moss migration are deferred by lane scope.

## D11 — Simplifier API and stable options

Port the pinned edge-collapse implementation directly, including its half-seam,
complex/fringe classification, wedge traversal, quadric arithmetic, direction
selection and stable 12-bit counting sort. Provide allocating and caller-buffer
variants, original vertex references, and bit-preserved linear result error.
Target counts are index counts and may be nonmultiples of three. A target is
not a guarantee. Scale is allocation-free and returns the extent without an
epsilon clamp; it needs no Workspace argument.

Expose empty options and Permissive, plus LockBorder, ErrorAbsolute, Regularize
and RegularizeLight because these share the implemented paths. Sparse and Prune
need separate algorithms; defer them and all experimental options. Reject their
bits rather than silently approximating behavior. All LOCK/PROTECT/PRIORITY
combinations are accepted for attribute simplification. Priority examines the
canonical vertex, as upstream does, and does not guarantee preservation.

Read Moss main without checkout changes at
`eddd56d1ba95b8bd2e1b8f9296bf080a86801a0e`. Cooker/editor use empty options;
the cooker retry uses Permissive and Boolean locks, mapped to LOCK/empty.
No LockBorder call was found. Cooker targets are 0.5/0.25/0.125 with error
`max(1-ratio, 0.05)`. The editor currently multiplies that error by its clamped
scale before calling relative mode; qualification includes those actual inputs
without changing the consumer. Current cooker attributes are normal xyz,
UV, tangent xyz (weights 2/2/2/1/1/1/1/1), plus optional RGBA (weight 8 each).
Opposite tangent handedness is locked by the consumer. Moss integration remains
outside this lane.

## D12 — Scratch, work and numerical contracts

Keep typed simplifier scratch local to each call. Charge every fallibly reserved
capacity, output capacity and pre-existing retained Workspace capacity against
the same byte limit before using it. Release local scratch on return; Usage
reports the peak. Reusing typed simplifier scratch is a later performance choice,
not required for correctness. Caller-buffer variants reserve/validate first and
may modify the input-length prefix on a later failure, matching D3.

Count validation, hash probes, vertex/adjacency visits, edge searches, quadric
and attribute accumulation, collapse candidates, ranking, counting-sort visits,
flip checks and remapping. Fixed scalar arithmetic and fixed-size histogram work
are not separate units. Tests establish exact work and byte boundaries.

Reject non-finite supplied attributes even when their weight is zero, and
non-finite unused positions. Reject overflowing rescale/quadric/result
intermediates. Preserve upstream's internal absolute-error cutoff calculation,
including its zero-extent division behavior; do not reject an empty or
coincident mesh just because that internal cutoff is non-finite. The public
error limit itself must always be finite and nonnegative. These checked input
and allocation failures intentionally differ from C++ assertions/undefined cases.

## D13 — Final-source evidence and external artifacts

Interpret the brief's default `target/parity-artifacts` as the isolated
`$CARGO_TARGET_DIR/parity-artifacts`; a repository-local default would contradict
its explicit outside-repository requirement. Reject any MESHOPT_ARTIFACTS path
inside this checkout. Retain this lane's archives at
`/mnt/linux-extra/moss-capture-archive/meshopt-lane2`, independently of the
throwaway build target. JSON contains archive/member hashes and metadata only.
Remove the hostname field from both parity and fuzz hardware identities.

Re-run all five functions against final sources. Capture applicable upstream
native fixture inputs before their in-place calls, preserving the original
fixture bodies/assertions, and capture six applicable JS calls with an adapter
while separately executing all five unchanged JS suites. Uint16 JS input is
widened explicitly to the crate's u32 topology API. Record a fixed required
fixture inventory so a missing fixture fails qualification. Remaining native/JS
simplifier options and functions are excluded explicitly in COVERAGE.md.

Use native scalar-strict C++ for exact indices and f32 error bits; execute the
same messages in wasm32 and require native-Rust byte identity. Run 2,000 seeded
cases per entry point, and stable bounded mutation for 600 elapsed seconds per
entry point, retaining D7's instrumentation limits. Benchmarks use the D10
single-threaded allocating method on one million triangles at all three cooker
target ratios, with eight weighted attributes and Permissive for the attribute
entry point. No performance optimization is authorized in this lane.

## D14 — Package inventory with existing Git history

Lane 1's D8 export comparison assumed an unborn repository. This checkout has
history, so Cargo additionally generates `.cargo_vcs_info.json` in the working
repository's package inventory. The Git-free export cannot generate that file.
Compare the exact source inventories with this one generated metadata entry
excluded, record both full inventories, and continue verifying every included
source hash. The actual publish dry run still builds the real working-tree
package, including Cargo's dirty VCS metadata. Do not create synthetic Git
history, stage files, or change any Git metadata to make inventories match.

Current Cargo places the dry-run archive under `package/tmp-crate/`, while
lane 1 expected `package/`. Accept either documented observed output location,
require exactly one candidate, and hash/copy that archive. The temporary
registry copy was checked to have the same bytes; do not mistake it for a
second independent package build.

## D15 — No implicit allocating result clone

Do not implement Clone for SimplifiedMesh: a derived Vec clone would expose an
infallible crate-provided allocation outside Workspace accounting. Callers own
the returned Vec and can explicitly choose their own copying policy. Settings,
options and caller-buffer results remain Copy because they allocate nothing.

## D16 — Measure the geometry matrix and enforce the original bars

Replace the single-grid benchmark with 204 cases: 32, 8,192 and 1,000,000
triangles; smooth, duplicate-boundary seam patches, disconnected patches and
50-percent referenced vertex arrays; both allocating and caller-buffer APIs.
Scale has one allocation-free API. Plain simplification covers targets 0.5,
0.25 and 0.125. Attribute simplification pairs those targets with 1, 8 and 12
components. This is representative coverage, not every attribute/ratio pair.

Use one warm-up per driver invocation and at least ten alternating paired
samples. Tiny samples average 64 calls to reduce clock overhead. If the paired
ratio interquartile range straddles a time bar, collect up to thirty pairs.
Record start/end load for each attempt; repeat cases once when load exceeds
four and retain both attempts. Persistent high load limits confidence and is
not grounds to relax a gate. Retain nonpositive clock intervals and their raw
responses, then retry at most three times; fail if no positive interval results.

Time validation, required copies and the selected API, excluding protocol
serialization. Measure C++ peak requested scratch with its allocator callback
during the untimed warm-up, then disable allocation tracking for timing.
Include output capacity for allocating APIs and exclude caller buffers on both
sides. Rust measurements include retained Workspace capacity. Report each
case, each family geometric mean and maximum, and the unchanged 1.25 time
mean, 1.50 case and 1.25 memory bars. `--enforce` fails when any bar fails.

The expanded before run was intentionally stopped after all tiny/medium cases
and initial million-triangle diagnostics. Retain its partial record externally;
do not describe it as a complete baseline. The prior lane's recorded table is
also retained. Final acceptance requires the complete new matrix.

## D17 — Compact and bound simplifier scratch

Keep the public API unchanged. Store internal topology indices as u32 after
existing topology validation; convert to usize at slice access. Store collapse
candidates in twelve bytes, reusing the error slot for the pre-ranking direction
flag, as upstream does. Allocate candidates from the computed collapse bound,
reserve fallibly and push initialized records instead of zeroing records that
are immediately overwritten. Pre-slice attribute and gradient rows in scoring.

These changes reduce scratch bandwidth and allocation/initialization time.
All 43 tests, the corpus and 2,000 seeded cases per function passed after this
change, including native C++ comparison and executed wasm32 identity.

## D18 — Allocate overdraw scratch from live clusters

Reuse the dead hard-boundary buffer for sort order. Allocate floating sort
values and keys for the actual soft-cluster count instead of every triangle.
Preserve the upstream counting sort and tie order. Consolidate finite checks
only where a nonfinite intermediate necessarily propagates to a checked
result, without changing arithmetic order. Reserve histogram work once.
The corpus and 2,000 seeded cases per function passed with zero mismatches,
including wasm32 identity, after this change.

## D19 — Dispatch sequential position scans once and count work down

Dispatch packed position storage once before sequential bounds/centroid/value
scans. Count work down from the available budget, preserving overflow checks
on the cold failure path. This removes one hot overflow check per counted
visit without changing successful counts or limits. All differential corpus
and sweep cases passed again, including wasm32 identity.

Keep the pinned software libm backend for both std and no_std. Do not enable
architecture-specific unsafe kernels in a dependency to hide a safety-policy
change. `perf` cannot run because the installed wrapper has no matching kernel
tools; use gprofng sampling and retain the experiments outside the checkout.

## D20 — Inline checked accessors and pre-slice cache adjacency

Profiles identified calls to Positions::get in overdraw. Add inline hints to
checked position/attribute accessors, preserving every bounds/layout check.
Pre-slice each cache adjacency search and iterate its records before swapping
the found record with the last one. This preserves traversal and work counts
while exposing the contiguous search to the compiler. Both changes separately
passed the corpus and 2,000 seeded cases per function, including wasm32.

Release harness builds retain line tables for gprofng attribution. The profile
runner chooses the slowest million-triangle case per family from the completed
matrix and verifies its output against the retained benchmark. Profile samples
identify measured costs; they do not prove that every possible safe Rust
implementation must have the same performance.

## D21 — Buffer driver I/O outside timing

The first final-matrix attempt exposed substantial process overhead from the
C++ driver's byte-at-a-time stream operations on million-triangle messages.
Retain that interrupted run as `pre-io-benchmark.partial.json` externally.
Read bounded 64 KiB blocks and serialize responses into a byte vector outside
the timed region. Keep the same protocol, size checks, float bits and algorithm
timing boundaries. Re-run the full native/WASM corpus and sweep, then restart
the complete matrix. The five completed fuzz targets exercise unchanged Rust
sources, so this driver-only change does not invalidate their 300-second runs.

## D22 — Pair resident drivers with one warm-up

The process-per-sample runner reparsed each large input and repeated warm-up
for every pair. Replace that orchestration with two resident native drivers.
Each backend parses and warms up once per attempt, signals readiness, and runs
only when the parent sends its next sample trigger. Alternate backends for each
pair, then request final outputs and the complete timing stream. Verify the
stream against the individually received samples and compare final outputs
exactly. IPC, parsing and serialization remain outside timing. Keep tiny-call
batching, adaptive sample counts and high-load repeats.

Retain the prior interrupted record as `pre-paired-benchmark.partial.json`.
The final record identifies the paired protocol, every sample's backend order,
all attempts and their complete responses. This supersedes D16's invocation-
per-pair detail; the workload matrix and bars are unchanged. The native/WASM
parity and build gates are rerun for the final harness. Rust library and fuzz
sources remain unchanged since the completed five-minute smoke.

The first resident-driver run stopped after four nonpositive clock intervals;
it is retained as `clock-failure-benchmark.partial.json`. Add explicit `T`
sample framing so an unexpected response cannot be interpreted as a duration.
Pin both backends to the same allowed CPU and record that CPU in the result;
this removes cross-core placement as a source of comparison bias and reduces
migration-related timing uncertainty. Retain invalid intervals and fail after
the bounded retry budget; do not replace them with invented timings.

## D23 — Verify inlining in emitted code

Ordinary inline hints left four Positions::get call sites in overdraw's emitted
kernel. Force-inline the checked XYZ accessor and its checked internal adapter.
The pinned million-triangle smooth overdraw probe improved from approximately
3.7–3.8 times C++ to 1.69 times C++; these are diagnostic measurements, not the
full-matrix verdict. The corpus and complete seeded sweep passed again.

The longer attribute profile attributed 13.16 percent of sampled CPU time to
attribute_error and another 13.16 percent to rank itself. Force-inline the
pre-sliced attribute scoring helper into its callers; preserve accumulation
order and the same checked slices. The corpus and seeded sweep passed after
this separate change as well. Final qualification reruns fuzzing because these
last changes modify published Rust sources.

## D24 — Check nonnegative area accumulation once per cluster

The longer overdraw profile attributed 28.22 percent of sampled CPU time to
software sqrtf and 54.50 percent to the kernel; floating-point classification
was a prominent inlined cost. Remove the per-triangle finite check immediately
before sqrt: a nonfinite squared norm produces a nonfinite area, and adding
nonnegative areas cannot make that result finite. The existing checked area
sum still rejects it before output writes. Keep the separate normal-length
check, where taking the reciprocal of infinity could otherwise hide overflow.
No floating-point operation or summation order changes.

Add a boundary test with finite edges and cross product but overflowing squared
area. All 44 tests and the corpus/seeded sweep passed. The focused smooth
million-triangle overdraw ratio fell to about 1.41; use the final matrix for
acceptance rather than treating this probe as qualification.

## D25 — Store cache vertex fields together

Keep live count, adjacency offset and score in one twelve-byte internal record.
This replaces the separate vertex arrays, preserving their total requested
storage and traversal order while sharing bounds checks and cache lines during
scoring. Keep adjacency, triangle scores and emitted flags in their existing
scratch vectors. Charge retained typed capacity in every Workspace operation
and release it in clear/set_limits; cross-operation retention remains charged.
No public API changes. All tests and differential corpus/sweep cases passed.
The focused million-triangle cache ratio improved to about 1.27.

## D26 — Expose independent attribute terms to auto-vectorization

Compute attribute-error terms in groups of four, then add each term to the
running result in original component order. Preserve every term's multiplication
and addition order; there is no horizontal or reassociated reduction. Handle the
tail with the original scalar sequence. Emitted rank code contains packed
multiply/add instructions under the existing generic target, with no intrinsics
or unsafe code. The focused attribute probe improved to about 1.19 times C++.
The corpus and complete seeded sweep remained exact, including wasm32.

Add a separate resource regression proving that another operation charges the
retained cache-vertex buffer and that clear releases it. Final validation has
45 API tests in each feature mode. None of these changes alters public APIs.

## D27 — What made it faster

- Compact simplifier topology to u32 and collapse records to twelve bytes;
  reserve the proven candidate bound and push candidates without redundant zeroing.
- Allocate overdraw sort scratch for actual clusters and reuse dead boundary
  storage. Keep finite checks at points where nonfinite values cannot disappear.
- Dispatch sequential position scans once and force-inline checked position
  access after emitted code showed that ordinary inline hints were insufficient.
- Keep cache live counts, offsets and scores together, and pre-slice adjacency
  searches to share checks and improve locality.
- Pre-slice attribute rows and expose four independent terms to compiler
  vectorization while retaining the original scalar accumulation order.
- Count work down from the budget, leaving overflow handling on the cold
  failure path. Preserve fallible allocation, resource limits and public APIs.

Reject unsafe access/intrinsics, a hidden dependency kernel, reassociated
floating-point reductions, relaxed comparisons, and a reduced workload matrix.
The pinned software math backend remains unchanged. The corpus and complete
seeded sweep passed after each library optimization; the final fuzz smoke is
300 seconds per function, as requested for this lane.

The benchmark measures one warm-up and 10–30 paired samples per backend and
attempt, with 64 calls per tiny sample. Both backends share the recorded CPU.
Every case with start or end load above four is repeated, and both attempts
remain auditable. A profile identifies observed costs; it cannot establish a
universal lower bound for every possible safe implementation.

## D28 — Keep performance acceptance failed

The complete 204-case matrix exits zero and records **FAIL** against the
unchanged RFC bars. Do not mark the performance goal achieved. Keep the safe
optimizations and exact-parity evidence; no public API changed and no Git
mutation was performed.

| Family | Brief's starting 1M ratio | Final matrix geometric mean | Final maximum | Maximum memory ratio | Verdict |
|---|---:|---:|---:|---:|---|
| Vertex cache | 1.51 | 1.322 | 1.477 | 1.000 | FAIL: mean |
| Overdraw | 2.62 | 1.272 | 1.477 | 0.999 | FAIL: mean |
| Simplify | 1.55–1.67 | 1.313 | 1.574 | 1.064 | FAIL: mean and case |
| Attributes | 1.47–1.75 | 1.313 | 1.490 | 1.043 | FAIL: mean |
| Scale | 1.33 | 0.929 | 1.063 | 1.000 | PASS |

The starting column is supplied by the brief, not a controlled before/after
measurement of this expanded matrix. The separately archived lane 2 record
has different historical ratios: cache 1.320, overdraw 2.406, simplify
1.553–1.675, attributes 1.467–1.750 and scale 1.334. It remains at
`/mnt/linux-extra/moss-capture-archive/meshopt-lane3/lane2-benchmark.json`.
Neither historical single-grid table establishes a full-matrix speedup.

Both individual time failures are medium sparse simplification at target 0.5:
allocating 1.519 and caller-buffer 1.574. The latter takes median 1.834467 ms
versus C++ 1.165346 ms. This is the measured residual, not a demonstrated lower
bound. The other families fail only their mean, except scale which passes.
All memory measurements pass.

The matrix ran for 43.89 minutes. Load started at 11.92 and ended at 6.70;
all 204 cases required a repeat and all remained above four. Both attempts,
raw samples, dispersion and backend order are retained. No invalid clock
interval occurred. These conditions limit confidence; they do not turn a
failed bar into a pass.

Final-source gprofng experiments select the slowest measured million-triangle
case per family. Cache samples concentrate in its kernel. Overdraw samples
include the geometry kernel and software sqrt. Plain simplification samples
concentrate in rank, perform, adjacency and initialization; attributes also
spend time constructing quadrics. A separate medium sparse experiment ran
10,000 timed calls and checked every response against the benchmark. Its
samples include ranking, adjacency, initialization and software sqrt.

The collector reports timer-period changes and substantially undercounts
sampled time. The repeated experiment also warns about replacing vfork with
fork; all child experiments are selected. Headers, warnings, experiment lists,
function/line reports and response hashes are retained in `results/profiles.json`,
`results/medium-residual-profile.json`, and the external `final/profiles` and
`final/medium-residual` directories. Treat these as diagnostic attribution,
not calibrated CPU percentages, a proof of the entire C++ gap, or a proof that
no other safe implementation can meet the bar. The brief's alternative
impossibility stopping condition has **not** been established. Performance
qualification remains open; further work needs reliable profiling and measured
safe improvements, not a weaker threshold or an unsafe exception.

Final correctness/build evidence: fmt, clippy with warnings denied, both
feature-mode test runs (45 each), wasm32 build, native/C++/WASM corpus identity,
10,000 seeded differential cases and five upstream JS suites all pass. Five
fuzz targets ran 300 seconds each, totaling 554,090,714 executions and zero
crashes. Stable fuzzing has no sanitizer or coverage instrumentation. This
lane does not refresh legacy package/release gates or claim AArch64 execution.

Verify all current source identities, archive members, sample medians, family
aggregates and executable hashes before retaining binaries and a source archive.
Then remove exactly `/mnt/linux-extra/moss-cargo-targets/codex-meshopt-lane3`.
Large artifacts remain at
`/mnt/linux-extra/moss-capture-archive/meshopt-lane3/final`.

## D29 — Require quiet admission before lane 3b measurements

Keep the full 204-case matrix and the RFC limits: family geometric mean at
most 1.25, every representative ratio at most 1.50, memory ratio at most 1.25.
Read `/proc/loadavg` every 30 seconds until its one-minute value is strictly
below 1.5, with a 3600-second deadline. Retain every check and actual elapsed
wait. Admit the matrix after building and each paired attempt before startup.
Launch both resident drivers with `taskset` on the same recorded physical core;
this also pins parsing and warm-up, eliminating the old post-spawn affinity race.
Record load before and after each pair. Repeat a nonquiet attempt once and
exclude persistent nonquiet attempts from passing qualification.

Use schema 3 for these records. A quiet-admission timeout exits nonzero and
records an incomplete, unqualified run without fabricated ratios. Retain the
historical loaded matrix in `results/benchmark-lane3-high-load.json`; it cannot
serve as the requested quiet before measurement. Do not optimize the library
before the quiet baseline or infer a safe-Rust ceiling from noisy measurements.

## D30 — Use available hardware perf and a writable artifact destination

The `/usr/bin/perf` wrapper refuses the running kernel, but
`/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf` is executable and successfully
records cycles and counts cycles, instructions, branch misses and cache misses.
Use that binary directly. Retain paired C++/Rust instruction disassembly and
profiler diagnostics; hardware sampling attribution is not a proof of a
universal safe-Rust performance limit.

The requested `$MESHOPT_ARTIFACTS/lane3b` is outside the session's
writable roots. Creating it returned `Read-only file system`. Store this lane's
artifacts in `/mnt/linux-extra/moss-capture-archive/meshopt-lane3b` and record the
substitution in `results/lane3b-setup.json`. Use exactly the requested temporary
target `/mnt/linux-extra/moss-cargo-targets/codex-meshopt-lane3b`, then remove it
after retaining the evidence. Leave all Git metadata and state unchanged.

## D31 — Retain paired hardware evidence without claiming a ceiling

Profile the worst historical case in each of the four failing families with
the working perf binary. Both backends run on logical CPU 30, physical core 15,
socket 0; its SMT siblings are 30 and 31. Collect cycles, instructions, branch
misses and cache misses, then separately sample user cycles and retain perf
reports, instruction annotations and complete demangled objdump disassembly.
Every profiled final output matches the archived differential response exactly.
All eight reports and annotations exit zero, with zero reported lost samples.

| Historical case | Rust/C++ retired instructions | Diagnostic finding |
|---|---:|---|
| Cache, tiny seam-heavy caller-buffer | 1.727 | Rust swaps two 80-byte array contents; upstream swaps pointers |
| Overdraw, million disconnected caller-buffer | 1.509 | Software sqrtf accounts for 21.05% of sampled leaf cycles |
| Simplify, medium sparse caller-buffer, target 0.5 | 1.666 | Rank 23.13%, software sqrtf 19.84%, adjacency 7.48% |
| Attributes, tiny smooth allocating, a8, target 0.25 | 1.427 | Rank 17.90%, quadrics 15.29%, sort 12.17% |

The cache instruction annotation shows repeated stack loads and stores in the
inlined swap chain, including the load at address `0x2b931`. The strict C++
overdraw code emits `sqrtss` at `0x8489` and `0x8518`; Rust calls the pinned
software sqrtf with integer multiply/shift refinement and exact rounding.
Medium sparse simplification has 3.367 times the C++ branch misses in this
diagnostic invocation. These counters include parsing, one warm-up and final
serialization. Sampled runs and counter runs are separate, and load is recorded
at both ends. Their observed load is around eight. None is a quiet timing
measurement or an impossibility proof.

The cache reference swap, simplifier slice re-borrows and u32 counting-sort
histogram are concrete candidates after the quiet baseline. Retain the existing
hash implementation: its next-power-of-two size at least `n+n/4`, u32 position
hash, signed-zero normalization and quadratic probing match upstream exactly.
Do not change its load factor or traversal speculatively. Do not enable release
profile flags before the baseline; a root Cargo profile alone would not control
the separate parity package. Reject panic abort, target-cpu flags and unsafe.
Library changes remain deferred until the required quiet remeasurement.

Evidence is in `results/perf-profiles.json` and the external `perf-profiles`
directory, with candidate evaluations in `instruction-evidence.json`.

## D32 — Apply the coordinator's corrected measurement contract

The coordinator superseded quiet admission: this 32-thread machine hosts other
sessions. Stop the wait after 3000 seconds of recorded checks. Measure immediately
on a least-busy physical core selected from three one-second `/proc/stat`
intervals, then choose its least-busy allowed SMT sibling. Pin both resident
backends with taskset before parsing and warm-up. Retain every CPU's selection
samples, the chosen logical/physical core and per-pair load averages.

Take at least 20 alternating Rust/C++ pairs per case, extending to 30 when the
paired ratio IQR straddles a bar. The case metric is the median of the paired
Rust/C++ ratios, not the ratio of independent backend medians. Retain raw paired
ratios, quartiles, standard deviation, median absolute deviation and coefficient
of variation. Keep the complete matrix and unchanged RFC bars. Schema 4 records
this contract; load no longer gates measurements or triggers repeats. D29's
quiet requirement and D31's deferral pending quiet measurement are superseded.

The coordinator also reserved `/mnt/linux-extra/moss-capture-archive`. Move only
this lane's `meshopt-lane3b` directory out and make no further artifact writes
there. The requested `$MESHOPT_ARTIFACTS/lane3b` still rejects
mkdir with `Read-only file system`; the session's writable roots have not changed.
Preserve this lane temporarily at `/tmp/meshopt-lane3b-artifacts`, record the
relocation, and continue the authorized measurements. Final placement at the
requested destination remains a filesystem blocker. D30's archive destination
is superseded; the reserved lane directory has been removed.

## D33 — Optimize the measured cache and simplifier costs

The corrected 204-case before matrix has family GM (maximum) ratios of cache
1.328 (1.407), overdraw 1.282 (1.462), simplify 1.315 (1.511), attributes 1.314
(1.495), and scale 0.943 (1.088). It used CPU 19, physical core 9, with at least
20 alternating same-core pairs per case. The first four families fail. Retain
its inputs, outputs, source manifest, exact source archive and executables in
the external `before` directory. Fresh hardware profiles of these worst cases
have Rust/C++ retired-instruction ratios 1.734, 1.516, 1.666 and 1.394.

Swap references to the two cache arrays, matching upstream's pointer exchange
instead of copying 80-byte array contents. Re-borrow the equal-length position,
quadric and remap tables and initialized collapse prefix in plain simplifier
ranking. Keep the same arithmetic, candidate ordering and work visits. Change
the counting-sort histogram to u32: topology bounds input indices by u32::MAX,
and pick emits at most one candidate per index. Prefix sums still use usize;
all stored bucket values fit u32. Preserve stable bucket order.

Use target-independent `codegen-units = 1` and `lto = "fat"` in the root, parity
and fuzz release profiles. The separate harnesses require their own profiles;
a library's root profile does not control downstream consumers. Record the
actual three profile configurations in executable identities. Keep generic
targets, no contraction, panic unwinding and the pinned software math backend.
Reject unsafe architecture kernels, changing hash probing or arithmetic order.

After this batch, fmt and clippy pass. The phase-0.1 corpus and default 10,000-case
sweep have zero mismatches, including native C++ and executed WASM identity.
Twenty-pair worst-case diagnostics improve cache 1.407 to 1.291, overdraw 1.462
to 1.420, simplify 1.511 to 1.462 and attributes 1.495 to 1.284. These are subset
observations with recorded dispersion, not complete-matrix acceptance. Final
feature-mode tests, the full timing matrix and five final 300-second fuzz runs
remain required after the last library change.

## D34 — Remove redundant loop checks and gradient temporaries

Use fixed-size triangle slices in cache score initialization, output copying
and both overdraw cache-simulation passes. This exposes the triangle width to
the compiler without changing traversal or work visits. Move work-exhaustion
and overflow reporting into a cold, non-inlined helper; successful visits retain
the same subtraction and budget check. In simplifier adjacency, borrow the
vertex counter prefix and update each selected counter through one reference.
Re-borrow equal-length position, quadric and remap tables in quadric creation.

Accumulate each computed attribute gradient directly into the three triangle
vertices instead of zeroing and returning a 32-component temporary. Components
are independent: each component sees the same three additions in the same order,
including when a degenerate triangle repeats a vertex. Quadric scalar accumulation
and counted-work boundaries stay unchanged. The phase-0.1 corpus again passes
with zero mismatches, including executed WASM identity.

Reject three standalone safe square-root candidates. Integer isqrt with exact
rounding matches all mantissas at seven central/extreme normal exponents but
is about 1.58 times the pinned backend's time in the diagnostic loop. A specialized
inlined Goldschmidt transcription and a floating Newton estimate with exact
integer rounding match the same corpus; their median loop times are 0.0611,
0.0640 seconds versus libm 0.0634 seconds. Those marginal results do not justify
duplicating the math implementation. Retain libm and preserve its special-value,
subnormal and cross-target behavior. These experiments are diagnostics, not
Rust/C++ matrix acceptance or a safe-Rust ceiling proof.

## D35 — Cache exact reciprocals and batch successful scan accounting

Store the positional quadric's exact `w == 0 ? 0 : 1 / w` result after initial
accumulation and refresh it after every positional merge. Ranking multiplies
by that stored f32 result in the same expression order. This removes repeated
divisions without approximating a reciprocal or reassociating arithmetic.
The private quadric gains one f32 field; the measured memory bar still applies.
Attribute quadrics do not use the cache. Numerical validation excludes this
derived field: an overflowing reciprocal must still fail when its ranked error
is checked, preserving the previous error point and work usage.

For complete sequential scans, first check whether the remaining work budget
covers the scan. Charge once on success; on a callback error charge only the
visited prefix, including the failing record. If the budget cannot cover the
scan, retain the original visit-by-visit checks and mutations. Apply this to
topology, cache initialization, overdraw simulation/position scans, plain
simplifier ranking, attribute validation rows and counting sort. Keep nested
or variable-work traversals on their existing checks. A regression compares
errors, consumed work and partial mutations for every budget 0 through 8 and
every failure position in a five-record scan. Existing exact-budget API tests
also pass. The exhaustion helper remains cold.

The phase-0.1 native/C++/WASM corpus has zero mismatches after this batch. Freeze
library and harness sources for the final gates, default 10,000-case sweep and
complete 204-case paired timing matrix. These changes are accepted only if those
checks and the final five 300-second fuzz smokes pass. Do not infer acceptance
from the earlier diagnostic subsets.

## D36 — Close the measured residual with exact memoization and first writes

The complete intermediate matrix passes cache 1.130 (maximum 1.237), attributes
1.244 (1.359) and scale 0.809 (1.020). Overdraw 1.264 (1.456) and plain simplify
1.266 (1.466) still fail the family mean. Retain this failed enforced run in
`change5`, with its exact source and executables. It is not final acceptance.
Current-source diagnostic hardware profiles still attribute 17.70% of plain
simplifier leaf cycles to software sqrtf; its retired-instruction ratio is 1.666.

Memoize the pinned sqrtf per API call in 64 direct-mapped entries on the stack
(512 bytes). Compare the complete u32 operand pattern, including signed zero;
collisions recompute the pinned backend. Seed entries with the exact positive
zero input/result. Reuse only identical operands and return the stored f32
unchanged. No approximation, floating reassociation, target-specific kernel,
unsafe code, heap allocation or persistent/global cache is introduced. Thread
this cache through overdraw geometry and simplifier quadrics/flip checks.

Area-only operand diagnostics estimate 93.5% to 99.7% hits in medium geometries
with 64 entries. Sixteen entries lose some hits; 256 adds no medium-case benefit.
Choose 64 to retain room for the additional nested simplifier operands. Prototype
paired diagnostics retain exact C++ outputs; they are not a full-matrix gate.
The cache regression covers forced collisions, signed zeros, subnormals,
infinities, NaNs and 65,536 generated bit patterns against the pinned backend.

Reserve positions, remap and wedge storage fallibly at the original budget
points, then initialize each record at its first sequential write with Vec::push.
Retain an explicit vertex count for adjacency and classification before positions
are populated. The first adjacency is unwelded; remap/wedge are complete before
classification and every welded adjacency. Initialize forward/back and the hash
table to NONE once, rather than zero followed by NONE. Those initialization
methods are private and called once per State. Capacity accounting, hash sizing,
probe order, failure precedence and counted-work boundaries remain unchanged.

After publication, the phase-0.1 native/C++/WASM corpus and the default 10,000-case
sweep have zero mismatches. Both feature-mode test suites, all three clippy/fmt
checks, the no_std WASM build and seven measurement regressions pass. Freeze this
source for the final enforced matrix and subsequent five 300-second fuzz smokes.
Increase profiler DWARF stack capture to 32 KiB to cover the fixed histograms and
new cache when inspecting the final binary. No safe-Rust ceiling is claimed.

## D37 — Inline ranking and reduce memo collisions

The second full intermediate matrix finishes with cache 1.129 (maximum 1.255),
overdraw 1.059 (1.400), attributes 1.219 (1.321) and scale 0.786 (0.922) passing.
Plain simplification 1.262 (1.434) still fails the mean. CPU 25 ran all 204 cases
with the corrected paired protocol. Retain this enforced exit-1 result, exact
source and executables in `change6`; it is not final acceptance.

Reject a separate candidate that removes finite-error checks under explicit
quadric/coordinate magnitude bounds. Its selected-case mean is 1.287 versus
1.258 for the current binary's same cases. Keep every numerical check. The
current-source hardware profile instead exposes an out-of-line ranking closure
at 20.25% of sampled leaf cycles, with a call per candidate. Place the checked
per-candidate calculation in an explicitly inlined private helper and spell out
the full-budget and limited-budget loops. Full-budget numerical failure charges
only the visited prefix; limited budgets still charge before each visit. A new
regression covers every budget 0 through 6 and failure position in five records,
checking error, consumed work and the unchanged result suffix. Batch three
additional fixed scans: adjacency counting, quadric regularization and collapse
remap/lock initialization, through the already qualified Work::scan contract.

Increase the per-call square-root memo to 512 entries (4096 stack bytes) and
index with the high nine bits of wrapping u32 multiplication by 0x9e3779b9.
Compare the complete operand bits before reuse and retain the same libm miss
path. D36's area-only estimate did not cover collisions among normalization,
area and flip operands. Nine same-core, 20-pair Rust-candidate/Rust-current
comparisons show a 0.964 geometric-mean ratio. A broader C++ diagnostic retains
all 141 exact outputs and gives means cache 1.137, overdraw 0.989, plain 1.173,
attributes 1.205 and scale 0.839. These selected cases are diagnostics, not the
full RFC gate. Update the memo regression to generate actual collisions under
the new hash as well as its special values and 65,536 generated bit patterns.

Publish this candidate, then rerun phase 0.1, the default sweep, feature-mode
and harness gates, executed WASM identity, the complete paired timing matrix
and five final 300-second fuzz smokes. Allow explicit family selection in the
profiler so the final maxima of previously failing families can be inspected
even if they now pass; use the genuine final timing record for that selection.
No numerical checks, parity requirements or RFC bars are relaxed. No universal
safe-Rust ceiling is claimed.

## D38 — Relocate artifacts off the full root filesystem

The coordinator requires artifacts on `/mnt/linux-extra/meshopt-artifacts/lane3b`,
with no further `/tmp` or home artifact writes. That directory and the former
home destination are not visible in this sandbox. Creating the requested mount
path fails with `Read-only file system`; it is outside the exposed writable roots.
Use `/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/lane3b` on the writable
large volume as the available fallback. This artifact directory is separate from
the exact build target that must be deleted. The requested destination remains
a sandbox/mount blocker, not a benchmark or parity failure.

Move closed files immediately, verifying each SHA-256 before and after the move.
This relocates 609 files and frees about 2.7 GB on root. Defer only the open final
benchmark ZIP and its two logs until the remaining cases finish and writers
close; changing an active cross-filesystem ZIP would invalidate its evidence.
All subsequent commands and artifact writes use the large-volume fallback.
There are no home artifacts visible to move. Retain an exact original copy of
any record whose location fields are updated, and publish a relocation map with
original and updated hashes. Immutable buffers, binaries and source archives
retain their existing hashes. D32's temporary `/tmp` location is superseded.

## D39 — Final gate and stop decision

The complete final 204-case enforced benchmark exits 0. Family GM (maximum)
ratios before -> after are cache 1.328 (1.407) -> 1.123 (1.201), overdraw
1.282 (1.462) -> 1.098 (1.417), plain simplify 1.315 (1.511) -> 1.212 (1.385),
attributes 1.314 (1.495) -> 1.208 (1.342), and scale 0.943 (1.088) ->
0.748 (0.884). All timing and inherited heap-memory bars pass; the largest
memory ratio is 1.095. Stop optimizing under the coordinator's amended
measurement contract. There is no remaining case requiring a ceiling claim.

Before uses CPU 19/core 9; after uses CPU 20/core 10, chosen from the measured
least-busy physical cores. Every case has 20 or 30 alternating same-core pairs.
After one-minute load ranges from 9.62 to 89.98; no load admission or repeats.
Median per-case ratio CVs are 11.4%, 8.6%, 10.1%, 10.7% and 26.5% respectively.
Maximum CVs and all outliers, raw ratios, quartiles, MAD, standard deviation,
pair loads and selection samples remain in benchmark.json. These are shared-load
paired measurements; no quiet qualification or universal speedup is asserted.

Phase 0.1 and the default 10,000-case sweep exit 0 with zero mismatches, including
executed WASM identity and the pinned math probe. All feature-mode tests (48
per configuration), no_std WASM build, root/parity/fuzz fmt and clippy, and seven
measurement regressions pass. Every final seeded fuzz target runs for at least
300 elapsed seconds, with zero crashes and unchanged source/binary identities.
Fuzz affinity excludes both siblings of the benchmark's physical core.

Hardware stat/record, report, annotation and full objdump pass on the final
maximum of each previously failing family, with all four required events and
exact outputs. On the identical original worst plain input, Rust/C++ retired
instructions fall from 1.666 to 1.514; C++ counts differ by only 121 instructions
out of 1.234 billion. Profiles diagnose the cost; the paired matrix establishes
the bar. Retain source archives and all final native, WASM, C++ and fuzz binaries
before deleting the exact isolated build target. Verify Git metadata unchanged.

All 612 original moved files have hash-preserving copies on the large volume;
updated records retain byte-for-byte originals with a relocation map. The /tmp
lane directory is removed and no subsequent commands write artifacts there.
The requested /mnt/linux-extra/meshopt-artifacts/lane3b remains outside the
exposed writable roots and not visible; mkdir returns Read-only file system.
Keep evidence at /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/lane3b.
Final placement at the exact requested path is the sole filesystem residual.

The short final C++ cache cycle capture lost 27 samples. Repeat only that capture
with four times the event period; its output stays exact and all final captures
have zero lost samples. Retain the superseded capture and original hashes.

Cleanup completed: all archived file hashes were verified before deleting
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-lane3b`. All 125 Git metadata
file hashes remain unchanged. A final relocation audit resolves and verifies
all 612 original file hashes, including the closed benchmark and records whose
original bytes are retained separately. `lane3b-cleanup.json` records removal;
`lane3b-urgent-relocation.json` records each original file's final location.

## D40 — Lane 4 scope and artifact placement

The checkout starts clean on main at 8ec3b3d0ca78caedd445dfb827a9579e2b8d7fef.
Do not change Git metadata or the parallel 0.1.x checkout. Keep implementation
inside the named shell interfaces (run.sh, sweep.sh, benchmark.sh, fuzz.sh and
report.sh), qualification records, CI workflow, README and CHANGELOG. Leave
library, test, Rust harness and fuzz-target sources unchanged. The report
interface owns the external-record adapter so this lane does not broaden its
source-file ownership to runner.py or performance.py.

Creating /mnt/linux-extra/meshopt-artifacts/lane4 fails with Read-only file
system: that path is outside this sandbox's writable roots. As in D38, retain
artifacts at /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/lane4 on the
large volume, separate from the exact disposable build target. Exact requested
placement remains a filesystem blocker. Do not put fuzz artifacts on root,
in the repository, /tmp or home. Preserve every migrated record's original
bytes and SHA-256; summarize historical evidence without promoting it to a
current source-bound pass. Unavailable historical buffer hashes remain explicit.

## D41 — Profiles and unchanged performance bars

Keep the crate's own fat-LTO, one-codegen-unit release profile as its local
measurement baseline. It does not configure dependents. Override the dependent
parity harness's effective Cargo profile explicitly for Moss (thin LTO, one
codegen unit, optimization level 3) and defaults (LTO false, 16 codegen units,
optimization level 3), with release debug disabled in both. Record these exact
environment overrides alongside the source manifests and executable hashes.
Run both complete 204-case paired matrices even if either fails. Keep the RFC
1.25 family geometric mean, 1.50 maximum and 1.25 memory limits unchanged;
report each family and do no performance work in this lane. Retain D39's
shared-load protocol and uncertainty limits.

## D42 — CPU-accounted long mutation fuzzing

RFC 5.4 requires 24 CPU-hours per exposed target: five targets require at least
120 process CPU-hours. Use at most eight workers, pinned to separate physical
cores; consumer benchmarks select among the remaining physical cores. Each
seeded chunk records actual user plus system CPU seconds from /usr/bin/time.
Only their per-target sum counts toward release; elapsed time is retained
separately. Completed chunks are resumable only with identical source,
dependencies, executable hashes, seed and required budget. Keep the final
source and executable copies outside the target that must be deleted.

Use the existing stable seeded mutation targets without expanding their source
ownership. Their replay corpus consists of seed/execution-count descriptors
bound to exact archived source and binaries; these reproduce every generated
input. This is mutation/invariant testing without coverage or sanitizer
instrumentation, and finite fuzzing does not establish absence of bugs. Retain
stderr, usage, stdout and a replay descriptor for every shard; copy failure
records to crashes and block qualification. Do not relabel a short smoke as
release fuzzing.

## D43 — Native-output transport and target claims

The Linux x86-64 CI oracle job records C++/Rust/executed-WASM exact outputs.
Export the same input/native Rust output bytes into a deterministic ZIP with
fixed timestamps and a manifest of each member SHA-256, ZIP SHA-256, source
hashes and counts. CI artifact upload/download transports that corpus to
macOS arm64, Windows x86-64 and Linux arm64. Rust executes the complete corpus
and math probe and compares full bytes, without tolerance or contraction.
C++ stays in the Linux oracle job. Explicit runner host checks prevent an
architecture label from substituting for the requested execution target.

The workflow uses macos-14, windows-2022 and ubuntu-24.04-arm, checked against
GitHub's documented runner labels. Local actionlint is a syntax/static workflow
gate; it is not a remote platform execution pass. Native Linux arm64 C++
qualification required by RFC 5.3 remains distinct from the Rust cross-target
identity matrix and is not inferred from it. Do not advertise remote parity
until actual execution records pass. No publishing or Git action is authorized.

D43 continuation: the Linux arm64 matrix also compiles the pinned scalar-strict
C++ driver and compares every transported input's full output against qualified
Linux x86-64 native Rust. This implements RFC 5.3's native arm64 oracle route
while keeping C++ Linux-only. No remote execution pass is claimed locally.

D40 continuation: strengthen report verification to follow the fuzz record into
every replay descriptor, executable identity, dependency identity and shard
budget/execution sum. Retain the preceding report script and initial Moss
matrix as diagnostic evidence because the source manifest includes this script;
rerun Moss against the final verifier rather than accepting changed harness
sources. No library or fuzz-target code changed.

## D44 — Apply the owner's four-hour release amendment

The owner set the release budget to **4 CPU-hours per target on 2026-10-04**,
RFC amendment §14. This supersedes D42's 24-hour requirement. Accumulated user
plus system CPU time counts without rerunning sound completed shards. Preserve
the pre-amendment record and orchestration script in the continuation baseline.
Require unchanged algorithm sources, dependencies, executable hashes and seed;
an orchestration-only update does not discard that evidence. Record the old and
new budget explicitly. No performance bar or output-equality rule changes.

The repository's five existing targets are stable seeded mutation/invariant
programs, not cargo-fuzz targets. Retain them within this lane's source boundary
and identify that limit in records and README. Their seed and exact execution
count regenerate every input; coverage is unavailable, not zero or measured.
No sanitizer or coverage-guided qualification is claimed.

## D45 — Resume verified measurements and retain background robustness

An interrupted benchmark ZIP lacks its central directory. Recover only complete
local-header entries whose sizes, CRCs and SHA-256 hashes match the saved records;
retain the original ZIP and a recovery manifest. Adopt 197 verified workloads
into an isolated continuation directory because the old process was still
writing after the previous agent turn stopped. Never combine unrecorded partial
entries with completed evidence. Resume only when the rebuilt native executable,
oracle, dependencies, compiler and effective profile match exactly. The only
accepted source differences are the lane-owned shell orchestration. Keep the
old source manifest and per-session CPU selection beside the final identities.
Measure every remaining case and the complete Cargo-default matrix.

Keep the crate's own fat-LTO profile for its existing measurement baseline;
dependents select their own profiles. Moss means, maxima and memory bars remain
1.25, 1.50 and 1.25 respectively. A failed consumer family is reported without
performance edits or altered acceptance thresholds.

Background continuation uses nice 19, an explicit wall budget and a core cap
(default four, maximum eight), with one worker per available physical core.
Resume saved replay ranges and use unused seeds. Finish bounded active chunks
on SIGTERM, persist real process CPU usage, and append a cumulative ledger.
Save failure replay descriptors and logs; chunk timeouts are findings. Emit null
new coverage because the existing targets have no coverage instrumentation.
Weekly/manual CI adds one measured CPU-hour per target and uploads the evidence;
its cache is bound to algorithm sources and compiler. Any finding blocks the
next release. Local workflow lint and fixtures do not claim remote execution.

D40's filesystem limitation persists: the requested artifact directory is absent
and outside the session's writable roots. Retain artifacts at the already
recorded large-volume fallback, separate from the disposable build target.
Exact placement and remote execution cannot be inferred from local gates.

## D46 — Isolate surviving processes and retain timeout replay

The old benchmark process also shares the disposable target's scratch-input
path. It removed the continuation's input after two new cases, stopping the
run. Discard those two timing cases because their input could have raced; keep
the 197 pre-continuation cases whose buffers were verified. Give each record
directory its own scratch input and run immutable archived binaries. Rebuild
and require the same executable hashes before adopting the original cases.
The failed attempt and its logs remain diagnostic artifacts. No library or
measurement-protocol arithmetic changed.

A forced timeout exposes a wrapper defect: without a printed execution counter,
the saved replay descriptor requests zero executions. Save the deterministic
seed with an unbounded execution range instead, mark its counter unknown, and
retain the exact source and executable. Known panic counters still retain the
precise failing prefix. Check both paths with injected failures outside the
release corpus. These wrapper fixtures are not library findings.

D43 continuation: fetch the pinned reference's full history and tags in both
Linux oracle jobs. The checker compares HEAD with v1.3; a depth-one checkout
does not provide that tag. The checker itself is outside this lane's file
ownership. Its conditional currently ignores a failed git diff when the tag
is missing, so CI must supply the tag. Record that checker hardening separately
as an out-of-scope finding; the local oracle has the tag and the required diff
is empty.

## D47 — Require a complete matrix with immutable execution paths

The resumed Moss matrix completes with family GM (maximum): cache 1.182
(1.464), overdraw 1.131 (1.554), plain simplification 1.213 (1.420), attributes
1.211 (1.429), and scale 0.766 (0.869). Its overdraw maximum exceeds 1.50;
memory passes. Preserve the complete failed record and its per-family numbers.

D45's adoption is insufficient for consumer-profile qualification. The old
cases ran binaries from a shared target, and an interrupted default-profile
matrix also existed there. Rebuilding an identical executable afterward does
not prove that every preceding case executed that profile. The surviving
scratch-file race confirms that the old execution paths were shared. Supersede
the performance adoption: retain all old buffers and timing records as
diagnostics, and rerun the complete Moss matrix using immutable archived
executables and a unique scratch path, as D40's continuation requires. This
rerun addresses provenance, with no load admission, load-based repetition,
performance code changes or relaxed bars. The saved fuzz binaries and
source-bound CPU accounting remain valid and their accumulated time counts.

## D48 — Verify retained attachments and distinguish a completed report

The native buffer verifier expects exactly one buffer ZIP in its artifact map.
The lane adapter also retains source and executable attachments there, so pass
only the required buffer entry to that verifier after checking every attachment
SHA-256 in the outer adapter. Require archived executable hashes to match the
recorded build identities. Do not edit the shared Python verifier.

Finish the active immutable Moss matrix before changing the adapter. Preserve
its complete original record, ZIP, sources and executable copies. Rebind the
closed consumer matrices through the verified continuation interface: require
identical executable hashes, effective profiles, compiler, dependencies and
oracle, retain previous identities and record hashes, and permit only the
report adapter change. Reuse all 204 measurements without new timing samples.
Rerun native/WASM parity and the release sweep with the final verifier.

The spec requires the verification report to exit zero when complete and also
requires reporting a failed Moss bar without performance work. Therefore the
report command's exit status describes evidence verification. Missing, changed,
stale, incomplete or incorrect evidence remains nonzero. A fully verified
failed Moss matrix is reported explicitly with release summary passed=false
and moss_performance_accepted=false; it blocks release acceptance but does not
make a complete verification command fail. Benchmark --enforce remains
nonzero for either failed consumer matrix. No acceptance bar changes, no
performance edits, and no release-performance pass is inferred from exit zero.

## D49 — Final lane 4 outcome

The complete immutable Moss matrix has 204 cases. Family mean (maximum)
Rust/C++ ratios are cache 1.243 (1.315), overdraw 1.130 (1.361), plain
simplification 1.258 (1.429), attribute simplification 1.243 (1.356), and
scale 0.782 (0.935). Plain simplification exceeds the 1.25 mean bar;
release-performance acceptance is blocked. All memory bars pass. Cargo
release defaults also complete 204 cases: cache 1.240 (1.310), overdraw
1.063 (1.398), plain simplification 1.284 (1.420), attributes 1.257 (1.651),
and scale 1.030 (1.187). The default simplification means and attribute
maximum fail; memory passes. Keep the own-crate fat profile and all RFC bars.
No performance or library changes are made in this lane.

Final fixture parity has 279 cases and a 65,543-value math probe; the release
sweep has 10,000 cases for each of five functions. Native C++, native Rust and
executed wasm32 have zero mismatches. Each stable mutation target retains
4.101–4.260 process CPU-hours and zero findings. All 306 release shards match
their replay descriptors, execution statistics and measured CPU-usage files.
A background continuation retains those descriptors and appends ten new
chunks with unused seeds; the original release record is unchanged. SIGTERM,
ledger and injected panic/timeout replay checks pass. Coverage remains
unavailable; no cargo-fuzz, sanitizer or coverage-guided acceptance is claimed.

All sixteen build/package gates, both feature modes, MSRV, native/WASM
identity, upstream JS sanity, actionlint and the Done-when verification
commands exit zero. The real Cargo publish dry-run uses D9's offline loopback
registry fallback; it proves packaging/build behavior, not crates.io naming
or registry readiness. The package contains only source, tests and metadata;
its source and release-document bytes match the working tree. It is retained
with its SHA-256 outside the build target.

Delete only /mnt/linux-extra/moss-cargo-targets/codex-meshopt-lane4 after
retaining sources, executables, records and corpora. Verification after that
cleanup exits zero and does not recreate the target. Record complete verified
evidence separately from failed release-performance acceptance. All preexisting
Git metadata remains byte-identical; new metadata belongs to the concurrent
phase/0.2 and phase/0.3 worktrees. No Git state changes are performed here.
The final status contains only lane-owned files and none over 5 MB.

D40's exact artifact-placement blocker remains: use the retained large-volume
fallback /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/lane4 because
the requested /mnt/linux-extra/meshopt-artifacts/lane4 is not writable in this
session. Missing historical buffer archives are explicit and are not current
qualification evidence. Remote macOS, Windows and Linux arm64 execution is
configured, not locally established. The out-of-scope checker finding remains
parity/check-reference.sh:13: a failed git diff with an absent v1.3 tag can be
ignored by its conditional. CI supplies full history and tags; the local
oracle has the tag and the required empty diff. Keep the injected reproduction
and leave that checker outside this lane's file ownership.

## D50 — Prepaid work accounting for plain simplification under thin LTO

Lane 4 measured plain simplification at 1.258 (maximum 1.429) under the Moss
profile, failing the unchanged 1.25 mean bar. Diagnostic Moss-profile builds
(thin LTO, one codegen unit, opt-level 3) on the worst medium sparse case show
Rust/C++ cycles 1.42 and retired instructions 1.52. Per-visit work checks
(`Work::add` compare, branch and subtract) were the largest separable cost. A
throwaway build with the check removed ran 0.933 of the baseline time on seven
paired cases. Bounds checks and memory latency make up most of the rest; the
pinned libm sqrtf (RFC section 5) stays.

Decision: give the simplifier phases a `Meter` parameter. `Work` keeps
visit-by-visit checks. `Prepaid` counts locally and is used only when the
remaining budget covers a proven upper bound of the phase's visits. The phase
then charges its exact total once, or the visited prefix on an error. No
visit can exhaust a covered budget. Errors, consumed work, partial output
writes and exhaustion points are therefore identical to the checked path. The
bounds are adjacency (n + 2 * indices), classification without PERMISSIVE
(edges * (max degree + 1) + 5n), position rescale (n * (1 + weights)), quadrics
(2 * indices + n + attribute visits), pick (index count), perform without
PERMISSIVE (n + candidates * (max degree + 1)), update (n * (1 + attributes) + 2n)
and output filtering (index count). Adjacency now records the maximum degree for
these bounds. PERMISSIVE wedge walks and hash probing have no cheap bound and
keep the checked path. A `debug_assert` checks each bound. The rejected
alternative was per-loop duplicated fast paths in the D35 style: the same
semantics, but eight hand-copied loops. Reversal cost: cheap while unmerged; the
change is local to simplify.rs.

Pick reads one 64-entry rule table, precomputed from CAN/OPP. Kinds are
always below six, so its masked index never changes a lookup. Pick also holds
disjoint table borrows, so pushes no longer force reloads of the vertex-table
bases. On seven paired cases this measured 0.977 against the prepaid-only build.
Rejected after measurement (neutral, 1.007 and 1.012): disjoint borrows in
perform, flips, update and output filtering, and `#[inline]` on Q::triangle,
seam_target and complex_target. Their sources were reverted. LLVM already
inlines the small V/Q helpers under thin LTO. No further cross-crate helper was
on a hot path.

Equivalence evidence (scratch, outside the repository): 168 configurations
(smooth, flat, seam and seam-attribute grids; every option including PERMISSIVE,
LOCK_BORDER and both regularizations; vertex flags; three targets) were compared
against a byte-identical baseline crate. The check covered every work budget
from 0 to required + 1, sampled above 6,000. Results, error bits, usage work and
bytes, and caller destinations were identical in all 649,645 comparisons, with
debug assertions enabled. A unit test checks prepaid and checked phases for
every budget from 0 to 9 and every failure position, using exact, loose and
absent bounds.

Final gates on this source: fmt (root, parity, fuzz) and clippy with -D warnings
(all features, no default features, parity, fuzz). Tests passed for all
features and for no default features, on stable and 1.88, with 49 tests each.
The wasm32 no_std build passed. `parity/run.sh --phase 0.1` passed with zero
mismatches, including executed wasm32 identity and the math probe. So did
`parity/sweep.sh --phase 0.1` with 10,000 cases per function.

The complete 204-case Moss matrix (`--enforce`, exit 0) used CPU 17 (physical
core 8, 7.4% busy at selection), with at least 20 interleaved same-core pairs per
case. One-minute load was 18.5 at the start and 21.4 at the end. Family GM
(maximum), lane 4 -> now: cache 1.243 (1.315) -> 1.176 (1.344), overdraw 1.130
(1.361) -> 1.079 (1.422), plain simplify 1.258 (1.429) -> 1.111 (1.201),
attributes 1.243 (1.356) -> 1.185 (1.335), scale 0.782 (0.935) -> 0.829 (1.292).
The memory bars pass. Plain simplification clears the 1.25 bar by 0.139. Cache,
overdraw and scale code is unchanged. Their maxima moved on tiny cases: scale's
1.292 is tiny/smooth, whose other tiny cases are at or below 0.994. This is
shared-load dispersion, not a regression, and every case stays below 1.50.
Records: results/benchmark-moss.json, with details under
/mnt/linux-extra/meshopt-artifacts/simplify-perf/records.

Not re-established: the Cargo-default matrix, gates.json, fuzz.json and the
package checks are now stale against the changed simplify.rs. The Cargo-default
column in MEASURED_PERFORMANCE.md is lane 4's. The release fuzz CPU budget must
be rerun on this source before release qualification. This lane's spec does not
require them.

## D51 — Release evidence refresh after the simplify optimisation (branch release/0.1-refresh)

Re-established on source `src/simplify.rs` sha256 fc4a3c4b... (d1230c3), with no
source change: the Cargo-default matrix, `gates.json`, `fuzz.json` and the
package checks that D50 marked stale. Build target
`/mnt/linux-extra/moss-cargo-targets/claude-meshopt-release` (to be deleted);
artifacts in `/mnt/linux-extra/meshopt-artifacts/release-0.1`.

Cargo-default matrix (`benchmark.sh --consumer-profile default --enforce`, exit
0, 204 cases, record directory `benchmark-default-release`), family GM (max):
cache 1.189 (1.257), overdraw 1.048 (1.364), plain simplify 1.180 (1.306),
attributes 1.217 (1.310), scale 0.963 (1.142). All mean, maximum and memory
bars pass; D49's default-profile failures (1.284, 1.651) are gone. Run on a
shared host (load 13-42, including the 8 concurrent fuzz workers); high-load
cases keep both attempts in the record.

Release fuzz (`fuzz.sh --cpu-hours-per-target 4 --jobs 8`, seed 20261002,
fresh artifact directory so no earlier corpus is credited): CPU-hours/
executions per target: vertex_cache 4.131/749,426,103, overdraw 4.114/
1,607,030,104, simplify 4.242/770,511,072, simplify_with_attributes 4.111/
617,018,451, simplify_scale 4.242/17,315,143,374. Zero findings; corpus
hashes are in `results/fuzz.json` `corpus_sha256`. Coverage still unavailable.

Gates: `report.sh --execute gates` (all sixteen commands exit 0, including
`cargo package --list` and `cargo publish --dry-run --allow-dirty`, which uses
D9's offline loopback registry fallback; not a crates.io readiness claim).
Run before and after the README update below; the retained record is the second.

Decisions. (1) README performance table and fuzz range were stale (they still
said release performance was blocked); updated to the new records. The Moss
column now shows the D50 numbers and the fat-LTO column is kept as historical.
Alternative rejected: leave README untouched; it would have contradicted the
evidence. Reversal: free. This forced a gates re-run because README is in the
package and the gates source snapshot. (2) `run.json`, `sweep.json`, `js.json`
and `benchmark-moss.json` were not re-run: their recorded source hashes match
the current tree and `report.sh --verify-artifacts` verifies them; their
retained buffers were copied unchanged from
`/mnt/linux-extra/meshopt-artifacts/simplify-perf/records` and lane 4's
`js-final` into the release artifact directory. Alternative rejected: re-run
and re-record identical evidence. Reversal: free. (3) `RELEASE-0.1-CHECKLIST.md`
added at the repository root; it is not part of the crate package or the gates
source snapshot. (4) The first gates run was invoked directly and overwrote the
compact `gates.json` with a full record; it was repaired by re-running through
`report.sh --execute gates` (and the stray `.crate` removed), not by Git.

Unchanged limits: Linux arm64, macOS and Windows execution are CI-configured,
not locally established; historical lane 3 artifacts stay absent as before.

## D52 — Baseline, provenance and artifact placement (0.2; was P02-D1)

Recover the missing Moss source path read-only with git show from binding
commit dc4af42a5e94f8a0f22932c53977f66cd88aadc2. Reuse its MIT index
and byte-group decoder material with explicit attribution. Translate raw
vertex v1 and Quat arithmetic from pinned C++ 1.3; Moss is baseline material,
not the canonical 1.3 filter oracle.

The exact artifact path /mnt/linux-extra/meshopt-artifacts/p02 was denied
with Read-only file system. Retain all evidence at
/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02, outside the
isolated build target that will be deleted. Exact artifact placement remains
an environment residual. No Moss or upstream sources are edited.

Before any candidate timing, reject the provisional baseline because its C++
vector retained allocation between samples. Retain the provisional JSON and
bar in the artifact directory. Repeat with C++ output allocation on every
allocating call and required copies included. Register the corrected numeric
bar before candidate measurement; no candidate results inform this change.

## D53 — Codec API and EXT boundary (0.2; was P02-D2)

Add meshoptimizer_rs::codec with raw allocating and allocation-free caller
buffer decoders, separate header-version inspection, post-filters and a
checked BufferView. Use little-endian byte slices for both index widths; u16
narrowing wraps exactly as the reference does. Raw vertex/index/sequence
versions 0 and 1 are supported. EXT attributes retain vertex v0; the index
helper retains Moss's accepted v0/v1 domain. Reject zero EXT counts, bad
triangle counts, incompatible filters and invalid strides. Parent byteLength
and optional byteStride are checked explicitly with validate_parent; JSON,
resource resolution and loader budgets remain consumer responsibilities.

Raw caller-buffer decoding and filtering allocate no heap scratch. Every
operation takes Workspace for explicit limits and usage. Charge source bytes
plus decoded bytes before raw decoding, and decoded four-byte words before
filtering. Fixed stack blocks and caller buffers are excluded from heap
accounting. Allocating codecs validate minimum stream sizes before reserving
output, use fallible reservation and account actual output capacity. Late
stream/numerical failures may modify the used prefix; tails are preserved.

## D54 — Canonical filters and safe optimization (0.2; was P02-D3)

Follow pinned scalar C++ 1.3 for every meaningful output byte. Preserve Quat's
unscaled arithmetic and original-component rounding sign, including negative
scale words. Zero-length Oct normals return NumericalFailure because the C++
float-to-int conversion is undefined. Exp preserves scalar float-bit
construction and exceptional results instead of introducing clamps. SIMD
Oct/Quat conformance on valid encoded filters is checked separately with the
extension's one-unit allowance. No tolerance applies to canonical bytes.

Reuse Moss index parsing and byte-group material; translate v1 channel
controls, packed 1-bit groups, 16-bit zigzag and rotated 32-bit XOR channels
from upstream. Specialize safe chunk kernels by bit and component width.
Use one checked 24-byte lookahead per packed group and block staging for
contiguous writes. Keep pinned libm and forbid(unsafe_code) in the published
crate. The unpublished byte-export WASM adapter contains no unsafe blocks;
its no_mangle linkage attributes follow the existing parity adapter boundary.

## D55 — Evidence scope and pre-decided performance policy (0.2; was P02-D4)

Register numeric decoder minima from the corrected pinned Moss baseline
before candidate timing. Use 95 percent of throughput corresponding to each
workload's worst of twelve measured Moss samples, stating this conservative
shared-load uncertainty rule. Keep raw samples, dispersion, absolute
throughput and both scalar and SIMD ratios. Do not alter minima using
candidate results. Raw v1, extra encoder levels and the millions-of-triangles
case receive actual measurements and the scalar codec bar; they do not
receive inferred Moss measurements. Report allocating and caller-buffer
results separately. Below 80 percent of optimized C++ throughput selects the
safe scalar implementation as already decided by RFC 6.2; unsafe is excluded.

Run 2,000 seeded cases for each raw decoder, each filter and the checked
view, on both APIs and executed WASM. Export all applicable upstream native
decoder invocations and JS vectors, explicitly await async decoder work,
and run all five unchanged JS suites. Retain every-byte fixture truncations
and malformed statuses without comparing failed partial output. Undefined
Oct conversion is a mandatory Rust robustness case, not a C++ output oracle.

Provide thirteen cargo-fuzz targets, one per public decoder entry point.
Run AddressSanitizer/libFuzzer instrumented 300-second smokes with retained
corpora and source/binary identities. Use stable with RUSTC_BOOTSTRAP=1 only
for cargo-fuzz instrumentation because this environment has no nightly
toolchain; record that profile. These smokes meet the lane brief and do not
establish the separate four-CPU-hour-per-target release gate. Linux x86-64
and executed Node WASM are the local evidence; AArch64 and other native
release qualification remain the coordinator lane's responsibility.

## D56 — Final optimization and measurement correction (0.2; was P02-D5)

The initial candidate misses the filter minima and some raw scalar limits.
Keep every numeric minimum unchanged. Replace the vertex staging copy with
direct writes to checked blocks, specialize index widths and use a checked
five-byte varint lookahead. Add one private codec-only Workspace accounting
method as necessary wiring; existing geometry methods and scratch contents
remain unchanged. A retained-geometry-scratch test checks this accounting.

Use exact last-encoded-record output reuse in Oct/Quat, preserving Oct's
fourth component, rather than changing the square-root backend or floating
arithmetic. All changed records still execute pinned libm and strict scalar
1.3 formulas. Integer-backed component bounds prove the conversions remain
in i32 range; zero Oct normals retain their typed numerical error. The
registered filter timing inputs repeat directions/quaternions, so these
throughput ratios are scoped to that workload. Varied filter performance is
not established by the constant-input bar; varied filter correctness is
covered by the seeded sweeps. No wider float accumulation or unsafe is added.

Observe CPU load before the final measurement and select logical CPU 7
(sibling 6) on the same Ryzen 9 7945HX target. Keep the CPU-0 Moss minima
frozen. Rotate all backend orders and target 40 ms samples, increasing from
12 to 60 pairs when dispersion or gate uncertainty warrants it. Retain all
samples and the CPU-selection observation. No quiet-host claim is made.

Correct standalone C++ filter temporaries to copy-construct instead of
zeroing and then copying. This does not affect the measured Moss baseline
or its numeric minima. Retain the original baseline and a separate paired
Moss/C++ correction before final candidate acceptance. Record the changed
C++ binary identities; do not substitute these diagnostic samples into the
registered bar. C++ checks nearest rounding and gradual underflow on startup.

## D57 — Resolve the measured triangle scalar miss (0.2; was P02-D6)

The frozen full matrix passes every production minimum, with raw scalar
geometric means 1.140406 allocating and 1.062635 caller-buffer. Allocating
triangle v0 streaming/u32 reaches 1.542310, above the unchanged 1.50 limit;
caller-buffer maximum is 1.468908. Preserve the complete failed record,
raw samples, parity/fuzz records and executables under before-triangle-chunks.
All 162 measured canonical output hashes match scalar C++.

Pair a scratch chunk prototype with the unchanged decoder and C++ on CPU 11.
Across eighteen targeted API/case comparisons the median new/old time ratio
is 0.911526; individual results vary. Adopt checked code slices zipped with
six/twelve-byte triangle arrays. Public entry points already provide exactly
count * stride bytes; each triple is now one bounded record. Keep FIFO,
delta, version and u16 truncation semantics unchanged. Repeat parity, the
thirteen smokes and the full acceptance matrix; the prototype is diagnostic,
not an acceptance record. No numeric minimum or ratio limit changes.

## D58 — Final local verdict and cleanup (0.2; was P02-D7)

All local done-when gates pass. Every frozen production minimum passes.
allocating: raw scalar time geometric mean 1.009760, maximum 1.375315.
caller_buffer: raw scalar time geometric mean 0.972634, maximum 1.266216.
Keep safe scalar and publish all SIMD ratios in P02_PERFORMANCE.md.
287 fixtures, 3,637 malformed cases, 14,000 seeded cases and 81 complete
benchmark corpora pass native/WASM identity; all thirteen 300-second ASan
smokes pass with 64,266,473 executions. Preserve the prior failed
record and its raw samples. All final executable copies and source/archive
hashes are verified before deleting only codex-meshopt-p02. Exact artifact
placement remains read-only; use the recorded large-volume fallback.
No Git metadata changes. This local result does not claim the separate
platform/release sweep or four-CPU-hour fuzz qualification.

## D59 — Per-call codec configuration and encoder API (0.4)

Replace meshopt_encodeVertexVersion and meshopt_encodeIndexVersion with
checked per-call values: `VertexEncoding::new(version, level)` and
`IndexEncoding::new(version)`. Defaults are upstream's (vertex v1 level 2,
index v1). Versions above 1 return UnsupportedVersion. Vertex levels 0-9 are
accepted because the pinned reference asserts that range; its documentation
names 0-3, and levels 3-9 produce identical streams in 1.3. Version 0 ignores
the level, as upstream does. Rejected: a global setter (not thread-safe, the
RFC's intentional replacement) and a 0-3 level domain (narrower than the
reference). Reversal cost: free until publication.

Every encoder has an allocating form and an `_into` form returning the used
length. `_into` fails with BufferTooSmall exactly where C++ returns 0,
replicating each reference capacity check in order, including the vertex
encoder's 24-byte group lookahead and the index encoder's 16-byte per-triangle
slack; on failure the destination prefix may change, as in C++. The meshlet
encoder checks the final size first and leaves the destination untouched.
Allocating encoders reserve the bound (vertex count from the largest index for
index encoders, as the upstream JS wrapper does), account it before
reserving, and truncate to the used length without shrinking. Inputs are
exact: `vertices.len() == count * stride`, four floats per filter vector or
stride / 4 for Exp. Index encoders take u32 slices; u16 adapters are not
added. Work charges input bytes plus at most the bound's output bytes;
filter encoders charge input floats plus output words. Reversal cost: cheap
while unpublished.

## D60 — Undefined reference behaviour in 0.4 operations

C++ cases with undefined behaviour are never executed to obtain bytes. The
C++ driver refuses them before the call, and Rust gives a defined, documented
result that is retained as a robustness case:

- Exp encoder: `int(v * 2^-e + 0.5)` is undefined for non-finite input and
  for one-bit mantissas with exponent 128. Rust returns NumericalFailure in
  exactly those cases (the converted value is outside the i32 range).
- Color decoder: a zero alpha word gives an infinite scale, and 16-bit records
  with small alpha can scale outside i32. Rust returns NumericalFailure there,
  matching the Oct zero-vector rule of D54.
- Sequence encoder: baseline selection negates `int(index - last)`; a delta of
  exactly 2^31 is undefined. Pinned GCC builds at -O0 and -O3 both wrap and do
  not switch baselines; Rust keeps that wrapped result (`wrapping_abs`). The
  C++ driver reports such inputs as status -3 (no oracle); the harness records
  them as `cpp-undefined` with native/WASM identity only, and the sweep nudges
  generated sequences away from them.

Rejected: rejecting the sequence case (an invented restriction on valid
inputs) and saturating Exp/Color conversions silently (no reference result).
Reversal cost: cheap while unpublished.

## D61 — Sequence codec losslessness is upstream's

The sequence code stores `(zigzag(delta) << 1) | baseline` in 32 bits, so
bit 31 of large zigzag deltas is lost; upstream behaves identically, and the
port reproduces its bytes. Round-trip losslessness is asserted only when every
index is below 2^30; otherwise the harness requires C++ and Rust decodes of
each other's streams to agree. Vertex and meshlet streams are always checked
for lossless (triangles: per-triangle rotation) round trips.

The triangle codec initializes its edge and vertex FIFOs with 0xffffffff, so
an index equal to u32::MAX can match an empty slot and does not round-trip;
the fuzz smoke found this (retained under `superseded/fuzz-0.4-sentinel-crash`)
and C++ reproduces it byte for byte. Encoded bytes stay exact; the rotation
check is skipped only for lists containing u32::MAX. Rejecting that index
would invent a restriction the reference does not make. Separately, the 0.2
sweep generator produced Quat encodings with 2-3 bits, outside the encoder's
asserted 4-16; it now draws 4-16, since the 0.4 C++ driver refuses
out-of-assertion parameters (D60).

## D62 — Color stays outside the EXT helper

`decode_filter_color` is a raw filter. `Filter` and `BufferView` keep the EXT
minimum (None/Oct/Quat/Exp), as RFC 3 states; accepting COLOR there would
claim EXT content that EXT_meshopt_compression does not define. SIMD Color
output is recorded as a distance only (RFC 5.1: no assumed cross-ISA identity).

## D63 — Meshlet codec dependency on 0.3

0.3 is not merged to main: its layout was read from `phase/0.3` in the p03
worktree, read-only. That layout stores vertex references as `u32` and local
triangles as three `u8` per triangle (`Meshlets::vertices` / `triangles`, with
`Meshlet` offsets). The codec module (`src/codec/meshlet.rs`) depends only on
those slice shapes, not on 0.3 types, so integration is mechanical: no
format, type or re-export changes are needed; a later convenience taking a
`Meshlet` descriptor can slice `Meshlets` directly. 0.3 accepts up to 512
triangles per meshlet; the codec, like upstream, accepts at most 256 and
returns InvalidParameter beyond. Decoders write exactly count * size bytes
(upstream SIMD paths may also write alignment padding); `decode_meshlet_raw`
needs no 16-byte padding. Reversal cost: free; the dependency is recorded for
the 0.3/0.4 merge.

## D64 — 0.4 evidence scope

`parity/report.sh --execute run|sweep --phase 0.4` runs `parity/codec/runner04.py`.
The run re-executes all 287 0.2 decoder fixtures and 0.2 malformed cases, plus
563 native 0.4 invocations captured from the unchanged upstream tests (global
versions and caller capacities included) and 19 JS vectors (17 shipped-WASM
encoder outputs kept as `.expected`, 2 COLOR decoder vectors). Malformed
cases add every-byte prefix/suffix truncations, bit flips, count mismatches
and trailing bytes for each decodable meshlet fixture, every capacity around
each encoder fixture's size, zero-alpha and overflow Color records, and
parameters outside each reference assertion. The sweep runs 2,000 seeded cases
for each 0.2 operation and each of the fifteen 0.4 operations (encoders with
every version/level, real-like attribute streams, block-boundary counts,
explicit capacities, filter encoders over all bit widths and modes, Color
decoding of encoded and random words, meshlet encode/decode including
corrupted streams, and all four bound functions). Every case runs both APIs on
native and executed WASM; encoder outputs are cross-decoded in both
directions. Per-case data stays under MESHOPT_ARTIFACTS; git holds summaries.

Nineteen new cargo-fuzz targets (one per new entry point; filter encoders'
`_into` forms share one target) run 300-second ASan smokes; encoder targets
assert round trips. The four-CPU-hour release gate is not claimed.

## D65 — 0.4 performance protocol

The RFC 6.1 raw scalar codec bar applies unchanged per family and API: Rust /
scalar-strict C++ time ratio geometric mean <= 1.25, no case > 1.50. There is
no Moss baseline for these operations, so no minima are registered; SIMD C++
ratios are reported. Both consumer profiles of D41 are measured on identical
inputs. The C++ driver allocates fresh output for allocating calls and reuses
it for caller-buffer calls (D52's rule); input conversion and validation are
outside the timed region on both sides, and meshlet outputs are serialized
after timing. Every timed request is first checked for identical Rust/C++
output.

## D66 — 0.4 local verdict and cleanup

The final safe-Rust meshlet decoder avoids a copied vertex window and repeated
stream/count checks in the allocating path. It inlines the public decode entry
points across the crate boundary and the vertex loop, while leaving the larger
triangle loop out of line. Empty-workspace codec accounting takes a checked
fast path; allocating typed meshlet decode recounts capacity only when the
allocator returns more than requested. These changes preserve the checked
limits and output contracts. Earlier failed benchmark records remain under
`/mnt/linux-extra/meshopt-artifacts/p04/superseded/`; they are not acceptance
records.

Final-source gates all exit 0: three fmt checks, three clippy checks with
`-D warnings`, parity-driver and all/no-default-features tests, and the
wasm32 build. The scalar-strict run matches all 869 fixtures and 7,653
malformed cases; the seeded sweep matches all 44,000 cases (2,000 for each
of operations 1-7 and 11-25), including cross-decode and executed WASM
identity. Nineteen ASan cargo-fuzz targets pass 300-second smokes with
110,927,795 combined executions. These smokes do not establish the separate
four-CPU-hour-per-target release fuzz gate.

Both final benchmark profiles, run sequentially on CPU 3 with the same input
manifest SHA-256 `5b49549e3b6ab5021eeb12c313f244c24e1cb338b937450c7528af4716847e39`,
pass all twelve families for allocating and caller-buffer APIs under D65's
unchanged 1.25 geometric-mean and 1.50 per-case limits. The largest family
geometric mean is 1.239 (default, allocating sequence encode); the largest
single-case ratio is 1.457 (Moss, allocating Quat encode). The small
3-vertex/1-triangle typed meshlet decode now measures 1.165/1.185
(Moss allocating/caller-buffer) and 1.351/1.164 (Cargo defaults). The
rewritten bounds family measures geometric mean/maximum 0.684/1.155 (Moss)
and 0.680/1.150 (defaults); bounds have one API form. Exact per-family,
per-case ratios and raw samples are in the two `benchmark-0.4` records and
the slim `parity/results/benchmark-0.4-*.json` summaries. The measurements
are local Linux x86-64 evidence on a loaded host, not a release-platform or
Moss integration qualification.

`parity/report.sh --phase 0.4 --verify-artifacts` exits 0 and verifies all
recorded hashes and case buffers against the final sources. The generated
target directory is deleted afterward; detailed evidence remains under
`/mnt/linux-extra/meshopt-artifacts/p04` at 752 MB, below the 3 GB lane cap.
No Git metadata operations were performed. D63 records the unmerged 0.3
dependency: the isolated codec accepts its `u32` vertex and
three-`u8` triangle slices, with 256/256 codec limits, so later type
integration is mechanical.

## D67 — Phase 0.3 surface and packed layout

Implement all 15 meshlet, bounds, extraction, optimization, partition and spatial
function names listed in the additive coverage inventory. Preserve 1.3's packed
triangle offsets with no four-byte padding. Preserve arbitrary cluster index
lists in partitioning, old-to-new sort remaps, new-to-old point-cluster order,
optimization levels 0–9, unsigned byte wrapping and unchanged unused vertices.
Expose allocating and caller-buffer variants for buffer outputs, and explicitly
atomic destructive optimization/spatial-sort variants. Scalar helpers return
values without an artificial allocating counterpart.

Keep changes local to new modules/tests and unpublished p03 harness/fuzz
packages. Cargo's feature declaration, lib.rs module/reexport wiring, the phase
0.3 dispatcher and additive Workspace accounting hooks are necessary wiring;
no preexisting algorithm, shared math implementation or phase 0.1 harness path
is changed. Own Rust contains no unsafe blocks and the core still forbids
unsafe. C++ is only an external unpublished oracle. The original 0.3
implementation made no Git state change; D83 records its later integration.

## D68 — Resource and numerical boundaries

Use the existing Workspace limits: 1 GiB of owned output plus simultaneously
live scratch/retained capacity and 2^34 counted work. All owned storage reserves
fallibly, checks size arithmetic and accounts actual capacity. Release temporary
reservations at their lifetime boundaries. Child demo operations subtract live
parent storage/work from their limits, report child peaks to the parent, and
retain only actual returned output capacity. Fixed bounded stack arrays and
caller-owned destinations are excluded from heap-byte counts on both sides.
Public owned result types do not derive hidden allocating Clone.

Work units are record visits: full position and global-index validation;
adjacency initialization/insertion; meshlet candidate, seed, tree and emission
visits; bounds normal/extreme/enclosure/cone visits; extraction indices and
collision searches; optimization valence, candidates, moves, rotations and
remaps; partition adjacency/heap/merge visits; spatial key, radix and split
visits. Fixed parameter/layout checks and bounded local-byte validation are not
separate work units. Batched scans/searches preserve the short-budget prefix,
failing visit and side effects, and charge only an early search's visited prefix.
Tests check exact successful limits, one-unit/byte reductions, reused workspace
storage, and parent/callback accounting.

Reject nonfinite supplied geometry, including unused vertices, and invalid
radii/weights/settings. Finite extreme geometry retains intermediate overflow
checks and can return NumericalFailure. Within the validated |coordinate| and
radius <=1e8 envelope, fixed-order intermediate arithmetic is finite; specialize
those kernels to remove redundant checks. Spatial quantization's validated
finite extent/scale bounds each key coordinate by 65535 plus rounding. Preserve
signed-zero and stable comparison/tie behavior. SAH cost infinity/NaN in the
upstream deep fixture is internal ordering data: its upstream fallback is
supported, so do not reject it as output geometry overflow.

Caller-buffer builders document possible partial prefixes after late errors.
Optimization, extraction and destructive spatial sorting commit only on
success. Callback demo output and position dilation can precede a late failure;
this is documented and is distinct from atomic meshlet optimization.

## D69 — Math, safe optimization and rejected experiments

D19/D27's exact pinned generic libm 0.2.16 backend remains unchanged for std,
no_std, native and wasm32. Cache complete sqrt operand bits in a per-call
64-slot table, expanded to 512 for repeated builder/partition operands. A cache
miss uses the same software backend. No operand approximation, reassociation,
fast-math or optional ISA/unsafe kernel is introduced.

For moderate inputs, conservative lower-distance bounds skip candidates that
cannot win; values below 1e-18 use the unmodified squared-distance path to
avoid subnormal-square rounding. Sphere interior skips use the preceding
representable radius-minus-input-radius before squaring, and only above that
threshold. Force-inline typed sphere readers, use upstream-width extreme
indices, specialize fixed bounds stack sizes, copy initialized allocating
outputs directly, ping-pong radix buffers, and use four temporary box lanes
without padding persistent 24-byte boxes. Work accounting is batched only
where the same visited-prefix semantics are retained.

An experiment enabling libm's arch feature was reverted after rechecking D19;
it is superseded diagnostic evidence and never qualifies the final source.
An external safe integer-square-root experiment matched one million generated
positive input bits but took 2.731 seconds versus libm's 1.270 seconds in its
loop. It is not integrated and is not exhaustive backend qualification.
A historical hardware bounds capture had lost samples; it diagnosed point
reader overhead but does not establish a universal safe-Rust performance
ceiling or final-source timing. Acceptance uses the complete paired matrix.

## D70 — Optional clusterlod and local support snapshot

Port the complete pinned demo/clusterlod.h behind off-by-default `clusterlod`,
using core+alloc and no C++ dependency. Preserve its presets, weld hash including
signed zero, boundary classification/dilation, simplification/fallback paths,
attribute/flag/protect behavior, accumulated error, callback identifiers, cluster
optimization, bounds and hierarchy forest. Mutable packed positions make the
demo's dilation visible. Callback outputs may use nonsequential IDs; allocating
build owns and accounts outputs while callback build releases transient output
storage after the callback.

The demo needs later-lane sparse/sloppy/fold/error-clamped simplification that
this checkout's stable API does not expose. Reuse a private reviewed p01x
support snapshot, with original hashes in parity/p03/support-provenance.json,
and redirect paths inside that private module. It adds no stable simplifier
API and does not edit the parallel lane or this checkout's existing simplifier.
Existing MIT notices cover the upstream translation; private support is safe
Rust and uses the same pinned libm. Restrict simplify_ratio and the stopping
threshold to [0,1), ensuring progress instead of an unsupported infinite demo
loop. Document that output matches the demo rather than a stable library
contract. No meshlet codec or Moss integration is added.

## D71 — Evidence, timing and artifact placement

Use scalar-strict unmodified C++ 1.3 as the exact oracle, and execute identical
meaningful serialization through native Rust and Node wasm32 Rust. The record
contains source, executable, corpus, compiler, FP and runtime identities. Check
the named 47 upstream fixture files. Archive all sweep inputs and scalar output
bytes in a length-delimited corpus after every implementation agrees exactly.
The optional demo compares 80 fixtures including every meaningful group,
cluster, mutated position and hierarchy field.

Run 15 separate cargo-fuzz targets for at least 300 elapsed seconds apiece with
coverage and trace compares, libfuzzer-sys 0.4.13/cargo-fuzz 0.13.2, sanitizer
none and stable RUSTC_BOOTSTRAP. A prior CARGO_ENCODED_RUSTFLAGS empty value
suppressed cargo-fuzz's injected instrumentation: supersede that run, remove
the override, and require loaded inline counters and executed units in every
final log. Extend extraction to high global IDs/collisions, radius fuzzing to
arbitrary bits and the capacity helper to usize overflow boundaries. This is
the lane smoke, not the four-CPU-hour release gate.

The full benchmark has 624 rows: 15 functions, allocating/into forms for buffer
outputs, three sizes and four geometry kinds, scalar and SIMD-enabled C++.
Bounded cluster utilities use their legal maximum (512 triangles/256 vertices)
instead of passing million-triangle invalid inputs. Both resident drivers use
one measured least-busy allowed physical core, with 20 alternating paired
samples, increased to 30 when ratio quartiles cross a bar. Record loads,
dispersion, all raw times and requested heap usage. Validate, allocate and copy
inside timing; exclude serialization/I/O/builds. Black-box real Rust outputs;
do not add a second black-box on the empty transport Vec (C++ drops that Vec).
No quiet-load qualification, bar relaxation or optimized-output identity claim
is inferred from exact scalar parity. --enforce fails incomplete/quick/selected
matrices and failed family timing/memory verdicts.

The mandated /mnt/linux-extra/meshopt-artifacts/p03 lies outside the exposed
writable roots; its mkdir returns Read-only file system. Keep evidence on the
writable large volume at /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p03,
separate from the exact isolated build target. Record this filesystem residual.
Preserve final source/binaries and verify their hashes before deleting
/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p03. Final gate status will be
appended only from completed records, with any remaining acceptance failures
stated explicitly.

## D72 — Demo oracle boundaries and candidate profiling

The extended demo fixtures reached a pinned debug assertion that evaluates
`1u << 32` for 32-component attributes. That shift has no defined C++ value.
Compile only the demo header implementation with NDEBUG, restoring assertions
after its inclusion; the original library and unchanged native fixture bodies
remain separate translation units with assertions enabled. The release demo
body's attribute/protect loops use valid shifts 0–31. Keep width-32 fixtures and
exact output comparison; this is an explicit oracle configuration, not an
upstream source edit or a Rust-domain restriction.

The C++ demo also takes `&vector[0]` on empty storage. Avoid executing that
undefined boundary. The 80th differential demo fixture is a nonempty small mesh;
empty demo build/hierarchy behavior is Rust-only focused boundary evidence.
This corrects the earlier inventory's broad empty-input comparison wording.
All 80 nonempty extended demo fixtures passed after this oracle correction.

A same-input hardware capture on the pre-candidate builder has zero lost
samples in both binaries. Its four-event stat gives Rust 28.324 billion versus
C++ 12.834 billion retired instructions. The sampled Rust candidate visitor
accounts for 48.45% of cycles, execute for 32.77%, KD build for 6.29% and software
sqrt for 5.76%. Retain source identities, binaries, framed input and raw captures.
This diagnoses that candidate; it is not a final-source or universal ceiling.

Replace the outlined visitor with explicitly inline typed candidate methods
and direct three-index topology arithmetic. Keep every priority, ordering,
score operand and failing work-prefix rule. Remove redundant squared-distance
checks only in the proven moderate envelope. Reject KD capacities that cannot
fit the packed 30-bit child metadata before allocating tree storage. The
selected same-core diagnostic improves medium builder ratios from about 1.8
to about 1.6–1.7 and tiny ratios from about 2.4 to about 2.1; it still fails the
bar and is not a complete timing gate. Next evaluate lazy initialization of
the small exact memo, using 64 slots for builders with at most 128 triangles
and 512 for larger builders. No safety or performance bar is relaxed.

## D73 — Final resource preflight and local storage choice

Initialize the small 64-slot exact memo only on its first square root; non-math
functions avoid zeroing an unused table. Builders with at most 128 triangles
use that table; larger builders and partitioning use 512 slots. A targeted
clippy allowance keeps the explicit lazy initialization of the 512-byte array.
Both table sizes have full-bit collision/special-value/backend regressions.
The inherited root math module and backend configuration remain unchanged.

Specialize the full greedy builder and nearest lookup by the validated moderate
envelope, retaining the checked fallback for larger finite inputs. Inline cone
normalization and batch adjacency-removal searches without changing their
visited prefix. Store live-count and local-index fields together in an eight-byte
VertexState to share index checks and one allocation. Including offsets, this
vertex storage is 12 bytes versus C++'s 10, hence at most 1.20 of that component;
all requested-heap ratios still require the unchanged full-matrix 1.25 gate.
Scan/spatial builders keep their existing compact local-index storage.

A review found that preflight parameter/capacity errors could retain a previous
call's Workspace measurements. Start the new operations' accounting context
before those checks, including bounds dispatch and demo hierarchy bounds. A
reused-workspace regression covers fourteen early-error paths and the feature
hierarchy path. Fresh failures now report this call's zero work and retained
storage rather than stale output/work. Success arithmetic/work counts and
atomic output behavior are unchanged. The preceding timing attempts are
superseded/incomplete diagnostics, not final gates.

Add the new integration test file to Cargo's package include list. The crate
version and earlier public algorithms are unchanged because other milestones
are concurrent. Freeze the final implementation and rerun all lane gates.

## D74 — Packed-field overflow and minimum paired timing

Replace the conservative tree-capacity cutoff with checks at the actual 30-bit
leaf-count and relative-child-offset writes. A tree can reserve more nodes than
it uses, so rejecting its entire capacity could reject representable inputs
when callers raise limits. Check the stored values, preserve metadata on error,
and retain checked usize allocation sizing. A boundary regression verifies the
largest packed value and the first overflowing value without giant allocation.
This supersedes D72's preliminary capacity cutoff.

Use the RFC's minimum ten alternating paired samples, increasing to twenty and
then thirty whenever ratio quartiles cross either timing bar. This supersedes
D71's unconditional twenty-pair choice; D39 reported the older phase 0.1 run,
not a stronger p03 minimum. The 624-row coverage, same-core residency, input and
output retention, dispersion, shared-load qualification and numeric bars remain
unchanged. Earlier twenty-pair partial runs stay diagnostic and are superseded.
The extra pairs are retained for uncertain cases rather than required for every
unambiguous failure. Record this choice before the final complete run.

## D75 — Final diagnostics and sweep admission

The public phase 0.3 sweep dispatcher retains the existing harness's minimum
2,000 cases per function. Reject explicit zero or 1,999-case requests before
building or comparing anything. Keep the direct unpublished transport useful
for diagnostics; its records require the independent complete-inventory
verifier before they can establish the spec's sweep gate. Positive 2,000-case
execution and both negative controls are retained.

An isolated artifact copy evaluated per-selection candidate generation marks,
which skip repeated scoring without changing the priority/score tie rule. The
smooth-only diagnostic improves tiny builder ratios to roughly 1.6 and medium
ratios to roughly 1.3–1.4, but still fails the timing bar and adds heap storage.
Prepaying an entire selection's work scan was slower; simplifying the distance
memo lookup did not resolve the bar either. None is adopted or described as
fully qualified. The full source-frozen matrix continues on the tested library.

A final-source tiny smooth builder capture retains both binaries, framed input,
counter stats and cycle samples with zero lost samples. One million operations
retire 85.727 billion Rust versus 48.073 billion C++ instructions. Software sqrt
accounts for 18.87% of sampled Rust cycles and the inlined execute body 62.13%.
This is a fixed-input diagnostic on a separate CPU, not timing acceptance or a
proof that further safe optimization is impossible. Preserve the scalar-strict
math backend and the original numeric bars.

Retain checksum-pinned dependency package archives and their verified source
files. The libm source hashes also match the executed earlier-phase regression
record. Keep owned package caching under this lane's target; the artifact copies
remain after target cleanup. Existing shared ledgers receive append-only phase
0.3 sections and the new summary uses its own filename.

## D76 — Preserve child work totals on 32-bit targets

The final review found two optional-demo parent charges narrowing a child's
u64 work total to usize. Valid totals above u32::MAX could therefore fail with
SizeOverflow on WASM despite an adequate u64 work budget. Add a feature-local
u64 Work charge with the existing limit/overflow behavior; use it for child and
external support-operation accounting. Keep the earlier usize record-visit API
and all geometry arithmetic unchanged.

A synthetic producer charges u32::MAX plus 18 work units without giant geometry
allocation, then checks both child and external totals and a one-less budget.
The old artifact-copy test traps in this regression on executed wasm32; the
patched copy and final worktree's eleven executed WASM unit tests pass. Retain
both old/new probe binaries, sources and execution records. The all-feature and
no-default-plus-clusterlod native suites now have 66 tests; no-default has 60.
Rerun all functional, fixture/sweep/demo and fifteen instrumented fuzz gates.

The completed 624-row benchmark predates this source-only portability correction.
The rebuilt final native driver is byte-for-byte identical to the measured
driver, so the native timing matrix remains applicable. Require that executable
identity in the verifier. Retain both prior changed source files by hash;
current functional/WASM/fuzz records capture the corrected source. Canonical
math, packed layout, working limits and performance bars remain unchanged.

## D77 — Final verdict and cleanup

All fifteen named APIs and the optional full clusterlod demo are implemented.
The final eleven build gates, MSRV feature-mode tests, 697 focused messages,
52,000 sweep messages, 80 demo fixtures, eleven executed WASM unit tests and
fifteen instrumented smokes pass. Final smokes complete 52,334,430 executions
and at least 309.11 elapsed seconds per target. The independent evidence
verifier checks complete inventories, source/dependency/executable/input/output
hashes, both timing baselines and their arithmetic and exits zero.

SPEC acceptance is **FAIL on the performance bar**, not Done-when. The unchanged
enforced complete 624-row benchmark exits 1. Each baseline has eight failed
families; nine distinct families fail at least one baseline. Seven pass each
baseline and six pass both. Every requested-heap ratio passes; the maximum is
1.166658. The full per-family table is appended to MEASURED_PERFORMANCE.md and
the separate machine-readable record is MEASURED_P03.json. Do not infer a
universal safe-Rust ceiling from this shared-machine matrix or its diagnostic
profiles. The remaining acceptance work is reducing the failed timing ratios
while retaining exact parity, checked execution and the unchanged bars.

Preserve all final corpus/output records, exact executables, verified package
archives/sources, specification/RFC, source archive, prior changed source
versions, command logs, profile captures and hashes under D71's writable
artifact root. All 238 archive manifest files were checked before cleanup and
no target executable remained active. Delete the exact required target
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p03`; deletion completes and
cleanup.json records it. At that checkpoint no Git mutation had been performed.
HEAD was `8ec3b3d0ca78caedd445dfb827a9579e2b8d7fef`; D83 records the later rebase.

## D78 — Phase 0.3 performance: profiles, pairing and driver timing

The bar of RFC §6.1 is measured for two builds of the unpublished driver: the
Moss-like consumer profile (thin LTO, `codegen-units = 1`, `opt-level = 3`) and
Cargo's release defaults (`opt-level = 3`, 16 codegen units, thin-local LTO).
They are `[profile.consumer]` and `[profile.release-defaults]` in
`parity/p03/Cargo.toml` (Cargo reserves the `cargo-` prefix);
`MESHOPT_RUST_PROFILE` selects one, the identity records it, and each writes
its own `benchmark-<profile>` record. The library's root profile is unchanged:
it does not control consumers. Every case now uses at least twenty alternating
same-core pairs, thirty when the quartiles straddle a bar. Core selection,
load recording and the bars are unchanged.

Two driver-only corrections remove costs that are not the operation. The Rust
driver's dispatch closure returned its transport `Vec` by value; in the timed
loop the returned value was rebuilt from three separate stores and reread as
one wide load, a store-forwarding stall that took 68% of the cycles of the
20 ns `build_meshlets_bound` repeat. The closure now fills a caller-owned
buffer, like the C++ lambda's in-place return. For `build_meshlets_bound`
itself, both drivers time the bare call with inputs and result opaque to the
optimizer (an empty `asm volatile` constraint in C++, `black_box` in Rust)
instead of the dispatch wrapper, which dominated the measurement. Outputs,
validation and the timed regions of every other operation are unchanged.

The "before" columns are the incomplete new-profile run of the unchanged
source (178 consumer and 138 release-defaults rows: the three flexible and
scan builders and part of the spatial builder) together with the fat-LTO
624-row matrix of D77. The run was stopped to free cores: the shared machine
ran at load 20–45 with cores clocked near 600 MHz, and a full pass of one
profile takes most of a day there. Artifacts are under
`/mnt/linux-extra/meshopt-artifacts/p03/perf`; the build target is
`/mnt/linux-extra/moss-cargo-targets/claude-meshopt-p03perf`.

Reversal cost: free (driver and measurement only).

## D79 — Exact square roots without the scalar backend's cost

Every 0.3 root still equals the pinned `libm::sqrtf` bit for bit; two
mechanisms reduce how often and how slowly it runs.

`math::sqrt8` returns eight independent roots. When all eight operands are
positive, normal and finite, each lane uses exact IEEE f64 operations only (a
halved-exponent estimate, three Newton steps from above, rounding to f32, and
a one-ulp correction decided by exact f64 squares of the two rounding
midpoints), which the compiler evaluates in vector registers; any other
operand sends the chunk to `libm::sqrtf`. Over all 2^32 operand bit patterns
it matches `libm::sqrtf` with zero mismatches (`exhaustive-math.txt` in the
artifacts, binding the hash of `src/math.rs`). Throughput is 14 cycles per
root against 35 for the scalar backend. It takes the triangle areas of the
cluster/meshlet bounds and of the flexible builder's cone setup, and the seven
or three axis extents of the bounding sphere, whose selected extent root is
reused instead of being taken again.

`math::RootEstimate` bounds a root without computing it: a 256-entry table of
bounds over 1/128-wide mantissa buckets (0.4% wide, no division) and one
Newton step from the halved-exponent estimate (0.2% wide). A correctly rounded
root cannot cross a representable bound that encloses the real root, so a
comparison decided by these bounds equals the exact comparison. The same
exhaustive run checks both bound pairs for every in-range operand.
`root_at_least` uses them to skip kd-tree leaf, initial-seed and seed
candidates that cannot win, before any root is taken.

Rejected, with measurements retained: replacing the scalar backend by an f64
Newton root (same 35-cycle throughput, latency 80 instead of 66 cycles) or a
table-seeded one (no gain); libm's architecture feature (D19); SSE value
intrinsics, which need `#[target_feature]` contexts that are `unsafe` to enter
at MSRV 1.88; `_mm_prefetch`, which takes a raw pointer.

Reversal cost: cheap (two internal helpers).

## D80 — Neighbor search of the flexible builder

`getNeighborTriangle` dominates `build_meshlets` and `build_meshlets_flex`.
The selection is unchanged: lowest priority, then strictly lowest score, the
first visited winning ties, NaN scores never replacing an equal priority.

In bounded mode — moderate inputs, a positive normal radius with finite
reciprocal bounds — every score operand is finite, the radius and cone factor
are positive and `0 <= weight <= 1`. Each rounded operation of the score is
then monotone in each input: nondecreasing in the distance and in the cone
factor, the cone factor nonincreasing in the spread, and each rounded product
of the spread monotone in the meshlet-normal component with the sign of the
candidate's component. Scores computed from bounding inputs therefore bound
the exact upstream score. Candidates carry such bounds: the root bounds of
D79 for the distance, reciprocals of `1 / radius` widened past their rounding,
and bounds of the meshlet normal from root bounds of its squared length (the
normal's exact root is taken only on demand). A comparison decided by disjoint
bounds equals the exact comparison; otherwise both exact scores are computed.

In bounded mode on meshes of at least 256 triangles, each live triangle
adjacent to the current meshlet is listed once, when its first vertex joins
the meshlet, and emitted ones are dropped lazily. The scan visits the same
triangles but each only once. With finite scores the selection is the minimum
of (priority, score, visit order), so the order of evaluation does not matter
provided exact ties are broken by the upstream visit order: the earliest
meshlet vertex of the candidate by local index, then its position in that
vertex's current live list. Bound tests therefore reject only strict losses
in this mode. The per-vertex scan remains for smaller meshes, unbounded or
non-moderate inputs, a list beyond its 1024 entries, and budgets that do not
cover the search. Its repeat visits of one triangle through several meshlet
vertices are skipped: under the strict lexicographic selection they cannot
change the choice. The scan walks only meshlet vertices with live triangles.

Work accounting is unchanged. A search is charged the sum of the live list
lengths of the meshlet's vertices, once, when the budget covers it; otherwise
the per-vertex path keeps each per-visit exhaustion point. The removal search,
the kd-tree leaf loop and the adjacency scans likewise charge a covered visited
prefix once. The kd tree computes both children's statistics in one loop (two
independent Welford chains sharing their running weight) while each child
still charges its work and checks its mean where the recursion did. The
packed vertex record holds the adjacency offset, live count and local index
(12 bytes, as the two arrays it replaces); `emitted` gains a "listed" state.
`build_meshlets_bound` divides by a reciprocal table below 2^22, exact there.

Rejected after measurement: a dense i16 local-index array (no gain),
deferred batched exact roots for every candidate (slower), out-of-line
scoring, index clamping instead of bounds checks.

Reversal cost: cheap while unmerged; each part is internal.

## D81 — Spatial builder, radix sort, partition, extraction and bounds

The spatial builder pads boxes to four lanes (lane 3 unused), stores
`radixFloat` keys as u32 and frees them after sorting, so peak storage is
unchanged. Its area sweep and SAH pivot use local slices; the pivot is
specialized on its loop-invariant choices, and where every divisibility test
is provably true (`2 * min <= max`, step one) it computes eight base costs at
a time before visiting them in order. The stable radix sort builds all pass
histograms in one sweep and runs their prefix sums as independent chains, as
upstream's `computeHistogram`; it also serves the spatial sorts.

Partition skips finiteness checks that cannot fail for moderate coordinates
(|x| <= 1e8 bounds every partial sum and squared offset) and charges covered
adjacency visits once. Index extraction sizes its stack scratch to the input
tier. Bounds group their independent roots (D79); every failure there is
`NumericalFailure` and work is charged first, so the grouped order of checks
is unobservable.

Rejected after measurement: flat box arrays, lane-3 duplication or liveness
tricks to coax wider vector code, and software touch-ahead loads in place of
prefetch.

Reversal cost: cheap (internal).

## D82 — Phase 0.3 performance baseline and final qualification

The coordinator resolves the 0.3 baseline ambiguity in favor of the scalar
C++ build (`MESHOPTIMIZER_NO_SIMD`), as for 0.2. Both the Moss-like consumer
profile and Cargo release defaults must meet the unchanged RFC §6.1 family
bars against that build: geometric mean at most 1.25, every case at most 1.50,
and requested output plus scratch at most 1.25. The upstream SIMD C++ ratios
remain measured and published, but they do not gate 0.3. SIMD clusterizer
acceptance would require a separate later amendment. The benchmark's enforced
verdict now checks the scalar family verdicts; it continues to record both
C++ baselines, raw paired samples, load, requested heap and exact outputs.

The final 0.3 source additionally inlines neighbor search and active-list sync
for Cargo defaults and defers meshlet-normal setup until a neighbor search is
needed. The first seed never searches neighbors. The focused tiny-input
comparison showed lower paired ratios for both builders under Cargo defaults;
the complete matrix below determines acceptance.

The final source passes fmt, both clippy modes with warnings denied, both test
modes (including MSRV 1.88), no-default wasm32 build, the 0.3 fixture/WASM run,
the 2,000-case-per-family sweep, and all 80 `clusterlod` cases. Every comparison
has zero mismatches. The enforced matrices ran sequentially with 624 rows and
at least twenty interleaved same-core pairs per case in each profile; both
records say `complete=true` and `source_unchanged=true`. Consumer exits 0 and
passes all fifteen scalar families. Cargo defaults exits 1: fourteen scalar
families pass, while `partition_clusters` reaches geometric mean **1.450** and
maximum **1.778** (medium seams, caller buffer), above the unchanged 1.25/1.50
bar. Its requested heap ratio is 1.000; the maximum over all families is
1.167. The formerly failing tiny `build_meshlets` and `build_meshlets_flex`
cases now peak at 1.405 and 1.423 under Cargo defaults. The best complete
default-profile partition result on this final source is 1.450/1.778; no
release qualification is claimed.

The exact default-profile driver and scalar C++ binary were profiled on the
medium seams caller-buffer case. Per framed request, `perf stat` measured
6.571G versus 4.974G retired instructions (Rust/C++ 1.321), 1.949G versus
1.133G cycles (1.720), and 1.577G versus 0.862G branches (1.829). The tiny
seams case likewise took 7.934G versus 5.953G instructions (1.333) and
2.076G versus 1.317G cycles (1.576). The exact Rust binary's sampled cycles
are 46.3% `adjacent` and 42.8% `partition`; a line-table build of the same
source locates 15.1% at centroid accumulation, 8.9% at adjacency deduplication
and 5.9% at initial vertex filtering. These are broad costs rather than one
isolated call overhead. The recordings and binary hashes are under
`/mnt/linux-extra/meshopt-artifacts/p03/perf/final4`. This documents the
residual without asserting a universal safe-Rust limit or changing the bar.

The separately published SIMD comparison fails for spatial meshlets in both
profiles (consumer 1.225/1.616; defaults 1.195/1.706). Defaults partition also
measures 1.449/1.598 against SIMD. Under the coordinator's decision these do
not add 0.3 gates. The scalar defaults partition failure alone blocks the
phase's performance acceptance. Reversal cost: cheap while unmerged.

## D83 — Integrate phase 0.3 after 0.2 and 0.4

Rebase the single 0.3 commit onto main in this worktree. Retain main's script
and slim-record behavior, and route 0.3 run/sweep through `report.sh`. Keep the
0.3 per-message corpus outside Git under
`/mnt/linux-extra/meshopt-artifacts/p03`; commit only hashed summaries. Preserve
main's D1–D66 and renumber the 0.3 D40–D55 as D67–D82, including references
in 0.3 code comments and records.

The 0.4 meshlet codec continues to accept plain vertex and triangle slices.
The new 0.3 layout types could support a typed convenience wrapper, but this
integration does not add one. All-feature tests compile and exercise the codec
with the combined layout.

The earlier phase 0.3 performance verdict in D82 remains historical; this
integration changes no algorithm or performance bar. Phase 0.1's release report
remains stale until its final re-qualification, even though the 0.1 parity run
and sweep were repeated here. The combined-tree gate outcomes are recorded in
the slim summaries and the final integration handoff.

On the combined tree, fmt, clippy with `-D warnings` in both feature modes,
all-feature/no-default/no-default-plus-clusterlod tests, the no-default wasm32
build and executed WASM parity pass. Runs and sweeps for 0.1, 0.2, 0.3 and 0.4
have zero mismatches. Reports for 0.2, 0.3 and 0.4 pass artifact verification;
0.4's twelve benchmark families pass in both consumer profiles and all nineteen
instrumented fuzz smokes pass. The first 0.4 Cargo-default measurement on busy
CPU 0 narrowly failed the unchanged `quat_encode` maximum (1.5033 > 1.50).
That full failed record is retained under `benchmark-0.4/superseded`; the full
rerun on CPU 17 passes. Neither result was dropped from the handoff.
