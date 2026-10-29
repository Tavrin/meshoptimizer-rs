# Read-only adversarial source pass

Scope: the cache score tally, greedy/flex root dispatch and exact selection,
and private cluster-LOD simplifier phase accounting, compared with v0.2.14.
Performed by the completing agent; no independent reviewer or subagent is
claimed. Code stayed fixed during this pass.

- **Exhaustion and precedence:** each covered dispatch proves the entire
  phase fits before replacing checked charges. A failed bound calculation
  chooses the original path. On a finite/quadric error, the visited tally is
  applied before the same error propagates; no public callback runs inside
  the phase. The cache loop has no fallible callback on its covered path.
- **Partial writes:** limited cache visits and LOD filtering retain their
  original charge-before-mutation order. Picking restores its partially
  populated vector on every return; capacity and stored record widths stay
  identical. Deferred accounting changes no write on a covered path.
- **Roots and exceptional inputs:** the unsigned bit guard excludes negative
  zero, negative inputs, infinities and NaNs. The eight-lane guard falls back
  for the whole chunk if any operand is outside the proven domain. Checked
  arithmetic and finite validation are retained. no_std uses the old root
  backend and pruning throughout.
- **Selection and ties:** exact bounded scores use the same expression and
  accumulated normal as `resolve`. Priority is filtered first. A lower
  priority number selects before any tie lookup, so the initial sentinel
  triangle is never dereferenced. Listed ties use the existing upstream
  visit key; ordinary adjacency and KD searches retain strict-less order.
- **Reuse and storage:** phase tallies are call-local. The collapse vector
  is moved, not reallocated, and restored before every result. No retained
  workspace size formula, public type or validation gate changes.
- **Scope:** the unqualified SIMD specialization inherited from the partial
  tree was removed. Main simplify, spatial meshlets, shared main math and
  workspace helpers are unchanged; only the private LOD snapshot helpers
  listed in `PROOFS.md` change.

No counter-input was found in this source pass. It supplements the exact,
C++ and fuzz receipts; it does not assert exhaustive testing of all layouts,
floating-point values, option products, targets or allocation failures.
