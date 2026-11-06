# Virtual geometry seed qualification

`attr_page` accepts at most 256 KiB of raw block/page/disk bytes. It validates
blocks and pages independently, visits every public decode accessor, and also
maps bytes to a valid generated fixture (3 through 256 vertices, at most 254
triangles). That fixture exercises all eleven encodings, including each raw
component count, multiple channels, half subnormals and endpoint rounding,
uniform and ranged materials, and both disk codecs. Attribute oracles use the
documented Euclidean/component bounds; raw floats and tangent signs are exact.
Position error is checked against the quantization grid's displacement bound.
Decoded coordinates, positions, triangles, attributes and materials must agree
between the memory view, both disk codecs, and the file reader's transcoder.

`v2_build` accepts at most 4096 bytes and maps them to 2 through 12 cell grids:
at most 169 source vertices and 288 triangles. Geometry, cluster policy,
material seams, disk codec and an optional 8 by 8 alpha mask vary with the
input. Every build supplies normals, UVs and triangle slots. Assertions cover
triangle slot uniformity, inherited error monotonicity, source-region coverage
along every leaf-to-root path at twelve fixed camera/threshold combinations,
writer acceptance, resident open and full-file reading. Byte identity is
checked between a bounded serial workspace and one/two worker pools. The latter
use the library's unlimited workspace policy to exercise its parallel source
measurement branch; input dimensions and alpha resolution remain capped.
Bounded workspaces allow 64 MiB and 2^32 work units. Fallback measurement also
checks that a finite, nonempty coarsest mesh can be extracted.

`dag_format_v2` retains raw and resealed input paths and now also flips a disk
payload byte and reseals both page and resident hashes. This allows mutations
of uncompressed attribute/page bytes to reach validation behind the hash gate.

Generate fixed inputs with `cargo run --release --manifest-path
fuzz/seedgen/Cargo.toml --bin vg-seedgen -- fuzz/seeds`, using the required build
admission environment. The generator uses the crate writer to emit MDAG files
with all encodings, material ranges and FARF metadata. It also saves raw blocks,
memory pages, disk pages and mesh controls for the two new targets.

Build with `FUZZ_GROUPS=r0220 python3 fuzz/driver.py build`. Measure fixed seeds
with `python3 fuzz/driver.py cover attr_page v2_build dag_format_v2`, then run
`python3 fuzz/r0220/report.py` for matching-toolchain `llvm-cov report` tables
and exact line counts. Coverage uses fixed-input replay only. Crash inputs go
to `fuzz/artifacts/<target>/`. These commands require the build admission wrapper
and environment supplied by the operator. No campaign is part of qualification.

Final coverage and gate results are recorded in `RESULTS.md`.
