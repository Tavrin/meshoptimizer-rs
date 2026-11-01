"""Finalize after all admitted jobs exit, retaining failures and deleting scratch."""
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
from fractions import Fraction

root = Path.cwd()
lane = root / 'parity/0217-pipeline'
out = lane / 'receipts'
target = Path(os.environ['CARGO_TARGET_DIR'])
assert target.name == 'meshopt-0217-pipeline'
exits = {}
for line in (out / 'exits.tsv').read_text().splitlines():
    name, code = line.split('\t')
    exits[name] = int(code)

def read(name):
    p = out / name
    return p.read_text() if p.exists() else ''

def samples(text):
    result = {}
    pattern = r'sample (\S+) repetition=(\d+) arm=(\d) ns=(\d+)(?: batch=(\d+))?'
    for name, rep, arm, ns, batch in re.findall(pattern, text):
        result.setdefault(name, [[], []])[int(arm)].append(
            dict(repetition=int(rep), ns=int(ns), batch=int(batch or 1)))
    return result

def analyze(name, arms, campaign):
    if not all(arms) or len(arms[0]) != len(arms[1]):
        return None
    times = [[Fraction(x['ns'], x['batch']) for x in arm] for arm in arms]
    if any(x <= 0 for arm in times for x in arm):
        return None
    a, b = map(statistics.median, times)
    paired = statistics.median([p / m for p, m in zip(*times)])
    return dict(case=name, campaign=campaign, sample_pairs=len(arms[0]),
                pipeline_ns=float(a), manual_ns=float(b),
                paired_median_ratio=float(paired), ratio_of_medians=float(a / b),
                within_time_target=paired * 100 <= 102, samples=arms)

original = read('equivalence-bench.log')
first = samples(original)
extra = samples(read('supplement.log'))
prior = []
for name, old in first.items():
    if name.startswith('real-'):
        selected, campaign = old, 'original'
    elif name == 'million-smooth':
        selected = [old[k] + extra.get(name, [[], []])[k] for k in [0, 1]]
        campaign = 'all original and supplemental pairs'
    else:
        selected = extra.get(name, [[], []])
        campaign = 'batched replacement for sub-200ms clock resolution'
    row = analyze(name, selected, campaign)
    if row:
        row['clock_resolved'] = all(x['ns'] >= 200_000_000 for arm in selected for x in arm)
        prior.append(row)

quiet_text = read('quiet.log')
quiet = samples(quiet_text)
controlled = [row for name, arms in quiet.items()
              if (row := analyze(name, arms, 'CPU-accounted complete campaign'))]
cpu = []
pattern = r'cpu (\S+) repetition=(\d+) arm=(\d) ticks=(\d+) hz=(\d+) wall_ns=(\d+) qualified=(true|false)'
for name, rep, arm, ticks, hz, ns, declared in re.findall(pattern, quiet_text):
    ticks, hz, ns = int(ticks), int(hz), int(ns)
    lower = max(0, ticks * 1_000_000_000 // hz - 2_000_000_000 // hz)
    valid = ns >= 5_000_000_000 and lower * 100 >= (ns + 1_000_000) * 99
    cpu.append(dict(case=name, repetition=int(rep), arm=int(arm), ticks=ticks,
                    hz=hz, wall_ns=ns, observed_cpu_fraction=ticks * 1e9 / hz / ns if ns else None,
                    lower_bound=lower / (ns + 1_000_000), qualified=valid,
                    driver_qualified=declared == 'true'))
complete = (len(controlled) == 11 and len(cpu) == 154 and
            all(r['sample_pairs'] == 7 for r in controlled) and all(r['qualified'] for r in cpu))
performance = complete and all(r['within_time_target'] for r in controlled)
required = ['tests', 'default', 'no-std', 'doctests', 'docs', 'wasm', 'wasm-default',
            'final-fmt', 'final-clippy', 'final-pipeline', 'final-pipeline-scalar',
            'final-docs', 'final-semver', 'final-wasm', 'final-retained',
            'exact-default', 'exact-scalar', 'cpp-default', 'cpp-scalar',
            'equivalence-bench', 'supplement-build', 'supplement', 'quiet-build', 'quiet',
            'wasm-smoke-build-default', 'wasm-smoke-default',
            'wasm-smoke-build-no-std', 'wasm-smoke-no-std']
gates = {name: exits.get(name) for name in required}
gates['cpu-qualification'] = 0 if complete else 1
gates['performance'] = 0 if performance else 1
result = dict(status='PASS' if all(code == 0 for code in gates.values()) else 'FAIL',
              final_gates=gates, performance_limit=1.02,
              equivalent_cases=len(re.findall(r'^equivalent ', original, re.M)),
              prior_timings=prior, controlled_timings=controlled, cpu_samples=cpu,
              cpu_complete_campaign=complete, target_removed=False,
              initial_failures=dict(semver=exits.get('semver'), public=exits.get('public')),
              residuals=[] if performance else [
                  'The 1.02 time target is not qualified. No samples were selected by speed ratio.',
                  'Shared-host paired medians exceed the target for medium-seams and million-smooth.',
                  'The CPU-accounted attempt rejected its first pair; manual CPU occupancy was below 99%.'])

# Preserve measured binary identities before cleanup.
for name, path in [('last', target / 'release/pipeline'),
                   ('original-default', target / 'proof/pipeline-default')]:
    if path.exists():
        (out / ('binary-' + name + '.json')).write_text(json.dumps(
            dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest())) + '\n')

# Public logs retain numeric evidence; bind the original and redacted bytes.
old_bindings = json.loads(read('log-binding.json') or '{}')
bindings = {}
for path in sorted(out.glob('*.log')):
    data = path.read_bytes()
    sha = hashlib.sha256(data).hexdigest()
    previous = old_bindings.get(path.name, {})
    raw_sha = previous.get('raw_sha256', sha) if previous.get('public_sha256') == sha else sha
    text = data.decode(errors='replace')
    for prefix, label in [(str(target), '<TARGET>'), (str(root), '<WORKTREE>'),
                          (str(Path.home()), '<USER>')]:
        text = text.replace(prefix, label)
    path.write_text(text)
    bindings[path.name] = dict(raw_sha256=raw_sha,
                              public_sha256=hashlib.sha256(path.read_bytes()).hexdigest())
(out / 'log-binding.json').write_text(json.dumps(bindings, indent=2) + '\n')

summary = ['# Pipeline qualification', '', '**Verdict: FAIL — performance target unqualified.**', '',
           '`process_mesh` composes the existing functions through one workspace and returns',
           'one stage/quality report with staged errors. Geometry equivalence passed on',
           'all eleven grid/corpus cases, including all three retained real meshes.',
           'Both exact regression modes and both independent C++ F9 oracle modes passed.', '',
           'The original zero-duration tiny timings were rejected. Batched small-case',
           'measurements and all seven million-triangle pairs remain retained. Ratios',
           'below are medians of within-pair ratios; the limit remains 1.02.', '',
           '| Case | Pairs | One-call/manual |', '| --- | ---: | ---: |']
summary += [f"| {r['case']} | {r['sample_pairs']} | {r['paired_median_ratio']:.6f} |" for r in prior]
summary += ['', 'A separate CPU-accounted full-campaign attempt selected an idle allowed',
            'physical core and stopped on its first rejected pair. The manual arm had',
            'about 92% CPU occupancy; the conservative required lower bound was 99%.',
            'No quiet-core performance pass is claimed. The cause of shared-host',
            'above-limit medians is unproven. See `receipts/quiet.log` and `quiet-cpu.json`.', '',
            'Wasm timing is absent where no std clock is available; default and no_std',
            'runtime smoke checks exercise the new API. Retained scratch is counted',
            'even on validation failure. Cluster outputs retain any dilated positions.', '',
            'Final gate codes, samples and source/binary/log bindings are in',
            '`receipts/final.json`, `exits.tsv` and the binding JSON files.',
            'The dedicated build directory is removed after every admitted job exits.',
            'Git state is unchanged.', '']
(lane / 'RESULTS.md').write_text('\n'.join(summary))
source = [*sorted((root / 'src').rglob('*.rs')), root / 'Cargo.toml', root / 'Cargo.lock',
          root / 'tests/pipeline.rs', root / 'docs/USING.md', root / 'CHANGELOG.md',
          *sorted(out.glob('*.rs')), *sorted(lane.glob('*.rs')), *sorted(lane.glob('*.py')),
          *sorted(lane.glob('*.sh')), *sorted(lane.glob('*.md'))]
(out / 'source-binding.json').write_text(json.dumps(
    {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in source}, indent=2) + '\n')
(out / 'final.json').write_text(json.dumps(result, indent=2) + '\n')
public = subprocess.run(['bash', 'parity/tools/check_public_text.sh'], capture_output=True, text=True)
(out / 'final-public.log').write_text(public.stdout + public.stderr)
result['final_gates']['final-public'] = public.returncode
if public.returncode:
    result['status'] = 'FAIL'
if target.exists():
    shutil.rmtree(target)
result['target_removed'] = not target.exists()
with (out / 'exits.tsv').open('a') as f:
    for name in ['cpu-qualification', 'performance', 'final-public']:
        f.write(f"{name}\t{result['final_gates'][name]}\n")
(out / 'final.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(dict(status=result['status'], gates=result['final_gates'],
                      target_removed=result['target_removed']), indent=2))
raise SystemExit(0 if result['status'] == 'PASS' else 1)
