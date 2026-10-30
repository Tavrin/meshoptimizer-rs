# Vertex-only x86 tuning registration

Base: v0.2.15, `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
Starting integration: `91969326e363f6b311cffcda6c06a1a36da572a6`.
C++ oracle: 1.3 at `4c203430ca565cb59a468a91922c76c208169536`.

Only SSSE3 stride-12 specializations and vertex-only dispatch/inlining
boundaries are eligible. No index implementation, validation, metering or output order edits.
Initial screening uses the starting integration as control, an unisolated
stride-12 arm, and a generic no-inline boundary arm. The second experiment
uses an owned-only associated constant. The final experiment separates owned
dispatch entirely. Each source change precedes a fresh complete decoder
matrix. The broad guard additionally uses a control with the final owned
entry retained but its stride-12 fast path disabled; that control is measured
only for vertex/index decoding. The four required arms cover all 19 release
operations.

All Rust measurement arms compile at one absolute source path, with identical
package identity and consumer. Every source and manifest is touched before
building, and each log must show library recompilation. Frozen sources and
binary hashes bind each arm. Default std/SIMD features are used without
parity-internals. The consumer also enables clusterlod identically in all
Rust arms. Scalar C++ defines MESHOPTIMIZER_NO_SIMD; default C++ uses upstream
SSSE3 dispatch. Compiler options and allocation policy are identical to the
previous decoder campaign. Inputs are its eight packed XYZ/index fixtures:
five generated cases and three real meshes, with pre-encoded bytes shared
by every arm.

Performance gate: instructions:u, median of three interleaved samples pinned
to one CPU; reverse arm order on the middle repetition. Resident setup,
encoding, serialization and warmup remain outside counters; counted calls
include output allocation and destruction. Require 100% counter running.
SHIPS requires real-mesh vertex candidate/base geometric mean <=0.95,
index <=1.005 with no index case >1.01, and no other function >1.01.
Per-case tables and corpus means are disclosed; wall time is disclosure only.
The initial screening isolates decoder effects; a broader consumer guard is
required before claiming SHIPS. Rejected experiments remain retained.

Proof: exact base/candidate owned and caller-buffer results, Debug-encoded
typed errors including payloads, complete Usage and whole sentinel buffers.
Enumerate every small-fixture integer work limit through success+1 and byte
limits at output boundaries, in cold, warm and after-failure workspaces,
with default and no-default features. Test both scratch-size branches and
all group/block tails, widths 0/1/2/4/8, v0/v1, controls, channels, rotations,
reserved channels and seeded malformed cases. Forced scalar/SSE2/SSSE3/SSE4.1
and pinned C++ scalar/default differentials supplement the structural proof.
C++ malformed failure writes may differ from the inherited Rust contract;
record those differences separately and require success bytes and statuses.
Targeted proof total must stay within 7,200 CPU seconds.

Reproduction: supply CARGO_TARGET_DIR, DECODE_ART, MESHOPT_REFERENCE and
PEERS_PERF. Run build.py, measure.py and results.py for decoder screening, then proof.py
with DECODE_SCREEN_DIR identifying that screen. A passing screen also runs
guard_build.py and the complete 19-operation matrix. GUARD_DRIVER supplies the
retained e3ce993b... consumer seed used by prepare_guard.py. Run codegen.py and
normalize_codegen.py to retain relocation-normalized raw comparisons.
No integration or publishing commands are part of these drivers. Retain
receipts and source/binary bindings before removing the dedicated target.
