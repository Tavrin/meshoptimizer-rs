"""Audit completed receipts without remeasuring any case."""
from pathlib import Path
from collections import Counter
import hashlib,json,statistics
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
gates=dict(line.split('\t') for line in (S/'gates.tsv').read_text().splitlines())
required=['fmt','proof-fmt','all-features','no-default-features','clippy','boundary','exact-build-std','exact-scale-std','exact-overdraw-std','exact-build-no-std','exact-scale-no-std','exact-overdraw-no-std','cpp-differential','measurement','final-fmt','final-all-features','final-no-default-features','final-clippy','final-boundary','fuzz']
assert all(gates.get(k)=='0' for k in required),gates
fuzz=json.loads((S/'fuzz-results/fuzz.json').read_text())
assert fuzz['passed'] and fuzz['identities_unchanged']
assert all(r['cpu_seconds']>=300 and r['crashes']==0 and r['successes']>0 for r in fuzz['runs'])
raw=json.loads((S/'samples/raw.json').read_text());summary=json.loads((S/'samples/summary.json').read_text());binding=json.loads((S/'samples/binding.json').read_text());build=json.loads((S/'build-binding.json').read_text())
assert len(raw)==216 and len(summary)==24
assert all(n==3 for n in Counter((r['case'],r['arm']) for r in raw).values())
assert all(r['exit']==0 and r['cpp_equal'] and r['counter_running_percent']>=99 for r in raw)
for c in summary:
 for arm in ['base','candidate','M7']:
  assert c['median'][arm]==statistics.median(r['per_call'] for r in raw if r['case']==c['id'] and r['arm']==arm)
 assert c['median']['candidate']/c['median']['base']<=1.01
for p in (R/'src').rglob('*.rs'):
 rel=str(p.relative_to(R));digest=sha(p)
 assert digest==binding['sources'][str(p)]==build['sources'][rel]==fuzz['source_sha256'][rel],p
for a,h in build['binaries'].items():assert sha(S/f'driver-{a}')==h==binding['binaries'][a]
for p,h in binding['inputs'].items():assert sha(Path(p))==h,p
for mode in ['std','no-std']:
 assert 'PASS 1950606' in (S/f'gate-exact-scale-{mode}.log').read_text()
 assert 'PASS: 2835968' in (S/f'gate-exact-overdraw-{mode}.log').read_text()
assert 'PASS 356' in (S/'gate-cpp-differential.log').read_text()
result=dict(passed=True,required_exit_codes={k:int(gates[k]) for k in required},samples=len(raw),cases=len(summary),minimum_counter_running=min(r['counter_running_percent'] for r in raw),exact_observations=9573148,cpp_cases=356,fuzz_runs=fuzz['runs'],source_and_binary_bindings_verified=True)
(S/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
print('PASS final receipts, sources, binaries, medians, outputs, CPU budgets and exit codes')
