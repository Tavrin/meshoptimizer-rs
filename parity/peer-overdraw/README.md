# Job B evidence

registered protocol only: safe scalar overdraw changes, frozen synthetic and real corpus.
Read REGISTRATION.md for the one wall campaign and REPORT.md for its results.
`instructions.json` is diagnostic; it does not determine wall verdicts.

Full immutable inputs, harness export, source snapshots, drivers, raw perf files,
protocol responses, gate logs and wall pairs are retained under
`<scratch>/meshopt-peer-b/`. The setup/build/admission scripts
and original unmodified exported harness files are there too. No captured mesh
buffers are committed here. Only the designated Cargo target is removed.

Proof scripts:

- prepare-proof.py creates an external checker with separate candidate and
  v0.2.6 packages. Only the scratch baseline package name changes; its Rust
  sources remain byte-identical to the tag.
- exact.rs compares every output word, Result, reported work/bytes and partial
  caller buffer across API forms, packed/strided/little/big endian layouts,
  limits, frozen inputs, the existing overdraw seed generator and work prefixes.
- frozen-proof.py uses the exported comparator for full stride-64 real inputs,
  comparing output bytes with v0.2.6 and M7, and disclosing X differences.
- cpp-proof.py targets overdraw through the existing pinned C++ differential,
  fixtures and seeded generator; it enables the std dependency for this change.
- campaign.py uses the exported harness's resident driver protocol and analysis
  registered here. Exclusive file creation rejects rerunning a partial or
  completed campaign. Sampling requires all final gates and source hashes in
  the scratch wall.sh admission entry. Never overwrite that campaign.

All heavy commands use CPU-only <job-runner> and a declaration of at most 4 GB.
The initial counter launch/parse failures and canceled queued entry are retained
separately from the complete baseline and candidate instruction records.
