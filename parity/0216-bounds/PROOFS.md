# P3 bounds: final proof and experiment record

Cluster bounds and meshlet bounds pass the instruction gate. Meshlet builders,
flex builders and cluster LOD are **NO-GAIN**. Only packed position selection
and exact independent bounds roots remain. Both KD batching prototypes were
rejected and removed. No Git state mutation was made.

## Pins and workload

Initial worktree: `0454a96bef54f6fedf496ff6aa3862e7efbb7257`.
Baseline: v0.2.15, `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
The initial train already contains the remap lane; this lane does not edit it.
C++ reference: 1.3, `4c203430ca565cb59a468a91922c76c208169536`.
The 22 vendored C++ source/header files used by the timing consumer were
verified identical to this reference; see `cpp-source-binding.json`.

Rust 1.98.1 / LLVM 22.1.8 and C++ 13.3.0; no additional native Rust target or
floating-point flags. The timing consumer uses thin LTO, one codegen unit and
release opt-level 3. Default means std + SIMD; timing scalar means std with
SIMD disabled. C++ scalar uses `MESHOPTIMIZER_NO_SIMD`; default retains upstream
SIMD. Toolchain flags and CPU affinity are retained in `environment.json`.

The eight frozen inputs contain three real meshes and five corpus meshes
(tiny, medium smooth/seams/sparse, and one-million-triangle smooth).
`prepare.rs` builds baseline meshlets once (64 vertices, 124 triangles maximum)
and freezes their vertex references and byte triangles. Bounds-query inputs
are localized outside the counted region and identical in every arm.
`inputs.json` and `bound-inputs.json` bind their bytes. All meaningful bounds
fields are serialized by float bits and quantized integers. Builder outputs
and complete LOD callback/position records are also compared. Resident loops
exclude fixture preparation and serialization; returned-output construction
and collection remain included for every arm.

## Profile before editing

Instruction sampling was enabled only around the resident loop. Raw data,
symbol reports and source-line reports remain under `$BOUNDS_SCRATCH/profile`.
Profiles cover medium-smooth and real-b6fd.

| Real-b6fd workload | Sampled exclusive symbol | Share |
|---|---|---:|
| meshlets | Covered::nearest | 61.48% |
| meshlets | flex | 23.53% |
| meshlets | kd_build | 13.01% |
| cluster LOD | Covered::nearest | 21.57% |
| cluster LOD | bounds (128-triangle specialization) | 2.16% |
| cluster bounds | bounds (32-triangle specialization, general Positions) | 78.52% |
| cluster bounds | position validation | 6.46% |
| cluster bounds | cluster_bounds entry/uniquing | 6.25% |
| cluster bounds | libm sqrtf | 3.98% |

The meshlet profile has approximately 68,000 samples; cluster bounds about
6,500. Medium bounds profiles have only a few dozen samples and are qualitative.
Inlining and sampling skid limit source-line attribution. These shares locate
costs; they do not isolate the savings from a particular edit.

## Retained implementation and identity argument

`src/codec/simd/bounds.rs:13` extends the existing private audited boundary
with SSE2, AArch64 NEON and wasm simd128 roots. Existing Baseline tokens authorize
calls. No capability ordering, detector, public API or ISA token changes.
Each backend loads and stores two initialized four-float windows; there are
no position overreads, aliasing conversions or uninitialized lanes. Typed
input/output arrays bound every memory access. Unsupported targets and scalar
diagnostic ceilings retain the pinned backend.

Eight independent roots use hardware only when every unsigned operand bit
pattern is at most `f32::MAX.to_bits()`. This includes positive zero and
subnormals. Negative zero, negatives, infinities and NaNs retain the original
backend. Hardware roots use correctly rounded IEEE sqrt under the existing
round-to-nearest, gradual-underflow execution contract. No approximate sqrt,
reciprocal, FMA, reassociation or cross-lane reduction is introduced. Existing
sqrt qualification is recorded in `parity/SIMD_RESULTS.md` and
`parity/DECISIONS.md`; the new focused test also checks edge bit patterns and
32,768 randomized positive operands per supported forced ISA level.

`src/meshlet_util.rs:9` selects these roots and falls back to the old `sqrt8`.
All expressions producing the operands retain their original order. Triangle
normalization, cone math, sphere expansion and strict comparisons retain their
original sequence. The Context root memo is unchanged. KD traversal, candidate
processing and tie decisions are byte-for-byte unchanged in `src/meshlet.rs`.

Cluster and uncached meshlet bounds now reuse the existing packed-source
helper (`src/meshlet_util.rs:589`), previously used by cached meshlet bounds.
Selection follows the same validation, topology checks, uniquing/expansion and
work charges. Safe packed indexing reads identical coordinate bits; strided
layouts retain general checked Positions access. Moderate/checked arithmetic
selection is unchanged. No allocation, output capacity, workspace accounting,
fallible read or caller write moves across a failure boundary.

The unsafe inventory grows from **23 to 30 blocks (+7)**: one token-authorized
call and two fixed-array memory operations per architecture. The single module
allowance is unchanged. `SAFETY.md` is extended additively; repository and
published-package boundary checks both pass. No spatial, simplify or partition
implementation is edited.

## Measurement and attribution

The complete final campaign is six arms, eight inputs, five operations and
three interleaved repetitions: **720 samples**, all with 100% counter running
and identical outputs across every arm. Raw perf receipts are retained outside
the deleted target. `gate.py` independently checks raw counts, repetitions,
output identity and binary hashes. `bindings/` binds all six binaries to their
frozen source trees. Current library sources, Cargo files and safety inventory
were rechecked against both measured candidate3 arms after the full proof.

The gate uses median `instructions:u`, real-mesh geometric mean <= 0.95, and
no individual case > 1.01. Ratios below are candidate/reference; lower is better.

| Function | / v0.2.15 default real/corpus | Scalar / scalar C++ real/corpus | Default / default C++ real/corpus | Worst / base | Verdict |
|---|---:|---:|---:|---:|---|
| meshlets | 1.0000/1.0000 | 0.9727/1.0196 | 0.9727/1.0195 | 1.0001 | NO-GAIN |
| meshlets-flex | 1.0000/1.0000 | 0.9733/1.0197 | 0.9733/1.0196 | 1.0000 | NO-GAIN |
| cluster-lod | 0.9987/0.9933 | 1.1376/1.1135 | 1.1364/1.1090 | 0.9987 | NO-GAIN |
| cluster-bounds | 0.9316/0.8890 | 1.1322/1.1273 | 1.0749/1.0549 | 0.9323 | SHIPS |
| meshlet-bounds | 0.9327/0.8920 | 1.1437/1.1505 | 1.0868/1.0786 | 0.9333 | SHIPS |

`TABLE.md` discloses every case and median wall seconds. Shared-machine wall
times are disclosure only, not qualification or a wall-speed claim.
`ATTRIBUTION.md` separates packed access from SIMD: packed access reduces real
scalar instructions by 1.87% / 1.84%, and roots reduce the resulting scalar
candidate by 5.06% / 4.98%, for cluster / meshlet bounds respectively.

Builders remain NO-GAIN because the attempted KD batching increases gather,
mask, call and stack traffic around short leaves; existing default scalar
arithmetic already uses native roots and compiler vectorization. The inlined
prototype enlarges recursive nearest code from 550 to 1,794 bytes and its
explicit stack reservation from 16 to 248 bytes. This assembly observation
supports an explanation, not an isolated causal measurement. Its real ratios
are 1.1246 / 1.1246 / 1.0400 for builders / flex / LOD. Isolating the helper
worsens these to 1.1564 / 1.1564 / 1.0510. Both complete rejected campaigns and
bindings remain in external evidence; their counts and gates are under
`experiments/`. Cluster LOD remains NO-GAIN because bounds occupied only about
2% of its sampled instructions; its final real improvement is 0.13%, below 5%.

## Behavioral and safety evidence

The final fresh full proof is `proof.5GfyCo`, recorded in `proofs/summary.json`.
`proofs/source-binding.json` binds 97 implementation, test and recipe files;
the binding is identical at start, end and final verification.

| Check | Result | Exit |
|---|---|---:|
| All features, including doctests | 297 passed, 8 ignored, 0 failed | 0 |
| No default features, including doctests | 250 passed, 8 ignored, 0 failed | 0 |
| Exact base/candidate, std + SIMD | 228,760 comparisons, no differences | 0 |
| Exact base/candidate, no_std without SIMD | 228,760 comparisons, no differences | 0 |
| Larger geometry/KD/configuration matrix, each mode | 1,020,255 comparisons each, no differences | 0 / 0 |
| Pinned C++ scalar/default and Rust native/wasm differential | 3,608 cases, all seven clients identical | 0 |
| All-target/all-feature native Clippy, warnings denied | pass | 0 |
| Formatting, repository and package unsafe boundary | pass | 0 / 0 |
| Unsafe-free no-default build | pass | 0 |
| Rust 1.88 focused roots / no-default check | pass | 0 / 0 |
| Nightly Miri / x86_64 ASan focused roots | pass | 0 / 0 |
| AArch64 no_std + simd + clusterlod release compile | pass; execution unavailable | 0 |
| Public text | pass, repeated after final reporting | 0 |

The new focused layout test covers packed/interleaved cluster and meshlet
bounds, scalar extremes and invalid floats, every observed work limit, and
exact bounds bits plus Usage. `exact.rs` visits every small-case work limit
through observed success + 1, byte limits around allocation boundaries, cold,
repeated-query, after-short-output-failure and after-other-family workspaces.
It compares errors, full caller buffers including sentinels, owned outputs,
work/storage Usage, all meaningful bounds bits, LOD callback records and
mutated position bits. Cases include degeneracy, signed zero, tiny/large
coordinates, NaN/overflow, malformed topology, invalid indices and empty input.
Larger cases cross 1/8-lane and 32/128/512-triangle stack buckets and the
256-local-vertex boundary. The existing larger geometry driver additionally
covers larger KD trees, configurations and failure prefixes in both modes.

The C++ differential compiles directly from the pinned checkout. Its seven
clients are C++ scalar/default, native Rust scalar/default/no_std-SIMD, and
separate scalar/SIMD wasm artifacts executed in Node. There are 512 deterministic
seeds per operation (owned and caller-output builders each), plus 24 LOD cases.
SIMD wasm explicitly uses `-Ctarget-feature=+simd128` with inherited encoded
Rust flags cleared. `proofs/cpp-wasm.json` binds binaries and the complete corpus.
Native NEON execution is unavailable on this x86 host; successful compilation
does not establish NEON runtime qualification. Cross-build logs retain the wasm
warnings (a statically redundant unsafe call and existing unused helpers).

Timed final core proof used 234.60 CPU seconds; MSRV/Miri/ASan supplement used
84.68 CPU seconds. These receipts exclude the separately run C++/wasm build
stage. Targeted checks remain bounded; no allocation-failure injection or
exhaustive proof of all floating-point inputs or UB absence is claimed.

## Retained failures and handoff

The first candidate build passed focused tests and Clippy, but screen launch
exited 1 before any samples because `PEERS_PERF` was omitted. The environment
was corrected and the entire campaign completed. The first full proof attempt
passed preliminary checks but exited 101 building its baseline copy because
`docs/USING.md` was omitted by the harness. The baseline copy recipe was fixed;
a fresh complete proof reran every step successfully. Both failed receipts
remain under `experiments/`; neither was replaced by a focused retry.

Raw logs, perf data, inputs, frozen source snapshots, binary hashes, executable
drivers, differential corpus and rejected attempts remain in external
`$BOUNDS_SCRATCH` evidence. Public receipts here omit private machine paths.
The dedicated Cargo target is removed after all checks; see `cleanup.json`.
Branch and HEAD remain at their initial values; edits are uncommitted for the
coordinator to review and integrate.
