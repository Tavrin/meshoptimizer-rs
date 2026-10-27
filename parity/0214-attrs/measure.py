"""Four-arm frozen-corpus instruction gate; append and resume finished cases."""
import hashlib,json,os,subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2]; art=Path(os.environ['ATTR_ART']); inputs=Path(os.environ['ATTR_INPUTS']); perf=os.environ['ATTR_PERF']
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
binding=json.loads((art/'full-build-binding.json').read_text())
for p,h in binding['source'].items():assert sha(root/p)==h
for arm,h in binding['binaries'].items():assert sha(art/f'full-driver-{arm}')==h
metadata=json.loads((inputs.parent/'inputs.json').read_text())
for x in metadata:assert sha(inputs/(x['case']+'.bin'))==x['sha256']
registration=dict(binding=binding,inputs=metadata,perf=subprocess.check_output([perf,'--version'],text=True).strip(),cpu=9,ops=['simplify-attributes','simplify','overdraw'])
p=art/'full-campaign.json'
if p.exists():assert json.loads(p.read_text())==registration
else:p.write_text(json.dumps(registration,indent=2)+'\n')
(art/'full-samples').mkdir(exist_ok=True);(art/'full-interrupted').mkdir(exist_ok=True)
path=art/'full-counts.jsonl';rows=[json.loads(x) for x in path.read_text().splitlines()] if path.exists() else []
keys={(r['case'],r['op'],r['arm'],r['rep']) for r in rows};assert len(keys)==len(rows)
for x in metadata:
 for op in registration['ops']:
  for rep in range(3):
   for arm in (['base','candidate','cpp','rust'] if rep!=1 else ['rust','cpp','candidate','base']):
    key=(x['case'],op,arm,rep)
    if key in keys:continue
    name='-'.join(map(str,key));prefix=art/'full-samples'/name;out=Path(str(prefix)+'.bin')
    leftovers=list(prefix.parent.glob(name+'.*'))
    if leftovers:
     import tempfile
     dest=Path(tempfile.mkdtemp(prefix=name+'-',dir=art/'full-interrupted'))
     for f in leftovers:f.rename(dest/f.name)
    n=1000 if x['case']=='tiny-smooth' else 3 if x['case'].startswith('medium') else 1
    cr,cw=os.pipe();ar,aw=os.pipe()
    cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c','9',str(art/f'full-driver-{arm}'),op,str(inputs/(x['case']+'.bin')),str(out)]
    try:r=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=1200)
    finally:
     for fd in (cr,cw,ar,aw):os.close(fd)
    Path(str(prefix)+'.stdout').write_text(r.stdout);Path(str(prefix)+'.stderr').write_text(r.stderr)
    assert r.returncode==0 and r.stdout.startswith('READY\nCOUNTED '),(key,r.returncode,r.stderr)
    fields=next(s.split(';') for s in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in s)
    assert float(fields[4])==100. and int(fields[0])>0,(key,fields)
    row=dict(case=x['case'],op=op,arm=arm,rep=rep,instructions=int(fields[0]),repetitions=n,instructions_per_call=int(fields[0])/n,input_sha256=x['sha256'],binary_sha256=binding['binaries'][arm],output_sha256=sha(out),exit=r.returncode,running_percent=float(fields[4]))
    with path.open('a') as f:f.write(json.dumps(row)+'\n')
    rows.append(row);keys.add(key);print(name,flush=True)
