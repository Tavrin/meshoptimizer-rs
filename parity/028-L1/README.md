# SPEC-028 lane L1

Read REGISTRATION.md before the final wall campaign and REPORT.md for results.
All preparation, proof and measurements run through the installed moss-heavy
wrapper with MOSS_HEAVY_GPU=0, at most 4 GiB declared, and the exclusive
CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-L1. The retained
scratch directory is /mnt/linux-extra/moss-scratch/meshopt-028-L1.

The trusted-input peer driver is derived from bench/peer-screen's comparator.
Only its L1 output allocation boundaries are changed, by equalize.py. Its
existing FFI blocks are harness-only; no unsafe library code is added. X uses
public optimesh APIs and contains no copied optimesh implementation.

`prepare.py` archives the frozen comparator and v0.2.7, validates the five retained
input hashes against the frozen manifest, and refuses to overwrite preparation.
The committed adapters are already equalized; `equalize.py` records the original
transformation and is not a step to repeat on the committed files.

`control.py build EPOCH ARMS...` retains each Moss-scalar executable and build
receipt; arms are ours, base, M7 and X. `counts EPOCH ARMS...` counts only user
instructions during fixed resident calls through perf's enable/disable control
and discards all timer values. `profile EPOCH` records instruction samples for
source diagnosis using retained symbols. All six families and all five frozen
cases are always included.

`proof.py build` builds the baseline and candidate exact witnesses. `proof.py
final` compares frozen peer outputs and the existing 2,000 seeds/family, including
Result values, work usage and caller-buffer contents on success and failure.
The lane witness additionally varies strided float and little/big-endian byte
storage. `proof.py cpp` reuses the existing phase-0.3 differential protocol and
seed generator against the pinned upstream C++ reference for L1 only. `fuzz.py`
runs the widened generator until the two differential target processes consume
at least 300 CPU seconds; its JSON retains seed, count, hashes and replay command.

`gates.py` runs the required two feature configurations, clippy and formatting.
`campaign.py build-rfc` and `preflight-rfc` build/check the unchanged RFC113
cluster-C adapters with retained S1/S2 inputs. `campaign.py wall` is the single
registered campaign, with an exclusive marker and per-pair append-only receipts;
it includes A/A, O/M7, O/v0.2.7, X disclosure and RFC113 disclosure. No verdict
reruns or load-based exclusions are implemented.

`job.sh run COMMAND...` supplies the common serial-build environment inside an
admission. The actual submission logs and exact commands remain in scratch.
Retain binaries and receipts before deleting only this lane's Cargo target.

The spatial BVH's full-width invariant divider is additionally exercised by
`division_fuzz.py 300`. That generator includes the actual production helper
from src/meshlet_division.rs and compares it to native integer division for
random values, zero and values near usize::MAX, with divisors 1..512. Its budget
counts target user+system CPU, excluding compilation. The mesh generator's
budget separately counts its two resident target processes, excluding Python
orchestration CPU. Both generators retain deterministic replay descriptors.

`qualify.sh` collects the required proof before any wall requests and writes
QUALIFIED.json only after all proof exits are zero. The wall campaign checks
that source seal before and after sampling. `finish.py` exports compact raw
pairs, case intervals, verdicts and gate receipts to RESULTS.json and writes
the final report and measured-only changelog entry. `cleanup.py` verifies the
retained binaries before deleting exactly the lane's designated Cargo target.
