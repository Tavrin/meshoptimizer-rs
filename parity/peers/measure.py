"""Resumable interleaved resident-loop instruction samples, median of three."""
import hashlib,json,math,os,statistics,struct,subprocess,time
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
art=Path(os.environ['PEERS_ART']); root=Path(__file__).resolve().parents[2]
perf=os.environ['PEERS_PERF']; cpu=int(os.environ.get('PEERS_CPU','9'));assert cpu in os.sched_getaffinity(0)
ops=['simplify','simplify-attributes','simplify-sloppy','cache','overdraw','fetch','meshlets','vertex-encode','vertex-decode','index-encode','index-decode','cluster-lod']
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
binding=json.loads((art/'build-binding.json').read_text())
for name,h in binding['source'].items():assert sha(root/name)==h,('source changed',name)
for a,h in binding['binaries'].items():assert sha(art/f'driver-{a}')==h
inputs=json.loads((art/'inputs.json').read_text())
for x in inputs:assert sha(art/'inputs'/f"{x['case']}.bin")==x['sha256']
registration=dict(build=binding,inputs=inputs,perf=subprocess.check_output([perf,'--version'],text=True).strip(),cpu=cpu,ops=ops)
p=art/'campaign.json'
if p.exists():assert json.loads(p.read_text())==registration
else:p.write_text(json.dumps(registration,indent=2)+'\n')
for d in ['samples','outputs','interrupted']:(art/d).mkdir(exist_ok=True)
rows=[json.loads(s) for s in (art/'counts.jsonl').read_text().splitlines()] if (art/'counts.jsonl').exists() else []
keys={(r['case'],r['op'],r['arm'],r['rep']) for r in rows};assert len(keys)==len(rows)
for x in inputs:
 for op in ops:
  for rep in range(3):
   for arm in (['cpp','ours','rust'] if rep!=1 else ['rust','ours','cpp']):
    if op=='cluster-lod' and arm=='rust':continue
    key=(x['case'],op,arm,rep)
    if key in keys:continue
    name='-'.join(map(str,key));prefix=art/'samples'/name;out=art/'outputs'/(name+'.bin')
    leftovers=[*prefix.parent.glob(name+'.*'),*out.parent.glob(out.name+'*')]
    if leftovers:
     import tempfile
     dest=Path(tempfile.mkdtemp(prefix=name+'-',dir=art/'interrupted'))
     for file in leftovers:file.rename(dest/file.name)
    n=1000 if x['case']=='tiny-smooth' else 3 if x['case'].startswith('medium') else 1
    if op=='cluster-lod':n=10 if x['case']=='tiny-smooth' else 1
    cr,cw=os.pipe();ar,aw=os.pipe()
    cmd=['/usr/bin/time','-f','%M','-o',str(prefix)+'.rss',perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c',str(cpu),str(art/f'driver-{arm}'),op,str(art/'inputs'/f"{x['case']}.bin"),str(out)]
    started=time.monotonic()
    try:result=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=1200)
    finally:
     for fd in [cr,cw,ar,aw]:os.close(fd)
    row=dict(case=x['case'],op=op,arm=arm,rep=rep,repetitions=n,exit=result.returncode,input_sha256=x['sha256'],binary_sha256=binding['binaries'][arm],process_wall=time.monotonic()-started)
    Path(str(prefix)+'.stdout').write_text(result.stdout);Path(str(prefix)+'.stderr').write_text(result.stderr)
    Path(str(prefix)+'.json').write_text(json.dumps(row,indent=2)+'\n')
    assert result.returncode==0,(key,result.stderr)
    lines=result.stdout.splitlines();assert len(lines)==2 and lines[0]=='READY' and lines[1].startswith('COUNTED '),(key,lines)
    fields=next(s.split(';') for s in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in s)
    count=int(fields[0]);running=float(fields[4]);assert count>0 and running==100.,fields
    row.update(instructions=count,instructions_per_call=count/n,running_percent=running,wall_seconds_per_call=float(lines[1].split()[1])/n,peak_rss_kib=int(Path(str(prefix)+'.rss').read_text().strip()),output_sha256=sha(out),output_bytes=out.stat().st_size)
    if op=='meshlets':row['raw_sha256']=sha(Path(str(out)+'.raw'))
    old=[r for r in rows if (r['case'],r['op'],r['arm'])==key[:3]]
    assert not old or old[0]['output_sha256']==row['output_sha256'],('unstable',key)
    Path(str(prefix)+'.json').write_text(json.dumps(row,indent=2)+'\n')
    with (art/'counts.jsonl').open('a') as f:f.write(json.dumps(row)+'\n')
    rows.append(row);keys.add(key)
  print(x['case'],op,'complete',len(rows),flush=True)
(art/'measurement-exit.json').write_text(json.dumps(dict(exit=0,cells=len(inputs)*len(ops),samples=len(rows)),indent=2)+'\n')
