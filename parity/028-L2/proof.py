#!/usr/bin/env python3
"""Targeted baseline exact proof and existing C++ protocols; no timing."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import array, hashlib, json, os, random, shutil, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-028-L2'))
REF = Path(_public_path('<refs>/meshoptimizer'))
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
sys.path.insert(0, str(ROOT/'parity'))
import runner as r
import p01x
ENV = dict(os.environ, RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', RUSTFLAGS='', CARGO_ENCODED_RUSTFLAGS='', CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0')
source = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in (ROOT/'src').rglob('*.rs')}
record = dict(source=source, gates={}, frozen=[], cpp=[], passed=False)
(ART/'proof-outputs').mkdir(exist_ok=True)
def save(): (ART/('proof-protocol.json' if os.environ.get('MESHOPT_PROTOCOL_ONLY') else 'proof.json')).write_text(json.dumps(record,indent=2)+'\n')
def run(name, cmd):
    with (ART/f'proof-{name}.log').open('w') as log:
        result = subprocess.run([str(x) for x in cmd], env=ENV, stdout=log, stderr=subprocess.STDOUT)
    record['gates'][name] = result.returncode; save()
    if result.returncode:
        print((ART/f'proof-{name}.log').read_text()[-4000:]); raise RuntimeError(name)
if not os.environ.get('MESHOPT_PROTOCOL_ONLY'):
    run('prepare', ['python3', ROOT/'parity/028-L2/prepare-proof.py'])
    for mode in ['std','no-std']:
        cmd = ['cargo','build','--offline','--release','--manifest-path',ART/'proof/Cargo.toml']
        if mode == 'no-std': cmd += ['--no-default-features']
        run('build-'+mode, cmd)
        binary = ART/f'exact-{mode}'
        shutil.copy2(TARGET/'release/cook-chain-exact',binary)
        run('exact-'+mode, [binary, ART/'inputs'])
    # Frozen public adapter outputs must be byte-exact against both base and M7.
    candidate = sys.argv[1]
    for case in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']:
        for op in ['cache','cache-fifo','simplify-scale','overdraw']:
            values = {}
            for arm,binary in [('candidate',candidate),('base',ART/'driver-base-1'),('M7',ART/'driver-M7-1')]:
                output = ART/'proof-outputs'/f'{case}-{op}-{arm}.bin'
                p = subprocess.Popen([str(binary),op,str(ART/'inputs'/f'{case}.bin'),str(output)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
                assert p.stdout.readline().strip() == 'READY'
                p.communicate('stop\n'); assert p.returncode == 0
                values[arm] = hashlib.sha256(output.read_bytes()).hexdigest()
            assert len(set(values.values())) == 1, (case,op,values)
            record['frozen'].append(dict(case=case,operation=op,hashes=values)); save()
    # Check every real trace output with clocks disabled before wall sampling.
    for trace in json.loads((ART/'stages.json').read_text()):
        output = ART / ('stage-proof-'+trace['case']+'.tsv')
        run('stage-'+trace['case'], [candidate,'chain-check',trace['manifest'],output])
        for line,expected in zip(output.read_text().splitlines(),trace['calls'],strict=True):
            index,op,ns,file = line.split('\t')
            assert int(ns) == 0 and op == expected['operation']
            assert hashlib.sha256(Path(file).read_bytes()).hexdigest() == expected['output_sha256'], (trace['case'],index,op)
            Path(file).unlink()

# Existing native oracle, strict scalar C++ 1.3 and current Rust protocol.
run('oracle-identity',[ROOT/'parity/check-reference.sh']) if os.environ.get('MESHOPT_REFERENCE') else None
ENV['MESHOPT_REFERENCE'] = str(REF)
run('oracle-identity',[ROOT/'parity/check-reference.sh'])
cpp = ART/'cpp-reference'
run('cpp-build',['c++',*r.FLAGS,'-I',REF/'src',ROOT/'parity/reference.cpp',*[REF/'src'/f for f in ['vcacheoptimizer.cpp','overdrawoptimizer.cpp','allocator.cpp','simplifier.cpp','indexgenerator.cpp','vfetchoptimizer.cpp','quantization.cpp']],'-o',cpp])
run('rust-protocol-build',['cargo','build','--offline','--locked','--release','--manifest-path',ROOT/'parity/Cargo.toml','--features','meshoptimizer-rs/std'])
rust = ART/'rust-protocol'
shutil.copy2(TARGET/'release/meshopt-driver',rust)
cases = list(r.fixtures())
rng = random.Random(20261002)
for i in range(128):
    p,ix,t,kind = r.random_mesh(rng,i)
    cases.append((f'existing-seed-20261002-{i}-{kind}',p,ix,t))
for name,p,ix,threshold in cases:
    for op in [1,2,6]:
        indices = ix
        if op == 2 and len(ix):
            cached,_ = r.response(r.execute(rust,r.message(1,p,ix)),len(ix))
            indices = array.array('I'); indices.frombytes(cached)
        data = r.message(op,p,indices,threshold)
        a,b = r.execute(cpp,data),r.execute(rust,data)
        for arm,value in [('input',data),('cpp',a),('rust',b)]:
            (ART/'proof-outputs'/f'{name}-{op}.{arm}').write_bytes(value)
        assert a == b,(name,op)
        record['cpp'].append(dict(case=name,operation=op,input_sha256=hashlib.sha256(data).hexdigest(),output_sha256=hashlib.sha256(a).hexdigest())); save()
# FIFO uses the existing MO02 protocol and its fixture/seed generator.
for i,(name,p,ix) in enumerate(p01x.geometries()):
    for cache_variant in range(8):
        data = p01x.message(8,p,ix,cache_variant)
        a,b = r.execute(cpp,data),r.execute(rust,data)
        for arm,value in [('input',data),('cpp',a),('rust',b)]:
            (ART/'proof-outputs'/f'fifo-{name}-{cache_variant}.{arm}').write_bytes(value)
        assert a == b, ('fifo',name,cache_variant)
        record['cpp'].append(dict(case=name,operation='fifo',cache_variant=cache_variant,output_sha256=hashlib.sha256(a).hexdigest())); save()
run('existing-seeds', ['python3', ROOT/'parity/028-L2/seed-proof.py'])
assert source == {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in (ROOT/'src').rglob('*.rs')}
record['passed'] = True; save()
print('PASS', len(record['frozen']), 'frozen and',len(record['cpp']),'existing C++ protocol cases')
