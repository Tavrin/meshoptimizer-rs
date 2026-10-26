# H3 complete: NO-GAIN; both families reverted

Both safe-Rust candidates miss the pre-registered 5% instruction-reduction gate.
The final library source, Cargo.toml and Cargo.lock are byte-identical to v0.2.9
(`df7fc567b6540eb06cb583e2d323c01e5797609c`). No optimization, API change, new
unsafe code or version bump is retained. This draft PR records a negative result.

## Techniques and exact source binding

The first measured variant is commit `43da093`; the revised measured variant is
`9ebc843`. Both retained source snapshots were compared byte-for-byte with those
commits. References below bind to those commits, not the reverted checkout.

- Partition roots: [partition.rs:58](https://github.com/Tavrin/meshoptimizer-rs/blob/43da093/src/partition.rs#L58), direct std sqrt with canonical NaN handling; retain the software memo under no_std. Revised [partition.rs:27](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/partition.rs#L27) bounds the active heap, [partition.rs:213](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/partition.rs#L213) monomorphizes covered versus short-budget adjacency visits, and [partition.rs:334](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/partition.rs#L334) pre-slices adjacency streams. Only an infallible, fully covered loop is precharged; short budgets retain the original per-item path. Fallible leaf scoring is unchanged.
- Spatial first attempt: [meshlet_spatial.rs:95](https://github.com/Tavrin/meshoptimizer-rs/blob/43da093/src/meshlet_spatial.rs#L95), bounded SAH area streams and native division. It failed the million-smooth regression bound. The revised attempt restores that pivot code and uses [meshlet_spatial.rs:46](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/meshlet_spatial.rs#L46), fixed triangle records and compile-time count-output selection, plus producer iterators at [meshlet_spatial.rs:330](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/meshlet_spatial.rs#L330) and [meshlet_spatial.rs:375](https://github.com/Tavrin/meshoptimizer-rs/blob/9ebc843/src/meshlet_spatial.rs#L375).

## Registered instruction verdicts

Scalar std, opt3, thin LTO, one codegen unit; installed perf 6.17,
`perf stat -e instructions:u`, resident count-only protocol on CPU9. Three
rotating repetitions per arm; medians per case, equal-case geometric means per
family. Tiny/medium/million iterations: 10000/100/4. No wall timings.
M7 is meshopt 0.7.0 / C++ meshoptimizer 1.3 with MESHOPTIMIZER_NO_SIMD.
The archived bet-0 driver was exported from immutable `bench/peer-screen`
(`e8827e4de9b69c98b9a96c6f49f8192a6b045ff7`); its retained NAS replay adapter's
H3 kernels match that driver. `driver-derivation.patch` retains the complete diff.
All five input hashes match the archived branch's H3 receipts.

Apply verbatim: FAIL if any candidate/base >1.01; otherwise PASS if
candidate/M7 GM <=1.00; otherwise PARTIAL if candidate/base GM <=0.95;
otherwise NO-GAIN. Failed or NO-GAIN family changes are reverted.

| Campaign | Family | candidate/base GM | candidate/M7 GM | worst candidate/base | Verdict / action |
|---|---|---:|---:|---:|---|
| First candidate | partition | 0.976434257 | 1.378546470 | 0.985749007 | NO-GAIN; reverted |
| First candidate | meshlets-spatial | 1.004852493 | 1.169347103 | 1.019617758 | FAIL; reverted |
| Revised candidate | partition | 0.976751855 | 1.378995697 | 0.990752865 | NO-GAIN; reverted |
| Revised candidate | meshlets-spatial | 0.972126680 | 1.131264053 | 0.984801573 | NO-GAIN; reverted |
| Restored-source control | partition | 1.000000062 | 1.411817050 | 1.000000339 | NO-GAIN; unchanged control |
| Restored-source control | meshlets-spatial | 1.000000066 | 1.163700366 | 1.000000410 | NO-GAIN; unchanged control |

### First candidate: median instructions per call

| Frozen case | base | candidate | M7 | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| partition-tiny-smooth | 10006.12 | 9839.12 | 6952.12 | 0.983310230 | 1.415268526 |
| partition-medium-smooth | 2041823.19 | 2003140.18 | 1486935.67 | 0.981054672 | 1.347159948 |
| partition-medium-seams | 1584381.50 | 1561802.49 | 1163708.07 | 0.985749007 | 1.342091312 |
| partition-medium-sparse | 2190094.88 | 2151411.83 | 1513411.72 | 0.982337272 | 1.421564140 |
| partition-million-smooth | 273218086.75 | 259605414.00 | 189676369.25 | 0.950176531 | 1.368675576 |
| meshlets-spatial-tiny-smooth | 46781.12 | 45755.12 | 47591.02 | 0.978068086 | 0.961423249 |
| meshlets-spatial-medium-smooth | 32142644.93 | 32425530.03 | 26676882.80 | 1.008800928 | 1.215491715 |
| meshlets-spatial-medium-seams | 32185733.13 | 32491237.20 | 26699220.67 | 1.009491910 | 1.216935790 |
| meshlets-spatial-medium-sparse | 32216592.14 | 32499477.25 | 26676899.49 | 1.008780727 | 1.218262912 |
| meshlets-spatial-million-smooth | 7258259233.50 | 7400650010.25 | 5864459017.25 | 1.019617758 | 1.261949310 |

### Revised candidate: median instructions per call

| Frozen case | base | candidate | M7 | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| partition-tiny-smooth | 10006.12 | 9886.12 | 6952.13 | 0.988007345 | 1.422028896 |
| partition-medium-smooth | 2041824.10 | 1997438.45 | 1486936.14 | 0.978261766 | 1.343324973 |
| partition-medium-seams | 1584383.78 | 1569732.77 | 1163708.18 | 0.990752865 | 1.348905849 |
| partition-medium-sparse | 2190098.77 | 2145710.06 | 1513411.80 | 0.979732097 | 1.417796571 |
| partition-million-smooth | 273218303.25 | 258906579.00 | 189676531.75 | 0.947617989 | 1.364990052 |
| meshlets-spatial-tiny-smooth | 46781.12 | 46070.12 | 47591.03 | 0.984801573 | 0.968042125 |
| meshlets-spatial-medium-smooth | 32142645.15 | 31136799.86 | 26676884.64 | 0.968706829 | 1.167182761 |
| meshlets-spatial-medium-seams | 32185733.55 | 31185287.65 | 26699221.42 | 0.968916480 | 1.168022361 |
| meshlets-spatial-medium-sparse | 32216594.33 | 31210747.83 | 26676900.45 | 0.968778621 | 1.169954054 |
| meshlets-spatial-million-smooth | 7258259882.25 | 7037118154.50 | 5864459492.25 | 0.969532404 | 1.199960229 |

### Restored-source control: median instructions per call

| Frozen case | base | candidate | M7 | candidate/base | candidate/M7 |
|---|---:|---:|---:|---:|---:|
| partition-tiny-smooth | 10006.12 | 10006.12 | 6952.12 | 0.999999980 | 1.439289964 |
| partition-medium-smooth | 2041823.21 | 2041823.18 | 1486935.66 | 0.999999985 | 1.373175205 |
| partition-medium-seams | 1584381.50 | 1584381.51 | 1163708.07 | 1.000000006 | 1.361493961 |
| partition-medium-sparse | 2190094.86 | 2190094.86 | 1513411.73 | 1.000000000 | 1.447124280 |
| partition-million-smooth | 273218090.25 | 273218183.00 | 189676394.75 | 1.000000339 | 1.440443780 |
| meshlets-spatial-tiny-smooth | 46781.12 | 46781.12 | 47591.02 | 1.000000000 | 0.982981932 |
| meshlets-spatial-medium-smooth | 32142645.32 | 32142645.05 | 26676882.88 | 0.999999992 | 1.204887587 |
| meshlets-spatial-medium-seams | 32185733.03 | 32185746.24 | 26699222.26 | 1.000000410 | 1.205493775 |
| meshlets-spatial-medium-sparse | 32216591.98 | 32216591.85 | 26676899.34 | 0.999999996 | 1.207658785 |
| meshlets-spatial-million-smooth | 7258260562.50 | 7258260062.00 | 5864458990.50 | 0.999999931 | 1.237669165 |

## Real chains

| Chain | Recorded calls | H3 calls | Instructions / verdict |
|---|---:|---:|---|
| 214e85c568a64e8a | 16 | 0 | Not applicable |
| b6fdb93d8a1709e2 | 20 | 0 | Not applicable |
| a082ca1fd19bb86a | 20 | 0 | Not applicable |

The traces contain scale, simplify, cache and overdraw calls only. No proxy stage
or whole-chain performance claim is substituted. The NAS archive was extracted
into this lane's scratch; no NAS writes and no benchmark-branch changes occurred.

## Proof and harness repair

Bare hardware sqrt is not bit-identical over every f32 pattern: the pinned
software backend canonicalizes negative operands and NaN payloads. The tested
std guard preserves those bits. The actual first-variant helper was extracted
verbatim: all 4,294,967,296 inputs, zero replacement mismatches. Bare sqrt had
2,155,872,253 mismatches; first operand 0x7f800001. The final restored software
backend also passes all 2^32 patterns. No hardware-root change is shipped.

The first preliminary comparison built its arms from different harness revisions
while the corpus was extended. A later full proof exposed a stale no_std baseline
driver: copy2 had preserved source mtimes older than Cargo's cached fingerprint.
Its 3,053,556 observations differed from the current producer's 3,060,882 because
the disjoint-triangle test was missing. Both failed receipts are retained and
excluded. They are not library parity claims.

The repaired builder freezes all producer bytes once, writes fresh-mtime source
snapshots, uses unique package/binary names per arm and feature mode, and embeds
and verifies the frozen producer hash before comparing any output. All gates
and the complete exact/fuzz proof were rerun after the repair. Compressed-stream
equality is sufficient; differing compressed streams are compared as complete
uncompressed bytes, never by hashes alone.

The final exact proof covers packed inputs; every supported float and byte
stride and every starting offset within a stride, both endian orders, and short
final records; nonfinite/extreme/degenerate/malformed inputs; caller-buffer
shortages and sentinel tails; memory/error precedence; and every work limit
0 through success+1 on small connected and disjoint meshes. Serialized Results,
all output fields/bytes, Usage.work, Usage.bytes and partial writes are compared.
Existing p03 smoke seeds/header variants are replayed with the existing generator.
The lane generator widens the u8 vertex/triangle ceiling, adds disjoint triangles,
hard divisibility, aligned splits and byte/interleaved layouts. The pinned C++
differential runs 4,000 cases with std and 4,000 with no_std.

Final exact observations: **13,648,656**, plus **3,076** existing-seed observations; **1,796** complete comparison pairs, all equal. Widened deterministic differential fuzz: **114,432** input cases, **300.114902 candidate CPU seconds**, zero process failures/mismatches. CPU time comes from wait4 for the candidate driver only; base, compression, compilation and wall time are excluded.

Full budget/error/layout proof applies to the final restored source. Rejected
variants have canonical frozen-corpus output equality and the root proof; no complete
budget/layout proof is claimed for those rejected implementations.

## Exit codes and disposition

| Gate | Final exit |
|---|---:|
| test-all | 0 |
| test-nostd | 0 |
| clippy | 0 |
| fmt | 0 |
| boundary | 0 |
| differential | 0 |
| sqrt | 0 |
| exact comparison / widened fuzz | 0 |
| source/input binding | 0 |
| cleanup | 0 |

All **270** instruction processes exited 0 with counters running
>=99%; all observed frozen Rust and canonical C++ outputs matched. No sampled
instruction stream was selected or dropped to improve a verdict. Each rejected
campaign and the final control retains every raw sample.

Bootstrap history: the initial build exited 101 on a missing manifest before
archive extraction completed; no compilation or samples from it were used.
The preliminary and first full comparison attempts exited 1 on producer-version
mismatches described above. A temporary proof-script syntax preflight was fixed
before execution. The canceled queued jobs and all failed receipts remain in
scratch. Final gate success refers to the complete repaired rerun.

CPU-only moss-heavy (MOSS_HEAVY_GPU=0), declarations <=4 GiB; compilation and
qualification ultimately used a 2 GiB declaration, the preliminary proof 1 GiB.
Target `/mnt/linux-extra/moss-cargo-targets/meshopt-h3` deleted and absence verified; 680,864,758
apparent file bytes removed. All library sources and root manifests match v0.2.9. Lane
commits are authored as Tavrin without AI trailers. Draft PR:
https://github.com/Tavrin/meshoptimizer-rs/pull/33. No merge, tag, publish or
version bump.

Full evidence: `/mnt/linux-extra/moss-scratch/meshopt-0210-h3`. Compact counters, source/binary bindings, input binding,
producer identity, gate exits, harness incidents and cleanup are beside this
report. `rejected-r1/`, `trial-r2/` and `proof-attempt1/` in scratch retain the
rejected variants and failed proof receipts. `proof-output/` retains full
compressed observation streams, CPU receipts and per-process provenance.
