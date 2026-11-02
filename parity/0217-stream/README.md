# 0217-F1: streaming vertex and triangle decoders

`codec::VertexBufferDecoder` and `codec::IndexBufferDecoder<'_>` decode the
vertex buffer and triangle index buffer formats incrementally. Neither exists in
C++ meshoptimizer. They share `Status` and `Progress` with `IndexSequenceDecoder`
(`Progress::written` counts bytes for vertices, indices for triangles).

## Trailer rule

Both formats place data the decoder needs first at the end of the stream, so a
forward-only reader cannot start. The decoders therefore take the trailing data
as a separate argument and stream only the body between.

- Vertex: `new(count, stride, header, trailer, body_len, limits)`. `header` is
  the first byte; `trailer` is the whole padded tail, `max(stride, 32)` bytes
  for version 0 and `max(stride + stride / 4, 24)` for version 1; `push` takes
  exactly `body_len` bytes between them.
- Triangles: `new(index_count, prefix, trailer, body_len, limits)`. `prefix` is
  the header byte plus one code byte per triangle (borrowed for the decoder's
  lifetime, the format's separate code stream), `trailer` the 16-byte codebook.

The container must supply the lengths and the trailer (seekable source or a
container that stores them separately). Wrong lengths give `InvalidStream`.

## Memory and work

Fixed storage, no heap: a 9216-byte input window (largest compressed vertex
block plus the largest trailer), an 8192-byte decoded block and at most 640
bytes of predictor state for vertices; triangle FIFOs and one pending triangle
for indices. Independent of stream length. Work is preflighted once as encoded
plus decoded bytes, as the one-shot caller-buffer decoders do; fixed storage is
excluded from `max_bytes`. An incomplete block is re-parsed when more input
arrives, so very small chunks cost more than bulk feeding. Output is
provisional until `finish` succeeds; errors are sticky and equal the one-shot
decoders' errors on the same bytes.

## Results (final tree)

| check | result |
| --- | --- |
| identity, `tests/buffer_stream.rs` (random round trips v0/v1, malformed prefixes, 2000 random streams, long bodies cycling the window, empty-output, sticky errors, u16 narrowing, every available SIMD level) | 7 passed, `tests.log` |
| upstream corpus, chunks 1/7/4096/65536 vs one-shot (native fixtures + JS fixtures) | 916 comparisons identical, `corpus-wasm.log` |
| wasm32 (node), same identity tests including malformed inputs and SIMD | pass, `corpus-wasm.log` |
| fuzz target `buffer_stream` (arbitrary bytes, count, stride, chunk and output widths; streaming result must equal one-shot `Result`) | 1,997,788 runs, 614 CPU-seconds, 0 mismatches, exit 0 (`fuzz-run-0.log`, `fuzz.json`) |

## Throughput (release, 65,536 vertices of stride 32; 255x255 quad grid, 390,150 indices; MB/s of decoded output, `benchmark.log`)

| version | one-shot vertex | stream 4 KiB | stream 64 KiB | one-shot triangle | stream 4 KiB | stream 64 KiB |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 5700 | 3164 (0.56x) | 3957 (0.69x) | 5197 | 1232 (0.24x) | 1277 (0.25x) |
| 1 | 9809 | 1531 (0.16x) | 1830 (0.19x) | 5352 | 1278 (0.24x) | 1281 (0.24x) |

Streaming is slower than one-shot: the bounded block parser runs the generic
per-channel decode instead of the one-shot SIMD vertex path, and triangle decoding
makes one call per triangle. It is a memory-bounded interface, not a faster one.
The machine was shared with other jobs; read these as ratios, not absolutes.

## Decisions

- Kept the earlier design (trailer-first two-part feed, fixed 9216-byte window).
- Input window now advances a cursor instead of shifting bytes per consumed
  block; consumed bytes are compacted only when space runs out. The first version
  moved up to 9 KiB per triangle (about 0.04x); fixed after measuring.
- `TriangleState::one` no longer clones all FIFOs per triangle; every fallible
  step precedes the first FIFO write, so an error leaves state untouched.
- `Status` and `Progress` docs were generalised (no signature change); semver
  check against 0.2.16 passes.
- Not done: a SIMD streaming vertex path (would close most of the v1 vertex gap)
  and a forward-only variant that needs no separate trailer; both would need a
  format-level change or a larger buffer.

## Gate exit codes (`exits.tsv`)

fmt, clippy `-D warnings`, tests, no-std tests, doctests, docs `-D warnings`,
wasm32 build, semver-checks vs 0.2.16, reference check, unsafe boundary,
corpus + wasm identity, benchmark, fuzz build, fuzz, fmt check and the
public-text guard all exited 0. `gates.sh` reproduces them; `extra.sh` and
`fuzz.py` are its helpers.
