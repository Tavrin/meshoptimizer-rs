# Reproduce the meshopt 0.6.2 comparison

Unpublished measurement consumers; no library changes. Read
`../../parity/COMPARE_MESHOPT_CRATE.md` for results and scope.

Use `CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-cmp`.
Every build and collection runs sequentially inside
`MOSS_HEAVY_GPU=0 MOSS_HEAVY_DISK_FLOOR_GB=26 MOSS_LANE=meshopt-cmp
/mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ...`.
The wrapper uses rounded integer GiB; 26 keeps admission above 25 actual GiB.
The scripts additionally enforce a 25 GiB byte-level floor. Pause and requeue
when the floor holds; never rerun completed streams or discard pairs.
The original campaign is complete evidence once finalized; collection scripts
refuse to replace its completed streams. For a fresh campaign, preserve that
directory and give the helpers a new, consistent `ART` directory. Re-generating
the existing report uses its retained source inventories and measurements.

1. `python3 parity/compare/build.py defaults ours`, then `defaults theirs`,
   `moss ours`, and `moss theirs`. Each backend/profile starts with an empty
   lane target, builds the decode consumer once, touches that consumer once
   for a rebuild, and builds the API-root and timing consumers. Never delete
   another lane's cache. Build scripts retain binaries before the next clean.
2. `python3 parity/compare/measure.py prepare` freezes existing `runner.grid`
   / `performance.geometry` inputs and produces lossless codec/filter fixtures.
   After final driver builds, run `seal.py` once to seal the source epoch.
3. `python3 parity/compare/measure.py defaults outputs` checks every shared
   family against both implementations, including independent validity checks.
4. `python3 parity/compare/measure.py defaults timing`, then `moss timing`.
   Resident drivers share a selected physical CPU; algorithm setup, validation,
   allocating outputs, required copies, scratch and destruction are timed.
   Input parsing/generation, process launch, serialization, output normalization,
   fixture encoding for decode cases, and independent checks are excluded.
   Workspace capacity stays warm. Five alternating pairs are retained; a
   nominal paired log Student-t 95% interval straddling equality extends once
   to twenty pairs. This stopping rule does not establish simultaneous or
   sequentially adjusted statistical significance. All completed samples stay.
5. Run `platforms.py` through the same CPU admission, then `report.py` and
   `audit.py`. The target checks are compile-only and retain expected missing
   C-toolchain failures. Run the audit before committing the documentation
   because it verifies the measured revision as well as source hashes.
6. Run `export.py` to retain compact results and evidence hashes in the repo.
   Record the final queue receipts, and
   delete only the specified comparison Cargo target.

The C++ comparison uses allocating Rust adapters around the *published crate's*
public `meshopt::ffi` entry points. It does not claim to measure every idiomatic
wrapper: those wrappers may change return types, copy inputs or run an additional
meshlet optimization. Versions, filter precision and meshlet parameters are
explicitly matched. `vertex-encode-default` compares 1.3's v1 default to 0.25's
v0 default; the matched v0/v1 cases are reported separately.

The full API binary retains addresses of exported functions at runtime, including
ours' caller-buffer forms, experimental features, cluster LOD and parallel APIs.
It measures retained code, not an application workload, stack/heap use, or dynamic
system libraries. Decode-only uses default crate features. Footprint is the
stripped executable size and ELF section deltas from a matched empty consumer.

Raw sources, inputs, outputs, binaries, host metadata, build logs and paired
samples live in `/mnt/linux-extra/moss-scratch/meshopt-cmp`; MILESTONES.md records
progress and rejected measurements. Timing results are Linux x86-64 measurements
on a shared host. Historical upstream 1.3 qualification is not rerun here.
`measurement-source.json` seals the drivers, helper code and library source;
`driver-final-*.json` and `.sha256` files bind retained executable identities.
