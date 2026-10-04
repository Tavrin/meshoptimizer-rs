#!/usr/bin/env python3
"""Record P02's required checks without overwriting other lanes' records."""
import json
import os
from pathlib import Path
import subprocess
import time
from measure import ROOT,ART,TARGET,ENV,sha

def main():
    checks=[
        ['cargo','fmt','--all','--check'],
        ['cargo','fmt','--manifest-path','parity/codec/Cargo.toml','--check'],
        ['cargo','fmt','--manifest-path','fuzz/codec/Cargo.toml','--check'],
        ['cargo','clippy','--offline','--locked','--all-targets','--all-features','--','-D','warnings'],
        ['cargo','clippy','--offline','--locked','--manifest-path','parity/codec/Cargo.toml','--all-targets','--','-D','warnings'],
        ['cargo','test','--offline','--locked','--manifest-path','parity/codec/Cargo.toml'],
        ['cargo','test','--offline','--locked','--all-features'],
        ['cargo','test','--offline','--locked','--no-default-features'],
        ['cargo','build','--offline','--locked','--release','--target','wasm32-unknown-unknown','--no-default-features'],
        ['cargo','clippy','--offline','--locked','--manifest-path','fuzz/codec/Cargo.toml','--all-targets','--','-D','warnings'],
    ]
    rows=[]
    for i,args in enumerate(checks):
        env=ENV if 'fuzz/codec/Cargo.toml' not in args else dict(ENV,CARGO_HOME=str(ART/'cargo-home'))
        log=ART/f'gate-{i}.log';start=time.monotonic()
        with log.open('w') as output:result=subprocess.run(args,cwd=ROOT,env=env,stdout=output,stderr=subprocess.STDOUT)
        row={'command':args,'exit_code':result.returncode,'elapsed_seconds':time.monotonic()-start,'log':str(log),'log_sha256':sha(log)};rows.append(row)
        (ART/'gates.json').write_text(json.dumps(rows,indent=2));print(args,result.returncode,flush=True)
        if result.returncode:raise SystemExit(result.returncode)
if __name__=='__main__':main()
