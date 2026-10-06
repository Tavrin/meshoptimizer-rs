#!/usr/bin/env python3
"""Pinned N/2N counters, output/source identity; never judge elapsed times."""
import os,json,struct,subprocess,hashlib,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];ART=Path(os.environ['MESHOPT_ARTIFACTS'])
sys.path.insert(0,str(ROOT/'parity/simd'));import qualify as q
sha=q.m.sha
phase=sys.argv[1];names=sys.argv[2:]
inputs={r['case']:r for r in json.loads((ART/'inputs.json').read_text())}
perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'
result={'sources':q.sources(),'controller_sha256':sha(__file__),'cpu':26,'method':'N/2N user counters subtraction; elapsed time ignored','rows':[]}
for name in names:
 b=bytearray((ART/inputs[name]['path']).read_bytes());count,stride=struct.unpack_from('<II',b,8);n=30000 if count<100 else 12 if count>100000 else 300
 for api in ['caller-buffer','allocating']:
  for backend in (['before','scalar','cpp'] if phase=='baseline' else ['after']):
   binary=ART/('bin/rust-simd' if backend=='after' else 'control-bin/'+{'before':'rust-simd','scalar':'rust-scalar','cpp':'cpp-simd'}[backend])
   runs=[]
   for repeats in [n,2*n]:
    probe=bytearray(b)
    if api=='caller-buffer':struct.pack_into('<I',probe,16,struct.unpack_from('<I',probe,16)[0]|128)
    struct.pack_into('<II',probe,32,1,repeats);framed=struct.pack('<I',len(probe))+probe
    label=f'{phase}-{name}-{api}-{backend}-{repeats}';sp=ART/(label+'.stat')
    cmd=['taskset','-c','26',perf,'stat','-e','instructions:u,cycles:u,branches:u,branch-misses:u','-x',',','-o',str(sp),'--',str(binary)]
    r=subprocess.run(cmd,input=framed,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert r.returncode==0,(label,r.stderr)
    out=r.stdout[4:];status,size,samples=struct.unpack_from('<iII',out,4);assert status==0
    counts={}
    for line in sp.read_text().splitlines():
     f=line.split(',')
     if len(f)>2 and f[2].endswith(':u'):
      assert float(f[3])>0 and float(f[4])>=99.9,f
      counts[f[2][:-2]]=int(f[0])
    runs.append({'repeats':repeats,'counts':counts,'command':cmd,'exit_code':r.returncode,'input_sha256':hashlib.sha256(framed).hexdigest(),'output_sha256':hashlib.sha256(out[16:16+size]).hexdigest(),'output_bytes':size,'stat_sha256':sha(sp)})
   assert runs[0]['output_sha256']==runs[1]['output_sha256']
   row={'case':name,'api':api,'backend':backend,'binary_sha256':sha(binary),'n':n,'decoded_bytes':count*stride,'runs':runs,'per_byte':{k:(runs[1]['counts'][k]-runs[0]['counts'][k])/(n*count*stride) for k in runs[0]['counts']}}
   result['rows'].append(row);q.save(ART/(phase+'-counters.json'),result);print(name,api,backend,{k:round(v,4) for k,v in row['per_byte'].items()},flush=True)
assert q.sources()==result['sources'];result['complete']=True;q.save(ART/(phase+'-counters.json'),result)
