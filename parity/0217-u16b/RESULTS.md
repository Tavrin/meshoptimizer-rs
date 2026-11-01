# Additive index-width API qualification

The 143 width-aware entry points use the `_generic` suffix (135 functions,
two aliases and six methods). Original names retain their u32 signatures and
forward to the same generic implementation. Width-aware input and result types
use a `Generic` suffix; original type names remain concrete u32 forms. The
original encoding descriptor enum retains its variant imports. Independent source and rustdoc
inventories both count 143 additions.

The 23 baseline test/example files, 25 inline test modules and 59 existing documentation code blocks
are unchanged from `b89cc4fe03efe346e4695594f1ca4e4688a8c8f1`.
The local `train/0.2.17` ref is absent; this is the pre-round-1 baseline
commit recorded in the retained round-1 receipt. All 66 baseline Rust source
files matched it before and after the baseline build.
All baseline tests, examples and doctests compile; tests and doctests pass.
The unchanged parity driver builds. The frozen u16 corpus passes 88 comparisons;
u16 overflow tests retain typed errors. See the compatibility, API and gate receipts.
Existing byte-layout decoders retain upstream conversion rules; typed index
outputs perform checked narrowing. The pre-fix failed gate receipts and sources
are retained in the private evidence directory, and the exit history is preserved.

All 136 cells reproduce baseline/candidate/C++ output exactly. Three interleaved
repetitions per Rust arm count resident-loop `instructions:u` with 100% coverage.
Every cell is within the unchanged 1% limit.

| Metric | Result |
| --- | --- |
| Cells within 1% | 136 / 136 |
| Range | -0.2342% to +0.0505% |
| Worst cell | tiny-smooth / meshlets-spatial |
| Worst instructions/call | 47014.90 → 47038.63 |

All final gate exit codes are 0. Detailed per-cell medians, samples,
compiled source/binary identities and final source hashes are retained alongside
this report. Round 1 evidence remains in the sibling directory.
