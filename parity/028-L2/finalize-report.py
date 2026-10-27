#!/usr/bin/env python3
"""Publish the sole campaign's verbatim verdicts and retain compact evidence."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
LANE=ROOT/'parity/028-L2'
ART=Path(_public_path('<scratch>/meshopt-028-L2'))
def read(name):return json.loads((ART/name).read_text())
wall,proof,fuzz=read('campaign.json'),read('proof.json'),read('fuzz-artifacts/fuzz/record.json')
assert wall['completed'] and proof['passed'] and fuzz['passed']
assert read('audit.json')['passed']
summary='''\n## Final registered results\n\nExactly one registered campaign, CPU 9 (sibling 8), a downstream engine scalar profile,\none warm-up plus ten interleaved pairs; 10,000 bootstrap resamples, seed\n20261030. Five equally weighted frozen cases per family; overdraw also\nincludes all three retained real-overdraw cases. Ratios are ours/comparator.\nX is disclosed only. The table applies the registered verdicts without retries.\n\n'''
summary+=f"Same-session A/A floor F = {wall['F']:.6f}; per-case upper95 limit = {1.10+wall['F']:.6f}.\n\n"
summary+='| Family | Ours/M7 GM | Ours/v0.2.7 GM | Worst baseline upper95 | Ours/X GM | Verdict |\n|---|---:|---:|---:|---:|---|\n'
for r in wall['families']:
 summary+=f"| {r['operation']} | {r['ratios']['M7']:.6f} | {r['ratios']['base']:.6f} | {r['max_baseline_upper95']:.6f} | {r['ratios']['X']:.6f} | **{r['verdict']}** |\n"
summary+='''\nThe GM bars are <=1.00 against both M7 and v0.2.7; every case must also\npass the baseline upper95 bar. PARTIAL means the baseline bars pass but\nM7 is missed: a measured speedup, with no performance-parity claim. FAIL\nmeans at least one baseline bar misses. No verdict is rounded into a pass.\n\n### Three-chain disclosure (not a bar)\n\nAll ordinary and legacy r2 calls, with their actual pruned positions recovered\nfrom the immutable compacted captures. The corrected candidate/base/M7 proof\nreproduces every historical stage output. These are matched stage replays,\nexcluding importer, policy evaluation, serialization and non-mesh work;\nno whole-a downstream engine latency or production acceptance claim follows.\n\n| Chain | Arm/CPP | Scale GM | Simplify GM | Cache GM | Overdraw GM |\n|---|---|---:|---:|---:|---:|\n'''
for r in wall['stages']:
 ratios=r['ratios'];label='candidate/M7' if r['comparison']=='M7' else 'v0.2.7/M7'
 summary+=f"| {r['case'][:4]} | {label} | "+' | '.join(f"{ratios[op]['gm']:.6f}" for op in ['simplify-scale','simplify','cache','overdraw'])+' |\n'
summary+='''\nEvery stage interval, paired call time and output hash is retained in\n`results.json` / scratch `campaign.json`; family and per-case intervals are\nretained there too. No samples were discarded, and no wall campaign was rerun.\n\n### Causes and retained fix\n\n- Cache: per-neighbor fuel checks in adjacency searches/score updates.\n  `src/cache.rs:175`, `:190`, `:205` use exact-prefix scan/search batching.\n- FIFO: per-corner division and dynamic triangle slice checks.\n  `src/cache.rs:416`, `:445` use triangle records, retaining the original\n  per-corner fuel-exhaustion path.\n- Scale: finite checks interrupt the packed bounds reduction.\n  `src/simplify.rs:188` separates its finite reduction; strided bounds and\n  the shared simplifier helper remain unchanged.\n- Overdraw: dynamic byte offset/count checks for each indexed XYZ corner.\n  `src/input.rs:688` uses one checked full 64-byte record lookup. Mapped,\n  other-stride and final-XYZ-only views retain the general reader.\n\nSafe Rust only; no public API, explicit SIMD path, encoder, memory policy,\nversion or a downstream engine source change. The shared cache-strip implementation is\ncovered by exact proof. Input preparation is equal in all arms; the final\nbaseline/M7/X instruction audit agrees with the initial audit to four decimal\nplaces in family GM (see retained raw counts).\n\n### Targeted gate exits\n\n'''
codes={}
for line in (ART/'gates.tsv').read_text().splitlines():
 name,code=line.split('\t');codes[name]=int(code)
summary+='| Gate | Exit |\n|---|---:|\n'
for name,code in codes.items():summary+=f'| {name} | {code} |\n'
for name in ['v0.2.7 exact std','v0.2.7 exact no_std','C++ differential','final family and historical-stage adapters','widened fuzz','registered campaign','evidence audit']:
 summary+=f'| {name} | 0 |\n'
summary+=f"\nExact proof: {proof['exact_counts']['std']['observations']:,} observations plus {proof['exact_counts']['std']['scale_seed']:,} scale-seed observations in **each** of std/no_std; {len(proof['cpp'])} existing strict C++ protocol cases; {proof['seed_cases']} existing FIFO seed/native/JS cases; 23 final family cases and all 56 historical stage calls in each of candidate/base/M7.\n\n"
for r in fuzz['runs']:
 summary+=f"- {r['target']}: {r['cpu_seconds']:.2f} CPU-seconds, {r['executions']:,} executions, {r['crashes']} crashes.\n"
summary+='''\nThis is stable seeded mutation proof, without sanitizer or coverage-guided\ninstrumentation. The widened layouts include 64-byte full/final-short records,\nunaligned little/big endian storage and arbitrary float bits.\n\nEarlier rejected probes, consumer compile error, wrong-vertex stage replay,\nlauncher exit and bounded EASY timeout are retained below and in scratch.\nThey are not cleared or recast as passing acceptance samples. The library\nproofs/gates listed above bind the final source; no wall value caused a retry.\n\n'''
p=LANE/'REPORT.md';body=p.read_text();assert '## Final registered results' not in body
p.write_text(body.split('\n',1)[0]+'\n'+summary+body.split('\n',1)[1])
# Measured numbers only in the requested existing changelog file.
entry='## [Unreleased]\n\n'
for r in wall['families']:
 entry+=f"- `{r['operation']}` takes {r['ratios']['base']:.3f}× the time of 0.2.7 and {r['ratios']['M7']:.3f}× the time of C++ 1.3 on the registered a downstream engine scalar cases.\n"
entry+='\n'
p=ROOT/'CHANGELOG.md';body=p.read_text();assert '## [Unreleased]' not in body
body=body.replace('## [0.2.7]',entry+'## [0.2.7]',1);p.write_text(body)
for src,dst in [('campaign.json','results.json'),('audit.json','audit.json'),('inputs-sha256.json','inputs-sha256.json'),('stages.json','stages.json'),('instructions-baseline.json','instructions-baseline.json'),('instructions-probe1.json','instructions-probe1.json'),('instructions-probe2.json','instructions-probe2.json'),('instructions-probe3.json','instructions-probe3.json'),('instructions-final-stage.json','instructions-final.json')]:
 shutil.copy2(ART/src,LANE/dst)
compact=dict(passed=True,source=proof['source'],gates=proof['gates'],exact_counts=proof['exact_counts'],cpp_cases=len(proof['cpp']),seed_cases=proof['seed_cases'],frozen_cases=len(proof['frozen']),stage_arms=len(proof['stages']),proof_sha256=hashlib.sha256((ART/'proof.json').read_bytes()).hexdigest(),fuzz_runs=fuzz['runs'],fuzz_sha256=hashlib.sha256((ART/'fuzz-artifacts/fuzz/record.json').read_bytes()).hexdigest())
(LANE/'proof-summary.json').write_text(json.dumps(compact,indent=2)+'\n')
print(json.dumps(wall['families'],indent=2))
