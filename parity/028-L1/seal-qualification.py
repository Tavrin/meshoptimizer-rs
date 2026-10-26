from pathlib import Path
import json,hashlib
A=Path('/mnt/linux-extra/moss-scratch/meshopt-028-L1');R=Path.cwd()
load=lambda p:json.loads((A/p).read_text())
fuzz_source=load('FUZZ-SOURCE.json')
assert all(hashlib.sha256((R/path).read_bytes()).hexdigest()==h for path,h in fuzz_source['source'].items()),'proof source changed'
assert all(hashlib.sha256((A/name).read_bytes()).hexdigest()==h for name,h in fuzz_source['binaries'].items()),'proof image changed'
assert all(x['exit']==0 for x in load('gates.json'))
assert load('exact-proof.json')['exit']==0
assert len(load('cpp-seeded-differential.json'))==6
assert all(x['mismatches']==0 for x in load('cpp-seeded-differential.json'))
assert len(load('rfc-preflight.json'))==50
assert load('fuzz.json')['exit']==0 and load('fuzz.json')['target_cpu_seconds']>=300
assert load('division-fuzz.json')['exit']==0 and load('division-fuzz.json')['cpu_seconds']>=300
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(A/'driver-final-ours')==sha(A/'driver-trial7-ours'),'instruction image identity changed; diagnose before timing'
assert sha(R/'src/meshlet_division.rs')==load('division-fuzz.json')['production_source_sha256']
source={str(p.relative_to(R)):sha(p)for p in [*sorted((R/'src').rglob('*.rs')),*sorted((R/'parity/028-L1').rglob('*.rs')),R/'Cargo.toml',R/'Cargo.lock',R/'parity/028-L1/Cargo.toml',R/'parity/028-L1/Cargo.lock']}
(A/'QUALIFIED.json').write_text(json.dumps({'exit':0,'source':source,'instructions_reused_identical_image':'counts-trial7.json','proof':['gates.json','exact-proof.json','cpp-seeded-differential.json','rfc-preflight.json','fuzz.json','division-fuzz.json']},indent=2)+'\n')
