# 0.2.15 release receipts

Release preparation started from clean `release/0.2.15` at
`a8e87babdbfad13aaeb69b9b4a15da8db994028e`. Before documentation or version edits,
`requalify.py` ran the four C++ 1.3 differentials retained by the combined
qualification, plus the cache exact driver against v0.2.14. All returned zero:

| Comparison | All features | No default features | Differences |
|---|---:|---:|---:|
| Protocol (simplify family, overdraw, cache, fetch, quantization) | 1,246 | 1,246 | 0 |
| Pinned C++ overdraw | 178 | 178 | 0 |
| C++ cache/decode | 1,932 | 1,932 | 0 |
| Pinned C++ meshlets / cluster LOD | 1,280 | 1,280 | 0 |
| Cache/decode exact vs v0.2.14 | 992,130 | 992,130 | 0 |

The all-features dependency mode explicitly enables `std`, `simd`, `parallel`,
`clusterlod`, `experimental` and `parity-internals`. The no-default mode enables
no dependency features, except `clusterlod` for the meshlet/LOD driver's LOD API.
Only manifest selection, output paths and feature modes were adapted; inherited
case generation and comparisons were unchanged. `requalification.json` binds
source files, original drivers, derived driver/manifests, inputs and binaries.
The protocol and meshlet/LOD JSON receipts also bind their C++ executables.
The runtime library source remains byte-for-byte equal to the requalified source.

`checks.sh` runs final tests, explicit doctests and clippy with warnings denied
in both modes, fmt, unsafe/package boundary, public-text checks, full package
inventory, publication dry-run and whitespace checks. `check-exits.tsv` is
append-only, including any repeat checks. The inventory has 81 entries, includes
`docs/PERFORMANCE.md` and excludes development-only files. The dry-run verifies
the packaged crate and aborts before upload. Ignored corpus tests are not claimed
as rerun; the required differentials above use their full retained domains.

To reproduce from the source revision, set `RELEASE_SCRATCH` to a fresh retained
artifact directory, `RELEASE_INPUTS` to the eight fixtures bound by
`parity/0215-toward-cpp/inputs.json`, `MESHOPT_REFERENCE` to the pinned C++
checkout, and `CARGO_TARGET_DIR` to the dedicated release target. Run
`python3 parity/0215-qualify/receipts/release-0215/requalify.py` through CPU
admission before public edits, then run
`bash parity/0215-qualify/receipts/release-0215/checks.sh` through the same
admission path after release edits. Declare 4 GB, disable GPU admission, and
use `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`, `LC_ALL=C` and
`PYTHONDONTWRITEBYTECODE=1`. The first wrapper selected a 9 GB reservation/cap
from shared class history; the final-check wrapper selected 4 GB with a 6 GB cap.
No queued job was canceled. Keep raw logs, executables, fixtures and generated
manifests outside the package. Remove the dedicated target after all jobs finish.

Performance text uses only the combined qualification TABLE.md / REPORT.md and
the subsequent cache restoration qualification, rounded to three decimals in
the comparison tables. Historical lane measurements were not substituted.
No runtime API or algorithm changes, commit, push, tag, merge or publication
were made by this release preparation.
