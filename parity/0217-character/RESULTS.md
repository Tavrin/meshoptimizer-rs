# Character LOD comparison

## Sources and reproduction

The importer selects the largest triangle primitive in each model (it does not
merge material primitives or apply node transforms). Positions and deviations
are in that primitive's object-space units. Source GLBs and extracted streams
are external artifacts, never repository fixtures.

- [Fox](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/Fox):
  PixelMannen model, CC0; tomkranis rig/animation and AsoboStudio/scurest glTF
  conversion, CC BY 4.0. 1,728 vertices, 576 triangles, 24 bones.
  GLB SHA-256 `d97044e701822bac5a62696459b27d7b375aada5de8574ed4362edbba94771f7`.
- [CesiumMan](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/CesiumMan):
  Cesium, CC BY 4.0; Cesium logo subject to the model's separate legal-mark notice.
  3,273 vertices, 4,672 triangles, 19 bones.
  GLB SHA-256 `b7001eaeea8254bd44773bcd247e78696d94169388fbb2a1800fc69434e777d9`.
- [MorphPrimitivesTest](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/MorphPrimitivesTest):
  ft-lab, CC BY 4.0 (uncompressed GLB; Draco conversion credit Frank Galligan).
  Largest primitive: 21 vertices, 24 triangles, one blend target.
  GLB SHA-256 `2a5ed6470aca41edc813ef5b7eecf2af53264d8e0ab9e6f32156d54245ca18cf`.

Download with `python3 parity/0217-character/models.py <external-directory>`.
The input format is documented in `examples/character_compare.rs`. Build that
example in release mode, then pass each extracted `.bin` file. Confirm downloaded
hashes above; the download URLs track the sample asset repository's main branch.

Targets are source-relative 50%, 25%, 10%, rounded down to whole triangles;
each level is independent. Error cap 100 allows topology/metric effects to be
observed without tuning the cap per model. All metric weights and bone
importance are one. `plain` calls the original simplify entry point;
`character` enables bone and full blend-position channels; `skin_only` disables
the blend-position channels' values as a mechanism ablation.

Samples include bind and full-weight blend targets. Each skinned model also has
one synthetic diagnostic pose: bone b translates in Y by
`0.1 * rest_extent * ((b % 3) - 1)`. These are supplied model-space skin matrices,
not a glTF animation evaluation or a claim about animation quality. F9 density
zero measures all triangle corners in both directions; this isolates joint/morph
vertex motion but does not bound interior surface error. The maximum column is
the largest sampled symmetric distance across these poses.

Times exclude I/O, sample construction and F9 measurement. They include channel
preparation, validation, allocation and simplification. One warmup precedes seven
warm repetitions, reporting the median. Every repetition must produce the same
indices and error bits. Methods run consecutively in the listed order; these
small-model timings are descriptive, not a controlled performance qualification.

## Mechanism

The existing attribute metric accumulates rest geometry plus exact bone-weight
channels and every full blend target's normalized positions. It can reuse the
unchanged simplifier and its collapse selection, preserving existing outputs.
The pose reports measure geometric deviation independently of its summed
attribute energy. Extra pose quadrics are not implemented or directly timed;
this comparison can establish the attribute mechanism's benefit and cost, not
its superiority to a separate pose-quadric implementation. The bounded API
rejects more than 32 channels without truncation or lossy bone-ID projection.

## Results

Linux x86-64, AMD Ryzen 9 7945HX, release default features. Times are rounded
milliseconds under shared-host load; no performance threshold is claimed.
Distances are object-space units. P/C means plain/character. Max pose includes
bind and the separately sampled deformation poses described above.

| Model | Target | Triangles P/C (requested) | Bind P/C | Max pose P/C | ms P/C |
|---|---:|---:|---:|---:|---:|

| Fox | 50% | 576/576 (288) | 0/0 | 0/0 | 0.040/0.195 |
| Fox | 25% | 576/576 (144) | 0/0 | 0/0 | 0.040/0.193 |
| Fox | 10% | 576/576 (57) | 0/0 | 0/0 | 0.041/0.200 |
| CesiumMan | 50% | 2336/2336 (2336) | 0.00159047/0.00359471 | 0.0631619/0.00652756 | 1.000/3.000 |
| CesiumMan | 25% | 1168/1168 (1168) | 0.00922888/0.0193594 | 0.0832346/0.0193594 | 1.000/3.000 |
| CesiumMan | 10% | 590/592 (467) | 0.651062/0.651062 | 0.736567/0.736567 | 1.314/2.966 |
| MorphPrimitivesTest | 50% | 11/12 (12) | 0/0 | 0.0928477/0.0870388 | 0.007/0.006 |
| MorphPrimitivesTest | 25% | 6/6 (6) | 0/0 | 0.096225/0.096225 | 0.005/0.009 |
| MorphPrimitivesTest | 10% | 2/2 (2) | 0.447214/0.353553 | 0.447214/0.366414 | 0.007/0.008 |

### Decision and limits

Retain the existing attribute mechanism: CesiumMan's sampled maximum decreases
from 0.063162 to 0.006528 at 50% and from 0.083235 to 0.019359 at 25%, at a bind-pose
cost (0.001590 to 0.003595 and 0.009229 to 0.019359). At 10% both methods miss
the 467-triangle target, reaching 590/592 triangles with the same sampled maximum.
Fox reaches no target: both methods retain 576 triangles, so its zero deviation
is an identity result and establishes no character-quality benefit.

The morph-channel ablation reproduces plain output distances: at 50%, disabling
target channels gives 0.092848 max rather than 0.087039, but also produces 11 rather
than 12 triangles; this row is not an equal-triangle quality comparison. At 25%
both give 0.096225 at six triangles. At 10%, enabling the full target channels
reduces sampled maximum from 0.447214 to 0.366414 at the same two triangles.
This supports including every target in the existing attribute metric.
It does not establish a universal quality win or a comparison against unimplemented
extra pose quadrics. Separate pose quadrics remain an unmeasured alternative,
especially for rigs exceeding the explicit 32-channel budget.

### Validation

Every command exited 0: formatting, all-target/all-feature clippy with warnings
denied, default tests and doctests, all-feature tests and doctests, no-default-feature
tests and doctests, no-default-feature wasm32-unknown-unknown build, public-text
guard, pinned C++ reference check, exact C++/native Rust/wasm differential run,
release comparison build, and all three model comparisons (including character
chain pose-report generation). The exact run checked 279 cases across vertex
cache, overdraw, simplify, attribute simplify, and scale, with zero mismatches.
Existing algorithms and parity fixtures were not edited. No Git state was changed.

The new tests cover empty-input equivalence, exact metric construction (bone
identity, importance, full targets), eight influences, malformed streams, channel
overflow, bind/blend/joint pose reports, and an exact whole-call work boundary.
Raw logs, model files and differential receipts remain in the external scratch
directory used for this run. The fresh build target was removed after validation.
