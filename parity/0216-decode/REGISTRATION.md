# Decoder qualification registration

Base: v0.2.15, commit `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
C++: 1.3, reference commit and source hashes in `cpp-binding.json`.
The native C++ consumer uses meshopt 0.7.0, whose vertex/index codec sources
are identical to that reference. Default C++ enables upstream runtime SSSE3;
scalar C++ defines `MESHOPTIMIZER_NO_SIMD`. Rust uses the default std/SIMD features; the existing release consumer also
enables clusterlod for its other compiled dispatch arms, identically for both
Rust versions. The measured operations are vertex and index decode only.

Measure vertex and triangle-index allocating decoders on the eight retained
release inputs: five generated meshes and three real meshes. Packed XYZ is
stride 12; triangle indices use stride 4. All four native arms use the same
pre-encoded fixture, resident warmup, allocation contract, interleaving and
three repetitions. Setup, encoding and output serialization are outside the
instruction counter. Native `instructions:u`, median of three, is the gate;
wall time is disclosure only. The native consumer uses release opt-level 3,
thin LTO and one codegen unit, identically for base and candidate.

Tuning requires candidate/base <=0.97 on vertex decode, with no measured
vertex **or index** case above 1.01. Apply the real-mesh geometric mean and
also disclose the corpus mean and every cell. Rejected campaigns are retained;
no thresholds change after measurement. Each subsequent campaign changes the
code before taking a fresh complete four-arm matrix.

Node executes separate scalar and simd128 Rust artifacts against the pinned
upstream shipped C++ SIMD wasm decoder. Compare decoded bytes before timing;
use three rotated arm orders. These wall-time ratios disclose wasm behavior
and do not establish a native instruction-count gain. A local fresh C++ wasm
compiler is unavailable; the retained upstream wasm artifact is used instead.

NEON ships on native identity, not a local performance claim. The local phase
cross-compiles AArch64 and wires forced-level tests/C++ differentials into both
arm64 CI jobs. The coordinator owns pushing and reading those CI results.
No Git integration or publishing occurs in this lane.

Proof matrix: every small-fixture work limit from zero through success + 1,
byte limits around the output footprint, allocating/caller-buffer APIs,
cold/warm/after-failure workspaces, both default and no-default features.
Forced ISA comparisons cover all remainders modulo 16 around group and block
boundaries, v0/v1, widths 0/1/2/4/8, controls 0..3, channels 0/1/2 and reserved
3, and all 16 rotation nibbles. Compare errors, all destination bytes,
untouched tails and Usage. Add seeded larger mutations, native C++ scalar
and default comparisons, executed wasm, a supported x86 Miri subset and a
fixed 20,000-run ASan fuzz budget. The targeted proof budget is <=2 CPU-hours.

Canonical-path control: after three rejected prototypes, rebuild base, a
NEON-only control with unchanged x86 source, and the stride-12 candidate at
one identical absolute build path. Take a fresh complete five-arm matrix
including both C++ arms, still median of three. The control must also stay
within the unchanged 1.01 guard. This isolates source-path/package-identity
code-generation effects; prior receipts remain retained and rejected.
