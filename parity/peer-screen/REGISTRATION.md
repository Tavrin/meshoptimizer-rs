# Bet 0 registration

Recorded 2026-10-25T15:02:51.863728+00:00; no builds, correctness runs or samples have occurred.
Baseline: 775ff8cab4fb2ba01966acd1b277aa7019eb8937 (v0.2.4).

## Registered protocol (write REGISTRATION.md before any sample; apply verbatim)
- **Arms:**
  - O, ours at 0.2.4;
  - M7, meshopt 0.7.0 through `meshopt::ffi`, as the existing adapters do;
  - X, optimesh 1.1.0 through its public API.
  Use the same frozen inputs, Moss's thin-LTO consumer profile and the Cargo-default profile, scalar
  and SIMD builds as the harness already defines, and the same CPU pinning.
- **Families:** every family the harness covers today, plus:
  - meshlet building and cluster LOD where X offers them (M7 has no cluster LOD: mark it
    unavailable);
  - the three real meshes 214e, b6fd and a082, for simplify and overdraw, through the a082-diag
    replay inputs at /mnt/linux-extra/moss-scratch/meshopt-a082-diag/, if X and M7 accept them;
  - browser decode on the 42 frozen streams: O simd128, X wasm SIMD, and upstream JS SIMD as the
    reference.
- **Correctness before timing:**
  - Each arm's output for each family is checked against scalar C++ 1.3. Record one of: identical;
    different but valid (the harness's independent validity checks pass, with triangle count and
    reported error recorded); or invalid.
  - M7 is expected to be identical; X ports 1.1 and may differ.
  - A family whose outputs are not identical is still timed, but labelled "not output-equivalent".
    Its timing table rows carry the output metrics next to the ratio.
  - An invalid output is not timed.
- **Timing:** use the harness's existing paired protocol, unchanged: five alternating pairs, extended
  once to twenty when the paired interval straddles equality. Report each family as the geometric
  mean of case ratios (peer time / ours) with its 95% interval, plus the max case.
- **Noise floor:** one same-session A/A family set, O against a second O build from identical source
  in a different path, on the three densest families. F is the largest |family GM − 1|.
- **Verdict per (peer, family):**
  - PEER FASTER if the upper bound of peer/ours is < 1 − F − 0.02;
  - OURS FASTER if the lower bound is > 1 + F + 0.02;
  - otherwise EQUAL WITHIN NOISE;
  - or UNAVAILABLE.
  A verdict applies only to output-equivalent families; others get "NOT EQUIVALENT" plus the
  numbers.
- **"Beats ours anywhere":**
  - Yes, if any family with an X cluster-LOD or wasm decode arm is PEER FASTER.
  - Also yes, if any non-equivalent family shows X faster beyond the same bound with no worse
    reported error at the same triangle count. That case is flagged for owner review, not counted
    as a quality win.
- **Infrastructure:** a harness timeout or crash allows one rerun of that family, which is recorded.
  No retuning, and no reruns to change a verdict.


Safety admission is a prerequisite. A safety stop produces no timing verdict and no invented F.
