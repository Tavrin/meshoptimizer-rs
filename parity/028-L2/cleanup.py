#!/usr/bin/env python3
"""Retain the qualified source/binaries and delete only this lane's target."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib,json,shutil,tarfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-028-L2'))
TARGET=Path(_public_path('<target-dir>'))
audit=json.loads((ART/'audit.json').read_text());assert audit['passed']
assert not TARGET.is_symlink() and TARGET.parent==Path(_public_path('<target-dir>'))
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in audit['source'].items())
generator=TARGET/'release/cook-seed-generator'
if generator.exists() and not (ART/'cook-seed-generator').exists(): shutil.copy2(generator,ART/'cook-seed-generator')
files=[p for p in ART.glob('driver-*') if p.is_file()]+[p for p in ART.glob('exact-*') if p.is_file()]
files += [p for p in [ART/'cpp-reference',ART/'seed-protocol-base',ART/'seed-protocol-candidate',ART/'rust-protocol',ART/'native-fixtures',ART/'p01x-native-fixtures',ART/'strict-overdraw',ART/'cook-seed-generator'] if p.exists()]
files += list((ART/'fuzz-artifacts/fuzz/binaries').glob('*'))
assert files
binaries={str(p.relative_to(ART)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
archive=ART/'final-source.tar.gz';assert not archive.exists()
with tarfile.open(archive,'w:gz') as tar:
    for name in ['src','fuzz/src','parity/028-L2','Cargo.toml','Cargo.lock','fuzz/Cargo.toml','fuzz/Cargo.lock','CHANGELOG.md']:
        tar.add(ROOT/name,arcname=name,filter=lambda item:None if '__pycache__' in item.name else item)
size=sum(p.stat().st_size for p in TARGET.rglob('*') if p.is_file()) if TARGET.exists() else 0
if TARGET.exists():shutil.rmtree(TARGET)
assert not TARGET.exists()
receipt=dict(deleted=str(TARGET),bytes_removed=size,absent=True,retained_binaries=binaries,source_sha256=audit['source'],source_archive=str(archive),source_archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),utc=time.time())
(ART/'cleanup.json').write_text(json.dumps(receipt,indent=2)+'\n')
(ROOT/'parity/028-L2/CLEANUP.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('Deleted',TARGET,'bytes',size,'retained binaries',len(binaries))
