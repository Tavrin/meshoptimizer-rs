"""Combined train qualification. Invoke each stage through CPU admission.

Environment: CARGO_TARGET_DIR (fresh disposable storage), MESHOPT_REFERENCE,
QUALIFY_INPUTS (retained 0.2.15 inputs). No fuzz entry point is invoked.
"""
import hashlib, importlib.util, io, json, os, resource, shutil, statistics, struct
import subprocess, sys, tarfile, time
from pathlib import Path

R = Path(__file__).resolve().parents[3]
A = Path(__file__).resolve().parent
T = Path(os.environ['CARGO_TARGET_DIR'])
REF = Path(os.environ['MESHOPT_REFERENCE'])
ALL = ['std', 'simd', 'parallel', 'clusterlod', 'experimental', 'parity-internals']
OPS = 'simplify simplify-attributes simplify-sloppy simplify-update simplify-prune simplify-points cache overdraw fetch meshlets meshlets-flex meshlets-spatial cluster-lod vertex-encode vertex-decode index-encode index-decode remap-vertex remap-index'.split()
ARMS = ['base-scalar', 'train-scalar', 'base-default', 'train-default', 'cpp-scalar', 'cpp-default', 'optimesh']
os.environ.update(CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='2', LC_ALL='C', PYTHONDONTWRITEBYTECODE='1')
os.environ.pop('CARGO_ENCODED_RUSTFLAGS', None)
os.environ.update(RUSTFLAGS='', RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='')

def sha(p):
    with Path(p).open('rb') as f: return hashlib.file_digest(f, 'sha256').hexdigest()

def write(p, x):
    Path(p).parent.mkdir(parents=True, exist_ok=True)
    Path(p).write_text(json.dumps(x, indent=2, sort_keys=True) + '\n')

def clean(s):
    for p, label in [(str(T), '<target>'), (str(R), '<worktree>'), (str(REF), '<reference>'), (str(Path.home()), '<home>')]:
        s = s.replace(p, label)
    return s

def floor():
    while shutil.disk_usage(T).free < 100 * 1024**3:
        print('Build paused: disk below 100 GiB floor', flush=True)
        time.sleep(30)

def run(name, cmd, env=None, estimate=600):
    assert estimate <= 7200
    floor()
    logs = A/'logs'; logs.mkdir(exist_ok=True)
    n=1
    while (logs/f'{name}-{n}.log').exists(): n+=1
    log=logs/f'{name}-{n}.log'; raw=T/'raw.log'
    # Estimate recorded before execution; completed process CPU is retained.
    with (A/'sizes.jsonl').open('a') as f:
        f.write(json.dumps(dict(name=name, cpu_seconds_upper_estimate=estimate, command=clean(' '.join(map(str,cmd)))))+'\n')
    before=resource.getrusage(resource.RUSAGE_CHILDREN); start=time.monotonic()
    with raw.open('w') as f:
        p=subprocess.run(list(map(str,cmd)),stdout=f,stderr=subprocess.STDOUT,env=dict(os.environ,**(env or {})))
    log.write_text(clean(raw.read_text(errors='replace')));raw.unlink()
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    row=dict(name=name,attempt=n,exit=p.returncode,wall_seconds=time.monotonic()-start,
             cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,log=str(log.relative_to(A)),log_sha256=sha(log))
    with (A/'exits.jsonl').open('a') as f: f.write(json.dumps(row)+'\n'); f.flush(); os.fsync(f.fileno())
    print(name,p.returncode,round(row['cpu_seconds'],2),flush=True)
    if p.returncode: print(log.read_text()[-5000:],flush=True);raise SystemExit(p.returncode)
    assert row['cpu_seconds']<=7200, row
    return log

def prepare():
    assert not T.exists(), 'target must be fresh'
    T.mkdir(parents=True)
    assert subprocess.check_output(['git','rev-parse','--abbrev-ref','HEAD'],text=True).strip()=='train/0.2.16'
    archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
    b=T/'base';b.mkdir()
    tarfile.open(fileobj=io.BytesIO(archive)).extractall(b,filter='data')
    p=b/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
    inputs=json.loads((R/'parity/0215-qualify/campaign/campaign.json').read_text())['inputs']
    source=Path(os.environ['QUALIFY_INPUTS']); (T/'inputs').mkdir()
    for x in inputs:
        src=source/(x['case']+'.bin');assert sha(src)==x['sha256'],x
        os.link(src,T/'inputs'/src.name)
    write(A/'inputs.json',inputs)
    write(A/'start.json',dict(head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        base=subprocess.check_output(['git','rev-parse','v0.2.15^{}'],text=True).strip(),
        sources={str(p.relative_to(R)):sha(p) for p in (R/'src').rglob('*') if p.is_file()},
        helpers={str(p.relative_to(R)):sha(p) for p in (R/'parity').glob('0216-*/*.rs')},
        rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu_affinity=sorted(os.sched_getaffinity(0))))
    run('reference',['bash','parity/check-reference.sh'],estimate=1)
    run('export-lod',['python3','parity/0215-lod-chain/corpus.py',T/'lod-corpus'],estimate=10)

def manifest(d, bins, features, extras='', feature_section=''):
    d.mkdir(parents=True,exist_ok=True)
    deps=''
    for alias,package,path in [('baseline','meshoptimizer-baseline',T/'base'),('candidate','meshoptimizer-rs',R)]:
        deps+=f'{alias}={{package="{package}",path={json.dumps(str(path))},default-features=false,features={json.dumps(features)}}}\n'
    (d/'Cargo.toml').write_text('[package]\nname="combined-proof"\nversion="0.0.0"\nedition="2021"\n'+feature_section+'[dependencies]\n'+deps+extras+''.join(f'[[bin]]\nname="{n}"\npath={json.dumps(str(p))}\n' for n,p in bins.items())+'[profile.release]\nlto="thin"\ncodegen-units=1\ndebug-assertions=true\noverflow-checks=true\n')

def build_exact(mode):
    # Bounds/geometry need clusterlod even when std and simd are disabled,
    # matching the lane's documented scalar proof configuration.
    features=ALL if mode=='all' else ['clusterlod']
    names=['remap','spatial','repair','simplify','decode','bounds','vdecode','stream']
    bins={n:R/f'parity/0216-{n}/exact.rs' for n in names}
    bins.update(remap_cpp=R/'parity/0216-remap/cpp-differential.rs',repair_corpus=R/'parity/0216-repair/corpus.rs',geometry=R/'parity/0215-toward-cpp/geometry-exact.rs')
    d=T/('exact-'+mode)
    section='[features]\nall=[]\n' if mode=='all' else ''
    manifest(d,bins,features,'meshopt="=0.7.0"\n',section)
    run('exact-lock-'+mode,['cargo','generate-lockfile','--offline','--manifest-path',d/'Cargo.toml'],estimate=5)
    run('exact-build-'+mode,['cargo','build','--offline','--locked','--release','--manifest-path',d/'Cargo.toml',*(['--features','all'] if mode=='all' else [])],estimate=1800)
    (T/'bin').mkdir(exist_ok=True)
    for name in bins: shutil.copy2(T/'release'/name,T/'bin'/f'{name}-{mode}')
    write(A/('exact-binding-'+mode+'.json'),dict(binaries={n:sha(T/'bin'/f'{n}-{mode}') for n in bins},manifest=clean((d/'Cargo.toml').read_text())))
    # Parallel has no no-feature API; preserve lane's scalar parallel mode too.
    d=T/('par-'+mode)
    manifest(d,{'par':R/'parity/0216-par-clod/exact.rs'},features if mode=='all' else ['parallel','clusterlod','parity-internals'],'rayon="=1.11.0"\n')
    run('par-build-'+mode,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml'],estimate=300)
    shutil.copy2(T/'release/par',T/'bin'/f'par-{mode}')

def exact(name,mode):
    args=[];estimate=600
    if name=='stream':
        if not (T/'stream-cpp').exists():
            run('stream-cpp-build',['c++','-O2','-std=c++11','-DMESHOPTIMIZER_NO_SIMD','-I'+str(REF/'src'),R/'parity/0216-stream/reference.cpp',REF/'src/indexcodec.cpp','-o',T/'stream-cpp'],estimate=60)
        args=[R/'parity/0216-stream/corpus.tsv',T/'stream-cpp',A/f'stream-cases-{mode}.tsv']
    if name=='repair_corpus': args=[T/'lod-corpus']
    if name=='spatial':
        run('spatial-size-'+mode,[T/'bin'/f'{name}-{mode}'],env={'SPATIAL_SIZE':'1'},estimate=10);estimate=3600
    run('exact-'+name+'-'+mode,[T/'bin'/f'{name}-{mode}',*args],estimate=estimate)

def par_corpus(mode):
    # Execute the exact lane driver unchanged, including all documented schedules.
    run('par-lod-'+mode,[T/'bin'/f'par-{mode}',T/'lod-corpus'],estimate=600)
    # Explicit spec worker matrix. Only the worker list changes in this copy.
    p=T/'par-spec.rs';s=(R/'parity/0216-par-clod/exact.rs').read_text()
    assert 'for workers in [1, 2, 4, 8, 32]' in s
    p.write_text(s.replace('for workers in [1, 2, 4, 8, 32]','for workers in [1, 4, 16]'))
    d=T/('par-spec-'+mode)
    manifest(d,{'par-spec':p},ALL if mode=='all' else ['parallel','clusterlod','parity-internals'],'rayon="=1.11.0"\n')
    run('par-spec-build-'+mode,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml'],estimate=300)
    write(A/('par-spec-binding-'+mode+'.json'),dict(binary=sha(T/'release/par-spec'),source_sha256=sha(p),manifest=clean((d/'Cargo.toml').read_text())))
    run('par-spec-lod-'+mode,[T/'release/par-spec',T/'lod-corpus'],estimate=600)
    run('par-spec-inputs-'+mode,[T/'release/par-spec',T/'inputs'],estimate=3600)

def gates(mode):
    flag='--all-features' if mode=='all' else '--no-default-features'
    run('test-'+mode,['cargo','test','--offline','--locked',flag],estimate=600)
    run('clippy-'+mode,['cargo','clippy','--offline','--locked',flag,'--all-targets','--','-D','warnings'],estimate=300)
    for group,folder in [('lod',T/'lod-corpus'),('inputs',T/'inputs')]:
        run('lod-identity-'+group+'-'+mode,['cargo','test','--offline','--locked','--release',flag,'--test','lod_chain','corpus_identity','--','--ignored','--exact','--nocapture'],env={'MESHOPT_LOD_CORPUS':str(folder)},estimate=1800)
    if mode=='all':
        run('fmt',['cargo','fmt','--all','--','--check'],estimate=10)
        run('boundary',['bash','parity/check-unsafe-boundary.sh'],estimate=10)

def proof_group():
    # Retained lane timings put this group below one CPU-hour. Reserve a
    # conservative 6000 CPU seconds, below the per-job two-hour ceiling.
    with (A/'sizes.jsonl').open('a') as f:f.write(json.dumps(dict(name='proof-group',cpu_seconds_upper_estimate=6000,basis='sum of finite lane proof/build timings, with compilation and production-corpus margin'))+'\n')
    for mode in ['all','none']:
        build_exact(mode)
        for name in ['remap','remap_cpp','repair','repair_corpus','simplify','decode','bounds','vdecode','stream','geometry','spatial','par']:exact(name,mode)
        par_corpus(mode)
        gates(mode)

def build_peers():
    canonical=T/'canonical';canonical.mkdir(exist_ok=True)
    d=canonical/'parity/peers';d.mkdir(parents=True,exist_ok=True)
    shutil.copytree(R/'parity/peers/src',d/'src',dirs_exist_ok=True)
    shutil.copy2(R/'parity/peers/Cargo.toml',d/'Cargo.toml')
    run('adapt-simplify',['python3',R/'parity/0216-qualify/additional_driver.py',d/'src/main.rs'],estimate=2)
    run('adapt-spatial',['python3',R/'parity/0216-qualify/prepare_driver.py',d/'src/main.rs'],estimate=2)
    # The basics adapter requires its original indentation anchor.
    p=d/'src/main.rs';p.write_text(p.read_text().replace('Self::RawMesh(ms,v,t)=>','   Self::RawMesh(ms,v,t)=>'))
    run('adapt-remap',['python3',R/'parity/0215-basics/prepare_driver.py',d],estimate=2)
    original=p.read_text()
    # Reuse the bounds lane's frozen-group adapter, with identical topology
    # across all arms. Setup and localization precede the counted region.
    original=original.replace('struct Input {','struct Input { groups: Vec<BoundGroup>,')
    original=original.replace('let p=b.chunks_exact(12)','let p:Vec<[f32;3]>=b.chunks_exact(12)')
    original=original.replace('let i=words(&b);','let i=words(&b);let groups=if std::env::args().nth(1).is_some_and(|op|op.ends_with("bounds")){bound_groups(path,&p)}else{Vec::new()};')
    original=original.replace('Self{remap,unique,p,i,attrs,','Self{groups,remap,unique,p,i,attrs,')
    adapter=(R/'parity/0216-bounds/build.py').read_text()
    block=adapter[adapter.index('    group_setup ='):adapter.index('    p.write_text(code)')]
    namespace={'code':original}
    exec(compile('\n'.join(l[4:] if l.startswith('    ') else l for l in block.splitlines()),'bounds-adapter','exec'),namespace)
    original=namespace['code']
    # The upstream Rust port has the same bounds result layout.
    rust_bounds=namespace['ours'].replace('let p=m::Positions::from_packed(&g.p);','')
    rust_bounds=rust_bounds.replace('m::compute_cluster_bounds(&g.i,p,w)','optimesh::meshletutils::compute_cluster_bounds(&g.i,g.p.as_flattened(),g.p.len(),12)')
    rust_bounds=rust_bounds.replace('m::compute_meshlet_bounds(&g.v,&g.t,p,w)','optimesh::meshletutils::compute_meshlet_bounds(&g.v,&g.t,g.t.len()/3,g.p.as_flattened(),g.p.len(),12)')
    rust_bounds=rust_bounds.replace('}.unwrap();','};')
    original=original.replace('  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{v::',rust_bounds+'  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{v::',1)
    original=original.replace('let mut w=m::Workspace::new(m::Limits{max_bytes:3usize<<30,..m::Limits::default()});','let mut w=m::Workspace::new(m::Limits{max_bytes:usize::MAX,max_work:u64::MAX});')
    assert 'max_bytes:usize::MAX,max_work:u64::MAX' in original
    for op in OPS+['cluster-bounds','meshlet-bounds']:
        assert f'"{op}"' in original,op
    # Time collection is solely for uncounted parallel disclosure calls.
    original=original.replace('control("enable");for _','control("enable");let start=if std::env::var_os("PROBE_CTL").is_none(){Some(Instant::now())}else{None};for _')
    original=original.replace('control("disable");println!("COUNTED")','let elapsed=start.map_or(0.0,|s|s.elapsed().as_secs_f64());control("disable");println!("COUNTED {elapsed:.9}")')
    binding={}
    for arm in ARMS+['parallel']:
        base=arm.startswith('base');source=T/'base' if base else R
        # Same package identity, source path and consumer path for every arm.
        # Touch copied files to prevent Cargo from reusing the preceding arm.
        if (canonical/'src').exists():shutil.rmtree(canonical/'src')
        shutil.copytree(source/'src',canonical/'src')
        shutil.copytree(source/'docs',canonical/'docs',dirs_exist_ok=True)
        for name in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(source/name,canonical/name)
        cm=canonical/'Cargo.toml';cm.write_text(cm.read_text().replace('name = "meshoptimizer-baseline"','name = "meshoptimizer-rs"',1))
        for path in (canonical/'src').rglob('*'):
            if path.is_file():path.touch()
        path=canonical;package='meshoptimizer-rs'
        features=['std','clusterlod']+(['simd'] if arm.endswith('default') or arm=='parallel' else [])+(['parallel'] if arm=='parallel' else [])
        text=original.replace('m::clusterlod::build_with_output(', 'm::clusterlod::build_parallel_with_output(') if arm=='parallel' else original
        p.write_text(text)
        (d/'Cargo.toml').write_text(f'[package]\nname="combined-peer"\nversion="0.0.0"\nedition="2021"\n[features]\nours=["dep:meshoptimizer-rs"]\ncpp=["dep:meshopt"]\nrust=["dep:optimesh"]\n[dependencies]\nmeshoptimizer-rs={{package="{package}",path={json.dumps(str(path))},optional=true,default-features=false,features={json.dumps(features)}}}\nmeshopt={{version="=0.7.0",optional=true}}\noptimesh={{version="=1.1.0",optional=true,default-features=false}}\n[profile.release]\nlto="thin"\ncodegen-units=1\n')
        feature='cpp' if arm.startswith('cpp') else 'rust' if arm=='optimesh' else 'ours'
        run('build-'+arm,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml','--features',feature],env={'CXXFLAGS':('-DMESHOPTIMIZER_NO_SIMD ' if arm=='cpp-scalar' else '')+'-fno-fast-math -ffp-contract=off'},estimate=600)
        shutil.copy2(T/'release/combined-peer',T/'bin'/arm)
        binding[arm]=dict(binary=sha(T/'bin'/arm),consumer=hashlib.sha256(text.encode()).hexdigest(),manifest=clean((d/'Cargo.toml').read_text()),sources={str(p.relative_to(canonical)):sha(p) for p in (canonical/'src').rglob('*') if p.is_file()})
    (A/'peer-driver.rs').write_text(original)
    write(A/'peer-bindings.json',binding)
    registry=Path(os.environ.get('CARGO_HOME',str(Path.home()/'.cargo')))/'registry/src'
    cpp=next(registry.glob('*/meshopt-0.7.0'))
    cpp_files={}
    for p in (cpp/'vendor/src').iterdir():
        if p.suffix in ['.cpp','.h']:
            assert p.read_bytes()==(REF/'src'/p.name).read_bytes(),p.name
            cpp_files[p.name]=sha(p)
    assert (cpp/'vendor/demo/clusterlod.h').read_bytes()==(REF/'demo/clusterlod.h').read_bytes()
    write(A/'environment.json',dict(cpp_source_identity=True,cpp_files=cpp_files,cpp_lod_sha256=sha(cpp/'vendor/demo/clusterlod.h'),
        reference_revision=subprocess.check_output(['git','-C',str(REF),'rev-parse','HEAD'],text=True).strip(),
        rustc=subprocess.check_output(['rustc','-Vv'],text=True),cxx=subprocess.check_output(['c++','--version'],text=True),
        perf=subprocess.check_output([os.environ['PEERS_PERF'],'--version'],text=True).strip(),cores=physical_cpus(),
        cpu_model=next(l.split(':',1)[1].strip() for l in Path('/proc/cpuinfo').read_text().splitlines() if l.startswith('model name')),kernel=os.uname().release,
        flags={k:os.environ.get(k,'') for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_INCREMENTAL','LC_ALL','RUSTC_WRAPPER']},
        optimesh_crate_sha256=sha(next((registry.parent/'cache').glob('*/optimesh-1.1.0.crate')))))

def sample(case,op,arm,rep,kind='instructions'):
    records=A/('counts.jsonl' if kind=='instructions' else 'wall.jsonl')
    rows=[json.loads(l) for l in records.read_text().splitlines()] if records.exists() else []
    if any((x['case'],x['op'],x['arm'],x['rep'])==(case,op,arm,rep) for x in rows):return
    folder=A/'samples';folder.mkdir(exist_ok=True)
    prefix=folder/f'{kind}-{case}-{op}-{arm}-{rep}'
    stem=prefix.name;attempt=1
    while list(folder.glob(prefix.name+'.*')):
        attempt+=1;prefix=folder/f'{stem}-attempt{attempt}'
    n=1000 if case=='tiny-smooth' else 3 if case.startswith('medium') else 1
    if op=='cluster-lod':n=10 if case=='tiny-smooth' else 1
    if op.endswith('bounds'):n=100 if case=='tiny-smooth' else 1
    output=T/'sample-output';cpus=physical_cpus()
    binary='parallel' if arm.startswith('parallel-') else arm
    threads=1 if arm=='parallel-1' else 4
    cmd=['taskset','-c',','.join(map(str,cpus if arm in ['parallel','parallel-4'] else cpus[:1])),str(T/'bin'/binary),op,str(T/'inputs'/(case+'.bin')),str(output)]
    fds=[];env=dict(os.environ,RAYON_NUM_THREADS=str(threads),BOUNDS_SCRATCH=str(T))
    if kind=='instructions':
        cr,cw=os.pipe();ar,aw=os.pipe();fds=[cr,cw,ar,aw]
        cmd=[os.environ['PEERS_PERF'],'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--',*cmd]
        env.update(PROBE_CTL=str(cw),PROBE_ACK=str(ar))
    # Each single sample <= 1200 CPU seconds, based on retained lane counts.
    floor();start=resource.getrusage(resource.RUSAGE_CHILDREN);t=time.monotonic()
    try:p=subprocess.run(cmd,input=f'{n}\nstop\n',capture_output=True,text=True,pass_fds=fds,env=env)
    finally:
        for fd in fds:os.close(fd)
    Path(str(prefix)+'.log').write_text(clean(p.stdout+p.stderr))
    assert p.returncode==0,(case,op,arm,p.stderr[-4000:])
    lines=p.stdout.splitlines();assert len(lines)==2 and lines[0]=='READY' and lines[1].startswith('COUNTED '),lines
    end=resource.getrusage(resource.RUSAGE_CHILDREN)
    row=dict(case=case,op=op,arm=arm,rep=rep,repetitions=n,exit=p.returncode,output_sha256=sha(output),
        cpu_seconds=end.ru_utime+end.ru_stime-start.ru_utime-start.ru_stime,wall_seconds=time.monotonic()-t,
        sample_prefix=str(prefix.relative_to(A)),attempt=attempt)
    same=[x for x in rows if x['case']==case and x['op']==op and x['arm']==arm]
    assert not same or {x['output_sha256'] for x in same}=={row['output_sha256']},('unstable',case,op,arm)
    if kind=='instructions':
        fields=next(l.split(';') for l in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in l)
        assert float(fields[4])==100.,fields
        assert int(fields[0])>0,fields
        row.update(instructions=int(fields[0]),instructions_per_call=int(fields[0])/n)
    else:row['seconds_per_call']=float(lines[1].split()[1])/n
    assert row['cpu_seconds']<=1200,row
    with records.open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
    output.unlink()
    if Path(str(output)+'.raw').exists():Path(str(output)+'.raw').unlink()
    print(case,op,arm,rep,round(row.get('instructions_per_call',row.get('seconds_per_call')),4),flush=True)

def physical_cpus():
    seen=set();cpus=[]
    for c in sorted(os.sched_getaffinity(0)):
        t=Path(f'/sys/devices/system/cpu/cpu{c}/topology');key=tuple((t/p).read_text() for p in ['physical_package_id','core_id'])
        if key not in seen:seen.add(key);cpus.append(c)
        if len(cpus)==4:return cpus
    raise AssertionError('four cores required')

def measure(op, selected=None):
    with (A/'sizes.jsonl').open('a') as f:f.write(json.dumps(dict(name='measure-'+op,per_sample_cpu_seconds_upper_estimate=1200,per_job_cpu_seconds_upper_estimate=7200,method='retained lane campaigns; tiny=1000, medium=3, large=1'))+'\n')
    for x in json.loads((A/'inputs.json').read_text()):
        if selected and x['case'] not in selected:continue
        assert sha(T/'inputs'/(x['case']+'.bin'))==x['sha256']
        for rep in range(3):
            for arm in ARMS if rep!=1 else ARMS[::-1]:
                if arm=='optimesh' and op=='cluster-lod':continue
                sample(x['case'],op,arm,rep)

def wall():
    for x in json.loads((A/'inputs.json').read_text()):
        if not x['case'].startswith('real-'):continue
        for rep in range(5):
            for arm in ['parallel-1','parallel-4'] if rep%2==0 else ['parallel-4','parallel-1']:
                sample(x['case'],'cluster-lod',arm,rep,'wall')

def bound_inputs():
    source=Path(os.environ['QUALIFY_BOUND_INPUTS'])
    binding=json.loads((R/'parity/0216-bounds/bound-inputs.json').read_text())
    (T/'bound-inputs').mkdir()
    for name,digest in binding['inputs'].items():
        assert sha(source/name)==digest
        os.link(source/name,T/'bound-inputs'/name)
    write(A/'bound-inputs.json',binding)

def decode_differential(lane, mode):
    art=A/(lane+'-'+mode);art.mkdir(exist_ok=True)
    env={'DECODE_ART':str(art)}
    if mode=='none':env.update(DECODE_NO_STD='1',RUSTFLAGS='-C target-feature=+ssse3,+popcnt,+sse4.1')
    run(lane+'-diff-build-'+mode,['python3',R/f'parity/0216-{lane}/build_differential.py'],env=env,estimate=180)
    run(lane+'-diff-'+mode,['python3',R/f'parity/0216-{lane}/differential.py'],env=env,estimate=1800)
    write(art/'binaries.json',{str(p.relative_to(T)):sha(p) for p in [T/'proof/release/differential',T/'cpp-scalar-exact',T/'cpp-default-exact']})

def protocol():
    d=T/'proof/0216b-protocol';d.mkdir(parents=True,exist_ok=True)
    sources=[REF/'src'/n for n in ['vcacheoptimizer.cpp','overdrawoptimizer.cpp','allocator.cpp','simplifier.cpp','indexgenerator.cpp','vfetchoptimizer.cpp','quantization.cpp']]
    run('protocol-cpp',['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-I'+str(REF/'src'),R/'parity/reference.cpp',*sources,'-o',d/'cpp-reference'],estimate=180)
    for mode in ['all','none']:
        (d/'Cargo.toml').write_text('[package]\nname="meshoptimizer-parity"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={path='+json.dumps(str(R))+',default-features=false,features='+json.dumps(ALL if mode=='all' else [])+'}\nlibm={version="=0.2.16",default-features=false}\n[lib]\npath='+json.dumps(str(R/'parity/src/lib.rs'))+'\n[[bin]]\nname="meshopt-driver"\npath='+json.dumps(str(R/'parity/src/main.rs'))+'\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
        run('protocol-build-'+mode,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml'],estimate=300)
        shutil.copy2(T/'release/meshopt-driver',d/('rust-'+mode))
    run('protocol-differential',['python3',R/'parity/0216-qualify/protocol-modes.py'],env={'MESHOPT_SCRATCH':str(T)},estimate=300)
    shutil.copytree(d,A/'protocol',ignore=shutil.ignore_patterns('Cargo*','cpp-reference','rust-*'))

def p03_build():
    d=T/'p03';d.mkdir(exist_ok=True)
    shutil.copytree(R/'parity/p03/src',d/'src',dirs_exist_ok=True)
    main=d/'src/main.rs'
    s=main.read_text().replace('128 * 1024 * 1024','512 * 1024 * 1024')
    s=s.replace('let result = meshopt_p03_parity::execute(&b).unwrap();','''let execute=||meshopt_p03_parity::execute(&b).unwrap();
        #[cfg(feature="all")]
        let result={use meshoptimizer_rs::codec::{Level,with_level};
        let level=match std::env::var("SPATIAL_LEVEL").as_deref(){Ok("scalar")=>Level::Scalar,Ok("sse2")=>Level::Sse2,Ok("ssse3")=>Level::Ssse3,_=>Level::Sse41};with_level(level,execute).unwrap()};
        #[cfg(not(feature="all"))] let result=execute();''')
    main.write_text(s)
    p=d/'src/lib.rs';p.write_text(p.read_text().replace('128 * 1024 * 1024','512 * 1024 * 1024'))
    for mode in ['all','none']:
        features=ALL if mode=='all' else ['clusterlod']
        (d/'Cargo.toml').write_text('[package]\nname="meshopt-p03-parity"\nversion="0.0.0"\nedition="2021"\n[features]\nall=[]\n[dependencies]\nmeshoptimizer-rs={path='+json.dumps(str(R))+',default-features=false,features='+json.dumps(features)+'}\n[lib]\ncrate-type=["rlib"]\n[[bin]]\nname="p03-driver"\npath="src/main.rs"\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
        run('p03-build-'+mode,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml',*(['--features','all'] if mode=='all' else [])],estimate=300)
        shutil.copy2(T/'release/p03-driver',T/'bin'/('p03-'+mode))
    transport=d/'reference.cpp';transport.write_text((R/'parity/p03/reference.cpp').read_text().replace('128*1024*1024','512*1024*1024'))
    sources=[REF/'src'/f'{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
    for mode in ['scalar','default']:
        run('p03-cpp-'+mode,['c++','-std=c++17','-O3','-fno-fast-math','-ffp-contract=off',*(['-DMESHOPTIMIZER_NO_SIMD'] if mode=='scalar' else []),'-I'+str(REF/'src'),'-I'+str(REF/'demo'),transport,*sources,'-o',T/'bin'/('p03-cpp-'+mode)],estimate=300)
    write(A/'p03-binding.json',dict(binaries={p.name:sha(p) for p in (T/'bin').glob('p03-*')},sources={str(p.relative_to(d)):sha(p) for p in (d/'src').rglob('*') if p.is_file()},transport=sha(transport)))

def p03_diff():
    spec=importlib.util.spec_from_file_location('p03',R/'parity/p03/runner.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
    clients={'cpp':r.Client('cpp',T/'bin/p03-cpp-scalar'),'cpp-default':r.Client('cpp-default',T/'bin/p03-cpp-default'),'none':r.Client('none',T/'bin/p03-none')}
    for level in ['scalar','sse2','ssse3','sse41']:
        r.ENV['SPATIAL_LEVEL']=level;clients[level]=r.Client(level,T/'bin/p03-all')
    d=T/'p03-diff';d.mkdir(exist_ok=True);counts={}
    try:
        with (d/'corpus.bin').open('wb') as records:
            for op in [1,3,4,6,7,8]:
                count=0
                for seed in range(20261029,20261541 if op!=4 else 20262029):
                    data=r.case(op,seed);r.compare(clients,data,f'{op}-{seed}',d,records);count+=1
                    if op in [1,3,4]:
                        data=bytearray(data);struct.pack_into('<I',data,4,op|0x10000)
                        r.compare(clients,data,f'{op}-{seed}-into',d,records);count+=1
                counts[str(op)]=count
            for seed in range(24):
                p,i=r.grid(2+seed%6,seed%2)
                data=r.message(16,p,i,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=seed*37&511,split=0.1,radii=[j/100 for j in range(len(p))])
                r.compare(clients,data,f'lod-{seed}',d,records)
            counts['16']=24
            for n in range(1,8):
                for scale in [0.,1e-30,1.,1e30]:
                    p,idx=r.grid(n+2);p=[tuple((-0. if (i+k)%2 else 0.) if scale==0 else x*scale for k,x in enumerate(v)) for i,v in enumerate(p)]
                    r.compare(clients,r.message(4,p,idx,mv=8,min=1,mt=n,split=0.5),f'extreme-{n}-{scale}',d,records)
            counts['spatial-extremes']=28
        reals=[]
        for x in json.loads((A/'inputs.json').read_text()):
            if not x['case'].startswith('real-'):continue
            data=(T/'inputs'/(x['case']+'.bin')).read_bytes();nv,ni=struct.unpack_from('<II',data);end=8+nv*12+ni*4
            msg=struct.pack('<4s6I2f3I',b'MO03',4,nv,ni,64,41,124,0.,0.5,4,0,0)+data[8:end]+bytes(nv*4)
            outputs={n:c.run(msg)[0] for n,c in clients.items()}
            assert len({hashlib.sha256(v).hexdigest() for v in outputs.values()})==1,x
            reals.append(dict(case=x['case'],sha256=hashlib.sha256(outputs['cpp']).hexdigest()))
    finally:
        for c in clients.values():c.close()
    write(A/'p03-differential.json',dict(exit=0,counts=counts,real=reals,corpus_sha256=sha(d/'corpus.bin')))
    print(counts,flush=True)

def build_cost():
    d=T/'cost';d.mkdir(exist_ok=True)
    (d/'Cargo.toml').write_text('[package]\nname="qualify-cost"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={path='+json.dumps(str(R))+'}\n[[bin]]\nname="cost"\npath='+json.dumps(str(A/'cost.rs'))+'\n[profile.release]\nlto="thin"\ncodegen-units=1\n')
    run('build-cost',['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml'],estimate=300)
    shutil.copy2(T/'release/cost',T/'bin/cost')
    write(A/'cost-binding.json',dict(binary=sha(T/'bin/cost'),source=sha(A/'cost.rs'),manifest=clean((d/'Cargo.toml').read_text())))

def costs():
    for x in json.loads((A/'inputs.json').read_text()):
        if x['case'].startswith('real-'):continue
        for rep in range(3):
            ops=['sanitize','weld','orient','fill','sequence-one-shot','sequence-stream']
            for op in ops if rep!=1 else ops[::-1]:sample(x['case'],op,'cost',rep)

def differential_group():
    with (A/'sizes.jsonl').open('a') as f:f.write(json.dumps(dict(name='build-and-differential-group',cpu_seconds_upper_estimate=6000,basis='retained decoder differential about 90-120 CPU seconds per mode; spatial differential about 76; compile and matrix margin'))+'\n')
    build_peers();bound_inputs();build_cost();protocol();p03_build();p03_diff()
    for lane in ['decode','vdecode']:
        for mode in ['all','none']:decode_differential(lane,mode)

def measure_inputs(group):
    names=[x['case'] for x in json.loads((A/'inputs.json').read_text()) if (not x['case'].startswith('real-') if group=='corpus' else x['case']==group)]
    assert names,group
    with (A/'sizes.jsonl').open('a') as f:f.write(json.dumps(dict(name='measure-inputs-'+group,cpu_seconds_upper_estimate=6000,basis='retained full release matrix instruction counts and lane timings; one production mesh or five synthetic meshes, including warmup/setup margin'))+'\n')
    for op in OPS+['cluster-bounds','meshlet-bounds']:measure(op,names)

def disclosure_group():
    costs();wall();summarize()

def summarize():
    import math,re
    rows=[json.loads(l) for l in (A/'counts.jsonl').read_text().splitlines()]
    jobs=[json.loads(l) for l in (A/'jobs.jsonl').read_text().splitlines()]
    inputs=json.loads((A/'inputs.json').read_text());ops=OPS+['cluster-bounds','meshlet-bounds']
    expected=sum(8*3*(len(ARMS)-(op=='cluster-lod')) for op in ops)+5*3*6
    assert len(rows)==expected,(len(rows),expected)
    assert len({(x['case'],x['op'],x['arm'],x['rep']) for x in rows})==len(rows)
    failed=[x for x in jobs if x['exit']!=0]
    # Prior infrastructure/build attempts remain disclosed; the final run of
    # every required stage must succeed.
    last={x['stage']:x for x in jobs};assert all(x['exit']==0 for x in last.values()),last
    start=json.loads((A/'start.json').read_text())
    current={str(p.relative_to(R)):sha(p) for p in (R/'src').rglob('*') if p.is_file()}
    write(A/'final-source.json',dict(sources=current,unchanged=current==start['sources']))
    med={};hashes={}
    for x in rows:
        key=(x['case'],x['op'],x['arm'])
        med.setdefault(key,[]).append(x['instructions_per_call']);hashes.setdefault(key,set()).add(x['output_sha256'])
    med={k:statistics.median(v) for k,v in med.items()}
    cells=[];regressions=[]
    for op in ops:
        for x in inputs:
            case=x['case'];m={a:med[case,op,a] for a in ARMS if not(a=='optimesh' and op=='cluster-lod')}
            for suffix in ['scalar','default']:
                assert hashes[case,op,'base-'+suffix]==hashes[case,op,'train-'+suffix],('base/train output',case,op,suffix)
                assert hashes[case,op,'train-'+suffix]==hashes[case,op,'cpp-'+suffix],('C++ output',case,op,suffix)
            values=dict(scalar_base=m['train-scalar']/m['base-scalar'],default_base=m['train-default']/m['base-default'],
                scalar_cpp=m['train-scalar']/m['cpp-scalar'],default_cpp=m['train-default']/m['cpp-default'])
            if op!='cluster-lod':values.update(scalar_optimesh=m['train-scalar']/m['optimesh'],default_optimesh=m['train-default']/m['optimesh'])
            for k in ['scalar_base','default_base']:
                if values[k]>1.01:regressions.append(dict(case=case,op=op,mode=k,ratio=values[k]))
            cells.append(dict(case=case,op=op,medians=m,ratios=values,
                cpp_identity={s:hashes[case,op,'train-'+s]==hashes[case,op,'cpp-'+s] for s in ['scalar','default']},
                optimesh_identity=None if op=='cluster-lod' else hashes[case,op,'train-default']==hashes[case,op,'optimesh']))
    def gm(v):return math.exp(sum(math.log(x) for x in v)/len(v))
    summaries=[]
    for op in ops:
        c=[x for x in cells if x['op']==op];groups={}
        for name,is_real in [('real',True),('corpus',False)]:
            group=[x for x in c if x['case'].startswith('real-')==is_real]
            groups[name]={k:gm([x['ratios'][k] for x in group]) for k in c[0]['ratios']}
        summaries.append(dict(op=op,groups=groups,max_scalar=max(x['ratios']['scalar_base'] for x in c),max_default=max(x['ratios']['default_base'] for x in c)))
    write(A/'release-results.json',dict(samples=len(rows),campaign_samples=len(rows)-90,cells=cells,summary=summaries,regressions=regressions))
    table=['# 0.2.16 combined release table','',
        'Median of three instructions:u samples per case. Each entry is real-mesh GM / corpus GM. Ratios below 1 mean fewer instructions. Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. optimesh 1.1.0 is scalar; C++ 1.3 is matched scalar/default. No optimesh cluster LOD API.','',
        '| Function | scalar / 0.2.15 | default / 0.2.15 | scalar / optimesh | default / optimesh | scalar / scalar C++ | default / default C++ | max case scalar / default vs base |',
        '|---|---:|---:|---:|---:|---:|---:|---:|']
    metrics=['scalar_base','default_base','scalar_optimesh','default_optimesh','scalar_cpp','default_cpp']
    for x in summaries:
        values=[f"{x['groups']['real'][k]:.3f} / {x['groups']['corpus'][k]:.3f}" if k in x['groups']['real'] else '—' for k in metrics]
        table.append('| '+x['op']+' | '+' | '.join(values)+f" | {x['max_scalar']:.6f} / {x['max_default']:.6f} |")
    table+=['',f"Campaign samples: {len(rows)-90}. Base/train output identities: {len(ops)*8*2}/{len(ops)*8*2}.",
        f"Cases above 1.01: {len(regressions)}. Full per-case medians and ratios: [release-results.json](combined-ff38162/release-results.json)."]
    (R/'parity/0216-qualify/TABLE.md').write_text('\n'.join(table)+'\n')
    observations={}
    exits=[json.loads(l) for l in (A/'exits.jsonl').read_text().splitlines()]
    for x in exits:
        if x['name'].startswith('exact-') and not x['name'].startswith(('exact-lock','exact-build')):
            text=(A/x['log']).read_text()
            lines=[s for s in text.splitlines() if re.search(r'(PASS|comparisons|differences|total_cases)',s)]
            observations[x['name']]=dict(exit=x['exit'],counts=lines[-6:])
    write(A/'exact-counts.json',observations)
    cost=[]
    for x in inputs:
        case=x['case']
        if case.startswith('real-'):continue
        m={op:med[case,op,'cost'] for op in ['sanitize','weld','orient','fill','sequence-one-shot','sequence-stream']}
        cost.append(dict(case=case,instructions=m,repair_vs_sanitize={op:m[op]/m['sanitize'] for op in ['weld','orient','fill']},stream_vs_one_shot=m['sequence-stream']/m['sequence-one-shot']))
    write(A/'cost-results.json',dict(cases=cost,corpus_gm=dict(weld=gm([x['repair_vs_sanitize']['weld'] for x in cost]),orient=gm([x['repair_vs_sanitize']['orient'] for x in cost]),fill=gm([x['repair_vs_sanitize']['fill'] for x in cost]),stream=gm([x['stream_vs_one_shot'] for x in cost]))))
    wall_rows=[json.loads(l) for l in (A/'wall.jsonl').read_text().splitlines()];assert len(wall_rows)==30
    wall=[]
    for case in ['real-214e','real-a082','real-b6fd']:
        m={a:statistics.median(x['seconds_per_call'] for x in wall_rows if x['case']==case and x['arm']==a) for a in ['parallel-1','parallel-4']}
        assert len({x['output_sha256'] for x in wall_rows if x['case']==case})==1
        assert {x['output_sha256'] for x in wall_rows if x['case']==case}==hashes[case,'cluster-lod','train-default']
        wall.append(dict(case=case,one_worker_seconds=m['parallel-1'],four_worker_seconds=m['parallel-4'],speedup=m['parallel-1']/m['parallel-4']))
    write(A/'wall-results.json',wall)
    print('samples',len(rows),'regressions',len(regressions),'failed attempts',len(failed),flush=True)
    for x in regressions:print(x,flush=True)

if __name__=='__main__':
    assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU')=='0'
    runner_bytes=Path(__file__).read_bytes();runner_sha=hashlib.sha256(runner_bytes).hexdigest()
    (A/'runners').mkdir(exist_ok=True)
    retained=A/'runners'/(runner_sha+'.py')
    if not retained.exists():retained.write_bytes(runner_bytes)
    stage=' '.join(sys.argv[1:]);before=resource.getrusage(resource.RUSAGE_CHILDREN);own_before=resource.getrusage(resource.RUSAGE_SELF);start=time.monotonic();code=0
    with (A/'job-sizing.jsonl').open('a') as f:
        f.write(json.dumps(dict(stage=stage,upper_cpu_seconds_estimate=7200,basis='finite lane matrices and retained campaign timings; per-command tighter estimates in sizes.jsonl'))+'\n')
    try:globals()[sys.argv[1]](*sys.argv[2:])
    except BaseException as e:
        code=e.code if isinstance(e,SystemExit) and isinstance(e.code,int) else 1
        raise
    finally:
        after=resource.getrusage(resource.RUSAGE_CHILDREN)
        cpu=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime
        own_after=resource.getrusage(resource.RUSAGE_SELF)
        own_cpu=own_after.ru_utime+own_after.ru_stime-own_before.ru_utime-own_before.ru_stime
        with (A/'jobs.jsonl').open('a') as f:
            f.write(json.dumps(dict(stage=stage,exit=code,child_cpu_seconds=cpu,supervisor_cpu_seconds=own_cpu,total_cpu_seconds=cpu+own_cpu,elapsed_seconds=time.monotonic()-start,runner_sha256=runner_sha))+'\n')
        assert cpu+own_cpu<=7200,(stage,cpu+own_cpu)
