# Single-agent read-only review

Reviewed the complete source diff, work-accounting implementation, generated
comparison drivers, accepted option masks and instruction collector. No
subagents were used. This is a source review by the implementing agent, not
an independent reviewer or a cross-platform execution result.

- Work scan coverage is exactly the number of post-solve position reads, with
  one charge before each read. The existing scan implementation preserves
  consumed prefixes on callback failure and uses the checked path on budget
  exhaustion. No new helper or change outside the update API was introduced.
- Spatial ordered comparisons preserve the old operand order on ties, including
  signed zero. Products and their sum preserve operation order. The forward and
  reverse accumulators have disjoint state and no intervening error or charge.
  There is no removed Rust slice bound, finite check or numeric error path.
- Box layout and allocation types are unchanged; the fourth lane never
  contributes to an observable area. The transformations add no public API,
  unsafe block, intrinsic or target-specific dispatch.
- The exact drivers compare full caller buffers, result/error representations
  and both Usage fields. Update positions and attributes are compared by float
  bits. Every integer work limit from zero through unlimited observed usage
  plus one is included. Spatial uses an even wider sweep. Failed priming is
  recorded and must not be described as successful-warm coverage.
- The update sweep enumerates all 512 masks, compares acceptance on both
  revisions and skips unsupported masks in the no-default build (128 accepted).
  The spatial loop product is eight cases, four settings and four variants,
  or 128 cells per feature mode. Its inherited final print label says 140;
  that stale literal is not used to count observations or assert completion.
- Measurement uses median-of-three user instruction counts, validates counter
  running percentage, binds sources and binaries, preserves partial campaigns,
  and checks the regression ceiling before advancing to the next cell. It
  correctly stopped campaign 1. The later update revert follows that receipt;
  no subsequent optimization edit is permitted.

No observable counter-input was found. Performance remains determined solely
by the registered campaign. The isolated contract-cost report has no W/R/F
shares for these two target functions; no causal percentage is inferred from
other functions or from sampled profiles.

Final disposition: update was reverted on the campaign-1 regression, and
spatial was reverted on the campaign-2 NO-GAIN result. Final source/manifests
match v0.2.14 exactly; both replay patches and the retained bindings were checked.
