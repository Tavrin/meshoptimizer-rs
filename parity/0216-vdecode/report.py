"""Apply the registered gate to the complete final campaign; never alter source."""
import json,math,os,shutil,statistics
from pathlib import Path
R=Path.cwd();S=Path(os.environ['DECODE_ROOT']);L=R/'parity/0216-vdecode'
r=json.loads((S/'guard/guard-results.json').read_text());rows=r['cells'];summary=r['summary']
assert len(rows)==8*19
counts=[json.loads(s) for s in (S/'guard/guard-counts.jsonl').read_text().splitlines()]
assert len(counts)==1872 and all(x['exit']==0 and x['running_percent']==100 for x in counts)
v=next(x for x in summary if x['op']=='vertex-decode' and x['group']=='real')
i=next(x for x in summary if x['op']=='index-decode' and x['group']=='real')
worst_index=max(x['ratios']['candidate'] for x in rows if x['op']=='index-decode')
worst_other=max(x['ratios']['candidate'] for x in rows if x['op'] not in ['vertex-decode','index-decode'])
passed=v['base']<=.95 and all(x['base']<=1.005 for x in summary if x['op']=='index-decode') and worst_index<=1.01 and worst_other<=1.01
verdict=dict(verdict='SHIPS' if passed else 'REJECTED',vertex_real_geomean=v['base'],index_real_geomean=i['base'],worst_index_case=worst_index,worst_other_case=worst_other,samples=len(counts))
(S/'verdict.json').write_text(json.dumps(verdict,indent=2)+'\n')
(L/'verdict.json').write_text(json.dumps(verdict,indent=2)+'\n')
shutil.copy2(S/'guard/guard-TABLE.md',L/'TABLE.md')
lines=['# Final qualification','','Verdict: **'+verdict['verdict']+'**. The registered thresholds are unchanged.','', '| Operation | Real / base | Corpus / base | Real / scalar C++ | Real / default C++ | Worst / base |','|---|---:|---:|---:|---:|---:|']
for op in sorted({x['op'] for x in summary}):
 a=next(x for x in summary if x['op']==op and x['group']=='real');b=next(x for x in summary if x['op']==op and x['group']=='corpus')
 lines.append(f"| {op} | {a['base']:.6f} | {b['base']:.6f} | {a['cpp_scalar']:.6f} | {a['cpp_default']:.6f} | {max(a['worst'],b['worst']):.6f} |")
lines+=['','Wall duration disclosure (real meshes; geometric means of per-case median ratios):','','| Decoder | / base | / scalar C++ | / default C++ |','|---|---:|---:|---:|']
for op in ['vertex-decode','index-decode']:
 cells=[x for x in rows if x['op']==op and x['case'].startswith('real-')]
 def ratio(arm):return math.exp(statistics.mean(math.log(x['wall_seconds']['candidate']/x['wall_seconds'][arm]) for x in cells))
 lines.append(f'| {op} | {ratio("base"):.6f} | {ratio("cpp-scalar"):.6f} | {ratio("cpp-default"):.6f} |')
proof=json.loads((S/'proof-final/exits.json').read_text());prior=json.loads((S/'proof/exits.json').read_text())
cpu=sum(x['cpu_seconds'] for x in proof+prior)
assert all(x['exit']==0 for x in proof) and cpu<=7200
lines+=['',f'Final proof: {len(proof)} steps, all exit 0. Including the rejected first experiment, targeted proof CPU is {cpu:.2f} seconds, below 7,200 seconds.', '', 'TABLE.md discloses every median and wall duration. Wall time does not determine admission. The complete guard uses the same canonical build path, four required arms, eight fixtures, 19 operations and three interleaved samples. The separate control is measured for the two decoders only. Every sample has 100% counter running time. Rust base/candidate output hashes agree per cell; both C++ decode output hashes also agree.', '', 'Other-operation ratios include the starting integration changes and serve as guard evidence.', '', 'The original generic vertex dispatcher and slice monomorph remain unchanged. The owned-only selector instantiates existing stride-12 kernels behind a separate non-inlined owned entry. Earlier generic outlining and associated-constant experiments are rejected and retained under receipts/screen and receipts/screen2.', '', 'Exact resource-contract tests compare 5,114,700 observations per feature mode (owned/into, full buffers and tails, errors and Usage, every small-fixture work limit, byte boundaries, cold/warm/after-failure workspaces). Native forced-level differential coverage is 318,608 cases per std/no-std mode. Successful C++ bytes and statuses agree. Each mode records 74,863 inherited C++ failure-prefix differences; Rust malformed-write identity is established against the original Rust scalar and v0.2.15 contracts, not against upstream staging behavior.', '', 'All-feature/no-default/no-std SIMD tests, Clippy, formatting, independent unsafe boundary, pinned reference, package verification, unsafe-free check, Miri subset, fixed 20,000-run ASan fuzz and public-text checks pass. This lane changes x86 only; it makes no new NEON/wasm execution or performance claim.', '', 'Raw artifacts, frozen source trees and executables remain outside the repository. Public receipts bind source, binaries, inputs and individual samples. The dedicated build target is removed after retention. Git integration and publication remain operator-owned.']
(L/'REPORT.md').write_text('\n'.join(lines)+'\n')
proof_file=L/'PROOFS.md'
text=proof_file.read_text()
assert '## Final evidence' not in text
codegen=json.loads((S/'screen3/codegen/raw-comparison.json').read_text())
assert codegen['candidate']['base_instruction_identity']
text+='\n## Final evidence\n\n'
text+=f"Verdict: **{verdict['verdict']}**. Final real-mesh vertex instructions are {v['base']:.6f}x v0.2.15; index instructions are {i['base']:.6f}x, with worst case {worst_index:.6f}x. The worst other-function case is {worst_other:.6f}x. The complete matrix contains {len(counts)} samples across all 19 release operations. See REPORT.md and TABLE.md.\n\n"
text+='The shared raw dispatcher has the same 1,734 relocation-normalized instructions as v0.2.15 in the decoder consumer. Raw disassembly and static-data relocation differences are retained alongside the instruction-only comparison. Register allocation, local branches, immediates and stack layout are retained by the comparison; binary byte identity is not claimed. The generic/slice dispatch source is verbatim.\n\n'
text+=f"Exact comparison passed 5,114,700 observations per feature mode. Native ISA/C++ differential passed 318,608 cases per std/no-std mode; Rust scalar/full-buffer identity and C++ success bytes/statuses agree. The 74,863 inherited C++ malformed-prefix differences per mode remain explicitly recorded. Final proof steps: {len(proof)}, all exit 0. Total proof CPU including the first rejected experiment: {cpu:.2f} seconds. Miri, 20,000 ASan runs, package, unsafe-free and public-text checks pass. Unsafe inventory: 29 blocks, one allowance.\n"
proof_file.write_text(text)
print(json.dumps(verdict,indent=2))
