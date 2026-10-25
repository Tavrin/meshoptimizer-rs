# moss-qx5g3 r2 overdraw qualification

Verdict: **FAIL**. Not a release candidate.

Implementation: `d74b914194a7830975e6877a4e70be294458ec54`. Baseline: `afe803bbf60796a7af6b5001594a979af009cfac` (0.2.3). Draft PR: https://github.com/Tavrin/meshoptimizer-rs/pull/10. r1 FAIL remains unchanged.

Evidence root: `/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/r2`. Exact registration: REGISTRATION.md, timestamped before any r2 sample. Source and executable manifests bind every run. Final documentation changes do not change the tested implementation source.

## Diagnosis and changes

Measured stage wall Rust/C++: 214e 0.319310/0.206382 s; b6fd 0.161409/0.093344 s. User-cycle attribution estimates, not independent loop wall timers. Full counts, limitations and candidates: DIAGNOSIS.md and diagnosis.json.

| Rank | Primary loop/function | Estimated s |
|---|---|---:|
| 1 | Mesh centroid scan | 0.074775 |
| 2 | Indexed position reads | 0.066691 |
| 3 | Remaining kernel initialization / unresolved inline work | 0.056587 |
| 4 | Validation | 0.050524 |
| 5 | Hard/soft boundaries | 0.036377 |

Frozen allocating case (100 diagnostic calls): Rust total 1.999816 s, wrapper allocation/zeroing 0.094769 s, validation 0.090901 s, kernel 1.763860 s (including pinned sqrt 0.410020 s and approximately 0.1083 s of output copies), unresolved harness/wrapper 0.050286 s. Symbolized C++ total 1.193103 s. Allocating wrapper has no index-copy pass of its own. Whole-process attribution includes warmup/harness uncertainty; most excess is in the kernel, not demonstrated to be an unavoidable allocation contract cost.

Fixes landed in predicted-gain order:

- `18732cc`: accumulate centroid during the required finite-position scan; retain addition order and original later numerical checks/divisions and exact work-charge prefix.
- `d74b914`: specialize checked triangle position reads once per cluster, including byte order and unaligned offsets; batch per-triangle work through existing Work::scan with exact failure-prefix handling.

Dead byte-exact hypotheses: none. The performance hypothesis that these two fixes would close the real overdraw bar was disproved (214e and b6fd remain above 1.50); no post-campaign tuning was attempted. Both fixes passed per-fix native/WASM differential checks (50/50/65/62/52 cases, zero mismatches). The lower-gain third candidate was deferred before the registered campaign. No algorithm, float-expression, pinned sqrt, allocation contract, API, Limits, Workspace, corpus or bar change. r1 fixes remain. No simplifier/cache/encoder/SIMD/Moss implementation work. Per-fix gains were not isolated. Regression setup corrections are in BUILD_NOTES.md.

## Registered real cases

One warmup pair and ten interleaved pairs per baseline/final arm and case, CPU 8, sibling 9 excluded from fuzz workers. No timing-based exclusions or reruns. C/B denotes Rust/C++ paired ratio. Intervals: 10,000 paired bootstrap resamples, fixed seed 20261025. Full raw ratios, backend medians and ratio-of-medians: timings.json; raw events: campaign/raw.jsonl.

| Case | Stage | 0.2.3 C/B [95%] | Final C/B [95%] |
|---|---|---:|---:|
| 214e | simplify | 1.380 [1.368, 1.410] | 0.963 [0.940, 0.974] |
| 214e | overdraw | 3.281 [3.242, 3.422] | 1.535 [1.495, 1.579] |
| 214e | scale_raw | 3.796 [3.685, 4.157] | 2.222 [2.091, 2.402] |
| 214e | cache | 0.926 [0.899, 0.946] | 0.941 [0.874, 0.998] |
| b6fd | simplify | 1.298 [1.267, 1.304] | 0.947 [0.921, 0.967] |
| b6fd | overdraw | 3.011 [2.940, 3.079] | 1.596 [1.556, 1.613] |
| b6fd | scale_raw | 3.051 [2.975, 3.262] | 1.788 [1.707, 1.881] |
| b6fd | cache | 1.365 [1.296, 1.395] | 1.423 [1.383, 1.452] |
| a082 | simplify | 1.052 [1.038, 1.086] | 0.941 [0.932, 0.954] |
| a082 | overdraw | 2.272 [2.250, 2.314] | 1.467 [1.452, 1.485] |
| a082 | scale_raw | 3.740 [3.663, 3.941] | 2.179 [2.133, 2.202] |
| a082 | cache | 1.095 [1.076, 1.122] | 1.111 [1.093, 1.128] |

Three-case C++ 1.3 record/hash checks: True. Each of 44 replays per case is compared, including legacy records; real-hash-checks.json retains digests. Replay processes produced matching output hashes; the optional independent post-run blob rehash is NOT RUN as recorded below.

## Frozen unchanged matrix

Both arms use the identical one-line 600-second resident read/readiness timeout committed in cc4ff36. No sample policy, workload, corpus or threshold was changed. Full baseline source is a scratch archive of afe803b. The 204 cases cover all five geometry families. The matrix retains its pre-existing memory metrics and gate result separately.

| Family | Baseline GM / max | Final GM / max | GM change |
|---|---:|---:|---:|
| vertex_cache | 1.132886 / 1.205343 | 1.124791 / 1.195407 | -0.715% |
| overdraw | 1.104734 / 1.495309 | 1.086187 / 1.444646 | -1.679% |
| simplify | 1.055605 / 1.217006 | 1.043579 / 1.197683 | -1.139% |
| simplify_with_attributes | 1.074811 / 1.238512 | 1.071103 / 1.225558 | -0.345% |
| simplify_scale | 0.940649 / 1.067233 | 0.784758 / 0.915833 | -16.573% |

Frozen infrastructure reruns: []. A measured value never triggers a rerun.

Other infrastructure recovery: the replay harness initial fat-LTO link hit the 7 GiB scope cap before any real timing sample. A separate build with 9 GiB declared RAM retains its own receipt. This is build recovery, not a frozen-arm or timing-value rerun. The first a082 final warmup also experienced scope reclaim (177.546 s cache wall); runtime-pressure.jsonl records the pressure and automatic wrapper limit growth, with no OOM. The warmup is retained and excluded only by the registered warmup rule. No measured sample was replaced.

## Coordinator RAM hold

Coordinator steer recorded in STATE.md and coordinator-hold.json: train 11 needed RAM; no new moss-heavy job while r2/HOLD exists. The agent left both running jobs untouched. The coordinator later stopped fuzz to admit train 11; one authorized same-source/binary/seed/core --resume continuation completed the remaining CPU budget with exit 0. This was not a fuzz failure or an infrastructure rerun. A blocking shell loop checks every 120 seconds. Hold-caused events are ineligible for infrastructure reruns. Observed removal: 2026-10-25T08:44:12Z. Retained-buffer verification status: NOT RUN: Deferred for train 11 RAM priority; optional audit never started.

## Proof commands and exits

All builds/tests/benchmarks use `/mnt/linux-extra/moss-coord/bin/moss-heavy.sh` and the prescribed isolated target. Measurement jobs set MOSS_HEAVY_MEASUREMENT=1.

- `parity/gates.sh`: exit 0.
- `parity/run.sh --phase 0.1 --profile scalar-strict`: exit 0.
- `parity/sweep.sh --phase 0.1 --cases-per-family 2000 --seed 20261002`: exit 0.
- `parity/js.sh --phase 0.1 --require-coverage`: exit 0.
- Initial `parity/fuzz.sh --phase 0.1 --cpu-hours-per-target 4 --jobs 8 --seed 20261002 --wall-seconds 16268`: wrapper exit 2 after coordinator scope stop; NOT a fuzz failure. Completed shards had zero crashes. Interrupted accounting files and checkpoint are retained separately.
- Authorized `parity/fuzz.sh --phase 0.1 --cpu-hours-per-target 4 --jobs 8 --seed 20261002 --resume --wall-seconds 8401`: exit 0; passed; cumulative four CPU-hours per target. Same artifact directory, source, binaries, seed and eight cores; this is part of the plan, not an infrastructure rerun.
- Logical fuzz gate: exit 0; passed; cumulative four CPU-hours per target.
- `r2/real-hash-checks.py`: exit 0.
- `r2/verify-retained.py`: NOT RUN: Deferred for train 11 RAM priority; optional audit never started.
- `parity/report.sh`: not run; not a gate.

Foundation subcommands (from retained gates.json):

- `cargo fmt --check`: exit 0.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: exit 0.
- `cargo test --locked`: exit 0.
- `cargo test --locked --all-features`: exit 0.
- `cargo test --locked --no-default-features`: exit 0.
- `cargo build --target wasm32-unknown-unknown --no-default-features`: exit 0.
- `cargo build --locked --no-default-features`: exit 0.
- `cargo +1.88 test --locked --all-features`: exit 0.
- `cargo +1.88 test --locked --no-default-features`: exit 0.
- `cargo package --list`: exit 0.
- `cargo publish --dry-run --allow-dirty`: exit 0.
- `cargo doc --locked --no-deps --all-features`: exit 0.
- `cargo doc --locked --no-deps --no-default-features`: exit 0.
- `cargo clippy --locked --manifest-path parity/Cargo.toml --all-targets -- -D warnings`: exit 0.
- `cargo clippy --locked --manifest-path fuzz/Cargo.toml --all-targets -- -D warnings`: exit 0.
- `cargo fmt --manifest-path parity/Cargo.toml --check`: exit 0.
- `cargo fmt --manifest-path fuzz/Cargo.toml --check`: exit 0.

Fuzz CPU budgets and executions:

```json
{
  "required_cpu_seconds_per_target": 14400.0,
  "worker_cpus": [
    0,
    2,
    4,
    6,
    10,
    12,
    14,
    16
  ],
  "passed": true,
  "coverage": "unavailable: existing stable targets have no coverage instrumentation",
  "runs": [
    {
      "target": "vertex_cache",
      "cpu_seconds": 14942.570000000002,
      "elapsed_seconds": 14999.98,
      "executions": 1648025892,
      "successes": 896555738,
      "crashes": 0,
      "shards": 50
    },
    {
      "target": "overdraw",
      "cpu_seconds": 15256.79,
      "elapsed_seconds": 15300.0,
      "executions": 3416969427,
      "successes": 1439243215,
      "crashes": 0,
      "shards": 51
    },
    {
      "target": "simplify",
      "cpu_seconds": 14949.47,
      "elapsed_seconds": 14999.99,
      "executions": 1676484544,
      "successes": 854308964,
      "crashes": 0,
      "shards": 50
    },
    {
      "target": "simplify_with_attributes",
      "cpu_seconds": 14947.629999999996,
      "elapsed_seconds": 15000.02,
      "executions": 1428992508,
      "successes": 554819075,
      "crashes": 0,
      "shards": 50
    },
    {
      "target": "simplify_scale",
      "cpu_seconds": 15233.069999999998,
      "elapsed_seconds": 15300.01,
      "executions": 41116728163,
      "successes": 38753438064,
      "crashes": 0,
      "shards": 51
    }
  ],
  "identities_unchanged": true
}
```

## Decision table applied verbatim

- real_case_targets: **False**.
- both_frozen_complete: **True**.
- final_frozen_family_bars: **True**.
- three_families_within_2_percent_baseline: **True**.
- all_required_spec_proof: **True**.

All required checks must hold. Verdict: **FAIL**; no PARTIAL overall verdict, retuning, or timing-value reruns. A coordinator-interrupted fuzz budget may separately be marked PARTIAL under the later steer.

Current proof status: fuzz exit 0; passed; cumulative four CPU-hours per target; retained integrity NOT RUN: Deferred for train 11 RAM priority; optional audit never started.

## Not verified

Optional independent retained-buffer rehash: NOT RUN: Deferred for train 11 RAM priority; optional audit never started. This extra audit is separate from the named spec gates. Individual per-fix timing gains; universal performance outside these registered inputs; other CPUs/platforms beyond the required native/WASM/MSRV gates; historical aggregate release report. Owner review, merge and release have not been performed. Any missing required proof above is a FAIL, not inferred from focused checks.
