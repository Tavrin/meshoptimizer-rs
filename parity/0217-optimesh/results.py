"""Summarize a measure.py campaign: medians, ratios and the registered gate.

Usage: results.py COUNTS_JSONL OUTPUT_JSON
"""
import json
import math
import statistics
import sys

rows = [json.loads(line) for line in open(sys.argv[1])]
cases = sorted({r['case'] for r in rows}, key=[r['case'] for r in rows].index)
real = [c for c in cases if c.startswith('real-')]
med = {}
hashes = {}
for c in cases:
    for arm in ['base', 'candidate', 'cpp', 'rust']:
        cell = [r for r in rows if r['case'] == c and r['arm'] == arm]
        assert len(cell) == 3, (c, arm, len(cell))
        med[c, arm] = statistics.median(r['instructions_per_call'] for r in cell)
        hashes[c, arm] = {r['output_sha256'] for r in cell}
table = []
for c in cases:
    row = dict(case=c, **{arm: med[c, arm] for arm in ['base', 'candidate', 'cpp', 'rust']})
    row['candidate/base'] = med[c, 'candidate'] / med[c, 'base']
    row['candidate/optimesh'] = med[c, 'candidate'] / med[c, 'rust']
    row['candidate/cpp'] = med[c, 'candidate'] / med[c, 'cpp']
    row['base/optimesh'] = med[c, 'base'] / med[c, 'rust']
    row['base_candidate_identical'] = hashes[c, 'base'] == hashes[c, 'candidate'] and len(hashes[c, 'base']) == 1
    row['candidate_cpp_identical'] = hashes[c, 'candidate'] == hashes[c, 'cpp']
    table.append(row)


def geo(key):
    values = [r[key] for r in table if r['case'] in real]
    return math.exp(sum(map(math.log, values)) / len(values))


gate = dict(candidate_over_optimesh=geo('candidate/optimesh'), candidate_over_base=geo('candidate/base'),
            candidate_over_cpp=geo('candidate/cpp'), base_over_optimesh=geo('base/optimesh'),
            max_candidate_over_base=max(r['candidate/base'] for r in table),
            all_base_candidate_identical=all(r['base_candidate_identical'] for r in table))
lead = gate['candidate_over_optimesh'] <= 0.99
gain = gate['candidate_over_base'] <= 0.97
gate['verdict'] = 'SHIPS' if lead and gain else 'IMPROVES' if gain else 'NO-GAIN'
gate['revert_required'] = [r['case'] for r in table if r['candidate/base'] > 1.01]
json.dump(dict(cases=table, gate=gate), open(sys.argv[2], 'w'), indent=2)
for r in table:
    print(f"{r['case']:15} base={r['base']:.0f} cand={r['candidate']:.0f} cpp={r['cpp']:.0f} "
          f"optimesh={r['rust']:.0f} c/b={r['candidate/base']:.4f} c/o={r['candidate/optimesh']:.4f} "
          f"c/cpp={r['candidate/cpp']:.4f} same={r['base_candidate_identical']} cpp_same={r['candidate_cpp_identical']}")
print(json.dumps(gate))
