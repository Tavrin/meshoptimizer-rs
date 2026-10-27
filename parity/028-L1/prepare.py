#!/usr/bin/env python3
"""Export frozen comparator and v0.2.7 without touching any other worktree."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import subprocess, tarfile, io, shutil, json, hashlib
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-028-L1'))
LANE=ROOT/'parity/028-L1'
def archive(ref, paths, dest):
    data=subprocess.check_output(['git','-C',_public_path('<repo>'),'archive',ref,*paths])
    dest.mkdir(parents=True,exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(data)) as t:t.extractall(dest,filter='data')
if __name__=='__main__':
    assert not (ART/'inputs.json').exists(), 'retain existing preparation; do not overwrite'
    archive('bench/peer-screen',['parity/compare'],ART)
    archive('v0.2.7',['src','Cargo.toml','Cargo.lock'],ART/'baseline')
    source=ART/'parity/compare'
    # The committed job adapters already include the equalization. Preserve them.
    original=ART/'original-adapter'
    shutil.copytree(source/'src',original/'src')
    for f in ['Cargo.toml','Cargo.lock']:shutil.copy2(source/f,original/f)
    p=original/'Cargo.toml';p.write_text(p.read_text().replace('path = "../.."',f'path = "{ROOT}"'))
    subprocess.run(['cargo','generate-lockfile','--offline','--manifest-path',str(p)],check=True)
    frozen=Path(_public_path('<scratch>/meshopt-peer-screen/campaign-20261025T150615Z'))
    (ART/'inputs').mkdir(exist_ok=True)
    expected=json.loads((source/'results.json').read_text())['outputs']
    records=[]
    for name in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']:
        p=frozen/'inputs'/f'{name}.bin';dst=ART/'inputs'/p.name;shutil.copy2(p,dst)
        assert hashlib.sha256(p.read_bytes()).hexdigest()==next(r['input_sha256']for r in expected if r['case']==name and r['operation']=='vertex-decode')
        records.append({'case':name,'source':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
    (ART/'inputs.json').write_text(json.dumps(records,indent=2)+'\n')
    (ART/'outputs').mkdir(exist_ok=True)
