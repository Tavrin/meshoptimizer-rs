"""Four-arm cache regression gate plus candidate-only HSR disclosure.

Supply HSR_INPUTS (eight frozen peers meshes), HSR_REFERENCE (C++ 1.3),
HSR_PERF, and the dedicated CARGO_TARGET_DIR; run through CPU admission.
HSR has no upstream/base implementation: comparisons for it are N/A, rather
than attributing a different algorithm's speed to HSR.
"""
import hashlib,io,json,os,random,shutil,statistics,struct,subprocess,tarfile,time
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'receipts';target=Path(os.environ['CARGO_TARGET_DIR'])
scratch=target/'campaign';scratch.mkdir(parents=True,exist_ok=True)
inputs=Path(os.environ['HSR_INPUTS']);reference=Path(os.environ['HSR_REFERENCE']);perf=os.environ['HSR_PERF']
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(name,args,env=None):
    start=time.monotonic();p=subprocess.run(list(map(str,args)),stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,env=env)
    (receipt/(name+'.log')).write_text(p.stdout.replace(str(target),'<target>').replace(str(root),'<worktree>').replace(str(reference),'<reference>').replace(str(inputs),'<inputs>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,p.stdout[-600:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
    return p.stdout
campaign_cpu_start=sum(os.times()[:4])
binding={}
base=scratch/'base-source';base.mkdir(exist_ok=True)
archive=subprocess.check_output(['git','archive','09da597f438471d0ad9c16e460f3598e9356e9ea','src','docs','Cargo.toml','Cargo.lock','README.md'])
with tarfile.open(fileobj=io.BytesIO(archive)) as t:t.extractall(base,filter='data')
for arm,source,features,hidden,cpp in [('base',base,['std','simd'],False,False),('candidate',root,['std','simd'],True,False),('candidate-none',root,[],True,False),('cpp-scalar',root,[],False,True),('cpp-default',root,[],False,True)]:
    folder=scratch/('build-'+arm);folder.mkdir(exist_ok=True)
    dependency='meshopt = "=0.7.0"' if cpp else f'm={{package="meshoptimizer-rs",path={json.dumps(str(source))},default-features=false,features={json.dumps(features)}}}'
    flags=['cpp'] if cpp else ['ours']+(['hidden'] if hidden else [])
    manifest=folder/'Cargo.toml'
    manifest.write_text(f'''[package]
name="hsr-driver"
version="0.0.0"
edition="2021"
[features]
ours=[]
cpp=[]
hidden=[]
[dependencies]
{dependency}
[[bin]]
name="hsr-driver"
path={json.dumps(str(lane/'driver.rs'))}
[profile.release]
codegen-units=1
lto="fat"
''')
    env=dict(os.environ,CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else '')
    run('build-'+arm,['cargo','build','--offline','--release','--manifest-path',manifest,'--features',','.join(flags)],env)
    shutil.copy2(target/'release/hsr-driver',scratch/('driver-'+arm));binding[arm]=sha(scratch/('driver-'+arm))
run('oracle-build',['c++','-std=c++17','-O2','-ffp-contract=off',lane/'oracle.cpp','-o',scratch/'oracle'])
# Check reference pin and vendored upstream 1.3 identity, not just crate version.
run('reference-check',['bash','parity/check-reference.sh'],dict(os.environ,MESHOPT_REFERENCE=str(reference)))
vendored=next(Path.home().glob('.cargo/registry/src/*/meshopt-0.7.0/vendor/src/meshoptimizer.h'),None)
if vendored is None:
    vendored=next(Path.home().glob('.cargo/registry/src/*/meshopt-0.7.0/**/meshoptimizer.h'))
cpp_sources={}
for f in vendored.parent.glob('*'):
    if f.is_file() and f.suffix in ['.h','.cpp']:
        expected=reference/'src'/f.name
        assert expected.exists() and f.read_bytes()==expected.read_bytes(),f.name
        cpp_sources[f.name]=sha(f)
(receipt/'cpp-source-binding.json').write_text(json.dumps(cpp_sources,indent=2)+'\n')
# Independent brute-force differential on bounded seeded scenes; no exact
# duplicate faces here (duplicate policy has its own focused tests).
rng=random.Random(217);oracle=scratch/'oracle-inputs';oracle.mkdir(exist_ok=True);observations=0
for case in range(80):
    n=1+case%24;p=[];idx=[]
    for t in range(n):
        p.extend([[rng.randrange(-8,9)/4 for _ in range(3)] for _ in range(3)])
        idx.extend([t*3,t*3+1,t*3+2])
    if case%2==0:
        offset=len(p)
        p.extend([[x,y,z] for z in [-4.,4.] for y in [-4.,4.] for x in [-4.,4.]])
        # Standard cube, with this loop's binary corner numbering.
        idx.extend(offset+v for v in [0,3,1,0,2,3,4,5,7,4,7,6,0,1,5,0,5,4,1,3,7,1,7,5,3,2,6,3,6,7,2,0,4,2,4,6])
    path=oracle/(str(case)+'.bin');path.write_bytes(struct.pack('<II',len(p),len(idx))+b''.join(struct.pack('<3f',*v) for v in p)+struct.pack('<'+'I'*len(idx),*idx))
    for s in [1,2,4]:
        expected=scratch/'oracle.out';subprocess.run([str(scratch/'oracle'),str(path),str(expected),str(s)],check=True)
        for arm in ['candidate','candidate-none']:
            actual=scratch/'rust.out';pr=subprocess.run([str(scratch/('driver-'+arm)),'hidden',str(path),str(actual)],input='stop\n',text=True,capture_output=True,env=dict(os.environ,HSR_SUBDIVISIONS=str(s)))
            assert pr.returncode==0,pr.stderr
            assert actual.read_bytes()==expected.read_bytes(),(case,s,arm)
            observations+=1
(receipt/'oracle.json').write_text(json.dumps(dict(exit=0,scenes=80,subdivisions=[1,2,4],modes=2,observations=observations,oracle_sha256=sha(scratch/'oracle')))+'\n')
print('PASS independent C++ brute-force oracle',observations,flush=True)
expected=json.loads((root/'parity/peers/inputs.json').read_text())
for row in expected:assert sha(inputs/(row['case']+'.bin'))==row['sha256']
(receipt/'inputs.json').write_text(json.dumps(expected,indent=2)+'\n')
(receipt/'binaries.json').write_text(json.dumps(binding,indent=2)+'\n')
# Isolate counters to warm resident algorithm calls using perf's FIFO controls.
rows=[];cpu=min(os.sched_getaffinity(0))
for op in ['cache','hidden']:
    for case in expected:
        name=case['case'];arms=['base','candidate','cpp-scalar','cpp-default'] if op=='cache' else ['candidate']
        for rep in range(3):
            for arm in arms if rep!=1 else list(reversed(arms)):
                label=f'{op}-{name}-{arm}-{rep}';prefix=scratch/label;ctl=str(prefix)+'.ctl';ack=str(prefix)+'.ack'
                os.mkfifo(ctl);os.mkfifo(ack)
                outfile=str(prefix)+'.out';statfile=str(prefix)+'.perf'
                command=['taskset','-c',str(cpu),perf,'stat','-x',';','-e','instructions:u','-o',statfile,'-D','-1','--control=fifo:'+ctl+','+ack,'--',str(scratch/('driver-'+arm)),op,str(inputs/(name+'.bin')),outfile]
                start=time.monotonic();p=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=dict(os.environ,HSR_CTL=ctl,HSR_ACK=ack))
                lines=[]
                while True:
                    line=p.stdout.readline();lines.append(line)
                    if line.strip()=='READY':break
                    if not line:
                        raise RuntimeError((label,p.wait(),p.stderr.read()))
                calls=1 if name.startswith('real-') or op=='hidden' else 10
                p.stdin.write(str(calls)+'\n');p.stdin.flush();line=p.stdout.readline();lines.append(line);assert line.startswith('COUNTED'),(label,line,p.stderr.read())
                p.stdin.write('stop\n');p.stdin.flush();stdout,stderr=p.communicate();assert p.returncode==0,(label,stderr)
                fields=next(line.split(';') for line in Path(statfile).read_text().splitlines() if ';instructions:u;' in line)
                count=int(fields[0]);elapsed=float(line.split()[1])
                report=json.loads(lines[0].split(' ',2)[2]) if op=='hidden' else None
                row=dict(op=op,case=name,arm=arm,rep=rep,calls=calls,instructions_per_call=count/calls,wall_seconds_per_call=elapsed/calls,setup_and_run_seconds=time.monotonic()-start,output_sha256=sha(Path(outfile)),report=report)
                rows.append(row)
                with (receipt/'counts.jsonl').open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
                (receipt/(label+'.perf')).write_text(Path(statfile).read_text())
                print(label,round(count/calls),f'{elapsed/calls:.3f}s',report,flush=True)
# Every upstream/cache arm has exact output identity on every measured mesh.
summary=[]
for case in expected:
    name=case['case'];cell=[r for r in rows if r['case']==name and r['op']=='cache'];assert len({r['output_sha256'] for r in cell})==1
    med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in ['base','candidate','cpp-scalar','cpp-default']}
    summary.append(dict(case=name,instructions=med,candidate_vs_base=med['candidate']/med['base']))
    # no-default-feature C++ differential on the same complete meshes
    actual=scratch/'none.out'
    pr=subprocess.run([str(scratch/'driver-candidate-none'),'cache',str(inputs/(name+'.bin')),str(actual)],input='stop\n',text=True,capture_output=True);assert pr.returncode==0,pr.stderr
    assert sha(actual)==cell[0]['output_sha256']
hidden=[]
for case in expected:
    cell=[r for r in rows if r['case']==case['case'] and r['op']=='hidden']
    assert len({r['output_sha256'] for r in cell})==1
    assert len({json.dumps(r['report'],sort_keys=True) for r in cell})==1
    report=cell[0]['report']
    hidden.append(dict(case=case['case'],report=report,removed_share=report['removed']/report['input'],median_wall_seconds=statistics.median(r['wall_seconds_per_call'] for r in cell),median_instructions=statistics.median(r['instructions_per_call'] for r in cell)))
result=dict(cache=summary,hidden=hidden,hsr_comparison='N/A: extension absent from v0.2.16 and upstream C++ 1.3',cache_regression_gate=all(r['candidate_vs_base']<=1.01 for r in summary),cpu=cpu,median_repetitions=3,campaign_cpu_seconds=sum(os.times()[:4])-campaign_cpu_start)
(receipt/'performance.json').write_text(json.dumps(result,indent=2)+'\n')
assert result['cache_regression_gate'],result
print('PASS four-arm cache identity/regression, both Rust feature modes; candidate HSR disclosure complete',flush=True)
