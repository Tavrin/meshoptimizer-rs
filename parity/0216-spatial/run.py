"""Reproducible CPU-only spatial SIMD qualification. All paths are supplied by the caller."""
import hashlib, importlib.util, io, json, math, os, resource, shutil, statistics, struct, subprocess, sys, tarfile, time
from pathlib import Path
R=Path(__file__).resolve().parents[2]
L=Path(__file__).resolve().parent
S=Path(os.environ['MESHOPT_SCRATCH'])
T=Path(os.environ['CARGO_TARGET_DIR'])
A=S/'art'
REF=Path(os.environ['MESHOPT_REFERENCE'])
os.environ.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',LC_ALL='C',PARITY_RUNNER_ACTIVE='1',PARITY_RUNNER_GPU='0')
S.mkdir(exist_ok=True,parents=True); A.mkdir(exist_ok=True)
def sha(p):
    with Path(p).open('rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()
def run(name,cmd,**kw):
    log=A/(name+'.log')
    attempt=1
    while log.exists():
        attempt+=1; log=A/(name+f'-attempt{attempt}.log')
    before=resource.getrusage(resource.RUSAGE_CHILDREN); start=time.monotonic()
    with log.open('x') as f: p=subprocess.run(list(map(str,cmd)),cwd=R,stdout=f,stderr=subprocess.STDOUT,**kw)
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    row=dict(name=name,exit=p.returncode,wall_seconds=time.monotonic()-start,
        cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,log_sha256=sha(log))
    with (A/'exits.jsonl').open('a') as f: f.write(json.dumps(row)+'\n')
    print(name,p.returncode,round(row['cpu_seconds'],2),flush=True)
    if p.returncode: print(log.read_text()[-4000:],flush=True); raise SystemExit(p.returncode)
def prepare():
    for name in ['base','candidate']:
        tree=S/name; tree.mkdir()
        if name=='base':
            data=subprocess.check_output(['git','archive','1ecf69abe9f92c17e13dc38fa97be659aee44582'],cwd=R)
            with tarfile.open(fileobj=io.BytesIO(data)) as tar: tar.extractall(tree,filter='data')
        else:
            for part in ['src','docs','parity/peers','parity/p03']: shutil.copytree(R/part,tree/part)
            for part in ['Cargo.toml','Cargo.lock','README.md','LICENSE']: shutil.copy2(R/part,tree/part)
        # Add spatial to the established allocation/serialization consumer, retaining wall timing.
        peer=tree/'parity/peers/src/main.rs'
        subprocess.run([sys.executable,str(R/'parity/0216-qualify/prepare_driver.py'),str(peer)],check=True)
        s=peer.read_text().replace('control("enable");for _','control("enable");let start=Instant::now();for _')
        s=s.replace('control("disable");println!("COUNTED")','let elapsed=start.elapsed().as_secs_f64();control("disable");println!("COUNTED {elapsed:.9}")')
        peer.write_text(s)
        manifest=tree/'parity/peers/Cargo.toml'
        s=manifest.read_text().replace('features = ["clusterlod"]','default-features = false, features = ["std", "clusterlod"]')
        s=s.replace('ours = ["dep:meshoptimizer-rs"]','ours = ["dep:meshoptimizer-rs"]\nvector = ["ours", "meshoptimizer-rs/simd"]')
        manifest.write_text(s)
        p03=tree/'parity/p03/Cargo.toml'
        s=p03.read_text().replace('[dependencies]','[features]\nvector = ["meshoptimizer-rs/std", "meshoptimizer-rs/simd", "meshoptimizer-rs/parity-internals"]\n[dependencies]')
        p03.write_text(s)
        # Force individual native ceilings without exporting an ISA token.
        p=tree/'parity/p03/src/main.rs'; s=p.read_text()
        s=s.replace('let result = meshopt_p03_parity::execute(&b).unwrap();', '''let execute = || meshopt_p03_parity::execute(&b).unwrap();
        #[cfg(feature = "vector")]
        let result = {
            use meshoptimizer_rs::codec::{Level, with_level};
            let level = match std::env::var("SPATIAL_LEVEL").as_deref() {
                Ok("scalar") => Level::Scalar, Ok("sse2") => Level::Sse2,
                Ok("ssse3") => Level::Ssse3, _ => Level::Sse41,
            };
            with_level(level, execute).unwrap()
        };
        #[cfg(not(feature = "vector"))]
        let result = execute();''')
        s=s.replace('128 * 1024 * 1024','512 * 1024 * 1024');p.write_text(s)
        p=tree/'parity/p03/src/lib.rs';p.write_text(p.read_text().replace('128 * 1024 * 1024','512 * 1024 * 1024'))
    base=S/'base/Cargo.toml'
    base.write_text(base.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
    # Only the exact driver links the renamed base. Restore its consumers' package alias.
    for m in [S/'base/parity/peers/Cargo.toml',S/'base/parity/p03/Cargo.toml']:
        m.write_text(m.read_text().replace('meshoptimizer-rs = {','meshoptimizer-rs = { package = "meshoptimizer-baseline",'))
    e=S/'exact'; e.mkdir()
    shutil.copy2(L/'exact.rs',e/'main.rs')
    features=', '.join('"'+arm+'/'+f+'"' for arm in ['baseline','candidate'] for f in ['std','simd','parity-internals'])
    (e/'Cargo.toml').write_text(f'''[package]
name="spatial-exact"
version="0.0.0"
edition="2021"
[features]
all=[{features}]
[dependencies]
baseline={{package="meshoptimizer-baseline",path="../base",default-features=false}}
candidate={{package="meshoptimizer-rs",path="../candidate",default-features=false}}
[[bin]]
name="spatial-exact"
path="main.rs"
[profile.release]
lto="thin"
codegen-units=1
''')
    source=Path(os.environ['MESHOPT_INPUTS'])
    shutil.copytree(source/'inputs',A/'inputs',copy_function=os.link)
    shutil.copy2(source/'inputs.json',A/'inputs.json')
    binding={str(p.relative_to(S)):sha(p) for name in ['base','candidate','exact'] for p in (S/name).rglob('*') if p.is_file()}
    (A/'source-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
def build():
    binaries={}
    for arm,tree,feature,flags in [('base','base','vector',''),('candidate','candidate','vector',''),
        ('scalar','candidate','ours',''),('cpp-scalar','candidate','cpp','-DMESHOPTIMIZER_NO_SIMD'),
        ('cpp-default','candidate','cpp','')]:
        m=S/tree/'parity/peers/Cargo.toml'
        env=dict(os.environ,CXXFLAGS=flags+' -fno-fast-math -ffp-contract=off')
        run('build-'+arm,['cargo','build','--offline','--release','--manifest-path',m,'--no-default-features','--features',feature],env=env)
        shutil.copy2(T/'release/meshopt-peers',A/('driver-'+arm))
        binaries[arm]=sha(A/('driver-'+arm))
    sources={str(p.relative_to(S)):sha(p) for tree in ['base','candidate'] for p in [*(S/tree/'src').rglob('*.rs'),S/tree/'Cargo.toml',S/tree/'Cargo.lock',S/tree/'parity/peers/Cargo.toml',S/tree/'parity/peers/Cargo.lock',S/tree/'parity/peers/src/main.rs']}
    (A/'build-binding.json').write_text(json.dumps(dict(binaries=binaries,sources=sources),indent=2)+'\n')
def exact():
    m=S/'exact/Cargo.toml'
    for mode,flags in [('none',[]),('all',['--features','all'])]:
        run('exact-build-'+mode,['cargo','build','--offline','--release','--manifest-path',m,*flags])
        shutil.copy2(T/'release/spatial-exact',A/('exact-'+mode))
        run('exact-size-'+mode,[A/('exact-'+mode)],env=dict(os.environ,SPATIAL_SIZE='1'))
        run('exact-'+mode,[A/('exact-'+mode)])
def checks():
    checks=[
        ('tests-all',['cargo','test','--offline','--locked','--all-features']),
        ('tests-none',['cargo','test','--offline','--locked','--no-default-features']),
        ('clippy',['cargo','clippy','--offline','--locked','--all-features','--all-targets','--','-D','warnings']),
        ('fmt',['cargo','fmt','--check']),
        ('boundary',['bash','parity/check-unsafe-boundary.sh']),
        ('public',['bash','parity/tools/check_public_text.sh']),
        ('neon',['cargo','check','--offline','--locked','--target','aarch64-unknown-linux-gnu','--all-features'])]
    for name,cmd in checks: run(name,cmd)
    run('unsafe-free',['cargo','check','--offline','--locked','--no-default-features'],env=dict(os.environ,RUSTFLAGS='-Funsafe_code'))
def p03build():
    sources=[REF/'src'/f'{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
    run('reference',['bash','parity/check-reference.sh'])
    transport=A/'p03-reference.cpp'
    transport.write_text((R/'parity/p03/reference.cpp').read_text().replace('128*1024*1024','512*1024*1024'))
    for mode in ['scalar','default']:
        run('p03-cpp-'+mode,['c++','-std=c++17','-O3','-fno-fast-math','-ffp-contract=off',*(['-DMESHOPTIMIZER_NO_SIMD'] if mode=='scalar' else []),'-I'+str(REF/'src'),'-I'+str(REF/'demo'),transport,*sources,'-o',A/('p03-cpp-'+mode)])
    m=S/'candidate/parity/p03/Cargo.toml'
    for mode,flags in [('none',[]),('all',['--features','vector'])]:
        run('p03-build-'+mode,['cargo','build','--offline','--release','--manifest-path',m,*flags])
        shutil.copy2(T/'release/meshopt-p03-driver',A/('p03-'+mode))
        env=dict(os.environ,RUSTFLAGS='-C target-feature=+simd128' if mode=='all' else '')
        run('wasm-build-'+mode,['cargo','build','--offline','--release','--target','wasm32-unknown-unknown','--manifest-path',m,'--lib',*flags],env=env)
        shutil.copy2(T/'wasm32-unknown-unknown/release/meshopt_p03_parity.wasm',A/('p03-'+mode+'.wasm'))

    env=dict(os.environ,RUSTFLAGS='-C target-feature=+simd128')
    run('wasm-kernel-tests-build',['cargo','test','--offline','--locked','--target','wasm32-wasip1','--lib','--no-run','--features','parity-internals'],env=env)
    modules=list((T/'wasm32-wasip1/debug/deps').glob('meshoptimizer_rs-*.wasm'))
    assert len(modules)==1,modules
    shutil.copy2(modules[0],A/'kernel-tests.wasm')
    run('wasm-kernel-tests',['node',L/'wasi.cjs',A/'kernel-tests.wasm','meshlet_spatial::simd_tests','--nocapture'])

def differential():
    spec=importlib.util.spec_from_file_location('runner',R/'parity/p03/runner.py'); r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
    clients={'cpp':r.Client('cpp',A/'p03-cpp-scalar'),'cpp-default':r.Client('cpp-default',A/'p03-cpp-default'),'none':r.Client('none',A/'p03-none')}
    # r.Client uses a frozen ENV; supply each child's explicit ceiling.
    for level in ['scalar','sse2','ssse3','sse41']:
        r.ENV['SPATIAL_LEVEL']=level
        clients[level]=r.Client(level,A/'p03-all')
    for mode in ['none','all']: clients['wasm-'+mode]=r.Client('wasm',A/('p03-'+mode+'.wasm'))
    folder=A/'differential'
    attempt=1
    while folder.exists():
        attempt+=1;folder=A/f'differential-attempt{attempt}'
    folder.mkdir()
    count=0
    try:
        with (folder/'corpus.bin').open('wb') as records:
            for seed in range(20261004,20262004):
                data=r.case(4,seed)
                for into in [False,True]:
                    b=bytearray(data)
                    if into: struct.pack_into('<I',b,4,4|0x10000)
                    r.compare(clients,b,str(count),folder,records);count+=1
            for n in range(1,8):
                for scale in [0.,1e-30,1.,1e30]:
                    p,idx=r.grid(n+2)
                    p=[tuple((-0. if (i+k)%2 else 0.) if scale==0 else x*scale for k,x in enumerate(v)) for i,v in enumerate(p)]
                    data=r.message(4,p,idx,mv=8,min=1,mt=n,split=0.5)
                    r.compare(clients,data,str(count),folder,records);count+=1
        # Real meshes use the same frozen data as measurements; retain hashes, not duplicate assets.
        reals=[]
        for x in json.loads((A/'inputs.json').read_text()):
            if not x['case'].startswith('real-'): continue
            data=(A/'inputs'/ (x['case']+'.bin')).read_bytes()
            nv,ni=struct.unpack_from('<II',data); end=8+nv*12+ni*4
            msg=struct.pack('<4s6I2f3I',b'MO03',4,nv,ni,64,41,124,0.,0.5,4,0,0)+data[8:end]+bytes(nv*4)
            outputs={name:c.run(msg)[0] for name,c in clients.items()}
            assert len({hashlib.sha256(v).hexdigest() for v in outputs.values()})==1,x['case']
            reals.append(dict(case=x['case'],input_sha256=x['sha256'],output_sha256=hashlib.sha256(outputs['cpp']).hexdigest()))
            print('differential',x['case'],'PASS',flush=True)
    finally:
        failures=[]
        for name,c in clients.items():
            try:c.close()
            except (AssertionError,BrokenPipeError):failures.append((name,c.proc.poll()))
        if failures: print('client failures',failures,flush=True)
    (A/'differential.json').write_text(json.dumps(dict(exit=0,corpus_cases=count,corpus_sha256=sha(folder/'corpus.bin'),real=reals),indent=2)+'\n')
def measure():
    changed=False
    for tree in ['base','candidate']:
        manifest=S/tree/'parity/peers/Cargo.toml'
        text=manifest.read_text()
        updated=text.replace(', "parity-internals"','')
        if updated!=text:
            changed=True;manifest.write_text(updated)
    if changed:
        for arm in ['base','candidate','scalar','cpp-scalar','cpp-default']:
            shutil.copy2(A/('driver-'+arm),A/('diagnostic-driver-'+arm))
        build()
    perf=os.environ['PEERS_PERF'];cpu=min(os.sched_getaffinity(0));arms=['base','candidate','scalar','cpp-scalar','cpp-default']
    (A/'samples').mkdir(exist_ok=True); rows=[]
    for x in json.loads((A/'inputs.json').read_text()):
        for rep in range(3):
            for arm in (arms if rep!=1 else arms[::-1]):
                prefix=A/'samples'/f"{x['case']}-{arm}-{rep}"
                n=1000 if x['case']=='tiny-smooth' else 3 if x['case'].startswith('medium') else 1
                cr,cw=os.pipe(); ar,aw=os.pipe()
                cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c',str(cpu),str(A/('driver-'+arm)),'meshlets-spatial',str(A/'inputs'/(x['case']+'.bin')),str(prefix)+'.out']
                try:
                    p=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True)
                finally:
                    for fd in [cr,cw,ar,aw]:os.close(fd)
                Path(str(prefix)+'.log').write_text(p.stdout+p.stderr)
                assert p.returncode==0,(arm,p.stderr)
                lines=p.stdout.splitlines();assert lines[0]=='READY' and lines[1].startswith('COUNTED ')
                fields=next(l.split(';') for l in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in l)
                assert float(fields[4])==100.
                row=dict(case=x['case'],arm=arm,rep=rep,repetitions=n,exit=p.returncode,running_percent=float(fields[4]),instructions_per_call=int(fields[0])/n,wall_seconds_per_call=float(lines[1].split()[1])/n,output_sha256=sha(str(prefix)+'.out'))
                rows.append(row)
                with (A/'counts.jsonl').open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
        assert len({r['output_sha256'] for r in rows if r['case']==x['case']})==1
        print('measure',x['case'],'complete',flush=True)
    cells=[]
    for x in json.loads((A/'inputs.json').read_text()):
        med={arm:statistics.median(r['instructions_per_call'] for r in rows if r['case']==x['case'] and r['arm']==arm) for arm in arms}
        wall={arm:statistics.median(r['wall_seconds_per_call'] for r in rows if r['case']==x['case'] and r['arm']==arm) for arm in arms}
        cells.append(dict(case=x['case'],instructions=med,wall_seconds=wall,vs_base=med['candidate']/med['base'],scalar_vs_cpp=med['scalar']/med['cpp-scalar'],default_vs_cpp=med['candidate']/med['cpp-default']))
    real=[c for c in cells if c['case'].startswith('real-')]
    gm={k:math.exp(sum(math.log(c[k]) for c in real)/len(real)) for k in ['vs_base','scalar_vs_cpp','default_vs_cpp']}
    result=dict(exit=0,samples=len(rows),cpu=cpu,cells=cells,real_geomean=gm,verdict='SHIPS' if gm['vs_base']<=.95 and max(c['vs_base'] for c in cells)<=1.01 else 'DOES NOT SHIP')
    (A/'measurements.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(gm),result['verdict'],flush=True)
def main():
    resource.setrlimit(resource.RLIMIT_CPU,(7200,7200))
    runner=Path(__file__).read_bytes(); runner_digest=hashlib.sha256(runner).hexdigest()
    (A/('runner-'+runner_digest+'.py')).write_bytes(runner)
    before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic();code=0
    try:globals()[sys.argv[1]]()
    except BaseException as error:
        code=error.code if isinstance(error,SystemExit) and isinstance(error.code,int) else 1
        raise
    finally:
        after=resource.getrusage(resource.RUSAGE_CHILDREN)
        with (A/'stage-exits.jsonl').open('a') as f:
            f.write(json.dumps(dict(stage=sys.argv[1],exit=code,wall_seconds=time.monotonic()-start,
                child_cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,runner_sha256=runner_digest))+'\n')
if __name__=='__main__':main()

