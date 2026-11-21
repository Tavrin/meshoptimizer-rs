"""Record evidence and source custody, then delete the disposable target."""
import run as q
import json, os, shutil, subprocess
# A: receipts (QUALIFY_RECEIPTS); H: harness sources; L: logs and exit records (QUALIFY_LOGS).
A=q.OUT;H=q.A;L=q.LOGS
for p in L.rglob('*.log'):
 s=p.read_text(errors='replace');p.write_text(q.clean(s).replace('\x00','').ljust(len(s)))
q.run('cpp-success-and-inherited-failures',['python3',H/'cpp_gate.py'],check=False)
q.run('instruction-and-output-bars',['python3',H/'instruction_gate.py'],check=False)
q.run('collect',['python3',H/'collect.py'],check=False)
start=json.loads((A/'start.json').read_text())
custody=dict(head=os.environ.get('QUALIFY_COMMIT') or q.subprocess.check_output(['git','rev-parse','HEAD'],text=True,cwd=q.HARNESS).strip(),source_equal={str(p.relative_to(q.R)):q.sha(p) for p in (q.R/'src').rglob('*') if p.is_file()}==start['sources'],cargo_toml_equal=q.sha(q.R/'Cargo.toml')==start['cargo_toml'],cargo_lock_equal=q.sha(q.R/'Cargo.lock')==start['cargo_lock'])
custody['head_equal']=custody['head']==start['head'];custody['rustc_equal']=subprocess.check_output(['rustc','-Vv'],text=True)==start['rustc'];q.write(A/'final-custody.json',custody)
q.write(A/'harness-binding.json',{p.name:q.sha(p) for p in H.iterdir() if p.is_file() and p.suffix in ['.py','.rs','.mjs','.txt']})
s=json.loads((A/'qualification-summary.json').read_text());metrics=s['bench'];inst=s.get('release-results') or {};det=s.get('determinism') or {}
exits=[json.loads(l) for l in (L/'exits.jsonl').read_text().splitlines()];latest={x['name']:x for x in exits}
bars={k:sum(m['bars'][k] for m in metrics) for k in metrics[0]['bars']} if metrics else {}
verdict='QUALIFIED' if len(metrics)==5 and all(v==5 for v in bars.values()) and det.get('all_equal') and not inst.get('regressions',[1]) and s['exact_driver_runs']==34 and not s['exact_driver_failed'] and inst.get('samples')==1008 and len(det.get('variants',{}))==8 and not s.get('campaign-failures') and not s.get('supplement-failures') and not any(r['exit'] for r in latest.values()) and all(v for k,v in custody.items() if k!='head') else 'NOT QUALIFIED'
vg=(A/'VG_TABLE.md').read_text();bar_table=['','| Bar | Pass / measured meshes |','|---|---:|']+[f'| {k} | {v}/{len(metrics)} |' for k,v in bars.items()]
bar_table+=['','| Mesh | '+' | '.join(bars)+' |','|---|'+'---|'*len(bars)]
for m in metrics:bar_table.append('| '+m['mesh']+' | '+' | '.join('PASS' if m['bars'][k] else 'FAIL' for k in bars)+' |')
bar_table += ['', 'Build bars require at most half the reference time at equal thread count. File and resident bars require bytes per total DAG triangle no greater than the reference. Cut bars require fewer triangles at equal sampled p95 deviation (5 and 8 px, log-log interpolation without extrapolation). The measured/claimed bar is the sampled median at 1 px, at most 1.3. Every self-check and the 1T/8T byte comparison must pass.']
shading=A/'shading/core.json';shade=[]
if shading.exists():
 shade=['','## Shading weights 0 and 0.25','','16 directions, six projected spans, one-pixel threshold, 16,384 probes in each surface direction. Five interleaved timing repetitions. Sampled CPU results.','','| Mesh | Weight | Mean cut triangles | Normal p95 / max degrees | Geometric ratio median / p95 / max | Build ms 1T / 8T |','|---|---:|---:|---:|---:|---:|']
 for r in json.loads(shading.read_text()):
  qual=r['quality'];tm=r['timing']
  # Preserve actual field names and all values in core.json; display known scalar metrics.
  geo=" / ".join(f"{qual[k]:.4f}" for k in ["geometric_median_ratio","geometric_p95_ratio","geometric_max_ratio"])
  shade.append(f"| {r['mesh']} | {r['weight']} | {qual['mean_triangles']:.2f} | {qual['normal_p95_deg']:.3f} / {qual['normal_max_deg']:.3f} | {geo} | {tm['1']:.2f} / {tm['8']:.2f} |")
else:shade=['','## Shading weights 0 and 0.25','','Not verified: no fresh completed shading campaign.']
not_verified=['','## Not verified','','GPU decode, renderer integration and production runtime acceptance; ARM execution; continuous geometric or shading certificates; fuzz campaigns (excluded by the brief). Finite exact fixtures do not exhaust all possible inputs.']
if s['exact_driver_runs']!=34: not_verified.append('The full 34-run ordinary exact matrix did not complete.')
if inst.get('samples')!=1008: not_verified.append('The complete 1,008-cell instruction campaign did not complete.')
if not det.get('all_equal'): not_verified.append('The complete cross-target determinism matrix did not pass.')
if len(metrics)!=5: not_verified.append('The complete five-mesh benchmark did not complete.')
if not shading.exists() or len(json.loads(shading.read_text()))!=10: not_verified.append('The complete ten-row shading campaign did not complete.')
table=A/'TABLE.md'
if not table.exists():table.write_text('# Instruction campaign incomplete\n')
# Idempotently replace appended sections.
table.write_text(table.read_text().split('\n# Final virtual-geometry')[0]+'\n'+vg+'\n'.join(bar_table+shade+not_verified)+'\n')
lines=['# 0.2.20 candidate qualification','',f'**Verdict: {verdict} against the complete bars.**',f"Candidate `{start['head']}`; baseline v0.2.19 `{start['base']}`. The manifest still declares 0.2.19; release versioning is operator-owned. Source is frozen; no Git index/reference changes or fuzz campaigns.",'','## Required gates','','| Gate | Exit | Log |','|---|---:|---|']
for n in ['fmt','clippy','release','release-all-features','doc','public-text']:
 r=latest.get(n);lines.append(f"| {n} | {r['exit'] if r else 'not run'} | {r['log'] if r else ''} |")
lines+=['','## Exact equivalence',f"Ordinary exact-driver runs: {s['exact_driver_runs']}; nonzero exits: {s['exact_driver_failed']}. Legacy cluster-LOD and MDAG 1 are checked in both feature modes, plus parallel and streaming identity.",'']
v2obs=[]
for mode in ['all','none']:
 r=latest.get('v2-exact-run-'+mode)
 if r: v2obs += [l for l in (L/r['log']).read_text().splitlines() if l.startswith('grid=')]
changed=sum('equal=false' in l for l in v2obs)
lines += [f'Weight-zero v2 observations: {changed}/{len(v2obs)} changed versus 0.2.19; {len(v2obs)-changed}/{len(v2obs)} are identical. These include three grid sizes, four input policies, and both feature modes. Builder revision 5 to 8 records the changed v2 policy.', '']
lines += ['| Driver | Exit | Retained summary |','|---|---:|---|']
for name,r in sorted(latest.items()):
 if name.startswith('exact-') and not name.startswith(('exact-lock-','exact-build-')):
  text=(L/r['log']).read_text().strip().splitlines()
  summary=(text[-1] if text else 'no output').replace('|','\\|')
  lines.append(f"| {name} | {r['exit']} | {summary} |")
lines.append('')
for n in ['legacy-run-all','legacy-run-none','v2-exact-run-all','v2-exact-run-none']:
 if n in latest:
  p=L/latest[n]['log'];lines += [f"`{n}` (exit {latest[n]['exit']}):",'```',p.read_text().strip(),'```']
lines+=['','V2 builder revision changed from 5 to 8: revision 6 records weighted region grouping; revision 8 records normal-loss pricing; revision 7 was reserved for a rejected codec trial. V2 byte differences are retained above without normalization.','']
p03=json.loads((A/'p03-differential.json').read_text()) if (A/'p03-differential.json').exists() else {}
cpp=json.loads((A/'cpp-observations.json').read_text()) if (A/'cpp-observations.json').exists() else {}
lines += ['## C++ 1.3',json.dumps(cpp.get('totals',{}),sort_keys=True),f"Spatial/meshlet/cluster-LOD generated cases: {sum(p03.get('counts',{}).values())}; real inputs: {len(p03.get('real',[]))}; exit: {p03.get('exit','not run')}. See p03-differential.json.",'','Failed-call destination writes are compared with the retained 0.2.19 receipts; they are separate from success-output differences. See cpp-observations.json and the retained counterexample.','','## Instructions',f"{inst.get('samples',0)} samples; {len(inst.get('regressions',[]))} case/mode regressions above 1%. One count per case, fixed arms cached by binary/input/operation and harness identity. See TABLE.md, release-results.json and instruction-bars.json. Counts cover the 21 retained operation families on eight frozen inputs.",'']
for r in inst.get('regressions',[]):lines.append('- '+json.dumps(r,sort_keys=True))
lines+=['','## Semver']
for n in ['semver-default','semver-all-features']:
 r=latest.get(n);lines.append(f"{n}: exit {r['exit'] if r else 'not run'}; "+(f"log {r['log']}" if r else ''))
lines+=['','## Determinism',f"Cross-variant equality: {det.get('all_equal',False)} across {len(det.get('variants',{}))} variants, including weights 0 and 0.25, std/no_std library dependencies, native/wasm32, and 1/2/4/8/16 workers in native all-feature release/debug. Driver hosts use standard host support. See determinism.json.",'','Benchmark reference: Bevy MeshletMesh commit 8cc060f01c3d4d5920781124cc8c32013bb47730. Input hashes and binary bindings are retained in corpus-check.json and bench/binding.json.','',vg,*bar_table,*shade,'','## Execution policy','','Release tests and doctests use RUST_TEST_THREADS=2. The initial default-worker release doctest attempt was killed by systemd-oomd (exit 137, reported peak 6 GB) under user-slice memory pressure. The retry retains the 4 GB declaration and every command, assertion and timeout. The all-features test/doctest retains its separate 9 GB reservation. See [resource-interruption.json](resource-interruption.json).','','The initial v2 adapter compile failure (101) and its rerun are retained in v2-adapter-repair.json and the command logs.','', '## Command failures']
for r in exits:
 if r['exit']:lines.append(f"- {r['name']}: exit {r['exit']}, log {r['log']}.")
lines+=not_verified+['',f"Source custody: {json.dumps(custody,sort_keys=True)}",'']
(A/'REPORT.md').write_text('\n'.join(lines))
p=q.R/'parity/vg-bench/RESULTS.md';old=p.read_text();marker='## Final 0.2.20 release qualification'
old=old.split(marker)[0].rstrip();p.write_text(old+'\n\n'+marker+f'\n\n**{verdict}**. Fresh candidate qualification, required gates, instruction counts, exact/C++ differentials and both shading weights: [REPORT](../0220-qualify/REPORT.md), [TABLE](../0220-qualify/TABLE.md).\n\n'+vg+'\n'.join(bar_table)+'\n')
# Retain binary identities before removing all disposable build outputs.
q.write(A/'retained-binaries.json',{str(p.relative_to(q.T)):q.sha(p) for p in (q.T/'bin').rglob('*') if p.is_file()})
q.run('public-text-final',['bash','parity/tools/check_public_text.sh'],check=False,cwd=q.HARNESS)
last=json.loads((L/'exits.jsonl').read_text().splitlines()[-1])
with (A/'REPORT.md').open('a') as f: f.write(f"Final public-text gate: exit {last['exit']}; log {last['log']} (logs are kept outside the repository).\n")
assert q.T.name=='meshopt-0220q' and q.T.is_absolute()
shutil.rmtree(q.T);q.write(A/'cleanup.json',dict(target_removed=not q.T.exists()))
print('Reports written; disposable target removed.',flush=True)
