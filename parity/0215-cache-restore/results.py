"""Evaluate median-of-three cache instruction counts and export portable results."""
import json
import math
from pathlib import Path
import statistics
import sys

campaign = Path(sys.argv[1])
rows = [json.loads(s) for s in (campaign / 'counts.jsonl').read_text().splitlines()]
reg = json.loads((campaign / 'campaign.json').read_text())
arms = reg['arms']
cells = {}
for x in reg['inputs']:
    case = x['case']
    counts = {}
    hashes = set()
    for arm in arms:
        rs = [r for r in rows if r['case'] == case and r['arm'] == arm]
        assert len(rs) == 3 and {r['rep'] for r in rs} == {0, 1, 2}
        assert all(r['exit'] == 0 and r['running_percent'] == 100 for r in rs)
        counts[arm] = statistics.median(r['instructions_per_call'] for r in rs)
        hashes.update(r['output_sha256'] for r in rs)
    assert len(hashes) == 1
    cells[case] = dict(medians=counts, output_sha256=hashes.pop(), ratios_base={a: counts[a] / counts['base'] for a in arms})

def gm(values):
    return math.exp(statistics.mean(math.log(x) for x in values))

summary = {}
for arm in arms:
    summary[arm] = {}
    for group in ['real', 'corpus']:
        cs = [v for k, v in cells.items() if k.startswith('real-') == (group == 'real')]
        summary[arm][group] = {peer: gm(c['medians'][arm] / c['medians'][peer] for c in cs) for peer in arms}
    summary[arm]['max_vs_base'] = max(c['ratios_base'][arm] for c in cells.values())
verdict = None
if 'candidate' in arms:
    c = summary['candidate']
    verdict = 'SHIPS' if c['real']['base'] <= 0.97 and c['max_vs_base'] <= 1.01 else 'NO-GAIN'
result = dict(samples=len(rows), cells=cells, summary=summary, verdict=verdict)
(campaign / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(dict(summary=summary, verdict=verdict), indent=2))
