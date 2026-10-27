"""Retain source, dependency and binary bindings before target cleanup."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib,json,subprocess,os,sys
R=Path(__file__).resolve().parents[2];S=Path(_public_path('<scratch>/meshopt-0210-H2'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
sources={str(p.relative_to(R)):sha(p) for p in [*(R/'src').rglob('*.rs'),R/'Cargo.toml',R/'Cargo.lock']}
if sys.argv[1]=='before':
 (S/'build-sources-before.json').write_text(json.dumps(sources,indent=2)+'\n')
else:
 assert sources==json.loads((S/'build-sources-before.json').read_text()),'source changed during build'
 out=dict(sources=sources,binaries={a:sha(S/f'driver-{a}') for a in ['base','candidate','M7']},harnesses={a:{str(p.relative_to(S/f'harness-{a}')):sha(p) for p in (S/f'harness-{a}').rglob('*') if p.is_file()} for a in ['base','candidate','M7']},head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),admission=os.environ['PARITY_RUNNER_ACTIVE'])
 (S/'build-binding.json').write_text(json.dumps(out,indent=2)+'\n')
