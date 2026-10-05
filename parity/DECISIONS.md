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

## D84 — 0.1.x inventory, profile and artifact destination

Implement SPEC-p01x only in this checkout, without Git metadata changes or
subagents. Place this lane's decisions after main's D83. The pinned
header adds filterIndexBuffer, filterIndexBufferMulti, generatePositionRemap
and generateVertexRemapCustom to the requested remapping surface. Stripification
and analysis remain 0.5; codecs, meshlets, tangents and experimental remeshing
remain outside this lane. Coverage lists each required upstream function.

Expose PreserveFolds and ErrorClamped only through experimental. Stable Sparse
and Prune apply to all edge-simplification forms; RegularizeLight already exists
and gains a separate qualification family. Sparse uses first-reference order
and subset bounds. Grid sloppy accepts LOCK alone; other vertex flags are errors.
Preserve upstream comparison order, probing, callback traversal, integer wrapping,
float operation order, and the distinct binary/numerical position equality rules.

In the original 0.1.x lane, the requested destination was unavailable and
evidence was retained on a writable sibling volume. Integration records now
point to /mnt/linux-extra/meshopt-artifacts/p01x; the coordinator moves the
historical data there. That archive is separate from the disposable build
target. Historical hashes need verification after the move.

Parallel lane 4 D41 settles two consumer profiles: Moss uses opt-level 3, thin
LTO, one codegen unit, release debug disabled; defaults uses opt-level 3, LTO
false, sixteen codegen units, release debug disabled. Keep the local crate
fat-LTO profile and benchmark both consumers with explicit environment overrides.
The RFC timing and memory bars remain unchanged.


## D85 — Quantization domains and benchmark transport

The inline normalized quantizers have no header bit-range assertion. Accept
Unorm widths 0 through 30 and Snorm widths 1 through 31: their signed C++ shifts,
subtractions and float-to-int conversions have defined results. Reject widths
that produce negative/oversized shifts or signed overflow. Do not restrict these
helpers to the common 8/16-bit storage widths. quantizeFloat accepts 0 through
23 as asserted in quantization.cpp, including 0 despite the header prose.

Keep every transport conversion, output flattening and byte-to-word expansion
outside timed calls. Build keys, colors and remapping inputs before warm-up.
Time validation, algorithm allocations, required destructive-input copies and
execution. Compare all meaningful outputs after measurement; keep paired
resident drivers and alternating same-core samples. Quantization batches vary
with workload size; single exponent helpers retain bounded scalar work.


## D86 — Sparse storage, position equality and live peak storage

Use the upstream-sized bit filter and quadratic-probed sparse reverse table,
with first-reference order unchanged. Internal indexed position/attribute views
read original records without packed geometry copies. Release the filter and
reverse table at the upstream phase boundaries. Preserve the maximum observed
heap request in Usage when later phases release scratch.

Standalone pruning uses simplifier.cpp's numerical position equality, including
signed zeros. Adjacency/tessellation use indexgenerator.cpp's binary position
equality. An explicit disconnected signed-zero regression distinguishes these
contracts. The port's initial pruning transcription used the wrong equality;
source review identified it and the corrected implementation is requalified.

## D87 — Direct output and measured safe-Rust optimizations

Replace allocating temporary results in caller-buffer remapping, filtering,
shadow, adjacency, tessellation, provoking, sloppy, prune and point paths with
shared kernels writing the caller's used prefix. Document late work exhaustion
as permitting partial output. Keep tails untouched. Release temporary welding
hash tables at upstream phase boundaries and count the maximum live storage.
Every preprocessing kernel has a measured exact work/heap boundary regression.

Retain checked arithmetic and bounds while specializing the optional point
colors and grid locks at the loop boundary. FIFO combines per-vertex scratch
fields and independent live-count/timestamp updates; repeated triangle corners
still update in upstream order. Fixed scans charge work in batches while
preserving the original failure prefix and work-exhaustion behavior.

Scalar quantizers allocate no output. Both timing adapters therefore preallocate
transport output before timing and time only checked scalar calls and identical
stores. Batch operation selection occurs outside the inner loop on both sides.
This removes transport-owned Vec growth from measurements, not algorithm
allocation. Edge simplification validation also covers topology and targets in
the C++ adapter; these checks are included in both timed boundaries.

The mutation smoke now generates valid bounded indices for option families and
exercises allocating and caller-output paths. Header/float/layout mutations are
retained as robustness cases. This is seeded elapsed-time smoke instrumentation,
not coverage-guided or sanitizer fuzzing and not the RFC's broader release gate.

## D88 — Adapter allocation bound and immutable validation binaries

The first 300-second mutation pass exposed unchecked transport allocation:
unused key-width fields in non-stream requests could allocate tens of GiB
before reaching the library. Retain all 32 original executables, seeds and
stderr in p01x-fuzz-before-adapter-fix. No library mismatch was observed in
that run; eighteen targets aborted in the adapter, so the smoke failed.

Check protocol widths and strides before allocation, allocate byte output only
for byte-producing operations, and bound all transport output to 128 MiB.
Add a direct regression for mutated irrelevant widths. These are protocol
bounds; the public library retains its independent checked views and Limits.
Repeat all smokes after the correction, including experimental update option
combinations and caller-output forms. Never count an adapter abort as a pass.

Copy each built executable to a content-addressed path within the single
required target. Concurrent fuzz, parity and profile builds cannot overwrite
an executable already bound to a record. Both reservation implementations
recheck capacity only when try_reserve_exact returns a capacity different from
its request; the actual capacity remains checked and charged in that case.

Retain all original benchmark workloads. Add attribute counts 0, 1 and 12 and
ratios 0.25 and 0.75 to the existing eight-attribute, 0.5-ratio edge cases; add
half-count point reductions. These additions satisfy representative parameter
coverage and do not replace any measured slow case. Reserve both siblings of
the measured benchmark core from fuzz affinity and retain load telemetry.

## D89 — Zero-target points and bounded-work fast paths

Retain the zero-target counterexample: extreme finite point coordinates made
Rust rescale and return NumericalFailure while C++ returned an empty result.
Match the upstream early return before rescaling and allocation. Keep finite
position/color, parameter and layout validation; a zero-byte budget succeeds
for fresh scratch and the caller destination stays unchanged. Add native and
executed WASM differential coverage plus a direct resource-boundary regression.

Cache retained workspace capacity once while a local allocation budget holds
its exclusive borrow. Charge every requested and actual owned allocation and
peak without resizing unrelated scratch or recomputing unchanged capacities.
Keep checked sums, fallible reservations, and retained capacity in the limit.

Split FIFO adjacency from hot live-count/timestamp fields without increasing
storage. Batch fixed scans and bounded searches when their entire bound fits;
otherwise retain per-visit checks before callbacks and mutation. Searches charge
only actual visits in either path. A regression compares result, work and callback
prefix for every small exhaustion boundary. Inline these helpers at the hot loop.

The preprocessing benchmark starts with ten alternating pairs, the RFC 6.1
minimum, and adds ten when the ratio interquartile range crosses a timing bar,
up to thirty. Preserve all 888 workloads and both consumer profiles. The interrupted
pre-refinement Moss baseline remains diagnostic evidence, never qualification.
Check positive finite sample intervals and exact agreement between resident
sample frames and the final serialized sample stream. Reject an incomplete CPU
override instead of attaching another core's sibling metadata to a record.

## D90 — Reusable scratch and preserved math backend

Reuse Workspace integer storage for FIFO counts, offsets, live records,
adjacency and dead-end storage; keep only required per-call resets. Reuse checked
float/integer scratch for point normalization, grid IDs, hash cells, reservoirs
and candidate errors. Output ownership and meaningful destination prefixes
remain unchanged. Retained capacities are charged on every call, including
mixed-operation workspace reuse. Numerical remap tables also reuse scratch.

Dispatch packed positions and point colors before hashing or repeated reads;
retain the checked byte/interleaved paths and compare big-endian strided views
with packed results. Binary adjacency/tessellation welding retains float bits;
numerical/custom welding retains signed-zero equality and callback order.
Grid quantization narrows through u16 only after normalization to [0,1] and a
grid bound of 1024; the integer result matches upstream and remains safe.

Reject an adaptive heap square-root cache trial. Its 32 KiB table increased
scratch and slowed the measured sloppy case; retain diagnostic records and
restore the unchanged 512-entry cache and pinned libm backend. There is no
architecture-specific or unsafe math implementation and no parity tolerance.

## D91 — Explicit destructive entry points

Keep the requested upstream snake-case `simplify_with_update` and the caller
fetch `_into` entry. Add `_in_place` reexports for both destructive operations
so the RFC naming convention is available without replacing the requested API.
They are identical functions with the same mutable views, checked errors,
resource accounting and partial-mutation contracts. The differential adapter
calls these explicit spellings; they share their upstream family records.

## D92 — Measurements on early failures

Reset the previous call's measurements before rejecting stream collections,
remap layouts or caller-buffer sizes. Keep retained scratch. Count remap-bound
validation and its maximum destination record in one scan, before allocation
or mutation. A regression primes scratch, exercises nine early-failure paths,
and confirms both current-call measurements and subsequent scratch reuse.

## D93 — Edge, quadric and point fast paths

Specialize adjacency/tessellation, filter/shadow and welded-edge traversal
before the hot loops. Use direct bounded edge/cell probes with the same hash,
probe order and exact limited-call exhaustion point. Inline small quadric
arithmetic without changing operation order or the pinned square-root backend.

For point cells containing exactly one source record each, return source order
without reservoirs. Check the selected grid's squared color weight first;
infinity times zero remains a NumericalFailure. Validate all supplied colors
and preserve caller tails and counted-write exhaustion prefixes. For other
colorless cells, omit zero color accumulation and arithmetic after the same
weight check. Reject an overflowing normalization reciprocal before rescaling;
finite bounds and a finite reciprocal make every normalized component finite.
The zero-target return still precedes this reciprocal check.

Keep initialized fetch buffers and batch index visits while borrowing each
remap entry once. Reject a fallibly reserved append-output trial: paired held
binaries show allocating fetch slowed by about 45 to 50 percent. Remove forced
inlining of generic remap generation after the same comparison shows slower
remap/custom/shadow calls. Retain the measured trial source and executables.

Retain the complete previous Moss baseline, source archive, executables,
functional records and smokes under before-edge-point-specialization. Its 888
outputs match, but only 17 of 32 families pass all bars. The command exits 1.
Interrupt the default-profile build before measurement to optimize first;
this does not count as a default-profile benchmark verdict.

## D94 — Colorless reservoir layout

Keep three position sums and a count in four floats for colorless point cells;
colored cells retain the upstream seven fields. Preserve accumulation order,
selected-grid weight checks, candidate tie order and error handling. Charge
actual retained float capacity, including calls that previously used colored
scratch. This changes storage only, with no extra allocation or math backend.

## D95 — Gradient records and provoking visits

Borrow three disjoint gradient records once for triangles with distinct vertex
IDs, then visit attributes and gradients together. Repeated IDs retain the
original sequential scatter additions. Preserve attribute and vertex addition
order, float arithmetic, work counts and peak storage. Specialize welded
adjacency before traversal and use direct three-corner gathers in sloppy
simplification. Batch provoking valence, triangle and remaining-corner visits;
retain rotations, wrapping valences, tie order and exact limited-call prefixes.

## D96 — Counted normalization validation

Count only the visited position prefix when bounds validation fails, including
the failing record; preserve the numerical bounds and subtraction order. Work
exhaustion reports the visited limited prefix before allocation or caller
mutation. Add a NaN-first and two-visit-limit regression. Count lock-support
validation and quadric finalization scans in sloppy simplification. Zip packed
positions with normalization outputs; retain the checked strided/byte fallback.

## D97 — UNorm endpoints

Return clamped UNorm endpoints before interior float conversion. Preserve NaN
and both-zero behavior and the upstream f32 endpoint rounding: bit counts 24
through 30 return 2^N for values at least one; lower counts return 2^N minus
one. Keep the interior multiplication and rounding order. Add native/C++/WASM
fixtures for every supported bit count with adjacent values below/above one,
infinity, NaN and negative zero, plus explicit high-bit endpoint regressions.

## D98 — Exact bounded grid conversion

Reconstruct the integer significand for rounded grid coordinates in [0.5,1024),
using a right shift of 14 through 24 bits. Validated finite normalization and
grids in 1 through 1024 establish this private precondition; debug builds assert
it. Keep the original multiplication and addition before conversion. A native
verifier extracts the helper verbatim and agrees with defined truncating casts
for all 92,274,688 representable operands in that interval. Retain its source,
executable, hashes and exit status. This uses no unsafe or architecture code.

## D99 — Reused filtering and shadow tables

Reuse checked Workspace integer storage for vertex remaps, vertex hash tables
and three-word triangle keys. Initialize required tables once per algorithm
call, and count triangle-key initialization. Keep exact hash probes, source
representatives, winding, output prefixes and caller tails. Charge requested
and actual capacities plus owned output, including all retained other scratch.
Validate destination capacity before scratch growth. Retain the preceding
measurement/source/executable trials; none substitutes for final qualification.

## D100 — Bounded sloppy quadrics

After finite bounds and reciprocal validation, normalized coordinates lie in
[0,2]. Triangle normals have magnitude at most two even when their squared
length rounds in the subnormal range. A triangle's coefficient magnitude is
at most 2048. Rounded f32 accumulation of increments bounded by 2048 cannot
escape magnitude 2^36: beyond that scale, an increment is below half an ulp.

A nonzero triangle weight is at least 2^-38 (two square roots starting from a
positive f32 squared length); nonnegative accumulated weight cannot decrease.
The cached inverse is below 2^39. Quadric evaluation on normalized coordinates
is below 2^43, so the final positional error is below 2^82 and remains finite.
Keep the same arithmetic and visits; omit redundant per-quadric/per-candidate
finite checks only in sloppy simplification. Other simplifiers retain their
checks. Add tiny/large finite-scale and normalized-subnormal feature fixtures
to native/C++/executed-WASM comparisons. Unsupported bounds or reciprocals
still fail before this phase.

## D101 — Fixed-charge scans and transport metadata

Batch complete scans whose pre-visit charge is fixed, retaining each original
charge on the limited path and the exact visited prefix on callback failure.
Cover unit, three-visit and attribute-triangle charges with an exhaustive small
boundary regression. Apply this to adjacency and quadric accumulation and the
validated sloppy candidate scans without changing arithmetic or traversal.
Specialize allocating and caller sloppy entry points before the shared kernel.
Dispatch packed position welding before its probes, including sparse views of
original authored positions; strided and byte views retain checked readers.

Compute remapped output transport length before Rust timing, matching the C++
adapter. The library still validates the remap and destination inside timing.
The removed scan only duplicated transport metadata work that C++ already
excluded. Retain the preceding 592-case diagnostic: 25 of 32 families pass all
bars; this smaller matrix is not the full qualification gate.

## D102 — Direct accumulation loops and compact sloppy records

Profiles show the fixed-scan callbacks being outlined in the heavy quadric
loops. Precharge a scan only when its entire fixed charge fits and every visit
has no fallible callback. Run its original loop directly; otherwise check the
original charge before each visit. Keep callback-aware scans for paths that
can return an error. This retains work, limited-call prefixes and arithmetic.

Store eleven floats per sloppy cell. During accumulation the last slot is the
weight; after accumulation replace it with the same reciprocal used before.
There are no later additions. Candidate evaluation still uses the shared
quadric formula and operation order. Attribute and regular simplification
quadrics retain their existing weight and inverse fields. Keep the preceding
focused measurement and profiled executable as diagnostic evidence.

## D103 — Bounded triangle filtering and disjoint gradients

Batch sloppy triangle filtering only when every triangle and its entire hash
probe bound fit the remaining work. Count actual visits once per batch; use
the original per-visit path when no complete batch fits. Keep writes before
probes, canonical triangle rotations, probe order and duplicate ties. Chunk
large calls according to the current remaining budget without extra storage.

Use safe slice get_disjoint_mut for distinct gradient records instead of
sorting their IDs. Repeated IDs keep sequential additions. Replace a complex
private packed-source return type with an alias to satisfy Clippy. Retain the
preceding failed Clippy log and the 216-case benchmark diagnostic; it remains
unqualified where bars miss.

Cap a batch's visit bound at usize::MAX as well as remaining u64 work, so its
local actual-visit counter cannot overflow on wasm32. Regressions compare
precharged and scalar scans at every small limit and check huge batch bounds.

## D104 — Retained typed simplifier scratch

Retain successful simplifier state in Workspace, including typed quadrics,
gradients and adjacency. Before reuse, clear per-call lengths and reset every
accumulation, classification, hash and lock field. Retain the exact sqrt memo
entries: they are keyed by complete operands and use the unchanged pinned math.
Failed calls may release this optional typed cache; early input validation
still precedes taking it. Clear and set_limits release it together with other
scratch.

Charge all retained capacities, including arrays unused by a later operation,
plus requested/actual growth and owned output. Workspace prepare paths and
local budgets include this cache even when another family uses the workspace.
Keep output ownership, arithmetic, hash order and counted algorithm visits.
The cache stores no borrowed source or caller output. It is private API state.

## D105 — Sloppy and update buffer reuse

Retain sloppy normalized positions, grid IDs, tables, cell quadrics, remaps,
errors and exact sqrt memo entries after success. Validate bounds and the
reciprocal before taking this cache. Reset every table, remap and quadric that
can carry algorithm state; overwrite all active normalized records and IDs.
Retain capacities unused by a subsequent zero-output call and charge them.
All workspace and local-budget accounting includes both typed caches.

Reuse update's required original-topology copy as owned scratch within the
typed state. It remains distinct from caller indices and is copied inside the
timed operation. Count the same copy visits and charge its actual capacity.
Keep both pools private, fallible and released by clear/set_limits. Preserve
the previous focused result: all five options pass; update and sloppy miss.

## D106 — Unchanged capacities and exact bounded floor

While a local budget exclusively borrows the workspace, charge the complete
cached owned capacities once. Reuse without growth cannot change those bytes
or the retained capacities; retain the owner-capacity invariant check. Every
new allocation or growth still checks requested and actual total capacity.
Keep checked sizes, peaks and byte-limit failures. No resource check is removed
for storage that can grow.

For private rounded grid coordinates in [0.5,1024), adding 2^23 rounds to an
integer. Its mantissa is that integer; subtract one when the rounded integer
exceeds the original value. This gives the exact floor, including ties. Native
exhaustive verification agrees with defined truncating casts for all 92,274,688
representable operands. Retain verifier source/executable/hash/exit evidence.
The strict default-rounding profile is binding. No unsafe or architecture code
is introduced; native/C++/WASM differential gates must pass again.

## D107 — Exponent transport allocation in the reference adapter

Store the C++ scalar exponent result in the existing scalar count field and
serialize its single word after timing. The upstream scalar API does not
allocate; the adapter previously pushed a word into a transport vector inside
timing. Remove that extra allocation and validate the same supported bounds
and parameter domain inside the measured C++ call. Output bytes stay identical.

Retain the prior source, 32 completed 300-second smokes, all eleven passing
local gates, run fixtures, unfinished benchmark and partial sweep under
before-exponent-adapter-fix. Interrupted ZIPs lacked central directories;
preserve their raw bytes and recover only CRC-verified complete local entries
into separately named diagnostic archives. These partial runs never qualify.
Rerun complete qualification with the corrected adapter.

## D108 — Serialized qualification builds

Serialize each reference/driver/exporter build through a target-local process
lock until all content-addressed executables and identities are captured.
Consumer profiles share temporary output paths; a second profile must not
replace the first profile's driver before its record binds it. Independent
execution can overlap after that interval. The lock affects build coordination
only, and is released when its process exits. Run fixtures before launching
concurrent benchmark and sweep execution, so fixture regeneration cannot race
a reader. No Git metadata or other lane's target is involved.

## D109 — Applicable JS and native preprocessing fixtures

Capture the ten newly applicable JS calls from unchanged test bodies, explicitly
awaiting module and test-runner promises. This covers compactMesh's fetch/index
remaps, update, all three point/color cases, prune, and reorderMesh's strip,
fetch-remap and index-remap stages. Recover the intermediate strip topology
using the returned inverse remap; retain the original test assertions.
Protocol 2 uses existing stream fields for explicit remap words and existing
attribute fields for authored point colors. These fields do not change the
benchmark inputs or timing boundary. Require their layouts before use.

Capture all three native customAllocator fetch inputs before mutation, preserving
the original allocator assertions and u16 adapter, plus emptyMesh's FIFO input.
Global allocator callbacks and cluster bounds stay upstream-only sanity calls;
Rust uses its existing fallible per-workspace storage and explicit u32 widening.
Require exactly 88 native and ten JS preprocessing fixture inputs at run time.
InternalDebug remains outside the supported simplification option domain.

## D110 — Remap loops and fetch scratch

Stop the corrected-adapter benchmark after 662 complete cases: remapping already
misses registered bars. Retain its raw interrupted ZIP, CRC-verified recovered
entries, all bound executables and source. The full 74,000-case sweep, 906 fixture
comparisons, eleven local gates and 32 300-second smokes pass on that revision;
none qualifies the subsequently changed source. Held-binary diagnostics show
that the recompiled C++ adapter also runs several other operations faster.
The corrected adapter remains the reference. Do not relax workloads or bars.

Specialize remap copies for the upstream common widths 4/8/12/16, with a general
checked-width fallback. Precharge a copy only when its whole fixed budget fits;
otherwise retain per-visit charging and partial writes. Fuse index and mapped
value validation when both complete validation charges fit. Defer sentinel
errors until index validation and destination-size checks finish, preserving
error precedence, counted prefixes and unchanged caller output on those errors.
Count both validation roles. Keep the original limited-budget path.

Retain fetch's remap table in Workspace and reset every active entry to the
unused sentinel. Charge all retained capacities, required output and actual
allocation growth. Dispatch common copy widths once per call and keep the
original first-use numbering, copies and per-index work. Add a regression for
error precedence and limited writes. These changes require new full gates.

## D111 — Single-stream hashing and no-scratch accounting

Retain the complete 592-case remap-copy diagnostic: twenty families pass all
bars on that smaller matrix. Vertex-byte copies improve substantially; index
remapping and hash-based helpers still miss. It does not qualify the full matrix.

Select a dedicated single-stream hash/equality closure when the validated stream
count is one. Keep byte keys, seeded hashing, probe order, canonical numbering
and the multiple-stream fallback identical. Inline the small key/hash helpers;
do not repeat the rejected forced-inline whole generation kernel from D93.

For calls that allocate no scratch, charge all retained capacities directly
instead of resizing five unused vectors to zero length. Their active state is
not consumed by those calls; later users still resize and initialize their own
ranges. Keep requested/actual allocation checks in every operation that grows
storage. In index remapping, use direct complete-budget validation loops and
specialized index iterators; preserve the original limited paths, error
precedence and measured work. Tests and Clippy pass; requalification remains
required.

## D112 — Bounded hash visits, provoking loops and empty workspaces

Preserve the preceding complete diagnostic matrices and executables: 25 of 32
families pass Moss's smaller matrix; 17 pass Cargo defaults. Neither is the full
qualification gate. Keep the corrected reference adapter unchanged.

Specialize canonical generation internally. Batch records only when the sum of
one index visit and the complete probe bound per record fits remaining work and
usize. Count actual index/probe visits, including a numerical failure, once per
batch. The limited path retains every pre-visit charge. Keys, callbacks, probes,
numbering and initialization remain identical. A regression checks fixed
callback/write/work witnesses at every budget from zero through 64.

Retain provoking remap/valence buffers in Workspace, resetting every active
entry. Replace infallible callbacks with direct whole-budget loops and per-visit
fallbacks, preserving wrapping valence, rotations, first corners and tails.
Charge all retained storage and outputs; caller buffers remain separate.

Keep a conservative private has_retained marker: false proves all scratch and
optional caches are absent. Mark it before every possible workspace growth and
when committing either typed cache; clear resets it. Failed growth may leave it
true. The true path still calculates all actual capacities. Debug assertions and
mixed-cache limit tests verify this invariant. Inline small work/retained helpers
so no-scratch calls avoid empty-vector accounting. Allocation checks and limits
for storage that can exist or grow remain unchanged. Tests and Clippy pass;
all differential and measurement gates must run on the new source.

## D113 — Fixed triangles and reference validation

Retain the preceding 23-of-32 smaller-matrix verdicts for both profiles. A
same-input default-profile instruction witness matches complete outputs:
provoking uses 178,325,064 Rust versus 97,731,575 C++ retired instructions,
including driver work; the preceding Rust uses 230,018,849. These diagnose costs,
not acceptance. A short 65-sample symbol profile loses no samples and identifies
generic array-map/rotation and memmove code in the hot path. Retain raw stat,
profile, commands, input, outputs and executable/source identities.

Replace known three-element integer maps and cyclic rotations with explicit
three-element expressions in filtering and provoking. Keep order, tie rules,
wrapping valence and every work charge. Avoid the general slice rotation helpers.

The C++ remap adapter omitted the upstream unused-sentinel precondition inside
timing, while Rust checks it. Validate mapped values during the same index pass,
defer a sentinel error until index validation finishes, and handle null indices
by scanning the remap. Validate scalar quantization parameter domains when a
scalar is called, and point targets, color weights and supplied colors. Existing
finite position/attribute/weight checks remain. These correct comparable valid-
domain validation under RFC 6.1; they do not add Rust resource limits to C++ or
change oracle algorithms. Rebuild and rerun all gates against this adapter.

## D114 — Representative colored point reduction

The original 888-case matrix omitted colored point reduction. Keep every case
and add half-target colored points on all four shapes, both APIs and all three
sizes, using deterministic three-channel colors and weight 0.5. This adds 24
cases: 912 per consumer profile. The two-size diagnostic now has 608 cases.
Fixtures, native/WASM identity and seeded sweeps already exercise authored and
synthetic colors. Require all 912 cases and both consumer profiles in final
verification; historical matrices remain diagnostics for their registered scope.

## D115 — Fused shadow emission and owned fetch topology

Retain the complete 608-case corrected-adapter diagnostics: thirty families pass
Moss and twenty-five pass Cargo defaults. They do not qualify the full matrix.

Fuse canonical shadow hashing with index emission, matching the oracle's single
walk. Preserve all keys, probes, first representatives and total work; bounded
batches include the output visit, with pre-visit charging on the limited path.
Caller output may contain a prefix on late work exhaustion, as documented. A
regression checks every budget through completion and preserves the unused tail.

Allocate and copy fetch topology once, then rewrite the owned buffer in place.
Count the required copy, charge requested and actual capacity, and retain the
same first-use vertex bytes and caller-buffer kernel. This avoids a separate
zero-filled output/index-read walk. All gates must run on this new source.

## D116 — Direct FIFO visits and longer timing batches

Retain the complete 186-case default-profile remaining-family diagnostic, source
and binaries. Four families pass; FIFO misses mean, update misses one tiny max,
and allocating sparse fetch has unstable timings. These are not final verdicts.
FIFO's same-input instruction profile attributes an additional 32.65 percent of
samples to an outlined emission callback. Replace it and its candidate scan with
direct fixed-charge loops and unchanged limited-budget paths. Precharge the
infallible boundary-edge and solve-lock scans; retain floating operation order.
Inline the small quadric finite check and borrowed mutable-view conversions.

The fetch medium/sparse Rust timing coefficient of variation is 1.15, whereas
its same-input instruction count is 59,821,530 against 59,046,402 C++ (including
driver work). Million/sparse timings depend strongly on paired order. Preserve
these diagnostics. Allocate fetch's copied topology and bytes before preparing
the remap table, matching C++ allocation order; budget all retained and new
capacities before execution. End the local heap budget before scratch growth.

Use identical predetermined batches on both preprocessing adapters: 1,024 calls
below 3,000 indices, sixteen below 300,000, one otherwise. The previous short
batches are historical diagnostics. Every call still validates, copies, allocates
and executes inside timing; retain all raw paired intervals and the original
912 workloads, sample policy and bars. No result-based case or sample filtering.

Validate point colors through the counted fallible scan, so an invalid first
color reports one visit rather than charging unvisited records. Add a prefix
regression. Update the smoke proof limit to the binding RFC's four CPU-hour
release requirement; this lane's explicitly requested budget remains 300 elapsed
seconds per function. Full requalification is required on the resulting source.

## D117 — Bounded triangle-filter probes

Retain the 608-case default diagnostic: thirty-one families pass; allocating
filtering on tiny smooth geometry misses the maximum. Profiles match complete
outputs and identify generation and triangle probing as the dominant work.

Use a direct triangle probe leaf with a bounded batch whose per-triangle bound
includes the outer visit and every possible table probe. Count actual visits,
including numerical failure; preserve pre-visit checks on the limited path.
Keep compact vertex numbering, canonical rotations, collision order, duplicate
removal, source winding and writes identical. A fixed witness checks every work
budget across original, rotated, reversed and degenerate triangles. Inline the
small remap validation helper without changing errors or counted prefixes. Retain the
same adapter, timing batches, workloads and acceptance bars. Requalify all gates.

## D118 — Compact checked float views

Retain the complete preceding 608-case default diagnostic: filtering now passes;
update on tiny sparse input remains over the maximum. Eleven local gates, 920
fixture comparisons and all thirty-two 300-second smokes pass on that source.
Stop its incomplete sweep with exit 130 to optimize first; preserve the raw ZIP
and CRC-verified recovered equal triplets. None qualifies the changed source.

After ordinary checked layout validation, borrow tightly packed float positions
as safe three-element slice chunks, including an explicit offset and count.
Preserve all values, counts and padding; byte and wider-stride views retain the
general path. Validate float attribute records through one checked record slice,
retaining per-component visits, position-before-attribute order, first failure
and the general byte-view fallback. No arithmetic operations are reordered.
Add a compact-offset/padding and exact invalid-attribute-prefix regression.
Keep the same reference, timing batches, workloads and bars; rerun all gates.

## D119 — Counted attribute-row normalization

Retain the 32-of-32 default diagnostic and interrupted full Moss attempt from the
preceding source. Its tiny sparse caller-update median is 1.5045358319020972,
above the unchanged maximum of 1.5. Stop with exit 130 to optimize; recover and
verify the interrupted ZIP, preserving raw samples and bound executables.

Normalize checked float attribute records through their borrowed record slice
and a counted fallible scan. Keep original component and positive-weight order,
one multiply per active value, finite-result checks, zero-weight visits and
compact destination order. Byte views retain the scalar component path. Count
the actual failure prefix and preserve per-visit checks when the row budget does
not fit. The existing mixed-layout, zero-weight, numeric and exact-limit tests
exercise both paths. Requalify fixtures, sweeps, smokes and both profiles.

## D120 — FIFO initialized scratch and direct setup scans

Retain both preceding complete diagnostics: all families pass defaults; Moss
FIFO's tiny sparse caller case misses the maximum. Update now passes both.

Workspace preparation initializes newly extended integer and flag elements to
zero. Clear only the intersection of FIFO's count/hot/emitted ranges with each
vector's previous active length; fresh and extended elements already have their
required initial value. Preserve reset semantics for smaller, larger and mixed
operations, including a partially overlapping hot pair. Capacity accounting and
growth checks remain unchanged. A mixed-fetch/FIFO reuse regression checks all
results and work against fresh workspaces at three cache sizes, including wrap.

Replace FIFO's infallible count, offset, insertion and hot-state setup callbacks
with direct precharged loops and per-visit limited paths. Preserve every record
visit, insertion order, offset and wrapping integer operation. Requalify all
gates on the new source; retain both profiles, all workloads and unchanged bars.

## D121 — Sloppy direct scans and fetch remap initialization

Retain D120's complete 912-workload Moss result: thirty families pass, while
million disconnected caller Sloppy measures 1.5028046427425936 and million sparse
allocating fetch measures 1.680978884849942, above the unchanged maximum of 1.5.
Moss exits 1. Stop defaults with exit 130; preserve its partial record and buffers.
The D120 fixture, full sweep, local checks, source and executables are historical
proof for that source, not qualification for this change.

Identical-output million-case instruction profiles show Sloppy at 60,616,848,093
instructions versus C++ 40,849,259,767, with outlined scans and triangle emission
among its costs. Use direct infallible counted triangle and error scans, manual
three-corner rotation, and the existing per-visit limited paths. Preserve every
visit, floating operation, representative tie and triangle order.

Fetch retires 7,251,364,872 instructions versus C++ 6,883,368,810 and fewer cycles
in this diagnostic; this does not reproduce the recorded timing miss. Initialize
new integer remap entries directly to the sentinel and reset only the previous
active intersection. Preserve allocation order, capacity accounting, checked
limits, first-use numbering and reuse behavior. Add a mixed FIFO/fetch regression
covering both forms, shrinking, growing, empty input and destination padding.
Requalify all gates with the same reference, profiles, workloads and bars.

## D122 — Initialized typed fetch records

Retain D121's 920 matching fixtures, 74,000 matching seeded cases, all eleven local
checks and both complete two-family diagnostics. Sloppy passes both; allocating
fetch's million sparse case remains above the maximum in each. These are
historical results, not a final benchmark verdict.

For fixed widths 4, 8, 12 and 16, reserve the complete output capacity fallibly
as a Vec of initialized byte arrays. Push one copied array on each first use,
then safely flatten that Vec without another allocation. Standard-library
into_flattened is stable since Rust 1.80, within this crate's Rust 1.88 minimum.
The generic-width path remains initialized bytes. This avoids writing an unused
output tail and then overwriting every used record. It differs from the rejected
generic byte-append trial by copying one statically sized record per push.

Charge requested and actual full capacity before execution, including unused
records; preserve topology-copy and scratch allocation order, counted work,
checked views, first-use order and limited-path checks. The unique first-use
count cannot exceed the reserved source count, so pushes require no additional
allocation. Add strided-offset witnesses for every fixed width and the generic
fallback, unused vertices, exact output bytes, full capacity and work limits.
Requalify unchanged fixtures, sweeps, smokes, all 912 workloads and both profiles.

## D123 — Keep typed reservation local to fetch

Retain D122's full exact sweep, fixtures and local checks, and both passing
48-workload fetch/Sloppy diagnostics. Its full Moss attempt records 632 workloads
before being stopped with exit 130: tiny seam-heavy allocating shadow generation
measures 1.9635326001589135, with a narrow paired ratio range. Preserve and recover
that raw archive; it does not qualify the changed source.

Restore Budget::filled verbatim from D121. Keep the new checked empty reservation
helper separate and used only for typed fetch output. The previous factoring
changed a shared allocation path despite the intended fetch-only optimization.
Do not alter the reference, workloads, sampling, flags, limits or bars. Requalify
all families on the resulting source, preserving every preceding result.

## D124 — Fetch output strategy by input size

Retain D123's 608-case Moss diagnostic with all thirty-two families passing.
Defaults passes thirty-one: tiny smooth allocating fetch measures
1.771242085582236, above the unchanged maximum. The queued full benchmark does
not start and exits 1 on this diagnostic gate. Keep the full record and samples.

Use the original initialized byte-output path below 1,024 supplied vertices.
Use typed first-use copies at and above that threshold, retaining their measured
large sparse improvement. The threshold changes execution strategy, not the
workloads, acceptance bars, record membership or sample selection. Both paths
reserve and charge full capacities before execution, use the same first-use
numbering, copies and work checks, and return identical initialized used bytes.
Extend the fixed-width and mixed-workspace witnesses to cross the threshold,
including strided offsets, unused vertices and exact memory/work boundaries.
Requalify every gate on the changed source.

## D125 — Prepared shadow sentinels and fixed-width keys

Retain D124's 924 exact fixtures, 74,000 exact seeded comparisons and all eleven
local checks. Moss passes all thirty-two families in its 608-case diagnostic;
defaults passes thirty-one, with tiny sparse allocating multi-stream shadow at
1.5618944177614738. The diagnostic guard exits 1 before starting the full matrices.
Keep all raw samples and source/executable bindings.

The same-input CPU-0 profile matches outputs and retires 631,153,868 Rust
instructions versus 571,149,435 C++, with fewer Rust cycles. This does not
reproduce the CPU-15 timing miss. Hash/equality dominates its cycle samples.
Prepare shadow's private integer scratch directly with sentinels and remove
its subsequent duplicate fills. Keep the original logical initialization charges
and failure order. Filtering retains its original initialization path.

Specialize single-stream shadow keys at widths 4, 8, 12 and 16 as checked byte
arrays. Keep the same Murmur operations, byte equality including padding,
canonical first references, collisions, visits and limited-budget checks.
Other widths and multiple streams use the existing general path. Add mixed
FIFO/shadow reuse, offset/stride, width, empty/growth/shrink, both-form and output
tail witnesses. Existing every-budget shadow checks still cover partial writes.
Requalify all families with the same profiles, inputs, sampling and bars.

## D126 — Exponent scalar transport outside timing

Retain D125's 924 matching fixtures, 74,000 exact seeded comparisons and eleven
passing local checks. Its complete 608-workload Moss diagnostic passes all
families; defaults passes thirty-one. Position exponent measures mean
1.255058259215078 and maximum 1.6255089507839753. The diagnostic guard exits 1
before the full benchmark starts. Shadow now passes both profiles.

The Rust timing adapter constructed a one-word transport Vec per exponent call,
whereas D107 already made C++ carry a scalar and serialize after timing. Change
Rust's payload to carry the scalar too. Construct its output word buffer after
the timer, matching the stated exclusion of serialization and the reference.
Preserve the helper, validation, bit representation, batching, workloads, samples
and bars. Required algorithm-owned allocations remain inside timing. Requalify
all source/executable identities and gates; earlier timings are historical.

## D127 — Prepared owned remaps and longer fixed batches

Retain D126's 924 fixtures, 74,000 seeded comparisons and all eleven local gates.
Both complete 608-workload diagnostics pass every family. Its full Moss attempt
is stopped with exit 130 after a tiny smooth allocating custom-remap miss.
The exact same input, Rust executable, C++ executable and CPU 15 measure
0.9585386152386463 in the diagnostic and 1.94185916805916 in the full attempt.
Retain both records, hashes, raw samples and their explicit comparison. The Rust
batches span about 0.33 to 0.81 milliseconds; no sample is removed or reclassified.

Allocate owned remaps initialized to the required sentinel, and skip their
second fill in a prepared kernel specialization. Retain the same logical
initialization charge, table reset, callback/probe order, limited checks,
allocation order, requested/actual capacity accounting and errors. Caller-buffer
and filtering initialization stay on their existing unprepared specialization.
The existing every-budget custom callback and remap-limit witnesses apply.

Before requalification, use longer predetermined batches equally in both native
adapters: 32,768 calls below 3,000 indices, sixteen below 300,000, one otherwise;
position exponent uses 262,144 scalar calls. These fixed counts replace D116's
short batches to amortize sub-millisecond scheduling/frequency effects. Every
call still validates, copies, allocates and executes as applicable inside timing;
serialization stays outside. Keep all 912 workloads, alternating pairs, ten to
thirty samples, raw intervals, unchanged time/memory bars and source guards.
This is a recorded protocol change, not a result-based retry or sample filter.
Requalify fixtures, sweeps, smokes and both complete profiles on the new source.

## D128 — Fixed-width keys in single-stream generation

Retain D127's 924 exact fixtures, 74,000 exact seeded cases and eleven passing
local checks. Its longer-batch Moss diagnostic records 202 workloads before
being stopped with exit 130 after tiny smooth allocating filtering measures
1.882115003775167. Stop the queued full benchmark with exit 130 as well; recover
and preserve the partial diagnostic buffers. None is a final bar verdict.

Use checked fixed-width byte arrays in the single-stream generation path at
widths 4, 8, 12 and 16, as in D125's shadow path. This covers filtering's remap
stage and caller generation. Preserve the same hash operations, initialized
key bytes, numeric/canonical distinction, equality, callbacks/probes and work
charges. Multiple streams and other widths keep their general path. Existing
mixed-layout, every-budget filtering/remap and cross-backend fixtures cover the
result and failure contracts. Requalify all families; retain D127's fixed batches,
all workloads, sample policy and unchanged bars.

## D129 — Complete matrices from disjoint size groups

Avoid replaying the current D128 tiny/medium measurements. Before measuring the
remaining group, complete each consumer matrix as two fixed, disjoint groups:
all 608 tiny/medium workloads, followed by all 304 million workloads. Use the
same saved CPU, source snapshot, executable hashes, profile overrides, timing
boundary, D127 batches and sample policy. Require these identities to match.
Retain every input, output and raw sample; union membership must equal the exact
912-workload matrix, without overlap, skipped cases or sample filtering.

The complete verifier recomputes each case summary and family geometric mean,
maximum and peak-heap ratio over the union and enforces the unchanged bars.
Archive the fragment records/hashes, assembler code and complete buffer ZIPs.
The enforced assembly command exits 0 only if both full consumer matrices pass;
no fragment alone establishes acceptance. Historical source epochs remain
non-qualifying. This changes orchestration only; repository code and native
adapters remain frozen. Record the split explicitly in the final compact record.

## D130 — Preserve buffers without duplicating million archives

The shared large volume has 16 GB free before the million measurements. Stop
only the waiting D129 orchestrator with exit 130, before it measures any cases.
Change archive transport so each million fragment ZIP becomes its complete ZIP;
append the tiny/medium members there. Point the retained million fragment record
at that complete archive and record its final hash before hashing the fragment
record. Preserve every fragment workload, input, output and raw sample. The
complete archive verifier still checks exact membership and every member hash.

Restart the waiting orchestration with this change. Neither source, executables,
profiles, timing, membership nor acceptance bars change. Keep the assembler code
and its final hash. This bounds additional archive growth to one complete ZIP
per profile, rather than retaining a duplicate million payload archive.

Require complete fragment membership, rather than a fragment-only geometric
mean, before completing the full matrix. A fragment aggregate cannot determine
the full 912-case aggregate. The independent full verifier and enforced command
apply every bar over the complete union. Restart this still-waiting orchestrator
once more with exit 130; no million samples have been measured or discarded.

## D131 — D128 maximum misses require another source change

D128 retains 924 exact fixture comparisons, 74,000 exact seeded comparisons,
76 root tests in each feature mode and eleven passing local checks. Stop its
Moss tiny/medium diagnostic with exit 130 after its maximum exceeds 1.5:
`simplify_with_update/tiny/sparse/mode-1` measures 1.5039728238969359 and
`generate_vertex_remap_custom/medium/smooth/mode-1` measures
1.588478319023424. Stop both still-waiting continuations with exit 130. Preserve
the completed records, original and recovered buffers, source and bound
executables under `fixed-generation-trial`; these are not qualifying bars.

Profile the two saved inputs and bound executables, then optimize the safe Rust
paths. Keep the exact outputs, callback order, checks, work and heap limits,
D127 fixed batches, all 912 workloads, sample policy, both profiles and bars.
Requalify the changed source; do not replay unchanged source until it passes.

## D132 — Short custom-remap scans and shared triangle planes

The saved D128 custom-remap input profiles at 1,439,077,402 Rust instructions
versus 1,049,612,083 C++ instructions; Rust uses 545,303,986 versus 359,259,180
cycles on the diagnostic CPU. Its generation kernel holds 79.49% of samples,
with a large loop body and register spills even for already-mapped indices.
Outline only the first-seen custom-remap probe body. Keep the common visitation
charge and mapped test in the small loop; keep the same hash, probing, callback
order, limited-work prefix and allocation accounting. Other remap families keep
their inlined probe path.

The saved update input profiles at 48,539,640,453 versus 31,102,384,780
instructions and 13,218,710,264 versus 9,376,529,049 cycles. Quadric construction
holds 34.00% of Rust samples. Compute the normalized face plane once and reuse it
for volume gradients. Preserve the original cross product, normalization, plane
and gradient arithmetic order, accumulator order and work charges. The memoized
square root is pure and exact, so removing a repeated lookup cannot change its
result. Retain the profiling inputs, instructions, cycles, sampling reports and
output identity; diagnostic counts do not establish timing acceptance.

## D133 — Keep custom callbacks behind position equality

D132's eleven local checks and all 924 fixture comparisons pass. Its focused
Moss update ratios are 1.399 and 1.478 for allocating and caller forms; custom
remap is 1.560 and 1.477, so the allocating maximum still misses. Stop the
focused continuation and the incomplete sweep with exit 130, retaining their
records, buffers, source and executable identities under `short-custom-plane-trial`.

The D128 assembly witness hoists custom callback state and the callback's modulo
calculation into probing before numerical equality succeeds. Isolate that
callback invocation in a non-inlined helper, used only after positions compare
equal. Keep callback order, arguments, results, hash probing, work charges and
all output/limit behavior. Retain D132's plane reuse and short mapped scan. This
changes library code rather than adapter tracing or benchmark input/policy.
Requalify the new source, starting with the saved missed cases in both profiles.

## D134 — Preserve wall timing and remove bounded bookkeeping

D133 retains 924 exact fixture comparisons and eleven passing local checks. Both
focused diagnostic commands exit 0, but their diagnostic-only ratios still miss
the bar: Moss allocating update/custom are 1.508049062218985 and
1.5033565359822592; defaults allocating update is 1.5834009068344073. Preserve
these records, source and seven executable copies in `outlined-callback-trial`.
The RFC names time ratios without specifying a clock. Retain the established
wall-clock contract and every failed measurement; do not substitute CPU time.

Custom remap now dispatches indexed and unindexed traversal once. The indexed
ample-work path iterates a checked index subslice, removing the optional-buffer
branch and per-index lookup check. Keep initialization, record/probe/callback
order, bounded batching and the original limited prefix.

For update solving, canonical position roots partition wedge cycles. After
adjacency, an upper bound on remaining charges is two vertex scans, four visits
per stored attribute gradient and two adjacency scans. If this bound fits the
remaining work, accumulate actual visits without per-visit exhaustion checks;
charge the exact visited prefix on success or numerical failure. Otherwise use
the original per-visit checks. Overflow in the bound selects the limited path.
No float expression, destination write order, allocation or resource limit is
changed. Requalify the changed source and keep all workloads and bars.

## D135 — Retain every current focused sample in the full matrices

Reuse all four predetermined D134 focused cases per profile, including defaults
whose measurements are still pending when this decision is recorded. Their
case membership was fixed before this source epoch was measured: allocating
and caller forms of tiny sparse update and medium smooth custom remap. Measure
all 604 disjoint remaining tiny/medium cases, then all 304 million cases. No
current focused case or sample is dropped, replayed or selected by its ratio.

Require source, executable, reference, profile, clock/boundary, fixed batches,
sample policy and CPU identities to match. Assemble the 608-case size group,
then apply D129/D130 to form the exact 912-case matrix. The final independent
verifier also checks the nested four-plus-604 assembly and every fragment hash.
The four-case diagnostic alone does not establish any family's geometric mean.
A focused maximum miss stops completion because the full maximum cannot pass.
All bars remain unchanged. All 78 root tests, including D134's two new contract
tests, and the eleven local gates pass; final sweep and full timing are pending.

## D136 — Lossless storage of duplicate historical recovery ZIPs

The shared volume has about 11 GB free before the million groups. Historical
interrupted ZIPs and their recovered ZIPs duplicate the same compressed member
payloads. Retain the raw ZIPs and encode each recovered ZIP as a recipe: its
original local headers, compressed-payload spans in the raw ZIP and exact central
directory bytes. Verify every compressed span against the recovered ZIP, then
verify its complete original SHA256 and byte count through reconstruction from
the saved recipe. Remove only the duplicate recovered ZIP after both checks.

Keep `lossless-recoveries.json` and `restore_recovered_recipe.py`; restoration
recreates the original recovered ZIP byte for byte, including its archived hash.
No historical input, output or raw sample is removed. Every qualifying archive
stays an ordinary complete ZIP. This changes artifact storage only; source,
executables, wall timing, cases, samples and bars remain frozen.

The trial replaced two historical recovery ZIPs before stopping with exit 1 at
its 16 MiB directory-size guard. The shared volume subsequently gained free
space externally. Restore both originals rather than continue this storage
trial. Each restored ZIP matches its original SHA256 and byte count: 27,350,414
and 740,676,464 bytes. `lossless-restoration.json` records the checks. Retain the
partial manifest, recipes and failed log as diagnostic evidence; no recovered
archive remains removed and no qualification archive was affected. The trial
is unapplied in the final artifact inventory.

## D137 — Inline first-seen custom remaps after callback isolation

D134's source `db4a731ac894767c02ba69e480229df0b83136a9987784d1e3871edb16d00c25`
passes 924 fixtures, 74,000 sweep cases and eleven local gates. Both 608-case
small/medium groups completed, but defaults allocating custom remap on medium
sparse geometry has median 1.520766148, exceeding 1.5. Moss's group passes;
its worst maximum is 1.474358. Stop the million continuation and final checks
with exit 130; retain all records and samples in `indexed-custom-trial`.
Recover its interrupted million ZIP and verify all 24 recorded workloads (72
members). This epoch does not qualify. The 604-case command's exit 0 describes
diagnostic execution, not acceptance of its failed defaults bar.

Profile the exact failing defaults input against bound executables on CPU 0.
Rust executes 1,449,205,503 instructions and C++ 999,525,658; Rust's first-seen
outlined function accounts for 60.88% of sampled cycles, with substantial call,
register-save and spill costs. CPU counters diagnose source cost; they do not
replace wall-clock acceptance. An initial diagnostic requested 200 transport
samples, exceeding the adapter's cap; retain its rejected-input log and use the
supported 100-sample transport. No qualifying timing sample was rerun.

Inline the custom first-seen probe body into the indexed/unindexed kernel now
that D133 keeps callback work behind a separate call boundary. Also assign an
inserted remap directly, eliminating the redundant sentinel reread after a new
entry has already been written. Preserve hash/probe order, numerical comparison,
callback traversal, compact numbering, work charges and limited-work prefixes.
Keep checked safe indexing and all heap bounds. Requalify every gate against
this changed source; do not reuse D134 timing or parity receipts.

## D138 — Predetermined ten-case diagnostic and complete timing membership

Before measuring D137, select custom remap on all four medium shapes in both
allocating and caller forms, plus tiny sparse update in both forms: ten cases
per profile. Retain every sample and include all ten in the final 608-case size
group. Measure its 598 disjoint remaining tiny/medium cases, then all 304 million
cases, for the unchanged exact 912-case matrix. Extend D135's nested membership
verification to ten plus 598; retain source/executable/profile/clock/batch/sample
and CPU identities and unchanged bars. No case or sample is selected by its
measured ratio, skipped, replayed or filtered. A diagnostic maximum or heap miss
precludes completion; its incomplete family means are not final acceptance.

## D139 — Checked packed arrays for large owned fixed-width fetch

D137's source `7c0b2caf7e69d2a05e372c5ba891dc9faa4577332e3e5427ed8695a632163f79`
passes all eleven local gates, 924 fixtures and 74,000 seeded comparisons.
Both 608-case size groups complete with passing case maxima and heap ratios.
Defaults FIFO's group geometric mean is 1.250148942; a full-family verdict still
requires the million group. Stop the million continuation when Moss allocating
fetch on million sparse geometry records 1.741135, above 1.5. Retain all samples,
297 completed million workloads and the exit-130 orchestration receipts in
`inline-custom-trial`; this epoch does not qualify.

Profile that exact input against its immutable executables on CPU 0. Rust
executes 7,300,049,010 instructions versus C++ 6,883,368,388; Rust's fetch function
accounts for 72.29% of sampled cycles. These source-cost diagnostics do not
replace the failed wall-clock verdict. Extract the failing input directly from
its original local ZIP member while full interrupted-archive recovery proceeds;
verify its decompressed length, CRC and recorded SHA256 before profiling.

For the large owned 4/8/12/16-byte paths, dispatch tightly packed records once
and use a checked `[[u8; SIZE]]` view. Each first-use record then needs one typed
source index instead of dynamic byte-range arithmetic/checks. Strided records
retain the original checked fixed-width slice access. Keep initialized pushes,
fallible full-capacity reservation, actual capacity accounting, compact order,
work counts and limited-work prefixes. No unsafe initialization or reinterpretation
is introduced. Requalify the changed source against every gate and full bar.

## D140 — Whole matrices with expensive failure-prone families first

For D139's new source, run one complete 912-case matrix per profile, sequentially,
with enforcement enabled. Predetermine size order million, tiny, medium. Measure
vertex fetch across all four shapes, then sloppy across all four shapes, then
all remaining families in their existing shape/family order. Cache each size
group's four generated geometries in the parent; generation stays outside timing. The exact membership, forms, variants, sample
policy, wall-clock boundary, fixed batches, core selection and bars are unchanged.
This identifies a new miss early without dropping any accepted case or sample.
There is no focused timing run or fragment assembly for this epoch. A completed
qualifying matrix contains all 912 workloads and every prescribed paired sample.
A case maximum or heap miss stops its epoch after recording the complete case.
Retain failed attempts; never repeat an unchanged source until a bar passes.

## D141 — Initialized typed fetch output and a scalar write cursor

D139's source `158fa42e6e09d4a46cb14c2b85ffea97c275f481672ac9d1c72c346c1361df12`
passes all eleven local gates, 924 fixtures and 74,000 seeded comparisons.
D140's enforced Moss matrix stops after its seventh case: allocating million
sparse fetch records 1.664227231, still above 1.5. The matrix and final-check
commands exit 1. Retain its seven complete cases, every sample and all bound
executables in `packed-fetch-trial`; verify all 21 ZIP members against their
recorded lengths and hashes. This epoch does not qualify.

Replace repeated initialized `Vec::push` calls in the large fixed-width owned
fetch path with a full initialized typed record buffer and a scalar first-use
write cursor. Checked source and destination array indexing remains safe; trim
only the used output length after execution, then flatten the initialized
records. This removes per-record Vec length/capacity mutation and matches the
reference allocating form's full-buffer initialization. Keep fallible allocation,
full actual capacity accounting, compact output order, work charges and error
prefixes. Remove the now-unused reservation-only Budget helper. Retain D139's
packed source view and the strided fallback. Requalify all gates and both D140
complete matrices against this changed source; do not reuse older timing.

## D142 — Coordinator time box for the remaining fetch maximum

The coordinator authorizes D141's current fetch attempt and at most one further
fetch optimization. If the affected case still exceeds 1.5, stop optimizing
fetch. Complete remaining qualification with its failed bar and documented
residual: retain profile evidence, the best observed ratio and the concrete
safe-Rust/source-cost reason it cannot be closed within this time box. Keep the
bar unchanged; the coordinator owns the residual decision. Preserve every other
passing family. Do not claim a residual as an unconditional performance pass.

## D143 — Final permitted fetch attempt: iterator append

D141's source `a5779043780c97c2231b575d76e8f6cd5ba393780751a953b54b10bf07e01bfb`
passes all eleven local gates and 924 fixtures. Its enforced Moss matrix fails
on allocating million sparse fetch at 1.878920636, worse than D139's 1.664227231.
The matrix exits 1 after seven recorded cases. Stop the still-running sweep with
exit 130 before changing sources; retain its partial archive and all benchmark
members, logs and bound executables in `initialized-fetch-trial`. Do not qualify
this source or use its samples for final acceptance.

Use the one further fetch attempt allowed by D142. Restore the fallibly reserved
typed output and D139's checked packed source view. In the ample-work path, append
first-use records through `Vec::extend` over a filtering iterator while rewriting
indices and tracking compact numbering. This permits the standard-library append
cursor to stay inside its own loop instead of mutating a borrowed Vec on every
first-use record. At most one record per validated vertex is appended; the full
source count is reserved and charged before iteration, so no growth is needed.
The limited-work path retains per-visit charges and initialized pushes. Preserve
exact output, actual capacity accounting and error prefixes; no unsafe code or
uninitialized output is introduced. This is the final fetch optimization attempt.
Requalify completely, keeping 1.5 unchanged and documenting any residual under
D142; make no further fetch optimization.

## D144 — Fixed-width allocating vertex-remap keys

D143's source passes all eleven local gates, 924 fixtures and 74,000 seeded
comparisons. Its Moss matrix stops after 477 cases at allocating vertex remap on
tiny disconnected geometry: 1.998288406 against 1.5. Fetch's previously failing
case passes at 1.378806161. Retain the complete failed epoch and all 1,431 closed
ZIP members in `iterator-fetch-trial`; both orchestrations exit 1. No benchmark
retry of unchanged source is performed. This non-fetch miss is not covered by
D142's residual authorization.

Exact-input CPU profiling retains immutable binaries and equal C++/Rust outputs.
Rust uses 24,379,789,309 instructions versus C++ 19,362,912,190 across 100 adapter
samples; dynamic memcmp accounts for 17.73% of sampled Rust cycles. The allocating
single-stream path still used dynamic-width hashing/equality while the caller
path already specialized four common widths. Dispatch allocating 4-, 8-, 12- and
16-byte keys to the same checked fixed-width array accesses. Keep general widths,
strides, allocation, capacity accounting, work limits and error order unchanged.
D143's fetch section is byte-identical; no further fetch optimization is made.
Requalify all gates and complete matrices, preserving every other family's bars.

D144 extends D140's predetermined whole-matrix order: within each size, visit
fetch, sloppy and now vertex remap across all four shapes before remaining
families. Membership, sample policy, fixed batches, timing and bars remain
unchanged. This exposes the newly identified tiny remap miss before lengthy
option cases. Independently verify this exact order; retain every case/sample.

## D145 — Final local 0.1.x verdict and retained evidence

The frozen source `747af689c02da527aa843d97a9512f0812a2ab62fcdbc5f0fa841b4ed5f50aa1` completes the brief's local
qualification under D142. Correctness and integrity gates pass. The unconditional
performance verdict is `recorded residual; coordinator decision pending`.
All 27 additional functions and five option families are implemented. Sparse,
Prune and RegularizeLight are stable; PreserveFolds and ErrorClamped remain
behind `experimental`, with independent records. Unknown option bits are rejected.
No codecs, meshlets, Moss integration or publication are claimed.

All eleven local checks exit 0, including formatting, strict Clippy, all-feature
and no-default-feature tests (78 root tests each), core no_std WASM compilation,
adapter tests and measurement tests. The fixture command exits 0 with 924
exact C++/native Rust/WASM comparisons. The sweep command exits 0 with 74000
exact comparisons: 2,000 cases for each of 32 new and five existing families.
The independent archive verifier confirms every retained input/output identity.

D144 specializes the allocating single-stream remap's common widths after a
retained tiny-case miss. The D143 fetch section is unchanged. Fresh qualification
covers this final source rather than reusing the failed epoch.

Both complete 912-workload consumer matrices retain all 32 family verdicts
against unchanged geometric-mean, maximum-time and peak-heap bars. Their
unconditional all-bars result is `fail with the documented fetch residual`. D127 uses identical
fixed batches in both adapters. D144 extends D140 to run complete matrices with million cases
first and fetch/sloppy/remap across all shapes first within each size. Every prescribed
case and paired sample is retained. The independent verifier checks exact
membership and scheduling, source/executable identities, raw outputs and timing
summaries. Both enforced matrix commands exit 0; no fragment assembly is used.

| Family | Moss GM | Moss max | Moss heap max | Defaults GM | Defaults max | Defaults heap max | Verdict |
|---|---:|---:|---:|---:|---:|---:|---|
| `vertex_cache_strip` | 1.156608175 | 1.240929969 | 1.000000000 | 1.157376353 | 1.299164679 | 1.000000000 | pass |
| `vertex_cache_fifo` | 1.234650415 | 1.465943075 | 1.000000000 | 1.208330027 | 1.398317265 | 1.000000000 | pass |
| `generate_vertex_remap` | 1.054421411 | 1.194598146 | 1.000000000 | 1.022292210 | 1.123881357 | 1.000000000 | pass |
| `generate_vertex_remap_multi` | 0.960848947 | 1.074388813 | 1.000000000 | 0.940236906 | 1.023275122 | 1.000000000 | pass |
| `generate_vertex_remap_custom` | 0.977988794 | 1.178649303 | 1.000000000 | 1.086820279 | 1.419409244 | 1.000000000 | pass |
| `remap_vertex_buffer` | 0.843468412 | 1.124842047 | 1.000000000 | 0.864994559 | 1.153643251 | 1.000000000 | pass |
| `remap_index_buffer` | 1.009119886 | 1.169866747 | 1.000000000 | 1.143411121 | 1.408898429 | 1.000000000 | pass |
| `filter_index_buffer` | 0.969029011 | 1.091127328 | 1.000000000 | 0.992944924 | 1.174090712 | 1.000000000 | pass |
| `filter_index_buffer_multi` | 0.918424595 | 1.114666299 | 1.000000000 | 0.957003092 | 1.094156774 | 1.000000000 | pass |
| `generate_shadow_index_buffer` | 1.076521368 | 1.223520524 | 1.000000000 | 1.004433400 | 1.150860058 | 1.000000000 | pass |
| `generate_shadow_index_buffer_multi` | 0.995008493 | 1.145689377 | 1.000000000 | 0.915410488 | 1.132770587 | 1.000000000 | pass |
| `generate_position_remap` | 0.791564529 | 0.940421238 | 1.000000000 | 0.796759747 | 1.006568838 | 1.000000000 | pass |
| `generate_adjacency_index_buffer` | 1.127062085 | 1.238053536 | 1.000000000 | 1.177521680 | 1.290351883 | 1.000000000 | pass |
| `generate_tessellation_index_buffer` | 1.129629321 | 1.246140135 | 1.000000000 | 1.195118151 | 1.286663358 | 1.000000000 | pass |
| `generate_provoking_index_buffer` | 1.099636275 | 1.214545897 | 1.000000000 | 1.181850037 | 1.322432668 | 1.000000000 | pass |
| `vertex_fetch` | 0.973930893 | 1.725607205 | 1.000000000 | 0.988555444 | 1.852204690 | 1.000000000 | recorded maximum residual |
| `vertex_fetch_remap` | 0.936172943 | 1.173748781 | 1.000000000 | 1.024425446 | 1.159305322 | 1.000000000 | pass |
| `simplify_sloppy` | 1.199863725 | 1.481215355 | 1.000000000 | 1.184249656 | 1.420795519 | 1.000000000 | pass |
| `simplify_prune` | 1.055969308 | 1.312070700 | 1.215968444 | 1.038282184 | 1.387150566 | 1.215968444 | pass |
| `simplify_points` | 1.026971321 | 1.274472406 | 0.965909091 | 0.995225587 | 1.230062933 | 0.965909091 | pass |
| `simplify_with_update` | 1.025463929 | 1.401171143 | 1.179795945 | 1.038249094 | 1.418823335 | 1.179795945 | pass |
| `quantize_unorm` | 0.899202966 | 0.946229555 | 1.000000000 | 0.896170226 | 0.936247493 | 1.000000000 | pass |
| `quantize_snorm` | 1.124721556 | 1.184852506 | 1.000000000 | 1.146502273 | 1.183915750 | 1.000000000 | pass |
| `quantize_half` | 0.169378066 | 0.197651799 | 1.000000000 | 0.184537610 | 0.220261680 | 1.000000000 | pass |
| `quantize_float` | 0.717145946 | 0.751098304 | 1.000000000 | 0.735937532 | 0.828524492 | 1.000000000 | pass |
| `dequantize_half` | 0.149545165 | 0.191424054 | 1.000000000 | 0.158687358 | 0.286193292 | 1.000000000 | pass |
| `compute_position_exponent` | 0.964542537 | 1.097489121 | 1.000000000 | 1.006641155 | 1.113894841 | 1.000000000 | pass |
| `simplify_sparse` | 1.029878311 | 1.340327136 | 1.235782117 | 1.037069772 | 1.361104900 | 1.235782117 | pass |
| `simplify_prune_option` | 1.006428270 | 1.271598861 | 1.070429829 | 1.017066552 | 1.344551211 | 1.070429829 | pass |
| `simplify_preserve_folds` | 1.042600304 | 1.329189912 | 1.072378925 | 1.047956039 | 1.391019840 | 1.072378925 | pass |
| `simplify_error_clamped` | 0.994238031 | 1.340610444 | 1.072378925 | 1.008978081 | 1.392738116 | 1.072378925 | pass |
| `simplify_regularize_light` | 1.014921128 | 1.402286736 | 1.072378925 | 1.006067560 | 1.415860508 | 1.072378925 | pass |

The coordinator-authorized residual retains the 1.5 maximum bar:
- `moss` / `vertex_fetch/million/sparse/mode-1`: 1.725607205; failed maximum, coordinator decision pending.
- `defaults` / `vertex_fetch/million/sparse/mode-1`: 1.852204690; failed maximum, coordinator decision pending.
Best retained same-input `moss` ratio: 1.378806161 in `iterator-fetch-trial/records/p01x-benchmark-moss.partial.json`. Historical diagnostics are not substituted for the final source's verdict.
Best retained same-input `defaults` ratio: 1.852204690 in `records/p01x-benchmark-defaults.json`. Historical diagnostics are not substituted for the final source's verdict.

D139 and `fetch-large-direct-profiles` retain exact-input source/binary evidence:
Rust 7,300,049,010 instructions versus C++ 6,883,368,388, with 72.29% of Rust
sampled cycles in fetch and 11.71% in bulk copying. Packed arrays improved the
failed ratio; full initialization worsened it; D143 used the last permitted
safe iterator-append attempt. The same-input D143 epoch reached 1.378806161, but its byte-identical fetch
section records the final-source miss after the separate D144 remap fix. Both
allowed fetch attempts are used; D142 forbids further fetch optimization. Checked
index/source access, initialized append and required copies remain. No tested
revision establishes a full final-source pass within this time box. The counter
gap does not by itself explain the larger wall gap or isolate allocator/cache/
shared-machine effects. This is no
claim that a safe-Rust solution is intrinsically impossible. Stop fetch
optimization and leave the residual decision to the coordinator. Every other
case maximum, family mean and heap bar passes.

Final-source exact-input diagnostic: 6,999,202,257 Rust instructions versus 6,883,368,840 C++ instructions across 100 adapter samples. Its manifest binds the final source/executable identities, changed transport count and exact equal outputs. Counters include adapter/startup work and are diagnostic; these samples are not substituted for the wall-clock bar.

All 32 current-source mutation targets exit 0 after at least 300.000000
elapsed seconds each, totaling 668,359,862 executions, with unchanged source
and executable identities. These seeded mutation/determinism smokes have no
coverage or sanitizer instrumentation and do not establish the RFC release fuzz
gate. Local Linux x86-64 and executed WASM identity are established; AArch64,
other native platforms and release/integration acceptance remain separate.
Shared-machine wall timing is retained with every raw sample and observed load;
no quiet-machine or universal speed claim is made.

The requested artifact directory was unavailable to the earlier managed session.
The committed archive pointers now use `/mnt/linux-extra/meshopt-artifacts/p01x`.
The coordinator moves and verifies the earlier data. Source, reference, dependencies, binding brief/RFC,
executables, raw buffers, logs and verifier/orchestrator code are retained there.
D130 avoids duplicating million payload archives. D136's storage trial was rolled
back: both replaced historical ZIPs were restored with their original hashes
and byte counts. All original ZIPs and supplemental trial evidence are retained.
Historical failed epochs remain
non-qualifying; any unavailable historical executable is identified separately.
Every executable needed for the final verdict is archived and hash-verified.

The scope audit confirms unchanged lane commit metadata, branch and HEAD, and an
empty staged diff. The lane index-file hash changed during this session; no
Git metadata was restored or rewritten to conceal it. Shared common-main metadata
also changed externally during the parallel lane. README, CI and existing 0.1 records
are untouched. No changed/new checkout file exceeds 5 MB; diff checks pass.
Delete only the named build target after retained-evidence verification. Keep
the final cleanup receipt and review archive; do not recreate the target.

D145 cleanup completed: the exact named build target is deleted after retained
evidence verification. `target-cleanup.json` records the verified source archive,
final executable counts and actual deletion; the target was not recreated.

## D146: two-stage maximum-bar decision for final 0.1 integration

The owner approved this rule on 2026-10-04, as relayed by the coordinator,
replacing clean full-profile repeats. Stage 1 is the complete 912-case 0.1.x
and 204-case retained 0.1 matrix under each of the Moss and default consumer
profiles on the rebased source. The family geometric-mean and heap bars use
stage 1 without alteration. For **every** stage-1 case with Rust/C++ ratio
strictly greater than 1.5, run 30 fresh interleaved Rust/C++ pairs on one
quiet pinned physical core, using the same input bytes and archived profile
binaries. Earlier diagnostics never count as stage 2.

For each flagged case, compute the mean and sample standard deviation of the
30 log(Rust/C++) paired ratios. The two-sided 95% Student-t interval uses
29 degrees of freedom. PASS only when `exp(upper) <= 1.5`; FAIL when
`exp(lower) > 1.5`; an interval overlapping 1.5 is INCONCLUSIVE and counts
as FAIL. Preserve every stage-1 maximum and all raw stage-2 samples. Record
the stage-2 interval, verdict, input/source/executable identities, core and
load. A failed or inconclusive case remains a documented residual; a passed
case clears only that case maximum, never a family mean or heap failure.

The maximum over more than 1,100 noisy cases can produce false failures, but
selectively remeasuring only misses biases a naive second reading toward
passing. Requiring the complete fresh sample and its upper confidence bound
makes that selection explicit and conservative. The coordinator can cheaply
reverse this acceptance decision before publication. This rule is recorded
before stage-2 measurements begin.

## P07-D1 — accepted SIMD scope and binding measurement method

Implement only vertex decoding, Oct/Quat/Exp/Color filters and meshlet decoding
in one private audited codec::simd module. Index/sequence and every encoder
stay safe scalar. Keep scalar reference, exact arithmetic, checked arrays and
lowering-only parity dispatch. SIMD_BAR.md is committed before timing.

Owner directive on 2026-10-05 replaces twelve-to-sixty pairs: iterations use
only touched families, about five interleaved pairs; the final complete matrix
runs once, early stops after 5–20 pairs on a paired 95% interval, and sends
only borderline cases to thirty fresh D146 pairs. Bursts last <15 minutes,
then release cores. Pin one core, retain load and raw samples, pause on GPU
lease or active moss-scoreboard measurements. No post-measurement bar edits.

## P07-D2 — sqrt exceptional bits and canonical SIMD adapter

The first raw hardware check fails at 0xbf800000 (-1): SSE sqrt yields
0xffc00000 but pinned libm 0.2.16 without default features yields 0x7fc00000.
Correct rounding does not specify NaN payload/sign. Keep the full-bit check
unchanged: the vector sqrt adapter selects pinned libm's canonical NaN for
negative or NaN inputs and uses IEEE hardware sqrt for nonnegative inputs,
preserving -0 and positive infinity. This repairs the backend, without
changing filter arithmetic, scalar output, oracle pin or tolerance. The raw
instruction alone is not bit-identical over all patterns; the checked adapter
is the unit qualified by the exhaustive run. No reciprocal/rsqrt or FMA.

## P07-D3 — Color conformance retains the inherited scalar contract

All native SIMD ceilings and both wasm builds match scalar Rust and scalar
C++ on the Color sweep. The new harness initially applied EXT's one-unit
allowance to random raw Color words and failed seed 20261005 op18 case 3.
The inherited P04 harness explicitly records Color SIMD distance without
assuming cross-ISA identity (runner04.py:174); Color is outside EXT and the
amendment does not change RFC 5.1's selected scalar Color contract. Upstream
SIMD saturates decoded channels where scalar integer output wraps; arbitrary
raw words can therefore differ by more than one output unit. Preserve that
failed attempt, compare Rust paths exactly on every raw word, retain exact
scalar C++ comparison, and report upstream Color SIMD distance separately.
The EXT allowance still gates Oct/Quat, and Exp remains exact. This is an
explicit inherited-domain classification, never a tolerance between Rust paths.

## P07-D4 — resume the same matrix across short admission windows

The GPU queue repeatedly left windows shorter than the controller's fifteen-
second poll. Preserve the 111 completed native API/case rows and continue
only the remaining rows with a two-second admission poll. This changes no
input, binary, bar, confidence rule, pair count or D146 decision. The original
record is copied before resumption; both controller source hashes are retained.
The continuation checks every original source and executable identity, skips
completed API/case keys, keeps all prior samples/admission telemetry, and still
releases cores between bounded bursts. The wasm controller uses the same
faster poll. This is one matrix with checkpoints, not a repeated full matrix.

## P07-D5 — retain complete Node pair telemetry

Before the Node matrix, inspection found that the original controller checked
admission on every pair but did not retain discarded pairs or stage-2 load
telemetry. Keep that source unchanged as part of the frozen identity, and use
`parity/measure-simd-js.py` as a reporting adapter. It adds those records and
hashes both the original controller and the exact executed controller text.
Pair order, calibration, confidence intervals, 5–20 stopping, thirty fresh
D146 pairs, numeric bars and production binaries are unchanged. The adapter
preserves completed API/case keys if a bootstrap checkpoint exists. A watcher
stops only this lane's controller and children after the complete native
checkpoint, allowing the reporting handoff before continuing Node timing.
The watcher and handoff receipt are retained with the artifacts.

## P07-D6 — independent scalar-baseline confidence assessment

Keep every original matrix sample and mean unchanged. The final S3 assessment
computes both stage-1 and stage-2 intervals independently; either significant
regression fails, so a C++ maximum-bar retry cannot clear an S3 failure.
Include P02's varying Exp inputs in S2/S3, while repeated Oct/Quat remain exempt.

The native controller stopped on the C++ SIMD interval. For S4, four cases
remain borderline against the separate scalar C++ binary. After the Node
matrix, top up only these cases to twenty total interleaved pairs, stopping
as soon as clear. Only intervals still borderline receive thirty fresh D146
pairs. The clearly failed scalar case is not retried. Preserve original family
means, medians and frozen minima; record the supplemental scalar confidence
assessment separately. This is neither a changed bar nor another full matrix.

## P07-D7 — final implementation disposition

The final frozen matrix contains 276 native and 174 Node API/case rows, with
four native and twelve Node borderline-only D146 cases. Independent verification
passes source/binary identities, every archive/member hash, exhaustive coverage,
unique matrix keys, pre-resume row preservation and admitted-pair telemetry.
S4's four scalar-borderline cases clear with nine additional pairs total and
no second stage. All frozen index/sequence minima pass; allocating S4 retains
its clear `index-2-v0-streaming-s4` maximum failure. Caller-buffer S4 passes.

S1, S2, S3 and S5 fail. Publish all gaps and keep the implementation explicitly
unqualified for release or an upstream-parity claim. Native SIMD driver builds
include lowering diagnostics; consumer builds without them are not timed here.
Do not tune code after this once-only final matrix or relabel a failed bar.
The next optimization lane may use these recorded failures for focused work.
Native/wasm local exact parity, arithmetic checks, Miri and bounded ASan smokes
pass; ARM execution, platform identity, release sweeps/fuzz budgets, dedicated
ARM timing and phase-0.6 composition remain the records listed in SIMD_RESULTS.
Retain the source/binary artifacts and manifest outside the prescribed Cargo
target, then delete that target as the brief requires.

## P07-P1 — Profile before the performance changes

The perf lane starts at implementation `2feefb2`, bars `179ba52`, and the
unchanged upstream `4c203430`. Retained binaries, framed caller-buffer inputs,
four hardware counters, disassembly and cycle reports are in
`/mnt/linux-extra/meshopt-artifacts/p07perf/profiles-before`. Counters include
startup, transport, warmup and 3,000 calls; they diagnose costs, not timing bars.
Cycle captures report zero lost samples. The tiny meshlet capture is short and
its attribution is less precise than the resident filter/vertex captures.

| Function | Observed root cause and upstream comparison | Candidate |
|---|---|---|
| Quat | 90.20% sampled cycles in Rust SIMD filter; scalar component gathering and scalar per-component rotated stores surround vector arithmetic. Upstream loads two packed vectors, shifts/sign-extends lanes and rotates four packed u64 records. | Packed loads/stores and per-record rotation, retaining canonical subtraction order and sign-biased truncation. |
| Oct | Generic four-record gathering/scattering; upstream directly unpacks packed byte/halfword lanes. Upstream rsqrt and round-to-even are forbidden by exactness. | Packed unpack/repack with IEEE sqrt/div and canonical rounding. |
| Vertex v0 | 42.31% cycles in separately called group kernel, 51.80% in raw decoder. Rust reconstructs escape masks on stack and stages every prefix/transpose; upstream builds masks in registers, unrolls four groups and shares checks. | Register mask composition, batched group dispatch, direct complete-vector loads/stores. |
| Vertex v1 | 41.32% group kernel, 52.92% raw decoder; 16-bit and rotated XOR reconstruction remain scalar. Upstream vectorizes all three channel forms. | Shared group improvements, vector reconstruction where measured. |
| Views NONE/filtered | Reuse the same vertex and filter kernels; validation remains inside both timed APIs. | Improve constituent kernels; keep view validation and budgets. |
| Meshlet typed/raw | Per-four-vertex dispatch/copy and triangle state spilled through a byte array; upstream keeps its SIMD state in registers and batches packed output. | Inline small dispatch wrappers and keep triangle state in registers. |
| Color | Already packed; exact division, range checks and wrapping output differ from upstream reciprocal estimates/saturating output. | Preserve exactness; measure remaining arithmetic cost. |
| Exp | Already packed and near upstream; no evidence for a rewrite. | Retain kernel. |

None of these upstream paths uses prefetching. Adding a raw-pointer prefetch
would exceed the amendment's two allowed unsafe kinds; no prefetch is adopted.
Keep strict 24-byte group lookahead and checked staged tails. No AVX tier,
estimate, FMA, tolerance, workload or bar change is authorized.

## P07-P2 — Packed kernels and decoder-level dispatch

Replace x86/wasm Oct and Quat's generic component gathering with packed lane
loads, sign extension, vector rounding and packed wrapping output. Quat keeps
the scalar subtraction order, exact sqrt/div, sign-dependent half-unit bias,
and the saved rotation selector. Oct retains zero-length rejection. The NEON
macro path remains unchanged; this host supplies compile checks, not NEON
execution evidence. Exp and Color arithmetic are retained.

Byte groups construct escape masks in registers and use immutable group
configuration constants. Native byte-plane decoding batches four full groups
with a 96-byte window, but every general/tail group retains the required
24-byte lookahead, including a zero-bit group. This is bounded unrolling, not
speculative memory access. Prefix reconstruction reads full planes directly,
uses checked packed stores for stride four, and extracts complete wide-stride
records from vector registers. Short tails still use initialized array staging.
Wasm receives packed byte-group and prefix reconstruction improvements; its
meshlet path remains scalar as prescribed.

Per-group ISA dispatch was still costly after the first byte-loop change.
Move the checked native vertex decode body behind one SSSE3/POPCNT token;
retain the original scalar parser as fallback and keep 16-bit/rotated-XOR
reconstruction scalar. The backend copy preserves header, controls, padded
tail, destination and error checks. This duplication is a maintenance cost:
future scalar parser changes must update and differentially verify the native
body. Meshlet vertex dispatch similarly covers its whole loop, and triangle
state remains in vector registers until output extraction. Wraparound still
falls back to the scalar decoder.

All new unsafe operations are token-authorized target-feature calls. Loads
and stores still use the existing fixed-array pointer seams; no raw pointer
arithmetic, prefetch or unchecked indexing is added. The refreshed audit has
20 individually documented blocks and one allowance. Native SIMD Miri covers
all four integration tests and both integer-kernel tests; safe scalar codec
Miri covers eight tests. Only Miri's large vertex matrix is bounded to nine
boundary counts and strides 4/12; the normal native test retains its full
count/stride matrix. A first oversized Miri run was deliberately stopped and
its log/receipt retained; it is not credited as a completed check.

Iteration records are diagnostic five-pair touched-function subsets. Two
subsets and the final candidate's vertex subset collected zero rows while
admission was occupied; their pause records are retained separately. A first
diagnostic correctness process briefly shared the timing core's SMT sibling
before being moved; that timing record remains diagnostic only. Final
correctness uses other cores. The final controller retains completed keys,
exact executed controller text and all admission decisions. Polling changes
from fifteen to two seconds; inputs, numerical rules, bars, backend order and
stopping rules do not change.

## P07-P3 — Final gaps, best ratios and owner-queued completion

Implementation commit `5b84289` freezes the candidate. No production kernel
changes after the final matrix starts. The owner supersedes the lease-free
gate: run timed commands through the shared queue as `meshopt-timing:p07`,
retry 75, and omit the scoreboard check. Preserve the first three completed
native rows, then finish the single full matrix including unchanged controls.
The new adapters hash the original numerical controller and exact executed
text, verify lease-holder ancestry, retain unfinished rows/calibration/fresh
stage-2 samples, and release at a 690-second pair boundary. The external
840-second command limit keeps each lease below fifteen minutes. Native
finishes in 504.318 leased seconds, Node in 82.754; neither needs a second
leased matrix burst. Their queued commands and the S4 supplement exit 0.
The earlier admission-only pauses and owner-switch receipt remain archived.

Final GMs below use the complete registered case families. Diagnostic best
ratios are matching resident cases from the five-pair iteration records;
they are not substitutes for those family means. The complete before/after
table, all failed bars and platform/release limits are in SIMD_RESULTS.

| Remaining function gap | Final GM allocating / caller buffer; best observed diagnostic | Current profile evidence and decision |
|---|---|---|
| Quat | 1.349 / 1.339; resident 1.327 / 1.364 | Rust/C++ instructions fall 4.715 to 1.285 in matched 3,000-call probes; 88.94% sampled cycles remain in the filter. Packed gather/scatter is fixed; IEEE sqrt/div and canonical rounding remain. No reciprocal estimate or tolerance is adopted. |
| Oct | 1.538 / 1.557; Oct16 resident 1.377 / 1.414 | Current instruction ratios 1.572 (Oct8) and 1.511 (Oct16), versus 3.501 before for Oct8. Filters account for 96.44% / 93.41% of cycles. Exact arithmetic and wrapping output still exceed upstream's estimate-based loop. |
| Vertex | 2.065 / 2.115; stride-four resident 1.649 / 1.777 | v0/v1 instruction ratios fall 2.722/2.718 to 1.817/1.812. Bytes still consume 53.42%/58.47%, prefix reconstruction 43.94%/39.67%. The tiny probe has 44.35% prefix, 28.79% bytes and 13.12% memset; fixed initialized scratch/tails remain costly. Width-two and rotated XOR remain scalar. Keep the checked parser and strict lookahead. |
| Views NONE / filtered | 1.994/2.234 and 1.799/1.690 | NONE has the same 1.817 instruction ratio as vertex; the varied Quat view is 1.699, with 39.16% prefix, 32.72% filter and 23.28% bytes. Validation/copies remain in the public timed API. These constituent profiles explain the remaining work without bypassing validation. |
| Meshlet typed / raw | 1.901/2.401 and 1.726/2.031; typed resident 2.266/2.599, raw 2.046/2.229 | Steady-call instruction ratios are 2.734 / 2.448. Triangle kernels account for 70.03% / 65.69%; checked output extraction and callbacks remain. The final probe uses 300,000 calls, the initial short probe 3,000, so their whole-process instruction ratios are not an A/B comparison. Keep counter-wrap fallback and scalar wasm. |
| Color | 1.508 / 1.547; kernel unchanged | Instruction ratio remains 1.478 versus 1.477 before, with 88.65% in the filter. Exact division, range checks and raw-word wrapping remain required. Historical caller-buffer GM 1.492 remains the best full-epoch ratio; no Color speedup is claimed. |
| Exp / S3 | 0.950 / 0.947; kernel unchanged | Instruction ratio remains 0.875 versus 0.876 before. The C++ comparison is already competitive; significant scalar-Rust regressions in small/diagnostic-ceiling cases still count against S3. ISA/ceiling dispatch and short checked tails are retained. |
| Node S5 | 1.330 / 1.354 versus 2.457 / 2.769 before | Isolated V8 captures attribute 93.4% of vertex and 95.0% of NONE-view ticks to Rust raw decoding; the varied Quat view has 67.6% raw and 26.3% filter. Transfer/wrapper code is a small sampled share. Packed wasm groups/reconstruction help, but checked decode and filtered tails still miss the mean/maxima bars. |
| Allocating S4 | scalar-C++ GM 1.197; two v1 streaming maxima still fail | Exact failed inputs are profiled against scalar C++. Instruction ratios 0.969 (u16) / 0.954 (u32), with 96.06% / 92.06% in raw decode and 2.85% / 5.90% in memset. These 50-call diagnostic probes do not identify a decisive microarchitectural cause for the allocating timing miss or clear it. Index code is unchanged and outside the SIMD optimization scope. Retain intervals 1.531–1.823 and 1.570–1.739 against 1.50. |

Current native/Node instruction, cycle, disassembly and V8 reports are retained
in `profiles-final`, `profiles-tiny-final`, `profiles-wasm-isolated-final` and
`profiles-s4-final`. Cycle reports have no lost samples. Captures started under
the earlier gate retain both admission snapshots, including any overlapping
end; they are diagnostic attribution, not accepted timing-bar pairs. The
remaining captures and S4 follow the owner queue. Profile counters include
startup, transport and warmup; no noisy wall-time result is promoted to a bar.

S1/S2 fail both APIs. S3 has 67 significant case/ceiling regressions: default
19/18 and SSE2 14/16 for allocating/caller-buffer. S4 passes caller-buffer;
allocating fails the two v1 streaming cases above, while every frozen minimum
passes. S5 fails both, with 19/22 failed maxima. Native/Node use seven/six
fresh D146 cases; the S4 supplement adds twelve stage-1 pairs and no D146.
No failed case is retried to select a better ratio, and family means remain
stage-1 medians. The best complete-family ratios for the touched functions
are the final epoch above. The unchanged controls are descriptive comparisons,
not evidence that this patch changed their algorithms.

The independent verifier initially had a fixed 48-row S4 expectation, missing
the frozen million-element controls; replace it with the exact keys derived
from the complete native matrix. That verification failure is retained, then
the corrected verifier passes without changing a sample. An extra S4 profile
adapter initially selected the already-completed native profile route; its
no-op receipt/text is retained separately, then the corrected route captures
both failing index inputs. No performance matrix or supplement is repeated.

Done-when is satisfied by implementation plus residual profile/best-ratio
evidence. Do not claim that the SIMD bars, full release gates, ARM execution,
Windows/macOS identity, dedicated-host timing, Moss integration or phase-0.6
composition are qualified. Retain all receipts/binaries/source archives
outside the exact target, delete that target, and never push this branch.

## P07-R3 — Third performance round: vertex and meshlet structure

Start `e6eac35` on `phase/0.7`; upstream remains
`4c203430ca565cb59a468a91922c76c208169536`. Only vertex reconstruction,
byte-group decoding and meshlet decoding are optimized. Filters, index/sequence,
encoders, bars and frozen inputs are unchanged. This owner's explicit target
`codex-meshopt-p07r3` supersedes the spec's earlier target spelling. Evidence
is retained at `/mnt/linux-extra/meshopt-artifacts/p07r3` outside that target.
No subagents or push are authorized. The first two diagnostics used the owner's
`gpu-lease.sh run meshopt-timing:p07 -- ...`. The coordinator then requires
visible shared admission: from diagnostic three onward, every timed burst uses
`MOSS_HEAVY_GPU=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ...`,
without nesting a lease command. Cancel the still-queued third direct request
before it measures anything. The admission receipts check both wrapper and
lease ancestry, queue telemetry and the declared four-GB peak. Five paired
touched-case probes precede one complete native/Node matrix with existing
early stopping and pair-boundary checkpoints.

The prior source's instruction/cycle captures in `p07perf/profiles-final`
are the initial profile evidence (P07-P3). Source and disassembly comparison
against the pinned kernels finds these concrete structural differences:

| Function | Starting structure and upstream instruction comparison | Round-three change |
|---|---|---|
| Vertex `decodeBytesGroupSimd` | Rust branches for 0/8 bits and converts two-bit headers through a bit-width array; upstream indexes nine header-space configuration rows directly. Both use `pshufb`, two `pmulhuw`, interleave, sentinel compare, movemask, SAD and escape shuffles. Rust's 16-byte escape table entries double upstream's eight-byte table footprint. | Direct v0/v1 header-space indices, branchless 0/literal rows, const-specialized header shifts, and typed eight-byte unaligned loads. |
| Vertex group consumption | Rust derives consumed bytes from `popcnt(pmovmskb(mask))`, serializing the next load behind SIMD extraction. Upstream's x64 `SIMD_LATENCYOPT` instead loads the packed u64, ANDs shifted fields, masks one bit per escape and popcounts it independently. | Same scalar-u64 escape count and explicit advance tables, so the next group's position is independent of the SIMD shuffle result. |
| Byte-plane headers | The original native path tests for 96 remaining bytes but still performs four per-group lookahead checks. Upstream explicitly unrolls four groups under a shared 96-byte check. | Checked `[u8; 96]` window and four explicitly unrolled groups; each consumes at most 24 bytes. General/tail paths still reject any group with fewer than 24 remaining bytes, even zero groups. |
| Vertex reconstruction | Byte prefixes are computed in four component planes before transposition; halfword and rotated-XOR channels remain scalar. Upstream transposes first and reconstructs all three channel forms using packed vector records. | Packed four-record prefix kernels for byte/halfword sums and rotated u32 XOR, complete checked stores and scalar-sized tail writes. |
| Meshlet triangle groups | Separate shuffle/increment vectors live in a 48-byte tuple; upstream packs increment bytes into the unused first six shuffle bytes and derives increments with `pslldq 10`. Rust extracts an edge-format value and invokes an output callback per triangle. | Packed 16-byte masks and separate byte metadata, register output extraction, then a decoder-level grouped-output path for typed and raw destinations. |
| Meshlet validation/state | Rust extracts the byte counter from SIMD state and repeats group, lookahead, output-index and odd-tail decisions. Upstream keeps vertex/triangle state in registers and writes complete groups directly. Scalar Rust's u32 counter must still be preserved when the byte state reaches 256. | Single checked stream walk, vector-resident vertex state, const-sized full-group writes, separate counted tails, exact final consumption and unchanged scalar fallback on counter wrap. |

The first five-pair probe exposes a slower initial vertex rewrite: resident
stride-12 allocating/caller ratios are about 2.7/2.8. This failed diagnostic
and its exact sources/binaries are retained in `iteration1`; they are not
final evidence. The subsequent probe adds the independently loaded u64
consumption path and smaller shuffle tables, and interleaves the starting
Rust binary as an additional diagnostic backend. There is no unchanged-source
retry to select a nicer timing result.

All unsafe operations remain in the audited module: token-authorized calls
and fixed-array unaligned loads/stores. The eight-byte load is a new typed
seam with its own argument; grouped meshlet wrappers are token calls. No raw
pointer arithmetic, uninitialized storage, unchecked access, prefetch, AVX,
estimate, tolerance, arithmetic filter change or corpus/bar alteration is
introduced. Meshlet malformed-prefix writes can occur before exact final
consumption rejects the stream; successful bytes and typed errors remain
canonical, and error prefix bytes are outside the contract. wasm meshlets
retain scalar decoding.

Ordinary regressions cover all 256 pair codes, odd tails, counter-wrap
fallback, both output widths and both APIs, and all 256 byte headers in each
of the three format modes at strict lookahead thresholds. Miri's meshlet
regression bounds pair-code execution to all nibble values plus mixed
reuse/restart orders; the native and differential matrices remain complete.

The grouped-output/preflight trial is rejected: the second interleaved probe
shows meshlet new/starting-binary ratios 1.55–1.81. Its disassembly exposes
repeated sink slice replacement, dynamic-sized memcpy and a second complete
metadata scan. `iteration2` preserves the trial's sources/binaries and samples.
Replace it with const-sized full-group output, separate counted tails, a single
stream walk and scalar counter tracking. Final exact consumption determines
malformed length errors; every individual memory load remains checked.
The starting binary in the same probe confirms vertex improvements: new/old
0.753–0.775 for resident stride four and 0.919–0.971 for stride twelve. The
probe is diagnostic only and does not replace complete-family means.

The first visible-admission adapter misinterprets `MOSS_HEAVY_ADMITTED` as a
text receipt: it is the resolved command executable. It exits before any pair
in `diagnostic3-adapter-rejected.log`. Read the live `heavy.reservations` row
instead, check its wrapper PID/reserved GB and the ancestor's four-GB timeout
command, and hash the admitted executable. The numeric harness and compiled
sources are unchanged; no completed pair is retried.

Shared admission also retains the frozen scoreboard exclusion: an active
when-idle unit with a sleep child is eligible; an actual scoreboard command
pauses/discards pairs. The owner changes GPU admission, not this condition.

The wrapper may adapt its reservation using measured class history. Preserve
that actual value separately from the owner's four-GB declaration; the scope
cap must cover at least four GB. Never override or misreport its sizing.

### Final admission and recovery receipts

The shared queue admits the first native full-matrix burst ahead of the
queued third probe/profiles. It checkpoints 153 rows after 690.923 seconds;
queue order is recorded as observed, not presented as FIFO. Later the third
probe runs exactly five pairs per API/case, then the 30 profile captures.
The unchanged frozen candidate takes 0.580–0.635 of the starting binary's
meshlet time in that probe. No code is changed after final sampling begins.

A native continuation saves four further rows, then its post-pair reservation
read sees the coordinator's transient file rewrite and raises StopIteration.
It saves an empty unfinished row: no sample from the interrupted pair is
accepted. The raw times from that one interrupted pair are unavailable; its
trace and explicit discarded marker are retained. Retry reservation reads for
200 ms and decline admission if still absent, allowing subsequent discarded
pairs to retain raw times. All 157 accepted rows remain unchanged; resume the
unfinished case with its original calibration, never replay a completed row.
The next burst completes all 276 rows in 2m09s. Historical helper versions are
archived and checked against the earlier segments' hashes.

Node's first launch exits before any sample because the arithmetic timing
modules were not prepared. Build the existing qualification crate's two
wasm modules into a separate `node-bin`, preserving all six native/parity
binary identities. Scalar Node timing remains diagnostic lowering inside the
SIMD-enabled module. Untimed preflight verifies all 87 eligible inputs through
both modules and APIs against the canonical golden outputs. Resume its zero-row
record with the corrected paths; retain the failed executed adapter and log.
These are controller repairs, not changed cases, bars or retries of failures.

### Remaining gaps and best retained result

One complete native matrix gives allocating/caller family means: vertex
**1.832/1.967**, meshlet **1.329/1.534**, raw meshlet **1.228/1.315**.
The starting means are 2.065/2.115, 1.901/2.401 and 1.726/2.031. These complete
family means are the best retained round-three epoch; the paired probe is
separate diagnostic evidence. No failed case is retried for a nicer result.
The remaining SIMD bars are not waived.

The final counters below compare fixed caller-buffer inputs and repeat counts,
with exact identical outputs from starting Rust, current Rust and C++ SIMD.
Counters include startup/transport/warmup and are not case-bar evidence.
All 30 captures exit 0, under the shared queue, with source/binary identities.

| Input | Instructions new/old | Cycles new/old | Instructions new/C++ | Cycles new/C++ | Branches new/C++ |
|---|---:|---:|---:|---:|---:|
| vertex-v0-resident-s12 | 1.369 | 0.961 | 2.487 | 2.249 | 10.018 |
| vertex-v1-resident-s12 | 1.375 | 0.997 | 2.491 | 2.334 | 9.692 |
| vertex-v1-resident-s12-level9 | 1.375 | 0.934 | 2.491 | 2.186 | 9.692 |
| meshlet-64-126-v4-t3 | 0.628 | 0.574 | 1.718 | 1.467 | 3.252 |
| meshlet-raw-64-126 | 0.630 | 0.593 | 1.542 | 1.293 | 2.201 |

For the resident v1 vertex case, 60.54% of cycle samples fall in packed
reconstruction (`deltas8_kernel`), 33.79% in byte-group decoding and 4.89% in
block parsing. Independent consumption shortens the serial byte-load chain,
while the new four-record reconstruction/scatter and checked iterations cost
more instructions: roughly 1.37 of starting Rust and 2.49 of C++ in these
stride-12 cases. The assembly retains dynamic strided scatter/tail/iterator
branches and checked window advances. C++ explicitly unrolls all sixteen
record writes; Rust retains counted output loops. Thus the smaller tables and
latency path improve timings without eliminating this instruction-count gap.
A further reconstruction rewrite remains justified by this profile; this round
freezes its candidate before the one final matrix rather than retiming a patch.

For typed meshlets, 84.86% of samples lie in the whole grouped kernel, and
roughly 13% in the driver/API path. For raw meshlets, the grouped kernel takes
76.40% and the driver 23.43%. Const-sized writes and one stream walk remove
37% of starting instructions and 41–43% of cycles. Remaining bounds/count
branches, the canonical u32-counter fallback guard and output packing explain
extra work against C++: 1.718/1.542 instruction ratios and 3.252/2.201 branch
ratios for typed/raw. Upstream's three-byte output combines two pairs into
4+8-byte writes; Rust emits exact six-byte pairs with counted tails and no
padding writes. Preserve these residuals and the best complete means instead
of claiming the bars or a dedicated-host qualification.

The first S4 request is premature: its unchanged numeric controller requires
both matrices complete and exits before sampling while Node is still queued.
Retain that receipt; submit its actual assessment after Node finishes.

S4's actual dependent run completes in five seconds, with twenty added pairs
among three borderline candidates and no fresh S4 D146. Native/Node finish
sixteen/seventeen D146 cases. The independent verifier checks all 276/174
unique rows, exact accepted-sample counts, recomputed intervals/early stopping,
source/binary/golden/archive identities, helper-version proofs and static/Miri
receipts. It passes. S1/S2 and S5 still fail both APIs; S3 has nineteen
significant case/ceiling regressions. S4 passes caller-buffer and all frozen
minima; allocating fails three maxima. Preserve these outcomes rather than
claiming full bar, release/platform, Moss or phase-0.6 acceptance.

## P07-F4 — execution from e41f4bb

The explicit brief authorizes six separate fix commits; no push, merge or
rebase. Frozen inputs and SIMD_BAR stay unchanged. The brief's 1.25/1.50
summary differs from registered S1 1.10/1.30 and S5 maximum 1.60; assess
both, without replacing either. Initial disk admission is 16 GiB free, below
the required 25 GiB. Build/counter/parity/Miri/timing work is paused while
code work continues; no alternate target or foreign cleanup is authorized.

### Fix 1: native reconstruction

Complete sixteen-record groups now use four explicit prefix/scatter calls,
with four explicit stores each and one checked complete destination span.
Partial groups retain initialized staging and counted live stores. All three
channel operations reuse exactly the existing integer arithmetic. No new
unsafe block or padding write. Validation remains pending disk admission;
source structure alone does not establish removed machine-code spills.

### Fix 2: wasm block dispatch and integer channels

Wasm now selects its vertex kernel once per call, batches four byte groups
under the existing 96-byte proof, and retains 24-byte lookahead for shorter
windows. Byte, halfword and rotated-XOR reconstruction transpose first and
use packed four-record prefixes plus explicit complete-group stores. Parser
layout and errors are copied from the validated native path. This replaces
per-group runtime dispatch and sixteen per-plane prefix stages. No new
unsafe seam. Native Miri cannot establish wasm correctness: executed wasm
parity, counters and codegen inspection remain mandatory and pending disk.

### Fix 3: canonical short reconstruction and tails

Only complete groups enter the vector transpose. The remaining zero to
fifteen records use the shared canonical scalar reconstruction with original
plane spacing and the final SIMD prefix as their baseline. This removes four
partial-plane staging copies and partial scatter loops on every block tail,
including the frozen seventeen-record inputs, rather than choosing a timing
threshold. Tiny calls with no complete group never stage vectors. Count zero
executes neither path. Existing boundary/tail regression coverage is retained;
S3 is still unestablished until the final admitted pass.

### Fix 4: exact Color range reduction

Replace eight component range comparisons and their mask chain with
component minima/maxima and two endpoint comparisons. All multiply/divide,
association, +0.5 truncation and packing remain unchanged. Alpha zero
produces NaN in the final alpha component: native min/max keep that final
operand; wasm min/max propagate NaN. Thus exceptional lanes still take the
scalar fallback. Oct/Quat nearest/estimate/sign-bit shortcuts are rejected:
no safe evidence establishes their signed-zero/rounding identity. Their
arithmetic remains unchanged, so unchanged exhaustive Oct/Quat code is not
credited as a fresh result. Color needs seeded/edge validation and counters.
Disk recovered to 90 GiB before the first build. All-feature native tests
pass for fixes 1–3; per-fix acceptance still awaits exact archives/counters.

### Fix 5: four-triangle meshlet output

Two complete pairs share one canonical counter-limit guard and one output
span. Typed stride-three output packs exactly twelve bytes, explicitly
masking the first pair's counter byte before joining pair two; stride-four
and raw output pack sixteen bytes. Independent input lookahead checks remain
because consumption is data-dependent. Separate pair/odd tails keep exact
counts. Counter overflow still abandons SIMD for the scalar u32 semantics.
No new memory seam: vector packing uses the existing array store. Existing
all-code/odd-tail/counter-wrap regression is required before acceptance.

### Fix 6: safe sequence cursor

Sequence retains a remaining slice and checks its five-byte first chunk
instead of numeric position-plus-five on each index. Single-byte values
advance one byte directly; multi-byte values preserve all four continuation
steps, overlong acceptance, wrapping baselines and the exact four-byte final
tail. Triangle index decoding is unchanged. No unsafe code is added.
The historical 2.888 allocating maximum is not explained by this edit:
retained diagnostic evidence shows a 1.300 instruction ratio and comparable
memset shares. Collect matched counters and final admitted timing before
deciding whether it persists; do not infer an allocation cause from time alone.

### Frozen candidate, validation and final timing scope

Implementation fixes are separately committed as a48d060, 70727c3, 72e03af,
f72221b, b0f0377 and eb6bac6. The four-triangle regression extension and
timing adapter are 52f22d5. Production source is frozen before timing.
Evidence root: `/mnt/linux-extra/meshopt-artifacts/p07-fix4`. All 138 archived
inputs are hard-linked unchanged from p07r3 and checked by SHA-256; the
pinned upstream checkout passes check-reference.sh. Frozen benchmark parity
passes both APIs, scalar/SSE2/SSSE3/SSE4.1 and executed wasm without/with SIMD.
Fixtures pass 869 cases and malformed streams 7,653. All-feature tests,
unsafe-free tests, native/wasm SIMD Clippy and the expanded five-test Miri
integration run pass. Integer/sequence Miri and the larger seeded sweep are
still running when final timing enters the visible queue. They are pinned
to cores 24/25, separately from measurement core 26; affinity receipts are
retained. No new unsafe block: boundary/package gates retain 23 blocks and
one module-level allowance. No broader platform/release acceptance is claimed.

The qualification binaries were built before a test-only pair-transition
extension. build.json retains the original compiled_sources and records the
new integration-test identity separately, with an explicit non-compiled source
update; no linked library or driver source changed. The added ordinary test
and subsequent Miri run both pass. The counter parser initially checked the
wrong perf CSV column, accepting no aggregate record. It was corrected to
verify 100-percent event scheduling, and all captures were recollected.
Counter reruns are diagnostic, not timing retries.

Current native counters against starting Rust: resident vertex stride12/32
about 0.49 instructions, varied Quat view 0.55, tiny vertex 0.74, repeated
tiny Oct view 0.80, Color 0.86–0.87, typed meshlet 0.95, raw meshlet 0.86,
sequence 0.808 under both APIs. Reconstruction and tail effects are combined
in these source-matched comparisons, not attributed as isolated A/B fixes.
Native delta stack frame falls 0x168 to 0x98; vector stack stores fall twelve
to one (tail baseline), with 104 to 37 stack operand sites across the whole
function. Static sites include errors/tails; general-purpose address spills
and checked-span branches remain. Wasm public/raw resident probes retire
about 0.66–0.67 starting Rust instructions. V8 raw code is smaller, but work
moved to a 32,976-byte vertex decode body plus prefix/scatter helpers; do not
claim a whole-path code-size or spill reduction from raw-function size alone.

Final timing measures 94 touched native cases / both APIs (188 rows): vertex,
NONE/filtered views, Color, typed/raw meshlets and sequence. Standalone
Oct/Quat/Exp and triangle index timing are outside this scope. Node measures
73 touched eligible cases / both APIs (146 rows): vertex, sequence and views.
The 292 Node timing-module preflight outputs match canonical golden hashes.
Scalar Node timing remains diagnostic lowering in a SIMD-enabled module.
Adapters retain 5–20 paired early stopping, fresh thirty-pair D146 only for
borderline cases, both registered and brief maxima, complete telemetry and
690-second pair-boundary checkpoints under the exact 840-second heavy wrapper.
Sequence scalar-C++ S4 intervals participate in the same stopping decision,
so no separate later S4 timing pass is needed. Starting Rust is interleaved
only in sequence rows to investigate the historical allocating maximum.
Native and Node queue requests use the required four-GB declaration; actual
reservation and cap are distinct wrapper receipts. No nested lease wrapper.
Unchanged standalone Exp S3 failures are retained as historical unclosed gaps,
not erased by a scope-limited new S3 result.

### Expanded-seed failure retained

The larger optional sweep requested 10,000 cases per family, rather than the
prior 1,000-case SIMD smoke: P02's 70,000 cases pass. The strict P04 comparison
stops after 99,264 completed cases at seed-20261005-op20-9264. Its mutated
odd-tail code is 0x1c: a nonzero unused high nibble. Pinned upstream scalar
decodes five vertices / one triangle successfully; upstream SIMD consumes
both triangle nibbles through decodeTriangleGroup and ends beyond bound
(-3). Current native ceilings, both wasm builds and both starting Rust
binaries accept identical scalar bytes. Reproduction input, all statuses and
output hashes, the failed log and complete partial ZIP are retained. This is
a pre-existing upstream scalar/SIMD malformed-tail disagreement, not a fix-4
regression; the expanded strict sweep remains FAIL. No harness exception,
smaller replacement sweep or altered frozen input is used to make it green.
The verifier explicitly retains this shortfall. All frozen comparisons pass.

All seven static/test/Miri receipts now pass. Native burst one checkpoints at
its 690-second pair boundary and releases after 11m34s with exit zero. Resume
only unfinished rows and accepted partial samples; no completed row is rerun.
Node remains in the visible shared queue. Allocation-sensitive sequence gaps
persist in the single pass; the row's matched starting Rust is retained.
Further counter-only cold-packet/fault diagnostics were collected during a
subsequent queue window with neither own timing job admitted, on core 25.
No admitted sample overlapped these diagnostics; wall values in their binary
responses were ignored. This supersedes the earlier plan to collect after timing.


### Sequence allocation cause: bounded investigation, unresolved

The safe cursor fix remains implemented. Counter differences for streaming
v1 stride-four show 44.25 million new Rust versus 42.36 million C++ user
instructions per cold allocating decoded call (1.045), compared with 54.73
million starting Rust. Cold packets include transport/setup per two calls;
these are counter diagnostics, not equal-interface timing-bar measurements.
Warm-loop differences remove almost all page faults; cold allocating Rust
has about 3,074 minor faults per decoded call versus C++ 4,080, while caller
buffers have about 3,057 on both sides. Fault volume therefore does not
explain extra Rust allocating time. Differential counts for rare faults may
be negative from startup noise and are not interpreted as physical rates.

Sampled minor-fault stacks locate Rust faults in output initialization and
response copying; C++ faults also occur in warmup/timed output zeroing and
response copying. The C++ timed initialization has 6,144 represented faults
across three calls (2,048 per call); Rust's combined warmup/timed allocation
samples are consistent with the same initialization volume. Rust unwinding
stops before separating these calls. User-cycle counts exclude kernel fault
service, and host allocator/clock effects remain unisolated. perf kernel
cycles are unavailable under current host permissions; no host setting was
changed. Preserve sequence-packets.json, sequence-fault-summary.json, raw
stat/data/scripts and source/binary hashes. Do not claim extra Rust zeroing,
page-fault service, allocator choice or scheduling as the established cause.

Abandon further cause attribution within this bounded fix-four pass: the
matched final sequence rows retain starting/new Rust and C++ and can establish
whether the slowdown persists, but instruction/fault evidence cannot establish
its cause. No extra admitted timing experiment or failed-case selection retry
is introduced. This subgoal remains explicitly unresolved with evidence.

MSRV 1.88 no-std SIMD and wasm simd128 no-std compile checks pass. Each emits
three existing unused-helper warnings in its no-std backend. Their exact logs
and zero exits are in portability-checks.json; no extra target execution is
claimed. The seven main test/static/Miri receipts remain unchanged.


The native resume preserved rows, partial samples and controller segments,
but its adapter rebuilt the top-level record without copying lease_bursts;
the final JSON retains only the last burst's exact elapsed value. Retain both
original wrapper logs and a separate timing-bursts.json derived from those
logs (whole-second wrapper precision). The first native wrapper reports
11m34s, the second 1m05s, both exit zero. The lost first exact fractional
elapsed value is not reconstructed or claimed. Per-pair holder/ancestor,
reservation and queue telemetry remains intact across both segments.


### Final scoped outcome and stop

The independent verifier recomputes unique scope membership, archived source,
binary/input/BAR identities, parity ZIP members, zero-exit static/Miri receipts,
paired intervals/early stopping/D146 eligibility, golden Node preflight and
admission telemetry. It passes while preserving the expanded P04 FAIL.
Final native/Node matrices are 188/146 rows. Native S3 has zero significant
comparisons in either stage across touched cases/default/SSE2; the two
untouched historical standalone Exp regressions remain unclosed. S1/S2/S5
still fail. Sequence-only S4 caller-buffer passes; allocating fails two maxima
although all minima pass. Exact family/case numbers are in the new scoped
P07_FIX4_PERFORMANCE.md and SIMD_RESULTS.md, not substituted for the historical
full-matrix report.

Retain all six implementations. Typed meshlet means 1.332 allocating / 1.564
caller-buffer are slightly worse than 1.329/1.534 despite a 0.95 instruction
ratio. No late unmeasured revert is performed. Color arithmetic is exact and
only range checks simplify; further cheaper Oct/Quat divide/sqrt/round work is
abandoned without a proof. Tiny tails use canonical scalar reconstruction;
remaining checked setup/dispatch/maxima are recorded, not patched by an
unmeasured size threshold. Sequence maximum persists at 2.876 (same-pass
starting 3.205, new/starting 0.897), with its cause unresolved as recorded.

Both native bursts and Node exit zero, wrappers report 694/65/153 seconds,
all under 840. The first native fractional receipt loss is disclosed above;
accepted rows/partials and per-pair proof are intact. No final case retry,
new timing epoch, broader full matrix or remaining host/platform qualification
is attempted. Implemented fixes, explicit abandoned cause attribution,
complete timing and recorded failures satisfy the requested stop condition.
Archive identities and cleanup receipt, delete only the exact prescribed
target, commit records with configured repo author and stop. Never push,
merge or rebase.


Cleanup complete: only `/mnt/linux-extra/moss-cargo-targets/codex-p07-fix4`
(1.1 GB) is removed after all own build/test/timing sessions finish. Archived
binaries are present; cleanup.json confirms target absence and before/after
disk receipts. No foreign target, artifact, cache or lease is removed.


## P07 fix-five — scope and Color conversion proof

Start f5bdb698afd879668d4959261ab3e519e9cdbfc5, clean phase/0.7 worktree.
Follow SPEC-p07-fix5 and the fix-four development-first/admission/cleanup rules.
No subagents or integration actions. Registered SIMD_BAR.md and all 138
benchmark requests stay byte-identical. S1 is 1.10/1.30, S2 1.25/1.50,
S4 1.25/1.50 plus frozen minima, S5 1.25/1.60; filtered views belong to S2.
Artifacts: /mnt/linux-extra/meshopt-artifacts/p07-fix5; only build target
/mnt/linux-extra/moss-cargo-targets/codex-p07-fix5, removed after receipts.

Color instruction diagnosis: the old packed kernel performs six FP min/max
reductions and two range comparisons on each four-record batch. Replace
that with an integer alpha guard, preserving all FP arithmetic and truncation.
For 8-bit inputs, absolute RGB integer components are at most 511; nonzero
alpha guarantees scale >= 1, so every rounded value fits i32. For 16-bit
inputs, absolute components are at most 131071; alpha >= 4 guarantees scale
>= 7 and magnitude below 131071 * (65535/7) + 0.5 < 2^31. Other batches
use canonical scalar conversion, including NumericalFailure and integer wrap.
The bound is sufficient, not a new input restriction. Native instructions
per decoded byte at resident stride 4/8 fall to 0.898/0.905 starting Rust.
The added alpha-boundary/wrap regression and existing extreme/tail tests pass;
development wasm exact outputs pass on 42 filter/view cases. No approximate
division, saturation or changed rounding is introduced. Final all-level
parity, Miri and one leased timing epoch remain required.


### Fix-five meshlet instruction/store repair

The starting 12-byte typed triangle copy spills one vector to the stack.
Emit native words explicitly, consume typed output spans, and SIMD-shuffle
16-bit vertex references into contiguous packed output. Four-byte metadata
records remove multiply-by-three addressing. Keep counter-wrap fallback,
per-window bounds, odd tails and exact caller tail preservation. Raw sinks
retain indexed output: consuming their spans increased representative
instructions from 3.418 to 3.750 per byte and was rejected. The revised
raw path takes 3.251 (0.951 starting); typed v4/t3 takes 4.461 vs 4.830
(0.924), with zero vector stack stores vs one in the starting kernel.
Focused SIMD/codec/codec04 tests and all 138 benchmark exact outputs pass.
Safety model is unchanged; no new unsafe block. Final Miri/parity and timing
remain required. Further unchecked input/output spans are rejected.


### Fix-five standalone filters and wasm filtered views

Unroll the native exact Exp four-wide transform four times per loop.
Resident-s4/streaming-s12 work is 0.936/0.939 starting Rust. The old SIMD
and safe-scalar instruction counts are identical, so historical Exp S3
failures require fresh paired evidence, not an instruction-only clearance.
Keep the scalar bit construction and every tail; no reciprocal estimates.

Wasm Quat previously extracted four packed records to scalar memory and
rotated variable u64 words. Use two vector byte-swizzles with record-local
selectors, preserving exact 16-bit rotation and all arithmetic. Existing
rotation selector tests plus 42 development wasm filter/view goldens pass.

Native Oct/Quat arithmetic is not approximated: sqrt/div precision and
scalar rounding remain authoritative. An API-wrapper inlining trial
changed resident work negligibly and tiny vertex only 0.997 starting;
revert those annotations before the single final timing epoch. Do not
introduce an input-size threshold selected from historical failed cases.
Native S1 and remaining filtered-view maxima are explicitly still at risk.


### Fix-five sequence allocation investigation and bounded initialization

Allocation interposition and mmap/munmap/brk traces on the frozen v1
streaming-s4 request establish exact 8,388,612-byte output allocations,
not capacity growth. Both drivers allocate a new output while the old one
is live. C++ releases its old vector before decoding; Rust assignment
releases it after decoding. Rust additionally reserves an unused caller
buffer in the allocating benchmark path. Those are harness differences,
not established causes of the 2.876 time ratio; retain the frozen harness
instead of changing lifetime/allocator policy to select a passing result.

The production allocating API did zero-fill every output page before
writing decoded records. Decode into initialized 64-record stack blocks
and append into the pre-reserved/accounted exact-capacity Vec instead.
No MaybeUninit, unsafe initialization, capacity growth, private allocator
or changed byte/work accounting. The per-record append trial was rejected:
streaming allocation work 7.25 vs 5.25 instructions/byte. Block append
takes 5.492 (about 1.046 starting); caller-buffer stays 5.25. Cold packet
minor faults stay about 3,074 vs starting 3,074, and C++ about 4,081.
Warm faults are near zero. Cold user instructions rise 44.25M to 46.28M
while warm user cycles fall 9.24M to 8.87M (diagnostic, noisy, not a bar).
Removing a separate zero-fill pass therefore does not prove a page-fault
count explanation. Kernel page-fault latency is not measured. Final matched
starting/new/C++ sequence rows must determine acceptance; if the maximum
persists, cause attribution remains unresolved rather than fabricated.

Block-boundary regression covers 63/64/65/127/128/129 records, versions 0/1,
widths 2/4, exact retained capacity/resource limits, invalid headers,
unsupported versions, extra encoded bytes and untouched caller tails.
Focused codec/SIMD tests and 138 native benchmark goldens pass.


### Fix-five bounded tiny vertex setup and checked triangle advances

Use a 128-byte initialized plane buffer for at most two 16-record groups;
keep 1024 bytes for larger vertex calls, on native and wasm. This boundary
follows plane layout (3*block + ceil(block/16)*16 <= 128 for block <= 32),
not a selected timing crossover. Native tiny v1/s12 instructions fall
22.397 to 21.446 per byte (0.958), tiny filtered Oct/s8 30.632 to 29.522
(0.964). Add 31/32/33 records to the existing version/stride/malformed
regression, including stride 256 natively and both group boundaries in Miri.

Triangle metadata advances are at most six by construction. Expose that
bound with &7 so the 16-byte checked lookahead proves subsequent cursor
slicing safe; exact metadata consumption is unchanged. Representative
typed/raw work is now 4.279/3.314 vs starting 4.830/3.418; branch work
0.423/0.343 vs 0.571/0.426 per byte. Checked loads, counter wrap fallback,
odd tails and audited unsafe blocks remain unchanged. Finite counter/tests
are development evidence; registered maxima still need the one final pass.


The first all-target Clippy gate rejects the tiny wrapper's two alternative
ISA calls inside one unsafe block. Choose the private target-feature kernel
before entering the block; the single call remains authorized by Ssse3.
No lint suppression or unsafe function declaration is added. Retain the
failed log and pre-fix identities under pre-lint-fix, rebuild and rerun
source-bound correctness/static checks before timing.


### Fix-five final timing scope and verifier

Measure 124 native cases (248 API rows): vertex, NONE and filtered views,
standalone Oct/Quat/Exp/Color, typed/raw meshlets and sequence. Node scope
is the same 73 eligible cases / 146 API rows as fix four (vertex/views/
sequence); upstream JS exposes no standalone filters or meshlet API.
Use only registered maxima for early stopping; no second generic 1.50
brief threshold on S1/S5. Five to twenty pairs and fresh borderline-only
D146 remain unchanged. Sequence also interleaves fix-four Rust in this
same pass. Both controllers checkpoint accepted pairs, stage-two partials
and every prior lease receipt; release at the pair-boundary 690-second
budget and requeue under the visible heavy 4GB timeout-840 wrapper.

No 276-row full matrix or case-selection retries. Family means and S3
remain independent gates. Independently verify source/binary/BAR/input
hashes, exact frozen archives, expected upstream tail split, Node output
hashes, unique scope, stopping intervals, D146 eligibility, paired lease
telemetry, wrapper exits and safety/test/portability receipts. Before is
fix four where measured and the clearly labeled fix-three standalone
Oct/Quat/Exp epoch otherwise. No missing release/platform gates are cleared.


### Effective-flag correction before timing

V8 instruction diagnostics expose a harness configuration defect: new
arithmetic wasm work rose about fourfold and both modules lacked the
custom-section simd128 feature marker. Cargo gives CARGO_ENCODED_RUSTFLAGS
precedence even when it is the empty string. The generic environment reset
in codec/measure.py therefore neutralizes RUSTFLAGS set by qualify.build
and by the first round-five Node/check controllers. Preserve those records
under pre-effective-flags and do not credit them as SIMD/Miri ISA proof.
Fix qualify.build to propagate the requested options through encoded flags
and assert the compiled wasm feature marker. The archived Node builder,
unsafe-free, wasm Clippy, Miri and portability controllers do the same.

Round-four Node timing images do contain simd128; they remain the S5 before
epoch. Round-four codec wasm-simd.wasm lacks the marker, so its executed
fixture proof does not establish simd128 execution despite the recorded
intent. This round reruns all frozen fixtures against actual scalar and
SIMD wasm modules. The corrected Miri invocation forces SSSE3/POPCNT/SSE4.1
instead of relying on a neutralized flag. No final timing has run yet.

### Common Color bit depths while queued

Cancel only our two waiting heavy requests before accepting any timing pair;
retain their queue-only logs. Add exact common-depth scales: all 8-bit alpha
words >=128 imply scale 255; all 16-bit alpha words in 2048..4095 imply
scale 4095; all >=32768 imply scale 65535. Fixed-scale f32 division rounds
identically to the existing generic calculation. Mixed depths still propagate
bits and divide, and the sufficient conversion guard/scalar fallback stays.
No approximate arithmetic or input restrictions. Boundary/mixed-lane/tail
regressions cover 127/128, 2047/2048, 4095/4096 and 32767/32768. An archived
untimed proof checks all valid alpha depths with extreme channels across
native levels and actual wasm scalar/SIMD. Frozen proofs and Node preflight
pass; complete effective-flag Miri/static checks remain required before credit.
Final N/2N instruction counts and source/binary identities are in counters.json;
final.asm retains generated code. No timing was assessed during this change.

### Fix-five final result and stop decision

Complete one scoped epoch: 248 unique native rows and 146 Node rows, each
in one visible heavy-4GB timeout-840 burst. Wrapper receipts are 316 and 117
seconds, exit 0; no accepted pair or completed row repeats. Native uses 19
borderline-only fresh D146 rows; Node uses 9. Independent verification
recomputes intervals/stopping rules, preserves stage-one family means,
and checks every source, binary, input, lease and wrapper identity.
Artifacts: /mnt/linux-extra/meshopt-artifacts/p07-fix5.

All 869 frozen fixture cases, 7653 malformed cases and 138 benchmark identities
pass at native ceilings and actual wasm scalar/SIMD; 292 Node output hashes
pass. Exhaustive valid-alpha Color depths and the expected upstream tail
split pass. All 7 effective-flag test/Clippy/Miri receipts, portability and
23-block single-unsafe-module/package gates pass. The earlier ineffective
flags remain disclosed, and do not supply final SIMD/Miri evidence.

Color passes both S2 APIs: means 1.015/1.064. Sequence passes S4 scalar-C++
and every P02 minimum: means 1.134/1.045. Within-pass v1 streaming-s4
allocating candidate/fix-four is 0.514; old/C++ is 1.815 and new/C++ 0.933.
The exact historical 2.876 maximum does not reproduce. Allocation growth
and minor-fault count explanations are contradicted by development traces;
eager initialization is removed. Kernel/allocator contributions to the
historical peak remain unisolated and cause attribution is abandoned.

Registered aggregate S1 fails both APIs: means 1.120/1.151, 7/8 maximum
failures. S2 means 1.160/1.104 pass its mean threshold, but 7/5 maxima fail.
Oct remains a mean/maximum failure, typed meshlets remain mean/maximum
failures, allocating vertex/NONE views retain tiny maxima, and raw caller
meshlets retain a mean/maximum failure. Allocating Exp streaming-s32 is
3.252 with interval 1.611–4.250. The historical Exp caller resident-s4 and
streaming-s12 S3 failures no longer show significant slowdown, but two
other caller Exp stride-32 cases fail S3: tiny 1.0294–1.1441 and resident
1.0905–1.1188. Node S5 means 1.179/1.168 pass its mean threshold, but 7/8
maxima fail. See P07_FIX5_PERFORMANCE.md for every case and verdict.

Keep the measured implementation and all negative evidence. Abandon further
native Oct/Quat normalization changes without a demonstrated cheaper exact
IEEE operation; approximate division/normalization remains rejected. The
wrapper-inlining trial gave negligible instruction benefit. The wasm Quat
swizzle candidate does not establish V8 improvement (development work was
about 4.5% higher), and the final S5 maxima stay failed. Further tiny/meshlet,
Oct/Exp and wasm tuning is explicitly abandoned within this one-pass brief,
with final intervals and development counters as evidence. No post-timing
code change or rescue measurement is selected. Registered acceptance is
not achieved; the scoped execution stop condition is met.

Delete only /mnt/linux-extra/moss-cargo-targets/codex-p07-fix5 after all
checks and timings finish. cleanup.json records target absence and
1,486,684,160 allocated bytes before deletion. Preserve artifacts outside
that target and archive the final clean committed source. No push, merge,
rebase, subagent, ARM execution or broader release/integration acceptance.

### Fix-six Oct attribution and exact sign arithmetic

Start clean at f069e84; artifacts /mnt/linux-extra/meshopt-artifacts/p07-fix6;
only target codex-p07-fix6, no subagents/push/merge/rebase. Inherited round-four
one-final-pass rule, frozen inputs and registered bars remain binding. Report
both RFC (1.25/1.50) and registered bars; use the registered maximum for stopping
and independently derive the RFC result from the same samples. No timing yet.

Instruction-count bisect across retained r3/r4/r5 binaries contradicts an Oct
code regression: tiny s4 13.265, resident s4 5.782, resident s8 3.016 instructions
per decoded byte at every epoch, caller-buffer. Oct kernel source is unchanged.
Do not invent a responsible commit from different timing epochs. Exact MIN/XOR
reflection replaces compare/select, and comparison-derived sign bits construct
the canonical half bias. Integer inputs are finite, input zero is +0, and MIN
uses +0 on equality. Keep original arithmetic association, sqrt/div and checked
zero-length errors; canonical rounding still compares >=0, including signed zero.
Native resident s4 falls to 4.970, s8 to 2.610; filtered Oct view to 8.988 from
9.800. Allocating reductions match. Native exhaustive 16,777,216 Oct8 words and
all Oct16/Quat edge combinations pass; existing per-level/tail tests pass.
Counters and binaries retained; actual wasm proof and final bars still required.

Meshlet caller-buffer decoders perform no zero-fill/output copy. Typed resident
v4/t3 prior median is 107.49ns caller vs 115.38ns allocating, while C++ has a
stronger caller baseline. Raw prior caller median is worse (186.99 vs 108.33ns),
but instruction work is lower (3.314 vs 3.951/byte), so extra loop work alone does
not explain it. Check output bounds/setup and preserve both absolute and relative
evidence. Exp S3 failures are caller varied-filter-3-tiny-s32 and resident-s32.

### Fix-six Exp convergence and rejected meshlet wrapper trial

For x86-64 Exp, use the unchanged safe reference directly. LLVM already emits
baseline SSE2 for that loop, for every diagnostic ceiling; eliminate manual
runtime dispatch and its separate 64-byte unrolling. Other backends keep their
explicit SIMD kernels. This is code-path convergence for S3, not scalar-reference
rewriting or approximate arithmetic. Resident s32 instruction work increases
0.674 to 0.720/byte; scalar comparator is 0.721. Diagnostic cycles are 0.213 vs
0.216, but do not claim timing acceptance. Tiny caller work is 1.186 vs 1.167;
scalar comparator 1.184. Existing extreme-word, layout and tail tests pass.
Final timing must determine both named S3 results and any lost Exp gains.

A separate typed/raw caller-wrapper inline(always) trial was rejected: typed
resident 4.281 vs 4.279 instructions/byte; raw 3.362 vs 3.314; no useful gain.
Retain source/counters/binary as meshlet-inline artifacts and restore original
wrapper attributes. No caller clear/copy was removed because none exists.
