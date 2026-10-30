"""Summarize completed checks and bind the exact reviewed files, without Git writes."""
import hashlib
import json
import os
import subprocess
from pathlib import Path

root=Path.cwd(); receipt=root/'parity/0217-hsr/eval'
report=Path(os.environ['HSR_EVAL_REPORT'])
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
rows=json.loads((receipt/'summary.json').read_text())
assert len(rows)==9 and all(r['changed_pixels']==0 for r in rows)
assert len(json.loads((receipt/'prior-scene-identity.json').read_text()))==9
oracle=json.loads((receipt/'oracle.json').read_text());assert oracle['observations']==1200
codes={}
for folder in [receipt/'checks',receipt]:
    for line in (folder/'exits.tsv').read_text().splitlines():
        name,code,*_=line.split('\t'); codes[str(folder.relative_to(receipt))+'/'+name]=int(code)
assert all(code==0 for code in codes.values()),codes
required=['fmt','clippy-all','clippy-none','tests-all','tests-none','wasm-all','wasm-none','boundary','exact-all','exact-none','hidden-release','docs-all','public-text']
assert all(codes.get('checks/'+name)==0 for name in required)
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()=='acc06584a22b935bd141aca8bef378b23ba40a47'
subprocess.run(['git','--no-optional-locks','diff','--check'],check=True)
scene_metadata=json.loads((receipt/'scenes.json').read_text())
text='| Scene (input triangles) | Pixel size | Resolution | Removed | HSR seconds | Simplify seconds | HSR / simplify | Changed pixels / 8 views |\n'
text+='| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n'
for r in rows:
    name=next(s['title'] for s in scene_metadata if s['name']==r['scene'])
    size={'coarse':'side/64','default':'default (side/128)','fine':'side/256'}[r['density']]
    text+=f"| {name} ({r['triangles']:,}) | {size} | {r['resolution']} | {r['removed_percent']:.4f}% | {r['seconds']:.6f} | {r['simplify_seconds']:.6f} | {r['hsr_vs_simplify']:.2f}x | {r['changed_pixels']} |\n"
text+='\nRemoval reasons (duplicate / enclosed back-to-back / other occluded):\n\n'
text+='| Scene | Coarse | Default | Fine |\n| --- | --- | --- | --- |\n'
for scene in scene_metadata:
    cells=[]
    for density in ['coarse','default','fine']:
        r=next(r for r in rows if r['scene']==scene['name'] and r['density']==density)['report']
        cells.append(f"{r['duplicate']} / {r['back_to_back']} / {r['occluded']}")
    text+=f"| {scene['title']} | "+' | '.join(cells)+' |\n'
low=all(r['removed_percent']<1 for r in rows if r['density']=='default')
if low:
    recommendation='''Retain the default pixel size. Default removal is below 1% on every tested scene,
so these scenes do not establish a broadly worthwhile default scene-reduction pass.
The feature remains useful for exact duplicate removal and geometry enclosed by
sufficiently large opaque occluders (the nested-shell regression establishes that
case). A finer density should be a caller choice based on measured savings and
cost; these bounded measurements do not justify silently changing the default.
'''
else:
    recommendation='''Retain the current default; make density an explicit per-scene choice. Default
removal is 42.5357% / 0.6340% / 20.6293% on Breakfast / Living / Conference,
so the all-scenes-below-1% condition is false. Fine density improves removal,
but Living Room's call grows from 2.286 to 20.105 seconds. No default or
visibility behavior is changed by this fix.

Keep this as opt-in offline exterior filtering. Default HSR takes 33.952 / 2.286 /
9.499 seconds versus simplify's 0.068 / 0.281 / 0.037 seconds: approximately
499x / 8.1x / 257x. Routine cook-time performance remains unresolved; this
scene matrix does not meet a 3-second and 3x-simplify speed goal. Simplify has
a different geometric contract, so these ratios are cost comparisons rather
than quality-equivalent algorithm substitutions. Runtime rendering benefits
were not measured.

These indoor scenes have exterior shells, as the inspected default-depth
contact sheet shows. Outside-view savings do not establish visibility from
cameras inside the rooms. For interior navigation, callers must supply accessible
finite viewpoints and qualify that different mode separately. This evaluation
covers opaque, static outside visibility; it does not justify general scene
culling for arbitrary camera paths.
'''
text+='\n### Recommendation\n\n'+recommendation
text+='\n### Gates and identity\n\n'
text+='All native tests (all features and no default features), Clippy `-D warnings`\n'
text+='in both modes, formatting, doctests, warnings-denied documentation, WASM builds,\n'
text+='unsafe/package boundary and release hidden-surface tests exited 0. WASM is build\n'
text+='evidence only. The default-SIMD WASM build retains existing dispatch dead-code\n'
text+='warnings; the no-default-feature WASM build passes.\n\n'
text+='The unchanged exact preprocessing/repair harness passes in both feature modes\n'
text+='(87,272 observations per mode). The unchanged independent C++ sampled-visibility\n'
text+='oracle matches the retained original ray implementation; conservative raster\n'
text+='results contain its kept set. All 80 scenes x 3 subdivision settings x 5 arms\n'
text+='pass (1,200 observations): original rays, pre-fix raster HEAD, candidate default,\n'
text+='candidate no-default and candidate parallel. All raster arms are byte-identical.\n'
text+='Pre-fix HEAD also produces identical indices on all nine complete scene/density\n'
text+='cases. This validates no visibility behavior change in the measured matrix.\n\n'
text+='The first admitted gate startup exited 1 before executing checks because the\n'
text+='harness expected the optional MOSS_HEAVY_GPU environment override. bootstrap.json\n'
text+='retains this failure. The corrected harness still requires active admission;\n'
text+='the complete gates rerun inside the already-queued evaluation job.\n\n'
text+='The first full gate attempt failed its final public-text check on hard-coded\n'
text+='local evaluation paths. checks-attempt1 retains that complete attempt. Paths\n'
text+='are now explicit environment inputs; the full gate suite was rerun.\n\n'
text+='The first oracle phase exited 101 while building the original ray driver,\n'
text+='whose API predates pixel_size. oracle-attempt1 retains that failure. The\n'
text+='original driver is now used for rays; the full oracle/scene identity phase\n'
text+='was rerun through admission. Earlier harness failures were not cancellations.\n\n'
text+='The next oracle startup exited 1 after a successful pre-fix build because\n'
text+='the source directory and executable shared a path. oracle-attempt2 retains\n'
text+='that failure; distinct executable names and path preflight fix the collision.\n'
text+='The complete oracle and scene identity phase was rerun.\n\n'
text+='The render positive control detects 151,800 changed pixels when an exposed\n'
text+='triangle is deleted. The default-depth contact sheet was visually inspected;\n'
text+='the before/after exterior shells match. All 72 original/filtered view pairs\n'
text+='across the nine density cases have zero changed 16-bit depth pixels.\n\n'
text+='Raw timing samples, complete removal reports, visual hashes, source identities,\n'
text+='gate logs and exit codes are retained in `parity/0217-hsr/eval/`. No failed check\n'
text+='has been reclassified as passing, and no queued job was canceled.\n\n'
text+='```text\n'
for name,code in codes.items():text+=f'{name}: {code}\n'
text+='```\n'
s=report.read_text()
start=s.index('## Evaluation\n\n')+len('## Evaluation\n\n')
end=s.index('\n## Visual check boundary',start)
s=s[:start]+text.rstrip()+'\n'+s[end:]
report.write_text(s)
# Bind unchanged source files to HEAD and all implementation/evaluator sources.
unchanged={}
for p in sorted((root/'src').rglob('*.rs')):
    relative=str(p.relative_to(root)); original=subprocess.check_output(['git','show','HEAD:'+relative])
    if relative!='src/hidden.rs':
        assert p.read_bytes()==original,relative
        unchanged[relative]=sha(p)
files=[root/'src/hidden.rs',root/'tests/hidden.rs',root/'Cargo.toml',root/'Cargo.lock',*receipt.glob('*.rs'),*receipt.glob('*.py'),root/'parity/0217-hsr/oracle.cpp',root/'parity/0217-hsr/exact.rs',root/'parity/0217-hsr/speed/bench.rs',root/'parity/0217-hsr/speed/before.rs']
binding=dict(head='acc06584a22b935bd141aca8bef378b23ba40a47',
    source={str(p.relative_to(root)):sha(p) for p in files},unchanged_sources=unchanged,
    prior_hidden_sha256=hashlib.sha256(subprocess.check_output(['git','show','HEAD:src/hidden.rs'])).hexdigest(),
    report_sha256=sha(report),report_path=report.name)
(receipt/'source-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
(receipt/'implementation.diff').write_bytes(subprocess.check_output(['git','--no-optional-locks','diff','--','src/hidden.rs','tests/hidden.rs']))
print(text)
print('PASS final report, source binding and whitespace; HEAD unchanged')
