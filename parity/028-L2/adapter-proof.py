#!/usr/bin/env python3
"""No-clock proof of the final exported adapters and every real stage arm."""
import hashlib, json, os, subprocess, sys
from pathlib import Path
ART = Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
binaries=dict(zip(['candidate','base','M7','X'],sys.argv[1:]))
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1<<20),b''):h.update(b)
 return h.hexdigest()
record=dict(frozen=[],stages=[],mismatches=[],passed=False)
if os.environ.get('MESHOPT_STAGES_ONLY'):
 record=json.loads((ART/'adapter-proof.json').read_text())
 assert len(record['frozen'])==23 and all(r['exact'] for r in record['frozen'])
 assert not record['stages'] and not record['mismatches']
record['binaries']={arm:hashlib.sha256(Path(binary).read_bytes()).hexdigest() for arm,binary in binaries.items()}
def save(): (ART/'adapter-proof.json').write_text(json.dumps(record,indent=2)+'\n')
if not os.environ.get('MESHOPT_STAGES_ONLY'):
 for op in ['cache','cache-fifo','simplify-scale','overdraw']:
  for case in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']+(['real-214e-overdraw','real-b6fd-overdraw','real-a082-overdraw'] if op=='overdraw' else []):
   hashes={}
   for arm in ['candidate','base','M7']:
    out=ART/f'adapter-{arm}-{case}-{op}.bin'
    p=subprocess.Popen([binaries[arm],op,str(ART/'inputs'/f'{case}.bin'),str(out)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
    assert p.stdout.readline().strip()=='READY'
    p.communicate('stop\n');assert p.returncode==0
    hashes[arm]=sha(out)
   exact=len(set(hashes.values()))==1
   record['frozen'].append(dict(case=case,operation=op,hashes=hashes,exact=exact));save()
   if not exact: record['mismatches'].append(dict(case=case,op=op,hashes=hashes)); save()
for trace in json.loads((ART/'stages.json').read_text()):
 arms={}
 for arm in ['candidate','base','M7']:
  out=ART/f'adapter-chain-{arm}-{trace["case"]}.tsv'
  subprocess.run([binaries[arm],'chain-check',trace['manifest'],str(out)],check=True)
  calls=[]
  for line,expected in zip(out.read_text().splitlines(),trace['calls'],strict=True):
   index,op,ns,file=line.split('\t');digest=sha(file)
   assert int(ns)==0
   calls.append(dict(index=int(index),operation=op,hash=digest,expected=expected['output_sha256'],exact_capture=digest==expected['output_sha256']))
   if digest==expected['output_sha256']:Path(file).unlink()
  arms[arm]=calls
  record['stages'].append(dict(case=trace['case'],arm=arm,calls=calls));save()
 for i,expected in enumerate(trace['calls']):
  hashes={arm:calls[i]['hash'] for arm,calls in arms.items()}
  if len(set(hashes.values()))!=1 or next(iter(hashes.values()))!=expected['output_sha256']:
   record['mismatches'].append(dict(case=trace['case'],index=i,hashes=hashes,expected=expected['output_sha256'])); save()
record['passed']=not record['mismatches'];save()
if record['mismatches']:
 print(json.dumps(record['mismatches'],indent=2));raise SystemExit(1)
print('PASS 23 family cases and every ordinary/legacy real stage against baseline and M7')
