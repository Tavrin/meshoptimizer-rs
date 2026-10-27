# H2 safe hot loops

Baseline: v0.2.9, `df7fc567b6540eb06cb583e2d323c01e5797609c`. Candidate source/binary/input hashes and all raw counter samples are retained under `<scratch>/meshopt-0210-H2/`. No wall timings were taken.

## Techniques

- `src/input.rs:588`: Pre-slice fully padded 64-byte records into fixed-size arrays. Short trailing records retain the original scale reader and validation-return position; mapped and other layouts retain their original paths. Preserve padding exclusion and the independent finiteness pass.
- `src/overdraw.rs:128`: Prove the three stamp access bounds together with their maximum index.
- `src/overdraw.rs:224`: Precharge the exact soft-cluster release bound when all work fits and the threshold proves the target finite. A monomorphized prepaid kernel omits budget checks; tight budgets and overflow-capable thresholds retain the original per-visit path.
- `src/overdraw.rs:265`: Iterate adjacent hard-cluster boundaries through slices rather than repeated indexed lookups.

No new unsafe, API, allocation policy, SIMD, square-root backend, version, or other-family changes. The existing C++ driver is reused; no new FFI driver was added.

## Instruction gate

Counter-only resident perf 6.17 `instructions:u`, CPU9, median of three per arm, opt3/thin LTO/one codegen unit, Rust std without SIMD, scalar meshoptimizer 1.3 (meshopt 0.7.0). All counters must report ≥99% running. See REGISTRATION.md and retained preparation/binding JSON for archived harness provenance.

| Family / population | candidate/base GM | candidate/M7 GM | worst candidate/base | Verdict |
|---|---:|---:|---:|---|
| simplify-scale / all | 0.517509 | 1.066398 | 0.632927 | PARTIAL |
| simplify-scale / frozen | 0.526727 | 1.078286 | 0.632927 | PARTIAL |
| overdraw / all | 0.930364 | 1.584082 | 0.967339 | PARTIAL |
| overdraw / frozen | 0.923822 | 1.220458 | 0.967339 | PARTIAL |

Frozen includes the three real-overdraw fixtures. All is the GM over distinct frozen and chain call inputs. The per-case regression rule applies to every input.

| Chain / stage | base instructions | candidate instructions | M7 instructions | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| 214e / simplify-scale | 679845409.50 | 341623013.00 | 326326193.00 | 0.502501 | 1.046876 |
| 214e / overdraw | 2102892324.50 | 1990937441.50 | 901598804.00 | 0.946761 | 2.208230 |
| a082 / simplify-scale | 723733714.50 | 363676949.50 | 347392579.00 | 0.502501 | 1.046876 |
| a082 / overdraw | 5776364878.00 | 5303653451.00 | 2981254115.50 | 0.918165 | 1.779001 |
| b6fd / simplify-scale | 180935666.50 | 90920828.50 | 86849521.00 | 0.502504 | 1.046878 |
| b6fd / overdraw | 1442004964.00 | 1323824753.00 | 743245215.00 | 0.918045 | 1.781141 |

Chain stage totals sum per-call medians with retained call multiplicity. These are captured stage replays, not whole-application measurements. Reorder counters include the adapter’s saved-input clone and output copy.

| Case | base instructions/call | candidate instructions/call | M7 instructions/call | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| frozen-simplify-scale-tiny-smooth | 1629.10 | 1031.10 | 852.10 | 0.632927 | 1.210069 |
| frozen-simplify-scale-medium-smooth | 211638.72 | 106565.72 | 101661.72 | 0.503527 | 1.048238 |
| frozen-simplify-scale-medium-seams | 259588.73 | 130645.72 | 124677.72 | 0.503280 | 1.047867 |
| frozen-simplify-scale-medium-sparse | 422888.74 | 212726.72 | 203061.72 | 0.503032 | 1.047596 |
| frozen-simplify-scale-million-smooth | 25134662.00 | 12630496.00 | 12064828.50 | 0.502513 | 1.046886 |
| frozen-overdraw-tiny-smooth | 16564.13 | 16023.13 | 20224.14 | 0.967339 | 0.792278 |
| frozen-overdraw-medium-smooth | 1790948.02 | 1651412.02 | 1598660.61 | 0.922088 | 1.032997 |
| frozen-overdraw-medium-seams | 1819436.04 | 1674524.01 | 1592645.72 | 0.920353 | 1.051410 |
| frozen-overdraw-medium-sparse | 1905023.04 | 1765487.04 | 1627907.14 | 0.926754 | 1.084513 |
| frozen-overdraw-million-smooth | 214071189.75 | 197570478.50 | 191777659.75 | 0.922920 | 1.030206 |
| realfixture-overdraw-214e | 618573455.25 | 578988775.50 | 283854700.75 | 0.936007 | 2.039736 |
| realfixture-overdraw-b6fd | 438963122.25 | 394389820.50 | 248932752.50 | 0.898458 | 1.584323 |
| realfixture-overdraw-a082 | 1755820963.75 | 1577528266.00 | 995683034.00 | 0.898456 | 1.584368 |
| chain-214e-0-simplify-scale | 339922704.75 | 170811506.50 | 163163096.50 | 0.502501 | 1.046876 |
| chain-214e-4-overdraw | 619379275.75 | 579794658.50 | 284667719.75 | 0.936090 | 2.036742 |
| chain-214e-7-overdraw | 432066886.50 | 415674062.25 | 166131682.25 | 0.962060 | 2.502076 |
| chain-b6fd-0-simplify-scale | 90467833.25 | 45460414.25 | 43424760.50 | 0.502504 | 1.046878 |
| chain-b6fd-3-overdraw | 439769140.50 | 395195875.00 | 249739229.25 | 0.898644 | 1.582434 |
| chain-b6fd-6-overdraw | 165772824.00 | 155491821.75 | 77029337.25 | 0.937981 | 2.018605 |
| chain-b6fd-9-overdraw | 115460517.50 | 111224679.75 | 44854041.00 | 0.963314 | 2.479703 |
| chain-a082-0-simplify-scale | 361866857.25 | 181838474.75 | 173696289.50 | 0.502501 | 1.046876 |
| chain-a082-3-overdraw | 1763355273.00 | 1585062805.50 | 1003217517.00 | 0.898890 | 1.579979 |
| chain-a082-6-overdraw | 663030972.00 | 621909892.50 | 308049103.00 | 0.937980 | 2.018866 |
| chain-a082-9-overdraw | 461796194.00 | 444854027.50 | 179360437.75 | 0.963312 | 2.480224 |

## Proof and exit codes

```text
retained-initial-candidate-build	101
retained-pilot-2-regression	3
fmt	0
proof-fmt	0
all-features	0
no-default-features	0
clippy	0
boundary	0
exact-build-std	0
exact-scale-std	0
exact-overdraw-std	0
exact-build-no-std	0
exact-scale-no-std	0
exact-overdraw-no-std	0
cpp-differential	0
measurement	0
final-fmt	0
final-all-features	0
final-no-default-features	0
final-clippy	0
final-boundary	0
fuzz	0
receipt-audit	0
report	0
cleanup	0
```

All exact comparisons cover complete Result values, every output word (including failure prefixes and untouched destination tails), and Usage work/bytes against a separately compiled v0.2.9. Scale compares float bits and Result values; its API has no workspace. Existing seed cases and frozen corpus are included, in std and no_std; every work prefix from zero through the terminal prefix plus one is checked on small finite/nonfinite/overflow/subnormal/disconnected meshes. All accepted layout classes are covered, including full/short 64-byte little/big-endian views and unaligned offsets. Existing pinned C++ differentials and counter output hashes provide independent parity evidence.

The existing fuzz generators already reach full and short 64-byte records for scale and overdraw, both byte orders and unaligned offsets; no widening was needed. Retained CPU-accounted fuzz logs must show at least 300 CPU seconds per family and zero crashes.

Initial candidate build exited 101 (integer type inference in extracted clustering helper); fixed with an explicit u32 annotation before measurement. The failed receipt is retained. Cancelled admission waits produced no measurements. Pilot samples are separate from the final campaign.

## Completed observations

Final instruction campaign: 24 distinct cases, 216 samples, every process exit 0, 100% counter running, all Rust/C++ outputs byte-identical and all chain outputs matching their historical hashes.

Exact comparisons: 1,950,606 scale Result/bit observations and 2,835,968 overdraw Result/output/Usage observations per configuration, for 9,573,148 across std and no_std. Existing pinned C++ differential: 356 cases, exit 0.

Round 1 was retained under round1/ and replaced to preserve the original short-record validation-return position. The final round rebuilds and reruns all required gates, measurements and CPU fuzzing against the accepted source. No implementation source changed during final qualification.

Rejected scale approach: a chained full-prefix/final-record iterator regressed the tiny and medium pilot cases by 45.7% and 50.2%. It was replaced before the final campaign. Raw pilot samples and a reconstruction verified against every sampled source hash remain under pilot-2/ and pilot-2-rejected-source/. Failed build and rejected pilot receipts remain recorded; they are not passing gates.

Both final families are PARTIAL. Finite validation and checked indexed reads remain; no C++ instruction parity or wall-time improvement is claimed.

| Fuzz target | CPU seconds | Executions | Crashes |
|---|---:|---:|---:|
| overdraw | 359.95 | 84027386 | 0 |
| simplify_scale | 359.93 | 487213556 | 0 |

CPU-accounted fuzz verdict: passed=True; source/dependency/executable identities unchanged=True. Full replay descriptors, binaries and source archive are retained outside the build target.

## Cleanup

Designated Cargo target deleted; exit 0, absence verified in CLEANUP.json. Standalone binaries, archived inputs, raw samples, exact/fuzz replay sources, hashes and failed/rejected receipts remain in scratch. No merge, tag, package publication or version bump.
