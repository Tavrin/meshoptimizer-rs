"""Run inside the required CPU admission; all locations come from the environment."""
import importlib.util, json, os, traceback
from pathlib import Path
p=Path(__file__).with_name('run.py')
spec=importlib.util.spec_from_file_location('qual',p); q=importlib.util.module_from_spec(spec);spec.loader.exec_module(q)
fail=[]
def stage(name, f):
 try: f()
 except BaseException as e:
  fail.append({'stage':name,'error':q.clean(repr(e))})
  (q.A/(name+'-failure.log')).write_text(q.clean(traceback.format_exc()))
  print('STAGE FAILED',name,repr(e),flush=True)
 q.write(q.A/'campaign-failures.json',fail)
for name,cmd in [('fmt',['cargo','fmt','--check']),('clippy',['cargo','clippy','--all-targets','--all-features','--','-D','warnings']),('release',['cargo','test','--release']),('release-all-features',['cargo','test','--release','--all-features']),('public-text',['bash','parity/tools/check_public_text.sh'])]:
 stage(name,lambda name=name,cmd=cmd:q.run(name,cmd,check=False))
stage('semver',q.semver)
for mode in ['all','none']:
 stage('exact-'+mode,lambda mode=mode:q.proof_exact(mode))
stage('identity',q.identity_group)
stage('differential',q.differential_group)
stage('peers',q.build_group)
for group in ['corpus','real-214e','real-a082','real-b6fd']:
 stage('measure-'+group,lambda group=group:q.measure(group))
stage('summarize',q.summarize)
print('CAMPAIGN COMPLETE',fail,flush=True)
