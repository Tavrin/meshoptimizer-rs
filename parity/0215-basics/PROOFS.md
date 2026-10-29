# Proofs: 0.2.15 basics lane (remap family)

All changes are in `src/remap.rs`, safe scalar code: no `unsafe`, no
`get_unchecked`, no SIMD, no new panicking path. Observables that must stay
byte-identical to the train head: outputs, errors and their precedence,
`Usage` (work and bytes), and caller buffers (partial writes, untouched
tails). `exact.rs` checks all of them, base against candidate, at every work
limit from 0 to the success charge + 1 and at byte limits around the success
footprint, cold and warm, in both feature modes, plus 3000 random larger
meshes per mode (`proof.sh`).

## 1. `remap_required` (remap_vertex_buffer, remap_vertex_buffer_into)

Before: a per-entry scan charging one unit per entry, failing with
`IndexOutOfBounds` at the first mapped entry `i >= count`, and tracking
`required = max(i + 1)` over mapped entries.

After: when the whole scan is covered, one reduction
`top = max(i.wrapping_add(1))`. `NONE + 1` wraps to 0, so unmapped entries
never raise it; a mapped entry gives `i + 1 >= 1`. `VertexStream::new`
bounds `count <= u32::MAX`, and `i + 1` does not wrap for any mapped `i`, so
`top <= count` holds exactly when every mapped `i < count`, and then `top`
equals the old `required`. That case charges `remap.len()`, as the covered
scan did on success. Any other case (limited work, or `top > count`) runs the
unchanged scan, which reproduces the old charge and error exactly; the
reduction itself has no side effect.

## 2. `remap_vertex_packed` (record sizes 4, 8, 12, 16)

Applies only when `stride == size` and `offset == 0` and the full copy is
covered (`precharge` of `remap.len()` units, the same call and charge as
before; a failed `precharge` changes nothing, so the fallback to the old
`remap_vertex_copy` is exact). Source record `i` is then
`data[i*size .. i*size + size]`, which is chunk `i` of `data.as_chunks()`;
the remap length equals the record count, and the stream was validated to
hold every record, so the zip visits the same `(i, remap[i])` pairs in the
same order. Each mapped entry overwrites destination record `remap[i]`
(chunk `remap[i]` of `destination.as_chunks_mut()`), so repeated targets
still keep the last source record and unmapped records stay untouched. The
destination holds at least `required * size` bytes, so every index is in
bounds (a bounds check remains; it cannot fail).

## 3. `remap_index_copy` (remap_index_buffer only)

Used only by the allocating `remap_index_buffer`, whose output is dropped
on any error, so only the result and `Usage` are observable. Old covered
path: copy with an exit at the first unmapped value, charging `at + 1`
there or `count` on success. New covered path: copy everything (indexed:
four lookups per step plus the tail; unindexed: the remap prefix
`remap[..count]`, which is what `remap[i]` for `i < count` copied), then an
OR-reduction over the output detects any unmapped value, and only then
`position` finds the first one and charges `at + 1`. Same result, same
charge. All indices were validated before (unchanged `validate_indices`),
so the extra lookups past the first unmapped value cannot panic. The
limited path is the old per-index loop with the index source spelled
`input.map_or(i, |v| v[i])`.

## Reverted after campaign 1

Campaign 1 also carried `hash_update` over `as_chunks::<4>()` and a
word-wise multi-stream key compare (`key_equal`, no `bcmp` call). Both were
proven exact by the same driver, and they made remap-multi 0.963x and
shadow-multi 0.958x of base on the real meshes, but the shared
`shadow_streams` codegen moved single-stream shadow to 1.029x. The
registration reverts the change behind any case above 1.01, and only
reverts are allowed after a campaign, so both were reverted and campaign 2
measured the final state.

## Not changed

`decode_index_buffer`: attributed, not changed (below). Vertex fetch and
index encode are out of this lane's scope.

## Verdicts (campaign 2, final source, 864 samples, exit 0)

Three-real-mesh geometric means; `campaign-2/results.json` has every cell.

| function (op) | cand/base | cand/C++ | cand/optimesh | worst case vs base | verdict |
|---|---|---|---|---|---|
| `remap_vertex_buffer` (remap-vertex) | 0.465 | 1.394 | 0.453 | 0.683 | SHIPS |
| `remap_index_buffer` (remap-index) | 0.772 | 1.625 | 0.975 | 0.866 | SHIPS |
| `generate_vertex_remap` (remap) | 1.000 | 0.696 | 0.576 | 1.000 | NO-GAIN |
| `generate_vertex_remap_multi` (remap-multi) | 1.000 | 1.031 | 0.934 | 1.000 | NO-GAIN |
| `generate_vertex_remap_custom` (remap-custom) | 1.000 | 0.979 | 0.730 | 1.000 | NO-GAIN |
| `generate_position_remap` (position-remap) | 1.000 | 0.865 | 0.623 | 1.000 | NO-GAIN |
| `generate_shadow_index_buffer` (shadow) | 1.000 | 0.729 | 0.626 | 1.000 | NO-GAIN |
| `generate_shadow_index_buffer_multi` (shadow-multi) | 1.000 | 1.083 | 1.003 | 1.000 | NO-GAIN |
| `decode_index_buffer` (index-decode) | 1.000 | 0.994 | 0.907 | 1.000 | NO-GAIN |

Candidate and base outputs are byte-identical in every cell, and every
candidate output equals the C++ output (the C++ differential: eight
fixtures, nine ops). Corpus ratios against base: remap-vertex 0.47-0.68,
remap-index 0.77-0.87, every other op 1.000.

## Attribution

- `remap_vertex_buffer` was 3.0x C++: a per-entry validation scan
  (unmapped test, bound test, running maximum, error exit) plus a copy
  that rebuilt both record ranges with bounds checks. Still 1.39x C++
  after section 1 and 2: the destination bound check that safe code keeps
  and the zeroed allocation.
- `remap_index_buffer` was 2.1x C++: a per-index exit on unmapped values
  and one lookup per step. Still 1.63x C++ after section 3: the up-front
  `Work::indices` validation (an unsigned maximum that baseline x86-64
  vectorizes poorly) and the per-lookup bound check.
- remap-multi and shadow-multi (1.03x and 1.08x C++): see "Reverted after
  campaign 1". Outlining the insertion path (the dormant `OUTLINE`
  parameter of `generate_kernel`) and fixed-width key dispatch per stream
  were screened and regressed (up to 1.49x and 1.08x of base), so they
  never entered a campaign.
- `decode_index_buffer` is at parity on the real-mesh mean (0.994x C++):
  real-214e and real-b6fd are 0.92x, real-a082 1.15x. a082 is not
  cache-ordered: 60% of its triangles use the 0xfe/0xff explicit-index
  codes and 35% a free-index varint (4.4 bytes per triangle, against 1.0
  for the other two). Our varint reader keeps a five-byte checked
  lookahead per value and spills the last free index to the stack; C++
  reads unchecked. The checks cannot fail inside a triangle that starts in
  the safe region, but removing them needs a window-based reader worth a
  few percent on a082 only; not changed.
- `generate_vertex_remap`, `generate_position_remap`, single-stream shadow
  and the custom remap already lead C++ on the real meshes (0.70-0.98x).

## Proof runs (final source)

`proof.sh`: every step exit 0 (`proof-exits.tsv`): tests with all features
(277 passed) and without default features (231 passed), clippy
`-D warnings`, fmt, unsafe boundary, public text; the exact driver in std
and no-std builds, 3840 generated cases and 1,893,337 comparisons each, 0
differences; the random sweep, about 36,000 meshes and 5.2 million
comparisons per mode, 0 differences (`proof-results.txt`).

Not measured: 16-bit index outputs, strided (non-packed) streams in the
campaign (the exact driver covers them), wasm, and the `_into` vertex
variant (it shares the new validation and copy).
