# optimize_overdraw instruction gate (registered before measurement)

Arms: base (v0.2.14), candidate (this branch), M7 (C++ 1.3 via `meshopt`
0.7.0), optimesh 1.1.0. Workloads: the frozen five-case corpus and the three
real meshes in the peers format, the peers consumer and allocation policy,
threshold 1.05, median of three repetitions, `instructions:u` only (perf
6.17), CPU-only admission with 4 GB declared.

Verdict for optimize_overdraw, applied verbatim:

- SHIPS if the three-real-mesh geometric mean of candidate/optimesh is
  <= 0.99 and of candidate/base is <= 0.97;
- IMPROVES if only candidate/base <= 0.97;
- NO-GAIN otherwise.

Any case with candidate/base > 1.01 means that change is fixed or reverted.
No code edits after a campaign's results except reverts; at most two
campaigns. Output bytes must equal base in every repetition.

`build.py` builds the four arms from the scratch `base` and `candidate`
trees, `prepare_driver.py` removes wall timing from the consumer, the qualify
lane's `prepare_inputs.py` reproduces the peers inputs (hash-checked against
`parity/peers/inputs.json`), `measure.py` appends one record per finished
sample and skips finished samples on restart, and `results.py` applies the
verdict.
