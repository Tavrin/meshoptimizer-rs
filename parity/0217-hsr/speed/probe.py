"""One admitted large-mesh speed probe before the full paired campaign."""
import json,os,shutil,subprocess
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr/speed';target=Path(os.environ['CARGO_TARGET_DIR']);folder=target/'probe';folder.mkdir(parents=True,exist_ok=True)
(folder/'Cargo.toml').write_text(f'''[package]
name="hsr-probe"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path={json.dumps(str(root))}}}
[[bin]]
name="hsr-probe"
path={json.dumps(str(lane/'bench.rs'))}
[profile.release]
codegen-units=1
lto="fat"
''')
subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(folder/'Cargo.toml')],check=True)
subprocess.run([str(target/'release/hsr-probe'),'hidden',str(Path(os.environ['HSR_INPUTS'])/'real-a082.bin'),str(folder/'out')],input='1\nstop\n',text=True,check=True)
