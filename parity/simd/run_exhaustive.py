#!/usr/bin/env python3
"""Bounded, resumable native/Node arithmetic ranges; one core at a time."""
import argparse, hashlib, json, os, subprocess, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('backend',choices=['native','wasm']);p.add_argument('--cpu',type=int,default=0);args=p.parse_args()
art=Path(os.environ['MESHOPT_ARTIFACTS']);target=Path(os.environ['CARGO_TARGET_DIR']);directory=art/('exhaustive-'+args.backend);directory.mkdir(exist_ok=True)
name='meshopt-simd-qualification' if args.backend=='native' else 'meshopt_simd_qualification.wasm'
binary=target/'release'/name if args.backend=='native' else target/'wasm32-unknown-unknown/release'/name
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
identity={'binary_sha256':sha(binary),'sources':{str(f.relative_to(ROOT)):sha(f) for f in sorted([*ROOT.glob('src/**/*.rs'),*ROOT.glob('parity/simd/src/*.rs'),ROOT/'parity/simd/Cargo.toml',ROOT/'parity/simd/Cargo.lock'])},'cpu':args.cpu,'node':subprocess.check_output(['node','--version'],text=True).strip() if args.backend=='wasm' else None,'profile':'release, generic x86-64 runtime dispatch' if args.backend=='native' else 'wasm32 +simd128; native Rust scalar reference within same module'}
(art/'bin').mkdir(exist_ok=True);import shutil
shutil.copy2(binary,art/'bin'/(name+'-'+args.backend))
for family,total in [('sqrt',1<<32),('exp',1<<32),('oct8',1<<24),('oct16',100000000),('quat',100000000)]:
    for start in range(0,total,1<<27):
        end=min(total,start+(1<<27));record=directory/f'{family}-{start}-{end}.json'
        if record.exists():
            prior=json.loads(record.read_text());assert prior['identity']==identity;continue
        command=[str(binary),family,str(start),str(end)] if args.backend=='native' else ['node',str(ROOT/'parity/simd/exhaustive.mjs'),str(binary),str(['sqrt','exp','oct8','oct16','quat'].index(family)),str(start),str(end)]
        began=time.monotonic();load=os.getloadavg()
        run=subprocess.run(['taskset','-c',str(args.cpu),*command],capture_output=True,text=True)
        result={'identity':identity,'family':family,'start':start,'end':end,'records':end-start,'exit_code':run.returncode,'stdout':run.stdout,'stderr':run.stderr,'wall_seconds':time.monotonic()-began,'load_before':load,'load_after':os.getloadavg()}
        record.write_text(json.dumps(result,indent=2)+'\n')
        assert run.returncode==0,(family,start,run.stderr)
        print(args.backend,family,start,end,'PASS',round(result['wall_seconds'],2),flush=True)
        # Explicitly release the core between bounded ranges.
        time.sleep(.05)
assert sha(binary)==identity['binary_sha256']
if args.backend=='native':subprocess.run(['taskset','-c',str(args.cpu),str(binary),'edges'],check=True)
(directory/'complete.json').write_text(json.dumps({'identity':identity,'complete':True,'records':{k:v for k,v in [('sqrt',1<<32),('exp',1<<32),('oct8',1<<24),('oct16',100000000),('quat',100000000)]}},indent=2)+'\n')
