# Attribute simplification and overdraw attribution

Base is 8740a55 (0.2.13); comparison is the frozen peers harness, packed XYZ,
four synthetic attributes and its original input order. The peer totals below
are median-of-three `instructions:u` per call from `parity/peers/counts.jsonl`,
not sampled estimates. C++ is meshopt 0.7.0 / meshoptimizer 1.3; X is optimesh
1.1.0. No wall-time claim. All three input hashes and outputs are in the peer
receipts. Ratios and counts are not evidence of quality equivalence beyond the
peer report's byte comparisons.

| Real case / operation | base | C++ | X | C++/base | X/base |
|---|---:|---:|---:|---:|---:|
| 214e / attributes | 7245934164 | 4926656932 | 5677184641 | .680 | .784 |
| a082 / attributes | 10500555913 | 7595291764 | 9086074882 | .723 | .865 |
| b6fd / attributes | 2622682195 | 1892982009 | 2268784880 | .722 | .865 |
| 214e / simplify | 3840016644 | 2813433357 | 3510227120 | .733 | .914 |
| a082 / simplify | 6458992632 | 5024122900 | 6399032761 | .778 | .991 |
| b6fd / simplify | 1612173318 | 1250240984 | 1597148262 | .776 | .991 |
| 214e / overdraw | 1458435688 | 992615861 | 1227652034 | .681 | .842 |
| a082 / overdraw | 1546984769 | 1076432085 | 1324635884 | .696 | .856 |
| b6fd / overdraw | 386758335 | 269119661 | 331167755 | .696 | .856 |

## Sampled function mapping

A fresh admitted instruction-event profile was captured once for every cell
above and every arm, using the peer resident loop and 1,000,000 instruction
sampling period. Per-case records, symbol and source-line reports and binary
hashes are retained in scratch. Sample shares are approximate and inlining
makes them **not** additive function-to-function costs. C++ profiling was built
with debug info, which changes its build-script assertion configuration: its
214e attributes sample was approximately 5.067 billion instructions versus
4.927 billion in the frozen release measurement. Use it for location, not for a
new C++ ratio. The other arms' sampled totals agree closely with frozen totals.

| Mapped phase | Ours | C++ 1.3 | X 1.1.0 | Diagnostic samples |
|---|---|---|---|---|
| Topology, normalization, control | `run_state`, `allocating`; `src/simplify.rs:2409` | `meshopt_simplifyEdge`; `simplifier.cpp:2340` | `simplify_edge`; `simplifier.rs:2486` | attributes: ours run_state 26–35%, X entire inlined simplify_edge 80–83%; C++ entry 35–44% |
| Positional and attribute quadrics | `State::quadrics`, `attribute_quadric`; `src/simplify.rs:1680`, `:1741` | `fillAttributeQuadrics`; `simplifier.cpp:1208` | `fill_attribute_quadrics`; `simplifier.rs:1249` (inlined) | attributes: ours quadrics 25–31% plus attribute_quadric 10–16%; C++ attribute phase 27–36% |
| Adjacency | `State::adjacency`; `src/simplify.rs:1283` | `updateEdgeAdjacency`; `simplifier.cpp:52` | `EdgeAdjacency::update`; `simplifier.rs:126` | X attributes 8–11%; separate phase in each |
| Overdraw cluster/cache, geometry, sort | `optimize_overdraw` and `kernel`; `src/overdraw.rs:18`, `:324` | `meshopt_optimizeOverdraw`, `calculateSortOrderRadix`; `overdrawoptimizer.cpp` | `optimize_overdraw`; `overdrawoptimizer.rs:332` | respective entry symbols encompass 95–97%, 88–89%, 96%; C++ sort adds 7%; inlining prevents phase-level cost comparisons |

Source-line sampling attributes 3.71–5.45% of attribute instructions and
11.06–13.99% of overdraw instructions to the inlined `f32::is_finite`
implementation; mapping is statistical and can include inlining artifacts.
Quadric addition at `src/simplify.rs:653` accounts for 4.96–10.10% in the
attribute samples. These samples identify hot checks and additions, not a
safe or quantified way to remove them.

## Ranked hypotheses and constraints

1. Quadric construction and normalization dominate attributes. Our quadric
   additions and finite sweeps (`src/simplify.rs:653`, `:1857`) are visible in
   the line samples; the peer constructs and accumulates attribute quadrics in
   `simplifier.rs:1249` with direct hardware `.sqrt()` in its geometry helpers.
   Our `attribute_quadric` uses a hash-based software-root cache at
   `src/simplify.rs:1686`. Root backend and check costs are hypotheses, not
   isolated savings. Exact rounding, error precedence and charges constrain
   replacement; 0212's older ablations are not quantitative proof for this
   revision.
2. Both peer and ours scan clusters and positions for overdraw; peer direct
   position reads and geometry (`overdrawoptimizer.rs:178`, `:220`) lack our
   checked numerical and accounting paths (`src/overdraw.rs:324`, `:420`).
   The sampled source lines also cover cache updates (`src/overdraw.rs:139`,
   peer `overdrawoptimizer.rs:29`). No individual structural excess is
   isolated against X. The 0212 packed rewrite was rejected by the >1.01
   per-case gate, so it is not evidence for another overdraw change.
3. Plain simplify is close to X on two meshes; changes shared with attributes
   must pass every simplify case. Bounds, accounting and numerical checks are
   observable under limits and cannot simply be removed.

optimesh 1.1.0 declares MIT, and its LICENSE credits Arseny Kapoulkine and Marco
Gomez. Source was inspected as data; no code was copied. C++ and X symbol
locations refer to their pinned registry crates. Profiles locate work; they do
not establish the causal saving of a proposed edit.
