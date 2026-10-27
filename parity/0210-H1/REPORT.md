# registered protocol job H1: cache/strip and FIFO hot loops
Status: standard cache PARTIAL (kept); FIFO NO-GAIN (reverted to 0.2.9 source).
Base v0.2.9 df7fc56. Safe Rust; no new unsafe, no get_unchecked, no API change.

## Techniques
- Pre-sliced scratch and length asserts: src/cache.rs:115-121, 217-218, 237.
- Adjacency build: precharge when all corner visits fit, otherwise per corner: src/cache.rs:136-154.
- Maximum incidence d computed in the existing initialization scan: src/cache.rs:157.
- Whole-loop bound f*(38+22d), with overflow taking the fallback (src/cache.rs:173-204). If it
  fits, prepay it, run the FAST cache_loop (src/cache.rs:207) accumulating exact visits, and refund
  bound minus spent (CacheWork src/cache.rs:588-660; Work::refund_cache src/workspace.rs:650-656).
  Otherwise the !FAST path delegates every charge to the 0.2.9 Work calls.
- Iterator removal search: src/cache.rs:251, 635.

Proof: final remaining = remaining0 - spent; spent sums the same per-visit counts as 0.2.9;
spent <= bound <= remaining0, so 0.2.9 cannot exhaust either. Per emission 1+16+3d+19+19d;
cursor visits <= 2f in total.

Review fixes:
- the cache bound goes from 37 to 38 per face, because a cursor stop is revisited;
- the FIFO bound 3n+2v+1 under-reserved, since isolated [i,i,i] faces need 3n+3v-2;
- finish() debug-asserts and saturates;
- cursor-heavy meshes were added to the exact.rs sweep and to tests/preprocessing.rs:1170.

## Instruction gate (median of 3, instructions:u, perf 6.17, CPU 9)
| Family | vs 0.2.9 | vs C++ 1.3 | Verdict |
|---|---:|---:|---|
| cache, frozen corpus | 0.913 | 1.318 | PARTIAL |
| cache, all calls | 0.906 | 1.260 | PARTIAL |
| chain 214e | 0.917 | 1.216 | PARTIAL |
| chain a082 | 0.899 | 1.252 | PARTIAL |
| chain b6fd | 0.899 | 1.252 | PARTIAL |
| FIFO, campaign 1 | 0.966 | 1.298 | NO-GAIN, reverted |
| FIFO, final | 1.000 | 1.344 | unchanged |

Worst cache case against 0.2.9: 0.935. The per-case table is in
<scratch>/meshopt-0210-H1/results.json.

## Gates (exit codes)
- test all-features 0; test no-default 0; clippy 0; fmt 0; boundary 0.
- exact std 0; exact no-std 0.
- cpp-proof 0 (466 C++ cases + 4,098 seeds).
- fuzz 0 (300 CPU-s, 1,124 shards).
- measure 0; results 0.

Not verified: strip has no registered performance corpus, so it has output parity only. No wall
timings were taken.
