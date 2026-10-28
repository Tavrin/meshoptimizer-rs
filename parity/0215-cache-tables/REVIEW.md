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
