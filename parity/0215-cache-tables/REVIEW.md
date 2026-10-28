# Read-only adversarial review

Reviewed the complete E2 source/test diff against base
`dbb75e0` (v0.2.14). This is a same-author review, not an independent review;
the lane prohibits subagents.

- Invalid construction: both lengths must be exact, and every element must
  be finite before copying. Private fixed arrays and immutable getters preserve
  that invariant after construction. No allocation or unbounded validation.
- Bounds: only cache indices 0 through 16 and live indices 0 through 8 can
  reach scoring. Tables do not change the cache, adjacency or work-bound sizes.
- Defaults: preset implementations retain the original constant arrays,
  clamping, addition order and const selection. No dynamic dispatch. Kernel
  mutation order, comparison/tie order, work charges, reservation and refund
  logic are unchanged. FIFO is untouched.
- Negative, zero and extreme finite scores: no positivity assumption is used
  for memory safety. No positive next candidate falls back to the existing
  emitted-triangle cursor. Large finite values may overflow score arithmetic;
  rustdoc explicitly records this limitation. No quality guarantee is made.
- Errors and limits: both new wrappers duplicate the standard wrapper order,
  including topology validation, output byte charging, destination checks,
  scratch reservation and usage finalization. `_into` retains partial-prefix
  failure semantics and untouched tails. Table ownership uses inline storage
  outside workspace accounting, like other caller-owned settings.
- Proof scope: default identity is compared to the existing API, not a newly
  executed C++ oracle. The retained corpus is explicitly requested by an
  ignored test rather than silently omitted when unavailable. Tight-budget
  tests compare Results, full destination buffers and Usage, including reuse.

Verdict: no blocking source finding. Independent review, GPU tuning, performance
qualification, cross-target testing and a fresh C++ differential remain outside
this lane's executed proof.

## Second-agent check

A separate agent re-read the commit against the specification and reran the
driver. Dispositions:

- Spec items (API, `_into`, validation, default identity over the corpus,
  doctest, guide recipe, changelog and README lines, three test classes): all
  present. Corpus files were re-hashed against `corpus.sha256` before the run.
- Selection semantics were undocumented (accepted, fixed in docs): the kernel
  only picks a triangle by score when its sum is positive, as upstream does, so
  non-positive tables fall back to input order. Rustdoc and the guide now say
  so, and the overflow note states its consequence (ordering degrades,
  triangles are never dropped or duplicated).
- Magnitude bound on finite scores (refuted as a blocker): rounding drift in
  the incremental triangle sums has no cheap provable bound, and overflow only
  yields infinite or NaN sums. Those change which candidate wins or route
  through the existing input-order fallback; emission still tracks every
  triangle exactly once. Finite-only validation matches the spec.

The rerun includes the documentation fix: all 13 driver checks exit 0
(`proof-exits.tsv`), the corpus test passes all eight meshes in both feature
modes, and `source.sha256` is refreshed for the edited files.
