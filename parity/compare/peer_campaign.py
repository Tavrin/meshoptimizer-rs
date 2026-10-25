#!/usr/bin/env python3
"""Sequential admissions and checkpoint resumes. Run from this worktree."""
import os,sys,pathlib,subprocess,time,json
ROOT=pathlib.Path(__file__).resolve().parents[2];ART=pathlib.Path(os.environ['MESHOPT_COMPARE_ARTIFACTS'])
env=dict(os.environ,CARGO_TARGET_DIR='/mnt/linux-extra/moss-cargo-targets/meshopt-peer-screen',MOSS_HEAVY_GPU='0',MOSS_HEAVY_DISK_FLOOR_GB='26',MOSS_LANE='meshopt-peer-screen')
def run(name,script,args=(),measurement=False):
 for attempt in range(1000):
  receipt=ART/f'admission-{name}-{attempt:03}.log'
  if receipt.exists():continue
  cmd=['/mnt/linux-extra/moss-coord/bin/moss-heavy.sh','4','timeout','840','python3',str(ROOT/'parity/compare'/script),*args]
  e=dict(env,MOSS_HEAVY_MEASUREMENT='1'if measurement else'0')
  print('ADMIT',name,attempt,flush=True)
  with receipt.open('w')as f:r=subprocess.run(cmd,env=e,stdout=f,stderr=subprocess.STDOUT)
  print('EXIT',name,r.returncode,flush=True)
  if r.returncode==75:
   # Outside admission, allow train jobs to take their slot before requeueing.
   time.sleep(10);continue
  if r.returncode:raise RuntimeError(f'{name}: rc={r.returncode}; {receipt}')
  return
 raise RuntimeError('checkpoint budget exhausted')
if __name__=='__main__':
 stage=sys.argv[1]
 if stage=='prepare':run('prepare','peer_screen.py',['prepare'])
 elif stage=='correctness':run('correctness','peer_screen.py',['correctness'])
 elif stage=='timing':
  run('timing-moss-simd-AA','peer_screen.py',['timing','moss','simd','AA'],True)
  for profile in ['defaults','moss']:
   for mode in ['scalar','simd']:
    for arm in ['M7','X']:run(f'timing-{profile}-{mode}-{arm}','peer_screen.py',['timing',profile,mode,arm],True)
 elif stage=='wasm-build':run('wasm-build','peer_wasm_build.py')
 elif stage=='browser-correctness':run(stage,'peer_wasm_postprocess.py',[],True)
 elif stage=='browser-timing':run(stage,'peer_browser_run.py',['timing'],True)
