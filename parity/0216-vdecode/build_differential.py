"""Build the portable native differential entry points (also used by arm64 CI)."""
import os,shutil,subprocess
from pathlib import Path
root=Path.cwd();target=Path(os.environ['CARGO_TARGET_DIR']);ref=Path(os.environ['MESHOPT_REFERENCE'])
p=target/'differential-source';p.mkdir(parents=True,exist_ok=True)
shutil.copy2(root/'parity/0216-vdecode/differential.rs',p/'main.rs')
features=',default-features=false,features=["simd","parity-internals"]' if os.environ.get('DECODE_NO_STD') else ',features=["parity-internals"]'
(p/'Cargo.toml').write_text('[package]\nname="decode-differential"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={path='+repr(str(root))+features+'}\n[[bin]]\nname="differential"\npath="main.rs"\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(p/'Cargo.toml'),'--target-dir',str(target/'proof')],check=True)
for name in ['cpp-scalar','cpp-default']:
    subprocess.run(['c++','-std=c++17','-O3','-DNDEBUG',*(['-DMESHOPTIMIZER_NO_SIMD'] if name=='cpp-scalar' else []),'-I',str(ref/'src'),str(root/'parity/0216-vdecode/differential.cpp'),str(ref/'src/vertexcodec.cpp'),'-o',str(target/(name+'-exact'))],check=True)
