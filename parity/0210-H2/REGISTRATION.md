# H2 instruction gate

Baseline: v0.2.9 (`df7fc567b6540eb06cb583e2d323c01e5797609c`).
Candidate: this job, safe Rust, std, no SIMD feature. Peer: meshopt 0.7.0,
vendored meshoptimizer 1.3, `MESHOPTIMIZER_NO_SIMD`.

Use the archived `bench/peer-screen` bet 0 harness, with the retained NAS replay
adapter. The adapter adds counter-only resident control, corrects the peer scale
stride (12 -> supplied stride), and suppresses unused parser allocations for
replayed inputs. All arms share exactly the same adapter and profile: opt3,
thin LTO, one codegen unit. Counter: perf 6.17 `instructions:u`, CPU9, resident
ready/enable/count/disable protocol, setup and output serialization excluded.
No wall timings. Save all samples and binary/source/input hashes.

Frozen cases: five synthetic cases per family, plus the three real overdraw
fixtures. Scale inputs use 64-byte little-endian records. Real chains: all scale
and overdraw calls, with identical captured input/parameters deduplicated for
measurement and their multiplicities retained for stage totals. Reorder stage
counts include the saved-input clone and output copy, as in the retained adapter.

Take the median of 3 per arm. Verdicts, per family:
- **PASS:** candidate/M7 GM ≤1.00.
- **PARTIAL:** candidate/base GM ≤0.95, but M7 not reached.
- **NO-GAIN:** candidate/base GM >0.95. Revert that family's change.
- **FAIL:** any case with candidate/base >1.01, regardless of the other verdicts. Fix it or revert.

Frozen family GMs and chain stage totals will be reported separately. Also
report a GM over every distinct case for each family, without giving duplicate
chain calls additional votes. Apply the per-case regression gate to every case.
Pilot counts guide revisions; retain them and keep them separate from the final
triplicate campaign. Functional tests and fuzz throughput are not timing claims.
