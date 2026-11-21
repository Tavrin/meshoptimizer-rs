"""Single final milestone: counterbalanced timings and original quality campaign."""
import hashlib,json,os,subprocess
from pathlib import Path
E=Path(os.environ["VG_EVIDENCE"]); C=Path(os.environ['VG_CORPUS']); B=E/'candidate-vg-bench'; BASE=E/'baseline-vg-bench'
cores=sorted(os.sched_getaffinity(0)); one=str(cores[0]); many=','.join(map(str,cores[:8])); out=E/'final';out.mkdir(exist_ok=False)
meshes=['bunny','dead_tree_trunk_02','namaqualand_boulder_02','coast_rocks_01','dragon']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(name,cmd):
 with (out/(name+'.log')).open('w') as f:p=subprocess.run(list(map(str,cmd)),stdout=f,stderr=subprocess.STDOUT)
 with (out/'exits.jsonl').open('a') as f:f.write(json.dumps(dict(name=name,exit=p.returncode,cmd=list(map(str,cmd))))+'\n')
 if p.returncode:raise RuntimeError((name,p.returncode))
 return [json.loads(l) for l in (out/(name+'.log')).read_text().splitlines() if l.startswith('{')]
(out/'environment.json').write_text(json.dumps(dict(cores=cores,one=one,eight=many,binaries={p.name:sha(p) for p in [B,BASE]},reference='8cc060f01c3d4d5920781124cc8c32013bb47730',repeats=5,directions=64,samples=200000),indent=2))
for mesh in meshes:
 print('measure',mesh,flush=True)
 v=E/(mesh+'.vgm')
 if not v.exists():run('prep-'+mesh,[B,'prep',C/(mesh+('.ply' if mesh in ['bunny','dragon'] else '.gltf')),v])
 rows=[]
 for rep in range(1,6):
  for threads,cpu in [(1,one),(8,many)]:
   arms=[('ours',B,'ours-v2'),('baseline',BASE,'ours-v2'),('bevy',B,'bevy')]
   # Rotate three arms and reverse on alternate repeats.
   shift=(rep-1)%3;arms=arms[shift:]+arms[:shift]
   if rep%2==0:arms.reverse()
   for arm,b,policy in arms:
    artifact=out/(mesh+f'.{arm}.{threads}T.bin')
    cmd=['taskset','-c',cpu,'/usr/bin/time','-v',b,'build',policy,v,'--threads',threads]
    if rep==1:cmd+=['--out',artifact]
    for row in run(f'build-{mesh}-{arm}-{threads}T-{rep}',cmd):
     row.update(builder=arm,repeat=rep,rss_kib=row['vmhwm_kib'])
     if rep==1:row['artifact_sha256']=sha(artifact)
     rows.append(row)
 # Candidate/reference evaluation uses the same complete orbit and sampling as 0.2.19.
 for arm in ['ours','bevy']:
  extra=['--set','v2=1'] if arm=='ours' else ['--artifact',out/(mesh+'.bevy.8T.bin')]
  for row in run(f'eval-{mesh}-{arm}',['taskset','-c',many,B,'eval',arm,v,'--dirs','64','--jobs','8','--samples','200000',*extra]):
   row['builder']=arm;rows.append(row)
 for arm in ['ours','baseline']:
  assert sha(out/(mesh+f'.{arm}.1T.bin'))==sha(out/(mesh+f'.{arm}.8T.bin')),(mesh,arm,'thread identity')
 (out/(mesh+'.jsonl')).write_text(''.join(json.dumps(r)+'\n' for r in rows))
 print('complete',mesh,flush=True)
print('final milestone complete',flush=True)
