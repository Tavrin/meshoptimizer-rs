from pathlib import Path
import hashlib, json, os, shutil, subprocess, sys
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
assert os.environ.get('MESHOPT_ADMISSION_ID') and os.environ['MESHOPT_GPU']=='0'
phase=sys.argv[1]
ROOT=Path(__file__).resolve().parents[2]
if phase=='final':
    sources={str(p):hashlib.sha256(p.read_bytes()).hexdigest()
             for p in [ROOT/'Cargo.toml',ROOT/'Cargo.lock',*sorted((ROOT/'src').rglob('*.rs')),ROOT/'parity/0212-overdraw/exact.rs',*sorted((ROOT/'tests').rglob('*.rs')),*sorted((ROOT/'examples').rglob('*.rs')),ROOT/'docs/USING.md',ROOT/'README.md',ROOT/'parity/reference.cpp',ROOT/'parity/runner.py',*sorted((ROOT/'parity/src').rglob('*.rs')),ROOT/'parity/Cargo.toml',ROOT/'parity/Cargo.lock']}
    (A/'final-source.json').write_text(json.dumps(sources,indent=2)+'\n')
    subprocess.run(['tar','-czf',str(A/'final-source.tar.gz'),'src','Cargo.toml','Cargo.lock','tests','examples','docs/USING.md','README.md','parity/reference.cpp','parity/runner.py','parity/src','parity/Cargo.toml','parity/Cargo.lock'],cwd=ROOT,check=True)
    versions={t:subprocess.check_output([t,'--version'],text=True) for t in ['rustc','cargo','c++']}
    versions['perf']=subprocess.check_output([os.environ['MESHOPT_PERF'],'--version'],text=True)
    (A/'tool-versions.json').write_text(json.dumps(versions,indent=2)+'\n')
for arm in (['base','A','B','AB','C','M7','X'] if phase=='attribution' else ['D','E','F','G','H','I'] if phase=='predicate' else ['J'] if phase=='block' else ['base','candidate','M7','X']):
    log=A/f'{phase}-build-{arm}.log'
    if log.exists():
        assert json.loads((A/f'{phase}-build-{arm}.json').read_text())['exit']==0
        continue
    cmd=['cargo','build','--offline','--manifest-path',str(A/f'harness-{arm}/Cargo.toml'),'--profile',os.environ['MESHOPT_PROFILE'],'--bin','cmp-driver','--features',{'M7':'peer7','X':'optimesh'}.get(arm,'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod')]
    if phase=='final':
        # Resolve changed path packages without compilation, then lock builds.
        metadata=['cargo','metadata','--offline','--format-version','1','--manifest-path',str(A/f'harness-{arm}/Cargo.toml')]
        with (A/f'final-metadata-{arm}.json').open('x') as out, (A/f'final-metadata-{arm}.log').open('x') as err:
            result=subprocess.run(metadata,stdout=out,stderr=err)
        assert result.returncode==0, (arm,result.returncode)
        cmd.append('--locked')
    if arm in ['M7','X']:cmd.append('-vv')
    with log.open('x') as f:r=subprocess.run(cmd,env=dict(os.environ,CC_ENABLE_DEBUG_OUTPUT='1'),stdout=f,stderr=subprocess.STDOUT)
    receipt={'command':cmd,'exit':r.returncode,'admission':os.environ['MESHOPT_ADMISSION_ID'],'harness_sha256':{str(p.relative_to(A/f'harness-{arm}')):hashlib.sha256(p.read_bytes()).hexdigest() for p in (A/f'harness-{arm}').rglob('*') if p.is_file()}}
    if not r.returncode:
        binary=A/f'{phase}-driver-{arm}';shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/os.environ['MESHOPT_PROFILE']/'cmp-driver',binary)
        receipt['sha256']=hashlib.sha256(binary.read_bytes()).hexdigest()
    (A/f'{phase}-build-{arm}.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(phase,arm,r.returncode,flush=True)
    if r.returncode:print(log.read_text()[-3000:]);sys.exit(r.returncode)

if phase=='predicate':
    for arm in ['base','M7','X']:shutil.copy2(A/f'attribution-driver-{arm}',A/f'predicate-driver-{arm}')
    subprocess.run(['python3',str(ROOT/'parity/0212-overdraw/measure.py'),'predicate'],check=True)

if phase=='block':
    for arm in ['base','M7','X']:shutil.copy2(A/f'attribution-driver-{arm}',A/f'block-driver-{arm}')
    shutil.copy2(A/'predicate-driver-I',A/'block-driver-I')
    subprocess.run(['python3',str(ROOT/'parity/0212-overdraw/measure.py'),'block'],check=True)
