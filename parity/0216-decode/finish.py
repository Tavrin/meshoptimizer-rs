"""Finish the reviewable receipt set and delete only this lane's build target."""
import hashlib,json,math,os,shutil,statistics,subprocess
from pathlib import Path
root=Path.cwd();art=Path(os.environ['DECODE_ART']);lane=root/'parity/0216-decode';target=Path(os.environ['CARGO_TARGET_DIR']).resolve()
proof=json.loads((art/'proof-final/proof-exits.json').read_text());msrv=json.loads((art/'proof-final/msrv-exits.json').read_text())
assert all(r['exit']==0 for r in proof+msrv)
initial_cpu=sum(r['cpu_seconds'] for r in json.loads((art/'proof-exits.json').read_text()))
final_cpu=sum(r['cpu_seconds'] for r in proof+msrv)
assert initial_cpu+final_cpu<=7200
verdict=json.loads((art/'final-verdict.json').read_text());native=json.loads((art/'canonical-fresh/results.json').read_text());wasm=json.loads((art/'wasm-default-baseline.json').read_text())
g=lambda v:math.exp(statistics.mean(math.log(x) for x in v))
summary=[]
for op,wire in [('vertex-decode',1),('index-decode',2)]:
    ns=[r for r in native['rows'] if r['op']==op and r['case'].startswith('real-')]
    ws=[r for r in wasm['rows'] if r['op']==wire and r['case'].startswith('real-')]
    summary.append(dict(api=op,x86_instructions=g([r['baseline_default_ratio'] for r in ns]),wasm_wall=g([r['ratio'] for r in ws])))
(lane/'BASELINE.md').write_text('# Default-feature baseline\n\nReal-mesh geometric means of per-case medians (three samples). Ratios are Rust / C++ default.\n\n| Decoder | x86 instructions:u | wasm Node wall time (disclosure) |\n|---|---:|---:|\n'+''.join(f"| {r['api']} | {r['x86_instructions']:.4f} | {r['wasm_wall']:.4f} |\n" for r in summary)+'\nEvery native cell and the scalar C++ arm are in TABLE.md; raw samples and the default wasm adapter identities are retained in receipts/.\n')
p=lane/'PROOFS.md';s=p.read_text().replace('Local qualification is in progress. Native arm64 execution belongs to the\ncoordinator\'s CI phase; local cross-compilation cannot clear that gate.', 'Local qualification completed. Native arm64 execution belongs to the\ncoordinator\'s CI phase; local cross-compilation cannot clear that gate.')
s+='''
## Final verdict and receipts

SSSE3 stride-12 tuning is **REJECTED and reverted**. The correctly rebuilt
candidate reduced real-mesh vertex instructions to 0.910984x base, but its
worst index-decode case reached 1.014631x, above 1.01. Outlining did not remove
the regression. The final source is the measured unchanged-x86 control;
its executable is byte-identical to the v0.2.15 executable at the common
build path. No x86 or wasm tuning is retained or claimed. The wasm baseline
shows a bridge-inclusive loss, without sufficient attribution to justify a
kernel change. The final runtime change is the full NEON path only.

Default-build ratios are in BASELINE.md. TABLE.md describes the final
unchanged-x86 control; TUNING.md and the fresh campaign's tuning-results.json
retain the rejected candidate. Earlier path experiments and the explicitly
INVALID stale-binary experiment remain separate from qualified results.

The exact driver passed **3,505,440 observations per feature mode**, comparing
typed errors, owned results, whole caller buffers and Usage on cold, warm and
after-failure workspaces. Every small-fixture work limit and output byte-limit
boundaries were tested; 2,000 larger seeded/mutated cases supplement them.
Only the proof baseline package name was changed to avoid Cargo's same-name,
same-version path-package lockfile collision; its Rust sources were unchanged.
The initial lockfile failures are retained; final exact runs both exit zero.

Each native differential mode passed **318,608 cases** at scalar, SSE2, SSSE3
and SSE4.1 ceilings. Both std and no-std SIMD were executed; no-std's ISA
capabilities were compile-time enabled on this supported host. Rust results,
Usage in integration tests, and all destination bytes (including failure
prefixes and tails) match scalar. C++ scalar/default successful bytes and
normalized success/failure statuses match. **74,863 C++ failure-prefix
comparisons differ in each mode**: upstream stages failed blocks differently
from the unchanged Rust scalar decoder. These are observed inherited
boundaries, not C++ malformed-buffer byte-identity claims. The exact driver
and unchanged scalar source preserve the Rust contract.

Executed wasm passed **316,608** scalar/simd128 observations, including errors
and complete destinations. ASan completed a fixed **20,000** seeded runs;
Miri passed the supported integer-kernel subset. The final suite also passes
all-feature/no-default tests, no-std SIMD tests, Clippy, actual AArch64 library
builds with/without std, cross-target test compilation, unsafe-free build,
package verification, unsafe inventory, formatting, and public text checks.
Focused Rust 1.88 SIMD and no-default checks also pass. The owned decoder's
1 MiB staging threshold and staged destination tails have focused coverage.

Native NEON execution is **pending coordinator CI**, on both Linux arm64
and Apple arm64, for std/no-std forced-level and C++ differential matrices.
No ARM performance claim is made. Cross-compilation does not clear identity.

Four initial installed-supervisor attempts received SIGTERM immediately after
admission; they were not canceled by this lane. Subsequent jobs retained the
required CPU admission and GPU=0, using an invocation-local fixed 4 GiB
systemd cap instead of the failing optional elastic scope supervisor. No
installed runner configuration or queue steering was changed.
'''
s+=f"\nFinal proof steps: **{len(proof)}**, all exit **0**; MSRV steps: **{len(msrv)}**, both exit **0**. Final proof CPU: **{final_cpu:.2f} s**; including the retained initial proof attempt: **{initial_cpu+final_cpu:.2f} s**, below 7,200 s. `receipts/proof-final/` retains individual exits and logs.\n"
p.write_text(s)
(lane/'README.md').write_text('# Decoder lane\n\n- PROOFS.md: identity argument, limitations, verdict and proof exits.\n- BASELINE.md and TABLE.md: default-build measurements and final source.\n- TUNING.md: rejected stride-12 experiment.\n- REGISTRATION.md: unchanged gates and coverage.\n- receipts/: text evidence, source/binary hashes and build manifests.\n\nBuilds remain externally admitted. Set CARGO_TARGET_DIR, DECODE_ART, DECODE_INPUTS, PEERS_PERF and MESHOPT_REFERENCE explicitly. Baseline extraction uses read-only git archive; no integration commands are part of this lane. canonical_fresh.sh forces recompilation and applies the fixed verdict before proofs. The coordinator owns Git integration and native arm64 CI acceptance.\n')
subprocess.run(['python3','parity/0216-decode/collect.py'],check=True)
versions={tool:subprocess.check_output(args,text=True) for tool,args in [('stable',['rustc','-Vv']),('nightly',['rustc','+nightly','-Vv']),('msrv',['rustc','+1.88','-Vv'])]}
(lane/'receipts/toolchains.json').write_text(json.dumps(versions,indent=2)+'\n')
subprocess.run(['bash','parity/check-reference.sh'],check=True)
assert subprocess.check_output(['git','--no-optional-locks','rev-parse','v0.2.15^{commit}'],text=True).strip()=='1ecf69abe9f92c17e13dc38fa97be659aee44582'
subprocess.run(['bash','parity/tools/check_public_text.sh'],check=True)
subprocess.run(['git','--no-optional-locks','diff','--check'],check=True)
assert subprocess.check_output(['git','--no-optional-locks','rev-parse','HEAD'],text=True).strip()=='1ecf69abe9f92c17e13dc38fa97be659aee44582'
subprocess.run(['git','--no-optional-locks','diff','--cached','--exit-code','--quiet'],check=True)
assert target.name=='meshopt-0216-decode' and target.is_dir()
shutil.rmtree(target)
record=dict(target_deleted=not target.exists(),head_unchanged=True,staged_diff_empty=True,final_proof_steps=len(proof),final_proof_cpu_seconds=final_cpu,total_proof_cpu_seconds=initial_cpu+final_cpu,baseline=summary,tuning=verdict)
(lane/'receipts/cleanup.json').write_text(json.dumps(record,indent=2)+'\n')
manifest={str(p.relative_to(lane/'receipts')):hashlib.sha256(p.read_bytes()).hexdigest() for p in (lane/'receipts').rglob('*') if p.is_file() and p.name!='SHA256SUMS.json'}
(lane/'receipts/SHA256SUMS.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
print(json.dumps(record),flush=True)
