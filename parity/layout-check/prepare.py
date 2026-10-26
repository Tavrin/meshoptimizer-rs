
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import subprocess,tarfile,io,shutil,json,hashlib
A=Path(__file__).parent;F=A.parent/'fix1';R=Path.cwd()
for name,rev in [('C','a97301e'),('B','775ff8c'),('Bprime','775ff8c')]:
 lib=A/f'lib-{name}';lib.mkdir();tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git','archive',rev]))).extractall(lib,filter='data')
 for variant in (['default','pinned'] if name!='Bprime' else ['default']):
  h=A/f'{name}-{variant}-harness';shutil.copytree(F/'old-harness',h)
  p=h/'Cargo.toml';s=p.read_text().replace(str(R),str(lib));p.write_text(s)
# use fixed-region probe sources and manifests unchanged except dependency paths
for name,source in [('C','after-old'),('B','v024')]:
 h=A/f'probe-{name}-src';shutil.copytree(F/source,h)
 p=h/'Cargo.toml';s=p.read_text();import re
 s=re.sub(r'path="[^"]+"',f'path="{A/f"lib-{name}"}"',s);p.write_text(s)
# adapt only arm selection and evidence root, leaving timing loop unchanged
s=(F/'measure-case.py').read_text().replace(_public_path("A=Path('<scratch>/meshopt-validate-once/fix1')"),f'A=Path({str(A)!r})')
s=s.replace("A.parent/'inputs'","A.parent/'inputs'")
s=s.replace("A/('new-driver'if comparison=='new-M7'else'old-driver')","A/('Bprime-default' if comparison=='AA' else 'C-'+comparison)")
s=s.replace("A/('baseline-M7'if comparison=='new-M7'else'baseline-O')","A/('B-default' if comparison=='AA' else 'B-'+comparison)")
s=s.replace("sha(A/(('new-driver'if comparison=='new-M7'else'old-driver')if k=='A'else'baseline-M7'if comparison=='new-M7'else'baseline-O'))","sha(A/(('Bprime-default' if comparison=='AA' else 'C-'+comparison) if k=='A' else ('B-default' if comparison=='AA' else 'B-'+comparison)))")
(A/'measure.py').write_text(s)
b=_public_path('''#!/bin/bash
set -eu
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= CARGO_ENCODED_RUSTFLAGS=
A=<scratch>/meshopt-validate-once/layout-check
''')
for name in ['B','Bprime','C']:
 for v in (['default','pinned'] if name!='Bprime' else ['default']):
  # linker map records symbols without changing codegen flags
  flags='' if v=='default' else '-C llvm-args=-align-all-functions=6'
  b+=f'export RUSTFLAGS="{flags}"\ncargo rustc --manifest-path "$A/{name}-{v}-harness/Cargo.toml" --offline --profile moss --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/{name}-{v}.map\ncp "$CARGO_TARGET_DIR/moss/cmp-driver" "$A/{name}-{v}"\n'
for name,binname in [('C','probe-after-old'),('B','probe-v024')]:
 b+=f'export RUSTFLAGS=\ncargo build --manifest-path "$A/probe-{name}-src/Cargo.toml" --offline --release\ncp "$CARGO_TARGET_DIR/release/{binname}" "$A/probe-{name}"\n'
(A/'build.sh').write_text(b)
(A/'registration-commit.txt').write_text(subprocess.check_output(['git','rev-parse','HEAD'],text=True))
