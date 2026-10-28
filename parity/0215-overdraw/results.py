"""Apply REGISTRATION.md to one finished campaign. Usage: results.py CAMPAIGN."""
import json
import math
import os
from pathlib import Path
import statistics
import sys

out_dir = Path(os.environ['PEERS_ART']) / sys.argv[1]
rows = [json.loads(line) for line in (out_dir / 'counts.jsonl').read_text().splitlines()]
inputs = json.loads((out_dir / 'campaign.json').read_text())['inputs']
arms = ['base', 'candidate', 'cpp', 'rust']
cases = {}
for case in inputs:
    key = (case['case'], 'overdraw')
    samples = {a: [r for r in rows if (r['case'], r['op'], r['arm']) == (*key, a)] for a in arms}
    assert all(len(v) == 3 and sorted(r['rep'] for r in v) == [0, 1, 2] for v in samples.values())
    assert all(r['exit'] == 0 and r['running_percent'] == 100 for v in samples.values() for r in v)
    assert all(len({r['output_sha256'] for r in v}) == 1 for v in samples.values())
    output = {a: v[0]['output_sha256'] for a, v in samples.items()}
    assert output['base'] == output['candidate'], key
    med = {a: statistics.median(r['instructions_per_call'] for r in v) for a, v in samples.items()}
    cases[case['case']] = dict(medians=med, output_sha256=output,
                               candidate_base=med['candidate'] / med['base'],
                               candidate_cpp=med['candidate'] / med['cpp'],
                               candidate_optimesh=med['candidate'] / med['rust'],
                               base_optimesh=med['base'] / med['rust'],
                               base_cpp=med['base'] / med['cpp'])
real = [c['case'] for c in inputs if c['case'].startswith('real-')]
assert len(real) == 3


def gm(field, names):
    return math.exp(sum(math.log(cases[c][field]) for c in names) / len(names))


corpus = [c for c in cases if c not in real]
summary = {f: dict(real=gm(f, real), corpus=gm(f, corpus))
           for f in ['candidate_base', 'candidate_optimesh', 'candidate_cpp', 'base_optimesh', 'base_cpp']}
regressions = {c: v['candidate_base'] for c, v in cases.items() if v['candidate_base'] > 1.01}
base_ok = summary['candidate_base']['real'] <= 0.97
verdict = ('SHIPS' if base_ok and summary['candidate_optimesh']['real'] <= 0.99
           else 'IMPROVES' if base_ok else 'NO-GAIN')
result = dict(verdict=verdict, regressions=regressions, summary=summary, cases=cases)
(out_dir / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(dict(verdict=verdict, regressions=regressions, summary=summary), indent=2))
raise SystemExit(1 if regressions else 0)
