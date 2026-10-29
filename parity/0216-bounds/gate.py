"""Recheck retained samples and emit the per-function default-build gate."""
import hashlib,json,math,os,statistics,sys
from pathlib import Path
s=Path(os.environ['BOUNDS_SCRATCH']);d=s/sys.argv[1]
campaign=json.loads((d/'campaign.json').read_text());arms=campaign['arms']
candidate=next(a for a in arms if a.startswith('candidate') and a.endswith('-default'))
rows=[json.loads(x) for x in (d/'counts.jsonl').read_text().splitlines()]
assert len(rows)==3*len(arms)*len(campaign['ops'])*len(campaign['inputs'])
assert len({(r['case'],r['op'],r['arm'],r['rep']) for r in rows})==len(rows)
for arm in arms:
 with (s/f'driver-{arm}').open('rb') as f: assert hashlib.file_digest(f,'sha256').hexdigest()==campaign['binaries'][arm]
for r in rows:
 name='-'.join(str(r[k]) for k in ['case','op','arm','rep'])
 fields=next(l.split(';') for l in (d/'samples'/f'{name}.perf').read_text().splitlines() if ';instructions:u;' in l)
 assert int(fields[0])==r['instructions'] and float(fields[4])==100.
result={}
for op in campaign['ops']:
 ratios={}
 for item in campaign['inputs']:
  case=item['case'];cell=[r for r in rows if r['case']==case and r['op']==op]
  assert len(cell)==len(arms)*3 and len({r['output_sha256'] for r in cell})==1
  med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in arms}
  ratios[case]=med[candidate]/med['base-default']
 agg={kind:math.exp(statistics.mean(math.log(v) for c,v in ratios.items() if (c.startswith('real') if kind=='real' else not c.startswith('real')))) for kind in ['real','corpus']}
 worst=max(ratios.values());verdict='SHIPS' if agg['real']<=.95 and worst<=1.01 else 'NO-GAIN'
 result[op]=dict(real_gm=agg['real'],corpus_gm=agg['corpus'],worst=worst,verdict=verdict,ratios=ratios)
 print(op,f"real={agg['real']:.6f} corpus={agg['corpus']:.6f} worst={worst:.6f}",verdict,flush=True)
(d/'gate.json').write_text(json.dumps(dict(samples=len(rows),candidate=candidate,metrics=result),indent=2)+'\n')
