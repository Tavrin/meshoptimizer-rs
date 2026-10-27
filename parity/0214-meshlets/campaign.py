#!/usr/bin/env python3
"""Append-only, resumable instruction campaign for four meshlet workloads."""
import hashlib,json,os,subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
assert os.environ.get('PARITY_RUNNER_MEASUREMENT')=='1'
root=Path(__file__).resolve().parents[2]
art=Path(os.environ['MESHLETS_ART'])
inputs=Path(os.environ['MESHLETS_INPUTS'])
perf=os.environ['MESHLETS_PERF']
cpu=int(os.environ.get('MESHLETS_CPU','9'))
assert cpu in os.sched_getaffinity(0)
ops=['meshlets','meshlets-flex','meshlets-spatial','cluster-lod']
cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']
arms=['base','candidate','cpp','rust']
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
files=sorted((root/'src').rglob('*.rs'))+[root/'Cargo.toml',root/'Cargo.lock',root/'parity/0214-meshlets/campaign.py',root/'parity/0214-meshlets/adapter.py',root/'parity/peers/src/main.rs',root/'parity/peers/Cargo.toml',root/'parity/peers/Cargo.lock']
seal=dict(sources={str(p.relative_to(root)):sha(p) for p in files}, binaries={a:sha(art/f'driver-campaign-{a}') for a in arms},inputs={c:sha(inputs/f'{c}.bin') for c in cases},perf=subprocess.check_output([perf,'--version'],text=True).strip(),cpu=cpu,ops=ops,cases=cases,settings='64/124, flex min 64 split 0 weight 0, spatial min 64 fill 0.5, cluster default raster 128')
registration=art/'campaign-registration.json'
if registration.exists(): assert json.loads(registration.read_text())==seal
else: registration.write_text(json.dumps(seal,indent=2)+'\n')
path=art/'campaign.jsonl'
rows=[json.loads(s) for s in path.read_text().splitlines()] if path.exists() else []
keys={(r['case'],r['op'],r['arm'],r['rep']) for r in rows}
assert len(keys)==len(rows)
for case in cases:
 for op in ops:
  for rep in range(3):
   for arm in (arms if rep!=1 else list(reversed(arms))):
    if arm=='rust' and op=='cluster-lod':continue
    key=(case,op,arm,rep)
    if key in keys:continue
    name='-'.join(map(str,key)); out=art/(name+'.bin'); stat=art/(name+'.stat')
    n=1000 if case=='tiny-smooth' else 3 if case.startswith('medium') else 1
    if op=='cluster-lod':n=10 if case=='tiny-smooth' else 1
    cr,cw=os.pipe();ar,aw=os.pipe()
    cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(stat),'-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c',str(cpu),str(art/f'driver-campaign-{arm}'),op,str(inputs/f'{case}.bin'),str(out)]
    try:r=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=1200)
    finally:
     for fd in (cr,cw,ar,aw):os.close(fd)
    (art/(name+'.stderr')).write_text(r.stderr)
    assert r.returncode==0,(key,r.stderr)
    fields=next(s.split(';') for s in stat.read_text().splitlines() if ';instructions:u;' in s)
    assert int(fields[0])>0 and float(fields[4])==100.,(key,fields)
    row=dict(case=case,op=op,arm=arm,rep=rep,repetitions=n,instructions=int(fields[0]),per_call=int(fields[0])/n,output_sha256=sha(out),exit=r.returncode)
    if op.startswith('meshlets'):row['raw_sha256']=sha(Path(str(out)+'.raw'))
    with path.open('a') as f:f.write(json.dumps(row)+'\n')
    rows.append(row);keys.add(key)
    print(name,row['per_call'],flush=True)
