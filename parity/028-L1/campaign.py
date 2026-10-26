#!/usr/bin/env python3
"""One registered, pinned-core, interleaved final campaign. Refuses overwrite."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import os,sys,json,math,random,statistics,subprocess,struct,hashlib,shutil,time
from pathlib import Path
from control import ROOT,LANE,ART,TARGET,OPS,CASES,Driver,sha,save
CORE=2

def bootstrap(pairs):
    logs=[math.log(a/b)for a,b in pairs];rng=random.Random(20261030);n=len(logs)
    res=sorted(sum(logs[rng.randrange(n)]for _ in logs)/n for _ in range(10000))
    return {'ratio':math.exp(statistics.mean(logs)),'lower95':math.exp(res[249]),'upper95':math.exp(res[9749])}

def build_rfc():
    assert os.environ.get('MOSS_HEAVY_ACTIVE')
    records=[]
    cmd=['cargo','build','--locked','--offline','--manifest-path',str(LANE/'Cargo.toml'),'--profile','moss','--features','ours,full','--bin','l1-rfc113']
    with (ART/'rfc-build.log').open('x')as f:p=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
    records.append({'command':cmd,'exit':p.returncode});save(ART/'rfc-build.json',records);assert p.returncode==0
    shutil.copy2(TARGET/'moss/l1-rfc113',ART/'rfc-ours')
    vendor=Path(_public_path('<scratch>/meshopt-cluster-dag/reference-adapter/vendor/meshoptimizer'))
    cmd=['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-I'+str(vendor/'src'),'-I'+str(vendor),str(LANE/'rfc113-bridge.cpp'),str(LANE/'rfc113-reference.cpp'),*[str(p)for p in sorted((vendor/'src').glob('*.cpp'))],'-o',str(ART/'rfc-cpp')]
    with (ART/'rfc-cpp-build.log').open('x')as f:p=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
    records.append({'command':cmd,'exit':p.returncode});save(ART/'rfc-build.json',records);assert p.returncode==0
    old=Path(_public_path('<scratch>/meshopt-cluster-c'));cases=json.loads((old/'cases.json').read_text())
    folder=ART/'rfc-inputs';folder.mkdir(exist_ok=True)
    for c in cases:
        source=Path(c['input']);assert sha(source)==c['input_sha256'];dst=folder/source.name;shutil.copy2(source,dst);c['input']=str(dst)
    save(ART/'rfc-cases.json',cases)
    save(ART/'rfc-identity.json',{'source':{str(p):sha(p)for p in [LANE/'rfc113.rs',LANE/'rfc113-bridge.cpp',LANE/'rfc113-reference.cpp',vendor/'clusterlod.h',*sorted((vendor/'src').glob('*.cpp'))]},'binaries':{a:sha(ART/f'rfc-{a}')for a in ['ours','cpp']}})
class RfcDriver:
    def __init__(self,arm):self.p=subprocess.Popen(['taskset','-c',str(CORE),str(ART/f'rfc-{arm}')],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    def run(self,data):
        self.p.stdin.write(struct.pack('<I',len(data))+data);self.p.stdin.flush();h=self.p.stdout.read(4);assert len(h)==4
        n=struct.unpack('<I',h)[0];out=self.p.stdout.read(n);assert len(out)==n
        return (out[:-8],struct.unpack_from('<Q',out,len(out)-8)[0])if data[:4]==b'R11T' else (out,None)
    def close(self):self.p.stdin.close();self.p.wait();assert self.p.returncode==0

def preflight_rfc():
    pool={a:RfcDriver(a)for a in ['cpp','ours']};rows=[]
    try:
        for c in json.loads((ART/'rfc-cases.json').read_text()):
            data=Path(c['input']).read_bytes();assert hashlib.sha256(data).hexdigest()==c['input_sha256']
            outs={a:d.run(data)[0]for a,d in pool.items()};assert outs['cpp']==outs['ours'],c['name']
            if c['set']=='S1':assert outs['ours']==Path(c['oracle']).read_bytes()
            rows.append({'set':c['set'],'case':c['name'],'sha256':hashlib.sha256(outs['ours']).hexdigest()});save(ART/'rfc-preflight.json',rows)
    finally:
        for d in pool.values():d.close()

def wall():
    assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
    qualification=json.loads((ART/'QUALIFIED.json').read_text())
    assert qualification['exit']==0
    assert all(sha(ROOT/p)==h for p,h in qualification['source'].items()),'qualified source changed'
    assert CORE in os.sched_getaffinity(0)
    identity={'source':{str(p.relative_to(ROOT)):sha(p)for p in list((ROOT/'src').rglob('*.rs'))+list(LANE.rglob('*.rs'))+list(LANE.glob('*.py'))+[LANE/'Cargo.toml',LANE/'Cargo.lock',LANE/'REGISTRATION.md']},'binaries':{p.name:sha(p)for p in ART.glob('driver-final-*')},'inputs':json.loads((ART/'inputs.json').read_text()),'baseline_source':{str(p.relative_to(ART/'baseline')):sha(p)for p in (ART/'baseline/src').rglob('*.rs')},'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx':subprocess.check_output(['c++','--version'],text=True),'siblings':Path(f'/sys/devices/system/cpu/cpu{CORE}/topology/thread_siblings_list').read_text().strip(),'core':CORE,'load_before':os.getloadavg(),'start':time.time()}
    with (ART/'campaign-identity.json').open('x')as f:json.dump(identity,f,indent=2)
    # This exclusive marker is the one-campaign guard; partial campaigns are failures.
    rows=[];floor=[]
    for op in OPS:
        for case in CASES:
            n=5000 if case=='tiny-smooth' else 2 if case=='million-smooth' else 16
            for comparator in ['AA','M7','base','X']:
                a=Driver('final','base'if comparator=='AA' else 'ours',op,case,CORE)
                b=Driver('final','base'if comparator=='AA' else comparator,op,case,CORE)
                try:
                    a.sample(n);b.sample(n)
                    pairs=[]
                    for i in range(10):
                        if i%2==0:av=a.sample(n);bv=b.sample(n)
                        else:bv=b.sample(n);av=a.sample(n)
                        pairs.append([av,bv])
                        with (ART/'wall-raw.jsonl').open('a')as f:f.write(json.dumps({'op':op,'case':case,'comparator':comparator,'pair':i,'order':'AB'if i%2==0 else'BA','a_ns':av,'b_ns':bv,'iterations':n,'load':os.getloadavg()})+'\n')
                    stats=bootstrap(pairs)
                    row={'op':op,'case':case,'comparator':comparator,'pairs':pairs,**stats};rows.append(row);save(ART/'wall.json',rows)
                    if comparator=='AA':floor.append(max(abs(stats['lower95']-1),abs(stats['upper95']-1)))
                finally:a.close();b.close()
            print('wall',op,case,flush=True)
    F=max(floor);table=[]
    for op in OPS:
        gms={arm:math.exp(statistics.mean(math.log(r['ratio'])for r in rows if r['op']==op and r['comparator']==arm))for arm in ['M7','base','X']}
        upper=max(r['upper95']for r in rows if r['op']==op and r['comparator']=='base')
        regression=gms['base']<=1 and upper<=1.10+F
        verdict='PASS'if regression and gms['M7']<=1 else'PARTIAL'if regression else'FAIL'
        table.append({'family':op,**gms,'max_case_upper95_base':upper,'F':F,'verdict':verdict})
    save(ART/'registered-table.json',table)
    pool={a:RfcDriver(a)for a in ['ours','cpp']};rfc=[]
    try:
        for c in json.loads((ART/'rfc-cases.json').read_text()):
            data=Path(c['input']).read_bytes();data=b'R11T'+data[4:]
            for d in pool.values():d.run(data)
            pairs=[]
            for i in range(10):
                if i%2==0:ao,at=pool['ours'].run(data);bo,bt=pool['cpp'].run(data)
                else:bo,bt=pool['cpp'].run(data);ao,at=pool['ours'].run(data)
                assert ao==bo,c['name'];pairs.append([at,bt])
                with (ART/'rfc-wall-raw.jsonl').open('a')as f:f.write(json.dumps({'set':c['set'],'case':c['name'],'pair':i,'ours_ns':at,'cpp_ns':bt,'order':'AB'if i%2==0 else'BA','load':os.getloadavg()})+'\n')
            rfc.append({'set':c['set'],'case':c['name'],'pairs':pairs,'ours_median':statistics.median(p[0]for p in pairs),'cpp_median':statistics.median(p[1]for p in pairs)})
            save(ART/'rfc-wall.json',rfc);print('rfc',c['set'],c['name'],flush=True)
    finally:
        for d in pool.values():d.close()
    disclosure={s:sum(r['ours_median']for r in rfc if r['set']==s)/sum(r['cpp_median']for r in rfc if r['set']==s)for s in ['S1','S2']}
    disclosure['all']=sum(r['ours_median']for r in rfc)/sum(r['cpp_median']for r in rfc)
    disclosure['max_case']=max(r['ours_median']/r['cpp_median']for r in rfc)
    save(ART/'rfc-disclosure.json',disclosure)
    assert all(sha(ROOT/p)==h for p,h in identity['source'].items()),'source changed during campaign'
    assert all(sha(ROOT/p)==h for p,h in qualification['source'].items()),'qualified source changed during wall campaign'
    save(ART/'campaign-complete.json',{'exit':0,'finished':time.time(),'F':F,'disclosure':disclosure,'load_after':os.getloadavg()})
if __name__=='__main__':
    if sys.argv[1]=='build-rfc':build_rfc()
    elif sys.argv[1]=='preflight-rfc':preflight_rfc()
    else:wall()
