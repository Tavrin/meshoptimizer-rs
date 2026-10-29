# 0.2.15 release fuzz targets

Coverage-guided libFuzzer targets for the 0.2.15 train (cargo-fuzz 0.13.2,
nightly, `-O -a`: release optimisation with debug assertions and overflow
checks, coverage and trace-compares, no sanitizer).

Changed families are differential: each input runs on the train and on v0.2.14
(`baseline`), cold then warm on one workspace, and the Debug form of every
Result, every caller buffer and the workspace Usage must match. New APIs check
their invariants, cold/warm determinism, and that a run under a work limit
either fails or equals the unlimited run.

| Target | Functions | Kind |
|---|---|---|
| `simplify` | simplify, simplify_with_attributes (+ `_into`), simplify_with_update; option masks, widths 0-9, weights, flags, packed / strided / byte positions | differential |
| `simplify_extra` | simplify_sloppy, simplify_prune, simplify_points (+ `_into`) | differential |
| `overdraw` | optimize_overdraw, `_into`, `_in_place`, too-small buffer | differential |
| `meshlets` | build_meshlets, build_meshlets_flex, build_meshlets_spatial | differential |
| `clod` | clusterlod::build_with_output (raster and RT configs, attributes, locks) | differential |
| `cache` | optimize_vertex_cache (+ `_into`, `_in_place`, strip, fifo); cache score tables | differential + invariants |
| `remap` | remap_vertex_buffer, remap_index_buffer (+ `_into`), generate_vertex_remap | differential |
| `sanitize` | sanitize, sanitize_into, every option incl. WELD_POSITIONS | invariants |
| `analyze` | analyze_mesh, weld mode and list limits; agreement with sanitize's invalid-triangle counts | invariants |
| `vertex_layout` | VertexFieldSnapshot / VertexLayout, snapshot positions in a real call | invariants |
| `lod_chain` | build_lod_chain: counts / ratios, uniform / per-level errors, options, attributes, flags | invariants |

Positions come in four modes (parameter byte 0): raw float bits, a small grid,
the small grid scaled by 2^k for k up to 127 (finite coordinates up to near
`f32::MAX`, and overflow to infinity at the top), and raw bits with non-finite
values replaced by `+-f32::MAX`. Every target, including overdraw, sloppy and
points, decodes positions this way.

## Rule (owner, 0.2.15)

Start from the seed corpus. Run each target in its own single process until
its edge coverage (libFuzzer `cov:`) has not grown for 600 CPU seconds, with at
least 600 and at most 7200 CPU seconds per target. Zero crashes.
`fuzz.py run` applies exactly this rule and kills nothing else.

## Commands

Inside the CPU-only admission runner (`PARITY_RUNNER_ACTIVE`,
`PARITY_RUNNER_GPU=0`), with `QUALIFY_SCRATCH` (holding `base/`: `git archive
v0.2.14` with the package renamed `meshoptimizer-baseline`), `CARGO_TARGET_DIR`
and `QUALIFY_CORPUS` (the frozen peer-screen source corpus: tiny-smooth,
medium-*, million-smooth and real-*-simplify `.bin`, sha256 in
`parity/0215-cache-tables/corpus.sha256`):

```sh
python3 parity/0215-qualify/fuzz/fuzz.py build    # copies this dir, builds all 11
python3 parity/0215-qualify/fuzz/fuzz.py seeds    # 174 seeds from the corpus
python3 parity/0215-qualify/fuzz/fuzz.py run TARGET...   # concurrently, one process each
```

`lod_chain:bounded` runs `lod_chain` with ratios kept in [0, 2]
(`QUALIFY_LOD_BOUNDED=1`) in its own directory.

Receipts: one JSON line per target appended to
`$QUALIFY_SCRATCH/fuzz/receipts.jsonl` (target, binary sha256, seed, exit,
stop reason `coverage-flat` or `max-cpu`, CPU seconds, initial/final `cov`,
CPU second of the last coverage growth, executions, corpus size, crash files).
Copy them to `parity/0215-qualify/fuzz-receipts.jsonl`. libFuzzer exit 72 is
the rule's own SIGINT stop; any other exit or a non-empty crash list fails.
Crash inputs stay in `$QUALIFY_SCRATCH/fuzz/runs/<target>/crashes/`.
