"""Bind the completed receipts, summarize results, then remove the dedicated target."""
import hashlib,io,json,os,re,shutil,subprocess,tarfile
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'receipts';target=Path(os.environ['CARGO_TARGET_DIR'])
assert target.name=='meshopt-0217-hsr' and target.is_dir()
performance=json.loads((receipt/'performance.json').read_text());assert performance['cache_regression_gate']
rows=[json.loads(s) for s in (receipt/'counts.jsonl').read_text().splitlines()];assert len(rows)==120
assert json.loads((receipt/'oracle.json').read_text())['observations']==480
for name in ['tests-all','tests-none','clippy-all','clippy-none','wasm-all','wasm-none','boundary','exact-all','exact-none','hidden-release']:
    assert any(line.startswith(name+'\t0\t') for line in (receipt/'exits.tsv').read_text().splitlines()),name
base='09da597f438471d0ad9c16e460f3598e9356e9ea'
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==base
archive=subprocess.check_output(['git','archive',base,'src'])
unchanged={}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
with tarfile.open(fileobj=io.BytesIO(archive)) as t:
    for member in t.getmembers():
        if member.isfile() and member.name.endswith('.rs') and member.name!='src/lib.rs':
            data=t.extractfile(member).read();assert data==(root/member.name).read_bytes(),member.name
            unchanged[member.name]=hashlib.sha256(data).hexdigest()
# Bind algorithm sources, API docs, public tests, exact drivers and fuzz source.
files=[*root.glob('src/**/*.rs'),root/'tests/hidden.rs',root/'Cargo.toml',root/'Cargo.lock',root/'README.md',root/'docs/USING.md',*lane.glob('*.rs'),*lane.glob('*.cpp'),*lane.glob('*.py'),*lane.glob('*.sh')]
binding=dict(base=base,unchanged_algorithm_files=unchanged,source={str(p.relative_to(root)):sha(p) for p in sorted(set(files))},binaries=json.loads((receipt/'binaries.json').read_text()),exact_binaries={p.name:sha(p) for p in (target/'release/deps').glob('hsr_exact-*') if p.is_file() and os.access(p,os.X_OK)})
(receipt/'source-binding.json').write_text(json.dumps(binding,indent=2,sort_keys=True)+'\n')
environment=dict(rustc=subprocess.check_output(['rustc','-vV'],text=True),perf=subprocess.check_output([os.environ['HSR_PERF'],'--version'],text=True).strip(),counter='instructions:u',locale='C',feature_modes=['all features (exact/gates)','std+simd (performance)','no default features'],wasm_features=['std,simd,experimental,clusterlod','no default features'])
(receipt/'environment.json').write_text(json.dumps(environment,indent=2)+'\n')
# Replace the provisional evidence paragraph with the completed record.
p=lane/'PROOFS.md';s=p.read_text();a=s.index('The final receipts, exit codes, feature modes, exact comparison counts,');b=s.index('For performance,',a)
s=s[:a]+'''`receipts/exits.tsv` retains final passing gates and the two pre-measurement
baseline-layout failures (exit 101). Earlier lint failures and their corrected
full reruns are retained in `initial/`, `second/`, and `third/`; bootstrap test
constructor corrections are described in `bootstrap.json`. No failed attempt is
presented as a passing gate. The definitive full run passes all-feature and
no-default-feature tests, Clippy with warnings denied, package/unsafe boundary,
WASM builds, exact comparisons, release feature tests and public-text checks.
Doctests pass 40 cases with all features and 37 without default features; five
pre-existing ignored examples remain in each mode.
WASM is build evidence only; no execution on WASM is claimed. The default-SIMD
WASM build retains five existing unused-dispatch warnings. No new unsafe code.

The 15 focused tests cover nested cubes, an independent interior surface hidden
by a closed lid and visible through its opening, a resting box with fully and
partially covered support triangles from accessible cameras, exposed and enclosed
back-to-back walls, split-index/signed-zero duplicates, visible sample retention,
tolerance, perspective and sampled-volume views, nonpacked positions, empty and
extreme finite geometry, invalid inputs/options, the inclusive density limits,
workspace recovery, output tails, every work limit through success + 1 and byte
boundaries. Every failing caller-buffer call preserves all sentinel entries.
The constructed fixtures and independent oracle code are dedicated to CC0-1.0.

`exact.rs` compares 46 unchanged preprocessing/repair API-policy combinations on
nine fixtures, cold/repeated/after-buffer-failure/retry, at every work limit
through success + 1 and at byte boundaries. Results/errors, complete destination
and Usage match v0.2.16: **87,272 observations per mode, 174,544 total**. The exact
executions each take less than a second. All pre-existing algorithm source files
except the export/documentation root are verified byte-identical to the base in
`source-binding.json`; existing algorithms are not altered by this feature.

`oracle.cpp` is an independent brute-force implementation of sampled visibility,
not an upstream HSR function. Eighty seeded scenes, half surrounded by a closed
shell, compare all kept index bytes at denominators 1, 2 and 4 against both Rust
feature modes: **480 comparisons passed**. This bounded differential and the
small exact sweeps remain well below two CPU-hours. `cpp-source-binding.json`
checks the vendored C++ source against the pinned 1.3 reference. The four cache
arms additionally agree on every complete real/corpus mesh, including the
no-default-feature Rust driver. The source-only fuzz target is `fuzz.rs`; it was
formatted but intentionally neither built nor run.

`campaign.py` measures warm resident calls with input parsing, initial call and
output serialization outside counters. It uses instructions:u, three interleaved
repetitions (reverse arm order in repetition two), a fixed CPU and median per
cell. The same default options apply to all HSR cases. Output hashes and reports
are identical across repetitions. Raw perf records and all 120 observations are
retained. Wall times are disclosure only. Counter enable/disable control overhead
is included; it is shared by all arms. HSR calls also include the driver's
small options-environment check and report formatting, so these are conservative
whole-driver call costs, not isolated kernel counts. `counts.jsonl` also retains complete
setup-and-run wall times, which include fixture loading and warmup.

The unchanged-cache candidate/base regression gate is <=1.01 for every input.

| Input | Base instructions | Candidate | C++ scalar | C++ default | Candidate/base |
| --- | ---: | ---: | ---: | ---: | ---: |
'''+s[b:]
marker='For performance,';at=s.index(marker)
cache=''
for row in performance['cache']:
    m=row['instructions'];cache+=f"| {row['case']} | {m['base']:.0f} | {m['candidate']:.0f} | {m['cpp-scalar']:.0f} | {m['cpp-default']:.0f} | {row['candidate_vs_base']:.6f} |\n"
hidden='''
HSR disclosure with default 26 directions and seven samples per triangle:

| Input | Removed / input triangles | Share | Median call wall seconds | Median instructions | Counted work | Peak charged bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
'''
for row in performance['hidden']:
    r=row['report'];hidden+=f"| {row['case']} | {r['removed']} / {r['input']} | {row['removed_share']*100:.6f}% | {row['median_wall_seconds']:.6f} | {row['median_instructions']:.0f} | {r['work']} | {r['bytes']} |\n"
scaling='''
The separated-triangle N versus 8N test (128 versus 1,024 triangles) records
**9,856 versus 124,928 work units: 12.6753x work for 8x input**, below the 16x
bound. This is a counted-work scaling result on the stated fixture, not a
worst-case linear-complexity or wall-time claim.

'''
s=s[:at]+cache+hidden+scaling+s[at:]
s+=f"\nThe complete final campaign consumed {performance['campaign_cpu_seconds']:.3f} CPU seconds including its builds, oracle, fixture loading and measurements.\n"
s+='''
`run.sh` reproduces both phases inside a caller-supplied CPU admission wrapper,
using a dedicated target and the HSR_INPUTS, HSR_REFERENCE and HSR_PERF variables.
Its optional `checks` or `campaign` argument selects one phase. Source bindings
cover implementation, tests, guide, drivers and fuzz source; binary and input
hashes are retained. No queue was canceled, and no other model or AI CLI was
invoked. The dedicated target is removed after the last admitted job completes;
`receipts/cleanup.json` records the deletion. Final Git integration is left to
the operator.
'''
p.write_text(s)
for name,cmd in [('final-format',['cargo','fmt','--all','--','--check']),('final-whitespace',['git','diff','--check']),('final-public-text',['bash','parity/tools/check_public_text.sh'])]:
    out=subprocess.run(cmd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    (receipt/(name+'.log')).write_text(out.stdout.replace(str(root),'<worktree>').replace(str(target),'<target>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{out.returncode}\n')
    print(name,out.returncode,out.stdout,flush=True);assert out.returncode==0
shutil.rmtree(target)
(receipt/'cleanup.json').write_text(json.dumps(dict(exit=0,dedicated_target_removed=not target.exists(),admitted_jobs_completed=True,no_queue_canceled=True))+'\n')
assert not target.exists()
(receipt/'completion.json').write_text(json.dumps(dict(exit=0,final_gates_pass=True,exact_observations=174544,cpp_oracle_comparisons=480,perf_samples=len(rows),fuzz_run=False,wasm_executed=False))+'\n')
print('PASS final gates, source identity, receipts and dedicated target cleanup')
