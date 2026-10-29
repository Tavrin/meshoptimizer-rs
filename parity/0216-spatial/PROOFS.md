# Spatial BVH SIMD observable preservation

Base: v0.2.15, `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
C++ oracle: 1.3 source at `4c203430ca565cb59a468a91922c76c208169536`.
Only the private box merge/area sweep changes. The scalar fallback retains the
base expression and loop. No public API, box layout, allocation, partition,
comparison, validation, work charge or failure point changes.

## Arithmetic identity

For each axis, forward accumulator visit i reads `boxes[order[i]]`; reverse
visit i reads `boxes[order[n-1-i]]`. They start with the same MAX/-MAX bounds.
The first three coordinates are independent. SSE min/max select the incoming
second operand on equality, exactly as the scalar ordered selects do, including
opposite signed zeros. NEON/wasm use strict less/greater comparisons and bit
selection, rather than their floating min/max instructions. Induction over the
same visits gives identical bound bits. Inputs reaching the sweep are finite:
the unchanged centroid validation rejects nonfinite bounds before splitting.

Each ISA subtracts hi-lo in each lane. SSE/wasm multiply lanes `[x,y,z]` by
`[y,z,x]`, then add `(xy+yz)+zx`; NEON extracts x/y/z and evaluates that exact
expression. There is no FMA, approximate arithmetic or reassociation. The
fourth lane is initialized padding and contributes nothing. Overflow can
produce infinity or NaN; NaN payload bits are not observable by the split
comparisons. Direct kernel tests compare finite/zero/infinite area bits and
require NaN classification agreement. End-to-end tests require exact output
bytes, including cases with signed zeros and large finite coordinates.

Therefore every area consumer sees the same numerical value/classification,
every split comparison has the same result, and the unchanged stable partition
produces the same axes, boundary marks, descriptors, vertices and triangles.

## Work, failures, storage and caller writes

`ctx.tick(n)` remains immediately before each axis sweep. All other charges,
allocations and fallible operations remain at their original locations. The
new sweep is infallible on the builder's validated indices, uses checked Rust
indexing, and writes only internal areas. It performs no allocation and does
not touch the workspace or caller output. Its vectors replace scalar local
accumulators. There is no new storage charge or changed peak allocation.

Thus each work or byte limit reaches the same preceding failure or the same
post-sweep operation with identical metering and state. Caller buffers and
their unwritten suffixes are unchanged on success and failure. This argument
also applies to reused workspaces after successful and failed calls.

## ISA and memory boundary

The existing dispatch issuer alone creates Baseline tokens. SSE2, AArch64
NEON and wasm simd128 kernels live inside `codec::simd`. SSE2 and NEON each have
one unsafe token-authorized entry; the wasm entry is safe Rust because simd128 is
enabled crate-wide (an unsafe block there is `unused_unsafe`, which fails the
wasm `clippy -D warnings` CI step). Each kernel has one unsafe unaligned load
from `&[f32;4]`. Those
arrays contain exactly 16 initialized bytes; no record overread or tail
padding is assumed. Zero-length and 1-3-element sweeps retain checked indexing.
The no-simd build omits the entire audited module and is checked with
`-Funsafe_code`. Inventory rises from 23 to 28 blocks, retaining one allowance.
The independent package gate now verifies exact block inventory as well.

## Executable proof design

`exact.rs` links archived base and frozen candidate with separate package
names. Both feature modes enumerate every integer work limit from zero past
unlimited usage plus a conservative margin. Thirteen boundary inputs, four
settings and three priming states cover cold workspaces, successful priming,
and invalid-index failure priming. Each observation executes the owned and
caller-buffer API twice on the same workspace and compares Results, complete
Usage, all caller bytes and sentinel tails. The SIMD build repeats under
forced scalar, SSE2, SSSE3 and SSE4.1 ceilings. A size-only pass records exact
enumeration counts before execution; each process has a two-CPU-hour ceiling.

`run.py` builds the pinned C++ protocol twice (scalar and default SSE), native
Rust in both feature modes, and separate scalar/SIMD wasm artifacts. Node
executes both wasm artifacts. Two thousand generated spatial corpus messages
(owned/into), 28 signed-zero/finite-extreme/tail cases, and all three real
meshes compare exact output bytes across these arms and forced native levels.
No AArch64 user-mode/native runner is installed: NEON is compile-checked, and
NEON execution remains unverified.

## Performance method

Five arms: default base, default candidate, scalar candidate, scalar C++ 1.3,
and default C++ 1.3 with SSE enabled. The established peer allocation and
serialization driver is extended only with spatial settings (64 vertices,
41 minimum/124 maximum triangles, fill 0.5). Setup, output serialization and
warmup precede the counted resident loop. Each case has three samples; the
middle repetition reverses arm order. A single allowed CPU is pinned.
`instructions:u` must report 100% running time. Wall duration is disclosed
separately and does not determine the verdict. All eight frozen corpus/real
input hashes and every output hash are retained.

SHIPS requires real-mesh geometric mean candidate/base <= 0.95 and every
case <= 1.01. Scalar/scalar-C++ and default/default-C++ are separate ratios.
Results and exit receipts are recorded in REPORT.md after qualification.
