"""Build the attribution harness for all three fresh arms in one admission."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess

ART = Path(_public_path('<scratch>/meshopt-0212-simplify'))
assert os.environ.get('PARITY_RUNNER_ACTIVE')
assert os.environ['PARITY_RUNNER_GPU'] == '0'
assert os.environ['CARGO_TARGET_DIR'] == _public_path('<target-dir>')
ROOT = Path(__file__).resolve().parents[2]
source_files = {str(p): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', *sorted((ROOT / 'src').rglob('*'))]
                if p.is_file()}
(ART / 'candidate-source.json').write_text(json.dumps(source_files, indent=2) + '\n')
versions = {}
for tool in ['rustc', 'cargo', 'c++']:
    versions[tool] = subprocess.check_output([tool, '--version'], text=True)
(ART / 'tool-versions.json').write_text(json.dumps(versions, indent=2) + '\n')
for arm in ['base', 'candidate', 'M7']:
    log = ART / f'build-{arm}.log'
    assert not log.exists()
    command = ['cargo', 'build', '--offline', '--locked', '--manifest-path',
               str(ART / f'harness-{arm}/Cargo.toml'), '--profile', 'bench-release',
               '--bin', 'cmp-driver', '--features', 'peer7' if arm == 'M7'
               else 'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod']
    if arm == 'M7':
        command.append('-vv')
    with log.open('x') as stream:
        result = subprocess.run(command, env=dict(os.environ, CC_ENABLE_DEBUG_OUTPUT="1"), stdout=stream, stderr=subprocess.STDOUT)
    receipt = {'arm': arm, 'command': command, 'exit': result.returncode,
               'admission': os.environ['PARITY_RUNNER_ACTIVE']}
    if result.returncode == 0:
        binary = ART / f'driver-{arm}'
        shutil.copy2(Path(os.environ['CARGO_TARGET_DIR']) / 'bench-release/cmp-driver', binary)
        receipt['sha256'] = hashlib.sha256(binary.read_bytes()).hexdigest()
    (ART / f'build-{arm}.json').write_text(json.dumps(receipt, indent=2) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
