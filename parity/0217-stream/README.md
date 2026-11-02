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
| fuzz target `buffer_stream` (arbitrary bytes, count, stride, chunk and output widths; streaming result must equal one-shot `Result`) | 3,075,288 runs, 604 CPU-seconds, 0 mismatches, exit 0 (`fuzz-run-0.log`, `fuzz.json`) |

## Throughput (release, 65,536 vertices of stride 32; 255x255 quad grid, 390,150 indices; MB/s of decoded output, `benchmark.log`)

| version | one-shot vertex | stream 4 KiB | stream 64 KiB | one-shot triangle | stream 4 KiB | stream 64 KiB |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 6248 | 4439 (0.71x) | 5471 (0.88x) | 5633 | 5165 (0.92x) | 5138 (0.91x) |
| 1 | 10385 | 7445 (0.72x) | 8465 (0.82x) | 5917 | 5952 (1.01x) | 5780 (0.98x) |

Before the speed pass (same machine, earlier tree): vertex v0 0.56x / 0.69x,
v1 0.16x / 0.19x, triangles 0.24x / 0.25x at 4 KiB / 64 KiB chunks. A second run
of the final tree gave 64 KiB ratios 0.87 / 0.80 (vertex v0 / v1) and 0.91 / 1.01
(triangles). Wall time on a shared machine is disclosure only; the decision
metric is instructions in user mode per decode (`perf stat -e instructions:u`,
`BENCH_ONLY=<case> BENCH_RUNS=200`, setup subtracted), where the
instructions-equivalent ratio is one-shot / streaming:

| case | one-shot | stream 4 KiB (before / after) | stream 64 KiB (before / after) |
| --- | --- | --- | --- |
| vertex v0 | 7.45 M | 0.51 / 0.76x | 0.65 / 0.97x |
| vertex v1 | 5.18 M | 0.17 / 0.79x | 0.21 / 0.96x |
| triangles v0 | 7.23 M | 0.26 / 0.87x | 0.26 / 0.87x |
| triangles v1 | 6.97 M | 0.26 / 0.97x | 0.26 / 0.97x |

Profile before the change (`perf record -e instructions:u`, fat LTO inlines
everything into the bench `main`): the streaming path was 50% of samples in
`main` (per-triangle `TriangleState::one`, scalar `scalar_deltas` for v1
half-word and XOR channels, per-call `bytes` dispatch), 17% one-shot triangle
`run`, 7.5% / 5.2% / 3.7% `deltas_kernel` and 4% / 3.4% `bytes_kernel` (one-shot
SIMD vertex), 2.9% memmove and 1.8% memset (input window and scratch).

Streaming now runs the one-shot SIMD block kernels for each buffered block and
decodes triangles in batches. It is still a memory-bounded interface; decode
whole buffers when they fit. One-shot cost: the one-shot vertex kernels keep their own copy of the block
loop and instantiate `bytes_kernel` and `deltas_kernel` separately from the
incremental decoder's (`BLOCK` const parameter), so adding the shared block
function no longer changes inlining into the one-shot path. Median of 5
`perf stat -e instructions:u` runs, 200 decodes, setup subtracted: committed HEAD
7.60 M (v0) and 5.18 M (v1); this tree 7.45 M and 5.18 M (x86_64, AVX-512
path); the shared-function version had measured 7.70 M and 5.27 M. The one-shot
triangle decode is unchanged (7.234 M). The aarch64 and wasm32 kernels still
call the shared block function from the one-shot path; they were not measured.


## Decisions

- Kept the earlier design (trailer-first two-part feed, fixed 9216-byte window).
- Input window now advances a cursor instead of shifting bytes per consumed
  block; consumed bytes are compacted only when space runs out. The first version
  moved up to 9 KiB per triangle (about 0.04x); fixed after measuring.
- `TriangleState::one` no longer clones all FIFOs per triangle; every fallible
  step precedes the first FIFO write, so an error leaves state untouched.
- `Status` and `Progress` docs were generalised (no signature change); semver
  check against 0.2.16 passes.
- Speed pass: a buffered vertex block runs `vertex_block_kernel`, the block half
  of the whole-stream SIMD kernel (x86 SSSE3 and AVX-512, NEON, simd128; the
  scalar parser remains for other levels). `TriangleState::run` decodes every
  triangle that the window and the output allow in one call and stops, resumable,
  at the first triangle whose lookahead has not arrived. Rejected: sharing the
  triangle loop with the one-shot decoder (it cost the one-shot decode 8% more
  instructions); that copy stays separate. Reversed after measurement: sharing the vertex block function cost the one-shot
  vertex decode +1.4% to +1.7% instructions, so on x86_64 the one-shot kernel
  keeps its own copy of the block loop (reversal cost: free). aarch64 and simd128
  were only cross-checked, not run.
- Not done: a forward-only variant that needs no separate trailer; it would need
  a format-level change or a larger buffer.

## Gate exit codes (`exits.tsv`)

fmt, clippy `-D warnings`, tests, no-std tests, doctests, docs `-D warnings`,
wasm32 build, semver-checks vs 0.2.16, reference check, unsafe boundary,
corpus + wasm identity, benchmark, fuzz build, fuzz, fmt check and the
public-text guard all exited 0. `gates.sh` reproduces them; `extra.sh` and
`fuzz.py` are its helpers.
