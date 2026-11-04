"""0.2.18 train qualification (adapted from parity/0217-qualify/run.py; base is v0.2.17). Each stage runs inside CPU admission (declaration 4).

Stages used for 0.2.18: prepare, core (exact drivers + gates, identity, C++ differentials, peer builds), measure <group>,
summarize, then new_apis_determinism, semver and lane_costs (defined at the end of this file). The 0.2.17 wasm timing,
decoder identity and old disclosure stages are retained below but were not run: no pre-existing function changed.

Environment: CARGO_TARGET_DIR (fresh), MESHOPT_REFERENCE (pinned C++ 1.3 checkout),
QUALIFY_INPUTS (frozen 0.2.15 campaign inputs), PEERS_PERF (perf). src/ is frozen;
only this directory receives files. Method follows parity/0216-qualify/combined-ff38162.
"""
import hashlib, io, json, os, resource, shutil, statistics, subprocess, sys, tarfile, time
from pathlib import Path

R = Path(__file__).resolve().parents[2]
A = Path(__file__).resolve().parent
T = Path(os.environ['CARGO_TARGET_DIR'])
REF = Path(os.environ['MESHOPT_REFERENCE'])
LOGS = Path(os.environ.get('QUALIFY_LOGS', 'qualify-logs'))
ALL = ['std', 'simd', 'parallel', 'clusterlod', 'experimental', 'parity-internals']
OPS = ('simplify simplify-attributes simplify-sloppy simplify-update simplify-prune simplify-points cache overdraw fetch '
       'meshlets meshlets-flex meshlets-spatial cluster-lod vertex-encode vertex-decode index-encode index-decode '
       'remap-vertex remap-index cluster-bounds meshlet-bounds').split()
BASE_TAG = 'v0.2.17'
ARMS = ['base-scalar', 'train-scalar', 'base-default', 'train-default', 'cpp-scalar', 'cpp-default']
os.environ.update(CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='2', LC_ALL='C', PYTHONDONTWRITEBYTECODE='1')
os.environ.pop('CARGO_ENCODED_RUSTFLAGS', None)
os.environ.update(RUSTFLAGS='', RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='')


def sha(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def write(p, x):
    Path(p).parent.mkdir(parents=True, exist_ok=True)
    Path(p).write_text(json.dumps(x, indent=2, sort_keys=True) + '\n')


def clean(s):
    for p, label in [(str(T), '<target>'), (str(R), '<worktree>'), (str(REF), '<reference>'), (str(Path.home()), '<home>')]:
        s = s.replace(p, label)
    return s


def floor():
    while shutil.disk_usage(T).free < float(os.environ.get('QUALIFY_DISK_FLOOR_GB', '100')) * 1024**3:
        print('paused: disk below 100 GiB floor', flush=True)
        time.sleep(30)


def run(name, cmd, env=None, estimate=600, check=True):
    assert estimate <= 7200
    floor()
    LOGS.mkdir(parents=True, exist_ok=True)
    (A / 'logs').mkdir(exist_ok=True)
    n = 1
    while (A / 'logs' / f'{name}-{n}.log').exists():
        n += 1
    log = A / 'logs' / f'{name}-{n}.log'
    raw = LOGS / 'raw.log'
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.monotonic()
    with raw.open('w') as f:
        p = subprocess.run(list(map(str, cmd)), stdout=f, stderr=subprocess.STDOUT, env=dict(os.environ, **(env or {})))
    text = clean(raw.read_text(errors='replace'))
    raw.unlink()
    # Keep the head and tail of very long logs; full text goes to the scratch log dir.
    (LOGS / f'{name}-{n}.log').write_text(text)
    log.write_text(text if len(text) < 60000 else text[:20000] + '\n...[truncated; full log in scratch]...\n' + text[-30000:])
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    row = dict(name=name, attempt=n, exit=p.returncode, wall_seconds=time.monotonic() - start,
               cpu_seconds=after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime, log=str(log.relative_to(A)))
    with (A / 'exits.jsonl').open('a') as f:
        f.write(json.dumps(row) + '\n')
    print(name, p.returncode, round(row['cpu_seconds'], 1), flush=True)
    if p.returncode and check:
        print(text[-4000:], flush=True)
        raise SystemExit(p.returncode)
    return log


def prepare():
    T.mkdir(parents=True, exist_ok=True)
    assert not (T / 'base').exists(), 'prepare already ran'
    archive = subprocess.check_output(['git', 'archive', BASE_TAG, 'src', 'docs', 'examples', 'Cargo.toml', 'Cargo.lock', 'README.md'], cwd=R)
    b = T / 'base'
    b.mkdir()
    tarfile.open(fileobj=io.BytesIO(archive)).extractall(b, filter='data')
    p = b / 'Cargo.toml'
    p.write_text(p.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
    inputs = json.loads((R / 'parity/0216-qualify/inputs.json').read_text())
    source = Path(os.environ['QUALIFY_INPUTS'])
    (T / 'inputs').mkdir()
    for x in inputs:
        src = source / (x['case'] + '.bin')
        assert sha(src) == x['sha256'], x
        shutil.copy2(src, T / 'inputs' / src.name)
    write(A / 'inputs.json', inputs)
    (T / 'bin').mkdir()
    write(A / 'start.json', dict(
        head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=R, text=True).strip(),
        base=subprocess.check_output(['git', 'rev-parse', BASE_TAG + '^{}'], cwd=R, text=True).strip(),
        sources={str(q.relative_to(R)): sha(q) for q in (R / 'src').rglob('*') if q.is_file()},
        cargo_toml=sha(R / 'Cargo.toml'), cargo_lock=sha(R / 'Cargo.lock'),
        rustc=subprocess.check_output(['rustc', '-Vv'], text=True),
        cpu_affinity=sorted(os.sched_getaffinity(0))))
    run('reference', ['bash', 'parity/check-reference.sh'], estimate=1)
    run('export-lod', ['python3', 'parity/0215-lod-chain/corpus.py', T / 'lod-corpus'], estimate=10)


def manifest(d, bins, features, extras='', feature_section='', base_features=None):
    d.mkdir(parents=True, exist_ok=True)
    deps = ''
    for alias, package, path in [('baseline', 'meshoptimizer-baseline', T / 'base'), ('candidate', 'meshoptimizer-rs', R)]:
        deps += f'{alias}={{package="{package}",path={json.dumps(str(path))},default-features=false,features={json.dumps(base_features or features)}}}\n'
    (d / 'Cargo.toml').write_text(
        '[package]\nname="combined-proof"\nversion="0.0.0"\nedition="2021"\n' + feature_section + '[dependencies]\n' + deps + extras +
        ''.join(f'[[bin]]\nname="{n}"\npath={json.dumps(str(p))}\n' for n, p in bins.items()) +
        '[profile.release]\ncodegen-units=1\n')


# name -> driver source (all compare crate `baseline` = v0.2.16 against `candidate` = train)
DRIVERS = {
    'remap': 'parity/0216-remap/exact.rs', 'spatial': 'parity/0216-spatial/exact.rs', 'repair': 'parity/0216-repair/exact.rs',
    'simplify': 'parity/0216-simplify/exact.rs', 'decode': 'parity/0216-decode/exact.rs', 'bounds': 'parity/0216-bounds/exact.rs',
    'vdecode': 'parity/0216-vdecode/exact.rs', 'stream': 'parity/0216-stream/exact.rs',
    'remap_cpp': 'parity/0216-remap/cpp-differential.rs', 'repair_corpus': 'parity/0216-repair/corpus.rs',
    'geometry': 'parity/0215-toward-cpp/geometry-exact.rs',
    'lodq': 'parity/0217-lodq/exact.rs', 'hsr': 'parity/0217-hsr/exact.rs', 'cache_table': 'parity/0215-cache-restore/table-exact.rs',
    'basics': 'parity/0215-basics/exact.rs', 'merge': 'parity/0215-qualify/merge-exact.rs',
    'overdraw': 'parity/0216-qualify/overdraw-exact.rs', 'lodq_diff': 'parity/0217-lodq/differential.rs',
}


def build_exact(mode, names=None):
    features = ALL if mode == 'all' else ['clusterlod']
    names = names or list(DRIVERS)
    bins = {n: R / DRIVERS[n] for n in names}
    d = T / ('exact-' + mode)
    section = '[features]\nall=[]\n' if mode == 'all' else ''
    manifest(d, bins, features, 'meshopt="=0.7.0"\n', section)
    run('exact-lock-' + mode, ['cargo', 'generate-lockfile', '--offline', '--manifest-path', d / 'Cargo.toml'], estimate=5)
    run('exact-build-' + mode, ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', d / 'Cargo.toml',
                                *(['--features', 'all'] if mode == 'all' else [])], estimate=2400, check=False)
    for n in bins:
        if (T / 'release' / n).exists():
            shutil.copy2(T / 'release' / n, T / 'bin' / f'{n}-{mode}')
    write(A / ('exact-binding-' + mode + '.json'), dict(
        binaries={n: sha(T / 'bin' / f'{n}-{mode}') for n in bins if (T / 'bin' / f'{n}-{mode}').exists()},
        manifest=clean((d / 'Cargo.toml').read_text())))
    d = T / ('par-' + mode)
    manifest(d, {'par': R / 'parity/0216-par-clod/exact.rs'}, features if mode == 'all' else ['parallel', 'clusterlod', 'parity-internals'], 'rayon="=1.11.0"\n')
    run('par-build-' + mode, ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=600)
    shutil.copy2(T / 'release/par', T / 'bin' / f'par-{mode}')


def exact(name, mode):
    args = []
    estimate = 1800
    env = {}
    if name == 'stream':
        if not (T / 'stream-cpp').exists():
            run('stream-cpp-build', ['c++', '-O2', '-std=c++11', '-DMESHOPTIMIZER_NO_SIMD', '-I' + str(REF / 'src'), R / 'parity/0216-stream/reference.cpp',
                                     REF / 'src/indexcodec.cpp', '-o', T / 'stream-cpp'], estimate=60)
        args = [R / 'parity/0216-stream/corpus.tsv', T / 'stream-cpp', A / f'stream-cases-{mode}.tsv']
    if name == 'repair_corpus':
        args = [T / 'lod-corpus']
    if name == 'merge':
        args = ['quick']
    if name == 'overdraw':
        small = T / 'small-inputs'
        small.mkdir(exist_ok=True)
        for case in ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse']:
            if not (small / (case + '.bin')).exists():
                shutil.copy2(T / 'inputs' / (case + '.bin'), small / (case + '.bin'))
        (T / 'overdraw-scratch').mkdir(exist_ok=True)
        args = [small]
        env = {'MESHOPT_OVERDRAW_SCRATCH': str(T / 'overdraw-scratch')}
    return run('exact-' + name + '-' + mode, [T / 'bin' / f'{name}-{mode}', *args], env=env, estimate=estimate, check=False)


def proof_exact(mode):
    build_exact(mode)
    for name in DRIVERS:
        if name != 'lodq_diff' and (T / 'bin' / f'{name}-{mode}').exists():
            exact(name, mode)
    exact_par(mode)


def proof_group():
    for mode in ['all', 'none']:
        proof_exact(mode)
        gates(mode)


def identity_group():
    """Parallel-vs-serial (1/4/16 workers) and streaming-vs-one-shot identity on the eight frozen inputs."""
    d = T / 'identity'
    manifest(d, {'parallel_identity': A / 'parallel_identity.rs', 'stream_identity': A / 'stream_identity.rs'}, ALL, 'rayon="=1.11.0"\n')
    run('identity-build', ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=900)
    for n in ['parallel_identity', 'stream_identity']:
        shutil.copy2(T / 'release' / n, T / 'bin' / n)
        run(n + '-all', [T / 'bin' / n, T / 'inputs'], estimate=3000, check=False)
    d = T / 'identity-none'
    manifest(d, {'stream_identity': A / 'stream_identity.rs'}, ['clusterlod'])
    run('identity-none-build', ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=600, check=False)
    run('stream_identity-none', [T / 'release/stream_identity', T / 'inputs'], estimate=3000, check=False)


def exact_par(mode):
    run('par-lod-' + mode, [T / 'bin' / f'par-{mode}', T / 'lod-corpus'], estimate=1200, check=False)
    p = T / 'par-spec.rs'
    s = (R / 'parity/0216-par-clod/exact.rs').read_text()
    assert 'for workers in [1, 2, 4, 8, 32]' in s
    p.write_text(s.replace('for workers in [1, 2, 4, 8, 32]', 'for workers in [1, 4, 16]'))
    d = T / ('par-spec-' + mode)
    manifest(d, {'par-spec': p}, ALL if mode == 'all' else ['parallel', 'clusterlod', 'parity-internals'], 'rayon="=1.11.0"\n')
    run('par-spec-build-' + mode, ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=600)
    run('par-spec-lod-' + mode, [T / 'release/par-spec', T / 'lod-corpus'], estimate=1200, check=False)
    run('par-spec-inputs-' + mode, [T / 'release/par-spec', T / 'inputs'], estimate=3600, check=False)


def gates(mode):
    flag = '--all-features' if mode == 'all' else '--no-default-features'
    run('test-' + mode, ['cargo', 'test', '--offline', '--locked', flag], estimate=1800, check=False)
    run('clippy-' + mode, ['cargo', 'clippy', '--offline', '--locked', flag, '--all-targets', '--', '-D', 'warnings'], estimate=900, check=False)
    for group, folder in [('lod', T / 'lod-corpus'), ('inputs', T / 'inputs')]:
        run('lod-identity-' + group + '-' + mode, ['cargo', 'test', '--offline', '--locked', '--release', flag, '--test', 'lod_chain', 'corpus_identity',
                                                  '--', '--ignored', '--exact', '--nocapture'], env={'MESHOPT_LOD_CORPUS': str(folder)}, estimate=1800, check=False)
    if mode == 'all':
        run('fmt', ['cargo', 'fmt', '--all', '--', '--check'], estimate=30, check=False)
        run('boundary', ['bash', 'parity/check-unsafe-boundary.sh'], estimate=60, check=False)


def physical_cpus():
    seen = set()
    cpus = []
    for c in sorted(os.sched_getaffinity(0)):
        t = Path(f'/sys/devices/system/cpu/cpu{c}/topology')
        key = tuple((t / p).read_text() for p in ['physical_package_id', 'core_id'])
        if key not in seen:
            seen.add(key)
            cpus.append(c)
        if len(cpus) == 4:
            return cpus
    raise AssertionError('four cores required')


def bound_inputs():
    """Freeze the v0.2.16 meshlet topology used by the standalone bounds ops; digests must equal the 0.2.16 campaign's."""
    d = T / 'bound-prepare'
    d.mkdir(exist_ok=True)
    (d / 'Cargo.toml').write_text('[package]\nname="bounds-prepare"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={package="meshoptimizer-baseline",path='
                                  + json.dumps(str(T / 'base')) + '}\n[[bin]]\nname="bounds-prepare"\npath=' + json.dumps(str(R / 'parity/0216-bounds/prepare.rs')) + '\n')
    run('bounds-prepare-build', ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=300)
    out = T / 'bound-inputs'
    out.mkdir(exist_ok=True)
    expected = json.loads((R / 'parity/0216-bounds/bound-inputs.json').read_text())['inputs']
    got = {}
    for name in expected:
        run('bounds-prepare-' + name, [T / 'release/bounds-prepare', T / 'inputs' / name, out / name], estimate=120)
        got[name] = sha(out / name)
    write(A / 'bound-inputs.json', dict(expected=expected, got=got, equal=got == expected))


def build_peers():
    canonical = T / 'canonical'
    canonical.mkdir(exist_ok=True)
    d = canonical / 'parity/peers'
    (d / 'src').mkdir(parents=True, exist_ok=True)
    shutil.copy2(A / 'peer-driver.rs.txt', d / 'src/main.rs')
    binding = {}
    for arm in ARMS:
        base = arm.startswith('base')
        source = T / 'base' if base else R
        if (canonical / 'src').exists():
            shutil.rmtree(canonical / 'src')
        shutil.copytree(source / 'src', canonical / 'src')
        shutil.copytree(source / 'docs', canonical / 'docs', dirs_exist_ok=True)
        if (canonical / 'examples').exists():
            shutil.rmtree(canonical / 'examples')
        if (source / 'examples').exists():
            shutil.copytree(source / 'examples', canonical / 'examples')
        for name in ['Cargo.toml', 'Cargo.lock', 'README.md']:
            shutil.copy2(source / name, canonical / name)
        cm = canonical / 'Cargo.toml'
        cm.write_text(cm.read_text().replace('name = "meshoptimizer-baseline"', 'name = "meshoptimizer-rs"', 1))
        for path in (canonical / 'src').rglob('*'):
            if path.is_file():
                path.touch()
        features = ['std', 'clusterlod'] + (['simd'] if arm.endswith('default') else [])
        (d / 'Cargo.toml').write_text(
            '[package]\nname="combined-peer"\nversion="0.0.0"\nedition="2021"\n[features]\nours=["dep:meshoptimizer-rs"]\ncpp=["dep:meshopt"]\nrust=["dep:optimesh"]\n'
            f'[dependencies]\nmeshoptimizer-rs={{package="meshoptimizer-rs",path={json.dumps(str(canonical))},optional=true,default-features=false,features={json.dumps(features)}}}\n'
            'meshopt={version="=0.7.0",optional=true}\noptimesh={version="=1.1.0",optional=true,default-features=false}\n[profile.release]\nlto="thin"\ncodegen-units=1\n')
        feature = 'cpp' if arm.startswith('cpp') else 'ours'
        run('build-' + arm, ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml', '--features', feature],
            env={'CXXFLAGS': ('-DMESHOPTIMIZER_NO_SIMD ' if arm == 'cpp-scalar' else '') + '-fno-fast-math -ffp-contract=off'}, estimate=900)
        shutil.copy2(T / 'release/combined-peer', T / 'bin' / arm)
        binding[arm] = dict(binary=sha(T / 'bin' / arm), manifest=clean((d / 'Cargo.toml').read_text()),
                            sources={str(p.relative_to(canonical)): sha(p) for p in (canonical / 'src').rglob('*') if p.is_file()})
    write(A / 'peer-bindings.json', binding)
    registry = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))) / 'registry/src'
    cpp = next(registry.glob('*/meshopt-0.7.0'))
    files = {}
    for p in (cpp / 'vendor/src').iterdir():
        if p.suffix in ['.cpp', '.h']:
            assert p.read_bytes() == (REF / 'src' / p.name).read_bytes(), p.name
            files[p.name] = sha(p)
    write(A / 'environment.json', dict(
        cpp_source_identity=True, cpp_files=files, reference_revision=subprocess.check_output(['git', '-C', str(REF), 'rev-parse', 'HEAD'], text=True).strip(),
        rustc=subprocess.check_output(['rustc', '-Vv'], text=True), cxx=subprocess.check_output(['c++', '--version'], text=True),
        perf=subprocess.check_output([os.environ['PEERS_PERF'], '--version'], text=True).strip(), cores=physical_cpus(),
        cpu_model=next(l.split(':', 1)[1].strip() for l in Path('/proc/cpuinfo').read_text().splitlines() if l.startswith('model name')),
        kernel=os.uname().release))


def build_group():
    bound_inputs()
    build_peers()


def sample(case, op, arm, rep):
    records = A / 'counts.jsonl'
    rows = [json.loads(l) for l in records.read_text().splitlines()] if records.exists() else []
    if any((x['case'], x['op'], x['arm'], x['rep']) == (case, op, arm, rep) for x in rows):
        return
    folder = LOGS / 'samples'
    folder.mkdir(exist_ok=True)
    prefix = folder / f'instructions-{case}-{op}-{arm}-{rep}'
    n = 1000 if case == 'tiny-smooth' else 3 if case.startswith('medium') else 1
    if op == 'cluster-lod':
        n = 10 if case == 'tiny-smooth' else 1
    if op.endswith('bounds'):
        n = 100 if case == 'tiny-smooth' else 1
    output = T / (f'sample-output-{case}' + ('-cost' if arm == 'cost' else ''))
    cpus = physical_cpus()
    cmd = ['taskset', '-c', str(cpus[3] if arm == 'cost' else cpus[0]), str(T / 'bin' / arm), op, str(T / 'inputs' / (case + '.bin')), str(output)]
    cr, cw = os.pipe()
    ar, aw = os.pipe()
    fds = [cr, cw, ar, aw]
    cmd = [os.environ['PEERS_PERF'], 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix) + '.perf', '-D', '-1', '--control', f'fd:{cr},{aw}', '--', *cmd]
    env = dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar), BOUNDS_SCRATCH=str(T))
    floor()
    start = resource.getrusage(resource.RUSAGE_CHILDREN)
    t = time.monotonic()
    try:
        p = subprocess.run(cmd, input=f'{n}\nstop\n', capture_output=True, text=True, pass_fds=fds, env=env)
    finally:
        for fd in fds:
            os.close(fd)
    Path(str(prefix) + '.log').write_text(clean(p.stdout + p.stderr))
    assert p.returncode == 0, (case, op, arm, p.stderr[-3000:])
    lines = p.stdout.splitlines()
    assert len(lines) == 2 and lines[0] == 'READY' and lines[1].startswith('COUNTED '), lines
    end = resource.getrusage(resource.RUSAGE_CHILDREN)
    row = dict(case=case, op=op, arm=arm, rep=rep, repetitions=n, output_sha256=sha(output),
               cpu_seconds=end.ru_utime + end.ru_stime - start.ru_utime - start.ru_stime, wall_seconds=time.monotonic() - t)
    same = [x for x in rows if x['case'] == case and x['op'] == op and x['arm'] == arm]
    assert not same or {x['output_sha256'] for x in same} == {row['output_sha256']}, ('unstable', case, op, arm)
    fields = next(l.split(';') for l in Path(str(prefix) + '.perf').read_text().splitlines() if ';instructions:u;' in l)
    assert float(fields[4]) == 100. and int(fields[0]) > 0, fields
    row.update(instructions=int(fields[0]), instructions_per_call=int(fields[0]) / n)
    with records.open('a') as f:
        f.write(json.dumps(row) + '\n')
        f.flush()
        os.fsync(f.fileno())
    output.unlink()
    if Path(str(output) + '.raw').exists():
        Path(str(output) + '.raw').unlink()
    print(case, op, arm, rep, round(row['instructions_per_call'], 1), flush=True)


def measure(case_group):
    inputs = json.loads((A / 'inputs.json').read_text())
    names = [x['case'] for x in inputs if (not x['case'].startswith('real-') if case_group == 'corpus' else x['case'] == case_group)]
    assert names, case_group
    for op in OPS:
        for case in names:
            for rep in range(3):
                for arm in ARMS if rep != 1 else ARMS[::-1]:
                    sample(case, op, arm, rep)


def old0216():
    """Reuse the retained 0.2.16 differential stages, redirecting every receipt into this directory."""
    import importlib.util
    spec = importlib.util.spec_from_file_location('old0216', R / 'parity/0216-qualify/combined-ff38162/run.py')
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    m.A = A
    m.R = R
    m.floor = floor  # the retained module hard-codes a 100 GiB pause; use this file's configurable floor
    return m


def differential_group():
    o = old0216()
    o.protocol()
    o.p03_build()
    o.p03_diff()
    for lane in ['decode', 'vdecode']:
        for mode in ['all', 'none']:
            o.decode_differential(lane, mode)
            if lane == 'decode' and mode == 'all':
                extended_decode()
    lodq_differential()


def extended_decode():
    """Decoder lane's AVX2/AVX-512 extension of the unchanged decode differential (host has VBMI/VBMI2)."""
    ref = REF
    art = A / 'decode-all'
    (art / 'extended').mkdir(parents=True, exist_ok=True)
    p = T / 'differential-source/main.rs'
    p.write_text(p.read_text().replace('Some("sse41")=>Level::Sse41,', 'Some("avx2")=>Level::Avx2,Some("avx512")=>Level::Avx512,Some("sse41")=>Level::Sse41,'))
    run('build-extended-differential', ['cargo', 'build', '--offline', '--release', '--manifest-path', p.parent / 'Cargo.toml', '--target-dir', T / 'proof'], estimate=300)
    for name, flags in [('cpp-avx2', ['-mavx2', '-mpopcnt']), ('cpp-avx512', ['-mavx512vbmi', '-mavx512vbmi2', '-mavx512vl', '-mpopcnt'])]:
        run('build-' + name, ['c++', '-std=c++17', '-O3', '-DNDEBUG', *flags, '-I', REF / 'src', R / 'parity/0216-decode/differential.cpp', REF / 'src/vertexcodec.cpp', '-o', T / (name + '-exact')], estimate=120)
    script = T / 'differential.py'
    s = (R / 'parity/0216-decode/differential.py').read_text()
    s = s.replace("['scalar','sse2','ssse3','sse41']", "['scalar','sse2','ssse3','sse41','avx2','avx512']").replace("['cpp-scalar','cpp-default']", "['cpp-scalar','cpp-default','cpp-avx2','cpp-avx512']")
    script.write_text(s)
    run('decode-extended-differential', ['python3', script], env={'DECODE_ART': str(art / 'extended')}, estimate=1800)


def lodq_differential():
    d = T / 'lodq'
    d.mkdir(exist_ok=True)
    run('lodq-oracle-build', ['c++', '-O2', '-std=c++17', R / 'parity/0217-lodq/oracle.cpp', '-o', d / 'oracle'], estimate=120)
    for mode in ['all', 'none']:
        run('lodq-cpp-' + mode, [T / 'bin' / f'lodq_diff-{mode}', d / 'oracle'], estimate=1800, check=False)


def decoders_identity():
    """Decoder lane identity driver (corpus x every ISA ceiling + wasm + C++), receipts redirected here."""
    art = T / 'art'
    art.mkdir(exist_ok=True)
    env = {'MESHOPT_ARTIFACTS': str(art)}
    run('codec-build', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/codec/Cargo.toml', '--features', 'simd'], estimate=600)
    run('codec-wasm-build', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/codec/Cargo.toml', '--features', 'simd', '--target', 'wasm32-unknown-unknown', '--lib'],
        env={'RUSTFLAGS': '-C target-feature=+simd128'}, estimate=600)
    s = (R / 'parity/0217-decoders/identity.py').read_text()
    s = s.replace("art=root/'parity/0217-decoders/receipts'", "art=pathlib.Path(os.environ['QUALIFY_ART'])")
    assert 'QUALIFY_ART' in s
    script = T / 'identity.py'
    script.write_text(s)
    (A / 'decoders').mkdir(exist_ok=True)
    env['QUALIFY_ART'] = str(A / 'decoders')
    run('decoders-identity', ['python3', script], env=env, estimate=3000, check=False)


def perf_instructions(cmd, env=None, cpu=None):
    """instructions:u of one whole process under perf stat (no control fd)."""
    out = LOGS / 'perf.tmp'
    full = ['perf', 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(out), '--', *map(str, cmd)]
    if cpu is not None:
        full = ['taskset', '-c', str(cpu), *full]
    p = subprocess.run(full, capture_output=True, text=True, env=dict(os.environ, **(env or {})))
    assert p.returncode == 0, (cmd, p.stderr[-2000:])
    fields = next(l.split(';') for l in out.read_text().splitlines() if ';instructions:u;' in l)
    out.unlink()
    assert float(fields[4]) == 100.
    return int(fields[0]), p.stdout


def build_disclosure():
    d = T / 'cost'
    d.mkdir(exist_ok=True)
    (d / 'Cargo.toml').write_text(
        '[package]\nname="qualify-cost"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={path=' + json.dumps(str(R)) + '}\n'
        '[[bin]]\nname="cost"\npath=' + json.dumps(str(A / 'cost.rs')) + '\n[[bin]]\nname="hsr-bench"\npath=' + json.dumps(str(R / 'parity/0217-hsr/eval/bench.rs')) +
        '\n[profile.release]\nlto="thin"\ncodegen-units=1\n')
    run('build-cost', ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=600)
    shutil.copy2(T / 'release/cost', T / 'bin/cost')
    shutil.copy2(T / 'release/hsr-bench', T / 'bin/hsr-bench')
    run('build-examples', ['cargo', 'build', '--offline', '--locked', '--release', '--example', 'buffer_stream_bench', '--example', 'character_eval'], estimate=900)
    shutil.copy2(T / 'release/examples/buffer_stream_bench', T / 'bin/buffer_stream_bench')
    shutil.copy2(T / 'release/examples/character_eval', T / 'bin/character_eval')
    write(A / 'cost-binding.json', dict(binaries={n: sha(T / 'bin' / n) for n in ['cost', 'hsr-bench', 'buffer_stream_bench', 'character_eval']}))


def costs():
    """process_mesh vs the manual sequence, instructions:u, corpus and real meshes."""
    for x in json.loads((A / 'inputs.json').read_text()):
        for rep in range(3):
            ops = ['process', 'manual']
            for op in ops if rep != 1 else ops[::-1]:
                sample(x['case'], op, 'cost', rep)


def streaming_costs():
    """Streaming vs one-shot decode, instructions:u per decode (BENCH_RUNS=200 minus the setup-only run)."""
    b = T / 'bin/buffer_stream_bench'
    rows = []
    setup = [perf_instructions([b], env={'BENCH_ONLY': 'none', 'BENCH_RUNS': '200'}, cpu=physical_cpus()[3])[0] for _ in range(3)]
    setup = statistics.median(setup)
    for version in (0, 1):
        for kind in ('vertex', 'tri'):
            res = {}
            for mode in ('oneshot', '4096', '65536'):
                name = f'v{version}:{kind}:{mode}'
                counts = [perf_instructions([b], env={'BENCH_ONLY': name, 'BENCH_RUNS': '200'}, cpu=physical_cpus()[3])[0] for _ in range(3)]
                res[mode] = (statistics.median(counts) - setup) / 200
            rows.append(dict(version=version, kind=kind, instructions_per_decode=res,
                             ratio_4096=res['4096'] / res['oneshot'], ratio_65536=res['65536'] / res['oneshot']))
            print(rows[-1], flush=True)
    write(A / 'streaming-costs.json', dict(setup_instructions=setup, rows=rows))


def hsr_costs():
    """Hidden-surface removal on the HSR lane's existing eval meshes: wall seconds (median of 3) and whole-process instructions."""
    src = Path(os.environ['HSR_EVAL_DIR'])
    rows = []
    for name in ['breakfast_room', 'living_room', 'conference']:
        path = src / (name + '.bin')
        times, instr = [], []
        for _ in range(3):
            p = subprocess.run(['taskset', '-c', str(physical_cpus()[3]), str(T / 'bin/hsr-bench'), 'hsr', str(path), str(T / 'hsr-out')], input='stop\n', capture_output=True, text=True)
            assert p.returncode == 0, p.stderr[-2000:]
            line = next(l for l in p.stdout.splitlines() if l.startswith('RESULT'))
            times.append(float(line.split()[1]))
            report = line.split(' ', 2)[2]
        count, _ = perf_instructions([T / 'bin/hsr-bench', 'hsr', path, T / 'hsr-out'], cpu=physical_cpus()[3])
        rows.append(dict(mesh=name, seconds=times, median_seconds=statistics.median(times), process_instructions_u=count, report=report))
        print(rows[-1], flush=True)
    write(A / 'hsr-costs.json', rows)


def character_costs():
    """Character LOD: single-pass vs best-of times (ms, from the example's own CSV) on the best-of lane's 12 eval inputs, median of 3 runs."""
    src = Path(os.environ['BESTOF_INPUTS_DIR'])
    rows = []
    for d in sorted(src.iterdir()):
        path = d / (d.name + '.eval.bin')
        runs = []
        for _ in range(3):
            p = subprocess.run(['taskset', '-c', str(physical_cpus()[3]), str(T / 'bin/character_eval'), str(path), '--best-of'], capture_output=True, text=True)
            assert p.returncode == 0, (d.name, p.stderr[-1500:])
            lines = [l for l in p.stdout.splitlines() if l and l[0].isdigit()]
            runs.append([l.split(',') for l in lines])
        per_target = []
        for k in range(len(runs[0])):
            per_target.append(dict(target=runs[0][k][0], chosen=runs[0][k][1], triangles=runs[0][k][2],
                                   character_ms=statistics.median(float(r[k][6]) for r in runs), bestof_ms=statistics.median(float(r[k][7]) for r in runs)))
        rows.append(dict(model=d.name, targets=per_target))
        print(rows[-1], flush=True)
    write(A / 'character-costs.json', rows)


def wasm_platform():
    """wasm32 (node) one-shot vertex/index decode wall medians: v0.2.16 vs train vs the C++ wasm decoder (reference js)."""
    run('aarch64-check', ['cargo', 'check', '--offline', '--locked', '--target', 'aarch64-unknown-linux-gnu', '--all-features', '--lib'], estimate=600, check=False)
    flags = {'RUSTFLAGS': '-C target-feature=+simd128'}
    base = T / 'simd-base'
    (base / 'src').mkdir(parents=True, exist_ok=True)
    shutil.copytree(R / 'parity/simd/src', base / 'src', dirs_exist_ok=True)
    manifest_text = (R / 'parity/simd/Cargo.toml').read_text()
    needle = 'meshoptimizer-rs = { path = "../..", default-features = false, features = ["std", "simd", "parity-internals"] }'
    assert needle in manifest_text
    (base / 'Cargo.toml').write_text(manifest_text.replace(needle, 'meshoptimizer-rs = { package = "meshoptimizer-baseline", path = ' + json.dumps(str(T / 'base')) +
                                                           ', default-features = false, features = ["std", "simd", "parity-internals"] }'))
    shutil.copy2(R / 'parity/simd/Cargo.lock', base / 'Cargo.lock')
    out = A / 'wasm'
    out.mkdir(exist_ok=True)
    run('wasm-build-before', ['cargo', 'build', '--offline', '--release', '--manifest-path', base / 'Cargo.toml', '--target', 'wasm32-unknown-unknown', '--lib', '--target-dir', T / 'wasm-before'], env=flags, estimate=600)
    run('wasm-build-after', ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', R / 'parity/simd/Cargo.toml', '--target', 'wasm32-unknown-unknown', '--lib', '--target-dir', T / 'wasm-after'], env=flags, estimate=600)
    shutil.copy2(T / 'wasm-before/wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm', out / 'before.wasm')
    shutil.copy2(T / 'wasm-after/wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm', out / 'after.wasm')
    s = (R / 'parity/0217-decoders/measure.py').read_text()
    old = "art = root/'parity/0217-decoders/receipts'"
    assert old in s
    script = T / 'wasm_measure.py'
    script.write_text(s.replace(old, "art = pathlib.Path(os.environ['QUALIFY_WASM_ART'])"))
    run('wasm-measure', ['python3', script], env={'QUALIFY_WASM_ART': str(out), 'DECODE_INPUTS': str(T / 'inputs'), 'DECODE_MANIFEST': str(A / 'inputs.json')}, estimate=3000)
    write(out / 'binding.json', {n: sha(out / n) for n in ['before.wasm', 'after.wasm']})


def summarize():
    import math
    rows = [json.loads(l) for l in (A / 'counts.jsonl').read_text().splitlines()]
    inputs = json.loads((A / 'inputs.json').read_text())
    camp = [x for x in rows if x['arm'] in ARMS]
    expected = len(OPS) * 8 * 3 * len(ARMS)
    assert len(camp) == expected, (len(camp), expected)
    med, hashes = {}, {}
    for x in rows:
        key = (x['case'], x['op'], x['arm'])
        med.setdefault(key, []).append(x['instructions_per_call'])
        hashes.setdefault(key, set()).add(x['output_sha256'])
    med = {k: statistics.median(v) for k, v in med.items()}
    cells, regressions = [], []
    for op in OPS:
        for x in inputs:
            case = x['case']
            m = {a: med[case, op, a] for a in ARMS}
            ident = {}
            for sfx in ['scalar', 'default']:
                ident['base_train_' + sfx] = hashes[case, op, 'base-' + sfx] == hashes[case, op, 'train-' + sfx]
                ident['train_cpp_' + sfx] = hashes[case, op, 'train-' + sfx] == hashes[case, op, 'cpp-' + sfx]
            v = dict(scalar_base=m['train-scalar'] / m['base-scalar'], default_base=m['train-default'] / m['base-default'],
                     scalar_cpp=m['train-scalar'] / m['cpp-scalar'], default_cpp=m['train-default'] / m['cpp-default'])
            for k in ['scalar_base', 'default_base']:
                if v[k] > 1.01:
                    regressions.append(dict(case=case, op=op, mode=k, ratio=v[k]))
            cells.append(dict(case=case, op=op, medians=m, ratios=v, identity=ident))

    def gm(v):
        return math.exp(sum(math.log(x) for x in v) / len(v))
    summaries = []
    for op in OPS:
        c = [x for x in cells if x['op'] == op]
        groups = {}
        for name, is_real in [('real', True), ('corpus', False)]:
            g = [x for x in c if x['case'].startswith('real-') == is_real]
            groups[name] = {k: gm([x['ratios'][k] for x in g]) for k in c[0]['ratios']}
        summaries.append(dict(op=op, groups=groups, max_scalar=max(x['ratios']['scalar_base'] for x in c), max_default=max(x['ratios']['default_base'] for x in c),
                              identical=all(all(x['identity'].values()) for x in c)))
    write(A / 'release-results.json', dict(samples=len(camp), cells=cells, summary=summaries, regressions=regressions))
    table = ['# 0.2.18 combined release table', '',
             'Median of three instructions:u samples per case. Each entry is real-mesh GM / corpus GM of train/other. Ratios below 1 mean fewer instructions. '
             'Real: real-214e, real-a082, real-b6fd. Corpus: five frozen synthetic meshes. C++ 1.3 is matched scalar/default. Base is v0.2.17. '
             'Flag column: `>1%` marks a function with any case, or either group GM, more than 1% above v0.2.17 in that mode.', '',
             '| Function | scalar / 0.2.17 | default / 0.2.17 | scalar / scalar C++ | default / default C++ | max case scalar / default vs base | flag |',
             '|---|---:|---:|---:|---:|---:|---|']
    metrics = ['scalar_base', 'default_base', 'scalar_cpp', 'default_cpp']
    flagged = []
    for x in summaries:
        vals = [f"{x['groups']['real'][k]:.3f} / {x['groups']['corpus'][k]:.3f}" for k in metrics]
        slow = x['max_scalar'] > 1.01 or x['max_default'] > 1.01 or any(x['groups'][g][k] > 1.01 for g in ['real', 'corpus'] for k in ['scalar_base', 'default_base'])
        if slow:
            flagged.append(x['op'])
        table.append(f"| {x['op']} | " + ' | '.join(vals) + f" | {x['max_scalar']:.6f} / {x['max_default']:.6f} | {'>1%' if slow else ''} |")
    ident = sum(all(c['identity'][k] for k in ('base_train_scalar', 'base_train_default')) for c in cells)
    cpp = sum(all(c['identity'][k] for k in ('train_cpp_scalar', 'train_cpp_default')) for c in cells)
    table += ['', f"Campaign samples: {len(camp)}. Base/train output identity cells (scalar and default both): {ident}/{len(cells)}. Train/C++ output identity cells: {cpp}/{len(cells)}.",
              f"Cases above 1.01 vs v0.2.17: {len(regressions)}. Functions flagged: {', '.join(flagged) if flagged else 'none'}. Per-case medians and ratios: [release-results.json](release-results.json)."]
    (A / 'TABLE.md').write_text('\n'.join(table) + '\n')
    print('samples', len(camp), 'regressions', len(regressions), 'flagged', flagged, flush=True)
    for x in regressions:
        print(x, flush=True)


def par_identity():
    d = T / 'identity'
    run('par-identity-build', ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=900)
    shutil.copy2(T / 'release/parallel_identity', T / 'bin/parallel_identity')
    run('parallel_identity-all', [T / 'bin/parallel_identity', T / 'inputs'], estimate=3000, check=False)


def differential_rest():
    o = old0216()
    for lane in ['decode', 'vdecode']:
        for mode in ['all', 'none']:
            o.decode_differential(lane, mode)
            if lane == 'decode' and mode == 'all':
                extended_decode()
    lodq_differential()


def boundary_gate():
    run('boundary-final', ['bash', 'parity/check-unsafe-boundary.sh'], estimate=60, check=False)


def core2():
    failures = []
    for part in (par_identity, differential_rest, build_group, boundary_gate):
        try:
            part()
        except SystemExit as e:
            failures.append((part.__name__, e.code))
            print('PART FAILED', part.__name__, e.code, flush=True)
    write(A / 'core2-failures.json', failures)


def core():
    """Proof, identity, C++ differentials and peer builds in one admitted job; a failing part is recorded and the rest continue."""
    failures = []
    for part in (proof_group, identity_group, differential_group, build_group):
        if part.__name__ in os.environ.get('QUALIFY_SKIP', '').split(','):
            print('SKIP (completed in an earlier admitted job of this run)', part.__name__, flush=True)
            continue
        try:
            part()
        except SystemExit as e:
            failures.append((part.__name__, e.code))
            print('PART FAILED', part.__name__, e.code, flush=True)
    write(A / 'core-failures.json', failures)


def disclosure_rest():
    costs()
    streaming_costs()
    hsr_costs()
    character_costs()


def disclosure_group():
    build_disclosure()
    costs()
    streaming_costs()
    hsr_costs()
    character_costs()


DET_VARIANTS = [
    # name, features of meshoptimizer-rs, target, profile, extra env
    ('native-all', ALL, None, 'release', {}),
    ('native-all-debug', ALL, None, 'dev', {}),
    ('native-default', ['std', 'simd'], None, 'release', {}),
    ('native-nostd', [], None, 'release', {}),
    ('native-nostd-simd', ['simd'], None, 'release', {}),
    ('wasm-nostd', [], 'wasm32-unknown-unknown', 'release', {}),
    ('wasm-default', ['std', 'simd'], 'wasm32-unknown-unknown', 'release', {}),
    ('wasm-default-simd128', ['std', 'simd'], 'wasm32-unknown-unknown', 'release', {'RUSTFLAGS': '-C target-feature=+simd128'}),
]


def new_apis_determinism():
    """Hash equality of bake_impostor and trim_alpha_cards over std / no_std, native / wasm32, release / debug."""
    results = {}
    for name, features, target, profile, env in DET_VARIANTS:
        d = T / 'det' / name
        d.mkdir(parents=True, exist_ok=True)
        (d / 'Cargo.toml').write_text(
            '[package]\nname="det"\nversion="0.0.0"\nedition="2021"\n'
            f'[lib]\nname="determinism"\npath={json.dumps(str(A / "determinism.rs"))}\ncrate-type=["rlib","cdylib"]\n'
            f'[[bin]]\nname="determinism_main"\npath={json.dumps(str(A / "determinism_main.rs"))}\n'
            f'[dependencies]\nmeshoptimizer-rs={{path={json.dumps(str(R))},default-features=false,features={json.dumps(features)}}}\n'
            '[profile.release]\ncodegen-units=1\n')
        run('det-lock-' + name, ['cargo', 'generate-lockfile', '--offline', '--manifest-path', d / 'Cargo.toml'], estimate=5)
        tdir = T / 'det-target' / name
        build = ['cargo', 'build', '--offline', '--locked', '--manifest-path', d / 'Cargo.toml', '--target-dir', tdir]
        if profile == 'release':
            build.append('--release')
        if target:
            build += ['--target', target, '--lib']
        else:
            build += ['--bin', 'determinism_main']
        run('det-build-' + name, build, env=env, estimate=900)
        out = tdir / (target or '') / ('release' if profile == 'release' else 'debug')
        if target:
            wasm = out / 'determinism.wasm'
            n = len(results['native-all']['hashes'])
            log = run('det-run-' + name, ['node', A / 'determinism_wasm.mjs', wasm, n], estimate=300)
            hashes = [l.split()[1] for l in log.read_text().splitlines() if l and l[0].isdigit()]
            binary = wasm
        else:
            log = run('det-run-' + name, [out / 'determinism_main'], estimate=300)
            rows = [l.split() for l in log.read_text().splitlines() if len(l.split()) == 2 and len(l.split()[1]) == 16]
            hashes = [r[1] for r in rows]
            binary = out / 'determinism_main'
        results[name] = dict(hashes=hashes, binary_sha256=sha(binary), features=features, target=target or 'native', profile=profile)
        if name == 'native-all':
            results[name]['cases'] = [r[0] for r in rows]
    reference = results['native-all']['hashes']
    for name, r in results.items():
        r['equal_to_native_all'] = r['hashes'] == reference and len(reference) > 0
    write(A / 'determinism.json', dict(variants=results, all_equal=all(r['equal_to_native_all'] for r in results.values())))
    assert all(r['equal_to_native_all'] for r in results.values()), 'determinism mismatch'


def semver():
    """cargo-semver-checks against the published baseline (CARGO_SEMVER_CHECKS = path of the cargo-semver-checks binary)."""
    tool = os.environ['CARGO_SEMVER_CHECKS']
    for label, extra in [('default', []), ('all-features', ['--all-features'])]:
        run('semver-' + label, [tool, 'semver-checks', 'check-release', '--baseline-version', BASE_TAG[1:], *extra], estimate=1800, check=False)


def lane_costs():
    """Cost disclosure for the new APIs: the three lanes' measurement harnesses rebuilt against this tree.

    LANE_HARNESS_ROOT holds meshopt-{impostors,cutout,blend}/harness; LANE_ASSETS_IMPOSTOR and LANE_ASSETS_CUTOUT hold the
    asset folders they read. Wall times are the harnesses' own (best of 3), taken on one pinned core of a shared host.
    """
    import re
    root = Path(os.environ['LANE_HARNESS_ROOT'])
    assets = {'impostors': Path(os.environ['LANE_ASSETS_IMPOSTOR']), 'cutout': Path(os.environ['LANE_ASSETS_CUTOUT'])}
    out = T / 'lane-out'
    out.mkdir(exist_ok=True)
    cpu = physical_cpus()[3]
    binding = {}
    for lane in ['impostors', 'cutout', 'blend']:
        src = root / f'meshopt-{lane}' / 'harness'
        d = T / 'lane' / lane
        if d.exists():
            shutil.rmtree(d)
        shutil.copytree(src, d, ignore=shutil.ignore_patterns('target'))
        text = (d / 'Cargo.toml').read_text()
        text, n = re.subn(r'(meshoptimizer-rs\s*=\s*\{\s*path\s*=\s*)"[^"]*"', lambda m: m.group(1) + json.dumps(str(R)), text)
        assert n == 1, lane
        (d / 'Cargo.toml').write_text(text)
        run('lane-build-' + lane, ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'], estimate=900)
        shutil.copy2(T / 'release' / {'impostors': 'impostor-harness', 'cutout': 'cutout-harness', 'blend': 'blend'}[lane], T / 'bin' / f'lane-{lane}')
        binding[lane] = sha(T / 'bin' / f'lane-{lane}')
        if lane == 'blend':
            shutil.copy2(T / 'release/trim', T / 'bin/lane-blend-trim')
            binding['blend-trim'] = sha(T / 'bin/lane-blend-trim')
    write(A / 'lane-binding.json', binding)
    for name, binary, folder in [('impostors', 'lane-impostors', 'impostors'), ('cutout', 'lane-cutout', 'cutout'),
                                 ('blend', 'lane-blend', 'impostors'), ('blend-trim', 'lane-blend-trim', 'cutout')]:
        (out / name).mkdir(exist_ok=True)
        run('lane-cost-' + name, ['taskset', '-c', cpu, T / 'bin' / binary, assets[folder], out / name], estimate=3000, check=False)


def determinism_retry():
    """Determinism stage; when it fails, wait up to 30 minutes for an edit of the driver files and retry (at most 4 attempts)."""
    files = [A / 'determinism.rs', A / 'determinism_main.rs', A / 'determinism_wasm.mjs']
    for attempt in range(4):
        stamp = [f.stat().st_mtime for f in files]
        try:
            return new_apis_determinism()
        except (SystemExit, AssertionError) as e:
            print('determinism attempt', attempt + 1, 'failed', repr(e), flush=True)
            if attempt == 3:
                raise
            for _ in range(180):
                if [f.stat().st_mtime for f in files] != stamp:
                    break
                time.sleep(10)
            else:
                raise


def everything():
    """One admitted job for the whole 0.2.18 qualification; a failing part is recorded and the rest continue."""
    parts = [determinism_retry, semver, core]
    parts += [(lambda g=g: measure(g)) for g in ('corpus', 'real-214e', 'real-a082', 'real-b6fd')]
    parts += [summarize, lane_costs]
    failures = []
    for part in parts:
        name = getattr(part, '__name__', '<measure>')
        if name in os.environ.get('QUALIFY_SKIP', '').split(','):
            print('SKIP (completed in an earlier admitted job of this run)', name, flush=True)
            continue
        try:
            part()
        except (Exception, SystemExit) as e:
            failures.append((name, repr(e)))
            print('PART FAILED', name, repr(e), flush=True)
    write(A / 'everything-failures.json', failures)


if __name__ == '__main__':
    assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU') == '0'
    stage = sys.argv[1]
    start = time.monotonic()
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    code = 0
    try:
        globals()[stage](*sys.argv[2:])
    except BaseException as e:
        code = e.code if isinstance(e, SystemExit) and isinstance(e.code, int) else 1
        raise
    finally:
        after = resource.getrusage(resource.RUSAGE_CHILDREN)
        with (A / 'jobs.jsonl').open('a') as f:
            f.write(json.dumps(dict(stage=' '.join(sys.argv[1:]), exit=code, child_cpu_seconds=after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime,
                                    elapsed_seconds=time.monotonic() - start)) + '\n')
