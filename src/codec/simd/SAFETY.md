# SIMD safety audit

This module is private. Its only unsafe operations are calls authorized by an
ISA token and fixed-array vector loads/stores. All value intrinsics run inside
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

| Block (source line) | Kind and safety argument |
|---|---|
| `src/codec/simd/aarch64.rs:66` | fixed-array memory access: the initialized array supplies 16 readable bytes; vld1q_u8 permits byte alignment and the pointer is not retained. |
| `src/codec/simd/aarch64.rs:73` | fixed-array memory access: the exclusive array supplies 16 writable bytes; vst1q_u8 permits byte alignment and the pointer is not retained. |
| `src/codec/simd/aarch64.rs:106` | token-authorized target_feature call: Baseline is issued only on this architecture with neon enabled; the safe kernel checks all slice bounds. |
| `src/codec/simd/aarch64.rs:111` | token-authorized target_feature call: Baseline proves neon availability; complete initialized input and exclusive output arrays supply all memory used by the kernel. |
| `src/codec/simd/aarch64.rs:115` | token-authorized target_feature call: Baseline proves AArch64 NEON, and the complete input is typed. |
| `src/codec/simd/aarch64.rs:141` | token-authorized target_feature call: Baseline proves this backend's ISA; the kernel uses only values and the already audited fixed-array store helper. |
| `src/codec/simd/aarch64.rs:274` | token-authorized target_feature call: the dispatch token proves this kernel's ISA, and the kernel checks each complete input window and scalar-equivalent bound before use. |
| `src/codec/simd/wasm32.rs:63` | fixed-array memory access: the initialized array supplies 16 readable bytes; v128_load permits unaligned addresses and retains no pointer. |
| `src/codec/simd/wasm32.rs:70` | fixed-array memory access: the exclusive array supplies 16 writable bytes; v128_store permits unaligned addresses and retains no pointer. |
| `src/codec/simd/x86.rs:10` | fixed-array memory access: the borrowed initialized array supplies exactly 16 readable bytes; loadu has no alignment requirement and retains no pointer. |
| `src/codec/simd/x86.rs:17` | fixed-array memory access: the exclusive array supplies exactly 16 writable bytes; storeu requires no alignment, and the pointer is not retained. |
| `src/codec/simd/x86.rs:99` | token-authorized target_feature call: Baseline is issued only for detected x86-64 SSE2; x86-64 makes SSE2 mandatory. All slice bounds are checked by the safe kernel. |
| `src/codec/simd/x86.rs:105` | token-authorized target_feature call: Ssse3 is constructed only after SSSE3 AND POPCNT detection (or compile-time features in no_std); the kernel's complete input is typed. |
| `src/codec/simd/x86.rs:168` | token-authorized target_feature call: Sse41 is issued only after SSSE3, POPCNT and SSE4.1 are detected; the kernel uses SSSE3/SSE4.1 on a complete initialized byte array. |
| `src/codec/simd/x86.rs:200` | token-authorized target_feature call: Baseline proves SSE2. The kernel slices only validated component planes, stages short vectors, and scatters via checked destination slices. |
| `src/codec/simd/x86.rs:251` | token-authorized target_feature call: Baseline proves this backend's ISA; the kernel uses only values and the already audited fixed-array store helper. |
| `src/codec/simd/x86.rs:395` | token-authorized target_feature call: the dispatch token proves this kernel's ISA, and the kernel checks each complete input window and scalar-equivalent bound before use. |

Inventory: **17 unsafe blocks**, one module-level allowance. The token-aware
boundary gate checks code tokens and exact file/line inventory independently
for repository sources and the published package. The unsafe-free build is
checked with --no-default-features and RUSTFLAGS=-Funsafe_code.

Miri execution and intrinsic coverage, per-target parity, ASan differential
fuzzing budgets, exhaustive filter/sqrt records and performance qualifications
are recorded in parity/SIMD_RESULTS.md. Finite tests are not a proof of UB
absence. AArch64 NEON is not Miri-covered; its execution/fuzzing requires
native CI. A level without its release safety/parity record must not ship.
