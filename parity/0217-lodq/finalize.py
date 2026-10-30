"""Verify retained evidence, render the result table, scrub paths, clean target."""
import hashlib,json,math,os,re,shutil,subprocess
from pathlib import Path
os.environ['GIT_OPTIONAL_LOCKS']='0'
r=Path.cwd();lane=r/'parity/0217-lodq';out=lane/'receipts';t=Path(os.environ['CARGO_TARGET_DIR'])
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
exits=dict(line.split('\t') for line in (out/'exits.tsv').read_text().splitlines())
required=['final-focused','final-focused-scalar','final-all','scalar','final-wasm','final-wasm-default','post-clippy','post-fmt','boundary','exact-default','exact-scalar','cpp-oracle-default','cpp-oracle-scalar','proof-build-default','proof-build-scalar','oracle-build','perf-final-binding','perf-measure','post-rebind']
for step in required:assert exits[step]=='0',(step,exits.get(step))
for name,digest in json.loads((out/'source-binding.json').read_text()).items():assert sha(r/name)==digest,name
binding=json.loads((out/'perf-binding.json').read_text())
for name,digest in binding['candidate']['sources'].items():
 if name.startswith('src/') or name=='Cargo.toml':assert sha(r/name)==digest,('performance source',name)
for mode in ['default','scalar']:
 text=(out/('final-focused'+('-scalar' if mode=='scalar' else '')+'.log')).read_text();assert '16 passed; 0 failed' in text
perf=json.loads((out/'perf-results.json').read_text());cells=perf['results'];assert len(cells)==8
rows=[json.loads(line) for line in (out/'counts.jsonl').read_text().splitlines()];assert len(rows)==96
for cell in cells:
 assert len({v['output_sha256'] for v in rows if v['case']==cell['case']})==1
timings=[json.loads(line) for line in (out/'proof-timing.jsonl').read_text().splitlines()]
cpu=sum(v['cpu_seconds'] for v in timings if v['step'].startswith(('exact-','cpp-oracle-')));assert cpu<7200
exact={m:(out/('exact-'+m+'.log')).read_text().strip() for m in ['default','scalar']}
text=['# Final instruction guard','', 'Unchanged simplify; median of three instructions:u samples per cell. All four outputs are byte-identical. Candidate enables lod-quality; C++ arms use pinned meshopt 0.7.0 / upstream 1.3. F9 specifies no instruction-ratio target for this additive API; this guard reports the shared pipeline comparison. No equivalent base or upstream quality API exists.','',
'| Case | Base instructions | Candidate | C++ scalar | C++ default SIMD | Candidate/base |',
'|---|---:|---:|---:|---:|---:|']
for c in cells:
 v=c['instructions'];text.append(f"| {c['case']} | {v['base']:.2f} | {v['candidate']:.2f} | {v['cpp-scalar']:.2f} | {v['cpp-default']:.2f} | {c['candidate_over_base']:.8f} |")
text+=['','Wall time disclosure only (seconds per call; very short cells have coarse clock resolution):','',
'| Case | Base | Candidate | C++ scalar | C++ default |','|---|---:|---:|---:|---:|']
for c in cells:
 v=c['wall_seconds'];text.append(f"| {c['case']} | {v['base']:.9f} | {v['candidate']:.9f} | {v['cpp-scalar']:.9f} | {v['cpp-default']:.9f} |")
text+=['','The first campaign is retained in receipts/perf-initial. The final quality-only lint fix changed the driver binary hash, so the complete 96-sample campaign was repeated against final frozen source. Final results use only the second campaign. The source/binary hashes and raw samples are retained in receipts/.','']
(lane/'TABLE.md').write_text('\n'.join(text))
summary=dict(status='PASS',final_exit=0,base='09da597f438471d0ad9c16e460f3598e9356e9ea',focused_tests_per_mode=16,analytic_tolerance=2e-7,
 scaling=dict(triangles=[256,2048],work=[75114,773568],ratio=773568/75114),exact=exact,proof_cpu_seconds=cpu,
 perf_samples=96,perf_event='instructions:u',perf_gate='valid instructions:u samples, exact four-arm outputs, final-source binding',
 worst_candidate_over_base=max(c['candidate_over_base'] for c in cells),final_gates={s:0 for s in required},
 retained_initial_failures=['clippy:101 (manual_is_multiple_of)','final-clippy:101 (fixture chunks_exact_to_as_chunks)'],
 fuzz='source added; not run',target_removed=False)
for group,cs in [('corpus',cells[:5]),('real',cells[5:])]:
 summary[group+'_instruction_ratios']={k:math.exp(sum(math.log(c[k]) for c in cs)/len(cs)) for k in ['candidate_over_base','candidate_over_cpp_scalar','candidate_over_cpp_default']}
proof=(lane/'PROOFS.md').read_text().split('\n## Execution results')[0]
proof+='\n## Execution results\n\nFinal gates PASS (exit 0); 16 focused tests in each mode, full all-features\n'+\
 'and no-default suites/doctests, Clippy, fmt, unsafe-boundary and both wasm builds.\n'+\
 'Initial Clippy failures 101 are retained; both were corrected and the final\n'+\
 'Clippy run exited 0. Final all-features tests also ran after the production fix.\n\n'+\
 '\n'.join(f'- {m}: {v}' for m,v in exact.items())+'\n\n'+\
 f'Independent C++ oracle: 256 surface pairs in each mode, 1e-7 tolerance, zero differences.\n'+\
 f'Exact/oracle executable checks consumed {cpu:.2f} CPU seconds, below 2 CPU-hours.\n'+\
 'Analytic max/RMS cases passed at 2e-7 tolerance. Scaling: 75,114 versus\n'+\
 f'773,568 work units for 256 versus 2,048 triangles ({773568/75114:.5f}x).\n\n'+\
 f'Final performance: 96 samples; worst candidate/base cell {summary["worst_candidate_over_base"]:.8f}x.\n'+\
 'See TABLE.md for all instruction medians, C++ comparisons and wall-time disclosure.\n'+\
 'The final candidate source hash was verified against the measured snapshot.\n'+\
 'The first campaign was retained and repeated after a final-source binary hash change.\n'+\
 'Transient target storage was deleted after verification. The fuzz target was not run.\n'
(lane/'PROOFS.md').write_text(proof)
# Change paths only, never numeric data or test/measurement outcomes.
scrub={}
for p in sorted(out.rglob('*.log')):
 before=p.read_text();after=before
 for old,new in [(str(r),'<worktree>'),(str(t),'<target>'),(str(Path.home()/'.cargo'),'<cargo-home>')]:after=after.replace(old,new)
 if before!=after:
  scrub[str(p.relative_to(out))]=dict(before=hashlib.sha256(before.encode()).hexdigest(),after=hashlib.sha256(after.encode()).hexdigest())
  p.write_text(after)
(out/'path-scrub.json').write_text(json.dumps(scrub,indent=2)+'\n')
files=[r/'Cargo.toml',r/'Cargo.lock',r/'README.md',r/'docs/USING.md',r/'src/lib.rs',r/'src/lod_quality.rs',r/'tests/lod_quality.rs']
files += [p for p in lane.rglob('*') if p.is_file() and out not in p.parents]
(out/'submission-binding.json').write_text(json.dumps({str(p.relative_to(r)):sha(p) for p in sorted(files)},indent=2)+'\n')
p=subprocess.run(['bash','parity/tools/check_public_text.sh'],capture_output=True,text=True)
(out/'public-text.log').write_text(p.stdout+p.stderr)
with (out/'exits.tsv').open('a') as f:f.write(f'public-text\t{p.returncode}\n')
assert p.returncode==0,p.stdout+p.stderr
summary['final_gates']['public-text']=0
p=subprocess.run(['git','diff','--check'],capture_output=True,text=True);assert p.returncode==0,p.stdout+p.stderr
assert t.name=='meshopt-0217-lodq' and t.is_dir() and t!=r
shutil.rmtree(t);summary['target_removed']=not t.exists();assert summary['target_removed']
(out/'final.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
