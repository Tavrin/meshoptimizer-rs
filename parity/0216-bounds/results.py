"""Emit the frozen median-of-three P3 gate, with wall-time disclosure."""
import hashlib,json,math,os,statistics,sys
from pathlib import Path
s=Path(os.environ['BOUNDS_SCRATCH']);d=s/sys.argv[1]
rows=[json.loads(l) for l in (d/'counts.jsonl').read_text().splitlines()]
arms=json.loads((d/'campaign.json').read_text())['arms']
candidate_default=next(a for a in arms if a.startswith('candidate') and a.endswith('-default'))
candidate_scalar=candidate_default.removesuffix('-default')+'-scalar'
cases=[x['case'] for x in json.loads((s/'inputs.json').read_text())]
ops=['meshlets','meshlets-flex','cluster-lod','cluster-bounds','meshlet-bounds']
def gm(xs):return math.exp(statistics.mean(map(math.log,xs)))
def cell(op,case):
 c=[r for r in rows if r['op']==op and r['case']==case]
 assert len(c)==len(arms)*3 and len({r['output_sha256'] for r in c})==1
 return {a:statistics.median(r['instructions_per_call'] for r in c if r['arm']==a) for a in arms}
metrics={};lines=['# P3 instruction gate','', 'Median of three `instructions:u` samples; 100% counter running. Lower is better.', '', '| Function | / v0.2.15 default real/corpus | Scalar / scalar C++ real/corpus | Default / default C++ real/corpus | Worst / base | Verdict |','|---|---:|---:|---:|---:|---|']
for op in ops:
 ratios={case: {label:cell(op,case)[a]/cell(op,case)[b] for label,a,b in [('base',candidate_default,'base-default'),('scalar',candidate_scalar,'cpp-scalar'),('default',candidate_default,'cpp-default')]} for case in cases}
 agg={label:{kind:gm([ratios[c][label] for c in cases if (c.startswith('real') if kind=='real' else not c.startswith('real'))]) for kind in ['real','corpus']} for label in ['base','scalar','default']}
 worst=max(v['base'] for v in ratios.values()); verdict='SHIPS' if agg['base']['real']<=.95 and worst<=1.01 else 'NO-GAIN'
 metrics[op]=dict(ratios=ratios,aggregate=agg,worst=worst,verdict=verdict)
 fields=[f"{agg[l]['real']:.4f}/{agg[l]['corpus']:.4f}" for l in ['base','scalar','default']]
 lines.append('| '+op+' | '+' | '.join(fields)+f' | {worst:.4f} | {verdict} |')
lines+=['','## Every case','','| Case | Function | Candidate / base default | Scalar / scalar C++ | Default / default C++ |','|---|---|---:|---:|---:|']
for case in cases:
 for op in ops:
  r=metrics[op]['ratios'][case];lines.append(f"| {case} | {op} | {r['base']:.4f} | {r['scalar']:.4f} | {r['default']:.4f} |")
lines+=['','## Wall time disclosure','','Resident-call seconds, median of three. Shared-machine observations, not the gate. Bound queries use frozen baseline meshlets (64 vertices, 124 triangles maximum), localized outside counts. Vertex/index inputs are shared across arms; output collection is included for every arm.','','| Case | Function | Base default s | Candidate default s | Candidate scalar s | C++ scalar s | C++ default s |','|---|---|---:|---:|---:|---:|---:|']
for case in cases:
 for op in ops:
  c=[r for r in rows if r['case']==case and r['op']==op]
  fields=[f"{statistics.median(r['seconds_per_call'] for r in c if r['arm']==a):.8f}" for a in ['base-default',candidate_default,candidate_scalar,'cpp-scalar','cpp-default']]
  lines.append('| '+case+' | '+op+' | '+' | '.join(fields)+' |')
(d/'TABLE.md').write_text('\n'.join(lines)+'\n')
(d/'results.json').write_text(json.dumps(dict(samples=len(rows),metrics=metrics),indent=2)+'\n')
print('\n'.join(lines[:10]))
