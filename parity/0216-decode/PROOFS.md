# Vertex decoder proofs and qualification

Local qualification completed. Native arm64 execution belongs to the
coordinator's CI phase; local cross-compilation cannot clear that gate.

## Structural identity

The NEON full decoder retains the canonical scalar parser's header/version
checks, trailer layout, block sizes, controls, channel order, predictor update,
exact final-consumption check and destination block/finish calls. Resource
preflight, error precedence, allocation and `Usage` accounting remain outside
the SIMD boundary and are unchanged. Stack scratch is initialized and bounded.

The byte parser maps selectors to the same 0/1/2/4/8-bit groups as scalar.
Every ordinary group first obtains a checked 24-byte array. The four-group
shortcut first obtains 96 bytes: each group consumes at most 24, so all four
canonical lookaheads succeed. Zero shortcuts apply only to zero-width groups;
literal shortcuts only to 8-bit groups. These shortcuts change no consumed
byte count and only write plane scratch. Later plane writes may overlap dead
padding exactly as in the scalar parser; all live plane bytes remain intact.

NEON zip operations interleave byte planes A/B and C/D, then interleave those
halfwords to form four vectors of four four-byte records. For channel 0,
unsigned byte shift, negate and XOR implement each byte's zigzag inverse;
channel 1 does the same independently for each little-endian halfword.
For channel 2, two variable word shifts and OR implement rotate-left, including
rotation zero (the right shift by 32 produces zero). The one-record and
two-record zero-filled shifts compute inclusive prefixes over four records.
Wrapping byte/halfword addition is associative modulo 2^8/2^16; XOR is
associative. Combining the repeated previous word therefore gives exactly the
scalar recurrence, without cross-component carries or float arithmetic.
The last word seeds the next vector. Only complete sets of 16 records use
vector reconstruction; the unchanged scalar tail starts at that exact record
and receives the same predictor. Scatter stores touch exactly each channel's
four live bytes. No input overread or destination overstore is used.

All four planes of a channel are validated before that channel writes output.
Channels commit in canonical order; malformed later channels therefore retain
the same complete destination and partial writes as scalar. Owned decoding
uses the same initialized block destination and drops output on error. Tests
compare the entire caller buffer even when decoding fails.

The SSSE3 prototype adds const stride-12 monomorphs to the existing checked
kernel. No parser arithmetic, byte format, resource charge or output order
changes. Outlining variants were diagnostic experiments; their measurements
are retained separately. Performance admission uses the unchanged limits in
`REGISTRATION.md`.

## Unsafe boundary

The NEON entry adds one token-authorized target-feature call. All memory
accesses reuse the audited array-backed load/store helpers. Only the private
dispatcher issues the existing AArch64 capability token. Inventory rises from
23 to 24 unsafe blocks; the single module allowance remains. The independent
published-package checker now compares every packaged unsafe block against
the safety inventory, in addition to enforcing the single allowance.

## Measurement controls

The first three prototype campaigns built the Rust arms at different source
paths. They showed vertex savings but the same index regression above 1.01.
Those receipts remain rejected. A canonical-path campaign adds a no-x86-change
control and builds all Rust arms sequentially at the same absolute path and
package identity. This tests build-path effects without changing the guard.
The first same-path experiment was also rejected: copy2 preserved source
mtimes, so Cargo reused the base executable for the control and tuning arms.
Its identical binary hashes and freshness logs are retained with an INVALID
marker. The corrected campaign touches every source/manifest before each
build, requires a recorded library recompilation for every Rust arm and
requires distinct control/tuning binary hashes. All inputs, binaries, source
hashes and individual samples are retained.

The initial wasm timing adapter enabled diagnostic dispatch ceilings. Its
results are retained as diagnostic measurements. The default-build table uses
a separate adapter depending on the actual default features, with no diagnostic
feature, built against frozen v0.2.15. Both artifacts execute in Node, and output
identity is checked against the pinned upstream shipped SIMD wasm decoder.
A fresh local C++ wasm compilation is unavailable; wall-time ratios are
bridge-inclusive disclosures and are not instruction-performance claims.

## External execution boundary

Both `ubuntu-24.04-arm` and `aarch64-apple-darwin` CI jobs explicitly execute
SIMD tests with forced scalar/NEON ceilings, repeat with no-std SIMD, and run
native C++ scalar/default differentials. Their receipts are uploaded separately.
No local ARM runner or qemu is available. The coordinator must push and read
those jobs before treating native NEON identity as qualified.

## Final verdict and receipts

SSSE3 stride-12 tuning is **REJECTED and reverted**. The correctly rebuilt
candidate reduced real-mesh vertex instructions to 0.910984x base, but its
worst index-decode case reached 1.014631x, above 1.01. Outlining did not remove
the regression. The final source is the measured unchanged-x86 control;
its executable is byte-identical to the v0.2.15 executable at the common
build path. No x86 or wasm tuning is retained or claimed. The wasm baseline
shows a bridge-inclusive loss, without sufficient attribution to justify a
kernel change. The final runtime change is the full NEON path only.

Default-build ratios are in BASELINE.md. TABLE.md describes the final
unchanged-x86 control; TUNING.md and the fresh campaign's tuning-results.json
retain the rejected candidate. Earlier path experiments and the explicitly
INVALID stale-binary experiment remain separate from qualified results.

The exact driver passed **3,505,440 observations per feature mode**, comparing
typed errors, owned results, whole caller buffers and Usage on cold, warm and
after-failure workspaces. Every small-fixture work limit and output byte-limit
boundaries were tested; 2,000 larger seeded/mutated cases supplement them.
Only the proof baseline package name was changed to avoid Cargo's same-name,
same-version path-package lockfile collision; its Rust sources were unchanged.
The initial lockfile failures are retained; final exact runs both exit zero.

Each native differential mode passed **318,608 cases** at scalar, SSE2, SSSE3
and SSE4.1 ceilings. Both std and no-std SIMD were executed; no-std's ISA
capabilities were compile-time enabled on this supported host. Rust results,
Usage in integration tests, and all destination bytes (including failure
prefixes and tails) match scalar. C++ scalar/default successful bytes and
normalized success/failure statuses match. **74,863 C++ failure-prefix
comparisons differ in each mode**: upstream stages failed blocks differently
from the unchanged Rust scalar decoder. These are observed inherited
boundaries, not C++ malformed-buffer byte-identity claims. The exact driver
and unchanged scalar source preserve the Rust contract.

Executed wasm passed **316,608** scalar/simd128 observations, including errors
and complete destinations. ASan completed a fixed **20,000** seeded runs;
Miri passed the supported integer-kernel subset. The final suite also passes
all-feature/no-default tests, no-std SIMD tests, Clippy, actual AArch64 library
builds with/without std, cross-target test compilation, unsafe-free build,
package verification, unsafe inventory, formatting, and public text checks.
Focused Rust 1.88 SIMD and no-default checks also pass. The owned decoder's
1 MiB staging threshold and staged destination tails have focused coverage.

Native NEON execution is **pending coordinator CI**, on both Linux arm64
and Apple arm64, for std/no-std forced-level and C++ differential matrices.
No ARM performance claim is made. Cross-compilation does not clear identity.

Four initial installed-supervisor attempts received SIGTERM immediately after
admission; they were not canceled by this lane. Subsequent jobs retained the
required CPU admission and GPU=0, using an invocation-local fixed 4 GiB
systemd cap instead of the failing optional elastic scope supervisor. No
installed runner configuration or queue steering was changed.

Final proof steps: **26**, all exit **0**; MSRV steps: **2**, both exit **0**. Final proof CPU: **377.09 s**; including the retained initial proof attempt: **972.23 s**, below 7,200 s. `receipts/proof-final/` retains individual exits and logs.
