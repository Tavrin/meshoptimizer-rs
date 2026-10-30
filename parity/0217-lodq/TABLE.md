# Final instruction guard

Unchanged simplify; median of three instructions:u samples per cell. All four outputs are byte-identical. Candidate enables lod-quality; C++ arms use pinned meshopt 0.7.0 / upstream 1.3. F9 specifies no instruction-ratio target for this additive API; this guard reports the shared pipeline comparison. No equivalent base or upstream quality API exists.

| Case | Base instructions | Candidate | C++ scalar | C++ default SIMD | Candidate/base |
|---|---:|---:|---:|---:|---:|
| tiny-smooth | 110645.84 | 110645.84 | 120315.51 | 120315.08 | 1.00000000 |
| medium-smooth | 12038800.00 | 12038791.00 | 10958272.00 | 10958317.00 | 0.99999925 |
| medium-seams | 15301356.00 | 15301329.00 | 14178405.00 | 14178459.00 | 0.99999824 |
| medium-sparse | 13015083.00 | 13015092.00 | 11868158.00 | 11868183.00 | 1.00000069 |
| million-smooth | 2426542817.00 | 2426542869.00 | 2256440699.00 | 2256440691.00 | 1.00000002 |
| real-214e | 3154100898.00 | 3154088391.00 | 2813433117.00 | 2813432890.00 | 0.99999603 |
| real-a082 | 5750946652.00 | 5750940563.00 | 5024122854.00 | 5024126932.00 | 0.99999894 |
| real-b6fd | 1435176706.00 | 1435176666.00 | 1250240398.00 | 1250240391.00 | 0.99999997 |

Wall time disclosure only (seconds per call; very short cells have coarse clock resolution):

| Case | Base | Candidate | C++ scalar | C++ default |
|---|---:|---:|---:|---:|
| tiny-smooth | 0.000009999 | 0.000009999 | 0.000009999 | 0.000009999 |
| medium-smooth | 0.000999936 | 0.000999936 | 0.000999936 | 0.000999937 |
| medium-seams | 0.000999936 | 0.000999936 | 0.000999936 | 0.000999936 |
| medium-sparse | 0.001999872 | 0.000999935 | 0.001999872 | 0.000999936 |
| million-smooth | 0.253983771 | 0.261983261 | 0.286981662 | 0.311980066 |
| real-214e | 0.844946012 | 0.858945116 | 1.048932977 | 0.989936746 |
| real-a082 | 1.988872918 | 2.159861991 | 2.332850935 | 2.325851386 |
| real-b6fd | 0.220985880 | 0.224985624 | 0.279982110 | 0.287981598 |

The first campaign is retained in receipts/perf-initial. The final quality-only lint fix changed the driver binary hash, so the complete 96-sample campaign was repeated against final frozen source. Final results use only the second campaign. The source/binary hashes and raw samples are retained in receipts/.
