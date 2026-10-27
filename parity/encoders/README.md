# Allocating encoder consumer profiles (registered protocol)

This campaign extends the frozen P03 requests to Cargo defaults (O3, no LTO,
16 codegen units) and the downstream engine consumer (O3, thin LTO, one codegen unit). It
retains the original scalar-strict C++ 1.3 oracle and normal SIMD time comparator.
The peer is meshopt 0.6.2, with bundled C++ 0.25 and its published build flags.
Tiny Oct/Quat implementations are not retuned.

Set `CARGO_TARGET_DIR=<target-dir>`
and `CARGO_INCREMENTAL=0`. Every command below must run through
`PARITY_RUNNER_GPU=0 PARITY_JOB=meshopt-enc03
<job-runner> <GB> timeout 840 <command>`.
Declare 3 GB for these builds, preparation, parity and measurements (measured
controller peak RSS was below 1.2 GiB).
The wrapper waits for RAM admission; no quiet-machine wait is used.
For this measured job, `DOWNSTREAM_HEAVY_USE_DECLARED=1` uses the 3 GB declaration
when generic interpreter history over-reserves. This is the explicit declaration
mode documented by a downstream engine HEAVY_JOBS; queue, RAM, disk and runtime guards remain.

1. `python3 parity/encoders/build_profiles.py`
2. `python3 parity/encoders/profiles.py prepare`
3. `python3 parity/encoders/profiles.py reproduce defaults`, then `reproduce downstream`.
   This hashes and measures the exact historical owned stride-four request before
   any tuning. A non-reproducing gap stays non-reproducible, with its interval;
   it cannot be attributed to a new implementation change.
4. `python3 parity/encoders/profiles.py parity defaults`, then `parity downstream`.
5. `python3 parity/encoders/profiles.py timing defaults`, then `timing downstream`.
6. `python3 parity/encoders/profiles.py peer defaults`, then `peer downstream`.

Evidence defaults to `<scratch>/meshopt-enc03`;
`MESHOPT_ENCODER_ARTIFACTS` selects a new epoch without replacing old receipts.
The Cargo target is fixed, stays below 10 GB, and is deleted after completion.
Prepared geometry must match the hashes in the committed historical peer ledger;
the exact decoder request must hash to
`13413c1e2c69f3de2f28a32005650fb46dfac1f92621cce393acefb3929040e9`.

The upstream campaign uses CPU26, rotating/reversing interleaved pairs and
independent Rust/C++ duplicate processes for A/A resolution. All timed
processes receive the same calibration request before iteration selection. Batches calibrate
to 25 ms. Check at five complete pairs. If the nominal paired-log Student-t interval
is not wholly on one side of 1.50, or either A/A interval is outside [0.8, 1.25],
collect exactly 256 pairs with no intermediate confidence decisions. This
precision rule is registered for every case before a new campaign begins. Bursts checkpoint after 690 seconds, below the 15-minute limit.
For more than 20 pairs, the 20-pair critical value is retained as a
conservative bound. The original 8 ms/20-pair and 25 ms/40-pair epochs remain separate receipts.
All samples, failed rows and unresolved rows remain; unresolved rows prevent
qualification. Completed streams cannot be rerun. Interrupted cases are retained
and require an explicit disposition, rather than silent repetition.

Family geometric means must be <=1.25 and every case upper95 <=1.50 in each
profile. The owned-decode exception is assessed separately, not folded into the
encoder mean. The peer refresh reuses `parity/compare/measure.py` and its original
five-or-twenty-pair rule, independent validators and frozen five geometry inputs;
only shared allocating encoder operations are selected. Matched v0/v1 vertex
operations are included, the mismatched default-version comparison is excluded.
Its family target is <=1.0. Intervals are nominal: early stopping does not provide
simultaneous or sequentially adjusted confidence. No language-only conclusion,
remote-platform qualification, new API, SIMD work or downstream integration is
implied.

For matched before/after measurements, set `MESHOPT_ENCODER_BASELINE_BINARIES`
to a retained initial-source epoch (containing its `build.json` and profile
binaries) before preparing the candidate epoch. This adds the original Rust
binary to the same rotating measurement stream, under identical calibration
and iteration counts. Set `MESHOPT_ENCODER_BASELINE` to its `report.json` when
running `report_profiles.py`. Completed streams and their source manifests are
never replaced. Peer before/after epochs are independent refreshes; native
before/after rows are paired in the same stream.

`exp_edges.py` adds untimed scalar-oracle checks over finite exponent fields,
all 24 precisions and four modes, six strides, zero carry and both APIs. Set
`MESHOPT_EDGE_PROFILE=defaults` or `downstream`, with the candidate artifact directory.
The unit regression compares coherent bulk quaternion rotations with the
unchanged single-record kernel for all four selected components and every
supported precision; the retained extra checks compare these with scalar C++.

`exp_exhaustive.py defaults` (or `downstream`) compares every normal mantissa and
both signs at precisions 1..22 with scalar C++ 1.3: 369,098,752 quantized
components per profile. The normal exponent is representative of the exact
power-of-two scaling domain; `exp_edges.py` covers the exponent guards,
zero carry and the general fallback separately.

The native allocating index/sequence oracle derives the same range-dependent
bound as Rust, including its required OR scan inside the timed call. Earlier
native receipts used a fixed worst-case 32-bit bound and therefore different
allocation sizes; they remain legacy evidence. Caller-buffer capacity semantics
are unchanged. The peer adapter continues to use its existing known vertex
count and is reported separately; its original workload is not slowed to meet
a target.
