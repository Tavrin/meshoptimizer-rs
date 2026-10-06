#!/usr/bin/env python3
"""Build the immutable eb5b1a2 library with the final identical lane consumer."""
import io,json,subprocess,shutil,tarfile,os
from pathlib import Path
import run as lane
A=lane.ART;R=lane.ROOT;T=lane.TARGET
assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert shutil.disk_usage(A).free>=25*1024**3
base=A/'baseline-tree'
assert not base.exists(), 'immutable baseline already constructed'
base.mkdir()
blob=subprocess.check_output(['git','archive','eb5b1a2','src','Cargo.toml','Cargo.lock','LICENSE','README.md'],cwd=R)
with tarfile.open(fileobj=io.BytesIO(blob)) as archive:archive.extractall(base,filter='data')
shutil.copytree(R/'parity/codec',base/'parity/codec',ignore=shutil.ignore_patterns('__pycache__'))
source={str(p.relative_to(base)):lane.sha(p) for p in (base/'src').rglob('*.rs')}
consumer={str(p.relative_to(base)):lane.sha(p) for p in (base/'parity/codec').rglob('*') if p.is_file()}
assert consumer=={str(p.relative_to(R)):lane.sha(p) for p in (R/'parity/codec').rglob('*') if p.is_file() and '__pycache__' not in p.parts}
subprocess.run(['cargo','build','--offline','--locked','--release','--features','simd','--manifest-path',str(base/'parity/codec/Cargo.toml')],cwd=base,env=lane.ENV,check=True)
shutil.copy2(T/'release/codec-driver',A/'rust-before-fair')
lane.save('build-before-fair',{'revision':'eb5b1a2df97c6607123befc45bc830e2aeff2a76','source':source,'consumer':consumer,'binary':lane.sha(A/'rust-before-fair'),'compiler':subprocess.check_output(['rustc','-Vv'],text=True),'admission':os.environ['MOSS_HEAVY_ACTIVE']})
assert source==json.loads((A/'build-before.json').read_text())['source']
subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(R/'parity/encoders/Cargo.toml')],cwd=R,env=lane.ENV,check=True)
shutil.copy2(T/'release/encoder-crate-oracle',A/'crate')
lane.save('build-crate-fair',{'consumer':lane.sha(R/'parity/encoders/src/main.rs'),'binary':lane.sha(A/'crate'),'manifest':lane.sha(R/'parity/encoders/Cargo.toml'),'lock':lane.sha(R/'parity/encoders/Cargo.lock'),'admission':os.environ['MOSS_HEAVY_ACTIVE']})
print('fair baseline and old-crate adapters built',flush=True)
