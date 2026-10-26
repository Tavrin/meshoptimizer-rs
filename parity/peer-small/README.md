# SPEC-027 lane C evidence

All heavy commands run from this worktree through
`/mnt/linux-extra/moss-coord/bin/moss-heavy.sh`, declaring at most 4 GB,
with `MOSS_HEAVY_GPU=0` and
`CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-peer-c`.
Use a lane-specific `MOSS_HEAVY_CLASS` for build, instructions, proof and gates
so unrelated Python jobs do not determine this lane's reservation history.

The comparator is exported unchanged from `bench/peer-screen` into
`/mnt/linux-extra/moss-scratch/meshopt-peer-c/parity/compare`. Separate scratch
manifests use that source with an absolute dependency path to the archived
v0.2.6 or this worktree. No comparator code changes. Offline lock regeneration
is necessary because the exported branch's local dependency is an older crate
version; both before/after lockfiles are retained in scratch. Frozen inputs
are copied from the retained bet-0 campaign and SHA-256 checked against the
exported `results.json`.

Execution order:

1. `build.py base X M7` retains immutable scalar Moss binaries and source hashes.
2. `instructions.py EPOCH BINARY...` runs admitted CPU-7 resident counter
   measurements. Elapsed replies are discarded; failed counter setup attempts
   and rejected candidate measurements remain in scratch.
3. Freeze library source. `build_proof.py` builds the existing C++ 1.3
   protocol drivers, native/JS fixture exporters, and separate baseline/candidate
   exact-Result transports. Oracle checkout is clean and pinned by the existing
   reference guard. It also exports the first 256 inputs per relevant existing
   preprocessing fuzz seed, using the original RNG and mutations.
4. `proof.py CANDIDATE_BINARY` compares all frozen family outputs against the
   archived baseline and M7, existing C++ differentials for affected functions,
   preprocessing fuzz inputs, the original p03 smoke seed and deterministic
   mutations, and exhaustive small resource-limit transitions. The latter
   include public Results, usage, caller-buffer contents and atomic variants.
   Typed fetch boundaries are covered separately. Raw outputs are retained;
   large limit output is compressed losslessly. Protocol timing/storage trailers
   are excluded from algorithm byte comparisons; resource usage is checked by
   direct API probes.
5. `gates.sh` records fmt, all-feature tests, and all-target/all-feature clippy
   with warnings denied. No 10k sweep or timed fuzz campaign.
6. `campaign.py CANDIDATE BASE X` under
   `MOSS_HEAVY_MEASUREMENT=1` implements REGISTRATION.md. It creates its receipt
   exclusively and refuses another campaign. All ten pairs, calibrations,
   per-case intervals, family intervals and the same-session floor are retained.
7. Retain compact receipts in this directory, push a signed-off Tavrin commit
   and draft PR, append the spec resume, and delete the designated target.

The scratch baseline and oracle sources are private proof inputs, not changes
to another lane. This lane never writes another lane's worktree.
