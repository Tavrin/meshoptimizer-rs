"""P4 bounded exact observations and pinned C++ protocol differential.

Invoke under CPU admission, with the environment used by run.py and
MESHOPT_REFERENCE pointing to the clean pinned oracle. Proof source is shared
with existing protocol consumers; exact.rs narrows the boundary campaign to
four functions and a finite, complete work sweep on each selected small case.
"""
from pathlib import Path
import os
import shutil
import sys
import json
import subprocess
import resource
import run as r

S, ROOT, LANE, T = r.S, r.ROOT, r.LANE, r.T
TAG = os.environ.get('P4_PROOF_TAG', '')
original_run = r.run

def tagged_run(name, command, **kwargs):
    return original_run(TAG + name, command, **kwargs)

r.run = tagged_run

def main():
    d=S/'proof'
    d.mkdir()
    r.write(d/'source-binding.json',dict(exact=r.sha(LANE/'exact.rs'),supervisor=r.sha(LANE/'proof.py'),sources={str(p.relative_to(ROOT)):r.sha(p) for p in (ROOT/'src').rglob('*.rs')}))
    b=d/'base'
    shutil.copytree(S/'base/src',b/'src')
    shutil.copytree(S/'base/docs',b/'docs')
    for name in ['Cargo.toml','Cargo.lock','README.md']:
        shutil.copy2(S/'base'/name,b/name)
    manifest=(b/'Cargo.toml').read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1)
    (b/'Cargo.toml').write_text(manifest)
    (d/'Cargo.toml').write_text('''[package]
name = "p4-exact"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/simd", "candidate/simd", "baseline/experimental", "candidate/experimental", "baseline/parallel", "candidate/parallel", "baseline/clusterlod", "candidate/clusterlod", "baseline/parity-internals", "candidate/parity-internals"]
[dependencies]
baseline = {package="meshoptimizer-baseline",path="base",default-features=false}
candidate = {package="meshoptimizer-rs",path='''+json.dumps(str(ROOT))+''',default-features=false}
[[bin]]
name = "p4-exact"
path = '''+json.dumps(str(LANE/'exact.rs'))+'''
[profile.release]
lto = "thin"
codegen-units = 1
''')
    r.run('exact-lock',['cargo','generate-lockfile','--offline','--manifest-path',str(d/'Cargo.toml')])
    before=resource.getrusage(resource.RUSAGE_CHILDREN)
    for mode in ['all','none']:
        args=['--features','all'] if mode=='all' else []
        r.run('exact-build-'+mode,['cargo','build','--offline','--locked','--release','--manifest-path',str(d/'Cargo.toml'),*args])
        binary=d/('exact-'+mode)
        shutil.copy2(T/'release/p4-exact',binary)
        r.run('exact-'+mode,[str(binary)])
        # A bounded append-only seeded stream supplements the complete sweeps.
        total = 0.0
        seed = 21640000
        with (d/('fuzz-'+mode+'.jsonl')).open('x') as journal:
            while total < 150:
                start = resource.getrusage(resource.RUSAGE_CHILDREN)
                child = subprocess.run([str(binary),str(seed)],capture_output=True)
                end = resource.getrusage(resource.RUSAGE_CHILDREN)
                cpu=end.ru_utime+end.ru_stime-start.ru_utime-start.ru_stime
                row=dict(seed=seed,exit=child.returncode,cpu_seconds=cpu,binary=r.sha(binary))
                journal.write(json.dumps(row)+'\n');journal.flush();os.fsync(journal.fileno())
                assert child.returncode==0,(seed,child.stderr)
                seed+=1;total+=cpu
        print('fuzz',mode,seed-21640000,total,flush=True)
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    r.write(d/'cpu.json',dict(child_cpu_seconds=(after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime)))
    # Reuse the pinned protocol corpus, including exact float and mutable output
    # serialization; no comparison is inferred from a successful exit alone.
    os.environ.update(MESHOPT_SCRATCH=str(S),MESHOPT_QUALIFY_TARGET=str(T))
    out=d/'0216b-protocol';out.mkdir()
    ref=Path(os.environ['MESHOPT_REFERENCE'])
    r.run('reference-pin',['bash','parity/check-reference.sh'])
    sources=[ref/'src'/name for name in ['vcacheoptimizer.cpp','overdrawoptimizer.cpp',
        'allocator.cpp','simplifier.cpp','indexgenerator.cpp','vfetchoptimizer.cpp','quantization.cpp']]
    r.run('cpp-build',['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD',
        '-fno-fast-math','-ffp-contract=off','-I'+str(ref/'src'),str(ROOT/'parity/reference.cpp'),
        *map(str,sources),'-o',str(out/'cpp-reference')])
    for mode in ['all','none']:
        features=['std','simd','parallel','clusterlod','experimental','parity-internals'] if mode=='all' else []
        (out/'Cargo.toml').write_text('''[package]
name = "meshoptimizer-parity"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {path='''+json.dumps(str(ROOT))+''',default-features=false,features='''+json.dumps(features)+'''}
libm = {version="=0.2.16",default-features=false}
[lib]
path = '''+json.dumps(str(ROOT/'parity/src/lib.rs'))+'''
[[bin]]
name = "meshopt-driver"
path = '''+json.dumps(str(ROOT/'parity/src/main.rs'))+'''
[profile.release]
codegen-units = 1
lto = "fat"
''')
        r.run('protocol-lock-'+mode,['cargo','generate-lockfile','--offline','--manifest-path',str(out/'Cargo.toml')])
        r.run('protocol-build-'+mode,['cargo','build','--offline','--locked','--release','--manifest-path',str(out/'Cargo.toml')])
        shutil.copy2(T/'release/meshopt-driver',out/('rust-'+mode))
    r.run('cpp-differential',[sys.executable,str(ROOT/'parity/0216-qualify/protocol-modes.py')])
    for mode in ['all','none']:
        shutil.copy2(d/'0216b-protocol'/('exit-'+mode+'.json'),LANE/('cpp-'+mode+'.json'))
    for name,cmd in [
        ('tests-default',['cargo','test','--locked']),
        ('tests-none',['cargo','test','--locked','--no-default-features']),
        ('tests-all',['cargo','test','--locked','--all-features']),
        ('clippy',['cargo','clippy','--locked','--all-features','--all-targets','--','-D','warnings']),
        ('fmt',['cargo','fmt','--check']),
        ('unsafe-boundary',['bash','parity/check-unsafe-boundary.sh']),
        ('public-text',['bash','parity/tools/check_public_text.sh'])]:
        r.run(name,cmd)
    r.write(LANE/'proof-receipts.json',{p.stem:json.loads(p.read_text()) for p in (S/'logs').glob('*.json') if not p.name.startswith('build-')})

if __name__=='__main__':main()
