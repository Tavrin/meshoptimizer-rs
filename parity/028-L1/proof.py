#!/usr/bin/env python3
"""Reuse existing p03 differential seeds plus frozen corpus; retain replay corpus."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import os,sys,subprocess,json,struct,hashlib,importlib.util
from control import ROOT,LANE,ART,TARGET,OPS,CASES,Driver,sha,save
os.environ.setdefault('MESHOPT_ARTIFACTS',str(ART/'p03'))
os.environ.setdefault('MESHOPT_REFERENCE',_public_path('<refs>/meshoptimizer'))
spec=importlib.util.spec_from_file_location('p03',ROOT/'parity/p03/runner.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
FAMILIES={1:'meshlets',2:'meshlets-scan',3:'meshlets-flex',4:'meshlets-spatial',12:'partition',14:'spatial-triangles'}
def wide_partition(seed):
    p,indices=r.grid(14 if seed%2==0 else 24,seed%2)
    return r.message(12,p,indices,counts=[1]*len(indices),target=4|(0x80000000 if seed%2 else 0))

class Client:
    def __init__(self,path):self.p=subprocess.Popen([str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    def run(self,data):
        self.p.stdin.write(struct.pack('<I',len(data))+data);self.p.stdin.flush()
        head=self.p.stdout.read(4);assert len(head)==4
        return self.p.stdout.read(struct.unpack('<I',head)[0])
    def close(self):self.p.stdin.close();self.p.wait();assert self.p.returncode==0

def build_proof():
    records=[]
    for arm in ['base','ours']:
        folder=ART/'baseline/parity/028-L1' if arm=='base' else LANE
        if arm=='base':
            import shutil
            folder.mkdir(parents=True,exist_ok=True)
            for f in ['Cargo.toml','Cargo.lock','proof.rs']:shutil.copy2(LANE/f,folder/f)
            shutil.copytree(LANE/'src',folder/'src',dirs_exist_ok=True)
        cmd=['cargo','build','--offline','--locked','--release','--manifest-path',str(folder/'Cargo.toml'),'--features','ours,full','--bin','l1-proof']
        log=ART/f'proof-build-{arm}.log'
        with log.open('w')as f:ret=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
        records.append({'arm':arm,'exit':ret.returncode,'command':cmd,'library_source':{str(p.relative_to(folder.parents[1])):sha(p)for p in (folder.parents[1]/'src').rglob('*.rs')},'witness_sha256':sha(folder/'proof.rs')});save(ART/'proof-builds.json',records)
        assert ret.returncode==0,log.read_text()
        import shutil
        shutil.copy2(TARGET/'release/l1-proof',ART/f'proof-{arm}')

def existing_cpp():
    # Existing differential protocol and generator, restricted to the L1 families.
    subprocess.run([str(ROOT/'parity/check-reference.sh')],check=True)
    cmd=['c++',*r.FLAGS,'-I'+str(r.REF/'src'),'-I'+str(r.REF/'demo'),str(ROOT/'parity/p03/reference.cpp'),*[str(r.REF/f'src/{n}.cpp')for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']],'-o',str(ART/'p03-cpp')]
    with (ART/'p03-cpp-build.log').open('x')as f:ret=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
    assert ret.returncode==0
    cmd=['cargo','build','--locked','--offline','--release','--manifest-path',str(ROOT/'parity/p03/Cargo.toml'),'--bin','meshopt-p03-driver']
    with (ART/'p03-rust-build.log').open('x')as f:ret=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
    assert ret.returncode==0
    import shutil
    shutil.copy2(TARGET/'release/meshopt-p03-driver',ART/'p03-ours')
    save(ART/'cpp-identity.json',{'binary_sha256':{a:sha(ART/f'p03-{a}')for a in ['cpp','ours']},'reference_commit':subprocess.check_output(['git','-C',str(r.REF),'rev-parse','HEAD'],text=True).strip(),'flags':r.FLAGS,'source_sha256':{str(p):sha(p)for p in [ROOT/'parity/p03/reference.cpp',ROOT/'parity/p03/runner.py',ROOT/'parity/p03/src/lib.rs',*sorted((r.REF/'src').glob('*.cpp'))]}})
    clients={a:r.Client(a,ART/f'p03-{a}')for a in ['cpp','ours']}
    rows=[]
    with (ART/'cpp-seeded-corpus.bin').open('xb')as corpus:
        for op,name in FAMILIES.items():
            for seed in range(20261004,20261004+2000):
                data=r.case(op,seed);r.compare(clients,data,f'{name}-{seed}',ART,corpus)
                into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000)
                r.compare(clients,into,f'{name}-{seed}-into',ART,corpus)
            if op==12:
                for seed in range(64):
                    data=wide_partition(seed);r.compare(clients,data,f'wide-partition-{seed}',ART,corpus)
                    into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000);r.compare(clients,into,f'wide-partition-{seed}-into',ART,corpus)
            rows.append({'family':name,'seeds':2000,'variants':['alloc','into'],'mismatches':0,'extra_generation_wrap_seeds':64 if op==12 else 0});save(ART/'cpp-seeded-differential.json',rows)
            print('cpp exact',name,flush=True)
    for c in clients.values():c.close()

def differential(frozen=True, epoch='final'):
    records=[]
    if frozen:
        for op in OPS:
            for case in CASES:
                hashes={};blobs={}
                for arm in ['ours','base','M7','X']:
                    d=Driver('final',arm,op,case);blobs[arm]=d.output.read_bytes();hashes[arm]=sha(d.output);d.close()
                assert blobs['ours']==blobs['base']==blobs['M7'],(op,case,hashes)
                # Meshlet padding differs by API; normalized logical output is identical;
                # raw per-API bytes checked against baseline separately.
                if op.startswith('meshlets'):
                    assert (ART/'outputs'/f'final-ours-{op}-{case}.bin.raw').read_bytes()==(ART/'outputs'/f'final-base-{op}-{case}.bin.raw').read_bytes()
                records.append({'op':op,'case':case,'hashes':hashes,'X_exact':blobs['ours']==blobs['X']});save(ART/'frozen-differential.json',records)
    clients={a:Client(ART/f'proof-{a}')for a in ['base','ours']}
    prefix=''if frozen else epoch+'-'
    corpus=ART/(prefix+'proof-corpus.bin');assert not corpus.exists()
    stats={'comparisons':0,'families':{},'binary_sha256':{a:sha(ART/f'proof-{a}')for a in clients},'result_values_and_work':True}
    with corpus.open('wb')as receipt:
        def compare(data,label):
            values={a:c.run(data)for a,c in clients.items()}
            if values['ours']!=values['base']:
                (ART/'divergence.input').write_bytes(data)
                for a,out in values.items():(ART/f'divergence-{a}.txt').write_bytes(out)
                raise AssertionError(label)
            receipt.write(struct.pack('<II',len(data),len(values['ours']))+data+values['ours'])
            stats['comparisons']+=1
        for op,name in FAMILIES.items():
            for seed in range(20261004,20261004+2000):
                data=bytearray(r.case(op,seed));struct.pack_into('<I',data,44,seed%4);compare(data,[name,seed,'alloc'])
                into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000);compare(into,[name,seed,'into'])
                if seed<20261004+100:
                    for work in [0,1,4,16,64,256,1024,4096]:
                        compare(data+struct.pack('<QQB',work,512*1024*1024,0),[name,seed,'work',work])
                    for short in [1,2,3]:compare(into+struct.pack('<QQB',2**63,512*1024*1024,short),[name,seed,'short',short])
                    for byte_limit in [0,64,256,1024,4096]:compare(data+struct.pack('<QQB',2**63,byte_limit,0),[name,seed,'memory',byte_limit])
            stats['families'][name]=2000
            save(ART/(prefix+'exact-proof.json'),stats);print('exact',name,stats['comparisons'],flush=True)
        # Generation wrap, absent from the original small seeded generator.
        for seed in range(64):
            for layout in range(4):
                data=bytearray(wide_partition(seed));struct.pack_into('<I',data,44,layout)
                compare(data,['partition','generation-wrap',seed,layout])
                into=bytearray(data);struct.pack_into('<I',into,4,12|0x10000)
                compare(into,['partition','generation-wrap-into',seed,layout])
        for case in CASES:
            raw=(ART/'inputs'/f'{case}.bin').read_bytes();nv,ni=struct.unpack_from('<II',raw)
            p=[struct.unpack_from('<3f',raw,8+i*12)for i in range(nv)]
            idx=struct.unpack_from('<'+'I'*ni,raw,8+nv*12)
            for op,name in FAMILIES.items():
                counts=[min(96,ni-i)for i in range(0,ni,96)]if op==12 else []
                data=r.message(op,p,idx,mv=64,min=64,mt=124,split=0.5 if op==4 else 0,counts=counts)
                compare(data,[name,case]);into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000);compare(into,[name,case,'into'])
    for c in clients.values():c.close()
    stats.update(corpus_sha256=sha(corpus),extra_generation_wrap_cases=512,exit=0);save(ART/(prefix+'exact-proof.json'),stats)
if __name__=='__main__':
    assert os.environ.get('PARITY_RUNNER_ACTIVE')
    if sys.argv[1]=='build':build_proof()
    elif sys.argv[1]=='cpp':existing_cpp()
    else:differential(sys.argv[1]=='final',sys.argv[1])
