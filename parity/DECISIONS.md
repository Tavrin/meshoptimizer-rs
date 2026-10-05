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

## D147 — RFC 113 cluster-LOD cook scaling

The baseline nine-mesh Rust/scalar-C++ sum-of-medians ratio was 6.391. On the
same S2 output, the baseline ratio tracks group count strongly (Pearson 0.960
over nine meshes), more than DAG depth (0.503). Earlier S0b C++ wall times
only confirm mesh ordering; they use a different config and vary by about 30%.

`perf record -g` on identical pyramid and riverforest branches inputs found
approximately 79% and 80% of Rust samples in full-mesh validation plus
per-group simplification setup before changes. The C++ call graphs spread
samples across meshlet construction, simplification, callback output, and
partitioning. Source inspection found three repeated full-source costs: Rust
copied all attributes and validated them for every group; meshlet construction
and initial bounds rescanned all positions for each group/cluster; and Rust's
flex builder initialized and swept source-sized adjacency for every small
group, while C++ `buildTriangleAdjacencySparse` visits only referenced
vertices when `vertex_count > index_count`.

The safe Rust change validates the source once, copies attributes once,
reuses lock flags per level, and passes the validated position magnitude to
private meshlet/bounds paths. It compacts flex-builder inputs for small groups
of a large source mesh, then maps meshlet vertices back to source indices.
The public API and scalar byte output stay unchanged. The intermediate full
20-pair run before group compaction retained 15/15 valid byte matches and
measured 1.319 scalar aggregate on CPU 0; it is superseded because another
lane also used that core. The final 20-pair consumer run on CPU 16 measures
1.171 scalar aggregate (2.570 s C++, 3.009 s
Rust), with every mesh at or below 1.394 and all 15 valid layout outputs
identical. The Moss-style C++ build measures 2.597 s, giving a 1.159 Rust/Moss
ratio. The scalar bar passes without SIMD Rust. The optimized scalar ratio no
longer rises with group count (nine-mesh Pearson -0.388) or depth (-0.810).
A separate 20-pair Cargo-defaults run measures 1.071 scalar/1.096 Moss
aggregate, but selected a much busier core; the profiles cannot be ranked by
their wall times. Full per-mesh medians, load, stage profiles and gates are in
the handoff.

Moss's `build.rs` does not define `MESHOPTIMIZER_NO_SIMD`; the harness also
times a second C++ binary with release `cc` flags and SIMD enabled. Vendored
`clusterizer.cpp` places explicit SSE/NEON code only in the spatial BVH
builder, while this S2 config sets `cluster_spatial=false`. Thus any
scalar/Moss-style difference on these cases is not evidence of that explicit
SIMD path. The two C++ binaries' output bytes are checked separately and
match on all 15 valid cases; no active S2 stage shows an explicit SIMD gain.

## D148 — Avoid redundant initialization in compact group scratch

The compact builder maps each group to a dense temporary index range. Its
first version zero-filled full-length local index, position, and reverse-map
vectors before overwriting every used element, and zero-filled then reset the
hash table to the empty sentinel. `Context::filled` now initializes the table
directly to its sentinel; the three vectors reserve their bounded capacity
and append initialized elements only. All allocation accounting and work
ticks remain in `Context`. Expected effect: less memory traffic per small
group, with the greatest opportunity on pyramid and riverforest branches,
where the group count and source/group size ratio are largest. Exact byte
parity is the acceptance gate before timing this change.

The final-source short call-graph check confirms that sparse group inputs
removed the remaining source-sized adjacency hotspot: `meshlet::adjacency`
fell from 16.67% to 0.97% of pyramid Rust samples and from 15.65% to no
sample in the branches capture. The remaining Rust samples are spread over
meshlet flex/nearest, simplification, and callback output. These are sampled
stage shares, not a paired timing result; the 20-pair bar remains the verdict.

## D149 — Bounded cluster-LOD simplification scratch

The one-time position, topology, attribute and weight validation remains. The
full attribute copy and vertex flags now begin only inside a `simplify` call and
are released before callback output or reclustering. A one-triangle mesh with
2,048 unused vertices and 32 attributes uses no such buffers. A driver built
from archived `main` and one built from this branch compare every byte limit
across a 513-byte window around the old success boundary, plus four wider
limits. The old peak is 34,904 bytes and the new peak is 24,576; all 517
limits that succeeded before still succeed.

## D150 — Invalidate the validated position range after dilation

The validated `moderate` decision is conservative after any dilation call:
later flex/bounds builds use their checked intermediate path. The open-mesh
near-threshold regression starts with every X coordinate at or below `1e8`,
actually moves vertices under dilation, and agrees byte-for-byte with both
archived `main` and the vendored C++ demo on the generated case. This input
does not move an X coordinate across the threshold, so the crossing itself is
covered by the source invariant, not claimed as observed differential evidence.

## D151 — RFC 113 three-way and portable harness gates

The RFC 113 runner now fails when either Moss-style or scalar C++ disagrees
with Rust. The codec builder creates a fresh target directory before C++
compilation. The deterministic differential builds archived `main`, current
Rust, and the vendored C++ demo and compares the complete result on large,
sparse, 32-attribute, compact-boundary, and near-threshold dilation inputs.
The bounded-memory companion compares `LimitExceeded` and success across its
byte sweep. Full parity and paired timing require final-HEAD evidence, with
timing admitted only below load 12 and without a GPU lease holder.

## D152 — Owner-revised RFC 113 admission and bounded early stopping

The owner's binding 2026-10-05 directives replace the original load-average
and no-active-scoreboard gate. Admit only when `gpu-lease.sh status` reports
no holder and no `moss-scoreboard-*` unit is measuring. An active unit whose
only child is `sleep` is waiting and does not block. Record its MainPID and
child process tree. Load is informational, never an admission threshold.
Check before each interleaved scalar-C++ / Moss-C++ / Rust pair; a new blocker
allows the current pair to finish, then closes all driver processes before
waiting. Failed admission polls retain the five-minute cadence.

This continuation makes no implementation changes. The initial fixed-20
consumer run was interrupted by the new sampling directive; its incomplete
printed ratios are diagnostic and excluded from the final verdict. Run one
fresh final matrix under each profile with adaptive sampling. Begin each
mesh at five pairs, examine the nominal two-sided 95% Student-t interval of
paired log(Rust/scalar-C++) ratios after each additional pair, and stop when
the interval is wholly within or over 1.5; cap stage 1 at 20 pairs. These
sequential intervals are stopping heuristics, not simultaneous confidence
coverage. Retain every sample and stopping decision. The 1.2 aggregate bar
still uses the ratio of summed stage-1 per-mesh medians, unchanged.

Only a case whose interval still overlaps 1.5 at the 20-pair cap receives the
D146 second stage: 30 fresh interleaved pairs, df=29, PASS only when the upper
bound is at most 1.5; FAIL when the lower bound exceeds 1.5; overlap is
INCONCLUSIVE and counts as FAIL. The owner's newer borderline-only rule
supersedes D146's older rule to retest every point estimate above 1.5. Stage 2
can clear only that case maximum, never replace stage-1 aggregate data.
Stage 2 retains one selected physical core across any lease-induced pause,
as D146 requires; driver processes close while waiting and reopen on that
same core for the remaining fresh pairs.

Choose the least-busy physical core by a one-second sample of both siblings,
excluding physical cores containing logical CPUs 0/1. Pin all three drivers
there within a burst. Record selection, load before/after, utilization of
both siblings, and pair boundaries for every burst. Close drivers after each
case and before every admission pause. Cap active bursts at 840 seconds
(before a new pair), then close drivers and cool down for 60 seconds; a pair
is always completed before releasing the core. During future implementation
iterations, measure only touched families/cases at about five pairs; do not
repeat full matrices. Exact parity and final-HEAD identity gates are retained.

## D153 — Borrow cluster-LOD inputs without retained simplification scratch

The recovery baseline is `3db42ccc81ecc9ac5e21d181d3deac735573f711`
(with D149–D152 intact). Before source edits, `perf record -F 199 -g
--call-graph dwarf,8192` captured three identical cooks per scalar backend
on pyramid, branches, leaves and modular. The Rust group simplification
wrapper accounts for respectively 45.94%, 46.24%, 43.30% and 25.98% of
sampled cycles; bulk zeroing accounts for another 9.60%, 9.50%, 8.39% and
5.01%. Actual simplifier state work is separately attributed. Source inspection
locates the repeated cost in D149's full-source attribute copy/initialization
and flag conversion for every small sparse group. D150 cannot cause this S2
regression: the timed config disables dilation.

These are diagnostic stage shares under recorded shared-machine load, collected
while a GPU lease was held. They are not paired wall-time evidence. The spec's
**timing** admission gate remains binding for every iterative and final pair.
Baseline source/binary identities, raw perf captures, reports, core/load and
admission records are retained in `/mnt/linux-extra/meshopt-artifacts/clodrec`.

The private support attribute view now borrows the already validated immutable
public view. Sparse remapping still happens inside the existing simplifier and
preserves strided floats, unaligned bytes and explicit byte order. The existing
boundary-lock allocation uses the private simplifier's transparent one-byte
flag type; discovery bit 7 is cleared from every entry before simplification.
This removes both per-group full-source buffers, instead of caching them
across callbacks or reclustering. All other heap allocations/lifetimes and
work charges are unchanged, and child budgets still include live parent
storage. Thus no input acquires additional owned heap capacity versus the
pre-optimization path; the single-cluster path allocates neither buffer.
No unsafe code, SIMD, public API or arithmetic change is introduced.

Unconditional position-range invalidation after dilation remains intact.
A new multi-level regression compares complete output, mutated positions,
work and exact workspace-byte boundaries across packed/padded float and
unaligned little-/big-endian attribute layouts, with LOCK/PROTECT/PRIORITY,
zero-weight components and both dilation settings. It passed before and after
the source change. The existing memory/dilation regressions remain green.

`parity/rfc113/timing.py` makes D152's previously external adaptive method
reproducible from the worktree. It builds and hashes the current sources and
binaries, records process-tree admission evidence and both physical-core
siblings, closes drivers before lease pauses, caps bursts at 840 seconds,
and reserves D146's 30-pair stage for unresolved 20-pair intervals. `--case`
and `--pairs 5` select affected cases for diagnostic iteration. Final results
use one adaptive pass per profile. Admission and stopping regressions are
part of the focused harness tests.

## D154 — Final cluster-LOD recovery acceptance and cleanup

The implementation `9589e0cab7fdf8eef50654a744a6f05a20a01047` meets
both unchanged bars on one final adaptive pass per Rust profile. Consumer
Rust/scalar-C++ aggregate is **1.1959471126155838**, Cargo defaults
**1.0899528253461015**. Every mesh is below 1.5; observed maxima are
1.269240275051873 and 1.315468938081677 respectively (the full-precision
records are authoritative). Moss-style comparison aggregates are 1.1604228528
and 1.1280284467. The profiles use different admitted bursts and loads, so
these wall times do not rank the profiles or claim quiet-machine performance.

Only the four affected meshes received five diagnostic pairs. The final pass
retains 98 fresh interleaved three-way pairs: 49 per profile, 5–9 per mesh.
Every stopping interval passes; no borderline case reached 20 pairs, and no
D146 second stage was needed. Longest active burst: 23.405 seconds.
Every pair-start admission and every current source/binary identity was
independently rechecked before cleanup.

Queued GPU work left short free windows between leases. The initial periodic
attempt was stopped while paused, before any measured pair. The retained
`measure-on-release.py` adapter watches holder process exits with Linux pidfd
notifications and performs a fresh unchanged admission check after an exit.
Periodic failed checks retain D152's 300-second cadence; cooldown stays 60
seconds. The observer never acquires a lease or interrupts another job, and
all drivers close before waiting. It checks the prepared source/executable
manifest instead of rebuilding between free windows. Its own hash and the
prepared-build manifest hash are included in the result identities. Timers,
interleaving, core selection, stopping intervals and aggregate calculation
remain those of the committed runner.

Both final profiles match all 15 supported layouts byte for byte against
scalar and Moss-style C++; all 90 codec comparisons, 12 three-way cases and
517 byte limits pass. All 0.1–0.4 fixture/seeded gates and the 80 cluster-LOD
C++/native/WASM cases pass, with independently verified manifests. Both
feature test/clippy configurations and fmt pass. D149's single-cluster memory
guarantee and D150's unconditional post-dilation invalidation remain intact.

Evidence is in `/mnt/linux-extra/meshopt-artifacts/clodrec` and the detailed
handoff is `parity/results/rfc113-clod-recover.md`. Twenty required native/WASM
executable identities were retained and hash-verified, along with baseline
and implementation source archives, raw perf data, raw pairs and validation
corpora. The exact named target
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodrec` was deleted after
verification; `target-cleanup.json` and `artifact-manifest.json` are the
receipts. No push or integration/runtime qualification is performed.

## D155 — P06 independent batches and resource determinism

Implement SPEC-p06 on phase/0.6 from published a4b3c48 only. Rayon 1.11.0 is
optional, pinned and compatible with Rust 1.88; `parallel` enables `std`.
Default/no_std algorithms and the unsafe prohibition are unchanged. Use ordered
preallocated result slots, fallible allocation before execution, and an inner
Result per item. A bad item never cancels the rest. Panics retain Rayon semantics.
Use the global/current pool; an existing pool's install selects a custom pool.

Every executing thread has a private thread-local Workspace. Clear it before
and after each item. Retaining arbitrary preceding scratch could make a later
item exceed its budget depending on scheduling; reuse across items is rejected
for this API. A LOD chain runs levels sequentially through the existing plain
or attribute simplifier, keeping original vertex references and error bits.
Charge all live level outputs plus scratch and cumulative work to one chain
budget. Result slots and Rayon infrastructure are batch-level overhead excluded
from item limits; callers must bound aggregate live outputs themselves.

Parallelize independent meshes, full views and full cluster-LOD builds. Mutable
cluster-LOD positions are disjoint borrowed slices, with the same late-failure
mutation contract as the sequential demo. Do not parallelize groups inside a
single DAG: dilation shares positions and later levels consume prior groups.
Preserve callback order, refinement IDs and hierarchy order. Add view decoding
alongside the requested encoding, composing the existing scalar view API for
later SIMD integration. No per-view block split or float reduction is added.

The baseline contains 0.1/0.1.x/0.2/0.3/0.4. Phase 0.5 modules, tests and a
0.5 dispatcher are absent; p05 is a separate unmerged worktree. Record this
qualification limit instead of merging, copying that lane, or inventing a
0.5 pass. Run every available existing exact parity gate unchanged. A p06
thread-count test additionally compares every meaningful byte and f32 bit at
1, 2, 8 and available_parallelism threads, including mixed errors, limits,
worker histories, dilation and concurrent calls. Reversal cost: cheap before
integration; API changes would break consumers after publication.

## D156 — Owner's fast measurement directive for P06

The binding owner directive on 2026-10-05 supersedes older unconditional
10–30-pair/full-repeat timing rules for this lane. During iteration, measure
only touched cases/families at about five interleaved pairs. For a final bar
verdict start at five, stop when clearly within/over the bar, cap at twenty;
only borderline cases use D146's second stage. Run a full timing matrix once
at the end. P06 has no hard speed bar, so the final new-workload speed curve
uses exactly five alternating sequential-Rust/parallel-Rust pairs per family
at 1/2/4/8/16 threads. No legacy performance matrix is rerun; legacy exact
parity/build gates are not relaxed. No iterative timing is needed for the
additive wrappers.

Use the pinned upstream demo/pirate.obj (Clint Bellanger, CC-BY-SA 3.0) as an
authored realistic mesh, with sixteen deterministic spatial variants. Report
this single-model corpus honestly, not as private Moss assets or broad mesh
coverage. Pool creation, parsing, generation, serialization and output hashing
are outside timing; validation, output/scratch allocation, execution and
required fresh dilation-input copies are inside. Compare complete serialized
outputs after every measured sample. Pin one allowed logical CPU per physical
core, selecting by /proc/stat load; retain topology, loads, raw times, executable,
source, dependency and input SHA-256 identities. Use Cargo release defaults in
the unpublished dependent driver; the library's profile does not configure it.

Admit warm-ups and each pair only with no GPU lease holder and no actively
measuring moss-scoreboard-* service. A service with only a sleep child is idle.
Retain admission observations before and after each pair, finish the current
pair if a lease starts, then pause. Each burst has a deadline below fifteen
minutes and releases all worker cores on exit; resumption keeps completed pairs.
No busy measurement process waits for admission. No gain is presumed and no
noisy sample is removed. The curve compares parallel and sequential Rust,
not an algorithmic C++ performance claim.

## D157 — P06 final local result

All implemented batches are qualified against their sequential operations at
1/2/8/32 threads, including bitwise bounds/errors/dilation, item errors and
resource boundaries. The seven differential tests pass. Both Clippy modes
with warnings denied, both feature-mode test suites, parallel without clusterlod,
fmt, strict rustdoc, no-default/all-feature WASM builds and the P06 driver
fmt/Clippy/build pass. Rust 1.88 all-feature tests and doc examples also pass.
Default and no-default dependency trees still contain only pinned libm.
Parallel WASM execution and remote-platform acceptance are not claimed.

Every available existing exact gate passes against unmodified pinned C++ and
executed WASM: geometry 279 fixtures + 10,000 seeded cases; preprocessing
645 fixtures + 64,000 seeded cases; 0.2 287 fixtures + 3,637 malformed +
14,000 seeded cases; 0.3 697 fixture/generated messages + 52,000 sweep
messages + 80 cluster-LOD demo cases; 0.4 869 fixtures + 7,653 malformed +
44,000 seeded cases. Applicable upstream JS suites run in those harnesses.
There are zero mismatches. Prior performance/fuzz/release records are unchanged
and are not promoted to P06 acceptance. Phase 0.5 remains absent/unqualified
in this baseline; its integration owner must run that gate after integration.

The one final curve takes 56.93 seconds and keeps all 150 alternating pairs.
Every observed admission is clear; the scoreboard's only child is sleep. Load
is 15.46–18.00. At 1/2/4/8/16 threads, paired median speed-ups versus sequential
Rust are LOD 1.026/2.001/3.211/6.216/8.201, encoding
1.207/2.173/3.453/5.445/7.599, decoding 1.193/2.094/3.031/4.261/3.284,
meshlets 1.112/1.553/4.102/3.872/6.136, cluster LOD
0.991/1.911/3.913/5.505/7.818, hierarchy 0.845/0.817/0.852/0.988/0.623.
Small forests do not amortize batching; retain that slowdown. Sixteen-thread
cluster LOD pairs range 1.82–10.23x; the shared-load median is no universal
promise. There is no hard P06 bar or borderline D146 case. D156's fast owner
method is followed; no repeated full matrix or iterative timing is performed.

The external read-only verifier rechecks every recorded source, dependency,
archive/member hash, meaningful comparison and corpus inventory. Retain slim
summary MEASURED_P06.json and reviewable method/table in parity/p06/README.md.
Evidence stays under /mnt/linux-extra/meshopt-artifacts/p06, separate from the
specified disposable target. Preserve exact binaries and sources, verify hashes,
then delete only codex-meshopt-p06. Commit only on phase/0.6 with the authorized
author; no push, merge, rebase or other worktree change. The remaining 0.5
qualification prerequisite is recorded rather than concealed or bypassed.


## D158 — P06 review fixes 2–5: late mutation and small hierarchy dispatch

Finding 1 is resolved by the integration owner; do not rerun or alter the 0.5
lane here. Findings 2–5 are addressed in this checkout on phase/0.6.

Replace the unproven partial-failure coverage with an explicit 16-triangle
curved open disk witness: dilation changes position bits before max_work=3200
returns LimitExceeded. Small successful neighbours on both sides share the same
limits. Compare complete group outputs/errors and every position bit against
sequential execution at 1/2/8/N (32 here) threads, repeating fresh inputs three
times in each pool. Both successes and the failed item's mutation are asserted.
Add hierarchy tests on either side of both dispatch cutoffs, with ordered
invalid items and tight work/memory limits.

Quick hierarchy-only profile: five alternating pairs at 1/2/4/8 threads,
plus one dispatch-only control per pair (pool.install, indexed output slots,
parallel traversal, no hierarchy or Workspace work). The 16-thread baseline
was paused before samples when a new GPU lease appeared; keep those four rows
rather than extending this diagnostic. The burst lasted 7.693 seconds, with
loads 12.90–22.68 across admission observations. This same pinned Pirate family
has 16 forests, 231 groups and 1848 group-by-level visits (eight levels each).
Serial medians were 54.062/57.358/53.020/64.411 us; full batch medians were
91.051/58.700/54.101/151.004 us. Dispatch-only medians were
15.860/13.946/16.361/18.885 us: fixed pool entry/task dispatch alone consumes a
substantial fraction of the serial work. It does not amortize on this family.
Scheduler-sensitive outliers remain; this control does not establish allocator
contention or a universal crossover size. No profiler sampling or long timing
matrix is needed to decide this small-workload fallback.

Run hierarchy items sequentially inside the batch API when total groups <=256
and sum(groups.len * max(level_count,1)) <=2048. Saturating arithmetic affects
only dispatch selection; normal per-item validation and limits retain their
precedence. Preserve ordered slots, item Workspace clearing, errors and output
bits. Larger forests still use Rayon. The conservative round-number cutoff
covers the measured small family; it is not a measured optimal crossover, and
larger-family performance is not qualified here. Reject unconditional serial
execution for all forests and an arbitrary per-thread cutoff.

Final hierarchy-only check: five pairs at 1/2/4/8/16 threads, 7.586-second burst,
loads 29.11–30.60. Paired medians S/batch are 0.878/0.836/0.775/0.545/0.725x.
Serial medians are 59.582/54.703/55.093/57.969/75.432 us; batch medians are
67.356/68.409/83.276/112.933/105.147 us. Every output matches across calls and
thread counts. Preserve these residual slowdowns: the existing transport times
caller-side pool.install, even though the small batch now dispatches no items.
The dispatch-only control reaches median 2867.606/3568.975 us at 8/16 threads
under this load. The fallback removes internal parallel item scheduling; it
does not remove custom-pool entry or promise a gain under shared scheduling.

Each timed pair was admitted with no GPU lease holder and no actively measuring
moss-scoreboard unit (only sleep children). After the diagnostic's last completed
row a new lease appeared, and all workers were released before waiting. No
measurements ran while waiting. Preserve complete raw pairs, controls, loads,
affinity/admission observations, output hashes, source/dependency/input identities
and exact binaries under /mnt/linux-extra/meshopt-artifacts/p06fix/{before,after}.
The baseline curve intentionally remains incomplete; the final curve is complete
for the selected hierarchy family only. Other families and historical P06
qualification records are unchanged, not reverified or promoted by this fix.

Document that Rayon lazily initializes its global pool on first use and panics
inside Rayon if initialization fails, distinct from per-item Error isolation.
Small sequential hierarchy batches need no global pool; other batches/larger
hierarchies use the current pool. Remove phase 0.6 from README's remaining work.
The lean driver now supports one-family/short-burst selection and a dispatch
control. Exclude the dynamic CPU scaling percentage from immutable resume
identity; loads remain observations. No API changes or new dependencies.


Final fix gates: cargo fmt (crate and driver), strict Clippy --all-targets in
all-feature/no-default modes, both test suites, all nine deterministic parallel
tests, WASM builds in both modes, and driver strict Clippy pass. Gate logs and
exit statuses are retained as p06fix/gates.{log,json}. The parallel-only suite
without clusterlod also passed during the fix. Before/after archived sources,
all 45 timing pairs, dispatch controls, all output/binary hashes, admission
checks and sub-five-minute deadlines were verified before target deletion.
No new C++/legacy-parity, GPU execution, diverse-corpus, larger-forest speed,
remote CI or release qualification is claimed by these focused fixes.

## D159 — Assemble the first 0.2.0 integration tranche

Merge parity/rfc113 at 9027712 (including recovery 9589e0c), then phase/0.6
at 70686e6 into release/0.2.0 from published main a4b3c48, only in rel020.
Keep RFC 113 decisions D147–D154; renumber P06's D147–D150 to D155–D158
and update their references. Preserve both branches' code and every decision.
The package remains 0.1.0 until the coordinator bumps it at release.

Re-run fmt, strict Clippy, tests and wasm32 builds in all-feature and
no-default modes, 0.1/0.1.x/0.2/0.3/0.4 fixture runs and 2,000-case-per-family
sweeps, the 80-case cluster-LOD demo gate, RFC 113's 15 supported layouts and
90 codec comparisons, and all nine P06 thread-count determinism tests.
Use only /mnt/linux-extra/moss-cargo-targets/codex-meshopt-rel020 for builds
and remove it after preserving and verifying evidence. Retain fresh records
under /mnt/linux-extra/meshopt-artifacts/rel020.

No timing is rerun: the source branches' measured evidence stands, with its
recorded corpus, profiles and remaining limitations. RFC 113 runs with
--pairs 0. The two stride-32 S2 protect-mask setups remain invalid, outside
the fifteen supported layouts. Phase 0.5 and phase 0.7 remain outside this
first integration tranche; no remote, GPU or Moss runtime acceptance is added.

Final integration gates all exit 0: both strict Clippy/test/wasm32 modes,
fmt, all nine P06 determinism tests, every registered 0.1–0.4 fixture run
and 2,000-case-per-family sweep including 0.1.x, all 80 cluster-LOD demo
cases, fifteen supported RFC 113 layouts and ninety codec comparisons.
There are zero parity mismatches. The first preprocessing invocation only
hit the artifact directory's immutable-record guard; it was retained as a
failed routing attempt and rerun with separate p01x-run/p01x-sweep directories.
No corpus, case budget or algorithm was changed to pass.

The independent artifact audit verifies source hashes, archive/member hashes
and executable identities, and preserves exact harness binaries and the
implementation source archive before deleting the named Cargo target.
parity/results/integration-0.2.0.json links the gate, audit, manifest and
target-deletion receipts under /mnt/linux-extra/meshopt-artifacts/rel020.
Only coverage wording and this outcome/evidence were added after validation;
algorithm, test, harness and dependency bytes remain the validated bytes.
Package version remains 0.1.0. No timing, push or other-worktree change.

## D67 — 0.5 scope and public API

The pinned 1.3 header contains 87 exported C functions and two public C++
inline quantizers. The checkout ledger classifies all 89: 46 ported here,
40 owned by the unmerged 0.1.x/0.3 branches, and three deliberately replaced
or excluded. The 0.5 work adds 15 names: four strip functions, four analyzers,
four opacity-map functions, tangent generation, and two experimental functions.
The raster analyzer and opacity-map rasterizer share no public dependency on
those unmerged branches. Reimplementing their needed helpers privately keeps
the branch ownership intact. Global allocator callbacks become per-call
`Workspace` limits; the two C++-only inline quantizers stay private to the
filter implementation. Version/level global setters were already replaced by
explicit per-call configuration in 0.4.

The Rust-facing shape follows the prior lane: `Result` for invalid or
resource-limited inputs, `u32` topology, packed/interleaved/byte `Positions`,
and allocating plus caller-buffer forms for functions with output arrays.
`generate_normals` and `remesh` are gated by `experimental` independently of
tangents. `REMESH_SHELL` and `REMESH_SOLVE` retain their pinned option bits.
The remesher's base-case table is generated from pinned C++ rotations and
checked in as Rust constants; it is not a runtime C++ or build dependency.

## D68 — 0.5 oracle, differential and robustness domain

The oracle remains meshoptimizer `4c203430ca565cb59a468a91922c76c208169536`
compiled scalar-strict (`MESHOPTIMIZER_NO_SIMD`, no fast math, no FMA
contraction). Six focused Rust fixtures cover published upstream vectors and
normal/tangent float bits, OMM, raster, strip/cache and remesh cases. The
seeded differential runner compares every output byte on 2,000 cases for each
of the 15 functions; an executed Node wasm32 runner compares identical inputs
and complete outputs to native Rust. Separate normal and remesh records remain
visible in the per-family case logs and summaries. C++ assertion-domain
violations are Rust errors, not purported C++ parity. Fuzz targets mutate
each entry point with AddressSanitizer and bounded inputs. An initial OMM
compact smoke found an overlapping-offset case that copied beyond the output;
checked offsets now reject it with `BufferTooSmall` and the saved input replays
without a panic. The initial OMM raster smoke timed out on a level-12 case;
the fuzzer now limits valid raster levels to five, while still testing invalid
levels above twelve. The failed first-run record stays under the external
`superseded` artifact directory.

## D69 — 0.5 benchmark protocol and current rejection

The RFC's 1.25 geometric-mean and 1.50 individual-case Rust/scalar-C++ time
limits remain unchanged. Ten interleaved paired samples per case under both
Moss-like and Cargo-default release profiles record raw times and byte-checked
outputs. Each family uses three sizes and separately measures caller-buffer
forms where exposed. These local measurements fail the time bar in 18/21
family/API groups under the Moss-like profile and 17/21 under Cargo defaults.
Only unstripify (both forms) and overdraw pass in the Moss-like record; only
allocating stripify, both unstripify forms and OMM compact pass under Cargo
defaults. In particular, tangent and experimental normal generation are
roughly 2.7–3.8 times scalar C++; remeshing is roughly 1.5–1.8 times. The existing corpus does
not yet include million-triangle cases or enough geometry varieties from RFC
6.1, and the comparable C++ peak storage is not measured, so the 1.25
temporary-memory bar cannot be declared passed. No performance exception was
approved. The 0.5 artifact verifier must therefore fail until these shortfalls
are resolved, even though focused parity, fuzz and WASM checks can pass.

## D70 — 0.5 final local handoff

`cargo fmt --all -- --check`, clippy with `-D warnings` in all-features,
no-default-features and experimental configurations, the same three test
configurations, and the wasm32 release build exit 0. The 0.5 fixture run exits
0 with six focused tests. The final native sweep and executed WASM identity
each match all 30,000 cases, 2,000 per public 0.5 function. Fifteen
AddressSanitizer cargo-fuzz targets each complete more than 300 seconds,
totalling 120,625,376 executions with no crashes. The package inventory
includes every new source module and its focused test. Detailed per-case and
fuzz logs and the exact native, WASM and sanitizer executables remain outside
the checkout; the retained artifact tree is 309 MB, below the 2 GB cap.

Both benchmark commands exit 1 under the unchanged RFC bar, and
`parity/report.sh --phase 0.5 --verify-artifacts` exits 1 with exactly two
errors: `failed benchmark-moss` and `failed benchmark-default`. This lane is
**not performance-qualified or complete under the spec's Done-when gate**.
The measured failures are retained as reviewable records rather than converted
to a passing summary. No Git metadata operation was performed.
The named temporary Cargo target was deleted after retaining binaries and
hash-checking the artifacts; the verifier gives the same two benchmark errors
without the target directory.

## D71 — 0.5 performance measurement correction

The first 0.5 timing record was incomplete: three representative inputs per
family, ten pairs, no comparable C++ requested-storage measurement, and no
million-triangle or topology-variety corpus. The P05 follow-up keeps the RFC
1.25 geometric-mean, 1.50 maximum-case and 1.25 memory limits unchanged. It
uses 20 alternating pairs per case on the measured least-busy physical core,
records load around each pair, and never waits for quiet. The expanded matrix
has 160 family/API cases per profile, including random, connected,
seam-heavy, sparse and explicitly disconnected topologies and
million-triangle cases for geometry-dependent operations. OMM functions that
ignore the mesh use six varied parameter/input cases. The C++ scalar-strict
driver measures scratch requests through meshoptimizer's allocator callback
and adds requested output vectors; Rust uses per-call workspace requested
storage. Caller-owned destinations are excluded from both sides.

The apparent 2–4x gaps in `stripify_bound`, `unstripify_bound`, and
`opacity_map_entry_size` required a harness correction before interpreting
them. Both drivers now batch at least 100,000 calls, vary valid inputs within
the batch, dispatch outside the timed helper loop, and consume each result
with equivalent compiler barriers. The earlier Rust loop built an unused
`Workspace` on each helper call, while C++ used a volatile XOR sink; `perf`
instruction counts exposed that asymmetry. Superseded measurements remain in
the external artifact tree. The final records alone determine the verdict.

## D72 — 0.5 second safe-Rust optimization pass and timing isolation

The formatted 160-case benchmark still misses the unchanged bars, especially
on million-triangle stripify/unstripify/remesh, disconnected tangent/normal
cases, and the cache and OMM analyzers. Before the next timed run, the Rust
pass batches work-budget charges where the number of visits is known:
stripify valence setup and triangle emission, unstripify decoding,
vertex-fetch cache visits and remesh voxel samples. Each path retains the scalar fallback for low work
limits, including the original failure point and partial usage. Position
accessors in remesh, tangents and normals are forced inline; standard builds
use scalar `f32::sqrt` (and OMM sampling uses `f32::floor`) while `no_std`
keeps `libm`. Validation in normal/tangent generation and OMM measurement
uses `Work::scan` to avoid per-record fuel checks when the budget covers the
scan. These changes are expected to reduce the measured hot-path instructions,
especially for large meshes and repeated texture samples; no speedup is
claimed before paired measurements and exact parity checks.

A follow-up index-validation path first takes the maximum of a valid index
stream and charges the full scan once. Invalid inputs and low fuel replay the
old per-index scan, preserving the first error and its charged prefix. This
targets the extra Rust validation pass in cache/fetch analysis, stripification,
normal/tangent generation and OMM measurement. It is expected to help large
index streams while keeping exact validation semantics.

Remesh voxelization now has separate compile-time mark and accumulate modes.
The selected mode is fixed for each pass, removing a per-voxel option branch
from the million-triangle hot path without changing the sampling order or
floating-point expressions. Its expected gain is concentrated in remesh.
Unstripify also writes the three triangle indices through one checked output
slice; this removes a small array-to-slice copy in its million-index loop.
Vertex-cache analysis now computes the three miss flags directly from the
triangle's three indices, matching the pinned scalar C++ loop more closely
than an iterator filter and count. This targets its 2.5x worst medium case.
Normal and tangent generation now batch exact adjacency, per-face geometry and
accumulation visit charges when the fuel covers the phase. Tangent accumulation
still charges only nondegenerate faces; tight budgets use per-corner charging.
This targets the repeated fuel branch in their hot loops.
Remesh also batches its full-position extent scan and uses the counted view
scanner for finite-value validation, avoiding a per-vertex fuel check in each
of two passes over million-mesh inputs.
The OMM texture sampler and floor helper are forced inline so repeated raster
edge samples can avoid a nested call and reuse fixed texture dimensions.
Overdraw and coverage rasterization now charge the known bounding-box pixel
visits once per triangle and the fixed viewport scan once per axis, with the
old per-pixel fallback under tight fuel limits. Position validation uses a
counted scan and transformed index visits are charged together. This targets
their default-profile near-bar cases without changing any pixel arithmetic.
Focused tests confirm that the exact successful work budget still succeeds and
one fewer unit rejects stripification, unstripification, fetch analysis,
tangent/normal generation and remeshing. The index-validation unit test also
checks the first error and charged prefix across invalid indices and low fuel.

The C++ allocator hook now skips its tracking-map lookup outside the memory
probe, removing an unfair timed C++ overhead. Perf profiling repeats each
selected worst case at least ten times and omits Rust response hashing; the
whole-driver instruction comparison still includes fixture setup and is
diagnostic, not an algorithm-only count. At the coordinator's request, timed
benchmarks were initially held until 23:30 Paris on 2026-10-04, then deferred
to 02:30 Paris on 2026-10-05 unless the `p01x` benchmark binary exited first.
At 23:01 Paris on 2026-10-04, the process check found only `p01x` coordinator
services and no benchmark binary, so the replacement timing run began under
the early-start condition. Parity and correctness checks completed before it.

Strict Clippy exposed that caller-buffer tangent and normal benchmark branches
fed unit return values to `black_box`. They now consume the resulting output
buffers, matching the C++ driver's observable destination reads. This makes
their timed work explicit; old candidate timings are superseded.
The replacement benchmark summaries and detail records pin hashes
of every Rust library source, P05 benchmark/fixture driver source and pinned
C++ reference source; the
artifact verifier rejects a source edit after measurement. It also verifies
that each retained perf run used the worst failing case and that its compact
instruction summary matches the hashed raw record.

Before the timing window, the candidate passes eight focused 0.5 tests,
including both exact-budget tests, all-feature and no-default-feature test
configurations, strict Clippy in all-feature and no-default experimental
configurations, 30,000 native C++ differential cases and 30,000 executed WASM
identity cases with zero mismatches. The then-stale benchmark and perf
identities were replaced after the `p01x` benchmark process cleared.

## D73 — 0.5 adjacency-pair hot-path follow-up

The D72 timed run and perf profiles exposed normal-group pair merging as the
largest Rust sample on its worst seam-heavy case; the same pair loop appears
in tangent merging. Both loops now charge the known number of adjacency pairs
once when the remaining work budget covers the group, retaining per-pair
charging under a tight budget. Normal merging also avoids loading and dotting
face normals until the two corners share an edge, and hoists the first face's
normal out of the inner loop. The ordered floating-point dot expression and
`dp > cutoff` predicate are unchanged, including NaN behavior. Expected effect:
fewer work-limit branches and dot products for seam-heavy normal meshes, and
fewer work-limit branches for tangent meshes. The change is conditional on
exact parity, tight-budget tests, and replacement benchmark/profile records.

The replacement records meet those evidence conditions. Eight focused P05
tests, 30,000 native differential cases and 30,000 executed WASM identity
cases pass with zero mismatches. The full 160-case, 20-pair matrices and
worst-failing-family `perf stat`/`perf record` runs are source- and binary-hash
bound. `parity/report.sh --phase 0.5 --verify-artifacts` exits 0 with
`verified: true`, no errors and `performance_qualified: false`. Moss passes
9/21 API groups and Cargo defaults pass 7/21. The same seam-heavy normal
case fell from 1.63x to 1.34x in Moss and 1.71x to 1.48x in defaults; the
normal family still misses geometric-mean and caller-buffer memory limits.
The selected Moss tangent million-triangle instruction ratio fell from
1.41x to 1.37x but the family remains above time and caller-buffer memory
limits. These observations support retaining the exact D73 change, not a
performance-qualification claim.

The remaining failing families and their selected-case instruction ratios
are recorded in `P05_PERFORMANCE.md`. Stripify and unstripify retain large
million-triangle timing maxima; cache/fetch and OMM retain algorithmic
instruction gaps; default overdraw has a connected-mesh maximum above 1.50;
and tangent/normal caller APIs require complete staged results to preserve
their atomic failure contract. The current safe-Rust pass does not close those
gaps. No bar change or exception is applied, and the release performance gate
remains blocked.

## D74 — 0.5 sequential strip-decoder reads before the deferred timing gate

The completed D73 profiles still identify `unstripify_core` as the hot Rust
symbol on the million-triangle case. Its loop loaded the two preceding strip
indices by checked indexing on every visit, although those values are the
previous two elements of the same sequential stream. It now carries those
two values through the loop, including across restart markers, and retains
the same validation, work charges, winding swap, degeneracy test and writes.
Expected effect: fewer indexed loads and bounds checks on long strips, with
the largest possible gain on the million-triangle unstripify case. This is
an exact safe-Rust loop transformation, pending differential parity and the
coordinator's 02:30 Paris load-gated timed rerun.

The output cursor now advances a checked three-element slice only when a
nondegenerate triangle is emitted. The initial bound check proves the cursor
has room for every possible emitted triangle; the per-emission split carries
the remaining output span without recomputing `size..size+3`. Expected effect:
less checked-range arithmetic and a simpler sequential store path on long
strips. Caller-buffer tails and partial writes under tight work limits keep
their original behavior.

## D75 — 0.5 cache-transform counter bound

The cache analyzer's per-miss `checked_add(1)` cannot overflow after the
existing topology gate: it accepts at most `u32::MAX` index entries, and each
entry can cause at most one transform. The hot loop now uses a plain increment,
retaining the same u32 value and every possible public error. Expected effect:
remove an overflow branch and error path on each cache miss, particularly on
the seam-heavy cache case where the final D73 time and instruction ratios
still miss the bar. Exact parity, budget checks, and the deferred timed run
will qualify the change.

## D76 — 0.5 scalar OMM raster setup and midpoint expansion

The D73 default `perf record` on the failing caller-buffer raster case places
substantial Rust samples in the three-corner array-map machinery, alongside
the recursive raster routine. The initial three UV samples are now written
explicitly in the same order. Recursive midpoint generation now calls an
always-inlined scalar helper for edges 0-1, 1-2 and 2-0, removing a dynamic
`% 3` edge lookup while preserving each `(a+b)/2` and texture sample in order.
Expected effect: lower small-case OMM raster overhead and repeated recursion
cost without changing the result or the atomic caller-buffer staging contract.
The unchanged memory bar can still fail for that caller API. Differential
parity and the deferred load-gated benchmark determine retention.

## D77 — 0.5 contiguous raster rows

The default overdraw profile selects a connected-mesh case with a 1.82x
maximum, and most Rust samples lie in the software raster analyzer. For each
bounding-box row, the pixel loop now iterates a checked contiguous row slice
instead of recomputing and checking a full-buffer index for every pixel. Empty
bounding boxes return after the same zero work charge. Axis permutation uses
three explicit triangle vertices in place of an array-map call. Pixel visit
order, depth comparisons, float steps, work-budget failure points and output
remain unchanged. Expected effect: fewer bounds checks and indexing operations
on shaded and covered pixels, especially the connected overdraw case. This
change awaits native/WASM differential checks and the deferred timed matrix.

## D78 — 0.5 direct tangent-corner loads

The D73 Moss tangent profile attributes its largest Rust sample to
accumulation. The three per-face positions are now read explicitly in corner
order, replacing an array-map wrapper inside that hot loop. This matches the
already explicit normal-face accumulation and preserves the same three
validated indices, position values and floating-point operations. Expected
effect: less per-face iterator/setup overhead in large tangent meshes. Exact
parity and load-gated timings will decide its measured effect.

## D79 — 0.5 explicit voxel sample coordinates

`perf record` places almost all selected worst-case remesh samples in the two
voxel passes. The inner sample loop now converts its three point components
and clamps the three voxel coordinates directly, replacing two fixed-size
array maps. Invariant doubled scale, grid cutoff and solve-option values are
computed once per pass. The exact scalar multiplication/cast order and unsigned
clamp condition remain the same. Expected effect: less array/closure overhead
and repeated setup per voxel sample, especially for seam-heavy remesh inputs.
Native/WASM parity and the deferred timed matrix remain the retention gates.

## D80 — 0.5 direct normal/tangent hash and face reads

The tangent profile also samples remap hashing, while face tangent generation
and normal remap hashing used fixed three-element array maps. These paths now
load the three values directly. Signed zero still hashes as positive zero,
normal-bit shifts remain identical, and triangle corners are read in their
original order. Expected effect: less fixed-array iterator machinery in the
normal/tangent setup loops, with the strongest effect on large input meshes.
The exact differential and load-gated timing records will test the result.

## D81 — 0.5 caller-buffer OMM raster without staged output on sufficient fuel

The unchanged memory bar previously failed without a finite ratio because
C++ requests zero scratch for caller-buffer OMM rasterization while Rust staged
the complete result. `raster_state` charges exactly `(4^(level+1)-1)/3` visits;
its level-one edge-specialized path charges the same four children as the
ordinary recursion. After the existing destination-length and input checks,
the caller API now writes directly into the caller's buffer only when its
remaining full-call work limit covers that count. It reports zero new output
storage, and leaves any caller tail untouched. Under a tighter work budget it
uses the original staged path, preserving the destination and the exact
failure prefix. The shared setup helper keeps allocating and caller results
bit-identical. A focused test covers levels 0–3 in both state formats,
zero-scratch success, equal bytes, untouched tails and atomic low-fuel and
invalid-input errors.
Expected effect: eliminate the caller memory failure and an allocation/copy,
with a possible caller timing gain. The full differential sweep and timed
matrix are the retention gates; the separate OMM allocating API still owns
its output.

## D82 — 0.5 inline eight-triangle strip-buffer removal

The D73 stripify profile spends nearly half of Rust samples in `memmove` while
the hot loop shifts at most seven triangles in its fixed eight-slot buffer.
Both ordered removals now copy those few triangle records in a small inline
loop, preserving order and the subsequent valence updates. The start-triangle
heuristic also reads its three valences explicitly instead of an array-map
wrapper. Expected effect: avoid a libc call on every emitted triangle and
reduce tiny-array setup, targeting the million-triangle stripify maximum.
The C++ path also uses `memmove`, so only the load-gated paired measurement can
establish whether this safe-Rust specialization beats it. Native and WASM
output parity remain mandatory.

## D83 — 0.5 executed caller-buffer OMM identity witness

The differential driver now invokes caller-buffer OMM rasterization alongside
the allocating API for each raster case, asserts identical bytes, an untouched
caller tail, and zero requested Rust scratch, then emits the unchanged
allocating bytes for the pinned C++ comparison. This executes the new D81
path in both the native 30,000-case sweep and WASM identity run. It extends
evidence for the new caller behavior without changing the scalar C++ oracle
or the benchmark case and bar definitions.

## D84 — 0.5 native scalar log2 for OMM level selection

`opacity_map_measure` still called the software `libm::log2f` in standard
builds for each triangle with adaptive subdivision. It now uses `f32::log2`
there, while the `no_std` path retains `libm::log2f`. The input clamp,
rounding to a level, quantized keys and probe order are unchanged. Expected
effect: fewer instructions in adaptive OMM measurement, particularly on
large meshes; no effect when `target_edge == 0`. Native, WASM and no-default
parity are required before retaining the change. The load-gated timed run
will decide its measured effect.

The freshly rebuilt release drivers pass 30,000 native C++ differential
cases and 30,000 executed WASM identity cases with zero mismatches. The
all-feature focused tests and the no-default experimental test suite pass;
strict all-feature Clippy and `fmt --check` exit 0. These results establish
the output and feature-mode gates, while the timed effect remains pending.

## D85 — 0.5 inline the six-value OMM measurement hash

The prior worst-case OMM measure `perf record` has a distinct `hash_update`
sample bucket for 5.4% of Rust samples on the million-triangle connected
mesh. The helper processes exactly six quantized integers per triangle, so
it is now explicitly inlined at its sole call site. The hash multiplications,
updates, key values and probe order are identical. Expected effect: avoid a
million small calls and permit fixed-length loop optimization in large OMM
measurement; the impact on other families should be zero. Rebuilt native
and WASM differential identity and the load-gated timed matrix remain the
retention checks.

## D86 — 0.5 complete timing admission and interrupted-run checkpoints

The resumed review found that the old external load gate checked only time
and load. The coordinator now also requires no GPU lease holder and no active
`moss-scoreboard-*` user unit. `p05/timing_gate.py` records all four conditions
and repeats blocked checks every 120 seconds. Each calibration and timed pair,
and each `perf stat` and `perf record` invocation, must pass this admission.
Compilation and exact-output checks remain available while the gate is closed.
Core selection occurs after admission, immediately before measurement; pairs
retain that same core. The verifier checks admission evidence in every sample.

The expanded benchmark now atomically checkpoints completed cases outside git.
A resumed run accepts them only with identical Rust and C++ binaries, source
hashes, consumer overrides and an available original core. It rejects source
changes during a matrix and deletes the partial checkpoint after completion.
The timing bar, corpus, per-case batch sizes and 20 alternating pairs are
unchanged. Current-source records will replace D73 only after admitted runs.

## D87 — 0.5 atomic normal/tangent caller output after fallible setup

The D73 memory failures counted a complete staged output in each caller API.
Both generators now finish topology, remap, adjacency, group construction and
fallible scratch allocation before writing the caller buffer. At that point
the remaining counted work is at most one accumulation visit per corner; a
remaining-fuel check proves that no later operation can fail. A tighter fuel
limit retains the previous staging path and its exact failure prefix. Normal
smoothing scratch is allocated before the first caller write.

A conservative u64 bound for all hash probes, adjacency visits, possible
corner pairs, faces and accumulation permits initial admission with a byte
budget that excludes output storage. When that bound is too conservative,
the original memory admission remains, then the actual remaining-fuel check
can still eliminate staging after grouping. Requested storage excludes output
only when it is actually written in the caller buffer. No floating-point
operation or output order changes. This extends D81's atomic caller principle
to the remaining two memory-failing APIs, without a second algorithm pass.

The focused test covers all tangent options, three normal smoothing values,
identical output, untouched tails, reduced storage, memory rejection, and
every fuel limit through successful completion. The native/WASM differential
driver now executes both caller generators as an identity and storage witness.
The allocating output remains the pinned C++ comparison payload. Timed gains
and family memory acceptance remain pending the coordinator's admission.

## D88 — 0.5 execute differential parity with the no_std math path

The parity consumer now forwards a default `std` feature explicitly to the
crate, so `--no-default-features` builds the same std-based protocol driver
with the crate's no_std/libm path. The ordinary and no-default sweeps rebuild
their selected driver before execution and retain separate source- and
binary-identified compressed records. WASM identity similarly rebuilds its
driver and requires a current native sweep. This executes D84's adaptive level
selection against the same C++ oracle in both crate feature modes, rather than
using compilation alone as a no_std parity claim.

`perf` input fixtures are retained as gzip files with compressed and original
SHA-256 hashes; the verifier checks both. The raw fixture is removed after its
profile runs, limiting disk growth while preserving replayable evidence.

The resumed candidate passes each 30,000-case native, no-default/libm and
executed WASM sweep with zero mismatches, plus all 114 distinct benchmark
corpus shapes (including the million-triangle cases) against pinned C++.
All-feature, no-default, and no-default experimental test suites pass; strict
root and parity-driver Clippy and both formatting checks exit 0. Compressing
45 superseded profile inputs reduced their storage from 657,123,624 to
104,873,398 bytes without discarding their replay bytes. Timings remain
pending the complete admission; no D73 ratio is a current-source claim.

## D89 — 0.5 decoder fuel specialization and tangent probe keys

While timing admission was still closed, the D73 strip-decoder hot path was
specialized into two const-generic loops. A full-fuel call charges its stream
once and enters a loop containing no work-budget condition; a tight budget
retains the per-index charge and exact partial-output failure point. Sequential
index reads, restart handling, winding and destination slice advancement are
identical to D74. This makes removal of the successful-loop fuel branch explicit
instead of depending on optimizer loop unswitching.

The tangent remap loop now reads each candidate position, normal and UV once
and passes those same values to hashing and collision comparisons. Probe order,
canonical signed-zero handling and equality are unchanged. Expected effect:
fewer repeated view/index loads in collision-heavy remap work. Fresh native,
no-default and WASM differential checks and the admitted timing/profile runs
are the retention gates. The waiting benchmark process was stopped before any
sample and will restart with the replacement source and binaries.

## D90 — 0.5 raster-tree fuel specialization

OMM rasterization already has the exact full-tree visit count used by D81's
atomic caller admission. Both allocating and caller paths now charge that
count once when the remaining budget covers it, then use a const-generic
recursive routine with no per-node work-budget test. Tight budgets retain
the previous per-node charges, including the four level-one specialized
children, preserving their exact error and usage prefix. Texture sampling,
midpoint arithmetic, recursion order and packed writes are unchanged.
Expected effect: reduce work-accounting branches and writes in the profiled
OMM raster hot path. The existing level/state atomic caller test and fresh
native/no-default/WASM sweeps gate retention; timing still awaits admission.

## D91 — 0.5 initialize scratch once, while preserving reuse

Cache timestamps, fetch flags and strip valences were filled with zero after
`Workspace::prepare` had already zero-initialized any newly added entries.
The three 0.5 consumers now clear only the prefix retained from the preceding
call; growth still receives zeroes from `prepare`. Shrinking clears the full
newly used retained prefix, and later growth reinitializes the newly visible
entries as before. This removes redundant clearing for the fresh-workspace
benchmark calls without changing reusable-workspace behavior, memory limits
or counted work. The shared workspace allocator is unchanged.

Normal and tangent remap tables similarly initialize directly to their empty
u32 sentinel instead of zero-initializing and then overwriting the entire
table. Allocation sizes, checks, failure ordering and probe values are the
same. Expected effect: reduce initialization stores in the profiled cache,
fetch, strip and remap families. Exact feature-mode/WASM parity and the
admitted paired matrices qualify the change; no timing gain is assumed.

## D92 — 0.5 requested memory is a peak of live generator storage

D87 removed output staging, but projecting its storage against the retained
D73 C++ counters still exceeded 1.25 on tiny normals/tangents and disconnected
tangents. The source lifetime trace found that Rust's quota estimate summed
the remap table with later adjacency/groups even though the table is dropped
inside `build_remap` before those allocations. It also omitted the extra
adjacency offset and rounded tangent per-face storage (16-byte tangent,
4-byte group, 1-byte sign) to 24 rather than 21 requested bytes.

The generators now account the maximum of the remap phase and the later live
output-plus-scratch phase, using the actual requested element sizes and extra
offset. Retained workspace capacity is still added by `account_codec`; caller
buffers and allocator overhead remain excluded in both languages. A staged
low-fuel path still includes its complete owned output. This corrects the
comparison basis and permits budgets matching the true requested peak; it
does not lower the bar, remove an allocation from the count, or use RSS.

A focused externally visible budget test checks the pinned eight-vertex,
four-face C++ peaks of 228 normal and 248 tangent bytes, untouched caller tails,
and atomic rejection one byte below the tangent peak. Caller identity witnesses
now allow the remap phase to dominate both API peaks, so they do not assume
every input's peak decreases by exactly the output size. Fresh feature-mode
parity and both measured consumer memory matrices remain required.

The final D92 rebuild passes 30,000 native, 30,000 no-default/libm and 30,000
executed WASM differential cases with zero mismatches, and all 114 expanded
corpus shapes against C++. Normal/tangent caller witnesses compare f32 bits,
including signed zero, rather than float equality. The 11-test fixture run,
all-feature/no-default/no-default experimental suites, strict root and consumer
Clippy, formatting and diff whitespace checks pass. The admitted runner was
restarted only after these checks; no candidate timed sample ran before the
coordinator's time, load, lease and scoreboard conditions.

## D93 — 0.5 profiler success case and driver source identity

The profiling script wrote its detailed summary only after a failing family.
If a consumer eventually passed every family, the empty profile set would
therefore fail while hashing a missing summary. It now always writes a final
summary, including the valid empty set; no perf invocation is needed in that
case. The benchmark source identity also covers both C++ harness sources,
including the profiling driver, rather than only `bench.cpp`.

This changes evidence orchestration only. Production Rust and all executed
D92 native/no-default/WASM/corpus results are unchanged. The waiting runner
was restarted before any admitted sample, so both matrices will identify the
complete harness source. Timing still waits for load, lease and scoreboard
admission after the already-passed 02:30 threshold.

## D94 — 0.5 coordinator admission amendment and per-run CPU telemetry

The coordinator replaced the timing gate on 2026-10-05: admission now requires
no GPU lease holder and no scoreboard unit actively measuring. Load average
and the former time cutoff no longer gate admission. A unit whose MainPID has
exactly one child named `sleep` is waiting and is explicitly permitted; unknown
status or other active children block. Unit state, MainPID and child inventory
are retained with every admission. Blocked checks still repeat every 120 seconds.
The waiting D93 runner was stopped before any timed sample.

The same-core alternating pair protocol and three one-second least-busy
physical-core selection remain unchanged. Every measured pair now records
load before/after, elapsed seconds and `/proc/stat` busy/total ticks and usage
for every allowed logical CPU, including the selected core and its siblings.
Profiler stat/record runs retain the same CPU telemetry and load, with a
30-second subprocess limit. Benchmark batches target 20 milliseconds per
backend, cap at ten million iterations, and use one iteration for longer
calls; Rust requests have a 30-second response deadline. All benchmark inputs
are finite, previously checked expanded-corpus cases. Admission is checked
before calibration, memory probes, warm-up pairs, every measured pair, and
every perf invocation. A lease/measurement that begins within a pair is
observed at the next boundary: finish that pair, then pause.

The verifier requires the revised admission evidence and valid per-CPU
counters without applying a load threshold. Syntax, live waiting-unit
classification and counter/verifier agreement pass. Production Rust and D92
native/no-default/WASM exact parity are unchanged. Both consumer profiles
will rebuild from the source-bound harness before measurements.

## D95 — owner requires family acceptance and bounded focused iteration

The binding owner directive supersedes the documented-whole-family shortfall
alternative for tonight's 0.2.0 delivery: every 0.5 family mean must meet 1.25
under both profiles. Only genuinely unreachable individual edge cases may be
residuals with fresh profiles; a failing family is not accepted. The subsequent
measurement directive stops repeated full matrices. The D94 matrix was stopped
at 21 complete Moss cases and retained externally as diagnostic evidence.

Iteration uses only the touched families/cases, five alternating same-core
pairs per case, with separate diagnostic artifacts that cannot overwrite final
summaries. Initial focused diagnostics cover the still-unmeasured D74–D92
normal, tangent, vertex-cache and OMM-measure changes. Exact parity remains
mandatory before timing. The amended lease/measurement gate and per-CPU/load
telemetry remain in force. Affinity is released for both drivers before each
admission check, including blocked waits. At ten minutes the burst releases
CPU affinity for 30 seconds and selects a fresh least-busy physical core;
long admission waits likewise trigger a fresh selection. Every pair records
its CPU; Rust requests and perf subprocesses have 30-second deadlines.

The final full matrix runs once after focused optimization, starting at five
pairs per case and screening at 5, 10 and 20, stopping a clearly passing or
failing maximum at each look. The interval on mean log paired ratio uses
Student-t with a Bonferroni allocation of 0.05/3 to each of the three two-sided
looks (critical values 3.960786482770179, 2.933324088373988 and
2.625105913222785). This accounts for repeated screening; raw samples and
screen bounds are retained. Family means and memory still use stage 1.
Only cases still borderline at the twenty-pair cap receive D146's exactly
30 fresh pairs and two-sided 95% df=29 interval. That stage never pools
selection samples and never clears a family mean or memory failure. The owner
specifically requires fast bounded bursts; these rules supersede the former
fixed twenty-pair requirement and D146's stage-2 trigger for clear failures.

## D96 — five-pair baseline and fresh worst-case profiles

D95 focused diagnostics completed 48 cases per profile with exactly five
alternating pairs. Baseline artifacts, binaries, compressed fixture inputs,
perf data and source snapshots are retained in external `baseline-D95`.
The Moss means/maxima are: cache 1.394/1.875, measure 1.276/1.487,
tangents allocating 1.286/1.422 and caller 1.349/1.612, normals allocating
1.225/1.353 and caller 1.251/1.372. Default: cache 1.401/1.833,
measure 1.289/1.425, tangents 1.357/1.428 and 1.382/1.605,
normals 1.259/1.526 and 1.334/1.544. Every measured memory group passes;
normal/tangent peaks match C++ (ratio 1.0), measure is at most 1.156.

Fresh Moss whole-driver instruction ratios are cache 1.548, measure 1.384,
tangents 1.317 and normals 1.287. Cache samples concentrate in its analyzer
(86%) and topology validation (13%). Tangents concentrate in accumulation,
merging, remap and generator finalization; normal accumulation, merging and
smoothing dominate. These are diagnostics, not final acceptance.

The initial diagnostic profiler path ternary returned a string instead of a
Path. It stopped before perf calls. The corrected path resumed profiling
without discarding or reusing the completed timing samples; a reconstructed
original harness source is retained with its verified hash. No production
Rust changed between the two baseline consumer matrices.

## D97 — dispatch packed normal/tangent position reads once

Normal and tangent generators now choose a packed position reader once at
entry. Private monomorphized helpers read that slice directly; interleaved
and byte layouts use the existing checked Positions path. There is no new
public API, allocation, unsafe operation or SIMD. Validation, counted visits,
merge order, floating arithmetic, output and memory accounting are unchanged.
This targets repeated layout dispatch in the profiled accumulation/remap
paths, especially with default non-LTO consumer code generation.

The existing caller-storage/fuel contract test now compares packed allocating
results with packed, padded interleaved (including unused NaN padding), little
endian and big endian caller views. It checks output bits, untouched tails,
all fuel boundaries, atomic failures and identical counted work. Fresh exact
native/no-default/WASM sweeps and focused normal/tangent corpus checks gate
retention. Only those two families will be remeasured with five paired samples
under each profile before retaining the performance change.

D97's extended contract, strict all-feature/no-default Clippy, 30,000 native,
30,000 libm and 30,000 executed WASM differential cases pass exactly. Both
normal/tangent families also pass all sixteen expanded shapes including
million triangles. A counted-scan reader method is compiled only with its
experimental normal consumer. The five-pair D97 runner had no samples while
waiting on a GPU lease, so it was stopped to finish independent D98 changes.
No intermediate timing samples are reused or claimed.

## D98 — specialize disabled cache simulation and batch bounded OMM visits

The cache analyzer dispatches warp simulation, primitive-group flushing and
fuel accounting once into const-generic kernels. When warp simulation is
zero, it does not compute the three unused pre-update misses; zero group
size removes its counter/flush path. The checked topology pass, duplicate
index update order, wrapping timestamps, final unique scan, requested bytes
and tight-fuel prefix are unchanged. Existing exact-boundary tests now cover
all four warp/group combinations. This targets the freshly profiled seam-heavy
cache case (warp/group both zero), not an altered benchmark contract.

OMM measurement proves a worst-case bound of `triangles * (buckets + 1)`
after validation. If remaining fuel covers it, a const-generic loop counts
actual triangle/probe visits locally and charges once, including the exact
prefix on a numerical error. Overflow or insufficient fuel uses the original
per-visit path. Hash equality checks level before the six UV integers, matching
C++'s short-circuit order without changing deduplication. A focused test checks
identical numerical-failure work prefixes in fast and tight paths, and the
one-visit-short fuel failure. Memory requests and float calculations are unchanged.

Combined D97/D98 retention requires fresh native/libm/WASM parity, the touched
expanded corpus and strict checks, then only cache/measure/normal/tangent
five-pair diagnostics under both profiles. Full final matrices remain deferred.
The C++ reference build is reused only when its source/header/driver hashes,
exact flags, compiler executable hash and retained binary hash all match,
reducing repeated compilation without relaxing binary provenance.

D98 strict all-feature/no-default Clippy, twelve fixture/contract/budget tests,
30,000 native, 30,000 libm and 30,000 WASM cases and all 32 touched expanded
shapes pass exactly. Its waiting timing process again had no samples because
the heavy GPU lease remained held. It was stopped before D99 was applied;
these checks are functional proof, not measured speedups.

## D99 — initialize strip outputs as they are emitted

The allocating strip/unstrip paths previously reserved and zero-initialized
the full worst-case bound before overwriting the used prefix. Both algorithms
emit sequentially and never read output, so a private monomorphized writer
now appends into a fallibly pre-reserved Vec. Caller writers still copy the
same values into the provided slice, preserving each failure prefix and tail.
Every valid append fits the checked upstream bound; no allocation is needed
after the initial reserve. Debug assertions guard that capacity invariant.
There is no unsafe code or uninitialized Rust value. Requested output storage
still counts the full reserved bound, so memory accounting and limits are
unchanged. Input validation, visit charges, restart/degenerate handling and
allocation-failure ordering remain unchanged.

This targets the admitted D94 million-triangle allocating strip case at 1.795
while its caller form was 1.113. The old data is diagnostic evidence, not a
final qualification under the new early-stopping rule. A focused contract test
compares allocating/caller results and counted work at every fuel boundary,
checks that partial writes equal the exact completed result prefix, and checks
untouched tails. Fresh exact feature-mode/WASM/corpus proof precedes a six-family
five-pair diagnostic. No timed D97/D98 samples are discarded or reused because
none had been admitted during their lease waits.

D99 strict Clippy in all-feature and no-default modes, all thirteen phase-0.5
fixture/contract/budget tests, 30,000 native, 30,000 libm and 30,000 executed
WASM cases and all 48 distinct touched expanded shapes pass exactly. The
six-family diagnostic again reached the admission gate without a timed sample;
the capture GPU lease remained held. It was stopped before the harness-only
D100 change, so no measurement is discarded or pooled.

## D100 — implement the owner's bounded final maximum decision

Iteration still requires an explicit touched-family list and exactly five
interleaved pairs per case. Diagnostic `all` is rejected. Final mode alone
requires the complete matrix, screening after 5, 10 and 20 pairs and stopping
at the first clear result. Each screening uses the paired mean log ratio and
a two-sided Student interval with alpha 0.05/3 for the three possible looks
(Bonferroni correction); critical values for df 4/9/19 are
3.960786482770179, 2.933324088373988 and 2.625105913222785. Clear failures
return for optimization; only cases still borderline at 20 enter D146.

D146 collects exactly 30 fresh alternating pairs using the same retained
binaries, resident input and calibrated iteration count. It selects a fresh
least-busy physical core, then fixes that core for the entire case, including
admission and burst pauses. Its two-sided 95% mean-log interval uses df29
critical value 2.045229642132703. Upper endpoint <=1.5 passes; lower endpoint
>1.5 fails; overlap is INCONCLUSIVE and fails. Stage-one medians, maxima,
family geometric means and requested-memory ratios remain unchanged and
visible; stage-two samples never enter their calculation. Family means still
must be <=1.25 and memory <=1.25 under both profiles.

Both streams retain pair order/index, load before/after, CPU utilization,
admission and elapsed time. Checkpoints now retain all core selections. The
verifier recomputes screening/interval decisions, rejects premature or late
stopping, requires exactly the 160 cases/21 API groups and consumer settings,
and validates stage-two input/binary/core identity and fresh admission times.
Ten-minute burst release and lease/scoreboard pause behavior remain D94/D95.
Eighteen synthetic protocol checks using a retained admitted utilization record
passed, including early-stop rejection, bad identities, mixed-core sampling,
old timestamps and wrong alternating order. These are harness correctness
checks, not performance measurements or production changes. The full final
matrix remains deferred until focused optimization is complete.

## D101 — remove proven redundant normal/tangent loop work while gated

The D100 six-family timing process still had no admitted samples when stopped;
the capture lease remained held. All-feature, no-default and libm/experimental
root test suites had meanwhile passed. Production changes remain grounded in
the retained D95 normal/tangent profiles, whose hottest loops include smoothing
and tangent accumulation; no speedup is claimed before fresh diagnostics.

Normal smoothing's scratch allocation is freshly default-initialized to
positive zero before its first use. Remove that first redundant full-buffer
clear, retaining the same clear before every subsequent smoothing pass. Caller
and allocating paths both provide this fresh storage, so arithmetic, padding,
allocation ordering, requested bytes and error atomicity are unchanged.

Tangent accumulation dispatches once on a checked bound of three visits per
face. If remaining fuel covers it, a const-generic loop skips per-face work
checks, counts three visits only for each nondegenerate tangent face and
charges once on completion. The tight/overflow fallback retains the previous
per-face/per-corner exhaustion point. All corner math and iteration order are
identical; finite input validation and all fallible allocation precede this
loop. Existing exact work-boundary/caller tests and fresh native/libm/WASM
and expanded touched-case differential checks must pass before diagnostics.

D101 strict all-feature/no-default Clippy, the thirteen phase fixtures and
contract/budget tests, 30,000 native, 30,000 libm and 30,000 executed WASM
cases and all 48 expanded touched shapes pass exactly. The six-family,
five-pair runner rebuilt its Moss binary and is polling the GPU lease gate;
the current scoreboard MainPID has only a sleep child and does not block.
Current-candidate speed and the final complete matrix remain unmeasured.

## D102 — branchless fetch misses under a proven byte bound

The D101 runner remained asleep at the lease gate with no admitted timed
samples. Its all-feature, no-default and libm/experimental test suites and
consumer formatting also passed. The functional-artifact preflight reported
no non-benchmark errors; the final benchmark records still refer to D73.
The waiting process was stopped before this production edit.

D73's default fetch mean 1.332/max 1.518 and instruction ratio 1.42 motivate
removing its per-cache-miss overflow branch. With the existing vertex-size
range 1..=256, an index spans at most five 64-byte lines. At most
`u32::MAX / (5*64)` indices therefore prove the entire u32 accumulation safe.
A const-generic kernel counts misses branchlessly in that bounded case.
Larger streams retain checked additions and the identical overflow error/work
prefix. Fuel dispatch retains both the locally counted bulk path and exact
per-index/per-line exhaustion fallback. Validation, visited flags, line
order, tag updates, arithmetic and requested memory remain unchanged.

Existing fetch exact-work-boundary tests, native/libm/WASM differential sweeps
and expanded touched shapes gate retention. The next five-pair diagnostic
adds only fetch to the six already changed families (88 rows per profile),
not a full matrix. No speedup is claimed before admitted measurements.

D102 passes strict all-feature/no-default Clippy, all-feature/no-default/
libm-experimental suites, thirteen phase fixtures/contracts/budget tests,
formatting, 30,000 native, 30,000 libm and 30,000 executed WASM differential
cases, and all 56 expanded touched shapes exactly. The existing fetch budget
test now covers sizes 1, 12, 64, 127, 128, 255 and 256, including five-line
unaligned vertices. The seven-family runner admitted 39 Moss cases before a
new capture lease began; it finished its pair and released affinity at the
next boundary, then resumed 120-second blocked checks.

Complete five-pair Moss groups so far: strip allocating GM 0.962532/max
1.149211, strip caller 0.968451/1.184244; unstrip allocating 1.077230/2.182479,
unstrip caller 1.265043/6.077575. Memory ratios are 1.0. Seven of eight cache
cases are present; a partial mean is not an accepted family result. The default
profile and remaining Moss groups/profiles are pending. Static inspection of
the retained current Rust/C++ decoder binaries motivates a prepared, unapplied
branchless-degeneracy/checked-triangle writer candidate. Prepared OMM packed-UV
and range-accounting code is likewise unapplied pending its measurements.
Neither prototype is a retained optimization or a measured speedup.

## D103 — prioritize the measured decoder failure during the long lease wait

After more than thirty minutes paused at a subsequent capture lease, preserve
all 39 completed D102 Moss cases, their exact Rust/C++ binaries and bound
production/harness source snapshot externally in `baseline-D102-partial`.
Stop the sleeping runner, retaining every sample. Its complete sixteen unstrip
cases also seed a source/binary-bound historical worst-case perf run before
new-candidate timing. Remaining changed families/default cases are pending,
not silently accepted or discarded; this was a focused diagnostic, never the
full final matrix.

The decoder now tests its three winding-invariant degeneracy comparisons
before any winding swap, using boolean bitwise conjunction to avoid parity-
dependent short-circuit branches. This preserves the same integer result and
all visit/failure prefixes. Caller emission advances checked whole three-index
chunks instead of adding/checking a growing output range; the initial bound
proves sufficient chunks for every possible emission. Extra tails, all tight-
fuel partial prefixes, heap requests and the allocating append path remain
unchanged. No explicit SIMD or unsafe code is introduced. The retained scalar
C++ library's compiler-generated packed loads/shuffle are diagnostic codegen,
not a relaxation of the enforced no-hand-SIMD baseline.

Static inspection motivates these changes but does not prove a speedup.
Existing all-fuel prefix/tail tests and fresh exact feature-mode/WASM/expanded
unstrip proof precede sixteen-row, five-pair diagnostics under both profiles.

Diagnostics now accept optional `--cases family:shape` filters, restricting
only the named families and keeping other selected families complete. This
allows the pending million-triangle cache case without repeating its seven
completed D102 cases. Such mixed-source diagnostic observations remain
separately identified and cannot qualify a family. Final mode rejects every
case filter and the verifier requires an empty filter map and all 160 cases.
Checkpoint identities include filters; resumed same-source streams require
identical allowed-CPU inventory and select a fresh least-busy physical core
while retaining their earlier core selections and completed case samples.

## D104 — match caller-page commitment and initialize owned decoder chunks

Preserve all fifteen completed D103 Moss cases, binaries and bound source
snapshot externally in `baseline-D103-partial` before stopping its sleeping
last-case runner. D103 passed thirteen fixtures/contracts/budget tests, strict
checks, 30,000 native, 30,000 libm and 30,000 executed WASM cases and eight
expanded unstrip shapes. Its allocating million case still measured 2.897.
D102's frozen worst caller perf run completed: Rust/C++ instructions
4,850,674,589/3,840,368,115 = 1.263, cycles 1,452,598,786/1,039,605,494;
branch misses 106,787/145,491. These whole-process counts include setup and
do not support blaming the 6.078 wall ratio solely on decoder branch misses.

An untimed mechanical allocation probe, using the actual host's optimized
Rust/C++ standard-vector constructors and reading `/proc/self/stat` minor
fault counts, identified asymmetric caller setup. For 32 MB of zeroed u32s,
Rust incurs 1 fault at construction then 7,813 on first writes; C++ incurs
7,816 at construction then zero on first writes. Rust's zeroed Vec obtains
lazy calloc pages whereas C++ vector initialization has eagerly touched them.
The old separate warm-up requests discard their caller buffers, so they do
not commit the later measured request's storage. This is an identified harness
asymmetry, not a lowered timing bar or an accepted performance result.

Both phase-0.5 drivers now write opaque positive zero to one element per
<=4 KiB span and the final element of the used caller-buffer type, after all
setup and before the API timer. This commits every caller output page in both
languages, preserves initial contents, and adds no timed instruction or heap
request. Scratch allocations and allocating outputs remain inside the API
measurement. Unused Rust caller types are not touched. All six caller families
use the same policy; only phase 0.5 changes. Native, libm and WASM parity and
fresh consumer binaries qualify retention.

The allocating unstrip API now pre-reserves and initializes its full checked
bound, then uses the same checked triangle-chunk decoder as caller storage
and truncates to the returned count. It removes the per-emission Vec reserve
and length update, matches C++'s initialized owned output, and preserves all
quota checks, requested bytes, allocation order and exact work/failure prefixes.
Strip's successful append-output implementation remains unchanged.

The D104 resumed validation passes all thirteen phase tests, strict root
checks, 30,000 native, 30,000 libm and 30,000 executed WASM cases and eight
expanded unstrip shapes exactly. No D104 timed sample had been admitted when
its initial waiting process was stopped for the following harness refinements.

The C++ normal/tangent sinks now retain the output pointer directly through
an opaque compiler barrier, matching Rust's buffer retention instead of
converting possibly negative float components to unsigned integers. The bound
helper barriers still consume their varying scalar results. This removes a
harness-only undefined conversion; every C++ API/output remains unchanged.

Profiling checks binary hashes before running, selects a fresh least-busy
physical core after first admission and input preparation, then fixes that
case's core for both backends and stat/record runs. Each family retains its
three-interval selection; `benchmark_cpu` remains the earlier diagnostic CPU.
The verifier checks both identities and the independent profile-core metadata.
This prevents an initial long gate wait from starting a profile on a stale
selection. Children exit and release affinity after each bounded run; the
unaffined parent waits at the lease/measurement gate. Production parity is
unchanged by these orchestration-only refinements.

D104 consumer strict Clippy now also passes all-feature and no-default checks.
Four negative CLI probes reject an out-of-range diagnostic case, a filter for
an unselected family, every final-mode case filter, and diagnostic full matrices
before a build or timing starts. Archives copy only the profile summary's
referenced input/data files and verify their hashes, avoiding stale family
files and keeping retained evidence small. Fresh D104 pairs remain pending the
shared lease; no final matrix or acceptance is claimed.

While D104 waits, an external OMM prototype preserves packed/interleaved UV
reads and batches charged probes over 128-triangle ranges whenever the whole
input bound cannot fit the remaining fuel. Its new boundary/source contract
passes in std and libm builds. An additional 480 untimed differential calls
against the retained implementation cover varied strides, finite/nonfinite
UVs, range sizes and fuel limits, with identical results/errors, work prefixes
and requested bytes. This prototype is not applied to production or measured;
it remains a candidate only if fresh OMM diagnostics miss the bar.

Fresh D104 complete Cargo test suites pass in all-feature, no-default and
no-default-plus-experimental modes; every suite reports zero failures. Logs
and compact suite counts are retained externally in `D104-full-tests.json`.
The external OMM prototype also passes all 480 retained-implementation calls
in native std mode, for 960 total std/libm differential comparisons.

A second external prototype specializes normal accumulation's covered/tight
work paths at compile time, retaining the original upfront/full or per-corner
charge order. All thirteen phase contracts and 600 bit-exact owned/caller
comparisons pass in each std/libm mode, including caller tails, errors, usage
and requested bytes. Like the OMM prototype, it is not production or timing
evidence, and will only be retained if the pending focused diagnostics warrant
it. D104's unstrip archive and the following five-family pending diagnostics
are chained with explicit exit/source/binary/row-count checks; only one own
benchmark process can measure at a time. The separate four-family remaining
changed-path script is prepared but not launched.

## D105 — bounded OMM probe accounting and covered normal accumulation

D104 admitted zero samples during its initial lease wait. Preserve its complete
untimed source and successful proof logs externally before stopping only the
own sleeping benchmark/chainer and applying the two externally validated
prototypes. The D95 retained diagnostics still miss OMM measure's family mean
and several normal groups; these are production candidates for those measured
hot paths, not unmeasured speedup claims.

OMM measurement dispatches packed UV pairs once, retaining the general padded
reader and identical six finite checks/math. When the whole-input probe bound
cannot be covered, each 128-triangle range independently proves its maximum
probe visits and selects batched or exact per-probe charging. Shared hash/output
storage and global source indices preserve deduplication and source order;
no allocation or requested-memory change is introduced. A production contract
covers the 129th triangle, packed/padded unused NaNs, exact exhaustion and the
first invalid UV across the boundary. External std/libm differential comparisons
match retained values/errors/usage/storage in 960 calls.

Normal accumulation specializes the covered and tight work loops at compile
time, keeping upfront complete charging or the original exact per-corner
prefix. Every floating operation and corner order is unchanged. All phase
contracts and 1,200 external std/libm owned/caller comparisons pass with exact
float bits, tails, errors, work and requested bytes. No unsafe or hand SIMD.

Diagnostic full-matrix rejection now checks the resolved family set, including
a spelling with all fifteen explicit comma-separated names. The final-only
full matrix, five-pair diagnostics, two-minute gate and bounded early/D146
policy remain unchanged. Rebuild and fresh native/libm/WASM proof precede
focused timing; no D105 performance qualification is claimed yet.

D105 passes fourteen phase tests, strict library/consumer Clippy in both feature
modes, 79 all-feature, 74 no-default and 79 no-default-plus-experimental tests.
Fresh standard native, libm native and executed WASM sweeps each pass 30,000
cases. The expanded check was extended after its first eight-shape run, then
rerun successfully across all 24 unstrip/normal/OMM shapes, including each
million-triangle input. The explicit fifteen-name diagnostic full-matrix CLI
probe fails before building, as intended. All source/binary identities and
proof logs are retained; the restarted focused queue still requires admission.

A final D105 orchestration review found that the next case's untimed fixture
was prepared before its calibration admission check. Admission now precedes
case setup, so a lease starting during the prior case's last pair pauses before
new large fixture work. Both stage-one and fresh D146 pairs release parent and
Rust affinity immediately after recording their counters, before checkpoint
work or the next gate. This tightens the finish-current-pair rule without
changing a timer, iteration, threshold, sample or production output. The initial
D105 wait admitted no sample before its own sleeping workers were restarted;
the production's 90,000 differential, 24-shape and full test proofs remain
current because no Rust/C++ source or build configuration changed.

The live D105 benchmark/verifier passes all eighteen focused protocol checks
for conservative early stopping, fresh D146 pass/fail classification, and
rejection of pooled/extra stages, wrong hashes, stale admission times, switched
cores and reversed pair order. These are untimed protocol checks using retained
admission/utilization metadata; they are not performance samples.

D105 completes and archives sixteen five-pair unstrip cases per consumer in
`baseline-D105-unstrip`, including each bound source, Rust/C++ binary, and fresh
worst-case stat/record profile. Caller means/maxima are 0.723/1.157 Moss and
0.742/1.224 default; all sixteen caller cases pass time and requested memory.
Allocating means are 0.885/0.891, but the tiny and million cases still miss
maximum: Moss tiny 1.543, million 1.851; default tiny 1.560, million 1.791.
Moss worst-case whole-process instruction ratio is 1.065 and cycles are nearly
equal to C++, so repeated long-loop profiling does not reproduce the short
owned benchmark's wall gap. This is motivation to inspect allocator/setup
state, not permission to waive either case. Their residuals are not accepted.

The queued pending-path diagnostic first measures the million-triangle Moss
cache case at 0.998 (five pairs), then pauses before fetch at a new GPU lease.
Its other groups/default profile remain pending. The seven earlier D102 cache
cases are preserved separately; source-mixed diagnostics do not qualify a
family. No final full matrix has run.

## D106 — resident benchmark inputs and direct decoder validation

Preserve the completed one-row D105 Moss pending cache diagnostic alongside
its identical source/Rust/C++ snapshot in `baseline-D105-unstrip`, then stop
only the own waiting workers. Fresh owned unstrip profiling is 1.065/1.068
instructions Rust/C++ across consumers; the short benchmark still misses its
tiny and million cases. No exception is made.

The C++ benchmark keeps immutable case inputs resident throughout calibration,
warm-up and every pair. Rust previously regenerated all case vectors and
constructed/dropped a large serialization/hash buffer for every BENCH request,
outside the API timer. Those asymmetric allocator changes can relocate owned
outputs onto fresh pages. Rust now retains exactly one immutable case and its
verified hash, keyed by seed/count/triangles/style; changed keys drop the old
case before construction. All families use the same input-residency policy as
C++. This reduces untimed machine occupancy without warming an owned output
outside its API or altering the timer/iterations/bar. Two untimed contracts
prove same-key storage reuse and byte-identical fixture hashes/output after
seed/layout changes. Timing effects remain unmeasured; current archived ratios
cannot be presented as matched algorithm-only speedups.

The owned unstrip wrapper gains an inline hint. Its validation helper uses
explicit typed arguments instead of a captured mutable closure, preserving
all checked bounds, allocation/quota order, work prefixes, outputs and caller
tails. All thirteen existing external phase contracts pass before application.
Fresh production native/libm/WASM proof and focused five-pair unstrip cases
0/7 precede the pending cache/fetch/measure/normal/tangent groups. The latter
require fresh method-consistent observations; the earlier cache row is kept
as historical evidence rather than pooled into the new source.

D106 consumer cache tests passed, but strict Clippy detected an included-source
test-layout lint: the WASM library includes the CLI before its FFI declarations,
so the CLI's final test module preceded more library items. Move the two tests
to a dedicated integration target including the same private CLI helpers. No
production statement changes; new same-source consumer strict checks and
differential identities precede timing. The own initial D106 wait admitted
zero samples before restart.

After the integration-test layout correction, D106 passes both consumer cache
tests in std/libm mode and strict consumer Clippy with all features/no defaults.
Production's fourteen phase tests, 79/74/79 complete feature-mode tests,
30,000 native, 30,000 libm and 30,000 executed WASM cases all pass; eight expanded
decoder shapes remain exact. The 0/7 focused timing source is frozen and the
archive/pending-path chain has explicit success/identity guards. No fresh D106
sample has been admitted yet; only the binding lease/measurement gate applies.

While D106 remains gated, an external raster candidate mirrors scalar C++'s
`rasterizeOpacityRec<2>/<4>` dispatch: specialize validated state format once
alongside covered/tight charging, retaining every floating operation, sample,
bit emission and work prefix. All external phase contracts and 2,400 std/libm
owned/caller differential comparisons pass for levels 0–4, varied UVs, padded
texture strides, tight fuel and untouched tails, with identical requested
storage. It remains outside production and unmeasured, to be considered only
if fresh raster diagnostics still miss; the timed D106 source stays frozen.

The waiting interval also produces an external remesh candidate. For each
voxelization pass, a checked triangle-count times maximum sampling bound selects
a const-generic uncharged loop only when all visits fit remaining work. That
loop totals actual visits (including degenerate triangles) and charges once;
the tight path retains the original triangle/sample exhaustion prefixes.
There is no new allocation, floating operation, or public API. All thirteen
external phase contracts pass, followed by 3,600 comparisons per feature mode
(7,200 std/libm total) covering bound/allocating/caller outputs, zero/short/full
destinations, degeneracies, all four options, four resolutions and exact fuel
boundaries. Values, tails, errors, work and requested bytes match production.
This candidate remains unapplied and unmeasured pending fresh remesh evidence.
The D106 decoder run still has zero samples after an hour of lease admission
waits; the scoreboard's sole sleeping child does not block it.

## D107 — allocate only matched selected caller outputs in both drivers

Resume by reviewing the complete uncommitted implementation, D95/D100 owner
protocol, D106 validation receipts and checkpointed measurements. D106 finishes
its four five-pair decoder cases under both profiles: Moss allocating/caller
focused means 1.147/0.972 and maxima 1.295/1.047; default focused means
1.319/1.033 and maxima 1.756/1.202. These two-shape focused means are not full
family means. The default tiny allocating case remains an ordinary failure,
not an accepted edge residual. The million allocating ratios are 1.017/0.991.

Retain the exact D106 source archive, both timing streams and binary hashes in
`frozen-D106`. Stop only the own sleeping obsolete-family chainer and waiting
profiler after all focused pairs have completed. Complete the pending default
perf evidence from that frozen artifact directory, using its original Rust
and C++ binaries; its source never changes. No D106/D107 samples are pooled.

Finish the external conditional-buffer candidates before applying them. The
initial external Rust draft redundantly conditioned the untimed parity OMM
branch; discard that change. Both drivers now allocate only the selected caller
API's exact bound: strip bound, actual prepared unstrip bound, raster entry
size, four/three floats per tangent/normal corner, or the remesh bound. No
caller-output vector is allocated for an allocating or allocation-free API.
Rust flattens UV storage only for OMM measure. Each selected caller buffer is
still initialized and page-committed before the timer; owned output and scratch
remain inside the timer. All timer loops, calibration, memory accounting,
input residency, thresholds and scalar C++ options remain unchanged. This is
matched untimed setup, not an algorithm speedup or a relaxed acceptance bar.

External standard/libm drivers pass strict Clippy and 102 representative exact
C++ comparisons each, with matching input hashes and output bytes. C++ syntax
validation passes. Fresh production consumer checks and native/libm/WASM
records precede a five-pair recheck of only decoder shapes 0/7, then the pending
changed families. The D106 failing maximum still requires optimization; final
acceptance requires all complete family/API means and memory ratios to pass
under both profiles and D100/D146 maximum decisions.

D107 production refresh passes both consumer resident tests in each feature
mode, consumer formatting and strict Clippy, fourteen phase contracts, and
30,000 native, 30,000 libm and 30,000 executed WASM cases exactly. The frozen
D106 default profile completes from its original retained binaries: tiny owned
decoder instruction ratio 1.373, with 63.4% of Rust samples in unstripify and
16.3% in freeing. Source inspection finds reserve_output out of line in the
default binary, preventing knowledge of the returned empty Vec from simplifying
initialization. A one-annotation external prototype is under existing-contract
validation while D107 remains frozen. It is not yet a production/timing change.

## D108 — expose owned strip allocation and decoder wrapper to consumers

Archive complete D107 four-case/five-pair diagnostics and fresh profiles in
`baseline-D107-unstrip`, verifying both source/binary snapshots before source
changes. Moss focused allocating/caller means are 1.190/1.156, maxima
1.317/1.251; defaults are 1.369/1.049, maxima 1.659/1.206. Both million
allocating cases pass, but the default tiny allocating failure is not waived.
Its fresh instruction ratio is 1.370. These are two-shape diagnostic means,
not complete family means or final maximum decisions.

D106/D107 default assembly retains reserve_output as an opaque call, and the
owned wrapper performs vector-length/capacity dispatch plus dynamic workspace
accounting despite the consuming benchmark creating a fresh default workspace.
The external inline-helper prototype passes fourteen phase contracts in both
standard/libm modes. Its generated code removes the helper call, but a plain
inline hint still leaves the owned wrapper out of line. Require inlining for
the small fallible allocation helper and the owned unstrip wrapper so consumer
optimization can propagate the empty Vec and caller workspace. Algorithms,
allocation/quota order, fallible reserves, work charging, failure prefixes,
checked bounds and outputs do not change. No unsafe or hand SIMD is added.

Fresh feature-mode checks and native/libm/WASM proof precede five-pair decoder
shapes 0/7 under both consumers. The helper is shared with owned stripify, so
stripify also receives fresh focused coverage before final qualification.
All families still require the unchanged owner bars and D100/D146 protocol.

## D109 — admit brief free-lease windows without synchronized long polling

D108 finishes validation with 79/74/79 passing complete feature-mode tests,
strict root/consumer checks, 90,000 native/libm/WASM exact differential cases
and all eighteen final/D146 protocol checks. It admits zero timing samples
while repeated two-minute polls find an occupied lease. A manual receipt at
09:45:23 finds FREE and a scoreboard with only a sleep child just fourteen
seconds after the runner's blocked receipt. This demonstrates that the fixed
long cadence misses available windows; machine load is not the blocker.

Stop only the own zero-sample waiting queue/driver and change phase-0.5 gate
polling to fifteen seconds. The owner explicitly requires admission whenever
no lease holder or actively measuring scoreboard is present; polling cadence
is orchestration, not a measurement threshold. Every admitted boundary still
checks fresh status, releases waiting affinity, selects a fresh least-busy
core after long waits, completes only the current pair if a lease starts, and
uses the same ten-minute burst limit. No timer, corpus, samples, statistical
screen, D146 protocol, production library or Rust/C++ driver changes. No
previous measurements are pooled into this source identity. Native/libm/WASM
proof remains current because those sources and build settings are unchanged.

## D110 — calibrate resident batches after matched warm-up

Archive complete D108/D109 decoder diagnostics and fresh profiles in
`baseline-D109-unstrip`. Default focused cases pass; Moss tiny allocating
measures 1.849, while its fresh whole-process instruction ratio is 1.281.
Moss tiny's cold one-call probe selected only 14,358 iterations; its resident
Rust/C++ medians are 62.4/34.0 ns, so batches last roughly 0.9/0.5 ms rather
than the intended twenty milliseconds. Its five ratios include 2.865/3.098,
so the undersized batch is a concrete method problem, not a residual waiver.

After the existing equal-iteration warm-up pair, recalculate equal iterations
from the maximum measured resident per-call time of those two warmed batches.
Keep the twenty-millisecond target, ten-million cap, one-iteration minimum for
large calls and 100,000 minimum for helpers. When the count changes, both
backends receive another matching warm-up at the final count before sampling.
Retain cold/warm per-call times and both iteration counts with each row. Owned
allocation remains inside every API call; no returned output is prewarmed or
reused. Input residency, corpus, pair order, core/gate, ten-minute burst,
requested memory and D100/D146 thresholds/sampling remain unchanged.

No production Rust/C++ or build setting changes, so D108's exact native/libm/
WASM and feature-mode proofs remain current. Fresh five-pair decoder shapes
0/7 under both profiles check the corrected batch calibration before any
pending-family diagnostics. Final evidence must use the corrected method;
older streams remain source-identified diagnostics and are never pooled.

The first D110 launch rejected a busy executable before any calibration or
sample: an orphan pending-family child had already launched during the D108
queue-parent handoff. Stop its own shell, then stop the benchmark at a blocked
pair boundary. Preserve all 36 completed old-method Moss cases, their original
source tar and both binary hashes in `baseline-D109-pending-partial`. Its native
source and loaded method were still D109; the on-disk D110 Python edit would
have correctly rejected its final qualification. No record is pooled or
accepted, and the corrected D110 driver restarts only after that child exits.

## D111 — select each benchmark API outside both timed loops

Archive D110's complete four-row/five-pair decoder diagnostics and fresh
worst-case profiles in `baseline-D110-unstrip`. Warmed batch calibration does
not clear Moss tiny's 2.889 maximum; default focused decoder rows pass. No
maximum is waived. All raw cold/warm calibration and iteration counts remain
visible, and previous short batches are not reused.

The Rust driver's timed loop previously matched the runtime operation string
on every iteration, while C++ switched on an integer. Trivial helper loops
were already dispatched outside timing, but ordinary small APIs still relied
on different compiler unswitching decisions in their large multi-family loop.
Both drivers now select the API once before entering the timer and invoke a
statically typed Rust closure/C++ lambda loop. Bodies are copied mechanically:
API parameters, caller/allocating branches, fresh Rust workspace creation and
destruction, owned-output drop, result barriers, requested-memory collection,
iteration counts and timer duration are unchanged. Compiler-independent API
selection is a matched method correction, not a measured library speedup.

The external Rust candidate passes strict Clippy and the C++ candidate passes
syntax validation before application. Fresh consumer contracts and production
native/libm/WASM identity precede five-pair decoder shapes 0/7. D108 library
source and its full feature-mode tests remain unchanged. Final timing and the
pending changed-family diagnostics require the D111 matched-dispatch method;
source/binary snapshots prevent pooling older-method results. All owner bars,
fifteen-second lease polling, ten-minute bursts and D100/D146 rules persist.

The separately retained old-method partial Moss fetch group has mean 1.335
(maximum 1.442); it is a failing whole-family diagnostic, not a residual.
An external safe-Rust fetch candidate replaces general ceiling division with
a proven-safe add/shift, totals known-valid cache-line visits outside the inner
line loop, and sums exclusively zero/one visited flags. Its standard and libm
contracts each compare 23,220 calls against production across every vertex
width, exact fuel boundaries and workspace reuse. It is not applied or timed;
fresh method-consistent fetch measurements decide whether it is needed.

## D112 — initialize small decoder outputs by emission and simplify fetch work

Preserve D111's full focused streams and fresh profiles in
`baseline-D111-unstrip` before applying production changes. Default focused
allocating/caller means are 1.174/1.097, maxima 1.293/1.338. Moss caller mean
1.018/max 1.149 and million owned ratio 0.998 pass, but tiny owned 2.883 fails.
Its fresh whole-process instruction ratio is 1.227 and cycle ratio 2.243;
Rust profile includes 17.7% in memset, versus 2.15% C++. Stable warmed batches
rule out the previous undersized-batch explanation. No residual is accepted.

Owned unstrip outputs with checked bound <=64 u32s use the already existing
safe Vec append emission: only emitted triangles are initialized, and the
successful Vec length is their count. Larger outputs retain the initialized
whole-chunk path that passes the million case. Both paths make the same single
fallible full-bound reservation and enforce identical checked bounds, quota/
allocation order, charging, failure usage and exact output. Caller decoding
is unchanged. External standard/libm runs each pass fourteen phase contracts,
including every existing tight-fuel owned/caller prefix and tail witness.

The old-method complete Moss fetch diagnostic misses the family mean at
1.335 across eight cases, with large-case ratios 1.298–1.442. Dispatcher cost
is negligible relative to these large calls; the safe instruction reductions
already validated externally are therefore worth retaining without another
full unchanged-group iteration. Replace general ceil division by (end+63)/64:
validated total vertex bytes <=isize::MAX proves that addition safe on 32/64
bits. In overflow-checked mode preserve each exact visited prefix; in the
proven-safe mode add the whole index count and each tag-span length instead
of incrementing a visit count in the inner line loop. Sum the visited bytes,
which can only be zero/one after existing initialization and writes. All
23,220 standard and 23,220 libm width/fuel/reuse comparisons match output bits,
errors, counted work and requested storage. No unsafe or explicit SIMD.

Remove two whitespace-only C++ driver lines identified by diff-check. Timed
statements and dispatch/calibration/gate/statistical method do not change.
Fresh full feature-mode checks and native/libm/WASM proof precede focused
five-pair decoder shapes 0/7 and complete eight-shape fetch under both profiles.
Only those touched production families are remeasured in this iteration.

D112 initial root strict Clippy rejects the deliberate add/shift as manual
ceil division; consumer checks had passed. Add a scoped lint allowance to that
single let statement, with the existing non-overflow proof and codegen reason.
No API or timing sample had run. Restart strict/full parity validation after
this source-only annotation. The external no-restart adjacent-window decoder
candidate also passes fourteen standard/libm phase contracts, including first/
second input exhaustion; it remains unapplied pending the small-append timing.

## D113 — retain output storage directly and specialize common decoder/fetch loops

Archive D112's complete twelve-row streams and worst-case profiles for both
consumers in `baseline-D112-focused`. Moss fetch mean/max are 1.290/1.359;
all its maxima and memory pass, but its family mean does not. Default fetch
also misses the mean. Moss tiny owned decoder remains 2.835; no edge residual
is accepted. Its full requested bound and million path remain exact.

An identified sink asymmetry survives matched dispatch: Rust made entire
owned Vec descriptors opaque through by-value black_box, while C++ retained
pointers/scalars and let its destructor use known metadata. Retain each Rust
owned output's data pointer and count, then drop it normally; the inline helper
never makes ownership/capacity metadata opaque. C++ retains the corresponding
owned pointer/count. Explicitly retain all six caller output pointers in both
drivers, including previously count-only strip/unstrip/raster/remesh sinks;
normal/tangent already retained storage. Retain all three OMM measure buffers
and all in-place compact data/metadata arrays. This also closes the possibility
that inlining discards caller writes. All API calls, data operations, parameters,
allocation/destruction positions and timers remain unchanged. External Rust
strict Clippy and C++ syntax checks pass before application.

The no-restart decoder selects adjacent three-element windows once, with
winding parity equal to the original input index (window index plus two).
The tight path charges its first two input visits separately, preserving
first/second exhaustion and every subsequent emission prefix. Restarted strips
retain the original decoder. Fourteen standard/libm phase contracts pass in
both modes before application; caller tails and every tight-fuel prefix remain
covered. The small append/full large chunk output split is unchanged.

Fetch profiling puts 91% of Rust samples in its general tag-range loop, with
whole-process instruction ratio 1.443. For validated vertex widths <=64 and
already proven fuel/u32 bounds, an index touches exactly one or two lines.
Specialize that common path: visit the first line directly, conditionally
visit the second, and charge exactly 2*index_count plus second-line count.
The earlier checked upper bound 6*index_count proves safe arithmetic/fuel.
Wider vertices and tight/overflow-checking cases keep their exact old path.
23,220 standard and 23,220 libm width/fuel/reuse comparisons each pass against
D112 production before application. No new allocation, unsafe or explicit SIMD.

Fresh full feature-mode checks and native/libm/WASM identity precede twelve
five-pair decoder/fetch diagnostic rows per consumer. Sinks touch other ordinary
APIs, so pending family observations and final evidence must use this method;
all earlier diagnostics remain separately source-bound and are never pooled.
The owner bar, lease/scoreboard gate, fifteen-second polling, ten-minute bursts
and D100/D146 sampling/maximum decisions remain unchanged.

D113 complete validation passes 79/74/79 feature-mode tests, strict root Clippy
in all three modes, both consumer Clippy/resident modes, and 30,000 native,
30,000 libm and 30,000 executed WASM cases. HEAD remains 11a2b5a on phase/0.5;
the clean scalar reference remains 4c203430ca565cb59a468a91922c76c208169536.
No D113 sample is admitted during the initial occupied Moss lease wait.

## D114 — owner schedules timing through the shared lease queue

The owner supersedes the lease-free/scoreboard gate: captures hand the lease
back to back, so gap waiting starves this lane. Stop the zero-sample D113
waiter at its blocked boundary. The complete worktree/source/binary checkpoint
and prior validation remain externally retained; no measurements are discarded.

Every benchmark/profile command now enters the shared gpu-lease.sh queue with
MOSS_GPU_LEASE_TIMEOUT=3600 and a meshopt-timing:p05-* label. Admission verifies
the wrapper's lease environment, matching held label/PID and ancestor chain.
The scoreboard check is removed because the shared lease serializes its work.
Unknown/wrong/uncovered ownership fails immediately; it never times outside
its own lease. Pair receipts retain the holder, ancestry, load and CPU counters.

lease_run.py wraps the granted command with an independent 840-second timeout
and five-second kill grace, below the owner's fifteen-minute burst limit.
Benchmark commands end after ten minutes at a completed-case boundary (76),
retain source/binary-bound completed rows, release, and requeue. Profile commands
likewise retain completed family records and requeue at the next family boundary.
Exit 75 retries queue admission; other errors, including a hard timeout, stop.
Inherited lease markers are cleared outside the queue to force a fresh grant.
Source/reference changes between grants abort rather than pooling revisions.
Stage-one 5/10/20 and fresh D146 decisions, paired API timers, caller allocation,
output retention, production Rust/C++, memory and owner bars do not change.

Python syntax, twelve receipt checks (one positive, eleven negative), eighteen
final/D146 protocol checks and queued-runner 75/76 retry/hard-cap checks pass.
These are synthetic protocol checks, not timing evidence. All Rust/C++/Cargo
source hashes match D113's strict/test validation. Native/libm/executed-WASM
identity receipts are refreshed before the queued focused measurements.

The nested 840-second timeout uses foreground mode, preserving the shared
wrapper's process group. A synthetic supervised command and sleep descendant
share that group and terminate under group TERM; the retained receipt explicitly
contains no API timing. Fresh D114 native/libm/executed-WASM sweeps each match
30,000 cases after this process-control refinement. No Rust/C++/Cargo source
changes or production test repetition is needed. Submit the focused diagnostics
as meshopt-timing:p05-{moss,default}-focus, with separate leased worst-case profiles.

## D115 — inline the checked decoder into its already inlined owned wrapper

The D114 shared queue admits both profiles and profiles: Moss focus completes
in 39 seconds. Preserve all twelve rows/binaries/source/perf evidence per profile
in baseline-D114-focused before source mutation. Complete fetch mean/max passes:
Moss 0.932/1.081, default 0.929/1.105, memory 1.0. The two decoder edge cases pass
for default (owned maximum 1.184, caller 1.195). Moss caller maximum is 1.210,
but tiny owned is 2.145, with million owned 0.986; it is a clear failure and no
residual is accepted. The subset decoder mean is not a complete family mean.

Fresh tiny Moss profile has instruction ratio 1.088 but cycle ratio 1.692;
68% of Rust cycles are in the inlined main driver. Both binaries still call an
out-of-line unstripify_checked<Vec<u32>> despite the public owned wrapper being
always inline. Its mutable workspace/output boundary prevents consumer folding
of known fresh workspace metadata and quota state. Change only that existing
private checked helper from inline to inline(always). All checks, allocation,
work/error order and algorithms remain identical; restarted and plain decoder
paths are both retained. No harness or other production family changes.

Full strict/test/native/libm/executed-WASM proof precedes five pairs for decoder
shapes 0/7 only (four rows per consumer), submitted through D114 queued bursts.
The prior fetch evidence remains source-bound to D114, never pooled into the
new source's final matrix. The final matrix still has not started.

## D116 — expose the emitting decoder loop instead of its mutable Vec boundary

D115's checked helper is now absent from the Moss symbol table, but the next
emitting unstripify_loop<false,Vec<u32>> remains out of line. Tiny owned Moss
still clearly fails at 2.229 (41.67ns versus 18.93ns); default maximum passes
at 1.090. Both caller edge maxima and million owned pass; memory is 1.0.
Retain all four-row streams, source/binaries and fresh failing profiles in
baseline-D115-focused before mutation. No partial decoder mean is promoted.

The emitting call can mutate Vec pointer/capacity/length through its append
interface, leaving the consumer unable to scalarize ownership metadata across
that boundary. Also mark unstripify_loop and its plain adjacent-window helper
inline(always). This is solely a code-generation hint; all data, bounds, work,
quota/error order and append/chunk behavior are unchanged. Full proof precedes
four-row decoder-edge diagnostics under both profiles through queued leases.

## D117 — matched standalone timer functions in the two drivers

D116 completes both four-row profiles and fresh failing profiles. Preserve all
records/binaries/sources in baseline-D116-focused before mutation. Moss tiny
owned remains above the bar (maximum 1.555; 42.39ns versus 28.53ns medians),
while default maximum 1.226, all caller maxima and million owned pass. Neither
the partial mean nor borderline diagnostic maximum establishes acceptance.

Inlining the emitting loop removes its symbols but does not remove the Moss/
default owned gap. Inspect generated timer/dispatcher boundaries: most timed
loops merge into the benchmark/command machinery; only one monomorph remains
separate. Test matched non-inlined timed_iterations functions in Rust and C++.
Each statically selected closure still invokes its API inside the same timed
loop; allocations, fresh workspace creation/destruction, requested memory and
pointer/count sinks stay timed. Call into the timer is outside its clock in
both languages. This isolates register allocation from the large dispatcher,
without changing API work, parameters, output or allocation sizes. Tiny helper
loops are untouched. All production Rust is unchanged from D116.

Strict/full proof precedes four-row decoder-edge diagnostics per profile. The
new method affects ordinary timer loops, so remaining touched-family diagnostics
and the eventual single final matrix must use it. Prior streams stay separately
source-bound and are never pooled. Queued ownership and all bars remain unchanged.

## D118 — fixed initialized chunks for small owned decoder outputs too

Archive D117 both four-row streams and fresh failing profiles in
baseline-D117-focused. Moss owned tiny still fails at 2.056 (41.41ns/20.15ns),
million owned passes 0.999. Caller edge maxima pass below 0.906; default owned/
caller maxima pass 1.007/1.034. Matched standalone timers retain allocation/
workspace work and expose the remaining owned-only gap. No residual is accepted.

Remove D112's <=64 Vec-append branch. All owned sizes now use the existing full
fallible bound reservation, initialized storage and fixed three-element chunk
emitter, as large outputs already did. C++ initializes that same bound. The
mutable decoder borrows the chunk slice instead of the Vec descriptor, so it
cannot modify allocation pointer/capacity/length; truncation happens once after
its count returns. This removes per-triangle reserve/length bookkeeping and
keeps ownership metadata local to the caller. Allocation size/error ordering,
quota, counted work, restart/plain algorithms, returned values and caller tails
are unchanged. Initialization stays timed; no unsafe or explicit SIMD is used.
Full proof precedes the same four-row five-pair decoder edge diagnostics per
consumer. Other families and the D117 matched/queued method are untouched.

## D119 — select owned/caller API before each statically typed timer loop

D118 initialized chunks reduce tiny Moss owned to 31.83ns; its maximum 1.551
still misses the unchanged bar. Preserve complete streams/source/binaries and
profiles in baseline-D118-focused. An external LD_PRELOAD allocation counter
under a separate shared queued lease subtracts zero-iteration setup from
10,000 owned calls: both Rust profiles and C++ perform exactly 10,000 mallocs
of 108 bytes and 10,000 frees, with zero calloc/realloc delta. Command-line
length differs by four bytes in one setup allocation. Instrumented times are
excluded; no extra allocation is inferred and no timing artifact is promoted.

Ordinary timer closures still selected caller versus owned inside every loop.
Move that mode choice before the timer in both drivers for the six dual-mode
families. Every closure now contains one statically selected API with the same
arguments, sinks, workspace/allocation/destruction, per-call memory and timing.
Existing per-iteration scalar argument setup stays inside both closures. This
removes repeated mode dispatch and the mixed caller-buffer/owned allocation
metadata boundary, letting the two compilers optimize those paths independently.
Family dispatch, standalone timer functions, calibration, owned/caller storage,
lease queue, statistical rules and production Rust are unchanged from D118.
Rust consumer fmt and C++ syntax pass before full strict/native/libm/WASM proof;
only decoder shapes 0/7 receive the next five-pair diagnostic probe.

## D120 — visible shared admission replaces direct lease submission

The owner supersedes D114 scheduling: every next timed burst enters
MOSS_HEAVY_GPU=1 moss-heavy.sh 4 timeout 840, with its meshopt-timing:p05-*
dashboard lane. Four GB is the declared peak, adjustable explicitly if needed.
The shared wrapper reserves memory, queues visibly, and obtains/releases the
GPU lease itself. No direct or nested gpu-lease.sh call remains in this lane.
Timing receipts verify the admitted timeout, reserved memory, heavy ancestor,
matching dashboard lane, own lease ancestor and the kernel flock in holder
fdinfo/200 for the shared lock inode. This rejects advisory stale metadata.

The foreground 840-second timeout plus five-second kill grace retains the
shared descendant cleanup. Ten-minute complete-case/family checkpoints release
and requeue, 75 retries admission, 76 resumes a completed checkpointed burst.
Source/reference identities remain immutable between bursts. No scoreboard or
lease-free gap polling returns. Statistical rules, corpus and bars are unchanged.
D119's complete focused streams and profiles were archived before this change;
Moss tiny owned still fails at 1.550 (28.06ns/17.60ns medians), default maximum
passes 1.102, and both caller and million-owned probes pass. No partial mean
qualifies the decoder and no residual is accepted.

## D121 — retain decoded input slice metadata outside both decoder timers

D119 selects the API before timing but Rust still unwraps the optional fixture
Vec on every decoder iteration. Borrow its initialized slice once before the
mode choice; C++ likewise caches its prepared strip data pointer before timing.
Both still pass exactly the same strip length/data/restart value to every call.
No API work, allocation, initialization, workspace/error checks or result sinks
move outside the timer. Production Rust stays at D118. Full strict/native/libm/
executed-WASM proof precedes only the four decoder edge rows per profile under
D120 visible admission. This probes the remaining tiny Moss gap without pooling
prior samples or altering the bar.

## D122 — keep Vec ownership local to fallible output reservation

D121's first visible admission completed all four Moss decoder-edge rows with
five pairs each, a 26-second lease and measured 0.5 GB process peak against the
honest 4 GB declaration. Its own heavy/lease ancestor and kernel-fd receipts
pass. Tiny owned remains a clear diagnostic failure at 1.561; caller maximum
1.111 and million owned 1.050 pass. The two-case owned mean 1.280 is incomplete,
not a full family verdict. Archive source, binary, stream, admission log and
assembly in baseline-D121-partial. Default was not measured at D121; its older
D119 complete source-bound streams remain separate. Stop only the queued,
unadmitted profile/chainer; no live timed burst is interrupted. Current failing
profiles have not been recollected at D121, so do not claim that they have.

D119's retained worst-case profile places 46.9% of Rust cycles in the decoder
timer and 37.4% in free. Current D121 assembly still has three overlapping
metadata accesses around a private Result<Vec> allocation boundary. Change that
helper to reserve into a borrowed local Vec and return Result<()>; owned strip
and decoder wrappers construct the same empty Vec locally. The same checked
bound, exact fallible reservation, allocation error mapping, quota/error/work
order, initialization, outputs and requested capacity remain. This removes an
ownership transfer through the private result representation without moving
any allocation or API work outside timing. No public API, unsafe or SIMD change.

An external same-driver prototype passes fourteen existing phase contracts in
both std and libm and strict Clippy. Its Moss assembly eliminates those three
overlapping accesses (474 to 457 static assembly lines); this is code-generation
evidence, not measured speedup. Full root/consumer/native/libm/executed-WASM
proof precedes eight diagnostic rows per consumer: only strip/decoder shapes
0/7, five pairs each. Both profiles and their failing profiles share a bounded
D120 heavy admission, checkpointing independently and requeueing at complete
case/family boundaries. The 1.25/1.50 bars and final D146 rule remain unchanged.

D122 completes eight five-pair strip/decoder rows per consumer in one visible
78-second admission, peak 0.5 GB, exit 0 for the combined diagnostic/profile
worker. All eight API-subset groups pass time/memory; no failing-family profile
is required for these subsets. Preserve both matching sources/binaries and
empty completed profile summaries in baseline-D122-focused. Owned decoder
maxima are 0.999 Moss / 1.033 default; caller 1.118 / 1.152. Strip owned maxima
1.407 / 1.459, caller 1.149 / 1.144. Requested-memory ratios are all 1.0.
These are edge-subset observations, never full family qualification or a
matched-core causal speedup against old streams. The next five-pair diagnostics
cover only pending cache/OMM measure/normal/tangent families (48 rows per profile),
then other touched families. The final matrix still has not started.

## D123 — specialize merge charging and bounded normal probes; inline cache slices

D122 pending diagnostics complete all 48 rows per consumer with five pairs in
one visible 217-second admission, peak 0.5 GB. Archive exact source, binaries,
streams and fresh failing profiles in baseline-D122-pending. OMM measure passes
(Moss/default means 1.176/1.154, maxima 1.381/1.302, memory 1.156). Tangent owned
means still fail 1.263/1.269, and default caller fails 1.291. Cache maxima fail
1.541/1.766; Moss normal owned maximum fails 1.535. No failing mean is waived.

Fresh tangent profiles place 47–55% of cycles in merge; normal hashing costs
20.5% and merging 15.0% in its Moss worst case. Specialize the existing merge
bulk-charge choice with const-generic loops: same checked pair count, precharge,
pair order and tight-budget fallback. Normal hashing proves at most buckets
visits per vertex over 128-entry chunks, then charges actual probes locally;
uncovered chunks retain per-probe error/work prefixes. Allocation, initialization,
hash/equality/probe order and floating operations remain unchanged. No new storage.

Inline cache entry/kernel and borrow the validated timestamp prefix ending at
each triangle's maximum index. This proves its three accesses with one safe
slice bound. Disabled warp sizing needs only a nonzero presence flag for warp
counts; enabled sizing retains its exact counter. Validation, work errors and
requested memory stay unchanged. Fresh strict, native/libm and executed WASM
proof precedes only these three touched families: 40 five-pair rows per profile
plus fresh failing profiles, through D120 shared admission. No speedup or final
qualification is inferred before that run; the complete final matrix is pending.

## D124 — reject cache slicing regression; specialize timestamp preparation

Archive both D123 streams and fresh failing profiles in baseline-D123-focus.
Its visible lease lasted 167 seconds, combined worker exit 0. Normals pass all
four groups (Moss/default owned means 1.189/1.178, caller 1.145/1.180; every max
<=1.318, memory 1.0). Tangent owned passes 1.243/1.206 and default caller 1.240;
Moss caller mean still fails 1.275, maximum 1.340 on the million case. Retain the
proven merge/hash changes, with no mean waiver or full qualification claim.

Cache worsens to means 1.364/1.402, maxima 1.737/1.855. Its fresh Moss tiny
instruction ratio is 2.066 versus D122 1.933; preparation/reservation functions
remain prominent. Restore the exact D122 cache kernel/entry, rejecting all
D123 cache slicing, inlining and disabled-warp counter changes. Factor the
existing Workspace preparation body into an always-inlined private helper.
The generic entry keeps its identical behavior; a timestamp-only private entry
passes fixed zero lengths for the other buffers, exposing those zero-size
reservations. Preflight capacity accounting, reservation/truncation order,
actual-capacity accounting, quota clear-on-error and initialization remain exact.
No allocation or workspace work moves outside the API timer.

Fresh D123 million tangent profiling places 11.9% of cycles in remap hashing.
Apply normal hashing's same checked 128-entry bound/actual-probe charging there.
The tight-fuel fallback retains every per-probe failure prefix; hash, equality,
probe order, float operations, allocations and outputs remain unchanged. No
speculative sign/math reordering is included. Strict/native/libm/executed-WASM
proof precedes only cache/tangent diagnostics (24 five-pair rows per consumer),
with fresh failing-family profiles in one bounded visible shared admission.
Final acceptance and all bars remain unchanged.

## D125 — match complete statistic retention in analyzer timers

D124 completes both 24-row streams and profiles in a 117-second visible
admission, archived with exact source/binaries in baseline-D124-focus. All
tangent groups pass: Moss owned/caller means 1.248/1.236, default 1.228/1.231,
maxima <=1.351 and memory 1.0. Cache means pass 1.105/1.116; Moss maximum 1.450
passes, default tiny remains 1.613. Its fresh whole-process instruction ratio
is 1.992, dominated by analyzer, kernel and fallible allocation. No residual
or full family qualification is inferred from five-pair diagnostics.

Driver inspection identifies asymmetric statistic sinks for cache, fetch,
overdraw and coverage: Rust makes the complete returned struct opaque by value,
while C++ retains a single scalar using a volatile read/modify/write. Both now
retain a pointer to the complete local struct with their existing opaque
compiler barrier. Return production, result storage, all API work, allocation,
workspace creation/destruction and parameters remain timed; no result field
is omitted from the sink. Complete untimed result-byte comparison still runs
before any timings. No production Rust, corpus, calibration or bar changes.

Consumer formatting and C++ syntax pass. Full strict/native/libm/executed-WASM
proof precedes only those four affected analyzers (32 five-pair rows per profile)
and fresh failing-family profiles via D120 admission. Previously passed tangent,
normal and OMM diagnostics stay separately source-bound; final qualification
still requires fresh complete matrices under this matched method.

## D126 — expose fallible cache entry to caller optimization

D125 completes 32 rows per consumer plus fresh failing cache profiles in an
89-second shared admission; archive exact sources/binaries in baseline-D125-focus.
Fetch, overdraw and coverage pass both profiles. Cache means pass 1.147/1.149;
Moss tiny maximum 1.492 passes diagnostically, default tiny still fails 1.591.
Matched complete-statistic pointer retention remains; it does not prove a
causal speedup or excuse the remaining edge. No residual is accepted.

Replace the cache's anonymous result closure with a private always-inlined
helper containing the identical body. Inline the public wrapper and timestamp
preparation; expose the tiny workspace begin/finish methods with ordinary
inline hints. This lets fresh-workspace callers propagate their known empty
buffers/default limits through the fallible entry, especially without LTO.
The eight cache kernels retain ordinary out-of-line boundaries: D123's
regressing kernel inlining/slicing remains rejected. All parameter/index/size
checks, reservation/accounting, work dispatch, result math and begin/finish
order stay exact; explicit-limit and retained-workspace calls remain supported.
No heap, API, unsafe, SIMD or timing-boundary changes.

Strict/native/libm/executed-WASM proof precedes five-pair diagnostics for cache
and the two still-pending D105 raster/remesh families (36 rows per consumer),
plus fresh failing-family profiles through shared admission. Untouched passed
families are not remeasured during this iteration. One complete final matrix
per consumer remains pending after focused acceptance; bars/D146 are unchanged.

## D127 — specialize raster states/floor and remesh work dispatch

Archive D126 both 36-row streams and failing profiles in baseline-D126-focus.
The 86-second shared admission completes exact matching evidence. Cache now
passes both profiles (means 1.093/1.151, maxima 1.346/1.499, memory 1.0), with
no accepted residual. Raster owned/caller means fail 1.366/1.439 Moss and
1.423/1.507 default; default remesh means fail 1.296/1.259. Memory is 1.0.
Fresh default instruction ratios are raster 1.338 and remesh 1.485. Raster
cycles split 44.1% recursion, 33.7% setup, 10.6% floorf; remesh spends 98.1%
in its two voxelization modes. Means must pass; neither gap is an edge waiver.

Promote the previously prepared, independently checked raster-state and remesh
charging candidates. Raster dispatches once on its already validated 2/4 state
format and propagates it as a const generic through recursion/emission. Recursion,
precharged visit count, tight-budget fallback, geometry/float order and bytes
remain unchanged. Replace sample's finite floor with scalar bit rounding derived
from libm's MIT-licensed generic floor (musl). Values >=2^23 are already integral;
smaller finite magnitudes mask fractional bits, rounding negatives downward.
Preserve signed zero and the existing feature-specific non-finite handler. This
removes the observed host floorf call without unsafe, hand SIMD or fast math.
An external source-extracted floor proof checks 33,426,938 exponent/mantissa/
sign/boundary/non-finite patterns against std floor with zero mismatches; inputs
are opaque to avoid folding. It is semantic evidence, not a timing measurement.

Remesh proves a checked whole-input upper bound on triangle/sample visits and
uses a const charging kernel, counting actual visits once when covered. The
bound uses validated resolution and clamped sample limits. Overflow/tight fuel
retains every original per-triangle/sample error and usage prefix. Allocation,
voxels, accumulation order, output/tails and quotas remain unchanged. Full strict
native/libm/executed-WASM proof precedes only these two families: 28 five-pair
rows per consumer plus fresh failing profiles via shared admission. Final
matrices remain pending; all bars and D146 rules are unchanged.


## D128 — queued five-family prototype and strict artifact-side proof

D127 completes 28 five-pair rows per profile in a 97-second visible admission;
archive exact sources/binaries/streams/fresh profiles in baseline-D127-focus.
Raster Moss owned mean 1.245 passes, but caller maximum 1.596 fails. Default
raster means 1.302/1.361 fail. Remesh regresses to Moss means 1.467/1.358,
default 1.483/1.304, and maxima up to 2.126. Reject its whole-input charging
specialization; neither lower instruction counts nor parity excuses slower time.

Prepare D128 in an artifact-side source copy while D127 waits, using a uniquely
named consumer in the required shared Cargo target. Root timing sources remain
frozen. Inline named raster owned/caller bodies and setup, revert remesh charging
to the original per-triangle bound and specialize validated packed position reads,
flatten the covered no-warp/no-group cache loop, inline the owned strip wrapper,
and cache immutable tangent neighbor endpoints on a 256-byte stack array for
covered groups of at most 32 corners. Pair/union order, outputs, error/work
prefixes, quotas, heap requests and floating operations remain exact; tight
budgets and larger tangent groups retain the existing loops. No unsafe/SIMD/API
or timing-boundary change. Native/libm/executed-WASM each match 30,000 cases;
format, strict Clippy, required tests and fourteen phase contracts all pass.

The immediately following visible admission completes 50 five-pair rows per
profile and fresh profiles; archive in D128-proof/baseline-D128-focused. Moss
all nine subset/API groups pass (raster means 1.125/1.140, maxima 1.234/1.246;
remesh owned mean 1.248 remains close to the bar). Tangents pass both profiles:
Moss means 1.165/1.169, default 1.172/1.195, maxima <=1.339, memory 1.0.
Default owned strip subset mean/max 1.309/1.626, cache subset mean 1.336,
raster means 1.340/1.365, and owned remesh mean 1.264 still fail. Every heap
ratio is 1.0. Subset means do not qualify full families; no residual is accepted.

## D129 — expose small helpers and separate per-triangle remesh sample charging

Promote the exact D128 prototype production changes after its admission ends
and its matching evidence is archived. Its default fresh profiles put 59.8%
of raster cycles in edge, 14.3% of strip cycles in key, and cache reservation/
accounting calls prominently in the tiny path. Inline private raster edge,
strip first/next/key, and workspace reserve/total helpers so the default consumer
can fold known zero reservations and small values. Bodies, reservation order,
capacity/quota accounting and validation remain unchanged. Shared helper
semantics are checked in every required root feature mode.

Factor remesh's original sample loop into const charging kernels selected AFTER
the original per-triangle visits check/precharge. A stack record passes the
already computed origin, edges, normal, sample count, reciprocal and weight.
Covered triangles have no inner charging branch; tight triangles retain each
original sample visit/error prefix. No global work bound, extra heap, new
floating operation, geometry order or initialization change is introduced.

Fresh full root/consumer strict/native/libm/executed-WASM proof gates the next
visible bounded admission: only cache/strip edge controls and complete raster/
remesh families, 34 five-pair rows per profile plus fresh failing profiles.
The worker releases/requeues if proof is unfinished. No timing result or final
qualification is inferred from preparation; all bars/D146 remain unchanged.


D129 completes both streams and fresh profiles, archived in baseline-D129-focus.
Moss cache/strip/raster pass their diagnostic groups; default cache subset also
passes (mean/max 1.207/1.219). Default strip subset mean 1.303 and raster means
1.337/1.397 still fail. Extracted remesh sample kernels regress to means
1.538/1.449 Moss, 1.562/1.445 default; reject that extraction. Default raster's
fresh profile now puts 64.2% in the array-map closure despite edge inlining.
Default strip's million profile puts 85.6% in its owned Vec-emitting core.

## D130 — reject sample-row charging dispatch

An independently proved artifact-side alternative restores D128 remesh and
selects covered/tight sample loops once per row inside each triangle. Native,
libm and executed WASM each match 30,000 cases; all strict checks/contracts pass.
Sixteen five-pair rows per profile plus fresh profiles complete in a visible
72-second admission (D130-proof/baseline-D130-remesh). Both profiles regress:
owned/caller means 1.514/1.392 Moss, 1.510/1.407 default; maxima up to 2.170.
Reject it too. D128's unchanged per-triangle loop remains the retained starting
point; branch removal and lower instruction counts cannot substitute for time.

## D131 — direct raster edges and checked owned strip emission

Prepare another frozen artifact-side prototype while admissions wait. Replace
raster's nine-edge array map with nine explicit calls in identical edge order,
eliminating the profiled array-drain closure without changing any sample or
floating operation. Owned strip uses the existing SliceOutput over its exactly
reserved, initialized allocation, then truncates to the same emitted count.
C++ already initializes that owned bound; initialization remains inside the API
timer in both drivers. Owned output/scratch/usage stay charged identically,
allocation/error/work order and bytes remain exact. No extra allocation.

All strict root/consumer checks/contracts and 30,000-case native/libm/executed-
WASM proofs pass before the queued sixteen-row/profile raster/strip burst.
Only strip shapes 0/7 and complete raster are measured, five pairs plus fresh
failing profiles. Root and prototype timing sources remain frozen while queued.

## D132 — scalar voxel coordinate clamp without signed float conversion

D128 remesh's original per-triangle work loop remains intact in this artifact-
side candidate. In its grid-marking pass only, extract the integer part from
float bits before shifting/clamping to the validated cutoff <=253. Magnitudes
below one and NaNs map to zero; negative integral/large/infinite coordinates
select cutoff; other positive coordinates shift the significand and clamp.
This is exactly the original saturating i32 cast, signed right shift and
unsigned cutoff test. The accumulating pass keeps original casts/octant bits.
All geometry/float operation order, work prefixes, heap storage and voxel order
remain. No unsafe, SIMD or new API.

A source-extracted scalar proof compares 134,615,444 exponent/sign/mantissa and
all-cutoff integer/ULP-boundary cases against the original expression with zero
mismatches. This is semantic evidence, never timing. Full strict/native/libm/
executed-WASM proof gates a separately queued sixteen-row/profile remesh burst.
No source promotion, speedup or qualification is inferred before measurement;
the complete final matrix remains pending and no residual has been accepted.


D131 completes in a 57-second visible admission with no failing groups;
archive D131-proof/baseline-D131-raster-strip. Moss owned/caller strip subset
means 1.081/1.105, default 1.176/0.997, maxima <=1.257. Raster complete-family
means 1.097/1.159 Moss and 1.105/1.127 default, maxima <=1.302. Memory is 1.0.
This clears the prior Moss caller maximum and default raster means; full final
qualification is still required. D132 completes in 78 seconds with fresh
profiles (D132-proof/baseline-D132-coordinates), but regresses remesh means to
1.588/1.485 Moss, 1.682/1.524 default. Reject scalar bit coordinate conversion;
retain its exactness proof as rejected-candidate evidence, not a speedup.

## D133 — separate packed remesh kernels and specialize solve mode

Another artifact-side remesh candidate restores D128's original per-triangle
loop/casts. Dispatch once on the already validated solve flag, propagate it as
a const generic, and inline quadric accumulation so unused solve fields can be
removed. Give the packed/strided voxelization kernels an explicit out-of-line
boundary; D128's private kernels were merged into the dispatch wrappers.
Input validation, float/order/coordinates, options, voxel accumulation, heap,
quota/accounting and tight-work prefixes remain unchanged. No unsafe, SIMD or
API change. Full strict/contracts/native/libm/executed-WASM proof gates only
sixteen five-pair remesh rows per profile plus fresh failing profiles through
the visible queue. This is prepared code, not acceptance; no residual is waived.


## D134 — expose remesh entry and owned-bound specialization

Prepare/prove a separate artifact-side alternative while D133 waits at the
shared wrapper's disk floor. Restore D128 remesh exactly and inline its private
run/position dispatch and public owned/bound/caller wrappers. This exposes the
None/Some destination and fresh-workspace limit values to caller optimization,
especially in Cargo defaults. Bodies, work/errors, requested heap, float/voxel
order and output initialization remain identical. Strict feature-mode checks,
contracts and 30,000 native/libm/executed-WASM cases all pass. Expanded native/
libm checks match all eight remesh shape controls, including the million case.

Queue sixteen five-pair rows per profile immediately behind D133; the external
worker releases/requeues while its predecessor is unfinished and skips timing
if D133 already clears every diagnostic group. No redundant measurement is
performed in that case. Root sources remain frozen; no lease/disk gate bypass.
Remove only this lane's completed debug incremental compiler cache to reduce
disk pressure, retaining every source/binary/parity/profiling receipt. Future
prototype debug proof disables incremental caching in the same required target.

## D135 — memoize each remesh sample row's rounded base point

Prepare a further artifact-side fallback from D128 while the two admissions
wait. Compute u*reciprocal and each rounded origin+u*edge once per sample row,
then add the original v*edge inside the inner loop. Each output coordinate has
the same multiplication/addition grouping and rounding; no reassociation,
recurrence or fused operation is introduced. Original per-triangle charging
and every tight sample-work/error prefix remain. No new heap, API or SIMD.
Full strict/native/libm/executed-WASM proof precedes a staged sixteen-row/profile
remesh burst, run only if both earlier candidates still miss the bar. The final
matrix launcher is prepared but has not started; acceptance/residual rules stand.


D135 passes full strict/contracts/native/libm/executed-WASM proof and expanded
eight-shape remesh checks in both modes. Its actual default-profile grid and
accumulation kernels have respectively 1385/1547 instructions, exactly matching
D128's opcodes/registers/branches and referenced float constants after binding
ELF constants; only relocation/error metadata differs. The compiler already
performs the proposed row-base reuse. Reject this default-mean follow-on before
measurement and cancel only its identified queued heavy-wrapper PID. No timed
burst was interrupted; preserve static/source/binary evidence in D135-*.
D133/D134 remain queued with independent tested changes; bars remain fixed.


D133 finishes both sixteen-row/profile streams and fresh profiles in 131
seconds (D133-proof/baseline-D133-solve). Owned means 1.277/1.303 fail, default
caller 1.254 fails; retain its narrower kernel instruction profile as evidence,
not acceptance. D134 finishes in 221 seconds (D134-proof/baseline-D134-entry),
but blanket run/entry inlining regresses default means to 1.446/1.434. Moss
owned mean 1.285 and both maxima around 1.53 fail. Reject that inlining too.

## D136 — reuse successful owned-bound validation with exact work charges

The owned remesher already validates these same immutable index/position views,
resolution and flags inside its successful bound call. A private const flag
lets only its second run reuse that proof, charging the identical index and
position visits. Bound/caller entry points still perform every original check.
A successful bound proves these fresh validation charges fit the unchanged
work limit; later work, allocation/quota errors and prefixes remain exact.
No geometry or float calculation, allocation order, heap, API or SIMD change.
This artifact-side candidate also retains D134's entry hints for comparison.
Full strict/contracts/native/libm/executed-WASM proof passes before the queued
sixteen-row/profile burst. No speedup or final acceptance is inferred.

## D137 — equivalent scalar saturating coordinate conversion

Prepare a further artifact-side remesh fallback from D128 while D136 waits.
Convert the bit-cleared nonnegative magnitude to u32 with Rust's saturating
truncation, then clamp to the signed endpoint and restore the sign with
wrapping integer negation. This exactly retains i32 saturation, signed zero,
NaN-to-zero, infinities and all finite coordinate/octant bits. Geometry float
operations, original per-triangle work loop, errors and heap remain unchanged.
A source-extracted proof matches 33,558,016 exponent/sign/mantissa/boundary cases
against the original signed cast with zero mismatches. Full strict/native/libm/
executed-WASM proof gates a staged sixteen-row/profile remesh fallback. No unsafe,
SIMD, approximation or public API; no timing claim before admission.

## D138 — validation reuse with original remesh entry boundaries

D134's unconditional entry expansion is rejected. Prepare another artifact-
side candidate from D136 that keeps its proved owned-validation reuse but
restores D128's public/private entry and voxelization dispatch boundaries.
This isolates validation memoization from regressing code expansion. Float,
heap, work/error prefix and input/output semantics remain unchanged. Full
strict/native/libm/executed-WASM proof precedes staged remesh-only diagnostics,
conditionally skipped if an earlier candidate clears both profiles. Final
full matrices still have not started; no mean or maximum residual is waived.


D136 completes both sixteen-row/profile streams and fresh profiles in 242
seconds (D136-proof/baseline-D136-validated). Moss means 1.277/1.282 and default
1.395/1.437 fail. D137 completes in 190 seconds (D137-proof/baseline-D137-cast),
but equivalent unsigned conversion also regresses: Moss means 1.479/1.432,
default 1.712/1.617. Reject both measured candidates; semantic equivalence and
changed instruction selection cannot establish speed. D138 validation-only
with original entry boundaries remains queued, strict proof complete.

## D139 — bound only the grid-marking cell conversion

Prepare another artifact-side remesh fallback from D128 while D138 waits.
Grid marking needs the clamped cell, not the full signed coordinate/octant bits.
Clamp its conversion input to finite [0,506] with max/min, mapping NaNs to zero;
return cutoff for original values <=-1 and otherwise shift/clamp the converted
cell. The validated cutoff is at most253, so large positive values still select
the identical cutoff. Sub-unit negatives, signed zeros, NaNs and infinities
retain the original cast/shift/unsigned-clamp result. Accumulation keeps its
original coordinate casts and octant bits. Original sample geometry/float
expression grouping, per-triangle work, errors, allocation and heap stay exact.

A source-extracted proof matches134,615,444 exponent/sign/mantissa/all-cutoff/
integer-ULP boundary cases against the old expression in BOTH native and executed
WASM with zero mismatches. These are semantic proofs, never speed measurements.
Full strict/contracts/native/libm/executed-WASM proof gates a staged remesh-only
sixteen-row/profile burst, skipped if D138 already clears both profiles. Actual
Moss/default configurations are prebuilt without timing. No unsafe, explicit
SIMD, public API, corpus or bar change. Final matrices have not started.


D138 completes in a178-second visible admission with fresh default profiles
(D138-proof/baseline-D138-validation-only). Moss owned/caller means1.243/1.196
pass, maxima1.422/1.425 and memory1.0. Default means1.293/1.307 still fail,
maxima<=1.492. Validation reuse alone does not clear the default bar; neither
mean is an edge residual. D139's max/min intentionally yields finite zero for
NaNs, unlike clamp; document the local manual-clamp Clippy exception. Cancel
only the unproved queued D139 wrapper before that annotation, rerun all strict/
identity proof, then requeue its immutable proved source. No timing was collected
on the failed lint revision. Root timing sources remain unchanged.

## D140 — derive the finite grid conversion bound once per voxelization

Prepare an artifact-side D139 fallback using the validated actual cutoff.
Compute its exactly representable twice-cutoff upper bound once, cap its integer
source at253, and clip each grid-only conversion to[0,upper]. The shifted cell
is already <=cutoff, eliminating three repeated integer clamps per sample.
Original negative-cell handling, signed zeros, NaNs/infinities, floating sample
geometry/grouping, work/error prefixes, allocations and output stay exact.
Accumulation keeps its old signed coordinate/octant conversion. Independent
native and executed-WASM proofs each match134,615,444 original-expression cases
with zero mismatches, including every cutoff and integer/ULP boundaries. Full
strict/native/libm/executed-WASM proof gates a remesh-only16-row/profile staged
burst; skip it if D139 clears both profiles. No unsafe, explicit SIMD, public
API, bar or corpus change. Full final matrices remain unrun.


D139 completes both profiles in 192 seconds (D139-proof/baseline-D139-grid).
Moss means1.406/1.256 and default1.388/1.357 fail. D140 finishes in88 seconds:
Moss1.419/1.328, default1.468/1.396, with failing maxima too. Reject both grid
conversion candidates. Exact native/WASM scalar proofs establish semantics,
not speed; no failing mean can become a residual. Heap ratios remain1.0.

## D141 — scalar direct half-cell conversion for grid marking

Prepare an artifact-side D128 fallback while D140 waits. Grid-only coordinates
use `(value * 0.5) as u8`, with the original <=-1 cutoff selection and final
cutoff clamp. Saturation at255 remains above the maximum validated cutoff253;
subnormals round only below the first integer boundary. Accumulating signed
coordinate/octant conversion and all sample geometry stay unchanged. Independent
native and executed-WASM proofs each compare134,615,444 cases with the original
expression, zero mismatches. Full strict/contracts/native/libm/executed-WASM
proof passes. Stage only16 remesh rows per profile, five pairs, visibly admitted;
skip timing if D140 passes. No heap, work, errors, public API, bar or SIMD change.

## D142 — fixed-pitch private stack grid for small-resolution marking

Prepare another artifact-side D128 fallback while D141 measures. Validated
resolutions4..8 have cell coordinates0..5. Mark a512-byte stack grid with pitch8;
coordinate masks preserve each value and prove its fixed-array index<=511.
After successful marking, copy its occupied cube rows into the original heap
grid's interior. Heap allocation/initialization/quota/order and outer border
remain identical. Partial private grid contents on marking failure are dropped
before any public output write; work visits/errors are unchanged. Sample float
operations, original casts and sample order stay intact. Accumulation and all
resolutions>8 use the original dynamic representation. No unsafe, explicit SIMD,
new API or case/seed specialization. Existing pinned resolution16 tests exercise
the fallback. Strict/native/libm/executed-WASM proof gates a conditional16-row/
profile remesh burst. Full final matrices are still unrun; no residual accepted.


D141 finishes in 173 seconds (D141-proof/baseline-D141-cell-byte). Moss means
1.270/1.198, default1.274/1.272; only Moss caller passes. Reject this conversion
candidate: all maxima/memory passing does not clear its failing means.

D142 strict/native/libm/executed-WASM proof and expanded eight-shape std/libm
checks pass. Its admission finishes in 34 seconds with exit1 after 16 complete
Moss rows: means 1.195/1.168, maxima 1.264/1.261, heap 1.0. The external diagnostic
worker expects the old C++ library basename after prototype package renaming.
Actual renamed binary SHA matches the stream; its source/binaries/raw pairs are
archived in D142-proof/baseline-D142-partial. No default timing/perf evidence,
no promotion or performance qualification. Owner stop forbids another burst.

## D143 — fixed-pitch accumulation prototype; abandoned without timing

While D142 was in flight, extend its small-resolution private representation to
accumulation: copy counted grid rows and row offsets into 512-byte/64-entry stack
arrays, retaining heap requests/quotas and original signed cast/octant/float math.
Resolution>8 keeps the original fallback. Root stays immutable during timing.
Full root/consumer strict checks, contracts and30,000-case native/libm/executed-WASM
proof pass. No timing was submitted. At the owner's wrap-up request, abandon this
artifact-side candidate rather than queue another burst. It remains a hypothesis,
not a measured rejection or a retained production optimization.

## D144 — owner stop, validated tree, logical commits and diagnosis handoff

The owner explicitly replaces the continue-to-qualification task with immediate
wrap-up: no new timing; validated code only; logical commits using repo author
configuration and no AI trailers; complete status/rejection record; delete this
lane's exact Cargo target. This authorizes commits despite the original edit-only
spec, and does not authorize rebase. All own timing controllers have ended.

Promote only D131 raster/strip and restore D128 remesh over rejected D129 sample
extraction. Other validated D129 files remain. Remesh default owned mean 1.2641
fails; every family still lacks a current complete final matrix. No residual or
unreachable claim. Fresh combined-tree fmt/strict Clippy,79/74/79 root feature-mode
tests, two consumer resident tests per mode, fourteen phase contracts and native/
libm/executed-WASM 30,000-case identity sweeps pass. Expanded selected-family parity
covers 38 shapes per std/libm mode. `D144-functional-preflight.json` has zero functional
errors and 51 historical benchmark/profile errors; full verify remains exit1.
Do not relabel old final summaries as qualified. Preserve exact sources/proof bins
and pending diagnosis scripts externally before deleting only
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p05perf`.

### Current per-family observations (not current final acceptance)

| Family | Before GM M / D | Latest applicable evidence GM (max), Moss | Default | Max heap ratio | Scope / status |
|---|---:|---:|---:|---:|---|
| `stripify` | 1.346 / 1.266 | 1.106 (1.176) | 1.176 (1.257) | 1.000 | D131 edge subset passes; full pending |
| `stripify_bound` | 2.803 / 3.009 | 0.424 (0.476) | 0.423 (0.478) | 1.000 | D73 historical pass; fresh final pending |
| `unstripify` | 1.149 / 1.154 | 1.010 (1.118) | 0.995 (1.152) | 1.000 | D122 edge subset passes; full pending |
| `unstripify_bound` | 2.416 / 3.287 | 0.638 (0.671) | 0.503 (0.521) | 1.000 | D73 historical pass; fresh final pending |
| `vertex_cache` | 1.817 / 1.857 | 1.057 (1.222) | 1.207 (1.219) | 1.000 | D129 edge subset passes; full pending |
| `vertex_fetch` | 1.432 / 1.309 | 0.932 (1.062) | 0.925 (1.064) | 1.000 | D125 full-family diagnostic passes |
| `overdraw` | 1.166 / 1.770 | 0.935 (1.096) | 1.005 (1.393) | 1.000 | D125 full-family diagnostic passes |
| `coverage` | 1.260 / 1.725 | 0.940 (1.140) | 0.964 (1.205) | 1.000 | D125 full-family diagnostic passes |
| `omm_measure` | 2.545 / 2.152 | 1.176 (1.381) | 1.154 (1.302) | 1.156 | D122 full-family diagnostic passes |
| `omm_rasterize` | 1.434 / 1.973 | 1.159 (1.286) | 1.127 (1.301) | 1.000 | D131 full-family diagnostic passes |
| `omm_entry_size` | 1.877 / 2.674 | 0.526 (0.540) | 0.488 (0.494) | 1.000 | D73 historical pass; fresh final pending |
| `omm_compact` | 1.390 / 1.148 | 1.246 (1.319) | 1.115 (1.181) | 1.000 | D73 historical pass; fresh final pending |
| `tangents` | 2.938 / 2.869 | 1.169 (1.330) | 1.195 (1.339) | 1.000 | D128 full-family diagnostic passes |
| `normals` | 3.530 / 3.214 | 1.189 (1.275) | 1.180 (1.318) | 1.000 | D123 full-family diagnostic passes |
| `remesh` | 1.613 / 1.732 | 1.248 (1.377) | 1.264 (1.348) | 1.000 | D128; default owned mean FAIL |

## Candidate ledger and dead hypotheses at owner stop

Every source candidate is recorded in D71–D143 above. This index distinguishes
retained corrections from failed explanations, performance rejections, a compiler
no-op, and unfinished hypotheses. All artifacts are under
`/mnt/linux-extra/meshopt-artifacts/p05`; completed D128–D141 prototypes have
source/binary-bound `Dnnn-proof/baseline-*` archives. Earlier streams use
`baseline-Dnnn-*`, except frozen D106. D142 uses `D142-proof/baseline-D142-partial`.
No samples across sources/methods are pooled. Lower instruction counts or
semantic equivalence never establish a wall-time pass.

| Candidate / hypothesis | Outcome and disposition |
|---|---|
| D71–D73: batched helper timing, matched strict scalar reference, analyzer/generator loop reductions and adjacent normal pairs | Retained foundations. D73 full matrix still fails12/21 Moss and14/21 default groups. Historical data uses the earlier method. |
| D74–D85: sequential strip reads, cache bounds, scalar raster rows/setup, direct corner/voxel/hash reads, covered caller raster, eight-triangle removal, scalar log2 and inline OMM hash | Retained validated changes; caller witness D83. Individually untimed while gated, subsequently exercised by diagnostics. No individual speedup inferred. |
| D87/D89–D92: covered caller normal/tangent storage, fuel-specialized decoder/raster, once-only scratch initialization, retired remap removal | Retained semantics and requested-heap reductions. D88 adds executed libm proof. Superseded narrow emission choices are listed below. |
| D97/D98/D101/D102/D105: packed reads, disabled cache simulation, bounded OMM ranges, normal clear/charging, tangent accumulation and fetch bounds | Retained validated changes. Interim queues admitted no samples or partial samples; D102/D103 partial streams are preserved separately. |
| D99/D112: append owned strip/unstrip output, including the <=64 small decoder branch | Rejected as the final ownership layout. D118 replaces decoder append with initialized chunks; D131 replaces strip append with SliceOutput. Exact capacity/work/heap retained. |
| D103: branchless decoder degeneracy and checked triangle chunks | Retained, but D105 owned tiny/million maxima1.543/1.851 Moss and1.560/1.791 default still fail. Not sufficient alone. |
| D104: caller page commitment and owned output chunks | Retained matching correction; not a sufficient explanation for the remaining owned-only gap. |
| D106: resident immutable inputs and explicit decoder validation | Retained matching correction; focused default owned maximum1.756 still fails. Frozen D106 source/binaries preserved. |
| D107: matched conditional caller buffers | Retained correction. Discarded its first draft's unrelated untimed OMM parity branch edit. Does not alone clear tiny owned decoding. |
| D108/D115/D116: inline allocation, checked wrapper and emitting loop | Retained hints; successive Moss tiny maxima1.849/2.229/1.555 still fail. Removing opaque symbols alone is insufficient. |
| D110: warmed equal-batch calibration | Retained fix for concrete sub-ms batches. Corrected tiny Moss maximum2.889 still fails, rejecting undersized batches as the whole explanation. |
| D111: static API selection outside both loops | Retained method fix. Tiny owned Moss2.883 still fails. Wider dispatcher asymmetry is insufficient. |
| D112/D113: proven add/shift fetch arithmetic, no-restart decoder and <=64 fetch specialization | Retained semantics. D112 fetch mean1.290 Moss still fails; D114 matched method subsequently clears fetch. |
| D113: retain output pointer/count instead of making Vec ownership opaque | Retained matched correction; D114 tiny owned Moss2.145 still fails. |
| D117: matched standalone timer functions | Retained method correction; tiny owned Moss2.056 still fails. |
| D118: initialize all owned decoder bounds and emit fixed chunks | Retained; tiny Moss maximum1.551 still misses. Separate allocation-count experiment proves one108-byte malloc/free per owned call in both languages; extra allocation hypothesis rejected. Instrumented times excluded. |
| D119/D121: select owned/caller mode and prepare equal slice metadata outside timers | Retained matching corrections; tiny Moss maxima1.550/1.561 still fail. D121 default/profiles were not collected. |
| D122: keep Vec ownership local to fallible reservation | Retained. Decoder edge subsets finally pass both profiles; full family remains unqualified. |
| D123: cache kernel inlining, slicing and timestamp changes | Rejected: cache means1.364/1.402, maxima1.737/1.855. Restore kernel in D124. D123 normal merging/probe changes retained; tangent charging refined in D124. |
| D124: original cache kernel, fixed timestamp preparation, actual tangent probes | Retained; tangents pass, cache default tiny maximum1.613 still fails. |
| D125: make the complete statistics struct pointer opaque in both drivers | Retained matched sink correction. Fetch/overdraw/coverage pass; cache default max1.591 still fails. |
| D126: inline cache entry and fixed timestamp seam, keep kernel out of line | Retained. Full cache diagnostic passes but default max1.499 is borderline; D129 edge subset passes. No final maximum accepted. |
| D127: state-specialized raster and exact scalar floor | Retained; scalar proof33,426,938 cases. Raster default means1.302/1.361 still fail before D131. |
| D128: inline raster setup/entry, packed remesh reader, flat no-warp cache, strip wrapper and bounded tangent endpoint cache | Retain cache/tangent/remesh; strip/raster superseded by D131. Mixed candidate is not accepted: default raster/strip/cache subsets fail and remesh owned mean 1.264 fails. |
| D129: inline small strip/workspace helpers | Retained where still present; strip/raster replaced by D131. Per-triangle remesh extraction separately rejected below. |
| D131: nine direct raster edges in original order and initialized checked owned strip output | Retained in final tree. Raster full diagnostic and strip edge subset pass both profiles, heap 1.0; no complete final matrix. |

| Remesh candidate | Moss owned/caller GM | Default owned/caller GM | Decision |
|---|---:|---:|---|
| D127: whole-input charging | 1.467 / 1.358 | 1.482 / 1.304 | Rejected; at least one mean fails. |
| D129: extracted sample helpers | 1.538 / 1.449 | 1.562 / 1.445 | Rejected; at least one mean fails. |
| D130: covered/tight sample-row split | 1.514 / 1.392 | 1.510 / 1.407 | Rejected; at least one mean fails. |
| D132: bit-decoded coordinate clamp | 1.588 / 1.485 | 1.682 / 1.524 | Rejected; at least one mean fails. |
| D133: packed kernel boundaries / constant solve | 1.277 / 1.184 | 1.303 / 1.254 | Rejected; at least one mean fails. |
| D134: blanket entry/run inlining | 1.285 / 1.242 | 1.446 / 1.434 | Rejected; at least one mean fails. |
| D136: owned validation reuse + entry expansion | 1.277 / 1.282 | 1.395 / 1.437 | Rejected; at least one mean fails. |
| D137: equivalent unsigned saturating cast | 1.479 / 1.432 | 1.712 / 1.617 | Rejected; at least one mean fails. |
| D138: validation reuse only, original boundaries | 1.243 / 1.196 | 1.293 / 1.307 | Rejected; at least one mean fails. |
| D139: finite[0,506] grid conversion clamp | 1.406 / 1.256 | 1.388 / 1.357 | Rejected; at least one mean fails. |
| D140: twice-cutoff finite grid upper bound | 1.419 / 1.328 | 1.468 / 1.396 | Rejected; at least one mean fails. |
| D141: direct saturating half-cell byte conversion | 1.270 / 1.198 | 1.274 / 1.272 | Rejected; at least one mean fails. |
| D135: rounded base point per sample row | — | — | Rejected before timing: generated default marking/accumulation instructions, registers, branches and float constants identical to D128. Compiler already hoists it. |
| D142: fixed-pitch512-byte stack marking grid, resolutions4..8 | 1.195 / 1.168 | unmeasured | Abandoned at owner stop; Moss maxima 1.264/1.261, heap 1.0. Wrapper exits1 after Moss due renamed C++ library basename; no default or perf run. Not a demonstrated performance rejection. |
| D143: extend fixed-pitch stack grid/row map to accumulation | unmeasured | unmeasured | Abandoned without any timing submission; full strict/native/libm/executed-WASM proof passes. No performance inference. |

The operative rejection is failure against the unchanged bar; none of these
means is classified as a residual or as unreachable in safe Rust. D142/D143
remain usable hypotheses for the next diagnosis lane. D139/D140/D141 independent
native and executed-WASM conversion proofs each match134,615,444 cases; D132
native proof matches134,615,444 and D137 native cast proof33,558,016. These proofs
say nothing about speed. Source-extracted float/coordinate proof artifacts and
expanded std/libm corpus receipts remain in the respective proof directories.

Rejected scheduling assumptions are also closed: waiting for a quiet/load-free
host, periodic free-lease gaps, scoreboard polling, and direct lease submission
are obsolete. D114 introduced fair lease scheduling; D120 supersedes it with
visible shared heavy admission. None authorizes a new burst after this owner stop.

## D160 — P05 diagfix Phase 0: integrate rewritten release history

SPEC-p05-diagfix explicitly authorizes its one initial rebase in this worktree;
its later no-rebase rule applies after Phase 0. Rebase the four 0.5 commits from
80663a8 onto origin/release/0.2.0 at 419dc7a, preserving release preprocessing,
meshlets, cluster LOD and parallel APIs together with all 0.5 modules/evidence.
Keep release package/version/features and include both sets of tests. Preserve
Workspace's value-initialized cache reservations and later retained simplifier
accounting, adding the 0.5 timestamp seam and index validation fast path. Keep
both decision histories; label the old 0.5 excluded-branch coverage as historical.
No other branch is changed; the backup remains at 80663a8. Never push or rebase
again in this lane. Refresh the two 0.5 consumer/fuzz lockfiles to version 0.2.0.

All Phase 0 functional gates pass before diagnosis: fmt; strict root Clippy and
149/123/128 tests in all/no-default/no-default+experimental modes; both consumer
Clippy/test modes; full 0.1/0.1.x/0.2/0.3/0.4 fixtures and seeded strict C++/WASM
sweeps (including 50,000 0.1 and 64,000 0.1.x comparisons); upstream JS suites;
80 cluster-LOD cases; all 15 supported RFC113 layouts and 90 codec comparisons
with zero timing pairs. P05 native/libm/executed-WASM each match 30,000 cases.
No corpus/bar changes, unsafe or SIMD. Receipts, exact sources and proof binaries
are under /mnt/linux-extra/meshopt-artifacts/p05-diagfix/phase0; the slim record is
parity/results/p05-diagfix-phase0.json. Historical performance/fuzz records are
not promoted to current evidence. This closes Phase 0, not performance acceptance.

## D161 — P05 all-family instruction diagnosis before batch fixes

Complete Phase 1 without timing. Every 0.5 family and both API forms where
available have fresh instruction/branch counts on frozen shapes 0/2 (remesh
0/4) under both consumers. Retain immutable source/binary/assembly/sample
identities in /mnt/linux-extra/meshopt-artifacts/p05-diagfix/before. Counts
subtract fixture/setup with two batch lengths; timer output is discarded.
P05_DIAGNOSIS.md records all causes, quantified limits, ranked fixes and explicit
abandonment of unsupported further edits. Remesh default medium owned retires
9,646,512 vs 6,461,084 instructions and 1,033,479 vs 190,275 branches; marking/
accumulation dominate. Implement the unfinished D142/D143 representation idea,
redundant raster-clear removal, packed raster readers and small compact-hash
inlining together, each in a separate validated commit. Do not repeat rejected
D127–D141 candidates or infer time acceptance from counts. Add lean touched-family
screening/D146 support before the single Phase 3 campaign; preserve the final
matrix gate and frozen shapes. Abandoned families remain unqualified.

## D162 — Constant-pitch small-grid marking

Implement rank 1 from D161 using D142's previously unfinished representation:
for resolutions 4..8, mark a private initialized 512-byte grid with pitch 8 and
masked coordinates, then copy successful interior rows into the unchanged heap
grid. Keep original resolution>8 fallback, signed casts, sample order, borders,
heap requests/quotas and exact work/error behavior. No public output is written
by failed marking. New assembly eliminates runtime-pitch products/dynamic grid
indexing in that bounded store; original casts remain.

All root feature-mode tests/strict Clippy, fourteen contracts and 30,000 native,
libm and executed-WASM cases each pass (`marking-proof`). Fresh two-profile counts
(`marking-counts`) support retention: medium owned instructions fall 6.40% Moss /
10.24% default, branches 13.52%/13.82%; caller instructions fall 4.45%/7.15%.
Tiny owned/caller instruction changes range -0.23%..-1.16%; branch differences
~0.4% are fixed-cost noise, not a timing claim. Both consumer binaries, sources,
counts and objdump are retained under p05-diagfix. No timing yet.

## D163 — Constant-pitch small-grid accumulation

Extend D162's bounded representation to accumulation with a 512-byte grid and
64-entry row offsets. Copy unchanged counted grid rows/row offsets to stack,
then form masked constant-pitch indices. Keep original voxel index/octant bits,
float operations, all resolution>8 paths, heap requests/quotas and fuel prefixes.
Root tests/strict Clippy in all three modes and 30,000 native/libm/executed-WASM
cases each pass (`accumulation-proof`). Both-profile objdump/counts support the
next step (`accumulation-counts`): medium owned instruction ratios to C++ are
1.292 Moss / 1.276 default, caller 1.248/1.242, versus D162 1.347/1.340 and
1.322/1.327. Tiny ratios remain ~1.47–1.49 owned/~1.43–1.45 caller. These are
instruction diagnostics, not timing qualification. No timing occurred.

## D164 — Skip the first redundant viewport clear; refine reduction diagnosis

Remove only axis-zero Pixel::fill because reserve already initializes all
65,536 pixels to positive zero. Keep clears between views and unchanged work,
heap, scalar raster operations and failure behavior. Assembly now guards the
memset call with axis!=0, removing one 1 MiB clear per call. Retired instruction
counts are neutral (<0.002% change) because bulk memset retirement is not a
measure of written bytes; no timing speedup inferred. All root checks and three
30,000-case exact sweeps pass (`raster-clear-proof`, `raster-clear-counts`).

Further default assembly inspection identifies the missing overdraw vector
reduction: the full-fuel selector remains inside its scalar pixel loop, while
Moss emits packed integer reductions and default coverage already vectorizes.
The baseline tiny overdraw cost is 6,376,421 instructions/1,127,511 branches
(default), 4,595,281/742,413 (Moss), 4,249,103/678,716 (C++). Refine D161's plan
with a covered/tight-fuel pixel-scan specialization before the packed reader.
Keep charge-before-read and partial private statistics/fuel in the fallback;
only integer sums may vectorize. This is distinct from rejected cache expansion
and OMM state specialization. P05_DIAGNOSIS.md records the evidence and ranking.

## D165 — Covered viewport reductions with exact tight-fuel fallback

Factor the pixel reduction into const coverage/charging helpers. Dispatch once
per view after the existing full-scan fuel check/precharge. Covered paths contain
no per-pixel fuel selector; tight paths charge before each pixel and retain
original private statistics order/prefixes. Only integer reductions vectorize;
all raster float order, heap and total work remain unchanged. A regression with
literal counter-prefix expectations covers limits 0..5 in both analysis modes.

Root tests/strict Clippy in all modes and three exact 30,000-case sweeps pass
(`raster-scan-proof`). Fresh counts/asm (`raster-scan-counts`) confirm the intended
default overdraw change: tiny instructions -25.00%, branches -32.69%; medium
instructions -3.96%, branches -4.80%. Ratios to C++ improve 1.501->1.125 tiny and
1.072->1.030 medium. Moss is instruction-neutral; coverage changes are at most
+0.13% in these counts. No timing inference or changed bar. Retain the fix.

## D166 — Select a checked packed viewport reader once

Dispatch transform on the release's packed_values helper, which refuses a
logical mapped view; use PositionReader for direct packed reads and the existing
checked generic layout. Preserve scalar min/max/transform order, finite checks,
all visits, allocations and typed public errors. Regression coverage adds
padded floats and unaligned LE/BE bytes with ignored NaN padding, exact values
and fuel/error/usage boundaries, plus unused-NaN numerical precedence. A private
mapped-view regression excludes a NaN storage row and compares logical reordered
geometry/usage to a materialized equivalent. The older 0.5 packed helper alone
does not check the release's mapping field, so it is not used by this new path.

151/125/130 root tests (all/no-default/experimental), strict Clippy, fourteen
phase contracts and native/libm/executed-WASM 30,000 cases each pass. Receipts
are raster-reader-mapped-proof; earlier reader-only proof/counts remain historical.
Final counts/assembly are raster-reader-mapped-counts: most observed cases are
instruction-neutral; default medium overdraw improves slightly (~0.2%). This
retains an explicit checked layout seam without claiming a large or timed gain.
No other API, float operation, unsafe, SIMD or frozen input is changed.
