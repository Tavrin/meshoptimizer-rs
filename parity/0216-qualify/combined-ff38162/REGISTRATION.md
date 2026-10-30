# Combined qualification at ff38162

Base: v0.2.15, 1ecf69abe9f92c17e13dc38fa97be659aee44582. Candidate starts
at ff38162, after all nine lane merges. Existing evidence remains intact.
Only this subdirectory receives new receipts until the release report/table
are written. Library source remains frozen unless a whole lane needs reverting.

The runner calls the exact lane drivers directly; scripts combining proof,
fuzzing, source formatting, or prior-receipt replacement are deliberately not
executed. Fixed fixture matrices embedded in exact/differential drivers remain
part of their normal invocation; no fuzz mode, fuzz executable, feedback-driven
campaign, or open-ended seed loop is run.

Both all-features and no-default scalar configurations are tested. The combined
bounds/geometry consumer additionally enables clusterlod in the scalar build,
as those lane drivers require it. The parallel scalar driver additionally
enables parallel and parity-internals. The lane's unchanged parallel corpus
driver exercises 1/2/4/8/32 workers; a retained one-line worker-list adapter
adds the required 1/4/16 matrix. The decoder's forced-level no-std differential
enables simd explicitly, as in its lane; its exact no-default consumer disables
simd. Spatial forces scalar/SSE2/SSSE3/SSE4.1 using its unchanged driver.

CPU-only admission wraps every stage, with declaration 4. Builds pause below
100 GiB available. The fresh target holds transient manifests, archived base,
compiled programs and input links, and is removed after retained bindings and
receipts are verified. Every finite job is sized at no more than 7200 CPU
seconds; tighter process estimates and measured CPU totals are journaled.
Sizing uses the completed lane receipts (bounds exact about 7 s, geometry
about 60 s; simplify proof less than 600 s including builds; remap proof about
103 s including builds; spatial's finite 156-cell matrix per forced level).
Core exact/build/test work is grouped in a finite job estimated below 6000 CPU
seconds from those receipts, including a compilation and production-corpus
margin. The peer builds and C++ differential matrices form another job bounded
at 6000 CPU seconds (the retained decoder differential takes about 90-120 CPU
seconds per mode; spatial differential about 76). Measurement jobs are grouped
as the five synthetic meshes, then one production mesh per job, each estimated
below 6000 CPU seconds from the retained instruction counts and lane timings,
including setup and warmup margin. Cost/wall disclosure is a separate finite
job. These groupings were fixed before any measurement sample. No queue timeout.

One instructions:u campaign, three interleaved repetitions with the middle
arm order reversed. Five frozen corpus and three real inputs are byte-bound
to the 0.2.15 campaign manifest. Seven arms: base and train, each scalar and
default; C++ 1.3 scalar and default; optimesh 1.1.0 scalar. Allocation and
output destruction are counted; fixture loading, remap setup, warmup and
serialization are outside the region. Release thin LTO, one codegen unit,
no native-CPU flags, same canonical package/source/consumer path for all arms.
Rust workspaces use unlimited logical work/storage limits in every arm; this
also admits the parallel API's documented execution path. Physical admission
and its memory cap still apply.
Default and scalar comparisons against base are separate. Any per-case median
ratio above 1.01 is a failure requiring lane attribution and a whole-lane
revert plus reproof; no geometric mean may conceal such a failure.

Bounds use the exact retained baseline meshlet topology from the bounds lane,
verified against its manifest before use. Every arm localizes the same groups
before counters. The release table includes standalone cluster/meshlet bounds
in addition to the inherited nineteen-operation release matrix. optimesh has
no cluster-LOD implementation. New repair and streaming APIs have no base or
C++ counterparts; inherited functions are checked exactly, new API contracts
and the streaming C++ wire differential are checked by their lane drivers.

Cost disclosure: corpus only, default train, median of three instructions:u.
Repair uses zero-tolerance positions-only weld, orient, and max-16-edge fill,
against sanitize EMPTY on the same raw inputs. Streaming uses 4096-byte chunks,
owned u32 output, and version-1 index sequences encoded before counting;
one-shot uses owned packed 32-bit output. Setup verifies equal decoded bytes.
These are distinct calls, not interchangeable operations.

Parallel wall disclosure: the same default parallel consumer with one versus
four workers, three real meshes, median of five interleaved AB/BA samples.
Distinct physical cores are selected from the allowed affinity mask. One worker
uses the first core; four workers use four. Exact serialized callbacks must match.
Wall measurements do not replace instruction regression gates.

Arm64 execution belongs to CI under the decoder's documented rule: Rust NEON
must equal Rust scalar and C++ scalar; C++ NEON divergences are disclosure
only. This x86 qualification does not claim local arm64 execution.
