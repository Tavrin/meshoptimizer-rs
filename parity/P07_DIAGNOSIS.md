# P07 SIMD diagnosis — 2026-10-05

## Decision and scope

Open a new implementation lane for full-group reconstruction and output, then
wasm batching. The dominant native vertex cost is the checked, counted scatter
around otherwise packed SIMD math; the dominant wasm cost is inside raw decoding.
Keep exact scalar arithmetic, malformed-stream checks and the single audited
unsafe module. Do not substitute upstream rounding or estimates to clear the bar.

Diagnosis starts from clean `phase/0.7` HEAD `002872f` (implementation `4e7eea0`).
The probe CPU is 26 on an AMD Ryzen 9 7945HX; all reported PMU
captures have 100% event scheduling, with no unsupported counters.
No production source, tests, frozen inputs or numeric bars changed. Only this
report and the stale summary section of SIMD_RESULTS are edited and committed.
Throwaway source lives exclusively under the artifact directory below.

Evidence root: `/mnt/linux-extra/meshopt-artifacts/p07-diag/`. Read `records.json`,
`counts.json`, `wasm-counts.json`, `followup-counts.json`, `asm-summary.json`,
`wat-summary.json`, `v8-summary.json`, `samples.json`,
`historical-native-counts.json`, `manifest.json` and their
referenced raw `.stat`, `.input`, `.response`, `.report` and assembly files.
The controller scripts are retained beside them. These are focused diagnostic
probes, not a new matrix, performance acceptance, Miri or release qualification.

## 1. Reconcile measurement epochs

| Epoch / implementation | Vertex GM allocating / caller | S3 significant comparisons | wasm S5 GM allocating / caller | wasm worst allocating / caller |
|---|---:|---:|---:|---:|
| original / `2feefb2` | 2.283 / 2.610 | 49 | 2.457 / 2.769 | 6.732 / 7.281 |
| second / `5b84289` | 2.065 / 2.115 | 67 | 1.330 / 1.354 | 2.454 / 2.724 |
| current / `4e7eea0` | 1.832 / 1.967 | 19 | 1.310 / 1.361 | 2.537 / 2.740 |

`parity/SIMD_PERFORMANCE.md` is current: it reads the complete round-three
276-row native and 174-row Node records in `p07r3`. The old native table in
`SIMD_RESULTS.md` was an unlabeled original-epoch table, not a different mean
formula or a contradiction in the raw data. Its vertex 2.283/2.610, S3 49,
S1/S2 and S4 summaries are now replaced by the current values. Historical
before/after sections remain labeled and preserved. The original wasm table is
also explicitly labeled historical, with the current S5 figures beside it.

Every epoch uses stage-1 medians for family geometric means; D146 only resolves
borderline maxima. Repeated standalone Oct/Quat are separate from varied-filter
means. `records.json` independently recomputes S3 from both stages and reproduces
49, 67 and 19 comparisons. These count API/case/ceiling comparisons, not 49
different inputs. Current S3 splits 6 allocating / 13 caller-buffer; ceilings
split default 8 / SSE2 11. S1/S2/S5 still fail both APIs. Allocating S4 fails
three maxima, caller-buffer S4 and all frozen minima pass. No bar is relaxed.

## 2. Counter method and identity

Use user-mode `perf stat -e instructions:u,cycles:u,branches:u,branch-misses:u`
on CPU 26, with unchanged archived drivers and frozen requests. The distro
`/usr/bin/perf` wrapper fails on kernel 7.0.0-28; the working installed executable
is `/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf`. Its counters execute successfully.
No tool installation or kernel setting change is needed.

For each native API probe collect N and 2N calls in separate processes and report
`(counter(2N) - counter(N)) / (N * decoded_bytes)`. N is 100,000 for tiny inputs,
1,000 for resident inputs and 100 for streaming inputs. One identical warmup call
and startup/transport are thus subtracted. Caller buffers are selected except
the separately labeled allocating-sequence followup. Meshlet bytes are
`vertex_count*vertex_stride + triangle_count*triangle_stride`; raw bytes are
`4*(vertex_count + triangle_count)`. The initial meshlet denominator error was
corrected from MC04 fields without rerunning or changing a counter; the correction
is explicit in counts.json. Raw captures remain intact.

The existing native driver also returns a legacy clock field. It is retained in
responses but never assessed as a new wall-clock experiment. No timing pair or
wall-clock micro-experiment was run, so no timing lease was needed. Cycles are
diagnostic, and cycle/misprediction subtraction can be noisy (even negative
branch-miss differences). Rank causes using retired instructions, branches,
assembly and sampled attribution, not those noisy differences.

The before/after source and all six binary hashes in `p07r3/build.json` were checked
against this checkout and the archived binaries. Frozen input hashes were checked
before probes; source hashes were checked again afterward. Upstream is a clean
checkout at `4c203430ca565cb59a468a91922c76c208169536`, checked by
`parity/check-reference.sh`. Original-epoch wasm probes use its archived module
and round-three frozen bytes. Module/output hashes are retained per probe.

| Executed object | SHA-256 |
|---|---|
| native Rust SIMD | `9c454aa5fd3e51befb36b1773912ed3a696d8630f7df46f8c74bf8cd19275796` |
| native C++ SIMD | `2ae0a9f7cf2485919968f388e1ed3913366ab78b1f003458801ad37657ec3e58` |
| native safe Rust scalar | `a772666a240256a1cbb82e6c1011561ffbca6344786fe4c2d3f54b75b86116ee` |
| current arithmetic wasm SIMD | `066a236807fa6397bcc5db7b902e0721a2dfb65aefe1c947c6df9fa86267a776` |
| original arithmetic wasm SIMD | `3b72bc5b44784f67ff494918d58f982b4c3ed4783b18c98496f7e780a53068d4` |
| shipped JS decoder | `10b313edaa0e322b0c1b46443f6c8b186b42139fe1eca189d50c09511d0ebedc` |
| selected shipped SIMD wasm | `757416de9b71e606c813fe6726b3bf2ab675682c3d6b99042126e7a7e16c2c60` |

Rust native flags are release/fat-LTO/one codegen unit, generic CPU,
`std,simd,parity-internals`; C++ uses GCC 13.3 `-O3 -DNDEBUG -fno-fast-math
-ffp-contract=off`. Rust compiler is 1.98.1 / LLVM 22.1.8. Node is 22.22.0.
The upstream shipped wasm header records clang 22.1.0-wasi-sdk. SIMD wasm has
`-C target-feature=+simd128`. This is a comparison of the recorded driver builds;
diagnostic lowering instrumentation is present and tested separately below.

## 3. Retired native instructions and branches per decoded byte

Both columns below use the same input and caller-buffer API. Full raw totals,
repeat counts, commands, exit codes and output hashes are in counts.json.
Rust successful outputs also agree with safe scalar Rust; C++ filters retain
the already documented distinct conformance contract.

| Frozen input | Bytes/call | Rust insn/B | C++ insn/B | Insn ratio | Rust branches/B | C++ branches/B |
|---|---:|---:|---:|---:|---:|---:|
| `vertex-v1-resident-s12` | 49164 | 14.265 | 5.715 | 2.496 | 1.437 | 0.146 |
| `view-none-resident-s12` | 49164 | 14.225 | 5.708 | 2.492 | 1.439 | 0.141 |
| `varied-view-2-resident-s8` | 32776 | 15.913 | 6.914 | 2.302 | 1.417 | 0.228 |
| `varied-filter-1-resident-s4` | 16388 | 5.782 | 3.644 | 1.587 | 0.130 | 0.067 |
| `varied-filter-1-resident-s8` | 32776 | 3.016 | 1.978 | 1.525 | 0.065 | 0.033 |
| `varied-filter-2-resident-s8` | 32776 | 3.328 | 2.572 | 1.294 | 0.034 | 0.033 |
| `varied-filter-4-resident-s8` | 32776 | 3.045 | 2.040 | 1.493 | 0.065 | 0.033 |
| `meshlet-64-126-v4-t3` | 634 | 5.085 | 2.951 | 1.723 | 0.667 | 0.203 |
| `meshlet-raw-64-126` | 760 | 3.983 | 2.575 | 1.547 | 0.547 | 0.247 |
| `index-3-v1-resident-s4` | 16388 | 6.514 | 5.008 | 1.301 | 1.002 | 1.002 |
| `index-3-v1-streaming-s4` | 8388612 | 6.500 | 5.000 | 1.300 | 1.000 | 1.000 |
| `varied-filter-3-resident-s4` | 16388 | 0.734 | 0.828 | 0.887 | 0.034 | 0.066 |

### Vertex and views: dynamic scatter, spills and function boundaries

Dispatch: the host supports SSE4.1; default vertex takes `x86::vertex_kernel`
(SSSE3+POPCNT), reconstruction SSE2. SSE2 ceiling falls back to safe byte parsing
but still calls SSE2 `deltas8_kernel`. NONE views share this raw path; filtered
views then call the filter once. CPUID detection is cached, not repeated per call.

Side-by-side full-group structure (native Rust x86.rs:340–480, 573–645, 882–980;
upstream vertexcodec.cpp:decodeBytesGroupSimd and decodeDeltas4Simd):

| Work per full group | C++ SIMD | Rust SIMD |
|---|---|---|
| Byte plane: 16 decoded bytes | packed u64 consumption; 8-byte packed load, 16-byte escape load; 4 config-vector reads, 2 eight-byte escape-table reads; `pshufb`, 2 `pmulhuw`, mask/SAD, `popcnt`, one output-vector store | Same packed core and eight-byte escape tables now; four groups under 96-byte window. Residual checked window advances and `first_chunk` failure branches remain. |
| Reconstruction: 16 records, one four-byte channel | four plane-vector loads, eight interleave operations, sixteen explicitly unrolled 32-bit strided stores; transpose before zigzag/prefix | four plane-vector loads and eight interleaves, then four packed prefix groups; `stride==4` gets vector stores, other strides enter counted, checked scalar scatter; live-record/tail decisions remain inside the full-group walk |
| Tail and bounds | reconstruct aligned scratch, copy live output once; pointer lookahead checks outside unrolled work | `min(16)`, four slice ranges, partial-plane zero buffers/memcpy, `min(4)`, chunks/zip iteration and individual output-span checks |
| Inlining and spills | delta/group bodies inline into `decodeVertexBlockSimd` (0x6480); no vector-to-stack stores in that complete function | distinct `vertex_kernel` (0x31540), `bytes_kernel` (0x331a0), `deltas8_kernel` (0x32a70); `vertex_kernel` calls bytes per plane and deltas per channel; array/iterator state materializes on stack |

Concrete reconstruction assembly: 0x32d50–0x32dab performs four checked full-plane
loads; 0x32dfa–0x32e15 writes all four transposed vectors to stack before the
counted record loop reloads them at 0x32f1b. The full scatter path at
0x32fa0–0x330fe contains repeated `imul`, `cmov`, comparisons and scalar
`movd`/`pshufd` stores. Four partial-plane memcpy calls live on the tail path
0x32c38–0x32d09. The complete delta function allocates a 0x168-byte stack frame,
has 105 stack-memory operand sites and 12 vector stack stores; these static counts
include tails/cold errors and are not counts per group. C++ has zero vector stack
stores in its whole block function. `deltas-annotate.txt` retains sampled locations.

The authenticated round-three resident-v1 cycle record attributes 60.54% to
deltas8, 33.79% to byte decoding and 4.89% to parsing. New retired counters reproduce
about 2.50x instructions and 9.85x branches for vertex. NONE is 2.49x / 10.18x.
The varied Quat view is 2.30x instructions / 6.21x branches, because it combines
this reconstruction with a cheaper but still more expensive filter. This is a
loop/codegen problem after dispatch, not evidence for more ISA detection caching.

### Oct, Quat and Color: exact rounding dominates the remaining delta

Default filters take baseline SSE2. Each loop processes four records; Oct8/Color8
load/store one 16-byte vector, Oct16/Color16 load/store two. Quat loads two vectors,
preserves four selectors and emits four rotated u64 records. Table lookups are
absent from these steady arithmetic loops. Four-record chunk bounds disappear;
one scalar tail remains (the resident input has 4097 records). All intrinsic
helpers inline into Rust `simd::filter` at 0x2d470. C++ has separate filter
functions. Neither compiler spills vector registers in these native steady loops;
Rust Quat packed temporaries are eliminated, so they are not a remaining cause.

Counts below are static instructions in the complete steady loop interval,
including its backedge and exceptional branch, excluding setup/tail. They are
distinct from the retired per-byte measurements above.

| Four-record loop | C++ interval; instructions / branches | Rust interval; instructions / branches | Specific difference |
|---|---|---|---|
| Oct8 | 0xcd50–0xce3c; 58 / 1 | 0x2ddb0–0x2df38; 92 / 2 | C++ `rsqrtps`, three `cvtps2dq`; Rust exact `sqrtps` + `divps`, zero-length error guard, sign-aware half addition + three `cvttps2dq` |
| Oct16 | 0xcc10–0xccfc; 63 / 1 | 0x2df90–0x2e124; 96 / 2 | Both use sqrt/div; extra Rust zero-length validation, exact arithmetic association and canonical rounding/packing. Estimates cannot explain this row. |
| Quat | 0xce80–0xcfbe; 82 / 1 | 0x2db90–0x2dd3c; 106 / 1 | Both use sqrt/div and four scalar rotations; Rust preserves scalar association, clamps with canonical compare/select, and rounds signed components with half addition + truncation instead of four nearest conversions |
| Color8 | 0xd160–0xd251; 56 / 1 | 0x2d7b0–0x2d92a; 91 / 2 | C++ reciprocal estimate + nearest conversion; Rust division, four half additions/truncations and eight lane-wise range comparisons combined into one fallback decision |
| Color16 | 0xd008–0xd123; 65 / 1 | 0x2d970–0x2daec; 97 / 2 | Both divide; Rust canonical rounding/range checks and fallback still add 32 static instructions |

These account for measured instruction ratios Oct8 1.587, Oct16 1.525, Quat 1.294
and Color16 1.493. The +24 Quat and +32 Color16 loop instructions persist even
with no spills or extra table reads. Upstream nearest/estimate results have a
different contract; changing Rust to those instructions without exact correction
would break parity. Range errors/exceptional conversion fallback must survive.

### Meshlets: remaining validation and two-triangle packing

Default takes SSE4.1 whole-output kernels; SSE2 is scalar, wasm meshlets are scalar.
Four vertex deltas use one 16-byte source load, one shuffle-table vector plus its
advance metadata, packed prefix math and one 16-byte u32 store (u16 output packs).
Each triangle pair uses an eight-byte source load, one 16-byte packed mask and
byte metadata; shuffle/increment/state remain in registers. C++ uses the same
kind of tables. There is no evidence that SIMD is absent in the measured path.

Rust typed kernel 0x29fd0 vertex loop 0x2a040–0x2a0d4 is 36 static instructions
with four branches: source lookahead, consumption, output bound and backedge.
C++ vertex loop 0x10828–0x108a9 is 31 with two branches. Rust triangle full-pair
loop 0x2a260–0x2a2fa is 36 with five branches; upstream 0x10910–0x109b0 is 36
with two branches but processes **two pairs/four triangles** before its backedge.
Upstream packs the four three-byte triangles with one 4-byte and one 8-byte
store. Rust writes each exact six-byte pair using narrower stores and retains
counter-wrap validation plus a separate odd tail. Raw output emits u32 triangle
records, so the typed three-byte store opportunity alone does not cover raw.

These loops inline their step/sink helpers; full-group native loops have no
vector stack spills. Whole-function static call sites (13 typed / 14 raw) mostly
include cold errors/tails; they are not 13 calls per group. The retired ratios
are 1.723x instructions and 3.280x branches typed, 1.547x / 2.213x raw.
Prior authenticated profiles attribute 84.86% / 76.40% of typed/raw cycles to
the grouped kernels, the rest largely API/driver work. Counter-wrap fallback
is required because the canonical counter is u32, not modulo 256.

### Sequence: scalar cursor checks, not a SIMD regression

Both Rust builds take scalar index.rs:198–227; upstream also takes scalar
`meshopt_decodeIndexSequence`. There is no SIMD sequence dispatch, lookup table
or vector spill. Each index uses one to five byte loads, one of two u32 last
baselines, and one u16/u32 destination store. Both compilers inline vbyte decode,
unroll its four continuation steps, and select the baseline via a one-bit index.

Rust raw 0x2b0f4–0x2b157 includes an integer-add overflow comparison and a checked
five-byte lookahead on every index. It tracks numeric position plus output iterator
state. C++ 0xca5d–0xcaa1 uses a cursor pointer, one `data >= safe_end` test, one
count backedge and the lead-byte branch. On the mostly single-byte frozen stream
this totals **26 versus 20 retired instructions per u32 index**, while both
retire four branches per index. No branch-prediction explanation is needed to
establish the six-instruction gap. Tiny/version/u16 variants retain their guards.

Allocating followup reproduces 6.500 / 5.000 instructions/B, the same 1.300 ratio
as caller-buffer. Cycle samples allocate 91.19% / 90.00% to Rust/C++ decoding
and 7.49% / 8.55% to memset. Thus extra Rust initialization or transport does
**not** explain the 2.888x allocating matrix maximum. That maximum is genuine
retained timing evidence, but its additional microarchitectural/shared-host cause
is not identified by these counters. Do not claim a sequence rewrite will clear
it; qualify that row under the specified admitted timing procedure in the next lane.

## 4. S3 inversion: classify the 49 historical comparisons and current 19

| Epoch | Default comparisons | SSE2 comparisons | Tiny/micro comparisons | Large Exp comparisons |
|---|---:|---:|---:|---:|
| original | 27 | 22 | 47 (38 named tiny, 9 one-vertex/one-triangle meshlets) | 2 |
| second | 37 | 30 | 65 | 2 |
| current | 8 | 11 | 17 | 2 |

All identities and failed API/case/ceiling intervals are in records.json. The
original 49 therefore do not show 49 large SIMD throughput failures. Original
meshlets paid per-group callbacks/checked extraction; round-three full-output
decoding removes their nine inversions. Small views/vertices retain setup,
counted partial groups and tail traffic. SSE2 compares a hybrid parser plus
SSE2 reconstruction with an independently compiled unsafe-free scalar library;
it is not a measurement of the SSSE3 byte kernel.

Two fresh original-binary probes use byte-identical original/round-three inputs
(`historical-native-counts.json`). Original `view-1-tiny-s8` costs 4804 / 833
instructions/branches per call versus safe scalar 4994 / 746: lower instruction
volume but more branches, so SIMD work count alone is not a speed guarantee.
Original one-vertex/one-triangle typed meshlet costs 494 / 73 versus 456 / 59
(+8.3% instructions, +23.7% branches). Its callback/partial-output overhead is
large enough to explain a meaningful inversion. Both outputs match scalar.
The historical delta kernel constructs plane prefixes and extracts every strided
record from temporary vector buffers; it retains partial-plane copies and
per-group dispatch calls. The historical typed meshlet emits through per-item callbacks.
These are structural evidence for the original inversions, independent of later
round-three profiles or changes.

Fresh caller-buffer instruction/branch counts **per call**, after startup subtraction:

| Input | Default SIMD | SSE2 ceiling | Safe scalar | SIMD library lowered scalar |
|---|---:|---:|---:|---:|
| `view-1-tiny-s8` | 5219 / 698 | 5261 / 922 | 4980 / 744 | 5857 / 983 |
| `view-3-tiny-s12` | 6310 / 712 | 6654 / 1091 | 6873 / 1034 | 7670 / 1219 |
| `vertex-v1-tiny-s12` | 6175 / 733 | 8174 / 1538 | 8403 / 1476 | 9193 / 1669 |
| `meshlet-1-1-v4-t3` | 435 / 55 | 607 / 88 | 459 / 59 | 575 / 80 |

For the actual default `view-1-tiny-s8` inversion the SIMD path costs 5219
instructions versus scalar 4980 (+4.8%), with 66.90% of cycle samples in
deltas8, 6.71% in bytes and 4.80% in memset. The repeated Oct filter already
falls back to the scalar exact-record cache (3.98% sampled); it is not repeatedly
performing four-vector normalization. `view-3-tiny-s12` puts 65.53% in deltas8,
12.20% in bytes and 6.14% in memset even though its instruction count is lower
than scalar. SSE2 `vertex-v1-tiny-s12` puts 59.71% in deltas8 and 29.17% in
the scalar parser. This distinguishes instruction throughput/latency and
small-copy overhead from simple instruction volume. Partial-plane memcpy and
spill-heavy scatter are real executed costs; instruction count alone does not
predict each tiny row’s time.

Feature detection is cached in dispatch.rs:24–60. `selected()` additionally
reads the diagnostic ceiling; safe-scalar library builds compile away SIMD.
A scratch same-source baseline and a scratch variant bypassing only ceiling
selection preserve exact probe outputs. Default instruction savings are 0.191%
(Oct view), 0.158% (Exp view), 0.081% (vertex). These are below the 5% pursuit
threshold; removing TLS ceiling checks is rejected as the main fix. The lowered
scalar column also shows that lowering is not identical to an unsafe-free build.

The two large current inversions are Exp, not vertex throughput. Resident-s4
uses 0.7343 SIMD versus 0.7337 safe-scalar instructions/B: the scalar compiler
already vectorizes this easy arithmetic, and the explicit vector path adds
dispatch/tail code with a different generated loop. One current streaming
interval is only 1.006–1.022, below this diagnosis’s approximately 5% pursuit
threshold. Retain both S3 failures, but do not spend this lane on marginal
shared-host Exp effects or infer per-call CPUID from them.

## 5. wasm: historical scalar islands and current checked loops

The shipped JS decoder is itself an embedded SIMD wasm module. Its feature
detector selects SIMD in this Node runtime; the selected module is extracted
without modifying upstream. Rust also executes simd128: WAT and V8 assembly
contain `i8x16.shuffle/swizzle`, packed loads/stores and vector math. Missing
whole-parser specialization is the issue, not absence of the target feature.

Counter probe: isolate one backend per Node process, 2000 warmup calls, then
4000 and 8000 calls with fixed input. Subtract totals as above. `public` retains
source/output copies and caller-buffer wrapper behavior; `raw` preloads source
and skips per-call JS copies (Rust still has bench_run/API validation, C++ calls
its export directly). These are diagnostic counter probes, not equivalent
performance-bar APIs. Tiering/startup subtraction can be imperfect; retired
ratios below are work ratios, not wall-clock ratios.

| Epoch / input / interface | Rust insn/B | C++ insn/B | Insn ratio | Rust branches/B | C++ branches/B |
|---|---:|---:|---:|---:|---:|
| p07 / `vertex-v1-resident-s12` / public | 55.134 | 5.848 | 9.429 | 4.548 | 0.354 |
| p07 / `vertex-v1-resident-s12` / raw | 55.084 | 5.719 | 9.632 | 4.540 | 0.339 |
| p07r3 / `vertex-v1-resident-s12` / public | 18.330 | 5.787 | 3.167 | 2.414 | 0.348 |
| p07r3 / `vertex-v1-resident-s12` / raw | 18.346 | 5.779 | 3.175 | 2.411 | 0.339 |
| p07r3 / `varied-view-2-resident-s8` / public | 20.165 | 6.983 | 2.888 | 2.367 | 0.395 |

Removing transfers leaves the historical work gap about 9.6x and the current
one about 3.2x. The historical-to-current Rust work drops from roughly 55 to 18
instructions/B (about 67%), consistent with replacing scalar bit extraction
and output work; additionally, the historical `simd::deltas8` dispatcher was
x86-only, so wasm reconstruction used scalar prefix loops. The current dispatcher
now admits simd128 for byte prefixes. This is diagnostic epoch evidence, not an isolated A/B attribution
of each code change. The current 3.2x work ratio coexists with a much smaller
1.31–1.37x vertex timing GM because SIMD/instruction dependencies and runtime
codegen differ. Never convert those ratios directly into a claimed time speedup.

Concrete side-by-side findings:

| Aspect | Shipped upstream SIMD wasm | Rust wasm |
|---|---|---|
| Historical 16-byte group | vector field extraction / masks / swizzle | historical `portable_group!` loops over 16 scalar fields, shifts/divides/indexes and builds mask; another eight-scalar-byte loop builds shuffle; only final expansion uses SIMD |
| Current 16-byte group | inlined, unrolled four groups per header; full decode token/path selected outside work | wasm32.rs:234–277 uses vector extraction, but generic vertex.rs:109–127 still calls `simd::group` per group, reads dispatch/cache and bits table, validates 24-byte slice, branches 0/8/bits; no wasm whole `bytes`/`vertex` path in simd/mod.rs |
| Escape tables | fixed packed SIMD configuration and escape shuffle tables | two 16-byte MASKS loads (4096-byte table), two popcounts; mask-to-popcount pointer advance still depends on vector bitmask |
| Prefix/reconstruction | transpose first; four vector plane loads, eight interleaves, unrolled vector-record prefix and sixteen lane-32 stores; handles byte/halfword/XOR channels in SIMD | current byte prefix is performed per plane: four doubling shuffle/add stages plus previous value per component, then eight interleaves and checked scatter; halfword and rotated XOR still use scalar `decode_deltas` |
| Checked control/code size | shipped raw vertex WAT has 37 `br_if` sites and 9 loops | Rust monolithic raw WAT has 257 `br_if` sites, 19 loops and 32 call sites; includes index/sequence alternatives and cold errors, so these are static sites, not per-group counts |
| V8 codegen | optimized raw function 2: 10,456 instruction bytes / 197 jump sites | optimized raw function 36: 19,016 instruction bytes / 609 jump sites; includes other decoder arms. Raw V8 assembly is retained, including vector code and calls to checked error paths. |

Both optimized V8 bodies spill SIMD temporaries: the static raw bodies have
22 upstream versus 11 Rust xmm/rbp spill-reference lines. Their different scopes
mean these are not dynamic spill counts; excess spills alone cannot explain the
wasm gap. The current Rust vector-prefix algorithm has 16 shuffle/add prefix
stages for four planes, in addition to transposition, while upstream reconstructs
unrolled transposed records. More promising work is removing counted/scalar
islands and dispatch/validation repetition, preserving checks at complete windows.

There is **no wasm-bindgen layer** here: modules instantiate with empty imports
and export plain `bench_prepare`, `bench_run`, source/output pointers. Allocation
and memory growth are preparation/warmup concerns for these caller-buffer probes.
`memory-rust.json` records 1,179,648 bytes before/warmed/after; C++ grows once
from 131,072 to 196,608 during warmup, then stays at 196,608 across the loop.
Both public paths copy source into wasm and output back out; no worker messaging
is used. Historic and current raw counter gaps remain after eliminating these
copies, so JS transfer/wasm-bindgen/memory-growth fixes are rejected for the
large resident decoder gap. Tiny public wrappers may matter separately.

Second-epoch V8 sample attribution (wasm32.rs and vertex.rs hashes match;
subsequent mod.rs changes add native-only meshlet entry points/tests)
places 93.4% of vertex and 95.0% of NONE-view ticks in raw decode. The varied
Quat view is 67.6% raw, 26.3% filter. Current filtered means 1.615 / 1.778 and
worst 2.740 remain consistent with both raw and canonical filter arithmetic.
Historical filter macros also used scalar gather/scatter around vector float
math; current packed kernels remove that work, while exact rounding and tails
remain. Historical S5 2.457/2.769 and worst 7.281 must not be described as current.

## 6. Ranked implementation plan

Effect ranges are engineering targets grounded in instruction budgets or sample
shares, not promised time results. Use them to select a bounded first experiment;
accept only source-matched counters and the required parity/gates, then admitted
timing under the existing frozen cases. A fresh lane must preserve malformed
errors, caller-buffer tails, wrapping and arithmetic identity before claiming gain.

| Rank | Change and expected effect | Exact-parity / unsafe-audit risk | Effort and first acceptance experiment |
|---|---|---|---|
| 1 | Native full-16 reconstruction/scatter: separate complete groups from partial tails; explicitly unroll stores in checked spans and keep four transposed records in registers. Reconstruction is 60.54% of resident cycles; removing 1/3–1/2 of its work targets 20–30% whole-row time reduction (Amdahl), with 14.265 insn/B as starting point. | Medium: no padding writes outside caller destination; last/live record updates and all three channel forms must match scalar. Prefer safe fixed arrays, no new unsafe seam. Existing typed loads/stores stay inside audited module; Miri boundaries mandatory. | 2–4 days. Counter-first stride 12/32 and 17/16/block boundaries; compare emitted full-group stores, spills, exact outputs and malformed errors. |
| 2 | wasm whole-block bytes/reconstruction entry: dispatch once, unroll four validated groups, retain vector byte/halfword/XOR channels and complete-group output. Current raw 18.35 versus 5.78 insn/B gives 68% excess-work ceiling; target at least 20–30% fewer retired instructions before timing. | Medium/high: wasm path must retain strict 24/96-byte lookahead and exact scalar arithmetic. New ISA token calls only inside existing unsafe module; no new unchecked parser, no uninitialized tails. wasm execution and native/scalar reference proof required; host Miri does not prove wasm codegen. | 3–5 days. First resident v0/v1 stride12 counter/parity probe; inspect V8 raw body and dispatch calls; then short/filtered boundaries. |
| 3 | Small-input setup/tail specialization: full-group loop for live records plus bounded safe short path; avoid four partial-plane memcpy calls and spilled scatter. Tiny default/SSE2 samples spend 60–67% in deltas8; removing 20% of that cost targets 12–13% whole-row reduction. Do not hardcode a timing threshold from this diagnosis. | Low/medium if safe; scalar routing must actually use canonical short decoding, preserve cached repeated filters and exact caller tails. No new unsafe seam. Must test SSE2 hybrid and unsafe-free scalar separately. | 1–2 days after rank 1. Reuse exact current failed tiny keys and original micro meshlets; compare five or fewer admitted interleaved pairs only after parity/counters. |
| 4 | Exact filter instruction simplification: canonical sign/round/pack and exceptional-mask algebra with corrected fast cases; retain scalar fallback. Quat 106 vs 82 loop instructions and Color16 97 vs 65 leave 23–33% excess-work budgets, subject to exact-parity feasibility; eliminating half the excess targets 10–15% loop instruction reduction. Oct8 has an additional exact sqrt/div latency floor. | High arithmetic risk: nearest, reciprocal/rsqrt estimates or reassociation alone break canonical bytes. Require exhaustive/edge/seeded arithmetic parity at the existing scale. No additional unsafe module; keep Miri coverage and scalar reference. | 3–5 days. Quat/Color16 varied resident counters first; reject any proposal whose savings rely on adopting upstream tolerance. |
| 5 | Full four-triangle meshlet output and shared validated spans: combine two full pairs into exact 12-byte typed writes, hoist guards proven by bounded input/output, keep u32 counter check and separate tail. Typed 1.72x and raw 1.55x work ratios; reducing 15–20% kernel work targets 13–17% typed whole-row reduction using 84.86% attribution. | Medium: exact 12-byte writes only when four triangles live; no upstream padding writes into caller tails. Counter wrap/restart and malformed prefix/error behavior remain. Use existing typed seams; any new load/store needs its own audit block and Miri regression. | 2–3 days. Full pair/all pair codes, odd tails, both widths and counter wrap; inspect branches per pair and prove raw improvement separately. |
| 6 | Scalar sequence cursor fast path: prove bounded remaining input once, simplify position/overflow checks and single-byte path while retaining five-byte tail/error semantics. Six of 26 insn/index exceed C++; eliminating 2–4 targets 8–15% fewer instructions. This does not promise to clear the 2.888x allocating row. | Low/medium with safe slice windows; both versions, widths, truncated/overlong varints and wrapping baselines must match scalar. Keep this path unsafe-free; do not expand unsafe boundaries to make scalar faster. | 1–2 days. Resident + failed allocating stream counters/output/error proof; leave allocating timing cause open until admitted evidence. |

Rejected as primary fixes: per-call CPUID caching (already cached); removing
diagnostic ceiling TLS (<0.2% default instruction savings); JS copies/growth or
wasm-bindgen (raw gap persists, no bindgen); filter estimate/tolerance substitution
(wrong contract); private target directories, full-matrix retries or new ISA
families (not needed for these findings). Pure micro-optimizations with estimated
gain below roughly 5% are not advanced. The marginal Exp rows remain failures.

## Completion and limits

Items 1–5 are covered by the epoch reconciliation, per-path retired counts and
assembly comparisons, classified S3/wasm causes and ranked parity-aware plan.
There is no implementation or newly qualified performance claim. Confidence is
high in the structural excess work and rejected default-dispatch/boundary causes;
prospective time gains and the additional sequence allocating-max cause remain
unproved. Individual static site counts include other arms/tails and must not be
used as retired hot-path counts.

Reproduction commands and exact execution hashes live in the evidence root.
`cleanup.json` records deletion of the exact temporary Cargo target
`/mnt/linux-extra/moss-cargo-targets/codex-p07-diag`; retained evidence and scratch
binaries are outside it. Documentation checks verify all current summary ratios,
both-stage S3 counts, probe outputs and unchanged source/bar identity.

Out-of-scope followup: explaining the extra allocating sequence maximum requires
matched admitted timing/microarchitectural tracing beyond these structural
counter probes (2.888x retained time maximum versus 1.300x retired work, with
similar 7.49%/8.55% memset shares). New full-matrix, ARM/platform, exhaustive
arithmetic, Miri or release qualification belongs to the implementation lane,
not this diagnosis. No push, merge, rebase or bar/input change is authorized.
