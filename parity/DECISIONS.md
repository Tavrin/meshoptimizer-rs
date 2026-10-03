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
