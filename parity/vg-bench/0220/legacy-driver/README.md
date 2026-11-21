# Legacy byte comparison driver

The frozen baseline library sources are identified by the initial hashes in
`../custody.json`. Its package version has the suffix `-baseline` to allow both
path dependencies to link. Include the examples and the README, USING and migration
documents named by its manifest and `include_str!` declarations.

Copy the three Rust files to `src/`, substitute the two directory placeholders in
`Cargo.toml.in`, and retain the supplied lock. In an admitted CPU job run
`cargo run --release --locked --offline --manifest-path <driver>/Cargo.toml`.
`VG_EVIDENCE` points to the qualification evidence directory with the five VGM
inputs and `final/<mesh>.bevy.8T.bin` artifacts. The driver asserts exact equality
on the three grid fixtures and all five source meshes at 1/8 workers. It also
accounts the seven slices in each saved reference asset.
