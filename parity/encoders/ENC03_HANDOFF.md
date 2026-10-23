# SPEC-03 draft handoff — work mode ON

Implementation commit: `cc73d4050119ae208a8a7bb5cabae73adcbd0071`. Final correctness checks pass. Final-source
native and complete peer performance qualification is **pending**, so the spec’s
performance done-when condition has not been established. The four-lane-day cap
was not exhausted. No benchmark was started after work mode was enabled.

The P03 report separates earlier source `a6fc95f` full native/peer results from
the final `seen-frontier` diagnostic index/Exp scout. Do not use the earlier
native pass or the scout as final acceptance. All failed, rejected and
unresolved epochs remain under `/mnt/linux-extra/moss-scratch/meshopt-enc03`.

After Etienne explicitly turns work mode off, run headlessly from this worktree:

```sh
cd /home/etienne/dev/meshoptimizer-rs-wt/enc03
MOSS_HEAVY_USE_DECLARED=1 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/measure.sh
```

The retained controller exports all required environment variables, wraps every
phase in CPU-only `moss-heavy.sh 3 timeout 840`, records distinct burst logs and
runs these phases in order: exact historical request in defaults/Moss; all 111
native encoder cases in defaults/Moss; all 40 shared peer cases in defaults/Moss.
It checkpoints complete prefixes after 690 seconds. Run it without an outer
heavy admission scope so each burst releases admission independently. The final
report is produced only after every phase completes. No quiet-machine wait.
The fixed target directory has been deleted; retained binaries and fixtures are
outside it, and these timing commands do not rebuild Cargo artifacts.

The fully expanded individual phase commands are below. Export the environment
from the controller first if invoking them individually. A completed stream must
never be repeated; pending interrupted cases require explicit disposition.

```sh
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh reproduce defaults
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh reproduce moss
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh timing defaults
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh timing moss
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh peer defaults
MOSS_HEAVY_USE_DECLARED=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 3 timeout 840 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/phase.sh peer moss
python3 parity/encoders/report_profiles.py
```

A/A unresolved cases or breached bars block qualification; do not select the
better of the scout and the confirmation. Source guards and binary hashes must
still match. After measurement, refresh P03/CHANGELOG and portable evidence,
then reassess the draft. No publish, tag, merge or downstream runtime is covered.

Machine-readable handoff: `/mnt/linux-extra/moss-scratch/meshopt-enc03/handoff.json`.
Raw final receipts: `/mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier`.
Portable evidence: [summary](../results/enc03-allocating-profiles.json) and
[compressed receipts](../results/enc03-allocating-profiles.json.gz).
