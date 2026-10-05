#!/usr/bin/env python3
"""Bind counters, frozen output identity, paired RFC results and explicit limits."""
import hashlib,json,math,os,statistics,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];ART=Path(os.environ['MESHOPT_ARTIFACTS'])
sys.path.insert(0,str(ROOT/'parity/simd'));import qualify as q
sha=q.m.sha
read=lambda n:json.loads((ART/(n+'.json')).read_text())
for n,h in read('control-build')['binaries'].items():assert sha(ART/'control-bin'/n)==h
for n,h in read('control-build')['sources'].items():
 assert hashlib.sha256(subprocess.check_output(['git','show','3f58dda:'+n],cwd=ROOT)).hexdigest()==h,n
for n,h in read('control-node-build')['binaries'].items():assert sha(ART/'control-node-bin'/n)==h
for n,h in read('control-node-build')['sources'].items():assert hashlib.sha256(subprocess.check_output(['git','show','3f58dda:'+n],cwd=ROOT)).hexdigest()==h,n
build=read('build');assert q.sources()==build['sources']
for n,h in build['binaries'].items():assert sha(ART/'bin'/n)==h
identity=read('all-identity');assert identity['sources']==build['sources'] and identity['binaries']==build['binaries'] and identity['passed']
node=read('node-build');assert node['sources']==build['sources']
for n,h in node['binaries'].items():assert sha(ART/'node-bin'/n)==h
assert read('node-preflight')['passed']
np=read('node-preflight-binding');assert np['sources']==build['sources'] and np['binaries']==node['binaries'] and np['preflight_sha256']==sha(ART/'node-preflight.json')
wasm_effect=read('wasm-counter-effect');assert wasm_effect['passed'] and wasm_effect['sources']==build['sources']
assert wasm_effect['selected_wasm_sha256']==node['binaries']['arithmetic-simd.wasm']
for n,h in {**wasm_effect['controllers'],**wasm_effect['evidence']}.items():assert sha(ART/n)==h
for row in read('checks'):
 assert row['exit_code']==0 and row['sources']==build['sources'] and sha(ART/row['log'])==row['log_sha256']
effect=read('counter-effect');assert effect['passed'] and effect['sources']==build['sources']
assert read('baseline-counters')['sources']==read('control-build')['sources']
counters=read('final-counters');assert counters['complete'] and counters['sources']==build['sources']
for r in counters['rows']:
 assert r['binary_sha256']==build['binaries']['rust-simd']
 for run in r['runs']:assert run['exit_code']==0
perf=read('performance');assert perf['complete'] and perf['sources']==build['sources'] and perf['binaries']==build['binaries']
names={n for n,b in q.corpus() if q.family(n) in ['exp','oct','view-filtered']}
assert {(r['case'],r['api']) for r in perf['rows']}=={(n,a) for n in names for a in ['allocating','caller-buffer']}
assert len(perf['rows'])==2*len(names)
for burst in perf['lease_bursts']:assert burst['wall_seconds']<840
for seg in perf['controller_segments']:
 assert sha(ROOT/seg['source'])==seg['sha256']
 assert sha(ART/seg['executed_controller_path'])==seg['executed_controller_sha256']
 assert seg['control_binaries']==read('control-build')['binaries']
summary={'verified':True,'native':{},'wasm':{},'s3_failures':[],'bar':'RFC geomean <=1.25, final 95% maximum upper <=1.50; no significant applicable S3 slowdown','limits':'Native Exp/Oct/filtered-view and WASM vertex/views final epoch. Native allocating vertex remains unresolved after fresh counter/asm analysis; prior verdict only. No release/integration/other-platform claim.'}
for r in perf['rows']:
 raw=r['raw_seconds'];n=len(raw['rust']);assert 5<=n<=20
 ci=q.interval([x/y for x,y in zip(raw['rust'],raw['simd'])])
 assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(ci,r.get('stage1_interval',r['interval'])))
 borderline=ci[0]<=1.5<ci[1];assert ('stage2' in r)==borderline
 for i in range(5,n):
  c=q.interval([x/y for x,y in zip(raw['rust'][:i],raw['simd'][:i])]);assert c[0]<=1.5<c[1]
 final=r.get('stage2',r)
 if 'stage2' in r:assert all(len(v)==30 for v in final['raw_seconds'].values())
 vals=final['raw_seconds'];ci=q.interval([x/y for x,y in zip(vals['rust'],vals['simd'])])
 assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(ci,r['interval']))
 oldci=q.interval([x/y for x,y in zip(vals['rust'],vals['old'])]);assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(oldci,r['paired_new_old_interval']))
 assert math.isclose(r['time_ratio'],statistics.median(raw['rust'])/statistics.median(raw['simd']),rel_tol=1e-12)
 assert math.isclose(r['paired_new_old_ratio'],statistics.median(x/y for x,y in zip(raw['rust'],raw['old'])),rel_tol=1e-12)
 for stage in [r]+([r['stage2']] if 'stage2' in r else []):
  assert len(stage['telemetry'])==len(stage['raw_seconds']['rust'])
  for t in stage['telemetry']:
   assert set(t['order'])==set(stage['raw_seconds'])
   for side in ['before','after']:
    a=t[side];assert a['admitted'] and a['queue_receipt'] and not a['measuring'] and a['declared_gb']==4 and a['reserved_gb']>=1 and a['memory_cap_gb']>=4
 if r['s3_applicable']:
  c=q.interval([x/y for x,y in zip(vals['rust'],vals['scalar'])]);assert r['s3']['rust']['interval']==c
  if c[0]>1:summary['s3_failures'].append({'api':r['api'],'case':r['case'],'interval':c})
for family in ['exp','oct','view-filtered']:
 summary['native'][family]={}
 for api in ['allocating','caller-buffer']:
  rows=[r for r in perf['rows'] if q.family(r['case'])==family and r['api']==api]
  gm=math.exp(statistics.mean(math.log(r['time_ratio']) for r in rows));paired=math.exp(statistics.mean(math.log(r['paired_new_old_ratio']) for r in rows))
  failed=[{'case':r['case'],'interval':r['interval']} for r in rows if r['interval'][1]>1.5]
  summary['native'][family][api]={'geomean':gm,'paired_new_old':paired,'max_upper':max(r['interval'][1] for r in rows),'failed_maxima':failed,'rfc_pass':gm<=1.25 and not failed}
wp=read('wasm-performance');assert wp['complete'] and wp['sources']==build['sources'] and wp['binaries']==build['binaries'] and wp['timing_binaries']==node['binaries']
wn={n for n,b in q.corpus() if struct.unpack_from('<I',b,4)[0] in [1,7]}
assert {(r['case'],r['api']) for r in wp['rows']}=={(n,a) for n in wn for a in ['allocating','caller-buffer']}
for burst in wp['lease_bursts']:assert burst['wall_seconds']<840
for seg in wp['controller_segments']:
 assert sha(ROOT/seg['source'])==seg['sha256'] and sha(ART/seg['executed_controller_path'])==seg['executed_controller_sha256']
 assert seg['control_binaries']==read('control-node-build')['binaries']
for r in wp['rows']:
 raw=r['raw_seconds'];n=len(raw['rust']);assert 5<=n<=20
 ci=q.interval([x/y for x,y in zip(raw['rust'],raw['cpp'])])
 assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(ci,r.get('stage1_interval',r['interval'])))
 assert ('stage2' in r)==(ci[0]<=1.5<ci[1])
 for i in range(5,n):
  c=q.interval([x/y for x,y in zip(raw['rust'][:i],raw['cpp'][:i])]);assert c[0]<=1.5<c[1]
 final=r.get('stage2',r)
 if 'stage2' in r:assert all(len(v)==30 for v in final['raw_seconds'].values())
 vals=final['raw_seconds'];ci=q.interval([x/y for x,y in zip(vals['rust'],vals['cpp'])])
 assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(ci,r['interval']))
 oldci=q.interval([x/y for x,y in zip(vals['rust'],vals['old'])]);assert all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(oldci,r['paired_new_old_interval']))
 assert math.isclose(r['time_ratio'],statistics.median(raw['rust'])/statistics.median(raw['cpp']),rel_tol=1e-12)
 assert math.isclose(r['paired_new_old_ratio'],statistics.median(x/y for x,y in zip(raw['rust'],raw['old'])),rel_tol=1e-12)
 for stage in [r]+([r['stage2']] if 'stage2' in r else []):
  assert len(stage['telemetry'])==len(stage['raw_seconds']['rust'])
  for t in stage['telemetry']:
   assert set(t['order'])==set(stage['raw_seconds'])
   for side in ['before','after']:
    a=t[side];assert a['admitted'] and a['queue_receipt'] and not a['measuring'] and a['declared_gb']==4 and a['reserved_gb']>=1 and a['memory_cap_gb']>=4
for family in ['vertex','view-none','view-filtered']:
 summary['wasm'][family]={}
 for api in ['allocating','caller-buffer']:
  rows=[r for r in wp['rows'] if q.family(r['case'])==family and r['api']==api]
  gm=math.exp(statistics.mean(math.log(r['time_ratio']) for r in rows));paired=math.exp(statistics.mean(math.log(r['paired_new_old_ratio']) for r in rows))
  failed=[{'case':r['case'],'interval':r['interval']} for r in rows if r['interval'][1]>1.5]
  summary['wasm'][family][api]={'geomean':gm,'paired_new_old':paired,'max_upper':max(r['interval'][1] for r in rows),'failed_maxima':failed,'rfc_pass':gm<=1.25 and not failed}
summary['scoped_pass']=all(r['rfc_pass'] for platform in ['native','wasm'] for apis in summary[platform].values() for r in apis.values()) and not summary['s3_failures']
summary['complete_rfc_pass']=False  # Native vertex A retains its unresolved prior failed maximum.
q.save(ART/'summary.json',summary);print(json.dumps(summary,indent=2))
