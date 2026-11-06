"""Collect current receipts without changing the final verdict document."""
import hashlib,importlib.util,json,statistics
from pathlib import Path
A=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('vgr',A/'vg_report.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
exits=[json.loads(l) for l in (A/'exits.jsonl').read_text().splitlines()]
exact=[x for x in exits if x['name'].startswith('exact-') and not x['name'].startswith(('exact-lock-','exact-build-'))]
summary={'commands':len(exits),'failed_commands':[x for x in exits if x['exit']],'exact_driver_runs':len(exact),'exact_driver_failed':sum(x['exit']!=0 for x in exact)}
for n in ['determinism','release-results','campaign-failures','supplement-failures']:
 p=A/(n+'.json');summary[n]=json.loads(p.read_text()) if p.exists() else None
metrics=[];table=['# Final virtual-geometry qualification measurements','',
'Candidate v2 builder and MDAG 2 attribute payloads against the pinned reference. Five interleaved repeats; median build plus serialise milliseconds. Ratios below 1 favor the candidate.','',
'| Mesh | Build ratio 1T / 8T | File B/DAG tri ours / ref | Resident B/DAG tri ours / ref | Cut tris ratio at 5 / 8 px | Measured/claimed median ours / ref |',
'|---|---:|---:|---:|---:|---:|']
for p in sorted((A/'bench').glob('*.jsonl')):
 rows=[json.loads(l) for l in p.read_text().splitlines() if l.startswith('{')];m=p.stem
 builds=[x for x in rows if x['kind']=='build'];shapes={x['builder']:x for x in rows if x['kind']=='shape'}
 if not builds or set(shapes)!={'ours','bevy'}:continue
 ratios={}
 for t in [1,8]:
  times={b:statistics.median(x['build_ms']+x['serialise_ms'] for x in builds if x['builder']==b and x['threads']==t) for b in ['ours','bevy']}
  ratios[str(t)]=dict(ours=times['ours'],reference=times['bevy'],ratio=times['ours']/times['bevy'])
 cuts=[x for x in rows if x['kind']=='cut' and x['span_px']==-1 and x['threshold_px']>=1e-6]
 curves={b:sorted((x['p95_max_dev_px'],x['mean_triangles']) for x in cuts if x['builder']==b) for b in ['ours','bevy']}
 triangles={}
 for target in [5,8]:
  a,b=[r.interpolate(curves[arm],target) for arm in ['ours','bevy']];triangles[str(target)]=a/b if a and b else None
 claim={b:next(x['median_measured_over_claimed'] for x in cuts if x['builder']==b and x['threshold_px']==1) for b in ['ours','bevy']}
 per={k:{b:shapes[b][k]/shapes[b]['total_triangles'] for b in ['ours','bevy']} for k in ['file_bytes','resident_bytes']}
 hashes={x['artifact_sha256'] for x in builds if x['builder']=='ours' and x.get('artifact_sha256')}
 checks=[x for x in rows if x['kind']=='check'];bounds={
 'build_half_reference_1T':ratios['1']['ratio']<=.5,'build_half_reference_8T':ratios['8']['ratio']<=.5,
 'file_per_dag_triangle':per['file_bytes']['ours']<=per['file_bytes']['bevy'],
 'resident_per_dag_triangle':per['resident_bytes']['ours']<=per['resident_bytes']['bevy'],
 'fewer_cut_triangles_5px':triangles['5'] is not None and triangles['5']<1,
 'fewer_cut_triangles_8px':triangles['8'] is not None and triangles['8']<1,
 'measured_claimed_median_1_3':claim['ours']<=1.3,
 'self_checks':bool(checks) and all(x['pass'] for x in checks),
 'thread_byte_identity':len(hashes)==1 and sum(x.get('artifact_sha256') is not None for x in builds if x['builder']=='ours')==2}
 metrics.append(dict(mesh=m,build=ratios,bytes=per,triangles=triangles,claimed=claim,bars=bounds,hashes=sorted(hashes),self_checks=len(checks)))
 fmt=lambda v:'n/a' if v is None else f'{v:.3f}'
 table.append(f"| {m} | {ratios['1']['ratio']:.3f} / {ratios['8']['ratio']:.3f} | {per['file_bytes']['ours']:.2f} / {per['file_bytes']['bevy']:.2f} | {per['resident_bytes']['ours']:.2f} / {per['resident_bytes']['bevy']:.2f} | {fmt(triangles['5'])} / {fmt(triangles['8'])} | {claim['ours']:.3f} / {claim['bevy']:.3f} |")
summary['bench']=metrics
p=A/'cpp-observations.json'
summary['cpp_differentials']=json.loads(p.read_text()) if p.exists() else None
(A/'qualification-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(A/'VG_TABLE.md').write_text('\n'.join(table)+'\n')
print(json.dumps({'commands':len(exits),'failures':len(summary['failed_commands']),'exact_runs':len(exact),'bench_meshes':len(metrics),'bench_bar_passes':{k:sum(x['bars'][k] for x in metrics) for k in metrics[0]['bars']} if metrics else {}}))
