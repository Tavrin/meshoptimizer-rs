# Remap proof argument

Baseline is v0.2.15 (`1ecf69abe9f92c17e13dc38fa97be659aee44582`).
All library edits are safe scalar Rust in `src/remap.rs`. Hashing, remap
generation, public signatures and caller-index-buffer remapping are unchanged.
No explicit SIMD or new allocation/metering helpers are added to the public API.

## Owned index output

The combined path requires indexed input, remap length <= u32::MAX,
checked `2 * input.len()` work coverage, and checked retained-plus-output
bytes within the workspace limit. Otherwise the exact baseline path runs.
This includes all work exhaustion cases and storage-pressure eviction cases.

Reservation is fallible and workspace-independent. If it fails, the original
validation/allocation path decides the error, preserving validation precedence.
The temporary initialized output has exactly the input length. No workspace
or caller storage changes during the speculative gather. Allocation failure
injection is not performed; actual reservation failure explicitly falls back.

Each checked gather loads `remap[input[i]]` once. Four-record groups and the
tail visit the original order. An out-of-range index stops the gather and
replays the original validator with untouched Work and workspace state.
Consequently the first invalid index still returns IndexOutOfBounds and its
exact prefix charge, even if a preceding valid index mapped to NONE. The
owned temporary is dropped; its writes cannot escape to the caller.

A successful gather proves all indices valid. Charge the original validation
visits, then the same requested owned bytes and, if different, actual capacity
bytes. Only then reduce gathered values for NONE and find its first position
on failure. This is the baseline's sentinel selection and charge, after the
same logical allocation point. On success charge the entire copy scan.
The work-coverage predicate proves every deferred charge fits. Size and byte
arithmetic remains checked; unindexed calls use the baseline unchanged.

The first index candidate accumulated sentinel presence per element. It
increased instructions by 33% on real meshes and was rejected and restored.
The second candidate retains a separate vectorizable sentinel reduction,
combining only index validation and gathering. Campaign 1's clippy failure
is retained: the reserve/resize spelling triggered slow_vector_initialization;
resize_with keeps fallible reservation and initializes the same zero values.

## Packed vertex output

Keep the required-target prepass, destination size validation, owned hole
zeroing and work precharge exactly as before. Nonpacked streams, other record
widths and insufficient work all execute the original copy function.

Only sizes 4/8/12/16 with stride == size and offset == 0 enter the fixed-record
helper. A validated stream holds at least count complete records, and remap
length equals count. Restricting the source record slice to remap.len() is
therefore in bounds, including empty input and surplus source bytes.

The isolated monomorph visits four consecutive records at a time, then the
0..3-record tail. Within each group it writes in source order, exactly as
the original zip. NONE skips its write; repeated targets retain the last
source. Destination bounds were established from the largest mapped target;
chunk indexing retains its safe bounds check. Caller holes and tail bytes
are untouched; owned holes remain initialized to zero. Work is charged by
the same precharge or original limited path, so partial-write prefixes match.

## Evidence

The final campaign has 192 samples, all at 100% counter running. Every
four-arm output agrees. The current library is byte-identical to the frozen
campaign-2 candidate; its baseline is byte-identical to the frozen v0.2.15
source. `TABLE.md` gives all case ratios and wall time disclosure.

| Function | / v0.2.15 | / scalar C++ | / default C++ | Worst / base | Verdict |
|---|---:|---:|---:|---:|---|
| remap_vertex_buffer | 0.779 | 1.087 | 1.087 | 0.926 | SHIPS |
| remap_index_buffer | 0.885 | 1.437 | 1.437 | 0.886 | SHIPS |

Final `proof-exits.tsv`: all 14 steps exit 0. Tests: 295 all-feature and
249 no-default-feature tests pass; eight intentional ignores in each mode.
Clippy with warnings denied, fmt, independent unsafe/package boundary and
public-text checks pass.

| Library mode | Systematic comparisons | Random comparisons | C++ comparisons | Differences |
|---|---:|---:|---:|---:|
| std + SIMD | 10,811,136 | 6,033,408 | 2,592 | 0 |
| no default features | 10,811,136 | 6,036,192 | 2,592 | 0 |

Each baseline/candidate comparison checks four workspace states. The
systematic sweep uses 4,660 generated cases, every work boundary through
2 * max(index count, remap count) + 1, and crossed byte limits around the
observed allocation size. Random sweeps use 3,000 seeded larger meshes,
expanded into 62,848/62,877 cases, with short and exact work/byte limits.
The proof source binding is unchanged from start to finish; proof-results.json
binds the retained executable and raw-log digests.

Previous attempts remain explicit: campaign-1 rejected the index prototype
and stopped on clippy exit 101; the first final-source proof passed tests,
clippy, fmt and boundary, then exited 127 because a comment edit changed
Bash's current script offsets. The complete fresh final proof supersedes
those partial attempts. No library change followed campaign-2 measurement.

Finite tests supplement the argument; they do not exhaust all possible
allocations or inputs. No allocator-failure injector, Miri, wasm or native
non-x86 execution was run. Performance coverage is packed 12-byte position
records and 32-bit indices on the retained eight fixtures. Other record
widths, strided streams, unindexed input, caller-buffer variants, malformed
inputs and resource limits are correctness coverage, not separately claimed
performance results.
