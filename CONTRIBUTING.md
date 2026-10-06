# Contributing

Bug reports and pull requests are welcome. For security issues, use
[SECURITY.md](SECURITY.md).

Rust 1.88 is the minimum supported version. CI requires:

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo test --locked --no-default-features
cargo build --locked --no-default-features
cargo build --locked --target wasm32-unknown-unknown --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
```

Every public item needs rustdoc. Unsafe code is restricted to the private
`codec::simd` module and its audited blocks in
[SAFETY.md](src/codec/simd/SAFETY.md). The no-default-features build forbids it;
run `parity/check-unsafe-boundary.sh` for both source and package checks.
Non-test code must not call `unwrap`, `expect`, or panic. All crate-controlled
heap allocations and size arithmetic must remain fallible and accounted for.
Add focused tests for changed behavior. Algorithm output evidence belongs in
the unpublished differential harness, not duplicated golden unit tests.

For numerical changes, run the corpus and sweep in
[parity/README.md](parity/README.md), preserving exact equality and the pinned
reference. Include source identities, retained failing inputs and raw
benchmark samples where relevant. Explicitly distinguish lane evidence from
complete release or consumer acceptance.

Commits must be signed off under the
[Developer Certificate of Origin](https://developercertificate.org/), using
`git commit -s`. Contributions are licensed under the MIT terms in [LICENSE](LICENSE).
