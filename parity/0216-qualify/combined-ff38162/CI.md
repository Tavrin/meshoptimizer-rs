# Source-identical arm64 CI receipts

PR #80 head `896d5922f9c08eba7ef718a8775235b4ac4a2e98` was merged as
`ff38162d6de2c6c543a45536733442939100801b`. A live check of the PR and job
metadata confirms both native arm64 jobs succeeded, including the step
"Assert Rust NEON and C++ scalar parity; disclose C++ NEON divergences".

`git diff --exit-code 896d592 ff38162 -- src Cargo.toml Cargo.lock` returned
0; its empty output is retained in `ci-source-diff.patch`. Both `src` trees
are `340e0a96d37848e659485cc6919d0cc0ae595e2b` (`ci-source-trees.txt`).
These are retained CI results for the source-identical PR head, not a new
local arm64 run. This binding must be revisited if qualification reverts source.

- [Linux arm64 job](https://github.com/Tavrin/meshoptimizer-rs/actions/runs/40999840674/job/123883226250)
- [Apple arm64 job](https://github.com/Tavrin/meshoptimizer-rs/actions/runs/40999840674/job/123883226216)

The downloaded `neon-decode-aarch64-unknown-linux-gnu` and
`neon-decode-aarch64-apple-darwin` artifacts each contain default and no-std
differential receipts. Every receipt covers **318,608 cases**, forces scalar
and NEON, and reports **0 Rust forced-level differences** and **0 successful
byte differences against C++ scalar**. Four receipts total **1,274,432 cases**.

Per receipt, **37,431 C++ scalar failure-write differences** are disclosed;
failed destination prefixes are not a C++ byte-identity claim. The upstream
C++ NEON/default decoder has **30,500 ISA status/success-output divergences**
from C++ scalar per receipt. Under the merged workflow's documented rule,
C++ NEON is disclosure only; Rust must match Rust scalar and C++ scalar.
