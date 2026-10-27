import os
"""Independently bind retained sources, binaries, inputs, outputs and counters."""
from pathlib import Path
import hashlib, json, statistics, subprocess
ROOT=Path(__file__).resolve().parents[2]
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
records=[json.loads(s) for s in (A/'final/raw.jsonl').read_text().splitlines()]
assert len(records)==156
cells={}
for r in records:
    assert r['launcher_sha256']==sha(ROOT/'parity/0212-overdraw/measure.py')==sha(A/'final/launcher.py')
    assert r['exit']==0 and r['running_percent']>=99
    assert r['input_sha256']==sha(Path(r['command'][-2]))
    assert r['binary_sha256']==sha(A/f"final-driver-{r['arm']}")
    prefix=A/'final/samples'/f"fair-{r['case']}-{r['operation']}-{r['arm']}-{r['sample']}"
    assert json.loads(prefix.with_suffix('.json').read_text())==r
    line=next(s for s in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in s)
    assert int(line.split(';')[0])==r['instructions']
    assert r['instructions']/r['repetitions']==r['instructions_per_call']
    # Every sample has its own retained output, bound at measurement time.
    out=A/'final/outputs'/f"fair-{r['case']}-{r['operation']}-{r['arm']}-{r['sample']}.bin"
    assert sha(out)==r['output_sha256']
    cells.setdefault((r['case'],r['operation'],r['arm']),[]).append(r)
for (case,op,arm),rs in cells.items():
    assert sorted(r['sample'] for r in rs)==[0,1,2]
    if arm=='candidate':assert {r['output_sha256'] for r in rs}=={r['output_sha256'] for r in cells[case,op,'base']}
source=json.loads((A/'final-source.json').read_text())
assert all(sha(Path(p))==v for p,v in source.items())
gates=dict(line.split('\t') for line in (A/'gate-exits.tsv').read_text().splitlines())
assert len(gates)==len((A/'gate-exits.tsv').read_text().splitlines())
assert all(v=='0' for v in gates.values())
assert {'test-all','test-no-default','clippy','fmt','boundary','exact-build-std','exact-std','exact-build-no-std','exact-no-std','cpp','fuzz'}<=gates.keys()
for arm in ['base','candidate','M7','X']:
    receipt=json.loads((A/f'final-build-{arm}.json').read_text())
    assert receipt['exit']==0 and '--locked' in receipt['command']
    assert receipt['sha256']==sha(A/f'final-driver-{arm}')
    assert all(sha(A/f'harness-{arm}'/p)==v for p,v in receipt['harness_sha256'].items())
for p in (A/'cpp-outputs').glob('*.cpp'):
    assert p.read_bytes()==p.with_suffix('.rust').read_bytes()
files={}
for folder in ['attribution','predicate','block','final','cpp-outputs','proof','proof-base','base','harness-base','harness-candidate','harness-M7','harness-X']:
    for p in (A/folder).rglob('*'):
        if p.is_file() and (folder in ['attribution','predicate','block','final','cpp-outputs','proof'] or p.suffix in ['.rs','.cpp','.h'] or p.name in ['Cargo.toml','Cargo.lock','USING.md']):files[str(p)]=sha(p)
reference=Path(os.environ['MESHOPT_REFERENCE'])
assert subprocess.check_output(['git','-C',str(reference),'rev-parse','HEAD'],text=True).strip()=='4c203430ca565cb59a468a91922c76c208169536'
assert not subprocess.check_output(['git','-C',str(reference),'status','--porcelain'],text=True)
for p in (reference/'src').glob('*'):
    if p.suffix in ['.cpp','.h']:files[str(p)]=sha(p)
for package in ['meshopt-0.7.0','optimesh-1.1.0']:
    dirs=list((Path(os.environ.get('CARGO_HOME', Path.home()/'.cargo'))/'registry/src').glob('*/'+package))
    assert len(dirs)==1
    for p in dirs[0].rglob('*'):
        if p.is_file() and (p.suffix in ['.rs','.cpp','.h'] or p.name in ['Cargo.toml','Cargo.lock']):files[str(p)]=sha(p)
for p in [*A.glob('final-*'),*A.glob('*-sources.json'),*A.glob('*-build-*.json'),*A.glob('*-driver-*'),A/'ENTRY.json',A/'tool-versions.json',A/'runner.sh',A/'env.sh',A/'cpp-reference',A/'rust-protocol',A/'exact-std',A/'exact-no-std',A/'cpp-differential.json',A/'fuzz.jsonl',A/'fuzz-output.log',A/'gate-entry.json']:
    if p.is_file():files[str(p)]=sha(p)
for p in A.glob('gate-*'):
    if p.is_file():files[str(p)]=sha(p)
for p in [*ROOT.glob('src/**/*.rs'),*Path(__file__).parent.glob('*'),ROOT/'parity/reference.cpp',ROOT/'parity/runner.py',ROOT/'parity/check-unsafe-boundary.sh',ROOT/'parity/simd/check_boundary.py',ROOT/'parity/simd/check_package.py']:
    if p.is_file() and p.name not in ['binding.json','REPORT.md','results.json','instructions.tsv','proof-exits.tsv','cleanup.json','admissions.json']:files[str(p)]=sha(p)
(A/'binding.json').write_text(json.dumps({'audit_exit':0,'samples':len(records),'files':files},indent=2)+'\n')
print('PASS evidence audit',len(records),'samples',len(files),'files')
