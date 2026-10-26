# SPEC-026 decode proof

PASS against published 0.2.5 `bea3e04`. Candidate library source is `4f2557f`,
checked after merging `origin/main` (`f1ca898`) as `64e5494`. No library code
changed during this proof. No timings were run; registered results stand.

## Exact native Results

The original-source release builds use `with_level(Level::Scalar, ...)`.
All 360 vertex fixtures from `differential.py` are reconstructed with its seed
and RNG history; every encoded SHA256 matches the retained original campaign.
Both vertex versions, all five strides and all count/shape fixtures are included.

- 3,047,360 truncation inputs: every offset from empty through the full stream.
- 3,047,000 mutations: XOR 255 at every encoded byte, including every control,
  group header, block boundary and channel selector.
- 155,136 assignments: all 256 byte values at the header, first-block controls
  and each channel selector on the 17-record fixtures for every stride/shape.
- Total: **6,249,496 cases / 12,498,992 API records**, zero mismatches, exits 0/0.

Each case runs allocating and caller-buffer APIs. Success carries the entire
output byte string; failure carries the actual `Error` Debug representation,
including its variant and any fields (the pinned enum has no payload fields).
API Results and successful output also agree within each build. Every
caller-buffer call asserts that its 37-byte destination tail is untouched,
including on errors. Late-error vertex prefixes are outside this comparison.

## wasm32 with std

Build target `wasm32-wasip1`, dependency defaults disabled, only `std` enabled;
`simd` disabled. Node 22.22.0 executes WASI. Both unmodified source revisions
and separate counter-instrumented candidate copies run in debug and release.

**3,380 filter fixtures plus 360 valid vertex fixtures = 3,740 cases /
14,240 API/output records per profile**. Debug and release each have zero byte
or error mismatches and exit 0/0. The same corrected suite also passes natively.
All six corrected original-source/counter comparisons have transcript SHA256
`cc8e99acc5f6895dd089e11a607b737cd41181895c4e4abaa3e025371bfab859`.

Filter calls cover allocating EXT views, caller-buffer EXT views and direct
in-place APIs. EXT input uses vertex version 0, as required by that API.
Direct filter partial bytes are compared even on errors; all destination tails
are checked. Count-zero EXT views consistently return `InvalidParameter`.
Oct zero records consistently return `NumericalFailure`.

Counts cover 0–5, 7, 8, 15–17, 63–65, 127–129. Four-record groups vary each lane;
Oct includes both widths, repeated keys, signed extrema and a zero vector at
every record position. Quat scale words include negative and positive values
around the `abs(d | 3) < 256` fallback boundary, in each of four lane positions,
plus extrema, repeated records, bulk groups and tails. Valid vertex fixtures
also reach the wasm-only stride-48/64 reconstruction paths.

## Reachability

Counters exist only in separate scratch source copies. Original-source builds
supply the primary parity results. Counter outputs must also match baseline.
In the corrected driver, every allocating and caller-buffer filter call with
at least four records separately asserts that its corresponding bulk counter
increases. Thus successful direct calls cannot hide rejected EXT calls.

Debug and release wasm counts are identical:

| Function | Calls per profile |
|---|---:|
| filter_root | 423,219 |
| scalar_oct | 4,118 |
| oct_records | 4,044 |
| scalar_oct_with_root | 4,082 |
| scalar_quat | 5,782 |
| quat_cached | 38,890 |
| trunc_normal | 600,300 |
| quat_records | 4,602 |
| packed | 371,534 |
| scalar_deltas | 63,940 |
| strided_bytes | 5,630 |

The corrected native counter comparison reaches all ten native functions;
`strided_bytes` is wasm-only. All counter comparison exits are 0/0.

## Receipts and decisions

Full frozen sources, both driver revisions, corpus/metadata, build logs,
executables, counter output and transcript hashes remain at
`/mnt/linux-extra/moss-scratch/meshopt-decode-proof/`. Compact outcomes are in
`PROOF-SUMMARY.json`; final gate and cleanup receipts accompany this report.

The first filter driver used vertex version 1 for EXT views. Both revisions
correctly rejected those allocating/caller-buffer calls with
`UnsupportedVersion`; direct calls reached all changed functions. Those
receipts are retained but superseded for filter acceptance by the version-0
completion driver. Its first launch exited 1 before builds because the guard
waited for the entire native counter replay instead of the completed native
exact proof. The corrected guard requires `native-exact.json`; the completion
run exits 0. No library output/error mismatch or library change occurred.

`decode-proof.py`/`.sh` are the fresh-scratch driver/recipe;
`decode-proof-filters.py`/`.sh` retain the explicit completion of the initial
harness gap. Both recipes use CPU-only moss-heavy, declaration 4 GB, and the
exact designated target. They contain no timing calls. The completion preserves
all initial receipts and uses fresh package/binary/log names.

Claims are bounded to these cases and targets. This adds no C++ parity claim,
SIMD performance result, new timing, version bump, release or publication.

The original recipe's optional exhaustive native counter replay was externally
stopped with SIGTERM (recipe exit **143**) after the corrected native counter
suite had independently proved all ten native functions. The original-source
6,249,496-case proof was already complete (0/0); it was not interrupted or
repeated. The stopped replay is retained as an external stop, never a passing
check or a technical failure. The final fresh-scratch recipe uses the valid
vertex/filter suite for native counters, avoiding that duplicate replay.
Required parity/counter comparisons and all builds exit 0; the initial
completion dependency guard exits 1; the redundant replay exits 143.
