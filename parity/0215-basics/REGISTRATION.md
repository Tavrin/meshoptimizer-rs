# Registration: 0.2.15 pipeline basics (remap family, index decode)

Fixed before any campaign result.

Scope (requester's ruling, narrower than SPEC-0215-S7): the remap family
(`generate_vertex_remap`, `generate_vertex_remap_multi`,
`generate_vertex_remap_custom`, `remap_vertex_buffer`, `remap_index_buffer`,
`generate_position_remap`, `generate_shadow_index_buffer`,
`generate_shadow_index_buffer_multi`) and `decode_index_buffer`. Vertex fetch
and index encode are out of scope: a SIMD-on screen already put the train
head below both C++ and optimesh there.

Arms: base (train/0.2.15 head f7d3620), candidate, cpp (C++ 1.3 through
meshopt 0.7.0, compiled with `MESHOPTIMIZER_NO_SIMD`), rust (optimesh 1.1.0
without its `simd` feature). Our arms build without the `simd` feature. The
remap and index paths have no SIMD code in any arm, so this only fixes the
comparison as scalar against scalar.

Consumer: `parity/peers`, adapted in scratch by
`parity/0215-contract-meshlets/prepare_driver.py` (counting only) and
`prepare_driver.py` here. Ops, all on 32-bit indices and the frozen
positions (12-byte records) plus the consumer's synthetic 16-byte attribute
stream:

| op | call |
|---|---|
| remap | indexed remap of positions |
| remap-multi | indexed remap of positions + attributes (two streams) |
| remap-custom | position remap with callback `abs(a.x - b.x) < 0.25` on attributes |
| position-remap | `generate_position_remap` |
| remap-vertex | `remap_vertex_buffer` of positions through a precomputed remap |
| remap-index | `remap_index_buffer` through the same precomputed remap |
| shadow | shadow indices over positions |
| shadow-multi | shadow indices over positions + attributes |
| index-decode | `decode_index_buffer`, 4-byte output |

The precomputed remap (first reference of each position) is built outside
the counted region, identically for every arm. Each arm allocates its output
inside the counted call, as the consumer always has.

Workloads: the frozen five-mesh corpus and the three real meshes
(`inputs.json`, same fixtures as the toward-C++ lane). Metric: median of three
`instructions:u` samples per cell, 100% counter running, CPU 9, interleaved arm
order (reversed on the second repetition). No wall timings.

Verdict per function (spec gate, verbatim): SHIPS if candidate/base <= 0.97
on the three-real-mesh geometric mean and no case above 1.01;
candidate/optimesh and candidate/C++ are reported. NO-GAIN otherwise; any
change behind a NO-GAIN or a case above 1.01 is reverted. Candidate and base
outputs must be byte-identical in every cell. No code edits after a
campaign's results except reverts; at most two campaigns.
