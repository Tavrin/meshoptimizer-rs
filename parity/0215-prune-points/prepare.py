"""Build source-bound peer and exact drivers in caller-provided scratch."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
scratch = Path(os.environ['MESHOPT_SCRATCH'])
target = Path(os.environ['CARGO_TARGET_DIR'])
art = scratch / 'campaign-1'
art.mkdir(exist_ok=True)

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def run(cmd, name):
    path = art/(name+'.log')
    if path.exists():
        old = art/'previous-attempts'
        old.mkdir(exist_ok=True)
        path.rename(old/(name+'-'+str(len(list(old.iterdir())))+'.log'))
    with path.open('w') as log:
        r = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT)
    assert r.returncode == 0, (name, r.returncode)

# Verify every baseline library byte against the release, including its manifest.
base = scratch / 'base'
for p in [*base.glob('src/**/*.rs'), base/'Cargo.toml', base/'Cargo.lock']:
    expected = subprocess.check_output(['git', '-C', str(root), 'show', 'v0.2.14:'+str(p.relative_to(base))])
    assert p.read_bytes() == expected, p
candidate = scratch / 'candidate'
shutil.rmtree(candidate / 'src')
shutil.copytree(root / 'src', candidate / 'src')
for f in ['Cargo.toml', 'Cargo.lock']:
    shutil.copy2(root/f, candidate/f)
for tree in [base, candidate]:
    dst = tree / 'parity/peers'
    (dst/'src').mkdir(parents=True, exist_ok=True)
    for f in ['Cargo.toml', 'Cargo.lock', 'src/main.rs']:
        shutil.copy2(root/'parity/peers'/f, dst/f)
    for helper in ['prepare_driver.py', 'additional_driver.py']:
        subprocess.run(['python3', str(root/'parity/0216-qualify'/helper), str(dst/'src/main.rs')], check=True)
for arm, feature, tree in [('base','ours',base), ('candidate','ours',candidate),
                           ('cpp','cpp',base), ('rust','rust',base)]:
    if os.environ.get('S3_REUSE_PEERS'):
        old = json.loads((art/'build-binding.json').read_text())
        assert sha(art/('driver-'+arm)) == old['binaries'][arm]
        for name, digest in old['sources'].items(): assert sha(scratch/name) == digest
        continue
    run(['cargo','build','--offline','--locked','--release','--manifest-path',str(tree/'parity/peers/Cargo.toml'),
         '--no-default-features','--features',feature,'-j','1'], 'build-'+arm)
    shutil.copy2(target/'release/meshopt-peers', art/('driver-'+arm))
    print('built', arm, flush=True)
shutil.copy2(scratch/'art/inputs.json', art/'inputs.json')
if not (art/'inputs').exists():
    (art/'inputs').symlink_to(scratch/'art/inputs', target_is_directory=True)
binding = dict(sources={f'{t.name}/{p.relative_to(t)}': sha(p)
    for t in [base,candidate] for p in [*t.glob('src/**/*.rs'), t/'Cargo.toml',
        t/'parity/peers/Cargo.toml',t/'parity/peers/src/main.rs',t/'parity/peers/Cargo.lock']},
    binaries={a:sha(art/f'driver-{a}') for a in ['base','candidate','cpp','rust']})
(art/'build-binding.json').write_text(json.dumps(binding, indent=2)+'\n')

proofbase = scratch/'proof-base'
proofbase.mkdir(exist_ok=True)
if (proofbase/'src').exists(): shutil.rmtree(proofbase/'src')
shutil.copytree(base/'src', proofbase/'src')
shutil.copytree(base/'docs', proofbase/'docs', dirs_exist_ok=True)
(proofbase/'Cargo.toml').write_text((base/'Cargo.toml').read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
proof = scratch/'proof'
proof.mkdir(exist_ok=True)
source = (root/'parity/0215-contract-simplify/exact.rs').read_text()
source = source[:source.index('// Monotone multiplication')]
(proof/'exact.rs').write_text(source + (lane/'exact-main.rs').read_text())
for mode, features in [('all', ['std','simd','parallel','clusterlod','experimental','parity-internals']), ('none', [])]:
    manifest = f'''[package]
name = "prune-points-exact"
version = "0.0.0"
edition = "2021"
[dependencies]
baseline = {{ package = "meshoptimizer-baseline", path = {json.dumps(str(proofbase))}, default-features = false, features = {json.dumps(features)} }}
candidate = {{ package = "meshoptimizer-rs", path = {json.dumps(str(candidate))}, default-features = false, features = {json.dumps(features)} }}
[[bin]]
name = "exact"
path = "exact.rs"
[profile.release]
codegen-units = 1
lto = "fat"
'''
    (proof/'Cargo.toml').write_text(manifest)
    run(['cargo','generate-lockfile','--offline','--manifest-path',str(proof/'Cargo.toml')], 'lock-exact-'+mode)
    run(['cargo','build','--offline','--locked','--release','--manifest-path',str(proof/'Cargo.toml')], 'build-exact-'+mode)
    shutil.copy2(target/'release/exact', proof/('exact-'+mode))
    print('built exact', mode, flush=True)
(proof/'binding.json').write_text(json.dumps(dict(driver=sha(proof/'exact.rs'),
    binaries={m:sha(proof/f'exact-{m}') for m in ['all','none']}, source=binding['sources']), indent=2)+'\n')
