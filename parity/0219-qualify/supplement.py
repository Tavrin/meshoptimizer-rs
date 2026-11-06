"""Run preserved and new API checks after the instruction campaign."""
import importlib.util,traceback
from pathlib import Path
spec=importlib.util.spec_from_file_location('qual',Path(__file__).with_name('run.py'));q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)
fail=[]
def stage(n,f):
 try:f()
 except BaseException as e:
  fail.append([n,q.clean(repr(e))]);(q.A/(n+'-failure.log')).write_text(q.clean(traceback.format_exc()));print('FAILED',n,repr(e),flush=True)
 q.write(q.A/'supplement-failures.json',fail)
def legacy(mode):
 d=q.T/('legacy-'+mode);q.manifest(d,{'legacy_exact':q.A/'legacy_exact.rs'},q.ALL if mode=='all' else ['clusterlod'])
 q.run('legacy-build-'+mode,['cargo','build','--offline','--release','--manifest-path',d/'Cargo.toml'])
 q.shutil.copy2(q.T/'release/legacy_exact',q.T/'bin'/('legacy-exact-'+mode))
 q.write(q.A/('legacy-binding-'+mode+'.json'),dict(binary=q.sha(q.T/'bin'/('legacy-exact-'+mode)),manifest=q.clean((d/'Cargo.toml').read_text())))
 q.run('legacy-run-'+mode,[q.T/'bin'/('legacy-exact-'+mode)])
stage('cpp-failure-counterexample',lambda:q.run('cpp-failure-counterexample',['python3',q.A/'cpp_failure_repro.py']))
for mode in ['all','none']:stage('legacy-'+mode,lambda mode=mode:legacy(mode))
stage('determinism',q.new_apis_determinism)
for variant in ['wasm-nostd','wasm-default','wasm-default-simd128']:
 binary=q.T/'det-target'/variant/'wasm32-unknown-unknown/release/determinism.wasm'
 if binary.exists():stage('wasm-timing-'+variant,lambda variant=variant,binary=binary:q.run('wasm-timing-'+variant,['node',q.A/'wasm_timing.mjs',binary]))
print('SUPPLEMENT COMPLETE' ,fail,flush=True)
