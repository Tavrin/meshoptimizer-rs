#!/usr/bin/env python3
"""Frozen input comparison; CPU-admitted, pinned, interleaved resident drivers."""
from pathlib import Path
import sys,os,json,subprocess,hashlib,time,struct,math,statistics,select,shutil,collections
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
sys.path.insert(0,str(ROOT/'parity'))
import runner,performance,quiet
PERF=['vertex-decode','vertex-encode','vertex-encode-v0','vertex-encode-default','index-decode','index-encode','sequence-decode','sequence-encode','oct-decode','quat-decode','exp-decode','color-decode','oct-encode','quat-encode','exp-encode','color-encode','cache','cache-strip','cache-fifo','overdraw','fetch','fetch-remap','simplify','stripify','unstripify','meshlets','meshlets-scan','meshlets-flex','meshlets-spatial']
EXTRA=['vertex-version','index-version','vertex-bound','index-bound','sequence-bound','strip-bound','unstrip-bound','meshlet-bound','remap','remap-multi','remap-custom','remap-vertices','remap-indices','shadow','shadow-multi','position-remap','adjacency','tessellation','provoking','simplify-attributes','simplify-sloppy','simplify-prune','simplify-points','simplify-update','analyze-cache','analyze-fetch','analyze-overdraw','analyze-coverage','cluster-bounds','sphere-bounds','meshlet-bounds','optimize-meshlet','partition','spatial-remap','spatial-triangles','spatial-points','quantize-unorm','quantize-snorm','quantize-half','dequantize-half','quantize-float','simplify-scale']
CASES=[('tiny-smooth',32,'smooth'),('medium-smooth',8192,'smooth'),('medium-seams',8192,'seam-heavy'),('medium-sparse',8192,'sparse'),('million-smooth',1000000,'smooth')]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def disk():
 if shutil.disk_usage(ART).free<25*1024**3:raise SystemExit('PAUSED: disk below 25 GiB')
def pack(p,i,buffers):return struct.pack('<II',len(p)//3,len(i))+p.tobytes()+i.tobytes()+b''.join(struct.pack('<I',len(b))+b for b in buffers)
def words(b):return struct.unpack('<'+'I'*(len(b)//4),b)
def triangles(i):return collections.Counter(min(tuple(i[k:k+3]),tuple(i[k+1:k+3])+tuple(i[k:k+1]),tuple(i[k+2:k+3])+tuple(i[k:k+2])) for k in range(0,len(i),3))
class Driver:
 def __init__(self,backend,profile,op,path,cpu=None,suffix=''):
  self.output=ART/'outputs'/f'{profile}-{path.stem}-{op}-{backend}{suffix}.bin';self.output.parent.mkdir(exist_ok=True)
  self.binary=ART/f'cmp-driver-{profile}-{backend}'
  cmd=[str(self.binary),op,str(path),str(self.output)]
  if cpu is not None:cmd=['taskset','-c',str(cpu)]+cmd
  self.p=subprocess.Popen(cmd,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
  if self.read()!='READY':raise ValueError('driver not ready')
 def read(self):
  if not select.select([self.p.stdout],[],[],180)[0]:raise TimeoutError('driver timeout')
  line=self.p.stdout.readline().strip()
  if not line:raise RuntimeError(self.p.stderr.read())
  return line
 def sample(self,n):self.p.stdin.write(str(n)+'\n');self.p.stdin.flush();return float(self.read())
 def close(self):
  if self.p.poll() is None:
   self.p.stdin.write('stop\n');self.p.stdin.flush();self.p.communicate(timeout=180)
  assert self.p.returncode==0,self.p.returncode
 def __enter__(self):return self
 def __exit__(self,*args):
  if args[0] and self.p.poll() is None:self.p.kill();self.p.wait()
  else:self.close()
def execute(backend,profile,op,path,suffix=''):
 with Driver(backend,profile,op,path,suffix=suffix) as d:
  out=d.output.read_bytes()
  if suffix=='-cross':d.output.unlink()
  return out
def prepare():
 inputs=ART/'inputs';inputs.mkdir(exist_ok=True)
 metadata=[]
 for name,nt,shape in CASES:
  p,i=performance.geometry(runner,nt,shape);b=[b'']*7;path=inputs/f'{name}.bin';path.write_bytes(pack(p,i,b))
  for k,op in enumerate(['vertex-encode','index-encode','sequence-encode','oct-encode','quat-encode','exp-encode','color-encode']):b[k]=execute('ours','defaults',op,path,suffix='-fixture')
  path.write_bytes(pack(p,i,b))
  strip=execute('ours','defaults','stripify',path,suffix='-fixture');sb=list(b);sb[2]=strip
  (inputs/f'{name}-strip.bin').write_bytes(pack(p,i,sb))
  metadata.append({'name':name,'triangles':nt,'shape':shape,'vertices':len(p)//3,'indices':len(i),'sha256':sha(path),'strip_sha256':sha(inputs/f'{name}-strip.bin'),'codec_sha256':[hashlib.sha256(x).hexdigest() for x in b]})
  print('frozen',name,flush=True)
 (ART/'inputs.json').write_text(json.dumps(metadata,indent=2)+'\n')
def unpack(path):
 b=path.read_bytes();n,ni=struct.unpack_from('<II',b);p=b[8:8+n*12];i=words(b[8+n*12:8+n*12+ni*4]);c=8+n*12+ni*4;buffers=[]
 for _ in range(7):l=struct.unpack_from('<I',b,c)[0];c+=4;buffers.append(b[c:c+l]);c+=l
 return n,p,i,buffers
# Independent semantic checks precede timing. No assert merely compares the two implementations.
def valid(op,out,path,backend,profile):
 n,p,i,buf=unpack(path);nt=len(i)//3
 if op.startswith('vertex-encode') or op in ['index-encode','sequence-encode']:
  k=0 if op.startswith('vertex') else 1 if op.startswith('index') else 2;bs=list(buf);bs[k]=out
  import array
  pp=array.array('f');pp.frombytes(p);ii=array.array('I',i)
  cross=ART/'inputs'/f'cross-{backend}-{op}.bin';cross.write_bytes(pack(pp,ii,bs))
  decode=['vertex-decode','index-decode','sequence-decode'][k]
  for other in ['ours','theirs']:
   decoded=execute(other,profile,decode,cross,suffix='-cross')
   if k==0:assert decoded==p
   elif k==1:assert triangles(words(decoded))==triangles(i)
   else:assert words(decoded)==i
  cross.unlink()
  return 'both decoders; exact vertices/sequence or oriented triangle multiset'
 if op=='vertex-decode':assert out==p;return 'lossless original bytes'
 if op=='index-decode':assert triangles(words(out))==triangles(i);return 'oriented triangle multiset'
 if op=='sequence-decode':assert words(out)==i;return 'lossless sequence'
 if op in ['cache','cache-strip','cache-fifo','overdraw','unstripify','spatial-triangles']:
  a=words(out);assert len(a)==len(i) and all(v<n for v in a) and triangles(a)==triangles(i);return 'index-valid; oriented triangle multiset'
 if op=='stripify':
  import array
  pp=array.array('f');pp.frombytes(p);ii=array.array('I',i);bs=list(buf);bs[2]=out;cross=ART/'inputs'/f'cross-{backend}-strip.bin';cross.write_bytes(pack(pp,ii,bs))
  for other in ['ours','theirs']:assert triangles(words(execute(other,profile,'unstripify',cross,suffix='-cross')))==triangles(i)
  cross.unlink()
  return 'both unstripifiers; oriented triangle multiset'
 if op=='fetch':
  ni=struct.unpack_from('<I',out)[0];a=words(out[4:4+ni*4]);verts=out[4+ni*4:];assert ni==len(i) and len(verts)%12==0 and all(v<len(verts)//12 for v in a)
  assert b''.join(verts[v*12:v*12+12] for v in a)==b''.join(p[v*12:v*12+12] for v in i);return 'vertex bytes reconstruct original indexed stream'
 if op.startswith('meshlets'):
  count,nv=struct.unpack_from('<II',out);a=words(out[8:8+count*16]);v=words(out[8+count*16:8+count*16+nv*4]);t=out[8+count*16+nv*4:];flat=[]
  for k in range(count):
   vo,to,vc,tc=a[k*4:k*4+4];assert 0<vc<=64 and 0<tc<=124 and vo+vc<=nv and to+tc*3<=len(t)
   for j in t[to:to+tc*3]:assert j<vc;flat.append(v[vo+j])
  assert all(j<n for j in flat) and triangles(flat)==triangles(i);return 'descriptor bounds; local/global indices; oriented triangle multiset'
 if op in ['simplify','simplify-attributes','simplify-sloppy']:
  a=words(out[:-4]);e=struct.unpack('<f',out[-4:])[0];assert len(a)%3==0 and len(a)<=len(i) and all(v<n for v in a) and math.isfinite(e) and e>=0;return 'index/count-valid; finite reported error (no quality equivalence claim)'
 if op=='simplify-update':
  a=words(out);ni=a[0];assert ni%3==0 and ni<=len(i) and all(v<n for v in a[1:ni+1]) and len(a)==ni+2+n*3
  assert all(math.isfinite(v) for v in struct.unpack('<'+'f'*(n*3+1),out[(ni+1)*4:]));return 'index/count-valid; finite mutated geometry/error'
 if op=='simplify-prune':a=words(out);assert len(a)%3==0 and all(v<n for v in a) and not (triangles(a)-triangles(i));return 'valid subset of original oriented triangles'
 if op=='simplify-points':a=words(out);assert 0<len(a)<=n//2 and len(set(a))==len(a) and all(v<n for v in a);return 'unique valid selected vertex ids'
 if op in ['fetch-remap','remap','remap-multi','remap-custom']:
  a=words(out);used=set(i);assert len(a)==n and all(v==0xffffffff or v<n for v in a) and all(a[j]!=0xffffffff for j in used)
  mapped=sorted(set(a[j] for j in used));assert mapped==list(range(len(mapped)));return 'referenced vertices mapped to compact contiguous ids'
 if op=='position-remap':a=words(out);assert len(a)==n and all(v<n and p[j*12:j*12+12]==p[v*12:v*12+12] for j,v in enumerate(a));return 'each representative has identical position bytes'
 if op in ['shadow','shadow-multi']:
  a=words(out);assert len(a)==len(i) and all(v<n and p[v*12:v*12+12]==p[j*12:j*12+12] for v,j in zip(a,i));return 'each rewritten index preserves vertex bytes'
 if op=='remap-vertices':assert len(out)%12==0 and set(out[j:j+12] for j in range(0,len(out),12))==set(p[j*12:j*12+12] for j in set(i));return 'exact referenced unique vertex records'
 if op=='remap-indices':a=words(out);assert len(a)==len(i) and all(v<n for v in a);return 'valid compact indices; paired remap bytes checked separately'
 if op in ['adjacency','tessellation']:a=words(out);assert len(a)==len(i)*(2 if op=='adjacency' else 4) and all(v<n for v in a);return 'expected index count and bounds'
 if op=='provoking':a=words(out);nv=a[0];ix=a[1:len(i)+1];rem=a[len(i)+1:];assert len(rem)==nv and all(v<nv for v in ix) and all(v<n for v in rem);assert triangles(tuple(rem[v] for v in ix))==triangles(i);return 'reorder reconstructs oriented original triangles'
 if op in ['spatial-remap','spatial-points']:assert sorted(words(out))==list(range(n));return 'vertex permutation'
 if op=='partition':a=words(out);assert len(a)==1+math.ceil(len(i)/96) and a[0]>0 and all(v<a[0] for v in a[1:]);return 'one in-range partition id per input cluster'
 if op.endswith('-bounds'):
  vals=struct.unpack('<11f4b',out);assert all(math.isfinite(v) for v in vals[:11]) and vals[3]>=0
  if op=='sphere-bounds':
   xyz=struct.iter_unpack('<3f',p);assert all(math.dist(v,vals[:3])<=vals[3]+max(1e-5,vals[3]*1e-6) for v in xyz)
  return 'finite fields/nonnegative radius; sphere contains all points when applicable'
 if op=='optimize-meshlet':
  nv=struct.unpack_from('<I',out)[0];v=words(out[4:4+nv*4]);t=out[4+nv*4:];assert len(t)==3 and all(j<nv for j in t) and triangles(tuple(v[j] for j in t))==triangles((0,1,2));return 'local indices reconstruct original triangle'
 if op.endswith('-decode') and op.split('-')[0] in ['oct','quat','exp','color']:
  stride=16 if op=='exp-decode' else 8;assert len(out)==n*stride
  if op=='exp-decode':assert all(math.isfinite(v[0]) for v in struct.iter_unpack('<f',out))
  elif op in ['oct-decode','quat-decode']:
   for v in struct.iter_unpack('<4h',out):assert abs(sum((j/32767)**2 for j in v[:3 if op=='oct-decode' else 4])-1)<0.002
  return 'expected layout; unit decoded directions/quaternions or finite exponential values'
 if op.endswith('-encode') and op.split('-')[0] in ['oct','quat','exp','color']:
  assert len(out)==n*(16 if op=='exp-encode' else 8);return 'expected encoded layout; decoded filter validity covered separately'
 if op=='dequantize-half':
  assert len(out)==256*4
  for j,(value,) in enumerate(struct.iter_unpack('<f',out)):
   half=j*251;exp=(half>>10)&31;mant=half&1023
   assert (math.isnan(value) if exp==31 and mant else math.isinf(value) if exp==31 else math.isfinite(value))
   assert (math.copysign(1,value)<0)==bool(half&32768)
  return 'half bit pattern maps to expected finite/Inf/NaN category and sign'
 if op in ['quantize-float','simplify-scale','analyze-coverage']:
  assert len(out)%4==0 and all(math.isfinite(v[0]) for v in struct.iter_unpack('<f',out));return 'finite numeric result'
 if op=='quantize-unorm':assert len(out)==n*4*4 and all(v<=4095 for v in words(out));return '12-bit unsigned normalized range'
 if op=='quantize-snorm':assert len(out)==n*4*4 and all(abs(v)<=2047 for (v,) in struct.iter_unpack('<i',out));return '12-bit signed normalized range'
 if op=='quantize-half':assert len(out)==n*3*4 and all(v<=65535 for v in words(out));return '16-bit range'
 if op.startswith('analyze-'):assert len(out)>0;return 'defined statistics; exact comparison to peer, not performance acceptance'
 if op.endswith('-bound'):assert len(out)==8 and int.from_bytes(out,'little')>0;return 'positive representable capacity'
 if op.endswith('-version'):assert out in [b'\0',b'\1'];return 'supported codec version'
 raise ValueError('no validator '+op)
def interval(ratios):
 logs=[math.log(v) for v in ratios];mu=statistics.mean(logs);n=len(logs);t={5:2.776445,20:2.093024}[n];radius=t*statistics.stdev(logs)/math.sqrt(n)
 return [math.exp(mu-radius),math.exp(mu+radius)]
def source_check():
 manifest=json.loads((ART/'measurement-source.json').read_text())
 assert manifest['files']=={p:sha(ROOT/p) for p in manifest['files']},'measurement source epoch changed'
def measure(profile,group):
 assert os.environ.get('MOSS_HEAVY_ACTIVE'),'CPU shared admission required'
 source_check()
 disk();recordpath=ART/f'measure-{profile}-{group}.json';started=time.monotonic()
 identity={b:sha(ART/f'cmp-driver-{profile}-{b}') for b in ['ours','theirs']}
 if recordpath.exists():
  result=json.loads(recordpath.read_text());assert not result['completed'],'do not repeat completed streams'
  assert result['binary_sha256']==identity,'cannot resume a changed executable epoch'
 else:
  result={'profile':profile,'group':group,'core':quiet.physical_core(),'started':time.time(),'binary_sha256':identity,'rows':[],'completed':False,'bursts':[]}
 rows=result['rows'];cpu=result['core']['cpu'];assert cpu in os.sched_getaffinity(0)
 burst={'pid':os.getpid(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE'],'started':time.time(),'cpu':cpu};result['bursts'].append(burst)
 def save():
  tmp=recordpath.with_suffix('.tmp');tmp.write_text(json.dumps(result,indent=2)+'\n');tmp.replace(recordpath)
 def budget():
  disk()
  if time.monotonic()-started>720:
   burst['finished']=time.time();burst['checkpoint_stop']=True;save();raise SystemExit(75)
 save()
 for name,nt,shape in CASES:
  for op in (PERF if group=='timing' else PERF+EXTRA):
   prior=next((r for r in rows if r['case']==name and r['operation']==op),None)
   if prior and prior.get('completed'):continue
   budget();path=ART/'inputs'/f'{name}{"-strip" if op=="unstripify" else ""}.bin';ds={}
   try:
    for b in ['ours','theirs']:ds[b]=Driver(b,profile,op,path,cpu)
    output={b:d.output.read_bytes() for b,d in ds.items()};checks={b:valid(op,o,path,b,profile) for b,o in output.items()}
    row=prior or {'case':name,'operation':op,'input_sha256':sha(path),'outputs':{b:{'sha256':hashlib.sha256(o).hexdigest(),'bytes':len(o),'validation':checks[b]} for b,o in output.items()},'byte_identical':output['ours']==output['theirs'],'samples':[],'completed':False}
    if prior:assert 'pending_pair' not in prior,'interrupted pair retained; do not silently repeat it'
    if prior:assert prior['input_sha256']==sha(path) and all(prior['outputs'][b]['sha256']==hashlib.sha256(output[b]).hexdigest() for b in ds)
    else:rows.append(row)
    if op.startswith('meshlets'):
     raw={b:Path(str(d.output)+'.raw').read_bytes() for b,d in ds.items()};row['raw_byte_identical']=raw['ours']==raw['theirs'];row['raw_outputs']={b:{'bytes':len(o),'sha256':hashlib.sha256(o).hexdigest()} for b,o in raw.items()}
    save()
    if group=='timing':
     if 'iterations' not in row:
      calibration={b:ds[b].sample(1) for b in ds};row['iterations']=max(1,min(1000000,math.ceil(25000000/min(calibration.values()))));row['calibration_ns']=calibration;save()
     for pair in range(len(row['samples']),20):
      if pair>=5 and not row['five_pair_ci'][0]<=1<=row['five_pair_ci'][1]:break
      budget();order=['ours','theirs'] if pair%2==0 else ['theirs','ours'];load=os.getloadavg();v={}
      # Retain each measured backend even if the outer hard timeout interrupts its pair.
      row['pending_pair']={'pair':pair,'order':order,'load':load,'ns':v,'cpu':cpu}
      for backend in order:
       v[backend]=ds[backend].sample(row['iterations']);row['pending_pair']['ns']=dict(v);save()
      row['samples'].append({'pair':pair,'order':order,'ns':v,'ratio':v['ours']/v['theirs'],'load':load,'unix':time.time(),'cpu':cpu})
      del row['pending_pair']
      if pair==4:row['five_pair_ci']=interval([v['ratio'] for v in row['samples']])
      save()
     ratios=[v['ratio'] for v in row['samples']];row['median_ratio']=statistics.median(ratios);row['ci95']=interval(ratios)
    row['completed']=True;save()
    print(profile,name,op,'identical' if row['byte_identical'] else 'DIFF',f"{row.get('median_ratio',0):.3f}",len(row['samples']),flush=True)
   finally:
    for d in ds.values():d.close()
 result['completed']=True;result['finished']=time.time();burst['finished']=time.time();burst['duration_seconds']=time.monotonic()-started
 assert result['binary_sha256']=={b:sha(ART/f'cmp-driver-{profile}-{b}') for b in ['ours','theirs']};source_check();save()
if __name__=='__main__':
 if sys.argv[1]=='prepare':prepare()
 else:measure(*sys.argv[1:])
