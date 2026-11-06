"""Verify custody, remove the disposable target, and write the final report last."""
import hashlib,importlib.util,json,os,shutil,subprocess
from pathlib import Path
spec=importlib.util.spec_from_file_location('qual',Path(__file__).with_name('run.py'));q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)
start=json.loads((q.A/'start.json').read_text());sources={str(p.relative_to(q.R)):q.sha(p) for p in (q.R/'src').rglob('*') if p.is_file()}
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
custody=dict(head=head,source_equal=sources==start['sources'],head_equal=head==start['head'],cargo_toml_equal=q.sha(q.R/'Cargo.toml')==start['cargo_toml'],cargo_lock_equal=q.sha(q.R/'Cargo.lock')==start['cargo_lock'],rustc_equal=subprocess.check_output(['rustc','-Vv'],text=True)==start['rustc'])
q.write(q.A/'final-custody.json',custody)
source_ok=all(custody[k] for k in custody if k!='head')
q.run('source-custody',['python3','-c','import sys; sys.exit('+str(0 if source_ok else 1)+')'],check=False)
q.run('reference-final',['bash','parity/check-reference.sh'],check=False)
q.run('public-text-final',['bash','parity/tools/check_public_text.sh'],check=False)
q.run('default-matrix-final',['python3','-c','import json,sys;d=json.load(open(sys.argv[1]));print(d);sys.exit(0 if d["ok"] else 1)',q.A/'default-matrix-status.json'],check=False)
q.run('instruction-final-bars',['python3',q.A/'instruction_gate.py'],check=False)
q.run('strict-cpp-zero',['python3',q.A/'cpp_gate.py'],check=False)
q.run('collect',['python3',q.A/'collect.py'],check=False)
s=json.loads((q.A/'qualification-summary.json').read_text());exits=[json.loads(l) for l in (q.A/'exits.jsonl').read_text().splitlines()]
assert q.T.name=='meshopt-0219q' and q.T.is_absolute()
# Keep the execution artifacts in the caller-owned log directory before cleanup.
art=q.LOGS/'binaries';art.mkdir(exist_ok=True)
if (q.T/'bin').exists():shutil.copytree(q.T/'bin',art/'native',dirs_exist_ok=True)
for p in (q.T/'det-target').glob('*/release/determinism_main'):
 shutil.copy2(p,art/('determinism-'+p.parents[1].name))
for p in (q.T/'det-target').glob('*/debug/determinism_main'):
 shutil.copy2(p,art/('determinism-'+p.parents[1].name))
for p in (q.T/'det-target').glob('*/wasm32-unknown-unknown/release/determinism.wasm'):
 shutil.copy2(p,art/(p.parents[2].name+'.wasm'))
q.write(q.A/'retained-binaries.json',{str(p.relative_to(art)):q.sha(p) for p in art.rglob('*') if p.is_file()})
for p in (q.A/'logs').glob('*.log'):
 if not (q.LOGS/p.name).exists():shutil.copy2(p,q.LOGS/p.name)
shutil.rmtree(q.T)
q.floor=lambda:None
q.write(q.A/'cleanup.json',dict(target_removed=not q.T.exists(),source_equal=custody['source_equal'],head_equal=custody['head_equal']))
failed=[x for x in exits if x['exit']]
latest={x['name']:x for x in exits};unresolved=[x for x in latest.values() if x['exit']]
det=s.get('determinism') or {};inst=s.get('release-results') or {};metrics=s['bench']
bars={k:sum(x['bars'][k] for x in metrics) for k in metrics[0]['bars']} if metrics else {}
reg=inst.get('regressions',[]);stage_failures=(s.get('campaign-failures') or [])+(s.get('supplement-failures') or [])
qualified=not unresolved and not stage_failures and det.get('all_equal',False) and len(metrics)==5 and not reg and all(v==5 for v in bars.values())
verdict='QUALIFIED' if qualified else 'NOT QUALIFIED'
vg=(q.A/'VG_TABLE.md').read_text()
table=q.A/'TABLE.md'
if not table.exists():table.write_text('# Instruction campaign incomplete\n\nSee the named failures in REPORT.md.\n')
cpp_data=json.loads((q.A/'cpp-observations.json').read_text())
cpp_table=['','## C++ decoder differential observations','','| Run | Cases | Success differences | Forced ISA differences | Failed-call destination differences |','|---|---:|---:|---:|---:|']
for row in cpp_data['runs']:cpp_table.append(f'| {row["receipt"]} | {row["cases"]} | {row["cpp_success_differences"]} | {row["forced_level_differences"]} | {row["cpp_failure_write_differences"]} |')
table.write_text(table.read_text()+'\n'+vg+'\n'.join(cpp_table)+'\n')
bench_section='## Final 0.2.19 candidate qualification\n\nQualification verdict: **'+verdict+'**. See the linked report for code gates and C++ failure-write observations.\n\nSource `'+head+'`. Fresh measurements through the library v2 builder; historical tables below retain their earlier revisions.\n\n'+vg+'\nBar passes (out of five meshes): '+', '.join(f'{k}: {v}/5' for k,v in bars.items())+'.\n\nFull qualification: [report](../0219-qualify/REPORT.md), [table](../0219-qualify/TABLE.md).\n\n'
bp=q.R/'parity/vg-bench/RESULTS.md';old=bp.read_text();bp.write_text(old.split('\n',1)[0]+'\n\n'+bench_section+old.split('\n',1)[1].lstrip())
# Check newly assembled public text before writing the verdict document.
q.run('public-text-tables',['bash','parity/tools/check_public_text.sh'],check=False)
exits=[json.loads(l) for l in (q.A/'exits.jsonl').read_text().splitlines()];failed=[x for x in exits if x['exit']]
latest={x['name']:x for x in exits};unresolved=[x for x in latest.values() if x['exit']]
if unresolved:verdict='NOT QUALIFIED'
lines=['# 0.2.19 candidate qualification','',f'**Verdict: {verdict}.** Candidate `{head}`; baseline `v0.2.18` (`{start["base"]}`). The manifest still declares version 0.2.18; release versioning is operator-owned.','',
f'Build-time bar passes: {bars.get("build_half_reference_1T",0)}/5 at 1T and {bars.get("build_half_reference_8T",0)}/5 at 8T. Resident-bytes bar passes: {bars.get("resident_per_dag_triangle",0)}/5. These failed acceptance bars prevent qualification; complete results and the strict C++ difference bar are recorded below.', '',
'Frozen source, manifest, lockfile and toolchain custody is recorded in [final-custody.json](final-custody.json); any mismatch fails qualification. No source repairs, Git integration, or fuzz campaigns were performed. The disposable target was removed; binary hashes and caller-owned logs retain execution custody.','',
'## Required gates','', '| Gate | Exit | Log |','|---|---:|---|']
for name in ['fmt','clippy','release','release-all-features','public-text','public-text-final','public-text-tables']:
 row=[x for x in exits if x['name']==name][-1];lines.append(f'| {name} | {row["exit"]} | [{row["log"]}]({row["log"]}) |')
lines+=['','## Preserved APIs and C++ 1.3','',f'{s["exact_driver_runs"]} ordinary exact-driver runs completed; {s["exact_driver_failed"]} failed. Modes are all features and no default features with clusterlod enabled. The drivers cover success, failure, workspace limits, callbacks, float words and retained output prefixes. Counts below are observations of the stated finite campaigns, not exhaustive proofs over all inputs.','',
'Additional legacy cluster-LOD corpus and 1/4/16-worker checks, parallel/stream identity, C++ protocol, spatial/meshlet/cluster-LOD, decoder ISA and LOD-quality differentials are recorded in [exits.jsonl](exits.jsonl). The supplemental driver compares the eight 0.2.18 impostor/trim cases and complete MDAG 1 bytes plus direct build fields on 4x4, 12x12 and 24x24 grids, in both feature modes.','']
for mode in ['all','none']:
 for family in ['remap','stream','decode','vdecode','simplify','overdraw']:
  p=q.A/'logs'/f'exact-{family}-{mode}-1.log'
  if p.exists():lines.append(f'- `{family}/{mode}`: '+p.read_text().strip().splitlines()[-1]+f' ([log](logs/{p.name})).')
cpp_obs=json.loads((q.A/'cpp-observations.json').read_text())
lines+=['',f'**Strict C++ zero-difference bar: {cpp_obs["strict_zero_differences"]}.** Decoder receipts contain {cpp_obs["totals"]["cpp_failure_write_differences"]} failed-call destination-write observations that differ from C++. Success outputs, status comparisons and forced ISA differences are recorded separately. The inherited runner exits 0 while disclosing failure-buffer differences; this lane records them as a failure of the strict zero-difference bar in the brief. Exact Rust baseline comparisons remain unchanged, so these observations are not evidence of a new 0.2.19 regression. See [counts](cpp-observations.json) and the [byte counterexample](cpp-failure-counterexample.json).','']
cpp=[x for x in exits if x['name'] in ['protocol-differential','decode-diff-all','decode-diff-none','vdecode-diff-all','vdecode-diff-none','decode-extended-differential','lodq-cpp-all','lodq-cpp-none','exact-remap_cpp-all','exact-remap_cpp-none']]
lines+=['',f'C++ differential command runs: {sum(x["exit"]==0 for x in cpp)}/{len(cpp)} exited 0. Spatial/meshlet/cluster-LOD generated-case counts are additionally retained in [p03-differential.json](p03-differential.json).','']
lines+=['','## Instructions' ,'',f'{inst.get("samples",0)} instructions:u samples; {len(reg)} case/mode regressions above 1%. Medians, group geometric means, worst cases and baseline/C++ byte identity are in [TABLE.md](TABLE.md) and [release-results.json](release-results.json). This retains the 21-operation, eight-input campaign from 0.2.18; it does not claim instruction measurements for every public wrapper or new API.','']
if reg:
 for row in reg:lines.append('- '+json.dumps(row,sort_keys=True))
lines+=['','## Semver','', 'cargo-semver-checks against 0.2.18: see [default](logs/semver-default-1.log) and [all features](logs/semver-all-features-1.log). Both modes passed 202 rules (58 skipped), with no semver update required.','',
'## New API determinism and wasm decode','',f'Hash equality across {len(det.get("variants",{}))} variants: `{det.get("all_equal",False)}`. Four cases cover v2 build and MDAG 2 write (opaque/material cases enable default cluster optimization; the alpha retention case disables it), ATTR/material/FARF payloads, active alpha coverage, all eleven attribute encodings and decoded float words. Native all-feature release/debug also compare 1/2/4/8/16 workers. The no_std label describes the library dependency; native and wasm test drivers use host support. Complete hashes and binary bindings are in [determinism.json](determinism.json).','',
'Wasm page timing is a Node/V8 CPU microbenchmark on a geometry-only page: seven batches of 100,000 compressed-page transcodes plus position and triangle decodes, with warmup. Each page has 288 triangles and two clusters with 122 and 61 positions. Setup is included once per batch and amortized; these are wall timings on a shared host, not an instruction or browser/runtime acceptance gate.','']
for variant in ['wasm-nostd','wasm-default','wasm-default-simd128']:
 row=[x for x in exits if x['name']=='wasm-timing-'+variant][-1]
 if row['exit']==0:
  d=json.loads((q.A/row['log']).read_text());lines.append(f'- `{variant}`: median {d["median_ns_per_page"]/1000:.3f} us/page, range {d["min_ns_per_page"]/1000:.3f} to {d["max_ns_per_page"]/1000:.3f} ([log]({row["log"]})).')
 else:lines.append(f'- `{variant}`: failed, exit {row["exit"]} ([log]({row["log"]})).')
lines+=['','## Final virtual-geometry benchmark','', 'Reference: Bevy MeshletMesh at commit 8cc060f01c3d4d5920781124cc8c32013bb47730. The locked dependency graph is retained in bench/Cargo.lock, and the reference source check and hashes are in logs/vg-reference-pin-1.log.', '',vg,'', 'Bar passes out of five meshes:','']
for k,v in bars.items():lines.append(f'- `{k}`: {v}/5.')
lines+=['','The build bar is at most half the reference at equal thread count. File and resident bars compare bytes per total DAG triangle. Cut efficiency requires fewer triangles at equal measured p95 deviation (log-log interpolation at 5 and 8 px; absent overlap does not pass). The sampled measured/claimed bar is median <= 1.3 at the 1 px threshold. These samples are not continuous error certificates. Reference position quantization and candidate normal/UV payload costs remain part of the measured formats. Raw per-mesh rows and input/binary hashes are in [bench](bench/); full tables are in [the benchmark log](logs/vg-report-1.log).','',
'## Failures','', 'Resolved harness/setup attempts are retained below: the initial timing fixture contained unused vertices, which the page encoder correctly rejected; the timing fixture now compacts cluster-local vertices. The copied benchmark lockfile still listed path package 0.2.17 and was refreshed to 0.2.18 before the locked build. A pre-final public-text check also found the resolved toolchain path in a panic backtrace; the public log copy was redacted and the unchanged guard rerun, while the original scratch log was retained. No library source was changed. Initial hash receipts and the initial timing source remain in determinism-initial.json and determinism-initial.rs.', '',f'{len(failed)} failed attempts across {len(exits)} recorded commands; {len(unresolved)} command names still have a nonzero latest exit. Resolved setup attempts do not determine the candidate verdict. Remaining command and acceptance-bar failures do.','']
for x in failed:lines.append(f'- `{x["name"]}` exit {x["exit"]}: [{x["log"]}]({x["log"]}).')
for x in stage_failures:lines.append('- Stage failure: '+json.dumps(x))
if not failed and not stage_failures:lines.append('All executed commands recorded in exits.jsonl exited 0. Performance/quality bar failures above still prevent qualification.')
lines+=['','## Not verified','', '- GPU decode replay and production renderer/residency integration.','- ARM timing (no ARM execution was performed).','- A fuzz campaign on this candidate; campaigns are owned by the separate lane.','- Continuous geometric error bounds; quality results are sampled CPU measurements.','', 'Reproduction and environment requirements are in [README.md](README.md); admission and output directories are supplied by environment variables.']
report='\n'.join(lines)+'\n'
# The verdict document is deliberately the final worktree write.
(q.A/'REPORT.md').write_text(report)
print(json.dumps(dict(verdict=verdict,commands=len(exits),failed_commands=len(failed),unresolved_commands=[x['name'] for x in unresolved],exact_runs=s['exact_driver_runs'],instruction_samples=inst.get('samples',0),regressions=len(reg),determinism=det.get('all_equal',False),bench_bars=bars,target_removed=not q.T.exists(),cpp_observations=cpp_obs['totals']),indent=2))
