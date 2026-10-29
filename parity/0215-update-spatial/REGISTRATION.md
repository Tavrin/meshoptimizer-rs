# Registered instruction campaign

Baseline: v0.2.14. Campaign 1 candidate: the two source changes described in
PROOFS.md. Campaign 2 candidate: spatial only, after reverting the update W
change on the mandatory regression gate. No optimization edit follows results.
Functions: simplify-update and meshlets-spatial only. Arms: base, candidate,
cpp (M7, C++ 1.3), rust (optimesh 1.1.0). Inputs: the unchanged five frozen
corpus inputs and three real inputs from the peers harness. Settings and
allocation policy use the existing qualification driver adapters unchanged.

Metric: median of three resident-loop `perf stat -e instructions:u` samples,
`LC_ALL=C`, 100% counter running required. Interleave arms and reverse the
middle repetition. CPU 9, compiler release profile from parity/peers, no
target CPU override. No wall timings. Count allocation and destruction;
exclude input parsing and serialization with the existing perf control protocol.

Per function, three-real-mesh geometric mean candidate/optimesh <= 0.99 and
candidate/base <= 0.97 means SHIPS; only candidate/base <= 0.97 means
IMPROVES; otherwise NO-GAIN. Any case candidate/base > 1.01 forces a revert.
No implementation edits after campaign results except reverts; at most two
campaigns. The inherited single-input screens and sampling profiles are
development diagnostics, not campaign results.

The collector appends and fsyncs each accepted sample, skips completed keys,
verifies binary/source/input bindings on restart, retains interrupted files,
and rejects unstable outputs and counter multiplexing. Campaign 1 uses a fresh
artifact directory. A second campaign, if needed, must use a separate directory.
Set `LANE_CAMPAIGN=campaign2` for that directory. The first candidate, proof
sources and binaries are archived before preparing the second candidate;
original campaign 1 samples, bindings and regression receipt remain intact.

Invocation, after setting scratch, target and perf executable environment:

```sh
bash parity/0215-update-spatial/run.sh qualify
bash parity/0215-update-spatial/run.sh checks
bash parity/0215-update-spatial/run.sh cpp
bash parity/0215-update-spatial/run.sh fuzz
bash parity/0215-update-spatial/run.sh measure
```

Every invocation runs inside the required CPU-only admission wrapper with
4 GB declared. Scratch paths are supplied by environment, never embedded here.

## Check-lane campaign 3 (registered before measurement)

A later check lane was authorized by the coordinator to retry the targets after
both original campaigns. This is a third campaign and a recorded deviation from
the two-campaign limit above; campaigns 1 and 2 and their verdicts are kept
unchanged. Candidate: the spatial-only source change in PROOFS.md, "Check lane:
spatial"; update source is identical to v0.2.14, so its cells serve as a
noise control. Arms, inputs, metric, collector, CPU and decision rule are
exactly those above, applied verbatim. Set `LANE_CAMPAIGN=campaign3`; public
results go to `parity/0215-update-spatial/check/`. The single-input screens and
sampling profiles that motivated the change are diagnostics, not results. No
implementation edit follows campaign 3 results except a revert.
