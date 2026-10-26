from pathlib import Path
import os, subprocess, json, hashlib, filecmp
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3');D=S/'proof-output';D.mkdir(exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE')
records=[]
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def run(exe,args,label):
 dest=D/f'{label}.zst';cpu=D/f'{label}.cpu'
 assert not dest.exists(),dest
 with dest.open('wb') as out, (D/f'{label}.stderr').open('w') as err:
  p=subprocess.Popen(['/usr/bin/time','-f','%U %S','-o',str(cpu),str(S/exe),*args],stdout=subprocess.PIPE,stderr=err)
  z=subprocess.Popen(['zstd','-q','-T1','-3','-c'],stdin=p.stdout,stdout=out)
  p.stdout.close();rc=p.wait();zc=z.wait()
 assert rc==zc==0,(exe,rc,zc)
 seconds=sum(map(float,cpu.read_text().split()))
 observations=(D/f'{label}.stderr').read_text().strip()
 return dest,dict(exe=exe,args=args,label=label,exit=rc,compression_exit=zc,cpu_seconds=seconds,observations=observations,output_sha256=sha(dest),bytes=dest.stat().st_size)
def pair(args,label,mode='std',seeds=False):
 prefix='seeds' if seeds else 'exact'
 a,ar=run(f'{prefix}-base-{mode}',args,f'{label}-base-{mode}')
 b,br=run(f'{prefix}-candidate-{mode}',args,f'{label}-candidate-{mode}')
 # zstd uses a fixed deterministic stream configuration, so byte equality of
 # compressed files entails equality of the complete observations, not hashes.
 assert filecmp.cmp(a,b,shallow=False),(label,mode,'exact observation mismatch')
 records.append(dict(label=label,mode=mode,base=ar,candidate=br,exact_equal=True))
 (S/'proof-results.json').write_text(json.dumps(records,indent=2)+'\n')
 print(label,mode,'exact equal',br['cpu_seconds'],flush=True)
 return br['cpu_seconds']
for mode in ['std','nostd']:
 pair(['small'],'small',mode)
 pair([],'existing-fuzz-seeds',mode,seeds=True)
 files=[str(S/'meshopt-028-L2/inputs'/f'{n}.bin') for n in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']]
 pair(['frozen',*files],'frozen',mode)
# Count candidate generator CPU time, excluding compression, base, compilation,
# and all wall time. Widened mesh/layout generator must receive >=300 CPU sec.
cpu=0.;batch=0
while cpu<300.:
 mode=['std','nostd'][batch%2]
 cpu+=pair(['random',str(0x21000000+batch*104729),'64'],f'fuzz-{batch:05}',mode)
 batch+=1
(S/'fuzz-results.json').write_text(json.dumps(dict(candidate_cpu_seconds=cpu,batches=batch,cases=batch*64,exit=0,errors=0),indent=2)+'\n')
print('FUZZ COMPLETE',cpu,batch,flush=True)
