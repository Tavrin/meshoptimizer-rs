# downstream-qx5g3 release check v2

**RELEASE**. Candidate `f40f5af`; baseline 0.2.3 `afe803b`; fallback `a7688ef`. Candidate src diff from r2 `d74b914` is empty.

Rules were revised after v1 NO RELEASE in light of the measured A/A floor. Candidate code is unchanged. z9kkq diagnosis and repro are context only; no historical timings feed F, T or a decision.

## Session and decision

Each comparison: CPU 8, one startup warm-up pair and exactly 12 measured AB/BA pairs (ABBA ordering); case median paired ratio; family geometric mean of case medians. Bootstrap percentile 95%, NumPy PCG64 seed 20261025, 10000 within-case paired resamples. All ordinary arms use fat LTO, codegen-units=1, empty Rust flags. B/B′ are identical source built in separate archive paths.

## C session

F = **0.093803340**, T = ceil_to_0.001(1 + F + 0.02) = **1.114**. Floor case: `simplify_with_attributes/million/sparse/allocating/a1/r0.5`; A/A median 0.906196660, 95% [0.819413986, 1.032837493]. F is the absolute deviation of this negative control shift; no causal attribution is inferred.

| Family | Cases | C/B GM | 95% interval | Regresses (>1.02) |
|---|---:|---:|---|---|
| overdraw | 24 | 0.984246 | [0.978449, 0.990056] | False |
| simplify | 72 | 0.990813 | [0.987572, 0.993462] | False |
| simplify_scale | 12 | 0.847715 | [0.843571, 0.853184] | False |
| simplify_with_attributes | 72 | 0.996145 | [0.991733, 0.998792] | False |
| vertex_cache | 24 | 1.004957 | [1.000902, 1.009067] | False |

### Flagged, replication and layout

| Case | C/B median [95%] | Cp/Bp median [95%] | Result |
|---|---|---|---|
| None: no C/B lower bound exceeds this session’s T. | — | — | No replication or layout case required |

Main + A/A + replication session wall time: 40.81 minutes, including any admission PAUSE. Raw records: `<scratch>/meshopt-simplify-perf/release-v2/C-attempt-1/record.json`.

## Alignment qualification

First option qualified: **`-C llvm-args=-align-all-functions=6`**. Both Bp and Cp build; Cp passed `parity/run.sh --phase 0.1 --profile scalar-strict` and the three real-case exact hash checks against retained C++ 1.3 outputs. Parity rebuild binary is byte-identical to retained Cp. All required symbols start at 64-byte alignment:

```text
Bp meshoptimizer_rs::cache::kernel::<false> 0x36180 mod64= 0
Bp meshoptimizer_rs::cache::kernel::<true> 0x38580 mod64= 0
Bp meshoptimizer_rs::overdraw::kernel 0x50a40 mod64= 0
Cp meshoptimizer_rs::cache::kernel::<false> 0x36ac0 mod64= 0
Cp meshoptimizer_rs::cache::kernel::<true> 0x38ec0 mod64= 0
Cp meshoptimizer_rs::overdraw::kernel 0x52980 mod64= 0
```

Full raw symbol tables: Bp-symbols.txt and Cp-symbols.txt. Qualification records/logs are retained under the scratch evidence root. The scratch runner preserves the requested flag instead of resetting it. Its inherited descriptive rust_profile sentence says empty flags; Cargo fingerprints (retained), explicit build flags and aligned symbols establish the actual option; parity/src and src are unchanged. The real-case harness only redirects its dependency to the candidate archive. Initial build setup failures, cancellation, unqualified binaries, and the real-case checker build OOM (7 GiB cap, wrapper retry refused an immutable completed parity receipt; unfinished real-checker build resumed alone at 11 GiB) are retained in `build-preflight-incident.json`; these occurred before measurement and did not consume a measurement rerun.

## Absolute bars and inherited correctness

r2 complete frozen-final matrix: `<scratch>/meshopt-simplify-perf/r2/frozen-final-1/benchmark.json`; SHA-256 `c3034c1bc6b3130d9c91fe3a704859505bf1f60a84522809036d033a37245fbd`; 204/204 cases. Family GM ≤1.25 and non-exempt case max ≤1.50: **True**. Named million/disconnected/allocating overdraw exemption retained exactly as v1. No new C++ timing.

| Family | GM vs C++ 1.3 | Max |
|---|---:|---:|
| vertex_cache | 1.124791 | 1.195407 |
| overdraw | 1.086187 | 1.444646 |
| simplify | 1.043579 | 1.197683 |
| simplify_with_attributes | 1.071103 | 1.225558 |
| simplify_scale | 0.784758 | 0.915833 |

Inherited candidate correctness and fuzz: r2/REPORT.md, r2/source-continuity.json, r2/parity-final/exits.tsv, r2/final-tracked-summaries/{run,sweep}.json, r2/fuzz-final/fuzz.json; bindings in ../release-check/inherited-proof-binding.json. Native/WASM/sweep/JS and ≥4 CPU hours per fuzz target, zero crashes are inherited, not rerun. Only the alignment-option phase 0.1 and three real-case checks are fresh.

## Receipts and scheduling

[Registration](REGISTRATION.md), [analysis with every case and A/A interval](analysis.json), [all raw measured pairs](SAMPLES.csv), [protocol verification](verification.json), [source/profile/binary identities](identities.json). Scratch evidence root: `<scratch>/meshopt-simplify-perf/release-v2/`. Full raw resident responses, archive sources, executables, scripts and wrapper logs remain there. No perf profiles were required or collected because the flagged set is empty. No evidence was deleted. All six private Cargo target directories were removed after retaining every ELF/WASM binary by content hash. [Cleanup receipt](cleanup.json), [independent audit](independent-audit.json), [admission receipts](heavy-receipts.tsv), and [frozen protocol scripts](protocol/) are retained. The independent audit verified 4,896 unique raw pairs, recomputed all case medians and family GMs, and checked every registered script and arm binary hash. The first generic-python cleanup submission was canceled while queued because it inherited an unrelated 9 GiB class estimate; the identical cleanup script ran through its own 2 GiB entry point. This occurred after measurement.

PAUSE events: 11. Every waiting train detected at a completed-case boundary released admission; no mid-pair pause or completed-case repeat.

```json
{"event": "PAUSE", "utc": "2026-10-25T12:02:21.393655+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 155, "AA": 0, "replication": 0}, "waiting": [{"pid": 1119437, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:04:54.690934+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 168, "AA": 0, "replication": 0}, "waiting": [{"pid": 1147712, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:18:31.985659+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 176, "AA": 0, "replication": 0}, "waiting": [{"pid": 1435486, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:17.313704+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 0, "replication": 0}, "waiting": [{"pid": 1528777, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:20.517505+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 14, "replication": 0}, "waiting": [{"pid": 1528414, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:27.715827+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 85, "replication": 0}, "waiting": [{"pid": 1533212, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:32.172929+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 85, "replication": 0}, "waiting": [{"pid": 1532365, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:38.041246+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 114, "replication": 0}, "waiting": [{"pid": 1538700, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:41.281546+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 116, "replication": 0}, "waiting": [{"pid": 1538283, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:25:46.729267+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 135, "replication": 0}, "waiting": [{"pid": 1541797, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
{"event": "PAUSE", "utc": "2026-10-25T12:29:01.054505+00:00", "arm": "C", "attempt": 1, "completed_cases": {"main": 204, "AA": 158, "replication": 0}, "waiting": [{"pid": 1577127, "cwd": "<worktree>", "queue": "heavy.waiting"}]}
```

Wrapper segment exits (75 is a train PAUSE, not a measurement failure):

```json
{"arm": "C", "attempt": 1, "segment": 1, "exit": 75, "unix": 1792929742.2126374}
{"arm": "C", "attempt": 1, "segment": 2, "exit": 75, "unix": 1792929895.2704234}
{"arm": "C", "attempt": 1, "segment": 3, "exit": 75, "unix": 1792930712.3585982}
{"arm": "C", "attempt": 1, "segment": 4, "exit": 75, "unix": 1792931117.7385325}
{"arm": "C", "attempt": 1, "segment": 5, "exit": 75, "unix": 1792931121.131018}
{"arm": "C", "attempt": 1, "segment": 6, "exit": 75, "unix": 1792931128.5810454}
{"arm": "C", "attempt": 1, "segment": 7, "exit": 75, "unix": 1792931132.6265388}
{"arm": "C", "attempt": 1, "segment": 8, "exit": 75, "unix": 1792931138.8207545}
{"arm": "C", "attempt": 1, "segment": 9, "exit": 75, "unix": 1792931142.010283}
{"arm": "C", "attempt": 1, "segment": 10, "exit": 75, "unix": 1792931147.275038}
{"arm": "C", "attempt": 1, "segment": 11, "exit": 75, "unix": 1792931341.9679897}
{"arm": "C", "attempt": 1, "segment": 12, "exit": 0, "unix": 1792932005.1259205}
```

## Disposition and not verified

The registered family and three-part case rules were applied verbatim. No retuning or timing-value rerun. No src, harness, corpus, bar, version, tag, merge or publish changes in the repository. The README known-exceptions line and CHANGELOG [Unreleased] performance entry were added as required. No fallback was run: the candidate passes. The r2 real-overdraw targets remain unmet; their measured 1.47–1.60× C++ exception is documented. Historical v1 NO RELEASE is retained. Owner review and publishing remain owner actions.

Not verified: performance outside this frozen corpus, other CPUs/platforms, individual-fix causal gains, independent full fuzz/WASM rerun. Owner review and publication remain pending.

Total lane wall time through evidence finalization: 97.24 minutes. Measurement session: 40.81 minutes including eleven train pauses. One measurement attempt; zero infrastructure reruns.
