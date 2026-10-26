# SPEC-027 lane A receipts

See REPORT.md and REGISTRATION.md in this directory. Raw evidence lives under
`<scratch>/meshopt-peer-a/`. The comparator was exported from
`bench/peer-screen`; that branch and other lane worktrees are untouched.

`results.json` retains all 300 measured pairs (600 samples), the warm-ups,
per-case intervals, A/A floor, family verdicts and executable/source identities.
`proof-summary.json` and `direct.json` summarize exactness; the full 717-case
protocol receipt and generated fuzz inputs are retained in scratch.
`evidence-sha256.json` binds retained raw artifacts, including rejected probes.

The Python/shell helpers record this lane's procedure. They use lane-local
absolute paths. Do not replay a build, proof or instruction collector into the
completed evidence directory. To reproduce independently, first export the
harness into a fresh scratch directory, update the path constants, use a fresh
lane target, and register a new experiment. `campaign.py` refuses any existing
wall.json. The acceptance audit deliberately exits 1 for the retained FAIL.

Sequence: setup.py, build.py B X M7, diagnose.py, source iteration using
instruction counts, final build.py O, prepare-proof.py, final-gates.sh,
direct.py, campaign.py (MOSS_HEAVY_MEASUREMENT=1), audit.py. Every heavy
command must run through <job-runner> 4 with MOSS_HEAVY_GPU=0. The original
bench/peer-screen branch must remain unchanged. Long fuzz/sweeps stay nightly.
