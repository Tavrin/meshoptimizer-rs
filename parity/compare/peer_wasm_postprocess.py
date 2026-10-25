#!/usr/bin/env python3
"""Match REPORT-3's wasm-opt -Oz --enable-simd step, retaining raw modules."""
import pathlib,os,subprocess,json,hashlib,sys
ROOT=pathlib.Path(__file__).resolve().parents[2];ART=pathlib.Path(os.environ['MESHOPT_COMPARE_ARTIFACTS'])
assert os.environ.get('MOSS_HEAVY_ACTIVE')
receipt=ART/'wasm-postprocess.json'
if not receipt.exists():
 records=[]
 for profile in ['defaults','moss']:
  for arm in ['O','X']:
   target=ART/f'peer-{profile}-{arm}.wasm';raw=ART/f'peer-{profile}-{arm}-raw.wasm'
   if not raw.exists():target.rename(raw)
   assert not target.exists(),'retain prior postprocess attempt'
   cmd=['wasm-opt','-Oz','--enable-simd','--enable-bulk-memory','--enable-sign-ext','--enable-nontrapping-float-to-int','--enable-multivalue','--enable-mutable-globals','--enable-reference-types',str(raw),'-o',str(target)]
   with(ART/f'wasm-opt-{profile}-{arm}.log').open('w')as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
   if r.returncode:raise SystemExit(r.returncode)
   records.append({'command':cmd,'raw_sha256':hashlib.sha256(raw.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE']})
 receipt.write_text(json.dumps({'version':subprocess.check_output(['wasm-opt','--version'],text=True).strip(),'modules':records},indent=2)+'\n')
r=subprocess.run([sys.executable,str(ROOT/'parity/compare/peer_browser_run.py'),'correctness'])
sys.exit(r.returncode)
