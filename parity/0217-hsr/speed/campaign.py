"""Admitted deterministic conservative-raster qualification; old receipts stay intact."""
import hashlib,json,os,random,shutil,statistics,struct,subprocess,time
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'speed';target=Path(os.environ['CARGO_TARGET_DIR']);scratch=target/'speed';scratch.mkdir(parents=True,exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0')
inputs=Path(os.environ['HSR_INPUTS'])
perf=os.environ['HSR_PERF']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(name,args,**kw):
    start=time.monotonic();p=subprocess.run(list(map(str,args)),text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,**kw)
    (receipt/(name+'.log')).write_text(p.stdout.replace(str(target),'<target>').replace(str(root),'<worktree>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,p.stdout[-1200:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
    return p.stdout
old=scratch/'old';old.mkdir(exist_ok=True)
for directory in ['src','docs']:
    shutil.copytree(root/directory,old/directory,dirs_exist_ok=True)
for name in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(root/name,old/name)
shutil.copy2(receipt/'before.rs',old/'src/hidden.rs')
binaries={}
for arm,source,features in [('before',old,['std','simd']),('after',root,['std','simd']),('none',root,[]),('parallel',root,['std','simd','parallel'])]:
    folder=scratch/('build-'+arm);folder.mkdir(exist_ok=True)
    (folder/'Cargo.toml').write_text(f'''[package]
name="hsr-speed"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path={json.dumps(str(source))},default-features=false,features={json.dumps(features)}}}
[[bin]]
name="hsr-speed"
path={json.dumps(str(receipt/'bench.rs'))}
[profile.release]
codegen-units=1
lto="fat"
''')
    run('build-'+arm,['cargo','build','--offline','--release','--manifest-path',folder/'Cargo.toml'])
    shutil.copy2(target/'release/hsr-speed',scratch/arm);binaries[arm]=sha(scratch/arm)
# Larger separated-triangle scaling also reports wall time; work is the
# deterministic gate, matching the original F8 scaling protocol.
scale_paths={}
for n in [32768,262144]:
    path=scratch/('scale-'+str(n)+'.bin')
    with path.open('wb') as f:
        f.write(struct.pack('<II',3*n,3*n))
        for j in range(n):
            x=float(j*3)
            for v in [(x,0.,0.),(x+1.,0.,0.),(x,1.,0.)]:f.write(struct.pack('<3f',*v))
        for j in range(3*n):f.write(struct.pack('<I',j))
    scale_paths[n]=path
scale_rows=[]
for rep in range(3):
    for n in [32768,262144] if rep!=1 else [262144,32768]:
        out=scratch/('scale-'+str(n)+'.out')
        log=run('scaling-'+str(n)+'-'+str(rep),[scratch/'after','hidden',scale_paths[n],out],input='1\nstop\n')
        lines=log.splitlines();report=json.loads(lines[0].split(' ',2)[2]);elapsed=float(next(line.split()[1] for line in lines if line.startswith('COUNTED')))
        scale_rows.append(dict(n=n,rep=rep,seconds=elapsed,work=report['work'],output_sha256=sha(out)))
scale={}
for n in [32768,262144]:
    cell=[r for r in scale_rows if r['n']==n];assert len({r['output_sha256'] for r in cell})==1
    scale[str(n)]=dict(seconds=statistics.median(r['seconds'] for r in cell),work=cell[0]['work'])
scale['work_ratio']=scale['262144']['work']/scale['32768']['work']
scale['wall_ratio']=scale['262144']['seconds']/scale['32768']['seconds']
assert scale['work_ratio']<=10,scale
(receipt/'scaling.json').write_text(json.dumps(dict(summary=scale,rows=scale_rows),indent=2)+'\n')
run('oracle-build',['c++','-std=c++17','-O2','-ffp-contract=off',lane/'oracle.cpp','-o',scratch/'oracle'])
# Independent C++ oracle and preserved old implementation on the same 80 scenes.
rng=random.Random(217);scenes=scratch/'scenes';scenes.mkdir(exist_ok=True);observations=0;extra=0
for case in range(80):
    n=1+case%24;p=[];idx=[]
    for t in range(n):
        p.extend([[rng.randrange(-8,9)/4 for _ in range(3)] for _ in range(3)]);idx.extend([t*3,t*3+1,t*3+2])
    if case%2==0:
        offset=len(p);p.extend([[x,y,z] for z in [-4.,4.] for y in [-4.,4.] for x in [-4.,4.]])
        idx.extend(offset+v for v in [0,3,1,0,2,3,4,5,7,4,7,6,0,1,5,0,5,4,1,3,7,1,7,5,3,2,6,3,6,7,2,0,4,2,4,6])
    path=scenes/(str(case)+'.bin');path.write_bytes(struct.pack('<II',len(p),len(idx))+b''.join(struct.pack('<3f',*v) for v in p)+struct.pack('<'+'I'*len(idx),*idx))
    for s in [1,2,4]:
        expected=scratch/'oracle.out';subprocess.run([str(scratch/'oracle'),str(path),str(expected),str(s)],check=True)
        expected_bytes=expected.read_bytes();expected_tris=set(struct.iter_unpack('<III',expected_bytes));hashes=[]
        for arm in ['before','after','none','parallel']:
            actual=scratch/(arm+'.out');pr=subprocess.run([str(scratch/arm),'hidden',str(path),str(actual)],input='stop\n',text=True,capture_output=True,env=dict(os.environ,HSR_SUBDIVISIONS=str(s)))
            assert pr.returncode==0,(case,s,arm,pr.stderr)
            data=actual.read_bytes()
            if arm=='before':assert data==expected_bytes,(case,s,arm)
            else:
                actual_tris=set(struct.iter_unpack('<III',data));assert expected_tris<=actual_tris,(case,s,arm)
                extra+=len(actual_tris-expected_tris);hashes.append(sha(actual))
            observations+=1
        assert len(set(hashes))==1,(case,s,hashes)
(receipt/'oracle.json').write_text(json.dumps(dict(exit=0,scenes=80,subdivisions=[1,2,4],observations=observations,candidate_checks=720,extra_kept_total=extra,criterion='old exact C++; candidate superset and feature-mode identity',oracle_sha256=sha(scratch/'oracle')))+'\n')
print('PASS conservative C++/old differential',observations,flush=True)
expected=json.loads((root/'parity/peers/inputs.json').read_text())
for row in expected:assert sha(inputs/(row['case']+'.bin'))==row['sha256']
(receipt/'inputs.json').write_text(json.dumps(expected,indent=2)+'\n')
(receipt/'binaries.json').write_text(json.dumps(binaries,indent=2)+'\n')
cpu=min(os.sched_getaffinity(0));rows=[]
# Full paired/interleaved medians on real meshes, all corpus cases after only.
for case in expected:
    name=case['case'];arms=[('after','hidden')]
    if name.startswith('real-'):arms=[('before','hidden'),('after','hidden'),('after','simplify')]
    outputs={}
    for rep in range(3):
        for arm,op in arms if rep!=1 else list(reversed(arms)):
            label=f'{name}-{arm}-{op}-{rep}';prefix=scratch/label;ctl=str(prefix)+'.ctl';ack=str(prefix)+'.ack';os.mkfifo(ctl);os.mkfifo(ack)
            outfile=scratch/(label+'.out');statfile=scratch/(label+'.perf')
            cmd=['taskset','-c',str(cpu),perf,'stat','-x',';','-e','instructions:u','-o',str(statfile),'-D','-1','--control=fifo:'+ctl+','+ack,'--',str(scratch/arm),op,str(inputs/(name+'.bin')),str(outfile)]
            p=subprocess.Popen(cmd,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=dict(os.environ,HSR_CTL=ctl,HSR_ACK=ack))
            first=p.stdout.readline();assert first.startswith('RESULT'),(label,first,p.stderr.read());report=json.loads(first.split(' ',2)[2]);assert p.stdout.readline().strip()=='READY'
            calls=1;p.stdin.write('1\n');p.stdin.flush();counted=p.stdout.readline();assert counted.startswith('COUNTED'),(label,counted,p.stderr.read())
            p.stdin.write('stop\n');p.stdin.flush();_,err=p.communicate();assert p.returncode==0,(label,err)
            stat=statfile.read_text();fields=next(line.split(';') for line in stat.splitlines() if ';instructions:u;' in line)
            assert float(fields[4])>=99.0,fields
            row=dict(case=name,arm=arm,op=op,rep=rep,wall_seconds=float(counted.split()[1]),instructions=int(fields[0]),report=report,output_sha256=sha(outfile))
            rows.append(row)
            with (receipt/'counts.jsonl').open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
            (receipt/(label+'.perf')).write_text(stat)
            print(label,row['wall_seconds'],report,flush=True)
            if rep==0 and op=='hidden':outputs[arm]=outfile
    if name.startswith('real-'):
        # Unique ordinal indices in real fixtures: exact kept-triangle subset.
        # Match sorted original triangle ordinals instead of two large tuple sets.
        old_data=outputs['before'].read_bytes();new_data=outputs['after'].read_bytes()
        cursor=0
        for (a,b,c) in struct.iter_unpack('<III',old_data):
            tri=struct.pack('<III',a,b,c)
            while new_data[cursor:cursor+12]!=tri:
                cursor+=12;assert cursor<len(new_data),(name,cursor)
            cursor+=12
    # Feature modes must agree on complete large/corpus outputs, too.
    candidate_hash=next(r['output_sha256'] for r in rows if r['case']==name and r['arm']=='after' and r['op']=='hidden')
    for arm in ['none','parallel']:
        out=scratch/(name+'-'+arm+'.out');run('identity-'+name+'-'+arm,[scratch/arm,'hidden',inputs/(name+'.bin'),out],input='stop\n')
        assert sha(out)==candidate_hash,(name,arm)
summary=[]
for case in expected:
    name=case['case'];cell=[r for r in rows if r['case']==name];entry={'case':name}
    for arm,op in [('before','hidden'),('after','hidden'),('after','simplify')]:
        group=[r for r in cell if r['arm']==arm and r['op']==op]
        if not group:continue
        assert len({r['output_sha256'] for r in group})==1,(name,arm,op)
        entry[arm+'-'+op]=dict(seconds=statistics.median(r['wall_seconds'] for r in group),instructions=statistics.median(r['instructions'] for r in group),report=group[0]['report'])
    if name.startswith('real-'):
        entry['vs_simplify']=entry['after-hidden']['seconds']/entry['after-simplify']['seconds']
    summary.append(entry)
largest=next(e for e in summary if e['case']=='real-a082');passed=largest['after-hidden']['seconds']<=3 and largest['vs_simplify']<=3
(receipt/'performance.json').write_text(json.dumps(dict(rows=summary,gate=passed,cpu=cpu,repetitions=3,counter='instructions:u',single_thread=True),indent=2)+'\n')
assert passed,largest
print('PASS largest mesh <=3s and <=3x simplify',flush=True)
