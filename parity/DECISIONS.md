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
