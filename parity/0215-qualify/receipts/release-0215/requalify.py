"""Release-source requalification; run under CPU admission, before public edits.

RELEASE_SCRATCH is a fresh retained artifact directory; RELEASE_INPUTS holds the
frozen eight peer fixtures; MESHOPT_REFERENCE is the pinned C++ 1.3 checkout.
CARGO_TARGET_DIR is dedicated and removed only after every release check finishes.
The inherited case generators and comparisons are unchanged. Adapters below only
select manifests, output directories and both explicit dependency feature modes.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

root = Path.cwd()
scratch = Path(os.environ['RELEASE_SCRATCH'])
inputs = Path(os.environ['RELEASE_INPUTS'])
target = Path(os.environ['CARGO_TARGET_DIR'])
receipt = root / 'parity/0215-qualify/receipts/release-0215'
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU'] == '0'
os.environ.update(PARITY_RUNNER_ACTIVE='release-0215', PARITY_RUNNER_GPU='0',
                  MESHOPT_ADMISSION_ID='release-0215', PYTHONDONTWRITEBYTECODE='1')
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == 'a8e87babdbfad13aaeb69b9b4a15da8db994028e'
scratch.mkdir(parents=True, exist_ok=True)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def run(name, args, **env):
    start = time.monotonic()
    with (scratch / (name + '.log')).open('w') as log:
        p = subprocess.run([str(x) for x in args], stdout=log, stderr=subprocess.STDOUT,
                           env=dict(os.environ, **{k: str(v) for k, v in env.items()}))
    line = f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}s\n'
    with (receipt / 'requalification-exits.tsv').open('a') as out:
        out.write(line)
    print(line, end='', flush=True)
    if p.returncode:
        print((scratch / (name + '.log')).read_text()[-6000:], flush=True)
        raise SystemExit(p.returncode)

binding = dict(commit='a8e87babdbfad13aaeb69b9b4a15da8db994028e',
               source={str(p.relative_to(root)): sha(p) for p in sorted(root.glob('src/**/*.rs'))},
               inputs={}, drivers={}, binaries={})
for row in json.loads((root / 'parity/0215-toward-cpp/inputs.json').read_text()):
    p = inputs / (row['case'] + '.bin')
    assert sha(p) == row['sha256'], p
    binding['inputs'][p.name] = row['sha256']
run('reference', ['bash', 'parity/check-reference.sh'])
# Read-only archive of the release baseline; no checkout or Git state mutation.
base = scratch / 'base'
base.mkdir(exist_ok=True)
archive = subprocess.check_output(['git', 'archive', 'v0.2.14', 'src', 'docs', 'Cargo.toml', 'Cargo.lock', 'README.md'])
subprocess.run(['tar', '-x', '-C', str(base)], input=archive, check=True)
p = base / 'Cargo.toml'
p.write_text(p.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
all_features = ['std', 'simd', 'parallel', 'clusterlod', 'experimental', 'parity-internals']

def manifest(folder, package, binary, source, features, extra='', library=None, baseline=False):
    folder.mkdir(parents=True, exist_ok=True)
    dependency = f'meshoptimizer-rs = {{ path = "{root}", default-features = false, features = {json.dumps(features)} }}\n'
    if baseline:
        dependency = (f'candidate = {{ package = "meshoptimizer-rs", path = "{root}", default-features = false, features = {json.dumps(features)} }}\n'
                      f'baseline = {{ package = "meshoptimizer-baseline", path = "{base}", default-features = false, features = {json.dumps(features)} }}\n')
    text = f'[package]\nname = "{package}"\nversion = "0.0.0"\nedition = "2021"\n[dependencies]\n{dependency}{extra}'
    if library:
        text += f'[lib]\npath = "{library}"\n'
    text += f'[[bin]]\nname = "{binary}"\npath = "{source}"\n[profile.release]\nlto = "fat"\ncodegen-units = 1\ndebug-assertions = true\n'
    path = folder / 'Cargo.toml'
    path.write_text(text)
    run(folder.name + '-lock', ['cargo', 'generate-lockfile', '--offline', '--manifest-path', path])
    return path

protocol = scratch / 'protocol'
protocol.mkdir(exist_ok=True)
(protocol / 'candidate').symlink_to(root, target_is_directory=True)
run('cpp-protocol-both', ['bash', 'parity/0216-qualify/run-protocol-modes.sh'],
    MESHOPT_SCRATCH=protocol, MESHOPT_QUALIFY_TARGET=target)
for mode in ['all', 'none']:
    shutil.copy2(protocol / f'proof/0216b-protocol/exit-{mode}.json', receipt / f'protocol-{mode}.json')

for mode, features in [('all', all_features), ('none', [])]:
    folder = scratch / f'overdraw-{mode}'
    path = manifest(folder, 'meshoptimizer-parity', 'meshopt-driver', root/'parity/src/main.rs', features,
                    'libm = { version = "=0.2.16", default-features = false }\n', root/'parity/src/lib.rs')
    script = (root/'parity/0212-overdraw/differential.py').read_text()
    script = script.replace('ROOT = Path(__file__).resolve().parents[2]', 'ROOT = Path.cwd()')
    script = script.replace("str(ROOT / 'parity/Cargo.toml'), '--features', 'meshoptimizer-rs/std'", repr(str(path)))
    derived = folder/'differential.py'
    derived.write_text(script)
    run(f'cpp-overdraw-{mode}', ['python3', derived], MESHOPT_OVERDRAW_SCRATCH=folder)
    rows = json.loads((folder/'cpp-differential.json').read_text())
    (receipt/f'overdraw-{mode}.json').write_text(json.dumps(dict(exit=0, cases=len(rows), cases_sha256=sha(folder/'cpp-differential.json'))) + '\n')
    folder = scratch/f'cache-cpp-{mode}'
    path = manifest(folder, 'release-cache-cpp', 'cpp-differential', root/'parity/0215-toward-cpp/cpp-differential.rs', features, 'meshopt = "=0.7.0"\n')
    # This inherited driver imports the underscore crate name.
    path.write_text(path.read_text().replace('meshoptimizer-rs = {', 'meshoptimizer_rs = { package = "meshoptimizer-rs",'))
    run(f'cache-cpp-build-{mode}', ['cargo', 'build', '--offline', '--release', '--manifest-path', path])
    shutil.copy2(target/'release/cpp-differential', folder/'driver')
    run(f'cpp-cache-decode-{mode}', [folder/'driver', inputs])
    folder = scratch/f'cache-exact-{mode}'
    path = manifest(folder, 'release-cache-exact', 'cache-exact', root/'parity/0215-toward-cpp/exact.rs', features, baseline=True)
    run(f'cache-exact-build-{mode}', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', path])
    shutil.copy2(target/'release/cache-exact', folder/'driver')
    run(f'cache-exact-{mode}', [folder/'driver'])

# The inherited meshlet/LOD script compares both clients to the same C++ outputs.
art = scratch/'meshlets'
art.mkdir(exist_ok=True)
paths = {}
for mode, features in [('all', all_features), ('none', ['clusterlod'])]:
    paths[mode] = manifest(scratch/f'meshlets-{mode}', 'meshopt-p03-parity', 'meshopt-p03-driver',
                           root/'parity/p03/src/main.rs', features, library=root/'parity/p03/src/lib.rs')
script = (root/'parity/0215-contract-meshlets/differential.py').read_text()
script = script.replace("root = Path(__file__).resolve().parents[2]", 'root = Path.cwd()')
script = script.replace("[('nostd',[]),('std',['--features','meshoptimizer-rs/std'])]", "[('none',[]),('all',[])]")
script = script.replace("str(root/'parity/p03/Cargo.toml')", "str(" + repr({k:str(v) for k,v in paths.items()}) + "[name])")
derived = art/'differential.py'
derived.write_text(script)
run('cpp-meshlets-clod-both', ['python3', derived], PEERS_ART=art)
shutil.copy2(art/'cpp-differential/result.json', receipt/'meshlets-clod.json')
for name in ['cpp-cache-decode-all', 'cpp-cache-decode-none', 'cache-exact-all', 'cache-exact-none']:
    shutil.copy2(scratch/(name+'.log'), receipt/(name+'.log'))
for p in sorted(scratch.rglob('*')):
    if p.is_file() and (p.name in ['driver', 'differential.py', 'Cargo.toml'] or p.name.startswith(('p03-', 'rust-'))):
        binding['binaries' if os.access(p, os.X_OK) else 'drivers'][str(p.relative_to(scratch))] = sha(p)
for name in ['parity/0216-qualify/run-protocol-modes.sh', 'parity/0216-qualify/protocol-modes.py', 'parity/0212-overdraw/differential.py', 'parity/0215-toward-cpp/cpp-differential.rs', 'parity/0215-toward-cpp/exact.rs', 'parity/0215-contract-meshlets/differential.py']:
    binding['drivers'][name] = sha(root/name)
binding['exit'] = 0
(receipt/'requalification.json').write_text(json.dumps(binding, indent=2, sort_keys=True)+'\n')
print('PASS: four C++ differentials and cache exact comparison, both modes', flush=True)
