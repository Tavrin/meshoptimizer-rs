"""Write isolated proof/build inputs; no compilation or measurement here."""

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
import io
import json
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-0212-simplify'))
ATTR = Path(_public_path('<scratch>/meshopt-0212-simp-attr'))
assert not ART.exists(), 'refuse to overwrite retained artifacts'
ART.mkdir(parents=True)
commit = subprocess.check_output(['git', '-C', str(ROOT), 'rev-parse', 'v0.2.11^{}'], text=True).strip()
archive = subprocess.check_output(['git', '-C', str(ROOT), 'archive', commit])
base = ART / 'base'
base.mkdir()
with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
    tar.extractall(base, filter='data')
# Cargo cannot resolve two path packages with the same name+version into a
# single proof consumer. Rename only the baseline package, preserving source.
proof_base = ART / 'proof-base'
shutil.copytree(base, proof_base)
p = proof_base / 'Cargo.toml'
p.write_text(p.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
proof = ART / 'proof'
proof.mkdir()
(proof / 'Cargo.toml').write_text(f'''[package]
name = "simplify-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = {{ package = "meshoptimizer-rs", path = "{ROOT}", default-features = false, features = ["experimental"] }}
baseline = {{ package = "meshoptimizer-baseline", path = "{proof_base}", default-features = false, features = ["experimental"] }}
[[bin]]
name = "simplify-exact"
path = "{ROOT}/parity/0212-simplify/exact.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
# Reuse the attribution lane's resident-loop driver, inputs and scalar M7
# manifest, but build every arm afresh. No attribution counts are reused.
shutil.copytree(ATTR / 'inputs', ART / 'inputs')
stages = json.loads((ATTR / 'stages.json').read_text())
for stage in stages:
    stage['input'] = str(ART / 'inputs' / Path(stage['input']).name)
    for field in ['vertex_blob', 'index_blob']:
        blob = Path(stage[field])
        with blob.open('rb') as stream:
            assert hashlib.file_digest(stream, 'sha256').hexdigest() == blob.stem
(ART / 'stages.json').write_text(json.dumps(stages, indent=2) + '\n')
for arm, source in [('base', base), ('candidate', ROOT), ('M7', None)]:
    harness = ART / f'harness-{arm}'
    shutil.copytree(ATTR / ('harness-M7' if arm == 'M7' else 'harness-base'), harness)
    p = harness / 'Cargo.toml'
    manifest = p.read_text()
    if source:
        manifest = manifest.replace(str(ATTR / 'src-copy'), str(source))
    # Preserve C++ NDEBUG/frame-pointer policy from the repaired M7 arm.
    if arm == 'M7':
        manifest = re.sub(r'debug = 1\nstrip = false', 'debug = 0\nstrip = false', manifest)
    p.write_text(manifest)
    # Stage TSVs refer to read-only retained blobs by absolute path. Freeze both
    # the TSV and its referenced blob hashes in the entry receipt.
inputs = {}
for p in (ART / 'inputs').glob('*'):
    inputs[str(p)] = hashlib.sha256(p.read_bytes()).hexdigest()
for name in ['stage-blobs.json', 'binding.json', 'M7-REPAIR.json']:
    shutil.copy2(ATTR / name, ART / ('attribution-' + name))
(ART / 'ENTRY.json').write_text(json.dumps({
    'baseline_commit': commit,
    'candidate_head': subprocess.check_output(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], text=True).strip(),
    'candidate_diff': subprocess.check_output(['git', '-C', str(ROOT), 'diff'], text=True),
    'inputs': inputs,
    'attribution_driver_sha256': hashlib.sha256((ATTR / 'harness-base/src/main.rs').read_bytes()).hexdigest(),
    'hold_release': 'must be explicitly acknowledged before admission',
}, indent=2) + '\n')
