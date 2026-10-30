"""Interleaved resident-loop instructions and pinned four-core wall disclosures."""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess

assert os.environ.get('MOSS_HEAVY_ACTIVE')
lane=Path(__file__).resolve().parent
scratch=Path(os.environ['CARGO_TARGET_DIR'])/'proof-work'
inputs=Path(os.environ['MESHOPT_INPUTS'])
perf=os.environ.get('MESHOPT_PERF','/usr/lib/linux-tools/6.17.0-22-generic/perf')
binding=json.loads((lane/'build-binding.json').read_text())
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
for arm in ['base','serial','candidate','cpp-scalar','cpp-default']:
    assert sha(scratch/('driver-'+arm))==binding['binaries'][arm]
root=lane.parents[1]
for path,digest in binding['sources'].items():
    if not path.startswith('consumer-'):assert sha(root/path)==digest,('source drift',path)
# Distinct physical cores, no SMT siblings.
cpus=[];seen=set()
for cpu in sorted(os.sched_getaffinity(0)):
    topology=Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
    core=tuple((topology/p).read_text().strip() for p in ['physical_package_id','core_id'])
    if core not in seen:cpus.append(cpu);seen.add(core)
    if len(cpus)==4:break
assert len(cpus)==4
names=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']
manifest=[]
for name in names:
    path=inputs/(name+'.bin')
    manifest.append(dict(case=name,sha256=sha(path),bytes=path.stat().st_size))
(lane/'inputs.json').write_text(json.dumps(manifest,indent=2)+'\n')
(lane/'environment.json').write_text(json.dumps(dict(cpus=cpus,perf=subprocess.check_output([perf,'--version'],text=True).strip(),event='instructions:u',instruction_repetitions=3,wall_repetitions=5,order='forward,reverse,forward; wall alternates AB/BA',workers=4),indent=2)+'\n')
rows_path=lane/'counts.jsonl'
rows=[json.loads(x) for x in rows_path.read_text().splitlines()] if rows_path.exists() else []
keys={(r['kind'],r['case'],r['arm'],r['rep']) for r in rows}
def record(row):
    with rows_path.open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
    rows.append(row);keys.add((row['kind'],row['case'],row['arm'],row['rep']))
def sample(kind,name,arm,rep):
    key=(kind,name,arm,rep)
    if key in keys:return
    n=100 if name=='tiny-smooth' else 3 if name.startswith('medium') else 1
    out=scratch/'sample-output.bin'
    cmd=['taskset','-c',','.join(map(str,cpus if arm=='candidate' else cpus[:1])),str(scratch/('driver-'+arm)),'cluster-lod',str(inputs/(name+'.bin')),str(out)]
    env=dict(os.environ,LC_ALL='C',RAYON_NUM_THREADS='4')
    fds=[]
    if kind=='instructions':
        cr,cw=os.pipe();ar,aw=os.pipe();fds=[cr,cw,ar,aw]
        report=scratch/'sample.perf'
        cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(report),'-D','-1','--control',f'fd:{cr},{aw}','--']+cmd
        env.update(PROBE_CTL=str(cw),PROBE_ACK=str(ar))
    try:r=subprocess.run(cmd,input=f'{n}\nstop\n',capture_output=True,text=True,env=env,pass_fds=fds)
    finally:
        for fd in fds:os.close(fd)
    assert r.returncode==0,(key,r.stderr[-4000:],r.stdout[-4000:])
    lines=r.stdout.splitlines()
    assert len(lines)==2 and lines[0]=='READY' and lines[1].startswith('COUNTED '),(key,lines)
    row=dict(kind=kind,case=name,arm=arm,rep=rep,repetitions=n,output_sha256=sha(out),output_bytes=out.stat().st_size)
    if kind=='instructions':
        fields=next(line.split(';') for line in report.read_text().splitlines() if ';instructions:u;' in line)
        count=int(fields[0]);assert count>0 and float(fields[4])==100.0,fields
        row.update(instructions=count,instructions_per_call=count/n)
    else:row['seconds_per_call']=float(lines[1].split()[1])/n
    peers=[r for r in rows if r['case']==name]
    assert not peers or {r['output_sha256'] for r in peers}=={row['output_sha256']},('output mismatch',key)
    record(row)
    print(kind,name,arm,rep,row.get('instructions_per_call',row.get('seconds_per_call')),flush=True)

arms=['base','serial','candidate','cpp-scalar','cpp-default']
for name in names:
    for rep in range(3):
        for arm in arms if rep%2==0 else reversed(arms):sample('instructions',name,arm,rep)
for name in names[-3:]:
    for rep in range(5):
        for arm in ['serial','candidate'] if rep%2==0 else ['candidate','serial']:sample('wall',name,arm,rep)
result=dict(instructions={},wall={},output_identity=True,samples=len(rows))
for name in names:
    cell={arm:statistics.median(r['instructions_per_call'] for r in rows if r['kind']=='instructions' and r['case']==name and r['arm']==arm) for arm in arms}
    result['instructions'][name]=cell
for name in names[-3:]:
    cell={arm:statistics.median(r['seconds_per_call'] for r in rows if r['kind']=='wall' and r['case']==name and r['arm']==arm) for arm in ['serial','candidate']}
    cell['speedup']=cell['serial']/cell['candidate'];result['wall'][name]=cell
(lane/'results.json').write_text(json.dumps(result,indent=2)+'\n')
print('PASS samples',len(rows),flush=True)
