# 0.2.16 remap scalar campaign

Baseline: v0.2.15, commit `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
Only `remap_index_buffer` and the packed vertex-copy helper may change.
No hash, remap-generation, explicit SIMD, or Git mutations.

Four fresh arms: baseline Rust, candidate Rust, scalar C++ 1.3 and default
C++ 1.3 (SIMD enabled). Rust uses std without the simd feature. The C++
oracle is meshopt 0.7.0; its bundled core matches the pinned reference
`4c203430ca565cb59a468a91922c76c208169536`. Both targeted C++ kernels are
scalar in either build. Release opt-level 3, thin LTO, one codegen unit;
no native-CPU flags. Every arm uses the same adapted peer consumer.

Inputs: the unchanged eight fixtures in `../0215-basics/inputs.json`:
five corpus cases and three real meshes. XYZ records are 12 bytes; the
remap is generated before counting, in first-reference order. Allocation,
copy and output drop are counted. Setup and output serialization are not.

Metric: instructions:u on one pinned CPU, median of three resident-loop
samples per cell. Interleave the four arms and reverse the order in the
second repetition. Require 100% counter running and identical serialized
outputs across all arms. Record wall seconds from the same loop, for
disclosure only. Preserve all completed samples and source/binary hashes.

Per-function gate: candidate/baseline real-mesh geometric mean <= 0.95,
and no real or corpus case > 1.01. Otherwise NO-GAIN: restore that
function's baseline source and retain its rejected campaign. Measure the
final source again after any revert. Report candidate/scalar-C++ and
candidate/default-C++ independently.

Proof: compare baseline/candidate results, exact errors, Usage and complete
caller buffers at every small-case work limit, crossed with byte boundaries;
cold, repeated, retained-scratch and post-failure workspaces. Run in default
and no-default library feature modes, plus deterministic random larger cases
and valid-input C++ differentials. Targeted fixtures bound the proof work;
finite tests supplement the structural argument in PROOFS.md.
