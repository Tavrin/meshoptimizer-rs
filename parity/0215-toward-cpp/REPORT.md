# Toward C++: 0.2.15 instruction and contract proof

## Check lane: campaign 2 (current result)

A second agent checked the campaign-1 commit and then optimized further. It
found no case where the batched cache tally, the guarded native meshlet
roots, the exact bounded scores or the metered cluster-LOD phases can change
output, errors, their order, `Usage` or partial writes compared with
v0.2.14. The checks were: the tally adds exactly what the per-entry charges
add, because the covered loop has no early exit; the root guard admits only
+0, subnormals and finite positives; the exact-score selection and the
removed pruning keep strict-less and listed tie order; the four phase bounds
match their charges (`vertex_count + 2n`, `n`,
`p.len() * (ac + 1) + forward + back`, `n`).

Attribution for the remaining meshlet gap (perf record, instructions:u, on
real-b6fd): `nearest_inner::<false, true>` took 66% of `meshlets`. Its
recursion passed nine arguments, five of them on the stack and reloaded
per call, and each leaf item paid two index checks. C++ `kdtreeNearest`
takes 64% of a smaller total. The fix is a covered-search record
(`src/meshlet.rs`, `Covered::nearest`; proof in PROOFS.md "Covered KD search
record"). Cluster LOD benefits too, because its meshlet builds use the same
search.

Vertex decode was attributed but not changed. Of 126.0M instructions per
call on real-b6fd, `bytes_kernel` takes 51% and `deltas_kernel` 41%; C++
1.3 spends 94.6M of its 100.1M in one fused `decodeVertexBlockSimd`. The
gap is inside the x86 SIMD kernels. The spec makes SIMD a non-goal and
forbids new unsafe code. The scalar remainder (block driver 2.8%, output
memset 1.1%) cannot reach the 0.97 gate, so the verdict stays NO-GAIN.

This second campaign follows the same registration with the same arms,
inputs and runner. It is the second and last campaign allowed. The
coordinator authorized edits after campaign 1. No library code was edited
after campaign 2's results.

| Function | Candidate/base | Candidate/optimesh | Base/C++ | Candidate/C++ | Verdict | Campaign 1 (c/base, c/opt, c/C++) |
|---|---:|---:|---:|---:|---|---|
| cache | 0.9422 | 0.9062 | 1.3130 | 1.2372 | SHIPS | 0.9422, 0.9062, 1.2372 |
| vertex-decode | 1.0000 | 0.7090 | 1.2565 | 1.2565 | NO-GAIN | 1.0000, 0.7090, 1.2565 |
| meshlets | 0.8261 | 0.7836 | 1.2120 | 1.0012 | SHIPS | 0.9611, 0.9116, 1.1648 |
| meshlets-flex | 0.8263 | 0.7840 | 1.2125 | 1.0018 | SHIPS | 0.9612, 0.9121, 1.1654 |
| cluster-lod | 0.9089 | N/A | 1.3080 | 1.1889 | SHIPS | 0.9476, N/A, 1.2395 |

All 456 samples were accepted. Maximum candidate/base over all cases was
1.0000223. No case regressed (`regressions` is empty). The base, candidate
and C++ output hashes are equal in every cell. On real-214e, `meshlets` now
uses fewer instructions than C++ (13,272,832,724 against 13,286,174,217).
Corpus meshes gain less (meshlets 0.975 against base) because they rarely
fall back to the KD search. Medians per case are in `campaign-2/results.json`.

Proof, run in a fresh directory against the final source (every step exited 0,
`campaign-2/proof-exits.tsv`): test-all, test-no-default, clippy, fmt,
boundary and public-text passed. Exact cache/decode compared 992,130 cases per
feature mode with 0 differences. Geometry exact compared 996,773 + 1,020,255
per mode with 0 differences. The meshlet/LOD exact driver produced 154,590
identical records per mode. The C++ cache/decode check passed 1,932 cases;
the reference check passed; the C++ meshlet/LOD differential passed 1,280 cases
(400/400/400/80). Fuzz ran 2,270,000 cases in 300.29 child CPU seconds.

Not verified: other architectures, wasm and no_std performance; the native-root
exhaustive experiment was not rerun. The record covers this host only.

## Campaign 1

Baseline: v0.2.14. One registered four-arm instruction-only campaign,
median of three per cell, five frozen corpus meshes and three real meshes.
All accepted counters ran 100%; LC_ALL=C. No wall-time claim.
The completed campaign, result evaluation and report export exited 0:
456 accepted samples, 40 function/input cells, no second campaign or
post-campaign library edit.

## Attribution and techniques

The retained 0214 contract-cost experiment attributes 39.5% of the cache
gap to work accounting; for meshlets W/R explain 55.5%/29.8%, and for
cluster LOD 41.8%/25.7%. These historical interventions are attribution,
not savings measured for this candidate. Optimesh 1.1.0 cache, clusterizer
and vertex-codec sources were read as MIT-licensed data; none was copied.

The candidate batches infallible covered cache score visits, uses guarded
native std roots in greedy/flex meshlets and evaluates exact scores instead
of root brackets, and meters four private cluster-LOD simplifier phases
under checked work bounds. PROOFS.md records file:line derivations and
REVIEW.md the completing agent's read-only adversarial pass. No finite
check, public API, unsafe code or SIMD implementation was added or removed.
The inherited stride-12 SIMD specialization was removed as out of scope;
vertex decoding has no retained source change.

Shared-helper changes: only the private cluster-LOD snapshot's
`support/workspace.rs` (Meter/Tally/dispatch) and `support/simplify.rs`
(adjacency/pick/update/filter, local table/vector borrows). Main shared
workspace/math/processing and other function families are unchanged.

## Three-real-mesh geometric means

| Function | Candidate/base | Candidate/optimesh | Base/C++ | Candidate/C++ | Verdict |
|---|---:|---:|---:|---:|---|
| cache | 0.9422 | 0.9062 | 1.3130 | 1.2372 | SHIPS |
| vertex-decode | 1.0000 | 0.7090 | 1.2565 | 1.2565 | NO-GAIN |
| meshlets | 0.9611 | 0.9116 | 1.2120 | 1.1648 | IMPROVES |
| meshlets-flex | 0.9612 | 0.9121 | 1.2125 | 1.1654 | IMPROVES |
| cluster-lod | 0.9476 | N/A | 1.3080 | 1.2395 | SHIPS |

The context-specific C++ substitution in SPEC-0215-S5 and the inherited
registration is applied: SHIPS requires candidate/base <=0.97 and
base/C++ minus candidate/C++ >=0.05. IMPROVES requires only the former.
The later generic optimesh formula conflicts with that explicit
substitution; candidate/optimesh is still reported transparently.
Every case must also be <=1.01 versus base.

## Per-case medians of instructions per call

| Case | Op | Base | Candidate | C++ | Optimesh | Candidate/base |
|---|---|---:|---:|---:|---:|---:|
| tiny-smooth | cache | 30947 | 29335 | 23697 | 33385 | 0.947910 |
| medium-smooth | cache | 11244929 | 10619956 | 8526949 | 11504267 | 0.944422 |
| medium-seams | cache | 9860490 | 9294217 | 7418200 | 10141679 | 0.942572 |
| medium-sparse | cache | 11309326 | 10684346 | 8597908 | 11551829 | 0.944738 |
| million-smooth | cache | 1395907990 | 1318537000 | 1058999256 | 1425848655 | 0.944573 |
| real-214e | cache | 2361192794 | 2225879646 | 1803471038 | 2453329083 | 0.942693 |
| real-a082 | cache | 2734107723 | 2575542812 | 2079324204 | 2844728348 | 0.942005 |
| real-b6fd | cache | 683531383 | 643890136 | 519827889 | 710725686 | 0.942005 |
| tiny-smooth | vertex-decode | 4400 | 4400 | 2830 | 4300 | 1.000000 |
| medium-smooth | vertex-decode | 230318 | 230327 | 173252 | 330424 | 1.000039 |
| medium-seams | vertex-decode | 268812 | 268803 | 200138 | 386211 | 0.999967 |
| medium-sparse | vertex-decode | 320612 | 320612 | 218977 | 480059 | 1.000000 |
| million-smooth | vertex-decode | 23503207 | 23503162 | 17353148 | 34124938 | 0.999998 |
| real-214e | vertex-decode | 461996871 | 461996878 | 365792941 | 651683881 | 1.000000 |
| real-a082 | vertex-decode | 513099156 | 513099162 | 410937051 | 724683862 | 1.000000 |
| real-b6fd | vertex-decode | 125963212 | 125963194 | 100124053 | 177399734 | 1.000000 |
| tiny-smooth | meshlets | 61487 | 59858 | 44903 | 62039 | 0.973507 |
| medium-smooth | meshlets | 27835421 | 27032003 | 26954148 | 36763957 | 0.971137 |
| medium-seams | meshlets | 21330276 | 20771214 | 20692660 | 29322545 | 0.973790 |
| medium-sparse | meshlets | 27947920 | 27144480 | 26982002 | 36779119 | 0.971252 |
| million-smooth | meshlets | 4053995498 | 3929245319 | 3870400654 | 5200575788 | 0.969228 |
| real-214e | meshlets | 16170123863 | 15537773616 | 13286174106 | 16849217177 | 0.960894 |
| real-a082 | meshlets | 17228134919 | 16564227669 | 14278322598 | 18265513099 | 0.961464 |
| real-b6fd | meshlets | 4155798982 | 3993052877 | 3427833996 | 4407790434 | 0.960839 |
| tiny-smooth | meshlets-flex | 61516 | 59888 | 44932 | 62083 | 0.973537 |
| medium-smooth | meshlets-flex | 27835565 | 27032145 | 26953633 | 36763465 | 0.971137 |
| medium-seams | meshlets-flex | 21330621 | 20771590 | 20692343 | 29322329 | 0.973792 |
| medium-sparse | meshlets-flex | 27947656 | 27144251 | 26981449 | 36778598 | 0.971253 |
| million-smooth | meshlets-flex | 4063225002 | 3937879715 | 3875326619 | 5206483914 | 0.969151 |
| real-214e | meshlets-flex | 16176050323 | 15545674769 | 13285980910 | 16849020937 | 0.961030 |
| real-a082 | meshlets-flex | 17234063126 | 16572127010 | 14278100359 | 18265287696 | 0.961591 |
| real-b6fd | meshlets-flex | 4157279422 | 3995026588 | 3427779146 | 4407735769 | 0.960971 |
| tiny-smooth | cluster-lod | 100253 | 98639 | 75156 | N/A | 0.983900 |
| medium-smooth | cluster-lod | 116353476 | 112958353 | 99764731 | N/A | 0.970821 |
| medium-seams | cluster-lod | 113466844 | 109380568 | 94520230 | N/A | 0.963987 |
| medium-sparse | cluster-lod | 118167688 | 114772618 | 100754647 | N/A | 0.971269 |
| million-smooth | cluster-lod | 15275048721 | 14831456025 | 13010266708 | N/A | 0.970960 |
| real-214e | cluster-lod | 86126302250 | 81654100300 | 65734166483 | N/A | 0.948074 |
| real-a082 | cluster-lod | 97163229371 | 92054694669 | 74324039681 | N/A | 0.947423 |
| real-b6fd | cluster-lod | 24065480661 | 22797014107 | 18418151386 | N/A | 0.947291 |

Maximum candidate/base: 1.00003908. No case exceeds 1.01.
All base/candidate and candidate/C++ canonical hashes match in every
cell; the complete per-arm hashes and optimesh outputs are retained
in results.json and counts.jsonl.

## Proof receipts and boundaries

All final proof steps below exited 0. The first proof attempt failed
with exit 101 because the added geometry driver requested unsupported
LOD defaults of size 2. A subsequent supervisor exited 2 before any
proof step because its binding helper had been placed at the wrong
location. The next attempt was deliberately terminated after the
source binding changed during a proof-script restart fix. These
failed/interrupted logs remain in scratch; the corrected supervisor
was rerun in a fresh proof directory. Library code was unchanged
through these harness repairs. Its 27 proof steps completed with
exit 0, then the combined supervisor exited 1 when the peer build
snapshot omitted the documentation included by lib.rs (Cargo exit
101). The snapshot helper now includes and hashes those documents;
the failed build log remains retained and no measurement sample
had been taken. Proof-source-binding.json retains the helper version
executed during proof; the final build helper differs only in that
snapshot-document repair. The campaign is resumed separately.

| Command / step | Exit |
|---|---:|
| test-all | 0 |
| test-no-default | 0 |
| clippy | 0 |
| fmt | 0 |
| boundary | 0 |
| public-text | 0 |
| exact-build-std | 0 |
| exact-std | 0 |
| geometry-exact-std | 0 |
| exact-build-no-std | 0 |
| exact-no-std | 0 |
| geometry-exact-no-std | 0 |
| meshlets-build-baseline-std | 0 |
| meshlets-run-baseline-std | 0 |
| meshlets-build-baseline-no-std | 0 |
| meshlets-run-baseline-no-std | 0 |
| meshlets-build-candidate-std | 0 |
| meshlets-run-candidate-std | 0 |
| meshlets-build-candidate-no-std | 0 |
| meshlets-run-candidate-no-std | 0 |
| meshlets-exact-std | 0 |
| meshlets-exact-no-std | 0 |
| cpp-build | 0 |
| cpp-cache-decode | 0 |
| reference | 0 |
| cpp-meshlets-clod | 0 |
| fuzz | 0 |

Exact cache/decode comparisons:

- compared 992130 differences 0
- compared 992130 differences 0

Connected geometry work sweeps:

- proof-geometry-exact-std.log: listed (64, 124, 41, 0.75, 2.0) compared 1020255 differences 0
- proof-geometry-exact-no-std.log: listed (64, 124, 41, 0.75, 2.0) compared 1020255 differences 0

The inherited disconnected meshlet/LOD exact driver added 154,590 matching
records per feature mode. Total: 4,333,950 baseline/candidate comparisons.
The C++ cache/decode driver passed 1,932 cases; the pinned C++ meshlet/LOD
protocol passed 1,280 cases in each std and no_std mode.

Differential fuzz: 2284000 cases, 300.126 child CPU seconds, exit 0.
The generator now reaches >=256-face candidate lists and tiny/large
finite coordinates, and includes attributes, locks and both LOD defaults.
Exact drivers compare full caller buffers, results and Usage, callback
records and position bits at every work limit through success+1 on the
selected meshes and settings. Repeated/high-valence cache inputs and
connected/repeated/coincident geometry exercise the work-bound paths.
The inherited exact driver adds disconnected broad KD searches.

These are finite boundary domains, supplemented by the written proof,
rather than enumeration of all floating values, strides, numeric
settings, option products or allocation failures. The native-root
argument follows the baseline release disposition; its exhaustive
root experiment was not rerun. Runtime verification is for this host
only: other architectures, wasm and no_std performance were not
measured. No independent review, release, merge, tag or publication
is claimed. The change is submitted as a draft.

## Reproduction and retention

README.md gives the admitted build/proof/measurement sequence.
REGISTRATION.md fixes arms and gates; source and binary bindings,
input hashes, every accepted instruction sample and exact-step
receipts are committed alongside this report. Raw outputs, perf
records, profiles, private inputs, executables and admission logs
remain in scratch. The dedicated target cleanup is recorded in
cleanup.json after all lane jobs exit.

Final retained-evidence audit exited 0: all 456 raw counters, sample receipts
and complete output hashes, frozen sources and binaries were rechecked;
`audit.json` seals their hashes. The dedicated target was removed and its
absence verified (`cleanup.json`). Build/proof/measurement jobs declared
4 GB and received a wrapper-selected 6 GB cap. The final read-only file
audit declared 1 GB; generic interpreter class history selected a 9 GB
reservation/cap. These are admission disclosures, not measured memory costs.
Final public-text and whitespace checks exited 0.
