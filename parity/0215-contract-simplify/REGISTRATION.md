# Qualification registration

Baseline: 0.2.13. The four arms are baseline, candidate, C++ 1.3 and
optimesh 1.1.0. Use the peers consumer's allocating calls, frozen packed
XYZ/index corpus and three real inputs, with the same simplify target and
error. Update uses positions only; prune uses error 0.5; points use half the
vertex count without colors. Record median instructions:u from three runs per
arm. A candidate/base ratio above 1.01 on any case fails. All three real meshes
must beat optimesh on simplify, attributes and overdraw for PASS. Otherwise,
all three at or below 0.90 on attributes or overdraw give PARTIAL; the
remaining outcome is NO-GAIN. No wall-time gate is used.

Proof compares separate 0.2.13 and candidate crates, including complete caller
buffers, errors and work/storage usage, then the pinned C++ differential and
five child CPU-minutes of differential fuzz. The reporting artifact must state
any missing or failed checks rather than infer qualification from a pilot.
