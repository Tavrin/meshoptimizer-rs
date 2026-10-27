import os
"""Apply SPEC-0212-simplify2's registered four-arm instruction gate."""
from pathlib import Path
import collections
import json
import math
import statistics

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
ARMS = ['base', 'post39', 'candidate', 'M7']
rows = [json.loads(line) for line in (ART / 'counts.jsonl').read_text().splitlines()]
assert len(rows) == 576
by_cell = collections.defaultdict(dict)
for row in rows:
    assert row['exit'] == 0 and row['validation'] and row['running_percent'] == 100.0
    build = json.loads((ART / f"build-{row['arm']}.json").read_text())
    assert build['exit'] == 0 and row['binary_sha256'] == build['sha256']
    arm = by_cell[(row['family'], row['case'])].setdefault(row['arm'], {})
    assert row['rep'] not in arm
    arm[row['rep']] = row

def verdict(base, post, failed, real):
    if failed:
        return 'FAIL'
    if not real:
        return 'WITHIN-REGRESSION-LIMIT'
    return 'PASS' if base <= 0.80 else 'PARTIAL' if post <= 0.95 else 'NO-GAIN'

cells = []
for (family, case), arms in sorted(by_cell.items()):
    assert set(arms) == set(ARMS)
    med = {}
    for arm, reps in arms.items():
        assert set(reps) == {0, 1, 2}
        assert len({r['output_sha256'] for r in reps.values()}) == 1
        med[arm] = statistics.median(r['per_call'] for r in reps.values())
    assert len({arms[a][0]['output_sha256'] for a in ['base', 'post39', 'candidate']}) == 1
    base = med['candidate']/med['base']
    post = med['candidate']/med['post39']
    cells.append(dict(family=family, case=case, medians=med, candidate_base=base,
                      candidate_post39=post, candidate_M7=med['candidate']/med['M7'],
                      verdict=verdict(base, post, post > 1.01, family.startswith('real-'))))
assert len(cells) == 48
families = []
for family in sorted({c['family'] for c in cells}):
    group = [c for c in cells if c['family'] == family]
    real = family.startswith('real-')
    if real:
        totals = {a: sum(c['medians'][a] for c in group) for a in ARMS}
        base, post, peer = [totals['candidate']/totals[a] for a in ['base', 'post39', 'M7']]
    else:
        base, post, peer = [math.exp(statistics.mean(math.log(c[k]) for c in group))
                            for k in ['candidate_base', 'candidate_post39', 'candidate_M7']]
    families.append(dict(family=family, cases=len(group), candidate_base=base,
                         candidate_post39=post, candidate_M7=peer,
                         verdict=verdict(base, post, any(c['candidate_post39'] > 1.01 for c in group), real)))
failed = any(c['candidate_post39'] > 1.01 for c in cells)
result = dict(cells=cells, families=families, regression_fail=failed)
(ART / 'results.json').write_text(json.dumps(result, indent=2)+'\n')
for f in families:
    print(f"{f['family']}: base={f['candidate_base']:.6f} post39={f['candidate_post39']:.6f} M7={f['candidate_M7']:.6f} {f['verdict']}")
if failed:
    for c in cells:
        if c['candidate_post39'] > 1.01:
            print('REJECT', c['family'], c['case'], c['candidate_post39'])
    raise SystemExit(1)
