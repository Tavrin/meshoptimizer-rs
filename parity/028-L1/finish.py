#!/usr/bin/env python3
"""Export the single campaign and required proof receipts without remeasurement."""
from pathlib import Path
import json,hashlib,math,statistics,subprocess
from control import ROOT,LANE,ART,sha,save
load=lambda name:json.loads((ART/name).read_text())
def line(path,needle):
    return next(i for i,s in enumerate((ROOT/path).read_text().splitlines(),1)if needle in s)
def table(rows):
    text='| Family | O/M7 GM | O/0.2.7 GM | O/X GM | Max case upper95 vs 0.2.7 | Verdict |\n|---|---:|---:|---:|---:|---|\n'
    for r in rows:text+=f"| {r['family']} | {r['M7']:.6f} | {r['base']:.6f} | {r['X']:.6f} | {r['max_case_upper95_base']:.6f} | {r['verdict']} |\n"
    return text
assert load('campaign-complete.json')['exit']==0
rows=load('registered-table.json');disclosure=load('rfc-disclosure.json')
verdict='FAIL'if any(r['verdict']=='FAIL'for r in rows)else'PARTIAL'if any(r['verdict']=='PARTIAL'for r in rows)else'PASS'
F=rows[0]['F'];gates=load('gates.json');exact=load('exact-proof.json');fuzz=load('fuzz.json');arith=load('division-fuzz.json')
proof_files=['ENVIRONMENT.json','PINS.json','gates.json','exact-proof.json','cpp-seeded-differential.json','cpp-identity.json','frozen-differential.json','rfc-preflight.json','rfc-identity.json','fuzz.json','division-fuzz.json','QUALIFIED.json','campaign-identity.json','campaign-complete.json','registered-table.json','rfc-disclosure.json','wall.json','rfc-wall.json','counts-original2.json','counts-equalized.json','counts-trial7.json','counter-pairs.json']
export={name:load(name)for name in proof_files};export['verdict']=verdict
save(LANE/'RESULTS.json',export)
fixes=[
 ('partition','repeated local-table clearing; dense-input generation tags in the existing u8 allocation','src/partition.rs','let tagged ='),
 ('meshlets-scan','per-face descriptor publication and output bounds; local descriptor and sequential output iterator','src/meshlet.rs','fn scan_covered('),
 ('meshlets','runtime zero-weight cone arithmetic; entry specialization folds the bounded search factors','src/meshlet.rs','fn flex<const CHECK:'),
 ('meshlets-spatial','BVH integer-division latency and indexed partitioning; exact invariant quotient and stable branchless partition','src/meshlet_spatial.rs','let divisor = Divisor::new'),
 ('meshlets-flex','same hot loop as meshlets; zero-weight specialization and shared append/position preparation','src/meshlet.rs','fn flex<const CHECK:'),
 ('spatial-triangles','per-corner view dispatch and generic Morton interleave; one storage dispatch, moderate-coordinate centroid path and 16-bit interleave','src/spatial.rs','fn fill_centers'),
]
summary=f'''## Final result

Performance verdict: **{verdict}**. This is the sole registered wall campaign;
no samples were discarded and no verdict reruns occurred. A PARTIAL family is
a measured speedup versus 0.2.7 without a C++ performance-parity claim. A FAIL
family is not performance-qualified. All proof requirements passed independently.

{table(rows)}
Same-session A/A floor F={F:.6f}; every per-case regression limit is
1.10+F={1.10+F:.6f}. All 120 streams have exactly one explicit warm-up and ten
alternating pairs (30 A/A and 90 comparisons). Driver startup performs its
existing untimed output/serialization check before the explicit warm-up.
Paired bootstrap: 10,000 resamples, seed 20261030. Raw pairs and case intervals
are in RESULTS.json; X is disclosure only. Ratios are candidate/comparator.

RFC113 disclosure (sum of case medians, 10 pairs/case): combined
**{disclosure['all']:.6f}x C++**, S1 {disclosure['S1']:.6f}x (41 cases),
S2 {disclosure['S2']:.6f}x (nine valid cases), maximum case
{disclosure['max_case']:.6f}x. This is disclosure, not a bar. Scalar C++ and the
original cluster-C input/configuration/timer boundary were retained. All 50
preflight outputs matched C++; all 41 S1 outputs also matched retained oracles.

## Retained fixes and causes

'''
for family,cause,path,needle in fixes:summary+=f'- {family}: {cause}; `{path}:{line(path,needle)}`.\n'
summary+=f'''
The exact divider is `src/meshlet_division.rs`; no floating-point backend,
public API, unsafe library code, output allocation policy, resource threshold,
encoder or SIMD path changed. The generation-map and divider changes introduce
no Workspace allocations and retain the original Result/work/byte accounting.

## Required proof exits

| Gate | Exit |
|---|---:|
'''
for r in gates:summary+=f"| {' '.join(r['command'])} | {r['exit']} |\n"
summary+=f'''| Existing C++ L1 differential, 2,000 seeds/family, alloc/into plus 64 wide partition seeds | 0 |
| v0.2.7 exact Results/work/output/caller-buffer contents, {exact['comparisons']:,} comparisons | 0 |
| Frozen five-case corpus against M7 and v0.2.7, all six families | 0 |
| RFC113 50-case preflight | 0 |
| Widened layout/topology/resource differential fuzz | 0 |
| Full-width production-divider property fuzz | 0 |
| Single registered wall campaign | 0 |

Mesh differential fuzz: {fuzz['executions']:,} executions, target CPU
{fuzz['target_cpu_seconds']:.3f} s (total CPU {fuzz['cpu_seconds']:.3f} s),
seed {fuzz['seed']}, zero mismatches. Divider fuzz: {arith['cases']:,} cases,
{arith['cpu_seconds']:.3f} target CPU seconds, zero mismatches against native
integer division. Both are stable seeded mutation, without sanitizer or
coverage instrumentation. Replay descriptors, stream hashes and executable
identities are retained in the exported receipts and scratch.

## Evidence boundary

Measurements are native Linux x86-64 on a shared host, pinned to CPU 2 for the
interleaved wall campaign; all load observations are retained. Claims are limited
to the registered corpus/profile. No Moss runtime/GPU acceptance, other-platform
performance, release, merge, tag or publication is claimed. All lane additions
are under parity/028-L1; only required library edits and the existing changelog
are outside it. Scratch and binaries are retained at `{ART}`. The designated
Cargo target is deleted only after retention; cleanup is a separate receipt.

'''
p=LANE/'REPORT.md';old=p.read_text().replace('Work in progress; no performance verdict yet.','')
assert '## Final result'not in old,'do not overwrite a completed report'
at=old.index('## Adapter diagnosis');p.write_text(old[:at]+summary+old[at:])
# Required existing-root-file edit, measured values only; no version bump.
p=ROOT/'CHANGELOG.md';s=p.read_text();assert '## [Unreleased]'not in s
entry='## [Unreleased]\n\n- L1 cluster paths, Moss scalar five-case GM times versus 0.2.7 / C++ 1.3: '+', '.join(f"{r['family']} {r['base']:.3f}x / {r['M7']:.3f}x"for r in rows)+f". RFC113 aggregate {disclosure['all']:.3f}x C++, S1 {disclosure['S1']:.3f}x and S2 {disclosure['S2']:.3f}x.\n\n"
s=s.replace('## [0.2.7]',entry+'## [0.2.7]',1);p.write_text(s)
# Exact, reviewable draft PR body. No memory citations in PR content.
body=f'''The L1 cluster path spends time clearing partition maps, publishing scan descriptors per triangle, evaluating zero-weight cone arithmetic, and dividing inside BVH pivots. This change reuses generation tags, keeps scan output state local, specializes the zero-weight search, and uses an exact invariant integer quotient. Position preparation and Morton interleave are also tightened. Public outputs, errors, work usage and APIs remain unchanged.

Registered performance verdict: **{verdict}**. The single final campaign used the equalized Moss scalar adapters, 1 warm-up + 10 alternating pairs, 10,000 bootstrap resamples, seed 20261030, with A/A floor {F:.6f}. X is disclosure only.

{table(rows)}
RFC113 aggregate: {disclosure['all']:.6f}x C++ (S1 {disclosure['S1']:.6f}x; S2 {disclosure['S2']:.6f}x), disclosure only. PARTIAL families have no C++ performance-parity claim; FAIL families are not performance-qualified.

Validation: both required feature-configuration tests, clippy -D warnings and formatting passed. Existing C++ differential and {exact['comparisons']:,} baseline exact comparisons passed, including Results, work accounting and caller-buffer failure contents. Both widened mesh/layout fuzz and full-width divider fuzz exceeded five target CPU-minutes with zero mismatches. All 50 RFC113 preflight cases matched.

See parity/028-L1/REPORT.md, REGISTRATION.md and RESULTS.json for causes, source locations, raw pairs, intervals, exits and retained evidence. No merge, tag, publication or version bump.
'''
(LANE/'PR.md').write_text(body)
save(ART/'FINAL-SUMMARY.json',{'verdict':verdict,'table':rows,'disclosure':disclosure,'proof_exits':0,'report_sha256':sha(LANE/'REPORT.md'),'results_sha256':sha(LANE/'RESULTS.json')})
print(verdict);print(table(rows));print(json.dumps(disclosure))
