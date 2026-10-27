# registered protocol finish handoff

Final measured source revision: `8319f7d3624694f7f71f2fe1f45ff977c10e17d9`. The documentation-only final commit preserves all measured source and consumer hashes. PR [#6](https://github.com/Tavrin/meshoptimizer-rs/pull/6) was merged as `f20a177` and released in 0.2.2.

Local CI-equivalent test-job commands passed on pinned Rust 1.99.1; both MSRV test commands passed on 1.88. The lint fix changes one regression-test loop to `as_chunks::<4>()`, with no encoded-path or API change. Global default toolchain is unchanged.

Parity: defaults: 17,264 comparisons, 0 mismatches, downstream: 17,264 comparisons, 0 mismatches.

Native allocating acceptance: **PASS**. Historical owned request: **PASS**. Separate shared-peer family target: **PASS**. Per-family/profile final values, case intervals, resolution and correctness: [P03](../P03_ENCODER_PERFORMANCE.md).

The retained build controller rebuilt a fresh `seen-frontier` epoch. The previous guarded epoch is preserved as `seen-frontier-pre-ci-0f04876`. Earlier `a6fc95f` and diagnostic-scout numbers are superseded for final acceptance, without deleting their receipts. Every final measurement burst used separate headless CPU-only heavy admission, no outer heavy scope, no quiet-machine wait and no borrowing. The defaults `vertex-streaming-s32-v1-l0` row was interrupted by the outer timeout and rerun from pair 0 under the unchanged rule; the interrupted samples and pre-disposition receipt are retained. The rerun received a second pair-4 decisiveness check, which stops on a decisive pass or fail alike; see `INTERRUPT-DISPOSITION.md`. Completed streams were not repeated. No additional optimization was attempted after qualification.

Durable receipts: `<scratch>/meshopt-enc03/seen-frontier`. Local CI log and epoch disposition: `<scratch>/meshopt-enc03/finish`. Portable [summary](../results/enc03-allocating-profiles.json) and [compressed receipts](../results/enc03-allocating-profiles.json.gz).

Reversal: the lint fix is one test-loop line; reverting it restores the CI lint failure. Prior epochs are retained for inspection but do not qualify changed source. Not verified: downstream runtime/cook, GPU/browser execution, universal performance or release fuzz budgets. No merge, publication or tag is authorized.
