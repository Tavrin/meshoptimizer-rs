# Second index candidate, registered before measurement

Campaign 1 rejected the index candidate: 1.333x baseline instructions on
real meshes; vertex remapping passed at 0.779x. Restore the rejected index
function, then replace its per-element sentinel accumulation with the
baseline's vectorizable sentinel reduction and four checked gathers per
iteration. No limits, errors, output semantics, or gate changes.

Campaign 2 remeasures both functions, all eight inputs and all four arms.
Apply the same <=0.95 real-mesh GM and <=1.01 worst-case gate. A failing
function is restored and reported NO-GAIN. Preserve campaign 1, including
its clippy failure (the fallible reserve/resize spelling was linted as slow
zero initialization); use resize_with with the same zero values.
