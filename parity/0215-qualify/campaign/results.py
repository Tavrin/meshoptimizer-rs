"""Release table from the scalar campaign (campaign/counts.jsonl).

Usage: results.py COUNTS_JSONL OUT_DIR. Writes OUT_DIR/results.json and
OUT_DIR/TABLE.md. Per function and comparison (train vs v0.2.14, vs optimesh
1.1.0, vs C++ 1.3): geometric mean of per-case median instruction ratios over
the three real meshes and over the five corpus meshes; the largest per-case
train/v0.2.14 ratio; any case above 1.01 is flagged. Output identity is
counted per case (train vs C++, train vs optimesh).
"""
import json
import math
from pathlib import Path
import statistics
import sys

rows = [json.loads(line) for line in Path(sys.argv[1]).read_text().splitlines()]
out = Path(sys.argv[2])
real = ['real-214e', 'real-a082', 'real-b6fd']
corpus = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth']
ops = list(dict.fromkeys(r['op'] for r in rows))
med, sha = {}, {}
for r in rows:
    med.setdefault((r['op'], r['case'], r['arm']), []).append(r['instructions_per_call'])
    sha.setdefault((r['op'], r['case'], r['arm']), set()).add(r['output_sha256'])
for k, v in med.items():
    assert len(v) == 3, (k, len(v))
    med[k] = statistics.median(v)
for k, v in sha.items():
    assert len(v) == 1, ('unstable output', k)


def gm(values):
    return math.exp(sum(math.log(v) for v in values) / len(values)) if values else None


results, flags = {}, []
for op in ops:
    entry = {}
    for peer, name in [('base', 'v0.2.14'), ('rust', 'optimesh'), ('cpp', 'cpp')]:
        ratios = {c: med[(op, c, 'candidate')] / med[(op, c, peer)]
                  for c in real + corpus if (op, c, peer) in med}
        entry[name] = dict(real_gm=gm([ratios[c] for c in real if c in ratios]),
                           corpus_gm=gm([ratios[c] for c in corpus if c in ratios]), cases=ratios)
    base = entry['v0.2.14']['cases']
    entry['max_case_vs_v0.2.14'] = max(base.values())
    for case, ratio in base.items():
        if ratio > 1.01:
            flags.append(dict(op=op, case=case, ratio=ratio))
    for peer in ['cpp', 'rust']:
        same = [c for c in real + corpus if (op, c, peer) in sha and sha[(op, c, peer)] == sha[(op, c, 'candidate')]]
        total = [c for c in real + corpus if (op, c, peer) in sha]
        entry['identical_' + peer] = f'{len(same)}/{len(total)}'
    assert all(sha[(op, c, 'base')] == sha[(op, c, 'candidate')] for c in real + corpus), op
    results[op] = entry
(out / 'results.json').write_text(json.dumps(dict(ops=results, flags_above_1_01=flags,
                                                  samples=len(rows)), indent=2) + '\n')


def f(v):
    return '—' if v is None else f'{v:.3f}'


lines = ['# 0.2.15 release table (scalar, instructions:u)', '',
         'Median of three per case; GM = geometric mean of per-case ratios (train / peer).',
         'Real = real-214e, real-a082, real-b6fd; corpus = tiny, medium-smooth, medium-seams,',
         'medium-sparse, million-smooth. SIMD off in every arm: meshoptimizer-rs without `simd`,',
         'optimesh 1.1.0 without `simd`, C++ 1.3 (meshopt 0.7.0) built with MESHOPTIMIZER_NO_SIMD.',
         'Train and v0.2.14 outputs are byte-identical in every cell.', '',
         '| Function | vs 0.2.14 real | vs 0.2.14 corpus | max case vs 0.2.14 | vs optimesh real | vs optimesh corpus | vs C++ real | vs C++ corpus | same output as C++ | same as optimesh |',
         '|---|---:|---:|---:|---:|---:|---:|---:|---|---|']
for op, e in results.items():
    lines.append(f"| {op} | {f(e['v0.2.14']['real_gm'])} | {f(e['v0.2.14']['corpus_gm'])} | "
                 f"{f(e['max_case_vs_v0.2.14'])} | {f(e['optimesh']['real_gm'])} | {f(e['optimesh']['corpus_gm'])} | "
                 f"{f(e['cpp']['real_gm'])} | {f(e['cpp']['corpus_gm'])} | {e['identical_cpp']} | {e['identical_rust']} |")
lines += ['', 'Cases above 1.01 vs v0.2.14: ' + ('none' if not flags else
          ', '.join(f"{x['op']}/{x['case']} {x['ratio']:.4f}" for x in flags)), '']
(out / 'TABLE.md').write_text('\n'.join(lines))
print('\n'.join(lines))
