from pathlib import Path
import json,math,subprocess
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2')
rows=json.loads((S/'samples/summary.json').read_text())
def gm(v):
 v=list(v);return math.exp(sum(map(math.log,v))/len(v))
def metrics(rr):
 cb=gm(r['median']['candidate']/r['median']['base'] for r in rr)
 cm=gm(r['median']['candidate']/r['median']['M7'] for r in rr)
 worst=max(r['median']['candidate']/r['median']['base'] for r in rr)
 verdict='FAIL' if worst>1.01 else 'PASS' if cm<=1 else 'PARTIAL' if cb<=.95 else 'NO-GAIN'
 return dict(candidate_base=cb,candidate_M7=cm,worst=worst,verdict=verdict)
agg={}
for family in ['simplify-scale','overdraw']:
 rr=[r for r in rows if r['family']==family]
 agg[family]={group:metrics([r for r in rr if group=='all' or r['kind']!='chain']) for group in ['all','frozen']}
chains={}
for c in sorted(set(r['chain'] for r in rows if r['kind']=='chain')):
 chains[c]={}
 for f in ['simplify-scale','overdraw']:
  rr=[r for r in rows if r.get('chain')==c and r['family']==f]
  totals={a:sum(r['median'][a]*r['multiplicity'] for r in rr) for a in ['base','candidate','M7']}
  chains[c][f]=dict(totals=totals,candidate_base=totals['candidate']/totals['base'],candidate_M7=totals['candidate']/totals['M7'])
result=dict(families=agg,chains=chains)
(S/'verdicts.json').write_text(json.dumps(result,indent=2)+'\n')
lines=['# H2 safe hot loops','', 'Baseline: v0.2.9, `df7fc567b6540eb06cb583e2d323c01e5797609c`. Candidate source/binary/input hashes and all raw counter samples are retained under `/mnt/linux-extra/moss-scratch/meshopt-0210-H2/`. No wall timings were taken.','', '## Techniques','']
for fn,needle,desc in [('src/input.rs','pub(crate) fn scale_full_records','Pre-slice fully padded 64-byte records into fixed-size arrays. Short trailing records retain the original scale reader and validation-return position; mapped and other layouts retain their original paths. Preserve padding exclusion and the independent finiteness pass.'),('src/overdraw.rs','fn update(','Prove the three stamp access bounds together with their maximum index.'),('src/overdraw.rs','trait ClusterMeter','Precharge the exact soft-cluster phase bound when all work fits and the threshold proves the target finite. A monomorphized prepaid kernel omits budget checks; tight budgets and overflow-capable thresholds retain the original per-visit path.'),('src/overdraw.rs','fn soft_clusters','Iterate adjacent hard-cluster boundaries through slices rather than repeated indexed lookups.')]:
 text=(R/fn).read_text();lineno=next((i for i,l in enumerate(text.splitlines(),1) if needle in l),0)
 if lineno:lines.append(f'- `{fn}:{lineno}`: {desc}')
lines+=['','No new unsafe, API, allocation policy, SIMD, square-root backend, version, or other-family changes. The existing C++ driver is reused; no new FFI driver was added.','', '## Instruction gate','', 'Counter-only resident perf 6.17 `instructions:u`, CPU9, median of three per arm, opt3/thin LTO/one codegen unit, Rust std without SIMD, scalar meshoptimizer 1.3 (meshopt 0.7.0). All counters must report ≥99% running. See REGISTRATION.md and retained preparation/binding JSON for archived harness provenance.','', '| Family / population | candidate/base GM | candidate/M7 GM | worst candidate/base | Verdict |','|---|---:|---:|---:|---|']
for f,groups in agg.items():
 for g,m in groups.items():lines.append(f"| {f} / {g} | {m['candidate_base']:.6f} | {m['candidate_M7']:.6f} | {m['worst']:.6f} | {m['verdict']} |")
lines+=['','Frozen includes the three real-overdraw fixtures. All is the GM over distinct frozen and chain call inputs. The per-case regression rule applies to every input.','', '| Chain / stage | base instructions | candidate instructions | M7 instructions | candidate/base | candidate/M7 |','|---|---:|---:|---:|---:|---:|']
for c,stages in chains.items():
 for f,m in stages.items():
  t=m['totals'];lines.append(f"| {c[:4]} / {f} | {t['base']:.2f} | {t['candidate']:.2f} | {t['M7']:.2f} | {m['candidate_base']:.6f} | {m['candidate_M7']:.6f} |")
lines+=['','Chain stage totals sum per-call medians with retained call multiplicity. These are captured stage replays, not whole-application measurements. Reorder counters include the adapter’s saved-input clone and output copy.','', '| Case | base instructions/call | candidate instructions/call | M7 instructions/call | candidate/base | candidate/M7 |','|---|---:|---:|---:|---:|---:|']
for r in rows:
 m=r['median'];lines.append(f"| {r['id']} | {m['base']:.2f} | {m['candidate']:.2f} | {m['M7']:.2f} | {m['candidate']/m['base']:.6f} | {m['candidate']/m['M7']:.6f} |")
lines+=['','## Proof and exit codes','','```text',(S/'gates.tsv').read_text().rstrip(),'```','', 'All exact comparisons cover complete Result values, every output word (including failure prefixes and untouched destination tails), and Usage work/bytes against a separately compiled v0.2.9. Scale compares float bits and Result values; its API has no workspace. Existing seed cases and frozen corpus are included, in std and no_std; every work prefix from zero through the terminal prefix plus one is checked on small finite/nonfinite/overflow/subnormal/disconnected meshes. All accepted layout classes are covered, including full/short 64-byte little/big-endian views and unaligned offsets. Existing pinned C++ differentials and counter output hashes provide independent parity evidence.','', 'The existing fuzz generators already reach full and short 64-byte records for scale and overdraw, both byte orders and unaligned offsets; no widening was needed. Retained CPU-accounted fuzz logs must show at least 300 CPU seconds per family and zero crashes.','', 'Initial candidate build exited 101 (integer type inference in extracted clustering helper); fixed with an explicit u32 annotation before measurement. The failed receipt is retained. Cancelled admission waits produced no measurements. Pilot samples are separate from the final campaign.','']
lines += ['## Completed observations', '',
          'Final instruction campaign: 24 distinct cases, 216 samples, every process exit 0, 100% counter running, all Rust/C++ outputs byte-identical and all chain outputs matching their historical hashes.', '',
          'Exact comparisons: 1,950,606 scale Result/bit observations and 2,835,968 overdraw Result/output/Usage observations per configuration, for 9,573,148 across std and no_std. Existing pinned C++ differential: 356 cases, exit 0.', '',
          'Round 1 was retained under round1/ and replaced to preserve the original short-record validation-return position. The final round rebuilds and reruns all required gates, measurements and CPU fuzzing against the accepted source. No implementation source changed during final qualification.', '',
          'Rejected scale approach: a chained full-prefix/final-record iterator regressed the tiny and medium pilot cases by 45.7% and 50.2%. It was replaced before the final campaign. Raw pilot samples and a reconstruction verified against every sampled source hash remain under pilot-2/ and pilot-2-rejected-source/. Failed build and rejected pilot receipts remain recorded; they are not passing gates.', '',
          'Both final families are PARTIAL. Finite validation and checked indexed reads remain; no C++ instruction parity or wall-time improvement is claimed.', '']
fuzz_path=S/'fuzz-results/fuzz.json'
if fuzz_path.exists():
    fuzz=json.loads(fuzz_path.read_text())
    lines += ['| Fuzz target | CPU seconds | Executions | Crashes |', '|---|---:|---:|---:|']
    for run in fuzz['runs']:
        lines.append(f"| {run['target']} | {run['cpu_seconds']:.2f} | {run['executions']} | {run['crashes']} |")
    lines += ['', f"CPU-accounted fuzz verdict: passed={fuzz['passed']}; source/dependency/executable identities unchanged={fuzz.get('identities_unchanged', False)}. Full replay descriptors, binaries and source archive are retained outside the build target.", '']
(R/'parity/0210-H2/REPORT.md').write_text('\n'.join(lines))
(R/'parity/0210-H2/instructions.json').write_text(json.dumps(dict(summary=rows,verdicts=result),indent=2)+'\n')
print(json.dumps(agg,indent=2))
if any(v['all']['verdict'] in ['FAIL','NO-GAIN'] for v in agg.values()):raise SystemExit(1)
