"""Build a default-feature adapter against frozen v0.2.15, without diagnostic ceilings."""
import os,shutil,subprocess
from pathlib import Path
root=Path.cwd();target=Path(os.environ['CARGO_TARGET_DIR']);art=Path(os.environ['DECODE_ART']);d=target/'wasm-default-source';d.mkdir(exist_ok=True)
shutil.copy2(root/'parity/0216-decode/wasm_adapter.rs',d/'lib.rs')
(d/'Cargo.toml').write_text('[package]\nname="decode-wasm-default"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="lib.rs"\ncrate-type=["cdylib"]\n[dependencies]\nmeshoptimizer-rs={path="../frozen-base"}\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
for flags,name in [('', 'scalar'),('-C target-feature=+simd128','simd')]:
    env=dict(os.environ,RUSTFLAGS=flags);env.pop('CARGO_ENCODED_RUSTFLAGS',None)
    subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(d/'Cargo.toml'),'--target','wasm32-unknown-unknown','--lib'],env=env,check=True)
    shutil.copy2(target/'wasm32-unknown-unknown/release/decode_wasm_default.wasm',art/('wasm-default-'+name+'.wasm'))
