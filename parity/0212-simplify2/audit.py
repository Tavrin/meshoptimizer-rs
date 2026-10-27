"""Independently bind the final source, proof logs and all four-arm receipts."""
from pathlib import Path
import hashlib
import io
import json
import os
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
LANE = ROOT / 'parity/0212-simplify2'
ARMS = ['base', 'post39', 'candidate', 'M7']

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
proof_source = json.loads((ART/'proof-source.json').read_text())
for name, expected in proof_source.items():
    assert sha(name) == expected, ('changed after proof start', name)
source = json.loads((ART / 'candidate-source.json').read_text())
for name, expected in source.items():
    assert sha(name) == expected, name
entry = json.loads((ART / 'ENTRY.json').read_text())
for arm, commit in [('base', entry['baseline_commit']), ('post39', entry['post39_commit'])]:
    archive = subprocess.check_output(['git', '-C', str(ROOT), 'archive', commit])
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        for item in tar.getmembers():
            if item.isfile():
                data = tar.extractfile(item).read()
                assert (ART / arm / item.name).read_bytes() == data, (arm, item.name)
                if arm == 'base' and item.name.startswith('src/'):
                    assert (ART / 'proof-base' / item.name).read_bytes() == data
assert (ART/'proof-base/Cargo.toml').read_text() == (ART/'base/Cargo.toml').read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1)
for arm in ARMS:
    build = json.loads((ART / f'build-{arm}.json').read_text())
    assert build['exit'] == 0 and sha(ART / f'driver-{arm}') == build['sha256']
    assert sha(ART/f'harness-{arm}/src/main.rs') == entry['attribution_driver_sha256']
old = json.loads((ART/'attribution-binding.json').read_text())
m7 = Path(os.environ['MESHOPT_M7_SOURCE'])
for relative, expected in old['M7_source'].items():
    assert sha(m7/relative) == expected
assert sha(ART/'driver-M7') == old['binary']['M7']
rows = [json.loads(line) for line in (ART/'counts.jsonl').read_text().splitlines()]
assert len(rows) == 576
for row in rows:
    stem = f"{row['case']}-{row['operation']}-{row['arm']}-{row['rep']}"
    prefix = ART/'samples'/stem
    output = ART/'outputs'/f'{stem}.bin'
    assert sha(output) == row['output_sha256'] and sha(row['input']) == row['input_sha256']
    assert sha(ART/f"driver-{row['arm']}") == row['binary_sha256']
    fields = next(l for l in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in l).split(';')
    assert int(fields[0]) == row['instructions'] and float(fields[4]) == 100
    assert row['exit'] == 0 and row['validation']
    assert prefix.with_suffix('.stdout').read_text() == 'READY\nCOUNTED\n'
    if row['arm'] == 'candidate':
        for arm in ['base','post39']:
            assert output.read_bytes() == (ART/'outputs'/f"{row['case']}-{row['operation']}-{arm}-0.bin").read_bytes()
for filename, expected in entry['inputs'].items():
    assert sha(filename) == expected
for stage in json.loads((ART/'stages.json').read_text()):
    for field in ['vertex_blob','index_blob']:
        assert sha(stage[field]) == Path(stage[field]).stem
exits = dict(line.split('\t') for line in (ART/'gate-exits.tsv').read_text().splitlines())
required = {'test-all','test-no-default','clippy','fmt','boundary','exact-build-std','exact-build-no-std','exact-std','exact-no-std','cpp','fuzz'}
assert required <= exits.keys() and all(v == '0' for v in exits.values())
observations = {}
for mode in ['std','no-std']:
    text = (ART/f'gate-exact-{mode}.log').read_text().strip()
    assert text.startswith('PASS observations=')
    observations[mode] = int(text.split('=')[1])
fuzz = json.loads((ART/'fuzz.json').read_text())
assert fuzz[-1]['cpu_seconds'] >= 300 and all(r['exit'] == 0 for r in fuzz)
for row in fuzz:
    assert (ART/f"fuzz-{row['seed']}.log").read_text().startswith(f"seed={row['seed']} observations=")
cpp = len(json.loads((ART/'cpp-differential.json').read_text()))
assert cpp == 356
results = json.loads((ART/'results.json').read_text())
assert not results['regression_fail']
files = ['tool-syntax-repair.txt','attempt1/results.json','attempt1/measurement-exit.json','attempt1/proof-admission-cancel.json','proof-source.json','ENTRY.json','candidate-source.json','tool-versions.json','gate-exits.tsv','counts.jsonl','results.json','measurement-exit.json','fuzz.json','cpp-differential.json','proof/Cargo.toml','proof/Cargo.lock','exact-std','exact-no-std','cpp-reference','rust-protocol','attribution-binding.json','attribution-stage-blobs.json']
files += [str(p.relative_to(ART)) for p in ART.glob('gate-*.log')]
files += [f'{kind}-{a}{suffix}' for a in ARMS for kind,suffix in [('driver',''),('build','.json')]]
files += [f'harness-{a}/{f}' for a in ARMS for f in ['Cargo.toml','Cargo.lock','src/main.rs']]
receipt = dict(exit=0, admission=os.environ['PARITY_RUNNER_ACTIVE'], scratch=str(ART),
               source=source, tools={p.name: sha(p) for p in LANE.iterdir() if p.suffix in ['.py','.rs','.sh']},
               samples=len(rows), observations=observations, normalization_extent_patterns=0x7f800000, cpp_comparisons=cpp,
               fuzz_seeds=len(fuzz), fuzz_cpu_seconds=fuzz[-1]['cpu_seconds'],
               evidence_sha256={name:sha(ART/name) for name in sorted(files)})
(ART/'audit.json').write_text(json.dumps(receipt, indent=2)+'\n')
for src,dst in [('audit.json','binding.json'),('gate-exits.tsv','proof-exits.tsv'),('results.json','results.json')]:
    if src == 'audit.json':
        # Publish only portable bindings; complete receipts stay in scratch.
        public = json.loads((ART/src).read_text())
        replacements = [(str(ROOT), '<repo>'), (str(ART), '<scratch>/meshopt-0212-simplify2')]
        def portable(value):
            if isinstance(value, str):
                for before, after in replacements:
                    value = value.replace(before, after)
                return value
            if isinstance(value, dict):
                return {portable(k): portable(v) for k,v in value.items()}
            if isinstance(value, list):
                return [portable(v) for v in value]
            return value
        (LANE/dst).write_text(json.dumps(portable(public), indent=2)+'\n')
    else:
        shutil.copy2(ART/src, LANE/dst)
with (LANE/'instructions.tsv').open('w') as stream:
    stream.write('family\tcase\tbase\tpost39\tcandidate\tM7\tcandidate/base\tcandidate/post39\tcandidate/M7\tverdict\n')
    for c in results['cells']:
        stream.write('\t'.join(map(str,[c['family'],c['case'],*[c['medians'][a] for a in ARMS],c['candidate_base'],c['candidate_post39'],c['candidate_M7'],c['verdict']]))+'\n')
print('PASS audit', len(rows), 'samples', observations)
