"""Apply the pre-registered three-arm instruction gate without rounding."""
from pathlib import Path
import collections
import json
import math
import os
import statistics

ART = Path(os.environ['MESHOPT_SCRATCH'])
ARMS = ['base', 'candidate', 'M7']
rows = [json.loads(line) for line in (ART/'counts.jsonl').read_text().splitlines()]
assert len(rows) == 432
by_cell = collections.defaultdict(dict)
for row in rows:
    assert row['exit'] == 0 and row['validation'] and row['running_percent'] == 100
    build = json.loads((ART/f"build-{row['arm']}.json").read_text())
    assert build['exit'] == 0 and row['binary_sha256'] == build['sha256']
    reps = by_cell[(row['family'],row['case'])].setdefault(row['arm'],{})
    assert row['rep'] not in reps
    reps[row['rep']] = row
cells = []
for (family,case),arms in sorted(by_cell.items()):
    assert set(arms) == set(ARMS)
    med = {}
    for arm,reps in arms.items():
        assert set(reps) == {0,1,2}
        assert len({r['output_sha256'] for r in reps.values()}) == 1
        med[arm] = statistics.median(r['per_call'] for r in reps.values())
    assert arms['base'][0]['output_sha256'] == arms['candidate'][0]['output_sha256']
    base,peer = [med['candidate']/med[a] for a in ['base','M7']]
    cells.append(dict(family=family,case=case,medians=med,candidate_base=base,candidate_M7=peer,
                      verdict='FAIL' if base>1.01 else 'WITHIN-REGRESSION-LIMIT'))
assert len(cells) == 48
families = []
for family in sorted({c['family'] for c in cells}):
    group = [c for c in cells if c['family'] == family]
    real = family.startswith('real-')
    if real:
        totals = {a:sum(c['medians'][a] for c in group) for a in ARMS}
        base,peer = [totals['candidate']/totals[a] for a in ['base','M7']]
    else:
        base,peer = [math.exp(statistics.mean(math.log(c[k]) for c in group)) for k in ['candidate_base','candidate_M7']]
    families.append(dict(family=family,cases=len(group),candidate_base=base,candidate_M7=peer,
        verdict='FAIL' if any(c['candidate_base']>1.01 for c in group) else
                ('PASS' if peer<=1.15 else 'PARTIAL' if base<=0.90 else 'NO-GAIN') if real else 'WITHIN-REGRESSION-LIMIT'))
failed = any(c['candidate_base']>1.01 for c in cells)
real = [f for f in families if f['family'].startswith('real-')]
verdict = 'FAIL' if failed else 'PASS' if all(f['candidate_M7']<=1.15 for f in real) else 'PARTIAL' if all(f['candidate_base']<=0.90 for f in real) else 'NO-GAIN'
result = dict(cells=cells,families=families,regression_fail=failed,verdict=verdict)
with (ART/'results.json').open('x') as stream:
    stream.write(json.dumps(result,indent=2)+'\n')
for f in families:
    print(f"{f['family']}: base={f['candidate_base']:.6f} M7={f['candidate_M7']:.6f} {f['verdict']}")
print('OVERALL',verdict)
raise SystemExit(int(failed))
