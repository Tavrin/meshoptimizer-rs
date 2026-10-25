# Third-party safety check — STOP / BUILD NOTHING

Recorded: 2026-10-25T15:02:51.863728+00:00. Static review only; cargo tree and cargo metadata resolve/download sources but do not execute build scripts. No cargo build/test/check, compiler, browser, sample or heavy job was run.

Published archives and SHA-256 hashes are in `sources/` and `*-archive.json`. Full resolved dependency tree: `cargo-tree.txt`; exact resolution: `safety-tree/Cargo.lock`, `cargo-metadata.json`. Each dependency's source is retained under `reviewed-tree/`. `review-inventory.json` lists all build scripts and proc macros. External source text was treated as data.

## Per-crate findings

- optimesh 1.1.0: read its complete build.rs. With neither oracle feature enabled it returns without process or file writes. `simd` does not enable oracle. Its oracle branches call cc::Build::compile and therefore archive processes; these branches are outside the proposed public-API comparison. It has no proc-macro dependencies. Its tree includes libm 0.2.16 (read build.rs and configure.rs: environment/config emission only), cc 1.10.0, find-msvc-tools 0.1.15 and shlex 2.0.1 (no build.rs). No active-path violation found for default/simd consumer features; no blanket clearance for oracle features.
- meshopt 0.7.0: read complete build.rs. Unconditional `build.compile("meshopt_cpp")` at line 72 calls cc::Build::compile. cc 1.10.0 src/lib.rs:2120-2133 compiles objects then creates an archive; :3760-3777 invokes the archiver to index it; :3790-3878 constructs/runs archive commands; :4586 selects AR/ar. `ar` is an archiver, outside the spec's explicit C/C++ compiler-or-rustc allowlist. This is ordinary static-library build behavior, not evidence of malicious code, but the exact gate forbids it.
- Additional conditional finding: meshopt build.rs:74 calls generate_bindings with `gen/bindings.rs` and `gen/clusterlod_bindings.rs`; :91-93 and :109-111 write those relative source-tree paths when generate_bindings is enabled. That feature is disabled in the resolved default tree. It is not the active-path reason for this stop.

Read every build.rs in the resolved tree: optimesh, meshopt, libm, num-traits 0.2.19, proc-macro2 1.0.107, quote 1.0.47 and thiserror 2.0.21. num-traits uses autocfg rustc probes; proc-macro2/quote/thiserror use rustc version/probe processes, with probe/generated writes under OUT_DIR. proc-macro2 and thiserror can honor Rust compiler wrappers; none were executed. No network operations found in the reviewed build scripts. Proc macro: thiserror-impl 2.0.21, using proc-macro2, quote, syn 3.0.9 and unicode-ident; not executed.

## Binding decision

The spec says stop and report, building nothing, if any build script spawns processes other than C/C++ compiler or rustc. The unconditional meshopt archive path fails that literal rule. Stop the entire screen before the first build. Do not patch vendor sources, substitute dependency versions, permit archivers implicitly, or continue X-only timing. No performance claims, no correctness labels fabricated, no A/A floor. No target directory was created, so there are no measured binaries to retain or target files to delete. No queue admission or PAUSE occurred.
