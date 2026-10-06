import os,json,struct,subprocess,hashlib
from pathlib import Path
ROOT=Path('/home/etienne/dev/meshopt-wt/p07'); ART=Path(os.environ['MESHOPT_ARTIFACTS']); OLD=ART.parent/'p07-fix6'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
import sys;sys.path.insert(0,str(ROOT/'parity/simd'));import qualify as q;build={'sources':q.sources()};phase=os.environ.get('FIX7_PHASE','baseline')
inputs={r['case']:r for r in json.loads((ART/'inputs.json').read_text())}
perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'
names=[]
for stride in [12,32]:
 source=(ART/'inputs'/f'varied-filter-3-streaming-s{stride}.input').read_bytes()[44:]
 for count in [17,129,4097,8193,32769,65537,262145]:
  name=f'exp-grid-{count}-s{stride}'
  probe=q.m.request(6,count,stride,source[:count*stride]);p=ART/(name+'.input');p.write_bytes(probe)
  inputs[name]={'path':p.name};names.append(name)

result={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'sources':build['sources'],'controller_sha256':sha(__file__),'cpu':26,'method':'N and 2N user-mode counter subtraction; no elapsed times assessed','rows':[]}
for name in names:
 b=bytearray((ART/inputs[name]['path']).read_bytes());count,stride=struct.unpack_from('<II',b,8);n=100000 if count<100 else 20 if count>100000 else 1000
 for api in ['caller-buffer','allocating']:
  for backend in ['manual','canonical']:
   binary=ART/'exp-all-trial'
   runs=[]
   for repeats in [n,2*n]:
    probe=bytearray(b)
    if api=='caller-buffer':struct.pack_into('<I',probe,16,struct.unpack_from('<I',probe,16)[0]|128)
    struct.pack_into('<I',probe,32,1);struct.pack_into('<I',probe,36,repeats)
    framed=struct.pack('<I',len(probe))+probe;label=f'{phase}-{name}-{api}-{backend}-{repeats}';(ART/(label+'.input')).write_bytes(framed)
    cmd=['taskset','-c','26',perf,'stat','-e','instructions:u,cycles:u,branches:u,branch-misses:u','-x',',','-o',str(ART/(label+'.stat')),'--',str(binary),*(['scalar'] if backend=='canonical' else [])]
    r=subprocess.run(cmd,input=framed,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert r.returncode==0,(label,r.stderr)
    (ART/(label+'.response')).write_bytes(r.stdout);out=r.stdout[4:];status,size,samples=struct.unpack_from('<iII',out,4);assert status==0
    counts={}
    for line in (ART/(label+'.stat')).read_text().splitlines():
     f=line.split(',')
     if len(f)>2 and f[2].endswith(':u'):
      assert float(f[3])>0 and float(f[4])>=99.9, f
      counts[f[2][:-2]]=int(f[0])
    runs.append({'repeats':repeats,'counts':counts,'command':cmd,'exit_code':r.returncode,'input_sha256':sha(ART/(label+'.input')),'output_sha256':hashlib.sha256(out[16:16+size]).hexdigest(),'output_bytes':size})
   assert runs[0]['output_sha256']==runs[1]['output_sha256']
   decoded=count*stride
   if name.startswith('meshlet'):
    tc,ts=struct.unpack_from('<II',b,24);decoded=(count+tc)*4 if name.startswith('meshlet-raw') else count*stride+tc*ts
   row={'case':name,'api':api,'backend':backend,'binary_sha256':sha(binary),'n':n,'decoded_bytes':decoded,'runs':runs,'per_byte':{k:(runs[1]['counts'][k]-runs[0]['counts'][k])/(n*decoded) for k in runs[0]['counts']}}
   result['rows'].append(row);(ART/(phase+'-counters.json')).write_text(json.dumps(result,indent=2)+'\n');print(name,api,backend,{k:round(v,3) for k,v in row['per_byte'].items()},flush=True)
# Color C++ is a different arithmetic contract; canonical after/before/scalar
# still must agree, including the terminal tail byte.
for name in names:
 for api in {r['api'] for r in result['rows'] if r['case']==name}:
  rows=[r for r in result['rows'] if r['case']==name and r['api']==api and r['backend']!='cpp']
  assert len({r['runs'][0]['output_sha256'] for r in rows})==1
assert all(sha(ROOT/k)==v for k,v in build['sources'].items())
result['complete']=True;(ART/(phase+'-counters.json')).write_text(json.dumps(result,indent=2)+'\n')
