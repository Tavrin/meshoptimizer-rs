# H3 instruction registration

Pinned base: v0.2.9 / df7fc567b6540eb06cb583e2d323c01e5797609c.
Families: partition_clusters, build_meshlets_spatial only. Five frozen cases each:
tiny-smooth, medium-smooth, medium-seams, medium-sparse, million-smooth.
Driver: bet 0 comparator exported with git archive from bench/peer-screen, with
retained count-only replay adapter from the read-only NAS meshopt-028-L2 archive.
Three real chains: inspect stages.json, include every applicable H3 call;
record not applicable if the trace contains no such calls.

Scalar std, opt3, thin LTO, one codegen unit, M7 meshopt 0.7.0 / C++ 1.3,
MESHOPTIMIZER_NO_SIMD. Installed perf 6.17, instructions:u only. Resident
ready/enable/count/disable/stop protocol. CPU9; three rotating repetitions per
arm, medians per case, equal-case geometric means per family. Iterations:
tiny 10000, medium 100, million 4. Counter running >=99%. All output equality
checks occur before enable; no wall timings. No favorable sample selection.

Apply verbatim, separately per family: FAIL if any candidate/base >1.01;
otherwise PASS if candidate/M7 GM <=1.00; otherwise PARTIAL if candidate/base
GM <=0.95; otherwise NO-GAIN. Revert every NO-GAIN or unresolved FAIL family.
Retain rejected campaigns separately; rerun the entire final campaign after
any implementation change. No threshold changes.

All heavy jobs: CPU-only moss-heavy, <=4 GB declared; target
/mnt/linux-extra/moss-cargo-targets/meshopt-h3, deleted at completion.
Scratch: /mnt/linux-extra/moss-scratch/meshopt-0210-h3.
