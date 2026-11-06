"""Exercise default cluster optimization in opaque and material hash cases."""
import importlib.util,json,traceback
from pathlib import Path
spec=importlib.util.spec_from_file_location('q',Path(__file__).with_name('run.py'));q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)
q.LOGS=q.LOGS/'default-matrix';fail=[]
try:q.new_apis_determinism()
except BaseException as e:
 fail.append(['determinism',q.clean(repr(e))]);(q.A/'default-matrix-failure.log').write_text(q.clean(traceback.format_exc()))
for variant in ['wasm-nostd','wasm-default','wasm-default-simd128']:
 try:q.run('wasm-timing-'+variant,['node',q.A/'wasm_timing.mjs',q.T/'det-target'/variant/'wasm32-unknown-unknown/release/determinism.wasm'])
 except BaseException as e:fail.append([variant,q.clean(repr(e))])
q.write(q.A/'default-matrix-status.json',dict(ok=not fail,failures=fail))
print('DEFAULT MATRIX COMPLETE',fail,flush=True)
