"""Build each feature arm sequentially inside the shared admission runner."""
import hashlib, json, os, shutil, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2]
art=Path(os.environ['PEERS_ART'])
for arm in ['ours','cpp','rust']:
    command=['cargo','build','--offline','--release','--manifest-path',str(root/'parity/peers/Cargo.toml'),'--no-default-features','--features',arm,'-j','1']
    with (art/f'build-{arm}.log').open('w') as log:
        result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
    (art/f'build-{arm}.json').write_text(json.dumps(dict(command=command,exit=result.returncode))+'\n')
    if result.returncode:raise SystemExit(result.returncode)
    shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/meshopt-peers',art/f'driver-{arm}')
    print(arm,'built',flush=True)
(art/'build-binding.json').write_text(json.dumps(dict(binaries={a:hashlib.sha256((art/f'driver-{a}').read_bytes()).hexdigest() for a in ['ours','cpp','rust']},source={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [*root.glob('src/**/*.rs'),*root.glob('parity/peers/src/*.rs'),root/'parity/peers/Cargo.toml',root/'parity/peers/Cargo.lock']}),indent=2)+'\n')
