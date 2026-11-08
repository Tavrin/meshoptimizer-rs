from pathlib import Path
import os,json,subprocess,hashlib,shutil
E=Path(os.environ['VG_EVIDENCE']);R=E/'shading';D=R/'selected-build';D.mkdir(exist_ok=False)
checks=[('fmt',['cargo','fmt']),('v2-tests',['cargo','test','--release','--all-features','--test','vg_build_v2']),('harness',['cargo','build','--release','--locked','--manifest-path','parity/vg-bench/Cargo.toml'])]
for name,cmd in checks:
 argv=cmd if name=='fmt' else [os.environ['VG_HEAVY_WRAPPER'],'4',*cmd]
 env=dict(os.environ);env.pop('MOSS_HEAVY_PROOF_SECONDS',None)
 with (D/(name+'.log')).open('w') as f:rc=subprocess.run(argv,env=env,stdout=f,stderr=subprocess.STDOUT).returncode
 with (D/'exits.jsonl').open('a') as f:f.write(json.dumps(dict(name=name,exit=rc,command=cmd))+'\n')
 print(name,rc,flush=True)
 if rc:raise SystemExit(rc)
shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/vg-bench',R/'vg-bench-final')
(D/'binary.json').write_text(json.dumps(dict(sha256=hashlib.sha256((R/'vg-bench-final').read_bytes()).hexdigest()),indent=2)+'\n')
(D/'source.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for root in ['src','tests','docs','parity/vg-bench/src'] for p in Path(root).rglob('*') if p.is_file()},indent=2)+'\n')
