# Targeted differential sizing

The numeric-extremes draft defines eight six-vertex meshes, three attribute
widths, and the empty mask plus the five single masks and all ten pairs:
384 input cells per feature mode. Each cell calls seven API/buffer variants.
The unlimited usages determine the size of each per-cell work-limit sweep;
there is no honest fixed observation total until those usages are measured.

A pilot is one cell and seven variants per feature mode. The pilot compares
an unlimited call and each limit through its maximum observed usage plus one,
with two consecutive calls per workspace. It is a sizing and harness check,
not the target proof: its input is ordinary, and a repeated call after failure
is not evidence of an independently successful warm-up.

The completed warm draft prepares a small valid predecessor at the tested
limit, compares its result, and reports failed primers separately; it does not
establish successful warming when that predecessor fails. Changing workspace
limits clears retained allocations. A traversal boundary must be measured at each
actual nearest traversal entry: it is consumed work at entry plus twice the
node-slice length, not just the node-slice length or the preceding KD setup
charge. Both requirements must be reflected in the finished harness.

The cluster-LOD draft sizes one 4 by 4 mesh x two attribute widths x four
lock patterns x 4096 boolean configurations, or 32768 cells per feature
mode. Its limits are determined by a downward failure search and a stride of
64. Both the initial and separately primed grids were run. Some priming calls
failed, so successful warming at every tested limit remains unproven.
