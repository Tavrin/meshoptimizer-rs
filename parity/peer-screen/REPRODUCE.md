# Bet 0 peer screen

This extends `parity/compare/` and its resident driver, geometry generators and
independent validators. The published peers are optional dependencies of that
unpublished consumer only. Library source and dependency manifests stay unchanged.

Start a fresh campaign under `/mnt/linux-extra/moss-scratch/meshopt-peer-screen/`.
Write its registration and amended safety admission before building. Verify the
run-1 tree and hashes exactly as the spec requires; enable neither optimesh oracle
feature nor meshopt generate_bindings. Never replace completed campaign streams.

Set `MESHOPT_COMPARE_ARTIFACTS` to that campaign and
`CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-peer-screen`.
Use the README's admission settings: `MOSS_HEAVY_GPU=0`,
`MOSS_HEAVY_DISK_FLOOR_GB=26`, declared 4 GiB, and this lane's `MOSS_LANE`.
Every heavy invocation goes through the installed `moss-heavy.sh`, with a script
that calls Cargo using `--manifest-path`. The wrapper may reserve more than the
4 GiB declaration from its learned history; this screen does not override it.
The six-hour lane cap sums admitted heavy.log runtimes (including all builds and
preflights) plus the current admitted burst; queue waiting is excluded.

`peer_build.py defaults O scalar` builds missing native profile/arm/mode entries
sequentially within the admission. It copies identical library/driver sources to a
second path for A/A and builds the Wasm modules and scalar C++ stream reference.
Each executable, command, source hashes and build log is retained before proceeding.

Use `peer_campaign.py prepare`, then `correctness`. The historically named
`peer_campaign.py browser-correctness` stage only retains raw modules and applies
REPORT-3's `wasm-opt -Oz --enable-simd` postprocess; it does not collect browser bytes.
Preparation reproduces the five original frozen input hashes and retains the full stride-64 real
replay vertex/index blobs and recorded call parameters, retaining provenance. The source epoch and selected physical
CPU are sealed. Review output labels before invoking `peer_campaign.py timing` and
then browser byte checks and timing using the admitted calls below. Native timing first collects the single A/A set in Moss
SIMD on simplify, meshlets and meshlets-spatial; its three family GMs define F.
These stages acquire admission themselves; measurement stages
set `MOSS_HEAVY_MEASUREMENT=1`. A train waiter produces PAUSE between cases/streams;
checkpoint resumes preserve completed rows and pairs.

`peer_report.py` applies the registration and writes the per-family table and compact
JSON. `peer_audit.py` checks evidence identities, ordering, stopping, validity labels,
F and exact verdict inequalities. These helpers never collect new samples.

The browser extension retains REPORT-3's frozen 42 streams, RustArm/JsArm transport
classes and muted headless CDP driver. It substitutes registered alternating pairs
for the historical unpaired timing loop and checks fresh scalar C++ 1.3 reference
bytes. Browser correctness and timing have separate receipts. Loopback-only HTTP
persists pair receipts and reports train waiters. No desktop window is opened.

After retaining all measured native/Wasm binaries, remove only the designated Cargo
target. Preserve every campaign, failed build, infrastructure failure and PAUSE.
Unmeasured families receive explicit reasons; no historical ratio fills a gap.

For browser execution, call `peer_campaign.run` from a controller, once native
measurement has finished:

```python
from peer_campaign import run
run('browser-correctness-moss', 'peer_browser_run.py', ['correctness', 'moss'], True)
run('browser-timing-moss', 'peer_browser_run.py', ['timing', 'moss'], True)
```

Run 2 exhausted both default-profile browser correctness attempts before any byte
check. Those families are terminally unmeasured in this campaign. Do not retry them
using the later Moss navigation fix. `run-qualified-stages.py` in the retained
campaign is the exact controller used for the remaining native and Moss browser
stages. It preserves every completed checkpoint.

Final report supplements in the campaign (`supplement-output-metrics.py` and
`final-report-supplement.py`) read retained outputs only. `run-audit-cached.py` runs the unchanged audit with a hash cache to avoid rereading
identical binaries/inputs for each sample. No supplement executes a benchmark.

Run 2 ended at an external lane hold until 2026-10-26 00:57:19 CET. The queued
controller was stopped at a case boundary; no live pair was interrupted. Remaining
families are terminally unmeasured in this campaign, not candidates for a verdict
rerun. The six-hour admitted-time cap was not exhausted. Report generation and the
cached audit were lightweight receipt-only checks; no build or measurement ran
outside admission. The initially planned admitted finalization was replaced by
these checks because the lane was held. The audit passed without failures.
