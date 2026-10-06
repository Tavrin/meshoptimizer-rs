#!/usr/bin/env python3
"""Focused CPU correctness/portability checks before the final timing stream."""
import run as lane
import os,subprocess,shutil,time
assert os.environ.get('MOSS_HEAVY_ACTIVE')
commands=[
 ['python3','parity/encoders/run.py','counters','before-fair'],
 ['python3','parity/encoders/check_counter_effects.py'],
 ['python3','parity/encoders/edge_and_capacity.py'],
 ['cargo','test','--offline','--locked','--features','parity-internals','--test','codec04','--test','codec','--test','simd'],
 ['cargo','test','--offline','--locked','--lib','staged_vertex_destination_matches_initialized_reference'],
 ['cargo','+nightly','miri','test','--offline','--locked','--no-default-features','--features','std','--lib','staged_vertex_destination_matches_initialized_reference'],
 ['cargo','+1.88','check','--offline','--locked','--no-default-features'],
 ['cargo','check','--offline','--locked','--no-default-features','--target','wasm32-unknown-unknown'],
 ['cargo','clippy','--offline','--locked','--lib','--','-D','warnings'],
 ['cargo','fmt','--check'],
 ['python3','parity/simd/check_boundary.py'],
]
rows=[]
for cmd in commands:
 free=shutil.disk_usage(lane.ART).free
 assert free>=25*1024**3,'25 GiB build floor'
 print('VERIFY',cmd,flush=True);start=time.monotonic();p=subprocess.run(cmd,cwd=lane.ROOT,env=lane.ENV)
 rows.append({'command':cmd,'exit_code':p.returncode,'seconds':time.monotonic()-start,'free_bytes_at_start':free})
 lane.save('verification',{'rows':rows,'admission':os.environ['MOSS_HEAVY_ACTIVE'],'source':{str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'src').rglob('*.rs')}})
 assert p.returncode==0,cmd
print('VERIFY cargo package --list => published unsafe boundary',flush=True)
p=subprocess.run(['cargo','package','--offline','--locked','--list'],cwd=lane.ROOT,env=lane.ENV,stdout=subprocess.PIPE,check=True)
(lane.ART/'package-list.txt').write_bytes(p.stdout)
subprocess.run(['python3','parity/simd/check_package.py'],input=p.stdout,cwd=lane.ROOT,env=lane.ENV,check=True)
lane.save('verification',{'rows':rows,'package_audit':True,'package_list_sha256':lane.sha(lane.ART/'package-list.txt'),'admission':os.environ['MOSS_HEAVY_ACTIVE'],'source':{str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'src').rglob('*.rs')}})
