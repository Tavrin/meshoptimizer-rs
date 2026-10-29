"""Apply REGISTRATION.md to one finished campaign. Usage: results.py CAMPAIGN."""
import json
import math
import os
from pathlib import Path
import statistics
import sys

out_dir = Path(os.environ['PEERS_ART']) / sys.argv[1]
rows = [json.loads(line) for line in (out_dir / 'counts.jsonl').read_text().splitlines()]
campaign = json.loads((out_dir / 'campaign.json').read_text())
inputs, ops = campaign['inputs'], campaign['ops']
real = [c['case'] for c in inputs if c['case'].startswith('real-')]
assert len(real) == 3
corpus = [c['case'] for c in inputs if c['case'] not in real]


def gm(values):
    values = [v for v in values if v is not None]
    return math.exp(sum(math.log(v) for v in values) / len(values)) if values else None


result = dict(ops={})
regressions = {}
for op in ops:
    arms = ['base', 'candidate', 'cpp'] + ([] if op == 'cluster-lod' else ['rust'])
    cases = {}
    for case in inputs:
        key = (case['case'], op)
        samples = {a: [r for r in rows if (r['case'], r['op'], r['arm']) == (*key, a)] for a in arms}
        assert all(len(v) == 3 and sorted(r['rep'] for r in v) == [0, 1, 2] for v in samples.values()), key
        assert all(r['exit'] == 0 and r['running_percent'] == 100 for v in samples.values() for r in v)
        assert all(len({r['output_sha256'] for r in v}) == 1 for v in samples.values())
        output = {a: v[0]['output_sha256'] for a, v in samples.items()}
        assert output['base'] == output['candidate'], key
        assert output['candidate'] == output['cpp'], ('C++ output', key)
        med = {a: statistics.median(r['instructions_per_call'] for r in v) for a, v in samples.items()}
        opt = med.get('rust')
        cases[case['case']] = dict(
            medians=med, output_sha256=output,
            candidate_base=med['candidate'] / med['base'],
            candidate_cpp=med['candidate'] / med['cpp'],
            base_cpp=med['base'] / med['cpp'],
            candidate_optimesh=med['candidate'] / opt if opt else None,
            base_optimesh=med['base'] / opt if opt else None)
        if cases[case['case']]['candidate_base'] > 1.01:
            regressions[f"{case['case']}/{op}"] = cases[case['case']]['candidate_base']
    fields = ['candidate_base', 'candidate_cpp', 'base_cpp', 'candidate_optimesh', 'base_optimesh']
    summary = {f: dict(real=gm(cases[c][f] for c in real), corpus=gm(cases[c][f] for c in corpus))
               for f in fields}
    base_ok = summary['candidate_base']['real'] <= 0.97
    # Registered substitution for these already-leading functions: the
    # optimesh condition becomes candidate/C++ at least 0.05 below base/C++.
    cpp_ok = summary['base_cpp']['real'] - summary['candidate_cpp']['real'] >= 0.05
    verdict = 'SHIPS' if base_ok and cpp_ok else 'IMPROVES' if base_ok else 'NO-GAIN'
    result['ops'][op] = dict(verdict=verdict, summary=summary, cases=cases)
result['regressions'] = regressions
(out_dir / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(dict(regressions=regressions, ops={op: dict(verdict=v['verdict'], real={
    f: v['summary'][f]['real'] for f in v['summary']}) for op, v in result['ops'].items()}), indent=2))
raise SystemExit(1 if regressions else 0)
