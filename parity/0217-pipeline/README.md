# One-call pipeline qualification

The public API is documented in `docs/USING.md`. `tests/pipeline.rs` compares
public-API sequences, including static attributes, skin, repair, codecs, meshlets
and cluster outputs. It also checks whole-call budgets, atomic errors and empty
surfaces. Complete vertex records preserve attribute seams through weld/fetch.

Run `check.sh` under the required CPU admission runner, reservation 4, with a
fresh dedicated `CARGO_TARGET_DIR` and `MOSS_HEAVY_GPU=0`. `proof.sh` invokes the proof driver under the same
admission and requires `PIPELINE_INPUTS` to name the retained packed corpus.
It extracts the pinned v0.2.16 baseline read-only, runs the existing exact
regression and independent C++ F9 oracle in both configurations, then compares
the pipeline against the manual sequence and measures alternating invocations.
The sampled quality density is zero for corpus timings (triangle corners still
sampled); both arms use identical settings and fresh workspaces with unbounded logical
limits, inside the admission runner memory cap. Small fixtures separately test
the default and finite budgets. Large cases use three alternating pairs; other
cases use seven. Trial times
and medians are retained. Timings include output allocation, quality and drop.
No new C++ equivalent is claimed for the pipeline itself.

Gate exits, corpus input hashes and proof logs are retained in `receipts/`.
The final report records any failed or unproven criterion. Build artifacts are
removed after all admitted jobs exit.

The original clock returned zero for tiny single invocations. `supplement.sh`
batches fixtures below 100,000 indices and adds four alternating pairs for the
million-triangle case. Sub-200ms original small-fixture timings are retained but
excluded for clock resolution. All three original and four supplemental million-
triangle pairs are pooled; no unfavorable large-case samples are removed. The
original three-pair real-mesh timings remain in use. The final threshold is
checked exactly from integer elapsed counts and batch sizes, without rounding.

Final performance verdict uses the median of within-pair one-call/manual
ratios. Separate-arm medians are also retained for disclosure. Pooling raw arm
medians across campaigns can hide a regression when both arms' absolute times
change between campaigns; it is not used to clear the 1.02 threshold. The
threshold is unchanged. Shared-host timing scatter does not prove a cause.

A final controlled campaign (`quiet.sh`) chooses an allowed core by a fixed
one-second idle observation, considering its SMT siblings. Every arm must run
for at least five seconds and show a conservative CPU-occupancy lower bound of
99%, subtracting two scheduler ticks for accounting precision and allowing a
one-millisecond wall-clock quantum. Batch sizes are calibrated once per case
to target nine seconds; no speed-ratio observation controls sample selection. It stops on the
first rejected arm and never selects samples by their speed ratio. All eleven
cases otherwise use seven alternating pairs. Prior campaign failures remain
retained. The paired-median limit stays 1.02.

For a fresh validation without replaying superseded timing campaigns, run
`check.sh`, then `PIPELINE_PROOFS_ONLY=1 bash proof.sh`, `wasm.sh`, and `quiet.sh`
under the prescribed admission runner. Put the installed semver checker on PATH.
Supply the same retained inputs and dedicated target directory. `finalize.py`
records the verdict and deletes build scratch only after every job exits.
The retained historical campaign results remain part of this lane's verdict.

The wasm clock fallback follows the target's restricted std environment:
[official target documentation](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html).
