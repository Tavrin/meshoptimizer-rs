# Final measurements

Instructions: median of three samples per cell. The table reports the geometric mean over the three real meshes; worst includes all eight fixtures.

| Function | / v0.2.15 | / scalar C++ | / default C++ | Worst / base | Verdict |
|---|---:|---:|---:|---:|---|
| remap-vertex | 0.779 | 1.087 | 1.087 | 0.926 | SHIPS |
| remap-index | 0.885 | 1.437 | 1.437 | 0.886 | SHIPS |

All four arms returned identical bytes in all 16 operation/input cells. All 192 samples ran with 100% counter availability. The initial rejected candidate remains in campaign-1.

## Wall time disclosure

Median milliseconds per resident-loop call; CPU contention and scheduling are not controlled by the instruction gate. No wall-time acceptance claim.

| Case | Function | Base ms | Candidate ms | Scalar C++ ms | Default C++ ms |
|---|---|---:|---:|---:|---:|
| tiny-smooth | remap-vertex | 0.000053 | 0.000045 | 0.000046 | 0.000050 |
| tiny-smooth | remap-index | 0.000069 | 0.000066 | 0.000056 | 0.000051 |
| medium-smooth | remap-vertex | 0.005130 | 0.004569 | 0.004218 | 0.003710 |
| medium-smooth | remap-index | 0.013893 | 0.012711 | 0.009311 | 0.009468 |
| medium-seams | remap-vertex | 0.004989 | 0.005477 | 0.004208 | 0.005751 |
| medium-seams | remap-index | 0.014273 | 0.013051 | 0.009167 | 0.008997 |
| medium-sparse | remap-vertex | 0.007625 | 0.006819 | 0.008232 | 0.009965 |
| medium-sparse | remap-index | 0.013522 | 0.011478 | 0.008486 | 0.008740 |
| million-smooth | remap-vertex | 0.846804 | 0.784126 | 0.708894 | 0.731907 |
| million-smooth | remap-index | 3.346298 | 2.291031 | 1.867194 | 1.827309 |
| real-214e | remap-vertex | 33.885040 | 39.746672 | 34.616677 | 34.879792 |
| real-214e | remap-index | 23.938170 | 24.509175 | 20.576854 | 23.782387 |
| real-a082 | remap-vertex | 30.386415 | 31.508988 | 31.831354 | 31.997767 |
| real-a082 | remap-index | 40.346569 | 41.845280 | 42.885737 | 38.517246 |
| real-b6fd | remap-vertex | 3.206384 | 3.296233 | 2.761247 | 2.832571 |
| real-b6fd | remap-index | 2.203467 | 1.621371 | 1.288856 | 1.231829 |

## Per-case instruction ratios

| Case | Function | Candidate / base | Candidate / scalar C++ | Candidate / default C++ |
|---|---|---:|---:|---:|
| tiny-smooth | remap-vertex | 0.925589 | 1.021393 | 1.021495 |
| tiny-smooth | remap-index | 0.869721 | 1.207697 | 1.207737 |
| medium-smooth | remap-vertex | 0.787173 | 1.086089 | 1.086039 |
| medium-smooth | remap-index | 0.885573 | 1.431555 | 1.431441 |
| medium-seams | remap-vertex | 0.785689 | 1.086211 | 1.086211 |
| medium-seams | remap-index | 0.885516 | 1.431454 | 1.431368 |
| medium-sparse | remap-vertex | 0.757968 | 1.063248 | 1.063028 |
| medium-sparse | remap-index | 0.885514 | 1.431467 | 1.431438 |
| million-smooth | remap-vertex | 0.779369 | 1.086390 | 1.086389 |
| million-smooth | remap-index | 0.884639 | 1.437355 | 1.437353 |
| real-214e | remap-vertex | 0.779335 | 1.086526 | 1.086527 |
| real-214e | remap-index | 0.884636 | 1.437380 | 1.437379 |
| real-a082 | remap-vertex | 0.779336 | 1.086526 | 1.086527 |
| real-a082 | remap-index | 0.884634 | 1.437383 | 1.437386 |
| real-b6fd | remap-vertex | 0.779357 | 1.086521 | 1.086524 |
| real-b6fd | remap-index | 0.884645 | 1.437317 | 1.437318 |
