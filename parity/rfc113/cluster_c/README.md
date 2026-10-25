# Cluster lane C proof adapters

The registration and receipts are retained under
`/mnt/linux-extra/moss-scratch/meshopt-cluster-c/`. `REGISTRATION.md` precedes
all verdict samples. `campaign.py` imports the unmodified statistics, physical
core selection and admission functions from `../timing.py`.

`bridge.cpp` preserves the historical per-mesh flattening and hierarchy. Its
additional arguments select S1's protect mask, padded attribute weights, vertex
cap and dilation. S2 retains its original config and boundary locks. The Rust
arm calls `build_dag_into`, with ceiling-sized caller buffers allocated before
requests and reused between requests; input parsing, input copies, a fresh
Workspace, construction and MCLD serialization remain inside its timer. Both
arms return the exact same byte stream and an internal elapsed-nanosecond
suffix. No process, pipe or Python time contributes to ratios. The library
remains format-agnostic.

`prepare` freezes the original input/config identities. `preflight` checks both
profiles against scalar C++ and all 41 S1 `.clod` files, without issuing timing
requests. `instructions` runs untimed requests under `perf stat` and retains
attempted instruction-sampled call graphs; postprocessing failures and raw captures are retained. `case PROFILE INDEX` runs one registered case:
5–20 adaptive interleaved pairs on one pinned physical core, then 30 fresh pairs
only if the 20-pair interval is unresolved. Aggregate ratios use stage-1 medians.
There is no load threshold, favorable-pair selection or verdict-driven rerun.

The scratch controller waits for bet 0's completion receipt before any case,
yields to train jobs between cases, releases heavy admission between cases, and
stops after the first mesh in both profiles for the milestone disposition.
