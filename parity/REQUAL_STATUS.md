# Final 0.1 integration requalification status

Updated 2026-10-04 23:12 CEST. The owner-approved D146 two-stage rule replaced
the clean full-profile repeats. This file preserves the earlier 23:30 estimate:
before the rule change, repeats were projected to finish around 01:00-02:15;
the first full matrices actually finished by 23:02.

The rebased source passed fmt, warning-free clippy, all required root tests,
the wasm32 build and executed identity, JavaScript sanity, and exact run/sweep
parity for 0.1 (including 0.1.x), 0.2, 0.3 and 0.4 with zero mismatches.
Artifact verification for 0.2, 0.3 and 0.4 passed. Both 0.4 benchmark profiles
passed every family bar; all 19 current-source fuzz smokes ran at least 300
seconds and the 0.4 report verified their artifacts.

Stage 1 retained the complete 912-case 0.1.x and 204-case retained 0.1
matrices under both Moss and default profiles. Every family geometric mean and
heap bar passed. Seven profile-specific case maxima exceeded 1.5. D146 was
recorded before stage 2, which ran 30 new alternating pairs per flagged case
on pinned CPU 21 of physical core 20-21. The table gives the two-sided 95%
Student-t confidence interval for the mean log Rust/C++ ratio, exponentiated.
PASS requires the upper bound at or below 1.5; FAIL requires the lower bound
above 1.5; INCONCLUSIVE counts as failure.

| Profile and case | Stage 1 | Stage-2 95% ratio interval | Verdict |
|---|---:|---:|---|
| Moss retained 0.1 overdraw/million/disconnected/allocating | 1.522 | 1.475-1.582 | INCONCLUSIVE, residual |
| Default 0.1.x filter_index_buffer/tiny/seam-heavy/mode-2 | 1.509 | 0.963-0.995 | PASS |
| Default 0.1.x simplify_sloppy/million/disconnected/mode-1 | 1.507 | 1.444-1.509 | INCONCLUSIVE, residual |
| Default 0.1.x simplify_sloppy/million/disconnected/mode-2 | 1.508 | 1.386-1.516 | INCONCLUSIVE, residual |
| Default 0.1.x vertex_fetch/million/sparse/mode-1 | 1.543 | 1.542-2.088 | FAIL, residual |
| Moss 0.1.x generate_vertex_remap_custom/million/smooth/mode-1 | 1.561 | 1.019-1.142 | PASS |
| Moss 0.1.x vertex_fetch/million/sparse/mode-1 | 1.815 | 1.546-2.073 | FAIL, residual |

The complete per-pair samples, input/output bytes and hashes, source and
executable identities, core selection and load observations are in
`results/stage2-0.1.json` and its external artifact. The earlier core-28
diagnostics were excluded from stage 2. Five cases remain documented
residuals; no stage-1 maximum, family mean or heap value was rewritten.
The older four-CPU-hour 0.1 release fuzz record is historical for this
combined source, so full release acceptance remains separate.
The final `parity/report.sh --verify-artifacts` exited 0 on 2026-10-04;
`results/requalification-0.1.json` records the verified stage-2 verdicts
and the five residuals. Earlier unavailable historical archives are identified
as historical evidence and were not used for this qualification.
