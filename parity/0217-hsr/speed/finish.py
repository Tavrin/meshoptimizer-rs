"""Bind completed speed evidence. Call only after the last admitted job exits."""
import hashlib,io,json,os,re,subprocess,tarfile
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'speed';target=Path(os.environ['CARGO_TARGET_DIR'])
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
performance=json.loads((receipt/'performance.json').read_text());assert performance['gate']
(receipt/'environment.json').write_text(json.dumps(dict(
    rustc=subprocess.check_output(['rustc','-vV'],text=True),
    perf=subprocess.check_output([os.environ['HSR_PERF'],'--version'],text=True).strip(),
    counter='instructions:u',locale='C',cpu=performance['cpu'],
    build=dict(profile='release',codegen_units=1,lto='fat',jobs=2),
    workspace_max_bytes=3<<30,features=['std,simd','no default features','std,simd,parallel'],
    sampling=dict(outside_directions=26,subdivisions=2,pixel_size=0,
        auto_pixel_width='longest referenced side / 128',square_resolution=256),
    simplify=dict(target='half index count rounded down to triangles',relative_error=0.5,flags=0)
),indent=2)+'\n')
oracle=json.loads((receipt/'oracle.json').read_text());assert oracle['observations']==960
old_binding=json.loads((lane/'receipts/source-binding.json').read_text())
assert sha(receipt/'before.rs') == old_binding['source']['src/hidden.rs']
for name in ['tests-all','tests-none','clippy-all','clippy-none','wasm-all','wasm-none','boundary','exact-all','exact-none','hidden-release','public-text']:
    assert any(line.startswith(name+'\t0\t') for line in (receipt/'checks/exits.tsv').read_text().splitlines()),name
for name in ['final-hidden-all','final-hidden-none','final-clippy-all','final-clippy-none','final-pinned-scaling']:
    assert any(line.startswith(name+'\t0\t') for line in (receipt/'exits.tsv').read_text().splitlines()),name
base='09da597f438471d0ad9c16e460f3598e9356e9ea';unchanged={}
assert subprocess.check_output(['git','--no-optional-locks','rev-parse','HEAD'],text=True).strip()==base
with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git','archive',base,'src']))) as t:
    for member in t.getmembers():
        if member.isfile() and member.name.endswith('.rs') and member.name!='src/lib.rs':
            data=t.extractfile(member).read();assert data==(root/member.name).read_bytes(),member.name
            unchanged[member.name]=hashlib.sha256(data).hexdigest()
proof=receipt/'PROOFS.md';s=proof.read_text().split('## Receipts',1)[0]+'## Receipts\n\n'
s+='''All original F8 behaviors pass, plus new pixel-option/atomicity, subpixel
aperture, legacy shared-edge rounding, hidden/finite-view limit recovery, and
1/2/4-thread pool determinism tests. The final release suite has 20 tests with
all features and 19 without default features. All-feature and no-default
unit/integration tests and doctests, Clippy with warnings denied, both WASM
builds, unsafe/package boundary, exact baseline comparisons and public-text
checks pass. WASM was built, not executed. No new unsafe code.

The exact drivers retain the original 87,272 observations per feature mode,
174,544 total, checking old APIs' bytes, errors, output tails and Usage across
all work limits and byte boundaries. Other algorithm sources remain byte-identical
to v0.2.16; their retained C++ cache qualification is unaffected by this extension.
The independent C++ HSR oracle is unchanged. Eighty scenes x three densities
compare the saved ray implementation exactly with C++; the three candidate
feature modes must keep a superset and agree exactly with each other: 960 total
comparisons. Full real/corpus outputs also agree in all three candidate modes,
and all old-kept real triangles remain in the candidate output.

Three repetitions use one pinned CPU, acknowledged perf controls and
instructions:u around warm resident calls. Arms are interleaved and reversed
in repetition two. Parsing, serialization and warmup are excluded; allocations,
classification and destruction remain counted. Simplify targets half the input
index count, relative error 0.5, no flags, on the identical complete input.
The speed gate uses wall time; counters are retained alongside it. All completed
observations and initial failed fixture attempts are preserved.

| Input | Before s | After s | Simplify s | HSR/simplify | Before removed | After removed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
'''
for r in performance['rows']:
    if not r['case'].startswith('real-'):continue
    a=r['after-hidden'];b=r['before-hidden'];c=r['after-simplify'];n=a['report']['input']
    s+=f"| {r['case']} | {b['seconds']:.6f} | {a['seconds']:.6f} | {c['seconds']:.6f} | {r['vs_simplify']:.6f} | {b['report']['removed']} ({100*b['report']['removed']/n:.6f}%) | {a['report']['removed']} ({100*a['report']['removed']/n:.6f}%) |\n"
scale_record=json.loads((receipt/'scaling.json').read_text());scaling=scale_record['summary']
assert scale_record['work_gate'] and scale_record['wall_gate']
assert scale_record['calls_per_batch']==256
s+=f'''
The release N=128 versus 8N=1024 fixture charges 2,816 versus 25,600 work
units: **9.090909x** (assertion <=10x). The larger N=32768 versus 8N=262144
fixture charges {scaling['32768']['work']} versus {scaling['262144']['work']}:
**{scaling['work_ratio']:.6f}x** work and **{scaling['wall_ratio']:.6f}x** median
wall time ({scaling['32768']['seconds']:.9f} versus {scaling['262144']['seconds']:.9f} s).
The larger final scaling samples use 256-call resident batches on the same
pinned CPU as the real-mesh timings; all three repetitions are retained.
An initial pinned single-call measurement (about 2 ms at N) reported 15x wall
scaling; it is retained in `scaling-pinned-single.json`, not silently discarded.
The longer batch protocol amortizes the short-call clock/allocator behavior
without selecting favorable pairs. Earlier unpinned observations remain in
`scaling-unpinned.json`; all completed samples are retained.
These separated tiny triangles cannot certify whole-pixel occlusion, so the
area filter skips raster allocation. This bounded result is not a worst-case
complexity claim. The raster deliberately trades removal aggressiveness for coverage
certainty: meshes dominated by subpixel triangles may retain most or all faces.
No promise of a minimum removal share is made.

`exits.tsv` and `checks/exits.tsv` retain gate exit codes. The initial
fixture-failure runs and first C++ differential failure are retained rather
than counted as passes. No queued job
was canceled. No other model, AI CLI, dependency change, Git integration or
publication was used. The dedicated fresh target is deleted only after all
admitted jobs have completed, with a separate cleanup receipt.
'''
proof.write_text(s)
p=lane/'PROOFS.md';s=p.read_text();prefix='''> Historical F8 proof record. The current outside-view implementation and speed
> qualification are documented in [speed/PROOFS.md](speed/PROOFS.md). The original
> receipts below remain intact; their timings and sampled-ray design describe
> the saved implementation, not the new software item buffer.

'''
if not s.startswith('> Historical'):p.write_text(prefix+s)
files=[*root.glob('src/**/*.rs'),root/'tests/hidden.rs',root/'Cargo.toml',root/'Cargo.lock',root/'README.md',root/'docs/USING.md',*receipt.glob('*.rs'),*receipt.glob('*.py'),proof]
(receipt/'source-binding.json').write_text(json.dumps(dict(base=base,unchanged_algorithm_files=unchanged,source={str(p.relative_to(root)):sha(p) for p in sorted(set(files))}),indent=2,sort_keys=True)+'\n')
print('PASS source binding, proof update and unchanged algorithm source checks')
