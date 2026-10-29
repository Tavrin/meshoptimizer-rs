"""Summarize a campaign: medians, ratios, real-mesh geometric means, verdicts.

Usage: results.py CAMPAIGN (under PEERS_ART). Writes CAMPAIGN/results.json and
prints a Markdown table. Ratios are of median instructions per call. With a
candidate arm the registered gate is applied: SHIPS if candidate/base <= 0.95
on the three-real-mesh geometric mean and no case above 1.01.
"""
import json
import math
import os
from pathlib import Path
import statistics
import sys

art = Path(os.environ['PEERS_ART'])
d = art / sys.argv[1]
camp = json.loads((d / 'campaign.json').read_text())
rows = [json.loads(s) for s in (d / 'counts.jsonl').read_text().splitlines()]
arms, ops, cases = camp['arms'], camp['ops'], [x['case'] for x in camp['inputs']]
real = [c for c in cases if c.startswith('real-')]
ours = 'candidate' if 'candidate' in arms else 'base'
med, sha = {}, {}
for r in rows:
    med.setdefault((r['case'], r['op'], r['arm']), []).append(r['instructions_per_call'])
    sha[(r['case'], r['op'], r['arm'])] = r['output_sha256']
med = {k: statistics.median(v) for k, v in med.items()}


def gm(values):
    return math.exp(sum(map(math.log, values)) / len(values))


out = {}
lines = ['| op | ' + ' | '.join(f'{ours}/{a}' for a in arms if a != ours) + ' | worst case vs base | same output as C++ |',
         '|---|' + '---|' * (len(arms) + 1)]
for op in ops:
    entry = {'cases': {}}
    for c in cases:
        entry['cases'][c] = {a: med[(c, op, a)] for a in arms}
    ratios = {a: gm([med[(c, op, ours)] / med[(c, op, a)] for c in real]) for a in arms if a != ours}
    entry['real_gm'] = ratios
    worst = max(med[(c, op, ours)] / med[(c, op, 'base')] for c in cases) if ours != 'base' else 1.0
    entry['worst_vs_base'] = worst
    same = all(sha[(c, op, ours)] == sha[(c, op, 'cpp')] for c in cases) if 'cpp' in arms else None
    entry['same_output_as_cpp'] = same
    if ours == 'candidate':
        entry['verdict'] = 'SHIPS' if ratios['base'] <= 0.95 and worst <= 1.01 else 'NO-GAIN'
    out[op] = entry
    lines.append(f'| {op} | ' + ' | '.join(f'{ratios[a]:.3f}' for a in arms if a != ours)
                 + f' | {worst:.3f} | {same} |')
(d / 'results.json').write_text(json.dumps(out, indent=2) + '\n')
print('\n'.join(lines))
