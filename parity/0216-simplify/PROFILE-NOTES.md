Update-specific scalar experiment: the original `simplify_extra.rs:1098–1121`
post-solve bounds pass repeatedly calls the mutable position view's checked reader
and work counter. It is included in solve/finalize above. For dense packed views,
try a direct ordered slice scan with the existing Work::scan fallback semantics;
sparse and strided views retain the original loop. No bounds reduction reorder.

Additional disassembly resolution: in real-214e sloppy, the large leaf location
`simplify_extra.rs:0` at baseline instruction `0x7a4f1` is the first multiply in
the scalar cell-quadric error evaluation, immediately after loading a coefficient.
The surrounding annotated block corresponds to original `:1768–1774`, not an
unknown algorithm phase. Other line-zero samples remain unresolved in the table;
no periods have been reassigned from a single-address observation. Eval uses
ordered scalar products and adds; accumulation's existing packed adds do not
imply a packed eval. A packed eval would require gathering/permuting three
coefficient/position lanes and extracting terms for the same ordered sum.
These instruction samples alone do not establish that arithmetic rather than
coefficient access is the limiting cost. No explicit eval backend is claimed
or shipped on that evidence.

For update, C++ `finalizeVertices` at `simplifier.cpp:611` receives the offset
and scale already produced by rescalePositions. Rust instead recomputes bounds
at original `simplify_extra.rs:1098–1121`. Reusing the already ordered bounds
is the second scalar experiment targeting this attributable extra pass.
