# P02 decoder qualification

The library API is `meshoptimizer_rs::codec`. Raw vertex, triangle and sequence
decoders accept versions 0 and 1. `BufferView` checks EXT mode/filter/count/stride
and optional parent metadata. EXT attribute streams retain version 0. The
index helper retains Moss's version 0/1 domain. Output uses little-endian bytes
for both index widths. Version inspection checks the header only.

Each allocating and caller-buffer operation takes `Workspace`, preserving
explicit storage/work limits. Raw caller-buffer operations and post-filters
allocate no heap memory. Late malformed-stream or numerical errors may modify
the used prefix; destination tails are preserved. Codec work charges source
bytes plus decoded bytes before decoding; filters charge one visit per
four-byte word. Fixed stack scratch is excluded from heap storage accounting.

The scalar 1.3 oracle defines canonical filter output. Zero-length Oct normals
return `NumericalFailure`; their C++ float-to-int conversion is undefined and
is not an output oracle. Quat follows 1.3's unscaled arithmetic, including its
rounding sign for negative scale words. Exp follows scalar bit construction
and multiplication; upstream-defined exceptional results are not clamped.

Set `CARGO_TARGET_DIR` to
`/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p02` and `MESHOPT_REFERENCE`
to the clean pinned upstream checkout. The specified artifact directory was
read-only in this environment; `MESHOPT_ARTIFACTS` was set to
`/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02`. All generated
messages, output bytes, raw samples, logs, corpora and executable copies live
there. Source fixtures are exported from the pinned oracle without editing it.
Bar registration also needs `MESHOPT_MOSS_REFERENCE`, a Moss checkout.

`run.sh` and `sweep.sh` with `--phase 0.2` write their per-case records and
buffer archives to `$MESHOPT_ARTIFACTS/run-0.2` and `$MESHOPT_ARTIFACTS/sweep-0.2`
(or `--record-directory NAME`), and only slim SHA-256 summaries to
`parity/results/run-0.2.json` and `parity/results/sweep-0.2.json`.
`parity/report.sh --phase 0.2 --verify-artifacts` re-checks those identities,
every case buffer, and that the recorded sources are current.

```sh
parity/benchmark.sh --phase 0.2 --register-decoder-bars
parity/run.sh --phase 0.2 --profile scalar-strict
parity/sweep.sh --phase 0.2 --cases-per-family 2000 --seed 20261004
python3 parity/codec/gates.py
python3 parity/codec/fuzz.py
parity/benchmark.sh --phase 0.2 --enforce
python3 parity/codec/verify.py
```

Registration rejects an existing bar. The committed bar and retained baseline
are required for candidate measurement; the candidate never changes minima.
Performance includes validation, allocation and required copies, with warm-up
and at least twelve paired samples on one CPU. Candidate samples target 40 ms, rotate backend order and increase to sixty when dispersion crosses a gate. The matrix covers tiny,
resident and streaming buffers, both output widths, each filter, full views,
raw v1, representative encoder levels and a two-million-triangle buffer.

`run.sh` exports all applicable native decoder invocations, executes all five
unchanged upstream JS suites, and explicitly awaits the adapted decoder suite.
Color vectors are recorded as outside 0.2. Native, C++ and executed WASM
responses are compared; every-byte truncations and malformed headers retain
statuses without comparing failure output. `sweep.sh` runs 2,000 cases for each
of the three raw codecs, three filters and checked view. Both allocating and
caller-buffer APIs run on native and WASM. SIMD Oct/Quat conformance permits
one decoded integer unit separately from canonical scalar equality.

The unpublished WASM adapter uses checked byte exports and has no unsafe
blocks. Its linkage attributes remain outside the published crate. The core
retains `forbid(unsafe_code)` and pinned libm for std/no_std math.

`fuzz/codec` contains thirteen standard cargo-fuzz targets. Its instrumented
ASan/libFuzzer builds use cargo-fuzz 0.13.2 and libfuzzer-sys 0.4.13; this host
uses stable with `RUSTC_BOOTSTRAP=1` for the instrumentation flags. Each smoke
runs for at least 300 seconds, with corpus, crash directory, executions,
elapsed time, CPU, logs and binary/source hashes retained. Smoke evidence
does not establish the RFC's separate four-CPU-hour release fuzz gate or other
native target qualification. No package is published and Moss is not migrated.
