# LOD quality qualification

Design, proof boundaries and final verdict: [PROOFS.md](PROOFS.md).
Instruction medians and wall disclosure: [TABLE.md](TABLE.md).
Machine-readable final status and all step exits: `receipts/final.json` and
`receipts/exits.tsv`. Initial lint failures and the superseded performance
campaign remain retained.

Reproduce using the CPU admission runner required by the lane, reservation 4,
`MOSS_HEAVY_GPU=0`, `LC_ALL=C`, and the lane's dedicated `CARGO_TARGET_DIR`.
Supply `LOD_INPUTS` (the SHA-256-bound peer inputs) and `LOD_PERF` (the required
perf executable). Run the commands below under admission, sequentially:

1. `bash parity/0217-lodq/check.sh`
2. `python3 parity/0217-lodq/proof.py`
3. `bash parity/0217-lodq/perf.sh`
4. `bash parity/0217-lodq/final-check.sh`
5. `bash parity/0217-lodq/post-check.sh`

After all jobs have exited, `python3 parity/0217-lodq/finalize.py` verifies the
receipts and final-source bindings, renders the table, replaces machine paths
in logs with symbolic roots (retaining before/after SHA-256), runs the public
text check, and removes the dedicated target directory. It does not modify Git
state. Archive extraction uses the pinned tag read-only; generated trees,
binaries and input references remain outside the source tree until cleanup.
No fuzz campaign is run; `fuzz/` contains the requested libFuzzer target source.
