"""Independently bind source, exact proof and every accepted instruction sample."""
from pathlib import Path
import hashlib
import io
import json
import os
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
LANE = ROOT/'parity/0213-simplify3'
ARMS = ['base','candidate','M7']
def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream,'sha256').hexdigest()
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
for manifest in ['proof-source.json','candidate-source.json']:
    for path,expected in json.loads((ART/manifest).read_text()).items():
        assert sha(path)==expected, ('source changed',path)
entry = json.loads((ART/'ENTRY.json').read_text())
with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git','archive',entry['baseline_commit']]))) as tar:
    for item in tar.getmembers():
        if item.isfile():
            data = tar.extractfile(item).read()
            assert (ART/'base'/item.name).read_bytes()==data
            if item.name.startswith('src/'):
                assert (ART/'proof-base'/item.name).read_bytes()==data
assert (ART/'proof-base/Cargo.toml').read_text()==(ART/'base/Cargo.toml').read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1)
for arm in ARMS:
    build = json.loads((ART/f'build-{arm}.json').read_text())
    assert build['exit']==0 and sha(ART/f'driver-{arm}')==build['sha256']
    assert sha(ART/f'harness-{arm}/src/main.rs')==entry['attribution_driver_sha256']
stamp = json.loads((ART/'ATTRIBUTION-COMPLETE.json').read_text())
assert stamp['library_sha256'] == sha(ART/'base/src/simplify.rs')
assert stamp['report_sha256'] == sha(LANE/'ATTRIBUTION.md')
assert stamp['valid_profiles'] == 36 and stamp['rejected_throttled_profiles'] == 36
profiles = [json.loads(line) for line in (ART/'attribution-valid/records.jsonl').read_text().splitlines()]
assert len(profiles) == 36
for profile in profiles:
    assert profile['binary_sha256'] == sha(ART/f"driver-{profile['arm']}")
    assert profile['sampling_period'] == 1000000
    assert all(profile[k] == 0 for k in ['stat_exit','record_exit','report_exit','lines_exit','annotate_exit'])
    assert not (ART/'attribution-valid'/f"{profile['case']}-{profile['arm']}.sampling-check").read_text().strip()
cpp_compile = (ART/'build-M7.log').read_text()
assert '-DNDEBUG' in cpp_compile and '-DMESHOPTIMIZER_NO_SIMD' in cpp_compile
assert '-ffast-math' not in cpp_compile
old = json.loads((ART/'attribution-binding.json').read_text())
for relative,expected in old['M7_source'].items():
    assert sha(Path(os.environ['MESHOPT_M7_SOURCE'])/relative)==expected
rows = [json.loads(s) for s in (ART/'counts.jsonl').read_text().splitlines()]
assert len(rows)==432
keys = set()
for row in rows:
    key = (row['case'],row['operation'],row['arm'],row['rep'])
    assert key not in keys
    keys.add(key)
    stem = '-'.join(map(str,key))
    prefix = ART/'samples'/stem
    output = ART/'outputs'/f'{stem}.bin'
    assert sha(output)==row['output_sha256'] and sha(row['input'])==row['input_sha256']
    assert sha(ART/f"driver-{row['arm']}")==row['binary_sha256']
    fields = next(s for s in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in s).split(';')
    assert int(fields[0])==row['instructions'] and float(fields[4])==100
    assert row['exit']==0 and row['validation']
    assert prefix.with_suffix('.stdout').read_text()=='READY\nCOUNTED\n'
    if row['arm']=='candidate':
        assert output.read_bytes()==(ART/'outputs'/f"{row['case']}-{row['operation']}-base-0.bin").read_bytes()
for filename,expected in entry['inputs'].items():
    assert sha(filename)==expected
for stage in json.loads((ART/'stages.json').read_text()):
    for field in ['vertex_blob','index_blob']:
        assert sha(stage[field])==Path(stage[field]).stem
exits = dict(s.split('\t') for s in (ART/'gate-exits.tsv').read_text().splitlines())
required = {'test-all','test-no-default','clippy','fmt','boundary','public-text','exact-build-std','exact-build-no-std','exact-std','exact-no-std','cpp','fuzz'}
assert required<=exits.keys() and all(v=='0' for v in exits.values())
observations = {}
for mode in ['std','no-std']:
    text = (ART/f'gate-exact-{mode}.log').read_text().strip()
    assert text.startswith('PASS observations=')
    observations[mode] = int(text.split('=')[1])
fuzz = json.loads((ART/'fuzz.json').read_text())
assert fuzz[-1]['cpu_seconds']>=300 and all(r['exit']==0 for r in fuzz)
for row in fuzz:
    assert (ART/f"fuzz-{row['seed']}.log").read_text().startswith(f"seed={row['seed']} observations=")
cpp = json.loads((ART/'cpp-differential.json').read_text())
assert len(cpp)==356
result = json.loads((ART/'results.json').read_text())
assert not result['regression_fail']
files = ['ATTRIBUTION-COMPLETE.json','CAMPAIGN.json','proof-source.json','preparation-exits.json','ENTRY.json','candidate-source.json','tool-versions.json','gate-exits.tsv','counts.jsonl','results.json','measurement-exit.json','fuzz.json','cpp-differential.json','proof/Cargo.toml','proof/Cargo.lock','exact-std','exact-no-std','cpp-reference','rust-protocol','attribution-binding.json','attribution-stage-blobs.json']
files += [str(p.relative_to(ART)) for p in ART.glob('gate-*.log')]
files += [f'{kind}-{a}{suffix}' for a in ARMS for kind,suffix in [('driver',''),('build','.json')]]
files += [f'harness-{a}/{f}' for a in ARMS for f in ['Cargo.toml','Cargo.lock','src/main.rs']]
receipt = dict(exit=0,admission=os.environ['PARITY_RUNNER_ACTIVE'],
    observations=observations,cpp_comparisons=len(cpp),fuzz_seeds=len(fuzz),fuzz_cpu_seconds=fuzz[-1]['cpu_seconds'],
    source=json.loads((ART/'candidate-source.json').read_text()),
    tools={p.name:sha(p) for p in LANE.iterdir() if p.suffix in ['.py','.rs','.sh']},
    files={f:sha(ART/f) for f in files},samples=len(rows),
    raw={str(p.relative_to(ART)):sha(p) for d in ['samples','outputs','attribution-valid','attribution'] for p in (ART/d).iterdir() if p.is_file()})
(ART/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
# Public binding uses relative source names and portable scratch placeholders.
public = dict(receipt)
public['source'] = {str(Path(p).relative_to(ROOT)):h for p,h in receipt['source'].items()}
(LANE/'binding.json').write_text(json.dumps(public,indent=2)+'\n')
(LANE/'proof-exits.tsv').write_text((ART/'gate-exits.tsv').read_text())
(LANE/'instructions.tsv').write_text('family\tcase\tarm\trep\tinstructions\trepetitions\n'+''.join(f"{r['family']}\t{r['case']}\t{r['arm']}\t{r['rep']}\t{r['instructions']}\t{r['repetitions']}\n" for r in rows))
(LANE/'results.json').write_text((ART/'results.json').read_text())
print('PASS evidence audit',len(rows),'samples')
