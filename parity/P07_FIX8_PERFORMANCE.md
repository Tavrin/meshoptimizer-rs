# P07 fix-eight — disk-blocked native candidates

**RFC closure is not established.** First counters/scout, source candidates and
static checks are complete. Candidate compilation, runtime parity, post-change
counter effects, Miri and the ONE final paired timing pass are pending because
`/mnt/linux-extra` is below the mandatory 25-GiB build floor. No candidate
elapsed-time sample has been taken. The final timing allowance is unused.

Source changes are separately committed:
- `5b9da67`: use the shared, compiler-vectorized canonical native Exp body,
  eliminating the tiny per-call size branch and the explicit large-size path;
- `513340c`: parameterize only canonical Oct's root operation, with existing
  safe SSE2 helpers supplying a correctly rounded hardware root in short x86
  tails; retain the pinned software root for scalar/reference and other targets.

These are unvalidated source candidates, not qualified performance fixes.
There is no new unsafe block; static audit passes the same 23 documented blocks.
Formatting and Python controller syntax pass. The new ordinary tail regression
and bounded Miri subset are authored but have not run.

## RFC residual disposition

A = allocating, C = caller-buffer. Every number below is the **prior fix-seven
record**, not a fresh fix-eight timing result. No prior pass qualifies the
changed source. The RFC limits remain geomean <=1.25 and maximum <=1.50.

| Platform / family / API | Prior geomean | Prior failed maximum 95% interval | Fix-eight disposition |
|---|---:|---|---|
| native / Exp / A | 0.996 | streaming32: 1.596–3.944 | Large maximum cause unidentified: abandon; S3-driven scalar candidate unvalidated |
| native / Oct / C | 0.882 | varied tiny4: 1.571–1.696 | Software root in short tail identified; candidate validation blocked |
| native / vertex / A | 1.124 | v1 streaming4: 1.538–2.847 | No isolated cause at first counters; abandon |
| native / view-filtered / A | 1.044 | repeated Oct streaming4: 1.568–1.746 | No isolated cause at first counters; abandon; changed filters still require native final pass |
| wasm / view-filtered / A | 1.281 | Multiple, up to 2.151 | No isolated cause at first counters; abandon, no retiming |
| wasm / view-filtered / C | 1.288 | Multiple, up to 1.762 | No isolated cause at first counters; abandon, no retiming |
| wasm / view-none / A | 1.325 | streaming12: 1.426–1.672 | No isolated cause at first counters; abandon, no retiming |
| native / S3 / A | — | tiny Exp12 SIMD/scalar: 1.1431–1.1751 | Fixed branch overhead identified; candidate validation blocked |

Other APIs/families retain only their prior evidence. Overall target is not met;
no native, WASM, release, integration, ARM or other-platform acceptance is implied.

## First cheap measurement and kill decisions

CPU26 `perf stat` instructions/cycles/branches/branch-misses user counters,
N/2N subtraction; matched successful output hashes. No elapsed times assessed.
At unchanged 3f58dda, tiny Exp12 has 2.7059 instructions/byte allocating versus
2.6814 scalar, and 2.0392 caller versus 2.0147 scalar: an extra size branch
before the exact same non-inlined arithmetic body. Large Exp32 allocating
cycles/byte are 0.2856 explicit versus 0.2405 canonical, but neither explains
the historical 3.944 upper maximum against C++. The scalar body is already
packed and unrolled in retained assembly. The candidate is motivated by the
identified tiny S3 overhead, without a claimed large-maximum cure.

Tiny Oct4 caller uses 12.0443 instructions/byte versus C++ 7.9413; 4.314 versus
2.7131 cycles/byte. Retained assembly contains the software-root tail. The
candidate replaces only that root, without padded recursive SIMD tails,
approximation or normalization-order changes. A final counter-effect assertion
requires an actual instruction reduction for both tiny Exp and Oct, plus
unchanged frozen output bytes; it remains unexecuted.

Native allocating vertex/view instructions match caller work (~6.1066 and
5.4043/byte), and fresh cycles do not reproduce a distinct allocating deficit
against C++. Both sides initialize output. No isolated cause: abandon rather
than changing parsing or allocation speculatively.

The first WASM analysis covers unfiltered12, repeated Oct8/Exp12 and varied
Oct4/Oct8/Quat8/Exp12, both APIs and C++ public paths. Separate V8 N/2N process
subtraction has tiering/GC contamination (some cycle differences are negative)
and reversed API instruction gaps. The common bounded decoder plus allocation
and copy seams do not isolate a specific cause for the remaining view failures.
All WASM residuals are abandoned under the first-analysis kill rule.

## Evidence and remaining work

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p07-fix8`.
First measurement and append-only milestones:
`/mnt/linux-extra/moss-scratch/p07-fix8/MILESTONES.md`; read-only map: `SCOUT.md`.

Retained round-seven native and Node control binaries independently match their
recorded hashes and every source hash at 3f58dda. Every frozen expected fixture,
malformed and benchmark archive is independently verified; this is verification
of retained expected bytes, not execution of the changed candidate. Native and
WASM first-analysis counters completed with exit zero. First native compile
attempt declined with disk-pause exit 75 before invoking Cargo; disk retries
have not met the floor.

Prepared controllers: profile-p07-fix8.py, measure-p07-fix8-leased.py and
verify-p07-fix8.py. Exact pending steps/limits are retained in
`remaining-work.json` in the artifact directory. After the disk gate opens:
compile candidate/scalar native drivers, assert post-change counter effects,
execute retained frozen statuses/bytes at every local ceiling and both APIs,
run safety/ordinary/Miri/MSRV checks, then use one native Exp/Oct/filtered-view
final epoch with round-seven controls in each pair. Preserve the inherited
4-GB heavy wrapper, 840-second timeout, 690-second pair checkpoints, 5–20
pair early stops and fresh 30-pair D146 stages only for borderline maxima.
No WASM rescue epoch; do not retime an unproven candidate. Record all failures.
Delete only `/mnt/linux-extra/moss-cargo-targets/codex-p07-fix8` at the final stop.

Terminal receipt: bounded 600-second disk wait exited **75** before Cargo.
Final free bytes: **16603815936**, below **26843545600** required.
The wait process has exited; the exact target was never created and is confirmed
absent. No queued or background candidate work remains. `terminal-receipt.json`
binds the logs; `remaining-work.json` lists every unrun gate. Fix-eight execution
is **incomplete and blocked**, with the final timing allowance still unused.
