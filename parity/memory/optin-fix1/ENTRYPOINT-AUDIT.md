# registered protocol memory opt-in fix1 entry-point audit

- simplify and simplify_with_attributes: allocating validates then acquires Simplify before reservations, then run -> run_state -> State::retain.
- simplify_into and simplify_with_attributes_into: into validates then acquires Simplify before run -> run_state -> State::retain.
- simplify_with_update: fixed to acquire Simplify immediately after validate succeeds, before taking original or creating Budget; calls run_state then State::retain.
- SPARSE dispatch: run_state -> sparse_run -> run_state (SPARSE cleared); inherits acquisition from the above public entry points, no independent caller.
- simplify_scale, simplify_prune, simplify_prune_into, simplify_sloppy, simplify_sloppy_into, simplify_points and simplify_points_into do not call run_state or State::retain. No additional omission found in scope.

Focused four-vertex cache/update/cache test runs Lean then Reuse; checks zero cache capacities after Lean update and absent simplify state after cache, with equal cache outputs, update index count/error bits/mutated indices and position bits, and each call's counted work. Target count equals input length; no attributes.

Broader variant-test mode parameterization remains the recorded optional coverage suggestion, outside this fix. No timing, other-family changes, merge, tag, or publication.

Instruction proof reuses optin/measure.py with only the registered case loop restricted to micro and 214e and the capacity/RSS runs removed. Retained baseline A executable matches optin/BINARIES.sha256; candidate B rebuilt using same retained harness, same release settings, default Reuse. Both arms rerun once, CPU 8. No previous receipts overwritten.
