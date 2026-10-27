"""Audit retained final evidence without compiling or measuring new work."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib
import io
import json
import os
import shutil
import tarfile
import subprocess

ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-0212-simplify'))
LANE = ROOT / 'parity/0212-simplify'
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

results = json.loads((ART / 'results.json').read_text())
assert not results['regression_fail']
source = json.loads((ART / 'candidate-source.json').read_text())
for filename, expected in source.items():
    assert sha(filename) == expected, filename
entry = json.loads((ART / 'ENTRY.json').read_text())
archive = subprocess.check_output(['git', '-C', str(ROOT), 'archive', entry['baseline_commit']])
with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
    for item in tar.getmembers():
        if item.isfile():
            data = tar.extractfile(item).read()
            assert (ART / 'base' / item.name).read_bytes() == data, item.name
            if item.name.startswith('src/'):
                assert (ART / 'proof-base' / item.name).read_bytes() == data, item.name
assert (ART / 'proof-base/Cargo.toml').read_text() == (ART / 'base/Cargo.toml').read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1)
for arm in ['base', 'candidate', 'M7']:
    build = json.loads((ART / f'build-{arm}.json').read_text())
    assert build['exit'] == 0 and sha(ART / f'driver-{arm}') == build['sha256']
    assert sha(ART / f'harness-{arm}/src/main.rs') == entry['attribution_driver_sha256']
# The new scalar M7 executable is bit-identical to the attribution executable;
# independently verify every registry source file against that retained binding.
old = json.loads((ART / 'attribution-binding.json').read_text())
m7 = Path(_public_path('~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/meshopt-0.7.0'))
for relative, expected in old['M7_source'].items():
    assert sha(m7 / relative) == expected, relative
assert sha(ART / 'driver-M7') == old['binary']['M7']
rows = [json.loads(line) for line in (ART / 'counts.jsonl').read_text().splitlines()]
assert len(rows) == 432
for row in rows:
    prefix = ART / 'samples' / f"{row['case']}-{row['operation']}-{row['arm']}-{row['rep']}"
    data = ART / 'outputs' / f"{row['case']}-{row['operation']}-{row['arm']}-{row['rep']}.bin"
    assert sha(data) == row['output_sha256'] and sha(row['input']) == row['input_sha256']
    assert sha(ART / f"driver-{row['arm']}") == row['binary_sha256']
    perf = next(line for line in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in line).split(';')
    assert int(perf[0]) == row['instructions'] and float(perf[4]) == 100
    assert row['exit'] == 0 and row['validation']
    assert prefix.with_suffix('.stdout').read_text() == 'READY\nCOUNTED\n'
    if row['arm'] == 'candidate':
        assert data.read_bytes() == (ART / 'outputs' / f"{row['case']}-{row['operation']}-base-0.bin").read_bytes()
for filename, expected in entry['inputs'].items():
    assert sha(filename) == expected
for stage in json.loads((ART / 'stages.json').read_text()):
    for field in ['vertex_blob', 'index_blob']:
        assert sha(stage[field]) == Path(stage[field]).stem
fuzz = json.loads((ART / 'fuzz.json').read_text())
assert fuzz[-1]['cpu_seconds'] >= 300 and all(row['exit'] == 0 for row in fuzz)
observations = {}
for mode in ['std', 'no-std']:
    text = (ART / f'gate-exact-{mode}.log').read_text().strip()
    assert text.startswith('PASS observations=')
    observations[mode] = int(text.split('=')[1])
fuzz_observations = 0
for row in fuzz:
    log = (ART / f"fuzz-{row['seed']}.log").read_text().strip()
    assert log.startswith(f"seed={row['seed']} observations=")
    fuzz_observations += int(log.split('observations=')[1])
assert '1 passed' in (ART / 'gate-sqrt.log').read_text()
assert len(json.loads((ART / 'cpp-differential.json').read_text())) == 356
selected = ['ENTRY.json', 'candidate-source.json', 'tool-versions.json', 'gate-exits.tsv', 'results.json', 'counts.jsonl', 'measurement-exit.json', 'fuzz.json', 'cpp-differential.json', 'attribution-binding.json', 'attribution-stage-blobs.json']
selected += [f'build-{arm}.json' for arm in ['base', 'candidate', 'M7']]
selected += ['exact-std', 'exact-no-std', 'cpp-reference', 'rust-protocol', 'proof/Cargo.toml', 'proof/Cargo.lock']
selected += [str(p.relative_to(ART)) for p in ART.glob('gate-*.log')]
selected += [f'harness-{arm}/{name}' for arm in ['base', 'candidate', 'M7'] for name in ['Cargo.toml', 'Cargo.lock', 'src/main.rs']]
receipt = dict(exit=0, admission=os.environ['PARITY_RUNNER_ACTIVE'], scratch=str(ART),
               baseline_commit=entry['baseline_commit'], candidate_source=source,
               lane_tools={p.name: sha(p) for p in sorted(LANE.iterdir()) if p.suffix in ['.py', '.sh', '.rs']},
               samples=len(rows), exact_observations=observations,
               sqrt_patterns=2**32, cpp_comparisons=356, fuzz_seeds=len(fuzz),
               fuzz_cpu_seconds=fuzz[-1]['cpu_seconds'], fuzz_observations=fuzz_observations,
               m7_source_matches_attribution=True, m7_binary_matches_attribution=True,
               evidence_sha256={name: sha(ART / name) for name in sorted(selected)})
(ART / 'audit.json').write_text(json.dumps(receipt, indent=2) + '\n')
shutil.copy2(ART / 'audit.json', LANE / 'binding.json')
shutil.copy2(ART / 'gate-exits.tsv', LANE / 'proof-exits.tsv')
shutil.copy2(ART / 'results.json', LANE / 'results.json')
with (LANE / 'instructions.tsv').open('w') as stream:
    stream.write('family\tcase\tbase\tcandidate\tM7\tcandidate/base\tcandidate/M7\tverdict\n')
    for cell in results['cells']:
        med = cell['medians']
        stream.write(f"{cell['family']}\t{cell['case']}\t{med['base']}\t{med['candidate']}\t{med['M7']}\t{cell['candidate_base']}\t{cell['candidate_M7']}\t{cell['verdict']}\n")
print(json.dumps({k: v for k, v in receipt.items() if k not in ['candidate_source', 'lane_tools', 'evidence_sha256']}))
