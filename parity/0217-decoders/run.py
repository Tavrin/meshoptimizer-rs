"""Sequential checks with retained exit codes; invoke inside CPU admission."""
import hashlib, io, json, os, re, pathlib, shutil, subprocess, sys, tarfile, time
root = pathlib.Path.cwd()
art = root / 'parity/0217-decoders/receipts'
target = pathlib.Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU'] == '0'
rows = []
def step(name, args, **extra):
    if '--native' in sys.argv and name == 'codec-wasm-build':
        sys.exit(int(any(row["exit"] for row in rows)))
    started = time.monotonic()
    with (art / (name + '.log')).open('w') as log:
        try:
            code = subprocess.run(args, env=dict(os.environ, **extra), stdout=log, stderr=subprocess.STDOUT).returncode
        except OSError as error:
            log.write(str(error)); code = 127
    logfile=art/(name+'.log')
    raw_hash=hashlib.sha256(logfile.read_bytes()).hexdigest()
    text=logfile.read_text()
    for value, label in [(str(root),'<repo>'),(str(target),'<target-dir>'),(os.environ['MESHOPT_REFERENCE'],'<reference>')]:
        text=text.replace(value,label)
    text=re.sub(r'/home/[^/]+/', '<home>/', text)
    logfile.write_text(text)
    rows.append(dict(step=name, exit=code, seconds=time.monotonic()-started,raw_log_sha256=raw_hash))
    (art/'exits.json').write_text(json.dumps(rows, indent=2)+'\n')
    print(name, code, flush=True)
    return code
if 'DECODE_INPUTS' in os.environ:
    step('group-profile', ['python3', 'parity/0217-decoders/profile.py'])
step('fmt-write', ['cargo', 'fmt'])
step('audit', ['python3', 'parity/simd/update_audit.py'])
step('fmt', ['cargo', 'fmt', '--check'])
step('test', ['cargo', 'test', '--locked', '--all-features'])
step('no-default', ['cargo', 'test', '--locked', '--no-default-features'])
step('clippy', ['cargo', 'clippy', '--locked', '--all-targets', '--all-features', '--', '-D', 'warnings'])
step('wasm-clippy', ['cargo', 'clippy', '--locked', '--target', 'wasm32-unknown-unknown', '--all-targets', '--all-features', '--', '-D', 'warnings'], RUSTFLAGS='-C target-feature=+simd128')
step('doctests', ['cargo', 'test', '--locked', '--doc', '--all-features'])
step('docs', ['cargo', 'doc', '--locked', '--all-features', '--no-deps'], RUSTDOCFLAGS='-D warnings')
step('msrv', ['cargo', '+1.88', 'check', '--locked', '--all-features', '--lib'])
baseline=target/'semver-baseline'; baseline.mkdir(exist_ok=True)
archive=subprocess.check_output(['git', 'archive', 'v0.2.16', 'src', 'Cargo.toml', 'Cargo.lock', 'README.md', 'docs', 'tests', 'examples', 'LICENSE'])
tarfile.open(fileobj=io.BytesIO(archive)).extractall(baseline, filter='data')
step('semver', ['cargo', 'semver-checks', '--baseline-root', str(baseline), '--all-features'])
step('boundary', ['bash', 'parity/check-unsafe-boundary.sh'])
step('reference', ['bash', 'parity/check-reference.sh'])
step('build-differential', ['python3', 'parity/0216-decode/build_differential.py'])
# Use the unchanged differential assertions, extending only ISA selection in a scratch copy.
p = target/'differential-source/main.rs'
if p.exists():
    p.write_text(p.read_text().replace('Some("sse41")=>Level::Sse41,', 'Some("avx2")=>Level::Avx2,Some("avx512")=>Level::Avx512,Some("sse41")=>Level::Sse41,'))
    step('build-extended-differential', ['cargo', 'build', '--offline', '--release', '--manifest-path', str(p.parent/'Cargo.toml'), '--target-dir', str(target/'proof')])
    ref=pathlib.Path(os.environ['MESHOPT_REFERENCE'])
    for name, flags in [('cpp-avx2',['-mavx2','-mpopcnt']),('cpp-avx512',['-mavx512vbmi','-mavx512vbmi2','-mavx512vl','-mpopcnt'])]:
        step('build-'+name, ['c++','-std=c++17','-O3','-DNDEBUG',*flags,'-I',str(ref/'src'),'parity/0216-decode/differential.cpp',str(ref/'src/vertexcodec.cpp'),'-o',str(target/(name+'-exact'))])
    script = target/'differential.py'
    script.write_text((root/'parity/0216-decode/differential.py').read_text().replace("['scalar','sse2','ssse3','sse41']", "['scalar','sse2','ssse3','sse41','avx2','avx512']"))
    script.write_text(script.read_text().replace("['cpp-scalar','cpp-default']", "['cpp-scalar','cpp-default','cpp-avx2','cpp-avx512']"))
    step('differential', ['python3', str(script)])
step('codec-build', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/codec/Cargo.toml', '--features', 'simd'])
step('codec-wasm-build', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/codec/Cargo.toml', '--features', 'simd', '--target', 'wasm32-unknown-unknown', '--lib'], RUSTFLAGS='-C target-feature=+simd128')
step('corpus-identity', ['python3', 'parity/0217-decoders/identity.py'])
step('wasm-after-build', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/simd/Cargo.toml', '--target', 'wasm32-unknown-unknown', '--lib'], RUSTFLAGS='-C target-feature=+simd128')
shutil.copy2(target/'wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm', art/'after.wasm')
step('wasm-measure', ['python3', 'parity/0217-decoders/measure.py'])

assert all(row["exit"] == 0 for row in rows), rows
