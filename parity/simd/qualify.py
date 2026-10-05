#!/usr/bin/env python3
"""P07 per-level/native/WASM identity and bounded paired SIMD measurements."""
import argparse, hashlib, json, math, os, random, shutil, statistics, struct, subprocess, sys, time, zipfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'codec'))
import measure as m
import runner as r02
import runner04 as r04
ROOT,TARGET,ART,REF=m.ROOT,m.TARGET,m.ART,m.REF
BIN=ART/'bin'
BAR=ROOT/'parity/SIMD_BAR.md'

def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sources():
    files=[*ROOT.glob('src/**/*'),*ROOT.glob('tests/*.rs'),*ROOT.glob('parity/simd/**/*'),*ROOT.glob('parity/codec/*.rs'),*ROOT.glob('parity/codec/src/*.rs'),ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'parity/codec/Cargo.toml',ROOT/'parity/codec/Cargo.lock',BAR]
    return {str(p.relative_to(ROOT)):m.sha(p) for p in sorted(files) if p.is_file() and '__pycache__' not in p.parts}

def build():
    before=sources()
    BIN.mkdir(parents=True,exist_ok=True)
    for scalar,name in [(True,'cpp-scalar'),(False,'cpp-simd')]:shutil.copy2(m.build_cpp(scalar),BIN/name)
    for features,name in [([], 'rust-scalar'),(['--features','simd'],'rust-simd')]:
        m.cmd(['cargo','build','--offline','--locked','--release','--manifest-path',ROOT/'parity/codec/Cargo.toml',*features])
        shutil.copy2(TARGET/'release/codec-driver',BIN/name)
    for flags,features,name in [('',[],'wasm-scalar'),('-C target-feature=+simd128',['--features','simd'],'wasm-simd')]:
        env=dict(m.ENV,RUSTFLAGS=flags)
        subprocess.run(['cargo','build','--offline','--locked','--release','--target','wasm32-unknown-unknown','--lib','--manifest-path',str(ROOT/'parity/codec/Cargo.toml'),*features],env=env,check=True)
        shutil.copy2(TARGET/'wasm32-unknown-unknown/release/meshopt_codec_parity.wasm',BIN/(name+'.wasm'))
    assert before==sources(), 'source changed during build'
    save(ART/'build.json',{'sources':sources(),'binaries':{p.name:m.sha(p) for p in BIN.iterdir()},'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx':subprocess.check_output(['c++','--version'],text=True),'node':subprocess.check_output(['node','--version'],text=True),'flags':m.FLAGS,'rust_profile':'release fat LTO one codegen unit; generic CPU; std runtime detection; wasm +simd128 only'})

class Rust(m.Driver):
    def __init__(self,level=None):
        cpu=int(os.environ.get('MESHOPT_BENCH_CPU',min(os.sched_getaffinity(0))))
        self.p=subprocess.Popen(['taskset','-c',str(cpu),str(BIN/'rust-simd'),*([level] if level else [])],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=m.ENV)

def drivers(wasm=True):
    # Unsupported ceilings fail closed in the driver before executing instructions.
    levels=subprocess.check_output([BIN/'rust-simd','levels'],text=True).split()
    out={'cpp-scalar':m.Driver(BIN/'cpp-scalar'),'cpp-simd':m.Driver(BIN/'cpp-simd'),'rust-scalar':m.Driver(BIN/'rust-scalar')}
    for l in levels:out[l]=Rust(l)
    if wasm:out.update({'wasm-scalar':r02.Wasm(BIN/'wasm-scalar.wasm'),'wasm-simd':r02.Wasm(BIN/'wasm-simd.wasm')})
    return out

def corpus():
    path=ART/'inputs.json'
    if path.exists():
        rows=json.loads(path.read_text())
        for r in rows:assert m.sha(ART/r['path'])==r['sha256']
        return [(r['case'],(ART/r['path']).read_bytes()) for r in rows]
    cpp=m.Driver(BIN/'cpp-scalar')
    cases=m.corpus(cpp)
    for name,b in list(cases):
        if name.startswith('vertex-v0'):
            _,data,_=cpp.call(b);count,stride=struct.unpack_from('<II',b,8)
            _,encoded,_=cpp.call(m.request(11,count,stride,data,version=1))
            cases.append((name.replace('-v0-','-v1-'),m.request(1,count,stride,encoded)))
    resident=next(b for name,b in cases if name=='vertex-v0-resident-s12')
    _,data,_=cpp.call(resident);count,stride=struct.unpack_from('<II',resident,8)
    for version in [0,1]:
        for level in [0,9]:
            _,encoded,_=cpp.call(m.request(11,count,stride,data,version=version,level=level))
            cases.append((f'vertex-v{version}-resident-s12-level{level}',m.request(1,count,stride,encoded)))
    n=6291456;indices=struct.pack(f'<{n}I',*[i%60000 for i in range(n)])
    for v in [0,1]:
        _,encoded,_=cpp.call(m.request(12,n,4,indices,version=v));cases.append((f'index-2-v{v}-millions-s4',m.request(2,n,4,encoded)))
    assert len(cases)==81
    rng=random.Random(20261005)
    for count,label in [(17,'tiny'),(4097,'resident'),(262145,'streaming')]:
        for f,stride in [(1,4),(1,8),(2,8),(3,4),(3,12),(3,32),(4,4),(4,8)]:
            floats=r02.filter_floats(rng,f,count,stride) if f<4 else struct.pack(f'<{count*4}f',*[rng.random() for _ in range(count*4)])
            status,filtered,_=cpp.call(m.request(13+f,count,stride,floats,level=8 if stride==4 else 12));assert status==0
            cases.append((f'varied-filter-{f}-{label}-s{stride}',m.request(18 if f==4 else 3+f,count,stride,filtered)))
            if f<4:
                status,encoded,_=cpp.call(m.request(11,count,stride,filtered,version=0));assert status==0
                cases.append((f'varied-view-{f}-{label}-s{stride}',m.request(7,count,stride,encoded,filter=f)))
    for vc,tc in [(1,1),(64,126),(256,256)]:
        vertices=sorted(rng.sample(range(1<<20),vc));triangles=bytes(rng.randrange(vc) for _ in range(tc*3))
        status,encoded,_=cpp.call(m.request(19,vc,0,struct.pack(f'<{vc}I',*vertices)+triangles,version=tc));assert status==0
        for vs in [2,4]:
            for ts in [3,4]:cases.append((f'meshlet-{vc}-{tc}-v{vs}-t{ts}',m.request(20,vc,vs,encoded,version=tc,level=ts)))
        cases.append((f'meshlet-raw-{vc}-{tc}',m.request(21,vc,4,encoded,version=tc,level=4)))
    cpp.close();directory=ART/'inputs';directory.mkdir(exist_ok=True)
    rows=[]
    for name,b in cases:
        p=directory/(name+'.input');p.write_bytes(b);rows.append({'case':name,'path':str(p.relative_to(ART)),'sha256':m.sha(p)})
    save(path,rows);return cases

def cpp_simd_conformance(name,b,expected,actual):
    op=struct.unpack_from('<I',b,4)[0];f=op-3 if op in [4,5,6] else 4 if op==18 else struct.unpack_from('<I',b,20)[0] if op==7 else 0
    assert actual[0]==expected[0] or (actual[0]==0)==(expected[0]==0),name
    if expected[0]!=0:return
    if f in [1,2,4]:
        stride=struct.unpack_from('<I',b,12)[0];fmt=('B' if stride==4 else 'H') if f==4 else 'b' if f==1 and stride==4 else 'h'
        a=struct.unpack('<'+fmt*(len(expected[1])//struct.calcsize(fmt)),expected[1]);v=struct.unpack('<'+fmt*(len(actual[1])//struct.calcsize(fmt)),actual[1])
        distance=max([abs(x-y) for x,y in zip(a,v)]+[0])
        if f==4:
            # Color is outside EXT. P04 preserves scalar integer wrap on raw
            # words; upstream SIMD saturates such out-of-representation values.
            # Keep exact Rust/scalar equality and report the separate distance.
            return {'color_simd_max_unit_difference':distance}
        assert distance<=1,(name,'C++ SIMD one-unit conformance')
    else:assert actual[1]==expected[1],(name,'C++ SIMD exact')

def compare(cases,label):
    before=sources();ds=drivers();ids={p.name:m.sha(p) for p in BIN.iterdir()};counts={};records=[]
    with zipfile.ZipFile(ART/(label+'.zip'),'w',compression=zipfile.ZIP_DEFLATED) as archive:
        for name,b in cases:
            op=struct.unpack_from('<I',b,4)[0];c=ds['cpp-scalar'].call(b);expected=ds['rust-scalar'].call(b)
            if c[0]!=-3:assert (c[0]==0)==(expected[0]==0) and (c[0]!=0 or c[1]==expected[1]),(name,'scalar oracle',c[0],expected[0])
            for level,d in ds.items():
                if level.startswith('cpp'):continue
                actual=d.call(b)
                assert actual[0]==expected[0] and (actual[0]!=0 or actual[1]==expected[1]),(name,level,actual[0],expected[0])
                if op not in [8,9,22,23,24,25]:
                    into=bytearray(b);struct.pack_into('<I',into,16,struct.unpack_from('<I',b,16)[0]|128)
                    a=d.call(into);assert a[0]==expected[0] and (a[0]!=0 or a[1]==expected[1]),(name,level,'caller buffer')
                counts[level]=counts.get(level,0)+1
            conformance=cpp_simd_conformance(name,b,expected,ds['cpp-simd'].call(b)) if c[0]!=-3 else None
            archive.writestr(name+'.input',b);archive.writestr(name+'.output',expected[1] if expected[0]==0 else b'')
            records.append({'case':name,'conformance':conformance,'status':expected[0],'input_sha256':hashlib.sha256(b).hexdigest(),'output_sha256':hashlib.sha256(expected[1]).hexdigest() if expected[0]==0 else None})
            if len(records)%500==0:print(label,len(records),'zero mismatches',flush=True)
    for d in ds.values():d.close()
    assert before==sources() and ids=={p.name:m.sha(p) for p in BIN.iterdir()}
    save(ART/(label+'.json'),{'sources':before,'binaries':ids,'cases':records,'counts_per_level':counts,'mismatches':0,'archive_sha256':m.sha(ART/(label+'.zip'))})
    print(label,len(records),'PASS',flush=True)

def admission():
    gpu=subprocess.run(['/mnt/linux-extra/moss-coord/bin/gpu-lease.sh','status'],capture_output=True,text=True)
    if gpu.returncode or 'GPU lease: FREE' not in gpu.stdout or 'HELD' in gpu.stdout:return {'admitted':False,'gpu':gpu.stdout+gpu.stderr}
    units=subprocess.run(['systemctl','--user','list-units','moss-scoreboard-*','--state=active','--no-legend','--plain'],capture_output=True,text=True,check=True).stdout
    measuring=[]
    for line in units.splitlines():
        unit=line.split()[0]
        group=subprocess.check_output(['systemctl','--user','show',unit,'-p','ControlGroup','--value'],text=True).strip()
        for p in (Path('/sys/fs/cgroup')/group.lstrip('/')).rglob('cgroup.procs'):
            for pid in p.read_text().split():
                try:cmd=(Path('/proc')/pid/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace')
                except FileNotFoundError:continue
                if 'moss-scoreboard.sh' in cmd or 'moss-scoreboard.py' in cmd:measuring.append({'pid':pid,'cmd':cmd})
    return {'admitted':not measuring,'gpu':gpu.stdout,'units':units,'measuring':measuring,'load':os.getloadavg(),'unix':time.time()}

def interval(values):
    logs=[math.log(x) for x in values];center=statistics.mean(logs)
    # Student-t critical values for n=5..20 (two-sided 95%).
    critical=[2.776,2.571,2.447,2.365,2.306,2.262,2.228,2.201,2.179,2.160,2.145,2.131,2.120,2.110,2.101,2.093][len(values)-5] if len(values)<=20 else 2.045
    radius=critical*statistics.stdev(logs)/math.sqrt(len(logs))
    return [math.exp(center-radius),math.exp(center+radius)]

def family(name):
    if 'filter-' in name:return 'color' if 'filter-4-' in name else ['oct','quat','exp'][int(name.split('filter-')[1][0])-1]
    if 'view-' in name:return 'view-none' if 'view-none' in name else 'view-filtered'
    if name.startswith('meshlet'):return 'meshlet-raw' if name.startswith('meshlet-raw') else 'meshlet'
    if name.startswith('index'):return 'sequence' if name.startswith('index-3') else 'index'
    return 'vertex'

def limit(name):return 1.50 if family(name) in ['oct','quat','exp','color','view-filtered','index','sequence'] else 1.30

def bench(cases,diagnostic=False):
    path=ART/('diagnostic.json' if diagnostic else 'performance.json')
    if path.exists() and not diagnostic:raise ValueError('final matrix already exists; do not repeat')
    ds={'scalar':m.Driver(BIN/'rust-scalar'),'rust':Rust(),'simd':m.Driver(BIN/'cpp-simd'),'cpp-scalar':m.Driver(BIN/'cpp-scalar'),'sse2':Rust('sse2')}
    raw=[];before=sources();ids={p.name:m.sha(p) for p in BIN.iterdir()};burst=time.monotonic()
    record={'method':'owner 5–20 pair early stopping; 5 diagnostic pairs; paired 95% Student-t log-ratio interval; borderline-only fresh D146; <15-minute bursts','cpu':int(os.environ.get('MESHOPT_BENCH_CPU',min(os.sched_getaffinity(0)))),'sources':before,'binaries':ids,'bar_sha256':m.sha(BAR),'rows':raw,'admissions':[]}
    for api in ['allocating','caller-buffer']:
        for name,b in cases:
            if time.monotonic()-burst>780:
                for d in ds.values():d.close()
                save(path,record);print('releasing cores between bursts',flush=True);time.sleep(10)
                ds={'scalar':m.Driver(BIN/'rust-scalar'),'rust':Rust(),'simd':m.Driver(BIN/'cpp-simd'),'cpp-scalar':m.Driver(BIN/'cpp-scalar'),'sse2':Rust('sse2')};burst=time.monotonic()
            probe=bytearray(b)
            if api=='caller-buffer':struct.pack_into('<I',probe,16,struct.unpack_from('<I',probe,16)[0]|128)
            gate=admission();record['admissions'].append(gate)
            while not gate['admitted']:
                save(path,record);print('timing paused: GPU/scoreboard',flush=True);time.sleep(2);gate=admission();record['admissions'].append(gate)
            struct.pack_into('<I',probe,32,1);trial=ds['simd'].call(probe)[2][0]
            iterations=max(1,min(1000000,math.ceil(.004/max(trial,1e-9))));struct.pack_into('<I',probe,36,iterations)
            active={k:d for k,d in ds.items() if k!='sse2' or family(name) in ['vertex','view-none','view-filtered']}
            row={'case':name,'api':api,'family':family(name),'iterations':iterations,'raw_seconds':{k:[] for k in active},'telemetry':[],'discarded':[],'output_sha256':{}}
            while len(row['raw_seconds']['rust'])<(5 if diagnostic else 20):
                gate=admission()
                if not gate['admitted']:
                    save(path,record);time.sleep(2);continue
                pair={};hashes={};i=len(row['raw_seconds']['rust']);order=list(active);rotation=i%len(order);order=order[rotation:]+order[:rotation]
                for k in order:
                    status,data,t=ds[k].call(probe);assert status==0,(name,k,status);pair[k]=t[0];hashes[k]=hashlib.sha256(data).hexdigest()
                after=admission()
                if not after['admitted']:
                    row['discarded'].append({'pair':pair,'before':gate,'after':after});continue
                for k in active:row['raw_seconds'][k].append(pair[k])
                row['telemetry'].append({'before':gate,'after':after,'order':order});row['output_sha256']=hashes
                if len(row['raw_seconds']['rust'])>=5:
                    ci=interval([x/y for x,y in zip(row['raw_seconds']['rust'],row['raw_seconds']['simd'])]);row['interval']=ci
                    if ci[1]<=limit(name) or ci[0]>limit(name):break
            if not diagnostic and row['interval'][0]<=limit(name)<row['interval'][1]:
                stage={k:[] for k in active};stage_telemetry=[]
                while len(stage['rust'])<30:
                    gate=admission()
                    if not gate['admitted']:save(path,record);time.sleep(2);continue
                    pair={};order=list(active);i=len(stage['rust']);rotation=i%len(order);order=order[rotation:]+order[:rotation]
                    for k in order:
                        status,data,t=active[k].call(probe);assert status==0;pair[k]=t[0]
                    after=admission()
                    if not after['admitted']:row['discarded'].append({'stage':2,'pair':pair,'before':gate,'after':after});continue
                    for k in active:stage[k].append(pair[k])
                    stage_telemetry.append({'before':gate,'after':after,'order':order})
                row['stage2']={'raw_seconds':stage,'telemetry':stage_telemetry,'interval':interval([x/y for x,y in zip(stage['rust'],stage['simd'])])}
                row['stage1_interval']=row['interval'];row['interval']=row['stage2']['interval']
            row['median_seconds']={k:statistics.median(v) for k,v in row['raw_seconds'].items()}
            row['time_ratio']=row['median_seconds']['rust']/row['median_seconds']['simd']
            row['scalar_rust_ratio']=row['median_seconds']['rust']/row['median_seconds']['scalar']
            row['worst_limit']=limit(name);row['verdict']='pass' if row['interval'][1]<=limit(name) else 'fail' if row['interval'][0]>limit(name) else 'borderline'
            s3raw=row['stage2']['raw_seconds'] if 'stage2' in row else row['raw_seconds']
            row['s3']={k:{'interval':interval([x/y for x,y in zip(s3raw[k],s3raw['scalar'])]),'pass':interval([x/y for x,y in zip(s3raw[k],s3raw['scalar'])])[0]<=1.0} for k in ['rust','sse2'] if k in s3raw}
            row['s3_applicable']=family(name) not in ['index','sequence'] and not name.startswith('filter-')
            raw.append(row);save(path,record)
            print(api,name,round(row['time_ratio'],3),'scalar',round(row['scalar_rust_ratio'],3),len(row['raw_seconds']['rust']),row['verdict'],flush=True)
    for d in ds.values():d.close()
    assert sources()==before and ids=={p.name:m.sha(p) for p in BIN.iterdir()}
    record['complete']=True;save(path,record)


def main():
    p=argparse.ArgumentParser();p.add_argument('action',choices=['build','inputs','fixtures','sweep','bench','diagnostic']);p.add_argument('--cases',type=int,default=1000);p.add_argument('--select',default='');args=p.parse_args();ART.mkdir(parents=True,exist_ok=True)
    if args.action=='build':build()
    elif args.action=='inputs':corpus()
    elif args.action=='fixtures':
        fixed=r02.fixtures();compare(fixed,'fixtures');compare(r02.malformed(fixed),'malformed')
        cpp=m.Driver(BIN/'cpp-scalar');fixed04=r04.fixtures04();compare([(n,b) for n,b,_ in fixed04],'fixtures04');compare([(n,b) for n,b,_ in r04.malformed04(fixed04,cpp)],'malformed04');cpp.close()
        compare(corpus(),'benchmark-identity')
    elif args.action=='sweep':
        cpp=m.Driver(BIN/'cpp-scalar');compare(r02.generated(cpp,20261005,args.cases),'sweep02');compare([(n,b) for n,b,_ in r04.generated04(cpp,20261005,args.cases)],'sweep04');cpp.close()
    else:
        cases=corpus()
        if args.action=='diagnostic':cases=[(n,b) for n,b in cases if n in ['vertex-v0-resident-s4','vertex-v0-resident-s12','vertex-v1-resident-s12','varied-filter-1-resident-s4','varied-filter-1-resident-s8','varied-filter-2-resident-s8','varied-filter-3-resident-s12','varied-filter-4-resident-s8','meshlet-64-126-v4-t3','meshlet-raw-64-126']]
        if args.select: cases=[(n,b) for n,b in cases if any(s in n for s in args.select.split(','))]
        bench(cases,args.action=='diagnostic')
if __name__=='__main__':main()
