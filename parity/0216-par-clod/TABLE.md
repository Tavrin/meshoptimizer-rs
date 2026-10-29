# Measurements

Instruction counts are median of three resident-loop samples, `instructions:u`,
including worker threads. All five arms produce identical serialized output on
all eight peer meshes. Wall results are median of five interleaved pinned-core
samples on a shared host; they disclose speed rather than gate shipment.

| Mesh | v0.2.15 (Ginst) | Candidate serial | Candidate 4 workers | C++ scalar 1.3 | C++ default 1.3 |
|---|---:|---:|---:|---:|---:|
| tiny-smooth | 0.000099 | 0.000099 | 0.000099 | 0.000075 | 0.000075 |
| medium-smooth | 0.113146 | 0.113145 | 0.113309 | 0.099765 | 0.099766 |
| medium-seams | 0.109351 | 0.109349 | 0.109514 | 0.094521 | 0.094521 |
| medium-sparse | 0.114962 | 0.114961 | 0.115117 | 0.100755 | 0.100755 |
| million-smooth | 14.850888 | 14.850660 | 14.854962 | 13.010269 | 13.010273 |
| real-214e | 78.182486 | 78.180907 | 78.199508 | 65.734202 | 65.734197 |
| real-a082 | 88.386829 | 88.384971 | 88.409635 | 74.324065 | 74.324039 |
| real-b6fd | 21.899716 | 21.899279 | 21.905136 | 18.418151 | 18.418153 |

| Real mesh | Single thread (s) | 4 workers (s) | Speedup |
|---|---:|---:|---:|
| real-214e | 8.981425 | 5.490611 | 1.636x |
| real-a082 | 9.453394 | 5.278334 | 1.791x |
| real-b6fd | 2.168861 | 1.268288 | 1.710x |

Raw samples and output hashes: `counts.jsonl`; cores/protocol: `environment.json`.
