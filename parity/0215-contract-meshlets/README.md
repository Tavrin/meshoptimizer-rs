# Contract meshlet qualification

The scripts use a dedicated scratch directory, one target directory, and a
CPU-only admission runner. Set `CARGO_INCREMENTAL=0`. The builder checks for
40 GiB free before each arm, records its executable hash, and cleans the
package before continuing. An interrupted build can resume completed arms.
Set `CARGO_TARGET_DIR`, `PEERS_ART`, `PEERS_PERF`, `PARITY_RUNNER_ACTIVE`, and
`PARITY_RUNNER_GPU=0` in the admitted process. `build.py` expects `base` and
`candidate` source trees alongside the artifact directory. Apply
`prepare_driver.py` to each scratch copy of the peer consumer before building;
this adds flexible and spatial meshlet operations with 64 vertices, minimum 41
and maximum 124 triangles, zero cone weight, split factor 2 and fill weight 0.5.
The original peer allocation and serialization policy applies. Copy the frozen
peer `inputs.json` and input files into the artifact directory before measuring.
`measure.py` interleaves arms, appends a durable record for each completed
sample and resumes only matching source, executable and input bindings. It uses
three samples and requires 100% counter running. Optimesh has no cluster LOD.

`exact.rs` is compiled twice, once against the archived base and once against
the candidate. It records result values, usage and complete caller buffers at
each work limit, and complete cluster LOD callback records and position bits.
`fuzz.rs` calls the p03 generator; `fuzz.py` records and resumes batches until
at least 300 child CPU seconds. A binary hash prevents mixing builds.
