# Verification

How meshoptimizer-rs checks parity with C++ meshoptimizer 1.3, and what the checks cover.

## Parity scope

The port targets commit `4c203430ca565cb59a468a91922c76c208169536`, whose `src` tree equals
upstream tag v1.3. Parity is exact against the scalar implementation on the recorded inputs: index
order, floating-point error bits and encoded bytes match. Fixtures, differential sweeps, fuzzing and
executed WASM checks establish parity on finite recorded inputs. They do not prove it for every
possible input. See
[UPSTREAM.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.5/UPSTREAM.md).

## Safety and verification

Public APIs check slices, layouts, indices, finite geometry, overflow and limits, returning typed
`Error` values. Crate-controlled allocations use fallible reservations. The only unsafe allowance is
the private `codec::simd` module; each block has a safety argument in
[SAFETY.md](https://github.com/Tavrin/meshoptimizer-rs/blob/v0.2.5/src/codec/simd/SAFETY.md).
Simplification, optimization, encoders and index/sequence decoding are safe Rust. Scalar builds
forbid unsafe code. SIMD must match the canonical scalar bytes; filters keep scalar arithmetic
instead of upstream's SIMD approximations.

Recorded Miri runs cover scalar codecs, dispatch and unwind handling, x86 integer kernels, filters
and meshlets. NEON is not Miri-covered. Differential fixtures, seeded sweeps and fuzz targets
compare outputs and malformed-input handling; CI defines cross-target replay and SIMD safety
checks. These checks are bounded: a compile check or short smoke run does not establish full
per-target release fuzz budgets or execution on every supported platform. See the
[SIMD verification record](https://github.com/Tavrin/meshoptimizer-rs/blob/release/0.2.0/parity/SIMD_RESULTS.md).

For vertex decoding on arm64, Rust NEON must match Rust scalar on every status
and destination byte. C++ scalar supplies the status and successful-output oracle;
inherited failure-write differences are counted. Upstream C++ NEON diverges from
its own scalar decoder on malformed streams, so those ISA divergences are recorded
with examples as disclosure only. x86 still requires zero C++ ISA divergences.

A malformed meshlet tail is accepted by scalar upstream and Rust but rejected by upstream SSE. Rust
follows the scalar oracle; the expanded strict sweep still fails on that upstream discrepancy. Two
Stride-32 cluster-LOD test setups are invalid because their protect masks exceed the layout. AVX2, AVX-512
and 32-bit ARM SIMD are not implemented; index/sequence decoding remains scalar.

## Cluster DAGs

`clusterlod::build_dag_into` reproduces the reference oracle 41/41 files byte for byte, including
emission-time position snapshots and dilation; zero distinct vertex tuples collide under the S1
hash. Position snapshots retain their emission-time bits when dilation moves source positions
later.

## Error and buffer behaviour

Most operations offer allocating, caller-buffer (`_into`) and, where relevant, `_in_place` forms.
Codec versions and levels are per-call settings. `Workspace` reuses scratch storage and enforces
per-call byte and work limits; its defaults are 1 GiB and 2^34 work units. Retained scratch is
reused when it fits; when a call would exceed the byte limit, scratch it does not use is released
first and the buffers it reuses are shrunk to the size it needs. Late caller-buffer errors can leave
a written prefix; unused tails are preserved. In-place calls preserve their input on failure.

Parallel batches return results in input order, matching sequential bytes at any thread count.
Limits apply per item, outside batch result slots and Rayon infrastructure; callers should bound
batch size. Rayon pool initialization can panic. Small hierarchy batches run sequentially to avoid
dispatch overhead.

For `build_dag_into`, admit the full caller capacities separately from `Workspace` limits. On
`BufferTooSmall` or any other error, discard output and restore original positions before retrying
with newly admitted storage. Tails beyond successful prefixes are preserved. Under `parallel`,
`build_dags_batch_into` accepts a bounded window of `DagBuildInput` values and returns counts or
errors in input order; each worker has its own workspace and per-item limits. The library does not
serialize a file format or promise an aggregate batch-memory bound.
