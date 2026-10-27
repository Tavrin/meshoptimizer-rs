# Contract simplification qualification

**NO-GAIN; proof in progress.** Campaign 2 measured the positional-only library source at 0967578. No candidate/base case exceeded 1.01; the maximum was 1.000154. The real-mesh PASS rule and PARTIAL rule both failed. Campaign 1 rejected the combined positional and grid-meter revision; the grid-meter change was reverted before this campaign.

The prior contract-cost experiment attributed 44.2% of the pooled simplify gap, 52.8% of the attributes gap and 68.0% of the overdraw gap to finite checks conditional on work/root ablation. Work accounting explained 40.4% of the sloppy gap. These are conditional instruction attributions, not a unique decomposition.

The positional change removes the finite scan of positional quadrics (`src/simplify.rs:1741`, `src/simplify.rs:1859`, `src/simplify.rs:2466`); the bound is in PROOFS.md. Attribute and gradient checks remain. Native roots were already present; the portable path is unchanged. Earlier overdraw variants failed their regression gate and were not included.

Median instructions per call from three runs per arm; ratios are candidate/base, candidate/C++ 1.3 and candidate/optimesh 1.1.0. Ratios below 1 mean fewer instructions. The peers-format inputs use synthetic attributes.

| Input | Operation | Candidate/base | Candidate/C++ | Candidate/optimesh |
|---|---|---:|---:|---:|
| tiny-smooth | simplify | 0.9887 | 0.9266 | 0.9571 |
| tiny-smooth | simplify-attributes | 0.9904 | 1.0289 | 0.9840 |
| tiny-smooth | simplify-update | 0.9899 | 0.9592 | 0.9756 |
| tiny-smooth | simplify-sloppy | 1.0000 | 1.3634 | 0.9795 |
| tiny-smooth | simplify-prune | 1.0000 | 1.5373 | 1.1647 |
| tiny-smooth | simplify-points | 1.0000 | 1.3848 | 0.8826 |
| tiny-smooth | overdraw | 1.0000 | 0.7922 | 0.8906 |
| medium-smooth | simplify | 0.9823 | 1.1096 | 0.9377 |
| medium-smooth | simplify-attributes | 0.9926 | 1.1261 | 0.9436 |
| medium-smooth | simplify-update | 0.9861 | 1.1427 | 0.9980 |
| medium-smooth | simplify-sloppy | 1.0000 | 1.3562 | 0.9958 |
| medium-smooth | simplify-prune | 1.0000 | 1.4780 | 1.2048 |
| medium-smooth | simplify-points | 1.0000 | 1.6223 | 1.0202 |
| medium-smooth | overdraw | 1.0000 | 1.0328 | 0.9131 |
| medium-seams | simplify | 0.9830 | 1.0903 | 0.9246 |
| medium-seams | simplify-attributes | 0.9911 | 1.1436 | 0.9588 |
| medium-seams | simplify-update | 0.9855 | 1.1277 | 0.9705 |
| medium-seams | simplify-sloppy | 1.0000 | 1.4618 | 0.8438 |
| medium-seams | simplify-prune | 1.0000 | 1.4958 | 1.2043 |
| medium-seams | simplify-points | 1.0000 | 1.4903 | 0.8369 |
| medium-seams | overdraw | 1.0002 | 1.0515 | 0.9171 |
| medium-sparse | simplify | 0.9686 | 1.1265 | 0.9477 |
| medium-sparse | simplify-attributes | 0.9843 | 1.1816 | 1.0000 |
| medium-sparse | simplify-update | 0.9754 | 1.1736 | 1.0206 |
| medium-sparse | simplify-sloppy | 1.0000 | 1.4164 | 0.9495 |
| medium-sparse | simplify-prune | 1.0000 | 1.5412 | 1.2108 |
| medium-sparse | simplify-points | 1.0000 | 1.6679 | 1.0911 |
| medium-sparse | overdraw | 1.0000 | 1.0842 | 0.9412 |
| million-smooth | simplify | 0.9894 | 1.0787 | 0.9067 |
| million-smooth | simplify-attributes | 0.9937 | 1.1257 | 0.9321 |
| million-smooth | simplify-update | 0.9910 | 1.1041 | 0.9486 |
| million-smooth | simplify-sloppy | 1.0000 | 1.3248 | 1.0523 |
| million-smooth | simplify-prune | 1.0000 | 1.4749 | 1.2061 |
| million-smooth | simplify-points | 1.0000 | 1.5943 | 0.9754 |
| million-smooth | overdraw | 1.0000 | 1.0302 | 0.9091 |
| real-214e | simplify | 0.9107 | 1.2600 | 1.0172 |
| real-214e | simplify-attributes | 0.9535 | 1.4453 | 1.2587 |
| real-214e | simplify-update | 0.9318 | 1.4278 | 1.1445 |
| real-214e | simplify-sloppy | 1.0000 | 1.5153 | 0.8155 |
| real-214e | simplify-prune | 1.0000 | 1.6090 | 1.2302 |
| real-214e | simplify-points | 1.0000 | 1.7117 | 1.0915 |
| real-214e | overdraw | 1.0000 | 1.4693 | 1.1880 |
| real-a082 | simplify | 0.9432 | 1.2230 | 0.9571 |
| real-a082 | simplify-attributes | 0.9655 | 1.3649 | 1.1375 |
| real-a082 | simplify-update | 0.9523 | 1.3320 | 1.0415 |
| real-a082 | simplify-sloppy | 1.0000 | 1.5035 | 0.8714 |
| real-a082 | simplify-prune | 1.0000 | 1.6023 | 1.2312 |
| real-a082 | simplify-points | 1.0000 | 1.7132 | 1.0912 |
| real-a082 | overdraw | 1.0000 | 1.4371 | 1.1679 |
| real-b6fd | simplify | 0.9431 | 1.2266 | 0.9570 |
| real-b6fd | simplify-attributes | 0.9655 | 1.3678 | 1.1378 |
| real-b6fd | simplify-update | 0.9522 | 1.3354 | 1.0414 |
| real-b6fd | simplify-sloppy | 1.0000 | 1.5115 | 0.8231 |
| real-b6fd | simplify-prune | 1.0000 | 1.6030 | 1.2307 |
| real-b6fd | simplify-points | 1.0000 | 1.6394 | 0.9765 |
| real-b6fd | overdraw | 1.0000 | 1.4371 | 1.1679 |

The 672 samples cover eight inputs and seven operations. Baseline, candidate and C++ output hashes matched in every cell and repetition. The largest candidate/base ratio was 1.000154. Real-mesh candidate/base ratios for simplify were 0.9107, 0.9432 and 0.9431; for attributes 0.9535, 0.9655 and 0.9655; and for overdraw 1.0000 on all three. The candidate did not beat optimesh on all three required families and meshes, and neither attributes nor overdraw reached 0.90 on all three.

The first campaign's source and raw evidence remain retained; its rejected results are in `rejected-results.json`. The registered driver source was cleaned after campaign 1 and could not be recovered with its recorded hash. Campaign 2 rebuilt all four arms from one reconstructed peers driver, keeping the same registered workloads, compiler profile, input hashes and call shapes. Its source and binary hashes are retained with the samples. The first reconstruction build attempt exited 101 before compilation due to a baseline manifest path; the corrected build and measurement exited 0. Each arm's target tree was removed after its binary was retained. All builds and measurements used CPU-only admission, a 4 GB declaration and `CARGO_INCREMENTAL=0`.

The standard exact comparison passed 1,200,243,710 simplify and 20,558,672 overdraw observations. All-feature and no-default tests, clippy, fmt, boundary and public-text checks exited 0. The proof runner then exited 75 before the no-default exact build when free space fell below the 100 GiB floor; the widened differential fuzz passed 1,363 seeds and 300.225 child CPU seconds. The no-default exact comparison and C++ differential remain pending. The expanded simplify matrix crosses all 512 option masks with eight represented position layouts, three attribute storage layouts, widths 0, 1, 3 and 32, and absent or mixed vertex flags. It does not exhaust the other supported attribute widths, arbitrary per-vertex flag combinations, or every valid stride and offset; the requested all-configuration comparison therefore remains unproven even if this sweep succeeds.
