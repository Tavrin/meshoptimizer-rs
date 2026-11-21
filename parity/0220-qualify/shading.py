"""Fresh two-weight shading rows using the benchmark binary and prepared meshes from bench.py."""
import run as q
import json, os, statistics
b=q.T/'bin/vg-bench';e=q.OUT/'shading';e.mkdir(parents=True,exist_ok=True)
cores=sorted(os.sched_getaffinity(0));cpus={1:str(cores[0]),8:','.join(map(str,cores[:8]))}
rows=[]
for mesh in ['bunny','dead_tree_trunk_02','namaqualand_boulder_02','coast_rocks_01','dragon']:
 builds=[]
 for rep in range(1,6):
  for threads in [1,8]:
   for weight in ([0.,.25] if rep%2 else [.25,0.]):
    artifact=q.T/f'{mesh}.shade-{weight}.{threads}T.bin'
    cmd=['taskset','-c',cpus[threads],b,'build','ours-v2',q.T/(mesh+'.vgm'),'--threads',str(threads),'--normal-error-weight',str(weight)]
    if rep==1:cmd+=['--out',artifact]
    log=q.run(f'shade-build-{mesh}-{weight}-{threads}T-{rep}',cmd,estimate=1800)
    row=next(json.loads(l) for l in log.read_text().splitlines() if l.startswith('{') and json.loads(l)['kind']=='build')
    assert row['normal_error_weight']==weight and not row['duplicate_normals'],(mesh,weight,row)
    row.update(repeat=rep,weight=weight)
    if rep==1:row['artifact_sha256']=q.sha(artifact)
    builds.append(row)
 for w in [0.,.25]:
  assert q.sha(q.T/f'{mesh}.shade-{w}.1T.bin')==q.sha(q.T/f'{mesh}.shade-{w}.8T.bin')
 assert q.sha(q.T/(mesh+'.ours-v2.bin'))==q.sha(q.T/f'{mesh}.shade-0.0.8T.bin'),(mesh,'default versus explicit zero')
 log=q.run('shade-eval-'+mesh,['taskset','-c',cpus[8],b,'shade-eval',q.T/(mesh+'.vgm'),*[q.T/f'{mesh}.shade-{w}.8T.bin' for w in [0.,.25]],'--samples','16384','--dirs','16','--jobs','8','--with-tangents','0'],estimate=7200)
 quality=[json.loads(l) for l in log.read_text().splitlines() if l.startswith('{')]
 assert len(quality)==2 and all(x['cut_oracle_pass'] for x in quality)
 for w,qual in zip([0.,.25],quality):
  times={str(t):statistics.median(x['build_ms']+x['serialise_ms'] for x in builds if x['weight']==w and x['threads']==t) for t in [1,8]}
  rows.append(dict(mesh=mesh,weight=w,quality=qual,timing=times))
 q.write(e/(mesh+'-builds.json'),builds);q.write(e/'core.json',rows)
q.write(e/'protocol.json',dict(binary=q.sha(b),repeats=5,weights=[0.,.25],directions=16,samples_each_direction=16384,spans=[64,128,256,512,1024,2048],threshold_px=1,cpus=cpus,worker_identity=True))
