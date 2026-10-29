# Incremental index-sequence decoder

## Contract and state machine

`codec::IndexSequenceDecoder::new(index_count, limits)` returns a fallible,
fixed-state decoder. `push(input, output)` returns `Progress { consumed,
written, status }`, where `Status` is `NeedInput`, `NeedOutput`, or `Done`.
`finish(self)` validates EOF. Construction is the opt-in; this safe scalar
API works with default features and with no default features. No new Cargo
feature or format version is introduced.

State advances through header, indices, and four opaque padding bytes. Header
high bits must be `0xd0`; versions 0 and 1 are accepted. Between calls, the
index phase retains a partial varint's accumulator and group count, the
remaining record count, and two predictor baselines. A record finishes on a
clear continuation bit or the fifth byte, including fifth bytes with a set
continuation bit. Accumulation uses the same seven-bit groups and truncating
32-bit shifts as the one-shot decoder. The low bit selects the baseline;
shifting it away and zigzag decoding gives the same wrapping delta. Predictor
state and output update only when that record is complete.

During the index phase, zero output capacity returns `NeedOutput` without
consuming payload bytes, even if input is also empty. Header and padding do
not require output space. Input exhaustion returns `NeedInput`; only `finish`
turns incompleteness into `InvalidStream`. Exactly four padding bytes lead to
`Done`; extra bytes in that call or any later call are terminal errors. Empty
calls after `Done` are harmless. All writes remain provisional until `finish`
succeeds. Successful calls preserve all destination elements after `written`.
An error can follow provisional writes and returns no progress receipt; errors
are sticky, so callers must discard that decoder and provisional output.

## Identity argument and deliberate error timing

For a successful `finish`, all declared records ended and exactly four bytes
follow them. Thus every record start had at least the five readable bytes
required by one-shot lookahead, and the total source length was at least
`count + 5`. The same group accumulation and predictor recurrence therefore
produce the same `u32` records. Conversely a one-shot success parses the same
header and records and leaves exactly four bytes, so the streaming state
machine accepts it regardless of chunking. Padding values are not constrained
by either implementation. No noncanonical varint rejection is added.

Unsupported versions are rejected as soon as their header arrives. On a short
stream this can be `UnsupportedVersion` before one-shot's `InvalidStream`
minimum-length guard. This earlier error is documented in rustdoc and the
guide and modeled explicitly by the proof driver. Valid-header truncation,
unterminated/incomplete payload, missing padding, and excess input return
`InvalidStream` at `finish` or earlier. Five continuation bytes are a complete
upstream varint, not intrinsically malformed.

## Resources and safety

The decoder allocates no heap storage, so `max_bytes = 0` is supported. Checked
`count * 4` uses the existing platform allocation bound and returns
`SizeOverflow` first. Construction reserves four work units per declared
index; each consumed compressed byte costs one more. The budget persists
across all pushes. Complete valid streams have the same total work charge as
the one-shot decoder, although failure timing differs because source length
is initially unknown. Empty calls and output stalls cost nothing. Extra bytes
after completion are rejected before being charged.

All state fields are private. While accumulating, `groups < 5`, so shifts are
0, 7, 14, 21, or 28; the top group truncates exactly as upstream. Output stores
follow an explicit capacity check, remaining records decrement only when
positive, and padding increments only below four. There is no unsafe code,
unbounded staging, platform I/O, thread, or transport adapter in the new API.

## Retained verification

`proof.sh` uses a read-only archive of baseline
`1ecf69abe9f92c17e13dc38fa97be659aee44582` as a separately named crate. The clean
C++ oracle is pinned to `4c203430ca565cb59a468a91922c76c208169536`, with a
source tree identical to v1.3. The existing native and JS fixture exporters
capture the unchanged upstream tests; `corpus.tsv` retains all raw sequence
and glTF INDICES requests. The driver also generates 160 sequences, 20 explicit
noncanonical-varint cases, every truncation, malformed headers, and extra
padding cases. Corpus bytes and results are retained in `cases-default.tsv`
and `cases-no-default.tsv`.

For every case, both output widths and allocating/caller-buffer/undersized
caller-buffer APIs are compared baseline versus candidate at every work limit
from zero through `source length + count * 4 + 1`, covering the stride-2
success charge as well. Byte limits cover zero, footprint minus one, exact
footprint, and footprint plus one. Cold, warmed, and after-failure workspaces
compare result/error, `Usage`, and complete destination bytes, including
partial writes and untouched tails. Streaming is compared at every split
with output capacities 0 (retried with 1), 1, 3, and count plus one, plus
one-byte feeds with interspersed empty calls. Every streaming work limit is
also checked with whole-input and one-byte/zero-output feeds. C++ matches
success/failure for every case and exact output bytes on success. Both feature
modes run the same driver and must emit identical case receipts.

Six focused integration tests cover chunk identity, progress/backpressure,
noncanonical varints/opaque padding, malformed EOF/sticky errors, and whole
stream limits/platform bounds, and declared-count/padding completion. The
guide recipe is an executed doctest.
The additional standard cargo-fuzz target `sequence_stream` is compiled only;
no fuzz execution is requested or claimed. Finite differential testing
supplements the state-machine argument; it is not an exhaustive-input proof.

## Final results

Both modes completed with zero differences. Counts below are **per mode**:

| Check | Count |
|---|---:|
| Upstream corpus entries | 21 |
| Generated sequences | 160 |
| Explicit noncanonical cases | 20 |
| Total cases, including malformed/truncated variants | 9,990 |
| C++ differentials | 9,990 |
| Streaming chunk/output configurations | 2,703,362 |
| Streaming work budgets | 2,284,337 |
| Baseline/candidate observations | 164,442,780 |

Case receipts are byte-identical between modes. The final all-features suite
passed 297 tests, and the no-default-features suite passed 251; each retained
eight existing ignored tests. Six focused streaming tests pass in both runs.
The native fixture capture executes 52 unchanged fixture functions; the JS
export awaits all 21 upstream decoder tests.

Every final gate in `exits.tsv` exits 0: reference pin/cleanliness, unchanged
one-shot codec sources, formatting, tests, both clippy modes, audited unsafe
and package boundary, fuzz-target compilation, fixture export, C++ build,
both exact drivers, and public text. The first attempt stopped at clippy 101
for the test helper's constant-size `chunks_exact`; `clippy-attempt1.log` and
the append-only exit receipt preserve that failure. The helper now uses
`as_chunks`, and the entire suite was rerun successfully.

The final retained commands report 143.97 CPU seconds in total; the two
exact runs account for 94.32 CPU seconds, comfortably within the two-CPU-hour
boundary-case budget. These are execution-cost receipts, not performance
comparisons. `verification.json` records counts, exits, and per-command CPU
cost. `sources.sha256` binds the source, corpus, scripts, and retained logs.

The dedicated build directory is removed after retaining these receipts.
No queued job was canceled. Git branch, HEAD, index, refs, and publication
state were not changed; all deliverables are file edits for the coordinator.

