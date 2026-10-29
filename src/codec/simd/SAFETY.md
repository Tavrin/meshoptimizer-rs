# SIMD safety audit

This module is private. Its only unsafe operations are calls authorized by an
ISA token and fixed-array vector loads/stores, including spatial box sweeps. All value intrinsics run inside
a matching target_feature function. No transmute, raw-pointer arithmetic,
MaybeUninit, get_unchecked, static mut, heap allocation or thread-local kernel
state is used. Scalar parsing validates the layout and the complete 24-byte
vertex group before dispatch. Meshlet windows are checked 16-byte arrays.
Loads/stores permit byte alignment. Tails are staged in initialized stack
arrays and copied with checked slices. Stack scratch stays below 16 KiB
(vertex planes 1024 bytes; filter batching under 2 KiB). Immutable const
shuffle tables require no initialization. Detection is cached in AtomicU8;
only dispatch.rs constructs tokens. Optional diagnostic ceilings cannot exceed
detected capabilities. With std ceilings are thread-local and unwind-safe.
Without std diagnostic ceilings are global: concurrency can only lower them,
so outputs and instruction safety remain unchanged.

Filters use the scalar arithmetic order, IEEE sqrt/div, sign-biased truncation,
and no FMA. Oct checks zero length before conversion; integer Oct/Quat fields
bound every conversion in i32. Color validates conversion range or falls back
to the checked scalar kernel before writing that batch. Exp uses scalar bit
construction and multiplication. The scalar reuse cache remains available.

The NEON full vertex path uses checked 24-byte group windows (or a
checked 96-byte window for four groups), initialized 1024-byte plane scratch
and the canonical per-channel commit order. Zip operations transpose four
planes into records. Byte/halfword wrapping sums or word XOR prefixes shift
by one then two records and add/XOR the previous record. Variable NEON shifts
implement exact 32-bit rotation, including rotation zero. Only complete
16-record groups use vector reconstruction; the canonical scalar tail uses
the last reconstructed record. No output overstore or input overread is used.
Native arm64 CI must execute forced scalar/NEON comparisons, including errors,
partial writes and destination tails; local cross-compilation is not identity
or performance evidence.

Geometry kernels take eight independent roots in initialized four-float
windows. Only nonnegative finite operands use hardware sqrt; all other bit
patterns retain the pinned scalar backend, including negative zero and NaNs.
Sphere expansion, cone math and strict tie decisions keep their original
order. SSE2, NEON and simd128 use exact IEEE sqrt, with no FMA, reciprocal
estimate or approximate root. Diagnostic ceilings also apply to geometry.
Geometry batching adds bounded stack arrays and no heap allocation or workspace
charge. The rejected KD distance experiments are recorded in the P3 proofs.

| Block (source line) | Kind and safety argument |
|---|---|
| `src/codec/simd/aarch64.rs:66` | fixed-array memory access: the initialized array supplies 16 readable bytes; vld1q_u8 permits byte alignment and the pointer is not retained. |
| `src/codec/simd/aarch64.rs:73` | fixed-array memory access: the exclusive array supplies 16 writable bytes; vst1q_u8 permits byte alignment and the pointer is not retained. |
| `src/codec/simd/aarch64.rs:106` | token-authorized target_feature call: Baseline is issued only on this architecture with neon enabled; the safe kernel checks all slice bounds. |
| `src/codec/simd/aarch64.rs:111` | token-authorized target_feature call: Baseline proves neon availability; complete initialized input and exclusive output arrays supply all memory used by the kernel. |
| `src/codec/simd/aarch64.rs:115` | token-authorized target_feature call: Baseline proves AArch64 NEON, and the complete input is typed. |
| `src/codec/simd/aarch64.rs:141` | token-authorized target_feature call: Baseline proves this backend's ISA; the kernel uses only values and the already audited fixed-array store helper. |
| `src/codec/simd/aarch64.rs:274` | token-authorized target_feature call: the dispatch token proves this kernel's ISA, and the kernel checks each complete input window and scalar-equivalent bound before use. |
| `src/codec/simd/aarch64.rs:495` | token-authorized target_feature call: Baseline proves AArch64 NEON. Every load/store uses an initialized fixed array behind checked slices; the parser validates complete group windows and keeps scalar channel commit order. |
| `src/codec/simd/bounds.rs:21` | token-authorized target_feature call: Baseline proves the selected backend's ISA. Both complete initialized arrays are accessed only through fixed four-float windows. |
| `src/codec/simd/bounds.rs:63` | fixed-array memory access: v supplies exactly 16 initialized readable bytes. loadu permits unaligned input and retains no pointer. |
| `src/codec/simd/bounds.rs:70` | fixed-array memory access: v supplies exactly 16 exclusive writable bytes. storeu permits unaligned output and retains no pointer. |
| `src/codec/simd/bounds.rs:87` | fixed-array memory access: v supplies 16 initialized readable bytes, with the alignment permitted by vld1q_f32. The pointer is not retained. |
| `src/codec/simd/bounds.rs:94` | fixed-array memory access: v supplies 16 exclusive writable bytes, with the alignment permitted by vst1q_f32. The pointer is not retained. |
| `src/codec/simd/bounds.rs:111` | fixed-array memory access: v supplies 16 initialized readable bytes. v128_load permits unaligned input and retains no pointer. |
| `src/codec/simd/bounds.rs:118` | fixed-array memory access: v supplies 16 exclusive writable bytes. v128_store permits unaligned output and retains no pointer. |
| `src/codec/simd/wasm32.rs:58` | fixed-array memory access: the initialized array supplies 16 readable bytes; v128_load permits unaligned addresses and retains no pointer. |
| `src/codec/simd/wasm32.rs:65` | fixed-array memory access: the exclusive array supplies 16 writable bytes; v128_store permits unaligned addresses and retains no pointer. |
| `src/codec/simd/x86.rs:10` | fixed-array memory access: the borrowed initialized array supplies exactly 16 readable bytes; loadu has no alignment requirement and retains no pointer. |
| `src/codec/simd/x86.rs:17` | fixed-array memory access: the initialized array supplies eight readable bytes; loadl permits byte alignment, zeroes upper lanes and retains no pointer. |
| `src/codec/simd/x86.rs:37` | fixed-array memory access: the exclusive array supplies exactly 16 writable bytes; storeu requires no alignment, and the pointer is not retained. |
| `src/codec/simd/x86.rs:279` | token-authorized target_feature call: Baseline is issued only for detected x86-64 SSE2; x86-64 makes SSE2 mandatory. All slice bounds are checked by the safe kernel. |
| `src/codec/simd/x86.rs:336` | token-authorized target_feature call: Ssse3 is constructed only after SSSE3 AND POPCNT detection (or compile-time features in no_std); the kernel's complete input is typed. |
| `src/codec/simd/x86.rs:396` | token-authorized target_feature call: Ssse3 proves SSSE3 and POPCNT. The kernel checks a complete 24-byte window before each group and writes checked output chunks only. |
| `src/codec/simd/x86.rs:502` | token-authorized target_feature call: Sse41 is issued only after SSSE3, POPCNT and SSE4.1 are detected; the kernel uses SSSE3/SSE4.1 on a complete initialized byte array. |
| `src/codec/simd/x86.rs:534` | token-authorized target_feature call: Sse41 proves SSSE3/SSE4.1; the kernel checks every 16-byte input window and emits only the scalar decoder's validated record count. |
| `src/codec/simd/x86.rs:576` | token-authorized target_feature call: Baseline proves SSE2. The kernel slices only validated component planes, stages short vectors, and scatters via checked destination slices. |
| `src/codec/simd/x86.rs:701` | token-authorized target_feature call: Baseline proves this backend's ISA; the kernel uses only values and the already audited fixed-array store helper. |
| `src/codec/simd/x86.rs:933` | token-authorized target_feature call: the dispatch token proves this kernel's ISA, and the kernel checks each complete input window and scalar-equivalent bound before use. |
| `src/codec/simd/x86.rs:954` | token-authorized target_feature call: Ssse3 proves SSSE3 and POPCNT for every private kernel choice. All retain checked layout, group lookahead and bounded destinations; stack scratch is at most 1,280 bytes; memory operations are array-backed. |
| `src/codec/simd/x86.rs:1141` | token-authorized target_feature call: Sse41 proves SSSE3/SSE4.1; every input load is checked and array-backed, and complete groups and tails use counted output slices. |
| `src/codec/simd/x86.rs:1169` | token-authorized target_feature call: Sse41 proves SSSE3/SSE4.1; every input load is checked and array-backed, and complete groups and tails use counted output slices. |
| `src/codec/simd/spatial.rs:30` | fixed-array memory access: four initialized floats supply exactly 16 readable bytes; SSE loadu permits unaligned addresses and retains no pointer. |
| `src/codec/simd/spatial.rs:57` | token-authorized target_feature call: Baseline proves SSE2; the sweep uses checked indices and fixed-array loads only. |
| `src/codec/simd/spatial.rs:89` | fixed-array memory access: four initialized floats supply exactly 16 readable bytes; NEON load permits unaligned addresses and retains no pointer. |
| `src/codec/simd/spatial.rs:115` | token-authorized target_feature call: Baseline proves AArch64 NEON; the sweep uses checked indices and fixed-array loads only. |
| `src/codec/simd/spatial.rs:147` | fixed-array memory access: four initialized floats supply exactly 16 readable bytes; wasm load permits unaligned addresses and retains no pointer. |

Inventory: **36 unsafe blocks**, one module-level allowance. The token-aware
boundary gate checks code tokens and exact file/line inventory independently
for repository sources and the published package. The unsafe-free build is
checked with --no-default-features and RUSTFLAGS=-Funsafe_code.

Miri execution and intrinsic coverage, per-target parity, ASan differential
fuzzing budgets, exhaustive filter/sqrt records and performance qualifications
are recorded in parity/SIMD_RESULTS.md. Finite tests are not a proof of UB
absence. AArch64 NEON is not Miri-covered; its execution/fuzzing requires
native CI. A level without its release safety/parity record must not ship.

Spatial sweeps dispatch once per axis and keep independent forward/reverse
accumulators in registers. SSE min/max select their second operand on equality;
NEON and wasm use strict comparison and bit selection to preserve exactly that
choice, including signed zero. Area uses (x*y + y*z) + z*x without FMA or a
horizontal reassociation. Lane 3 never reaches an area. Empty and short sweeps
need no padding outside the four-float record, heap scratch or unchecked access.
Work charges and fallible operations remain outside the infallible sweep.
Spatial qualification and the native NEON execution limitation are recorded in
parity/0216-spatial/PROOFS.md.
