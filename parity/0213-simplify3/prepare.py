"""Freeze release sources, exact consumer and the unchanged release harness."""
from pathlib import Path
import hashlib
import io
import json
import os
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
ATTR = Path(os.environ['MESHOPT_ATTRIBUTION'])
commit = subprocess.check_output(['git', 'rev-parse', 'v0.2.12^{}'], text=True).strip()
assert commit.startswith('fe108a0') or subprocess.check_output(['git', 'diff', commit, 'fe108a0', '--', 'src'], text=True) == ''
base = ART / 'base'
base.mkdir()
with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git', 'archive', commit]))) as tar:
    tar.extractall(base, filter='data')
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
path = "{ROOT}/parity/0213-simplify3/exact.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
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
    manifest = re.sub(r'\[profile\.[a-z]+\]', '[profile.bench-release]', manifest, count=1)
    if arm == 'M7':
        manifest = manifest.replace('debug = 1', 'debug = 0')
    p.write_text(manifest)
inputs = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in (ART / 'inputs').iterdir() if p.is_file()}
for name in ['stage-blobs.json', 'binding.json', 'M7-REPAIR.json']:
    shutil.copy2(ATTR / name, ART / ('attribution-' + name))
(ART / 'ENTRY.json').write_text(json.dumps(dict(baseline_commit=commit,
    candidate_head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    inputs=inputs, attribution_driver_sha256=hashlib.sha256((ATTR/'harness-base/src/main.rs').read_bytes()).hexdigest()), indent=2)+'\n')
