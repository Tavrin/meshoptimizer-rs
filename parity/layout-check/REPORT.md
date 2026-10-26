# Bounds layout check

RELEASABLE in next 0.2.x together with the retained new-API result.

| Family | Instructions | Default wall | Pinned wall | Verdict |
|---|---:|---:|---:|---|
| meshlet-bounds | 0.999940 | 0.913579 | 0.990512 | PASS |
| cluster-bounds | 1.001166 | 0.913268 | 1.017870 | PASS |

F = 0.083839596; T = 1.104 (ceil to 0.001 of 1 + F + 0.02). A/A family GMs: {"meshlet-bounds": 1.0838395961586433, "cluster-bounds": 1.0646445398503799}.

Candidate a97301e; baseline 775ff8c. Registration and DECISIONS disclosure committed as 42f7374 before samples. Post-fix1 rule revision disclosed; fix1 FAIL remains historical. New API meshlet 1.243711 and cluster 0.993831 against M7 retained from fix1, not remeasured.

CPU 11, five frozen cases, unchanged thin-LTO moss consumer, alternating five pairs extended to twenty only when five-pair CI contains equality; 25 ms calibration. Case paired-ratio GMs feed equal-weight family GMs. A/A uses baseline archives Bprime/B in different paths in this session. Default flags empty; pinned flag `-C llvm-args=-align-all-functions=6`. Both pinned kernels in both arms are mod64=0; see ALIGNMENT.json and alignment-symbols.txt. Fixed-call instructions:u counters use fix1 controlled regions (10,000 calls, million-smooth 100). Byte equality checked before calibration in all 30 comparisons.

Preflight encoded-flags override was caught before samples; preflight artifacts retained and pinned arms rebuilt with the same registered flag. See BUILD-PREFLIGHT.json. The first generic-Python admission was cancelled while queued, before payload/samples; it was resubmitted through a dedicated shell entry point without an admission override or declaration change (ADMISSION-PREFLIGHT.json). No time-value reruns. Infrastructure retry used: False. Train-first checks and any PAUSE are in CAMPAIGN.jsonl; no waits on other meshopt lanes.

Registered decision applied verbatim: REGRESSION if instruction GM >1.01 or both wall GMs >T; LAYOUT if instructions <=1.01, default >T, pinned <=T; PASS otherwise.

Receipts: /mnt/linux-extra/moss-scratch/meshopt-validate-once/layout-check; raw timing/calibration/load and instruction stat files retained here and under parity/layout-check. Source/binary/input epoch hashes in MEASUREMENT-IDENTITY.json. Full linker maps and executables retained in scratch. No library source changes, new API/fetch-remap remeasurement, release/tag/version action, GPU or production acceptance.

Cleanup: designated Cargo target deleted after executable retention; see CLEANUP.json and RETAINED-EXECUTABLES.json.

Audit PASS: 30 wall comparisons, 10 instruction cases, 234 frozen hashes unchanged, 19 train-first PAUSE records; no infrastructure retry. Load1 range 7.80–12.23; elapsed 24.21 minutes from registration including admission waits. Identical-source A/A variation is substantial; these walls do not establish an optimization speedup or quiet-host performance.
