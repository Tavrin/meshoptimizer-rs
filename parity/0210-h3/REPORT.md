# H3 qualification — in progress

Base: v0.2.9, df7fc567b6540eb06cb583e2d323c01e5797609c.
Scope: partition_clusters and build_meshlets_spatial only.

Candidate techniques: family-local std scalar square roots with canonical
exceptional-input bits, keeping the no_std software memo; bounded slices and
iterators for spatial SAH area streams, box initialization, axis keys and side
marking; native integer division in split feasibility. No API, work-charge,
allocation policy or error-order change. No new unsafe code.

The pinned libm canonicalizes negative operands and NaN payloads. Bare hardware
sqrt is not bit-identical on every f32 input; the candidate explicitly preserves
that exceptional-input contract. The exhaustive checker extracts the actual
partition helper verbatim and checks every one of 2^32 bit patterns.

Qualification is pending CPU-only moss-heavy admission. No instruction verdict
or performance acceptance is claimed. The first build attempt exited 101 because
it began before archive extraction completed; no compilation or measurements
from that attempt are used. Its log is retained. A later queued 4 GiB build was
canceled and requeued with a 2 GiB declaration for single-job compilation.

All three extracted real traces (214e, b6fd, a082; 56 calls total) contain only
scale, simplify, cache and overdraw calls. H3 chain measurements are not
applicable; no substitute whole-chain performance claim is made.

Method and immutable thresholds: REGISTRATION.md. Scratch and complete logs:
/mnt/linux-extra/moss-scratch/meshopt-0210-h3/. Designated target:
/mnt/linux-extra/moss-cargo-targets/meshopt-h3, to be deleted after qualification.
The final report will replace this pending status with instruction tables,
per-family verdicts, proof results, source bindings and gate exit codes.

## First campaign (rejected)

| Family | candidate/base GM | candidate/C++ GM | worst candidate/base | Verdict |
|---|---:|---:|---:|---|
| partition | 0.976434 | 1.378546 | 0.985749 | NO-GAIN |
| spatial meshlets | 1.004852 | 1.169347 | 1.019618 | FAIL |

All 90 instruction measurements exited 0. Retained in scratch/rejected-r1/.
The root-only partition change did not meet the 5% threshold. The spatial pivot
iterator/native-division rewrite regressed the million-triangle case and was
removed from the revised candidate. No acceptance is claimed for either family.

The revised candidate adds partition heap/adjacency slices and a monomorphized
adjacency loop: precharge only a completely covered, infallible loop; otherwise
retain each original per-item charge. Fallible leaf scoring keeps its original
charge sequence. Spatial counting uses fixed triangle records and compile-time
output selection. The revised campaign remains queued; its script applies the
registered family reversions automatically before final qualification.
