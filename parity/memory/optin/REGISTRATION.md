# SPEC-027 memory opt-in targeted proof

Baseline A: v0.2.6, peeled commit 33556c48efc13f3f36a9293aa5fbf451c0884fe3.
B: opt-in patch after merge 839c31c, bound by SOURCE.json.

Use existing spec027 timed-harness and corpus (214e/b6fd/a082); preserve the
six-call simplify workload. Same release profile: thin LTO, codegen-units=1,
no explicit target-cpu or alignment changes. CPU 8. CPU-only <job-runner>,
4 GB declaration, CARGO_TARGET_DIR=<target-dir>.

1. Default A/B one fixed chain per case, perf stat -e instructions:u, including
   the unchanged full driver. One six-simplify microbench per arm. Default B/A
   must be <=1.01 for every row. No wall campaign. If a row exceeds 1.01,
   perform the spec's one CPU-pinned ABBA wall check and rebuild both arms with
   -C llvm-args=-align-all-functions=6; retain all results and apply the v2
   LAYOUT rule without relaxing the instruction threshold.
2. Separate pure-capacity probe copies (same diag accessor) run the 214e step-1
   driver, default A and B. At every observe point compare the seven retained
   capacity totals, Usage bytes and counted work. No allocator override.
3. Exactly one uninstrumented B Lean 214e process under /usr/bin/time -v.
   Peak must be <=2.318 GiB. Compare complete stable result JSON, indices,
   float/error bits, legacy results and every SHA256-named output sidecar to A.
   Exclude only instrumented_processing_ns, as in the existing campaign.
4. Gates: all-feature tests, all-target/all-feature strict clippy, fmt and the
   focused both-mode family/quadric retention test. Preserve failed receipts.

All source snapshots, binaries, raw counters, full results and admission logs
stay in this new optin directory. Historical measurements are preserved.
Stop at about three hours; no a downstream engine changes, new timing campaign or other-family work.
