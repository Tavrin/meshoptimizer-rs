# Simplify third pass

Set `MESHOPT_SCRATCH` to a new private scratch directory,
`MESHOPT_ATTRIBUTION` to the retained release attribution inputs,
`MESHOPT_PERF` to perf 6.17, `MESHOPT_REFERENCE` to the pinned C++ checkout,
`MESHOPT_M7_SOURCE` to the pinned scalar M7 source, and `CARGO_TARGET_DIR`
to the dedicated disposable target. Supply a shared admission adapter exposing
`PARITY_RUNNER_ACTIVE`, `PARITY_RUNNER_GPU=0` and measurement status.
All builds and measurements must use that adapter with a 4 GB declaration.

1. `prepare.py` freezes release sources, inputs, proof consumer and harnesses.
2. Admit `build.py base M7`, then `attribute.py` with `MESHOPT_PROFILE_LABEL=attribution-valid` and
   `MESHOPT_PROFILE_PERIOD=1000000`; inspect annotated instructions
   and write ATTRIBUTION.md before changing library code.
3. After implementation, admit `gates.sh`, then `build.py candidate`.
4. Admit `measure.py`, then run `results.py`. Finished samples are append-only
   and skipped on restart. Incomplete artifacts move to `interrupted/`.
5. Audit source, binaries, raw samples and all proof results; write REPORT.md,
   run the public-text check, and remove only the dedicated target directory.

The registration fixes the gate and campaign cap. Preserve rejected campaigns;
no implementation edits after a campaign result except reversions.

The admitted `bash parity/0213-simplify3/qualify.sh` sequence formats the source
before binding it, runs the complete proof, builds the candidate, then runs the
campaign, audit and report without another admission. Attribution has already
completed at that point; its completion receipt binds the release source and
ATTRIBUTION.md before implementation.

The retained `attribution/` directory contains the rejected throttled initial
sampling run. `attribution-valid/` contains the replacement profiles, sampling
quality checks, inline stacks and function estimates used by ATTRIBUTION.md.
