import hashlib,json,os,subprocess,tempfile
from pathlib import Path
s=Path(os.environ['MESHOPT_SCRATCH'])
root=Path(__file__).resolve().parents[2]
perf=os.environ['MESHOPT_PERF']
cpu=os.environ.get('MESHOPT_CPU','9')
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU')=='0'
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']
ops=['simplify','simplify-attributes','simplify-update','simplify-sloppy','simplify-prune','simplify-points','overdraw']
arms=['base','candidate','cpp','rust']
files=[*(s/f'full-driver-{a}' for a in arms),*(s/'inputs'/f'{x}.bin' for x in cases),*(root/'src').rglob('*.rs'), *(s/'base/src').rglob('*.rs'), s/'driver-source.sha256', Path(__file__)]
binding={str(p):sha(p) for p in files}
p=s/'full-binding.json'
if p.exists():assert json.loads(p.read_text())==binding
else:p.write_text(json.dumps(binding,indent=2)+'\n')
(s/'interrupted').mkdir(exist_ok=True)
rows=s/'full-counts.jsonl'
keys={(x['case'],x['op'],x['arm'],x['rep']) for x in map(json.loads,rows.read_text().splitlines())} if rows.exists() else set()
for case in cases:
 for op in ops:
  for rep in range(3):
   for arm in (arms if rep != 1 else arms[::-1]):
    if (case,op,arm,rep) in keys:continue
    name=f'{case}-{op}-{arm}-{rep}';prefix=s/('full-'+name);out=Path(str(prefix)+'.out')
    leftovers=[Path(str(prefix)+ext) for ext in ('.out','.perf','.log') if Path(str(prefix)+ext).exists()]
    if leftovers:
     dest=Path(tempfile.mkdtemp(prefix=name+'-',dir=s/'interrupted'))
     for leftover in leftovers:leftover.rename(dest/leftover.name)
    n=1000 if case.startswith('tiny') else 3 if case.startswith('medium') else 1
    cr,cw=os.pipe();ar,aw=os.pipe()
    cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c',cpu,str(s/f'full-driver-{arm}'),op,str(s/'inputs'/f'{case}.bin'),str(out)]
    try:r=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=1200)
    finally:
     for fd in (cr,cw,ar,aw):os.close(fd)
    Path(str(prefix)+'.log').write_text(r.stdout+'\n'+r.stderr)
    assert r.returncode==0,(name,r.stderr)
    fields=next(x.split(';') for x in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in x)
    assert float(fields[4])==100
    row=dict(case=case,op=op,arm=arm,rep=rep,count=int(fields[0]),repetitions=n,output=sha(out),binary=binding[str(s/f'full-driver-{arm}')])
    with rows.open('a') as f:f.write(json.dumps(row)+'\n')
    print(row,flush=True)
