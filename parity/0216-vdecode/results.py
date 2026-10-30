"""Summarize medians without changing any admission threshold."""
import collections,json,math,os,statistics
from pathlib import Path
A=Path(os.environ['DECODE_ART']);phase=os.environ.get('DECODE_PHASE','screen')
rows=[json.loads(s) for s in (A/(phase+'-counts.jsonl')).read_text().splitlines()]
cells=collections.defaultdict(dict)
for r in rows:cells[(r['case'],r['op'])].setdefault(r['arm'],[]).append(r)
result=[]
for (case,op),arms in sorted(cells.items()):
 assert all(len(v)==3 for v in arms.values())
 ins={a:statistics.median(r['instructions_per_call'] for r in rs) for a,rs in arms.items()}
 wall={a:statistics.median(r['wall_seconds_per_call'] for r in rs) for a,rs in arms.items()}
 # Base/candidate successful outputs must agree, even on the diagnostic runs.
 hashes={r['output_sha256'] for a,rs in arms.items() for r in rs if a in ['base','control','unisolated','candidate']}
 assert len(hashes)==1,(case,op,hashes)
 if op in ['vertex-decode','index-decode']:assert len({r['output_sha256'] for rs in arms.values() for r in rs})==1,(case,op,'C++ output')
 result.append(dict(case=case,op=op,instructions=ins,wall_seconds=wall,ratios={a:v/ins['base'] for a,v in ins.items()},cpp_scalar=ins['candidate']/ins['cpp-scalar'] if 'cpp-scalar' in ins else None,cpp_default=ins['candidate']/ins['cpp-default'] if 'cpp-default' in ins else None))
def gm(v):return math.exp(statistics.mean(map(math.log,v)))
summary=[]
for op in sorted({x['op'] for x in result}):
 for group in ['real','corpus']:
  rs=[x for x in result if x['op']==op and (x['case'].startswith('real-') == (group=='real'))]
  if not rs:continue
  summary.append(dict(op=op,group=group,base=gm([r['ratios']['candidate'] for r in rs]),worst=max(r['ratios']['candidate'] for r in rs),cpp_scalar=gm([r['cpp_scalar'] for r in rs]) if all(r['cpp_scalar'] for r in rs) else None,cpp_default=gm([r['cpp_default'] for r in rs]) if all(r['cpp_default'] for r in rs) else None))
r=dict(cells=result,summary=summary)
(A/(phase+'-results.json')).write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(summary,indent=2))
lines=['# Instruction medians and wall disclosure','','| Case | Operation | Candidate/base | / scalar C++ | / default C++ | Base seconds | Candidate seconds |','|---|---|---:|---:|---:|---:|---:|']
for x in result:
 def f(v):return '-' if v is None else f'{v:.6f}'
 lines.append(f"| {x['case']} | {x['op']} | {f(x['ratios']['candidate'])} | {f(x['cpp_scalar'])} | {f(x['cpp_default'])} | {x['wall_seconds']['base']:.9f} | {x['wall_seconds']['candidate']:.9f} |")
(A/(phase+'-TABLE.md')).write_text('\n'.join(lines)+'\n')
