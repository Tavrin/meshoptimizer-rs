"""Retain normalized text receipts, summarize evidence, remove dedicated target."""
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

lane=Path(__file__).resolve().parent
root=lane.parents[1]
target=Path(os.environ['CARGO_TARGET_DIR']).resolve()
assert target.name=='meshopt-0216-par-clod' and (target/'proof-work').is_dir()
scratch=target/'proof-work'
logs=lane/'logs';logs.mkdir(exist_ok=True)
def normalize(s):
    for path,label in [(scratch,'${SCRATCH}'),(root,'${WORKTREE}'),(target,'${TARGET}'),(Path.home(),'${USER_HOME}')]:
        s=s.replace(str(path),label)
    return s
for source in [*scratch.glob('*.log'),*lane.glob('*.log')]:
    (logs/source.name).write_text(normalize(source.read_text()))
    if source.parent==lane:source.unlink()
# Every final named step must have passed, even when a previous attempt failed.
exits={}
for line in (lane/'exits.tsv').read_text().splitlines():
    name,code=line.split('\t');exits[name]=int(code)
required=['fmt','test-all','test-no-default','test-scalar-parallel','clippy','unsafe-boundary','build-drivers','corpus-export','targeted-simd','lod-corpus-simd','real-corpus-simd','targeted-scalar','lod-corpus-scalar','real-corpus-scalar','measure','test-no-std-clusterlod','fuzz-source-check','prefixes']
assert all(exits.get(name)==0 for name in required),exits
counts={}
for name in ['targeted-simd','lod-corpus-simd','real-corpus-simd','targeted-scalar','lod-corpus-scalar','real-corpus-scalar','prefix-simd','prefix-scalar','exact-no-std']:
    text=(logs/(name+'.log')).read_text()
    matches=re.findall(r'PASS (?:prefix )?comparisons (\d+)',text)
    assert len(matches)==1,(name,matches)
    counts[name]=int(matches[0])
environment=json.loads((lane/'environment.json').read_text())
environment['toolchains']={name:subprocess.check_output(command,text=True).strip() for name,command in [('rustc',['rustc','--version','--verbose']),('cargo',['cargo','--version']),('cxx',['c++','--version'])]}
environment['cpu_model']=next(line.split(':',1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name'))
environment['exclusive_cores']=False
environment['contention_note']='Other admitted CPU work was observed on core 0 during the instruction campaign; wall results are shared-host disclosures.'
(lane/'environment.json').write_text(json.dumps(environment,indent=2)+'\n')
result=json.loads((lane/'results.json').read_text())
prefix=json.loads((lane/'prefix-results.json').read_text())
assert result['output_identity'] and result['samples']==150
text='''# Measurements

Instruction counts are median of three resident-loop samples, `instructions:u`,
including worker threads. All five arms produce identical serialized output on
all eight peer meshes. Wall results are median of five interleaved pinned-core
samples on a shared host; they disclose speed rather than gate shipment.

| Mesh | v0.2.15 (Ginst) | Candidate serial | Candidate 4 workers | C++ scalar 1.3 | C++ default 1.3 |
|---|---:|---:|---:|---:|---:|
'''
for name,cell in result['instructions'].items():
    text+='| '+name+' | '+' | '.join(f"{cell[a]/1e9:.6f}" for a in ['base','serial','candidate','cpp-scalar','cpp-default'])+' |\n'
text+='\n| Real mesh | Single thread (s) | 4 workers (s) | Speedup |\n|---|---:|---:|---:|\n'
for name,cell in result['wall'].items():
    text+=f"| {name} | {cell['serial']:.6f} | {cell['candidate']:.6f} | {cell['speedup']:.3f}x |\n"
text+='\nRaw samples and output hashes: `counts.jsonl`; cores/protocol: `environment.json`.\n'
(lane/'TABLE.md').write_text(text)
summary=dict(comparisons=counts,total_exact_comparisons=sum(counts.values()),cpp_lod_comparisons=prefix['cpp_lod_comparisons'],performance_samples=result['samples'],last_exit_per_step=exits,fuzz_executed=False)
(lane/'proof-results.json').write_text(json.dumps(summary,indent=2)+'\n')
proof=(lane/'PROOFS.md').read_text().split('\n## Final execution result\n')[0]
proof+='\n## Final execution result\n\n'
proof+=f"All {sum(counts.values()):,} exact observation-vector comparisons passed; each targeted vector contains four successive calls. Breakdown:\n\n"
proof+='| Driver | Comparisons |\n|---|---:|\n'
for name,count in counts.items():proof+=f'| {name} | {count:,} |\n'
proof+=f"\nAdditional C++ LOD differential: {prefix['cpp_lod_comparisons']} arm comparisons over {len(prefix['cpp_lod_outputs'])} fixtures, zero differences. The eight peer fixtures include five synthetic and three real meshes; the LOD corpus has 45 fixtures. Full output matrices cover 1/2/4/8/32 workers and both SIMD modes; prefix matrices also cover all worker counts on every fixture. The truly no_std crate additionally passes the exhaustive targeted serial observer.\n\n"
proof+='All final named checks exit 0. Initial test compilation failed with an incorrect test callback return type; the initial no-default doctest run failed due to a missing feature guard on the guide recipe. Both were fixed before the successful final full gates; original failure logs are retained. No fuzz input was executed (the new target was compiled only).\n\n'
proof+='The bounded exhaustive target uses 29 tiny cases (grid side two, invalid configurations and geometry), plus 20 larger config cases on grid side ten. Its work limits are fully enumerated rather than sampled. This remains far below the two CPU-hour boundary-case allocation.\n\n'
proof+='See [TABLE.md](TABLE.md) for instruction and wall-time disclosures. Identity passes: ship the feature as opt-in. The dedicated target is deleted after retaining receipts. Git index, branches, commits and references are unchanged.\n'
(lane/'PROOFS.md').write_text(proof)
# Bind all final source, test, documentation and proof-driver files (not outputs).
paths=[*root.glob('src/clusterlod/*.rs'),root/'Cargo.toml',root/'Cargo.lock',root/'CHANGELOG.md',root/'docs/USING.md',root/'tests/clusterlod_parallel.rs',root/'fuzz/README.md',*root.glob('fuzz/parallel-clod/*'),*lane.glob('*.py'),*lane.glob('*.sh'),lane/'exact.rs']
(lane/'source.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(root))+'\n' for p in sorted(set(paths)) if p.is_file()))
shutil.rmtree(target)
(lane/'cleanup.json').write_text(json.dumps(dict(dedicated_target=target.name,removed=not target.exists(),retained_logs=len(list(logs.glob('*.log')))),indent=2)+'\n')
print(json.dumps(summary,indent=2))
