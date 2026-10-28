# Typed vertex-layout extension

Base: v0.2.14, `dbb75e03ec011ae04645e6d40b0039561991286d`.

## API and decisions

`VertexLayout<V, N>::new(offset, selector)` derives stride from `size_of::<V>()`
and checks stride, alignment, field width and offset arithmetic. The selector's
`fn(&V) -> &[f32; N]` type checks element type and field width without a macro.
`copy_from` checks the selected address on every record. `VertexBuffer::from_vertices`
combines both steps. Its `positions` and `attributes` methods return existing views.
This extension is explicitly opt-in and not in C++ meshoptimizer.

A safe generic struct-to-byte borrow would expose potentially uninitialized
padding. Instead, snapshot construction fallibly allocates one full struct stride
per vertex, zeroes it and copies only selected float bits in little-endian order.
The snapshot retains the original stride and offset. Other fields and padding
are never read. Caller-owned snapshot bytes are outside Workspace accounting;
input size must be bounded by the caller. This is an allocating, copying API,
not a zero-copy view or general vertex serializer. Existing input storage,
constructors, dispatch and algorithm implementations are untouched.

Float typing is a compile-time requirement (negative doctest for `[u32; 3]`),
while layout/address failures have typed runtime errors. Empty slices validate
static layout but cannot validate a selector address; empty existing views use
offset zero internally. Non-finite values and signed zero retain their bits;
downstream geometric operations retain their existing validation semantics.

No new dependencies, feature gates, proc macros or version bump. A future
optional derive might generate selectors/offsets, but is not needed for this API.
No performance claim: allocation and full-stride storage are deliberate costs.

## Bounded differential design

Driver: `tests/vertex_layout.rs`, four tests, plus rustdoc and guide examples.
The bounded cases were selected before full qualification:

- 12-byte packed position arrays, 32-byte aligned records, 40-byte records with
  both internal and trailing padding; explicit hand-written stride/offset values.
- Independent little-endian serialization of two padded records, plus attribute
  component comparisons, signed zero, a NaN payload, infinity and empty input.
- Misaligned offset, insufficient struct alignment, zero/33-component fields,
  zero/oversized/non-multiple-of-four strides, offset addition overflow and fields
  extending beyond a record. External selectors, wrong fields and a selector
  changing fields on the second record must fail.
- One deterministic 9-by-9 curved grid (81 vertices, 128 triangles), compared
  against independently built packed and interleaved views. Check exact simplify
  indices and error bits, attribute-aware simplify indices/error bits with UVs,
  and every meshlet descriptor, vertex reference and triangle byte.

These establish helper/manual equivalence for the recorded cases. They are not a
new C++ corpus qualification. Existing algorithm source remains identical to base.

## Read-only adversarial pass

Self-review only; no subagents or independent reviewer. Checked generic width
overflow (short-circuit rejects N > 32 before multiplying), checked record size
and platform allocation bounds, pointer-address comparison without dereferencing
erased memory, selector changes across records, empty nonzero-offset fields,
snapshot lifetime and float-bit preservation. Removed an unnecessary infallibly
allocating Clone implementation. No unsafe code or lint escapes were introduced.
Selector panics propagate and selectors may have side effects; both remain caller
code. Allocation failure is handled but was not fault-injected. An independent
release review and cross-platform/C++ qualification remain release-lane work.

## Qualification

Linux x86_64, Rust 1.98.1; no performance measurements. All builds and test
commands ran under the required 4 GiB admission runner, GPU disabled,
incremental compilation disabled, using the task's dedicated target directory
(the direct task instruction overrides the spec's target-directory name).

| Command | Result |
| --- | --- |
| `cargo test --locked --all-features` | exit 0; includes 4 helper tests; doctests 27 passed, 5 ignored |
| `cargo test --locked --no-default-features` | exit 0; includes 4 helper tests; doctests 24 passed, 5 ignored |
| `cargo test --locked --doc --all-features` | exit 0; 27 passed, 5 ignored |
| `cargo test --locked --test vertex_layout --all-features` | exit 0; 4 passed after the test-only lint fix |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | final exit 0 |
| `cargo fmt --check` | exit 0 |
| `python3 parity/simd/check_boundary.py` | exit 0; one allowance, 23 documented blocks |
| `cargo package --locked --allow-dirty --list \| python3 parity/simd/check_package.py` | exit 0 with pipefail; one published allowance |
| `bash parity/tools/check_public_text.sh` | exit 0 |
| `cargo +1.88 test --locked --no-default-features --test vertex_layout` | exit 0; 4 passed |
| `git diff --check` | exit 0 |

Initial clippy exited 101 for `chunks_exact_to_as_chunks` in the manual test
fixture. Replaced that iteration with `as_chunks_mut::<10>()`, reran the affected
test and clippy successfully. No production code changed after the full suites.
The first all-features run was reclaimed/requeued by admission before completing
successfully; this is not a timing result.

The boundary script's two constituent checks were run explicitly, with package
listing using normal verification options to respect the task's flag restriction.
No boundary rule was weakened. Existing ignored exhaustive-root and migration
doctests were not executed. No fresh C++ differential corpus, cross-target run,
allocator fault injection, independent review or performance campaign was run.
The focused Rust 1.88 result is not a full MSRV release qualification.
