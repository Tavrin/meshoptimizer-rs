# Known pinned upstream behavior

meshoptimizer 1.3, commit `4c203430ca565cb59a468a91922c76c208169536`.

## Odd meshlet malformed tail: scalar accepts, SIMD rejects

Frozen seed case `seed-20261005-op20-9264` has five vertices, one triangle,
vertex/triangle widths 4/4. The corrupted final code byte is `0x1c`.
Upstream scalar returns 0 and 24 output bytes; upstream SSE SIMD returns -3
(final consumption disagreement). Rust scalar, SSE2, SSSE3, SSE4.1 and
executed wasm scalar/SIMD return 0 and match the scalar output exactly.
Both allocating and caller-buffer driver paths reproduce the disagreement.
This is known upstream behavior, not a reason to change Rust acceptance,
exclude a frozen input, alter the oracle or declare the strict expanded
scalar/SIMD conformance sweep green.

Encoded meshlet (27 bytes):

```text
943fe890013fd1c493776900000000000000000000000000f2101c
```

Full MC02 request SHA-256:
`1602803bd71de17b647bdfdf64f7f5e1e9230836796c257359ba1491c5c97512`.
Scalar output SHA-256:
`84f10ea2dbf10053d297030ad90332fba171139f972049575862fda7f5c777f7`.

Reproduce using this checkout's frozen driver protocol and pinned binaries:

```sh
source /mnt/linux-extra/meshopt-artifacts/p07-fix5/env.sh
python3 parity/reproduce-upstream-tail.py
```

The script embeds the entire request, verifies its hash, asserts the expected
split on both APIs and records source/binary/output identities in
`/mnt/linux-extra/meshopt-artifacts/p07-fix5/upstream-tail-reproduction.json`.
Original failure and partial sweep archive remain under `p07-fix4`:
`sweep04-failure.json`, `seed-failure-reproduction.json`, `sweep04.zip`.
