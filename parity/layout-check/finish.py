
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import json,hashlib,datetime,shutil,subprocess
A=Path(__file__).parent;R=Path(_public_path('<repo>'));D=R/'parity/layout-check';T=Path(_public_path('<target-dir>'))
assert (A/'COMPLETE.json').exists()
assert T.is_dir() and not T.is_symlink()
executables={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in A.iterdir() if p.is_file() and p.read_bytes()[:4]==b'\x7fELF'}
assert all(str(A/p) in executables for p in ['C-default','B-default','Bprime-default','C-pinned','B-pinned','probe-C','probe-B'])
(A/'RETAINED-EXECUTABLES.json').write_text(json.dumps(executables,indent=2))
size=sum(p.stat().st_size for p in T.rglob('*') if p.is_file());shutil.rmtree(T)
(A/'CLEANUP.json').write_text(json.dumps(dict(target=str(T),bytes_removed=size,absent=not T.exists(),utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),executables_retained=True),indent=2))
assert not subprocess.check_output(['git','diff','a97301e','--','src/'],cwd=R)
report=A/'REPORT.md';s=report.read_text();s+='\nCleanup: designated Cargo target deleted after executable retention; see CLEANUP.json and RETAINED-EXECUTABLES.json.\n';report.write_text(s)
for name in ['REPORT.md','CLEANUP.json','RETAINED-EXECUTABLES.json']:shutil.copy2(A/name,D/name)
manifest={str(p.relative_to(A)):hashlib.sha256(p.read_bytes()).hexdigest() for p in A.rglob('*') if p.is_file() and '__pycache__' not in p.parts and not p.name.endswith('.bin')}
(A/'EVIDENCE-SHA256SUMS.json').write_text(json.dumps(manifest,indent=2));shutil.copy2(A/'EVIDENCE-SHA256SUMS.json',D/'EVIDENCE-SHA256SUMS.json')
print(json.dumps(dict(cleanup=True,bytes_removed=size,retained_executables=len(executables))))
