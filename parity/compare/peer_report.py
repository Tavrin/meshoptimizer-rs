#!/usr/bin/env python3
"""Registered family estimates and literal verdicts; no measurement or retuning."""
from pathlib import Path
import os,json,math,statistics,collections,hashlib
import peer_screen as p
ART=p.ART;ROOT=p.ROOT

def estimate(rows):
 assert rows and all(r.get('completed') for r in rows)
 logs=[[math.log(s['ratio'])for s in r['samples']]for r in rows]
 means=[statistics.mean(v)for v in logs];mu=statistics.mean(means)
 se=math.sqrt(sum(statistics.variance(v)/len(v)for v in logs))/len(logs)
 t=2.776445 if any(len(v)==5 for v in logs)else 2.093024
 return {'gm':math.exp(mu),'ci95':[math.exp(mu-t*se),math.exp(mu+t*se)],'max':max(math.exp(v)for v in means),'cases':len(rows),'pair_counts':[len(v)for v in logs]}
def verdict(x,F,equivalent):
 if not equivalent:return 'NOT EQUIVALENT'
 if x['ci95'][1]<1-F-0.02:return 'PEER FASTER'
 if x['ci95'][0]>1+F+0.02:return 'OURS FASTER'
 return 'EQUAL WITHIN NOISE'
def report():
 checks=json.loads((ART/'correctness.json').read_text())if(ART/'correctness.json').exists()else[]
 floors=[]
 for path in sorted(ART.glob('timing-*-AA.json')):
  rows=json.loads(path.read_text())
  for op in p.DENSE:
   selected=[r for r in rows if r['operation']==op and r.get('completed')]
   if len(selected)==5:floors.append(dict(stream=path.name,operation=op,**estimate(selected)))
 F=max((abs(x['gm']-1)for x in floors),default=None)
 # One registered Moss SIMD A/A set, over exactly three dense families.
 floor_complete=len(floors)==3 and {x['operation']for x in floors}==set(p.DENSE) and all(x['stream']=='timing-moss-simd-AA.json'for x in floors)
 records=[];unmeasured=[]
 for profile in ['defaults','moss']:
  for mode in ['scalar','simd']:
   for arm in ['M7','X']:
    path=ART/f'timing-{profile}-{mode}-{arm}.json';rows=json.loads(path.read_text())if path.exists()else[]
    for real in [False,True]:
     for op in (['simplify','overdraw']if real else p.OPS):
      names=[f'real-{s}'for s,_ in p.REAL]if real else[n for n,_,_ in p.m.CASES]
      selected=[r for r in rows if r['case']in names and r['operation']==op and r.get('completed')]
      labels=[r for r in checks if r['key'][0:2]==[profile,mode]and r['key'][2]in names and r['key'][3]==op and r['key'][4]in ['O',arm]]
      base={'peer':arm,'profile':profile,'mode':mode,'family':('real-'if real else'')+op,'labels':labels}
      if len(selected)!=len(names):
       reasons=[{'case':r['key'][2],'arm':r['key'][4],'status':r['status'],'reason':r.get('reason','')}for r in labels if r['status']in['invalid','UNAVAILABLE','unmeasured']]
       base.update(verdict='UNMEASURED',reason=reasons or 'no complete registered timing set',measured_cases=len(selected),expected_cases=len(names));unmeasured.append(base)
      else:
       x=estimate(selected);equivalent=all(r['status']=='identical'for r in labels)and len(labels)==2*len(names)
       base.update(**x,equivalent=equivalent,verdict=verdict(x,F,equivalent)if floor_complete else'UNMEASURED: A/A floor incomplete')
       if not equivalent:
        base['output_metrics']=[{k:r[k]for k in ['key','status','triangles','reported_error','oracle_triangles','oracle_reported_error','meshlets']if k in r}for r in labels]
       records.append(base)
 # Browser families are attributes and triangle decoding across frozen streams.
 for profile in ['defaults','moss']:
  path=ART/f'browser-{profile}/data/chromium-timing.json'
  data=json.loads(path.read_text())if path.exists()else{}
  for mode in ['ATTRIBUTES','TRIANGLES']:
   rows=[{'samples':r['peers']['X']['samples'],'completed':True}for r in data.get('rows',[])if r['mode']==mode and r.get('peers',{}).get('X',{}).get('completed')]
   expected=21
   base={'peer':'X','profile':profile,'mode':'wasm-simd128','family':'browser-'+mode.lower()}
   if len(rows)==expected:
    x=estimate(rows);base.update(**x,equivalent=True,verdict=verdict(x,F,True)if floor_complete else'UNMEASURED: A/A floor incomplete');records.append(base)
   else:base.update(verdict='UNMEASURED',reason='no complete 21-stream registered Chromium timing set',measured_cases=len(rows));unmeasured.append(base)
 unavailable=[{'peer':a,'family':'cluster-LOD','verdict':'UNAVAILABLE','reason':'UNAVAILABLE by registered scope: spec excludes M7 cluster LOD; its published build does contain clusterlod bindings, which are not tested here'if a=='M7'else'published optimesh 1.1.0 archive exposes no public clusterlod module; README describes repository demo, absent from the published archive'}for a in ['M7','X']]
 faster=[r for r in records if r['peer']=='X'and r['family'].startswith(('browser-','cluster-LOD'))and r['verdict']=='PEER FASTER']
 owner=[]
 for r in records:
  if r['peer']!='X'or r['verdict']!='NOT EQUIVALENT'or not floor_complete or r['ci95'][1]>=1-F-0.02:continue
  labels=r['labels'];valid=True
  for name in set(x['key'][2]for x in labels):
   a=next(x for x in labels if x['key'][2]==name and x['key'][4]=='O');b=next(x for x in labels if x['key'][2]==name and x['key'][4]=='X')
   if not all(k in a and k in b for k in ['triangles','reported_error'])or a['triangles']!=b['triangles']or b['reported_error']>a['reported_error']:valid=False
  if valid:owner.append({k:r[k]for k in ['profile','mode','family','gm','ci95']})
 answer='YES'if faster or owner else'NOT ESTABLISHED'if any(r['family'].startswith('browser-')for r in unmeasured)else'NO'
 result={'schema':1,'campaign':str(ART),'F':F,'floor_complete':floor_complete,'floor_families':floors,'records':records,'unmeasured':unmeasured,'unavailable':unavailable,'beats_ours_anywhere':answer,'owner_review':owner,'qualifying_faster':[{'profile':r['profile'],'family':r['family'],'gm':r['gm'],'ci95':r['ci95']}for r in faster]}
 (ART/'results.json').write_text(json.dumps(result,indent=2)+'\n')
 lines=['# Peer screen — meshoptimizer-rs 0.2.4, meshopt 0.7.0, optimesh 1.1.0','',f'Campaign: `{ART}`. Draft PR #12. Safety: PASS under the 2026-10-25 amendment; first review reused with identical dependency tree and reviewed-source hashes.', '',f'**Optimesh beats ours anywhere (registered rule): {answer}.** Cluster LOD is unavailable from its published public API. Browser decode and non-equivalent quality checks are assessed below. No general Rust-library ranking or Moss/product claim follows.', '',f'A/A floor F = {F!r}; complete = {floor_complete}. Thresholds: peer upper < 1-F-0.02; ours lower > 1+F+0.02. Other equivalent results are EQUAL WITHIN NOISE. Non-equivalent outputs keep that label regardless of speed.','', 'Ratios are peer/O; GM, nominal paired 95% interval, and max case. Five alternating pairs extend once to twenty iff the initial interval straddles equality. Family aggregation is registered in REGISTRATION.md. Shared host, selected physical CPU, warm workspace, allocation/copies/scratch/destruction included. No adjusted significance claim.', '', '## Correctness','']
 for arm in ['O','M7','X']:
  counts=collections.Counter(r['status']for r in checks if r['key'][4]==arm);lines.append(f'- {arm}: '+', '.join(f'{k}: {v}'for k,v in sorted(counts.items()))+'.')
 for profile in ['defaults','moss']:
  bp=ART/f'browser-{profile}/data/chromium-bytes.json'
  if bp.exists():
   bc=json.loads(bp.read_text());summary=collections.Counter((a,status)for r in bc['rows']for a,status in r['bytes'].items());lines.append(f'- Browser {profile}: '+', '.join(f'{a} {status}: {n}'for (a,status),n in sorted(summary.items()))+'.')
 lines+=['','Full per-case labels, hashes, triangle counts and reported errors: `correctness.json`. Scalar C++ 1.3 from meshopt 0.7.0 is the oracle; M7 SIMD filters can differ from canonical scalar bytes. Meshlet padding normalization is outside timing. A valid simplifier output does not establish equal geometric quality.','', '## Per-peer family table','','| Peer | Profile | Mode | Family | GM | 95% interval | Max case | Verdict | Output metrics |','|---|---|---|---|---:|---|---:|---|---|']
 for r in records:
  metrics=''
  if not r.get('equivalent'):
   parts=[]
   for x in r.get('output_metrics',[]):
    if x['key'][4]!=r['peer']:continue
    if 'reported_error'in x:parts.append(f"{x['key'][2]}: tris {x['triangles']}/{x['oracle_triangles']}, err {x['reported_error']:.6g}/{x['oracle_reported_error']:.6g}")
    elif 'meshlets'in x:parts.append(f"{x['key'][2]}: {x['meshlets']} meshlets, {x['triangles']} tris")
   metrics='; '.join(parts)or'valid; exact bytes differ (see per-case hashes)'
  lo,hi=r['ci95'];lines.append(f"| {r['peer']} | {r['profile']} | {r['mode']} | {r['family']} | {r['gm']:.6f} | [{lo:.6f}, {hi:.6f}] | {r['max']:.6f} | {r['verdict']} | {metrics} |")
 lines+=['','## Unavailable and unmeasured','']
 for r in unavailable+unmeasured:lines.append(f"- {r['peer']} / {r.get('profile','all')} / {r.get('mode','all')} / {r['family']}: {r['verdict']} — {r['reason']}.")
 lines+=['','## Owner review','',json.dumps(owner), '', '## Receipts and limits','', '`REGISTRATION.md`, `SAFETY.md`, `safety-reuse.json`, per-arm cargo trees, build/admission logs, retained native/Wasm binaries, `epoch.json`, `inputs.json`, `real-inputs.json`, `correctness.json`, `timing-*.json`, browser stream/reference hashes and `browser-progress.jsonl` are in the campaign directory. The old peer-screen and meshopt-cmp evidence is untouched.','', 'Real operation replays retain the full captured stride-64 vertex records and the exact first simplify/overdraw record parameters. The packed XYZ prefix is used only by independent validators. Each operation has a fresh resident workspace; this is not a cooker-chain replay. Browser reuses REPORT-3 inputs/transports and headless runner, with registered pairs and fresh scalar C++ reference bytes. O/X decode to allocating output, then copy to caller storage; upstream JS retains its shipped source-copy transport.','', 'Not verified: production/Moss acceptance, cluster-LOD performance, other hosts/architectures/browsers, geometric quality beyond recorded validity/error/count, full library qualification, or unmeasured rows. No library src/, README or Moss change.']
 text='\n'.join(lines)+'\n';(ART/'REPORT.md').write_text(text);(ROOT/'parity/COMPARE_PEERS_2026-10.md').write_text(text)
 compact={k:v for k,v in result.items()if k!='records'};compact['records']=[{k:v for k,v in r.items()if k!='labels'}for r in records];(ROOT/'parity/compare/peer-results.json').write_text(json.dumps(compact,indent=2)+'\n')
 print(json.dumps({'F':F,'floor_complete':floor_complete,'measured_families':len(records),'unmeasured':len(unmeasured),'beats_ours_anywhere':answer},indent=2))
if __name__=='__main__':report()
