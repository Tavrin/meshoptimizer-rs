# Stable fuzz smoke

`parity/fuzz.sh --seconds 600 --seed 20261002` builds and runs the five stable
bounded mutation targets concurrently for ten elapsed minutes each. The
machine has no installed cargo-fuzz/nightly toolchain, so this is the spec's
stable fallback. It has no libFuzzer coverage guidance or sanitizer
instrumentation and does not satisfy the later 24-CPU-hour release gate.

Inputs vary triangle lengths, indices, finite and arbitrary float bits,
thresholds, layouts, flags and storage/work limits. Successful calls must
preserve optimizer triangle multisets, simplifier index bounds and error
finiteness, resource limits, caller-buffer tails and variant output identity. Failed in-place calls must preserve their input.
The harness itself bounds allocation to small buffers.

The record in `parity/results/fuzz.json` includes per-target execution and
success counts, elapsed/user/system CPU time, maximum RSS, seed, captured
output, and pre/post source, dependency and executable identities. Crashes
and invariant failures stop the target and retain its stderr. The panic hook
prints the target, seed and iteration. Reproduce the prefix with the built
`vertex_cache` or `overdraw` binary's arguments: elapsed seconds, seed, and
maximum executions. A nonzero initial seed is required.


The cluster milestone adds the safe `build_dag_into` target. Run it with the
same CPU-accounted release harness:

```sh
parity/fuzz.sh --target build_dag_into --cpu-hours-per-target 4 --jobs 8 --seed 20261025
```

This target mutates indexed meshes, finite/arbitrary position bits, canonical
configs, hierarchy widths, workspace limits and each caller-buffer capacity.
It includes too-small buffers and checks successful prefixes, index/source and
record ranges, untouched tails, external guards and workspace accounting.
The stable seeded harness retains replay seeds and execution counts; it has no
coverage guidance or sanitizer instrumentation.

`parallel-clod/target.rs` is the separate `parallel_clod` libFuzzer source for
intra-mesh cluster LOD identity. It bounds meshes and topology, varies float
bits, limits, dilation, callbacks and worker counts, and compares exact output
words, errors, usage and mutable positions on reused workspaces. This feature
lane adds the source only; execution belongs to the dedicated fuzz lane.
