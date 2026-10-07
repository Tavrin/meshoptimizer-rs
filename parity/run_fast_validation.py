#!/usr/bin/env python3
"""Resumable validation driver. Polls coordinator admission every two minutes."""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

from fast_qualify import ART, ROOT, TARGET, REFERENCE, sha, timing_status, write, require_paths


def wait_admission(wait):
    while True:
        status = timing_status()
        with (ART / 'admission-polls.jsonl').open('a') as log:
            log.write(json.dumps(status) + '\n')
        if status['allowed']:
            return status
        print('Admission blocked: ' + '; '.join(status['blocked']), flush=True)
        if not wait:
            raise SystemExit(75)
        time.sleep(120)


def environment(artifacts):
    env = dict(os.environ, CARGO_TARGET_DIR=str(TARGET), MESHOPT_ARTIFACTS=str(artifacts),
               MESHOPT_REFERENCE=str(REFERENCE), CARGO_NET_OFFLINE='true',
               CARGO_BUILD_JOBS='2', CARGO_INCREMENTAL='0', RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='',
               RUSTFLAGS='', CARGO_ENCODED_RUSTFLAGS='', TMPDIR=str(TARGET / 'tmp'))
    env['MESHOPT_RESULTS'] = str(artifacts / 'records')
    for key in ('MESHOPT_BENCH_CPU', 'MESHOPT_BENCHMARK_CPU'):
        env.pop(key, None)
    for key in list(env):
        if key.startswith('CARGO_PROFILE_'):
            del env[key]
    env['CARGO_PROFILE_DEV_DEBUG'] = '0'
    return env


def sync_harness(root):
    if root.resolve() == ROOT:
        return
    for name in ('fast_qualify.py', 'fast_stats.py', 'fast_validate.py', 'test_fast_stats.py'):
        shutil.copyfile(ROOT / 'parity' / name, root / 'parity' / name)


def run_attempt(label, command, root, artifacts, wait, journal, extra_env=None, allow_bar_failure=False):
    """Persist exit status and wall duration; never mistake a partial file for success."""
    previous = journal.get(label)
    key_command = list(map(str, command))
    if '--resume' in key_command:
        at = key_command.index('--resume')
        del key_command[at:at+2]
    sources = {}
    for base in (root / 'src', root / 'parity', root / 'fuzz'):
        for path in sorted(base.rglob('*')):
            if (path.is_file() and 'results' not in path.parts and
                    path.suffix in ('.rs', '.py', '.cpp', '.h', '.cjs', '.toml', '.lock', '.sh')):
                sources[str(path.relative_to(root))] = sha(path)
    for name in ('Cargo.toml', 'Cargo.lock', 'parity/DECODER_BAR.md'):
        path = root / name
        if path.is_file():
            sources[name] = sha(path)
    argument_files = {}
    for argument in key_command:
        if Path(argument).suffix in ('.json', '.toml', '.py'):
            path = Path(argument) if Path(argument).is_absolute() else root / argument
            if path.is_file():
                argument_files[str(path)] = sha(path)
    key = {'command': key_command, 'root': str(root), 'argument_files_sha256': argument_files,
           'source_digest': hashlib.sha256(json.dumps(sources, sort_keys=True).encode()).hexdigest(),
           'orchestrator_sha256': sha(ROOT / 'parity/run_fast_validation.py'),
           'harness_sha256': sha(ROOT / 'parity/fast_qualify.py'),
           'statistics_sha256': sha(ROOT / 'parity/fast_stats.py')}
    if previous and previous.get('key') == key and previous.get('successful'):
        return previous
    artifacts.mkdir(parents=True, exist_ok=True)
    admission = (wait_admission(wait) if not label.startswith(('python-', 'root-', 'parity-', 'codec-', 'p03-', 'compare'))
                 else {'allowed': True, 'condition': 'untimed validation'})
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / 'tmp').mkdir(exist_ok=True)
    env = environment(artifacts)
    if extra_env:
        env.update(extra_env)
    started = time.time()
    history = []
    if previous and previous.get('key') == key:
        history = previous.get('attempt_history', []) + [
            {name: previous[name] for name in ('started_unix', 'finished_unix', 'exit_code', 'wall_seconds', 'log')
             if name in previous}]
    result = {'key': key, 'command': list(map(str, command)), 'started_unix': started, 'admission': admission,
              'attempt_history': history, 'successful': False,
              'log': str(ART / (label + '-' + str(time.time_ns()) + '.log'))}
    journal[label] = result
    write(ART / 'validation-jobs.json', journal)
    print('Starting ' + label, flush=True)
    with Path(result['log']).open('w') as log:
        process = subprocess.run(list(map(str, command)), cwd=root, env=env,
                                 stdout=log, stderr=subprocess.STDOUT)
    result.update(finished_unix=time.time(), exit_code=process.returncode)
    result['wall_seconds'] = result['finished_unix'] - started
    log_lines = Path(result['log']).read_text().rstrip().splitlines()
    bar_failure = bool(allow_bar_failure and process.returncode == 1 and log_lines and
                       log_lines[-1].startswith('CANDIDATE VERDICT'))
    result['successful'] = process.returncode == 0 or bar_failure
    write(ART / 'validation-jobs.json', journal)
    print(f"Finished {label}: exit {process.returncode}, {result['wall_seconds']:.1f}s", flush=True)
    if not result['successful'] and process.returncode != 75:
        raise SystemExit(f"{label} failed; inspect {result['log']}")
    return result


def run_job(label, command, root, artifacts, wait, journal, extra_env=None, allow_bar_failure=False):
    while True:
        result = run_attempt(label, command, root, artifacts, wait, journal, extra_env, allow_bar_failure)
        if result['successful']:
            return result
        if not wait:
            raise SystemExit(75)
        print(f'{label}: admission changed during setup; retrying after two minutes.', flush=True)
        time.sleep(120)


def run_fast(item, wait, journal, smoke=False, workers=None):
    root = Path(item['source_root'])
    sync_harness(root)
    label = 'noise-' + ('single' if workers == 1 else 'parallel') if smoke else f"fast-{item['revision'].replace('/', '-')}-{item['phase']}-{item['profile']}"
    command = [sys.executable, root / 'parity/fast_qualify.py', '--phase', item['phase'],
               '--profile', item['profile'], '--full']
    if smoke:
        command += ['--smoke']
    if workers is not None:
        command += ['--workers', str(workers)]
    previous = journal.get(label, {})
    # A failed/interrupted worker process retains fully sampled cases. Resume only
    # under the same harness; fast_qualify independently checks every source/input.
    if not previous.get('successful') and previous.get('key', {}).get('harness_sha256') == sha(ROOT / 'parity/fast_qualify.py'):
        partials = []
        for path in ART.glob(f"fast-{item['phase']}-{item['profile']}-*/record.partial.json"):
            record = json.loads(path.read_text())
            if (record.get('started_unix', 0) >= previous.get('started_unix', float('inf')) and
                    record.get('source_root') == str(root) and record.get('smoke') == smoke):
                partials.append(path)
        if partials:
            command += ['--resume', str(max(partials, key=lambda p: p.stat().st_mtime_ns))]
    result = run_job(label, command, root, ART, wait, journal)
    # Recover the record from this job's final line rather than directory mtimes.
    candidates = []
    for path in ART.glob(f"fast-{item['phase']}-{item['profile']}-*/record.json"):
        record = json.loads(path.read_text())
        if (result['started_unix'] <= record['started_unix'] <= result['finished_unix'] and
                record.get('source_root') == str(root) and record.get('smoke') == smoke and
                record.get('complete') and record.get('source_unchanged') and record.get('binary_unchanged') and
                record.get('harness_sha256') == result['key']['harness_sha256'] and
                record.get('statistics_sha256') == result['key']['statistics_sha256']):
            candidates.append(path)
    if len(candidates) != 1:
        raise SystemExit(f'{label}: expected exactly one complete unchanged-source record, got {candidates}')
    return str(candidates[0])


def full03(item, wait, journal):
    root = Path(item['source_root'])
    folder = ART / ('full-03-' + item['profile'])
    # The wrapper checks admission again after build and physical-core selection.
    # Benchmark bodies, samples, bars and recorded historical sources are unchanged.
    code = """
import sys
sys.path.insert(0, 'parity')
from fast_qualify import timing_admission, admitted_cores
sys.path.insert(0, 'parity/p03')
import benchmark
def select_cpu(folder):
    selection = admitted_cores(requested=1)
    timing_admission()
    record = selection['selected'][0]
    benchmark.r.os.sched_setaffinity(0, {record['cpu']})
    (folder / 'cpu.json').write_text(benchmark.json.dumps(selection, indent=2))
    return record
benchmark.select_cpu = select_cpu
sys.argv = ['benchmark.py']
benchmark.main()
"""
    result = run_job('full-03-' + item['profile'], [sys.executable, '-c', code], root,
                     folder, wait, journal, {'MESHOPT_RUST_PROFILE': item['profile']})
    item['full'] = str(folder / ('benchmark-' + item['profile']) / 'record.json')
    record = json.loads(Path(item['full']).read_text())
    if not record.get('complete') or not record.get('source_unchanged'):
        raise SystemExit('incomplete or changed-source full 0.3 run')
    item['full_wall_seconds'] = result['wall_seconds']
    item['full_wall_provenance'] = 'fresh exact-source full subprocess duration including build; validation-jobs.json'


def full02(item, wait, journal):
    root = Path(item['source_root'])
    folder = ART / 'full-02-default'
    folder.mkdir(exist_ok=True)
    shutil.copyfile(item['registered_baseline'], folder / 'baseline.json')
    # Reference existing content-keyed inputs instead of duplicating their bytes.
    from fast_validate import read_record
    old_inputs, _ = read_record(item['full_input_manifest'])
    write(folder / 'benchmark-inputs.json', old_inputs)
    code = """
import sys, os
sys.path.insert(0, 'parity')
from fast_qualify import timing_admission, admitted_cores
sys.path.insert(0, 'parity/codec')
import measure, runner
original = runner.build
def build():
    result = original()
    selection = admitted_cores(requested=1)
    timing_admission()
    os.environ['MESHOPT_BENCH_CPU'] = str(selection['selected'][0]['cpu'])
    return result
runner.build = build
measure.candidate()
"""
    result = run_job('full-02-default', [sys.executable, '-c', code], root, folder,
                     wait, journal, allow_bar_failure=True)
    full = folder / 'candidate.json'
    if not full.is_file():
        raise SystemExit('full 0.2 did not produce a complete candidate record')
    # Compare verdicts with the original historical candidate. The new run supplies
    # only the missing wall duration and remains independently reviewable.
    item['full_wall_seconds'] = result['wall_seconds']
    item['full_wall_provenance'] = 'fresh same-source full subprocess duration including build; original archived verdicts retained'
    item['contemporary_full'] = str(full)


def full01x(item, wait, journal):
    root = Path(item['source_root'])
    folder = ART / ('full-01x-' + item['profile'])
    profile = 'defaults' if item['profile'] == 'default' else 'moss'
    code = """
import sys, os
from types import SimpleNamespace
sys.path.insert(0, 'parity')
from fast_qualify import timing_admission, admitted_cores
import quiet, p01x
def physical_core():
    selection = admitted_cores(requested=1)
    timing_admission()
    return {**selection['selected'][0], 'allowed_cpus': sorted(os.sched_getaffinity(0)),
            'method': 'fastq admission wrapper; least-busy available physical core',
            'selection': selection}
quiet.physical_core = physical_core
p01x.benchmark(SimpleNamespace(consumer_profile=os.environ['FASTQ_FULL_PROFILE'], enforce=False))
"""
    result = run_job('full-01x-' + item['profile'], [sys.executable, '-c', code], root,
                     folder, wait, journal, {'FASTQ_FULL_PROFILE': profile,
                                           'MESHOPT_RESULTS': str(folder / 'records')})
    full = folder / 'records' / ('p01x-benchmark-' + profile + '.json')
    if not full.is_file():
        raise SystemExit('full preprocessing run did not produce a complete record')
    item['full_wall_seconds'] = result['wall_seconds']
    item['full_wall_provenance'] = 'fresh same-source full subprocess duration including build; original archived verdicts retained'
    item['contemporary_full'] = str(full)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('plan', type=Path)
    parser.add_argument('--wait', action='store_true', help='poll every 120 seconds until all four conditions hold')
    parser.add_argument('--workers', type=int)
    args = parser.parse_args()
    require_paths()
    ART.mkdir(parents=True, exist_ok=True)
    driver_lock = (ART / 'validation-driver.lock').open('a+')
    try:
        fcntl.flock(driver_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise SystemExit('A fastq validation driver is already running; inspect validation-driver.lock and validation-jobs.json.')
    driver_lock.seek(0)
    driver_lock.truncate()
    driver_lock.write(json.dumps({'pid': os.getpid(), 'started_unix': time.time(), 'plan': str(args.plan.resolve())}) + '\n')
    driver_lock.flush()
    plan = json.loads(args.plan.read_text())
    journal_path = ART / 'validation-jobs.json'
    journal = json.loads(journal_path.read_text()) if journal_path.exists() else {}
    # Untimed checks need no timing admission; measurement jobs remain gated.
    checks = [('python-tests', [sys.executable, '-m', 'unittest', 'parity.test_fast_stats', 'parity.test_fast_validate'])]
    for label, manifest in [('root', 'Cargo.toml'), ('parity', 'parity/Cargo.toml'),
                            ('codec', 'parity/codec/Cargo.toml'), ('p03', 'parity/p03/Cargo.toml')]:
        checks += [(label + '-fmt', ['cargo', 'fmt', '--manifest-path', manifest, '--check']),
                   (label + '-clippy', ['cargo', 'clippy', '--offline', '--locked', '--manifest-path', manifest,
                                       '--all-targets', '--', '-D', 'warnings']),
                   (label + '-tests', ['cargo', 'test', '--offline', '--locked', '--manifest-path', manifest])]
    for label, command in checks:
        run_job(label, command, ROOT, ART / 'checks', args.wait, journal)
    for item in plan['comparisons']:
        sync_harness(Path(item['source_root']))
        if item['phase'] == '0.2':
            full02(item, args.wait, journal)
        if item['phase'] == '0.1.x':
            full01x(item, args.wait, journal)
        if item['phase'] == '0.3':
            sync_harness(Path(item['source_root']))
            full03(item, args.wait, journal)
        item['fast'] = run_fast(item, args.wait, journal, workers=args.workers)
        write(ART / 'validation-manifest.json', plan)
    item = plan['comparisons'][0]
    single = run_fast(item, args.wait, journal, smoke=True, workers=1)
    parallel = run_fast(item, args.wait, journal, smoke=True, workers=args.workers)
    plan['smoke_noise'] = [{'single': single, 'parallel': parallel}]
    write(ART / 'validation-manifest.json', plan)
    run_job('compare', [sys.executable, ROOT / 'parity/fast_validate.py', ART / 'validation-manifest.json'],
            ROOT, ART, args.wait, journal)
    # This target belongs exclusively to fastq; retained evidence is outside it.
    if TARGET.exists():
        shutil.rmtree(TARGET)
    write(ART / 'cleanup.json', {'removed': str(TARGET), 'finished_unix': time.time()})
    print('Validation complete; temporary fastq build target removed.', flush=True)


if __name__ == '__main__':
    main()
