#!/usr/bin/env python3
"""Five CPU-minute seeded differential; replay by seed and retained count."""
import sys,os,random,struct,time,resource,hashlib,json
from proof import Client,r,FAMILIES,wide_partition
from control import ART,save,sha,LANE,ROOT

def cpu():
    a=resource.getrusage(resource.RUSAGE_SELF);b=resource.getrusage(resource.RUSAGE_CHILDREN)
    # Resident child CPU is added separately below until the children exit.
    return a.ru_utime+a.ru_stime+b.ru_utime+b.ru_stime

def proc_cpu(pid):
    s=open(f'/proc/{pid}/stat').read().rsplit(')',1)[1].split();return (int(s[11])+int(s[12]))/os.sysconf('SC_CLK_TCK')
assert os.environ.get('PARITY_RUNNER_ACTIVE')
seed=20261030;max_count=int(sys.argv[1])if len(sys.argv)>1 else None
rng=random.Random(seed);clients={a:Client(ART/f'proof-{a}')for a in ['base','ours']}
start=cpu();child_start=sum(proc_cpu(c.p.pid)for c in clients.values());wall=time.time();count=0;layouts=[0]*4;ops={n:0 for n in FAMILIES.values()};results={};digest=hashlib.sha256()
identity={a:sha(ART/f'proof-{a}')for a in clients}
while True:
    if count%512==0:
        elapsed=sum(proc_cpu(c.p.pid)for c in clients.values())-child_start
        if max_count is None and elapsed>=300:break
    if max_count is not None and count>=max_count:break
    op=rng.choice(list(FAMILIES));case_seed=seed+count
    data=bytearray(wide_partition(case_seed)if op==12 and rng.randrange(16)==0 else r.case(op,case_seed));layout=rng.randrange(4);struct.pack_into('<I',data,44,layout)
    if rng.randrange(2):struct.pack_into('<I',data,4,op|0x10000)
    nv,ni=struct.unpack_from('<II',data,8)
    # Add hostile floats, invalid topology/settings, and exact short resources.
    choice=rng.randrange(8)
    if choice==0 and nv:
        struct.pack_into('<I',data,48+rng.randrange(nv*3)*4,rng.getrandbits(32))
    elif choice==1 and ni:
        struct.pack_into('<I',data,48+nv*12+rng.randrange(ni)*4,rng.getrandbits(32))
    elif choice==2:
        struct.pack_into('<I',data,rng.choice([16,20,24,28,32,36]),rng.choice([0,1,2,3,64,256,512,0x7f800000,0x7fc00001,0xffffffff]))
    if rng.randrange(2):
        data.extend(struct.pack('<QQB',2**63,512*1024*1024,0))
    else:
        data.extend(struct.pack('<QQB',rng.choice([0,1,4,16,64,256,1024,4096,16384,2**63]),rng.choice([0,64,256,1024,4096,65536,512*1024*1024]),rng.randrange(4)))
    values={a:c.run(data)for a,c in clients.items()}
    if values['base']!=values['ours']:
        (ART/'fuzz-divergence.input').write_bytes(data)
        for a,v in values.items():(ART/f'fuzz-divergence-{a}.txt').write_bytes(v)
        save(ART/'fuzz-failure.json',{'seed':seed,'execution':count,'layout':layout,'op':op});raise SystemExit(1)
    digest.update(struct.pack('<II',len(data),len(values['ours'])));digest.update(data);digest.update(values['ours'])
    layouts[layout]+=1;ops[FAMILIES[op]]+=1
    status=values['ours'].split(b'\n',1)[0].split(b'(',1)[0].decode();results[status]=results.get(status,0)+1
    count+=1
    if count%10000==0:
        save(ART/'fuzz-progress.json',{'seed':seed,'executions':count,'cpu_seconds':elapsed,'layouts':layouts,'families':ops});print('fuzz',count,'cpu',round(elapsed,2),flush=True)
elapsed=sum(proc_cpu(c.p.pid)for c in clients.values())-child_start
for c in clients.values():c.close()
final_cpu=cpu()-start
save(ART/'fuzz.json',{'exit':0,'seed':seed,'executions':count,'cpu_seconds':final_cpu,'target_cpu_seconds':elapsed,'wall_seconds':time.time()-wall,'layouts':layouts,'families':ops,'results':results,'binary_sha256':identity,'stream_sha256':digest.hexdigest(),'generator_sha256':sha(LANE/'fuzz.py'),'existing_generator_sha256':sha(ROOT/'parity/p03/runner.py'),'replay':['python3',str(LANE/'fuzz.py'),str(count)],'method':'stable seeded mutation, exact baseline comparison; no sanitizer or coverage instrumentation'})
assert max_count is not None or elapsed>=300
print('fuzz complete',count,final_cpu,flush=True)
