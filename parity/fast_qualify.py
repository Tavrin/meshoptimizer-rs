#!/usr/bin/env python3
"""Parallel, change-scoped performance qualification. See FAST_QUALIFY_VALIDATION.md."""
import argparse
import array
from concurrent.futures import ProcessPoolExecutor, as_completed
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import statistics
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / 'parity'))
import fast_stats as fs

TARGET = Path(os.environ.get('CARGO_TARGET_DIR', ROOT.parent / 'meshopt-fastq-target')).resolve()
ART = Path(os.environ.get('MESHOPT_ARTIFACTS', ROOT.parent / 'meshopt-fastq-artifacts')).resolve()
REFERENCE = Path(os.environ.get('MESHOPT_REFERENCE', ROOT.parent / 'meshoptimizer')).resolve()
BAR_GM = 1.25
BAR_CASE = 1.50
BAR_MEMORY = 1.25
TIMING_GATE_UNIX = float(os.environ.get('MESHOPT_FAST_NOT_BEFORE', '0'))
GPU_LEASE = os.environ.get('MESHOPT_FAST_GPU_LEASE', '')
SCOREBOARD_PATTERN = os.environ.get('MESHOPT_FAST_SCOREBOARD_PATTERN', '')


def timing_status():
    """Read all coordinator conditions; unavailable evidence fails closed."""
    now = time.time()
    status = {'checked_unix': now, 'allowed': False, 'blocked': []}
    if now < TIMING_GATE_UNIX:
        status['blocked'].append('before configured timing deadline')
    checks = {
        'uptime': ['uptime'],
        'gpu_lease': [str(GPU_LEASE), 'status'],
        'scoreboard_units': ['systemctl', '--user', 'list-units',
                            '--state=active,activating,reloading,deactivating',
                            SCOREBOARD_PATTERN or 'meshopt-scoreboard-*', '--plain', '--no-legend', '--no-pager'],
    }
    if not GPU_LEASE or not SCOREBOARD_PATTERN:
        status['blocked'].append('configure MESHOPT_FAST_GPU_LEASE and MESHOPT_FAST_SCOREBOARD_PATTERN')
    for name, command in checks.items():
        try:
            status[name] = subprocess.check_output(command, text=True, stderr=subprocess.STDOUT,
                                                   timeout=20).strip()
        except (OSError, subprocess.SubprocessError) as error:
            status['blocked'].append(f'{name} unavailable: {error}')
    match = re.search(r'load averages?:\s*([0-9]+(?:[.,][0-9]+)?)', status.get('uptime', ''))
    if match:
        status['load_one'] = float(match.group(1).replace(',', '.'))
        if status['load_one'] >= 12:
            status['blocked'].append(f"1-minute load {status['load_one']:.2f} is not below 12")
    else:
        status['blocked'].append('cannot read 1-minute load from uptime')
    if not status.get('gpu_lease', '').startswith('GPU lease: FREE'):
        status['blocked'].append('GPU lease held or unknown')
    if status.get('scoreboard_units'):
        status['blocked'].append('active matching scoreboard unit')
    status['allowed'] = not status['blocked']
    status['condition'] = ('configured deadline elapsed, uptime 1-minute load < 12, '
                           'no GPU lease holder, no active matching scoreboard unit')
    return status


def timing_admission():
    status = timing_status()
    if not status['allowed']:
        print('timing admission refused: ' + '; '.join(status['blocked']) +
              '; poll again in two minutes', file=sys.stderr, flush=True)
        raise SystemExit(75)
    return status


def admitted_cores(requested=None):
    try:
        return fs.physical_cores(requested=requested)
    except ValueError as error:
        if str(error) != 'no physical core below 50% busy at admission':
            raise
        print(str(error) + '; poll again in two minutes', file=sys.stderr, flush=True)
        raise SystemExit(75) from error


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + '.tmp')
    temporary.write_text(json.dumps(value, indent=2) + '\n')
    temporary.replace(path)


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    sys.modules[name] = result
    spec.loader.exec_module(result)
    return result


def dependency_files():
    """Hash the exact installed dependency bytes, not just lockfile versions."""
    metadata = json.loads(subprocess.check_output(
        ['cargo', 'metadata', '--offline', '--locked', '--format-version', '1'], cwd=ROOT))
    result = {}
    for package in metadata['packages']:
        if package['source'] is None:
            continue
        directory = Path(package['manifest_path']).parent
        for path in sorted(directory.rglob('*')):
            if path.is_file():
                result[f"{package['name']}-{package['version']}/{path.relative_to(directory)}"] = sha(path)
    return result


def setup(phase, profile, artifact_dir):
    for key in list(os.environ):
        if key.startswith('CARGO_PROFILE_'):
            del os.environ[key]
    os.environ.update(CARGO_TARGET_DIR=str(TARGET), MESHOPT_ARTIFACTS=str(artifact_dir),
                      MESHOPT_REFERENCE=str(REFERENCE), CARGO_NET_OFFLINE='true',
                      RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', RUSTFLAGS='',
                      CARGO_ENCODED_RUSTFLAGS='')
    os.environ.update(CARGO_INCREMENTAL='0', CARGO_PROFILE_DEV_DEBUG='0', CARGO_BUILD_JOBS='2')
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / 'tmp').mkdir(exist_ok=True)
    os.environ['TMPDIR'] = str(TARGET / 'tmp')
    if phase in ('0.1', '0.1.x'):
        import runner as r
        r.ENV.update(os.environ)
        for key in list(r.ENV):
            if key.startswith('CARGO_PROFILE_'):
                del r.ENV[key]
        r.ENV['CARGO_PROFILE_DEV_DEBUG'] = '0'
        if profile == 'moss':
            r.ENV.update(CARGO_PROFILE_RELEASE_OPT_LEVEL='3', CARGO_PROFILE_RELEASE_DEBUG='0',
                         CARGO_PROFILE_RELEASE_LTO='thin', CARGO_PROFILE_RELEASE_CODEGEN_UNITS='1')
        elif profile == 'default':
            r.ENV.update(CARGO_PROFILE_RELEASE_OPT_LEVEL='3', CARGO_PROFILE_RELEASE_DEBUG='0',
                         CARGO_PROFILE_RELEASE_LTO='false', CARGO_PROFILE_RELEASE_CODEGEN_UNITS='16')
        elif profile != 'crate':
            raise ValueError('0.1 profile must be crate, moss or default')
        reference, target, _ = r.paths()
        binaries, identity = r.build(reference, target, wasm=False)
        identity['effective_profile_overrides'] = {
            key: value for key, value in r.ENV.items()
            if key.startswith('CARGO_PROFILE_RELEASE_')
        }
        return {k: str(v) for k, v in binaries.items() if k in ('cpp', 'rust')}, identity
    sys.path.insert(0, str(ROOT / 'parity' / ('p03' if phase == '0.3' else 'codec')))
    if phase == '0.3':
        os.environ['MESHOPT_RUST_PROFILE'] = profile
        r = module('fast_p03_runner', ROOT / 'parity/p03/runner.py')
        dependencies = dependency_files()
        binaries, identity = r.build(False)
        if dependencies != dependency_files():
            raise ValueError('dependency changed during build')
        identity['dependencies'] = dependencies
        return {k: str(v) for k, v in binaries.items()}, identity
    if phase == '0.4':
        m = module('fast_codec_measure', ROOT / 'parity/codec/measure.py')
        m04 = module('fast_codec_measure04', ROOT / 'parity/codec/measure04.py')
        before = m04.runner04.sources()
        binaries = {'scalar': m.build_cpp(True), 'simd': m.build_cpp(),
                    'rust': m04.build_rust(profile)}
        if before != m04.runner04.sources():
            raise ValueError('source changed during build')
        identity = {'sources': before,
                    'executables': {k: sha(v) for k, v in binaries.items()},
                    'profile': m04.PROFILES[profile]}
        return {k: str(v) for k, v in binaries.items()}, identity
    if phase == '0.2':
        m = module('fast_codec_measure', ROOT / 'parity/codec/measure.py')
        r = module('fast_codec_runner', ROOT / 'parity/codec/runner.py')
        binaries = r.build()
        identity = {'sources': r.sources(),
                    'executables': {k: sha(v) for k, v in binaries.items()}}
        return {k: str(v) for k, v in binaries.items() if k != 'wasm'}, identity
    raise ValueError('unsupported phase')


def case_file(directory, name, data, family, **extra):
    data = bytes(data)
    digest = hashlib.sha256(data).hexdigest()
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / (name.replace('/', '--') + '--' + digest[:16] + '.input')
    if path.exists():
        if sha(path) != digest:
            raise ValueError('existing input differs: ' + name)
    else:
        path.write_bytes(data)
    return {'case': name, 'family': family, 'input': str(path), 'input_sha256': digest, **extra}


def cases01(directory, binaries, smoke):
    import runner as r
    import performance as p
    out = []
    for size, triangles, shape in p.workloads():
        if smoke and not (size == 'tiny' and shape == 'smooth'):
            continue
        positions, indices = p.geometry(r, triangles, shape)
        cached = None
        for op, ratio, ac in p.configurations():
            topology = indices
            if op == 2:
                if cached is None:
                    raw, _ = r.response(r.execute(Path(binaries['rust']), r.message(1, positions, indices)), len(indices))
                    cached = array.array('I')
                    cached.frombytes(raw)
                    if sys.byteorder != 'little':
                        cached.byteswap()
                topology = cached
            for api in (['allocation-free'] if op == 6 else ['allocating', 'caller-buffer']):
                name = f'{r.FAMILIES[op]}/{size}/{shape}/{api}/a{ac}/r{ratio}'
                data = bytearray(r.message(op, positions, topology,
                                           mode=2 if api == 'caller-buffer' else 1,
                                           samples=100, variant={0: 0, 1: 1, 8: 4, 12: 5}[ac]))
                if ratio is not None:
                    struct.pack_into('<IfI', data, 28+len(positions)*4+len(topology)*4,
                                     int(len(topology)*ratio), max(1-ratio, .05), 32 if op == 5 else 0)
                out.append(case_file(directory, name, data, r.FAMILIES[op], op=op,
                                     output_count=len(topology) if op <= 2 else None))
    return out


def cases03(directory, smoke):
    r = module('fast_p03_runner', ROOT / 'parity/p03/runner.py')
    b = module('fast_p03_benchmark', ROOT / 'parity/p03/benchmark.py')
    out = []
    for op, family in enumerate(r.NAMES, 1):
        for label, data in b.cases(op, smoke):
            out.append(case_file(directory, family + '/' + label, data, family))
    return out


def cases01x(directory, smoke):
    """Mirror the final preprocessing matrix; validation binds every input hash."""
    import runner as r
    import performance as p
    b = module('fast_p01x', ROOT / 'parity/p01x.py')
    out = []
    for size, triangles, shape in p.workloads():
        if smoke and not (size == 'tiny' and shape == 'smooth'):
            continue
        positions, indices = p.geometry(r, triangles, shape)
        for op, family in b.FAMILIES.items():
            variants = [(None, None, '')]
            if op == 27 or op >= 34:
                extra = {'smooth': (0, .25), 'seam-heavy': (12, .75),
                         'disconnected': (1, .5)}.get(shape)
                if extra:
                    variants.append((*extra, f'/a{extra[0]}-r{extra[1]}'))
            if op == 26 and shape in ('smooth', 'seam-heavy'):
                variants.append((None, .5, '/half-points'))
            if op == 26:
                variants.append((None, .5, '/colored-half-points'))
            for mode in (1, 2):
                for ac, ratio, suffix in variants:
                    variant = 3 if op == 27 or op >= 34 or suffix == '/colored-half-points' else 4
                    data = b.message(op, positions, indices, variant, mode, 100, ac, ratio)
                    name = f'{family}/{size}/{shape}/mode-{mode}' + suffix
                    out.append(case_file(directory, name, data, family, op=op, output_count=None))
    return out


def cases04(directory, smoke):
    m = module('fast_codec_measure', ROOT / 'parity/codec/measure.py')
    b = module('fast_codec_measure04', ROOT / 'parity/codec/measure04.py')
    base, _ = b.inputs()
    out = []
    for name, data in base:
        if smoke and not ('tiny' in name or name.startswith('bounds/')):
            continue
        family = name.split('/')[0]
        for api in (['allocating'] if family == 'bounds' else ['allocating', 'caller_buffer']):
            req = bytearray(data)
            if api == 'caller_buffer':
                struct.pack_into('<I', req, 16, struct.unpack_from('<I', req, 16)[0] | 128)
            out.append(case_file(directory, f'{family}/{api}/{name.split("/", 1)[1]}', req,
                                 family + '/' + api, decoded_bytes=struct.unpack_from('<II', req, 8)[0] * struct.unpack_from('<II', req, 8)[1]))
    return out


def registered_minima():
    if not os.environ.get('MESHOPT_DECODER_BASELINE'):
        raise ValueError('set MESHOPT_DECODER_BASELINE to the exact registered baseline.json')
    source = Path(os.environ['MESHOPT_DECODER_BASELINE'])
    if not source.exists():
        raise ValueError('exact registered 0.2 baseline.json is missing; rounded DECODER_BAR.md values cannot replace the bar')
    expected = json.loads((ROOT / 'parity/P02_RESULTS.json').read_text())['baseline_sha256']
    if sha(source) != expected:
        raise ValueError('registered 0.2 baseline SHA-256 mismatch')
    result = {row['case']: row['minimum_bytes_second'] for row in json.loads(source.read_text())['raw']}
    if len(result) < 50:
        raise ValueError('incomplete registered decoder minima')
    return result


def cases02(directory, binaries, smoke):
    m = module('fast_codec_measure', ROOT / 'parity/codec/measure.py')
    artifact_dir = Path(os.environ['MESHOPT_ARTIFACTS'])
    if not (artifact_dir / 'benchmark-inputs.json').exists():
        cpp = m.Driver(binaries['scalar'])
        base = m.corpus(cpp)
        cpp.close()
        manifest = []
        for name, data in base:
            item = case_file(directory, name, data, 'inputs')
            manifest.append({'case': name, 'path': item['input'], 'sha256': item['input_sha256']})
        write(artifact_dir / 'benchmark-inputs.json', manifest)
    r = module('fast_codec_runner', ROOT / 'parity/codec/runner.py')
    base = m.candidate_cases(binaries)
    minima = registered_minima()
    out = []
    for name, data in base:
        if smoke and not 'tiny' in name:
            continue
        family = name.split('-')[0]
        for api in ('allocating', 'caller_buffer'):
            req = bytearray(data)
            if api == 'caller_buffer':
                struct.pack_into('<I', req, 16, struct.unpack_from('<I', req, 16)[0] | 128)
            decoded = struct.unpack_from('<II', req, 8)
            out.append(case_file(directory, f'{family}/{api}/{name}', req, api,
                                 raw_scalar=name.startswith(('vertex-', 'index-')),
                                 decoded_bytes=decoded[0] * decoded[1],
                                 registered_minimum=minima.get(name)))
    return out


def fingerprint(phase, profile, identity, family):
    """Conservative whole-source reuse, including nested harness/build inputs."""
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock']
    for name in ('src', 'tests', 'parity', 'fuzz'):
        paths += sorted((ROOT / name).rglob('*'))
    extensions = {'.py', '.rs', '.cpp', '.h', '.toml', '.lock', '.sh', '.cjs', '.mjs'}
    files = {str(p.relative_to(ROOT)): sha(p) for p in paths if p.is_file() and
             'results' not in p.relative_to(ROOT).parts and
             (p.suffix in extensions or p.name == 'DECODER_BAR.md')}
    oracle = {str(p.relative_to(REFERENCE)): sha(p)
              for directory in ('src', 'js', 'demo')
              for p in sorted((REFERENCE / directory).rglob('*')) if p.is_file()}
    payload = {'phase': phase, 'profile': profile, 'family': family,
               'sources': files, 'oracle': oracle, 'identity': identity}
    return hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()


def source_unchanged(phase, identity):
    """Re-read the build's complete source/dependency snapshot after sampling."""
    if phase in ('0.1', '0.1.x'):
        import runner as r
        return r.snapshot(REFERENCE) == identity['source_sha256']
    if phase == '0.3':
        r = module('fast_p03_runner', ROOT / 'parity/p03/runner.py')
        return (r.snapshot() == identity['sources'] and
                dependency_files() == identity['dependencies'])
    if phase == '0.4':
        r = module('fast_codec_runner04', ROOT / 'parity/codec/runner04.py')
    else:
        r = module('fast_codec_runner', ROOT / 'parity/codec/runner.py')
    return r.sources() == identity['sources']


def prior_is_qualified(previous, old, receipt):
    """A record's self-declared release_grade never substitutes for a receipt."""
    if (receipt is None or not old.get('complete') or old.get('smoke') or
            not old.get('source_unchanged') or not old.get('binary_unchanged') or
            old.get('sequential_error', {}).get('maximum_pairs') != receipt.get('maximum_pairs')):
        return False
    return (sha(previous) in receipt.get('validated_records_sha256', []) or
            old.get('release_grade') is True and
            old.get('validation_receipt_sha256') == sha(ART / 'validation-receipt.json'))


def require_paths():
    for name in ('CARGO_TARGET_DIR', 'MESHOPT_ARTIFACTS', 'MESHOPT_REFERENCE'):
        if not os.environ.get(name):
            raise ValueError('set ' + name + ' explicitly before running validation')
    for path in (TARGET, ART):
        if any(path == source or source in path.parents or
                   path == TARGET and path in source.parents
               for source in (ROOT, REFERENCE)):
            raise ValueError('target and artifacts must be outside both source trees')
    if TARGET == ART or TARGET in ART.parents or ART in TARGET.parents:
        raise ValueError('target and artifacts must be separate trees')


def validation_receipt():
    path = ART / 'validation-receipt.json'
    if not path.exists():
        return None
    receipt = json.loads(path.read_text())
    if (receipt.get('accepted') is True and
            receipt.get('harness_sha256') == sha(ROOT / 'parity/fast_qualify.py') and
            receipt.get('statistics_sha256') == sha(ROOT / 'parity/fast_stats.py') and
            (ROOT / 'parity/fast_validate.py').is_file() and
            receipt.get('validator_sha256') == sha(ROOT / 'parity/fast_validate.py') and
            20 <= receipt.get('maximum_pairs', 0) <= 90):
        return receipt
    return None


def init_worker(cores):
    from multiprocessing import current_process
    index = (current_process()._identity[0] - 1) % len(cores)
    core = cores[index]
    os.sched_setaffinity(0, {core['cpu']})
    os.environ['MESHOPT_BENCH_CPU'] = str(core['cpu'])
    os.environ['MESHOPT_FAST_CORE'] = json.dumps(core)


def measure_task(phase, task, binaries, alpha, max_pairs):
    core = json.loads(os.environ['MESHOPT_FAST_CORE'])
    cpu = core['cpu']
    before = fs.core_load(cpu)
    data = Path(task['input']).read_bytes()
    if hashlib.sha256(data).hexdigest() != task['input_sha256']:
        raise ValueError('case input changed: ' + task['case'])
    samples = {'rust': [], 'scalar': [], 'simd': []}
    per_pair = []
    outputs = {}
    memory = {}
    repeats = None
    if phase in ('0.1', '0.1.x'):
        import runner as r
        import performance as p
        r.benchmark_cpu = cpu
        path = TARGET / f'fast-{os.getpid()}.input'
        path.write_bytes(data)
        drivers = {k: p.Paired(r, v, path) for k, v in binaries.items()}
        baseline = 'cpp'
        try:
            for i in range(max_pairs):
                telemetry = fs.core_load(cpu)
                for backend in (('rust', 'cpp') if i % 2 == 0 else ('cpp', 'rust')):
                    elapsed = drivers[backend].sample()
                    if not math.isfinite(elapsed) or elapsed <= 0:
                        raise ValueError('invalid timing interval')
                    samples['rust' if backend == 'rust' else 'scalar'].append(elapsed)
                per_pair.append(fs.add_core_delta(telemetry, fs.core_load(cpu)))
                decision = fs.stop(samples['rust'], samples['scalar'], alpha,
                                   maximum=max_pairs)
                if decision['stopped']:
                    break
            for backend, driver in drivers.items():
                raw = driver.finish()
                memory[backend] = struct.unpack('<Q', raw[-8:])[0]
                output, times = r.response(raw[:-8], task['output_count'], driver.count)
                outputs[backend] = hashlib.sha256(output).hexdigest()
                if times != samples['rust' if backend == 'rust' else 'scalar']:
                    raise ValueError('timing stream differs from final response')
        finally:
            for driver in drivers.values():
                driver.close()
            path.unlink(missing_ok=True)
        if outputs['rust'] != outputs['cpp']:
            raise ValueError('benchmark output mismatch: ' + task['case'])
        ratio = statistics.median(a/b for a, b in zip(samples['rust'], samples['scalar']))
        mem_ratio = memory['rust']/memory['cpp'] if memory['cpp'] else (1.0 if memory['rust'] == 0 else math.inf)
    elif phase == '0.3':
        r = module('fast_p03_runner', ROOT / 'parity/p03/runner.py')
        clients = {k: r.Client(k, v) for k, v in binaries.items()}
        try:
            expected = {k: c.run(data) for k, c in clients.items()}
            outputs = {k: hashlib.sha256(v[0]).hexdigest() for k, v in expected.items()}
            if outputs['rust'] != outputs['cpp']:
                raise ValueError('scalar output mismatch')
            memory = {k: v[1][1] for k, v in expected.items()}
            probe = bytearray(data)
            struct.pack_into('<I', probe, 44, 1)
            trial = clients['cpp'].run(probe)[1][0]
            repeats = min(2000000, max(1, math.ceil(.008/max(trial, 1e-9))))
            struct.pack_into('<I', probe, 44, repeats)
            struct.pack_into('<I', probe, 4, struct.unpack_from('<I', probe, 4)[0] | 0x20000)
            for i in range(max_pairs):
                telemetry = fs.core_load(cpu)
                order = ('rust', 'cpp', 'cpp-simd') if i % 2 == 0 else ('cpp-simd', 'cpp', 'rust')
                for backend in order:
                    elapsed = clients[backend].run(probe)[1][0]
                    if not math.isfinite(elapsed) or elapsed <= 0:
                        raise ValueError('invalid timing interval')
                    samples['rust' if backend == 'rust' else 'scalar' if backend == 'cpp' else 'simd'].append(elapsed)
                per_pair.append(fs.add_core_delta(telemetry, fs.core_load(cpu)))
                decision = fs.stop(samples['rust'], samples['scalar'], alpha, maximum=max_pairs)
                if decision['stopped']:
                    break
        finally:
            for client in clients.values():
                client.close()
        ratio = statistics.median(a/b for a, b in zip(samples['rust'], samples['scalar']))
        mem_ratio = memory['rust']/memory['cpp'] if memory['cpp'] else (1.0 if memory['rust'] == 0 else math.inf)
    else:
        m = module('fast_codec_measure', ROOT / 'parity/codec/measure.py')
        drivers = {k: m.Driver(v) for k, v in binaries.items()}
        try:
            check = {k: d.call(data) for k, d in drivers.items()}
            outputs = {k: hashlib.sha256(v[1]).hexdigest() for k, v in check.items()}
            if check['rust'][0] != 0 or check['scalar'][0] != 0 or outputs['rust'] != outputs['scalar']:
                raise ValueError('scalar output mismatch')
            probe = bytearray(data)
            struct.pack_into('<I', probe, 32, 1)
            trial = drivers['simd'].call(probe)[2][0]
            repeats = min(1000000, max(1, math.ceil(.04/max(trial, 1e-9))))
            struct.pack_into('<I', probe, 36, repeats)
            for i in range(max_pairs):
                telemetry = fs.core_load(cpu)
                order = ('rust', 'scalar', 'simd') if i % 2 == 0 else ('simd', 'scalar', 'rust')
                for backend in order:
                    status, output, timings = drivers[backend].call(probe)
                    if status != 0 or hashlib.sha256(output).hexdigest() != outputs[backend]:
                        raise ValueError('timed output changed')
                    elapsed = timings[0]
                    if not math.isfinite(elapsed) or elapsed <= 0:
                        raise ValueError('invalid timing interval')
                    samples[backend].append(elapsed)
                per_pair.append(fs.add_core_delta(telemetry, fs.core_load(cpu)))
                decision = fs.stop(samples['rust'], samples['scalar'],
                                   alpha / 2 if phase == '0.2' else alpha,
                                   paired=False, maximum=max_pairs)
                if phase == '0.2':
                    # The registered throughput floor is a separate, unchanged gate.
                    # Raw scalar timing applies only to vertex/index decoders.
                    decision = fs.codec_stop(decision, samples['rust'], alpha / 2,
                                             task['registered_minimum'], task['decoded_bytes'],
                                             task['raw_scalar'], maximum_pairs=max_pairs)
                if decision['stopped']:
                    break
        finally:
            for driver in drivers.values():
                driver.close()
        ratio = statistics.median(samples['rust'])/statistics.median(samples['scalar'])
        mem_ratio = 1.0
    after = fs.core_load(cpu)
    case_pass = ratio <= BAR_CASE and mem_ratio <= BAR_MEMORY
    if phase == '0.2':
        case_pass = (not task['raw_scalar'] or ratio <= BAR_CASE) and (
            task['registered_minimum'] is None or
            task['decoded_bytes']/statistics.median(samples['rust']) >= task['registered_minimum'])
    return {**task, 'reused': False, 'cpu': cpu, 'core': core,
            'core_load_before': before, 'core_load_after': fs.add_core_delta(before, after),
            'pairs': per_pair, 'samples_seconds': samples, 'repeats': repeats,
            'output_sha256': outputs, 'memory_bytes': memory,
            'ratio': ratio, 'memory_ratio': mem_ratio, 'sequential': decision,
            'case_pass': case_pass,
            'finished_unix': time.time()}


def family_verdict(rows, phase):
    ratios = [r['ratio'] for r in rows if phase != '0.2' or r['raw_scalar']]
    gm = math.exp(statistics.mean(math.log(x) for x in ratios))
    maximum = max(ratios)
    memory = max(r['memory_ratio'] for r in rows)
    result = {'cases': len(ratios) if phase == '0.2' else len(rows),
              'geometric_mean': gm, 'maximum': maximum,
              'maximum_memory_ratio': memory,
              'pass': gm <= BAR_GM and maximum <= BAR_CASE and memory <= BAR_MEMORY}
    if phase == '0.2':
        minimum_fail = [r['case'] for r in rows if r['registered_minimum'] is not None
                        and r['decoded_bytes']/statistics.median(r['samples_seconds']['rust']) < r['registered_minimum']]
        result['failed_registered_minima'] = minimum_fail
        result['registered_cases'] = sum(r['registered_minimum'] is not None for r in rows)
        result['pass'] &= not minimum_fail
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--phase', choices=['0.1', '0.1.x', '0.2', '0.3', '0.4'], required=True)
    parser.add_argument('--profile', choices=['crate', 'moss', 'default', 'consumer', 'release-defaults'], required=True)
    parser.add_argument('--workers', type=int)
    parser.add_argument('--full', action='store_true', help='disable change-scoped reuse')
    parser.add_argument('--smoke', action='store_true', help='small iteration subset; never qualification')
    parser.add_argument('--prior', type=Path, action='append', default=[])
    parser.add_argument('--resume', type=Path, help='resume completed cases from an identical-source partial run')
    parser.add_argument('--max-pairs', type=int, default=80)
    args = parser.parse_args()
    if args.workers is not None and args.workers < 1 or args.max_pairs < 20 or args.max_pairs > 90:
        parser.error('workers must be positive and max-pairs must be 20..90')
    if args.phase == '0.1' and args.profile not in ('crate', 'moss', 'default') or args.phase in ('0.1.x', '0.4') and args.profile not in ('moss', 'default') or args.phase == '0.3' and args.profile not in ('consumer', 'release-defaults') or args.phase == '0.2' and args.profile != 'default':
        parser.error('invalid phase/profile combination')
    require_paths()
    admission = timing_admission()
    if args.phase == '0.1.x' and not (ROOT / 'parity/p01x.py').is_file():
        parser.error('0.1.x requires a source export containing the final preprocessing harness')
    if args.phase == '0.2':
        registered_minima()
    started = time.time()
    ART.mkdir(parents=True, exist_ok=True)
    directory = ART / f'fast-{args.phase}-{args.profile}-{int(started)}'
    directory.mkdir()
    binaries, identity = setup(args.phase, args.profile, directory)
    inputs = ART / 'case-inputs' / args.phase
    if args.phase == '0.1':
        cases = cases01(inputs, binaries, args.smoke)
    elif args.phase == '0.1.x':
        cases = cases01x(inputs, args.smoke)
    elif args.phase == '0.2':
        cases = cases02(inputs, binaries, args.smoke)
    elif args.phase == '0.3':
        cases = cases03(inputs, args.smoke)
    else:
        cases = cases04(inputs, args.smoke)
    families = sorted({c['family'] for c in cases})
    fingerprints = {f: fingerprint(args.phase, args.profile, identity, f) for f in families}
    receipt = validation_receipt()
    if receipt and receipt['maximum_pairs'] != args.max_pairs:
        receipt = None
    reused = []
    reused_families = set()
    pending = cases[:]
    resumed = []
    resume_origin = None
    resume_wall = 0.0
    if args.resume:
        old = json.loads(args.resume.read_text())
        if (old.get('schema') != 'meshopt-fast-qualify/1' or old.get('complete') or
                old.get('phase') != args.phase or old.get('profile') != args.profile or
                old.get('smoke') != args.smoke or old.get('full') != args.full or
                old.get('fingerprints') != fingerprints or
                old.get('binary_sha256') != {k: sha(v) for k, v in binaries.items()} or
                old.get('sequential_error', {}).get('per_case_alpha') != .05 / max(1, len(cases)) or
                old.get('sequential_error', {}).get('maximum_pairs') != args.max_pairs):
            parser.error('partial run does not match current source, binaries, matrix mode and stopping rule')
        current = {c['case']: c for c in cases}
        names = set()
        resume_origin = {'path': str(args.resume.resolve()), 'sha256': sha(args.resume)}
        for row in old['rows']:
            task = current.get(row['case'])
            if (task is None or row['case'] in names or row.get('reused') or
                    task['input_sha256'] != row['input_sha256'] or not row['sequential']['stopped']):
                parser.error('invalid completed case in partial run')
            names.add(row['case'])
            resumed.append({**row, 'input': task['input'], 'resumed_from': resume_origin})
        pending = [c for c in pending if c['case'] not in names]
        resume_wall = old.get('prior_active_wall_seconds', 0.0) + max(
            0.0, old.get('checkpoint_unix', old.get('failed_unix', old['started_unix'])) - old['started_unix'])
    if not args.full and not args.smoke and not args.resume:
        for previous in args.prior:
            old = json.loads(previous.read_text())
            qualified_old = prior_is_qualified(previous, old, receipt)
            if (old.get('schema') != 'meshopt-fast-qualify/1' or not old.get('complete') or
                    not qualified_old or old.get('smoke')):
                continue
            for family in families:
                if family in reused_families:
                    continue
                if old.get('fingerprints', {}).get(family) != fingerprints[family]:
                    continue
                current = {c['case']: c['input_sha256'] for c in cases if c['family'] == family}
                current_cases = {c['case']: c for c in cases if c['family'] == family}
                source = [r for r in old['rows'] if r['family'] == family]
                if {r['case']: r['input_sha256'] for r in source} != current:
                    continue
                origin = {'path': str(previous.resolve()), 'sha256': sha(previous)}
                reused += [{**row, 'input': current_cases[row['case']]['input'],
                            'reused': True, 'reused_from': origin} for row in source]
                reused_families.add(family)
                pending = [c for c in pending if c['family'] != family]
    selection = admitted_cores(requested=args.workers)
    cores = selection['selected']
    measurement_admission = timing_admission()
    alpha = .05 / max(1, len(cases))
    record = {'schema': 'meshopt-fast-qualify/1', 'phase': args.phase, 'profile': args.profile,
              'source_root': str(ROOT),
              'harness_sha256': sha(ROOT / 'parity/fast_qualify.py'),
              'statistics_sha256': sha(ROOT / 'parity/fast_stats.py'),
              'started_unix': started, 'smoke': args.smoke, 'full': args.full,
              'qualification_attempt': not args.smoke, 'release_grade': False,
              'verdict_status': 'not a qualification' if args.smoke else 'provisional until validation acceptance',
              'bar': {'family_geometric_mean': BAR_GM, 'case_maximum': BAR_CASE, 'memory': BAR_MEMORY},
              'sequential_error': {'method': 'anytime binomial order-statistic median interval',
                                   'familywise_alpha': .05, 'per_case_alpha': alpha,
                                   'maximum_pairs': args.max_pairs},
              'selection': selection, 'identity': identity, 'binary_sha256': {k: sha(v) for k, v in binaries.items()},
              'timing_admission': admission, 'measurement_admission': measurement_admission,
              'validation_receipt_sha256': sha(ART / 'validation-receipt.json') if receipt else None,
              'fingerprints': fingerprints, 'rows': reused + resumed, 'complete': False,
              'resume_origin': resume_origin, 'prior_active_wall_seconds': resume_wall,
              'checkpoint_unix': time.time()}
    write(directory / 'record.partial.json', record)
    try:
        with ProcessPoolExecutor(max_workers=len(cores), initializer=init_worker,
                                 initargs=(cores,)) as pool:
            futures = [pool.submit(measure_task, args.phase, c, binaries, alpha, args.max_pairs) for c in pending]
            for future in as_completed(futures):
                try:
                    row = future.result()
                except BaseException:
                    for outstanding in futures:
                        outstanding.cancel()
                    raise
                record['rows'].append(row)
                record['checkpoint_unix'] = time.time()
                write(directory / 'record.partial.json', record)
                print(f"{row['case']}: {row['ratio']:.3f}, {row['sequential']['pairs']} pairs on CPU {row['cpu']}", flush=True)
        record['rows'].sort(key=lambda row: row['case'])
        record['families'] = {f: family_verdict([r for r in record['rows'] if r['family'] == f], args.phase) for f in families}
        record['complete'] = len(record['rows']) == len(cases) and all(sha(c['input']) == c['input_sha256'] for c in cases)
        record['source_unchanged'] = (source_unchanged(args.phase, identity) and
                                      all(fingerprint(args.phase, args.profile, identity, f) == fingerprints[f] for f in families))
        record['binary_unchanged'] = {k: sha(v) for k, v in binaries.items()} == record['binary_sha256']
        record['passed'] = record['complete'] and record['source_unchanged'] and record['binary_unchanged'] and all(v['pass'] for v in record['families'].values())
        record['release_grade'] = bool(receipt and args.full and record['complete'] and
                                       record['source_unchanged'] and record['binary_unchanged'] and
                                       not args.smoke)
        record['verdict_status'] = ('not a qualification' if args.smoke else
                                    'validated method' if record['release_grade'] else
                                    'provisional until validation acceptance')
        record['finished_unix'] = time.time()
        record['wall_seconds'] = resume_wall + record['finished_unix'] - started
        write(directory / 'record.json', record)
        (directory / 'record.partial.json').unlink()
        label = 'SMOKE (NOT A QUALIFICATION)' if args.smoke else 'FAST'
        print(f"{label} {args.phase}/{args.profile}: {len(cases)} cases, {record['wall_seconds']:.1f}s, PASS={record['passed']}", flush=True)
    except BaseException:
        record['failed_unix'] = time.time()
        record['checkpoint_unix'] = record['failed_unix']
        write(directory / 'record.partial.json', record)
        raise


if __name__ == '__main__':
    main()
