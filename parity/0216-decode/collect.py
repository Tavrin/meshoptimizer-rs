"""Retain text receipts and source/binary bindings before deleting build output."""
import hashlib,json,os,shutil
from pathlib import Path
root=Path.cwd();art=Path(os.environ['DECODE_ART']);out=root/'parity/0216-decode/receipts';out.mkdir(exist_ok=True)
replacements={str(root):'<repo>',str(Path(os.environ['CARGO_TARGET_DIR'])):'<target-dir>',str(art):'<artifacts>',str(Path(os.environ['MESHOPT_REFERENCE'])):'<reference>'}

def clean(text):
    for a,b in sorted(replacements.items(),key=lambda x:-len(x[0])):text=text.replace(a,b)
    text=text.replace(str(Path.home()),'<home>')
    return text

for name in ['baseline-counts.jsonl','baseline-exit.json','wasm-baseline.json','wasm-default-baseline.json','proof-exits.json','differential.json','differential-no-std.json','build-binding.json','inputs.json','final-verdict.json','fuzz-level-counts.json','fuzz-binding.json','wasm-proof-binding.json']:
    p=art/name
    if p.exists():(out/name).write_text(clean(p.read_text()))
for folder in ['campaign-2','campaign-3','canonical','canonical-fresh']:
    dest=out/folder;dest.mkdir(exist_ok=True)
    for name in ['candidate-counts.jsonl','results.json','build-binding.json','candidate-exit.json','campaign-candidate.json','tuning-results.json','INVALID.md']:
        p=art/folder/name
        if p.exists():(dest/name).write_text(clean(p.read_text()))
# The first prototype uses the artifact root.
for name in ['candidate-counts.jsonl','results.json','candidate-exit.json','campaign-candidate.json','tuning-results.json','INVALID.md']:
    p=art/name
    if p.exists():(out/('prototype-1-'+name)).write_text(clean(p.read_text()))
log_names={x['step']+'.log' for x in json.loads((art/'proof-exits.json').read_text())}
log_names.update(['build-base.log','build-cpp-scalar.log','build-cpp-default.log'])
for name in sorted(log_names):
    p=art/name
    if p.exists():(out/name).write_text(clean(p.read_text()))
final=art/'proof-final';dest=out/'proof-final';dest.mkdir(exist_ok=True)
for p in final.iterdir():
    if p.suffix in ['.json','.log']:(dest/p.name).write_text(clean(p.read_text()))
# Bind compiled harnesses through Cargo's per-mode fingerprints and artifacts.
target=Path(os.environ['CARGO_TARGET_DIR']);builds=[]
for fp in target.rglob('bin-*.json'):
    if '.fingerprint' not in fp.parts:continue
    if not fp.parent.name.startswith(('decode-exact-','decode-differential-')):continue
    suffix=fp.parent.name.rsplit('-',1)[1];name='exact' if fp.name=='bin-exact.json' else 'differential' if fp.name=='bin-differential.json' else None
    if name is None:continue
    binary=fp.parent.parent.parent/'deps'/(name+'-'+suffix)
    if binary.exists():builds.append(dict(fingerprint=json.loads(fp.read_text()),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),harness=name))
(out/'compiled-harnesses.json').write_text(clean(json.dumps(builds,indent=2))+'\n')
manifest_sources={}
for directory in ['proof-source','proof-baseline','wasm-default-source','wasm-proof-source','differential-source']:
    d=target/directory
    for name in ['Cargo.toml','Cargo.lock']:
        p=d/name
        if p.exists():manifest_sources[directory+'/'+name]=clean(p.read_text())
(out/'scratch-manifests.json').write_text(json.dumps(manifest_sources,indent=2)+'\n')
source={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [*root.glob('src/**/*.rs'),root/'src/codec/simd/SAFETY.md',root/'tests/simd.rs',root/'.github/workflows/ci.yml',*[p for p in (root/'parity/0216-decode').iterdir() if p.suffix in ['.rs','.py','.sh','.cpp']]]}
(out/'final-source-binding.json').write_text(json.dumps(source,indent=2)+'\n')
manifest={str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest() for p in out.rglob('*') if p.is_file() and p.name!='SHA256SUMS.json'}
(out/'SHA256SUMS.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
print('retained',len(manifest),'text receipts')
