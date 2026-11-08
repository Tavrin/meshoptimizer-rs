from pathlib import Path
import os,json,subprocess,hashlib
E=Path(os.environ['VG_EVIDENCE']);R=E/'shading';C=R/'campaign';D=R/'selected-validation';D.mkdir(exist_ok=False)
B=R/'vg-bench-final';selection=json.loads((R/'default-and-budget-selection.json').read_text());weight=selection['default_weight']
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(B)==json.loads((R/'selected-build/binary.json').read_text())['sha256']
for p,h in json.loads((R/'selected-build/source.json').read_text()).items():assert sha(Path(p))==h,p
cores=sorted(os.sched_getaffinity(0));cpu=str(cores[0]);rows=[];identities=[]
def run(name,cmd):
 with (D/(name+'.log')).open('w') as f:rc=subprocess.run(list(map(str,cmd)),stdout=f,stderr=subprocess.STDOUT).returncode
 with (D/'exits.jsonl').open('a') as f:f.write(json.dumps(dict(name=name,exit=rc,command=list(map(str,cmd))))+'\n')
 print(name,rc,flush=True)
 if rc:raise RuntimeError((name,rc))
 return [json.loads(x) for x in (D/(name+'.log')).read_text().splitlines() if x.startswith('{')]
for mesh in ['bunny','dead_tree_trunk_02','namaqualand_boulder_02','coast_rocks_01','dragon']:
 row=run('limits-'+mesh,['taskset','-c',cpu,B,'limits','ours',E/(mesh+'.vgm'),'--jobs',1,'--suggested'])[0]
 assert row['pass'],row
 assert row['max_bytes']>=2*row['peak_bytes'] and row['max_work']>=2*row['work'],('headroom',row)
 rows.append(row);(D/'limits.json').write_text(json.dumps(rows,indent=2)+'\n')
 file=D/(mesh+'.default.1T.bin');build=run('identity-'+mesh,['taskset','-c',cpu,B,'build','ours-v2',E/(mesh+'.vgm'),'--threads',1,'--out',file])[0]
 assert build['normal_error_weight']==weight
 before=C/f'{mesh}.normal-{str(weight).replace(".","p")}.1T.bin'
 assert sha(file)==sha(before),(mesh,'default identity')
 identities.append(dict(mesh=mesh,sha256=sha(file),same_as=before.name,default_weight=weight))
 (D/'identities.json').write_text(json.dumps(identities,indent=2)+'\n')
(D/'status.json').write_text(json.dumps(dict(exit=0,finite_helper_pass=True,twice_headroom=True,default_identity=True),indent=2)+'\n')
