"""Pre-release fuzz driver for meshoptimizer-rs 0.2.16.

Usage (inside the admission wrapper: moss-heavy.sh 4 with MOSS_HEAVY_GPU=0):
  fuzz.py build             build every target group (cargo-fuzz, nightly, -O -a,
                            coverage + trace-compares, no sanitizer)
  (seeds.py SEEDGEN_BIN writes the seed corpora under $FUZZ_SCRATCH/seeds)
  fuzz.py cover TARGET...   `cargo fuzz coverage` on the seeds (own target dir
                            $CARGO_TARGET_DIR/cov); reach.py then tabulates it
  fuzz.py prepare TARGET... build, then cover
  fuzz.py run TARGET...     run targets concurrently (JOBS at a time), one
                            libFuzzer process each, under the rule below

Rule (owner, 2026-10-30): start from the seed corpus; run in 600 s chunks
reusing the target's corpus; stop a target when a whole chunk adds no coverage
after at least 600 CPU-s, cap 7200 CPU-s; zero crashes. CPU time is the child's
utime+stime from /proc. A libFuzzer timeout counts as a failure. One JSON line
per chunk is appended to $FUZZ_SCRATCH/receipts.jsonl; the grown corpora live in
$FUZZ_SCRATCH/corpus/<target>/ and are never deleted.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import threading
import time

assert os.environ.get('PARITY_RUNNER_ACTIVE') or os.environ.get('MOSS_HEAVY_ACTIVE'), 'run inside moss-heavy.sh'
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
S = Path(os.environ['FUZZ_SCRATCH'])
TARGET_DIR = Path(os.environ['CARGO_TARGET_DIR'])
JOBS = int(os.environ.get('FUZZ_JOBS', '5'))
CHUNK, MIN_CPU, MAX_CPU = 600, 600.0, 7200.0
TICK = os.sysconf('SC_CLK_TCK')

DIFF = ['simplify', 'simplify_extra', 'meshlets', 'clod', 'remap', 'lod_chain', 'bounds_diff']
REPAIR = ['weld_vertices', 'orient_faces', 'fill_holes']
CODEC = ['vertex', 'vertex_into', 'simd', 'sequence_stream']
PARALLEL = ['parallel_clod']
GROUPS = {
    'diff': (S / 'fuzz' / 'crate', DIFF),
    'repair': (ROOT / 'parity/0216-repair/fuzz', REPAIR),
    'codec': (ROOT / 'fuzz/codec', CODEC),
    'parallel': (ROOT / 'fuzz/parallel-clod', PARALLEL),
}
TARGETS = DIFF + REPAIR + CODEC + PARALLEL

def sha(path):
    with open(path, 'rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def group_of(name):
    return next(g for g, (_, names) in GROUPS.items() if name in names)


def binary(name):
    return TARGET_DIR / 'x86_64-unknown-linux-gnu/release' / name


def env():
    e = dict(os.environ, CARGO_NET_OFFLINE='true', RUSTFLAGS='', CARGO_INCREMENTAL='0')
    e.pop('CARGO_ENCODED_RUSTFLAGS', None)
    return e


def prepare_diff_crate():
    copy = S / 'fuzz' / 'crate'
    if copy.exists():
        shutil.rmtree(copy)
    shutil.copytree(HERE / 'diff', copy)
    for link, dest in [('base', S / 'base'), ('candidate', ROOT)]:
        p = S / 'fuzz' / link
        if not p.exists():
            p.symlink_to(dest)
    subprocess.run(['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(copy / 'Cargo.toml')],
                   check=True, env=env())


def build():
    prepare_diff_crate()
    binding = {}
    for g, (fuzz_dir, names) in GROUPS.items():
        subprocess.run(['cargo', '+nightly', 'fuzz', 'build', '--fuzz-dir', str(fuzz_dir), '--target-dir',
                        str(TARGET_DIR), '-O', '-a', '--sanitizer', 'none'], check=True, env=env(), cwd=ROOT)
        for n in names:
            binding[n] = sha(binary(n))
    binding['commit'] = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, cwd=ROOT).stdout.strip()
    (S / 'build-binding.json').write_text(json.dumps(binding, indent=2, sort_keys=True) + '\n')
    print('built', len(TARGETS), 'targets', flush=True)


STATUS = re.compile(r'^#(\d+)\s+(\w+)\s+cov: (\d+) ft: (\d+)')


def cpu_seconds(pid):
    try:
        f = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
        return (int(f[11]) + int(f[12])) / TICK
    except (OSError, IndexError):
        return None


def prior_chunks(name):
    path = S / 'receipts.jsonl'
    if not path.exists():
        return []
    return [r for r in map(json.loads, path.read_text().splitlines()) if r['target'] == name]


def run_chunk(name, index, seed):
    work = S / 'runs' / name
    corpus = S / 'corpus' / name
    crashes = work / 'crashes'
    for d in (corpus, crashes, work / 'logs'):
        d.mkdir(parents=True, exist_ok=True)
    log = work / 'logs' / f'chunk{index}.log'
    counts = work / f'level-counts-chunk{index}.txt'
    exe = binary(name)
    seeds = S / 'seeds' / name
    cmd = [str(exe), str(corpus), str(seeds), '-print_final_stats=1', '-rss_limit_mb=1536', '-timeout=60',
           '-max_len=16384', f'-seed={seed}', f'-max_total_time={CHUNK}', f'-artifact_prefix={crashes}/']
    child_env = dict(os.environ, MESHOPT_FUZZ_COUNTS=str(counts))
    before = set(p.name for p in crashes.iterdir())
    start = time.time()
    state = dict(cov=0, ft=0, execs=0, inited=None, cpu=0.0, stats='')
    with log.open('w') as lf:
        proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True,
                                errors='replace', env=child_env)

        def reader():
            for line in proc.stderr:
                lf.write(line)
                m = STATUS.match(line)
                if m:
                    state['execs'] = int(m[1])
                    state['ft'] = max(state['ft'], int(m[4]))
                    if m[2] == 'INITED' and state['inited'] is None:
                        state['inited'] = int(m[3])
                    state['cov'] = max(state['cov'], int(m[3]))
        t = threading.Thread(target=reader, daemon=True)
        t.start()
        while proc.poll() is None:
            time.sleep(2)
            cpu = cpu_seconds(proc.pid)
            if cpu is not None:
                state['cpu'] = cpu
        t.join(timeout=60)
    found = sorted(set(p.name for p in crashes.iterdir()) - before)
    rec = dict(target=name, chunk=index, binary_sha256=sha(exe), seed=seed, exit=proc.returncode,
               cpu_seconds=round(state['cpu'], 1), wall_seconds=round(time.time() - start, 1),
               cov_initial=state['inited'], cov_final=state['cov'], ft_final=state['ft'],
               executions=state['execs'], corpus_files=len(list(corpus.iterdir())),
               seed_files=len(list(seeds.iterdir())), crashes=found)
    if counts.exists():
        rec['level_counts_scalar_sse2_ssse3_sse41_neon_wasm'] = counts.read_text().strip()
    return rec


def run_target(name, index):
    seed = 20261030 + index
    chunks = prior_chunks(name)
    ref = max([c['cov_final'] for c in chunks], default=0)
    if chunks:
        # Resume: a target that already satisfied the rule is not run again.
        last, total = chunks[-1], sum(c['cpu_seconds'] for c in chunks)
        if last['exit'] == 0 and not last['crashes']:
            if total >= MAX_CPU:
                return 'max-cpu'
            if total >= MIN_CPU and not last['grew']:
                return 'coverage-flat'
    while True:
        n = len(chunks) + 1
        rec = run_chunk(name, n, seed + 1000 * n)
        init = rec['cov_initial'] or 0
        total = sum(c['cpu_seconds'] for c in chunks) + rec['cpu_seconds']
        grew = rec['cov_final'] > max(ref, init)
        rec['grew'] = grew
        rec['cpu_total'] = round(total, 1)
        chunks.append(rec)
        with (S / 'receipts.jsonl').open('a') as f:
            f.write(json.dumps(rec) + '\n')
            f.flush()
            os.fsync(f.fileno())
        print(json.dumps(rec), flush=True)
        ref = max(ref, rec['cov_final'])
        if rec['exit'] != 0 or rec['crashes']:
            return 'FAIL'
        if total >= MAX_CPU:
            return 'max-cpu'
        if total >= MIN_CPU and not grew:
            return 'coverage-flat'


def run(names):
    queue = list(enumerate(names))
    lock = threading.Lock()
    results = {}

    def worker():
        while True:
            with lock:
                if not queue:
                    return
                i, name = queue.pop(0)
            results[name] = run_target(name, i)
            print('DONE', name, results[name], flush=True)
    threads = [threading.Thread(target=worker) for _ in range(JOBS)]
    for th in threads:
        th.start()
    for th in threads:
        th.join()
    print(json.dumps(results, indent=1), flush=True)
    sys.exit(0 if all(v in ('coverage-flat', 'max-cpu') for v in results.values()) else 1)


def cover(names):
    for name in names:
        fuzz_dir, _ = GROUPS[group_of(name)]
        seeds = S / 'seeds' / name
        # cargo-fuzz writes the instrumented binary to <target-dir>/<triple>/release/<name>,
        # so coverage builds get a target dir of their own.
        cov_dir = TARGET_DIR / 'cov'
        subprocess.run(['cargo', '+nightly', 'fuzz', 'coverage', '--fuzz-dir', str(fuzz_dir), '--target-dir',
                        str(cov_dir), '-O', '-a', name, str(seeds)], check=True, env=env(), cwd=ROOT)
        # The profile lands in <fuzz-dir>/coverage/<name>/coverage.profdata; reach.py reads it.
        print('coverage', name, 'ok', flush=True)


if __name__ == '__main__':
    command = sys.argv[1]
    if command == 'build':
        build()
    elif command == 'prepare':
        build()
        cover(sys.argv[2:] or TARGETS)
    elif command == 'cover':
        cover(sys.argv[2:] or TARGETS)
    elif command == 'run':
        run(sys.argv[2:] or TARGETS)
    else:
        sys.exit('unknown command ' + command)
