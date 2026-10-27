# Candidate decisions and observable invariants

The attribution report was completed before library edits and is bound to the
release source hash. The rejected throttled profiles remain intact; only the
unthrottled replacement profiles inform the implementation.

Implementation follows attribution order:

1. Fuse the original-coordinate bounds reduction into the welding traversal.
   Per-axis comparisons see exactly the same values in the same order, including
   signed zeros. The n-vertex bounds work charge remains after classification,
   and only then are extents subtracted and overflow checked. A failed earlier
   classification therefore retains precedence. Empty bounds still yield zero
   extent. Reciprocal overflow retains the original first-normalized-vertex
   error position.
2. Initialize already-reserved remap/wedge/cache prefixes once, then use disjoint
   slices. Capacity, reservation order and Usage.bytes are unchanged. Failed
   local State is dropped; early initialization of its scratch is unobservable.
   Full 64-byte little-endian records dispatch once to a sequential reader;
   mapped, short-final and other layouts retain the original reader. Hash size,
   canonical negative-zero handling, probe order, representatives, wedge linking
   and probe/read charges are unchanged.
3. Normalize through a fixed-length destination iterator. The coordinate math,
   vertex/attribute visits, arithmetic guards and their positions are unchanged.
   Counted full-record validation keeps Work.scan's original prefix accounting.
   The existing full-record helper now exposes its already-valid ExactSizeIterator
   property internally; there is no public API or other-family implementation change.
4. A quadric is finite iff the maximum unsigned absolute bit pattern of its eleven
   checked fields is below 0x7f800000. This includes both signs, all subnormals,
   infinities and NaN payloads. The cached inverse is excluded exactly as before;
   its behavior and all caller error positions remain unchanged.
5. Traverse classification edge slices with shared vertex prefixes, preserving
   outer vertex order, edge order, early search stops and every work.add. The
   welded permissive path remains unchanged. Adjacency borrows its edge prefix
   once; its reset/count/prefix/write phases remain in place.
6. Positional-only quadric updates use common vertex prefixes; the attributed
   and volume path retains its prior loop. Every update guard remains after the
   same quadric write. Forward/back loops still update in-place in the original
   order. Triangle compaction copies one checked triangle and writes one checked
   triangle record; write<=read, and its three-unit charge precedes the same read
   and write. The unused destination tail is preserved.

No work charge, precharge bound, error position, fallible reservation, API,
unsafe boundary, scalar arithmetic order, sqrt backend or other-family algorithm
is changed. Existing release bounds and their checked low-budget fallback stay
unchanged. The exact release differential exercises all 512 masks, every small
work limit, cold/warm workspaces, both retention modes, worst-case hash/valence/
wedge meshes, finite overflow and signed/special operands. Added full/short/
unaligned byte layouts and every-byte output observations reach the new reader
and its fallbacks; all 512 masks are swept on those small layouts too. The widened
fuzzer chooses those layouts and runs for at least five child CPU minutes.

One full instruction campaign is planned after the complete proof. All 432
samples append and bind the final source/binaries. No implementation edits follow
its results except reversions; the two-campaign maximum and strict per-case
regression rejection remain unchanged. The final disposition is recorded in REPORT.md.

Final disposition: retain the exact-qualified candidate. One full campaign; all three real chains PARTIAL, no rejected case, PASS target unmet. No post-campaign code edits or threshold changes. Complete proof, gate calculation, evidence audit and target cleanup exit 0.
