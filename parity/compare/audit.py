#!/usr/bin/env python3
"""Independent integrity/semantic audit of the completed comparison."""
from pathlib import Path
import json,hashlib,subprocess,sys,math,statistics,re,os,struct
ROOT=Path(__file__).resolve().parents[2];ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
sys.path.insert(0,str(Path(__file__).parent));import measure
checks=[]
def check(name,condition):
 checks.append({'name':name,'passed':bool(condition)})
 if not condition:raise AssertionError(name)
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
source=read(ART/'source-manifest.json');check('library sources unchanged',source=={str(p.relative_to(ROOT)):sha(p) for p in sorted((ROOT/'src').rglob('*')) if p.is_file()})
epoch=read(ART/'measurement-source.json');check('measurement source epoch unchanged',epoch['files']=={p:sha(ROOT/p) for p in epoch['files']})
# source-manifest paths were frozen from the repository's relative cwd.
check('pinned comparison revision',subprocess.check_output(['git','-C',str(ROOT),'rev-parse','HEAD'],text=True).strip()=='dcaf997ef55d2c4ccc05ce0ef885d2d9a16018d0')
check('vendor normalized v0.25 identity',all(r['matches_v0_25_normalized_eol'] for r in read(ART/'vendor-version.json')))
check('all 89 upstream names classified',len(read(ART/'api-inventory.json'))==89)
for r in read(ART/'inputs.json'):
 for suffix,key in [('', 'sha256'),('-strip','strip_sha256')]:check('frozen input '+r['name']+suffix,sha(ART/'inputs'/f'{r["name"]}{suffix}.bin')==r[key])
for prof in ['defaults','moss']:
 for b in ['ours','theirs']:
  for program in ['cmp-decode','cmp-full','cmp-empty','cmp-driver']:
   p=ART/f'{program}-{prof}-{b}';check('retained binary '+p.name,sha(p)==Path(str(p)+'.sha256').read_text().strip())
 check('decoder backend artifacts distinct '+prof,sha(ART/f'cmp-decode-{prof}-ours')!=sha(ART/f'cmp-decode-{prof}-theirs'))
 check('driver backend artifacts distinct '+prof,sha(ART/f'cmp-driver-{prof}-ours')!=sha(ART/f'cmp-driver-{prof}-theirs'))
path=ART/'measure-defaults-outputs.json';out=read(path)
fixtures={r['name']:r for r in read(ART/'inputs.json')}
expected={(name,op) for name,_,_ in measure.CASES for op in measure.PERF+measure.EXTRA}
check('complete untimed output corpus',out['completed'] and {(r['case'],r['operation']) for r in out['rows']}==expected and len(out['rows'])==len(expected))
for r in out['rows']:
 for b in ['ours','theirs']:
  p=ART/'outputs'/f'defaults-{r["case"]}{"-strip" if r["operation"]=="unstripify" else ""}-{r["operation"]}-{b}.bin'
  check('output hash '+p.name,p.stat().st_size==r['outputs'][b]['bytes'] and sha(p)==r['outputs'][b]['sha256'])
  if r['operation'] in ['oct-encode','quat-encode','exp-encode','color-encode']:
   k=['oct-encode','quat-encode','exp-encode','color-encode'].index(r['operation'])+3
   check('filter encoding is independently validated decode fixture '+p.name,r['outputs'][b]['sha256']==fixtures[r['case']]['codec_sha256'][k])
  if r['operation'] in ['analyze-cache','analyze-fetch','analyze-overdraw']:
   values=p.read_bytes();offset={'analyze-cache':8,'analyze-fetch':4,'analyze-overdraw':8}[r['operation']]
   check('finite nonnegative analysis statistics '+p.name,all(math.isfinite(v) and v>=0 for (v,) in struct.iter_unpack('<f',values[offset:])))
  if 'raw_outputs' in r:
   raw=Path(str(p)+'.raw');check('raw meshlet output hash '+raw.name,raw.stat().st_size==r['raw_outputs'][b]['bytes'] and sha(raw)==r['raw_outputs'][b]['sha256'])
# Independent validators must reject bad effects; not just agree with each other.
inputpath=ART/'inputs'/'tiny-smooth.bin';n,p,i,buffers=measure.unpack(inputpath)
for label,op,bad in [('vertex-byte corruption','vertex-decode',bytes(len(p))),('missing triangle','cache',b''.join(v.to_bytes(4,'little') for v in i[:-3]))]:
 rejected=False
 try:measure.valid(op,bad,inputpath,'ours','defaults')
 except AssertionError:rejected=True
 check('negative control '+label,rejected)
check('default vertex format difference observed',all(not r['byte_identical'] for r in out['rows'] if r['operation']=='vertex-encode-default'))
for name,_,_ in measure.CASES:
 n,positions,indices,_=measure.unpack(ART/'inputs'/f'{name}.bin')
 for backend in ['ours','theirs']:
  prefix=ART/'outputs'/f'defaults-{name}'
  vertices=Path(str(prefix)+f'-remap-vertices-{backend}.bin').read_bytes()
  remapped=measure.words(Path(str(prefix)+f'-remap-indices-{backend}.bin').read_bytes())
  check('paired vertex/index remap reconstruction '+name+'/'+backend,b''.join(vertices[j*12:j*12+12] for j in remapped)==b''.join(positions[j*12:j*12+12] for j in indices))
  for operation,triangles in [('cluster-bounds',indices[:124*3]),('meshlet-bounds',[0,1,2])]:
   referenced=set()
   for k in range(0,len(triangles),3):
    tri=triangles[k:k+3];a,b,c=[struct.unpack_from('<3f',positions,j*12) for j in tri]
    ab=[b[j]-a[j] for j in range(3)];ac=[c[j]-a[j] for j in range(3)]
    cross=[ab[1]*ac[2]-ab[2]*ac[1],ab[2]*ac[0]-ab[0]*ac[2],ab[0]*ac[1]-ab[1]*ac[0]]
    if any(cross):referenced.update(tri)
   values=struct.unpack('<11f4b',Path(str(prefix)+f'-{operation}-{backend}.bin').read_bytes())
   check('bounds contain referenced points '+name+'/'+operation+'/'+backend,all(math.dist(struct.unpack_from('<3f',positions,j*12),values[:3])<=values[3]+max(1e-5,values[3]*1e-6) for j in referenced))
   if not referenced:check('degenerate-only bounds are canonical zero '+name+'/'+operation+'/'+backend,all(v==0 for v in values))
# Also require meaningful raw meshlet layout changes somewhere, not a padding-normalized no-effect claim.
check('raw meshlet layout effect observed',any(not r['raw_byte_identical'] for r in out['rows'] if r['operation'].startswith('meshlets')))
expected={(name,op) for name,_,_ in measure.CASES for op in measure.PERF}
untimed={(r['case'],r['operation']):r for r in out['rows']}
for prof in ['defaults','moss']:
 j=read(ART/f'measure-{prof}-timing.json');check('complete timing corpus '+prof,j['completed'] and {(r['case'],r['operation']) for r in j['rows']}==expected and len(j['rows'])==len(expected))
 check('timing executable identities '+prof,j['binary_sha256']=={b:sha(ART/f'cmp-driver-{prof}-{b}') for b in ['ours','theirs']})
 for r in j['rows']:
  for backend in ['ours','theirs']:
   p=ART/'outputs'/f'{prof}-{r["case"]}{"-strip" if r["operation"]=="unstripify" else ""}-{r["operation"]}-{backend}.bin'
   check('timed output hash '+p.name,sha(p)==r['outputs'][backend]['sha256'])
   check('profile-invariant output '+p.name,r['outputs'][backend]['sha256']==untimed[r['case'],r['operation']]['outputs'][backend]['sha256'])
  samples=r['samples'];check('pair count '+prof+'/'+r['case']+'/'+r['operation'],len(samples) in [5,20])
  check('completed pair stream',r['completed'] and 'pending_pair' not in r)
  check('borderline stopping '+prof+'/'+r['case']+'/'+r['operation'],(len(samples)==20)==(r['five_pair_ci'][0]<=1<=r['five_pair_ci'][1]))
  ratios=[]
  for k,s in enumerate(samples):
   check('alternating and pinned pair',s['cpu']==j['core']['cpu'] and s['order']==(['ours','theirs'] if k%2==0 else ['theirs','ours']))
   check('valid sample',all(math.isfinite(v) and v>0 for v in s['ns'].values()));ratio=s['ns']['ours']/s['ns']['theirs'];check('matched ratio',ratio==s['ratio']);ratios.append(ratio)
  check('median and interval calculation',statistics.median(ratios)==r['median_ratio'] and measure.interval(ratios)==r['ci95'])
# Receipts retain queue waits, run durations and exit outcomes, including rejected preflights.
receipts=[l for l in Path('/mnt/linux-extra/moss-coord/heavy.log').read_text().splitlines() if '\tmeshopt-cmp\t' in l or l.split('\t')[-3:-2]==['meshopt-cmp']]
check('shared receipts available',bool(receipts))
check('all comparison receipts CPU only',all('[gpu]' not in l for l in receipts))
check('bounded admitted runs',all(int(l.split('\t')[3])<900 for l in receipts))
(ART/'admission-receipts.tsv').write_text('\n'.join(receipts)+'\n')
check('report has no pending sections','PENDING' not in (ROOT/'parity/COMPARE_MESHOPT_CRATE.md').read_text())
(ART/'audit.json').write_text(json.dumps({'passed':True,'checks':checks,'receipts':len(receipts),'negative_controls':True},indent=2)+'\n')
print('AUDIT PASS',len(checks),'checks;',len(receipts),'CPU admission receipts')
