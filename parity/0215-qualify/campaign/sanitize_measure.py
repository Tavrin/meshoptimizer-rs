"""Sanitize / analyze_mesh cost disclosure (train only, scalar).

Usage: sanitize_measure.py (inside the CPU-only admission runner).
Environment as measure.py. Builds sanitize-perf.rs against the train without
`simd`, then counts instructions:u for each op on the eight frozen inputs,
median of three, and writes campaign/sanitize-counts.jsonl and
sanitize-table.md (ratio of each op to filter_index_buffer on the same input).
"""
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
here = Path(__file__).resolve().parent
root = here.parents[2]
scratch = Path(os.environ['QUALIFY_SCRATCH'])
art = scratch / 'campaign'
crate = art / 'sanitize-perf'
crate.mkdir(parents=True, exist_ok=True)
(crate / 'Cargo.toml').write_text(f'''[package]
name = "sanitize-perf"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {{ path = "{root}", default-features = false, features = ["std"] }}
[[bin]]
name = "sanitize-perf"
path = "{here / 'sanitize-perf.rs'}"
[profile.release]
codegen-units = 1
lto = "thin"
''')
target = Path(os.environ['CARGO_TARGET_DIR']) / 'sanitize-perf'
subprocess.run(['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(crate / 'Cargo.toml')], check=True)
subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', str(crate / 'Cargo.toml'),
                '--target-dir', str(target)], check=True)
driver = art / 'driver-sanitize'
shutil.copy2(target / 'release/sanitize-perf', driver)
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
ops = ['filter', 'sanitize', 'sanitize-weld', 'sanitize-full', 'analyze', 'analyze-weld']
inputs = json.loads((scratch / 'inputs.json').read_text())
counts = art / 'sanitize-counts.jsonl'
rows = []
for x in inputs:
    path = scratch / 'inputs' / f"{x['case']}.bin"
    with path.open('rb') as f:
        assert hashlib.file_digest(f, 'sha256').hexdigest() == x['sha256']
    for rep in range(3):
        for op in (ops if rep != 1 else list(reversed(ops))):
            n = 1000 if x['case'] == 'tiny-smooth' else 3 if x['case'].startswith('medium') else 1
            prefix = art / f"sanitize-{x['case']}-{op}-{rep}"
            cr, cw = os.pipe()
            ar, aw = os.pipe()
            cmd = [perf, 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix) + '.perf', '-D', '-1',
                   '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', str(cpu), str(driver), op, str(path),
                   str(prefix) + '.out']
            try:
                result = subprocess.run(cmd, input=f'{n}\nstop\n', capture_output=True, text=True, timeout=3600,
                                        env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar), LC_ALL='C'),
                                        pass_fds=(cr, cw, ar, aw))
            finally:
                for fd in [cr, cw, ar, aw]:
                    os.close(fd)
            assert result.returncode == 0 and result.stdout.splitlines() == ['READY', 'COUNTED'], result.stderr[-2000:]
            fields = next(s.split(';') for s in Path(str(prefix) + '.perf').read_text().splitlines()
                          if ';instructions:u;' in s)
            assert float(fields[4]) == 100.
            Path(str(prefix) + '.out').unlink()
            rows.append(dict(case=x['case'], op=op, rep=rep, instructions_per_call=int(fields[0]) / n))
counts.write_text(''.join(json.dumps(r) + '\n' for r in rows))
med = {}
for r in rows:
    med.setdefault((r['case'], r['op']), []).append(r['instructions_per_call'])
med = {k: statistics.median(v) for k, v in med.items()}
cases = [x['case'] for x in inputs]
lines = ['| Input | filter_index_buffer (instructions) | ' + ' | '.join(f'{o} / filter' for o in ops[1:]) + ' |',
         '|---|---:|' + '---:|' * (len(ops) - 1)]
for c in cases:
    lines.append(f"| {c} | {med[(c, 'filter')]:.0f} | " +
                 ' | '.join(f"{med[(c, o)] / med[(c, 'filter')]:.2f}" for o in ops[1:]) + ' |')
lines.append('| geometric mean | | ' + ' | '.join(
    f"{math.exp(sum(math.log(med[(c, o)] / med[(c, 'filter')]) for c in cases) / len(cases)):.2f}"
    for o in ops[1:]) + ' |')
(art / 'sanitize-table.md').write_text('\n'.join(lines) + '\n')
print('\n'.join(lines))
