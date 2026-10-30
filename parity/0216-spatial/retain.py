"""Retain public, path-neutral receipts after the complete campaign finishes."""
import hashlib, json, math, os, re, shutil, subprocess
from pathlib import Path
R=Path(__file__).resolve().parents[2]
L=Path(__file__).resolve().parent
S=Path(os.environ['MESHOPT_SCRATCH'])
T=Path(os.environ['CARGO_TARGET_DIR'])
A=S/'art'
def sha(p):
    with Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
measure=json.loads((A/'measurements.json').read_text())
diff=json.loads((A/'differential.json').read_text())
assert measure['exit']==diff['exit']==0
# Confirm every frozen Rust source still equals the edited worktree.
files=sorted((R/'src').rglob('*.rs'))
for p in files:
    frozen=(S/'candidate'/p.relative_to(R)).read_text()
    if p==R/'src/lib.rs': frozen=frozen.replace('EXT filters meshlet decoding','EXT filters, meshlet decoding')
    assert p.read_text()==frozen,p
doc_delta=dict(file='src/lib.rs',compiled_sha256=sha(S/'candidate/src/lib.rs'),final_sha256=sha(R/'src/lib.rs'),
    change='crate documentation punctuation only; executable tokens unchanged')
binding={str(p.relative_to(R)):sha(p) for p in [*files,R/'Cargo.toml',R/'Cargo.lock',L/'run.py',L/'retain.py',L/'exact.rs',L/'wasi.cjs',R/'src/codec/simd/SAFETY.md',R/'parity/simd/check_package.py']}
final=dict(doc_only_delta=doc_delta,base='1ecf69abe9f92c17e13dc38fa97be659aee44582',reference='4c203430ca565cb59a468a91922c76c208169536',sources=binding,
    binaries={p.name:sha(p) for p in A.iterdir() if p.is_file() and (p.name.startswith(('driver-','p03-','exact-','transport128-','diagnostic-driver-')) or p.suffix=='.wasm') and p.suffix not in ['.log','.json','.cpp']},
    rustc=subprocess.check_output(['rustc','-Vv'],text=True),
    node=subprocess.check_output(['node','--version'],text=True).strip(),
    perf=subprocess.check_output([os.environ['PEERS_PERF'],'--version'],text=True).strip(),
    cpu_model=next(line.split(':',1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')))
final['frozen_drivers']={str(p.relative_to(S)):sha(p) for tree in ['base','candidate'] for sub in ['parity/peers','parity/p03'] for p in (S/tree/sub).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.lock']}
final['transport_cpp_sha256']=sha(A/'p03-reference.cpp')
ref=Path(os.environ['MESHOPT_REFERENCE'])
final['cpp_sources']={str(p.relative_to(ref)):sha(p) for p in (ref/'src').glob('*') if p.suffix in ['.cpp','.h']}
vendors=list((Path.home()/'.cargo/registry/src').glob('*/meshopt-0.7.0/vendor/src/clusterizer.cpp'))
assert len(vendors)==1 and sha(vendors[0])==sha(ref/'src/clusterizer.cpp')
final['peer_cpp_clusterizer_sha256']=sha(vendors[0])
(L/'final-binding.json').write_text(json.dumps(final,indent=2)+'\n')
for name in ['counts.jsonl','measurements.json','inputs.json','differential.json','source-binding.json','build-binding.json','exits.jsonl','stage-exits.jsonl']:
    shutil.copy2(A/name,L/name)
receipts=L/'receipts';receipts.mkdir(exist_ok=True)
substitutions={str(R):'<repo>',str(S):'<scratch>',str(T):'<target>',os.environ['MESHOPT_REFERENCE']:'<reference>',str(Path.home()):'<home>'}
logs={}
for p in sorted(A.glob('*.log')):
    s=p.read_text()
    for old,new in sorted(substitutions.items(),key=lambda x:-len(x[0])):s=s.replace(old,new)
    dest=receipts/p.name;dest.write_text(s)
    logs[p.name]=dict(raw_sha256=sha(p),public_sha256=sha(dest))
for p in sorted((A/'samples').glob('*.perf')):
    shutil.copy2(p,receipts/p.name)
(L/'receipt-binding.json').write_text(json.dumps(logs,indent=2)+'\n')
sizes={mode:sum(map(int,re.findall(r'limits=(\d+)',(A/('exact-size-'+mode+'.log')).read_text()))) for mode in ['none','all']}
lines=['# Spatial SIMD qualification','',
    'Verdict: **'+measure['verdict']+'**. Ratios below use median instructions:u, three samples per arm.',
    '','| Case | Default / v0.2.15 | Scalar / scalar C++ | Default / default C++ |',
    '|---|---:|---:|---:|']
for c in measure['cells']:
    lines.append(f"| {c['case']} | {c['vs_base']:.6f} | {c['scalar_vs_cpp']:.6f} | {c['default_vs_cpp']:.6f} |")
g=measure['real_geomean']
lines.append(f"| **Real-mesh geometric mean** | **{g['vs_base']:.6f}** | **{g['scalar_vs_cpp']:.6f}** | **{g['default_vs_cpp']:.6f}** |")
lines+=['','SHIPS requires real-mesh default/base <=0.95 and every case <=1.01.',
    'The C++ wrapper uses version 1.3, with clusterizer.cpp identical to the pinned oracle.',
    'The timed Rust driver enables std/simd and clusterlod, with diagnostic parity-internals disabled.',
    'Scalar C++ explicitly defines MESHOPTIMIZER_NO_SIMD; default C++ enables its SSE path.',
    'All floating-point C++ builds use -fno-fast-math -ffp-contract=off.',
    '','Wall seconds per call (disclosure only):','',
    '| Case | Base | Default | Scalar | Scalar C++ | Default C++ |','|---|---:|---:|---:|---:|---:|']
for c in measure['cells']:
    lines.append('| '+c['case']+' | '+' | '.join(f"{c['wall_seconds'][a]:.6f}" for a in ['base','candidate','scalar','cpp-scalar','cpp-default'])+' |')
lines+=['',f"Performance: {measure['samples']} samples, CPU {measure['cpu']}; every counter reports 100% running.",
    'Every measured arm and repetition has the same output hash within each case.',
    '',f"Exact limits: {sizes['none']:,} no-feature cells and {sizes['all']:,} SIMD/forced-ceiling cells.",
    'Each cell compares base/candidate owned and caller-buffer APIs twice, with complete errors, Usage and sentinels.',
    'Cold, successful-prime and invalid-index-prime states; 13 inputs and four settings.',
    'Both exact runs exit 0; execution CPU totals are in exits.jsonl and are below two hours.',
    '',f"C++/native/wasm differential: {diff['corpus_cases']} corpus/boundary messages plus three full real meshes.",
    'Native forced Scalar/SSE2/SSSE3/SSE4.1, no-feature Rust, scalar/default C++, and scalar/SIMD wasm all agree.',
    'Both wasm artifacts execute in Node. Three focused direct SIMD kernel tests also execute under Node WASI.',
    'NEON compiles for aarch64-unknown-linux-gnu; no native/user-mode runner is installed, so execution is unverified.',
    'No new sanitizer or Miri run was performed; finite tests supplement the structural proof.',
    '',
    'Full tests (including doctests), all-features and no-default-features: exit 0. Clippy, fmt, repository/package boundary,',
    'public-text, AArch64 check and -Funsafe_code no-feature check: exit 0. See receipts and exits.jsonl.',
    'Unsafe inventory: 23 -> 29 blocks (+6), one allowance. No non-SIMD unsafe code.',
    '',
    'The first frozen-candidate build exited 101 because the harness omitted included documentation.',
    'The harness copy was corrected. A later real-mesh differential exceeded the inherited C++ protocol 128 MiB cap.',
    'Only scratch transport limits were widened to 512 MiB for all protocol arms; algorithms are unchanged.',
    'That failed stage, original corpus and original C++ binaries are retained. The corrected full differential passed.',
    '',
    'The final crate doc comment corrects one comma after the frozen build; executable tokens are unchanged, and both source hashes are retained.',
    'Source, binary, input, output, raw-log and sanitized-log hashes are retained in the binding JSON files.',
    'The complete raw artifacts and executables remain under the caller-supplied scratch directory; only the dedicated',
    'Cargo target is removed at completion. Git commits, refs and index are not intentionally modified.',
    '',
    'Reproduction: set MESHOPT_SCRATCH, CARGO_TARGET_DIR, MESHOPT_REFERENCE, MESHOPT_INPUTS (a directory containing',
    'inputs/ and inputs.json), and PEERS_PERF; run run.py stages prepare, checks, build, exact, p03build, differential,',
    'and measure sequentially through the prescribed CPU admission wrapper. Run retain.py after completion.',
    'PROOFS.md contains the arithmetic, memory and failure-state argument.']
(L/'REPORT.md').write_text('\n'.join(lines)+'\n')
print(measure['verdict'],json.dumps(g))

