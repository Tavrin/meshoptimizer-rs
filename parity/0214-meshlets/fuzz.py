#!/usr/bin/env python3
"""Seeded differential fuzz; budget counts the two witness processes' CPU."""
import hashlib, importlib.util, json, os, random, struct, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2]
art=Path(os.environ['MESHLETS_ART'])
os.environ['MESHOPT_REFERENCE']=os.environ['MESHLETS_REFERENCE']
os.environ['MESHOPT_ARTIFACTS']=str(art)
spec=importlib.util.spec_from_file_location('p03',root/'parity/p03/runner.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
class Client:
 def __init__(self,p):self.p=subprocess.Popen([str(p)],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
 def run(self,d):
  self.p.stdin.write(struct.pack('<I',len(d))+d);self.p.stdin.flush();h=self.p.stdout.read(4);assert len(h)==4
  n=struct.unpack('<I',h)[0];v=self.p.stdout.read(n);assert len(v)==n;return v
 def close(self):self.p.stdin.close();assert self.p.wait()==0
def cpu(pid):
 s=Path(f'/proc/{pid}/stat').read_text().rsplit(')',1)[1].split();return (int(s[11])+int(s[12]))/os.sysconf('SC_CLK_TCK')
clients={a:Client(art/f'witness-{a}') for a in ('base','candidate')}
start=sum(cpu(c.p.pid) for c in clients.values());rng=random.Random(20261031);count=0;digest=hashlib.sha256();counts={1:0,3:0,4:0}
while True:
 if count%256==0:
  elapsed=sum(cpu(c.p.pid) for c in clients.values())-start
  if elapsed>=300:break
 op=rng.choice([1,3,4]);d=bytearray(r.case(op,20261031+count));struct.pack_into('<I',d,44,rng.randrange(4))
 if rng.randrange(2):struct.pack_into('<I',d,4,op|0x10000)
 nv,ni=struct.unpack_from('<II',d,8);choice=rng.randrange(6)
 if choice==0 and nv:struct.pack_into('<I',d,48+rng.randrange(nv*3)*4,rng.getrandbits(32))
 elif choice==1 and ni:struct.pack_into('<I',d,48+nv*12+rng.randrange(ni)*4,rng.getrandbits(32))
 elif choice==2:struct.pack_into('<I',d,rng.choice([16,20,24,28,32]),rng.choice([0,1,3,64,256,512,0x7f800000,0x7fc00001,0xffffffff]))
 if count % 2 == 0:
  # Separate triangles force repeated KD fallback, including coincident and
  # nearly coincident coordinates, while preserving valid topology.
  n=2048+rng.randrange(1025); points=[]; indices=[]
  for j in range(n):
   x=float(j % 7) + rng.randrange(3)*0.00001
   base=len(points);points.extend([(x,0.,0.),(x+0.3,0.,0.),(x,0.3,0.)]);indices.extend([base,base+1,base+2])
  d=bytearray(r.message(op | (0x10000 if count%8==0 else 0),points,indices,mv=16,min=4,mt=8,weight=0.,split=0.))
  struct.pack_into('<I',d,44,rng.randrange(4))
  d.extend(struct.pack('<QQB',2**63,512*1024*1024,0))
 else:
  d.extend(struct.pack('<QQB',rng.choice([0,1,4,64,256,1024,4096,16384,2**63]),rng.choice([0,64,1024,65536,512*1024*1024]),rng.randrange(4)))
 values={a:c.run(d) for a,c in clients.items()}
 if values['base']!=values['candidate']:
  (art/'fuzz-divergence.input').write_bytes(d)
  for a,v in values.items():(art/f'fuzz-divergence-{a}').write_bytes(v)
  raise AssertionError((count,op))
 digest.update(struct.pack('<II',len(d),len(values['base'])));digest.update(d);digest.update(values['base']);counts[op]+=1;count+=1
 if count%10000==0:print('fuzz',count,'cpu',round(elapsed,2),flush=True)
for c in clients.values():c.close()
(art/'fuzz.json').write_text(json.dumps(dict(exit=0,cpu_seconds=elapsed,executions=count,counts=counts,stream_sha256=digest.hexdigest()),indent=2)+'\n')
print('fuzz complete',count,elapsed,flush=True)
