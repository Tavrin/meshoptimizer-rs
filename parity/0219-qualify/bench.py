"""Final v2/reference benchmark, using the retained harness and frozen corpus."""
import importlib.util,json,os,shutil,traceback
from pathlib import Path
spec=importlib.util.spec_from_file_location('qual',Path(__file__).with_name('run.py'));q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)
e=q.A/'bench';e.mkdir(exist_ok=True);src=Path(os.environ['VG_CORPUS']);d=q.T/'vg-harness'
shutil.copytree(q.R/'parity/vg-bench',d,ignore=shutil.ignore_patterns('target'),dirs_exist_ok=True)
p=d/'Cargo.toml';p.write_text(p.read_text().replace('path = "../.."','path = '+json.dumps(str(q.R))))
q.run('vg-corpus-check',['python3',q.A/'corpus_check.py'])
q.run('vg-lock',['cargo','update','--offline','--package','meshoptimizer-rs','--manifest-path',p],estimate=60)
q.run('vg-build',['cargo','build','--offline','--locked','--release','--features','bevy','--manifest-path',p],estimate=3600)
q.run('vg-reference-pin',['python3',q.A/'reference_pin.py'])
shutil.copy2(d/'Cargo.lock',e/'Cargo.lock')
b=q.T/'release/vg-bench';shutil.copy2(b,q.T/'bin/vg-bench');q.write(e/'binding.json',dict(binary=q.sha(b),sources={str(p.relative_to(d)):q.sha(p) for p in (d/'src').rglob('*') if p.is_file()},lock=q.sha(d/'Cargo.lock'),manifest=q.clean((d/'Cargo.toml').read_text())))
cores=sorted(os.sched_getaffinity(0));one=str(cores[0]);many=','.join(map(str,cores[:8]));fail=[]
q.write(e/'environment.json',dict(affinity=cores,one_core=one,eight_cores=many,repeats=5,directions=64,samples=200000,threads=[1,8],reference_revision='8cc060f01c3d4d5920781124cc8c32013bb47730'))
for mesh in ['bunny','dead_tree_trunk_02','namaqualand_boulder_02','coast_rocks_01','dragon']:
 try:
  inp=src/(mesh+('.ply' if mesh in ['bunny','dragon'] else '.gltf'));vgm=q.T/(mesh+'.vgm')
  q.write(e/(mesh+'-input.json'),dict(mesh=mesh,files={p.name:q.sha(p) for p in [inp,*([src/(mesh+'.bin')] if inp.suffix=='.gltf' else [])]}))
  q.run('vg-prep-'+mesh,[b,'prep',inp,vgm])
  out=e/(mesh+'.jsonl');out.write_text('')
  for rep in range(1,6):
   for threads,cpu in [(1,one),(8,many)]:
    for arm in (['ours-v2','bevy'] if rep%2 else ['bevy','ours-v2']):
     artifact=q.T/(mesh+'.'+arm+'.bin');cmd=['taskset','-c',cpu,'/usr/bin/time','-v',b,'build',arm,vgm,'--threads',str(threads)]
     if rep==1:cmd+=['--out',artifact]
     log=q.run(f'vg-build-{mesh}-{arm}-{threads}-{rep}',cmd,estimate=1800)
     for l in log.read_text().splitlines():
      if l.startswith('{'):
       row=json.loads(l);row.update(repeat=rep,rss_kib=row['vmhwm_kib']);row['builder']='ours' if arm=='ours-v2' else arm;row['library_policy']='v2' if arm=='ours-v2' else 'reference';row['artifact_sha256']=q.sha(artifact) if rep==1 else None;out.open('a').write(json.dumps(row)+'\n')
  for arm in ['ours','bevy']:
   extra=['--set','v2=1'] if arm=='ours' else ['--artifact',q.T/(mesh+'.bevy.bin')]
   log=q.run('vg-eval-'+mesh+'-'+arm,['taskset','-c',many,b,'eval',arm,vgm,'--dirs','64','--jobs','8','--samples','200000',*extra],estimate=7200)
   for l in log.read_text().splitlines():
    if l.startswith('{'):
     row=json.loads(l);row['builder']=arm;row['library_policy']='v2' if arm=='ours' else 'reference';out.open('a').write(json.dumps(row)+'\n')
 except BaseException as x:
  fail.append([mesh,q.clean(repr(x))]);(e/(mesh+'-failure.log')).write_text(q.clean(traceback.format_exc()));print('VG FAILED',mesh,repr(x),flush=True)
 q.write(e/'failures.json',fail)
q.run('vg-materials',[b,'materials',src/'flighthelmet.gltf'],estimate=1800,check=False)
q.run('vg-report',['python3','-I',q.A/'vg_report.py',*sorted(e.glob('*.jsonl'))],estimate=60,check=False)
print('VG COMPLETE',fail,flush=True)
