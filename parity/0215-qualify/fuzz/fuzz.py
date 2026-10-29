"""Release fuzz under the owner's rule for the 0.2.15 qualification.

Usage (inside the CPU-only admission runner, PARITY_RUNNER_ACTIVE set):
  fuzz.py build            copy this directory to $QUALIFY_SCRATCH/fuzz and
                           build every target (cargo-fuzz, nightly, -O -a,
                           coverage + trace-compares, no sanitizer)
  fuzz.py seeds            seed corpus from the frozen source corpus
  fuzz.py run TARGET...    run targets concurrently, one process each

Rule: start from the corpus seeds; run each target until its edge coverage
(libFuzzer `cov:`) has not grown for 600 CPU seconds, with at least 600 and at
most 7200 CPU seconds per target; zero crashes. CPU time is the child's
utime+stime from /proc. One JSON receipt per target is appended to
$QUALIFY_SCRATCH/fuzz/receipts.jsonl.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import struct
import subprocess
import sys
import threading
import time

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
S = Path(os.environ['QUALIFY_SCRATCH'])
F = S / 'fuzz'
TARGET_DIR = Path(os.environ['CARGO_TARGET_DIR']) / 'fuzz'
TARGETS = ['simplify', 'simplify_extra', 'overdraw', 'meshlets', 'clod', 'cache', 'remap',
           'sanitize', 'analyze', 'vertex_layout', 'lod_chain']
MIN_CPU, FLAT_CPU, MAX_CPU = 600.0, 600.0, 7200.0
TICK = os.sysconf('SC_CLK_TCK')


def sha(path):
    with open(path, 'rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def binary(name):
    return TARGET_DIR / 'x86_64-unknown-linux-gnu/release' / name


def build():
    copy = F / 'crate'
    if copy.exists():
        shutil.rmtree(copy)
    shutil.copytree(HERE, copy, ignore=shutil.ignore_patterns('fuzz.py', '*.json', '*.jsonl'))
    for link, dest in [('base', S / 'base'), ('candidate', ROOT)]:
        p = F / link
        if not p.exists():
            p.symlink_to(dest)
    env = dict(os.environ, CARGO_NET_OFFLINE='true', RUSTFLAGS='', CARGO_INCREMENTAL='0')
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    subprocess.run(['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(copy / 'Cargo.toml')],
                   check=True, env=env)
    subprocess.run(['cargo', '+nightly', 'fuzz', 'build', '--fuzz-dir', str(copy), '--target-dir', str(TARGET_DIR),
                    '-O', '-a', '--sanitizer', 'none'], check=True, env=env, cwd=ROOT)
    binding = {name: sha(binary(name)) for name in TARGETS}
    binding['sources'] = {str(p.relative_to(copy)): sha(p) for p in sorted(copy.rglob('*'))
                          if p.is_file() and p.suffix in {'.rs', '.toml', '.lock'}}
    (F / 'build-binding.json').write_text(json.dumps(binding, indent=2, sort_keys=True) + '\n')
    print('built', len(TARGETS), 'targets', flush=True)


def read_mesh(path):
    raw = path.read_bytes()
    n, ni = struct.unpack_from('<II', raw)
    p = struct.unpack_from(f'<{n * 3}I', raw, 8)
    ix = struct.unpack_from(f'<{ni}I', raw, 8 + n * 12)
    return p, ix


def seeds():
    corpus = Path(os.environ['QUALIFY_CORPUS'])
    out = F / 'seeds'
    out.mkdir(parents=True, exist_ok=True)
    names = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth',
             'real-214e-simplify', 'real-a082-simplify', 'real-b6fd-simplify']
    count = 0
    for name in names:
        p, ix = read_mesh(corpus / f'{name}.bin')
        for triangles in [4, 16, 64, 256]:
            remap, cropped, verts = {}, [], []
            for v in ix[:triangles * 3]:
                if v not in remap:
                    remap[v] = len(verts)
                    verts.append(p[v * 3:v * 3 + 3])
                cropped.append(remap[v])
            if len(verts) > 384:
                continue
            body = struct.pack('<HH', len(verts), len(cropped))
            body += b''.join(struct.pack('<3I', *v) for v in verts)
            body += struct.pack(f'<{len(cropped)}H', *cropped)
            body += bytes(range(256))
            for k, (mode, scale, layout, family) in enumerate(
                    [(0, 0, 0, 0), (0, 0, 1, 1), (0, 0, 2, 2), (0, 0, 0, 3), (3, 0, 0, 4), (0, 0, 1, 5)]):
                q = bytes([mode, scale, 0, 0, 0, 0, layout, family, 3, 64, 7, 0, 0, 5, 0, 0])
                (out / f'{name}-{triangles}-{k}').write_bytes(q + body)
                count += 1
    print('seeds', count, flush=True)


STATUS = re.compile(r'^#(\d+)\s+(\w+)\s+cov: (\d+) ft: (\d+)')


def cpu_seconds(pid):
    try:
        fields = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
        return (int(fields[11]) + int(fields[12])) / TICK
    except (OSError, IndexError):
        return None


def run_one(spec, seed):
    # TARGET:bounded runs lod_chain with QUALIFY_LOD_BOUNDED set, in its own directory.
    name, _, variant = spec.partition(':')
    env = dict(os.environ, QUALIFY_LOD_BOUNDED='1') if variant == 'bounded' else None
    work = F / 'runs' / (name + ('-' + variant if variant else ''))
    corpus = work / 'corpus'
    crashes = work / 'crashes'
    for d in (corpus, crashes):
        d.mkdir(parents=True, exist_ok=True)
    log = work / 'fuzz.log'
    exe = binary(name)
    cmd = [str(exe), str(corpus), str(F / 'seeds'), '-print_final_stats=1', '-rss_limit_mb=640',
           '-timeout=60', '-max_len=16384', f'-seed={seed}', f'-artifact_prefix={crashes}/']
    start = time.time()
    state = dict(cov=0, ft=0, execs=0, inited=None, last_cov_cpu=0.0, cpu=0.0, reason=None)
    with log.open('w') as lf:
        proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True,
                                errors='replace', env=env)

        def reader():
            for line in proc.stderr:
                lf.write(line)
                m = STATUS.match(line)
                if m:
                    execs, kind, cov, ft = int(m[1]), m[2], int(m[3]), int(m[4])
                    state['execs'] = execs
                    state['ft'] = max(state['ft'], ft)
                    if kind == 'INITED' and state['inited'] is None:
                        state['inited'] = cov
                    if cov > state['cov']:
                        state['cov'] = cov
                        state['last_cov_cpu'] = state['cpu']
        t = threading.Thread(target=reader, daemon=True)
        t.start()
        while proc.poll() is None:
            time.sleep(5)
            cpu = cpu_seconds(proc.pid)
            if cpu is not None:
                state['cpu'] = cpu
            if state['cpu'] >= MAX_CPU:
                state['reason'] = 'max-cpu'
            elif state['cpu'] >= MIN_CPU and state['cpu'] - state['last_cov_cpu'] >= FLAT_CPU:
                state['reason'] = 'coverage-flat'
            if state['reason']:
                proc.send_signal(signal.SIGINT)
                try:
                    proc.wait(timeout=120)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()
                break
        t.join(timeout=60)
    rc = proc.returncode
    found = sorted(p.name for p in crashes.iterdir())
    receipt = dict(target=spec, binary_sha256=sha(exe), seed=seed, exit=rc, stop=state['reason'] or 'exited',
                   cpu_seconds=round(state['cpu'], 1), wall_seconds=round(time.time() - start, 1),
                   cov_initial=state['inited'], cov_final=state['cov'], ft_final=state['ft'],
                   cov_last_growth_cpu=round(state['last_cov_cpu'], 1), executions=state['execs'],
                   corpus_files=len(list(corpus.iterdir())), seeds=len(list((F / 'seeds').iterdir())),
                   crashes=found)
    with (F / 'receipts.jsonl').open('a') as f:
        f.write(json.dumps(receipt) + '\n')
        f.flush()
        os.fsync(f.fileno())
    print(json.dumps(receipt), flush=True)
    return receipt


def run(names):
    threads, results = [], []
    for i, name in enumerate(names):
        assert name.partition(':')[0] in TARGETS, name
        th = threading.Thread(target=lambda n=name, s=20261029 + i: results.append(run_one(n, s)))
        th.start()
        threads.append(th)
    for th in threads:
        th.join()
    bad = [r for r in results if r['crashes'] or r['stop'] not in ('coverage-flat', 'max-cpu')]
    sys.exit(1 if bad or len(results) != len(names) else 0)


if __name__ == '__main__':
    command = sys.argv[1]
    if command == 'build':
        build()
    elif command == 'seeds':
        seeds()
    elif command == 'run':
        run(sys.argv[2:] or TARGETS)
    else:
        sys.exit('unknown command ' + command)
