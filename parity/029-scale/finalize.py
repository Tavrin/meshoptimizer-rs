#!/usr/bin/env python3
"""Bind retained receipts and render measured results into the lane report."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib, json, os, platform, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-029-scale'))
rows=json.loads((ART/'instructions-revised/summary.json').read_text())
gates=dict(line.split('\t') for line in (ART/'gates.tsv').read_text().splitlines())
required={'instructions','fmt','all-features','no-default-features','clippy',
          'boundary','exact-build-std','exact-std','exact-build-no-std',
          'exact-no-std','cpp-differential','fuzz'}
assert required <= gates.keys() and all(v=='0' for v in gates.values()),gates
build=json.loads((ART/'build.json').read_text())
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==sha
           for p,sha in build['sources'].items())

real=[r for r in rows if len(r['case'])==4]
packed=[r for r in rows if r['stride']==12]
verdict='FAIL' if any(r['baseline_ratio']>1 for r in real) else 'PASS' if all(r['baseline_ratio']<=0.90 for r in real) else 'PARTIAL'
packed_pass=max(r['baseline_ratio'] for r in packed)<=1.01
if not packed_pass: verdict='FAIL'
fuzz=json.loads((ART/'fuzz-results/fuzz.json').read_text()) if (ART/'fuzz-results/fuzz.json').exists() else None
assert fuzz and fuzz['passed'] and all(r['cpu_seconds'] >= 300 and r['crashes'] == 0 for r in fuzz['runs'])
receipt=dict(fuzz=fuzz,verdict=verdict,rows=rows,gates=gates,build=json.loads((ART/'build.json').read_text()),platform=platform.platform(),rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu=subprocess.check_output(['lscpu'],text=True),git_base=subprocess.check_output(['git','rev-parse','v0.2.8'],text=True).strip(),source={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for folder in ['src','fuzz/src'] for p in (ROOT/folder).rglob('*.rs')})
(ROOT/'parity/029-scale/results.json').write_text(json.dumps(receipt,indent=2)+'\n')
def table(values):
    text='| Case | v0.2.8 instructions | Candidate | M7 | Candidate/base | Candidate/M7 |\n|---|---:|---:|---:|---:|---:|\n'
    for r in values:
        m=r['median'];text+=f"| {r['case']} | {m['base']:.2f} | {m['candidate']:.2f} | {m['M7']:.2f} | {r['baseline_ratio']:.6f} | {r['M7_ratio']:.6f} |\n"
    return text
for epoch in ['final','revised']:
    (ROOT/f'parity/029-scale/instructions-{epoch}.json').write_bytes((ART/f'instructions-{epoch}/raw.json').read_bytes())
p=ROOT/'parity/029-scale/REPORT.md'
s=p.read_text();s+='\n## Final measured results\n\n**'+verdict+'** under the registered instruction gate, including the packed non-regression limit.\n\n'+table(real)+'\nBoth ordinary and legacy scale stages use the same input; each row measures\nthe shared stage call. Frozen corpus (12 = packed, 64 = real byte layout):\n\n'+table([r for r in rows if r not in real])+'\nCounts exclude parsing, conversion, input allocation and warm-up. Rust and C++\ncall the nonallocating public scale API and consume its result; no result\ntransport allocation or importer/non-mesh stage is included. All resident\noutputs agree bit-for-bit across all three arms. No whole-cook latency claim.\n\n## Gate exit codes\n\n| Gate | Exit |\n|---|---:|\n'+''.join(f'| {k} | {v} |\n' for k,v in gates.items())+_public_path('\nRaw perf counters, stderr, input and binary hashes, exact checker binaries,\nC++ protocol observations and fuzz receipts are retained under\n`<scratch>/meshopt-029-scale/`. `results.json` binds the\nsource hashes, compiler, CPU, profile and per-case instruction medians.\n')
p.write_text(s)
print(verdict)
