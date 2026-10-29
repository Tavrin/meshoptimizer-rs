# Vertex decoder identity and isolation

The only arithmetic change is to instantiate the existing x86 vertex kernel
with FIXED=12 when the validated runtime stride equals 12. Only the owned Vec destination reaches the new selector. Both SCRATCH=128
(count <=32) and SCRATCH=1024 (larger counts) retain the same selection rule.
Substituting 12 for a runtime value already known to equal 12 changes no
arithmetic, slice boundary, record address, parser decision or stored byte.
The original generic dispatcher remains verbatim for slice/staged destinations
and fallback owned strides. Versions,
selectors, widths, channel order, rotations, predictors and scalar tails all
use the original implementation. No additional ISA instruction family is introduced. One new token-authorized
call uses the existing checked kernel; it adds no raw-memory operation.

All complete plane groups are decoded before a channel writes. This ordering,
including malformed later channels and final-consumption errors after decoded
blocks, remains unchanged. Each destination monomorph keeps its checked block
slice, initialized bytes, finish_block call and untouched caller-buffer tail.
The same four-byte scatter and modular prefix arithmetic therefore produce
the same live bytes and failure prefixes as the scalar reference.

Validation, allocation, output-capacity accounting, work precharges and error
precedence remain outside the specialized kernel and are unchanged. The new owned entry and its no-inline boundary do not allocate, fail, meter
work or change state. Consequently every work/byte limit reaches the same preceding failure
or proceeds with identical output and Usage, including reused workspaces after
success and failure. Finite tests supplement this structural argument; they
do not exhaust all possible streams or allocator outcomes.

The first experiment put a no-inline boundary around the complete generic
vertex::decode. It passed exact proofs and saved vertex instructions, but
changed raw's register allocation: one real index case regressed to 1.024898x.
That placement is rejected. The second experiment guarded stride-12 cases by
an owned-only associated constant. Despite constant pruning, it still changed
the slice monomorph's codegen and reproduced the 1.014631x corpus index
regression. Both complete measurement matrices remain retained.

The final design leaves the generic Destination trait, vertex::decode and
x86::vertex implementations verbatim. Only the owned allocator call selects a
new non-inlined vertex::decode_owned entry. It tries a separate SSSE3 stride-12
selector, then falls back to the original generic decoder. The original
slice decoder remains in raw, alongside triangle decoding, with no new cases
or conditions in its source. The no-SIMD and non-x86 allocator calls retain
their original expressions. The staged stride-four destination is unchanged.

The new private target-feature call receives the dispatcher's Ssse3 token.
It selects exactly the existing kernel at stride 12, using the same count
threshold and validated Vec destination. Initialized scratch and checked
windows are unchanged. The unsafe inventory is updated independently; the
unsafe-free build omits the entire added path.

Qualification results, codegen comparisons and the final gate verdict are
recorded after the registered campaigns complete. All source hashes and
individual exits are retained; proof success alone does not admit performance.

## Final evidence

Verdict: **SHIPS**. Final real-mesh vertex instructions are 0.910983x v0.2.15; index instructions are 1.000000x, with worst case 1.000000x. The worst other-function case is 1.000185x. The complete matrix contains 1872 samples across all 19 release operations. See REPORT.md and TABLE.md.

The shared raw dispatcher has the same 1,734 relocation-normalized instructions as v0.2.15 in the decoder consumer. Raw disassembly and static-data relocation differences are retained alongside the instruction-only comparison. Register allocation, local branches, immediates and stack layout are retained by the comparison; binary byte identity is not claimed. The generic/slice dispatch source is verbatim.

Exact comparison passed 5,114,700 observations per feature mode. Native ISA/C++ differential passed 318,608 cases per std/no-std mode; Rust scalar/full-buffer identity and C++ success bytes/statuses agree. The 74,863 inherited C++ malformed-prefix differences per mode remain explicitly recorded. Final proof steps: 21, all exit 0. Total proof CPU including the first rejected experiment: 1300.64 seconds. Miri, 20,000 ASan runs, package, unsafe-free and public-text checks pass. Unsafe inventory: 29 blocks, one allowance.
