# Stable fuzz smoke

`parity/fuzz.sh --seconds 600 --seed 20261002` builds and runs the two stable
bounded mutation targets concurrently for ten elapsed minutes each. The
machine has no installed cargo-fuzz/nightly toolchain, so this is the spec's
stable fallback. It has no libFuzzer coverage guidance or sanitizer
instrumentation and does not satisfy the later 24-CPU-hour release gate.

Inputs vary triangle lengths, indices, finite and arbitrary float bits,
thresholds, layouts, flags and storage/work limits. Successful calls must
preserve the triangle multiset and resource limits, caller-buffer tails and
variant output identity. Failed in-place calls must preserve their input.
The harness itself bounds allocation to small buffers.

The record in `parity/results/fuzz.json` includes per-target execution and
success counts, elapsed/user/system CPU time, maximum RSS, seed, captured
output, and pre/post source, dependency and executable identities. Crashes
and invariant failures stop the target and retain its stderr. The panic hook
prints the target, seed and iteration. Reproduce the prefix with the built
`vertex_cache` or `overdraw` binary's arguments: elapsed seconds, seed, and
maximum executions. A nonzero initial seed is required.
