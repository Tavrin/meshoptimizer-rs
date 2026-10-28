# Caller-defined vertex-cache score tables

Base: `dbb75e0` (v0.2.14). Scope: E2, opt-in API only. No release version
change, performance optimization, merge, tag or publication.

## API and preservation argument

`VertexScoreTable::new` validates finite slices with exactly 17 cache-position
scores and 9 live-valence scores and copies them to private inline arrays.
There is no allocation, mutable table access or variable cache capacity.
`Default` copies the unchanged upstream standard constants. Read-only accessors
allow callers to copy and tune either array before constructing another table.

`optimize_vertex_cache_with_table` and its `_into` form mirror the existing
standard wrappers: the same topology, sizing, reservation, output allocation,
usage finalization and error precedence. Caller-owned table and destination
storage do not count toward workspace limits; allocated output does.

The shared kernel receives a statically dispatched scorer. Standard and strip
APIs instantiate zero-sized presets with the original constant expressions.
The only loop changes pass the scorer and replace the two scoring calls.
Cache/adjacency construction, triangle selection, floating-point addition
order, comparisons, ties, writes, work charging and scratch accounting are
unchanged. No reassociation or new native-root operation is introduced.
FIFO and all unrelated algorithms are untouched.

## Tests and commands

Run `bash parity/0215-cache-tables/checks.sh` through the required CPU build
admission wrapper with reservation 4 GiB, `MOSS_HEAVY_GPU=0`,
`CARGO_INCREMENTAL=0`, `LC_ALL=C`, the user-selected isolated
`CARGO_TARGET_DIR`, `MESHOPT_CACHE_CORPUS` pointing to the retained inputs,
and `MESHOPT_CACHE_LOGS` pointing to a retained local log directory.
The script records each exit in `proof-exits.tsv`; source bindings are in
`source.sha256`. The exact commands are retained in the script.

- `cargo test --locked --all-features`: 208 unit/integration tests and 25
  doctests pass; the existing exhaustive root probe and five migration snippets
  remain ignored. The corpus test is explicitly run separately.
- `cargo test --locked --no-default-features`: 165 unit/integration tests and
  22 doctests pass, with the same ignored categories.
- The table validation test checks short/long tables and NaN, positive infinity
  and negative infinity in every one of the 26 entries. Extreme finite values
  are accepted, as specified.
- The zero-score custom table deterministically preserves a known input order
  that differs from the standard result; repeated allocating and `_into` calls
  share a workspace and retain the destination tail.
- The small-mesh differential checks default-table versus standard Results,
  full destination buffers and Usage at every work prefix through success,
  four byte limits, short/long destinations, malformed topology, invalid indices,
  empty input and degeneracies. Repeated `_into` calls also exercise reuse.
- The explicit ignored corpus test compares full default-table output and
  Usage with the existing API, then checks `_into` output and untouched tails,
  on all five benchmark meshes and three real meshes in both feature modes.
  `corpus.sha256` binds the retained complete input files. Geometry is not
  committed. Each file starts with little-endian vertex/index counts, packed
  XYZ positions, and original u32 indices; trailing harness fields are unused.
- Explicit doctests in both modes, clippy with all targets and warnings denied
  in both modes, formatting, source unsafe boundary, package unsafe boundary,
  public-text check and whitespace check are part of the driver.

The package boundary uses `cargo package --locked --allow-dirty --list` piped
to `parity/simd/check_package.py`. It runs the same inventory check as the
existing boundary shell script without a verification-disabling option.

The first full driver run passed tests, both corpus modes and all other checks
but failed both clippy modes (exit 101): the new test reader used constant-sized
`chunks_exact`. Replacing it with `as_chunks::<4>()` addresses that lint without
changing the parsed data. `initial-proof-exits.tsv` retains that failed run;
`proof-exits.tsv` records the subsequent complete rerun, using separate logs.
Final result: driver exit 0, all 13 checks exit 0. Source and all eight corpus
SHA-256 bindings were also checked successfully.

## Limits of the evidence

No performance campaign or target-GPU tuning was run. No fresh C++ differential,
cross-target build, exhaustive floating-point probe or combined release
qualification was run. Default identity here is against the existing crate
API, with source-level preservation of the upstream presets. Extreme finite
tables can overflow intermediate scores; output quality is not guaranteed.
The read-only adversarial review in `REVIEW.md` is same-author, not independent.
No shared helpers outside `src/cache.rs` changed; `src/lib.rs` only adds exports.
