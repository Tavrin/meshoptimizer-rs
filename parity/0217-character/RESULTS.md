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

## Round 2: real animations and merged meshes (2026-10-31)

**All required gates passed; three earlier jobs remain queued. Target cleanup waits for them.**

Everything above is the unchanged round-one record. Its former channel limit is
superseded by the fix described below.

The accepted corpus contains 12 models: 11 rigged characters and MorphStressTest
as an auxiliary morph stress fixture. RiggedSimple and MorphPrimitivesTest are
additional retained sanity runs, outside that count. Source-page licence checks, author credits, download URLs, file sizes
and SHA-256 hashes are retained in [SOURCES.json](SOURCES.json). Corpus GLBs total
16,801,844 bytes. All retained GLB downloads, including the two sanity fixtures
and an unused minion-b01 alternate, total 17,045,036 bytes. BrainStem was excluded because its source page lists a Poser
EULA. The official base-character download returned HTTP 403; the two derived
exports instead have source-page provenance and a preserved original CC0 licence.
Gangnam's original geometry is MIT, with CC0 skeleton/hands. MakeHuman's generated
body, morphs and baked clips are CC0 independently of its generator's GPL licence.

This is a bounded API evaluation, not production acceptance or a general claim
of better quality. The baseline rejected four well-formed inputs at 53, 65, 65 and 329 channels.
The fixed character path accepts these exact streams within the existing workspace
limits. A rigid morph head is represented by its actual source-node transform;
RobotExpressive's 44 deformation transforms comprise 43 authored joints plus that
one rigid node. This changes no source animation and relaxes no weight validation. Superhero and Gangnam have no embedded clips; no animation
was synthesized or retargeted for them. MakeHuman combines face/body proportion, blink and gaze targets; this is not
qualification on an ARKit capture head.

### Method and reproduction

Use `eval_models.py EXTERNAL_MODELS` with the GLBs arranged in one directory per
model as in the source manifest; it requires NumPy. Build the `character_eval`
example in release mode and run it on each `.eval.bin`. `render_eval.py
EXTERNAL_CAMPAIGN` reads frozen `final-inputs/` (or `models/`) and `logs/` and produces diagnostic PNGs.
The retained raw campaign additionally contains logs, CSVs, input streams, pose
labels/key times, output indices, source/binary hashes and exit receipts. Model
bytes and images are external, never repository fixtures. The alternate
minion-b01 download is retained but excluded from the accepted corpus.

All skinned triangle primitives in the active scene are merged in model space;
morph-only primitives are included too. Unskinned rigid attachments and scene
props are excluded. In particular, RobotExpressive's rigid torso/limbs are not
part of the skinned comparison. Node matrices transform positions/deltas and
inverse-transpose normals. Skin channels use global joint-node identities,
including every declared bone. Named morph targets shared by a character's mesh
parts are combined by target name; unnamed targets are local to a mesh node.
Rigid morph parts in an otherwise skinned character have one full influence on
their actual source-node transform. Primitive indices are promoted before global
offsets are added, including the 100k-vertex merged model. All original vertex
semantics, material IDs and morph normal/tangent deltas are included in exact byte weld keys, split into supported streams for the crate's
`generate_vertex_remap_multi`. No position tolerance or lossy joint projection
is used. Skin weights are normalized on import; no influence is discarded.
An independent equality audit found zero posed-coordinate difference for every
merged duplicate in every retained authored pose sample across all 14 runs; its
unique-key counts also match the crate remap results. This is a sampled-pose check,
not a proof over unobserved animations.

At most 20 authored key times per clip are selected evenly over the sorted union
of that clip's sampler times, preserving endpoints. Other channels at those times
use STEP, linear/quaternion-slerp, or cubic-Hermite interpolation as authored.
Times outside an individual sampler hold its first or last value, as glTF requires.
World-space LBS evaluates `joint_world * inverse_bind * local_position`; morph
animation and authored default weights are included. Separately each named morph
target is evaluated at weight one with other targets zero. Bind means the base
geometry at zero morph weights. No synthetic pose contributes a measurement.
The importer has a separate analytic fixture for transformed multi-primitive
skinning and full-target positions.

Targets are independent 50%, 25%, 10%, and 5% when the original character has at
least 10,000 triangles. Error cap is 100 for every method. Plain uses all non-skin vertex attributes (normals, tangents, UV sets and vertex
colours where present), with unit attribute weights. Character uses
`CharacterOptions::default()`. The importance run changes only bones whose names
contain head, hand, finger or thumb to four; opaque names retain one. The current
character API cannot additionally accept the plain method's normal/UV channels;
this difference is part of the API comparison, not an equal-metric ablation.

F9 samples every triangle corner and one interior point per nondegenerate
triangle in both directions. Density is 1e-6 divided by the posed extent squared,
so ceil(area * density) gives one interior sample per nondegenerate triangle.
Reported distances are world/model-space units, deterministic sampled lower
estimates rather than continuous surface bounds. Animation p95 is the nearest-rank
95th percentile of per-pose symmetric maxima; morph p95 is defined the same way
across full-target poses. It is not a percentile of individual surface samples.
Missing pose classes are shown as unavailable in the tables. Timings exclude I/O,
welding, pose evaluation and quality measurement. After a warmup, batches are
calibrated toward 10 ms (bounded by 4,096 calls and one million source indices
per batch). The median of three batch-average per-call times is reported;
inputs/results are black-boxed. Identical index buffers and error bits are
required across repetitions. A separate timing-only pass reproduced every one
of the 135 retained quality meshes byte-for-byte (117 in the accepted corpus).
Methods run consecutively under shared load; timings are descriptive. The host
is Linux x86-64 with an AMD Ryzen 9 7945HX; Rust/Cargo 1.98.1, release profile,
default std/simd features. No performance threshold is claimed.

### Inventory and welding

P/C/I below means plain/default character/importance-four. Counts before/after
welding include all selected parts; welding alone does not remove triangles.

| Model | Parts | Vertices before/after | Triangles | Bones / shapes | Clips / sampled keyframes |
|---|---:|---:|---:|---:|---:|
| Bat | 1 | 359/318 | 376 | 16 / 0 | 1 / 20 |
| CesiumMan | 1 | 3273/3273 | 4672 | 19 / 0 | 1 / 20 |
| Corgi | 1 | 513/416 | 508 | 16 / 0 | 1 / 20 |
| Fox | 1 | 1728/434 | 576 | 24 / 0 | 3 / 58 |
| Gangnam | 79 | 100638/100637 | 194390 | 65 / 0 | 0 / 0 |
| MakeHuman | 8 | 20754/20754 | 36924 | 53 / 92 | 12 / 240 |
| MorphStressTest | 2 | 1528/1528 | 2412 | 0 / 8 | 3 / 60 |
| RiggedFigure | 1 | 370/365 | 256 | 19 / 0 | 1 / 2 |
| RobotExpressive | 7 | 2850/2850 | 1354 | 44 / 3 | 14 / 260 |
| Shark | 1 | 348/268 | 286 | 16 / 0 | 1 / 20 |
| Superhero | 3 | 8483/8483 | 14318 | 65 / 0 | 0 / 0 |
| minion-a01 | 4 | 219/219 | 240 | 5 / 0 | 1 / 20 |

### Measurements

| Model | Target | Triangles P/C/I | Bind P/C/I | Animation max P/C/I | Animation p95 P/C/I | Morph max P/C/I | Morph p95 P/C/I | ms P/C/I  Best-of method: max (plain / character) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Bat | 50% | 184/186/186 | 31.7837/21.4981/21.4981 | 31.7837/21.4981/21.4981 | 31.7837/21.4981/21.4981 | n/a/n/a/n/a | n/a/n/a/n/a | 0.073137/0.109725/0.109327  character: 21.4980745 (31.7837383 / 21.4980745) |
| Bat | 25% | 89/93/93 | 227.209/84.9344/84.9344 | 268.5/98.0996/98.0996 | 261.377/93.193/93.193 | n/a/n/a/n/a | n/a/n/a/n/a | 0.080311/0.135024/0.137619  character: 98.0995825 (268.499578 / 98.0995825) |
| Bat | 10% | 53/69/69 | 336.538/290.086/290.086 | 351.674/302.317/302.317 | 350.673/299.878/299.878 | n/a/n/a/n/a | n/a/n/a/n/a | 0.082285/0.148428/0.148428  character: 302.317443 (351.674196 / 302.317443) |
| CesiumMan | 50% | 2336/2336/2336 | 0.0066351/0.00373859/0.00373859 | 0.034836/0.012756/0.012756 | 0.0346711/0.0121999/0.0121999 | n/a/n/a/n/a | n/a/n/a/n/a | 1.4999/2.49984/2.63799  character: 0.0127559775 (0.034836046 / 0.0127559775) |
| CesiumMan | 25% | 1168/1168/1168 | 0.0213081/0.0193594/0.0193594 | 0.040504/0.0214111/0.0214111 | 0.0404751/0.0207889/0.0207889 | n/a/n/a/n/a | n/a/n/a/n/a | 1.61419/2.62483/2.74982  character: 0.021411072 (0.0405040037 / 0.021411072) |
| CesiumMan | 10% | 594/588/588 | 0.651062/0.54802/0.54802 | 0.669012/0.635762/0.635762 | 0.663454/0.634398/0.634398 | n/a/n/a/n/a | n/a/n/a/n/a | 1.87488/2.74982/2.88869  character: 0.635762155 (0.669011681 / 0.635762155) |
| Corgi | 50% | 253/254/254 | 25.0492/11.6987/11.6987 | 25.0492/11.6987/11.6987 | 25.0492/11.6987/11.6987 | n/a/n/a/n/a | n/a/n/a/n/a | 0.120325/0.185008/0.190737  character: 11.6987331 (25.0491963 / 11.6987331) |
| Corgi | 25% | 127/127/127 | 77.696/47.5539/47.5539 | 83.5937/47.5539/47.5539 | 77.696/47.5539/47.5539 | n/a/n/a/n/a | n/a/n/a/n/a | 0.132009/0.195789/0.171864  character: 47.5538856 (83.5937233 / 47.5538856) |
| Corgi | 10% | 59/59/59 | 307.225/307.225/307.225 | 324.848/324.848/324.848 | 324.328/324.328/324.328 | n/a/n/a/n/a | n/a/n/a/n/a | 0.132804/0.210248/0.194865  plain: 324.848322 (324.848322 / 324.848322) |
| Fox | 50% | 288/288/288 | 1.55404/2.93161/3.23097 | 8.98257/3.21807/3.53675 | 8.79242/3.19932/3.52944 | n/a/n/a/n/a | n/a/n/a/n/a | 0.101177/0.210839/0.20922  character: 3.21807136 (8.98256846 / 3.21807136) |
| Fox | 25% | 144/144/144 | 4.55005/11.8784/11.8784 | 10.7225/11.8784/11.8784 | 9.67541/11.8784/11.8784 | n/a/n/a/n/a | n/a/n/a/n/a | 0.106414/0.215677/0.218736  plain: 10.7224612 (10.7224612 / 11.8784184) |
| Fox | 10% | 56/56/56 | 11.8784/20.3766/20.3766 | 15.2029/20.3766/20.3766 | 13.4797/20.3766/20.3766 | n/a/n/a/n/a | n/a/n/a/n/a | 0.122688/0.261213/0.254657  plain: 15.2029058 (15.2029058 / 20.3766318) |
| Gangnam | 50% | 97194/97195/97195 | 0.000897904/0.000864407/0.000864407 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 144.814/328.57/328.803  character: 0.000864406937 (0.00089790403 / 0.000864406937) |
| Gangnam | 25% | 48595/48586/48586 | 0.0013441/0.00116821/0.0015102 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 179.416/353.115/319.618  character: 0.00116820735 (0.00134409904 / 0.00116820735) |
| Gangnam | 10% | 19422/19436/19439 | 0.00352761/0.00457864/0.00457864 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 177.676/401.965/401.748  plain: 0.00352761171 (0.00352761171 / 0.00457863984) |
| Gangnam | 5% | 9718/9719/9719 | 0.00753503/0.00571659/0.00689108 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 179.613/364.48/372.352  character: 0.00571658968 (0.0075350347 / 0.00571658968) |
| MakeHuman | 50% | 18462/18462/18461 | 0.00152945/0.00124431/0.00125861 | 0.0146956/0.0137957/0.0137982 | 0.0146622/0.0129116/0.0133996 | 0.0125779/0.00913868/0.00913868 | 0.00626234/0.00618963/0.00618963 | 11.3856/210.286/221.56  character: 0.0137956965 (0.0146955612 / 0.0137956965) |
| MakeHuman | 25% | 9231/9231/9230 | 0.0040431/0.00380298/0.00380298 | 0.0333929/0.0196981/0.0196981 | 0.0331706/0.0196942/0.0196942 | 0.0205209/0.0182385/0.0182385 | 0.00908523/0.0085604/0.00869246 | 12.5123/239.659/229.482  character: 0.0196981207 (0.0333928786 / 0.0196981207) |
| MakeHuman | 10% | 3691/3681/3675 | 0.00888565/0.00645163/0.008164 | 0.033632/0.0317572/0.0272604 | 0.0329974/0.0313259/0.026744 | 0.030342/0.0293046/0.0293046 | 0.0132621/0.0129229/0.0128263 | 14.5452/251.57/246.781  character: 0.0317571773 (0.0336319788 / 0.0317571773) |
| MakeHuman | 5% | 1844/1842/1845 | 0.0205968/0.0122735/0.0177644 | 0.0482495/0.0377995/0.037286 | 0.0447679/0.0375796/0.0362091 | 0.0340246/0.0418337/0.035226 | 0.0241946/0.0167797/0.0194061 | 14.6683/223.94/231.89  character: 0.041833694 (0.0482495194 / 0.041833694) |
| MorphStressTest | 50% | 1206/1206/1206 | 0.0102372/0.00436313/0.00436313 | 0.0102372/0.00436314/0.00436314 | 0.0102372/0.00436314/0.00436314 | 0.0102372/0.00436313/0.00436313 | 0.0102372/0.00436313/0.00436313 | 0.46872/0.784922/0.822581  character: 0.00436314241 (0.0102372343 / 0.00436314241) |
| MorphStressTest | 25% | 602/602/602 | 0.0158368/0.00955761/0.00955761 | 0.0158368/0.00955762/0.00955762 | 0.0158368/0.00955761/0.00955761 | 0.0158368/0.00955761/0.00955761 | 0.0158368/0.00955761/0.00955761 | 0.57865/0.929031/0.830719  character: 0.00955762101 (0.015836826 / 0.00955762101) |
| MorphStressTest | 10% | 240/240/240 | 0.0204124/0.0205517/0.0205517 | 0.0207857/0.0205719/0.0205719 | 0.0207857/0.0205719/0.0205719 | 0.0207857/0.0205719/0.0205719 | 0.0207857/0.0205719/0.0205719 | 0.610292/0.908324/0.911616  character: 0.020571866 (0.0207856947 / 0.020571866) |
| RiggedFigure | 50% | 128/128/128 | 0.0861329/0.0861329/0.0861329 | 0.0868967/0.0868967/0.0868967 | 0.0868967/0.0868967/0.0868967 | n/a/n/a/n/a | n/a/n/a/n/a | 0.039915/0.083858/0.08702  plain: 0.0868967037 (0.0868967037 / 0.0868967037) |
| RiggedFigure | 25% | 118/120/120 | 0.0924557/0.0861329/0.0861329 | 0.0959094/0.0868967/0.0868967 | 0.0959094/0.0868967/0.0868967 | n/a/n/a/n/a | n/a/n/a/n/a | 0.05087/0.091045/0.090487  character: 0.0868967037 (0.0959093646 / 0.0868967037) |
| RiggedFigure | 10% | 118/120/120 | 0.0924557/0.0861329/0.0861329 | 0.0959094/0.0868967/0.0868967 | 0.0959094/0.0868967/0.0868967 | n/a/n/a/n/a | n/a/n/a/n/a | 0.047413/0.080394/0.078467  character: 0.0868967037 (0.0959093646 / 0.0868967037) |
| RobotExpressive | 50% | 1354/1354/1354 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0.119988/0.641344/0.648755  plain: 1.26e-15 (1.26e-15 / 1.26e-15) |
| RobotExpressive | 25% | 1354/1354/1354 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0.130185/0.634151/0.767831  plain: 1.26e-15 (1.26e-15 / 1.26e-15) |
| RobotExpressive | 10% | 1354/1354/1354 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0.140616/0.656208/0.62496  plain: 1.26e-15 (1.26e-15 / 1.26e-15) |
| Shark | 50% | 142/142/142 | 54.7856/45.2203/45.2203 | 54.7856/46.4611/46.4611 | 54.7856/45.2203/45.2203 | n/a/n/a/n/a | n/a/n/a/n/a | 0.060251/0.095323/0.094223  character: 46.4611412 (54.785578 / 46.4611412) |
| Shark | 25% | 91/87/87 | 156.269/160.512/160.512 | 156.269/170.516/170.516 | 156.269/166.728/166.728 | n/a/n/a/n/a | n/a/n/a/n/a | 0.065427/0.104515/0.103941  plain: 156.268984 (156.268984 / 170.516165) |
| Shark | 10% | 85/87/87 | 160.512/160.512/160.512 | 170.516/170.516/170.516 | 166.728/166.728/166.728 | n/a/n/a/n/a | n/a/n/a/n/a | 0.063574/0.095607/0.101556  plain: 170.516165 (170.516165 / 170.516165) |
| Superhero | 50% | 7158/7158/7158 | 0.00417/0.00382002/0.00498465 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 7.48641/16.7108/16.5554  character: 0.00382001675 (0.00417000337 / 0.00382001675) |
| Superhero | 25% | 3579/3578/3579 | 0.0104783/0.0102537/0.0102537 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 7.85761/16.7491/17.6577  character: 0.0102537157 (0.0104783157 / 0.0102537157) |
| Superhero | 10% | 1431/1431/1431 | 0.0255354/0.0199794/0.0261847 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 7.49951/17.779/18.0049  character: 0.0199793697 (0.0255353927 / 0.0199793697) |
| Superhero | 5% | 714/714/714 | 0.0429201/0.0475139/0.0613414 | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | n/a/n/a/n/a | 7.78713/17.2419/17.113  plain: 0.0429200577 (0.0429200577 / 0.047513892) |
| minion-a01 | 50% | 120/120/120 | 0.17186/0.0405847/0.0405847 | 0.17186/0.0434788/0.0434788 | 0.17186/0.0414125/0.0414125 | n/a/n/a/n/a | n/a/n/a/n/a | 0.063041/0.052183/0.055175  character: 0.0434787694 (0.171859894 / 0.0434787694) |
| minion-a01 | 25% | 60/62/62 | 0.252923/0.252923/0.252923 | 0.344046/0.312956/0.312956 | 0.344046/0.312956/0.312956 | n/a/n/a/n/a | n/a/n/a/n/a | 0.073004/0.064454/0.055945  character: 0.312956441 (0.344045594 / 0.312956441) |
| minion-a01 | 10% | 60/62/62 | 0.252923/0.252923/0.252923 | 0.354983/0.312956/0.312956 | 0.354983/0.312956/0.312956 | n/a/n/a/n/a | n/a/n/a/n/a | 0.05985/0.053609/0.0534  character: 0.312956441 (0.354982726 / 0.312956441) |


### Diagnosis, fixes and decision

The former 32-channel character gate was an admission defect for this corpus.
Well-formed baseline probes rejected Gangnam/Superhero (65), MakeHuman (329) and
RobotExpressive (53, including its rigid head binding). The simplifier already
stores its attribute gradients dynamically. The fix adds a checked private packed
view and metered wide weight storage for character calls. It neither truncates
bones/targets nor changes the public `Attributes` constructors' 32-component
limit. The existing simplify implementation and C++ parity scripts/fixtures are
unchanged. Narrow character calls keep their previous arithmetic and accounting.
Tests cover high bone IDs without projection, 92 targets and chain reports,
exact/short work budgets, zero memory budget, empty wide validation, and continued
public-constructor rejection above 32.

The importer was also corrected for global offsets above uint16, all appearance
attributes in the plain baseline, rigid morph nodes mixed with skinning, and
holding the first value before a late-start sampler. Bat/Corgi/Shark measurements
were replaced after that correction; the old inputs, CSVs and images are retained.
All 27 simplified index buffers and all simplification input bytes are identical
to their independently timed versions. Only evaluated pose streams changed.
The analytic fixture checks a non-identity inverse bind matrix, node hierarchy,
multiple primitives, a genuinely animated rigid morph node, and all 13 supplied
appearance components, including a delayed sampler that must hold its first
value. The timing pass black-boxes inputs/results and batches
small calls; byte-identical meshes bind those timings to the retained F9 results.

Welding resolves Fox's round-one identity result: 1,728 vertices become 434 and
both methods reach 288/144/56 triangles. At 50%, character reduces animation
maximum from 8.98257 to 3.21807 at the same 288 triangles. At 25% and 10%, default
character instead regresses from 10.72246 to 11.87842 and from 15.20291 to 20.37663.
Fourfold head/hand importance does not repair those regressions. Importance scales
influence differences; a rigid, constant-weight region has no internal influence
gradient. It is not a general geometry-salience or worst-pose guarantee.
The summed surrogate and sampled worst-pose distance make different tradeoffs;
plain also retains appearance channels unavailable in the character API. No
narrow collapse-selection change was made: the original output-identity contract
is preserved.

RobotExpressive's default rows retain all 1,354 triangles. Its zeros are identity
results, not quality wins. The separate, explicitly labelled `PERMISSIVE` control
reaches 676/338/134 triangles for plain and character. This isolates the stall to
the upstream conservative topology policy on this hard-seam mesh. The primary
matrix retains `EMPTY`; permissive topology is not enabled silently to pass a
reduction target. At 338 triangles, permissive plain/character animation maxima
are 0.174144/0.150204; at 134 triangles, morph maxima are 0.697474/0.693363.
Higher importance worsens that last morph maximum to 0.861323.

Many 10% cases lose entire components, visible in the saved images. The diagnostic
error cap of 100 intentionally exposes these failures; it is not a recommended
shipping tolerance. MakeHuman shows useful large-mesh results at 50% and 25%, but
at 5% character's morph maximum increases from plain's 0.034025 to 0.041834 while
its animation maximum decreases from 0.048250 to 0.037799 (1,844/1,842 triangles).
These are distinct tradeoffs, not a universal character-quality win.

### Pose-metric experiments (Fox)

Extra real-pose full-position channels, repeated bind-position channels as a
same-width control, and skin weight 0.25 were tested first. The extra channels
made no material improvement at 25%/10%; the quarter skin weight still regressed
against plain. Those variants were removed from the tracked harness. Their source,
compiled pilot and raw CSV remain external receipts.

A separate single-thread scratch copy then added area-weighted plane quadrics for
every actual clip sample, evaluated at the retained vertex's position in each
pose. The added cost is the maximum normalized plane energy over those poses.
Accepted wedge collapses merge the corresponding pose quadrics; the original
bind-space topology/seam/flip rules remain intact. This supplements the plain
normal/UV metric, not the bone-channel metric. A bind-only plane control uses the
same code and pose count. All inputs, pose streams and weld numbering were checked
identical to the main Fox case. This is a geometric pose-quadric prototype, distinct
from the discarded pose-position attribute experiment.

| Target | Triangles (all) | Animation max plain | Default character | Pose planes | Bind-plane control | Prototype ms pose/bind |
|---|---:|---:|---:|---:|---:|---:|
| 50% | 288 | 8.98257 | 3.21807 | 3.69147 | 5.50043 | 2.775/2.480 |
| 25% | 144 | 10.72246 | 11.87842 | 6.42663 | 10.72246 | 2.291/2.311 |
| 10% | 56 | 15.20291 | 20.37663 | 16.74491 | 18.03841 | 2.230/2.345 |

The pose planes help at 25%, including against the bind-only control, but lose to
default character at 50% and to plain at 10%. They are not retained as a production
backend. The scratch probe has process-scoped state, uses f64 planes and locks,
is bounded to 4,096 vertices/256 poses, and is not workspace-metered or qualified
for no_std, parallel use or larger characters. Its timings include setup and are
not a production performance comparison. No claim of pose-quadric superiority is
made. The source and experiment receipts remain external for reproduction.

### Images and qualification boundary

Five CPU front-view diagnostics at 10% are saved as Corgi.png, CesiumMan.png,
Bat.png, minion-a01.png and Shark.png, with a contact sheet and selection.json.
Selection ranks characters by the largest measured 10% deviation divided by the
same worst pose's extent. Every panel uses that same pose and orthographic framing;
all three original/plain/character meshes are shown. These are shaded geometry
renders with painter-sorted triangles, not textured GPU captures.

The bounded admission fix is validated on real rigs and morph sets. Quality remains
asset-, topology- and ratio-dependent, with explicit regressions and target misses.
Two large humanoids have no embedded clips; they establish bind-only behavior.
Neither identity results nor these CPU images establish production animation or
rendering acceptance. Use real posed quality reports to accept individual levels.

Earlier unsuccessful attempts remain in the exit ledger. Exit 75 was an admission
quota refusal, followed by the successful batched all-feature run. Exit 1 covers
the corrected uint16 import overflow, NumPy scalar JSON export, and a comparison
against older metadata lacking the optional rigid-node field. The two exit-101
scratch prototype builds failed on incomplete copied documentation and then a
module/artifact-resolution problem; the independently named complete scratch
package subsequently built and ran successfully. These do not erase failed
receipts or establish any missing final gate.

The batched final gates all exited zero: formatting, all-target/all-feature
clippy with warnings denied, default/no-default tests, all-feature doctests,
wasm32 no_std, pinned reference, public-text guard, and importer tests. The exact
C++/native Rust/wasm run checked 279 cases with zero mismatches. Its complete
artifact directory is retained separately before the earlier queued driver can
write another run. The earlier all-feature fixed-library test run also exited
zero. Three previously queued jobs are still pending; cleanup waits for them.

### Exit receipts

gate-exits.tsv

```text
all-feature-tests	0
fmt	0
release-evaluator-build	0
all-feature-final-admission	75
batched-import-attempt1	1
batched-import	0
all-feature-tests-fixed	0
release-evaluator-fixed	0
fox-mechanism-pilot	0
quadric-snapshot-build-attempt1	101
final-comparison	0
render-attempt1	1
final-method-status-audit	0
render-fixed	0
quadric-snapshot-build-attempt2	101
batched-timing-and-index-identity	0
pose-quadric-probe	0
robot-topology-control	0
front-render	0
front-render-verified	0
clamp-importer-tests	0
sampler-clamp-correction	1
clamp-importer-tests	0
sampler-clamp-correction-fixed	0
clamped-render	0
final-fmt	0
final-public-text	0
weld-pose-audit	0
batched-final-fmt	0
batched-final-clippy	0
batched-final-default-tests	0
batched-final-no-default-tests	0
batched-final-doctests	0
batched-final-wasm	0
batched-final-reference	0
batched-final-exact	0
batched-final-public-text	0
batched-final-importer-tests	0
```
measurement-exits.tsv

```text
Bat	0
CesiumMan	0
Corgi	0
Fox	0
MorphPrimitivesTest	0
MorphStressTest	0
RiggedFigure	0
RiggedSimple	0
RobotExpressive	0
Shark	0
minion-a01	0
```
final-measurement-exits.tsv

```text
Bat	0
CesiumMan	0
Corgi	0
Fox	0
Gangnam	0
MakeHuman	0
MorphPrimitivesTest	0
MorphStressTest	0
RiggedFigure	0
RiggedSimple	0
RobotExpressive	0
Shark	0
Superhero	0
minion-a01	0
```

## Best-of selection

Reproduce with `character_eval INPUT.eval.bin --best-of` after building the
release example. This reuses the 12 accepted corpus inputs, byte-identical to
round two, with the same weld, plain normal/UV/etc. attributes, unit character
weights, independent targets, authored animation and full-target samples.
Pre-evaluated samples enter through `CharacterInputs::sample_poses`; full blend
poses are also measured through the existing blend inputs. The single density
uses the largest supplied pose extent; as before it produces one interior sample
per nondegenerate triangle plus all corners. Candidate scores reproduce the
retained round-two maxima within its CSV precision. Every winning index buffer
is byte-identical to the corresponding retained plain/character buffer.

The first MakeHuman attempt returned `LimitExceeded` under the default whole-call
budget (exit 1, retained). The evaluation harness now explicitly grants 2^42
aggregate work visits, keeping the default 1 GiB byte cap and every quality
sampling setting. The first five models also completed under the default budget;
their accepted results were retained. Library defaults and gates are unchanged.

The added column gives winner, selected maximum, and both newly measured
plain/character maxima. Selection compares unrounded f64 scores, then fewer
triangles, then plain. Tests separately verify successive levels use the winner,
report against the original source, and preserve flags and rounded ratio targets.
These independent rows do not prove superiority to a separately evolving plain
chain. The guarantee covers only the supplied F9 samples, not continuous surfaces
or unobserved poses. Default character behavior remains unchanged.

All 39 rows satisfy selected max <= plain max and <= character max; 28 strictly improve plain and 11 tie it. Winners: 28 character, 11 plain. Fox 25% and 10% now select plain. The raw full-precision audit checks every inequality.

Timing below includes preparation, simplification, allocation and all F9
reports, excludes I/O, welding and pose construction, and measures one full
character call followed by one best-of call per target. No warmup, repetition
median or controlled speed claim is implied. The earlier ms P/C/I column remains
its original simplification-only timing; it cannot be subtracted from these
whole-call measurements. Best-of measures both candidates and retains the winning
reports without measuring the winner again.

| Model | Total character ms | Total best-of ms | Extra ms | Ratio |
|---|---:|---:|---:|---:|
| Bat | 68.642 | 146.242 | 77.600 | 2.131x |
| CesiumMan | 1278.241 | 2698.992 | 1420.751 | 2.111x |
| Corgi | 117.274 | 232.476 | 115.202 | 1.982x |
| Fox | 338.574 | 721.321 | 382.747 | 2.130x |
| Gangnam | 5836.372 | 10992.453 | 5156.081 | 1.883x |
| MakeHuman | 406184.101 | 848084.061 | 441899.960 | 2.088x |
| MorphStressTest | 3182.915 | 6303.189 | 3120.274 | 1.980x |
| RiggedFigure | 11.245 | 22.370 | 11.125 | 1.989x |
| RobotExpressive | 8891.155 | 18290.878 | 9399.724 | 2.057x |
| Shark | 73.069 | 138.086 | 65.017 | 1.890x |
| Superhero | 376.942 | 685.401 | 308.459 | 1.818x |
| minion-a01 | 48.599 | 90.914 | 42.314 | 1.871x |

Across all targets: 426407.128 ms character, 888406.382 ms best-of, +461999.255 ms (2.083x total; median per-row ratio 2.014x). These are descriptive shared-host observations.

Validation for best-of: formatting, all-target/all-feature clippy with warnings
denied, default/all-feature/no-default-feature tests and doctests, wasm32
no-default-feature build, pinned C++ reference check, and public-text guard all
exited 0. The unchanged exact differential gate checked 279 cases against C++
and wasm with zero mismatches. Final character tests also passed with and without
default features. The updated evaluation example passed clippy and release build;
all 12 final model evaluations and the 39-row score/index audit exited 0.
The initial default-budget measurement exit 1 remains retained, as described above.
No parity fixtures or existing algorithm outputs were changed. Raw CSVs, input
streams, binary/source identities and gate receipts remain external. The dedicated
fresh build target was removed after all jobs completed. No Git state was changed.
